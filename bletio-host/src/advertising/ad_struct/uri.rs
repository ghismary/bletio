use bletio_utils::EncodeToBuffer;

use crate::{advertising::Uri, assigned_numbers::AdType};

/// Uniform Resource Identifier.
///
/// The URI is encoded as defined in
/// [Supplement to the Bluetooth Core Specification, Part A, 1.18](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-64bd7c4c-daf3-7a73-143a-b3dba8faac95).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct UriAdStruct {
    uri: Uri,
}

impl UriAdStruct {
    pub(crate) fn new(uri: Uri) -> Self {
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

#[cfg(test)]
mod test {
    use bletio_utils::{Buffer, BufferOps};
    use rstest::rstest;

    use crate::advertising::custom_uri_scheme;
    use crate::assigned_numbers::ProvisionedUriScheme;

    use super::*;

    #[rstest]
    #[case(
        Uri::new(ProvisionedUriScheme::Http, "//example.org/"),
        &[0x11, 0x24, 0x16, 0x00, b'/', b'/', b'e', b'x', b'a', b'm', b'p', b'l', b'e', b'.', b'o', b'r', b'g', b'/']
    )]
    #[case(
        Uri::new(custom_uri_scheme!("custom"), "uri"),
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
    #[case(Uri::new(ProvisionedUriScheme::Http, "//example.org/a-path-that-is-too-long"))]
    #[case(Uri::new(custom_uri_scheme!("very-very-very-long-custom-scheme"), "rest"))]
    fn test_uri_ad_struct_failure(#[case] uri: Uri) {
        let mut buffer = Buffer::<24>::default();
        let value = UriAdStruct::new(uri);
        let err = value.encode(&mut buffer);
        assert_eq!(err, Err(bletio_utils::Error::BufferTooSmall));
    }
}
