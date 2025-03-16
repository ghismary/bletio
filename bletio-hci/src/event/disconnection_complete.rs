use crate::{ConnectionHandle, ErrorCode};

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DisconnectionCompleteEvent {
    pub(crate) status: ErrorCode,
    pub(crate) connection_handle: ConnectionHandle,
    pub(crate) reason: ErrorCode,
}

pub(crate) mod parser {
    use nom::{combinator::map, IResult, Parser};

    use super::*;
    use crate::connection::connection_handle::parser::connection_handle;
    use crate::event::parser::hci_error_code;

    pub(crate) fn disconnection_complete_event(
        input: &[u8],
    ) -> IResult<&[u8], DisconnectionCompleteEvent> {
        map(
            (hci_error_code, connection_handle, hci_error_code),
            |(status, connection_handle, reason)| DisconnectionCompleteEvent {
                status,
                connection_handle,
                reason,
            },
        )
        .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::packet::parser::packet;
    use crate::packet::Packet;
    use crate::Event;

    use super::*;

    #[rstest]
    #[case(
        &[4, 5, 4, 0, 0, 0, 22],
        DisconnectionCompleteEvent {
            status: ErrorCode::Success,
            connection_handle: ConnectionHandle::try_new(0).unwrap(),
            reason: ErrorCode::ConnectionTerminatedByLocalHost
        }
    )]
    #[case(
        &[4, 5, 4, 0, 1, 0, 5],
        DisconnectionCompleteEvent {
            status: ErrorCode::Success,
            connection_handle: ConnectionHandle::try_new(1).unwrap(),
            reason: ErrorCode::AuthenticationFailure
        }
    )]
    fn test_disconnection_complete_event_parsing_success(
        #[case] input: &[u8],
        #[case] expected: DisconnectionCompleteEvent,
    ) {
        let (rest, packet) = packet(input).unwrap();
        assert_eq!(
            packet,
            Packet::Event(Event::DisconnectionComplete(expected))
        );
        assert!(rest.is_empty());
    }

    #[test]
    fn test_disconnection_complete_event_invalid_length() {
        let err = packet(&[4, 5, 3, 0, 0, 0]);
        assert!(err.is_err());
    }
}
