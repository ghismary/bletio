use arrayvec::ArrayVec;

use crate::uuid::{Uuid128, Uuid16, Uuid32};
use crate::Error;

#[derive(Debug)]
pub enum AdvertisingDataType {
    ServiceUuid16(ServiceUuid16AdvertisingData),
    ServiceUuid32(ServiceUuid32AdvertisingData),
    ServiceUuid128(ServiceUuid128AdvertisingData),
}

#[derive(Debug)]
pub struct ServiceUuid16AdvertisingData {
    uuids: ArrayVec<Uuid16, 15>,
    complete: bool,
}

impl ServiceUuid16AdvertisingData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.uuids.len()
    }

    pub fn is_complete(&self) -> bool {
        self.complete
    }

    pub fn is_empty(&self) -> bool {
        self.uuids.is_empty()
    }

    #[must_use]
    pub fn complete(mut self, complete: bool) -> Self {
        self.complete = complete;
        self
    }

    pub fn try_add(mut self, uuid: Uuid16) -> Result<Self, Error> {
        self.uuids
            .try_push(uuid)
            .map_err(|_| Error::BufferTooSmall)?;
        Ok(self)
    }
}

impl Default for ServiceUuid16AdvertisingData {
    fn default() -> Self {
        Self {
            uuids: ArrayVec::new(),
            complete: false,
        }
    }
}

impl TryFrom<&[Uuid16]> for ServiceUuid16AdvertisingData {
    type Error = Error;

    fn try_from(value: &[Uuid16]) -> Result<Self, Self::Error> {
        if value.len() <= 15 {
            Ok(Self {
                uuids: value.iter().cloned().collect(),
                complete: false,
            })
        } else {
            Err(Error::BufferTooSmall)
        }
    }
}

impl TryFrom<&[u16]> for ServiceUuid16AdvertisingData {
    type Error = Error;

    fn try_from(value: &[u16]) -> Result<Self, Self::Error> {
        if value.len() <= 15 {
            Ok(Self {
                uuids: value.iter().map(|v| (*v).into()).collect(),
                complete: false,
            })
        } else {
            Err(Error::BufferTooSmall)
        }
    }
}

#[derive(Debug)]
pub struct ServiceUuid32AdvertisingData {
    uuids: ArrayVec<Uuid32, 7>,
    complete: bool,
}

impl ServiceUuid32AdvertisingData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.uuids.len()
    }

    pub fn is_complete(&self) -> bool {
        self.complete
    }

    pub fn is_empty(&self) -> bool {
        self.uuids.is_empty()
    }

    #[must_use]
    pub fn complete(mut self, complete: bool) -> Self {
        self.complete = complete;
        self
    }

    pub fn try_add(mut self, uuid: Uuid32) -> Result<Self, Error> {
        self.uuids
            .try_push(uuid)
            .map_err(|_| Error::BufferTooSmall)?;
        Ok(self)
    }
}

impl Default for ServiceUuid32AdvertisingData {
    fn default() -> Self {
        Self {
            uuids: ArrayVec::new(),
            complete: false,
        }
    }
}

impl TryFrom<&[Uuid32]> for ServiceUuid32AdvertisingData {
    type Error = Error;

    fn try_from(value: &[Uuid32]) -> Result<Self, Self::Error> {
        if value.len() <= 7 {
            Ok(Self {
                uuids: value.iter().cloned().collect(),
                complete: false,
            })
        } else {
            Err(Error::BufferTooSmall)
        }
    }
}

impl TryFrom<&[u32]> for ServiceUuid32AdvertisingData {
    type Error = Error;

    fn try_from(value: &[u32]) -> Result<Self, Self::Error> {
        if value.len() <= 7 {
            Ok(Self {
                uuids: value.iter().map(|v| (*v).into()).collect(),
                complete: false,
            })
        } else {
            Err(Error::BufferTooSmall)
        }
    }
}
#[derive(Debug)]
pub struct ServiceUuid128AdvertisingData {
    uuid: Uuid128,
    complete: bool,
}

impl ServiceUuid128AdvertisingData {
    pub fn new(uuid: Uuid128) -> Self {
        Self {
            uuid,
            complete: false,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.complete
    }

    #[must_use]
    pub fn complete(mut self, complete: bool) -> Self {
        self.complete = complete;
        self
    }
}

impl From<Uuid128> for ServiceUuid128AdvertisingData {
    fn from(value: Uuid128) -> Self {
        Self {
            uuid: value,
            complete: false,
        }
    }
}

impl From<u128> for ServiceUuid128AdvertisingData {
    fn from(value: u128) -> Self {
        Self {
            uuid: value.into(),
            complete: false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_service_uuid16_advertising_data_creation_success() -> Result<(), Error> {
        let mut value: ServiceUuid16AdvertisingData =
            [Uuid16(0x1803), Uuid16(0x180F), Uuid16(0x181A)]
                .as_slice()
                .try_into()?;
        assert_eq!(value.len(), 3);
        assert!(!value.is_complete());
        value = [0x1808, 0x180D, 0x180F, 0x1810].as_slice().try_into()?;
        value = value.complete(true);
        assert_eq!(value.len(), 4);
        assert!(value.is_complete());
        value = ServiceUuid16AdvertisingData::new()
            .try_add(Uuid16(0x1803))?
            .try_add(Uuid16(0x180F))?
            .try_add(Uuid16(0x181A))?;
        assert_eq!(value.len(), 3);
        assert!(!value.is_complete());
        value = ServiceUuid16AdvertisingData::default()
            .try_add(0x1808.into())?
            .try_add(0x180D.into())?
            .try_add(0x180F.into())?
            .try_add(0x1810.into())?
            .complete(true);
        assert_eq!(value.len(), 4);
        assert!(value.is_complete());
        Ok(())
    }

    #[test]
    fn test_service_uuid16_advertising_data_creation_failure() {
        let value: Result<ServiceUuid16AdvertisingData, _> = [
            0x1802, 0x1803, 0x1804, 0x1815, 0x1806, 0x1807, 0x1808, 0x1809, 0x180A, 0x180B, 0x180C,
            0x180D, 0x180E, 0x180F, 0x1810, 0x1811,
        ]
        .as_slice()
        .try_into();
        assert!(value.is_err());
        let value = ServiceUuid16AdvertisingData::new()
            .try_add(0x1802.into())
            .unwrap()
            .try_add(0x1803.into())
            .unwrap()
            .try_add(0x1804.into())
            .unwrap()
            .try_add(0x1805.into())
            .unwrap()
            .try_add(0x1806.into())
            .unwrap()
            .try_add(0x1807.into())
            .unwrap()
            .try_add(0x1808.into())
            .unwrap()
            .try_add(0x1809.into())
            .unwrap()
            .try_add(0x180A.into())
            .unwrap()
            .try_add(0x180B.into())
            .unwrap()
            .try_add(0x180C.into())
            .unwrap()
            .try_add(0x180D.into())
            .unwrap()
            .try_add(0x180E.into())
            .unwrap()
            .try_add(0x180F.into())
            .unwrap()
            .try_add(0x1810.into())
            .unwrap();
        assert!(value.try_add(0x1811.into()).is_err());
    }

    #[test]
    fn test_service_uuid32_advertising_data_creation_success() -> Result<(), Error> {
        let mut value: ServiceUuid32AdvertisingData = [
            Uuid32(0x0000_1803),
            Uuid32(0x0000_180F),
            Uuid32(0x0000_181A),
        ]
        .as_slice()
        .try_into()?;
        assert_eq!(value.len(), 3);
        assert!(!value.is_complete());
        value = [0x0000_1808, 0x0000_180D, 0x0000_180F, 0x0000_1810]
            .as_slice()
            .try_into()?;
        value = value.complete(true);
        assert_eq!(value.len(), 4);
        assert!(value.is_complete());
        value = ServiceUuid32AdvertisingData::new()
            .try_add(Uuid32(0x0000_1803))?
            .try_add(Uuid32(0x0000_180F))?
            .try_add(Uuid32(0x0000_181A))?;
        assert_eq!(value.len(), 3);
        assert!(!value.is_complete());
        value = ServiceUuid32AdvertisingData::default()
            .try_add(0x0000_1808.into())?
            .try_add(0x0000_180D.into())?
            .try_add(0x0000_180F.into())?
            .try_add(0x0000_1810.into())?
            .complete(true);
        assert_eq!(value.len(), 4);
        assert!(value.is_complete());
        Ok(())
    }

    #[test]
    fn test_service_uuid32_advertising_data_creation_failure() {
        let value: Result<ServiceUuid32AdvertisingData, _> = [
            0x0000_1802,
            0x0000_1803,
            0x0000_1804,
            0x0000_1815,
            0x0000_1806,
            0x0000_1807,
            0x0000_1808,
            0x0000_1809,
        ]
        .as_slice()
        .try_into();
        assert!(value.is_err());
        let value = ServiceUuid32AdvertisingData::new()
            .try_add(0x0000_1802.into())
            .unwrap()
            .try_add(0x0000_1803.into())
            .unwrap()
            .try_add(0x0000_1804.into())
            .unwrap()
            .try_add(0x0000_1805.into())
            .unwrap()
            .try_add(0x0000_1806.into())
            .unwrap()
            .try_add(0x0000_1807.into())
            .unwrap()
            .try_add(0x0000_1808.into())
            .unwrap();
        assert!(value.try_add(0x0000_1809.into()).is_err());
    }

    #[test]
    fn test_service_uuid128_advertising_data_creation_success() {
        let mut value =
            ServiceUuid128AdvertisingData::new(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640.into());
        value = value.complete(true);
        assert!(value.is_complete());

        let value: ServiceUuid128AdvertisingData = 0xA624BAC7_A46C_4EC8_B3D6_4C82E5A56D96.into();
        assert!(!value.is_complete());
    }
}
