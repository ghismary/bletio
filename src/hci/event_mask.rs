use crate::utils::encode_le_u64;
use crate::Error;

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

    pub(crate) fn encode(&self) -> Result<[u8; 8], Error> {
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
    inquiry_complete = 0,
    inquiry_result = 1,
    connection_complete = 2,
    connection_request = 3,
    disconnection_complete = 4,
    authentication_complete = 5,
    remote_name_request_complete = 6,
    encryption_change = 7,
    change_connection_link_key_complete = 8,
    master_link_key_complete = 9,
    read_remote_supported_features_complete = 10,
    read_remote_version_information_complete = 11,
    qos_setup_complete = 12,
    hardware_error = 15,
    flush_occured = 16,
    role_change = 17,
    mode_change = 19,
    return_link_keys = 20,
    pin_code_request = 21,
    link_key_request = 22,
    link_key_notification = 23,
    loopback_command = 24,
    data_buffer_overflow = 25,
    max_slots_change = 26,
    read_clock_offset_complete = 27,
    connection_packet_type_changed = 28,
    qos_violation = 29,
    page_scan_mode_change = 30,
    page_scan_repetition_mode_change = 31,
    flow_specification_complete = 32,
    inquiry_result_with_rssi = 33,
    read_remote_extended_features_complete = 34,
    synchronous_connection_complete = 43,
    synchronous_connection_changed = 44,
    sniff_subrating = 45,
    extended_inquiry_result = 46,
    encryption_key_refresh_complete = 47,
    io_capability_request = 48,
    io_capability_request_reply = 49,
    user_confirmation_request = 50,
    user_passkey_request = 51,
    remote_oob_data_request = 52,
    simple_pairing_complete = 53,
    link_supervision_timeout_changed = 55,
    enhanced_flush_complete = 56,
    user_passkey_notification = 58,
    keypress_notification = 59,
    remote_host_supported_features_notification = 60,
    le_meta_event = 61,
}
