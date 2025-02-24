use bletio_utils::EncodeToBuffer;

use crate::{advertising::Uri, assigned_numbers::AdType};

/// Uniform Resource Identifier.
///
/// The URI is encoded as defined in
/// [Supplement to the Bluetooth Core Specification, Part A, 1.18](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-64bd7c4c-daf3-7a73-143a-b3dba8faac95).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct UriAdStruct {
    uri: Uri,
}

impl UriAdStruct {
    pub(crate) const fn new(uri: Uri) -> Self {
        Self { uri }
    }
}

impl EncodeToBuffer for UriAdStruct {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        let mut len = buffer.try_push((self.encoded_size() - 1) as u8)?;
        len += buffer.try_push(AdType::Uri as u8)?;
        len += self.uri.encode(buffer)?;
        Ok(len)
    }

    fn encoded_size(&self) -> usize {
        2 + self.uri.encoded_size()
    }
}

pub(crate) mod parser {
    use nom::{combinator::map, IResult, Parser};

    use crate::advertising::{ad_struct::AdStruct, uri::parser::uri};

    use super::*;

    pub(crate) fn uri_ad_struct(input: &[u8]) -> IResult<&[u8], AdStruct> {
        map(uri, |uri| AdStruct::Uri(UriAdStruct::new(uri))).parse(input)
    }
}

#[cfg(test)]
mod test {
    use bletio_utils::{Buffer, BufferOps};
    use rstest::rstest;

    use crate::advertising::{ad_struct::AdStruct, custom_uri_scheme, AdvertisingError};
    use crate::assigned_numbers::ProvisionedUriScheme;

    use super::{parser::*, *};

    #[rstest]
    #[case(
        Uri::try_new(ProvisionedUriScheme::Http, "//example.org/").unwrap(),
        &[0x11, 0x24, 0x16, 0x00, b'/', b'/', b'e', b'x', b'a', b'm', b'p', b'l', b'e', b'.', b'o', b'r', b'g', b'/']
    )]
    #[case(
        Uri::try_new(custom_uri_scheme!("custom"), "uri").unwrap(),
        &[0x0D, 0x24, 0x01, 0x00, b'c', b'u', b's', b't', b'o', b'm', b':', b'u', b'r', b'i']
    )]
    fn test_uri_ad_struct_success(
        #[case] uri: Uri,
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<24>::default();
        let value = UriAdStruct::new(uri);
        assert_eq!(value.encoded_size(), encoded_data.len());
        value.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }

    #[rstest]
    #[case(Uri::try_new(ProvisionedUriScheme::Http, "//example.org/a-path-that-is-too-long"))]
    #[case(Uri::try_new(custom_uri_scheme!("very-long-custom-scheme"), "the-uri-hier-part-that-is-too-long"))]
    fn test_uri_ad_struct_failure(#[case] err: Result<Uri, AdvertisingError>) {
        assert_eq!(
            err,
            Err(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
        );
    }

    #[rstest]
    #[case(
        &[0x16, 0x00, b'/', b'/', b'e', b'x', b'a', b'm', b'p', b'l', b'e', b'.', b'o', b'r', b'g', b'/'],
        Uri::try_new(ProvisionedUriScheme::Http, "//example.org/").unwrap()
    )]
    #[case(
        &[0x01, 0x00, b'c', b'u', b's', b't', b'o', b'm', b':', b'r', b'e', b's', b't'],
        Uri::try_new(custom_uri_scheme!("custom"), "rest").unwrap()
    )]
    fn test_uri_ad_struct_parsing_success(#[case] input: &[u8], #[case] uri: Uri) {
        assert_eq!(
            uri_ad_struct(input),
            Ok((&[] as &[u8], AdStruct::Uri(UriAdStruct::new(uri))))
        );
    }

    #[rstest]
    #[case(
        &[0x16, 0x00, b'/', b'/', b'e', b'x', b'a', b'm', b'p', b'l', b'e', b'.', b'o', b'r', b'g', b'/', b'a', b'-', b'p',
            b'a', b't', b'h', b'-', b't', b'h', b'a', b't', b'-', b'i', b's', b'-', b't', b'o', b'o', b'-', b'l', b'o', b'n', b'g']
    )]
    #[case(
        &[0x01, 0x00, b'c', b'u', b's', b't', b'o', b'm', b':', b'a', b'-', b'h', b'i', b'e', b'r', b'-', b'p', b'a', b'r',
            b't', b'-', b't', b'h', b'a', b't', b'-', b'i', b's', b'-', b't', b'o', b'o', b'-', b'l', b'o', b'n', b'g']
    )]
    fn test_uri_ad_struct_parsing_failure(#[case] input: &[u8]) {
        assert!(uri_ad_struct(input).is_err());
    }
}
