//! Advertising structures contained in [AdvertisingData](crate::advertising::AdvertisingData)
//! and [ScanResponseData](crate::advertising::ScanResponseData).
//!
//! The format for the [AdvertisingData](crate::advertising::AdvertisingData) and
//! [ScanResponseData](crate::advertising::ScanResponseData) is defined in the
//! [Core Specification 6.0, Vol.3, Part C, 11](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-51247611-bdce-274e-095c-afb6d879c55c).
//!
//! The formats of each advertising structures and their meanings are defined in the
//! [Core Specification Supplement, Part A, 1](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-36b7e551-d4cf-9ae3-a8ee-0482fbc1d5bc).

pub(crate) mod flags;
mod manufacturer_specific_data;
mod peripheral_connection_interval_range;
mod service_uuid;
pub(crate) mod tx_power_level;

use bitflags::bitflags;
pub use flags::FlagsAdStruct;
pub use manufacturer_specific_data::ManufacturerSpecificDataAdStruct;
pub use peripheral_connection_interval_range::PeripheralConnectionIntervalRangeAdStruct;
pub use service_uuid::{
    ServiceListComplete, ServiceUuid128AdStruct, ServiceUuid16AdStruct, ServiceUuid32AdStruct,
};
pub use tx_power_level::TxPowerLevelAdStruct;

pub(crate) const AD_STRUCT_LENGTH_OFFSET: usize = 0;
pub(crate) const AD_STRUCT_TYPE_OFFSET: usize = 1;
pub(crate) const AD_STRUCT_DATA_OFFSET: usize = 2;

pub(crate) trait AdStruct {
    fn encoded_data(&self) -> &[u8];
    fn r#type(&self) -> AdStructType;
    fn is_unique(&self) -> bool;
}

#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct AdStructType(u8);

bitflags! {
    impl AdStructType: u8 {
        const SERVICE_UUID16 = 1 << 0;
        const SERVICE_UUID32 = 1 << 1;
        const SERVICE_UUID128 = 1 << 2;
        const FLAGS = 1 << 3;
        const MANUFACTURER_SPECIFIC_DATA = 1 << 4;
        const TX_POWER_LEVEL = 1 << 5;
        const PERIPHERAL_CONNECTION_INTERVAL_RANGE = 1 << 6;
    }
}
