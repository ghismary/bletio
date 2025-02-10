use core::future::Future;

#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum HciDriverError {
    #[error("HCI driver write failure")]
    WriteFailure,
    #[error("HCI driver read failure")]
    ReadFailure,
    #[error("HCI driver timeout")]
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
        timeout: u16,
    ) -> impl Future<Output = Result<Self::Output, HciDriverError>>;
}
