use bitflags::bitflags;
use bletio_utils::EncodeToBuffer;

bitflags! {
    /// HCI event mask.
    ///
    /// The values are defined in
    /// [Core Specification 6.0, Vol. 4, Part E, 7.3.1](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-f65458cb-06cf-778a-868e-845078cc8817).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct EventMask: u64 {
        /// This event occurs when a connection is terminated.
        const DISCONNECTION_COMPLETE = 1 << 4;
        /// Indicates that the change of the encryption mode has been completed.
        const ENCRYPTION_CHANGE = 1 << 7;
        /// Indicate the completion of the process obtaining the version information of the remote Controller.
        const READ_REMOTE_VERSION_INFORMATION_COMLETE = 1 << 11;
        /// Notifies the Host that a hardware failure has occurred in the Controller.
        const HARDWARE_ERROR = 1 << 15;
        /// Indicates that the Controller’s data buffers have been overflowed.
        const DATA_BUFFER_OVERFLOW = 1 << 25;
        /// Indicates to the Host that the encryption key was refreshed.
        const ENCRYPTION_KEY_REFRESH_COMPLETE = 1 << 47;
        /// Encapsulates all LE Controller specific events.
        const LE_META = 1 << 61;
    }
}

impl EncodeToBuffer for EventMask {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.encode_le_u64(self.bits())
    }

    fn encoded_size(&self) -> usize {
        8
    }
}

impl Default for EventMask {
    fn default() -> Self {
        Self::from_bits_retain(0x0000_1FFF_FFFF_FFFF)
    }
}

pub(crate) mod parser {
    use nom::{
        combinator::{all_consuming, map},
        number::le_u64,
        IResult, Parser,
    };

    use super::EventMask;

    pub(crate) fn event_mask(input: &[u8]) -> IResult<&[u8], EventMask> {
        all_consuming(map(le_u64(), EventMask::from_bits_retain)).parse(input)
    }
}

#[cfg(test)]
mod test {
    use bletio_utils::{Buffer, BufferOps};
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_eventmask_default() {
        let value = EventMask::default();
        assert_eq!(value.bits(), 0x0000_1FFF_FFFF_FFFF);
    }

    #[rstest]
    #[case(EventMask::HARDWARE_ERROR, &[0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])]
    #[case(EventMask::DATA_BUFFER_OVERFLOW, &[0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00])]
    #[case(EventMask::HARDWARE_ERROR | EventMask::DATA_BUFFER_OVERFLOW, &[0x00, 0x80, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00])]
    fn test_eventmask_encoding(
        #[case] event_mask: EventMask,
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<8>::default();
        assert_eq!(event_mask.encoded_size(), encoded_data.len());
        event_mask.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }

    #[rstest]
    #[case(&[0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], EventMask::HARDWARE_ERROR)]
    #[case(&[0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00], EventMask::DATA_BUFFER_OVERFLOW)]
    #[case(&[0x00, 0x80, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00], EventMask::HARDWARE_ERROR | EventMask::DATA_BUFFER_OVERFLOW)]
    fn test_eventmask_parsing(#[case] input: &[u8], #[case] expected_event_mask: EventMask) {
        let (rest, event_mask) = parser::event_mask(input).unwrap();
        assert!(rest.is_empty());
        assert_eq!(event_mask, expected_event_mask);
    }
}
