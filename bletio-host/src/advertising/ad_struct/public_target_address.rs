use core::ops::Deref;

use bletio_hci::PublicDeviceAddress;
use bletio_utils::EncodeToBuffer;
use heapless::Vec;

use crate::{advertising::AdvertisingError, assigned_numbers::AdType};

const PUBLIC_TARGET_ADDRESS_NB_MAX_ADDRESSES: usize = 4;

/// Public Target Address list.
///
/// This defines the address of one or more intended recipients of an advertisement when one
/// or more devices were bonded using a public address. This data type is intended to be used
/// to avoid a situation where a bonded device unnecessarily responds to an advertisement
/// intended for another bonded device.
///
/// See [Supplement to the Bluetooth Core Specification, Part A, 1.13](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-d42b32b3-1877-b82c-fd79-5d755328de9f).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PublicTargetAddressAdStruct {
    addresses: Vec<PublicDeviceAddress, PUBLIC_TARGET_ADDRESS_NB_MAX_ADDRESSES>,
}

impl PublicTargetAddressAdStruct {
    pub(crate) fn try_new(addresses: &[PublicDeviceAddress]) -> Result<Self, AdvertisingError> {
        if addresses.is_empty() {
            Err(AdvertisingError::PublicTargetAddressAdStructMustContainAtLeastOneAddress)
        } else {
            Ok(Self {
                addresses: addresses
                    .try_into()
                    .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?,
            })
        }
    }
}

impl Deref for PublicTargetAddressAdStruct {
    type Target = Vec<PublicDeviceAddress, PUBLIC_TARGET_ADDRESS_NB_MAX_ADDRESSES>;

    fn deref(&self) -> &Self::Target {
        &self.addresses
    }
}

impl EncodeToBuffer for PublicTargetAddressAdStruct {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push((self.encoded_size() - 1) as u8)?;
        buffer.try_push(AdType::PublicTargetAddress as u8)?;
        for address in self.addresses.iter() {
            address.encode(buffer)?;
        }
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        (self.addresses.len() * self.addresses[0].encoded_size()) + 2
    }
}

pub(crate) mod parser {
    use bletio_hci::common::device_address::parser::address;
    use nom::{
        combinator::{fail, map, map_res},
        IResult, Parser,
    };

    use crate::advertising::ad_struct::AdStruct;

    use super::*;

    pub(crate) fn public_target_address_ad_struct(mut input: &[u8]) -> IResult<&[u8], AdStruct> {
        let len = input.len() / 6;
        if (len > PUBLIC_TARGET_ADDRESS_NB_MAX_ADDRESSES) || ((input.len() % 6) != 0) {
            fail::<_, &[u8], _>().parse(input)?;
        }
        let mut ad_struct = PublicTargetAddressAdStruct {
            addresses: Default::default(),
        };
        let mut index = 0;
        while index < len {
            let (rest, _) = map_res(map(address, PublicDeviceAddress::new), |address| {
                ad_struct.addresses.push(address)
            })
            .parse(input)?;
            input = rest;
            index += 1;
        }
        Ok((&[], AdStruct::PublicTargetAddress(ad_struct)))
    }
}

#[cfg(test)]
mod test {
    use bletio_utils::{Buffer, BufferOps};
    use rstest::rstest;

    use crate::advertising::ad_struct::AdStruct;

    use super::{parser::*, *};

    #[rstest]
    #[case(
        &[PublicDeviceAddress::new([0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56])],
        &[0x07, 0x17, 0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56])]
    #[case(
        &[PublicDeviceAddress::new([0xF4, 0x23, 0x14, 0xC3, 0xDC, 0x24]), PublicDeviceAddress::new([0x38, 0x5E, 0x43, 0xCA, 0x4C, 0x40])],
        &[0x0D, 0x17, 0xF4, 0x23, 0x14, 0xC3, 0xDC, 0x24, 0x38, 0x5E, 0x43, 0xCA, 0x4C, 0x40]
    )]
    fn test_public_target_address_ad_struct_success(
        #[case] addresses: &[PublicDeviceAddress],
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<31>::default();
        let ad_struct = PublicTargetAddressAdStruct::try_new(addresses).unwrap();
        ad_struct.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        assert_eq!(ad_struct.iter().count(), addresses.len());
        Ok(())
    }

    #[test]
    fn test_public_target_address_ad_struct_failure_empty() {
        let err = PublicTargetAddressAdStruct::try_new(&[]);
        assert_eq!(
            err,
            Err(AdvertisingError::PublicTargetAddressAdStructMustContainAtLeastOneAddress)
        );
    }

    #[test]
    fn test_public_target_address_ad_struct_failure_too_big() {
        let addresses = [
            PublicDeviceAddress::new([0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56]),
            PublicDeviceAddress::new([0xF4, 0x23, 0x14, 0xC3, 0xDC, 0x24]),
            PublicDeviceAddress::new([0x38, 0x5E, 0x43, 0xCA, 0x4C, 0x40]),
            PublicDeviceAddress::new([0xCE, 0x2E, 0x0B, 0x04, 0x32, 0x56]),
            PublicDeviceAddress::new([0xF5, 0x23, 0x14, 0xC3, 0xDC, 0x24]),
            PublicDeviceAddress::new([0x39, 0x5E, 0x43, 0xCA, 0x4C, 0x40]),
        ];
        let err = PublicTargetAddressAdStruct::try_new(&addresses);
        assert_eq!(
            err,
            Err(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
        );
    }

    #[rstest]
    #[case(&[0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56], &[PublicDeviceAddress::new([0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56])])]
    #[case(
        &[0xF4, 0x23, 0x14, 0xC3, 0xDC, 0x24, 0x38, 0x5E, 0x43, 0xCA, 0x4C, 0x40],
        &[PublicDeviceAddress::new([0xF4, 0x23, 0x14, 0xC3, 0xDC, 0x24]), PublicDeviceAddress::new([0x38, 0x5E, 0x43, 0xCA, 0x4C, 0x40])]
    )]
    fn test_public_target_address_ad_struct_parsing_success(
        #[case] input: &[u8],
        #[case] addresses: &[PublicDeviceAddress],
    ) {
        assert_eq!(
            public_target_address_ad_struct(input),
            Ok((
                &[] as &[u8],
                AdStruct::PublicTargetAddress(
                    PublicTargetAddressAdStruct::try_new(addresses).unwrap()
                )
            ))
        );
    }

    #[rstest]
    #[case(
        &[0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56, 0xF4, 0x23, 0x14, 0xC3, 0xDC, 0x24, 0x38, 0x5E, 0x43,
            0xCA, 0x4C, 0x40, 0xCE, 0x2E, 0x0B, 0x04, 0x32, 0x56, 0xF5, 0x23, 0x14, 0xC3, 0xDC, 0x24]
    )]
    #[case(
        &[0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56, 0xF4, 0x23, 0x14, 0xC3, 0xDC, 0x24, 0x38, 0x5E, 0x43]
    )]
    fn test_public_target_address_ad_struct_parsing_failure(#[case] input: &[u8]) {
        assert!(public_target_address_ad_struct(input).is_err());
    }
}
