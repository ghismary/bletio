#[derive(Debug)]
pub struct TxPowerLevel(pub i8);

impl From<i8> for TxPowerLevel {
    fn from(value: i8) -> Self {
        Self(value)
    }
}
