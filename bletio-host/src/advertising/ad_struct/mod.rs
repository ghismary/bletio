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
use bletio_utils::EncodeToBuffer;
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

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdStruct {
    AdvertisingInterval(AdvertisingIntervalAdStruct),
    Appearance(AppearanceAdStruct),
    Flags(FlagsAdStruct),
    LeSupportedFeatures(LeSupportedFeaturesAdStruct),
    LocalName(LocalNameAdStruct),
    ManufacturerSpecificData(ManufacturerSpecificDataAdStruct),
    PeripheralConnectionIntervalRange(PeripheralConnectionIntervalRangeAdStruct),
    PublicTargetAddress(PublicTargetAddressAdStruct),
    RandomTargetAddress(RandomTargetAddressAdStruct),
    ServiceDataUuid16(ServiceDataUuid16AdStruct),
    ServiceDataUuid32(ServiceDataUuid32AdStruct),
    ServiceDataUuid128(ServiceDataUuid128AdStruct),
    ServiceSolicitationUuid16(ServiceSolicitationUuid16AdStruct),
    ServiceSolicitationUuid32(ServiceSolicitationUuid32AdStruct),
    ServiceSolicitationUuid128(ServiceSolicitationUuid128AdStruct),
    ServiceUuid16(ServiceUuid16AdStruct),
    ServiceUuid32(ServiceUuid32AdStruct),
    ServiceUuid128(ServiceUuid128AdStruct),
    TxPowerLevel(TxPowerLevelAdStruct),
    Uri(UriAdStruct),
    Unhandled(u8),
}

impl EncodeToBuffer for AdStruct {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        match self {
            AdStruct::AdvertisingInterval(v) => v.encode(buffer),
            AdStruct::Appearance(v) => v.encode(buffer),
            AdStruct::Flags(v) => v.encode(buffer),
            AdStruct::LeSupportedFeatures(v) => v.encode(buffer),
            AdStruct::LocalName(v) => v.encode(buffer),
            AdStruct::ManufacturerSpecificData(v) => v.encode(buffer),
            AdStruct::PeripheralConnectionIntervalRange(v) => v.encode(buffer),
            AdStruct::PublicTargetAddress(v) => v.encode(buffer),
            AdStruct::RandomTargetAddress(v) => v.encode(buffer),
            AdStruct::ServiceDataUuid16(v) => v.encode(buffer),
            AdStruct::ServiceDataUuid32(v) => v.encode(buffer),
            AdStruct::ServiceDataUuid128(v) => v.encode(buffer),
            AdStruct::ServiceSolicitationUuid16(v) => v.encode(buffer),
            AdStruct::ServiceSolicitationUuid32(v) => v.encode(buffer),
            AdStruct::ServiceSolicitationUuid128(v) => v.encode(buffer),
            AdStruct::ServiceUuid16(v) => v.encode(buffer),
            AdStruct::ServiceUuid32(v) => v.encode(buffer),
            AdStruct::ServiceUuid128(v) => v.encode(buffer),
            AdStruct::TxPowerLevel(v) => v.encode(buffer),
            AdStruct::Uri(v) => v.encode(buffer),
            AdStruct::Unhandled(_) => Err(bletio_utils::Error::CannotEncode),
        }
    }

    fn encoded_size(&self) -> usize {
        match self {
            AdStruct::AdvertisingInterval(v) => v.encoded_size(),
            AdStruct::Appearance(v) => v.encoded_size(),
            AdStruct::Flags(v) => v.encoded_size(),
            AdStruct::LeSupportedFeatures(v) => v.encoded_size(),
            AdStruct::LocalName(v) => v.encoded_size(),
            AdStruct::ManufacturerSpecificData(v) => v.encoded_size(),
            AdStruct::PeripheralConnectionIntervalRange(v) => v.encoded_size(),
            AdStruct::PublicTargetAddress(v) => v.encoded_size(),
            AdStruct::RandomTargetAddress(v) => v.encoded_size(),
            AdStruct::ServiceDataUuid16(v) => v.encoded_size(),
            AdStruct::ServiceDataUuid32(v) => v.encoded_size(),
            AdStruct::ServiceDataUuid128(v) => v.encoded_size(),
            AdStruct::ServiceSolicitationUuid16(v) => v.encoded_size(),
            AdStruct::ServiceSolicitationUuid32(v) => v.encoded_size(),
            AdStruct::ServiceSolicitationUuid128(v) => v.encoded_size(),
            AdStruct::ServiceUuid16(v) => v.encoded_size(),
            AdStruct::ServiceUuid32(v) => v.encoded_size(),
            AdStruct::ServiceUuid128(v) => v.encoded_size(),
            AdStruct::TxPowerLevel(v) => v.encoded_size(),
            AdStruct::Uri(v) => v.encoded_size(),
            AdStruct::Unhandled(_) => 0,
        }
    }
}
