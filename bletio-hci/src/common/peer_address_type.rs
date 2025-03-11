use bletio_utils::Error as UtilsError;
use bletio_utils::{BufferOps, EncodeToBuffer};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{DeviceAddress, Error};

/// Peer address type.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.5](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3142c154-1bdd-37b2-cc6e-006aa755f5f7).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::InvalidPeerAddressType))]
#[repr(u8)]
#[non_exhaustive]
pub(crate) enum PeerAddressType {
    /// Public Device Address (default) or Public Identity Address.
    #[default]
    Public = 0x00,
    /// Random Device Address or Random (static) Identity Address.
    Random = 0x01,
}

impl From<&DeviceAddress> for PeerAddressType {
    fn from(value: &DeviceAddress) -> Self {
        match value {
            DeviceAddress::Public(_) => Self::Public,
            DeviceAddress::Random(_) => Self::Random,
        }
    }
}

impl EncodeToBuffer for PeerAddressType {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, UtilsError> {
        buffer.try_push((*self).into())
    }

    fn encoded_size(&self) -> usize {
        size_of::<PeerAddressType>()
    }
}

pub(crate) mod parser {
    use nom::{bytes::take, combinator::map_res, number::le_u8, IResult, Parser};

    use crate::common::peer_address_type::PeerAddressType;
    use crate::{DeviceAddress, Error, PublicDeviceAddress, RandomAddress};

    fn peer_address_type(input: &[u8]) -> IResult<&[u8], PeerAddressType> {
        map_res(le_u8(), TryInto::try_into).parse(input)
    }

    pub(crate) fn peer_address(input: &[u8]) -> IResult<&[u8], DeviceAddress> {
        map_res(
            (
                peer_address_type,
                map_res(take(6u8), TryInto::<[u8; 6]>::try_into),
            ),
            |(peer_address_type, peer_address)| {
                Ok::<DeviceAddress, Error>(match peer_address_type {
                    PeerAddressType::Public => {
                        let address: PublicDeviceAddress = peer_address.into();
                        address.into()
                    }
                    PeerAddressType::Random => {
                        let address: RandomAddress = peer_address.try_into()?;
                        address.into()
                    }
                })
            },
        )
        .parse(input)
    }
}
