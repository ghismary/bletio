use bletio_hci::TxPowerLevel;
use bletio_utils::EncodeToBuffer;

use crate::assigned_numbers::AdType;

const TX_POWER_LEVEL_AD_STRUCT_SIZE: usize = 2;

/// Transmitted power level of the packet.
///
/// The TX Power Level should be the radiated power level. This value should be set to be as accurate
/// as possible, as defined in [Supplement to the Bluetooth Core Specification, Part A, 1.5](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-8d2b460f-594d-2f40-f5f1-80eab38f399d).
///
/// Note: When the TX Power Level Adevertising Structure is not present, the TX power level of the packet is unknown.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TxPowerLevelAdStruct {
    tx_power_level: TxPowerLevel,
}

impl TxPowerLevelAdStruct {
    pub(crate) const fn new(tx_power_level: TxPowerLevel) -> Self {
        Self { tx_power_level }
    }
}

impl EncodeToBuffer for TxPowerLevelAdStruct {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push(TX_POWER_LEVEL_AD_STRUCT_SIZE as u8)?;
        buffer.try_push(AdType::TxPowerLevel as u8)?;
        buffer.try_push(self.tx_power_level.value() as u8)?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        TX_POWER_LEVEL_AD_STRUCT_SIZE + 1
    }
}

pub(crate) mod parser {
    use bletio_hci::TxPowerLevel;
    use nom::{
        combinator::{map, map_res},
        number::le_i8,
        IResult, Parser,
    };

    use crate::advertising::ad_struct::AdStruct;

    use super::*;

    fn tx_power_level(input: &[u8]) -> IResult<&[u8], TxPowerLevel> {
        map_res(le_i8(), TryInto::try_into).parse(input)
    }

    pub(crate) fn tx_power_level_ad_struct(input: &[u8]) -> IResult<&[u8], AdStruct> {
        map(tx_power_level, |v| {
            AdStruct::TxPowerLevel(TxPowerLevelAdStruct::new(v))
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
    #[case(-127, &[0x02, 0x0A, 0x81])]
    #[case(20, &[0x02, 0x0A, 0x14])]
    #[case(0, &[0x02, 0x0A, 0x00])]
    fn test_tx_power_level_ad_struct(
        #[case] tx_power_level: i8,
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<3>::default();
        let value = TxPowerLevelAdStruct::new(TxPowerLevel::try_new(tx_power_level).unwrap());
        value.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }
}
