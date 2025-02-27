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
        fn inner(
            local_name: &str,
            complete: LocalNameComplete,
        ) -> Result<String<LOCAL_NAME_MAX_LENGTH>, ()> {
            match complete {
                LocalNameComplete::Complete => local_name.try_into(),
                LocalNameComplete::Shortened(len) => {
                    if local_name.len() > len {
                        (&local_name[..len]).try_into()
                    } else {
                        let mut local_name_str: String<LOCAL_NAME_MAX_LENGTH> =
                            local_name.try_into()?;
                        for _ in local_name.len()..len {
                            local_name_str.push(' ')?;
                        }
                        Ok(local_name_str)
                    }
                }
            }
        }
        Ok(Self {
            local_name: inner(local_name, complete)
                .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?,
            complete,
        })
    }

    pub fn value(&self) -> &str {
        self.local_name.as_str()
    }

    pub fn complete(&self) -> LocalNameComplete {
        self.complete
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

pub(crate) mod parser {
    use nom::{bytes::take, combinator::map_res, IResult, Parser};

    use crate::advertising::ad_struct::AdStruct;

    use super::*;

    pub(crate) fn shortened_local_name_ad_struct(input: &[u8]) -> IResult<&[u8], AdStruct> {
        let len = input.len();
        let mut ad_struct = LocalNameAdStruct {
            local_name: Default::default(),
            complete: LocalNameComplete::Shortened(len),
        };
        map_res(map_res(take(len), core::str::from_utf8), |v| {
            ad_struct.local_name.push_str(v)
        })
        .parse(input)?;
        Ok((&[], AdStruct::LocalName(ad_struct)))
    }

    pub(crate) fn complete_local_name_ad_struct(input: &[u8]) -> IResult<&[u8], AdStruct> {
        let len = input.len();
        let mut ad_struct = LocalNameAdStruct {
            local_name: Default::default(),
            complete: LocalNameComplete::Complete,
        };
        map_res(map_res(take(len), core::str::from_utf8), |v| {
            ad_struct.local_name.push_str(v)
        })
        .parse(input)?;
        Ok((&[], AdStruct::LocalName(ad_struct)))
    }
}

#[cfg(test)]
mod test {
    use bletio_utils::{Buffer, BufferOps};
    use rstest::rstest;

    use crate::advertising::ad_struct::AdStruct;

    use super::{parser::*, *};

    #[rstest]
    #[case("", LocalNameComplete::Complete, "", &[0x01, 0x09])]
    #[case("", LocalNameComplete::Shortened(3), "   ", &[0x04, 0x08, b' ', b' ', b' '])]
    #[case("bletio", LocalNameComplete::Complete, "bletio", &[0x07, 0x09, b'b', b'l', b'e', b't', b'i', b'o'])]
    #[case("bletio", LocalNameComplete::Shortened(3), "ble", &[0x04, 0x08, b'b', b'l', b'e'])]
    #[case("bletio", LocalNameComplete::Shortened(5), "bleti", &[0x06, 0x08, b'b', b'l', b'e', b't', b'i'])]
    #[case("bletio", LocalNameComplete::Shortened(8), "bletio  ", &[0x09, 0x08, b'b', b'l', b'e', b't', b'i', b'o', b' ', b' '])]
    fn test_local_name_ad_struct_success(
        #[case] local_name: &str,
        #[case] complete: LocalNameComplete,
        #[case] encoded_local_name: &str,
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<10>::default();
        let ad_struct = LocalNameAdStruct::try_new(local_name, complete).unwrap();
        ad_struct.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        assert_eq!(ad_struct.value(), encoded_local_name);
        assert_eq!(ad_struct.complete(), complete);
        Ok(())
    }

    #[rstest]
    #[case("a very very very long local name", LocalNameComplete::Complete)]
    #[case("a very very very long local name", LocalNameComplete::Shortened(30))]
    fn test_local_name_ad_struct_failure(
        #[case] local_name: &str,
        #[case] complete: LocalNameComplete,
    ) {
        let err = LocalNameAdStruct::try_new(local_name, complete);
        assert_eq!(
            err,
            Err(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
        );
    }

    #[test]
    fn test_shortened_local_name_ad_struct_parsing_success() {
        assert_eq!(
            shortened_local_name_ad_struct("ble".as_bytes()),
            Ok((
                &[] as &[u8],
                AdStruct::LocalName(
                    LocalNameAdStruct::try_new("ble", LocalNameComplete::Shortened(3)).unwrap()
                )
            ))
        );
    }

    #[test]
    fn test_shortened_local_name_ad_struct_parsing_failure() {
        assert!(
            shortened_local_name_ad_struct("a very very very long local name".as_bytes()).is_err()
        );
    }

    #[test]
    fn test_complete_local_name_ad_struct_parsing_success() {
        assert_eq!(
            complete_local_name_ad_struct("bletio".as_bytes()),
            Ok((
                &[] as &[u8],
                AdStruct::LocalName(
                    LocalNameAdStruct::try_new("bletio", LocalNameComplete::Complete).unwrap()
                )
            ))
        );
    }

    #[test]
    fn test_complete_local_name_ad_struct_parsing_failure() {
        assert!(
            complete_local_name_ad_struct("a very very very long local name".as_bytes()).is_err()
        );
    }
}
