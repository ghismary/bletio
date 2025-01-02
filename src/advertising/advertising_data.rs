use arrayvec::ArrayVec;

use crate::advertising::{
    AdStruct, ServiceUuid128AdStruct, ServiceUuid16AdStruct, ServiceUuid32AdStruct,
};
use crate::Error;

#[derive(Debug, Default)]
pub struct AdvertisingData {
    ad_structs: ArrayVec<AdStruct, 10>,
}

impl AdvertisingData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn try_set_service_uuid16(
        mut self,
        ad_struct: ServiceUuid16AdStruct,
    ) -> Result<Self, Error> {
        let ad_struct_size = ad_struct.size();
        if ad_struct_size == 0 {
            // If the service uuid16 ad struct doesn't contain anything, just ignore it
            return Ok(self);
        }
        if self.has_service_uuid16() {
            return Err(Error::AdStructAlreadyPresent);
        }
        if !self.can_fit(ad_struct_size) {
            return Err(Error::AdStructDoesNotFit);
        }
        self.ad_structs
            .try_push(AdStruct::ServiceUuid16(ad_struct))
            .map_err(|_| Error::AdStructDoesNotFit)?;
        Ok(self)
    }

    pub fn try_set_service_uuid32(
        mut self,
        ad_struct: ServiceUuid32AdStruct,
    ) -> Result<Self, Error> {
        let ad_struct_size = ad_struct.size();
        if ad_struct_size == 0 {
            // If the service uuid32 ad struct doesn't contain anything, just ignore it
            return Ok(self);
        }
        if self.has_service_uuid32() {
            return Err(Error::AdStructAlreadyPresent);
        }
        if !self.can_fit(ad_struct_size) {
            return Err(Error::AdStructDoesNotFit);
        }
        self.ad_structs
            .try_push(AdStruct::ServiceUuid32(ad_struct))
            .map_err(|_| Error::AdStructDoesNotFit)?;
        Ok(self)
    }

    pub fn try_set_service_uuid128(
        mut self,
        ad_struct: ServiceUuid128AdStruct,
    ) -> Result<Self, Error> {
        let ad_struct_size = ad_struct.size();
        if self.has_service_uuid128() {
            return Err(Error::AdStructAlreadyPresent);
        }
        if !self.can_fit(ad_struct_size) {
            return Err(Error::AdStructDoesNotFit);
        }
        self.ad_structs
            .try_push(AdStruct::ServiceUuid128(ad_struct))
            .map_err(|_| Error::AdStructDoesNotFit)?;
        Ok(self)
    }

    fn has_service_uuid16(&self) -> bool {
        self.ad_structs
            .iter()
            .any(|item| matches!(item, AdStruct::ServiceUuid16(_)))
    }

    fn has_service_uuid32(&self) -> bool {
        self.ad_structs
            .iter()
            .any(|item| matches!(item, AdStruct::ServiceUuid32(_)))
    }

    fn has_service_uuid128(&self) -> bool {
        self.ad_structs
            .iter()
            .any(|item| matches!(item, AdStruct::ServiceUuid128(_)))
    }

    fn total_size(&self) -> usize {
        self.ad_structs.iter().map(|item| item.size()).sum()
    }

    fn remaining_size(&self) -> usize {
        31 - self.total_size()
    }

    fn can_fit(&self, size: usize) -> bool {
        self.remaining_size() >= size
    }
}
