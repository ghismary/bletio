use core::{
    num::{NonZeroU16, NonZeroU8},
    time::Duration,
};

use crate::{
    AdvertisingData, AdvertisingEnable, AdvertisingParameters, Command, CommandCompleteEvent,
    CommandOpCode, Error, Event, EventList, EventMask, EventParameter, FilterDuplicates, HciBuffer,
    HciDriver, LeEventMask, LeFilterAcceptListAddress, Packet, PublicDeviceAddress,
    RandomStaticDeviceAddress, ScanEnable, ScanParameters, ScanResponseData, SupportedCommands,
    SupportedFeatures, SupportedLeFeatures, SupportedLeStates, TxPowerLevel, WithTimeout,
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

    pub async fn cmd_le_add_device_to_filter_accept_list(
        &mut self,
        address: impl Into<LeFilterAcceptListAddress>,
    ) -> Result<(), Error> {
        let event = self
            .execute_command(Command::LeAddDeviceToFilterAcceptList(address.into()))
            .await?;
        match event.parameter {
            EventParameter::Status(param) if param.status.is_success() => Ok(()),
            EventParameter::Status(param) => Err(Error::ErrorCode(param.status)),
            _ => unreachable!("parsing would have failed"),
        }
    }

    pub async fn cmd_le_clear_filter_accept_list(&mut self) -> Result<(), Error> {
        let event = self
            .execute_command(Command::LeClearFilterAcceptList)
            .await?;
        match event.parameter {
            EventParameter::Status(param) if param.status.is_success() => Ok(()),
            EventParameter::Status(param) => Err(Error::ErrorCode(param.status)),
            _ => unreachable!("parsing would have failed"),
        }
    }

    pub async fn cmd_le_rand(&mut self) -> Result<[u8; 8], Error> {
        let event = self.execute_command(Command::LeRand).await?;
        match event.parameter {
            EventParameter::StatusAndRandomNumber(param) if param.status.is_success() => {
                Ok(param.random_number)
            }
            EventParameter::StatusAndRandomNumber(param) => Err(Error::ErrorCode(param.status)),
            _ => unreachable!("parsing would have failed"),
        }
    }

    pub async fn cmd_le_read_advertising_channel_tx_power(
        &mut self,
    ) -> Result<TxPowerLevel, Error> {
        let event = self
            .execute_command(Command::LeReadAdvertisingChannelTxPower)
            .await?;
        match event.parameter {
            EventParameter::StatusAndTxPowerLevel(param) if param.status.is_success() => {
                Ok(param.tx_power_level)
            }
            EventParameter::StatusAndTxPowerLevel(param) => Err(Error::ErrorCode(param.status)),
            _ => unreachable!("parsing would have failed"),
        }
    }

    pub async fn cmd_le_read_buffer_size(&mut self) -> Result<(u16, u16), Error> {
        let event = self.execute_command(Command::LeReadBufferSize).await?;
        match event.parameter {
            EventParameter::StatusAndLeBufferSize(param) if param.status.is_success() => Ok((
                param.le_acl_data_packet_length,
                param.total_num_le_acl_data_packets as u16,
            )),
            EventParameter::StatusAndLeBufferSize(param) => Err(Error::ErrorCode(param.status)),
            _ => unreachable!("parsing would have failed"),
        }
    }

    pub async fn cmd_le_read_filter_accept_list_size(&mut self) -> Result<usize, Error> {
        let event = self
            .execute_command(Command::LeReadFilterAcceptListSize)
            .await?;
        match event.parameter {
            EventParameter::StatusAndFilterAcceptListSize(param) if param.status.is_success() => {
                Ok(param.filter_accept_list_size)
            }
            EventParameter::StatusAndFilterAcceptListSize(param) => {
                Err(Error::ErrorCode(param.status))
            }
            _ => unreachable!("parsing would have failed"),
        }
    }

    pub async fn cmd_le_read_local_supported_features_page_0(
        &mut self,
    ) -> Result<SupportedLeFeatures, Error> {
        let event = self
            .execute_command(Command::LeReadLocalSupportedFeaturesPage0)
            .await?;
        match event.parameter {
            EventParameter::StatusAndSupportedLeFeatures(param) if param.status.is_success() => {
                Ok(param.supported_le_features)
            }
            EventParameter::StatusAndSupportedLeFeatures(param) => {
                Err(Error::ErrorCode(param.status))
            }
            _ => unreachable!("parsing would have failed"),
        }
    }

    pub async fn cmd_le_read_supported_states(&mut self) -> Result<SupportedLeStates, Error> {
        let event = self.execute_command(Command::LeReadSupportedStates).await?;
        match event.parameter {
            EventParameter::StatusAndSupportedLeStates(param) if param.status.is_success() => {
                Ok(param.supported_le_states)
            }
            EventParameter::StatusAndSupportedLeStates(param) => {
                Err(Error::ErrorCode(param.status))
            }
            _ => unreachable!("parsing would have failed"),
        }
    }

    pub async fn cmd_le_remove_device_from_filter_accept_list(
        &mut self,
        address: impl Into<LeFilterAcceptListAddress>,
    ) -> Result<(), Error> {
        let event = self
            .execute_command(Command::LeRemoveDeviceFromFilterAcceptList(address.into()))
            .await?;
        match event.parameter {
            EventParameter::Status(param) if param.status.is_success() => Ok(()),
            EventParameter::Status(param) => Err(Error::ErrorCode(param.status)),
            _ => unreachable!("parsing would have failed"),
        }
    }

    pub async fn cmd_le_set_advertising_data(
        &mut self,
        data: AdvertisingData,
    ) -> Result<(), Error> {
        self.cmd_with_status_response(Command::LeSetAdvertisingData(data))
            .await
    }

    pub async fn cmd_le_set_advertising_enable(
        &mut self,
        enable: AdvertisingEnable,
    ) -> Result<(), Error> {
        self.cmd_with_status_response(Command::LeSetAdvertisingEnable(enable))
            .await
    }

    pub async fn cmd_le_set_advertising_parameters(
        &mut self,
        parameters: AdvertisingParameters,
    ) -> Result<(), Error> {
        self.cmd_with_status_response(Command::LeSetAdvertisingParameters(parameters))
            .await
    }

    pub async fn cmd_le_set_random_address(
        &mut self,
        address: RandomStaticDeviceAddress,
    ) -> Result<(), Error> {
        self.cmd_with_status_response(Command::LeSetRandomAddress(address))
            .await
    }

    pub async fn cmd_le_set_scan_enable(
        &mut self,
        scan_enable: ScanEnable,
        filter_duplicates: FilterDuplicates,
    ) -> Result<(), Error> {
        self.cmd_with_status_response(Command::LeSetScanEnable(scan_enable, filter_duplicates))
            .await
    }

    pub async fn cmd_le_set_scan_parameters(
        &mut self,
        parameters: ScanParameters,
    ) -> Result<(), Error> {
        self.cmd_with_status_response(Command::LeSetScanParameters(parameters))
            .await
    }

    pub async fn cmd_le_set_scan_response_data(
        &mut self,
        data: ScanResponseData,
    ) -> Result<(), Error> {
        self.cmd_with_status_response(Command::LeSetScanResponseData(data))
            .await
    }

    pub async fn cmd_le_set_event_mask(&mut self, data: LeEventMask) -> Result<(), Error> {
        self.cmd_with_status_response(Command::LeSetEventMask(data))
            .await
    }

    pub async fn cmd_read_bd_addr(&mut self) -> Result<PublicDeviceAddress, Error> {
        let event = self.execute_command(Command::ReadBdAddr).await?;
        match event.parameter {
            EventParameter::StatusAndBdAddr(param) if param.status.is_success() => {
                Ok(param.bd_addr)
            }
            EventParameter::StatusAndBdAddr(param) => Err(Error::ErrorCode(param.status)),
            _ => unreachable!("parsing would have failed"),
        }
    }

    pub async fn cmd_read_buffer_size(
        &mut self,
    ) -> Result<(NonZeroU16, NonZeroU8, NonZeroU16, u16), Error> {
        let event = self.execute_command(Command::ReadBufferSize).await?;
        match event.parameter {
            EventParameter::StatusAndBufferSize(param) if param.status.is_success() => Ok((
                param.acl_data_packet_length,
                param.synchronous_data_packet_length,
                param.total_num_acl_data_packets,
                param.total_num_synchronous_packets,
            )),
            EventParameter::StatusAndBufferSize(param) => Err(Error::ErrorCode(param.status)),
            _ => unreachable!("parsing would have failed"),
        }
    }

    pub async fn cmd_read_local_supported_commands(&mut self) -> Result<SupportedCommands, Error> {
        let event = self
            .execute_command(Command::ReadLocalSupportedCommands)
            .await?;
        match event.parameter {
            EventParameter::StatusAndSupportedCommands(param) if param.status.is_success() => {
                Ok(param.supported_commands)
            }
            EventParameter::StatusAndSupportedCommands(param) => {
                Err(Error::ErrorCode(param.status))
            }
            _ => unreachable!("parsing would have failed"),
        }
    }

    pub async fn cmd_read_local_supported_features(&mut self) -> Result<SupportedFeatures, Error> {
        let event = self
            .execute_command(Command::ReadLocalSupportedFeatures)
            .await?;
        match event.parameter {
            EventParameter::StatusAndSupportedFeatures(param) if param.status.is_success() => {
                Ok(param.supported_features)
            }
            EventParameter::StatusAndSupportedFeatures(param) => {
                Err(Error::ErrorCode(param.status))
            }
            _ => unreachable!("parsing would have failed"),
        }
    }

    pub async fn cmd_reset(&mut self) -> Result<(), Error> {
        self.cmd_with_status_response(Command::Reset).await
    }

    pub async fn cmd_set_event_mask(&mut self, event_mask: EventMask) -> Result<(), Error> {
        self.cmd_with_status_response(Command::SetEventMask(event_mask))
            .await
    }

    pub async fn wait_for_event(&mut self) -> Result<EventList, Error> {
        let mut event_list = core::mem::take(&mut self.event_list);

        loop {
            if (self.read_buffer.is_empty() && !event_list.is_empty()) || event_list.is_full() {
                return Ok(event_list);
            }

            match self.hci_read_and_parse_packet().await {
                Ok((remaining, packet)) => {
                    match packet {
                        Packet::Command(_) => {
                            // The Host is not supposed to receive commands, ignore it!
                            #[cfg(feature = "defmt")]
                            defmt::warn!("Received command while waiting for event, ignore it!");
                        }
                        Packet::Event(event) => {
                            // INVARIANT: The remaining is known to be shorter than the buffer.
                            self.read_buffer = remaining.try_into().unwrap();

                            // INVARIANT: The event list is known to be able to hold this event,
                            // otherwise we would have returned at the beginning of the loop.
                            event_list.push(event).unwrap();
                        }
                    }
                }
                Err(e) => {
                    self.read_buffer.clear();
                    return Err(e);
                }
            }
        }
    }

    async fn cmd_with_status_response(&mut self, command: Command) -> Result<(), Error> {
        let event = self.execute_command(command).await?;
        match event.parameter {
            EventParameter::Status(param) if param.status.is_success() => Ok(()),
            EventParameter::Status(param) => Err(Error::ErrorCode(param.status)),
            _ => unreachable!("parsing would have failed"),
        }
    }

    async fn execute_command(&mut self, command: Command) -> Result<CommandCompleteEvent, Error> {
        if self.num_hci_command_packets == 0 {
            self.wait_controller_ready().await?;
        }
        let event = self
            .send_command_and_wait_response(command)
            .with_timeout(HCI_COMMAND_TIMEOUT)
            .await??;
        self.num_hci_command_packets = event.num_hci_command_packets;
        Ok(event)
    }

    async fn send_command_and_wait_response(
        &mut self,
        command: Command,
    ) -> Result<CommandCompleteEvent, Error> {
        let command_packet = command.encode()?;
        self.driver.write(command_packet.data()).await?;
        loop {
            match self.hci_read_and_parse_packet().await {
                Ok((remaining, packet)) => {
                    match packet {
                        Packet::Command(_) => {
                            // The Host is not supposed to receive commands!
                            return Err(Error::InvalidPacket);
                        }
                        Packet::Event(event) => {
                            // INVARIANT: The remaining is known to be shorter than the buffer.
                            self.read_buffer = remaining.try_into().unwrap();

                            match event {
                                Event::CommandComplete(event)
                                    if event.opcode == command.opcode() =>
                                {
                                    return Ok(event);
                                }
                                _ => self.handle_event(event),
                            }
                        }
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
            match self.hci_read_and_parse_packet().await {
                Ok((remaining, packet)) => {
                    match packet {
                        Packet::Command(_) => {
                            // The Host is not supposed to receive commands!
                            return Err(Error::InvalidPacket);
                        }
                        Packet::Event(event) => {
                            // INVARIANT: The remaining is known to be shorter than the buffer.
                            self.read_buffer = remaining.try_into().unwrap();

                            self.handle_event(event)
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

    async fn hci_read_and_parse_packet(&mut self) -> Result<(&[u8], Packet), Error> {
        if self.read_buffer.is_empty() {
            self.read_buffer.read(&mut self.driver).await?;
        }
        let (remaining, hci_packet) = crate::packet::parser::packet(self.read_buffer.data())
            .map_err(|_| Error::InvalidPacket)?;
        Ok((remaining, hci_packet))
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::CommandComplete(command_complete_event)
                if command_complete_event.opcode == CommandOpCode::Nop =>
            {
                self.num_hci_command_packets = command_complete_event.num_hci_command_packets;
            }
            Event::CommandComplete(_) => {
                unreachable!("an event for an issued command should already have been handled before reaching here")
            }
            Event::Unsupported(_) => {
                // Ignore unsupported event
            }
            _ => {
                // Other events will be handled higher in the stack
                if self.event_list.push(event).is_err() {
                    #[cfg(feature = "defmt")]
                    defmt::warn!("HCI event list is full, cannot add more!");
                }
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
    use crate::{DeviceAddress, ErrorCode, HciDriverError};

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
            hci.cmd_le_set_scan_response_data(ScanResponseData::default())
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
    fn mock_cmd_reset_receive_command_instead_of_event_while_waiting_for_controller() -> Mock {
        tokio_test::io::Builder::new().read(&[1, 3, 12, 0]).build()
    }

    #[fixture]
    fn mock_cmd_reset_receive_unandled_event() -> Mock {
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
    #[case::receive_command_instead_of_event_while_waiting_for_controller(
        mock_cmd_reset_receive_command_instead_of_event_while_waiting_for_controller(),
        Err(Error::InvalidPacket)
    )]
    #[case::receive_unhandled_event(
        mock_cmd_reset_receive_unandled_event(),
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
