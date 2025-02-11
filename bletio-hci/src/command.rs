use bletio_utils::{Buffer, BufferOps, EncodeToBuffer};
use num_enum::{FromPrimitive, IntoPrimitive};

use crate::{
    AdvertisingData, AdvertisingEnable, AdvertisingParameters, Error, EventMask, LeEventMask,
    PacketType, RandomStaticDeviceAddress, ScanResponseData,
};

const NOP_OGF: u16 = 0x00;
const CONTROLLER_AND_BASEBAND_OGF: u16 = 0x03;
const INFORMATIONAL_PARAMETERS_OGF: u16 = 0x04;
const LE_CONTROLLER_OGF: u16 = 0x08;

const fn opcode(ogf: u16, ocf: u16) -> u16 {
    ogf << 10 | ocf
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, FromPrimitive)]
#[repr(u16)]
pub(crate) enum CommandOpCode {
    Nop = opcode(NOP_OGF, 0x0000),
    SetEventMask = opcode(CONTROLLER_AND_BASEBAND_OGF, 0x0001),
    Reset = opcode(CONTROLLER_AND_BASEBAND_OGF, 0x0003),
    ReadLocalSupportedCommands = opcode(INFORMATIONAL_PARAMETERS_OGF, 0x0002),
    ReadLocalSupportedFeatures = opcode(INFORMATIONAL_PARAMETERS_OGF, 0x0003),
    ReadBufferSize = opcode(INFORMATIONAL_PARAMETERS_OGF, 0x0005),
    ReadBdAddr = opcode(INFORMATIONAL_PARAMETERS_OGF, 0x0009),
    LeSetEventMask = opcode(LE_CONTROLLER_OGF, 0x0001),
    LeReadBufferSize = opcode(LE_CONTROLLER_OGF, 0x0002),
    LeReadLocalSupportedFeaturesPage0 = opcode(LE_CONTROLLER_OGF, 0x0003),
    LeSetRandomAddress = opcode(LE_CONTROLLER_OGF, 0x0005),
    LeSetAdvertisingParameters = opcode(LE_CONTROLLER_OGF, 0x0006),
    LeReadAdvertisingChannelTxPower = opcode(LE_CONTROLLER_OGF, 0x0007),
    LeSetAdvertisingData = opcode(LE_CONTROLLER_OGF, 0x0008),
    LeSetScanResponseData = opcode(LE_CONTROLLER_OGF, 0x0009),
    LeSetAdvertisingEnable = opcode(LE_CONTROLLER_OGF, 0x000A),
    // LeReadFilterAcceptListSize = opcode(LE_CONTROLLER_OGF, 0x000F),
    // LeClearFilterAcceptList = opcode(LE_CONTROLLER_OGF, 0x0010),
    // LeAddDeviceToFilterAcceptList = opcode(LE_CONTROLLER_OGF, 0x0011),
    // LeRemoveDeviceFromFilterAcceptList = opcode(LE_CONTROLLER_OGF, 0x0012),
    // LeEncrypt = opcode(LE_CONTROLLER_OGF, 0x0017),
    LeRand = opcode(LE_CONTROLLER_OGF, 0x0018),
    LeReadSupportedStates = opcode(LE_CONTROLLER_OGF, 0x001C),
    #[num_enum(catch_all)]
    Unsupported(u16),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Command {
    // LeAddDeviceToFilterAcceptList(AddressType, Address),
    // LeClearFilterAcceptList,
    // LeEncrypt(Key, Data),
    LeRand,
    LeReadAdvertisingChannelTxPower,
    LeReadBufferSize,
    LeReadLocalSupportedFeaturesPage0,
    LeReadSupportedStates,
    // LeReadFilterAcceptListSize,
    // LeRemoveDeviceFromFilterAcceptList(AddressType, Address),
    LeSetEventMask(LeEventMask),
    LeSetAdvertisingEnable(AdvertisingEnable),
    LeSetAdvertisingData(AdvertisingData),
    LeSetAdvertisingParameters(AdvertisingParameters),
    LeSetRandomAddress(RandomStaticDeviceAddress),
    LeSetScanResponseData(ScanResponseData),
    Nop,
    ReadBdAddr,
    ReadBufferSize,
    ReadLocalSupportedCommands,
    ReadLocalSupportedFeatures,
    Reset,
    SetEventMask(EventMask),
    Unsupported(u16),
}

impl Command {
    pub(crate) fn encode(&self) -> Result<CommandPacket, Error> {
        Ok(match self {
            Command::LeReadAdvertisingChannelTxPower
            | Command::LeReadBufferSize
            | Command::LeReadLocalSupportedFeaturesPage0
            | Command::LeReadSupportedStates
            | Command::Nop
            | Command::LeRand
            | Command::ReadBdAddr
            | Command::ReadBufferSize
            | Command::ReadLocalSupportedCommands
            | Command::ReadLocalSupportedFeatures
            | Command::Reset => CommandPacket::new(self.opcode()),
            Command::LeSetAdvertisingEnable(enable) => {
                CommandPacket::new(self.opcode()).encode(enable)?
            }
            Command::LeSetAdvertisingData(data) => {
                CommandPacket::new(self.opcode()).encode(data)?
            }
            Command::LeSetAdvertisingParameters(parameters) => {
                CommandPacket::new(self.opcode()).encode(parameters)?
            }
            Command::LeSetEventMask(le_event_mask) => {
                CommandPacket::new(self.opcode()).encode(le_event_mask)?
            }
            Command::LeSetRandomAddress(random_address) => {
                CommandPacket::new(self.opcode()).encode(random_address)?
            }
            Command::LeSetScanResponseData(data) => {
                CommandPacket::new(self.opcode()).encode(data)?
            }
            Command::SetEventMask(event_mask) => {
                CommandPacket::new(self.opcode()).encode(event_mask)?
            }
            Command::Unsupported(opcode) => return Err(Error::InvalidCommand(*opcode)),
        })
    }

    pub(crate) fn opcode(&self) -> CommandOpCode {
        match self {
            // Self::LeClearFilterAcceptList => CommandOpCode::LeClearFilterAcceptList,
            Self::LeRand => CommandOpCode::LeRand,
            Self::LeReadAdvertisingChannelTxPower => CommandOpCode::LeReadAdvertisingChannelTxPower,
            Self::LeReadBufferSize => CommandOpCode::LeReadBufferSize,
            // Self::LeReadFilterAcceptListSize => CommandOpCode::LeReadFilterAcceptListSize,
            Self::LeReadLocalSupportedFeaturesPage0 => {
                CommandOpCode::LeReadLocalSupportedFeaturesPage0
            }
            Self::LeReadSupportedStates => CommandOpCode::LeReadSupportedStates,
            Self::LeSetAdvertisingEnable(_) => CommandOpCode::LeSetAdvertisingEnable,
            Self::LeSetAdvertisingData(_) => CommandOpCode::LeSetAdvertisingData,
            Self::LeSetAdvertisingParameters(_) => CommandOpCode::LeSetAdvertisingParameters,
            Self::LeSetEventMask(_) => CommandOpCode::LeSetEventMask,
            Self::LeSetRandomAddress(_) => CommandOpCode::LeSetRandomAddress,
            Self::LeSetScanResponseData(_) => CommandOpCode::LeSetScanResponseData,
            Self::Nop => CommandOpCode::Nop,
            Self::ReadBdAddr => CommandOpCode::ReadBdAddr,
            Self::ReadBufferSize => CommandOpCode::ReadBufferSize,
            Self::ReadLocalSupportedCommands => CommandOpCode::ReadLocalSupportedCommands,
            Self::ReadLocalSupportedFeatures => CommandOpCode::ReadLocalSupportedFeatures,
            Self::Reset => CommandOpCode::Reset,
            Self::SetEventMask(_) => CommandOpCode::SetEventMask,
            Self::Unsupported(opcode) => CommandOpCode::Unsupported(*opcode),
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
    fn new(opcode: CommandOpCode) -> Self {
        let mut buffer = Buffer {
            data: [0; HCI_COMMAND_MAX_SIZE],
            offset: HCI_COMMAND_PACKET_OPCODE_OFFSET,
        };
        buffer.data[HCI_COMMAND_PACKET_TYPE_OFFSET] = PacketType::Command as u8;
        // INVARIANT: The buffer space is known to be enough.
        buffer.encode_le_u16(opcode.into()).unwrap();
        buffer.offset = HCI_COMMAND_PACKET_DATA_OFFSET;
        Self { buffer }
    }

    fn encode<E: EncodeToBuffer>(mut self, data: &E) -> Result<Self, Error> {
        self.buffer.data[HCI_COMMAND_PACKET_LENGTH_OFFSET] +=
            data.encode(&mut self.buffer)
                .map_err(|_| Error::DataWillNotFitCommandPacket)? as u8;
        Ok(self)
    }

    pub(crate) fn data(&self) -> &[u8] {
        self.buffer.data()
    }
}

pub(crate) mod parser {
    use nom::{bytes::take, combinator::map, number::le_u16, sequence::pair, IResult, Parser};

    use crate::advertising_data::parser::{advertising_data, scan_response_data};
    use crate::advertising_enable::parser::advertising_enable;
    use crate::advertising_parameters::parser::advertising_parameters;
    use crate::device_address::parser::random_address;
    use crate::event_mask::parser::event_mask;
    use crate::le_event_mask::parser::le_event_mask;
    use crate::packet::parser::parameter_total_length;
    use crate::{Command, CommandOpCode, Packet};

    pub(crate) fn command_opcode(input: &[u8]) -> IResult<&[u8], CommandOpCode> {
        map(le_u16(), CommandOpCode::from).parse(input)
    }

    pub(crate) fn command(input: &[u8]) -> IResult<&[u8], Packet> {
        let (input, (command_opcode, parameter_total_length)) =
            pair(command_opcode, parameter_total_length).parse(input)?;
        let (input, parameters) = take(parameter_total_length).parse(input)?;
        Ok((
            input,
            Packet::Command(match command_opcode {
                // CommandOpCode::LeClearFilterAcceptList => Command::LeClearFilterAcceptList,
                CommandOpCode::LeRand => Command::LeRand,
                CommandOpCode::LeReadAdvertisingChannelTxPower => {
                    Command::LeReadAdvertisingChannelTxPower
                }
                CommandOpCode::LeReadBufferSize => Command::LeReadBufferSize,
                // CommandOpCode::LeReadFilterAcceptListSize => {
                //     Command::LeReadFilterAcceptListSize
                // }
                CommandOpCode::LeReadLocalSupportedFeaturesPage0 => {
                    Command::LeReadLocalSupportedFeaturesPage0
                }
                CommandOpCode::LeReadSupportedStates => Command::LeReadSupportedStates,
                CommandOpCode::LeSetAdvertisingEnable => {
                    let (_, advertising_enable) = advertising_enable(parameters)?;
                    Command::LeSetAdvertisingEnable(advertising_enable)
                }
                CommandOpCode::LeSetAdvertisingData => {
                    let (_, advertising_data) = advertising_data(parameters)?;
                    Command::LeSetAdvertisingData(advertising_data)
                }
                CommandOpCode::LeSetAdvertisingParameters => {
                    let (_, advertising_parameters) = advertising_parameters(parameters)?;
                    Command::LeSetAdvertisingParameters(advertising_parameters)
                }
                CommandOpCode::LeSetEventMask => {
                    let (_, le_event_mask) = le_event_mask(parameters)?;
                    Command::LeSetEventMask(le_event_mask)
                }
                CommandOpCode::LeSetRandomAddress => {
                    let (_, random_address) = random_address(parameters)?;
                    Command::LeSetRandomAddress(random_address)
                }
                CommandOpCode::LeSetScanResponseData => {
                    let (_, scan_response_data) = scan_response_data(parameters)?;
                    Command::LeSetScanResponseData(scan_response_data)
                }
                CommandOpCode::Nop => Command::Nop,
                CommandOpCode::ReadBdAddr => Command::ReadBdAddr,
                CommandOpCode::ReadBufferSize => Command::ReadBufferSize,
                CommandOpCode::ReadLocalSupportedCommands => Command::ReadLocalSupportedCommands,
                CommandOpCode::ReadLocalSupportedFeatures => Command::ReadLocalSupportedFeatures,
                CommandOpCode::Reset => Command::Reset,
                CommandOpCode::SetEventMask => {
                    let (_, event_mask) = event_mask(parameters)?;
                    Command::SetEventMask(event_mask)
                }
                CommandOpCode::Unsupported(opcode) => Command::Unsupported(opcode),
            }),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(NOP_OGF, 0, 0)]
    #[case(CONTROLLER_AND_BASEBAND_OGF, 3, 0x0C03)]
    #[case(INFORMATIONAL_PARAMETERS_OGF, 5, 0x1005)]
    #[case(LE_CONTROLLER_OGF, 10, 0x200A)]
    fn test_opcode(#[case] ogf: u16, #[case] ocf: u16, #[case] expected_opcode: u16) {
        assert_eq!(opcode(ogf, ocf), expected_opcode);
    }

    #[rstest]
    #[case(0x0000, CommandOpCode::Nop)]
    #[case(0x200A, CommandOpCode::LeSetAdvertisingEnable)]
    #[case(0x0C08, CommandOpCode::Unsupported(0x0C08))]
    fn test_hci_command_opcode_from(#[case] input: u16, #[case] expected: CommandOpCode) {
        let opcode: CommandOpCode = input.into();
        assert_eq!(opcode, expected);
        let value: u16 = opcode.into();
        assert_eq!(value, input);
    }

    #[test]
    fn test_command_packet_default() {
        let packet = CommandPacket::new(CommandOpCode::Nop);
        assert_eq!(packet.data(), &[1, 0, 0, 0]);
    }

    #[rstest]
    #[case::nop(Command::Nop, CommandOpCode::Nop, &[1, 0, 0, 0])]
    #[case::le_rand(Command::LeRand, CommandOpCode::LeRand, &[1, 24, 32, 0])]
    #[case::le_read_advertising_channel_tx_power(
        Command::LeReadAdvertisingChannelTxPower, CommandOpCode::LeReadAdvertisingChannelTxPower, &[1, 7, 32, 0]
    )]
    #[case::le_read_buffer_size(Command::LeReadBufferSize, CommandOpCode::LeReadBufferSize, &[1, 2, 32, 0])]
    #[case::le_read_local_supported_features_page_0(
        Command::LeReadLocalSupportedFeaturesPage0, CommandOpCode::LeReadLocalSupportedFeaturesPage0, &[1, 3, 32, 0]
    )]
    #[case::le_read_supported_states(Command::LeReadSupportedStates, CommandOpCode::LeReadSupportedStates, &[1, 28, 32, 0])]
    #[case::le_set_advertising_enable(
        Command::LeSetAdvertisingEnable(AdvertisingEnable::Enabled), CommandOpCode::LeSetAdvertisingEnable, &[1, 10, 32, 1, 1]
    )]
    #[case::le_set_advertising_data(
        Command::LeSetAdvertisingData(AdvertisingData::default()),
        CommandOpCode::LeSetAdvertisingData,
        &[1, 8, 32, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    )]
    #[case::le_set_advertising_parameters(
        Command::LeSetAdvertisingParameters(AdvertisingParameters::default()),
        CommandOpCode::LeSetAdvertisingParameters,
        &[1, 6, 32, 15, 0, 8, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0]
    )]
    #[case::le_set_event_mask(
        Command::LeSetEventMask(LeEventMask::default()), CommandOpCode::LeSetEventMask, &[1, 1, 32, 8, 31, 0, 0, 0, 0, 0, 0, 0]
    )]
    #[case::le_set_random_address(
        Command::LeSetRandomAddress([68, 223, 27, 9, 83, 250].try_into().unwrap()),
        CommandOpCode::LeSetRandomAddress,
        &[1, 5, 32, 6, 68, 223, 27, 9, 83, 250]
    )]
    #[case::le_set_scan_response_data(
        Command::LeSetScanResponseData(ScanResponseData::default()),
        CommandOpCode::LeSetScanResponseData,
        &[1, 9, 32, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    )]
    #[case::read_bd_addr(Command::ReadBdAddr, CommandOpCode::ReadBdAddr, &[1, 9, 16, 0])]
    #[case::read_buffer_size(Command::ReadBufferSize, CommandOpCode::ReadBufferSize, &[1, 5, 16, 0])]
    #[case::read_local_supported_commands(
        Command::ReadLocalSupportedCommands, CommandOpCode::ReadLocalSupportedCommands, &[1, 2, 16, 0]
    )]
    #[case::read_local_supported_features(
        Command::ReadLocalSupportedFeatures, CommandOpCode::ReadLocalSupportedFeatures, &[1, 3, 16, 0]
    )]
    #[case::reset(Command::Reset, CommandOpCode::Reset, &[1, 3, 12, 0])]
    #[case::set_event_mask(
        Command::SetEventMask(EventMask::HARDWARE_ERROR | EventMask::DATA_BUFFER_OVERFLOW | EventMask::DISCONNECTION_COMPLETE),
        CommandOpCode::SetEventMask,
        &[1, 1, 12, 8, 16, 128, 0, 2, 0, 0, 0, 0]
    )]
    fn test_command_encode(
        #[case] command: Command,
        #[case] expected_opcode: CommandOpCode,
        #[case] expected_data: &[u8],
    ) -> Result<(), Error> {
        let packet = command.encode()?;
        assert_eq!(packet.data(), expected_data);
        assert_eq!(command.opcode(), expected_opcode);
        Ok(())
    }

    #[test]
    fn test_encode_unsupported_command() {
        // Use Flush command
        let command = Command::Unsupported(0x0C08);
        let err = command.encode();
        assert!(matches!(err, Err(Error::InvalidCommand(0x0C08))));
        assert_eq!(command.opcode(), CommandOpCode::Unsupported(0x0C08));
    }

    #[test]
    fn test_encode_failure() {
        struct Object;
        impl EncodeToBuffer for Object {
            fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, bletio_utils::Error> {
                let data = [0u8; 1024];
                buffer.copy_from_slice(&data)
            }

            fn encoded_size(&self) -> usize {
                1024
            }
        }
        let object = Object;
        assert!(matches!(
            CommandPacket::new(CommandOpCode::Nop).encode(&object),
            Err(Error::DataWillNotFitCommandPacket)
        ));
    }
}
