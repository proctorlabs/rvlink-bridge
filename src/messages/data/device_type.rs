use super::*;

#[allow(dead_code)]
#[derive(Default, Debug, Display, PartialEq, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum DeviceType {
    #[default]
    Unknown = 0,
    Generic = 1,
    Tablet = 2,
    LatchingRelay = 3,
    MomentaryRelay = 4,
    LatchingHBridge = 5,
    MomentaryHBridge = 6,
    LevelerType1 = 7,
    Switch = 8,
    TouchscreenSwitch = 9,
    TankSensor = 10,
    LevelerType2 = 11,
    HourMeter = 12,
    RgbLight = 13,
    RealTimeClock = 14,
    IrRemoteControl = 15,
    HvacControl = 16,
    LevelerType3 = 17,
    CanToEthernetGateway = 18,
    InTransitPowerDisconnect = 19,
    DimmableLight = 20,
    OnecontrolTouchPad = 21,
    AndroidMobileDevice = 22,
    IosMobileDevice = 23,
    GeneratorGenie = 24,
    TemperatureSensor = 25,
    AcPowerMonitor = 26,
    DcPowerMonitor = 27,
    SetecPowerManager = 28,
    OnecontrolCloudGateway = 29,
    LatchingRelayType2 = 30,
    MomentaryRelayType2 = 31,
    LatchingHBridgeType2 = 32,
    MomentaryHBridgeType2 = 33,
    OnecontrolApplication = 34,
    ConfiguratorApplication = 35,
    BluetoothGateway = 36,
    MaxxFan = 37,
    RainSensor = 38,
    ChassisInfo = 39,
    LevelerType4 = 40,
    WifiGateway = 41,
    TpmsTireLinc = 42,
    MonitorPanel = 43,
    AccessoryGateway = 44,
    Camera = 45,
    JaycoAusTbbGw = 46,
    AwningSensor = 47,
    BrakeController = 48,
    BatteryMonitor = 49,
    ReflashBootloader = 50,
    DoorLock = 51,
    AudibleAlert = 52,
    EchoBrakeControl = 53,
}
