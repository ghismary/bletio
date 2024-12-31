use crate::hci::error_code::HciErrorCode;
use crate::hci::supported_commands::SupportedCommands;
use crate::hci::supported_features::SupportedFeatures;
use crate::hci::supported_le_features::SupportedLeFeatures;
use crate::hci::supported_le_states::SupportedLeStates;
use crate::Error;

#[derive(Debug)]
enum EventParameter {
    Status(StatusEventParameter),
}

#[derive(Debug)]
pub(crate) struct StatusEventParameter {
    pub(crate) status: HciErrorCode,
}

impl TryFrom<&[u8]> for StatusEventParameter {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(Error::InvalidEventPacket)
        } else {
            Ok(StatusEventParameter {
                status: value[0].try_into()?,
            })
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct LmpFeaturesEventParameter {
    pub(crate) value: SupportedFeatures,
}

impl From<u64> for LmpFeaturesEventParameter {
    fn from(value: u64) -> Self {
        LmpFeaturesEventParameter {
            value: value.into(),
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct SupportedCommandsEventParameter {
    pub(crate) value: SupportedCommands,
}

impl TryFrom<&[u8]> for SupportedCommandsEventParameter {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(SupportedCommandsEventParameter {
            value: <[u8; 64]>::try_from(value)
                .map_err(|_| Error::InvalidEventPacket)?
                .into(),
        })
    }
}

#[derive(Debug)]
pub(crate) struct LeFeaturesEventParameter {
    pub(crate) value: SupportedLeFeatures,
}

impl From<u64> for LeFeaturesEventParameter {
    fn from(value: u64) -> Self {
        LeFeaturesEventParameter {
            value: value.into(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct LeStatesEventParameter {
    pub(crate) value: SupportedLeStates,
}

impl From<u64> for LeStatesEventParameter {
    fn from(value: u64) -> Self {
        LeStatesEventParameter {
            value: value.into(),
        }
    }
}
