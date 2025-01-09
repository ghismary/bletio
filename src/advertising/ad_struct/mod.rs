mod flags;
mod manufacturer_specific_data;
mod peripheral_connection_interval_range;
mod service_uuid;
mod tx_power_level;

use bitflags::bitflags;
pub use flags::FlagsAdStruct;
pub use manufacturer_specific_data::ManufacturerSpecificDataAdStruct;
pub use peripheral_connection_interval_range::PeripheralConnectionIntervalRangeAdStruct;
pub use service_uuid::{
    ServiceListCompletion, ServiceUuid128AdStruct, ServiceUuid16AdStruct, ServiceUuid32AdStruct,
};
pub use tx_power_level::TxPowerLevelAdStruct;

pub(crate) const AD_STRUCT_LENGTH_OFFSET: usize = 0;
pub(crate) const AD_STRUCT_TYPE_OFFSET: usize = 1;
pub(crate) const AD_STRUCT_DATA_OFFSET: usize = 2;

pub(crate) trait AdStruct {
    fn data(&self) -> &[u8];
    fn r#type(&self) -> AdStructType;
    fn unique(&self) -> bool;
}

#[derive(Debug, Default, Copy, Clone)]
pub struct AdStructType(u8);

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
