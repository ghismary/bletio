use crate::Error;

/// Connection handle to be used for transmitting a data packet over a Controller.
///
/// Range: 0x000 to 0xEFF
///
/// See [Core Specification 6.0, Vol. 4, Part E, 5.4.2](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-bc4ffa33-44ef-e93c-16c8-14aa99597cfc).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ConnectionHandle {
    value: u16,
}

impl ConnectionHandle {
    pub(crate) const fn try_new(handle: u16) -> Result<Self, Error> {
        if handle <= 0x0EFF {
            Ok(Self { value: handle })
        } else {
            Err(Error::InvalidConnectionHandle(handle))
        }
    }
}

impl TryFrom<u16> for ConnectionHandle {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Error> {
        Self::try_new(value)
    }
}

pub(crate) mod parser {
    use nom::{combinator::map_res, number::complete::le_u16, IResult, Parser};

    use super::*;

    pub(crate) fn connection_handle(input: &[u8]) -> IResult<&[u8], ConnectionHandle> {
        map_res(le_u16, TryFrom::try_from).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(0x0000)]
    #[case(0x0010)]
    #[case(0x0EFF)]
    fn test_connection_handle_success(#[case] input: u16) {
        let handle: ConnectionHandle = input.try_into().unwrap();
        assert_eq!(handle.value, input);
    }

    #[rstest]
    #[case(0x0F00)]
    #[case(0x1000)]
    #[case(0xFFFF)]
    fn test_connection_handle_failure(#[case] input: u16) {
        let err = ConnectionHandle::try_new(input);
        assert_eq!(err, Err(Error::InvalidConnectionHandle(input)));
    }
}
