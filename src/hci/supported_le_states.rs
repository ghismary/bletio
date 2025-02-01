use crate::le_states::{LeCombinedState, LeSingleState, LeState};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SupportedLeStates {
    value: u64,
}

impl From<u64> for SupportedLeStates {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

impl SupportedLeStates {
    pub fn is_supported(&self, state: impl Into<LeState>) -> bool {
        self.is_supported_internal(state.into())
    }

    fn is_supported_internal(&self, state: LeState) -> bool {
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
    use rstest::*;

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

    #[fixture]
    fn all_supported_states() -> SupportedLeStates {
        0x0000_03FF_FFFF_FFFF.into()
    }

    #[rstest]
    fn test_unsupported_combined_state(all_supported_states: SupportedLeStates) {
        assert!(
            !all_supported_states.is_supported(LeState::Combined(LeCombinedState(
                LeSingleState::ConnectionMasterRole,
                LeSingleState::ConnectionSlaveRole
            )))
        );
    }

    #[rstest]
    #[case(LeSingleState::NonConnectableAdvertising)]
    #[case(LeSingleState::ScannableAdvertising)]
    #[case(LeSingleState::ConnectableAdvertising)]
    #[case(LeSingleState::HighDutyCycleDirectedAdvertising)]
    #[case(LeSingleState::PassiveScanning)]
    #[case(LeSingleState::ActiveScanning)]
    #[case(LeSingleState::Initiating)]
    #[case(LeSingleState::ConnectionSlaveRole)]
    #[case(LeCombinedState(
        LeSingleState::NonConnectableAdvertising,
        LeSingleState::PassiveScanning,
    ))]
    #[case(LeCombinedState(LeSingleState::ScannableAdvertising, LeSingleState::PassiveScanning,))]
    #[case(LeCombinedState(LeSingleState::ConnectableAdvertising, LeSingleState::PassiveScanning,))]
    #[case(LeCombinedState(
        LeSingleState::HighDutyCycleDirectedAdvertising,
        LeSingleState::PassiveScanning,
    ))]
    #[case(LeCombinedState(
        LeSingleState::NonConnectableAdvertising,
        LeSingleState::ActiveScanning,
    ))]
    #[case(LeCombinedState(LeSingleState::ScannableAdvertising, LeSingleState::ActiveScanning,))]
    #[case(LeCombinedState(LeSingleState::ConnectableAdvertising, LeSingleState::ActiveScanning,))]
    #[case(LeCombinedState(
        LeSingleState::HighDutyCycleDirectedAdvertising,
        LeSingleState::ActiveScanning,
    ))]
    #[case(LeCombinedState(LeSingleState::NonConnectableAdvertising, LeSingleState::Initiating,))]
    #[case(LeCombinedState(LeSingleState::ScannableAdvertising, LeSingleState::Initiating,))]
    #[case(LeCombinedState(
        LeSingleState::NonConnectableAdvertising,
        LeSingleState::ConnectionMasterRole,
    ))]
    #[case(LeCombinedState(
        LeSingleState::ScannableAdvertising,
        LeSingleState::ConnectionMasterRole,
    ))]
    #[case(LeCombinedState(
        LeSingleState::NonConnectableAdvertising,
        LeSingleState::ConnectionSlaveRole,
    ))]
    #[case(LeCombinedState(
        LeSingleState::ScannableAdvertising,
        LeSingleState::ConnectionSlaveRole,
    ))]
    #[case(LeCombinedState(LeSingleState::PassiveScanning, LeSingleState::Initiating,))]
    #[case(LeCombinedState(LeSingleState::ActiveScanning, LeSingleState::Initiating,))]
    #[case(LeCombinedState(LeSingleState::PassiveScanning, LeSingleState::ConnectionMasterRole,))]
    #[case(LeCombinedState(LeSingleState::ActiveScanning, LeSingleState::ConnectionMasterRole,))]
    #[case(LeCombinedState(LeSingleState::PassiveScanning, LeSingleState::ConnectionSlaveRole,))]
    #[case(LeCombinedState(LeSingleState::ActiveScanning, LeSingleState::ConnectionSlaveRole,))]
    #[case(LeCombinedState(LeSingleState::Initiating, LeSingleState::ConnectionMasterRole,))]
    #[case(LeSingleState::LowDutyCycleDirectedAdvertising)]
    #[case(LeCombinedState(
        LeSingleState::LowDutyCycleDirectedAdvertising,
        LeSingleState::PassiveScanning,
    ))]
    #[case(LeCombinedState(
        LeSingleState::LowDutyCycleDirectedAdvertising,
        LeSingleState::ActiveScanning,
    ))]
    #[case(LeCombinedState(LeSingleState::ConnectableAdvertising, LeSingleState::Initiating,))]
    #[case(LeCombinedState(
        LeSingleState::HighDutyCycleDirectedAdvertising,
        LeSingleState::Initiating,
    ))]
    #[case(LeCombinedState(
        LeSingleState::LowDutyCycleDirectedAdvertising,
        LeSingleState::Initiating,
    ))]
    #[case(LeCombinedState(
        LeSingleState::ConnectableAdvertising,
        LeSingleState::ConnectionMasterRole,
    ))]
    #[case(LeCombinedState(
        LeSingleState::HighDutyCycleDirectedAdvertising,
        LeSingleState::ConnectionMasterRole,
    ))]
    #[case(LeCombinedState(
        LeSingleState::LowDutyCycleDirectedAdvertising,
        LeSingleState::ConnectionMasterRole,
    ))]
    #[case(LeCombinedState(
        LeSingleState::ConnectableAdvertising,
        LeSingleState::ConnectionSlaveRole,
    ))]
    #[case(LeCombinedState(
        LeSingleState::HighDutyCycleDirectedAdvertising,
        LeSingleState::ConnectionSlaveRole,
    ))]
    #[case(LeCombinedState(
        LeSingleState::LowDutyCycleDirectedAdvertising,
        LeSingleState::ConnectionSlaveRole,
    ))]
    #[case(LeCombinedState(LeSingleState::Initiating, LeSingleState::ConnectionSlaveRole,))]
    fn test_all_states_supported(
        all_supported_states: SupportedLeStates,
        #[case] state: impl Into<LeState>,
    ) {
        assert!(all_supported_states.is_supported_internal(state.into()));
    }
}
