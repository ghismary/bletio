use crate::advertising::ad_struct::{
    AdStruct, AdStructType, AD_STRUCT_DATA_OFFSET, AD_STRUCT_LENGTH_OFFSET, AD_STRUCT_TYPE_OFFSET,
};
use crate::advertising::advertising_data::ADVERTISING_DATA_MAX_SIZE;
use crate::advertising::AdvertisingError;
use crate::assigned_numbers::{AdType, ServiceUuid};
use crate::utils::{encode_le_u128, encode_le_u16, encode_le_u32};
use crate::uuid::{Uuid128, Uuid32};
use crate::Error;

/// Whether a service list is complete or not.
///
/// Used when creating list of UUID16, UUID32 or UUID128 services Advertising Structures.
/// See [`ServiceUuid16AdStruct::try_new`], [`ServiceUuid32AdStruct::try_new`] and [`ServiceUuid128AdStruct::try_new`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceListComplete {
    Complete,
    Incomplete,
}

/// List of 16-bit Bluetooth Service or Service Class UUIDs.
///
/// This list can be complete or incomplete. If the list is empty, it shall be marked as complete,
/// as defined in [Supplement to the Bluetooth Core Specification, Part A, 1.1](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-b1d0edbc-fc9e-507a-efe4-3fd4b4817a52).
#[derive(Debug, Clone)]
pub struct ServiceUuid16AdStruct {
    buffer: [u8; ADVERTISING_DATA_MAX_SIZE],
    offset: usize,
}

impl ServiceUuid16AdStruct {
    /// Create a list of 16-bit Bluetooth Service or Service Class UUIDs Advertising Structure.
    ///
    /// # Arguments
    ///
    /// * `uuids` — A slice of Service UUIDs.
    /// * `complete` — Whether the list is complete or not.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bletio::assigned_numbers::ServiceUuid;
    /// # use bletio::advertising::ad_struct::{ServiceListComplete, ServiceUuid16AdStruct};
    /// # fn main() -> Result<(), bletio::Error> {
    /// let ad_struct = ServiceUuid16AdStruct::try_new(
    ///     [ServiceUuid::Battery, ServiceUuid::EnvironmentalSensing].as_slice(),
    ///     ServiceListComplete::Incomplete
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_new(uuids: &[ServiceUuid], complete: ServiceListComplete) -> Result<Self, Error> {
        if uuids.is_empty() && complete == ServiceListComplete::Incomplete {
            return Err(AdvertisingError::EmptyServiceUuidListShallBeComplete)?;
        }
        let uuids_size = uuids.len() * 2;
        let mut s = Self {
            offset: AD_STRUCT_DATA_OFFSET,
            buffer: Default::default(),
        };
        s.buffer[AD_STRUCT_LENGTH_OFFSET] = 1 + uuids_size as u8;
        s.buffer[AD_STRUCT_TYPE_OFFSET] = match complete {
            ServiceListComplete::Complete => AdType::CompleteListOfServiceUuid16,
            ServiceListComplete::Incomplete => AdType::IncompleteListOfServiceUuid16,
        } as u8;
        for uuid in uuids {
            s.offset += encode_le_u16(&mut s.buffer[s.offset..], *uuid as u16)
                .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?;
        }
        Ok(s)
    }

    #[cfg(test)]
    fn is_complete(&self) -> bool {
        self.buffer[AD_STRUCT_TYPE_OFFSET] == (AdType::CompleteListOfServiceUuid16 as u8)
    }

    #[cfg(test)]
    fn is_empty(&self) -> bool {
        self.offset == AD_STRUCT_DATA_OFFSET
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        (self.offset - AD_STRUCT_DATA_OFFSET) / 2
    }
}

impl Default for ServiceUuid16AdStruct {
    fn default() -> Self {
        let mut s = Self {
            offset: AD_STRUCT_DATA_OFFSET,
            buffer: Default::default(),
        };
        s.buffer[AD_STRUCT_LENGTH_OFFSET] = 1;
        s.buffer[AD_STRUCT_TYPE_OFFSET] = AdType::CompleteListOfServiceUuid16 as u8;
        s
    }
}

impl AdStruct for ServiceUuid16AdStruct {
    fn encoded_data(&self) -> &[u8] {
        &self.buffer[..self.offset]
    }
    fn r#type(&self) -> AdStructType {
        AdStructType::SERVICE_UUID16
    }

    fn is_unique(&self) -> bool {
        true
    }
}

/// List of 32-bit Bluetooth Service or Service Class UUIDs.
///
/// This list can be complete or incomplete. If the list is empty, it shall be marked as complete,
/// as defined in [Supplement to the Bluetooth Core Specification, Part A, 1.1](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-b1d0edbc-fc9e-507a-efe4-3fd4b4817a52).
#[derive(Debug, Clone)]
pub struct ServiceUuid32AdStruct {
    buffer: [u8; ADVERTISING_DATA_MAX_SIZE],
    offset: usize,
}

impl ServiceUuid32AdStruct {
    /// Create a list of 32-bit Bluetooth Service or Service Class UUIDs Advertising Structure.
    ///
    /// # Arguments
    ///
    /// * `uuids` — A slice of `Uuid32`s.
    /// * `complete` — Whether the list is complete or not.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bletio::advertising::ad_struct::{ServiceListComplete, ServiceUuid32AdStruct};
    /// # use bletio::uuid::Uuid32;
    /// # fn main() -> Result<(), bletio::Error> {
    /// let ad_struct = ServiceUuid32AdStruct::try_new(
    ///     [Uuid32(0x0000_1803)].as_slice(),
    ///     ServiceListComplete::Complete
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_new(
        uuids: &[impl Into<Uuid32> + Copy],
        complete: ServiceListComplete,
    ) -> Result<Self, Error> {
        if uuids.is_empty() && complete == ServiceListComplete::Incomplete {
            return Err(AdvertisingError::EmptyServiceUuidListShallBeComplete)?;
        }
        let uuids_size = uuids.len() * 4;
        let mut s = Self {
            offset: AD_STRUCT_DATA_OFFSET,
            buffer: Default::default(),
        };
        s.buffer[AD_STRUCT_LENGTH_OFFSET] = 1 + uuids_size as u8;
        s.buffer[AD_STRUCT_TYPE_OFFSET] = match complete {
            ServiceListComplete::Complete => AdType::CompleteListOfServiceUuid32,
            ServiceListComplete::Incomplete => AdType::IncompleteListOfServiceUuid32,
        } as u8;
        for uuid in uuids {
            let uuid = (*uuid).into();
            s.offset += encode_le_u32(&mut s.buffer[s.offset..], uuid.0)
                .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?;
        }
        Ok(s)
    }

    #[cfg(test)]
    fn is_complete(&self) -> bool {
        self.buffer[AD_STRUCT_TYPE_OFFSET] == (AdType::CompleteListOfServiceUuid32 as u8)
    }

    #[cfg(test)]
    fn is_empty(&self) -> bool {
        self.offset == AD_STRUCT_DATA_OFFSET
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        (self.offset - AD_STRUCT_DATA_OFFSET) / 4
    }
}

impl Default for ServiceUuid32AdStruct {
    fn default() -> Self {
        let mut s = Self {
            offset: AD_STRUCT_DATA_OFFSET,
            buffer: Default::default(),
        };
        s.buffer[AD_STRUCT_LENGTH_OFFSET] = 1;
        s.buffer[AD_STRUCT_TYPE_OFFSET] = AdType::CompleteListOfServiceUuid32 as u8;
        s
    }
}

impl AdStruct for ServiceUuid32AdStruct {
    fn encoded_data(&self) -> &[u8] {
        &self.buffer[..self.offset]
    }
    fn r#type(&self) -> AdStructType {
        AdStructType::SERVICE_UUID32
    }

    fn is_unique(&self) -> bool {
        true
    }
}

/// List of Global 128-bit Service UUIDs.
///
/// This list can be complete or incomplete. If the list is empty, it shall be marked as complete,
/// as defined in [Supplement to the Bluetooth Core Specification, Part A, 1.1](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-b1d0edbc-fc9e-507a-efe4-3fd4b4817a52).
#[derive(Debug, Clone)]
pub struct ServiceUuid128AdStruct {
    buffer: [u8; ADVERTISING_DATA_MAX_SIZE],
    offset: usize,
}

impl ServiceUuid128AdStruct {
    /// Create a list of Global 128-bit Service UUIDs Advertising Structure.
    ///
    /// # Arguments
    ///
    /// * `uuids` — A slice of `Uuid128`s.
    /// * `complete` — Whether the list is complete or not.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bletio::advertising::ad_struct::{ServiceListComplete, ServiceUuid128AdStruct};
    /// # use bletio::uuid::Uuid128;
    /// # fn main() -> Result<(), bletio::Error> {
    /// let ad_struct = ServiceUuid128AdStruct::try_new(
    ///     [Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)].as_slice(),
    ///     ServiceListComplete::Incomplete
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_new(
        uuids: &[impl Into<Uuid128> + Copy],
        complete: ServiceListComplete,
    ) -> Result<Self, Error> {
        if uuids.is_empty() && complete == ServiceListComplete::Incomplete {
            return Err(AdvertisingError::EmptyServiceUuidListShallBeComplete)?;
        }
        let uuids_size = uuids.len() * 16;
        let mut s = Self {
            offset: AD_STRUCT_DATA_OFFSET,
            buffer: Default::default(),
        };
        s.buffer[AD_STRUCT_LENGTH_OFFSET] = 1 + uuids_size as u8;
        s.buffer[AD_STRUCT_TYPE_OFFSET] = match complete {
            ServiceListComplete::Complete => AdType::CompleteListOfServiceUuid128,
            ServiceListComplete::Incomplete => AdType::IncompleteListOfServiceUuid128,
        } as u8;
        for uuid in uuids {
            let uuid = (*uuid).into();
            s.offset += encode_le_u128(&mut s.buffer[s.offset..], uuid.0)
                .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?;
        }
        Ok(s)
    }

    #[cfg(test)]
    fn is_complete(&self) -> bool {
        self.buffer[AD_STRUCT_TYPE_OFFSET] == (AdType::CompleteListOfServiceUuid128 as u8)
    }

    #[cfg(test)]
    fn is_empty(&self) -> bool {
        self.offset == AD_STRUCT_DATA_OFFSET
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        (self.offset - AD_STRUCT_DATA_OFFSET) / 16
    }
}

impl Default for ServiceUuid128AdStruct {
    fn default() -> Self {
        let mut s = Self {
            offset: AD_STRUCT_DATA_OFFSET,
            buffer: Default::default(),
        };
        s.buffer[AD_STRUCT_LENGTH_OFFSET] = 1;
        s.buffer[AD_STRUCT_TYPE_OFFSET] = AdType::CompleteListOfServiceUuid128 as u8;
        s
    }
}

impl AdStruct for ServiceUuid128AdStruct {
    fn encoded_data(&self) -> &[u8] {
        &self.buffer[..self.offset]
    }
    fn r#type(&self) -> AdStructType {
        AdStructType::SERVICE_UUID128
    }

    fn is_unique(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_service_uuid16_ad_struct_creation_success() -> Result<(), Error> {
        let value = ServiceUuid16AdStruct::default();
        assert_eq!(value.len(), 0);
        assert_eq!(value.encoded_data(), &[0x01, 0x03]);
        assert!(value.r#type().contains(AdStructType::SERVICE_UUID16));
        assert!(value.is_unique());

        let value = ServiceUuid16AdStruct::try_new(
            [
                ServiceUuid::LinkLoss,
                ServiceUuid::Battery,
                ServiceUuid::EnvironmentalSensing,
            ]
            .as_slice(),
            ServiceListComplete::Incomplete,
        )?;
        assert_eq!(value.len(), 3);
        assert!(!value.is_complete());
        assert_eq!(
            value.encoded_data(),
            &[0x07, 0x02, 0x03, 0x18, 0x0F, 0x18, 0x1A, 0x18]
        );
        assert!(value.r#type().contains(AdStructType::SERVICE_UUID16));
        assert!(value.is_unique());

        let value = ServiceUuid16AdStruct::try_new(
            [
                ServiceUuid::Glucose,
                ServiceUuid::HeartRate,
                ServiceUuid::Battery,
                ServiceUuid::BloodPressure,
            ]
            .as_slice(),
            ServiceListComplete::Complete,
        )?;
        assert_eq!(value.len(), 4);
        assert!(value.is_complete());
        assert_eq!(
            value.encoded_data(),
            &[0x09, 0x03, 0x08, 0x18, 0x0D, 0x18, 0x0F, 0x18, 0x10, 0x18]
        );
        assert!(value.r#type().contains(AdStructType::SERVICE_UUID16));
        assert!(value.is_unique());

        let value = ServiceUuid16AdStruct::try_new([].as_slice(), ServiceListComplete::Complete)?;
        assert!(value.is_empty());
        assert!(value.is_complete());
        assert_eq!(value.encoded_data(), &[0x01, 0x03]);
        assert!(value.r#type().contains(AdStructType::SERVICE_UUID16));
        assert!(value.is_unique());

        Ok(())
    }

    #[test]
    fn test_service_uuid16_ad_struct_creation_failure() {
        let err = ServiceUuid16AdStruct::try_new(
            [
                ServiceUuid::ImmediateAlert,
                ServiceUuid::LinkLoss,
                ServiceUuid::TxPower,
                ServiceUuid::CurrentTime,
                ServiceUuid::ReferenceTimeUpdate,
                ServiceUuid::NextDstChange,
                ServiceUuid::Glucose,
                ServiceUuid::HealthThermometer,
                ServiceUuid::DeviceInformation,
                ServiceUuid::HeartRate,
                ServiceUuid::PhoneAlertStatus,
                ServiceUuid::Battery,
                ServiceUuid::BloodPressure,
                ServiceUuid::AlertNotification,
                ServiceUuid::HumanInterfaceDevice,
            ]
            .as_slice(),
            ServiceListComplete::Complete,
        )
        .expect_err("Too many Uuid16 to fit in the advertising data");
        assert!(matches!(
            err,
            Error::Advertising(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
        ));

        let err = ServiceUuid16AdStruct::try_new([].as_slice(), ServiceListComplete::Incomplete)
            .expect_err("An empty Service UUID list shall be marked as complete");
        assert!(matches!(
            err,
            Error::Advertising(AdvertisingError::EmptyServiceUuidListShallBeComplete)
        ));
    }

    #[test]
    fn test_service_uuid32_ad_struct_creation_success() -> Result<(), Error> {
        let value = ServiceUuid32AdStruct::default();
        assert_eq!(value.len(), 0);
        assert_eq!(value.encoded_data(), &[0x01, 0x05]);
        assert!(value.r#type().contains(AdStructType::SERVICE_UUID32));
        assert!(value.is_unique());

        let value = ServiceUuid32AdStruct::try_new(
            [
                Uuid32(0x0000_1803),
                Uuid32(0x0000_180F),
                Uuid32(0x0000_181A),
            ]
            .as_slice(),
            ServiceListComplete::Incomplete,
        )?;
        assert_eq!(value.len(), 3);
        assert!(!value.is_complete());
        assert_eq!(
            value.encoded_data(),
            &[0x0D, 0x04, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00, 0x1A, 0x18, 0x00, 0x00]
        );
        assert!(value.r#type().contains(AdStructType::SERVICE_UUID32));
        assert!(value.is_unique());

        let value = ServiceUuid32AdStruct::try_new(
            [0x0000_1808, 0x0000_180D, 0x0000_180F, 0x0000_1810].as_slice(),
            ServiceListComplete::Complete,
        )?;
        assert_eq!(value.len(), 4);
        assert!(value.is_complete());
        assert_eq!(
            value.encoded_data(),
            &[
                0x11, 0x05, 0x08, 0x18, 0x00, 0x00, 0x0D, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00,
                0x10, 0x18, 0x00, 0x00
            ]
        );
        assert!(value.r#type().contains(AdStructType::SERVICE_UUID32));
        assert!(value.is_unique());

        let empty_uuids: [Uuid32; 0] = [];
        let value =
            ServiceUuid32AdStruct::try_new(empty_uuids.as_slice(), ServiceListComplete::Complete)?;
        assert!(value.is_empty());
        assert!(value.is_complete());
        assert_eq!(value.encoded_data(), &[0x01, 0x05]);
        assert!(value.r#type().contains(AdStructType::SERVICE_UUID32));
        assert!(value.is_unique());

        Ok(())
    }

    #[test]
    fn test_service_uuid32_ad_struct_creation_failure() {
        let err = ServiceUuid32AdStruct::try_new(
            [
                0x0000_1802,
                0x0000_1803,
                0x0000_1804,
                0x0000_1815,
                0x0000_1806,
                0x0000_1807,
                0x0000_1808,
                0x0000_1809,
            ]
            .as_slice(),
            ServiceListComplete::Complete,
        )
        .expect_err("Too many Uuid32 to fit in the advertising data");
        assert!(matches!(
            err,
            Error::Advertising(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
        ));

        let empty_uuids: [Uuid32; 0] = [];
        let err =
            ServiceUuid32AdStruct::try_new(empty_uuids.as_slice(), ServiceListComplete::Incomplete)
                .expect_err("An empty Service UUID list shall be marked as complete");
        assert!(matches!(
            err,
            Error::Advertising(AdvertisingError::EmptyServiceUuidListShallBeComplete)
        ));
    }

    #[test]
    fn test_service_uuid128_ad_struct_creation_success() -> Result<(), Error> {
        let value = ServiceUuid128AdStruct::default();
        assert_eq!(value.len(), 0);
        assert_eq!(value.encoded_data(), &[0x01, 0x07]);
        assert!(value.r#type().contains(AdStructType::SERVICE_UUID128));
        assert!(value.is_unique());

        let value = ServiceUuid128AdStruct::try_new(
            [Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)].as_slice(),
            ServiceListComplete::Incomplete,
        )?;
        assert_eq!(value.len(), 1);
        assert!(!value.is_complete());
        assert_eq!(
            value.encoded_data(),
            &[
                0x11, 0x06, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22,
                0x7E, 0x28, 0xA1, 0xF5
            ]
        );
        assert!(value.r#type().contains(AdStructType::SERVICE_UUID128));
        assert!(value.is_unique());

        let value = ServiceUuid128AdStruct::try_new(
            [0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640].as_slice(),
            ServiceListComplete::Complete,
        )?;
        assert_eq!(value.len(), 1);
        assert!(value.is_complete());
        assert_eq!(
            value.encoded_data(),
            &[
                0x11, 0x07, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22,
                0x7E, 0x28, 0xA1, 0xF5
            ]
        );
        assert!(value.r#type().contains(AdStructType::SERVICE_UUID128));
        assert!(value.is_unique());

        let empty_uuids: [Uuid128; 0] = [];
        let value =
            ServiceUuid128AdStruct::try_new(empty_uuids.as_slice(), ServiceListComplete::Complete)?;
        assert!(value.is_empty());
        assert!(value.is_complete());
        assert_eq!(value.encoded_data(), &[0x01, 0x07]);
        assert!(value.r#type().contains(AdStructType::SERVICE_UUID128));
        assert!(value.is_unique());

        Ok(())
    }

    #[test]
    fn test_service_uuid128_ad_struct_creation_failure() {
        let err = ServiceUuid128AdStruct::try_new(
            [
                0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640,
                0xA624BAC7_A46C_4EC8_B3D6_4C82E5A56D96,
            ]
            .as_slice(),
            ServiceListComplete::Complete,
        )
        .expect_err("Too many Uuid128 to fit in the advertising data");
        assert!(matches!(
            err,
            Error::Advertising(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
        ));

        let empty_uuids: [Uuid128; 0] = [];
        let err = ServiceUuid128AdStruct::try_new(
            empty_uuids.as_slice(),
            ServiceListComplete::Incomplete,
        )
        .expect_err("An empty Service UUID list shall be marked as complete");
        assert!(matches!(
            err,
            Error::Advertising(AdvertisingError::EmptyServiceUuidListShallBeComplete)
        ));
    }
}
