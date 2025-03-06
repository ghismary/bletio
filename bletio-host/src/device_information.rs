use core::num::NonZeroU16;

use bitflags::Flags;
use bletio_hci::{
    PublicDeviceAddress, RandomStaticDeviceAddress, SupportedCommands, SupportedFeatures,
    SupportedLeFeatures, SupportedLeStates, TxPowerLevel,
};

use crate::assigned_numbers::AppearanceValue;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct DeviceInformation<'a> {
    pub(crate) appearance: AppearanceValue,
    pub(crate) le_data_packet_length: NonZeroU16,
    pub(crate) local_name: &'a str,
    pub(crate) num_le_data_packets: NonZeroU16,
    pub(crate) public_device_address: PublicDeviceAddress,
    pub(crate) random_static_device_address: Option<RandomStaticDeviceAddress>,
    pub(crate) supported_commands: SupportedCommands,
    pub(crate) supported_features: SupportedFeatures,
    pub(crate) supported_le_features: SupportedLeFeatures,
    pub(crate) supported_le_states: SupportedLeStates,
    pub(crate) tx_power_level: TxPowerLevel,
}

impl DeviceInformation<'_> {
    pub(crate) fn is_command_supported(&self, command: SupportedCommands) -> bool {
        self.supported_commands.contains(command)
    }

    pub(crate) fn is_feature_supported(&self, feature: SupportedFeatures) -> bool {
        self.supported_features.contains(feature)
    }
}

impl Default for DeviceInformation<'_> {
    fn default() -> Self {
        Self {
            appearance: AppearanceValue::GenericUnknown,
            le_data_packet_length: NonZeroU16::MIN,
            local_name: Default::default(),
            num_le_data_packets: NonZeroU16::MIN,
            public_device_address: Default::default(),
            random_static_device_address: Default::default(),
            supported_commands: Default::default(),
            supported_features: Default::default(),
            supported_le_features: Default::default(),
            supported_le_states: Default::default(),
            tx_power_level: Default::default(),
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
        assert_eq!(device_information.le_data_packet_length, NonZeroU16::MIN);
        assert_eq!(device_information.local_name, "");
        assert_eq!(device_information.num_le_data_packets, NonZeroU16::MIN);
        assert_eq!(
            device_information.public_device_address,
            PublicDeviceAddress::default()
        );
        assert_eq!(device_information.random_static_device_address, None);
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
            le_data_packet_length: NonZeroU16::new(255).unwrap(),
            local_name: "bletio-device",
            num_le_data_packets: NonZeroU16::new(2).unwrap(),
            public_device_address: PublicDeviceAddress::new([0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56]),
            random_static_device_address: Some(
                RandomStaticDeviceAddress::try_new([0x44, 0xDF, 0x1B, 0x09, 0x53, 0xFA]).unwrap(),
            ),
            supported_commands: SupportedCommands::LE_RAND | SupportedCommands::LE_ENCRYPT,
            supported_features: SupportedFeatures::LE_SUPPORTED_CONTROLLER,
            supported_le_features: SupportedLeFeatures::LE_CODED_PHY,
            supported_le_states: SupportedLeStates::default(),
            tx_power_level: TxPowerLevel::try_new(3).unwrap(),
        };
        assert_eq!(
            device_information.appearance,
            AppearanceValue::TemperatureSensor
        );
        assert_eq!(
            device_information.le_data_packet_length,
            NonZeroU16::new(255).unwrap()
        );
        assert_eq!(device_information.local_name, "bletio-device");
        assert_eq!(
            device_information.num_le_data_packets,
            NonZeroU16::new(2).unwrap()
        );
        assert_eq!(
            device_information.public_device_address.value(),
            &[0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56]
        );
        assert_eq!(
            device_information
                .random_static_device_address
                .clone()
                .map(|v| *(v.value())),
            Some([0x44, 0xDF, 0x1B, 0x09, 0x53, 0xFA])
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
