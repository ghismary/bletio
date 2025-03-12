use bletio_utils::EncodeToBuffer;
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::Error;

/// Own address type.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.8.5](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3142c154-1bdd-37b2-cc6e-006aa755f5f7).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[num_enum(error_type(name = Error, constructor = Error::InvalidOwnAddressType))]
#[repr(u8)]
#[non_exhaustive]
pub enum OwnAddressType {
    /// Public Device Address (default).
    #[default]
    PublicDeviceAddress = 0x00,
    /// Random Device Address.
    RandomDeviceAddress = 0x01,
    /// Controller generates Resolvable Private Address based on the local IRK from the resolving list.
    /// If the resolving list contains no matching entry, use the public address.
    GeneratedResolvablePrivateAddressFallbackPublic = 0x02,
    /// Controller generates Resolvable Private Address based on the local IRK from the resolving list.
    /// If the resolving list contains no matching entry, use the random address from `LE_Set_Random_Address`.
    GeneratedResolvablePrivateAddressFallbackRandom = 0x03,
}

impl EncodeToBuffer for OwnAddressType {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.try_push((*self).into())
    }

    fn encoded_size(&self) -> usize {
        size_of::<OwnAddressType>()
    }
}

pub(crate) mod parser {
    use nom::{combinator::map_res, number::complete::le_u8, IResult, Parser};

    use super::*;

    pub(crate) fn own_address_type(input: &[u8]) -> IResult<&[u8], OwnAddressType> {
        map_res(le_u8, TryInto::try_into).parse(input)
    }
}
