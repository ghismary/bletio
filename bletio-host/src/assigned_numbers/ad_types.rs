//! Assigned numbers for Bluetooth Common Data Types.
//!
//! FILE GENERATED FROM REVISION efc887857333456bda4ac2762443a7a86c99a27b OF THE BLUETOOTH SIG REPOSITORY, DO NOT EDIT!!!

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(dead_code)]
#[non_exhaustive]
/// Assigned numbers for Bluetooth Common Data Types.
pub(crate) enum AdType {
    /// Flags AdType - See Core Specification Supplement, Part A, Section 1.3
    Flags = 0x01,
    /// Incomplete List of 16-bit Service or Service Class UUIDs AdType - See Core Specification Supplement, Part A, Section 1.1
    IncompleteListOfServiceUuid16 = 0x02,
    /// Complete List of 16-bit Service or Service Class UUIDs AdType - See Core Specification Supplement, Part A, Section 1.1
    CompleteListOfServiceUuid16 = 0x03,
    /// Incomplete List of 32-bit Service or Service Class UUIDs AdType - See Core Specification Supplement, Part A, Section 1.1
    IncompleteListOfServiceUuid32 = 0x04,
    /// Complete List of 32-bit Service or Service Class UUIDs AdType - See Core Specification Supplement, Part A, Section 1.1
    CompleteListOfServiceUuid32 = 0x05,
    /// Incomplete List of 128-bit Service or Service Class UUIDs AdType - See Core Specification Supplement, Part A, Section 1.1
    IncompleteListOfServiceUuid128 = 0x06,
    /// Complete List of 128-bit Service or Service Class UUIDs AdType - See Core Specification Supplement, Part A, Section 1.1
    CompleteListOfServiceUuid128 = 0x07,
    /// Shortened Local Name AdType - See Core Specification Supplement, Part A, Section 1.2
    ShortenedLocalName = 0x08,
    /// Complete Local Name AdType - See Core Specification Supplement, Part A, Section 1.2
    CompleteLocalName = 0x09,
    /// Tx Power Level AdType - See Core Specification Supplement, Part A, Section 1.5
    TxPowerLevel = 0x0A,
    /// Class of Device AdType - See Core Specification Supplement, Part A, Section 1.6
    ClassOfDevice = 0x0D,
    /// Simple Pairing Hash C-192 AdType - See Core Specification Supplement, Part A, Section 1.6
    SimplePairingHashC192 = 0x0E,
    /// Simple Pairing Randomizer R-192 AdType - See Core Specification Supplement, Part A, Section 1.6
    SimplePairingRandomizerR192 = 0x0F,
    /// Security Manager TK Value AdType - See Core Specification Supplement, Part A, Section 1.8 (when used in OOB data blocks)
    SecurityManagerTkValue = 0x10,
    /// Security Manager Out of Band Flags AdType - See Core Specification Supplement, Part A, Section 1.7
    SecurityManagerOutOfBandFlags = 0x11,
    /// Peripheral Connection Interval Range AdType - See Core Specification Supplement, Part A, Section 1.9
    PeripheralConnectionIntervalRange = 0x12,
    /// List of 16-bit Service Solicitation UUIDs AdType - See Core Specification Supplement, Part A, Section 1.10
    ListOfSolicitationServiceUuid16 = 0x14,
    /// List of 128-bit Service Solicitation UUIDs AdType - See Core Specification Supplement, Part A, Section 1.10
    ListOfSolicitationServiceUuid128 = 0x15,
    /// Service Data - 16-bit UUID AdType - See Core Specification Supplement, Part A, Section 1.11
    ServiceDataUuid16 = 0x16,
    /// Public Target Address AdType - See Core Specification Supplement, Part A, Section 1.13
    PublicTargetAddress = 0x17,
    /// Random Target Address AdType - See Core Specification Supplement, Part A, Section 1.14
    RandomTargetAddress = 0x18,
    /// Appearance AdType - See Core Specification Supplement, Part A, Section 1.12
    Appearance = 0x19,
    /// Advertising Interval AdType - See Core Specification Supplement, Part A, Section 1.15
    AdvertisingInterval = 0x1A,
    /// LE Bluetooth Device Address AdType - See Core Specification Supplement, Part A, Section 1.16
    LeBluetoothDeviceAddress = 0x1B,
    /// LE Role AdType - See Core Specification Supplement, Part A, Section 1.17
    LeRole = 0x1C,
    /// Simple Pairing Hash C-256 AdType - See Core Specification Supplement, Part A, Section 1.6
    SimplePairingHashC256 = 0x1D,
    /// Simple Pairing Randomizer R-256 AdType - See Core Specification Supplement, Part A, Section 1.6
    SimplePairingRandomizerR256 = 0x1E,
    /// List of 32-bit Service Solicitation UUIDs AdType - See Core Specification Supplement, Part A, Section 1.10
    ListOfSolicitationServiceUuid32 = 0x1F,
    /// Service Data - 32-bit UUID AdType - See Core Specification Supplement, Part A, Section 1.11
    ServiceDataUuid32 = 0x20,
    /// Service Data - 128-bit UUID AdType - See Core Specification Supplement, Part A, Section 1.11
    ServiceDataUuid128 = 0x21,
    /// LE Secure Connections Confirmation Value AdType - See Core Specification Supplement, Part A, Section 1.6
    LeSecureConnectionsConfirmationValue = 0x22,
    /// LE Secure Connections Random Value AdType - See Core Specification Supplement, Part A, Section 1.6
    LeSecureConnectionsRandomValue = 0x23,
    /// URI AdType - See Core Specification Supplement, Part A, Section 1.18
    Uri = 0x24,
    /// Indoor Positioning AdType - See Indoor Positioning Service
    IndoorPositioning = 0x25,
    /// Transport Discovery Data AdType - See Transport Discovery Service
    TransportDiscoveryData = 0x26,
    /// LE Supported Features AdType - See Core Specification Supplement, Part A, Section 1.19
    LeSupportedFeatures = 0x27,
    /// Channel Map Update Indication AdType - See Core Specification Supplement, Part A, Section 1.20
    ChannelMapUpdateIndication = 0x28,
    /// PB-ADV AdType - See Mesh Profile Specification, Section 5.2.1
    PbAdv = 0x29,
    /// Mesh Message AdType - See Mesh Profile Specification, Section 3.3.1
    MeshMessage = 0x2A,
    /// Mesh Beacon AdType - See Mesh Profile Specification, Section 3.9
    MeshBeacon = 0x2B,
    /// BIGInfo AdType - See Core Specification Supplement, Part A, Section 1.21
    BigInfo = 0x2C,
    /// Broadcast_Code AdType - See Core Specification Supplement, Part A, Section 1.22
    BroadcastCode = 0x2D,
    /// Resolvable Set Identifier AdType - See Coordinated Set Identification Profile v1.0 or later
    ResolvableSetIdentifier = 0x2E,
    /// Advertising Interval - long AdType - See Core Specification Supplement, Part A, Section 1.15
    AdvertisingIntervalLong = 0x2F,
    /// Broadcast_Name AdType - See Public Broadcast Profile v1.0 or later
    BroadcastName = 0x30,
    /// Encrypted Advertising Data AdType - See Core Specification Supplement, Part A, Section 1.23
    EncryptedAdvertisingData = 0x31,
    /// Periodic Advertising Response Timing Information AdType - See Core Specification Supplement, Part A, Section 1.24
    PeriodicAdvertisingResponseTimingInformation = 0x32,
    /// Electronic Shelf Label AdType - See ESL Profile
    ElectronicShelfLabel = 0x34,
    /// 3D Information Data AdType - See 3D Synchronization Profile
    ThreeDimensionalInformationData = 0x3D,
    /// Manufacturer Specific Data AdType - See Core Specification Supplement, Part A, Section 1.4
    ManufacturerSpecificData = 0xFF,
}
