//! Scan Parameters.
//!
//! These Scan Parameters need to be defined to start scanning.

use bletio_utils::{BufferOps, EncodeToBuffer, Error as UtilsError};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{Error, OwnAddressType, ScanInterval, ScanWindow};

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
        number::complete::le_u8,
        IResult, Parser,
    };

    use crate::common::own_address_type::parser::own_address_type;
    use crate::scanning::{scan_interval::parser::scan_interval, scan_window::parser::scan_window};

    use super::*;

    fn scan_type(input: &[u8]) -> IResult<&[u8], ScanType> {
        map_res(le_u8, TryInto::try_into).parse(input)
    }

    fn filter_policy(input: &[u8]) -> IResult<&[u8], ScanningFilterPolicy> {
        map_res(le_u8, TryInto::try_into).parse(input)
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
    use super::*;

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
