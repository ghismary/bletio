use bitflags::bitflags;

use crate::advertising::ad_struct::common_data_types::CommonDataType;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub struct FlagsAdStruct(u8);

bitflags! {
    impl FlagsAdStruct: u8 {
        const LE_LIMITED_DISCOVERABLE_MODE = 1 << 0;
        const LE_GENERAL_DISCOVERABLE_MODE = 1 << 1;
        const BREDR_NOT_SUPPORTED = 1 << 2;
        const SIMULTANEOUS_LE_AND_BREDR_TO_SAME_DEVICE_CAPABLE_CONTROLLER = 1 << 3;
    }
}

impl FlagsAdStruct {
    pub(crate) fn size(&self) -> usize {
        3
    }

    pub(crate) fn encode(&self, buffer: &mut [u8]) -> Result<usize, Error> {
        buffer[0] = (self.size() - 1) as u8;
        buffer[1] = CommonDataType::Flags as u8;
        buffer[2] = self.bits();
        Ok(self.size())
    }
}

impl Default for FlagsAdStruct {
    fn default() -> Self {
        Self::BREDR_NOT_SUPPORTED
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_flags() {
        let mut flags = FlagsAdStruct::default();
        assert_eq!(flags.bits(), FlagsAdStruct::BREDR_NOT_SUPPORTED.bits());
        flags |= FlagsAdStruct::LE_GENERAL_DISCOVERABLE_MODE;
        assert_eq!(flags.bits(), 0x06);
    }
}
