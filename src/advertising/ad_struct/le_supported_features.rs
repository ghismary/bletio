use crate::advertising::ad_struct::{AdStruct, AdStructBuffer, AdStructType};

use crate::assigned_numbers::AdType;
use crate::SupportedLeFeatures;

// TODO: Increase when supporting multiple pages of LE features.
const LE_SUPPORTED_FEATURES_AD_STRUCT_SIZE: usize = 10;

/// The LE features supported by the device.
///
/// This supported LE features are defined in
/// [Supplement to the Bluetooth Core Specification, Part A, 1.19](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-c237ce97-d191-6c26-dc09-4da951e18332),
/// and
/// [Core Specification 6.0, Vol. 6, Part B, 4.6](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/low-energy-controller/link-layer-specification.html#UUID-25d414b5-8c50-cd46-fd17-80f0f816f354).
#[derive(Debug, Clone)]
pub struct LeSupportedFeaturesAdStruct {
    buffer: AdStructBuffer<LE_SUPPORTED_FEATURES_AD_STRUCT_SIZE>,
}

impl LeSupportedFeaturesAdStruct {
    /// Create an LE Supported Features Advertising Structure.
    ///
    /// # Arguments
    ///
    /// * `features` — The supported LE features to notify.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bletio::advertising::ad_struct::LeSupportedFeaturesAdStruct;
    /// # use bletio::SupportedLeFeatures;
    /// let ad_struct = LeSupportedFeaturesAdStruct::new(
    ///     SupportedLeFeatures::LE_ENCRYPTION | SupportedLeFeatures::CONNECTION_PARAMETERS_REQUEST_PROCEDURE
    /// );
    /// ```
    pub fn new(features: SupportedLeFeatures) -> Self {
        let mut s = Self {
            buffer: AdStructBuffer::new(AdType::LeSupportedFeatures),
        };
        // INVARIANT: The buffer space is known to be enough.
        s.buffer.encode_le_u64(features.bits()).unwrap();
        s
    }
}

impl AdStruct for LeSupportedFeaturesAdStruct {
    fn encoded_data(&self) -> &[u8] {
        self.buffer.data()
    }
    fn r#type(&self) -> AdStructType {
        AdStructType::LE_SUPPORTED_FEATURES
    }

    fn is_unique(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_le_supported_features_ad_struct() {
        let value = LeSupportedFeaturesAdStruct::new(
            SupportedLeFeatures::LE_2M_PHY | SupportedLeFeatures::LE_CODED_PHY,
        );
        assert_eq!(
            value.encoded_data(),
            &[0x09, 0x27, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        );
        assert!(value.r#type().contains(AdStructType::LE_SUPPORTED_FEATURES));
        assert!(value.is_unique());
    }
}
