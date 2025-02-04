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

mod ble_device_information;
mod controller_capabilities;

use advertising::AdvertisingError;

/// Errors that can happen during the BLE stack usage.
#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Advertising related error.
    #[error(transparent)]
    Advertising(#[from] AdvertisingError),
    /// HCI related error.
    #[error(transparent)]
    Hci(#[from] HciError),
    /// The Bluetooth controller is not LE capable.
    #[error("The Bluetooth controller is not LE capable")]
    NonLeCapableController,
}

impl From<HciDriverError> for Error {
    fn from(value: HciDriverError) -> Self {
        Self::Hci(value.into())
    }
}
