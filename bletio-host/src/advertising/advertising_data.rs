use core::ops::RangeInclusive;

use bletio_hci::{
    AdvertisingInterval, ConnectionInterval, LeAdvertisingReportData, PublicDeviceAddress,
    RandomAddress, SupportedLeFeatures, TxPowerLevel,
};
use bletio_utils::{BufferOps, EncodeToBuffer};

use crate::advertising::ad_struct::{
    AdStruct, AdvertisingIntervalAdStruct, AppearanceAdStruct, FlagsAdStruct,
    LeSupportedFeaturesAdStruct, LocalNameAdStruct, ManufacturerSpecificDataAdStruct,
    PeripheralConnectionIntervalRangeAdStruct, PublicTargetAddressAdStruct,
    RandomTargetAddressAdStruct, ServiceDataUuid128AdStruct, ServiceDataUuid16AdStruct,
    ServiceDataUuid32AdStruct, ServiceSolicitationUuid128AdStruct,
    ServiceSolicitationUuid16AdStruct, ServiceSolicitationUuid32AdStruct, ServiceUuid128AdStruct,
    ServiceUuid16AdStruct, ServiceUuid32AdStruct, TxPowerLevelAdStruct, UriAdStruct,
};
use crate::advertising::{AdvertisingError, Flags, LocalNameComplete, ServiceListComplete, Uri};
use crate::assigned_numbers::{AppearanceValue, CompanyIdentifier, ServiceUuid};
use crate::uuid::{Uuid128, Uuid32};
use crate::{DeviceInformation, Error};

const EMPTY_ADVERTISING_DATA_ITERATOR: AdvertisingDataIterator = AdvertisingDataIterator {
    data: &[],
    next_index: 0,
};

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
        if let Some(scanresp_data) = &scanresp_data {
            if adv_data
                .iter()
                .any(|ad_struct| matches!(ad_struct, AdStruct::Appearance(_)))
                && scanresp_data
                    .iter()
                    .any(|ad_struct| matches!(ad_struct, AdStruct::Appearance(_)))
            {
                return Err(
                    AdvertisingError::AppearanceNotAllowedInBothAdvertisingDataAndScanResponseData,
                )?;
            }
        }
        Ok(Self {
            adv_data,
            scanresp_data,
        })
    }

    pub fn advertising_data(&self) -> &AdvertisingData {
        &self.adv_data
    }

    pub fn iter(
        &self,
    ) -> core::iter::Chain<AdvertisingDataIterator<'_>, AdvertisingDataIterator<'_>> {
        if let Some(scanresp_data) = self.scanresp_data.as_ref() {
            self.adv_data.iter().chain(scanresp_data.iter())
        } else {
            self.adv_data.iter().chain(EMPTY_ADVERTISING_DATA_ITERATOR)
        }
    }

    pub fn scan_response_data(&self) -> Option<&ScanResponseData> {
        self.scanresp_data.as_ref()
    }

    pub(crate) fn fill_automatic_data(
        &self,
        device_information: &DeviceInformation,
    ) -> Result<Self, Error> {
        let mut adv_data_builder = AdvertisingData::builder();
        for ad_struct in self.adv_data.iter() {
            if matches!(ad_struct, AdStruct::Appearance(_)) {
                adv_data_builder = adv_data_builder
                    .add_ad_struct(AppearanceAdStruct::new(device_information.appearance))?;
            } else if matches!(ad_struct, AdStruct::LeSupportedFeatures(_)) {
                adv_data_builder = adv_data_builder.add_ad_struct(
                    LeSupportedFeaturesAdStruct::new(device_information.supported_le_features),
                )?;
            } else if matches!(ad_struct, AdStruct::TxPowerLevel(_)) {
                adv_data_builder = adv_data_builder
                    .add_ad_struct(TxPowerLevelAdStruct::new(device_information.tx_power_level))?;
            } else if let AdStruct::LocalName(local_name) = ad_struct {
                adv_data_builder = adv_data_builder.add_ad_struct(LocalNameAdStruct::try_new(
                    device_information.local_name,
                    local_name.complete,
                )?)?;
            } else {
                adv_data_builder = adv_data_builder.add_ad_struct(ad_struct)?;
            }
        }

        let adv_data = adv_data_builder.build();

        let scanresp_data = if let Some(scanresp_data) = self.scanresp_data.as_ref() {
            let mut scanresp_data_builder = ScanResponseData::builder();
            for ad_struct in scanresp_data.iter() {
                if matches!(ad_struct, AdStruct::Appearance(_)) {
                    scanresp_data_builder = scanresp_data_builder
                        .add_ad_struct(AppearanceAdStruct::new(device_information.appearance))?;
                } else if matches!(ad_struct, AdStruct::LeSupportedFeatures(_)) {
                    scanresp_data_builder = scanresp_data_builder.add_ad_struct(
                        LeSupportedFeaturesAdStruct::new(device_information.supported_le_features),
                    )?;
                } else if matches!(ad_struct, AdStruct::TxPowerLevel(_)) {
                    scanresp_data_builder = scanresp_data_builder.add_ad_struct(
                        TxPowerLevelAdStruct::new(device_information.tx_power_level),
                    )?;
                } else if let AdStruct::LocalName(local_name) = ad_struct {
                    scanresp_data_builder =
                        scanresp_data_builder.add_ad_struct(LocalNameAdStruct::try_new(
                            device_information.local_name,
                            local_name.complete,
                        )?)?;
                } else {
                    scanresp_data_builder = scanresp_data_builder.add_ad_struct(ad_struct)?;
                }
            }

            Some(scanresp_data_builder.build())
        } else {
            None
        };

        FullAdvertisingData::try_new(adv_data, scanresp_data)
    }
}

/// Builder to create `AdvertisingData` packets.
#[derive(Debug, Default, PartialEq, Eq)]
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
        if self.has_ad_struct(|ad_struct| matches!(ad_struct, AdStruct::AdvertisingInterval(_))) {
            Err(AdvertisingError::OnlyOneAdvertisingIntervalAllowedInAdvertisingDataOrScanResponseData)
        } else {
            self.add_ad_struct(AdvertisingIntervalAdStruct::new(interval))
        }
    }

    /// Add an Appearance Advertising Structure to the `AdvertisingData`.
    pub fn with_appearance(self) -> Result<Self, AdvertisingError> {
        if self.has_ad_struct(|ad_struct| matches!(ad_struct, AdStruct::Appearance(_))) {
            Err(AdvertisingError::OnlyOneAppearanceAllowedInAdvertisingDataOrScanResponseData)
        } else {
            self.add_ad_struct(AppearanceAdStruct::new(AppearanceValue::GenericUnknown))
        }
    }

    /// Add a Flags Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `flags` — The Flags value to put in the added Flags Advertising Structure.
    pub fn with_flags(self, flags: Flags) -> Result<Self, AdvertisingError> {
        if self.has_ad_struct(|ad_struct| matches!(ad_struct, AdStruct::Flags(_))) {
            Err(AdvertisingError::OnlyOneFlagsAllowedInAdvertisingData)
        } else {
            self.add_ad_struct(FlagsAdStruct::new(flags))
        }
    }

    /// Add a LE Supported Features Advertising Structure to the `AdvertisingData`.
    pub fn with_le_supported_features(self) -> Result<Self, AdvertisingError> {
        if self.has_ad_struct(|ad_struct| matches!(ad_struct, AdStruct::LeSupportedFeatures(_))) {
            Err(AdvertisingError::OnlyOneLeSupportedFeaturesAllowedInAdvertisingDataOrScanResponseData)
        } else {
            self.add_ad_struct(LeSupportedFeaturesAdStruct::new(
                SupportedLeFeatures::default(),
            ))
        }
    }

    /// Add a Local Name Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `complete` — Whether the local name should be put complete or shortened in the added Local Name Advertising Structure.
    pub fn with_local_name(self, complete: LocalNameComplete) -> Result<Self, AdvertisingError> {
        if self.has_ad_struct(|ad_struct| matches!(ad_struct, AdStruct::LocalName(_))) {
            Err(AdvertisingError::OnlyOneLocalNameAllowedInAdvertisingDataOrScanResponseData)
        } else {
            self.add_ad_struct(LocalNameAdStruct::try_new("", complete)?)
        }
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
        if self.has_ad_struct(|ad_struct| matches!(ad_struct, AdStruct::PublicTargetAddress(_))) {
            Err(AdvertisingError::OnlyOnePublicTargetAddressAllowedInAdvertisingDataOrScanResponseData)
        } else {
            self.add_ad_struct(PublicTargetAddressAdStruct::try_new(addresses)?)
        }
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
        if self.has_ad_struct(|ad_struct| matches!(ad_struct, AdStruct::RandomTargetAddress(_))) {
            Err(AdvertisingError::OnlyOneRandomTargetAddressAllowedInAdvertisingDataOrScanResponseData)
        } else {
            self.add_ad_struct(RandomTargetAddressAdStruct::try_new(addresses)?)
        }
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

    fn has_ad_struct(&self, func: impl FnMut(AdStruct) -> bool) -> bool {
        self.data.iter().any(func)
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

    pub fn iter(&self) -> AdvertisingDataIterator {
        AdvertisingDataIterator {
            data: self.data.data(),
            next_index: 0,
        }
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

impl From<&LeAdvertisingReportData> for AdvertisingData {
    fn from(value: &LeAdvertisingReportData) -> Self {
        Self { data: value.into() }
    }
}

/// Builder to create `ScanResponseData` packets.
#[derive(Debug, Default, PartialEq, Eq)]
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
        if self.has_ad_struct(|ad_struct| matches!(ad_struct, AdStruct::AdvertisingInterval(_))) {
            Err(AdvertisingError::OnlyOneAdvertisingIntervalAllowedInAdvertisingDataOrScanResponseData)
        } else {
            self.add_ad_struct(AdvertisingIntervalAdStruct::new(interval))
        }
    }

    /// Add an Appearance Advertising Structure to the `ScanResponseData`.
    pub fn with_appearance(self) -> Result<Self, AdvertisingError> {
        if self.has_ad_struct(|ad_struct| matches!(ad_struct, AdStruct::Appearance(_))) {
            Err(AdvertisingError::OnlyOneAppearanceAllowedInAdvertisingDataOrScanResponseData)
        } else {
            self.add_ad_struct(AppearanceAdStruct::new(AppearanceValue::GenericUnknown))
        }
    }

    /// Add a LE Supported Features Advertising Structure to the `ScanResponseData`.
    pub fn with_le_supported_features(self) -> Result<Self, AdvertisingError> {
        if self.has_ad_struct(|ad_struct| matches!(ad_struct, AdStruct::LeSupportedFeatures(_))) {
            Err(AdvertisingError::OnlyOneLeSupportedFeaturesAllowedInAdvertisingDataOrScanResponseData)
        } else {
            self.add_ad_struct(LeSupportedFeaturesAdStruct::new(
                SupportedLeFeatures::default(),
            ))
        }
    }

    /// Add a Local Name Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `complete` — Whether the local name should be put complete or shortened in the added Local Name Advertising Structure.
    pub fn with_local_name(self, complete: LocalNameComplete) -> Result<Self, AdvertisingError> {
        if self.has_ad_struct(|ad_struct| matches!(ad_struct, AdStruct::LocalName(_))) {
            Err(AdvertisingError::OnlyOneLocalNameAllowedInAdvertisingDataOrScanResponseData)
        } else {
            self.add_ad_struct(LocalNameAdStruct::try_new("", complete)?)
        }
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
        if self.has_ad_struct(|ad_struct| matches!(ad_struct, AdStruct::PublicTargetAddress(_))) {
            Err(AdvertisingError::OnlyOnePublicTargetAddressAllowedInAdvertisingDataOrScanResponseData)
        } else {
            self.add_ad_struct(PublicTargetAddressAdStruct::try_new(addresses)?)
        }
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
        if self.has_ad_struct(|ad_struct| matches!(ad_struct, AdStruct::RandomTargetAddress(_))) {
            Err(AdvertisingError::OnlyOneRandomTargetAddressAllowedInAdvertisingDataOrScanResponseData)
        } else {
            self.add_ad_struct(RandomTargetAddressAdStruct::try_new(addresses)?)
        }
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

    fn has_ad_struct(&self, func: impl FnMut(AdStruct) -> bool) -> bool {
        self.data.iter().any(func)
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

    pub fn iter(&self) -> AdvertisingDataIterator {
        AdvertisingDataIterator {
            data: self.data.data(),
            next_index: 0,
        }
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

impl From<&LeAdvertisingReportData> for ScanResponseData {
    fn from(value: &LeAdvertisingReportData) -> Self {
        Self { data: value.into() }
    }
}

pub struct AdvertisingDataIterator<'a> {
    data: &'a [u8],
    next_index: usize,
}

impl Iterator for AdvertisingDataIterator<'_> {
    type Item = AdStruct;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.next_index >= self.data.len() {
                return None;
            }
            if self.next_index == 0 {
                match parser::advertising_data_length(self.data) {
                    Ok(_) => self.next_index += 1,
                    Err(_) => unreachable!("That means data is empty and so we already returned"),
                }
            }

            match parser::ad_struct(&self.data[self.next_index..]) {
                Ok((_, (len, ad_struct))) => {
                    self.next_index += len;
                    return Some(ad_struct);
                }
                Err(_) => match parser::ad_struct_length(&self.data[self.next_index..]) {
                    Ok((_, len)) => {
                        self.next_index += len + 1;
                    }
                    Err(_) => return None,
                },
            }
        }
    }
}

pub(crate) mod parser {
    use nom::{
        combinator::{map, map_res},
        number::{le_u128, le_u16, le_u32, le_u8},
        IResult, Parser,
    };

    use crate::{
        advertising::ad_struct::{
            advertising_interval::parser::advertising_interval_ad_struct,
            appearance::parser::appearance_ad_struct,
            flags::parser::flags_ad_struct,
            le_supported_features::parser::le_supported_features_ad_struct,
            local_name::parser::{complete_local_name_ad_struct, shortened_local_name_ad_struct},
            manufacturer_specific_data::parser::manufacturer_specific_data_ad_struct,
            peripheral_connection_interval_range::parser::peripheral_connection_interval_range_ad_struct,
            public_target_address::parser::public_target_address_ad_struct,
            random_target_address::parser::random_target_address_ad_struct,
            service_data::parser::{
                service_data_uuid128_ad_struct, service_data_uuid16_ad_struct,
                service_data_uuid32_ad_struct,
            },
            service_solicitation::parser::{
                list_of_solicitation_service_uuid128_ad_struct,
                list_of_solicitation_service_uuid16_ad_struct,
                list_of_solicitation_service_uuid32_ad_struct,
            },
            service_uuid::parser::{
                complete_list_of_service_uuid128_ad_struct,
                complete_list_of_service_uuid16_ad_struct,
                complete_list_of_service_uuid32_ad_struct,
                incomplete_list_of_service_uuid128_ad_struct,
                incomplete_list_of_service_uuid16_ad_struct,
                incomplete_list_of_service_uuid32_ad_struct,
            },
            tx_power_level::parser::tx_power_level_ad_struct,
            uri::parser::uri_ad_struct,
            AdStruct,
        },
        assigned_numbers::{AdType, ServiceUuid},
        uuid::{Uuid128, Uuid32},
    };

    pub(crate) fn advertising_data_length(input: &[u8]) -> IResult<&[u8], u8> {
        le_u8().parse(input)
    }

    pub(crate) fn ad_struct_length(input: &[u8]) -> IResult<&[u8], usize> {
        map(le_u8(), |v| v as usize).parse(input)
    }

    fn ad_type(input: &[u8]) -> IResult<&[u8], AdType> {
        map_res(le_u8(), TryInto::try_into).parse(input)
    }

    pub(crate) fn service_uuid(input: &[u8]) -> IResult<&[u8], ServiceUuid> {
        map_res(le_u16(), TryFrom::try_from).parse(input)
    }

    pub(crate) fn uuid32(input: &[u8]) -> IResult<&[u8], Uuid32> {
        map(le_u32(), Into::into).parse(input)
    }

    pub(crate) fn uuid128(input: &[u8]) -> IResult<&[u8], Uuid128> {
        map(le_u128(), Into::into).parse(input)
    }

    pub(crate) fn ad_struct(input: &[u8]) -> IResult<&[u8], (usize, AdStruct)> {
        let (rest, (ad_struct_length, ad_type)) = (ad_struct_length, ad_type).parse(input)?;
        let len = ad_struct_length - 1;
        let parameters = &rest[..len];
        let (_rest, ad_struct) = match ad_type {
            AdType::Flags => flags_ad_struct.parse(parameters),
            AdType::IncompleteListOfServiceUuid16 => {
                incomplete_list_of_service_uuid16_ad_struct.parse(parameters)
            }
            AdType::CompleteListOfServiceUuid16 => {
                complete_list_of_service_uuid16_ad_struct.parse(parameters)
            }
            AdType::IncompleteListOfServiceUuid32 => {
                incomplete_list_of_service_uuid32_ad_struct.parse(parameters)
            }
            AdType::CompleteListOfServiceUuid32 => {
                complete_list_of_service_uuid32_ad_struct.parse(parameters)
            }
            AdType::IncompleteListOfServiceUuid128 => {
                incomplete_list_of_service_uuid128_ad_struct.parse(parameters)
            }
            AdType::CompleteListOfServiceUuid128 => {
                complete_list_of_service_uuid128_ad_struct.parse(parameters)
            }
            AdType::ShortenedLocalName => shortened_local_name_ad_struct.parse(parameters),
            AdType::CompleteLocalName => complete_local_name_ad_struct.parse(parameters),
            AdType::TxPowerLevel => tx_power_level_ad_struct.parse(parameters),
            AdType::PeripheralConnectionIntervalRange => {
                peripheral_connection_interval_range_ad_struct.parse(parameters)
            }
            AdType::ListOfSolicitationServiceUuid16 => {
                list_of_solicitation_service_uuid16_ad_struct.parse(parameters)
            }
            AdType::ListOfSolicitationServiceUuid128 => {
                list_of_solicitation_service_uuid128_ad_struct.parse(parameters)
            }
            AdType::ServiceDataUuid16 => service_data_uuid16_ad_struct.parse(parameters),
            AdType::PublicTargetAddress => public_target_address_ad_struct.parse(parameters),
            AdType::RandomTargetAddress => random_target_address_ad_struct.parse(parameters),
            AdType::Appearance => appearance_ad_struct.parse(parameters),
            AdType::AdvertisingInterval => advertising_interval_ad_struct.parse(parameters),
            AdType::ListOfSolicitationServiceUuid32 => {
                list_of_solicitation_service_uuid32_ad_struct.parse(parameters)
            }
            AdType::ServiceDataUuid32 => service_data_uuid32_ad_struct.parse(parameters),
            AdType::ServiceDataUuid128 => service_data_uuid128_ad_struct.parse(parameters),
            AdType::Uri => uri_ad_struct.parse(parameters),
            AdType::LeSupportedFeatures => le_supported_features_ad_struct.parse(parameters),
            AdType::ManufacturerSpecificData => {
                manufacturer_specific_data_ad_struct.parse(parameters)
            }
            _ => Ok((&[] as &[u8], AdStruct::Unhandled(ad_type.into()))),
        }?;
        Ok((&rest[len..], (ad_struct_length + 1, ad_struct)))
    }
}

#[cfg(test)]
mod test {
    use bletio_hci::RandomStaticDeviceAddress;
    use bletio_utils::{Buffer, BufferOps};
    use rstest::{fixture, rstest};

    use crate::{
        advertising::advertising_data::parser::ad_struct, assigned_numbers::ProvisionedUriScheme,
    };

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
            .with_local_name(LocalNameComplete::Shortened(3))
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
        &[0x1C, 0x02, 0x01, 0x06, 0x01, 0x27, 0x04, 0x08, b' ', b' ', b' ', 0x07, 0x17, 0xF4, 0x23, 0x14,
            0xC3, 0xDC, 0x24, 0x09, 0x04, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00]
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
    fn test_advertising_data_builder_success(
        #[case] adv_data: AdvertisingData,
        #[case] expected_encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<32>::default();
        assert_eq!(adv_data.encoded_size(), 32);
        assert_eq!(adv_data.encode(&mut buffer)?, 32);
        assert_eq!(buffer.data(), expected_encoded_data);
        Ok(())
    }

    #[test]
    fn test_advertising_data_builder_failure() {
        let builder = AdvertisingData::builder();
        let err = builder
            .with_service_uuid16(
                &[
                    ServiceUuid::AlertNotification,
                    ServiceUuid::Battery,
                    ServiceUuid::BloodPressure,
                ],
                ServiceListComplete::Complete,
            )
            .unwrap()
            .with_uri(Uri::try_new(ProvisionedUriScheme::Https, "//example.org/").unwrap())
            .unwrap()
            .with_manufacturer_specific_data(CompanyIdentifier::Inventel, [0u8; 5].as_slice());
        assert_eq!(
            err,
            Err(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
        );
    }

    #[test]
    fn test_advertising_data_builder_unique_advertising_interval() {
        let builder = AdvertisingData::builder();
        let err = builder
            .with_advertising_interval(AdvertisingInterval::default())
            .unwrap()
            .with_advertising_interval(AdvertisingInterval::default());
        assert_eq!(
            err,
            Err(AdvertisingError::OnlyOneAdvertisingIntervalAllowedInAdvertisingDataOrScanResponseData)
        );
    }

    #[test]
    fn test_advertising_data_builder_unique_appearance() {
        let builder = AdvertisingData::builder();
        let err = builder.with_appearance().unwrap().with_appearance();
        assert_eq!(
            err,
            Err(AdvertisingError::OnlyOneAppearanceAllowedInAdvertisingDataOrScanResponseData)
        );
    }

    #[test]
    fn test_advertising_data_builder_unique_flags() {
        let builder = AdvertisingData::builder();
        let err = builder
            .with_flags(Flags::default())
            .unwrap()
            .with_flags(Flags::default());
        assert_eq!(
            err,
            Err(AdvertisingError::OnlyOneFlagsAllowedInAdvertisingData)
        );
    }

    #[test]
    fn test_advertising_data_builder_unique_le_supported_features() {
        let builder = AdvertisingData::builder();
        let err = builder
            .with_le_supported_features()
            .unwrap()
            .with_le_supported_features();
        assert_eq!(
            err,
            Err(AdvertisingError::OnlyOneLeSupportedFeaturesAllowedInAdvertisingDataOrScanResponseData)
        );
    }

    #[test]
    fn test_advertising_data_builder_unique_local_name() {
        let builder = AdvertisingData::builder();
        let err = builder
            .with_local_name(LocalNameComplete::Complete)
            .unwrap()
            .with_local_name(LocalNameComplete::Shortened(3));
        assert_eq!(
            err,
            Err(AdvertisingError::OnlyOneLocalNameAllowedInAdvertisingDataOrScanResponseData)
        );
    }

    #[test]
    fn test_advertising_data_builder_unique_public_target_address() {
        let addresses = [PublicDeviceAddress::new([
            0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56,
        ])];
        let builder = AdvertisingData::builder();
        let err = builder
            .with_public_target_address(&addresses)
            .unwrap()
            .with_public_target_address(&addresses);
        assert_eq!(
            err,
            Err(AdvertisingError::OnlyOnePublicTargetAddressAllowedInAdvertisingDataOrScanResponseData)
        );
    }

    #[test]
    fn test_advertising_data_builder_unique_random_target_address() {
        let addresses = [
            RandomStaticDeviceAddress::try_new([0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0xF7])
                .unwrap()
                .into(),
        ];
        let builder = AdvertisingData::builder();
        let err = builder
            .with_random_target_address(&addresses)
            .unwrap()
            .with_random_target_address(&addresses);
        assert_eq!(
            err,
            Err(AdvertisingError::OnlyOneRandomTargetAddressAllowedInAdvertisingDataOrScanResponseData)
        );
    }

    #[test]
    fn test_advertising_data_from_le_advertising_report_data() {
        let data = [25; 16];
        let mut buffer: Buffer<31> = Buffer::default();
        buffer.copy_from_slice(&data[..]).unwrap();
        let data = LeAdvertisingReportData::from(buffer);
        let adv_data: AdvertisingData = (&data).into();
        assert_eq!(
            adv_data.data.data(),
            &[16, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25]
        );
    }

    #[test]
    fn test_hci_advertising_data_from_advertising_data() {
        let adv_data = AdvertisingData::default();
        let hci_adv_data: bletio_hci::AdvertisingData = (&adv_data).into();
        assert_eq!(adv_data.data, hci_adv_data);
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
            .with_local_name(LocalNameComplete::Shortened(5))
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
        &[0x1B, 0x01, 0x27, 0x06, 0x08, b' ', b' ', b' ', b' ', b' ', 0x07, 0x17, 0xF4, 0x23, 0x14, 0xC3,
            0xDC, 0x24, 0x09, 0x04, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
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
    fn test_scan_response_data_builder_success(
        #[case] scanresp_data: ScanResponseData,
        #[case] expected_encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<32>::default();
        assert_eq!(scanresp_data.encoded_size(), 32);
        assert_eq!(scanresp_data.encode(&mut buffer)?, 32);
        assert_eq!(buffer.data(), expected_encoded_data);
        Ok(())
    }

    #[test]
    fn test_scan_response_data_builder_failure() {
        let builder = ScanResponseData::builder();
        let err = builder
            .with_service_uuid16(
                &[
                    ServiceUuid::AlertNotification,
                    ServiceUuid::Battery,
                    ServiceUuid::BloodPressure,
                ],
                ServiceListComplete::Complete,
            )
            .unwrap()
            .with_uri(Uri::try_new(ProvisionedUriScheme::Https, "//example.org/").unwrap())
            .unwrap()
            .with_manufacturer_specific_data(CompanyIdentifier::Inventel, [0u8; 5].as_slice());
        assert_eq!(
            err,
            Err(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
        );
    }

    #[test]
    fn test_scan_response_data_builder_unique_advertising_interval() {
        let builder = ScanResponseData::builder();
        let err = builder
            .with_advertising_interval(AdvertisingInterval::default())
            .unwrap()
            .with_advertising_interval(AdvertisingInterval::default());
        assert_eq!(
            err,
            Err(AdvertisingError::OnlyOneAdvertisingIntervalAllowedInAdvertisingDataOrScanResponseData)
        );
    }

    #[test]
    fn test_scan_response_data_builder_unique_appearance() {
        let builder = ScanResponseData::builder();
        let err = builder.with_appearance().unwrap().with_appearance();
        assert_eq!(
            err,
            Err(AdvertisingError::OnlyOneAppearanceAllowedInAdvertisingDataOrScanResponseData)
        );
    }

    #[test]
    fn test_scan_response_data_builder_unique_le_supported_features() {
        let builder = ScanResponseData::builder();
        let err = builder
            .with_le_supported_features()
            .unwrap()
            .with_le_supported_features();
        assert_eq!(
            err,
            Err(AdvertisingError::OnlyOneLeSupportedFeaturesAllowedInAdvertisingDataOrScanResponseData)
        );
    }

    #[test]
    fn test_scan_response_data_builder_unique_local_name() {
        let builder = ScanResponseData::builder();
        let err = builder
            .with_local_name(LocalNameComplete::Complete)
            .unwrap()
            .with_local_name(LocalNameComplete::Shortened(3));
        assert_eq!(
            err,
            Err(AdvertisingError::OnlyOneLocalNameAllowedInAdvertisingDataOrScanResponseData)
        );
    }

    #[test]
    fn test_scan_response_data_builder_unique_public_target_address() {
        let addresses = [PublicDeviceAddress::new([
            0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56,
        ])];
        let builder = ScanResponseData::builder();
        let err = builder
            .with_public_target_address(&addresses)
            .unwrap()
            .with_public_target_address(&addresses);
        assert_eq!(
            err,
            Err(AdvertisingError::OnlyOnePublicTargetAddressAllowedInAdvertisingDataOrScanResponseData)
        );
    }

    #[test]
    fn test_scan_response_data_builder_unique_random_target_address() {
        let addresses = [
            RandomStaticDeviceAddress::try_new([0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0xF7])
                .unwrap()
                .into(),
        ];
        let builder = ScanResponseData::builder();
        let err = builder
            .with_random_target_address(&addresses)
            .unwrap()
            .with_random_target_address(&addresses);
        assert_eq!(
            err,
            Err(AdvertisingError::OnlyOneRandomTargetAddressAllowedInAdvertisingDataOrScanResponseData)
        );
    }

    #[test]
    fn test_scan_response_data_from_le_advertising_report_data() {
        let data = [25; 16];
        let mut buffer: Buffer<31> = Buffer::default();
        buffer.copy_from_slice(&data[..]).unwrap();
        let data = LeAdvertisingReportData::from(buffer);
        let adv_data: ScanResponseData = (&data).into();
        assert_eq!(
            adv_data.data.data(),
            &[16, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25]
        );
    }

    #[test]
    fn test_hci_scan_response_data_from_scan_response_data() {
        let scanresp_data = ScanResponseData::default();
        let hci_scanresp_data: bletio_hci::ScanResponseData = (&scanresp_data).into();
        assert_eq!(scanresp_data.data, hci_scanresp_data);
    }

    #[test]
    fn test_full_advertising_data_success() -> Result<(), Error> {
        let local_name = "bletio";
        let uri = Uri::try_new(ProvisionedUriScheme::Https, "//example.org").unwrap();
        let adv_data = AdvertisingData::builder()
            .with_appearance()
            .unwrap()
            .with_tx_power_level()
            .unwrap()
            .with_le_supported_features()
            .unwrap()
            .with_local_name(LocalNameComplete::Shortened(3))
            .unwrap()
            .build();
        let scanresp_data = ScanResponseData::builder()
            .with_local_name(LocalNameComplete::Complete)
            .unwrap()
            .with_uri(uri.clone())
            .unwrap()
            .build();
        let full_adv_data = FullAdvertisingData::try_new(adv_data.clone(), scanresp_data.clone())?;
        assert_eq!(full_adv_data.iter().count(), 6);
        assert_eq!(full_adv_data.advertising_data().iter().count(), 4);
        assert_eq!(
            full_adv_data.scan_response_data().unwrap().iter().count(),
            2
        );
        let mut it = full_adv_data.iter();
        assert_eq!(
            it.next(),
            Some(AdStruct::Appearance(AppearanceAdStruct::new(
                AppearanceValue::GenericUnknown
            )))
        );
        assert_eq!(
            it.next(),
            Some(AdStruct::TxPowerLevel(TxPowerLevelAdStruct::new(
                TxPowerLevel::default()
            )))
        );
        assert_eq!(
            it.next(),
            Some(AdStruct::LeSupportedFeatures(
                LeSupportedFeaturesAdStruct::new(SupportedLeFeatures::default())
            ))
        );
        assert_eq!(
            it.next(),
            Some(AdStruct::LocalName(
                LocalNameAdStruct::try_new("", LocalNameComplete::Shortened(3)).unwrap()
            ))
        );
        assert_eq!(
            it.next(),
            Some(AdStruct::LocalName(
                LocalNameAdStruct::try_new("", LocalNameComplete::Complete).unwrap()
            ))
        );
        assert_eq!(
            it.next(),
            Some(AdStruct::Uri(UriAdStruct::new(uri.clone())))
        );
        assert_eq!(it.next(), None);

        let appearance = AppearanceValue::Thermostat;
        let tx_power_level = TxPowerLevel::try_new(-8).unwrap();
        let supported_le_features =
            SupportedLeFeatures::LE_2M_PHY | SupportedLeFeatures::LE_CODED_PHY;
        let mut device_information = DeviceInformation::default();
        device_information.appearance = appearance;
        device_information.local_name = "bletio";
        device_information.tx_power_level = tx_power_level;
        device_information.supported_le_features = supported_le_features;
        let filled_full_adv_data = full_adv_data.fill_automatic_data(&device_information)?;
        assert_eq!(filled_full_adv_data.iter().count(), 6);
        assert_eq!(filled_full_adv_data.advertising_data().iter().count(), 4);
        assert_eq!(
            filled_full_adv_data
                .scan_response_data()
                .unwrap()
                .iter()
                .count(),
            2
        );
        let mut it = filled_full_adv_data.iter();
        assert_eq!(
            it.next(),
            Some(AdStruct::Appearance(AppearanceAdStruct::new(appearance)))
        );
        assert_eq!(
            it.next(),
            Some(AdStruct::TxPowerLevel(TxPowerLevelAdStruct::new(
                tx_power_level
            )))
        );
        assert_eq!(
            it.next(),
            Some(AdStruct::LeSupportedFeatures(
                LeSupportedFeaturesAdStruct::new(supported_le_features)
            ))
        );
        assert_eq!(
            it.next(),
            Some(AdStruct::LocalName(
                LocalNameAdStruct::try_new(local_name, LocalNameComplete::Shortened(3)).unwrap()
            ))
        );
        assert_eq!(
            it.next(),
            Some(AdStruct::LocalName(
                LocalNameAdStruct::try_new(local_name, LocalNameComplete::Complete).unwrap()
            ))
        );
        assert_eq!(it.next(), Some(AdStruct::Uri(UriAdStruct::new(uri))));
        assert_eq!(it.next(), None);

        Ok(())
    }

    #[test]
    fn test_full_advertising_data_success_no_scan_response_data() -> Result<(), Error> {
        let local_name = "bletio";
        let adv_data = AdvertisingData::builder()
            .with_appearance()
            .unwrap()
            .with_local_name(LocalNameComplete::Complete)
            .unwrap()
            .build();
        let full_adv_data = FullAdvertisingData::try_new(adv_data.clone(), None)?;
        assert_eq!(full_adv_data.iter().count(), 2);
        assert_eq!(full_adv_data.advertising_data().iter().count(), 2);
        assert_eq!(full_adv_data.scan_response_data(), None);
        let mut it = full_adv_data.iter();
        assert_eq!(
            it.next(),
            Some(AdStruct::Appearance(AppearanceAdStruct::new(
                AppearanceValue::GenericUnknown
            )))
        );
        assert_eq!(
            it.next(),
            Some(AdStruct::LocalName(
                LocalNameAdStruct::try_new("", LocalNameComplete::Complete).unwrap()
            ))
        );
        assert_eq!(it.next(), None);

        let appearance = AppearanceValue::Thermostat;
        let mut device_information = DeviceInformation::default();
        device_information.appearance = appearance;
        device_information.local_name = "bletio";
        let filled_full_adv_data = full_adv_data.fill_automatic_data(&device_information)?;
        assert_eq!(filled_full_adv_data.iter().count(), 2);
        assert_eq!(filled_full_adv_data.advertising_data().iter().count(), 2);
        assert_eq!(filled_full_adv_data.scan_response_data(), None);
        let mut it = filled_full_adv_data.iter();
        assert_eq!(
            it.next(),
            Some(AdStruct::Appearance(AppearanceAdStruct::new(appearance)))
        );
        assert_eq!(
            it.next(),
            Some(AdStruct::LocalName(
                LocalNameAdStruct::try_new(local_name, LocalNameComplete::Complete).unwrap()
            ))
        );
        assert_eq!(it.next(), None);

        Ok(())
    }

    #[test]
    fn test_full_advertising_data_success_fill_scan_response_data() -> Result<(), Error> {
        let local_name = "bletio";
        let uri = Uri::try_new(ProvisionedUriScheme::Https, "//example.org").unwrap();
        let adv_data = AdvertisingData::builder()
            .with_uri(uri.clone())
            .unwrap()
            .build();
        let scanresp_data = ScanResponseData::builder()
            .with_local_name(LocalNameComplete::Complete)
            .unwrap()
            .with_appearance()
            .unwrap()
            .with_tx_power_level()
            .unwrap()
            .with_le_supported_features()
            .unwrap()
            .build();
        let full_adv_data = FullAdvertisingData::try_new(adv_data.clone(), scanresp_data.clone())?;
        assert_eq!(full_adv_data.iter().count(), 5);
        assert_eq!(full_adv_data.advertising_data().iter().count(), 1);
        assert_eq!(
            full_adv_data.scan_response_data().unwrap().iter().count(),
            4
        );
        let mut it = full_adv_data.iter();
        assert_eq!(
            it.next(),
            Some(AdStruct::Uri(UriAdStruct::new(uri.clone())))
        );
        assert_eq!(
            it.next(),
            Some(AdStruct::LocalName(
                LocalNameAdStruct::try_new("", LocalNameComplete::Complete).unwrap()
            ))
        );
        assert_eq!(
            it.next(),
            Some(AdStruct::Appearance(AppearanceAdStruct::new(
                AppearanceValue::GenericUnknown
            )))
        );
        assert_eq!(
            it.next(),
            Some(AdStruct::TxPowerLevel(TxPowerLevelAdStruct::new(
                TxPowerLevel::default()
            )))
        );
        assert_eq!(
            it.next(),
            Some(AdStruct::LeSupportedFeatures(
                LeSupportedFeaturesAdStruct::new(SupportedLeFeatures::default())
            ))
        );
        assert_eq!(it.next(), None);

        let appearance = AppearanceValue::Thermostat;
        let tx_power_level = TxPowerLevel::try_new(-8).unwrap();
        let supported_le_features =
            SupportedLeFeatures::LE_2M_PHY | SupportedLeFeatures::LE_CODED_PHY;
        let mut device_information = DeviceInformation::default();
        device_information.appearance = appearance;
        device_information.local_name = "bletio";
        device_information.tx_power_level = tx_power_level;
        device_information.supported_le_features = supported_le_features;
        let filled_full_adv_data = full_adv_data.fill_automatic_data(&device_information)?;
        assert_eq!(filled_full_adv_data.iter().count(), 5);
        assert_eq!(filled_full_adv_data.advertising_data().iter().count(), 1);
        assert_eq!(
            filled_full_adv_data
                .scan_response_data()
                .unwrap()
                .iter()
                .count(),
            4
        );
        let mut it = filled_full_adv_data.iter();
        assert_eq!(it.next(), Some(AdStruct::Uri(UriAdStruct::new(uri))));
        assert_eq!(
            it.next(),
            Some(AdStruct::LocalName(
                LocalNameAdStruct::try_new(local_name, LocalNameComplete::Complete).unwrap()
            ))
        );
        assert_eq!(
            it.next(),
            Some(AdStruct::Appearance(AppearanceAdStruct::new(appearance)))
        );
        assert_eq!(
            it.next(),
            Some(AdStruct::TxPowerLevel(TxPowerLevelAdStruct::new(
                tx_power_level
            )))
        );
        assert_eq!(
            it.next(),
            Some(AdStruct::LeSupportedFeatures(
                LeSupportedFeaturesAdStruct::new(supported_le_features)
            ))
        );
        assert_eq!(it.next(), None);

        Ok(())
    }

    #[test]
    fn test_full_advertising_data_failure() {
        let adv_data = AdvertisingData::builder()
            .with_appearance()
            .unwrap()
            .build();
        let scanresp_data = ScanResponseData::builder()
            .with_appearance()
            .unwrap()
            .build();
        let err = FullAdvertisingData::try_new(adv_data, scanresp_data);
        assert_eq!(
            err,
            Err(Error::Advertising(
                AdvertisingError::AppearanceNotAllowedInBothAdvertisingDataAndScanResponseData
            ))
        );
    }

    #[rstest]
    #[case(&[0x02, 0x01, 0x06], AdStruct::Flags(FlagsAdStruct::new(Flags::LE_GENERAL_DISCOVERABLE_MODE | Flags::BREDR_NOT_SUPPORTED)))]
    #[case(
        &[0x07, 0x02, 0x03, 0x18, 0x0F, 0x18, 0x1A, 0x18],
        AdStruct::ServiceUuid16(ServiceUuid16AdStruct::try_new(
            &[ServiceUuid::LinkLoss, ServiceUuid::Battery, ServiceUuid::EnvironmentalSensing],
            ServiceListComplete::Incomplete
        ).unwrap())
    )]
    #[case(
        &[0x07, 0x03, 0x03, 0x18, 0x0F, 0x18, 0x1A, 0x18],
        AdStruct::ServiceUuid16(ServiceUuid16AdStruct::try_new(
            &[ServiceUuid::LinkLoss, ServiceUuid::Battery, ServiceUuid::EnvironmentalSensing],
            ServiceListComplete::Complete
        ).unwrap())
    )]
    #[case(
        &[0x0D, 0x04, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00, 0x1A, 0x18, 0x00, 0x00],
        AdStruct::ServiceUuid32(ServiceUuid32AdStruct::try_new(
            &[Uuid32(0x0000_1803), Uuid32(0x0000_180F), Uuid32(0x0000_181A)],
            ServiceListComplete::Incomplete
        ).unwrap())
    )]
    #[case(
        &[0x0D, 0x05, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00, 0x1A, 0x18, 0x00, 0x00],
        AdStruct::ServiceUuid32(ServiceUuid32AdStruct::try_new(
            &[Uuid32(0x0000_1803), Uuid32(0x0000_180F), Uuid32(0x0000_181A)],
            ServiceListComplete::Complete
        ).unwrap())
    )]
    #[case(
        &[0x11, 0x06, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5],
        AdStruct::ServiceUuid128(ServiceUuid128AdStruct::try_new(
            &[Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)],
            ServiceListComplete::Incomplete
        ).unwrap())
    )]
    #[case(
        &[0x11, 0x07, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5],
        AdStruct::ServiceUuid128(ServiceUuid128AdStruct::try_new(
            &[Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)],
            ServiceListComplete::Complete
        ).unwrap())
    )]
    #[case(
        &[0x06, 0x08, b'b', b'l', b'e', b't', b'i'],
        AdStruct::LocalName(LocalNameAdStruct::try_new("bletio", LocalNameComplete::Shortened(5)).unwrap())
    )]
    #[case(
        &[0x07, 0x09, b'b', b'l', b'e', b't', b'i', b'o'],
        AdStruct::LocalName(LocalNameAdStruct::try_new("bletio", LocalNameComplete::Complete).unwrap())
    )]
    #[case(&[0x02, 0x0A, 0x14], AdStruct::TxPowerLevel(TxPowerLevelAdStruct::new(TxPowerLevel::try_new(20).unwrap())))]
    #[case(
        &[0x05, 0x12, 0x06, 0x00, 0x80, 0x0C],
        AdStruct::PeripheralConnectionIntervalRange(
            PeripheralConnectionIntervalRangeAdStruct::new(0x0006.try_into().unwrap()..=0x0C80.try_into().unwrap())
        )
    )]
    #[case(
        &[0x05, 0x14, 0x03, 0x18, 0x0F, 0x18],
        AdStruct::ServiceSolicitationUuid16(
            ServiceSolicitationUuid16AdStruct::try_new(&[ServiceUuid::LinkLoss, ServiceUuid::Battery]).unwrap()
        )
    )]
    #[case(
        &[0x11, 0x15, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5],
        AdStruct::ServiceSolicitationUuid128(
            ServiceSolicitationUuid128AdStruct::try_new(&[Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)]).unwrap()
        )
    )]
    #[case(
        &[0x05, 0x16, 0x03, 0x18, 0x01, 0x14],
        AdStruct::ServiceDataUuid16(
            ServiceDataUuid16AdStruct::try_new(ServiceUuid::LinkLoss, &[0x01, 0x14]).unwrap()
        )
    )]
    #[case(
        &[0x07, 0x17, 0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56],
        AdStruct::PublicTargetAddress(
            PublicTargetAddressAdStruct::try_new(&[PublicDeviceAddress::new([0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56])]).unwrap()
        )
    )]
    #[case(
        &[0x07, 0x18, 0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0xF7],
        AdStruct::RandomTargetAddress(
            RandomTargetAddressAdStruct::try_new(&[RandomStaticDeviceAddress::try_new([0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0xF7]).unwrap().into()]).unwrap()
        )
    )]
    #[case(&[0x03, 0x19, 0x44, 0x08], AdStruct::Appearance(AppearanceAdStruct::new(AppearanceValue::StandmountedSpeaker)))]
    #[case(&[0x03, 0x1A, 0x00, 0x08], AdStruct::AdvertisingInterval(AdvertisingIntervalAdStruct::new(AdvertisingInterval::default())))]
    #[case(
        &[0x09, 0x1F, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00],
        AdStruct::ServiceSolicitationUuid32(
            ServiceSolicitationUuid32AdStruct::try_new(&[Uuid32(0x0000_1803), Uuid32(0x0000_180F)]).unwrap()
        )
    )]
    #[case(
        &[0x0F, 0x20, 0x03, 0x18, 0x00, 0x00, 0x09, 0x1F, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00],
        AdStruct::ServiceDataUuid32(
            ServiceDataUuid32AdStruct::try_new(Uuid32(0x0000_1803), &[0x09, 0x1F, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00]).unwrap()
        )
    )]
    #[case(
        &[0x15, 0x21, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5, 0xD6, 0x0F, 0x28, 0x6E],
        AdStruct::ServiceDataUuid128(
            ServiceDataUuid128AdStruct::try_new(Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640), &[0xD6, 0x0F, 0x28, 0x6E]).unwrap()
        )
    )]
    #[case(
        &[0x11, 0x24, 0x16, 0x00, b'/', b'/', b'e', b'x', b'a', b'm', b'p', b'l', b'e', b'.', b'o', b'r', b'g', b'/'],
        AdStruct::Uri(UriAdStruct::new(Uri::try_new(ProvisionedUriScheme::Http, "//example.org/").unwrap()))
    )]
    #[case(&[0x01, 0x27], AdStruct::LeSupportedFeatures(LeSupportedFeaturesAdStruct::new(SupportedLeFeatures::default())))]
    #[case(
        &[0x1E, 0xFF, 0x4C, 0x00, 0x12, 0x19, 0x00, 0x9A, 0x9A, 0xE9, 0x80, 0x96, 0x3C, 0xA0, 0x14, 0xFB, 0xE2,
            0x14, 0x41, 0x88, 0xF5, 0xDA, 0xB6, 0x07, 0x99, 0xD3, 0x15, 0x57, 0x6C, 0x01, 0x00],
        AdStruct::ManufacturerSpecificData(
            ManufacturerSpecificDataAdStruct::try_new(CompanyIdentifier::AppleInc,
            &[0x12, 0x19, 0x00, 0x9A, 0x9A, 0xE9, 0x80, 0x96, 0x3C, 0xA0, 0x14, 0xFB, 0xE2, 0x14,
                0x41, 0x88, 0xF5, 0xDA, 0xB6, 0x07, 0x99, 0xD3, 0x15, 0x57, 0x6C, 0x01, 0x00]
            ).unwrap()
        )
    )]
    #[case(&[0x01, 0x30], AdStruct::Unhandled(0x30))]
    fn test_ad_struct_parsing(#[case] input: &[u8], #[case] expected_ad_struct: AdStruct) {
        assert_eq!(
            ad_struct(input),
            Ok((&[] as &[u8], (input.len(), expected_ad_struct)))
        );
    }

    #[test]
    fn test_advertising_data_iterator() {
        let data: &[u8] = &[0x06, 0x05, 0x12, 0x80, 0x0C, 0x06, 0x00];
        let mut it = AdvertisingDataIterator {
            data,
            next_index: 0,
        };
        assert_eq!(it.next(), None);
        assert_eq!(it.next_index, 7);
    }
}
