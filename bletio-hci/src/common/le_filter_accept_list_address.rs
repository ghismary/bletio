use bletio_utils::EncodeToBuffer;
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{DeviceAddress, Error, PublicDeviceAddress, RandomAddress};

/// Address type contained in a LE FilterAccept List Add/Remove command.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.16](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-41d5d4cf-0666-b270-9079-fd7da56c896a)
/// & [Core Specification 6.0, Vol.4, Part E, 7.8.17](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-8f8e1acb-23ba-5c1f-9e05-dc9331a9a86d)
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[num_enum(error_type(name = Error, constructor = Error::InvalidLeFilterAcceptListAddressType))]
#[repr(u8)]
#[non_exhaustive]
pub(crate) enum LeFilterAcceptListAddressType {
    /// Public device address.
    PublicDevice = 0x00,
    /// Random device address.
    RandomDevice = 0x01,
    /// Devices sending anonymous advertisements.
    Anonymous = 0xff,
}

/// Address contained in a LE Filter Accept List Add/Remove command.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.16](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-41d5d4cf-0666-b270-9079-fd7da56c896a)
/// & [Core Specification 6.0, Vol.4, Part E, 7.8.17](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-8f8e1acb-23ba-5c1f-9e05-dc9331a9a86d)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LeFilterAcceptListAddress {
    PublicDevice(PublicDeviceAddress),
    RandomDevice(RandomAddress),
    Anonymous,
}

impl LeFilterAcceptListAddress {
    pub(crate) fn r#type(&self) -> LeFilterAcceptListAddressType {
        match self {
            Self::PublicDevice(_) => LeFilterAcceptListAddressType::PublicDevice,
            Self::RandomDevice(_) => LeFilterAcceptListAddressType::RandomDevice,
            Self::Anonymous => LeFilterAcceptListAddressType::Anonymous,
        }
    }

    pub(crate) fn value(&self) -> &[u8; 6] {
        match self {
            Self::PublicDevice(address) => address.value(),
            Self::RandomDevice(address) => address.value(),
            Self::Anonymous => &[0u8; 6],
        }
    }
}

impl From<DeviceAddress> for LeFilterAcceptListAddress {
    fn from(value: DeviceAddress) -> Self {
        match value {
            DeviceAddress::Public(address) => LeFilterAcceptListAddress::PublicDevice(address),
            DeviceAddress::Random(address) => LeFilterAcceptListAddress::RandomDevice(address),
        }
    }
}

impl From<PublicDeviceAddress> for LeFilterAcceptListAddress {
    fn from(value: PublicDeviceAddress) -> Self {
        LeFilterAcceptListAddress::PublicDevice(value)
    }
}

impl From<RandomAddress> for LeFilterAcceptListAddress {
    fn from(value: RandomAddress) -> Self {
        LeFilterAcceptListAddress::RandomDevice(value)
    }
}

impl EncodeToBuffer for LeFilterAcceptListAddress {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push(self.r#type().into())?;
        buffer.copy_from_slice(self.value())?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        7
    }
}

pub(crate) mod parser {
    use nom::{
        bytes::take,
        combinator::{all_consuming, map, map_res},
        number::complete::le_u8,
        IResult, Parser,
    };

    use crate::common::device_address::parser::address;

    use super::*;

    fn le_filter_accept_list_address_type(
        input: &[u8],
    ) -> IResult<&[u8], LeFilterAcceptListAddressType> {
        map_res(le_u8, TryFrom::try_from).parse(input)
    }

    fn le_filter_accept_list_address_public(
        input: &[u8],
    ) -> IResult<&[u8], LeFilterAcceptListAddress> {
        all_consuming(map(
            map_res(address, PublicDeviceAddress::try_from),
            Into::into,
        ))
        .parse(input)
    }

    fn le_filter_accept_list_address_random(
        input: &[u8],
    ) -> IResult<&[u8], LeFilterAcceptListAddress> {
        all_consuming(map(map_res(address, RandomAddress::try_from), Into::into)).parse(input)
    }

    pub(crate) fn le_filter_accept_list_address(
        input: &[u8],
    ) -> IResult<&[u8], LeFilterAcceptListAddress> {
        let (rest, address_type) = le_filter_accept_list_address_type.parse(input)?;
        Ok(match address_type {
            LeFilterAcceptListAddressType::PublicDevice => {
                le_filter_accept_list_address_public(rest)?
            }
            LeFilterAcceptListAddressType::RandomDevice => {
                le_filter_accept_list_address_random(rest)?
            }
            LeFilterAcceptListAddressType::Anonymous => {
                let (rest, _) = all_consuming(take(6u8)).parse(rest)?;
                (rest, LeFilterAcceptListAddress::Anonymous)
            }
        })
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::RandomStaticDeviceAddress;

    use super::{parser::*, *};

    #[rstest]
    #[case(
        PublicDeviceAddress::new([0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56]).into(),
        LeFilterAcceptListAddressType::PublicDevice,
        &[0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56]
    )]
    #[case(
        RandomAddress::Static(RandomStaticDeviceAddress::try_new([0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0xF7]).unwrap()).into(),
        LeFilterAcceptListAddressType::RandomDevice,
        &[0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0xF7]
    )]
    fn test_le_filter_accept_list_address_non_anonymous(
        #[case] address: DeviceAddress,
        #[case] expected_type: LeFilterAcceptListAddressType,
        #[case] expected_value: &[u8],
    ) {
        let address: LeFilterAcceptListAddress = address.into();
        assert_eq!(address.r#type(), expected_type);
        assert_eq!(address.value(), expected_value);
    }

    #[test]
    fn test_le_filter_accept_list_address_anonymous() {
        let address = LeFilterAcceptListAddress::Anonymous;
        assert_eq!(address.r#type(), LeFilterAcceptListAddressType::Anonymous);
        assert_eq!(address.value(), &[0u8; 6]);
    }

    #[test]
    fn test_le_filter_accept_list_address_from_public_address() {
        let data = [0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56];
        let address = PublicDeviceAddress::new(data);
        let address: LeFilterAcceptListAddress = address.into();
        assert_eq!(
            address.r#type(),
            LeFilterAcceptListAddressType::PublicDevice
        );
        assert_eq!(address.value(), &data);
    }

    #[test]
    fn test_le_filter_accept_list_address_from_random_address() {
        let data = [0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0xF7];
        let address = RandomAddress::Static(RandomStaticDeviceAddress::try_new(data).unwrap());
        let address: LeFilterAcceptListAddress = address.into();
        assert_eq!(
            address.r#type(),
            LeFilterAcceptListAddressType::RandomDevice
        );
        assert_eq!(address.value(), &data);
    }

    #[rstest]
    #[case(
        &[0x00, 0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56],
        PublicDeviceAddress::new([0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56]).into()
    )]
    #[case(
        &[0x01, 0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0xF7],
        RandomAddress::Static(RandomStaticDeviceAddress::try_new([0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0xF7]).unwrap()).into()
    )]
    #[case(&[0xFF, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05], LeFilterAcceptListAddress::Anonymous)]
    fn test_le_filter_accept_list_address_parsing(
        #[case] input: &[u8],
        #[case] expected_address: LeFilterAcceptListAddress,
    ) {
        assert_eq!(
            le_filter_accept_list_address(input),
            Ok((&[] as &[u8], expected_address))
        );
    }
}
