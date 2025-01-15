use crate::advertising::ad_struct::{AdStruct, AdStructBuffer, AdStructType};

use crate::assigned_numbers::{AdType, AppearanceValue};

const APPEARANCE_AD_STRUCT_SIZE: usize = 4;

/// The external appearance of the device.
///
/// The appearance of the device is defined in
/// [Supplement to the Bluetooth Core Specification, Part A, 1.12](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-ccfc3325-626d-a1cf-3083-1d5a9112023a),
/// and
/// [Core Specification 6.0, Vol. 3, Part C, 12.2](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-ec0b9e4b-8d14-7280-a0ae-68c61f6f00eb).
#[derive(Debug, Clone)]
pub struct AppearanceAdStruct {
    buffer: AdStructBuffer<APPEARANCE_AD_STRUCT_SIZE>,
}

impl AppearanceAdStruct {
    /// Create an Appearance Advertising Structure.
    ///
    /// # Arguments
    ///
    /// * `appearance` — The appearance of the device to notify.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bletio::advertising::ad_struct::AppearanceAdStruct;
    /// # use bletio::assigned_numbers::AppearanceValue;
    /// let ad_struct = AppearanceAdStruct::new(AppearanceValue::BroadcastingDevice);
    /// ```
    pub fn new(appearance: AppearanceValue) -> Self {
        let mut s = Self {
            buffer: AdStructBuffer::new(AdType::Appearance),
        };
        // INVARIANT: The buffer space is known to be enough.
        s.buffer.encode_le_u16(appearance as u16).unwrap();
        s
    }
}

impl AdStruct for AppearanceAdStruct {
    fn encoded_data(&self) -> &[u8] {
        self.buffer.data()
    }
    fn r#type(&self) -> AdStructType {
        AdStructType::APPEARANCE
    }

    fn is_unique(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_appearance_ad_struct() {
        let value = AppearanceAdStruct::new(AppearanceValue::StandmountedSpeaker);
        assert_eq!(value.encoded_data(), &[0x03, 0x19, 0x44, 0x08]);
        assert!(value.r#type().contains(AdStructType::APPEARANCE));
        assert!(value.is_unique());

        let value = AppearanceAdStruct::new(AppearanceValue::InsulinPen);
        assert_eq!(value.encoded_data(), &[0x03, 0x19, 0x48, 0x0D]);
        assert!(value.r#type().contains(AdStructType::APPEARANCE));
        assert!(value.is_unique());
    }
}
