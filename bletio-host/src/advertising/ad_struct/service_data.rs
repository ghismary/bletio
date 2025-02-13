use bletio_utils::EncodeToBuffer;

use crate::assigned_numbers::{AdType, ServiceUuid};
use crate::uuid::{Uuid128, Uuid32};

/// Service Data for 16-bit Service UUIDs.
///
/// See [Supplement to the Bluetooth Core Specification, Part A, 1.11](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-dea15cd4-bc0f-91f0-82c1-3bbe596f7bf6).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ServiceDataUuid16AdStruct<'a> {
    uuid: ServiceUuid,
    data: &'a [u8],
}

impl<'a> ServiceDataUuid16AdStruct<'a> {
    pub(crate) fn new(uuid: ServiceUuid, data: &'a [u8]) -> Self {
        Self { uuid, data }
    }
}

impl EncodeToBuffer for ServiceDataUuid16AdStruct<'_> {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push((self.encoded_size() - 1) as u8)?;
        buffer.try_push(AdType::ServiceDataUuid16 as u8)?;
        buffer.encode_le_u16(self.uuid as u16)?;
        buffer.copy_from_slice(self.data)?;
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
pub(crate) struct ServiceDataUuid32AdStruct<'a> {
    uuid: Uuid32,
    data: &'a [u8],
}

impl<'a> ServiceDataUuid32AdStruct<'a> {
    pub(crate) fn new(uuid: Uuid32, data: &'a [u8]) -> Self {
        Self { uuid, data }
    }
}

impl EncodeToBuffer for ServiceDataUuid32AdStruct<'_> {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push((self.encoded_size() - 1) as u8)?;
        buffer.try_push(AdType::ServiceDataUuid32 as u8)?;
        buffer.encode_le_u32(self.uuid.0)?;
        buffer.copy_from_slice(self.data)?;
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
pub(crate) struct ServiceDataUuid128AdStruct<'a> {
    uuid: Uuid128,
    data: &'a [u8],
}

impl<'a> ServiceDataUuid128AdStruct<'a> {
    pub(crate) fn new(uuid: Uuid128, data: &'a [u8]) -> Self {
        Self { uuid, data }
    }
}

impl EncodeToBuffer for ServiceDataUuid128AdStruct<'_> {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push((self.encoded_size() - 1) as u8)?;
        buffer.try_push(AdType::ServiceDataUuid128 as u8)?;
        buffer.encode_le_u128(self.uuid.0)?;
        buffer.copy_from_slice(self.data)?;
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
        let value = ServiceDataUuid16AdStruct::new(uuid, data);
        value.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }

    #[test]
    fn test_service_data_uuid16_ad_struct_failure() {
        let data = [0u8; 28];
        let mut buffer = Buffer::<31>::default();
        let value = ServiceDataUuid16AdStruct::new(ServiceUuid::ImmediateAlert, &data);
        let err = value.encode(&mut buffer);
        assert_eq!(err, Err(bletio_utils::Error::BufferTooSmall));
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
        let value = ServiceDataUuid32AdStruct::new(uuid, data);
        value.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }

    #[test]
    fn test_service_data_uuid32_ad_struct_failure() {
        let data = [0u8; 26];
        let mut buffer = Buffer::<31>::default();
        let value = ServiceDataUuid32AdStruct::new(Uuid32(0x0000_1802), &data);
        let err = value.encode(&mut buffer);
        assert_eq!(err, Err(bletio_utils::Error::BufferTooSmall));
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
        let value = ServiceDataUuid128AdStruct::new(uuid, data);
        value.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }

    #[test]
    fn test_service_data_uuid128_ad_struct_failure() {
        let data = [0u8; 14];
        let mut buffer = Buffer::<31>::default();
        let value =
            ServiceDataUuid128AdStruct::new(Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640), &data);
        let err = value.encode(&mut buffer);
        assert_eq!(err, Err(bletio_utils::Error::BufferTooSmall));
    }
}
