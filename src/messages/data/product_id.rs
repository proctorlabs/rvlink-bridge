use super::*;

#[allow(dead_code)]
#[derive(Default, Debug, Display, PartialEq, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
#[repr(u16)]
pub enum ProductID {
    #[default]
    Unknown = 0,
    IdsCanNetworkAnalyzerPcTool = 1,
    LciLincpadWifiHubAssembly = 2,
    LciLincpad2MotorVelocitySyncInwallSlideControlAssembly = 3,
    LciLincpadDockingStationAssembly = 4,
    LciMf5f3vFuseMuxReceiverLincpadAssembly = 5,
    LciMf8f5vFuseMuxReceiverLincpadAssembly = 6,
    LciLincpadLightingControlAssembly = 7,
    LciLincpadMultifunction8OutputReceiverAssembly = 8,
    LciLincpadMultifunction5OutputReceiverAssembly = 9,
    LciLincpadLevelUpLevelingControlAssembly = 10,
    LciLincpadTankMonitorControlAssembly = 11,
    LciLincpadSwitchToCanConverterControlAssembly = 12,
    LciLincpadLincToCanTouchpadAssembly = 13,
    LciLincpadTablet = 14,
    LciLincpad6LegHallEffectEjLevelerAssembly = 15,
    LciLincpad4ptCamperJackControlAssembly = 16,
    LciMyrv4pt5wHallEffectEjLevelerControlAssembly = 17,
    LciMyrv5ptHallEffectHybridEjTtLevelerAssy = 18,
    LciMyrv4ptFoldingJackTtLevelerAssy = 19,
    LciMyrvHourMeterWithStartStopDrive = 20,
    LciMyrvRgbLedLightingControllerAssembly = 21,
    LciBleMf5f3vFuseMuxReceiverLinctabAssembly = 22,
    LciBleMf8f5vFuseMuxReceiverLinctabAssembly = 23,
    LciLinctabBleMultifunction8OutputReceiverAssembly = 24,
    LciLinctabBleMultifunction5OutputReceiverAssembly = 25,
    LciIrRemoteControlAssembly = 26,
    LciMyrvHvacDualZoneControlUnitAssembly = 27,
    LciMultichannelLedControllerAssembly = 28,
    LciMyrvGeneratorGenieControlUnit = 29,
    LciMyrvHvacSingleZoneControlUnitAssembly = 30,
    LciMyrvHvacDualZoneControlUnitWithGenHourMeter = 31,
    LciMyrvHvacSingleZoneControlUnitWithGenHourMeter = 32,
    LciCanToEthernetGateway = 33,
    LciLevelUpUnityControlAssy = 34,
    Lci3ptClassCHydraulicLevelerAssembly = 35,
    LciMyrv5inOnecontrolTouchPanelAssembly = 36,
    LciMyrv7inOnecontrolTouchPanelAssembly = 37,
    LciMultifunctionOmega10Reversing4Latching = 38,
    LciTtLeveler4X3kCJackAssembly = 39,
    LciMyrvMotorized4ptHydraulicLevelerAssembly = 40,
    LciMyrvMotorized3ptHydraulicLevelerAssembly = 41,
    LciIpdmControllerAssembly = 42,
    LciTankMonitorV2ControlAssembly = 43,
    LciMyrvSmartArmAwningControlAssembly = 44,
    LciMyrv10inOnecontrolTouchPanelAssembly = 45,
    LciOnecontrolAndroidMobileApplication = 46,
    LciOnecontrolIosMobileApplication = 47,
    LciMyrvTtLevelerHdAssy = 48,
    LciMyrvTtLevelerSeAssy = 49,
    LciMyrvTtLevelerLtAssy = 50,
    LciMyrv5w6ptGc30LevelerType3Assy = 51,
    LciMyrv5w4ptGc30LevelerType3Assy = 52,
    LciMyrvSmartPowerTongueJackControlAssy = 53,
    LciMultifunctionOmega8Reversing4Latching = 54,
    LciMultifunctionOmega6Reversing4Latching = 55,
    LciMyrvMultifunction5OutputReceiverAssembly20a = 56,
    LciMyrvTtLevelerS3kGc = 57,
    LciMyrvTtLevelerS2kGc = 58,
    LciMyrvTtLevelerS3k3k = 59,
    LciMyrvTtLevelerS2k3k = 60,
    LciMyrvTtLevelerS2k2k = 61,
    LciLedLightingController811OutputCanOnlyAssembly = 62,
    LciLedLightingController8OutputAssembly = 63,
    LciLedLightingController81OutputCanOnlyAssembly = 64,
    LciLedLightingController8OutputCanOnlyAssembly = 65,
    LciMyrvTtLevelerM3k3kWLcdTouchpadAssy = 66,
    Lci9ZoneLedLightingControl8Dimming1LatchingAssembly = 67,
    LciEuroslideAssembly = 68,
    SimulatedProduct = 69,
    IdsCanSniffer = 70,
    LciMyrvTtLevelerM2kGcWLcdTpAsssembly = 71,
    LciMyrvTtLevelerM2k3kWLcdTpAssembly = 72,
    LciMyrvTtLevelerS3kGcWLcdTpAssembly = 73,
    LciMyrvTtLevelerS2kGcWLcdTpAssembly = 74,
    LciMyrvTtLevelerS3k3kWLcdTpAssembly = 75,
    LciMyrvTtLevelerS2k3kWLcdTpAssembly = 76,
    LciMyrvTtLevelerM2kGcAssembly = 77,
    LciMyrvTtLevelerM2k3kAssembly = 78,
    ClassCStabilizerControl2kCJacksAssembly = 79,
    MyrvInWallSlideControllerAssembly = 80,
    MyrvHvacSingleZoneControlWithAutoStartGenHourMeter = 81,
    MyrvHvacDualZoneControlWithAutoStartGenHourMeter = 82,
    MyrvAutoStartGeneratorGenieControlUnit = 83,
    MyrvPgInWallSlideControllerAssemblyTowableWManualProgram = 84,
    MyrvPgInWallSlideControllerAssemblyMotorizedWAutoProgram = 85,
    MyrvPgInWallSlideControllerAssemblyMotorizedWManualProgram = 86,
    MyrvHvacV2SingleZoneControlWithAutoStartGenHourMeter = 87,
    MyrvHvacV2DualZoneControlWithAutoStartGenHourMeter = 88,
    MyrvHvacV2TripleZoneControlWithAutoStartGenHourMeter = 89,
    MyrvHvacV2SingleZoneControl = 90,
    MyrvHvacV2DualZoneControl = 91,
    MyrvHvacV2TripleZoneControl = 92,
    MultifunctionUnityPartialAssembly1 = 93,
    MultifunctionUnityPartialAssembly2 = 94,
    MultifunctionUnityPartialAssembly3 = 95,
    MyrvSwitchBlock8bv001Assembly = 96,
    OnecontrolCloudGatewayAssembly = 97,
    CanToEthernetGateway12vOut = 98,
    MyrvClassCLevelerAssembly = 99,
    MyrvBluetoothGatewayAssembly = 100,
    MultifunctionUberPartialAssembly1RelayControl = 101,
    MultifunctionUberPartialAssembly2HvacControl = 102,
    MultifunctionUberPartialAssembly3LightingControl = 103,
    MultifunctionBase5OutElectricalBleAssembly = 104,
    MultifunctionBase5OutElectricalNonBleAssembly = 105,
    MultifunctionBase8OutElectricalBleAssembly = 106,
    MultifunctionBase8OutElectricalNonBleAssembly = 107,
    MultifunctionBase5OutHydraulicBleAssembly = 108,
    MultifunctionBase5OutHydraulicNonBleAssembly = 109,
    MultifunctionBase8OutHydraulicBleAssembly = 110,
    MultifunctionBase8OutHydraulicNonBleAssembly = 111,
    LciLinctab2MotorVelocitySyncInwallSlideControlAssemblyOption2 = 112,
    LciLinctab2MotorVelocitySyncInwallSlideControlAssemblyOption3 = 113,
    LciLinctab2MotorVelocitySyncInwallSlideControlAssemblyOption4 = 114,
    AndroidDevice = 115,
    IosDevice = 116,
    WindowsDevice = 117,
    MyrvRvCThermostatControl = 118,
    LciOnecontrol2MotorVelocitySyncInwallSlideControlAssemblyOption2 = 119,
    LciOnecontrol2MotorVelocitySyncInwallSlideControlAssemblyOption3 = 120,
    LciOnecontrol2MotorVelocitySyncInwallSlideControlAssemblyOption4 = 121,
    MultifunctionUnityM25PartialAssembly1RelayControl = 122,
    MultifunctionUnityM25PartialAssembly2HvacControl = 123,
    MultifunctionUnityM25PartialAssembly3TankMonitor = 124,
    MultifunctionUnityM3PartialAssembly1RelayControl = 125,
    MultifunctionUnityM3PartialAssembly2HvacControl = 126,
    MultifunctionUnityX3PartialAssembly1RelayControl = 127,
    MultifunctionUnityX3PartialAssembly2HvacControl = 128,
    LciTtM5k5kLevelerFinalAssy = 129,
    LciTtS5k5kLevelerFinalAssy = 130,
    OnecontrolFifthWheelLeveler6ptGc30V2Assembly = 131,
    LevelUpUnityKneelingAxleLevelerAssembly = 132,
    LciOnecontrol5w6ptGc30LevelerType3Assembly = 133,
    LciOnecontrol5w4ptGc30LevelerType3Assembly = 134,
    MultifunctionUnityM3tPartialAssembly1RelayControl = 135,
    MultifunctionUnityM3tPartialAssembly2HvacControl = 136,
    MultifunctionUnityX2Assembly = 137,
    BluetoothGatewayDaughterBoardXtAssembly = 138,
    MultifunctionUnityX1Assembly = 139,
    OnecontrolLevelUpAdvantageAssy = 140,
    OnecontrolGc30Advantage6ptFifthWheelLevelerAssy = 141,
    OnecontrolGc30Advantage4ptFifthWheelLevelerAssy = 142,
    LciOnecontrolRvCLevelerControlAssembly = 143,
    MultifunctionUnityX0Assembly = 144,
    MultifunctionUnityX15Assembly = 145,
    MultifunctionUnityX1HdJaycoAssembly = 146,
    AquafiHotspotAssembly = 147,
    CellularGatewayAssembly = 148,
    OnecontrolHotspotAssembly = 149,
    BluetoothGatewayDaughterBoardEsp32ProgrammedPcba = 150,
    OnecontrolLevelUpAdvantageSlideAssy = 151,
    TpmsTireLinc = 152,
    OnecontrolMotorized4ptHydraulicLeveler = 153,
    OnecontrolTtLevelerAdvantageS3k3kAssembly = 154,
    OnecontrolTtLevelerAdvantageS3k5kAssembly = 155,
    OnecontrolTtLevelerAdvantageS2k3kAssembly = 156,
    OnecontrolTtLevelerAdvantageS2k5kAssembly = 157,
    OnecontrolTtLevelerAdvantageS5k5kAssembly = 158,
    EuroSlideSmartRoom12voltV3 = 159,
    Onecontrol4ptTtLevelerAdvantageAssy = 160,
    Onecontrol4ptClassAAdvantageLevelerAssembly = 161,
    MonitorPanel6x9Assembly = 162,
    MultifunctionUnityX180tAssembly = 163,
    BluetoothGatewayDaughterBoardXtV2Pcba = 164,
    LciSureshadeIosMobileApplication = 165,
    LciSureshadeAndroidMobileApplication = 166,
    CameraRearObservationOemAssembly = 167,
    CameraRearObservationAftermarketAssembly = 168,
    Onecontrol3ptMotorizedAdvantageLevelerAssembly = 169,
    MultifunctionUnityX145Assembly = 170,
    OnecontrolBtGwPartialAssembly1Rs485Gw = 171,
    OnecontrolBtGwPartialAssembly2RvlinkTpmsGw = 172,
    OnecontrolBtGwPartialAssembly3AccessoryGw = 173,
    BottlecheckWirelessLpTankSensor = 174,
    DumpTrailerController2SwitchAssembly = 175,
    DumpTrailerController4SwitchAssembly = 176,
    CellularRouterGen2HotspotOnly = 177,
    CellularRouterGen2TelematicsOnly = 178,
    CellularRouterGen2HotspotWithTelematics = 179,
    OnecontrolTemperatureSensorBtAssembly = 180,
    UnityX260Assembly = 181,
    AbsSwayControllerAssembly = 182,
    LciMyrvRvCStandaloneThermostatAssembly = 183,
    BluetoothGatewayDaughterBoardRvlinkEsp32ProgrammedPcba = 184,
    PremiumMonitorPanelAssembly = 185,
    RvcHvacV2SingleZoneControlAssembly = 186,
    OnecontrolLevelUpAdvBtAl = 187,
    OnecontrolLevelUpAdvSlideOutputBtAl = 188,
    OnecontrolGc30Adv4pt5WheelLevelerBtAl = 189,
    OnecontrolGc30Adv6pt5WheelLevelerBtAl = 190,
    OnecontrolGc30HdAdv6pt5WheelLevelerBtAl = 191,
    OnecontrolGc30HdAdv6pt5WheelLevelerAl = 192,
    OnecontrolM3k3kTtAdvLevelerBtAl = 193,
    OnecontrolM3k5kTtAdvLevelerBtAl = 194,
    OnecontrolM5k5kTtAdvLevelerBtAl = 195,
    OnecontrolM3k3kTtAdvLevelerAl = 196,
    OnecontrolM3k5kTtAdvLevelerAl = 197,
    OnecontrolM5k5kTtAdvLevelerAl = 198,
    RvcHvacV2SingleZoneControlAlOption2 = 199,
    RvcHvacV2SingleZoneControlAlOption3 = 200,
    RvcHvacV2SingleZoneControlAlOption4 = 201,
    SwayCommand20ControllerAl = 202,
    CanReFlashBootloader = 203,
    DcBatteryMonitor = 204,
    LippertOneWindSensor = 205,
    FifthTankMonitorPanel = 206,
    Onecontrol4ptMotorizedTritonAdvantageLevelerBt = 207,
    Onecontrol5thWheelTritonAdvantageLevelerSlideBt = 208,
    Onecontrol3ptMotorizedTritonAdvantageLevelerBt = 209,
    LippertAmBtDoorLockAssembly = 210,
    UnityX270Assembly = 211,
}
