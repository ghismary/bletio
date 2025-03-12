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
    pub(crate) status: ErrorCode,
    pub(crate) parameter: Option<EventParameter>,
}

impl CommandCompleteEvent {
    pub(crate) fn new(
        num_hci_command_packets: u8,
        opcode: CommandOpCode,
        status: ErrorCode,
        parameter: Option<impl Into<EventParameter>>,
    ) -> Self {
        Self {
            num_hci_command_packets,
            opcode,
            status,
            parameter: parameter.map(Into::into),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) enum EventParameter {
    BdAddr(BdAddrEventParameter),
    BufferSize(BufferSizeEventParameter),
    LeBufferSize(LeBufferSizeEventParameter),
    RandomNumber(RandomNumberEventParameter),
    SupportedCommands(SupportedCommandsEventParameter),
    SupportedFeatures(SupportedFeaturesEventParameter),
    SupportedLeFeatures(SupportedLeFeaturesEventParameter),
    SupportedLeStates(SupportedLeStatesEventParameter),
    TxPowerLevel(TxPowerLevelEventParameter),
    FilterAcceptListSize(FilterAcceptListSizeEventParameter),
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct BdAddrEventParameter {
    pub(crate) bd_addr: PublicDeviceAddress,
}

impl From<BdAddrEventParameter> for EventParameter {
    fn from(value: BdAddrEventParameter) -> Self {
        Self::BdAddr(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct BufferSizeEventParameter {
    pub(crate) acl_data_packet_length: NonZeroU16,
    pub(crate) synchronous_data_packet_length: NonZeroU8,
    pub(crate) total_num_acl_data_packets: NonZeroU16,
    pub(crate) total_num_synchronous_packets: u16,
}

impl From<BufferSizeEventParameter> for EventParameter {
    fn from(value: BufferSizeEventParameter) -> Self {
        Self::BufferSize(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct LeBufferSizeEventParameter {
    pub(crate) le_acl_data_packet_length: u16,
    pub(crate) total_num_le_acl_data_packets: u8,
}

impl From<LeBufferSizeEventParameter> for EventParameter {
    fn from(value: LeBufferSizeEventParameter) -> Self {
        Self::LeBufferSize(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct RandomNumberEventParameter {
    pub(crate) random_number: [u8; 8],
}

impl From<RandomNumberEventParameter> for EventParameter {
    fn from(value: RandomNumberEventParameter) -> Self {
        Self::RandomNumber(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct SupportedCommandsEventParameter {
    pub(crate) supported_commands: SupportedCommands,
}

impl From<SupportedCommandsEventParameter> for EventParameter {
    fn from(value: SupportedCommandsEventParameter) -> Self {
        Self::SupportedCommands(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct SupportedFeaturesEventParameter {
    pub(crate) supported_features: SupportedFeatures,
}

impl From<SupportedFeaturesEventParameter> for EventParameter {
    fn from(value: SupportedFeaturesEventParameter) -> Self {
        Self::SupportedFeatures(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct SupportedLeFeaturesEventParameter {
    pub(crate) supported_le_features: SupportedLeFeatures,
}

impl From<SupportedLeFeaturesEventParameter> for EventParameter {
    fn from(value: SupportedLeFeaturesEventParameter) -> Self {
        Self::SupportedLeFeatures(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct SupportedLeStatesEventParameter {
    pub(crate) supported_le_states: SupportedLeStates,
}

impl From<SupportedLeStatesEventParameter> for EventParameter {
    fn from(value: SupportedLeStatesEventParameter) -> Self {
        Self::SupportedLeStates(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct TxPowerLevelEventParameter {
    pub(crate) tx_power_level: TxPowerLevel,
}

impl From<TxPowerLevelEventParameter> for EventParameter {
    fn from(value: TxPowerLevelEventParameter) -> Self {
        Self::TxPowerLevel(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct FilterAcceptListSizeEventParameter {
    pub(crate) filter_accept_list_size: usize,
}

impl From<FilterAcceptListSizeEventParameter> for EventParameter {
    fn from(value: FilterAcceptListSizeEventParameter) -> Self {
        Self::FilterAcceptListSize(value)
    }
}

pub(crate) mod parser {
    use bitflags::Flags;
    use nom::{
        bytes::take,
        combinator::{eof, map, map_res},
        number::complete::{le_i8, le_u16, le_u64, le_u8},
        sequence::pair,
        IResult, Parser,
    };

    use crate::command::parser::command_opcode;
    use crate::event::parser::{hci_error_code, num_hci_command_packets};

    use super::*;

    fn supported_commands(input: &[u8]) -> IResult<&[u8], SupportedCommands> {
        map(map_res(take(64u8), TryInto::try_into), |v: [u8; 64]| {
            v.into()
        })
        .parse(input)
    }

    fn supported_features(input: &[u8]) -> IResult<&[u8], SupportedFeatures> {
        map(le_u64, SupportedFeatures::from_bits_truncate).parse(input)
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
            map_res(le_u16, TryInto::try_into),
            map_res(le_u8, TryInto::try_into),
            map_res(le_u16, TryInto::try_into),
            le_u16,
        )
            .parse(input)
    }

    fn le_buffer_size(input: &[u8]) -> IResult<&[u8], (u16, u8)> {
        (le_u16, le_u8).parse(input)
    }

    fn le_supported_features_page_0(input: &[u8]) -> IResult<&[u8], SupportedLeFeatures> {
        map(take(8u8), Into::into).parse(input)
    }

    fn le_supported_states(input: &[u8]) -> IResult<&[u8], SupportedLeStates> {
        map(le_u64, Into::into).parse(input)
    }

    fn tx_power_level(input: &[u8]) -> IResult<&[u8], TxPowerLevel> {
        map_res(le_i8, TryInto::try_into).parse(input)
    }

    fn random_number(input: &[u8]) -> IResult<&[u8], [u8; 8]> {
        map_res(take(8u8), TryInto::try_into).parse(input)
    }

    fn filter_accept_list_size(input: &[u8]) -> IResult<&[u8], usize> {
        map(le_u8, |v| v as usize).parse(input)
    }

    pub(crate) fn command_complete_event(input: &[u8]) -> IResult<&[u8], CommandCompleteEvent> {
        let (return_parameters, (num_hci_command_packets, command_opcode)) =
            pair(num_hci_command_packets, command_opcode).parse(input)?;
        let (status, event_parameter) = match command_opcode {
            CommandOpCode::Nop => {
                eof(return_parameters)?;
                (ErrorCode::Success, None::<EventParameter>)
            }
            CommandOpCode::SetEventMask
            | CommandOpCode::Reset
            | CommandOpCode::LeAddDeviceToFilterAcceptList
            | CommandOpCode::LeClearFilterAcceptList
            | CommandOpCode::LeCreateConnectionCancel
            | CommandOpCode::LeRemoveDeviceFromFilterAcceptList
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
                (status, None::<EventParameter>)
            }
            CommandOpCode::LeRand => {
                let (rest, status) = hci_error_code(return_parameters)?;
                let (rest, random_number) = if status.is_success() {
                    random_number(rest)?
                } else {
                    (rest, [0u8; 8])
                };
                eof(rest)?;
                (
                    status,
                    Some(RandomNumberEventParameter { random_number }.into()),
                )
            }
            CommandOpCode::LeReadAdvertisingChannelTxPower => {
                let (rest, status) = hci_error_code(return_parameters)?;
                let (rest, tx_power_level) = if status.is_success() {
                    tx_power_level(rest)?
                } else {
                    (rest, TxPowerLevel::default())
                };
                eof(rest)?;
                (
                    status,
                    Some(TxPowerLevelEventParameter { tx_power_level }.into()),
                )
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
                (
                    status,
                    Some(
                        LeBufferSizeEventParameter {
                            le_acl_data_packet_length,
                            total_num_le_acl_data_packets,
                        }
                        .into(),
                    ),
                )
            }
            CommandOpCode::LeReadFilterAcceptListSize => {
                let (rest, status) = hci_error_code(return_parameters)?;
                let (rest, filter_accept_list_size) = if status.is_success() {
                    filter_accept_list_size(rest)?
                } else {
                    (rest, 0)
                };
                eof(rest)?;
                (
                    status,
                    Some(
                        FilterAcceptListSizeEventParameter {
                            filter_accept_list_size,
                        }
                        .into(),
                    ),
                )
            }
            CommandOpCode::LeReadLocalSupportedFeaturesPage0 => {
                let (rest, status) = hci_error_code(return_parameters)?;
                let (rest, supported_le_features) = if status.is_success() {
                    le_supported_features_page_0(rest)?
                } else {
                    (rest, SupportedLeFeatures::empty())
                };
                eof(rest)?;
                (
                    status,
                    Some(
                        SupportedLeFeaturesEventParameter {
                            supported_le_features,
                        }
                        .into(),
                    ),
                )
            }
            CommandOpCode::LeReadSupportedStates => {
                let (rest, status) = hci_error_code(return_parameters)?;
                let (rest, supported_le_states) = if status.is_success() {
                    le_supported_states(rest)?
                } else {
                    (rest, SupportedLeStates::default())
                };
                eof(rest)?;
                (
                    status,
                    Some(
                        SupportedLeStatesEventParameter {
                            supported_le_states,
                        }
                        .into(),
                    ),
                )
            }
            CommandOpCode::ReadLocalSupportedCommands => {
                let (rest, status) = hci_error_code(return_parameters)?;
                let (rest, supported_commands) = if status.is_success() {
                    supported_commands(rest)?
                } else {
                    (rest, SupportedCommands::empty())
                };
                eof(rest)?;
                (
                    status,
                    Some(SupportedCommandsEventParameter { supported_commands }.into()),
                )
            }
            CommandOpCode::ReadLocalSupportedFeatures => {
                let (rest, status) = hci_error_code(return_parameters)?;
                let (rest, supported_features) = if status.is_success() {
                    supported_features(rest)?
                } else {
                    (rest, SupportedFeatures::empty())
                };
                eof(rest)?;
                (
                    status,
                    Some(SupportedFeaturesEventParameter { supported_features }.into()),
                )
            }
            CommandOpCode::ReadBdAddr => {
                let (rest, status) = hci_error_code(return_parameters)?;
                let (rest, bd_addr) = if status.is_success() {
                    bd_addr(rest)?
                } else {
                    (rest, PublicDeviceAddress::default())
                };
                eof(rest)?;
                (status, Some(BdAddrEventParameter { bd_addr }.into()))
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
                (
                    status,
                    Some(
                        BufferSizeEventParameter {
                            acl_data_packet_length,
                            synchronous_data_packet_length,
                            total_num_acl_data_packets,
                            total_num_synchronous_packets,
                        }
                        .into(),
                    ),
                )
            }
            CommandOpCode::Disconnect
            | CommandOpCode::LeCreateConnection
            | CommandOpCode::Unsupported(_) => {
                return Err(nom::Err::Failure(nom::error::Error::new(
                    return_parameters,
                    nom::error::ErrorKind::Fail,
                )));
            }
        };
        Ok((
            &[],
            CommandCompleteEvent::new(
                num_hci_command_packets,
                command_opcode,
                status,
                event_parameter,
            ),
        ))
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::{packet::parser::packet, Event, Packet};

    use super::*;

    #[rstest]
    #[case::le_add_device_to_filter_accept_list(CommandCompleteEvent::new(
            1, CommandOpCode::LeAddDeviceToFilterAcceptList, ErrorCode::Success, None::<EventParameter>
        ), &[4, 14, 4, 1, 17, 32, 0])]
    #[case::le_create_connection_cancel(CommandCompleteEvent::new(
            1, CommandOpCode::LeCreateConnectionCancel, ErrorCode::Success, None::<EventParameter>
        ), &[4, 14, 4, 1, 14, 32, 0])]
    #[case::le_clear_filter_accept_list(CommandCompleteEvent::new(
            1, CommandOpCode::LeClearFilterAcceptList, ErrorCode::Success, None::<EventParameter>
        ), &[4, 14, 4, 1, 16, 32, 0])]
    #[case::le_rand(CommandCompleteEvent::new(
            1, CommandOpCode::LeRand, ErrorCode::Success,
            Some(RandomNumberEventParameter { random_number: [68, 223, 27, 9, 83, 58, 224, 240] })
        ), &[4, 14, 12, 1, 24, 32, 0, 68, 223, 27, 9, 83, 58, 224, 240])]
    #[case::le_read_advertising_channel_tx_power(CommandCompleteEvent::new(
            1, CommandOpCode::LeReadAdvertisingChannelTxPower, ErrorCode::Success,
            Some(TxPowerLevelEventParameter {
                tx_power_level: TxPowerLevel::try_new(9).unwrap()
            })
        ), &[4, 14, 5, 1, 7, 32, 0, 9])]
    #[case::le_read_buffer_size(CommandCompleteEvent::new(
            1, CommandOpCode::LeReadBufferSize, ErrorCode::Success,
            Some(LeBufferSizeEventParameter {
                le_acl_data_packet_length: 255, total_num_le_acl_data_packets: 24
            })
        ), &[4, 14, 7, 1, 2, 32, 0, 255, 0, 24])]
    #[case::le_read_filter_accept_list_size(CommandCompleteEvent::new(
            1, CommandOpCode::LeReadFilterAcceptListSize, ErrorCode::Success,
            Some(FilterAcceptListSizeEventParameter { filter_accept_list_size: 12 })
        ), &[4, 14, 5, 1, 15, 32, 0, 12])]
    #[case::le_read_local_supported_features_page_0(CommandCompleteEvent::new(
            1, CommandOpCode::LeReadLocalSupportedFeaturesPage0, ErrorCode::Success,
            Some(SupportedLeFeaturesEventParameter {
                supported_le_features: SupportedLeFeatures::LE_ENCRYPTION | SupportedLeFeatures::LE_EXTENDED_ADVERTISING
            })
        ), &[4, 14, 12, 1, 3, 32, 0, 1, 16, 0, 0, 0, 0, 0, 0])]
    #[case::le_read_supported_states(CommandCompleteEvent::new(
            1, CommandOpCode::LeReadSupportedStates, ErrorCode::Success,
            Some(SupportedLeStatesEventParameter {
                supported_le_states: 0x0000_03FF_FFFF_FFFF.into()
            })
        ), &[4, 14, 12, 1, 28, 32, 0, 255, 255, 255, 255, 255, 3, 0, 0])]
    #[case::le_remove_device_from_filter_accept_list(CommandCompleteEvent::new(
            1, CommandOpCode::LeRemoveDeviceFromFilterAcceptList, ErrorCode::Success, None::<EventParameter>
        ), &[4, 14, 4, 1, 18, 32, 0])]
    #[case::le_set_advertising_data(CommandCompleteEvent::new(
            1, CommandOpCode::LeSetAdvertisingData, ErrorCode::Success, None::<EventParameter>
        ), &[4, 14, 4, 1, 8, 32, 0])]
    #[case::le_set_advertising_enable(CommandCompleteEvent::new(
            1, CommandOpCode::LeSetAdvertisingEnable, ErrorCode::Success, None::<EventParameter>
        ), &[4, 14, 4, 1, 10, 32, 0])]
    #[case::le_set_advertising_parameters(CommandCompleteEvent::new(
            1, CommandOpCode::LeSetAdvertisingParameters, ErrorCode::Success, None::<EventParameter>
        ), &[4, 14, 4, 1, 6, 32, 0])]
    #[case::le_set_event_mask(CommandCompleteEvent::new(
            1, CommandOpCode::LeSetEventMask, ErrorCode::Success, None::<EventParameter>
        ), &[4, 14, 4, 1, 1, 32, 0])]
    #[case::le_set_random_address(CommandCompleteEvent::new(
            1, CommandOpCode::LeSetRandomAddress, ErrorCode::Success, None::<EventParameter>
        ), &[4, 14, 4, 1, 5, 32, 0])]
    #[case::le_set_scan_enable(CommandCompleteEvent::new(
            1, CommandOpCode::LeSetScanEnable, ErrorCode::Success, None::<EventParameter>
        ), &[4, 14, 4, 1, 12, 32, 0])]
    #[case::le_set_scan_parameters(CommandCompleteEvent::new(
            1, CommandOpCode::LeSetScanParameters, ErrorCode::Success, None::<EventParameter>
        ), &[4, 14, 4, 1, 11, 32, 0])]
    #[case::le_set_scan_response_data(CommandCompleteEvent::new(
            1, CommandOpCode::LeSetScanResponseData, ErrorCode::Success, None::<EventParameter>
        ), &[4, 14, 4, 1, 9, 32, 0])]
    #[case::nop(CommandCompleteEvent::new(1, CommandOpCode::Nop, ErrorCode::Success, None::<EventParameter>), &[4, 14, 3, 1, 0, 0])]
    #[case::read_bd_addr(CommandCompleteEvent::new(
            1, CommandOpCode::ReadBdAddr, ErrorCode::Success,
            Some(BdAddrEventParameter {
                bd_addr: PublicDeviceAddress::new([0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56])
            })
        ), &[4, 14, 10, 1, 9, 16, 0, 0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56])]
    #[case::read_buffer_size(CommandCompleteEvent::new(
            1, CommandOpCode::ReadBufferSize, ErrorCode::Success,
            Some(BufferSizeEventParameter {
                acl_data_packet_length: NonZeroU16::new(255).unwrap(),
                synchronous_data_packet_length: NonZeroU8::new(255).unwrap(),
                total_num_acl_data_packets: NonZeroU16::new(24).unwrap(),
                total_num_synchronous_packets: 12,
            })
        ), &[4, 14, 11, 1, 5, 16, 0, 255, 0, 255, 24, 0, 12, 0])]
    #[case::read_local_supported_commands(CommandCompleteEvent::new(
            1, CommandOpCode::ReadLocalSupportedCommands, ErrorCode::Success,
            Some(SupportedCommandsEventParameter {
                supported_commands: SupportedCommands::LE_RAND | SupportedCommands::LE_READ_LOCAL_SUPPORTED_FEATURES_PAGE_0
            })
        ), &[
            4, 14, 68, 1, 2, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 128,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ])]
    #[case::read_local_supported_features(CommandCompleteEvent::new(
            1, CommandOpCode::ReadLocalSupportedFeatures, ErrorCode::Success,
            Some(SupportedFeaturesEventParameter {
                supported_features: SupportedFeatures::LE_SUPPORTED_CONTROLLER
            })
        ), &[4, 14, 12, 1, 3, 16, 0, 0, 0, 0, 0, 64, 0, 0, 0])]
    #[case::reset(CommandCompleteEvent::new(
            1, CommandOpCode::Reset, ErrorCode::Success, None::<EventParameter>
        ), &[4, 14, 4, 1, 3, 12, 0])]
    #[case::set_event_mask(CommandCompleteEvent::new(
            1, CommandOpCode::SetEventMask, ErrorCode::Success, None::<EventParameter>
        ), &[4, 14, 4, 1, 1, 12, 0])]
    fn test_command_complete_event_parsing_success(
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
        let err = packet(&[4, 14, 4, 1, 8, 12, 0]);
        assert!(err.is_err());
    }
}
