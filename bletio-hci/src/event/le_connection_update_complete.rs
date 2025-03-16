use crate::{ConnectionHandle, ConnectionInterval, ErrorCode, Latency, SupervisionTimeout};

/// LE Connection Update Complete Event.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.7.65.3](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-bacd71f4-fabc-238d-72ee-f9aaaf5cbf22).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LeConnectionUpdateCompleteEvent {
    pub(crate) status: ErrorCode,
    pub(crate) connection_handle: ConnectionHandle,
    pub(crate) connection_interval: ConnectionInterval,
    pub(crate) peripheral_latency: Latency,
    pub(crate) supervision_timeout: SupervisionTimeout,
}

impl LeConnectionUpdateCompleteEvent {
    pub fn connection_handle(&self) -> ConnectionHandle {
        self.connection_handle
    }

    pub fn connection_interval(&self) -> ConnectionInterval {
        self.connection_interval
    }

    pub fn peripheral_latency(&self) -> Latency {
        self.peripheral_latency
    }

    pub fn status(&self) -> ErrorCode {
        self.status
    }

    pub fn supervision_timeout(&self) -> SupervisionTimeout {
        self.supervision_timeout
    }
}

pub(crate) mod parser {
    use nom::{
        combinator::{all_consuming, map},
        IResult, Parser,
    };

    use super::*;
    use crate::connection::supervision_timeout::parser::supervision_timeout;
    use crate::connection::{
        connection_handle::parser::connection_handle,
        connection_interval::parser::connection_interval, latency::parser::latency,
    };
    use crate::event::parser::hci_error_code;
    use crate::LeMetaEvent;

    pub(crate) fn le_connection_update_complete_event(input: &[u8]) -> IResult<&[u8], LeMetaEvent> {
        map(
            all_consuming((
                hci_error_code,
                connection_handle,
                connection_interval,
                latency,
                supervision_timeout,
            )),
            |(
                status,
                connection_handle,
                connection_interval,
                peripheral_latency,
                supervision_timeout,
            )| {
                LeMetaEvent::LeConnectionUpdateComplete(LeConnectionUpdateCompleteEvent {
                    status,
                    connection_handle,
                    connection_interval,
                    peripheral_latency,
                    supervision_timeout,
                })
            },
        )
        .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{connection_interval, latency, supervision_timeout};

    #[test]
    fn test_le_connection_update_complete_event() {
        let connection_handle = ConnectionHandle::try_new(0).unwrap();
        let connection_interval = connection_interval!(64);
        let peripheral_latency = latency!(0);
        let status = ErrorCode::Success;
        let supervision_timeout = supervision_timeout!(32);
        let le_connection_update_complete_event = LeConnectionUpdateCompleteEvent {
            status,
            connection_handle,
            connection_interval,
            peripheral_latency,
            supervision_timeout,
        };
        assert_eq!(
            le_connection_update_complete_event.connection_handle(),
            connection_handle
        );
        assert_eq!(
            le_connection_update_complete_event.connection_interval(),
            connection_interval
        );
        assert_eq!(
            le_connection_update_complete_event.peripheral_latency(),
            peripheral_latency
        );
        assert_eq!(le_connection_update_complete_event.status(), status);
        assert_eq!(
            le_connection_update_complete_event.supervision_timeout(),
            supervision_timeout
        );
    }
}
