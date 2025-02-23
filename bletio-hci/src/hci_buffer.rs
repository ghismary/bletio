use bletio_utils::{Buffer, BufferOps, Error as UtilsError};

use crate::{HciDriver, HciDriverError};

const HCI_MAX_READ_BUFFER_SIZE: usize = 259;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct HciBuffer {
    buffer: Buffer<HCI_MAX_READ_BUFFER_SIZE>,
}

impl HciBuffer {
    pub(crate) fn clear(&mut self) {
        self.buffer.clear();
    }

    pub(crate) fn data(&self) -> &[u8] {
        self.buffer.data()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub(crate) fn len(&self) -> usize {
        self.buffer.len()
    }

    pub(crate) async fn read<H: HciDriver>(
        &mut self,
        driver: &mut H,
    ) -> Result<usize, HciDriverError> {
        let offset = self.len();
        let read_len = driver.read(&mut self.buffer.data[offset..]).await?;
        self.buffer.offset += read_len;
        Ok(read_len)
    }
}

impl TryFrom<&[u8]> for HciBuffer {
    type Error = UtilsError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self {
            buffer: value.try_into()?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::*;

    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_hci_buffer() -> Result<(), bletio_utils::Error> {
        let initial_data: &[u8] = &[];
        let read_data: &[u8] = &[0x01, 0x02, 0x03, 0x04, 0x05];

        let mock = tokio_test::io::Builder::new().read(read_data).build();
        let mut hci_driver = TokioHciDriver { hci: mock };

        let mut buffer: HciBuffer = initial_data.try_into()?;
        assert_eq!(buffer.data(), initial_data);
        assert_eq!(buffer.len(), 0);

        let read_len = buffer.read(&mut hci_driver).await.unwrap();
        assert_eq!(read_len, read_data.len());
        assert_eq!(buffer.data(), read_data);
        assert_eq!(buffer.len(), read_data.len());

        Ok(())
    }
}
