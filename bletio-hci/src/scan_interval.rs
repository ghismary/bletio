use bletio_utils::Error as UtilsError;
use bletio_utils::{BufferOps, EncodeToBuffer};

use crate::{Error, ScanWindow};

/// Scan interval.
///
/// This is the time interval from when the Controller started its last LE scan until it begins the subsequent LE scan.
///
/// Here are the characteristics of this scan interval:
///  - Range: 0x0004 to 0x4000
///  - Default: 0x0010 (10 ms)
///  - Time = N × 0.625 ms
///  - Time Range: 2.5 ms to 10.24 s
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.10](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-fff6f93e-315b-04fe-51bf-a18f78ceec89).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ScanInterval {
    value: u16,
}

impl ScanInterval {
    /// Create a valid scan interval.
    pub const fn try_new(value: u16) -> Result<Self, Error> {
        if (value >= 0x0004) && (value <= 0x4000) {
            Ok(Self { value })
        } else {
            Err(Error::InvalidScanInterval(value))
        }
    }

    /// Get the value of the scan interval in milliseconds.
    pub const fn milliseconds(&self) -> f32 {
        (self.value as f32) * 0.625
    }

    pub const fn value(&self) -> u16 {
        self.value
    }
}

impl Default for ScanInterval {
    fn default() -> Self {
        Self { value: 0x0010 }
    }
}

impl TryFrom<u16> for ScanInterval {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl EncodeToBuffer for ScanInterval {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, UtilsError> {
        buffer.encode_le_u16(self.value)
    }

    fn encoded_size(&self) -> usize {
        size_of::<u16>()
    }
}

impl PartialEq<ScanWindow> for ScanInterval {
    fn eq(&self, other: &ScanWindow) -> bool {
        self.value.eq(&other.value())
    }
}

impl PartialOrd<ScanWindow> for ScanInterval {
    fn partial_cmp(&self, other: &ScanWindow) -> Option<core::cmp::Ordering> {
        self.value.partial_cmp(&other.value())
    }
}

/// Create a [`ScanInterval`], checking that it is valid at compile-time.
///
/// # Examples
///
/// ```
/// # use bletio_hci::scan_interval;
/// let interval = scan_interval!(0x0020);
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __scan_interval__ {
    ($value:expr) => {{
        const {
            match $crate::scan_interval::ScanInterval::try_new($value) {
                Ok(v) => v,
                Err(_) => panic!("the scan interval value is invalid, it needs to be between 0x0004 and 0x4000")
            }
        }
    }};
}

#[doc(inline)]
pub use __scan_interval__ as scan_interval;

pub(crate) mod parser {
    use nom::{combinator::map_res, number::le_u16, IResult, Parser};

    use super::*;

    pub(crate) fn scan_interval(input: &[u8]) -> IResult<&[u8], ScanInterval> {
        map_res(le_u16(), TryInto::try_into).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_scan_interval_default() {
        let value = ScanInterval::default();
        assert_eq!(value.value(), 0x0010);
        assert_relative_eq!(value.milliseconds(), 10f32, epsilon = 1.0e-6);
    }

    #[rstest]
    #[case(0x0004, 2.5f32)]
    #[case(0x4000, 10240f32)]
    fn test_scan_interval_success(
        #[case] input: u16,
        #[case] expected_milliseconds: f32,
    ) -> Result<(), Error> {
        let value = ScanInterval::try_new(input)?;
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
    fn test_scan_interval_failure(#[case] input: u16) {
        let err = ScanInterval::try_new(input);
        assert_eq!(err, Err(Error::InvalidScanInterval(input)));
    }

    #[rstest]
    #[case(0x0004)]
    #[case(0x0010)]
    #[case(0x4000)]
    fn test_scan_interval_eq_scan_window(#[case] input: u16) -> Result<(), Error> {
        let scan_interval = ScanInterval::try_new(input)?;
        let scan_window = ScanWindow::try_new(input)?;
        assert_eq!(scan_interval, scan_window);
        assert_eq!(scan_window, scan_interval);
        Ok(())
    }

    #[rstest]
    #[case(0x0004, 0x0010)]
    #[case(0x0010, 0x4000)]
    #[case(0x4000, 0x0004)]
    fn test_scan_interval_ne_scan_window(
        #[case] scan_interval_value: u16,
        #[case] scan_window_value: u16,
    ) -> Result<(), Error> {
        let scan_interval = ScanInterval::try_new(scan_interval_value)?;
        let scan_window = ScanWindow::try_new(scan_window_value)?;
        assert_ne!(scan_interval, scan_window);
        assert_ne!(scan_window, scan_interval);
        Ok(())
    }

    #[rstest]
    #[case(0x0004, 0x0010)]
    #[case(0x0010, 0x4000)]
    fn test_scan_interval_smaller_than_scan_window(
        #[case] scan_interval_value: u16,
        #[case] scan_window_value: u16,
    ) -> Result<(), Error> {
        use claims::{assert_ge, assert_le};

        let scan_interval = ScanInterval::try_new(scan_interval_value)?;
        let scan_window = ScanWindow::try_new(scan_window_value)?;
        assert_le!(scan_interval, scan_window);
        assert_ge!(scan_window, scan_interval);
        Ok(())
    }

    #[rstest]
    #[case(0x0010, 0x0004)]
    #[case(0x4000, 0x0010)]
    fn test_scan_window_smaller_than_scan_interval(
        #[case] scan_interval_value: u16,
        #[case] scan_window_value: u16,
    ) -> Result<(), Error> {
        use claims::{assert_ge, assert_le};

        let scan_interval = ScanInterval::try_new(scan_interval_value)?;
        let scan_window = ScanWindow::try_new(scan_window_value)?;
        assert_ge!(scan_interval, scan_window);
        assert_le!(scan_window, scan_interval);
        Ok(())
    }
}
