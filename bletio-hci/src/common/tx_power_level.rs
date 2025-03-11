use crate::Error;

/// The TX Power Level, that is to say the radiated power level, in dBm.
///
/// The value ranges from -127 to 20 dBm.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TxPowerLevel {
    value: i8,
}

impl TxPowerLevel {
    pub const fn try_new(value: i8) -> Result<Self, Error> {
        if value > 20 {
            Err(Error::InvalidTxPowerLevelValue(value))
        } else {
            Ok(Self { value })
        }
    }

    pub const fn value(&self) -> i8 {
        self.value
    }
}

impl TryFrom<i8> for TxPowerLevel {
    type Error = Error;

    fn try_from(value: i8) -> Result<Self, Error> {
        Self::try_new(value)
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(-127)]
    #[case(20)]
    #[case(0)]
    fn test_tx_power_level_try_new_success(#[case] input: i8) {
        let value = TxPowerLevel::try_new(input).unwrap();
        assert_eq!(value.value(), input);
    }

    #[rstest]
    #[case(21)]
    #[case(64)]
    #[case(127)]
    fn test_tx_power_level_try_new_failure(#[case] input: i8) {
        let err = TxPowerLevel::try_new(input);
        assert_eq!(err, Err(Error::InvalidTxPowerLevelValue(input)));
    }
}
