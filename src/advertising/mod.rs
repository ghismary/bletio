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
