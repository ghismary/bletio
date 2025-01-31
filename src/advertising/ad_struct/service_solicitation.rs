use crate::advertising::ad_struct::{AdStruct, AdStructBuffer, AdStructType};
use crate::advertising::advertising_data::ADVERTISING_DATA_MAX_SIZE;
use crate::assigned_numbers::{AdType, ServiceUuid};
use crate::uuid::{Uuid128, Uuid32};
use crate::Error;

/// List of 16-bit Service Solicitation UUIDs.
///
/// A Peripheral may send the Service Solicitation Advertising Structure to invite Centrals that expose one or
/// more of the services specified in the Service Solicitation data to connect. The Peripheral should be in the
/// undirected connectable mode and in one of the discoverable modes. This enables a Central providing one or more
/// of these services to connect to the Peripheral, so that the Peripheral can use the services on the Central.
///
/// See [Supplement to the Bluetooth Core Specification, Part A, 1.10](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-302574d9-585b-209a-c32f-c5b6278f3377).
#[derive(Debug, Clone)]
pub struct ServiceSolicitationUuid16AdStruct {
    buffer: AdStructBuffer<ADVERTISING_DATA_MAX_SIZE>,
}

impl ServiceSolicitationUuid16AdStruct {
    /// Create a list of 16-bit Service Solicitation UUIDs.
    ///
    /// # Arguments
    ///
    /// * `uuids` — A slice of Service UUIDs.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bletio::assigned_numbers::ServiceUuid;
    /// # use bletio::advertising::ad_struct::ServiceSolicitationUuid16AdStruct;
    /// # fn main() -> Result<(), bletio::Error> {
    /// let ad_struct = ServiceSolicitationUuid16AdStruct::try_new(
    ///     &[ServiceUuid::Battery],
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_new(uuids: &[ServiceUuid]) -> Result<Self, Error> {
        let mut s = Self::default();
        for uuid in uuids {
            s.buffer.encode_le_u16(*uuid as u16)?;
        }
        Ok(s)
    }

    #[cfg(test)]
    fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.buffer.len() / 2
    }
}

impl Default for ServiceSolicitationUuid16AdStruct {
    fn default() -> Self {
        Self {
            buffer: AdStructBuffer::new(AdType::ListOfSolicitationServiceUuid16),
        }
    }
}

impl AdStruct for ServiceSolicitationUuid16AdStruct {
    fn encoded_data(&self) -> &[u8] {
        self.buffer.data()
    }
    fn r#type(&self) -> AdStructType {
        AdStructType::SERVICE_SOLICITATION_UUID16
    }

    fn is_unique(&self) -> bool {
        false
    }
}

/// List of 32-bit Service Solicitation UUIDs.
///
/// A Peripheral may send the Service Solicitation Advertising Structure to invite Centrals that expose one or
/// more of the services specified in the Service Solicitation data to connect. The Peripheral should be in the
/// undirected connectable mode and in one of the discoverable modes. This enables a Central providing one or more
/// of these services to connect to the Peripheral, so that the Peripheral can use the services on the Central.
///
/// See [Supplement to the Bluetooth Core Specification, Part A, 1.10](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-302574d9-585b-209a-c32f-c5b6278f3377).
#[derive(Debug, Clone)]
pub struct ServiceSolicitationUuid32AdStruct {
    buffer: AdStructBuffer<ADVERTISING_DATA_MAX_SIZE>,
}

impl ServiceSolicitationUuid32AdStruct {
    /// Create a list of 32-bit Service Solicitation UUIDs.
    ///
    /// # Arguments
    ///
    /// * `uuids` — A slice of `Uuid32`s.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bletio::advertising::ad_struct::ServiceSolicitationUuid32AdStruct;
    /// # use bletio::uuid::Uuid32;
    /// # fn main() -> Result<(), bletio::Error> {
    /// let ad_struct = ServiceSolicitationUuid32AdStruct::try_new(
    ///     [Uuid32(0x0000_1803)].as_slice(),
    /// )?;
    /// # Ok(())
    /// # }
    pub fn try_new(uuids: &[impl Into<Uuid32> + Copy]) -> Result<Self, Error> {
        let mut s = Self::default();
        for uuid in uuids {
            s.buffer.encode_le_u32((*uuid).into().0)?;
        }
        Ok(s)
    }

    #[cfg(test)]
    fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.buffer.len() / 4
    }
}

impl Default for ServiceSolicitationUuid32AdStruct {
    fn default() -> Self {
        Self {
            buffer: AdStructBuffer::new(AdType::ListOfSolicitationServiceUuid32),
        }
    }
}

impl AdStruct for ServiceSolicitationUuid32AdStruct {
    fn encoded_data(&self) -> &[u8] {
        self.buffer.data()
    }
    fn r#type(&self) -> AdStructType {
        AdStructType::SERVICE_SOLICITATION_UUID32
    }

    fn is_unique(&self) -> bool {
        false
    }
}

/// List of 128-bit Service Solicitation UUIDs.
///
/// A Peripheral may send the Service Solicitation Advertising Structure to invite Centrals that expose one or
/// more of the services specified in the Service Solicitation data to connect. The Peripheral should be in the
/// undirected connectable mode and in one of the discoverable modes. This enables a Central providing one or more
/// of these services to connect to the Peripheral, so that the Peripheral can use the services on the Central.
///
/// See [Supplement to the Bluetooth Core Specification, Part A, 1.10](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-302574d9-585b-209a-c32f-c5b6278f3377).
#[derive(Debug, Clone)]
pub struct ServiceSolicitationUuid128AdStruct {
    buffer: AdStructBuffer<ADVERTISING_DATA_MAX_SIZE>,
}

impl ServiceSolicitationUuid128AdStruct {
    /// Create a list of 128-bit Service Solicitation UUIDs.
    ///
    /// # Arguments
    ///
    /// * `uuids` — A slice of `Uuid128`s.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bletio::advertising::ad_struct::ServiceSolicitationUuid128AdStruct;
    /// # use bletio::uuid::Uuid128;
    /// # fn main() -> Result<(), bletio::Error> {
    /// let ad_struct = ServiceSolicitationUuid128AdStruct::try_new(
    ///     [Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)].as_slice(),
    /// )?;
    /// # Ok(())
    /// # }
    pub fn try_new(uuids: &[impl Into<Uuid128> + Copy]) -> Result<Self, Error> {
        let mut s = Self::default();
        for uuid in uuids {
            s.buffer.encode_le_u128((*uuid).into().0)?;
        }
        Ok(s)
    }

    #[cfg(test)]
    fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.buffer.len() / 16
    }
}

impl Default for ServiceSolicitationUuid128AdStruct {
    fn default() -> Self {
        Self {
            buffer: AdStructBuffer::new(AdType::ListOfSolicitationServiceUuid128),
        }
    }
}

impl AdStruct for ServiceSolicitationUuid128AdStruct {
    fn encoded_data(&self) -> &[u8] {
        self.buffer.data()
    }
    fn r#type(&self) -> AdStructType {
        AdStructType::SERVICE_SOLICITATION_UUID128
    }

    fn is_unique(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::advertising::AdvertisingError;

    #[test]
    fn test_service_solicitation_uuid16_ad_struct_creation_success() -> Result<(), Error> {
        let value = ServiceSolicitationUuid16AdStruct::default();
        assert_eq!(value.len(), 0);
        assert_eq!(value.encoded_data(), &[0x01, 0x14]);
        assert!(value
            .r#type()
            .contains(AdStructType::SERVICE_SOLICITATION_UUID16));
        assert!(!value.is_unique());

        let value = ServiceSolicitationUuid16AdStruct::try_new(
            [ServiceUuid::LinkLoss, ServiceUuid::Battery].as_slice(),
        )?;
        assert_eq!(value.len(), 2);
        assert_eq!(value.encoded_data(), &[0x05, 0x14, 0x03, 0x18, 0x0F, 0x18]);
        assert!(value
            .r#type()
            .contains(AdStructType::SERVICE_SOLICITATION_UUID16));
        assert!(!value.is_unique());

        let value = ServiceSolicitationUuid16AdStruct::try_new(
            [
                ServiceUuid::Glucose,
                ServiceUuid::HeartRate,
                ServiceUuid::BloodPressure,
            ]
            .as_slice(),
        )?;
        assert_eq!(value.len(), 3);
        assert_eq!(
            value.encoded_data(),
            &[0x07, 0x14, 0x08, 0x18, 0x0D, 0x18, 0x10, 0x18]
        );
        assert!(value
            .r#type()
            .contains(AdStructType::SERVICE_SOLICITATION_UUID16));
        assert!(!value.is_unique());

        let value = ServiceSolicitationUuid16AdStruct::try_new([].as_slice())?;
        assert!(value.is_empty());
        assert_eq!(value.encoded_data(), &[0x01, 0x14]);
        assert!(value
            .r#type()
            .contains(AdStructType::SERVICE_SOLICITATION_UUID16));
        assert!(!value.is_unique());

        Ok(())
    }

    #[test]
    fn test_service_solicitation_uuid16_ad_struct_creation_failure() {
        let err = ServiceSolicitationUuid16AdStruct::try_new(
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
        )
        .expect_err("Too many Uuid16 to fit in the advertising data");
        assert!(matches!(
            err,
            Error::Advertising(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
        ));
    }

    #[test]
    fn test_service_solicitation_uuid32_ad_struct_creation_success() -> Result<(), Error> {
        let value = ServiceSolicitationUuid32AdStruct::default();
        assert_eq!(value.len(), 0);
        assert_eq!(value.encoded_data(), &[0x01, 0x1F]);
        assert!(value
            .r#type()
            .contains(AdStructType::SERVICE_SOLICITATION_UUID32));
        assert!(!value.is_unique());

        let value = ServiceSolicitationUuid32AdStruct::try_new(
            [Uuid32(0x0000_1803), Uuid32(0x0000_180F)].as_slice(),
        )?;
        assert_eq!(value.len(), 2);
        assert_eq!(
            value.encoded_data(),
            &[0x09, 0x1F, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00]
        );
        assert!(value
            .r#type()
            .contains(AdStructType::SERVICE_SOLICITATION_UUID32));
        assert!(!value.is_unique());

        let value = ServiceSolicitationUuid32AdStruct::try_new(
            [0x0000_1808, 0x0000_180D, 0x0000_180F].as_slice(),
        )?;
        assert_eq!(value.len(), 3);
        assert_eq!(
            value.encoded_data(),
            &[0x0D, 0x1F, 0x08, 0x18, 0x00, 0x00, 0x0D, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00]
        );
        assert!(value
            .r#type()
            .contains(AdStructType::SERVICE_SOLICITATION_UUID32));
        assert!(!value.is_unique());

        let empty_uuids: [Uuid32; 0] = [];
        let value = ServiceSolicitationUuid32AdStruct::try_new(empty_uuids.as_slice())?;
        assert!(value.is_empty());
        assert_eq!(value.encoded_data(), &[0x01, 0x1F]);
        assert!(value
            .r#type()
            .contains(AdStructType::SERVICE_SOLICITATION_UUID32));
        assert!(!value.is_unique());

        Ok(())
    }

    #[test]
    fn test_service_solicitation_uuid32_ad_struct_creation_failure() {
        let err = ServiceSolicitationUuid32AdStruct::try_new(
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
        )
        .expect_err("Too many Uuid32 to fit in the advertising data");
        assert!(matches!(
            err,
            Error::Advertising(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
        ));
    }

    #[test]
    fn test_service_solicitation_uuid128_ad_struct_creation_success() -> Result<(), Error> {
        let value = ServiceSolicitationUuid128AdStruct::default();
        assert_eq!(value.len(), 0);
        assert_eq!(value.encoded_data(), &[0x01, 0x15]);
        assert!(value
            .r#type()
            .contains(AdStructType::SERVICE_SOLICITATION_UUID128));
        assert!(!value.is_unique());

        let value = ServiceSolicitationUuid128AdStruct::try_new(
            [Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)].as_slice(),
        )?;
        assert_eq!(value.len(), 1);
        assert_eq!(
            value.encoded_data(),
            &[
                0x11, 0x15, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22,
                0x7E, 0x28, 0xA1, 0xF5
            ]
        );
        assert!(value
            .r#type()
            .contains(AdStructType::SERVICE_SOLICITATION_UUID128));
        assert!(!value.is_unique());

        let value = ServiceSolicitationUuid128AdStruct::try_new(
            [0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640].as_slice(),
        )?;
        assert_eq!(value.len(), 1);
        assert_eq!(
            value.encoded_data(),
            &[
                0x11, 0x15, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22,
                0x7E, 0x28, 0xA1, 0xF5
            ]
        );
        assert!(value
            .r#type()
            .contains(AdStructType::SERVICE_SOLICITATION_UUID128));
        assert!(!value.is_unique());

        let empty_uuids: [Uuid128; 0] = [];
        let value = ServiceSolicitationUuid128AdStruct::try_new(empty_uuids.as_slice())?;
        assert!(value.is_empty());
        assert_eq!(value.encoded_data(), &[0x01, 0x15]);
        assert!(value
            .r#type()
            .contains(AdStructType::SERVICE_SOLICITATION_UUID128));
        assert!(!value.is_unique());

        Ok(())
    }

    #[test]
    fn test_service_solicitation_uuid128_ad_struct_creation_failure() {
        let err = ServiceSolicitationUuid128AdStruct::try_new(
            [
                0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640,
                0xA624BAC7_A46C_4EC8_B3D6_4C82E5A56D96,
            ]
            .as_slice(),
        )
        .expect_err("Too many Uuid128 to fit in the advertising data");
        assert!(matches!(
            err,
            Error::Advertising(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
        ));
    }
}
