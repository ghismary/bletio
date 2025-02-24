use core::num::{NonZeroU16, NonZeroU8};

use crate::{
    CommandOpCode, ErrorCode, PublicDeviceAddress, SupportedCommands, SupportedFeatures,
    SupportedLeFeatures, SupportedLeStates, TxPowerLevel,
};

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CommandCompleteEvent {
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
    use bitflags::Flags;
    use nom::{
        bytes::take,
        combinator::{eof, fail, map, map_res},
        number::{le_i8, le_u16, le_u64, le_u8},
        sequence::pair,
        IResult, Parser,
    };

    use crate::command::parser::command_opcode;

    use super::*;

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

    pub(crate) fn command_complete_event(input: &[u8]) -> IResult<&[u8], CommandCompleteEvent> {
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
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::{packet::parser::packet, Event, Packet};

    use super::*;

    #[rstest]
    #[case::le_rand(CommandCompleteEvent::new(
            1, CommandOpCode::LeRand,
            StatusAndRandomNumberEventParameter {
                status: ErrorCode::Success, random_number: [68, 223, 27, 9, 83, 58, 224, 240]
            }
        ), &[4, 14, 12, 1, 24, 32, 0, 68, 223, 27, 9, 83, 58, 224, 240])]
    #[case::le_read_advertising_channel_tx_power(CommandCompleteEvent::new(
            1, CommandOpCode::LeReadAdvertisingChannelTxPower,
            StatusAndTxPowerLevelEventParameter {
                status: ErrorCode::Success, tx_power_level: TxPowerLevel::try_new(9).unwrap()
            }
        ), &[4, 14, 5, 1, 7, 32, 0, 9])]
    #[case::le_read_buffer_size(CommandCompleteEvent::new(
            1, CommandOpCode::LeReadBufferSize,
            StatusAndLeBufferSizeEventParameter {
                status: ErrorCode::Success, le_acl_data_packet_length: 255, total_num_le_acl_data_packets: 24
            }
        ), &[4, 14, 7, 1, 2, 32, 0, 255, 0, 24])]
    #[case::le_read_local_supported_features_page_0(CommandCompleteEvent::new(
            1, CommandOpCode::LeReadLocalSupportedFeaturesPage0,
            StatusAndSupportedLeFeaturesEventParameter {
                status: ErrorCode::Success,
                supported_le_features: SupportedLeFeatures::LE_ENCRYPTION | SupportedLeFeatures::LE_EXTENDED_ADVERTISING
            }
        ), &[4, 14, 12, 1, 3, 32, 0, 1, 16, 0, 0, 0, 0, 0, 0])]
    #[case::le_read_supported_states(CommandCompleteEvent::new(
            1, CommandOpCode::LeReadSupportedStates,
            StatusAndSupportedLeStatesEventParameter {
                status: ErrorCode::Success, supported_le_states: 0x0000_03FF_FFFF_FFFF.into()
            }
        ), &[4, 14, 12, 1, 28, 32, 0, 255, 255, 255, 255, 255, 3, 0, 0])]
    #[case::le_set_advertising_data(CommandCompleteEvent::new(
            1, CommandOpCode::LeSetAdvertisingData,
            StatusEventParameter { status: ErrorCode::Success }
        ), &[4, 14, 4, 1, 8, 32, 0])]
    #[case::le_set_advertising_enable(CommandCompleteEvent::new(
            1, CommandOpCode::LeSetAdvertisingEnable,
            StatusEventParameter { status: ErrorCode::Success }
        ), &[4, 14, 4, 1, 10, 32, 0])]
    #[case::le_set_advertising_parameters(CommandCompleteEvent::new(
            1, CommandOpCode::LeSetAdvertisingParameters,
            StatusEventParameter { status: ErrorCode::Success }
        ), &[4, 14, 4, 1, 6, 32, 0])]
    #[case::le_set_event_mask(CommandCompleteEvent::new(
            1, CommandOpCode::LeSetEventMask,
            StatusEventParameter { status: ErrorCode::Success }
        ), &[4, 14, 4, 1, 1, 32, 0])]
    #[case::le_set_random_address(CommandCompleteEvent::new(
            1, CommandOpCode::LeSetRandomAddress,
            StatusEventParameter { status: ErrorCode::Success }
        ), &[4, 14, 4, 1, 5, 32, 0])]
    #[case::le_set_scan_enable(CommandCompleteEvent::new(
            1, CommandOpCode::LeSetScanEnable,
            StatusEventParameter { status: ErrorCode::Success }
        ), &[4, 14, 4, 1, 12, 32, 0])]
    #[case::le_set_scan_parameters(CommandCompleteEvent::new(
            1, CommandOpCode::LeSetScanParameters,
            StatusEventParameter { status: ErrorCode::Success }
        ), &[4, 14, 4, 1, 11, 32, 0])]
    #[case::le_set_scan_response_data(CommandCompleteEvent::new(
            1, CommandOpCode::LeSetScanResponseData,
            StatusEventParameter { status: ErrorCode::Success }
        ), &[4, 14, 4, 1, 9, 32, 0])]
    #[case::nop(CommandCompleteEvent::new(
            1, CommandOpCode::Nop,
            EventParameter::Empty
        ), &[4, 14, 3, 1, 0, 0])]
    #[case::read_bd_addr(CommandCompleteEvent::new(
            1, CommandOpCode::ReadBdAddr,
            StatusAndBdAddrEventParameter {
                status: ErrorCode::Success,
                bd_addr: PublicDeviceAddress::new([0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56])
            }
        ), &[4, 14, 10, 1, 9, 16, 0, 0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56])]
    #[case::read_buffer_size(CommandCompleteEvent::new(
            1, CommandOpCode::ReadBufferSize,
            StatusAndBufferSizeEventParameter {
                status: ErrorCode::Success,
                acl_data_packet_length: NonZeroU16::new(255).unwrap(),
                synchronous_data_packet_length: NonZeroU8::new(255).unwrap(),
                total_num_acl_data_packets: NonZeroU16::new(24).unwrap(),
                total_num_synchronous_packets: 12,
            }
        ), &[4, 14, 11, 1, 5, 16, 0, 255, 0, 255, 24, 0, 12, 0])]
    #[case::read_local_supported_commands(CommandCompleteEvent::new(
            1, CommandOpCode::ReadLocalSupportedCommands,
            StatusAndSupportedCommandsEventParameter {
                status: ErrorCode::Success,
                supported_commands: SupportedCommands::LE_RAND | SupportedCommands::LE_READ_LOCAL_SUPPORTED_FEATURES_PAGE_0
            }
        ), &[
            4, 14, 68, 1, 2, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 128,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ])]
    #[case::read_local_supported_features(CommandCompleteEvent::new(
            1, CommandOpCode::ReadLocalSupportedFeatures,
            StatusAndSupportedFeaturesEventParameter {
                status: ErrorCode::Success,
                supported_features: SupportedFeatures::LE_SUPPORTED_CONTROLLER
            }
        ), &[4, 14, 12, 1, 3, 16, 0, 0, 0, 0, 0, 64, 0, 0, 0])]
    #[case::reset(CommandCompleteEvent::new(
            1, CommandOpCode::Reset,
            StatusEventParameter { status: ErrorCode::Success }
        ), &[4, 14, 4, 1, 3, 12, 0])]
    #[case::set_event_mask(CommandCompleteEvent::new(
            1, CommandOpCode::SetEventMask,
            StatusEventParameter { status: ErrorCode::Success }
        ), &[4, 14, 4, 1, 1, 12, 0])]
    fn test_command_complete_event_parsing(
        #[case] event: CommandCompleteEvent,
        #[case] input: &[u8],
    ) {
        let (rest, packet) = packet(input).unwrap();
        assert_eq!(packet, Packet::Event(Event::CommandComplete(event)));
        assert!(rest.is_empty());
    }

    #[rstest]
    #[case::read_buffer_size_invalid_acl_data_packet_length(&[4, 14, 11, 1, 5, 16, 0, 0, 0, 255, 24, 0, 12, 0])]
    #[case::read_buffer_size_invalid_synchronous_data_packet_length(&[4, 14, 11, 1, 5, 16, 0, 255, 0, 0, 24, 0, 12, 0])]
    #[case::read_buffer_size_invalid_total_num_acl_data_packets(&[4, 14, 11, 1, 5, 16, 0, 255, 0, 255, 0, 0, 12, 0])]
    fn test_command_complete_event_parsing_failure(#[case] input: &[u8]) {
        assert!(packet(input).is_err());
    }

    #[test]
    fn test_command_complete_event_for_unsupported_command_parsing() {
        // Using Flush command opcode
        let err = packet(&[4, 14, 4, 1, 08, 12, 0]);
        assert!(err.is_err());
    }
}
