use bletio_utils::{Buffer, BufferOps, EncodeToBuffer};
use num_enum::{FromPrimitive, IntoPrimitive};

use crate::{
    AdvertisingData, AdvertisingEnable, AdvertisingParameters, ConnectionHandle,
    ConnectionParameters, ConnectionUpdateParameters, Error, EventMask, FilterDuplicates,
    LeEventMask, LeFilterAcceptListAddress, PacketType, RandomStaticDeviceAddress, Reason,
    ScanEnable, ScanParameters,
};

const NOP_OGF: u16 = 0x00;
const LINK_CONTROL_OGF: u16 = 0x01;
const CONTROLLER_AND_BASEBAND_OGF: u16 = 0x03;
const INFORMATIONAL_PARAMETERS_OGF: u16 = 0x04;
const LE_CONTROLLER_OGF: u16 = 0x08;

const fn opcode(ogf: u16, ocf: u16) -> u16 {
    (ogf << 10) | ocf
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, FromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u16)]
pub(crate) enum CommandOpCode {
    Nop = opcode(NOP_OGF, 0x0000),
    Disconnect = opcode(LINK_CONTROL_OGF, 0x0006),
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
    LeSetScanParameters = opcode(LE_CONTROLLER_OGF, 0x000B),
    LeSetScanEnable = opcode(LE_CONTROLLER_OGF, 0x000C),
    LeCreateConnection = opcode(LE_CONTROLLER_OGF, 0x000D),
    LeCreateConnectionCancel = opcode(LE_CONTROLLER_OGF, 0x000E),
    LeReadFilterAcceptListSize = opcode(LE_CONTROLLER_OGF, 0x000F),
    LeClearFilterAcceptList = opcode(LE_CONTROLLER_OGF, 0x0010),
    LeAddDeviceToFilterAcceptList = opcode(LE_CONTROLLER_OGF, 0x0011),
    LeRemoveDeviceFromFilterAcceptList = opcode(LE_CONTROLLER_OGF, 0x0012),
    LeConnectionUpdate = opcode(LE_CONTROLLER_OGF, 0x0013),
    // LeEncrypt = opcode(LE_CONTROLLER_OGF, 0x0017),
    LeRand = opcode(LE_CONTROLLER_OGF, 0x0018),
    LeReadSupportedStates = opcode(LE_CONTROLLER_OGF, 0x001C),
    #[num_enum(catch_all)]
    Unsupported(u16),
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) enum Command {
    Disconnect(ConnectionHandle, Reason),
    LeAddDeviceToFilterAcceptList(LeFilterAcceptListAddress),
    LeClearFilterAcceptList,
    LeCreateConnection(ConnectionParameters),
    LeCreateConnectionCancel,
    LeConnectionUpdate(ConnectionUpdateParameters),
    // LeEncrypt(Key, Data),
    LeRand,
    LeReadAdvertisingChannelTxPower,
    LeReadBufferSize,
    LeReadLocalSupportedFeaturesPage0,
    LeReadSupportedStates,
    LeReadFilterAcceptListSize,
    LeRemoveDeviceFromFilterAcceptList(LeFilterAcceptListAddress),
    LeSetEventMask(LeEventMask),
    LeSetAdvertisingEnable(AdvertisingEnable),
    LeSetAdvertisingData(AdvertisingData),
    LeSetAdvertisingParameters(AdvertisingParameters),
    LeSetRandomAddress(RandomStaticDeviceAddress),
    LeSetScanEnable(ScanEnable, FilterDuplicates),
    LeSetScanParameters(ScanParameters),
    LeSetScanResponseData(AdvertisingData),
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
            Command::LeClearFilterAcceptList
            | Command::LeCreateConnectionCancel
            | Command::LeReadAdvertisingChannelTxPower
            | Command::LeReadBufferSize
            | Command::LeReadFilterAcceptListSize
            | Command::LeReadLocalSupportedFeaturesPage0
            | Command::LeReadSupportedStates
            | Command::Nop
            | Command::LeRand
            | Command::ReadBdAddr
            | Command::ReadBufferSize
            | Command::ReadLocalSupportedCommands
            | Command::ReadLocalSupportedFeatures
            | Command::Reset => CommandPacket::new(self.opcode()),
            Command::Disconnect(connection_handle, reason) => CommandPacket::new(self.opcode())
                .encode(connection_handle)?
                .encode(reason)?,
            Command::LeAddDeviceToFilterAcceptList(address)
            | Command::LeRemoveDeviceFromFilterAcceptList(address) => {
                CommandPacket::new(self.opcode()).encode(address)?
            }
            Command::LeConnectionUpdate(parameters) => {
                CommandPacket::new(self.opcode()).encode(parameters)?
            }
            Command::LeCreateConnection(parameters) => {
                CommandPacket::new(self.opcode()).encode(parameters)?
            }
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
            Command::LeSetScanEnable(enable, filter_duplicates) => {
                CommandPacket::new(self.opcode())
                    .encode(enable)?
                    .encode(filter_duplicates)?
            }
            Command::LeSetScanParameters(parameters) => {
                CommandPacket::new(self.opcode()).encode(parameters)?
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

    pub(crate) const fn opcode(&self) -> CommandOpCode {
        match self {
            Self::Disconnect(_, _) => CommandOpCode::Disconnect,
            Self::LeAddDeviceToFilterAcceptList(_) => CommandOpCode::LeAddDeviceToFilterAcceptList,
            Self::LeClearFilterAcceptList => CommandOpCode::LeClearFilterAcceptList,
            Self::LeConnectionUpdate(_) => CommandOpCode::LeConnectionUpdate,
            Self::LeCreateConnection(_) => CommandOpCode::LeCreateConnection,
            Self::LeCreateConnectionCancel => CommandOpCode::LeCreateConnectionCancel,
            Self::LeRand => CommandOpCode::LeRand,
            Self::LeReadAdvertisingChannelTxPower => CommandOpCode::LeReadAdvertisingChannelTxPower,
            Self::LeReadBufferSize => CommandOpCode::LeReadBufferSize,
            Self::LeReadFilterAcceptListSize => CommandOpCode::LeReadFilterAcceptListSize,
            Self::LeReadLocalSupportedFeaturesPage0 => {
                CommandOpCode::LeReadLocalSupportedFeaturesPage0
            }
            Self::LeReadSupportedStates => CommandOpCode::LeReadSupportedStates,
            Self::LeRemoveDeviceFromFilterAcceptList(_) => {
                CommandOpCode::LeRemoveDeviceFromFilterAcceptList
            }
            Self::LeSetAdvertisingEnable(_) => CommandOpCode::LeSetAdvertisingEnable,
            Self::LeSetAdvertisingData(_) => CommandOpCode::LeSetAdvertisingData,
            Self::LeSetAdvertisingParameters(_) => CommandOpCode::LeSetAdvertisingParameters,
            Self::LeSetEventMask(_) => CommandOpCode::LeSetEventMask,
            Self::LeSetRandomAddress(_) => CommandOpCode::LeSetRandomAddress,
            Self::LeSetScanEnable(_, _) => CommandOpCode::LeSetScanEnable,
            Self::LeSetScanParameters(_) => CommandOpCode::LeSetScanParameters,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
    use nom::{
        bytes::take, combinator::map, number::complete::le_u16, sequence::pair, IResult, Parser,
    };

    use crate::advertising::{
        advertising_data::parser::advertising_data, advertising_enable::parser::advertising_enable,
        advertising_parameters::parser::advertising_parameters,
    };
    use crate::common::{
        device_address::parser::random_address, event_mask::parser::event_mask,
        le_event_mask::parser::le_event_mask,
        le_filter_accept_list_address::parser::le_filter_accept_list_address,
    };
    use crate::connection::connection_handle::parser::connection_handle;
    use crate::connection::connection_parameters::parser::connection_parameters;
    use crate::connection::connection_update_parameters::parser::connection_update_parameters;
    use crate::connection::reason::parser::reason;
    use crate::packet::parser::parameter_total_length;
    use crate::scanning::{
        scan_enable::parser::scan_enable_parameters, scan_parameters::parser::scan_parameters,
    };
    use crate::{Command, CommandOpCode, ConnectionHandle, Packet, Reason};

    pub(crate) fn command_opcode(input: &[u8]) -> IResult<&[u8], CommandOpCode> {
        map(le_u16, CommandOpCode::from).parse(input)
    }

    fn disconnect(input: &[u8]) -> IResult<&[u8], (ConnectionHandle, Reason)> {
        (connection_handle, reason).parse(input)
    }

    pub(crate) fn command(input: &[u8]) -> IResult<&[u8], Packet> {
        let (input, (command_opcode, parameter_total_length)) =
            pair(command_opcode, parameter_total_length).parse(input)?;
        let (input, parameters) = take(parameter_total_length).parse(input)?;
        Ok((
            input,
            Packet::Command(match command_opcode {
                CommandOpCode::Disconnect => {
                    let (_, (connection_handle, reason)) = disconnect(parameters)?;
                    Command::Disconnect(connection_handle, reason)
                }
                CommandOpCode::LeAddDeviceToFilterAcceptList => {
                    let (_, le_filter_accept_list_address) =
                        le_filter_accept_list_address(parameters)?;
                    Command::LeAddDeviceToFilterAcceptList(le_filter_accept_list_address)
                }
                CommandOpCode::LeClearFilterAcceptList => Command::LeClearFilterAcceptList,
                CommandOpCode::LeConnectionUpdate => {
                    let (_, connection_update_parameters) =
                        connection_update_parameters(parameters)?;
                    Command::LeConnectionUpdate(connection_update_parameters)
                }
                CommandOpCode::LeCreateConnection => {
                    let (_, connection_parameters) = connection_parameters(parameters)?;
                    Command::LeCreateConnection(connection_parameters)
                }
                CommandOpCode::LeCreateConnectionCancel => Command::LeCreateConnectionCancel,
                CommandOpCode::LeRand => Command::LeRand,
                CommandOpCode::LeReadAdvertisingChannelTxPower => {
                    Command::LeReadAdvertisingChannelTxPower
                }
                CommandOpCode::LeReadBufferSize => Command::LeReadBufferSize,
                CommandOpCode::LeReadFilterAcceptListSize => Command::LeReadFilterAcceptListSize,
                CommandOpCode::LeReadLocalSupportedFeaturesPage0 => {
                    Command::LeReadLocalSupportedFeaturesPage0
                }
                CommandOpCode::LeReadSupportedStates => Command::LeReadSupportedStates,
                CommandOpCode::LeRemoveDeviceFromFilterAcceptList => {
                    let (_, le_filter_accept_list_address) =
                        le_filter_accept_list_address(parameters)?;
                    Command::LeRemoveDeviceFromFilterAcceptList(le_filter_accept_list_address)
                }
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
                CommandOpCode::LeSetScanEnable => {
                    let (_, (scan_enable, filter_duplicates)) = scan_enable_parameters(parameters)?;
                    Command::LeSetScanEnable(scan_enable, filter_duplicates)
                }
                CommandOpCode::LeSetScanParameters => {
                    let (_, scan_parameters) = scan_parameters(parameters)?;
                    Command::LeSetScanParameters(scan_parameters)
                }
                CommandOpCode::LeSetScanResponseData => {
                    let (_, scan_response_data) = advertising_data(parameters)?;
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
    use crate::{
        packet::parser::packet, AdvertisingChannelMap, AdvertisingFilterPolicy,
        AdvertisingIntervalRange, AdvertisingType, DeviceAddress, OwnAddressType, Packet,
        PublicDeviceAddress, RandomAddress,
    };

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
    #[case::disconnect(
        Command::Disconnect(ConnectionHandle::try_new(0).unwrap(), Reason::RemoteUserTerminatedConnection),
        CommandOpCode::Disconnect,
        &[1, 6, 4, 3, 0, 0, 19]
    )]
    #[case::le_add_device_to_filter_accept_list(
        Command::LeAddDeviceToFilterAcceptList(PublicDeviceAddress::from([0x38, 0x5E, 0x43, 0xCA, 0x4C, 0x40]).into()),
        CommandOpCode::LeAddDeviceToFilterAcceptList,
        &[1, 17, 32, 7, 0, 0x38, 0x5E, 0x43, 0xCA, 0x4C, 0x40]
    )]
    #[case::le_clear_filter_accept_list(Command::LeClearFilterAcceptList, CommandOpCode::LeClearFilterAcceptList, &[1, 16, 32, 0])]
    #[case::le_connection_update(
        Command::LeConnectionUpdate(ConnectionUpdateParameters::default()),
        CommandOpCode::LeConnectionUpdate,
        &[1, 19, 32, 14, 0, 0, 64, 0, 64, 0, 0, 0, 32, 0, 0, 0, 0, 0]
    )]
    #[case::le_create_connection(
        Command::LeCreateConnection(ConnectionParameters::default()),
        CommandOpCode::LeCreateConnection,
        &[1, 13, 32, 25, 16, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 64, 0, 0, 0, 32, 0, 0, 0, 0, 0]
    )]
    #[case::le_create_connection_cancel(
        Command::LeCreateConnectionCancel, CommandOpCode::LeCreateConnectionCancel, &[1, 14, 32, 0]
    )]
    #[case::le_rand(Command::LeRand, CommandOpCode::LeRand, &[1, 24, 32, 0])]
    #[case::le_read_advertising_channel_tx_power(
        Command::LeReadAdvertisingChannelTxPower, CommandOpCode::LeReadAdvertisingChannelTxPower, &[1, 7, 32, 0]
    )]
    #[case::le_read_buffer_size(Command::LeReadBufferSize, CommandOpCode::LeReadBufferSize, &[1, 2, 32, 0])]
    #[case::le_read_filter_accept_list_size(Command::LeReadFilterAcceptListSize, CommandOpCode::LeReadFilterAcceptListSize, &[1, 15, 32, 0])]
    #[case::le_read_local_supported_features_page_0(
        Command::LeReadLocalSupportedFeaturesPage0, CommandOpCode::LeReadLocalSupportedFeaturesPage0, &[1, 3, 32, 0]
    )]
    #[case::le_read_supported_states(Command::LeReadSupportedStates, CommandOpCode::LeReadSupportedStates, &[1, 28, 32, 0])]
    #[case::le_remove_device_from_filter_accept_list(
        Command::LeRemoveDeviceFromFilterAcceptList(PublicDeviceAddress::from([0x38, 0x5E, 0x43, 0xCA, 0x4C, 0x40]).into()),
        CommandOpCode::LeRemoveDeviceFromFilterAcceptList,
        &[1, 18, 32, 7, 0, 0x38, 0x5E, 0x43, 0xCA, 0x4C, 0x40]
    )]
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
    #[case::le_set_scan_enable(
        Command::LeSetScanEnable(ScanEnable::Enabled, FilterDuplicates::Disabled), CommandOpCode::LeSetScanEnable, &[1, 12, 32, 2, 1, 0]
    )]
    #[case::le_set_scan_parameters(
        Command::LeSetScanParameters(ScanParameters::default()),
        CommandOpCode::LeSetScanParameters,
        &[1, 11, 32, 7, 0, 16, 0, 16, 0, 0, 0]
    )]
    #[case::le_set_scan_response_data(
        Command::LeSetScanResponseData(AdvertisingData::default()),
        CommandOpCode::LeSetScanResponseData,
        &[1, 9, 32, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    )]
    #[case::nop(Command::Nop, CommandOpCode::Nop, &[1, 0, 0, 0])]
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
    fn test_command_encode_unsupported_command() {
        // Use Flush command
        let command = Command::Unsupported(0x0C08);
        let err = command.encode();
        assert!(matches!(err, Err(Error::InvalidCommand(0x0C08))));
        assert_eq!(command.opcode(), CommandOpCode::Unsupported(0x0C08));
    }

    #[test]
    fn test_command_encode_failure() {
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
        assert_eq!(object.encoded_size(), 1024);
        assert!(matches!(
            CommandPacket::new(CommandOpCode::Nop).encode(&object),
            Err(Error::DataWillNotFitCommandPacket)
        ));
    }

    #[test]
    fn test_unsupported_command_parsing() {
        // Use Flush command
        let (rest, packet) = packet(&[1, 8, 12, 0]).unwrap();
        assert!(matches!(
            packet,
            Packet::Command(Command::Unsupported(0x0C08))
        ));
        assert!(rest.is_empty());
    }

    #[rstest]
    #[case::disconnect(
        Command::Disconnect(ConnectionHandle::try_new(0).unwrap(), Reason::RemoteUserTerminatedConnection),
        &[1, 6, 4, 3, 0, 0, 19]
    )]
    #[case::le_add_device_to_filter_accept_list(
        Command::LeAddDeviceToFilterAcceptList(PublicDeviceAddress::from([0x38, 0x5E, 0x43, 0xCA, 0x4C, 0x40]).into()),
        &[1, 17, 32, 7, 0, 0x38, 0x5E, 0x43, 0xCA, 0x4C, 0x40]
    )]
    #[case::le_clear_filter_accept_list(Command::LeClearFilterAcceptList, &[1, 16, 32, 0])]
    #[case::le_connection_update(
        Command::LeConnectionUpdate(ConnectionUpdateParameters::default()),
        &[1, 19, 32, 14, 0, 0, 64, 0, 64, 0, 0, 0, 32, 0, 0, 0, 0, 0]
    )]
    #[case::le_create_connection(
        Command::LeCreateConnection(ConnectionParameters::default()),
        &[1, 13, 32, 25, 16, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 64, 0, 0, 0, 32, 0, 0, 0, 0, 0]
    )]
    #[case::le_create_connection_cancel(Command::LeCreateConnectionCancel, &[1, 14, 32, 0])]
    #[case::le_rand(Command::LeRand, &[1, 24, 32, 0])]
    #[case::le_read_advertising_channel_tx_power(Command::LeReadAdvertisingChannelTxPower, &[1, 7, 32, 0])]
    #[case::le_read_buffer_size(Command::LeReadBufferSize, &[1, 2, 32, 0])]
    #[case::le_read_filter_accept_list_size(Command::LeReadFilterAcceptListSize, &[1, 15, 32, 0])]
    #[case::le_read_local_supported_features_page_0(Command::LeReadLocalSupportedFeaturesPage0, &[1, 3, 32, 0])]
    #[case::le_read_supported_states(Command::LeReadSupportedStates, &[1, 28, 32, 0])]
    #[case::le_remove_device_from_filter_accept_list(
        Command::LeRemoveDeviceFromFilterAcceptList(PublicDeviceAddress::from([0x38, 0x5E, 0x43, 0xCA, 0x4C, 0x40]).into()),
        &[1, 18, 32, 7, 0, 0x38, 0x5E, 0x43, 0xCA, 0x4C, 0x40]
    )]
    #[case::le_set_advertising_data(
        Command::LeSetAdvertisingData(AdvertisingData::default()),
        &[1, 8, 32, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    )]
    #[case::le_set_advertising_enable(
        Command::LeSetAdvertisingEnable(AdvertisingEnable::Enabled), &[1, 10, 32, 1, 1]
    )]
    #[case::le_set_advertising_parameters::default(
        Command::LeSetAdvertisingParameters(AdvertisingParameters::default()),
        &[1, 6, 32, 15, 0, 8, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0]
    )]
    #[case::le_set_advertising_parameters::random_peer_address(
        Command::LeSetAdvertisingParameters(AdvertisingParameters::try_new(
            AdvertisingIntervalRange::try_new(0x0020.try_into().unwrap(), 0x0030.try_into().unwrap()).unwrap(),
            AdvertisingType::ScannableUndirected,
            OwnAddressType::RandomDeviceAddress,
            DeviceAddress::Random(RandomAddress::Static(RandomStaticDeviceAddress::try_new([0xFE, 0x92, 0x2F, 0x0F, 0x4B, 0xD2]).unwrap())),
            AdvertisingChannelMap::CHANNEL37 | AdvertisingChannelMap::CHANNEL38,
            AdvertisingFilterPolicy::ConnectionAllAndScanFilterAcceptList,
        ).unwrap()),
        &[1, 6, 32, 15, 32, 0, 48, 0, 2, 1, 1, 0xFE, 0x92, 0x2F, 0x0F, 0x4B, 0xD2, 3, 1]
    )]
    #[case::le_set_event_mask(
        Command::LeSetEventMask(LeEventMask::default()),
        &[1, 1, 32, 8, 31, 0, 0, 0, 0, 0, 0, 0]
    )]
    #[case::le_set_random_address(
        Command::LeSetRandomAddress([68, 223, 27, 9, 83, 250].try_into().unwrap()),
        &[1, 5, 32, 6, 68, 223, 27, 9, 83, 250]
    )]
    #[case::le_set_scan_enable(
        Command::LeSetScanEnable(ScanEnable::Enabled, FilterDuplicates::Disabled),
        &[1, 12, 32, 2, 1, 0]
    )]
    #[case::le_set_scan_parameters(
        Command::LeSetScanParameters(ScanParameters::default()),
        &[1, 11, 32, 7, 0, 16, 0, 16, 0, 0, 0]
    )]
    #[case::le_set_scan_response_data(
        Command::LeSetScanResponseData(AdvertisingData::default()),
        &[1, 9, 32, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    )]
    #[case::nop(Command::Nop, &[1, 0, 0, 0])]
    #[case::read_bd_addr(Command::ReadBdAddr, &[1, 9, 16, 0])]
    #[case::read_buffer_size(Command::ReadBufferSize, &[1, 5, 16, 0])]
    #[case::read_local_supported_commands(Command::ReadLocalSupportedCommands, &[1, 2, 16, 0])]
    #[case::read_local_supported_features(Command::ReadLocalSupportedFeatures, &[1, 3, 16, 0])]
    #[case::reset(Command::Reset, &[1, 3, 12, 0])]
    #[case::set_event_mask(
        Command::SetEventMask(EventMask::HARDWARE_ERROR | EventMask::DATA_BUFFER_OVERFLOW),
        &[1, 1, 12, 8, 0, 128, 0, 2, 0, 0, 0, 0]
    )]
    fn test_supported_command_parsing(#[case] command: Command, #[case] input: &[u8]) {
        let (rest, hci_packet) = packet(input).unwrap();
        assert_eq!(hci_packet, Packet::Command(command));
        assert!(rest.is_empty());
    }
}
