use crate::advertising::AdvertisingError;
use crate::assigned_numbers::AdType;
use bletio_hci::{ConnectionInterval, ConnectionIntervalRange};
use bletio_utils::{BufferOps, EncodeToBuffer};
use core::ops::RangeInclusive;

const PERIPHERAL_CONNECTION_INTERVAL_RANGE_AD_STRUCT_SIZE: usize = 5;

/// Peripheral’s preferred connection interval range, for all logical connections.
///
/// For more information about this connection interval, see
/// [Core Specification 6.0, Vol.3, Part C, 12.3](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-7ef0bdcb-4c81-1aea-5f65-4a69eab5c899).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PeripheralConnectionIntervalRangeAdStruct {
    range: PeripheralConnectionIntervalRange,
}

impl PeripheralConnectionIntervalRangeAdStruct {
    pub(crate) const fn new(range: PeripheralConnectionIntervalRange) -> Self {
        Self { range }
    }

    pub fn value(&self) -> &PeripheralConnectionIntervalRange {
        &self.range
    }
}

impl EncodeToBuffer for PeripheralConnectionIntervalRangeAdStruct {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, bletio_utils::Error> {
        buffer.try_push(PERIPHERAL_CONNECTION_INTERVAL_RANGE_AD_STRUCT_SIZE as u8)?;
        buffer.try_push(AdType::PeripheralConnectionIntervalRange as u8)?;
        self.range.encode(buffer)?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        PERIPHERAL_CONNECTION_INTERVAL_RANGE_AD_STRUCT_SIZE + 1
    }
}

/// Value to be used in a peripheral connection interval range.
///
/// Here are the characteristics of this peripheral connection interval value:
///  - Range: 0x0006 to 0x0C80 if specified
///  - Can be unspecified
///  - Time = N × 1.25 ms
///  - Time Range: 7.5 ms to 4 s
#[derive(Debug, Copy, Clone, Default, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeripheralConnectionInterval {
    Defined(ConnectionInterval),
    #[default]
    Undefined,
}

impl PeripheralConnectionInterval {
    /// Create a peripheral connection interval value.
    pub const fn try_new(value: u16) -> Result<Self, AdvertisingError> {
        match value {
            0xFFFF => Ok(Self::Undefined),
            _ => match ConnectionInterval::try_new(value) {
                Ok(value) => Ok(Self::Defined(value)),
                Err(_) => Err(AdvertisingError::InvalidPeripheralConnectionIntervalValue(
                    value,
                )),
            },
        }
    }

    /// Get the value of the connection interval value in milliseconds.
    pub const fn milliseconds(&self) -> Option<f32> {
        match self {
            Self::Defined(value) => Some(value.milliseconds()),
            Self::Undefined => None,
        }
    }
}

impl PartialEq for PeripheralConnectionInterval {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                PeripheralConnectionInterval::Defined(v1),
                PeripheralConnectionInterval::Defined(v2),
            ) => v1.eq(v2),
            _ => false,
        }
    }
}

impl PartialOrd for PeripheralConnectionInterval {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match (self, other) {
            (
                PeripheralConnectionInterval::Defined(v1),
                PeripheralConnectionInterval::Defined(v2),
            ) => v1.partial_cmp(v2),
            _ => Some(core::cmp::Ordering::Less),
        }
    }
}

impl TryFrom<u16> for PeripheralConnectionInterval {
    type Error = AdvertisingError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl From<PeripheralConnectionInterval> for u16 {
    fn from(value: PeripheralConnectionInterval) -> Self {
        match value {
            PeripheralConnectionInterval::Undefined => 0xFFFF,
            PeripheralConnectionInterval::Defined(value) => value.into(),
        }
    }
}

impl EncodeToBuffer for PeripheralConnectionInterval {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, bletio_utils::Error> {
        buffer.encode_le_u16((*self).into())?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        size_of::<u16>()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PeripheralConnectionIntervalRange {
    value: RangeInclusive<PeripheralConnectionInterval>,
}

impl PeripheralConnectionIntervalRange {
    pub const fn try_new(min: u16, max: u16) -> Result<Self, AdvertisingError> {
        if (min == 0xFFFF) || (max == 0xFFFF) || (min <= max) {
            let min = match PeripheralConnectionInterval::try_new(min) {
                Ok(min) => min,
                Err(e) => return Err(e),
            };
            let max = match PeripheralConnectionInterval::try_new(max) {
                Ok(max) => max,
                Err(e) => return Err(e),
            };
            Ok(Self { value: min..=max })
        } else {
            Err(AdvertisingError::InvalidPeripheralConnectionIntervalRange)
        }
    }

    pub const fn min(&self) -> PeripheralConnectionInterval {
        *self.value.start()
    }

    pub const fn max(&self) -> PeripheralConnectionInterval {
        *self.value.end()
    }

    /// Create an undefined connection interval range.
    pub const fn undefined() -> Self {
        Self {
            value: PeripheralConnectionInterval::Undefined
                ..=PeripheralConnectionInterval::Undefined,
        }
    }
}

impl Default for PeripheralConnectionIntervalRange {
    fn default() -> Self {
        Self {
            value: Default::default()..=Default::default(),
        }
    }
}

impl From<ConnectionIntervalRange> for PeripheralConnectionIntervalRange {
    fn from(value: ConnectionIntervalRange) -> Self {
        Self {
            value: PeripheralConnectionInterval::Defined(value.min())
                ..=PeripheralConnectionInterval::Defined(value.max()),
        }
    }
}

impl EncodeToBuffer for PeripheralConnectionIntervalRange {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, bletio_utils::Error> {
        self.value.start().encode(buffer)?;
        self.value.end().encode(buffer)?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        self.value.start().encoded_size() + self.value.end().encoded_size()
    }
}

/// Create a `PeripheralConnectionIntervalRange`, checking that it is valid at compile-time.
///
/// # Examples
///
/// ```
/// # use bletio_host::advertising::peripheral_connection_interval_range;
/// let range = peripheral_connection_interval_range!(0x0020, 0x0030);
/// ```
///
/// ```
/// # use bletio_host::advertising::peripheral_connection_interval_range;
/// let range = peripheral_connection_interval_range!(0x0020,);
/// ```
///
/// ```
/// # use bletio_host::advertising::peripheral_connection_interval_range;
/// let range = peripheral_connection_interval_range!(, 0x0040);
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __peripheral_connection_interval_range__ {
    ($min:expr, $max:expr) => {{
        const {
            match $crate::advertising::PeripheralConnectionIntervalRange::try_new($min, $max) {
                Ok(v) => v,
                Err($crate::advertising::AdvertisingError::InvalidPeripheralConnectionIntervalRange) => panic!("the peripheral connection interval range minimum value must be smaller or equal to the maximum value"),
                Err(_) => panic!("the connection interval value is invalid, it needs to be between 0x0006 and 0x0C80")
            }
        }
    }};
    ($min:expr,) => {{
        const {
            match $crate::advertising::PeripheralConnectionIntervalRange::try_new($min, 0xFFFF) {
                Ok(v) => v,
                Err(_) => panic!("the connection interval value is invalid, it needs to be between 0x0006 and 0x0C80")
            }
        }
    }};
    (, $max:expr) => {{
        const {
            match $crate::advertising::PeripheralConnectionIntervalRange::try_new(0xFFFF, $max) {
                Ok(v) => v,
                Err(_) => panic!("the connection interval value is invalid, it needs to be between 0x0006 and 0x0C80")
            }
        }
    }};
    () => {{
       $crate::advertising::PeripheralConnectionIntervalRange::undefined()
    }};
}

#[doc(inline)]
pub use __peripheral_connection_interval_range__ as peripheral_connection_interval_range;

pub(crate) mod parser {
    use super::*;
    use crate::advertising::ad_struct::AdStruct;
    use nom::{
        combinator::{map, map_res},
        number::le_u16,
        IResult, Parser,
    };

    fn peripheral_connection_interval_range(
        input: &[u8],
    ) -> IResult<&[u8], PeripheralConnectionIntervalRange> {
        map_res((le_u16(), le_u16()), |(start, end)| {
            PeripheralConnectionIntervalRange::try_new(start, end)
        })
        .parse(input)
    }

    pub(crate) fn peripheral_connection_interval_range_ad_struct(
        input: &[u8],
    ) -> IResult<&[u8], AdStruct> {
        map(peripheral_connection_interval_range, |range| {
            AdStruct::PeripheralConnectionIntervalRange(
                PeripheralConnectionIntervalRangeAdStruct::new(range),
            )
        })
        .parse(input)
    }
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;
    use bletio_hci::{connection_interval_range, Error};
    use bletio_utils::{Buffer, BufferOps};
    use claims::{assert_ge, assert_le};
    use rstest::rstest;

    use super::{parser::*, *};
    use crate::advertising::ad_struct::AdStruct;

    #[rstest]
    #[case(0x0006.try_into().unwrap(), 0x0006, Some(7.5f32))]
    #[case(PeripheralConnectionInterval::try_new(0x0C80).unwrap(), 0x0C80, Some(4000f32))]
    #[case(PeripheralConnectionInterval::try_new(0xFFFF).unwrap(), 0xFFFF, None)]
    #[case(PeripheralConnectionInterval::Undefined, 0xFFFF, None)]
    #[case(PeripheralConnectionInterval::default(), 0xFFFF, None)]
    fn test_peripheral_connection_interval_success(
        #[case] interval: PeripheralConnectionInterval,
        #[case] expected_value: u16,
        #[case] expected_milliseconds: Option<f32>,
    ) {
        let value: u16 = interval.into();
        assert_eq!(value, expected_value);
        match expected_milliseconds {
            Some(expected) => {
                assert_relative_eq!(interval.milliseconds().unwrap(), expected, epsilon = 1.0e-6)
            }
            None => assert!(interval.milliseconds().is_none()),
        }
    }

    #[rstest]
    #[case(0x0000)]
    #[case(0x0005)]
    #[case(0x0C81)]
    #[case(0xFFFE)]
    fn test_peripheral_connection_interval_failure(#[case] input: u16) {
        let result = PeripheralConnectionInterval::try_new(input);
        assert_eq!(
            result,
            Err(AdvertisingError::InvalidPeripheralConnectionIntervalValue(
                input
            ))
        );
    }

    #[test]
    fn test_peripheral_connection_interval_comparison() -> Result<(), AdvertisingError> {
        let value1: PeripheralConnectionInterval = 0x0020.try_into()?;
        let value2 = PeripheralConnectionInterval::try_new(0x0020)?;
        assert_eq!(value1, value2);

        let value1 = PeripheralConnectionInterval::Undefined;
        let value2 = PeripheralConnectionInterval::Undefined;
        assert_ne!(value1, value2);
        assert_le!(value1, value2);
        assert_le!(value2, value1);

        let value1: PeripheralConnectionInterval = 0x0006.try_into()?;
        let value2 = PeripheralConnectionInterval::try_new(0x0C80)?;
        assert_le!(value1, value2);
        assert_ge!(value2, value1);

        let value1 = PeripheralConnectionInterval::Undefined;
        let value2 = PeripheralConnectionInterval::try_new(0x0100)?;
        assert_le!(value1, value2);
        assert_le!(value2, value1);

        Ok(())
    }

    #[test]
    fn test_peripheral_connection_interval_range_default() {
        let range = PeripheralConnectionIntervalRange::default();
        assert!(matches!(
            range.min(),
            PeripheralConnectionInterval::Undefined
        ));
        assert!(matches!(
            range.max(),
            PeripheralConnectionInterval::Undefined
        ));
    }

    #[rstest]
    #[case(0x0020, 0x0020)]
    #[case(0x0020, 0x0030)]
    #[case(0xFFFF, 0x0020)]
    #[case(0x0030, 0xFFFF)]
    #[case(0xFFFF, 0xFFFF)]
    fn test_peripheral_connection_interval_range_success(
        #[case] min: u16,
        #[case] max: u16,
    ) -> Result<(), Error> {
        let range = PeripheralConnectionIntervalRange::try_new(min, max).unwrap();
        let range_min: u16 = range.min().into();
        let range_max: u16 = range.max().into();
        assert_eq!(range_min, min);
        assert_eq!(range_max, max);
        Ok(())
    }

    #[rstest]
    #[case(
        0x0005,
        0x0020,
        AdvertisingError::InvalidPeripheralConnectionIntervalValue(0x0005)
    )]
    #[case(
        0x0030,
        0x0C81,
        AdvertisingError::InvalidPeripheralConnectionIntervalValue(0x0C81)
    )]
    #[case(
        0x0030,
        0x0020,
        AdvertisingError::InvalidPeripheralConnectionIntervalRange
    )]
    #[case(
        0x0C80,
        0x0C7F,
        AdvertisingError::InvalidPeripheralConnectionIntervalRange
    )]
    fn test_peripheral_connection_interval_range_failure(
        #[case] min: u16,
        #[case] max: u16,
        #[case] error: AdvertisingError,
    ) {
        let err = PeripheralConnectionIntervalRange::try_new(min, max);
        assert_eq!(err, Err(error));
    }

    #[rstest]
    #[case(connection_interval_range!(0x0020, 0x0020), peripheral_connection_interval_range!(0x0020, 0x0020))]
    #[case(connection_interval_range!(0x0020, 0x0030), peripheral_connection_interval_range!(0x0020, 0x0030))]
    fn test_peripheral_connection_interval_range_from_connection_interval_range(
        #[case] range: ConnectionIntervalRange,
        #[case] expected_peripheral_range: PeripheralConnectionIntervalRange,
    ) {
        let range: PeripheralConnectionIntervalRange = range.into();
        assert_eq!(range, expected_peripheral_range);
    }

    #[rstest]
    #[case(
        peripheral_connection_interval_range!(0x0006, 0x0C80),
        Some(peripheral_connection_interval_range!(0x0006, 0x0C80)),
        &[0x05, 0x12, 0x06, 0x00, 0x80, 0x0C]
    )]
    #[case(
        peripheral_connection_interval_range!(, 0x0C80),
        None,
        &[0x05, 0x12, 0xFF, 0xFF, 0x80, 0x0C]
    )]
    #[case(
        peripheral_connection_interval_range!(0x0006,),
        None,
        &[0x05, 0x12, 0x06, 0x00, 0xFF, 0xFF]
    )]
    #[case(PeripheralConnectionIntervalRange::undefined(), None, &[0x05, 0x12, 0xFF, 0xFF, 0xFF, 0xFF])]
    #[case(
        peripheral_connection_interval_range!(0x0010, 0x0010),
        Some(peripheral_connection_interval_range!(0x0010, 0x0010)),
        &[0x05, 0x12, 0x10, 0x00, 0x10, 0x00]
    )]
    fn test_peripheral_connection_interval_range_ad_struct(
        #[case] range: PeripheralConnectionIntervalRange,
        #[case] expected_range: Option<PeripheralConnectionIntervalRange>,
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<6>::default();
        let ad_struct = PeripheralConnectionIntervalRangeAdStruct::new(range.clone());
        ad_struct.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        if let Some(range) = expected_range {
            assert_eq!(*ad_struct.value(), range);
        }
        Ok(())
    }

    #[rstest]
    #[case(&[0x06, 0x00, 0x80, 0x0C], peripheral_connection_interval_range!(0x0006, 0x0C80))]
    #[case(&[0x10, 0x00, 0x10, 0x00], peripheral_connection_interval_range!(0x0010, 0x0010))]
    fn test_peripheral_connection_interval_range_ad_struct_parsing_success(
        #[case] input: &[u8],
        #[case] range: PeripheralConnectionIntervalRange,
    ) {
        assert_eq!(
            peripheral_connection_interval_range_ad_struct(input),
            Ok((
                &[] as &[u8],
                AdStruct::PeripheralConnectionIntervalRange(
                    PeripheralConnectionIntervalRangeAdStruct::new(range)
                )
            ))
        );
    }

    #[rstest]
    #[case(&[0xFF, 0xFF, 0x80, 0x0C])]
    #[case(&[0x06, 0x00, 0xFF, 0xFF])]
    #[case(&[0xFF, 0xFF, 0xFF, 0xFF])]
    fn test_peripheral_connection_interval_range_ad_struct_parsing_undefined_success(
        #[case] input: &[u8],
    ) {
        assert!(peripheral_connection_interval_range_ad_struct(input).is_ok());
    }

    #[rstest]
    #[case(&[0x80, 0x0C, 0x06, 0x00])]
    #[case(&[0x06, 0x00, 0xFE, 0xFF])]
    fn test_peripheral_connection_interval_range_ad_struct_parsing_failure(#[case] input: &[u8]) {
        assert!(peripheral_connection_interval_range_ad_struct(input).is_err())
    }
}
