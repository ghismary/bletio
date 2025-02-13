use bletio_hci::{Hci, HciDriver};

use crate::assigned_numbers::AppearanceValue;
use crate::{BleHost, BleHostObserver, Error};

#[derive(Debug)]
pub struct BleDeviceBuilder<'a, H, O>
where
    H: HciDriver,
    O: BleHostObserver,
{
    hci: Hci<H>,
    observer: O,
    appearance: Option<AppearanceValue>,
    local_name: Option<&'a str>,
}

impl<'a, H, O> BleDeviceBuilder<'a, H, O>
where
    H: HciDriver,
    O: BleHostObserver,
{
    pub fn build(self) -> BleDevice<'a, H, O> {
        BleDevice {
            hci: self.hci,
            observer: self.observer,
            appearance: self.appearance.unwrap_or(AppearanceValue::GenericUnknown),
            local_name: self.local_name.unwrap_or("bletio"),
        }
    }

    pub fn with_appearance(mut self, appearance: AppearanceValue) -> Self {
        self.appearance = Some(appearance);
        self
    }

    pub fn with_local_name(mut self, local_name: &'a str) -> Self {
        self.local_name = Some(local_name);
        self
    }
}

pub struct BleDevice<'a, H, O>
where
    H: HciDriver,
    O: BleHostObserver,
{
    hci: Hci<H>,
    observer: O,
    appearance: AppearanceValue,
    local_name: &'a str,
}

impl<'a, H, O> BleDevice<'a, H, O>
where
    H: HciDriver,
    O: BleHostObserver,
{
    pub fn builder(hci_driver: H, observer: O) -> BleDeviceBuilder<'a, H, O> {
        BleDeviceBuilder {
            hci: Hci::new(hci_driver),
            observer,
            appearance: Default::default(),
            local_name: Default::default(),
        }
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        let host = BleHost::setup(&mut self.hci, self.appearance, self.local_name).await?;
        let _host = self.observer.ready(host).await;

        // todo!();

        Ok(())
    }
}
