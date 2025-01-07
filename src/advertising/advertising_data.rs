use crate::advertising::ad_struct::{AdStruct, AdStructType};
use crate::advertising::{
    FlagsAdStruct, ServiceUuid128AdStruct, ServiceUuid16AdStruct, ServiceUuid32AdStruct,
};
use crate::Error;

pub(crate) const ADVERTISING_DATA_MAX_SIZE: usize = 31;
const ADVERTISING_DATA_SIZE_OFFSET: usize = 0;
const ADVERTISING_DATA_DATA_OFFSET: usize = 1;

#[derive(Debug, Default)]
pub struct AdvertisingDataBuilder {
    obj: AdvertisingData,
    present_ad_structs: AdStructType,
}

impl AdvertisingDataBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> AdvertisingData {
        self.obj
    }

    pub fn with_flags(mut self, flags: FlagsAdStruct) -> Result<Self, Error> {
        self.try_add(flags)?;
        Ok(self)
    }

    pub fn with_service_uuid16(
        mut self,
        service_uuid16: ServiceUuid16AdStruct,
    ) -> Result<Self, Error> {
        self.try_add(service_uuid16)?;
        Ok(self)
    }

    pub fn with_service_uuid32(
        mut self,
        service_uuid32: ServiceUuid32AdStruct,
    ) -> Result<Self, Error> {
        self.try_add(service_uuid32)?;
        Ok(self)
    }

    pub fn with_service_uuid128(
        mut self,
        service_uuid128: ServiceUuid128AdStruct,
    ) -> Result<Self, Error> {
        self.try_add(service_uuid128)?;
        Ok(self)
    }

    fn try_add(&mut self, ad_struct: impl AdStruct) -> Result<(), Error> {
        let ad_struct_type = ad_struct.r#type();
        if ad_struct.unique() && self.present_ad_structs.contains(ad_struct_type) {
            return Err(Error::AdStructAlreadyPresent);
        }
        self.obj.try_add(ad_struct)?;
        self.present_ad_structs |= ad_struct_type;
        Ok(())
    }
}

#[derive(Debug)]
pub struct AdvertisingData {
    buffer: [u8; ADVERTISING_DATA_MAX_SIZE + 1],
    offset: usize,
}

impl AdvertisingData {
    pub fn builder() -> AdvertisingDataBuilder {
        AdvertisingDataBuilder::new()
    }

    fn try_add(&mut self, ad_struct: impl AdStruct) -> Result<(), Error> {
        let ad_struct_data = ad_struct.data();
        let ad_struct_size = ad_struct_data.len();
        if self.remaining_size() < ad_struct_size {
            return Err(Error::BufferTooSmall);
        }
        self.buffer[self.offset..self.offset + ad_struct_size].copy_from_slice(ad_struct_data);
        self.offset += ad_struct_size;
        self.buffer[ADVERTISING_DATA_SIZE_OFFSET] += ad_struct_size as u8;
        Ok(())
    }

    pub(crate) fn data(&self) -> &[u8] {
        &self.buffer
    }

    fn total_size(&self) -> usize {
        self.buffer[ADVERTISING_DATA_SIZE_OFFSET] as usize
    }

    fn remaining_size(&self) -> usize {
        ADVERTISING_DATA_MAX_SIZE - self.total_size()
    }
}

impl Default for AdvertisingData {
    fn default() -> Self {
        Self {
            buffer: Default::default(),
            offset: ADVERTISING_DATA_DATA_OFFSET,
        }
    }
}
