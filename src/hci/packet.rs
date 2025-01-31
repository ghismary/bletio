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

#[derive(Debug)]
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

    use super::*;
    use crate::{
        hci::{
            EventMask, EventParameter, HciCommandOpCode, HciErrorCode, SupportedCommands,
            SupportedFeatures,
        },
        le_states::{LeCombinedState, LeSingleState, LeState},
        AdvertisingEnable, SupportedLeFeatures,
    };
    use bitflags::Flags;

    #[test]
    fn test_nop_event_parsing() {
        let (rest, hci_packet) = parser::packet(&[4, 14, 3, 1, 0, 0]).unwrap();
        assert!(matches!(
            hci_packet,
            HciPacket::Event(Event::CommandComplete(_))
        ));
        assert!(rest.is_empty());
        if let HciPacket::Event(Event::CommandComplete(event)) = hci_packet {
            assert_eq!(event.num_hci_command_packets, 1);
            assert_eq!(event.opcode, HciCommandOpCode::Nop);
            assert!(matches!(event.parameter, EventParameter::Empty));
        }
    }

    #[test]
    fn test_reset_command_parsing() {
        let (rest, hci_packet) = parser::packet(&[1, 3, 12, 0]).unwrap();
        assert!(matches!(hci_packet, HciPacket::Command(HciCommand::Reset)));
        assert!(rest.is_empty());
    }

    #[test]
    fn test_command_complete_event_for_reset_command_parsing() {
        let (rest, hci_packet) = parser::packet(&[4, 14, 4, 1, 3, 12, 0]).unwrap();
        assert!(matches!(
            hci_packet,
            HciPacket::Event(Event::CommandComplete(_))
        ));
        assert!(rest.is_empty());
        if let HciPacket::Event(Event::CommandComplete(event)) = hci_packet {
            assert_eq!(event.num_hci_command_packets, 1);
            assert_eq!(event.opcode, HciCommandOpCode::Reset);
            assert!(matches!(event.parameter, EventParameter::Status(_)));
            if let EventParameter::Status(param) = event.parameter {
                assert_eq!(param.status, HciErrorCode::Success);
            }
        }
    }

    #[test]
    fn test_read_local_supported_commands_command_parsing() {
        let (rest, hci_packet) = parser::packet(&[1, 2, 16, 0]).unwrap();
        assert!(matches!(
            hci_packet,
            HciPacket::Command(HciCommand::ReadLocalSupportedCommands)
        ));
        assert!(rest.is_empty());
    }

    #[test]
    fn test_command_complete_event_for_read_local_supported_commands_command_parsing() {
        let (rest, hci_packet) = parser::packet(&[
            4, 14, 68, 1, 2, 16, 0, 32, 0, 0, 0, 0, 64, 0, 0, 0, 0, 224, 0, 0, 0, 40, 34, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 247, 255, 255, 127, 0, 0, 0, 0, 240, 249, 255, 255, 255, 255, 7, 224,
            3, 0, 4, 249, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ])
        .unwrap();
        assert!(matches!(
            hci_packet,
            HciPacket::Event(Event::CommandComplete(_))
        ));
        assert!(rest.is_empty());
        if let HciPacket::Event(Event::CommandComplete(event)) = hci_packet {
            assert_eq!(event.num_hci_command_packets, 1);
            assert_eq!(event.opcode, HciCommandOpCode::ReadLocalSupportedCommands);
            assert!(matches!(
                event.parameter,
                EventParameter::StatusAndSupportedCommands(_)
            ));
            if let EventParameter::StatusAndSupportedCommands(param) = event.parameter {
                assert_eq!(param.status, HciErrorCode::Success);
                assert!(param
                    .supported_commands
                    .contains(SupportedCommands::LE_READ_LOCAL_SUPPORTED_FEATURES_PAGE_0));
                assert!(!param
                    .supported_commands
                    .contains(SupportedCommands::LE_CREATE_BIG));
            }
        }
    }

    #[test]
    fn test_read_local_supported_features_command_parsing() {
        let (rest, hci_packet) = parser::packet(&[1, 3, 16, 0]).unwrap();
        assert!(matches!(
            hci_packet,
            HciPacket::Command(HciCommand::ReadLocalSupportedFeatures)
        ));
        assert!(rest.is_empty());
    }

    #[test]
    fn test_command_complete_event_for_read_local_supported_features_command_parsing() {
        let (rest, hci_packet) =
            parser::packet(&[4, 14, 12, 1, 3, 16, 0, 0, 0, 0, 0, 96, 0, 0, 0]).unwrap();
        assert!(matches!(
            hci_packet,
            HciPacket::Event(Event::CommandComplete(_))
        ));
        assert!(rest.is_empty());
        if let HciPacket::Event(Event::CommandComplete(event)) = hci_packet {
            assert_eq!(event.num_hci_command_packets, 1);
            assert_eq!(event.opcode, HciCommandOpCode::ReadLocalSupportedFeatures);
            assert!(matches!(
                event.parameter,
                EventParameter::StatusAndSupportedFeatures(_)
            ));
            if let EventParameter::StatusAndSupportedFeatures(param) = event.parameter {
                assert_eq!(param.status, HciErrorCode::Success);
                assert!(param
                    .supported_features
                    .contains(SupportedFeatures::LE_SUPPORTED_CONTROLLER));
                assert!(!param.supported_features.contains(
                    SupportedFeatures::SIMULTANEOUS_LE_AND_BREDR_TO_SAME_DEVICE_CAPABLE_CONTROLLER
                ));
            }
        }
    }

    #[test]
    fn test_set_event_mask_command_parsing() {
        let (rest, hci_packet) =
            parser::packet(&[1, 1, 12, 8, 144, 136, 0, 2, 0, 128, 0, 32]).unwrap();
        assert!(matches!(
            hci_packet,
            HciPacket::Command(HciCommand::SetEventMask(_))
        ));
        assert!(rest.is_empty());
        if let HciPacket::Command(HciCommand::SetEventMask(event_mask)) = hci_packet {
            assert!(event_mask.contains(EventMask::HARDWARE_ERROR));
            assert!(event_mask.contains(EventMask::DATA_BUFFER_OVERFLOW))
        }
    }

    #[test]
    fn test_command_complete_event_for_set_event_mask_command_parsing() {
        let (rest, hci_packet) = parser::packet(&[4, 14, 4, 1, 1, 12, 0]).unwrap();
        assert!(matches!(
            hci_packet,
            HciPacket::Event(Event::CommandComplete(_))
        ));
        assert!(rest.is_empty());
        if let HciPacket::Event(Event::CommandComplete(event)) = hci_packet {
            assert_eq!(event.num_hci_command_packets, 1);
            assert_eq!(event.opcode, HciCommandOpCode::SetEventMask);
            assert!(matches!(event.parameter, EventParameter::Status(_)));
            if let EventParameter::Status(param) = event.parameter {
                assert_eq!(param.status, HciErrorCode::Success);
            }
        }
    }

    #[test]
    fn test_le_read_buffer_size_command_parsing() {
        let (rest, hci_packet) = parser::packet(&[1, 2, 32, 0]).unwrap();
        assert!(matches!(
            hci_packet,
            HciPacket::Command(HciCommand::LeReadBufferSize)
        ));
        assert!(rest.is_empty());
    }

    #[test]
    fn test_command_complete_event_for_le_read_buffer_size_command_parsing() {
        let (rest, hci_packet) = parser::packet(&[4, 14, 7, 1, 2, 32, 0, 255, 0, 24]).unwrap();
        assert!(matches!(
            hci_packet,
            HciPacket::Event(Event::CommandComplete(_))
        ));
        assert!(rest.is_empty());
        if let HciPacket::Event(Event::CommandComplete(event)) = hci_packet {
            assert_eq!(event.num_hci_command_packets, 1);
            assert_eq!(event.opcode, HciCommandOpCode::LeReadBufferSize);
            assert!(matches!(
                event.parameter,
                EventParameter::StatusAndLeBufferSize(_)
            ));
            if let EventParameter::StatusAndLeBufferSize(param) = event.parameter {
                assert_eq!(param.status, HciErrorCode::Success);
                assert_eq!(param.le_acl_data_packet_length, 255);
                assert_eq!(param.total_num_le_acl_data_packets, 24);
            }
        }
    }

    #[test]
    fn test_read_buffer_size_command_parsing() {
        let (rest, hci_packet) = parser::packet(&[1, 5, 16, 0]).unwrap();
        assert!(matches!(
            hci_packet,
            HciPacket::Command(HciCommand::ReadBufferSize)
        ));
        assert!(rest.is_empty());
    }

    #[test]
    fn test_command_complete_event_for_read_buffer_size_command_parsing() {
        let (rest, hci_packet) =
            parser::packet(&[4, 14, 11, 1, 5, 16, 0, 255, 0, 255, 24, 0, 12, 0]).unwrap();
        assert!(matches!(
            hci_packet,
            HciPacket::Event(Event::CommandComplete(_))
        ));
        assert!(rest.is_empty());
        if let HciPacket::Event(Event::CommandComplete(event)) = hci_packet {
            assert_eq!(event.num_hci_command_packets, 1);
            assert_eq!(event.opcode, HciCommandOpCode::ReadBufferSize);
            assert!(matches!(
                event.parameter,
                EventParameter::StatusAndBufferSize(_)
            ));
            if let EventParameter::StatusAndBufferSize(param) = event.parameter {
                assert_eq!(param.status, HciErrorCode::Success);
                assert_eq!(param.acl_data_packet_length, NonZeroU16::new(255).unwrap());
                assert_eq!(
                    param.synchronous_data_packet_length,
                    NonZeroU8::new(255).unwrap()
                );
                assert_eq!(
                    param.total_num_acl_data_packets,
                    NonZeroU16::new(24).unwrap()
                );
                assert_eq!(param.total_num_synchronous_packets, 12);
            }
        }
    }

    #[test]
    fn test_command_complete_event_for_read_buffer_size_command_parsing_failure_invalid_acl_data_packet_length(
    ) {
        let err = parser::packet(&[4, 14, 11, 1, 5, 16, 0, 0, 0, 255, 24, 0, 12, 0]);
        assert!(err.is_err());
    }

    #[test]
    fn test_command_complete_event_for_read_buffer_size_command_parsing_failure_invalid_synchronous_data_packet_length(
    ) {
        let err = parser::packet(&[4, 14, 11, 1, 5, 16, 0, 255, 0, 0, 24, 0, 12, 0]);
        assert!(err.is_err());
    }

    #[test]
    fn test_command_complete_event_for_read_buffer_size_command_parsing_failure_invalid_total_num_acl_data_packets(
    ) {
        let err = parser::packet(&[4, 14, 11, 1, 5, 16, 0, 255, 0, 255, 0, 0, 12, 0]);
        assert!(err.is_err());
    }

    #[test]
    fn test_le_read_local_supported_features_page_0_command_parsing() {
        let (rest, hci_packet) = parser::packet(&[1, 3, 32, 0]).unwrap();
        assert!(matches!(
            hci_packet,
            HciPacket::Command(HciCommand::LeReadLocalSupportedFeaturesPage0)
        ));
        assert!(rest.is_empty());
    }

    #[test]
    fn test_command_complete_event_for_le_read_local_supported_features_page_0_command_parsing() {
        let (rest, hci_packet) =
            parser::packet(&[4, 14, 12, 1, 3, 32, 0, 255, 121, 1, 15, 144, 0, 0, 0]).unwrap();
        assert!(matches!(
            hci_packet,
            HciPacket::Event(Event::CommandComplete(_))
        ));
        assert!(rest.is_empty());
        if let HciPacket::Event(Event::CommandComplete(event)) = hci_packet {
            assert_eq!(event.num_hci_command_packets, 1);
            assert_eq!(
                event.opcode,
                HciCommandOpCode::LeReadLocalSupportedFeaturesPage0
            );
            assert!(matches!(
                event.parameter,
                EventParameter::StatusAndSupportedLeFeatures(_)
            ));
            if let EventParameter::StatusAndSupportedLeFeatures(param) = event.parameter {
                assert_eq!(param.status, HciErrorCode::Success);
                assert!(param
                    .supported_le_features
                    .contains(SupportedLeFeatures::LE_ENCRYPTION));
                assert!(!param
                    .supported_le_features
                    .contains(SupportedLeFeatures::CHANNEL_SOUNDING));
            }
        }
    }

    #[test]
    fn test_le_read_supported_states_command_parsing() {
        let (rest, hci_packet) = parser::packet(&[1, 28, 32, 0]).unwrap();
        assert!(matches!(
            hci_packet,
            HciPacket::Command(HciCommand::LeReadSupportedStates)
        ));
        assert!(rest.is_empty());
    }

    #[test]
    fn test_command_complete_event_for_le_read_supported_states_command_parsing() {
        let (rest, hci_packet) =
            parser::packet(&[4, 14, 12, 1, 28, 32, 0, 255, 255, 255, 255, 255, 3, 0, 0]).unwrap();
        assert!(matches!(
            hci_packet,
            HciPacket::Event(Event::CommandComplete(_))
        ));
        assert!(rest.is_empty());
        if let HciPacket::Event(Event::CommandComplete(event)) = hci_packet {
            assert_eq!(event.num_hci_command_packets, 1);
            assert_eq!(event.opcode, HciCommandOpCode::LeReadSupportedStates);
            assert!(matches!(
                event.parameter,
                EventParameter::StatusAndSupportedLeStates(_)
            ));
            if let EventParameter::StatusAndSupportedLeStates(param) = event.parameter {
                assert_eq!(param.status, HciErrorCode::Success);
                assert!(param
                    .supported_le_states
                    .is_supported(LeState::Single(LeSingleState::NonConnectableAdvertising)));
                assert!(!param.supported_le_states.is_supported(LeState::Combined(
                    LeCombinedState(
                        LeSingleState::ConnectionMasterRole,
                        LeSingleState::ConnectionSlaveRole
                    )
                )));
            }
        }
    }

    #[test]
    fn test_command_complete_event_for_le_set_advertising_parameters_command_parsing() {
        let (rest, hci_packet) = parser::packet(&[4, 14, 4, 1, 6, 32, 0]).unwrap();
        assert!(matches!(
            hci_packet,
            HciPacket::Event(Event::CommandComplete(_))
        ));
        assert!(rest.is_empty());
        if let HciPacket::Event(Event::CommandComplete(event)) = hci_packet {
            assert_eq!(event.num_hci_command_packets, 1);
            assert_eq!(event.opcode, HciCommandOpCode::LeSetAdvertisingParameters);
            assert!(matches!(event.parameter, EventParameter::Status(_)));
            if let EventParameter::Status(param) = event.parameter {
                assert_eq!(param.status, HciErrorCode::Success);
            }
        }
    }

    #[test]
    fn test_command_complete_event_for_le_set_advertising_data_command_parsing() {
        let (rest, hci_packet) = parser::packet(&[4, 14, 4, 1, 8, 32, 0]).unwrap();
        assert!(matches!(
            hci_packet,
            HciPacket::Event(Event::CommandComplete(_))
        ));
        assert!(rest.is_empty());
        if let HciPacket::Event(Event::CommandComplete(event)) = hci_packet {
            assert_eq!(event.num_hci_command_packets, 1);
            assert_eq!(event.opcode, HciCommandOpCode::LeSetAdvertisingData);
            assert!(matches!(event.parameter, EventParameter::Status(_)));
            if let EventParameter::Status(param) = event.parameter {
                assert_eq!(param.status, HciErrorCode::Success);
            }
        }
    }

    #[test]
    fn test_command_complete_event_for_le_set_scan_response_data_command_parsing() {
        let (rest, hci_packet) = parser::packet(&[4, 14, 4, 1, 9, 32, 0]).unwrap();
        assert!(matches!(
            hci_packet,
            HciPacket::Event(Event::CommandComplete(_))
        ));
        assert!(rest.is_empty());
        if let HciPacket::Event(Event::CommandComplete(event)) = hci_packet {
            assert_eq!(event.num_hci_command_packets, 1);
            assert_eq!(event.opcode, HciCommandOpCode::LeSetScanResponseData);
            assert!(matches!(event.parameter, EventParameter::Status(_)));
            if let EventParameter::Status(param) = event.parameter {
                assert_eq!(param.status, HciErrorCode::Success);
            }
        }
    }

    #[test]
    fn test_le_set_advertising_enable_command_parsing() {
        let (rest, hci_packet) = parser::packet(&[1, 10, 32, 1, 1]).unwrap();
        assert!(matches!(
            hci_packet,
            HciPacket::Command(HciCommand::LeSetAdvertisingEnable(_))
        ));
        assert!(rest.is_empty());
        if let HciPacket::Command(HciCommand::LeSetAdvertisingEnable(advertising_enable)) =
            hci_packet
        {
            assert_eq!(advertising_enable, AdvertisingEnable::Enabled);
        }
    }

    #[test]
    fn test_command_complete_event_for_le_set_advertising_enable_command_parsing() {
        let (rest, hci_packet) = parser::packet(&[4, 14, 4, 1, 10, 32, 0]).unwrap();
        assert!(matches!(
            hci_packet,
            HciPacket::Event(Event::CommandComplete(_))
        ));
        assert!(rest.is_empty());
        if let HciPacket::Event(Event::CommandComplete(event)) = hci_packet {
            assert_eq!(event.num_hci_command_packets, 1);
            assert_eq!(event.opcode, HciCommandOpCode::LeSetAdvertisingEnable);
            assert!(matches!(event.parameter, EventParameter::Status(_)));
            if let EventParameter::Status(param) = event.parameter {
                assert_eq!(param.status, HciErrorCode::Success);
            }
        }
    }
}
