#[derive(Debug, Default)]
pub struct SupportedFeatures {
    value: u64,
}

impl From<u64> for SupportedFeatures {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

macro_rules! features {
    (
        $(
            $(#[$docs:meta])*
            ($func:ident, $value:expr),
        )+
    ) => {
        impl SupportedFeatures {
            $(
                pub fn $func(&self) -> bool {
                    (self.value & (1 << $value)) != 0
                }
            )+
        }
    };
}

features! {
    (has_bredr_not_supported, 37),
    (has_le_supported_controller, 38),
}
