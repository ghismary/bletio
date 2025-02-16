use core::num::{NonZeroU16, NonZeroU8};

use crate::{
    CommandOpCode, ErrorCode, PublicDeviceAddress, SupportedCommands, SupportedFeatures,
    SupportedLeFeatures, SupportedLeStates, TxPowerLevel,
};

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) enum Event {
    CommandComplete(CommandCompleteEvent),
    Unsupported(u8),
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub(crate) enum EventCode {
    CommandComplete = 0x0E,
    Unsupported(u8),
}

impl From<u8> for EventCode {
    fn from(value: u8) -> Self {
        match value {
            0x0E => Self::CommandComplete,
            _ => Self::Unsupported(value),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct CommandCompleteEvent {
    pub(crate) num_hci_command_packets: u8,
    pub(crate) opcode: CommandOpCode,
    pub(crate) parameter: EventParameter,
}

impl CommandCompleteEvent {
    pub(crate) fn new(
        num_hci_command_packets: u8,
        opcode: CommandOpCode,
        parameter: impl Into<EventParameter>,
    ) -> Self {
        Self {
            num_hci_command_packets,
            opcode,
            parameter: parameter.into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) enum EventParameter {
    Empty,
    Status(StatusEventParameter),
    StatusAndBdAddr(StatusAndBdAddrEventParameter),
    StatusAndBufferSize(StatusAndBufferSizeEventParameter),
    StatusAndLeBufferSize(StatusAndLeBufferSizeEventParameter),
    StatusAndRandomNumber(StatusAndRandomNumberEventParameter),
    StatusAndSupportedCommands(StatusAndSupportedCommandsEventParameter),
    StatusAndSupportedFeatures(StatusAndSupportedFeaturesEventParameter),
    StatusAndSupportedLeFeatures(StatusAndSupportedLeFeaturesEventParameter),
    StatusAndSupportedLeStates(StatusAndSupportedLeStatesEventParameter),
    StatusAndTxPowerLevel(StatusAndTxPowerLevelEventParameter),
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct StatusEventParameter {
    pub(crate) status: ErrorCode,
}

impl From<StatusEventParameter> for EventParameter {
    fn from(value: StatusEventParameter) -> Self {
        Self::Status(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct StatusAndBdAddrEventParameter {
    pub(crate) status: ErrorCode,
    pub(crate) bd_addr: PublicDeviceAddress,
}

impl From<StatusAndBdAddrEventParameter> for EventParameter {
    fn from(value: StatusAndBdAddrEventParameter) -> Self {
        Self::StatusAndBdAddr(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct StatusAndBufferSizeEventParameter {
    pub(crate) status: ErrorCode,
    pub(crate) acl_data_packet_length: NonZeroU16,
    pub(crate) synchronous_data_packet_length: NonZeroU8,
    pub(crate) total_num_acl_data_packets: NonZeroU16,
    pub(crate) total_num_synchronous_packets: u16,
}

impl From<StatusAndBufferSizeEventParameter> for EventParameter {
    fn from(value: StatusAndBufferSizeEventParameter) -> Self {
        Self::StatusAndBufferSize(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct StatusAndLeBufferSizeEventParameter {
    pub(crate) status: ErrorCode,
    pub(crate) le_acl_data_packet_length: u16,
    pub(crate) total_num_le_acl_data_packets: u8,
}

impl From<StatusAndLeBufferSizeEventParameter> for EventParameter {
    fn from(value: StatusAndLeBufferSizeEventParameter) -> Self {
        Self::StatusAndLeBufferSize(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct StatusAndRandomNumberEventParameter {
    pub(crate) status: ErrorCode,
    pub(crate) random_number: [u8; 8],
}

impl From<StatusAndRandomNumberEventParameter> for EventParameter {
    fn from(value: StatusAndRandomNumberEventParameter) -> Self {
        Self::StatusAndRandomNumber(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct StatusAndSupportedCommandsEventParameter {
    pub(crate) status: ErrorCode,
    pub(crate) supported_commands: SupportedCommands,
}

impl From<StatusAndSupportedCommandsEventParameter> for EventParameter {
    fn from(value: StatusAndSupportedCommandsEventParameter) -> Self {
        Self::StatusAndSupportedCommands(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct StatusAndSupportedFeaturesEventParameter {
    pub(crate) status: ErrorCode,
    pub(crate) supported_features: SupportedFeatures,
}

impl From<StatusAndSupportedFeaturesEventParameter> for EventParameter {
    fn from(value: StatusAndSupportedFeaturesEventParameter) -> Self {
        Self::StatusAndSupportedFeatures(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct StatusAndSupportedLeFeaturesEventParameter {
    pub(crate) status: ErrorCode,
    pub(crate) supported_le_features: SupportedLeFeatures,
}

impl From<StatusAndSupportedLeFeaturesEventParameter> for EventParameter {
    fn from(value: StatusAndSupportedLeFeaturesEventParameter) -> Self {
        Self::StatusAndSupportedLeFeatures(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct StatusAndSupportedLeStatesEventParameter {
    pub(crate) status: ErrorCode,
    pub(crate) supported_le_states: SupportedLeStates,
}

impl From<StatusAndSupportedLeStatesEventParameter> for EventParameter {
    fn from(value: StatusAndSupportedLeStatesEventParameter) -> Self {
        Self::StatusAndSupportedLeStates(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct StatusAndTxPowerLevelEventParameter {
    pub(crate) status: ErrorCode,
    pub(crate) tx_power_level: TxPowerLevel,
}

impl From<StatusAndTxPowerLevelEventParameter> for EventParameter {
    fn from(value: StatusAndTxPowerLevelEventParameter) -> Self {
        Self::StatusAndTxPowerLevel(value)
    }
}

pub(crate) mod parser {
    use core::num::{NonZeroU16, NonZeroU8};

    use bitflags::Flags;
    use nom::{
        bytes::take,
        combinator::{eof, fail, map, map_res},
        number::{le_i8, le_u16, le_u64, le_u8},
        sequence::pair,
        IResult, Parser,
    };

    use crate::{
        command::parser::command_opcode, packet::parser::parameter_total_length,
        CommandCompleteEvent, CommandOpCode, ErrorCode, Event, EventCode, EventParameter, Packet,
        PublicDeviceAddress, StatusAndBdAddrEventParameter, StatusAndBufferSizeEventParameter,
        StatusAndLeBufferSizeEventParameter, StatusAndRandomNumberEventParameter,
        StatusAndSupportedCommandsEventParameter, StatusAndSupportedFeaturesEventParameter,
        StatusAndSupportedLeFeaturesEventParameter, StatusAndSupportedLeStatesEventParameter,
        StatusAndTxPowerLevelEventParameter, StatusEventParameter, SupportedCommands,
        SupportedFeatures, SupportedLeFeatures, SupportedLeStates, TxPowerLevel,
    };

    fn event_code(input: &[u8]) -> IResult<&[u8], EventCode> {
        map_res(le_u8(), EventCode::try_from).parse(input)
    }

    fn num_hci_command_packets(input: &[u8]) -> IResult<&[u8], u8> {
        le_u8().parse(input)
    }

    fn hci_error_code(input: &[u8]) -> IResult<&[u8], ErrorCode> {
        map_res(le_u8(), ErrorCode::try_from).parse(input)
    }

    fn supported_commands(input: &[u8]) -> IResult<&[u8], SupportedCommands> {
        map(map_res(take(64u8), TryInto::try_into), |v: [u8; 64]| {
            v.into()
        })
        .parse(input)
    }

    fn supported_features(input: &[u8]) -> IResult<&[u8], SupportedFeatures> {
        map(le_u64(), SupportedFeatures::from_bits_truncate).parse(input)
    }

    fn bd_addr(input: &[u8]) -> IResult<&[u8], PublicDeviceAddress> {
        map(
            map_res(take(6u8), TryInto::try_into),
            PublicDeviceAddress::new,
        )
        .parse(input)
    }

    fn buffer_size(input: &[u8]) -> IResult<&[u8], (NonZeroU16, NonZeroU8, NonZeroU16, u16)> {
        (
            map_res(le_u16(), TryInto::try_into),
            map_res(le_u8(), TryInto::try_into),
            map_res(le_u16(), TryInto::try_into),
            le_u16(),
        )
            .parse(input)
    }

    fn le_buffer_size(input: &[u8]) -> IResult<&[u8], (u16, u8)> {
        (le_u16(), le_u8()).parse(input)
    }

    fn le_supported_features_page_0(input: &[u8]) -> IResult<&[u8], SupportedLeFeatures> {
        map(take(8u8), Into::into).parse(input)
    }

    fn le_supported_states(input: &[u8]) -> IResult<&[u8], SupportedLeStates> {
        map(le_u64(), Into::into).parse(input)
    }

    fn tx_power_level(input: &[u8]) -> IResult<&[u8], TxPowerLevel> {
        map_res(le_i8(), TryInto::try_into).parse(input)
    }

    fn random_number(input: &[u8]) -> IResult<&[u8], [u8; 8]> {
        map_res(take(8u8), TryInto::try_into).parse(input)
    }

    fn command_complete_event(input: &[u8]) -> IResult<&[u8], CommandCompleteEvent> {
        let (return_parameters, (num_hci_command_packets, command_opcode)) =
            pair(num_hci_command_packets, command_opcode).parse(input)?;
        let event_parameter = match command_opcode {
            CommandOpCode::Nop => {
                eof(return_parameters)?;
                EventParameter::Empty
            }
            CommandOpCode::SetEventMask
            | CommandOpCode::Reset
            | CommandOpCode::LeSetAdvertisingEnable
            | CommandOpCode::LeSetAdvertisingData
            | CommandOpCode::LeSetAdvertisingParameters
            | CommandOpCode::LeSetEventMask
            | CommandOpCode::LeSetRandomAddress
            | CommandOpCode::LeSetScanEnable
            | CommandOpCode::LeSetScanParameters
            | CommandOpCode::LeSetScanResponseData => {
                let (rest, status) = hci_error_code(return_parameters)?;
                eof(rest)?;
                StatusEventParameter { status }.into()
            }
            CommandOpCode::LeRand => {
                let (rest, status) = hci_error_code(return_parameters)?;
                let (rest, random_number) = if status.is_success() {
                    random_number(rest)?
                } else {
                    (rest, [0u8; 8])
                };
                eof(rest)?;
                StatusAndRandomNumberEventParameter {
                    status,
                    random_number,
                }
                .into()
            }
            CommandOpCode::LeReadAdvertisingChannelTxPower => {
                let (rest, status) = hci_error_code(return_parameters)?;
                let (rest, tx_power_level) = if status.is_success() {
                    tx_power_level(rest)?
                } else {
                    (rest, TxPowerLevel::default())
                };
                eof(rest)?;
                StatusAndTxPowerLevelEventParameter {
                    status,
                    tx_power_level,
                }
                .into()
            }
            CommandOpCode::LeReadBufferSize => {
                let (rest, status) = hci_error_code(return_parameters)?;
                let (rest, (le_acl_data_packet_length, total_num_le_acl_data_packets)) =
                    if status.is_success() {
                        le_buffer_size(rest)?
                    } else {
                        (rest, (0, 0))
                    };
                eof(rest)?;
                StatusAndLeBufferSizeEventParameter {
                    status,
                    le_acl_data_packet_length,
                    total_num_le_acl_data_packets,
                }
                .into()
            }
            CommandOpCode::LeReadLocalSupportedFeaturesPage0 => {
                let (rest, status) = hci_error_code(return_parameters)?;
                let (rest, supported_le_features) = if status.is_success() {
                    le_supported_features_page_0(rest)?
                } else {
                    (rest, SupportedLeFeatures::empty())
                };
                eof(rest)?;
                StatusAndSupportedLeFeaturesEventParameter {
                    status,
                    supported_le_features,
                }
                .into()
            }
            CommandOpCode::LeReadSupportedStates => {
                let (rest, status) = hci_error_code(return_parameters)?;
                let (rest, supported_le_states) = if status.is_success() {
                    le_supported_states(rest)?
                } else {
                    (rest, SupportedLeStates::default())
                };
                eof(rest)?;
                StatusAndSupportedLeStatesEventParameter {
                    status,
                    supported_le_states,
                }
                .into()
            }
            CommandOpCode::ReadLocalSupportedCommands => {
                let (rest, status) = hci_error_code(return_parameters)?;
                let (rest, supported_commands) = if status.is_success() {
                    supported_commands(rest)?
                } else {
                    (rest, SupportedCommands::empty())
                };
                eof(rest)?;
                StatusAndSupportedCommandsEventParameter {
                    status,
                    supported_commands,
                }
                .into()
            }
            CommandOpCode::ReadLocalSupportedFeatures => {
                let (rest, status) = hci_error_code(return_parameters)?;
                let (rest, supported_features) = if status.is_success() {
                    supported_features(rest)?
                } else {
                    (rest, SupportedFeatures::empty())
                };
                eof(rest)?;
                StatusAndSupportedFeaturesEventParameter {
                    status,
                    supported_features,
                }
                .into()
            }
            CommandOpCode::ReadBdAddr => {
                let (rest, status) = hci_error_code(return_parameters)?;
                let (rest, bd_addr) = if status.is_success() {
                    bd_addr(rest)?
                } else {
                    (rest, PublicDeviceAddress::default())
                };
                eof(rest)?;
                StatusAndBdAddrEventParameter { status, bd_addr }.into()
            }
            CommandOpCode::ReadBufferSize => {
                let (rest, status) = hci_error_code(return_parameters)?;
                let (
                    rest,
                    (
                        acl_data_packet_length,
                        synchronous_data_packet_length,
                        total_num_acl_data_packets,
                        total_num_synchronous_packets,
                    ),
                ) = if status.is_success() {
                    buffer_size(rest)?
                } else {
                    (rest, (NonZeroU16::MIN, NonZeroU8::MIN, NonZeroU16::MIN, 0))
                };
                eof(rest)?;
                StatusAndBufferSizeEventParameter {
                    status,
                    acl_data_packet_length,
                    synchronous_data_packet_length,
                    total_num_acl_data_packets,
                    total_num_synchronous_packets,
                }
                .into()
            }
            CommandOpCode::Unsupported(_) => {
                fail::<_, &[u8], _>().parse(return_parameters)?;
                unreachable!("the fail parser will systematically return an error")
            }
        };
        Ok((
            &[],
            CommandCompleteEvent::new(num_hci_command_packets, command_opcode, event_parameter),
        ))
    }

    pub(crate) fn event(input: &[u8]) -> IResult<&[u8], Packet> {
        let (input, (event_code, parameter_total_length)) =
            pair(event_code, parameter_total_length).parse(input)?;
        let (input, parameters) = take(parameter_total_length).parse(input)?;
        Ok((
            input,
            Packet::Event(match event_code {
                EventCode::CommandComplete => {
                    let (_, event) = command_complete_event(parameters)?;
                    Event::CommandComplete(event)
                }
                EventCode::Unsupported(event_code) => Event::Unsupported(event_code),
            }),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_event_code() {
        let event_code: EventCode = 0x0Eu8.into();
        assert!(matches!(event_code, EventCode::CommandComplete));

        let event_code: EventCode = 0xFFu8.into();
        assert!(matches!(event_code, EventCode::Unsupported(0xFF)));
    }
}
