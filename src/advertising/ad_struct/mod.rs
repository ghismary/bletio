mod common_data_types;
mod flags;
mod service_uuid;

pub use flags::FlagsAdStruct;
pub use service_uuid::{ServiceUuid128AdStruct, ServiceUuid16AdStruct, ServiceUuid32AdStruct};

pub(crate) const AD_STRUCT_LENGTH_OFFSET: usize = 0;
pub(crate) const AD_STRUCT_TYPE_OFFSET: usize = 1;
pub(crate) const AD_STRUCT_DATA_OFFSET: usize = 2;

pub trait AdStruct {
    fn data(&self) -> &[u8];
    fn r#type(&self) -> AdStructType;
    fn unique(&self) -> bool;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AdStructType {
    Flags,
    ServiceUuid16,
    ServiceUuid32,
    ServiceUuid128,
}
