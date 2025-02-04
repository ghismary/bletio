use core::num::NonZeroU16;

use bletio_hci::{SupportedCommands, SupportedFeatures, SupportedLeFeatures, SupportedLeStates};

#[derive(Debug)]
pub(crate) struct ControllerCapabilities {
    pub(crate) supported_commands: SupportedCommands,
    pub(crate) supported_features: SupportedFeatures,
    pub(crate) supported_le_features: SupportedLeFeatures,
    pub(crate) supported_le_states: SupportedLeStates,
    pub(crate) le_data_packet_length: NonZeroU16,
    pub(crate) num_le_data_packets: NonZeroU16,
}

impl Default for ControllerCapabilities {
    fn default() -> Self {
        Self {
            supported_commands: SupportedCommands::default(),
            supported_features: SupportedFeatures::default(),
            supported_le_features: SupportedLeFeatures::default(),
            supported_le_states: SupportedLeStates::default(),
            le_data_packet_length: NonZeroU16::MIN,
            num_le_data_packets: NonZeroU16::MIN,
        }
    }
}
