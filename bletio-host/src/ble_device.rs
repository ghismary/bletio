use bletio_hci::{
    ConnectionPeerAddress, Event, EventList, Hci, HciDriver, LeAdvertisingReport,
    LeAdvertisingReportEventType, LeConnectionCompleteEvent, LeMetaEvent,
};

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
                Ok(event_list) => {
                    // Specific handling for LE advertising reports that needs to be grouped together.
                    if event_list
                        .iter()
                        .any(|e| matches!(e, Event::LeMeta(LeMetaEvent::LeAdvertisingReport(_))))
                    {
                        host = self.notify_le_advertising_reports(host, &event_list).await;
                    }

                    // Handling of other events, ignoring the previously handled LE advertising reports.
                    for event in event_list.iter().filter(|e| {
                        !matches!(e, Event::LeMeta(LeMetaEvent::LeAdvertisingReport(_)))
                    }) {
                        if let Event::LeMeta(LeMetaEvent::LeConnectionComplete(
                            le_connection_complete_event,
                        )) = event
                        {
                            host = self
                                .notify_le_connection_complete(host, le_connection_complete_event)
                                .await?;
                        }
                    }
                }
                Err(Error::Hci(bletio_hci::Error::InvalidPacket)) => {
                    // Ignore invalid HCI packet
                    #[cfg(feature = "defmt")]
                    defmt::warn!("Received invalid HCI packet");
                }
                Err(e) => return Err(e),
            }
        }
    }

    pub async fn notify_le_connection_complete<H>(
        &self,
        mut host: BleHostStates<'a, H>,
        event: &LeConnectionCompleteEvent,
    ) -> Result<BleHostStates<H>, Error>
    where
        H: HciDriver,
    {
        if event.status().is_success() {
            host = match host {
                BleHostStates::Initiating(h) => BleHostStates::ConnectedCentral(h.change_state()),
                BleHostStates::Advertising(h) => {
                    BleHostStates::ConnectedPeripheral(h.stop_advertising().await?.change_state())
                }
                _host => _host,
            }
        }

        Ok(self.observer.connection_complete(host, event).await)
    }

    pub async fn notify_le_advertising_reports<'e, H>(
        &self,
        mut host: BleHostStates<'a, H>,
        event_list: &'e EventList,
    ) -> BleHostStates<'a, H>
    where
        H: HciDriver,
    {
        fn find_corresponding_scan_response(
            event_list: &EventList,
            address: &ConnectionPeerAddress,
        ) -> Option<LeAdvertisingReport> {
            for reports in event_list.iter().filter_map(|e| match e {
                Event::LeMeta(LeMetaEvent::LeAdvertisingReport(reports)) => Some(reports),
                _ => None,
            }) {
                if let Some(report) = reports.iter().find(|r| {
                    (r.event_type() == LeAdvertisingReportEventType::ScanResponse)
                        && r.address() == address
                }) {
                    return Some(report);
                }
            }

            None
        }

        for reports in event_list.iter().filter_map(|e| match e {
            Event::LeMeta(LeMetaEvent::LeAdvertisingReport(reports)) => Some(reports),
            _ => None,
        }) {
            for report in reports
                .iter()
                .filter(|r| r.event_type() != LeAdvertisingReportEventType::ScanResponse)
            {
                let adv_data = report.data().into();
                let scanresp_report =
                    find_corresponding_scan_response(event_list, report.address());
                let scanresp_data = scanresp_report.map(|r| r.data().into());
                let full_adv_data = FullAdvertisingData::try_new(adv_data, scanresp_data).unwrap();
                host = self
                    .observer
                    .advertising_report_received(
                        host,
                        report.event_type(),
                        report.address(),
                        report.rssi(),
                        full_adv_data,
                    )
                    .await;
            }
        }

        host
    }
}
