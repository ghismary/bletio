use num_enum::TryFromPrimitive;

use crate::Error;

/// Enable/disable advertising.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.9](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e58c6816-c25e-367a-0023-9da1700a3794).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, TryFromPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::InvalidAdvertisingEnableValue))]
#[repr(u8)]
#[non_exhaustive]
pub enum AdvertisingEnable {
    #[default]
    /// Advertising is disabled (default).
    Disabled = 0x00,
    /// Advertising is enabled.
    Enabled = 0x01,
}

impl bletio_utils::EncodeToBuffer for AdvertisingEnable {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push(*self as u8)
    }

    fn encoded_size(&self) -> usize {
        1
    }
}

pub(crate) mod parser {
    use nom::{
        combinator::{all_consuming, map_res},
        number::le_u8,
        IResult, Parser,
    };

    use super::AdvertisingEnable;

    pub(crate) fn advertising_enable(input: &[u8]) -> IResult<&[u8], AdvertisingEnable> {
        all_consuming(map_res(le_u8(), TryInto::try_into)).parse(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, Ok(AdvertisingEnable::Disabled))]
    #[case(1, Ok(AdvertisingEnable::Enabled))]
    #[case(2, Err(Error::InvalidAdvertisingEnableValue(2)))]
    #[case(255, Err(Error::InvalidAdvertisingEnableValue(255)))]
    fn test_advertising_enable_try_from_u8(
        #[case] input: u8,
        #[case] expected: Result<AdvertisingEnable, Error>,
    ) {
        let result = input.try_into();
        assert_eq!(result, expected);
    }
}
