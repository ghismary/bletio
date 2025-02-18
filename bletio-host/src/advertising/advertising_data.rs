use core::ops::RangeInclusive;

use bletio_hci::{
    AdvertisingInterval, ConnectionInterval, PublicDeviceAddress, RandomAddress,
    SupportedLeFeatures, TxPowerLevel,
};
use bletio_utils::{BufferOps, EncodeToBuffer};

use crate::advertising::ad_struct::{
    AdvertisingIntervalAdStruct, AppearanceAdStruct, FlagsAdStruct, LeSupportedFeaturesAdStruct,
    LocalNameAdStruct, ManufacturerSpecificDataAdStruct, PeripheralConnectionIntervalRangeAdStruct,
    PublicTargetAddressAdStruct, RandomTargetAddressAdStruct, ServiceDataUuid128AdStruct,
    ServiceDataUuid16AdStruct, ServiceDataUuid32AdStruct, ServiceSolicitationUuid128AdStruct,
    ServiceSolicitationUuid16AdStruct, ServiceSolicitationUuid32AdStruct, ServiceUuid128AdStruct,
    ServiceUuid16AdStruct, ServiceUuid32AdStruct, TxPowerLevelAdStruct, UriAdStruct,
};
use crate::advertising::{AdvertisingError, Flags, LocalNameComplete, ServiceListComplete, Uri};
use crate::assigned_numbers::{AppearanceValue, CompanyIdentifier, ServiceUuid};
use crate::uuid::{Uuid128, Uuid32};
use crate::{DeviceInformation, Error};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FullAdvertisingData {
    pub(crate) adv_data: AdvertisingData,
    pub(crate) scanresp_data: Option<ScanResponseData>,
}

impl FullAdvertisingData {
    pub fn try_new(
        adv_data: AdvertisingData,
        scanresp_data: impl Into<Option<ScanResponseData>>,
    ) -> Result<Self, Error> {
        let scanresp_data = scanresp_data.into();
        // if let Some(scanresp_data) = &scanresp_data {
        //     if let (Some(_), Some(_)) = (
        //         adv_data.base.appearance.as_ref(),
        //         scanresp_data.base.appearance.as_ref(),
        //     ) {
        //         return Err(
        //             AdvertisingError::AppearanceNotAllowedInBothAdvertisingDataAndScanResponseData,
        //         )?;
        //     }
        // }
        Ok(Self {
            adv_data,
            scanresp_data,
        })
    }

    pub(crate) fn fill_automatic_data(&self, device_information: &DeviceInformation) -> Self {
        let mut filled = self.clone();
        // filled.adv_data.base.fill_automatic_data(device_information);
        // if let Some(scanresp_data) = filled.scanresp_data.as_mut() {
        //     scanresp_data.base.fill_automatic_data(device_information);
        // }
        filled
    }
}

/// Builder to create `AdvertisingData` packets.
#[derive(Debug, Default)]
pub struct AdvertisingDataBuilder {
    data: AdvertisingData,
}

impl AdvertisingDataBuilder {
    /// Create a builder to instantiate `AdvertisingData`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the `AdvertisingData`, containing all the Advertising Structures that has been added.
    pub fn build(self) -> AdvertisingData {
        self.data
    }

    /// Add an Advertising Interval Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `interval` — The Advertising Interval value to put in the added Advertising Interval Advertising Structure.
    pub fn with_advertising_interval(
        self,
        interval: AdvertisingInterval,
    ) -> Result<Self, AdvertisingError> {
        // TODO: Check that it is not already present
        self.add_ad_struct(AdvertisingIntervalAdStruct::new(interval))
    }

    /// Add an Appearance Advertising Structure to the `AdvertisingData`.
    pub fn with_appearance(self) -> Result<Self, AdvertisingError> {
        // TODO: Check that it is not already present
        self.add_ad_struct(AppearanceAdStruct::new(AppearanceValue::GenericUnknown))
    }

    /// Add a Flags Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `flags` — The Flags value to put in the added Flags Advertising Structure.
    pub fn with_flags(self, flags: Flags) -> Result<Self, AdvertisingError> {
        // TODO: Check that it is not already present
        self.add_ad_struct(FlagsAdStruct::new(flags))
    }

    /// Add a LE Supported Features Advertising Structure to the `AdvertisingData`.
    pub fn with_le_supported_features(self) -> Result<Self, AdvertisingError> {
        // TODO: Check that it is not already present
        self.add_ad_struct(LeSupportedFeaturesAdStruct::new(
            SupportedLeFeatures::default(),
        ))
    }

    /// Add a Local Name Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `complete` — Whether the local name should be put complete or shortened in the added Local Name Advertising Structure.
    pub fn with_local_name(self, complete: LocalNameComplete) -> Result<Self, AdvertisingError> {
        // TODO: Check that it is not already present
        self.add_ad_struct(LocalNameAdStruct::try_new("", complete)?)
    }

    /// Add a Manufacturer Specific Data Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `manufacturer` — The `CompanyIdentifier` to put in the added Manufacturer Specific Data Advertising Structure.
    /// * `data` — The data to put in the added Manufacturer Specific Data Advertising Structure.
    pub fn with_manufacturer_specific_data(
        self,
        manufacturer: CompanyIdentifier,
        data: &[u8],
    ) -> Result<Self, AdvertisingError> {
        self.add_ad_struct(ManufacturerSpecificDataAdStruct::try_new(
            manufacturer,
            data,
        )?)
    }

    /// Add a Peripheral Connection Interval Range Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `range` — The Connection Interval value to put in the added Peripheral Connection Range Advertising Structure.
    pub fn with_peripheral_connection_interval_range(
        self,
        range: RangeInclusive<ConnectionInterval>,
    ) -> Result<Self, AdvertisingError> {
        self.add_ad_struct(PeripheralConnectionIntervalRangeAdStruct::new(range))
    }

    /// Add a Public Target Address Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `addresses` — The list of public device addresses to put in the added Public Target Address Advertising Structure.
    pub fn with_public_target_address(
        self,
        addresses: &[PublicDeviceAddress],
    ) -> Result<Self, AdvertisingError> {
        // TODO: Check that it is not already present
        self.add_ad_struct(PublicTargetAddressAdStruct::try_new(addresses)?)
    }

    /// Add a Random Target Address Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `addresses` — The list of random addresses to put in the added Random Target Address Advertising Structure.
    pub fn with_random_target_address(
        self,
        addresses: &[RandomAddress],
    ) -> Result<Self, AdvertisingError> {
        // TODO: Check that it is not already present
        self.add_ad_struct(RandomTargetAddressAdStruct::try_new(addresses)?)
    }

    /// Add a Service Data for a 16-bit Service UUID Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `uuid` — The 16-bit Service UUID to put in the added Service Data Advertising Structure.
    /// * `data` — The data to put in the added Service Data Advertising Structure.
    pub fn with_service_data_uuid16(
        self,
        uuid: ServiceUuid,
        data: &[u8],
    ) -> Result<Self, AdvertisingError> {
        self.add_ad_struct(ServiceDataUuid16AdStruct::try_new(uuid, data)?)
    }

    /// Add a Service Data for a 32-bit Service UUID Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `uuid` — The 32-bit Service UUID to put in the added Service Data Advertising Structure.
    /// * `data` — The data to put in the added Service Data Advertising Structure.
    pub fn with_service_data_uuid32(
        self,
        uuid: Uuid32,
        data: &[u8],
    ) -> Result<Self, AdvertisingError> {
        self.add_ad_struct(ServiceDataUuid32AdStruct::try_new(uuid, data)?)
    }

    /// Add a Service Data for a 128-bit Service UUID Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `uuid` — The 128-bit Service UUID to put in the added Service Data Advertising Structure.
    /// * `data` — The data to put in the added Service Data Advertising Structure.
    pub fn with_service_data_uuid128(
        self,
        uuid: Uuid128,
        data: &[u8],
    ) -> Result<Self, AdvertisingError> {
        self.add_ad_struct(ServiceDataUuid128AdStruct::try_new(uuid, data)?)
    }

    /// Add a list of 16-bit Service Solicitation UUIDs Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `uuids` — The list of 16-bit Service UUIDs to put in the added Service Solicitation UUID16 Advertising Structure.
    pub fn with_service_solicitation_uuid16(
        self,
        uuids: &[ServiceUuid],
    ) -> Result<Self, AdvertisingError> {
        self.add_ad_struct(ServiceSolicitationUuid16AdStruct::try_new(uuids)?)
    }

    /// Add a list of 32-bit Service Solicitation UUIDs Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `uuids` — The list of 32-bit Service UUIDs to put in the added Service Solicitation UUID32 Advertising Structure.
    pub fn with_service_solicitation_uuid32(
        self,
        uuids: &[Uuid32],
    ) -> Result<Self, AdvertisingError> {
        self.add_ad_struct(ServiceSolicitationUuid32AdStruct::try_new(uuids)?)
    }

    /// Add a list of 128-bit Service Solicitation UUIDs Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `uuids` — The list of 128-bit Service UUIDs to put in the added Service Solicitation UUID128 Advertising Structure.
    pub fn with_service_solicitation_uuid128(
        self,
        uuids: &[Uuid128],
    ) -> Result<Self, AdvertisingError> {
        self.add_ad_struct(ServiceSolicitationUuid128AdStruct::try_new(uuids)?)
    }

    /// Add a list of 16-bit Service UUIDs Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `uuids` — The list of 16-bit Service UUIDs to put in the added Service UUID16 Advertising Structure.
    /// * `complete` — Whether the provided list is complete or not.
    pub fn with_service_uuid16(
        self,
        uuids: &[ServiceUuid],
        complete: ServiceListComplete,
    ) -> Result<Self, AdvertisingError> {
        self.add_ad_struct(ServiceUuid16AdStruct::try_new(uuids, complete)?)
    }

    /// Add a list of 32-bit Service UUIDs Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `uuids` — The list of 32-bit Service UUIDs to put in the added Service UUID32 Advertising Structure.
    /// * `complete` — Whether the provided list is complete or not.
    pub fn with_service_uuid32(
        self,
        uuids: &[Uuid32],
        complete: ServiceListComplete,
    ) -> Result<Self, AdvertisingError> {
        self.add_ad_struct(ServiceUuid32AdStruct::try_new(uuids, complete)?)
    }

    /// Add a list of 128-bit Service UUIDs Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `uuids` — The list of 128-bit Service UUIDs to put in the added Service UUID128 Advertising Structure.
    /// * `complete` — Whether the provided list is complete or not.
    pub fn with_service_uuid128(
        self,
        uuids: &[Uuid128],
        complete: ServiceListComplete,
    ) -> Result<Self, AdvertisingError> {
        self.add_ad_struct(ServiceUuid128AdStruct::try_new(uuids, complete)?)
    }

    /// Add a TX Power Level Advertising Structure to the `AdvertisingData`.
    pub fn with_tx_power_level(self) -> Result<Self, AdvertisingError> {
        self.add_ad_struct(TxPowerLevelAdStruct::new(TxPowerLevel::default()))
    }

    /// Add a Uri Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `uri` — The Uri to put in the added Uri Advertising Structure.
    pub fn with_uri(self, uri: Uri) -> Result<Self, AdvertisingError> {
        self.add_ad_struct(UriAdStruct::new(uri))
    }

    fn add_ad_struct(mut self, ad_struct: impl EncodeToBuffer) -> Result<Self, AdvertisingError> {
        self.data
            .data
            .fill(|b| ad_struct.encode(b))
            .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?;
        Ok(self)
    }
}

// impl<'a> AdvertisingDataBase<'a> {
//     fn fill_automatic_data(&mut self, device_information: &'a DeviceInformation) {
//         if self.appearance.is_some() {
//             self.appearance = Some(AppearanceAdStruct::new(device_information.appearance));
//         }
//         if self.le_supported_features.is_some() {
//             self.le_supported_features = Some(LeSupportedFeaturesAdStruct::new(
//                 device_information.supported_le_features,
//             ));
//         }
//         if let Some(local_name) = self.local_name.clone() {
//             self.local_name = Some(LocalNameAdStruct::new(
//                 device_information.local_name,
//                 local_name.complete,
//             ));
//         }
//         if self.tx_power_level.is_some() {
//             self.tx_power_level =
//                 Some(TxPowerLevelAdStruct::new(device_information.tx_power_level));
//         }
//     }
// }

/// Advertising Data sent when advertising.
///
/// The packet format for the Advertising Data is defined in
/// [Core Specification 6.0, Vol.3, Part C, 11](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-51247611-bdce-274e-095c-afb6d879c55c).
///
/// It may contain some Advertising Structures, whose formats are specified in
/// [Supplement to the Bluetooth Core Specification, Part A, 1](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-36b7e551-d4cf-9ae3-a8ee-0482fbc1d5bc).
///
/// Use the [`AdvertisingDataBuilder`] to instantiate it.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AdvertisingData {
    data: bletio_hci::AdvertisingData,
}

impl AdvertisingData {
    /// Instantiate a builder to create Advertising Data.
    pub fn builder() -> AdvertisingDataBuilder {
        AdvertisingDataBuilder::new()
    }
}

impl EncodeToBuffer for AdvertisingData {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, bletio_utils::Error> {
        self.data.encode(buffer)
    }

    fn encoded_size(&self) -> usize {
        self.data.encoded_size()
    }
}

impl From<&AdvertisingData> for bletio_hci::AdvertisingData {
    fn from(value: &AdvertisingData) -> Self {
        value.data.clone()
    }
}

/// Builder to create `ScanResponseData` packets.
#[derive(Debug, Default)]
pub struct ScanResponseDataBuilder {
    data: ScanResponseData,
}

impl ScanResponseDataBuilder {
    /// Create a builder to instantiate `ScanResponseData`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the `ScanResponseData`, containing all the Advertising Structures that has been added.
    pub fn build(self) -> ScanResponseData {
        self.data
    }

    /// Add an Advertising Interval Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `interval` — The Advertising Interval value to put in the added Advertising Interval Advertising Structure.
    pub fn with_advertising_interval(
        self,
        interval: AdvertisingInterval,
    ) -> Result<Self, AdvertisingError> {
        // TODO: Check that it is not already present
        self.add_ad_struct(AdvertisingIntervalAdStruct::new(interval))
    }

    /// Add an Appearance Advertising Structure to the `ScanResponseData`.
    pub fn with_appearance(self) -> Result<Self, AdvertisingError> {
        // TODO: Check that it is not already present
        self.add_ad_struct(AppearanceAdStruct::new(AppearanceValue::GenericUnknown))
    }

    /// Add a LE Supported Features Advertising Structure to the `ScanResponseData`.
    pub fn with_le_supported_features(self) -> Result<Self, AdvertisingError> {
        // TODO: Check that it is not already present
        self.add_ad_struct(LeSupportedFeaturesAdStruct::new(
            SupportedLeFeatures::default(),
        ))
    }

    /// Add a Local Name Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `complete` — Whether the local name should be put complete or shortened in the added Local Name Advertising Structure.
    pub fn with_local_name(self, complete: LocalNameComplete) -> Result<Self, AdvertisingError> {
        // TODO: Check that it is not already present
        self.add_ad_struct(LocalNameAdStruct::try_new("", complete)?)
    }

    /// Add a Manufacturer Specific Data Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `manufacturer` — The `CompanyIdentifier` to put in the added Manufacturer Specific Data Advertising Structure.
    /// * `data` — The data to put in the added Manufacturer Specific Data Advertising Structure.
    pub fn with_manufacturer_specific_data(
        self,
        manufacturer: CompanyIdentifier,
        data: &[u8],
    ) -> Result<Self, AdvertisingError> {
        self.add_ad_struct(ManufacturerSpecificDataAdStruct::try_new(
            manufacturer,
            data,
        )?)
    }

    /// Add a Peripheral Connection Interval Range Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `range` — The Connection Interval value to put in the added Peripheral Connection Range Advertising Structure.
    pub fn with_peripheral_connection_interval_range(
        self,
        range: RangeInclusive<ConnectionInterval>,
    ) -> Result<Self, AdvertisingError> {
        self.add_ad_struct(PeripheralConnectionIntervalRangeAdStruct::new(range))
    }

    /// Add a Public Target Address Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `addresses` — The list of public device addresses to put in the added Public Target Address Advertising Structure.
    pub fn with_public_target_address(
        self,
        addresses: &[PublicDeviceAddress],
    ) -> Result<Self, AdvertisingError> {
        // TODO: Check that it is not already present
        self.add_ad_struct(PublicTargetAddressAdStruct::try_new(addresses)?)
    }

    /// Add a Random Target Address Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `addresses` — The list of random addresses to put in the added Random Target Address Advertising Structure.
    pub fn with_random_target_address(
        self,
        addresses: &[RandomAddress],
    ) -> Result<Self, AdvertisingError> {
        // TODO: Check that it is not already present
        self.add_ad_struct(RandomTargetAddressAdStruct::try_new(addresses)?)
    }

    /// Add a Service Data for a 16-bit Service UUID Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `uuid` — The 16-bit Service UUID to put in the added Service Data Advertising Structure.
    /// * `data` — The data to put in the added Service Data Advertising Structure.
    pub fn with_service_data_uuid16(
        self,
        uuid: ServiceUuid,
        data: &[u8],
    ) -> Result<Self, AdvertisingError> {
        self.add_ad_struct(ServiceDataUuid16AdStruct::try_new(uuid, data)?)
    }

    /// Add a Service Data for a 32-bit Service UUID Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `uuid` — The 32-bit Service UUID to put in the added Service Data Advertising Structure.
    /// * `data` — The data to put in the added Service Data Advertising Structure.
    pub fn with_service_data_uuid32(
        self,
        uuid: Uuid32,
        data: &[u8],
    ) -> Result<Self, AdvertisingError> {
        self.add_ad_struct(ServiceDataUuid32AdStruct::try_new(uuid, data)?)
    }

    /// Add a Service Data for a 128-bit Service UUID Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `uuid` — The 128-bit Service UUID to put in the added Service Data Advertising Structure.
    /// * `data` — The data to put in the added Service Data Advertising Structure.
    pub fn with_service_data_uuid128(
        self,
        uuid: Uuid128,
        data: &[u8],
    ) -> Result<Self, AdvertisingError> {
        self.add_ad_struct(ServiceDataUuid128AdStruct::try_new(uuid, data)?)
    }

    /// Add a list of 16-bit Service Solicitation UUIDs Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `uuids` — The list of 16-bit Service UUIDs to put in the added Service Solicitation UUID16 Advertising Structure.
    pub fn with_service_solicitation_uuid16(
        self,
        uuids: &[ServiceUuid],
    ) -> Result<Self, AdvertisingError> {
        self.add_ad_struct(ServiceSolicitationUuid16AdStruct::try_new(uuids)?)
    }

    /// Add a list of 32-bit Service Solicitation UUIDs Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `uuids` — The list of 32-bit Service UUIDs to put in the added Service Solicitation UUID32 Advertising Structure.
    pub fn with_service_solicitation_uuid32(
        self,
        uuids: &[Uuid32],
    ) -> Result<Self, AdvertisingError> {
        self.add_ad_struct(ServiceSolicitationUuid32AdStruct::try_new(uuids)?)
    }

    /// Add a list of 128-bit Service Solicitation UUIDs Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `uuids` — The list of 128-bit Service UUIDs to put in the added Service Solicitation UUID128 Advertising Structure.
    pub fn with_service_solicitation_uuid128(
        self,
        uuids: &[Uuid128],
    ) -> Result<Self, AdvertisingError> {
        self.add_ad_struct(ServiceSolicitationUuid128AdStruct::try_new(uuids)?)
    }

    /// Add a list of 16-bit Service UUIDs Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `uuids` — The list of 16-bit Service UUIDs to put in the added Service UUID16 Advertising Structure.
    /// * `complete` — Whether the provided list is complete or not.
    pub fn with_service_uuid16(
        self,
        uuids: &[ServiceUuid],
        complete: ServiceListComplete,
    ) -> Result<Self, AdvertisingError> {
        self.add_ad_struct(ServiceUuid16AdStruct::try_new(uuids, complete)?)
    }

    /// Add a list of 32-bit Service UUIDs Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `uuids` — The list of 32-bit Service UUIDs to put in the added Service UUID32 Advertising Structure.
    /// * `complete` — Whether the provided list is complete or not.
    pub fn with_service_uuid32(
        self,
        uuids: &[Uuid32],
        complete: ServiceListComplete,
    ) -> Result<Self, AdvertisingError> {
        self.add_ad_struct(ServiceUuid32AdStruct::try_new(uuids, complete)?)
    }

    /// Add a list of 128-bit Service UUIDs Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `uuids` — The list of 128-bit Service UUIDs to put in the added Service UUID128 Advertising Structure.
    /// * `complete` — Whether the provided list is complete or not.
    pub fn with_service_uuid128(
        self,
        uuids: &[Uuid128],
        complete: ServiceListComplete,
    ) -> Result<Self, AdvertisingError> {
        self.add_ad_struct(ServiceUuid128AdStruct::try_new(uuids, complete)?)
    }

    /// Add a TX Power Level Advertising Structure to the `ScanResponseData`.
    pub fn with_tx_power_level(self) -> Result<Self, AdvertisingError> {
        self.add_ad_struct(TxPowerLevelAdStruct::new(TxPowerLevel::default()))
    }

    /// Add a Uri Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `uri` — The Uri to put in the added Uri Advertising Structure.
    pub fn with_uri(self, uri: Uri) -> Result<Self, AdvertisingError> {
        self.add_ad_struct(UriAdStruct::new(uri))
    }

    fn add_ad_struct(mut self, ad_struct: impl EncodeToBuffer) -> Result<Self, AdvertisingError> {
        self.data
            .data
            .fill(|b| ad_struct.encode(b))
            .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?;
        Ok(self)
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
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ScanResponseData {
    data: bletio_hci::ScanResponseData,
}

impl ScanResponseData {
    /// Instantiate a builder to create Scan Response Data.
    pub fn builder() -> ScanResponseDataBuilder {
        ScanResponseDataBuilder::new()
    }
}

impl EncodeToBuffer for ScanResponseData {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, bletio_utils::Error> {
        self.data.encode(buffer)
    }

    fn encoded_size(&self) -> usize {
        self.data.encoded_size()
    }
}

impl From<&ScanResponseData> for bletio_hci::ScanResponseData {
    fn from(value: &ScanResponseData) -> Self {
        value.data.clone()
    }
}

#[cfg(test)]
mod test {
    use bletio_hci::RandomStaticDeviceAddress;
    use bletio_utils::{Buffer, BufferOps};
    use rstest::{fixture, rstest};

    use crate::assigned_numbers::ProvisionedUriScheme;

    use super::*;

    #[fixture]
    fn advertising_data_builder_empty() -> AdvertisingData {
        let builder = AdvertisingData::builder();
        assert_eq!(builder.data, AdvertisingData::default());
        builder.build()
    }

    #[fixture]
    fn advertising_data_builder_service_uuid16() -> AdvertisingData {
        AdvertisingData::builder()
            .with_advertising_interval(AdvertisingInterval::default())
            .unwrap()
            .with_appearance()
            .unwrap()
            .with_local_name(LocalNameComplete::Complete)
            .unwrap()
            .with_service_data_uuid16(ServiceUuid::Battery, &[0x50, 0x84, 0x91, 0xAF])
            .unwrap()
            .with_service_uuid16(
                &[ServiceUuid::Battery, ServiceUuid::BloodPressure],
                ServiceListComplete::Complete,
            )
            .unwrap()
            .build()
    }

    #[fixture]
    fn advertising_data_builder_service_uuid32() -> AdvertisingData {
        const ADDRESSES: &[PublicDeviceAddress] = &[PublicDeviceAddress::new([
            0xF4, 0x23, 0x14, 0xC3, 0xDC, 0x24,
        ])];
        AdvertisingData::builder()
            .with_flags(Flags::BREDR_NOT_SUPPORTED | Flags::LE_GENERAL_DISCOVERABLE_MODE)
            .unwrap()
            .with_le_supported_features()
            .unwrap()
            .with_local_name(LocalNameComplete::Shortened(9))
            .unwrap()
            .with_public_target_address(ADDRESSES)
            .unwrap()
            .with_service_uuid32(
                &[Uuid32(0x0000_1803), Uuid32(0x0000_180F)],
                ServiceListComplete::Incomplete,
            )
            .unwrap()
            .build()
    }

    #[fixture]
    fn advertising_data_builder_service_uuid128() -> AdvertisingData {
        AdvertisingData::builder()
            .with_peripheral_connection_interval_range(
                0x0006.try_into().unwrap()..=0x0010.try_into().unwrap(),
            )
            .unwrap()
            .with_service_uuid128(
                &[Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)],
                ServiceListComplete::Complete,
            )
            .unwrap()
            .with_tx_power_level()
            .unwrap()
            .build()
    }

    #[fixture]
    fn advertising_data_builder_service_solicitation_uuid16() -> AdvertisingData {
        AdvertisingData::builder()
            .with_service_solicitation_uuid16(&[ServiceUuid::Battery, ServiceUuid::BloodPressure])
            .unwrap()
            .with_uri(Uri::try_new(ProvisionedUriScheme::Https, "//example.org/").unwrap())
            .unwrap()
            .build()
    }

    #[fixture]
    fn advertising_data_builder_service_solicitation_uuid32() -> AdvertisingData {
        AdvertisingData::builder()
            .with_manufacturer_specific_data(
                CompanyIdentifier::StMicroelectronics,
                &[0x9E, 0xF5, 0x40, 0x7C, 0x0F],
            )
            .unwrap()
            .with_service_data_uuid32(Uuid32(0x0000_1803), &[0x50, 0x84, 0x91, 0xAF])
            .unwrap()
            .with_service_solicitation_uuid32(&[Uuid32(0x0000_1803), Uuid32(0x0000_180F)])
            .unwrap()
            .build()
    }

    #[fixture]
    fn advertising_data_builder_service_solicitation_uuid128() -> AdvertisingData {
        const ADDRESSES: &[RandomAddress] = &[RandomAddress::Static(
            match RandomStaticDeviceAddress::try_new([0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0xF7]) {
                Ok(v) => v,
                Err(_) => unreachable!(),
            },
        )];
        AdvertisingData::builder()
            .with_random_target_address(ADDRESSES)
            .unwrap()
            .with_service_solicitation_uuid128(&[Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)])
            .unwrap()
            .build()
    }

    #[fixture]
    fn advertising_data_builder_service_data_uuid128() -> AdvertisingData {
        AdvertisingData::builder()
            .with_service_data_uuid128(
                Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640),
                &[0x50, 0x84, 0x91, 0xAF],
            )
            .unwrap()
            .build()
    }

    #[rstest]
    #[case::empty(
        advertising_data_builder_empty(),
        &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    )]
    #[case::service_uuid16(
        advertising_data_builder_service_uuid16(),
        &[0x18, 0x03, 0x1A, 0x00, 0x08, 0x03, 0x19, 0x00, 0x00, 0x01, 0x09, 0x07, 0x16, 0x0F, 0x18, 0x50,
            0x84, 0x91, 0xAF, 0x05, 0x03, 0x0F, 0x18, 0x10, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    )]
    #[case::service_uuid32(
        advertising_data_builder_service_uuid32(),
        &[0x19, 0x02, 0x01, 0x06, 0x01, 0x27, 0x01, 0x08, 0x07, 0x17, 0xF4, 0x23, 0x14, 0xC3, 0xDC, 0x24,
            0x09, 0x04, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    )]
    #[case::service_uuid128(
        advertising_data_builder_service_uuid128(),
        &[0x1B, 0x05, 0x12, 0x06, 0x00, 0x10, 0x00, 0x11, 0x07, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C,
            0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5, 0x02, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00]
    )]
    #[case::service_solicitation_uuid16(
        advertising_data_builder_service_solicitation_uuid16(),
        &[0x18, 0x05, 0x14, 0x0F, 0x18, 0x10, 0x18, 0x11, 0x24, 0x17, 0x00, b'/', b'/', b'e', b'x', b'a',
            b'm', b'p', b'l', b'e', b'.', b'o', b'r', b'g', b'/', 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    )]
    #[case::service_solicitation_uuid32(
        advertising_data_builder_service_solicitation_uuid32(),
        &[0x1D, 0x08, 0xFF, 0x30, 0x00, 0x9E, 0xF5, 0x40, 0x7C, 0x0F, 0x09, 0x20, 0x03, 0x18, 0x00, 0x00,
            0x50, 0x84, 0x91, 0xAF, 0x09, 0x1F, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00, 0x00, 0x00]
    )]
    #[case::service_solicitation_uuid128(
        advertising_data_builder_service_solicitation_uuid128(),
        &[0x1A, 0x07, 0x18, 0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0xF7, 0x11, 0x15, 0x40, 0xD6, 0x6E, 0xFD, 0xD0,
            0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5, 0x00, 0x00, 0x00, 0x00, 0x00]
    )]
    #[case::service_data_uuid128(
        advertising_data_builder_service_data_uuid128(),
        &[0x16, 0x15, 0x21, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E,
            0x28, 0xA1, 0xF5, 0x50, 0x84, 0x91, 0xAF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    )]
    fn test_advertising_data_builder(
        #[case] adv_data: AdvertisingData,
        #[case] expected_encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<32>::default();
        assert_eq!(adv_data.encoded_size(), 32);
        assert_eq!(adv_data.encode(&mut buffer)?, 32);
        assert_eq!(buffer.data(), expected_encoded_data);
        Ok(())
    }

    #[fixture]
    fn scan_response_data_builder_empty() -> ScanResponseData {
        let builder = ScanResponseData::builder();
        assert_eq!(builder.data, ScanResponseData::default());
        builder.build()
    }

    #[fixture]
    fn scan_response_data_builder_service_uuid16() -> ScanResponseData {
        ScanResponseData::builder()
            .with_advertising_interval(AdvertisingInterval::default())
            .unwrap()
            .with_appearance()
            .unwrap()
            .with_local_name(LocalNameComplete::Complete)
            .unwrap()
            .with_service_data_uuid16(ServiceUuid::Battery, &[0x50, 0x84, 0x91, 0xAF])
            .unwrap()
            .with_service_uuid16(
                &[ServiceUuid::Battery, ServiceUuid::BloodPressure],
                ServiceListComplete::Complete,
            )
            .unwrap()
            .build()
    }

    #[fixture]
    fn scan_response_data_builder_service_uuid32() -> ScanResponseData {
        const ADDRESSES: &[PublicDeviceAddress] = &[PublicDeviceAddress::new([
            0xF4, 0x23, 0x14, 0xC3, 0xDC, 0x24,
        ])];
        ScanResponseData::builder()
            .with_le_supported_features()
            .unwrap()
            .with_local_name(LocalNameComplete::Shortened(7))
            .unwrap()
            .with_public_target_address(ADDRESSES)
            .unwrap()
            .with_service_uuid32(
                &[Uuid32(0x0000_1803), Uuid32(0x0000_180F)],
                ServiceListComplete::Incomplete,
            )
            .unwrap()
            .build()
    }

    #[fixture]
    fn scan_response_data_builder_service_uuid128() -> ScanResponseData {
        ScanResponseData::builder()
            .with_peripheral_connection_interval_range(
                0x0006.try_into().unwrap()..=0x0010.try_into().unwrap(),
            )
            .unwrap()
            .with_service_uuid128(
                &[Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)],
                ServiceListComplete::Complete,
            )
            .unwrap()
            .with_tx_power_level()
            .unwrap()
            .build()
    }

    #[fixture]
    fn scan_response_data_builder_service_solicitation_uuid16() -> ScanResponseData {
        ScanResponseData::builder()
            .with_service_solicitation_uuid16(&[ServiceUuid::Battery, ServiceUuid::BloodPressure])
            .unwrap()
            .with_uri(Uri::try_new(ProvisionedUriScheme::Https, "//example.org/").unwrap())
            .unwrap()
            .build()
    }

    #[fixture]
    fn scan_response_data_builder_service_solicitation_uuid32() -> ScanResponseData {
        ScanResponseData::builder()
            .with_manufacturer_specific_data(
                CompanyIdentifier::StMicroelectronics,
                &[0x9E, 0xF5, 0x40, 0x7C, 0x0F],
            )
            .unwrap()
            .with_service_data_uuid32(Uuid32(0x0000_1803), &[0x50, 0x84, 0x91, 0xAF])
            .unwrap()
            .with_service_solicitation_uuid32(&[Uuid32(0x0000_1803), Uuid32(0x0000_180F)])
            .unwrap()
            .build()
    }

    #[fixture]
    fn scan_response_data_builder_service_solicitation_uuid128() -> ScanResponseData {
        const ADDRESSES: &[RandomAddress] = &[RandomAddress::Static(
            match RandomStaticDeviceAddress::try_new([0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0xF7]) {
                Ok(v) => v,
                Err(_) => unreachable!(),
            },
        )];
        ScanResponseData::builder()
            .with_random_target_address(ADDRESSES)
            .unwrap()
            .with_service_solicitation_uuid128(&[Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)])
            .unwrap()
            .build()
    }

    #[fixture]
    fn scan_response_data_builder_service_data_uuid128() -> ScanResponseData {
        ScanResponseData::builder()
            .with_service_data_uuid128(
                Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640),
                &[0x50, 0x84, 0x91, 0xAF],
            )
            .unwrap()
            .build()
    }

    #[rstest]
    #[case::empty(
        scan_response_data_builder_empty(),
        &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    )]
    #[case::service_uuid16(
        scan_response_data_builder_service_uuid16(),
        &[0x18, 0x03, 0x1A, 0x00, 0x08, 0x03, 0x19, 0x00, 0x00, 0x01, 0x09, 0x07, 0x16, 0x0F, 0x18, 0x50,
            0x84, 0x91, 0xAF, 0x05, 0x03, 0x0F, 0x18, 0x10, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    )]
    #[case::service_uuid32(
        scan_response_data_builder_service_uuid32(),
        &[0x16, 0x01, 0x27, 0x01, 0x08, 0x07, 0x17, 0xF4, 0x23, 0x14, 0xC3, 0xDC, 0x24, 0x09, 0x04, 0x03,
            0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    )]
    #[case::service_uuid128(
        scan_response_data_builder_service_uuid128(),
        &[0x1B, 0x05, 0x12, 0x06, 0x00, 0x10, 0x00, 0x11, 0x07, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C,
            0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5, 0x02, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00]
    )]
    #[case::service_solicitation_uuid16(
        scan_response_data_builder_service_solicitation_uuid16(),
        &[0x18, 0x05, 0x14, 0x0F, 0x18, 0x10, 0x18, 0x11, 0x24, 0x17, 0x00, b'/', b'/', b'e', b'x', b'a',
            b'm', b'p', b'l', b'e', b'.', b'o', b'r', b'g', b'/', 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    )]
    #[case::service_solicitation_uuid32(
        scan_response_data_builder_service_solicitation_uuid32(),
        &[0x1D, 0x08, 0xFF, 0x30, 0x00, 0x9E, 0xF5, 0x40, 0x7C, 0x0F, 0x09, 0x20, 0x03, 0x18, 0x00, 0x00,
            0x50, 0x84, 0x91, 0xAF, 0x09, 0x1F, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00, 0x00, 0x00]
    )]
    #[case::service_solicitation_uuid128(
        scan_response_data_builder_service_solicitation_uuid128(),
        &[0x1A, 0x07, 0x18, 0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0xF7, 0x11, 0x15, 0x40, 0xD6, 0x6E, 0xFD, 0xD0,
            0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5, 0x00, 0x00, 0x00, 0x00, 0x00]
    )]
    #[case::service_data_uuid128(
        scan_response_data_builder_service_data_uuid128(),
        &[0x16, 0x15, 0x21, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E,
            0x28, 0xA1, 0xF5, 0x50, 0x84, 0x91, 0xAF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    )]
    fn test_scan_response_data_builder(
        #[case] scanresp_data: ScanResponseData,
        #[case] expected_encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<32>::default();
        assert_eq!(scanresp_data.encoded_size(), 32);
        assert_eq!(scanresp_data.encode(&mut buffer)?, 32);
        assert_eq!(buffer.data(), expected_encoded_data);
        Ok(())
    }

    // #[test]
    // fn test_full_advertising_data_success() -> Result<(), Error> {
    //     let local_name = "bletio";
    //     let adv_data = AdvertisingData::builder()
    //         .with_appearance()
    //         .with_tx_power_level()
    //         .with_le_supported_features()
    //         .with_local_name(LocalNameComplete::Shortened(3))
    //         .build();
    //     let scanresp_data = ScanResponseData::builder()
    //         .with_local_name(LocalNameComplete::Complete)
    //         .build();
    //     let full_adv_data = FullAdvertisingData::try_new(adv_data.clone(), scanresp_data.clone())?;
    //     assert_eq!(
    //         full_adv_data.adv_data.base.appearance,
    //         Some(AppearanceAdStruct::new(AppearanceValue::GenericUnknown))
    //     );
    //     assert_eq!(
    //         full_adv_data.adv_data.base.local_name,
    //         Some(LocalNameAdStruct::new("", LocalNameComplete::Shortened(3)))
    //     );
    //     assert_eq!(
    //         full_adv_data.adv_data.base.tx_power_level,
    //         Some(TxPowerLevelAdStruct::new(TxPowerLevel::default()))
    //     );
    //     assert_eq!(
    //         full_adv_data.adv_data.base.le_supported_features,
    //         Some(LeSupportedFeaturesAdStruct::new(
    //             SupportedLeFeatures::default()
    //         ))
    //     );
    //     assert_eq!(
    //         full_adv_data
    //             .scanresp_data
    //             .as_ref()
    //             .unwrap()
    //             .base
    //             .local_name,
    //         Some(LocalNameAdStruct::new("", LocalNameComplete::Complete))
    //     );
    //     assert_eq!(full_adv_data.adv_data, adv_data);
    //     assert_eq!(full_adv_data.scanresp_data, Some(scanresp_data));

    //     let appearance = AppearanceValue::Thermostat;
    //     let tx_power_level = TxPowerLevel::try_new(-8).unwrap();
    //     let supported_le_features =
    //         SupportedLeFeatures::LE_2M_PHY | SupportedLeFeatures::LE_CODED_PHY;
    //     let mut device_information = DeviceInformation::default();
    //     device_information.appearance = appearance;
    //     device_information.local_name = "bletio";
    //     device_information.tx_power_level = tx_power_level;
    //     device_information.supported_le_features = supported_le_features;
    //     let filled_full_adv_data = full_adv_data.fill_automatic_data(&device_information);
    //     assert_eq!(
    //         filled_full_adv_data.adv_data.base.appearance,
    //         Some(AppearanceAdStruct::new(appearance))
    //     );
    //     assert_eq!(
    //         filled_full_adv_data.adv_data.base.local_name,
    //         Some(LocalNameAdStruct::new(
    //             local_name,
    //             LocalNameComplete::Shortened(3)
    //         ))
    //     );
    //     assert_eq!(
    //         filled_full_adv_data.adv_data.base.tx_power_level,
    //         Some(TxPowerLevelAdStruct::new(tx_power_level))
    //     );
    //     assert_eq!(
    //         filled_full_adv_data.adv_data.base.le_supported_features,
    //         Some(LeSupportedFeaturesAdStruct::new(supported_le_features))
    //     );
    //     assert_eq!(
    //         filled_full_adv_data
    //             .scanresp_data
    //             .as_ref()
    //             .unwrap()
    //             .base
    //             .local_name,
    //         Some(LocalNameAdStruct::new(
    //             local_name,
    //             LocalNameComplete::Complete
    //         ))
    //     );

    //     Ok(())
    // }

    // #[test]
    // fn test_full_advertising_data_failure() {
    //     let adv_data = AdvertisingData::builder().with_appearance().build();
    //     let scanresp_data = ScanResponseData::builder().with_appearance().build();
    //     let err = FullAdvertisingData::try_new(adv_data, scanresp_data);
    //     assert_eq!(
    //         err,
    //         Err(Error::Advertising(
    //             AdvertisingError::AppearanceNotAllowedInBothAdvertisingDataAndScanResponseData
    //         ))
    //     );
    // }
}
