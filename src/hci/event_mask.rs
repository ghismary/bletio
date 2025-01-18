use bitflags::bitflags;

use crate::utils::encode_le_u64;

bitflags! {
    /// HCI event mask.
    ///
    /// The values are defined in
    /// [Core Specification 6.0, Vol. 4, Part E, 7.3.1](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-f65458cb-06cf-778a-868e-845078cc8817).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) struct EventMask: u64 {
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

impl EventMask {
    pub(crate) fn encode(&self) -> [u8; 8] {
        let mut buffer = [0; 8];
        // INVARIANT: The buffer space is known to be enough.
        encode_le_u64(&mut buffer[..], self.bits()).unwrap();
        buffer
    }
}

impl Default for EventMask {
    fn default() -> Self {
        Self::from_bits_retain(0x0000_1FFF_FFFF_FFFF)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eventmask_default() {
        let value = EventMask::default();
        assert_eq!(value.bits(), 0x0000_1FFF_FFFF_FFFF);
    }

    #[test]
    fn test_eventmask_encode() {
        let value = EventMask::DISCONNECTION_COMPLETE
            | EventMask::ENCRYPTION_CHANGE
            | EventMask::READ_REMOTE_VERSION_INFORMATION_COMLETE
            | EventMask::HARDWARE_ERROR
            | EventMask::DATA_BUFFER_OVERFLOW
            | EventMask::ENCRYPTION_KEY_REFRESH_COMPLETE
            | EventMask::LE_META;
        assert_eq!(
            value.encode().as_slice(),
            &[0x90, 0x88, 0x00, 0x02, 0x00, 0x80, 0x00, 0x20]
        );
    }
}
