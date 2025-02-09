use core::num::NonZeroU16;

use bitflags::Flags;
use bletio_hci::{
    PublicDeviceAddress, SupportedCommands, SupportedFeatures, SupportedLeFeatures,
    SupportedLeStates, TxPowerLevel,
};

use crate::assigned_numbers::AppearanceValue;

#[derive(Debug, Clone)]
pub(crate) struct DeviceInformation {
    pub(crate) appearance: AppearanceValue,
    pub(crate) supported_commands: SupportedCommands,
    pub(crate) supported_features: SupportedFeatures,
    pub(crate) supported_le_features: SupportedLeFeatures,
    pub(crate) supported_le_states: SupportedLeStates,
    pub(crate) le_data_packet_length: NonZeroU16,
    pub(crate) num_le_data_packets: NonZeroU16,
    pub(crate) public_device_address: PublicDeviceAddress,
    pub(crate) tx_power_level: TxPowerLevel,
}

impl DeviceInformation {
    pub(crate) fn is_command_supported(&self, command: SupportedCommands) -> bool {
        self.supported_commands.contains(command)
    }

    pub(crate) fn is_feature_supported(&self, feature: SupportedFeatures) -> bool {
        self.supported_features.contains(feature)
    }
}

impl Default for DeviceInformation {
    fn default() -> Self {
        Self {
            appearance: AppearanceValue::GenericUnknown,
            supported_commands: SupportedCommands::default(),
            supported_features: SupportedFeatures::default(),
            supported_le_features: SupportedLeFeatures::default(),
            supported_le_states: SupportedLeStates::default(),
            le_data_packet_length: NonZeroU16::MIN,
            num_le_data_packets: NonZeroU16::MIN,
            public_device_address: PublicDeviceAddress::default(),
            tx_power_level: TxPowerLevel::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_device_information() {
        let device_information = DeviceInformation::default();
        assert_eq!(
            device_information.appearance,
            AppearanceValue::GenericUnknown
        );
        assert_eq!(
            device_information.supported_commands,
            SupportedCommands::default()
        );
        assert_eq!(
            device_information.supported_features,
            SupportedFeatures::default()
        );
        assert_eq!(
            device_information.supported_le_features,
            SupportedLeFeatures::default()
        );
        assert_eq!(
            device_information.supported_le_states,
            SupportedLeStates::default()
        );
        assert_eq!(device_information.le_data_packet_length, NonZeroU16::MIN);
        assert_eq!(device_information.num_le_data_packets, NonZeroU16::MIN);
        assert_eq!(
            device_information.public_device_address,
            PublicDeviceAddress::default()
        );
        assert_eq!(device_information.tx_power_level, TxPowerLevel::default());
        assert!(!device_information.is_command_supported(SupportedCommands::LE_RAND));
        assert!(
            !device_information.is_feature_supported(SupportedFeatures::LE_SUPPORTED_CONTROLLER)
        );
    }

    #[test]
    fn test_device_information() {
        let device_information = DeviceInformation {
            appearance: AppearanceValue::TemperatureSensor,
            supported_commands: SupportedCommands::LE_RAND | SupportedCommands::LE_ENCRYPT,
            supported_features: SupportedFeatures::LE_SUPPORTED_CONTROLLER,
            supported_le_features: SupportedLeFeatures::LE_CODED_PHY,
            supported_le_states: SupportedLeStates::default(),
            le_data_packet_length: NonZeroU16::new(255).unwrap(),
            num_le_data_packets: NonZeroU16::new(2).unwrap(),
            public_device_address: PublicDeviceAddress::new([0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56]),
            tx_power_level: TxPowerLevel::try_new(3).unwrap(),
        };
        assert_eq!(
            device_information.appearance,
            AppearanceValue::TemperatureSensor
        );
        assert_eq!(
            device_information.supported_commands,
            SupportedCommands::LE_RAND | SupportedCommands::LE_ENCRYPT
        );
        assert_eq!(
            device_information.supported_features,
            SupportedFeatures::LE_SUPPORTED_CONTROLLER
        );
        assert_eq!(
            device_information.supported_le_features,
            SupportedLeFeatures::LE_CODED_PHY
        );
        assert_eq!(
            device_information.supported_le_states,
            SupportedLeStates::default()
        );
        assert_eq!(
            device_information.le_data_packet_length,
            NonZeroU16::new(255).unwrap()
        );
        assert_eq!(
            device_information.num_le_data_packets,
            NonZeroU16::new(2).unwrap()
        );
        assert_eq!(
            device_information.public_device_address.value(),
            &[0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56]
        );
        assert_eq!(device_information.tx_power_level.value(), 3);
        assert!(device_information.is_command_supported(SupportedCommands::LE_RAND));
        assert!(device_information.is_command_supported(SupportedCommands::LE_ENCRYPT));
        assert!(!device_information.is_command_supported(SupportedCommands::LE_CREATE_BIG));
        assert!(device_information.is_feature_supported(SupportedFeatures::LE_SUPPORTED_CONTROLLER));
        assert!(!device_information.is_feature_supported(
            SupportedFeatures::SIMULTANEOUS_LE_AND_BREDR_TO_SAME_DEVICE_CAPABLE_CONTROLLER
        ));
    }
}
