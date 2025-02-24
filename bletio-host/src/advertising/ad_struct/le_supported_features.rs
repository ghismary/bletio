use bitflags::Flags;
use bletio_hci::SupportedLeFeatures;
use bletio_utils::EncodeToBuffer;

use crate::assigned_numbers::AdType;

/// The LE features supported by the device.
///
/// This supported LE features are defined in
/// [Supplement to the Bluetooth Core Specification, Part A, 1.19](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-c237ce97-d191-6c26-dc09-4da951e18332),
/// and
/// [Core Specification 6.0, Vol. 6, Part B, 4.6](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/low-energy-controller/link-layer-specification.html#UUID-25d414b5-8c50-cd46-fd17-80f0f816f354).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LeSupportedFeaturesAdStruct {
    features: SupportedLeFeatures,
}

impl LeSupportedFeaturesAdStruct {
    pub(crate) const fn new(features: SupportedLeFeatures) -> Self {
        Self { features }
    }

    fn last_non_zero_index(&self) -> Option<usize> {
        self.features.bits().0.iter().rposition(|v| *v != 0)
    }

    fn minimum_size(&self) -> usize {
        match self.last_non_zero_index() {
            Some(index) => index + 1,
            None => 0,
        }
    }
}

impl EncodeToBuffer for LeSupportedFeaturesAdStruct {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push((self.encoded_size() - 1) as u8)?;
        buffer.try_push(AdType::LeSupportedFeatures as u8)?;
        for byte in &self.features.bits().0[..self.minimum_size()] {
            buffer.try_push(*byte)?;
        }
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        self.minimum_size() + 2
    }
}

pub(crate) mod parser {
    use nom::{bytes::take, combinator::map, IResult, Parser};

    use crate::advertising::ad_struct::AdStruct;

    use super::*;

    pub(crate) fn le_supported_features_ad_struct(input: &[u8]) -> IResult<&[u8], AdStruct> {
        map(map(take(input.len()), Into::into), |features| {
            AdStruct::LeSupportedFeatures(LeSupportedFeaturesAdStruct::new(features))
        })
        .parse(input)
    }
}

#[cfg(test)]
mod test {
    use bletio_utils::{Buffer, BufferOps};
    use rstest::rstest;

    use crate::advertising::ad_struct::AdStruct;

    use super::{parser::*, *};

    #[rstest]
    #[case(SupportedLeFeatures::default(), &[0x01, 0x27])]
    #[case(SupportedLeFeatures::LL_PRIVACY, &[0x02, 0x27, 0x40])]
    #[case(SupportedLeFeatures::LE_2M_PHY | SupportedLeFeatures::LE_CODED_PHY, &[0x03, 0x27, 0x00, 0x09])]
    #[case(
        SupportedLeFeatures::LL_EXTENDED_FEATURE_SET | SupportedLeFeatures::MONITORING_ADVERTISERS,
        &[0x0A, 0x27, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x01]
    )]
    fn test_le_supported_features_ad_struct(
        #[case] features: SupportedLeFeatures,
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<11>::default();
        let value = LeSupportedFeaturesAdStruct::new(features);
        value.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }

    #[rstest]
    #[case(&[0x40], SupportedLeFeatures::LL_PRIVACY)]
    #[case(&[0x00, 0x09], SupportedLeFeatures::LE_2M_PHY | SupportedLeFeatures::LE_CODED_PHY)]
    #[case(
        &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x01],
        SupportedLeFeatures::LL_EXTENDED_FEATURE_SET | SupportedLeFeatures::MONITORING_ADVERTISERS
    )]
    fn test_le_supported_features_ad_struct_parsing(
        #[case] input: &[u8],
        #[case] features: SupportedLeFeatures,
    ) {
        assert_eq!(
            le_supported_features_ad_struct(input),
            Ok((
                &[] as &[u8],
                AdStruct::LeSupportedFeatures(LeSupportedFeaturesAdStruct::new(features))
            ))
        );
    }
}
