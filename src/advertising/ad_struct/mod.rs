pub mod service_uuid;

use service_uuid::{ServiceUuid128AdStruct, ServiceUuid16AdStruct, ServiceUuid32AdStruct};

#[derive(Debug)]
pub enum AdStruct {
    ServiceUuid16(ServiceUuid16AdStruct),
    ServiceUuid32(ServiceUuid32AdStruct),
    ServiceUuid128(ServiceUuid128AdStruct),
}

impl AdStruct {
    pub(crate) fn size(&self) -> usize {
        match self {
            AdStruct::ServiceUuid16(value) => value.size(),
            AdStruct::ServiceUuid32(value) => value.size(),
            AdStruct::ServiceUuid128(value) => value.size(),
        }
    }
}
