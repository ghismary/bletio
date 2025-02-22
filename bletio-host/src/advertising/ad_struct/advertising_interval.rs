use bletio_hci::AdvertisingInterval;
use bletio_utils::EncodeToBuffer;

use crate::assigned_numbers::AdType;

const ADVERTISING_INTERVAL_AD_STRUCT_SIZE: usize = 3;

/// The advertising interval.
///
/// The advertising interval is defined in
/// [Supplement to the Bluetooth Core Specification, Part A, 1.15](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-30c6e42e-5327-f52c-d79e-1b174095c712),
/// and
/// [Core Specification 6.0, Vol. 6, Part B, 4.4.2.2](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/low-energy-controller/link-layer-specification.html#UUID-f6cd1541-800c-c516-b32b-95dd0479840b).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AdvertisingIntervalAdStruct {
    interval: AdvertisingInterval,
}

impl AdvertisingIntervalAdStruct {
    pub(crate) const fn new(interval: AdvertisingInterval) -> Self {
        Self { interval }
    }
}

impl EncodeToBuffer for AdvertisingIntervalAdStruct {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push(ADVERTISING_INTERVAL_AD_STRUCT_SIZE as u8)?;
        buffer.try_push(AdType::AdvertisingInterval as u8)?;
        buffer.encode_le_u16(self.interval.value())?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        ADVERTISING_INTERVAL_AD_STRUCT_SIZE + 1
    }
}

pub(crate) mod parser {
    use nom::{
        combinator::{map, map_res},
        number::le_u16,
        IResult, Parser,
    };

    use crate::advertising::ad_struct::AdStruct;

    use super::*;

    pub(crate) fn advertising_interval_ad_struct(input: &[u8]) -> IResult<&[u8], AdStruct> {
        map(map_res(le_u16(), TryInto::try_into), |interval| {
            AdStruct::AdvertisingInterval(AdvertisingIntervalAdStruct::new(interval))
        })
        .parse(input)
    }
}

#[cfg(test)]
mod test {
    use bletio_utils::{Buffer, BufferOps};
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(AdvertisingInterval::default(), &[0x03, 0x1A, 0x00, 0x08])]
    #[case(AdvertisingInterval::try_new(0x0020).unwrap(), &[0x03, 0x1A, 0x20, 0x00])]
    #[case(AdvertisingInterval::try_new(0x4000).unwrap(), &[0x03, 0x1A, 0x00, 0x40])]
    fn test_advertising_interval_ad_struct(
        #[case] interval: AdvertisingInterval,
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<4>::default();
        let value = AdvertisingIntervalAdStruct::new(interval);
        value.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }
}
