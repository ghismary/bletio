pub(crate) mod command;
mod error_code;
pub(crate) mod event;
pub(crate) mod event_mask;
pub(crate) mod event_parameter;
pub(crate) mod opcode;
pub(crate) mod supported_commands;
pub(crate) mod supported_features;
pub(crate) mod supported_le_features;
pub(crate) mod supported_le_states;

pub use error_code::HciErrorCode;

/// Error occuring in the HCI part of the BLE stack.
#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum HciError {
    /// HCI access is denied.
    #[error("HCI access is denied")]
    AccessDenied,
    /// HCI error code.
    #[error("HCI error code {0:?}")]
    ErrorCode(HciErrorCode), // TODO: Should it be exposed to the user?
    /// Internal error.
    #[error("HCI internal error: {0}")]
    Internal(&'static str),
    /// Invalid or unhandled HCI error code.
    #[error("Invalid HCI error code {0}")]
    InvalidErrorCode(u8),
    /// Invalid or unhandled HCI event code.
    #[error("Invalid HCI event code {0}")]
    InvalidEventCode(u8),
    /// Invalid HCI event packet.
    #[error("Invalid HCI event packet")]
    InvalidEventPacket,
    /// Invalid or unhandled HCI OpCode.
    #[error("Invalid HCI OpCode {0}")]
    InvalidOpcode(u16),
    /// Invalid or unhandled HCI packet type.
    #[error("Invalid HCI packet type {0}")]
    InvalidPacketType(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum PacketType {
    Command = 0x01,
    AclData = 0x02,
    SynchronousData = 0x03,
    Event = 0x04,
}

impl TryFrom<u8> for PacketType {
    type Error = HciError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(PacketType::Command),
            0x02 => Ok(PacketType::AclData),
            0x03 => Ok(PacketType::SynchronousData),
            0x04 => Ok(PacketType::Event),
            _ => Err(HciError::InvalidPacketType(value)),
        }
    }
}
