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
            (ConnectionIntervalValueType::Undefined, ConnectionIntervalValueType::Defined(_)) => {
                Some(core::cmp::Ordering::Less)
            }
            (ConnectionIntervalValueType::Defined(_), ConnectionIntervalValueType::Undefined) => {
                Some(core::cmp::Ordering::Greater)
            }
            (ConnectionIntervalValueType::Undefined, ConnectionIntervalValueType::Undefined) => {
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_connection_interval_value_creation_success() -> Result<(), Error> {
        let value: ConnectionIntervalValue = 0x0006.try_into()?;
        let u16_value: u16 = value.into();
        assert_eq!(u16_value, 0x0006);

        let value: ConnectionIntervalValue = 0x0C80.try_into()?;
        let u16_value: u16 = value.into();
        assert_eq!(u16_value, 0x0C80);

        let value = ConnectionIntervalValue::undefined();
        let u16_value: u16 = value.into();
        assert_eq!(u16_value, 0xFFFF);

        let value = ConnectionIntervalValue::default();
        let u16_value: u16 = value.into();
        assert_eq!(u16_value, 0xFFFF);

        Ok(())
    }

    #[test]
    fn test_connection_interval_value_creation_failure() {
        let result: Result<ConnectionIntervalValue, Error> = 0x0000.try_into();
        let err = result.expect_err("Invalid connection interval value");
        assert!(matches!(err, Error::InvalidConnectionIntervalValue(0x0000)));

        let result: Result<ConnectionIntervalValue, Error> = 0x0005.try_into();
        let err = result.expect_err("Invalid connection interval value");
        assert!(matches!(err, Error::InvalidConnectionIntervalValue(0x0005)));

        let result: Result<ConnectionIntervalValue, Error> = 0x0C81.try_into();
        let err = result.expect_err("Invalid connection interval value");
        assert!(matches!(err, Error::InvalidConnectionIntervalValue(0x0C81)));

        let result: Result<ConnectionIntervalValue, Error> = 0xFFFF.try_into();
        let err = result.expect_err("Invalid connection interval value");
        assert!(matches!(err, Error::InvalidConnectionIntervalValue(0xFFFF)));
    }
}
