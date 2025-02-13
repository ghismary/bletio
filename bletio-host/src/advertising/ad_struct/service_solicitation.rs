use bletio_utils::EncodeToBuffer;

use crate::assigned_numbers::{AdType, ServiceUuid};
use crate::uuid::{Uuid128, Uuid32};

/// List of 16-bit Service Solicitation UUIDs.
///
/// A Peripheral may send the Service Solicitation Advertising Structure to invite Centrals that expose one or
/// more of the services specified in the Service Solicitation data to connect. The Peripheral should be in the
/// undirected connectable mode and in one of the discoverable modes. This enables a Central providing one or more
/// of these services to connect to the Peripheral, so that the Peripheral can use the services on the Central.
///
/// See [Supplement to the Bluetooth Core Specification, Part A, 1.10](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-302574d9-585b-209a-c32f-c5b6278f3377).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ServiceSolicitationUuid16AdStruct<'a> {
    uuids: &'a [ServiceUuid],
}

impl<'a> ServiceSolicitationUuid16AdStruct<'a> {
    pub(crate) const fn new(uuids: &'a [ServiceUuid]) -> Self {
        Self { uuids }
    }
}

impl EncodeToBuffer for ServiceSolicitationUuid16AdStruct<'_> {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push((self.encoded_size() - 1) as u8)?;
        buffer.try_push(AdType::ListOfSolicitationServiceUuid16 as u8)?;
        for uuid in self.uuids {
            buffer.encode_le_u16(*uuid as u16)?;
        }
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        (self.uuids.len() * 2) + 2
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ServiceSolicitationUuid32AdStruct<'a> {
    uuids: &'a [Uuid32],
}

impl<'a> ServiceSolicitationUuid32AdStruct<'a> {
    pub(crate) const fn new(uuids: &'a [Uuid32]) -> Self {
        Self { uuids }
    }
}

impl EncodeToBuffer for ServiceSolicitationUuid32AdStruct<'_> {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push((self.encoded_size() - 1) as u8)?;
        buffer.try_push(AdType::ListOfSolicitationServiceUuid32 as u8)?;
        for uuid in self.uuids {
            buffer.encode_le_u32(uuid.0)?;
        }
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        (self.uuids.len() * 4) + 2
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ServiceSolicitationUuid128AdStruct<'a> {
    uuids: &'a [Uuid128],
}

impl<'a> ServiceSolicitationUuid128AdStruct<'a> {
    pub(crate) const fn new(uuids: &'a [Uuid128]) -> Self {
        Self { uuids }
    }
}

impl EncodeToBuffer for ServiceSolicitationUuid128AdStruct<'_> {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push((self.encoded_size() - 1) as u8)?;
        buffer.try_push(AdType::ListOfSolicitationServiceUuid128 as u8)?;
        for uuid in self.uuids {
            buffer.encode_le_u128(uuid.0)?;
        }
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        (self.uuids.len() * 16) + 2
    }
}

#[cfg(test)]
mod test {
    use bletio_utils::{Buffer, BufferOps};
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(&[], &[0x01, 0x14])]
    #[case(&[ServiceUuid::LinkLoss, ServiceUuid::Battery], &[0x05, 0x14, 0x03, 0x18, 0x0F, 0x18])]
    #[case(
        &[ServiceUuid::Glucose, ServiceUuid::HeartRate, ServiceUuid::BloodPressure],
        &[0x07, 0x14, 0x08, 0x18, 0x0D, 0x18, 0x10, 0x18]
    )]
    fn test_service_solicitation_uuid16_ad_struct_success(
        #[case] uuids: &[ServiceUuid],
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<31>::default();
        let value = ServiceSolicitationUuid16AdStruct::new(uuids);
        value.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }

    #[test]
    fn test_service_solicitation_uuid16_ad_struct_failure() {
        let mut buffer = Buffer::<31>::default();
        let value = ServiceSolicitationUuid16AdStruct::new(&[
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
        ]);
        let err = value.encode(&mut buffer);
        assert_eq!(err, Err(bletio_utils::Error::BufferTooSmall));
    }

    #[rstest]
    #[case(&[], &[0x01, 0x1F])]
    #[case(&[Uuid32(0x0000_1803), Uuid32(0x0000_180F)], &[0x09, 0x1F, 0x03, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00])]
    #[case(
        &[Uuid32(0x0000_1808), Uuid32(0x0000_180D), Uuid32(0x0000_180F)],
        &[0x0D, 0x1F, 0x08, 0x18, 0x00, 0x00, 0x0D, 0x18, 0x00, 0x00, 0x0F, 0x18, 0x00, 0x00]
    )]
    fn test_service_solicitation_uuid32_ad_struct_success(
        #[case] uuids: &[Uuid32],
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<31>::default();
        let value = ServiceSolicitationUuid32AdStruct::new(uuids);
        value.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }

    #[test]
    fn test_service_solicitation_uuid32_ad_struct_failure() {
        let mut buffer = Buffer::<31>::default();
        let value = ServiceSolicitationUuid32AdStruct::new(&[
            Uuid32(0x0000_1802),
            Uuid32(0x0000_1803),
            Uuid32(0x0000_1804),
            Uuid32(0x0000_1815),
            Uuid32(0x0000_1806),
            Uuid32(0x0000_1807),
            Uuid32(0x0000_1808),
            Uuid32(0x0000_1809),
        ]);
        let err = value.encode(&mut buffer);
        assert_eq!(err, Err(bletio_utils::Error::BufferTooSmall));
    }

    #[rstest]
    #[case(&[], &[0x01, 0x15])]
    #[case(
        &[Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640)],
        &[0x11, 0x15, 0x40, 0xD6, 0x6E, 0xFD, 0xD0, 0x11, 0x2C, 0xAD, 0x9E, 0x4C, 0x7D, 0x22, 0x7E, 0x28, 0xA1, 0xF5]
    )]
    fn test_service_solicitation_uuid128_ad_struct_success(
        #[case] uuids: &[Uuid128],
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<31>::default();
        let value = ServiceSolicitationUuid128AdStruct::new(uuids);
        value.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }

    #[test]
    fn test_service_solicitation_uuid128_ad_struct_failure() {
        let mut buffer = Buffer::<31>::default();
        let value = ServiceSolicitationUuid128AdStruct::new(&[
            Uuid128(0xF5A1287E_227D_4C9E_AD2C_11D0FD6ED640),
            Uuid128(0xA624BAC7_A46C_4EC8_B3D6_4C82E5A56D96),
        ]);
        let err = value.encode(&mut buffer);
        assert_eq!(err, Err(bletio_utils::Error::BufferTooSmall));
    }
}
