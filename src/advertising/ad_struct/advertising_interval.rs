use crate::advertising::ad_struct::{AdStruct, AdStructBuffer, AdStructType};

use crate::advertising::AdvertisingIntervalValue;
use crate::assigned_numbers::AdType;

const ADVERTISING_INTERVAL_AD_STRUCT_SIZE: usize = 4;

/// The advertising interval.
///
/// The advertising interval is defined in
/// [Supplement to the Bluetooth Core Specification, Part A, 1.15](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-30c6e42e-5327-f52c-d79e-1b174095c712),
/// and
/// [Core Specification 6.0, Vol. 6, Part B, 4.4.2.2](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/low-energy-controller/link-layer-specification.html#UUID-f6cd1541-800c-c516-b32b-95dd0479840b).
#[derive(Debug, Clone)]
pub struct AdvertisingIntervalAdStruct {
    buffer: AdStructBuffer<ADVERTISING_INTERVAL_AD_STRUCT_SIZE>,
}

impl AdvertisingIntervalAdStruct {
    /// Create an Advertising Interval Advertising Structure.
    ///
    /// # Arguments
    ///
    /// * `interval` — The advertising interval to notify.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bletio::advertising::ad_struct::AdvertisingIntervalAdStruct;
    /// # use bletio::advertising::AdvertisingIntervalValue;
    /// let ad_struct = AdvertisingIntervalAdStruct::new(AdvertisingIntervalValue::default());
    /// ```
    pub fn new(interval: AdvertisingIntervalValue) -> Self {
        // TODO: Handle extended/primary advertising interval values and periodic interval values.
        let mut s = Self {
            buffer: AdStructBuffer::new(AdType::AdvertisingInterval),
        };
        // INVARIANT: The buffer space is known to be enough.
        s.buffer.encode_le_u16(interval.value()).unwrap();
        s
    }
}

impl AdStruct for AdvertisingIntervalAdStruct {
    fn encoded_data(&self) -> &[u8] {
        self.buffer.data()
    }
    fn r#type(&self) -> AdStructType {
        AdStructType::ADVERTISING_INTERVAL
    }

    fn is_unique(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Error;

    #[test]
    fn test_appearance_ad_struct() -> Result<(), Error> {
        let value = AdvertisingIntervalAdStruct::new(AdvertisingIntervalValue::default());
        assert_eq!(value.encoded_data(), &[0x03, 0x1A, 0x00, 0x08]);
        assert!(value.r#type().contains(AdStructType::ADVERTISING_INTERVAL));
        assert!(value.is_unique());

        let value = AdvertisingIntervalAdStruct::new(AdvertisingIntervalValue::try_new(0x0020)?);
        assert_eq!(value.encoded_data(), &[0x03, 0x1A, 0x20, 0x00]);
        assert!(value.r#type().contains(AdStructType::ADVERTISING_INTERVAL));
        assert!(value.is_unique());

        let value = AdvertisingIntervalAdStruct::new(AdvertisingIntervalValue::try_new(0x4000)?);
        assert_eq!(value.encoded_data(), &[0x03, 0x1A, 0x00, 0x40]);
        assert!(value.r#type().contains(AdStructType::ADVERTISING_INTERVAL));
        assert!(value.is_unique());

        Ok(())
    }
}
