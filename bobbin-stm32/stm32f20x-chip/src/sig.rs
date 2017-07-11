//! Signals

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

pub const UART7_TX: Uart7Tx = Uart7Tx {};
pub struct Uart7Tx {}
impl Tx for Uart7Tx {}

pub const UART7_RX: Uart7Rx = Uart7Rx {};
pub struct Uart7Rx {}
impl Rx for Uart7Rx {}

pub const UART7_CTS: Uart7Cts = Uart7Cts {};
pub struct Uart7Cts {}
impl Cts for Uart7Cts {}

pub const UART7_RTS: Uart7Rts = Uart7Rts {};
pub struct Uart7Rts {}
impl Rts for Uart7Rts {}

pub const UART7_CK: Uart7Ck = Uart7Ck {};
pub struct Uart7Ck {}
impl Ck for Uart7Ck {}

pub const UART8_TX: Uart8Tx = Uart8Tx {};
pub struct Uart8Tx {}
impl Tx for Uart8Tx {}

pub const UART8_RX: Uart8Rx = Uart8Rx {};
pub struct Uart8Rx {}
impl Rx for Uart8Rx {}

pub const UART8_CTS: Uart8Cts = Uart8Cts {};
pub struct Uart8Cts {}
impl Cts for Uart8Cts {}

pub const UART8_RTS: Uart8Rts = Uart8Rts {};
pub struct Uart8Rts {}
impl Rts for Uart8Rts {}

pub const UART8_CK: Uart8Ck = Uart8Ck {};
pub struct Uart8Ck {}
impl Ck for Uart8Ck {}

pub const TIM2_ETR: Tim2Etr = Tim2Etr {};
pub struct Tim2Etr {}

pub const TIM5_CH1: Tim5Ch1 = Tim5Ch1 {};
pub struct Tim5Ch1 {}

pub const TIM8_ETR: Tim8Etr = Tim8Etr {};
pub struct Tim8Etr {}

pub const UART4_TX: Uart4Tx = Uart4Tx {};
pub struct Uart4Tx {}

pub const ETH_MII_CRS: EthMiiCrs = EthMiiCrs {};
pub struct EthMiiCrs {}

pub const EVENTOUT: Eventout = Eventout {};
pub struct Eventout {}

pub const TIM5_CH2: Tim5Ch2 = Tim5Ch2 {};
pub struct Tim5Ch2 {}

pub const UART4_RX: Uart4Rx = Uart4Rx {};
pub struct Uart4Rx {}

pub const ETH_MII_RX_CLK: EthMiiRxClk = EthMiiRxClk {};
pub struct EthMiiRxClk {}

pub const ETH_RMII_REF_CLK: EthRmiiRefClk = EthRmiiRefClk {};
pub struct EthRmiiRefClk {}

pub const TIM5_CH3: Tim5Ch3 = Tim5Ch3 {};
pub struct Tim5Ch3 {}

pub const TIM9_CH1: Tim9Ch1 = Tim9Ch1 {};
pub struct Tim9Ch1 {}

pub const ETH_MDIO: EthMdio = EthMdio {};
pub struct EthMdio {}

pub const TIM5_CH4: Tim5Ch4 = Tim5Ch4 {};
pub struct Tim5Ch4 {}

pub const TIM9_CH2: Tim9Ch2 = Tim9Ch2 {};
pub struct Tim9Ch2 {}

pub const OTG_HS_ULPI_D0: OtgHsUlpiD0 = OtgHsUlpiD0 {};
pub struct OtgHsUlpiD0 {}

pub const ETH_MII_COL: EthMiiCol = EthMiiCol {};
pub struct EthMiiCol {}

pub const LCD_B5: LcdB5 = LcdB5 {};
pub struct LcdB5 {}

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

pub const LCD_VSYNC: LcdVsync = LcdVsync {};
pub struct LcdVsync {}

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

pub const TIM13_CH1: Tim13Ch1 = Tim13Ch1 {};
pub struct Tim13Ch1 {}

pub const DCMI_PXCLK: DcmiPxclk = DcmiPxclk {};
pub struct DcmiPxclk {}

pub const LCD_G2: LcdG2 = LcdG2 {};
pub struct LcdG2 {}

pub const TIM1_CH1N: Tim1Ch1n = Tim1Ch1n {};
pub struct Tim1Ch1n {}

pub const SPI1_MOSI: Spi1Mosi = Spi1Mosi {};
pub struct Spi1Mosi {}

pub const TIM14_CH1: Tim14Ch1 = Tim14Ch1 {};
pub struct Tim14Ch1 {}

pub const ETH_MII_RX_DV: EthMiiRxDv = EthMiiRxDv {};
pub struct EthMiiRxDv {}

pub const ETH_RMII_CRS_DV: EthRmiiCrsDv = EthRmiiCrsDv {};
pub struct EthRmiiCrsDv {}

pub const MCO1: Mco1 = Mco1 {};
pub struct Mco1 {}

pub const TIM1_CH1: Tim1Ch1 = Tim1Ch1 {};
pub struct Tim1Ch1 {}

pub const I2C3_SCL: I2c3Scl = I2c3Scl {};
pub struct I2c3Scl {}

pub const OTG_FS_SOF: OtgFsSof = OtgFsSof {};
pub struct OtgFsSof {}

pub const LCD_R6: LcdR6 = LcdR6 {};
pub struct LcdR6 {}

pub const TIM1_CH2: Tim1Ch2 = Tim1Ch2 {};
pub struct Tim1Ch2 {}

pub const I2C3_SMBA: I2c3Smba = I2c3Smba {};
pub struct I2c3Smba {}

pub const DCMI_D0: DcmiD0 = DcmiD0 {};
pub struct DcmiD0 {}

pub const TIM1_CH3: Tim1Ch3 = Tim1Ch3 {};
pub struct Tim1Ch3 {}

pub const OTG_FS_ID: OtgFsId = OtgFsId {};
pub struct OtgFsId {}

pub const DCMI_D1: DcmiD1 = DcmiD1 {};
pub struct DcmiD1 {}

pub const TIM1_CH4: Tim1Ch4 = Tim1Ch4 {};
pub struct Tim1Ch4 {}

pub const CAN1_RX: Can1Rx = Can1Rx {};
pub struct Can1Rx {}

pub const OTG_FS_DM: OtgFsDm = OtgFsDm {};
pub struct OtgFsDm {}

pub const LCD_R4: LcdR4 = LcdR4 {};
pub struct LcdR4 {}

pub const TIM1_ETR: Tim1Etr = Tim1Etr {};
pub struct Tim1Etr {}

pub const CAN1_TX: Can1Tx = Can1Tx {};
pub struct Can1Tx {}

pub const OTG_FS_DP: OtgFsDp = OtgFsDp {};
pub struct OtgFsDp {}

pub const LCD_R5: LcdR5 = LcdR5 {};
pub struct LcdR5 {}

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

pub const TIM1_CH2N: Tim1Ch2n = Tim1Ch2n {};
pub struct Tim1Ch2n {}

pub const TIM8_CH2N: Tim8Ch2n = Tim8Ch2n {};
pub struct Tim8Ch2n {}

pub const LCD_R3: LcdR3 = LcdR3 {};
pub struct LcdR3 {}

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

pub const IS2C_CK: Is2cCk = Is2cCk {};
pub struct Is2cCk {}

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

pub const FMC_SDCKE1: FmcSdcke1 = FmcSdcke1 {};
pub struct FmcSdcke1 {}

pub const DCMI_D10: DcmiD10 = DcmiD10 {};
pub struct DcmiD10 {}

pub const I2C1_SCL: I2c1Scl = I2c1Scl {};
pub struct I2c1Scl {}

pub const CAN2_TX: Can2Tx = Can2Tx {};
pub struct Can2Tx {}

pub const FMC_SDNE1: FmcSdne1 = FmcSdne1 {};
pub struct FmcSdne1 {}

pub const DCMI_D5: DcmiD5 = DcmiD5 {};
pub struct DcmiD5 {}

pub const I2C1_SDA: I2c1Sda = I2c1Sda {};
pub struct I2c1Sda {}

pub const FMC_NL: FmcNl = FmcNl {};
pub struct FmcNl {}

pub const DCMI_VSYNC: DcmiVsync = DcmiVsync {};
pub struct DcmiVsync {}

pub const TIM10_CH1: Tim10Ch1 = Tim10Ch1 {};
pub struct Tim10Ch1 {}

pub const ETH_MII_TXD3: EthMiiTxd3 = EthMiiTxd3 {};
pub struct EthMiiTxd3 {}

pub const SDIO_D4: SdioD4 = SdioD4 {};
pub struct SdioD4 {}

pub const DCMI_D6: DcmiD6 = DcmiD6 {};
pub struct DcmiD6 {}

pub const LCD_B6: LcdB6 = LcdB6 {};
pub struct LcdB6 {}

pub const TIM11_CH1: Tim11Ch1 = Tim11Ch1 {};
pub struct Tim11Ch1 {}

pub const SPI2_NSS: Spi2Nss = Spi2Nss {};
pub struct Spi2Nss {}

pub const I2S2_WS: I2s2Ws = I2s2Ws {};
pub struct I2s2Ws {}

pub const SDIO_D5: SdioD5 = SdioD5 {};
pub struct SdioD5 {}

pub const DCMI_D7: DcmiD7 = DcmiD7 {};
pub struct DcmiD7 {}

pub const LCD_B7: LcdB7 = LcdB7 {};
pub struct LcdB7 {}

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

pub const LCD_G4: LcdG4 = LcdG4 {};
pub struct LcdG4 {}

pub const SIM2_CH4: Sim2Ch4 = Sim2Ch4 {};
pub struct Sim2Ch4 {}

pub const I2C2_SDA: I2c2Sda = I2c2Sda {};
pub struct I2c2Sda {}

pub const OTG_HS_ULPI_D4: OtgHsUlpiD4 = OtgHsUlpiD4 {};
pub struct OtgHsUlpiD4 {}

pub const ETH_MII_TX_EN: EthMiiTxEn = EthMiiTxEn {};
pub struct EthMiiTxEn {}

pub const ETH_RMII_TX_EN: EthRmiiTxEn = EthRmiiTxEn {};
pub struct EthRmiiTxEn {}

pub const LCD_G5: LcdG5 = LcdG5 {};
pub struct LcdG5 {}

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

pub const TIM2_CH2N: Tim2Ch2n = Tim2Ch2n {};
pub struct Tim2Ch2n {}

pub const SPI2_MISO: Spi2Miso = Spi2Miso {};
pub struct Spi2Miso {}

pub const I2S2EXT_SD: I2s2extSd = I2s2extSd {};
pub struct I2s2extSd {}

pub const TIM12_CH1: Tim12Ch1 = Tim12Ch1 {};
pub struct Tim12Ch1 {}

pub const OTG_HS_DM: OtgHsDm = OtgHsDm {};
pub struct OtgHsDm {}

pub const RTC_REFIN: RtcRefin = RtcRefin {};
pub struct RtcRefin {}

pub const SPI2_MOSI: Spi2Mosi = Spi2Mosi {};
pub struct Spi2Mosi {}

pub const I2S2_SD: I2s2Sd = I2s2Sd {};
pub struct I2s2Sd {}

pub const TIM12_CH2: Tim12Ch2 = Tim12Ch2 {};
pub struct Tim12Ch2 {}

pub const OTG_HS_DP: OtgHsDp = OtgHsDp {};
pub struct OtgHsDp {}

pub const OTG_HS_ULPI_STP: OtgHsUlpiStp = OtgHsUlpiStp {};
pub struct OtgHsUlpiStp {}

pub const FMC_SDNWE: FmcSdnwe = FmcSdnwe {};
pub struct FmcSdnwe {}

pub const ETH_MDC: EthMdc = EthMdc {};
pub struct EthMdc {}

pub const OTG_HS_ULPI_DIR: OtgHsUlpiDir = OtgHsUlpiDir {};
pub struct OtgHsUlpiDir {}

pub const ETH_MII_TXD2: EthMiiTxd2 = EthMiiTxd2 {};
pub struct EthMiiTxd2 {}

pub const FMC_SDNE0: FmcSdne0 = FmcSdne0 {};
pub struct FmcSdne0 {}

pub const OTG_HS_ULPI_NXT: OtgHsUlpiNxt = OtgHsUlpiNxt {};
pub struct OtgHsUlpiNxt {}

pub const ETH_MII_TX_CLK: EthMiiTxClk = EthMiiTxClk {};
pub struct EthMiiTxClk {}

pub const FMC_SDCKE0: FmcSdcke0 = FmcSdcke0 {};
pub struct FmcSdcke0 {}

pub const ETH_MII_RXD0: EthMiiRxd0 = EthMiiRxd0 {};
pub struct EthMiiRxd0 {}

pub const ETH_RMII_RXD0: EthRmiiRxd0 = EthRmiiRxd0 {};
pub struct EthRmiiRxd0 {}

pub const ETH_MII_RXD1: EthMiiRxd1 = EthMiiRxd1 {};
pub struct EthMiiRxd1 {}

pub const ETH_RMII_RXD1: EthRmiiRxd1 = EthRmiiRxd1 {};
pub struct EthRmiiRxd1 {}

pub const TIM8_CH1: Tim8Ch1 = Tim8Ch1 {};
pub struct Tim8Ch1 {}

pub const I2S2_MCK: I2s2Mck = I2s2Mck {};
pub struct I2s2Mck {}

pub const SDIO_D6: SdioD6 = SdioD6 {};
pub struct SdioD6 {}

pub const LCD_HSYNC: LcdHsync = LcdHsync {};
pub struct LcdHsync {}

pub const TIM8_CH2: Tim8Ch2 = Tim8Ch2 {};
pub struct Tim8Ch2 {}

pub const I2S3_MCK: I2s3Mck = I2s3Mck {};
pub struct I2s3Mck {}

pub const SDIO_D7: SdioD7 = SdioD7 {};
pub struct SdioD7 {}

pub const LCD_G6: LcdG6 = LcdG6 {};
pub struct LcdG6 {}

pub const TIM8_CH3: Tim8Ch3 = Tim8Ch3 {};
pub struct Tim8Ch3 {}

pub const SDIO_D0: SdioD0 = SdioD0 {};
pub struct SdioD0 {}

pub const DCMI_D2: DcmiD2 = DcmiD2 {};
pub struct DcmiD2 {}

pub const MCO2: Mco2 = Mco2 {};
pub struct Mco2 {}

pub const TIM8_CH4: Tim8Ch4 = Tim8Ch4 {};
pub struct Tim8Ch4 {}

pub const I2C3_SDA: I2c3Sda = I2c3Sda {};
pub struct I2c3Sda {}

pub const I2S_CKIN: I2sCkin = I2sCkin {};
pub struct I2sCkin {}

pub const SDIO_D1: SdioD1 = SdioD1 {};
pub struct SdioD1 {}

pub const DCMI_D3: DcmiD3 = DcmiD3 {};
pub struct DcmiD3 {}

pub const I2S3_CK: I2s3Ck = I2s3Ck {};
pub struct I2s3Ck {}

pub const SDIO_D2: SdioD2 = SdioD2 {};
pub struct SdioD2 {}

pub const DCMI_D8: DcmiD8 = DcmiD8 {};
pub struct DcmiD8 {}

pub const LCD_R2: LcdR2 = LcdR2 {};
pub struct LcdR2 {}

pub const SDIO_D3: SdioD3 = SdioD3 {};
pub struct SdioD3 {}

pub const DCMI_D4: DcmiD4 = DcmiD4 {};
pub struct DcmiD4 {}

pub const SDIO_CK: SdioCk = SdioCk {};
pub struct SdioCk {}

pub const DCMI_D9: DcmiD9 = DcmiD9 {};
pub struct DcmiD9 {}

pub const FMC_D2: FmcD2 = FmcD2 {};
pub struct FmcD2 {}

pub const FMC_D3: FmcD3 = FmcD3 {};
pub struct FmcD3 {}

pub const TIM3_ETR: Tim3Etr = Tim3Etr {};
pub struct Tim3Etr {}

pub const UART5_RX: Uart5Rx = Uart5Rx {};
pub struct Uart5Rx {}

pub const SDIO_CMD: SdioCmd = SdioCmd {};
pub struct SdioCmd {}

pub const DCMI_D11: DcmiD11 = DcmiD11 {};
pub struct DcmiD11 {}

pub const FMC_CLK: FmcClk = FmcClk {};
pub struct FmcClk {}

pub const LCD_G7: LcdG7 = LcdG7 {};
pub struct LcdG7 {}

pub const FMC_NOE: FmcNoe = FmcNoe {};
pub struct FmcNoe {}

pub const FMC_NWE: FmcNwe = FmcNwe {};
pub struct FmcNwe {}

pub const SAI1_SD_A: Sai1SdA = Sai1SdA {};
pub struct Sai1SdA {}

pub const FMC_NWAIT: FmcNwait = FmcNwait {};
pub struct FmcNwait {}

pub const LCD_B2: LcdB2 = LcdB2 {};
pub struct LcdB2 {}

pub const FMC_NE1: FmcNe1 = FmcNe1 {};
pub struct FmcNe1 {}

pub const FMC_NCE2: FmcNce2 = FmcNce2 {};
pub struct FmcNce2 {}

pub const FMC_D13: FmcD13 = FmcD13 {};
pub struct FmcD13 {}

pub const FMC_D14: FmcD14 = FmcD14 {};
pub struct FmcD14 {}

pub const FMC_D15: FmcD15 = FmcD15 {};
pub struct FmcD15 {}

pub const FMC_A16: FmcA16 = FmcA16 {};
pub struct FmcA16 {}

pub const FMC_A17: FmcA17 = FmcA17 {};
pub struct FmcA17 {}

pub const FMC_A18: FmcA18 = FmcA18 {};
pub struct FmcA18 {}

pub const FMC_D0: FmcD0 = FmcD0 {};
pub struct FmcD0 {}

pub const FMC_D1: FmcD1 = FmcD1 {};
pub struct FmcD1 {}

pub const TIM4_ETR: Tim4Etr = Tim4Etr {};
pub struct Tim4Etr {}

pub const FMC_NBL0: FmcNbl0 = FmcNbl0 {};
pub struct FmcNbl0 {}

pub const FMC_NBL1: FmcNbl1 = FmcNbl1 {};
pub struct FmcNbl1 {}

pub const TRACECLK: Traceclk = Traceclk {};
pub struct Traceclk {}

pub const SPI4_SCK: Spi4Sck = Spi4Sck {};
pub struct Spi4Sck {}

pub const SAI1_MCLK_A: Sai1MclkA = Sai1MclkA {};
pub struct Sai1MclkA {}

pub const FMC_A23: FmcA23 = FmcA23 {};
pub struct FmcA23 {}

pub const TRACED0: Traced0 = Traced0 {};
pub struct Traced0 {}

pub const SAI1_SD_B: Sai1SdB = Sai1SdB {};
pub struct Sai1SdB {}

pub const FMC_A19: FmcA19 = FmcA19 {};
pub struct FmcA19 {}

pub const TRACED1: Traced1 = Traced1 {};
pub struct Traced1 {}

pub const SPI4_NSS: Spi4Nss = Spi4Nss {};
pub struct Spi4Nss {}

pub const SAI1_FS_A: Sai1FsA = Sai1FsA {};
pub struct Sai1FsA {}

pub const FMC_A20: FmcA20 = FmcA20 {};
pub struct FmcA20 {}

pub const TRACED2: Traced2 = Traced2 {};
pub struct Traced2 {}

pub const SPI4_MISO: Spi4Miso = Spi4Miso {};
pub struct Spi4Miso {}

pub const SAI1_SCK_A: Sai1SckA = Sai1SckA {};
pub struct Sai1SckA {}

pub const FMC_A21: FmcA21 = FmcA21 {};
pub struct FmcA21 {}

pub const TRACED3: Traced3 = Traced3 {};
pub struct Traced3 {}

pub const SPI4_MOSI: Spi4Mosi = Spi4Mosi {};
pub struct Spi4Mosi {}

pub const FMC_A22: FmcA22 = FmcA22 {};
pub struct FmcA22 {}

pub const FMC_D4: FmcD4 = FmcD4 {};
pub struct FmcD4 {}

pub const FMC_D5: FmcD5 = FmcD5 {};
pub struct FmcD5 {}

pub const FMC_D6: FmcD6 = FmcD6 {};
pub struct FmcD6 {}

pub const FMC_D7: FmcD7 = FmcD7 {};
pub struct FmcD7 {}

pub const FMC_D8: FmcD8 = FmcD8 {};
pub struct FmcD8 {}

pub const LCD_G3: LcdG3 = LcdG3 {};
pub struct LcdG3 {}

pub const FMC_D9: FmcD9 = FmcD9 {};
pub struct FmcD9 {}

pub const LCD_B4: LcdB4 = LcdB4 {};
pub struct LcdB4 {}

pub const FMC_D10: FmcD10 = FmcD10 {};
pub struct FmcD10 {}

pub const LCD_DE: LcdDe = LcdDe {};
pub struct LcdDe {}

pub const FMC_D11: FmcD11 = FmcD11 {};
pub struct FmcD11 {}

pub const LCD_CLK: LcdClk = LcdClk {};
pub struct LcdClk {}

pub const FMC_D12: FmcD12 = FmcD12 {};
pub struct FmcD12 {}

pub const LCD_R7: LcdR7 = LcdR7 {};
pub struct LcdR7 {}

pub const FMC_A0: FmcA0 = FmcA0 {};
pub struct FmcA0 {}

pub const FMC_A1: FmcA1 = FmcA1 {};
pub struct FmcA1 {}

pub const FMC_A2: FmcA2 = FmcA2 {};
pub struct FmcA2 {}

pub const FMC_A3: FmcA3 = FmcA3 {};
pub struct FmcA3 {}

pub const FMC_A4: FmcA4 = FmcA4 {};
pub struct FmcA4 {}

pub const FMC_A5: FmcA5 = FmcA5 {};
pub struct FmcA5 {}

pub const SPI5_NSS: Spi5Nss = Spi5Nss {};
pub struct Spi5Nss {}

pub const FMC_NIORD: FmcNiord = FmcNiord {};
pub struct FmcNiord {}

pub const SPI5_SCK: Spi5Sck = Spi5Sck {};
pub struct Spi5Sck {}

pub const SAI1_MCLK_B: Sai1MclkB = Sai1MclkB {};
pub struct Sai1MclkB {}

pub const FMC_NREG: FmcNreg = FmcNreg {};
pub struct FmcNreg {}

pub const SPI5_MISO: Spi5Miso = Spi5Miso {};
pub struct Spi5Miso {}

pub const SAI_SCK_B: SaiSckB = SaiSckB {};
pub struct SaiSckB {}

pub const FMC_NIOWR: FmcNiowr = FmcNiowr {};
pub struct FmcNiowr {}

pub const SPI5_MOSI: Spi5Mosi = Spi5Mosi {};
pub struct Spi5Mosi {}

pub const SAI_FS_B: SaiFsB = SaiFsB {};
pub struct SaiFsB {}

pub const FMC_CD: FmcCd = FmcCd {};
pub struct FmcCd {}

pub const FMC_INTR: FmcIntr = FmcIntr {};
pub struct FmcIntr {}

pub const FMC_SDNRAS: FmcSdnras = FmcSdnras {};
pub struct FmcSdnras {}

pub const DCMI_D12: DcmiD12 = DcmiD12 {};
pub struct DcmiD12 {}

pub const FMC_A6: FmcA6 = FmcA6 {};
pub struct FmcA6 {}

pub const FMC_A7: FmcA7 = FmcA7 {};
pub struct FmcA7 {}

pub const FMC_A8: FmcA8 = FmcA8 {};
pub struct FmcA8 {}

pub const FMC_A9: FmcA9 = FmcA9 {};
pub struct FmcA9 {}

pub const FMC_A10: FmcA10 = FmcA10 {};
pub struct FmcA10 {}

pub const FMC_A11: FmcA11 = FmcA11 {};
pub struct FmcA11 {}

pub const FMC_A12: FmcA12 = FmcA12 {};
pub struct FmcA12 {}

pub const FMC_A13: FmcA13 = FmcA13 {};
pub struct FmcA13 {}

pub const FMC_A14: FmcA14 = FmcA14 {};
pub struct FmcA14 {}

pub const FMC_BA0: FmcBa0 = FmcBa0 {};
pub struct FmcBa0 {}

pub const FMC_A15: FmcA15 = FmcA15 {};
pub struct FmcA15 {}

pub const FMC_BA1: FmcBa1 = FmcBa1 {};
pub struct FmcBa1 {}

pub const FMC_INT2: FmcInt2 = FmcInt2 {};
pub struct FmcInt2 {}

pub const FMC_INT3: FmcInt3 = FmcInt3 {};
pub struct FmcInt3 {}

pub const DCMI_D13: DcmiD13 = DcmiD13 {};
pub struct DcmiD13 {}

pub const SPI6_NSS: Spi6Nss = Spi6Nss {};
pub struct Spi6Nss {}

pub const ETH_PS_OUT: EthPsOut = EthPsOut {};
pub struct EthPsOut {}

pub const FMC_SDCLK: FmcSdclk = FmcSdclk {};
pub struct FmcSdclk {}

pub const FMC_NE2: FmcNe2 = FmcNe2 {};
pub struct FmcNe2 {}

pub const FMC_NCE3: FmcNce3 = FmcNce3 {};
pub struct FmcNce3 {}

pub const FMC_NCE4_1: FmcNce41 = FmcNce41 {};
pub struct FmcNce41 {}

pub const FMC_NE3: FmcNe3 = FmcNe3 {};
pub struct FmcNe3 {}

pub const SPI6_MISO: Spi6Miso = Spi6Miso {};
pub struct Spi6Miso {}

pub const FMC_NCE4_2: FmcNce42 = FmcNce42 {};
pub struct FmcNce42 {}

pub const LCD_B3: LcdB3 = LcdB3 {};
pub struct LcdB3 {}

pub const SPI6_SCK: Spi6Sck = Spi6Sck {};
pub struct Spi6Sck {}

pub const FMC_NE4: FmcNe4 = FmcNe4 {};
pub struct FmcNe4 {}

pub const LCD_B1: LcdB1 = LcdB1 {};
pub struct LcdB1 {}

pub const SPI6_MOSI: Spi6Mosi = Spi6Mosi {};
pub struct Spi6Mosi {}

pub const FMC_A24: FmcA24 = FmcA24 {};
pub struct FmcA24 {}

pub const FMC_A25: FmcA25 = FmcA25 {};
pub struct FmcA25 {}

pub const FMC_SNDCAS: FmcSndcas = FmcSndcas {};
pub struct FmcSndcas {}

pub const LCD_R0: LcdR0 = LcdR0 {};
pub struct LcdR0 {}

pub const LCD_R1: LcdR1 = LcdR1 {};
pub struct LcdR1 {}

pub const FMC_D16: FmcD16 = FmcD16 {};
pub struct FmcD16 {}

pub const FMC_D17: FmcD17 = FmcD17 {};
pub struct FmcD17 {}

pub const FMC_D18: FmcD18 = FmcD18 {};
pub struct FmcD18 {}

pub const FMC_D19: FmcD19 = FmcD19 {};
pub struct FmcD19 {}

pub const FMC_D20: FmcD20 = FmcD20 {};
pub struct FmcD20 {}

pub const FMC_D21: FmcD21 = FmcD21 {};
pub struct FmcD21 {}

pub const FMC_D22: FmcD22 = FmcD22 {};
pub struct FmcD22 {}

pub const FMC_D23: FmcD23 = FmcD23 {};
pub struct FmcD23 {}

pub const FMC_D24: FmcD24 = FmcD24 {};
pub struct FmcD24 {}

pub const FMC_D25: FmcD25 = FmcD25 {};
pub struct FmcD25 {}

pub const FMC_D26: FmcD26 = FmcD26 {};
pub struct FmcD26 {}

pub const FMC_D27: FmcD27 = FmcD27 {};
pub struct FmcD27 {}

pub const FMC_NBL2: FmcNbl2 = FmcNbl2 {};
pub struct FmcNbl2 {}

pub const FMC_NBL3: FmcNbl3 = FmcNbl3 {};
pub struct FmcNbl3 {}

pub const DCM_VSYNC: DcmVsync = DcmVsync {};
pub struct DcmVsync {}

pub const FMC_D28: FmcD28 = FmcD28 {};
pub struct FmcD28 {}

pub const FMC_D29: FmcD29 = FmcD29 {};
pub struct FmcD29 {}

pub const FMC_D30: FmcD30 = FmcD30 {};
pub struct FmcD30 {}

pub const FMC_D31: FmcD31 = FmcD31 {};
pub struct FmcD31 {}

pub const LCD_G0: LcdG0 = LcdG0 {};
pub struct LcdG0 {}

pub const LCD_G1: LcdG1 = LcdG1 {};
pub struct LcdG1 {}

pub const LCD_B0: LcdB0 = LcdB0 {};
pub struct LcdB0 {}

