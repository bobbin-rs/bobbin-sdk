pub trait Signal<T> {}

pub trait Tim {}
pub trait SignalTim<T> {}
pub trait Tx {}
pub trait SignalTx<T> {}
pub trait Rx {}
pub trait SignalRx<T> {}
pub trait Cts {}
pub trait SignalCts<T> {}
pub trait Rts {}
pub trait SignalRts<T> {}
pub trait Ck {}
pub trait SignalCk<T> {}

pub const TIM2_CH1: Tim2Ch1 = Tim2Ch1 {};
pub struct Tim2Ch1 {}
impl Tim for Tim2Ch1 {}

pub const TIM2_CH2: Tim2Ch2 = Tim2Ch2 {};
pub struct Tim2Ch2 {}
impl Tim for Tim2Ch2 {}

pub const TIM2_CH3: Tim2Ch3 = Tim2Ch3 {};
pub struct Tim2Ch3 {}
impl Tim for Tim2Ch3 {}

pub const TIM2_CH4: Tim2Ch4 = Tim2Ch4 {};
pub struct Tim2Ch4 {}
impl Tim for Tim2Ch4 {}

pub const TIM3_CH1: Tim3Ch1 = Tim3Ch1 {};
pub struct Tim3Ch1 {}
impl Tim for Tim3Ch1 {}

pub const TIM3_CH2: Tim3Ch2 = Tim3Ch2 {};
pub struct Tim3Ch2 {}
impl Tim for Tim3Ch2 {}

pub const TIM3_CH3: Tim3Ch3 = Tim3Ch3 {};
pub struct Tim3Ch3 {}
impl Tim for Tim3Ch3 {}

pub const TIM3_CH4: Tim3Ch4 = Tim3Ch4 {};
pub struct Tim3Ch4 {}
impl Tim for Tim3Ch4 {}

pub const TIM4_CH1: Tim4Ch1 = Tim4Ch1 {};
pub struct Tim4Ch1 {}
impl Tim for Tim4Ch1 {}

pub const TIM4_CH2: Tim4Ch2 = Tim4Ch2 {};
pub struct Tim4Ch2 {}
impl Tim for Tim4Ch2 {}

pub const TIM4_CH3: Tim4Ch3 = Tim4Ch3 {};
pub struct Tim4Ch3 {}
impl Tim for Tim4Ch3 {}

pub const TIM4_CH4: Tim4Ch4 = Tim4Ch4 {};
pub struct Tim4Ch4 {}
impl Tim for Tim4Ch4 {}

pub const TIM5_CH1: Tim5Ch1 = Tim5Ch1 {};
pub struct Tim5Ch1 {}
impl Tim for Tim5Ch1 {}

pub const TIM5_CH2: Tim5Ch2 = Tim5Ch2 {};
pub struct Tim5Ch2 {}
impl Tim for Tim5Ch2 {}

pub const TIM5_CH3: Tim5Ch3 = Tim5Ch3 {};
pub struct Tim5Ch3 {}
impl Tim for Tim5Ch3 {}

pub const TIM5_CH4: Tim5Ch4 = Tim5Ch4 {};
pub struct Tim5Ch4 {}
impl Tim for Tim5Ch4 {}

pub const TIM9_CH1: Tim9Ch1 = Tim9Ch1 {};
pub struct Tim9Ch1 {}
impl Tim for Tim9Ch1 {}

pub const TIM9_CH2: Tim9Ch2 = Tim9Ch2 {};
pub struct Tim9Ch2 {}
impl Tim for Tim9Ch2 {}

pub const TIM10_CH1: Tim10Ch1 = Tim10Ch1 {};
pub struct Tim10Ch1 {}
impl Tim for Tim10Ch1 {}

pub const TIM11_CH1: Tim11Ch1 = Tim11Ch1 {};
pub struct Tim11Ch1 {}
impl Tim for Tim11Ch1 {}

pub const TIM12_CH1: Tim12Ch1 = Tim12Ch1 {};
pub struct Tim12Ch1 {}
impl Tim for Tim12Ch1 {}

pub const TIM12_CH2: Tim12Ch2 = Tim12Ch2 {};
pub struct Tim12Ch2 {}
impl Tim for Tim12Ch2 {}

pub const TIM13_CH1: Tim13Ch1 = Tim13Ch1 {};
pub struct Tim13Ch1 {}
impl Tim for Tim13Ch1 {}

pub const TIM14_CH1: Tim14Ch1 = Tim14Ch1 {};
pub struct Tim14Ch1 {}
impl Tim for Tim14Ch1 {}

pub const TIM1_CH1: Tim1Ch1 = Tim1Ch1 {};
pub struct Tim1Ch1 {}
impl Tim for Tim1Ch1 {}

pub const TIM1_CH2: Tim1Ch2 = Tim1Ch2 {};
pub struct Tim1Ch2 {}
impl Tim for Tim1Ch2 {}

pub const TIM1_CH3: Tim1Ch3 = Tim1Ch3 {};
pub struct Tim1Ch3 {}
impl Tim for Tim1Ch3 {}

pub const TIM1_CH4: Tim1Ch4 = Tim1Ch4 {};
pub struct Tim1Ch4 {}
impl Tim for Tim1Ch4 {}

pub const TIM8_CH1: Tim8Ch1 = Tim8Ch1 {};
pub struct Tim8Ch1 {}
impl Tim for Tim8Ch1 {}

pub const TIM8_CH2: Tim8Ch2 = Tim8Ch2 {};
pub struct Tim8Ch2 {}
impl Tim for Tim8Ch2 {}

pub const TIM8_CH3: Tim8Ch3 = Tim8Ch3 {};
pub struct Tim8Ch3 {}
impl Tim for Tim8Ch3 {}

pub const TIM8_CH4: Tim8Ch4 = Tim8Ch4 {};
pub struct Tim8Ch4 {}
impl Tim for Tim8Ch4 {}

pub const USART1_TX: Usart1Tx = Usart1Tx {};
pub struct Usart1Tx {}
impl Tx for Usart1Tx {}

pub const USART1_RX: Usart1Rx = Usart1Rx {};
pub struct Usart1Rx {}
impl Rx for Usart1Rx {}

pub const USART1_CTS: Usart1Cts = Usart1Cts {};
pub struct Usart1Cts {}
impl Cts for Usart1Cts {}

pub const USART1_RTS: Usart1Rts = Usart1Rts {};
pub struct Usart1Rts {}
impl Rts for Usart1Rts {}

pub const USART1_CK: Usart1Ck = Usart1Ck {};
pub struct Usart1Ck {}
impl Ck for Usart1Ck {}

pub const USART2_TX: Usart2Tx = Usart2Tx {};
pub struct Usart2Tx {}
impl Tx for Usart2Tx {}

pub const USART2_RX: Usart2Rx = Usart2Rx {};
pub struct Usart2Rx {}
impl Rx for Usart2Rx {}

pub const USART2_CTS: Usart2Cts = Usart2Cts {};
pub struct Usart2Cts {}
impl Cts for Usart2Cts {}

pub const USART2_RTS: Usart2Rts = Usart2Rts {};
pub struct Usart2Rts {}
impl Rts for Usart2Rts {}

pub const USART2_CK: Usart2Ck = Usart2Ck {};
pub struct Usart2Ck {}
impl Ck for Usart2Ck {}

pub const USART3_TX: Usart3Tx = Usart3Tx {};
pub struct Usart3Tx {}
impl Tx for Usart3Tx {}

pub const USART3_RX: Usart3Rx = Usart3Rx {};
pub struct Usart3Rx {}
impl Rx for Usart3Rx {}

pub const USART3_CTS: Usart3Cts = Usart3Cts {};
pub struct Usart3Cts {}
impl Cts for Usart3Cts {}

pub const USART3_RTS: Usart3Rts = Usart3Rts {};
pub struct Usart3Rts {}
impl Rts for Usart3Rts {}

pub const USART3_CK: Usart3Ck = Usart3Ck {};
pub struct Usart3Ck {}
impl Ck for Usart3Ck {}

pub const USART6_TX: Usart6Tx = Usart6Tx {};
pub struct Usart6Tx {}
impl Tx for Usart6Tx {}

pub const USART6_RX: Usart6Rx = Usart6Rx {};
pub struct Usart6Rx {}
impl Rx for Usart6Rx {}

pub const USART6_CTS: Usart6Cts = Usart6Cts {};
pub struct Usart6Cts {}
impl Cts for Usart6Cts {}

pub const USART6_RTS: Usart6Rts = Usart6Rts {};
pub struct Usart6Rts {}
impl Rts for Usart6Rts {}

pub const USART6_CK: Usart6Ck = Usart6Ck {};
pub struct Usart6Ck {}
impl Ck for Usart6Ck {}

pub const USART4_TX: Usart4Tx = Usart4Tx {};
pub struct Usart4Tx {}
impl Tx for Usart4Tx {}

pub const USART4_RX: Usart4Rx = Usart4Rx {};
pub struct Usart4Rx {}
impl Rx for Usart4Rx {}

pub const USART4_CTS: Usart4Cts = Usart4Cts {};
pub struct Usart4Cts {}
impl Cts for Usart4Cts {}

pub const USART4_RTS: Usart4Rts = Usart4Rts {};
pub struct Usart4Rts {}
impl Rts for Usart4Rts {}

pub const USART4_CK: Usart4Ck = Usart4Ck {};
pub struct Usart4Ck {}
impl Ck for Usart4Ck {}

pub const USART5_TX: Usart5Tx = Usart5Tx {};
pub struct Usart5Tx {}
impl Tx for Usart5Tx {}

pub const USART5_RX: Usart5Rx = Usart5Rx {};
pub struct Usart5Rx {}
impl Rx for Usart5Rx {}

pub const USART5_CTS: Usart5Cts = Usart5Cts {};
pub struct Usart5Cts {}
impl Cts for Usart5Cts {}

pub const USART5_RTS: Usart5Rts = Usart5Rts {};
pub struct Usart5Rts {}
impl Rts for Usart5Rts {}

pub const USART5_CK: Usart5Ck = Usart5Ck {};
pub struct Usart5Ck {}
impl Ck for Usart5Ck {}

pub const TIM2_CH1_ETR: Tim2Ch1Etr = Tim2Ch1Etr {};
pub struct Tim2Ch1Etr {}

pub const TIM8_ETR: Tim8Etr = Tim8Etr {};
pub struct Tim8Etr {}

pub const UART4_TX: Uart4Tx = Uart4Tx {};
pub struct Uart4Tx {}

pub const ETH_MII_CRS: EthMiiCrs = EthMiiCrs {};
pub struct EthMiiCrs {}

pub const EVENTOUT: Eventout = Eventout {};
pub struct Eventout {}

pub const UART4_RX: Uart4Rx = Uart4Rx {};
pub struct Uart4Rx {}

pub const ETH_MII_RX_CLK: EthMiiRxClk = EthMiiRxClk {};
pub struct EthMiiRxClk {}

pub const ETH_RMII_REF_CLK: EthRmiiRefClk = EthRmiiRefClk {};
pub struct EthRmiiRefClk {}

pub const ETH_MDIO: EthMdio = EthMdio {};
pub struct EthMdio {}

pub const OTG_HS_ULPI_D0: OtgHsUlpiD0 = OtgHsUlpiD0 {};
pub struct OtgHsUlpiD0 {}

pub const ETH_MII_COL: EthMiiCol = EthMiiCol {};
pub struct EthMiiCol {}

pub const SPI1_NSS: Spi1Nss = Spi1Nss {};
pub struct Spi1Nss {}

pub const SPI3_NSS: Spi3Nss = Spi3Nss {};
pub struct Spi3Nss {}

pub const I2S3_WS: I2s3Ws = I2s3Ws {};
pub struct I2s3Ws {}

pub const OTG_HS_SOF: OtgHsSof = OtgHsSof {};
pub struct OtgHsSof {}

pub const DCMI_HSYNC: DcmiHsync = DcmiHsync {};
pub struct DcmiHsync {}

pub const TIM8_CH1N: Tim8Ch1n = Tim8Ch1n {};
pub struct Tim8Ch1n {}

pub const SPI1_SCK: Spi1Sck = Spi1Sck {};
pub struct Spi1Sck {}

pub const OTG_HS_ULPI_CK: OtgHsUlpiCk = OtgHsUlpiCk {};
pub struct OtgHsUlpiCk {}

pub const TIM1_BKIN: Tim1Bkin = Tim1Bkin {};
pub struct Tim1Bkin {}

pub const TIM8_BKIN: Tim8Bkin = Tim8Bkin {};
pub struct Tim8Bkin {}

pub const SPI1_MISO: Spi1Miso = Spi1Miso {};
pub struct Spi1Miso {}

pub const DCMI_PIXCK: DcmiPixck = DcmiPixck {};
pub struct DcmiPixck {}

pub const TIM1_CH1N: Tim1Ch1n = Tim1Ch1n {};
pub struct Tim1Ch1n {}

pub const SPI1_MOSI: Spi1Mosi = Spi1Mosi {};
pub struct Spi1Mosi {}

pub const ETH_MII_RX_DV: EthMiiRxDv = EthMiiRxDv {};
pub struct EthMiiRxDv {}

pub const ETH_RMII_CRS_DV: EthRmiiCrsDv = EthRmiiCrsDv {};
pub struct EthRmiiCrsDv {}

pub const MCO1: Mco1 = Mco1 {};
pub struct Mco1 {}

pub const I2C3_SCL: I2c3Scl = I2c3Scl {};
pub struct I2c3Scl {}

pub const OTG_FS_SOF: OtgFsSof = OtgFsSof {};
pub struct OtgFsSof {}

pub const I2C3_SMBA: I2c3Smba = I2c3Smba {};
pub struct I2c3Smba {}

pub const DCMI_D0: DcmiD0 = DcmiD0 {};
pub struct DcmiD0 {}

pub const OTG_FS_ID: OtgFsId = OtgFsId {};
pub struct OtgFsId {}

pub const DCMI_D1: DcmiD1 = DcmiD1 {};
pub struct DcmiD1 {}

pub const CAN1_RX: Can1Rx = Can1Rx {};
pub struct Can1Rx {}

pub const OTG_FS_DM: OtgFsDm = OtgFsDm {};
pub struct OtgFsDm {}

pub const TIM1_ETR: Tim1Etr = Tim1Etr {};
pub struct Tim1Etr {}

pub const CAN1_TX: Can1Tx = Can1Tx {};
pub struct Can1Tx {}

pub const OTG_FS_DP: OtgFsDp = OtgFsDp {};
pub struct OtgFsDp {}

pub const JTMS: Jtms = Jtms {};
pub struct Jtms {}

pub const SWDIO: Swdio = Swdio {};
pub struct Swdio {}

pub const JTCK: Jtck = Jtck {};
pub struct Jtck {}

pub const SWCLK: Swclk = Swclk {};
pub struct Swclk {}

pub const JTDI: Jtdi = Jtdi {};
pub struct Jtdi {}

pub const TIM2_ETR: Tim2Etr = Tim2Etr {};
pub struct Tim2Etr {}

pub const TIM1_CH2N: Tim1Ch2n = Tim1Ch2n {};
pub struct Tim1Ch2n {}

pub const TIM8_CH2N: Tim8Ch2n = Tim8Ch2n {};
pub struct Tim8Ch2n {}

pub const OTG_HS_ULPI_D1: OtgHsUlpiD1 = OtgHsUlpiD1 {};
pub struct OtgHsUlpiD1 {}

pub const ETH_MII_RXD2: EthMiiRxd2 = EthMiiRxd2 {};
pub struct EthMiiRxd2 {}

pub const TIM1_CH3N: Tim1Ch3n = Tim1Ch3n {};
pub struct Tim1Ch3n {}

pub const TIM8_CH3N: Tim8Ch3n = Tim8Ch3n {};
pub struct Tim8Ch3n {}

pub const OTG_HS_ULPI_D2: OtgHsUlpiD2 = OtgHsUlpiD2 {};
pub struct OtgHsUlpiD2 {}

pub const ETH_MII_RXD3: EthMiiRxd3 = EthMiiRxd3 {};
pub struct EthMiiRxd3 {}

pub const JTDO: Jtdo = Jtdo {};
pub struct Jtdo {}

pub const TRACESWO: Traceswo = Traceswo {};
pub struct Traceswo {}

pub const SPI3_SCK: Spi3Sck = Spi3Sck {};
pub struct Spi3Sck {}

pub const I2S3_CK: I2s3Ck = I2s3Ck {};
pub struct I2s3Ck {}

pub const NJTRST: Njtrst = Njtrst {};
pub struct Njtrst {}

pub const SPI3_MISO: Spi3Miso = Spi3Miso {};
pub struct Spi3Miso {}

pub const I2S3EXT_SD: I2s3extSd = I2s3extSd {};
pub struct I2s3extSd {}

pub const I2C1_SMBA: I2c1Smba = I2c1Smba {};
pub struct I2c1Smba {}

pub const SPI3_MOSI: Spi3Mosi = Spi3Mosi {};
pub struct Spi3Mosi {}

pub const I2S3_SD: I2s3Sd = I2s3Sd {};
pub struct I2s3Sd {}

pub const CAN2_RX: Can2Rx = Can2Rx {};
pub struct Can2Rx {}

pub const OTG_HS_ULPI_D7: OtgHsUlpiD7 = OtgHsUlpiD7 {};
pub struct OtgHsUlpiD7 {}

pub const ETH_PPS_OUT: EthPpsOut = EthPpsOut {};
pub struct EthPpsOut {}

pub const DCMI_D10: DcmiD10 = DcmiD10 {};
pub struct DcmiD10 {}

pub const I2C1_SCL: I2c1Scl = I2c1Scl {};
pub struct I2c1Scl {}

pub const CAN2_TX: Can2Tx = Can2Tx {};
pub struct Can2Tx {}

pub const DCMI_D5: DcmiD5 = DcmiD5 {};
pub struct DcmiD5 {}

pub const I2C1_SDA: I2c1Sda = I2c1Sda {};
pub struct I2c1Sda {}

pub const FSMC_NL: FsmcNl = FsmcNl {};
pub struct FsmcNl {}

pub const DCMI_VSYNC: DcmiVsync = DcmiVsync {};
pub struct DcmiVsync {}

pub const ETH_MII_TXD3: EthMiiTxd3 = EthMiiTxd3 {};
pub struct EthMiiTxd3 {}

pub const SDIO_D4: SdioD4 = SdioD4 {};
pub struct SdioD4 {}

pub const DCMI_D6: DcmiD6 = DcmiD6 {};
pub struct DcmiD6 {}

pub const SPI2_NSS: Spi2Nss = Spi2Nss {};
pub struct Spi2Nss {}

pub const I2S2_WS: I2s2Ws = I2s2Ws {};
pub struct I2s2Ws {}

pub const SDIO_D5: SdioD5 = SdioD5 {};
pub struct SdioD5 {}

pub const DCMI_D7: DcmiD7 = DcmiD7 {};
pub struct DcmiD7 {}

pub const I2C2_SCL: I2c2Scl = I2c2Scl {};
pub struct I2c2Scl {}

pub const SPI2_SCK: Spi2Sck = Spi2Sck {};
pub struct Spi2Sck {}

pub const I2S2_CK: I2s2Ck = I2s2Ck {};
pub struct I2s2Ck {}

pub const OTG_HS_ULPI_D3: OtgHsUlpiD3 = OtgHsUlpiD3 {};
pub struct OtgHsUlpiD3 {}

pub const ETH_MII_RX_ER: EthMiiRxEr = EthMiiRxEr {};
pub struct EthMiiRxEr {}

pub const I2C2_SDA: I2c2Sda = I2c2Sda {};
pub struct I2c2Sda {}

pub const OTG_HS_ULPI_D4: OtgHsUlpiD4 = OtgHsUlpiD4 {};
pub struct OtgHsUlpiD4 {}

pub const ETH_MII_TX_EN: EthMiiTxEn = EthMiiTxEn {};
pub struct EthMiiTxEn {}

pub const ETH_RMII_TX_EN: EthRmiiTxEn = EthRmiiTxEn {};
pub struct EthRmiiTxEn {}

pub const I2C2_SMBA: I2c2Smba = I2c2Smba {};
pub struct I2c2Smba {}

pub const OTG_HS_ULPI_D5: OtgHsUlpiD5 = OtgHsUlpiD5 {};
pub struct OtgHsUlpiD5 {}

pub const ETH_MII_TXD0: EthMiiTxd0 = EthMiiTxd0 {};
pub struct EthMiiTxd0 {}

pub const ETH_RMII_TXD0: EthRmiiTxd0 = EthRmiiTxd0 {};
pub struct EthRmiiTxd0 {}

pub const OTG_HS_ID: OtgHsId = OtgHsId {};
pub struct OtgHsId {}

pub const OTG_HS_ULPI_D6: OtgHsUlpiD6 = OtgHsUlpiD6 {};
pub struct OtgHsUlpiD6 {}

pub const ETH_MII_TXD1: EthMiiTxd1 = EthMiiTxd1 {};
pub struct EthMiiTxd1 {}

pub const ETH_RMII_TXD1: EthRmiiTxd1 = EthRmiiTxd1 {};
pub struct EthRmiiTxd1 {}

pub const SPI2_MISO: Spi2Miso = Spi2Miso {};
pub struct Spi2Miso {}

pub const I2S2EXT_SD: I2s2extSd = I2s2extSd {};
pub struct I2s2extSd {}

pub const OTG_HS_DM: OtgHsDm = OtgHsDm {};
pub struct OtgHsDm {}

pub const RTC_REFIN: RtcRefin = RtcRefin {};
pub struct RtcRefin {}

pub const SPI2_MOSI: Spi2Mosi = Spi2Mosi {};
pub struct Spi2Mosi {}

pub const I2S2_SD: I2s2Sd = I2s2Sd {};
pub struct I2s2Sd {}

pub const OTG_HS_DP: OtgHsDp = OtgHsDp {};
pub struct OtgHsDp {}

pub const OTG_HS_ULPI_STP: OtgHsUlpiStp = OtgHsUlpiStp {};
pub struct OtgHsUlpiStp {}

pub const ETH_MDC: EthMdc = EthMdc {};
pub struct EthMdc {}

pub const OTG_HS_ULPI_DIR: OtgHsUlpiDir = OtgHsUlpiDir {};
pub struct OtgHsUlpiDir {}

pub const ETH_MII_TXD2: EthMiiTxd2 = EthMiiTxd2 {};
pub struct EthMiiTxd2 {}

pub const OTG_HS_ULPI_NXT: OtgHsUlpiNxt = OtgHsUlpiNxt {};
pub struct OtgHsUlpiNxt {}

pub const ETH_MII_TX_CLK: EthMiiTxClk = EthMiiTxClk {};
pub struct EthMiiTxClk {}

pub const ETH_MII_RXD0: EthMiiRxd0 = EthMiiRxd0 {};
pub struct EthMiiRxd0 {}

pub const ETH_RMII_RXD0: EthRmiiRxd0 = EthRmiiRxd0 {};
pub struct EthRmiiRxd0 {}

pub const ETH_MII_RXD1: EthMiiRxd1 = EthMiiRxd1 {};
pub struct EthMiiRxd1 {}

pub const ETH_RMII_RXD1: EthRmiiRxd1 = EthRmiiRxd1 {};
pub struct EthRmiiRxd1 {}

pub const I2S2_MCK: I2s2Mck = I2s2Mck {};
pub struct I2s2Mck {}

pub const SDIO_D6: SdioD6 = SdioD6 {};
pub struct SdioD6 {}

pub const I2S3_MCK: I2s3Mck = I2s3Mck {};
pub struct I2s3Mck {}

pub const SDIO_D7: SdioD7 = SdioD7 {};
pub struct SdioD7 {}

pub const SDIO_D0: SdioD0 = SdioD0 {};
pub struct SdioD0 {}

pub const DCMI_D2: DcmiD2 = DcmiD2 {};
pub struct DcmiD2 {}

pub const MCO2: Mco2 = Mco2 {};
pub struct Mco2 {}

pub const I2C3_SDA: I2c3Sda = I2c3Sda {};
pub struct I2c3Sda {}

pub const I2S_CKIN: I2sCkin = I2sCkin {};
pub struct I2sCkin {}

pub const SDIO_D1: SdioD1 = SdioD1 {};
pub struct SdioD1 {}

pub const DCMI_D3: DcmiD3 = DcmiD3 {};
pub struct DcmiD3 {}

pub const SDIO_D2: SdioD2 = SdioD2 {};
pub struct SdioD2 {}

pub const DCMI_D8: DcmiD8 = DcmiD8 {};
pub struct DcmiD8 {}

pub const SDIO_D3: SdioD3 = SdioD3 {};
pub struct SdioD3 {}

pub const DCMI_D4: DcmiD4 = DcmiD4 {};
pub struct DcmiD4 {}

pub const UART5_TX: Uart5Tx = Uart5Tx {};
pub struct Uart5Tx {}

pub const SDIO_CK: SdioCk = SdioCk {};
pub struct SdioCk {}

pub const DCMI_D9: DcmiD9 = DcmiD9 {};
pub struct DcmiD9 {}

pub const FSMC_D2: FsmcD2 = FsmcD2 {};
pub struct FsmcD2 {}

pub const FSMC_D3: FsmcD3 = FsmcD3 {};
pub struct FsmcD3 {}

pub const TIM3_ETR: Tim3Etr = Tim3Etr {};
pub struct Tim3Etr {}

pub const UART5_RX: Uart5Rx = Uart5Rx {};
pub struct Uart5Rx {}

pub const SDIO_CMD: SdioCmd = SdioCmd {};
pub struct SdioCmd {}

pub const DCMI_D11: DcmiD11 = DcmiD11 {};
pub struct DcmiD11 {}

pub const FSMC_CLK: FsmcClk = FsmcClk {};
pub struct FsmcClk {}

pub const FSMC_NOE: FsmcNoe = FsmcNoe {};
pub struct FsmcNoe {}

pub const FSMC_NWE: FsmcNwe = FsmcNwe {};
pub struct FsmcNwe {}

pub const FSMC_NWAIT: FsmcNwait = FsmcNwait {};
pub struct FsmcNwait {}

pub const FSMC_NE1: FsmcNe1 = FsmcNe1 {};
pub struct FsmcNe1 {}

pub const FSMC_NCE2: FsmcNce2 = FsmcNce2 {};
pub struct FsmcNce2 {}

pub const FSMC_D13: FsmcD13 = FsmcD13 {};
pub struct FsmcD13 {}

pub const FSMC_D14: FsmcD14 = FsmcD14 {};
pub struct FsmcD14 {}

pub const FSMC_D15: FsmcD15 = FsmcD15 {};
pub struct FsmcD15 {}

pub const FSMC_A16: FsmcA16 = FsmcA16 {};
pub struct FsmcA16 {}

pub const FSMC_A17: FsmcA17 = FsmcA17 {};
pub struct FsmcA17 {}

pub const FSMC_A18: FsmcA18 = FsmcA18 {};
pub struct FsmcA18 {}

pub const FSMC_D0: FsmcD0 = FsmcD0 {};
pub struct FsmcD0 {}

pub const FSMC_D1: FsmcD1 = FsmcD1 {};
pub struct FsmcD1 {}

pub const TIM4_ETR: Tim4Etr = Tim4Etr {};
pub struct Tim4Etr {}

pub const FSMC_NBL0: FsmcNbl0 = FsmcNbl0 {};
pub struct FsmcNbl0 {}

pub const FSMC_NBL1: FsmcNbl1 = FsmcNbl1 {};
pub struct FsmcNbl1 {}

pub const TRACECLK: Traceclk = Traceclk {};
pub struct Traceclk {}

pub const FSMC_A23: FsmcA23 = FsmcA23 {};
pub struct FsmcA23 {}

pub const TRACED0: Traced0 = Traced0 {};
pub struct Traced0 {}

pub const FSMC_A19: FsmcA19 = FsmcA19 {};
pub struct FsmcA19 {}

pub const TRACED1: Traced1 = Traced1 {};
pub struct Traced1 {}

pub const FSMC_A20: FsmcA20 = FsmcA20 {};
pub struct FsmcA20 {}

pub const TRACED2: Traced2 = Traced2 {};
pub struct Traced2 {}

pub const FSMC_A21: FsmcA21 = FsmcA21 {};
pub struct FsmcA21 {}

pub const TRACED3: Traced3 = Traced3 {};
pub struct Traced3 {}

pub const FSMC_A22: FsmcA22 = FsmcA22 {};
pub struct FsmcA22 {}

pub const FSMC_D4: FsmcD4 = FsmcD4 {};
pub struct FsmcD4 {}

pub const FSMC_D5: FsmcD5 = FsmcD5 {};
pub struct FsmcD5 {}

pub const FSMC_D6: FsmcD6 = FsmcD6 {};
pub struct FsmcD6 {}

pub const FSMC_D7: FsmcD7 = FsmcD7 {};
pub struct FsmcD7 {}

pub const FSMC_D8: FsmcD8 = FsmcD8 {};
pub struct FsmcD8 {}

pub const FSMC_D9: FsmcD9 = FsmcD9 {};
pub struct FsmcD9 {}

pub const FSMC_D10: FsmcD10 = FsmcD10 {};
pub struct FsmcD10 {}

pub const FSMC_D11: FsmcD11 = FsmcD11 {};
pub struct FsmcD11 {}

pub const FSMC_D12: FsmcD12 = FsmcD12 {};
pub struct FsmcD12 {}

pub const FSMC_A0: FsmcA0 = FsmcA0 {};
pub struct FsmcA0 {}

pub const FSMC_A1: FsmcA1 = FsmcA1 {};
pub struct FsmcA1 {}

pub const FSMC_A2: FsmcA2 = FsmcA2 {};
pub struct FsmcA2 {}

pub const FSMC_A3: FsmcA3 = FsmcA3 {};
pub struct FsmcA3 {}

pub const FSMC_A4: FsmcA4 = FsmcA4 {};
pub struct FsmcA4 {}

pub const FSMC_A5: FsmcA5 = FsmcA5 {};
pub struct FsmcA5 {}

pub const FSMC_NIORD: FsmcNiord = FsmcNiord {};
pub struct FsmcNiord {}

pub const FSMC_NREG: FsmcNreg = FsmcNreg {};
pub struct FsmcNreg {}

pub const FSMC_NIOWR: FsmcNiowr = FsmcNiowr {};
pub struct FsmcNiowr {}

pub const FSMC_CD: FsmcCd = FsmcCd {};
pub struct FsmcCd {}

pub const FSMC_INTR: FsmcIntr = FsmcIntr {};
pub struct FsmcIntr {}

pub const DCMI_D12: DcmiD12 = DcmiD12 {};
pub struct DcmiD12 {}

pub const FSMC_A6: FsmcA6 = FsmcA6 {};
pub struct FsmcA6 {}

pub const FSMC_A7: FsmcA7 = FsmcA7 {};
pub struct FsmcA7 {}

pub const FSMC_A8: FsmcA8 = FsmcA8 {};
pub struct FsmcA8 {}

pub const FSMC_A9: FsmcA9 = FsmcA9 {};
pub struct FsmcA9 {}

pub const FSMC_A10: FsmcA10 = FsmcA10 {};
pub struct FsmcA10 {}

pub const FSMC_A11: FsmcA11 = FsmcA11 {};
pub struct FsmcA11 {}

pub const FSMC_A12: FsmcA12 = FsmcA12 {};
pub struct FsmcA12 {}

pub const FSMC_A13: FsmcA13 = FsmcA13 {};
pub struct FsmcA13 {}

pub const FSMC_A14: FsmcA14 = FsmcA14 {};
pub struct FsmcA14 {}

pub const FSMC_A15: FsmcA15 = FsmcA15 {};
pub struct FsmcA15 {}

pub const FSMC_INT2: FsmcInt2 = FsmcInt2 {};
pub struct FsmcInt2 {}

pub const FSMC_INT3: FsmcInt3 = FsmcInt3 {};
pub struct FsmcInt3 {}

pub const FSMC_NE2: FsmcNe2 = FsmcNe2 {};
pub struct FsmcNe2 {}

pub const FSMC_NCE3: FsmcNce3 = FsmcNce3 {};
pub struct FsmcNce3 {}

pub const FSMC_NCE4_1: FsmcNce41 = FsmcNce41 {};
pub struct FsmcNce41 {}

pub const FSMC_NE3: FsmcNe3 = FsmcNe3 {};
pub struct FsmcNe3 {}

pub const FSMC_NCE4_2: FsmcNce42 = FsmcNce42 {};
pub struct FsmcNce42 {}

pub const FSMC_NE4: FsmcNe4 = FsmcNe4 {};
pub struct FsmcNe4 {}

pub const FSMC_A24: FsmcA24 = FsmcA24 {};
pub struct FsmcA24 {}

pub const FSMC_A25: FsmcA25 = FsmcA25 {};
pub struct FsmcA25 {}

pub const DCMI_D13: DcmiD13 = DcmiD13 {};
pub struct DcmiD13 {}

