use bitflags::bitflags;

use crate::advertising::ad_struct::{
    AdStruct, AdStructType, AD_STRUCT_DATA_OFFSET, AD_STRUCT_LENGTH_OFFSET, AD_STRUCT_TYPE_OFFSET,
};
use crate::assigned_numbers::AdType;

const FLAGS_AD_STRUCT_SIZE: usize = 3;

/// Flags informing about general capabilities of the device.
///
/// The Flags Advertising Structure shall be included when any of the Flag bits are non-zero and the
/// advertising packet is connectable, otherwise the Flags Advertising Structure may be omitted, as
/// defined in [Supplement to the Bluetooth Core Specification, Part A, 1.3](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-801bc3e0-519d-2291-8acd-d32d1fd27a4e).
///
/// See [`Flags`] for more information about each of the flags.
#[derive(Debug, Clone)]
pub struct FlagsAdStruct {
    buffer: [u8; FLAGS_AD_STRUCT_SIZE],
}

impl FlagsAdStruct {
    /// Create a Flags Advertising Structure.
    ///
    /// # Arguments
    ///
    /// * `flags` — The flags to notify.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bletio::advertising::ad_struct::FlagsAdStruct;
    /// # use bletio::advertising::Flags;
    /// let ad_struct = FlagsAdStruct::new(
    ///     Flags::BREDR_NOT_SUPPORTED | Flags::LE_GENERAL_DISCOVERABLE_MODE
    /// );
    /// ```
    pub fn new(flags: Flags) -> Self {
        let mut s = Self {
            buffer: Default::default(),
        };
        s.buffer[AD_STRUCT_LENGTH_OFFSET] = (FLAGS_AD_STRUCT_SIZE - 1) as u8;
        s.buffer[AD_STRUCT_TYPE_OFFSET] = AdType::Flags as u8;
        s.buffer[AD_STRUCT_DATA_OFFSET] = flags.bits();
        s
    }
}

impl AdStruct for FlagsAdStruct {
    fn encoded_data(&self) -> &[u8] {
        &self.buffer
    }
    fn r#type(&self) -> AdStructType {
        AdStructType::FLAGS
    }

    fn is_unique(&self) -> bool {
        true
    }
}

/// Flags to be used in a [FlagsAdStruct](crate::advertising::ad_struct::FlagsAdStruct)
/// Advertising Structure, as defined in
/// [Supplement to the Bluetooth Core Specification, Part A, 1.3](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-801bc3e0-519d-2291-8acd-d32d1fd27a4e).
#[derive(Debug, Clone, Copy)]
pub struct Flags(u8);

bitflags! {
    impl Flags: u8 {
        /// Low-Energy Limited Discoverable Mode.
        const LE_LIMITED_DISCOVERABLE_MODE = 1 << 0;
        /// Low-Energy General Discoverable Mode.
        const LE_GENERAL_DISCOVERABLE_MODE = 1 << 1;
        /// BR/EDR (Bluetooth Classic) not supported.
        const BREDR_NOT_SUPPORTED = 1 << 2;
        /// Simultaneous LE and BR/EDR to Same Device Capable (Controller).
        const SIMULTANEOUS_LE_AND_BREDR_TO_SAME_DEVICE_CAPABLE_CONTROLLER = 1 << 3;
    }
}

impl Default for Flags {
    fn default() -> Self {
        Self::BREDR_NOT_SUPPORTED
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_flags() {
        let mut flags = Flags::default();
        assert_eq!(flags.bits(), Flags::BREDR_NOT_SUPPORTED.bits());
        flags |= Flags::LE_GENERAL_DISCOVERABLE_MODE;
        assert_eq!(flags.bits(), 0x06);
    }

    #[test]
    fn test_flags_ad_struct() {
        let value = FlagsAdStruct::new(Flags::default());
        assert_eq!(value.encoded_data(), &[0x02, 0x01, 0x04]);
        assert!(value.r#type().contains(AdStructType::FLAGS));
        assert!(value.is_unique());

        let value =
            FlagsAdStruct::new(Flags::LE_GENERAL_DISCOVERABLE_MODE | Flags::BREDR_NOT_SUPPORTED);
        assert_eq!(value.encoded_data(), &[0x02, 0x01, 0x06]);
        assert!(value.r#type().contains(AdStructType::FLAGS));
        assert!(value.is_unique());
    }
}
