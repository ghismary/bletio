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
    use nom::{
        combinator::{fail, map_res},
        number::le_u8,
        IResult, Parser,
    };

    use crate::event::le_advertising_report::parser::le_advertising_report_event;

    use super::*;

    fn le_meta_event_code(input: &[u8]) -> IResult<&[u8], LeMetaEventCode> {
        map_res(le_u8(), LeMetaEventCode::try_from).parse(input)
    }

    pub(crate) fn le_meta_event(input: &[u8]) -> IResult<&[u8], LeMetaEvent> {
        let (parameters, le_meta_event_code) = le_meta_event_code(input)?;
        match le_meta_event_code {
            LeMetaEventCode::LeAdvertisingReport => le_advertising_report_event(parameters),
            LeMetaEventCode::Unsupported(_) => {
                fail::<_, &[u8], _>().parse(parameters)?;
                unreachable!("the fail parser will systematically return an error")
            }
        }
    }
}
