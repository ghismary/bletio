use crate::Error;

/// Value to be used in a connection interval range.
///
/// Here are the characteristics of this connection interval value:
///  - Range: 0x0006 to 0x0C80 if specified
///  - Can be unspecified
///  - Time = N × 1.25 ms
///  - Time Range: 7.5 ms to 4 s
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ConnectionInterval {
    value: ConnectionIntervalType,
}

impl ConnectionInterval {
    /// Create a connection interval value with a defined value.
    pub const fn try_new(value: u16) -> Result<Self, Error> {
        if (value >= 0x0006) && (value <= 0x0C80) {
            Ok(Self {
                value: ConnectionIntervalType::Defined(value),
            })
        } else {
            Err(Error::InvalidConnectionIntervalValue(value))
        }
    }

    /// Create an undefined connection interval value.
    pub const fn undefined() -> Self {
        Self {
            value: ConnectionIntervalType::Undefined,
        }
    }

    /// Get the value of the connection interval value in milliseconds.
    pub const fn milliseconds(&self) -> Option<f32> {
        match self.value {
            ConnectionIntervalType::Defined(value) => Some(value as f32 * 1.25),
            ConnectionIntervalType::Undefined => None,
        }
    }
}

impl Default for ConnectionInterval {
    fn default() -> Self {
        Self::undefined()
    }
}

impl TryFrom<u16> for ConnectionInterval {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl From<ConnectionInterval> for u16 {
    fn from(value: ConnectionInterval) -> Self {
        match value.value {
            ConnectionIntervalType::Undefined => 0xFFFF,
            ConnectionIntervalType::Defined(value) => value,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
enum ConnectionIntervalType {
    Defined(u16),
    Undefined,
}

impl PartialEq for ConnectionIntervalType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ConnectionIntervalType::Defined(v1), ConnectionIntervalType::Defined(v2)) => v1.eq(v2),
            _ => false,
        }
    }
}

impl PartialOrd for ConnectionIntervalType {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match (self, other) {
            (ConnectionIntervalType::Defined(v1), ConnectionIntervalType::Defined(v2)) => {
                v1.partial_cmp(v2)
            }
            _ => Some(core::cmp::Ordering::Less),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_relative_eq;
    use claims::{assert_ge, assert_le};
    use rstest::rstest;

    #[rstest]
    #[case(0x0006.try_into().unwrap(), 0x0006, Some(7.5f32))]
    #[case(ConnectionInterval::try_new(0x0C80).unwrap(), 0x0C80, Some(4000f32))]
    #[case(ConnectionInterval::undefined(), 0xFFFF, None)]
    #[case(ConnectionInterval::default(), 0xFFFF, None)]
    fn test_connection_interval_creation_success(
        #[case] interval: ConnectionInterval,
        #[case] expected_value: u16,
        #[case] expected_milliseconds: Option<f32>,
    ) {
        let value: u16 = interval.into();
        assert_eq!(value, expected_value);
        match expected_milliseconds {
            Some(expected) => {
                assert_relative_eq!(interval.milliseconds().unwrap(), expected, epsilon = 1.0e-6)
            }
            None => assert!(interval.milliseconds().is_none()),
        }
    }

    #[rstest]
    #[case(0x0000)]
    #[case(0x0005)]
    #[case(0x0C81)]
    #[case(0xFFFF)]
    fn test_connection_interval_creation_failure(#[case] input: u16) {
        let result = ConnectionInterval::try_new(input);
        assert_eq!(result, Err(Error::InvalidConnectionIntervalValue(input)));
    }

    #[test]
    fn test_connection_interval_value_comparison() -> Result<(), Error> {
        let value1: ConnectionInterval = 0x0020.try_into()?;
        let value2 = ConnectionInterval::try_new(0x0020)?;
        assert_eq!(value1, value2);

        let value1 = ConnectionInterval::undefined();
        let value2 = ConnectionInterval::undefined();
        assert_ne!(value1, value2);
        assert_le!(value1, value2);
        assert_le!(value2, value1);

        let value1: ConnectionInterval = 0x0006.try_into()?;
        let value2 = ConnectionInterval::try_new(0x0C80)?;
        assert_le!(value1, value2);
        assert_ge!(value2, value1);

        let value1 = ConnectionInterval::undefined();
        let value2 = ConnectionInterval::try_new(0x0100)?;
        assert_le!(value1, value2);
        assert_le!(value2, value1);

        Ok(())
    }
}
