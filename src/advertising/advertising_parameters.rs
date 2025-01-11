use bitflags::bitflags;
use core::ops::RangeInclusive;

use crate::utils::encode_le_u16;
use crate::Error;

#[derive(Debug, Copy, Clone)]
pub struct AdvertisingIntervalValue {
    value: u16,
}

impl AdvertisingIntervalValue {
    pub fn milliseconds(&self) -> f32 {
        (self.value as f32) * 0.625
    }
}

impl Default for AdvertisingIntervalValue {
    fn default() -> Self {
        // Value defined in Core Specification 4.2, Vol. 2, Part E, 7.8.5
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

#[derive(Debug, Copy, Clone, Default)]
#[repr(u8)]
// Values defined in Core Specification 4.2, Vol. 2, Part E, 7.8.5
pub enum AdvertisingType {
    #[default]
    ConnectableUndirected = 0x00,
    ConnectableHighDutyCycleDirected = 0x01,
    ScannableUndirected = 0x02,
    NonConnectableUndirected = 0x03,
    ConnectableLowDutyCycleDirected = 0x04,
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(u8)]
// Values defined in Core Specification 4.2, Vol. 2, Part E, 7.8.5
pub enum OwnAddressType {
    #[default]
    PublicDeviceAddress = 0x00,
    RandomDeviceAddress = 0x01,
    GeneratedResolvablePrivateAddressFallbackPublic = 0x02,
    GeneratedResolvablePrivateAddressFallbackRandom = 0x03,
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(u8)]
// Values defined in Core Specification 4.2, Vol. 2, Part E, 7.8.5
pub enum PeerAddressType {
    #[default]
    Public = 0x00,
    Random = 0x01,
}

#[derive(Debug, Clone, Default)]
pub struct PeerAddress {
    value: [u8; 6],
}

#[derive(Debug, Copy, Clone)]
pub struct AdvertisingChannelMap(u8);

bitflags! {
    impl AdvertisingChannelMap: u8 {
        const CHANNEL37 = 1 << 0;
        const CHANNEL38 = 1 << 1;
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

#[derive(Debug, Copy, Clone, Default)]
#[repr(u8)]
pub enum AdvertisingFilterPolicy {
    #[default]
    ScanAllAndConnectionAll = 0x00,
    ConnectionAllAndScanWhiteList = 0x01,
    ScanAllAndConnectionWhiteList = 0x02,
    ScanWhiteListAndConnectionWhiteList = 0x03,
}

#[derive(Debug, Default)]
pub struct AdvertisingParametersBuilder {
    obj: AdvertisingParameters,
}

impl AdvertisingParametersBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn try_build(self) -> Result<AdvertisingParameters, Error> {
        if self.obj.is_valid() {
            Ok(self.obj)
        } else {
            Err(Error::InvalidAdvertisingParameters)
        }
    }

    pub fn with_interval(mut self, interval: RangeInclusive<AdvertisingIntervalValue>) -> Self {
        self.obj.interval = interval;
        self
    }

    pub fn with_type(mut self, r#type: AdvertisingType) -> Self {
        self.obj.r#type = r#type;
        self
    }

    pub fn with_own_address_type(mut self, own_address_type: OwnAddressType) -> Self {
        self.obj.own_address_type = own_address_type;
        self
    }

    pub fn with_peer_address_type(mut self, peer_address_type: PeerAddressType) -> Self {
        self.obj.peer_address_type = peer_address_type;
        self
    }

    pub fn with_peer_address(mut self, peer_address: PeerAddress) -> Self {
        self.obj.peer_address = peer_address;
        self
    }

    pub fn with_channel_map(mut self, channel_map: AdvertisingChannelMap) -> Self {
        self.obj.channel_map = channel_map;
        self
    }

    pub fn with_filter_policy(mut self, filter_policy: AdvertisingFilterPolicy) -> Self {
        self.obj.filter_policy = filter_policy;
        self
    }
}

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
