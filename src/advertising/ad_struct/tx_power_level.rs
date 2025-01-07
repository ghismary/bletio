use crate::advertising::ad_struct::{
    AdStruct, AdStructType, AD_STRUCT_DATA_OFFSET, AD_STRUCT_LENGTH_OFFSET, AD_STRUCT_TYPE_OFFSET,
};

use crate::advertising::ad_struct::common_data_types::CommonDataType;
use crate::advertising::tx_power_level::TxPowerLevel;

const TX_POWER_LEVEL_AD_STRUCT_SIZE: usize = 3;

#[derive(Debug, Clone, Copy)]
pub struct TxPowerLevelAdStruct {
    buffer: [u8; TX_POWER_LEVEL_AD_STRUCT_SIZE],
}

impl TxPowerLevelAdStruct {
    pub fn new(tx_power_level: impl Into<TxPowerLevel>) -> Self {
        let mut s = Self {
            buffer: Default::default(),
        };
        s.buffer[AD_STRUCT_LENGTH_OFFSET] = (TX_POWER_LEVEL_AD_STRUCT_SIZE - 1) as u8;
        s.buffer[AD_STRUCT_TYPE_OFFSET] = CommonDataType::TxPowerLevel as u8;
        s.buffer[AD_STRUCT_DATA_OFFSET] = tx_power_level.into().0 as u8;
        s
    }
}

impl AdStruct for TxPowerLevelAdStruct {
    fn data(&self) -> &[u8] {
        &self.buffer
    }
    fn r#type(&self) -> AdStructType {
        AdStructType::TX_POWER_LEVEL
    }

    fn unique(&self) -> bool {
        false
    }
}
