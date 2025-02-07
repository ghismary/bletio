#![no_std]

mod advertising_data;
mod advertising_enable;
mod advertising_parameters;
mod command;
mod connection_interval;
mod error_code;
mod event;
mod event_mask;
mod le_states;
mod packet;
mod supported_commands;
mod supported_features;
mod supported_le_features;
mod supported_le_states;
mod tx_power_level;

#[cfg(all(feature = "embassy", feature = "tokio"))]
compile_error!("feature \"embassy\" and feature \"tokio\" cannot be enabled at the same time");

#[cfg(feature = "embassy")]
mod embassy;
#[cfg(feature = "tokio")]
mod tokio;

use core::future::Future;
use core::num::{NonZeroU16, NonZeroU8};

use bletio_utils::{Buffer, BufferOps, Error as UtilsError};

pub(crate) use command::{Command, CommandOpCode};
pub(crate) use event::{
    CommandCompleteEvent, Event, EventCode, EventParameter, StatusAndBufferSizeEventParameter,
    StatusAndLeBufferSizeEventParameter, StatusAndSupportedCommandsEventParameter,
    StatusAndSupportedFeaturesEventParameter, StatusAndSupportedLeFeaturesEventParameter,
    StatusAndSupportedLeStatesEventParameter, StatusAndTxPowerLevelEventParameter,
    StatusEventParameter,
};
pub(crate) use packet::{Packet, PacketType};

pub use advertising_data::{AdvertisingData, ScanResponseData};
pub use advertising_enable::AdvertisingEnable;
pub use advertising_parameters::{
    AdvertisingChannelMap, AdvertisingFilterPolicy, AdvertisingIntervalValue,
    AdvertisingParameters, AdvertisingType, OwnAddressType, PeerAddress, PeerAddressType,
};
pub use connection_interval::ConnectionInterval;
pub use error_code::ErrorCode;
pub use event_mask::EventMask;
pub use le_states::{LeCombinedState, LeSingleState, LeState};
pub use supported_commands::SupportedCommands;
pub use supported_features::SupportedFeatures;
pub use supported_le_features::SupportedLeFeatures;
pub use supported_le_states::SupportedLeStates;
pub use tx_power_level::TxPowerLevel;

const HCI_COMMAND_TIMEOUT: u16 = 1000; // ms
const HCI_MAX_READ_BUFFER_SIZE: usize = 259;

/// Error occuring in the HCI part of the BLE stack.
#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// The provided data is too big to fit in an HCI command packet.
    #[error("The provided data is too big to fit in an HCI command packet")]
    DataWillNotFitCommandPacket,
    /// HCI error code.
    #[error("HCI error code {0:?}")]
    ErrorCode(ErrorCode),
    #[error(transparent)]
    HciDriver(#[from] HciDriverError),
    /// The provided advertising enable value is invalid.
    #[error("The advertising enable value {0} is invalid")]
    InvalidAdvertisingEnableValue(u8),
    /// The provided advertising filter policy is invalid.
    #[error("The advertising filter policy {0} is invalid")]
    InvalidAdvertisingFilterPolicy(u8),
    /// The provided advertising interval value is invalid, it needs to be between 0x0020 and 0x4000.
    #[error(
        "The advertising interval value {0} is invalid, it needs to be between 0x0020 and 0x4000"
    )]
    InvalidAdvertisingIntervalValue(u16),
    /// The provided advertising type is invalid.
    #[error("The advertising type {0} is invalid")]
    InvalidAdvertisingType(u8),
    /// Invalid HCI command.
    #[error("Invalid HCI command with opcode {0}")]
    InvalidCommand(u16),
    /// The provided connection interval value is invalid, it needs to be between 0x0006 and 0x0C80.
    #[error(
        "The connection interval value {0} is invalid, it needs to be between 0x0006 and 0x0C80"
    )]
    InvalidConnectionIntervalValue(u16),
    /// Invalid or unhandled HCI error code.
    #[error("Invalid HCI error code {0}")]
    InvalidErrorCode(u8),
    /// Invalid HCI event packet.
    #[error("Invalid HCI event packet")]
    InvalidEventPacket,
    /// The provided own address type is invalid.
    #[error("The own address type {0} is invalid")]
    InvalidOwnAddressType(u8),
    /// Invalid HCI packet, either malformed or not expected (eg. Command received by the Host).
    #[error("Invalid HCI packet")]
    InvalidPacket,
    /// Invalid or unhandled HCI packet type.
    #[error("Invalid HCI packet type {0}")]
    InvalidPacketType(u8),
    /// The provided peer address type is invalid.
    #[error("The peer address type {0} is invalid")]
    InvalidPeerAddressType(u8),
    /// The provided TX power level value is invalid.
    #[error("The TX power level value {0} is invalid")]
    InvalidTxPowerLevelValue(i8),
}

#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum HciDriverError {
    #[error("HCI driver write failure")]
    WriteFailure,
    #[error("HCI driver read failure")]
    ReadFailure,
    #[error("HCI driver timeout")]
    Timeout,
}

pub trait HciDriver {
    fn read(&mut self, buf: &mut [u8]) -> impl Future<Output = Result<usize, HciDriverError>>;
    fn write(&mut self, buf: &[u8]) -> impl Future<Output = Result<usize, HciDriverError>>;
}

pub trait WithTimeout {
    type Output;

    fn with_timeout(
        self,
        timeout: u16,
    ) -> impl Future<Output = Result<Self::Output, HciDriverError>>;
}

#[derive(Debug, Clone, Default)]
struct HciBuffer {
    buffer: Buffer<HCI_MAX_READ_BUFFER_SIZE>,
}

impl HciBuffer {
    fn data(&self) -> &[u8] {
        self.buffer.data()
    }

    fn len(&self) -> usize {
        self.buffer.len()
    }

    async fn read<H: HciDriver>(&mut self, driver: &mut H) -> Result<(), HciDriverError> {
        let offset = self.len();
        self.buffer.offset += driver.read(&mut self.buffer.data[offset..]).await?;
        Ok(())
    }
}

impl TryFrom<&[u8]> for HciBuffer {
    type Error = UtilsError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self {
            buffer: value.try_into()?,
        })
    }
}

#[derive(Debug)]
pub struct Hci<H>
where
    H: HciDriver,
{
    driver: H,
    num_hci_command_packets: u8,
    read_buffer: HciBuffer,
}

impl<H> Hci<H>
where
    H: HciDriver,
{
    pub fn new(hci_driver: H) -> Self {
        Self {
            driver: hci_driver,
            num_hci_command_packets: 0,
            read_buffer: Default::default(),
        }
    }

    pub async fn cmd_set_event_mask(&mut self, event_mask: EventMask) -> Result<(), Error> {
        self.cmd_with_status_response(Command::SetEventMask(event_mask))
            .await
    }

    pub async fn cmd_reset(&mut self) -> Result<(), Error> {
        self.cmd_with_status_response(Command::Reset).await
    }

    pub async fn cmd_read_local_supported_commands(&mut self) -> Result<SupportedCommands, Error> {
        let event = self
            .execute_command(Command::ReadLocalSupportedCommands)
            .await?;
        match event.parameter {
            EventParameter::StatusAndSupportedCommands(param) if param.status.is_success() => {
                Ok(param.supported_commands)
            }
            EventParameter::StatusAndSupportedCommands(param) => {
                Err(Error::ErrorCode(param.status))
            }
            _ => Err(Error::InvalidEventPacket),
        }
    }

    pub async fn cmd_read_local_supported_features(&mut self) -> Result<SupportedFeatures, Error> {
        let event = self
            .execute_command(Command::ReadLocalSupportedFeatures)
            .await?;
        match event.parameter {
            EventParameter::StatusAndSupportedFeatures(param) if param.status.is_success() => {
                Ok(param.supported_features)
            }
            EventParameter::StatusAndSupportedFeatures(param) => {
                Err(Error::ErrorCode(param.status))
            }
            _ => Err(Error::InvalidEventPacket),
        }
    }

    pub async fn cmd_read_buffer_size(
        &mut self,
    ) -> Result<(NonZeroU16, NonZeroU8, NonZeroU16, u16), Error> {
        let event = self.execute_command(Command::ReadBufferSize).await?;
        match event.parameter {
            EventParameter::StatusAndBufferSize(param) if param.status.is_success() => Ok((
                param.acl_data_packet_length,
                param.synchronous_data_packet_length,
                param.total_num_acl_data_packets,
                param.total_num_synchronous_packets,
            )),
            EventParameter::StatusAndBufferSize(param) => Err(Error::ErrorCode(param.status)),
            _ => Err(Error::InvalidEventPacket),
        }
    }

    pub async fn cmd_le_read_advertising_channel_tx_power(
        &mut self,
    ) -> Result<TxPowerLevel, Error> {
        let event = self
            .execute_command(Command::LeReadAdvertisingChannelTxPower)
            .await?;
        match event.parameter {
            EventParameter::StatusAndTxPowerLevel(param) if param.status.is_success() => {
                Ok(param.tx_power_level)
            }
            EventParameter::StatusAndTxPowerLevel(param) => Err(Error::ErrorCode(param.status)),
            _ => Err(Error::InvalidEventPacket),
        }
    }

    pub async fn cmd_le_read_local_supported_features_page_0(
        &mut self,
    ) -> Result<SupportedLeFeatures, Error> {
        let event = self
            .execute_command(Command::LeReadLocalSupportedFeaturesPage0)
            .await?;
        match event.parameter {
            EventParameter::StatusAndSupportedLeFeatures(param) if param.status.is_success() => {
                Ok(param.supported_le_features)
            }
            EventParameter::StatusAndSupportedLeFeatures(param) => {
                Err(Error::ErrorCode(param.status))
            }
            _ => Err(Error::InvalidEventPacket),
        }
    }

    pub async fn cmd_le_read_buffer_size(&mut self) -> Result<(u16, u16), Error> {
        let event = self.execute_command(Command::LeReadBufferSize).await?;
        match event.parameter {
            EventParameter::StatusAndLeBufferSize(param) if param.status.is_success() => Ok((
                param.le_acl_data_packet_length,
                param.total_num_le_acl_data_packets as u16,
            )),
            EventParameter::StatusAndLeBufferSize(param) => Err(Error::ErrorCode(param.status)),
            _ => Err(Error::InvalidEventPacket),
        }
    }

    pub async fn cmd_le_read_supported_states(&mut self) -> Result<SupportedLeStates, Error> {
        let event = self.execute_command(Command::LeReadSupportedStates).await?;
        match event.parameter {
            EventParameter::StatusAndSupportedLeStates(param) if param.status.is_success() => {
                Ok(param.supported_le_states)
            }
            EventParameter::StatusAndSupportedLeStates(param) => {
                Err(Error::ErrorCode(param.status))
            }
            _ => Err(Error::InvalidEventPacket),
        }
    }

    pub async fn cmd_le_set_advertising_enable(
        &mut self,
        enable: AdvertisingEnable,
    ) -> Result<(), Error> {
        self.cmd_with_status_response(Command::LeSetAdvertisingEnable(enable))
            .await
    }

    pub async fn cmd_le_set_advertising_data(
        &mut self,
        data: AdvertisingData,
    ) -> Result<(), Error> {
        self.cmd_with_status_response(Command::LeSetAdvertisingData(data))
            .await
    }

    pub async fn cmd_le_set_advertising_parameters(
        &mut self,
        parameters: AdvertisingParameters,
    ) -> Result<(), Error> {
        self.cmd_with_status_response(Command::LeSetAdvertisingParameters(parameters))
            .await
    }

    pub async fn cmd_le_set_scan_response_data(
        &mut self,
        data: ScanResponseData,
    ) -> Result<(), Error> {
        self.cmd_with_status_response(Command::LeSetScanResponseData(data))
            .await
    }

    async fn cmd_with_status_response(&mut self, command: Command) -> Result<(), Error> {
        let event = self.execute_command(command).await?;
        match event.parameter {
            EventParameter::Status(param) if param.status.is_success() => Ok(()),
            EventParameter::Status(param) => Err(Error::ErrorCode(param.status)),
            _ => Err(Error::InvalidEventPacket),
        }
    }

    async fn execute_command(&mut self, command: Command) -> Result<CommandCompleteEvent, Error> {
        if self.num_hci_command_packets == 0 {
            self.wait_controller_ready().await?;
        }
        let event = self
            .send_command_and_wait_response(command)
            .with_timeout(HCI_COMMAND_TIMEOUT)
            .await??;
        self.num_hci_command_packets = event.num_hci_command_packets;
        Ok(event)
    }

    async fn send_command_and_wait_response(
        &mut self,
        command: Command,
    ) -> Result<CommandCompleteEvent, Error> {
        let command_packet = command.encode()?;
        self.driver.write(command_packet.data()).await?;
        loop {
            let (remaining, packet) = self.hci_read_and_parse_packet().await?;
            match packet {
                Packet::Command(_) => {
                    // The Host is not supposed to receive commands!
                    return Err(Error::InvalidPacket);
                }
                Packet::Event(event) => {
                    // INVARIANT: The remaining is known to be shorter than the buffer.
                    self.read_buffer = remaining.try_into().unwrap();

                    match event {
                        Event::CommandComplete(event) if event.opcode == command.opcode() => {
                            return Ok(event);
                        }
                        Event::CommandComplete(_) | Event::Unsupported(_) => {
                            self.handle_event(event)
                        }
                    }
                }
            }

            // TODO: Try to parse the remaining if there are some data
        }
    }

    async fn wait_controller_ready(&mut self) -> Result<(), Error> {
        while self.num_hci_command_packets == 0 {
            let (remaining, packet) = self.hci_read_and_parse_packet().await?;
            match packet {
                Packet::Command(_) => {
                    // The Host is not supposed to receive commands!
                    return Err(Error::InvalidPacket);
                }
                Packet::Event(event) => {
                    // INVARIANT: The remaining is known to be shorter than the buffer.
                    self.read_buffer = remaining.try_into().unwrap();

                    self.handle_event(event)
                }
            }

            // TODO: Try to parse the remaining if there are some data
        }
        Ok(())
    }

    async fn hci_read_and_parse_packet(&mut self) -> Result<(&[u8], Packet), Error> {
        self.read_buffer.read(&mut self.driver).await?;
        let (remaining, hci_packet) = crate::packet::parser::packet(self.read_buffer.data())
            .map_err(|_| Error::InvalidPacket)?;
        Ok((remaining, hci_packet))
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::CommandComplete(command_complete_event)
                if command_complete_event.opcode == CommandOpCode::Nop =>
            {
                self.num_hci_command_packets = command_complete_event.num_hci_command_packets;
            }
            Event::CommandComplete(_) => {
                unreachable!("an event for an issued command should already have been handled before reaching here")
            }
            Event::Unsupported(_event_code) => {
                // Ignore unsupported event
            }
        }
    }
}
