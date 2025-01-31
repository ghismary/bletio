use crate::le_states::{LeCombinedState, LeSingleState, LeState};

#[derive(Debug, Default)]
pub struct SupportedLeStates {
    value: u64,
}

impl From<u64> for SupportedLeStates {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

impl SupportedLeStates {
    pub fn is_supported(&self, state: LeState) -> bool {
        let state = state.simplify();
        match state {
            LeState::Single(LeSingleState::NonConnectableAdvertising) => {
                (self.value & 0x0000_0000_0000_0001) != 0
            }
            LeState::Single(LeSingleState::ScannableAdvertising) => {
                (self.value & 0x0000_0000_0000_0002) != 0
            }
            LeState::Single(LeSingleState::ConnectableAdvertising) => {
                (self.value & 0x0000_0000_0000_0004) != 0
            }
            LeState::Single(LeSingleState::HighDutyCycleDirectedAdvertising) => {
                (self.value & 0x0000_0000_0000_0008) != 0
            }
            LeState::Single(LeSingleState::PassiveScanning) => {
                (self.value & 0x0000_0000_0000_0010) != 0
            }
            LeState::Single(LeSingleState::ActiveScanning) => {
                (self.value & 0x0000_0000_0000_0020) != 0
            }
            LeState::Single(LeSingleState::Initiating) => (self.value & 0x0000_0000_0000_0040) != 0,
            LeState::Single(LeSingleState::ConnectionSlaveRole) => {
                (self.value & 0x0000_0000_0000_0080) != 0
            }
            LeState::Combined(LeCombinedState(
                LeSingleState::NonConnectableAdvertising,
                LeSingleState::PassiveScanning,
            )) => (self.value & 0x0000_0000_0000_0100) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::ScannableAdvertising,
                LeSingleState::PassiveScanning,
            )) => (self.value & 0x0000_0000_0000_0200) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::ConnectableAdvertising,
                LeSingleState::PassiveScanning,
            )) => (self.value & 0x0000_0000_0000_0400) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::HighDutyCycleDirectedAdvertising,
                LeSingleState::PassiveScanning,
            )) => (self.value & 0x0000_0000_0000_0800) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::NonConnectableAdvertising,
                LeSingleState::ActiveScanning,
            )) => (self.value & 0x0000_0000_0000_1000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::ScannableAdvertising,
                LeSingleState::ActiveScanning,
            )) => (self.value & 0x0000_0000_0000_2000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::ConnectableAdvertising,
                LeSingleState::ActiveScanning,
            )) => (self.value & 0x0000_0000_0000_4000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::HighDutyCycleDirectedAdvertising,
                LeSingleState::ActiveScanning,
            )) => (self.value & 0x0000_0000_0000_8000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::NonConnectableAdvertising,
                LeSingleState::Initiating,
            )) => (self.value & 0x0000_0000_0001_0000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::ScannableAdvertising,
                LeSingleState::Initiating,
            )) => (self.value & 0x0000_0000_0002_0000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::NonConnectableAdvertising,
                LeSingleState::ConnectionMasterRole,
            )) => (self.value & 0x0000_0000_0004_0000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::ScannableAdvertising,
                LeSingleState::ConnectionMasterRole,
            )) => (self.value & 0x0000_0000_0008_0000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::NonConnectableAdvertising,
                LeSingleState::ConnectionSlaveRole,
            )) => (self.value & 0x0000_0000_0010_0000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::ScannableAdvertising,
                LeSingleState::ConnectionSlaveRole,
            )) => (self.value & 0x0000_0000_0020_0000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::PassiveScanning,
                LeSingleState::Initiating,
            )) => (self.value & 0x0000_0000_0040_0000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::ActiveScanning,
                LeSingleState::Initiating,
            )) => (self.value & 0x0000_0000_0080_0000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::PassiveScanning,
                LeSingleState::ConnectionMasterRole,
            )) => (self.value & 0x0000_0000_0100_0000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::ActiveScanning,
                LeSingleState::ConnectionMasterRole,
            )) => (self.value & 0x0000_0000_0200_0000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::PassiveScanning,
                LeSingleState::ConnectionSlaveRole,
            )) => (self.value & 0x0000_0000_0400_0000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::ActiveScanning,
                LeSingleState::ConnectionSlaveRole,
            )) => (self.value & 0x0000_0000_0800_0000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::Initiating,
                LeSingleState::ConnectionMasterRole,
            )) => (self.value & 0x0000_0000_1000_0000) != 0,
            LeState::Single(LeSingleState::LowDutyCycleDirectedAdvertising) => {
                (self.value & 0x0000_0000_2000_0000) != 0
            }
            LeState::Combined(LeCombinedState(
                LeSingleState::LowDutyCycleDirectedAdvertising,
                LeSingleState::PassiveScanning,
            )) => (self.value & 0x0000_0000_4000_0000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::LowDutyCycleDirectedAdvertising,
                LeSingleState::ActiveScanning,
            )) => (self.value & 0x0000_0000_8000_0000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::ConnectableAdvertising,
                LeSingleState::Initiating,
            )) => (self.value & 0x0000_0001_0000_0000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::HighDutyCycleDirectedAdvertising,
                LeSingleState::Initiating,
            )) => (self.value & 0x0000_0002_0000_0000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::LowDutyCycleDirectedAdvertising,
                LeSingleState::Initiating,
            )) => (self.value & 0x0000_0004_0000_0000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::ConnectableAdvertising,
                LeSingleState::ConnectionMasterRole,
            )) => (self.value & 0x0000_0008_0000_0000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::HighDutyCycleDirectedAdvertising,
                LeSingleState::ConnectionMasterRole,
            )) => (self.value & 0x0000_0010_0000_0000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::LowDutyCycleDirectedAdvertising,
                LeSingleState::ConnectionMasterRole,
            )) => (self.value & 0x0000_0020_0000_0000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::ConnectableAdvertising,
                LeSingleState::ConnectionSlaveRole,
            )) => (self.value & 0x0000_0040_0000_0000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::HighDutyCycleDirectedAdvertising,
                LeSingleState::ConnectionSlaveRole,
            )) => (self.value & 0x0000_0080_0000_0000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::LowDutyCycleDirectedAdvertising,
                LeSingleState::ConnectionSlaveRole,
            )) => (self.value & 0x0000_0100_0000_0000) != 0,
            LeState::Combined(LeCombinedState(
                LeSingleState::Initiating,
                LeSingleState::ConnectionSlaveRole,
            )) => (self.value & 0x0000_0200_0000_0000) != 0,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_single_state_is_supported() {
        let supported_states: SupportedLeStates = 0x0000_0000_0000_0022.into();
        assert!(supported_states.is_supported(LeState::Single(LeSingleState::ScannableAdvertising)));
        assert!(supported_states.is_supported(LeState::Single(LeSingleState::ActiveScanning)));
        assert!(!supported_states.is_supported(LeState::Single(LeSingleState::Initiating)));
    }

    #[test]
    fn test_combined_state_is_supported() {
        let supported_states: SupportedLeStates = 0x0000_0010_0400_0000.into();
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::HighDutyCycleDirectedAdvertising,
                LeSingleState::ConnectionMasterRole,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::ConnectionMasterRole,
                LeSingleState::HighDutyCycleDirectedAdvertising,
            )))
        );
        assert!(
            !supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::Initiating,
                LeSingleState::HighDutyCycleDirectedAdvertising,
            )))
        )
    }

    #[test]
    fn test_all_states_supported() {
        let supported_states: SupportedLeStates = 0x0000_03FF_FFFF_FFFF.into();
        assert!(supported_states
            .is_supported(LeState::Single(LeSingleState::NonConnectableAdvertising)));
        assert!(supported_states.is_supported(LeState::Single(LeSingleState::ScannableAdvertising)));
        assert!(
            supported_states.is_supported(LeState::Single(LeSingleState::ConnectableAdvertising))
        );
        assert!(supported_states.is_supported(LeState::Single(
            LeSingleState::HighDutyCycleDirectedAdvertising
        )));
        assert!(supported_states.is_supported(LeState::Single(LeSingleState::PassiveScanning)));
        assert!(supported_states.is_supported(LeState::Single(LeSingleState::ActiveScanning)));
        assert!(supported_states.is_supported(LeState::Single(LeSingleState::Initiating)));
        assert!(supported_states.is_supported(LeState::Single(LeSingleState::ConnectionSlaveRole)));
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::NonConnectableAdvertising,
                LeSingleState::PassiveScanning,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::ScannableAdvertising,
                LeSingleState::PassiveScanning,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::ConnectableAdvertising,
                LeSingleState::PassiveScanning,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::HighDutyCycleDirectedAdvertising,
                LeSingleState::PassiveScanning,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::NonConnectableAdvertising,
                LeSingleState::ActiveScanning,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::ScannableAdvertising,
                LeSingleState::ActiveScanning,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::ConnectableAdvertising,
                LeSingleState::ActiveScanning,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::HighDutyCycleDirectedAdvertising,
                LeSingleState::ActiveScanning,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::NonConnectableAdvertising,
                LeSingleState::Initiating,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::ScannableAdvertising,
                LeSingleState::Initiating,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::NonConnectableAdvertising,
                LeSingleState::ConnectionMasterRole,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::ScannableAdvertising,
                LeSingleState::ConnectionMasterRole,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::NonConnectableAdvertising,
                LeSingleState::ConnectionSlaveRole,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::ScannableAdvertising,
                LeSingleState::ConnectionSlaveRole,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::PassiveScanning,
                LeSingleState::Initiating,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::ActiveScanning,
                LeSingleState::Initiating,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::PassiveScanning,
                LeSingleState::ConnectionMasterRole,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::ActiveScanning,
                LeSingleState::ConnectionMasterRole,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::PassiveScanning,
                LeSingleState::ConnectionSlaveRole,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::ActiveScanning,
                LeSingleState::ConnectionSlaveRole,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::Initiating,
                LeSingleState::ConnectionMasterRole,
            )))
        );
        assert!(supported_states.is_supported(LeState::Single(
            LeSingleState::LowDutyCycleDirectedAdvertising
        )));
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::LowDutyCycleDirectedAdvertising,
                LeSingleState::PassiveScanning,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::LowDutyCycleDirectedAdvertising,
                LeSingleState::ActiveScanning,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::ConnectableAdvertising,
                LeSingleState::Initiating,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::HighDutyCycleDirectedAdvertising,
                LeSingleState::Initiating,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::LowDutyCycleDirectedAdvertising,
                LeSingleState::Initiating,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::ConnectableAdvertising,
                LeSingleState::ConnectionMasterRole,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::HighDutyCycleDirectedAdvertising,
                LeSingleState::ConnectionMasterRole,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::LowDutyCycleDirectedAdvertising,
                LeSingleState::ConnectionMasterRole,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::ConnectableAdvertising,
                LeSingleState::ConnectionSlaveRole,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::HighDutyCycleDirectedAdvertising,
                LeSingleState::ConnectionSlaveRole,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::LowDutyCycleDirectedAdvertising,
                LeSingleState::ConnectionSlaveRole,
            )))
        );
        assert!(
            supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::Initiating,
                LeSingleState::ConnectionSlaveRole,
            )))
        );
    }
}
