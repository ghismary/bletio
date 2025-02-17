#![no_std]

pub mod advertising_parameters;
pub mod scan_parameters;

mod advertising_data;
mod advertising_enable;
mod command;
mod connection_interval;
mod device_address;
mod error;
mod error_code;
mod event;
mod event_mask;
mod hci;
mod hci_buffer;
mod le_event_mask;
mod le_states;
mod own_address_type;
mod packet;
mod rssi;
mod scan_enable;
mod supported_commands;
mod supported_features;
mod supported_le_features;
mod supported_le_states;
mod traits;
mod tx_power_level;

#[cfg(all(feature = "embassy", feature = "tokio"))]
compile_error!("feature \"embassy\" and feature \"tokio\" cannot be enabled at the same time");

#[cfg(feature = "embassy")]
mod timeout_embassy;
#[cfg(feature = "tokio")]
mod timeout_tokio;

pub(crate) use command::{Command, CommandOpCode};
pub(crate) use event::command_complete::EventParameter;
pub(crate) use hci_buffer::HciBuffer;
pub(crate) use packet::{Packet, PacketType};

pub use advertising_data::{AdvertisingData, ScanResponseData};
pub use advertising_enable::AdvertisingEnable;
pub use advertising_parameters::{
    advertising_interval_range, AdvertisingChannelMap, AdvertisingFilterPolicy,
    AdvertisingInterval, AdvertisingIntervalRange, AdvertisingParameters, AdvertisingType,
};
pub use connection_interval::ConnectionInterval;
pub use device_address::{
    DeviceAddress, PublicDeviceAddress, RandomAddress, RandomNonResolvablePrivateAddress,
    RandomResolvablePrivateAddress, RandomStaticDeviceAddress,
};
pub use error::Error;
pub use error_code::ErrorCode;
pub use event::{
    command_complete::CommandCompleteEvent,
    le_advertising_report::{
        LeAdvertisingReportAddress, LeAdvertisingReportData, LeAdvertisingReportEventType,
        LeAdvertisingReportList,
    },
    le_meta::LeMetaEvent,
    Event,
};
pub use event_mask::EventMask;
pub use hci::Hci;
pub use le_event_mask::LeEventMask;
pub use le_states::{LeCombinedState, LeSingleState, LeState};
pub use own_address_type::OwnAddressType;
pub use rssi::Rssi;
pub use scan_enable::{FilterDuplicates, ScanEnable};
pub use scan_parameters::{
    scan_interval, scan_window, ScanInterval, ScanParameters, ScanType, ScanWindow,
    ScanningFilterPolicy,
};
pub use supported_commands::SupportedCommands;
pub use supported_features::SupportedFeatures;
pub use supported_le_features::SupportedLeFeatures;
pub use supported_le_states::SupportedLeStates;
pub use traits::{HciDriver, HciDriverError, WithTimeout};
pub use tx_power_level::TxPowerLevel;

#[cfg(test)]
mod test {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    use super::*;

    pub(crate) struct TokioHciDriver<H>
    where
        H: tokio::io::AsyncRead + tokio::io::AsyncWrite,
    {
        pub(crate) hci: H,
    }

    impl<H> HciDriver for TokioHciDriver<H>
    where
        H: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
    {
        async fn read(&mut self, buf: &mut [u8]) -> Result<usize, HciDriverError> {
            let len = self
                .hci
                .read(buf)
                .await
                .map_err(|_| HciDriverError::ReadFailure)?;
            Ok(len)
        }

        async fn write(&mut self, buf: &[u8]) -> Result<usize, HciDriverError> {
            self.hci
                .write(buf)
                .await
                .map_err(|_| HciDriverError::WriteFailure)
        }
    }
}
