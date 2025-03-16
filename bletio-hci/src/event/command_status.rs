use crate::command::CommandOpCode;
use crate::ErrorCode;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CommandStatusEvent {
    pub(crate) status: ErrorCode,
    pub(crate) num_hci_command_packets: u8,
    pub(crate) opcode: CommandOpCode,
}

impl CommandStatusEvent {
    pub(crate) fn new(
        status: ErrorCode,
        num_hci_command_packets: u8,
        opcode: CommandOpCode,
    ) -> Self {
        Self {
            status,
            num_hci_command_packets,
            opcode,
        }
    }
}

pub(crate) mod parser {
    use nom::{
        combinator::{all_consuming, map},
        IResult, Parser,
    };

    use super::*;
    use crate::command::parser::command_opcode;
    use crate::event::parser::{hci_error_code, num_hci_command_packets};

    pub(crate) fn command_status_event(input: &[u8]) -> IResult<&[u8], CommandStatusEvent> {
        all_consuming(map(
            (hci_error_code, num_hci_command_packets, command_opcode),
            |(status, num_hci_command_packets, opcode)| {
                CommandStatusEvent::new(status, num_hci_command_packets, opcode)
            },
        ))
        .parse(input)
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;
    use crate::{
        packet::{parser::packet, Packet},
        Event,
    };

    #[rstest]
    #[case(CommandStatusEvent::new(ErrorCode::Success, 1, CommandOpCode::LeConnectionUpdate), &[4, 15, 4, 0, 1, 19, 32])]
    #[case(CommandStatusEvent::new(ErrorCode::Success, 1, CommandOpCode::LeCreateConnection), &[4, 15, 4, 0, 1, 13, 32])]
    #[case(CommandStatusEvent::new(ErrorCode::CommandDisallowed, 1, CommandOpCode::LeCreateConnection), &[4, 15, 4, 12, 1, 13, 32])]
    #[case(CommandStatusEvent::new(ErrorCode::Success, 1, CommandOpCode::Disconnect), &[4, 15, 4, 0, 1, 6, 4])]
    fn test_command_status_event_parsing_success(
        #[case] event: CommandStatusEvent,
        #[case] input: &[u8],
    ) {
        let (rest, packet) = packet(input).unwrap();
        assert_eq!(packet, Packet::Event(Event::CommandStatus(event)));
        assert!(rest.is_empty());
    }

    #[rstest]
    #[case::remaining_data(&[4, 15, 6, 0, 1, 13, 32, 0, 0])]
    #[case::not_enough_data(&[4, 15, 4, 0])]
    fn test_command_status_event_parsing_failure(#[case] input: &[u8]) {
        assert!(packet(input).is_err());
    }
}
