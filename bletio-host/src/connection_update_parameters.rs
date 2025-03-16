use core::ops::Deref;

use bletio_hci::{
    ConnectionEventLengthRange, ConnectionHandle, ConnectionIntervalRange, Latency,
    SupervisionTimeout,
};

use crate::Error;

/// Builder to create [`ConnectionUpdateParameters`].
#[derive(Debug, Default)]
pub struct ConnectionUpdateParametersBuilder {
    connection_handle: ConnectionHandle,
    connection_interval_range: ConnectionIntervalRange,
    max_latency: Latency,
    supervision_timeout: SupervisionTimeout,
    connection_event_length_range: ConnectionEventLengthRange,
}

impl ConnectionUpdateParametersBuilder {
    /// Create a builder to instantiate [`ConnectionUpdateParameters`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Try building the [`ConnectionUpdateParameters`], checking that every set parameters are valid.
    pub fn try_build(self) -> Result<ConnectionUpdateParameters, Error> {
        Ok(ConnectionUpdateParameters {
            inner: bletio_hci::ConnectionUpdateParameters::try_new(
                self.connection_handle,
                self.connection_interval_range,
                self.max_latency,
                self.supervision_timeout,
                self.connection_event_length_range,
            )
            .map_err(|_| Error::InvalidConnectionUpdateParameters)?,
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

    /// Define the connection handle of the connection to be updated.
    pub fn with_connection_handle(mut self, connection_handle: ConnectionHandle) -> Self {
        self.connection_handle = connection_handle;
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

    /// Define the max latency to be used.
    pub fn with_max_latency(mut self, max_latency: Latency) -> Self {
        self.max_latency = max_latency;
        self
    }

    /// Define the supervision timeout to be used.
    pub fn with_supervision_timeout(mut self, supervision_timeout: SupervisionTimeout) -> Self {
        self.supervision_timeout = supervision_timeout;
        self
    }
}

/// Connection update parameters to use to create a connection.
///
/// It contains this information:
///  - the connection handle
///  - the connection interval range
///  - the max latency
///  - the supervision timeout
///  - the connection event length range
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.18](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-93abb353-5b77-9ab0-096b-6d0e6052c788).
///
/// Use the [`ConnectionUpdateParametersBuilder`] to instantiate it.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ConnectionUpdateParameters {
    inner: bletio_hci::ConnectionUpdateParameters,
}

impl ConnectionUpdateParameters {
    /// Instantiate a builder to create Connection Parameters.
    pub fn builder() -> ConnectionUpdateParametersBuilder {
        ConnectionUpdateParametersBuilder::new()
    }
}

impl Deref for ConnectionUpdateParameters {
    type Target = bletio_hci::ConnectionUpdateParameters;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg(test)]
mod test {
    use bletio_hci::{
        connection_event_length_range, connection_interval_range, latency, supervision_timeout,
    };

    use super::*;

    #[test]
    fn test_default_connection_update_parameters() -> Result<(), Error> {
        let params = ConnectionUpdateParameters::builder().try_build()?;
        assert_eq!(
            params.deref(),
            &bletio_hci::ConnectionUpdateParameters::default()
        );
        Ok(())
    }

    #[test]
    fn test_valid_connection_update_parameters() -> Result<(), Error> {
        let connection_event_length_range = connection_event_length_range!(16, 16);
        let connection_interval_range = connection_interval_range!(16, 32);
        let max_latency = latency!(0);
        let supervision_timeout = supervision_timeout!(16);
        let params = ConnectionUpdateParameters::builder()
            .with_connection_event_length_range(connection_event_length_range.clone())
            .with_connection_handle(ConnectionHandle::default())
            .with_connection_interval_range(connection_interval_range.clone())
            .with_max_latency(max_latency)
            .with_supervision_timeout(supervision_timeout)
            .try_build()?;
        assert_eq!(
            params.connection_event_length_range(),
            &connection_event_length_range
        );
        assert_eq!(params.connection_handle().value(), 0);
        assert_eq!(
            params.connection_interval_range(),
            &connection_interval_range
        );
        assert_eq!(params.max_latency(), max_latency);
        assert_eq!(params.supervision_timeout(), supervision_timeout);
        Ok(())
    }

    #[test]
    fn test_invalid_connection_update_parameters_supervision_timeout_too_short() {
        let err = ConnectionUpdateParameters::builder()
            .with_connection_interval_range(connection_interval_range!(0x0030, 0x0050))
            .with_supervision_timeout(supervision_timeout!(0x0010))
            .try_build();
        assert_eq!(err, Err(Error::InvalidConnectionUpdateParameters));
    }
}
