use core::future::Future;
use core::time::Duration;

use crate::{HciDriverError, WithTimeout};

impl<F: Future> WithTimeout for F {
    type Output = F::Output;

    async fn with_timeout(
        self,
        timeout_duration: Duration,
    ) -> Result<Self::Output, HciDriverError> {
        embassy_time::with_timeout(
            embassy_time::Duration::from_millis(timeout_duration.as_millis() as u64),
            self,
        )
        .await
        .map_err(|_| HciDriverError::Timeout)
    }
}
