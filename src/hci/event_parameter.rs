use crate::hci::error_code::HciErrorCode;
use crate::hci::supported_commands::SupportedCommands;
use crate::hci::supported_features::SupportedFeatures;
use crate::hci::supported_le_features::SupportedLeFeatures;
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

impl TryFrom<&[u8]> for LmpFeaturesEventParameter {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(LmpFeaturesEventParameter {
            value: <[u8; 8]>::try_from(value)
                .map_err(|_| Error::InvalidEventPacket)?
                .into(),
        })
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

impl TryFrom<&[u8]> for LeFeaturesEventParameter {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(LeFeaturesEventParameter {
            value: <[u8; 8]>::try_from(value)
                .map_err(|_| Error::InvalidEventPacket)?
                .into(),
        })
    }
}
