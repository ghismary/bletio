use bletio_utils::EncodeToBuffer;
use heapless::Vec;

use crate::advertising::AdvertisingError;
use crate::assigned_numbers::AdType;
use crate::assigned_numbers::CompanyIdentifier;

const MANUFACTURER_SPECIFIC_DATA_MAX_LENGTH: usize = 27;

/// Manufacturer specific data.
///
/// It contains the [`CompanyIdentifier`] and any additional data octets for which the
/// interpretation shall be defined by the manufacturer specified by the company
/// identifier, as defined in [Supplement to the Bluetooth Core Specification, Part A, 1.4](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-39e09443-1451-0664-140d-65060c9f6783).
///
/// This is used for example for iBeacons and Eddystone beacons.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ManufacturerSpecificDataAdStruct {
    manufacturer: CompanyIdentifier,
    data: Vec<u8, MANUFACTURER_SPECIFIC_DATA_MAX_LENGTH>,
}

impl ManufacturerSpecificDataAdStruct {
    pub(crate) fn try_new(
        manufacturer: CompanyIdentifier,
        data: &[u8],
    ) -> Result<Self, AdvertisingError> {
        Ok(Self {
            manufacturer,
            data: data
                .try_into()
                .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?,
        })
    }

    pub fn manufacturer(&self) -> CompanyIdentifier {
        self.manufacturer
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }
}

impl EncodeToBuffer for ManufacturerSpecificDataAdStruct {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push((self.encoded_size() - 1) as u8)?;
        buffer.try_push(AdType::ManufacturerSpecificData as u8)?;
        buffer.encode_le_u16(self.manufacturer as u16)?;
        buffer.copy_from_slice(self.data.as_slice())?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        4 + self.data.len()
    }
}

pub(crate) mod parser {
    use nom::{
        bytes::take,
        combinator::{fail, map_res},
        number::complete::le_u16,
        IResult, Parser,
    };

    use crate::advertising::ad_struct::AdStruct;

    use super::*;

    fn company_identifier(input: &[u8]) -> IResult<&[u8], CompanyIdentifier> {
        map_res(le_u16, TryFrom::try_from).parse(input)
    }

    pub(crate) fn manufacturer_specific_data_ad_struct(input: &[u8]) -> IResult<&[u8], AdStruct> {
        let (rest, manufacturer) = company_identifier.parse(input)?;
        let len = rest.len();
        if len > MANUFACTURER_SPECIFIC_DATA_MAX_LENGTH {
            fail::<_, &[u8], _>().parse(input)?;
        }
        let mut ad_struct = ManufacturerSpecificDataAdStruct {
            manufacturer,
            data: Default::default(),
        };
        map_res(take(len), |data| ad_struct.data.extend_from_slice(data)).parse(rest)?;
        Ok((&[], AdStruct::ManufacturerSpecificData(ad_struct)))
    }
}

#[cfg(test)]
mod test {
    use bletio_utils::{Buffer, BufferOps};
    use rstest::rstest;

    use crate::advertising::ad_struct::AdStruct;

    use super::{parser::*, *};

    #[rstest]
    #[case(
        CompanyIdentifier::AppleInc,
        &[
            0x12, 0x19, 0x00, 0x9A, 0x9A, 0xE9, 0x80, 0x96, 0x3C, 0xA0, 0x14, 0xFB, 0xE2, 0x14,
            0x41, 0x88, 0xF5, 0xDA, 0xB6, 0x07, 0x99, 0xD3, 0x15, 0x57, 0x6C, 0x01, 0x00],
        &[
            0x1E, 0xFF, 0x4C, 0x00, 0x12, 0x19, 0x00, 0x9A, 0x9A, 0xE9, 0x80, 0x96, 0x3C, 0xA0,
            0x14, 0xFB, 0xE2, 0x14, 0x41, 0x88, 0xF5, 0xDA, 0xB6, 0x07, 0x99, 0xD3, 0x15, 0x57,
            0x6C, 0x01, 0x00
        ]
    )]
    #[case(CompanyIdentifier::Inventel, &[], &[0x03, 0xFF, 0x1E, 0x00])]
    fn test_manufacturer_specific_data_ad_struct_success(
        #[case] manufacturer: CompanyIdentifier,
        #[case] data: &[u8],
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<31>::default();
        let ad_struct = ManufacturerSpecificDataAdStruct::try_new(manufacturer, data).unwrap();
        ad_struct.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        assert_eq!(ad_struct.manufacturer(), manufacturer);
        assert_eq!(ad_struct.data(), data);
        Ok(())
    }

    #[test]
    fn test_manufacturer_specific_data_ad_struct_failure() {
        let err = ManufacturerSpecificDataAdStruct::try_new(
            CompanyIdentifier::Withings,
            &[
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B,
                0x1C, 0x1D, 0x1E, 0x1F,
            ],
        );
        assert_eq!(
            err,
            Err(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
        );
    }

    #[test]
    fn test_manufacturer_specific_data_ad_struct_parsing_success() {
        assert_eq!(
            manufacturer_specific_data_ad_struct(&[
                0x4C, 0x00, 0x12, 0x19, 0x00, 0x9A, 0x9A, 0xE9, 0x80, 0x96, 0x3C, 0xA0, 0x14, 0xFB,
                0xE2, 0x14, 0x41, 0x88, 0xF5, 0xDA, 0xB6, 0x07, 0x99, 0xD3, 0x15, 0x57, 0x6C, 0x01,
                0x00
            ]),
            Ok((
                &[] as &[u8],
                AdStruct::ManufacturerSpecificData(
                    ManufacturerSpecificDataAdStruct::try_new(
                        CompanyIdentifier::AppleInc,
                        &[
                            0x12, 0x19, 0x00, 0x9A, 0x9A, 0xE9, 0x80, 0x96, 0x3C, 0xA0, 0x14, 0xFB,
                            0xE2, 0x14, 0x41, 0x88, 0xF5, 0xDA, 0xB6, 0x07, 0x99, 0xD3, 0x15, 0x57,
                            0x6C, 0x01, 0x00
                        ]
                    )
                    .unwrap()
                )
            ))
        );
    }

    #[test]
    fn test_manufacturer_specific_data_ad_struct_parsing_failure() {
        assert!(manufacturer_specific_data_ad_struct(&[
            0xFF, 0x03, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B,
            0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19,
            0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
        ])
        .is_err());
    }
}
