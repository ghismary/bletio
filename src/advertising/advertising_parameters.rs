//! Advertising Parameters.
//!
//! These Advertising Parameters need to be defined to start advertising.
//!
//! # Examples
//!
//! Setting the advertising type and interval:
//! ```
//! # use bletio::advertising::advertising_parameters::{AdvertisingParameters, AdvertisingType};
//! # fn main() -> Result<(), bletio::Error> {
//! let adv_params = AdvertisingParameters::builder()
//!     .with_type(AdvertisingType::NonConnectableUndirected)
//!     .with_interval(0x100.try_into()?..=0x100.try_into()?)
//!     .try_build()?;
//! # Ok(())
//! # }
//! ```
//!
//! Using the default advertising parameters:
//! ```
//! # use bletio::advertising::advertising_parameters::AdvertisingParameters;
//! let adv_params = AdvertisingParameters::default();
//! ```

use bitflags::bitflags;
use core::ops::RangeInclusive;

use crate::utils::encode_le_u16;
use crate::Error;

/// Advertising interval value.
///
/// Used for undirected and low duty cycle directed advertising.
///
/// Here are the characteristics of this advertising interval value:
///  - Range: 0x0020 to 0x4000
///  - Default: 0x0800 (1.28 s)
///  - Time = N × 0.625 ms
///  - Time Range: 20 ms to 10.24 s
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.5](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3142c154-1bdd-37b2-cc6e-006aa755f5f7).
#[derive(Debug, Copy, Clone)]
pub struct AdvertisingIntervalValue {
    value: u16,
}

impl AdvertisingIntervalValue {
    /// Create a valid advertising interval value.
    pub fn try_new(value: u16) -> Result<Self, Error> {
        value.try_into()
    }

    /// Get the value of the advertising interval value in milliseconds.
    pub fn milliseconds(&self) -> f32 {
        (self.value as f32) * 0.625
    }
}

impl Default for AdvertisingIntervalValue {
    fn default() -> Self {
        Self { value: 0x0800 }
    }
}

impl TryFrom<u16> for AdvertisingIntervalValue {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if (0x0020..=0x4000).contains(&value) {
            Ok(Self { value })
        } else {
            Err(Error::InvalidAdvertisingIntervalValue(value))
        }
    }
}

/// Advertising type.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.5](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3142c154-1bdd-37b2-cc6e-006aa755f5f7).
#[derive(Debug, Copy, Clone, Default)]
#[repr(u8)]
#[non_exhaustive]
pub enum AdvertisingType {
    /// Connectable and scannable undirected advertising (`ADV_IND`) (default).
    #[default]
    ConnectableUndirected = 0x00,
    /// Connectable high duty cycle directed advertising (`ADV_DIRECT_IND`, high duty cycle).
    ConnectableHighDutyCycleDirected = 0x01,
    /// Scannable undirected advertising (`ADV_SCAN_IND`).
    ScannableUndirected = 0x02,
    /// Non connectable undirected advertising (`ADV_NONCONN_IND`).
    NonConnectableUndirected = 0x03,
    /// Connectable low duty cycle directed advertising (`ADV_DIRECT_IND`, low duty cycle).
    ConnectableLowDutyCycleDirected = 0x04,
}

/// Own address type.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.5](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3142c154-1bdd-37b2-cc6e-006aa755f5f7).
#[derive(Debug, Copy, Clone, Default)]
#[repr(u8)]
#[non_exhaustive]
pub enum OwnAddressType {
    /// Public Device Address (default).
    #[default]
    PublicDeviceAddress = 0x00,
    /// Random Device Address.
    RandomDeviceAddress = 0x01,
    /// Controller generates Resolvable Private Address based on the local IRK from the resolving list.
    /// If the resolving list contains no matching entry, use the public address.
    GeneratedResolvablePrivateAddressFallbackPublic = 0x02,
    /// Controller generates Resolvable Private Address based on the local IRK from the resolving list.
    /// If the resolving list contains no matching entry, use the random address from `LE_Set_Random_Address`.
    GeneratedResolvablePrivateAddressFallbackRandom = 0x03,
}

/// Peer address type.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.5](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3142c154-1bdd-37b2-cc6e-006aa755f5f7).
#[derive(Debug, Copy, Clone, Default)]
#[repr(u8)]
#[non_exhaustive]
pub enum PeerAddressType {
    /// Public Device Address (default) or Public Identity Address.
    #[default]
    Public = 0x00,
    /// Random Device Address or Random (static) Identity Address.
    Random = 0x01,
}

/// Peer address.
///
/// This is the address of the device to be connected.
///
/// Can be:
///  - Public Device Address
///  - Random Device Address
///  - Public Identity Address
///  - Random (static) Identity Address
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.5](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3142c154-1bdd-37b2-cc6e-006aa755f5f7).
#[derive(Debug, Clone, Default)]
pub struct PeerAddress {
    value: [u8; 6],
}

/// Channel map of the channels to use for advertising.
///
/// Defaults to all the 3 channels (37, 38 & 39).
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.5](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3142c154-1bdd-37b2-cc6e-006aa755f5f7).
#[derive(Debug, Copy, Clone)]
pub struct AdvertisingChannelMap(u8);

bitflags! {
    impl AdvertisingChannelMap: u8 {
        /// Channel 37 shall be used.
        const CHANNEL37 = 1 << 0;
        /// Channel 38 shall be used.
        const CHANNEL38 = 1 << 1;
        /// Channel 39 shall be used.
        const CHANNEL39 = 1 << 2;
    }
}

impl AdvertisingChannelMap {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for AdvertisingChannelMap {
    fn default() -> Self {
        Self::all()
    }
}

/// Advertising Filter Policy.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.5](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3142c154-1bdd-37b2-cc6e-006aa755f5f7).
#[derive(Debug, Copy, Clone, Default)]
#[repr(u8)]
#[non_exhaustive]
pub enum AdvertisingFilterPolicy {
    /// Process scan and connection requests from all devices (i.e., the Filter Accept List is not in use) (default).
    #[default]
    ScanAllAndConnectionAll = 0x00,
    /// Process connection requests from all devices and scan requests only from devices that are in the Filter Accept List.
    ConnectionAllAndScanFilterAcceptList = 0x01,
    /// Process scan requests from all devices and connection requests only from devices that are in the Filter Accept List.
    ScanAllAndConnectionFilterAcceptList = 0x02,
    /// Process scan and connection requests only from devices in the Filter Accept List.
    ScanWhiteListAndConnectionFilterAcceptList = 0x03,
}

/// A builder to create [`AdvertisingParameters`].
#[derive(Debug, Default)]
pub struct AdvertisingParametersBuilder {
    obj: AdvertisingParameters,
}

impl AdvertisingParametersBuilder {
    /// Create a builder to instantiate [`AdvertisingParameters`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Try building the [`AdvertisingParameters`], checking that every set parameters are valid.
    pub fn try_build(self) -> Result<AdvertisingParameters, Error> {
        if self.obj.is_valid() {
            Ok(self.obj)
        } else {
            Err(Error::InvalidAdvertisingParameters)
        }
    }

    /// Define the advertising interval.
    pub fn with_interval(mut self, interval: RangeInclusive<AdvertisingIntervalValue>) -> Self {
        self.obj.interval = interval;
        self
    }

    /// Define the advertising type.
    pub fn with_type(mut self, r#type: AdvertisingType) -> Self {
        self.obj.r#type = r#type;
        self
    }

    /// Define our own address type.
    pub fn with_own_address_type(mut self, own_address_type: OwnAddressType) -> Self {
        self.obj.own_address_type = own_address_type;
        self
    }

    /// Define the peer address type.
    pub fn with_peer_address_type(mut self, peer_address_type: PeerAddressType) -> Self {
        self.obj.peer_address_type = peer_address_type;
        self
    }

    /// Define the peer address.
    pub fn with_peer_address(mut self, peer_address: PeerAddress) -> Self {
        self.obj.peer_address = peer_address;
        self
    }

    /// Define the advertising channels to be used.
    pub fn with_channel_map(mut self, channel_map: AdvertisingChannelMap) -> Self {
        self.obj.channel_map = channel_map;
        self
    }

    /// Defined the advertising filter policy.
    pub fn with_filter_policy(mut self, filter_policy: AdvertisingFilterPolicy) -> Self {
        self.obj.filter_policy = filter_policy;
        self
    }
}

/// Advertising parameters to be set before starting advertising.
///
/// It contains this information:
///  - the advertising interval
///  - the advertising type
///  - our own address type
///  - the peer address type
///  - the peer address
///  - the advertising channel map
///  - the advertising filter policy
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.5](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3142c154-1bdd-37b2-cc6e-006aa755f5f7).
///
/// Use the [`AdvertisingParametersBuilder`] to instantiate it.
#[derive(Debug, Clone)]
pub struct AdvertisingParameters {
    interval: RangeInclusive<AdvertisingIntervalValue>,
    r#type: AdvertisingType,
    own_address_type: OwnAddressType,
    peer_address_type: PeerAddressType,
    peer_address: PeerAddress,
    channel_map: AdvertisingChannelMap,
    filter_policy: AdvertisingFilterPolicy,
}

impl AdvertisingParameters {
    /// Instantiate a builder to create Advertising Parameters.
    pub fn builder() -> AdvertisingParametersBuilder {
        AdvertisingParametersBuilder::new()
    }

    fn is_valid(&self) -> bool {
        !self.channel_map.is_empty()
            && match self.r#type {
                AdvertisingType::ScannableUndirected
                | AdvertisingType::NonConnectableUndirected
                    if self.interval.start().value < 0x00A0 =>
                {
                    false
                }
                AdvertisingType::ConnectableHighDutyCycleDirected
                | AdvertisingType::ConnectableLowDutyCycleDirected => {
                    // TODO: Check validity of peer address type and peer address
                    true
                }
                _ => true,
            }
    }

    pub(crate) fn encode(&self) -> Result<([u8; 15], usize), Error> {
        let mut buffer = [0u8; 15];
        let mut offset = 0;
        offset += encode_le_u16(&mut buffer[offset..], self.interval.start().value)?;
        offset += encode_le_u16(&mut buffer[offset..], self.interval.end().value)?;
        buffer[offset] = self.r#type as u8;
        offset += 1;
        buffer[offset] = self.own_address_type as u8;
        offset += 1;
        buffer[offset] = self.peer_address_type as u8;
        offset += 1;
        buffer[offset..offset + 6].copy_from_slice(self.peer_address.value.as_slice());
        offset += 6;
        buffer[offset] = self.channel_map.bits();
        offset += 1;
        buffer[offset] = self.filter_policy as u8;
        offset += 1;
        Ok((buffer, offset))
    }
}

impl Default for AdvertisingParameters {
    fn default() -> Self {
        Self {
            interval: (AdvertisingIntervalValue::default()..=AdvertisingIntervalValue::default()),
            r#type: AdvertisingType::default(),
            own_address_type: OwnAddressType::default(),
            peer_address_type: PeerAddressType::default(),
            peer_address: PeerAddress::default(),
            channel_map: AdvertisingChannelMap::default(),
            filter_policy: AdvertisingFilterPolicy::default(),
        }
    }
}
