#[derive(Debug, Default)]
pub struct SupportedLeFeatures {
    value: u64,
}

impl From<u64> for SupportedLeFeatures {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

macro_rules! le_features {
    (
        $(
            $(#[$docs:meta])*
            $func:ident = $value:expr,
        )+
    ) => {
        impl SupportedLeFeatures {
            $(
                pub fn $func(&self) -> bool {
                    (self.value & (1 << $value)) != 0
                }
            )+
        }
    };
}

le_features! {
    has_le_encryption = 0,
    has_connection_parameters_request_procedure = 1,
    has_extended_reject_indication = 2,
    has_slave_initiated_features_exchange = 3,
    has_le_ping = 4,
    has_le_data_packet_length_extension = 5,
    has_ll_privacy = 6,
    has_extended_scanner_filter_policies = 7,
}
