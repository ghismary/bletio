use crate::advertising::ad_struct::common_data_types::CommonDataType;
use crate::advertising::ad_struct::{
    AdStruct, AdStructType, AD_STRUCT_DATA_OFFSET, AD_STRUCT_LENGTH_OFFSET, AD_STRUCT_TYPE_OFFSET,
};
use crate::advertising::advertising_data::ADVERTISING_DATA_MAX_SIZE;

use crate::utils::{encode_le_u128, encode_le_u16, encode_le_u32};
use crate::uuid::{Uuid128, Uuid16, Uuid32};
use crate::Error;

macro_rules! service_uuids {
    (
        $(
            $(#[$docs:meta])*
            ($struct_name:ident, $bytes:expr, $struct_type:expr, $uuid_type:ident, $encode_func:ident, $complete:expr, $incomplete:expr),
        )+
    ) => {
            $(
                #[derive(Debug)]
                pub struct $struct_name {
                    buffer: [u8; ADVERTISING_DATA_MAX_SIZE],
                    offset: usize,
                }

                impl $struct_name {
                    pub fn try_new(uuids: &[impl Into<$uuid_type> + Copy], complete: bool) -> Result<Self, Error> {
                        let uuids_size = uuids.len() * $bytes;
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
                        (self.offset - AD_STRUCT_DATA_OFFSET) / $bytes
                    }

                    pub fn is_complete(&self) -> bool {
                        self.buffer[AD_STRUCT_TYPE_OFFSET] == ($complete as u8)
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
                            $complete
                        } else {
                            $incomplete
                        } as u8;
                        self
                    }

                    pub fn try_add(mut self, uuid: impl Into<$uuid_type>) -> Result<Self, Error> {
                        let uuid = uuid.into();
                        self.offset += $encode_func(&mut self.buffer[self.offset..], uuid.0)?;
                        Ok(self)
                    }
                }

                impl Default for $struct_name {
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

                impl AdStruct for $struct_name {
                    fn data(&self) -> &[u8] {
                        &self.buffer[..self.offset]
                    }
                    fn r#type(&self) -> AdStructType {
                        $struct_type
                    }

                    fn unique(&self) -> bool {
                        true
                    }
                }
            )+
        }
}

service_uuids! {
    (ServiceUuid16AdStruct, 2, AdStructType::ServiceUuid16, Uuid16, encode_le_u16, CommonDataType::CompleteListOfServiceUuid16, CommonDataType::IncompleteListOfServiceUuid16),
    (ServiceUuid32AdStruct, 4, AdStructType::ServiceUuid32, Uuid32, encode_le_u32, CommonDataType::CompleteListOfServiceUuid32, CommonDataType::IncompleteListOfServiceUuid32),
    (ServiceUuid128AdStruct, 16, AdStructType::ServiceUuid128, Uuid128, encode_le_u128, CommonDataType::CompleteListOfServiceUuid128, CommonDataType::IncompleteListOfServiceUuid128),
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
        let err = ServiceUuid16AdStruct::try_new(
            [
                0x1802, 0x1803, 0x1804, 0x1815, 0x1806, 0x1807, 0x1808, 0x1809, 0x180A, 0x180B,
                0x180C, 0x180D, 0x180E, 0x180F, 0x1810,
            ]
            .as_slice(),
            true,
        )
        .expect_err("Too many Uuid16 to fit in the advertising data");
        assert!(matches!(err, Error::BufferTooSmall));
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
        let err = value
            .try_add(0x1810)
            .expect_err("Too many Uuid16 to fit in the advertising data");
        assert!(matches!(err, Error::BufferTooSmall));
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
        let err = ServiceUuid32AdStruct::try_new(
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
        )
        .expect_err("Too many Uuid32 to fit in the advertising data");
        assert!(matches!(err, Error::BufferTooSmall));
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
        let err = value
            .try_add(0x0000_1809)
            .expect_err("Too many Uuid32 to fit in the advertising data");
        assert!(matches!(err, Error::BufferTooSmall));
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
        let err = ServiceUuid128AdStruct::try_new(
            [
                0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640,
                0xA624BAC7_A46C_4EC8_B3D6_4C82E5A56D96,
            ]
            .as_slice(),
            true,
        )
        .expect_err("Too many Uuid128 to fit in the advertising data");
        assert!(matches!(err, Error::BufferTooSmall));
        let value = ServiceUuid128AdStruct::default()
            .try_add(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)
            .unwrap();
        let err = value
            .try_add(0xA624BAC7_A46C_4EC8_B3D6_4C82E5A56D96)
            .expect_err("Too many Uuid128 to fit in the advertising data");
        assert!(matches!(err, Error::BufferTooSmall));
    }
}
