#![no_std]

use bitflags::Flags;
use core::cell::{BorrowMutError, RefCell};
use core::marker::PhantomData;
use embedded_io::Error as EmbeddedIoError;

pub mod advertising;
pub mod assigned_numbers;
mod connection_interval_value;
pub mod hci;
pub mod le_states;
mod supported_le_features;
mod utils;
pub mod uuid;

pub use connection_interval_value::ConnectionIntervalValue;
pub use supported_le_features::SupportedLeFeatures;

use advertising::advertising_parameters::AdvertisingParameters;
use advertising::AdvertisingError;
use advertising::{AdvertisingData, AdvertisingEnable, ScanResponseData};
use hci::command::Command;
use hci::event::{CommandCompleteEvent, Event};
use hci::event_mask::EventMask;
use hci::event_parameter::{
    LeFeaturesEventParameter, LeStatesEventParameter, LmpFeaturesEventParameter,
    StatusEventParameter, SupportedCommandsEventParameter,
};
use hci::opcode::OpCode;
use hci::supported_commands::SupportedCommands;
use hci::supported_le_states::SupportedLeStates;
use hci::{HciError, HciErrorCode, PacketType, SupportedFeatures};

/// Errors that can happen during the BLE stack usage.
#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Advertising related error.
    #[error(transparent)]
    Advertising(#[from] AdvertisingError),
    /// HCI related error.
    #[error(transparent)]
    Hci(#[from] HciError),
    /// IO error.
    #[error("IO error {0:?}")]
    IO(embedded_io::ErrorKind),
    /// Invalid connection interval value.
    #[error(
        "The connection interval value {0} is invalid, it needs to be between 0x0006 and 0x0C80"
    )]
    InvalidConnectionIntervalValue(u16), // TODO: Put somewhere else, in Advertising?
    /// The Bluetooth controller is not LE capable.
    #[error("The Bluetooth controller is not LE capable")]
    NonLeCapableController,
}

impl From<BorrowMutError> for Error {
    fn from(_value: BorrowMutError) -> Self {
        Self::Hci(HciError::AccessDenied)
    }
}

#[derive(Debug)]
struct ControllerCapabilities {
    supported_commands: SupportedCommands,
    supported_features: SupportedFeatures,
    supported_le_features: SupportedLeFeatures,
    supported_le_states: SupportedLeStates,
    num_hci_command_packets: usize,
    le_data_packet_length: usize,
    num_le_data_packets: usize,
}

impl Default for ControllerCapabilities {
    fn default() -> Self {
        Self {
            supported_commands: SupportedCommands::default(),
            supported_features: SupportedFeatures::default(),
            supported_le_features: SupportedLeFeatures::default(),
            supported_le_states: SupportedLeStates::default(),
            num_hci_command_packets: 0,
            le_data_packet_length: 255,
            num_le_data_packets: 1,
        }
    }
}

#[derive(Debug)]
struct Hci<HCID>
where
    HCID: embedded_io::Read + embedded_io::Write,
{
    driver: RefCell<HCID>,
}

impl<HCID> Hci<HCID>
where
    HCID: embedded_io::Read + embedded_io::Write,
    <HCID as embedded_io::ErrorType>::Error: embedded_io::Error,
{
    fn new(hci_driver: HCID) -> Self {
        Self {
            driver: RefCell::new(hci_driver),
        }
    }

    fn execute_command(&self, command: Command) -> Result<CommandCompleteEvent, Error> {
        self.hci_write(command.encode()?.data())?;
        let event = self.hci_wait_for_command_complete(command.opcode())?;
        let status_event_parameter: StatusEventParameter =
            event.return_parameters.slice(0)?.try_into()?;
        match status_event_parameter.status {
            HciErrorCode::Success => Ok(event),
            _ => Err(Error::Hci(HciError::ErrorCode(
                status_event_parameter.status,
            ))),
        }
    }

    fn cmd_set_event_mask(&self, event_mask: EventMask) -> Result<CommandCompleteEvent, Error> {
        self.execute_command(Command::SetEventMask(event_mask))
    }

    fn cmd_reset(&mut self) -> Result<CommandCompleteEvent, Error> {
        self.execute_command(Command::Reset)
    }

    fn cmd_read_local_supported_commands(
        &mut self,
    ) -> Result<SupportedCommandsEventParameter, Error> {
        let event = self.execute_command(Command::ReadLocalSupportedCommands)?;
        let supported_commands_event_parameter: SupportedCommandsEventParameter =
            event.return_parameters.slice(1)?[..64].try_into()?;
        Ok(supported_commands_event_parameter)
    }

    fn cmd_read_local_supported_features(&mut self) -> Result<LmpFeaturesEventParameter, Error> {
        let event = self.execute_command(Command::ReadLocalSupportedFeatures)?;
        let lmp_features_event_parameter: LmpFeaturesEventParameter =
            event.return_parameters.le_u64(1)?.into();
        Ok(lmp_features_event_parameter)
    }

    fn cmd_read_buffer_size(&mut self) -> Result<(usize, usize), Error> {
        let event = self.execute_command(Command::ReadBufferSize)?;
        let le_data_packet_length = event.return_parameters.le_u16(1)? as usize;
        let num_le_data_packets = event.return_parameters.le_u16(4)? as usize;
        Ok((le_data_packet_length, num_le_data_packets))
    }

    fn cmd_le_read_local_supported_features_page_0(
        &mut self,
    ) -> Result<LeFeaturesEventParameter, Error> {
        let event = self.execute_command(Command::LeReadLocalSupportedFeaturesPage0)?;
        let le_features_event_parameter: LeFeaturesEventParameter =
            event.return_parameters.le_u64(1)?.into();
        Ok(le_features_event_parameter)
    }

    fn cmd_le_read_buffer_size(&mut self) -> Result<(usize, usize), Error> {
        let event = self.execute_command(Command::LeReadBufferSize)?;
        let le_data_packet_length = event.return_parameters.le_u16(1)? as usize;
        let num_le_data_packets = event.return_parameters.u8(3)? as usize;
        Ok((le_data_packet_length, num_le_data_packets))
    }

    fn cmd_le_read_supported_states(&mut self) -> Result<LeStatesEventParameter, Error> {
        let event = self.execute_command(Command::LeReadSupportedStates)?;
        let le_states_event_parameter: LeStatesEventParameter =
            event.return_parameters.le_u64(1)?.into();
        Ok(le_states_event_parameter)
    }

    fn cmd_le_set_advertise_enable(
        &self,
        enable: AdvertisingEnable,
    ) -> Result<CommandCompleteEvent, Error> {
        self.execute_command(Command::LeSetAdvertiseEnable(enable))
    }

    fn cmd_le_set_advertising_data(
        &self,
        data: &AdvertisingData,
    ) -> Result<CommandCompleteEvent, Error> {
        self.execute_command(Command::LeSetAdvertisingData(data))
    }

    fn cmd_le_set_advertising_parameters(
        &self,
        parameters: &AdvertisingParameters,
    ) -> Result<CommandCompleteEvent, Error> {
        self.execute_command(Command::LeSetAdvertisingParameters(parameters))
    }

    fn cmd_le_set_scan_response_data(
        &self,
        data: &ScanResponseData,
    ) -> Result<CommandCompleteEvent, Error> {
        self.execute_command(Command::LeSetScanResponseData(data))
    }

    fn hci_write(&self, data: &[u8]) -> Result<usize, Error> {
        self.driver
            .try_borrow_mut()?
            .write(data)
            .map_err(|err| Error::IO(err.kind()))
    }

    fn hci_wait_for_command_complete(&self, opcode: OpCode) -> Result<CommandCompleteEvent, Error> {
        // TODO: Handle timeout
        loop {
            if let Some(HciPollResult::Event(event)) = self.hci_poll()? {
                match event {
                    Event::CommandComplete(command_complete_event)
                        if command_complete_event.opcode == opcode =>
                    {
                        return Ok(command_complete_event)
                    }
                    _ => {}
                }
            }
        }
    }

    fn hci_poll(&self) -> Result<Option<HciPollResult>, Error> {
        if let Some(packet_type) = self.hci_read()? {
            match packet_type.try_into()? {
                PacketType::Command => {
                    return Err(Error::Hci(HciError::InvalidPacketType(
                        PacketType::Command as u8,
                    )))
                }
                PacketType::AclData => {
                    todo!()
                }
                PacketType::SynchronousData => {
                    return Err(Error::Hci(HciError::InvalidPacketType(
                        PacketType::SynchronousData as u8,
                    )))
                }
                PacketType::Event => {
                    let event = Event::read(&self.driver)?;
                    return Ok(Some(HciPollResult::Event(event)));
                }
                PacketType::IsoData => {
                    return Err(Error::Hci(HciError::InvalidPacketType(
                        PacketType::IsoData as u8,
                    )))
                }
            }
        }

        Ok(None)
    }

    fn hci_read(&self) -> Result<Option<u8>, Error> {
        let mut buffer = [0u8];
        let res = self.driver.try_borrow_mut()?.read(&mut buffer);
        match res {
            Ok(1) => Ok(Some(buffer[0])),
            _ => Ok(None),
        }
    }
}

pub trait HostState {}

pub struct BleHost<'a, HCID, State: HostState = Initial>
where
    HCID: embedded_io::Read + embedded_io::Write,
{
    controller_capabilities: ControllerCapabilities,
    hci: &'a Hci<HCID>,
    phantom: PhantomData<State>,
}

#[derive(Debug, Default)]
pub struct Initial;
#[derive(Debug, Default)]
pub struct Standby;
#[derive(Debug, Default)]
pub struct Advertising;

impl HostState for Initial {}
impl HostState for Standby {}
impl HostState for Advertising {}

impl<'a, HCID> BleHost<'a, HCID, Initial>
where
    HCID: embedded_io::Read + embedded_io::Write,
{
    // Perform setup has described in Core specification 4.2, Vol. 6, Part D, 2.1
    fn setup(hci: &'a mut Hci<HCID>) -> Result<BleHost<'a, HCID, Standby>, Error>
    where
        HCID: embedded_io::Read + embedded_io::Write,
    {
        let mut controller_capabilities = ControllerCapabilities::default();

        hci.cmd_reset()?;

        controller_capabilities.supported_commands = hci.cmd_read_local_supported_commands()?.value;
        controller_capabilities.supported_features = hci.cmd_read_local_supported_features()?.value;
        if !controller_capabilities
            .supported_features
            .contains(SupportedFeatures::LE_SUPPORTED_CONTROLLER)
        {
            return Err(Error::NonLeCapableController);
        }

        let event_mask = EventMask::HARDWARE_ERROR
            | EventMask::DATA_BUFFER_OVERFLOW
            | EventMask::LE_META
            | EventMask::DISCONNECTION_COMPLETE
            | EventMask::READ_REMOTE_VERSION_INFORMATION_COMLETE
            | EventMask::ENCRYPTION_CHANGE
            | EventMask::ENCRYPTION_KEY_REFRESH_COMPLETE;
        hci.cmd_set_event_mask(event_mask)?;
        // // TODO: set LE event mask

        (
            controller_capabilities.le_data_packet_length,
            controller_capabilities.num_le_data_packets,
        ) = hci.cmd_le_read_buffer_size()?;
        if (controller_capabilities.le_data_packet_length == 0)
            || (controller_capabilities.num_le_data_packets == 0)
                && controller_capabilities
                    .supported_commands
                    .contains(SupportedCommands::READ_BUFFER_SIZE)
        {
            (
                controller_capabilities.le_data_packet_length,
                controller_capabilities.num_le_data_packets,
            ) = hci.cmd_read_buffer_size()?;
        }
        if controller_capabilities
            .supported_commands
            .contains(SupportedCommands::LE_READ_LOCAL_SUPPORTED_FEATURES_PAGE_0)
        {
            controller_capabilities.supported_le_features =
                hci.cmd_le_read_local_supported_features_page_0()?.value;
        }
        controller_capabilities.supported_le_states = hci.cmd_le_read_supported_states()?.value; // TODO: needed??

        Ok(BleHost::<HCID, Standby> {
            controller_capabilities,
            hci,
            phantom: PhantomData,
        })
    }
}

impl<'a, HCID> BleHost<'a, HCID, Standby>
where
    HCID: embedded_io::Read + embedded_io::Write,
{
    pub fn start_advertising(
        self,
        adv_params: &AdvertisingParameters,
        adv_data: &AdvertisingData,
        scanresp_data: Option<&ScanResponseData>,
    ) -> Result<BleHost<'a, HCID, Advertising>, Error> {
        self.hci.cmd_le_set_advertising_parameters(adv_params)?;
        // TODO: Read Advertising Channel Tx Power
        self.hci.cmd_le_set_advertising_data(adv_data)?;
        let default_scanresp_data = ScanResponseData::default();
        self.hci
            .cmd_le_set_scan_response_data(match scanresp_data {
                Some(scanresp_data) => scanresp_data,
                None => &default_scanresp_data,
            })?;
        self.hci
            .cmd_le_set_advertise_enable(AdvertisingEnable::Enabled)?;
        Ok(BleHost::<HCID, Advertising> {
            controller_capabilities: self.controller_capabilities,
            hci: self.hci,
            phantom: PhantomData,
        })
    }
}

impl<'a, HCID> BleHost<'a, HCID, Advertising>
where
    HCID: embedded_io::Read + embedded_io::Write,
{
    pub fn stop_advertising(self) -> Result<BleHost<'a, HCID, Standby>, Error> {
        self.hci
            .cmd_le_set_advertise_enable(AdvertisingEnable::Disabled)?;
        Ok(BleHost::<HCID, Standby> {
            controller_capabilities: self.controller_capabilities,
            hci: self.hci,
            phantom: PhantomData,
        })
    }
}

impl<'a, HCID> BleHost<'a, HCID>
where
    HCID: embedded_io::Read + embedded_io::Write,
{
    pub fn supported_commands(&self) -> &SupportedCommands {
        &self.controller_capabilities.supported_commands
    }

    pub fn supported_features(&self) -> &SupportedFeatures {
        &self.controller_capabilities.supported_features
    }

    pub fn supported_le_features(&self) -> &SupportedLeFeatures {
        &self.controller_capabilities.supported_le_features
    }

    pub fn supported_le_states(&self) -> &SupportedLeStates {
        &self.controller_capabilities.supported_le_states
    }
}

pub struct BleDevice<HCID>
where
    HCID: embedded_io::Read + embedded_io::Write,
{
    hci: Hci<HCID>,
}

impl<HCID> BleDevice<HCID>
where
    HCID: embedded_io::Read + embedded_io::Write,
{
    pub fn new(hci_driver: HCID) -> Self {
        Self {
            hci: Hci::new(hci_driver),
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        let host = BleHost::setup(&mut self.hci)?;

        todo!();

        Ok(())
    }
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)] // TODO: To remove when AclData is used
enum HciPollResult {
    _AclData,
    Event(Event),
}
