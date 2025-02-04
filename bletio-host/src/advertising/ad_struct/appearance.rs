use bletio_utils::EncodeToBuffer;

use crate::assigned_numbers::{AdType, AppearanceValue};

const APPEARANCE_AD_STRUCT_SIZE: usize = 3;

/// The external appearance of the device.
///
/// The appearance of the device is defined in
/// [Supplement to the Bluetooth Core Specification, Part A, 1.12](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-ccfc3325-626d-a1cf-3083-1d5a9112023a),
/// and
/// [Core Specification 6.0, Vol. 3, Part C, 12.2](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-ec0b9e4b-8d14-7280-a0ae-68c61f6f00eb).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppearanceAdStruct {
    appearance: AppearanceValue,
}

impl AppearanceAdStruct {
    pub(crate) fn new(appearance: AppearanceValue) -> Self {
        Self { appearance }
    }
}

impl EncodeToBuffer for AppearanceAdStruct {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push(APPEARANCE_AD_STRUCT_SIZE as u8)?;
        buffer.try_push(AdType::Appearance as u8)?;
        buffer.encode_le_u16(self.appearance as u16)?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        APPEARANCE_AD_STRUCT_SIZE + 1
    }
}

#[cfg(test)]
mod test {
    use bletio_utils::{Buffer, BufferOps};
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(AppearanceValue::StandmountedSpeaker, &[0x03, 0x19, 0x44, 0x08])]
    #[case(AppearanceValue::InsulinPen, &[0x03, 0x19, 0x48, 0x0D])]
    fn test_appearance_ad_struct(
        #[case] appearance: AppearanceValue,
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<4>::default();
        let value = AppearanceAdStruct::new(appearance);
        value.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }
}
