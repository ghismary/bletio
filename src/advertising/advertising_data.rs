use arrayvec::ArrayVec;

use crate::advertising::{
    AdStruct, ServiceUuid128AdStruct, ServiceUuid16AdStruct, ServiceUuid32AdStruct,
};
use crate::Error;

const ADVERTISING_DATA_MAX_SIZE: usize = 31;

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
        if !ad_struct.is_valid() {
            return Err(Error::InvalidAdStruct);
        }
        if self.has_service_uuid16() {
            return Err(Error::AdStructAlreadyPresent);
        }
        if !self.can_fit(ad_struct.size()) {
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
        if !ad_struct.is_valid() {
            return Err(Error::InvalidAdStruct);
        }
        if self.has_service_uuid32() {
            return Err(Error::AdStructAlreadyPresent);
        }
        if !self.can_fit(ad_struct.size()) {
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
        if !ad_struct.is_valid() {
            return Err(Error::InvalidAdStruct);
        }
        if self.has_service_uuid128() {
            return Err(Error::AdStructAlreadyPresent);
        }
        if !self.can_fit(ad_struct.size()) {
            return Err(Error::AdStructDoesNotFit);
        }
        self.ad_structs
            .try_push(AdStruct::ServiceUuid128(ad_struct))
            .map_err(|_| Error::AdStructDoesNotFit)?;
        Ok(self)
    }

    pub(crate) fn encode(&self) -> Result<([u8; ADVERTISING_DATA_MAX_SIZE + 1], usize), Error> {
        let mut buffer = [0u8; ADVERTISING_DATA_MAX_SIZE + 1];
        buffer[0] = self.total_size() as u8;
        let mut offset = 1;
        for item in &self.ad_structs {
            offset += item.encode(&mut buffer[offset..])?;
        }
        Ok((buffer, offset))
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
        ADVERTISING_DATA_MAX_SIZE - self.total_size()
    }

    fn can_fit(&self, size: usize) -> bool {
        self.remaining_size() >= size
    }
}
