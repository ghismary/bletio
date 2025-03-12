use num_enum::TryFromPrimitive;

use crate::{AclData, Command, Error, Event};

/// HCI packet type.
///
/// HCI does not provide the ability to differentiate the five HCI packet types. Therefore, if
/// the HCI packets are sent via a common physical interface, an HCI packet indicator has
/// to be added. This is this packet type.
///
/// See [Core Specification 6.0, Vol. 4, Part A, 2](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/uart-transport-layer.html#UUID-361053ee-862f-c591-00bd-1a941a12f949).
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[num_enum(error_type(name = Error, constructor = Error::InvalidPacketType))]
#[repr(u8)]
#[non_exhaustive]
pub(crate) enum PacketType {
    Command = 0x01,
    AclData = 0x02,
    SynchronousData = 0x03,
    Event = 0x04,
    IsoData = 0x05,
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(clippy::large_enum_variant)]
pub(crate) enum Packet {
    Command(Command),
    AclData(AclData),
    Event(Event),
}

pub(crate) mod parser {
    use nom::{combinator::map_res, number::complete::le_u8, IResult, Parser};

    use crate::{
        acl_data::parser::acl_data, command::parser::command, event::parser::event, Packet,
        PacketType,
    };

    pub(crate) fn parameter_total_length(input: &[u8]) -> IResult<&[u8], u8> {
        le_u8.parse(input)
    }

    pub(crate) fn packet(input: &[u8]) -> IResult<&[u8], Packet> {
        let (input, packet_type) = map_res(le_u8, PacketType::try_from).parse(input)?;
        match packet_type {
            PacketType::Command => command.parse(input),
            PacketType::AclData => acl_data.parse(input),
            PacketType::SynchronousData => {
                // TODO: Handle synchronous data parsing.
                Err(nom::Err::Failure(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Fail,
                )))
            }
            PacketType::Event => event.parse(input),
            PacketType::IsoData => {
                // TODO: Handle ISO data parsing.
                Err(nom::Err::Failure(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Fail,
                )))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use claims::assert_err;
    use rstest::rstest;

    use super::{parser::*, *};

    #[test]
    fn test_valid_packet_type() -> Result<(), Error> {
        let packet_type: PacketType = 4u8.try_into()?;
        assert_eq!(packet_type, PacketType::Event);
        Ok(())
    }

    #[test]
    fn test_invalid_packet_type() {
        let err: Result<PacketType, Error> = 10u8.try_into();
        assert!(matches!(err, Err(Error::InvalidPacketType(_))));
    }

    #[rstest]
    #[case(&[0x03])]
    #[case(&[0x05])]
    fn test_unhandled_packet_type_parsing(#[case] input: &[u8]) {
        assert_err!(packet(input));
    }
}
