//! Advertising parameters and data handling.
//!
//! This module gives access to all that is need to start advertising:
//!  - definition of the [advertising parameters](crate::advertising::advertising_parameters)
//!  - definition of all the [advertising structures](crate::advertising::ad_struct) to be used in the [`AdvertisingData`] or [`ScanResponseData`] packets.

pub use bletio_hci::{
    AdvertisingChannelMap, AdvertisingEnable, AdvertisingFilterPolicy, AdvertisingInterval,
    AdvertisingIntervalRange, AdvertisingType,
};

mod ad_struct;

pub mod advertising_data;
pub mod advertising_parameters;
pub mod scan_parameters;
pub mod uri;

pub use ad_struct::flags::Flags;
pub use ad_struct::local_name::LocalNameComplete;
pub use ad_struct::service_uuid::ServiceListComplete;
pub use advertising_data::{
    AdvertisingData, AdvertisingDataBuilder, FullAdvertisingData, ScanResponseData,
    ScanResponseDataBuilder,
};
pub use advertising_parameters::{AdvertisingParameters, AdvertisingParametersBuilder};
pub use scan_parameters::{ScanParameters, ScanParametersBuilder};
pub use uri::{custom_uri_scheme, CustomUriScheme, Uri, UriScheme};

/// Error occuring in the advertising part of the BLE stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdvertisingError {
    /// The provided advertising data is too big to fit in an advertising data or scan response data packet.
    AdvertisingDataWillNotFitAdvertisingPacket,
    /// The Appearance Advertising Structure is not allowed to be present in both the Advertising Data and the Scan Response Data.
    AppearanceNotAllowedInBothAdvertisingDataAndScanResponseData,
    /// An empty service UUID list Advertising Structure needs to be complete.
    EmptyServiceUuidListShallBeComplete,
    /// The Public Target Address Advertising Structure must contain at least one address.
    PublicTargetAddressAdStructMustContainAtLeastOneAddress,
    /// The Random Target Address Advertising Structure must contain at least one address.
    RandomTargetAddressAdStructMustContainAtLeastOneAddress,
    /// The advertising parameters are not valid, probably because the advertising type is ScannableUndirected or NonConnectableUndirected, and the minimum advertising interval value is less than 0x00A0.
    InvalidAdvertisingParameters,
    /// The scan parameters are not valid, probably because the scan window is larger than the scan interval.
    InvalidScanParameters,
}
