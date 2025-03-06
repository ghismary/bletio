//! Advertising Parameters.
//!
//! These Advertising Parameters need to be defined to start advertising.

#[cfg(not(feature = "defmt"))]
use bitflags::bitflags;
#[cfg(feature = "defmt")]
use defmt::bitflags;

use bletio_utils::{BufferOps, EncodeToBuffer, Error as UtilsError};
use core::ops::RangeInclusive;
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{DeviceAddress, Error, OwnAddressType};

/// Advertising interval.
///
/// Used for undirected and low duty cycle directed advertising.
///
/// Here are the characteristics of this advertising interval:
///  - Range: 0x0020 to 0x4000
///  - Default: 0x0800 (1.28 s)
///  - Time = N × 0.625 ms
///  - Time Range: 20 ms to 10.24 s
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.5](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3142c154-1bdd-37b2-cc6e-006aa755f5f7).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AdvertisingInterval {
    value: u16,
}

impl AdvertisingInterval {
    /// Create a valid advertising interval.
    pub const fn try_new(value: u16) -> Result<Self, Error> {
        if check_advertising_interval_value(value) {
            Ok(Self { value })
        } else {
            Err(Error::InvalidAdvertisingInterval(value))
        }
    }

    #[doc(hidden)]
    pub const fn new_unchecked(value: u16) -> Self {
        Self { value }
    }

    /// Get the value of the advertising interval in milliseconds.
    pub const fn milliseconds(&self) -> f32 {
        (self.value as f32) * 0.625
    }

    pub const fn value(&self) -> u16 {
        self.value
    }
}

impl Default for AdvertisingInterval {
    fn default() -> Self {
        Self { value: 0x0800 }
    }
}

impl TryFrom<u16> for AdvertisingInterval {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl EncodeToBuffer for AdvertisingInterval {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, UtilsError> {
        buffer.encode_le_u16(self.value)
    }

    fn encoded_size(&self) -> usize {
        size_of::<u16>()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AdvertisingIntervalRange {
    value: RangeInclusive<AdvertisingInterval>,
}

impl AdvertisingIntervalRange {
    pub fn try_new(min: u16, max: u16) -> Result<Self, Error> {
        if check_advertising_range_order(min, max) {
            let min: AdvertisingInterval = min.try_into()?;
            let max: AdvertisingInterval = max.try_into()?;
            Ok(Self { value: min..=max })
        } else {
            Err(Error::InvalidAdvertisingIntervalRange)
        }
    }

    #[doc(hidden)]
    pub const fn new_unchecked(min: u16, max: u16) -> Self {
        Self {
            value: AdvertisingInterval::new_unchecked(min)
                ..=AdvertisingInterval::new_unchecked(max),
        }
    }

    pub const fn min(&self) -> AdvertisingInterval {
        *self.value.start()
    }

    pub const fn max(&self) -> AdvertisingInterval {
        *self.value.end()
    }
}

impl Default for AdvertisingIntervalRange {
    fn default() -> Self {
        Self {
            value: Default::default()..=Default::default(),
        }
    }
}

impl EncodeToBuffer for AdvertisingIntervalRange {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, UtilsError> {
        self.value.start().encode(buffer)?;
        self.value.end().encode(buffer)?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        self.value.start().encoded_size() + self.value.end().encoded_size()
    }
}

#[diagnostic::on_unimplemented(
    message = "the advertising interval value is invalid, it needs to be between 0x0020 and 0x4000"
)]
#[doc(hidden)]
pub trait AdvertisingIntervalValueValid {}

#[doc(hidden)]
pub struct AdvertisingIntervalValueIsValid<const VALID: bool>;

#[doc(hidden)]
impl AdvertisingIntervalValueValid for AdvertisingIntervalValueIsValid<true> {}

#[doc(hidden)]
pub const fn advertising_interval_value_is_valid<T: AdvertisingIntervalValueValid>() {}

#[doc(hidden)]
pub const fn check_advertising_interval_value(value: u16) -> bool {
    (value >= 0x0020) && (value <= 0x4000)
}

#[diagnostic::on_unimplemented(
    message = "the advertising interval minimum value must be smaller or equal to the maximum value"
)]
#[doc(hidden)]
pub trait AdvertisingIntervalRangeOrderValid {}

#[doc(hidden)]
pub struct AdvertisingIntervalRangeOrderIsValid<const VALID: bool>;

#[doc(hidden)]
impl AdvertisingIntervalRangeOrderValid for AdvertisingIntervalRangeOrderIsValid<true> {}

#[doc(hidden)]
pub const fn advertising_interval_range_order_is_valid<T: AdvertisingIntervalRangeOrderValid>() {}

#[doc(hidden)]
pub const fn check_advertising_range_order(min: u16, max: u16) -> bool {
    min <= max
}

/// Create an [`AdvertisingIntervalRange`], checking that it is valid at compile-time.
///
/// # Examples
///
/// ```
/// # use bletio_hci::advertising_interval_range;
/// let range = advertising_interval_range!(0x0020, 0x0030);
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __advertising_interval_range__ {
    ($min:expr, $max:expr) => {{
        const MIN_ERR: bool =
            bletio_hci::advertising_parameters::check_advertising_interval_value($min);
        bletio_hci::advertising_parameters::advertising_interval_value_is_valid::<
            bletio_hci::advertising_parameters::AdvertisingIntervalValueIsValid<MIN_ERR>,
        >();
        const MAX_ERR: bool =
            bletio_hci::advertising_parameters::check_advertising_interval_value($min);
        bletio_hci::advertising_parameters::advertising_interval_value_is_valid::<
            bletio_hci::advertising_parameters::AdvertisingIntervalValueIsValid<MAX_ERR>,
        >();
        const ORDER_ERR: bool =
            bletio_hci::advertising_parameters::check_advertising_range_order($min, $max);
        bletio_hci::advertising_parameters::advertising_interval_range_order_is_valid::<
            bletio_hci::advertising_parameters::AdvertisingIntervalRangeOrderIsValid<ORDER_ERR>,
        >();
        bletio_hci::advertising_parameters::AdvertisingIntervalRange::new_unchecked($min, $max)
    }};
}

#[doc(inline)]
pub use __advertising_interval_range__ as advertising_interval_range;

/// Advertising type.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.5](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3142c154-1bdd-37b2-cc6e-006aa755f5f7).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[num_enum(error_type(name = Error, constructor = Error::InvalidAdvertisingType))]
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

impl EncodeToBuffer for AdvertisingType {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, UtilsError> {
        buffer.try_push((*self).into())
    }

    fn encoded_size(&self) -> usize {
        size_of::<AdvertisingType>()
    }
}

/// Peer address type.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.5](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3142c154-1bdd-37b2-cc6e-006aa755f5f7).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::InvalidPeerAddressType))]
#[repr(u8)]
#[non_exhaustive]
enum PeerAddressType {
    /// Public Device Address (default) or Public Identity Address.
    #[default]
    Public = 0x00,
    /// Random Device Address or Random (static) Identity Address.
    Random = 0x01,
}

impl From<&DeviceAddress> for PeerAddressType {
    fn from(value: &DeviceAddress) -> Self {
        match value {
            DeviceAddress::Public(_) => Self::Public,
            DeviceAddress::Random(_) => Self::Random,
        }
    }
}

impl EncodeToBuffer for PeerAddressType {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, UtilsError> {
        buffer.try_push((*self).into())
    }

    fn encoded_size(&self) -> usize {
        size_of::<PeerAddressType>()
    }
}

bitflags! {
    /// Channel map of the channels to use for advertising.
    ///
    /// Defaults to all the 3 channels (37, 38 & 39).
    ///
    /// See [Core Specification 6.0, Vol.4, Part E, 7.8.5](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3142c154-1bdd-37b2-cc6e-006aa755f5f7).
    #[cfg_attr(not(feature = "defmt"), derive(Debug, Clone, Copy, PartialEq, Eq))]
    pub struct AdvertisingChannelMap: u8 {
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

impl EncodeToBuffer for AdvertisingChannelMap {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, UtilsError> {
        buffer.try_push(self.bits())
    }

    fn encoded_size(&self) -> usize {
        size_of::<AdvertisingChannelMap>()
    }
}

/// Advertising Filter Policy.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.5](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3142c154-1bdd-37b2-cc6e-006aa755f5f7).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[num_enum(error_type(name = Error, constructor = Error::InvalidAdvertisingFilterPolicy))]
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
    ScanFilterAcceptListAndConnectionFilterAcceptList = 0x03,
}

impl EncodeToBuffer for AdvertisingFilterPolicy {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, UtilsError> {
        buffer.try_push((*self).into())
    }

    fn encoded_size(&self) -> usize {
        size_of::<AdvertisingFilterPolicy>()
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
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AdvertisingParameters {
    interval: AdvertisingIntervalRange,
    r#type: AdvertisingType,
    own_address_type: OwnAddressType,
    peer_address: DeviceAddress,
    channel_map: AdvertisingChannelMap,
    filter_policy: AdvertisingFilterPolicy,
}

impl AdvertisingParameters {
    pub fn try_new(
        interval: AdvertisingIntervalRange,
        r#type: AdvertisingType,
        own_address_type: OwnAddressType,
        peer_address: DeviceAddress,
        channel_map: AdvertisingChannelMap,
        filter_policy: AdvertisingFilterPolicy,
    ) -> Result<AdvertisingParameters, Error> {
        if channel_map.is_empty() {
            Err(Error::AtLeastOneChannelMustBeEnabledInTheAdvertisingChannelMap)
        } else {
            Ok(AdvertisingParameters {
                interval,
                r#type,
                own_address_type,
                peer_address,
                channel_map,
                filter_policy,
            })
        }
    }

    pub fn channel_map(&self) -> AdvertisingChannelMap {
        self.channel_map
    }

    pub fn filter_policy(&self) -> AdvertisingFilterPolicy {
        self.filter_policy
    }

    pub fn interval(&self) -> AdvertisingIntervalRange {
        self.interval.clone()
    }

    pub fn own_address_type(&self) -> OwnAddressType {
        self.own_address_type
    }

    pub fn peer_address(&self) -> &DeviceAddress {
        &self.peer_address
    }

    pub fn r#type(&self) -> AdvertisingType {
        self.r#type
    }
}

impl EncodeToBuffer for AdvertisingParameters {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, UtilsError> {
        let peer_address_type: PeerAddressType = (&self.peer_address).into();
        self.interval.encode(buffer)?;
        self.r#type.encode(buffer)?;
        self.own_address_type.encode(buffer)?;
        peer_address_type.encode(buffer)?;
        self.peer_address.encode(buffer)?;
        self.channel_map.encode(buffer)?;
        self.filter_policy.encode(buffer)?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        self.interval.encoded_size()
            + self.r#type.encoded_size()
            + self.own_address_type.encoded_size()
            + size_of::<PeerAddressType>()
            + self.peer_address.encoded_size()
            + self.channel_map.encoded_size()
            + self.filter_policy.encoded_size()
    }
}

pub(crate) mod parser {
    use nom::{
        bytes::take,
        combinator::{all_consuming, map, map_res},
        number::{le_u16, le_u8},
        sequence::pair,
        IResult, Parser,
    };

    use crate::{
        device_address::RandomAddress, own_address_type::parser::own_address_type,
        PublicDeviceAddress,
    };

    use super::*;

    fn advertising_interval(input: &[u8]) -> IResult<&[u8], AdvertisingInterval> {
        map_res(le_u16(), TryInto::try_into).parse(input)
    }

    fn advertising_interval_range(input: &[u8]) -> IResult<&[u8], AdvertisingIntervalRange> {
        map(
            pair(advertising_interval, advertising_interval),
            |(start, end)| AdvertisingIntervalRange { value: start..=end },
        )
        .parse(input)
    }

    fn advertising_type(input: &[u8]) -> IResult<&[u8], AdvertisingType> {
        map_res(le_u8(), TryInto::try_into).parse(input)
    }

    fn peer_address_type(input: &[u8]) -> IResult<&[u8], PeerAddressType> {
        map_res(le_u8(), TryInto::try_into).parse(input)
    }

    fn peer_address(input: &[u8]) -> IResult<&[u8], DeviceAddress> {
        map_res(
            (
                peer_address_type,
                map_res(take(6u8), TryInto::<[u8; 6]>::try_into),
            ),
            |(peer_address_type, peer_address)| {
                Ok::<DeviceAddress, Error>(match peer_address_type {
                    PeerAddressType::Public => {
                        let address: PublicDeviceAddress = peer_address.into();
                        address.into()
                    }
                    PeerAddressType::Random => {
                        let address: RandomAddress = peer_address.try_into()?;
                        address.into()
                    }
                })
            },
        )
        .parse(input)
    }

    fn channel_map(input: &[u8]) -> IResult<&[u8], AdvertisingChannelMap> {
        map(le_u8(), AdvertisingChannelMap::from_bits_truncate).parse(input)
    }

    fn filter_policy(input: &[u8]) -> IResult<&[u8], AdvertisingFilterPolicy> {
        map_res(le_u8(), TryInto::try_into).parse(input)
    }

    pub(crate) fn advertising_parameters(input: &[u8]) -> IResult<&[u8], AdvertisingParameters> {
        all_consuming(map(
            (
                advertising_interval_range,
                advertising_type,
                own_address_type,
                peer_address,
                channel_map,
                filter_policy,
            ),
            |(interval, r#type, own_address_type, peer_address, channel_map, filter_policy)| {
                AdvertisingParameters {
                    interval,
                    r#type,
                    own_address_type,
                    peer_address,
                    channel_map,
                    filter_policy,
                }
            },
        ))
        .parse(input)
    }
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;
    use bletio_utils::Buffer;
    use rstest::rstest;

    use crate::{RandomAddress, RandomStaticDeviceAddress};

    use super::*;

    #[test]
    fn test_advertising_interval_default() {
        let value = AdvertisingInterval::default();
        assert_eq!(value.value(), 0x800);
        assert_relative_eq!(value.milliseconds(), 1280f32, epsilon = 1.0e-6);
    }

    #[rstest]
    #[case(0x0020, 20f32)]
    #[case(0x4000, 10240f32)]
    fn test_advertising_interval_success(
        #[case] input: u16,
        #[case] expected_milliseconds: f32,
    ) -> Result<(), Error> {
        let value = AdvertisingInterval::try_new(input)?;
        assert_eq!(value.value(), input);
        assert_relative_eq!(
            value.milliseconds(),
            expected_milliseconds,
            epsilon = 1.0e-6
        );
        Ok(())
    }

    #[rstest]
    #[case(0x0010)]
    #[case(0x8000)]
    fn test_advertising_interval_failure(#[case] input: u16) {
        let err = AdvertisingInterval::try_new(input);
        assert_eq!(err, Err(Error::InvalidAdvertisingInterval(input)));
    }

    #[rstest]
    #[case(0x0020, 0x0020)]
    #[case(0x0020, 0x0030)]
    fn test_advertising_interval_range_success(
        #[case] min: u16,
        #[case] max: u16,
    ) -> Result<(), Error> {
        let value = AdvertisingIntervalRange::try_new(min, max)?;
        assert_eq!(value.min().value, min);
        assert_eq!(value.max().value, max);
        Ok(())
    }

    #[rstest]
    #[case(0x0000, 0x0020, Error::InvalidAdvertisingInterval(0x0000))]
    #[case(0x0030, 0x5000, Error::InvalidAdvertisingInterval(0x5000))]
    #[case(0x0030, 0x0020, Error::InvalidAdvertisingIntervalRange)]
    #[case(0x4000, 0x3000, Error::InvalidAdvertisingIntervalRange)]
    fn test_advertising_interval_range_failure(
        #[case] min: u16,
        #[case] max: u16,
        #[case] error: Error,
    ) {
        let err = AdvertisingIntervalRange::try_new(min, max);
        assert_eq!(err, Err(error));
    }

    #[rstest]
    #[case(DeviceAddress::default(), PeerAddressType::Public, &[0x00])]
    #[case(
        DeviceAddress::Random(RandomAddress::Static(RandomStaticDeviceAddress::try_new([0xFE, 0x92, 0x2F, 0x0F, 0x4B, 0xD2]).unwrap())),
        PeerAddressType::Random, &[0x01]
    )]
    fn test_peer_address_type(
        #[case] input: DeviceAddress,
        #[case] expected: PeerAddressType,
        #[case] expected_encoded_peer_address_type: &[u8],
    ) {
        let peer_address_type: PeerAddressType = (&input).into();
        assert_eq!(peer_address_type, expected);
        let mut buffer = Buffer::<6>::default();
        assert_eq!(peer_address_type.encoded_size(), 1);
        peer_address_type.encode(&mut buffer).unwrap();
        assert_eq!(buffer.data(), expected_encoded_peer_address_type);
    }

    #[rstest]
    #[case(AdvertisingChannelMap::default(), AdvertisingChannelMap::CHANNEL37 | AdvertisingChannelMap::CHANNEL38 | AdvertisingChannelMap::CHANNEL39)]
    #[case(AdvertisingChannelMap::new(), AdvertisingChannelMap::CHANNEL37 | AdvertisingChannelMap::CHANNEL38 | AdvertisingChannelMap::CHANNEL39)]
    fn test_advertising_channel_map(
        #[case] input: AdvertisingChannelMap,
        #[case] expected: AdvertisingChannelMap,
    ) {
        assert_eq!(input, expected);
    }

    #[test]
    fn test_default_advertising_parameters() -> Result<(), Error> {
        let adv_params = AdvertisingParameters::default();
        assert_eq!(
            adv_params,
            AdvertisingParameters {
                interval: AdvertisingIntervalRange::default(),
                r#type: AdvertisingType::default(),
                own_address_type: OwnAddressType::default(),
                peer_address: DeviceAddress::default(),
                channel_map: AdvertisingChannelMap::default(),
                filter_policy: AdvertisingFilterPolicy::default(),
            },
        );

        Ok(())
    }
}
