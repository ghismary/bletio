//! Advertising parameters and data handling.
//!
//! This module gives access to all that is need to start advertising:
//!  - definition of the [advertising parameters](advertising_parameters)
//!  - definition of all the [advertising structures](ad_struct) to be used in the [`AdvertisingData`] or [`ScanResponseData`] packets.

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
pub use ad_struct::peripheral_connection_interval_range::{
    peripheral_connection_interval_range, PeripheralConnectionInterval,
    PeripheralConnectionIntervalRange,
};
pub use ad_struct::service_uuid::ServiceListComplete;
pub use ad_struct::AdStruct;
pub use advertising_data::{
    AdvertisingData, AdvertisingDataBuilder, FullAdvertisingData, ScanResponseData,
    ScanResponseDataBuilder,
};
pub use advertising_parameters::{AdvertisingParameters, AdvertisingParametersBuilder};
pub use scan_parameters::{ScanParameters, ScanParametersBuilder};
pub use uri::{custom_uri_scheme, CustomUriScheme, Uri, UriScheme};

/// Error occurring in the advertising part of the BLE stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdvertisingError {
    /// The provided advertising data is too big to fit in an advertising data or scan response data packet.
    AdvertisingDataWillNotFitAdvertisingPacket,
    /// The Appearance Advertising Structure is not allowed to be present in both the Advertising Data and the Scan Response Data.
    AppearanceNotAllowedInBothAdvertisingDataAndScanResponseData,
    /// An empty service UUID list Advertising Structure needs to be complete.
    EmptyServiceUuidListShallBeComplete,
    /// Only one advertising interval Advertising Structure is allowed in an Advertising Data or Scan Response Data block.
    OnlyOneAdvertisingIntervalAllowedInAdvertisingDataOrScanResponseData,
    /// Only one appearance Advertising Structure is allowed in an Advertising Data or Scan Response Data block.
    OnlyOneAppearanceAllowedInAdvertisingDataOrScanResponseData,
    /// Only one flags Advertising Structure is allowed in an Advertising Data block.
    OnlyOneFlagsAllowedInAdvertisingData,
    /// Only one LE supported features Advertising Structure is allowed in an Advertising Data or Scan Response Data block.
    OnlyOneLeSupportedFeaturesAllowedInAdvertisingDataOrScanResponseData,
    /// Only one local name Advertising Structure is allowed in an Advertising Data or Scan Response Data block.
    OnlyOneLocalNameAllowedInAdvertisingDataOrScanResponseData,
    /// Only one public target address Advertising Structure is allowed in an Advertising Data or Scan Response Data block.
    OnlyOnePublicTargetAddressAllowedInAdvertisingDataOrScanResponseData,
    /// Only one random target address Advertising Structure is allowed in an Advertising Data or Scan Response Data block.
    OnlyOneRandomTargetAddressAllowedInAdvertisingDataOrScanResponseData,
    /// The Public Target Address Advertising Structure must contain at least one address.
    PublicTargetAddressAdStructMustContainAtLeastOneAddress,
    /// The Random Target Address Advertising Structure must contain at least one address.
    RandomTargetAddressAdStructMustContainAtLeastOneAddress,
    /// The provided Advertising Type value is invalid.
    InvalidAdTypeValue(u8),
    /// The advertising parameters are not valid, probably because the advertising type is ScannableUndirected or NonConnectableUndirected, and the minimum advertising interval value is less than 0x00A0.
    InvalidAdvertisingParameters,
    /// The provided Appearance value is invalid.
    InvalidAppearanceValue(u16),
    /// The provided Company Identifier value is invalid.
    InvalidCompanyIdentifierValue(u16),
    /// The peripheral connection interval range is invalid.
    InvalidPeripheralConnectionIntervalRange,
    /// The provided peripheral connection interval value is invalid.
    InvalidPeripheralConnectionIntervalValue(u16),
    /// The provided Provisioned Uri Scheme value is invalid.
    InvalidProvisionedUriSchemeValue(u16),
    /// The scan parameters are not valid, probably because the scan window is larger than the scan interval.
    InvalidScanParameters,
    /// The provided Service Uuid value is invalid.
    InvalidServiceUuidValue(u16),
}
