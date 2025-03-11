use bletio_utils::Error as UtilsError;
use bletio_utils::{BufferOps, EncodeToBuffer};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{Error, PublicDeviceAddress, RandomAddress};

/// Address type contained in a LE Advertising Report event & LE Connection command.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.7.65.2](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-bacd71f4-fabc-238d-72ee-f9aaaf5cbf22)
/// & [Core Specification 6.0, Vol.4, Part E, 7.8.12](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-18ff009e-8e3a-a32c-160f-23e297c0fc9d).
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[num_enum(error_type(name = Error, constructor = Error::InvalidConnectionPeerAddressType))]
#[repr(u8)]
#[non_exhaustive]
enum ConnectionPeerAddressType {
    /// Public device address.
    PublicDevice = 0x00,
    /// Random device address.
    RandomDevice = 0x01,
    /// Public identity address (corresponds to a resolved RPA).
    PublicIdentity = 0x02,
    /// Random (static) identity address (corresponds to a resolved RPA).
    RandomIdentity = 0x03,
}

impl EncodeToBuffer for ConnectionPeerAddressType {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, UtilsError> {
        buffer.try_push((*self).into())
    }

    fn encoded_size(&self) -> usize {
        size_of::<ConnectionPeerAddressType>()
    }
}

/// Address contained in a LE Advertising Report event & LE Connection command.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.7.65.2](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-bacd71f4-fabc-238d-72ee-f9aaaf5cbf22).
/// & [Core Specification 6.0, Vol.4, Part E, 7.8.12](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-18ff009e-8e3a-a32c-160f-23e297c0fc9d).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConnectionPeerAddress {
    PublicDevice(PublicDeviceAddress),
    RandomDevice(RandomAddress),
    PublicIdentity(PublicDeviceAddress),
    RandomIdentity(RandomAddress),
}

impl ConnectionPeerAddress {
    pub const fn value(&self) -> &[u8; 6] {
        match self {
            Self::PublicDevice(address) => address.value(),
            Self::RandomDevice(address) => address.value(),
            Self::PublicIdentity(address) => address.value(),
            Self::RandomIdentity(address) => address.value(),
        }
    }

    fn r#type(&self) -> ConnectionPeerAddressType {
        match self {
            ConnectionPeerAddress::PublicDevice(_) => ConnectionPeerAddressType::PublicDevice,
            ConnectionPeerAddress::RandomDevice(_) => ConnectionPeerAddressType::RandomDevice,
            ConnectionPeerAddress::PublicIdentity(_) => ConnectionPeerAddressType::PublicIdentity,
            ConnectionPeerAddress::RandomIdentity(_) => ConnectionPeerAddressType::RandomIdentity,
        }
    }
}

impl Default for ConnectionPeerAddress {
    fn default() -> Self {
        Self::PublicDevice(Default::default())
    }
}

impl EncodeToBuffer for ConnectionPeerAddress {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, UtilsError> {
        self.r#type().encode(buffer)?;
        match self {
            ConnectionPeerAddress::PublicDevice(address) => address.encode(buffer)?,
            ConnectionPeerAddress::RandomDevice(address) => address.encode(buffer)?,
            ConnectionPeerAddress::PublicIdentity(address) => address.encode(buffer)?,
            ConnectionPeerAddress::RandomIdentity(address) => address.encode(buffer)?,
        };
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        7
    }
}

pub(crate) mod parser {
    use nom::{combinator::map_res, number::le_u8, IResult, Parser};

    use super::*;
    use crate::common::device_address::parser::address;

    fn connection_peer_address_type(input: &[u8]) -> IResult<&[u8], ConnectionPeerAddressType> {
        map_res(le_u8(), TryInto::try_into).parse(input)
    }

    pub(crate) fn connection_peer_address(input: &[u8]) -> IResult<&[u8], ConnectionPeerAddress> {
        let (rest, address_type) = connection_peer_address_type(input)?;
        let (rest, address) = address(rest)?;
        Ok((
            rest,
            match address_type {
                ConnectionPeerAddressType::PublicDevice => {
                    ConnectionPeerAddress::PublicDevice(address.into())
                }
                ConnectionPeerAddressType::RandomDevice => {
                    ConnectionPeerAddress::RandomDevice(address.try_into().map_err(|_| {
                        nom::Err::Failure(nom::error::Error::new(
                            input,
                            nom::error::ErrorKind::Fail,
                        ))
                    })?)
                }
                ConnectionPeerAddressType::PublicIdentity => {
                    ConnectionPeerAddress::PublicIdentity(address.into())
                }
                ConnectionPeerAddressType::RandomIdentity => {
                    ConnectionPeerAddress::RandomIdentity(address.try_into().map_err(|_| {
                        nom::Err::Failure(nom::error::Error::new(
                            input,
                            nom::error::ErrorKind::Fail,
                        ))
                    })?)
                }
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use bletio_utils::Buffer;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        ConnectionPeerAddress::PublicDevice(PublicDeviceAddress::new([0x38, 0x5E, 0x43, 0xCA, 0x4C, 0x40])),
        ConnectionPeerAddressType::PublicDevice,
        &[0x00, 0x38, 0x5E, 0x43, 0xCA, 0x4C, 0x40]
    )]
    #[case(
        ConnectionPeerAddress::RandomDevice([0xC6, 0x37, 0x7B, 0xF9, 0x7B, 0xC2].try_into().unwrap()),
        ConnectionPeerAddressType::RandomDevice,
        &[0x01, 0xC6, 0x37, 0x7B, 0xF9, 0x7B, 0xC2]
    )]
    #[case(
        ConnectionPeerAddress::PublicIdentity(PublicDeviceAddress::new([0x38, 0x5E, 0x43, 0xCA, 0x4C, 0x40])),
        ConnectionPeerAddressType::PublicIdentity,
        &[0x02, 0x38, 0x5E, 0x43, 0xCA, 0x4C, 0x40]
    )]
    #[case(
        ConnectionPeerAddress::RandomIdentity([0xC6, 0x37, 0x7B, 0xF9, 0x7B, 0xC2].try_into().unwrap()),
        ConnectionPeerAddressType::RandomIdentity,
        &[0x03, 0xC6, 0x37, 0x7B, 0xF9, 0x7B, 0xC2]
    )]
    fn test_connection_peer_address(
        #[case] address: ConnectionPeerAddress,
        #[case] expected_type: ConnectionPeerAddressType,
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        assert_eq!(expected_type.encoded_size(), 1);
        assert_eq!(address.r#type(), expected_type);
        let mut buffer = Buffer::<7>::default();
        assert_eq!(address.encoded_size(), encoded_data.len());
        address.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }
}
