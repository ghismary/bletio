use crate::Error;
use bletio_utils::{BufferOps, EncodeToBuffer};
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// HCI disconnection reason as defined in
/// [Core Specification 6.0, Vol.4, Part E, 7.1.6](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-a211724f-97dc-d1f7-2c28-240854fb271c).
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[num_enum(error_type(name = Error, constructor = Error::InvalidReason))]
#[repr(u8)]
#[non_exhaustive]
pub enum Reason {
    /// Pairing or authentication failed due to incorrect results in the pairing or authentication procedure.
    /// This could be due to an incorrect PIN or Link Key.
    AuthenticationFailure = 0x05,
    /// The user on the remote device either terminated the connection or stopped broadcasting packets.
    RemoteUserTerminatedConnection = 0x13,
    /// The remote device terminated the connection because of low resources.
    RemoteDeviceTerminatedConnectionDueToLowResources = 0x14,
    /// The remote device terminated the connection because the device is about to power off.
    RemoteDeviceTerminatedConnectionDueToPowerOff = 0x15,
    /// The remote device does not support the feature associated with the issued command, LMP PDU, or Link Layer Control PDU.
    UnsupportedRemoteFeatureUnsupportedLmpFeature = 0x1A,
    /// It was not possible to pair as a unit key was requested, and it is not supported.
    PairingWithUnitKeyNotSupported = 0x29,
    /// The remote device either terminated the connection or rejected a request because of one or more
    /// unacceptable connection parameters.
    UnacceptableConnectionParameters = 0x3B,
}

impl EncodeToBuffer for Reason {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, bletio_utils::Error> {
        buffer.try_push((*self).into())?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        size_of::<Reason>()
    }
}

pub(crate) mod parser {
    use nom::{combinator::map_res, number::complete::le_u8, IResult, Parser};

    use super::*;

    pub(crate) fn reason(input: &[u8]) -> IResult<&[u8], Reason> {
        map_res(le_u8, TryFrom::try_from).parse(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bletio_utils::Buffer;
    use rstest::rstest;

    #[rstest]
    #[case(0x05, Reason::AuthenticationFailure, &[0x05])]
    #[case(0x14, Reason::RemoteDeviceTerminatedConnectionDueToLowResources, &[0x14])]
    #[case(0x3B, Reason::UnacceptableConnectionParameters, &[0x3B])]
    fn test_reason_success(
        #[case] input: u8,
        #[case] expected_reason: Reason,
        #[case] encoded_data: &[u8],
    ) -> Result<(), Error> {
        let reason: Reason = input.try_into()?;
        assert_eq!(reason, expected_reason);
        let mut buffer = Buffer::<1>::default();
        assert_eq!(reason.encoded_size(), encoded_data.len());
        reason.encode(&mut buffer).unwrap();
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }

    #[rstest]
    #[case(0x00)]
    #[case(0x12)]
    #[case(0x3A)]
    fn test_reason_failure(#[case] input: u8) {
        let err = Reason::try_from(input);
        assert_eq!(err, Err(Error::InvalidReason(input)));
    }
}
