use crate::advertising::ad_struct::{AdStruct, AdStructBuffer, AdStructType};
use crate::advertising::advertising_data::ADVERTISING_DATA_MAX_SIZE;
use crate::advertising::AdvertisingError;
use crate::assigned_numbers::{AdType, ProvisionedUriScheme};
use crate::Error;

const EMPTY_SCHEME_NAME_VALUE: u16 = 0x0001;

/// An URI to be included in the Universal Resource Identifier Advertising Structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uri {
    scheme: UriScheme,
    hier_part: &'static str,
}

impl Uri {
    /// Create an URI.
    pub fn new(scheme: impl Into<UriScheme>, hier_part: &'static str) -> Self {
        fn inner(scheme: UriScheme, hier_part: &'static str) -> Uri {
            Uri { scheme, hier_part }
        }
        inner(scheme.into(), hier_part)
    }

    fn try_encode(
        &self,
        buffer: &mut AdStructBuffer<ADVERTISING_DATA_MAX_SIZE>,
    ) -> Result<(), AdvertisingError> {
        self.scheme.try_encode(buffer)?;
        buffer.copy_from_slice(self.hier_part.as_bytes())
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
    while i < scheme.as_bytes().len() {
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
    let c = scheme.as_bytes()[scheme.as_bytes().len() - 1];
    c.is_ascii_alphanumeric()
}

/// A custom URI scheme if the scheme you want to use is not defined in [`ProvisionedUriScheme`].
///
/// Use [`custom_uri_scheme`] to create it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomUriScheme {
    scheme: &'static str,
}

impl CustomUriScheme {
    #[doc(hidden)]
    pub const fn new(scheme: &'static str) -> Self {
        Self { scheme }
    }
}

/// Create a [`CustomUriScheme`], checking that it is valid at compile-time.
///
/// # Examples
///
/// ```
/// # use bletio::advertising::ad_struct::custom_uri_scheme;
/// let scheme = custom_uri_scheme!("custom");
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __custom_uri_scheme__ {
    ($scheme:expr) => {{
        const NOT_EMPTY_ERR: bool =
            $crate::advertising::ad_struct::uri::check_custom_uri_scheme_is_not_empty($scheme);
        $crate::advertising::ad_struct::uri::custom_uri_scheme_is_not_empty::<
            $crate::advertising::ad_struct::uri::CustomUriSchemeIsNotEmpty<NOT_EMPTY_ERR>,
        >();
        const ONLY_ASCII_CHARS_ERR: bool =
            $crate::advertising::ad_struct::uri::check_custom_uri_scheme_has_only_ascii_chars(
                $scheme,
            );
        $crate::advertising::ad_struct::uri::custom_uri_scheme_has_only_ascii_chars::<
            $crate::advertising::ad_struct::uri::CustomUriSchemeHasOnlyAsciiChars<
                ONLY_ASCII_CHARS_ERR,
            >,
        >();
        const ONLY_VALID_CHARS_ERR: bool =
            $crate::advertising::ad_struct::uri::check_custom_uri_scheme_has_only_valid_chars(
                $scheme,
            );
        $crate::advertising::ad_struct::uri::custom_uri_scheme_has_only_valid_chars::<
            $crate::advertising::ad_struct::uri::CustomUriSchemeHasOnlyValidChars<
                ONLY_VALID_CHARS_ERR,
            >,
        >();
        const ALPHABETIC_FIRST_CHAR_ERR: bool =
            $crate::advertising::ad_struct::uri::check_custom_uri_scheme_has_alphabetic_first_char(
                $scheme,
            );
        $crate::advertising::ad_struct::uri::custom_uri_scheme_has_alphabetic_first_char::<
            $crate::advertising::ad_struct::uri::CustomUriSchemeHasAlphabeticFirstChar<
                ALPHABETIC_FIRST_CHAR_ERR,
            >,
        >();
        const ALPHANUMERIC_LAST_CHAR_ERR: bool =
            $crate::advertising::ad_struct::uri::check_custom_uri_scheme_has_alphanumeric_last_char(
                $scheme,
            );
        $crate::advertising::ad_struct::uri::custom_uri_scheme_has_alphanumeric_last_char::<
            $crate::advertising::ad_struct::uri::CustomUriSchemeHasAlphanumericLastChar<
                ALPHANUMERIC_LAST_CHAR_ERR,
            >,
        >();
        $crate::advertising::ad_struct::uri::CustomUriScheme::new($scheme)
    }};
}

#[doc(inline)]
pub use __custom_uri_scheme__ as custom_uri_scheme;

/// An URI scheme, either provisioned or custom.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UriScheme {
    /// A provisioned URI scheme.
    Provisioned(ProvisionedUriScheme),
    /// A custom URI scheme.
    Custom(CustomUriScheme),
}

impl UriScheme {
    fn try_encode(
        &self,
        buffer: &mut AdStructBuffer<ADVERTISING_DATA_MAX_SIZE>,
    ) -> Result<(), AdvertisingError> {
        buffer.encode_le_u16(self.value())?;
        if let Self::Custom(scheme) = &self {
            buffer.copy_from_slice(scheme.scheme.as_bytes())?;
            buffer.try_push(b':')?;
        }
        Ok(())
    }

    fn value(&self) -> u16 {
        match self {
            Self::Provisioned(scheme) => *scheme as u16,
            Self::Custom(_) => EMPTY_SCHEME_NAME_VALUE,
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

/// Uniform Resource Identifier.
///
/// The URI is encoded as defined in
/// [Supplement to the Bluetooth Core Specification, Part A, 1.18](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v12/CSS/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html#UUID-64bd7c4c-daf3-7a73-143a-b3dba8faac95).
#[derive(Debug, Clone)]
pub struct UriAdStruct {
    buffer: AdStructBuffer<ADVERTISING_DATA_MAX_SIZE>,
}

impl UriAdStruct {
    /// Create an URI Advertising Structure.
    ///
    /// # Arguments
    ///
    /// * `uri` — The [`Uri`] to notify.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bletio::advertising::ad_struct::{Uri, UriAdStruct};
    /// # use bletio::assigned_numbers::ProvisionedUriScheme;
    /// # fn main() -> Result<(), bletio::Error> {
    /// let ad_struct = UriAdStruct::try_new(
    ///     Uri::new(ProvisionedUriScheme::Http, "//example.org")
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_new(uri: Uri) -> Result<Self, Error> {
        let mut s = Self {
            buffer: AdStructBuffer::new(AdType::Uri),
        };
        uri.try_encode(&mut s.buffer)?;
        Ok(s)
    }
}

impl AdStruct for UriAdStruct {
    fn encoded_data(&self) -> &[u8] {
        self.buffer.data()
    }
    fn r#type(&self) -> AdStructType {
        AdStructType::URI
    }

    fn is_unique(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

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

    #[test]
    fn test_uri_scheme_try_encode_success() -> Result<(), AdvertisingError> {
        let mut buffer = AdStructBuffer::new(AdType::Uri);
        let scheme: UriScheme = ProvisionedUriScheme::Http.into();
        scheme.try_encode(&mut buffer)?;
        assert_eq!(buffer.data(), &[0x03, 0x24, 0x16, 0x00]);

        let mut buffer = AdStructBuffer::new(AdType::Uri);
        let scheme: UriScheme = custom_uri_scheme!("custom").into();
        scheme.try_encode(&mut buffer)?;
        assert_eq!(
            buffer.data(),
            &[0x0A, 0x24, 0x01, 0x00, b'c', b'u', b's', b't', b'o', b'm', b':']
        );

        Ok(())
    }

    #[test]
    fn test_uri_scheme_try_encode_failure() {
        let mut buffer = AdStructBuffer::new(AdType::Uri);
        let scheme: UriScheme = custom_uri_scheme!("very-very-very-long-custom-scheme").into();
        let err = scheme
            .try_encode(&mut buffer)
            .expect_err("Custom URI scheme too long");
        assert!(matches!(
            err,
            AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket
        ));
    }

    #[test]
    fn test_uri_try_encode_success() -> Result<(), AdvertisingError> {
        let mut buffer = AdStructBuffer::new(AdType::Uri);
        let uri: Uri = Uri::new(ProvisionedUriScheme::Http, "//example.org/");
        uri.try_encode(&mut buffer)?;
        assert_eq!(
            buffer.data(),
            &[
                0x11, 0x24, 0x16, 0x00, b'/', b'/', b'e', b'x', b'a', b'm', b'p', b'l', b'e', b'.',
                b'o', b'r', b'g', b'/'
            ]
        );

        let mut buffer = AdStructBuffer::new(AdType::Uri);
        let uri: Uri = Uri::new(custom_uri_scheme!("custom"), "rest");
        uri.try_encode(&mut buffer)?;
        assert_eq!(
            buffer.data(),
            &[
                0x0E, 0x24, 0x01, 0x00, b'c', b'u', b's', b't', b'o', b'm', b':', b'r', b'e', b's',
                b't'
            ]
        );

        Ok(())
    }

    #[test]
    fn test_uri_try_encode_failure() {
        let mut buffer = AdStructBuffer::new(AdType::Uri);
        let uri: Uri = Uri::new(
            ProvisionedUriScheme::Http,
            "//example.org/a-path-that-is-too-long",
        );
        let err = uri.try_encode(&mut buffer).expect_err("URI too long");
        assert!(matches!(
            err,
            AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket
        ));

        let mut buffer = AdStructBuffer::new(AdType::Uri);
        let uri: Uri = Uri::new(custom_uri_scheme!("custom"), "a-hier-part-that-is-too-long");
        let err = uri.try_encode(&mut buffer).expect_err("URI too long");
        assert!(matches!(
            err,
            AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket
        ));
    }

    #[test]
    fn test_uri_ad_struct_creation_success() -> Result<(), Error> {
        let value = UriAdStruct::try_new(Uri::new(ProvisionedUriScheme::Http, "//example.org/"))?;
        assert_eq!(
            value.encoded_data(),
            &[
                0x11, 0x24, 0x16, 0x00, b'/', b'/', b'e', b'x', b'a', b'm', b'p', b'l', b'e', b'.',
                b'o', b'r', b'g', b'/'
            ]
        );
        assert!(value.r#type().contains(AdStructType::URI));
        assert!(!value.is_unique());

        let value = UriAdStruct::try_new(Uri::new(custom_uri_scheme!("custom"), "uri"))?;
        assert_eq!(
            value.encoded_data(),
            &[0x0D, 0x24, 0x01, 0x00, b'c', b'u', b's', b't', b'o', b'm', b':', b'u', b'r', b'i']
        );
        assert!(value.r#type().contains(AdStructType::URI));
        assert!(!value.is_unique());

        Ok(())
    }

    #[test]
    fn test_uri_ad_struct_creation_failure() {
        let err = UriAdStruct::try_new(Uri::new(
            ProvisionedUriScheme::Http,
            "//example.org/a-path-that-is-too-long",
        ))
        .expect_err("Uri does not fit");
        assert!(matches!(
            err,
            Error::Advertising(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
        ));

        let err = UriAdStruct::try_new(Uri::new(
            custom_uri_scheme!("very-very-very-long-custom-scheme"),
            "rest",
        ))
        .expect_err("Uri does not fit");
        assert!(matches!(
            err,
            Error::Advertising(AdvertisingError::AdvertisingDataWillNotFitAdvertisingPacket)
        ));
    }
}
