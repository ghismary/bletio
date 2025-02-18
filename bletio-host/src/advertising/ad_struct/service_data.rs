use bletio_utils::EncodeToBuffer;
use heapless::Vec;

use crate::advertising::AdvertisingError;
use crate::assigned_numbers::{AdType, ServiceUuid};
use crate::uuid::{Uuid128, Uuid32};

const SERVICE_DATA_UUID16_MAX_LENGTH: usize = 27;
const SERVICE_DATA_UUID32_MAX_LENGTH: usize = 25;
const SERVICE_DATA_UUID128_MAX_LENGTH: usize = 13;

/// Service Data for 16-bit Service UUIDs.
///
/// See [Supplement to the Bluetooth Core Specification, Part A, 1.11](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-dea15cd4-bc0f-91f0-82c1-3bbe596f7bf6).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct ServiceDataUuid16AdStruct {
    uuid: ServiceUuid,
    data: Vec<u8, SERVICE_DATA_UUID16_MAX_LENGTH>,
}

impl ServiceDataUuid16AdStruct {
    pub(crate) fn try_new(uuid: ServiceUuid, data: &[u8]) -> Result<Self, AdvertisingError> {
        Ok(Self {
            uuid,
            data: data
                .try_into()
                .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?,
        })
    }
}

impl EncodeToBuffer for ServiceDataUuid16AdStruct {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push((self.encoded_size() - 1) as u8)?;
        buffer.try_push(AdType::ServiceDataUuid16 as u8)?;
        buffer.encode_le_u16(self.uuid as u16)?;
        buffer.copy_from_slice(self.data.as_slice())?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        self.data.len() + 4
    }
}

/// Service Data for 32-bit Service UUIDs.
///
/// See [Supplement to the Bluetooth Core Specification, Part A, 1.11](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-dea15cd4-bc0f-91f0-82c1-3bbe596f7bf6).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct ServiceDataUuid32AdStruct {
    uuid: Uuid32,
    data: Vec<u8, SERVICE_DATA_UUID32_MAX_LENGTH>,
}

impl ServiceDataUuid32AdStruct {
    pub(crate) fn try_new(uuid: Uuid32, data: &[u8]) -> Result<Self, AdvertisingError> {
        Ok(Self {
            uuid,
            data: data
                .try_into()
                .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?,
        })
    }
}

impl EncodeToBuffer for ServiceDataUuid32AdStruct {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push((self.encoded_size() - 1) as u8)?;
        buffer.try_push(AdType::ServiceDataUuid32 as u8)?;
        buffer.encode_le_u32(self.uuid.0)?;
        buffer.copy_from_slice(self.data.as_slice())?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        self.data.len() + 6
    }
}

/// Service Data for 128-bit Service UUIDs.
///
/// See [Supplement to the Bluetooth Core Specification, Part A, 1.11](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-dea15cd4-bc0f-91f0-82c1-3bbe596f7bf6).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct ServiceDataUuid128AdStruct {
    uuid: Uuid128,
    data: Vec<u8, SERVICE_DATA_UUID128_MAX_LENGTH>,
}

impl ServiceDataUuid128AdStruct {
    pub(crate) fn try_new(uuid: Uuid128, data: &[u8]) -> Result<Self, AdvertisingError> {
        Ok(Self {
            uuid,
            data: data
                .try_into()
                .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?,
        })
    }
}

impl EncodeToBuffer for ServiceDataUuid128AdStruct {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push((self.encoded_size() - 1) as u8)?;
        buffer.try_push(AdType::ServiceDataUuid128 as u8)?;
        buffer.encode_le_u128(self.uuid.0)?;
        buffer.copy_from_slice(self.data.as_slice())?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        self.data.len() + 18
    }
}

#[cfg(test)]
mod test {
    use bletio_utils::{Buffer, BufferOps};
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(ServiceUuid::LinkLoss, &[0x01, 0x14], &[0x05, 0x16, 0x03, 0x18, 0x01, 0x14])]
    #[case(ServiceUuid::Battery, &[0x14, 0x03, 0x18, 0x0F], &[0x07, 0x16, 0x0F, 0x18, 0x14, 0x03, 0x18, 0x0F])]
    fn test_service_data_uuid16_ad_struct_success(
        #[case] uuid: ServiceUuid,
        #[case] data: &[u8],
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<31>::default();
        let value = ServiceDataUuid16AdStruct::try_new(uuid, data).unwrap();
        value.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }

    #[test]
    fn test_service_data_uuid16_ad_struct_failure() {
        let data = [0u8; 28];
        let err = ServiceDataUuid16AdStruct::try_new(ServiceUuid::ImmediateAlert, &data);
        assert_eq!(
            err,
            Err(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
        );
    }

    #[rstest]
    #[case(
        Uuid32(0x0000_1803), &[0x09, 0x1F, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00],
        &[0x0F, 0x20, 0x03, 0x18, 0x00, 0x00, 0x09, 0x1F, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00]
    )]
    #[case(Uuid32(0x0000_1808), &[0x0D, 0x1F, 0x08, 0x18, 0x00, 0x00, 0x0D, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00],
        &[0x13, 0x20, 0x08, 0x18, 0x00, 0x00, 0x0D, 0x1F, 0x08, 0x18, 0x00, 0x00, 0x0D, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00]
    )]
    fn test_service_data_uuid32_ad_struct_success(
        #[case] uuid: Uuid32,
        #[case] data: &[u8],
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<31>::default();
        let value = ServiceDataUuid32AdStruct::try_new(uuid, data).unwrap();
        value.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }

    #[test]
    fn test_service_data_uuid32_ad_struct_failure() {
        let data = [0u8; 26];
        let err = ServiceDataUuid32AdStruct::try_new(Uuid32(0x0000_1802), &data);
        assert_eq!(
            err,
            Err(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
        );
    }

    #[rstest]
    #[case(
        Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640), &[0xD6, 0x0F, 0x28, 0x6E],
        &[0x15, 0x21, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5, 0xD6, 0x0F, 0x28, 0x6E]
    )]
    fn test_service_data_uuid128_ad_struct_success(
        #[case] uuid: Uuid128,
        #[case] data: &[u8],
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<31>::default();
        let value = ServiceDataUuid128AdStruct::try_new(uuid, data).unwrap();
        value.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }

    #[test]
    fn test_service_data_uuid128_ad_struct_failure() {
        let data = [0u8; 14];
        let err = ServiceDataUuid128AdStruct::try_new(
            Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640),
            &data,
        );
        assert_eq!(
            err,
            Err(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
        );
    }
}
