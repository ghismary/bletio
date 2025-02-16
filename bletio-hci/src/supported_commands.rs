use bitflags::Flags;
use bletio_utils::{bitflags_array, BitFlagsArray};

bitflags_array! {
    /// Supported HCI commands.
    ///
    /// These commands are defined in
    /// [Core Specification 6.0, Vol. 4, Part E, 6.27](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-d5f3af07-8495-3fe6-8afe-c6e6db371233).
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct SupportedCommands: 64 {
        const READ_BUFFER_SIZE = (14, 7);
        const READ_BD_ADDR = (15, 1);
        const LE_SET_EVENT_MASK = (25, 0);
        const LE_READ_BUFFER_SIZE = (25, 1);
        const LE_READ_LOCAL_SUPPORTED_FEATURES_PAGE_0 = (25, 2);
        const LE_SET_RANDOM_ADDRESS = (25, 4);
        const LE_SET_ADVERTISING_PARAMETERS = (25, 5);
        const LE_READ_ADVERTISING_PHYSICAL_CHANNEL_TX_POWER = (25, 6);
        const LE_SET_ADVERTISING_DATA = (25, 7);
        const LE_SET_SCAN_RESPONSE_DATA = (26, 0);
        const LE_SET_ADVERTISING_ENABLE = (26, 1);
        const LE_SET_SCAN_PARAMETERS = (26, 2);
        const LE_SET_SCAN_ENABLE = (26, 3);
        const LE_CREATE_CONNECTION = (26, 4);
        const LE_CREATE_CONNECTION_CANCEL = (26, 5);
        const LE_READ_FILTER_ACCEPT_LIST_SIZE = (26, 6);
        const LE_CLEAR_FILTER_ACCEPT_LIST = (26, 7);
        const LE_ADD_DEVICE_TO_FILTER_ACCEPT_LIST = (27, 0);
        const LE_REMOVE_DEVICE_FROM_FILTER_ACCEPT_LIST = (27, 1);
        const LE_CONNECTION_UPDATE = (27, 2);
        const LE_SET_HOST_CHANNEL_QUALIFICATION = (27, 3);
        const LE_READ_CHANNEL_MAP = (27, 4);
        const LE_READ_REMOTE_FEATURES_PAGE_0 = (27, 5);
        const LE_ENCRYPT = (27, 6);
        const LE_RAND = (27, 7);
        const LE_ENABLE_ENCRYPTION = (28, 0);
        const LE_LONG_TERM_KEY_REQUEST_REPLY = (28, 1);
        const LE_LONG_TERM_KEY_REQUEST_NEGATIVE_REPLY = (28, 2);
        const LE_READ_SUPPORTED_STATES = (28, 3);
        const LE_RECEIVER_TEST = (28, 4);
        const LE_TRANSMITTER_TEST = (28, 5);
        const LE_TEST_END = (28, 6);
        const LE_ENABLE_MONITORING_ADVERTISERS = (28, 7);
        const LE_REMOTE_CONNECTION_PARAMETER_REQUEST_REPLY = (33, 4);
        const LE_REMOTE_CONNECTION_PARAMETER_REQUEST_NEGATIVE_REPLY = (33, 5);
        const LE_SET_DATA_LENGTH = (33, 6);
        const LE_READ_SUGGESTED_DEFAULT_DATA_LENGTH = (33, 7);
        const LE_WRITE_SUGGESTED_DEFAULT_DATA_LENGTH = (34, 0);
        const LE_READ_LOCAL_P256_PUBLIC_KEY = (34, 1);
        const LE_GENERATE_DHKEY = (34, 2);
        const LE_ADD_DEVICE_TO_RESOLVING_LIST = (34, 3);
        const LE_REMOVE_DEVICE_FROM_RESOLVING_LIST = (34, 4);
        const LE_CLEAR_RESOLVING_LIST = (34, 5);
        const LE_READ_RESOLVING_LIST_SIZE = (34, 6);
        const LE_READ_PEER_RESOLVABLE_ADDRESS = (34, 7);
        const LE_READ_LOCAL_RESOLVABLE_ADDRESS = (35, 0);
        const LE_SET_ADDRESS_RESOLUTION_ENABLE = (35, 1);
        const LE_SET_RESOLVABLE_PRIVATE_ADDRESS_TIMEOUT = (35, 2);
        const LE_READ_MAXIMUM_DATA_LENGTH = (35, 3);
        const LE_READ_PH = (35, 4);
        const LE_SET_DEFAULT_PHY = (35, 5);
        const LE_SET_PHY = (35, 6);
        const LE_RECEIVER_TEST_V2 = (35, 7);
        const LE_TRANSMITTER_TEST_V2 = (36, 0);
        const LE_SET_ADVERTISING_SET_RANDOM_ADDRESS = (36, 1);
        const LE_SET_EXTENDED_ADVERTISING_PARAMETERS = (36, 2);
        const LE_SET_EXTENDED_ADVERTISING_DATA = (36, 3);
        const LE_SET_EXTENDED_SCAN_RESPONSE_DATA = (36, 4);
        const LE_SET_EXTENDED_ADVERTISING_ENABLE = (36, 5);
        const LE_READ_MAXIMUM_ADVERTISING_DATA_LENGTH = (36, 6);
        const LE_READ_NUMBER_OF_SUPPORTED_ADVERTISING_SETS = (36, 7);
        const LE_REMOVE_ADVERTISING_SET = (37, 0);
        const LE_CLEAR_ADVERTISING_SETS = (37, 1);
        const LE_SET_PERIODIC_ADVERTISING_PARAMETERS = (37, 2);
        const LE_SET_PERIODIC_ADVERTISING_DATA = (37, 3);
        const LE_SET_PERIODIC_ADVERTISING_ENABLE = (37, 4);
        const LE_SET_EXTENDED_SCAN_PARAMETERS = (37, 5);
        const LE_SET_EXTENDED_SCAN_ENABLE = (37, 6);
        const LE_EXTENDED_CREATE_CONNECTION = (37, 7);
        const LE_PERIODIC_ADVERTISING_CREATE_SYNC = (38, 0);
        const LE_PERIODIC_ADVERTISING_CREATE_SYNC_CANCEL = (38, 1);
        const LE_PERIODIC_ADVERTISING_TERMINATE_SYNC = (38, 2);
        const LE_ADD_DEVICE_TO_PERIODIC_ADVERTISER_LIST = (38, 3);
        const LE_REMOVE_DEVICE_FROM_PERIODIC_ADVERTISER_LIST = (38, 4);
        const LE_CLEAR_PERIODIC_ADVERTISER_LIST = (38, 5);
        const LE_READ_PERIODIC_ADVERTISER_LIST_SIZE = (38, 6);
        const LE_READ_TRANSMIT_POWER = (38, 7);
        const LE_READ_RF_PATH_COMPENSATION = (39, 0);
        const LE_WRITE_RF_PATH_COMPENSATION = (39, 1);
        const LE_SET_PRIVACY_MODE = (39, 2);
        const LE_RECEIVER_TEST_V3 = (39, 3);
        const LE_TRANSMITTER_TEST_V3 = (39, 4);
        const LE_SET_CONNECTIONLESS_CTE_TRANSMIT_PARAMETERS = (39, 5);
        const LE_SET_CONNECTIONLESS_CTE_TRANSMIT_ENABLE = (39, 6);
        const LE_SET_CONNECTIONLESS_IQ_SAMPLING_ENABLE = (39, 7);
        const LE_SET_CONNECTION_CTE_RECEIVE_PARAMETERS = (40, 0);
        const LE_SET_CONNECTION_CTE_TRANSMIT_PARAMETERS = (40, 1);
        const LE_CONNECTION_CTE_REQUEST_ENABLE = (40, 2);
        const LE_CONNECTION_CTE_RESPONSE_ENABLE = (40, 3);
        const LE_READ_ANTENNA_INFORMATION = (40, 4);
        const LE_SET_PERIODIC_ADVERTISING_RECEIVE_ENABLE = (40, 5);
        const LE_PERIODIC_ADVERTISING_SYNC_TRANSFER = (40, 6);
        const LE_PERIODIC_ADVERTISING_SET_INFO_TRANSFER = (40, 7);
        const LE_SET_PERIODIC_ADVERTISING_SYNC_TRANSFER_PARAMETERS = (41, 0);
        const LE_SET_DEFAULT_PERIODIC_ADVERTISING_SYNC_TRANSFER_PARAMETERS = (41, 1);
        const LE_GENERATE_DHKEY_V2 = (41, 2);
        const LE_MODIFY_SLEEP_CLOCK_ACCURACY = (41, 4);
        const LE_READ_BUFFER_SIZE_V2 = (41, 5);
        const LE_READ_ISO_TX_SYNC = (41, 6);
        const LE_SET_CIG_PARAMETERS = (41, 7);
        const LE_SET_CIG_PARAMETERS_TEST = (42, 0);
        const LE_CREATE_CIS = (42, 1);
        const LE_REMOVE_CIG = (42, 2);
        const LE_ACCEPT_CIS_REQUEST = (42, 3);
        const LE_REJECT_CIS_REQUEST = (42, 4);
        const LE_CREATE_BIG = (42, 5);
        const LE_CREATE_BIG_TEST = (42, 6);
        const LE_TERMINATE_BIG = (42, 7);
        const LE_BIG_CREATE_SYNC = (43, 0);
        const LE_BIG_TERMINATE_SYNC = (43, 1);
        const LE_REQUEST_PEER_SCA = (43, 2);
        const LE_SETUP_ISO_DATA_PATH = (43, 3);
        const LE_REMOVE_ISO_DATA_PATH = (43, 4);
        const LE_ISO_TRANSMIT_TEST = (43, 5);
        const LE_ISO_RECEIVE_TEST = (43, 6);
        const LE_ISO_READ_TEST_COUNTERS = (43, 7);
        const LE_ISO_TEST_END = (44, 0);
        const LE_SET_HOST_FEATURE = (44, 1);
        const LE_READ_ISO_LINK_QUALITY = (44, 2);
        const LE_ENHANCED_READ_TRANSMIT_POWER_LEVEL = (44, 3);
        const LE_READ_REMOTE_TRANSMIT_POWER_LEVEL = (44, 4);
        const LE_SET_PATH_LOSS_REPORTING_PARAMETERS = (44, 5);
        const LE_SET_PATH_LOSS_REPORTING_ENABLE = (44, 6);
        const LE_SET_TRANSMIT_POWER_REPORTING_ENABLE = (44, 7);
        const LE_TRANSMITTER_TEST_V4 = (45, 0);
        const LE_SET_DATA_RELATED_ADDRESS_CHANGES = (45, 6);
        const LE_SET_DEFAULT_SUBRATE = (46, 0);
        const LE_SUBRATE_REQUEST = (46, 1);
        const LE_SET_EXTENDED_ADVERTISING_PARAMETERS_V2 = (46, 2);
        const LE_SET_DECISION_DATA = (46, 3);
        const LE_SET_DECISION_INSTRUCTIONS = (46, 4);
        const LE_SET_PERIODIC_ADVERTISING_SUBEVENT_DATA = (46, 5);
        const LE_SET_PERIODIC_ADVERTISING_RESPONSE_DATA = (46, 6);
        const LE_SET_PERIODIC_SYNC_SUBEVENT = (46, 7);
        const LE_EXTENDED_CREATE_CONNECTION_V2 = (47, 0);
        const LE_SET_PERIODIC_ADVERTISING_PARAMETERS_V2 = (47, 1);
        const LE_READ_ALL_LOCAL_SUPPORTED_FEATURES = (47, 2);
        const LE_READ_ALL_REMOTE_FEATURES = (47, 3);
        const LE_SET_HOST_FEATURE_V2 = (47, 4);
        const LE_ADD_DEVICE_TO_MONITORED_ADVERTISERS_LIST = (47, 5);
        const LE_REMOVE_DEVICE_FROM_MONITORED_ADVERTISERS_LIST = (47, 6);
        const LE_CLEAR_MONITORED_ADVERTISERS_LIST = (47, 7);
        const LE_READ_MONITORED_ADVERTISERS_LIST_SIZE = (48, 0);
        const LE_FRAME_SPACE_UPDATE = (48, 1);
    }
}

impl From<[u8; 64]> for SupportedCommands {
    fn from(value: [u8; 64]) -> Self {
        SupportedCommands::from_bits_retain(BitFlagsArray(value))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_supported_commands() {
        assert_eq!(SupportedCommands::default(), SupportedCommands::empty());
        assert_eq!(
            SupportedCommands::LE_SET_DATA_LENGTH.bits(),
            BitFlagsArray::new(33, 6)
        );
        let value: SupportedCommands = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ]
        .into();
        assert_eq!(value, SupportedCommands::LE_SET_ADVERTISING_PARAMETERS)
    }
}
