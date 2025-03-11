use bletio_utils::Error as UtilsError;
use bletio_utils::{BufferOps, EncodeToBuffer};

use crate::Error;

/// Supervision timeout.
///
/// Supervision timeout for the LE Link (See [Core Specification 6.0, Vol.6, Part B, 4.5.2](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/low-energy-controller/link-layer-specification.html#UUID-741a7d39-97ed-b38f-d802-ba2f7a516703)).
///
/// Here are the characteristics of this supervision timeout:
///  - Range: 0x000A to 0x0C80
///  - Default: 0x000A (100 ms)
///  - Time = N × 10 ms
///  - Time Range: 100 ms to 32 s
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.12](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-18ff009e-8e3a-a32c-160f-23e297c0fc9d).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SupervisionTimeout {
    value: u16,
}

impl SupervisionTimeout {
    /// Create a valid supervision timeout.
    pub const fn try_new(value: u16) -> Result<Self, Error> {
        if (value >= 0x000A) && (value <= 0x0C80) {
            Ok(Self { value })
        } else {
            Err(Error::InvalidSupervisionTimeout(value))
        }
    }

    /// Get the value of the supervision timeout in milliseconds.
    pub const fn milliseconds(&self) -> f32 {
        (self.value as f32) * 10.0
    }

    pub const fn value(&self) -> u16 {
        self.value
    }
}

impl Default for SupervisionTimeout {
    fn default() -> Self {
        Self { value: 0x0020 }
    }
}

impl TryFrom<u16> for SupervisionTimeout {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl EncodeToBuffer for SupervisionTimeout {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, UtilsError> {
        buffer.encode_le_u16(self.value)
    }

    fn encoded_size(&self) -> usize {
        size_of::<u16>()
    }
}

/// Create a `SupervisionTimeout`, checking that it is valid at compile-time.
///
/// # Examples
///
/// ```
/// # use bletio_hci::supervision_timeout;
/// let timeout = supervision_timeout!(0x0050);
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __supervision_timeout__ {
    ($value:expr) => {{
        const {
            match $crate::SupervisionTimeout::try_new($value) {
                Ok(v) => v,
                Err(_) => panic!("the supervision timeout value is invalid, it needs to be between 0x000A and 0x0C80")
            }
        }
    }};
}

#[doc(inline)]
pub use __supervision_timeout__ as supervision_timeout;

pub(crate) mod parser {
    use nom::{combinator::map_res, number::complete::le_u16, IResult, Parser};

    use super::*;

    pub(crate) fn supervision_timeout(input: &[u8]) -> IResult<&[u8], SupervisionTimeout> {
        map_res(le_u16, TryInto::try_into).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(0x000A, 100f32)]
    #[case(0x0020, 320f32)]
    #[case(0x0C80, 32000f32)]
    fn test_supervision_timeout_success(
        #[case] input: u16,
        #[case] expected_milliseconds: f32,
    ) -> Result<(), Error> {
        let timeout = SupervisionTimeout::try_new(input)?;
        assert_eq!(timeout.value(), input);
        assert_relative_eq!(
            timeout.milliseconds(),
            expected_milliseconds,
            epsilon = 1.0e-6
        );
        Ok(())
    }

    #[rstest]
    #[case(0x0009)]
    #[case(0x0C81)]
    #[case(0x4000)]
    fn test_supervision_timeout_failure(#[case] input: u16) {
        let err = SupervisionTimeout::try_new(input);
        assert_eq!(err, Err(Error::InvalidSupervisionTimeout(input)));
    }
}
