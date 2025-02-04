use bletio_hci::SupportedLeFeatures;
use bletio_utils::EncodeToBuffer;

use crate::assigned_numbers::AdType;

// TODO: Handle multiple pages of LE features.
//
const LE_SUPPORTED_FEATURES_AD_STRUCT_SIZE: usize = 9;

/// The LE features supported by the device.
///
/// This supported LE features are defined in
/// [Supplement to the Bluetooth Core Specification, Part A, 1.19](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-c237ce97-d191-6c26-dc09-4da951e18332),
/// and
/// [Core Specification 6.0, Vol. 6, Part B, 4.6](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/low-energy-controller/link-layer-specification.html#UUID-25d414b5-8c50-cd46-fd17-80f0f816f354).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LeSupportedFeaturesAdStruct {
    features: SupportedLeFeatures,
}

impl LeSupportedFeaturesAdStruct {
    pub(crate) fn new(features: SupportedLeFeatures) -> Self {
        Self { features }
    }
}

impl EncodeToBuffer for LeSupportedFeaturesAdStruct {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push(LE_SUPPORTED_FEATURES_AD_STRUCT_SIZE as u8)?;
        buffer.try_push(AdType::LeSupportedFeatures as u8)?;
        buffer.encode_le_u64(self.features.bits())?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        LE_SUPPORTED_FEATURES_AD_STRUCT_SIZE + 1
    }
}

#[cfg(test)]
mod test {
    use bletio_utils::{Buffer, BufferOps};

    use super::*;

    #[test]
    fn test_le_supported_features_ad_struct() -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<10>::default();
        let value = LeSupportedFeaturesAdStruct::new(
            SupportedLeFeatures::LE_2M_PHY | SupportedLeFeatures::LE_CODED_PHY,
        );
        value.encode(&mut buffer)?;
        assert_eq!(
            buffer.data(),
            &[0x09, 0x27, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        );
        Ok(())
    }
}
