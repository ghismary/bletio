//! Advertising structures contained in [AdvertisingData](crate::advertising::AdvertisingData)
//! and [ScanResponseData](crate::advertising::ScanResponseData).
//!
//! The format for the [AdvertisingData](crate::advertising::AdvertisingData) and
//! [ScanResponseData](crate::advertising::ScanResponseData) is defined in the
//! [Core Specification 6.0, Vol.3, Part C, 11](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-51247611-bdce-274e-095c-afb6d879c55c).
//!
//! The formats of each advertising structures and their meanings are defined in the
//! [Core Specification Supplement, Part A, 1](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-36b7e551-d4cf-9ae3-a8ee-0482fbc1d5bc).

pub(crate) mod advertising_interval;
pub(crate) mod appearance;
pub(crate) mod flags;
pub(crate) mod le_supported_features;
pub(crate) mod local_name;
pub(crate) mod manufacturer_specific_data;
pub(crate) mod peripheral_connection_interval_range;
pub(crate) mod public_target_address;
pub(crate) mod random_target_address;
pub(crate) mod service_data;
pub(crate) mod service_solicitation;
pub(crate) mod service_uuid;
pub(crate) mod tx_power_level;
pub(crate) mod uri;

pub(crate) use advertising_interval::AdvertisingIntervalAdStruct;
pub(crate) use appearance::AppearanceAdStruct;
pub(crate) use flags::FlagsAdStruct;
pub(crate) use le_supported_features::LeSupportedFeaturesAdStruct;
pub(crate) use local_name::LocalNameAdStruct;
pub(crate) use manufacturer_specific_data::ManufacturerSpecificDataAdStruct;
pub(crate) use peripheral_connection_interval_range::PeripheralConnectionIntervalRangeAdStruct;
pub(crate) use public_target_address::PublicTargetAddressAdStruct;
pub(crate) use random_target_address::RandomTargetAddressAdStruct;
pub(crate) use service_data::{
    ServiceDataUuid128AdStruct, ServiceDataUuid16AdStruct, ServiceDataUuid32AdStruct,
};
pub(crate) use service_solicitation::{
    ServiceSolicitationUuid128AdStruct, ServiceSolicitationUuid16AdStruct,
    ServiceSolicitationUuid32AdStruct,
};
pub(crate) use service_uuid::{
    ServiceUuid128AdStruct, ServiceUuid16AdStruct, ServiceUuid32AdStruct,
};
pub(crate) use tx_power_level::TxPowerLevelAdStruct;
pub(crate) use uri::UriAdStruct;
