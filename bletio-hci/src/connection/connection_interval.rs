use core::ops::RangeInclusive;

use bletio_utils::Error as UtilsError;
use bletio_utils::{BufferOps, EncodeToBuffer};

use crate::Error;

/// Value to be used in a connection interval range.
///
/// Here are the characteristics of this connection interval value:
///  - Range: 0x0006 to 0x0C80
///  - Time = N × 1.25 ms
///  - Time Range: 7.5 ms to 4 s
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ConnectionInterval {
    value: u16,
}

impl ConnectionInterval {
    /// Create a connection interval value.
    pub const fn try_new(value: u16) -> Result<Self, Error> {
        if (value >= 0x0006) && (value <= 0x0C80) {
            Ok(Self { value })
        } else {
            Err(Error::InvalidConnectionIntervalValue(value))
        }
    }

    /// Get the value of the connection interval value in milliseconds.
    pub const fn milliseconds(&self) -> f32 {
        self.value as f32 * 1.25
    }

    pub const fn value(&self) -> u16 {
        self.value
    }
}

impl Default for ConnectionInterval {
    fn default() -> Self {
        Self { value: 64 }
    }
}

impl TryFrom<u16> for ConnectionInterval {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl From<ConnectionInterval> for u16 {
    fn from(value: ConnectionInterval) -> Self {
        value.value
    }
}

impl EncodeToBuffer for ConnectionInterval {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, UtilsError> {
        buffer.encode_le_u16((*self).into())?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        size_of::<u16>()
    }
}

/// Create an [`ConnectionInterval`], checking that it is valid at compile-time.
///
/// # Examples
///
/// ```
/// # use bletio_hci::connection_interval;
/// let range = connection_interval!(0x0020);
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __connection_interval__ {
    ($value:expr) => {{
        const {
            match $crate::ConnectionInterval::try_new($value) {
                Ok(v) => v,
                Err(_) => panic!("the connection interval value is invalid, it needs to be between 0x0006 and 0x0C80")
            }
        }
    }};
}

#[doc(inline)]
pub use __connection_interval__ as connection_interval;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ConnectionIntervalRange {
    value: RangeInclusive<ConnectionInterval>,
}

impl ConnectionIntervalRange {
    pub const fn try_new(min: u16, max: u16) -> Result<Self, Error> {
        if min <= max {
            let min = match ConnectionInterval::try_new(min) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };
            let max = match ConnectionInterval::try_new(max) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };
            Ok(Self { value: min..=max })
        } else {
            Err(Error::InvalidConnectionIntervalRange)
        }
    }

    pub const fn min(&self) -> ConnectionInterval {
        *self.value.start()
    }

    pub const fn max(&self) -> ConnectionInterval {
        *self.value.end()
    }
}

impl Default for ConnectionIntervalRange {
    fn default() -> Self {
        Self {
            value: Default::default()..=Default::default(),
        }
    }
}

impl EncodeToBuffer for ConnectionIntervalRange {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, UtilsError> {
        self.value.start().encode(buffer)?;
        self.value.end().encode(buffer)?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        self.value.start().encoded_size() + self.value.end().encoded_size()
    }
}

/// Create an [`ConnectionIntervalRange`], checking that it is valid at compile-time.
///
/// # Examples
///
/// ```
/// # use bletio_hci::connection_interval_range;
/// let range = connection_interval_range!(0x0020, 0x0030);
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __connection_interval_range__ {
    ($min:expr, $max:expr) => {{
        const {
            match $crate::ConnectionIntervalRange::try_new($min, $max) {
                Ok(v) => v,
                Err($crate::Error::InvalidConnectionIntervalRange) => panic!("the connection interval minimum value must be smaller or equal to the maximum value"),
                Err(_) => panic!("the connection interval value is invalid, it needs to be between 0x0006 and 0x0C80")
            }
        }
    }};
}

#[doc(inline)]
pub use __connection_interval_range__ as connection_interval_range;

pub mod parser {
    use nom::{combinator::map_res, number::complete::le_u16, IResult, Parser};

    use super::*;

    pub(crate) fn connection_interval(input: &[u8]) -> IResult<&[u8], ConnectionInterval> {
        map_res(le_u16, TryInto::try_into).parse(input)
    }

    pub fn connection_interval_range(input: &[u8]) -> IResult<&[u8], ConnectionIntervalRange> {
        map_res((le_u16, le_u16), |(start, end)| {
            ConnectionIntervalRange::try_new(start, end)
        })
        .parse(input)
    }
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;
    use claims::{assert_ge, assert_le};
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_connection_interval_default() {
        let value = ConnectionInterval::default();
        assert_eq!(value.value(), 64);
        assert_relative_eq!(value.milliseconds(), 80f32, epsilon = 1.0e-6);
    }

    #[rstest]
    #[case(0x0006.try_into().unwrap(), 0x0006, 7.5f32)]
    #[case(ConnectionInterval::try_new(0x0C80).unwrap(), 0x0C80, 4000f32)]
    #[case(ConnectionInterval::default(), 0x0040, 80f32)]
    fn test_connection_interval_success(
        #[case] interval: ConnectionInterval,
        #[case] expected_value: u16,
        #[case] expected_milliseconds: f32,
    ) {
        let value: u16 = interval.into();
        assert_eq!(value, expected_value);
        assert_relative_eq!(
            interval.milliseconds(),
            expected_milliseconds,
            epsilon = 1.0e-6
        )
    }

    #[rstest]
    #[case(0x0000)]
    #[case(0x0005)]
    #[case(0x0C81)]
    #[case(0xFFFE)]
    #[case(0xFFFF)]
    fn test_connection_interval_failure(#[case] input: u16) {
        let result = ConnectionInterval::try_new(input);
        assert_eq!(result, Err(Error::InvalidConnectionIntervalValue(input)));
    }

    #[test]
    fn test_connection_interval_comparison() -> Result<(), Error> {
        let value1: ConnectionInterval = 0x0020.try_into()?;
        let value2 = ConnectionInterval::try_new(0x0020)?;
        assert_eq!(value1, value2);

        let value1: ConnectionInterval = 0x0006.try_into()?;
        let value2 = ConnectionInterval::try_new(0x0C80)?;
        assert_le!(value1, value2);
        assert_ge!(value2, value1);

        Ok(())
    }

    #[test]
    fn test_connection_interval_range_default() {
        let range = ConnectionIntervalRange::default();
        assert_eq!(range.min(), ConnectionInterval::default());
        assert_eq!(range.max(), ConnectionInterval::default());
    }

    #[rstest]
    #[case(0x0020, 0x0020)]
    #[case(0x0020, 0x0030)]
    fn test_connection_interval_range_success(
        #[case] min: u16,
        #[case] max: u16,
    ) -> Result<(), Error> {
        let range = ConnectionIntervalRange::try_new(min, max)?;
        assert_eq!(range.min().value, min);
        assert_eq!(range.max().value, max);
        Ok(())
    }

    #[rstest]
    #[case(0x0005, 0x0020, Error::InvalidConnectionIntervalValue(0x0005))]
    #[case(0x0030, 0x0C81, Error::InvalidConnectionIntervalValue(0x0C81))]
    #[case(0x0030, 0x0020, Error::InvalidConnectionIntervalRange)]
    #[case(0x0C80, 0x0C7F, Error::InvalidConnectionIntervalRange)]
    fn test_connection_interval_range_failure(
        #[case] min: u16,
        #[case] max: u16,
        #[case] error: Error,
    ) {
        let err = ConnectionIntervalRange::try_new(min, max);
        assert_eq!(err, Err(error));
    }
}
