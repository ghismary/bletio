use bletio_utils::EncodeToBuffer;
use heapless::Vec;

use crate::advertising::AdvertisingError;
use crate::assigned_numbers::{AdType, ServiceUuid};
use crate::uuid::{Uuid128, Uuid32};

const SERVICE_UUID16_NB_MAX: usize = 14;
const SERVICE_UUID32_NB_MAX: usize = 7;
const SERVICE_UUID128_NB_MAX: usize = 1;

/// Whether a service list is complete or not.
///
/// Used when creating list of UUID16, UUID32 or UUID128 services Advertising Structures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceListComplete {
    Complete,
    Incomplete,
}

/// List of 16-bit Bluetooth Service or Service Class UUIDs.
///
/// This list can be complete or incomplete. If the list is empty, it shall be marked as complete,
/// as defined in [Supplement to the Bluetooth Core Specification, Part A, 1.1](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-b1d0edbc-fc9e-507a-efe4-3fd4b4817a52).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ServiceUuid16AdStruct {
    uuids: Vec<ServiceUuid, SERVICE_UUID16_NB_MAX>,
    ad_type: AdType,
}

impl ServiceUuid16AdStruct {
    pub(crate) fn try_new(
        uuids: &[ServiceUuid],
        complete: ServiceListComplete,
    ) -> Result<Self, AdvertisingError> {
        if uuids.is_empty() && complete == ServiceListComplete::Incomplete {
            return Err(AdvertisingError::EmptyServiceUuidListShallBeComplete);
        }
        Ok(Self {
            ad_type: Self::ad_type(complete),
            uuids: uuids
                .try_into()
                .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?,
        })
    }

    const fn ad_type(complete: ServiceListComplete) -> AdType {
        match complete {
            ServiceListComplete::Complete => AdType::CompleteListOfServiceUuid16,
            ServiceListComplete::Incomplete => AdType::IncompleteListOfServiceUuid16,
        }
    }
}

impl EncodeToBuffer for ServiceUuid16AdStruct {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push((self.encoded_size() - 1) as u8)?;
        buffer.try_push(self.ad_type as u8)?;
        for uuid in self.uuids.iter() {
            buffer.encode_le_u16(*uuid as u16)?;
        }
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        (self.uuids.len() * 2) + 2
    }
}

/// List of 32-bit Bluetooth Service or Service Class UUIDs.
///
/// This list can be complete or incomplete. If the list is empty, it shall be marked as complete,
/// as defined in [Supplement to the Bluetooth Core Specification, Part A, 1.1](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-b1d0edbc-fc9e-507a-efe4-3fd4b4817a52).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ServiceUuid32AdStruct {
    uuids: Vec<Uuid32, SERVICE_UUID32_NB_MAX>,
    ad_type: AdType,
}

impl ServiceUuid32AdStruct {
    pub(crate) fn try_new(
        uuids: &[Uuid32],
        complete: ServiceListComplete,
    ) -> Result<Self, AdvertisingError> {
        if uuids.is_empty() && complete == ServiceListComplete::Incomplete {
            return Err(AdvertisingError::EmptyServiceUuidListShallBeComplete);
        }
        Ok(Self {
            ad_type: Self::ad_type(complete),
            uuids: uuids
                .try_into()
                .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?,
        })
    }

    const fn ad_type(complete: ServiceListComplete) -> AdType {
        match complete {
            ServiceListComplete::Complete => AdType::CompleteListOfServiceUuid32,
            ServiceListComplete::Incomplete => AdType::IncompleteListOfServiceUuid32,
        }
    }
}

impl EncodeToBuffer for ServiceUuid32AdStruct {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push((self.encoded_size() - 1) as u8)?;
        buffer.try_push(self.ad_type as u8)?;
        for uuid in self.uuids.iter() {
            buffer.encode_le_u32(uuid.0)?;
        }
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        (self.uuids.len() * 4) + 2
    }
}

/// List of Global 128-bit Service UUIDs.
///
/// This list can be complete or incomplete. If the list is empty, it shall be marked as complete,
/// as defined in [Supplement to the Bluetooth Core Specification, Part A, 1.1](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-b1d0edbc-fc9e-507a-efe4-3fd4b4817a52).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ServiceUuid128AdStruct {
    uuids: Vec<Uuid128, SERVICE_UUID128_NB_MAX>,
    ad_type: AdType,
}

impl ServiceUuid128AdStruct {
    pub(crate) fn try_new(
        uuids: &[Uuid128],
        complete: ServiceListComplete,
    ) -> Result<Self, AdvertisingError> {
        if uuids.is_empty() && complete == ServiceListComplete::Incomplete {
            return Err(AdvertisingError::EmptyServiceUuidListShallBeComplete);
        }
        Ok(Self {
            ad_type: Self::ad_type(complete),
            uuids: uuids
                .try_into()
                .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?,
        })
    }

    const fn ad_type(complete: ServiceListComplete) -> AdType {
        match complete {
            ServiceListComplete::Complete => AdType::CompleteListOfServiceUuid128,
            ServiceListComplete::Incomplete => AdType::IncompleteListOfServiceUuid128,
        }
    }
}

impl EncodeToBuffer for ServiceUuid128AdStruct {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push((self.encoded_size() - 1) as u8)?;
        buffer.try_push(self.ad_type as u8)?;
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
    use nom::combinator::{fail, map_res};
    use nom::{IResult, Parser};

    use crate::advertising::ad_struct::{AdStruct, ServiceUuid16AdStruct};
    use crate::advertising::advertising_data::parser::{service_uuid, uuid128, uuid32};

    use super::*;

    pub(crate) fn complete_list_of_service_uuid16_ad_struct(
        mut input: &[u8],
    ) -> IResult<&[u8], AdStruct> {
        let len = input.len() / 2;
        if (len > SERVICE_UUID16_NB_MAX) || ((input.len() % 2) != 0) {
            fail::<_, &[u8], _>().parse(input)?;
        }
        let mut ad_struct = ServiceUuid16AdStruct {
            uuids: Default::default(),
            ad_type: AdType::CompleteListOfServiceUuid16,
        };
        let mut index = 0;
        while index < len {
            let (rest, _) =
                map_res(service_uuid, |uuid| ad_struct.uuids.push(uuid)).parse(input)?;
            input = rest;
            index += 1;
        }
        Ok((&[], AdStruct::ServiceUuid16(ad_struct)))
    }

    pub(crate) fn incomplete_list_of_service_uuid16_ad_struct(
        mut input: &[u8],
    ) -> IResult<&[u8], AdStruct> {
        let len = input.len() / 2;
        if !(1..=SERVICE_UUID16_NB_MAX).contains(&len) || ((input.len() % 2) != 0) {
            fail::<_, &[u8], _>().parse(input)?;
        }
        let mut ad_struct = ServiceUuid16AdStruct {
            uuids: Default::default(),
            ad_type: AdType::IncompleteListOfServiceUuid16,
        };
        let mut index = 0;
        while index < len {
            let (rest, _) =
                map_res(service_uuid, |uuid| ad_struct.uuids.push(uuid)).parse(input)?;
            input = rest;
            index += 1;
        }
        Ok((&[], AdStruct::ServiceUuid16(ad_struct)))
    }

    pub(crate) fn complete_list_of_service_uuid32_ad_struct(
        mut input: &[u8],
    ) -> IResult<&[u8], AdStruct> {
        let len = input.len() / 4;
        if (len > SERVICE_UUID32_NB_MAX) || ((input.len() % 4) != 0) {
            fail::<_, &[u8], _>().parse(input)?;
        }
        let mut ad_struct = ServiceUuid32AdStruct {
            uuids: Default::default(),
            ad_type: AdType::CompleteListOfServiceUuid32,
        };
        let mut index = 0;
        while index < len {
            let (rest, _) = map_res(uuid32, |uuid| ad_struct.uuids.push(uuid)).parse(input)?;
            input = rest;
            index += 1;
        }
        Ok((&[], AdStruct::ServiceUuid32(ad_struct)))
    }

    pub(crate) fn incomplete_list_of_service_uuid32_ad_struct(
        mut input: &[u8],
    ) -> IResult<&[u8], AdStruct> {
        let len = input.len() / 4;
        if !(1..=SERVICE_UUID32_NB_MAX).contains(&len) || ((input.len() % 4) != 0) {
            fail::<_, &[u8], _>().parse(input)?;
        }
        let mut ad_struct = ServiceUuid32AdStruct {
            uuids: Default::default(),
            ad_type: AdType::IncompleteListOfServiceUuid32,
        };
        let mut index = 0;
        while index < len {
            let (rest, _) = map_res(uuid32, |uuid| ad_struct.uuids.push(uuid)).parse(input)?;
            input = rest;
            index += 1;
        }
        Ok((&[], AdStruct::ServiceUuid32(ad_struct)))
    }

    pub(crate) fn complete_list_of_service_uuid128_ad_struct(
        mut input: &[u8],
    ) -> IResult<&[u8], AdStruct> {
        let len = input.len() / 16;
        if (len > SERVICE_UUID128_NB_MAX) || ((input.len() % 16) != 0) {
            fail::<_, &[u8], _>().parse(input)?;
        }
        let mut ad_struct = ServiceUuid128AdStruct {
            uuids: Default::default(),
            ad_type: AdType::CompleteListOfServiceUuid128,
        };
        let mut index = 0;
        while index < len {
            let (rest, _) = map_res(uuid128, |uuid| ad_struct.uuids.push(uuid)).parse(input)?;
            input = rest;
            index += 1;
        }
        Ok((&[], AdStruct::ServiceUuid128(ad_struct)))
    }

    pub(crate) fn incomplete_list_of_service_uuid128_ad_struct(
        mut input: &[u8],
    ) -> IResult<&[u8], AdStruct> {
        let len = input.len() / 16;
        if !(1..=SERVICE_UUID128_NB_MAX).contains(&len) || ((input.len() % 16) != 0) {
            fail::<_, &[u8], _>().parse(input)?;
        }
        let mut ad_struct = ServiceUuid128AdStruct {
            uuids: Default::default(),
            ad_type: AdType::IncompleteListOfServiceUuid128,
        };
        let mut index = 0;
        while index < len {
            let (rest, _) = map_res(uuid128, |uuid| ad_struct.uuids.push(uuid)).parse(input)?;
            input = rest;
            index += 1;
        }
        Ok((&[], AdStruct::ServiceUuid128(ad_struct)))
    }
}

#[cfg(test)]
mod test {
    use bletio_utils::{Buffer, BufferOps};
    use rstest::rstest;

    use crate::advertising::ad_struct::AdStruct;

    use super::{parser::*, *};

    #[rstest]
    #[case(ServiceListComplete::Complete, AdType::CompleteListOfServiceUuid16)]
    #[case(ServiceListComplete::Incomplete, AdType::IncompleteListOfServiceUuid16)]
    fn test_service_uudi16_ad_type(#[case] input: ServiceListComplete, #[case] expected: AdType) {
        assert_eq!(ServiceUuid16AdStruct::ad_type(input), expected);
    }

    #[rstest]
    #[case(ServiceListComplete::Complete, AdType::CompleteListOfServiceUuid32)]
    #[case(ServiceListComplete::Incomplete, AdType::IncompleteListOfServiceUuid32)]
    fn test_service_uudi32_ad_type(#[case] input: ServiceListComplete, #[case] expected: AdType) {
        assert_eq!(ServiceUuid32AdStruct::ad_type(input), expected);
    }

    #[rstest]
    #[case(ServiceListComplete::Complete, AdType::CompleteListOfServiceUuid128)]
    #[case(
        ServiceListComplete::Incomplete,
        AdType::IncompleteListOfServiceUuid128
    )]
    fn test_service_uudi128_ad_type(#[case] input: ServiceListComplete, #[case] expected: AdType) {
        assert_eq!(ServiceUuid128AdStruct::ad_type(input), expected);
    }

    #[rstest]
    #[case(&[], ServiceListComplete::Complete, AdType::CompleteListOfServiceUuid16, &[0x01, 0x03])]
    #[case(
        &[ServiceUuid::LinkLoss, ServiceUuid::Battery, ServiceUuid::EnvironmentalSensing],
        ServiceListComplete::Incomplete, AdType::IncompleteListOfServiceUuid16,
        &[0x07, 0x02, 0x03, 0x18, 0x0F, 0x18, 0x1A, 0x18]
    )]
    #[case(
        &[ServiceUuid::Glucose, ServiceUuid::HeartRate, ServiceUuid::Battery, ServiceUuid::BloodPressure],
        ServiceListComplete::Complete, AdType::CompleteListOfServiceUuid16,
        &[0x09, 0x03, 0x08, 0x18, 0x0D, 0x18, 0x0F, 0x18, 0x10, 0x18]
    )]
    fn test_service_uui16_ad_struct_success(
        #[case] uuids: &[ServiceUuid],
        #[case] complete: ServiceListComplete,
        #[case] ad_type: AdType,
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<31>::default();
        let value = ServiceUuid16AdStruct::try_new(uuids, complete).unwrap();
        assert_eq!(value.uuids, uuids);
        assert_eq!(value.ad_type, ad_type);
        value.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }

    #[test]
    fn test_service_uuid16_ad_struct_failure() {
        let err = ServiceUuid16AdStruct::try_new(
            &[
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
            ],
            ServiceListComplete::Complete,
        );
        assert_eq!(
            err,
            Err(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
        );

        let err = ServiceUuid16AdStruct::try_new(&[], ServiceListComplete::Incomplete);
        assert_eq!(
            err,
            Err(AdvertisingError::EmptyServiceUuidListShallBeComplete)
        );
    }

    #[rstest]
    #[case(&[], ServiceListComplete::Complete, AdType::CompleteListOfServiceUuid32, &[0x01, 0x05])]
    #[case(
        &[Uuid32(0x0000_1803), Uuid32(0x0000_180F), Uuid32(0x0000_181A)],
        ServiceListComplete::Incomplete, AdType::IncompleteListOfServiceUuid32,
        &[0x0D, 0x04, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00, 0x1A, 0x18, 0x00, 0x00]
    )]
    #[case(
        &[Uuid32(0x0000_1808), Uuid32(0x0000_180D), Uuid32(0x0000_180F), Uuid32(0x0000_1810)],
        ServiceListComplete::Complete, AdType::CompleteListOfServiceUuid32,
        &[0x11, 0x05, 0x08, 0x18, 0x00, 0x00, 0x0D, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00, 0x10, 0x18, 0x00, 0x00]
    )]
    fn test_service_uui32_ad_struct_success(
        #[case] uuids: &[Uuid32],
        #[case] complete: ServiceListComplete,
        #[case] ad_type: AdType,
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<31>::default();
        let value = ServiceUuid32AdStruct::try_new(uuids, complete).unwrap();
        assert_eq!(value.uuids, uuids);
        assert_eq!(value.ad_type, ad_type);
        value.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }

    #[test]
    fn test_service_uuid32_ad_struct_failure() {
        let err = ServiceUuid32AdStruct::try_new(
            &[
                Uuid32(0x0000_1802),
                Uuid32(0x0000_1803),
                Uuid32(0x0000_1804),
                Uuid32(0x0000_1805),
                Uuid32(0x0000_1806),
                Uuid32(0x0000_1807),
                Uuid32(0x0000_1808),
                Uuid32(0x0000_1809),
            ],
            ServiceListComplete::Complete,
        );
        assert_eq!(
            err,
            Err(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
        );

        let err = ServiceUuid32AdStruct::try_new(&[], ServiceListComplete::Incomplete);
        assert_eq!(
            err,
            Err(AdvertisingError::EmptyServiceUuidListShallBeComplete)
        );
    }

    #[rstest]
    #[case(&[], ServiceListComplete::Complete, AdType::CompleteListOfServiceUuid128, &[0x01, 0x07])]
    #[case(
        &[Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)],
        ServiceListComplete::Incomplete, AdType::IncompleteListOfServiceUuid128,
        &[0x11, 0x06, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5]
    )]
    #[case(
        &[Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)],
        ServiceListComplete::Complete, AdType::CompleteListOfServiceUuid128,
        &[0x11, 0x07, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5]
    )]
    fn test_service_uui128_ad_struct_success(
        #[case] uuids: &[Uuid128],
        #[case] complete: ServiceListComplete,
        #[case] ad_type: AdType,
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<31>::default();
        let value = ServiceUuid128AdStruct::try_new(uuids, complete).unwrap();
        assert_eq!(value.uuids, uuids);
        assert_eq!(value.ad_type, ad_type);
        value.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }

    #[test]
    fn test_service_uuid128_ad_struct_failure() {
        let err = ServiceUuid128AdStruct::try_new(
            &[
                Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640),
                Uuid128(0xA624BAC7_A46C_4EC8_B3D6_4C82E5A56D96),
            ],
            ServiceListComplete::Complete,
        );
        assert_eq!(
            err,
            Err(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
        );

        let err = ServiceUuid128AdStruct::try_new(&[], ServiceListComplete::Incomplete);
        assert_eq!(
            err,
            Err(AdvertisingError::EmptyServiceUuidListShallBeComplete)
        );
    }

    #[rstest]
    #[case(&[], &[])]
    #[case(&[0x03, 0x18, 0x0F, 0x18, 0x1A, 0x18], &[ServiceUuid::LinkLoss, ServiceUuid::Battery, ServiceUuid::EnvironmentalSensing])]
    #[case(
        &[0x08, 0x18, 0x0D, 0x18, 0x0F, 0x18, 0x10, 0x18],
        &[ServiceUuid::Glucose, ServiceUuid::HeartRate, ServiceUuid::Battery, ServiceUuid::BloodPressure]
    )]
    fn test_complete_list_of_service_uuid16_ad_struct_parsing_success(
        #[case] input: &[u8],
        #[case] uuids: &[ServiceUuid],
    ) {
        assert_eq!(
            complete_list_of_service_uuid16_ad_struct(input),
            Ok((
                &[] as &[u8],
                AdStruct::ServiceUuid16(
                    ServiceUuid16AdStruct::try_new(uuids, ServiceListComplete::Complete).unwrap()
                )
            ))
        );
    }

    #[rstest]
    #[case(
        &[0x02, 0x18, 0x03, 0x18, 0x04, 0x18, 0x05, 0x18, 0x06, 0x18, 0x07, 0x18, 0x08, 0x18,
            0x09, 0x18, 0x0A, 0x18, 0x0D, 0x18, 0x0E, 0x18, 0x0F, 0x18, 0x10, 0x18, 0x11, 0x18, 0x12, 0x18])]
    #[case(&[0x02, 0x18, 0x03])]
    fn test_complete_list_of_service_uuid16_ad_struct_parsing_failure(#[case] input: &[u8]) {
        assert!(complete_list_of_service_uuid16_ad_struct(input).is_err());
    }

    #[rstest]
    #[case(&[0x03, 0x18, 0x0F, 0x18, 0x1A, 0x18], &[ServiceUuid::LinkLoss, ServiceUuid::Battery, ServiceUuid::EnvironmentalSensing])]
    #[case(
        &[0x08, 0x18, 0x0D, 0x18, 0x0F, 0x18, 0x10, 0x18],
        &[ServiceUuid::Glucose, ServiceUuid::HeartRate, ServiceUuid::Battery, ServiceUuid::BloodPressure]
    )]
    fn test_incomplete_list_of_service_uuid16_ad_struct_parsing_success(
        #[case] input: &[u8],
        #[case] uuids: &[ServiceUuid],
    ) {
        assert_eq!(
            incomplete_list_of_service_uuid16_ad_struct(input),
            Ok((
                &[] as &[u8],
                AdStruct::ServiceUuid16(
                    ServiceUuid16AdStruct::try_new(uuids, ServiceListComplete::Incomplete).unwrap()
                )
            ))
        );
    }

    #[rstest]
    #[case(&[])]
    #[case(
        &[0x02, 0x18, 0x03, 0x18, 0x04, 0x18, 0x05, 0x18, 0x06, 0x18, 0x07, 0x18, 0x08, 0x18,
            0x09, 0x18, 0x0A, 0x18, 0x0D, 0x18, 0x0E, 0x18, 0x0F, 0x18, 0x10, 0x18, 0x11, 0x18, 0x12, 0x18])]
    #[case(&[0x02, 0x18, 0x03])]
    fn test_incomplete_list_of_service_uuid16_ad_struct_parsing_failure(#[case] input: &[u8]) {
        assert!(incomplete_list_of_service_uuid16_ad_struct(input).is_err());
    }

    #[rstest]
    #[case(&[], &[])]
    #[case(
        &[0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00, 0x1A, 0x18, 0x00, 0x00],
        &[Uuid32(0x0000_1803), Uuid32(0x0000_180F), Uuid32(0x0000_181A)]
    )]
    #[case(
        &[0x08, 0x18, 0x00, 0x00, 0x0D, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00, 0x10, 0x18, 0x00, 0x00],
        &[Uuid32(0x0000_1808), Uuid32(0x0000_180D), Uuid32(0x0000_180F), Uuid32(0x0000_1810)]
    )]
    fn test_complete_list_of_service_uuid32_ad_struct_parsing_success(
        #[case] input: &[u8],
        #[case] uuids: &[Uuid32],
    ) {
        assert_eq!(
            complete_list_of_service_uuid32_ad_struct(input),
            Ok((
                &[] as &[u8],
                AdStruct::ServiceUuid32(
                    ServiceUuid32AdStruct::try_new(uuids, ServiceListComplete::Complete).unwrap()
                )
            ))
        );
    }

    #[rstest]
    #[case(
        &[0x02, 0x18, 0x00, 0x00, 0x03, 0x18, 0x00, 0x00, 0x04, 0x18, 0x00, 0x00, 0x05, 0x18, 0x00, 0x00,
            0x06, 0x18, 0x00, 0x00, 0x07, 0x18, 0x00, 0x00, 0x08, 0x18, 0x00, 0x00, 0x09, 0x18, 0x00, 0x00])]
    #[case(&[0x02, 0x18, 0x00])]
    fn test_complete_list_of_service_uuid32_ad_struct_parsing_failure(#[case] input: &[u8]) {
        assert!(complete_list_of_service_uuid32_ad_struct(input).is_err());
    }

    #[rstest]
    #[case(
        &[0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00, 0x1A, 0x18, 0x00, 0x00],
        &[Uuid32(0x0000_1803), Uuid32(0x0000_180F), Uuid32(0x0000_181A)]
    )]
    #[case(
        &[0x08, 0x18, 0x00, 0x00, 0x0D, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00, 0x10, 0x18, 0x00, 0x00],
        &[Uuid32(0x0000_1808), Uuid32(0x0000_180D), Uuid32(0x0000_180F), Uuid32(0x0000_1810)]
    )]
    fn test_incomplete_list_of_service_uuid32_ad_struct_parsing_success(
        #[case] input: &[u8],
        #[case] uuids: &[Uuid32],
    ) {
        assert_eq!(
            incomplete_list_of_service_uuid32_ad_struct(input),
            Ok((
                &[] as &[u8],
                AdStruct::ServiceUuid32(
                    ServiceUuid32AdStruct::try_new(uuids, ServiceListComplete::Incomplete).unwrap()
                )
            ))
        );
    }

    #[rstest]
    #[case(&[])]
    #[case(
        &[0x02, 0x18, 0x00, 0x00, 0x03, 0x18, 0x00, 0x00, 0x04, 0x18, 0x00, 0x00, 0x05, 0x18, 0x00, 0x00,
            0x06, 0x18, 0x00, 0x00, 0x07, 0x18, 0x00, 0x00, 0x08, 0x18, 0x00, 0x00, 0x09, 0x18, 0x00, 0x00])]
    #[case(&[0x02, 0x18, 0x00, 0x00, 0x03, 0x18])]
    fn test_incomplete_list_of_service_uuid32_ad_struct_parsing_failure(#[case] input: &[u8]) {
        assert!(incomplete_list_of_service_uuid32_ad_struct(input).is_err());
    }

    #[rstest]
    #[case(&[], &[])]
    #[case(
        &[0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5],
        &[Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)]
    )]
    #[case(
        &[0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5],
        &[Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)]
    )]
    fn test_complete_list_of_service_uuid128_ad_struct_parsing_success(
        #[case] input: &[u8],
        #[case] uuids: &[Uuid128],
    ) {
        assert_eq!(
            complete_list_of_service_uuid128_ad_struct(input),
            Ok((
                &[] as &[u8],
                AdStruct::ServiceUuid128(
                    ServiceUuid128AdStruct::try_new(uuids, ServiceListComplete::Complete).unwrap()
                )
            ))
        );
    }

    #[rstest]
    #[case(
        &[0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5,
            0x96, 0x6D, 0xA5, 0xE5, 0x82, 0x4C, 0xD6, 0xB3, 0xC8, 0x4E, 0x6C, 0xA4, 0xC7, 0xBA, 0x24, 0xA6])]
    #[case(&[0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D])]
    fn test_complete_list_of_service_uuid128_ad_struct_parsing_failure(#[case] input: &[u8]) {
        assert!(complete_list_of_service_uuid128_ad_struct(input).is_err());
    }

    #[rstest]
    #[case(
        &[0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5],
        &[Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)]
    )]
    #[case(
        &[0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5],
        &[Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)]
    )]
    fn test_incomplete_list_of_service_uuid128_ad_struct_parsing_success(
        #[case] input: &[u8],
        #[case] uuids: &[Uuid128],
    ) {
        assert_eq!(
            incomplete_list_of_service_uuid128_ad_struct(input),
            Ok((
                &[] as &[u8],
                AdStruct::ServiceUuid128(
                    ServiceUuid128AdStruct::try_new(uuids, ServiceListComplete::Incomplete)
                        .unwrap()
                )
            ))
        );
    }

    #[rstest]
    #[case(&[])]
    #[case(
        &[0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5,
            0x96, 0x6D, 0xA5, 0xE5, 0x82, 0x4C, 0xD6, 0xB3, 0xC8, 0x4E, 0x6C, 0xA4, 0xC7, 0xBA, 0x24, 0xA6])]
    #[case(&[0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D])]
    fn test_incomplete_list_of_service_uuid128_ad_struct_parsing_failure(#[case] input: &[u8]) {
        assert!(incomplete_list_of_service_uuid128_ad_struct(input).is_err());
    }
}
