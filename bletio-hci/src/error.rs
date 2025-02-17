use crate::{ErrorCode, HciDriverError};

/// Error occuring in the HCI part of the BLE stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// At least one channel must be enabled in the advertising channel map.
    AtLeastOneChannelMustBeEnabledInTheAdvertisingChannelMap,
    /// The provided data is too big to fit in an HCI command packet.
    DataWillNotFitCommandPacket,
    /// HCI error code.
    ErrorCode(ErrorCode),
    HciDriver(HciDriverError),
    /// The provided advertising enable value is invalid.
    InvalidAdvertisingEnableValue(u8),
    /// The provided advertising filter policy is invalid.
    InvalidAdvertisingFilterPolicy(u8),
    /// The provided advertising interval value is invalid, it needs to be between 0x0020 and 0x4000.
    InvalidAdvertisingInterval(u16),
    /// The advertising interval range is invalid, the first value must be smaller or equal to the second one.
    InvalidAdvertisingIntervalRange,
    /// The provided advertising type is invalid.
    InvalidAdvertisingType(u8),
    /// Invalid HCI command.
    InvalidCommand(u16),
    /// The provided connection interval value is invalid, it needs to be between 0x0006 and 0x0C80.
    InvalidConnectionIntervalValue(u16),
    /// Invalid or unhandled HCI error code.
    InvalidErrorCode(u8),
    /// Invalid HCI event packet.
    InvalidEventPacket,
    /// The provided filter duplicates value is invalid.
    InvalidFilterDuplicatesValue(u8),
    /// The provided LE advertising report address type is invalid.
    InvalidLeAdvertisingReportAddressType(u8),
    /// The provided LE advertising report event type is invalid.
    InvalidLeAdvertisingReportEventType(u8),
    /// The provided LE advertising report num reports is invalid.
    InvalidLeAdvertisingReportNumReports(u8),
    /// The provided own address type is invalid.
    InvalidOwnAddressType(u8),
    /// Invalid HCI packet, either malformed or not expected (eg. Command received by the Host).
    InvalidPacket,
    /// Invalid or unhandled HCI packet type.
    InvalidPacketType(u8),
    /// The provided peer address type is invalid.
    InvalidPeerAddressType(u8),
    /// The provided public device address is invalid.
    InvalidPublicDeviceAddress,
    /// The provided random address is invalid.
    InvalidRandomAddress,
    /// The provided random non-resolvable private address is invalid.
    InvalidRandomNonResolvablePrivateAddress,
    /// The provided random resolvable private address is invalid.
    InvalidRandomResolvablePrivateAddress,
    /// The provided random static device address is invalid.
    InvalidRandomStaticDeviceAddress,
    /// The provided RSSI value is invalid.
    InvalidRssiValue(i8),
    /// The provided scan enable value is invalid.
    InvalidScanEnableValue(u8),
    /// The provided scan interval is invalid, it needs to be between 0x0004 and 0x4000.
    InvalidScanInterval(u16),
    /// The provided scan type is invalid.
    InvalidScanType(u8),
    /// The provided scan window is invalid, it needs to be between 0x0004 and 0x4000.
    InvalidScanWindow(u16),
    /// The provided scanning filter policy is invalid.
    InvalidScanningFilterPolicy(u8),
    /// The provided TX power level value is invalid.
    InvalidTxPowerLevelValue(i8),
    /// The scan window must be smaller or equal to the scan interval.
    ScanWindowMustBeSmallerOrEqualToScanInterval,
}

impl From<HciDriverError> for Error {
    fn from(value: HciDriverError) -> Self {
        Self::HciDriver(value)
    }
}
