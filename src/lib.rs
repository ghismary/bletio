#![no_std]

extern crate alloc;

mod hci;

use core::cell::{BorrowMutError, RefCell};
use embedded_io::Error as EmbeddedIoError;

use hci::command::Command;

#[derive(Debug)]
pub enum Error {
    HciAccessDenied,
    IO(embedded_io::ErrorKind),
}

impl From<BorrowMutError> for Error {
    fn from(_value: BorrowMutError) -> Self {
        Self::HciAccessDenied
    }
}

pub struct BleStack<T>
where
    T: embedded_io::Read + embedded_io::Write,
{
    hci: RefCell<T>,
}

impl<T> BleStack<T>
where
    T: embedded_io::Read + embedded_io::Write,
    <T as embedded_io::ErrorType>::Error: embedded_io::Error,
{
    pub fn new(hci: T) -> Self {
        Self { hci: hci.into() }
    }

    pub fn init(&self) -> Result<(), Error> {
        self.hci_write(Command::Reset.encode().data())?;
        Ok(())
    }

    fn hci_write(&self, data: &[u8]) -> Result<usize, Error> {
        self.hci
            .try_borrow_mut()?
            .write(data)
            .map_err(|err| Error::IO(err.kind()))
    }
}
