#![no_std]

use bletio_hci::Error as HciError;
use bletio_hci::HciDriverError;

pub mod advertising;
pub mod assigned_numbers;
pub mod ble_device;
pub mod ble_host;
pub mod uuid;

pub use ble_device::BleDevice;
pub use ble_host::{
    BleHost, BleHostObserver, BleHostState, BleHostStateAdvertising, BleHostStateInitial,
    BleHostStateStandby, BleHostStates,
};

mod device_information;

pub(crate) use device_information::DeviceInformation;

use advertising::AdvertisingError;

/// Errors that can happen during the BLE stack usage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Advertising related error.
    Advertising(AdvertisingError),
    /// The host is in a state where it cannot wait for an event.
    CannotWaitForEventInThisState,
    /// HCI related error.
    Hci(HciError),
    /// The Bluetooth controller is not LE capable.
    NonLeCapableController,
    /// The Random Static Device Address has already been created.
    RandomAddressAlreadyCreated,
}

impl From<AdvertisingError> for Error {
    fn from(value: AdvertisingError) -> Self {
        Self::Advertising(value)
    }
}

impl From<HciError> for Error {
    fn from(value: HciError) -> Self {
        Self::Hci(value)
    }
}

impl From<HciDriverError> for Error {
    fn from(value: HciDriverError) -> Self {
        Self::Hci(value.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_error_from_hci_driver_error() {
        let err: Error = HciDriverError::ReadFailure.into();
        assert_eq!(
            err,
            Error::Hci(HciError::HciDriver(HciDriverError::ReadFailure))
        );
    }
}
