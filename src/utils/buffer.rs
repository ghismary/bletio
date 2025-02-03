use crate::utils::{encode_le_u128, encode_le_u16, encode_le_u32, encode_le_u64, UtilsError};

pub(crate) trait BufferOps {
    fn data(&self) -> &[u8];
    fn is_full(&self) -> bool;
    fn len(&self) -> usize;
    fn remaining_len(&self) -> usize;
    fn copy_from_slice(&mut self, data: &[u8]) -> Result<usize, UtilsError>;
    fn encode_le_u16(&mut self, data: u16) -> Result<usize, UtilsError>;
    fn encode_le_u32(&mut self, data: u32) -> Result<usize, UtilsError>;
    fn encode_le_u64(&mut self, data: u64) -> Result<usize, UtilsError>;
    fn encode_le_u128(&mut self, data: u128) -> Result<usize, UtilsError>;
    fn try_push(&mut self, data: u8) -> Result<usize, UtilsError>;
}

pub(crate) trait EncodeToBuffer {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, UtilsError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Buffer<const CAP: usize> {
    pub(crate) data: [u8; CAP],
    pub(crate) offset: usize,
}

impl<const CAP: usize> BufferOps for Buffer<CAP> {
    fn data(&self) -> &[u8] {
        &self.data[..self.offset]
    }

    fn is_full(&self) -> bool {
        self.offset == CAP
    }

    fn len(&self) -> usize {
        self.offset
    }

    fn remaining_len(&self) -> usize {
        CAP - self.len()
    }

    fn copy_from_slice(&mut self, data: &[u8]) -> Result<usize, UtilsError> {
        let data_size = data.len();
        if self.remaining_len() < data_size {
            Err(UtilsError::BufferTooSmall)
        } else {
            self.offset += data_size;
            self.data[self.offset - data_size..self.offset].copy_from_slice(data);
            Ok(data_size)
        }
    }

    fn encode_le_u16(&mut self, data: u16) -> Result<usize, UtilsError> {
        if self.remaining_len() < 2 {
            Err(UtilsError::BufferTooSmall)
        } else {
            self.offset += 2;
            encode_le_u16(&mut self.data[self.offset - 2..self.offset], data)?;
            Ok(2)
        }
    }

    fn encode_le_u32(&mut self, data: u32) -> Result<usize, UtilsError> {
        if self.remaining_len() < 4 {
            Err(UtilsError::BufferTooSmall)
        } else {
            self.offset += 4;
            encode_le_u32(&mut self.data[self.offset - 4..self.offset], data)?;
            Ok(4)
        }
    }

    fn encode_le_u64(&mut self, data: u64) -> Result<usize, UtilsError> {
        if self.remaining_len() < 8 {
            Err(UtilsError::BufferTooSmall)
        } else {
            self.offset += 8;
            encode_le_u64(&mut self.data[self.offset - 8..self.offset], data)?;
            Ok(8)
        }
    }

    fn encode_le_u128(&mut self, data: u128) -> Result<usize, UtilsError> {
        if self.remaining_len() < 16 {
            Err(UtilsError::BufferTooSmall)
        } else {
            self.offset += 16;
            encode_le_u128(&mut self.data[self.offset - 16..self.offset], data)?;
            Ok(16)
        }
    }

    fn try_push(&mut self, data: u8) -> Result<usize, UtilsError> {
        if self.is_full() {
            Err(UtilsError::BufferTooSmall)
        } else {
            self.data[self.offset] = data;
            self.offset += 1;
            Ok(1)
        }
    }
}

impl<const CAP: usize> Default for Buffer<CAP> {
    fn default() -> Self {
        Self {
            data: [0; CAP],
            offset: 0,
        }
    }
}

impl<const CAP: usize> TryFrom<&[u8]> for Buffer<CAP> {
    type Error = UtilsError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() > CAP {
            Err(UtilsError::BufferTooSmall)
        } else {
            let mut s = Self::default();
            s.copy_from_slice(value)?;
            Ok(s)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_buffer_default() {
        let buffer: Buffer<8> = Buffer::default();
        assert!(!buffer.is_full());
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.remaining_len(), 8);
        assert_eq!(buffer.data(), &[]);
    }

    #[test]
    fn test_buffer_copy_from_slice_success() -> Result<(), UtilsError> {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let mut buffer: Buffer<8> = Buffer::default();
        buffer.copy_from_slice(&data)?;
        assert!(buffer.is_full());
        assert_eq!(buffer.len(), 8);
        assert_eq!(buffer.remaining_len(), 0);
        assert_eq!(buffer.data(), data);

        let data = [0x01, 0x02, 0x03, 0x04, 0x05];
        let mut buffer: Buffer<8> = Buffer::default();
        buffer.copy_from_slice(&data)?;
        assert!(!buffer.is_full());
        assert_eq!(buffer.len(), 5);
        assert_eq!(buffer.remaining_len(), 3);
        assert_eq!(buffer.data(), data);

        Ok(())
    }

    #[test]
    fn test_buffer_copy_from_slice_failure() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05];
        let mut buffer: Buffer<4> = Buffer::default();
        let err = buffer.copy_from_slice(&data).expect_err("Buffer too small");
        assert!(matches!(err, UtilsError::BufferTooSmall));
    }

    #[test]
    fn test_buffer_encode_le_u16_success() -> Result<(), UtilsError> {
        let data = 0x3456;
        let mut buffer: Buffer<8> = Buffer::default();
        buffer.encode_le_u16(data)?;
        assert!(!buffer.is_full());
        assert_eq!(buffer.len(), 2);
        assert_eq!(buffer.remaining_len(), 6);
        assert_eq!(buffer.data(), &[0x56, 0x34]);
        Ok(())
    }

    #[test]
    fn test_buffer_encode_le_u16_failure() {
        let data = 0x3456;
        let mut buffer: Buffer<1> = Buffer::default();
        let err = buffer.encode_le_u16(data).expect_err("Buffer too small");
        assert!(matches!(err, UtilsError::BufferTooSmall));
    }

    #[test]
    fn test_buffer_encode_le_u32_success() -> Result<(), UtilsError> {
        let data = 0x34567890;
        let mut buffer: Buffer<8> = Buffer::default();
        buffer.encode_le_u32(data)?;
        assert!(!buffer.is_full());
        assert_eq!(buffer.len(), 4);
        assert_eq!(buffer.remaining_len(), 4);
        assert_eq!(buffer.data(), &[0x90, 0x78, 0x56, 0x34]);
        Ok(())
    }

    #[test]
    fn test_buffer_encode_le_u32_failure() {
        let data = 0x34567890;
        let mut buffer: Buffer<3> = Buffer::default();
        let err = buffer.encode_le_u32(data).expect_err("Buffer too small");
        assert!(matches!(err, UtilsError::BufferTooSmall));
    }

    #[test]
    fn test_buffer_encode_le_u64_success() -> Result<(), UtilsError> {
        let data = 0x0102030405060708;
        let mut buffer: Buffer<16> = Buffer::default();
        buffer.encode_le_u64(data)?;
        assert!(!buffer.is_full());
        assert_eq!(buffer.len(), 8);
        assert_eq!(buffer.remaining_len(), 8);
        assert_eq!(
            buffer.data(),
            &[0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01]
        );
        Ok(())
    }

    #[test]
    fn test_buffer_encode_le_u64_failure() {
        let data = 0x0102030405060708;
        let mut buffer: Buffer<5> = Buffer::default();
        let err = buffer.encode_le_u64(data).expect_err("Buffer too small");
        assert!(matches!(err, UtilsError::BufferTooSmall));
    }

    #[test]
    fn test_buffer_encode_le_u128_success() -> Result<(), UtilsError> {
        let data = 0x0102030405060708090A0B0C0D0E0F10;
        let mut buffer: Buffer<32> = Buffer::default();
        buffer.encode_le_u128(data)?;
        assert!(!buffer.is_full());
        assert_eq!(buffer.len(), 16);
        assert_eq!(buffer.remaining_len(), 16);
        assert_eq!(
            buffer.data(),
            &[
                0x10, 0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03,
                0x02, 0x01
            ]
        );
        Ok(())
    }

    #[test]
    fn test_buffer_encode_le_u128_failure() {
        let data = 0x0102030405060708090A0B0C0D0E0F10;
        let mut buffer: Buffer<12> = Buffer::default();
        let err = buffer.encode_le_u128(data).expect_err("Buffer too small");
        assert!(matches!(err, UtilsError::BufferTooSmall));
    }

    #[test]
    fn test_buffer_try_push() -> Result<(), UtilsError> {
        let mut buffer: Buffer<2> = Buffer::default();
        buffer.try_push(0x28)?;
        assert!(!buffer.is_full());
        assert_eq!(buffer.len(), 1);
        assert_eq!(buffer.remaining_len(), 1);
        assert_eq!(buffer.data(), &[0x28]);

        buffer.try_push(0x73)?;
        assert!(buffer.is_full());
        assert_eq!(buffer.len(), 2);
        assert_eq!(buffer.remaining_len(), 0);
        assert_eq!(buffer.data(), &[0x28, 0x73]);

        let err = buffer.try_push(0x50).expect_err("Buffer too small");
        assert!(matches!(err, UtilsError::BufferTooSmall));

        Ok(())
    }

    #[test]
    fn test_buffer_try_from_success() -> Result<(), UtilsError> {
        let data = [0x00, 0x01, 0x02, 0x03];
        let buffer: Buffer<64> = Buffer::try_from(data.as_slice())?;
        assert!(!buffer.is_full());
        assert_eq!(buffer.len(), 4);
        assert_eq!(buffer.remaining_len(), 60);
        assert_eq!(buffer.data(), data.as_slice());
        Ok(())
    }

    #[test]
    fn test_buffer_try_from_failure() {
        let data = [0x00, 0x01, 0x02, 0x03];
        let err: Result<Buffer<2>, UtilsError> = Buffer::try_from(data.as_slice());
        assert!(matches!(err, Err(UtilsError::BufferTooSmall)));
    }
}
