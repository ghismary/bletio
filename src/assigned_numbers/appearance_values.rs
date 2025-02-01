//! Assigned numbers for appearance values.
//!
//! FILE GENERATED FROM REVISION efc887857333456bda4ac2762443a7a86c99a27b OF THE BLUETOOTH SIG REPOSITORY, DO NOT EDIT!!!

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
#[allow(dead_code)]
#[non_exhaustive]
/// Assigned numbers for appearance values defined in
/// [Assigned Numbers, 2.6](https://bitbucket.org/bluetooth-SIG/public/src/main/assigned_numbers/core/appearance_values.yaml).
///
/// It is to be used when creating an Appearance Advertising Structure.
/// See [AppearanceAdStruct::new](crate::advertising::ad_struct::AppearanceAdStruct::new).
pub enum AppearanceValue {
    /// `Unknown - Generic Unknown` Appearance value
    GenericUnknown = 0x0000,
    /// `Phone - Generic Phone` Appearance value
    GenericPhone = 0x0040,
    /// `Computer - Generic Computer` Appearance value
    GenericComputer = 0x0080,
    /// `Computer - Desktop Workstation` Appearance value
    DesktopWorkstation = 0x0081,
    /// `Computer - Server-class Computer` Appearance value
    ServerClassComputer = 0x0082,
    /// `Computer - Laptop` Appearance value
    Laptop = 0x0083,
    /// `Computer - Handheld PC/PDA (clamshell)` Appearance value
    HandheldPcPdaClamshell = 0x0084,
    /// `Computer - Palm-size PC/PDA` Appearance value
    PalmSizePcPda = 0x0085,
    /// `Computer - Wearable computer (watch size)` Appearance value
    WearableComputerWatchSize = 0x0086,
    /// `Computer - Tablet` Appearance value
    Tablet = 0x0087,
    /// `Computer - Docking Station` Appearance value
    DockingStation = 0x0088,
    /// `Computer - All in One` Appearance value
    AllInOne = 0x0089,
    /// `Computer - Blade Server` Appearance value
    BladeServer = 0x008A,
    /// `Computer - Convertible` Appearance value
    Convertible = 0x008B,
    /// `Computer - Detachable` Appearance value
    Detachable = 0x008C,
    /// `Computer - IoT Gateway` Appearance value
    IoTGateway = 0x008D,
    /// `Computer - Mini PC` Appearance value
    MiniPc = 0x008E,
    /// `Computer - Stick PC` Appearance value
    StickPc = 0x008F,
    /// `Watch - Generic Watch` Appearance value
    GenericWatch = 0x00C0,
    /// `Watch - Sports Watch` Appearance value
    SportsWatch = 0x00C1,
    /// `Watch - Smartwatch` Appearance value
    Smartwatch = 0x00C2,
    /// `Clock - Generic Clock` Appearance value
    GenericClock = 0x0100,
    /// `Display - Generic Display` Appearance value
    GenericDisplay = 0x0140,
    /// `Remote Control - Generic Remote Control` Appearance value
    GenericRemoteControl = 0x0180,
    /// `Eye-glasses - Generic Eye-glasses` Appearance value
    GenericEyeGlasses = 0x01C0,
    /// `Tag - Generic Tag` Appearance value
    GenericTag = 0x0200,
    /// `Keyring - Generic Keyring` Appearance value
    GenericKeyring = 0x0240,
    /// `Media Player - Generic Media Player` Appearance value
    GenericMediaPlayer = 0x0280,
    /// `Barcode Scanner - Generic Barcode Scanner` Appearance value
    GenericBarcodeScanner = 0x02C0,
    /// `Thermometer - Generic Thermometer` Appearance value
    GenericThermometer = 0x0300,
    /// `Thermometer - Ear Thermometer` Appearance value
    EarThermometer = 0x0301,
    /// `Heart Rate Sensor - Generic Heart Rate Sensor` Appearance value
    GenericHeartRateSensor = 0x0340,
    /// `Heart Rate Sensor - Heart Rate Belt` Appearance value
    HeartRateBelt = 0x0341,
    /// `Blood Pressure - Generic Blood Pressure` Appearance value
    GenericBloodPressure = 0x0380,
    /// `Blood Pressure - Arm Blood Pressure` Appearance value
    ArmBloodPressure = 0x0381,
    /// `Blood Pressure - Wrist Blood Pressure` Appearance value
    WristBloodPressure = 0x0382,
    /// `Human Interface Device - Generic Human Interface Device` Appearance value
    GenericHumanInterfaceDevice = 0x03C0,
    /// `Human Interface Device - Keyboard` Appearance value
    Keyboard = 0x03C1,
    /// `Human Interface Device - Mouse` Appearance value
    Mouse = 0x03C2,
    /// `Human Interface Device - Joystick` Appearance value
    Joystick = 0x03C3,
    /// `Human Interface Device - Gamepad` Appearance value
    Gamepad = 0x03C4,
    /// `Human Interface Device - Digitizer Tablet` Appearance value
    DigitizerTablet = 0x03C5,
    /// `Human Interface Device - Card Reader` Appearance value
    CardReader = 0x03C6,
    /// `Human Interface Device - Digital Pen` Appearance value
    DigitalPen = 0x03C7,
    /// `Human Interface Device - Barcode Scanner` Appearance value
    BarcodeScanner = 0x03C8,
    /// `Human Interface Device - Touchpad` Appearance value
    Touchpad = 0x03C9,
    /// `Human Interface Device - Presentation Remote` Appearance value
    PresentationRemote = 0x03CA,
    /// `Glucose Meter - Generic Glucose Meter` Appearance value
    GenericGlucoseMeter = 0x0400,
    /// `Running Walking Sensor - Generic Running Walking Sensor` Appearance value
    GenericRunningWalkingSensor = 0x0440,
    /// `Running Walking Sensor - In-Shoe Running Walking Sensor` Appearance value
    InShoeRunningWalkingSensor = 0x0441,
    /// `Running Walking Sensor - On-Shoe Running Walking Sensor` Appearance value
    OnShoeRunningWalkingSensor = 0x0442,
    /// `Running Walking Sensor - On-Hip Running Walking Sensor` Appearance value
    OnHipRunningWalkingSensor = 0x0443,
    /// `Cycling - Generic Cycling` Appearance value
    GenericCycling = 0x0480,
    /// `Cycling - Cycling Computer` Appearance value
    CyclingComputer = 0x0481,
    /// `Cycling - Speed Sensor` Appearance value
    SpeedSensor = 0x0482,
    /// `Cycling - Cadence Sensor` Appearance value
    CadenceSensor = 0x0483,
    /// `Cycling - Power Sensor` Appearance value
    PowerSensor = 0x0484,
    /// `Cycling - Speed and Cadence Sensor` Appearance value
    SpeedAndCadenceSensor = 0x0485,
    /// `Control Device - Generic Control Device` Appearance value
    GenericControlDevice = 0x04C0,
    /// `Control Device - Switch` Appearance value
    Switch = 0x04C1,
    /// `Control Device - Multi-switch` Appearance value
    MultiSwitch = 0x04C2,
    /// `Control Device - Button` Appearance value
    Button = 0x04C3,
    /// `Control Device - Slider` Appearance value
    Slider = 0x04C4,
    /// `Control Device - Rotary Switch` Appearance value
    RotarySwitch = 0x04C5,
    /// `Control Device - Touch Panel` Appearance value
    TouchPanel = 0x04C6,
    /// `Control Device - Single Switch` Appearance value
    SingleSwitch = 0x04C7,
    /// `Control Device - Double Switch` Appearance value
    DoubleSwitch = 0x04C8,
    /// `Control Device - Triple Switch` Appearance value
    TripleSwitch = 0x04C9,
    /// `Control Device - Battery Switch` Appearance value
    BatterySwitch = 0x04CA,
    /// `Control Device - Energy Harvesting Switch` Appearance value
    EnergyHarvestingSwitch = 0x04CB,
    /// `Control Device - Push Button` Appearance value
    PushButton = 0x04CC,
    /// `Control Device - Dial` Appearance value
    Dial = 0x04CD,
    /// `Network Device - Generic Network Device` Appearance value
    GenericNetworkDevice = 0x0500,
    /// `Network Device - Access Point` Appearance value
    AccessPoint = 0x0501,
    /// `Network Device - Mesh Device` Appearance value
    MeshDevice = 0x0502,
    /// `Network Device - Mesh Network Proxy` Appearance value
    MeshNetworkProxy = 0x0503,
    /// `Sensor - Generic Sensor` Appearance value
    GenericSensor = 0x0540,
    /// `Sensor - Motion Sensor` Appearance value
    MotionSensor = 0x0541,
    /// `Sensor - Air quality Sensor` Appearance value
    AirQualitySensor = 0x0542,
    /// `Sensor - Temperature Sensor` Appearance value
    TemperatureSensor = 0x0543,
    /// `Sensor - Humidity Sensor` Appearance value
    HumiditySensor = 0x0544,
    /// `Sensor - Leak Sensor` Appearance value
    LeakSensor = 0x0545,
    /// `Sensor - Smoke Sensor` Appearance value
    SmokeSensor = 0x0546,
    /// `Sensor - Occupancy Sensor` Appearance value
    OccupancySensor = 0x0547,
    /// `Sensor - Contact Sensor` Appearance value
    ContactSensor = 0x0548,
    /// `Sensor - Carbon Monoxide Sensor` Appearance value
    CarbonMonoxideSensor = 0x0549,
    /// `Sensor - Carbon Dioxide Sensor` Appearance value
    CarbonDioxideSensor = 0x054A,
    /// `Sensor - Ambient Light Sensor` Appearance value
    AmbientLightSensor = 0x054B,
    /// `Sensor - Energy Sensor` Appearance value
    EnergySensor = 0x054C,
    /// `Sensor - Color Light Sensor` Appearance value
    ColorLightSensor = 0x054D,
    /// `Sensor - Rain Sensor` Appearance value
    RainSensor = 0x054E,
    /// `Sensor - Fire Sensor` Appearance value
    FireSensor = 0x054F,
    /// `Sensor - Wind Sensor` Appearance value
    WindSensor = 0x0550,
    /// `Sensor - Proximity Sensor` Appearance value
    ProximitySensor = 0x0551,
    /// `Sensor - Multi-Sensor` Appearance value
    MultiSensor = 0x0552,
    /// `Sensor - Flush Mounted Sensor` Appearance value
    FlushMountedSensor = 0x0553,
    /// `Sensor - Ceiling Mounted Sensor` Appearance value
    CeilingMountedSensor = 0x0554,
    /// `Sensor - Wall Mounted Sensor` Appearance value
    WallMountedSensor = 0x0555,
    /// `Sensor - Multisensor` Appearance value
    Multisensor = 0x0556,
    /// `Sensor - Energy Meter` Appearance value
    EnergyMeter = 0x0557,
    /// `Sensor - Flame Detector` Appearance value
    FlameDetector = 0x0558,
    /// `Sensor - Vehicle Tire Pressure Sensor` Appearance value
    VehicleTirePressureSensor = 0x0559,
    /// `Light Fixtures - Generic Light Fixtures` Appearance value
    GenericLightFixtures = 0x0580,
    /// `Light Fixtures - Wall Light` Appearance value
    WallLight = 0x0581,
    /// `Light Fixtures - Ceiling Light` Appearance value
    CeilingLight = 0x0582,
    /// `Light Fixtures - Floor Light` Appearance value
    FloorLight = 0x0583,
    /// `Light Fixtures - Cabinet Light` Appearance value
    CabinetLight = 0x0584,
    /// `Light Fixtures - Desk Light` Appearance value
    DeskLight = 0x0585,
    /// `Light Fixtures - Troffer Light` Appearance value
    TrofferLight = 0x0586,
    /// `Light Fixtures - Pendant Light` Appearance value
    PendantLight = 0x0587,
    /// `Light Fixtures - In-ground Light` Appearance value
    InGroundLight = 0x0588,
    /// `Light Fixtures - Flood Light` Appearance value
    FloodLight = 0x0589,
    /// `Light Fixtures - Underwater Light` Appearance value
    UnderwaterLight = 0x058A,
    /// `Light Fixtures - Bollard with Light` Appearance value
    BollardWithLight = 0x058B,
    /// `Light Fixtures - Pathway Light` Appearance value
    PathwayLight = 0x058C,
    /// `Light Fixtures - Garden Light` Appearance value
    GardenLight = 0x058D,
    /// `Light Fixtures - Pole-top Light` Appearance value
    PoleTopLight = 0x058E,
    /// `Light Fixtures - Spotlight` Appearance value
    Spotlight = 0x058F,
    /// `Light Fixtures - Linear Light` Appearance value
    LinearLight = 0x0590,
    /// `Light Fixtures - Street Light` Appearance value
    StreetLight = 0x0591,
    /// `Light Fixtures - Shelves Light` Appearance value
    ShelvesLight = 0x0592,
    /// `Light Fixtures - Bay Light` Appearance value
    BayLight = 0x0593,
    /// `Light Fixtures - Emergency Exit Light` Appearance value
    EmergencyExitLight = 0x0594,
    /// `Light Fixtures - Light Controller` Appearance value
    LightController = 0x0595,
    /// `Light Fixtures - Light Driver` Appearance value
    LightDriver = 0x0596,
    /// `Light Fixtures - Bulb` Appearance value
    Bulb = 0x0597,
    /// `Light Fixtures - Low-bay Light` Appearance value
    LowBayLight = 0x0598,
    /// `Light Fixtures - High-bay Light` Appearance value
    HighBayLight = 0x0599,
    /// `Fan - Generic Fan` Appearance value
    GenericFan = 0x05C0,
    /// `Fan - Ceiling Fan` Appearance value
    CeilingFan = 0x05C1,
    /// `Fan - Axial Fan` Appearance value
    AxialFan = 0x05C2,
    /// `Fan - Exhaust Fan` Appearance value
    ExhaustFan = 0x05C3,
    /// `Fan - Pedestal Fan` Appearance value
    PedestalFan = 0x05C4,
    /// `Fan - Desk Fan` Appearance value
    DeskFan = 0x05C5,
    /// `Fan - Wall Fan` Appearance value
    WallFan = 0x05C6,
    /// `HVAC - Generic HVAC` Appearance value
    GenericHvac = 0x0600,
    /// `HVAC - Thermostat` Appearance value
    Thermostat = 0x0601,
    /// `HVAC - Humidifier` Appearance value
    Humidifier = 0x0602,
    /// `HVAC - De-humidifier` Appearance value
    DeHumidifier = 0x0603,
    /// `HVAC - Heater` Appearance value
    Heater = 0x0604,
    /// `HVAC - Radiator` Appearance value
    HvacRadiator = 0x0605,
    /// `HVAC - Boiler` Appearance value
    HvacBoiler = 0x0606,
    /// `HVAC - Heat Pump` Appearance value
    HvacHeatPump = 0x0607,
    /// `HVAC - Infrared Heater` Appearance value
    HvacInfraredHeater = 0x0608,
    /// `HVAC - Radiant Panel Heater` Appearance value
    HvacRadiantPanelHeater = 0x0609,
    /// `HVAC - Fan Heater` Appearance value
    HvacFanHeater = 0x060A,
    /// `HVAC - Air Curtain` Appearance value
    HvacAirCurtain = 0x060B,
    /// `Air Conditioning - Generic Air Conditioning` Appearance value
    GenericAirConditioning = 0x0640,
    /// `Humidifier - Generic Humidifier` Appearance value
    GenericHumidifier = 0x0680,
    /// `Heating - Generic Heating` Appearance value
    GenericHeating = 0x06C0,
    /// `Heating - Radiator` Appearance value
    HeatingRadiator = 0x06C1,
    /// `Heating - Boiler` Appearance value
    HeatingBoiler = 0x06C2,
    /// `Heating - Heat Pump` Appearance value
    HeatingHeatPump = 0x06C3,
    /// `Heating - Infrared Heater` Appearance value
    HeatingInfraredHeater = 0x06C4,
    /// `Heating - Radiant Panel Heater` Appearance value
    HeatingRadiantPanelHeater = 0x06C5,
    /// `Heating - Fan Heater` Appearance value
    HeatingFanHeater = 0x06C6,
    /// `Heating - Air Curtain` Appearance value
    HeatingAirCurtain = 0x06C7,
    /// `Access Control - Generic Access Control` Appearance value
    GenericAccessControl = 0x0700,
    /// `Access Control - Access Door` Appearance value
    AccessDoor = 0x0701,
    /// `Access Control - Garage Door` Appearance value
    GarageDoor = 0x0702,
    /// `Access Control - Emergency Exit Door` Appearance value
    EmergencyExitDoor = 0x0703,
    /// `Access Control - Access Lock` Appearance value
    AccessLock = 0x0704,
    /// `Access Control - Elevator` Appearance value
    Elevator = 0x0705,
    /// `Access Control - Window` Appearance value
    Window = 0x0706,
    /// `Access Control - Entrance Gate` Appearance value
    EntranceGate = 0x0707,
    /// `Access Control - Door Lock` Appearance value
    DoorLock = 0x0708,
    /// `Access Control - Locker` Appearance value
    Locker = 0x0709,
    /// `Motorized Device - Generic Motorized Device` Appearance value
    GenericMotorizedDevice = 0x0740,
    /// `Motorized Device - Motorized Gate` Appearance value
    MotorizedGate = 0x0741,
    /// `Motorized Device - Awning` Appearance value
    Awning = 0x0742,
    /// `Motorized Device - Blinds or Shades` Appearance value
    BlindsOrShades = 0x0743,
    /// `Motorized Device - Curtains` Appearance value
    Curtains = 0x0744,
    /// `Motorized Device - Screen` Appearance value
    Screen = 0x0745,
    /// `Power Device - Generic Power Device` Appearance value
    GenericPowerDevice = 0x0780,
    /// `Power Device - Power Outlet` Appearance value
    PowerOutlet = 0x0781,
    /// `Power Device - Power Strip` Appearance value
    PowerStrip = 0x0782,
    /// `Power Device - Plug` Appearance value
    Plug = 0x0783,
    /// `Power Device - Power Supply` Appearance value
    PowerSupply = 0x0784,
    /// `Power Device - LED Driver` Appearance value
    LedDriver = 0x0785,
    /// `Power Device - Fluorescent Lamp Gear` Appearance value
    FluorescentLampGear = 0x0786,
    /// `Power Device - HID Lamp Gear` Appearance value
    HidLampGear = 0x0787,
    /// `Power Device - Charge Case` Appearance value
    ChargeCase = 0x0788,
    /// `Power Device - Power Bank` Appearance value
    PowerBank = 0x0789,
    /// `Light Source - Generic Light Source` Appearance value
    GenericLightSource = 0x07C0,
    /// `Light Source - Incandescent Light Bulb` Appearance value
    IncandescentLightBulb = 0x07C1,
    /// `Light Source - LED Lamp` Appearance value
    LedLamp = 0x07C2,
    /// `Light Source - HID Lamp` Appearance value
    HidLamp = 0x07C3,
    /// `Light Source - Fluorescent Lamp` Appearance value
    FluorescentLamp = 0x07C4,
    /// `Light Source - LED Array` Appearance value
    LedArray = 0x07C5,
    /// `Light Source - Multi-Color LED Array` Appearance value
    MultiColorLedArray = 0x07C6,
    /// `Light Source - Low voltage halogen` Appearance value
    LowVoltageHalogen = 0x07C7,
    /// `Light Source - Organic light emitting diode (OLED)` Appearance value
    OrganicLightEmittingDiodeOled = 0x07C8,
    /// `Window Covering - Generic Window Covering` Appearance value
    GenericWindowCovering = 0x0800,
    /// `Window Covering - Window Shades` Appearance value
    WindowShades = 0x0801,
    /// `Window Covering - Window Blinds` Appearance value
    WindowBlinds = 0x0802,
    /// `Window Covering - Window Awning` Appearance value
    WindowAwning = 0x0803,
    /// `Window Covering - Window Curtain` Appearance value
    WindowCurtain = 0x0804,
    /// `Window Covering - Exterior Shutter` Appearance value
    ExteriorShutter = 0x0805,
    /// `Window Covering - Exterior Screen` Appearance value
    ExteriorScreen = 0x0806,
    /// `Audio Sink - Generic Audio Sink` Appearance value
    GenericAudioSink = 0x0840,
    /// `Audio Sink - Standalone Speaker` Appearance value
    StandaloneSpeaker = 0x0841,
    /// `Audio Sink - Soundbar` Appearance value
    Soundbar = 0x0842,
    /// `Audio Sink - Bookshelf Speaker` Appearance value
    BookshelfSpeaker = 0x0843,
    /// `Audio Sink - Standmounted Speaker` Appearance value
    StandmountedSpeaker = 0x0844,
    /// `Audio Sink - Speakerphone` Appearance value
    Speakerphone = 0x0845,
    /// `Audio Source - Generic Audio Source` Appearance value
    GenericAudioSource = 0x0880,
    /// `Audio Source - Microphone` Appearance value
    Microphone = 0x0881,
    /// `Audio Source - Alarm` Appearance value
    Alarm = 0x0882,
    /// `Audio Source - Bell` Appearance value
    Bell = 0x0883,
    /// `Audio Source - Horn` Appearance value
    Horn = 0x0884,
    /// `Audio Source - Broadcasting Device` Appearance value
    BroadcastingDevice = 0x0885,
    /// `Audio Source - Service Desk` Appearance value
    ServiceDesk = 0x0886,
    /// `Audio Source - Kiosk` Appearance value
    Kiosk = 0x0887,
    /// `Audio Source - Broadcasting Room` Appearance value
    BroadcastingRoom = 0x0888,
    /// `Audio Source - Auditorium` Appearance value
    Auditorium = 0x0889,
    /// `Motorized Vehicle - Generic Motorized Vehicle` Appearance value
    GenericMotorizedVehicle = 0x08C0,
    /// `Motorized Vehicle - Car` Appearance value
    Car = 0x08C1,
    /// `Motorized Vehicle - Large Goods Vehicle` Appearance value
    LargeGoodsVehicle = 0x08C2,
    /// `Motorized Vehicle - 2-Wheeled Vehicle` Appearance value
    _2WheeledVehicle = 0x08C3,
    /// `Motorized Vehicle - Motorbike` Appearance value
    Motorbike = 0x08C4,
    /// `Motorized Vehicle - Scooter` Appearance value
    Scooter = 0x08C5,
    /// `Motorized Vehicle - Moped` Appearance value
    Moped = 0x08C6,
    /// `Motorized Vehicle - 3-Wheeled Vehicle` Appearance value
    _3WheeledVehicle = 0x08C7,
    /// `Motorized Vehicle - Light Vehicle` Appearance value
    LightVehicle = 0x08C8,
    /// `Motorized Vehicle - Quad Bike` Appearance value
    QuadBike = 0x08C9,
    /// `Motorized Vehicle - Minibus` Appearance value
    Minibus = 0x08CA,
    /// `Motorized Vehicle - Bus` Appearance value
    Bus = 0x08CB,
    /// `Motorized Vehicle - Trolley` Appearance value
    Trolley = 0x08CC,
    /// `Motorized Vehicle - Agricultural Vehicle` Appearance value
    AgriculturalVehicle = 0x08CD,
    /// `Motorized Vehicle - Camper / Caravan` Appearance value
    CamperCaravan = 0x08CE,
    /// `Motorized Vehicle - Recreational Vehicle / Motor Home` Appearance value
    RecreationalVehicleMotorHome = 0x08CF,
    /// `Domestic Appliance - Generic Domestic Appliance` Appearance value
    GenericDomesticAppliance = 0x0900,
    /// `Domestic Appliance - Refrigerator` Appearance value
    Refrigerator = 0x0901,
    /// `Domestic Appliance - Freezer` Appearance value
    Freezer = 0x0902,
    /// `Domestic Appliance - Oven` Appearance value
    Oven = 0x0903,
    /// `Domestic Appliance - Microwave` Appearance value
    Microwave = 0x0904,
    /// `Domestic Appliance - Toaster` Appearance value
    Toaster = 0x0905,
    /// `Domestic Appliance - Washing Machine` Appearance value
    WashingMachine = 0x0906,
    /// `Domestic Appliance - Dryer` Appearance value
    Dryer = 0x0907,
    /// `Domestic Appliance - Coffee maker` Appearance value
    CoffeeMaker = 0x0908,
    /// `Domestic Appliance - Clothes iron` Appearance value
    ClothesIron = 0x0909,
    /// `Domestic Appliance - Curling iron` Appearance value
    CurlingIron = 0x090A,
    /// `Domestic Appliance - Hair dryer` Appearance value
    HairDryer = 0x090B,
    /// `Domestic Appliance - Vacuum cleaner` Appearance value
    VacuumCleaner = 0x090C,
    /// `Domestic Appliance - Robotic vacuum cleaner` Appearance value
    RoboticVacuumCleaner = 0x090D,
    /// `Domestic Appliance - Rice cooker` Appearance value
    RiceCooker = 0x090E,
    /// `Domestic Appliance - Clothes steamer` Appearance value
    ClothesSteamer = 0x090F,
    /// `Wearable Audio Device - Generic Wearable Audio Device` Appearance value
    GenericWearableAudioDevice = 0x0940,
    /// `Wearable Audio Device - Earbud` Appearance value
    Earbud = 0x0941,
    /// `Wearable Audio Device - Headset` Appearance value
    Headset = 0x0942,
    /// `Wearable Audio Device - Headphones` Appearance value
    Headphones = 0x0943,
    /// `Wearable Audio Device - Neck Band` Appearance value
    NeckBand = 0x0944,
    /// `Aircraft - Generic Aircraft` Appearance value
    GenericAircraft = 0x0980,
    /// `Aircraft - Light Aircraft` Appearance value
    LightAircraft = 0x0981,
    /// `Aircraft - Microlight` Appearance value
    Microlight = 0x0982,
    /// `Aircraft - Paraglider` Appearance value
    Paraglider = 0x0983,
    /// `Aircraft - Large Passenger Aircraft` Appearance value
    LargePassengerAircraft = 0x0984,
    /// `AV Equipment - Generic AV Equipment` Appearance value
    GenericAvEquipment = 0x09C0,
    /// `AV Equipment - Amplifier` Appearance value
    Amplifier = 0x09C1,
    /// `AV Equipment - Receiver` Appearance value
    Receiver = 0x09C2,
    /// `AV Equipment - Radio` Appearance value
    Radio = 0x09C3,
    /// `AV Equipment - Tuner` Appearance value
    Tuner = 0x09C4,
    /// `AV Equipment - Turntable` Appearance value
    Turntable = 0x09C5,
    /// `AV Equipment - CD Player` Appearance value
    CdPlayer = 0x09C6,
    /// `AV Equipment - DVD Player` Appearance value
    DvdPlayer = 0x09C7,
    /// `AV Equipment - Bluray Player` Appearance value
    BlurayPlayer = 0x09C8,
    /// `AV Equipment - Optical Disc Player` Appearance value
    OpticalDiscPlayer = 0x09C9,
    /// `AV Equipment - Set-Top Box` Appearance value
    SetTopBox = 0x09CA,
    /// `Display Equipment - Generic Display Equipment` Appearance value
    GenericDisplayEquipment = 0x0A00,
    /// `Display Equipment - Television` Appearance value
    Television = 0x0A01,
    /// `Display Equipment - Monitor` Appearance value
    Monitor = 0x0A02,
    /// `Display Equipment - Projector` Appearance value
    Projector = 0x0A03,
    /// `Hearing aid - Generic Hearing aid` Appearance value
    GenericHearingAid = 0x0A40,
    /// `Hearing aid - In-ear hearing aid` Appearance value
    InEarHearingAid = 0x0A41,
    /// `Hearing aid - Behind-ear hearing aid` Appearance value
    BehindEarHearingAid = 0x0A42,
    /// `Hearing aid - Cochlear Implant` Appearance value
    CochlearImplant = 0x0A43,
    /// `Gaming - Generic Gaming` Appearance value
    GenericGaming = 0x0A80,
    /// `Gaming - Home Video Game Console` Appearance value
    HomeVideoGameConsole = 0x0A81,
    /// `Gaming - Portable handheld console` Appearance value
    PortableHandheldConsole = 0x0A82,
    /// `Signage - Generic Signage` Appearance value
    GenericSignage = 0x0AC0,
    /// `Signage - Digital Signage` Appearance value
    DigitalSignage = 0x0AC1,
    /// `Signage - Electronic Label` Appearance value
    ElectronicLabel = 0x0AC2,
    /// `Pulse Oximeter - Generic Pulse Oximeter` Appearance value
    GenericPulseOximeter = 0x0C40,
    /// `Pulse Oximeter - Fingertip Pulse Oximeter` Appearance value
    FingertipPulseOximeter = 0x0C41,
    /// `Pulse Oximeter - Wrist Worn Pulse Oximeter` Appearance value
    WristWornPulseOximeter = 0x0C42,
    /// `Weight Scale - Generic Weight Scale` Appearance value
    GenericWeightScale = 0x0C80,
    /// `Personal Mobility Device - Generic Personal Mobility Device` Appearance value
    GenericPersonalMobilityDevice = 0x0CC0,
    /// `Personal Mobility Device - Powered Wheelchair` Appearance value
    PoweredWheelchair = 0x0CC1,
    /// `Personal Mobility Device - Mobility Scooter` Appearance value
    MobilityScooter = 0x0CC2,
    /// `Continuous Glucose Monitor - Generic Continuous Glucose Monitor` Appearance value
    GenericContinuousGlucoseMonitor = 0x0D00,
    /// `Insulin Pump - Generic Insulin Pump` Appearance value
    GenericInsulinPump = 0x0D40,
    /// `Insulin Pump - Insulin Pump, durable pump` Appearance value
    InsulinPumpDurablePump = 0x0D41,
    /// `Insulin Pump - Insulin Pump, patch pump` Appearance value
    InsulinPumpPatchPump = 0x0D44,
    /// `Insulin Pump - Insulin Pen` Appearance value
    InsulinPen = 0x0D48,
    /// `Medication Delivery - Generic Medication Delivery` Appearance value
    GenericMedicationDelivery = 0x0D80,
    /// `Spirometer - Generic Spirometer` Appearance value
    GenericSpirometer = 0x0DC0,
    /// `Spirometer - Handheld Spirometer` Appearance value
    HandheldSpirometer = 0x0DC1,
    /// `Outdoor Sports Activity - Generic Outdoor Sports Activity` Appearance value
    GenericOutdoorSportsActivity = 0x1440,
    /// `Outdoor Sports Activity - Location Display` Appearance value
    LocationDisplay = 0x1441,
    /// `Outdoor Sports Activity - Location and Navigation Display` Appearance value
    LocationAndNavigationDisplay = 0x1442,
    /// `Outdoor Sports Activity - Location Pod` Appearance value
    LocationPod = 0x1443,
    /// `Outdoor Sports Activity - Location and Navigation Pod` Appearance value
    LocationAndNavigationPod = 0x1444,
    /// `Industrial Measurement Device - Generic Industrial Measurement Device` Appearance value
    GenericIndustrialMeasurementDevice = 0x1480,
    /// `Industrial Measurement Device - Torque Testing Device` Appearance value
    TorqueTestingDevice = 0x1481,
    /// `Industrial Measurement Device - Caliper` Appearance value
    Caliper = 0x1482,
    /// `Industrial Measurement Device - Dial Indicator` Appearance value
    DialIndicator = 0x1483,
    /// `Industrial Measurement Device - Micrometer` Appearance value
    Micrometer = 0x1484,
    /// `Industrial Measurement Device - Height Gauge` Appearance value
    HeightGauge = 0x1485,
    /// `Industrial Measurement Device - Force Gauge` Appearance value
    ForceGauge = 0x1486,
    /// `Industrial Tools - Generic Industrial Tools` Appearance value
    GenericIndustrialTools = 0x14C0,
    /// `Industrial Tools - Machine Tool Holder` Appearance value
    MachineToolHolder = 0x14C1,
    /// `Industrial Tools - Generic Clamping Device` Appearance value
    GenericClampingDevice = 0x14C2,
    /// `Industrial Tools - Clamping Jaws/Jaw Chuck` Appearance value
    ClampingJawsJawChuck = 0x14C3,
    /// `Industrial Tools - Clamping (Collet) Chuck` Appearance value
    ClampingColletChuck = 0x14C4,
    /// `Industrial Tools - Clamping Mandrel` Appearance value
    ClampingMandrel = 0x14C5,
    /// `Industrial Tools - Vise` Appearance value
    Vise = 0x14C6,
    /// `Industrial Tools - Zero-Point Clamping System` Appearance value
    ZeroPointClampingSystem = 0x14C7,
    /// `Industrial Tools - Torque Wrench` Appearance value
    TorqueWrench = 0x14C8,
    /// `Industrial Tools - Torque Screwdriver` Appearance value
    TorqueScrewdriver = 0x14C9,
}
