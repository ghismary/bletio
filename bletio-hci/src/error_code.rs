use num_enum::TryFromPrimitive;

use crate::Error;

/// HCI error codes as defined in
/// [Core Specification 6.0, Vol.1, Part F](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/architecture,-change-history,-and-conventions/controller-error-codes.html).
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::InvalidErrorCode))]
#[repr(u8)]
#[non_exhaustive]
pub enum ErrorCode {
    /// Success.
    Success = 0x00,
    /// The Controller does not understand the HCI Command packet opcode that the Host sent.
    /// The opcode given might not correspond to any of the opcodes specified in the Core Specification,
    /// or any vendor-specific opcodes, or the command may have not been implemented.
    UnknownHciCommand = 0x01,
    /// A command was sent from the Host that should identify a connection, but that connection does not
    /// exist or does not identify the correct type of connection.
    UnknownConnectionIdentifier = 0x02,
    /// Something in the Controller has failed in a manner that cannot be described with any other error code.
    /// The meaning implied with this error code is implementation dependent.
    HardwareFailure = 0x03,
    /// A page timed out because of the Page Timeout configuration parameter.
    PageTimeout = 0x04,
    /// Pairing or authentication failed due to incorrect results in the pairing or authentication procedure.
    /// This could be due to an incorrect PIN or Link Key.
    AuthenticationFailure = 0x05,
    /// Pairing failed because of a missing PIN, or authentication failed because of a missing Key.
    PinOrKeyMissing = 0x06,
    /// The Controller has run out of memory to store new parameters.
    MemoryCapacityExceeded = 0x07,
    /// Either the link supervision timeout has expired for a given connection or the synchronization timeout
    /// has expired for a given broadcast.
    ConnectionTimeout = 0x08,
    /// An attempt to create another connection failed because the Controller is already at its limit of the
    /// number of connections it can support. The number of connections a device can support is implementation dependent.
    ConnectionLimitExceeded = 0x09,
    /// The Controller has reached the limit to the number of synchronous connections that can be achieved to a device.
    /// The number of synchronous connections a device can support is implementation dependent.
    SynchronousConnectionLimitToADeviceExceeded = 0x0A,
    /// An attempt was made to create a new Connection to a device when there is already a connection to this
    /// device and multiple connections to the same device are not permitted.
    AclConnectionAlreadyExists = 0x0B,
    /// The command requested cannot be executed because the Controller is in a state where it cannot process this
    /// command at this time.
    CommandDisallowed = 0x0C,
    /// A connection was rejected due to limited resources.
    ConnectionRejectedDueToLimitedResources = 0x0D,
    /// A connection was rejected due to security requirements not being fulfilled, like authentication or pairing.
    ConnectionRejectedDueToSecurityReasons = 0x0E,
    /// A connection was rejected because this device does not accept the BD_ADDR.
    /// This may be because the device will only accept connections from specific BD_ADDRs.
    ConnectionRejectedDueToUnacceptableBdAddr = 0x0F,
    /// The Connection Accept Timeout has been exceeded for this connection attempt.
    ConnectionAcceptTimeoutExceeded = 0x10,
    /// A feature or parameter value in the HCI command is not supported.
    UnsupportedFeatureOrParameterValue = 0x11,
    /// At least one of the HCI command parameters is invalid:
    ///  - the parameter total length is invalid.
    ///  - a command parameter is an invalid type.
    ///  - a connection identifier does not match the corresponding event.
    ///  - a parameter is odd when it is required to be even.
    ///  - a parameter is outside of the specified range.
    ///  - two or more parameter values have inconsistent values.
    InvalidHciCommandParameters = 0x12,
    /// The user on the remote device either terminated the connection or stopped broadcasting packets.
    RemoteUserTerminatedConnection = 0x13,
    /// The remote device terminated the connection because of low resources.
    RemoteDeviceTerminatedConnectionDueToLowResources = 0x14,
    /// The remote device terminated the connection because the device is about to power off.
    RemoteDeviceTerminatedConnectionDueToPowerOff = 0x15,
    /// The local device terminated the connection, terminated synchronization with a broadcaster, or terminated
    /// broadcasting packets.
    ConnectionTerminatedByLocalHost = 0x16,
    /// The Controller is disallowing an authentication or pairing procedure because too little time has elapsed
    /// since the last authentication or pairing attempt failed.
    RepeatedAttempts = 0x17,
    /// The device does not allow pairing. For example, when a device only allows pairing during a certain time
    /// window after some user input allows pairing.
    PairingNotAllowed = 0x18,
    /// The Controller has received an unknown LMP opcode.
    UnknownLmpPdu = 0x19,
    /// The remote device does not support the feature associated with the issued command, LMP PDU, or Link Layer Control PDU.
    UnsupportedRemoteFeatureUnsupportedLmpFeature = 0x1A,
    /// The offset requested in the LMP_SCO_LINK_REQ PDU has been rejected.
    ScoOffsetRejected = 0x1B,
    /// The interval requested in the LMP_SCO_LINK_REQ PDU has been rejected.
    ScoIntervalRejected = 0x1C,
    /// The air mode requested in the LMP_SCO_LINK_REQ PDU has been rejected.
    ScoAirModeRejected = 0x1D,
    /// Some LMP PDU / LL Control PDU parameters were invalid:
    ///  - the PDU length is invalid.
    ///  - a parameter is odd when it is required to be even.
    ///  - a parameter is outside of the specified range.
    ///  - two or more parameters have inconsistent values.
    InvalidLmpParametersInvalidLlParameters = 0x1E,
    /// No other error code specified is appropriate to use.
    UnspecifiedError = 0x1F,
    /// An LMP PDU or an LL Control PDU contains at least one parameter value that is not supported by the Controller
    /// at this time. This is normally used after a long negotiation procedure, for example during an LMP_HOLD_REQ,
    /// LMP_SNIFF_REQ and LMP_ENCRYPTION_KEY_SIZE_REQ PDU exchanges. This may be used by the Link Layer, for example
    /// during the Connection Parameters Request Link Layer Control procedure.
    UnsupportedLmpParameterValueUnsupportedLlParameterValue = 0x20,
    /// The Controller will not allow a role change at this time.
    RoleChangeNotAllowed = 0x21,
    /// An LMP transaction failed to respond within the LMP response timeout or an LL transaction failed to respond
    /// within the LL response timeout.
    LmpResponseTimeoutLlResponseTimeout = 0x22,
    /// An LMP transaction or LL procedure has collided with the same transaction or procedure that is already in progress.
    LmpErrorTransactionCollision = 0x23,
    /// The Controller sent an LMP PDU with an opcode that was not allowed.
    LmpPduNotAllowed = 0x24,
    /// The requested encryption mode is not acceptable at this time.
    EncryptionModeNotAcceptable = 0x25,
    /// A link key cannot be changed because a fixed unit key is being used.
    LinkKeyCannotBeChanged = 0x26,
    /// The requested Quality of Service is not supported.
    RequestedQosNotSupported = 0x27,
    /// An LMP PDU or LL PDU that includes an instant cannot be performed because the instant when this would have occurred has passed.
    InstantPassed = 0x28,
    /// It was not possible to pair as a unit key was requested and it is not supported.
    PairingWithUnitKeyNotSupported = 0x29,
    /// An LMP transaction or LL Procedure was started that collides with an ongoing transaction.
    DifferentTransactionCollision = 0x2A,
    /// The specified quality of service parameters could not be accepted at this time, but other parameters may be acceptable.
    QosUnacceptableParameter = 0x2C,
    /// The specified quality of service parameters cannot be accepted and QoS negotiation should be terminated.
    QosRejected = 0x2D,
    /// The Controller cannot perform channel assessment because it is not supported.
    ChannelAssessmentNotSupported = 0x2E,
    /// The HCI command or LMP PDU sent is only possible on an encrypted link.
    InsufficientSecurity = 0x2F,
    /// A parameter value requested is outside the mandatory range of parameters for the given HCI command or LMP PDU
    /// and the recipient does not accept that value.
    ParameterOutOfMandatoryRange = 0x30,
    /// A Role Switch is pending. This can be used when an HCI command or LMP PDU cannot be accepted because of a
    /// pending role switch. This can also be used to notify a peer device about a pending role switch.
    RoleSwitchPending = 0x32,
    /// The current Synchronous negotiation was terminated with the negotiation state set to Reserved Slot Violation.
    ReservedSlotViolation = 0x34,
    /// A role switch was attempted but it failed and the original piconet structure is restored.
    /// The switch may have failed because the TDD switch or piconet switch failed.
    RoleSwitchFailed = 0x35,
    /// The extended inquiry response, with the requested requirements for FEC, is too large to fit in any of the
    /// packet types supported by the Controller.
    ExtendedInquiryResponseTooLarge = 0x36,
    /// The IO capabilities request or response was rejected because the sending Host does not support
    /// Secure Simple Pairing even though the receiving Link Manager does.
    SecureSimplePairingNotSupportedByHost = 0x37,
    /// The Host is busy with another pairing operation and unable to support the requested pairing.
    /// The receiving device should retry pairing again later.
    HostBusyPairing = 0x38,
    /// The Controller could not calculate an appropriate value for the Channel selection operation.
    ConnectionRejectedDueToNoSuitableChannelFound = 0x39,
    /// The operation was rejected because the Controller was busy and unable to process the request.
    ControllerBusy = 0x3A,
    /// The remote device either terminated the connection or rejected a request because of one or more
    /// unacceptable connection parameters.
    UnacceptableConnectionParameters = 0x3B,
    /// Advertising for a fixed duration completed or, for directed advertising, that advertising completed
    /// without a connection being created.
    AdvertisingTimeout = 0x3C,
    /// Either the connection or the synchronization was terminated because the Message Integrity Check (MIC) failed on a received packet.
    ConnectionTerminatedDueToMicFailure = 0x3D,
    /// The LL initiated a connection or initiated synchronization but the connection has failed to be
    /// established or the Link Layer failed to synchronize within the specified time.
    ConnectionFailedToBeEstablished = 0x3E,
    /// The Central, at this time, is unable to make a coarse adjustment to the piconet clock, using the
    /// supplied parameters. Instead the Central will attempt to move the clock using clock dragging.
    CoarseClockAdjustmentRejectedButWillTryToAdjustUsingClockDragging = 0x40,
    /// The LMP PDU is rejected because the Type 0 submap is not currently defined.
    Type0SubmapNotDefined = 0x41,
    /// A command was sent from the Host that should identify an Advertising or Sync handle, but the Advertising or Sync handle does not exist.
    UnknownAdvertisingIdentifier = 0x42,
    /// The number of operations requested has been reached and has indicated the completion of the activity (e.g., advertising or scanning).
    LimitReached = 0x43,
    /// A request to the Controller issued by the Host and still pending was successfully canceled.
    OperationCancelledByHost = 0x44,
    /// An attempt was made to send or receive a packet that exceeds the maximum allowed packet length.
    PacketTooLong = 0x45,
    /// Information was provided too late to the Controller.
    TooLate = 0x46,
    /// Information was provided too early to the Controller.
    TooEarly = 0x47,
    /// The result of the requested operation would yield too few physical channels.
    InsufficientChannels = 0x48,
}

impl ErrorCode {
    pub const fn is_success(&self) -> bool {
        matches!(self, Self::Success)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hci_error_code_creation_success() -> Result<(), Error> {
        let value: ErrorCode = 0x3A.try_into()?;
        assert_eq!(value, ErrorCode::ControllerBusy);
        Ok(())
    }

    #[test]
    fn test_hci_error_code_creation_failure() {
        let err = ErrorCode::try_from(0x93).expect_err("Invalid HCI error code");
        assert!(matches!(err, Error::InvalidErrorCode(0x93)));
    }

    #[test]
    fn test_hci_error_code_is_success() {
        assert!(ErrorCode::Success.is_success());
        assert!(!ErrorCode::PacketTooLong.is_success());
    }
}
