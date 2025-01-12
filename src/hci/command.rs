use crate::advertising::advertising_parameters::AdvertisingParameters;
use crate::advertising::{AdvertisingData, AdvertisingEnable, ScanResponseData};
use crate::hci::event_mask::EventMask;
use crate::hci::opcode::{
    OcfControllerAndBaseband, OcfInformationalParameters, OcfLeController, OpCode,
};
use crate::hci::PacketType;
use crate::Error;

#[derive(Debug)]
pub(crate) enum Command<'a> {
    // LeAddDeviceToWhiteList(AddressType, Address),
    // LeClearWhiteList,
    // LeEncrypt(Key, Data),
    // LeRand,
    LeReadBufferSize,
    LeReadLocalSupportedFeatures,
    LeReadSupportedStates,
    // LeReadWhiteListSize,
    // LeRemoveDeviceFromWhiteList(AddressType, Address),
    // LeSetEventMask(LeEventMask),
    LeSetAdvertiseEnable(AdvertisingEnable),
    LeSetAdvertisingData(&'a AdvertisingData),
    LeSetAdvertisingParameters(&'a AdvertisingParameters),
    // LeSetRandomAddress(RandomAddress),
    LeSetScanResponseData(&'a ScanResponseData),
    // Nop,
    // ReadBdAddr,
    ReadBufferSize,
    ReadLocalSupportedCommands,
    ReadLocalSupportedFeatures,
    Reset,
    SetEventMask(EventMask),
}

impl Command<'_> {
    pub(crate) fn encode(&self) -> Result<CommandPacket, Error> {
        Ok(match self {
            Command::LeReadBufferSize
            | Command::LeReadLocalSupportedFeatures
            | Command::LeReadSupportedStates
            | Command::ReadBufferSize
            | Command::ReadLocalSupportedCommands
            | Command::ReadLocalSupportedFeatures
            | Command::Reset => CommandPacket::new(self.opcode()),
            Command::LeSetAdvertiseEnable(enable) => {
                let buffer = [*enable as u8];
                CommandPacket::new(self.opcode()).append(buffer.as_slice())
            }
            Command::LeSetAdvertisingData(data) => {
                CommandPacket::new(self.opcode()).append(data.encoded_data())
            }
            Command::LeSetAdvertisingParameters(parameters) => {
                let (buffer, len) = parameters.encode()?;
                CommandPacket::new(self.opcode()).append(&buffer[..len])
            }
            Command::LeSetScanResponseData(data) => {
                CommandPacket::new(self.opcode()).append(data.encoded_data())
            }
            Command::SetEventMask(event_mask) => {
                CommandPacket::new(self.opcode()).append(&event_mask.encode()?)
            }
        })
    }

    pub(crate) fn opcode(&self) -> OpCode {
        match self {
            Command::LeReadBufferSize => OcfLeController::LeReadBufferSize.into(),
            Command::LeReadLocalSupportedFeatures => {
                OcfLeController::LeReadLocalSupportedFeatures.into()
            }
            Command::LeReadSupportedStates => OcfLeController::LeReadSupportedStates.into(),
            Command::LeSetAdvertiseEnable(_) => OcfLeController::LeSetAdvertiseEnable.into(),
            Command::LeSetAdvertisingData(_) => OcfLeController::LeSetAdvertisingData.into(),
            Command::LeSetAdvertisingParameters(_) => {
                OcfLeController::LeSetAdvertisingParameters.into()
            }
            Command::LeSetScanResponseData(_) => OcfLeController::LeSetScanResponseData.into(),
            Command::ReadBufferSize => OcfInformationalParameters::ReadBufferSize.into(),
            Command::ReadLocalSupportedCommands => {
                OcfInformationalParameters::ReadLocalSupportedCommands.into()
            }
            Command::ReadLocalSupportedFeatures => {
                OcfInformationalParameters::ReadLocalSupportedFeatures.into()
            }
            Command::Reset => OcfControllerAndBaseband::Reset.into(),
            Command::SetEventMask(_) => OcfControllerAndBaseband::SetEventMask.into(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct CommandPacket {
    buffer: [u8; 259], // Packet Type (1) + Opcode (2) + Parameter Total Length (1) + Up to 255 bytes of parameters
    len: usize,
}

impl CommandPacket {
    fn new(opcode: OpCode) -> Self {
        let mut packet = Self {
            buffer: [0; 259],
            len: 4,
        };
        packet.buffer[0] = PacketType::Command as u8;
        packet.buffer[1] = (opcode.value() & 0xff) as u8;
        packet.buffer[2] = ((opcode.value() & 0xff00) >> 8) as u8;
        packet
    }

    #[must_use]
    pub(crate) fn append(self, data: &[u8]) -> Self {
        let mut packet = self;
        let data_len = data.len();
        packet.buffer[3] += data_len as u8;
        packet.buffer[packet.len..packet.len + data_len].copy_from_slice(data);
        packet.len += data_len;
        packet
    }

    pub(crate) fn data(&self) -> &[u8] {
        &self.buffer[0..self.len]
    }
}
