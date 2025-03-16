use crate::Error;
use bletio_utils::{BufferOps, EncodeToBuffer};

/// Connection handle to be used for transmitting a data packet over a Controller.
///
/// Range: 0x000 to 0xEFF
///
/// See [Core Specification 6.0, Vol. 4, Part E, 5.4.2](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-bc4ffa33-44ef-e93c-16c8-14aa99597cfc).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
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

    pub fn value(&self) -> u16 {
        self.value
    }
}

impl EncodeToBuffer for ConnectionHandle {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, bletio_utils::Error> {
        buffer.encode_le_u16(self.value)?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        size_of::<u16>()
    }
}

pub(crate) mod parser {
    use nom::{combinator::map_res, number::complete::le_u16, IResult, Parser};

    use super::*;

    pub(crate) fn connection_handle(input: &[u8]) -> IResult<&[u8], ConnectionHandle> {
        map_res(le_u16, ConnectionHandle::try_new).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bletio_utils::Buffer;
    use rstest::rstest;

    #[rstest]
    #[case(0x0000, &[0x00, 0x00])]
    #[case(0x0010, &[0x10, 0x00])]
    #[case(0x0EFF, &[0xFF, 0x0E])]
    fn test_connection_handle_success(#[case] input: u16, #[case] encoded_data: &[u8]) {
        let handle = ConnectionHandle::try_new(input).unwrap();
        assert_eq!(handle.value, input);
        let mut buffer = Buffer::<2>::default();
        assert_eq!(handle.encoded_size(), encoded_data.len());
        handle.encode(&mut buffer).unwrap();
        assert_eq!(buffer.data(), encoded_data);
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
