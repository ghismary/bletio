use crate::advertising::ad_struct::common_data_types::CommonDataType;
use crate::advertising::ad_struct::{
    AdStruct, AdStructType, AD_STRUCT_DATA_OFFSET, AD_STRUCT_LENGTH_OFFSET, AD_STRUCT_TYPE_OFFSET,
};
use crate::advertising::advertising_data::ADVERTISING_DATA_MAX_SIZE;

use crate::utils::{encode_le_u128, encode_le_u16, encode_le_u32};
use crate::uuid::{Uuid128, Uuid16, Uuid32};
use crate::Error;

#[derive(Debug)]
pub struct ServiceUuid16AdStruct {
    buffer: [u8; ADVERTISING_DATA_MAX_SIZE],
    offset: usize,
}

impl ServiceUuid16AdStruct {
    pub fn try_new(uuids: &[impl Into<Uuid16> + Copy], complete: bool) -> Result<Self, Error> {
        let uuids_size = uuids.len() * 2;
        if (2 + uuids_size) > ADVERTISING_DATA_MAX_SIZE {
            return Err(Error::BufferTooSmall);
        }
        let mut s = Self {
            offset: AD_STRUCT_DATA_OFFSET,
            buffer: Default::default(),
        };
        s.buffer[AD_STRUCT_LENGTH_OFFSET] = 1 + uuids_size as u8;
        s = s.complete(complete);
        for uuid in uuids {
            s = s.try_add(*uuid)?;
        }
        Ok(s)
    }

    pub fn len(&self) -> usize {
        (self.offset - AD_STRUCT_DATA_OFFSET) / 2
    }

    pub fn is_complete(&self) -> bool {
        self.buffer[AD_STRUCT_TYPE_OFFSET] == (CommonDataType::CompleteListOfServiceUuid16 as u8)
    }

    pub fn is_empty(&self) -> bool {
        self.offset == AD_STRUCT_DATA_OFFSET
    }

    pub fn is_valid(&self) -> bool {
        !self.is_empty() || self.is_complete()
    }

    #[must_use]
    pub fn complete(mut self, complete: bool) -> Self {
        self.buffer[AD_STRUCT_TYPE_OFFSET] = if complete {
            CommonDataType::CompleteListOfServiceUuid16
        } else {
            CommonDataType::IncompleteListOfServiceUuid16
        } as u8;
        self
    }

    pub fn try_add(mut self, uuid: impl Into<Uuid16>) -> Result<Self, Error> {
        let uuid = uuid.into();
        self.offset += encode_le_u16(&mut self.buffer[self.offset..], uuid.0)?;
        Ok(self)
    }
}

impl Default for ServiceUuid16AdStruct {
    fn default() -> Self {
        let mut s = Self {
            offset: AD_STRUCT_DATA_OFFSET,
            buffer: Default::default(),
        };
        s.buffer[AD_STRUCT_LENGTH_OFFSET] = 1;
        s = s.complete(true);
        s
    }
}

impl AdStruct for ServiceUuid16AdStruct {
    fn data(&self) -> &[u8] {
        &self.buffer[..self.offset]
    }
    fn r#type(&self) -> AdStructType {
        AdStructType::ServiceUuid16
    }

    fn unique(&self) -> bool {
        true
    }
}

#[derive(Debug)]
pub struct ServiceUuid32AdStruct {
    buffer: [u8; ADVERTISING_DATA_MAX_SIZE],
    offset: usize,
}

impl ServiceUuid32AdStruct {
    pub fn try_new(uuids: &[impl Into<Uuid32> + Copy], complete: bool) -> Result<Self, Error> {
        let uuids_size = uuids.len() * 4;
        if (2 + uuids_size) > ADVERTISING_DATA_MAX_SIZE {
            return Err(Error::BufferTooSmall);
        }
        let mut s = Self {
            offset: AD_STRUCT_DATA_OFFSET,
            buffer: Default::default(),
        };
        s.buffer[AD_STRUCT_LENGTH_OFFSET] = 1 + uuids_size as u8;
        s = s.complete(complete);
        for uuid in uuids {
            s = s.try_add(*uuid)?;
        }
        Ok(s)
    }

    pub fn len(&self) -> usize {
        (self.offset - AD_STRUCT_DATA_OFFSET) / 4
    }

    pub fn is_complete(&self) -> bool {
        self.buffer[AD_STRUCT_TYPE_OFFSET] == (CommonDataType::CompleteListOfServiceUuid32 as u8)
    }

    pub fn is_empty(&self) -> bool {
        self.offset == AD_STRUCT_DATA_OFFSET
    }

    pub fn is_valid(&self) -> bool {
        !self.is_empty() || self.is_complete()
    }

    #[must_use]
    pub fn complete(mut self, complete: bool) -> Self {
        self.buffer[AD_STRUCT_TYPE_OFFSET] = if complete {
            CommonDataType::CompleteListOfServiceUuid32
        } else {
            CommonDataType::IncompleteListOfServiceUuid32
        } as u8;
        self
    }

    pub fn try_add(mut self, uuid: impl Into<Uuid32>) -> Result<Self, Error> {
        let uuid = uuid.into();
        self.offset += encode_le_u32(&mut self.buffer[self.offset..], uuid.0)?;
        Ok(self)
    }
}

impl Default for ServiceUuid32AdStruct {
    fn default() -> Self {
        let mut s = Self {
            offset: AD_STRUCT_DATA_OFFSET,
            buffer: Default::default(),
        };
        s.buffer[AD_STRUCT_LENGTH_OFFSET] = 1;
        s = s.complete(true);
        s
    }
}

impl AdStruct for ServiceUuid32AdStruct {
    fn data(&self) -> &[u8] {
        &self.buffer[..self.offset]
    }
    fn r#type(&self) -> AdStructType {
        AdStructType::ServiceUuid32
    }

    fn unique(&self) -> bool {
        true
    }
}

#[derive(Debug)]
pub struct ServiceUuid128AdStruct {
    buffer: [u8; ADVERTISING_DATA_MAX_SIZE],
    offset: usize,
}

impl ServiceUuid128AdStruct {
    pub fn try_new(uuids: &[impl Into<Uuid128> + Copy], complete: bool) -> Result<Self, Error> {
        let uuids_size = uuids.len() * 16;
        if (2 + uuids_size) > ADVERTISING_DATA_MAX_SIZE {
            return Err(Error::BufferTooSmall);
        }
        let mut s = Self {
            offset: AD_STRUCT_DATA_OFFSET,
            buffer: Default::default(),
        };
        s.buffer[AD_STRUCT_LENGTH_OFFSET] = 1 + uuids_size as u8;
        s = s.complete(complete);
        for uuid in uuids {
            s = s.try_add(*uuid)?;
        }
        Ok(s)
    }

    pub fn len(&self) -> usize {
        (self.offset - AD_STRUCT_DATA_OFFSET) / 16
    }

    pub fn is_complete(&self) -> bool {
        self.buffer[AD_STRUCT_TYPE_OFFSET] == (CommonDataType::CompleteListOfServiceUuid128 as u8)
    }

    pub fn is_empty(&self) -> bool {
        self.offset == AD_STRUCT_DATA_OFFSET
    }

    pub fn is_valid(&self) -> bool {
        !self.is_empty() || self.is_complete()
    }

    #[must_use]
    pub fn complete(mut self, complete: bool) -> Self {
        self.buffer[AD_STRUCT_TYPE_OFFSET] = if complete {
            CommonDataType::CompleteListOfServiceUuid128
        } else {
            CommonDataType::IncompleteListOfServiceUuid128
        } as u8;
        self
    }

    pub fn try_add(mut self, uuid: impl Into<Uuid128>) -> Result<Self, Error> {
        let uuid = uuid.into();
        self.offset += encode_le_u128(&mut self.buffer[self.offset..], uuid.0)?;
        Ok(self)
    }
}

impl Default for ServiceUuid128AdStruct {
    fn default() -> Self {
        let mut s = Self {
            offset: AD_STRUCT_DATA_OFFSET,
            buffer: Default::default(),
        };
        s.buffer[AD_STRUCT_LENGTH_OFFSET] = 1;
        s = s.complete(true);
        s
    }
}

impl AdStruct for ServiceUuid128AdStruct {
    fn data(&self) -> &[u8] {
        &self.buffer[..self.offset]
    }
    fn r#type(&self) -> AdStructType {
        AdStructType::ServiceUuid128
    }

    fn unique(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_service_uuid16_advertising_data_creation_success() -> Result<(), Error> {
        let mut value = ServiceUuid16AdStruct::try_new(
            [Uuid16(0x1803), Uuid16(0x180F), Uuid16(0x181A)].as_slice(),
            false,
        )?;
        assert_eq!(value.len(), 3);
        assert!(!value.is_complete());
        value = ServiceUuid16AdStruct::try_new([0x1808, 0x180D, 0x180F, 0x1810].as_slice(), true)?;
        assert_eq!(value.len(), 4);
        assert!(value.is_complete());
        value = ServiceUuid16AdStruct::default()
            .try_add(Uuid16(0x1803))?
            .try_add(Uuid16(0x180F))?
            .try_add(Uuid16(0x181A))?;
        assert_eq!(value.len(), 3);
        assert!(value.is_complete());
        value = ServiceUuid16AdStruct::default()
            .try_add(0x1808)?
            .try_add(0x180D)?
            .try_add(0x180F)?
            .try_add(0x1810)?
            .complete(false);
        assert_eq!(value.len(), 4);
        assert!(!value.is_complete());
        Ok(())
    }

    #[test]
    fn test_service_uuid16_advertising_data_creation_failure() {
        let value = ServiceUuid16AdStruct::try_new(
            [
                0x1802, 0x1803, 0x1804, 0x1815, 0x1806, 0x1807, 0x1808, 0x1809, 0x180A, 0x180B,
                0x180C, 0x180D, 0x180E, 0x180F, 0x1810,
            ]
            .as_slice(),
            true,
        );
        assert!(value.is_err());
        let value = ServiceUuid16AdStruct::default()
            .try_add(0x1802)
            .unwrap()
            .try_add(0x1803)
            .unwrap()
            .try_add(0x1804)
            .unwrap()
            .try_add(0x1805)
            .unwrap()
            .try_add(0x1806)
            .unwrap()
            .try_add(0x1807)
            .unwrap()
            .try_add(0x1808)
            .unwrap()
            .try_add(0x1809)
            .unwrap()
            .try_add(0x180A)
            .unwrap()
            .try_add(0x180B)
            .unwrap()
            .try_add(0x180C)
            .unwrap()
            .try_add(0x180D)
            .unwrap()
            .try_add(0x180E)
            .unwrap()
            .try_add(0x180F)
            .unwrap();
        assert!(value.try_add(0x1810).is_err());
    }

    #[test]
    fn test_service_uuid32_advertising_data_creation_success() -> Result<(), Error> {
        let mut value = ServiceUuid32AdStruct::try_new(
            [
                Uuid32(0x0000_1803),
                Uuid32(0x0000_180F),
                Uuid32(0x0000_181A),
            ]
            .as_slice(),
            false,
        )?;
        assert_eq!(value.len(), 3);
        assert!(!value.is_complete());
        value = ServiceUuid32AdStruct::try_new(
            [0x0000_1808, 0x0000_180D, 0x0000_180F, 0x0000_1810].as_slice(),
            true,
        )?;
        assert_eq!(value.len(), 4);
        assert!(value.is_complete());
        value = ServiceUuid32AdStruct::default()
            .try_add(Uuid32(0x0000_1803))?
            .try_add(Uuid32(0x0000_180F))?
            .try_add(Uuid32(0x0000_181A))?;
        assert_eq!(value.len(), 3);
        assert!(value.is_complete());
        value = ServiceUuid32AdStruct::default()
            .try_add(0x0000_1808)?
            .try_add(0x0000_180D)?
            .try_add(0x0000_180F)?
            .try_add(0x0000_1810)?
            .complete(false);
        assert_eq!(value.len(), 4);
        assert!(!value.is_complete());
        Ok(())
    }

    #[test]
    fn test_service_uuid32_advertising_data_creation_failure() {
        let value = ServiceUuid32AdStruct::try_new(
            [
                0x0000_1802,
                0x0000_1803,
                0x0000_1804,
                0x0000_1815,
                0x0000_1806,
                0x0000_1807,
                0x0000_1808,
                0x0000_1809,
            ]
            .as_slice(),
            true,
        );
        assert!(value.is_err());
        let value = ServiceUuid32AdStruct::default()
            .try_add(0x0000_1802)
            .unwrap()
            .try_add(0x0000_1803)
            .unwrap()
            .try_add(0x0000_1804)
            .unwrap()
            .try_add(0x0000_1805)
            .unwrap()
            .try_add(0x0000_1806)
            .unwrap()
            .try_add(0x0000_1807)
            .unwrap()
            .try_add(0x0000_1808)
            .unwrap();
        assert!(value.try_add(0x0000_1809).is_err());
    }

    #[test]
    fn test_service_uuid128_advertising_data_creation_success() -> Result<(), Error> {
        let mut value = ServiceUuid128AdStruct::try_new(
            [Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)].as_slice(),
            false,
        )?;
        assert_eq!(value.len(), 1);
        assert!(!value.is_complete());
        value = ServiceUuid128AdStruct::try_new(
            [0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640].as_slice(),
            true,
        )?;
        assert_eq!(value.len(), 1);
        assert!(value.is_complete());
        value = ServiceUuid128AdStruct::default()
            .try_add(Uuid128(0xA624BAC7_A46C_4EC8_B3D6_4C82E5A56D96))?;
        assert_eq!(value.len(), 1);
        assert!(value.is_complete());
        value = ServiceUuid128AdStruct::default()
            .try_add(0xA624BAC7_A46C_4EC8_B3D6_4C82E5A56D96)?
            .complete(false);
        assert_eq!(value.len(), 1);
        assert!(!value.is_complete());
        Ok(())
    }

    #[test]
    fn test_service_uuid128_advertising_data_creation_failure() {
        let value = ServiceUuid128AdStruct::try_new(
            [
                0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640,
                0xA624BAC7_A46C_4EC8_B3D6_4C82E5A56D96,
            ]
            .as_slice(),
            true,
        );
        assert!(value.is_err());
        let value = ServiceUuid128AdStruct::default()
            .try_add(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)
            .unwrap();
        assert!(value
            .try_add(0xA624BAC7_A46C_4EC8_B3D6_4C82E5A56D96)
            .is_err());
    }
}
