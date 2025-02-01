use crate::hci::{Event, HciCommand, HciError};

macro_rules! hci_packet_types {
    (
        $(
            $(#[$docs:meta])*
            $packet_type:ident = $value:expr,
        )+
    ) => {
        /// HCI packet type.
        ///
        /// HCI does not provide the ability to differentiate the five HCI packet types. Therefore, if
        /// the HCI packets are sent via a common physical interface, an HCI packet indicator has
        /// to be added. This is this packet type.
        ///
        /// See [Core Specification 6.0, Vol. 4, Part A, 2](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/uart-transport-layer.html#UUID-361053ee-862f-c591-00bd-1a941a12f949).
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr(u8)]
        #[non_exhaustive]
        pub(crate) enum HciPacketType {
            $(
                $(#[$docs])*
                $packet_type = $value,
            )+
        }

        impl TryFrom<u8> for HciPacketType {
            type Error = HciError;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    $(
                        $value => Ok(HciPacketType::$packet_type),
                    )+
                    _ => Err(HciError::InvalidPacketType(value)),
                }
            }
        }
    };
}

hci_packet_types! {
    Command = 0x01,
    AclData = 0x02,
    SynchronousData = 0x03,
    Event = 0x04,
    IsoData = 0x05,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum HciPacket<'a> {
    #[allow(dead_code)]
    Command(HciCommand<'a>),
    Event(Event),
}

pub(crate) mod parser {
    use nom::{combinator::map_res, number::le_u8, IResult, Parser};

    use crate::hci::{command::parser::command, event::parser::event, HciPacket, HciPacketType};

    pub(crate) fn parameter_total_length(input: &[u8]) -> IResult<&[u8], u8> {
        le_u8().parse(input)
    }

    pub(crate) fn packet(input: &[u8]) -> IResult<&[u8], HciPacket> {
        let (input, packet_type) = map_res(le_u8(), HciPacketType::try_from).parse(input)?;
        match packet_type {
            HciPacketType::Command => command.parse(input),
            HciPacketType::Event => event.parse(input),
            _ => {
                todo!("ACL data, synchronous data, and ISO data parsing")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use core::num::{NonZeroU16, NonZeroU8};

    use rstest::rstest;

    use super::*;
    use crate::{
        hci::{
            CommandCompleteEvent, EventMask, EventParameter, HciCommandOpCode, HciErrorCode,
            StatusAndBufferSizeEventParameter, StatusAndLeBufferSizeEventParameter,
            StatusAndSupportedCommandsEventParameter, StatusAndSupportedFeaturesEventParameter,
            StatusAndSupportedLeFeaturesEventParameter, StatusAndSupportedLeStatesEventParameter,
            StatusEventParameter, SupportedCommands, SupportedFeatures,
        },
        AdvertisingEnable, SupportedLeFeatures,
    };

    #[test]
    fn test_valid_packet_type() -> Result<(), HciError> {
        let hci_packet_type: HciPacketType = 4u8.try_into()?;
        assert_eq!(hci_packet_type, HciPacketType::Event);
        Ok(())
    }

    #[test]
    fn test_invalid_packet_type() {
        let err: Result<HciPacketType, HciError> = 10u8.try_into();
        assert!(matches!(err, Err(HciError::InvalidPacketType(_))));
    }

    #[test]
    fn test_unsupported_command_parsing() {
        // Use Flush command
        let (rest, hci_packet) = parser::packet(&[1, 8, 12, 0]).unwrap();
        assert!(matches!(
            hci_packet,
            HciPacket::Command(HciCommand::Unsupported(0x0C08))
        ));
        assert!(rest.is_empty());
    }

    #[rstest]
    #[case::le_read_buffer_size(HciCommand::LeReadBufferSize, &[1, 2, 32, 0])]
    #[case::le_read_local_supported_features_page_0(HciCommand::LeReadLocalSupportedFeaturesPage0, &[1, 3, 32, 0])]
    #[case::le_read_supported_states(HciCommand::LeReadSupportedStates, &[1, 28, 32, 0])]
    #[case::le_set_advertising_enable(HciCommand::LeSetAdvertisingEnable(
        AdvertisingEnable::Enabled
    ), &[1, 10, 32, 1, 1])]
    #[case::nop(HciCommand::Nop, &[1, 0, 0, 0])]
    #[case::read_buffer_size(HciCommand::ReadBufferSize, &[1, 5, 16, 0])]
    #[case::read_local_supported_commands(HciCommand::ReadLocalSupportedCommands, &[1, 2, 16, 0])]
    #[case::read_local_supported_features(HciCommand::ReadLocalSupportedFeatures, &[1, 3, 16, 0])]
    #[case::reset(HciCommand::Reset, &[1, 3, 12, 0])]
    #[case::set_event_mask(HciCommand::SetEventMask(
        EventMask::HARDWARE_ERROR | EventMask::DATA_BUFFER_OVERFLOW
    ), &[1, 1, 12, 8, 0, 128, 0, 2, 0, 0, 0, 0])]
    fn test_supported_command_parsing(#[case] command: HciCommand, #[case] input: &[u8]) {
        let (rest, hci_packet) = parser::packet(input).unwrap();
        assert_eq!(hci_packet, HciPacket::Command(command));
        assert!(rest.is_empty());
    }

    #[rstest]
    #[case::le_read_buffer_size(CommandCompleteEvent::new(
            1, HciCommandOpCode::LeReadBufferSize,
            StatusAndLeBufferSizeEventParameter {
                status: HciErrorCode::Success, le_acl_data_packet_length: 255, total_num_le_acl_data_packets: 24
            }
        ), &[4, 14, 7, 1, 2, 32, 0, 255, 0, 24])]
    #[case::le_read_local_supported_features_page_0(CommandCompleteEvent::new(
            1, HciCommandOpCode::LeReadLocalSupportedFeaturesPage0,
            StatusAndSupportedLeFeaturesEventParameter {
                status: HciErrorCode::Success,
                supported_le_features: SupportedLeFeatures::LE_ENCRYPTION | SupportedLeFeatures::LE_EXTENDED_ADVERTISING
            }
        ), &[4, 14, 12, 1, 3, 32, 0, 1, 16, 0, 0, 0, 0, 0, 0])]
    #[case::le_read_supported_states(CommandCompleteEvent::new(
            1, HciCommandOpCode::LeReadSupportedStates,
            StatusAndSupportedLeStatesEventParameter {
                status: HciErrorCode::Success, supported_le_states: 0x0000_03FF_FFFF_FFFF.into()
            }
        ), &[4, 14, 12, 1, 28, 32, 0, 255, 255, 255, 255, 255, 3, 0, 0])]
    #[case::le_set_advertising_data(CommandCompleteEvent::new(
            1, HciCommandOpCode::LeSetAdvertisingData,
            StatusEventParameter { status: HciErrorCode::Success }
        ), &[4, 14, 4, 1, 8, 32, 0])]
    #[case::le_set_advertising_enable(CommandCompleteEvent::new(
            1, HciCommandOpCode::LeSetAdvertisingEnable,
            StatusEventParameter { status: HciErrorCode::Success }
        ), &[4, 14, 4, 1, 10, 32, 0])]
    #[case::le_set_advertising_parameters(CommandCompleteEvent::new(
            1, HciCommandOpCode::LeSetAdvertisingParameters,
            StatusEventParameter { status: HciErrorCode::Success }
        ), &[4, 14, 4, 1, 6, 32, 0])]
    #[case::le_set_scan_response_data(CommandCompleteEvent::new(
            1, HciCommandOpCode::LeSetScanResponseData,
            StatusEventParameter { status: HciErrorCode::Success }
        ), &[4, 14, 4, 1, 9, 32, 0])]
    #[case::nop(CommandCompleteEvent::new(
            1, HciCommandOpCode::Nop,
            EventParameter::Empty
        ), &[4, 14, 3, 1, 0, 0])]
    #[case::read_buffer_size(CommandCompleteEvent::new(
            1, HciCommandOpCode::ReadBufferSize,
            StatusAndBufferSizeEventParameter {
                status: HciErrorCode::Success,
                acl_data_packet_length: NonZeroU16::new(255).unwrap(),
                synchronous_data_packet_length: NonZeroU8::new(255).unwrap(),
                total_num_acl_data_packets: NonZeroU16::new(24).unwrap(),
                total_num_synchronous_packets: 12,
            }
        ), &[4, 14, 11, 1, 5, 16, 0, 255, 0, 255, 24, 0, 12, 0])]
    #[case::read_local_supported_commands(CommandCompleteEvent::new(
            1, HciCommandOpCode::ReadLocalSupportedCommands,
            StatusAndSupportedCommandsEventParameter {
                status: HciErrorCode::Success,
                supported_commands: SupportedCommands::LE_RAND | SupportedCommands::LE_READ_LOCAL_SUPPORTED_FEATURES_PAGE_0
            }
        ), &[
            4, 14, 68, 1, 2, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 128,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ])]
    #[case::read_local_supported_features(CommandCompleteEvent::new(
            1, HciCommandOpCode::ReadLocalSupportedFeatures,
            StatusAndSupportedFeaturesEventParameter {
                status: HciErrorCode::Success,
                supported_features: SupportedFeatures::LE_SUPPORTED_CONTROLLER
            }
        ), &[4, 14, 12, 1, 3, 16, 0, 0, 0, 0, 0, 64, 0, 0, 0])]
    #[case::reset(CommandCompleteEvent::new(
            1, HciCommandOpCode::Reset,
            StatusEventParameter { status: HciErrorCode::Success }
        ), &[4, 14, 4, 1, 3, 12, 0])]
    #[case::set_event_mask(CommandCompleteEvent::new(
            1, HciCommandOpCode::SetEventMask,
            StatusEventParameter { status: HciErrorCode::Success }
        ), &[4, 14, 4, 1, 1, 12, 0])]
    fn test_command_complete_event_parsing(
        #[case] event: CommandCompleteEvent,
        #[case] input: &[u8],
    ) {
        let (rest, hci_packet) = parser::packet(input).unwrap();
        assert_eq!(hci_packet, HciPacket::Event(Event::CommandComplete(event)));
        assert!(rest.is_empty());
    }

    #[rstest]
    #[case::read_buffer_size_invalid_acl_data_packet_length(&[4, 14, 11, 1, 5, 16, 0, 0, 0, 255, 24, 0, 12, 0])]
    #[case::read_buffer_size_invalid_synchronous_data_packet_length(&[4, 14, 11, 1, 5, 16, 0, 255, 0, 0, 24, 0, 12, 0])]
    #[case::read_buffer_size_invalid_total_num_acl_data_packets(&[4, 14, 11, 1, 5, 16, 0, 255, 0, 255, 0, 0, 12, 0])]
    fn test_command_complete_event_parsing_failure(#[case] input: &[u8]) {
        assert!(parser::packet(input).is_err());
    }

    #[test]
    fn test_command_complete_event_for_unsupported_command_parsing() {
        // Using Flush command opcode
        let err = parser::packet(&[4, 14, 4, 1, 08, 12, 0]);
        assert!(err.is_err());
    }

    #[test]
    fn test_unsupported_event_parsing() {
        // Using Inquiry Complete event
        let (rest, hci_packet) = parser::packet(&[4, 1, 1, 0]).unwrap();
        assert!(matches!(
            hci_packet,
            HciPacket::Event(Event::Unsupported(1))
        ));
        assert!(rest.is_empty());
    }
}
