//! Advertising structures contained in [AdvertisingData](crate::advertising::AdvertisingData)
//! and [ScanResponseData](crate::advertising::ScanResponseData).
//!
//! The format for the [AdvertisingData](crate::advertising::AdvertisingData) and
//! [ScanResponseData](crate::advertising::ScanResponseData) is defined in the
//! [Core Specification 6.0, Vol.3, Part C, 11](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-51247611-bdce-274e-095c-afb6d879c55c).
//!
//! The formats of each advertising structures and their meanings are defined in the
//! [Core Specification Supplement, Part A, 1](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-36b7e551-d4cf-9ae3-a8ee-0482fbc1d5bc).

use bitflags::bitflags;

mod advertising_interval;
mod appearance;
pub(crate) mod flags;
mod le_supported_features;
mod manufacturer_specific_data;
mod peripheral_connection_interval_range;
mod service_solicitation;
mod service_uuid;
pub(crate) mod tx_power_level;
pub mod uri;

use crate::advertising::AdvertisingError;
use crate::assigned_numbers::AdType;
use crate::utils::{Buffer, BufferOps};
pub use advertising_interval::AdvertisingIntervalAdStruct;
pub use appearance::AppearanceAdStruct;
pub use flags::FlagsAdStruct;
pub use le_supported_features::LeSupportedFeaturesAdStruct;
pub use manufacturer_specific_data::ManufacturerSpecificDataAdStruct;
pub use peripheral_connection_interval_range::PeripheralConnectionIntervalRangeAdStruct;
pub use service_solicitation::{
    ServiceSolicitationUuid128AdStruct, ServiceSolicitationUuid16AdStruct,
    ServiceSolicitationUuid32AdStruct,
};
pub use service_uuid::{
    ServiceListComplete, ServiceUuid128AdStruct, ServiceUuid16AdStruct, ServiceUuid32AdStruct,
};
pub use tx_power_level::TxPowerLevelAdStruct;
pub use uri::{custom_uri_scheme, CustomUriScheme, Uri, UriAdStruct, UriScheme};

pub(crate) trait AdStruct {
    fn encoded_data(&self) -> &[u8];
    fn r#type(&self) -> AdStructType;
    fn is_unique(&self) -> bool;
}

#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct AdStructType(u16);

bitflags! {
    impl AdStructType: u16 {
        const SERVICE_UUID16 = 1 << 0;
        const SERVICE_UUID32 = 1 << 1;
        const SERVICE_UUID128 = 1 << 2;
        const FLAGS = 1 << 3;
        const MANUFACTURER_SPECIFIC_DATA = 1 << 4;
        const TX_POWER_LEVEL = 1 << 5;
        const PERIPHERAL_CONNECTION_INTERVAL_RANGE = 1 << 6;
        const SERVICE_SOLICITATION_UUID16 = 1 << 7;
        const SERVICE_SOLICITATION_UUID32 = 1 << 8;
        const SERVICE_SOLICITATION_UUID128 = 1 << 9;
        const APPEARANCE = 1 << 10;
        const ADVERTISING_INTERVAL = 1 << 11;
        const URI = 1 << 12;
        const LE_SUPPORTED_FEATURES = 1 << 13;
    }
}

const AD_STRUCT_LENGTH_OFFSET: usize = 0;
const AD_STRUCT_TYPE_OFFSET: usize = 1;
const AD_STRUCT_DATA_OFFSET: usize = 2;

#[derive(Debug, Clone)]
pub(crate) struct AdStructBuffer<const CAP: usize> {
    buffer: Buffer<CAP>,
}

impl<const CAP: usize> AdStructBuffer<CAP> {
    pub(crate) fn new(ad_type: AdType) -> Self {
        let mut s = Self::default();
        s.set_ad_type(ad_type);
        s
    }

    pub(crate) fn copy_from_slice(&mut self, data: &[u8]) -> Result<(), AdvertisingError> {
        self.buffer
            .copy_from_slice(data)
            .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?;
        self.buffer.data[AD_STRUCT_LENGTH_OFFSET] += data.len() as u8;
        Ok(())
    }

    pub(crate) fn data(&self) -> &[u8] {
        self.buffer.data()
    }

    pub(crate) fn encode_le_u16(&mut self, data: u16) -> Result<(), AdvertisingError> {
        self.buffer
            .encode_le_u16(data)
            .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?;
        self.buffer.data[AD_STRUCT_LENGTH_OFFSET] += 2;
        Ok(())
    }

    pub(crate) fn encode_le_u32(&mut self, data: u32) -> Result<(), AdvertisingError> {
        self.buffer
            .encode_le_u32(data)
            .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?;
        self.buffer.data[AD_STRUCT_LENGTH_OFFSET] += 4;
        Ok(())
    }

    pub(crate) fn encode_le_u64(&mut self, data: u64) -> Result<(), AdvertisingError> {
        self.buffer
            .encode_le_u64(data)
            .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?;
        self.buffer.data[AD_STRUCT_LENGTH_OFFSET] += 8;
        Ok(())
    }

    pub(crate) fn encode_le_u128(&mut self, data: u128) -> Result<(), AdvertisingError> {
        self.buffer
            .encode_le_u128(data)
            .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?;
        self.buffer.data[AD_STRUCT_LENGTH_OFFSET] += 16;
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.buffer.offset - AD_STRUCT_DATA_OFFSET
    }

    pub(crate) fn try_push(&mut self, data: u8) -> Result<(), AdvertisingError> {
        self.buffer
            .try_push(data)
            .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?;
        self.buffer.data[AD_STRUCT_LENGTH_OFFSET] += 1;
        Ok(())
    }

    #[cfg(test)]
    // TODO: Return an AdType
    fn ad_type(&self) -> u8 {
        self.buffer.data[AD_STRUCT_TYPE_OFFSET]
    }

    fn set_ad_type(&mut self, ad_type: AdType) {
        self.buffer.data[AD_STRUCT_TYPE_OFFSET] = ad_type as u8;
    }
}

impl<const CAP: usize> Default for AdStructBuffer<CAP> {
    fn default() -> Self {
        let mut buffer = Buffer {
            data: [0; CAP],
            offset: AD_STRUCT_DATA_OFFSET,
        };
        buffer.data[AD_STRUCT_LENGTH_OFFSET] = 1;
        Self { buffer }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_adstructbuffer() {
        let mut buffer = AdStructBuffer::<8>::new(AdType::PeripheralConnectionIntervalRange);
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
        assert_eq!(
            buffer.ad_type(),
            AdType::PeripheralConnectionIntervalRange as u8
        );
        assert_eq!(buffer.data(), &[0x01, 0x12]);

        buffer.set_ad_type(AdType::Appearance);
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.ad_type(), AdType::Appearance as u8);
        assert_eq!(buffer.data(), &[0x01, 0x19]);
    }

    #[test]
    fn test_adstructbuffer_copy_from_slice_success() -> Result<(), AdvertisingError> {
        let mut buffer = AdStructBuffer::<8>::new(AdType::ShortenedLocalName);
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.ad_type(), AdType::ShortenedLocalName as u8);
        assert_eq!(buffer.data(), &[0x01, 0x08]);

        buffer.copy_from_slice("name".as_bytes())?;
        assert!(!buffer.is_empty());
        assert_eq!(buffer.len(), 4);
        assert_eq!(buffer.ad_type(), AdType::ShortenedLocalName as u8);
        assert_eq!(buffer.data(), &[0x05, 0x08, 0x6E, 0x61, 0x6D, 0x65]);

        Ok(())
    }

    #[test]
    fn test_adstructbuffer_copy_from_slice_failure() {
        let mut buffer = AdStructBuffer::<8>::new(AdType::CompleteLocalName);
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.ad_type(), AdType::CompleteLocalName as u8);
        assert_eq!(buffer.data(), &[0x01, 0x09]);

        let err = buffer
            .copy_from_slice("complete-name".as_bytes())
            .expect_err("Buffer too small");
        assert!(matches!(
            err,
            AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket
        ));
    }

    #[test]
    fn test_adstructbuffer_encode_le_u16_success() -> Result<(), AdvertisingError> {
        let mut buffer = AdStructBuffer::<8>::new(AdType::CompleteListOfServiceUuid16);
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.ad_type(), AdType::CompleteListOfServiceUuid16 as u8);
        assert_eq!(buffer.data(), &[0x01, 0x03]);

        buffer.encode_le_u16(0x180F)?;
        assert!(!buffer.is_empty());
        assert_eq!(buffer.len(), 2);
        assert_eq!(buffer.ad_type(), AdType::CompleteListOfServiceUuid16 as u8);
        assert_eq!(buffer.data(), &[0x03, 0x03, 0x0F, 0x18]);

        Ok(())
    }

    #[test]
    fn test_adstructbuffer_encode_le_u16_failure() {
        let mut buffer = AdStructBuffer::<3>::new(AdType::CompleteListOfServiceUuid16);
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.ad_type(), AdType::CompleteListOfServiceUuid16 as u8);
        assert_eq!(buffer.data(), &[0x01, 0x03]);

        let err = buffer.encode_le_u16(0x180F).expect_err("Buffer too small");
        assert!(matches!(
            err,
            AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket
        ));
    }

    #[test]
    fn test_adstructbuffer_encode_le_u32_success() -> Result<(), AdvertisingError> {
        let mut buffer = AdStructBuffer::<8>::new(AdType::CompleteListOfServiceUuid32);
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.ad_type(), AdType::CompleteListOfServiceUuid32 as u8);
        assert_eq!(buffer.data(), &[0x01, 0x05]);

        buffer.encode_le_u32(0x0000180F)?;
        assert!(!buffer.is_empty());
        assert_eq!(buffer.len(), 4);
        assert_eq!(buffer.ad_type(), AdType::CompleteListOfServiceUuid32 as u8);
        assert_eq!(buffer.data(), &[0x05, 0x05, 0x0F, 0x18, 0x00, 0x00]);

        Ok(())
    }

    #[test]
    fn test_adstructbuffer_encode_le_u32_failure() {
        let mut buffer = AdStructBuffer::<4>::new(AdType::CompleteListOfServiceUuid32);
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.ad_type(), AdType::CompleteListOfServiceUuid32 as u8);
        assert_eq!(buffer.data(), &[0x01, 0x05]);

        let err = buffer
            .encode_le_u32(0x0000180F)
            .expect_err("Buffer too small");
        assert!(matches!(
            err,
            AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket
        ));
    }

    #[test]
    fn test_adstructbuffer_encode_le_u64_success() -> Result<(), AdvertisingError> {
        let mut buffer = AdStructBuffer::<12>::new(AdType::LeSupportedFeatures);
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.ad_type(), AdType::LeSupportedFeatures as u8);
        assert_eq!(buffer.data(), &[0x01, 0x27]);

        buffer.encode_le_u64(0x0102030405060708)?;
        assert!(!buffer.is_empty());
        assert_eq!(buffer.len(), 8);
        assert_eq!(buffer.ad_type(), AdType::LeSupportedFeatures as u8);
        assert_eq!(
            buffer.data(),
            &[0x09, 0x27, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01]
        );

        Ok(())
    }

    #[test]
    fn test_adstructbuffer_encode_le_u64_failure() {
        let mut buffer = AdStructBuffer::<8>::new(AdType::LeSupportedFeatures);
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.ad_type(), AdType::LeSupportedFeatures as u8);
        assert_eq!(buffer.data(), &[0x01, 0x27]);

        let err = buffer
            .encode_le_u64(0x0102030405060708)
            .expect_err("Buffer too small");
        assert!(matches!(
            err,
            AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket
        ));
    }

    #[test]
    fn test_adstructbuffer_encode_le_u128_success() -> Result<(), AdvertisingError> {
        let mut buffer = AdStructBuffer::<32>::new(AdType::CompleteListOfServiceUuid128);
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.ad_type(), AdType::CompleteListOfServiceUuid128 as u8);
        assert_eq!(buffer.data(), &[0x01, 0x07]);

        buffer.encode_le_u128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)?;
        assert!(!buffer.is_empty());
        assert_eq!(buffer.len(), 16);
        assert_eq!(buffer.ad_type(), AdType::CompleteListOfServiceUuid128 as u8);
        assert_eq!(
            buffer.data(),
            &[
                0x11, 0x07, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22,
                0x7E, 0x28, 0xA1, 0xF5
            ]
        );

        Ok(())
    }

    #[test]
    fn test_adstructbuffer_encode_le_u128_failure() {
        let mut buffer = AdStructBuffer::<16>::new(AdType::CompleteListOfServiceUuid128);
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.ad_type(), AdType::CompleteListOfServiceUuid128 as u8);
        assert_eq!(buffer.data(), &[0x01, 0x07]);

        let err = buffer
            .encode_le_u128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)
            .expect_err("Buffer too small");
        assert!(matches!(
            err,
            AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket
        ));
    }

    #[test]
    fn test_adstructbuffer_try_push_success() -> Result<(), AdvertisingError> {
        let mut buffer = AdStructBuffer::<8>::new(AdType::TxPowerLevel);
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.ad_type(), AdType::TxPowerLevel as u8);
        assert_eq!(buffer.data(), &[0x01, 0x0A]);

        buffer.try_push(0x19)?;
        assert!(!buffer.is_empty());
        assert_eq!(buffer.len(), 1);
        assert_eq!(buffer.ad_type(), AdType::TxPowerLevel as u8);
        assert_eq!(buffer.data(), &[0x02, 0x0A, 0x19]);

        Ok(())
    }

    #[test]
    fn test_adstructbuffer_try_push_failure() {
        let mut buffer = AdStructBuffer::<2>::new(AdType::TxPowerLevel);
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.ad_type(), AdType::TxPowerLevel as u8);
        assert_eq!(buffer.data(), &[0x01, 0x0A]);

        let err = buffer.try_push(0x19).expect_err("Buffer too small");
        assert!(matches!(
            err,
            AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket
        ));
    }
}
