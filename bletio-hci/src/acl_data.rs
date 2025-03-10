use bletio_utils::{Buffer, BufferOps};
use num_enum::TryFromPrimitive;

use crate::{ConnectionHandle, Error};

const ACL_DATA_MAX_SIZE: usize = 27;

/// Packet boundary flag of an ACL data packet.
///
/// See [Core Specification 6.0, Vol. 4, Part E, 5.4.2](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-bc4ffa33-44ef-e93c-16c8-14aa99597cfc).
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[num_enum(error_type(name = Error, constructor = Error::InvalidPacketBoundaryFlag))]
#[repr(u8)]
#[non_exhaustive]
pub enum PacketBoundaryFlag {
    /// First non-automatically-flushable packet of a higher layer message (start of a
    /// non-automatically-flushable L2CAP PDU) from Host to Controller.
    FirstNonAutomaticallyFlushablePacket = 0b00,
    /// Continuing fragment of a higher layer message.
    ContinuingFragment = 0b01,
    /// First automatically flushable packet of a higher layer message (start of an
    /// automatically-flushable L2CAP PDU).
    FirstAutomaticallyFlushablePacket = 0b10,
}

/// Broadcast flag of an ACL data packet.
///
/// See [Core Specification 6.0, Vol. 4, Part E, 5.4.2](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-bc4ffa33-44ef-e93c-16c8-14aa99597cfc).
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[num_enum(error_type(name = Error, constructor = Error::InvalidBroadcastFlag))]
#[repr(u8)]
#[non_exhaustive]
pub enum BroadcastFlag {
    /// Point-to-point (ACL-U or LE-U).
    PointToPoint,
    /// BR/EDR broadcast (APB-U).
    BrEdrBroadcast,
}

/// ACL data packet.
///
/// See [Core Specification 6.0, Vol. 4, Part E, 5.4.2](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-bc4ffa33-44ef-e93c-16c8-14aa99597cfc).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AclData {
    handle: ConnectionHandle,
    packet_boundary_flag: PacketBoundaryFlag,
    broadcast_flag: BroadcastFlag,
    data: Buffer<ACL_DATA_MAX_SIZE>,
}

impl AclData {
    pub(crate) fn try_new(
        handle: ConnectionHandle,
        packet_boundary_flag: PacketBoundaryFlag,
        broadcast_flag: BroadcastFlag,
        data: &[u8],
    ) -> Result<Self, Error> {
        let mut s = Self {
            handle,
            packet_boundary_flag,
            broadcast_flag,
            data: Buffer::default(),
        };
        s.data
            .copy_from_slice(data)
            .map_err(|_| Error::DataWillNotFitAclDataPacket)?;
        Ok(s)
    }
}

pub(crate) mod parser {
    use nom::bytes::take;
    use nom::{combinator::map_res, number::complete::le_u16, IResult, Parser};

    use super::*;
    use crate::{packet::Packet, ConnectionHandle};

    fn connection_handle_and_flags(
        input: &[u8],
    ) -> IResult<&[u8], (ConnectionHandle, PacketBoundaryFlag, BroadcastFlag)> {
        map_res(le_u16, |v| {
            let connection_handle = ConnectionHandle::try_new(v & 0xEFF)?;
            let packet_boundary_flag: PacketBoundaryFlag =
                (((v >> 12) & 0b0011) as u8).try_into()?;
            let broadcast_flag: BroadcastFlag = ((v >> 14) as u8).try_into()?;
            Ok::<_, Error>((connection_handle, packet_boundary_flag, broadcast_flag))
        })
        .parse(input)
    }

    fn data_total_length(input: &[u8]) -> IResult<&[u8], u16> {
        le_u16(input)
    }

    pub(crate) fn acl_data(input: &[u8]) -> IResult<&[u8], Packet> {
        let (rest, ((connection_handle, packet_boundary_flag, broadcast_flag), data_total_length)) =
            (connection_handle_and_flags, data_total_length).parse(input)?;
        let (rest, acl_data) = map_res(take(data_total_length), |data| {
            AclData::try_new(
                connection_handle,
                packet_boundary_flag,
                broadcast_flag,
                data,
            )
        })
        .parse(rest)?;
        Ok((rest, Packet::AclData(acl_data)))
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::packet::parser::packet;
    use crate::Packet;

    #[rstest]
    #[case(
        &[2, 0, 32, 16, 0, 12, 0, 5, 0, 18, 1, 8, 0, 24, 0, 40, 0, 0, 0, 42, 0],
        Packet::AclData(AclData::try_new(
            0.try_into().unwrap(),
            PacketBoundaryFlag::FirstAutomaticallyFlushablePacket,
            BroadcastFlag::PointToPoint,
            &[12, 0, 5, 0, 18, 1, 8, 0, 24, 0, 40, 0, 0, 0, 42, 0]
        ).unwrap())
    )]
    fn test_acl_data_parsing_success(#[case] input: &[u8], #[case] expected: Packet) {
        assert_eq!(packet(input), Ok((&[] as &[u8], expected)));
    }
}
