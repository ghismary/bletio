use core::num::{NonZeroU16, NonZeroU8};

use crate::{
    hci::{HciCommandOpCode, HciErrorCode, SupportedCommands, SupportedFeatures},
    SupportedLeFeatures, SupportedLeStates,
};

#[derive(Debug)]
pub(crate) enum Event {
    CommandComplete(CommandCompleteEvent),
    Unsupported(u8),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub(crate) struct CommandCompleteEvent {
    pub(crate) num_hci_command_packets: u8,
    pub(crate) opcode: HciCommandOpCode,
    pub(crate) parameter: EventParameter,
}

impl CommandCompleteEvent {
    pub(crate) fn new(
        num_hci_command_packets: u8,
        opcode: HciCommandOpCode,
        parameter: EventParameter,
    ) -> Self {
        Self {
            num_hci_command_packets,
            opcode,
            parameter,
        }
    }
}

#[derive(Debug)]
pub(crate) enum EventParameter {
    Empty,
    Status(StatusEventParameter),
    StatusAndSupportedCommands(StatusAndSupportedCommandsEventParameter),
    StatusAndSupportedFeatures(StatusAndSupportedFeaturesEventParameter),
    StatusAndBufferSize(StatusAndBufferSizeEventParameter),
    StatusAndLeBufferSize(StatusAndLeBufferSizeEventParameter),
    StatusAndSupportedLeFeatures(StatusAndSupportedLeFeaturesEventParameter),
    StatusAndSupportedLeStates(StatusAndSupportedLeStatesEventParameter),
}

#[derive(Debug)]
pub(crate) struct StatusEventParameter {
    pub(crate) status: HciErrorCode,
}

#[derive(Debug)]
pub(crate) struct StatusAndSupportedCommandsEventParameter {
    pub(crate) status: HciErrorCode,
    pub(crate) supported_commands: SupportedCommands,
}

#[derive(Debug)]
pub(crate) struct StatusAndSupportedFeaturesEventParameter {
    pub(crate) status: HciErrorCode,
    pub(crate) supported_features: SupportedFeatures,
}

#[derive(Debug)]
pub(crate) struct StatusAndBufferSizeEventParameter {
    pub(crate) status: HciErrorCode,
    pub(crate) acl_data_packet_length: NonZeroU16,
    pub(crate) synchronous_data_packet_length: NonZeroU8,
    pub(crate) total_num_acl_data_packets: NonZeroU16,
    pub(crate) total_num_synchronous_packets: u16,
}

#[derive(Debug)]
pub(crate) struct StatusAndLeBufferSizeEventParameter {
    pub(crate) status: HciErrorCode,
    pub(crate) le_acl_data_packet_length: u16,
    pub(crate) total_num_le_acl_data_packets: u8,
}

#[derive(Debug)]
pub(crate) struct StatusAndSupportedLeFeaturesEventParameter {
    pub(crate) status: HciErrorCode,
    pub(crate) supported_le_features: SupportedLeFeatures,
}

#[derive(Debug)]
pub(crate) struct StatusAndSupportedLeStatesEventParameter {
    pub(crate) status: HciErrorCode,
    pub(crate) supported_le_states: SupportedLeStates,
}

pub(crate) mod parser {
    use core::num::{NonZeroU16, NonZeroU8};

    use nom::{
        bytes::take,
        combinator::{eof, fail, map, map_res},
        number::{le_u16, le_u64, le_u8},
        sequence::pair,
        IResult, Parser,
    };

    use crate::{
        hci::{
            command::parser::command_opcode, packet::parser::parameter_total_length,
            supported_commands::SupportedCommands, supported_le_states::SupportedLeStates,
            CommandCompleteEvent, Event, EventCode, EventParameter, HciCommandOpCode, HciErrorCode,
            HciPacket, StatusAndBufferSizeEventParameter, StatusAndLeBufferSizeEventParameter,
            StatusAndSupportedCommandsEventParameter, StatusAndSupportedFeaturesEventParameter,
            StatusAndSupportedLeFeaturesEventParameter, StatusAndSupportedLeStatesEventParameter,
            StatusEventParameter, SupportedFeatures,
        },
        SupportedLeFeatures,
    };

    fn event_code(input: &[u8]) -> IResult<&[u8], EventCode> {
        map_res(le_u8(), EventCode::try_from).parse(input)
    }

    fn num_hci_command_packets(input: &[u8]) -> IResult<&[u8], u8> {
        le_u8().parse(input)
    }

    fn hci_error_code(input: &[u8]) -> IResult<&[u8], HciErrorCode> {
        map_res(le_u8(), HciErrorCode::try_from).parse(input)
    }

    fn supported_commands(input: &[u8]) -> IResult<&[u8], SupportedCommands> {
        map(map_res(take(64u8), TryFrom::try_from), |v: [u8; 64]| {
            v.into()
        })
        .parse(input)
    }

    fn supported_features(input: &[u8]) -> IResult<&[u8], SupportedFeatures> {
        map(le_u64(), SupportedFeatures::from_bits_retain).parse(input)
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
        map(le_u64(), SupportedLeFeatures::from_bits_retain).parse(input)
    }

    fn le_supported_states(input: &[u8]) -> IResult<&[u8], SupportedLeStates> {
        map(le_u64(), Into::into).parse(input)
    }

    fn command_complete_event(input: &[u8]) -> IResult<&[u8], CommandCompleteEvent> {
        let (return_parameters, (num_hci_command_packets, command_opcode)) =
            pair(num_hci_command_packets, command_opcode).parse(input)?;
        let event_parameter = match command_opcode {
            HciCommandOpCode::Nop => {
                eof(return_parameters)?;
                EventParameter::Empty
            }
            HciCommandOpCode::SetEventMask
            | HciCommandOpCode::Reset
            | HciCommandOpCode::LeSetAdvertisingEnable
            | HciCommandOpCode::LeSetAdvertisingData
            | HciCommandOpCode::LeSetAdvertisingParameters
            | HciCommandOpCode::LeSetScanResponseData => {
                let (rest, error_code) = hci_error_code(return_parameters)?;
                eof(rest)?;
                EventParameter::Status(StatusEventParameter { status: error_code })
            }
            HciCommandOpCode::ReadLocalSupportedCommands => {
                let (rest, error_code) = hci_error_code(return_parameters)?;
                let (rest, supported_commands) = supported_commands(rest)?;
                eof(rest)?;
                EventParameter::StatusAndSupportedCommands(
                    StatusAndSupportedCommandsEventParameter {
                        status: error_code,
                        supported_commands,
                    },
                )
            }
            HciCommandOpCode::ReadLocalSupportedFeatures => {
                let (rest, status) = hci_error_code(return_parameters)?;
                let (rest, supported_features) = supported_features(rest)?;
                eof(rest)?;
                EventParameter::StatusAndSupportedFeatures(
                    StatusAndSupportedFeaturesEventParameter {
                        status,
                        supported_features,
                    },
                )
            }
            HciCommandOpCode::ReadBufferSize => {
                let (rest, status) = hci_error_code(return_parameters)?;
                let (
                    rest,
                    (
                        acl_data_packet_length,
                        synchronous_data_packet_length,
                        total_num_acl_data_packets,
                        total_num_synchronous_packets,
                    ),
                ) = buffer_size(rest)?;
                eof(rest)?;
                EventParameter::StatusAndBufferSize(StatusAndBufferSizeEventParameter {
                    status,
                    acl_data_packet_length,
                    synchronous_data_packet_length,
                    total_num_acl_data_packets,
                    total_num_synchronous_packets,
                })
            }
            HciCommandOpCode::LeReadBufferSize => {
                let (rest, status) = hci_error_code(return_parameters)?;
                let (rest, (le_acl_data_packet_length, total_num_le_acl_data_packets)) =
                    le_buffer_size(rest)?;
                eof(rest)?;
                EventParameter::StatusAndLeBufferSize(StatusAndLeBufferSizeEventParameter {
                    status,
                    le_acl_data_packet_length,
                    total_num_le_acl_data_packets,
                })
            }
            HciCommandOpCode::LeReadLocalSupportedFeaturesPage0 => {
                let (rest, status) = hci_error_code(return_parameters)?;
                let (rest, supported_le_features) = le_supported_features_page_0(rest)?;
                eof(rest)?;
                EventParameter::StatusAndSupportedLeFeatures(
                    StatusAndSupportedLeFeaturesEventParameter {
                        status,
                        supported_le_features,
                    },
                )
            }
            HciCommandOpCode::LeReadSupportedStates => {
                let (rest, status) = hci_error_code(return_parameters)?;
                let (rest, supported_le_states) = le_supported_states(rest)?;
                eof(rest)?;
                EventParameter::StatusAndSupportedLeStates(
                    StatusAndSupportedLeStatesEventParameter {
                        status,
                        supported_le_states,
                    },
                )
            }
            HciCommandOpCode::Unsupported(_) => {
                fail::<_, &[u8], _>().parse(return_parameters)?;
                unreachable!("the fail parser will systematically return an error")
            }
        };
        Ok((
            &[],
            CommandCompleteEvent::new(num_hci_command_packets, command_opcode, event_parameter),
        ))
    }

    pub(crate) fn event(input: &[u8]) -> IResult<&[u8], HciPacket> {
        let (input, (event_code, parameter_total_length)) =
            pair(event_code, parameter_total_length).parse(input)?;
        let (input, parameters) = take(parameter_total_length).parse(input)?;
        Ok((
            input,
            HciPacket::Event(match event_code {
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
