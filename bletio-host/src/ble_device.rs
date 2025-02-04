use bletio_hci::{Hci, HciDriver};

use crate::assigned_numbers::AppearanceValue;
use crate::ble_device_information::BleDeviceInformation;
use crate::{BleHost, BleHostObserver, Error};

#[derive(Debug)]
pub struct BleDeviceBuilder<H, O>
where
    H: HciDriver,
    O: BleHostObserver,
{
    hci: Hci<H>,
    observer: O,
    appearance: Option<AppearanceValue>,
}

impl<H, O> BleDeviceBuilder<H, O>
where
    H: HciDriver,
    O: BleHostObserver,
{
    pub fn build(self) -> BleDevice<H, O> {
        BleDevice {
            hci: self.hci,
            observer: self.observer,
            device_info: BleDeviceInformation {
                appearance: self.appearance.unwrap_or(AppearanceValue::GenericUnknown),
            },
        }
    }

    pub fn with_appearance(mut self, appearance: AppearanceValue) -> Self {
        self.appearance = Some(appearance);
        self
    }
}

pub struct BleDevice<H, O>
where
    H: HciDriver,
    O: BleHostObserver,
{
    hci: Hci<H>,
    observer: O,
    device_info: BleDeviceInformation,
}

impl<H, O> BleDevice<H, O>
where
    H: HciDriver,
    O: BleHostObserver,
{
    pub fn builder(hci_driver: H, observer: O) -> BleDeviceBuilder<H, O> {
        BleDeviceBuilder {
            hci: Hci::new(hci_driver),
            observer,
            appearance: Default::default(),
        }
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        let host = BleHost::setup(&mut self.hci, &self.device_info).await?;
        let _host = self.observer.ready(host).await;

        // todo!();

        Ok(())
    }
}
