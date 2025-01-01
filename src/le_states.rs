use core::cmp::Ordering;

#[derive(Debug, Clone, Copy, Eq)]
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
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
pub struct LeCombinedState(pub LeSingleState, pub LeSingleState);

impl PartialEq for LeCombinedState {
    fn eq(&self, other: &Self) -> bool {
        ((self.0 == other.0) && (self.1 == other.1)) || ((self.0 == other.1) && (self.1 == other.0))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_same_combined_states_eq() {
        assert_eq!(
            LeState::Combined(LeCombinedState(
                LeSingleState::NonConnectableAdvertising,
                LeSingleState::PassiveScanning
            )),
            LeState::Combined(LeCombinedState(
                LeSingleState::NonConnectableAdvertising,
                LeSingleState::PassiveScanning
            ))
        )
    }

    #[test]
    fn test_different_combined_states_ne() {
        assert_ne!(
            LeState::Combined(LeCombinedState(
                LeSingleState::NonConnectableAdvertising,
                LeSingleState::PassiveScanning
            )),
            LeState::Combined(LeCombinedState(
                LeSingleState::ConnectableAdvertising,
                LeSingleState::ActiveScanning
            ))
        )
    }
    #[test]
    fn test_inverted_combined_states_eq() {
        assert_eq!(
            LeState::Combined(LeCombinedState(
                LeSingleState::NonConnectableAdvertising,
                LeSingleState::PassiveScanning
            )),
            LeState::Combined(LeCombinedState(
                LeSingleState::PassiveScanning,
                LeSingleState::NonConnectableAdvertising
            ))
        )
    }

    #[test]
    fn test_equivalent_single_and_combined_states_eq() {
        assert_eq!(
            LeState::Single(LeSingleState::ActiveScanning),
            LeState::Combined(LeCombinedState(
                LeSingleState::ActiveScanning,
                LeSingleState::ActiveScanning
            ))
        )
    }
}
