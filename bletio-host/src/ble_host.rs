use core::marker::PhantomData;
use core::num::NonZeroU16;
use core::ops::Deref;

use bletio_hci::{
    ConnectionHandle, ConnectionPeerAddress, DisconnectionCompleteEvent, EventList, EventMask,
    FilterDuplicates, Hci, HciDriver, LeAdvertisingReportEventType, LeConnectionCompleteEvent,
    LeConnectionUpdateCompleteEvent, LeEventMask, LeFilterAcceptListAddress, PublicDeviceAddress,
    RandomStaticDeviceAddress, Reason, Rssi, ScanEnable, SupportedCommands, SupportedFeatures,
    SupportedLeFeatures, SupportedLeStates,
};

use crate::advertising::{
    AdvertisingEnable, AdvertisingParameters, FullAdvertisingData, ScanParameters,
};
use crate::assigned_numbers::AppearanceValue;
use crate::device_information::DeviceInformation;
use crate::{ConnectionParameters, ConnectionUpdateParameters, Error};

pub trait BleHostState {}

pub struct BleHost<'a, H, State: BleHostState = BleHostStateInitial>
where
    H: HciDriver,
{
    hci: Hci<H>,
    device_information: DeviceInformation<'a>,
    phantom: PhantomData<State>,
}

#[derive(Debug, Default)]
pub struct BleHostStateInitial;
#[derive(Debug, Default)]
pub struct BleHostStateStandby;
#[derive(Debug, Default)]
pub struct BleHostStateAdvertising;
#[derive(Debug, Default)]
pub struct BleHostStateScanning;
#[derive(Debug, Default)]
pub struct BleHostStateInitiating;
#[derive(Debug, Default)]
pub struct BleHostStateConnectedCentral;
#[derive(Debug, Default)]
pub struct BleHostStateConnectedPeripheral;

impl BleHostState for BleHostStateInitial {}
impl BleHostState for BleHostStateStandby {}
impl BleHostState for BleHostStateAdvertising {}
impl BleHostState for BleHostStateScanning {}
impl BleHostState for BleHostStateInitiating {}
impl BleHostState for BleHostStateConnectedCentral {}
impl BleHostState for BleHostStateConnectedPeripheral {}

impl<'a, H> BleHost<'a, H, BleHostStateInitial>
where
    H: HciDriver,
{
    // Perform setup has described in Core specification 4.2, Vol. 6, Part D, 2.1
    pub(crate) async fn setup(
        mut hci: Hci<H>,
        appearance: AppearanceValue,
        local_name: &'a str,
    ) -> Result<BleHost<'a, H, BleHostStateStandby>, Error>
    where
        H: HciDriver,
    {
        let mut device_information = DeviceInformation {
            appearance,
            local_name,
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
    pub async fn add_le_filter_accept_list_device(
        &mut self,
        address: impl Into<LeFilterAcceptListAddress>,
    ) -> Result<(), Error> {
        if self
            .device_information
            .is_command_supported(SupportedCommands::LE_ADD_DEVICE_TO_FILTER_ACCEPT_LIST)
        {
            Ok(self
                .hci
                .cmd_le_add_device_to_filter_accept_list(address.into())
                .await?)
        } else {
            Err(Error::ControllerDoesNotSupportCommand(
                SupportedCommands::LE_ADD_DEVICE_TO_FILTER_ACCEPT_LIST,
            ))
        }
    }

    pub async fn clear_le_filter_accept_list(&mut self) -> Result<(), Error> {
        if self
            .device_information
            .is_command_supported(SupportedCommands::LE_CLEAR_FILTER_ACCEPT_LIST)
        {
            Ok(self.hci.cmd_le_clear_filter_accept_list().await?)
        } else {
            Err(Error::ControllerDoesNotSupportCommand(
                SupportedCommands::LE_CLEAR_FILTER_ACCEPT_LIST,
            ))
        }
    }

    pub async fn connect(
        mut self,
        connection_parameters: &ConnectionParameters,
    ) -> Result<BleHost<'a, H, BleHostStateInitiating>, Error> {
        self.hci
            .cmd_le_create_connection(connection_parameters.deref().clone())
            .await?;
        Ok(self.change_state())
    }

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

    pub async fn get_le_filter_accept_list_size(&mut self) -> Result<usize, Error> {
        if self
            .device_information
            .is_command_supported(SupportedCommands::LE_READ_FILTER_ACCEPT_LIST_SIZE)
        {
            Ok(self.hci.cmd_le_read_filter_accept_list_size().await?)
        } else {
            Err(Error::ControllerDoesNotSupportCommand(
                SupportedCommands::LE_READ_FILTER_ACCEPT_LIST_SIZE,
            ))
        }
    }

    pub async fn remove_le_filter_accept_list_device(
        &mut self,
        address: impl Into<LeFilterAcceptListAddress>,
    ) -> Result<(), Error> {
        if self
            .device_information
            .is_command_supported(SupportedCommands::LE_REMOVE_DEVICE_FROM_FILTER_ACCEPT_LIST)
        {
            Ok(self
                .hci
                .cmd_le_remove_device_from_filter_accept_list(address.into())
                .await?)
        } else {
            Err(Error::ControllerDoesNotSupportCommand(
                SupportedCommands::LE_REMOVE_DEVICE_FROM_FILTER_ACCEPT_LIST,
            ))
        }
    }

    pub async fn start_advertising(
        mut self,
        adv_params: &AdvertisingParameters,
        full_adv_data: &FullAdvertisingData,
    ) -> Result<BleHost<'a, H, BleHostStateAdvertising>, (Error, Self)> {
        async fn inner<H>(
            hci: &mut Hci<H>,
            device_information: &mut DeviceInformation<'_>,
            adv_params: &AdvertisingParameters,
            full_adv_data: &FullAdvertisingData,
        ) -> Result<(), Error>
        where
            H: HciDriver,
        {
            hci.cmd_le_set_advertising_parameters(adv_params.deref().clone())
                .await?;
            device_information.tx_power_level =
                hci.cmd_le_read_advertising_channel_tx_power().await?;

            let full_adv_data = full_adv_data.fill_automatic_data(device_information)?;
            let mut scanresp_data = bletio_hci::AdvertisingData::default();
            let adv_data = (&full_adv_data.adv_data).into();
            if let Some(data) = &(full_adv_data.scanresp_data) {
                scanresp_data = data.into();
            }

            hci.cmd_le_set_advertising_data(adv_data).await?;
            hci.cmd_le_set_scan_response_data(scanresp_data).await?;
            hci.cmd_le_set_advertising_enable(AdvertisingEnable::Enabled)
                .await?;
            Ok(())
        }
        match inner(
            &mut self.hci,
            &mut self.device_information,
            adv_params,
            full_adv_data,
        )
        .await
        {
            Ok(()) => Ok(self.change_state()),
            Err(e) => Err((e, self)),
        }
    }

    pub async fn start_scanning(
        mut self,
        scan_params: &ScanParameters,
        filter_duplicates: FilterDuplicates,
    ) -> Result<BleHost<'a, H, BleHostStateScanning>, (Error, Self)> {
        async fn inner<H>(
            hci: &mut Hci<H>,
            scan_params: &ScanParameters,
            filter_duplicates: FilterDuplicates,
        ) -> Result<(), Error>
        where
            H: HciDriver,
        {
            hci.cmd_le_set_scan_parameters(scan_params.deref().clone())
                .await?;
            hci.cmd_le_set_scan_enable(ScanEnable::Enabled, filter_duplicates)
                .await?;
            Ok(())
        }
        match inner(&mut self.hci, scan_params, filter_duplicates).await {
            Ok(()) => Ok(self.change_state()),
            Err(e) => Err((e, self)),
        }
    }
}

impl<'a, H> BleHost<'a, H, BleHostStateAdvertising>
where
    H: HciDriver,
{
    pub async fn stop_advertising(mut self) -> Result<BleHost<'a, H, BleHostStateStandby>, Error> {
        self.hci
            .cmd_le_set_advertising_enable(AdvertisingEnable::Disabled)
            .await?;
        Ok(self.change_state())
    }
}

impl<'a, H> BleHost<'a, H, BleHostStateScanning>
where
    H: HciDriver,
{
    pub async fn stop_scanning(mut self) -> Result<BleHost<'a, H, BleHostStateStandby>, Error> {
        self.hci
            .cmd_le_set_scan_enable(ScanEnable::Disabled, FilterDuplicates::Disabled)
            .await?;
        Ok(self.change_state())
    }
}

impl<'a, H> BleHost<'a, H, BleHostStateInitiating>
where
    H: HciDriver,
{
    pub async fn cancel_connection(mut self) -> Result<BleHost<'a, H, BleHostStateStandby>, Error> {
        self.hci.cmd_le_create_connection_cancel().await?;
        Ok(self.change_state())
    }
}

impl<H> BleHost<'_, H, BleHostStateConnectedCentral>
where
    H: HciDriver,
{
    pub async fn disconnect(
        &mut self,
        connection_handle: ConnectionHandle,
        reason: Reason,
    ) -> Result<(), Error> {
        self.hci.cmd_disconnect(connection_handle, reason).await?;
        Ok(())
    }

    pub async fn update_connection(
        &mut self,
        connection_update_parameters: ConnectionUpdateParameters,
    ) -> Result<(), Error> {
        self.hci
            .cmd_le_connection_update(connection_update_parameters.deref().clone())
            .await?;
        Ok(())
    }
}

impl<H> BleHost<'_, H, BleHostStateConnectedPeripheral>
where
    H: HciDriver,
{
    pub async fn disconnect(
        &mut self,
        connection_handle: ConnectionHandle,
        reason: Reason,
    ) -> Result<(), Error> {
        self.hci.cmd_disconnect(connection_handle, reason).await?;
        Ok(())
    }

    pub async fn update_connection(
        &mut self,
        connection_update_parameters: ConnectionUpdateParameters,
    ) -> Result<(), Error> {
        self.hci
            .cmd_le_connection_update(connection_update_parameters.deref().clone())
            .await?;
        Ok(())
    }
}

impl<'a, H, S> BleHost<'a, H, S>
where
    H: HciDriver,
    S: BleHostState,
{
    pub fn appearance(&self) -> AppearanceValue {
        self.device_information.appearance
    }

    pub fn local_name(&self) -> &str {
        self.device_information.local_name
    }

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

    pub(crate) fn change_state<NS>(self) -> BleHost<'a, H, NS>
    where
        NS: BleHostState,
    {
        BleHost::<'a, H, NS> {
            hci: self.hci,
            device_information: self.device_information,
            phantom: PhantomData,
        }
    }
}

pub enum BleHostStates<'a, H>
where
    H: HciDriver,
{
    Initial(BleHost<'a, H, BleHostStateInitial>),
    Standby(BleHost<'a, H, BleHostStateStandby>),
    Advertising(BleHost<'a, H, BleHostStateAdvertising>),
    Scanning(BleHost<'a, H, BleHostStateScanning>),
    Initiating(BleHost<'a, H, BleHostStateInitiating>),
    ConnectedCentral(BleHost<'a, H, BleHostStateConnectedCentral>),
    ConnectedPeripheral(BleHost<'a, H, BleHostStateConnectedPeripheral>),
}

impl<H> BleHostStates<'_, H>
where
    H: HciDriver,
{
    pub(crate) async fn wait_for_event(&mut self) -> Result<EventList, Error> {
        match self {
            Self::Initial(_) => Err(Error::CannotWaitForEventInThisState),
            Self::Standby(host) => Ok(host.hci.wait_for_event().await?),
            Self::Advertising(host) => Ok(host.hci.wait_for_event().await?),
            Self::Scanning(host) => Ok(host.hci.wait_for_event().await?),
            Self::Initiating(host) => Ok(host.hci.wait_for_event().await?),
            Self::ConnectedCentral(host) => Ok(host.hci.wait_for_event().await?),
            Self::ConnectedPeripheral(host) => Ok(host.hci.wait_for_event().await?),
        }
    }
}

pub trait BleHostObserver {
    #[allow(unused_variables)]
    fn advertising_report_received<'a, H>(
        &self,
        host: BleHostStates<'a, H>,
        event_type: LeAdvertisingReportEventType,
        address: &ConnectionPeerAddress,
        rssi: Option<Rssi>,
        data: FullAdvertisingData,
    ) -> impl core::future::Future<Output = BleHostStates<'a, H>>
    where
        H: HciDriver,
    {
        async { host }
    }

    #[allow(unused_variables)]
    fn connection_complete<'a, H>(
        &self,
        host: BleHostStates<'a, H>,
        event: &LeConnectionCompleteEvent,
    ) -> impl core::future::Future<Output = BleHostStates<'a, H>>
    where
        H: HciDriver,
    {
        async { host }
    }

    #[allow(unused_variables)]
    fn connection_update_complete<'a, H>(
        &self,
        host: BleHostStates<'a, H>,
        event: &LeConnectionUpdateCompleteEvent,
    ) -> impl core::future::Future<Output = BleHostStates<'a, H>>
    where
        H: HciDriver,
    {
        async { host }
    }

    #[allow(unused_variables)]
    fn disconnection_complete<'a, H>(
        &self,
        host: BleHostStates<'a, H>,
        event: &DisconnectionCompleteEvent,
    ) -> impl core::future::Future<Output = BleHostStates<'a, H>>
    where
        H: HciDriver,
    {
        async { host }
    }

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
