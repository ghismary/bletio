#![no_std]

extern crate alloc;

mod hci;

use core::cell::{BorrowMutError, RefCell};
use embedded_io::Error as EmbeddedIoError;

use crate::hci::command::Command;
use crate::hci::error_code::HciErrorCode;
use crate::hci::event::{CommandCompleteEvent, Event};
use crate::hci::event_parameter::{
    LeFeaturesEventParameter, StatusEventParameter, SupportedCommandsEventParameter,
};
use crate::hci::le_features::LeFeatures;
use crate::hci::opcode::OpCode;
use crate::hci::supported_commands::SupportedCommands;
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
    supported_le_features: LeFeatures,
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
            supported_le_features: LeFeatures::default(),
        }
    }

    pub fn init(&mut self) -> Result<(), Error> {
        self.cmd_reset()?;
        self.cmd_read_local_supported_commands()?;
        Ok(())
    }

    pub fn supported_commands(&self) -> &SupportedCommands {
        &self.supported_commands
    }

    pub fn supported_le_features(&self) -> &LeFeatures {
        &self.supported_le_features
    }

    fn execute_command(&self, command: Command) -> Result<CommandCompleteEvent, Error> {
        self.hci_write(command.encode().data())?;
        let event = self.hci_wait_for_command_complete(command.opcode())?;
        let status_event_parameter: StatusEventParameter =
            event.return_parameters.slice(0)?.try_into()?;
        match status_event_parameter.status {
            HciErrorCode::Success => Ok(event),
            _ => Err(Error::HciError(status_event_parameter.status)),
        }
    }

    fn cmd_reset(&self) -> Result<CommandCompleteEvent, Error> {
        self.execute_command(Command::Reset)
    }

    fn cmd_read_local_supported_commands(&mut self) -> Result<CommandCompleteEvent, Error> {
        let event = self.execute_command(Command::ReadLocalSupportedCommands)?;
        let supported_commands_event_parameter: SupportedCommandsEventParameter =
            event.return_parameters.slice(1)?[..64].try_into()?;
        self.supported_commands = supported_commands_event_parameter.value;
        Ok(event)
    }

    fn cmd_read_le_local_supported_features(&mut self) -> Result<CommandCompleteEvent, Error> {
        let event = self.execute_command(Command::LeReadLocalSupportedFeatures)?;
        let le_features_event_parameter: LeFeaturesEventParameter =
            event.return_parameters.slice(1)?[..8].try_into()?;
        self.supported_le_features = le_features_event_parameter.value;
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
enum HciPollResult {
    AclData,
    Event(Event),
}
