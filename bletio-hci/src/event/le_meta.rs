use num_enum::{FromPrimitive, IntoPrimitive};

use crate::LeAdvertisingReportList;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(clippy::large_enum_variant)]
pub enum LeMetaEvent {
    LeAdvertisingReport(LeAdvertisingReportList),
    Unsupported(u8),
}

#[derive(Debug, IntoPrimitive, FromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
enum LeMetaEventCode {
    LeAdvertisingReport = 0x02,
    #[num_enum(catch_all)]
    Unsupported(u8),
}

pub(crate) mod parser {
    use nom::{combinator::map_res, number::le_u8, IResult, Parser};

    use crate::event::le_advertising_report::parser::le_advertising_report_event;

    use super::*;

    fn le_meta_event_code(input: &[u8]) -> IResult<&[u8], LeMetaEventCode> {
        map_res(le_u8(), LeMetaEventCode::try_from).parse(input)
    }

    pub(crate) fn le_meta_event(input: &[u8]) -> IResult<&[u8], LeMetaEvent> {
        let (parameters, le_meta_event_code) = le_meta_event_code(input)?;
        match le_meta_event_code {
            LeMetaEventCode::LeAdvertisingReport => le_advertising_report_event(parameters),
            LeMetaEventCode::Unsupported(event_code) => {
                Ok((&[], LeMetaEvent::Unsupported(event_code)))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{packet::parser::packet, Event, LeMetaEvent, Packet};

    #[test]
    fn test_unsupported_le_meta_event_parsing() {
        // Using LE Monitored Advertisers Report event
        let (rest, packet) = packet(&[
            0x04, 0x3E, 0x09, 0x34, 0x00, 0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56, 0x00,
        ])
        .unwrap();
        assert!(matches!(
            packet,
            Packet::Event(Event::LeMeta(LeMetaEvent::Unsupported(0x34)))
        ));
        assert!(rest.is_empty());
    }
}
