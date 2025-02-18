use bletio_hci::RandomAddress;
use bletio_utils::EncodeToBuffer;
use heapless::Vec;

use crate::{advertising::AdvertisingError, assigned_numbers::AdType};

const RANDOM_TARGET_ADDRESS_NB_MAX_ADDRESSES: usize = 4;

/// Random Target Address list.
///
/// This defines the address of one or more intended recipients of an advertisement when one
/// or more devices were bonded using a random address. This data type is intended to be used
/// to avoid a situation where a bonded device unnecessarily responds to an advertisement
/// intended for another bonded device.
///
/// See [Supplement to the Bluetooth Core Specification, Part A, 1.14](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-9c825fe7-7092-a219-ecb4-2294c2c12d9a).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct RandomTargetAddressAdStruct {
    addresses: Vec<RandomAddress, RANDOM_TARGET_ADDRESS_NB_MAX_ADDRESSES>,
}

impl RandomTargetAddressAdStruct {
    // TODO: Limit to random static and random resolvable addresses
    pub(crate) fn try_new(addresses: &[RandomAddress]) -> Result<Self, AdvertisingError> {
        if addresses.is_empty() {
            Err(AdvertisingError::RandomTargetAddressAdStructMustContainAtLeastOneAddress)
        } else {
            Ok(Self {
                addresses: addresses
                    .try_into()
                    .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?,
            })
        }
    }
}

impl EncodeToBuffer for RandomTargetAddressAdStruct {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push((self.encoded_size() - 1) as u8)?;
        buffer.try_push(AdType::RandomTargetAddress as u8)?;
        for address in self.addresses.iter() {
            address.encode(buffer)?;
        }
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        (self.addresses.len() * self.addresses[0].encoded_size()) + 2
    }
}

#[cfg(test)]
mod test {
    use bletio_hci::{RandomResolvablePrivateAddress, RandomStaticDeviceAddress};
    use bletio_utils::{Buffer, BufferOps};
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        &[RandomStaticDeviceAddress::try_new([0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0xF7]).unwrap().into()],
        &[0x07, 0x18, 0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0xF7])]
    #[case(
        &[RandomResolvablePrivateAddress::try_new([0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0x77]).unwrap().into(),
            RandomStaticDeviceAddress::try_new([0xFE, 0x92, 0x2F, 0x0F, 0x4B, 0xD2]).unwrap().into()],
        &[0x0D, 0x18, 0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0x77, 0xFE, 0x92, 0x2F, 0x0F, 0x4B, 0xD2]
    )]
    fn test_random_target_address_ad_struct_success(
        #[case] addresses: &[RandomAddress],
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<31>::default();
        let value = RandomTargetAddressAdStruct::try_new(addresses).unwrap();
        value.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }

    #[test]
    fn test_random_target_address_ad_struct_failure_empty() {
        let err = RandomTargetAddressAdStruct::try_new(&[]);
        assert_eq!(
            err,
            Err(AdvertisingError::RandomTargetAddressAdStructMustContainAtLeastOneAddress)
        );
    }

    #[test]
    fn test_random_target_address_ad_struct_failure_too_big() {
        let addresses = [
            RandomStaticDeviceAddress::try_new([0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0xF7])
                .unwrap()
                .into(),
            RandomStaticDeviceAddress::try_new([0xFE, 0x92, 0x2F, 0x0F, 0x4B, 0xD2])
                .unwrap()
                .into(),
            RandomStaticDeviceAddress::try_new([0xC6, 0x37, 0x7B, 0xF9, 0x7B, 0xC2])
                .unwrap()
                .into(),
            RandomResolvablePrivateAddress::try_new([0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0x77])
                .unwrap()
                .into(),
            RandomResolvablePrivateAddress::try_new([0xFE, 0x92, 0x2F, 0x0F, 0x4B, 0x52])
                .unwrap()
                .into(),
            RandomResolvablePrivateAddress::try_new([0xC6, 0x37, 0x7B, 0xF9, 0x7B, 0x42])
                .unwrap()
                .into(),
        ];
        let err = RandomTargetAddressAdStruct::try_new(&addresses);
        assert_eq!(
            err,
            Err(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
        );
    }
}
