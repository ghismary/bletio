use bletio_utils::Error as UtilsError;
use bletio_utils::{BufferOps, EncodeToBuffer};

use crate::Error;

/// Peripheral Latency for the connection in number of connection events.
///
/// Here are the characteristics of this latency:
///  - Range: 0x0000 to 0x01F3
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.12](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-18ff009e-8e3a-a32c-160f-23e297c0fc9d).
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Latency {
    value: u16,
}

impl Latency {
    /// Create a valid latency.
    pub const fn try_new(value: u16) -> Result<Self, Error> {
        if value <= 0x01F3 {
            Ok(Self { value })
        } else {
            Err(Error::InvalidLatency(value))
        }
    }

    pub const fn value(&self) -> u16 {
        self.value
    }
}

impl TryFrom<u16> for Latency {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl EncodeToBuffer for Latency {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, UtilsError> {
        buffer.encode_le_u16(self.value)
    }

    fn encoded_size(&self) -> usize {
        size_of::<u16>()
    }
}

/// Create a `Latency`, checking that it is valid at compile-time.
///
/// # Examples
///
/// ```
/// # use bletio_hci::latency;
/// let latency = latency!(0x0100);
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __latency__ {
    ($value:expr) => {{
        const {
            match $crate::Latency::try_new($value) {
                Ok(v) => v,
                Err(_) => {
                    panic!("the latency value is invalid, it needs to be between 0x0000 and 0x01F3")
                }
            }
        }
    }};
}

#[doc(inline)]
pub use __latency__ as latency;

pub(crate) mod parser {
    use nom::{combinator::map_res, number::complete::le_u16, IResult, Parser};

    use super::*;

    pub(crate) fn latency(input: &[u8]) -> IResult<&[u8], Latency> {
        map_res(le_u16, TryInto::try_into).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(0x0000)]
    #[case(0x0020)]
    #[case(0x01F3)]
    fn test_latency_success(#[case] input: u16) -> Result<(), Error> {
        let latency = Latency::try_new(input)?;
        assert_eq!(latency.value(), input);
        Ok(())
    }

    #[rstest]
    #[case(0x01F4)]
    #[case(0x4000)]
    #[case(0xFFFF)]
    fn test_latency_failure(#[case] input: u16) {
        let err = Latency::try_new(input);
        assert_eq!(err, Err(Error::InvalidLatency(input)));
    }
}
