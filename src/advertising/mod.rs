//! Advertising parameters and data handling.
//!
//! This module gives access to all that is need to start advertising:
//!  - definition of the [advertising parameters](crate::advertising::advertising_parameters)
//!  - definition of all the [advertising structures](crate::advertising::ad_struct) to be used in the [`AdvertisingData`] or [`ScanResponseData`] packets.

pub mod ad_struct;
pub mod advertising_data;
pub mod advertising_parameters;

pub use ad_struct::flags::Flags;
pub use ad_struct::tx_power_level::TxPowerLevel;
pub use advertising_data::{AdvertisingData, ScanResponseData};

/// Error occuring in the advertising part of the BLE stack.
#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdvertisingError {
    /// The advertising structure must be present only once in the advertising data or scan response data.
    #[error("The advertising structure must be present only once in the advertising data or scan response data")]
    AdStructAlreadyPresent,
    /// The provided advertising data is too big to fit in an advertising data or scan response data packet.
    #[error("The provided advertising data is too big to fit in an advertising data or scan response data packet")]
    AdvertisingDataWillNotFitAdvertisingPacket,
    /// An empty service UUID list Advertising Structure needs to be complete.
    #[error("An empty service UUID list Advertising Structure needs to be complete")]
    EmptyServiceUuidListShallBeComplete,
    /// Internal error.
    #[error("Internal advertising error: {0}")]
    Internal(&'static str),
    /// The provided advertising interval value is invalid, it needs to be between 0x0020 and 0x4000.
    #[error(
        "The advertising interval value {0} is invalid, it needs to be between 0x0020 and 0x4000"
    )]
    InvalidAdvertisingIntervalValue(u16),
    /// The advertising parameters are not valid, probably because the advertising type is ScannableUndirected or NonConnectableUndirected, and the minimum advertising interval value is less than 0x00A0.
    #[error("The advertising parameters are not valid, probably because the advertising type is ScannableUndirected or NonConnectableUndirected, and the minimum advertising interval value is less than 0x00A0")]
    InvalidAdvertisingParameters,
}

/// Enable/disable advertising.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.9](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e58c6816-c25e-367a-0023-9da1700a3794).
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
#[non_exhaustive]
pub(crate) enum AdvertisingEnable {
    /// Advertising is disabled (default).
    Disabled = 0x00,
    /// Advertising is enabled.
    Enabled = 0x01,
}
