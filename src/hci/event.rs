use core::cell::RefCell;
use embedded_io::Error as EmbeddedIoError;

use crate::hci::opcode::OpCode;
use crate::Error;

#[derive(Debug)]
enum EventCode {
    CommandComplete = 0x0E,
}

impl TryFrom<u8> for EventCode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0E => Ok(EventCode::CommandComplete),
            _ => Err(Error::InvalidEventCode(value)),
        }
    }
}

#[derive(Debug)]
pub(crate) struct CommandCompleteEvent {
    num_hci_command_packets: u8,
    pub(crate) opcode: OpCode,
    pub(crate) return_parameters: EventParameterData,
}

impl TryFrom<EventPacket> for CommandCompleteEvent {
    type Error = Error;

    fn try_from(value: EventPacket) -> Result<Self, Self::Error> {
        if value.parameter_total_length < 3 {
            return Err(Error::InvalidEventPacket);
        }
        let return_parameters_length = value.parameter_total_length - 3;
        Ok(CommandCompleteEvent {
            num_hci_command_packets: value.data[0],
            opcode: (((value.data[2] as u16) << 8) + value.data[1] as u16).try_into()?,
            return_parameters: value.data[3..3 + return_parameters_length].into(),
        })
    }
}

#[derive(Debug)]
pub(crate) enum Event {
    CommandComplete(CommandCompleteEvent),
}

impl Event {
    pub(crate) fn read<T>(hci: &RefCell<T>) -> Result<Event, Error>
    where
        T: embedded_io::Read,
    {
        let event_packet = EventPacket::read(hci)?;
        match event_packet.event_code {
            EventCode::CommandComplete => Ok(Event::CommandComplete(event_packet.try_into()?)),
        }
    }
}

#[derive(Debug)]
pub(crate) struct EventPacket {
    event_code: EventCode,
    parameter_total_length: usize,
    data: [u8; 255],
}

impl EventPacket {
    fn read<T>(hci: &RefCell<T>) -> Result<Self, Error>
    where
        T: embedded_io::Read,
    {
        let mut event_code = [0u8];
        EventPacket::hci_read(hci, &mut event_code)?;
        let mut parameter_total_length = [0u8];
        EventPacket::hci_read(hci, &mut parameter_total_length)?;

        let mut event_packet = Self {
            event_code: event_code[0].try_into()?,
            parameter_total_length: parameter_total_length[0].into(),
            data: [0; 255],
        };
        EventPacket::hci_read(
            hci,
            &mut event_packet.data[..event_packet.parameter_total_length],
        )?;

        Ok(event_packet)
    }

    fn hci_read<T>(hci: &RefCell<T>, buffer: &mut [u8]) -> Result<usize, Error>
    where
        T: embedded_io::Read,
        <T as embedded_io::ErrorType>::Error: embedded_io::Error,
    {
        if buffer.is_empty() {
            Ok(0)
        } else {
            hci.try_borrow_mut()?
                .read(buffer)
                .map_err(|err| Error::IO(err.kind()))
        }
    }
}

#[derive(Debug)]
pub(crate) struct EventParameterData {
    len: usize,
    data: [u8; 255],
}

impl EventParameterData {
    pub(crate) fn slice(&self, offset: usize) -> Result<&[u8], Error> {
        if offset > self.len {
            Err(Error::InvalidEventPacket)
        } else {
            Ok(&self.data[offset..])
        }
    }

    pub(crate) fn le_u16(&self, offset: usize) -> Result<u16, Error> {
        if offset + 2 > self.len {
            Err(Error::InvalidEventPacket)
        } else {
            Ok((self.data[offset + 1] as u16) << 8 | (self.data[offset] as u16))
        }
    }

    pub(crate) fn u8(&self, offset: usize) -> Result<u8, Error> {
        if offset + 1 > self.len {
            Err(Error::InvalidEventPacket)
        } else {
            Ok(self.data[offset])
        }
    }
}

impl From<&[u8]> for EventParameterData {
    fn from(value: &[u8]) -> Self {
        let mut data = EventParameterData {
            len: value.len(),
            data: [0; 255],
        };
        data.data[..value.len()].copy_from_slice(value);
        data
    }
}
