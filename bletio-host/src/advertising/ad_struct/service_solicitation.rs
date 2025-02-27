use core::ops::Deref;

use bletio_utils::EncodeToBuffer;
use heapless::Vec;

use crate::advertising::AdvertisingError;
use crate::assigned_numbers::{AdType, ServiceUuid};
use crate::uuid::{Uuid128, Uuid32};

const SERVICE_SOLICITATION_UUID16_NB_MAX: usize = 14;
const SERVICE_SOLICITATION_UUID32_NB_MAX: usize = 7;
const SERVICE_SOLICITATION_UUID128_NB_MAX: usize = 1;

/// List of 16-bit Service Solicitation UUIDs.
///
/// A Peripheral may send the Service Solicitation Advertising Structure to invite Centrals that expose one or
/// more of the services specified in the Service Solicitation data to connect. The Peripheral should be in the
/// undirected connectable mode and in one of the discoverable modes. This enables a Central providing one or more
/// of these services to connect to the Peripheral, so that the Peripheral can use the services on the Central.
///
/// See [Supplement to the Bluetooth Core Specification, Part A, 1.10](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-302574d9-585b-209a-c32f-c5b6278f3377).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ServiceSolicitationUuid16AdStruct {
    uuids: Vec<ServiceUuid, SERVICE_SOLICITATION_UUID16_NB_MAX>,
}

impl ServiceSolicitationUuid16AdStruct {
    pub(crate) fn try_new(uuids: &[ServiceUuid]) -> Result<Self, AdvertisingError> {
        Ok(Self {
            uuids: uuids
                .try_into()
                .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?,
        })
    }
}

impl Deref for ServiceSolicitationUuid16AdStruct {
    type Target = Vec<ServiceUuid, SERVICE_SOLICITATION_UUID16_NB_MAX>;

    fn deref(&self) -> &Self::Target {
        &self.uuids
    }
}

impl EncodeToBuffer for ServiceSolicitationUuid16AdStruct {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push((self.encoded_size() - 1) as u8)?;
        buffer.try_push(AdType::ListOfSolicitationServiceUuid16 as u8)?;
        for uuid in self.uuids.iter() {
            buffer.encode_le_u16(*uuid as u16)?;
        }
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        (self.uuids.len() * 2) + 2
    }
}

/// List of 32-bit Service Solicitation UUIDs.
///
/// A Peripheral may send the Service Solicitation Advertising Structure to invite Centrals that expose one or
/// more of the services specified in the Service Solicitation data to connect. The Peripheral should be in the
/// undirected connectable mode and in one of the discoverable modes. This enables a Central providing one or more
/// of these services to connect to the Peripheral, so that the Peripheral can use the services on the Central.
///
/// See [Supplement to the Bluetooth Core Specification, Part A, 1.10](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-302574d9-585b-209a-c32f-c5b6278f3377).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ServiceSolicitationUuid32AdStruct {
    uuids: Vec<Uuid32, SERVICE_SOLICITATION_UUID32_NB_MAX>,
}

impl ServiceSolicitationUuid32AdStruct {
    pub(crate) fn try_new(uuids: &[Uuid32]) -> Result<Self, AdvertisingError> {
        Ok(Self {
            uuids: uuids
                .try_into()
                .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?,
        })
    }
}

impl Deref for ServiceSolicitationUuid32AdStruct {
    type Target = Vec<Uuid32, SERVICE_SOLICITATION_UUID32_NB_MAX>;

    fn deref(&self) -> &Self::Target {
        &self.uuids
    }
}

impl EncodeToBuffer for ServiceSolicitationUuid32AdStruct {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push((self.encoded_size() - 1) as u8)?;
        buffer.try_push(AdType::ListOfSolicitationServiceUuid32 as u8)?;
        for uuid in self.uuids.iter() {
            buffer.encode_le_u32(uuid.0)?;
        }
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        (self.uuids.len() * 4) + 2
    }
}

/// List of 128-bit Service Solicitation UUIDs.
///
/// A Peripheral may send the Service Solicitation Advertising Structure to invite Centrals that expose one or
/// more of the services specified in the Service Solicitation data to connect. The Peripheral should be in the
/// undirected connectable mode and in one of the discoverable modes. This enables a Central providing one or more
/// of these services to connect to the Peripheral, so that the Peripheral can use the services on the Central.
///
/// See [Supplement to the Bluetooth Core Specification, Part A, 1.10](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-302574d9-585b-209a-c32f-c5b6278f3377).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ServiceSolicitationUuid128AdStruct {
    uuids: Vec<Uuid128, SERVICE_SOLICITATION_UUID128_NB_MAX>,
}

impl ServiceSolicitationUuid128AdStruct {
    pub(crate) fn try_new(uuids: &[Uuid128]) -> Result<Self, AdvertisingError> {
        Ok(Self {
            uuids: uuids
                .try_into()
                .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?,
        })
    }
}

impl Deref for ServiceSolicitationUuid128AdStruct {
    type Target = Vec<Uuid128, SERVICE_SOLICITATION_UUID128_NB_MAX>;

    fn deref(&self) -> &Self::Target {
        &self.uuids
    }
}

impl EncodeToBuffer for ServiceSolicitationUuid128AdStruct {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push((self.encoded_size() - 1) as u8)?;
        buffer.try_push(AdType::ListOfSolicitationServiceUuid128 as u8)?;
        for uuid in self.uuids.iter() {
            buffer.encode_le_u128(uuid.0)?;
        }
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        (self.uuids.len() * 16) + 2
    }
}

pub(crate) mod parser {
    use nom::{
        combinator::{fail, map_res},
        IResult, Parser,
    };

    use crate::advertising::{
        ad_struct::AdStruct,
        advertising_data::parser::{service_uuid, uuid128, uuid32},
    };

    use super::*;

    pub(crate) fn list_of_solicitation_service_uuid16_ad_struct(
        mut input: &[u8],
    ) -> IResult<&[u8], AdStruct> {
        let len = input.len() / 2;
        if (len > SERVICE_SOLICITATION_UUID16_NB_MAX) || ((input.len() % 2) != 0) {
            fail::<_, &[u8], _>().parse(input)?;
        }
        let mut ad_struct = ServiceSolicitationUuid16AdStruct {
            uuids: Default::default(),
        };
        let mut index = 0;
        while index < len {
            let (rest, _) =
                map_res(service_uuid, |uuid| ad_struct.uuids.push(uuid)).parse(input)?;
            input = rest;
            index += 1;
        }
        Ok((&[], AdStruct::ServiceSolicitationUuid16(ad_struct)))
    }

    pub(crate) fn list_of_solicitation_service_uuid32_ad_struct(
        mut input: &[u8],
    ) -> IResult<&[u8], AdStruct> {
        let len = input.len() / 4;
        if (len > SERVICE_SOLICITATION_UUID32_NB_MAX) || ((input.len() % 4) != 0) {
            fail::<_, &[u8], _>().parse(input)?;
        }
        let mut ad_struct = ServiceSolicitationUuid32AdStruct {
            uuids: Default::default(),
        };
        let mut index = 0;
        while index < len {
            let (rest, _) = map_res(uuid32, |uuid| ad_struct.uuids.push(uuid)).parse(input)?;
            input = rest;
            index += 1;
        }
        Ok((&[], AdStruct::ServiceSolicitationUuid32(ad_struct)))
    }

    pub(crate) fn list_of_solicitation_service_uuid128_ad_struct(
        mut input: &[u8],
    ) -> IResult<&[u8], AdStruct> {
        let len = input.len() / 16;
        if (len > SERVICE_SOLICITATION_UUID128_NB_MAX) || ((input.len() % 16) != 0) {
            fail::<_, &[u8], _>().parse(input)?;
        }
        let mut ad_struct = ServiceSolicitationUuid128AdStruct {
            uuids: Default::default(),
        };
        let mut index = 0;
        while index < len {
            let (rest, _) = map_res(uuid128, |uuid| ad_struct.uuids.push(uuid)).parse(input)?;
            input = rest;
            index += 1;
        }
        Ok((&[], AdStruct::ServiceSolicitationUuid128(ad_struct)))
    }
}

#[cfg(test)]
mod test {
    use bletio_utils::{Buffer, BufferOps};
    use rstest::rstest;

    use crate::advertising::ad_struct::AdStruct;

    use super::{parser::*, *};

    #[rstest]
    #[case(&[], &[0x01, 0x14])]
    #[case(&[ServiceUuid::LinkLoss, ServiceUuid::Battery], &[0x05, 0x14, 0x03, 0x18, 0x0F, 0x18])]
    #[case(
        &[ServiceUuid::Glucose, ServiceUuid::HeartRate, ServiceUuid::BloodPressure],
        &[0x07, 0x14, 0x08, 0x18, 0x0D, 0x18, 0x10, 0x18]
    )]
    fn test_service_solicitation_uuid16_ad_struct_success(
        #[case] uuids: &[ServiceUuid],
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<31>::default();
        let ad_struct = ServiceSolicitationUuid16AdStruct::try_new(uuids).unwrap();
        ad_struct.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        assert_eq!(ad_struct.iter().count(), uuids.len());
        Ok(())
    }

    #[test]
    fn test_service_solicitation_uuid16_ad_struct_failure() {
        let err = ServiceSolicitationUuid16AdStruct::try_new(&[
            ServiceUuid::ImmediateAlert,
            ServiceUuid::LinkLoss,
            ServiceUuid::TxPower,
            ServiceUuid::CurrentTime,
            ServiceUuid::ReferenceTimeUpdate,
            ServiceUuid::NextDstChange,
            ServiceUuid::Glucose,
            ServiceUuid::HealthThermometer,
            ServiceUuid::DeviceInformation,
            ServiceUuid::HeartRate,
            ServiceUuid::PhoneAlertStatus,
            ServiceUuid::Battery,
            ServiceUuid::BloodPressure,
            ServiceUuid::AlertNotification,
            ServiceUuid::HumanInterfaceDevice,
        ]);
        assert_eq!(
            err,
            Err(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
        );
    }

    #[rstest]
    #[case(&[], &[0x01, 0x1F])]
    #[case(&[Uuid32(0x0000_1803), Uuid32(0x0000_180F)], &[0x09, 0x1F, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00])]
    #[case(
        &[Uuid32(0x0000_1808), Uuid32(0x0000_180D), Uuid32(0x0000_180F)],
        &[0x0D, 0x1F, 0x08, 0x18, 0x00, 0x00, 0x0D, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00]
    )]
    fn test_service_solicitation_uuid32_ad_struct_success(
        #[case] uuids: &[Uuid32],
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<31>::default();
        let ad_struct = ServiceSolicitationUuid32AdStruct::try_new(uuids).unwrap();
        ad_struct.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        assert_eq!(ad_struct.iter().count(), uuids.len());
        Ok(())
    }

    #[test]
    fn test_service_solicitation_uuid32_ad_struct_failure() {
        let err = ServiceSolicitationUuid32AdStruct::try_new(&[
            Uuid32(0x0000_1802),
            Uuid32(0x0000_1803),
            Uuid32(0x0000_1804),
            Uuid32(0x0000_1805),
            Uuid32(0x0000_1806),
            Uuid32(0x0000_1807),
            Uuid32(0x0000_1808),
            Uuid32(0x0000_1809),
        ]);
        assert_eq!(
            err,
            Err(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
        );
    }

    #[rstest]
    #[case(&[], &[0x01, 0x15])]
    #[case(
        &[Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)],
        &[0x11, 0x15, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5]
    )]
    fn test_service_solicitation_uuid128_ad_struct_success(
        #[case] uuids: &[Uuid128],
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<31>::default();
        let ad_struct = ServiceSolicitationUuid128AdStruct::try_new(uuids).unwrap();
        ad_struct.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        assert_eq!(ad_struct.iter().count(), uuids.len());
        Ok(())
    }

    #[test]
    fn test_service_solicitation_uuid128_ad_struct_failure() {
        let err = ServiceSolicitationUuid128AdStruct::try_new(&[
            Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640),
            Uuid128(0xA624BAC7_A46C_4EC8_B3D6_4C82E5A56D96),
        ]);
        assert_eq!(
            err,
            Err(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
        );
    }

    #[rstest]
    #[case(&[0x03, 0x18, 0x0F, 0x18], &[ServiceUuid::LinkLoss, ServiceUuid::Battery])]
    #[case(&[0x08, 0x18, 0x0D, 0x18, 0x10, 0x18], &[ServiceUuid::Glucose, ServiceUuid::HeartRate, ServiceUuid::BloodPressure])]
    fn test_service_solicitation_uuid16_ad_struct_parsing_success(
        #[case] input: &[u8],
        #[case] uuids: &[ServiceUuid],
    ) {
        assert_eq!(
            list_of_solicitation_service_uuid16_ad_struct(input),
            Ok((
                &[] as &[u8],
                AdStruct::ServiceSolicitationUuid16(
                    ServiceSolicitationUuid16AdStruct::try_new(uuids).unwrap()
                )
            ))
        );
    }

    #[rstest]
    #[case(
        &[0x02, 0x18, 0x03, 0x18, 0x04, 0x18, 0x05, 0x18, 0x06, 0x18, 0x07, 0x18, 0x08, 0x18,
            0x09, 0x18, 0x0A, 0x18, 0x0D, 0x18, 0x0E, 0x18, 0x0F, 0x18, 0x10, 0x18, 0x11, 0x18, 0x12, 0x18])]
    #[case(&[0x02, 0x18, 0x03])]
    fn test_service_solicitation_uuid16_ad_struct_parsing_failure(#[case] input: &[u8]) {
        assert!(list_of_solicitation_service_uuid16_ad_struct(input).is_err());
    }

    #[rstest]
    #[case(&[], &[])]
    #[case(&[0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00], &[Uuid32(0x0000_1803), Uuid32(0x0000_180F)])]
    #[case(&[0x08, 0x18, 0x00, 0x00, 0x0D, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00], &[Uuid32(0x0000_1808), Uuid32(0x0000_180D), Uuid32(0x0000_180F)])]
    fn test_service_solicitation_uuid32_ad_struct_parsing_success(
        #[case] input: &[u8],
        #[case] uuids: &[Uuid32],
    ) {
        assert_eq!(
            list_of_solicitation_service_uuid32_ad_struct(input),
            Ok((
                &[] as &[u8],
                AdStruct::ServiceSolicitationUuid32(
                    ServiceSolicitationUuid32AdStruct::try_new(uuids).unwrap()
                )
            ))
        );
    }

    #[rstest]
    #[case(
        &[0x02, 0x18, 0x00, 0x00, 0x03, 0x18, 0x00, 0x00, 0x04, 0x18, 0x00, 0x00, 0x05, 0x18, 0x00, 0x00,
            0x06, 0x18, 0x00, 0x00, 0x07, 0x18, 0x00, 0x00, 0x08, 0x18, 0x00, 0x00, 0x09, 0x18, 0x00, 0x00])]
    #[case(&[0x02, 0x18, 0x00, 0x00, 0x03, 0x18, 0x00])]
    fn test_service_solicitation_uuid32_ad_struct_parsing_failure(#[case] input: &[u8]) {
        assert!(list_of_solicitation_service_uuid32_ad_struct(input).is_err());
    }

    #[rstest]
    #[case(&[], &[])]
    #[case(
        &[0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5],
        &[Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)]
    )]
    fn test_service_solicitation_uuid128_ad_struct_parsing_success(
        #[case] input: &[u8],
        #[case] uuids: &[Uuid128],
    ) {
        assert_eq!(
            list_of_solicitation_service_uuid128_ad_struct(input),
            Ok((
                &[] as &[u8],
                AdStruct::ServiceSolicitationUuid128(
                    ServiceSolicitationUuid128AdStruct::try_new(uuids).unwrap()
                )
            ))
        );
    }

    #[rstest]
    #[case(
        &[0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5,
            0x96, 0x6D, 0xA5, 0xE5, 0x82, 0x4C, 0xD6, 0xB3, 0xC8, 0x4E, 0x6C, 0xA4, 0xC7, 0xBA, 0x24, 0xA6])]
    #[case(&[0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22])]
    fn test_service_solicitation_uuid128_ad_struct_parsing_failure(#[case] input: &[u8]) {
        assert!(list_of_solicitation_service_uuid128_ad_struct(input).is_err());
    }
}
