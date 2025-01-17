pub(crate) mod command;
mod error_code;
pub(crate) mod event;
pub(crate) mod event_mask;
pub(crate) mod event_parameter;
pub(crate) mod opcode;
pub(crate) mod supported_commands;
mod supported_features;
mod supported_le_features;
pub(crate) mod supported_le_states;

pub use error_code::HciErrorCode;
pub use supported_features::SupportedFeatures;
pub use supported_le_features::SupportedLeFeatures;

/// Error occuring in the HCI part of the BLE stack.
#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum HciError {
    /// HCI access is denied.
    #[error("HCI access is denied")]
    AccessDenied,
    /// The provided data is too big to fit in an HCI command packet.
    #[error("The provided data is too big to fit in an HCI command packet")]
    DataWillNotFitCommandPacket,
    /// HCI error code.
    #[error("HCI error code {0:?}")]
    ErrorCode(HciErrorCode), // TODO: Should it be exposed to the user?
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

macro_rules! hci_packet_types {
    (
        $(
            $(#[$docs:meta])*
            $packet_type:ident = $value:expr,
        )+
    ) => {
        /// HCI packet type.
        ///
        /// HCI does not provide the ability to differentiate the five HCI packet types. Therefore, if
        /// the HCI packets are sent via a common physical interface, an HCI packet indicator has
        /// to be added. This is this packet type.
        ///
        /// See [Core Specification 6.0, Vol. 4, Part A, 2](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/uart-transport-layer.html#UUID-361053ee-862f-c591-00bd-1a941a12f949).
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr(u8)]
        #[non_exhaustive]
        pub(crate) enum PacketType {
            $(
                $(#[$docs])*
                $packet_type = $value,
            )+
        }

        impl TryFrom<u8> for PacketType {
            type Error = HciError;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    $(
                        $value => Ok(PacketType::$packet_type),
                    )+
                    _ => Err(HciError::InvalidPacketType(value)),
                }
            }
        }
    };
}

hci_packet_types! {
    Command = 0x01,
    AclData = 0x02,
    SynchronousData = 0x03,
    Event = 0x04,
    IsoData = 0x05,
}
