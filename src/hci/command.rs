use crate::hci::PacketType;

#[derive(Debug)]
struct Buffer {
    buffer: [u8; 259], // Packet Type (1) + Opcode (2) + Parameter Total Length (1) + Up to 255 bytes of parameters
    len: usize,
}

impl Buffer {
    fn new() -> Self {
        let mut s = Self {
            buffer: [0; 259],
            len: 1,
        };
        s.buffer[0] = PacketType::Command as u8;
        s
    }

    fn data(&self) -> &[u8] {
        &self.buffer[0..self.len]
    }

    fn append_command_header(mut self, header: CommandHeader) -> Self {
        header.encode(&mut self.buffer[self.len..]);
        self.len += header.len as usize;
        self
    }
}

#[derive(Debug, Clone, Copy)]
enum Ogf {
    Nop = 0x00,
    LinkControl = 0x01,
    LinkPolicy = 0x02,
    ControllerAndBaseband = 0x03,
    InformationalParameters = 0x04,
    StatusParameters = 0x05,
    Testing = 0x06,
    LeController = 0x08,
}

#[derive(Debug, Clone, Copy)]
enum Ocf {
    Nop(OcfNop),
    ControllerAndBaseband(OcfControllerAndBaseband),
    InformationalParameters(OcfInformationalParameters),
    LeController(OcfLeController),
}

#[derive(Debug, Clone, Copy)]
enum OcfNop {
    Nop = 0x0000,
}

#[derive(Debug, Clone, Copy)]
enum OcfControllerAndBaseband {
    SetEventMask = 0x0001,
    Reset = 0x0003,
}

#[derive(Debug, Clone, Copy)]
enum OcfInformationalParameters {
    ReadBufferSize = 0x0005,
    ReadBdAddr = 0x0009,
}

#[derive(Debug, Clone, Copy)]
enum OcfLeController {
    LeSetEventMask = 0x0001,
    LeReadBufferSize = 0x0002,
    LeReadLocalSupportedFeatures = 0x0003,
    LeSetRandomAddress = 0x0005,
    LeReadWhiteListSize = 0x000F,
    LeClearWhiteList = 0x0010,
    LeAddDeviceToWhiteList = 0x0011,
    LeRemoveDeviceFromWhiteList = 0x0012,
    LeEncrypt = 0x0017,
    LeRand = 0x0018,
    LeReadSupportedStates = 0x001C,
}

#[derive(Debug)]
struct Opcode {
    ogf: Ogf,
    ocf: Ocf,
}

impl Opcode {
    fn new(ogf: Ogf, ocf: Ocf) -> Self {
        Self { ogf, ocf }
    }

    fn value(&self) -> u16 {
        let ocf = match self.ocf {
            Ocf::Nop(ocf) => ocf as u16,
            Ocf::ControllerAndBaseband(ocf) => ocf as u16,
            Ocf::InformationalParameters(ocf) => ocf as u16,
            Ocf::LeController(ocf) => ocf as u16,
        };
        ((self.ogf as u16) << 10) + ocf
    }
}

impl From<OcfNop> for Opcode {
    fn from(value: OcfNop) -> Self {
        Self::new(Ogf::Nop, Ocf::Nop(value))
    }
}

impl From<OcfControllerAndBaseband> for Opcode {
    fn from(value: OcfControllerAndBaseband) -> Self {
        Self::new(
            Ogf::ControllerAndBaseband,
            Ocf::ControllerAndBaseband(value),
        )
    }
}

impl From<OcfInformationalParameters> for Opcode {
    fn from(value: OcfInformationalParameters) -> Self {
        Self::new(
            Ogf::InformationalParameters,
            Ocf::InformationalParameters(value),
        )
    }
}

impl From<OcfLeController> for Opcode {
    fn from(value: OcfLeController) -> Self {
        Self::new(Ogf::LeController, Ocf::LeController(value))
    }
}

#[derive(Debug)]
struct CommandHeader {
    opcode: Opcode,
    len: u8,
}

impl CommandHeader {
    fn new(opcode: Opcode, len: u8) -> Self {
        Self { opcode, len }
    }

    fn encode(&self, buffer: &mut [u8]) {
        buffer[0] = (self.opcode.value() & 0xff) as u8;
        buffer[1] = ((self.opcode.value() & 0xff00) >> 8) as u8;
        buffer[2] = self.len;
    }
}

#[derive(Debug)]
enum Command {
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
    Reset,
    // SetEventMask(EventMask),
}

impl Command {
    fn encode(self) -> Buffer {
        match self {
            Command::LeClearWhiteList => Buffer::new().append_command_header(CommandHeader::new(
                OcfLeController::LeClearWhiteList.into(),
                0,
            )),
            Command::LeRand => Buffer::new()
                .append_command_header(CommandHeader::new(OcfLeController::LeRand.into(), 0)),
            Command::LeReadBufferSize => Buffer::new().append_command_header(CommandHeader::new(
                OcfLeController::LeReadBufferSize.into(),
                0,
            )),
            Command::LeReadLocalSupportedFeatures => Buffer::new().append_command_header(
                CommandHeader::new(OcfLeController::LeReadLocalSupportedFeatures.into(), 0),
            ),
            Command::LeReadSupportedStates => Buffer::new().append_command_header(
                CommandHeader::new(OcfLeController::LeReadSupportedStates.into(), 0),
            ),
            Command::LeReadWhiteListSize => Buffer::new().append_command_header(
                CommandHeader::new(OcfLeController::LeReadWhiteListSize.into(), 0),
            ),
            Command::Nop => {
                Buffer::new().append_command_header(CommandHeader::new(OcfNop::Nop.into(), 0))
            }
            Command::ReadBdAddr => Buffer::new().append_command_header(CommandHeader::new(
                OcfInformationalParameters::ReadBdAddr.into(),
                0,
            )),
            Command::ReadBufferSize => Buffer::new().append_command_header(CommandHeader::new(
                OcfInformationalParameters::ReadBufferSize.into(),
                0,
            )),
            Command::Reset => Buffer::new().append_command_header(CommandHeader::new(
                OcfControllerAndBaseband::Reset.into(),
                0,
            )),
        }
    }
}
