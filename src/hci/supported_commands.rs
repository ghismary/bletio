#[derive(Debug)]
pub struct SupportedCommands {
    value: [u8; 64],
}

impl Default for SupportedCommands {
    fn default() -> Self {
        Self { value: [0; 64] }
    }
}

impl From<[u8; 64]> for SupportedCommands {
    fn from(value: [u8; 64]) -> Self {
        Self { value }
    }
}

macro_rules! supported_commands {
    (
        $(
            $(#[$docs:meta])*
            ($func:ident, $byte:expr, $bit:expr),
        )+
    ) => {
        impl SupportedCommands {
            $(
                pub fn $func(&self) -> bool {
                    self.value[$byte] & (1 << $bit) != 0
                }
            )+
        }
    };
}

supported_commands! {
    (has_read_bd_addr, 15, 1),
    (has_le_set_event_mask, 25, 0),
    (has_le_read_buffer_size, 25, 1),
    (has_le_read_local_supported_features, 25, 2),
    (has_le_set_random_address, 25, 4),
    (has_le_set_advertising_parameters, 25, 5),
    (has_le_read_advertising_channel_tx_power, 25, 6),
    (has_le_set_advertising_data, 25, 7),
    (has_le_set_scan_response_data, 26, 0),
    (has_le_set_advertise_enable, 26, 1),
    (has_le_set_scan_parameters, 26, 2),
    (has_le_set_scan_enable, 26, 3),
    (has_le_create_connection, 26, 4),
    (has_le_create_connection_cancel, 26, 5),
    (has_le_read_white_list_size, 26, 6),
    (has_le_clear_white_list, 26, 7),
    (has_le_add_device_to_white_list, 27, 0),
    (has_le_remove_device_from_white_list, 27, 1),
    (has_le_connection_update, 27, 2),
    (has_le_set_host_channel_qualification, 27, 3),
    (has_le_read_channel_map, 27, 4),
    (has_le_read_remote_used_features, 27, 5),
    (has_le_encrypt, 27, 6),
    (has_le_rand, 27, 7),
    (has_le_start_encryption, 28, 0),
    (has_le_long_term_key_request_reply, 28, 1),
    (has_le_long_term_key_request_negative_reply, 28, 2),
    (has_le_read_supported_states, 28, 3),
    (has_le_receiver_test, 28, 4),
    (has_le_transmitter_test, 28, 5),
    (has_le_test_end, 28, 6),
    (has_le_remote_connection_parameter_request_reply, 33, 4),
    (has_le_remote_connection_parameter_request_negative_reply, 33, 5),
    (has_le_set_data_length, 33, 6),
    (has_le_read_suggested_default_data_length, 33, 7),
    (has_le_write_suggested_default_data_length, 34, 0),
    (has_le_read_local_p256_public_key, 34, 1),
    (has_le_generate_dh_key, 34, 2),
    (has_le_add_device_to_resolving_list, 34, 3),
    (has_le_remove_device_from_resolving_list, 34, 4),
    (has_le_clear_resolving_list, 34, 5),
    (has_le_read_resolving_list_size, 34, 6),
    (has_le_read_peer_resolvable_address, 34, 7),
    (has_le_read_local_resolvable_address, 35, 0),
    (has_le_set_address_resolution_enable, 35, 1),
    (has_le_set_resolvable_private_address_timeout, 35, 2),
    (has_le_read_maximum_data_length, 35, 3),
}
