use bletio_utils::EncodeToBuffer;

use crate::Error;

/// Device Address.
///
/// This is the address and its type that identifies a device.
///
/// Can be:
///  - Public Device Address
///  - Random Device Address
///    - Static Device Address
///    - Resolvable Private Address
///    - Non-resolvable Private Address
///
/// See [Core Specification 6.0, Vol.6, Part B, 1.3](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/low-energy-controller/link-layer-specification.html#UUID-a0f4480f-c97e-c6bc-58ad-6e05d3b7d3a9).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DeviceAddress {
    Public(PublicDeviceAddress),
    Random(RandomAddress),
}

impl DeviceAddress {
    pub const fn value(&self) -> &[u8; 6] {
        match self {
            Self::Public(address) => address.value(),
            Self::Random(address) => address.value(),
        }
    }
}

impl Default for DeviceAddress {
    fn default() -> Self {
        Self::Public(PublicDeviceAddress::default())
    }
}

impl From<PublicDeviceAddress> for DeviceAddress {
    fn from(value: PublicDeviceAddress) -> Self {
        Self::Public(value)
    }
}

impl From<RandomAddress> for DeviceAddress {
    fn from(value: RandomAddress) -> Self {
        Self::Random(value)
    }
}

impl From<RandomStaticDeviceAddress> for DeviceAddress {
    fn from(value: RandomStaticDeviceAddress) -> Self {
        Self::Random(RandomAddress::Static(value))
    }
}

impl From<RandomResolvablePrivateAddress> for DeviceAddress {
    fn from(value: RandomResolvablePrivateAddress) -> Self {
        Self::Random(RandomAddress::ResolvablePrivate(value))
    }
}

impl From<RandomNonResolvablePrivateAddress> for DeviceAddress {
    fn from(value: RandomNonResolvablePrivateAddress) -> Self {
        Self::Random(RandomAddress::NonResolvablePrivate(value))
    }
}

impl EncodeToBuffer for DeviceAddress {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        match self {
            Self::Public(public) => public.encode(buffer),
            Self::Random(random) => random.encode(buffer),
        }
    }

    fn encoded_size(&self) -> usize {
        match self {
            Self::Public(public) => public.encoded_size(),
            Self::Random(random) => random.encoded_size(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct AddressBase {
    value: [u8; 6],
}

impl From<[u8; 6]> for AddressBase {
    fn from(value: [u8; 6]) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PublicDeviceAddress {
    base: AddressBase,
}

impl PublicDeviceAddress {
    /// Create a public device address.
    pub const fn new(address: [u8; 6]) -> Self {
        Self {
            base: AddressBase { value: address },
        }
    }

    pub const fn value(&self) -> &[u8; 6] {
        &self.base.value
    }
}

impl From<[u8; 6]> for PublicDeviceAddress {
    fn from(value: [u8; 6]) -> Self {
        Self::new(value)
    }
}

impl TryFrom<&str> for PublicDeviceAddress {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (_, address) =
            parser::public_address_str(value).map_err(|_| Error::InvalidPublicDeviceAddress)?;
        Ok(address)
    }
}

impl EncodeToBuffer for PublicDeviceAddress {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.copy_from_slice(&self.base.value)
    }

    fn encoded_size(&self) -> usize {
        6
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RandomAddress {
    Static(RandomStaticDeviceAddress),
    ResolvablePrivate(RandomResolvablePrivateAddress),
    NonResolvablePrivate(RandomNonResolvablePrivateAddress),
}

impl RandomAddress {
    pub const fn value(&self) -> &[u8; 6] {
        match self {
            Self::Static(address) => address.value(),
            Self::ResolvablePrivate(address) => address.value(),
            Self::NonResolvablePrivate(address) => address.value(),
        }
    }
}

impl TryFrom<[u8; 6]> for RandomAddress {
    type Error = Error;

    fn try_from(value: [u8; 6]) -> Result<Self, Self::Error> {
        Ok(match value[5] & 0b1100_0000 {
            0b1100_0000 => Self::Static(RandomStaticDeviceAddress::try_from(value)?),
            0b0100_0000 => {
                Self::ResolvablePrivate(RandomResolvablePrivateAddress::try_from(value)?)
            }
            0b0000_0000 => {
                Self::NonResolvablePrivate(RandomNonResolvablePrivateAddress::try_from(value)?)
            }
            _ => return Err(Error::InvalidRandomAddress),
        })
    }
}

impl TryFrom<&str> for RandomAddress {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (_, address) =
            parser::random_address_str(value).map_err(|_| Error::InvalidRandomAddress)?;
        Ok(address)
    }
}

impl From<RandomStaticDeviceAddress> for RandomAddress {
    fn from(value: RandomStaticDeviceAddress) -> Self {
        Self::Static(value)
    }
}

impl From<RandomResolvablePrivateAddress> for RandomAddress {
    fn from(value: RandomResolvablePrivateAddress) -> Self {
        Self::ResolvablePrivate(value)
    }
}

impl From<RandomNonResolvablePrivateAddress> for RandomAddress {
    fn from(value: RandomNonResolvablePrivateAddress) -> Self {
        Self::NonResolvablePrivate(value)
    }
}

impl EncodeToBuffer for RandomAddress {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        match self {
            Self::Static(address) => address.encode(buffer),
            Self::ResolvablePrivate(address) => address.encode(buffer),
            Self::NonResolvablePrivate(address) => address.encode(buffer),
        }
    }

    fn encoded_size(&self) -> usize {
        match self {
            Self::Static(address) => address.encoded_size(),
            Self::ResolvablePrivate(address) => address.encoded_size(),
            Self::NonResolvablePrivate(address) => address.encoded_size(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RandomStaticDeviceAddress {
    base: AddressBase,
}

impl RandomStaticDeviceAddress {
    pub const fn try_new(address: [u8; 6]) -> Result<Self, Error> {
        if Self::is_valid(&address) {
            Ok(Self {
                base: AddressBase { value: address },
            })
        } else {
            Err(Error::InvalidRandomStaticDeviceAddress)
        }
    }

    pub const fn try_new_from_random_bytes(mut bytes: [u8; 6]) -> Result<Self, Error> {
        bytes[5] |= 0b1100_0000;
        Self::try_new(bytes)
    }

    pub const fn value(&self) -> &[u8; 6] {
        &self.base.value
    }

    const fn is_random_static_device_address(address: &[u8; 6]) -> bool {
        (address[5] & 0b1100_0000) == 0b1100_0000
    }

    const fn all_bits_are_zero(address: &[u8; 6]) -> bool {
        let mut i: usize = 0;
        while i < 5 {
            if address[i] != 0x00 {
                return false;
            }
            i += 1;
        }
        address[5] == 0b1100_0000
    }

    const fn all_bits_are_one(address: &[u8; 6]) -> bool {
        let mut i: usize = 0;
        while i < 6 {
            if address[i] != 0xFF {
                return false;
            }
            i += 1;
        }
        true
    }

    const fn is_valid(address: &[u8; 6]) -> bool {
        Self::is_random_static_device_address(address)
            && !Self::all_bits_are_zero(address)
            && !Self::all_bits_are_one(address)
    }
}

impl EncodeToBuffer for RandomStaticDeviceAddress {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.copy_from_slice(&self.base.value)
    }

    fn encoded_size(&self) -> usize {
        6
    }
}

impl TryFrom<[u8; 6]> for RandomStaticDeviceAddress {
    type Error = Error;

    fn try_from(value: [u8; 6]) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RandomResolvablePrivateAddress {
    base: AddressBase,
}

impl RandomResolvablePrivateAddress {
    pub const fn try_new(address: [u8; 6]) -> Result<Self, Error> {
        if Self::is_valid(&address) {
            Ok(Self {
                base: AddressBase { value: address },
            })
        } else {
            Err(Error::InvalidRandomResolvablePrivateAddress)
        }
    }

    pub const fn value(&self) -> &[u8; 6] {
        &self.base.value
    }

    const fn is_random_resolvable_private_address(address: &[u8; 6]) -> bool {
        (address[5] & 0b1100_0000) == 0b0100_0000
    }

    const fn all_bits_are_zero(address: &[u8; 6]) -> bool {
        let mut i: usize = 0;
        while i < 5 {
            if address[i] != 0x00 {
                return false;
            }
            i += 1;
        }
        address[5] == 0b0100_0000
    }

    const fn all_bits_are_one(address: &[u8; 6]) -> bool {
        let mut i: usize = 0;
        while i < 5 {
            if address[i] != 0xFF {
                return false;
            }
            i += 1;
        }
        address[5] == 0b0111_1111
    }

    const fn is_valid(address: &[u8; 6]) -> bool {
        Self::is_random_resolvable_private_address(address)
            && !Self::all_bits_are_zero(address)
            && !Self::all_bits_are_one(address)
    }
}

impl TryFrom<[u8; 6]> for RandomResolvablePrivateAddress {
    type Error = Error;

    fn try_from(value: [u8; 6]) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl EncodeToBuffer for RandomResolvablePrivateAddress {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.copy_from_slice(&self.base.value)
    }

    fn encoded_size(&self) -> usize {
        6
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RandomNonResolvablePrivateAddress {
    base: AddressBase,
}

impl RandomNonResolvablePrivateAddress {
    pub const fn try_new(address: [u8; 6]) -> Result<Self, Error> {
        if Self::is_valid(&address) {
            Ok(Self {
                base: AddressBase { value: address },
            })
        } else {
            Err(Error::InvalidRandomNonResolvablePrivateAddress)
        }
    }

    pub const fn value(&self) -> &[u8; 6] {
        &self.base.value
    }

    const fn is_random_non_resolvable_private_address(address: &[u8; 6]) -> bool {
        (address[5] & 0b1100_0000) == 0b0000_0000
    }

    const fn all_bits_are_zero(address: &[u8; 6]) -> bool {
        let mut i: usize = 3;
        while i < 6 {
            if address[i] != 0x00 {
                return false;
            }
            i += 1;
        }
        true
    }

    const fn all_bits_are_one(address: &[u8; 6]) -> bool {
        let mut i: usize = 3;
        while i < 5 {
            if address[i] != 0xFF {
                return false;
            }
            i += 1;
        }
        address[5] == 0b0011_1111
    }

    const fn is_valid(address: &[u8; 6]) -> bool {
        Self::is_random_non_resolvable_private_address(address)
            && !Self::all_bits_are_zero(address)
            && !Self::all_bits_are_one(address)
    }
}

impl TryFrom<[u8; 6]> for RandomNonResolvablePrivateAddress {
    type Error = Error;

    fn try_from(value: [u8; 6]) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl EncodeToBuffer for RandomNonResolvablePrivateAddress {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.copy_from_slice(&self.base.value)
    }

    fn encoded_size(&self) -> usize {
        6
    }
}

pub(crate) mod parser {
    use nom::{
        bytes::{tag, take},
        character::complete::hex_digit1,
        combinator::{all_consuming, map, map_res, verify},
        IResult, Parser,
    };

    use super::*;

    fn convert_str_digits_to_u8(value: &str) -> u8 {
        let mut it = value.chars();
        let digit1 = it.next().and_then(|v| v.to_digit(16)).unwrap();
        let digit2 = it.next().and_then(|v| v.to_digit(16)).unwrap();
        ((digit1 << 4) | digit2) as u8
    }

    fn hex_byte(input: &str) -> IResult<&str, u8> {
        map(
            verify(hex_digit1, |v: &str| v.len() == 2),
            convert_str_digits_to_u8,
        )
        .parse(input)
    }

    fn separator(input: &str) -> IResult<&str, &str> {
        tag(":").parse(input)
    }

    fn address(input: &str) -> IResult<&str, [u8; 6]> {
        all_consuming(map(
            (
                hex_byte, separator, hex_byte, separator, hex_byte, separator, hex_byte, separator,
                hex_byte, separator, hex_byte,
            ),
            |(byte0, _, byte1, _, byte2, _, byte3, _, byte4, _, byte5)| {
                [byte5, byte4, byte3, byte2, byte1, byte0]
            },
        ))
        .parse(input)
    }

    pub(crate) fn public_address_str(input: &str) -> IResult<&str, PublicDeviceAddress> {
        map(address, Into::into).parse(input)
    }

    pub(crate) fn random_address_str(input: &str) -> IResult<&str, RandomAddress> {
        map_res(address, TryInto::try_into).parse(input)
    }

    pub(crate) fn random_address(input: &[u8]) -> IResult<&[u8], RandomStaticDeviceAddress> {
        all_consuming(map_res(
            map_res(take(6u8), TryInto::try_into),
            |v: [u8; 6]| v.try_into(),
        ))
        .parse(input)
    }
}

#[cfg(test)]
mod test {
    use bletio_utils::{Buffer, BufferOps};
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case([0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56])]
    #[case([0xF4, 0x23, 0x14, 0xC3, 0xDC, 0x24])]
    #[case([0x38, 0x5E, 0x43, 0xCA, 0x4C, 0x40])]
    fn test_address_base(#[case] input: [u8; 6]) {
        let address_base: AddressBase = input.into();
        assert_eq!(address_base.value, input);
    }

    #[rstest]
    #[case([0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56])]
    #[case([0xF4, 0x23, 0x14, 0xC3, 0xDC, 0x24])]
    #[case([0x38, 0x5E, 0x43, 0xCA, 0x4C, 0x40])]
    fn test_public_device_address(#[case] input: [u8; 6]) {
        let address = PublicDeviceAddress::new(input);
        assert_eq!(address.value(), &input);
        let mut buffer = Buffer::<6>::default();
        assert_eq!(address.encoded_size(), input.len());
        address.encode(&mut buffer).unwrap();
        assert_eq!(buffer.data(), input);
        let address: DeviceAddress = address.into();
        assert!(matches!(address, DeviceAddress::Public(_)));
        assert_eq!(address.value(), &input);
    }

    #[rstest]
    #[case([0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0xF7])]
    #[case([0xFE, 0x92, 0x2F, 0x0F, 0x4B, 0xD2])]
    #[case([0xC6, 0x37, 0x7B, 0xF9, 0x7B, 0xC2])]
    fn test_valid_random_static_device_address(#[case] input: [u8; 6]) -> Result<(), Error> {
        let random_static_device_address = RandomStaticDeviceAddress::try_new(input)?;
        assert_eq!(random_static_device_address.value(), &input);
        let mut buffer = Buffer::<6>::default();
        assert_eq!(random_static_device_address.encoded_size(), input.len());
        random_static_device_address.encode(&mut buffer).unwrap();
        assert_eq!(buffer.data(), input);
        let random_address: RandomAddress = random_static_device_address.clone().into();
        assert!(matches!(random_address, RandomAddress::Static(_)));
        assert_eq!(random_address.value(), &input);
        let mut buffer = Buffer::<6>::default();
        assert_eq!(random_address.encoded_size(), input.len());
        random_address.encode(&mut buffer).unwrap();
        assert_eq!(buffer.data(), input);
        let device_address: DeviceAddress = random_static_device_address.into();
        assert!(matches!(
            device_address,
            DeviceAddress::Random(RandomAddress::Static(_))
        ));
        assert_eq!(device_address.value(), &input);
        let device_address: DeviceAddress = random_address.into();
        assert!(matches!(
            device_address,
            DeviceAddress::Random(RandomAddress::Static(_))
        ));
        assert_eq!(device_address.value(), &input);
        let mut buffer = Buffer::<6>::default();
        assert_eq!(device_address.encoded_size(), input.len());
        device_address.encode(&mut buffer).unwrap();
        assert_eq!(buffer.data(), input);
        Ok(())
    }

    #[rstest]
    #[case([0xff, 0xff, 0xff, 0xff, 0xff, 0xff])]
    #[case([0x00, 0x00, 0x00, 0x00, 0x00, 0xc0])]
    fn test_invalid_random_static_device_address(#[case] input: [u8; 6]) {
        let result = RandomStaticDeviceAddress::try_new(input);
        assert_eq!(result, Err(Error::InvalidRandomStaticDeviceAddress));
    }

    #[test]
    fn test_random_static_device_address_try_new_from_random_bytes() -> Result<(), Error> {
        let random_address =
            RandomStaticDeviceAddress::try_new_from_random_bytes([68, 223, 27, 9, 83, 58])?;
        assert_eq!(
            random_address.value(),
            &[0x44, 0xDF, 0x1B, 0x09, 0x53, 0xFA]
        );
        Ok(())
    }

    #[rstest]
    #[case([0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0x77])]
    #[case([0xFE, 0x92, 0x2F, 0x0F, 0x4B, 0x52])]
    #[case([0xC6, 0x37, 0x7B, 0xF9, 0x7B, 0x42])]
    fn test_valid_random_resolvable_private_address(#[case] input: [u8; 6]) -> Result<(), Error> {
        let random_resolvable_private_address = RandomResolvablePrivateAddress::try_new(input)?;
        assert_eq!(random_resolvable_private_address.value(), &input);
        let mut buffer = Buffer::<6>::default();
        assert_eq!(
            random_resolvable_private_address.encoded_size(),
            input.len()
        );
        random_resolvable_private_address
            .encode(&mut buffer)
            .unwrap();
        assert_eq!(buffer.data(), input);
        let random_address: RandomAddress = random_resolvable_private_address.clone().into();
        assert!(matches!(
            random_address,
            RandomAddress::ResolvablePrivate(_)
        ));
        assert_eq!(random_address.value(), &input);
        let mut buffer = Buffer::<6>::default();
        assert_eq!(random_address.encoded_size(), input.len());
        random_address.encode(&mut buffer).unwrap();
        assert_eq!(buffer.data(), input);
        let device_address: DeviceAddress = random_resolvable_private_address.into();
        assert!(matches!(
            device_address,
            DeviceAddress::Random(RandomAddress::ResolvablePrivate(_))
        ));
        assert_eq!(device_address.value(), &input);
        let device_address: DeviceAddress = random_address.into();
        assert!(matches!(
            device_address,
            DeviceAddress::Random(RandomAddress::ResolvablePrivate(_))
        ));
        assert_eq!(device_address.value(), &input);
        let mut buffer = Buffer::<6>::default();
        assert_eq!(device_address.encoded_size(), input.len());
        device_address.encode(&mut buffer).unwrap();
        assert_eq!(buffer.data(), input);
        Ok(())
    }

    #[rstest]
    #[case([0xff, 0xff, 0xff, 0xff, 0xff, 0x7f])]
    #[case([0x00, 0x00, 0x00, 0x00, 0x00, 0x40])]
    fn test_invalid_random_resolvable_private_address(#[case] input: [u8; 6]) {
        let result = RandomResolvablePrivateAddress::try_new(input);
        assert_eq!(result, Err(Error::InvalidRandomResolvablePrivateAddress));
    }

    #[rstest]
    #[case([0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0x37])]
    #[case([0xFE, 0x92, 0x2F, 0x0F, 0x4B, 0x12])]
    #[case([0xC6, 0x37, 0x7B, 0xF9, 0x7B, 0x02])]
    fn test_valid_random_non_resolvable_private_address(
        #[case] input: [u8; 6],
    ) -> Result<(), Error> {
        let random_non_resolvable_private_address =
            RandomNonResolvablePrivateAddress::try_new(input)?;
        assert_eq!(random_non_resolvable_private_address.value(), &input);
        let mut buffer = Buffer::<6>::default();
        assert_eq!(
            random_non_resolvable_private_address.encoded_size(),
            input.len()
        );
        random_non_resolvable_private_address
            .encode(&mut buffer)
            .unwrap();
        assert_eq!(buffer.data(), input);
        let random_address: RandomAddress = random_non_resolvable_private_address.clone().into();
        assert!(matches!(
            random_address,
            RandomAddress::NonResolvablePrivate(_)
        ));
        assert_eq!(random_address.value(), &input);
        let mut buffer = Buffer::<6>::default();
        assert_eq!(random_address.encoded_size(), input.len());
        random_address.encode(&mut buffer).unwrap();
        assert_eq!(buffer.data(), input);
        let device_address: DeviceAddress = random_non_resolvable_private_address.into();
        assert!(matches!(
            device_address,
            DeviceAddress::Random(RandomAddress::NonResolvablePrivate(_))
        ));
        assert_eq!(device_address.value(), &input);
        let device_address: DeviceAddress = random_address.into();
        assert!(matches!(
            device_address,
            DeviceAddress::Random(RandomAddress::NonResolvablePrivate(_))
        ));
        assert_eq!(device_address.value(), &input);
        let mut buffer = Buffer::<6>::default();
        assert_eq!(device_address.encoded_size(), input.len());
        device_address.encode(&mut buffer).unwrap();
        assert_eq!(buffer.data(), input);
        Ok(())
    }

    #[rstest]
    #[case([0xff, 0xff, 0xff, 0xff, 0xff, 0x3f])]
    #[case([0x00, 0x00, 0x00, 0xff, 0xff, 0x3f])]
    #[case([0x28, 0xc8, 0xe9, 0xff, 0xff, 0x3f])]
    #[case([0xfe, 0x92, 0x2f, 0xff, 0xff, 0x3f])]
    #[case([0xc6, 0x37, 0x7b, 0xff, 0xff, 0x3f])]
    #[case([0x00, 0x00, 0x00, 0x00, 0x00, 0x00])]
    #[case([0xff, 0xff, 0xff, 0x00, 0x00, 0x00])]
    #[case([0x28, 0xc8, 0xe9, 0x00, 0x00, 0x00])]
    #[case([0xfe, 0x92, 0x2f, 0x00, 0x00, 0x00])]
    #[case([0xc6, 0x37, 0x7b, 0x00, 0x00, 0x00])]
    fn test_invalid_random_non_resolvable_private_address(#[case] input: [u8; 6]) {
        let result = RandomNonResolvablePrivateAddress::try_new(input);
        assert_eq!(result, Err(Error::InvalidRandomNonResolvablePrivateAddress));
    }

    #[rstest]
    #[case("56:32:04:0B:2E:CD", Ok(PublicDeviceAddress::new([0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56])))]
    #[case("56:32:04:0b:2e:cd", Ok(PublicDeviceAddress::new([0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56])))]
    #[case("24:dc:c3:14:23:f4", Ok(PublicDeviceAddress::new([0xF4, 0x23, 0x14, 0xC3, 0xDC, 0x24])))]
    #[case("40:4c:ca:43:5e:38", Ok(PublicDeviceAddress::new([0x38, 0x5E, 0x43, 0xCA, 0x4C, 0x40])))]
    #[case::public_address_too_long("24:dc:c3:14:23:f4:cd", Err(Error::InvalidPublicDeviceAddress))]
    #[case::public_address_too_short("24:dc:c3:14:23", Err(Error::InvalidPublicDeviceAddress))]
    #[case::public_address_with_invalid_chars(
        "40:4g:ca:43:5z:38",
        Err(Error::InvalidPublicDeviceAddress)
    )]
    #[case::public_address_from_invalid_str("hello world", Err(Error::InvalidPublicDeviceAddress))]
    fn test_public_device_address_from_str(
        #[case] input: &str,
        #[case] expected: Result<PublicDeviceAddress, Error>,
    ) {
        assert_eq!(input.try_into(), expected);
    }

    #[rstest]
    #[case::static_device_address(
        "f7:6a:7d:e9:c8:28",
        Ok(RandomAddress::Static(RandomStaticDeviceAddress::try_new([0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0xF7]).unwrap()))
    )]
    #[case::static_device_address(
        "d2:4b:0f:2f:92:fe",
        Ok(RandomAddress::Static(RandomStaticDeviceAddress::try_new([0xFE, 0x92, 0x2F, 0x0F, 0x4B, 0xD2]).unwrap()))
    )]
    #[case::static_device_address(
        "c2:7b:f9:7b:37:c6",
        Ok(RandomAddress::Static(RandomStaticDeviceAddress::try_new([0xC6, 0x37, 0x7B, 0xF9, 0x7B, 0xC2]).unwrap()))
    )]
    #[case::static_device_address_only_1_bits(
        "ff:ff:ff:ff:ff:ff",
        Err(Error::InvalidRandomAddress)
    )]
    #[case::static_device_address_only_0_bits(
        "c0:00:00:00:00:00",
        Err(Error::InvalidRandomAddress)
    )]
    #[case::random_resolvable_private_address(
        "77:6a:7d:e9:c8:28",
        Ok(RandomAddress::ResolvablePrivate(RandomResolvablePrivateAddress::try_new([0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0x77]).unwrap()))
    )]
    #[case::random_resolvable_private_address(
        "52:4b:0f:2f:92:fe",
        Ok(RandomAddress::ResolvablePrivate(RandomResolvablePrivateAddress::try_new([0xFE, 0x92, 0x2F, 0x0F, 0x4B, 0x52]).unwrap()))
    )]
    #[case::random_resolvable_private_address(
        "42:7b:f9:7b:37:c6",
        Ok(RandomAddress::ResolvablePrivate(RandomResolvablePrivateAddress::try_new([0xC6, 0x37, 0x7B, 0xF9, 0x7B, 0x42]).unwrap()))
    )]
    #[case::random_resolvable_private_address_only_1_bits(
        "7f:ff:ff:ff:ff:ff",
        Err(Error::InvalidRandomAddress)
    )]
    #[case::random_resolvable_private_address_only_0_bits(
        "40:00:00:00:00:00",
        Err(Error::InvalidRandomAddress)
    )]
    #[case::random_non_resolvable_private_address(
        "37:6a:7d:e9:c8:28",
        Ok(RandomAddress::NonResolvablePrivate(RandomNonResolvablePrivateAddress::try_new([0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0x37]).unwrap()))
    )]
    #[case::random_non_resolvable_private_address(
        "12:4b:0f:2f:92:fe",
        Ok(RandomAddress::NonResolvablePrivate(RandomNonResolvablePrivateAddress::try_new([0xFE, 0x92, 0x2F, 0x0F, 0x4B, 0x12]).unwrap()))
    )]
    #[case::random_non_resolvable_private_address(
        "02:7b:f9:7b:37:c6",
        Ok(RandomAddress::NonResolvablePrivate(RandomNonResolvablePrivateAddress::try_new([0xC6, 0x37, 0x7B, 0xF9, 0x7B, 0x02]).unwrap()))
    )]
    #[case::random_non_resolvable_private_address_only_1_bits(
        "3f:ff:ff:7b:37:c6",
        Err(Error::InvalidRandomAddress)
    )]
    #[case::random_non_resolvable_private_address_only_0_bits(
        "00:00:00:e9:c8:28",
        Err(Error::InvalidRandomAddress)
    )]
    #[case::random_address_too_long("f7:6a:7d:e9:c8:28:c0", Err(Error::InvalidRandomAddress))]
    #[case::random_address_too_short("f7:6a:7d:e9:c8", Err(Error::InvalidRandomAddress))]
    #[case::random_address_with_invalid_chars(
        "f7:6n:7d:h9:c8:28",
        Err(Error::InvalidRandomAddress)
    )]
    #[case::random_address_with_invalid_str("hello world", Err(Error::InvalidRandomAddress))]
    #[case::random_address_with_invalid_subtype(
        "b7:6a:7d:e9:c8:28",
        Err(Error::InvalidRandomAddress)
    )]
    fn test_random_address_from_str(
        #[case] input: &str,
        #[case] expected: Result<RandomAddress, Error>,
    ) {
        assert_eq!(input.try_into(), expected);
    }
}
