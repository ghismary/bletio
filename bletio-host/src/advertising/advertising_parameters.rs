use core::ops::{Deref, RangeInclusive};

use bletio_hci::{
    AdvertisingChannelMap, AdvertisingFilterPolicy, AdvertisingIntervalValue, AdvertisingType,
    DeviceAddress, OwnAddressType,
};

use crate::advertising::AdvertisingError;

/// Builder to create [`AdvertisingParameters`].
#[derive(Debug, Default)]
pub struct AdvertisingParametersBuilder {
    data: AdvertisingParameters,
}

impl AdvertisingParametersBuilder {
    /// Create a builder to instantiate [`AdvertisingParameters`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Try building the [`AdvertisingParameters`], checking that every set parameters are valid.
    pub fn try_build(self) -> Result<AdvertisingParameters, AdvertisingError> {
        if self.is_valid() {
            Ok(self.data)
        } else {
            Err(AdvertisingError::InvalidAdvertisingParameters)?
        }
    }

    /// Define the advertising interval.
    pub fn with_interval(mut self, interval: RangeInclusive<AdvertisingIntervalValue>) -> Self {
        self.data.inner.interval = interval;
        self
    }

    /// Define the advertising type.
    pub fn with_type(mut self, r#type: AdvertisingType) -> Self {
        self.data.inner.r#type = r#type;
        self
    }

    /// Define our own address type.
    pub fn with_own_address_type(mut self, own_address_type: OwnAddressType) -> Self {
        self.data.inner.own_address_type = own_address_type;
        self
    }

    /// Define the peer address.
    pub fn with_peer_address(mut self, peer_address: DeviceAddress) -> Self {
        self.data.inner.peer_address = peer_address;
        self
    }

    /// Define the advertising channels to be used.
    pub fn with_channel_map(mut self, channel_map: AdvertisingChannelMap) -> Self {
        self.data.inner.channel_map = channel_map;
        self
    }

    /// Defined the advertising filter policy.
    pub fn with_filter_policy(mut self, filter_policy: AdvertisingFilterPolicy) -> Self {
        self.data.inner.filter_policy = filter_policy;
        self
    }

    fn is_valid(&self) -> bool {
        !self.data.inner.channel_map.is_empty()
            && match self.data.inner.r#type {
                AdvertisingType::ScannableUndirected
                | AdvertisingType::NonConnectableUndirected
                    if self.data.inner.interval.start().value() < 0x00A0 =>
                {
                    false
                }
                AdvertisingType::ConnectableHighDutyCycleDirected
                | AdvertisingType::ConnectableLowDutyCycleDirected => {
                    // TODO: Check validity of peer address type and peer address. Can it be checked?
                    true
                }
                _ => true,
            }
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
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AdvertisingParameters {
    inner: bletio_hci::AdvertisingParameters,
}

impl AdvertisingParameters {
    /// Instantiate a builder to create Advertising Parameters.
    pub fn builder() -> AdvertisingParametersBuilder {
        AdvertisingParametersBuilder::new()
    }
}

impl Deref for AdvertisingParameters {
    type Target = bletio_hci::AdvertisingParameters;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
