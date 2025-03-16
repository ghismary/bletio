use bletio_utils::Error as UtilsError;
use bletio_utils::{BufferOps, EncodeToBuffer};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{
    ConnectionEventLengthRange, ConnectionIntervalRange, ConnectionPeerAddress, Error, Latency,
    OwnAddressType, ScanInterval, ScanWindow, SupervisionTimeout,
};

/// Initiator filter policy to determine whether the Filter Accept List is used when creating a
/// connection.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.12](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-18ff009e-8e3a-a32c-160f-23e297c0fc9d).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[num_enum(error_type(name = Error, constructor = Error::InvalidInitiatorFilterPolicy))]
#[repr(u8)]
#[non_exhaustive]
pub enum InitiatorFilterPolicy {
    /// The Filter Accept List is not used to determine which advertiser to connect to (default).
    #[default]
    FilterAcceptListNotUsed = 0x00,
    /// The Filter Accept List is used to determine which advertiser to connect to.
    FilterAcceptListUsed = 0x01,
}

impl EncodeToBuffer for InitiatorFilterPolicy {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, bletio_utils::Error> {
        buffer.try_push((*self).into())
    }

    fn encoded_size(&self) -> usize {
        size_of::<InitiatorFilterPolicy>()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ConnectionParameters {
    scan_interval: ScanInterval,
    scan_window: ScanWindow,
    initiator_filter_policy: InitiatorFilterPolicy,
    peer_address: ConnectionPeerAddress,
    own_address_type: OwnAddressType,
    connection_interval_range: ConnectionIntervalRange,
    max_latency: Latency,
    supervision_timeout: SupervisionTimeout,
    connection_event_length_range: ConnectionEventLengthRange,
}

impl ConnectionParameters {
    #[allow(clippy::too_many_arguments)]
    pub fn try_new(
        scan_interval: ScanInterval,
        scan_window: ScanWindow,
        initiator_filter_policy: InitiatorFilterPolicy,
        peer_address: ConnectionPeerAddress,
        own_address_type: OwnAddressType,
        connection_interval_range: ConnectionIntervalRange,
        max_latency: Latency,
        supervision_timeout: SupervisionTimeout,
        connection_event_length_range: ConnectionEventLengthRange,
    ) -> Result<Self, Error> {
        if scan_window > scan_interval {
            Err(Error::ScanWindowMustBeSmallerOrEqualToScanInterval)
        } else if supervision_timeout.milliseconds()
            < ((1f32 + max_latency.value() as f32)
                * connection_interval_range.max().milliseconds()
                * 2f32)
        {
            Err(Error::SupervisionTimeoutIsNotBigEnough)
        } else {
            Ok(ConnectionParameters {
                scan_interval,
                scan_window,
                initiator_filter_policy,
                peer_address,
                own_address_type,
                connection_interval_range,
                max_latency,
                supervision_timeout,
                connection_event_length_range,
            })
        }
    }

    pub fn scan_interval(&self) -> ScanInterval {
        self.scan_interval
    }

    pub fn scan_window(&self) -> ScanWindow {
        self.scan_window
    }

    pub fn initiator_filter_policy(&self) -> InitiatorFilterPolicy {
        self.initiator_filter_policy
    }

    pub fn peer_address(&self) -> &ConnectionPeerAddress {
        &self.peer_address
    }

    pub fn own_address_type(&self) -> OwnAddressType {
        self.own_address_type
    }

    pub fn connection_interval_range(&self) -> &ConnectionIntervalRange {
        &self.connection_interval_range
    }

    pub fn max_latency(&self) -> Latency {
        self.max_latency
    }

    pub fn supervision_timeout(&self) -> SupervisionTimeout {
        self.supervision_timeout
    }

    pub fn connection_event_length_range(&self) -> &ConnectionEventLengthRange {
        &self.connection_event_length_range
    }
}

impl EncodeToBuffer for ConnectionParameters {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, UtilsError> {
        self.scan_interval.encode(buffer)?;
        self.scan_window.encode(buffer)?;
        self.initiator_filter_policy.encode(buffer)?;
        self.peer_address.encode(buffer)?;
        self.own_address_type.encode(buffer)?;
        self.connection_interval_range.encode(buffer)?;
        self.max_latency.encode(buffer)?;
        self.supervision_timeout.encode(buffer)?;
        self.connection_event_length_range.encode(buffer)?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        self.scan_interval.encoded_size()
            + self.scan_window.encoded_size()
            + self.initiator_filter_policy.encoded_size()
            + self.peer_address.encoded_size()
            + self.own_address_type.encoded_size()
            + self.connection_interval_range.encoded_size()
            + self.max_latency.encoded_size()
            + self.supervision_timeout.encoded_size()
            + self.connection_event_length_range.encoded_size()
    }
}

pub(crate) mod parser {
    use nom::combinator::{all_consuming, map};
    use nom::{combinator::map_res, number::complete::le_u8, IResult, Parser};

    use crate::common::own_address_type::parser::own_address_type;
    use crate::connection::{
        connection_event_length::parser::connection_event_length_range,
        connection_interval::parser::connection_interval_range,
        connection_peer_address::parser::connection_peer_address, latency::parser::latency,
        supervision_timeout::parser::supervision_timeout,
    };
    use crate::scanning::{scan_interval::parser::scan_interval, scan_window::parser::scan_window};

    use super::*;

    fn initiator_filter_policy(input: &[u8]) -> IResult<&[u8], InitiatorFilterPolicy> {
        map_res(le_u8, TryInto::try_into).parse(input)
    }

    pub(crate) fn connection_parameters(input: &[u8]) -> IResult<&[u8], ConnectionParameters> {
        all_consuming(map(
            (
                scan_interval,
                scan_window,
                initiator_filter_policy,
                connection_peer_address,
                own_address_type,
                connection_interval_range,
                latency,
                supervision_timeout,
                connection_event_length_range,
            ),
            |(
                scan_interval,
                scan_window,
                initiator_filter_policy,
                peer_address,
                own_address_type,
                connection_interval_range,
                max_latency,
                supervision_timeout,
                connection_event_length_range,
            )| {
                ConnectionParameters {
                    scan_interval,
                    scan_window,
                    initiator_filter_policy,
                    peer_address,
                    own_address_type,
                    connection_interval_range,
                    max_latency,
                    supervision_timeout,
                    connection_event_length_range,
                }
            },
        ))
        .parse(input)
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;
    use crate::{connection_interval_range, scan_interval, scan_window, supervision_timeout};

    #[test]
    fn test_connection_parameters_success() {
        let scan_interval = ScanInterval::default();
        let scan_window = ScanWindow::default();
        let initiator_filter_policy = InitiatorFilterPolicy::default();
        let peer_address = ConnectionPeerAddress::default();
        let own_address_type = OwnAddressType::default();
        let connection_interval_range = ConnectionIntervalRange::default();
        let max_latency = Latency::default();
        let supervision_timeout = SupervisionTimeout::default();
        let connection_event_length_range = ConnectionEventLengthRange::default();
        let params = ConnectionParameters::try_new(
            scan_interval,
            scan_window,
            initiator_filter_policy,
            peer_address.clone(),
            own_address_type,
            connection_interval_range.clone(),
            max_latency,
            supervision_timeout,
            connection_event_length_range.clone(),
        )
        .unwrap();
        assert_eq!(params.scan_interval(), scan_interval);
        assert_eq!(params.scan_window(), scan_window);
        assert_eq!(params.initiator_filter_policy(), initiator_filter_policy);
        assert_eq!(params.peer_address(), &peer_address);
        assert_eq!(params.own_address_type(), own_address_type);
        assert_eq!(
            params.connection_interval_range(),
            &connection_interval_range
        );
        assert_eq!(params.max_latency(), max_latency);
        assert_eq!(params.supervision_timeout(), supervision_timeout);
        assert_eq!(
            params.connection_event_length_range(),
            &connection_event_length_range
        );
    }

    #[rstest]
    #[case(
        scan_interval!(0x0020), scan_window!(0x0030), connection_interval_range!(0x0020, 0x0020), Latency::default(), SupervisionTimeout::default(),
        Error::ScanWindowMustBeSmallerOrEqualToScanInterval
    )]
    #[case(
        scan_interval!(0x0030), scan_window!(0x0020), connection_interval_range!(0x0030, 0x0050), Latency::default(), supervision_timeout!(0x0010),
        Error::SupervisionTimeoutIsNotBigEnough
    )]
    fn test_connection_parameters_failure(
        #[case] scan_interval: ScanInterval,
        #[case] scan_window: ScanWindow,
        #[case] connection_interval_range: ConnectionIntervalRange,
        #[case] max_latency: Latency,
        #[case] supervision_timeout: SupervisionTimeout,
        #[case] expected_error: Error,
    ) {
        let err = ConnectionParameters::try_new(
            scan_interval,
            scan_window,
            InitiatorFilterPolicy::default(),
            ConnectionPeerAddress::default(),
            OwnAddressType::default(),
            connection_interval_range,
            max_latency,
            supervision_timeout,
            ConnectionEventLengthRange::default(),
        );
        assert_eq!(err, Err(expected_error));
    }
}
