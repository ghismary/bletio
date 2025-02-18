use bletio_utils::EncodeToBuffer;
use heapless::String;

use crate::{advertising::AdvertisingError, assigned_numbers::AdType};

const LOCAL_NAME_MAX_LENGTH: usize = 29;

/// Whether the local name is complete or shortened.
///
/// Used when creating a Local Name Advertising Structures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LocalNameComplete {
    Complete,
    Shortened(usize),
}

/// The local name of the device.
///
/// The local name of the device is defined in
/// [Supplement to the Bluetooth Core Specification, Part A, 1.2](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-351cc997-6a3c-8980-31cb-21b2ffcb103f).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LocalNameAdStruct {
    local_name: String<LOCAL_NAME_MAX_LENGTH>,
    pub(crate) complete: LocalNameComplete,
}

impl LocalNameAdStruct {
    pub(crate) fn try_new(
        local_name: &str,
        complete: LocalNameComplete,
    ) -> Result<Self, AdvertisingError> {
        let len = match complete {
            LocalNameComplete::Complete => local_name.len(),
            LocalNameComplete::Shortened(len) => {
                if len > local_name.len() {
                    local_name.len()
                } else {
                    len
                }
            }
        };
        Ok(Self {
            local_name: (&local_name[..len])
                .try_into()
                .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?,
            complete,
        })
    }

    fn len(&self) -> usize {
        self.local_name.len()
    }
}

impl EncodeToBuffer for LocalNameAdStruct {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        let ad_type = match self.complete {
            LocalNameComplete::Complete => AdType::CompleteLocalName,
            LocalNameComplete::Shortened(_) => AdType::ShortenedLocalName,
        };
        buffer.try_push((self.encoded_size() - 1) as u8)?;
        buffer.try_push(ad_type as u8)?;
        buffer.copy_from_slice(self.local_name.as_bytes())?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        self.len() + 2
    }
}

#[cfg(test)]
mod test {
    use bletio_utils::{Buffer, BufferOps};
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("", LocalNameComplete::Complete, &[0x01, 0x09])]
    #[case("", LocalNameComplete::Shortened(3), &[0x01, 0x08])]
    #[case("bletio", LocalNameComplete::Complete, &[0x07, 0x09, b'b', b'l', b'e', b't', b'i', b'o'])]
    #[case("bletio", LocalNameComplete::Shortened(3), &[0x04, 0x08, b'b', b'l', b'e'])]
    #[case("bletio", LocalNameComplete::Shortened(5), &[0x06, 0x08, b'b', b'l', b'e', b't', b'i'])]
    #[case("bletio", LocalNameComplete::Shortened(10), &[0x07, 0x08, b'b', b'l', b'e', b't', b'i', b'o'])]
    fn test_local_name_ad_struct(
        #[case] local_name: &str,
        #[case] complete: LocalNameComplete,
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<8>::default();
        let value = LocalNameAdStruct::try_new(local_name, complete).unwrap();
        value.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }
}
