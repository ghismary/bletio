use crate::Error;

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
