use bletio_utils::Error as UtilsError;
use bletio_utils::{BufferOps, EncodeToBuffer};

use crate::{
    ConnectionEventLengthRange, ConnectionHandle, ConnectionIntervalRange, Error, Latency,
    SupervisionTimeout,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ConnectionUpdateParameters {
    connection_handle: ConnectionHandle,
    connection_interval_range: ConnectionIntervalRange,
    max_latency: Latency,
    supervision_timeout: SupervisionTimeout,
    connection_event_length_range: ConnectionEventLengthRange,
}

impl ConnectionUpdateParameters {
    pub fn try_new(
        connection_handle: ConnectionHandle,
        connection_interval_range: ConnectionIntervalRange,
        max_latency: Latency,
        supervision_timeout: SupervisionTimeout,
        connection_event_length_range: ConnectionEventLengthRange,
    ) -> Result<Self, Error> {
        if supervision_timeout.milliseconds()
            < ((1f32 + max_latency.value() as f32)
                * connection_interval_range.max().milliseconds()
                * 2f32)
        {
            Err(Error::SupervisionTimeoutIsNotBigEnough)
        } else {
            Ok(ConnectionUpdateParameters {
                connection_handle,
                connection_interval_range,
                max_latency,
                supervision_timeout,
                connection_event_length_range,
            })
        }
    }

    pub fn connection_handle(&self) -> &ConnectionHandle {
        &self.connection_handle
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

impl EncodeToBuffer for ConnectionUpdateParameters {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, UtilsError> {
        self.connection_handle.encode(buffer)?;
        self.connection_interval_range.encode(buffer)?;
        self.max_latency.encode(buffer)?;
        self.supervision_timeout.encode(buffer)?;
        self.connection_event_length_range.encode(buffer)?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        self.connection_handle.encoded_size()
            + self.connection_interval_range.encoded_size()
            + self.max_latency.encoded_size()
            + self.supervision_timeout.encoded_size()
            + self.connection_event_length_range.encoded_size()
    }
}

pub(crate) mod parser {
    use nom::combinator::{all_consuming, map};
    use nom::{IResult, Parser};

    use crate::connection::{
        connection_event_length::parser::connection_event_length_range,
        connection_handle::parser::connection_handle,
        connection_interval::parser::connection_interval_range, latency::parser::latency,
        supervision_timeout::parser::supervision_timeout,
    };

    use super::*;

    pub(crate) fn connection_update_parameters(
        input: &[u8],
    ) -> IResult<&[u8], ConnectionUpdateParameters> {
        all_consuming(map(
            (
                connection_handle,
                connection_interval_range,
                latency,
                supervision_timeout,
                connection_event_length_range,
            ),
            |(
                connection_handle,
                connection_interval_range,
                max_latency,
                supervision_timeout,
                connection_event_length_range,
            )| {
                ConnectionUpdateParameters {
                    connection_handle,
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
    use crate::{connection_interval_range, supervision_timeout};

    #[test]
    fn test_connection_update_parameters_success() {
        let connection_handle = ConnectionHandle::default();
        let connection_interval_range = ConnectionIntervalRange::default();
        let max_latency = Latency::default();
        let supervision_timeout = SupervisionTimeout::default();
        let connection_event_length_range = ConnectionEventLengthRange::default();
        let params = ConnectionUpdateParameters::try_new(
            connection_handle,
            connection_interval_range.clone(),
            max_latency,
            supervision_timeout,
            connection_event_length_range.clone(),
        )
        .unwrap();
        assert_eq!(params.connection_handle(), &connection_handle);
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
        connection_interval_range!(0x0030, 0x0050), Latency::default(), supervision_timeout!(0x0010),
        Error::SupervisionTimeoutIsNotBigEnough
    )]
    fn test_connection_parameters_failure(
        #[case] connection_interval_range: ConnectionIntervalRange,
        #[case] max_latency: Latency,
        #[case] supervision_timeout: SupervisionTimeout,
        #[case] expected_error: Error,
    ) {
        let err = ConnectionUpdateParameters::try_new(
            ConnectionHandle::default(),
            connection_interval_range,
            max_latency,
            supervision_timeout,
            ConnectionEventLengthRange::default(),
        );
        assert_eq!(err, Err(expected_error));
    }
}
