//! Advertising data packets.
//!
//! Normal advertising data and scan response data definitions.
//!
//! # Examples
//!
//! Create an Advertising Data with flags and a 16-bit Service:
//! ```
//! # use bletio::advertising::{AdvertisingData, Flags};
//! # use bletio::advertising::ad_struct::{FlagsAdStruct, ServiceListComplete, ServiceUuid16AdStruct};
//! # use bletio::assigned_numbers::ServiceUuid;
//! # fn main() -> Result<(), bletio::Error> {
//! let adv_data = AdvertisingData::builder()
//!     .with_flags(FlagsAdStruct::new(
//!         Flags::BREDR_NOT_SUPPORTED | Flags::LE_GENERAL_DISCOVERABLE_MODE
//!     ))?
//!     .with_service_uuid16(ServiceUuid16AdStruct::try_new(
//!         [ServiceUuid::Battery, ServiceUuid::EnvironmentalSensing].as_slice(),
//!         ServiceListComplete::Incomplete
//!     )?)?
//!     .build();
//! # Ok(())
//! # }
//! ```
//!
//! Create a Scan Response Data with TX power level and peripheral connection interval range:
//! ```
//! # use bletio::advertising::{Flags, ScanResponseData};
//! # use bletio::advertising::ad_struct::{PeripheralConnectionIntervalRangeAdStruct, TxPowerLevelAdStruct};
//! # fn main() -> Result<(), bletio::Error> {
//! let adv_data = ScanResponseData::builder()
//!     .with_peripheral_connection_interval_range(PeripheralConnectionIntervalRangeAdStruct::new(
//!         (0x0050.try_into()?..=0x0060.try_into()?)
//!     ))?
//!     .with_tx_power_level(TxPowerLevelAdStruct::new(0))?
//!     .build();
//! # Ok(())
//! # }
//! ```

use crate::advertising::ad_struct::{
    AdStruct, AdStructType, FlagsAdStruct, ManufacturerSpecificDataAdStruct,
    PeripheralConnectionIntervalRangeAdStruct, ServiceSolicitationUuid128AdStruct,
    ServiceSolicitationUuid16AdStruct, ServiceSolicitationUuid32AdStruct, ServiceUuid128AdStruct,
    ServiceUuid16AdStruct, ServiceUuid32AdStruct, TxPowerLevelAdStruct,
};
use crate::advertising::AdvertisingError;
use crate::Error;

pub(crate) const ADVERTISING_DATA_MAX_SIZE: usize = 31;
const ADVERTISING_DATA_SIZE_OFFSET: usize = 0;
const ADVERTISING_DATA_DATA_OFFSET: usize = 1;

/// Builder to create `AdvertisingData` packets.
#[derive(Debug, Default)]
pub struct AdvertisingDataBuilder {
    obj: AdvertisingData,
    present_ad_structs: AdStructType,
}

impl AdvertisingDataBuilder {
    /// Create a builder to instantiate `AdvertisingData`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the `AdvertisingData`, containing all the Advertising Structures that has been added.
    pub fn build(self) -> AdvertisingData {
        self.obj
    }

    /// Add a Flags Advertising Structure to the `AdvertisingData`.
    pub fn with_flags(mut self, flags: FlagsAdStruct) -> Result<Self, Error> {
        self.try_add(flags)?;
        Ok(self)
    }

    /// Add a Manufacturer Specific Data Advertising Structure to the `AdvertisingData`.
    pub fn with_manufacturer_specific_data(
        mut self,
        data: ManufacturerSpecificDataAdStruct,
    ) -> Result<Self, Error> {
        self.try_add(data)?;
        Ok(self)
    }

    /// Add a Peripheral Connection Interval Range Advertising Structure to the `AdvertisingData`.
    pub fn with_peripheral_connection_interval_range(
        mut self,
        range: PeripheralConnectionIntervalRangeAdStruct,
    ) -> Result<Self, Error> {
        self.try_add(range)?;
        Ok(self)
    }

    /// Add a list of 16-bit Service Solicitation UUIDs Advertising Structure to the `AdvertisingData`.
    pub fn with_service_solicitation_uuid16(
        mut self,
        service_uuid16: ServiceSolicitationUuid16AdStruct,
    ) -> Result<Self, Error> {
        self.try_add(service_uuid16)?;
        Ok(self)
    }

    /// Add a list of 32-bit Service Solicitation UUIDs Advertising Structure to the `AdvertisingData`.
    pub fn with_service_solicitation_uuid32(
        mut self,
        service_uuid32: ServiceSolicitationUuid32AdStruct,
    ) -> Result<Self, Error> {
        self.try_add(service_uuid32)?;
        Ok(self)
    }

    /// Add a list of 128-bit Service Solicitation UUIDs Advertising Structure to the `AdvertisingData`.
    pub fn with_service_solicitation_uuid128(
        mut self,
        service_uuid128: ServiceSolicitationUuid128AdStruct,
    ) -> Result<Self, Error> {
        self.try_add(service_uuid128)?;
        Ok(self)
    }

    /// Add a list of 16-bit Service UUIDs Advertising Structure to the `AdvertisingData`.
    pub fn with_service_uuid16(
        mut self,
        service_uuid16: ServiceUuid16AdStruct,
    ) -> Result<Self, Error> {
        self.try_add(service_uuid16)?;
        Ok(self)
    }

    /// Add a list of 32-bit Service UUIDs Advertising Structure to the `AdvertisingData`.
    pub fn with_service_uuid32(
        mut self,
        service_uuid32: ServiceUuid32AdStruct,
    ) -> Result<Self, Error> {
        self.try_add(service_uuid32)?;
        Ok(self)
    }

    /// Add a list of 128-bit Service UUIDs Advertising Structure to the `AdvertisingData`.
    pub fn with_service_uuid128(
        mut self,
        service_uuid128: ServiceUuid128AdStruct,
    ) -> Result<Self, Error> {
        self.try_add(service_uuid128)?;
        Ok(self)
    }

    /// Add a TX Power Level Advertising Structure to the `AdvertisingData`.
    pub fn with_tx_power_level(
        mut self,
        tx_power_level: TxPowerLevelAdStruct,
    ) -> Result<Self, Error> {
        self.try_add(tx_power_level)?;
        Ok(self)
    }

    fn try_add(&mut self, ad_struct: impl AdStruct) -> Result<(), Error> {
        let ad_struct_type = ad_struct.r#type();
        if ad_struct.is_unique() && self.present_ad_structs.contains(ad_struct_type) {
            return Err(AdvertisingError::AdStructAlreadyPresent)?;
        }
        self.obj.try_add(ad_struct)?;
        self.present_ad_structs |= ad_struct_type;
        Ok(())
    }
}

/// Advertising Data sent when advertising.
///
/// The packet format for the Advertising Data is defined in
/// [Core Specification 6.0, Vol.3, Part C, 11](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-51247611-bdce-274e-095c-afb6d879c55c).
///
/// It may contain some Advertising Structures, whose formats are specified in
/// [Supplement to the Bluetooth Core Specification, Part A, 1](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-36b7e551-d4cf-9ae3-a8ee-0482fbc1d5bc).
///
/// Use the [`AdvertisingDataBuilder`] to instantiate it.
#[derive(Debug)]
pub struct AdvertisingData {
    buffer: [u8; ADVERTISING_DATA_MAX_SIZE + 1],
    offset: usize,
}

impl AdvertisingData {
    /// Instantiate a builder to create Advertising Data.
    pub fn builder() -> AdvertisingDataBuilder {
        AdvertisingDataBuilder::new()
    }

    fn try_add(&mut self, ad_struct: impl AdStruct) -> Result<(), Error> {
        let ad_struct_data = ad_struct.encoded_data();
        let ad_struct_size = ad_struct_data.len();
        if self.remaining_size() < ad_struct_size {
            return Err(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?;
        }
        self.buffer[self.offset..self.offset + ad_struct_size].copy_from_slice(ad_struct_data);
        self.offset += ad_struct_size;
        self.buffer[ADVERTISING_DATA_SIZE_OFFSET] += ad_struct_size as u8;
        Ok(())
    }

    pub(crate) fn encoded_data(&self) -> &[u8] {
        &self.buffer
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
        }
    }
}

/// Builder to create `ScanResponseData` packets.
#[derive(Debug, Default)]
pub struct ScanResponseDataBuilder {
    obj: ScanResponseData,
    present_ad_structs: AdStructType,
}

impl ScanResponseDataBuilder {
    /// Create a builder to instantiate `ScanResponseData`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the `ScanResponseData`, containing all the Advertising Structures that has been added.
    pub fn build(self) -> ScanResponseData {
        self.obj
    }

    /// Add a Manufacturer Specific Data Advertising Structure to the `ScanResponseData`.
    pub fn with_manufacturer_specific_data(
        mut self,
        data: ManufacturerSpecificDataAdStruct,
    ) -> Result<Self, Error> {
        self.try_add(data)?;
        Ok(self)
    }

    /// Add a Peripheral Connection Interval Range Advertising Structure to the `ScanResponseData`.
    pub fn with_peripheral_connection_interval_range(
        mut self,
        range: PeripheralConnectionIntervalRangeAdStruct,
    ) -> Result<Self, Error> {
        self.try_add(range)?;
        Ok(self)
    }

    /// Add a list of 16-bit Service Solicitation UUIDs Advertising Structure to the `ScanResponseData`.
    pub fn with_service_solicitation_uuid16(
        mut self,
        service_uuid16: ServiceSolicitationUuid16AdStruct,
    ) -> Result<Self, Error> {
        self.try_add(service_uuid16)?;
        Ok(self)
    }

    /// Add a list of 32-bit Service Solicitation UUIDs Advertising Structure to the `ScanResponseData`.
    pub fn with_service_solicitation_uuid32(
        mut self,
        service_uuid32: ServiceSolicitationUuid32AdStruct,
    ) -> Result<Self, Error> {
        self.try_add(service_uuid32)?;
        Ok(self)
    }

    /// Add a list of 128-bit Service Solicitation UUIDs Advertising Structure to the `ScanResponseData`.
    pub fn with_service_solicitation_uuid128(
        mut self,
        service_uuid128: ServiceSolicitationUuid128AdStruct,
    ) -> Result<Self, Error> {
        self.try_add(service_uuid128)?;
        Ok(self)
    }

    /// Add a list of 16-bit Service UUIDs Advertising Structure to the `ScanResponseData`.
    pub fn with_service_uuid16(
        mut self,
        service_uuid16: ServiceUuid16AdStruct,
    ) -> Result<Self, Error> {
        self.try_add(service_uuid16)?;
        Ok(self)
    }

    /// Add a list of 32-bit Service UUIDs Advertising Structure to the `ScanResponseData`.
    pub fn with_service_uuid32(
        mut self,
        service_uuid32: ServiceUuid32AdStruct,
    ) -> Result<Self, Error> {
        self.try_add(service_uuid32)?;
        Ok(self)
    }

    /// Add a list of 128-bit Service UUIDs Advertising Structure to the `ScanResponseData`.
    pub fn with_service_uuid128(
        mut self,
        service_uuid128: ServiceUuid128AdStruct,
    ) -> Result<Self, Error> {
        self.try_add(service_uuid128)?;
        Ok(self)
    }

    /// Add a TX Power Level Advertising Structure to the `ScanResponseData`.
    pub fn with_tx_power_level(
        mut self,
        tx_power_level: TxPowerLevelAdStruct,
    ) -> Result<Self, Error> {
        self.try_add(tx_power_level)?;
        Ok(self)
    }

    fn try_add(&mut self, ad_struct: impl AdStruct) -> Result<(), Error> {
        let ad_struct_type = ad_struct.r#type();
        if ad_struct.is_unique() && self.present_ad_structs.contains(ad_struct_type) {
            return Err(AdvertisingError::AdStructAlreadyPresent)?;
        }
        self.obj.try_add(ad_struct)?;
        self.present_ad_structs |= ad_struct_type;
        Ok(())
    }
}

/// Scan Response Data that can be sent when the advertising is scannable.
///
/// The packet format for the Scan Response Data is defined in
/// [Core Specification 6.0, Vol.3, Part C, 11](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-51247611-bdce-274e-095c-afb6d879c55c).
///
/// It may contain some Advertising Structures, whose formats are specified in
/// [Supplement to the Bluetooth Core Specification, Part A, 1](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-36b7e551-d4cf-9ae3-a8ee-0482fbc1d5bc).
///
/// Use the [`ScanResponseDataBuilder`] to instantiate it.
#[derive(Debug)]
pub struct ScanResponseData {
    buffer: [u8; ADVERTISING_DATA_MAX_SIZE + 1],
    offset: usize,
}

impl ScanResponseData {
    /// Instantiate a builder to create Scan Response Data.
    pub fn builder() -> ScanResponseDataBuilder {
        ScanResponseDataBuilder::new()
    }

    fn try_add(&mut self, ad_struct: impl AdStruct) -> Result<(), Error> {
        let ad_struct_data = ad_struct.encoded_data();
        let ad_struct_size = ad_struct_data.len();
        if self.remaining_size() < ad_struct_size {
            return Err(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?;
        }
        self.buffer[self.offset..self.offset + ad_struct_size].copy_from_slice(ad_struct_data);
        self.offset += ad_struct_size;
        self.buffer[ADVERTISING_DATA_SIZE_OFFSET] += ad_struct_size as u8;
        Ok(())
    }

    pub(crate) fn encoded_data(&self) -> &[u8] {
        &self.buffer
    }

    fn total_size(&self) -> usize {
        self.buffer[ADVERTISING_DATA_SIZE_OFFSET] as usize
    }

    fn remaining_size(&self) -> usize {
        ADVERTISING_DATA_MAX_SIZE - self.total_size()
    }
}

impl Default for ScanResponseData {
    fn default() -> Self {
        Self {
            buffer: Default::default(),
            offset: ADVERTISING_DATA_DATA_OFFSET,
        }
    }
}
