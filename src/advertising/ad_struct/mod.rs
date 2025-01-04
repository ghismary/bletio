mod common_data_types;
mod flags;
mod service_uuid;

use crate::Error;
pub use flags::FlagsAdStruct;
pub use service_uuid::{ServiceUuid128AdStruct, ServiceUuid16AdStruct, ServiceUuid32AdStruct};

#[derive(Debug)]
pub enum AdStruct {
    Flags(FlagsAdStruct),
    ServiceUuid16(ServiceUuid16AdStruct),
    ServiceUuid32(ServiceUuid32AdStruct),
    ServiceUuid128(ServiceUuid128AdStruct),
}

impl AdStruct {
    pub(crate) fn size(&self) -> usize {
        match self {
            AdStruct::Flags(value) => value.size(),
            AdStruct::ServiceUuid16(value) => value.size(),
            AdStruct::ServiceUuid32(value) => value.size(),
            AdStruct::ServiceUuid128(value) => value.size(),
        }
    }

    pub(crate) fn encode(&self, buffer: &mut [u8]) -> Result<usize, Error> {
        match self {
            AdStruct::Flags(value) => value.encode(buffer),
            AdStruct::ServiceUuid16(value) => value.encode(buffer),
            AdStruct::ServiceUuid32(value) => value.encode(buffer),
            AdStruct::ServiceUuid128(value) => value.encode(buffer),
        }
    }
}
