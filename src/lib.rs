#![no_std]

pub mod advertising;
mod hci;
pub mod le_states;
mod utils;
pub mod uuid;

use crate::advertising::AdvertisingData;
use core::cell::{BorrowMutError, RefCell};
use embedded_io::Error as EmbeddedIoError;

use crate::hci::command::Command;
use crate::hci::error_code::HciErrorCode;
use crate::hci::event::{CommandCompleteEvent, Event};
use crate::hci::event_mask::EventMask;
use crate::hci::event_parameter::{
    LeFeaturesEventParameter, LeStatesEventParameter, LmpFeaturesEventParameter,
    StatusEventParameter, SupportedCommandsEventParameter,
};
use crate::hci::opcode::OpCode;
use crate::hci::supported_commands::SupportedCommands;
use crate::hci::supported_features::SupportedFeatures;
use crate::hci::supported_le_features::SupportedLeFeatures;
use crate::hci::supported_le_states::SupportedLeStates;
use crate::hci::PacketType;

#[derive(Debug)]
pub enum Error {
    HciAccessDenied,
    IO(embedded_io::ErrorKind),
    InvalidPacketType(u8),
    ReceivedUnhandledHciPacket(PacketType),
    ReceivedUnexpectedHciPacket,
    InvalidEventCode(u8),
    InvalidEventPacket,
    InvalidOpcode(u16),
    HciError(HciErrorCode),
    InvalidErrorCode(u8),
    NonLECapableController,
    InvalidStateCombination,
    BufferTooSmall,
    AdStructAlreadyPresent,
    AdStructDoesNotFit,
}

impl From<BorrowMutError> for Error {
    fn from(_value: BorrowMutError) -> Self {
        Self::HciAccessDenied
    }
}

pub struct BleStack<T>
where
    T: embedded_io::Read + embedded_io::Write,
{
    hci: RefCell<T>,
    supported_commands: SupportedCommands,
    supported_features: SupportedFeatures,
    supported_le_features: SupportedLeFeatures,
    supported_le_states: SupportedLeStates,
    num_hci_command_packets: usize,
    le_data_packet_length: usize,
    num_le_data_packets: usize,
}

impl<T> BleStack<T>
where
    T: embedded_io::Read + embedded_io::Write,
    <T as embedded_io::ErrorType>::Error: embedded_io::Error,
{
    pub fn new(hci: T) -> Self {
        Self {
            hci: hci.into(),
            supported_commands: SupportedCommands::default(),
            supported_features: SupportedFeatures::default(),
            supported_le_features: SupportedLeFeatures::default(),
            supported_le_states: SupportedLeStates::default(),
            num_hci_command_packets: 0,
            le_data_packet_length: 255,
            num_le_data_packets: 1,
        }
    }

    pub fn setup(&mut self) -> Result<(), Error> {
        // Perform setup has described in Core specification 4.2, Vol. 6, Part D, 2.1

        self.cmd_reset()?;

        self.cmd_read_local_supported_commands()?;
        self.cmd_read_local_supported_features()?;
        if !self.supported_features.has_le_supported_controller() {
            return Err(Error::NonLECapableController);
        }
        self.set_event_mask()?;
        // TODO: set LE event mask
        self.cmd_le_read_buffer_size()?;
        if (self.le_data_packet_length == 0)
            || (self.num_le_data_packets == 0) && self.supported_commands.has_read_buffer_size()
        {
            self.cmd_read_buffer_size()?;
        }
        if self
            .supported_commands
            .has_le_read_local_supported_features()
        {
            self.cmd_le_read_local_supported_features()?;
        }
        self.cmd_le_read_supported_states()?; // TODO: needed??

        Ok(())
    }

    pub fn supported_commands(&self) -> &SupportedCommands {
        &self.supported_commands
    }

    pub fn supported_features(&self) -> &SupportedFeatures {
        &self.supported_features
    }

    pub fn supported_le_features(&self) -> &SupportedLeFeatures {
        &self.supported_le_features
    }

    pub fn supported_le_states(&self) -> &SupportedLeStates {
        &self.supported_le_states
    }

    // TODO: Add scan response data & advertising parameters
    pub fn start_advertising(&self, adv_data: &AdvertisingData) -> Result<(), Error> {
        let (adv_data, adv_data_size) = adv_data.encode()?;
        log::info!("Adv data: {:?}", adv_data);
        log::info!("Adv data size: {}", adv_data_size);
        // TODO
        Ok(())
    }

    fn set_event_mask(&self) -> Result<CommandCompleteEvent, Error> {
        let event_mask = EventMask::new()
            .clear()
            .hardware_error(true)
            .data_buffer_overflow(true)
            .le_meta_event(true)
            .disconnection_complete(true)
            .read_remote_version_information_complete(true)
            .encryption_change(true)
            .encryption_key_refresh_complete(true);
        self.cmd_set_event_mask(event_mask)
    }

    fn execute_command(&self, command: Command) -> Result<CommandCompleteEvent, Error> {
        self.hci_write(command.encode()?.data())?;
        let event = self.hci_wait_for_command_complete(command.opcode())?;
        let status_event_parameter: StatusEventParameter =
            event.return_parameters.slice(0)?.try_into()?;
        match status_event_parameter.status {
            HciErrorCode::Success => Ok(event),
            _ => Err(Error::HciError(status_event_parameter.status)),
        }
    }

    fn cmd_set_event_mask(&self, event_mask: EventMask) -> Result<CommandCompleteEvent, Error> {
        self.execute_command(Command::SetEventMask(event_mask))
    }

    fn cmd_reset(&mut self) -> Result<CommandCompleteEvent, Error> {
        let event = self.execute_command(Command::Reset)?;
        self.num_hci_command_packets = event.num_hci_command_packets as usize;
        Ok(event)
    }

    fn cmd_read_local_supported_commands(&mut self) -> Result<CommandCompleteEvent, Error> {
        let event = self.execute_command(Command::ReadLocalSupportedCommands)?;
        let supported_commands_event_parameter: SupportedCommandsEventParameter =
            event.return_parameters.slice(1)?[..64].try_into()?;
        self.supported_commands = supported_commands_event_parameter.value;
        Ok(event)
    }

    fn cmd_read_local_supported_features(&mut self) -> Result<CommandCompleteEvent, Error> {
        let event = self.execute_command(Command::ReadLocalSupportedFeatures)?;
        let lmp_features_event_parameter: LmpFeaturesEventParameter =
            event.return_parameters.le_u64(1)?.into();
        self.supported_features = lmp_features_event_parameter.value;
        Ok(event)
    }

    fn cmd_read_buffer_size(&mut self) -> Result<CommandCompleteEvent, Error> {
        let event = self.execute_command(Command::ReadBufferSize)?;
        self.le_data_packet_length = event.return_parameters.le_u16(1)? as usize;
        self.num_le_data_packets = event.return_parameters.le_u16(4)? as usize;
        Ok(event)
    }

    fn cmd_le_read_local_supported_features(&mut self) -> Result<CommandCompleteEvent, Error> {
        let event = self.execute_command(Command::LeReadLocalSupportedFeatures)?;
        let le_features_event_parameter: LeFeaturesEventParameter =
            event.return_parameters.le_u64(1)?.into();
        self.supported_le_features = le_features_event_parameter.value;
        Ok(event)
    }

    fn cmd_le_read_buffer_size(&mut self) -> Result<CommandCompleteEvent, Error> {
        let event = self.execute_command(Command::LeReadBufferSize)?;
        self.le_data_packet_length = event.return_parameters.le_u16(1)? as usize;
        self.num_le_data_packets = event.return_parameters.u8(3)? as usize;
        Ok(event)
    }

    fn cmd_le_read_supported_states(&mut self) -> Result<CommandCompleteEvent, Error> {
        let event = self.execute_command(Command::LeReadSupportedStates)?;
        let le_states_event_parameter: LeStatesEventParameter =
            event.return_parameters.le_u64(1)?.into();
        self.supported_le_states = le_states_event_parameter.value;
        Ok(event)
    }

    fn hci_write(&self, data: &[u8]) -> Result<usize, Error> {
        self.hci
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
                    return Err(Error::ReceivedUnhandledHciPacket(PacketType::Command))
                }
                PacketType::AclData => {
                    todo!()
                }
                PacketType::SynchronousData => {
                    return Err(Error::ReceivedUnhandledHciPacket(
                        PacketType::SynchronousData,
                    ))
                }
                PacketType::Event => {
                    let event = Event::read(&self.hci)?;
                    return Ok(Some(HciPollResult::Event(event)));
                }
            }
        }

        Ok(None)
    }

    fn hci_read(&self) -> Result<Option<u8>, Error> {
        let mut buffer = [0u8];
        let res = self.hci.try_borrow_mut()?.read(&mut buffer);
        match res {
            Ok(1) => Ok(Some(buffer[0])),
            _ => Ok(None),
        }
    }
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)] // TODO: To remove when AclData is used
enum HciPollResult {
    _AclData,
    Event(Event),
}
