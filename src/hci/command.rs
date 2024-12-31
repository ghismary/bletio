use crate::hci::opcode::{
    OcfControllerAndBaseband, OcfInformationalParameters, OcfLeController, OcfNop, OpCode,
};
use crate::hci::PacketType;

#[derive(Debug)]
struct CommandHeader {
    opcode: OpCode,
    parameter_total_length: u8,
}

impl CommandHeader {
    fn new(opcode: OpCode, parameter_total_length: u8) -> Self {
        Self {
            opcode,
            parameter_total_length,
        }
    }

    fn encode(&self, buffer: &mut [u8]) -> usize {
        buffer[0] = (self.opcode.value() & 0xff) as u8;
        buffer[1] = ((self.opcode.value() & 0xff00) >> 8) as u8;
        buffer[2] = self.parameter_total_length;
        3
    }
}

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
    Reset,
    // SetEventMask(EventMask),
}

impl Command {
    pub(crate) fn encode(&self) -> CommandPacket {
        match self {
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
            | Command::Reset => {
                CommandPacket::new().append_command_header(CommandHeader::new(self.opcode(), 0))
            }
        }
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
            Command::Reset => OcfControllerAndBaseband::Reset.into(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct CommandPacket {
    buffer: [u8; 259], // Packet Type (1) + Opcode (2) + Parameter Total Length (1) + Up to 255 bytes of parameters
    len: usize,
}

impl CommandPacket {
    fn new() -> Self {
        let mut s = Self {
            buffer: [0; 259],
            len: 1,
        };
        s.buffer[0] = PacketType::Command as u8;
        s
    }

    pub(crate) fn data(&self) -> &[u8] {
        &self.buffer[0..self.len]
    }

    fn append_command_header(mut self, header: CommandHeader) -> Self {
        self.len += header.encode(&mut self.buffer[self.len..]);
        self
    }
}
