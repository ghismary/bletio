use core::marker::PhantomData;
use core::num::NonZeroU16;
use core::ops::Deref;

use bletio_hci::{
    EventMask, Hci, HciDriver, LeEventMask, PublicDeviceAddress, RandomStaticDeviceAddress,
    SupportedCommands, SupportedFeatures, SupportedLeFeatures, SupportedLeStates,
};

use crate::advertising::{AdvertisingEnable, AdvertisingParameters, FullAdvertisingData};
use crate::assigned_numbers::AppearanceValue;
use crate::device_information::DeviceInformation;
use crate::Error;

pub trait BleHostState {}

pub struct BleHost<'a, H, State: BleHostState = BleHostStateInitial>
where
    H: HciDriver,
{
    hci: &'a mut Hci<H>,
    device_information: DeviceInformation,
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
        appearance: AppearanceValue,
    ) -> Result<BleHost<'a, H, BleHostStateStandby>, Error>
    where
        H: HciDriver,
    {
        let mut device_information = DeviceInformation {
            appearance,
            ..Default::default()
        };

        hci.cmd_reset().await?;

        device_information.supported_commands = hci.cmd_read_local_supported_commands().await?;
        device_information.supported_features = hci.cmd_read_local_supported_features().await?;
        if !device_information.is_feature_supported(SupportedFeatures::LE_SUPPORTED_CONTROLLER) {
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
        if device_information.is_command_supported(SupportedCommands::LE_SET_EVENT_MASK) {
            hci.cmd_le_set_event_mask(LeEventMask::default()).await?;
        }

        let (le_data_packet_length, num_le_data_packets) = hci.cmd_le_read_buffer_size().await?;
        let le_data_packet_length: Result<NonZeroU16, _> = le_data_packet_length.try_into();
        let num_le_data_packets: Result<NonZeroU16, _> = num_le_data_packets.try_into();
        match (le_data_packet_length, num_le_data_packets) {
            (Err(_), Ok(_)) | (Ok(_), Err(_)) | (Err(_), Err(_)) => {
                if device_information.is_command_supported(SupportedCommands::READ_BUFFER_SIZE) {
                    (
                        device_information.le_data_packet_length,
                        _,
                        device_information.num_le_data_packets,
                        _,
                    ) = hci.cmd_read_buffer_size().await?;
                } else {
                    return Err(Error::NonLeCapableController);
                }
            }
            (Ok(le_data_packet_length), Ok(num_le_data_packets)) => {
                device_information.le_data_packet_length = le_data_packet_length;
                device_information.num_le_data_packets = num_le_data_packets;
            }
        }
        if device_information
            .is_command_supported(SupportedCommands::LE_READ_LOCAL_SUPPORTED_FEATURES_PAGE_0)
        {
            device_information.supported_le_features =
                hci.cmd_le_read_local_supported_features_page_0().await?;
        }

        device_information.supported_le_states = hci.cmd_le_read_supported_states().await?;
        device_information.public_device_address = hci.cmd_read_bd_addr().await?;

        Ok(BleHost::<H, BleHostStateStandby> {
            hci,
            device_information,
            phantom: PhantomData,
        })
    }
}

impl<'a, H> BleHost<'a, H, BleHostStateStandby>
where
    H: HciDriver,
{
    pub async fn create_random_address(&mut self) -> Result<(), Error> {
        if self
            .device_information
            .random_static_device_address
            .is_some()
        {
            return Err(Error::RandomAddressAlreadyCreated);
        }
        loop {
            let random_bytes = self.hci.cmd_le_rand().await?;
            if let Ok(random_address) = RandomStaticDeviceAddress::try_new_from_random_bytes(
                (&random_bytes[..6]).try_into().unwrap(),
            ) {
                self.hci
                    .cmd_le_set_random_address(random_address.clone())
                    .await?;
                self.device_information.random_static_device_address = Some(random_address);
                return Ok(());
            }
        }
    }

    pub async fn start_advertising(
        mut self,
        adv_params: &AdvertisingParameters,
        full_adv_data: &FullAdvertisingData<'_>,
    ) -> Result<BleHost<'a, H, BleHostStateAdvertising>, (Error, Self)> {
        async fn inner<H>(
            hci: &mut Hci<H>,
            device_information: &mut DeviceInformation,
            adv_params: &AdvertisingParameters,
            full_adv_data: &FullAdvertisingData<'_>,
        ) -> Result<(), Error>
        where
            H: HciDriver,
        {
            hci.cmd_le_set_advertising_parameters(adv_params.deref().clone())
                .await?;
            device_information.tx_power_level =
                hci.cmd_le_read_advertising_channel_tx_power().await?;

            let full_adv_data = full_adv_data.fill_automatic_data(device_information);
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
            &mut self.device_information,
            adv_params,
            full_adv_data,
        )
        .await
        {
            Ok(()) => Ok(BleHost::<H, BleHostStateAdvertising> {
                hci: self.hci,
                device_information: self.device_information,
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
            hci: self.hci,
            device_information: self.device_information,
            phantom: PhantomData,
        })
    }
}

impl<H, S> BleHost<'_, H, S>
where
    H: HciDriver,
    S: BleHostState,
{
    pub fn public_device_address(&self) -> &PublicDeviceAddress {
        &self.device_information.public_device_address
    }

    pub fn random_static_device_address(&self) -> Option<&RandomStaticDeviceAddress> {
        self.device_information
            .random_static_device_address
            .as_ref()
    }

    pub fn supported_commands(&self) -> &SupportedCommands {
        &self.device_information.supported_commands
    }

    pub fn supported_features(&self) -> &SupportedFeatures {
        &self.device_information.supported_features
    }

    pub fn supported_le_features(&self) -> &SupportedLeFeatures {
        &self.device_information.supported_le_features
    }

    pub fn supported_le_states(&self) -> &SupportedLeStates {
        &self.device_information.supported_le_states
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
