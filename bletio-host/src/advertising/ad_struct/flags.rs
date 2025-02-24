#[cfg(not(feature = "defmt"))]
use bitflags::bitflags;
#[cfg(feature = "defmt")]
use defmt::bitflags;

use bletio_utils::EncodeToBuffer;

use crate::assigned_numbers::AdType;

const FLAGS_AD_STRUCT_SIZE: usize = 2;

/// Flags informing about general capabilities of the device.
///
/// The Flags Advertising Structure shall be included when any of the Flag bits are non-zero and the
/// advertising packet is connectable, otherwise the Flags Advertising Structure may be omitted, as
/// defined in [Supplement to the Bluetooth Core Specification, Part A, 1.3](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-801bc3e0-519d-2291-8acd-d32d1fd27a4e).
///
/// See [`Flags`] for more information about each of the flags.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FlagsAdStruct {
    flags: Flags,
}

impl FlagsAdStruct {
    pub(crate) const fn new(flags: Flags) -> Self {
        Self { flags }
    }
}

impl EncodeToBuffer for FlagsAdStruct {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push(FLAGS_AD_STRUCT_SIZE as u8)?;
        buffer.try_push(AdType::Flags as u8)?;
        buffer.try_push(self.flags.bits())?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        FLAGS_AD_STRUCT_SIZE + 1
    }
}

bitflags! {
    /// Flags to be used in a [FlagsAdStruct](crate::advertising::ad_struct::FlagsAdStruct)
    /// Advertising Structure, as defined in
    /// [Supplement to the Bluetooth Core Specification, Part A, 1.3](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-801bc3e0-519d-2291-8acd-d32d1fd27a4e).
    #[cfg_attr(not(feature = "defmt"), derive(Debug, Clone, Copy, PartialEq, Eq))]
    pub struct Flags: u8 {
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

impl From<u8> for Flags {
    fn from(value: u8) -> Self {
        Self::from_bits_truncate(value)
    }
}

pub(crate) mod parser {
    use nom::{combinator::map, number::le_u8, IResult, Parser};

    use crate::advertising::ad_struct::AdStruct;

    use super::*;

    pub(crate) fn flags_ad_struct(input: &[u8]) -> IResult<&[u8], AdStruct> {
        map(le_u8(), |v| AdStruct::Flags(FlagsAdStruct::new(v.into()))).parse(input)
    }
}

#[cfg(test)]
mod test {
    use bletio_utils::{Buffer, BufferOps};
    use rstest::rstest;

    use crate::advertising::ad_struct::AdStruct;

    use super::{parser::*, *};

    #[test]
    fn test_flags() {
        let mut flags = Flags::default();
        assert_eq!(flags.bits(), Flags::BREDR_NOT_SUPPORTED.bits());
        flags |= Flags::LE_GENERAL_DISCOVERABLE_MODE;
        assert_eq!(flags.bits(), 0x06);
    }

    #[rstest]
    #[case(Flags::default(), &[0x02, 0x01, 0x04])]
    #[case(Flags::LE_GENERAL_DISCOVERABLE_MODE | Flags::BREDR_NOT_SUPPORTED, &[0x02, 0x01, 0x06])]
    fn test_flags_ad_struct(
        #[case] flags: Flags,
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<3>::default();
        let value = FlagsAdStruct::new(flags);
        value.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }

    #[rstest]
    #[case(&[0x04], Flags::BREDR_NOT_SUPPORTED)]
    #[case(&[0x06], Flags::LE_GENERAL_DISCOVERABLE_MODE | Flags::BREDR_NOT_SUPPORTED)]
    #[case(&[0xFF], Flags::all())]
    fn test_flags_ad_struct_parsing(#[case] input: &[u8], #[case] flags: Flags) {
        assert_eq!(
            flags_ad_struct(input),
            Ok((&[] as &[u8], AdStruct::Flags(FlagsAdStruct::new(flags))))
        );
    }
}
