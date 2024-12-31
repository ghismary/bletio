use crate::hci::le_states::{CombinedLeState, LeState, SingleLeState};

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
        match state {
            LeState::Single(SingleLeState::NonConnectableAdvertising)
            | LeState::Combined(CombinedLeState(
                SingleLeState::NonConnectableAdvertising,
                SingleLeState::NonConnectableAdvertising,
            )) => (self.value & 0x0000_0000_0000_0001) != 0,
            LeState::Single(SingleLeState::ScannableAdvertising)
            | LeState::Combined(CombinedLeState(
                SingleLeState::ScannableAdvertising,
                SingleLeState::ScannableAdvertising,
            )) => (self.value & 0x0000_0000_0000_0002) != 0,
            LeState::Single(SingleLeState::ConnectableAdvertising)
            | LeState::Combined(CombinedLeState(
                SingleLeState::ConnectableAdvertising,
                SingleLeState::ConnectableAdvertising,
            )) => (self.value & 0x0000_0000_0000_0004) != 0,
            LeState::Single(SingleLeState::HighDutyCycleDirectedAdvertising)
            | LeState::Combined(CombinedLeState(
                SingleLeState::HighDutyCycleDirectedAdvertising,
                SingleLeState::HighDutyCycleDirectedAdvertising,
            )) => (self.value & 0x0000_0000_0000_0008) != 0,
            LeState::Single(SingleLeState::PassiveScanning)
            | LeState::Combined(CombinedLeState(
                SingleLeState::PassiveScanning,
                SingleLeState::PassiveScanning,
            )) => (self.value & 0x0000_0000_0000_0010) != 0,
            LeState::Single(SingleLeState::ActiveScanning)
            | LeState::Combined(CombinedLeState(
                SingleLeState::ActiveScanning,
                SingleLeState::ActiveScanning,
            )) => (self.value & 0x0000_0000_0000_0020) != 0,
            LeState::Single(SingleLeState::Initiating)
            | LeState::Combined(CombinedLeState(
                SingleLeState::Initiating,
                SingleLeState::Initiating,
            )) => (self.value & 0x0000_0000_0000_0040) != 0,
            LeState::Single(SingleLeState::ConnectionSlaveRole)
            | LeState::Combined(CombinedLeState(
                SingleLeState::ConnectionSlaveRole,
                SingleLeState::ConnectionSlaveRole,
            )) => (self.value & 0x0000_0000_0000_0080) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::NonConnectableAdvertising,
                SingleLeState::PassiveScanning,
            )) => (self.value & 0x0000_0000_0000_0100) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::ScannableAdvertising,
                SingleLeState::PassiveScanning,
            )) => (self.value & 0x0000_0000_0000_0200) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::ConnectableAdvertising,
                SingleLeState::PassiveScanning,
            )) => (self.value & 0x0000_0000_0000_0400) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::HighDutyCycleDirectedAdvertising,
                SingleLeState::PassiveScanning,
            )) => (self.value & 0x0000_0000_0000_0800) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::NonConnectableAdvertising,
                SingleLeState::ActiveScanning,
            )) => (self.value & 0x0000_0000_0000_1000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::ScannableAdvertising,
                SingleLeState::ActiveScanning,
            )) => (self.value & 0x0000_0000_0000_2000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::ConnectableAdvertising,
                SingleLeState::ActiveScanning,
            )) => (self.value & 0x0000_0000_0000_4000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::HighDutyCycleDirectedAdvertising,
                SingleLeState::ActiveScanning,
            )) => (self.value & 0x0000_0000_0000_8000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::NonConnectableAdvertising,
                SingleLeState::Initiating,
            )) => (self.value & 0x0000_0000_0001_0000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::ScannableAdvertising,
                SingleLeState::Initiating,
            )) => (self.value & 0x0000_0000_0002_0000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::NonConnectableAdvertising,
                SingleLeState::ConnectionMasterRole,
            )) => (self.value & 0x0000_0000_0004_0000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::ScannableAdvertising,
                SingleLeState::ConnectionMasterRole,
            )) => (self.value & 0x0000_0000_0008_0000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::NonConnectableAdvertising,
                SingleLeState::ConnectionSlaveRole,
            )) => (self.value & 0x0000_0000_0010_0000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::ScannableAdvertising,
                SingleLeState::ConnectionSlaveRole,
            )) => (self.value & 0x0000_0000_0020_0000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::PassiveScanning,
                SingleLeState::Initiating,
            )) => (self.value & 0x0000_0000_0040_0000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::ActiveScanning,
                SingleLeState::Initiating,
            )) => (self.value & 0x0000_0000_0080_0000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::PassiveScanning,
                SingleLeState::ConnectionMasterRole,
            )) => (self.value & 0x0000_0000_0100_0000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::ActiveScanning,
                SingleLeState::ConnectionMasterRole,
            )) => (self.value & 0x0000_0000_0200_0000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::PassiveScanning,
                SingleLeState::ConnectionSlaveRole,
            )) => (self.value & 0x0000_0000_0400_0000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::ActiveScanning,
                SingleLeState::ConnectionSlaveRole,
            )) => (self.value & 0x0000_0000_0800_0000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::Initiating,
                SingleLeState::ConnectionMasterRole,
            )) => (self.value & 0x0000_0000_1000_0000) != 0,
            LeState::Single(SingleLeState::LowDutyCycleDirectedAdvertising)
            | LeState::Combined(CombinedLeState(
                SingleLeState::LowDutyCycleDirectedAdvertising,
                SingleLeState::LowDutyCycleDirectedAdvertising,
            )) => (self.value & 0x0000_0000_2000_0000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::LowDutyCycleDirectedAdvertising,
                SingleLeState::PassiveScanning,
            )) => (self.value & 0x0000_0000_4000_0000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::LowDutyCycleDirectedAdvertising,
                SingleLeState::ActiveScanning,
            )) => (self.value & 0x0000_0000_8000_0000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::ConnectableAdvertising,
                SingleLeState::Initiating,
            )) => (self.value & 0x0000_0001_0000_0000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::HighDutyCycleDirectedAdvertising,
                SingleLeState::Initiating,
            )) => (self.value & 0x0000_0002_0000_0000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::LowDutyCycleDirectedAdvertising,
                SingleLeState::Initiating,
            )) => (self.value & 0x0000_0004_0000_0000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::ConnectableAdvertising,
                SingleLeState::ConnectionMasterRole,
            )) => (self.value & 0x0000_0008_0000_0000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::HighDutyCycleDirectedAdvertising,
                SingleLeState::ConnectionMasterRole,
            )) => (self.value & 0x0000_0010_0000_0000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::LowDutyCycleDirectedAdvertising,
                SingleLeState::ConnectionMasterRole,
            )) => (self.value & 0x0000_0020_0000_0000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::ConnectableAdvertising,
                SingleLeState::ConnectionSlaveRole,
            )) => (self.value & 0x0000_0040_0000_0000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::HighDutyCycleDirectedAdvertising,
                SingleLeState::ConnectionSlaveRole,
            )) => (self.value & 0x0000_0080_0000_0000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::LowDutyCycleDirectedAdvertising,
                SingleLeState::ConnectionSlaveRole,
            )) => (self.value & 0x0000_0100_0000_0000) != 0,
            LeState::Combined(CombinedLeState(
                SingleLeState::Initiating,
                SingleLeState::ConnectionSlaveRole,
            )) => (self.value & 0x0000_0200_0000_0000) != 0,
            _ => false,
        }
    }
}
