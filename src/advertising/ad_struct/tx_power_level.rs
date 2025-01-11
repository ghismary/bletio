use crate::advertising::ad_struct::{
    AdStruct, AdStructType, AD_STRUCT_DATA_OFFSET, AD_STRUCT_LENGTH_OFFSET, AD_STRUCT_TYPE_OFFSET,
};

use crate::assigned_numbers::AdType;

const TX_POWER_LEVEL_AD_STRUCT_SIZE: usize = 3;

/// Transmitted power level of the packet.
///
/// The TX Power Level should be the radiated power level. This value should be set to be as accurate
/// as possible, as defined in [Supplement to the Bluetooth Core Specification, Part A, 1.5](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-8d2b460f-594d-2f40-f5f1-80eab38f399d).
///
/// Note: When the TX Power Level Adevertising Structure is not present, the TX power level of the packet is unknown.
#[derive(Debug, Clone)]
pub struct TxPowerLevelAdStruct {
    buffer: [u8; TX_POWER_LEVEL_AD_STRUCT_SIZE],
}

impl TxPowerLevelAdStruct {
    /// Create a TX Power Level Advertising Structure.
    ///
    /// # Arguments
    ///
    /// * `tx_power_level` — The TX Power Level to notify.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bletio::advertising::ad_struct::TxPowerLevelAdStruct;
    /// let ad_struct = TxPowerLevelAdStruct::new(8);
    /// ```
    pub fn new(tx_power_level: impl Into<TxPowerLevel>) -> Self {
        let mut s = Self {
            buffer: Default::default(),
        };
        s.buffer[AD_STRUCT_LENGTH_OFFSET] = (TX_POWER_LEVEL_AD_STRUCT_SIZE - 1) as u8;
        s.buffer[AD_STRUCT_TYPE_OFFSET] = AdType::TxPowerLevel as u8;
        s.buffer[AD_STRUCT_DATA_OFFSET] = tx_power_level.into().0 as u8;
        s
    }
}

impl AdStruct for TxPowerLevelAdStruct {
    fn encoded_data(&self) -> &[u8] {
        &self.buffer
    }
    fn r#type(&self) -> AdStructType {
        AdStructType::TX_POWER_LEVEL
    }

    fn is_unique(&self) -> bool {
        false
    }
}

/// The TX Power Level, that is to say the radiated power level, in dBm.
///
/// The value ranges from -127 to 127 dBm.
#[derive(Debug)]
pub struct TxPowerLevel(pub i8);

impl From<i8> for TxPowerLevel {
    fn from(value: i8) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tx_power_level_ad_struct() {
        let value = TxPowerLevelAdStruct::new(-127);
        assert_eq!(value.encoded_data(), &[0x02, 0x0A, 0x81]);
        assert!(value.r#type().contains(AdStructType::TX_POWER_LEVEL));
        assert!(!value.is_unique());

        let value = TxPowerLevelAdStruct::new(127);
        assert_eq!(value.encoded_data(), &[0x02, 0x0A, 0x7F]);
        assert!(value.r#type().contains(AdStructType::TX_POWER_LEVEL));
        assert!(!value.is_unique());

        let value = TxPowerLevelAdStruct::new(TxPowerLevel(0));
        assert_eq!(value.encoded_data(), &[0x02, 0x0A, 0x00]);
        assert!(value.r#type().contains(AdStructType::TX_POWER_LEVEL));
        assert!(!value.is_unique());
    }
}
