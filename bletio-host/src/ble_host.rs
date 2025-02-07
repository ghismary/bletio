use core::marker::PhantomData;
use core::num::NonZeroU16;
use core::ops::Deref;

use bitflags::Flags;
use bletio_hci::{
    EventMask, Hci, HciDriver, SupportedCommands, SupportedFeatures, SupportedLeFeatures,
    SupportedLeStates,
};

use crate::advertising::{AdvertisingEnable, AdvertisingParameters, FullAdvertisingData};
use crate::ble_device_information::BleDeviceInformation;
use crate::controller_capabilities::ControllerCapabilities;
use crate::Error;

pub trait BleHostState {}

pub struct BleHost<'a, H, State: BleHostState = BleHostStateInitial>
where
    H: HciDriver,
{
    controller_capabilities: ControllerCapabilities,
    hci: &'a mut Hci<H>,
    device_info: &'a BleDeviceInformation,
    phantom: PhantomData<State>,
}

#[derive(Debug, Default)]
pub struct BleHostStateInitial;
#[derive(Debug, Default)]
pub struct BleHostStateStandby;
#[derive(Debug, Default)]
pub struct BleHostStateAdvertising;

impl BleHostState for BleHostStateInitial {}
impl BleHostState for BleHostStateStandby {}
impl BleHostState for BleHostStateAdvertising {}

impl<'a, H> BleHost<'a, H, BleHostStateInitial>
where
    H: HciDriver,
{
    // Perform setup has described in Core specification 4.2, Vol. 6, Part D, 2.1
    pub(crate) async fn setup(
        hci: &'a mut Hci<H>,
        device_info: &'a BleDeviceInformation,
    ) -> Result<BleHost<'a, H, BleHostStateStandby>, Error>
    where
        H: HciDriver,
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

        let (le_data_packet_length, num_le_data_packets) = hci.cmd_le_read_buffer_size().await?;
        let le_data_packet_length: Result<NonZeroU16, _> = le_data_packet_length.try_into();
        let num_le_data_packets: Result<NonZeroU16, _> = num_le_data_packets.try_into();
        match (le_data_packet_length, num_le_data_packets) {
            (Err(_), Ok(_)) | (Ok(_), Err(_)) | (Err(_), Err(_)) => {
                if controller_capabilities
                    .supported_commands
                    .contains(SupportedCommands::READ_BUFFER_SIZE)
                {
                    (
                        controller_capabilities.le_data_packet_length,
                        _,
                        controller_capabilities.num_le_data_packets,
                        _,
                    ) = hci.cmd_read_buffer_size().await?;
                } else {
                    return Err(Error::NonLeCapableController);
                }
            }
            (Ok(le_data_packet_length), Ok(num_le_data_packets)) => {
                controller_capabilities.le_data_packet_length = le_data_packet_length;
                controller_capabilities.num_le_data_packets = num_le_data_packets;
            }
        }
        if controller_capabilities
            .supported_commands
            .contains(SupportedCommands::LE_READ_LOCAL_SUPPORTED_FEATURES_PAGE_0)
        {
            controller_capabilities.supported_le_features =
                hci.cmd_le_read_local_supported_features_page_0().await?;
        }
        controller_capabilities.supported_le_states = hci.cmd_le_read_supported_states().await?;

        Ok(BleHost::<H, BleHostStateStandby> {
            controller_capabilities,
            hci,
            device_info,
            phantom: PhantomData,
        })
    }
}

impl<'a, H> BleHost<'a, H, BleHostStateStandby>
where
    H: HciDriver,
{
    pub async fn start_advertising(
        mut self,
        adv_params: &AdvertisingParameters,
        full_adv_data: &FullAdvertisingData<'_>,
    ) -> Result<BleHost<'a, H, BleHostStateAdvertising>, (Error, Self)> {
        async fn inner<H>(
            hci: &mut Hci<H>,
            device_info: &BleDeviceInformation,
            controller_capabilities: &mut ControllerCapabilities,
            adv_params: &AdvertisingParameters,
            full_adv_data: &FullAdvertisingData<'_>,
        ) -> Result<(), Error>
        where
            H: HciDriver,
        {
            hci.cmd_le_set_advertising_parameters(adv_params.deref().clone())
                .await?;
            controller_capabilities.tx_power_level =
                hci.cmd_le_read_advertising_channel_tx_power().await?;

            let full_adv_data =
                full_adv_data.fill_automatic_data(device_info, controller_capabilities);
            let mut scanresp_data = bletio_hci::ScanResponseData::default();
            let adv_data = (&full_adv_data.adv_data).try_into()?;
            if let Some(data) = &full_adv_data.scanresp_data {
                scanresp_data = data.try_into()?;
            }

            hci.cmd_le_set_advertising_data(adv_data).await?;
            hci.cmd_le_set_scan_response_data(scanresp_data).await?;
            hci.cmd_le_set_advertising_enable(AdvertisingEnable::Enabled)
                .await?;
            Ok(())
        }
        match inner(
            self.hci,
            self.device_info,
            &mut self.controller_capabilities,
            adv_params,
            full_adv_data,
        )
        .await
        {
            Ok(()) => Ok(BleHost::<H, BleHostStateAdvertising> {
                controller_capabilities: self.controller_capabilities,
                hci: self.hci,
                device_info: self.device_info,
                phantom: PhantomData,
            }),
            Err(e) => Err((e, self)),
        }
    }
}

impl<'a, H> BleHost<'a, H, BleHostStateAdvertising>
where
    H: HciDriver,
{
    pub async fn stop_advertising(self) -> Result<BleHost<'a, H, BleHostStateStandby>, Error> {
        self.hci
            .cmd_le_set_advertising_enable(AdvertisingEnable::Disabled)
            .await?;
        Ok(BleHost::<H, BleHostStateStandby> {
            controller_capabilities: self.controller_capabilities,
            hci: self.hci,
            device_info: self.device_info,
            phantom: PhantomData,
        })
    }
}

impl<H> BleHost<'_, H>
where
    H: HciDriver,
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

pub enum BleHostStates<'a, H>
where
    H: HciDriver,
{
    Initial(BleHost<'a, H, BleHostStateInitial>),
    Standby(BleHost<'a, H, BleHostStateStandby>),
    Advertising(BleHost<'a, H, BleHostStateAdvertising>),
}

pub trait BleHostObserver {
    fn ready<'a, H>(
        &self,
        host: BleHost<'a, H, BleHostStateStandby>,
    ) -> impl core::future::Future<Output = BleHostStates<'a, H>>
    where
        H: HciDriver,
    {
        async { BleHostStates::Standby(host) }
    }
}
