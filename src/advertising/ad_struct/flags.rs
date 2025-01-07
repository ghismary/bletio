use crate::advertising::ad_struct::{
    AdStruct, AdStructType, AD_STRUCT_DATA_OFFSET, AD_STRUCT_LENGTH_OFFSET, AD_STRUCT_TYPE_OFFSET,
};

use crate::advertising::Flags;
use crate::assigned_numbers::ad_types::AdType;

const FLAGS_AD_STRUCT_SIZE: usize = 3;

#[derive(Debug, Clone, Copy)]
pub struct FlagsAdStruct {
    buffer: [u8; FLAGS_AD_STRUCT_SIZE],
}

impl FlagsAdStruct {
    pub fn new(flags: Flags) -> Self {
        let mut s = Self {
            buffer: Default::default(),
        };
        s.buffer[AD_STRUCT_LENGTH_OFFSET] = (FLAGS_AD_STRUCT_SIZE - 1) as u8;
        s.buffer[AD_STRUCT_TYPE_OFFSET] = AdType::Flags as u8;
        s.buffer[AD_STRUCT_DATA_OFFSET] = flags.bits();
        s
    }
}

impl AdStruct for FlagsAdStruct {
    fn data(&self) -> &[u8] {
        &self.buffer
    }
    fn r#type(&self) -> AdStructType {
        AdStructType::FLAGS
    }

    fn unique(&self) -> bool {
        true
    }
}
