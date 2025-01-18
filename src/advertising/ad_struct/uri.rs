use crate::advertising::ad_struct::{AdStruct, AdStructBuffer, AdStructType};
use crate::advertising::advertising_data::ADVERTISING_DATA_MAX_SIZE;
use crate::advertising::AdvertisingError;
use crate::assigned_numbers::{AdType, ProvisionedUriScheme};
use crate::Error;

const EMPTY_SCHEME_NAME_VALUE: u16 = 0x0001;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uri {
    scheme: UriScheme,
    hier_part: &'static str,
}

impl Uri {
    pub fn new(scheme: impl Into<UriScheme>, hier_part: &'static str) -> Self {
        Self {
            scheme: scheme.into(),
            hier_part,
        }
    }

    fn try_encode(
        &self,
        buffer: &mut AdStructBuffer<ADVERTISING_DATA_MAX_SIZE>,
    ) -> Result<(), AdvertisingError> {
        self.scheme.try_encode(buffer)?;
        buffer.copy_from_slice(self.hier_part.as_bytes())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomUriScheme {
    scheme: &'static str,
}

impl CustomUriScheme {
    pub fn try_new(scheme: &'static str) -> Result<CustomUriScheme, AdvertisingError> {
        scheme.try_into()
    }
}

impl TryFrom<&'static str> for CustomUriScheme {
    type Error = AdvertisingError;

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        let contains_invalid_characters = value
            .chars()
            .any(|c| !(c.is_ascii_alphanumeric() || c == '-' || c == '.'));
        let first_character_is_invalid = value
            .chars()
            .nth(0)
            .is_none_or(|c| !c.is_ascii_alphabetic());
        let last_character_is_invalid = value
            .chars()
            .last()
            .is_none_or(|c| !c.is_ascii_alphanumeric());

        if contains_invalid_characters || first_character_is_invalid || last_character_is_invalid {
            Err(AdvertisingError::InvalidCustomUriScheme(value))
        } else {
            Ok(Self { scheme: value })
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UriScheme {
    Provisioned(ProvisionedUriScheme),
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
    fn test_custom_uri_scheme_creation_success() -> Result<(), AdvertisingError> {
        let scheme = CustomUriScheme::try_new("custom")?;
        assert_eq!(scheme.scheme, "custom");

        let scheme: CustomUriScheme = "cus-tom".try_into()?;
        assert_eq!(scheme.scheme, "cus-tom");

        let scheme: CustomUriScheme = "cu.st.om".try_into()?;
        assert_eq!(scheme.scheme, "cu.st.om");

        let scheme = CustomUriScheme::try_new("cust0m")?;
        assert_eq!(scheme.scheme, "cust0m");

        let scheme = CustomUriScheme::try_new("be3")?;
        assert_eq!(scheme.scheme, "be3");

        Ok(())
    }

    #[test]
    fn test_custom_uri_scheme_creation_failed() {
        let err = CustomUriScheme::try_new("").expect_err("Invalid custom URI scheme");
        assert!(matches!(err, AdvertisingError::InvalidCustomUriScheme("")));

        let err = CustomUriScheme::try_new("2bedefined").expect_err("Invalid custom URI scheme");
        assert!(matches!(
            err,
            AdvertisingError::InvalidCustomUriScheme("2bedefined")
        ));

        let err = CustomUriScheme::try_new("-mine").expect_err("Invalid custom URI scheme");
        assert!(matches!(
            err,
            AdvertisingError::InvalidCustomUriScheme("-mine")
        ));

        let err = CustomUriScheme::try_new(".yours").expect_err("Invalid custom URI scheme");
        assert!(matches!(
            err,
            AdvertisingError::InvalidCustomUriScheme(".yours")
        ));

        let err = CustomUriScheme::try_new("mine.").expect_err("Invalid custom URI scheme");
        assert!(matches!(
            err,
            AdvertisingError::InvalidCustomUriScheme("mine.")
        ));

        let err = CustomUriScheme::try_new("yours-").expect_err("Invalid custom URI scheme");
        assert!(matches!(
            err,
            AdvertisingError::InvalidCustomUriScheme("yours-")
        ));

        let err = CustomUriScheme::try_new("scheme:").expect_err("Invalid custom URI scheme");
        assert!(matches!(
            err,
            AdvertisingError::InvalidCustomUriScheme("scheme:")
        ));
    }
}
