#[derive(Debug, Default)]
pub struct LeFeatures {
    value: [u8; 8],
}

impl From<[u8; 8]> for LeFeatures {
    fn from(value: [u8; 8]) -> Self {
        Self { value }
    }
}

macro_rules! le_features {
    (
        $(
            $(#[$docs:meta])*
            ($func:ident, $byte:expr, $bit:expr),
        )+
    ) => {
        impl LeFeatures {
            $(
                pub fn $func(&self) -> bool {
                    self.value[$byte] & (1 << $bit) != 0
                }
            )+
        }
    };
}

le_features! {
    (has_le_encryption, 0, 0),
    (has_connection_parameters_request_procedure, 0, 1),
    (has_extended_reject_indication, 0, 2),
    (has_slave_initiated_features_exchange, 0, 3),
    (has_le_ping, 0, 4),
    (has_le_data_packet_length_extension, 0, 5),
    (has_ll_privacy, 0, 6),
    (has_extended_scanner_filter_policies, 0, 7),
}
