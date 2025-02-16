use core::future::Future;
use core::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HciDriverError {
    WriteFailure,
    ReadFailure,
    Timeout,
}

pub trait HciDriver {
    fn read(&mut self, buf: &mut [u8]) -> impl Future<Output = Result<usize, HciDriverError>>;
    fn write(&mut self, buf: &[u8]) -> impl Future<Output = Result<usize, HciDriverError>>;
}

pub trait WithTimeout {
    type Output;

    fn with_timeout(
        self,
        timeout_duration: Duration,
    ) -> impl Future<Output = Result<Self::Output, HciDriverError>>;
}
