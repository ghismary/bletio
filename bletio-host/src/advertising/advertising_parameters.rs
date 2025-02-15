use core::ops::Deref;

use bletio_hci::{
    AdvertisingChannelMap, AdvertisingFilterPolicy, AdvertisingIntervalRange, AdvertisingType,
    DeviceAddress, OwnAddressType,
};

use crate::advertising::AdvertisingError;

/// Builder to create [`AdvertisingParameters`].
#[derive(Debug, Default)]
pub struct AdvertisingParametersBuilder {
    interval: AdvertisingIntervalRange,
    r#type: AdvertisingType,
    own_address_type: OwnAddressType,
    peer_address: DeviceAddress,
    channel_map: AdvertisingChannelMap,
    filter_policy: AdvertisingFilterPolicy,
}

impl AdvertisingParametersBuilder {
    /// Create a builder to instantiate [`AdvertisingParameters`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Try building the [`AdvertisingParameters`], checking that every set parameters are valid.
    pub fn try_build(self) -> Result<AdvertisingParameters, AdvertisingError> {
        Ok(AdvertisingParameters {
            inner: bletio_hci::AdvertisingParameters::try_new(
                self.interval,
                self.r#type,
                self.own_address_type,
                self.peer_address,
                self.channel_map,
                self.filter_policy,
            )
            .map_err(|_| AdvertisingError::InvalidAdvertisingParameters)?,
        })
    }

    /// Define the advertising channels to be used.
    pub fn with_channel_map(mut self, channel_map: AdvertisingChannelMap) -> Self {
        self.channel_map = channel_map;
        self
    }

    /// Defined the advertising filter policy.
    pub fn with_filter_policy(mut self, filter_policy: AdvertisingFilterPolicy) -> Self {
        self.filter_policy = filter_policy;
        self
    }

    /// Define the advertising interval.
    pub fn with_interval(mut self, interval: AdvertisingIntervalRange) -> Self {
        self.interval = interval;
        self
    }

    /// Define our own address type.
    pub fn with_own_address_type(mut self, own_address_type: OwnAddressType) -> Self {
        self.own_address_type = own_address_type;
        self
    }

    /// Define the peer address.
    pub fn with_peer_address(mut self, peer_address: DeviceAddress) -> Self {
        self.peer_address = peer_address;
        self
    }

    /// Define the advertising type.
    pub fn with_type(mut self, r#type: AdvertisingType) -> Self {
        self.r#type = r#type;
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_advertising_parameters() -> Result<(), AdvertisingError> {
        let adv_params = AdvertisingParameters::builder().try_build()?;
        assert_eq!(
            adv_params.deref(),
            &bletio_hci::AdvertisingParameters::default()
        );
        Ok(())
    }

    #[test]
    fn test_valid_advertising_parameters() -> Result<(), AdvertisingError> {
        let interval = AdvertisingIntervalRange::try_new(
            0x0100.try_into().unwrap(),
            0x0110.try_into().unwrap(),
        )
        .unwrap();
        let channel_map = AdvertisingChannelMap::CHANNEL37 | AdvertisingChannelMap::CHANNEL39;
        let adv_params = AdvertisingParameters::builder()
            .with_type(AdvertisingType::ScannableUndirected)
            .with_interval(interval.clone())
            .with_peer_address(DeviceAddress::default())
            .with_own_address_type(OwnAddressType::RandomDeviceAddress)
            .with_channel_map(channel_map)
            .with_filter_policy(AdvertisingFilterPolicy::ScanAllAndConnectionFilterAcceptList)
            .try_build()?;
        assert_eq!(adv_params.r#type(), AdvertisingType::ScannableUndirected);
        assert_eq!(adv_params.interval(), interval);
        assert!(matches!(
            adv_params.peer_address(),
            DeviceAddress::Public(_)
        ));
        assert_eq!(adv_params.peer_address().value(), &[0, 0, 0, 0, 0, 0]);
        assert_eq!(
            adv_params.own_address_type(),
            OwnAddressType::RandomDeviceAddress
        );
        assert_eq!(adv_params.channel_map(), channel_map);
        assert_eq!(
            adv_params.filter_policy(),
            AdvertisingFilterPolicy::ScanAllAndConnectionFilterAcceptList
        );
        Ok(())
    }

    #[test]
    fn test_invalid_advertising_parameters_empty_channel_map() {
        let err = AdvertisingParameters::builder()
            .with_channel_map(AdvertisingChannelMap::empty())
            .try_build();
        assert_eq!(err, Err(AdvertisingError::InvalidAdvertisingParameters));
    }
}
