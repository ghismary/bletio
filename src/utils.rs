#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UtilsError {
    #[error("Buffer too small")]
    BufferTooSmall,
}

#[derive(Debug, Clone)]
pub(crate) struct Buffer<const CAP: usize> {
    pub(crate) data: [u8; CAP],
    pub(crate) offset: usize,
}

impl<const CAP: usize> Buffer<CAP> {
    pub(crate) fn data(&self) -> &[u8] {
        &self.data[..self.offset]
    }

    fn is_full(&self) -> bool {
        self.offset == CAP
    }

    fn len(&self) -> usize {
        self.offset
    }

    pub(crate) fn remaining_len(&self) -> usize {
        CAP - self.len()
    }

    pub(crate) fn copy_from_slice(&mut self, data: &[u8]) -> Result<(), UtilsError> {
        let data_size = data.len();
        if self.remaining_len() < data_size {
            Err(UtilsError::BufferTooSmall)
        } else {
            self.offset += data_size;
            self.data[self.offset - data_size..self.offset].copy_from_slice(data);
            Ok(())
        }
    }

    pub(crate) fn encode_le_u16(&mut self, data: u16) -> Result<(), UtilsError> {
        if self.remaining_len() < 2 {
            Err(UtilsError::BufferTooSmall)
        } else {
            self.offset += 2;
            encode_le_u16(&mut self.data[self.offset - 2..self.offset], data)?;
            Ok(())
        }
    }

    pub(crate) fn encode_le_u32(&mut self, data: u32) -> Result<(), UtilsError> {
        if self.remaining_len() < 4 {
            Err(UtilsError::BufferTooSmall)
        } else {
            self.offset += 4;
            encode_le_u32(&mut self.data[self.offset - 4..self.offset], data)?;
            Ok(())
        }
    }

    pub(crate) fn encode_le_u128(&mut self, data: u128) -> Result<(), UtilsError> {
        if self.remaining_len() < 16 {
            Err(UtilsError::BufferTooSmall)
        } else {
            self.offset += 16;
            encode_le_u128(&mut self.data[self.offset - 16..self.offset], data)?;
            Ok(())
        }
    }

    pub(crate) fn try_push(&mut self, data: u8) -> Result<(), UtilsError> {
        if self.is_full() {
            Err(UtilsError::BufferTooSmall)
        } else {
            self.data[self.offset] = data;
            self.offset += 1;
            Ok(())
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

pub(crate) fn decode_le_u64(buffer: [u8; 8]) -> u64 {
    ((buffer[7] as u64) << 56)
        | ((buffer[6] as u64) << 48)
        | ((buffer[5] as u64) << 40)
        | ((buffer[4] as u64) << 32)
        | ((buffer[3] as u64) << 24)
        | ((buffer[2] as u64) << 16)
        | ((buffer[1] as u64) << 8)
        | (buffer[0] as u64)
}

pub(crate) fn decode_le_u16(buffer: [u8; 2]) -> u16 {
    ((buffer[1] as u16) << 8) | (buffer[0] as u16)
}

pub(crate) fn encode_le_u128(buffer: &mut [u8], data: u128) -> Result<usize, UtilsError> {
    if buffer.len() < 16 {
        Err(UtilsError::BufferTooSmall)
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

pub(crate) fn encode_le_u64(buffer: &mut [u8], data: u64) -> Result<usize, UtilsError> {
    if buffer.len() < 8 {
        Err(UtilsError::BufferTooSmall)
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

pub(crate) fn encode_le_u32(buffer: &mut [u8], data: u32) -> Result<usize, UtilsError> {
    if buffer.len() < 4 {
        Err(UtilsError::BufferTooSmall)
    } else {
        buffer[0] = (data & 0xFF) as u8;
        buffer[1] = ((data >> 8) & 0xFF) as u8;
        buffer[2] = ((data >> 16) & 0xFF) as u8;
        buffer[3] = ((data >> 24) & 0xFF) as u8;
        Ok(4)
    }
}

pub(crate) fn encode_le_u16(buffer: &mut [u8], data: u16) -> Result<usize, UtilsError> {
    if buffer.len() < 2 {
        Err(UtilsError::BufferTooSmall)
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
    fn test_decode_le_u16() {
        let buffer = [0x01u8, 0x0Cu8];
        assert_eq!(0x0C01, decode_le_u16(buffer));
    }

    #[test]
    fn test_decode_le_u64() {
        let buffer = [
            0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8,
        ];
        assert_eq!(0x0807060504030201, decode_le_u64(buffer));
    }

    #[test]
    fn test_encode_le_u64() {
        let mut buffer = [0; 8];
        assert!(encode_le_u64(&mut buffer[..], 578437695752307201).is_ok());
        assert_eq!(
            [0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8],
            buffer,
        );
    }
}
