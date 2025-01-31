use crate::advertising::advertising_parameters::AdvertisingParameters;
use crate::advertising::{AdvertisingData, AdvertisingEnable, ScanResponseData};
use crate::hci::event_mask::EventMask;
use crate::hci::HciError;
use crate::hci::HciPacketType;
use crate::utils::Buffer;
use crate::Error;

macro_rules! hci_command_opcodes {
    (
        $(
            $(#[$docs:meta])*
            ($opcode:ident, $const:ident) = ($ogf:expr, $ocf:literal),
        )+
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr(u16)]
        pub(crate) enum HciCommandOpCode {
            $(
                $(#[$docs])*
                $opcode = $const,
            )+
            Unsupported(u16),
        }

        impl HciCommandOpCode {
            pub(crate) fn value(&self) -> u16 {
                match self {
                    $(
                        Self::$opcode => $const,
                    )+
                    Self::Unsupported(v) => *v,
                }
            }
        }

        $(
            #[allow(non_upper_case_globals)]
            const $const: u16 = ($ogf << 10) + $ocf;
        )+

        impl From<u16> for HciCommandOpCode {
            fn from(value: u16) -> Self {
                match value {
                    $(
                        $const => HciCommandOpCode::$opcode,
                    )+
                    _ => HciCommandOpCode::Unsupported(value),
                }
            }
        }
    };
}

const NOP_OGF: u16 = 0x00;
const CONTROLLER_AND_BASEBAND_OGF: u16 = 0x03;
const INFORMATIONAL_PARAMETERS_OGF: u16 = 0x04;
const LE_CONTROLLER_OGF: u16 = 0x08;

hci_command_opcodes! {
    (Nop, NOP) = (NOP_OGF, 0x0000),
    (SetEventMask, SET_EVENT_MASK) = (CONTROLLER_AND_BASEBAND_OGF, 0x0001),
    (Reset, RESET) = (CONTROLLER_AND_BASEBAND_OGF, 0x0003),
    (ReadLocalSupportedCommands, READ_LOCAL_SUPPORTED_COMMANDS) = (INFORMATIONAL_PARAMETERS_OGF, 0x0002),
    (ReadLocalSupportedFeatures, READ_LOCAL_SUPPORTED_FEATURES) = (INFORMATIONAL_PARAMETERS_OGF, 0x0003),
    (ReadBufferSize, READ_BUFFER_SIZE) = (INFORMATIONAL_PARAMETERS_OGF, 0x0005),
    // (ReadBdAddr, READ_BD_ADDR) = (INFORMATIONAL_PARAMETERS_OGF, 0x0009),
    // (LeSetEventMask, LE_SET_EVENT_MASK) = (LE_CONTROLLER_OGF, 0x0001),
    (LeReadBufferSize, LE_READ_BUFFER_SIZE) = (LE_CONTROLLER_OGF, 0x0002),
    (LeReadLocalSupportedFeaturesPage0, LE_READ_LOCAL_SUPPORTED_FEATURES_PAGE_0) = (LE_CONTROLLER_OGF, 0x0003),
    // (LeSetRandomAddress, LE_SET_RANDOM_ADDRESS) = (LE_CONTROLLER_OGF, 0x0005),
    (LeSetAdvertisingParameters, LE_SET_ADVERTISING_PARAMETERS) = (LE_CONTROLLER_OGF, 0x0006),
    (LeSetAdvertisingData, LE_SET_ADVERTISING_DATA) = (LE_CONTROLLER_OGF, 0x0008),
    (LeSetScanResponseData, LE_SET_SCAN_RESPONSE_DATA) = (LE_CONTROLLER_OGF, 0x0009),
    (LeSetAdvertisingEnable, LE_SET_ADVERTISING_ENABLE) = (LE_CONTROLLER_OGF, 0x000A),
    // (LeReadFilterAcceptListSize, LE_READ_FILTER_ACCEPT_LIST_SIZE) = (LE_CONTROLLER_OGF, 0x000F),
    // (LeClearFilterAcceptList, LE_CLEAR_FILTER_ACCEPT_LIST) = (LE_CONTROLLER_OGF, 0x0010),
    // (LeAddDeviceToFilterAcceptList, LE_ADD_DEVICE_TO_FILTER_ACCEPT_LIST) = (LE_CONTROLLER_OGF, 0x0011),
    // (LeRemoveDeviceFromFilterAcceptList, LE_REMOVE_DEVICE_FROM_FILTER_ACCEPT_LIST) = (LE_CONTROLLER_OGF, 0x0012),
    // (LeEncrypt, LE_ENCRYPT) = (LE_CONTROLLER_OGF, 0x0017),
    // (LeRand, LE_RAND) = (LE_CONTROLLER_OGF, 0x0018),
    (LeReadSupportedStates, LE_READ_SUPPORTED_STATES) = (LE_CONTROLLER_OGF, 0x001C),
}

#[derive(Debug)]
pub(crate) enum HciCommand<'a> {
    // LeAddDeviceToFilterAcceptList(AddressType, Address),
    // LeClearFilterAcceptList,
    // LeEncrypt(Key, Data),
    // LeRand,
    LeReadBufferSize,
    LeReadLocalSupportedFeaturesPage0,
    LeReadSupportedStates,
    // LeReadFilterAcceptListSize,
    // LeRemoveDeviceFromFilterAcceptList(AddressType, Address),
    // LeSetEventMask(LeEventMask),
    LeSetAdvertisingEnable(AdvertisingEnable),
    LeSetAdvertisingData(&'a AdvertisingData),
    LeSetAdvertisingParameters(&'a AdvertisingParameters),
    // LeSetRandomAddress(RandomAddress),
    LeSetScanResponseData(&'a ScanResponseData),
    Nop,
    // ReadBdAddr,
    ReadBufferSize,
    ReadLocalSupportedCommands,
    ReadLocalSupportedFeatures,
    Reset,
    SetEventMask(EventMask),
    Unsupported(u16),
}

impl HciCommand<'_> {
    pub(crate) fn encode(&self) -> Result<CommandPacket, Error> {
        Ok(match self {
            HciCommand::LeReadBufferSize
            | HciCommand::LeReadLocalSupportedFeaturesPage0
            | HciCommand::LeReadSupportedStates
            | HciCommand::Nop
            | HciCommand::ReadBufferSize
            | HciCommand::ReadLocalSupportedCommands
            | HciCommand::ReadLocalSupportedFeatures
            | HciCommand::Reset => CommandPacket::new(self.opcode()),
            HciCommand::LeSetAdvertisingEnable(enable) => {
                let buffer = [*enable as u8];
                CommandPacket::new(self.opcode()).append(buffer.as_slice())?
            }
            HciCommand::LeSetAdvertisingData(data) => {
                CommandPacket::new(self.opcode()).append(data.encoded_data())?
            }
            HciCommand::LeSetAdvertisingParameters(parameters) => {
                CommandPacket::new(self.opcode()).append(parameters.encoded_data())?
            }
            HciCommand::LeSetScanResponseData(data) => {
                CommandPacket::new(self.opcode()).append(data.encoded_data())?
            }
            HciCommand::SetEventMask(event_mask) => {
                CommandPacket::new(self.opcode()).append(&event_mask.encode())?
            }
            HciCommand::Unsupported(opcode) => {
                return Err(Error::Hci(HciError::InvalidCommand(*opcode)))
            }
        })
    }

    pub(crate) fn opcode(&self) -> HciCommandOpCode {
        match self {
            // Self::LeClearFilterAcceptList => HciCommandOpCode::LeClearFilterAcceptList,
            // Self::LeRand => HciCommandOpCode::LeRand,
            Self::LeReadBufferSize => HciCommandOpCode::LeReadBufferSize,
            // Self::LeReadFilterAcceptListSize => HciCommandOpCode::LeReadFilterAcceptListSize,
            Self::LeReadLocalSupportedFeaturesPage0 => {
                HciCommandOpCode::LeReadLocalSupportedFeaturesPage0
            }
            Self::LeReadSupportedStates => HciCommandOpCode::LeReadSupportedStates,
            Self::LeSetAdvertisingEnable(_) => HciCommandOpCode::LeSetAdvertisingEnable,
            Self::LeSetAdvertisingData(_) => HciCommandOpCode::LeSetAdvertisingData,
            Self::LeSetAdvertisingParameters(_) => HciCommandOpCode::LeSetAdvertisingParameters,
            Self::LeSetScanResponseData(_) => HciCommandOpCode::LeSetScanResponseData,
            Self::Nop => HciCommandOpCode::Nop,
            // Self::ReadBdAddr => HciCommandOpCode::ReadBdAddr,
            Self::ReadBufferSize => HciCommandOpCode::ReadBufferSize,
            Self::ReadLocalSupportedCommands => HciCommandOpCode::ReadLocalSupportedCommands,
            Self::ReadLocalSupportedFeatures => HciCommandOpCode::ReadLocalSupportedFeatures,
            Self::Reset => HciCommandOpCode::Reset,
            Self::SetEventMask(_) => HciCommandOpCode::SetEventMask,
            Self::Unsupported(opcode) => HciCommandOpCode::Unsupported(*opcode),
        }
    }
}

// Packet Type (1) + Opcode (2) + Parameter Total Length (1) + Up to 255 bytes of parameters
const HCI_COMMAND_MAX_SIZE: usize = 259;

const HCI_COMMAND_PACKET_TYPE_OFFSET: usize = 0;
const HCI_COMMAND_PACKET_OPCODE_OFFSET: usize = 1;
const HCI_COMMAND_PACKET_LENGTH_OFFSET: usize = 3;
const HCI_COMMAND_PACKET_DATA_OFFSET: usize = 4;

#[derive(Debug)]
pub(crate) struct CommandPacket {
    buffer: Buffer<HCI_COMMAND_MAX_SIZE>,
}

impl CommandPacket {
    fn new(opcode: HciCommandOpCode) -> Self {
        let mut buffer = Buffer {
            data: [0; HCI_COMMAND_MAX_SIZE],
            offset: HCI_COMMAND_PACKET_OPCODE_OFFSET,
        };
        buffer.data[HCI_COMMAND_PACKET_TYPE_OFFSET] = HciPacketType::Command as u8;
        // INVARIANT: The buffer space is known to be enough.
        buffer.encode_le_u16(opcode.value()).unwrap();
        buffer.offset = HCI_COMMAND_PACKET_DATA_OFFSET;
        Self { buffer }
    }

    pub(crate) fn append(self, data: &[u8]) -> Result<Self, HciError> {
        let mut packet = self;
        let data_len = data.len();
        packet.buffer.data[HCI_COMMAND_PACKET_LENGTH_OFFSET] += data_len as u8;
        packet
            .buffer
            .copy_from_slice(data)
            .map_err(|_| HciError::DataWillNotFitCommandPacket)?;
        Ok(packet)
    }

    pub(crate) fn data(&self) -> &[u8] {
        self.buffer.data()
    }
}

pub(crate) mod parser {
    use nom::{bytes::take, combinator::map, number::le_u16, sequence::pair, IResult, Parser};

    use crate::advertising::parser::advertising_enable;
    use crate::hci::event_mask::parser::event_mask;
    use crate::hci::packet::parser::parameter_total_length;
    use crate::hci::{HciCommand, HciCommandOpCode, HciPacket};

    pub(crate) fn command_opcode(input: &[u8]) -> IResult<&[u8], HciCommandOpCode> {
        map(le_u16(), HciCommandOpCode::from).parse(input)
    }

    pub(crate) fn command(input: &[u8]) -> IResult<&[u8], HciPacket> {
        let (input, (command_opcode, parameter_total_length)) =
            pair(command_opcode, parameter_total_length).parse(input)?;
        let (input, parameters) = take(parameter_total_length).parse(input)?;
        Ok((
            input,
            HciPacket::Command(match command_opcode {
                HciCommandOpCode::Nop => HciCommand::Nop,
                HciCommandOpCode::SetEventMask => {
                    let (_, event_mask) = event_mask(parameters)?;
                    HciCommand::SetEventMask(event_mask)
                }
                HciCommandOpCode::Reset => HciCommand::Reset,
                HciCommandOpCode::ReadLocalSupportedCommands => {
                    HciCommand::ReadLocalSupportedCommands
                }
                HciCommandOpCode::ReadLocalSupportedFeatures => {
                    HciCommand::ReadLocalSupportedFeatures
                }
                HciCommandOpCode::ReadBufferSize => HciCommand::ReadBufferSize,
                // HciCommandOpCode::ReadBdAddr => HciCommand::ReadBdAddr,
                HciCommandOpCode::LeReadBufferSize => HciCommand::LeReadBufferSize,
                HciCommandOpCode::LeReadLocalSupportedFeaturesPage0 => {
                    HciCommand::LeReadLocalSupportedFeaturesPage0
                }
                HciCommandOpCode::LeSetAdvertisingEnable => {
                    let (_, advertising_enable) = advertising_enable(parameters)?;
                    HciCommand::LeSetAdvertisingEnable(advertising_enable)
                }
                HciCommandOpCode::LeSetAdvertisingData
                | HciCommandOpCode::LeSetAdvertisingParameters
                | HciCommandOpCode::LeSetScanResponseData => {
                    todo!()
                }
                // HciCommandOpCode::LeReadFilterAcceptListSize => {
                //     HciCommand::LeReadFilterAcceptListSize
                // }
                // HciCommandOpCode::LeClearFilterAcceptList => HciCommand::LeClearFilterAcceptList,
                // HciCommandOpCode::LeRand => HciCommand::LeRand,
                HciCommandOpCode::LeReadSupportedStates => HciCommand::LeReadSupportedStates,
                HciCommandOpCode::Unsupported(opcode) => HciCommand::Unsupported(opcode),
            }),
        ))
    }
}

// TEST [1, 3, 12, 0]
