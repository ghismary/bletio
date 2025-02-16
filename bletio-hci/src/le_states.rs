use core::cmp::Ordering;

#[derive(Debug, Clone, Copy, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LeState {
    Single(LeSingleState),
    Combined(LeCombinedState),
}

impl PartialEq for LeState {
    fn eq(&self, other: &Self) -> bool {
        let mut first = *self;
        let mut second = *other;
        first = first.simplify();
        second = second.simplify();
        match (first, second) {
            (LeState::Single(first), LeState::Single(second)) => first == second,
            (LeState::Combined(first), LeState::Combined(second)) => first == second,
            _ => false,
        }
    }
}

impl LeState {
    pub(crate) fn simplify(self) -> Self {
        match &self {
            LeState::Single(_) => self,
            LeState::Combined(LeCombinedState(first, second)) => match first.cmp(second) {
                Ordering::Equal => LeState::Single(*first),
                Ordering::Less => LeState::Combined(LeCombinedState(*first, *second)),
                Ordering::Greater => LeState::Combined(LeCombinedState(*second, *first)),
            },
        }
    }
}

impl From<LeSingleState> for LeState {
    fn from(value: LeSingleState) -> Self {
        Self::Single(value)
    }
}

impl From<LeCombinedState> for LeState {
    fn from(value: LeCombinedState) -> Self {
        Self::Combined(value)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LeSingleState {
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

#[derive(Debug, Clone, Copy, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LeCombinedState(pub LeSingleState, pub LeSingleState);

impl PartialEq for LeCombinedState {
    fn eq(&self, other: &Self) -> bool {
        ((self.0 == other.0) && (self.1 == other.1)) || ((self.0 == other.1) && (self.1 == other.0))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::same_combined_states(
        LeCombinedState(
            LeSingleState::NonConnectableAdvertising,
            LeSingleState::PassiveScanning
        ),
        LeCombinedState(
            LeSingleState::NonConnectableAdvertising,
            LeSingleState::PassiveScanning
        )
    )]
    #[case::inverted_combined_states(
        LeCombinedState(
            LeSingleState::NonConnectableAdvertising,
            LeSingleState::PassiveScanning
        ),
        LeCombinedState(
            LeSingleState::PassiveScanning,
            LeSingleState::NonConnectableAdvertising
        )
    )]
    #[case::equivalent_single_and_combined_states(
        LeSingleState::ActiveScanning,
        LeCombinedState(LeSingleState::ActiveScanning, LeSingleState::ActiveScanning)
    )]
    fn test_le_states_eq(#[case] a: impl Into<LeState>, #[case] b: impl Into<LeState>) {
        assert_eq!(a.into(), b.into());
    }

    #[rstest]
    #[case::different_combined_states(
        LeCombinedState(
            LeSingleState::NonConnectableAdvertising,
            LeSingleState::PassiveScanning
        ),
        LeCombinedState(LeSingleState::ConnectableAdvertising, LeSingleState::ActiveScanning)
    )]
    #[case::different_single_and_combined_states(
        LeSingleState::ActiveScanning,
        LeCombinedState(LeSingleState::ConnectableAdvertising, LeSingleState::PassiveScanning)
    )]
    fn test_le_states_ne(#[case] a: impl Into<LeState>, #[case] b: impl Into<LeState>) {
        assert_ne!(a.into(), b.into());
    }
}
