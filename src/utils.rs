use crate::Error;

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

pub(crate) fn encode_le_u64(buffer: &mut [u8], data: u64) -> Result<(), Error> {
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
        Ok(())
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
