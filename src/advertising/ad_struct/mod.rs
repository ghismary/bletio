mod common_data_types;
mod flags;
mod service_uuid;
mod tx_power_level;

use bitflags::bitflags;
pub use flags::FlagsAdStruct;
pub use service_uuid::{ServiceUuid128AdStruct, ServiceUuid16AdStruct, ServiceUuid32AdStruct};
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
        const FLAGS = 1 << 0;
        const SERVICE_UUID16 = 1 << 1;
        const SERVICE_UUID32 = 1 << 2;
        const SERVICE_UUID128 = 1 << 3;
        const TX_POWER_LEVEL = 1 << 4;
    }
}
