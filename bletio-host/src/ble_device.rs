use bletio_hci::{Event, Hci, HciDriver, LeAdvertisingReportList, LeMetaEvent};

use crate::advertising::FullAdvertisingData;
use crate::assigned_numbers::AppearanceValue;
use crate::{BleHost, BleHostObserver, BleHostStates, Error};

#[derive(Debug)]
pub struct BleDeviceBuilder<'a, O>
where
    O: BleHostObserver,
{
    observer: O,
    appearance: Option<AppearanceValue>,
    local_name: Option<&'a str>,
}

impl<'a, O> BleDeviceBuilder<'a, O>
where
    O: BleHostObserver,
{
    pub fn build(self) -> BleDevice<'a, O> {
        BleDevice {
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

pub struct BleDevice<'a, O>
where
    O: BleHostObserver,
{
    observer: O,
    appearance: AppearanceValue,
    local_name: &'a str,
}

impl<'a, O> BleDevice<'a, O>
where
    O: BleHostObserver,
{
    pub fn builder(observer: O) -> BleDeviceBuilder<'a, O> {
        BleDeviceBuilder {
            observer,
            appearance: Default::default(),
            local_name: Default::default(),
        }
    }

    pub async fn run<H>(&mut self, hci_driver: H) -> Result<(), Error>
    where
        H: HciDriver,
    {
        let host = BleHost::setup(Hci::new(hci_driver), self.appearance, self.local_name).await?;
        let mut host = self.observer.ready(host).await;

        loop {
            match host.wait_for_event().await {
                Ok(event) => {
                    if let Event::LeMeta(LeMetaEvent::LeAdvertisingReport(reports)) = event {
                        host = self.notify_le_advertising_reports(host, reports).await;
                    }
                }
                Err(Error::Hci(bletio_hci::Error::InvalidPacket)) => (), // Ignore invalid packet
                Err(e) => return Err(e),
            }
        }
    }

    pub async fn notify_le_advertising_reports<H>(
        &self,
        mut host: BleHostStates<'a, H>,
        reports: LeAdvertisingReportList,
    ) -> BleHostStates<'a, H>
    where
        H: HciDriver,
    {
        for report in reports.iter() {
            host = self
                .observer
                .advertising_report_received(
                    host,
                    report.event_type(),
                    report.address(),
                    report.rssi(),
                    FullAdvertisingData::default(),
                )
                .await;
        }
        host
    }
}
