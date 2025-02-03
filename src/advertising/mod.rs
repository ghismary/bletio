//! Advertising parameters and data handling.
//!
//! This module gives access to all that is need to start advertising:
//!  - definition of the [advertising parameters](crate::advertising::advertising_parameters)
//!  - definition of all the [advertising structures](crate::advertising::ad_struct) to be used in the [`AdvertisingData`] or [`ScanResponseData`] packets.

pub mod ad_struct;
pub mod advertising_data;
pub mod advertising_parameters;

pub use ad_struct::flags::Flags;
pub use ad_struct::tx_power_level::TxPowerLevel;
pub use advertising_data::{AdvertisingData, ScanResponseData};
pub use advertising_parameters::AdvertisingIntervalValue;

use crate::utils::EncodeToBuffer;

/// Error occuring in the advertising part of the BLE stack.
#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdvertisingError {
    /// The advertising structure must be present only once in the advertising data or scan response data.
    #[error("The advertising structure must be present only once in the advertising data or scan response data")]
    AdStructAlreadyPresent,
    /// The provided advertising data is too big to fit in an advertising data or scan response data packet.
    #[error("The provided advertising data is too big to fit in an advertising data or scan response data packet")]
    AdvertisingDataWillNotFitAdvertisingPacket,
    /// An empty service UUID list Advertising Structure needs to be complete.
    #[error("An empty service UUID list Advertising Structure needs to be complete")]
    EmptyServiceUuidListShallBeComplete,
    /// Internal error.
    #[error("Internal advertising error: {0}")]
    Internal(&'static str),
    /// The provided advertising enable value is invalid.
    #[error("The advertising enable value {0} is invalid")]
    /// The provided advertising filter policy is invalid.
    InvalidAdvertisingEnableValue(u8),
    #[error("The advertising filter policy {0} is invalid")]
    InvalidAdvertisingFilterPolicy(u8),
    /// The provided advertising interval value is invalid, it needs to be between 0x0020 and 0x4000.
    #[error(
        "The advertising interval value {0} is invalid, it needs to be between 0x0020 and 0x4000"
    )]
    InvalidAdvertisingIntervalValue(u16),
    /// The advertising parameters are not valid, probably because the advertising type is ScannableUndirected or NonConnectableUndirected, and the minimum advertising interval value is less than 0x00A0.
    #[error("The advertising parameters are not valid, probably because the advertising type is ScannableUndirected or NonConnectableUndirected, and the minimum advertising interval value is less than 0x00A0")]
    InvalidAdvertisingParameters,
    /// The provided advertising type is invalid.
    #[error("The advertising type {0} is invalid")]
    InvalidAdvertisingType(u8),
    /// The provided custom URI scheme is not valid.
    #[error("The URI scheme \"{0}\" is not valid")]
    InvalidCustomUriScheme(&'static str),
    /// The provided own address type is invalid.
    #[error("The own address type {0} is invalid")]
    InvalidOwnAddressType(u8),
    /// The provided peer address type is invalid.
    #[error("The peer address type {0} is invalid")]
    InvalidPeerAddressType(u8),
}

/// Enable/disable advertising.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.9](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e58c6816-c25e-367a-0023-9da1700a3794).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(u8)]
#[non_exhaustive]
pub(crate) enum AdvertisingEnable {
    #[default]
    /// Advertising is disabled (default).
    Disabled = 0x00,
    /// Advertising is enabled.
    Enabled = 0x01,
}

impl EncodeToBuffer for AdvertisingEnable {
    fn encode<B: crate::utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, crate::utils::UtilsError> {
        buffer.try_push(*self as u8)
    }
}

impl TryFrom<u8> for AdvertisingEnable {
    type Error = AdvertisingError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(AdvertisingEnable::Disabled),
            0x01 => Ok(AdvertisingEnable::Enabled),
            _ => Err(AdvertisingError::InvalidAdvertisingEnableValue(value)),
        }
    }
}

pub(crate) mod parser {
    use nom::{
        combinator::{all_consuming, map_res},
        number::le_u8,
        IResult, Parser,
    };

    use super::AdvertisingEnable;

    pub(crate) fn advertising_enable(input: &[u8]) -> IResult<&[u8], AdvertisingEnable> {
        all_consuming(map_res(le_u8(), TryInto::try_into)).parse(input)
    }
}
