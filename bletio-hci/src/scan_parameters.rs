//! Scan Parameters.
//!
//! These Scan Parameters need to be defined to start scanning.

use bletio_utils::{BufferOps, EncodeToBuffer, Error as UtilsError};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{Error, OwnAddressType};

/// Scan type.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.10](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-fff6f93e-315b-04fe-51bf-a18f78ceec89).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[num_enum(error_type(name = Error, constructor = Error::InvalidScanType))]
#[repr(u8)]
#[non_exhaustive]
pub enum ScanType {
    /// Passive Scanning. No scanning PDUs shall be sent (default).
    #[default]
    PassiveScanning = 0x00,
    /// Active scanning. Scanning PDUs may be sent.
    ActiveScanning = 0x01,
}

impl EncodeToBuffer for ScanType {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, UtilsError> {
        buffer.try_push((*self).into())
    }

    fn encoded_size(&self) -> usize {
        size_of::<ScanType>()
    }
}

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
        if check_scan_interval_value(value) {
            Ok(Self::new_unchecked(value))
        } else {
            Err(Error::InvalidScanInterval(value))
        }
    }

    #[doc(hidden)]
    pub const fn new_unchecked(value: u16) -> Self {
        Self { value }
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
        self.value.eq(&other.value)
    }
}

impl PartialOrd<ScanWindow> for ScanInterval {
    fn partial_cmp(&self, other: &ScanWindow) -> Option<core::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

#[diagnostic::on_unimplemented(
    message = "the scan interval value is invalid, it needs to be between 0x0004 and 0x4000"
)]
#[doc(hidden)]
pub trait ScanIntervalValueValid {}

#[doc(hidden)]
pub struct ScanIntervalValueIsValid<const VALID: bool>;

#[doc(hidden)]
impl ScanIntervalValueValid for ScanIntervalValueIsValid<true> {}

#[doc(hidden)]
pub const fn scan_interval_value_is_valid<T: ScanIntervalValueValid>() {}

#[doc(hidden)]
pub const fn check_scan_interval_value(value: u16) -> bool {
    (value >= 0x0004) && (value <= 0x4000)
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
        const ERR: bool = bletio_hci::scan_parameters::check_scan_interval_value($value);
        bletio_hci::scan_parameters::scan_interval_value_is_valid::<
            bletio_hci::scan_parameters::ScanIntervalValueIsValid<ERR>,
        >();
        bletio_hci::scan_parameters::ScanInterval::new_unchecked($value)
    }};
}

#[doc(inline)]
pub use __scan_interval__ as scan_interval;

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
        if check_scan_window_value(value) {
            Ok(Self::new_unchecked(value))
        } else {
            Err(Error::InvalidScanWindow(value))
        }
    }

    #[doc(hidden)]
    pub const fn new_unchecked(value: u16) -> Self {
        Self { value }
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
        self.value.eq(&other.value)
    }
}

impl PartialOrd<ScanInterval> for ScanWindow {
    fn partial_cmp(&self, other: &ScanInterval) -> Option<core::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

#[diagnostic::on_unimplemented(
    message = "the scan window value is invalid, it needs to be between 0x0004 and 0x4000"
)]
#[doc(hidden)]
pub trait ScanWindowValueValid {}

#[doc(hidden)]
pub struct ScanWindowValueIsValid<const VALID: bool>;

#[doc(hidden)]
impl ScanWindowValueValid for ScanWindowValueIsValid<true> {}

#[doc(hidden)]
pub const fn scan_window_value_is_valid<T: ScanWindowValueValid>() {}

#[doc(hidden)]
pub const fn check_scan_window_value(value: u16) -> bool {
    (value >= 0x0004) && (value <= 0x4000)
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
        const ERR: bool = bletio_hci::scan_parameters::check_scan_window_value($value);
        bletio_hci::scan_parameters::scan_window_value_is_valid::<
            bletio_hci::scan_parameters::ScanWindowValueIsValid<ERR>,
        >();
        bletio_hci::scan_parameters::ScanWindow::new_unchecked($value)
    }};
}

#[doc(inline)]
pub use __scan_window__ as scan_window;

/// Scanning Filter Policy.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.10](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-fff6f93e-315b-04fe-51bf-a18f78ceec89).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[num_enum(error_type(name = Error, constructor = Error::InvalidScanningFilterPolicy))]
#[repr(u8)]
#[non_exhaustive]
pub enum ScanningFilterPolicy {
    /// Basic unfiltered scanning filter policy (default.)
    #[default]
    BasicUnfiltered = 0x00,
    /// Basic filtered scanning filter policy.
    BasicFiltered = 0x01,
    /// Extended unfiltered scanning filter policy.
    ExtendedUnfiltered = 0x02,
    /// Extended filtered scanning filter policy.
    ExtendedFiltered = 0x03,
}

impl EncodeToBuffer for ScanningFilterPolicy {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, UtilsError> {
        buffer.try_push((*self).into())
    }

    fn encoded_size(&self) -> usize {
        size_of::<ScanningFilterPolicy>()
    }
}

/// Scan parameters to be set before starting scanning.
///
/// It contains this information:
///  - the scan type
///  - the scan interval
///  - the scan window
///  - our own address type
///  - the scanning filter policy
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.10](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-fff6f93e-315b-04fe-51bf-a18f78ceec89).
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ScanParameters {
    r#type: ScanType,
    interval: ScanInterval,
    window: ScanWindow,
    own_address_type: OwnAddressType,
    filter_policy: ScanningFilterPolicy,
}

impl ScanParameters {
    pub fn try_new(
        r#type: ScanType,
        interval: ScanInterval,
        window: ScanWindow,
        own_address_type: OwnAddressType,
        filter_policy: ScanningFilterPolicy,
    ) -> Result<ScanParameters, Error> {
        if window <= interval {
            Ok(ScanParameters {
                r#type,
                interval,
                window,
                own_address_type,
                filter_policy,
            })
        } else {
            Err(Error::ScanWindowMustBeSmallerOrEqualToScanInterval)
        }
    }

    pub fn filter_policy(&self) -> ScanningFilterPolicy {
        self.filter_policy
    }

    pub fn interval(&self) -> ScanInterval {
        self.interval
    }

    pub fn own_address_type(&self) -> OwnAddressType {
        self.own_address_type
    }

    pub fn r#type(&self) -> ScanType {
        self.r#type
    }

    pub fn window(&self) -> ScanWindow {
        self.window
    }
}

impl EncodeToBuffer for ScanParameters {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, UtilsError> {
        self.r#type.encode(buffer)?;
        self.interval.encode(buffer)?;
        self.window.encode(buffer)?;
        self.own_address_type.encode(buffer)?;
        self.filter_policy.encode(buffer)?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        self.r#type.encoded_size()
            + self.interval.encoded_size()
            + self.window.encoded_size()
            + self.own_address_type.encoded_size()
            + self.filter_policy.encoded_size()
    }
}

pub(crate) mod parser {
    use nom::{
        combinator::{all_consuming, map_res},
        number::{le_u16, le_u8},
        IResult, Parser,
    };

    use crate::own_address_type::parser::own_address_type;

    use super::*;

    fn scan_type(input: &[u8]) -> IResult<&[u8], ScanType> {
        map_res(le_u8(), TryInto::try_into).parse(input)
    }

    fn scan_interval(input: &[u8]) -> IResult<&[u8], ScanInterval> {
        map_res(le_u16(), TryInto::try_into).parse(input)
    }

    fn scan_window(input: &[u8]) -> IResult<&[u8], ScanWindow> {
        map_res(le_u16(), TryInto::try_into).parse(input)
    }

    fn filter_policy(input: &[u8]) -> IResult<&[u8], ScanningFilterPolicy> {
        map_res(le_u8(), TryInto::try_into).parse(input)
    }

    pub(crate) fn scan_parameters(input: &[u8]) -> IResult<&[u8], ScanParameters> {
        all_consuming(map_res(
            (
                scan_type,
                scan_interval,
                scan_window,
                own_address_type,
                filter_policy,
            ),
            |(r#type, interval, window, own_address_type, filter_policy)| {
                ScanParameters::try_new(r#type, interval, window, own_address_type, filter_policy)
            },
        ))
        .parse(input)
    }
}

#[cfg(test)]
mod test {
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

    #[test]
    fn test_valid_scan_parameters_success() -> Result<(), Error> {
        let interval = ScanInterval::try_new(0x0020)?;
        let window = ScanWindow::try_new(0x0010)?;
        let scan_params = ScanParameters::try_new(
            ScanType::PassiveScanning,
            interval,
            window,
            OwnAddressType::RandomDeviceAddress,
            ScanningFilterPolicy::BasicFiltered,
        )?;
        assert_eq!(scan_params.r#type(), ScanType::PassiveScanning);
        assert_eq!(scan_params.interval(), interval);
        assert_eq!(scan_params.window(), window);
        assert_eq!(
            scan_params.own_address_type(),
            OwnAddressType::RandomDeviceAddress
        );
        assert_eq!(
            scan_params.filter_policy(),
            ScanningFilterPolicy::BasicFiltered
        );
        Ok(())
    }

    #[test]
    fn test_invalid_scan_parameters_failure() -> Result<(), Error> {
        let err = ScanParameters::try_new(
            ScanType::ActiveScanning,
            ScanInterval::try_new(0x0010)?,
            ScanWindow::try_new(0x0020)?,
            OwnAddressType::PublicDeviceAddress,
            ScanningFilterPolicy::BasicUnfiltered,
        );
        assert_eq!(
            err,
            Err(Error::ScanWindowMustBeSmallerOrEqualToScanInterval)
        );
        Ok(())
    }
}
