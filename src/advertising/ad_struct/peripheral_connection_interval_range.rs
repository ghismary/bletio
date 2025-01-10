use core::ops::Range;

use crate::advertising::ad_struct::{
    AdStruct, AdStructType, AD_STRUCT_DATA_OFFSET, AD_STRUCT_LENGTH_OFFSET, AD_STRUCT_TYPE_OFFSET,
};
use crate::assigned_numbers::AdType;
use crate::connection_interval_value::ConnectionIntervalValue;
use crate::utils::encode_le_u16;

const PERIPHERAL_CONNECTION_INTERVAL_RANGE_AD_STRUCT_SIZE: usize = 6;

#[derive(Debug, Clone, Copy)]
pub struct PeripheralConnectionIntervalRangeAdStruct {
    buffer: [u8; PERIPHERAL_CONNECTION_INTERVAL_RANGE_AD_STRUCT_SIZE],
}

impl PeripheralConnectionIntervalRangeAdStruct {
    pub fn new(range: Range<ConnectionIntervalValue>) -> Self {
        let mut s = Self {
            buffer: Default::default(),
        };
        s.buffer[AD_STRUCT_LENGTH_OFFSET] =
            (PERIPHERAL_CONNECTION_INTERVAL_RANGE_AD_STRUCT_SIZE - 1) as u8;
        s.buffer[AD_STRUCT_TYPE_OFFSET] = AdType::PeripheralConnectionIntervalRange as u8;
        let min = range.start.into();
        let max = range.end.into();
        // INVARIANT: The buffer space is known to be enough.
        encode_le_u16(&mut s.buffer[AD_STRUCT_DATA_OFFSET..], min).unwrap();
        encode_le_u16(&mut s.buffer[AD_STRUCT_DATA_OFFSET + 2..], max).unwrap();
        s
    }
}

impl AdStruct for PeripheralConnectionIntervalRangeAdStruct {
    fn data(&self) -> &[u8] {
        &self.buffer
    }
    fn r#type(&self) -> AdStructType {
        AdStructType::PERIPHERAL_CONNECTION_INTERVAL_RANGE
    }

    fn unique(&self) -> bool {
        false
    }
}
