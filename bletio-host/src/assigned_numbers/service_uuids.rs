//! Assigned numbers for Bluetooth GATT services.
//!
//! FILE GENERATED FROM REVISION 4f955efbe9144b01c02e2ee2044f43d510cc8da0 OF THE BLUETOOTH SIG REPOSITORY, DO NOT EDIT!!!

use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::advertising::AdvertisingError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[num_enum(error_type(name = AdvertisingError, constructor = AdvertisingError::InvalidServiceUuidValue))]
#[repr(u16)]
#[non_exhaustive]
/// Assigned numbers for Bluetooth GATT services defined in
/// [Assigned Numbers, 3.4](https://bitbucket.org/bluetooth-SIG/public/src/main/assigned_numbers/uuids/service_uuids.yaml).
///
/// It is be used when creating a list of 16-bit Service UUIDs Advertising Structure.
/// See [ServiceUuid16AdStruct::try_new](crate::advertising::ad_struct::ServiceUuid16AdStruct::try_new).
pub enum ServiceUuid {
    /// GAP Service (0x1800) — org.bluetooth.service.gap
    Gap = 0x1800,
    /// GATT Service (0x1801) — org.bluetooth.service.gatt
    Gatt = 0x1801,
    /// Immediate Alert Service (0x1802) — org.bluetooth.service.immediate_alert
    ImmediateAlert = 0x1802,
    /// Link Loss Service (0x1803) — org.bluetooth.service.link_loss
    LinkLoss = 0x1803,
    /// Tx Power Service (0x1804) — org.bluetooth.service.tx_power
    TxPower = 0x1804,
    /// Current Time Service (0x1805) — org.bluetooth.service.current_time
    CurrentTime = 0x1805,
    /// Reference Time Update Service (0x1806) — org.bluetooth.service.reference_time_update
    ReferenceTimeUpdate = 0x1806,
    /// Next DST Change Service (0x1807) — org.bluetooth.service.next_dst_change
    NextDstChange = 0x1807,
    /// Glucose Service (0x1808) — org.bluetooth.service.glucose
    Glucose = 0x1808,
    /// Health Thermometer Service (0x1809) — org.bluetooth.service.health_thermometer
    HealthThermometer = 0x1809,
    /// Device Information Service (0x180A) — org.bluetooth.service.device_information
    DeviceInformation = 0x180A,
    /// Heart Rate Service (0x180D) — org.bluetooth.service.heart_rate
    HeartRate = 0x180D,
    /// Phone Alert Status Service (0x180E) — org.bluetooth.service.phone_alert_status
    PhoneAlertStatus = 0x180E,
    /// Battery Service (0x180F) — org.bluetooth.service.battery_service
    Battery = 0x180F,
    /// Blood Pressure Service (0x1810) — org.bluetooth.service.blood_pressure
    BloodPressure = 0x1810,
    /// Alert Notification Service (0x1811) — org.bluetooth.service.alert_notification
    AlertNotification = 0x1811,
    /// Human Interface Device Service (0x1812) — org.bluetooth.service.human_interface_device
    HumanInterfaceDevice = 0x1812,
    /// Scan Parameters Service (0x1813) — org.bluetooth.service.scan_parameters
    ScanParameters = 0x1813,
    /// Running Speed and Cadence Service (0x1814) — org.bluetooth.service.running_speed_and_cadence
    RunningSpeedAndCadence = 0x1814,
    /// Automation IO Service (0x1815) — org.bluetooth.service.automation_io
    AutomationIo = 0x1815,
    /// Cycling Speed and Cadence Service (0x1816) — org.bluetooth.service.cycling_speed_and_cadence
    CyclingSpeedAndCadence = 0x1816,
    /// Cycling Power Service (0x1818) — org.bluetooth.service.cycling_power
    CyclingPower = 0x1818,
    /// Location and Navigation Service (0x1819) — org.bluetooth.service.location_and_navigation
    LocationAndNavigation = 0x1819,
    /// Environmental Sensing Service (0x181A) — org.bluetooth.service.environmental_sensing
    EnvironmentalSensing = 0x181A,
    /// Body Composition Service (0x181B) — org.bluetooth.service.body_composition
    BodyComposition = 0x181B,
    /// User Data Service (0x181C) — org.bluetooth.service.user_data
    UserData = 0x181C,
    /// Weight Scale Service (0x181D) — org.bluetooth.service.weight_scale
    WeightScale = 0x181D,
    /// Bond Management Service (0x181E) — org.bluetooth.service.bond_management
    BondManagement = 0x181E,
    /// Continuous Glucose Monitoring Service (0x181F) — org.bluetooth.service.continuous_glucose_monitoring
    ContinuousGlucoseMonitoring = 0x181F,
    /// Internet Protocol Support Service (0x1820) — org.bluetooth.service.internet_protocol_support
    InternetProtocolSupport = 0x1820,
    /// Indoor Positioning Service (0x1821) — org.bluetooth.service.indoor_positioning
    IndoorPositioning = 0x1821,
    /// Pulse Oximeter Service (0x1822) — org.bluetooth.service.pulse_oximeter
    PulseOximeter = 0x1822,
    /// HTTP Proxy Service (0x1823) — org.bluetooth.service.http_proxy
    HttpProxy = 0x1823,
    /// Transport Discovery Service (0x1824) — org.bluetooth.service.transport_discovery
    TransportDiscovery = 0x1824,
    /// Object Transfer Service (0x1825) — org.bluetooth.service.object_transfer
    ObjectTransfer = 0x1825,
    /// Fitness Machine Service (0x1826) — org.bluetooth.service.fitness_machine
    FitnessMachine = 0x1826,
    /// Mesh Provisioning Service (0x1827) — org.bluetooth.service.mesh_provisioning
    MeshProvisioning = 0x1827,
    /// Mesh Proxy Service (0x1828) — org.bluetooth.service.mesh_proxy
    MeshProxy = 0x1828,
    /// Reconnection Configuration Service (0x1829) — org.bluetooth.service.reconnection_configuration
    ReconnectionConfiguration = 0x1829,
    /// Insulin Delivery Service (0x183A) — org.bluetooth.service.insulin_delivery
    InsulinDelivery = 0x183A,
    /// Binary Sensor Service (0x183B) — org.bluetooth.service.binary_sensor
    BinarySensor = 0x183B,
    /// Emergency Configuration Service (0x183C) — org.bluetooth.service.emergency_configuration
    EmergencyConfiguration = 0x183C,
    /// Authorization Control Service (0x183D) — org.bluetooth.service.authorization_control
    AuthorizationControl = 0x183D,
    /// Physical Activity Monitor Service (0x183E) — org.bluetooth.service.physical_activity_monitor
    PhysicalActivityMonitor = 0x183E,
    /// Elapsed Time Service (0x183F) — org.bluetooth.service.elapsed_time
    ElapsedTime = 0x183F,
    /// Generic Health Sensor Service (0x1840) — org.bluetooth.service.generic_health_sensor
    GenericHealthSensor = 0x1840,
    /// Audio Input Control Service (0x1843) — org.bluetooth.service.audio_input_control
    AudioInputControl = 0x1843,
    /// Volume Control Service (0x1844) — org.bluetooth.service.volume_control
    VolumeControl = 0x1844,
    /// Volume Offset Control Service (0x1845) — org.bluetooth.service.volume_offset
    VolumeOffsetControl = 0x1845,
    /// Coordinated Set Identification Service (0x1846) — org.bluetooth.service.coordinated_set_identification
    CoordinatedSetIdentification = 0x1846,
    /// Device Time Service (0x1847) — org.bluetooth.service.device_time
    DeviceTime = 0x1847,
    /// Media Control Service (0x1848) — org.bluetooth.service.media_control
    MediaControl = 0x1848,
    /// Generic Media Control Service (0x1849) — org.bluetooth.service.generic_media_control
    GenericMediaControl = 0x1849,
    /// Constant Tone Extension Service (0x184A) — org.bluetooth.service.constant_tone_extension
    ConstantToneExtension = 0x184A,
    /// Telephone Bearer Service (0x184B) — org.bluetooth.service.telephone_bearer
    TelephoneBearer = 0x184B,
    /// Generic Telephone Bearer Service (0x184C) — org.bluetooth.service.generic_telephone_bearer
    GenericTelephoneBearer = 0x184C,
    /// Microphone Control Service (0x184D) — org.bluetooth.service.microphone_control
    MicrophoneControl = 0x184D,
    /// Audio Stream Control Service (0x184E) — org.bluetooth.service.audio_stream_control
    AudioStreamControl = 0x184E,
    /// Broadcast Audio Scan Service (0x184F) — org.bluetooth.service.broadcast_audio_scan
    BroadcastAudioScan = 0x184F,
    /// Published Audio Capabilities Service (0x1850) — org.bluetooth.service.published_audio_capabilities
    PublishedAudioCapabilities = 0x1850,
    /// Basic Audio Announcement Service (0x1851) — org.bluetooth.service.basic_audio_announcement
    BasicAudioAnnouncement = 0x1851,
    /// Broadcast Audio Announcement Service (0x1852) — org.bluetooth.service.broadcast_audio_announcement
    BroadcastAudioAnnouncement = 0x1852,
    /// Common Audio Service (0x1853) — org.bluetooth.service.common_audio
    CommonAudio = 0x1853,
    /// Hearing Access Service (0x1854) — org.bluetooth.service.hearing_access
    HearingAccess = 0x1854,
    /// Telephony and Media Audio Service (0x1855) — org.bluetooth.service.telephony_and_media_audio
    TelephonyAndMediaAudio = 0x1855,
    /// Public Broadcast Announcement Service (0x1856) — org.bluetooth.service.public_broadcast_announcement
    PublicBroadcastAnnouncement = 0x1856,
    /// Electronic Shelf Label Service (0x1857) — org.bluetooth.service.electronic_shelf_label
    ElectronicShelfLabel = 0x1857,
    /// Gaming Audio Service (0x1858) — org.bluetooth.service.gaming_audio
    GamingAudio = 0x1858,
    /// Mesh Proxy Solicitation Service (0x1859) — org.bluetooth.service.mesh_proxy_solicitation
    MeshProxySolicitation = 0x1859,
    /// Industrial Measurement Device Service (0x185A) — org.bluetooth.service.industrial_measurement_device
    IndustrialMeasurementDevice = 0x185A,
    /// Ranging Service (0x185B) — org.bluetooth.service.ranging
    Ranging = 0x185B,
}
