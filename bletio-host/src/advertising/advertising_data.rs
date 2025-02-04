use core::ops::RangeInclusive;

use bletio_hci::{AdvertisingIntervalValue, ConnectionInterval, SupportedLeFeatures};
use bletio_utils::{BufferOps, EncodeToBuffer};

use crate::advertising::ad_struct::{
    AdvertisingIntervalAdStruct, AppearanceAdStruct, FlagsAdStruct, LeSupportedFeaturesAdStruct,
    ManufacturerSpecificDataAdStruct, PeripheralConnectionIntervalRangeAdStruct,
    ServiceSolicitationUuid128AdStruct, ServiceSolicitationUuid16AdStruct,
    ServiceSolicitationUuid32AdStruct, ServiceUuid128AdStruct, ServiceUuid16AdStruct,
    ServiceUuid32AdStruct, TxPowerLevelAdStruct, UriAdStruct,
};
use crate::advertising::{AdvertisingError, Flags, ServiceListComplete, TxPowerLevel, Uri};
use crate::assigned_numbers::{AppearanceValue, CompanyIdentifier, ServiceUuid};
use crate::ble_device_information::BleDeviceInformation;
use crate::controller_capabilities::ControllerCapabilities;
use crate::uuid::{Uuid128, Uuid32};
use crate::Error;

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
        // TODO: check validity
        Ok(Self {
            adv_data,
            scanresp_data: scanresp_data.into(),
        })
    }

    pub(crate) fn fill_automatic_data(
        &self,
        device_info: &BleDeviceInformation,
        controller_capabilities: &ControllerCapabilities,
    ) -> Self {
        let mut filled = self.clone();
        filled
            .adv_data
            .base
            .fill_automatic_data(device_info, controller_capabilities);
        if let Some(scanresp_data) = filled.scanresp_data.as_mut() {
            scanresp_data
                .base
                .fill_automatic_data(device_info, controller_capabilities);
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
        self.data.base.tx_power_level = Some(TxPowerLevelAdStruct::new(TxPowerLevel(0)));
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
    manufacturer_specific_data: Option<ManufacturerSpecificDataAdStruct<'a>>,
    peripheral_connection_interval: Option<PeripheralConnectionIntervalRangeAdStruct>,
    service_uuid16: Option<ServiceUuid16AdStruct<'a>>,
    service_uuid32: Option<ServiceUuid32AdStruct<'a>>,
    service_uuid128: Option<ServiceUuid128AdStruct<'a>>,
    service_solicitation_uuid16: Option<ServiceSolicitationUuid16AdStruct<'a>>,
    service_solicitation_uuid32: Option<ServiceSolicitationUuid32AdStruct<'a>>,
    service_solicitation_uuid128: Option<ServiceSolicitationUuid128AdStruct<'a>>,
    tx_power_level: Option<TxPowerLevelAdStruct>,
    uri: Option<UriAdStruct>,
}

impl AdvertisingDataBase<'_> {
    fn fill_automatic_data(
        &mut self,
        device_info: &BleDeviceInformation,
        controller_capabilities: &ControllerCapabilities,
    ) {
        if self.appearance.is_some() {
            self.appearance = Some(AppearanceAdStruct::new(device_info.appearance));
        }
        if self.le_supported_features.is_some() {
            self.le_supported_features = Some(LeSupportedFeaturesAdStruct::new(
                controller_capabilities.supported_le_features,
            ));
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
        self.data.base.tx_power_level = Some(TxPowerLevelAdStruct::new(TxPowerLevel(0)));
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
