//! Advertising structures contained in [AdvertisingData](crate::advertising::AdvertisingData)
//! and [ScanResponseData](crate::advertising::ScanResponseData).
//!
//! The format for the [AdvertisingData](crate::advertising::AdvertisingData) and
//! [ScanResponseData](crate::advertising::ScanResponseData) is defined in the
//! [Core Specification 6.0, Vol.3, Part C, 11](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-51247611-bdce-274e-095c-afb6d879c55c).
//!
//! The formats of each advertising structures and their meanings are defined in the
//! [Core Specification Supplement, Part A, 1](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-36b7e551-d4cf-9ae3-a8ee-0482fbc1d5bc).

use bitflags::bitflags;

mod advertising_interval;
mod appearance;
pub(crate) mod flags;
mod manufacturer_specific_data;
mod peripheral_connection_interval_range;
mod service_solicitation;
mod service_uuid;
pub(crate) mod tx_power_level;

use crate::advertising::AdvertisingError;
use crate::assigned_numbers::AdType;
use crate::utils::Buffer;
pub use advertising_interval::AdvertisingIntervalAdStruct;
pub use appearance::AppearanceAdStruct;
pub use flags::FlagsAdStruct;
pub use manufacturer_specific_data::ManufacturerSpecificDataAdStruct;
pub use peripheral_connection_interval_range::PeripheralConnectionIntervalRangeAdStruct;
pub use service_solicitation::{
    ServiceSolicitationUuid128AdStruct, ServiceSolicitationUuid16AdStruct,
    ServiceSolicitationUuid32AdStruct,
};
pub use service_uuid::{
    ServiceListComplete, ServiceUuid128AdStruct, ServiceUuid16AdStruct, ServiceUuid32AdStruct,
};
pub use tx_power_level::TxPowerLevelAdStruct;

pub(crate) trait AdStruct {
    fn encoded_data(&self) -> &[u8];
    fn r#type(&self) -> AdStructType;
    fn is_unique(&self) -> bool;
}

#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct AdStructType(u16);

bitflags! {
    impl AdStructType: u16 {
        const SERVICE_UUID16 = 1 << 0;
        const SERVICE_UUID32 = 1 << 1;
        const SERVICE_UUID128 = 1 << 2;
        const FLAGS = 1 << 3;
        const MANUFACTURER_SPECIFIC_DATA = 1 << 4;
        const TX_POWER_LEVEL = 1 << 5;
        const PERIPHERAL_CONNECTION_INTERVAL_RANGE = 1 << 6;
        const SERVICE_SOLICITATION_UUID16 = 1 << 7;
        const SERVICE_SOLICITATION_UUID32 = 1 << 8;
        const SERVICE_SOLICITATION_UUID128 = 1 << 9;
        const APPEARANCE = 1 << 10;
        const ADVERTISING_INTERVAL = 1 << 11;
    }
}

const AD_STRUCT_LENGTH_OFFSET: usize = 0;
const AD_STRUCT_TYPE_OFFSET: usize = 1;
const AD_STRUCT_DATA_OFFSET: usize = 2;

#[derive(Debug, Clone)]
pub(crate) struct AdStructBuffer<const CAP: usize> {
    buffer: Buffer<CAP>,
}

impl<const CAP: usize> AdStructBuffer<CAP> {
    pub(crate) fn new(ad_type: AdType) -> Self {
        let mut s = Self::default();
        s.set_ad_type(ad_type);
        s
    }

    pub(crate) fn copy_from_slice(&mut self, data: &[u8]) -> Result<(), AdvertisingError> {
        self.buffer
            .copy_from_slice(data)
            .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?;
        self.buffer.data[AD_STRUCT_LENGTH_OFFSET] += data.len() as u8;
        Ok(())
    }

    pub(crate) fn data(&self) -> &[u8] {
        self.buffer.data()
    }

    pub(crate) fn encode_le_u16(&mut self, data: u16) -> Result<(), AdvertisingError> {
        self.buffer
            .encode_le_u16(data)
            .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?;
        self.buffer.data[AD_STRUCT_LENGTH_OFFSET] += 2;
        Ok(())
    }

    pub(crate) fn encode_le_u32(&mut self, data: u32) -> Result<(), AdvertisingError> {
        self.buffer
            .encode_le_u32(data)
            .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?;
        self.buffer.data[AD_STRUCT_LENGTH_OFFSET] += 4;
        Ok(())
    }

    pub(crate) fn encode_le_u128(&mut self, data: u128) -> Result<(), AdvertisingError> {
        self.buffer
            .encode_le_u128(data)
            .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?;
        self.buffer.data[AD_STRUCT_LENGTH_OFFSET] += 16;
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.buffer.offset - AD_STRUCT_DATA_OFFSET
    }

    pub(crate) fn try_push(&mut self, data: u8) -> Result<(), AdvertisingError> {
        self.buffer
            .try_push(data)
            .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?;
        self.buffer.data[AD_STRUCT_LENGTH_OFFSET] += 1;
        Ok(())
    }

    #[cfg(test)]
    // TODO: Return an AdType
    fn ad_type(&self) -> u8 {
        self.buffer.data[AD_STRUCT_TYPE_OFFSET]
    }

    fn set_ad_type(&mut self, ad_type: AdType) {
        self.buffer.data[AD_STRUCT_TYPE_OFFSET] = ad_type as u8;
    }
}

impl<const CAP: usize> Default for AdStructBuffer<CAP> {
    fn default() -> Self {
        let mut buffer = Buffer {
            data: [0; CAP],
            offset: AD_STRUCT_DATA_OFFSET,
        };
        buffer.data[AD_STRUCT_LENGTH_OFFSET] = 1;
        Self { buffer }
    }
}
