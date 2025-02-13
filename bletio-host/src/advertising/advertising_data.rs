use core::ops::RangeInclusive;

use bletio_hci::{
    AdvertisingIntervalValue, ConnectionInterval, PublicDeviceAddress, SupportedLeFeatures,
    TxPowerLevel,
};
use bletio_utils::{BufferOps, EncodeToBuffer};

use crate::advertising::ad_struct::{
    AdvertisingIntervalAdStruct, AppearanceAdStruct, FlagsAdStruct, LeSupportedFeaturesAdStruct,
    LocalNameAdStruct, ManufacturerSpecificDataAdStruct, PeripheralConnectionIntervalRangeAdStruct,
    PublicTargetAddressAdStruct, ServiceDataUuid128AdStruct, ServiceDataUuid16AdStruct,
    ServiceDataUuid32AdStruct, ServiceSolicitationUuid128AdStruct,
    ServiceSolicitationUuid16AdStruct, ServiceSolicitationUuid32AdStruct, ServiceUuid128AdStruct,
    ServiceUuid16AdStruct, ServiceUuid32AdStruct, TxPowerLevelAdStruct, UriAdStruct,
};
use crate::advertising::{AdvertisingError, Flags, LocalNameComplete, ServiceListComplete, Uri};
use crate::assigned_numbers::{AppearanceValue, CompanyIdentifier, ServiceUuid};
use crate::uuid::{Uuid128, Uuid32};
use crate::{DeviceInformation, Error};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FullAdvertisingData<'a> {
    pub(crate) adv_data: AdvertisingData<'a>,
    pub(crate) scanresp_data: Option<ScanResponseData<'a>>,
}

impl<'a> FullAdvertisingData<'a> {
    pub fn try_new(
        adv_data: AdvertisingData<'a>,
        scanresp_data: impl Into<Option<ScanResponseData<'a>>>,
    ) -> Result<Self, Error> {
        let scanresp_data = scanresp_data.into();
        if let Some(scanresp_data) = &scanresp_data {
            if let (Some(_), Some(_)) = (
                adv_data.base.appearance.as_ref(),
                scanresp_data.base.appearance.as_ref(),
            ) {
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

    pub(crate) fn fill_automatic_data(&self, device_information: &'a DeviceInformation) -> Self {
        let mut filled = self.clone();
        filled.adv_data.base.fill_automatic_data(device_information);
        if let Some(scanresp_data) = filled.scanresp_data.as_mut() {
            scanresp_data.base.fill_automatic_data(device_information);
        }
        filled
    }
}

/// Builder to create `AdvertisingData` packets.
#[derive(Debug, Default)]
pub struct AdvertisingDataBuilder<'a> {
    data: AdvertisingData<'a>,
}

impl<'a> AdvertisingDataBuilder<'a> {
    /// Create a builder to instantiate `AdvertisingData`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the `AdvertisingData`, containing all the Advertising Structures that has been added.
    pub fn build(self) -> AdvertisingData<'a> {
        self.data
    }

    /// Add an Advertising Interval Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `interval` — The Advertising Interval value to put in the added Advertising Interval Advertising Structure.
    pub fn with_advertising_interval(mut self, interval: AdvertisingIntervalValue) -> Self {
        self.data.base.advertising_interval = Some(AdvertisingIntervalAdStruct::new(interval));
        self
    }

    /// Add an Appearance Advertising Structure to the `AdvertisingData`.
    pub fn with_appearance(mut self) -> Self {
        self.data.base.appearance = Some(AppearanceAdStruct::new(AppearanceValue::GenericUnknown));
        self
    }

    /// Add a Flags Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `flags` — The Flags value to put in the added Flags Advertising Structure.
    pub fn with_flags(mut self, flags: Flags) -> Self {
        self.data.flags = Some(FlagsAdStruct::new(flags));
        self
    }

    /// Add a LE Supported Features Advertising Structure to the `AdvertisingData`.
    pub fn with_le_supported_features(mut self) -> Self {
        self.data.base.le_supported_features = Some(LeSupportedFeaturesAdStruct::new(
            SupportedLeFeatures::default(),
        ));
        self
    }

    /// Add a Local Name Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `complete` — Whether the local name should be put complete or shortened in the added Local Name Advertising Structure.
    pub fn with_local_name(mut self, complete: LocalNameComplete) -> Self {
        self.data.base.local_name = Some(LocalNameAdStruct::new("", complete));
        self
    }

    /// Add a Manufacturer Specific Data Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `manufacturer` — The `CompanyIdentifier` to put in the added Manufacturer Specific Data Advertising Structure.
    /// * `data` — The data to put in the added Manufacturer Specific Data Advertising Structure.
    pub fn with_manufacturer_specific_data(
        mut self,
        manufacturer: CompanyIdentifier,
        data: &'a [u8],
    ) -> Self {
        self.data.base.manufacturer_specific_data =
            Some(ManufacturerSpecificDataAdStruct::new(manufacturer, data));
        self
    }

    /// Add a Peripheral Connection Interval Range Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `range` — The Connection Interval value to put in the added Peripheral Connection Range Advertising Structure.
    pub fn with_peripheral_connection_interval_range(
        mut self,
        range: RangeInclusive<ConnectionInterval>,
    ) -> Self {
        self.data.base.peripheral_connection_interval =
            Some(PeripheralConnectionIntervalRangeAdStruct::new(range));
        self
    }

    /// Add a Public Target Address Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `addresses` — The list of public device addresses to put in the added Public Target Address Advertising Structure.
    pub fn with_public_target_address(
        mut self,
        addresses: &'a [PublicDeviceAddress],
    ) -> Result<Self, AdvertisingError> {
        self.data.base.public_target_address =
            Some(PublicTargetAddressAdStruct::try_new(addresses)?);
        Ok(self)
    }

    /// Add a Service Data for a 16-bit Service UUID Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `uuid` — The 16-bit Service UUID to put in the added Service Data Advertising Structure.
    /// * `data` — The data to put in the added Service Data Advertising Structure.
    pub fn with_service_data_uuid16(mut self, uuid: ServiceUuid, data: &'a [u8]) -> Self {
        self.data.base.service_data_uuid16 = Some(ServiceDataUuid16AdStruct::new(uuid, data));
        self
    }

    /// Add a Service Data for a 32-bit Service UUID Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `uuid` — The 32-bit Service UUID to put in the added Service Data Advertising Structure.
    /// * `data` — The data to put in the added Service Data Advertising Structure.
    pub fn with_service_data_uuid32(mut self, uuid: Uuid32, data: &'a [u8]) -> Self {
        self.data.base.service_data_uuid32 = Some(ServiceDataUuid32AdStruct::new(uuid, data));
        self
    }

    /// Add a Service Data for a 128-bit Service UUID Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `uuid` — The 128-bit Service UUID to put in the added Service Data Advertising Structure.
    /// * `data` — The data to put in the added Service Data Advertising Structure.
    pub fn with_service_data_uuid128(mut self, uuid: Uuid128, data: &'a [u8]) -> Self {
        self.data.base.service_data_uuid128 = Some(ServiceDataUuid128AdStruct::new(uuid, data));
        self
    }

    /// Add a list of 16-bit Service Solicitation UUIDs Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `uuids` — The list of 16-bit Service UUIDs to put in the added Service Solicitation UUID16 Advertising Structure.
    pub fn with_service_solicitation_uuid16(
        mut self,
        uuids: &'a [ServiceUuid],
    ) -> Result<Self, AdvertisingError> {
        self.data.base.service_solicitation_uuid16 =
            Some(ServiceSolicitationUuid16AdStruct::new(uuids));
        Ok(self)
    }

    /// Add a list of 32-bit Service Solicitation UUIDs Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `uuids` — The list of 32-bit Service UUIDs to put in the added Service Solicitation UUID32 Advertising Structure.
    pub fn with_service_solicitation_uuid32(
        mut self,
        uuids: &'a [Uuid32],
    ) -> Result<Self, AdvertisingError> {
        self.data.base.service_solicitation_uuid32 =
            Some(ServiceSolicitationUuid32AdStruct::new(uuids));
        Ok(self)
    }

    /// Add a list of 128-bit Service Solicitation UUIDs Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `uuids` — The list of 128-bit Service UUIDs to put in the added Service Solicitation UUID128 Advertising Structure.
    pub fn with_service_solicitation_uuid128(
        mut self,
        uuids: &'a [Uuid128],
    ) -> Result<Self, AdvertisingError> {
        self.data.base.service_solicitation_uuid128 =
            Some(ServiceSolicitationUuid128AdStruct::new(uuids));
        Ok(self)
    }

    /// Add a list of 16-bit Service UUIDs Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `uuids` — The list of 16-bit Service UUIDs to put in the added Service UUID16 Advertising Structure.
    /// * `complete` — Whether the provided list is complete or not.
    pub fn with_service_uuid16(
        mut self,
        uuids: &'a [ServiceUuid],
        complete: ServiceListComplete,
    ) -> Result<Self, AdvertisingError> {
        self.data.base.service_uuid16 = Some(ServiceUuid16AdStruct::try_new(uuids, complete)?);
        Ok(self)
    }

    /// Add a list of 32-bit Service UUIDs Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `uuids` — The list of 32-bit Service UUIDs to put in the added Service UUID32 Advertising Structure.
    /// * `complete` — Whether the provided list is complete or not.
    pub fn with_service_uuid32(
        mut self,
        uuids: &'a [Uuid32],
        complete: ServiceListComplete,
    ) -> Result<Self, AdvertisingError> {
        self.data.base.service_uuid32 = Some(ServiceUuid32AdStruct::try_new(uuids, complete)?);
        Ok(self)
    }

    /// Add a list of 128-bit Service UUIDs Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `uuids` — The list of 128-bit Service UUIDs to put in the added Service UUID128 Advertising Structure.
    /// * `complete` — Whether the provided list is complete or not.
    pub fn with_service_uuid128(
        mut self,
        uuids: &'a [Uuid128],
        complete: ServiceListComplete,
    ) -> Result<Self, AdvertisingError> {
        self.data.base.service_uuid128 = Some(ServiceUuid128AdStruct::try_new(uuids, complete)?);
        Ok(self)
    }

    /// Add a TX Power Level Advertising Structure to the `AdvertisingData`.
    pub fn with_tx_power_level(mut self) -> Self {
        self.data.base.tx_power_level = Some(TxPowerLevelAdStruct::new(TxPowerLevel::default()));
        self
    }

    /// Add a Uri Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `uri` — The Uri to put in the added Uri Advertising Structure.
    pub fn with_uri(mut self, uri: Uri) -> Self {
        self.data.base.uri = Some(UriAdStruct::new(uri));
        self
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AdvertisingDataBase<'a> {
    advertising_interval: Option<AdvertisingIntervalAdStruct>,
    appearance: Option<AppearanceAdStruct>,
    le_supported_features: Option<LeSupportedFeaturesAdStruct>,
    local_name: Option<LocalNameAdStruct<'a>>,
    manufacturer_specific_data: Option<ManufacturerSpecificDataAdStruct<'a>>,
    peripheral_connection_interval: Option<PeripheralConnectionIntervalRangeAdStruct>,
    public_target_address: Option<PublicTargetAddressAdStruct<'a>>,
    service_data_uuid16: Option<ServiceDataUuid16AdStruct<'a>>,
    service_data_uuid32: Option<ServiceDataUuid32AdStruct<'a>>,
    service_data_uuid128: Option<ServiceDataUuid128AdStruct<'a>>,
    service_uuid16: Option<ServiceUuid16AdStruct<'a>>,
    service_uuid32: Option<ServiceUuid32AdStruct<'a>>,
    service_uuid128: Option<ServiceUuid128AdStruct<'a>>,
    service_solicitation_uuid16: Option<ServiceSolicitationUuid16AdStruct<'a>>,
    service_solicitation_uuid32: Option<ServiceSolicitationUuid32AdStruct<'a>>,
    service_solicitation_uuid128: Option<ServiceSolicitationUuid128AdStruct<'a>>,
    tx_power_level: Option<TxPowerLevelAdStruct>,
    uri: Option<UriAdStruct>,
}

impl<'a> AdvertisingDataBase<'a> {
    fn fill_automatic_data(&mut self, device_information: &'a DeviceInformation) {
        if self.appearance.is_some() {
            self.appearance = Some(AppearanceAdStruct::new(device_information.appearance));
        }
        if self.le_supported_features.is_some() {
            self.le_supported_features = Some(LeSupportedFeaturesAdStruct::new(
                device_information.supported_le_features,
            ));
        }
        if let Some(local_name) = self.local_name.clone() {
            self.local_name = Some(LocalNameAdStruct::new(
                device_information.local_name,
                local_name.complete,
            ));
        }
        if self.tx_power_level.is_some() {
            self.tx_power_level =
                Some(TxPowerLevelAdStruct::new(device_information.tx_power_level));
        }
    }
}

impl EncodeToBuffer for AdvertisingDataBase<'_> {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, bletio_utils::Error> {
        if let Some(interval) = self.advertising_interval.as_ref() {
            interval.encode(buffer)?;
        }
        if let Some(appearance) = self.appearance.as_ref() {
            appearance.encode(buffer)?;
        }
        if let Some(interval) = self.peripheral_connection_interval.as_ref() {
            interval.encode(buffer)?;
        }
        if let Some(features) = self.le_supported_features.as_ref() {
            features.encode(buffer)?;
        }
        if let Some(local_name) = self.local_name.as_ref() {
            local_name.encode(buffer)?;
        }
        if let Some(data) = self.manufacturer_specific_data.as_ref() {
            data.encode(buffer)?;
        }
        if let Some(addresses) = self.public_target_address.as_ref() {
            addresses.encode(buffer)?;
        }
        if let Some(service_data_uuid16) = self.service_data_uuid16.as_ref() {
            service_data_uuid16.encode(buffer)?;
        }
        if let Some(service_data_uuid32) = self.service_data_uuid32.as_ref() {
            service_data_uuid32.encode(buffer)?;
        }
        if let Some(service_data_uuid128) = self.service_data_uuid128.as_ref() {
            service_data_uuid128.encode(buffer)?;
        }
        if let Some(service_solicitation_uuid16) = self.service_solicitation_uuid16.as_ref() {
            service_solicitation_uuid16.encode(buffer)?;
        }
        if let Some(service_solicitation_uuid32) = self.service_solicitation_uuid32.as_ref() {
            service_solicitation_uuid32.encode(buffer)?;
        }
        if let Some(service_solicitation_uuid128) = self.service_solicitation_uuid128.as_ref() {
            service_solicitation_uuid128.encode(buffer)?;
        }
        if let Some(service_uuid16) = self.service_uuid16.as_ref() {
            service_uuid16.encode(buffer)?;
        }
        if let Some(service_uuid32) = self.service_uuid32.as_ref() {
            service_uuid32.encode(buffer)?;
        }
        if let Some(service_uuid128) = self.service_uuid128.as_ref() {
            service_uuid128.encode(buffer)?;
        }
        if let Some(tx_power_level) = self.tx_power_level.as_ref() {
            tx_power_level.encode(buffer)?;
        }
        if let Some(uri) = self.uri.as_ref() {
            uri.encode(buffer)?;
        }
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        let mut len = 0;
        if let Some(interval) = self.advertising_interval.as_ref() {
            len += interval.encoded_size();
        }
        if let Some(appearance) = self.appearance.as_ref() {
            len += appearance.encoded_size();
        }
        if let Some(interval) = self.peripheral_connection_interval.as_ref() {
            len += interval.encoded_size();
        }
        if let Some(features) = self.le_supported_features.as_ref() {
            len += features.encoded_size();
        }
        if let Some(local_name) = self.local_name.as_ref() {
            len += local_name.encoded_size();
        }
        if let Some(data) = self.manufacturer_specific_data.as_ref() {
            len += data.encoded_size();
        }
        if let Some(addresses) = self.public_target_address.as_ref() {
            len += addresses.encoded_size();
        }
        if let Some(service_data_uuid16) = self.service_data_uuid16.as_ref() {
            len += service_data_uuid16.encoded_size();
        }
        if let Some(service_data_uuid32) = self.service_data_uuid32.as_ref() {
            len += service_data_uuid32.encoded_size();
        }
        if let Some(service_data_uuid128) = self.service_data_uuid128.as_ref() {
            len += service_data_uuid128.encoded_size();
        }
        if let Some(service_solicitation_uuid16) = self.service_solicitation_uuid16.as_ref() {
            len += service_solicitation_uuid16.encoded_size();
        }
        if let Some(service_solicitation_uuid32) = self.service_solicitation_uuid32.as_ref() {
            len += service_solicitation_uuid32.encoded_size();
        }
        if let Some(service_solicitation_uuid128) = self.service_solicitation_uuid128.as_ref() {
            len += service_solicitation_uuid128.encoded_size();
        }
        if let Some(service_uuid16) = self.service_uuid16.as_ref() {
            len += service_uuid16.encoded_size();
        }
        if let Some(service_uuid32) = self.service_uuid32.as_ref() {
            len += service_uuid32.encoded_size();
        }
        if let Some(service_uuid128) = self.service_uuid128.as_ref() {
            len += service_uuid128.encoded_size();
        }
        if let Some(tx_power_level) = self.tx_power_level.as_ref() {
            len += tx_power_level.encoded_size();
        }
        if let Some(uri) = self.uri.as_ref() {
            len += uri.encoded_size();
        }
        len
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
pub struct AdvertisingData<'a> {
    base: AdvertisingDataBase<'a>,
    flags: Option<FlagsAdStruct>,
}

impl<'a> AdvertisingData<'a> {
    /// Instantiate a builder to create Advertising Data.
    pub fn builder() -> AdvertisingDataBuilder<'a> {
        AdvertisingDataBuilder::new()
    }
}

impl EncodeToBuffer for AdvertisingData<'_> {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, bletio_utils::Error> {
        if let Some(flags) = self.flags.as_ref() {
            flags.encode(buffer)?;
        }
        self.base.encode(buffer)?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        let mut len = 0;
        if let Some(flags) = self.flags.as_ref() {
            len += flags.encoded_size();
        }
        len += self.base.encoded_size();
        len
    }
}

impl TryFrom<&AdvertisingData<'_>> for bletio_hci::AdvertisingData {
    type Error = AdvertisingError;

    fn try_from(value: &AdvertisingData) -> Result<Self, Self::Error> {
        fn inner(
            value: &AdvertisingData,
        ) -> Result<bletio_hci::AdvertisingData, bletio_utils::Error> {
            let mut adv_data = bletio_hci::AdvertisingData::default();
            adv_data.fill(|buffer| value.encode(buffer))?;
            Ok(adv_data)
        }

        inner(value).map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
    }
}

/// Builder to create `ScanResponseData` packets.
#[derive(Debug, Default)]
pub struct ScanResponseDataBuilder<'a> {
    data: ScanResponseData<'a>,
}

impl<'a> ScanResponseDataBuilder<'a> {
    /// Create a builder to instantiate `ScanResponseData`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the `ScanResponseData`, containing all the Advertising Structures that has been added.
    pub fn build(self) -> ScanResponseData<'a> {
        self.data
    }

    /// Add an Advertising Interval Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `interval` — The Advertising Interval value to put in the added Advertising Interval Advertising Structure.
    pub fn with_advertising_interval(mut self, interval: AdvertisingIntervalValue) -> Self {
        self.data.base.advertising_interval = Some(AdvertisingIntervalAdStruct::new(interval));
        self
    }

    /// Add an Appearance Advertising Structure to the `ScanResponseData`.
    pub fn with_appearance(mut self) -> Self {
        self.data.base.appearance = Some(AppearanceAdStruct::new(AppearanceValue::GenericUnknown));
        self
    }

    /// Add a LE Supported Features Advertising Structure to the `ScanResponseData`.
    pub fn with_le_supported_features(mut self) -> Self {
        self.data.base.le_supported_features = Some(LeSupportedFeaturesAdStruct::new(
            SupportedLeFeatures::default(),
        ));
        self
    }

    /// Add a Local Name Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `complete` — Whether the local name should be put complete or shortened in the added Local Name Advertising Structure.
    pub fn with_local_name(mut self, complete: LocalNameComplete) -> Self {
        self.data.base.local_name = Some(LocalNameAdStruct::new("", complete));
        self
    }

    /// Add a Manufacturer Specific Data Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `manufacturer` — The `CompanyIdentifier` to put in the added Manufacturer Specific Data Advertising Structure.
    /// * `data` — The data to put in the added Manufacturer Specific Data Advertising Structure.
    pub fn with_manufacturer_specific_data(
        mut self,
        manufacturer: CompanyIdentifier,
        data: &'a [u8],
    ) -> Self {
        self.data.base.manufacturer_specific_data =
            Some(ManufacturerSpecificDataAdStruct::new(manufacturer, data));
        self
    }

    /// Add a Peripheral Connection Interval Range Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `range` — The Connection Interval value to put in the added Peripheral Connection Range Advertising Structure.
    pub fn with_peripheral_connection_interval_range(
        mut self,
        range: RangeInclusive<ConnectionInterval>,
    ) -> Self {
        self.data.base.peripheral_connection_interval =
            Some(PeripheralConnectionIntervalRangeAdStruct::new(range));
        self
    }

    /// Add a Public Target Address Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `addresses` — The list of public device addresses to put in the added Public Target Address Advertising Structure.
    pub fn with_public_target_address(
        mut self,
        addresses: &'a [PublicDeviceAddress],
    ) -> Result<Self, AdvertisingError> {
        self.data.base.public_target_address =
            Some(PublicTargetAddressAdStruct::try_new(addresses)?);
        Ok(self)
    }

    /// Add a Service Data for a 16-bit Service UUID Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `uuid` — The 16-bit Service UUID to put in the added Service Data Advertising Structure.
    /// * `data` — The data to put in the added Service Data Advertising Structure.
    pub fn with_service_data_uuid16(mut self, uuid: ServiceUuid, data: &'a [u8]) -> Self {
        self.data.base.service_data_uuid16 = Some(ServiceDataUuid16AdStruct::new(uuid, data));
        self
    }

    /// Add a Service Data for a 32-bit Service UUID Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `uuid` — The 32-bit Service UUID to put in the added Service Data Advertising Structure.
    /// * `data` — The data to put in the added Service Data Advertising Structure.
    pub fn with_service_data_uuid32(mut self, uuid: Uuid32, data: &'a [u8]) -> Self {
        self.data.base.service_data_uuid32 = Some(ServiceDataUuid32AdStruct::new(uuid, data));
        self
    }

    /// Add a Service Data for a 128-bit Service UUID Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `uuid` — The 128-bit Service UUID to put in the added Service Data Advertising Structure.
    /// * `data` — The data to put in the added Service Data Advertising Structure.
    pub fn with_service_data_uuid128(mut self, uuid: Uuid128, data: &'a [u8]) -> Self {
        self.data.base.service_data_uuid128 = Some(ServiceDataUuid128AdStruct::new(uuid, data));
        self
    }

    /// Add a list of 16-bit Service Solicitation UUIDs Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `uuids` — The list of 16-bit Service UUIDs to put in the added Service Solicitation UUID16 Advertising Structure.
    pub fn with_service_solicitation_uuid16(
        mut self,
        uuids: &'a [ServiceUuid],
    ) -> Result<Self, AdvertisingError> {
        self.data.base.service_solicitation_uuid16 =
            Some(ServiceSolicitationUuid16AdStruct::new(uuids));
        Ok(self)
    }

    /// Add a list of 32-bit Service Solicitation UUIDs Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `uuids` — The list of 32-bit Service UUIDs to put in the added Service Solicitation UUID32 Advertising Structure.
    pub fn with_service_solicitation_uuid32(
        mut self,
        uuids: &'a [Uuid32],
    ) -> Result<Self, AdvertisingError> {
        self.data.base.service_solicitation_uuid32 =
            Some(ServiceSolicitationUuid32AdStruct::new(uuids));
        Ok(self)
    }

    /// Add a list of 128-bit Service Solicitation UUIDs Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `uuids` — The list of 128-bit Service UUIDs to put in the added Service Solicitation UUID128 Advertising Structure.
    pub fn with_service_solicitation_uuid128(
        mut self,
        uuids: &'a [Uuid128],
    ) -> Result<Self, AdvertisingError> {
        self.data.base.service_solicitation_uuid128 =
            Some(ServiceSolicitationUuid128AdStruct::new(uuids));
        Ok(self)
    }

    /// Add a list of 16-bit Service UUIDs Advertising Structure to the `AdvertisingData`.
    ///
    /// # Arguments
    ///
    /// * `uuids` — The list of 16-bit Service UUIDs to put in the added Service UUID16 Advertising Structure.
    /// * `complete` — Whether the provided list is complete or not.
    pub fn with_service_uuid16(
        mut self,
        uuids: &'a [ServiceUuid],
        complete: ServiceListComplete,
    ) -> Result<Self, AdvertisingError> {
        self.data.base.service_uuid16 = Some(ServiceUuid16AdStruct::try_new(uuids, complete)?);
        Ok(self)
    }

    /// Add a list of 32-bit Service UUIDs Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `uuids` — The list of 32-bit Service UUIDs to put in the added Service UUID32 Advertising Structure.
    /// * `complete` — Whether the provided list is complete or not.
    pub fn with_service_uuid32(
        mut self,
        uuids: &'a [Uuid32],
        complete: ServiceListComplete,
    ) -> Result<Self, AdvertisingError> {
        self.data.base.service_uuid32 = Some(ServiceUuid32AdStruct::try_new(uuids, complete)?);
        Ok(self)
    }

    /// Add a list of 128-bit Service UUIDs Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `uuids` — The list of 128-bit Service UUIDs to put in the added Service UUID128 Advertising Structure.
    /// * `complete` — Whether the provided list is complete or not.
    pub fn with_service_uuid128(
        mut self,
        uuids: &'a [Uuid128],
        complete: ServiceListComplete,
    ) -> Result<Self, AdvertisingError> {
        self.data.base.service_uuid128 = Some(ServiceUuid128AdStruct::try_new(uuids, complete)?);
        Ok(self)
    }

    /// Add a TX Power Level Advertising Structure to the `ScanResponseData`.
    pub fn with_tx_power_level(mut self) -> Self {
        self.data.base.tx_power_level = Some(TxPowerLevelAdStruct::new(TxPowerLevel::default()));
        self
    }

    /// Add a Uri Advertising Structure to the `ScanResponseData`.
    ///
    /// # Arguments
    ///
    /// * `uri` — The Uri to put in the added Uri Advertising Structure.
    pub fn with_uri(mut self, uri: Uri) -> Self {
        self.data.base.uri = Some(UriAdStruct::new(uri));
        self
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
pub struct ScanResponseData<'a> {
    base: AdvertisingDataBase<'a>,
}

impl<'a> ScanResponseData<'a> {
    /// Instantiate a builder to create Scan Response Data.
    pub fn builder() -> ScanResponseDataBuilder<'a> {
        ScanResponseDataBuilder::new()
    }
}

impl EncodeToBuffer for ScanResponseData<'_> {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, bletio_utils::Error> {
        self.base.encode(buffer)
    }

    fn encoded_size(&self) -> usize {
        self.base.encoded_size()
    }
}

impl TryFrom<&ScanResponseData<'_>> for bletio_hci::ScanResponseData {
    type Error = AdvertisingError;

    fn try_from(value: &ScanResponseData) -> Result<Self, Self::Error> {
        fn inner(
            value: &ScanResponseData,
        ) -> Result<bletio_hci::ScanResponseData, bletio_utils::Error> {
            let mut scanresp_data = bletio_hci::ScanResponseData::default();
            scanresp_data.fill(|buffer| value.encode(buffer))?;
            Ok(scanresp_data)
        }

        inner(value).map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
    }
}

#[cfg(test)]
mod test {
    use bletio_utils::{Buffer, BufferOps};
    use rstest::{fixture, rstest};

    use crate::assigned_numbers::ProvisionedUriScheme;

    use super::*;

    #[test]
    fn test_advertising_data_base_default() -> Result<(), bletio_utils::Error> {
        let adv_data_base = AdvertisingDataBase::default();
        assert_eq!(adv_data_base.advertising_interval, None);
        assert_eq!(adv_data_base.appearance, None);
        assert_eq!(adv_data_base.le_supported_features, None);
        assert_eq!(adv_data_base.local_name, None);
        assert_eq!(adv_data_base.manufacturer_specific_data, None);
        assert_eq!(adv_data_base.peripheral_connection_interval, None);
        assert_eq!(adv_data_base.public_target_address, None);
        assert_eq!(adv_data_base.service_data_uuid16, None);
        assert_eq!(adv_data_base.service_data_uuid32, None);
        assert_eq!(adv_data_base.service_data_uuid128, None);
        assert_eq!(adv_data_base.service_uuid16, None);
        assert_eq!(adv_data_base.service_uuid32, None);
        assert_eq!(adv_data_base.service_uuid128, None);
        assert_eq!(adv_data_base.service_solicitation_uuid16, None);
        assert_eq!(adv_data_base.service_solicitation_uuid32, None);
        assert_eq!(adv_data_base.service_solicitation_uuid128, None);
        assert_eq!(adv_data_base.tx_power_level, None);
        assert_eq!(adv_data_base.uri, None);

        let mut buffer = Buffer::<16>::default();
        assert_eq!(adv_data_base.encoded_size(), 0);
        assert_eq!(adv_data_base.encode(&mut buffer)?, 0);
        assert_eq!(buffer.data(), &[]);

        Ok(())
    }

    #[fixture]
    fn advertising_data_builder_empty<'a>() -> AdvertisingData<'a> {
        let builder = AdvertisingData::builder();
        assert_eq!(builder.data, AdvertisingData::default());
        builder.build()
    }

    #[fixture]
    fn advertising_data_builder_service_uuid16<'a>() -> AdvertisingData<'a> {
        let uuids = &[ServiceUuid::Battery, ServiceUuid::BloodPressure];
        let builder = AdvertisingData::builder()
            .with_advertising_interval(AdvertisingIntervalValue::default())
            .with_appearance()
            .with_local_name(LocalNameComplete::Complete)
            .with_service_data_uuid16(ServiceUuid::Battery, &[0x50, 0x84, 0x91, 0xAF])
            .with_service_uuid16(uuids, ServiceListComplete::Complete)
            .unwrap();
        assert_eq!(
            builder.data.base.advertising_interval,
            Some(AdvertisingIntervalAdStruct::new(
                AdvertisingIntervalValue::default()
            ))
        );
        assert_eq!(
            builder.data.base.appearance,
            Some(AppearanceAdStruct::new(AppearanceValue::GenericUnknown))
        );
        assert_eq!(
            builder.data.base.local_name,
            Some(LocalNameAdStruct::new("", LocalNameComplete::Complete))
        );
        assert_eq!(
            builder.data.base.service_uuid16,
            Some(ServiceUuid16AdStruct::try_new(uuids, ServiceListComplete::Complete).unwrap())
        );
        builder.build()
    }

    #[fixture]
    fn advertising_data_builder_service_uuid32<'a>() -> AdvertisingData<'a> {
        let flags = Flags::BREDR_NOT_SUPPORTED | Flags::LE_GENERAL_DISCOVERABLE_MODE;
        let uuids = &[Uuid32(0x0000_1803), Uuid32(0x0000_180F)];
        const ADDRESSES: &[PublicDeviceAddress] = &[PublicDeviceAddress::new([
            0xF4, 0x23, 0x14, 0xC3, 0xDC, 0x24,
        ])];
        let builder = AdvertisingData::builder()
            .with_flags(flags)
            .with_le_supported_features()
            .with_local_name(LocalNameComplete::Shortened(9))
            .with_public_target_address(ADDRESSES)
            .unwrap()
            .with_service_uuid32(uuids, ServiceListComplete::Incomplete)
            .unwrap();
        assert_eq!(builder.data.flags, Some(FlagsAdStruct::new(flags)));
        assert_eq!(
            builder.data.base.le_supported_features,
            Some(LeSupportedFeaturesAdStruct::new(
                SupportedLeFeatures::default()
            ))
        );
        assert_eq!(
            builder.data.base.local_name,
            Some(LocalNameAdStruct::new("", LocalNameComplete::Shortened(9)))
        );
        assert_eq!(
            builder.data.base.public_target_address,
            Some(PublicTargetAddressAdStruct::try_new(ADDRESSES).unwrap())
        );
        assert_eq!(
            builder.data.base.service_uuid32,
            Some(ServiceUuid32AdStruct::try_new(uuids, ServiceListComplete::Incomplete).unwrap())
        );
        builder.build()
    }

    #[fixture]
    fn advertising_data_builder_service_uuid128<'a>() -> AdvertisingData<'a> {
        let connection_interval = 0x0006.try_into().unwrap()..=0x0010.try_into().unwrap();
        let uuids = &[Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)];
        let builder = AdvertisingData::builder()
            .with_peripheral_connection_interval_range(connection_interval.clone())
            .with_tx_power_level()
            .with_service_uuid128(uuids, ServiceListComplete::Complete)
            .unwrap();
        assert_eq!(
            builder.data.base.peripheral_connection_interval,
            Some(PeripheralConnectionIntervalRangeAdStruct::new(
                connection_interval
            ))
        );
        assert_eq!(
            builder.data.base.tx_power_level,
            Some(TxPowerLevelAdStruct::new(TxPowerLevel::default()))
        );
        assert_eq!(
            builder.data.base.service_uuid128,
            Some(ServiceUuid128AdStruct::try_new(uuids, ServiceListComplete::Complete).unwrap())
        );
        builder.build()
    }

    #[fixture]
    fn advertising_data_builder_service_solicitation_uuid16<'a>() -> AdvertisingData<'a> {
        let uri = Uri::new(ProvisionedUriScheme::Https, "//example.org/");
        let uuids = &[ServiceUuid::Battery, ServiceUuid::BloodPressure];
        let builder = AdvertisingData::builder()
            .with_uri(uri.clone())
            .with_service_solicitation_uuid16(uuids)
            .unwrap();
        assert_eq!(builder.data.base.uri, Some(UriAdStruct::new(uri)));
        assert_eq!(
            builder.data.base.service_solicitation_uuid16,
            Some(ServiceSolicitationUuid16AdStruct::new(uuids))
        );
        builder.build()
    }

    #[fixture]
    fn advertising_data_builder_service_solicitation_uuid32<'a>() -> AdvertisingData<'a> {
        let data = &[0x9E, 0xF5, 0x40, 0x7C, 0x0F];
        let uuids = &[Uuid32(0x0000_1803), Uuid32(0x0000_180F)];
        let builder = AdvertisingData::builder()
            .with_manufacturer_specific_data(CompanyIdentifier::StMicroelectronics, data)
            .with_service_data_uuid32(Uuid32(0x0000_1803), &[0x50, 0x84, 0x91, 0xAF])
            .with_service_solicitation_uuid32(uuids)
            .unwrap();
        assert_eq!(
            builder.data.base.manufacturer_specific_data,
            Some(ManufacturerSpecificDataAdStruct::new(
                CompanyIdentifier::StMicroelectronics,
                data
            ))
        );
        assert_eq!(
            builder.data.base.service_solicitation_uuid32,
            Some(ServiceSolicitationUuid32AdStruct::new(uuids))
        );
        builder.build()
    }

    #[fixture]
    fn advertising_data_builder_service_solicitation_uuid128<'a>() -> AdvertisingData<'a> {
        let uuids = &[Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)];
        let builder = AdvertisingData::builder()
            .with_service_solicitation_uuid128(uuids)
            .unwrap();
        assert_eq!(
            builder.data.base.service_solicitation_uuid128,
            Some(ServiceSolicitationUuid128AdStruct::new(uuids))
        );
        builder.build()
    }

    #[fixture]
    fn advertising_data_builder_service_data_uuid128<'a>() -> AdvertisingData<'a> {
        let uuid = Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640);
        let data: &[u8] = &[0x50, 0x84, 0x91, 0xAF];
        let builder = AdvertisingData::builder().with_service_data_uuid128(uuid, data);
        assert_eq!(
            builder.data.base.service_data_uuid128,
            Some(ServiceDataUuid128AdStruct::new(uuid, data))
        );
        builder.build()
    }

    #[rstest]
    #[case::empty(advertising_data_builder_empty(), 0, &[])]
    #[case::service_uuid16(
        advertising_data_builder_service_uuid16(),
        24, &[0x03, 0x1A, 0x00, 0x08, 0x03, 0x19, 0x00, 0x00, 0x01, 0x09, 0x07, 0x16, 0x0F, 0x18,
            0x50, 0x84, 0x91, 0xAF, 0x05, 0x03, 0x0F, 0x18, 0x10, 0x18]
    )]
    #[case::service_uuid32(
        advertising_data_builder_service_uuid32(),
        25, &[0x02, 0x01, 0x06, 0x01, 0x27, 0x01, 0x08, 0x07, 0x17, 0xF4, 0x23, 0x14, 0xC3, 0xDC, 0x24,
            0x09, 0x04, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00]
    )]
    #[case::service_uuid128(
        advertising_data_builder_service_uuid128(),
        27, &[0x05, 0x12, 0x06, 0x00, 0x10, 0x00, 0x11, 0x07, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C,
            0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5, 0x02, 0x0A, 0x00]
    )]
    #[case::service_solicitation_uuid16(
        advertising_data_builder_service_solicitation_uuid16(),
        24, &[0x05, 0x14, 0x0F, 0x18, 0x10, 0x18, 0x11, 0x24, 0x17, 0x00,
            b'/', b'/', b'e', b'x', b'a', b'm', b'p', b'l', b'e', b'.', b'o', b'r', b'g', b'/']
    )]
    #[case::service_solicitation_uuid32(
        advertising_data_builder_service_solicitation_uuid32(),
        29, &[0x08, 0xFF, 0x30, 0x00, 0x9E, 0xF5, 0x40, 0x7C, 0x0F, 0x09, 0x20, 0x03, 0x18, 0x00,
            0x00, 0x50, 0x84, 0x91, 0xAF, 0x09, 0x1F, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00]
    )]
    #[case::service_solicitation_uuid128(
        advertising_data_builder_service_solicitation_uuid128(),
        18, &[0x11, 0x15, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5]
    )]
    #[case::service_data_uuid128(
        advertising_data_builder_service_data_uuid128(),
        22, &[0x15, 0x21, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5, 0x50, 0x84, 0x91, 0xAF]
    )]
    fn test_advertising_data_builder(
        #[case] adv_data: AdvertisingData,
        #[case] expected_encoded_size: usize,
        #[case] expected_encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<31>::default();
        assert_eq!(adv_data.encoded_size(), expected_encoded_size);
        assert_eq!(adv_data.encode(&mut buffer)?, expected_encoded_size);
        assert_eq!(buffer.data(), expected_encoded_data);

        let mut hci_expected_encoded_data = [0u8; 32];
        hci_expected_encoded_data[0] = expected_encoded_size as u8;
        hci_expected_encoded_data[1..1 + expected_encoded_size].copy_from_slice(buffer.data());
        let mut buffer = Buffer::<32>::default();
        let hci_advertising_data: bletio_hci::AdvertisingData = (&adv_data).try_into().unwrap();
        assert_eq!(hci_advertising_data.encoded_size(), 32);
        assert_eq!(hci_advertising_data.encode(&mut buffer)?, 32);
        assert_eq!(buffer.data(), &hci_expected_encoded_data);

        Ok(())
    }

    #[fixture]
    fn scan_response_data_builder_empty<'a>() -> ScanResponseData<'a> {
        let builder = ScanResponseData::builder();
        assert_eq!(builder.data, ScanResponseData::default());
        builder.build()
    }

    #[fixture]
    fn scan_response_data_builder_service_uuid16<'a>() -> ScanResponseData<'a> {
        let uuids = &[ServiceUuid::Battery, ServiceUuid::BloodPressure];
        let builder = ScanResponseData::builder()
            .with_advertising_interval(AdvertisingIntervalValue::default())
            .with_appearance()
            .with_local_name(LocalNameComplete::Complete)
            .with_service_data_uuid16(ServiceUuid::Battery, &[0x50, 0x84, 0x91, 0xAF])
            .with_service_uuid16(uuids, ServiceListComplete::Complete)
            .unwrap();
        assert_eq!(
            builder.data.base.advertising_interval,
            Some(AdvertisingIntervalAdStruct::new(
                AdvertisingIntervalValue::default()
            ))
        );
        assert_eq!(
            builder.data.base.appearance,
            Some(AppearanceAdStruct::new(AppearanceValue::GenericUnknown))
        );
        assert_eq!(
            builder.data.base.local_name,
            Some(LocalNameAdStruct::new("", LocalNameComplete::Complete))
        );
        assert_eq!(
            builder.data.base.service_uuid16,
            Some(ServiceUuid16AdStruct::try_new(uuids, ServiceListComplete::Complete).unwrap())
        );
        builder.build()
    }

    #[fixture]
    fn scan_response_data_builder_service_uuid32<'a>() -> ScanResponseData<'a> {
        let uuids = &[Uuid32(0x0000_1803), Uuid32(0x0000_180F)];
        const ADDRESSES: &[PublicDeviceAddress] = &[PublicDeviceAddress::new([
            0xF4, 0x23, 0x14, 0xC3, 0xDC, 0x24,
        ])];
        let builder = ScanResponseData::builder()
            .with_le_supported_features()
            .with_local_name(LocalNameComplete::Shortened(7))
            .with_public_target_address(ADDRESSES)
            .unwrap()
            .with_service_uuid32(uuids, ServiceListComplete::Incomplete)
            .unwrap();
        assert_eq!(
            builder.data.base.le_supported_features,
            Some(LeSupportedFeaturesAdStruct::new(
                SupportedLeFeatures::default()
            ))
        );
        assert_eq!(
            builder.data.base.local_name,
            Some(LocalNameAdStruct::new("", LocalNameComplete::Shortened(7)))
        );
        assert_eq!(
            builder.data.base.public_target_address,
            Some(PublicTargetAddressAdStruct::try_new(ADDRESSES).unwrap())
        );
        assert_eq!(
            builder.data.base.service_uuid32,
            Some(ServiceUuid32AdStruct::try_new(uuids, ServiceListComplete::Incomplete).unwrap())
        );
        builder.build()
    }

    #[fixture]
    fn scan_response_data_builder_service_uuid128<'a>() -> ScanResponseData<'a> {
        let connection_interval = 0x0006.try_into().unwrap()..=0x0010.try_into().unwrap();
        let uuids = &[Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)];
        let builder = ScanResponseData::builder()
            .with_peripheral_connection_interval_range(connection_interval.clone())
            .with_tx_power_level()
            .with_service_uuid128(uuids, ServiceListComplete::Complete)
            .unwrap();
        assert_eq!(
            builder.data.base.peripheral_connection_interval,
            Some(PeripheralConnectionIntervalRangeAdStruct::new(
                connection_interval
            ))
        );
        assert_eq!(
            builder.data.base.tx_power_level,
            Some(TxPowerLevelAdStruct::new(TxPowerLevel::default()))
        );
        assert_eq!(
            builder.data.base.service_uuid128,
            Some(ServiceUuid128AdStruct::try_new(uuids, ServiceListComplete::Complete).unwrap())
        );
        builder.build()
    }

    #[fixture]
    fn scan_response_data_builder_service_solicitation_uuid16<'a>() -> ScanResponseData<'a> {
        let uri = Uri::new(ProvisionedUriScheme::Https, "//example.org/");
        let uuids = &[ServiceUuid::Battery, ServiceUuid::BloodPressure];
        let builder = ScanResponseData::builder()
            .with_uri(uri.clone())
            .with_service_solicitation_uuid16(uuids)
            .unwrap();
        assert_eq!(builder.data.base.uri, Some(UriAdStruct::new(uri)));
        assert_eq!(
            builder.data.base.service_solicitation_uuid16,
            Some(ServiceSolicitationUuid16AdStruct::new(uuids))
        );
        builder.build()
    }

    #[fixture]
    fn scan_response_data_builder_service_solicitation_uuid32<'a>() -> ScanResponseData<'a> {
        let data = &[0x9E, 0xF5, 0x40, 0x7C, 0x0F];
        let uuids = &[Uuid32(0x0000_1803), Uuid32(0x0000_180F)];
        let builder = ScanResponseData::builder()
            .with_manufacturer_specific_data(CompanyIdentifier::StMicroelectronics, data)
            .with_service_data_uuid32(Uuid32(0x0000_1803), &[0x50, 0x84, 0x91, 0xAF])
            .with_service_solicitation_uuid32(uuids)
            .unwrap();
        assert_eq!(
            builder.data.base.manufacturer_specific_data,
            Some(ManufacturerSpecificDataAdStruct::new(
                CompanyIdentifier::StMicroelectronics,
                data
            ))
        );
        assert_eq!(
            builder.data.base.service_solicitation_uuid32,
            Some(ServiceSolicitationUuid32AdStruct::new(uuids))
        );
        builder.build()
    }

    #[fixture]
    fn scan_response_data_builder_service_solicitation_uuid128<'a>() -> ScanResponseData<'a> {
        let uuids = &[Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)];
        let builder = ScanResponseData::builder()
            .with_service_solicitation_uuid128(uuids)
            .unwrap();
        assert_eq!(
            builder.data.base.service_solicitation_uuid128,
            Some(ServiceSolicitationUuid128AdStruct::new(uuids))
        );
        builder.build()
    }

    #[fixture]
    fn scan_response_data_builder_service_data_uuid128<'a>() -> ScanResponseData<'a> {
        let uuid = Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640);
        let data: &[u8] = &[0x50, 0x84, 0x91, 0xAF];
        let builder = ScanResponseData::builder().with_service_data_uuid128(uuid, data);
        assert_eq!(
            builder.data.base.service_data_uuid128,
            Some(ServiceDataUuid128AdStruct::new(uuid, data))
        );
        builder.build()
    }

    #[rstest]
    #[case::empty(scan_response_data_builder_empty(), 0, &[])]
    #[case::service_uuid16(
        scan_response_data_builder_service_uuid16(),
        24, &[0x03, 0x1A, 0x00, 0x08, 0x03, 0x19, 0x00, 0x00, 0x01, 0x09, 0x07, 0x16, 0x0F, 0x18,
            0x50, 0x84, 0x91, 0xAF, 0x05, 0x03, 0x0F, 0x18, 0x10, 0x18]
    )]
    #[case::service_uuid32(
        scan_response_data_builder_service_uuid32(),
        22, &[0x01, 0x27, 0x01, 0x08, 0x07, 0x17, 0xF4, 0x23, 0x14, 0xC3, 0xDC, 0x24, 0x09, 0x04, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00]
    )]
    #[case::service_uuid128(
        scan_response_data_builder_service_uuid128(),
        27, &[0x05, 0x12, 0x06, 0x00, 0x10, 0x00, 0x11, 0x07, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C,
            0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5, 0x02, 0x0A, 0x00]
    )]
    #[case::service_solicitation_uuid16(
        scan_response_data_builder_service_solicitation_uuid16(),
        24, &[0x05, 0x14, 0x0F, 0x18, 0x10, 0x18, 0x11, 0x24, 0x17, 0x00,
            b'/', b'/', b'e', b'x', b'a', b'm', b'p', b'l', b'e', b'.', b'o', b'r', b'g', b'/']
    )]
    #[case::service_solicitation_uuid32(
        scan_response_data_builder_service_solicitation_uuid32(),
        29, &[0x08, 0xFF, 0x30, 0x00, 0x9E, 0xF5, 0x40, 0x7C, 0x0F, 0x09, 0x20, 0x03, 0x18, 0x00,
            0x00, 0x50, 0x84, 0x91, 0xAF, 0x09, 0x1F, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00]
    )]
    #[case::service_solicitation_uuid128(
        scan_response_data_builder_service_solicitation_uuid128(),
        18, &[0x11, 0x15, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5]
    )]
    #[case::service_data_uuid128(
        scan_response_data_builder_service_data_uuid128(),
        22, &[0x15, 0x21, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5, 0x50, 0x84, 0x91, 0xAF]
    )]
    fn test_scan_response_data_builder(
        #[case] scanresp_data: ScanResponseData,
        #[case] expected_encoded_size: usize,
        #[case] expected_encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<31>::default();
        assert_eq!(scanresp_data.encoded_size(), expected_encoded_size);
        assert_eq!(scanresp_data.encode(&mut buffer)?, expected_encoded_size);
        assert_eq!(buffer.data(), expected_encoded_data);

        let mut hci_expected_encoded_data = [0u8; 32];
        hci_expected_encoded_data[0] = expected_encoded_size as u8;
        hci_expected_encoded_data[1..1 + expected_encoded_size].copy_from_slice(buffer.data());
        let mut buffer = Buffer::<32>::default();
        let hci_scan_response_data: bletio_hci::ScanResponseData =
            (&scanresp_data).try_into().unwrap();
        assert_eq!(hci_scan_response_data.encoded_size(), 32);
        assert_eq!(hci_scan_response_data.encode(&mut buffer)?, 32);
        assert_eq!(buffer.data(), &hci_expected_encoded_data);

        Ok(())
    }

    #[test]
    fn test_full_advertising_data_success() -> Result<(), Error> {
        let local_name = "bletio";
        let adv_data = AdvertisingData::builder()
            .with_appearance()
            .with_tx_power_level()
            .with_le_supported_features()
            .with_local_name(LocalNameComplete::Shortened(3))
            .build();
        let scanresp_data = ScanResponseData::builder()
            .with_local_name(LocalNameComplete::Complete)
            .build();
        let full_adv_data = FullAdvertisingData::try_new(adv_data.clone(), scanresp_data.clone())?;
        assert_eq!(
            full_adv_data.adv_data.base.appearance,
            Some(AppearanceAdStruct::new(AppearanceValue::GenericUnknown))
        );
        assert_eq!(
            full_adv_data.adv_data.base.local_name,
            Some(LocalNameAdStruct::new("", LocalNameComplete::Shortened(3)))
        );
        assert_eq!(
            full_adv_data.adv_data.base.tx_power_level,
            Some(TxPowerLevelAdStruct::new(TxPowerLevel::default()))
        );
        assert_eq!(
            full_adv_data.adv_data.base.le_supported_features,
            Some(LeSupportedFeaturesAdStruct::new(
                SupportedLeFeatures::default()
            ))
        );
        assert_eq!(
            full_adv_data
                .scanresp_data
                .as_ref()
                .unwrap()
                .base
                .local_name,
            Some(LocalNameAdStruct::new("", LocalNameComplete::Complete))
        );
        assert_eq!(full_adv_data.adv_data, adv_data);
        assert_eq!(full_adv_data.scanresp_data, Some(scanresp_data));

        let appearance = AppearanceValue::Thermostat;
        let tx_power_level = TxPowerLevel::try_new(-8).unwrap();
        let supported_le_features =
            SupportedLeFeatures::LE_2M_PHY | SupportedLeFeatures::LE_CODED_PHY;
        let mut device_information = DeviceInformation::default();
        device_information.appearance = appearance;
        device_information.local_name = "bletio";
        device_information.tx_power_level = tx_power_level;
        device_information.supported_le_features = supported_le_features;
        let filled_full_adv_data = full_adv_data.fill_automatic_data(&device_information);
        assert_eq!(
            filled_full_adv_data.adv_data.base.appearance,
            Some(AppearanceAdStruct::new(appearance))
        );
        assert_eq!(
            filled_full_adv_data.adv_data.base.local_name,
            Some(LocalNameAdStruct::new(
                local_name,
                LocalNameComplete::Shortened(3)
            ))
        );
        assert_eq!(
            filled_full_adv_data.adv_data.base.tx_power_level,
            Some(TxPowerLevelAdStruct::new(tx_power_level))
        );
        assert_eq!(
            filled_full_adv_data.adv_data.base.le_supported_features,
            Some(LeSupportedFeaturesAdStruct::new(supported_le_features))
        );
        assert_eq!(
            filled_full_adv_data
                .scanresp_data
                .as_ref()
                .unwrap()
                .base
                .local_name,
            Some(LocalNameAdStruct::new(
                local_name,
                LocalNameComplete::Complete
            ))
        );

        Ok(())
    }

    #[test]
    fn test_full_advertising_data_failure() {
        let adv_data = AdvertisingData::builder().with_appearance().build();
        let scanresp_data = ScanResponseData::builder().with_appearance().build();
        let err = FullAdvertisingData::try_new(adv_data, scanresp_data);
        assert_eq!(
            err,
            Err(Error::Advertising(
                AdvertisingError::AppearanceNotAllowedInBothAdvertisingDataAndScanResponseData
            ))
        );
    }
}
