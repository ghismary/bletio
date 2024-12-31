#[derive(Debug, Default)]
pub struct SupportedFeatures {
    value: [u8; 8],
}

impl From<[u8; 8]> for SupportedFeatures {
    fn from(value: [u8; 8]) -> Self {
        Self { value }
    }
}

macro_rules! features {
    (
        $(
            $(#[$docs:meta])*
            ($func:ident, $byte:expr, $bit:expr),
        )+
    ) => {
        impl SupportedFeatures {
            $(
                pub fn $func(&self) -> bool {
                    self.value[$byte] & (1 << $bit) != 0
                }
            )+
        }
    };
}

features! {
    (has_bredr_not_supported, 4, 5),
    (has_le_supported_controller, 4, 6),
}
