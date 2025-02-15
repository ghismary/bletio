use core::ops::Deref;

use bletio_hci::{OwnAddressType, ScanInterval, ScanType, ScanWindow, ScanningFilterPolicy};

use crate::advertising::AdvertisingError;

/// Builder to create [`ScanParameters`].
#[derive(Debug, Default)]
pub struct ScanParametersBuilder {
    r#type: ScanType,
    interval: ScanInterval,
    window: ScanWindow,
    own_address_type: OwnAddressType,
    filter_policy: ScanningFilterPolicy,
}

impl ScanParametersBuilder {
    /// Create a builder to instantiate [`ScanParameters`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Try building the [`ScanParameters`], checking that every set parameters are valid.
    pub fn try_build(self) -> Result<ScanParameters, AdvertisingError> {
        Ok(ScanParameters {
            inner: bletio_hci::ScanParameters::try_new(
                self.r#type,
                self.interval,
                self.window,
                self.own_address_type,
                self.filter_policy,
            )
            .map_err(|_| AdvertisingError::InvalidScanParameters)?,
        })
    }

    /// Defined the scanning filter policy.
    pub fn with_filter_policy(mut self, filter_policy: ScanningFilterPolicy) -> Self {
        self.filter_policy = filter_policy;
        self
    }
    /// Define the scan interval.
    pub fn with_interval(mut self, interval: ScanInterval) -> Self {
        self.interval = interval;
        self
    }

    /// Define our own address type.
    pub fn with_own_address_type(mut self, own_address_type: OwnAddressType) -> Self {
        self.own_address_type = own_address_type;
        self
    }

    /// Define the scan type.
    pub fn with_type(mut self, r#type: ScanType) -> Self {
        self.r#type = r#type;
        self
    }

    /// Define the scan window.
    pub fn with_window(mut self, window: ScanWindow) -> Self {
        self.window = window;
        self
    }
}

/// Scan parameters to be set before starting scanning.
///
/// It contains this information:
///  - the scan type
///  - the scan interval
///  - the scan window
///  - our own address type
///  - the scanning filter policy
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.10](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-fff6f93e-315b-04fe-51bf-a18f78ceec89).
///
/// Use the [`ScanParametersBuilder`] to instantiate it.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ScanParameters {
    inner: bletio_hci::ScanParameters,
}

impl ScanParameters {
    /// Instantiate a builder to create Scan Parameters.
    pub fn builder() -> ScanParametersBuilder {
        ScanParametersBuilder::new()
    }
}

impl Deref for ScanParameters {
    type Target = bletio_hci::ScanParameters;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg(test)]
mod test {
    use bletio_hci::{scan_interval, scan_window};

    use super::*;

    #[test]
    fn test_default_scan_parameters() -> Result<(), AdvertisingError> {
        let scan_params = ScanParameters::builder().try_build()?;
        assert_eq!(scan_params.deref(), &bletio_hci::ScanParameters::default());
        Ok(())
    }

    #[test]
    fn test_valid_scan_parameters() -> Result<(), AdvertisingError> {
        let interval = scan_interval!(0x0100);
        let window = scan_window!(0x0050);
        let scan_params = ScanParameters::builder()
            .with_type(ScanType::ActiveScanning)
            .with_interval(interval)
            .with_window(window)
            .with_own_address_type(OwnAddressType::RandomDeviceAddress)
            .with_filter_policy(ScanningFilterPolicy::BasicFiltered)
            .try_build()?;
        assert_eq!(scan_params.r#type(), ScanType::ActiveScanning);
        assert_eq!(scan_params.interval(), interval);
        assert_eq!(scan_params.window(), window);
        assert_eq!(
            scan_params.own_address_type(),
            OwnAddressType::RandomDeviceAddress
        );
        assert_eq!(
            scan_params.filter_policy(),
            ScanningFilterPolicy::BasicFiltered
        );
        Ok(())
    }

    #[test]
    fn test_invalid_scan_parameters_window_larger_than_interval() {
        let err = ScanParameters::builder()
            .with_interval(scan_interval!(0x0050))
            .with_window(scan_window!(0x0100))
            .try_build();
        assert_eq!(err, Err(AdvertisingError::InvalidScanParameters));
    }
}
