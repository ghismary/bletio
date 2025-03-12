use bletio_utils::Error as UtilsError;
use bletio_utils::{BufferOps, EncodeToBuffer};

use crate::{Error, ScanInterval};

/// Scan window.
///
/// The duration of the LE scan.
///
/// Here are the characteristics of this scan window:
///  - Range: 0x0004 to 0x4000
///  - Default: 0x0010 (10 ms)
///  - Time = N × 0.625 ms
///  - Time Range: 2.5 ms to 10.24 s
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.10](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-fff6f93e-315b-04fe-51bf-a18f78ceec89).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ScanWindow {
    value: u16,
}

impl ScanWindow {
    /// Create a valid scan window.
    pub const fn try_new(value: u16) -> Result<Self, Error> {
        if (value >= 0x0004) && (value <= 0x4000) {
            Ok(Self { value })
        } else {
            Err(Error::InvalidScanWindow(value))
        }
    }

    /// Get the value of the scan window in milliseconds.
    pub const fn milliseconds(&self) -> f32 {
        (self.value as f32) * 0.625
    }

    pub const fn value(&self) -> u16 {
        self.value
    }
}

impl Default for ScanWindow {
    fn default() -> Self {
        Self { value: 0x0010 }
    }
}

impl TryFrom<u16> for ScanWindow {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl EncodeToBuffer for ScanWindow {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, UtilsError> {
        buffer.encode_le_u16(self.value)
    }

    fn encoded_size(&self) -> usize {
        size_of::<u16>()
    }
}

impl PartialEq<ScanInterval> for ScanWindow {
    fn eq(&self, other: &ScanInterval) -> bool {
        self.value.eq(&other.value())
    }
}

impl PartialOrd<ScanInterval> for ScanWindow {
    fn partial_cmp(&self, other: &ScanInterval) -> Option<core::cmp::Ordering> {
        self.value.partial_cmp(&other.value())
    }
}

/// Create a [`ScanWindow`], checking that it is valid at compile-time.
///
/// # Examples
///
/// ```
/// # use bletio_hci::scan_window;
/// let window = scan_window!(0x0040);
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __scan_window__ {
    ($value:expr) => {{
        const {
            match $crate::ScanWindow::try_new($value) {
                Ok(v) => v,
                Err(_) => panic!("the scan window value is invalid, it needs to be between 0x0004 and 0x4000")
            }
        }
    }};
}

#[doc(inline)]
pub use __scan_window__ as scan_window;

pub(crate) mod parser {
    use nom::{combinator::map_res, number::complete::le_u16, IResult, Parser};

    use super::*;

    pub(crate) fn scan_window(input: &[u8]) -> IResult<&[u8], ScanWindow> {
        map_res(le_u16, TryInto::try_into).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use rstest::rstest;

    #[test]
    fn test_scan_window_default() {
        let value = ScanWindow::default();
        assert_eq!(value.value(), 0x0010);
        assert_relative_eq!(value.milliseconds(), 10f32, epsilon = 1.0e-6);
    }

    #[rstest]
    #[case(0x0004, 2.5f32)]
    #[case(0x4000, 10240f32)]
    fn test_scan_window_success(
        #[case] input: u16,
        #[case] expected_milliseconds: f32,
    ) -> Result<(), Error> {
        let value = ScanWindow::try_new(input)?;
        assert_eq!(value.value(), input);
        assert_relative_eq!(
            value.milliseconds(),
            expected_milliseconds,
            epsilon = 1.0e-6
        );
        Ok(())
    }

    #[rstest]
    #[case(0x0003)]
    #[case(0x8000)]
    fn test_scan_window_failure(#[case] input: u16) {
        let err = ScanWindow::try_new(input);
        assert_eq!(err, Err(Error::InvalidScanWindow(input)));
    }
}
