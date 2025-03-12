#[cfg(not(feature = "defmt"))]
use bitflags::bitflags;
#[cfg(feature = "defmt")]
use defmt::bitflags;

use bletio_utils::EncodeToBuffer;

bitflags! {
    /// HCI LE event mask.
    ///
    /// The values are defined in
    /// [Core Specification 6.0, Vol. 4, Part E, 7.8.1](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-cefc532c-3752-3f40-b5c1-91070b4dfef8).
    #[cfg_attr(not(feature = "defmt"), derive(Debug, Clone, Copy, PartialEq, Eq))]
    pub struct LeEventMask: u64 {
        /// Indicates to both of the Hosts forming the connection that a new connection has been created.
        const LE_CONNECTION_COMPLETE = 1 << 0;
        /// One or more Bluetooth devices have responded to an active scan or have broadcast advertisements that were received during a passive scan.
        const LE_ADVERTISING_REPORT = 1 << 1;
        /// Indicates that the Connection Update procedure has completed.
        const LE_CONNECTION_UPDATE_COMPLETE = 1 << 2;
        /// Indicates the completion of the process of the Controller obtaining page 0 of the features used on the connection
        /// and the features supported by the remote Bluetooth device.
        const LE_READ_REMOTE_FEATURES_PAGE_0_COMPLETE = 1 << 3;
        /// Indicates that the peer device, in the Central role, is attempting to encrypt or re-encrypt the link and is requesting
        /// the Long Term Key from the Host.
        const LE_LONG_TERM_KEY_REQUEST = 1 << 4;
        /// Indicates to the Central’s Host or the Peripheral’s Host that the remote device is requesting a change in the
        /// connection parameters using the Connection Update procedure.
        const LE_REMOTE_CONNECTION_PARAMETER_REQUEST = 1 << 5;
        /// Notifies the Host of a change to either the maximum LL Data PDU Payload length or the maximum transmission time
        /// of packets containing LL Data PDUs in either direction.
        const LE_DATA_LENGTH_CHANGE = 1 << 6;
        /// The local P-256 key generation is complete.
        const LE_READ_LOCAL_P256_PUBLIC_KEY_COMPLETE = 1 << 7;
        /// Indicates that LE Diffie-Hellman key generation has been completed by the Controller.
        const LE_GENERATE_DHKEY_COMPLETE = 1 << 8;
        /// Indicates to both of the Hosts forming the connection that a new connection has been created.
        const LE_ENHANCED_CONNECTION_COMPLETE = 1 << 9;
        /// Indicates that directed advertisements have been received.
        const LE_DIRECTED_AVERTISING_REPORT = 1 << 10;
        /// Indicate that the Controller has changed the transmitter PHY or receiver PHY in use.
        const LE_PHY_UPDATE_COMPLETE = 1 << 11;
        /// Indicates that one or more Bluetooth devices have responded to an active scan or have broadcast advertisements
        /// that were received during a passive scan.
        const LE_EXTENDED_ADVERTISING_REPORT = 1 << 12;
        /// Indicates that the Controller has received the first periodic advertising packet from an advertiser.
        const LE_PERIODIC_ADVERTISING_SYNC_ESTABLISHED = 1 << 13;
        /// Indicates that the Controller has received a periodic advertisement or has failed to receive an AUX_SYNC_SUBEVENT_IND PDU.
        const LE_PERIODIC_ADVERTISING_REPORT = 1 << 14;
        /// Indicates that the Controller has not received a Periodic Advertising packet from a train within the timeout period.
        const LE_PERIODIC_ADVERTISING_SYNC_LOST = 1 << 15;
        /// Indicates that scanning has ended because the duration has expired.
        const LE_SCAN_TIMEOUT = 1 << 16;
        /// Indicates that the Controller has terminated advertising in the specified advertising sets.
        const LE_ADVERTISING_SET_TERMINATED = 1 << 17;
        /// Indicates that a SCAN_REQ PDU or an AUX_SCAN_REQ PDU has been received by the advertiser.
        const LE_SCAN_REQUEST_RECEIVED = 1 << 18;
        /// Indicates which channel selection algorithm is used on a data physical channel connection.
        const LE_CHANNEL_SELECTION_ALGORITHM = 1 << 19;
        /// The Controller reports IQ information from the Constant Tone Extension of a received advertising packet forming
        /// part of the periodic advertising train.
        const LE_CONNECTIONLESS_IQ_REPORT = 1 << 20;
        /// The Controller reports the IQ samples from the Constant Tone Extension of a received packet.
        const LE_CONNECTION_IQ_REPORT = 1 << 21;
        /// The Controller reports an issue following a request to a peer device to reply with a packet containing an
        /// LL_CTE_RSP PDU and a Constant Tone Extension.
        const LE_CTE_REQUEST_FAILED = 1 << 22;
        /// The Controller reports that it has received periodic advertising synchronization information from a device
        /// and either successfully synchronized to the periodic advertising train or timed out while attempting to synchronize.
        const LE_PERIODIC_ADVERTISING_SYNC_TRANSFER_RECEIVED = 1 << 23;
        /// Indicates that a CIS has been established, was considered lost before being established, or (on the Central) was rejected by the Peripheral.
        const LE_CIS_ESTABLISHED = 1 << 24;
        /// Indicates that a Controller has received a request to establish a CIS.
        const LE_CIS_REQUEST = 1 << 25;
        /// Indicates that the HCI_LE_Create_BIG or HCI_LE_Create_BIG_Test command has completed and, if successful,
        /// the Link Layer has entered the Isochronous Broadcasting state.
        const LE_CREATE_BIG_COMPLETE = 1 << 26;
        /// Indicates that the transmission of all the BISes in the BIG are terminated.
        const LE_TERMINATE_BIG_COMPLETE = 1 << 27;
        /// Indicates that the HCI_LE_BIG_Create_Sync command has completed.
        const LE_BIG_SYNC_ESTABLISHED = 1 << 28;
        /// Indicates that the Controller has not received any PDUs on a BIG within the timeout period
        /// or the BIG has been terminated by the remote device.
        const LE_BIG_SYNC_LOST = 1 << 29;
        /// Indicates that the HCI_LE_Request_Peer_SCA command has been completed.
        const LE_REQUEST_PEER_SCA_COMPLETE = 1 << 30;
        /// Reports a path loss threshold crossing.
        const LE_PATH_LOST_THRESHOLD = 1 << 31;
        /// Reports the transmit power level on the ACL connection.
        const LE_TRANSMIT_POWER_REPORTING = 1 << 32;
        /// Indicates that the Controller has received an Advertising PDU that contained a BIGInfo field.
        const LE_BIGINFO_ADVERTISING_REPORT = 1 << 33;
        /// Indicate that a Connection Subrate Update procedure has completed and some parameters of the specified connection have changed.
        const LE_SUBRATE_CHANGE = 1 << 34;
        /// Indicates that the Controller has received the first periodic advertising packet from an advertiser.
        const LE_PERIODIC_ADVERTISING_SYNC_ESTABLISHED_V2 = 1 << 35;
        /// Indicates that the Controller has received a periodic advertisement or has failed to receive an AUX_SYNC_SUBEVENT_IND PDU.
        const LE_PERIODIC_ADVERTISING_REPORT_V2 = 1 << 36;
        /// The Controller reports that it has received periodic advertising synchronization information from a device
        /// and either successfully synchronized to the periodic advertising train or timed out while attempting to synchronize.
        const LE_PERIODIC_ADVERTISING_SYNC_TRANSFER_RECEIVED_V2 = 1 << 37;
        /// The Controller indicates that it is ready to transmit one or more subevents and is requesting the advertising data for these subevents.
        const LE_PERIODIC_ADVERTISING_SUBEVENT_DATA_REQUEST = 1 << 38;
        /// Indicates that one or more Bluetooth devices have responded to a periodic advertising subevent during a PAwR train.
        const LE_PERIODIC_ADVERTISING_RESPONSE_REPORT = 1 << 39;
        /// Indicates to both of the Hosts forming the connection that a new connection has been created.
        const LE_ENHANCED_CONNECTION_COMPLETE_V2 = 1 << 40;
        /// Indicates that a CIS has been established, was considered lost before being established, or (on the Central) was rejected by the Peripheral.
        const LE_CIS_ESTABLISHED_V2 = 1 << 41;
        /// Indicates the completion of the process of the Controller obtaining the features supported by a remote Bluetooth device.
        const LE_READ_ALL_REMOTE_FEATURES_COMPLETE = 1 << 42;
        /// Generated when a locally initiated CS Capabilities Exchange procedure has completed or when the local Controller
        /// has received an LL_CS_CAPABILITIES_REQ from the remote Controller.
        const LE_CS_READ_REMOTE_SUPPORTED_CAPABILITIES_COMPLETE = 1 << 43;
        /// Generated when a locally initiated CS Mode-0 Frequency Actuation Error Table Update procedure has completed.
        const LE_CS_READ_REMOTE_FAE_TABLE_COMPLETE = 1 << 44;
        /// Generated when a locally initiated CS Security Start procedure has completed or when the local Controller
        /// has responded to a CS security request from the remote Controller.
        const LE_CS_SECURITY_ENABLE_COMPLETE = 1 << 45;
        /// Generated when a locally initiated Channel Sounding Configuration procedure has completed or when the local Controller has
        /// responded to a CS configuration request from the remote Controller or when a CS configuration is created only with local context.
        const LE_CS_CONFIG_COMPLETE = 1 << 46;
        /// Generated when the local or remote Controller has scheduled a new CS procedure measurement or disabled an ongoing CS procedure
        /// measurement as a result of an HCI_LE_CS_Procedure_Enable command.
        const LE_CS_PROCEDURE_ENABLE_COMPLETE = 1 << 47;
        /// Generated when the local Controller has results to report for a CS subevent during the CS procedure.
        const LE_CS_SUBEVENT_RESULT = 1 << 48;
        /// Generated after the local Controller has completed a new CS subevent measurement and has already sent an
        /// HCI_LE_CS_Subevent_Result event for the specified CS subevent.
        const LE_CS_SUBEVENT_RESULT_CONTINUE = 1 << 49;
        /// Generated when the local Controller has stopped an ongoing CS test as a result of the HCI_LE_CS_Test_End command.
        const LE_CS_TEST_END_COMPLETE = 1 << 50;
        /// Indicates that an advertiser on the Monitored Advertisers List has met an RSSI threshold condition established
        /// by the HCI_LE_Add_Device_To_Monitored_Advertisers_List command.
        const LE_MONITORED_ADVERTISERS_REPORT = 1 << 51;
        /// Indicates that the Frame Space Update procedure has completed and, if initiated autonomously by the local Controller
        /// or the peer device, that at least one frame space value has changed.
        const LE_FRAME_SPACE_UPDATE_COMPLETE = 1 << 52;
    }
}

impl EncodeToBuffer for LeEventMask {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.encode_le_u64(self.bits())
    }

    fn encoded_size(&self) -> usize {
        8
    }
}

impl Default for LeEventMask {
    fn default() -> Self {
        Self::from_bits_truncate(0x0000_0000_0000_001F)
    }
}

pub(crate) mod parser {
    use nom::{
        combinator::{all_consuming, map},
        number::complete::le_u64,
        IResult, Parser,
    };

    use super::*;

    pub(crate) fn le_event_mask(input: &[u8]) -> IResult<&[u8], LeEventMask> {
        all_consuming(map(le_u64, LeEventMask::from_bits_truncate)).parse(input)
    }
}

#[cfg(test)]
mod test {
    use bletio_utils::{Buffer, BufferOps};
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_eventmask_default() {
        let value = LeEventMask::default();
        assert_eq!(value.bits(), 0x0000_0000_0000_001F);
    }

    #[rstest]
    #[case(LeEventMask::LE_CONNECTION_COMPLETE, &[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])]
    #[case(LeEventMask::LE_CONNECTION_UPDATE_COMPLETE, &[0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])]
    #[case(
        LeEventMask::LE_CONNECTION_COMPLETE | LeEventMask::LE_CONNECTION_UPDATE_COMPLETE,
        &[0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    )]
    fn test_le_event_mask_encoding(
        #[case] le_event_mask: LeEventMask,
        #[case] encoded_data: &[u8],
    ) -> Result<(), bletio_utils::Error> {
        let mut buffer = Buffer::<8>::default();
        assert_eq!(le_event_mask.encoded_size(), encoded_data.len());
        le_event_mask.encode(&mut buffer)?;
        assert_eq!(buffer.data(), encoded_data);
        Ok(())
    }

    #[rstest]
    #[case(&[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], LeEventMask::LE_CONNECTION_COMPLETE)]
    #[case(&[0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], LeEventMask::LE_CONNECTION_UPDATE_COMPLETE)]
    #[case(
        &[0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        LeEventMask::LE_CONNECTION_COMPLETE | LeEventMask::LE_CONNECTION_UPDATE_COMPLETE
    )]
    fn test_le_event_mask_parsing(
        #[case] input: &[u8],
        #[case] expected_le_event_mask: LeEventMask,
    ) {
        let (rest, le_event_mask) = parser::le_event_mask(input).unwrap();
        assert!(rest.is_empty());
        assert_eq!(le_event_mask, expected_le_event_mask);
    }
}
