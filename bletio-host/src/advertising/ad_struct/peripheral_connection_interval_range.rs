use core::ops::RangeInclusive;

use bletio_hci::ConnectionInterval;
use bletio_utils::EncodeToBuffer;

use crate::assigned_numbers::AdType;

const PERIPHERAL_CONNECTION_INTERVAL_RANGE_AD_STRUCT_SIZE: usize = 5;

/// Peripheral’s preferred connection interval range, for all logical connections.
///
/// For more information about this connection interval, see
/// [Core Specification 6.0, Vol.3, Part C, 12.3](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-7ef0bdcb-4c81-1aea-5f65-4a69eab5c899).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PeripheralConnectionIntervalRangeAdStruct {
    range: RangeInclusive<ConnectionInterval>,
}

impl PeripheralConnectionIntervalRangeAdStruct {
    pub(crate) const fn new(range: RangeInclusive<ConnectionInterval>) -> Self {
        Self { range }
    }

    pub fn value(&self) -> RangeInclusive<ConnectionInterval> {
        self.range.clone()
    }
}

impl EncodeToBuffer for PeripheralConnectionIntervalRangeAdStruct {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push(PERIPHERAL_CONNECTION_INTERVAL_RANGE_AD_STRUCT_SIZE as u8)?;
        buffer.try_push(AdType::PeripheralConnectionIntervalRange as u8)?;
        buffer.encode_le_u16((*self.range.start()).into())?;
        buffer.encode_le_u16((*self.range.end()).into())?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        PERIPHERAL_CONNECTION_INTERVAL_RANGE_AD_STRUCT_SIZE + 1
    }
}

pub(crate) mod parser {
    use core::ops::RangeInclusive;

    use nom::{
        combinator::{map, map_res, verify},
        number::le_u16,
        IResult, Parser,
    };

    use crate::advertising::ad_struct::AdStruct;

    use super::*;

    fn connection_interval(input: &[u8]) -> IResult<&[u8], ConnectionInterval> {
        map_res(le_u16(), TryInto::try_into).parse(input)
    }

    pub(crate) fn peripheral_connection_interval_range_ad_struct(
        input: &[u8],
    ) -> IResult<&[u8], AdStruct> {
        map(
            map(
                verify((connection_interval, connection_interval), |(a, b)| a <= b),
                |(start, end)| RangeInclusive::new(start, end),
            ),
            |range| {
                AdStruct::PeripheralConnectionIntervalRange(
                    PeripheralConnectionIntervalRangeAdStruct::new(range),
                )
            },
        )
        .parse(input)
    }
}

#[cfg(test)]
mod test {
    use bletio_utils::{Buffer, BufferOps};
    use rstest::rstest;

    use crate::advertising::ad_struct::AdStruct;

    use super::{parser::*, *};

    #[rstest]
    #[case(
        0x0006.try_into().unwrap()..=0x0C80.try_into().unwrap(),
        Some(0x0006.try_into().unwrap()..=0x0C80.try_into().unwrap()),
        &[0x05, 0x12, 0x06, 0x00, 0x80, 0x0C]
    )]
    #[case(ConnectionInterval::undefined()..=0x0C80.try_into().unwrap(), None, &[0x05, 0x12, 0xFF, 0xFF, 0x80, 0x0C])]
    #[case(0x0006.try_into().unwrap()..=ConnectionInterval::undefined(), None, &[0x05, 0x12, 0x06, 0x00, 0xFF, 0xFF])]
    #[case(ConnectionInterval::undefined()..=ConnectionInterval::undefined(), None, &[0x05, 0x12, 0xFF, 0xFF, 0xFF, 0xFF])]
    #[case(
        0x0010.try_into().unwrap()..=0x0010.try_into().unwrap(),
        Some(0x0010.try_into().unwrap()..=0x0010.try_into().unwrap()),
        &[0x05, 0x12, 0x10, 0x00, 0x10, 0x00]
    )]
    fn test_peripheral_connection_interval_range_ad_struct(
        #[case] range: RangeInclusive<ConnectionInterval>,
        #[case] expected_range: Option<RangeInclusive<ConnectionInterval>>,
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<6>::default();
        let ad_struct = PeripheralConnectionIntervalRangeAdStruct::new(range.clone());
        ad_struct.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        if let Some(range) = expected_range {
            assert_eq!(ad_struct.value(), range);
        }
        Ok(())
    }

    #[rstest]
    #[case(&[0x06, 0x00, 0x80, 0x0C], 0x0006.try_into().unwrap()..=0x0C80.try_into().unwrap())]
    #[case(&[0x10, 0x00, 0x10, 0x00], 0x0010.try_into().unwrap()..=0x0010.try_into().unwrap())]
    fn test_peripheral_connection_interval_range_ad_struct_parsing_success(
        #[case] input: &[u8],
        #[case] range: RangeInclusive<ConnectionInterval>,
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
