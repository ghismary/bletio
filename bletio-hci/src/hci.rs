use core::{
    num::{NonZeroU16, NonZeroU8},
    time::Duration,
};

use crate::{
    AdvertisingData, AdvertisingEnable, AdvertisingParameters, Command, ConnectionHandle,
    ConnectionParameters, Error, ErrorCode, Event, EventList, EventMask, EventParameter,
    FilterDuplicates, HciBuffer, HciDriver, LeEventMask, LeFilterAcceptListAddress, Packet,
    PublicDeviceAddress, RandomStaticDeviceAddress, Reason, ScanEnable, ScanParameters,
    SupportedCommands, SupportedFeatures, SupportedLeFeatures, SupportedLeStates, TxPowerLevel,
    WithTimeout,
};

const HCI_COMMAND_TIMEOUT: Duration = Duration::from_millis(1000);

#[derive(Debug)]
pub struct Hci<H>
where
    H: HciDriver,
{
    driver: H,
    num_hci_command_packets: u8,
    read_buffer: HciBuffer,
    event_list: EventList,
}

impl<H> Hci<H>
where
    H: HciDriver,
{
    pub fn new(hci_driver: H) -> Self {
        Self {
            driver: hci_driver,
            num_hci_command_packets: 0,
            read_buffer: Default::default(),
            event_list: Default::default(),
        }
    }

    pub async fn cmd_disconnect(
        &mut self,
        connection_handle: ConnectionHandle,
        reason: Reason,
    ) -> Result<(), Error> {
        self.execute_command_with_command_status_response(Command::Disconnect(
            connection_handle,
            reason,
        ))
        .await
    }

    pub async fn cmd_le_add_device_to_filter_accept_list(
        &mut self,
        address: impl Into<LeFilterAcceptListAddress>,
    ) -> Result<(), Error> {
        self.cmd_with_command_complete_response_without_parameter(
            Command::LeAddDeviceToFilterAcceptList(address.into()),
        )
        .await
    }

    pub async fn cmd_le_clear_filter_accept_list(&mut self) -> Result<(), Error> {
        self.cmd_with_command_complete_response_without_parameter(Command::LeClearFilterAcceptList)
            .await
    }

    pub async fn cmd_le_create_connection(
        &mut self,
        connection_parameters: ConnectionParameters,
    ) -> Result<(), Error> {
        self.execute_command_with_command_status_response(Command::LeCreateConnection(
            connection_parameters,
        ))
        .await
    }

    pub async fn cmd_le_create_connection_cancel(&mut self) -> Result<(), Error> {
        self.cmd_with_command_complete_response_without_parameter(Command::LeCreateConnectionCancel)
            .await
    }

    pub async fn cmd_le_rand(&mut self) -> Result<[u8; 8], Error> {
        let (status, param) = self
            .execute_command_with_command_complete_response(Command::LeRand)
            .await?;
        if let (ErrorCode::Success, Some(EventParameter::RandomNumber(param))) = (status, param) {
            Ok(param.random_number)
        } else {
            Err(Error::ErrorCode(status))
        }
    }

    pub async fn cmd_le_read_advertising_channel_tx_power(
        &mut self,
    ) -> Result<TxPowerLevel, Error> {
        let (status, param) = self
            .execute_command_with_command_complete_response(
                Command::LeReadAdvertisingChannelTxPower,
            )
            .await?;
        if let (ErrorCode::Success, Some(EventParameter::TxPowerLevel(param))) = (status, param) {
            Ok(param.tx_power_level)
        } else {
            Err(Error::ErrorCode(status))
        }
    }

    pub async fn cmd_le_read_buffer_size(&mut self) -> Result<(u16, u16), Error> {
        let (status, param) = self
            .execute_command_with_command_complete_response(Command::LeReadBufferSize)
            .await?;
        if let (ErrorCode::Success, Some(EventParameter::LeBufferSize(param))) = (status, param) {
            Ok((
                param.le_acl_data_packet_length,
                param.total_num_le_acl_data_packets as u16,
            ))
        } else {
            Err(Error::ErrorCode(status))
        }
    }

    pub async fn cmd_le_read_filter_accept_list_size(&mut self) -> Result<usize, Error> {
        let (status, param) = self
            .execute_command_with_command_complete_response(Command::LeReadFilterAcceptListSize)
            .await?;
        if let (ErrorCode::Success, Some(EventParameter::FilterAcceptListSize(param))) =
            (status, param)
        {
            Ok(param.filter_accept_list_size)
        } else {
            Err(Error::ErrorCode(status))
        }
    }

    pub async fn cmd_le_read_local_supported_features_page_0(
        &mut self,
    ) -> Result<SupportedLeFeatures, Error> {
        let (status, param) = self
            .execute_command_with_command_complete_response(
                Command::LeReadLocalSupportedFeaturesPage0,
            )
            .await?;
        if let (ErrorCode::Success, Some(EventParameter::SupportedLeFeatures(param))) =
            (status, param)
        {
            Ok(param.supported_le_features)
        } else {
            Err(Error::ErrorCode(status))
        }
    }

    pub async fn cmd_le_read_supported_states(&mut self) -> Result<SupportedLeStates, Error> {
        let (status, param) = self
            .execute_command_with_command_complete_response(Command::LeReadSupportedStates)
            .await?;
        if let (ErrorCode::Success, Some(EventParameter::SupportedLeStates(param))) =
            (status, param)
        {
            Ok(param.supported_le_states)
        } else {
            Err(Error::ErrorCode(status))
        }
    }

    pub async fn cmd_le_remove_device_from_filter_accept_list(
        &mut self,
        address: impl Into<LeFilterAcceptListAddress>,
    ) -> Result<(), Error> {
        self.cmd_with_command_complete_response_without_parameter(
            Command::LeRemoveDeviceFromFilterAcceptList(address.into()),
        )
        .await
    }

    pub async fn cmd_le_set_advertising_data(
        &mut self,
        data: AdvertisingData,
    ) -> Result<(), Error> {
        self.cmd_with_command_complete_response_without_parameter(Command::LeSetAdvertisingData(
            data,
        ))
        .await
    }

    pub async fn cmd_le_set_advertising_enable(
        &mut self,
        enable: AdvertisingEnable,
    ) -> Result<(), Error> {
        self.cmd_with_command_complete_response_without_parameter(Command::LeSetAdvertisingEnable(
            enable,
        ))
        .await
    }

    pub async fn cmd_le_set_advertising_parameters(
        &mut self,
        parameters: AdvertisingParameters,
    ) -> Result<(), Error> {
        self.cmd_with_command_complete_response_without_parameter(
            Command::LeSetAdvertisingParameters(parameters),
        )
        .await
    }

    pub async fn cmd_le_set_random_address(
        &mut self,
        address: RandomStaticDeviceAddress,
    ) -> Result<(), Error> {
        self.cmd_with_command_complete_response_without_parameter(Command::LeSetRandomAddress(
            address,
        ))
        .await
    }

    pub async fn cmd_le_set_scan_enable(
        &mut self,
        scan_enable: ScanEnable,
        filter_duplicates: FilterDuplicates,
    ) -> Result<(), Error> {
        self.cmd_with_command_complete_response_without_parameter(Command::LeSetScanEnable(
            scan_enable,
            filter_duplicates,
        ))
        .await
    }

    pub async fn cmd_le_set_scan_parameters(
        &mut self,
        parameters: ScanParameters,
    ) -> Result<(), Error> {
        self.cmd_with_command_complete_response_without_parameter(Command::LeSetScanParameters(
            parameters,
        ))
        .await
    }

    pub async fn cmd_le_set_scan_response_data(
        &mut self,
        data: AdvertisingData,
    ) -> Result<(), Error> {
        self.cmd_with_command_complete_response_without_parameter(Command::LeSetScanResponseData(
            data,
        ))
        .await
    }

    pub async fn cmd_le_set_event_mask(&mut self, data: LeEventMask) -> Result<(), Error> {
        self.cmd_with_command_complete_response_without_parameter(Command::LeSetEventMask(data))
            .await
    }

    pub async fn cmd_read_bd_addr(&mut self) -> Result<PublicDeviceAddress, Error> {
        let (status, param) = self
            .execute_command_with_command_complete_response(Command::ReadBdAddr)
            .await?;
        if let (ErrorCode::Success, Some(EventParameter::BdAddr(param))) = (status, param) {
            Ok(param.bd_addr)
        } else {
            Err(Error::ErrorCode(status))
        }
    }

    pub async fn cmd_read_buffer_size(
        &mut self,
    ) -> Result<(NonZeroU16, NonZeroU8, NonZeroU16, u16), Error> {
        let (status, param) = self
            .execute_command_with_command_complete_response(Command::ReadBufferSize)
            .await?;
        if let (ErrorCode::Success, Some(EventParameter::BufferSize(param))) = (status, param) {
            Ok((
                param.acl_data_packet_length,
                param.synchronous_data_packet_length,
                param.total_num_acl_data_packets,
                param.total_num_synchronous_packets,
            ))
        } else {
            Err(Error::ErrorCode(status))
        }
    }

    pub async fn cmd_read_local_supported_commands(&mut self) -> Result<SupportedCommands, Error> {
        let (status, param) = self
            .execute_command_with_command_complete_response(Command::ReadLocalSupportedCommands)
            .await?;
        if let (ErrorCode::Success, Some(EventParameter::SupportedCommands(param))) =
            (status, param)
        {
            Ok(param.supported_commands)
        } else {
            Err(Error::ErrorCode(status))
        }
    }

    pub async fn cmd_read_local_supported_features(&mut self) -> Result<SupportedFeatures, Error> {
        let (status, param) = self
            .execute_command_with_command_complete_response(Command::ReadLocalSupportedFeatures)
            .await?;
        if let (ErrorCode::Success, Some(EventParameter::SupportedFeatures(param))) =
            (status, param)
        {
            Ok(param.supported_features)
        } else {
            Err(Error::ErrorCode(status))
        }
    }

    pub async fn cmd_reset(&mut self) -> Result<(), Error> {
        self.cmd_with_command_complete_response_without_parameter(Command::Reset)
            .await
    }

    pub async fn cmd_set_event_mask(&mut self, event_mask: EventMask) -> Result<(), Error> {
        self.cmd_with_command_complete_response_without_parameter(Command::SetEventMask(event_mask))
            .await
    }

    pub async fn wait_for_event(&mut self) -> Result<EventList, Error> {
        let mut event_list = core::mem::take(&mut self.event_list);

        loop {
            if (self.read_buffer.is_empty() && !event_list.is_empty()) || event_list.is_full() {
                return Ok(event_list);
            }

            match Self::hci_read_and_parse_packet(&mut self.driver, &mut self.read_buffer).await {
                Ok((remaining, packet)) => {
                    match packet {
                        Packet::Command(_) => {
                            // The Host is not supposed to receive commands, ignore it!
                            #[cfg(feature = "defmt")]
                            defmt::warn!("Received command while waiting for event, ignore it!");
                        }
                        Packet::AclData(_data) => {
                            #[cfg(feature = "defmt")]
                            defmt::debug!("Received ACL data packet with data: {:?}", _data);
                            // TODO
                        }
                        Packet::Event(event) => {
                            Self::update_num_hci_command_packets(
                                &mut self.num_hci_command_packets,
                                &event,
                            );

                            // INVARIANT: The event list is known to be able to hold this event,
                            // otherwise we would have returned at the beginning of the loop.
                            event_list.push(event).unwrap();
                        }
                    }

                    // INVARIANT: The remaining is known to be shorter than the buffer.
                    self.read_buffer = remaining.try_into().unwrap();
                }
                Err(e) => {
                    self.read_buffer.clear();
                    return Err(e);
                }
            }
        }
    }

    async fn cmd_with_command_complete_response_without_parameter(
        &mut self,
        command: Command,
    ) -> Result<(), Error> {
        let (status, _) = self
            .execute_command_with_command_complete_response(command)
            .await?;
        if status.is_success() {
            Ok(())
        } else {
            Err(Error::ErrorCode(status))
        }
    }

    async fn execute_command(&mut self, command: Command) -> Result<Event, Error> {
        if self.num_hci_command_packets == 0 {
            self.wait_controller_ready().await?;
        }
        let event = self
            .send_command_and_wait_response(command)
            .with_timeout(HCI_COMMAND_TIMEOUT)
            .await??;
        Ok(event)
    }

    async fn execute_command_with_command_complete_response(
        &mut self,
        command: Command,
    ) -> Result<(ErrorCode, Option<EventParameter>), Error> {
        if let Event::CommandComplete(event) = self.execute_command(command).await? {
            Ok((event.status, event.parameter))
        } else {
            Err(Error::UnexpectedEvent)
        }
    }

    async fn execute_command_with_command_status_response(
        &mut self,
        command: Command,
    ) -> Result<(), Error> {
        match self.execute_command(command).await? {
            Event::CommandStatus(event) if event.status.is_success() => Ok(()),
            Event::CommandStatus(event) => Err(Error::ErrorCode(event.status)),
            _ => Err(Error::UnexpectedEvent), // Actually, not reachable
        }
    }

    async fn send_command_and_wait_response(&mut self, command: Command) -> Result<Event, Error> {
        let command_packet = command.encode()?;
        self.driver.write(command_packet.data()).await?;
        loop {
            match Self::hci_read_and_parse_packet(&mut self.driver, &mut self.read_buffer).await {
                Ok((remaining, packet)) => {
                    let result = match packet {
                        Packet::Command(_) => {
                            // The Host is not supposed to receive commands!
                            Some(Err(Error::InvalidPacket))
                        }
                        Packet::AclData(_) => {
                            todo!()
                        }
                        Packet::Event(event) => {
                            Self::update_num_hci_command_packets(
                                &mut self.num_hci_command_packets,
                                &event,
                            );

                            match &event {
                                Event::CommandComplete(command_complete_event)
                                    if command_complete_event.opcode == command.opcode() =>
                                {
                                    Some(Ok(event))
                                }
                                Event::CommandStatus(command_status_event)
                                    if command_status_event.opcode == command.opcode() =>
                                {
                                    Some(Ok(event))
                                }
                                Event::Unsupported(_) => {
                                    // Ignore unsupported event
                                    None
                                }
                                _ => {
                                    // Other events will be handled higher in the stack
                                    if self.event_list.push(event).is_err() {
                                        #[cfg(feature = "defmt")]
                                        defmt::warn!("HCI event list is full, cannot add more!");
                                    }
                                    None
                                }
                            }
                        }
                    };

                    // INVARIANT: The remaining is known to be shorter than the buffer.
                    self.read_buffer = remaining.try_into().unwrap();

                    if let Some(result) = result {
                        return result;
                    }
                }
                Err(e) => {
                    self.read_buffer.clear();
                    return Err(e);
                }
            }
        }
    }

    async fn wait_controller_ready(&mut self) -> Result<(), Error> {
        while self.num_hci_command_packets == 0 {
            match Self::hci_read_and_parse_packet(&mut self.driver, &mut self.read_buffer).await {
                Ok((remaining, packet)) => {
                    match packet {
                        Packet::Command(_) => {
                            // The Host is not supposed to receive commands!
                            return Err(Error::InvalidPacket);
                        }
                        Packet::AclData(_) => {
                            // The host is not expecting to receive ACL data now!
                            return Err(Error::InvalidPacket);
                        }
                        Packet::Event(event) => {
                            Self::update_num_hci_command_packets(
                                &mut self.num_hci_command_packets,
                                &event,
                            );

                            // INVARIANT: The remaining is known to be shorter than the buffer.
                            self.read_buffer = remaining.try_into().unwrap();
                        }
                    }
                }
                Err(e) => {
                    self.read_buffer.clear();
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    async fn hci_read_and_parse_packet<'a>(
        driver: &mut H,
        read_buffer: &'a mut HciBuffer,
    ) -> Result<(&'a [u8], Packet), Error>
    where
        H: HciDriver,
    {
        if read_buffer.is_empty() {
            read_buffer.read(driver).await?;
        }
        let (remaining, hci_packet) =
            crate::packet::parser::packet(read_buffer.data()).map_err(|_| Error::InvalidPacket)?;
        Ok((remaining, hci_packet))
    }

    fn update_num_hci_command_packets(num_hci_command_packets: &mut u8, event: &Event) {
        match event {
            Event::CommandComplete(event) => {
                *num_hci_command_packets = event.num_hci_command_packets;
            }
            Event::CommandStatus(event) => {
                *num_hci_command_packets = event.num_hci_command_packets;
            }
            _ => {
                // Ignore other events
            }
        }
    }
}

#[cfg(test)]
mod test {
    use core::time::Duration;

    use rstest::{fixture, rstest};
    use tokio_test::io::Mock;

    use super::*;
    use crate::test::*;
    use crate::{
        connection_event_length_range, connection_interval, latency, supervision_timeout,
        CentralClockAccuracy, ConnectionHandle, ConnectionIntervalRange, ConnectionPeerAddress,
        DeviceAddress, ErrorCode, HciDriverError, InitiatorFilterPolicy, Latency,
        LeConnectionCompleteEvent, LeMetaEvent, OwnAddressType, RandomResolvablePrivateAddress,
        Role, ScanInterval, ScanWindow, SupervisionTimeout,
    };

    fn mock_cmd_disconnect_success() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 6, 4, 3, 0, 0, 19])
            .read(&[4, 15, 4, 0, 1, 6, 4])
            .build()
    }

    #[fixture]
    fn mock_cmd_disconnect_command_disallowed() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 6, 4, 3, 0, 0, 19])
            .read(&[4, 15, 4, 12, 1, 6, 4])
            .build()
    }

    #[rstest]
    #[case::success(mock_cmd_disconnect_success(), Ok(()))]
    #[case::command_disallowed(
        mock_cmd_disconnect_command_disallowed(),
        Err(Error::ErrorCode(ErrorCode::CommandDisallowed))
    )]
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_cmd_disconnect_mask(#[case] mock: Mock, #[case] expected: Result<(), Error>) {
        let hci_driver = TokioHciDriver { hci: mock };
        let mut hci = Hci {
            driver: hci_driver,
            num_hci_command_packets: 1,
            read_buffer: Default::default(),
            event_list: Default::default(),
        };
        assert_eq!(
            hci.cmd_disconnect(
                ConnectionHandle::try_new(0).unwrap(),
                Reason::RemoteUserTerminatedConnection
            )
            .await,
            expected
        );
    }

    #[fixture]
    fn mock_cmd_le_add_device_to_filter_accept_list_success() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 17, 32, 7, 1, 68, 223, 27, 9, 83, 250])
            .read(&[4, 14, 4, 1, 17, 32, 0])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_add_device_to_filter_accept_list_command_disallowed() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 17, 32, 7, 1, 68, 223, 27, 9, 83, 250])
            .read(&[4, 14, 4, 1, 17, 32, 12])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_add_device_to_filter_accept_list_invalid_event_packet() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 17, 32, 7, 1, 68, 223, 27, 9, 83, 250])
            .read(&[4, 14, 7, 1, 17, 32, 0])
            .build()
    }

    #[rstest]
    #[case::success(
        mock_cmd_le_add_device_to_filter_accept_list_success(),
        Ok(())
    )]
    #[case::command_disallowed(
        mock_cmd_le_add_device_to_filter_accept_list_command_disallowed(),
        Err(Error::ErrorCode(ErrorCode::CommandDisallowed))
    )]
    #[case::invalid_event_packet(
        mock_cmd_le_add_device_to_filter_accept_list_invalid_event_packet(),
        Err(Error::InvalidPacket)
    )]
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_cmd_le_add_device_to_filter_accept_list(
        #[case] mock: Mock,
        #[case] expected: Result<(), Error>,
    ) {
        let hci_driver = TokioHciDriver { hci: mock };
        let mut hci = Hci {
            driver: hci_driver,
            num_hci_command_packets: 1,
            read_buffer: Default::default(),
            event_list: Default::default(),
        };
        assert_eq!(
            hci.cmd_le_add_device_to_filter_accept_list(DeviceAddress::Random(
                RandomStaticDeviceAddress::try_new([68, 223, 27, 9, 83, 250])
                    .unwrap()
                    .into()
            ))
            .await,
            expected
        );
    }

    #[fixture]
    fn mock_cmd_le_clear_filter_accept_list_success() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 16, 32, 0])
            .read(&[4, 14, 4, 1, 16, 32, 0])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_clear_filter_accept_list_hardware_failure() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 16, 32, 0])
            .read(&[4, 14, 4, 1, 16, 32, 3])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_clear_filter_accept_list_invalid_event_packet() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 16, 32, 0])
            .read(&[4, 14, 30, 1, 16, 32, 0, 2, 1, 7])
            .build()
    }

    #[rstest]
    #[case::success(mock_cmd_le_clear_filter_accept_list_success(), Ok(()))]
    #[case::hardware_failure(
        mock_cmd_le_clear_filter_accept_list_hardware_failure(),
        Err(Error::ErrorCode(ErrorCode::HardwareFailure))
    )]
    #[case::invalid_event_packet(
        mock_cmd_le_clear_filter_accept_list_invalid_event_packet(),
        Err(Error::InvalidPacket)
    )]
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_cmd_le_clear_filter_accept_list(
        #[case] mock: Mock,
        #[case] expected: Result<(), Error>,
    ) {
        let hci_driver = TokioHciDriver { hci: mock };
        let mut hci = Hci {
            driver: hci_driver,
            num_hci_command_packets: 1,
            read_buffer: Default::default(),
            event_list: Default::default(),
        };
        assert_eq!(hci.cmd_le_clear_filter_accept_list().await, expected);
    }

    #[fixture]
    fn mock_cmd_le_create_connection_success() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[
                1, 13, 32, 25, 16, 0, 16, 0, 0, 1, 83, 251, 125, 93, 119, 88, 0, 64, 0, 64, 0, 0,
                0, 32, 0, 10, 0, 100, 0,
            ])
            .read(&[4, 15, 4, 0, 1, 13, 32])
            .wait(Duration::from_millis(10))
            .read(&[
                4, 62, 19, 1, 0, 0, 0, 0, 1, 83, 251, 125, 93, 119, 88, 64, 0, 0, 0, 32, 0, 0,
            ])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_create_connection_command_disallowed() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[
                1, 13, 32, 25, 16, 0, 16, 0, 0, 1, 83, 251, 125, 93, 119, 88, 0, 64, 0, 64, 0, 0,
                0, 32, 0, 10, 0, 100, 0,
            ])
            .read(&[4, 15, 4, 12, 1, 13, 32])
            .build()
    }

    #[rstest]
    #[case::success(
        mock_cmd_le_create_connection_success(),
        Ok(()),
        Some(Event::LeMeta(LeMetaEvent::LeConnectionComplete(LeConnectionCompleteEvent {
            status: ErrorCode::Success,
            connection_handle: ConnectionHandle::try_new(0).unwrap(),
            role: Role::Central,
            peer_address: RandomResolvablePrivateAddress::try_new([83, 251, 125, 93, 119, 88]).unwrap().into(),
            connection_interval: connection_interval!(64),
            peripheral_latency: latency!(0),
            supervision_timeout: supervision_timeout!(32),
            central_clock_accuracy: CentralClockAccuracy::Ppm500
        })))
    )]
    #[case::command_disallowed(
        mock_cmd_le_create_connection_command_disallowed(),
        Err(Error::ErrorCode(ErrorCode::CommandDisallowed)),
        None
    )]
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_cmd_le_create_connection(
        #[case] mock: Mock,
        #[case] expected_cmd_result: Result<(), Error>,
        #[case] expected_event: Option<Event>,
    ) {
        let hci_driver = TokioHciDriver { hci: mock };
        let mut hci = Hci {
            driver: hci_driver,
            num_hci_command_packets: 1,
            read_buffer: Default::default(),
            event_list: Default::default(),
        };
        let connection_params = ConnectionParameters::try_new(
            ScanInterval::default(),
            ScanWindow::default(),
            InitiatorFilterPolicy::FilterAcceptListNotUsed,
            ConnectionPeerAddress::RandomDevice(
                RandomResolvablePrivateAddress::try_new([83, 251, 125, 93, 119, 88])
                    .unwrap()
                    .into(),
            ),
            OwnAddressType::PublicDeviceAddress,
            ConnectionIntervalRange::default(),
            Latency::default(),
            SupervisionTimeout::default(),
            connection_event_length_range!(10, 100),
        )
        .unwrap();
        assert_eq!(
            hci.cmd_le_create_connection(connection_params).await,
            expected_cmd_result
        );
        if expected_event.is_some() {
            let mut event_list = hci.wait_for_event().await.unwrap();
            assert_eq!(event_list.len(), 1);
            assert_eq!(event_list.pop(), expected_event);
        }
    }

    #[fixture]
    fn mock_cmd_le_create_connection_cancel_success() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 14, 32, 0])
            .read(&[4, 14, 4, 1, 14, 32, 0])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_create_connection_cancel_command_disallowed() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 14, 32, 0])
            .read(&[4, 14, 4, 1, 14, 32, 12])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_create_connection_cancel_invalid_event_packet() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 14, 32, 0])
            .read(&[4, 14, 7, 1, 14, 32, 0, 8, 74, 108])
            .build()
    }

    #[rstest]
    #[case::success(mock_cmd_le_create_connection_cancel_success(), Ok(()))]
    #[case::command_disallowed(
        mock_cmd_le_create_connection_cancel_command_disallowed(),
        Err(Error::ErrorCode(ErrorCode::CommandDisallowed))
    )]
    #[case::invalid_event_packet(
        mock_cmd_le_create_connection_cancel_invalid_event_packet(),
        Err(Error::InvalidPacket)
    )]
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_cmd_le_create_connection_cancel(
        #[case] mock: Mock,
        #[case] expected: Result<(), Error>,
    ) {
        let hci_driver = TokioHciDriver { hci: mock };
        let mut hci = Hci {
            driver: hci_driver,
            num_hci_command_packets: 1,
            read_buffer: Default::default(),
            event_list: Default::default(),
        };
        assert_eq!(hci.cmd_le_create_connection_cancel().await, expected);
    }

    #[fixture]
    fn mock_cmd_le_rand_success() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 24, 32, 0])
            .read(&[4, 14, 12, 1, 24, 32, 0, 68, 223, 27, 9, 83, 58, 224, 240])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_rand_hardware_failure() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 24, 32, 0])
            .read(&[4, 14, 4, 1, 24, 32, 3])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_rand_invalid_event_packet() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 24, 32, 0])
            .read(&[4, 14, 60, 1, 24, 32, 0, 1, 9, 2])
            .build()
    }

    #[rstest]
    #[case::success(
        mock_cmd_le_rand_success(),
        Ok([68, 223, 27, 9, 83, 58, 224, 240])
    )]
    #[case::hardware_failure(
        mock_cmd_le_rand_hardware_failure(),
        Err(Error::ErrorCode(ErrorCode::HardwareFailure))
    )]
    #[case::invalid_event_packet(
        mock_cmd_le_rand_invalid_event_packet(),
        Err(Error::InvalidPacket)
    )]
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_cmd_le_rand(#[case] mock: Mock, #[case] expected: Result<[u8; 8], Error>) {
        let hci_driver = TokioHciDriver { hci: mock };
        let mut hci = Hci {
            driver: hci_driver,
            num_hci_command_packets: 1,
            read_buffer: Default::default(),
            event_list: Default::default(),
        };
        assert_eq!(hci.cmd_le_rand().await, expected);
    }

    #[fixture]
    fn mock_cmd_le_read_advertising_channel_tx_power_success() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 7, 32, 0])
            .read(&[4, 14, 5, 1, 7, 32, 0, 9])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_read_advertising_channel_tx_power_unknown_hci_command() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 7, 32, 0])
            .read(&[4, 14, 4, 1, 7, 32, 1])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_read_advertising_channel_tx_power_invalid_event_packet() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 7, 32, 0])
            .read(&[4, 14, 15, 1, 7, 32, 0, 1, 9, 2])
            .build()
    }

    #[rstest]
    #[case::success(
        mock_cmd_le_read_advertising_channel_tx_power_success(),
        Ok(TxPowerLevel::try_new(9).unwrap())
    )]
    #[case::unknown_hci_command(
        mock_cmd_le_read_advertising_channel_tx_power_unknown_hci_command(),
        Err(Error::ErrorCode(ErrorCode::UnknownHciCommand))
    )]
    #[case::invalid_event_packet(
        mock_cmd_le_read_advertising_channel_tx_power_invalid_event_packet(),
        Err(Error::InvalidPacket)
    )]
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_cmd_le_read_advertising_channel_tx_power(
        #[case] mock: Mock,
        #[case] expected: Result<TxPowerLevel, Error>,
    ) {
        let hci_driver = TokioHciDriver { hci: mock };
        let mut hci = Hci {
            driver: hci_driver,
            num_hci_command_packets: 1,
            read_buffer: Default::default(),
            event_list: Default::default(),
        };
        assert_eq!(
            hci.cmd_le_read_advertising_channel_tx_power().await,
            expected
        );
    }

    #[fixture]
    fn mock_cmd_le_read_buffer_size_success() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 2, 32, 0])
            .read(&[4, 14, 7, 1, 2, 32, 0, 255, 0, 24])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_read_buffer_size_command_disallowed() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 2, 32, 0])
            .read(&[4, 14, 4, 1, 2, 32, 12])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_read_buffer_size_invalid_event_packet() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 2, 32, 0])
            .read(&[4, 14, 13, 1, 2, 32, 0])
            .build()
    }

    #[rstest]
    #[case::success(
        mock_cmd_le_read_buffer_size_success(),
        Ok((255, 24))
    )]
    #[case::command_disallowed(
        mock_cmd_le_read_buffer_size_command_disallowed(),
        Err(Error::ErrorCode(ErrorCode::CommandDisallowed))
    )]
    #[case::invalid_event_packet(
        mock_cmd_le_read_buffer_size_invalid_event_packet(),
        Err(Error::InvalidPacket)
    )]
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_cmd_le_read_buffer_size(
        #[case] mock: Mock,
        #[case] expected: Result<(u16, u16), Error>,
    ) {
        let hci_driver = TokioHciDriver { hci: mock };
        let mut hci = Hci {
            driver: hci_driver,
            num_hci_command_packets: 1,
            read_buffer: Default::default(),
            event_list: Default::default(),
        };
        assert_eq!(hci.cmd_le_read_buffer_size().await, expected);
    }

    #[fixture]
    fn mock_cmd_le_read_filter_accept_list_size_success() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 15, 32, 0])
            .read(&[4, 14, 5, 1, 15, 32, 0, 12])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_read_filter_accept_list_size_command_disallowed() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 15, 32, 0])
            .read(&[4, 14, 4, 1, 15, 32, 12])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_read_filter_accept_list_size_invalid_event_packet() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 15, 32, 0])
            .read(&[4, 14, 20, 1, 15, 32, 0])
            .build()
    }

    #[rstest]
    #[case::success(mock_cmd_le_read_filter_accept_list_size_success(), Ok(12))]
    #[case::command_disallowed(
        mock_cmd_le_read_filter_accept_list_size_command_disallowed(),
        Err(Error::ErrorCode(ErrorCode::CommandDisallowed))
    )]
    #[case::invalid_event_packet(
        mock_cmd_le_read_filter_accept_list_size_invalid_event_packet(),
        Err(Error::InvalidPacket)
    )]
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_cmd_le_read_filter_accept_list_size(
        #[case] mock: Mock,
        #[case] expected: Result<usize, Error>,
    ) {
        let hci_driver = TokioHciDriver { hci: mock };
        let mut hci = Hci {
            driver: hci_driver,
            num_hci_command_packets: 1,
            read_buffer: Default::default(),
            event_list: Default::default(),
        };
        assert_eq!(hci.cmd_le_read_filter_accept_list_size().await, expected);
    }

    #[fixture]
    fn mock_cmd_le_read_local_supported_features_page_0_success() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 3, 32, 0])
            .read(&[4, 14, 12, 1, 3, 32, 0, 1, 16, 0, 0, 0, 0, 0, 0])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_read_local_supported_features_page_0_hardware_failure() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 3, 32, 0])
            .read(&[4, 14, 4, 1, 3, 32, 3])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_read_local_supported_features_page_0_invalid_event_packet() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 3, 32, 0])
            .read(&[4, 14, 7, 1, 3, 32, 0])
            .build()
    }

    #[rstest]
    #[case::success(
        mock_cmd_le_read_local_supported_features_page_0_success(),
        Ok(SupportedLeFeatures::LE_ENCRYPTION | SupportedLeFeatures::LE_EXTENDED_ADVERTISING)
    )]
    #[case::hardware_failure(
        mock_cmd_le_read_local_supported_features_page_0_hardware_failure(),
        Err(Error::ErrorCode(ErrorCode::HardwareFailure))
    )]
    #[case::invalid_event_packet(
        mock_cmd_le_read_local_supported_features_page_0_invalid_event_packet(),
        Err(Error::InvalidPacket)
    )]
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_cmd_le_read_local_supported_features_page_0(
        #[case] mock: Mock,
        #[case] expected: Result<SupportedLeFeatures, Error>,
    ) {
        let hci_driver = TokioHciDriver { hci: mock };
        let mut hci = Hci {
            driver: hci_driver,
            num_hci_command_packets: 1,
            read_buffer: Default::default(),
            event_list: Default::default(),
        };
        assert_eq!(
            hci.cmd_le_read_local_supported_features_page_0().await,
            expected
        );
    }

    #[fixture]
    fn mock_cmd_le_read_supported_states_success() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 28, 32, 0])
            .read(&[4, 14, 12, 1, 28, 32, 0, 255, 255, 255, 255, 255, 3, 0, 0])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_read_supported_states_command_disallowed() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 28, 32, 0])
            .read(&[4, 14, 4, 1, 28, 32, 12])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_read_supported_states_invalid_event_packet() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 28, 32, 0])
            .read(&[4, 14, 37, 1, 28, 32, 0])
            .build()
    }

    #[rstest]
    #[case::success(
        mock_cmd_le_read_supported_states_success(),
        Ok(0x0000_03FF_FFFF_FFFF.into())
    )]
    #[case::command_disallowed(
        mock_cmd_le_read_supported_states_command_disallowed(),
        Err(Error::ErrorCode(ErrorCode::CommandDisallowed))
    )]
    #[case::invalid_event_packet(
        mock_cmd_le_read_supported_states_invalid_event_packet(),
        Err(Error::InvalidPacket)
    )]
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_cmd_le_read_supported_states(
        #[case] mock: Mock,
        #[case] expected: Result<SupportedLeStates, Error>,
    ) {
        let hci_driver = TokioHciDriver { hci: mock };
        let mut hci = Hci {
            driver: hci_driver,
            num_hci_command_packets: 1,
            read_buffer: Default::default(),
            event_list: Default::default(),
        };
        assert_eq!(hci.cmd_le_read_supported_states().await, expected);
    }

    #[fixture]
    fn mock_cmd_le_remove_device_from_filter_accept_list_success() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 18, 32, 7, 1, 68, 223, 27, 9, 83, 250])
            .read(&[4, 14, 4, 1, 18, 32, 0])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_remove_device_from_filter_accept_list_command_disallowed() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 18, 32, 7, 1, 68, 223, 27, 9, 83, 250])
            .read(&[4, 14, 4, 1, 18, 32, 12])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_remove_device_from_filter_accept_list_invalid_event_packet() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 18, 32, 7, 1, 68, 223, 27, 9, 83, 250])
            .read(&[4, 14, 7, 1, 18, 32, 0])
            .build()
    }

    #[rstest]
    #[case::success(
        mock_cmd_le_remove_device_from_filter_accept_list_success(),
        Ok(())
    )]
    #[case::command_disallowed(
        mock_cmd_le_remove_device_from_filter_accept_list_command_disallowed(),
        Err(Error::ErrorCode(ErrorCode::CommandDisallowed))
    )]
    #[case::invalid_event_packet(
        mock_cmd_le_remove_device_from_filter_accept_list_invalid_event_packet(),
        Err(Error::InvalidPacket)
    )]
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_cmd_le_remove_device_from_filter_accept_list(
        #[case] mock: Mock,
        #[case] expected: Result<(), Error>,
    ) {
        let hci_driver = TokioHciDriver { hci: mock };
        let mut hci = Hci {
            driver: hci_driver,
            num_hci_command_packets: 1,
            read_buffer: Default::default(),
            event_list: Default::default(),
        };
        assert_eq!(
            hci.cmd_le_remove_device_from_filter_accept_list(DeviceAddress::Random(
                RandomStaticDeviceAddress::try_new([68, 223, 27, 9, 83, 250])
                    .unwrap()
                    .into()
            ))
            .await,
            expected
        );
    }

    #[fixture]
    fn mock_cmd_le_set_advertising_data_success() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[
                1, 8, 32, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
            ])
            .read(&[4, 14, 4, 1, 8, 32, 0])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_set_advertising_data_unknown_hci_command() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[
                1, 8, 32, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
            ])
            .read(&[4, 14, 4, 1, 8, 32, 1])
            .build()
    }

    #[rstest]
    #[case::success(mock_cmd_le_set_advertising_data_success(), Ok(()))]
    #[case::unknown_hci_command(
        mock_cmd_le_set_advertising_data_unknown_hci_command(),
        Err(Error::ErrorCode(ErrorCode::UnknownHciCommand))
    )]
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_cmd_le_set_advertising_data(
        #[case] mock: Mock,
        #[case] expected: Result<(), Error>,
    ) {
        let hci_driver = TokioHciDriver { hci: mock };
        let mut hci = Hci {
            driver: hci_driver,
            num_hci_command_packets: 1,
            read_buffer: Default::default(),
            event_list: Default::default(),
        };
        assert_eq!(
            hci.cmd_le_set_advertising_data(AdvertisingData::default())
                .await,
            expected
        );
    }

    #[fixture]
    fn mock_cmd_le_set_advertising_enable_success() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 10, 32, 1, 1])
            .read(&[4, 14, 4, 1, 10, 32, 0])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_set_advertising_enable_command_disallowed() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 10, 32, 1, 1])
            .read(&[4, 14, 4, 1, 10, 32, 12])
            .build()
    }

    #[rstest]
    #[case::success(mock_cmd_le_set_advertising_enable_success(), Ok(()))]
    #[case::command_disallowed(
        mock_cmd_le_set_advertising_enable_command_disallowed(),
        Err(Error::ErrorCode(ErrorCode::CommandDisallowed))
    )]
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_cmd_le_set_advertising_enable(
        #[case] mock: Mock,
        #[case] expected: Result<(), Error>,
    ) {
        let hci_driver = TokioHciDriver { hci: mock };
        let mut hci = Hci {
            driver: hci_driver,
            num_hci_command_packets: 1,
            read_buffer: Default::default(),
            event_list: Default::default(),
        };
        assert_eq!(
            hci.cmd_le_set_advertising_enable(AdvertisingEnable::Enabled)
                .await,
            expected
        );
    }

    #[fixture]
    fn mock_cmd_le_set_advertising_parameters_success() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 6, 32, 15, 0, 8, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0])
            .read(&[4, 14, 4, 1, 6, 32, 0])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_set_advertising_parameters_unknown_hci_command() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 6, 32, 15, 0, 8, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0])
            .read(&[4, 14, 4, 1, 6, 32, 1])
            .build()
    }

    #[rstest]
    #[case::success(mock_cmd_le_set_advertising_parameters_success(), Ok(()))]
    #[case::unknown_hci_command(
        mock_cmd_le_set_advertising_parameters_unknown_hci_command(),
        Err(Error::ErrorCode(ErrorCode::UnknownHciCommand))
    )]
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_cmd_le_set_advertising_parameters(
        #[case] mock: Mock,
        #[case] expected: Result<(), Error>,
    ) {
        let hci_driver = TokioHciDriver { hci: mock };
        let mut hci = Hci {
            driver: hci_driver,
            num_hci_command_packets: 1,
            read_buffer: Default::default(),
            event_list: Default::default(),
        };
        assert_eq!(
            hci.cmd_le_set_advertising_parameters(AdvertisingParameters::default())
                .await,
            expected
        );
    }

    #[fixture]
    fn mock_cmd_le_set_event_mask_success() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 1, 32, 8, 31, 0, 0, 0, 0, 0, 0, 0])
            .read(&[4, 14, 4, 1, 1, 32, 0])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_set_event_mask_command_disallowed() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 1, 32, 8, 31, 0, 0, 0, 0, 0, 0, 0])
            .read(&[4, 14, 4, 1, 1, 32, 12])
            .build()
    }

    #[rstest]
    #[case::success(mock_cmd_le_set_event_mask_success(), Ok(()))]
    #[case::command_disallowed(
        mock_cmd_le_set_event_mask_command_disallowed(),
        Err(Error::ErrorCode(ErrorCode::CommandDisallowed))
    )]
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_cmd_le_set_event_mask(#[case] mock: Mock, #[case] expected: Result<(), Error>) {
        let hci_driver = TokioHciDriver { hci: mock };
        let mut hci = Hci {
            driver: hci_driver,
            num_hci_command_packets: 1,
            read_buffer: Default::default(),
            event_list: Default::default(),
        };
        assert_eq!(
            hci.cmd_le_set_event_mask(LeEventMask::default()).await,
            expected
        );
    }

    #[fixture]
    fn mock_cmd_le_set_random_address_success() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 5, 32, 6, 68, 223, 27, 9, 83, 250])
            .read(&[4, 14, 4, 1, 5, 32, 0])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_set_random_address_command_disallowed() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 5, 32, 6, 68, 223, 27, 9, 83, 250])
            .read(&[4, 14, 4, 1, 5, 32, 12])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_set_random_address_invalid_event_packet() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 5, 32, 6, 68, 223, 27, 9, 83, 250])
            .read(&[4, 14, 7, 1, 5, 32, 0])
            .build()
    }

    #[rstest]
    #[case::success(
        mock_cmd_le_set_random_address_success(),
        Ok(())
    )]
    #[case::command_disallowed(
        mock_cmd_le_set_random_address_command_disallowed(),
        Err(Error::ErrorCode(ErrorCode::CommandDisallowed))
    )]
    #[case::invalid_event_packet(
        mock_cmd_le_set_random_address_invalid_event_packet(),
        Err(Error::InvalidPacket)
    )]
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_cmd_le_set_random_address(
        #[case] mock: Mock,
        #[case] expected: Result<(), Error>,
    ) {
        let hci_driver = TokioHciDriver { hci: mock };
        let mut hci = Hci {
            driver: hci_driver,
            num_hci_command_packets: 1,
            read_buffer: Default::default(),
            event_list: Default::default(),
        };
        assert_eq!(
            hci.cmd_le_set_random_address(
                RandomStaticDeviceAddress::try_new([68, 223, 27, 9, 83, 250]).unwrap()
            )
            .await,
            expected
        );
    }

    #[fixture]
    fn mock_cmd_le_set_scan_enable_success() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 12, 32, 2, 1, 0])
            .read(&[4, 14, 4, 1, 12, 32, 0])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_set_scan_enable_invalid_hci_command_parameters() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 12, 32, 2, 1, 0])
            .read(&[4, 14, 4, 1, 12, 32, 18])
            .build()
    }

    #[rstest]
    #[case::success(mock_cmd_le_set_scan_enable_success(), Ok(()))]
    #[case::invalid_hci_command_parameters(
        mock_cmd_le_set_scan_enable_invalid_hci_command_parameters(),
        Err(Error::ErrorCode(ErrorCode::InvalidHciCommandParameters))
    )]
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_cmd_le_set_scan_enable(#[case] mock: Mock, #[case] expected: Result<(), Error>) {
        let hci_driver = TokioHciDriver { hci: mock };
        let mut hci = Hci {
            driver: hci_driver,
            num_hci_command_packets: 1,
            read_buffer: Default::default(),
            event_list: Default::default(),
        };
        assert_eq!(
            hci.cmd_le_set_scan_enable(ScanEnable::Enabled, FilterDuplicates::Disabled)
                .await,
            expected
        );
    }

    #[fixture]
    fn mock_cmd_le_set_scan_parameters_success() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 11, 32, 7, 0, 16, 0, 16, 0, 0, 0])
            .read(&[4, 14, 4, 1, 11, 32, 0])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_set_scan_parameters_unknown_hci_command() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 11, 32, 7, 0, 16, 0, 16, 0, 0, 0])
            .read(&[4, 14, 4, 1, 11, 32, 1])
            .build()
    }

    #[rstest]
    #[case::success(mock_cmd_le_set_scan_parameters_success(), Ok(()))]
    #[case::unknown_hci_command(
        mock_cmd_le_set_scan_parameters_unknown_hci_command(),
        Err(Error::ErrorCode(ErrorCode::UnknownHciCommand))
    )]
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_cmd_le_set_scan_parameters(
        #[case] mock: Mock,
        #[case] expected: Result<(), Error>,
    ) {
        let hci_driver = TokioHciDriver { hci: mock };
        let mut hci = Hci {
            driver: hci_driver,
            num_hci_command_packets: 1,
            read_buffer: Default::default(),
            event_list: Default::default(),
        };
        assert_eq!(
            hci.cmd_le_set_scan_parameters(ScanParameters::default())
                .await,
            expected
        );
    }

    #[fixture]
    fn mock_cmd_le_set_scan_response_data_success() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[
                1, 9, 32, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
            ])
            .read(&[4, 14, 4, 1, 9, 32, 0])
            .build()
    }

    #[fixture]
    fn mock_cmd_le_set_scan_response_data_unknown_hci_command() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[
                1, 9, 32, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
            ])
            .read(&[4, 14, 4, 1, 9, 32, 1])
            .build()
    }

    #[rstest]
    #[case::success(mock_cmd_le_set_scan_response_data_success(), Ok(()))]
    #[case::unknown_hci_command(
        mock_cmd_le_set_scan_response_data_unknown_hci_command(),
        Err(Error::ErrorCode(ErrorCode::UnknownHciCommand))
    )]
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_cmd_le_set_scan_response_data(
        #[case] mock: Mock,
        #[case] expected: Result<(), Error>,
    ) {
        let hci_driver = TokioHciDriver { hci: mock };
        let mut hci = Hci {
            driver: hci_driver,
            num_hci_command_packets: 1,
            read_buffer: Default::default(),
            event_list: Default::default(),
        };
        assert_eq!(
            hci.cmd_le_set_scan_response_data(AdvertisingData::default())
                .await,
            expected
        );
    }

    #[fixture]
    fn mock_cmd_read_bd_addr_success() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 9, 16, 0])
            .read(&[4, 14, 10, 1, 9, 16, 0, 0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56])
            .build()
    }

    #[fixture]
    fn mock_cmd_read_bd_addr_unknown_hci_command() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 9, 16, 0])
            .read(&[4, 14, 4, 1, 9, 16, 1])
            .build()
    }

    #[fixture]
    fn mock_cmd_read_bd_addr_invalid_event_packet() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 9, 16, 0])
            .read(&[4, 14, 32, 1, 9, 16, 0, 32, 31, 30])
            .build()
    }

    #[rstest]
    #[case::success(
        mock_cmd_read_bd_addr_success(),
        Ok(PublicDeviceAddress::new([0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56]))
    )]
    #[case::unknown_hci_command(
        mock_cmd_read_bd_addr_unknown_hci_command(),
        Err(Error::ErrorCode(ErrorCode::UnknownHciCommand))
    )]
    #[case::invalid_event_packet(
        mock_cmd_read_bd_addr_invalid_event_packet(),
        Err(Error::InvalidPacket)
    )]
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_cmd_read_bd_addr(
        #[case] mock: Mock,
        #[case] expected: Result<PublicDeviceAddress, Error>,
    ) {
        let hci_driver = TokioHciDriver { hci: mock };
        let mut hci = Hci {
            driver: hci_driver,
            num_hci_command_packets: 1,
            read_buffer: Default::default(),
            event_list: Default::default(),
        };
        assert_eq!(hci.cmd_read_bd_addr().await, expected);
    }

    #[fixture]
    fn mock_cmd_read_buffer_size_success() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 5, 16, 0])
            .read(&[4, 14, 11, 1, 5, 16, 0, 255, 0, 255, 24, 0, 12, 0])
            .build()
    }

    #[fixture]
    fn mock_cmd_read_buffer_size_hardware_failure() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 5, 16, 0])
            .read(&[4, 14, 4, 1, 5, 16, 3])
            .build()
    }

    #[fixture]
    fn mock_cmd_read_buffer_size_invalid_event_packet() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 5, 16, 0])
            .read(&[4, 14, 2, 1, 2])
            .build()
    }

    #[rstest]
    #[case::success(
        mock_cmd_read_buffer_size_success(),
        Ok((
            NonZeroU16::new(255).unwrap(),
            NonZeroU8::new(255).unwrap(),
            NonZeroU16::new(24).unwrap(),
            12
        ))
    )]
    #[case::hardware_failure(
        mock_cmd_read_buffer_size_hardware_failure(),
        Err(Error::ErrorCode(ErrorCode::HardwareFailure))
    )]
    #[case::invalid_event_packet(
        mock_cmd_read_buffer_size_invalid_event_packet(),
        Err(Error::InvalidPacket)
    )]
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_cmd_read_buffer_size(
        #[case] mock: Mock,
        #[case] expected: Result<(NonZeroU16, NonZeroU8, NonZeroU16, u16), Error>,
    ) {
        let hci_driver = TokioHciDriver { hci: mock };
        let mut hci = Hci {
            driver: hci_driver,
            num_hci_command_packets: 1,
            read_buffer: Default::default(),
            event_list: Default::default(),
        };
        assert_eq!(hci.cmd_read_buffer_size().await, expected);
    }

    #[fixture]
    fn mock_cmd_read_local_supported_commands_success() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 2, 16, 0])
            .read(&[
                4, 14, 68, 1, 2, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 4, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ])
            .build()
    }

    #[fixture]
    fn mock_cmd_read_local_supported_commands_unknown_hci_command() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 2, 16, 0])
            .read(&[4, 14, 4, 1, 2, 16, 1])
            .build()
    }

    #[fixture]
    fn mock_cmd_read_local_supported_commands_invalid_event_packet() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 2, 16, 0])
            .read(&[4, 14, 68, 1, 2, 16, 0, 0, 0, 0])
            .build()
    }

    #[rstest]
    #[case::success(
        mock_cmd_read_local_supported_commands_success(),
        Ok(SupportedCommands::LE_RAND | SupportedCommands::LE_READ_LOCAL_SUPPORTED_FEATURES_PAGE_0)
    )]
    #[case::unknown_hci_command(
        mock_cmd_read_local_supported_commands_unknown_hci_command(),
        Err(Error::ErrorCode(ErrorCode::UnknownHciCommand))
    )]
    #[case::invalid_event_packet(
        mock_cmd_read_local_supported_commands_invalid_event_packet(),
        Err(Error::InvalidPacket)
    )]
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_cmd_read_local_supported_commands(
        #[case] mock: Mock,
        #[case] expected: Result<SupportedCommands, Error>,
    ) {
        let hci_driver = TokioHciDriver { hci: mock };
        let mut hci = Hci {
            driver: hci_driver,
            num_hci_command_packets: 1,
            read_buffer: Default::default(),
            event_list: Default::default(),
        };
        assert_eq!(hci.cmd_read_local_supported_commands().await, expected);
    }

    #[fixture]
    fn mock_cmd_read_local_supported_features_success() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 3, 16, 0])
            .read(&[4, 14, 12, 1, 3, 16, 0, 0, 0, 0, 0, 64, 0, 0, 0])
            .build()
    }

    #[fixture]
    fn mock_cmd_read_local_supported_features_hardware_failure() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 3, 16, 0])
            .read(&[4, 14, 4, 1, 3, 16, 3])
            .build()
    }

    #[fixture]
    fn mock_cmd_read_local_supported_features_invalid_event_packet() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 3, 16, 0])
            .read(&[4, 14, 2, 1, 2])
            .build()
    }

    #[rstest]
    #[case::success(
        mock_cmd_read_local_supported_features_success(),
        Ok(SupportedFeatures::LE_SUPPORTED_CONTROLLER)
    )]
    #[case::hardware_failure(
        mock_cmd_read_local_supported_features_hardware_failure(),
        Err(Error::ErrorCode(ErrorCode::HardwareFailure))
    )]
    #[case::invalid_event_packet(
        mock_cmd_read_local_supported_features_invalid_event_packet(),
        Err(Error::InvalidPacket)
    )]
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_cmd_read_local_supported_features(
        #[case] mock: Mock,
        #[case] expected: Result<SupportedFeatures, Error>,
    ) {
        let hci_driver = TokioHciDriver { hci: mock };
        let mut hci = Hci {
            driver: hci_driver,
            num_hci_command_packets: 1,
            read_buffer: Default::default(),
            event_list: Default::default(),
        };
        assert_eq!(hci.cmd_read_local_supported_features().await, expected);
    }

    #[fixture]
    fn mock_cmd_reset_success() -> Mock {
        tokio_test::io::Builder::new()
            .read(&[4, 14, 3, 1, 0, 0])
            .write(&[1, 3, 12, 0])
            .read(&[4, 14, 4, 1, 3, 12, 0])
            .build()
    }

    #[fixture]
    fn mock_cmd_reset_hardware_failure() -> Mock {
        tokio_test::io::Builder::new()
            .read(&[4, 14, 3, 1, 0, 0])
            .write(&[1, 3, 12, 0])
            .read(&[4, 14, 4, 1, 3, 12, 3])
            .build()
    }

    #[fixture]
    fn mock_cmd_reset_hci_timeout() -> Mock {
        tokio_test::io::Builder::new()
            .read(&[4, 14, 3, 1, 0, 0])
            .write(&[1, 3, 12, 0])
            .wait(Duration::from_secs(10))
            .build()
    }

    #[fixture]
    fn mock_cmd_reset_receive_command_instead_of_event() -> Mock {
        tokio_test::io::Builder::new()
            .read(&[4, 14, 3, 1, 0, 0])
            .write(&[1, 3, 12, 0])
            .read(&[1, 3, 12, 0])
            .build()
    }

    #[fixture]
    fn mock_cmd_reset_receive_other_event_before_command_complete_event() -> Mock {
        tokio_test::io::Builder::new()
            .read(&[4, 14, 3, 1, 0, 0])
            .write(&[1, 3, 12, 0])
            .read(&[4, 14, 3, 1, 0, 0])
            .read(&[4, 14, 4, 1, 3, 12, 0])
            .build()
    }

    #[fixture]
    fn mock_cmd_reset_receive_command_status_event_instead_of_command_complete_event() -> Mock {
        tokio_test::io::Builder::new()
            .read(&[4, 14, 3, 1, 0, 0])
            .write(&[1, 3, 12, 0])
            .read(&[4, 15, 4, 0, 1, 3, 12])
            .build()
    }

    #[fixture]
    fn mock_cmd_reset_receive_command_instead_of_event_while_waiting_for_controller() -> Mock {
        tokio_test::io::Builder::new().read(&[1, 3, 12, 0]).build()
    }

    #[fixture]
    fn mock_cmd_reset_receive_acl_data_instead_of_event_while_waiting_for_controller() -> Mock {
        tokio_test::io::Builder::new()
            .read(&[
                2, 0, 32, 16, 0, 12, 0, 5, 0, 18, 1, 8, 0, 24, 0, 40, 0, 0, 0, 42, 0,
            ])
            .build()
    }

    #[fixture]
    fn mock_cmd_reset_receive_invalid_packet_while_waiting_for_controller() -> Mock {
        tokio_test::io::Builder::new()
            .read(&[7, 0, 16, 31, 2, 15])
            .build()
    }

    #[fixture]
    fn mock_cmd_reset_receive_unhandled_event() -> Mock {
        tokio_test::io::Builder::new()
            .read(&[4, 14, 3, 1, 0, 0])
            .write(&[1, 3, 12, 0])
            .read(&[4, 1, 3, 1, 0, 0])
            .read(&[4, 14, 4, 1, 3, 12, 0])
            .build()
    }

    #[rstest]
    #[case::success(mock_cmd_reset_success(), Ok(()))]
    #[case::hardware_failure(
        mock_cmd_reset_hardware_failure(),
        Err(Error::ErrorCode(ErrorCode::HardwareFailure))
    )]
    #[case::timeout(
        mock_cmd_reset_hci_timeout(),
        Err(Error::HciDriver(HciDriverError::Timeout))
    )]
    #[case::receive_command_instead_of_event(
        mock_cmd_reset_receive_command_instead_of_event(),
        Err(Error::InvalidPacket)
    )]
    #[case::receive_other_event_before_command_complete_event(
        mock_cmd_reset_receive_other_event_before_command_complete_event(),
        Ok(())
    )]
    #[case::receive_command_status_event_instead_of_command_complete_event(
        mock_cmd_reset_receive_command_status_event_instead_of_command_complete_event(),
        Err(Error::UnexpectedEvent)
    )]
    #[case::receive_command_instead_of_event_while_waiting_for_controller(
        mock_cmd_reset_receive_command_instead_of_event_while_waiting_for_controller(),
        Err(Error::InvalidPacket)
    )]
    #[case::receive_acl_data_instead_of_event_while_waiting_for_controller(
        mock_cmd_reset_receive_acl_data_instead_of_event_while_waiting_for_controller(),
        Err(Error::InvalidPacket)
    )]
    #[case::receive_invalid_packet_while_waiting_for_controller(
        mock_cmd_reset_receive_invalid_packet_while_waiting_for_controller(),
        Err(Error::InvalidPacket)
    )]
    #[case::receive_unhandled_event(
        mock_cmd_reset_receive_unhandled_event(),
        Ok(())
    )]
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_cmd_reset(#[case] mock: Mock, #[case] expected: Result<(), Error>) {
        let hci_driver = TokioHciDriver { hci: mock };
        let mut hci = Hci::new(hci_driver);
        assert_eq!(hci.cmd_reset().await, expected);
    }

    #[fixture]
    fn mock_cmd_set_event_mask_success() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 1, 12, 8, 0, 128, 0, 2, 0, 0, 0, 0])
            .read(&[4, 14, 4, 1, 1, 12, 0])
            .build()
    }

    #[fixture]
    fn mock_cmd_set_event_mask_command_disallowed() -> Mock {
        tokio_test::io::Builder::new()
            .write(&[1, 1, 12, 8, 0, 128, 0, 2, 0, 0, 0, 0])
            .read(&[4, 14, 4, 1, 1, 12, 12])
            .build()
    }

    #[rstest]
    #[case::success(mock_cmd_set_event_mask_success(), Ok(()))]
    #[case::command_disallowed(
        mock_cmd_set_event_mask_command_disallowed(),
        Err(Error::ErrorCode(ErrorCode::CommandDisallowed))
    )]
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_cmd_set_event_mask(#[case] mock: Mock, #[case] expected: Result<(), Error>) {
        let hci_driver = TokioHciDriver { hci: mock };
        let mut hci = Hci {
            driver: hci_driver,
            num_hci_command_packets: 1,
            read_buffer: Default::default(),
            event_list: Default::default(),
        };
        assert_eq!(
            hci.cmd_set_event_mask(EventMask::HARDWARE_ERROR | EventMask::DATA_BUFFER_OVERFLOW)
                .await,
            expected
        );
    }
}
