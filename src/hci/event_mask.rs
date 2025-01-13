use crate::utils::{encode_le_u64, UtilsError};

#[derive(Debug, Default)]
pub(crate) struct EventMask {
    value: u64,
}

impl EventMask {
    pub(crate) fn new() -> Self {
        EventMask {
            value: 0x0000_1FFF_FFFF_FFFF,
        }
    }

    #[must_use]
    pub(crate) fn clear(self) -> Self {
        Self { value: 0 }
    }

    pub(crate) fn encode(&self) -> Result<[u8; 8], UtilsError> {
        let mut buffer = [0; 8];
        encode_le_u64(&mut buffer[..], self.value)?;
        Ok(buffer)
    }
}

macro_rules! event_mask {
    (
        $(
            $(#[$docs:meta])*
            $field:ident = $bit:expr,
        )+
    ) => {
        impl EventMask {
            $(
                #[must_use]
                pub(crate) fn $field(self, value: bool) -> Self {
                    if value {
                        Self { value: self.value | (1 << $bit) }
                    } else {
                        Self { value: self.value & !(1 << $bit) }
                    }
                }
            )+
        }
    };
}

event_mask! {
    _inquiry_complete = 0,
    _inquiry_result = 1,
    _connection_complete = 2,
    _connection_request = 3,
    disconnection_complete = 4,
    _authentication_complete = 5,
    _remote_name_request_complete = 6,
    encryption_change = 7,
    _change_connection_link_key_complete = 8,
    _master_link_key_complete = 9,
    _read_remote_supported_features_complete = 10,
    read_remote_version_information_complete = 11,
    _qos_setup_complete = 12,
    hardware_error = 15,
    _flush_occured = 16,
    _role_change = 17,
    _mode_change = 19,
    _return_link_keys = 20,
    _pin_code_request = 21,
    _link_key_request = 22,
    _link_key_notification = 23,
    _loopback_command = 24,
    data_buffer_overflow = 25,
    _max_slots_change = 26,
    _read_clock_offset_complete = 27,
    _connection_packet_type_changed = 28,
    _qos_violation = 29,
    _page_scan_mode_change = 30,
    _page_scan_repetition_mode_change = 31,
    _flow_specification_complete = 32,
    _inquiry_result_with_rssi = 33,
    _read_remote_extended_features_complete = 34,
    _synchronous_connection_complete = 43,
    _synchronous_connection_changed = 44,
    _sniff_subrating = 45,
    _extended_inquiry_result = 46,
    encryption_key_refresh_complete = 47,
    _io_capability_request = 48,
    _io_capability_request_reply = 49,
    _user_confirmation_request = 50,
    _user_passkey_request = 51,
    _remote_oob_data_request = 52,
    _simple_pairing_complete = 53,
    _link_supervision_timeout_changed = 55,
    _enhanced_flush_complete = 56,
    _user_passkey_notification = 58,
    _keypress_notification = 59,
    _remote_host_supported_features_notification = 60,
    le_meta_event = 61,
}
