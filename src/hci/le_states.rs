#[derive(Debug)]
pub enum LeState {
    Single(SingleLeState),
    Combined(CombinedLeState),
}

#[derive(Debug, Eq, PartialEq)]
pub enum SingleLeState {
    ScannableAdvertising,
    ConnectableAdvertising,
    NonConnectableAdvertising,
    HighDutyCycleDirectedAdvertising,
    LowDutyCycleDirectedAdvertising,
    ActiveScanning,
    PassiveScanning,
    Initiating,
    ConnectionMasterRole,
    ConnectionSlaveRole,
}

#[derive(Debug)]
pub struct CombinedLeState(pub SingleLeState, pub SingleLeState);

impl PartialEq for CombinedLeState {
    fn eq(&self, other: &Self) -> bool {
        ((self.0 == other.0) && (self.1 == other.1)) || ((self.0 == other.1) && (self.1 == other.0))
    }
}

impl Eq for CombinedLeState {}
