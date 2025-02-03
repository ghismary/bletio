use crate::advertising::advertising_parameters::AdvertisingParameters;
use crate::advertising::{AdvertisingData, AdvertisingEnable, ScanResponseData};
use crate::hci::event_mask::EventMask;
use crate::hci::HciError;
use crate::hci::HciPacketType;
use crate::utils::{Buffer, BufferOps, EncodeToBuffer};
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

#[derive(Debug, PartialEq, Eq)]
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
    LeSetAdvertisingParameters(AdvertisingParameters),
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
                CommandPacket::new(self.opcode()).encode(enable)?
            }
            HciCommand::LeSetAdvertisingData(data) => {
                CommandPacket::new(self.opcode()).append(data.encoded_data())?
            }
            HciCommand::LeSetAdvertisingParameters(parameters) => {
                CommandPacket::new(self.opcode()).encode(parameters)?
            }
            HciCommand::LeSetScanResponseData(data) => {
                CommandPacket::new(self.opcode()).append(data.encoded_data())?
            }
            HciCommand::SetEventMask(event_mask) => {
                CommandPacket::new(self.opcode()).encode(event_mask)?
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

    fn encode<E: EncodeToBuffer>(mut self, data: &E) -> Result<Self, HciError> {
        self.buffer.data[HCI_COMMAND_PACKET_LENGTH_OFFSET] +=
            data.encode(&mut self.buffer)
                .map_err(|_| HciError::DataWillNotFitCommandPacket)? as u8;
        Ok(self)
    }

    // TODO: TO REMOVE!!
    fn append(self, data: &[u8]) -> Result<Self, HciError> {
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

    use crate::advertising::advertising_parameters::parser::advertising_parameters;
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
                // HciCommandOpCode::LeClearFilterAcceptList => HciCommand::LeClearFilterAcceptList,
                // HciCommandOpCode::LeRand => HciCommand::LeRand,
                HciCommandOpCode::LeReadBufferSize => HciCommand::LeReadBufferSize,
                // HciCommandOpCode::LeReadFilterAcceptListSize => {
                //     HciCommand::LeReadFilterAcceptListSize
                // }
                HciCommandOpCode::LeReadLocalSupportedFeaturesPage0 => {
                    HciCommand::LeReadLocalSupportedFeaturesPage0
                }
                HciCommandOpCode::LeReadSupportedStates => HciCommand::LeReadSupportedStates,
                HciCommandOpCode::LeSetAdvertisingEnable => {
                    let (_, advertising_enable) = advertising_enable(parameters)?;
                    HciCommand::LeSetAdvertisingEnable(advertising_enable)
                }
                HciCommandOpCode::LeSetAdvertisingParameters => {
                    let (_, advertising_parameters) = advertising_parameters(parameters)?;
                    HciCommand::LeSetAdvertisingParameters(advertising_parameters)
                }
                HciCommandOpCode::Nop => HciCommand::Nop,
                // HciCommandOpCode::ReadBdAddr => HciCommand::ReadBdAddr,
                HciCommandOpCode::ReadBufferSize => HciCommand::ReadBufferSize,
                HciCommandOpCode::ReadLocalSupportedCommands => {
                    HciCommand::ReadLocalSupportedCommands
                }
                HciCommandOpCode::ReadLocalSupportedFeatures => {
                    HciCommand::ReadLocalSupportedFeatures
                }
                HciCommandOpCode::Reset => HciCommand::Reset,
                HciCommandOpCode::SetEventMask => {
                    let (_, event_mask) = event_mask(parameters)?;
                    HciCommand::SetEventMask(event_mask)
                }
                HciCommandOpCode::LeSetAdvertisingData
                | HciCommandOpCode::LeSetScanResponseData => {
                    todo!()
                }
                HciCommandOpCode::Unsupported(opcode) => HciCommand::Unsupported(opcode),
            }),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hci_command_opcode_from() {
        let value = 0x0000u16;
        let opcode: HciCommandOpCode = value.into();
        assert_eq!(opcode, HciCommandOpCode::Nop);
        assert_eq!(opcode.value(), value);

        let value = 0x200Au16;
        let opcode: HciCommandOpCode = value.into();
        assert_eq!(opcode, HciCommandOpCode::LeSetAdvertisingEnable);
        assert_eq!(opcode.value(), value);

        // Use Flush command
        let value = 0x0C08u16;
        let opcode: HciCommandOpCode = value.into();
        assert_eq!(opcode, HciCommandOpCode::Unsupported(0x0C08));
        assert_eq!(opcode.value(), value);
    }

    #[test]
    fn test_command_packet_default() {
        let packet = CommandPacket::new(HciCommandOpCode::Nop);
        assert_eq!(packet.data(), &[1, 0, 0, 0]);
    }

    #[test]
    fn test_command_packet_with_append_success() -> Result<(), Error> {
        let mut packet = CommandPacket::new(HciCommandOpCode::LeSetAdvertisingEnable);
        packet = packet.append(&[1])?;
        assert_eq!(packet.data(), &[1, 10, 32, 1, 1]);
        Ok(())
    }

    #[test]
    fn test_command_packet_with_append_failure() {
        let packet = CommandPacket::new(HciCommandOpCode::ReadBufferSize);
        let data = [0u8; 320];
        let err = packet.append(data.as_slice());
        assert!(matches!(err, Err(HciError::DataWillNotFitCommandPacket)));
    }

    #[test]
    fn test_encode_nop_command() -> Result<(), Error> {
        let command = HciCommand::Nop;
        let packet = command.encode()?;
        assert_eq!(packet.data(), &[1, 0, 0, 0]);
        assert_eq!(command.opcode(), HciCommandOpCode::Nop);
        Ok(())
    }

    #[test]
    fn test_encode_le_read_buffer_size_command() -> Result<(), Error> {
        let command = HciCommand::LeReadBufferSize;
        let packet = command.encode()?;
        assert_eq!(packet.data(), &[1, 2, 32, 0]);
        assert_eq!(command.opcode(), HciCommandOpCode::LeReadBufferSize);
        Ok(())
    }

    #[test]
    fn test_encode_le_read_local_supported_features_page_0_command() -> Result<(), Error> {
        let command = HciCommand::LeReadLocalSupportedFeaturesPage0;
        let packet = command.encode()?;
        assert_eq!(packet.data(), &[1, 3, 32, 0]);
        assert_eq!(
            command.opcode(),
            HciCommandOpCode::LeReadLocalSupportedFeaturesPage0
        );
        Ok(())
    }

    #[test]
    fn test_encode_le_read_supported_states_command() -> Result<(), Error> {
        let command = HciCommand::LeReadSupportedStates;
        let packet = command.encode()?;
        assert_eq!(packet.data(), &[1, 28, 32, 0]);
        assert_eq!(command.opcode(), HciCommandOpCode::LeReadSupportedStates);
        Ok(())
    }

    #[test]
    fn test_encode_read_buffer_size_command() -> Result<(), Error> {
        let command = HciCommand::ReadBufferSize;
        let packet = command.encode()?;
        assert_eq!(packet.data(), &[1, 5, 16, 0]);
        assert_eq!(command.opcode(), HciCommandOpCode::ReadBufferSize);
        Ok(())
    }

    #[test]
    fn test_encode_read_local_supported_commands_command() -> Result<(), Error> {
        let command = HciCommand::ReadLocalSupportedCommands;
        let packet = command.encode()?;
        assert_eq!(packet.data(), &[1, 2, 16, 0]);
        assert_eq!(
            command.opcode(),
            HciCommandOpCode::ReadLocalSupportedCommands
        );
        Ok(())
    }

    #[test]
    fn test_encode_read_local_supported_features_command() -> Result<(), Error> {
        let command = HciCommand::ReadLocalSupportedFeatures;
        let packet = command.encode()?;
        assert_eq!(packet.data(), &[1, 3, 16, 0]);
        assert_eq!(
            command.opcode(),
            HciCommandOpCode::ReadLocalSupportedFeatures
        );
        Ok(())
    }

    #[test]
    fn test_encode_reset_command() -> Result<(), Error> {
        let command = HciCommand::Reset;
        let packet = command.encode()?;
        assert_eq!(packet.data(), &[1, 3, 12, 0]);
        assert_eq!(command.opcode(), HciCommandOpCode::Reset);
        Ok(())
    }

    #[test]
    fn test_encode_le_set_advertising_enable_command() -> Result<(), Error> {
        let command = HciCommand::LeSetAdvertisingEnable(AdvertisingEnable::Enabled);
        let packet = command.encode()?;
        assert_eq!(packet.data(), &[1, 10, 32, 1, 1]);
        assert_eq!(command.opcode(), HciCommandOpCode::LeSetAdvertisingEnable);
        Ok(())
    }

    #[test]
    fn test_encode_le_set_advertising_data_command() -> Result<(), Error> {
        let adv_data = AdvertisingData::default();
        let command = HciCommand::LeSetAdvertisingData(&adv_data);
        let packet = command.encode()?;
        assert_eq!(
            packet.data(),
            &[
                1, 8, 32, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
        assert_eq!(command.opcode(), HciCommandOpCode::LeSetAdvertisingData);
        Ok(())
    }

    #[test]
    fn test_encode_le_set_advertising_parameters_command() -> Result<(), Error> {
        let command = HciCommand::LeSetAdvertisingParameters(AdvertisingParameters::default());
        let packet = command.encode()?;
        assert_eq!(
            packet.data(),
            &[1, 6, 32, 15, 0, 8, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0]
        );
        assert_eq!(
            command.opcode(),
            HciCommandOpCode::LeSetAdvertisingParameters
        );
        Ok(())
    }

    #[test]
    fn test_encode_le_set_scan_response_data_command() -> Result<(), Error> {
        let scanresp_data = ScanResponseData::default();
        let command = HciCommand::LeSetScanResponseData(&scanresp_data);
        let packet = command.encode()?;
        assert_eq!(
            packet.data(),
            &[
                1, 9, 32, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
        assert_eq!(command.opcode(), HciCommandOpCode::LeSetScanResponseData);
        Ok(())
    }

    #[test]
    fn test_encode_set_event_mask_command() -> Result<(), Error> {
        let event_mask = EventMask::HARDWARE_ERROR
            | EventMask::DATA_BUFFER_OVERFLOW
            | EventMask::DISCONNECTION_COMPLETE;
        let command = HciCommand::SetEventMask(event_mask);
        let packet = command.encode()?;
        assert_eq!(packet.data(), &[1, 1, 12, 8, 16, 128, 0, 2, 0, 0, 0, 0]);
        assert_eq!(command.opcode(), HciCommandOpCode::SetEventMask);
        Ok(())
    }

    #[test]
    fn test_encode_unsupported_command() {
        // Use Flush command
        let command = HciCommand::Unsupported(0x0C08);
        let err = command.encode();
        assert!(matches!(
            err,
            Err(Error::Hci(HciError::InvalidCommand(0x0C08)))
        ));
        assert_eq!(command.opcode(), HciCommandOpCode::Unsupported(0x0C08));
    }

    #[test]
    fn test_encode_failure() {
        struct Object;
        impl EncodeToBuffer for Object {
            fn encode<B: BufferOps>(
                &self,
                buffer: &mut B,
            ) -> Result<usize, crate::utils::UtilsError> {
                let data = [0u8; 1024];
                buffer.copy_from_slice(&data)
            }
        }
        let object = Object;
        assert!(matches!(
            CommandPacket::new(HciCommandOpCode::Nop).encode(&object),
            Err(HciError::DataWillNotFitCommandPacket)
        ));
    }
}
