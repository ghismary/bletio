use crate::Error;

/// Value to be used in a connection interval range, as used in the
/// [PeripheralConnectionIntervalRangeAdStruct](crate::advertising::ad_struct::PeripheralConnectionIntervalRangeAdStruct)
/// Advertising Structure.
///
/// Here are the characteristics of this connection interval value:
///  - Range: 0x0006 to 0x0C80 if specified
///  - Can be unspecified
///  - Time = N × 1.25 ms
///  - Time Range: 7.5 ms to 4 s
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
pub struct ConnectionIntervalValue {
    value: ConnectionIntervalValueType,
}

impl ConnectionIntervalValue {
    /// Create a connection interval value with a defined value.
    pub fn try_new(value: u16) -> Result<Self, Error> {
        value.try_into()
    }

    /// Create an undefined connection interval value.
    pub fn undefined() -> Self {
        Self {
            value: ConnectionIntervalValueType::Undefined,
        }
    }

    /// Get the value of the connection interval value in milliseconds.
    pub fn milliseconds(&self) -> Option<f32> {
        match self.value {
            ConnectionIntervalValueType::Defined(value) => Some(value as f32 * 1.25),
            ConnectionIntervalValueType::Undefined => None,
        }
    }
}

impl Default for ConnectionIntervalValue {
    fn default() -> Self {
        Self::undefined()
    }
}

impl TryFrom<u16> for ConnectionIntervalValue {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if (0x0006..=0x0C80).contains(&value) {
            Ok(Self {
                value: ConnectionIntervalValueType::Defined(value),
            })
        } else {
            Err(Error::InvalidConnectionIntervalValue(value))
        }
    }
}

impl From<ConnectionIntervalValue> for u16 {
    fn from(value: ConnectionIntervalValue) -> Self {
        match value.value {
            ConnectionIntervalValueType::Undefined => 0xFFFF,
            ConnectionIntervalValueType::Defined(value) => value,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq)]
enum ConnectionIntervalValueType {
    Defined(u16),
    Undefined,
}

impl PartialEq for ConnectionIntervalValueType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                ConnectionIntervalValueType::Defined(v1),
                ConnectionIntervalValueType::Defined(v2),
            ) => v1.eq(v2),
            _ => false,
        }
    }
}

impl PartialOrd for ConnectionIntervalValueType {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match (self, other) {
            (
                ConnectionIntervalValueType::Defined(v1),
                ConnectionIntervalValueType::Defined(v2),
            ) => v1.partial_cmp(v2),
            _ => Some(core::cmp::Ordering::Less),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_relative_eq;
    use claim::{assert_ge, assert_le};

    #[test]
    fn test_connection_interval_value_creation_success() -> Result<(), Error> {
        let value: ConnectionIntervalValue = 0x0006.try_into()?;
        let u16_value: u16 = value.into();
        assert_eq!(u16_value, 0x0006);
        assert_relative_eq!(value.milliseconds().unwrap(), 7.5f32, epsilon = 1.0e-6);

        let value = ConnectionIntervalValue::try_new(0x0C80)?;
        let u16_value: u16 = value.into();
        assert_eq!(u16_value, 0x0C80);
        assert_relative_eq!(value.milliseconds().unwrap(), 4000f32, epsilon = 1.0e-6);

        let value = ConnectionIntervalValue::undefined();
        let u16_value: u16 = value.into();
        assert_eq!(u16_value, 0xFFFF);
        assert!(value.milliseconds().is_none());

        let value = ConnectionIntervalValue::default();
        let u16_value: u16 = value.into();
        assert_eq!(u16_value, 0xFFFF);
        assert!(value.milliseconds().is_none());

        Ok(())
    }

    #[test]
    fn test_connection_interval_value_creation_failure() {
        let result = ConnectionIntervalValue::try_new(0x0000);
        let err = result.expect_err("Invalid connection interval value");
        assert!(matches!(err, Error::InvalidConnectionIntervalValue(0x0000)));

        let result: Result<ConnectionIntervalValue, Error> = 0x0005.try_into();
        let err = result.expect_err("Invalid connection interval value");
        assert!(matches!(err, Error::InvalidConnectionIntervalValue(0x0005)));

        let result = ConnectionIntervalValue::try_new(0x0C81);
        let err = result.expect_err("Invalid connection interval value");
        assert!(matches!(err, Error::InvalidConnectionIntervalValue(0x0C81)));

        let result: Result<ConnectionIntervalValue, Error> = 0xFFFF.try_into();
        let err = result.expect_err("Invalid connection interval value");
        assert!(matches!(err, Error::InvalidConnectionIntervalValue(0xFFFF)));
    }

    #[test]
    fn test_connection_interval_value_comparison() -> Result<(), Error> {
        let value1: ConnectionIntervalValue = 0x0020.try_into()?;
        let value2 = ConnectionIntervalValue::try_new(0x0020)?;
        assert_eq!(value1, value2);

        let value1 = ConnectionIntervalValue::undefined();
        let value2 = ConnectionIntervalValue::undefined();
        assert_ne!(value1, value2);
        assert_le!(value1, value2);
        assert_le!(value2, value1);

        let value1: ConnectionIntervalValue = 0x0006.try_into()?;
        let value2 = ConnectionIntervalValue::try_new(0x0C80)?;
        assert_le!(value1, value2);
        assert_ge!(value2, value1);

        let value1 = ConnectionIntervalValue::undefined();
        let value2 = ConnectionIntervalValue::try_new(0x0100)?;
        assert_le!(value1, value2);
        assert_le!(value2, value1);

        Ok(())
    }
}
