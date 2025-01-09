use crate::advertising::ad_struct::{
    AdStruct, AdStructType, AD_STRUCT_DATA_OFFSET, AD_STRUCT_LENGTH_OFFSET, AD_STRUCT_TYPE_OFFSET,
};
use crate::advertising::advertising_data::ADVERTISING_DATA_MAX_SIZE;
use crate::assigned_numbers::ad_types::AdType;
use crate::utils::encode_le_u16;
use crate::uuid::Uuid16;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub struct ManufacturerSpecificDataAdStruct {
    buffer: [u8; ADVERTISING_DATA_MAX_SIZE],
}

impl ManufacturerSpecificDataAdStruct {
    pub fn try_new(manufacturer_uuid: Uuid16, data: &[u8]) -> Result<Self, Error> {
        let data_size = data.len();
        if (4 + data_size) > ADVERTISING_DATA_MAX_SIZE {
            return Err(Error::BufferTooSmall);
        }
        let mut s = Self {
            buffer: Default::default(),
        };
        s.buffer[AD_STRUCT_LENGTH_OFFSET] = 1 + data_size as u8;
        s.buffer[AD_STRUCT_TYPE_OFFSET] = AdType::ManufacturerSpecificData as u8;
        let mut offset = AD_STRUCT_DATA_OFFSET;
        encode_le_u16(&mut s.buffer[offset..], manufacturer_uuid.0)?;
        offset += 2;
        s.buffer[offset..offset + data_size].copy_from_slice(data);
        Ok(s)
    }
}

impl AdStruct for ManufacturerSpecificDataAdStruct {
    fn data(&self) -> &[u8] {
        &self.buffer
    }
    fn r#type(&self) -> AdStructType {
        AdStructType::MANUFACTURER_SPECIFIC_DATA
    }

    fn unique(&self) -> bool {
        false
    }
}
