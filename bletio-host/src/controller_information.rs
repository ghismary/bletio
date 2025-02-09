use core::num::NonZeroU16;

use bitflags::Flags;

use bletio_hci::{
    PublicDeviceAddress, SupportedCommands, SupportedFeatures, SupportedLeFeatures,
    SupportedLeStates, TxPowerLevel,
};

#[derive(Debug)]
pub(crate) struct ControllerInformation {
    pub(crate) supported_commands: SupportedCommands,
    pub(crate) supported_features: SupportedFeatures,
    pub(crate) supported_le_features: SupportedLeFeatures,
    pub(crate) supported_le_states: SupportedLeStates,
    pub(crate) le_data_packet_length: NonZeroU16,
    pub(crate) num_le_data_packets: NonZeroU16,
    pub(crate) public_device_address: PublicDeviceAddress,
    pub(crate) tx_power_level: TxPowerLevel,
}

impl ControllerInformation {
    pub(crate) fn is_command_supported(&self, command: SupportedCommands) -> bool {
        self.supported_commands.contains(command)
    }

    pub(crate) fn is_feature_supported(&self, feature: SupportedFeatures) -> bool {
        self.supported_features.contains(feature)
    }
}

impl Default for ControllerInformation {
    fn default() -> Self {
        Self {
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
