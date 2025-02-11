extern crate std;

use core::time::Duration;
use std::future::Future;

use tokio::time::timeout;

use crate::{HciDriverError, WithTimeout};

impl<F: Future> WithTimeout for F {
    type Output = F::Output;

    async fn with_timeout(
        self,
        timeout_duration: Duration,
    ) -> Result<Self::Output, HciDriverError> {
        timeout(timeout_duration, self)
            .await
            .map_err(|_| HciDriverError::Timeout)
    }
}

#[cfg(test)]
mod test {
    use tokio::time::sleep;

    use super::*;

    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_with_timeout_not_triggered() {
        assert!(sleep(Duration::from_millis(500))
            .with_timeout(Duration::from_millis(1000))
            .await
            .is_ok())
    }

    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_with_timeout_triggered() {
        let err = sleep(Duration::from_millis(1000))
            .with_timeout(Duration::from_millis(500))
            .await;
        assert!(matches!(err, Err(HciDriverError::Timeout)));
    }
}
