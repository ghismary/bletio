use core::ops::Deref;

use bletio_hci::{
    ConnectionEventLengthRange, ConnectionIntervalRange, ConnectionPeerAddress,
    InitiatorFilterPolicy, MaxLatency, OwnAddressType, ScanInterval, ScanWindow,
    SupervisionTimeout,
};

use crate::Error;

/// Builder to create [`ConnectionParameters`].
#[derive(Debug, Default)]
pub struct ConnectionParametersBuilder {
    scan_interval: ScanInterval,
    scan_window: ScanWindow,
    initiator_filter_policy: InitiatorFilterPolicy,
    peer_address: ConnectionPeerAddress,
    own_address_type: OwnAddressType,
    connection_interval_range: ConnectionIntervalRange,
    max_latency: MaxLatency,
    supervision_timeout: SupervisionTimeout,
    connection_event_length_range: ConnectionEventLengthRange,
}

impl ConnectionParametersBuilder {
    /// Create a builder to instantiate [`ConnectionParameters`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Try building the [`ConnectionParameters`], checking that every set parameters are valid.
    pub fn try_build(self) -> Result<ConnectionParameters, Error> {
        Ok(ConnectionParameters {
            inner: bletio_hci::ConnectionParameters::try_new(
                self.scan_interval,
                self.scan_window,
                self.initiator_filter_policy,
                self.peer_address,
                self.own_address_type,
                self.connection_interval_range,
                self.max_latency,
                self.supervision_timeout,
                self.connection_event_length_range,
            )
            .map_err(|_| Error::InvalidConnectionParameters)?,
        })
    }

    /// Define the connection event length range to be used.
    pub fn with_connection_event_length_range(
        mut self,
        connection_event_length_range: ConnectionEventLengthRange,
    ) -> Self {
        self.connection_event_length_range = connection_event_length_range;
        self
    }

    /// Define the connection interval range to be used.
    pub fn with_connection_interval_range(
        mut self,
        connection_interval_range: ConnectionIntervalRange,
    ) -> Self {
        self.connection_interval_range = connection_interval_range;
        self
    }

    /// Define the initiator filter policy to be used.
    pub fn with_initiator_filter_policy(
        mut self,
        initiator_filter_policy: InitiatorFilterPolicy,
    ) -> Self {
        self.initiator_filter_policy = initiator_filter_policy;
        self
    }

    /// Define the max latency to be used.
    pub fn with_max_latency(mut self, max_latency: MaxLatency) -> Self {
        self.max_latency = max_latency;
        self
    }

    /// Define our own address type.
    pub fn with_own_address_type(mut self, own_address_type: OwnAddressType) -> Self {
        self.own_address_type = own_address_type;
        self
    }

    /// Define the peer address to connect to.
    pub fn with_peer_address(mut self, peer_address: ConnectionPeerAddress) -> Self {
        self.peer_address = peer_address;
        self
    }

    /// Define the scan interval to be used.
    pub fn with_scan_interval(mut self, scan_interval: ScanInterval) -> Self {
        self.scan_interval = scan_interval;
        self
    }

    /// Define the scan window to be used.
    pub fn with_scan_window(mut self, scan_window: ScanWindow) -> Self {
        self.scan_window = scan_window;
        self
    }

    /// Define the supervision timeout to be used.
    pub fn with_supervision_timeout(mut self, supervision_timeout: SupervisionTimeout) -> Self {
        self.supervision_timeout = supervision_timeout;
        self
    }
}

/// Connection parameters to use to create a connection.
///
/// It contains this information:
///  - the scan interval
///  - the scan window
///  - the initiator filter policy
///  - the peer address type
///  - the peer address
///  - our own address type
///  - the connection interval range
///  - the max latency
///  - the supervision timeout
///  - the connection event length range
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.12](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-18ff009e-8e3a-a32c-160f-23e297c0fc9d).
///
/// Use the [`ConnectionParametersBuilder`] to instantiate it.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ConnectionParameters {
    inner: bletio_hci::ConnectionParameters,
}

impl ConnectionParameters {
    /// Instantiate a builder to create Connection Parameters.
    pub fn builder() -> ConnectionParametersBuilder {
        ConnectionParametersBuilder::new()
    }
}

impl Deref for ConnectionParameters {
    type Target = bletio_hci::ConnectionParameters;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg(test)]
mod test {
    use bletio_hci::{
        connection_event_length_range, connection_interval_range, max_latency, scan_interval,
        scan_window, supervision_timeout,
    };

    use super::*;

    #[test]
    fn test_default_connection_parameters() -> Result<(), Error> {
        let params = ConnectionParameters::builder().try_build()?;
        assert_eq!(params.deref(), &bletio_hci::ConnectionParameters::default());
        Ok(())
    }

    #[test]
    fn test_valid_connection_parameters() -> Result<(), Error> {
        let connection_event_length_range = connection_event_length_range!(16, 16);
        let connection_interval_range = connection_interval_range!(16, 32);
        let initiator_filter_policy = InitiatorFilterPolicy::FilterAcceptListNotUsed;
        let max_latency = max_latency!(0);
        let own_address_type = OwnAddressType::PublicDeviceAddress;
        let peer_address =
            ConnectionPeerAddress::PublicDevice([0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56].into());
        let scan_interval = scan_interval!(16);
        let scan_window = scan_window!(10);
        let supervision_timeout = supervision_timeout!(16);
        let params = ConnectionParameters::builder()
            .with_connection_event_length_range(connection_event_length_range.clone())
            .with_connection_interval_range(connection_interval_range.clone())
            .with_initiator_filter_policy(initiator_filter_policy)
            .with_max_latency(max_latency)
            .with_own_address_type(own_address_type)
            .with_peer_address(peer_address.clone())
            .with_scan_interval(scan_interval)
            .with_scan_window(scan_window)
            .with_supervision_timeout(supervision_timeout)
            .try_build()?;
        assert_eq!(
            params.connection_event_length_range(),
            &connection_event_length_range
        );
        assert_eq!(
            params.connection_interval_range(),
            &connection_interval_range
        );
        assert_eq!(params.initiator_filter_policy(), initiator_filter_policy);
        assert_eq!(params.max_latency(), max_latency);
        assert_eq!(params.own_address_type(), own_address_type);
        assert_eq!(params.peer_address(), &peer_address);
        assert_eq!(params.scan_interval(), scan_interval);
        assert_eq!(params.scan_window(), scan_window);
        assert_eq!(params.supervision_timeout(), supervision_timeout);
        Ok(())
    }

    #[test]
    fn test_invalid_connection_parameters_scan_window_larger_than_scan_interval() {
        let err = ConnectionParameters::builder()
            .with_scan_interval(scan_interval!(10))
            .with_scan_window(scan_window!(20))
            .try_build();
        assert_eq!(err, Err(Error::InvalidConnectionParameters));
    }

    #[test]
    fn test_invalid_connection_parameters_supervision_timeout_too_short() {
        let err = ConnectionParameters::builder()
            .with_connection_interval_range(connection_interval_range!(0x0030, 0x0050))
            .with_scan_interval(scan_interval!(30))
            .with_scan_window(scan_window!(20))
            .with_supervision_timeout(supervision_timeout!(0x0010))
            .try_build();
        assert_eq!(err, Err(Error::InvalidConnectionParameters));
    }
}
