use bitflags::bitflags;

#[derive(Debug, Clone, Copy)]
pub struct Flags(u8);

bitflags! {
    impl Flags: u8 {
        const LE_LIMITED_DISCOVERABLE_MODE = 1 << 0;
        const LE_GENERAL_DISCOVERABLE_MODE = 1 << 1;
        const BREDR_NOT_SUPPORTED = 1 << 2;
        const SIMULTANEOUS_LE_AND_BREDR_TO_SAME_DEVICE_CAPABLE_CONTROLLER = 1 << 3;
    }
}

impl Default for Flags {
    fn default() -> Self {
        Self::BREDR_NOT_SUPPORTED
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_flags() {
        let mut flags = Flags::default();
        assert_eq!(flags.bits(), Flags::BREDR_NOT_SUPPORTED.bits());
        flags |= Flags::LE_GENERAL_DISCOVERABLE_MODE;
        assert_eq!(flags.bits(), 0x06);
    }
}
