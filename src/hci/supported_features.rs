use bitflags::bitflags;

bitflags! {
    /// Features supported by the Link Manager.
    ///
    /// These features are defined in
    /// [Core Specification 6.0, Vol. 2, Part C, 3](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/br-edr-controller/link-manager-protocol-specification.html#UUID-248645a8-42ca-a871-78ce-4487981382d8).
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct SupportedFeatures: u64 {
        /// This feature indicates whether the Controller supports LE.
        /// The local Host uses this feature bit to determine whether the Controller supports LE.
        /// A remote device does not use this feature bit.
        const LE_SUPPORTED_CONTROLLER = 1 << 38;
        /// This feature indicates that the Controller supports simultaneous LE and BR/EDR links to the same remote device.
        /// The local Host uses this feature bit to determine whether the Controller is capable of supporting simultaneous LE and BR/EDR connections to a remote device.
        /// A remote device does not use this feature bit.
        const SIMULTANEOUS_LE_AND_BREDR_TO_SAME_DEVICE_CAPABLE_CONTROLLER = 1 << 49;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_supported_features() {
        assert_eq!(SupportedFeatures::default(), SupportedFeatures::empty());
        assert_eq!(SupportedFeatures::LE_SUPPORTED_CONTROLLER.bits(), 1 << 38);
    }
}
