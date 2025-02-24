//! Advertising structures contained in [AdvertisingData](crate::advertising::AdvertisingData)
//! and [ScanResponseData](crate::advertising::ScanResponseData).
//!
//! The format for the [AdvertisingData](crate::advertising::AdvertisingData) and
//! [ScanResponseData](crate::advertising::ScanResponseData) is defined in the
//! [Core Specification 6.0, Vol.3, Part C, 11](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-51247611-bdce-274e-095c-afb6d879c55c).
//!
//! The formats of each advertising structures and their meanings are defined in the
//! [Core Specification Supplement, Part A, 1](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-36b7e551-d4cf-9ae3-a8ee-0482fbc1d5bc).

pub(crate) mod advertising_interval;
pub(crate) mod appearance;
pub(crate) mod flags;
pub(crate) mod le_supported_features;
pub(crate) mod local_name;
pub(crate) mod manufacturer_specific_data;
pub(crate) mod peripheral_connection_interval_range;
pub(crate) mod public_target_address;
pub(crate) mod random_target_address;
pub(crate) mod service_data;
pub(crate) mod service_solicitation;
pub(crate) mod service_uuid;
pub(crate) mod tx_power_level;
pub(crate) mod uri;

pub(crate) use advertising_interval::AdvertisingIntervalAdStruct;
pub(crate) use appearance::AppearanceAdStruct;
use bletio_utils::EncodeToBuffer;
pub(crate) use flags::FlagsAdStruct;
pub(crate) use le_supported_features::LeSupportedFeaturesAdStruct;
pub(crate) use local_name::LocalNameAdStruct;
pub(crate) use manufacturer_specific_data::ManufacturerSpecificDataAdStruct;
pub(crate) use peripheral_connection_interval_range::PeripheralConnectionIntervalRangeAdStruct;
pub(crate) use public_target_address::PublicTargetAddressAdStruct;
pub(crate) use random_target_address::RandomTargetAddressAdStruct;
pub(crate) use service_data::{
    ServiceDataUuid128AdStruct, ServiceDataUuid16AdStruct, ServiceDataUuid32AdStruct,
};
pub(crate) use service_solicitation::{
    ServiceSolicitationUuid128AdStruct, ServiceSolicitationUuid16AdStruct,
    ServiceSolicitationUuid32AdStruct,
};
pub(crate) use service_uuid::{
    ServiceUuid128AdStruct, ServiceUuid16AdStruct, ServiceUuid32AdStruct,
};
pub(crate) use tx_power_level::TxPowerLevelAdStruct;
pub(crate) use uri::UriAdStruct;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdStruct {
    AdvertisingInterval(AdvertisingIntervalAdStruct),
    Appearance(AppearanceAdStruct),
    Flags(FlagsAdStruct),
    LeSupportedFeatures(LeSupportedFeaturesAdStruct),
    LocalName(LocalNameAdStruct),
    ManufacturerSpecificData(ManufacturerSpecificDataAdStruct),
    PeripheralConnectionIntervalRange(PeripheralConnectionIntervalRangeAdStruct),
    PublicTargetAddress(PublicTargetAddressAdStruct),
    RandomTargetAddress(RandomTargetAddressAdStruct),
    ServiceDataUuid16(ServiceDataUuid16AdStruct),
    ServiceDataUuid32(ServiceDataUuid32AdStruct),
    ServiceDataUuid128(ServiceDataUuid128AdStruct),
    ServiceSolicitationUuid16(ServiceSolicitationUuid16AdStruct),
    ServiceSolicitationUuid32(ServiceSolicitationUuid32AdStruct),
    ServiceSolicitationUuid128(ServiceSolicitationUuid128AdStruct),
    ServiceUuid16(ServiceUuid16AdStruct),
    ServiceUuid32(ServiceUuid32AdStruct),
    ServiceUuid128(ServiceUuid128AdStruct),
    TxPowerLevel(TxPowerLevelAdStruct),
    Uri(UriAdStruct),
    Unhandled(u8),
}

impl EncodeToBuffer for AdStruct {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        match self {
            AdStruct::AdvertisingInterval(v) => v.encode(buffer),
            AdStruct::Appearance(v) => v.encode(buffer),
            AdStruct::Flags(v) => v.encode(buffer),
            AdStruct::LeSupportedFeatures(v) => v.encode(buffer),
            AdStruct::LocalName(v) => v.encode(buffer),
            AdStruct::ManufacturerSpecificData(v) => v.encode(buffer),
            AdStruct::PeripheralConnectionIntervalRange(v) => v.encode(buffer),
            AdStruct::PublicTargetAddress(v) => v.encode(buffer),
            AdStruct::RandomTargetAddress(v) => v.encode(buffer),
            AdStruct::ServiceDataUuid16(v) => v.encode(buffer),
            AdStruct::ServiceDataUuid32(v) => v.encode(buffer),
            AdStruct::ServiceDataUuid128(v) => v.encode(buffer),
            AdStruct::ServiceSolicitationUuid16(v) => v.encode(buffer),
            AdStruct::ServiceSolicitationUuid32(v) => v.encode(buffer),
            AdStruct::ServiceSolicitationUuid128(v) => v.encode(buffer),
            AdStruct::ServiceUuid16(v) => v.encode(buffer),
            AdStruct::ServiceUuid32(v) => v.encode(buffer),
            AdStruct::ServiceUuid128(v) => v.encode(buffer),
            AdStruct::TxPowerLevel(v) => v.encode(buffer),
            AdStruct::Uri(v) => v.encode(buffer),
            AdStruct::Unhandled(_) => Err(bletio_utils::Error::CannotEncode),
        }
    }

    fn encoded_size(&self) -> usize {
        match self {
            AdStruct::AdvertisingInterval(v) => v.encoded_size(),
            AdStruct::Appearance(v) => v.encoded_size(),
            AdStruct::Flags(v) => v.encoded_size(),
            AdStruct::LeSupportedFeatures(v) => v.encoded_size(),
            AdStruct::LocalName(v) => v.encoded_size(),
            AdStruct::ManufacturerSpecificData(v) => v.encoded_size(),
            AdStruct::PeripheralConnectionIntervalRange(v) => v.encoded_size(),
            AdStruct::PublicTargetAddress(v) => v.encoded_size(),
            AdStruct::RandomTargetAddress(v) => v.encoded_size(),
            AdStruct::ServiceDataUuid16(v) => v.encoded_size(),
            AdStruct::ServiceDataUuid32(v) => v.encoded_size(),
            AdStruct::ServiceDataUuid128(v) => v.encoded_size(),
            AdStruct::ServiceSolicitationUuid16(v) => v.encoded_size(),
            AdStruct::ServiceSolicitationUuid32(v) => v.encoded_size(),
            AdStruct::ServiceSolicitationUuid128(v) => v.encoded_size(),
            AdStruct::ServiceUuid16(v) => v.encoded_size(),
            AdStruct::ServiceUuid32(v) => v.encoded_size(),
            AdStruct::ServiceUuid128(v) => v.encoded_size(),
            AdStruct::TxPowerLevel(v) => v.encoded_size(),
            AdStruct::Uri(v) => v.encoded_size(),
            AdStruct::Unhandled(_) => 0,
        }
    }
}

#[cfg(test)]
mod test {
    use bletio_hci::{
        AdvertisingInterval, PublicDeviceAddress, RandomStaticDeviceAddress, SupportedLeFeatures,
        TxPowerLevel,
    };
    use bletio_utils::{Buffer, BufferOps};
    use rstest::rstest;

    use crate::{
        advertising::{Flags, LocalNameComplete, ServiceListComplete, Uri},
        assigned_numbers::{AppearanceValue, CompanyIdentifier, ProvisionedUriScheme, ServiceUuid},
        uuid::{Uuid128, Uuid32},
    };

    use super::*;

    #[rstest]
    #[case(AdStruct::AdvertisingInterval(AdvertisingIntervalAdStruct::new(AdvertisingInterval::default())), &[0x03, 0x1A, 0x00, 0x08])]
    #[case(AdStruct::Appearance(AppearanceAdStruct::new(AppearanceValue::StandmountedSpeaker)), &[0x03, 0x19, 0x44, 0x08])]
    #[case(AdStruct::Flags(FlagsAdStruct::new(Flags::LE_GENERAL_DISCOVERABLE_MODE | Flags::BREDR_NOT_SUPPORTED)), &[0x02, 0x01, 0x06])]
    #[case(AdStruct::LeSupportedFeatures(LeSupportedFeaturesAdStruct::new(SupportedLeFeatures::default())), &[0x01, 0x27])]
    #[case(
        AdStruct::LocalName(LocalNameAdStruct::try_new("bletio", LocalNameComplete::Shortened(5)).unwrap()),
        &[0x06, 0x08, b'b', b'l', b'e', b't', b'i']
    )]
    #[case(
        AdStruct::ManufacturerSpecificData(
            ManufacturerSpecificDataAdStruct::try_new(CompanyIdentifier::AppleInc,
                &[0x12, 0x19, 0x00, 0x9A, 0x9A, 0xE9, 0x80, 0x96, 0x3C, 0xA0, 0x14, 0xFB, 0xE2, 0x14,
                    0x41, 0x88, 0xF5, 0xDA, 0xB6, 0x07, 0x99, 0xD3, 0x15, 0x57, 0x6C, 0x01, 0x00]
            ).unwrap()
        ),
        &[0x1E, 0xFF, 0x4C, 0x00, 0x12, 0x19, 0x00, 0x9A, 0x9A, 0xE9, 0x80, 0x96, 0x3C, 0xA0, 0x14, 0xFB, 0xE2,
            0x14, 0x41, 0x88, 0xF5, 0xDA, 0xB6, 0x07, 0x99, 0xD3, 0x15, 0x57, 0x6C, 0x01, 0x00]
    )]
    #[case(
        AdStruct::PeripheralConnectionIntervalRange(
            PeripheralConnectionIntervalRangeAdStruct::new(0x0006.try_into().unwrap()..=0x0C80.try_into().unwrap())
        ),
        &[0x05, 0x12, 0x06, 0x00, 0x80, 0x0C]
    )]
    #[case(
        AdStruct::PublicTargetAddress(
            PublicTargetAddressAdStruct::try_new(&[PublicDeviceAddress::new([0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56])]).unwrap()
        ),
        &[0x07, 0x17, 0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56]
    )]
    #[case(
        AdStruct::RandomTargetAddress(
            RandomTargetAddressAdStruct::try_new(&[RandomStaticDeviceAddress::try_new([0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0xF7]).unwrap().into()]).unwrap()
        ),
        &[0x07, 0x18, 0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0xF7]
    )]
    #[case(
        AdStruct::ServiceDataUuid16(
            ServiceDataUuid16AdStruct::try_new(ServiceUuid::LinkLoss, &[0x01, 0x14]).unwrap()
        ),
        &[0x05, 0x16, 0x03, 0x18, 0x01, 0x14]
    )]
    #[case(
        AdStruct::ServiceDataUuid32(
            ServiceDataUuid32AdStruct::try_new(Uuid32(0x0000_1803), &[0x09, 0x1F, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00]).unwrap()
        ),
        &[0x0F, 0x20, 0x03, 0x18, 0x00, 0x00, 0x09, 0x1F, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00]
    )]
    #[case(
        AdStruct::ServiceDataUuid128(
            ServiceDataUuid128AdStruct::try_new(Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640), &[0xD6, 0x0F, 0x28, 0x6E]).unwrap()
        ),
        &[0x15, 0x21, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5, 0xD6, 0x0F, 0x28, 0x6E]
    )]
    #[case(
        AdStruct::ServiceSolicitationUuid16(
            ServiceSolicitationUuid16AdStruct::try_new(&[ServiceUuid::LinkLoss, ServiceUuid::Battery]).unwrap()
        ),
        &[0x05, 0x14, 0x03, 0x18, 0x0F, 0x18]
    )]
    #[case(
        AdStruct::ServiceSolicitationUuid32(
            ServiceSolicitationUuid32AdStruct::try_new(&[Uuid32(0x0000_1803), Uuid32(0x0000_180F)]).unwrap()
        ),
        &[0x09, 0x1F, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00]
    )]
    #[case(
        AdStruct::ServiceSolicitationUuid128(
            ServiceSolicitationUuid128AdStruct::try_new(&[Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)]).unwrap()
        ),
        &[0x11, 0x15, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5]
    )]
    #[case(
        AdStruct::ServiceUuid16(ServiceUuid16AdStruct::try_new(
            &[ServiceUuid::LinkLoss, ServiceUuid::Battery, ServiceUuid::EnvironmentalSensing],
            ServiceListComplete::Complete
        ).unwrap()),
        &[0x07, 0x03, 0x03, 0x18, 0x0F, 0x18, 0x1A, 0x18]
    )]
    #[case(
        AdStruct::ServiceUuid32(ServiceUuid32AdStruct::try_new(
            &[Uuid32(0x0000_1803), Uuid32(0x0000_180F), Uuid32(0x0000_181A)],
            ServiceListComplete::Incomplete
        ).unwrap()),
        &[0x0D, 0x04, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00, 0x1A, 0x18, 0x00, 0x00]
    )]
    #[case(
        AdStruct::ServiceUuid128(ServiceUuid128AdStruct::try_new(
            &[Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)],
            ServiceListComplete::Incomplete
        ).unwrap()),
        &[0x11, 0x06, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5]
    )]
    #[case(AdStruct::TxPowerLevel(TxPowerLevelAdStruct::new(TxPowerLevel::try_new(20).unwrap())), &[0x02, 0x0A, 0x14])]
    #[case(
        AdStruct::Uri(UriAdStruct::new(Uri::try_new(ProvisionedUriScheme::Http, "//example.org/").unwrap())),
        &[0x11, 0x24, 0x16, 0x00, b'/', b'/', b'e', b'x', b'a', b'm', b'p', b'l', b'e', b'.', b'o', b'r', b'g', b'/']
    )]
    fn test_ad_struct_encode_success(#[case] ad_struct: AdStruct, #[case] encoded_data: &[u8]) {
        let mut buffer: Buffer<32> = Buffer::default();
        assert!(ad_struct.encode(&mut buffer).is_ok());
        assert_eq!(ad_struct.encoded_size(), encoded_data.len());
        assert_eq!(buffer.len(), encoded_data.len());
        assert_eq!(buffer.data(), encoded_data);
    }

    #[test]
    fn test_ad_struct_encode_failure() {
        let ad_struct = AdStruct::Unhandled(0x30);
        let mut buffer: Buffer<32> = Buffer::default();
        assert_eq!(
            ad_struct.encode(&mut buffer),
            Err(bletio_utils::Error::CannotEncode)
        );
        assert_eq!(ad_struct.encoded_size(), 0);
    }
}
