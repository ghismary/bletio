use crate::advertising::advertising_parameters::AdvertisingParameters;
use crate::advertising::{AdvertisingData, AdvertisingEnable, ScanResponseData};
use crate::hci::event_mask::EventMask;
use crate::hci::opcode::{
    OcfControllerAndBaseband, OcfInformationalParameters, OcfLeController, OpCode,
};
use crate::hci::HciError;
use crate::hci::PacketType;
use crate::utils::Buffer;
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
                CommandPacket::new(self.opcode()).append(buffer.as_slice())?
            }
            Command::LeSetAdvertisingData(data) => {
                CommandPacket::new(self.opcode()).append(data.encoded_data())?
            }
            Command::LeSetAdvertisingParameters(parameters) => {
                CommandPacket::new(self.opcode()).append(parameters.encoded_data())?
            }
            Command::LeSetScanResponseData(data) => {
                CommandPacket::new(self.opcode()).append(data.encoded_data())?
            }
            Command::SetEventMask(event_mask) => CommandPacket::new(self.opcode()).append(
                &event_mask
                    .encode()
                    .map_err(|_| HciError::DataWillNotFitCommandPacket)?,
            )?,
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
    fn new(opcode: OpCode) -> Self {
        let mut buffer = Buffer {
            data: [0; HCI_COMMAND_MAX_SIZE],
            offset: HCI_COMMAND_PACKET_OPCODE_OFFSET,
        };
        buffer.data[HCI_COMMAND_PACKET_TYPE_OFFSET] = PacketType::Command as u8;
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
