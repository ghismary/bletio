use core::ops::RangeInclusive;

use crate::advertising::ad_struct::{
    AdStruct, AdStructType, AD_STRUCT_DATA_OFFSET, AD_STRUCT_LENGTH_OFFSET, AD_STRUCT_TYPE_OFFSET,
};
use crate::assigned_numbers::AdType;
use crate::connection_interval_value::ConnectionIntervalValue;
use crate::utils::encode_le_u16;

const PERIPHERAL_CONNECTION_INTERVAL_RANGE_AD_STRUCT_SIZE: usize = 6;

/// Peripheral’s preferred connection interval range, for all logical connections.
///
/// For more information about this connection interval, see
/// [Core Specification 6.0, Vol.3, Part C, 12.3](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-7ef0bdcb-4c81-1aea-5f65-4a69eab5c899).
#[derive(Debug, Clone)]
pub struct PeripheralConnectionIntervalRangeAdStruct {
    buffer: [u8; PERIPHERAL_CONNECTION_INTERVAL_RANGE_AD_STRUCT_SIZE],
}

impl PeripheralConnectionIntervalRangeAdStruct {
    /// Create a Peripheral Connection Interval Range Advertising Structure.
    ///
    /// # Arguments
    ///
    /// * `range` — The connection interval range.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bletio::advertising::ad_struct::PeripheralConnectionIntervalRangeAdStruct;
    /// # fn main() -> Result<(), bletio::Error> {
    /// let ad_struct = PeripheralConnectionIntervalRangeAdStruct::new(
    ///     (0x0010.try_into()?..=0x0020.try_into()?)
    /// );
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ```
    /// # use bletio::ConnectionIntervalValue;
    /// # use bletio::advertising::ad_struct::PeripheralConnectionIntervalRangeAdStruct;
    /// # fn main() -> Result<(), bletio::Error> {
    /// let ad_struct = PeripheralConnectionIntervalRangeAdStruct::new(
    ///     (ConnectionIntervalValue::undefined()..=ConnectionIntervalValue::undefined())
    /// );
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ```
    /// # use bletio::ConnectionIntervalValue;
    /// # use bletio::advertising::ad_struct::PeripheralConnectionIntervalRangeAdStruct;
    /// # fn main() -> Result<(), bletio::Error> {
    /// let ad_struct = PeripheralConnectionIntervalRangeAdStruct::new(
    ///     (ConnectionIntervalValue::undefined()..=0x0030.try_into()?)
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(range: RangeInclusive<ConnectionIntervalValue>) -> Self {
        let mut s = Self {
            buffer: Default::default(),
        };
        s.buffer[AD_STRUCT_LENGTH_OFFSET] =
            (PERIPHERAL_CONNECTION_INTERVAL_RANGE_AD_STRUCT_SIZE - 1) as u8;
        s.buffer[AD_STRUCT_TYPE_OFFSET] = AdType::PeripheralConnectionIntervalRange as u8;
        let min = (*range.start()).into();
        let max = (*range.end()).into();
        // INVARIANT: The buffer space is known to be enough.
        encode_le_u16(&mut s.buffer[AD_STRUCT_DATA_OFFSET..], min).unwrap();
        encode_le_u16(&mut s.buffer[AD_STRUCT_DATA_OFFSET + 2..], max).unwrap();
        s
    }
}

impl AdStruct for PeripheralConnectionIntervalRangeAdStruct {
    fn encoded_data(&self) -> &[u8] {
        &self.buffer
    }
    fn r#type(&self) -> AdStructType {
        AdStructType::PERIPHERAL_CONNECTION_INTERVAL_RANGE
    }

    fn is_unique(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Error;

    #[test]
    fn test_peripheral_connection_interval_range_ad_struct() -> Result<(), Error> {
        let value =
            PeripheralConnectionIntervalRangeAdStruct::new(0x0006.try_into()?..=0x0C80.try_into()?);
        assert_eq!(value.encoded_data(), &[0x05, 0x12, 0x06, 0x00, 0x80, 0x0C]);
        assert!(value
            .r#type()
            .contains(AdStructType::PERIPHERAL_CONNECTION_INTERVAL_RANGE));
        assert!(!value.is_unique());

        let value = PeripheralConnectionIntervalRangeAdStruct::new(
            ConnectionIntervalValue::undefined()..=0x0C80.try_into()?,
        );
        assert_eq!(value.encoded_data(), &[0x05, 0x12, 0xFF, 0xFF, 0x80, 0x0C]);
        assert!(value
            .r#type()
            .contains(AdStructType::PERIPHERAL_CONNECTION_INTERVAL_RANGE));
        assert!(!value.is_unique());

        let value = PeripheralConnectionIntervalRangeAdStruct::new(
            0x0006.try_into()?..=ConnectionIntervalValue::undefined(),
        );
        assert_eq!(value.encoded_data(), &[0x05, 0x12, 0x06, 0x00, 0xFF, 0xFF]);
        assert!(value
            .r#type()
            .contains(AdStructType::PERIPHERAL_CONNECTION_INTERVAL_RANGE));
        assert!(!value.is_unique());

        let value = PeripheralConnectionIntervalRangeAdStruct::new(
            ConnectionIntervalValue::undefined()..=ConnectionIntervalValue::undefined(),
        );
        assert_eq!(value.encoded_data(), &[0x05, 0x12, 0xFF, 0xFF, 0xFF, 0xFF]);
        assert!(value
            .r#type()
            .contains(AdStructType::PERIPHERAL_CONNECTION_INTERVAL_RANGE));
        assert!(!value.is_unique());

        let value =
            PeripheralConnectionIntervalRangeAdStruct::new(0x0010.try_into()?..=0x0010.try_into()?);
        assert_eq!(value.encoded_data(), &[0x05, 0x12, 0x10, 0x00, 0x10, 0x00]);
        assert!(value
            .r#type()
            .contains(AdStructType::PERIPHERAL_CONNECTION_INTERVAL_RANGE));
        assert!(!value.is_unique());

        Ok(())
    }
}
