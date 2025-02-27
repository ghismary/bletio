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
pub struct ServiceDataUuid16AdStruct {
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

    pub fn uuid(&self) -> ServiceUuid {
        self.uuid
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
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
pub struct ServiceDataUuid32AdStruct {
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

    pub fn uuid(&self) -> Uuid32 {
        self.uuid
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
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
pub struct ServiceDataUuid128AdStruct {
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

    pub fn uuid(&self) -> Uuid128 {
        self.uuid
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
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

pub(crate) mod parser {
    use nom::{
        bytes::take,
        combinator::{fail, map_res},
        IResult, Parser,
    };

    use crate::advertising::{
        ad_struct::AdStruct,
        advertising_data::parser::{service_uuid, uuid128, uuid32},
    };

    use super::*;

    pub(crate) fn service_data_uuid16_ad_struct(input: &[u8]) -> IResult<&[u8], AdStruct> {
        let (rest, uuid) = service_uuid.parse(input)?;
        let len = rest.len();
        if len > SERVICE_DATA_UUID16_MAX_LENGTH {
            fail::<_, &[u8], _>().parse(input)?;
        }
        let mut ad_struct = ServiceDataUuid16AdStruct {
            uuid,
            data: Default::default(),
        };
        map_res(take(len), |data| ad_struct.data.extend_from_slice(data)).parse(rest)?;
        Ok((&[], AdStruct::ServiceDataUuid16(ad_struct)))
    }

    pub(crate) fn service_data_uuid32_ad_struct(input: &[u8]) -> IResult<&[u8], AdStruct> {
        let (rest, uuid) = uuid32.parse(input)?;
        let len = rest.len();
        if len > SERVICE_DATA_UUID32_MAX_LENGTH {
            fail::<_, &[u8], _>().parse(input)?;
        }
        let mut ad_struct = ServiceDataUuid32AdStruct {
            uuid,
            data: Default::default(),
        };
        map_res(take(len), |data| ad_struct.data.extend_from_slice(data)).parse(rest)?;
        Ok((&[], AdStruct::ServiceDataUuid32(ad_struct)))
    }

    pub(crate) fn service_data_uuid128_ad_struct(input: &[u8]) -> IResult<&[u8], AdStruct> {
        let (rest, uuid) = uuid128.parse(input)?;
        let len = rest.len();
        if len > SERVICE_DATA_UUID128_MAX_LENGTH {
            fail::<_, &[u8], _>().parse(input)?;
        }
        let mut ad_struct = ServiceDataUuid128AdStruct {
            uuid,
            data: Default::default(),
        };
        map_res(take(len), |data| ad_struct.data.extend_from_slice(data)).parse(rest)?;
        Ok((&[], AdStruct::ServiceDataUuid128(ad_struct)))
    }
}

#[cfg(test)]
mod test {
    use bletio_utils::{Buffer, BufferOps};
    use rstest::rstest;

    use crate::advertising::ad_struct::AdStruct;

    use super::{parser::*, *};

    #[rstest]
    #[case(ServiceUuid::LinkLoss, &[0x01, 0x14], &[0x05, 0x16, 0x03, 0x18, 0x01, 0x14])]
    #[case(ServiceUuid::Battery, &[0x14, 0x03, 0x18, 0x0F], &[0x07, 0x16, 0x0F, 0x18, 0x14, 0x03, 0x18, 0x0F])]
    fn test_service_data_uuid16_ad_struct_success(
        #[case] uuid: ServiceUuid,
        #[case] data: &[u8],
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<31>::default();
        let ad_struct = ServiceDataUuid16AdStruct::try_new(uuid, data).unwrap();
        ad_struct.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        assert_eq!(ad_struct.uuid(), uuid);
        assert_eq!(ad_struct.data(), data);
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
        let ad_struct = ServiceDataUuid32AdStruct::try_new(uuid, data).unwrap();
        ad_struct.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        assert_eq!(ad_struct.uuid(), uuid);
        assert_eq!(ad_struct.data(), data);
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
        let ad_struct = ServiceDataUuid128AdStruct::try_new(uuid, data).unwrap();
        ad_struct.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        assert_eq!(ad_struct.uuid(), uuid);
        assert_eq!(ad_struct.data(), data);
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

    #[rstest]
    #[case(&[0x03, 0x18, 0x01, 0x14], ServiceUuid::LinkLoss, &[0x01, 0x14])]
    #[case(&[0x0F, 0x18, 0x14, 0x03, 0x18, 0x0F], ServiceUuid::Battery, &[0x14, 0x03, 0x18, 0x0F])]
    fn test_service_data_uuid16_ad_struct_parsing_success(
        #[case] input: &[u8],
        #[case] uuid: ServiceUuid,
        #[case] data: &[u8],
    ) {
        assert_eq!(
            service_data_uuid16_ad_struct(input),
            Ok((
                &[] as &[u8],
                AdStruct::ServiceDataUuid16(
                    ServiceDataUuid16AdStruct::try_new(uuid, data).unwrap()
                )
            ))
        );
    }

    #[rstest]
    #[case(
        &[0x02, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])]
    fn test_service_data_uuid16_ad_struct_parsing_failure(#[case] input: &[u8]) {
        assert!(service_data_uuid16_ad_struct(input).is_err());
    }

    #[rstest]
    #[case(
        &[0x03, 0x18, 0x00, 0x00, 0x09, 0x1F, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00],
        Uuid32(0x0000_1803), &[0x09, 0x1F, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00]
    )]
    #[case(
        &[0x08, 0x18, 0x00, 0x00, 0x0D, 0x1F, 0x08, 0x18, 0x00, 0x00, 0x0D, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00],
        Uuid32(0x0000_1808), &[0x0D, 0x1F, 0x08, 0x18, 0x00, 0x00, 0x0D, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00]
    )]
    fn test_service_data_uuid32_ad_struct_parsing_success(
        #[case] input: &[u8],
        #[case] uuid: Uuid32,
        #[case] data: &[u8],
    ) {
        assert_eq!(
            service_data_uuid32_ad_struct(input),
            Ok((
                &[] as &[u8],
                AdStruct::ServiceDataUuid32(
                    ServiceDataUuid32AdStruct::try_new(uuid, data).unwrap()
                )
            ))
        );
    }

    #[rstest]
    #[case(
        &[0x02, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])]
    fn test_service_data_uuid32_ad_struct_parsing_failure(#[case] input: &[u8]) {
        assert!(service_data_uuid32_ad_struct(input).is_err());
    }

    #[rstest]
    #[case(
        &[0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5, 0xD6, 0x0F, 0x28, 0x6E],
        Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640), &[0xD6, 0x0F, 0x28, 0x6E]
    )]
    fn test_service_data_uuid128_ad_struct_parsing_success(
        #[case] input: &[u8],
        #[case] uuid: Uuid128,
        #[case] data: &[u8],
    ) {
        assert_eq!(
            service_data_uuid128_ad_struct(input),
            Ok((
                &[] as &[u8],
                AdStruct::ServiceDataUuid128(
                    ServiceDataUuid128AdStruct::try_new(uuid, data).unwrap()
                )
            ))
        );
    }

    #[rstest]
    #[case(
        &[0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])]
    fn test_service_data_uuid128_ad_struct_parsing_failure(#[case] input: &[u8]) {
        assert!(service_data_uuid128_ad_struct(input).is_err());
    }
}
