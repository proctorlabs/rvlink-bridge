use super::*;

#[allow(dead_code)]
#[derive(Default, Debug, Display, TryFromPrimitive, IntoPrimitive, PartialEq, Clone, Copy)]
#[repr(u16)]
pub enum ParameterID {
    #[default]
    Unknown = 0,
    ProductionBytes = 1,
    CanAdapterMac = 2,
    IdsCanCircuitId = 3,
    IdsCanFunctionName = 4,
    IdsCanFunctionInstance = 5,
    IdsCanNumDevicesOnNetwork = 6,
    IdsCanMaxNetworkHeartbeatTime = 7,
    SerialNumber = 8,
    CanBytesTx = 9,
    CanBytesRx = 10,
    CanMessagesTx = 11,
    CanMessagesRx = 12,
    CanTxBufferOverflowCount = 13,
    CanRxBufferOverflowCount = 14,
    CanTxMaxBytesQueued = 15,
    CanRxMaxBytesQueued = 16,
    UartBytesTx = 17,
    UartBytesRx = 18,
    UartMessagesTx = 19,
    UartMessagesRx = 20,
    UartTxBufferOverflowCount = 21,
    UartRxBufferOverflowCount = 22,
    UartTxMaxBytesQueued = 23,
    UartRxMaxBytesQueued = 24,
    WifiBytesTx = 25,
    WifiBytesRx = 26,
    WifiMessagesTx = 27,
    WifiMessagesRx = 28,
    WifiTxBufferOverflowCount = 29,
    WifiRxBufferOverflowCount = 30,
    WifiTxMaxBytesQueued = 31,
    WifiRxMaxBytesQueued = 32,
    WifiRssi = 33,
    RfBytesTx = 34,
    RfBytesRx = 35,
    RfMessagesTx = 36,
    RfMessagesRx = 37,
    RfTxBufferOverflowCount = 38,
    RfRxBufferOverflowCount = 39,
    RfTxMaxBytesQueued = 40,
    RfRxMaxBytesQueued = 41,
    RfRssi = 42,
    BatteryVoltage = 43,
    RegulatorVoltage = 44,
    NumTiltSensorAxes = 45,
    TiltAxis1Angle = 46,
    TiltAxis2Angle = 47,
    TiltAxis3Angle = 48,
    TiltAxis4Angle = 49,
    TiltAxis5Angle = 50,
    TiltAxis6Angle = 51,
    TiltAxis7Angle = 52,
    TiltAxis8Angle = 53,
    IdsCanFixedAddress = 54,
    FuseSetting1 = 55,
    FuseSetting2 = 56,
    FuseSetting3 = 57,
    FuseSetting4 = 58,
    FuseSetting5 = 59,
    FuseSetting6 = 60,
    FuseSetting7 = 61,
    FuseSetting8 = 62,
    FuseSetting9 = 63,
    FuseSetting10 = 64,
    FuseSetting11 = 65,
    FuseSetting12 = 66,
    FuseSetting13 = 67,
    FuseSetting14 = 68,
    FuseSetting15 = 69,
    FuseSetting16 = 70,
    ManufacturingPid1 = 71,
    ManufacturingPid2 = 72,
    ManufacturingPid3 = 73,
    ManufacturingPid4 = 74,
    ManufacturingPid5 = 75,
    ManufacturingPid6 = 76,
    ManufacturingPid7 = 77,
    ManufacturingPid8 = 78,
    ManufacturingPid9 = 79,
    ManufacturingPid10 = 80,
    ManufacturingPid11 = 81,
    ManufacturingPid12 = 82,
    ManufacturingPid13 = 83,
    ManufacturingPid14 = 84,
    ManufacturingPid15 = 85,
    ManufacturingPid16 = 86,
    ManufacturingPid17 = 87,
    ManufacturingPid18 = 88,
    ManufacturingPid19 = 89,
    ManufacturingPid20 = 90,
    ManufacturingPid21 = 91,
    ManufacturingPid22 = 92,
    ManufacturingPid23 = 93,
    ManufacturingPid24 = 94,
    ManufacturingPid25 = 95,
    ManufacturingPid26 = 96,
    ManufacturingPid27 = 97,
    ManufacturingPid28 = 98,
    ManufacturingPid29 = 99,
    ManufacturingPid30 = 100,
    ManufacturingPid31 = 101,
    ManufacturingPid32 = 102,
    MeteredTimeSec = 103,
    MaintenancePeriodSec = 104,
    LastMaintenanceTimeSec = 105,
    TimeZone = 106,
    RtcTimeSec = 107,
    RtcTimeMin = 108,
    RtcTimeHour = 109,
    RtcTimeDay = 110,
    RtcTimeMonth = 111,
    RtcTimeYear = 112,
    RtcEpochSec = 113,
    RtcSetTimeSec = 114,
    BleDeviceName1 = 115,
    BleDeviceName2 = 116,
    BleDeviceName3 = 117,
    BlePin = 118,
    SystemUptimeMs = 119,
    EthAdapterMac = 120,
    EthBytesTx = 121,
    EthBytesRx = 122,
    EthMessagesTx = 123,
    EthMessagesRx = 124,
    EthTxBufferOverflowCount = 125,
    EthRxBufferOverflowCount = 126,
    EthPacketsTxDiscarded = 127,
    EthPacketsRxDiscarded = 128,
    EthPacketsTxError = 129,
    EthPacketsRxError = 130,
    EthPacketsTxOverflow = 131,
    EthPacketsTxLateCollision = 132,
    EthPacketsTxExcessCollision = 133,
    EthPacketsTxUnderflow = 134,
    EthPacketsRxAlignErr = 135,
    EthPacketsRxCrcErr = 136,
    EthPacketsRxTruncErr = 137,
    EthPacketsRxLenErr = 138,
    EthPacketsRxCollision = 139,
    IpAddress = 140,
    IpSubnetmask = 141,
    IpGateway = 142,
    TcpNumConnections = 143,
    AuxBatteryVoltage = 144,
    RgbLightingGangEnable = 145,
    InputSwitchType = 146,
    DoorLockState = 147,
    GeneratorQuietHoursStartTime = 148,
    GeneratorQuietHoursEndTime = 149,
    GeneratorAutoStartLowVoltage = 150,
    GeneratorAutoStartHiTempC = 151,
    GeneratorAutoRunDurationMinutes = 152,
    GeneratorAutoRunMinOffTimeMinutes = 153,
    SoftwareBuildDateTime = 154,
    GeneratorQuietHoursEnabled = 155,
    ShorePowerAmpRating = 156,
    BatteryCapacityAmpHours = 157,
    PcbAssemblyPartNumber = 158,
    UnlockPin = 159,
    UnlockPinMode = 160,
    SimulateOnOffStyleLight = 161,
    FanSpeedControlType = 162,
    HvacControlType = 163,
    SoftwareFuseRatingAmps = 164,
    SoftwareFuseMaxRatingAmps = 165,
    CumminsOnanGeneratorFaultCode = 166,
    Motor1CurrentAmps = 167,
    Motor2CurrentAmps = 168,
    Motor3CurrentAmps = 169,
    Motor4CurrentAmps = 170,
    Motor5CurrentAmps = 171,
    Motor6CurrentAmps = 172,
    Motor7CurrentAmps = 173,
    Motor8CurrentAmps = 174,
    Motor9CurrentAmps = 175,
    Motor10CurrentAmps = 176,
    Motor11CurrentAmps = 177,
    Motor12CurrentAmps = 178,
    Motor13CurrentAmps = 179,
    Motor14CurrentAmps = 180,
    Motor15CurrentAmps = 181,
    Motor16CurrentAmps = 182,
    DeviceType = 183,
    InMotionLockoutBehavior = 184,
    RvcDetectedNodes = 185,
    RvcLostNodes = 186,
    RvcBytesTx = 187,
    RvcBytesRx = 188,
    RvcMessagesTx = 189,
    RvcMessagesRx = 190,
    RvcTxBuffersFree = 191,
    RvcTxBuffersUsed = 192,
    RvcRxBuffersFree = 193,
    RvcRxBuffersUsed = 194,
    RvcTxOutOfBuffersCount = 195,
    RvcRxOutOfBuffersCount = 196,
    RvcTxFailureCount = 197,
    RvcDefaultSrcAddr = 198,
    RvcDynamicAddr = 199,
    RvcMake = 200,
    RvcModel1 = 201,
    RvcModel2 = 202,
    RvcModel3 = 203,
    RvcSerial = 204,
    RvcIdNumber = 205,
    CloudGatewayAssetIdPart1 = 206,
    CloudGatewayAssetIdPart2 = 207,
    CloudGatewayAssetIdPart3 = 208,
    HvacZoneCapabilities = 209,
    IgnitionBehavior = 210,
    BleNumberOfForwardedCanDevices = 211,
    BleNumberOfConnects = 212,
    BleNumberOfDisconnects = 213,
    BleTotalTraffic = 214,
    BleWritesFromPhone = 215,
    BleNotificationsToPhoneSuccessful = 216,
    BleNotificationsToPhoneFailure = 217,
    BleMtuSizeCentral = 218,
    BleMtuSizePeripheral = 219,
    BleDataLengthTime = 220,
    BleSecurityUnlocked = 221,
    BleClientConnected = 222,
    BleCccdWritten = 223,
    BleNumBuffersFree = 224,
    BleLastTxError = 225,
    BleConnectedDeviceRssi = 226,
    BleDeadClientCounter = 227,
    BleLastDisconnectReason = 228,
    BleSpiRxMsgsDropped = 229,
    BleSpiTxMsgsDropped = 230,
    LowVoltageBehavior = 231,
    DhcpEnabled = 232,
    UdpDeviceName1 = 233,
    UdpDeviceName2 = 234,
    UdpDeviceName3 = 235,
    TcpBatchSize = 236,
    TcpBatchTime = 237,
    OnOffInputPin = 238,
    ExtendInputPin = 239,
    RetractInputPin = 240,
    InputPinCount = 241,
    DsiFaultInputPin = 242,
    DeviceActivationTimeout = 243,
    LevelerUiSupportedFeatures = 244,
    LevelerSensorTopology = 245,
    LevelerDriveType = 246,
    LevelerAutoModeProgress = 247,
    LeftFrontJackStrokeInches = 248,
    RightFrontJackStrokeInches = 249,
    LeftMiddleJackStrokeInches = 250,
    RightMiddleJackStrokeInches = 251,
    LeftRearJackStrokeInches = 252,
    RightRearJackStrokeInches = 253,
    LeftFrontJackMaxStrokeInches = 254,
    RightFrontJackMaxStrokeInches = 255,
    LeftMiddleJackMaxStrokeInches = 256,
    RightMiddleJackMaxStrokeInches = 257,
    LeftRearJackMaxStrokeInches = 258,
    RightRearJackMaxStrokeInches = 259,
    ParkbrakeBehavior = 260,
    ExtendedDeviceCapabilities = 261,
    CloudCapabilities = 262,
    RvMakeId = 263,
    RvModelId = 264,
    RvYear = 265,
    RvFloorplanId = 266,
    FloorplanPartNum = 267,
    FloorplanWrittenBy = 268,
    AssemblyPartNum = 269,
    AssemblyDateCode = 270,
    AssemblySerialNum = 271,
    LevelerAutoProcessSteps1 = 272,
    LevelerAutoProcessSteps2 = 273,
    LevelerAutoProcessSteps3 = 274,
    LevelerAutoProcessSteps4 = 275,
    LevelerAutoProcessSteps5 = 276,
    MonitorPanelDeviceId01 = 277,
    MonitorPanelDeviceId02 = 278,
    MonitorPanelDeviceId03 = 279,
    MonitorPanelDeviceId04 = 280,
    MonitorPanelDeviceId05 = 281,
    MonitorPanelDeviceId06 = 282,
    MonitorPanelDeviceId07 = 283,
    MonitorPanelDeviceId08 = 284,
    MonitorPanelDeviceId09 = 285,
    MonitorPanelDeviceId10 = 286,
    MonitorPanelDeviceId11 = 287,
    MonitorPanelDeviceId12 = 288,
    MonitorPanelDeviceId13 = 289,
    MonitorPanelDeviceId14 = 290,
    MonitorPanelDeviceId15 = 291,
    MonitorPanelDeviceId16 = 292,
    MonitorPanelDeviceId17 = 293,
    MonitorPanelDeviceId18 = 294,
    MonitorPanelDeviceId19 = 295,
    MonitorPanelDeviceId20 = 296,
    MonitorPanelDeviceId21 = 297,
    MonitorPanelDeviceId22 = 298,
    MonitorPanelDeviceId23 = 299,
    MonitorPanelDeviceId24 = 300,
    MonitorPanelDeviceId25 = 301,
    MonitorPanelDeviceId26 = 302,
    MonitorPanelDeviceId27 = 303,
    MonitorPanelDeviceId28 = 304,
    MonitorPanelDeviceId29 = 305,
    MonitorPanelDeviceId30 = 306,
    MonitorPanelDeviceId31 = 307,
    MonitorPanelDeviceId32 = 308,
    MonitorPanelDeviceId33 = 309,
    MonitorPanelDeviceId34 = 310,
    MonitorPanelDeviceId35 = 311,
    MonitorPanelDeviceId36 = 312,
    MonitorPanelDeviceId37 = 313,
    MonitorPanelDeviceId38 = 314,
    MonitorPanelDeviceId39 = 315,
    MonitorPanelDeviceId40 = 316,
    MonitorPanelDeviceId41 = 317,
    MonitorPanelDeviceId42 = 318,
    MonitorPanelDeviceId43 = 319,
    MonitorPanelDeviceId44 = 320,
    MonitorPanelDeviceId45 = 321,
    MonitorPanelDeviceId46 = 322,
    MonitorPanelDeviceId47 = 323,
    MonitorPanelDeviceId48 = 324,
    MonitorPanelControlTypeMomentarySwitch = 325,
    MonitorPanelControlTypeLatchingSwitch = 326,
    MonitorPanelControlTypeSupplyTank = 327,
    MonitorPanelControlTypeWasteTank = 328,
    MonitorPanelConfiguration = 329,
    BlePairingMode = 330,
    MonitorPanelCalibrationPartNbr = 331,
    ReadAddress16bitsData32bits = 332,
    WriteAddress16bitsData32bits = 333,
    TempSensorHighTempAlert = 334,
    TempSensorLowTempAlert = 335,
    AccGwAddDeviceMac = 336,
    AccGwWriteDeviceSwNum = 337,
    VehicleConfiguration = 338,
    TpmsSensorPosition = 339,
    TpmsSensorPresureFaultLimits = 340,
    TpmsSensorTemperatureFaultLimits = 341,
    TpmsSensorId = 342,
    SmartArmWindEventSetting = 343,
    AccRequestMode = 344,
    AccessorySetting01 = 345,
    AccessorySetting02 = 346,
    AccessorySetting03 = 347,
    AccessorySetting04 = 348,
    AccessorySetting05 = 349,
    AccessorySetting06 = 350,
    AccessorySetting07 = 351,
    AccessorySetting08 = 352,
    AccessorySetting09 = 353,
    AccessorySetting10 = 354,
    AccessorySetting11 = 355,
    AccessorySetting12 = 356,
    AccessorySetting13 = 357,
    AccessorySetting14 = 358,
    AccessorySetting15 = 359,
    AccessorySetting16 = 360,
    TireTrackWidth = 361,
    TireDiameter = 362,
    AbsRimTeethCount = 363,
    AbsMaintenancePeriod = 364,
    IlluminationSync = 365,
    RvCInstance = 366,
    HvacControlTypeSetting = 367,
    ActiveHvacControlType = 368,
    MonitorPanelControlTypeConfigTank = 369,
    NumberOfAxles = 370,
    LastMaintenanceOdometer = 371,
    AccGwNumDevices = 372,
    AccGwMacHigh = 373,
    AccGwMacLow = 374,
    DeviceTypeAtIndex = 375,
    BrakeModuleOrientation = 376,
    CoreMicrocontollerReset = 377,
    ProductFwPartNum = 378,
    CoreVersionInfo = 379,
    ProductIdNum = 380,
    ProductIdInConfigBlock = 381,
    LocapVersionInfo = 382,
    ProductFwPartNum1 = 383,
    ProductFwPartNum2 = 384,
    HbridgeSafetyAlertConfig = 385,
    AwningAutoProtectionCount = 386,
    MomentaryHbridgeCircuitRole = 387,
    SoundsHighestCapable = 388,
    SmartArmValanceCorrection = 389,
    JumpToBoot = 390,
}
