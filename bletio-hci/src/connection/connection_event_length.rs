use core::ops::RangeInclusive;

use bletio_utils::Error as UtilsError;
use bletio_utils::{BufferOps, EncodeToBuffer};

use crate::Error;

/// Connection Event Length.
///
/// The length of connection event recommended for a LE connection.
///
/// Here are the characteristics of this connection event length:
///  - Range: 0x0000 to 0xFFFF
///  - Time = N × 0.625 ms
///  - Time Range: 0 ms to 40.959375 s
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.12](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-18ff009e-8e3a-a32c-160f-23e297c0fc9d).
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ConnectionEventLength {
    value: u16,
}

impl ConnectionEventLength {
    /// Create a valid connection event length.
    pub const fn new(value: u16) -> Self {
        Self { value }
    }

    /// Get the value of the connection event length in milliseconds.
    pub const fn milliseconds(&self) -> f32 {
        (self.value as f32) * 0.625
    }

    pub const fn value(&self) -> u16 {
        self.value
    }
}

impl From<u16> for ConnectionEventLength {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}

impl EncodeToBuffer for ConnectionEventLength {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, UtilsError> {
        buffer.encode_le_u16(self.value)
    }

    fn encoded_size(&self) -> usize {
        size_of::<u16>()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ConnectionEventLengthRange {
    value: RangeInclusive<ConnectionEventLength>,
}

impl ConnectionEventLengthRange {
    pub const fn try_new(min: u16, max: u16) -> Result<Self, Error> {
        if min <= max {
            Ok(Self {
                value: ConnectionEventLength::new(min)..=ConnectionEventLength::new(max),
            })
        } else {
            Err(Error::InvalidConnectionEventLengthRange)
        }
    }

    pub const fn min(&self) -> ConnectionEventLength {
        *self.value.start()
    }

    pub const fn max(&self) -> ConnectionEventLength {
        *self.value.end()
    }
}

impl Default for ConnectionEventLengthRange {
    fn default() -> Self {
        Self {
            value: Default::default()..=Default::default(),
        }
    }
}

impl EncodeToBuffer for ConnectionEventLengthRange {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, UtilsError> {
        self.value.start().encode(buffer)?;
        self.value.end().encode(buffer)?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        self.value.start().encoded_size() + self.value.end().encoded_size()
    }
}

/// Create a `ConnectionEventLengthRange`, checking that it is valid at compile-time.
///
/// # Examples
///
/// ```
/// # use bletio_hci::connection_event_length_range;
/// let range = connection_event_length_range!(0x0020, 0x0030);
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __connection_event_length_range__ {
    ($min:expr, $max:expr) => {{
        const {
            match $crate::ConnectionEventLengthRange::try_new($min, $max) {
                Ok(v) => v,
                Err(_) => panic!("the connection event length range minimum value must be smaller or equal to the maximum value")
            }
        }
    }};
}

#[doc(inline)]
pub use __connection_event_length_range__ as connection_event_length_range;

pub(crate) mod parser {
    use nom::{combinator::map_res, number::complete::le_u16, IResult, Parser};

    use super::*;

    pub(crate) fn connection_event_length_range(
        input: &[u8],
    ) -> IResult<&[u8], ConnectionEventLengthRange> {
        map_res((le_u16, le_u16), |(start, end)| {
            ConnectionEventLengthRange::try_new(start, end)
        })
        .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(0x0000, 0f32)]
    #[case(0x0020, 20f32)]
    #[case(0xFFFF, 40959.375f32)]
    fn test_connection_event_length_success(
        #[case] input: u16,
        #[case] expected_milliseconds: f32,
    ) -> Result<(), Error> {
        let ce_length: ConnectionEventLength = input.into();
        assert_eq!(ce_length.value(), input);
        assert_relative_eq!(
            ce_length.milliseconds(),
            expected_milliseconds,
            epsilon = 1.0e-6
        );
        Ok(())
    }

    #[test]
    fn test_connection_event_length_range_default() {
        let range = ConnectionEventLengthRange::default();
        assert_eq!(range.min(), ConnectionEventLength::default());
        assert_eq!(range.max(), ConnectionEventLength::default());
    }

    #[rstest]
    #[case(0x0020, 0x0020)]
    #[case(0x0020, 0x0030)]
    fn test_connection_event_length_range_success(
        #[case] min: u16,
        #[case] max: u16,
    ) -> Result<(), Error> {
        let value = ConnectionEventLengthRange::try_new(min, max)?;
        assert_eq!(value.min().value, min);
        assert_eq!(value.max().value, max);
        Ok(())
    }

    #[rstest]
    #[case(0x0030, 0x0020, Error::InvalidConnectionEventLengthRange)]
    #[case(0x2000, 0x1000, Error::InvalidConnectionEventLengthRange)]
    fn test_connection_event_length_range_failure(
        #[case] min: u16,
        #[case] max: u16,
        #[case] error: Error,
    ) {
        let err = ConnectionEventLengthRange::try_new(min, max);
        assert_eq!(err, Err(error));
    }
}
