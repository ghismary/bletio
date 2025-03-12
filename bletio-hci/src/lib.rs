#![no_std]

pub mod common;

mod acl_data;
mod advertising;
mod command;
mod connection;
mod error;
mod error_code;
mod event;
mod hci;
mod hci_buffer;
mod packet;
mod scanning;
mod traits;

#[cfg(all(feature = "embassy", feature = "tokio"))]
compile_error!("feature \"embassy\" and feature \"tokio\" cannot be enabled at the same time");

#[cfg(feature = "embassy")]
mod timeout_embassy;
#[cfg(feature = "tokio")]
mod timeout_tokio;

pub(crate) use command::{Command, CommandOpCode};
pub(crate) use common::peer_address_type::PeerAddressType;
pub(crate) use event::command_complete::EventParameter;
pub(crate) use hci_buffer::HciBuffer;
pub(crate) use packet::{Packet, PacketType};

pub use acl_data::{AclData, BroadcastFlag, PacketBoundaryFlag};
pub use advertising::{
    advertising_data::AdvertisingData,
    advertising_enable::AdvertisingEnable,
    advertising_parameters::{
        advertising_interval_range, AdvertisingChannelMap, AdvertisingFilterPolicy,
        AdvertisingInterval, AdvertisingIntervalRange, AdvertisingParameters, AdvertisingType,
    },
};
pub use common::{
    device_address::{
        DeviceAddress, PublicDeviceAddress, RandomAddress, RandomNonResolvablePrivateAddress,
        RandomResolvablePrivateAddress, RandomStaticDeviceAddress,
    },
    event_mask::EventMask,
    le_event_mask::LeEventMask,
    le_filter_accept_list_address::LeFilterAcceptListAddress,
    le_states::{LeCombinedState, LeSingleState, LeState},
    own_address_type::OwnAddressType,
    rssi::Rssi,
    supported_commands::SupportedCommands,
    supported_features::SupportedFeatures,
    supported_le_features::SupportedLeFeatures,
    supported_le_states::SupportedLeStates,
    tx_power_level::TxPowerLevel,
};
pub use connection::{
    connection_handle::ConnectionHandle,
    connection_interval::{
        connection_interval, connection_interval_range, ConnectionInterval, ConnectionIntervalRange,
    },
    connection_parameters::{
        connection_event_length_range, ConnectionEventLength, ConnectionEventLengthRange,
        ConnectionParameters, InitiatorFilterPolicy,
    },
    connection_peer_address::ConnectionPeerAddress,
    latency::{latency, Latency},
    reason::Reason,
    supervision_timeout::{supervision_timeout, SupervisionTimeout},
};
pub use error::Error;
pub use error_code::ErrorCode;
pub use event::{
    command_complete::CommandCompleteEvent,
    le_advertising_report::{
        LeAdvertisingReport, LeAdvertisingReportData, LeAdvertisingReportEventType,
        LeAdvertisingReportList,
    },
    le_connection_complete::{CentralClockAccuracy, LeConnectionCompleteEvent, Role},
    le_meta::LeMetaEvent,
    Event, EventList,
};
pub use hci::Hci;
pub use scanning::{
    scan_enable::{FilterDuplicates, ScanEnable},
    scan_interval::{scan_interval, ScanInterval},
    scan_parameters::{ScanParameters, ScanType, ScanningFilterPolicy},
    scan_window::{scan_window, ScanWindow},
};
pub use traits::{HciDriver, HciDriverError, WithTimeout};

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
