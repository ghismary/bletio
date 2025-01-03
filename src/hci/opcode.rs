use crate::Error;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
enum Ogf {
    Nop = 0x00,
    _LinkControl = 0x01,
    _LinkPolicy = 0x02,
    ControllerAndBaseband = 0x03,
    InformationalParameters = 0x04,
    _StatusParameters = 0x05,
    _Testing = 0x06,
    LeController = 0x08,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Ocf {
    Nop(OcfNop),
    ControllerAndBaseband(OcfControllerAndBaseband),
    InformationalParameters(OcfInformationalParameters),
    LeController(OcfLeController),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u16)]
pub(crate) enum OcfNop {
    Nop = 0x0000,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u16)]
pub(crate) enum OcfControllerAndBaseband {
    SetEventMask = 0x0001,
    Reset = 0x0003,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u16)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum OcfInformationalParameters {
    ReadLocalSupportedCommands = 0x0002,
    ReadLocalSupportedFeatures = 0x0003,
    ReadBufferSize = 0x0005,
    ReadBdAddr = 0x0009,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u16)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum OcfLeController {
    LeSetEventMask = 0x0001,
    LeReadBufferSize = 0x0002,
    LeReadLocalSupportedFeatures = 0x0003,
    LeSetRandomAddress = 0x0005,
    LeSetAdvertisingParameters = 0x0006,
    LeSetAdvertisingData = 0x0008,
    LeSetScanResponseData = 0x0009,
    LeSetAdvertiseEnable = 0x000A,
    LeReadWhiteListSize = 0x000F,
    LeClearWhiteList = 0x0010,
    LeAddDeviceToWhiteList = 0x0011,
    LeRemoveDeviceFromWhiteList = 0x0012,
    LeEncrypt = 0x0017,
    LeRand = 0x0018,
    LeReadSupportedStates = 0x001C,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct OpCode {
    ogf: Ogf,
    ocf: Ocf,
}

impl OpCode {
    fn new(ogf: Ogf, ocf: Ocf) -> Self {
        Self { ogf, ocf }
    }

    pub(crate) fn value(&self) -> u16 {
        let ocf = match self.ocf {
            Ocf::Nop(ocf) => ocf as u16,
            Ocf::ControllerAndBaseband(ocf) => ocf as u16,
            Ocf::InformationalParameters(ocf) => ocf as u16,
            Ocf::LeController(ocf) => ocf as u16,
        };
        ((self.ogf as u16) << 10) + ocf
    }
}

impl TryFrom<u16> for OpCode {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let ogf = ((value & 0xFC00) >> 10) as u8;
        let ocf = value & 0x03FF;
        Ok(match ogf {
            0x00 => match ocf {
                0x0000 => OcfNop::Nop.into(),
                _ => return Err(Error::InvalidOpcode(value)),
            },
            0x01 => return Err(Error::InvalidOpcode(value)),
            0x02 => return Err(Error::InvalidOpcode(value)),
            0x03 => match ocf {
                0x0001 => OcfControllerAndBaseband::SetEventMask.into(),
                0x0003 => OcfControllerAndBaseband::Reset.into(),
                _ => return Err(Error::InvalidOpcode(value)),
            },
            0x04 => match ocf {
                0x0002 => OcfInformationalParameters::ReadLocalSupportedCommands.into(),
                0x0003 => OcfInformationalParameters::ReadLocalSupportedFeatures.into(),
                0x0005 => OcfInformationalParameters::ReadBufferSize.into(),
                0x0009 => OcfInformationalParameters::ReadBdAddr.into(),
                _ => return Err(Error::InvalidOpcode(value)),
            },
            0x05 => return Err(Error::InvalidOpcode(value)),
            0x06 => return Err(Error::InvalidOpcode(value)),
            0x08 => match ocf {
                0x0001 => OcfLeController::LeSetEventMask.into(),
                0x0002 => OcfLeController::LeReadBufferSize.into(),
                0x0003 => OcfLeController::LeReadLocalSupportedFeatures.into(),
                0x0005 => OcfLeController::LeSetRandomAddress.into(),
                0x0006 => OcfLeController::LeSetAdvertisingParameters.into(),
                0x0008 => OcfLeController::LeSetAdvertisingData.into(),
                0x0009 => OcfLeController::LeSetScanResponseData.into(),
                0x000A => OcfLeController::LeSetAdvertiseEnable.into(),
                0x000F => OcfLeController::LeReadWhiteListSize.into(),
                0x0010 => OcfLeController::LeClearWhiteList.into(),
                0x0011 => OcfLeController::LeAddDeviceToWhiteList.into(),
                0x0012 => OcfLeController::LeRemoveDeviceFromWhiteList.into(),
                0x0017 => OcfLeController::LeEncrypt.into(),
                0x0018 => OcfLeController::LeRand.into(),
                0x001C => OcfLeController::LeReadSupportedStates.into(),
                _ => return Err(Error::InvalidOpcode(value)),
            },
            _ => return Err(Error::InvalidOpcode(value)),
        })
    }
}

impl From<OcfNop> for OpCode {
    fn from(value: OcfNop) -> Self {
        Self::new(Ogf::Nop, Ocf::Nop(value))
    }
}

impl From<OcfControllerAndBaseband> for OpCode {
    fn from(value: OcfControllerAndBaseband) -> Self {
        Self::new(
            Ogf::ControllerAndBaseband,
            Ocf::ControllerAndBaseband(value),
        )
    }
}

impl From<OcfInformationalParameters> for OpCode {
    fn from(value: OcfInformationalParameters) -> Self {
        Self::new(
            Ogf::InformationalParameters,
            Ocf::InformationalParameters(value),
        )
    }
}

impl From<OcfLeController> for OpCode {
    fn from(value: OcfLeController) -> Self {
        Self::new(Ogf::LeController, Ocf::LeController(value))
    }
}
