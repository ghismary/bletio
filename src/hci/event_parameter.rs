use crate::hci::error_code::HciErrorCode;
use crate::hci::le_features::LeFeatures;
use crate::hci::supported_commands::SupportedCommands;
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
pub(crate) struct SupportedCommandsEventParameter {
    pub(crate) value: SupportedCommands,
}

impl TryFrom<&[u8]> for SupportedCommandsEventParameter {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 64 {
            Err(Error::InvalidEventPacket)
        } else {
            Ok(SupportedCommandsEventParameter {
                value: <[u8; 64]>::try_from(value).unwrap().into(),
            })
        }
    }
}

#[derive(Debug)]
pub(crate) struct LeFeaturesEventParameter {
    pub(crate) value: LeFeatures,
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
