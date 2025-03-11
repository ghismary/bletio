use bitflags::Flags;
use bletio_utils::{bitflags_array, BitFlagsArray};

bitflags_array! {
    /// LE features supported by the Link Layer.
    ///
    /// These features are defined in
    /// [Core Specification 6.0, Vol. 6, Part B, 4.6](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/low-energy-controller/link-layer-specification.html#UUID-25d414b5-8c50-cd46-fd17-80f0f816f354).
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct SupportedLeFeatures: 9 {
        /// The controller supports encryption on all the logical transports and:
        ///  - LL_ENC_REQ
        ///  - LL_ENC_RSP
        ///  - LL_START_ENC_REQ
        ///  - LL_START_ENC_RSP
        ///  - LL_PAUSE_ENC_REQ
        ///  - LL_PAUSE_ENC_RSP
        ///  - Encryption start procedure
        ///  - Encryption pause procedure
        const LE_ENCRYPTION = (0, 0);
        /// The controller supports the Extended Reject Indication feature and:
        ///  - LL_CONNECTION_PARAM_REQ
        ///  - LL_CONNECTION_PARAM_RSP
        ///  - Connection Parameters Request Procedure
        const CONNECTION_PARAMETERS_REQUEST_PROCEDURE = (0, 1);
        /// The controller supports LL_REJECT_EXT_IND.
        const EXTENDED_REJECT_INDICATION = (0, 2);
        /// The controller supports:
        ///  - LL_PERIPHERAL_FEATURE_REQ
        ///  - Receiving LL_FEATURE_RSP
        const PERIPHERAL_INITIATED_FEATURES_EXCHANGE = (0, 3);
        /// The controller supports:
        ///  - LL_PING_REQ
        ///  - LL_PING_RSP
        ///  - LE Ping procedure
        ///  - LE Authenticated Payload Timeout
        const LE_PING = (0, 4);
        /// The controller supports:
        ///  - LL_LENGTH_REQ and LL_LENGTH_RSP
        ///  - Data Length Update procedure
        const LE_DATA_PACKET_LENGTH_EXTENSION = (0, 5);
        /// The controller supports:
        ///  - Resolving List
        ///  - LL Privacy
        const LL_PRIVACY = (0, 6);
        /// The controller supports Extended Scanning Filter Policies.
        const EXTENDED_SCANNING_FILTER_POLICIES = (0, 7);
        /// The controller supports the Extended Reject Indication feature and:
        ///  - Transmission and reception using the 2M modulation scheme
        ///  - Longer preamble
        ///  - LL_PHY_REQ
        ///  - LL_PHY_RSP
        ///  - LL_PHY_UPDATE_IND
        ///  - PHY Update procedure
        const LE_2M_PHY = (1, 0);
        /// The controller transmitter supports Stable Modulation Index.
        const STABLE_MODULATION_INDEX_TRANSMITTER = (1, 1);
        /// The controller receiver supports Stable Modulation Index.
        const STABLE_MODULATION_INDEX_RECEIVER = (1, 2);
        /// The controller supports the Extended Reject Indication feature and:
        ///  - Transmission and reception using the Coded modulation scheme
        ///  - LL_PHY_REQ
        ///  - LL_PHY_RSP
        ///  - LL_PHY_UPDATE_IND
        ///  - PHY Update procedure
        ///  - Packet format for the LE Coded PHY
        ///  - Coding
        const LE_CODED_PHY = (1, 3);
        /// The controller supports the reception of an Advertising Physical Channel PDU payload of 255 octets, and:
        ///  - ADV_EXT_IND
        ///  - AUX_ADV_IND
        ///  - AUX_CHAIN_IND
        ///  - AUX_SCAN_REQ
        ///  - AUX_SCAN_RSP
        ///  - AUX_CONNECT_REQ
        ///  - AUX_CONNECT_RSP
        ///  - Common Extended Advertising Payload Format
        ///  - Advertising Sets
        ///  - Using AdvDataInfo (ADI)
        ///  - Advertising Sets
        ///  - Connect Requests on the Secondary Advertising Physical Channel
        ///  - Connectable Directed event type using ADV_EXT_IND
        ///  - Scannable Undirected event type using ADV_EXT_IND
        ///  - Connectable Undirected event type
        ///  - Scannable Directed event type
        ///  - Non-Connectable and Non-Scannable Directed event type
        /// If the controller supports connections it shall also support the Channel Selection Algorithm #2 feature
        const LE_EXTENDED_ADVERTISING = (1, 4);
        /// The controller supports the LE Extended Advertising feature, Channel Selection Algorithm #2 feature, and:
        ///  - AUX_SYNC_IND
        ///  - Periodic Advertising
        /// A Controller that supports Scanning state shall also support:
        ///  - Scanning for periodic advertisements
        ///  - Synchronization state for periodic advertising trains
        const LE_PERIODIC_ADVERTISING = (1, 5);
        /// The controller supports:
        ///  - ChSel bit set to 1
        ///  - Channel Selection Algorithm #2
        const CHANNEL_SELECTION_ALGORITHM_NO2 = (1, 6);
        /// The controller supports LE Power Class 1: 100 mW (+20 dBm) ≥ Pmax > 10 mW (+10 dBm)
        const LE_POWER_CLASS_1 = (1, 7);
        /// The controller supports:
        ///  - LL_MIN_USED_CHANNELS_IND
        ///  - Minimum Number Of Used Channels procedure
        const MINIMUM_NUMBER_OF_USED_CHANNELS_PROCEDURE = (2, 0);
        /// The controller supports the Receiving Constant Tone Extensions feature, the Extended Reject Indication feature,
        /// and the following on all supported PHYs that allow Constant Tone Extensions:
        ///  - LL_CTE_REQ
        ///  - LL_CTE_RSP
        ///  - Constant Tone Extension Request procedure, as initiator
        const CONNECTION_CTE_REQUEST = (2, 1);
        /// The controller supports the Extended Reject Indication feature and the following on all supported PHYs that allow Constant Tone Extensions:
        ///  - LL_CTE_REQ
        ///  - LL_CTE_RSP
        ///  - Transmitting Constant Tone Extensions
        ///  - Constant Tone Extension Request procedure, as responder
        const CONNECTION_CTE_RESPONSE = (2, 2);
        /// The controller supports the LE Periodic Advertising feature in Advertising state and the
        /// Transmitting Constant Tone Extensions on all supported PHYs that allow Constant Tone Extensions.
        const CONNECTIONLESS_CTE_TRANSMITTER = (2, 3);
        /// The controller supports the LE Periodic Advertising feature in Synchronization state and the following
        /// on all supported PHYs that allow Constant Tone Extensions:
        ///  - Receiving Advertising Physical Channel PDUs containing a CTEInfo field in the Extended Header
        ///  - IQ Sampling
        const CONNECTIONLESS_CTE_RECEIVER = (2, 4);
        /// The controller supports the following on all supported PHYs that allow Constant Tone Extensions:
        ///  - Transmitting Constant Tone Extensions
        ///  - Antenna Switching
        const ANTENNA_SWITCHING_DURING_CTE_TRANSMISSION = (2, 5);
        /// The controller supports the Receiving Constant Tone Extensions feature and
        /// Antenna Switching on all supported PHYs that allow Constant Tone Extensions.
        const ANTENNA_SWITCHING_DURING_CTE_RECEPTION = (2, 6);
        /// The controller supports the following on all supported PHYs that allow Constant Tone Extensions:
        ///  - Receiving Data Channel PDUs with the CP bit set to 1 and containing a CTEInfo field
        ///  - IQ Sampling
        const RECEIVING_CONSTANT_TONES_EXTENSIONS = (2, 7);
        /// The controller supports the LE Periodic Advertising feature and:
        ///  - LL_PERIODIC_SYNC_IND
        ///  - Periodic Advertising Sync Transfer procedure, as initiator
        const PERIODIC_ADVERTISING_SYNC_TRANSFER_SENDER = (3, 0);
        /// The controller supports the LE Periodic Advertising feature and:
        ///  - LL_PERIODIC_SYNC_IND
        ///  - Periodic Advertising Sync Transfer procedure, as recipient
        const PERIODIC_ADVERTISING_SYNC_TRANSFER_RECIPIENT = (3, 1);
        /// The controller supports:
        ///  - LL_CLOCK_ACCURACY_REQ and LL_CLOCK_ACCURACY_RSP
        ///  - Sleep Clock Accuracy Update procedure
        const SLEEP_CLOCK_ACCURACY_UPDATES = (3, 2);
        /// The controller validates the remote public key sent by the Host.
        const REMOTE_PUBLIC_KEY_VALIDATION = (3, 3);
        /// The controller supports the Channel Selection Algorithm #2 feature, the Sleep Clock Accuracy
        /// Updates feature, the Extended Reject Indication feature, and:
        ///  - LL_CIS_REQ
        ///  - LL_CIS_RSP
        ///  - LL_CIS_IND
        ///  - LL_CIS_TERMINATE_IND
        ///  - Connected Isochronous PDU
        ///  - Connected Isochronous Stream
        ///  - Connected Isochronous Group
        ///  - Connected Isochronous Stream Creation procedure
        ///  - Connected Isochronous Stream Termination procedure
        ///  - ISO Transmit Test Mode
        ///  - ISO Receive Test Mode
        ///  - Isochronous Adaptation Layer (ISOAL)
        const CONNECTED_ISOCHRONOUS_STREAM_CENTRAL = (3, 4);
        /// The controller supports the Channel Selection Algorithm #2 feature, the Sleep Clock Accuracy
        /// Updates feature, the Extended Reject Indication feature, and:
        ///  - LL_CIS_REQ
        ///  - LL_CIS_RSP
        ///  - LL_CIS_IND
        ///  - LL_CIS_TERMINATE_IND
        ///  - Connected Isochronous PDU
        ///  - Connected Isochronous Stream
        ///  - Connected Isochronous Group
        ///  - Connected Isochronous Stream Creation procedure
        ///  - Connected Isochronous Stream Termination procedure
        ///  - ISO Transmit Test Mode
        ///  - ISO Receive Test Mode
        ///  - Isochronous Adaptation Layer (ISOAL)
        const CONNECTED_ISOCHRONOUS_STREAM_PERIPHERAL = (3, 5);
        /// The controller supports the LE Periodic Advertising feature and:
        ///  - Broadcast Isochronous PDU
        ///  - BIG Control PDU
        ///  - Isochronous Broadcasting State
        ///  - BIG control procedures
        ///  - ISO Transmit Test Mode
        ///  - Isochronous Adaptation Layer (ISOAL)
        const ISOCHRONOUS_BROADCASTER = (3, 6);
        /// The controller supports the LE Periodic Advertising feature and:
        ///  - Broadcast Isochronous PDU
        ///  - BIG Control PDU
        ///  - Synchronization state
        ///  - BIG control procedures
        ///  - ISO Receive Test Mode
        ///  - Isochronous Adaptation Layer (ISOAL)
        const SYNCHRONIZED_RECEIVER = (3, 7);
        /// The host supports creating Connected Isochronous Stream.
        const CONNECTED_ISOCHRONOUS_STREAM_HOST_SUPPORT = (4, 0);
        /// The controller supports the Extended Reject Indication feature and:
        ///  - LL_POWER_CONTROL_REQ
        ///  - LL_POWER_CONTROL_RSP
        ///  - LL_POWER_CHANGE_IND
        ///  - Power level management
        ///  - Power Control Request procedure
        ///  - Power Change Indication procedure
        const LE_POWER_CONTROL_REQUEST = (4, 1);
        /// The controller supports the Extended Reject Indication feature and:
        ///  - LL_POWER_CONTROL_REQ
        ///  - LL_POWER_CONTROL_RSP
        ///  - LL_POWER_CHANGE_IND
        ///  - Power level management
        ///  - Power Control Request procedure
        ///  - Power Change Indication procedure
        const LE_POWER_CONTROL_REQUEST_BIS = (4, 2);
        /// The controller supports LE Path Loss Monitoring.
        const LE_PATH_LOSS_MONITORING = (4, 3);
        /// The controller supports the LE Periodic Advertising feature and:
        ///  - AUX_SYNC_IND PDU
        ///  - Periodic Advertising
        ///  - Periodic Advertising Trains
        const PERIODIC_ADVERTISING_ADI_SUPPORT = (4, 4);
        /// The controller supports all valid values for connSubrateFactor, and:
        ///  - LL_SUBRATE_REQ
        ///  - LL_SUBRATE_IND
        ///  - Connection Subrate Update procedure
        ///  - Connection Subrate Request procedure
        const CONNECTION_SUBRATING = (4, 5);
        /// The Host supports Connection Subrating.
        const CONNECTION_SUBRATING_HOST_SUPPORT = (4, 6);
        /// The controller supports:
        ///  - LL_CHANNEL_REPORTING_IND
        ///  - LL_CHANNEL_STATUS_IND
        ///  - Channel Classification Enable procedure
        ///  - Channel Classification Reporting procedure
        const CHANNEL_CLASSIFICATION = (4, 7);
        /// The controller supports the LE Extended Advertising and LE Coded PHY features and:
        ///  - Host selection of the coding scheme used in advertising
        ///  - Advertising reports specifying the coding scheme used
        const ADVERTISING_CODING_SELECTION = (5, 0);
        /// The Host supports Advertising Coding Selection.
        const ADVERTISING_CODING_SELECTION_HOST_SUPPORT = (5, 1);
        /// The controller supports the LE Extended Advertising feature and:
        ///  - ADV_DECISION_IND
        ///  - Decision scanning filter policies
        ///  - Decision PDU scanning
        const DECISION_BASED_ADVERTISING_FILTERING = (5, 2);
        /// The controller supports the LE Periodic Advertising feature in the Advertising state,
        /// the Periodic Advertising Sync Transfer - Sender feature, and:
        ///  - AUX_SYNC_SUBEVENT_IND
        ///  - AUX_SYNC_SUBEVENT_RSP
        ///  - LL_PERIODIC_SYNC_WR_IND
        ///  - Trains with responses
        const PERIODIC_ADVERTISING_WITH_RESPONSES_ADVERTISER = (5, 3);
        /// The controller supports the LE Periodic Advertising feature in the Scanning state,
        /// the Periodic Advertising Sync Transfer - Recipient feature, and:
        ///  - AUX_SYNC_SUBEVENT_IND
        ///  - AUX_SYNC_SUBEVENT_RSP
        ///  - LL_PERIODIC_SYNC_WR_IND
        ///  - Scanning for periodic advertisement
        ///  - Trains with responses
        const PERIODIC_ADVERTISING_WITH_RESPONSES_SCANNER = (5, 4);
        /// The controller supports at least one of the following features:
        ///  - Connected Isochronous Stream - Central
        ///  - Connected Isochronous Stream - Peripheral
        ///  - Isochronous Broadcaster
        ///  - Synchronized Receive
        /// And it supports:
        ///  - Unsegmented mode for framed PDUs
        ///  - Unsegmented mode in SDU synchronization reference and transport latency using framed PDUs
        const UNSEGMENTED_FRAMED_MODE = (5, 5);
        /// The controller supports the Extended Reject Indication feature, the Peripheral-initiated Features Exchange feature, and:
        ///  - Frequency bands and channel arrangement
        ///  - Modulation spectrum
        ///  - Stable phase
        ///  - Antenna switching for Channel Sounding
        ///  - Phase measurements
        ///  - LL_CS_SEC_REQ
        ///  - LL_CS_SEC_RSP
        ///  - LL_CS_CAPABILITIES_REQ
        ///  - LL_CS_CAPABILITES_RSP
        ///  - LL_CS_CONFIG_REQ
        ///  - LL_CS_CONFIG_RSP
        ///  - LL_CS_REQ
        ///  - LL_CS_RSP
        ///  - LL_CS_IND
        ///  - LL_CS_TERMINATE_REQ
        ///  - LL_CS_TERMINATE_RSP
        ///  - LL_CS_FAE_REQ
        ///  - LL_CS_FAE_RSP
        ///  - LL_CS_CHANNEL_MAP_IND
        ///  - Minimum Channel Sounding subevent space
        ///  - Active clock accuracy
        ///  - Window widening
        ///  - Channel Sounding
        ///  - Channel Sounding (Host Support)
        ///  - Channel Sounding Security Start procedure
        ///  - Channel Sounding Capabilities Exchange procedure
        ///  - Channel Sounding Configuration procedure
        ///  - Channel Sounding Start procedure
        ///  - Channel Sounding Procedure Repeat Termination procedure
        ///  - Channel Sounding Channel Map Update procedure
        ///  - Channel Sounding Mode-0 FAE Table Request procedure
        const CHANNEL_SOUNDING = (5, 6);
        /// The Host supports the Channel Sounding feature.
        const CHANELL_SOUNDING_HOST_SUPPORT = (5, 7);
        /// The controller supports the Channel Sounding feature and Phase measurements during T_PM.
        const CHANNEL_SOUNDING_TONE_QUALITY_INDICATION = (6, 0);
        /// The controller supports the Peripheral-initiated Features Exchange feature and:
        ///  - LL_FEATURE_EXT_REQ and LL_FEATURE_EXT_RSP
        ///  - Feature Page Exchange procedure
        const LL_EXTENDED_FEATURE_SET = (7, 7);
        /// The controller supports the Monitoring Advertisers.
        const MONITORING_ADVERTISERS = (8, 0);
        /// The controller supports a minimum frame space value that is less than or equal to 145 µs
        /// or maximum frame space value that is greater than or equal to 155 µs (or both), and
        /// supports  the Extended Reject Indication feature, and:
        ///  - LL_FRAME_SPACE_REQ
        ///  - LL_FRAME_SPACE_RSP
        ///  - Frame Space Update procedure
        const FRAME_SPACE_UPDATE = (8, 1);
    }
}

impl From<&[u8]> for SupportedLeFeatures {
    fn from(value: &[u8]) -> Self {
        let mut buffer = [0u8; 9];
        let len = if value.len() > 9 { 9 } else { value.len() };
        buffer[..len].copy_from_slice(&value[..len]);
        SupportedLeFeatures::from_bits_retain(BitFlagsArray(buffer))
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(SupportedLeFeatures::default(), &[0, 0, 0, 0, 0, 0, 0, 0, 0])]
    #[case(SupportedLeFeatures::empty(), &[0, 0, 0, 0, 0, 0, 0, 0, 0])]
    #[case(SupportedLeFeatures::LL_PRIVACY, &[64, 0, 0, 0, 0, 0, 0, 0, 0])]
    #[case(SupportedLeFeatures::LL_EXTENDED_FEATURE_SET | SupportedLeFeatures::MONITORING_ADVERTISERS, &[0, 0, 0, 0, 0, 0, 0, 128, 1])]
    fn test_supported_le_features(
        #[case] input: SupportedLeFeatures,
        #[case] expected_data: &[u8; 9],
    ) {
        assert_eq!(&input.bits().0, expected_data);
    }
}
