use crate::hci::event_mask::EventMask;
use crate::hci::opcode::{
    OcfControllerAndBaseband, OcfInformationalParameters, OcfLeController, OcfNop, OpCode,
};
use crate::hci::PacketType;
use crate::Error;

#[derive(Debug)]
pub(crate) enum Command {
    // LeAddDeviceToWhiteList(AddressType, Address),
    LeClearWhiteList,
    // LeEncrypt(Key, Data),
    LeRand,
    LeReadBufferSize,
    LeReadLocalSupportedFeatures,
    LeReadSupportedStates,
    LeReadWhiteListSize,
    // LeRemoveDeviceFromWhiteList(AddressType, Address),
    // LeSetEventMask(LeEventMask),
    // LeSetRandomAddress(RandomAddress),
    Nop,
    ReadBdAddr,
    ReadBufferSize,
    ReadLocalSupportedCommands,
    ReadLocalSupportedFeatures,
    Reset,
    SetEventMask(EventMask),
}

impl Command {
    pub(crate) fn encode(&self) -> Result<CommandPacket, Error> {
        Ok(match self {
            Command::LeClearWhiteList
            | Command::LeRand
            | Command::LeReadBufferSize
            | Command::LeReadLocalSupportedFeatures
            | Command::LeReadSupportedStates
            | Command::LeReadWhiteListSize
            | Command::Nop
            | Command::ReadBdAddr
            | Command::ReadBufferSize
            | Command::ReadLocalSupportedCommands
            | Command::ReadLocalSupportedFeatures
            | Command::Reset => CommandPacket::new(self.opcode()),
            Command::SetEventMask(event_mask) => {
                CommandPacket::new(self.opcode()).append(&event_mask.encode()?)
            }
        })
    }

    pub(crate) fn opcode(&self) -> OpCode {
        match self {
            Command::LeClearWhiteList => OcfLeController::LeClearWhiteList.into(),
            Command::LeRand => OcfLeController::LeRand.into(),
            Command::LeReadBufferSize => OcfLeController::LeReadBufferSize.into(),
            Command::LeReadLocalSupportedFeatures => {
                OcfLeController::LeReadLocalSupportedFeatures.into()
            }
            Command::LeReadSupportedStates => OcfLeController::LeReadSupportedStates.into(),
            Command::LeReadWhiteListSize => OcfLeController::LeReadWhiteListSize.into(),
            Command::Nop => OcfNop::Nop.into(),
            Command::ReadBdAddr => OcfInformationalParameters::ReadBdAddr.into(),
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

    pub(crate) fn append(mut self, data: &[u8]) -> Self {
        let data_len = data.len();
        self.buffer[3] += data_len as u8;
        self.buffer[self.len..self.len + data_len].copy_from_slice(data);
        self.len += data_len;
        self
    }

    pub(crate) fn data(&self) -> &[u8] {
        &self.buffer[0..self.len]
    }
}
