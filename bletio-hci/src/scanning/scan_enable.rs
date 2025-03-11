use bletio_utils::{BufferOps, EncodeToBuffer};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::Error;

/// Enable/disable scan.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.11](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-bf0262b2-c9d0-b457-8405-5cf531a0bff1).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[num_enum(error_type(name = Error, constructor = Error::InvalidScanEnableValue))]
#[repr(u8)]
#[non_exhaustive]
pub enum ScanEnable {
    #[default]
    /// Scanning is disabled (default).
    Disabled = 0x00,
    /// Scanning is enabled.
    Enabled = 0x01,
}

impl EncodeToBuffer for ScanEnable {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, bletio_utils::Error> {
        buffer.try_push((*self).into())
    }

    fn encoded_size(&self) -> usize {
        size_of::<ScanEnable>()
    }
}

/// Filter out duplicate advertising reports.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.11](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-bf0262b2-c9d0-b457-8405-5cf531a0bff1).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[num_enum(error_type(name = Error, constructor = Error::InvalidFilterDuplicatesValue))]
#[repr(u8)]
#[non_exhaustive]
pub enum FilterDuplicates {
    #[default]
    /// Duplicate filtering is disabled (default).
    Disabled = 0x00,
    /// Duplicate filtering is enabled.
    Enabled = 0x01,
}

impl EncodeToBuffer for FilterDuplicates {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, bletio_utils::Error> {
        buffer.try_push((*self).into())
    }

    fn encoded_size(&self) -> usize {
        size_of::<FilterDuplicates>()
    }
}

pub(crate) mod parser {
    use nom::{
        combinator::{all_consuming, map_res},
        number::le_u8,
        IResult, Parser,
    };

    use super::{FilterDuplicates, ScanEnable};

    fn scan_enable(input: &[u8]) -> IResult<&[u8], ScanEnable> {
        map_res(le_u8(), TryInto::try_into).parse(input)
    }

    fn filter_duplicates(input: &[u8]) -> IResult<&[u8], FilterDuplicates> {
        map_res(le_u8(), TryInto::try_into).parse(input)
    }

    pub(crate) fn scan_enable_parameters(
        input: &[u8],
    ) -> IResult<&[u8], (ScanEnable, FilterDuplicates)> {
        all_consuming((scan_enable, filter_duplicates)).parse(input)
    }
}

#[cfg(test)]
mod test {
    use bletio_utils::{Buffer, BufferOps};
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(0, Ok(ScanEnable::Disabled))]
    #[case(1, Ok(ScanEnable::Enabled))]
    #[case(2, Err(Error::InvalidScanEnableValue(2)))]
    #[case(255, Err(Error::InvalidScanEnableValue(255)))]
    fn test_scan_enable_try_from_u8(
        #[case] input: u8,
        #[case] expected: Result<ScanEnable, Error>,
    ) {
        let result = input.try_into();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(ScanEnable::Enabled, &[0x01])]
    #[case(ScanEnable::Disabled, &[0x00])]
    fn test_scan_enable_encoding(
        #[case] enable: ScanEnable,
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<1>::default();
        assert_eq!(enable.encoded_size(), encoded_data.len());
        enable.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }

    #[rstest]
    #[case(0, Ok(FilterDuplicates::Disabled))]
    #[case(1, Ok(FilterDuplicates::Enabled))]
    #[case(2, Err(Error::InvalidFilterDuplicatesValue(2)))]
    #[case(255, Err(Error::InvalidFilterDuplicatesValue(255)))]
    fn test_filter_duplicates_try_from_u8(
        #[case] input: u8,
        #[case] expected: Result<FilterDuplicates, Error>,
    ) {
        let result = input.try_into();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(FilterDuplicates::Enabled, &[0x01])]
    #[case(FilterDuplicates::Disabled, &[0x00])]
    fn test_filter_duplicates_encoding(
        #[case] enable: FilterDuplicates,
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<1>::default();
        assert_eq!(enable.encoded_size(), encoded_data.len());
        enable.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }
}
