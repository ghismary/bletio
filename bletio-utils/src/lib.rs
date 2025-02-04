#![no_std]

mod bit_flags_array;
mod buffer;

pub use bit_flags_array::BitFlagsArray;
pub use buffer::{Buffer, BufferOps, EncodeToBuffer};

#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    #[error("Buffer too small")]
    BufferTooSmall,
}

pub(crate) fn encode_le_u128(buffer: &mut [u8], data: u128) -> Result<usize, Error> {
    if buffer.len() < 16 {
        Err(Error::BufferTooSmall)
    } else {
        buffer[0] = (data & 0xFF) as u8;
        buffer[1] = ((data >> 8) & 0xFF) as u8;
        buffer[2] = ((data >> 16) & 0xFF) as u8;
        buffer[3] = ((data >> 24) & 0xFF) as u8;
        buffer[4] = ((data >> 32) & 0xFF) as u8;
        buffer[5] = ((data >> 40) & 0xFF) as u8;
        buffer[6] = ((data >> 48) & 0xFF) as u8;
        buffer[7] = ((data >> 56) & 0xFF) as u8;
        buffer[8] = ((data >> 64) & 0xFF) as u8;
        buffer[9] = ((data >> 72) & 0xFF) as u8;
        buffer[10] = ((data >> 80) & 0xFF) as u8;
        buffer[11] = ((data >> 88) & 0xFF) as u8;
        buffer[12] = ((data >> 96) & 0xFF) as u8;
        buffer[13] = ((data >> 104) & 0xFF) as u8;
        buffer[14] = ((data >> 112) & 0xFF) as u8;
        buffer[15] = ((data >> 120) & 0xFF) as u8;
        Ok(16)
    }
}

pub(crate) fn encode_le_u64(buffer: &mut [u8], data: u64) -> Result<usize, Error> {
    if buffer.len() < 8 {
        Err(Error::BufferTooSmall)
    } else {
        buffer[0] = (data & 0xFF) as u8;
        buffer[1] = ((data >> 8) & 0xFF) as u8;
        buffer[2] = ((data >> 16) & 0xFF) as u8;
        buffer[3] = ((data >> 24) & 0xFF) as u8;
        buffer[4] = ((data >> 32) & 0xFF) as u8;
        buffer[5] = ((data >> 40) & 0xFF) as u8;
        buffer[6] = ((data >> 48) & 0xFF) as u8;
        buffer[7] = ((data >> 56) & 0xFF) as u8;
        Ok(8)
    }
}

pub(crate) fn encode_le_u32(buffer: &mut [u8], data: u32) -> Result<usize, Error> {
    if buffer.len() < 4 {
        Err(Error::BufferTooSmall)
    } else {
        buffer[0] = (data & 0xFF) as u8;
        buffer[1] = ((data >> 8) & 0xFF) as u8;
        buffer[2] = ((data >> 16) & 0xFF) as u8;
        buffer[3] = ((data >> 24) & 0xFF) as u8;
        Ok(4)
    }
}

pub(crate) fn encode_le_u16(buffer: &mut [u8], data: u16) -> Result<usize, Error> {
    if buffer.len() < 2 {
        Err(Error::BufferTooSmall)
    } else {
        buffer[0] = (data & 0xFF) as u8;
        buffer[1] = ((data >> 8) & 0xFF) as u8;
        Ok(2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode_le_u128_success() {
        let mut buffer = [0; 16];
        assert!(encode_le_u128(&mut buffer[..], 21345817372864405881847059188222722561).is_ok());
        assert_eq!(
            [
                0x01u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F, 0x10
            ],
            buffer,
        );
    }

    #[test]
    fn test_encode_le_u128_failure() {
        let mut buffer = [0; 5];
        let err = encode_le_u128(&mut buffer[..], 21345817372864405881847059188222722561)
            .expect_err("Buffer too small");
        assert!(matches!(err, Error::BufferTooSmall));
    }

    #[test]
    fn test_encode_le_u64_success() {
        let mut buffer = [0; 8];
        assert!(encode_le_u64(&mut buffer[..], 578437695752307201).is_ok());
        assert_eq!(
            [0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8],
            buffer,
        );
    }

    #[test]
    fn test_encode_le_u64_failure() {
        let mut buffer = [0; 4];
        let err = encode_le_u64(&mut buffer[..], 578437695752307201).expect_err("Buffer too small");
        assert!(matches!(err, Error::BufferTooSmall));
    }

    #[test]
    fn test_encode_le_u32_success() {
        let mut buffer = [0; 4];
        assert!(encode_le_u32(&mut buffer[..], 67305985).is_ok());
        assert_eq!([0x01u8, 0x02u8, 0x03u8, 0x04u8], buffer,);
    }

    #[test]
    fn test_encode_le_u32_failure() {
        let mut buffer = [0; 3];
        let err = encode_le_u32(&mut buffer[..], 67305985).expect_err("Buffer too small");
        assert!(matches!(err, Error::BufferTooSmall));
    }

    #[test]
    fn test_encode_le_u16_success() {
        let mut buffer = [0; 2];
        assert!(encode_le_u16(&mut buffer[..], 513).is_ok());
        assert_eq!([0x01u8, 0x02u8], buffer);
    }

    #[test]
    fn test_encode_le_u16_failure() {
        let mut buffer = [0; 1];
        let err = encode_le_u16(&mut buffer[..], 513).expect_err("Buffer too small");
        assert!(matches!(err, Error::BufferTooSmall));
    }
}
