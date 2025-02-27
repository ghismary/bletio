//! Assigned numbers for company identifiers.
//!
//! FILE GENERATED FROM REVISION efc887857333456bda4ac2762443a7a86c99a27b OF THE BLUETOOTH SIG REPOSITORY, DO NOT EDIT!!!

use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::advertising::AdvertisingError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[num_enum(error_type(name = AdvertisingError, constructor = AdvertisingError::InvalidCompanyIdentifierValue))]
#[repr(u16)]
#[non_exhaustive]
/// Assigned numbers for company identifiers defined in
/// [Assigned Numbers, 7.1](https://bitbucket.org/bluetooth-SIG/public/src/main/assigned_numbers/company_identifiers/company_identifiers.yaml).
///
/// It is to be used when creating a Manufacturer Specific Data Advertising Structure.
/// See [ManufacturerSpecificDataAdStruct::try_new](crate::advertising::ad_struct::ManufacturerSpecificDataAdStruct::try_new).
pub enum CompanyIdentifier {
    /// Tactrix (0x0E94)
    Tactrix = 0x0E94,
    /// MIV ELECTRONICS, LTD (0x0E93)
    MivElectronicsLtd = 0x0E93,
    /// Glutz AG (0x0E92)
    GlutzAg = 0x0E92,
    /// Identita Inc. (0x0E91)
    IdentitaInc = 0x0E91,
    /// RainMaker Solutions, Inc. (0x0E90)
    RainMakerSolutionsInc = 0x0E90,
    /// Avetos Design LLC (0x0E8F)
    AvetosDesignLlc = 0x0E8F,
    /// Nobest Inc (0x0E8E)
    NobestInc = 0x0E8E,
    /// Celebrities Management Private Limited (0x0E8D)
    CelebritiesManagementPrivateLimited = 0x0E8D,
    /// Gopod Group Holding Limited (0x0E8C)
    GopodGroupHoldingLimited = 0x0E8C,
    /// Allgon AB (0x0E8B)
    AllgonAb = 0x0E8B,
    /// Tele-Radio i Lysekil AB (0x0E8A)
    TeleRadioILysekilAb = 0x0E8A,
    /// Brudden (0x0E89)
    Brudden = 0x0E89,
    /// Skewered Fencing, LLC (0x0E88)
    SkeweredFencingLlc = 0x0E88,
    /// OpenTech Alliance, Inc. (0x0E87)
    OpenTechAllianceInc = 0x0E87,
    /// Mercury Marine, a division of Brunswick Corporation (0x0E86)
    MercuryMarineADivisionOfBrunswickCorporation = 0x0E86,
    /// TigerLight, Inc. (0x0E85)
    TigerLightInc = 0x0E85,
    /// Tymphany HK Ltd (0x0E84)
    TymphanyHkLtd = 0x0E84,
    /// SPRiNTUS GmbH (0x0E83)
    SpRiNtusGmbH = 0x0E83,
    /// CHEVALIER TECH LIMITED (0x0E82)
    ChevalierTechLimited = 0x0E82,
    /// Guangdong Hengqin Xingtong Technology Co.,ltd. (0x0E81)
    GuangdongHengqinXingtongTechnologyCoLtd = 0x0E81,
    /// IQNEXXT Solutions GmbH (0x0E80)
    IqnexxtSolutionsGmbH = 0x0E80,
    /// SZR-Dev UG (0x0E7F)
    SzrDevUg = 0x0E7F,
    /// Archon Controls LLC (0x0E7E)
    ArchonControlsLlc = 0x0E7E,
    /// Jiangsu XinTongda Electric Technology Co.,Ltd. (0x0E7D)
    JiangsuXinTongdaElectricTechnologyCoLtd = 0x0E7D,
    /// final Inc. (0x0E7C)
    FinalInc = 0x0E7C,
    /// Circular (0x0E7B)
    Circular = 0x0E7B,
    /// Vivago Oy (0x0E7A)
    VivagoOy = 0x0E7A,
    /// Neptune First OU (0x0E79)
    NeptuneFirstOu = 0x0E79,
    /// HONG KONG COMMUNICATIONS COMPANY LIMITED (0x0E78)
    HongKongCommunicationsCompanyLimited = 0x0E78,
    /// MOBILE TECH, INC. (0x0E77)
    MobileTechInc = 0x0E77,
    /// Guangdong Nanguang Photo&Video Systems Co., Ltd. (0x0E76)
    GuangdongNanguangPhotoAndVideoSystemsCoLtd = 0x0E76,
    /// Le Touch (Shenzhen) Electronics Co., Ltd. (0x0E75)
    LeTouchShenzhenElectronicsCoLtd = 0x0E75,
    /// Rocky Radios LLC (0x0E74)
    RockyRadiosLlc = 0x0E74,
    /// Adventures of the Persistently Impaired (and other tales) Limited (0x0E73)
    AdventuresOfThePersistentlyImpairedAndOtherTalesLimited = 0x0E73,
    /// TOR.AI LIMITED (0x0E72)
    TorAiLimited = 0x0E72,
    /// ENABLEWEAR LLC (0x0E71)
    EnablewearLlc = 0x0E71,
    /// Powerstick.com (0x0E70)
    PowerstickCom = 0x0E70,
    /// OpConnect, Inc. (0x0E6F)
    OpConnectInc = 0x0E6F,
    /// I.M.LAB Inc (0x0E6E)
    IMLabInc = 0x0E6E,
    /// FEVOS LIMITED (0x0E6D)
    FevosLimited = 0x0E6D,
    /// RIGH, INC. (0x0E6C)
    RighInc = 0x0E6C,
    /// Shenzhen Goodocom Information Technology Co., Ltd. (0x0E6B)
    ShenzhenGoodocomInformationTechnologyCoLtd = 0x0E6B,
    /// Hyena Inc. (0x0E6A)
    HyenaInc = 0x0E6A,
    /// Megatronix (Beijing) Technology Co., Ltd (0x0E69)
    MegatronixBeijingTechnologyCoLtd = 0x0E69,
    /// EarTex Ltd (0x0E68)
    EarTexLtd = 0x0E68,
    /// NEXT DEVICES LTDA (0x0E67)
    NextDevicesLtda = 0x0E67,
    /// Shenzhen Baseus Technology Co., Ltd. (0x0E66)
    ShenzhenBaseusTechnologyCoLtd = 0x0E66,
    /// Daikin Industries, LTD (0x0E65)
    DaikinIndustriesLtd = 0x0E65,
    /// HuiTong intelligence Company Limited (0x0E64)
    HuiTongIntelligenceCompanyLimited = 0x0E64,
    /// LAST LOCK INC. (0x0E63)
    LastLockInc = 0x0E63,
    /// GOKI PTY LTD (0x0E62)
    GokiPtyLtd = 0x0E62,
    /// Queclink Wireless Solutions Co., Ltd. (0x0E61)
    QueclinkWirelessSolutionsCoLtd = 0x0E61,
    /// Ant Group Co., Ltd. (0x0E60)
    AntGroupCoLtd = 0x0E60,
    /// Ruptela (0x0E5F)
    Ruptela = 0x0E5F,
    /// SHAPER TOOLS, INC. (0x0E5E)
    ShaperToolsInc = 0x0E5E,
    /// L.T.H. Electronics Limited (0x0E5D)
    LTHElectronicsLimited = 0x0E5D,
    /// Amimon Ltd. (0x0E5C)
    AmimonLtd = 0x0E5C,
    /// Wuhu Hongjing Electronic Co.,Ltd (0x0E5B)
    WuhuHongjingElectronicCoLtd = 0x0E5B,
    /// OmniWave Microelectronics Shanghai Co., Ltd (0x0E5A)
    OmniWaveMicroelectronicsShanghaiCoLtd = 0x0E5A,
    /// Loewe Technology GmbH (0x0E59)
    LoeweTechnologyGmbH = 0x0E59,
    /// Urban Armor Gear, LLC (0x0E58)
    UrbanArmorGearLlc = 0x0E58,
    /// Altina Inc. (0x0E57)
    AltinaInc = 0x0E57,
    /// INEPRO Metering B.V. (0x0E56)
    IneproMeteringBV = 0x0E56,
    /// New Cosmos Electric Co., Ltd. (0x0E55)
    NewCosmosElectricCoLtd = 0x0E55,
    /// Relief Technologies AS (0x0E54)
    ReliefTechnologiesAs = 0x0E54,
    /// MINIRIG (0x0E53)
    Minirig = 0x0E53,
    /// Aquana, LLC (0x0E52)
    AquanaLlc = 0x0E52,
    /// Dyaco International Inc. (0x0E51)
    DyacoInternationalInc = 0x0E51,
    /// Zhejiang Desman Intelligent Technology Co., Ltd. (0x0E50)
    ZhejiangDesmanIntelligentTechnologyCoLtd = 0x0E50,
    /// eBet Gaming Sytems Pty Limited (0x0E4F)
    EBetGamingSytemsPtyLimited = 0x0E4F,
    /// QSC, LLC (0x0E4E)
    QscLlc = 0x0E4E,
    /// Brooksee, Inc. (0x0E4D)
    BrookseeInc = 0x0E4D,
    /// Luxshare Precision Industry Co., Ltd. (0x0E4C)
    LuxsharePrecisionIndustryCoLtd = 0x0E4C,
    /// PUDSEY DIAMOND ENGINEERING LIMITED (0x0E4B)
    PudseyDiamondEngineeringLimited = 0x0E4B,
    /// Nitto Denko Corporation (0x0E4A)
    NittoDenkoCorporation = 0x0E4A,
    /// Deone (Shanghai) Communication & Technology Co., Ltd (0x0E49)
    DeoneShanghaiCommunicationAndTechnologyCoLtd = 0x0E49,
    /// KARLUNA MUHENDISLIK SANAYI VE TICARET ANONIM SIRKETI (0x0E48)
    KarlunaMuhendislikSanayiVeTicaretAnonimSirketi = 0x0E48,
    /// Hosiden Besson Limited (0x0E47)
    HosidenBessonLimited = 0x0E47,
    /// SOUNDUCT (0x0E46)
    Sounduct = 0x0E46,
    /// Filo Srl (0x0E45)
    FiloSrl = 0x0E45,
    /// QUANTATEC (0x0E44)
    Quantatec = 0x0E44,
    /// InnoVision Medical Technologies, LLC (0x0E43)
    InnoVisionMedicalTechnologiesLlc = 0x0E43,
    /// Z-ONE Technology Co., Ltd. (0x0E42)
    ZOneTechnologyCoLtd = 0x0E42,
    /// Asustek Computer Inc. (0x0E41)
    AsustekComputerInc = 0x0E41,
    /// WKD Labs Ltd (0x0E40)
    WkdLabsLtd = 0x0E40,
    /// Wiser Devices, LLC (0x0E3F)
    WiserDevicesLlc = 0x0E3F,
    /// VANBOX (0x0E3E)
    Vanbox = 0x0E3E,
    /// Walmart Inc. (0x0E3D)
    WalmartInc = 0x0E3D,
    /// Viselabs (0x0E3C)
    Viselabs = 0x0E3C,
    /// Swift IOT Tech (Shenzhen) Co., LTD. (0x0E3B)
    SwiftIotTechShenzhenCoLtd = 0x0E3B,
    /// OFIVE LIMITED (0x0E3A)
    OfiveLimited = 0x0E3A,
    /// IRES Infrarot Energie Systeme GmbH (0x0E39)
    IresInfrarotEnergieSystemeGmbH = 0x0E39,
    /// SLOC GmbH (0x0E38)
    SlocGmbH = 0x0E38,
    /// CESYS Gesellschaft für angewandte Mikroelektronik mbH (0x0E37)
    CesysGesellschaftFürAngewandteMikroelektronikMbH = 0x0E37,
    /// Cousins and Sears LLC (0x0E36)
    CousinsAndSearsLlc = 0x0E36,
    /// SNAPPWISH LLC (0x0E35)
    SnappwishLlc = 0x0E35,
    /// Vermis, software solutions llc (0x0E34)
    VermisSoftwareSolutionsLlc = 0x0E34,
    /// Crescent NV (0x0E33)
    CrescentNv = 0x0E33,
    /// PACIFIC INDUSTRIAL CO., LTD. (0x0E32)
    PacificIndustrialCoLtd = 0x0E32,
    /// AlphaTheta Corporation (0x0E31)
    AlphaThetaCorporation = 0x0E31,
    /// Primax Electronics Ltd. (0x0E30)
    PrimaxElectronicsLtd = 0x0E30,
    /// ONWI (0x0E2F)
    Onwi = 0x0E2F,
    /// NIHON KOHDEN CORPORATION (0x0E2E)
    NihonKohdenCorporation = 0x0E2E,
    /// ECARX (Hubei) Tech Co.,Ltd. (0x0E2D)
    EcarxHubeiTechCoLtd = 0x0E2D,
    /// 9313-7263 Quebec inc. (0x0E2C)
    _93137263QuebecInc = 0x0E2C,
    /// JE electronic a/s (0x0E2B)
    JeElectronicAS = 0x0E2B,
    /// Huizhou Foryou General Electronics Co., Ltd. (0x0E2A)
    HuizhouForyouGeneralElectronicsCoLtd = 0x0E2A,
    /// Flipper Devices Inc. (0x0E29)
    FlipperDevicesInc = 0x0E29,
    /// PatchRx, Inc. (0x0E28)
    PatchRxInc = 0x0E28,
    /// NextSense, Inc. (0x0E27)
    NextSenseInc = 0x0E27,
    /// LIHJOEN SPEED METER CO., LTD. (0x0E26)
    LihjoenSpeedMeterCoLtd = 0x0E26,
    /// Hangzhou Hikvision Digital Technology Co., Ltd. (0x0E25)
    HangzhouHikvisionDigitalTechnologyCoLtd = 0x0E25,
    /// MS kajak7 UG (limited liability) (0x0E24)
    MsKajak7UgLimitedLiability = 0x0E24,
    /// HITO INC (0x0E22)
    HitoInc = 0x0E22,
    /// FOGO (0x0E21)
    Fogo = 0x0E21,
    /// CFLAB TEKNOLOJI TICARET LIMITED SIRKETI (0x0E20)
    CflabTeknolojiTicaretLimitedSirketi = 0x0E20,
    /// Blecon Ltd (0x0E1F)
    BleconLtd = 0x0E1F,
    /// 9512-5837 QUEBEC INC. (0x0E1E)
    _95125837QuebecInc = 0x0E1E,
    /// Capte B.V. (0x0E1D)
    CapteBV = 0x0E1D,
    /// SHENZHEN DIGITECH CO., LTD (0x0E1C)
    ShenzhenDigitechCoLtd = 0x0E1C,
    /// Time Location Systems AS (0x0E1B)
    TimeLocationSystemsAs = 0x0E1B,
    /// Sensovo GmbH (0x0E1A)
    SensovoGmbH = 0x0E1A,
    /// Hive-Zox International SA (0x0E19)
    HiveZoxInternationalSa = 0x0E19,
    /// Hangzhou Microimage Software Co.,Ltd. (0x0E18)
    HangzhouMicroimageSoftwareCoLtd = 0x0E18,
    /// Ad Hoc Electronics, llc. (0x0E17)
    AdHocElectronicsLlc = 0x0E17,
    /// Xiamen RUI YI Da Electronic Technology Co.,Ltd (0x0E16)
    XiamenRuiYiDaElectronicTechnologyCoLtd = 0x0E16,
    /// Eastern Partner Limited (0x0E15)
    EasternPartnerLimited = 0x0E15,
    /// Heilongjiang Tianyouwei Electronics Co.,Ltd. (0x0E14)
    HeilongjiangTianyouweiElectronicsCoLtd = 0x0E14,
    /// ASYSTOM (0x0E13)
    Asystom = 0x0E13,
    /// CLEVER LOGGER TECHNOLOGIES PTY LIMITED (0x0E12)
    CleverLoggerTechnologiesPtyLimited = 0x0E12,
    /// Wanzl GmbH & Co. KGaA (0x0E11)
    WanzlGmbHAndCoKGaA = 0x0E11,
    /// Eko Health, Inc. (0x0E10)
    EkoHealthInc = 0x0E10,
    /// SenseWorks Tecnologia Ltda. (0x0E0F)
    SenseWorksTecnologiaLtda = 0x0E0F,
    /// ALOGIC CORPORATION PTY LTD (0x0E0E)
    AlogicCorporationPtyLtd = 0x0E0E,
    /// FrontAct Co., Ltd. (0x0E0D)
    FrontActCoLtd = 0x0E0D,
    /// Yuanfeng Technology Co., Ltd. (0x0E0C)
    YuanfengTechnologyCoLtd = 0x0E0C,
    /// Sounding Audio Industrial Ltd. (0x0E0B)
    SoundingAudioIndustrialLtd = 0x0E0B,
    /// PRINT INTERNATIONAL LIMITED (0x0E0A)
    PrintInternationalLimited = 0x0E0A,
    /// Evolutive Systems SL (0x0E09)
    EvolutiveSystemsSl = 0x0E09,
    /// Heinrich Kopp GmbH (0x0E08)
    HeinrichKoppGmbH = 0x0E08,
    /// Pinpoint GmbH (0x0E07)
    PinpointGmbH = 0x0E07,
    /// iodyne, LLC (0x0E06)
    IodyneLlc = 0x0E06,
    /// SUREPULSE MEDICAL LIMITED (0x0E05)
    SurepulseMedicalLimited = 0x0E05,
    /// Doro AB (0x0E04)
    DoroAb = 0x0E04,
    /// Shenzhen eMeet technology Co.,Ltd (0x0E03)
    ShenzhenEMeetTechnologyCoLtd = 0x0E03,
    /// PROSYS DEV LIMITED (0x0E02)
    ProsysDevLimited = 0x0E02,
    /// Wuhu Mengbo Technology Co., Ltd. (0x0E01)
    WuhuMengboTechnologyCoLtd = 0x0E01,
    /// SHENZHEN SOUNDSOUL INFORMATION TECHNOLOGY CO.,LTD (0x0E00)
    ShenzhenSoundsoulInformationTechnologyCoLtd = 0x0E00,
    /// WaveRF, Corp. (0x0DFF)
    WaveRfCorp = 0x0DFF,
    /// Haptech, Inc. (0x0DFE)
    HaptechInc = 0x0DFE,
    /// BH Technologies (0x0DFD)
    BhTechnologies = 0x0DFD,
    /// SOJI ELECTRONICS JOINT STOCK COMPANY (0x0DFC)
    SojiElectronicsJointStockCompany = 0x0DFC,
    /// Beidou Intelligent Connected Vehicle Technology Co., Ltd. (0x0DFB)
    BeidouIntelligentConnectedVehicleTechnologyCoLtd = 0x0DFB,
    /// Shenzhen Matches IoT Technology Co., Ltd. (0x0DFA)
    ShenzhenMatchesIoTTechnologyCoLtd = 0x0DFA,
    /// Belusun Technology Ltd. (0x0DF9)
    BelusunTechnologyLtd = 0x0DF9,
    /// Chengdu CSCT Microelectronics Co., Ltd. (0x0DF8)
    ChengduCsctMicroelectronicsCoLtd = 0x0DF8,
    /// Hangzhou Zhaotong Microelectronics Co., Ltd. (0x0DF7)
    HangzhouZhaotongMicroelectronicsCoLtd = 0x0DF7,
    /// Mutrack Co., Ltd (0x0DF6)
    MutrackCoLtd = 0x0DF6,
    /// Delta Faucet Company (0x0DF5)
    DeltaFaucetCompany = 0x0DF5,
    /// REEKON TOOLS INC. (0x0DF4)
    ReekonToolsInc = 0x0DF4,
    /// hDrop Technologies Inc. (0x0DF3)
    HDropTechnologiesInc = 0x0DF3,
    /// Weber-Stephen Products LLC (0x0DF2)
    WeberStephenProductsLlc = 0x0DF2,
    /// iFLYTEK (Suzhou) Technology Co., Ltd. (0x0DF1)
    IFlytekSuzhouTechnologyCoLtd = 0x0DF1,
    /// Woncan (Hong Kong) Limited (0x0DF0)
    WoncanHongKongLimited = 0x0DF0,
    /// SING SUN TECHNOLOGY (INTERNATIONAL) LIMITED (0x0DEF)
    SingSunTechnologyInternationalLimited = 0x0DEF,
    /// Safety Swim LLC (0x0DEE)
    SafetySwimLlc = 0x0DEE,
    /// Flextronic GmbH (0x0DED)
    FlextronicGmbH = 0x0DED,
    /// ATEGENOS PHARMACEUTICALS INC (0x0DEC)
    AtegenosPharmaceuticalsInc = 0x0DEC,
    /// Fitz Inc. (0x0DEB)
    FitzInc = 0x0DEB,
    /// Kathrein Solutions GmbH (0x0DEA)
    KathreinSolutionsGmbH = 0x0DEA,
    /// General Laser GmbH (0x0DE9)
    GeneralLaserGmbH = 0x0DE9,
    /// Vivint, Inc. (0x0DE8)
    VivintInc = 0x0DE8,
    /// Dendro Technologies, Inc. (0x0DE7)
    DendroTechnologiesInc = 0x0DE7,
    /// DEXATEK Technology LTD (0x0DE6)
    DexatekTechnologyLtd = 0x0DE6,
    /// Ehong Technology Co.,Ltd (0x0DE5)
    EhongTechnologyCoLtd = 0x0DE5,
    /// EMBEINT INC (0x0DE4)
    EmbeintInc = 0x0DE4,
    /// Shenzhen MODSEMI Co., Ltd (0x0DE3)
    ShenzhenModsemiCoLtd = 0x0DE3,
    /// TAMADIC Co., Ltd. (0x0DE2)
    TamadicCoLtd = 0x0DE2,
    /// Outshiny India Private Limited (0x0DE1)
    OutshinyIndiaPrivateLimited = 0x0DE1,
    /// KNOG PTY. LTD. (0x0DE0)
    KnogPtyLtd = 0x0DE0,
    /// Shenzhen Lunci Technology Co., Ltd (0x0DDF)
    ShenzhenLunciTechnologyCoLtd = 0x0DDF,
    /// SHENZHEN DNS INDUSTRIES CO., LTD. (0x0DDE)
    ShenzhenDnsIndustriesCoLtd = 0x0DDE,
    /// Tozoa LLC (0x0DDD)
    TozoaLlc = 0x0DDD,
    /// AUTHOR-ALARM, razvoj in prodaja avtomobilskih sistemov proti kraji, d.o.o. (0x0DDC)
    AuthorAlarmRazvojInProdajaAvtomobilskihSistemovProtiKrajiDOO = 0x0DDC,
    /// KAGA FEI Co., Ltd. (0x0DDB)
    KagaFeiCoLtd = 0x0DDB,
    /// Minebea Intec GmbH (0x0DDA)
    MinebeaIntecGmbH = 0x0DDA,
    /// SCHELL GmbH & Co. KG (0x0DD9)
    SchellGmbHAndCoKg = 0x0DD9,
    /// Granchip IoT Technology (Guangzhou) Co.,Ltd (0x0DD8)
    GranchipIoTTechnologyGuangzhouCoLtd = 0x0DD8,
    /// Xiant Technologies, Inc. (0x0DD7)
    XiantTechnologiesInc = 0x0DD7,
    /// MODULAR MEDICAL, INC. (0x0DD6)
    ModularMedicalInc = 0x0DD6,
    /// BEEPINGS (0x0DD5)
    Beepings = 0x0DD5,
    /// KOQOON GmbH & Co.KG (0x0DD4)
    KoqoonGmbHAndCoKg = 0x0DD4,
    /// Global Satellite Engineering (0x0DD3)
    GlobalSatelliteEngineering = 0x0DD3,
    /// Sitecom Europe B.V. (0x0DD2)
    SitecomEuropeBV = 0x0DD2,
    /// OrangeMicro Limited (0x0DD1)
    OrangeMicroLimited = 0x0DD1,
    /// ESNAH (0x0DD0)
    Esnah = 0x0DD0,
    /// KUBU SMART LIMITED (0x0DCF)
    KubuSmartLimited = 0x0DCF,
    /// NOVAFON - Electromedical devices limited liability company (0x0DCE)
    NovafonElectromedicalDevicesLimitedLiabilityCompany = 0x0DCE,
    /// Astra LED AG (0x0DCD)
    AstraLedAg = 0x0DCD,
    /// Trotec GmbH (0x0DCC)
    TrotecGmbH = 0x0DCC,
    /// GEOPH, LLC (0x0DCB)
    GeophLlc = 0x0DCB,
    /// Aptener Mechatronics Private Limited (0x0DCA)
    AptenerMechatronicsPrivateLimited = 0x0DCA,
    /// farmunited GmbH (0x0DC9)
    FarmunitedGmbH = 0x0DC9,
    /// ETO GRUPPE TECHNOLOGIES GmbH (0x0DC8)
    EtoGruppeTechnologiesGmbH = 0x0DC8,
    /// MORNINGSTAR FX PTE. LTD. (0x0DC7)
    MorningstarFxPteLtd = 0x0DC7,
    /// Levita (0x0DC6)
    Levita = 0x0DC6,
    /// DHL (0x0DC5)
    Dhl = 0x0DC5,
    /// Hangzhou NationalChip Science & Technology Co.,Ltd (0x0DC4)
    HangzhouNationalChipScienceAndTechnologyCoLtd = 0x0DC4,
    /// GEARBAC TECHNOLOGIES INC. (0x0DC3)
    GearbacTechnologiesInc = 0x0DC3,
    /// Cirrus Research plc (0x0DC2)
    CirrusResearchPlc = 0x0DC2,
    /// NACHI-FUJIKOSHI CORP. (0x0DC1)
    NachiFujikoshiCorp = 0x0DC1,
    /// Shenzhen Jahport Electronic Technology Co., Ltd. (0x0DC0)
    ShenzhenJahportElectronicTechnologyCoLtd = 0x0DC0,
    /// HWM-Water Limited (0x0DBF)
    HwmWaterLimited = 0x0DBF,
    /// HELLA GmbH & Co. KGaA (0x0DBE)
    HellaGmbHAndCoKGaA = 0x0DBE,
    /// MAATEL (0x0DBD)
    Maatel = 0x0DBD,
    /// Felion Technologies Company Limited (0x0DBC)
    FelionTechnologiesCompanyLimited = 0x0DBC,
    /// Nexis Link Technology Co., Ltd. (0x0DBB)
    NexisLinkTechnologyCoLtd = 0x0DBB,
    /// Veo Technologies ApS (0x0DBA)
    VeoTechnologiesApS = 0x0DBA,
    /// CompanyDeep Ltd (0x0DB9)
    CompanyDeepLtd = 0x0DB9,
    /// Shenzhen Chuangyuan Digital Technology Co., Ltd (0x0DB8)
    ShenzhenChuangyuanDigitalTechnologyCoLtd = 0x0DB8,
    /// Morningstar Corporation (0x0DB7)
    MorningstarCorporation = 0x0DB7,
    /// SAFEGUARD EQUIPMENT, INC. (0x0DB6)
    SafeguardEquipmentInc = 0x0DB6,
    /// Djup AB (0x0DB5)
    DjupAb = 0x0DB5,
    /// Franklin Control Systems (0x0DB4)
    FranklinControlSystems = 0x0DB4,
    /// SHENZHEN REFLYING ELECTRONIC CO., LTD (0x0DB3)
    ShenzhenReflyingElectronicCoLtd = 0x0DB3,
    /// Whirlpool (0x0DB2)
    Whirlpool = 0x0DB2,
    /// TELE System Communications Pte. Ltd. (0x0DB1)
    TeleSystemCommunicationsPteLtd = 0x0DB1,
    /// Invisalert Solutions, Inc. (0x0DB0)
    InvisalertSolutionsInc = 0x0DB0,
    /// Hexagon Aura Reality AG (0x0DAF)
    HexagonAuraRealityAg = 0x0DAF,
    /// TITUM AUDIO, INC. (0x0DAE)
    TitumAudioInc = 0x0DAE,
    /// linktop (0x0DAD)
    Linktop = 0x0DAD,
    /// ITALTRACTOR ITM S.P.A. (0x0DAC)
    ItaltractorItmSPA = 0x0DAC,
    /// Efento (0x0DAB)
    Efento = 0x0DAB,
    /// Shenzhen EBELONG Technology Co., Ltd. (0x0DAA)
    ShenzhenEbelongTechnologyCoLtd = 0x0DAA,
    /// Inventronics GmbH (0x0DA9)
    InventronicsGmbH = 0x0DA9,
    /// Airwallet ApS (0x0DA8)
    AirwalletApS = 0x0DA8,
    /// Novoferm tormatic GmbH (0x0DA7)
    NovofermTormaticGmbH = 0x0DA7,
    /// Generac Corporation (0x0DA6)
    GeneracCorporation = 0x0DA6,
    /// PIXELA CORPORATION (0x0DA5)
    PixelaCorporation = 0x0DA5,
    /// HP Tuners (0x0DA4)
    HpTuners = 0x0DA4,
    /// Airgraft Inc. (0x0DA3)
    AirgraftInc = 0x0DA3,
    /// KIWI.KI GmbH (0x0DA2)
    KiwiKiGmbH = 0x0DA2,
    /// Fen Systems Ltd. (0x0DA1)
    FenSystemsLtd = 0x0DA1,
    /// SICK AG (0x0DA0)
    SickAg = 0x0DA0,
    /// MML US, Inc (0x0D9F)
    MmlUsInc = 0x0D9F,
    /// Impulse Wellness LLC (0x0D9E)
    ImpulseWellnessLlc = 0x0D9E,
    /// Cear, Inc. (0x0D9D)
    CearInc = 0x0D9D,
    /// Skytech Creations Limited (0x0D9C)
    SkytechCreationsLimited = 0x0D9C,
    /// Boxyz, Inc. (0x0D9B)
    BoxyzInc = 0x0D9B,
    /// Yeasound (Xiamen) Hearing Technology Co., Ltd (0x0D9A)
    YeasoundXiamenHearingTechnologyCoLtd = 0x0D9A,
    /// Caire Inc. (0x0D99)
    CaireInc = 0x0D99,
    /// E.F. Johnson Company (0x0D98)
    EFJohnsonCompany = 0x0D98,
    /// Zhejiang Huanfu Technology Co., LTD (0x0D97)
    ZhejiangHuanfuTechnologyCoLtd = 0x0D97,
    /// NEOKOHM SISTEMAS ELETRONICOS LTDA (0x0D96)
    NeokohmSistemasEletronicosLtda = 0x0D96,
    /// Hunter Industries Incorporated (0x0D95)
    HunterIndustriesIncorporated = 0x0D95,
    /// Shrooly Inc (0x0D94)
    ShroolyInc = 0x0D94,
    /// HagerEnergy GmbH (0x0D93)
    HagerEnergyGmbH = 0x0D93,
    /// TACHIKAWA CORPORATION (0x0D92)
    TachikawaCorporation = 0x0D92,
    /// Beamex Oy Ab (0x0D91)
    BeamexOyAb = 0x0D91,
    /// LAAS ApS (0x0D90)
    LaasApS = 0x0D90,
    /// Canon Electronics Inc. (0x0D8F)
    CanonElectronicsInc = 0x0D8F,
    /// Optivolt Labs, Inc. (0x0D8E)
    OptivoltLabsInc = 0x0D8E,
    /// RF Electronics Limited (0x0D8D)
    RfElectronicsLimited = 0x0D8D,
    /// Ultimea Technology (Shenzhen) Limited (0x0D8C)
    UltimeaTechnologyShenzhenLimited = 0x0D8C,
    /// Software Development, LLC (0x0D8B)
    SoftwareDevelopmentLlc = 0x0D8B,
    /// Simply Embedded Inc. (0x0D8A)
    SimplyEmbeddedInc = 0x0D8A,
    /// Nanohex Corp (0x0D89)
    NanohexCorp = 0x0D89,
    /// Geocene Inc. (0x0D88)
    GeoceneInc = 0x0D88,
    /// Quectel Wireless Solutions Co., Ltd. (0x0D87)
    QuectelWirelessSolutionsCoLtd = 0x0D87,
    /// ROCKWELL AUTOMATION, INC. (0x0D86)
    RockwellAutomationInc = 0x0D86,
    /// SEW-EURODRIVE GmbH & Co KG (0x0D85)
    SewEurodriveGmbHAndCoKg = 0x0D85,
    /// Testo SE & Co. KGaA (0x0D84)
    TestoSeAndCoKGaA = 0x0D84,
    /// ATLANTIC SOCIETE FRANCAISE DE DEVELOPPEMENT THERMIQUE (0x0D83)
    AtlanticSocieteFrancaiseDeDeveloppementThermique = 0x0D83,
    /// VELCO (0x0D82)
    Velco = 0x0D82,
    /// Beyerdynamic GmbH & Co. KG (0x0D81)
    BeyerdynamicGmbHAndCoKg = 0x0D81,
    /// Gravaa B.V. (0x0D80)
    GravaaBV = 0x0D80,
    /// Konova (0x0D7F)
    Konova = 0x0D7F,
    /// Daihatsu Motor Co., Ltd. (0x0D7E)
    DaihatsuMotorCoLtd = 0x0D7E,
    /// Taiko Audio B.V. (0x0D7D)
    TaikoAudioBV = 0x0D7D,
    /// BeiJing SmartChip Microelectronics Technology Co.,Ltd (0x0D7C)
    BeiJingSmartChipMicroelectronicsTechnologyCoLtd = 0x0D7C,
    /// MindMaze SA (0x0D7B)
    MindMazeSa = 0x0D7B,
    /// Xiamen Intretech Inc. (0x0D7A)
    XiamenIntretechInc = 0x0D7A,
    /// VIVIWARE JAPAN, Inc. (0x0D79)
    ViviwareJapanInc = 0x0D79,
    /// MITACHI CO.,LTD. (0x0D78)
    MitachiCoLtd = 0x0D78,
    /// DualNetworks SA (0x0D77)
    DualNetworksSa = 0x0D77,
    /// i-focus Co.,Ltd (0x0D76)
    IFocusCoLtd = 0x0D76,
    /// Indistinguishable From Magic, Inc. (0x0D75)
    IndistinguishableFromMagicInc = 0x0D75,
    /// ANUME s.r.o. (0x0D74)
    AnumeSRO = 0x0D74,
    /// iota Biosciences, Inc. (0x0D73)
    IotaBiosciencesInc = 0x0D73,
    /// Earfun Technology (HK) Limited (0x0D72)
    EarfunTechnologyHkLimited = 0x0D72,
    /// Kiteras Inc. (0x0D71)
    KiterasInc = 0x0D71,
    /// Kindhome (0x0D70)
    Kindhome = 0x0D70,
    /// Closed Joint Stock Company NVP BOLID (0x0D6F)
    ClosedJointStockCompanyNvpBolid = 0x0D6F,
    /// Look Cycle International (0x0D6E)
    LookCycleInternational = 0x0D6E,
    /// DYNAMOX S/A (0x0D6D)
    DynamoxSA = 0x0D6D,
    /// Ambient IoT Pty Ltd (0x0D6C)
    AmbientIoTPtyLtd = 0x0D6C,
    /// Crane Payment Innovations, Inc. (0x0D6B)
    CranePaymentInnovationsInc = 0x0D6B,
    /// Helge Kaiser GmbH (0x0D6A)
    HelgeKaiserGmbH = 0x0D6A,
    /// AIR AROMA INTERNATIONAL PTY LTD (0x0D69)
    AirAromaInternationalPtyLtd = 0x0D69,
    /// Status Audio LLC (0x0D68)
    StatusAudioLlc = 0x0D68,
    /// BLACK BOX NETWORK SERVICES INDIA PRIVATE LIMITED (0x0D67)
    BlackBoxNetworkServicesIndiaPrivateLimited = 0x0D67,
    /// Hendrickson USA , L.L.C (0x0D66)
    HendricksonUsaLLC = 0x0D66,
    /// Molnlycke Health Care AB (0x0D65)
    MolnlyckeHealthCareAb = 0x0D65,
    /// Southco (0x0D64)
    Southco = 0x0D64,
    /// SKF France (0x0D63)
    SkfFrance = 0x0D63,
    /// MEBSTER s.r.o. (0x0D62)
    MebsterSRO = 0x0D62,
    /// F.I.P. FORMATURA INIEZIONE POLIMERI - S.P.A. (0x0D61)
    FIPFormaturaIniezionePolimeriSPA = 0x0D61,
    /// Smart Products Connection, S.A. (0x0D60)
    SmartProductsConnectionSA = 0x0D60,
    /// SiChuan Homme Intelligent Technology co.,Ltd. (0x0D5F)
    SiChuanHommeIntelligentTechnologyCoLtd = 0x0D5F,
    /// Pella Corp (0x0D5E)
    PellaCorp = 0x0D5E,
    /// Stogger B.V. (0x0D5D)
    StoggerBV = 0x0D5D,
    /// Pison Technology, Inc. (0x0D5C)
    PisonTechnologyInc = 0x0D5C,
    /// Axis Communications AB (0x0D5B)
    AxisCommunicationsAb = 0x0D5B,
    /// Gunnebo Aktiebolag (0x0D5A)
    GunneboAktiebolag = 0x0D5A,
    /// HYUPSUNG MACHINERY ELECTRIC CO., LTD. (0x0D59)
    HyupsungMachineryElectricCoLtd = 0x0D59,
    /// ifm electronic gmbh (0x0D58)
    IfmElectronicGmbh = 0x0D58,
    /// Nanjing Xinxiangyuan Microelectronics Co., Ltd. (0x0D57)
    NanjingXinxiangyuanMicroelectronicsCoLtd = 0x0D57,
    /// Wellang.Co,.Ltd (0x0D56)
    WellangCoLtd = 0x0D56,
    /// NO CLIMB PRODUCTS LTD (0x0D55)
    NoClimbProductsLtd = 0x0D55,
    /// ISEKI FRANCE S.A.S (0x0D54)
    IsekiFranceSAS = 0x0D54,
    /// Luxottica Group S.p.A (0x0D53)
    LuxotticaGroupSPA = 0x0D53,
    /// DIVAN TRADING CO., LTD. (0x0D52)
    DivanTradingCoLtd = 0x0D52,
    /// Genetus inc. (0x0D51)
    GenetusInc = 0x0D51,
    /// NINGBO FOTILE KITCHENWARE CO., LTD. (0x0D50)
    NingboFotileKitchenwareCoLtd = 0x0D50,
    /// Movano Inc. (0x0D4F)
    MovanoInc = 0x0D4F,
    /// NIKAT SOLUTIONS PRIVATE LIMITED (0x0D4E)
    NikatSolutionsPrivateLimited = 0x0D4E,
    /// Optec, LLC (0x0D4D)
    OptecLlc = 0x0D4D,
    /// IotGizmo Corporation (0x0D4C)
    IotGizmoCorporation = 0x0D4C,
    /// Soundwave Hearing, LLC (0x0D4B)
    SoundwaveHearingLlc = 0x0D4B,
    /// Rockpile Solutions, LLC (0x0D4A)
    RockpileSolutionsLlc = 0x0D4A,
    /// Refrigerated Transport Electronics, Inc. (0x0D49)
    RefrigeratedTransportElectronicsInc = 0x0D49,
    /// Vemcon GmbH (0x0D48)
    VemconGmbH = 0x0D48,
    /// Shenzhen DOKE Electronic Co., Ltd (0x0D47)
    ShenzhenDokeElectronicCoLtd = 0x0D47,
    /// Thales Simulation & Training AG (0x0D46)
    ThalesSimulationAndTrainingAg = 0x0D46,
    /// Odeon, Inc. (0x0D45)
    OdeonInc = 0x0D45,
    /// Ex Makhina Inc. (0x0D44)
    ExMakhinaInc = 0x0D44,
    /// Gosuncn Technology Group Co., Ltd. (0x0D43)
    GosuncnTechnologyGroupCoLtd = 0x0D43,
    /// TEKTRO TECHNOLOGY CORPORATION (0x0D42)
    TektroTechnologyCorporation = 0x0D42,
    /// CPAC Systems AB (0x0D41)
    CpacSystemsAb = 0x0D41,
    /// SignalFire Telemetry, Inc. (0x0D40)
    SignalFireTelemetryInc = 0x0D40,
    /// Vogels Products B.V. (0x0D3F)
    VogelsProductsBV = 0x0D3F,
    /// LUMINOAH, INC. (0x0D3E)
    LuminoahInc = 0x0D3E,
    /// bHaptics Inc. (0x0D3D)
    BHapticsInc = 0x0D3D,
    /// SIRONA Dental Systems GmbH (0x0D3C)
    SironaDentalSystemsGmbH = 0x0D3C,
    /// Lone Star Marine Pty Ltd (0x0D3B)
    LoneStarMarinePtyLtd = 0x0D3B,
    /// Frost Solutions, LLC (0x0D3A)
    FrostSolutionsLlc = 0x0D3A,
    /// Systemic Games, LLC (0x0D39)
    SystemicGamesLlc = 0x0D39,
    /// CycLock (0x0D38)
    CycLock = 0x0D38,
    /// Zerene Inc. (0x0D37)
    ZereneInc = 0x0D37,
    /// XIHAO INTELLIGENGT TECHNOLOGY CO., LTD (0x0D36)
    XihaoIntelligengtTechnologyCoLtd = 0x0D36,
    /// Universidad Politecnica de Madrid (0x0D35)
    UniversidadPolitecnicaDeMadrid = 0x0D35,
    /// ZILLIOT TECHNOLOGIES PRIVATE LIMITED (0x0D34)
    ZilliotTechnologiesPrivateLimited = 0x0D34,
    /// Micropower Group AB (0x0D33)
    MicropowerGroupAb = 0x0D33,
    /// Badger Meter (0x0D32)
    BadgerMeter = 0x0D32,
    /// SYNCHRON, INC. (0x0D31)
    SynchronInc = 0x0D31,
    /// Laxmi Therapeutic Devices, Inc. (0x0D30)
    LaxmiTherapeuticDevicesInc = 0x0D30,
    /// Delta Development Team, Inc (0x0D2F)
    DeltaDevelopmentTeamInc = 0x0D2F,
    /// Advanced Electronic Applications, Inc (0x0D2E)
    AdvancedElectronicApplicationsInc = 0x0D2E,
    /// Cooler Pro, LLC (0x0D2D)
    CoolerProLlc = 0x0D2D,
    /// SIL System Integration Laboratory GmbH (0x0D2C)
    SilSystemIntegrationLaboratoryGmbH = 0x0D2C,
    /// Sensoryx AG (0x0D2B)
    SensoryxAg = 0x0D2B,
    /// PhysioLogic Devices, Inc. (0x0D2A)
    PhysioLogicDevicesInc = 0x0D2A,
    /// MIYAKAWA ELECTRIC WORKS LTD. (0x0D29)
    MiyakawaElectricWorksLtd = 0x0D29,
    /// FUJITSU COMPONENT LIMITED (0x0D28)
    FujitsuComponentLimited = 0x0D28,
    /// velocitux (0x0D27)
    Velocitux = 0x0D27,
    /// Burkert Werke GmbH & Co. KG (0x0D26)
    BurkertWerkeGmbHAndCoKg = 0x0D26,
    /// System Elite Holdings Group Limited (0x0D25)
    SystemEliteHoldingsGroupLimited = 0x0D25,
    /// Japan Display Inc. (0x0D24)
    JapanDisplayInc = 0x0D24,
    /// GREE Electric Appliances, Inc. of Zhuhai (0x0D23)
    GreeElectricAppliancesIncOfZhuhai = 0x0D23,
    /// Cedarware, Corp. (0x0D22)
    CedarwareCorp = 0x0D22,
    /// Cennox Group Limited (0x0D21)
    CennoxGroupLimited = 0x0D21,
    /// SCIENTERRA LIMITED (0x0D20)
    ScienterraLimited = 0x0D20,
    /// Synkopi, Inc. (0x0D1F)
    SynkopiInc = 0x0D1F,
    /// FESTINA LOTUS SA (0x0D1E)
    FestinaLotusSa = 0x0D1E,
    /// Electronics4All Inc. (0x0D1D)
    Electronics4AllInc = 0x0D1D,
    /// LIMBOID LLC (0x0D1C)
    LimboidLlc = 0x0D1C,
    /// RACHIO, INC. (0x0D1B)
    RachioInc = 0x0D1B,
    /// Maturix ApS (0x0D1A)
    MaturixApS = 0x0D1A,
    /// C.G. Air Systemes Inc. (0x0D19)
    CGAirSystemesInc = 0x0D19,
    /// Bioliberty Ltd (0x0D18)
    BiolibertyLtd = 0x0D18,
    /// Akix S.r.l. (0x0D17)
    AkixSRL = 0x0D17,
    /// Nations Technologies Inc. (0x0D16)
    NationsTechnologiesInc = 0x0D16,
    /// Spark (0x0D15)
    Spark = 0x0D15,
    /// Merry Electronics (S) Pte Ltd (0x0D14)
    MerryElectronicsSPteLtd = 0x0D14,
    /// MERRY ELECTRONICS CO., LTD. (0x0D13)
    MerryElectronicsCoLtd = 0x0D13,
    /// Spartek Systems Inc. (0x0D12)
    SpartekSystemsInc = 0x0D12,
    /// Great Dane LLC (0x0D11)
    GreatDaneLlc = 0x0D11,
    /// JVC KENWOOD Corporation (0x0D10)
    JvcKenwoodCorporation = 0x0D10,
    /// Timebirds Australia Pty Ltd (0x0D0F)
    TimebirdsAustraliaPtyLtd = 0x0D0F,
    /// PetVoice Co., Ltd. (0x0D0E)
    PetVoiceCoLtd = 0x0D0E,
    /// C.Ed. Schulte GmbH Zylinderschlossfabrik (0x0D0D)
    CEdSchulteGmbHZylinderschlossfabrik = 0x0D0D,
    /// Planmeca Oy (0x0D0C)
    PlanmecaOy = 0x0D0C,
    /// Research Products Corporation (0x0D0B)
    ResearchProductsCorporation = 0x0D0B,
    /// CATEYE Co., Ltd. (0x0D0A)
    CateyeCoLtd = 0x0D0A,
    /// Leica Geosystems AG (0x0D09)
    LeicaGeosystemsAg = 0x0D09,
    /// Datalogic USA, Inc. (0x0D08)
    DatalogicUsaInc = 0x0D08,
    /// Datalogic S.r.l. (0x0D07)
    DatalogicSRL = 0x0D07,
    /// doubleO Co., Ltd. (0x0D06)
    DoubleOCoLtd = 0x0D06,
    /// Energy Technology and Control Limited (0x0D05)
    EnergyTechnologyAndControlLimited = 0x0D05,
    /// Bartec Auto Id Ltd (0x0D04)
    BartecAutoIdLtd = 0x0D04,
    /// MakuSafe Corp (0x0D03)
    MakuSafeCorp = 0x0D03,
    /// Rocky Mountain ATV/MC Jake Wilson (0x0D02)
    RockyMountainAtvMcJakeWilson = 0x0D02,
    /// KEEPEN (0x0D01)
    Keepen = 0x0D01,
    /// Sparkpark AS (0x0D00)
    SparkparkAs = 0x0D00,
    /// Ergodriven Inc (0x0CFF)
    ErgodrivenInc = 0x0CFF,
    /// Thule Group AB (0x0CFE)
    ThuleGroupAb = 0x0CFE,
    /// Wuhan Woncan Construction Technologies Co., Ltd. (0x0CFD)
    WuhanWoncanConstructionTechnologiesCoLtd = 0x0CFD,
    /// ElectronX design (0x0CFC)
    ElectronXDesign = 0x0CFC,
    /// Tyromotion GmbH (0x0CFB)
    TyromotionGmbH = 0x0CFB,
    /// Protect Animals With Satellites LLC (0x0CFA)
    ProtectAnimalsWithSatellitesLlc = 0x0CFA,
    /// Tamblue Oy (0x0CF9)
    TamblueOy = 0x0CF9,
    /// core sensing GmbH (0x0CF8)
    CoreSensingGmbH = 0x0CF8,
    /// TVS Motor Company Ltd. (0x0CF7)
    TvsMotorCompanyLtd = 0x0CF7,
    /// OJ Electronics A/S (0x0CF6)
    OjElectronicsAS = 0x0CF6,
    /// BOS Balance of Storage Systems AG (0x0CF5)
    BosBalanceOfStorageSystemsAg = 0x0CF5,
    /// SOLUX PTY LTD (0x0CF4)
    SoluxPtyLtd = 0x0CF4,
    /// Radio Sound (0x0CF3)
    RadioSound = 0x0CF3,
    /// BestSens AG (0x0CF2)
    BestSensAg = 0x0CF2,
    /// Midmark (0x0CF1)
    Midmark = 0x0CF1,
    /// THOTAKA TEKHNOLOGIES INDIA PRIVATE LIMITED (0x0CF0)
    ThotakaTekhnologiesIndiaPrivateLimited = 0x0CF0,
    /// POGS B.V. (0x0CEF)
    PogsBV = 0x0CEF,
    /// MadgeTech, Inc (0x0CEE)
    MadgeTechInc = 0x0CEE,
    /// CV. NURI TEKNIK (0x0CED)
    CvNuriTeknik = 0x0CED,
    /// Pacific Coast Fishery Services (2003) Inc. (0x0CEC)
    PacificCoastFisheryServices2003Inc = 0x0CEC,
    /// Shenzhen Tingting Technology Co. LTD (0x0CEB)
    ShenzhenTingtingTechnologyCoLtd = 0x0CEB,
    /// HAYWARD INDUSTRIES, INC. (0x0CEA)
    HaywardIndustriesInc = 0x0CEA,
    /// PEAG, LLC dba JLab Audio (0x0CE9)
    PeagLlcDbaJLabAudio = 0x0CE9,
    /// Dongguan Yougo Electronics Co.,Ltd. (0x0CE8)
    DongguanYougoElectronicsCoLtd = 0x0CE8,
    /// TAG HEUER SA (0x0CE7)
    TagHeuerSa = 0x0CE7,
    /// McWong International, Inc. (0x0CE6)
    McWongInternationalInc = 0x0CE6,
    /// Amina Distribution AS (0x0CE5)
    AminaDistributionAs = 0x0CE5,
    /// Off-Highway Powertrain Services Germany GmbH (0x0CE4)
    OffHighwayPowertrainServicesGermanyGmbH = 0x0CE4,
    /// Taiwan Fuhsing (0x0CE3)
    TaiwanFuhsing = 0x0CE3,
    /// CORVENT MEDICAL, INC. (0x0CE2)
    CorventMedicalInc = 0x0CE2,
    /// Regal Beloit America, Inc. (0x0CE1)
    RegalBeloitAmericaInc = 0x0CE1,
    /// VODALOGIC PTY LTD (0x0CE0)
    VodalogicPtyLtd = 0x0CE0,
    /// SHENZHEN CHENYUN ELECTRONICS  CO., LTD (0x0CDF)
    ShenzhenChenyunElectronicsCoLtd = 0x0CDF,
    /// RESPONSE TECHNOLOGIES, LTD. (0x0CDE)
    ResponseTechnologiesLtd = 0x0CDE,
    /// Alif Semiconductor, Inc. (0x0CDD)
    AlifSemiconductorInc = 0x0CDD,
    /// Ypsomed AG (0x0CDC)
    YpsomedAg = 0x0CDC,
    /// Circus World Displays Limited (0x0CDB)
    CircusWorldDisplaysLimited = 0x0CDB,
    /// Wolf Steel ltd (0x0CDA)
    WolfSteelLtd = 0x0CDA,
    /// Minami acoustics Limited (0x0CD9)
    MinamiAcousticsLimited = 0x0CD9,
    /// SIA Mesh Group (0x0CD8)
    SiaMeshGroup = 0x0CD8,
    /// Maztech Industries, LLC (0x0CD7)
    MaztechIndustriesLlc = 0x0CD7,
    /// HHO (Hangzhou) Digital Technology Co., Ltd. (0x0CD6)
    HhoHangzhouDigitalTechnologyCoLtd = 0x0CD6,
    /// Numa Products, LLC (0x0CD5)
    NumaProductsLlc = 0x0CD5,
    /// Reoqoo IoT Technology Co., Ltd. (0x0CD4)
    ReoqooIoTTechnologyCoLtd = 0x0CD4,
    /// TechSwipe (0x0CD3)
    TechSwipe = 0x0CD3,
    /// EQOM SSC B.V. (0x0CD2)
    EqomSscBV = 0x0CD2,
    /// Imagine Marketing Limited (0x0CD1)
    ImagineMarketingLimited = 0x0CD1,
    /// MooreSilicon Semiconductor Technology (Shanghai) Co., LTD. (0x0CD0)
    MooreSiliconSemiconductorTechnologyShanghaiCoLtd = 0x0CD0,
    /// Shenzhen CESI Information Technology Co., Ltd. (0x0CCF)
    ShenzhenCesiInformationTechnologyCoLtd = 0x0CCF,
    /// SENOSPACE LLC (0x0CCE)
    SenospaceLlc = 0x0CCE,
    /// YanFeng Visteon(Chongqing) Automotive Electronic Co.,Ltd (0x0CCD)
    YanFengVisteonChongqingAutomotiveElectronicCoLtd = 0x0CCD,
    /// Kord Defence Pty Ltd (0x0CCC)
    KordDefencePtyLtd = 0x0CCC,
    /// NOTHING TECHNOLOGY LIMITED (0x0CCB)
    NothingTechnologyLimited = 0x0CCB,
    /// Cyclops Marine Ltd (0x0CCA)
    CyclopsMarineLtd = 0x0CCA,
    /// Innocent Technology Co., Ltd. (0x0CC9)
    InnocentTechnologyCoLtd = 0x0CC9,
    /// TrikThom (0x0CC8)
    TrikThom = 0x0CC8,
    /// SB C&S Corp. (0x0CC7)
    SbCAndSCorp = 0x0CC7,
    /// Serial Technology Corporation (0x0CC6)
    SerialTechnologyCorporation = 0x0CC6,
    /// Open Road Solutions, Inc. (0x0CC5)
    OpenRoadSolutionsInc = 0x0CC5,
    /// ABUS August Bremicker Soehne Kommanditgesellschaft (0x0CC4)
    AbusAugustBremickerSoehneKommanditgesellschaft = 0x0CC4,
    /// HMD Global Oy (0x0CC3)
    HmdGlobalOy = 0x0CC3,
    /// Anker Innovations Limited (0x0CC2)
    AnkerInnovationsLimited = 0x0CC2,
    /// CLEIO Inc. (0x0CC1)
    CleioInc = 0x0CC1,
    /// Garnet Instruments Ltd. (0x0CC0)
    GarnetInstrumentsLtd = 0x0CC0,
    /// Forward Thinking Systems LLC. (0x0CBF)
    ForwardThinkingSystemsLlc = 0x0CBF,
    /// Pricer AB (0x0CBD)
    PricerAb = 0x0CBD,
    /// TROX GmbH (0x0CBC)
    TroxGmbH = 0x0CBC,
    /// Emlid Tech Kft. (0x0CBB)
    EmlidTechKft = 0x0CBB,
    /// Ameso Tech (OPC) Private Limited (0x0CBA)
    AmesoTechOpcPrivateLimited = 0x0CBA,
    /// seca GmbH & Co. KG (0x0CB9)
    SecaGmbHAndCoKg = 0x0CB9,
    /// Shanghai Proxy Network Technology Co., Ltd. (0x0CB8)
    ShanghaiProxyNetworkTechnologyCoLtd = 0x0CB8,
    /// Cucumber Lighting Controls Limited (0x0CB7)
    CucumberLightingControlsLimited = 0x0CB7,
    /// THE EELECTRIC MACARON LLC (0x0CB6)
    TheEelectricMacaronLlc = 0x0CB6,
    /// Racketry, d. o. o. (0x0CB5)
    RacketryDOO = 0x0CB5,
    /// Eberspaecher Climate Control Systems GmbH (0x0CB4)
    EberspaecherClimateControlSystemsGmbH = 0x0CB4,
    /// janova GmbH (0x0CB3)
    JanovaGmbH = 0x0CB3,
    /// SHINKAWA Sensor Technology, Inc. (0x0CB2)
    ShinkawaSensorTechnologyInc = 0x0CB2,
    /// RF Creations (0x0CB1)
    RfCreations = 0x0CB1,
    /// SwipeSense, Inc. (0x0CB0)
    SwipeSenseInc = 0x0CB0,
    /// NEURINNOV (0x0CAF)
    Neurinnov = 0x0CAF,
    /// Evident Corporation (0x0CAE)
    EvidentCorporation = 0x0CAE,
    /// Shenzhen Openhearing Tech CO., LTD . (0x0CAD)
    ShenzhenOpenhearingTechCoLtd = 0x0CAD,
    /// Shenzhen Shokz Co.,Ltd. (0x0CAC)
    ShenzhenShokzCoLtd = 0x0CAC,
    /// HERUTU ELECTRONICS CORPORATION (0x0CAB)
    HerutuElectronicsCorporation = 0x0CAB,
    /// Shenzhen Poseidon Network Technology Co., Ltd (0x0CAA)
    ShenzhenPoseidonNetworkTechnologyCoLtd = 0x0CAA,
    /// Mievo Technologies Private Limited (0x0CA9)
    MievoTechnologiesPrivateLimited = 0x0CA9,
    /// Sonas, Inc. (0x0CA8)
    SonasInc = 0x0CA8,
    /// Verve InfoTec Pty Ltd (0x0CA7)
    VerveInfoTecPtyLtd = 0x0CA7,
    /// Megger Ltd (0x0CA6)
    MeggerLtd = 0x0CA6,
    /// Princess Cruise Lines, Ltd. (0x0CA5)
    PrincessCruiseLinesLtd = 0x0CA5,
    /// MITSUBISHI ELECTRIC LIGHTING CO, LTD (0x0CA4)
    MitsubishiElectricLightingCoLtd = 0x0CA4,
    /// MAQUET GmbH (0x0CA3)
    MaquetGmbH = 0x0CA3,
    /// XSENSE LTD (0x0CA2)
    XsenseLtd = 0x0CA2,
    /// YAMAHA MOTOR CO.,LTD. (0x0CA1)
    YamahaMotorCoLtd = 0x0CA1,
    /// BIGBEN (0x0CA0)
    Bigben = 0x0CA0,
    /// Dragonfly Energy Corp. (0x0C9F)
    DragonflyEnergyCorp = 0x0C9F,
    /// ECCEL CORPORATION SAS (0x0C9E)
    EccelCorporationSas = 0x0C9E,
    /// Ribbiot, INC. (0x0C9D)
    RibbiotInc = 0x0C9D,
    /// Sunstone-RTLS Ipari Szolgaltato Korlatolt Felelossegu Tarsasag (0x0C9C)
    SunstoneRtlsIpariSzolgaltatoKorlatoltFelelosseguTarsasag = 0x0C9C,
    /// NTT sonority, Inc. (0x0C9B)
    NttSonorityInc = 0x0C9B,
    /// ALF Inc. (0x0C9A)
    AlfInc = 0x0C9A,
    /// Vire Health Oy (0x0C99)
    VireHealthOy = 0x0C99,
    /// MiX Telematics International (PTY) LTD (0x0C98)
    MiXTelematicsInternationalPtyLtd = 0x0C98,
    /// Deako (0x0C97)
    Deako = 0x0C97,
    /// H+B Hightech GmbH (0x0C96)
    HBHightechGmbH = 0x0C96,
    /// Gemstone Lights Canada Ltd. (0x0C95)
    GemstoneLightsCanadaLtd = 0x0C95,
    /// Baxter Healthcare Corporation (0x0C94)
    BaxterHealthcareCorporation = 0x0C94,
    /// Movesense Oy (0x0C93)
    MovesenseOy = 0x0C93,
    /// Kesseböhmer Ergonomietechnik GmbH (0x0C92)
    KesseböhmerErgonomietechnikGmbH = 0x0C92,
    /// Yashu Systems (0x0C91)
    YashuSystems = 0x0C91,
    /// WESCO AG (0x0C90)
    WescoAg = 0x0C90,
    /// Radar Automobile Sales(Shandong)Co.,Ltd. (0x0C8F)
    RadarAutomobileSalesShandongCoLtd = 0x0C8F,
    /// Technocon Engineering Ltd. (0x0C8E)
    TechnoconEngineeringLtd = 0x0C8E,
    /// tonies GmbH (0x0C8D)
    ToniesGmbH = 0x0C8D,
    /// T-Mobile USA (0x0C8C)
    TMobileUsa = 0x0C8C,
    /// Heavys Inc (0x0C8B)
    HeavysInc = 0x0C8B,
    /// A.GLOBAL co.,Ltd. (0x0C8A)
    AGlobalCoLtd = 0x0C8A,
    /// AGZZX OPTOELECTRONICS TECHNOLOGY CO., LTD (0x0C89)
    AgzzxOptoelectronicsTechnologyCoLtd = 0x0C89,
    /// Nextivity Inc. (0x0C88)
    NextivityInc = 0x0C88,
    /// Weltek Technologies Company Limited (0x0C87)
    WeltekTechnologiesCompanyLimited = 0x0C87,
    /// Qingdao Eastsoft Communication Technology Co.,Ltd (0x0C86)
    QingdaoEastsoftCommunicationTechnologyCoLtd = 0x0C86,
    /// Amlogic, Inc. (0x0C85)
    AmlogicInc = 0x0C85,
    /// MAXON INDUSTRIES, INC. (0x0C84)
    MaxonIndustriesInc = 0x0C84,
    /// Watchdog Systems LLC (0x0C83)
    WatchdogSystemsLlc = 0x0C83,
    /// NACON (0x0C82)
    Nacon = 0x0C82,
    /// Carrier Corporation (0x0C81)
    CarrierCorporation = 0x0C81,
    /// CARDIOID - TECHNOLOGIES, LDA (0x0C80)
    CardioidTechnologiesLda = 0x0C80,
    /// Rochester Sensors, LLC (0x0C7F)
    RochesterSensorsLlc = 0x0C7F,
    /// BOOMING OF THINGS (0x0C7E)
    BoomingOfThings = 0x0C7E,
    /// 3ALogics, Inc. (0x0C7D)
    _3ALogicsInc = 0x0C7D,
    /// Mopeka Products LLC (0x0C7C)
    MopekaProductsLlc = 0x0C7C,
    /// PT SADAMAYA GRAHA TEKNOLOGI (0x0C7B)
    PtSadamayaGrahaTeknologi = 0x0C7B,
    /// Triductor Technology (Suzhou), Inc. (0x0C7A)
    TriductorTechnologySuzhouInc = 0x0C7A,
    /// Zhuhai Smartlink Technology Co., Ltd (0x0C79)
    ZhuhaiSmartlinkTechnologyCoLtd = 0x0C79,
    /// CHARGTRON IOT PRIVATE LIMITED (0x0C78)
    ChargtronIotPrivateLimited = 0x0C78,
    /// TEAC Corporation (0x0C77)
    TeacCorporation = 0x0C77,
    /// Shenzhen Gwell Times Technology Co. , Ltd (0x0C76)
    ShenzhenGwellTimesTechnologyCoLtd = 0x0C76,
    /// Embedded Engineering Solutions LLC (0x0C75)
    EmbeddedEngineeringSolutionsLlc = 0x0C75,
    /// yupiteru (0x0C74)
    Yupiteru = 0x0C74,
    /// Truma Gerätetechnik GmbH & Co. KG (0x0C73)
    TrumaGerätetechnikGmbHAndCoKg = 0x0C73,
    /// StreetCar ORV, LLC (0x0C72)
    StreetCarOrvLlc = 0x0C72,
    /// BitGreen Technolabz (OPC) Private Limited (0x0C71)
    BitGreenTechnolabzOpcPrivateLimited = 0x0C71,
    /// SCARAB SOLUTIONS LTD (0x0C70)
    ScarabSolutionsLtd = 0x0C70,
    /// Parakey AB (0x0C6F)
    ParakeyAb = 0x0C6F,
    /// Sensa LLC (0x0C6E)
    SensaLlc = 0x0C6E,
    /// Fidure Corp. (0x0C6D)
    FidureCorp = 0x0C6D,
    /// SNIFF LOGIC LTD (0x0C6C)
    SniffLogicLtd = 0x0C6C,
    /// GILSON SAS (0x0C6B)
    GilsonSas = 0x0C6B,
    /// CONSORCIO TRUST CONTROL - NETTEL (0x0C6A)
    ConsorcioTrustControlNettel = 0x0C6A,
    /// BLITZ electric motors. LTD (0x0C69)
    BlitzElectricMotorsLtd = 0x0C69,
    /// Emerja Corporation (0x0C68)
    EmerjaCorporation = 0x0C68,
    /// TRACKTING S.R.L. (0x0C67)
    TracktingSRL = 0x0C67,
    /// DEN Smart Home B.V. (0x0C66)
    DenSmartHomeBV = 0x0C66,
    /// WAKO CO,.LTD (0x0C65)
    WakoCoLtd = 0x0C65,
    /// dormakaba Holding AG (0x0C64)
    DormakabaHoldingAg = 0x0C64,
    /// phg Peter Hengstler GmbH + Co. KG (0x0C63)
    PhgPeterHengstlerGmbHCoKg = 0x0C63,
    /// Phiaton Corporation (0x0C62)
    PhiatonCorporation = 0x0C62,
    /// NNOXX, Inc (0x0C61)
    NnoxxInc = 0x0C61,
    /// KEBA Energy Automation GmbH (0x0C60)
    KebaEnergyAutomationGmbH = 0x0C60,
    /// Wuxi Linkpower Microelectronics Co.,Ltd (0x0C5F)
    WuxiLinkpowerMicroelectronicsCoLtd = 0x0C5F,
    /// BlueID GmbH (0x0C5E)
    BlueIdGmbH = 0x0C5E,
    /// StepUp Solutions ApS (0x0C5D)
    StepUpSolutionsApS = 0x0C5D,
    /// MGM WIRELESSS HOLDINGS PTY LTD (0x0C5C)
    MgmWirelesssHoldingsPtyLtd = 0x0C5C,
    /// Alban Giacomo S.P.A. (0x0C5B)
    AlbanGiacomoSPA = 0x0C5B,
    /// Lockswitch Sdn Bhd (0x0C5A)
    LockswitchSdnBhd = 0x0C5A,
    /// CYBERDYNE Inc. (0x0C59)
    CyberdyneInc = 0x0C59,
    /// Hykso Inc. (0x0C58)
    HyksoInc = 0x0C58,
    /// UNEEG medical A/S (0x0C57)
    UneegMedicalAS = 0x0C57,
    /// Rheem Sales Company, Inc. (0x0C56)
    RheemSalesCompanyInc = 0x0C56,
    /// Zintouch B.V. (0x0C55)
    ZintouchBV = 0x0C55,
    /// HiViz Lighting, Inc. (0x0C54)
    HiVizLightingInc = 0x0C54,
    /// Taco, Inc. (0x0C53)
    TacoInc = 0x0C53,
    /// ESCEA LIMITED (0x0C52)
    EsceaLimited = 0x0C52,
    /// INNOVA S.R.L. (0x0C51)
    InnovaSRL = 0x0C51,
    /// Imostar Technologies Inc. (0x0C50)
    ImostarTechnologiesInc = 0x0C50,
    /// SharkNinja Operating LLC (0x0C4F)
    SharkNinjaOperatingLlc = 0x0C4F,
    /// Tactile Engineering, Inc. (0x0C4E)
    TactileEngineeringInc = 0x0C4E,
    /// Seekwave Technology Co.,ltd. (0x0C4D)
    SeekwaveTechnologyCoLtd = 0x0C4D,
    /// Orpyx Medical Technologies Inc. (0x0C4C)
    OrpyxMedicalTechnologiesInc = 0x0C4C,
    /// ADTRAN, Inc. (0x0C4B)
    AdtranInc = 0x0C4B,
    /// atSpiro ApS (0x0C4A)
    AtSpiroApS = 0x0C4A,
    /// twopounds gmbh (0x0C49)
    TwopoundsGmbh = 0x0C49,
    /// VALEO MANAGEMENT SERVICES (0x0C48)
    ValeoManagementServices = 0x0C48,
    /// Epsilon Electronics,lnc (0x0C47)
    EpsilonElectronicsLnc = 0x0C47,
    /// Granwin IoT Technology (Guangzhou) Co.,Ltd (0x0C46)
    GranwinIoTTechnologyGuangzhouCoLtd = 0x0C46,
    /// Brose Verwaltung SE, Bamberg (0x0C45)
    BroseVerwaltungSeBamberg = 0x0C45,
    /// ONCELABS LLC (0x0C44)
    OncelabsLlc = 0x0C44,
    /// Berlinger & Co. AG (0x0C43)
    BerlingerAndCoAg = 0x0C43,
    /// Heath Consultants Inc. (0x0C42)
    HeathConsultantsInc = 0x0C42,
    /// Control Solutions LLC (0x0C41)
    ControlSolutionsLlc = 0x0C41,
    /// HORIBA, Ltd. (0x0C40)
    HoribaLtd = 0x0C40,
    /// Stinger Equipment, Inc. (0x0C3F)
    StingerEquipmentInc = 0x0C3F,
    /// BELLDESIGN Inc. (0x0C3E)
    BelldesignInc = 0x0C3E,
    /// Wrmth Corp. (0x0C3D)
    WrmthCorp = 0x0C3D,
    /// Classified Cycling (0x0C3C)
    ClassifiedCycling = 0x0C3C,
    /// ORB Innovations Ltd (0x0C3B)
    OrbInnovationsLtd = 0x0C3B,
    /// Equinosis, LLC (0x0C3A)
    EquinosisLlc = 0x0C3A,
    /// TIGER CORPORATION (0x0C39)
    TigerCorporation = 0x0C39,
    /// Noritz Corporation. (0x0C38)
    NoritzCorporation = 0x0C38,
    /// SignalQuest, LLC (0x0C37)
    SignalQuestLlc = 0x0C37,
    /// Cosmicnode BV (0x0C36)
    CosmicnodeBv = 0x0C36,
    /// Thermokon-Sensortechnik GmbH (0x0C35)
    ThermokonSensortechnikGmbH = 0x0C35,
    /// BYD Company Limited (0x0C34)
    BydCompanyLimited = 0x0C34,
    /// Exeger Operations AB (0x0C33)
    ExegerOperationsAb = 0x0C33,
    /// Xian Yisuobao Electronic Technology Co., Ltd. (0x0C32)
    XianYisuobaoElectronicTechnologyCoLtd = 0x0C32,
    /// KINDOO LLP (0x0C31)
    KindooLlp = 0x0C31,
    /// McIntosh Group Inc (0x0C30)
    McIntoshGroupInc = 0x0C30,
    /// BEEHERO, INC. (0x0C2F)
    BeeheroInc = 0x0C2F,
    /// Easee AS (0x0C2E)
    EaseeAs = 0x0C2E,
    /// OTF Product Sourcing, LLC (0x0C2D)
    OtfProductSourcingLlc = 0x0C2D,
    /// Zeku Technology (Shanghai) Corp., Ltd. (0x0C2C)
    ZekuTechnologyShanghaiCorpLtd = 0x0C2C,
    /// GigaDevice Semiconductor Inc. (0x0C2B)
    GigaDeviceSemiconductorInc = 0x0C2B,
    /// Caresix Inc. (0x0C2A)
    CaresixInc = 0x0C2A,
    /// DENSO AIRCOOL CORPORATION (0x0C29)
    DensoAircoolCorporation = 0x0C29,
    /// Embecta Corp. (0x0C28)
    EmbectaCorp = 0x0C28,
    /// Pal Electronics (0x0C27)
    PalElectronics = 0x0C27,
    /// Performance Electronics, Ltd. (0x0C26)
    PerformanceElectronicsLtd = 0x0C26,
    /// JURA Elektroapparate AG (0x0C25)
    JuraElektroapparateAg = 0x0C25,
    /// SMARTD TECHNOLOGIES INC. (0x0C24)
    SmartdTechnologiesInc = 0x0C24,
    /// KEYTEC,Inc. (0x0C23)
    KeytecInc = 0x0C23,
    /// Glamo Inc. (0x0C22)
    GlamoInc = 0x0C22,
    /// Foshan Viomi Electrical Technology Co., Ltd (0x0C21)
    FoshanViomiElectricalTechnologyCoLtd = 0x0C21,
    /// COMELIT GROUP S.P.A. (0x0C20)
    ComelitGroupSPA = 0x0C20,
    /// LVI Co. (0x0C1F)
    LviCo = 0x0C1F,
    /// EC sense co., Ltd (0x0C1E)
    EcSenseCoLtd = 0x0C1E,
    /// OFF Line Japan Co., Ltd. (0x0C1D)
    OffLineJapanCoLtd = 0x0C1D,
    /// GEMU (0x0C1C)
    Gemu = 0x0C1C,
    /// Rockchip Electronics Co., Ltd. (0x0C1B)
    RockchipElectronicsCoLtd = 0x0C1B,
    /// Catapult Group International Ltd (0x0C1A)
    CatapultGroupInternationalLtd = 0x0C1A,
    /// Arlo Technologies, Inc. (0x0C19)
    ArloTechnologiesInc = 0x0C19,
    /// CORROHM (0x0C18)
    Corrohm = 0x0C18,
    /// SomnoMed Limited (0x0C17)
    SomnoMedLimited = 0x0C17,
    /// TYKEE PTY. LTD. (0x0C16)
    TykeePtyLtd = 0x0C16,
    /// Geva Sol B.V. (0x0C15)
    GevaSolBV = 0x0C15,
    /// Fasetto, Inc. (0x0C14)
    FasettoInc = 0x0C14,
    /// Scandinavian Health Limited (0x0C13)
    ScandinavianHealthLimited = 0x0C13,
    /// IoSA (0x0C12)
    IoSa = 0x0C12,
    /// Gordon Murray Design Limited (0x0C11)
    GordonMurrayDesignLimited = 0x0C11,
    /// Cosmed s.r.l. (0x0C10)
    CosmedSRL = 0x0C10,
    /// AETERLINK (0x0C0F)
    Aeterlink = 0x0C0F,
    /// ALEX DENKO CO.,LTD. (0x0C0E)
    AlexDenkoCoLtd = 0x0C0E,
    /// Mereltron bv (0x0C0D)
    MereltronBv = 0x0C0D,
    /// Mendeltron, Inc. (0x0C0C)
    MendeltronInc = 0x0C0C,
    /// aconno GmbH (0x0C0B)
    AconnoGmbH = 0x0C0B,
    /// Automated Pet Care Products, LLC (0x0C0A)
    AutomatedPetCareProductsLlc = 0x0C0A,
    /// Senic Inc. (0x0C09)
    SenicInc = 0x0C09,
    /// limited liability company "Red" (0x0C08)
    LimitedLiabilityCompanyRed = 0x0C08,
    /// CONSTRUKTS, INC. (0x0C07)
    ConstruktsInc = 0x0C07,
    /// LED Smart Inc. (0x0C06)
    LedSmartInc = 0x0C06,
    /// Montage Connect, Inc. (0x0C05)
    MontageConnectInc = 0x0C05,
    /// Happy Health, Inc. (0x0C04)
    HappyHealthInc = 0x0C04,
    /// Puff Corp (0x0C03)
    PuffCorp = 0x0C03,
    /// Loomanet, Inc. (0x0C02)
    LoomanetInc = 0x0C02,
    /// NEOWRK SISTEMAS INTELIGENTES S.A. (0x0C01)
    NeowrkSistemasInteligentesSA = 0x0C01,
    /// MQA Limited (0x0C00)
    MqaLimited = 0x0C00,
    /// Ratio Electric BV (0x0BFF)
    RatioElectricBv = 0x0BFF,
    /// Media-Cartec GmbH (0x0BFE)
    MediaCartecGmbH = 0x0BFE,
    /// Esmé Solutions (0x0BFD)
    EsméSolutions = 0x0BFD,
    /// T+A elektroakustik GmbH & Co.KG (0x0BFC)
    TAElektroakustikGmbHAndCoKg = 0x0BFC,
    /// Dodam Enersys Co., Ltd (0x0BFB)
    DodamEnersysCoLtd = 0x0BFB,
    /// CleanBands Systems Ltd. (0x0BFA)
    CleanBandsSystemsLtd = 0x0BFA,
    /// Alio, Inc (0x0BF9)
    AlioInc = 0x0BF9,
    /// Innovacionnye Resheniya (0x0BF8)
    InnovacionnyeResheniya = 0x0BF8,
    /// Wacker Neuson SE (0x0BF7)
    WackerNeusonSe = 0x0BF7,
    /// greenTEG AG (0x0BF6)
    GreenTegAg = 0x0BF6,
    /// T5 tek, Inc. (0x0BF5)
    T5TekInc = 0x0BF5,
    /// ER Lab LLC (0x0BF4)
    ErLabLlc = 0x0BF4,
    /// PONE BIOMETRICS AS (0x0BF3)
    PoneBiometricsAs = 0x0BF3,
    /// Angel Medical Systems, Inc. (0x0BF2)
    AngelMedicalSystemsInc = 0x0BF2,
    /// Site IQ LLC (0x0BF1)
    SiteIqLlc = 0x0BF1,
    /// KIDO SPORTS CO., LTD. (0x0BF0)
    KidoSportsCoLtd = 0x0BF0,
    /// Safetytest GmbH (0x0BEF)
    SafetytestGmbH = 0x0BEF,
    /// LINKSYS USA, INC. (0x0BEE)
    LinksysUsaInc = 0x0BEE,
    /// CORAL-TAIYI Co. Ltd. (0x0BED)
    CoralTaiyiCoLtd = 0x0BED,
    /// Miracle-Ear, Inc. (0x0BEC)
    MiracleEarInc = 0x0BEC,
    /// Luna Health, Inc. (0x0BEB)
    LunaHealthInc = 0x0BEB,
    /// Twenty Five Seven, prodaja in storitve, d.o.o. (0x0BEA)
    TwentyFiveSevenProdajaInStoritveDOO = 0x0BEA,
    /// Shindengen Electric Manufacturing Co., Ltd. (0x0BE9)
    ShindengenElectricManufacturingCoLtd = 0x0BE9,
    /// Sensormate AG (0x0BE8)
    SensormateAg = 0x0BE8,
    /// Fresnel Technologies, Inc. (0x0BE7)
    FresnelTechnologiesInc = 0x0BE7,
    /// Puratap Pty Ltd (0x0BE6)
    PuratapPtyLtd = 0x0BE6,
    /// ZWILLING J.A. Henckels Aktiengesellschaft (0x0BE5)
    ZwillingJAHenckelsAktiengesellschaft = 0x0BE5,
    /// Deepfield Connect GmbH (0x0BE4)
    DeepfieldConnectGmbH = 0x0BE4,
    /// Comtel Systems Ltd. (0x0BE3)
    ComtelSystemsLtd = 0x0BE3,
    /// OTC engineering (0x0BE2)
    OtcEngineering = 0x0BE2,
    /// Back40 Precision (0x0BE1)
    Back40Precision = 0x0BE1,
    /// Koizumi Lighting Technology corp. (0x0BE0)
    KoizumiLightingTechnologyCorp = 0x0BE0,
    /// WINKEY ENTERPRISE (HONG KONG) LIMITED (0x0BDF)
    WinkeyEnterpriseHongKongLimited = 0x0BDF,
    /// Yale (0x0BDE)
    Yale = 0x0BDE,
    /// Coroflo Limited (0x0BDD)
    CorofloLimited = 0x0BDD,
    /// Ledworks S.r.l. (0x0BDC)
    LedworksSRL = 0x0BDC,
    /// CHAR-BROIL, LLC (0x0BDB)
    CharBroilLlc = 0x0BDB,
    /// Aardex Ltd. (0x0BDA)
    AardexLtd = 0x0BDA,
    /// Elics Basis Ltd. (0x0BD9)
    ElicsBasisLtd = 0x0BD9,
    /// PURA SCENTS, INC. (0x0BD8)
    PuraScentsInc = 0x0BD8,
    /// VINFAST TRADING AND PRODUCTION JOINT STOCK COMPANY (0x0BD7)
    VinfastTradingAndProductionJointStockCompany = 0x0BD7,
    /// Shenzhen Injoinic Technology Co., Ltd. (0x0BD6)
    ShenzhenInjoinicTechnologyCoLtd = 0x0BD6,
    /// Super B Lithium Power B.V. (0x0BD5)
    SuperBLithiumPowerBV = 0x0BD5,
    /// ndd Medizintechnik AG (0x0BD4)
    NddMedizintechnikAg = 0x0BD4,
    /// Procon Analytics, LLC (0x0BD3)
    ProconAnalyticsLlc = 0x0BD3,
    /// IDEC (0x0BD2)
    Idec = 0x0BD2,
    /// Hubei Yuan Times Technology Co., Ltd. (0x0BD1)
    HubeiYuanTimesTechnologyCoLtd = 0x0BD1,
    /// Durag GmbH (0x0BD0)
    DuragGmbH = 0x0BD0,
    /// LL Tec Group LLC (0x0BCF)
    LlTecGroupLlc = 0x0BCF,
    /// Neurosity, Inc. (0x0BCE)
    NeurosityInc = 0x0BCE,
    /// Amiko srl (0x0BCD)
    AmikoSrl = 0x0BCD,
    /// Sylvac sa (0x0BCC)
    SylvacSa = 0x0BCC,
    /// Divesoft s.r.o. (0x0BCB)
    DivesoftSRO = 0x0BCB,
    /// Perimeter Technologies, Inc. (0x0BCA)
    PerimeterTechnologiesInc = 0x0BCA,
    /// Neuvatek Inc. (0x0BC9)
    NeuvatekInc = 0x0BC9,
    /// OTF Distribution, LLC (0x0BC8)
    OtfDistributionLlc = 0x0BC8,
    /// Signtle Inc. (0x0BC7)
    SigntleInc = 0x0BC7,
    /// TCL COMMUNICATION EQUIPMENT CO.,LTD. (0x0BC6)
    TclCommunicationEquipmentCoLtd = 0x0BC6,
    /// Aperia Technologies, Inc. (0x0BC5)
    AperiaTechnologiesInc = 0x0BC5,
    /// TECHTICS ENGINEERING B.V. (0x0BC4)
    TechticsEngineeringBV = 0x0BC4,
    /// MCOT INC. (0x0BC3)
    McotInc = 0x0BC3,
    /// EntWick Co. (0x0BC2)
    EntWickCo = 0x0BC2,
    /// Miele & Cie. KG (0x0BC1)
    MieleAndCieKg = 0x0BC1,
    /// READY FOR SKY LLP (0x0BC0)
    ReadyForSkyLlp = 0x0BC0,
    /// HIMSA II K/S (0x0BBF)
    HimsaIiKS = 0x0BBF,
    /// SAAB Aktiebolag (0x0BBE)
    SaabAktiebolag = 0x0BBE,
    /// ETHEORY PTY LTD (0x0BBD)
    EtheoryPtyLtd = 0x0BBD,
    /// T2REALITY SOLUTIONS PRIVATE LIMITED (0x0BBC)
    T2RealitySolutionsPrivateLimited = 0x0BBC,
    /// SWISSINNO SOLUTIONS AG (0x0BBB)
    SwissinnoSolutionsAg = 0x0BBB,
    /// Huso, INC (0x0BBA)
    HusoInc = 0x0BBA,
    /// SaluStim Group Oy (0x0BB9)
    SaluStimGroupOy = 0x0BB9,
    /// INNOVAG PTY. LTD. (0x0BB8)
    InnovagPtyLtd = 0x0BB8,
    /// IONA Tech LLC (0x0BB7)
    IonaTechLlc = 0x0BB7,
    /// Build With Robots Inc. (0x0BB6)
    BuildWithRobotsInc = 0x0BB6,
    /// Xirgo Technologies, LLC (0x0BB5)
    XirgoTechnologiesLlc = 0x0BB5,
    /// New Cosmos USA, Inc. (0x0BB4)
    NewCosmosUsaInc = 0x0BB4,
    /// Flender GmbH (0x0BB3)
    FlenderGmbH = 0x0BB3,
    /// Fjorden Electra AS (0x0BB2)
    FjordenElectraAs = 0x0BB2,
    /// Beijing ranxin intelligence technology Co.,LTD (0x0BB1)
    BeijingRanxinIntelligenceTechnologyCoLtd = 0x0BB1,
    /// Ecolab Inc. (0x0BB0)
    EcolabInc = 0x0BB0,
    /// NITTO KOGYO CORPORATION (0x0BAF)
    NittoKogyoCorporation = 0x0BAF,
    /// Soma Labs LLC (0x0BAE)
    SomaLabsLlc = 0x0BAE,
    /// Roambotics, Inc. (0x0BAD)
    RoamboticsInc = 0x0BAD,
    /// Machfu Inc. (0x0BAC)
    MachfuInc = 0x0BAC,
    /// Grandex International Corporation (0x0BAB)
    GrandexInternationalCorporation = 0x0BAB,
    /// Infinitegra, Inc. (0x0BAA)
    InfinitegraInc = 0x0BAA,
    /// Allterco Robotics ltd (0x0BA9)
    AlltercoRoboticsLtd = 0x0BA9,
    /// GLOWFORGE INC. (0x0BA8)
    GlowforgeInc = 0x0BA8,
    /// hearX Group (Pty) Ltd (0x0BA7)
    HearXGroupPtyLtd = 0x0BA7,
    /// Nissan Motor Co., Ltd. (0x0BA6)
    NissanMotorCoLtd = 0x0BA6,
    /// SONICOS ENTERPRISES, LLC (0x0BA5)
    SonicosEnterprisesLlc = 0x0BA5,
    /// Vervent Audio Group (0x0BA4)
    VerventAudioGroup = 0x0BA4,
    /// Sonova Consumer Hearing GmbH (0x0BA3)
    SonovaConsumerHearingGmbH = 0x0BA3,
    /// TireCheck GmbH (0x0BA2)
    TireCheckGmbH = 0x0BA2,
    /// Bunn-O-Matic Corporation (0x0BA1)
    BunnOMaticCorporation = 0x0BA1,
    /// Data Sciences International (0x0BA0)
    DataSciencesInternational = 0x0BA0,
    /// Group Lotus Limited (0x0B9F)
    GroupLotusLimited = 0x0B9F,
    /// Audio Partnership Plc (0x0B9E)
    AudioPartnershipPlc = 0x0B9E,
    /// Sensoria Holdings LTD (0x0B9D)
    SensoriaHoldingsLtd = 0x0B9D,
    /// Komatsu Ltd. (0x0B9C)
    KomatsuLtd = 0x0B9C,
    /// GISMAN (0x0B9B)
    Gisman = 0x0B9B,
    /// Beijing Wisepool Infinite Intelligence Technology Co.,Ltd (0x0B9A)
    BeijingWisepoolInfiniteIntelligenceTechnologyCoLtd = 0x0B9A,
    /// The Goodyear Tire & Rubber Company (0x0B99)
    TheGoodyearTireAndRubberCompany = 0x0B99,
    /// Gymstory B.V. (0x0B98)
    GymstoryBV = 0x0B98,
    /// SILVER TREE LABS, INC. (0x0B97)
    SilverTreeLabsInc = 0x0B97,
    /// Telecom Design (0x0B96)
    TelecomDesign = 0x0B96,
    /// Netwake GmbH (0x0B95)
    NetwakeGmbH = 0x0B95,
    /// Dreem SAS (0x0B94)
    DreemSas = 0x0B94,
    /// Hangzhou BroadLink Technology Co., Ltd. (0x0B93)
    HangzhouBroadLinkTechnologyCoLtd = 0x0B93,
    /// Citisend Solutions, SL (0x0B92)
    CitisendSolutionsSl = 0x0B92,
    /// Alfen ICU B.V. (0x0B91)
    AlfenIcuBV = 0x0B91,
    /// Ineos Automotive Limited (0x0B90)
    IneosAutomotiveLimited = 0x0B90,
    /// Senscomm Semiconductor Co., Ltd. (0x0B8F)
    SenscommSemiconductorCoLtd = 0x0B8F,
    /// Gentle Energy Corp. (0x0B8E)
    GentleEnergyCorp = 0x0B8E,
    /// Pertech Industries Inc (0x0B8D)
    PertechIndustriesInc = 0x0B8D,
    /// MOTREX (0x0B8C)
    Motrex = 0x0B8C,
    /// American Technology Components, Incorporated (0x0B8B)
    AmericanTechnologyComponentsIncorporated = 0x0B8B,
    /// Seiko Instruments Inc. (0x0B8A)
    SeikoInstrumentsInc = 0x0B8A,
    /// Rotronic AG (0x0B89)
    RotronicAg = 0x0B89,
    /// Muguang (Guangdong) Intelligent Lighting Technology Co., Ltd (0x0B88)
    MuguangGuangdongIntelligentLightingTechnologyCoLtd = 0x0B88,
    /// Ampetronic Ltd (0x0B87)
    AmpetronicLtd = 0x0B87,
    /// Trek Bicycle (0x0B86)
    TrekBicycle = 0x0B86,
    /// VIMANA TECH PTY LTD (0x0B85)
    VimanaTechPtyLtd = 0x0B85,
    /// Presidio Medical, Inc. (0x0B84)
    PresidioMedicalInc = 0x0B84,
    /// Taiga Motors Inc. (0x0B83)
    TaigaMotorsInc = 0x0B83,
    /// Mammut Sports Group AG (0x0B82)
    MammutSportsGroupAg = 0x0B82,
    /// SCM Group (0x0B81)
    ScmGroup = 0x0B81,
    /// AXELIFE (0x0B80)
    Axelife = 0x0B80,
    /// ICU tech GmbH (0x0B7F)
    IcuTechGmbH = 0x0B7F,
    /// Offcode Oy (0x0B7E)
    OffcodeOy = 0x0B7E,
    /// FoundersLane GmbH (0x0B7D)
    FoundersLaneGmbH = 0x0B7D,
    /// Scangrip A/S (0x0B7C)
    ScangripAS = 0x0B7C,
    /// Hardcoder Oy (0x0B7B)
    HardcoderOy = 0x0B7B,
    /// Shenzhen KTC Technology Co.,Ltd. (0x0B7A)
    ShenzhenKtcTechnologyCoLtd = 0x0B7A,
    /// Sankyo Air Tech Co.,Ltd. (0x0B79)
    SankyoAirTechCoLtd = 0x0B79,
    /// FIELD DESIGN INC. (0x0B78)
    FieldDesignInc = 0x0B78,
    /// Aixlink(Chengdu) Co., Ltd. (0x0B77)
    AixlinkChengduCoLtd = 0x0B77,
    /// MAX-co., ltd (0x0B76)
    MaxCoLtd = 0x0B76,
    /// Triple W Japan Inc. (0x0B75)
    TripleWJapanInc = 0x0B75,
    /// BQN (0x0B74)
    Bqn = 0x0B74,
    /// HARADA INDUSTRY CO., LTD. (0x0B73)
    HaradaIndustryCoLtd = 0x0B73,
    /// Geeknet, Inc. (0x0B72)
    GeeknetInc = 0x0B72,
    /// lilbit ODM AS (0x0B71)
    LilbitOdmAs = 0x0B71,
    /// JDRF Electromag Engineering Inc (0x0B70)
    JdrfElectromagEngineeringInc = 0x0B70,
    /// Shenzhen Malide Technology Co.,Ltd (0x0B6F)
    ShenzhenMalideTechnologyCoLtd = 0x0B6F,
    /// React Mobile (0x0B6E)
    ReactMobile = 0x0B6E,
    /// SOLUM CO., LTD (0x0B6D)
    SolumCoLtd = 0x0B6D,
    /// Sensitech, Inc. (0x0B6C)
    SensitechInc = 0x0B6C,
    /// Samsara Networks, Inc (0x0B6B)
    SamsaraNetworksInc = 0x0B6B,
    /// Dymo (0x0B6A)
    Dymo = 0x0B6A,
    /// Addaday (0x0B69)
    Addaday = 0x0B69,
    /// Quha oy (0x0B68)
    QuhaOy = 0x0B68,
    /// CleanSpace Technology Pty Ltd (0x0B67)
    CleanSpaceTechnologyPtyLtd = 0x0B67,
    /// MITSUBISHI ELECTRIC AUTOMATION (THAILAND) COMPANY LIMITED (0x0B66)
    MitsubishiElectricAutomationThailandCompanyLimited = 0x0B66,
    /// The Apache Software Foundation (0x0B65)
    TheApacheSoftwareFoundation = 0x0B65,
    /// NingBo klite Electric Manufacture Co.,LTD (0x0B64)
    NingBoKliteElectricManufactureCoLtd = 0x0B64,
    /// Innolux Corporation (0x0B63)
    InnoluxCorporation = 0x0B63,
    /// NOVEA ENERGIES (0x0B62)
    NoveaEnergies = 0x0B62,
    /// Sentek Pty Ltd (0x0B61)
    SentekPtyLtd = 0x0B61,
    /// RATOC Systems, Inc. (0x0B60)
    RatocSystemsInc = 0x0B60,
    /// Rivieh, Inc. (0x0B5F)
    RiviehInc = 0x0B5F,
    /// CELLCONTROL, INC. (0x0B5E)
    CellcontrolInc = 0x0B5E,
    /// Fujian Newland Auto-ID Tech. Co., Ltd. (0x0B5D)
    FujianNewlandAutoIdTechCoLtd = 0x0B5D,
    /// Exponential Power, Inc. (0x0B5C)
    ExponentialPowerInc = 0x0B5C,
    /// Shenzhen ImagineVision Technology Limited (0x0B5B)
    ShenzhenImagineVisionTechnologyLimited = 0x0B5B,
    /// H.P. Shelby Manufacturing, LLC. (0x0B5A)
    HPShelbyManufacturingLlc = 0x0B5A,
    /// Versa Group B.V. (0x0B59)
    VersaGroupBV = 0x0B59,
    /// TOKAI-DENSHI INC (0x0B58)
    TokaiDenshiInc = 0x0B58,
    /// CONVERTRONIX TECHNOLOGIES AND SERVICES LLP (0x0B57)
    ConvertronixTechnologiesAndServicesLlp = 0x0B57,
    /// BORA - Vertriebs GmbH & Co KG (0x0B56)
    BoraVertriebsGmbHAndCoKg = 0x0B56,
    /// H G M Automotive Electronics, Inc. (0x0B55)
    HGMAutomotiveElectronicsInc = 0x0B55,
    /// Emotion Fitness GmbH & Co. KG (0x0B54)
    EmotionFitnessGmbHAndCoKg = 0x0B54,
    /// SHENZHEN KAADAS INTELLIGENT TECHNOLOGY CO.,Ltd (0x0B53)
    ShenzhenKaadasIntelligentTechnologyCoLtd = 0x0B53,
    /// ZIIP Inc (0x0B52)
    ZiipInc = 0x0B52,
    /// FUN FACTORY GmbH (0x0B51)
    FunFactoryGmbH = 0x0B51,
    /// Mesh Systems LLC (0x0B50)
    MeshSystemsLlc = 0x0B50,
    /// Breezi.io, Inc. (0x0B4F)
    BreeziIoInc = 0x0B4F,
    /// ICP Systems B.V. (0x0B4E)
    IcpSystemsBV = 0x0B4E,
    /// Adam Hall GmbH (0x0B4D)
    AdamHallGmbH = 0x0B4D,
    /// BiosBob.Biz (0x0B4C)
    BiosBobBiz = 0x0B4C,
    /// EMS Integrators, LLC (0x0B4B)
    EmsIntegratorsLlc = 0x0B4B,
    /// Nomono AS (0x0B4A)
    NomonoAs = 0x0B4A,
    /// SkyHawke Technologies (0x0B49)
    SkyHawkeTechnologies = 0x0B49,
    /// NIO USA, Inc. (0x0B48)
    NioUsaInc = 0x0B48,
    /// Gentex Corporation (0x0B47)
    GentexCorporation = 0x0B47,
    /// Bird Rides, Inc. (0x0B46)
    BirdRidesInc = 0x0B46,
    /// Electronic Sensors, Inc. (0x0B45)
    ElectronicSensorsInc = 0x0B45,
    /// nFore Technology Co., Ltd. (0x0B44)
    NForeTechnologyCoLtd = 0x0B44,
    /// INCITAT ENVIRONNEMENT (0x0B43)
    IncitatEnvironnement = 0x0B43,
    /// TSI (0x0B42)
    Tsi = 0x0B42,
    /// Sentrax GmbH (0x0B41)
    SentraxGmbH = 0x0B41,
    /// Havells India Limited (0x0B40)
    HavellsIndiaLimited = 0x0B40,
    /// MindRhythm, Inc. (0x0B3F)
    MindRhythmInc = 0x0B3F,
    /// ISEO Serrature S.p.a. (0x0B3E)
    IseoSerratureSPA = 0x0B3E,
    /// REALTIMEID AS (0x0B3D)
    RealtimeidAs = 0x0B3D,
    /// Dodge Industrial, Inc. (0x0B3C)
    DodgeIndustrialInc = 0x0B3C,
    /// AIC semiconductor (Shanghai) Co., Ltd. (0x0B3B)
    AicSemiconductorShanghaiCoLtd = 0x0B3B,
    /// Impact Biosystems, Inc. (0x0B3A)
    ImpactBiosystemsInc = 0x0B3A,
    /// Red 100 Lighting Co., ltd. (0x0B39)
    Red100LightingCoLtd = 0x0B39,
    /// WISYCOM S.R.L. (0x0B38)
    WisycomSRL = 0x0B38,
    /// Omnivoltaic Energy Solutions Limited Company (0x0B37)
    OmnivoltaicEnergySolutionsLimitedCompany = 0x0B37,
    /// SINTEF (0x0B36)
    Sintef = 0x0B36,
    /// BH SENS (0x0B35)
    BhSens = 0x0B35,
    /// CONZUMEX INDUSTRIES PRIVATE LIMITED (0x0B34)
    ConzumexIndustriesPrivateLimited = 0x0B34,
    /// ARMATURA LLC (0x0B33)
    ArmaturaLlc = 0x0B33,
    /// Hala Systems, Inc. (0x0B32)
    HalaSystemsInc = 0x0B32,
    /// Silver Wolf Vehicles Inc. (0x0B31)
    SilverWolfVehiclesInc = 0x0B31,
    /// ART SPA (0x0B30)
    ArtSpa = 0x0B30,
    /// Duke Manufacturing Co (0x0B2F)
    DukeManufacturingCo = 0x0B2F,
    /// MOCA System Inc. (0x0B2E)
    MocaSystemInc = 0x0B2E,
    /// REDARC ELECTRONICS PTY LTD (0x0B2D)
    RedarcElectronicsPtyLtd = 0x0B2D,
    /// ILLUMAGEAR, Inc. (0x0B2C)
    IllumagearInc = 0x0B2C,
    /// MAINBOT (0x0B2B)
    Mainbot = 0x0B2B,
    /// ACL Airshop B.V. (0x0B2A)
    AclAirshopBV = 0x0B2A,
    /// Tech-Venom Entertainment Private Limited (0x0B29)
    TechVenomEntertainmentPrivateLimited = 0x0B29,
    /// CHACON (0x0B28)
    Chacon = 0x0B28,
    /// Lumi United Technology Co., Ltd (0x0B27)
    LumiUnitedTechnologyCoLtd = 0x0B27,
    /// Baracoda Daily Healthtech. (0x0B26)
    BaracodaDailyHealthtech = 0x0B26,
    /// NIBROTECH LTD (0x0B25)
    NibrotechLtd = 0x0B25,
    /// BeiJing ZiJie TiaoDong KeJi Co.,Ltd. (0x0B24)
    BeiJingZiJieTiaoDongKeJiCoLtd = 0x0B24,
    /// iRhythm Technologies, Inc. (0x0B23)
    IRhythmTechnologiesInc = 0x0B23,
    /// Hygiene IQ, LLC. (0x0B22)
    HygieneIqLlc = 0x0B22,
    /// ams AG (0x0B21)
    AmsAg = 0x0B21,
    /// TKH Security B.V. (0x0B20)
    TkhSecurityBV = 0x0B20,
    /// Beijing ESWIN Computing Technology Co., Ltd. (0x0B1F)
    BeijingEswinComputingTechnologyCoLtd = 0x0B1F,
    /// PB INC. (0x0B1E)
    PbInc = 0x0B1E,
    /// Accelerated Systems (0x0B1D)
    AcceleratedSystems = 0x0B1D,
    /// Nanoleq AG (0x0B1C)
    NanoleqAg = 0x0B1C,
    /// Enerpac Tool Group Corp. (0x0B1B)
    EnerpacToolGroupCorp = 0x0B1B,
    /// Roca Sanitario, S.A. (0x0B1A)
    RocaSanitarioSA = 0x0B1A,
    /// WBS PROJECT H PTY LTD (0x0B19)
    WbsProjectHPtyLtd = 0x0B19,
    /// DECATHLON SE (0x0B18)
    DecathlonSe = 0x0B18,
    /// SIG SAUER, INC. (0x0B17)
    SigSauerInc = 0x0B17,
    /// Guard RFID Solutions Inc. (0x0B16)
    GuardRfidSolutionsInc = 0x0B16,
    /// NAOS JAPAN K.K. (0x0B15)
    NaosJapanKK = 0x0B15,
    /// Olumee (0x0B14)
    Olumee = 0x0B14,
    /// IOTOOLS (0x0B13)
    Iotools = 0x0B13,
    /// ToughBuilt Industries LLC (0x0B12)
    ToughBuiltIndustriesLlc = 0x0B12,
    /// ThermoWorks, Inc. (0x0B11)
    ThermoWorksInc = 0x0B11,
    /// Alfa Laval Corporate AB (0x0B10)
    AlfaLavalCorporateAb = 0x0B10,
    /// B.E.A. S.A. (0x0B0F)
    BEASA = 0x0B0F,
    /// Honda Lock Mfg. Co.,Ltd. (0x0B0E)
    HondaLockMfgCoLtd = 0x0B0E,
    /// SANYO DENKO Co.,Ltd. (0x0B0D)
    SanyoDenkoCoLtd = 0x0B0D,
    /// BluPeak (0x0B0C)
    BluPeak = 0x0B0C,
    /// Sanistaal A/S (0x0B0B)
    SanistaalAS = 0x0B0B,
    /// Belun Technology Company Limited (0x0B0A)
    BelunTechnologyCompanyLimited = 0x0B0A,
    /// soonisys (0x0B09)
    Soonisys = 0x0B09,
    /// Shenzhen Qianfenyi Intelligent Technology Co., LTD (0x0B08)
    ShenzhenQianfenyiIntelligentTechnologyCoLtd = 0x0B08,
    /// Workaround Gmbh (0x0B07)
    WorkaroundGmbh = 0x0B07,
    /// FAZUA GmbH (0x0B06)
    FazuaGmbH = 0x0B06,
    /// Marquardt GmbH (0x0B05)
    MarquardtGmbH = 0x0B05,
    /// I-PERCUT (0x0B04)
    IPercut = 0x0B04,
    /// Precision Triathlon Systems Limited (0x0B03)
    PrecisionTriathlonSystemsLimited = 0x0B03,
    /// IORA Technology Development Ltd. Sti. (0x0B02)
    IoraTechnologyDevelopmentLtdSti = 0x0B02,
    /// RESIDEO TECHNOLOGIES, INC. (0x0B01)
    ResideoTechnologiesInc = 0x0B01,
    /// Flaircomm Microelectronics Inc. (0x0B00)
    FlaircommMicroelectronicsInc = 0x0B00,
    /// FUSEAWARE LIMITED (0x0AFF)
    FuseawareLimited = 0x0AFF,
    /// Earda Technologies Co.,Ltd (0x0AFE)
    EardaTechnologiesCoLtd = 0x0AFE,
    /// Weber Sensors, LLC (0x0AFD)
    WeberSensorsLlc = 0x0AFD,
    /// Cerebrum Sensor Technologies Inc. (0x0AFC)
    CerebrumSensorTechnologiesInc = 0x0AFC,
    /// SMT ELEKTRONIK GmbH (0x0AFB)
    SmtElektronikGmbH = 0x0AFB,
    /// Chengdu Ambit Technology Co., Ltd. (0x0AFA)
    ChengduAmbitTechnologyCoLtd = 0x0AFA,
    /// Unisto AG (0x0AF9)
    UnistoAg = 0x0AF9,
    /// First Design System Inc. (0x0AF8)
    FirstDesignSystemInc = 0x0AF8,
    /// Irdeto (0x0AF7)
    Irdeto = 0x0AF7,
    /// AMETEK, Inc. (0x0AF6)
    AmetekInc = 0x0AF6,
    /// Unitech Electronic Inc. (0x0AF5)
    UnitechElectronicInc = 0x0AF5,
    /// Radioworks Microelectronics PTY LTD (0x0AF4)
    RadioworksMicroelectronicsPtyLtd = 0x0AF4,
    /// 701x Inc. (0x0AF3)
    _701XInc = 0x0AF3,
    /// Shanghai All Link Microelectronics Co.,Ltd (0x0AF2)
    ShanghaiAllLinkMicroelectronicsCoLtd = 0x0AF2,
    /// CRADERS,CO.,LTD (0x0AF1)
    CradersCoLtd = 0x0AF1,
    /// Leupold & Stevens, Inc. (0x0AF0)
    LeupoldAndStevensInc = 0x0AF0,
    /// GLP German Light Products GmbH (0x0AEF)
    GlpGermanLightProductsGmbH = 0x0AEF,
    /// Velentium, LLC (0x0AEE)
    VelentiumLlc = 0x0AEE,
    /// Saxonar GmbH (0x0AED)
    SaxonarGmbH = 0x0AED,
    /// FUTEK ADVANCED SENSOR TECHNOLOGY, INC (0x0AEC)
    FutekAdvancedSensorTechnologyInc = 0x0AEC,
    /// Square, Inc. (0x0AEB)
    SquareInc = 0x0AEB,
    /// Borda Technology (0x0AEA)
    BordaTechnology = 0x0AEA,
    /// FLIR Systems AB (0x0AE9)
    FlirSystemsAb = 0x0AE9,
    /// LEVEL, s.r.o. (0x0AE8)
    LevelSRO = 0x0AE8,
    /// Sunplus Technology Co., Ltd. (0x0AE7)
    SunplusTechnologyCoLtd = 0x0AE7,
    /// Hexology (0x0AE6)
    Hexology = 0x0AE6,
    /// unu GmbH (0x0AE5)
    UnuGmbH = 0x0AE5,
    /// DALI Alliance (0x0AE4)
    DaliAlliance = 0x0AE4,
    /// GlobalMed (0x0AE3)
    GlobalMed = 0x0AE3,
    /// IMATRIX SYSTEMS, INC. (0x0AE2)
    ImatrixSystemsInc = 0x0AE2,
    /// ChengDu ForThink Technology Co., Ltd. (0x0AE1)
    ChengDuForThinkTechnologyCoLtd = 0x0AE1,
    /// Viceroy Devices Corporation (0x0AE0)
    ViceroyDevicesCorporation = 0x0AE0,
    /// Douglas Dynamics L.L.C. (0x0ADF)
    DouglasDynamicsLLC = 0x0ADF,
    /// Vocera Communications, Inc. (0x0ADE)
    VoceraCommunicationsInc = 0x0ADE,
    /// Boss Audio (0x0ADD)
    BossAudio = 0x0ADD,
    /// Duravit AG (0x0ADC)
    DuravitAg = 0x0ADC,
    /// Reelables, Inc. (0x0ADB)
    ReelablesInc = 0x0ADB,
    /// Codefabrik GmbH (0x0ADA)
    CodefabrikGmbH = 0x0ADA,
    /// Shenzhen Aimore. Co.,Ltd (0x0AD9)
    ShenzhenAimoreCoLtd = 0x0AD9,
    /// Franz Kaldewei GmbH&Co KG (0x0AD8)
    FranzKaldeweiGmbHAndCoKg = 0x0AD8,
    /// AL-KO Geraete GmbH (0x0AD7)
    AlKoGeraeteGmbH = 0x0AD7,
    /// nymea GmbH (0x0AD6)
    NymeaGmbH = 0x0AD6,
    /// Streamit B.V. (0x0AD5)
    StreamitBV = 0x0AD5,
    /// Zhuhai Pantum Electronisc Co., Ltd (0x0AD4)
    ZhuhaiPantumElectroniscCoLtd = 0x0AD4,
    /// SSV Software Systems GmbH (0x0AD3)
    SsvSoftwareSystemsGmbH = 0x0AD3,
    /// Lautsprecher Teufel GmbH (0x0AD2)
    LautsprecherTeufelGmbH = 0x0AD2,
    /// EAGLE KINGDOM TECHNOLOGIES LIMITED (0x0AD1)
    EagleKingdomTechnologiesLimited = 0x0AD1,
    /// Nordic Strong ApS (0x0AD0)
    NordicStrongApS = 0x0AD0,
    /// CACI Technologies (0x0ACF)
    CaciTechnologies = 0x0ACF,
    /// KOBATA GAUGE MFG. CO., LTD. (0x0ACE)
    KobataGaugeMfgCoLtd = 0x0ACE,
    /// Visuallex Sport International Limited (0x0ACD)
    VisuallexSportInternationalLimited = 0x0ACD,
    /// Nuvoton (0x0ACC)
    Nuvoton = 0x0ACC,
    /// ise Individuelle Software und Elektronik GmbH (0x0ACB)
    IseIndividuelleSoftwareUndElektronikGmbH = 0x0ACB,
    /// Shenzhen CoolKit Technology Co., Ltd (0x0ACA)
    ShenzhenCoolKitTechnologyCoLtd = 0x0ACA,
    /// Swedlock AB (0x0AC9)
    SwedlockAb = 0x0AC9,
    /// Keepin Co., Ltd. (0x0AC8)
    KeepinCoLtd = 0x0AC8,
    /// Chengdu Aich Technology Co.,Ltd (0x0AC7)
    ChengduAichTechnologyCoLtd = 0x0AC7,
    /// Barnes Group Inc. (0x0AC6)
    BarnesGroupInc = 0x0AC6,
    /// Flexoptix GmbH (0x0AC5)
    FlexoptixGmbH = 0x0AC5,
    /// CODIUM (0x0AC4)
    Codium = 0x0AC4,
    /// Kenzen, Inc. (0x0AC3)
    KenzenInc = 0x0AC3,
    /// RealMega Microelectronics technology (Shanghai) Co. Ltd. (0x0AC2)
    RealMegaMicroelectronicsTechnologyShanghaiCoLtd = 0x0AC2,
    /// Shenzhen Jingxun Technology Co., Ltd. (0x0AC1)
    ShenzhenJingxunTechnologyCoLtd = 0x0AC1,
    /// Omni-ID USA, INC. (0x0AC0)
    OmniIdUsaInc = 0x0AC0,
    /// PAUL HARTMANN AG (0x0ABF)
    PaulHartmannAg = 0x0ABF,
    /// Robkoo Information & Technologies Co., Ltd. (0x0ABE)
    RobkooInformationAndTechnologiesCoLtd = 0x0ABE,
    /// Inventas AS (0x0ABD)
    InventasAs = 0x0ABD,
    /// KCCS Mobile Engineering Co., Ltd. (0x0ABC)
    KccsMobileEngineeringCoLtd = 0x0ABC,
    /// R-DAS, s.r.o. (0x0ABB)
    RDasSRO = 0x0ABB,
    /// Open Bionics Ltd. (0x0ABA)
    OpenBionicsLtd = 0x0ABA,
    /// STL (0x0AB9)
    Stl = 0x0AB9,
    /// Sens.ai Incorporated (0x0AB8)
    SensAiIncorporated = 0x0AB8,
    /// LogTag North America Inc. (0x0AB7)
    LogTagNorthAmericaInc = 0x0AB7,
    /// Xenter, Inc. (0x0AB6)
    XenterInc = 0x0AB6,
    /// Elstat Electronics Ltd. (0x0AB5)
    ElstatElectronicsLtd = 0x0AB5,
    /// Ellenby Technologies, Inc. (0x0AB4)
    EllenbyTechnologiesInc = 0x0AB4,
    /// INNER RANGE PTY. LTD. (0x0AB3)
    InnerRangePtyLtd = 0x0AB3,
    /// TouchTronics, Inc. (0x0AB2)
    TouchTronicsInc = 0x0AB2,
    /// InVue Security Products Inc (0x0AB1)
    InVueSecurityProductsInc = 0x0AB1,
    /// Visiontronic s.r.o. (0x0AB0)
    VisiontronicSRO = 0x0AB0,
    /// AIAIAI ApS (0x0AAF)
    AiaiaiApS = 0x0AAF,
    /// PS Engineering, Inc. (0x0AAE)
    PsEngineeringInc = 0x0AAE,
    /// Adevo Consulting AB (0x0AAD)
    AdevoConsultingAb = 0x0AAD,
    /// OSM HK Limited (0x0AAC)
    OsmHkLimited = 0x0AAC,
    /// Anhui Listenai Co (0x0AAB)
    AnhuiListenaiCo = 0x0AAB,
    /// Computime International Ltd (0x0AAA)
    ComputimeInternationalLtd = 0x0AAA,
    /// Spintly, Inc. (0x0AA9)
    SpintlyInc = 0x0AA9,
    /// Zencontrol Pty Ltd (0x0AA8)
    ZencontrolPtyLtd = 0x0AA8,
    /// Urbanista AB (0x0AA7)
    UrbanistaAb = 0x0AA7,
    /// Realityworks, inc. (0x0AA6)
    RealityworksInc = 0x0AA6,
    /// Shenzhen Uascent Technology Co., Ltd (0x0AA5)
    ShenzhenUascentTechnologyCoLtd = 0x0AA5,
    /// FAZEPRO LLC (0x0AA4)
    FazeproLlc = 0x0AA4,
    /// DIC Corporation (0x0AA3)
    DicCorporation = 0x0AA3,
    /// Care Bloom, LLC (0x0AA2)
    CareBloomLlc = 0x0AA2,
    /// LINCOGN TECHNOLOGY CO. LIMITED (0x0AA1)
    LincognTechnologyCoLimited = 0x0AA1,
    /// Loy Tec electronics GmbH (0x0AA0)
    LoyTecElectronicsGmbH = 0x0AA0,
    /// ista International GmbH (0x0A9F)
    IstaInternationalGmbH = 0x0A9F,
    /// LifePlus, Inc. (0x0A9E)
    LifePlusInc = 0x0A9E,
    /// Canon Finetech Nisca Inc. (0x0A9D)
    CanonFinetechNiscaInc = 0x0A9D,
    /// Xi'an Fengyu Information Technology Co., Ltd. (0x0A9C)
    XiAnFengyuInformationTechnologyCoLtd = 0x0A9C,
    /// Eello LLC (0x0A9B)
    EelloLlc = 0x0A9B,
    /// TEMKIN ASSOCIATES, LLC (0x0A9A)
    TemkinAssociatesLlc = 0x0A9A,
    /// Shanghai high-flying electronics technology Co.,Ltd (0x0A99)
    ShanghaiHighFlyingElectronicsTechnologyCoLtd = 0x0A99,
    /// Foil, Inc. (0x0A98)
    FoilInc = 0x0A98,
    /// SensTek (0x0A97)
    SensTek = 0x0A97,
    /// Lightricity Ltd (0x0A96)
    LightricityLtd = 0x0A96,
    /// Pamex Inc. (0x0A95)
    PamexInc = 0x0A95,
    /// OOBIK Inc. (0x0A94)
    OobikInc = 0x0A94,
    /// GiPStech S.r.l. (0x0A93)
    GiPStechSRL = 0x0A93,
    /// Carestream Dental LLC (0x0A92)
    CarestreamDentalLlc = 0x0A92,
    /// Monarch International Inc. (0x0A91)
    MonarchInternationalInc = 0x0A91,
    /// Shenzhen Grandsun Electronic Co.,Ltd. (0x0A90)
    ShenzhenGrandsunElectronicCoLtd = 0x0A90,
    /// TOTO LTD. (0x0A8F)
    TotoLtd = 0x0A8F,
    /// Perfect Company (0x0A8E)
    PerfectCompany = 0x0A8E,
    /// JCM TECHNOLOGIES S.A. (0x0A8D)
    JcmTechnologiesSA = 0x0A8D,
    /// DelpSys, s.r.o. (0x0A8C)
    DelpSysSRO = 0x0A8C,
    /// SANlight GmbH (0x0A8B)
    SaNlightGmbH = 0x0A8B,
    /// HAINBUCH GMBH SPANNENDE TECHNIK (0x0A8A)
    HainbuchGmbhSpannendeTechnik = 0x0A8A,
    /// SES-Imagotag (0x0A89)
    SesImagotag = 0x0A89,
    /// PSA Peugeot Citroen (0x0A88)
    PsaPeugeotCitroen = 0x0A88,
    /// Shanghai Smart System Technology Co., Ltd (0x0A87)
    ShanghaiSmartSystemTechnologyCoLtd = 0x0A87,
    /// ALIZENT International (0x0A86)
    AlizentInternational = 0x0A86,
    /// Snowball Technology Co., Ltd. (0x0A85)
    SnowballTechnologyCoLtd = 0x0A85,
    /// Greennote Inc, (0x0A84)
    GreennoteInc = 0x0A84,
    /// Rivata, Inc. (0x0A83)
    RivataInc = 0x0A83,
    /// Corsair (0x0A82)
    Corsair = 0x0A82,
    /// Universal Biosensors Pty Ltd (0x0A81)
    UniversalBiosensorsPtyLtd = 0x0A81,
    /// Cleer Limited (0x0A80)
    CleerLimited = 0x0A80,
    /// Intuity Medical (0x0A7F)
    IntuityMedical = 0x0A7F,
    /// KEBA Handover Automation GmbH (0x0A7E)
    KebaHandoverAutomationGmbH = 0x0A7E,
    /// Freedman Electronics Pty Ltd (0x0A7D)
    FreedmanElectronicsPtyLtd = 0x0A7D,
    /// WAFERLOCK (0x0A7C)
    Waferlock = 0x0A7C,
    /// UniqAir Oy (0x0A7B)
    UniqAirOy = 0x0A7B,
    /// Emlid Limited (0x0A7A)
    EmlidLimited = 0x0A7A,
    /// Webasto SE (0x0A79)
    WebastoSe = 0x0A79,
    /// Shenzhen Sunricher Technology Limited (0x0A78)
    ShenzhenSunricherTechnologyLimited = 0x0A78,
    /// AXTRO PTE. LTD. (0x0A77)
    AxtroPteLtd = 0x0A77,
    /// Synaptics Incorporated (0x0A76)
    SynapticsIncorporated = 0x0A76,
    /// Delta Cycle Corporation (0x0A75)
    DeltaCycleCorporation = 0x0A75,
    /// MICROSON S.A. (0x0A74)
    MicrosonSA = 0x0A74,
    /// Innohome Oy (0x0A73)
    InnohomeOy = 0x0A73,
    /// Jumo GmbH & Co. KG (0x0A72)
    JumoGmbHAndCoKg = 0x0A72,
    /// Senquip Pty Ltd (0x0A71)
    SenquipPtyLtd = 0x0A71,
    /// Ooma (0x0A70)
    Ooma = 0x0A70,
    /// Warner Bros. (0x0A6F)
    WarnerBros = 0x0A6F,
    /// Pac Sane Limited (0x0A6E)
    PacSaneLimited = 0x0A6E,
    /// KUUKANJYOKIN Co.,Ltd. (0x0A6D)
    KuukanjyokinCoLtd = 0x0A6D,
    /// Pokkels (0x0A6C)
    Pokkels = 0x0A6C,
    /// Olympic Ophthalmics, Inc. (0x0A6B)
    OlympicOphthalmicsInc = 0x0A6B,
    /// Scribble Design Inc. (0x0A6A)
    ScribbleDesignInc = 0x0A6A,
    /// HAPPIEST BABY, INC. (0x0A69)
    HappiestBabyInc = 0x0A69,
    /// Focus Ingenieria SRL (0x0A68)
    FocusIngenieriaSrl = 0x0A68,
    /// Beijing SuperHexa Century Technology CO. Ltd (0x0A67)
    BeijingSuperHexaCenturyTechnologyCoLtd = 0x0A67,
    /// JUSTMORPH PTE. LTD. (0x0A66)
    JustmorphPteLtd = 0x0A66,
    /// Lytx, INC. (0x0A65)
    LytxInc = 0x0A65,
    /// Geopal system A/S (0x0A64)
    GeopalSystemAS = 0x0A64,
    /// Gremsy JSC (0x0A63)
    GremsyJsc = 0x0A63,
    /// MOKO TECHNOLOGY Ltd (0x0A62)
    MokoTechnologyLtd = 0x0A62,
    /// Smart Parks B.V. (0x0A61)
    SmartParksBV = 0x0A61,
    /// DATANG SEMICONDUCTOR TECHNOLOGY CO.,LTD (0x0A60)
    DatangSemiconductorTechnologyCoLtd = 0x0A60,
    /// stryker (0x0A5F)
    Stryker = 0x0A5F,
    /// LaceClips llc (0x0A5E)
    LaceClipsLlc = 0x0A5E,
    /// MG Energy Systems B.V. (0x0A5D)
    MgEnergySystemsBV = 0x0A5D,
    /// Innovative Design Labs Inc. (0x0A5C)
    InnovativeDesignLabsInc = 0x0A5C,
    /// LEGIC Identsystems AG (0x0A5B)
    LegicIdentsystemsAg = 0x0A5B,
    /// Sontheim Industrie Elektronik GmbH (0x0A5A)
    SontheimIndustrieElektronikGmbH = 0x0A5A,
    /// TourBuilt, LLC (0x0A59)
    TourBuiltLlc = 0x0A59,
    /// Indigo Diabetes (0x0A58)
    IndigoDiabetes = 0x0A58,
    /// Meizhou Guo Wei Electronics Co., Ltd (0x0A57)
    MeizhouGuoWeiElectronicsCoLtd = 0x0A57,
    /// ambie (0x0A56)
    Ambie = 0x0A56,
    /// Inugo Systems Limited (0x0A55)
    InugoSystemsLimited = 0x0A55,
    /// SQL Technologies Corp. (0x0A54)
    SqlTechnologiesCorp = 0x0A54,
    /// KKM COMPANY LIMITED (0x0A53)
    KkmCompanyLimited = 0x0A53,
    /// Follow Sense Europe B.V. (0x0A52)
    FollowSenseEuropeBV = 0x0A52,
    /// CSIRO (0x0A51)
    Csiro = 0x0A51,
    /// Nextscape Inc. (0x0A50)
    NextscapeInc = 0x0A50,
    /// VANMOOF Global Holding B.V. (0x0A4F)
    VanmoofGlobalHoldingBV = 0x0A4F,
    /// Toytec Corporation (0x0A4E)
    ToytecCorporation = 0x0A4E,
    /// Lockn Technologies Private Limited (0x0A4D)
    LocknTechnologiesPrivateLimited = 0x0A4D,
    /// SiFli Technologies (shanghai) Inc. (0x0A4C)
    SiFliTechnologiesShanghaiInc = 0x0A4C,
    /// MistyWest Energy and Transport Ltd. (0x0A4B)
    MistyWestEnergyAndTransportLtd = 0x0A4B,
    /// Map Large, Inc. (0x0A4A)
    MapLargeInc = 0x0A4A,
    /// Venture Research Inc. (0x0A49)
    VentureResearchInc = 0x0A49,
    /// JRC Mobility Inc. (0x0A48)
    JrcMobilityInc = 0x0A48,
    /// The Wand Company Ltd (0x0A47)
    TheWandCompanyLtd = 0x0A47,
    /// Beijing HC-Infinite Technology Limited (0x0A46)
    BeijingHcInfiniteTechnologyLimited = 0x0A46,
    /// 3SI Security Systems, Inc (0x0A45)
    _3SiSecuritySystemsInc = 0x0A45,
    /// Novidan, Inc. (0x0A44)
    NovidanInc = 0x0A44,
    /// Busch Systems International Inc. (0x0A43)
    BuschSystemsInternationalInc = 0x0A43,
    /// Motionalysis, Inc. (0x0A42)
    MotionalysisInc = 0x0A42,
    /// OPEX Corporation (0x0A41)
    OpexCorporation = 0x0A41,
    /// GEWISS S.p.A. (0x0A40)
    GewissSPA = 0x0A40,
    /// Shenzhen Yopeak Optoelectronics Technology Co., Ltd. (0x0A3F)
    ShenzhenYopeakOptoelectronicsTechnologyCoLtd = 0x0A3F,
    /// Hefei Yunlian Semiconductor Co., Ltd (0x0A3E)
    HefeiYunlianSemiconductorCoLtd = 0x0A3E,
    /// DELABIE (0x0A3D)
    Delabie = 0x0A3D,
    /// Siteco GmbH (0x0A3C)
    SitecoGmbH = 0x0A3C,
    /// Galileo Technology Limited (0x0A3B)
    GalileoTechnologyLimited = 0x0A3B,
    /// Incotex Co. Ltd. (0x0A3A)
    IncotexCoLtd = 0x0A3A,
    /// BLUETICKETING SRL (0x0A39)
    BlueticketingSrl = 0x0A39,
    /// Bouffalo Lab (Nanjing)., Ltd. (0x0A38)
    BouffaloLabNanjingLtd = 0x0A38,
    /// 2587702 Ontario Inc. (0x0A37)
    _2587702OntarioInc = 0x0A37,
    /// NGK SPARK PLUG CO., LTD. (0x0A36)
    NgkSparkPlugCoLtd = 0x0A36,
    /// safectory GmbH (0x0A35)
    SafectoryGmbH = 0x0A35,
    /// Luxer Corporation (0x0A34)
    LuxerCorporation = 0x0A34,
    /// WMF AG (0x0A33)
    WmfAg = 0x0A33,
    /// Pinnacle Technology, Inc. (0x0A32)
    PinnacleTechnologyInc = 0x0A32,
    /// Nevro Corp. (0x0A31)
    NevroCorp = 0x0A31,
    /// Air-Weigh (0x0A30)
    AirWeigh = 0x0A30,
    /// Instamic, Inc. (0x0A2F)
    InstamicInc = 0x0A2F,
    /// Zuma Array Limited (0x0A2E)
    ZumaArrayLimited = 0x0A2E,
    /// Shenzhen Feasycom Technology Co., Ltd. (0x0A2D)
    ShenzhenFeasycomTechnologyCoLtd = 0x0A2D,
    /// Shenzhen H&T Intelligent Control Co., Ltd (0x0A2C)
    ShenzhenHAndTIntelligentControlCoLtd = 0x0A2C,
    /// PaceBait IVS (0x0A2B)
    PaceBaitIvs = 0x0A2B,
    /// Yamaha Corporation (0x0A2A)
    YamahaCorporation = 0x0A2A,
    /// Worthcloud Technology Co.,Ltd (0x0A29)
    WorthcloudTechnologyCoLtd = 0x0A29,
    /// NanoFlex Power Corporation (0x0A28)
    NanoFlexPowerCorporation = 0x0A28,
    /// AYU DEVICES PRIVATE LIMITED (0x0A27)
    AyuDevicesPrivateLimited = 0x0A27,
    /// Louis Vuitton (0x0A26)
    LouisVuitton = 0x0A26,
    /// Eran Financial Services LLC (0x0A25)
    EranFinancialServicesLlc = 0x0A25,
    /// Atmosic Technologies, Inc. (0x0A24)
    AtmosicTechnologiesInc = 0x0A24,
    /// BIXOLON CO.,LTD (0x0A23)
    BixolonCoLtd = 0x0A23,
    /// DAIICHIKOSHO CO., LTD. (0x0A22)
    DaiichikoshoCoLtd = 0x0A22,
    /// Apollogic Sp. z o.o. (0x0A21)
    ApollogicSpZOO = 0x0A21,
    /// Jiangxi Innotech Technology Co., Ltd (0x0A20)
    JiangxiInnotechTechnologyCoLtd = 0x0A20,
    /// DeVilbiss Healthcare LLC (0x0A1F)
    DeVilbissHealthcareLlc = 0x0A1F,
    /// CombiQ AB (0x0A1E)
    CombiQAb = 0x0A1E,
    /// API-K (0x0A1D)
    ApiK = 0x0A1D,
    /// INPEAK S.C. (0x0A1C)
    InpeakSC = 0x0A1C,
    /// Embrava Pty Ltd (0x0A1B)
    EmbravaPtyLtd = 0x0A1B,
    /// Link Labs, Inc. (0x0A1A)
    LinkLabsInc = 0x0A1A,
    /// Maxell, Ltd. (0x0A19)
    MaxellLtd = 0x0A19,
    /// Cambridge Animal Technologies Ltd (0x0A18)
    CambridgeAnimalTechnologiesLtd = 0x0A18,
    /// Plume Design Inc (0x0A17)
    PlumeDesignInc = 0x0A17,
    /// RIDE VISION LTD (0x0A16)
    RideVisionLtd = 0x0A16,
    /// Syng Inc (0x0A15)
    SyngInc = 0x0A15,
    /// CROXEL, INC. (0x0A14)
    CroxelInc = 0x0A14,
    /// Tec4med LifeScience GmbH (0x0A13)
    Tec4MedLifeScienceGmbH = 0x0A13,
    /// Dyson Technology Limited (0x0A12)
    DysonTechnologyLimited = 0x0A12,
    /// Sensolus (0x0A11)
    Sensolus = 0x0A11,
    /// SUBARU Corporation (0x0A10)
    SubaruCorporation = 0x0A10,
    /// LIXIL Corporation (0x0A0F)
    LixilCorporation = 0x0A0F,
    /// Roland Corporation (0x0A0E)
    RolandCorporation = 0x0A0E,
    /// Blue Peacock GmbH (0x0A0D)
    BluePeacockGmbH = 0x0A0D,
    /// Shanghai Yidian Intelligent Technology Co., Ltd. (0x0A0C)
    ShanghaiYidianIntelligentTechnologyCoLtd = 0x0A0C,
    /// SIANA Systems (0x0A0B)
    SianaSystems = 0x0A0B,
    /// Volan Technology Inc. (0x0A0A)
    VolanTechnologyInc = 0x0A0A,
    /// ECCT (0x0A09)
    Ecct = 0x0A09,
    /// Oras Oy (0x0A08)
    OrasOy = 0x0A08,
    /// Reflow Pty Ltd (0x0A07)
    ReflowPtyLtd = 0x0A07,
    /// Shanghai wuqi microelectronics Co.,Ltd (0x0A06)
    ShanghaiWuqiMicroelectronicsCoLtd = 0x0A06,
    /// Southwire Company, LLC (0x0A05)
    SouthwireCompanyLlc = 0x0A05,
    /// Flosonics Medical (0x0A04)
    FlosonicsMedical = 0x0A04,
    /// donutrobotics Co., Ltd. (0x0A03)
    DonutroboticsCoLtd = 0x0A03,
    /// Ayxon-Dynamics GmbH (0x0A02)
    AyxonDynamicsGmbH = 0x0A02,
    /// Cleveron AS (0x0A01)
    CleveronAs = 0x0A01,
    /// Ampler Bikes OU (0x0A00)
    AmplerBikesOu = 0x0A00,
    /// AIRSTAR (0x09FF)
    Airstar = 0x09FF,
    /// Lichtvision Engineering GmbH (0x09FE)
    LichtvisionEngineeringGmbH = 0x09FE,
    /// Keep Technologies, Inc. (0x09FD)
    KeepTechnologiesInc = 0x09FD,
    /// Confidex (0x09FC)
    Confidex = 0x09FC,
    /// TOITU CO., LTD. (0x09FB)
    ToituCoLtd = 0x09FB,
    /// Listen Technologies Corporation (0x09FA)
    ListenTechnologiesCorporation = 0x09FA,
    /// Hangzhou Yaguan Technology Co. LTD (0x09F9)
    HangzhouYaguanTechnologyCoLtd = 0x09F9,
    /// R.O. S.R.L. (0x09F8)
    ROSRL = 0x09F8,
    /// SENSATEC Co., Ltd. (0x09F7)
    SensatecCoLtd = 0x09F7,
    /// Mobile Action Technology Inc. (0x09F6)
    MobileActionTechnologyInc = 0x09F6,
    /// OKI Electric Industry Co., Ltd (0x09F5)
    OkiElectricIndustryCoLtd = 0x09F5,
    /// Spectrum Technologies, Inc. (0x09F4)
    SpectrumTechnologiesInc = 0x09F4,
    /// Beijing Zero Zero Infinity Technology Co.,Ltd. (0x09F3)
    BeijingZeroZeroInfinityTechnologyCoLtd = 0x09F3,
    /// Audeara Pty Ltd (0x09F2)
    AudearaPtyLtd = 0x09F2,
    /// OM Digital Solutions Corporation (0x09F1)
    OmDigitalSolutionsCorporation = 0x09F1,
    /// WatchGas B.V. (0x09F0)
    WatchGasBV = 0x09F0,
    /// Steinel Solutions AG (0x09EF)
    SteinelSolutionsAg = 0x09EF,
    /// OJMAR SA (0x09EE)
    OjmarSa = 0x09EE,
    /// Sibel Inc. (0x09ED)
    SibelInc = 0x09ED,
    /// Yukon advanced optics worldwide, UAB (0x09EC)
    YukonAdvancedOpticsWorldwideUab = 0x09EC,
    /// KEAN ELECTRONICS PTY LTD (0x09EB)
    KeanElectronicsPtyLtd = 0x09EB,
    /// Athlos Oy (0x09EA)
    AthlosOy = 0x09EA,
    /// LumenRadio AB (0x09E9)
    LumenRadioAb = 0x09E9,
    /// Melange Systems Pvt. Ltd. (0x09E8)
    MelangeSystemsPvtLtd = 0x09E8,
    /// Kabushikigaisha HANERON (0x09E7)
    KabushikigaishaHaneron = 0x09E7,
    /// Masonite Corporation (0x09E6)
    MasoniteCorporation = 0x09E6,
    /// Mobilogix (0x09E5)
    Mobilogix = 0x09E5,
    /// CPS AS (0x09E4)
    CpsAs = 0x09E4,
    /// Friday Home Aps (0x09E3)
    FridayHomeAps = 0x09E3,
    /// Wuhan Linptech Co.,Ltd. (0x09E2)
    WuhanLinptechCoLtd = 0x09E2,
    /// Tag-N-Trac Inc (0x09E1)
    TagNTracInc = 0x09E1,
    /// Preddio Technologies Inc. (0x09E0)
    PreddioTechnologiesInc = 0x09E0,
    /// Magnus Technology Sdn Bhd (0x09DF)
    MagnusTechnologySdnBhd = 0x09DF,
    /// JLD Technology Solutions, LLC (0x09DE)
    JldTechnologySolutionsLlc = 0x09DE,
    /// Innoware Development AB (0x09DD)
    InnowareDevelopmentAb = 0x09DD,
    /// AON2 Ltd. (0x09DC)
    Aon2Ltd = 0x09DC,
    /// Bionic Avionics Inc. (0x09DB)
    BionicAvionicsInc = 0x09DB,
    /// Nagravision SA (0x09DA)
    NagravisionSa = 0x09DA,
    /// VivoSensMedical GmbH (0x09D9)
    VivoSensMedicalGmbH = 0x09D9,
    /// Synergy Tecnologia em Sistemas Ltda (0x09D8)
    SynergyTecnologiaEmSistemasLtda = 0x09D8,
    /// Coyotta (0x09D7)
    Coyotta = 0x09D7,
    /// EAR TEKNIK ISITME VE ODIOMETRI CIHAZLARI SANAYI VE TICARET ANONIM SIRKETI (0x09D6)
    EarTeknikIsitmeVeOdiometriCihazlariSanayiVeTicaretAnonimSirketi = 0x09D6,
    /// GEAR RADIO ELECTRONICS CORP. (0x09D5)
    GearRadioElectronicsCorp = 0x09D5,
    /// ORBIS Inc. (0x09D4)
    OrbisInc = 0x09D4,
    /// HeartHero, inc. (0x09D3)
    HeartHeroInc = 0x09D3,
    /// Temperature Sensitive Solutions Systems Sweden AB (0x09D2)
    TemperatureSensitiveSolutionsSystemsSwedenAb = 0x09D2,
    /// ABLEPAY TECHNOLOGIES AS (0x09D1)
    AblepayTechnologiesAs = 0x09D1,
    /// Chess Wise B.V. (0x09D0)
    ChessWiseBV = 0x09D0,
    /// BlueStreak IoT, LLC (0x09CF)
    BlueStreakIoTLlc = 0x09CF,
    /// Julius Blum GmbH (0x09CE)
    JuliusBlumGmbH = 0x09CE,
    /// Blyott (0x09CD)
    Blyott = 0x09CD,
    /// Senso4s d.o.o. (0x09CC)
    Senso4SDOO = 0x09CC,
    /// Hx Engineering, LLC (0x09CB)
    HxEngineeringLlc = 0x09CB,
    /// Mobitrace (0x09CA)
    Mobitrace = 0x09CA,
    /// CrowdGlow Ltd (0x09C9)
    CrowdGlowLtd = 0x09C9,
    /// XUNTONG (0x09C8)
    Xuntong = 0x09C8,
    /// Combustion, LLC (0x09C7)
    CombustionLlc = 0x09C7,
    /// Honor Device Co., Ltd. (0x09C6)
    HonorDeviceCoLtd = 0x09C6,
    /// HungYi Microelectronics Co.,Ltd. (0x09C5)
    HungYiMicroelectronicsCoLtd = 0x09C5,
    /// UVISIO (0x09C4)
    Uvisio = 0x09C4,
    /// JAPAN TOBACCO INC. (0x09C3)
    JapanTobaccoInc = 0x09C3,
    /// Universal Audio, Inc. (0x09C2)
    UniversalAudioInc = 0x09C2,
    /// Rosewill (0x09C1)
    Rosewill = 0x09C1,
    /// AnotherBrain inc. (0x09C0)
    AnotherBrainInc = 0x09C0,
    /// Span.IO, Inc. (0x09BF)
    SpanIoInc = 0x09BF,
    /// Vessel Ltd. (0x09BE)
    VesselLtd = 0x09BE,
    /// Centre Suisse d'Electronique et de Microtechnique SA (0x09BD)
    CentreSuisseDElectroniqueEtDeMicrotechniqueSa = 0x09BD,
    /// Aerosens LLC (0x09BC)
    AerosensLlc = 0x09BC,
    /// SkyStream Corporation (0x09BB)
    SkyStreamCorporation = 0x09BB,
    /// Elimo Engineering Ltd (0x09BA)
    ElimoEngineeringLtd = 0x09BA,
    /// SAVOY ELECTRONIC LIGHTING (0x09B9)
    SavoyElectronicLighting = 0x09B9,
    /// PlayerData Limited (0x09B8)
    PlayerDataLimited = 0x09B8,
    /// Bout Labs, LLC (0x09B7)
    BoutLabsLlc = 0x09B7,
    /// Pegasus Technologies, Inc. (0x09B6)
    PegasusTechnologiesInc = 0x09B6,
    /// AUTEC Gesellschaft fuer Automationstechnik mbH (0x09B5)
    AutecGesellschaftFuerAutomationstechnikMbH = 0x09B5,
    /// PentaLock Aps. (0x09B4)
    PentaLockAps = 0x09B4,
    /// BlueX Microelectronics Corp Ltd. (0x09B3)
    BlueXMicroelectronicsCorpLtd = 0x09B3,
    /// DYPHI (0x09B2)
    Dyphi = 0x09B2,
    /// BLINQY (0x09B1)
    Blinqy = 0x09B1,
    /// Deublin Company, LLC (0x09B0)
    DeublinCompanyLlc = 0x09B0,
    /// ifLink Open Community (0x09AF)
    IfLinkOpenCommunity = 0x09AF,
    /// Pozyx NV (0x09AE)
    PozyxNv = 0x09AE,
    /// Narhwall Inc. (0x09AD)
    NarhwallInc = 0x09AD,
    /// Ambiq (0x09AC)
    Ambiq = 0x09AC,
    /// DashLogic, Inc. (0x09AB)
    DashLogicInc = 0x09AB,
    /// PHOTODYNAMIC INCORPORATED (0x09AA)
    PhotodynamicIncorporated = 0x09AA,
    /// Nippon Ceramic Co.,Ltd. (0x09A9)
    NipponCeramicCoLtd = 0x09A9,
    /// KHN Solutions LLC (0x09A8)
    KhnSolutionsLlc = 0x09A8,
    /// Paybuddy ApS (0x09A7)
    PaybuddyApS = 0x09A7,
    /// BEIJING ELECTRIC VEHICLE CO.,LTD (0x09A6)
    BeijingElectricVehicleCoLtd = 0x09A6,
    /// Security Enhancement Systems, LLC (0x09A5)
    SecurityEnhancementSystemsLlc = 0x09A5,
    /// KUMHO ELECTRICS, INC (0x09A4)
    KumhoElectricsInc = 0x09A4,
    /// ARDUINO SA (0x09A3)
    ArduinoSa = 0x09A3,
    /// ENGAGENOW DATA SCIENCES PRIVATE LIMITED (0x09A2)
    EngagenowDataSciencesPrivateLimited = 0x09A2,
    /// VOS Systems, LLC (0x09A1)
    VosSystemsLlc = 0x09A1,
    /// Proof Diagnostics, Inc. (0x09A0)
    ProofDiagnosticsInc = 0x09A0,
    /// Koya Medical, Inc. (0x099F)
    KoyaMedicalInc = 0x099F,
    /// Step One Limited (0x099E)
    StepOneLimited = 0x099E,
    /// YKK AP Inc. (0x099D)
    YkkApInc = 0x099D,
    /// deister electronic GmbH (0x099C)
    DeisterElectronicGmbH = 0x099C,
    /// Sendum Wireless Corporation (0x099B)
    SendumWirelessCorporation = 0x099B,
    /// New Audio LLC (0x099A)
    NewAudioLlc = 0x099A,
    /// eTactica ehf (0x0999)
    ETacticaEhf = 0x0999,
    /// Pixie Dust Technologies, Inc. (0x0998)
    PixieDustTechnologiesInc = 0x0998,
    /// NextMind (0x0997)
    NextMind = 0x0997,
    /// C. & E. Fein GmbH (0x0996)
    CAndEFeinGmbH = 0x0996,
    /// Bronkhorst High-Tech B.V. (0x0995)
    BronkhorstHighTechBV = 0x0995,
    /// VT42 Pty Ltd (0x0994)
    Vt42PtyLtd = 0x0994,
    /// Absolute Audio Labs B.V. (0x0993)
    AbsoluteAudioLabsBV = 0x0993,
    /// Big Kaiser Precision Tooling Ltd (0x0992)
    BigKaiserPrecisionToolingLtd = 0x0992,
    /// Telenor ASA (0x0991)
    TelenorAsa = 0x0991,
    /// Anton Paar GmbH (0x0990)
    AntonPaarGmbH = 0x0990,
    /// Aktiebolaget Regin (0x098F)
    AktiebolagetRegin = 0x098F,
    /// ADVEEZ (0x098E)
    Adveez = 0x098E,
    /// C3-WIRELESS, LLC (0x098D)
    C3WirelessLlc = 0x098D,
    /// bGrid B.V. (0x098C)
    BGridBV = 0x098C,
    /// Mequonic Engineering, S.L. (0x098B)
    MequonicEngineeringSL = 0x098B,
    /// Biovigil (0x098A)
    Biovigil = 0x098A,
    /// WIKA Alexander Wiegand SE & Co.KG (0x0989)
    WikaAlexanderWiegandSeAndCoKg = 0x0989,
    /// BHM-Tech Produktionsgesellschaft m.b.H (0x0988)
    BhmTechProduktionsgesellschaftMBH = 0x0988,
    /// TSE BRAKES, INC. (0x0987)
    TseBrakesInc = 0x0987,
    /// Cello Hill, LLC (0x0986)
    CelloHillLlc = 0x0986,
    /// Lumos Health Inc. (0x0985)
    LumosHealthInc = 0x0985,
    /// TeraTron GmbH (0x0984)
    TeraTronGmbH = 0x0984,
    /// Feedback Sports LLC (0x0983)
    FeedbackSportsLlc = 0x0983,
    /// ELPRO-BUCHS AG (0x0982)
    ElproBuchsAg = 0x0982,
    /// Bernard Krone Holding SE & Co.KG (0x0981)
    BernardKroneHoldingSeAndCoKg = 0x0981,
    /// DEKRA TESTING AND CERTIFICATION, S.A.U. (0x0980)
    DekraTestingAndCertificationSAU = 0x0980,
    /// ISEMAR S.R.L. (0x097F)
    IsemarSRL = 0x097F,
    /// SonicSensory Inc (0x097E)
    SonicSensoryInc = 0x097E,
    /// CLB B.V. (0x097D)
    ClbBV = 0x097D,
    /// Thorley Industries, LLC (0x097C)
    ThorleyIndustriesLlc = 0x097C,
    /// CTEK Sweden AB (0x097B)
    CtekSwedenAb = 0x097B,
    /// CORE CORPORATION (0x097A)
    CoreCorporation = 0x097A,
    /// BIOTRONIK SE & Co. KG (0x0979)
    BiotronikSeAndCoKg = 0x0979,
    /// ZifferEins GmbH & Co. KG (0x0978)
    ZifferEinsGmbHAndCoKg = 0x0978,
    /// TOYOTA motor corporation (0x0977)
    ToyotaMotorCorporation = 0x0977,
    /// Fauna Audio GmbH (0x0976)
    FaunaAudioGmbH = 0x0976,
    /// BlueIOT(Beijing) Technology Co.,Ltd (0x0975)
    BlueIotBeijingTechnologyCoLtd = 0x0975,
    /// ABEYE (0x0974)
    Abeye = 0x0974,
    /// Popit Oy (0x0973)
    PopitOy = 0x0973,
    /// Closed Joint Stock Company "Zavod Flometr" ("Zavod Flometr" CJSC) (0x0972)
    ClosedJointStockCompanyZavodFlometrZavodFlometrCjsc = 0x0972,
    /// GA (0x0971)
    Ga = 0x0971,
    /// IBA Dosimetry GmbH (0x0970)
    IbaDosimetryGmbH = 0x0970,
    /// Lund Motion Products, Inc. (0x096F)
    LundMotionProductsInc = 0x096F,
    /// Band Industries, inc. (0x096E)
    BandIndustriesInc = 0x096E,
    /// Gunwerks, LLC (0x096D)
    GunwerksLlc = 0x096D,
    /// 9374-7319 Quebec inc (0x096C)
    _93747319QuebecInc = 0x096C,
    /// Guide ID B.V. (0x096B)
    GuideIdBV = 0x096B,
    /// dricos, Inc. (0x096A)
    DricosInc = 0x096A,
    /// Woan Technology (Shenzhen) Co., Ltd. (0x0969)
    WoanTechnologyShenzhenCoLtd = 0x0969,
    /// Actev Motors, Inc. (0x0968)
    ActevMotorsInc = 0x0968,
    /// Neo Materials and Consulting Inc. (0x0967)
    NeoMaterialsAndConsultingInc = 0x0967,
    /// PointGuard, LLC (0x0966)
    PointGuardLlc = 0x0966,
    /// Asahi Kasei Corporation (0x0965)
    AsahiKaseiCorporation = 0x0965,
    /// Countrymate Technology Limited (0x0964)
    CountrymateTechnologyLimited = 0x0964,
    /// Moonbird BV (0x0963)
    MoonbirdBv = 0x0963,
    /// GL Solutions K.K. (0x0962)
    GlSolutionsKK = 0x0962,
    /// Linkura AB (0x0961)
    LinkuraAb = 0x0961,
    /// Sena Technologies Inc. (0x0960)
    SenaTechnologiesInc = 0x0960,
    /// NUANCE HEARING LTD (0x095F)
    NuanceHearingLtd = 0x095F,
    /// BioEchoNet inc. (0x095E)
    BioEchoNetInc = 0x095E,
    /// Electronic Theatre Controls (0x095D)
    ElectronicTheatreControls = 0x095D,
    /// LogiLube, LLC (0x095C)
    LogiLubeLlc095C = 0x095C,
    /// Lismore Instruments Limited (0x095B)
    LismoreInstrumentsLimited = 0x095B,
    /// Selekt Bilgisayar, lletisim Urunleri lnsaat Sanayi ve Ticaret Limited Sirketi (0x095A)
    SelektBilgisayarLletisimUrunleriLnsaatSanayiVeTicaretLimitedSirketi = 0x095A,
    /// HerdDogg, Inc (0x0959)
    HerdDoggInc = 0x0959,
    /// ZTE Corporation (0x0958)
    ZteCorporation = 0x0958,
    /// Ohsung Electronics (0x0957)
    OhsungElectronics = 0x0957,
    /// Kerlink (0x0956)
    Kerlink = 0x0956,
    /// Breville Group (0x0955)
    BrevilleGroup = 0x0955,
    /// Julbo (0x0954)
    Julbo = 0x0954,
    /// LogiLube, LLC (0x0953)
    LogiLubeLlc0953 = 0x0953,
    /// Apptricity Corporation (0x0952)
    ApptricityCorporation = 0x0952,
    /// PPRS (0x0951)
    Pprs = 0x0951,
    /// Capetech (0x0950)
    Capetech = 0x0950,
    /// Limited Liability Company "Mikrotikls" (0x094F)
    LimitedLiabilityCompanyMikrotikls = 0x094F,
    /// PassiveBolt, Inc. (0x094E)
    PassiveBoltInc = 0x094E,
    /// tkLABS INC. (0x094D)
    TkLabsInc = 0x094D,
    /// GimmiSys GmbH (0x094C)
    GimmiSysGmbH = 0x094C,
    /// Kindeva Drug Delivery L.P. (0x094B)
    KindevaDrugDeliveryLP = 0x094B,
    /// Zwift, Inc. (0x094A)
    ZwiftInc = 0x094A,
    /// Metronom Health Europe (0x0949)
    MetronomHealthEurope = 0x0949,
    /// Wearable Link Limited (0x0948)
    WearableLinkLimited = 0x0948,
    /// First Light Technologies Ltd. (0x0947)
    FirstLightTechnologiesLtd = 0x0947,
    /// AMC International Alfa Metalcraft Corporation AG (0x0946)
    AmcInternationalAlfaMetalcraftCorporationAg = 0x0946,
    /// Globe (Jiangsu) Co., Ltd (0x0945)
    GlobeJiangsuCoLtd = 0x0945,
    /// Agitron d.o.o. (0x0944)
    AgitronDOO = 0x0944,
    /// TRANSSION HOLDINGS LIMITED (0x0942)
    TranssionHoldingsLimited = 0x0942,
    /// Rivian Automotive, LLC (0x0941)
    RivianAutomotiveLlc = 0x0941,
    /// Hero Workout GmbH (0x0940)
    HeroWorkoutGmbH = 0x0940,
    /// JEPICO Corporation (0x093F)
    JepicoCorporation = 0x093F,
    /// Catalyft Labs, Inc. (0x093E)
    CatalyftLabsInc = 0x093E,
    /// Adolf Wuerth GmbH & Co KG (0x093D)
    AdolfWuerthGmbHAndCoKg = 0x093D,
    /// Xenoma Inc. (0x093C)
    XenomaInc = 0x093C,
    /// ENSESO LLC (0x093B)
    EnsesoLlc = 0x093B,
    /// LinkedSemi Microelectronics (Xiamen) Co., Ltd (0x093A)
    LinkedSemiMicroelectronicsXiamenCoLtd = 0x093A,
    /// ASTEM Co.,Ltd. (0x0939)
    AstemCoLtd = 0x0939,
    /// Henway Technologies, LTD. (0x0938)
    HenwayTechnologiesLtd = 0x0938,
    /// RealThingks GmbH (0x0937)
    RealThingksGmbH = 0x0937,
    /// Elekon AG (0x0936)
    ElekonAg = 0x0936,
    /// Reconnect, Inc. (0x0935)
    ReconnectInc = 0x0935,
    /// KiteSpring Inc. (0x0934)
    KiteSpringInc = 0x0934,
    /// SRAM (0x0933)
    Sram = 0x0933,
    /// BarVision, LLC (0x0932)
    BarVisionLlc = 0x0932,
    /// BREATHINGS Co., Ltd. (0x0931)
    BreathingsCoLtd = 0x0931,
    /// James Walker RotaBolt Limited (0x0930)
    JamesWalkerRotaBoltLimited = 0x0930,
    /// C.O.B.O. SpA (0x092F)
    COBOSpA = 0x092F,
    /// PS GmbH (0x092E)
    PsGmbH = 0x092E,
    /// Leggett & Platt, Incorporated (0x092D)
    LeggettAndPlattIncorporated = 0x092D,
    /// PCI Private Limited (0x092C)
    PciPrivateLimited = 0x092C,
    /// TekHome (0x092B)
    TekHome = 0x092B,
    /// Sappl Verwaltungs- und Betriebs GmbH (0x092A)
    SapplVerwaltungsUndBetriebsGmbH = 0x092A,
    /// Qingdao Haier Technology Co., Ltd. (0x0929)
    QingdaoHaierTechnologyCoLtd = 0x0929,
    /// AiRISTA (0x0928)
    AiRista = 0x0928,
    /// ROOQ GmbH (0x0927)
    RooqGmbH = 0x0927,
    /// Gooligum Technologies Pty Ltd (0x0926)
    GooligumTechnologiesPtyLtd = 0x0926,
    /// Yukai Engineering Inc. (0x0925)
    YukaiEngineeringInc = 0x0925,
    /// Fundacion Tecnalia Research and Innovation (0x0924)
    FundacionTecnaliaResearchAndInnovation = 0x0924,
    /// JSB TECH PTE LTD (0x0923)
    JsbTechPteLtd = 0x0923,
    /// Shanghai MXCHIP Information Technology Co., Ltd. (0x0922)
    ShanghaiMxchipInformationTechnologyCoLtd = 0x0922,
    /// KAHA PTE. LTD. (0x0921)
    KahaPteLtd = 0x0921,
    /// Omnisense Limited (0x0920)
    OmnisenseLimited = 0x0920,
    /// Myzee Technology (0x091F)
    MyzeeTechnology = 0x091F,
    /// Melbot Studios, Sociedad Limitada (0x091E)
    MelbotStudiosSociedadLimitada = 0x091E,
    /// Innokind, Inc. (0x091D)
    InnokindInc = 0x091D,
    /// Oblamatik AG (0x091C)
    OblamatikAg = 0x091C,
    /// Luminostics, Inc. (0x091B)
    LuminosticsInc = 0x091B,
    /// Albertronic BV (0x091A)
    AlbertronicBv = 0x091A,
    /// NO SMD LIMITED (0x0919)
    NoSmdLimited = 0x0919,
    /// Technosphere Labs Pvt. Ltd. (0x0918)
    TechnosphereLabsPvtLtd = 0x0918,
    /// ASR Microelectronics(ShenZhen)Co., Ltd. (0x0917)
    AsrMicroelectronicsShenZhenCoLtd = 0x0917,
    /// Ambient Sensors LLC (0x0916)
    AmbientSensorsLlc = 0x0916,
    /// Honda Motor Co., Ltd. (0x0915)
    HondaMotorCoLtd = 0x0915,
    /// INEO-SENSE (0x0914)
    IneoSense = 0x0914,
    /// Braveheart Wireless, Inc. (0x0913)
    BraveheartWirelessInc = 0x0913,
    /// Nerbio Medical Software Platforms Inc (0x0912)
    NerbioMedicalSoftwarePlatformsInc = 0x0912,
    /// Douglas Lighting Controls Inc. (0x0911)
    DouglasLightingControlsInc = 0x0911,
    /// ASR Microelectronics (Shanghai) Co., Ltd. (0x0910)
    AsrMicroelectronicsShanghaiCoLtd = 0x0910,
    /// VC Inc. (0x090F)
    VcInc = 0x090F,
    /// OPTIMUSIOT TECH LLP (0x090E)
    OptimusiotTechLlp = 0x090E,
    /// IOT Invent GmbH (0x090D)
    IotInventGmbH = 0x090D,
    /// Radiawave Technologies Co.,Ltd. (0x090C)
    RadiawaveTechnologiesCoLtd = 0x090C,
    /// EMBR labs, INC (0x090B)
    EmbrLabsInc = 0x090B,
    /// Zhuhai Hoksi Technology CO.,LTD (0x090A)
    ZhuhaiHoksiTechnologyCoLtd = 0x090A,
    /// 70mai Co.,Ltd. (0x0909)
    _70MaiCoLtd = 0x0909,
    /// Pinpoint Innovations Limited (0x0908)
    PinpointInnovationsLimited = 0x0908,
    /// User Hello, LLC (0x0907)
    UserHelloLlc = 0x0907,
    /// Scope Logistical Solutions (0x0906)
    ScopeLogisticalSolutions = 0x0906,
    /// Yandex Services AG (0x0905)
    YandexServicesAg = 0x0905,
    /// SUNCORPORATION (0x0904)
    Suncorporation = 0x0904,
    /// DATAMARS, Inc. (0x0903)
    DatamarsInc = 0x0903,
    /// TSC Auto-ID Technology Co., Ltd. (0x0902)
    TscAutoIdTechnologyCoLtd = 0x0902,
    /// Lucimed (0x0901)
    Lucimed = 0x0901,
    /// Beijing Zizai Technology Co., LTD. (0x0900)
    BeijingZizaiTechnologyCoLtd = 0x0900,
    /// Plastimold Products, Inc (0x08FF)
    PlastimoldProductsInc = 0x08FF,
    /// Ketronixs Sdn Bhd (0x08FE)
    KetronixsSdnBhd = 0x08FE,
    /// BioIntelliSense, Inc. (0x08FD)
    BioIntelliSenseInc = 0x08FD,
    /// Hill-Rom (0x08FC)
    HillRom = 0x08FC,
    /// Darkglass Electronics Oy (0x08FB)
    DarkglassElectronicsOy = 0x08FB,
    /// Troo Corporation (0x08FA)
    TrooCorporation = 0x08FA,
    /// Spacelabs Medical Inc. (0x08F9)
    SpacelabsMedicalInc = 0x08F9,
    /// instagrid GmbH (0x08F8)
    InstagridGmbH = 0x08F8,
    /// MTD Products Inc & Affiliates (0x08F7)
    MtdProductsIncAndAffiliates = 0x08F7,
    /// Dermal Photonics Corporation (0x08F6)
    DermalPhotonicsCorporation = 0x08F6,
    /// Tymtix Technologies Private Limited (0x08F5)
    TymtixTechnologiesPrivateLimited = 0x08F5,
    /// Kodimo Technologies Company Limited (0x08F4)
    KodimoTechnologiesCompanyLimited = 0x08F4,
    /// PSP - Pauli Services & Products GmbH (0x08F3)
    PspPauliServicesAndProductsGmbH = 0x08F3,
    /// Microoled (0x08F2)
    Microoled = 0x08F2,
    /// The L.S. Starrett Company (0x08F1)
    TheLSStarrettCompany = 0x08F1,
    /// Joovv, Inc. (0x08F0)
    JoovvInc = 0x08F0,
    /// Cumulus Digital Systems, Inc (0x08EF)
    CumulusDigitalSystemsInc = 0x08EF,
    /// Askey Computer Corp. (0x08EE)
    AskeyComputerCorp = 0x08EE,
    /// IMI Hydronic Engineering International SA (0x08ED)
    ImiHydronicEngineeringInternationalSa = 0x08ED,
    /// Denso Corporation (0x08EC)
    DensoCorporation = 0x08EC,
    /// Beijing Big Moment Technology Co., Ltd. (0x08EB)
    BeijingBigMomentTechnologyCoLtd = 0x08EB,
    /// COWBELL ENGINEERING CO.,LTD. (0x08EA)
    CowbellEngineeringCoLtd = 0x08EA,
    /// Taiwan Intelligent Home Corp. (0x08E9)
    TaiwanIntelligentHomeCorp = 0x08E9,
    /// Naonext (0x08E8)
    Naonext = 0x08E8,
    /// Barrot Technology Co.,Ltd. (0x08E7)
    BarrotTechnologyCoLtd = 0x08E7,
    /// Eneso Tecnologia de Adaptacion S.L. (0x08E6)
    EnesoTecnologiaDeAdaptacionSL = 0x08E6,
    /// Crowd Connected Ltd (0x08E5)
    CrowdConnectedLtd = 0x08E5,
    /// Rashidov ltd (0x08E4)
    RashidovLtd = 0x08E4,
    /// Republic Wireless, Inc. (0x08E3)
    RepublicWirelessInc = 0x08E3,
    /// Shenzhen Simo Technology co. LTD (0x08E2)
    ShenzhenSimoTechnologyCoLtd = 0x08E2,
    /// KOZO KEIKAKU ENGINEERING Inc. (0x08E1)
    KozoKeikakuEngineeringInc = 0x08E1,
    /// Philia Technology (0x08E0)
    PhiliaTechnology = 0x08E0,
    /// IRIS OHYAMA CO.,LTD. (0x08DF)
    IrisOhyamaCoLtd = 0x08DF,
    /// TE Connectivity Corporation (0x08DE)
    TeConnectivityCorporation = 0x08DE,
    /// code-Q (0x08DD)
    CodeQ = 0x08DD,
    /// SHENZHEN AUKEY E BUSINESS CO., LTD (0x08DC)
    ShenzhenAukeyEBusinessCoLtd = 0x08DC,
    /// Tertium Technology (0x08DB)
    TertiumTechnology = 0x08DB,
    /// Miridia Technology Incorporated (0x08DA)
    MiridiaTechnologyIncorporated = 0x08DA,
    /// Pointr Labs Limited (0x08D9)
    PointrLabsLimited = 0x08D9,
    /// WARES (0x08D8)
    Wares = 0x08D8,
    /// Inovonics Corp (0x08D7)
    InovonicsCorp = 0x08D7,
    /// Nome Oy (0x08D6)
    NomeOy = 0x08D6,
    /// KEYes (0x08D5)
    KeYes = 0x08D5,
    /// ADATA Technology Co., LTD. (0x08D4)
    AdataTechnologyCoLtd = 0x08D4,
    /// Novel Bits, LLC (0x08D3)
    NovelBitsLlc = 0x08D3,
    /// Virscient Limited (0x08D2)
    VirscientLimited = 0x08D2,
    /// Sensovium Inc. (0x08D1)
    SensoviumInc = 0x08D1,
    /// ESTOM Infotech Kft. (0x08D0)
    EstomInfotechKft = 0x08D0,
    /// betternotstealmybike UG (with limited liability) (0x08CF)
    BetternotstealmybikeUgWithLimitedLiability = 0x08CF,
    /// ZIMI CORPORATION (0x08CE)
    ZimiCorporation = 0x08CE,
    /// ifly (0x08CD)
    Ifly = 0x08CD,
    /// TGM TECHNOLOGY CO., LTD. (0x08CC)
    TgmTechnologyCoLtd = 0x08CC,
    /// JT INNOVATIONS LIMITED (0x08CB)
    JtInnovationsLimited = 0x08CB,
    /// Nubia Technology Co.,Ltd. (0x08CA)
    NubiaTechnologyCoLtd = 0x08CA,
    /// Noventa AG (0x08C9)
    NoventaAg = 0x08C9,
    /// Liteboxer Technologies Inc. (0x08C8)
    LiteboxerTechnologiesInc = 0x08C8,
    /// Monadnock Systems Ltd. (0x08C7)
    MonadnockSystemsLtd = 0x08C7,
    /// Integra Optics Inc (0x08C6)
    IntegraOpticsInc = 0x08C6,
    /// J. Wagner GmbH (0x08C5)
    JWagnerGmbH = 0x08C5,
    /// CellAssist, LLC (0x08C4)
    CellAssistLlc = 0x08C4,
    /// CHIPOLO d.o.o. (0x08C3)
    ChipoloDOO = 0x08C3,
    /// Lindinvent AB (0x08C2)
    LindinventAb = 0x08C2,
    /// Rayden.Earth LTD (0x08C1)
    RaydenEarthLtd = 0x08C1,
    /// Accent Advanced Systems SLU (0x08C0)
    AccentAdvancedSystemsSlu = 0x08C0,
    /// SIRC Co., Ltd. (0x08BF)
    SircCoLtd = 0x08BF,
    /// ubisys technologies GmbH (0x08BE)
    UbisysTechnologiesGmbH = 0x08BE,
    /// bf1systems limited (0x08BD)
    Bf1SystemsLimited = 0x08BD,
    /// Prevayl Limited (0x08BC)
    PrevaylLimited = 0x08BC,
    /// Tokai-rika co.,ltd. (0x08BB)
    TokaiRikaCoLtd = 0x08BB,
    /// HYPER ICE, INC. (0x08BA)
    HyperIceInc = 0x08BA,
    /// U-Shin Ltd. (0x08B9)
    UShinLtd = 0x08B9,
    /// Check Technology Solutions LLC (0x08B8)
    CheckTechnologySolutionsLlc = 0x08B8,
    /// ABB Inc (0x08B7)
    AbbInc = 0x08B7,
    /// Boehringer Ingelheim Vetmedica GmbH (0x08B6)
    BoehringerIngelheimVetmedicaGmbH = 0x08B6,
    /// TransferFi (0x08B5)
    TransferFi = 0x08B5,
    /// Sengled Co., Ltd. (0x08B4)
    SengledCoLtd = 0x08B4,
    /// IONIQ Skincare GmbH & Co. KG (0x08B3)
    IoniqSkincareGmbHAndCoKg = 0x08B3,
    /// PF SCHWEISSTECHNOLOGIE GMBH (0x08B2)
    PfSchweisstechnologieGmbh = 0x08B2,
    /// CORE|vision BV (0x08B1)
    CoreVisionBv = 0x08B1,
    /// Trivedi Advanced Technologies LLC (0x08B0)
    TrivediAdvancedTechnologiesLlc = 0x08B0,
    /// Polidea Sp. z o.o. (0x08AF)
    PolideaSpZOO = 0x08AF,
    /// Moticon ReGo AG (0x08AE)
    MoticonReGoAg = 0x08AE,
    /// Kayamatics Limited (0x08AD)
    KayamaticsLimited = 0x08AD,
    /// Topre Corporation (0x08AC)
    TopreCorporation = 0x08AC,
    /// Coburn Technology, LLC (0x08AB)
    CoburnTechnologyLlc = 0x08AB,
    /// SZ DJI TECHNOLOGY CO.,LTD (0x08AA)
    SzDjiTechnologyCoLtd = 0x08AA,
    /// Fraunhofer IIS (0x08A9)
    FraunhoferIis = 0x08A9,
    /// Shanghai Kfcube Inc (0x08A8)
    ShanghaiKfcubeInc = 0x08A8,
    /// TGR 1.618 Limited (0x08A7)
    Tgr1618Limited = 0x08A7,
    /// Intelligenceworks Inc. (0x08A6)
    IntelligenceworksInc = 0x08A6,
    /// UMEHEAL Ltd (0x08A5)
    UmehealLtd = 0x08A5,
    /// Realme Chongqing Mobile Telecommunications Corp., Ltd. (0x08A4)
    RealmeChongqingMobileTelecommunicationsCorpLtd = 0x08A4,
    /// Hoffmann SE (0x08A3)
    HoffmannSe = 0x08A3,
    /// Epic Systems Co., Ltd. (0x08A2)
    EpicSystemsCoLtd = 0x08A2,
    /// EXEO TECH CORPORATION (0x08A1)
    ExeoTechCorporation = 0x08A1,
    /// Aclara Technologies LLC (0x08A0)
    AclaraTechnologiesLlc = 0x08A0,
    /// Witschi Electronic Ltd (0x089F)
    WitschiElectronicLtd = 0x089F,
    /// i-SENS, inc. (0x089E)
    ISensInc = 0x089E,
    /// J-J.A.D.E. Enterprise LLC (0x089D)
    JJADEEnterpriseLlc = 0x089D,
    /// Embedded Devices Co. Company (0x089C)
    EmbeddedDevicesCoCompany = 0x089C,
    /// Saucon Technologies (0x089B)
    SauconTechnologies = 0x089B,
    /// Private limited company "Teltonika" (0x089A)
    PrivateLimitedCompanyTeltonika = 0x089A,
    /// SFS unimarket AG (0x0899)
    SfsUnimarketAg = 0x0899,
    /// Sensibo, Inc. (0x0898)
    SensiboInc = 0x0898,
    /// Current Lighting Solutions LLC (0x0897)
    CurrentLightingSolutionsLlc = 0x0897,
    /// Nokian Renkaat Oyj (0x0896)
    NokianRenkaatOyj = 0x0896,
    /// Gimer medical (0x0895)
    GimerMedical = 0x0895,
    /// EPIFIT (0x0894)
    Epifit = 0x0894,
    /// Maytronics Ltd (0x0893)
    MaytronicsLtd = 0x0893,
    /// Ingenieurbuero Birnfeld UG (haftungsbeschraenkt) (0x0892)
    IngenieurbueroBirnfeldUgHaftungsbeschraenkt = 0x0892,
    /// SmartWireless GmbH & Co. KG (0x0891)
    SmartWirelessGmbHAndCoKg = 0x0891,
    /// NICHIEI INTEC CO., LTD. (0x0890)
    NichieiIntecCoLtd = 0x0890,
    /// Tait International Limited (0x088F)
    TaitInternationalLimited = 0x088F,
    /// GIGA-TMS INC (0x088E)
    GigaTmsInc = 0x088E,
    /// Soliton Systems K.K. (0x088D)
    SolitonSystemsKK = 0x088D,
    /// GB Solution co.,Ltd (0x088C)
    GbSolutionCoLtd = 0x088C,
    /// Tricorder Arraay Technologies LLC (0x088B)
    TricorderArraayTechnologiesLlc = 0x088B,
    /// sclak s.r.l. (0x088A)
    SclakSRL = 0x088A,
    /// XANTHIO (0x0889)
    Xanthio = 0x0889,
    /// EnPointe Fencing Pty Ltd (0x0888)
    EnPointeFencingPtyLtd = 0x0888,
    /// Hydro-Gear Limited Partnership (0x0887)
    HydroGearLimitedPartnership = 0x0887,
    /// Movella Technologies B.V. (0x0886)
    MovellaTechnologiesBV = 0x0886,
    /// LEVOLOR INC (0x0885)
    LevolorInc = 0x0885,
    /// Controlid Industria, Comercio de Hardware e Servicos de Tecnologia Ltda (0x0884)
    ControlidIndustriaComercioDeHardwareEServicosDeTecnologiaLtda = 0x0884,
    /// Wintersteiger AG (0x0883)
    WintersteigerAg = 0x0883,
    /// PSYONIC, Inc. (0x0882)
    PsyonicInc = 0x0882,
    /// Optalert (0x0881)
    Optalert = 0x0881,
    /// imagiLabs AB (0x0880)
    ImagiLabsAb = 0x0880,
    /// Phillips Connect Technologies LLC (0x087F)
    PhillipsConnectTechnologiesLlc = 0x087F,
    /// 1bar.net Limited (0x087E)
    _1BarNetLimited = 0x087E,
    /// Konftel AB (0x087D)
    KonftelAb = 0x087D,
    /// Crosscan GmbH (0x087C)
    CrosscanGmbH = 0x087C,
    /// BYSTAMP (0x087B)
    Bystamp = 0x087B,
    /// ZRF, LLC (0x087A)
    ZrfLlc = 0x087A,
    /// MIZUNO Corporation (0x0879)
    MizunoCorporation = 0x0879,
    /// The Chamberlain Group, Inc. (0x0878)
    TheChamberlainGroupInc = 0x0878,
    /// Tome, Inc. (0x0877)
    TomeInc = 0x0877,
    /// SmartResQ ApS (0x0876)
    SmartResQApS = 0x0876,
    /// Berner International LLC (0x0875)
    BernerInternationalLlc = 0x0875,
    /// Treegreen Limited (0x0874)
    TreegreenLimited = 0x0874,
    /// Innophase Incorporated (0x0873)
    InnophaseIncorporated = 0x0873,
    /// 11 Health & Technologies Limited (0x0872)
    _11HealthAndTechnologiesLimited = 0x0872,
    /// Dension Elektronikai Kft. (0x0871)
    DensionElektronikaiKft = 0x0871,
    /// Wyze Labs, Inc (0x0870)
    WyzeLabsInc = 0x0870,
    /// Trackunit A/S (0x086F)
    TrackunitAS = 0x086F,
    /// Vorwerk Elektrowerke GmbH & Co. KG (0x086E)
    VorwerkElektrowerkeGmbHAndCoKg = 0x086E,
    /// Biometrika d.o.o. (0x086D)
    BiometrikaDOO = 0x086D,
    /// Revvo Technologies, Inc. (0x086C)
    RevvoTechnologiesInc = 0x086C,
    /// Pacific Track, LLC (0x086B)
    PacificTrackLlc = 0x086B,
    /// Odic Incorporated (0x086A)
    OdicIncorporated = 0x086A,
    /// EVVA Sicherheitstechnologie GmbH (0x0869)
    EvvaSicherheitstechnologieGmbH = 0x0869,
    /// WIOsense GmbH & Co. KG (0x0868)
    WiOsenseGmbHAndCoKg = 0x0868,
    /// Western Digital Techologies, Inc. (0x0867)
    WesternDigitalTechologiesInc = 0x0867,
    /// LAONZ Co.,Ltd (0x0866)
    LaonzCoLtd = 0x0866,
    /// Emergency Lighting Products Limited (0x0865)
    EmergencyLightingProductsLimited = 0x0865,
    /// Rafaelmicro (0x0864)
    Rafaelmicro = 0x0864,
    /// Yo-tronics Technology Co., Ltd. (0x0863)
    YoTronicsTechnologyCoLtd = 0x0863,
    /// SmartDrive (0x0862)
    SmartDrive = 0x0862,
    /// SmartSensor Labs Ltd (0x0861)
    SmartSensorLabsLtd = 0x0861,
    /// Alflex Products B.V. (0x0860)
    AlflexProductsBV = 0x0860,
    /// COMPEGPS TEAM,SOCIEDAD LIMITADA (0x085F)
    CompegpsTeamSociedadLimitada = 0x085F,
    /// Krog Systems LLC (0x085E)
    KrogSystemsLlc = 0x085E,
    /// Guilin Zhishen Information Technology Co.,Ltd. (0x085D)
    GuilinZhishenInformationTechnologyCoLtd = 0x085D,
    /// ACOS CO.,LTD. (0x085C)
    AcosCoLtd = 0x085C,
    /// Nisshinbo Micro Devices Inc. (0x085B)
    NisshinboMicroDevicesInc = 0x085B,
    /// DAKATECH (0x085A)
    Dakatech = 0x085A,
    /// BlueUp (0x0859)
    BlueUp = 0x0859,
    /// SOUNDBOKS (0x0858)
    Soundboks = 0x0858,
    /// Parsyl Inc (0x0857)
    ParsylInc = 0x0857,
    /// Canopy Growth Corporation (0x0856)
    CanopyGrowthCorporation0856 = 0x0856,
    /// Helios Sports, Inc. (0x0855)
    HeliosSportsInc = 0x0855,
    /// Tap Sound System (0x0854)
    TapSoundSystem = 0x0854,
    /// Pektron Group Limited (0x0853)
    PektronGroupLimited = 0x0853,
    /// Cognosos, Inc. (0x0852)
    CognososInc = 0x0852,
    /// Subeca, Inc. (0x0851)
    SubecaInc = 0x0851,
    /// Yealink (Xiamen) Network Technology Co.,LTD (0x0850)
    YealinkXiamenNetworkTechnologyCoLtd = 0x0850,
    /// Embedded Fitness B.V. (0x084F)
    EmbeddedFitnessBV = 0x084F,
    /// Carol Cole Company (0x084E)
    CarolColeCompany = 0x084E,
    /// SafePort (0x084D)
    SafePort = 0x084D,
    /// ORSO Inc. (0x084C)
    OrsoInc = 0x084C,
    /// Biotechware SRL (0x084B)
    BiotechwareSrl = 0x084B,
    /// ARCOM (0x084A)
    Arcom = 0x084A,
    /// Dopple Technologies B.V. (0x0849)
    DoppleTechnologiesBV = 0x0849,
    /// JUJU JOINTS CANADA CORP. (0x0848)
    JujuJointsCanadaCorp = 0x0848,
    /// DNANUDGE LIMITED (0x0847)
    DnanudgeLimited = 0x0847,
    /// USound GmbH (0x0846)
    USoundGmbH = 0x0846,
    /// Dometic Corporation (0x0845)
    DometicCorporation = 0x0845,
    /// Pepperl + Fuchs GmbH (0x0844)
    PepperlFuchsGmbH = 0x0844,
    /// FRAGRANCE DELIVERY TECHNOLOGIES LTD (0x0843)
    FragranceDeliveryTechnologiesLtd = 0x0843,
    /// Tangshan HongJia electronic technology co., LTD. (0x0842)
    TangshanHongJiaElectronicTechnologyCoLtd = 0x0842,
    /// General Luminaire (Shanghai) Co., Ltd. (0x0841)
    GeneralLuminaireShanghaiCoLtd = 0x0841,
    /// Down Range Systems LLC (0x0840)
    DownRangeSystemsLlc = 0x0840,
    /// D-Link Corp. (0x083F)
    DLinkCorp = 0x083F,
    /// Zorachka LTD (0x083E)
    ZorachkaLtd = 0x083E,
    /// Tokenize, Inc. (0x083D)
    TokenizeInc = 0x083D,
    /// BeerTech LTD (0x083C)
    BeerTechLtd = 0x083C,
    /// Piaggio Fast Forward (0x083B)
    PiaggioFastForward = 0x083B,
    /// BPW Bergische Achsen Kommanditgesellschaft (0x083A)
    BpwBergischeAchsenKommanditgesellschaft = 0x083A,
    /// A puissance 3 (0x0839)
    APuissance3 = 0x0839,
    /// Etymotic Research, Inc. (0x0838)
    EtymoticResearchInc = 0x0838,
    /// vivo Mobile Communication Co., Ltd. (0x0837)
    VivoMobileCommunicationCoLtd = 0x0837,
    /// Bitwards Oy (0x0836)
    BitwardsOy = 0x0836,
    /// Canopy Growth Corporation (0x0835)
    CanopyGrowthCorporation0835 = 0x0835,
    /// RIKEN KEIKI CO., LTD., (0x0834)
    RikenKeikiCoLtd = 0x0834,
    /// Conneqtech B.V. (0x0833)
    ConneqtechBV = 0x0833,
    /// Intermotive,Inc. (0x0832)
    IntermotiveInc = 0x0832,
    /// Foxble, LLC (0x0831)
    FoxbleLlc = 0x0831,
    /// Core Health and Fitness LLC (0x0830)
    CoreHealthAndFitnessLlc = 0x0830,
    /// Blippit AB (0x082F)
    BlippitAb = 0x082F,
    /// ABB S.p.A. (0x082E)
    AbbSPA = 0x082E,
    /// INCUS PERFORMANCE LTD. (0x082D)
    IncusPerformanceLtd = 0x082D,
    /// INGICS TECHNOLOGY CO., LTD. (0x082C)
    IngicsTechnologyCoLtd = 0x082C,
    /// shenzhen fitcare electronics Co.,Ltd (0x082B)
    ShenzhenFitcareElectronicsCoLtd = 0x082B,
    /// Mitutoyo Corporation (0x082A)
    MitutoyoCorporation = 0x082A,
    /// HEXAGON METROLOGY DIVISION ROMER (0x0829)
    HexagonMetrologyDivisionRomer = 0x0829,
    /// Shanghai Suisheng Information Technology Co., Ltd. (0x0828)
    ShanghaiSuishengInformationTechnologyCoLtd = 0x0828,
    /// Kickmaker (0x0827)
    Kickmaker = 0x0827,
    /// Hyundai Motor Company (0x0826)
    HyundaiMotorCompany = 0x0826,
    /// CME PTE. LTD. (0x0825)
    CmePteLtd = 0x0825,
    /// 8Power Limited (0x0824)
    _8PowerLimited = 0x0824,
    /// Nexite Ltd (0x0823)
    NexiteLtd = 0x0823,
    /// adafruit industries (0x0822)
    AdafruitIndustries = 0x0822,
    /// INOVA Geophysical, Inc. (0x0821)
    InovaGeophysicalInc = 0x0821,
    /// Brilliant Home Technology, Inc. (0x0820)
    BrilliantHomeTechnologyInc = 0x0820,
    /// eSenseLab LTD (0x081F)
    ESenseLabLtd = 0x081F,
    /// iNFORM Technology GmbH (0x081E)
    INformTechnologyGmbH = 0x081E,
    /// Potrykus Holdings and Development LLC (0x081D)
    PotrykusHoldingsAndDevelopmentLlc = 0x081D,
    /// Bobrick Washroom Equipment, Inc. (0x081C)
    BobrickWashroomEquipmentInc = 0x081C,
    /// DIM3 (0x081B)
    Dim3 = 0x081B,
    /// Shenzhen Conex (0x081A)
    ShenzhenConex = 0x081A,
    /// Hunter Douglas Inc (0x0819)
    HunterDouglasInc = 0x0819,
    /// tatwah SA (0x0818)
    TatwahSa = 0x0818,
    /// Wangs Alliance Corporation (0x0817)
    WangsAllianceCorporation = 0x0817,
    /// SPICA SYSTEMS LLC (0x0816)
    SpicaSystemsLlc = 0x0816,
    /// SKC Inc (0x0815)
    SkcInc = 0x0815,
    /// Ossur hf. (0x0814)
    OssurHf = 0x0814,
    /// Flextronics International USA Inc. (0x0813)
    FlextronicsInternationalUsaInc = 0x0813,
    /// Mstream Technologies., Inc. (0x0812)
    MstreamTechnologiesInc = 0x0812,
    /// Becker Antriebe GmbH (0x0811)
    BeckerAntriebeGmbH = 0x0811,
    /// LECO Corporation (0x0810)
    LecoCorporation = 0x0810,
    /// Paradox Engineering SA (0x080F)
    ParadoxEngineeringSa = 0x080F,
    /// TATTCOM LLC (0x080E)
    TattcomLlc = 0x080E,
    /// Azbil Co. (0x080D)
    AzbilCo = 0x080D,
    /// Ingy B.V. (0x080C)
    IngyBV = 0x080C,
    /// Nanoleaf Canada Limited (0x080B)
    NanoleafCanadaLimited = 0x080B,
    /// Altaneos (0x080A)
    Altaneos = 0x080A,
    /// Trulli Audio (0x0809)
    TrulliAudio = 0x0809,
    /// SISTEMAS KERN, SOCIEDAD ANÓMINA (0x0808)
    SistemasKernSociedadAnómina = 0x0808,
    /// ECD Electronic Components GmbH Dresden (0x0807)
    EcdElectronicComponentsGmbHDresden = 0x0807,
    /// TYRI Sweden AB (0x0806)
    TyriSwedenAb = 0x0806,
    /// Urbanminded Ltd (0x0805)
    UrbanmindedLtd = 0x0805,
    /// Andon Health Co.,Ltd (0x0804)
    AndonHealthCoLtd = 0x0804,
    /// Domintell s.a. (0x0803)
    DomintellSA = 0x0803,
    /// NantSound, Inc. (0x0802)
    NantSoundInc = 0x0802,
    /// CRONUS ELECTRONICS LTD (0x0801)
    CronusElectronicsLtd = 0x0801,
    /// Optek (0x0800)
    Optek = 0x0800,
    /// maxon motor ltd. (0x07FF)
    MaxonMotorLtd = 0x07FF,
    /// BIROTA (0x07FE)
    Birota = 0x07FE,
    /// JSK CO., LTD. (0x07FD)
    JskCoLtd = 0x07FD,
    /// Renault SA (0x07FC)
    RenaultSa = 0x07FC,
    /// Access Co., Ltd (0x07FB)
    AccessCoLtd = 0x07FB,
    /// Klipsch Group, Inc. (0x07FA)
    KlipschGroupInc = 0x07FA,
    /// Direct Communication Solutions, Inc. (0x07F9)
    DirectCommunicationSolutionsInc = 0x07F9,
    /// quip NYC Inc. (0x07F8)
    QuipNycInc = 0x07F8,
    /// Cesar Systems Ltd. (0x07F7)
    CesarSystemsLtd = 0x07F7,
    /// Shenzhen TonliScience and Technology Development Co.,Ltd (0x07F6)
    ShenzhenTonliScienceAndTechnologyDevelopmentCoLtd = 0x07F6,
    /// Byton North America Corporation (0x07F5)
    BytonNorthAmericaCorporation = 0x07F5,
    /// MEDIRLAB Orvosbiologiai Fejleszto Korlatolt Felelossegu Tarsasag (0x07F4)
    MedirlabOrvosbiologiaiFejlesztoKorlatoltFelelosseguTarsasag = 0x07F4,
    /// DIGISINE ENERGYTECH CO. LTD. (0x07F3)
    DigisineEnergytechCoLtd = 0x07F3,
    /// SERENE GROUP, INC (0x07F2)
    SereneGroupInc = 0x07F2,
    /// Zimi Innovations Pty Ltd (0x07F1)
    ZimiInnovationsPtyLtd = 0x07F1,
    /// e-moola.com Pty Ltd (0x07F0)
    EMoolaComPtyLtd = 0x07F0,
    /// Aktiebolaget Sandvik Coromant (0x07EF)
    AktiebolagetSandvikCoromant = 0x07EF,
    /// KidzTek LLC (0x07EE)
    KidzTekLlc = 0x07EE,
    /// Joule IQ, INC. (0x07ED)
    JouleIqInc = 0x07ED,
    /// Frecce LLC (0x07EC)
    FrecceLlc = 0x07EC,
    /// NOVABASE S.R.L. (0x07EB)
    NovabaseSRL = 0x07EB,
    /// ShapeLog, Inc. (0x07EA)
    ShapeLogInc = 0x07EA,
    /// Häfele GmbH & Co KG (0x07E9)
    HäfeleGmbHAndCoKg = 0x07E9,
    /// Packetcraft, Inc. (0x07E8)
    PacketcraftInc = 0x07E8,
    /// Komfort IQ, Inc. (0x07E7)
    KomfortIqInc = 0x07E7,
    /// Waybeyond Limited (0x07E6)
    WaybeyondLimited = 0x07E6,
    /// Minut, Inc. (0x07E5)
    MinutInc = 0x07E5,
    /// Geeksme S.L. (0x07E4)
    GeeksmeSL = 0x07E4,
    /// Airoha Technology Corp. (0x07E3)
    AirohaTechnologyCorp07E3 = 0x07E3,
    /// Alfred Kaercher SE & Co. KG (0x07E2)
    AlfredKaercherSeAndCoKg = 0x07E2,
    /// Lucie Labs (0x07E1)
    LucieLabs = 0x07E1,
    /// Edifier International Limited (0x07E0)
    EdifierInternationalLimited = 0x07E0,
    /// Snap-on Incorporated (0x07DF)
    SnapOnIncorporated = 0x07DF,
    /// Unlimited Engineering SL (0x07DE)
    UnlimitedEngineeringSl = 0x07DE,
    /// Linear Circuits (0x07DD)
    LinearCircuits = 0x07DD,
    /// ThingOS GmbH & Co KG (0x07DC)
    ThingOsGmbHAndCoKg = 0x07DC,
    /// Remedee Labs (0x07DB)
    RemedeeLabs = 0x07DB,
    /// STARLITE Co., Ltd. (0x07DA)
    StarliteCoLtd = 0x07DA,
    /// Micro-Design, Inc. (0x07D9)
    MicroDesignInc = 0x07D9,
    /// SOLUTIONS AMBRA INC. (0x07D8)
    SolutionsAmbraInc = 0x07D8,
    /// Nanjing Qinheng Microelectronics Co., Ltd (0x07D7)
    NanjingQinhengMicroelectronicsCoLtd = 0x07D7,
    /// ecobee Inc. (0x07D6)
    EcobeeInc = 0x07D6,
    /// hoots classic GmbH (0x07D5)
    HootsClassicGmbH = 0x07D5,
    /// Kano Computing Limited (0x07D4)
    KanoComputingLimited = 0x07D4,
    /// LIVNEX Co.,Ltd. (0x07D3)
    LivnexCoLtd = 0x07D3,
    /// React Accessibility Limited (0x07D2)
    ReactAccessibilityLimited = 0x07D2,
    /// Shanghai Panchip Microelectronics Co., Ltd (0x07D1)
    ShanghaiPanchipMicroelectronicsCoLtd = 0x07D1,
    /// Hangzhou Tuya Information  Technology Co., Ltd (0x07D0)
    HangzhouTuyaInformationTechnologyCoLtd = 0x07D0,
    /// NeoSensory, Inc. (0x07CF)
    NeoSensoryInc = 0x07CF,
    /// Shanghai Top-Chip Microelectronics Tech. Co., LTD (0x07CE)
    ShanghaiTopChipMicroelectronicsTechCoLtd = 0x07CE,
    /// Smart Wave Technologies Canada Inc (0x07CD)
    SmartWaveTechnologiesCanadaInc = 0x07CD,
    /// Barnacle Systems Inc. (0x07CC)
    BarnacleSystemsInc = 0x07CC,
    /// West Pharmaceutical Services, Inc. (0x07CB)
    WestPharmaceuticalServicesInc = 0x07CB,
    /// Modul-System HH AB (0x07CA)
    ModulSystemHhAb = 0x07CA,
    /// Skullcandy, Inc. (0x07C9)
    SkullcandyInc = 0x07C9,
    /// WRLDS Creations AB (0x07C8)
    WrldsCreationsAb = 0x07C8,
    /// iaconicDesign Inc. (0x07C7)
    IaconicDesignInc = 0x07C7,
    /// Bluenetics GmbH (0x07C6)
    BlueneticsGmbH = 0x07C6,
    /// June Life, Inc. (0x07C5)
    JuneLifeInc = 0x07C5,
    /// Johnson Health Tech NA (0x07C4)
    JohnsonHealthTechNa = 0x07C4,
    /// CIMTechniques, Inc. (0x07C3)
    CimTechniquesInc = 0x07C3,
    /// Radinn AB (0x07C2)
    RadinnAb = 0x07C2,
    /// A.W. Chesterton Company (0x07C1)
    AWChestertonCompany = 0x07C1,
    /// Biral AG (0x07C0)
    BiralAg = 0x07C0,
    /// REGULA Ltd. (0x07BF)
    RegulaLtd = 0x07BF,
    /// Axentia Technologies AB (0x07BE)
    AxentiaTechnologiesAb = 0x07BE,
    /// Genedrive Diagnostics Ltd (0x07BD)
    GenedriveDiagnosticsLtd = 0x07BD,
    /// KD CIRCUITS LLC (0x07BC)
    KdCircuitsLlc = 0x07BC,
    /// EPIC S.R.L. (0x07BB)
    EpicSRL = 0x07BB,
    /// Battery-Biz Inc. (0x07BA)
    BatteryBizInc = 0x07BA,
    /// Epona Biotec Limited (0x07B9)
    EponaBiotecLimited = 0x07B9,
    /// iSwip (0x07B8)
    ISwip = 0x07B8,
    /// ETABLISSEMENTS GEORGES RENAULT (0x07B7)
    EtablissementsGeorgesRenault = 0x07B7,
    /// Soundbrenner Limited (0x07B6)
    SoundbrennerLimited = 0x07B6,
    /// CRONO CHIP, S.L. (0x07B5)
    CronoChipSL = 0x07B5,
    /// Hormann KG Antriebstechnik (0x07B4)
    HormannKgAntriebstechnik = 0x07B4,
    /// 2N TELEKOMUNIKACE a.s. (0x07B3)
    _2NTelekomunikaceAS = 0x07B3,
    /// Moeco IOT Inc. (0x07B2)
    MoecoIotInc = 0x07B2,
    /// Thomas Dynamics, LLC (0x07B1)
    ThomasDynamicsLlc = 0x07B1,
    /// GV Concepts Inc. (0x07B0)
    GvConceptsInc = 0x07B0,
    /// Hong Kong Bouffalo Lab Limited (0x07AF)
    HongKongBouffaloLabLimited = 0x07AF,
    /// Aurea Solucoes Tecnologicas Ltda. (0x07AE)
    AureaSolucoesTecnologicasLtda = 0x07AE,
    /// New H3C Technologies Co.,Ltd (0x07AD)
    NewH3CTechnologiesCoLtd = 0x07AD,
    /// LoupeDeck Oy (0x07AC)
    LoupeDeckOy = 0x07AC,
    /// Granite River Solutions, Inc. (0x07AB)
    GraniteRiverSolutionsInc = 0x07AB,
    /// The Kroger Co. (0x07AA)
    TheKrogerCo = 0x07AA,
    /// Bruel & Kjaer Sound & Vibration (0x07A9)
    BruelAndKjaerSoundAndVibration = 0x07A9,
    /// conbee GmbH (0x07A8)
    ConbeeGmbH = 0x07A8,
    /// Zume, Inc. (0x07A7)
    ZumeInc = 0x07A7,
    /// Musen Connect, Inc. (0x07A6)
    MusenConnectInc = 0x07A6,
    /// RAB Lighting, Inc. (0x07A5)
    RabLightingInc = 0x07A5,
    /// Xiamen Mage Information Technology Co., Ltd. (0x07A4)
    XiamenMageInformationTechnologyCoLtd = 0x07A4,
    /// Comcast Cable (0x07A3)
    ComcastCable = 0x07A3,
    /// Roku, Inc. (0x07A2)
    RokuInc = 0x07A2,
    /// Apollo Neuroscience, Inc. (0x07A1)
    ApolloNeuroscienceInc = 0x07A1,
    /// Regent Beleuchtungskorper AG (0x07A0)
    RegentBeleuchtungskorperAg = 0x07A0,
    /// Pune Scientific LLP (0x079F)
    PuneScientificLlp = 0x079F,
    /// Smartloxx GmbH (0x079E)
    SmartloxxGmbH = 0x079E,
    /// Digibale Pty Ltd (0x079D)
    DigibalePtyLtd = 0x079D,
    /// Sky UK Limited (0x079C)
    SkyUkLimited = 0x079C,
    /// CST ELECTRONICS (PROPRIETARY) LIMITED (0x079B)
    CstElectronicsProprietaryLimited = 0x079B,
    /// GuangDong Oppo Mobile Telecommunications Corp., Ltd. (0x079A)
    GuangDongOppoMobileTelecommunicationsCorpLtd = 0x079A,
    /// PlantChoir Inc. (0x0799)
    PlantChoirInc = 0x0799,
    /// HoloKit, Inc. (0x0798)
    HoloKitInc = 0x0798,
    /// Water-i.d. GmbH (0x0797)
    WaterIDGmbH = 0x0797,
    /// StarLeaf Ltd (0x0796)
    StarLeafLtd = 0x0796,
    /// GASTEC CORPORATION (0x0795)
    GastecCorporation = 0x0795,
    /// The Coca-Cola Company (0x0794)
    TheCocaColaCompany = 0x0794,
    /// AEV spol. s r.o. (0x0793)
    AevSpolSRO = 0x0793,
    /// Cricut, Inc. (0x0792)
    CricutInc = 0x0792,
    /// Scosche Industries, Inc. (0x0791)
    ScoscheIndustriesInc = 0x0791,
    /// KOMPAN A/S (0x0790)
    KompanAS = 0x0790,
    /// Hanna Instruments, Inc. (0x078F)
    HannaInstrumentsInc = 0x078F,
    /// FUJIMIC NIIGATA, INC. (0x078E)
    FujimicNiigataInc = 0x078E,
    /// Cybex GmbH (0x078D)
    CybexGmbH = 0x078D,
    /// MINIBREW HOLDING B.V (0x078C)
    MinibrewHoldingBV = 0x078C,
    /// Optikam Tech Inc. (0x078B)
    OptikamTechInc = 0x078B,
    /// The Wildflower Foundation (0x078A)
    TheWildflowerFoundation = 0x078A,
    /// PCB Piezotronics, Inc. (0x0789)
    PcbPiezotronicsInc = 0x0789,
    /// BubblyNet, LLC (0x0788)
    BubblyNetLlc = 0x0788,
    /// Pangaea Solution (0x0787)
    PangaeaSolution = 0x0787,
    /// HLP Controls Pty Limited (0x0786)
    HlpControlsPtyLimited = 0x0786,
    /// O2 Micro, Inc. (0x0785)
    O2MicroInc = 0x0785,
    /// audifon GmbH & Co. KG (0x0784)
    AudifonGmbHAndCoKg = 0x0784,
    /// ESEMBER LIMITED LIABILITY COMPANY (0x0783)
    EsemberLimitedLiabilityCompany = 0x0783,
    /// DeviceDrive AS (0x0782)
    DeviceDriveAs = 0x0782,
    /// Qingping Technology (Beijing) Co., Ltd. (0x0781)
    QingpingTechnologyBeijingCoLtd = 0x0781,
    /// Finch Technologies Ltd. (0x0780)
    FinchTechnologiesLtd = 0x0780,
    /// Glenview Software Corporation (0x077F)
    GlenviewSoftwareCorporation = 0x077F,
    /// Sparkage Inc. (0x077E)
    SparkageInc = 0x077E,
    /// Sensority, s.r.o. (0x077D)
    SensoritySRO = 0x077D,
    /// radius co., ltd. (0x077C)
    RadiusCoLtd = 0x077C,
    /// AmaterZ, Inc. (0x077B)
    AmaterZInc = 0x077B,
    /// Niruha Systems Private Limited (0x077A)
    NiruhaSystemsPrivateLimited = 0x077A,
    /// Loopshore Oy (0x0779)
    LoopshoreOy = 0x0779,
    /// KOAMTAC INC. (0x0778)
    KoamtacInc = 0x0778,
    /// Cue (0x0777)
    Cue = 0x0777,
    /// Cyber Transport Control GmbH (0x0776)
    CyberTransportControlGmbH = 0x0776,
    /// 4eBusiness GmbH (0x0775)
    _4EBusinessGmbH = 0x0775,
    /// C-MAX Asia Limited (0x0774)
    CMaxAsiaLimited = 0x0774,
    /// Echoflex Solutions Inc. (0x0773)
    EchoflexSolutionsInc = 0x0773,
    /// Thirdwayv Inc. (0x0772)
    ThirdwayvInc = 0x0772,
    /// Corvex Connected Safety (0x0771)
    CorvexConnectedSafety = 0x0771,
    /// InnoCon Medical ApS (0x0770)
    InnoConMedicalApS = 0x0770,
    /// Successful Endeavours Pty Ltd (0x076F)
    SuccessfulEndeavoursPtyLtd = 0x076F,
    /// WuQi technologies, Inc. (0x076E)
    WuQiTechnologiesInc = 0x076E,
    /// Graesslin GmbH (0x076D)
    GraesslinGmbH = 0x076D,
    /// Noodle Technology inc (0x076C)
    NoodleTechnologyInc = 0x076C,
    /// Engineered Medical Technologies (0x076B)
    EngineeredMedicalTechnologies = 0x076B,
    /// Dmac Mobile Developments, LLC (0x076A)
    DmacMobileDevelopmentsLlc = 0x076A,
    /// Force Impact Technologies (0x0769)
    ForceImpactTechnologies = 0x0769,
    /// Peloton Interactive Inc. (0x0768)
    PelotonInteractiveInc = 0x0768,
    /// NITTO DENKO ASIA TECHNICAL CENTRE PTE. LTD. (0x0767)
    NittoDenkoAsiaTechnicalCentrePteLtd = 0x0767,
    /// ART AND PROGRAM, INC. (0x0766)
    ArtAndProgramInc = 0x0766,
    /// Voxx International (0x0765)
    VoxxInternational = 0x0765,
    /// WWZN Information Technology Company Limited (0x0764)
    WwznInformationTechnologyCompanyLimited = 0x0764,
    /// PIKOLIN S.L. (0x0763)
    PikolinSL = 0x0763,
    /// TerOpta Ltd (0x0762)
    TerOptaLtd = 0x0762,
    /// Mantis Tech LLC (0x0761)
    MantisTechLlc = 0x0761,
    /// Vimar SpA (0x0760)
    VimarSpA = 0x0760,
    /// Remote Solution Co., LTD. (0x075F)
    RemoteSolutionCoLtd = 0x075F,
    /// Katerra Inc. (0x075E)
    KaterraInc = 0x075E,
    /// RHOMBUS SYSTEMS, INC. (0x075D)
    RhombusSystemsInc = 0x075D,
    /// Antitronics Inc. (0x075C)
    AntitronicsInc = 0x075C,
    /// Smart Sensor Devices AB (0x075B)
    SmartSensorDevicesAb = 0x075B,
    /// HARMAN CO.,LTD. (0x075A)
    HarmanCoLtd = 0x075A,
    /// Shanghai InGeek Cyber Security Co., Ltd. (0x0759)
    ShanghaiInGeekCyberSecurityCoLtd = 0x0759,
    /// umanSense AB (0x0758)
    UmanSenseAb = 0x0758,
    /// ELA Innovation (0x0757)
    ElaInnovation = 0x0757,
    /// Lumens For Less, Inc (0x0756)
    LumensForLessInc = 0x0756,
    /// Brother Industries, Ltd (0x0755)
    BrotherIndustriesLtd = 0x0755,
    /// Michael Parkin (0x0754)
    MichaelParkin = 0x0754,
    /// JLG Industries, Inc. (0x0753)
    JlgIndustriesInc = 0x0753,
    /// Elatec GmbH (0x0752)
    ElatecGmbH = 0x0752,
    /// Changsha JEMO IC Design Co.,Ltd (0x0751)
    ChangshaJemoIcDesignCoLtd = 0x0751,
    /// Hamilton Professional Services of Canada Incorporated (0x0750)
    HamiltonProfessionalServicesOfCanadaIncorporated = 0x0750,
    /// MEDIATECH S.R.L. (0x074F)
    MediatechSRL = 0x074F,
    /// EAGLE DETECTION SA (0x074E)
    EagleDetectionSa = 0x074E,
    /// Amtech Systems, LLC (0x074D)
    AmtechSystemsLlc = 0x074D,
    /// iopool s.a. (0x074C)
    IopoolSA = 0x074C,
    /// Sarvavid Software Solutions LLP (0x074B)
    SarvavidSoftwareSolutionsLlp = 0x074B,
    /// Illusory Studios LLC (0x074A)
    IllusoryStudiosLlc = 0x074A,
    /// DIAODIAO (Beijing) Technology Co., Ltd. (0x0749)
    DiaodiaoBeijingTechnologyCoLtd = 0x0749,
    /// GuangZhou KuGou Computer Technology Co.Ltd (0x0748)
    GuangZhouKuGouComputerTechnologyCoLtd = 0x0748,
    /// OR Technologies Pty Ltd (0x0747)
    OrTechnologiesPtyLtd = 0x0747,
    /// Seitec Elektronik GmbH (0x0746)
    SeitecElektronikGmbH = 0x0746,
    /// WIZNOVA, Inc. (0x0745)
    WiznovaInc = 0x0745,
    /// SOCOMEC (0x0744)
    Socomec = 0x0744,
    /// Sanofi (0x0743)
    Sanofi = 0x0743,
    /// DML LLC (0x0742)
    DmlLlc = 0x0742,
    /// MAC SRL (0x0741)
    MacSrl = 0x0741,
    /// HITIQ LIMITED (0x0740)
    HitiqLimited = 0x0740,
    /// Beijing Unisoc Technologies Co., Ltd. (0x073F)
    BeijingUnisocTechnologiesCoLtd = 0x073F,
    /// Bluepack S.R.L. (0x073E)
    BluepackSRL = 0x073E,
    /// Beijing Hao Heng Tian Tech Co., Ltd. (0x073D)
    BeijingHaoHengTianTechCoLtd = 0x073D,
    /// Acubit ApS (0x073C)
    AcubitApS = 0x073C,
    /// Fantini Cosmi s.p.a. (0x073B)
    FantiniCosmiSPA = 0x073B,
    /// Chandler Systems Inc. (0x073A)
    ChandlerSystemsInc = 0x073A,
    /// Jiangsu Qinheng Co., Ltd. (0x0739)
    JiangsuQinhengCoLtd = 0x0739,
    /// Glass Security Pte Ltd (0x0738)
    GlassSecurityPteLtd = 0x0738,
    /// LLC Navitek (0x0737)
    LlcNavitek = 0x0737,
    /// Luna XIO, Inc. (0x0736)
    LunaXioInc = 0x0736,
    /// UpRight Technologies LTD (0x0735)
    UpRightTechnologiesLtd = 0x0735,
    /// DiUS Computing Pty Ltd (0x0734)
    DiUsComputingPtyLtd = 0x0734,
    /// Iguanavation, Inc. (0x0733)
    IguanavationInc = 0x0733,
    /// Dairy Tech, LLC (0x0732)
    DairyTechLlc = 0x0732,
    /// ABLIC Inc. (0x0731)
    AblicInc = 0x0731,
    /// Wildlife Acoustics, Inc. (0x0730)
    WildlifeAcousticsInc = 0x0730,
    /// OnePlus Electronics (Shenzhen) Co., Ltd. (0x072F)
    OnePlusElectronicsShenzhenCoLtd = 0x072F,
    /// Open Platform Systems LLC (0x072E)
    OpenPlatformSystemsLlc = 0x072E,
    /// Safera Oy (0x072D)
    SaferaOy = 0x072D,
    /// GWA Hygiene GmbH (0x072C)
    GwaHygieneGmbH = 0x072C,
    /// Bitkey Inc. (0x072B)
    BitkeyInc = 0x072B,
    /// JMR embedded systems GmbH (0x072A)
    JmrEmbeddedSystemsGmbH = 0x072A,
    /// SwaraLink Technologies (0x0729)
    SwaraLinkTechnologies = 0x0729,
    /// Eli Lilly and Company (0x0728)
    EliLillyAndCompany = 0x0728,
    /// STALKIT AS (0x0727)
    StalkitAs = 0x0727,
    /// PHC Corporation (0x0726)
    PhcCorporation = 0x0726,
    /// Tedee Sp. z o.o. (0x0725)
    TedeeSpZOO = 0x0725,
    /// Guangzhou SuperSound Information Technology Co.,Ltd (0x0724)
    GuangzhouSuperSoundInformationTechnologyCoLtd = 0x0724,
    /// Ford Motor Company (0x0723)
    FordMotorCompany = 0x0723,
    /// Xiamen Eholder Electronics Co.Ltd (0x0722)
    XiamenEholderElectronicsCoLtd = 0x0722,
    /// Clover Network, Inc. (0x0721)
    CloverNetworkInc = 0x0721,
    /// Oculeve, Inc. (0x0720)
    OculeveInc = 0x0720,
    /// Dongguan Liesheng Electronic Co.Ltd (0x071F)
    DongguanLieshengElectronicCoLtd = 0x071F,
    /// DONGGUAN HELE ELECTRONICS CO., LTD (0x071E)
    DongguanHeleElectronicsCoLtd = 0x071E,
    /// exoTIC Systems (0x071D)
    ExoTicSystems = 0x071D,
    /// F5 Sports, Inc (0x071C)
    F5SportsInc = 0x071C,
    /// Precor (0x071B)
    Precor = 0x071B,
    /// REVSMART WEARABLE HK CO LTD (0x071A)
    RevsmartWearableHkCoLtd = 0x071A,
    /// COREIOT PTY LTD (0x0719)
    CoreiotPtyLtd = 0x0719,
    /// IDIBAIX enginneering (0x0718)
    IdibaixEnginneering = 0x0718,
    /// iQsquare BV (0x0717)
    IQsquareBv = 0x0717,
    /// Altonics (0x0716)
    Altonics = 0x0716,
    /// MBARC LABS Inc (0x0715)
    MbarcLabsInc = 0x0715,
    /// MindPeace Safety LLC (0x0714)
    MindPeaceSafetyLlc = 0x0714,
    /// Respiri Limited (0x0713)
    RespiriLimited = 0x0713,
    /// Bull Group Company Limited (0x0712)
    BullGroupCompanyLimited = 0x0712,
    /// ABAX AS (0x0711)
    AbaxAs = 0x0711,
    /// Audiodo AB (0x0710)
    AudiodoAb = 0x0710,
    /// California Things Inc. (0x070F)
    CaliforniaThingsInc = 0x070F,
    /// FiveCo Sarl (0x070E)
    FiveCoSarl = 0x070E,
    /// SmartSnugg Pty Ltd (0x070D)
    SmartSnuggPtyLtd = 0x070D,
    /// Beijing Winner Microelectronics Co.,Ltd (0x070C)
    BeijingWinnerMicroelectronicsCoLtd = 0x070C,
    /// Element Products, Inc. (0x070B)
    ElementProductsInc = 0x070B,
    /// Huf Hülsbeck & Fürst GmbH & Co. KG (0x070A)
    HufHülsbeckAndFürstGmbHAndCoKg = 0x070A,
    /// Carewear Corp. (0x0709)
    CarewearCorp = 0x0709,
    /// Be Interactive Co., Ltd (0x0708)
    BeInteractiveCoLtd = 0x0708,
    /// Essity Hygiene and Health Aktiebolag (0x0707)
    EssityHygieneAndHealthAktiebolag = 0x0707,
    /// Wernher von Braun Center for ASdvanced Research (0x0706)
    WernherVonBraunCenterForASdvancedResearch = 0x0706,
    /// AB Electrolux (0x0705)
    AbElectrolux = 0x0705,
    /// JBX Designs Inc. (0x0704)
    JbxDesignsInc = 0x0704,
    /// Beijing Jingdong Century Trading Co., Ltd. (0x0703)
    BeijingJingdongCenturyTradingCoLtd = 0x0703,
    /// Akciju sabiedriba "SAF TEHNIKA" (0x0702)
    AkcijuSabiedribaSafTehnika = 0x0702,
    /// PAFERS TECH (0x0701)
    PafersTech = 0x0701,
    /// TraqFreq LLC (0x0700)
    TraqFreqLlc = 0x0700,
    /// Redpine Signals Inc (0x06FF)
    RedpineSignalsInc = 0x06FF,
    /// Mahr GmbH (0x06FE)
    MahrGmbH = 0x06FE,
    /// ESS Embedded System Solutions Inc. (0x06FD)
    EssEmbeddedSystemSolutionsInc = 0x06FD,
    /// Tom Communication Industrial Co.,Ltd. (0x06FC)
    TomCommunicationIndustrialCoLtd = 0x06FC,
    /// Sartorius AG (0x06FB)
    SartoriusAg = 0x06FB,
    /// Enequi AB (0x06FA)
    EnequiAb = 0x06FA,
    /// happybrush GmbH (0x06F9)
    HappybrushGmbH = 0x06F9,
    /// BodyPlus Technology Co.,Ltd (0x06F8)
    BodyPlusTechnologyCoLtd = 0x06F8,
    /// WILKA Schliesstechnik GmbH (0x06F7)
    WilkaSchliesstechnikGmbH = 0x06F7,
    /// Vitulo Plus BV (0x06F6)
    VituloPlusBv = 0x06F6,
    /// Vigil Technologies Inc. (0x06F5)
    VigilTechnologiesInc = 0x06F5,
    /// Touché Technology Ltd (0x06F4)
    TouchéTechnologyLtd = 0x06F4,
    /// Alfred International Inc. (0x06F3)
    AlfredInternationalInc = 0x06F3,
    /// Trapper Data AB (0x06F2)
    TrapperDataAb = 0x06F2,
    /// Shibutani Co., Ltd. (0x06F1)
    ShibutaniCoLtd = 0x06F1,
    /// Chargy Technologies, SL (0x06F0)
    ChargyTechnologiesSl = 0x06F0,
    /// ALCARE Co., Ltd. (0x06EF)
    AlcareCoLtd = 0x06EF,
    /// Avantis Systems Limited (0x06EE)
    AvantisSystemsLimited = 0x06EE,
    /// J Neades Ltd (0x06ED)
    JNeadesLtd = 0x06ED,
    /// Sigur (0x06EC)
    Sigur = 0x06EC,
    /// Houston Radar LLC (0x06EB)
    HoustonRadarLlc = 0x06EB,
    /// SafeLine Sweden AB (0x06EA)
    SafeLineSwedenAb = 0x06EA,
    /// Zmartfun Electronics, Inc. (0x06E9)
    ZmartfunElectronicsInc = 0x06E9,
    /// Almendo Technologies GmbH (0x06E8)
    AlmendoTechnologiesGmbH = 0x06E8,
    /// VELUX A/S (0x06E7)
    VeluxAS = 0x06E7,
    /// NIHON DENGYO KOUSAKU (0x06E6)
    NihonDengyoKousaku = 0x06E6,
    /// OPTEX CO.,LTD. (0x06E5)
    OptexCoLtd = 0x06E5,
    /// Aluna (0x06E4)
    Aluna = 0x06E4,
    /// Spinlock Ltd (0x06E3)
    SpinlockLtd = 0x06E3,
    /// Alango Technologies Ltd (0x06E2)
    AlangoTechnologiesLtd = 0x06E2,
    /// Milestone AV Technologies LLC (0x06E1)
    MilestoneAvTechnologiesLlc = 0x06E1,
    /// Avaya Inc. (0x06E0)
    AvayaInc = 0x06E0,
    /// HLI Solutions Inc. (0x06DF)
    HliSolutionsInc = 0x06DF,
    /// Navcast, Inc. (0x06DE)
    NavcastInc = 0x06DE,
    /// Intellithings Ltd. (0x06DD)
    IntellithingsLtd = 0x06DD,
    /// Industrial Network Controls, LLC (0x06DC)
    IndustrialNetworkControlsLlc = 0x06DC,
    /// Automatic Labs, Inc. (0x06DB)
    AutomaticLabsInc = 0x06DB,
    /// Zliide Technologies ApS (0x06DA)
    ZliideTechnologiesApS = 0x06DA,
    /// Shanghai Mountain View Silicon Co.,Ltd. (0x06D9)
    ShanghaiMountainViewSiliconCoLtd = 0x06D9,
    /// AW Company (0x06D8)
    AwCompany = 0x06D8,
    /// FUBA Automotive Electronics GmbH (0x06D7)
    FubaAutomotiveElectronicsGmbH = 0x06D7,
    /// JCT Healthcare Pty Ltd (0x06D6)
    JctHealthcarePtyLtd = 0x06D6,
    /// Sensirion AG (0x06D5)
    SensirionAg = 0x06D5,
    /// DYNAKODE TECHNOLOGY PRIVATE LIMITED (0x06D4)
    DynakodeTechnologyPrivateLimited = 0x06D4,
    /// TriTeq Lock and Security, LLC (0x06D3)
    TriTeqLockAndSecurityLlc = 0x06D3,
    /// CeoTronics AG (0x06D2)
    CeoTronicsAg = 0x06D2,
    /// Meyer Sound Laboratories, Incorporated (0x06D1)
    MeyerSoundLaboratoriesIncorporated = 0x06D1,
    /// Etekcity Corporation (0x06D0)
    EtekcityCorporation = 0x06D0,
    /// Belparts N.V. (0x06CF)
    BelpartsNV = 0x06CF,
    /// FIOR & GENTZ (0x06CE)
    FiorAndGentz = 0x06CE,
    /// DIG Corporation (0x06CD)
    DigCorporation = 0x06CD,
    /// Dongguan SmartAction Technology Co.,Ltd. (0x06CC)
    DongguanSmartActionTechnologyCoLtd = 0x06CC,
    /// Dyeware, LLC (0x06CB)
    DyewareLlc = 0x06CB,
    /// Shenzhen Zhongguang Infotech Technology Development Co., Ltd (0x06CA)
    ShenzhenZhongguangInfotechTechnologyDevelopmentCoLtd = 0x06CA,
    /// MYLAPS B.V. (0x06C9)
    MylapsBV = 0x06C9,
    /// Storz & Bickel GmbH & Co. KG (0x06C8)
    StorzAndBickelGmbHAndCoKg = 0x06C8,
    /// Somatix Inc (0x06C7)
    SomatixInc = 0x06C7,
    /// Simm Tronic Limited (0x06C6)
    SimmTronicLimited = 0x06C6,
    /// Urban Compass, Inc (0x06C5)
    UrbanCompassInc = 0x06C5,
    /// Dream Labs GmbH (0x06C4)
    DreamLabsGmbH = 0x06C4,
    /// King I Electronics.Co.,Ltd (0x06C3)
    KingIElectronicsCoLtd = 0x06C3,
    /// Measurlogic Inc. (0x06C2)
    MeasurlogicInc = 0x06C2,
    /// Alarm.com Holdings, Inc (0x06C1)
    AlarmComHoldingsInc = 0x06C1,
    /// CAME S.p.A. (0x06C0)
    CameSPA = 0x06C0,
    /// Delcom Products Inc. (0x06BF)
    DelcomProductsInc = 0x06BF,
    /// HitSeed Oy (0x06BE)
    HitSeedOy = 0x06BE,
    /// ABB Oy (0x06BD)
    AbbOy = 0x06BD,
    /// TWS Srl (0x06BC)
    TwsSrl = 0x06BC,
    /// Leaftronix Analogic Solutions Private Limited (0x06BB)
    LeaftronixAnalogicSolutionsPrivateLimited = 0x06BB,
    /// Beaconzone Ltd (0x06BA)
    BeaconzoneLtd = 0x06BA,
    /// Beflex Inc. (0x06B9)
    BeflexInc = 0x06B9,
    /// ShadeCraft, Inc (0x06B8)
    ShadeCraftInc = 0x06B8,
    /// iCOGNIZE GmbH (0x06B7)
    ICognizeGmbH = 0x06B7,
    /// Sociometric Solutions, Inc. (0x06B6)
    SociometricSolutionsInc = 0x06B6,
    /// Wabilogic Ltd. (0x06B5)
    WabilogicLtd = 0x06B5,
    /// Sencilion Oy (0x06B4)
    SencilionOy = 0x06B4,
    /// The Hablab ApS (0x06B3)
    TheHablabApS = 0x06B3,
    /// Tussock Innovation 2013 Limited (0x06B2)
    TussockInnovation2013Limited = 0x06B2,
    /// SimpliSafe, Inc. (0x06B1)
    SimpliSafeInc = 0x06B1,
    /// BRK Brands, Inc. (0x06B0)
    BrkBrandsInc = 0x06B0,
    /// Shoof Technologies (0x06AF)
    ShoofTechnologies = 0x06AF,
    /// SenseQ Inc. (0x06AE)
    SenseQInc = 0x06AE,
    /// InnovaSea Systems Inc. (0x06AD)
    InnovaSeaSystemsInc = 0x06AD,
    /// Ingchips Technology Co., Ltd. (0x06AC)
    IngchipsTechnologyCoLtd = 0x06AC,
    /// HMS Industrial Networks AB (0x06AB)
    HmsIndustrialNetworksAb = 0x06AB,
    /// Produal Oy (0x06AA)
    ProdualOy = 0x06AA,
    /// Soundmax Electronics Limited (0x06A9)
    SoundmaxElectronicsLimited = 0x06A9,
    /// GD Midea Air-Conditioning Equipment Co., Ltd. (0x06A8)
    GdMideaAirConditioningEquipmentCoLtd = 0x06A8,
    /// Chipsea Technologies (ShenZhen) Corp. (0x06A7)
    ChipseaTechnologiesShenZhenCorp = 0x06A7,
    /// Roambee Corporation (0x06A6)
    RoambeeCorporation = 0x06A6,
    /// TEKZITEL PTY LTD (0x06A5)
    TekzitelPtyLtd = 0x06A5,
    /// LIMNO Co. Ltd. (0x06A4)
    LimnoCoLtd = 0x06A4,
    /// Nymbus, LLC (0x06A3)
    NymbusLlc = 0x06A3,
    /// Globalworx GmbH (0x06A2)
    GlobalworxGmbH = 0x06A2,
    /// Cardo Systems, Ltd (0x06A1)
    CardoSystemsLtd = 0x06A1,
    /// OBIQ Location Technology Inc. (0x06A0)
    ObiqLocationTechnologyInc = 0x06A0,
    /// FlowMotion Technologies AS (0x069F)
    FlowMotionTechnologiesAs = 0x069F,
    /// Delta Electronics, Inc. (0x069E)
    DeltaElectronicsInc = 0x069E,
    /// Vakaros LLC (0x069D)
    VakarosLlc = 0x069D,
    /// Noomi AB (0x069C)
    NoomiAb = 0x069C,
    /// Dragonchip Limited (0x069B)
    DragonchipLimited = 0x069B,
    /// Adero, Inc. (0x069A)
    AderoInc = 0x069A,
    /// RandomLab SAS (0x0699)
    RandomLabSas = 0x0699,
    /// Wood IT Security, LLC (0x0698)
    WoodItSecurityLlc = 0x0698,
    /// Stemco Products Inc (0x0697)
    StemcoProductsInc = 0x0697,
    /// Gunakar Private Limited (0x0696)
    GunakarPrivateLimited = 0x0696,
    /// Koki Holdings Co., Ltd. (0x0695)
    KokiHoldingsCoLtd = 0x0695,
    /// T&A Laboratories LLC (0x0694)
    TAndALaboratoriesLlc = 0x0694,
    /// Hach - Danaher (0x0693)
    HachDanaher = 0x0693,
    /// Georg Fischer AG (0x0692)
    GeorgFischerAg = 0x0692,
    /// Curie Point AB (0x0691)
    CuriePointAb = 0x0691,
    /// Eccrine Systems, Inc. (0x0690)
    EccrineSystemsInc = 0x0690,
    /// JRM Group Limited (0x068F)
    JrmGroupLimited = 0x068F,
    /// Razer Inc. (0x068E)
    RazerInc = 0x068E,
    /// JetBeep Inc. (0x068D)
    JetBeepInc = 0x068D,
    /// Changzhou Sound Dragon Electronics and Acoustics Co., Ltd (0x068C)
    ChangzhouSoundDragonElectronicsAndAcousticsCoLtd = 0x068C,
    /// Jiangsu Teranovo Tech Co., Ltd. (0x068B)
    JiangsuTeranovoTechCoLtd = 0x068B,
    /// Raytac Corporation (0x068A)
    RaytacCorporation = 0x068A,
    /// Tacx b.v. (0x0689)
    TacxBV = 0x0689,
    /// Amsted Digital Solutions Inc. (0x0688)
    AmstedDigitalSolutionsInc = 0x0688,
    /// Cherry GmbH (0x0687)
    CherryGmbH = 0x0687,
    /// inQs Co., Ltd. (0x0686)
    InQsCoLtd = 0x0686,
    /// Greenwald Industries (0x0685)
    GreenwaldIndustries = 0x0685,
    /// Dermalapps, LLC (0x0684)
    DermalappsLlc = 0x0684,
    /// Eltako GmbH (0x0683)
    EltakoGmbH = 0x0683,
    /// Photron Limited (0x0682)
    PhotronLimited = 0x0682,
    /// Trade FIDES a.s. (0x0681)
    TradeFidesAS = 0x0681,
    /// Mannkind Corporation (0x0680)
    MannkindCorporation = 0x0680,
    /// NETGRID S.N.C. DI BISSOLI MATTEO, CAMPOREALE SIMONE, TOGNETTI FEDERICO (0x067F)
    NetgridSNCDiBissoliMatteoCamporealeSimoneTognettiFederico = 0x067F,
    /// MbientLab Inc (0x067E)
    MbientLabInc = 0x067E,
    /// Form Athletica Inc. (0x067D)
    FormAthleticaInc = 0x067D,
    /// Tile, Inc. (0x067C)
    TileInc = 0x067C,
    /// I.FARM, INC. (0x067B)
    IFarmInc = 0x067B,
    /// The Energy Conservatory, Inc. (0x067A)
    TheEnergyConservatoryInc = 0x067A,
    /// 4iiii Innovations Inc. (0x0679)
    _4IiiiInnovationsInc = 0x0679,
    /// SABIK Offshore GmbH (0x0678)
    SabikOffshoreGmbH = 0x0678,
    /// Innovation First, Inc. (0x0677)
    InnovationFirstInc = 0x0677,
    /// Expai Solutions Private Limited (0x0676)
    ExpaiSolutionsPrivateLimited = 0x0676,
    /// Deco Enterprises, Inc. (0x0675)
    DecoEnterprisesInc = 0x0675,
    /// BeSpoon (0x0674)
    BeSpoon = 0x0674,
    /// Innova Ideas Limited (0x0673)
    InnovaIdeasLimited = 0x0673,
    /// Kopi (0x0672)
    Kopi = 0x0672,
    /// Buzz Products Ltd. (0x0671)
    BuzzProductsLtd = 0x0671,
    /// Gema Switzerland GmbH (0x0670)
    GemaSwitzerlandGmbH = 0x0670,
    /// Hug Technology Ltd (0x066F)
    HugTechnologyLtd = 0x066F,
    /// Eurotronik Kranj d.o.o. (0x066E)
    EurotronikKranjDOO = 0x066E,
    /// Venso EcoSolutions AB (0x066D)
    VensoEcoSolutionsAb = 0x066D,
    /// Ztove ApS (0x066C)
    ZtoveApS = 0x066C,
    /// DewertOkin GmbH (0x066B)
    DewertOkinGmbH = 0x066B,
    /// Brady Worldwide Inc. (0x066A)
    BradyWorldwideInc = 0x066A,
    /// Livanova USA, Inc. (0x0669)
    LivanovaUsaInc = 0x0669,
    /// Bleb Technology srl (0x0668)
    BlebTechnologySrl = 0x0668,
    /// Spark Technology Labs Inc. (0x0667)
    SparkTechnologyLabsInc = 0x0667,
    /// WTO Werkzeug-Einrichtungen GmbH (0x0666)
    WtoWerkzeugEinrichtungenGmbH = 0x0666,
    /// Pure International Limited (0x0665)
    PureInternationalLimited = 0x0665,
    /// RHA TECHNOLOGIES LTD (0x0664)
    RhaTechnologiesLtd = 0x0664,
    /// Advanced Telemetry Systems, Inc. (0x0663)
    AdvancedTelemetrySystemsInc = 0x0663,
    /// Particle Industries, Inc. (0x0662)
    ParticleIndustriesInc = 0x0662,
    /// Mode Lighting Limited (0x0661)
    ModeLightingLimited = 0x0661,
    /// RTC Industries, Inc. (0x0660)
    RtcIndustriesInc = 0x0660,
    /// Ricoh Company Ltd (0x065F)
    RicohCompanyLtd = 0x065F,
    /// Alo AB (0x065E)
    AloAb = 0x065E,
    /// Panduit Corp. (0x065D)
    PanduitCorp = 0x065D,
    /// PixArt Imaging Inc. (0x065C)
    PixArtImagingInc = 0x065C,
    /// Sesam Solutions BV (0x065B)
    SesamSolutionsBv = 0x065B,
    /// Zound Industries International AB (0x065A)
    ZoundIndustriesInternationalAb = 0x065A,
    /// UnSeen Technologies Oy (0x0659)
    UnSeenTechnologiesOy = 0x0659,
    /// Payex Norge AS (0x0658)
    PayexNorgeAs = 0x0658,
    /// Meshtronix Limited (0x0657)
    MeshtronixLimited = 0x0657,
    /// ZhuHai AdvanPro Technology Company Limited (0x0656)
    ZhuHaiAdvanProTechnologyCompanyLimited = 0x0656,
    /// Renishaw PLC (0x0655)
    RenishawPlc = 0x0655,
    /// Ledlenser GmbH & Co. KG (0x0654)
    LedlenserGmbHAndCoKg = 0x0654,
    /// Meggitt SA (0x0653)
    MeggittSa = 0x0653,
    /// ITZ Innovations- und Technologiezentrum GmbH (0x0652)
    ItzInnovationsUndTechnologiezentrumGmbH = 0x0652,
    /// Stasis Labs, Inc. (0x0651)
    StasisLabsInc = 0x0651,
    /// Coravin, Inc. (0x0650)
    CoravinInc = 0x0650,
    /// Digital Matter Pty Ltd (0x064F)
    DigitalMatterPtyLtd = 0x064F,
    /// KRUXWorks Technologies Private Limited (0x064E)
    KruxWorksTechnologiesPrivateLimited = 0x064E,
    /// iLOQ Oy (0x064D)
    ILoqOy = 0x064D,
    /// Zumtobel Group AG (0x064C)
    ZumtobelGroupAg = 0x064C,
    /// Scale-Tec, Ltd (0x064B)
    ScaleTecLtd = 0x064B,
    /// Open Research Institute, Inc. (0x064A)
    OpenResearchInstituteInc = 0x064A,
    /// Ryeex Technology Co.,Ltd. (0x0649)
    RyeexTechnologyCoLtd = 0x0649,
    /// Ultune Technologies (0x0648)
    UltuneTechnologies = 0x0648,
    /// MED-EL (0x0647)
    MedEl = 0x0647,
    /// SGV Group Holding GmbH & Co. KG (0x0646)
    SgvGroupHoldingGmbHAndCoKg = 0x0646,
    /// BM3 (0x0645)
    Bm3 = 0x0645,
    /// Apogee Instruments (0x0644)
    ApogeeInstruments = 0x0644,
    /// makita corporation (0x0643)
    MakitaCorporation = 0x0643,
    /// Bluetrum Technology Co.,Ltd (0x0642)
    BluetrumTechnologyCoLtd = 0x0642,
    /// Revenue Collection Systems FRANCE SAS (0x0641)
    RevenueCollectionSystemsFranceSas = 0x0641,
    /// Dish Network LLC (0x0640)
    DishNetworkLlc = 0x0640,
    /// LDL TECHNOLOGY (0x063F)
    LdlTechnology = 0x063F,
    /// The Indoor Lab, LLC (0x063E)
    TheIndoorLabLlc = 0x063E,
    /// Xradio Technology Co.,Ltd. (0x063D)
    XradioTechnologyCoLtd = 0x063D,
    /// Contec Medical Systems Co., Ltd. (0x063C)
    ContecMedicalSystemsCoLtd = 0x063C,
    /// Kromek Group Plc (0x063B)
    KromekGroupPlc = 0x063B,
    /// Prolojik Limited (0x063A)
    ProlojikLimited = 0x063A,
    /// Shenzhen Minew Technologies Co., Ltd. (0x0639)
    ShenzhenMinewTechnologiesCoLtd = 0x0639,
    /// LX SOLUTIONS PTY LIMITED (0x0638)
    LxSolutionsPtyLimited = 0x0638,
    /// GiP Innovation Tools GmbH (0x0637)
    GiPInnovationToolsGmbH = 0x0637,
    /// ELECTRONICA INTEGRAL DE SONIDO S.A. (0x0636)
    ElectronicaIntegralDeSonidoSA = 0x0636,
    /// Crookwood (0x0635)
    Crookwood = 0x0635,
    /// Fanstel Corp (0x0634)
    FanstelCorp = 0x0634,
    /// Wangi Lai PLT (0x0633)
    WangiLaiPlt = 0x0633,
    /// Hugo Muller GmbH & Co KG (0x0632)
    HugoMullerGmbHAndCoKg = 0x0632,
    /// Fortiori Design LLC (0x0631)
    FortioriDesignLlc = 0x0631,
    /// Asthrea D.O.O. (0x0630)
    AsthreaDOO = 0x0630,
    /// ONKYO Corporation (0x062F)
    OnkyoCorporation = 0x062F,
    /// Procept (0x062E)
    Procept = 0x062E,
    /// Vossloh-Schwabe Deutschland GmbH (0x062D)
    VosslohSchwabeDeutschlandGmbH = 0x062D,
    /// ASPion GmbH (0x062C)
    AsPionGmbH = 0x062C,
    /// MinebeaMitsumi Inc. (0x062B)
    MinebeaMitsumiInc = 0x062B,
    /// Lunatico Astronomia SL (0x062A)
    LunaticoAstronomiaSl = 0x062A,
    /// PHONEPE PVT LTD (0x0629)
    PhonepePvtLtd = 0x0629,
    /// Ensto Oy (0x0628)
    EnstoOy = 0x0628,
    /// WEG S.A. (0x0627)
    WegSA = 0x0627,
    /// Amplifico (0x0626)
    Amplifico = 0x0626,
    /// Square Panda, Inc. (0x0625)
    SquarePandaInc = 0x0625,
    /// Biovotion AG (0x0624)
    BiovotionAg = 0x0624,
    /// Philadelphia Scientific (U.K.) Limited (0x0623)
    PhiladelphiaScientificUKLimited = 0x0623,
    /// Beam Labs, LLC (0x0622)
    BeamLabsLlc = 0x0622,
    /// Noordung d.o.o. (0x0621)
    NoordungDOO = 0x0621,
    /// Forciot Oy (0x0620)
    ForciotOy = 0x0620,
    /// Phrame Inc. (0x061F)
    PhrameInc = 0x061F,
    /// Diamond Kinetics, Inc. (0x061E)
    DiamondKineticsInc = 0x061E,
    /// SENS Innovation ApS (0x061D)
    SensInnovationApS = 0x061D,
    /// Univations Limited (0x061C)
    UnivationsLimited = 0x061C,
    /// silex technology, inc. (0x061B)
    SilexTechnologyInc = 0x061B,
    /// R.W. Beckett Corporation (0x061A)
    RWBeckettCorporation = 0x061A,
    /// Six Guys Labs, s.r.o. (0x0619)
    SixGuysLabsSRO = 0x0619,
    /// Audio-Technica Corporation (0x0618)
    AudioTechnicaCorporation = 0x0618,
    /// WIZCONNECTED COMPANY LIMITED (0x0617)
    WizconnectedCompanyLimited = 0x0617,
    /// OS42 UG (haftungsbeschraenkt) (0x0616)
    Os42UgHaftungsbeschraenkt = 0x0616,
    /// INTER ACTION Corporation (0x0615)
    InterActionCorporation = 0x0615,
    /// OnAsset Intelligence, Inc. (0x0614)
    OnAssetIntelligenceInc = 0x0614,
    /// Hans Dinslage GmbH (0x0613)
    HansDinslageGmbH = 0x0613,
    /// Playfinity AS (0x0612)
    PlayfinityAs = 0x0612,
    /// Beurer GmbH (0x0611)
    BeurerGmbH = 0x0611,
    /// ADH GUARDIAN USA LLC (0x0610)
    AdhGuardianUsaLlc = 0x0610,
    /// Signify Netherlands B.V. (0x060F)
    SignifyNetherlandsBV = 0x060F,
    /// Blueair AB (0x060E)
    BlueairAb = 0x060E,
    /// TDK Corporation (0x060D)
    TdkCorporation = 0x060D,
    /// Vuzix Corporation (0x060C)
    VuzixCorporation = 0x060C,
    /// Triax Technologies Inc (0x060B)
    TriaxTechnologiesInc = 0x060B,
    /// IQAir AG (0x060A)
    IqAirAg = 0x060A,
    /// BUCHI Labortechnik AG (0x0609)
    BuchiLabortechnikAg = 0x0609,
    /// KeySafe-Cloud (0x0608)
    KeySafeCloud = 0x0608,
    /// Rookery Technology Ltd (0x0607)
    RookeryTechnologyLtd = 0x0607,
    /// John Deere (0x0606)
    JohnDeere = 0x0606,
    /// FMW electronic Futterer u. Maier-Wolf OHG (0x0605)
    FmwElectronicFuttererUMaierWolfOhg = 0x0605,
    /// Cell2Jack LLC (0x0604)
    Cell2JackLlc = 0x0604,
    /// Fourth Evolution Inc (0x0603)
    FourthEvolutionInc = 0x0603,
    /// Geberit International AG (0x0602)
    GeberitInternationalAg = 0x0602,
    /// Schrader Electronics (0x0601)
    SchraderElectronics = 0x0601,
    /// iRobot Corporation (0x0600)
    IRobotCorporation = 0x0600,
    /// Wellnomics Ltd (0x05FF)
    WellnomicsLtd = 0x05FF,
    /// Niko nv (0x05FE)
    NikoNv = 0x05FE,
    /// Innoseis (0x05FD)
    Innoseis = 0x05FD,
    /// Masbando GmbH (0x05FC)
    MasbandoGmbH = 0x05FC,
    /// Arblet Inc. (0x05FB)
    ArbletInc = 0x05FB,
    /// Konami Sports Life Co., Ltd. (0x05FA)
    KonamiSportsLifeCoLtd = 0x05FA,
    /// Hagleitner Hygiene International GmbH (0x05F9)
    HagleitnerHygieneInternationalGmbH = 0x05F9,
    /// Anki Inc. (0x05F8)
    AnkiInc = 0x05F8,
    /// TRACMO, INC. (0x05F7)
    TracmoInc = 0x05F7,
    /// DPTechnics (0x05F6)
    DpTechnics = 0x05F6,
    /// GS TAG (0x05F5)
    GsTag = 0x05F5,
    /// Clearity, LLC (0x05F4)
    ClearityLlc = 0x05F4,
    /// SeeScan (0x05F3)
    SeeScan = 0x05F3,
    /// Try and E CO.,LTD. (0x05F2)
    TryAndECoLtd = 0x05F2,
    /// The Linux Foundation (0x05F1)
    TheLinuxFoundation = 0x05F1,
    /// beken (0x05F0)
    Beken = 0x05F0,
    /// SIKOM AS (0x05EF)
    SikomAs = 0x05EF,
    /// Wristcam Inc. (0x05EE)
    WristcamInc = 0x05EE,
    /// Fuji Xerox Co., Ltd (0x05ED)
    FujiXeroxCoLtd = 0x05ED,
    /// Gycom Svenska AB (0x05EC)
    GycomSvenskaAb = 0x05EC,
    /// Bayerische Motoren Werke AG (0x05EB)
    BayerischeMotorenWerkeAg = 0x05EB,
    /// ACS-Control-System GmbH (0x05EA)
    AcsControlSystemGmbH = 0x05EA,
    /// iconmobile GmbH (0x05E9)
    IconmobileGmbH = 0x05E9,
    /// COWBOY (0x05E8)
    Cowboy = 0x05E8,
    /// PressurePro (0x05E7)
    PressurePro = 0x05E7,
    /// Motion Instruments Inc. (0x05E6)
    MotionInstrumentsInc = 0x05E6,
    /// INEO ENERGY& SYSTEMS (0x05E5)
    IneoEnergyAndSystems = 0x05E5,
    /// Taiyo Yuden Co., Ltd (0x05E4)
    TaiyoYudenCoLtd = 0x05E4,
    /// Elemental Machines, Inc. (0x05E3)
    ElementalMachinesInc = 0x05E3,
    /// stAPPtronics GmbH (0x05E2)
    StApPtronicsGmbH = 0x05E2,
    /// Human, Incorporated (0x05E1)
    HumanIncorporated = 0x05E1,
    /// Viper Design LLC (0x05E0)
    ViperDesignLlc = 0x05E0,
    /// VIRTUALCLINIC.DIRECT LIMITED (0x05DF)
    VirtualclinicDirectLimited = 0x05DF,
    /// QT Medical INC. (0x05DE)
    QtMedicalInc = 0x05DE,
    /// essentim GmbH (0x05DD)
    EssentimGmbH = 0x05DD,
    /// Petronics Inc. (0x05DC)
    PetronicsInc = 0x05DC,
    /// Avid Identification Systems, Inc. (0x05DB)
    AvidIdentificationSystemsInc = 0x05DB,
    /// Applied Neural Research Corp (0x05DA)
    AppliedNeuralResearchCorp = 0x05DA,
    /// Toyo Electronics Corporation (0x05D9)
    ToyoElectronicsCorporation = 0x05D9,
    /// Farm Jenny LLC (0x05D8)
    FarmJennyLlc = 0x05D8,
    /// modum.io AG (0x05D7)
    ModumIoAg = 0x05D7,
    /// Zhuhai Jieli technology Co.,Ltd (0x05D6)
    ZhuhaiJieliTechnologyCoLtd = 0x05D6,
    /// TEGAM, Inc. (0x05D5)
    TegamInc = 0x05D5,
    /// LAMPLIGHT Co., Ltd. (0x05D4)
    LamplightCoLtd = 0x05D4,
    /// Acurable Limited (0x05D3)
    AcurableLimited = 0x05D3,
    /// frogblue TECHNOLOGY GmbH (0x05D2)
    FrogblueTechnologyGmbH = 0x05D2,
    /// Lindab AB (0x05D1)
    LindabAb = 0x05D1,
    /// Anova Applied Electronics (0x05D0)
    AnovaAppliedElectronics = 0x05D0,
    /// Biowatch SA (0x05CF)
    BiowatchSa = 0x05CF,
    /// V-ZUG Ltd (0x05CE)
    VZugLtd = 0x05CE,
    /// RJ Brands LLC (0x05CD)
    RjBrandsLlc = 0x05CD,
    /// WATTS ELECTRONICS (0x05CC)
    WattsElectronics = 0x05CC,
    /// LucentWear LLC (0x05CB)
    LucentWearLlc = 0x05CB,
    /// MHL Custom Inc (0x05CA)
    MhlCustomInc = 0x05CA,
    /// TBS Electronics B.V. (0x05C9)
    TbsElectronicsBV = 0x05C9,
    /// SOMFY SAS (0x05C8)
    SomfySas = 0x05C8,
    /// Lippert Components, INC (0x05C7)
    LippertComponentsInc = 0x05C7,
    /// Smart Animal Training Systems, LLC (0x05C6)
    SmartAnimalTrainingSystemsLlc = 0x05C6,
    /// SELVE GmbH & Co. KG (0x05C5)
    SelveGmbHAndCoKg = 0x05C5,
    /// Codecoup sp. z o.o. sp. k. (0x05C4)
    CodecoupSpZOOSpK = 0x05C4,
    /// Runtime, Inc. (0x05C3)
    RuntimeInc = 0x05C3,
    /// Grote Industries (0x05C2)
    GroteIndustries = 0x05C2,
    /// P.I.Engineering (0x05C1)
    PIEngineering = 0x05C1,
    /// Nalu Medical, Inc. (0x05C0)
    NaluMedicalInc = 0x05C0,
    /// Real-World-Systems Corporation (0x05BF)
    RealWorldSystemsCorporation = 0x05BF,
    /// RFID Global by Softwork SrL (0x05BE)
    RfidGlobalBySoftworkSrL = 0x05BE,
    /// ULC Robotics Inc. (0x05BD)
    UlcRoboticsInc = 0x05BD,
    /// Leviton Mfg. Co., Inc. (0x05BC)
    LevitonMfgCoInc = 0x05BC,
    /// Oxford Metrics plc (0x05BB)
    OxfordMetricsPlc = 0x05BB,
    /// igloohome (0x05BA)
    Igloohome = 0x05BA,
    /// Suzhou Pairlink Network Technology (0x05B9)
    SuzhouPairlinkNetworkTechnology = 0x05B9,
    /// Ambystoma Labs Inc. (0x05B8)
    AmbystomaLabsInc = 0x05B8,
    /// Beijing Pinecone Electronics Co.,Ltd. (0x05B7)
    BeijingPineconeElectronicsCoLtd = 0x05B7,
    /// Elecs Industry Co.,Ltd. (0x05B6)
    ElecsIndustryCoLtd = 0x05B6,
    /// verisilicon (0x05B5)
    Verisilicon = 0x05B5,
    /// White Horse Scientific ltd (0x05B4)
    WhiteHorseScientificLtd = 0x05B4,
    /// Parabit Systems, Inc. (0x05B3)
    ParabitSystemsInc = 0x05B3,
    /// CAREL INDUSTRIES S.P.A. (0x05B2)
    CarelIndustriesSPA = 0x05B2,
    /// Medallion Instrumentation Systems (0x05B1)
    MedallionInstrumentationSystems = 0x05B1,
    /// NewTec GmbH (0x05B0)
    NewTecGmbH = 0x05B0,
    /// OV LOOP, INC. (0x05AF)
    OvLoopInc = 0x05AF,
    /// CARMATE MFG.CO.,LTD (0x05AE)
    CarmateMfgCoLtd = 0x05AE,
    /// INIA (0x05AD)
    Inia = 0x05AD,
    /// GoerTek Dynaudio Co., Ltd. (0x05AC)
    GoerTekDynaudioCoLtd = 0x05AC,
    /// Nofence AS (0x05AB)
    NofenceAs = 0x05AB,
    /// Tramex Limited (0x05AA)
    TramexLimited = 0x05AA,
    /// Monidor (0x05A9)
    Monidor = 0x05A9,
    /// Tom Allebrandi Consulting (0x05A8)
    TomAllebrandiConsulting = 0x05A8,
    /// Sonos Inc (0x05A7)
    SonosInc = 0x05A7,
    /// Telecon Mobile Limited (0x05A6)
    TeleconMobileLimited = 0x05A6,
    /// Kiiroo BV (0x05A5)
    KiirooBv = 0x05A5,
    /// O. E. M. Controls, Inc. (0x05A4)
    OEMControlsInc = 0x05A4,
    /// Axiomware Systems Incorporated (0x05A3)
    AxiomwareSystemsIncorporated = 0x05A3,
    /// ADHERIUM(NZ) LIMITED (0x05A2)
    AdheriumNzLimited = 0x05A2,
    /// Shanghai Xiaoyi Technology Co.,Ltd. (0x05A1)
    ShanghaiXiaoyiTechnologyCoLtd = 0x05A1,
    /// Dream Devices Technologies Oy (0x05A0)
    DreamDevicesTechnologiesOy = 0x05A0,
    /// Fisher & Paykel Healthcare (0x059F)
    FisherAndPaykelHealthcare = 0x059F,
    /// Polycom, Inc. (0x059E)
    PolycomInc = 0x059E,
    /// Tandem Diabetes Care (0x059D)
    TandemDiabetesCare = 0x059D,
    /// Macrogiga Electronics (0x059C)
    MacrogigaElectronics = 0x059C,
    /// Dataflow Systems Limited (0x059B)
    DataflowSystemsLimited = 0x059B,
    /// Teledyne Lecroy, Inc. (0x059A)
    TeledyneLecroyInc = 0x059A,
    /// Lazlo326, LLC. (0x0599)
    Lazlo326Llc = 0x0599,
    /// rapitag GmbH (0x0598)
    RapitagGmbH = 0x0598,
    /// RadioPulse Inc (0x0597)
    RadioPulseInc = 0x0597,
    /// My Smart Blinds (0x0596)
    MySmartBlinds = 0x0596,
    /// Inor Process AB (0x0595)
    InorProcessAb = 0x0595,
    /// Kohler Company (0x0594)
    KohlerCompany = 0x0594,
    /// Spaulding Clinical Research (0x0593)
    SpauldingClinicalResearch = 0x0593,
    /// IZITHERM (0x0592)
    Izitherm = 0x0592,
    /// Viasat Group S.p.A. (0x0591)
    ViasatGroupSPA = 0x0591,
    /// Pur3 Ltd (0x0590)
    Pur3Ltd = 0x0590,
    /// HENDON SEMICONDUCTORS PTY LTD (0x058F)
    HendonSemiconductorsPtyLtd = 0x058F,
    /// Meta Platforms Technologies, LLC (0x058E)
    MetaPlatformsTechnologiesLlc = 0x058E,
    /// Jungheinrich Aktiengesellschaft (0x058D)
    JungheinrichAktiengesellschaft = 0x058D,
    /// Fracarro Radioindustrie SRL (0x058C)
    FracarroRadioindustrieSrl = 0x058C,
    /// Maxim Integrated Products (0x058B)
    MaximIntegratedProducts = 0x058B,
    /// START TODAY CO.,LTD. (0x058A)
    StartTodayCoLtd = 0x058A,
    /// Star Technologies (0x0589)
    StarTechnologies = 0x0589,
    /// ALT-TEKNIK LLC (0x0588)
    AltTeknikLlc = 0x0588,
    /// Derichs GmbH (0x0587)
    DerichsGmbH = 0x0587,
    /// LEGRAND (0x0586)
    Legrand = 0x0586,
    /// Hearing Lab Technology (0x0585)
    HearingLabTechnology = 0x0585,
    /// Gira Giersiepen GmbH & Co. KG (0x0584)
    GiraGiersiepenGmbHAndCoKg = 0x0584,
    /// Code Blue Communications (0x0583)
    CodeBlueCommunications = 0x0583,
    /// Breakwall Analytics, LLC (0x0582)
    BreakwallAnalyticsLlc = 0x0582,
    /// LYS TECHNOLOGIES LTD (0x0581)
    LysTechnologiesLtd = 0x0581,
    /// ARANZ Medical Limited (0x0580)
    AranzMedicalLimited = 0x0580,
    /// Scuf Gaming International, LLC (0x057F)
    ScufGamingInternationalLlc = 0x057F,
    /// Beco, Inc (0x057E)
    BecoInc = 0x057E,
    /// Instinct Performance (0x057D)
    InstinctPerformance = 0x057D,
    /// Toor Technologies LLC (0x057C)
    ToorTechnologiesLlc = 0x057C,
    /// Duracell U.S. Operations Inc. (0x057B)
    DuracellUSOperationsInc = 0x057B,
    /// OMNI Remotes (0x057A)
    OmniRemotes = 0x057A,
    /// Ensemble Tech Private Limited (0x0579)
    EnsembleTechPrivateLimited = 0x0579,
    /// Wellington Drive Technologies Ltd (0x0578)
    WellingtonDriveTechnologiesLtd = 0x0578,
    /// True Wearables, Inc. (0x0577)
    TrueWearablesInc = 0x0577,
    /// Globalstar, Inc. (0x0576)
    GlobalstarInc = 0x0576,
    /// Integral Memroy Plc (0x0575)
    IntegralMemroyPlc = 0x0575,
    /// AFFORDABLE ELECTRONICS INC (0x0574)
    AffordableElectronicsInc = 0x0574,
    /// Lighting Science Group Corp. (0x0573)
    LightingScienceGroupCorp = 0x0573,
    /// AntTail.com (0x0572)
    AntTailCom = 0x0572,
    /// PSIKICK, INC. (0x0571)
    PsikickInc = 0x0571,
    /// Consumer Sleep Solutions LLC (0x0570)
    ConsumerSleepSolutionsLlc = 0x0570,
    /// BikeFinder AS (0x056F)
    BikeFinderAs = 0x056F,
    /// VIZPIN INC. (0x056E)
    VizpinInc = 0x056E,
    /// Redmond Industrial Group LLC (0x056D)
    RedmondIndustrialGroupLlc = 0x056D,
    /// Long Range Systems, LLC (0x056C)
    LongRangeSystemsLlc = 0x056C,
    /// Rion Co., Ltd. (0x056B)
    RionCoLtd = 0x056B,
    /// Flipnavi Co.,Ltd. (0x056A)
    FlipnaviCoLtd = 0x056A,
    /// Audionics System, INC. (0x0569)
    AudionicsSystemInc = 0x0569,
    /// Bodyport Inc. (0x0568)
    BodyportInc = 0x0568,
    /// Xiamen Everesports Goods Co., Ltd (0x0567)
    XiamenEveresportsGoodsCoLtd = 0x0567,
    /// CORE TRANSPORT TECHNOLOGIES NZ LIMITED (0x0566)
    CoreTransportTechnologiesNzLimited = 0x0566,
    /// Beijing Smartspace Technologies Inc. (0x0565)
    BeijingSmartspaceTechnologiesInc = 0x0565,
    /// Beghelli Spa (0x0564)
    BeghelliSpa = 0x0564,
    /// Steinel Vertrieb GmbH (0x0563)
    SteinelVertriebGmbH = 0x0563,
    /// Thalmic Labs Inc. (0x0562)
    ThalmicLabsInc = 0x0562,
    /// Finder S.p.A. (0x0561)
    FinderSPA = 0x0561,
    /// Sarita CareTech APS (0x0560)
    SaritaCareTechAps = 0x0560,
    /// PROTECH S.A.S. DI GIRARDI ANDREA & C. (0x055F)
    ProtechSASDiGirardiAndreaAndC = 0x055F,
    /// Hekatron Vertriebs GmbH (0x055E)
    HekatronVertriebsGmbH = 0x055E,
    /// Valve Corporation (0x055D)
    ValveCorporation = 0x055D,
    /// Lely (0x055C)
    Lely = 0x055C,
    /// FRANKLIN TECHNOLOGY INC (0x055B)
    FranklinTechnologyInc = 0x055B,
    /// CANDY HOUSE, Inc. (0x055A)
    CandyHouseInc = 0x055A,
    /// Newcon Optik (0x0559)
    NewconOptik = 0x0559,
    /// benegear, inc. (0x0558)
    BenegearInc = 0x0558,
    /// Arwin Technology Limited (0x0557)
    ArwinTechnologyLimited = 0x0557,
    /// Otodynamics Ltd (0x0556)
    OtodynamicsLtd = 0x0556,
    /// KROHNE Messtechnik GmbH (0x0555)
    KrohneMesstechnikGmbH = 0x0555,
    /// National Instruments (0x0554)
    NationalInstruments = 0x0554,
    /// Nintendo Co., Ltd. (0x0553)
    NintendoCoLtd = 0x0553,
    /// Avempace SARL (0x0552)
    AvempaceSarl = 0x0552,
    /// Sylero (0x0551)
    Sylero = 0x0551,
    /// Versa Networks, Inc. (0x0550)
    VersaNetworksInc = 0x0550,
    /// Sinnoz (0x054F)
    Sinnoz = 0x054F,
    /// FORTRONIK storitve d.o.o. (0x054E)
    FortronikStoritveDOO = 0x054E,
    /// Sensome (0x054D)
    Sensome = 0x054D,
    /// Carefree Scott Fetzer Co Inc (0x054C)
    CarefreeScottFetzerCoInc = 0x054C,
    /// Advanced Electronic Designs, Inc. (0x054B)
    AdvancedElectronicDesignsInc = 0x054B,
    /// Linough Inc. (0x054A)
    LinoughInc = 0x054A,
    /// Smart Technologies and Investment Limited (0x0549)
    SmartTechnologiesAndInvestmentLimited = 0x0549,
    /// Knick Elektronische Messgeraete GmbH & Co. KG (0x0548)
    KnickElektronischeMessgeraeteGmbHAndCoKg = 0x0548,
    /// LOGICDATA Electronic & Software Entwicklungs GmbH (0x0547)
    LogicdataElectronicAndSoftwareEntwicklungsGmbH = 0x0547,
    /// Apexar Technologies S.A. (0x0546)
    ApexarTechnologiesSA = 0x0546,
    /// Candy Hoover Group s.r.l (0x0545)
    CandyHooverGroupSRL = 0x0545,
    /// OrthoSensor, Inc. (0x0544)
    OrthoSensorInc = 0x0544,
    /// MIWA LOCK CO.,Ltd (0x0543)
    MiwaLockCoLtd = 0x0543,
    /// Mist Systems, Inc. (0x0542)
    MistSystemsInc = 0x0542,
    /// Sharknet srl (0x0541)
    SharknetSrl = 0x0541,
    /// SilverPlus, Inc (0x0540)
    SilverPlusInc = 0x0540,
    /// Silergy Corp (0x053F)
    SilergyCorp = 0x053F,
    /// CLIM8 LIMITED (0x053E)
    Clim8Limited = 0x053E,
    /// TESA SA (0x053D)
    TesaSa = 0x053D,
    /// Screenovate Technologies Ltd (0x053C)
    ScreenovateTechnologiesLtd = 0x053C,
    /// prodigy (0x053B)
    Prodigy = 0x053B,
    /// Savitech Corp., (0x053A)
    SavitechCorp = 0x053A,
    /// OPPLE Lighting Co., Ltd (0x0539)
    OppleLightingCoLtd = 0x0539,
    /// Medela AG (0x0538)
    MedelaAg = 0x0538,
    /// MetaLogics Corporation (0x0537)
    MetaLogicsCorporation = 0x0537,
    /// ZTR Control Systems LLC (0x0536)
    ZtrControlSystemsLlc = 0x0536,
    /// Smart Component Technologies Limited (0x0535)
    SmartComponentTechnologiesLimited = 0x0535,
    /// Frontiergadget, Inc. (0x0534)
    FrontiergadgetInc = 0x0534,
    /// Nura Operations Pty Ltd (0x0533)
    NuraOperationsPtyLtd = 0x0533,
    /// CRESCO Wireless, Inc. (0x0532)
    CrescoWirelessInc = 0x0532,
    /// D&M Holdings Inc. (0x0531)
    DAndMHoldingsInc = 0x0531,
    /// Adolene, Inc. (0x0530)
    AdoleneInc = 0x0530,
    /// Center ID Corp. (0x052F)
    CenterIdCorp = 0x052F,
    /// LEDVANCE GmbH (0x052E)
    LedvanceGmbH = 0x052E,
    /// EXFO, Inc. (0x052D)
    ExfoInc = 0x052D,
    /// Geosatis SA (0x052C)
    GeosatisSa = 0x052C,
    /// Novartis AG (0x052B)
    NovartisAg = 0x052B,
    /// Keynes Controls Ltd (0x052A)
    KeynesControlsLtd = 0x052A,
    /// Lumen UAB (0x0529)
    LumenUab = 0x0529,
    /// Lunera Lighting Inc. (0x0528)
    LuneraLightingInc = 0x0528,
    /// Albrecht JUNG (0x0527)
    AlbrechtJung = 0x0527,
    /// Honeywell International Inc. (0x0526)
    HoneywellInternationalInc = 0x0526,
    /// HONGKONG NANO IC TECHNOLOGIES  CO., LIMITED (0x0525)
    HongkongNanoIcTechnologiesCoLimited = 0x0525,
    /// Hangzhou iMagic Technology Co., Ltd (0x0524)
    HangzhouIMagicTechnologyCoLtd = 0x0524,
    /// MTG Co., Ltd. (0x0523)
    MtgCoLtd = 0x0523,
    /// NS Tech, Inc. (0x0522)
    NsTechInc = 0x0522,
    /// IAI Corporation (0x0521)
    IaiCorporation = 0x0521,
    /// Target Corporation (0x0520)
    TargetCorporation = 0x0520,
    /// Setec Pty Ltd (0x051F)
    SetecPtyLtd = 0x051F,
    /// Detect Blue Limited (0x051E)
    DetectBlueLimited = 0x051E,
    /// OFF Line Co., Ltd. (0x051D)
    OffLineCoLtd = 0x051D,
    /// EDPS (0x051C)
    Edps = 0x051C,
    /// Angee Technologies Ltd. (0x051B)
    AngeeTechnologiesLtd = 0x051B,
    /// Leica Camera AG (0x051A)
    LeicaCameraAg = 0x051A,
    /// Tyto Life LLC (0x0519)
    TytoLifeLlc = 0x0519,
    /// MAMORIO.inc (0x0518)
    MamorioInc = 0x0518,
    /// Amtronic Sverige AB (0x0517)
    AmtronicSverigeAb = 0x0517,
    /// Footmarks (0x0516)
    Footmarks = 0x0516,
    /// RB Controls Co., Ltd. (0x0515)
    RbControlsCoLtd = 0x0515,
    /// FIBRO GmbH (0x0514)
    FibroGmbH = 0x0514,
    /// 9974091 Canada Inc. (0x0513)
    _9974091CanadaInc = 0x0513,
    /// Soprod SA (0x0512)
    SoprodSa = 0x0512,
    /// Brookfield Equinox LLC (0x0511)
    BrookfieldEquinoxLlc = 0x0511,
    /// UNI-ELECTRONICS, INC. (0x0510)
    UniElectronicsInc = 0x0510,
    /// Foundation Engineering LLC (0x050F)
    FoundationEngineeringLlc = 0x050F,
    /// Yichip Microelectronics (Hangzhou) Co.,Ltd. (0x050E)
    YichipMicroelectronicsHangzhouCoLtd = 0x050E,
    /// TRSystems GmbH (0x050D)
    TrSystemsGmbH = 0x050D,
    /// OSRAM GmbH (0x050C)
    OsramGmbH = 0x050C,
    /// Vibrissa Inc. (0x050B)
    VibrissaInc = 0x050B,
    /// Shake-on B.V. (0x050A)
    ShakeOnBV = 0x050A,
    /// Garage Smart, Inc. (0x0509)
    GarageSmartInc = 0x0509,
    /// Axes System sp. z o. o. (0x0508)
    AxesSystemSpZOO = 0x0508,
    /// Yellowcog (0x0507)
    Yellowcog = 0x0507,
    /// Hager (0x0506)
    Hager = 0x0506,
    /// InPlay, Inc. (0x0505)
    InPlayInc = 0x0505,
    /// PHYPLUS Inc (0x0504)
    PhyplusInc = 0x0504,
    /// Locoroll, Inc (0x0503)
    LocorollInc = 0x0503,
    /// Specifi-Kali LLC (0x0502)
    SpecifiKaliLlc = 0x0502,
    /// Polaris IND (0x0501)
    PolarisInd = 0x0501,
    /// Wiliot LTD. (0x0500)
    WiliotLtd = 0x0500,
    /// Microsemi Corporation (0x04FF)
    MicrosemiCorporation = 0x04FF,
    /// Woosim Systems Inc. (0x04FE)
    WoosimSystemsInc = 0x04FE,
    /// Tapkey GmbH (0x04FD)
    TapkeyGmbH = 0x04FD,
    /// SwingLync L. L. C. (0x04FC)
    SwingLyncLLC = 0x04FC,
    /// Benchmark Drives GmbH & Co. KG (0x04FB)
    BenchmarkDrivesGmbHAndCoKg = 0x04FB,
    /// Androtec GmbH (0x04FA)
    AndrotecGmbH = 0x04FA,
    /// Interactio (0x04F9)
    Interactio = 0x04F9,
    /// Convergence Systems Limited (0x04F8)
    ConvergenceSystemsLimited = 0x04F8,
    /// Shenzhen Goodix Technology Co., Ltd (0x04F7)
    ShenzhenGoodixTechnologyCoLtd = 0x04F7,
    /// McLear Limited (0x04F6)
    McLearLimited = 0x04F6,
    /// Pirelli Tyre S.P.A. (0x04F5)
    PirelliTyreSPA = 0x04F5,
    /// ZanCompute Inc. (0x04F4)
    ZanComputeInc = 0x04F4,
    /// Cerevast Medical (0x04F3)
    CerevastMedical = 0x04F3,
    /// InDreamer Techsol Private Limited (0x04F2)
    InDreamerTechsolPrivateLimited = 0x04F2,
    /// Theben AG (0x04F1)
    ThebenAg = 0x04F1,
    /// Kosi Limited (0x04F0)
    KosiLimited = 0x04F0,
    /// DaisyWorks, Inc (0x04EF)
    DaisyWorksInc = 0x04EF,
    /// Auxivia (0x04EE)
    Auxivia = 0x04EE,
    /// R9 Technology, Inc. (0x04ED)
    R9TechnologyInc = 0x04ED,
    /// Motorola Solutions (0x04EC)
    MotorolaSolutions = 0x04EC,
    /// Bird Home Automation GmbH (0x04EB)
    BirdHomeAutomationGmbH = 0x04EB,
    /// Pacific Bioscience Laboratories, Inc (0x04EA)
    PacificBioscienceLaboratoriesInc = 0x04EA,
    /// Busch Jaeger Elektro GmbH (0x04E9)
    BuschJaegerElektroGmbH = 0x04E9,
    /// STABILO International (0x04E8)
    StabiloInternational = 0x04E8,
    /// REHABTRONICS INC. (0x04E7)
    RehabtronicsInc = 0x04E7,
    /// Smart Solution Technology, Inc. (0x04E6)
    SmartSolutionTechnologyInc = 0x04E6,
    /// Avack Oy (0x04E5)
    AvackOy = 0x04E5,
    /// Woodenshark (0x04E4)
    Woodenshark = 0x04E4,
    /// Under Armour (0x04E3)
    UnderArmour = 0x04E3,
    /// EllieGrid (0x04E2)
    EllieGrid = 0x04E2,
    /// REACTEC LIMITED (0x04E1)
    ReactecLimited = 0x04E1,
    /// Guardtec, Inc. (0x04E0)
    GuardtecInc = 0x04E0,
    /// Emerson Electric Co. (0x04DF)
    EmersonElectricCo = 0x04DF,
    /// Lutron Electronics Co., Inc. (0x04DE)
    LutronElectronicsCoInc = 0x04DE,
    /// 4MOD Technology (0x04DD)
    _4ModTechnology = 0x04DD,
    /// IOTTIVE (OPC) PRIVATE LIMITED (0x04DC)
    IottiveOpcPrivateLimited = 0x04DC,
    /// Engineered Audio, LLC. (0x04DB)
    EngineeredAudioLlc = 0x04DB,
    /// Franceschi Marina snc (0x04DA)
    FranceschiMarinaSnc = 0x04DA,
    /// RM Acquisition LLC (0x04D9)
    RmAcquisitionLlc = 0x04D9,
    /// FUJIFILM Corporation (0x04D8)
    FujifilmCorporation = 0x04D8,
    /// Blincam, Inc. (0x04D7)
    BlincamInc = 0x04D7,
    /// LUGLOC LLC (0x04D6)
    LuglocLlc = 0x04D6,
    /// Gooee Limited (0x04D5)
    GooeeLimited = 0x04D5,
    /// 5th Element Ltd (0x04D4)
    _5ThElementLtd = 0x04D4,
    /// Queercon, Inc (0x04D3)
    QueerconInc = 0x04D3,
    /// Anloq Technologies Inc. (0x04D2)
    AnloqTechnologiesInc = 0x04D2,
    /// KTS GmbH (0x04D1)
    KtsGmbH = 0x04D1,
    /// Olympus Corporation (0x04D0)
    OlympusCorporation = 0x04D0,
    /// DOM Sicherheitstechnik GmbH & Co. KG (0x04CF)
    DomSicherheitstechnikGmbHAndCoKg = 0x04CF,
    /// GOOOLED S.R.L. (0x04CE)
    GoooledSRL = 0x04CE,
    /// Safetech Products LLC (0x04CD)
    SafetechProductsLlc = 0x04CD,
    /// Enflux Inc. (0x04CC)
    EnfluxInc = 0x04CC,
    /// Novo Nordisk A/S (0x04CB)
    NovoNordiskAS = 0x04CB,
    /// Steiner-Optik GmbH (0x04CA)
    SteinerOptikGmbH = 0x04CA,
    /// Thornwave Labs Inc (0x04C9)
    ThornwaveLabsInc = 0x04C9,
    /// Shanghai Flyco Electrical Appliance Co., Ltd. (0x04C8)
    ShanghaiFlycoElectricalApplianceCoLtd = 0x04C8,
    /// Svantek Sp. z o.o. (0x04C7)
    SvantekSpZOO = 0x04C7,
    /// Insta GmbH (0x04C6)
    InstaGmbH = 0x04C6,
    /// Seibert Williams Glass, LLC (0x04C5)
    SeibertWilliamsGlassLlc = 0x04C5,
    /// TeAM Hutchins AB (0x04C4)
    TeAmHutchinsAb = 0x04C4,
    /// Mantracourt Electronics Limited (0x04C3)
    MantracourtElectronicsLimited = 0x04C3,
    /// Dmet Products Corp. (0x04C2)
    DmetProductsCorp = 0x04C2,
    /// Sospitas, s.r.o. (0x04C1)
    SospitasSRO = 0x04C1,
    /// Statsports International (0x04C0)
    StatsportsInternational = 0x04C0,
    /// VIT Initiative, LLC (0x04BF)
    VitInitiativeLlc = 0x04BF,
    /// Averos FZCO (0x04BE)
    AverosFzco = 0x04BE,
    /// AlbynMedical (0x04BD)
    AlbynMedical = 0x04BD,
    /// Draegerwerk AG & Co. KGaA (0x04BC)
    DraegerwerkAgAndCoKGaA = 0x04BC,
    /// Neatebox Ltd (0x04BB)
    NeateboxLtd = 0x04BB,
    /// Crestron Electronics, Inc. (0x04BA)
    CrestronElectronicsInc = 0x04BA,
    /// CSR Building Products Limited (0x04B9)
    CsrBuildingProductsLimited = 0x04B9,
    /// Soraa Inc. (0x04B8)
    SoraaInc = 0x04B8,
    /// Analog Devices, Inc. (0x04B7)
    AnalogDevicesInc = 0x04B7,
    /// Diagnoptics Technologies (0x04B6)
    DiagnopticsTechnologies = 0x04B6,
    /// Swiftronix AB (0x04B5)
    SwiftronixAb = 0x04B5,
    /// Inuheat Group AB (0x04B4)
    InuheatGroupAb = 0x04B4,
    /// mobike (Hong Kong) Limited (0x04B3)
    MobikeHongKongLimited = 0x04B3,
    /// The Shadow on the Moon (0x04B2)
    TheShadowOnTheMoon = 0x04B2,
    /// Kartographers Technologies Pvt. Ltd. (0x04B1)
    KartographersTechnologiesPvtLtd = 0x04B1,
    /// Weba Sport und Med. Artikel GmbH (0x04B0)
    WebaSportUndMedArtikelGmbH = 0x04B0,
    /// BIOROWER Handelsagentur GmbH (0x04AF)
    BiorowerHandelsagenturGmbH = 0x04AF,
    /// ERM Electronic Systems LTD (0x04AE)
    ErmElectronicSystemsLtd = 0x04AE,
    /// Shure Inc (0x04AD)
    ShureInc = 0x04AD,
    /// Undagrid B.V. (0x04AC)
    UndagridBV = 0x04AC,
    /// Harbortronics, Inc. (0x04AB)
    HarbortronicsInc = 0x04AB,
    /// LINKIO SAS (0x04AA)
    LinkioSas = 0x04AA,
    /// DISCOVERY SOUND TECHNOLOGY, LLC (0x04A9)
    DiscoverySoundTechnologyLlc = 0x04A9,
    /// BioTex, Inc. (0x04A8)
    BioTexInc = 0x04A8,
    /// Dallas Logic Corporation (0x04A7)
    DallasLogicCorporation = 0x04A7,
    /// Vinetech Co., Ltd (0x04A6)
    VinetechCoLtd = 0x04A6,
    /// Guangzhou FiiO Electronics Technology Co.,Ltd (0x04A5)
    GuangzhouFiiOElectronicsTechnologyCoLtd = 0x04A5,
    /// Herbert Waldmann GmbH & Co. KG (0x04A4)
    HerbertWaldmannGmbHAndCoKg = 0x04A4,
    /// GT-tronics HK Ltd (0x04A3)
    GtTronicsHkLtd = 0x04A3,
    /// ovrEngineered, LLC (0x04A2)
    OvrEngineeredLlc = 0x04A2,
    /// PNI Sensor Corporation (0x04A1)
    PniSensorCorporation = 0x04A1,
    /// Vypin, LLC (0x04A0)
    VypinLlc = 0x04A0,
    /// Popper Pay AB (0x049F)
    PopperPayAb = 0x049F,
    /// AND!XOR LLC (0x049E)
    AndXorLlc = 0x049E,
    /// Uhlmann & Zacher GmbH (0x049D)
    UhlmannAndZacherGmbH = 0x049D,
    /// DyOcean (0x049C)
    DyOcean = 0x049C,
    /// nVisti, LLC (0x049B)
    NVistiLlc = 0x049B,
    /// Situne AS (0x049A)
    SituneAs = 0x049A,
    /// Ruuvi Innovations Ltd. (0x0499)
    RuuviInnovationsLtd = 0x0499,
    /// METER Group, Inc. USA (0x0498)
    MeterGroupIncUsa = 0x0498,
    /// Cochlear Limited (0x0497)
    CochlearLimited = 0x0497,
    /// Polymorphic Labs LLC (0x0496)
    PolymorphicLabsLlc = 0x0496,
    /// LMT Mercer Group, Inc (0x0495)
    LmtMercerGroupInc = 0x0495,
    /// SENNHEISER electronic GmbH & Co. KG (0x0494)
    SennheiserElectronicGmbHAndCoKg = 0x0494,
    /// Lynxemi Pte Ltd (0x0493)
    LynxemiPteLtd = 0x0493,
    /// ADC Technology, Inc. (0x0492)
    AdcTechnologyInc = 0x0492,
    /// SOREX - Wireless Solutions GmbH (0x0491)
    SorexWirelessSolutionsGmbH = 0x0491,
    /// Matting AB (0x0490)
    MattingAb = 0x0490,
    /// BlueKitchen GmbH (0x048F)
    BlueKitchenGmbH = 0x048F,
    /// Companion Medical, Inc. (0x048E)
    CompanionMedicalInc = 0x048E,
    /// S-Labs Sp. z o.o. (0x048D)
    SLabsSpZOO = 0x048D,
    /// Vectronix AG (0x048C)
    VectronixAg = 0x048C,
    /// CP Electronics Limited (0x048B)
    CpElectronicsLimited = 0x048B,
    /// Taelek Oy (0x048A)
    TaelekOy = 0x048A,
    /// Igarashi Engineering (0x0489)
    IgarashiEngineering = 0x0489,
    /// Automotive Data Solutions Inc (0x0488)
    AutomotiveDataSolutionsInc = 0x0488,
    /// Centrica Connected Home (0x0487)
    CentricaConnectedHome = 0x0487,
    /// DEV TECNOLOGIA INDUSTRIA, COMERCIO E MANUTENCAO DE EQUIPAMENTOS LTDA. - ME (0x0486)
    DevTecnologiaIndustriaComercioEManutencaoDeEquipamentosLtdaMe = 0x0486,
    /// SKIDATA AG (0x0485)
    SkidataAg = 0x0485,
    /// Revol Technologies Inc (0x0484)
    RevolTechnologiesInc = 0x0484,
    /// Multi Care Systems B.V. (0x0483)
    MultiCareSystemsBV = 0x0483,
    /// POS Tuning Udo Vosshenrich GmbH & Co. KG (0x0482)
    PosTuningUdoVosshenrichGmbHAndCoKg = 0x0482,
    /// Quintrax Limited (0x0481)
    QuintraxLimited = 0x0481,
    /// Dynometrics Inc. (0x0480)
    DynometricsInc = 0x0480,
    /// Pro-Mark, Inc. (0x047F)
    ProMarkInc = 0x047F,
    /// OurHub Dev IvS (0x047E)
    OurHubDevIvS = 0x047E,
    /// Occly LLC (0x047D)
    OcclyLlc = 0x047D,
    /// POWERMAT LTD (0x047C)
    PowermatLtd = 0x047C,
    /// MIYOSHI ELECTRONICS CORPORATION (0x047B)
    MiyoshiElectronicsCorporation = 0x047B,
    /// Sinosun Technology Co., Ltd. (0x047A)
    SinosunTechnologyCoLtd = 0x047A,
    /// mywerk system GmbH (0x0479)
    MywerkSystemGmbH = 0x0479,
    /// FarSite Communications Limited (0x0478)
    FarSiteCommunicationsLimited = 0x0478,
    /// Blue Spark Technologies (0x0477)
    BlueSparkTechnologies = 0x0477,
    /// Oxstren Wearable Technologies Private Limited (0x0476)
    OxstrenWearableTechnologiesPrivateLimited = 0x0476,
    /// Icom inc. (0x0475)
    IcomInc = 0x0475,
    /// iApartment co., ltd. (0x0474)
    IApartmentCoLtd = 0x0474,
    /// Steelcase, Inc. (0x0473)
    SteelcaseInc = 0x0473,
    /// Control-J Pty Ltd (0x0472)
    ControlJPtyLtd = 0x0472,
    /// TiVo Corp (0x0471)
    TiVoCorp = 0x0471,
    /// iDesign s.r.l. (0x0470)
    IDesignSRL = 0x0470,
    /// Develco Products A/S (0x046F)
    DevelcoProductsAS = 0x046F,
    /// Pambor Ltd. (0x046E)
    PamborLtd = 0x046E,
    /// BEGA Gantenbrink-Leuchten KG (0x046D)
    BegaGantenbrinkLeuchtenKg = 0x046D,
    /// Qingdao Realtime Technology Co., Ltd. (0x046C)
    QingdaoRealtimeTechnologyCoLtd = 0x046C,
    /// PMD Solutions (0x046B)
    PmdSolutions = 0x046B,
    /// INSIGMA INC. (0x046A)
    InsigmaInc = 0x046A,
    /// Palago AB (0x0469)
    PalagoAb = 0x0469,
    /// Kynesim Ltd (0x0468)
    KynesimLtd = 0x0468,
    /// Codenex Oy (0x0467)
    CodenexOy = 0x0467,
    /// CycleLabs Solutions inc. (0x0466)
    CycleLabsSolutionsInc = 0x0466,
    /// International Forte Group LLC (0x0465)
    InternationalForteGroupLlc = 0x0465,
    /// Bellman & Symfon Group AB (0x0464)
    BellmanAndSymfonGroupAb = 0x0464,
    /// Fathom Systems Inc. (0x0463)
    FathomSystemsInc = 0x0463,
    /// Bonsai Systems GmbH (0x0462)
    BonsaiSystemsGmbH = 0x0462,
    /// vhf elektronik GmbH (0x0461)
    VhfElektronikGmbH = 0x0461,
    /// Kolibree (0x0460)
    Kolibree = 0x0460,
    /// Real Time Automation, Inc. (0x045F)
    RealTimeAutomationInc = 0x045F,
    /// Nuviz, Inc. (0x045E)
    NuvizInc = 0x045E,
    /// Boston Scientific Corporation (0x045D)
    BostonScientificCorporation = 0x045D,
    /// Delta T Corporation (0x045C)
    DeltaTCorporation = 0x045C,
    /// SPACEEK LTD (0x045B)
    SpaceekLtd = 0x045B,
    /// 2048450 Ontario Inc (0x045A)
    _2048450OntarioInc = 0x045A,
    /// Lumenetix, Inc (0x0459)
    LumenetixInc = 0x0459,
    /// Mini Solution Co., Ltd. (0x0458)
    MiniSolutionCoLtd = 0x0458,
    /// RF INNOVATION (0x0457)
    RfInnovation = 0x0457,
    /// Nemik Consulting Inc (0x0456)
    NemikConsultingInc = 0x0456,
    /// Atomation (0x0455)
    Atomation = 0x0455,
    /// Sphinx Electronics GmbH & Co KG (0x0454)
    SphinxElectronicsGmbHAndCoKg = 0x0454,
    /// Qorvo Utrecht B.V. (0x0453)
    QorvoUtrechtBV = 0x0453,
    /// Svep Design Center AB (0x0452)
    SvepDesignCenterAb = 0x0452,
    /// Tunstall Nordic AB (0x0451)
    TunstallNordicAb = 0x0451,
    /// Teenage Engineering AB (0x0450)
    TeenageEngineeringAb = 0x0450,
    /// TTS Tooltechnic Systems AG & Co. KG (0x044F)
    TtsTooltechnicSystemsAgAndCoKg = 0x044F,
    /// Xtrava Inc. (0x044E)
    XtravaInc = 0x044E,
    /// VEGA Grieshaber KG (0x044D)
    VegaGrieshaberKg = 0x044D,
    /// LifeStyle Lock, LLC (0x044C)
    LifeStyleLockLlc = 0x044C,
    /// Nain Inc. (0x044B)
    NainInc = 0x044B,
    /// SHIMANO INC. (0x044A)
    ShimanoInc = 0x044A,
    /// 1UP USA.com llc (0x0449)
    _1UpUsaComLlc = 0x0449,
    /// Grand Centrix GmbH (0x0448)
    GrandCentrixGmbH = 0x0448,
    /// Fabtronics Australia Pty Ltd (0x0447)
    FabtronicsAustraliaPtyLtd = 0x0447,
    /// NETGEAR, Inc. (0x0446)
    NetgearInc = 0x0446,
    /// Kobian Canada Inc. (0x0445)
    KobianCanadaInc = 0x0445,
    /// Metanate Limited (0x0444)
    MetanateLimited = 0x0444,
    /// Tucker International LLC (0x0443)
    TuckerInternationalLlc = 0x0443,
    /// SECOM CO., LTD. (0x0442)
    SecomCoLtd = 0x0442,
    /// iProtoXi Oy (0x0441)
    IProtoXiOy = 0x0441,
    /// Valencell, Inc. (0x0440)
    ValencellInc = 0x0440,
    /// Tentacle Sync GmbH (0x043F)
    TentacleSyncGmbH = 0x043F,
    /// Thermomedics, Inc. (0x043E)
    ThermomedicsInc = 0x043E,
    /// Coiler Corporation (0x043D)
    CoilerCorporation = 0x043D,
    /// DeLaval (0x043C)
    DeLaval = 0x043C,
    /// Appside co., ltd. (0x043B)
    AppsideCoLtd = 0x043B,
    /// Nuheara Limited (0x043A)
    NuhearaLimited = 0x043A,
    /// Radiance Technologies (0x0439)
    RadianceTechnologies = 0x0439,
    /// Helvar Ltd (0x0438)
    HelvarLtd = 0x0438,
    /// eBest IOT Inc. (0x0437)
    EBestIotInc = 0x0437,
    /// Drayson Technologies (Europe) Limited (0x0436)
    DraysonTechnologiesEuropeLimited = 0x0436,
    /// Blocks Wearables Ltd. (0x0435)
    BlocksWearablesLtd = 0x0435,
    /// Hatch Baby, Inc. (0x0434)
    HatchBabyInc = 0x0434,
    /// Pillsy Inc. (0x0433)
    PillsyInc = 0x0433,
    /// Silk Labs, Inc. (0x0432)
    SilkLabsInc = 0x0432,
    /// Alticor Inc. (0x0431)
    AlticorInc = 0x0431,
    /// SnapStyk Inc. (0x0430)
    SnapStykInc = 0x0430,
    /// Danfoss A/S (0x042F)
    DanfossAS = 0x042F,
    /// MemCachier Inc. (0x042E)
    MemCachierInc = 0x042E,
    /// Meshtech AS (0x042D)
    MeshtechAs = 0x042D,
    /// Ticto N.V. (0x042C)
    TictoNV = 0x042C,
    /// iMicroMed Incorporated (0x042B)
    IMicroMedIncorporated = 0x042B,
    /// BD Medical (0x042A)
    BdMedical = 0x042A,
    /// Prolon Inc. (0x0429)
    ProlonInc = 0x0429,
    /// SmallLoop, LLC (0x0428)
    SmallLoopLlc = 0x0428,
    /// Focus fleet and fuel management inc (0x0427)
    FocusFleetAndFuelManagementInc = 0x0427,
    /// Husqvarna AB (0x0426)
    HusqvarnaAb = 0x0426,
    /// Unify Software and Solutions GmbH & Co. KG (0x0425)
    UnifySoftwareAndSolutionsGmbHAndCoKg = 0x0425,
    /// Trainesense Ltd. (0x0424)
    TrainesenseLtd = 0x0424,
    /// Chargifi Limited (0x0423)
    ChargifiLimited = 0x0423,
    /// DELSEY SA (0x0422)
    DelseySa = 0x0422,
    /// Backbone Labs, Inc. (0x0421)
    BackboneLabsInc = 0x0421,
    /// TecBakery GmbH (0x0420)
    TecBakeryGmbH = 0x0420,
    /// Kopin Corporation (0x041F)
    KopinCorporation = 0x041F,
    /// Dell Computer Corporation (0x041E)
    DellComputerCorporation = 0x041E,
    /// Benning Elektrotechnik und Elektronik GmbH & Co. KG (0x041D)
    BenningElektrotechnikUndElektronikGmbHAndCoKg = 0x041D,
    /// WaterGuru, Inc. (0x041C)
    WaterGuruInc = 0x041C,
    /// OrthoAccel Technologies (0x041B)
    OrthoAccelTechnologies = 0x041B,
    /// Friday Labs Limited (0x041A)
    FridayLabsLimited = 0x041A,
    /// Novalogy LTD (0x0419)
    NovalogyLtd = 0x0419,
    /// Fatigue Science (0x0417)
    FatigueScience = 0x0417,
    /// SODA GmbH (0x0416)
    SodaGmbH = 0x0416,
    /// Uber Technologies Inc (0x0415)
    UberTechnologiesInc = 0x0415,
    /// Lightning Protection International Pty Ltd (0x0414)
    LightningProtectionInternationalPtyLtd = 0x0414,
    /// Wireless Cables Inc (0x0413)
    WirelessCablesInc = 0x0413,
    /// SEFAM (0x0412)
    Sefam = 0x0412,
    /// Luidia Inc (0x0411)
    LuidiaInc = 0x0411,
    /// Fender Musical Instruments (0x0410)
    FenderMusicalInstruments = 0x0410,
    /// CO-AX Technology, Inc. (0x040F)
    CoAxTechnologyInc = 0x040F,
    /// SKF (U.K.) Limited (0x040E)
    SkfUKLimited = 0x040E,
    /// NorthStar Battery Company, LLC (0x040D)
    NorthStarBatteryCompanyLlc = 0x040D,
    /// Senix Corporation (0x040C)
    SenixCorporation = 0x040C,
    /// Jana Care Inc. (0x040B)
    JanaCareInc = 0x040B,
    /// ZF OPENMATICS s.r.o. (0x040A)
    ZfOpenmaticsSRO = 0x040A,
    /// RYSE INC. (0x0409)
    RyseInc = 0x0409,
    /// ToGetHome Inc. (0x0408)
    ToGetHomeInc = 0x0408,
    /// Swiss Audio SA (0x0407)
    SwissAudioSa = 0x0407,
    /// Airtago (0x0406)
    Airtago = 0x0406,
    /// Vertex International, Inc. (0x0405)
    VertexInternationalInc = 0x0405,
    /// Authomate Inc (0x0404)
    AuthomateInc = 0x0404,
    /// Gantner Electronic GmbH (0x0403)
    GantnerElectronicGmbH = 0x0403,
    /// Sears Holdings Corporation (0x0402)
    SearsHoldingsCorporation = 0x0402,
    /// Relations Inc. (0x0401)
    RelationsInc = 0x0401,
    /// i-developer IT Beratung UG (0x0400)
    IDeveloperItBeratungUg = 0x0400,
    /// Withings (0x03FF)
    Withings = 0x03FF,
    /// Littelfuse (0x03FE)
    Littelfuse = 0x03FE,
    /// Trimble Inc. (0x03FD)
    TrimbleInc = 0x03FD,
    /// Kimberly-Clark (0x03FC)
    KimberlyClark = 0x03FC,
    /// Nox Medical (0x03FB)
    NoxMedical = 0x03FB,
    /// Vyassoft Technologies Inc (0x03FA)
    VyassoftTechnologiesInc = 0x03FA,
    /// Becon Technologies Co.,Ltd. (0x03F9)
    BeconTechnologiesCoLtd = 0x03F9,
    /// Rockford Corp. (0x03F8)
    RockfordCorp = 0x03F8,
    /// Owl Labs Inc. (0x03F7)
    OwlLabsInc = 0x03F7,
    /// Iton Technology Corp. (0x03F6)
    ItonTechnologyCorp = 0x03F6,
    /// WHERE, Inc. (0x03F5)
    WhereInc = 0x03F5,
    /// PAL Technologies Ltd (0x03F4)
    PalTechnologiesLtd = 0x03F4,
    /// Flowscape AB (0x03F3)
    FlowscapeAb = 0x03F3,
    /// WindowMaster A/S (0x03F2)
    WindowMasterAS = 0x03F2,
    /// Hestan Smart Cooking Inc. (0x03F1)
    HestanSmartCookingInc = 0x03F1,
    /// CLINK (0x03F0)
    Clink = 0x03F0,
    /// foolography GmbH (0x03EF)
    FoolographyGmbH = 0x03EF,
    /// CUBE TECHNOLOGIES (0x03EE)
    CubeTechnologies = 0x03EE,
    /// BASIC MICRO.COM,INC. (0x03ED)
    BasicMicroComInc = 0x03ED,
    /// Jigowatts Inc. (0x03EC)
    JigowattsInc = 0x03EC,
    /// Ozo Edu, Inc. (0x03EB)
    OzoEduInc = 0x03EB,
    /// Hello Inc. (0x03EA)
    HelloInc = 0x03EA,
    /// SHENZHEN LEMONJOY TECHNOLOGY CO., LTD. (0x03E9)
    ShenzhenLemonjoyTechnologyCoLtd = 0x03E9,
    /// Reiner Kartengeraete GmbH & Co. KG. (0x03E8)
    ReinerKartengeraeteGmbHAndCoKg = 0x03E8,
    /// TRUE Fitness Technology (0x03E7)
    TrueFitnessTechnology = 0x03E7,
    /// IoT Instruments Oy (0x03E6)
    IoTInstrumentsOy = 0x03E6,
    /// ffly4u (0x03E5)
    Ffly4U = 0x03E5,
    /// Chip-ing AG (0x03E4)
    ChipIngAg = 0x03E4,
    /// Qualcomm Life Inc (0x03E3)
    QualcommLifeInc = 0x03E3,
    /// Sensoan Oy (0x03E2)
    SensoanOy = 0x03E2,
    /// SPD Development Company Ltd (0x03E1)
    SpdDevelopmentCompanyLtd = 0x03E1,
    /// Actions Technology Co.,Ltd (0x03E0)
    ActionsTechnologyCoLtd = 0x03E0,
    /// Grob Technologies, LLC (0x03DF)
    GrobTechnologiesLlc = 0x03DF,
    /// Nathan Rhoades LLC (0x03DE)
    NathanRhoadesLlc = 0x03DE,
    /// Andreas Stihl AG & Co. KG (0x03DD)
    AndreasStihlAgAndCoKg = 0x03DD,
    /// Nima Labs (0x03DC)
    NimaLabs = 0x03DC,
    /// Instabeat, Inc (0x03DB)
    InstabeatInc = 0x03DB,
    /// EnOcean GmbH (0x03DA)
    EnOceanGmbH = 0x03DA,
    /// 3IWare Co., Ltd. (0x03D9)
    _3IWareCoLtd = 0x03D9,
    /// Zen-Me Labs Ltd (0x03D8)
    ZenMeLabsLtd = 0x03D8,
    /// FINSECUR (0x03D7)
    Finsecur = 0x03D7,
    /// Yota Devices LTD (0x03D6)
    YotaDevicesLtd = 0x03D6,
    /// Wyzelink Systems Inc. (0x03D5)
    WyzelinkSystemsInc = 0x03D5,
    /// PEG PEREGO SPA (0x03D4)
    PegPeregoSpa = 0x03D4,
    /// Sigma Connectivity AB (0x03D3)
    SigmaConnectivityAb = 0x03D3,
    /// IOT Pot India Private Limited (0x03D2)
    IotPotIndiaPrivateLimited = 0x03D2,
    /// Density Inc. (0x03D1)
    DensityInc = 0x03D1,
    /// Watteam Ltd (0x03D0)
    WatteamLtd = 0x03D0,
    /// MIRA, Inc. (0x03CF)
    MiraInc = 0x03CF,
    /// CONTRINEX S.A. (0x03CE)
    ContrinexSA = 0x03CE,
    /// Wynd Technologies, Inc. (0x03CD)
    WyndTechnologiesInc = 0x03CD,
    /// Vonkil Technologies Ltd (0x03CC)
    VonkilTechnologiesLtd = 0x03CC,
    /// SYSDEV Srl (0x03CB)
    SysdevSrl = 0x03CB,
    /// In2things Automation Pvt. Ltd. (0x03CA)
    In2ThingsAutomationPvtLtd = 0x03CA,
    /// Gallagher Group (0x03C9)
    GallagherGroup = 0x03C9,
    /// Avvel International (0x03C8)
    AvvelInternational = 0x03C8,
    /// Structural Health Systems, Inc. (0x03C7)
    StructuralHealthSystemsInc = 0x03C7,
    /// Intricon (0x03C6)
    Intricon = 0x03C6,
    /// St. Jude Medical, Inc. (0x03C5)
    StJudeMedicalInc = 0x03C5,
    /// Pico Technology Inc. (0x03C4)
    PicoTechnologyInc = 0x03C4,
    /// Casambi Technologies Oy (0x03C3)
    CasambiTechnologiesOy = 0x03C3,
    /// Snapchat Inc (0x03C2)
    SnapchatInc = 0x03C2,
    /// Ember Technologies, Inc. (0x03C1)
    EmberTechnologiesInc = 0x03C1,
    /// Arch Systems Inc. (0x03C0)
    ArchSystemsInc = 0x03C0,
    /// iLumi Solutions Inc. (0x03BF)
    ILumiSolutionsInc = 0x03BF,
    /// Applied Science, Inc. (0x03BE)
    AppliedScienceInc = 0x03BE,
    /// amadas (0x03BD)
    Amadas = 0x03BD,
    /// ASB Bank Ltd (0x03BC)
    AsbBankLtd = 0x03BC,
    /// Abbott (0x03BB)
    Abbott = 0x03BB,
    /// Maxscend Microelectronics Company Limited (0x03BA)
    MaxscendMicroelectronicsCompanyLimited = 0x03BA,
    /// FREDERIQUE CONSTANT SA (0x03B9)
    FrederiqueConstantSa = 0x03B9,
    /// A-Safe Limited (0x03B8)
    ASafeLimited = 0x03B8,
    /// Airbly Inc. (0x03B7)
    AirblyInc = 0x03B7,
    /// Mattel (0x03B6)
    Mattel = 0x03B6,
    /// petPOMM, Inc (0x03B5)
    PetPommInc = 0x03B5,
    /// Alpha Nodus, inc. (0x03B4)
    AlphaNodusInc = 0x03B4,
    /// Midwest Instruments & Controls (0x03B3)
    MidwestInstrumentsAndControls = 0x03B3,
    /// Propagation Systems Limited (0x03B2)
    PropagationSystemsLimited = 0x03B2,
    /// Otodata Wireless Network Inc. (0x03B1)
    OtodataWirelessNetworkInc = 0x03B1,
    /// VIBRADORM GmbH (0x03B0)
    VibradormGmbH = 0x03B0,
    /// Comm-N-Sense Corp DBA Verigo (0x03AF)
    CommNSenseCorpDbaVerigo = 0x03AF,
    /// Allswell Inc. (0x03AE)
    AllswellInc = 0x03AE,
    /// XiQ (0x03AD)
    XiQ = 0x03AD,
    /// Smablo LTD (0x03AC)
    SmabloLtd = 0x03AC,
    /// Meizu Technology Co., Ltd. (0x03AB)
    MeizuTechnologyCoLtd = 0x03AB,
    /// Exon Sp. z o.o. (0x03AA)
    ExonSpZOO = 0x03AA,
    /// THINKERLY SRL (0x03A9)
    ThinkerlySrl = 0x03A9,
    /// Esrille Inc. (0x03A8)
    EsrilleInc = 0x03A8,
    /// AeroScout (0x03A7)
    AeroScout = 0x03A7,
    /// Medela, Inc (0x03A6)
    MedelaInc = 0x03A6,
    /// ACE CAD Enterprise Co., Ltd. (ACECAD) (0x03A5)
    AceCadEnterpriseCoLtdAcecad = 0x03A5,
    /// Token Zero Ltd (0x03A4)
    TokenZeroLtd = 0x03A4,
    /// SmartMovt Technology Co., Ltd (0x03A3)
    SmartMovtTechnologyCoLtd = 0x03A3,
    /// Candura Instruments (0x03A2)
    CanduraInstruments = 0x03A2,
    /// Alpine Labs LLC (0x03A1)
    AlpineLabsLlc = 0x03A1,
    /// IVT Wireless Limited (0x03A0)
    IvtWirelessLimited = 0x03A0,
    /// Molex Corporation (0x039F)
    MolexCorporation = 0x039F,
    /// SchoolBoard Limited (0x039E)
    SchoolBoardLimited = 0x039E,
    /// CareView Communications, Inc. (0x039D)
    CareViewCommunicationsInc = 0x039D,
    /// ALE International (0x039C)
    AleInternational = 0x039C,
    /// South Silicon Valley Microelectronics (0x039B)
    SouthSiliconValleyMicroelectronics = 0x039B,
    /// NeST (0x039A)
    NeSt = 0x039A,
    /// Nikon Corporation (0x0399)
    NikonCorporation = 0x0399,
    /// Thetatronics Ltd (0x0398)
    ThetatronicsLtd = 0x0398,
    /// LEGO System A/S (0x0397)
    LegoSystemAS = 0x0397,
    /// BLOKS GmbH (0x0396)
    BloksGmbH = 0x0396,
    /// SDATAWAY (0x0395)
    Sdataway = 0x0395,
    /// Netclearance Systems, Inc. (0x0394)
    NetclearanceSystemsInc = 0x0394,
    /// LAVAZZA S.p.A. (0x0393)
    LavazzaSPA = 0x0393,
    /// T&D (0x0392)
    TAndD = 0x0392,
    /// Thingsquare AB (0x0391)
    ThingsquareAb = 0x0391,
    /// INFOTECH s.r.o. (0x0390)
    InfotechSRO = 0x0390,
    /// Xiaomi Inc. (0x038F)
    XiaomiInc = 0x038F,
    /// Crownstone B.V. (0x038E)
    CrownstoneBV = 0x038E,
    /// Resmed Ltd (0x038D)
    ResmedLtd = 0x038D,
    /// Appion Inc. (0x038C)
    AppionInc = 0x038C,
    /// Noke (0x038B)
    Noke = 0x038B,
    /// Kohler Mira Limited (0x038A)
    KohlerMiraLimited = 0x038A,
    /// ActiveBlu Corporation (0x0389)
    ActiveBluCorporation = 0x0389,
    /// Kapsch TrafficCom AB (0x0388)
    KapschTrafficComAb = 0x0388,
    /// BluStor PMC, Inc. (0x0387)
    BluStorPmcInc = 0x0387,
    /// Aterica Inc. (0x0386)
    AtericaInc = 0x0386,
    /// Embedded Electronic Solutions Ltd. dba e2Solutions (0x0385)
    EmbeddedElectronicSolutionsLtdDbaE2Solutions = 0x0385,
    /// OCOSMOS Co., Ltd. (0x0384)
    OcosmosCoLtd = 0x0384,
    /// Kronos Incorporated (0x0383)
    KronosIncorporated = 0x0383,
    /// Precision Outcomes Ltd (0x0382)
    PrecisionOutcomesLtd = 0x0382,
    /// Sharp Corporation (0x0381)
    SharpCorporation = 0x0381,
    /// LLC "MEGA-F service" (0x0380)
    LlcMegaFService = 0x0380,
    /// Société des Produits Nestlé S.A. (0x037F)
    SociétéDesProduitsNestléSA = 0x037F,
    /// lulabytes S.L. (0x037E)
    LulabytesSL = 0x037E,
    /// MICRODIA Ltd. (0x037D)
    MicrodiaLtd = 0x037D,
    /// Cronologics Corporation (0x037C)
    CronologicsCorporation = 0x037C,
    /// Apption Labs Inc. (0x037B)
    ApptionLabsInc = 0x037B,
    /// Algoria (0x037A)
    Algoria = 0x037A,
    /// Shenzhen iMCO Electronic Technology Co.,Ltd (0x0379)
    ShenzhenIMcoElectronicTechnologyCoLtd = 0x0379,
    /// Propeller Health (0x0378)
    PropellerHealth = 0x0378,
    /// Plejd AB (0x0377)
    PlejdAb = 0x0377,
    /// Electronic Temperature Instruments Ltd (0x0376)
    ElectronicTemperatureInstrumentsLtd = 0x0376,
    /// Expain AS (0x0375)
    ExpainAs = 0x0375,
    /// Holman Industries (0x0374)
    HolmanIndustries = 0x0374,
    /// AppNearMe Ltd (0x0373)
    AppNearMeLtd = 0x0373,
    /// Nixie Labs, Inc. (0x0372)
    NixieLabsInc = 0x0372,
    /// ORBCOMM (0x0371)
    Orbcomm = 0x0371,
    /// Wazombi Labs OÜ (0x0370)
    WazombiLabsOü = 0x0370,
    /// Motiv, Inc. (0x036F)
    MotivInc = 0x036F,
    /// MOTIVE TECHNOLOGIES, INC. (0x036E)
    MotiveTechnologiesInc = 0x036E,
    /// AirBolt Pty Ltd (0x036D)
    AirBoltPtyLtd = 0x036D,
    /// Zipcar (0x036C)
    Zipcar = 0x036C,
    /// BRControls Products BV (0x036B)
    BrControlsProductsBv = 0x036B,
    /// SetPoint Medical (0x036A)
    SetPointMedical = 0x036A,
    /// littleBits (0x0369)
    LittleBits = 0x0369,
    /// Metormote AB (0x0368)
    MetormoteAb = 0x0368,
    /// Saphe International (0x0367)
    SapheInternational = 0x0367,
    /// BOLTT Sports technologies Private limited (0x0366)
    BolttSportsTechnologiesPrivateLimited = 0x0366,
    /// BioMech Sensor LLC (0x0365)
    BioMechSensorLlc = 0x0365,
    /// Favero Electronics Srl (0x0364)
    FaveroElectronicsSrl = 0x0364,
    /// FREELAP SA (0x0363)
    FreelapSa = 0x0363,
    /// ON Semiconductor (0x0362)
    OnSemiconductor = 0x0362,
    /// Wellinks Inc. (0x0361)
    WellinksInc = 0x0361,
    /// Insulet Corporation (0x0360)
    InsuletCorporation = 0x0360,
    /// Acromag (0x035F)
    Acromag = 0x035F,
    /// Naya Health, Inc. (0x035E)
    NayaHealthInc = 0x035E,
    /// KYS (0x035D)
    Kys = 0x035D,
    /// Eaton Corporation (0x035C)
    EatonCorporation = 0x035C,
    /// Matrix Inc. (0x035B)
    MatrixInc = 0x035B,
    /// Phillips-Medisize A/S (0x035A)
    PhillipsMedisizeAS = 0x035A,
    /// Novotec Medical GmbH (0x0359)
    NovotecMedicalGmbH = 0x0359,
    /// MagniWare Ltd. (0x0358)
    MagniWareLtd = 0x0358,
    /// Polymap Wireless (0x0357)
    PolymapWireless = 0x0357,
    /// Spectrum Brands, Inc. (0x0356)
    SpectrumBrandsInc = 0x0356,
    /// Sigma Designs, Inc. (0x0355)
    SigmaDesignsInc = 0x0355,
    /// TOPPAN FORMS CO.,LTD. (0x0354)
    ToppanFormsCoLtd = 0x0354,
    /// Alpha Audiotronics, Inc. (0x0353)
    AlphaAudiotronicsInc = 0x0353,
    /// iRiding(Xiamen)Technology Co.,Ltd. (0x0352)
    IRidingXiamenTechnologyCoLtd = 0x0352,
    /// Pieps GmbH (0x0351)
    PiepsGmbH = 0x0351,
    /// Bitstrata Systems Inc. (0x0350)
    BitstrataSystemsInc = 0x0350,
    /// Heartland Payment Systems (0x034F)
    HeartlandPaymentSystems = 0x034F,
    /// SafeTrust Inc. (0x034E)
    SafeTrustInc = 0x034E,
    /// TASER International, Inc. (0x034D)
    TaserInternationalInc = 0x034D,
    /// HM Electronics, Inc. (0x034C)
    HmElectronicsInc = 0x034C,
    /// Libratone A/S (0x034B)
    LibratoneAS = 0x034B,
    /// Vaddio (0x034A)
    Vaddio = 0x034A,
    /// VersaMe (0x0349)
    VersaMe = 0x0349,
    /// Arioneo (0x0348)
    Arioneo = 0x0348,
    /// Prevent Biometrics (0x0347)
    PreventBiometrics = 0x0347,
    /// Acuity Brands Lighting, Inc (0x0346)
    AcuityBrandsLightingInc = 0x0346,
    /// Locus Positioning (0x0345)
    LocusPositioning = 0x0345,
    /// Whirl Inc (0x0344)
    WhirlInc = 0x0344,
    /// Drekker Development Pty. Ltd. (0x0343)
    DrekkerDevelopmentPtyLtd = 0x0343,
    /// GERTEC BRASIL LTDA. (0x0342)
    GertecBrasilLtda = 0x0342,
    /// Etesian Technologies LLC (0x0341)
    EtesianTechnologiesLlc = 0x0341,
    /// Letsense s.r.l. (0x0340)
    LetsenseSRL = 0x0340,
    /// Automation Components, Inc. (0x033F)
    AutomationComponentsInc = 0x033F,
    /// Monitra SA (0x033E)
    MonitraSa = 0x033E,
    /// TPV Technology Limited (0x033D)
    TpvTechnologyLimited = 0x033D,
    /// Virtuosys (0x033C)
    Virtuosys = 0x033C,
    /// Courtney Thorne Limited (0x033B)
    CourtneyThorneLimited = 0x033B,
    /// Appception, Inc. (0x033A)
    AppceptionInc = 0x033A,
    /// Blue Sky Scientific, LLC (0x0339)
    BlueSkyScientificLlc0339 = 0x0339,
    /// COBI GmbH (0x0338)
    CobiGmbH = 0x0338,
    /// AJP2 Holdings, LLC (0x0337)
    Ajp2HoldingsLlc = 0x0337,
    /// GISTIC (0x0336)
    Gistic = 0x0336,
    /// Enlighted Inc (0x0335)
    EnlightedInc = 0x0335,
    /// Airthings ASA (0x0334)
    AirthingsAsa = 0x0334,
    /// Mul-T-Lock (0x0333)
    MulTLock = 0x0333,
    /// Electrocompaniet A.S. (0x0332)
    ElectrocompanietAS = 0x0332,
    /// 3flares Technologies Inc. (0x0331)
    _3FlaresTechnologiesInc = 0x0331,
    /// North Pole Engineering (0x0330)
    NorthPoleEngineering = 0x0330,
    /// OttoQ Inc (0x032F)
    OttoQInc = 0x032F,
    /// indoormap (0x032E)
    Indoormap = 0x032E,
    /// BM innovations GmbH (0x032D)
    BmInnovationsGmbH = 0x032D,
    /// NIPPON SMT.CO.,Ltd (0x032C)
    NipponSmtCoLtd = 0x032C,
    /// ESYLUX (0x032B)
    Esylux = 0x032B,
    /// Electronic Design Lab (0x032A)
    ElectronicDesignLab = 0x032A,
    /// Eargo, Inc. (0x0329)
    EargoInc = 0x0329,
    /// Grundfos A/S (0x0328)
    GrundfosAS = 0x0328,
    /// Essex Electronics (0x0327)
    EssexElectronics = 0x0327,
    /// Healthwear Technologies (Changzhou)Ltd (0x0326)
    HealthwearTechnologiesChangzhouLtd = 0x0326,
    /// Amotus Solutions (0x0325)
    AmotusSolutions = 0x0325,
    /// Astro, Inc. (0x0324)
    AstroInc = 0x0324,
    /// Rotor Bike Components (0x0323)
    RotorBikeComponents = 0x0323,
    /// Compumedics Limited (0x0322)
    CompumedicsLimited = 0x0322,
    /// Jewelbots (0x0321)
    Jewelbots = 0x0321,
    /// SONO ELECTRONICS. CO., LTD (0x0320)
    SonoElectronicsCoLtd = 0x0320,
    /// MetaSystem S.p.A. (0x031F)
    MetaSystemSPA = 0x031F,
    /// Eyefi, Inc. (0x031E)
    EyefiInc = 0x031E,
    /// Enterlab ApS (0x031D)
    EnterlabApS = 0x031D,
    /// Lab Sensor Solutions (0x031C)
    LabSensorSolutions = 0x031C,
    /// HQ Inc (0x031B)
    HqInc = 0x031B,
    /// Wurth Elektronik eiSos GmbH & Co. KG (0x031A)
    WurthElektronikEiSosGmbHAndCoKg = 0x031A,
    /// Eugster Frismag AG (0x0319)
    EugsterFrismagAg = 0x0319,
    /// Aspenta International (0x0318)
    AspentaInternational = 0x0318,
    /// CHUO Electronics CO., LTD. (0x0317)
    ChuoElectronicsCoLtd = 0x0317,
    /// AG Measurematics Pvt. Ltd. (0x0316)
    AgMeasurematicsPvtLtd = 0x0316,
    /// Thermo Fisher Scientific (0x0315)
    ThermoFisherScientific = 0x0315,
    /// RIIG AI Sp. z o.o. (0x0314)
    RiigAiSpZOO = 0x0314,
    /// DiveNav, Inc. (0x0313)
    DiveNavInc = 0x0313,
    /// Ducere Technologies Pvt Ltd (0x0312)
    DucereTechnologiesPvtLtd = 0x0312,
    /// PEEQ DATA (0x0311)
    PeeqData = 0x0311,
    /// SGL Italia S.r.l. (0x0310)
    SglItaliaSRL = 0x0310,
    /// Shortcut Labs (0x030F)
    ShortcutLabs = 0x030F,
    /// Deviceworx (0x030E)
    Deviceworx = 0x030E,
    /// Devdata S.r.l. (0x030D)
    DevdataSRL = 0x030D,
    /// Hilti AG (0x030C)
    HiltiAg = 0x030C,
    /// Magnitude Lighting Converters (0x030B)
    MagnitudeLightingConverters = 0x030B,
    /// Ellisys (0x030A)
    Ellisys = 0x030A,
    /// Dolby Labs (0x0309)
    DolbyLabs = 0x0309,
    /// Surefire, LLC (0x0308)
    SurefireLlc = 0x0308,
    /// FUJI INDUSTRIAL CO.,LTD. (0x0307)
    FujiIndustrialCoLtd = 0x0307,
    /// Life Laboratory Inc. (0x0306)
    LifeLaboratoryInc = 0x0306,
    /// Swipp ApS (0x0305)
    SwippApS = 0x0305,
    /// Oura Health Ltd (0x0304)
    OuraHealthLtd = 0x0304,
    /// IACA electronique (0x0303)
    IacaElectronique = 0x0303,
    /// Loop Devices, Inc (0x0302)
    LoopDevicesInc = 0x0302,
    /// Giatec Scientific Inc. (0x0301)
    GiatecScientificInc = 0x0301,
    /// World Moto Inc. (0x0300)
    WorldMotoInc = 0x0300,
    /// Silicon Laboratories (0x02FF)
    SiliconLaboratories = 0x02FF,
    /// Lierda Science & Technology Group Co., Ltd. (0x02FE)
    LierdaScienceAndTechnologyGroupCoLtd = 0x02FE,
    /// Uwanna, Inc. (0x02FD)
    UwannaInc = 0x02FD,
    /// Shanghai Frequen Microelectronics Co., Ltd. (0x02FC)
    ShanghaiFrequenMicroelectronicsCoLtd = 0x02FC,
    /// Clarius Mobile Health Corp. (0x02FB)
    ClariusMobileHealthCorp = 0x02FB,
    /// CoSTAR TEchnologies (0x02FA)
    CoStarTEchnologies = 0x02FA,
    /// IMAGINATION TECHNOLOGIES LTD (0x02F9)
    ImaginationTechnologiesLtd = 0x02F9,
    /// Runteq Oy Ltd (0x02F8)
    RunteqOyLtd = 0x02F8,
    /// DreamVisions co., Ltd. (0x02F7)
    DreamVisionsCoLtd = 0x02F7,
    /// Intemo Technologies (0x02F6)
    IntemoTechnologies = 0x02F6,
    /// Indagem Tech LLC (0x02F5)
    IndagemTechLlc = 0x02F5,
    /// Vensi, Inc. (0x02F4)
    VensiInc = 0x02F4,
    /// AuthAir, Inc (0x02F3)
    AuthAirInc = 0x02F3,
    /// GoPro, Inc. (0x02F2)
    GoProInc = 0x02F2,
    /// The Idea Cave, LLC (0x02F1)
    TheIdeaCaveLlc = 0x02F1,
    /// Blackrat Software (0x02F0)
    BlackratSoftware = 0x02F0,
    /// SMART-INNOVATION.inc (0x02EF)
    SmartInnovationInc = 0x02EF,
    /// Citizen Holdings Co., Ltd. (0x02EE)
    CitizenHoldingsCoLtd = 0x02EE,
    /// HTC Corporation (0x02ED)
    HtcCorporation = 0x02ED,
    /// Delta Systems, Inc (0x02EC)
    DeltaSystemsInc = 0x02EC,
    /// Ardic Technology (0x02EB)
    ArdicTechnology = 0x02EB,
    /// Fujitsu Limited (0x02EA)
    FujitsuLimited = 0x02EA,
    /// Sensogram Technologies, Inc. (0x02E9)
    SensogramTechnologiesInc = 0x02E9,
    /// American Music Environments (0x02E8)
    AmericanMusicEnvironments = 0x02E8,
    /// Connected Yard, Inc. (0x02E7)
    ConnectedYardInc = 0x02E7,
    /// Unwire (0x02E6)
    Unwire = 0x02E6,
    /// Espressif Systems (Shanghai) Co., Ltd. (0x02E5)
    EspressifSystemsShanghaiCoLtd = 0x02E5,
    /// Bytestorm Ltd. (0x02E4)
    BytestormLtd = 0x02E4,
    /// Carmanah Technologies Corp. (0x02E3)
    CarmanahTechnologiesCorp = 0x02E3,
    /// NTT docomo (0x02E2)
    NttDocomo = 0x02E2,
    /// Victron Energy BV (0x02E1)
    VictronEnergyBv = 0x02E1,
    /// University of Michigan (0x02E0)
    UniversityOfMichigan = 0x02E0,
    /// Blur Product Development (0x02DF)
    BlurProductDevelopment = 0x02DF,
    /// Samsung SDS Co., Ltd. (0x02DE)
    SamsungSdsCoLtd = 0x02DE,
    /// Flint Rehabilitation Devices, LLC (0x02DD)
    FlintRehabilitationDevicesLlc = 0x02DD,
    /// DeWalch Technologies, Inc. (0x02DC)
    DeWalchTechnologiesInc = 0x02DC,
    /// Digi International Inc (R) (0x02DB)
    DigiInternationalIncR = 0x02DB,
    /// Gilvader (0x02DA)
    Gilvader = 0x02DA,
    /// Fliegl Agrartechnik GmbH (0x02D9)
    FlieglAgrartechnikGmbH = 0x02D9,
    /// Neosfar (0x02D8)
    Neosfar = 0x02D8,
    /// NIPPON SYSTEMWARE CO.,LTD. (0x02D7)
    NipponSystemwareCoLtd = 0x02D7,
    /// Send Solutions (0x02D6)
    SendSolutions = 0x02D6,
    /// OMRON Corporation (0x02D5)
    OmronCorporation = 0x02D5,
    /// Secuyou ApS (0x02D4)
    SecuyouApS = 0x02D4,
    /// Powercast Corporation (0x02D3)
    PowercastCorporation = 0x02D3,
    /// Afero, Inc. (0x02D2)
    AferoInc = 0x02D2,
    /// Empatica Srl (0x02D1)
    EmpaticaSrl = 0x02D1,
    /// 3M (0x02D0)
    _3M = 0x02D0,
    /// Anima (0x02CF)
    Anima = 0x02CF,
    /// Teva Branded Pharmaceutical Products R&D, Inc. (0x02CE)
    TevaBrandedPharmaceuticalProductsRAndDInc = 0x02CE,
    /// BMA ergonomics b.v. (0x02CD)
    BmaErgonomicsBV = 0x02CD,
    /// Eijkelkamp Soil & Water (0x02CC)
    EijkelkampSoilAndWater = 0x02CC,
    /// AINA-Wireless Inc. (0x02CB)
    AinaWirelessInc = 0x02CB,
    /// ABOV Semiconductor (0x02CA)
    AbovSemiconductor = 0x02CA,
    /// PayRange Inc. (0x02C9)
    PayRangeInc = 0x02C9,
    /// OneSpan (0x02C8)
    OneSpan = 0x02C8,
    /// Electronics Tomorrow Limited (0x02C7)
    ElectronicsTomorrowLimited = 0x02C7,
    /// Ayatan Sensors (0x02C6)
    AyatanSensors = 0x02C6,
    /// Lenovo (Singapore) Pte Ltd. (0x02C5)
    LenovoSingaporePteLtd = 0x02C5,
    /// Wilson Sporting Goods (0x02C4)
    WilsonSportingGoods = 0x02C4,
    /// Techtronic Power Tools Technology Limited (0x02C3)
    TechtronicPowerToolsTechnologyLimited = 0x02C3,
    /// Guillemot Corporation (0x02C2)
    GuillemotCorporation = 0x02C2,
    /// LINE Corporation (0x02C1)
    LineCorporation = 0x02C1,
    /// Dash Robotics (0x02C0)
    DashRobotics = 0x02C0,
    /// Redbird Flight Simulations (0x02BF)
    RedbirdFlightSimulations = 0x02BF,
    /// Seguro Technology Sp. z o.o. (0x02BE)
    SeguroTechnologySpZOO = 0x02BE,
    /// Chemtronics (0x02BD)
    Chemtronics = 0x02BD,
    /// Genevac Ltd (0x02BC)
    GenevacLtd = 0x02BC,
    /// Koha.,Co.Ltd (0x02BB)
    KohaCoLtd = 0x02BB,
    /// Swissprime Technologies AG (0x02BA)
    SwissprimeTechnologiesAg = 0x02BA,
    /// Rinnai Corporation (0x02B9)
    RinnaiCorporation = 0x02B9,
    /// Chrono Therapeutics (0x02B8)
    ChronoTherapeutics = 0x02B8,
    /// Oort Technologies LLC (0x02B7)
    OortTechnologiesLlc = 0x02B7,
    /// Schneider Electric (0x02B6)
    SchneiderElectric = 0x02B6,
    /// HANSHIN ELECTRIC RAILWAY CO.,LTD. (0x02B5)
    HanshinElectricRailwayCoLtd = 0x02B5,
    /// Hyginex, Inc. (0x02B4)
    HyginexInc = 0x02B4,
    /// CLABER S.P.A. (0x02B3)
    ClaberSPA = 0x02B3,
    /// Oura Health Oy (0x02B2)
    OuraHealthOy = 0x02B2,
    /// Raden Inc (0x02B1)
    RadenInc = 0x02B1,
    /// Bestechnic(Shanghai),Ltd (0x02B0)
    BestechnicShanghaiLtd = 0x02B0,
    /// Technicolor USA Inc. (0x02AF)
    TechnicolorUsaInc = 0x02AF,
    /// WeatherFlow, Inc. (0x02AE)
    WeatherFlowInc = 0x02AE,
    /// Rx Networks, Inc. (0x02AD)
    RxNetworksInc = 0x02AD,
    /// RTB Elektronik GmbH & Co. KG (0x02AC)
    RtbElektronikGmbHAndCoKg = 0x02AC,
    /// BBPOS Limited (0x02AB)
    BbposLimited = 0x02AB,
    /// Doppler Lab (0x02AA)
    DopplerLab = 0x02AA,
    /// Chargelib (0x02A9)
    Chargelib = 0x02A9,
    /// miSport Ltd. (0x02A8)
    MiSportLtd = 0x02A8,
    /// Illuxtron international B.V. (0x02A7)
    IlluxtronInternationalBV = 0x02A7,
    /// Robert Bosch GmbH (0x02A6)
    RobertBoschGmbH = 0x02A6,
    /// Tendyron Corporation (0x02A5)
    TendyronCorporation = 0x02A5,
    /// Pacific Lock Company (0x02A4)
    PacificLockCompany = 0x02A4,
    /// Itude (0x02A3)
    Itude = 0x02A3,
    /// Sera4 Ltd. (0x02A2)
    Sera4Ltd = 0x02A2,
    /// InventureTrack Systems (0x02A1)
    InventureTrackSystems = 0x02A1,
    /// Impossible Camera GmbH (0x02A0)
    ImpossibleCameraGmbH = 0x02A0,
    /// Areus Engineering GmbH (0x029F)
    AreusEngineeringGmbH = 0x029F,
    /// Kupson spol. s r.o. (0x029E)
    KupsonSpolSRO = 0x029E,
    /// ALOTTAZS LABS, LLC (0x029D)
    AlottazsLabsLlc = 0x029D,
    /// Blue Sky Scientific, LLC (0x029C)
    BlueSkyScientificLlc029C = 0x029C,
    /// C2 Development, Inc. (0x029B)
    C2DevelopmentInc = 0x029B,
    /// Currant, Inc. (0x029A)
    CurrantInc = 0x029A,
    /// Inexess Technology Simma KG (0x0299)
    InexessTechnologySimmaKg = 0x0299,
    /// EISST Ltd (0x0298)
    EisstLtd = 0x0298,
    /// storm power ltd (0x0297)
    StormPowerLtd = 0x0297,
    /// Petzl (0x0296)
    Petzl = 0x0296,
    /// Sivantos GmbH (0x0295)
    SivantosGmbH = 0x0295,
    /// ELIAS GmbH (0x0294)
    EliasGmbH = 0x0294,
    /// Blue Bite (0x0293)
    BlueBite = 0x0293,
    /// SwiftSensors (0x0292)
    SwiftSensors = 0x0292,
    /// CliniCloud Inc (0x0291)
    CliniCloudInc = 0x0291,
    /// Multibit Oy (0x0290)
    MultibitOy = 0x0290,
    /// Church & Dwight Co., Inc (0x028F)
    ChurchAndDwightCoInc = 0x028F,
    /// RF Digital Corp (0x028E)
    RfDigitalCorp = 0x028E,
    /// IF, LLC (0x028D)
    IfLlc = 0x028D,
    /// NANOLINK APS (0x028C)
    NanolinkAps = 0x028C,
    /// Code Gears LTD (0x028B)
    CodeGearsLtd = 0x028B,
    /// Jetro AS (0x028A)
    JetroAs = 0x028A,
    /// SK Telecom (0x0289)
    SkTelecom = 0x0289,
    /// Willowbank Electronics Ltd (0x0288)
    WillowbankElectronicsLtd = 0x0288,
    /// Wally Ventures S.L. (0x0287)
    WallyVenturesSL = 0x0287,
    /// RF Code, Inc. (0x0286)
    RfCodeInc = 0x0286,
    /// WOWTech Canada Ltd. (0x0285)
    WowTechCanadaLtd = 0x0285,
    /// Synapse Electronics (0x0284)
    SynapseElectronics = 0x0284,
    /// Maven Machines, Inc. (0x0283)
    MavenMachinesInc = 0x0283,
    /// Sonova AG (0x0282)
    SonovaAg = 0x0282,
    /// StoneL (0x0281)
    StoneL = 0x0281,
    /// ITEC corporation (0x0280)
    ItecCorporation = 0x0280,
    /// ruwido austria gmbh (0x027F)
    RuwidoAustriaGmbh = 0x027F,
    /// HabitAware, LLC (0x027E)
    HabitAwareLlc = 0x027E,
    /// HUAWEI Technologies Co., Ltd. (0x027D)
    HuaweiTechnologiesCoLtd = 0x027D,
    /// Aseptika Ltd (0x027C)
    AseptikaLtd = 0x027C,
    /// DEFA AS (0x027B)
    DefaAs = 0x027B,
    /// Ekomini inc. (0x027A)
    EkominiInc = 0x027A,
    /// steute Schaltgerate GmbH & Co. KG (0x0279)
    SteuteSchaltgerateGmbHAndCoKg = 0x0279,
    /// Johnson Outdoors Inc (0x0278)
    JohnsonOutdoorsInc = 0x0278,
    /// bewhere inc (0x0277)
    BewhereInc = 0x0277,
    /// E.G.O. Elektro-Geraetebau GmbH (0x0276)
    EGOElektroGeraetebauGmbH = 0x0276,
    /// Geotab (0x0275)
    Geotab = 0x0275,
    /// Motsai Research (0x0274)
    MotsaiResearch = 0x0274,
    /// OCEASOFT (0x0273)
    Oceasoft = 0x0273,
    /// Alps Alpine Co., Ltd. (0x0272)
    AlpsAlpineCoLtd = 0x0272,
    /// Animas Corp (0x0271)
    AnimasCorp = 0x0271,
    /// LSI ADL Technology (0x0270)
    LsiAdlTechnology = 0x0270,
    /// Aptcode Solutions (0x026F)
    AptcodeSolutions = 0x026F,
    /// FLEURBAEY BVBA (0x026E)
    FleurbaeyBvba = 0x026E,
    /// Technogym SPA (0x026D)
    TechnogymSpa = 0x026D,
    /// Domster Tadeusz Szydlowski (0x026C)
    DomsterTadeuszSzydlowski = 0x026C,
    /// DEKA Research & Development Corp. (0x026B)
    DekaResearchAndDevelopmentCorp = 0x026B,
    /// Gemalto (0x026A)
    Gemalto = 0x026A,
    /// Torrox GmbH & Co KG (0x0269)
    TorroxGmbHAndCoKg = 0x0269,
    /// Cerevo (0x0268)
    Cerevo = 0x0268,
    /// XMI Systems SA (0x0267)
    XmiSystemsSa = 0x0267,
    /// Schawbel Technologies LLC (0x0266)
    SchawbelTechnologiesLlc = 0x0266,
    /// SMK Corporation (0x0265)
    SmkCorporation = 0x0265,
    /// DDS, Inc. (0x0264)
    DdsInc = 0x0264,
    /// Identiv, Inc. (0x0263)
    IdentivInc = 0x0263,
    /// Glacial Ridge Technologies (0x0262)
    GlacialRidgeTechnologies = 0x0262,
    /// SECVRE GmbH (0x0261)
    SecvreGmbH = 0x0261,
    /// SensaRx (0x0260)
    SensaRx = 0x0260,
    /// Yardarm Technologies (0x025F)
    YardarmTechnologies = 0x025F,
    /// Fluke Corporation (0x025E)
    FlukeCorporation = 0x025E,
    /// Lexmark International Inc. (0x025D)
    LexmarkInternationalInc = 0x025D,
    /// NetEase（Hangzhou）Network co.Ltd. (0x025C)
    NetEaseHangzhouNetworkCoLtd = 0x025C,
    /// Five Interactive, LLC dba Zendo (0x025B)
    FiveInteractiveLlcDbaZendo = 0x025B,
    /// University of Applied Sciences Valais/Haute Ecole Valaisanne (0x025A)
    UniversityOfAppliedSciencesValaisHauteEcoleValaisanne = 0x025A,
    /// ALTYOR (0x0259)
    Altyor = 0x0259,
    /// Devialet SA (0x0258)
    DevialetSa = 0x0258,
    /// AdBabble Local Commerce Inc. (0x0257)
    AdBabbleLocalCommerceInc = 0x0257,
    /// G24 Power Limited (0x0256)
    G24PowerLimited = 0x0256,
    /// Dai Nippon Printing Co., Ltd. (0x0255)
    DaiNipponPrintingCoLtd = 0x0255,
    /// Playbrush (0x0254)
    Playbrush = 0x0254,
    /// Xicato Inc. (0x0253)
    XicatoInc = 0x0253,
    /// UKC Technosolution (0x0252)
    UkcTechnosolution = 0x0252,
    /// Lumo Bodytech Inc. (0x0251)
    LumoBodytechInc = 0x0251,
    /// Sapphire Circuits LLC (0x0250)
    SapphireCircuitsLlc = 0x0250,
    /// Schneider Schreibgeräte GmbH (0x024F)
    SchneiderSchreibgeräteGmbH = 0x024F,
    /// Microtronics Engineering GmbH (0x024E)
    MicrotronicsEngineeringGmbH = 0x024E,
    /// M-Way Solutions GmbH (0x024D)
    MWaySolutionsGmbH = 0x024D,
    /// Blue Clover Devices (0x024C)
    BlueCloverDevices = 0x024C,
    /// Orlan LLC (0x024B)
    OrlanLlc = 0x024B,
    /// Uwatec AG (0x024A)
    UwatecAg = 0x024A,
    /// Transcranial Ltd (0x0249)
    TranscranialLtd = 0x0249,
    /// Parker Hannifin Corp (0x0248)
    ParkerHannifinCorp = 0x0248,
    /// FiftyThree Inc. (0x0247)
    FiftyThreeInc = 0x0247,
    /// ACKme Networks, Inc. (0x0246)
    AcKmeNetworksInc = 0x0246,
    /// Endress+Hauser (0x0245)
    EndressHauser = 0x0245,
    /// Iotera Inc (0x0244)
    IoteraInc = 0x0244,
    /// Masimo Corp (0x0243)
    MasimoCorp = 0x0243,
    /// 16Lab Inc (0x0242)
    _16LabInc = 0x0242,
    /// Bragi GmbH (0x0241)
    BragiGmbH = 0x0241,
    /// Argenox Technologies (0x0240)
    ArgenoxTechnologies = 0x0240,
    /// WaveWare Technologies Inc. (0x023F)
    WaveWareTechnologiesInc = 0x023F,
    /// Raven Industries (0x023E)
    RavenIndustries = 0x023E,
    /// ViCentra B.V. (0x023D)
    ViCentraBV = 0x023D,
    /// Awarepoint (0x023C)
    Awarepoint = 0x023C,
    /// Beijing CarePulse Electronic Technology Co, Ltd (0x023B)
    BeijingCarePulseElectronicTechnologyCoLtd = 0x023B,
    /// Alatech Tehnology (0x023A)
    AlatechTehnology = 0x023A,
    /// JIN CO, Ltd (0x0239)
    JinCoLtd = 0x0239,
    /// Trakm8 Ltd (0x0238)
    Trakm8Ltd = 0x0238,
    /// MSHeli s.r.l. (0x0237)
    MsHeliSRL = 0x0237,
    /// Pitpatpet Ltd (0x0236)
    PitpatpetLtd = 0x0236,
    /// Qrio Inc (0x0235)
    QrioInc = 0x0235,
    /// FengFan (BeiJing) Technology Co, Ltd (0x0234)
    FengFanBeiJingTechnologyCoLtd = 0x0234,
    /// Shenzhen SuLong Communication Ltd (0x0233)
    ShenzhenSuLongCommunicationLtd = 0x0233,
    /// x-Senso Solutions Kft (0x0232)
    XSensoSolutionsKft = 0x0232,
    /// ETA SA (0x0231)
    EtaSa = 0x0231,
    /// Foster Electric Company, Ltd (0x0230)
    FosterElectricCompanyLtd = 0x0230,
    /// Huami (Shanghai) Culture Communication CO., LTD (0x022F)
    HuamiShanghaiCultureCommunicationCoLtd = 0x022F,
    /// Siemens AG (0x022E)
    SiemensAg = 0x022E,
    /// Lupine (0x022D)
    Lupine = 0x022D,
    /// Pharynks Corporation (0x022C)
    PharynksCorporation = 0x022C,
    /// Tesla, Inc. (0x022B)
    TeslaInc = 0x022B,
    /// Stamer Musikanlagen GMBH (0x022A)
    StamerMusikanlagenGmbh = 0x022A,
    /// Muoverti Limited (0x0229)
    MuovertiLimited = 0x0229,
    /// Twocanoes Labs, LLC (0x0228)
    TwocanoesLabsLlc = 0x0228,
    /// LifeBEAM Technologies (0x0227)
    LifeBeamTechnologies = 0x0227,
    /// Merlinia A/S (0x0226)
    MerliniaAS = 0x0226,
    /// Nestlé Nespresso S.A. (0x0225)
    NestléNespressoSA = 0x0225,
    /// Comarch SA (0x0224)
    ComarchSa = 0x0224,
    /// Philip Morris Products S.A. (0x0223)
    PhilipMorrisProductsSA = 0x0223,
    /// Praxis Dynamics (0x0222)
    PraxisDynamics = 0x0222,
    /// Mobiquity Networks Inc (0x0221)
    MobiquityNetworksInc = 0x0221,
    /// Manus Machina BV (0x0220)
    ManusMachinaBv = 0x0220,
    /// Luster Leaf Products  Inc (0x021F)
    LusterLeafProductsInc = 0x021F,
    /// Goodnet, Ltd (0x021E)
    GoodnetLtd = 0x021E,
    /// Edamic (0x021D)
    Edamic = 0x021D,
    /// Mobicomm Inc (0x021C)
    MobicommInc = 0x021C,
    /// Cisco Systems, Inc (0x021B)
    CiscoSystemsInc = 0x021B,
    /// Blue Speck Labs, LLC (0x021A)
    BlueSpeckLabsLlc = 0x021A,
    /// DOTT Limited (0x0219)
    DottLimited = 0x0219,
    /// Hiotech AB (0x0218)
    HiotechAb = 0x0218,
    /// Tech4home, Lda (0x0217)
    Tech4HomeLda = 0x0217,
    /// MTI Ltd (0x0216)
    MtiLtd = 0x0216,
    /// Lukoton Experience Oy (0x0215)
    LukotonExperienceOy = 0x0215,
    /// IK Multimedia Production srl (0x0214)
    IkMultimediaProductionSrl = 0x0214,
    /// Wyler AG (0x0213)
    WylerAg = 0x0213,
    /// Interplan Co., Ltd (0x0212)
    InterplanCoLtd = 0x0212,
    /// Telink Semiconductor Co. Ltd (0x0211)
    TelinkSemiconductorCoLtd = 0x0211,
    /// ikeGPS (0x0210)
    IkeGps = 0x0210,
    /// Comodule GMBH (0x020F)
    ComoduleGmbh = 0x020F,
    /// Omron Healthcare Co., LTD (0x020E)
    OmronHealthcareCoLtd = 0x020E,
    /// Simplo Technology Co., LTD (0x020D)
    SimploTechnologyCoLtd = 0x020D,
    /// CoroWare Technologies, Inc (0x020C)
    CoroWareTechnologiesInc = 0x020C,
    /// Jaguar Land Rover Limited (0x020B)
    JaguarLandRoverLimited = 0x020B,
    /// Macnica Inc. (0x020A)
    MacnicaInc = 0x020A,
    /// InvisionHeart Inc. (0x0209)
    InvisionHeartInc = 0x0209,
    /// LumiGeek LLC (0x0208)
    LumiGeekLlc = 0x0208,
    /// STEMP Inc. (0x0207)
    StempInc = 0x0207,
    /// Otter Products, LLC (0x0206)
    OtterProductsLlc = 0x0206,
    /// Smartbotics Inc. (0x0205)
    SmartboticsInc = 0x0205,
    /// Tapcentive Inc. (0x0204)
    TapcentiveInc = 0x0204,
    /// Kemppi Oy (0x0203)
    KemppiOy = 0x0203,
    /// Rigado LLC (0x0202)
    RigadoLlc = 0x0202,
    /// AR Timing (0x0201)
    ArTiming = 0x0201,
    /// Verifone Systems Pte Ltd. Taiwan Branch (0x0200)
    VerifoneSystemsPteLtdTaiwanBranch = 0x0200,
    /// Freescale Semiconductor, Inc. (0x01FF)
    FreescaleSemiconductorInc = 0x01FF,
    /// Radio Systems Corporation (0x01FE)
    RadioSystemsCorporation = 0x01FE,
    /// Kontakt Micro-Location Sp. z o.o. (0x01FD)
    KontaktMicroLocationSpZOO = 0x01FD,
    /// Wahoo Fitness, LLC (0x01FC)
    WahooFitnessLlc = 0x01FC,
    /// Form Lifting, LLC (0x01FB)
    FormLiftingLlc = 0x01FB,
    /// Gozio Inc. (0x01FA)
    GozioInc = 0x01FA,
    /// Medtronic Inc. (0x01F9)
    MedtronicInc = 0x01F9,
    /// Anyka (Guangzhou) Microelectronics Technology Co, LTD (0x01F8)
    AnykaGuangzhouMicroelectronicsTechnologyCoLtd = 0x01F8,
    /// Gelliner Limited (0x01F7)
    GellinerLimited = 0x01F7,
    /// DJO Global (0x01F6)
    DjoGlobal = 0x01F6,
    /// Cool Webthings Limited (0x01F5)
    CoolWebthingsLimited = 0x01F5,
    /// UTC Fire and Security (0x01F4)
    UtcFireAndSecurity = 0x01F4,
    /// The University of Tokyo (0x01F3)
    TheUniversityOfTokyo = 0x01F3,
    /// Itron, Inc. (0x01F2)
    ItronInc = 0x01F2,
    /// Zebra Technologies Corporation (0x01F1)
    ZebraTechnologiesCorporation = 0x01F1,
    /// KloudNation (0x01F0)
    KloudNation = 0x01F0,
    /// Fullpower Technologies, Inc. (0x01EF)
    FullpowerTechnologiesInc = 0x01EF,
    /// Valeo Service (0x01EE)
    ValeoService = 0x01EE,
    /// CuteCircuit LTD (0x01ED)
    CuteCircuitLtd = 0x01ED,
    /// Spreadtrum Communications Shanghai Ltd (0x01EC)
    SpreadtrumCommunicationsShanghaiLtd = 0x01EC,
    /// AutoMap LLC (0x01EB)
    AutoMapLlc = 0x01EB,
    /// Advanced Application Design, Inc. (0x01EA)
    AdvancedApplicationDesignInc = 0x01EA,
    /// Sano, Inc. (0x01E9)
    SanoInc = 0x01E9,
    /// STIR (0x01E8)
    Stir = 0x01E8,
    /// IPS Group Inc. (0x01E7)
    IpsGroupInc = 0x01E7,
    /// Technology Solutions (UK) Ltd (0x01E6)
    TechnologySolutionsUkLtd = 0x01E6,
    /// Dynamic Devices Ltd (0x01E5)
    DynamicDevicesLtd = 0x01E5,
    /// Freedom Innovations (0x01E4)
    FreedomInnovations = 0x01E4,
    /// Caterpillar Inc (0x01E3)
    CaterpillarInc = 0x01E3,
    /// Lectronix, Inc. (0x01E2)
    LectronixInc = 0x01E2,
    /// Jolla Ltd (0x01E1)
    JollaLtd = 0x01E1,
    /// Widex A/S (0x01E0)
    WidexAS = 0x01E0,
    /// Bison Group Ltd. (0x01DF)
    BisonGroupLtd = 0x01DF,
    /// Minelab Electronics Pty Limited (0x01DE)
    MinelabElectronicsPtyLimited = 0x01DE,
    /// Koninklijke Philips N.V. (0x01DD)
    KoninklijkePhilipsNV = 0x01DD,
    /// iParking Ltd. (0x01DC)
    IParkingLtd = 0x01DC,
    /// Innblue Consulting (0x01DB)
    InnblueConsulting = 0x01DB,
    /// Logitech International SA (0x01DA)
    LogitechInternationalSa = 0x01DA,
    /// Savant Systems LLC (0x01D9)
    SavantSystemsLlc = 0x01D9,
    /// Code Corporation (0x01D8)
    CodeCorporation = 0x01D8,
    /// Squadrone Systems Inc. (0x01D7)
    SquadroneSystemsInc = 0x01D7,
    /// G-wearables inc. (0x01D6)
    GWearablesInc = 0x01D6,
    /// ELAD srl (0x01D5)
    EladSrl = 0x01D5,
    /// Newlab S.r.l. (0x01D4)
    NewlabSRL = 0x01D4,
    /// Sky Wave Design (0x01D3)
    SkyWaveDesign = 0x01D3,
    /// Gill Electronics (0x01D2)
    GillElectronics = 0x01D2,
    /// August Home, Inc (0x01D1)
    AugustHomeInc = 0x01D1,
    /// Primus Inter Pares Ltd (0x01D0)
    PrimusInterParesLtd = 0x01D0,
    /// BSH (0x01CF)
    Bsh = 0x01CF,
    /// HOUWA SYSTEM DESIGN, k.k. (0x01CE)
    HouwaSystemDesignKK = 0x01CE,
    /// Chengdu Synwing Technology Ltd (0x01CD)
    ChengduSynwingTechnologyLtd = 0x01CD,
    /// Sam Labs Ltd. (0x01CC)
    SamLabsLtd = 0x01CC,
    /// Fetch My Pet (0x01CB)
    FetchMyPet = 0x01CB,
    /// Laerdal Medical AS (0x01CA)
    LaerdalMedicalAs = 0x01CA,
    /// Avi-on (0x01C9)
    AviOn = 0x01C9,
    /// Poly-Control ApS (0x01C8)
    PolyControlApS = 0x01C8,
    /// Abiogenix Inc. (0x01C7)
    AbiogenixInc = 0x01C7,
    /// HASWARE Inc. (0x01C6)
    HaswareInc = 0x01C6,
    /// Bitcraze AB (0x01C5)
    BitcrazeAb = 0x01C5,
    /// DME Microelectronics (0x01C4)
    DmeMicroelectronics = 0x01C4,
    /// Bunch (0x01C3)
    Bunch = 0x01C3,
    /// Transenergooil AG (0x01C2)
    TransenergooilAg = 0x01C2,
    /// BRADATECH Corp. (0x01C1)
    BradatechCorp = 0x01C1,
    /// pironex GmbH (0x01C0)
    PironexGmbH = 0x01C0,
    /// Hongkong OnMicro Electronics Limited (0x01BF)
    HongkongOnMicroElectronicsLimited = 0x01BF,
    /// Pulsate Mobile Ltd. (0x01BE)
    PulsateMobileLtd = 0x01BE,
    /// Syszone Co., Ltd (0x01BD)
    SyszoneCoLtd = 0x01BD,
    /// SenionLab AB (0x01BC)
    SenionLabAb = 0x01BC,
    /// Cochlear Bone Anchored Solutions AB (0x01BB)
    CochlearBoneAnchoredSolutionsAb = 0x01BB,
    /// Stages Cycling LLC (0x01BA)
    StagesCyclingLlc = 0x01BA,
    /// HANA Micron (0x01B9)
    HanaMicron = 0x01B9,
    /// i+D3 S.L. (0x01B8)
    ID3SL = 0x01B8,
    /// General Electric Company (0x01B7)
    GeneralElectricCompany = 0x01B7,
    /// LM Technologies Ltd (0x01B6)
    LmTechnologiesLtd = 0x01B6,
    /// Nest Labs Inc. (0x01B5)
    NestLabsInc = 0x01B5,
    /// Trineo Sp. z o.o. (0x01B4)
    TrineoSpZOO = 0x01B4,
    /// Nytec, Inc. (0x01B3)
    NytecInc = 0x01B3,
    /// Nymi Inc. (0x01B2)
    NymiInc = 0x01B2,
    /// Netizens Sp. z o.o. (0x01B1)
    NetizensSpZOO = 0x01B1,
    /// Star Micronics Co., Ltd. (0x01B0)
    StarMicronicsCoLtd = 0x01B0,
    /// Sunrise Micro Devices, Inc. (0x01AF)
    SunriseMicroDevicesInc = 0x01AF,
    /// Earlens Corporation (0x01AE)
    EarlensCorporation = 0x01AE,
    /// FlightSafety International (0x01AD)
    FlightSafetyInternational = 0x01AD,
    /// Trividia Health, Inc. (0x01AC)
    TrividiaHealthInc = 0x01AC,
    /// Meta Platforms, Inc. (0x01AB)
    MetaPlatformsInc = 0x01AB,
    /// Geophysical Technology Inc. (0x01AA)
    GeophysicalTechnologyInc = 0x01AA,
    /// Canon Inc. (0x01A9)
    CanonInc = 0x01A9,
    /// Taobao (0x01A8)
    Taobao = 0x01A8,
    /// ENERGOUS CORPORATION (0x01A7)
    EnergousCorporation = 0x01A7,
    /// Wille Engineering (0x01A6)
    WilleEngineering = 0x01A6,
    /// Icon Health and Fitness (0x01A5)
    IconHealthAndFitness = 0x01A5,
    /// MSA Innovation, LLC (0x01A4)
    MsaInnovationLlc = 0x01A4,
    /// EROAD (0x01A3)
    Eroad = 0x01A3,
    /// GIGALANE.CO.,LTD (0x01A2)
    GigalaneCoLtd = 0x01A2,
    /// FIAMM (0x01A1)
    Fiamm = 0x01A1,
    /// Channel Enterprises (HK) Ltd. (0x01A0)
    ChannelEnterprisesHkLtd = 0x01A0,
    /// Strainstall Ltd (0x019F)
    StrainstallLtd = 0x019F,
    /// Ceruus (0x019E)
    Ceruus = 0x019E,
    /// CVS Health (0x019D)
    CvsHealth = 0x019D,
    /// Cokiya Incorporated (0x019C)
    CokiyaIncorporated = 0x019C,
    /// CUBETECH s.r.o. (0x019B)
    CubetechSRO = 0x019B,
    /// TRON Forum (0x019A)
    TronForum = 0x019A,
    /// SALTO SYSTEMS S.L. (0x0199)
    SaltoSystemsSL = 0x0199,
    /// VENGIT Korlatolt Felelossegu Tarsasag (0x0198)
    VengitKorlatoltFelelosseguTarsasag = 0x0198,
    /// WiSilica Inc. (0x0197)
    WiSilicaInc = 0x0197,
    /// Paxton Access Ltd (0x0196)
    PaxtonAccessLtd = 0x0196,
    /// Zuli (0x0195)
    Zuli = 0x0195,
    /// Acoustic Stream Corporation (0x0194)
    AcousticStreamCorporation = 0x0194,
    /// Maveric Automation LLC (0x0193)
    MavericAutomationLlc = 0x0193,
    /// Cloudleaf, Inc (0x0192)
    CloudleafInc = 0x0192,
    /// FDK CORPORATION (0x0191)
    FdkCorporation = 0x0191,
    /// Intelletto Technologies Inc. (0x0190)
    IntellettoTechnologiesInc = 0x0190,
    /// Fireflies Systems (0x018F)
    FirefliesSystems = 0x018F,
    /// Google LLC (0x018E)
    GoogleLlc = 0x018E,
    /// Extron Design Services (0x018D)
    ExtronDesignServices = 0x018D,
    /// Wilo SE (0x018C)
    WiloSe = 0x018C,
    /// Konica Minolta, Inc. (0x018B)
    KonicaMinoltaInc = 0x018B,
    /// Able Trend Technology Limited (0x018A)
    AbleTrendTechnologyLimited = 0x018A,
    /// Physical Enterprises Inc. (0x0189)
    PhysicalEnterprisesInc = 0x0189,
    /// Unico RBC (0x0188)
    UnicoRbc = 0x0188,
    /// Seraphim Sense Ltd (0x0187)
    SeraphimSenseLtd = 0x0187,
    /// CORE Lighting Ltd (0x0186)
    CoreLightingLtd = 0x0186,
    /// bel'apps LLC (0x0185)
    BelAppsLlc = 0x0185,
    /// Nectar (0x0184)
    Nectar = 0x0184,
    /// Walt Disney (0x0183)
    WaltDisney = 0x0183,
    /// HOP Ubiquitous (0x0182)
    HopUbiquitous = 0x0182,
    /// Gecko Health Innovations, Inc. (0x0181)
    GeckoHealthInnovationsInc = 0x0181,
    /// Gigaset Technologies GmbH (0x0180)
    GigasetTechnologiesGmbH = 0x0180,
    /// XTel Wireless ApS (0x017F)
    XTelWirelessApS = 0x017F,
    /// BluDotz Ltd (0x017E)
    BluDotzLtd = 0x017E,
    /// BatAndCat (0x017D)
    BatAndCat = 0x017D,
    /// Mercedes-Benz Group AG (0x017C)
    MercedesBenzGroupAg = 0x017C,
    /// taskit GmbH (0x017B)
    TaskitGmbH = 0x017B,
    /// Telemonitor, Inc. (0x017A)
    TelemonitorInc = 0x017A,
    /// LAPIS Semiconductor Co.,Ltd (0x0179)
    LapisSemiconductorCoLtd = 0x0179,
    /// CASIO COMPUTER CO., LTD. (0x0178)
    CasioComputerCoLtd = 0x0178,
    /// I-SYST inc. (0x0177)
    ISystInc = 0x0177,
    /// SentriLock (0x0176)
    SentriLock = 0x0176,
    /// Dynamic Controls (0x0175)
    DynamicControls = 0x0175,
    /// Everykey Inc. (0x0174)
    EverykeyInc = 0x0174,
    /// Kocomojo, LLC (0x0173)
    KocomojoLlc = 0x0173,
    /// Connovate Technology Private Limited (0x0172)
    ConnovateTechnologyPrivateLimited = 0x0172,
    /// Amazon.com Services LLC (0x0171)
    AmazonComServicesLlc = 0x0171,
    /// Roche Diabetes Care AG (0x0170)
    RocheDiabetesCareAg = 0x0170,
    /// Podo Labs, Inc (0x016F)
    PodoLabsInc = 0x016F,
    /// Volantic AB (0x016E)
    VolanticAb = 0x016E,
    /// LifeScan Inc (0x016D)
    LifeScanInc = 0x016D,
    /// MYSPHERA (0x016C)
    Mysphera = 0x016C,
    /// Qblinks (0x016B)
    Qblinks = 0x016B,
    /// Copeland Cold Chain LP (0x016A)
    CopelandColdChainLp = 0x016A,
    /// emberlight (0x0169)
    Emberlight = 0x0169,
    /// Spicebox LLC (0x0168)
    SpiceboxLlc = 0x0168,
    /// Ascensia Diabetes Care US Inc. (0x0167)
    AscensiaDiabetesCareUsInc = 0x0167,
    /// MISHIK Pte Ltd (0x0166)
    MishikPteLtd = 0x0166,
    /// Milwaukee Electric Tools (0x0165)
    MilwaukeeElectricTools = 0x0165,
    /// Qingdao Yeelink Information Technology Co., Ltd. (0x0164)
    QingdaoYeelinkInformationTechnologyCoLtd = 0x0164,
    /// PCH International (0x0163)
    PchInternational = 0x0163,
    /// MADSGlobalNZ Ltd. (0x0162)
    MadsGlobalNzLtd = 0x0162,
    /// yikes (0x0161)
    Yikes = 0x0161,
    /// AwoX (0x0160)
    AwoX = 0x0160,
    /// Timer Cap Co. (0x015F)
    TimerCapCo = 0x015F,
    /// Unikey Technologies, Inc. (0x015E)
    UnikeyTechnologiesInc = 0x015E,
    /// Estimote, Inc. (0x015D)
    EstimoteInc = 0x015D,
    /// Pitius Tec S.L. (0x015C)
    PitiusTecSL = 0x015C,
    /// Biomedical Research Ltd. (0x015B)
    BiomedicalResearchLtd = 0x015B,
    /// micas AG (0x015A)
    MicasAg = 0x015A,
    /// ChefSteps, Inc. (0x0159)
    ChefStepsInc = 0x0159,
    /// Inmite s.r.o. (0x0158)
    InmiteSRO = 0x0158,
    /// Anhui Huami Information Technology Co., Ltd. (0x0157)
    AnhuiHuamiInformationTechnologyCoLtd = 0x0157,
    /// Accumulate AB (0x0156)
    AccumulateAb = 0x0156,
    /// NETATMO (0x0155)
    Netatmo = 0x0155,
    /// Pebble Technology (0x0154)
    PebbleTechnology = 0x0154,
    /// ROL Ergo (0x0153)
    RolErgo = 0x0153,
    /// Vernier Software & Technology (0x0152)
    VernierSoftwareAndTechnology = 0x0152,
    /// OnBeep (0x0151)
    OnBeep = 0x0151,
    /// Pioneer Corporation (0x0150)
    PioneerCorporation = 0x0150,
    /// B&W Group Ltd. (0x014F)
    BAndWGroupLtd = 0x014F,
    /// Tangerine, Inc. (0x014E)
    TangerineInc = 0x014E,
    /// HUIZHOU DESAY SV AUTOMOTIVE CO., LTD. (0x014D)
    HuizhouDesaySvAutomotiveCoLtd = 0x014D,
    /// Mesh-Net Ltd (0x014C)
    MeshNetLtd = 0x014C,
    /// Master Lock (0x014B)
    MasterLock = 0x014B,
    /// Tivoli Audio, LLC (0x014A)
    TivoliAudioLlc = 0x014A,
    /// Perytons Ltd. (0x0149)
    PerytonsLtd = 0x0149,
    /// Ambimat Electronics (0x0148)
    AmbimatElectronics = 0x0148,
    /// Mighty Cast, Inc. (0x0147)
    MightyCastInc = 0x0147,
    /// Ciright (0x0146)
    Ciright = 0x0146,
    /// Novatel Wireless (0x0145)
    NovatelWireless = 0x0145,
    /// Lintech GmbH (0x0144)
    LintechGmbH = 0x0144,
    /// Bkon Connect (0x0143)
    BkonConnect = 0x0143,
    /// Grape Systems Inc. (0x0142)
    GrapeSystemsInc = 0x0142,
    /// FedEx Services (0x0141)
    FedExServices = 0x0141,
    /// Alpine Electronics (China) Co., Ltd (0x0140)
    AlpineElectronicsChinaCoLtd = 0x0140,
    /// B&B Manufacturing Company (0x013F)
    BAndBManufacturingCompany = 0x013F,
    /// Nod, Inc. (0x013E)
    NodInc = 0x013E,
    /// WirelessWERX (0x013D)
    WirelessWerx = 0x013D,
    /// Murata Manufacturing Co., Ltd. (0x013C)
    MurataManufacturingCoLtd = 0x013C,
    /// Allegion (0x013B)
    Allegion = 0x013B,
    /// Tencent Holdings Ltd. (0x013A)
    TencentHoldingsLtd = 0x013A,
    /// Focus Systems Corporation (0x0139)
    FocusSystemsCorporation = 0x0139,
    /// NTEO Inc. (0x0138)
    NteoInc = 0x0138,
    /// Prestigio Plaza Ltd. (0x0137)
    PrestigioPlazaLtd = 0x0137,
    /// Silvair, Inc. (0x0136)
    SilvairInc = 0x0136,
    /// Aireware LLC (0x0135)
    AirewareLlc = 0x0135,
    /// Resolution Products, Ltd. (0x0134)
    ResolutionProductsLtd = 0x0134,
    /// Blue Maestro Limited (0x0133)
    BlueMaestroLimited = 0x0133,
    /// MADS Inc (0x0132)
    MadsInc = 0x0132,
    /// Cypress Semiconductor (0x0131)
    CypressSemiconductor = 0x0131,
    /// Warehouse Innovations (0x0130)
    WarehouseInnovations = 0x0130,
    /// Clarion Co. Inc. (0x012F)
    ClarionCoInc = 0x012F,
    /// ASSA ABLOY (0x012E)
    AssaAbloy = 0x012E,
    /// Sony Corporation (0x012D)
    SonyCorporation = 0x012D,
    /// TEMEC Instruments B.V. (0x012C)
    TemecInstrumentsBV = 0x012C,
    /// SportIQ (0x012B)
    SportIq = 0x012B,
    /// Changzhou Yongse Infotech  Co., Ltd. (0x012A)
    ChangzhouYongseInfotechCoLtd = 0x012A,
    /// Nimble Devices Oy (0x0129)
    NimbleDevicesOy = 0x0129,
    /// GPSI Group Pty Ltd (0x0128)
    GpsiGroupPtyLtd = 0x0128,
    /// Salutica Allied Solutions (0x0127)
    SaluticaAlliedSolutions = 0x0127,
    /// Promethean Ltd. (0x0126)
    PrometheanLtd = 0x0126,
    /// SEAT es (0x0125)
    SeatEs = 0x0125,
    /// HID Global (0x0124)
    HidGlobal = 0x0124,
    /// Kinsa, Inc (0x0123)
    KinsaInc = 0x0123,
    /// AirTurn, Inc. (0x0122)
    AirTurnInc = 0x0122,
    /// Sino Wealth Electronic Ltd. (0x0121)
    SinoWealthElectronicLtd = 0x0121,
    /// Porsche AG (0x0120)
    PorscheAg = 0x0120,
    /// Volkswagen AG (0x011F)
    VolkswagenAg = 0x011F,
    /// Skoda Auto a.s. (0x011E)
    SkodaAutoAS = 0x011E,
    /// Arendi AG (0x011D)
    ArendiAg = 0x011D,
    /// Baidu (0x011C)
    Baidu = 0x011C,
    /// Hewlett Packard Enterprise (0x011B)
    HewlettPackardEnterprise = 0x011B,
    /// Qualcomm Labs, Inc. (0x011A)
    QualcommLabsInc = 0x011A,
    /// Wize Technology Co., Ltd. (0x0119)
    WizeTechnologyCoLtd = 0x0119,
    /// Radius Networks, Inc. (0x0118)
    RadiusNetworksInc = 0x0118,
    /// Wimoto Technologies Inc (0x0117)
    WimotoTechnologiesInc = 0x0117,
    /// 10AK Technologies (0x0116)
    _10AkTechnologies = 0x0116,
    /// e.solutions (0x0115)
    ESolutions = 0x0115,
    /// Xensr (0x0114)
    Xensr = 0x0114,
    /// Openbrain Technologies, Co., Ltd. (0x0113)
    OpenbrainTechnologiesCoLtd = 0x0113,
    /// Visybl Inc. (0x0112)
    VisyblInc = 0x0112,
    /// Steelseries ApS (0x0111)
    SteelseriesApS = 0x0111,
    /// Nippon Seiki Co., Ltd. (0x0110)
    NipponSeikiCoLtd = 0x0110,
    /// HiSilicon Technologies CO., LIMITED (0x010F)
    HiSiliconTechnologiesCoLimited = 0x010F,
    /// Audi AG (0x010E)
    AudiAg = 0x010E,
    /// DENSO TEN Limited (0x010D)
    DensoTenLimited = 0x010D,
    /// Transducers Direct, LLC (0x010C)
    TransducersDirectLlc = 0x010C,
    /// ERi, Inc (0x010B)
    ERiInc = 0x010B,
    /// Codegate Ltd (0x010A)
    CodegateLtd = 0x010A,
    /// Atus BV (0x0109)
    AtusBv = 0x0109,
    /// Chicony Electronics Co., Ltd. (0x0108)
    ChiconyElectronicsCoLtd = 0x0108,
    /// Demant A/S (0x0107)
    DemantAS = 0x0107,
    /// Innovative Yachtter Solutions (0x0106)
    InnovativeYachtterSolutions = 0x0106,
    /// Ubiquitous Computing Technology Corporation (0x0105)
    UbiquitousComputingTechnologyCorporation = 0x0105,
    /// PLUS Location Systems Pty Ltd (0x0104)
    PlusLocationSystemsPtyLtd = 0x0104,
    /// Bang & Olufsen A/S (0x0103)
    BangAndOlufsenAS = 0x0103,
    /// Keiser Corporation (0x0102)
    KeiserCorporation = 0x0102,
    /// Fugoo, Inc. (0x0101)
    FugooInc = 0x0101,
    /// TomTom International BV (0x0100)
    TomTomInternationalBv = 0x0100,
    /// Typo Products, LLC (0x00FF)
    TypoProductsLlc = 0x00FF,
    /// Stanley Black and Decker (0x00FE)
    StanleyBlackAndDecker = 0x00FE,
    /// ValenceTech Limited (0x00FD)
    ValenceTechLimited = 0x00FD,
    /// Delphi Corporation (0x00FC)
    DelphiCorporation = 0x00FC,
    /// KOUKAAM a.s. (0x00FB)
    KoukaamAS = 0x00FB,
    /// Crystal Alarm AB (0x00FA)
    CrystalAlarmAb = 0x00FA,
    /// StickNFind (0x00F9)
    StickNFind = 0x00F9,
    /// AceUni Corp., Ltd. (0x00F8)
    AceUniCorpLtd = 0x00F8,
    /// VSN Technologies, Inc. (0x00F7)
    VsnTechnologiesInc = 0x00F7,
    /// Elcometer Limited (0x00F6)
    ElcometerLimited = 0x00F6,
    /// Smartifier Oy (0x00F5)
    SmartifierOy = 0x00F5,
    /// Nautilus Inc. (0x00F4)
    NautilusInc = 0x00F4,
    /// Kent Displays Inc. (0x00F3)
    KentDisplaysInc = 0x00F3,
    /// Morse Project Inc. (0x00F2)
    MorseProjectInc = 0x00F2,
    /// Witron Technology Limited (0x00F1)
    WitronTechnologyLimited = 0x00F1,
    /// PayPal, Inc. (0x00F0)
    PayPalInc = 0x00F0,
    /// Bitsplitters GmbH (0x00EF)
    BitsplittersGmbH = 0x00EF,
    /// Above Average Outcomes, Inc. (0x00EE)
    AboveAverageOutcomesInc = 0x00EE,
    /// Jolly Logic, LLC (0x00ED)
    JollyLogicLlc = 0x00ED,
    /// BioResearch Associates (0x00EC)
    BioResearchAssociates = 0x00EC,
    /// Server Technology Inc. (0x00EB)
    ServerTechnologyInc = 0x00EB,
    /// www.vtracksystems.com (0x00EA)
    WwwVtracksystemsCom = 0x00EA,
    /// Vtrack Systems (0x00E9)
    VtrackSystems = 0x00E9,
    /// ACTS Technologies (0x00E8)
    ActsTechnologies = 0x00E8,
    /// KS Technologies (0x00E7)
    KsTechnologies = 0x00E7,
    /// Freshtemp (0x00E6)
    Freshtemp = 0x00E6,
    /// Eden Software Consultants Ltd. (0x00E5)
    EdenSoftwareConsultantsLtd = 0x00E5,
    /// L.S. Research, Inc. (0x00E4)
    LSResearchInc = 0x00E4,
    /// inMusic Brands, Inc (0x00E3)
    InMusicBrandsInc = 0x00E3,
    /// Semilink Inc (0x00E2)
    SemilinkInc = 0x00E2,
    /// Danlers Ltd (0x00E1)
    DanlersLtd = 0x00E1,
    /// Google (0x00E0)
    Google = 0x00E0,
    /// Misfit Wearables Corp (0x00DF)
    MisfitWearablesCorp = 0x00DF,
    /// Muzik LLC (0x00DE)
    MuzikLlc = 0x00DE,
    /// Hosiden Corporation (0x00DD)
    HosidenCorporation = 0x00DD,
    /// Procter & Gamble (0x00DC)
    ProcterAndGamble = 0x00DC,
    /// Snuza (Pty) Ltd (0x00DB)
    SnuzaPtyLtd = 0x00DB,
    /// txtr GmbH (0x00DA)
    TxtrGmbH = 0x00DA,
    /// Voyetra Turtle Beach (0x00D9)
    VoyetraTurtleBeach = 0x00D9,
    /// Qualcomm Connected Experiences, Inc. (0x00D8)
    QualcommConnectedExperiencesInc = 0x00D8,
    /// Qualcomm Technologies, Inc. (0x00D7)
    QualcommTechnologiesInc = 0x00D7,
    /// Timex Group USA, Inc. (0x00D6)
    TimexGroupUsaInc = 0x00D6,
    /// Austco Communication Systems (0x00D5)
    AustcoCommunicationSystems = 0x00D5,
    /// Kawantech (0x00D4)
    Kawantech = 0x00D4,
    /// Taixingbang Technology (HK) Co,. LTD. (0x00D3)
    TaixingbangTechnologyHkCoLtd = 0x00D3,
    /// Renesas Design Netherlands B.V. (0x00D2)
    RenesasDesignNetherlandsBV = 0x00D2,
    /// Polar Electro Europe B.V. (0x00D1)
    PolarElectroEuropeBV = 0x00D1,
    /// Dexcom, Inc. (0x00D0)
    DexcomInc = 0x00D0,
    /// ARCHOS SA (0x00CF)
    ArchosSa = 0x00CF,
    /// Eve Systems GmbH (0x00CE)
    EveSystemsGmbH = 0x00CE,
    /// Microchip Technology Inc. (0x00CD)
    MicrochipTechnologyInc = 0x00CD,
    /// Beats Electronics (0x00CC)
    BeatsElectronics = 0x00CC,
    /// Binauric SE (0x00CB)
    BinauricSe = 0x00CB,
    /// MC10 (0x00CA)
    Mc10 = 0x00CA,
    /// Evluma (0x00C9)
    Evluma = 0x00C9,
    /// GeLo Inc (0x00C8)
    GeLoInc = 0x00C8,
    /// Quuppa Oy. (0x00C7)
    QuuppaOy = 0x00C7,
    /// Selfly BV (0x00C6)
    SelflyBv = 0x00C6,
    /// Onset Computer Corporation (0x00C5)
    OnsetComputerCorporation = 0x00C5,
    /// LG Electronics (0x00C4)
    LgElectronics = 0x00C4,
    /// adidas AG (0x00C3)
    AdidasAg = 0x00C3,
    /// Geneq Inc. (0x00C2)
    GeneqInc = 0x00C2,
    /// Shenzhen Excelsecu Data Technology Co.,Ltd (0x00C1)
    ShenzhenExcelsecuDataTechnologyCoLtd = 0x00C1,
    /// AMICCOM Electronics Corporation (0x00C0)
    AmiccomElectronicsCorporation = 0x00C0,
    /// Stalmart Technology Limited (0x00BF)
    StalmartTechnologyLimited = 0x00BF,
    /// AAMP of America (0x00BE)
    AampOfAmerica = 0x00BE,
    /// Aplix Corporation (0x00BD)
    AplixCorporation = 0x00BD,
    /// Ace Sensor Inc (0x00BC)
    AceSensorInc = 0x00BC,
    /// S-Power Electronics Limited (0x00BB)
    SPowerElectronicsLimited = 0x00BB,
    /// Starkey Hearing Technologies (0x00BA)
    StarkeyHearingTechnologies = 0x00BA,
    /// Johnson Controls, Inc. (0x00B9)
    JohnsonControlsInc = 0x00B9,
    /// Qualcomm Innovation Center, Inc. (QuIC) (0x00B8)
    QualcommInnovationCenterIncQuIc = 0x00B8,
    /// TreLab Ltd (0x00B7)
    TreLabLtd = 0x00B7,
    /// Meso international (0x00B6)
    MesoInternational = 0x00B6,
    /// Swirl Networks (0x00B5)
    SwirlNetworks = 0x00B5,
    /// BDE Technology Co., Ltd. (0x00B4)
    BdeTechnologyCoLtd = 0x00B4,
    /// Clarinox Technologies Pty. Ltd. (0x00B3)
    ClarinoxTechnologiesPtyLtd = 0x00B3,
    /// Bekey A/S (0x00B2)
    BekeyAS = 0x00B2,
    /// Saris Cycling Group, Inc (0x00B1)
    SarisCyclingGroupInc = 0x00B1,
    /// Passif Semiconductor Corp (0x00B0)
    PassifSemiconductorCorp = 0x00B0,
    /// Cinetix (0x00AF)
    Cinetix = 0x00AF,
    /// Omegawave Oy (0x00AE)
    OmegawaveOy = 0x00AE,
    /// Peter Systemtechnik GmbH (0x00AD)
    PeterSystemtechnikGmbH = 0x00AD,
    /// Green Throttle Games (0x00AC)
    GreenThrottleGames = 0x00AC,
    /// Ingenieur-Systemgruppe Zahn GmbH (0x00AB)
    IngenieurSystemgruppeZahnGmbH = 0x00AB,
    /// CAEN RFID srl (0x00AA)
    CaenRfidSrl = 0x00AA,
    /// MARELLI EUROPE S.P.A. (0x00A9)
    MarelliEuropeSPA = 0x00A9,
    /// ARP Devices Limited (0x00A8)
    ArpDevicesLimited = 0x00A8,
    /// Visteon Corporation (0x00A7)
    VisteonCorporation = 0x00A7,
    /// Panda Ocean Inc. (0x00A6)
    PandaOceanInc = 0x00A6,
    /// OTL Dynamics LLC (0x00A5)
    OtlDynamicsLlc = 0x00A5,
    /// LINAK A/S (0x00A4)
    LinakAS = 0x00A4,
    /// Meta Watch Ltd. (0x00A3)
    MetaWatchLtd = 0x00A3,
    /// Vertu Corporation Limited (0x00A2)
    VertuCorporationLimited = 0x00A2,
    /// SR-Medizinelektronik (0x00A1)
    SrMedizinelektronik = 0x00A1,
    /// Kensington Computer Products Group (0x00A0)
    KensingtonComputerProductsGroup = 0x00A0,
    /// Suunto Oy (0x009F)
    SuuntoOy = 0x009F,
    /// Bose Corporation (0x009E)
    BoseCorporation = 0x009E,
    /// Geoforce Inc. (0x009D)
    GeoforceInc = 0x009D,
    /// Colorfy, Inc. (0x009C)
    ColorfyInc = 0x009C,
    /// Jiangsu Toppower Automotive Electronics Co., Ltd. (0x009B)
    JiangsuToppowerAutomotiveElectronicsCoLtd = 0x009B,
    /// Alpwise (0x009A)
    Alpwise = 0x009A,
    /// i.Tech Dynamic Global Distribution Ltd. (0x0099)
    ITechDynamicGlobalDistributionLtd = 0x0099,
    /// zero1.tv GmbH (0x0098)
    Zero1TvGmbH = 0x0098,
    /// ConnecteDevice Ltd. (0x0097)
    ConnecteDeviceLtd = 0x0097,
    /// ODM Technology, Inc. (0x0096)
    OdmTechnologyInc = 0x0096,
    /// NEC Lighting, Ltd. (0x0095)
    NecLightingLtd = 0x0095,
    /// Airoha Technology Corp. (0x0094)
    AirohaTechnologyCorp0094 = 0x0094,
    /// Universal Electronics, Inc. (0x0093)
    UniversalElectronicsInc = 0x0093,
    /// ThinkOptics, Inc. (0x0092)
    ThinkOpticsInc = 0x0092,
    /// Advanced PANMOBIL systems GmbH & Co. KG (0x0091)
    AdvancedPanmobilSystemsGmbHAndCoKg = 0x0091,
    /// Funai Electric Co., Ltd. (0x0090)
    FunaiElectricCoLtd = 0x0090,
    /// Telit Wireless Solutions GmbH (0x008F)
    TelitWirelessSolutionsGmbH = 0x008F,
    /// Quintic Corp (0x008E)
    QuinticCorp = 0x008E,
    /// Zscan Software (0x008D)
    ZscanSoftware = 0x008D,
    /// Gimbal Inc. (0x008C)
    GimbalInc = 0x008C,
    /// Topcon Positioning Systems, LLC (0x008B)
    TopconPositioningSystemsLlc = 0x008B,
    /// Jawbone (0x008A)
    Jawbone = 0x008A,
    /// GN Hearing A/S (0x0089)
    GnHearingAS = 0x0089,
    /// Ecotest (0x0088)
    Ecotest = 0x0088,
    /// Garmin International, Inc. (0x0087)
    GarminInternationalInc = 0x0087,
    /// Equinux AG (0x0086)
    EquinuxAg = 0x0086,
    /// BlueRadios, Inc. (0x0085)
    BlueRadiosInc = 0x0085,
    /// Ludus Helsinki Ltd. (0x0084)
    LudusHelsinkiLtd = 0x0084,
    /// TimeKeeping Systems, Inc. (0x0083)
    TimeKeepingSystemsInc = 0x0083,
    /// DSEA A/S (0x0082)
    DseaAS = 0x0082,
    /// WuXi Vimicro (0x0081)
    WuXiVimicro = 0x0081,
    /// DeLorme Publishing Company, Inc. (0x0080)
    DeLormePublishingCompanyInc = 0x0080,
    /// Autonet Mobile (0x007F)
    AutonetMobile = 0x007F,
    /// Sports Tracking Technologies Ltd. (0x007E)
    SportsTrackingTechnologiesLtd = 0x007E,
    /// Seers Technology Co., Ltd. (0x007D)
    SeersTechnologyCoLtd = 0x007D,
    /// A & R Cambridge (0x007C)
    AAndRCambridge = 0x007C,
    /// Hanlynn Technologies (0x007B)
    HanlynnTechnologies = 0x007B,
    /// MStar Semiconductor, Inc. (0x007A)
    MStarSemiconductorInc = 0x007A,
    /// lesswire AG (0x0079)
    LesswireAg = 0x0079,
    /// Nike, Inc. (0x0078)
    NikeInc = 0x0078,
    /// Laird Connectivity LLC (0x0077)
    LairdConnectivityLlc = 0x0077,
    /// Creative Technology Ltd. (0x0076)
    CreativeTechnologyLtd = 0x0076,
    /// Samsung Electronics Co. Ltd. (0x0075)
    SamsungElectronicsCoLtd = 0x0075,
    /// Zomm, LLC (0x0074)
    ZommLlc = 0x0074,
    /// Group Sense Ltd. (0x0073)
    GroupSenseLtd = 0x0073,
    /// ShangHai Super Smart Electronics Co. Ltd. (0x0072)
    ShangHaiSuperSmartElectronicsCoLtd = 0x0072,
    /// connectBlue AB (0x0071)
    ConnectBlueAb = 0x0071,
    /// Monster, LLC (0x0070)
    MonsterLlc = 0x0070,
    /// Sound ID (0x006F)
    SoundId = 0x006F,
    /// Summit Data Communications, Inc. (0x006E)
    SummitDataCommunicationsInc = 0x006E,
    /// BriarTek, Inc (0x006D)
    BriarTekInc = 0x006D,
    /// Beautiful Enterprise Co., Ltd. (0x006C)
    BeautifulEnterpriseCoLtd = 0x006C,
    /// Polar Electro OY (0x006B)
    PolarElectroOy = 0x006B,
    /// LTIMINDTREE LIMITED (0x006A)
    LtimindtreeLimited = 0x006A,
    /// A&D Engineering, Inc. (0x0069)
    AAndDEngineeringInc = 0x0069,
    /// General Motors (0x0068)
    GeneralMotors = 0x0068,
    /// GN Audio A/S (0x0067)
    GnAudioAS = 0x0067,
    /// 9Solutions Oy (0x0066)
    _9SolutionsOy = 0x0066,
    /// HP, Inc. (0x0065)
    HpInc = 0x0065,
    /// Band XI International, LLC (0x0064)
    BandXiInternationalLlc = 0x0064,
    /// MiCommand Inc. (0x0063)
    MiCommandInc = 0x0063,
    /// Gibson Guitars (0x0062)
    GibsonGuitars = 0x0062,
    /// RDA Microelectronics (0x0061)
    RdaMicroelectronics = 0x0061,
    /// RivieraWaves S.A.S (0x0060)
    RivieraWavesSAS = 0x0060,
    /// Wicentric, Inc. (0x005F)
    WicentricInc = 0x005F,
    /// Stonestreet One, LLC (0x005E)
    StonestreetOneLlc = 0x005E,
    /// Realtek Semiconductor Corporation (0x005D)
    RealtekSemiconductorCorporation = 0x005D,
    /// Belkin International, Inc. (0x005C)
    BelkinInternationalInc = 0x005C,
    /// Ralink Technology Corporation (0x005B)
    RalinkTechnologyCorporation = 0x005B,
    /// EM Microelectronic-Marin SA (0x005A)
    EmMicroelectronicMarinSa = 0x005A,
    /// Nordic Semiconductor ASA (0x0059)
    NordicSemiconductorAsa = 0x0059,
    /// Vizio, Inc. (0x0058)
    VizioInc = 0x0058,
    /// Harman International Industries, Inc. (0x0057)
    HarmanInternationalIndustriesInc = 0x0057,
    /// Sony Ericsson Mobile Communications (0x0056)
    SonyEricssonMobileCommunications = 0x0056,
    /// Plantronics, Inc. (0x0055)
    PlantronicsInc = 0x0055,
    /// 3DiJoy Corporation (0x0054)
    _3DiJoyCorporation = 0x0054,
    /// Free2move AB (0x0053)
    Free2MoveAb = 0x0053,
    /// J&M Corporation (0x0052)
    JAndMCorporation = 0x0052,
    /// Tzero Technologies, Inc. (0x0051)
    TzeroTechnologiesInc = 0x0051,
    /// SiRF Technology, Inc. (0x0050)
    SiRfTechnologyInc = 0x0050,
    /// APT Ltd. (0x004F)
    AptLtd = 0x004F,
    /// Avago Technologies (0x004E)
    AvagoTechnologies = 0x004E,
    /// Staccato Communications, Inc. (0x004D)
    StaccatoCommunicationsInc = 0x004D,
    /// Apple, Inc. (0x004C)
    AppleInc = 0x004C,
    /// Continental Automotive Systems (0x004B)
    ContinentalAutomotiveSystems = 0x004B,
    /// Accel Semiconductor Ltd. (0x004A)
    AccelSemiconductorLtd = 0x004A,
    /// 3DSP Corporation (0x0049)
    _3DspCorporation = 0x0049,
    /// Marvell Technology Group Ltd. (0x0048)
    MarvellTechnologyGroupLtd = 0x0048,
    /// Bluegiga (0x0047)
    Bluegiga = 0x0047,
    /// MediaTek, Inc. (0x0046)
    MediaTekInc = 0x0046,
    /// Atheros Communications, Inc. (0x0045)
    AtherosCommunicationsInc = 0x0045,
    /// Socket Mobile (0x0044)
    SocketMobile = 0x0044,
    /// PARROT AUTOMOTIVE SAS (0x0043)
    ParrotAutomotiveSas = 0x0043,
    /// CONWISE Technology Corporation Ltd (0x0042)
    ConwiseTechnologyCorporationLtd = 0x0042,
    /// Integrated Silicon Solution Taiwan, Inc. (0x0041)
    IntegratedSiliconSolutionTaiwanInc = 0x0041,
    /// Seiko Epson Corporation (0x0040)
    SeikoEpsonCorporation = 0x0040,
    /// Bluetooth SIG, Inc (0x003F)
    BluetoothSigInc = 0x003F,
    /// Systems and Chips, Inc (0x003E)
    SystemsAndChipsInc = 0x003E,
    /// IPextreme, Inc. (0x003D)
    IPextremeInc = 0x003D,
    /// BlackBerry Limited (0x003C)
    BlackBerryLimited = 0x003C,
    /// Gennum Corporation (0x003B)
    GennumCorporation = 0x003B,
    /// Panasonic Holdings Corporation (0x003A)
    PanasonicHoldingsCorporation = 0x003A,
    /// Integrated System Solution Corp. (0x0039)
    IntegratedSystemSolutionCorp = 0x0039,
    /// Syntronix Corporation (0x0038)
    SyntronixCorporation = 0x0038,
    /// Mobilian Corporation (0x0037)
    MobilianCorporation = 0x0037,
    /// Renesas Electronics Corporation (0x0036)
    RenesasElectronicsCorporation = 0x0036,
    /// Eclipse (HQ Espana) S.L. (0x0035)
    EclipseHqEspanaSL = 0x0035,
    /// Computer Access Technology Corporation (CATC) (0x0034)
    ComputerAccessTechnologyCorporationCatc = 0x0034,
    /// Commil Ltd (0x0033)
    CommilLtd = 0x0033,
    /// Red-M (Communications) Ltd (0x0032)
    RedMCommunicationsLtd = 0x0032,
    /// Synopsys, Inc. (0x0031)
    SynopsysInc = 0x0031,
    /// ST Microelectronics (0x0030)
    StMicroelectronics = 0x0030,
    /// MewTel Technology Inc. (0x002F)
    MewTelTechnologyInc = 0x002F,
    /// Norwood Systems (0x002E)
    NorwoodSystems = 0x002E,
    /// GCT Semiconductor (0x002D)
    GctSemiconductor = 0x002D,
    /// Macronix International Co. Ltd. (0x002C)
    MacronixInternationalCoLtd = 0x002C,
    /// Tenovis (0x002B)
    Tenovis = 0x002B,
    /// Symbol Technologies, Inc. (0x002A)
    SymbolTechnologiesInc = 0x002A,
    /// Hitachi Ltd (0x0029)
    HitachiLtd = 0x0029,
    /// R F Micro Devices (0x0028)
    RFMicroDevices = 0x0028,
    /// Open Interface (0x0027)
    OpenInterface = 0x0027,
    /// C Technologies (0x0026)
    CTechnologies = 0x0026,
    /// NXP B.V. (0x0025)
    NxpBV = 0x0025,
    /// Alcatel (0x0024)
    Alcatel = 0x0024,
    /// WavePlus Technology Co., Ltd. (0x0023)
    WavePlusTechnologyCoLtd = 0x0023,
    /// NEC Corporation (0x0022)
    NecCorporation = 0x0022,
    /// Mansella Ltd (0x0021)
    MansellaLtd = 0x0021,
    /// BandSpeed, Inc. (0x0020)
    BandSpeedInc = 0x0020,
    /// AVM Berlin (0x001F)
    AvmBerlin = 0x001F,
    /// Inventel (0x001E)
    Inventel = 0x001E,
    /// Qualcomm (0x001D)
    Qualcomm = 0x001D,
    /// Conexant Systems Inc. (0x001C)
    ConexantSystemsInc = 0x001C,
    /// Signia Technologies, Inc. (0x001B)
    SigniaTechnologiesInc = 0x001B,
    /// TTPCom Limited (0x001A)
    TtpComLimited = 0x001A,
    /// Rohde & Schwarz GmbH & Co. KG (0x0019)
    RohdeAndSchwarzGmbHAndCoKg = 0x0019,
    /// Transilica, Inc. (0x0018)
    TransilicaInc = 0x0018,
    /// Newlogic (0x0017)
    Newlogic = 0x0017,
    /// KC Technology Inc. (0x0016)
    KcTechnologyInc = 0x0016,
    /// RTX A/S (0x0015)
    RtxAS = 0x0015,
    /// Mitsubishi Electric Corporation (0x0014)
    MitsubishiElectricCorporation = 0x0014,
    /// Atmel Corporation (0x0013)
    AtmelCorporation = 0x0013,
    /// Zeevo, Inc. (0x0012)
    ZeevoInc = 0x0012,
    /// Widcomm, Inc. (0x0011)
    WidcommInc = 0x0011,
    /// Mitel Semiconductor (0x0010)
    MitelSemiconductor = 0x0010,
    /// Broadcom Corporation (0x000F)
    BroadcomCorporation = 0x000F,
    /// Parthus Technologies Inc. (0x000E)
    ParthusTechnologiesInc = 0x000E,
    /// Texas Instruments Inc. (0x000D)
    TexasInstrumentsInc = 0x000D,
    /// Digianswer A/S (0x000C)
    DigianswerAS = 0x000C,
    /// Silicon Wave (0x000B)
    SiliconWave = 0x000B,
    /// Qualcomm Technologies International, Ltd. (QTIL) (0x000A)
    QualcommTechnologiesInternationalLtdQtil = 0x000A,
    /// Infineon Technologies AG (0x0009)
    InfineonTechnologiesAg = 0x0009,
    /// Motorola (0x0008)
    Motorola = 0x0008,
    /// Lucent (0x0007)
    Lucent = 0x0007,
    /// Microsoft (0x0006)
    Microsoft = 0x0006,
    /// 3Com (0x0005)
    _3Com = 0x0005,
    /// Toshiba Corp. (0x0004)
    ToshibaCorp = 0x0004,
    /// IBM Corp. (0x0003)
    IbmCorp = 0x0003,
    /// Intel Corp. (0x0002)
    IntelCorp = 0x0002,
    /// Nokia Mobile Phones (0x0001)
    NokiaMobilePhones = 0x0001,
    /// Ericsson AB (0x0000)
    EricssonAb = 0x0000,
}
