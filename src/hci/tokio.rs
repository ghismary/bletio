extern crate std;

use std::future::Future;
use std::time::Duration;

use tokio::time::timeout;

use crate::hci::{HciDriverError, WithTimeout};

impl<F: Future> WithTimeout for F {
    type Output = F::Output;

    async fn with_timeout(self, timeout_ms: u16) -> Result<Self::Output, HciDriverError> {
        timeout(Duration::from_millis(timeout_ms as u64), self)
            .await
            .map_err(|_| HciDriverError::Timeout)
    }
}
