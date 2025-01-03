use core::ops::Range;

use crate::utils::encode_le_u16;
use crate::Error;

#[derive(Debug)]
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

#[derive(Debug, Default)]
pub struct PeerAddress {
    value: [u8; 6],
}

#[derive(Debug, Copy, Clone)]
pub struct AdvertisingChannelMap {
    value: u8,
}

impl AdvertisingChannelMap {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn channel37(mut self, enable: bool) -> Self {
        if enable {
            self.value |= 0b0000_0001;
        } else {
            self.value &= !0b0000_0001;
        }
        self
    }

    #[must_use]
    pub fn channel38(mut self, enable: bool) -> Self {
        if enable {
            self.value |= 0b0000_0010;
        } else {
            self.value &= !0b0000_0010;
        }
        self
    }

    #[must_use]
    pub fn channel39(mut self, enable: bool) -> Self {
        if enable {
            self.value |= 0b0000_0100;
        } else {
            self.value &= !0b0000_0100;
        }
        self
    }

    pub(crate) fn has_channels_enabled(&self) -> bool {
        self.value != 0b0000_0000
    }
}

impl Default for AdvertisingChannelMap {
    fn default() -> Self {
        Self { value: 0b0000_0111 }
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

#[derive(Debug)]
pub struct AdvertisingParameters {
    pub interval: Range<AdvertisingIntervalValue>,
    pub r#type: AdvertisingType,
    pub own_address_type: OwnAddressType,
    pub peer_address_type: PeerAddressType,
    pub peer_address: PeerAddress,
    pub channel_map: AdvertisingChannelMap,
    pub filter_policy: AdvertisingFilterPolicy,
}

impl AdvertisingParameters {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn is_valid(&self) -> bool {
        self.channel_map.has_channels_enabled()
            && match self.r#type {
                AdvertisingType::ScannableUndirected
                | AdvertisingType::NonConnectableUndirected
                    if self.interval.start.value < 0x00A0 =>
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
        offset += encode_le_u16(&mut buffer[offset..], self.interval.start.value)?;
        offset += encode_le_u16(&mut buffer[offset..], self.interval.end.value)?;
        buffer[offset] = self.r#type as u8;
        offset += 1;
        buffer[offset] = self.own_address_type as u8;
        offset += 1;
        buffer[offset] = self.peer_address_type as u8;
        offset += 1;
        buffer[offset..offset + 6].copy_from_slice(self.peer_address.value.as_slice());
        offset += 6;
        buffer[offset] = self.channel_map.value;
        offset += 1;
        buffer[offset] = self.filter_policy as u8;
        offset += 1;
        Ok((buffer, offset))
    }
}

impl Default for AdvertisingParameters {
    fn default() -> Self {
        Self {
            interval: Range {
                start: AdvertisingIntervalValue::default(),
                end: AdvertisingIntervalValue::default(),
            },
            r#type: AdvertisingType::default(),
            own_address_type: OwnAddressType::default(),
            peer_address_type: PeerAddressType::default(),
            peer_address: PeerAddress::default(),
            channel_map: AdvertisingChannelMap::default(),
            filter_policy: AdvertisingFilterPolicy::default(),
        }
    }
}
