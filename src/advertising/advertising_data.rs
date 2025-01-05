use crate::advertising::ad_struct::{AdStruct, AdStructType};
use crate::Error;
use arrayvec::ArrayVec;

pub(crate) const ADVERTISING_DATA_MAX_SIZE: usize = 31;
const ADVERTISING_DATA_SIZE_OFFSET: usize = 0;
const ADVERTISING_DATA_DATA_OFFSET: usize = 1;

#[derive(Debug)]
pub struct AdvertisingData {
    buffer: [u8; ADVERTISING_DATA_MAX_SIZE + 1],
    offset: usize,
    ad_struct_types: ArrayVec<AdStructType, 10>,
}

impl AdvertisingData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn try_add(mut self, ad_struct: impl AdStruct) -> Result<Self, Error> {
        if ad_struct.unique() && self.has_ad_struct_type(ad_struct.r#type()) {
            return Err(Error::AdStructAlreadyPresent);
        }
        let ad_struct_data = ad_struct.data();
        let ad_struct_size = ad_struct_data.len();
        if self.remaining_size() < ad_struct_size {
            return Err(Error::BufferTooSmall);
        }
        self.buffer[self.offset..self.offset + ad_struct_size].copy_from_slice(ad_struct_data);
        self.offset += ad_struct_size;
        self.buffer[ADVERTISING_DATA_SIZE_OFFSET] += ad_struct_size as u8;
        self.add_ad_struct_type(ad_struct.r#type())?;
        Ok(self)
    }

    pub(crate) fn data(&self) -> &[u8] {
        &self.buffer
    }

    fn add_ad_struct_type(&mut self, ad_struct_type: AdStructType) -> Result<(), Error> {
        self.ad_struct_types
            .try_push(ad_struct_type)
            .map_err(|_| Error::BufferTooSmall)
    }

    fn has_ad_struct_type(&self, r#type: AdStructType) -> bool {
        self.ad_struct_types.iter().any(|item| *item == r#type)
    }

    fn total_size(&self) -> usize {
        self.buffer[ADVERTISING_DATA_SIZE_OFFSET] as usize
    }

    fn remaining_size(&self) -> usize {
        ADVERTISING_DATA_MAX_SIZE - self.total_size()
    }
}

impl Default for AdvertisingData {
    fn default() -> Self {
        Self {
            buffer: Default::default(),
            offset: ADVERTISING_DATA_DATA_OFFSET,
            ad_struct_types: Default::default(),
        }
    }
}
