use crate::Error;

#[derive(Debug, Copy, Clone, Eq)]
pub enum ConnectionIntervalValue {
    Defined(u16),
    Undefined,
}

impl ConnectionIntervalValue {
    pub fn milliseconds(&self) -> Option<f32> {
        match self {
            Self::Defined(value) => Some(*value as f32 * 1.25),
            Self::Undefined => None,
        }
    }
}

impl Default for ConnectionIntervalValue {
    fn default() -> Self {
        Self::Undefined
    }
}

impl PartialEq for ConnectionIntervalValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ConnectionIntervalValue::Defined(v1), ConnectionIntervalValue::Defined(v2)) => {
                v1.eq(v2)
            }
            _ => false,
        }
    }
}

impl PartialOrd for ConnectionIntervalValue {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match (self, other) {
            (ConnectionIntervalValue::Defined(v1), ConnectionIntervalValue::Defined(v2)) => {
                v1.partial_cmp(v2)
            }
            (ConnectionIntervalValue::Undefined, ConnectionIntervalValue::Defined(_)) => {
                Some(core::cmp::Ordering::Less)
            }
            (ConnectionIntervalValue::Defined(_), ConnectionIntervalValue::Undefined) => {
                Some(core::cmp::Ordering::Greater)
            }
            (ConnectionIntervalValue::Undefined, ConnectionIntervalValue::Undefined) => None,
        }
    }
}

impl TryFrom<u16> for ConnectionIntervalValue {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if (0x0006..=0x0C80).contains(&value) {
            Ok(Self::Defined(value))
        } else {
            Err(Error::InvalidConnectionIntervalValue(value))
        }
    }
}

impl From<ConnectionIntervalValue> for u16 {
    fn from(value: ConnectionIntervalValue) -> Self {
        match value {
            ConnectionIntervalValue::Undefined => 0xFFFF,
            ConnectionIntervalValue::Defined(value) => value,
        }
    }
}
