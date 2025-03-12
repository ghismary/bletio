use core::ops::{Deref, DerefMut};

use heapless::Vec;
use num_enum::{FromPrimitive, IntoPrimitive};

use crate::event::command_status::CommandStatusEvent;
use crate::{CommandCompleteEvent, LeMetaEvent};

pub(crate) mod command_complete;
pub(crate) mod command_status;
pub(crate) mod le_advertising_report;
pub(crate) mod le_connection_complete;
pub(crate) mod le_meta;

const EVENT_LIST_NB_EVENTS: usize = 4;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(clippy::large_enum_variant)]
pub enum Event {
    CommandComplete(CommandCompleteEvent),
    CommandStatus(CommandStatusEvent),
    LeMeta(LeMetaEvent),
    Unsupported(u8),
}

#[derive(Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct EventList {
    events: Vec<Event, EVENT_LIST_NB_EVENTS>,
}

impl Deref for EventList {
    type Target = Vec<Event, EVENT_LIST_NB_EVENTS>;

    fn deref(&self) -> &Self::Target {
        &self.events
    }
}

impl DerefMut for EventList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.events
    }
}

#[derive(Debug, IntoPrimitive, FromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
enum EventCode {
    CommandComplete = 0x0E,
    CommandStatus = 0x0F,
    LeMeta = 0x3E,
    #[num_enum(catch_all)]
    Unsupported(u8),
}

pub(crate) mod parser {
    use nom::{
        bytes::take, combinator::map_res, number::complete::le_u8, sequence::pair, IResult, Parser,
    };

    use super::*;
    use crate::event::command_status::parser::command_status_event;
    use crate::{
        event::{
            command_complete::parser::command_complete_event, le_meta::parser::le_meta_event,
            EventCode,
        },
        packet::parser::parameter_total_length,
        ErrorCode, Packet,
    };

    fn event_code(input: &[u8]) -> IResult<&[u8], EventCode> {
        map_res(le_u8, EventCode::try_from).parse(input)
    }

    pub(crate) fn num_hci_command_packets(input: &[u8]) -> IResult<&[u8], u8> {
        le_u8.parse(input)
    }

    pub(crate) fn hci_error_code(input: &[u8]) -> IResult<&[u8], ErrorCode> {
        map_res(le_u8, ErrorCode::try_from).parse(input)
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
                EventCode::CommandStatus => {
                    let (_, event) = command_status_event(parameters)?;
                    Event::CommandStatus(event)
                }
                EventCode::LeMeta => {
                    let (_, event) = le_meta_event(parameters)?;
                    Event::LeMeta(event)
                }
                EventCode::Unsupported(event_code) => Event::Unsupported(event_code),
            }),
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::{packet::parser::packet, Packet};

    use super::*;

    #[test]
    fn test_event_code() {
        let event_code: EventCode = 0x0Eu8.into();
        assert!(matches!(event_code, EventCode::CommandComplete));

        let event_code: EventCode = 0xFFu8.into();
        assert!(matches!(event_code, EventCode::Unsupported(0xFF)));
    }

    #[test]
    fn test_event_list() {
        let event_list = EventList::default();
        assert_eq!(event_list.deref(), &event_list.events);
    }

    #[test]
    fn test_unsupported_event_parsing() {
        // Using Inquiry Complete event
        let (rest, packet) = packet(&[4, 1, 1, 0]).unwrap();
        assert!(matches!(packet, Packet::Event(Event::Unsupported(1))));
        assert!(rest.is_empty());
    }
}
