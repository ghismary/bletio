#![no_std]

use bitflags::Flags;
use core::marker::PhantomData;

pub mod advertising;
pub mod assigned_numbers;
mod connection_interval_value;
pub mod hci;
pub mod le_states;
mod supported_le_features;
mod utils;
pub mod uuid;

pub use connection_interval_value::ConnectionIntervalValue;
pub use supported_le_features::SupportedLeFeatures;

use advertising::advertising_parameters::AdvertisingParameters;
use advertising::AdvertisingError;
use advertising::{AdvertisingData, AdvertisingEnable, ScanResponseData};
use hci::event_mask::EventMask;
use hci::supported_commands::SupportedCommands;
use hci::supported_le_states::SupportedLeStates;
use hci::{Hci, HciDriverError, HciError, SupportedFeatures};

/// Errors that can happen during the BLE stack usage.
#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Advertising related error.
    #[error(transparent)]
    Advertising(#[from] AdvertisingError),
    /// HCI related error.
    #[error(transparent)]
    Hci(#[from] HciError),
    /// IO error.
    /// Invalid connection interval value.
    #[error(
        "The connection interval value {0} is invalid, it needs to be between 0x0006 and 0x0C80"
    )]
    InvalidConnectionIntervalValue(u16), // TODO: Put somewhere else, in Advertising?
    /// The Bluetooth controller is not LE capable.
    #[error("The Bluetooth controller is not LE capable")]
    NonLeCapableController,
}

impl From<HciDriverError> for Error {
    fn from(value: HciDriverError) -> Self {
        Self::Hci(value.into())
    }
}

#[derive(Debug)]
struct ControllerCapabilities {
    supported_commands: SupportedCommands,
    supported_features: SupportedFeatures,
    supported_le_features: SupportedLeFeatures,
    supported_le_states: SupportedLeStates,
    le_data_packet_length: u16,
    num_le_data_packets: u16,
}

impl Default for ControllerCapabilities {
    fn default() -> Self {
        Self {
            supported_commands: SupportedCommands::default(),
            supported_features: SupportedFeatures::default(),
            supported_le_features: SupportedLeFeatures::default(),
            supported_le_states: SupportedLeStates::default(),
            le_data_packet_length: 255,
            num_le_data_packets: 1,
        }
    }
}

pub trait HostState {}

pub struct BleHost<'a, H, State: HostState = Initial>
where
    H: hci::HciDriver,
{
    controller_capabilities: ControllerCapabilities,
    hci: &'a mut Hci<H>,
    phantom: PhantomData<State>,
}

#[derive(Debug, Default)]
pub struct Initial;
#[derive(Debug, Default)]
pub struct Standby;
#[derive(Debug, Default)]
pub struct Advertising;

impl HostState for Initial {}
impl HostState for Standby {}
impl HostState for Advertising {}

impl<'a, H> BleHost<'a, H, Initial>
where
    H: hci::HciDriver,
{
    // Perform setup has described in Core specification 4.2, Vol. 6, Part D, 2.1
    async fn setup(hci: &'a mut Hci<H>) -> Result<BleHost<'a, H, Standby>, Error>
    where
        H: hci::HciDriver,
    {
        let mut controller_capabilities = ControllerCapabilities::default();

        hci.cmd_reset().await?;

        controller_capabilities.supported_commands =
            hci.cmd_read_local_supported_commands().await?;
        controller_capabilities.supported_features =
            hci.cmd_read_local_supported_features().await?;
        if !controller_capabilities
            .supported_features
            .contains(SupportedFeatures::LE_SUPPORTED_CONTROLLER)
        {
            return Err(Error::NonLeCapableController);
        }

        let event_mask = EventMask::HARDWARE_ERROR
            | EventMask::DATA_BUFFER_OVERFLOW
            | EventMask::LE_META
            | EventMask::DISCONNECTION_COMPLETE
            | EventMask::READ_REMOTE_VERSION_INFORMATION_COMLETE
            | EventMask::ENCRYPTION_CHANGE
            | EventMask::ENCRYPTION_KEY_REFRESH_COMPLETE;
        hci.cmd_set_event_mask(event_mask).await?;
        // TODO: set LE event mask

        (
            controller_capabilities.le_data_packet_length,
            controller_capabilities.num_le_data_packets,
        ) = hci.cmd_le_read_buffer_size().await?;
        if (controller_capabilities.le_data_packet_length == 0)
            || (controller_capabilities.num_le_data_packets == 0)
                && controller_capabilities
                    .supported_commands
                    .contains(SupportedCommands::READ_BUFFER_SIZE)
        {
            (
                controller_capabilities.le_data_packet_length,
                _,
                controller_capabilities.num_le_data_packets,
                _,
            ) = hci.cmd_read_buffer_size().await?;
        }
        if controller_capabilities
            .supported_commands
            .contains(SupportedCommands::LE_READ_LOCAL_SUPPORTED_FEATURES_PAGE_0)
        {
            controller_capabilities.supported_le_features =
                hci.cmd_le_read_local_supported_features_page_0().await?;
        }
        controller_capabilities.supported_le_states = hci.cmd_le_read_supported_states().await?;

        Ok(BleHost::<H, Standby> {
            controller_capabilities,
            hci,
            phantom: PhantomData,
        })
    }
}

impl<'a, H> BleHost<'a, H, Standby>
where
    H: hci::HciDriver,
{
    pub async fn start_advertising(
        self,
        adv_params: &AdvertisingParameters,
        adv_data: &AdvertisingData,
        scanresp_data: Option<&ScanResponseData>,
    ) -> Result<BleHost<'a, H, Advertising>, (Error, Self)> {
        async fn inner<H>(
            hci: &mut Hci<H>,
            adv_params: &AdvertisingParameters,
            adv_data: &AdvertisingData,
            scanresp_data: Option<&ScanResponseData>,
        ) -> Result<(), Error>
        where
            H: hci::HciDriver,
        {
            hci.cmd_le_set_advertising_parameters(adv_params).await?;
            // TODO: Read Advertising Channel Tx Power
            hci.cmd_le_set_advertising_data(adv_data).await?;
            let default_scanresp_data = ScanResponseData::default();
            hci.cmd_le_set_scan_response_data(match scanresp_data {
                Some(scanresp_data) => scanresp_data,
                None => &default_scanresp_data,
            })
            .await?;
            hci.cmd_le_set_advertising_enable(AdvertisingEnable::Enabled)
                .await?;
            Ok(())
        }
        match inner(self.hci, adv_params, adv_data, scanresp_data).await {
            Ok(()) => Ok(BleHost::<H, Advertising> {
                controller_capabilities: self.controller_capabilities,
                hci: self.hci,
                phantom: PhantomData,
            }),
            Err(e) => Err((e, self)),
        }
    }
}

impl<'a, H> BleHost<'a, H, Advertising>
where
    H: hci::HciDriver,
{
    pub async fn stop_advertising(self) -> Result<BleHost<'a, H, Standby>, Error> {
        // self.hci
        //     .cmd_le_set_advertise_enable(AdvertisingEnable::Disabled)
        //     .await?;
        Ok(BleHost::<H, Standby> {
            controller_capabilities: self.controller_capabilities,
            hci: self.hci,
            phantom: PhantomData,
        })
    }
}

impl<H> BleHost<'_, H>
where
    H: hci::HciDriver,
{
    pub fn supported_commands(&self) -> &SupportedCommands {
        &self.controller_capabilities.supported_commands
    }

    pub fn supported_features(&self) -> &SupportedFeatures {
        &self.controller_capabilities.supported_features
    }

    pub fn supported_le_features(&self) -> &SupportedLeFeatures {
        &self.controller_capabilities.supported_le_features
    }

    pub fn supported_le_states(&self) -> &SupportedLeStates {
        &self.controller_capabilities.supported_le_states
    }
}

pub enum BleHostEnum<'a, H>
where
    H: hci::HciDriver,
{
    Initial(BleHost<'a, H, Initial>),
    Standby(BleHost<'a, H, Standby>),
    Advertising(BleHost<'a, H, Advertising>),
}

pub trait BleHostObserver {
    fn ready<'a, H>(
        &self,
        host: BleHost<'a, H, Standby>,
    ) -> impl core::future::Future<Output = BleHostEnum<'a, H>>
    where
        H: hci::HciDriver,
    {
        async { BleHostEnum::Standby(host) }
    }
}

pub struct BleDevice<H, O>
where
    H: hci::HciDriver,
    O: BleHostObserver,
{
    hci: Hci<H>,
    observer: O,
}

impl<H, O> BleDevice<H, O>
where
    H: hci::HciDriver,
    O: BleHostObserver,
{
    pub fn new(hci_driver: H, observer: O) -> Self {
        Self {
            hci: Hci::new(hci_driver),
            observer,
        }
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        let host = BleHost::setup(&mut self.hci).await?;
        let _host = self.observer.ready(host).await;

        // todo!();

        Ok(())
    }
}
