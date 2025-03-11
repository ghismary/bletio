use num_enum::TryFromPrimitive;

use crate::{
    ConnectionHandle, ConnectionInterval, DeviceAddress, Error, ErrorCode, Latency,
    SupervisionTimeout,
};

/// Role in a connection.
///
/// See [Core Specification 6.0, Vol. 4, Part E, 7.7.65.1](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-bacd71f4-fabc-238d-72ee-f9aaaf5cbf22).
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[num_enum(error_type(name = Error, constructor = Error::InvalidRole))]
#[repr(u8)]
#[non_exhaustive]
pub enum Role {
    /// Connection is central.
    Central = 0x00,
    /// Connection is peripheral.
    Peripheral = 0x01,
}

/// Central clock accuracy in a connection.
///
/// See [Core Specification 6.0, Vol. 4, Part E, 7.7.65.1](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-bacd71f4-fabc-238d-72ee-f9aaaf5cbf22).
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[num_enum(error_type(name = Error, constructor = Error::InvalidCentralClockAccuracy))]
#[repr(u8)]
#[non_exhaustive]
pub enum CentralClockAccuracy {
    /// 500 ppm.
    Ppm500 = 0x00,
    /// 250 ppm.
    Ppm250 = 0x01,
    /// 150 ppm.
    Ppm150 = 0x02,
    /// 100 ppm.
    Ppm100 = 0x03,
    /// 75 ppm.
    Ppm75 = 0x04,
    /// 50 ppm.
    Ppm50 = 0x05,
    /// 30 ppm.
    Ppm30 = 0x06,
    /// 20 ppm.
    Ppm20 = 0x07,
}

/// LE Connection Complete Event.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.7.65.1](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-bacd71f4-fabc-238d-72ee-f9aaaf5cbf22).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LeConnectionCompleteEvent {
    pub(crate) status: ErrorCode,
    pub(crate) connection_handle: ConnectionHandle,
    pub(crate) role: Role,
    pub(crate) peer_address: DeviceAddress,
    pub(crate) connection_interval: ConnectionInterval,
    pub(crate) peripheral_latency: Latency,
    pub(crate) supervision_timeout: SupervisionTimeout,
    pub(crate) central_clock_accuracy: CentralClockAccuracy,
}

impl LeConnectionCompleteEvent {
    pub fn central_clock_accuracy(&self) -> CentralClockAccuracy {
        self.central_clock_accuracy
    }

    pub fn connection_handle(&self) -> ConnectionHandle {
        self.connection_handle
    }

    pub fn connection_interval(&self) -> ConnectionInterval {
        self.connection_interval
    }

    pub fn peer_address(&self) -> &DeviceAddress {
        &self.peer_address
    }

    pub fn peripheral_latency(&self) -> Latency {
        self.peripheral_latency
    }

    pub fn role(&self) -> Role {
        self.role
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
        combinator::{all_consuming, map, map_res},
        number::complete::le_u8,
        IResult, Parser,
    };

    use super::*;
    use crate::common::peer_address_type::parser::peer_address;
    use crate::connection::supervision_timeout::parser::supervision_timeout;
    use crate::connection::{
        connection_handle::parser::connection_handle,
        connection_interval::parser::connection_interval, latency::parser::latency,
    };
    use crate::event::parser::hci_error_code;
    use crate::LeMetaEvent;

    fn role(input: &[u8]) -> IResult<&[u8], Role> {
        map_res(le_u8, TryFrom::try_from).parse(input)
    }

    fn central_clock_accuracy(input: &[u8]) -> IResult<&[u8], CentralClockAccuracy> {
        map_res(le_u8, TryFrom::try_from).parse(input)
    }

    pub(crate) fn le_connection_complete_event(input: &[u8]) -> IResult<&[u8], LeMetaEvent> {
        map(
            all_consuming((
                hci_error_code,
                connection_handle,
                role,
                peer_address,
                connection_interval,
                latency,
                supervision_timeout,
                central_clock_accuracy,
            )),
            |(
                status,
                connection_handle,
                role,
                peer_address,
                connection_interval,
                peripheral_latency,
                supervision_timeout,
                central_clock_accuracy,
            )| {
                LeMetaEvent::LeConnectionComplete(LeConnectionCompleteEvent {
                    status,
                    connection_handle,
                    role,
                    peer_address,
                    connection_interval,
                    peripheral_latency,
                    supervision_timeout,
                    central_clock_accuracy,
                })
            },
        )
        .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        connection_interval, latency, supervision_timeout, RandomResolvablePrivateAddress,
    };

    #[test]
    fn test_le_connection_complete_event() {
        let central_clock_accuracy = CentralClockAccuracy::Ppm150;
        let connection_handle = ConnectionHandle::try_new(0).unwrap();
        let connection_interval = connection_interval!(64);
        let peer_address: DeviceAddress =
            RandomResolvablePrivateAddress::try_new([83, 251, 125, 93, 119, 88])
                .unwrap()
                .into();
        let peripheral_latency = latency!(0);
        let role = Role::Central;
        let status = ErrorCode::Success;
        let supervision_timeout = supervision_timeout!(32);
        let le_connection_complete_event = LeConnectionCompleteEvent {
            status,
            connection_handle,
            role,
            peer_address: peer_address.clone(),
            connection_interval,
            peripheral_latency,
            supervision_timeout,
            central_clock_accuracy,
        };
        assert_eq!(
            le_connection_complete_event.central_clock_accuracy(),
            central_clock_accuracy
        );
        assert_eq!(
            le_connection_complete_event.connection_handle(),
            connection_handle
        );
        assert_eq!(
            le_connection_complete_event.connection_interval(),
            connection_interval
        );
        assert_eq!(le_connection_complete_event.peer_address(), &peer_address);
        assert_eq!(
            le_connection_complete_event.peripheral_latency(),
            peripheral_latency
        );
        assert_eq!(le_connection_complete_event.role(), role);
        assert_eq!(le_connection_complete_event.status(), status);
        assert_eq!(
            le_connection_complete_event.supervision_timeout(),
            supervision_timeout
        );
    }
}
