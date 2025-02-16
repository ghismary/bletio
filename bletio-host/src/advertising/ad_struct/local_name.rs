use bletio_utils::EncodeToBuffer;

use crate::assigned_numbers::AdType;

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
pub struct LocalNameAdStruct<'a> {
    local_name: &'a str,
    pub(crate) complete: LocalNameComplete,
}

impl<'a> LocalNameAdStruct<'a> {
    pub(crate) const fn new(local_name: &'a str, complete: LocalNameComplete) -> Self {
        Self {
            local_name,
            complete,
        }
    }

    const fn len(&self) -> usize {
        match self.complete {
            LocalNameComplete::Complete => self.local_name.len(),
            LocalNameComplete::Shortened(len) => {
                if len > self.local_name.len() {
                    self.local_name.len()
                } else {
                    len
                }
            }
        }
    }
}

impl EncodeToBuffer for LocalNameAdStruct<'_> {
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
        buffer.copy_from_slice(self.local_name[..self.len()].as_bytes())?;
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
        let value = LocalNameAdStruct::new(local_name, complete);
        value.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }
}
