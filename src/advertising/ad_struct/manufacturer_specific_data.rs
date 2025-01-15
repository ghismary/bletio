use crate::advertising::ad_struct::{
    AdStruct, AdStructType, AD_STRUCT_DATA_OFFSET, AD_STRUCT_LENGTH_OFFSET, AD_STRUCT_TYPE_OFFSET,
};
use crate::advertising::advertising_data::ADVERTISING_DATA_MAX_SIZE;
use crate::advertising::AdvertisingError;
use crate::assigned_numbers::AdType;
use crate::assigned_numbers::CompanyIdentifier;
use crate::utils::encode_le_u16;
use crate::Error;

/// Manufacturer specific data.
///
/// It contains the [`CompanyIdentifier`] and any additional data octets for which the
/// interpretation shall be defined by the manufacturer specified by the company
/// identifier, as defined in [Supplement to the Bluetooth Core Specification, Part A, 1.4](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-39e09443-1451-0664-140d-65060c9f6783).
///
/// This is used for example for iBeacons and Eddystone beacons.
#[derive(Debug, Clone)]
pub struct ManufacturerSpecificDataAdStruct {
    buffer: [u8; ADVERTISING_DATA_MAX_SIZE],
    offset: usize,
}

impl ManufacturerSpecificDataAdStruct {
    /// Create a Manufacturer Specific Data Advertising Structure.
    ///
    /// # Arguments
    ///
    /// * `manufacturer` — The [`CompanyIdentifier`] of the manufacturer.
    /// * `data` — The additional data octets.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bletio::advertising::ad_struct::ManufacturerSpecificDataAdStruct;
    /// # use bletio::assigned_numbers::CompanyIdentifier;
    /// # fn main() -> Result<(), bletio::Error> {
    /// let ad_struct = ManufacturerSpecificDataAdStruct::try_new(
    ///     CompanyIdentifier::AppleInc,
    ///     &[0x12, 0x19, 0x00, 0x9a, 0x9a, 0xe9, 0x80, 0x96, 0x3c, 0xa0, 0x14, 0xfb, 0xe2, 0x14,
    ///         0x41, 0x88, 0xf5, 0xda, 0xb6, 0x07, 0x99, 0xd3, 0x15, 0x57, 0x6c, 0x01, 0x00]
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_new(manufacturer: CompanyIdentifier, data: &[u8]) -> Result<Self, Error> {
        let data_size = data.len();
        if (4 + data_size) > ADVERTISING_DATA_MAX_SIZE {
            return Err(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?;
        }
        let mut s = Self {
            buffer: Default::default(),
            offset: AD_STRUCT_DATA_OFFSET,
        };
        s.buffer[AD_STRUCT_LENGTH_OFFSET] = 3 + data_size as u8;
        s.buffer[AD_STRUCT_TYPE_OFFSET] = AdType::ManufacturerSpecificData as u8;
        // INVARIANT: The buffer space is known to be enough.
        encode_le_u16(&mut s.buffer[s.offset..], manufacturer as u16).unwrap();
        s.offset += 2;
        s.buffer[s.offset..s.offset + data_size].copy_from_slice(data);
        s.offset += data_size;
        Ok(s)
    }
}

impl AdStruct for ManufacturerSpecificDataAdStruct {
    fn encoded_data(&self) -> &[u8] {
        &self.buffer[..self.offset]
    }
    fn r#type(&self) -> AdStructType {
        AdStructType::MANUFACTURER_SPECIFIC_DATA
    }

    fn is_unique(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_manufacturer_specific_data_ad_struct_creation_success() -> Result<(), Error> {
        let value = ManufacturerSpecificDataAdStruct::try_new(
            CompanyIdentifier::AppleInc,
            &[
                0x12, 0x19, 0x00, 0x9A, 0x9A, 0xE9, 0x80, 0x96, 0x3C, 0xA0, 0x14, 0xFB, 0xE2, 0x14,
                0x41, 0x88, 0xF5, 0xDA, 0xB6, 0x07, 0x99, 0xD3, 0x15, 0x57, 0x6C, 0x01, 0x00,
            ],
        )?;
        assert_eq!(
            value.encoded_data(),
            &[
                0x1E, 0xFF, 0x4C, 0x00, 0x12, 0x19, 0x00, 0x9A, 0x9A, 0xE9, 0x80, 0x96, 0x3C, 0xA0,
                0x14, 0xFB, 0xE2, 0x14, 0x41, 0x88, 0xF5, 0xDA, 0xB6, 0x07, 0x99, 0xD3, 0x15, 0x57,
                0x6C, 0x01, 0x00
            ]
        );
        assert!(value
            .r#type()
            .contains(AdStructType::MANUFACTURER_SPECIFIC_DATA));
        assert!(!value.is_unique());

        let value = ManufacturerSpecificDataAdStruct::try_new(CompanyIdentifier::Inventel, &[])?;
        assert_eq!(value.encoded_data(), &[0x03, 0xFF, 0x1E, 0x00]);
        assert!(value
            .r#type()
            .contains(AdStructType::MANUFACTURER_SPECIFIC_DATA));
        assert!(!value.is_unique());

        Ok(())
    }

    #[test]
    fn test_manufacturer_specific_data_ad_struct_creation_failure() {
        let err = ManufacturerSpecificDataAdStruct::try_new(
            CompanyIdentifier::Withings,
            &[
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B,
                0x1C, 0x1D, 0x1E, 0x1F,
            ],
        )
        .expect_err("Manufacturer specific data too big");
        assert!(matches!(
            err,
            Error::Advertising(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
        ));
    }
}
