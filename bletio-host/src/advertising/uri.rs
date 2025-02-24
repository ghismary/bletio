use bletio_utils::EncodeToBuffer;
use heapless::String;

use crate::{advertising::AdvertisingError, assigned_numbers::ProvisionedUriScheme};

const CUSTOM_URI_SCHEME_MAX_LENGTH: usize = 26;
const URI_HIER_PART_MAX_LENGTH: usize = 27;
const EMPTY_SCHEME_NAME_VALUE: u16 = 0x0001;

/// An URI to be included in the Universal Resource Identifier Advertising Structure.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Uri {
    scheme: UriScheme,
    hier_part: String<URI_HIER_PART_MAX_LENGTH>,
}

impl Uri {
    /// Create an URI.
    pub fn try_new(
        scheme: impl Into<UriScheme>,
        hier_part: &'static str,
    ) -> Result<Self, AdvertisingError> {
        fn inner(scheme: UriScheme, hier_part: &'static str) -> Result<Uri, AdvertisingError> {
            Ok(Uri {
                scheme,
                hier_part: hier_part
                    .try_into()
                    .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?,
            })
        }
        inner(scheme.into(), hier_part)
    }
}

impl EncodeToBuffer for Uri {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        self.scheme.encode(buffer)?;
        buffer.copy_from_slice(self.hier_part.as_bytes())?;
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        self.scheme.encoded_size() + self.hier_part.len()
    }
}

#[diagnostic::on_unimplemented(message = "the URI scheme cannot be empty")]
#[doc(hidden)]
pub trait CustomUriSchemeNotEmpty {}

#[doc(hidden)]
pub struct CustomUriSchemeIsNotEmpty<const VALID: bool>;

#[doc(hidden)]
impl CustomUriSchemeNotEmpty for CustomUriSchemeIsNotEmpty<true> {}

#[doc(hidden)]
pub const fn custom_uri_scheme_is_not_empty<T: CustomUriSchemeNotEmpty>() {}

#[doc(hidden)]
pub const fn check_custom_uri_scheme_is_not_empty(scheme: &str) -> bool {
    !scheme.is_empty()
}

#[diagnostic::on_unimplemented(message = "the URI scheme cannot contain non-ascii characters")]
#[doc(hidden)]
pub trait CustomUriSchemeOnlyAsciiChars {}

#[doc(hidden)]
pub struct CustomUriSchemeHasOnlyAsciiChars<const VALID: bool>;

#[doc(hidden)]
impl CustomUriSchemeOnlyAsciiChars for CustomUriSchemeHasOnlyAsciiChars<true> {}

#[doc(hidden)]
pub const fn custom_uri_scheme_has_only_ascii_chars<T: CustomUriSchemeOnlyAsciiChars>() {}

#[doc(hidden)]
pub const fn check_custom_uri_scheme_has_only_ascii_chars(scheme: &str) -> bool {
    scheme.is_ascii()
}

#[diagnostic::on_unimplemented(
    message = "the URI scheme cannot contain characters other than alphanumeric, '-' and '.'"
)]
#[doc(hidden)]
pub trait CustomUriSchemeOnlyValidChars {}

#[doc(hidden)]
pub struct CustomUriSchemeHasOnlyValidChars<const VALID: bool>;

#[doc(hidden)]
impl CustomUriSchemeOnlyValidChars for CustomUriSchemeHasOnlyValidChars<true> {}

#[doc(hidden)]
pub const fn custom_uri_scheme_has_only_valid_chars<T: CustomUriSchemeOnlyValidChars>() {}

#[doc(hidden)]
pub const fn check_custom_uri_scheme_has_only_valid_chars(scheme: &str) -> bool {
    let mut i = 0;
    while i < scheme.len() {
        let c = scheme.as_bytes()[i] as char;
        if !(c.is_ascii_alphanumeric() || c == '-' || c == '.') {
            return false;
        }
        i += 1;
    }
    true
}

#[diagnostic::on_unimplemented(
    message = "the URI scheme cannot start with a non-alphabetic character"
)]
#[doc(hidden)]
pub trait CustomUriSchemeAlphabeticFirstChar {}

#[doc(hidden)]
pub struct CustomUriSchemeHasAlphabeticFirstChar<const VALID: bool>;

#[doc(hidden)]
impl CustomUriSchemeAlphabeticFirstChar for CustomUriSchemeHasAlphabeticFirstChar<true> {}

#[doc(hidden)]
pub const fn custom_uri_scheme_has_alphabetic_first_char<T: CustomUriSchemeAlphabeticFirstChar>() {}

#[doc(hidden)]
pub const fn check_custom_uri_scheme_has_alphabetic_first_char(scheme: &str) -> bool {
    if scheme.is_empty() {
        return true; // Not really true, but the check on emptiness will trigger
    }
    let c = scheme.as_bytes()[0];
    c.is_ascii_alphabetic()
}

#[diagnostic::on_unimplemented(
    message = "the URI scheme cannot end with a non-alphanumeric character"
)]
#[doc(hidden)]
pub trait CustomUriSchemeAlphanumericLastChar {}

#[doc(hidden)]
pub struct CustomUriSchemeHasAlphanumericLastChar<const VALID: bool>;

#[doc(hidden)]
impl CustomUriSchemeAlphanumericLastChar for CustomUriSchemeHasAlphanumericLastChar<true> {}

#[doc(hidden)]
pub const fn custom_uri_scheme_has_alphanumeric_last_char<
    T: CustomUriSchemeAlphanumericLastChar,
>() {
}

#[doc(hidden)]
pub const fn check_custom_uri_scheme_has_alphanumeric_last_char(scheme: &str) -> bool {
    if scheme.is_empty() {
        return true; // Not really true, but the check on emptiness will trigger
    }
    let c = scheme.as_bytes()[scheme.len() - 1];
    c.is_ascii_alphanumeric()
}

#[diagnostic::on_unimplemented(
    message = "the URI scheme is too long, it must be less than 26 bytes long"
)]
#[doc(hidden)]
pub trait CustomUriSchemeNotTooLong {}

#[doc(hidden)]
pub struct CustomUriSchemeIsNotTooLong<const VALID: bool>;

#[doc(hidden)]
impl CustomUriSchemeNotTooLong for CustomUriSchemeIsNotTooLong<true> {}

#[doc(hidden)]
pub const fn custom_uri_scheme_is_not_too_long<T: CustomUriSchemeNotTooLong>() {}

#[doc(hidden)]
pub const fn check_custom_uri_scheme_is_not_too_long(scheme: &str) -> bool {
    scheme.len() <= CUSTOM_URI_SCHEME_MAX_LENGTH
}

/// Create a [`CustomUriScheme`], checking that it is valid at compile-time.
///
/// # Examples
///
/// ```
/// # use bletio_host::advertising::custom_uri_scheme;
/// let scheme = custom_uri_scheme!("custom");
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __custom_uri_scheme__ {
    ($scheme:expr) => {{
        const NOT_EMPTY_ERR: bool =
            $crate::advertising::uri::check_custom_uri_scheme_is_not_empty($scheme);
        $crate::advertising::uri::custom_uri_scheme_is_not_empty::<
            $crate::advertising::uri::CustomUriSchemeIsNotEmpty<NOT_EMPTY_ERR>,
        >();
        const ONLY_ASCII_CHARS_ERR: bool =
            $crate::advertising::uri::check_custom_uri_scheme_has_only_ascii_chars($scheme);
        $crate::advertising::uri::custom_uri_scheme_has_only_ascii_chars::<
            $crate::advertising::uri::CustomUriSchemeHasOnlyAsciiChars<ONLY_ASCII_CHARS_ERR>,
        >();
        const ONLY_VALID_CHARS_ERR: bool =
            $crate::advertising::uri::check_custom_uri_scheme_has_only_valid_chars($scheme);
        $crate::advertising::uri::custom_uri_scheme_has_only_valid_chars::<
            $crate::advertising::uri::CustomUriSchemeHasOnlyValidChars<ONLY_VALID_CHARS_ERR>,
        >();
        const ALPHABETIC_FIRST_CHAR_ERR: bool =
            $crate::advertising::uri::check_custom_uri_scheme_has_alphabetic_first_char($scheme);
        $crate::advertising::uri::custom_uri_scheme_has_alphabetic_first_char::<
            $crate::advertising::uri::CustomUriSchemeHasAlphabeticFirstChar<
                ALPHABETIC_FIRST_CHAR_ERR,
            >,
        >();
        const ALPHANUMERIC_LAST_CHAR_ERR: bool =
            $crate::advertising::uri::check_custom_uri_scheme_has_alphanumeric_last_char($scheme);
        $crate::advertising::uri::custom_uri_scheme_has_alphanumeric_last_char::<
            $crate::advertising::uri::CustomUriSchemeHasAlphanumericLastChar<
                ALPHANUMERIC_LAST_CHAR_ERR,
            >,
        >();
        const NOT_TOO_LONG_ERR: bool =
            $crate::advertising::uri::check_custom_uri_scheme_is_not_too_long($scheme);
        $crate::advertising::uri::custom_uri_scheme_is_not_too_long::<
            $crate::advertising::uri::CustomUriSchemeIsNotTooLong<NOT_TOO_LONG_ERR>,
        >();
        $crate::advertising::uri::CustomUriScheme::try_new($scheme).unwrap()
    }};
}

#[doc(inline)]
pub use __custom_uri_scheme__ as custom_uri_scheme;

/// A custom URI scheme if the scheme you want to use is not defined in [`ProvisionedUriScheme`].
///
/// Use [`custom_uri_scheme`] to create it.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CustomUriScheme {
    scheme: String<CUSTOM_URI_SCHEME_MAX_LENGTH>,
}

impl CustomUriScheme {
    #[doc(hidden)]
    pub fn try_new(scheme: &str) -> Result<Self, AdvertisingError> {
        Ok(Self {
            scheme: scheme
                .try_into()
                .map_err(|_| AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)?,
        })
    }
}

/// An URI scheme, either provisioned or custom.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UriScheme {
    /// A provisioned URI scheme.
    Provisioned(ProvisionedUriScheme),
    /// A custom URI scheme.
    Custom(CustomUriScheme),
}

impl UriScheme {
    const fn value(&self) -> u16 {
        match self {
            Self::Provisioned(scheme) => *scheme as u16,
            Self::Custom(_) => EMPTY_SCHEME_NAME_VALUE,
        }
    }
}

impl EncodeToBuffer for UriScheme {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.encode_le_u16(self.value())?;
        if let Self::Custom(scheme) = &self {
            buffer.copy_from_slice(scheme.scheme.as_bytes())?;
            buffer.try_push(b':')?;
        }
        Ok(self.encoded_size())
    }

    fn encoded_size(&self) -> usize {
        match self {
            Self::Provisioned(_) => 2,
            Self::Custom(scheme) => 3 + scheme.scheme.len(),
        }
    }
}

impl From<ProvisionedUriScheme> for UriScheme {
    fn from(value: ProvisionedUriScheme) -> Self {
        Self::Provisioned(value)
    }
}

impl From<CustomUriScheme> for UriScheme {
    fn from(value: CustomUriScheme) -> Self {
        Self::Custom(value)
    }
}

pub(crate) mod parser {
    use nom::{
        branch::alt,
        bytes::{tag, take, take_till1},
        combinator::{map, map_res, verify},
        number::le_u16,
        IResult, Parser,
    };

    use super::*;

    fn custom_uri_scheme(input: &[u8]) -> IResult<&[u8], UriScheme> {
        map(
            map_res(
                (
                    verify(le_u16(), |value| *value == EMPTY_SCHEME_NAME_VALUE),
                    map_res(take_till1(|b| b == b':'), core::str::from_utf8),
                    tag([b':'].as_slice()),
                ),
                |(_, scheme, _)| CustomUriScheme::try_new(scheme),
            ),
            Into::into,
        )
        .parse(input)
    }

    fn provisioned_uri_scheme(input: &[u8]) -> IResult<&[u8], UriScheme> {
        map(
            map_res(le_u16(), TryFrom::try_from),
            |scheme: ProvisionedUriScheme| scheme.into(),
        )
        .parse(input)
    }

    fn uri_scheme(input: &[u8]) -> IResult<&[u8], UriScheme> {
        alt((provisioned_uri_scheme, custom_uri_scheme)).parse(input)
    }

    pub(crate) fn uri(input: &[u8]) -> IResult<&[u8], Uri> {
        let (rest, scheme) = uri_scheme.parse(input)?;
        let len = rest.len();
        let mut uri = Uri {
            scheme,
            hier_part: Default::default(),
        };
        map_res(map_res(take(len), core::str::from_utf8), |v| {
            uri.hier_part.push_str(v)
        })
        .parse(rest)?;
        Ok((&[], uri))
    }
}

#[cfg(test)]
mod test {
    use bletio_utils::{Buffer, BufferOps};
    use rstest::rstest;

    use super::{parser::*, *};

    #[test]
    fn test_check_custom_uri_scheme_is_not_empty() {
        assert!(!check_custom_uri_scheme_is_not_empty(""));
        assert!(check_custom_uri_scheme_is_not_empty("custom"));
    }

    #[test]
    fn test_check_custom_uri_scheme_has_only_ascii_chars() {
        assert!(check_custom_uri_scheme_has_only_ascii_chars("custom"));
        assert!(!check_custom_uri_scheme_has_only_ascii_chars("custおm"));
    }

    #[test]
    fn test_check_custom_uri_scheme_has_only_valid_chars() {
        assert!(check_custom_uri_scheme_has_only_valid_chars("custom"));
        assert!(!check_custom_uri_scheme_has_only_valid_chars("cus/om"));
    }

    #[test]
    fn test_check_custom_uri_scheme_has_alphabetic_first_char() {
        assert!(check_custom_uri_scheme_has_alphabetic_first_char("custom"));
        assert!(!check_custom_uri_scheme_has_alphabetic_first_char(
            ".custom"
        ));
        assert!(!check_custom_uri_scheme_has_alphabetic_first_char(
            "2custom"
        ));
        assert!(check_custom_uri_scheme_has_alphabetic_first_char(""));
    }

    #[test]
    fn test_check_custom_uri_scheme_has_alphanumeric_last_char() {
        assert!(check_custom_uri_scheme_has_alphanumeric_last_char("custom"));
        assert!(!check_custom_uri_scheme_has_alphanumeric_last_char(
            "custom-"
        ));
        assert!(check_custom_uri_scheme_has_alphanumeric_last_char(""));
    }

    #[test]
    fn test_check_custom_uri_scheme_is_not_too_long() {
        assert!(check_custom_uri_scheme_is_not_too_long("custom"));
        assert!(!check_custom_uri_scheme_is_not_too_long(
            "a-very-very-long-uri-scheme"
        ));
        assert!(check_custom_uri_scheme_is_not_too_long(""));
    }

    #[test]
    fn test_custom_uri_scheme() {
        let scheme = custom_uri_scheme!("custom");
        assert_eq!(scheme.scheme, "custom");

        let scheme = custom_uri_scheme!("cus-tom");
        assert_eq!(scheme.scheme, "cus-tom");

        let scheme = custom_uri_scheme!("cu.st.om");
        assert_eq!(scheme.scheme, "cu.st.om");

        let scheme = custom_uri_scheme!("cust0m");
        assert_eq!(scheme.scheme, "cust0m");

        let scheme = custom_uri_scheme!("be3");
        assert_eq!(scheme.scheme, "be3");
    }

    #[rstest]
    #[case(ProvisionedUriScheme::Http.into(), &[0x16, 0x00])]
    #[case(custom_uri_scheme!("custom").into(), &[0x01, 0x00, b'c', b'u', b's', b't', b'o', b'm', b':'])]
    fn test_uri_scheme_success(
        #[case] uri_scheme: UriScheme,
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<12>::default();
        uri_scheme.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }

    #[test]
    fn test_uri_scheme_failure() {
        let err = CustomUriScheme::try_new("very-very-long-custom-scheme");
        assert_eq!(
            err,
            Err(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket),
        );

        let mut buffer = Buffer::<12>::default();
        let uri_scheme: UriScheme = custom_uri_scheme!("very-long-custom-scheme").into();
        let err = uri_scheme.encode(&mut buffer);
        assert_eq!(err, Err(bletio_utils::Error::BufferTooSmall));
    }

    #[rstest]
    #[case(
        Uri::try_new(ProvisionedUriScheme::Http, "//example.org/").unwrap(),
        &[0x16, 0x00, b'/', b'/', b'e', b'x', b'a', b'm', b'p', b'l', b'e', b'.', b'o', b'r', b'g', b'/']
    )]
    #[case(
        Uri::try_new(custom_uri_scheme!("custom"), "rest").unwrap(),
        &[0x01, 0x00, b'c', b'u', b's', b't', b'o', b'm', b':', b'r', b'e', b's', b't']
    )]
    fn test_uri_success(
        #[case] uri: Uri,
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<16>::default();
        uri.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }

    #[rstest]
    #[case(Uri::try_new(ProvisionedUriScheme::Http, "//example.org/a-path-that-is-too-long"))]
    #[case(Uri::try_new(custom_uri_scheme!("custom"), "a-hier-part-that-is-too-long"))]
    fn test_uri_failure(#[case] err: Result<Uri, AdvertisingError>) {
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
    fn test_uri_parsing_success(#[case] input: &[u8], #[case] expected_uri: Uri) {
        assert_eq!(uri(input), Ok((&[] as &[u8], expected_uri)));
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
    fn test_uri_parsing_failure(#[case] input: &[u8]) {
        assert!(uri(input).is_err());
    }
}
