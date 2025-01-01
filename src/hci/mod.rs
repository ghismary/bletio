pub(crate) mod command;
pub mod error_code;
pub(crate) mod event;
pub(crate) mod event_mask;
pub(crate) mod event_parameter;
pub(crate) mod opcode;
pub(crate) mod supported_commands;
pub(crate) mod supported_features;
pub(crate) mod supported_le_features;
pub(crate) mod supported_le_states;

use crate::Error;

#[derive(Debug)]
#[repr(u8)]
pub enum PacketType {
    Command = 0x01,
    AclData = 0x02,
    SynchronousData = 0x03,
    Event = 0x04,
}

impl TryFrom<u8> for PacketType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(PacketType::Command),
            0x02 => Ok(PacketType::AclData),
            0x03 => Ok(PacketType::SynchronousData),
            0x04 => Ok(PacketType::Event),
            _ => Err(Error::InvalidPacketType(value)),
        }
    }
}
