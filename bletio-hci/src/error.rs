use crate::{ErrorCode, HciDriverError};

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
    /// The provided public device address is invalid.
    #[error("The public device address is invalid.")]
    InvalidPublicDeviceAddress,
    /// The provided random address is invalid.
    #[error("The random address is invalid.")]
    InvalidRandomAddress,
    /// The provided random non-resolvable private address is invalid.
    #[error("The random non-resolvable private address is invalid.")]
    InvalidRandomNonResolvablePrivateAddress,
    /// The provided random resolvable private address is invalid.
    #[error("The random resolvable private address is invalid.")]
    InvalidRandomResolvablePrivateAddress,
    /// The provided random static device address is invalid.
    #[error("The random static device address is invalid")]
    InvalidRandomStaticDeviceAddress,
    /// The provided TX power level value is invalid.
    #[error("The TX power level value {0} is invalid")]
    InvalidTxPowerLevelValue(i8),
}
