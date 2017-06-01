pub trait Signal<T> {}

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

pub const UART4_TX: Uart4Tx = Uart4Tx {};
pub struct Uart4Tx {}
impl Tx for Uart4Tx {}

pub const UART4_RX: Uart4Rx = Uart4Rx {};
pub struct Uart4Rx {}
impl Rx for Uart4Rx {}

pub const UART4_CTS: Uart4Cts = Uart4Cts {};
pub struct Uart4Cts {}
impl Cts for Uart4Cts {}

pub const UART4_RTS: Uart4Rts = Uart4Rts {};
pub struct Uart4Rts {}
impl Rts for Uart4Rts {}

pub const UART4_CK: Uart4Ck = Uart4Ck {};
pub struct Uart4Ck {}
impl Ck for Uart4Ck {}

pub const UART5_TX: Uart5Tx = Uart5Tx {};
pub struct Uart5Tx {}
impl Tx for Uart5Tx {}

pub const UART5_RX: Uart5Rx = Uart5Rx {};
pub struct Uart5Rx {}
impl Rx for Uart5Rx {}

pub const UART5_CTS: Uart5Cts = Uart5Cts {};
pub struct Uart5Cts {}
impl Cts for Uart5Cts {}

pub const UART5_RTS: Uart5Rts = Uart5Rts {};
pub struct Uart5Rts {}
impl Rts for Uart5Rts {}

pub const UART5_CK: Uart5Ck = Uart5Ck {};
pub struct Uart5Ck {}
impl Ck for Uart5Ck {}

pub const TIM2_CH1: Tim2Ch1 = Tim2Ch1 {};
pub struct Tim2Ch1 {}

pub const TIM2_ETR: Tim2Etr = Tim2Etr {};
pub struct Tim2Etr {}

pub const TSC_G1_IO1: TscG1Io1 = TscG1Io1 {};
pub struct TscG1Io1 {}

pub const COMP1_OUT: Comp1Out = Comp1Out {};
pub struct Comp1Out {}

pub const TIM8_BKIN: Tim8Bkin = Tim8Bkin {};
pub struct Tim8Bkin {}

pub const TIM8_ETR: Tim8Etr = Tim8Etr {};
pub struct Tim8Etr {}

pub const EVENT_OUT: EventOut = EventOut {};
pub struct EventOut {}

pub const RTC_REFIN: RtcRefin = RtcRefin {};
pub struct RtcRefin {}

pub const TIM2_CH2: Tim2Ch2 = Tim2Ch2 {};
pub struct Tim2Ch2 {}

pub const TSC_G1_IO2: TscG1Io2 = TscG1Io2 {};
pub struct TscG1Io2 {}

pub const TIM15_CH1N: Tim15Ch1n = Tim15Ch1n {};
pub struct Tim15Ch1n {}

pub const TIM2_CH3: Tim2Ch3 = Tim2Ch3 {};
pub struct Tim2Ch3 {}

pub const TSC_G1_IO3: TscG1Io3 = TscG1Io3 {};
pub struct TscG1Io3 {}

pub const COMP2_OUT: Comp2Out = Comp2Out {};
pub struct Comp2Out {}

pub const TIM15_CH1: Tim15Ch1 = Tim15Ch1 {};
pub struct Tim15Ch1 {}

pub const TSC_G1_IO4: TscG1Io4 = TscG1Io4 {};
pub struct TscG1Io4 {}

pub const TIM15_CH2: Tim15Ch2 = Tim15Ch2 {};
pub struct Tim15Ch2 {}

pub const TIM3_CH2: Tim3Ch2 = Tim3Ch2 {};
pub struct Tim3Ch2 {}

pub const TSC_G2_IO1: TscG2Io1 = TscG2Io1 {};
pub struct TscG2Io1 {}

pub const SPI1_NSS: Spi1Nss = Spi1Nss {};
pub struct Spi1Nss {}

pub const SPI3_NSS: Spi3Nss = Spi3Nss {};
pub struct Spi3Nss {}

pub const I2S3_WS: I2s3Ws = I2s3Ws {};
pub struct I2s3Ws {}

pub const TSC_G2_IO2: TscG2Io2 = TscG2Io2 {};
pub struct TscG2Io2 {}

pub const SPI1_SCK: Spi1Sck = Spi1Sck {};
pub struct Spi1Sck {}

pub const TIM16_CH1: Tim16Ch1 = Tim16Ch1 {};
pub struct Tim16Ch1 {}

pub const TIM3_CH1: Tim3Ch1 = Tim3Ch1 {};
pub struct Tim3Ch1 {}

pub const TSC_G2_IO3: TscG2Io3 = TscG2Io3 {};
pub struct TscG2Io3 {}

pub const SPI1_MISO: Spi1Miso = Spi1Miso {};
pub struct Spi1Miso {}

pub const TIM1_BKIN: Tim1Bkin = Tim1Bkin {};
pub struct Tim1Bkin {}

pub const TIM17_CH1: Tim17Ch1 = Tim17Ch1 {};
pub struct Tim17Ch1 {}

pub const TSC_G2_IO4: TscG2Io4 = TscG2Io4 {};
pub struct TscG2Io4 {}

pub const TIM8_CH1N: Tim8Ch1n = Tim8Ch1n {};
pub struct Tim8Ch1n {}

pub const SPI1_MOSI: Spi1Mosi = Spi1Mosi {};
pub struct Spi1Mosi {}

pub const TIM1_CH1N: Tim1Ch1n = Tim1Ch1n {};
pub struct Tim1Ch1n {}

pub const MCO: Mco = Mco {};
pub struct Mco {}

pub const I2C3_SCL: I2c3Scl = I2c3Scl {};
pub struct I2c3Scl {}

pub const I2C2_SMBAL: I2c2Smbal = I2c2Smbal {};
pub struct I2c2Smbal {}

pub const I2S2_MCK: I2s2Mck = I2s2Mck {};
pub struct I2s2Mck {}

pub const TIM1_CH1: Tim1Ch1 = Tim1Ch1 {};
pub struct Tim1Ch1 {}

pub const COMP3_OUT: Comp3Out = Comp3Out {};
pub struct Comp3Out {}

pub const TIM4_ETR: Tim4Etr = Tim4Etr {};
pub struct Tim4Etr {}

pub const I2C3_SMBAL: I2c3Smbal = I2c3Smbal {};
pub struct I2c3Smbal {}

pub const TSC_G4_IO1: TscG4Io1 = TscG4Io1 {};
pub struct TscG4Io1 {}

pub const I2C2_SCL: I2c2Scl = I2c2Scl {};
pub struct I2c2Scl {}

pub const I2S3_MCK: I2s3Mck = I2s3Mck {};
pub struct I2s3Mck {}

pub const TIM1_CH2: Tim1Ch2 = Tim1Ch2 {};
pub struct Tim1Ch2 {}

pub const COMP5_OUT: Comp5Out = Comp5Out {};
pub struct Comp5Out {}

pub const TIM15_BKIN: Tim15Bkin = Tim15Bkin {};
pub struct Tim15Bkin {}

pub const TIM17_BKIN: Tim17Bkin = Tim17Bkin {};
pub struct Tim17Bkin {}

pub const TSC_G4_IO2: TscG4Io2 = TscG4Io2 {};
pub struct TscG4Io2 {}

pub const I2C2_SDA: I2c2Sda = I2c2Sda {};
pub struct I2c2Sda {}

pub const SPI2_MISO: Spi2Miso = Spi2Miso {};
pub struct Spi2Miso {}

pub const I2S2EXT_SD: I2s2extSd = I2s2extSd {};
pub struct I2s2extSd {}

pub const TIM1_CH3: Tim1Ch3 = Tim1Ch3 {};
pub struct Tim1Ch3 {}

pub const COMP6_OUT: Comp6Out = Comp6Out {};
pub struct Comp6Out {}

pub const TIM2_CH4: Tim2Ch4 = Tim2Ch4 {};
pub struct Tim2Ch4 {}

pub const SPI2_MOSI: Spi2Mosi = Spi2Mosi {};
pub struct Spi2Mosi {}

pub const I2S2_SD: I2s2Sd = I2s2Sd {};
pub struct I2s2Sd {}

pub const CAN_RX: CanRx = CanRx {};
pub struct CanRx {}

pub const TIM4_CH1: Tim4Ch1 = Tim4Ch1 {};
pub struct Tim4Ch1 {}

pub const TIM1_CH4: Tim1Ch4 = Tim1Ch4 {};
pub struct Tim1Ch4 {}

pub const TIM1_BKIN2: Tim1Bkin2 = Tim1Bkin2 {};
pub struct Tim1Bkin2 {}

pub const I2SCKIN: I2sckin = I2sckin {};
pub struct I2sckin {}

pub const TIM1_CH2N: Tim1Ch2n = Tim1Ch2n {};
pub struct Tim1Ch2n {}

pub const CAN_TX: CanTx = CanTx {};
pub struct CanTx {}

pub const TIM4_CH2: Tim4Ch2 = Tim4Ch2 {};
pub struct Tim4Ch2 {}

pub const TIM1_ETR: Tim1Etr = Tim1Etr {};
pub struct Tim1Etr {}

pub const SWDIO: Swdio = Swdio {};
pub struct Swdio {}

pub const JTMS: Jtms = Jtms {};
pub struct Jtms {}

pub const TIM16_CH1N: Tim16Ch1n = Tim16Ch1n {};
pub struct Tim16Ch1n {}

pub const TSC_G4_IO3: TscG4Io3 = TscG4Io3 {};
pub struct TscG4Io3 {}

pub const IR_OUT: IrOut = IrOut {};
pub struct IrOut {}

pub const TIM4_CH3: Tim4Ch3 = Tim4Ch3 {};
pub struct Tim4Ch3 {}

pub const SWCLK: Swclk = Swclk {};
pub struct Swclk {}

pub const JTCK: Jtck = Jtck {};
pub struct Jtck {}

pub const TSC_G4_IO4: TscG4Io4 = TscG4Io4 {};
pub struct TscG4Io4 {}

pub const I2C1_SDA: I2c1Sda = I2c1Sda {};
pub struct I2c1Sda {}

pub const TIM8_CH2: Tim8Ch2 = Tim8Ch2 {};
pub struct Tim8Ch2 {}

pub const JTDI: Jtdi = Jtdi {};
pub struct Jtdi {}

pub const TIM8_CH1: Tim8Ch1 = Tim8Ch1 {};
pub struct Tim8Ch1 {}

pub const TSC_SYNC: TscSync = TscSync {};
pub struct TscSync {}

pub const I2C1_SCL: I2c1Scl = I2c1Scl {};
pub struct I2c1Scl {}

pub const TIM3_CH3: Tim3Ch3 = Tim3Ch3 {};
pub struct Tim3Ch3 {}

pub const TSC_G3_IO2: TscG3Io2 = TscG3Io2 {};
pub struct TscG3Io2 {}

pub const TIM8_CH2N: Tim8Ch2n = Tim8Ch2n {};
pub struct Tim8Ch2n {}

pub const TIM3_CH4: Tim3Ch4 = Tim3Ch4 {};
pub struct Tim3Ch4 {}

pub const TSC_G3_IO3: TscG3Io3 = TscG3Io3 {};
pub struct TscG3Io3 {}

pub const TIM8_CH3N: Tim8Ch3n = Tim8Ch3n {};
pub struct Tim8Ch3n {}

pub const TIM1_CH3N: Tim1Ch3n = Tim1Ch3n {};
pub struct Tim1Ch3n {}

pub const COMP4_OUT: Comp4Out = Comp4Out {};
pub struct Comp4Out {}

pub const TSC_G3_IO4: TscG3Io4 = TscG3Io4 {};
pub struct TscG3Io4 {}

pub const JTDO: Jtdo = Jtdo {};
pub struct Jtdo {}

pub const TRACESWO: Traceswo = Traceswo {};
pub struct Traceswo {}

pub const TSC_G5_IO1: TscG5Io1 = TscG5Io1 {};
pub struct TscG5Io1 {}

pub const SPI3_SCK: Spi3Sck = Spi3Sck {};
pub struct Spi3Sck {}

pub const I2S3_CK: I2s3Ck = I2s3Ck {};
pub struct I2s3Ck {}

pub const TIM3_ETR: Tim3Etr = Tim3Etr {};
pub struct Tim3Etr {}

pub const JTRST: Jtrst = Jtrst {};
pub struct Jtrst {}

pub const SPI3_MISO: Spi3Miso = Spi3Miso {};
pub struct Spi3Miso {}

pub const I2S3EXT_SD: I2s3extSd = I2s3extSd {};
pub struct I2s3extSd {}

pub const TIM16_BKIN: Tim16Bkin = Tim16Bkin {};
pub struct Tim16Bkin {}

pub const I2C1_SMBAL: I2c1Smbal = I2c1Smbal {};
pub struct I2c1Smbal {}

pub const SPI3_MOSI: Spi3Mosi = Spi3Mosi {};
pub struct Spi3Mosi {}

pub const I2S3_SD: I2s3Sd = I2s3Sd {};
pub struct I2s3Sd {}

pub const I2C3_SDA: I2c3Sda = I2c3Sda {};
pub struct I2c3Sda {}

pub const TSC_G5_IO3: TscG5Io3 = TscG5Io3 {};
pub struct TscG5Io3 {}

pub const TIM8_BKIN2: Tim8Bkin2 = Tim8Bkin2 {};
pub struct Tim8Bkin2 {}

pub const TIM17_CH1N: Tim17Ch1n = Tim17Ch1n {};
pub struct Tim17Ch1n {}

pub const TSC_G5_IO4: TscG5Io4 = TscG5Io4 {};
pub struct TscG5Io4 {}

pub const FMC_NADV: FmcNadv = FmcNadv {};
pub struct FmcNadv {}

pub const TIM4_CH4: Tim4Ch4 = Tim4Ch4 {};
pub struct Tim4Ch4 {}

pub const TIM8_CH3: Tim8Ch3 = Tim8Ch3 {};
pub struct Tim8Ch3 {}

pub const TSC_G6_IO1: TscG6Io1 = TscG6Io1 {};
pub struct TscG6Io1 {}

pub const TSC_G6_IO2: TscG6Io2 = TscG6Io2 {};
pub struct TscG6Io2 {}

pub const SPI2_NSS: Spi2Nss = Spi2Nss {};
pub struct Spi2Nss {}

pub const I2S2_WS: I2s2Ws = I2s2Ws {};
pub struct I2s2Ws {}

pub const TSC_G6_IO3: TscG6Io3 = TscG6Io3 {};
pub struct TscG6Io3 {}

pub const SPI2_SCK: Spi2Sck = Spi2Sck {};
pub struct Spi2Sck {}

pub const I2S2_CK: I2s2Ck = I2s2Ck {};
pub struct I2s2Ck {}

pub const TSC_G6_IO4: TscG6Io4 = TscG6Io4 {};
pub struct TscG6Io4 {}

pub const COMP7_OUT: Comp7Out = Comp7Out {};
pub struct Comp7Out {}

pub const TIM1_CHETR: Tim1Chetr = Tim1Chetr {};
pub struct Tim1Chetr {}

pub const TSC_G3_IO1: TscG3Io1 = TscG3Io1 {};
pub struct TscG3Io1 {}

pub const TIM8_CH4: Tim8Ch4 = Tim8Ch4 {};
pub struct Tim8Ch4 {}

pub const FMC_D2: FmcD2 = FmcD2 {};
pub struct FmcD2 {}

pub const FMC_D3: FmcD3 = FmcD3 {};
pub struct FmcD3 {}

pub const FMC_CLK: FmcClk = FmcClk {};
pub struct FmcClk {}

pub const FMC_NOE: FmcNoe = FmcNoe {};
pub struct FmcNoe {}

pub const FMC_NWE: FmcNwe = FmcNwe {};
pub struct FmcNwe {}

pub const FMC_NWAIT: FmcNwait = FmcNwait {};
pub struct FmcNwait {}

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

pub const TSC_G8_IO1: TscG8Io1 = TscG8Io1 {};
pub struct TscG8Io1 {}

pub const FMC_A17: FmcA17 = FmcA17 {};
pub struct FmcA17 {}

pub const TSC_G8_IO2: TscG8Io2 = TscG8Io2 {};
pub struct TscG8Io2 {}

pub const FMC_A18: FmcA18 = FmcA18 {};
pub struct FmcA18 {}

pub const TSC_G8_IO3: TscG8Io3 = TscG8Io3 {};
pub struct TscG8Io3 {}

pub const FMC_D0: FmcD0 = FmcD0 {};
pub struct FmcD0 {}

pub const TSC_G8_IO4: TscG8Io4 = TscG8Io4 {};
pub struct TscG8Io4 {}

pub const FMC_D1: FmcD1 = FmcD1 {};
pub struct FmcD1 {}

pub const TIM20_ETR: Tim20Etr = Tim20Etr {};
pub struct Tim20Etr {}

pub const FMC_NBL0: FmcNbl0 = FmcNbl0 {};
pub struct FmcNbl0 {}

pub const TIM20_CH4: Tim20Ch4 = Tim20Ch4 {};
pub struct Tim20Ch4 {}

pub const FMC_NBL1: FmcNbl1 = FmcNbl1 {};
pub struct FmcNbl1 {}

pub const TRACECK: Traceck = Traceck {};
pub struct Traceck {}

pub const TSC_G7_IO1: TscG7Io1 = TscG7Io1 {};
pub struct TscG7Io1 {}

pub const SPI4_SCK: Spi4Sck = Spi4Sck {};
pub struct Spi4Sck {}

pub const TIM20_CH1: Tim20Ch1 = Tim20Ch1 {};
pub struct Tim20Ch1 {}

pub const FMC_A23: FmcA23 = FmcA23 {};
pub struct FmcA23 {}

pub const TRACED0: Traced0 = Traced0 {};
pub struct Traced0 {}

pub const TSC_G7_IO2: TscG7Io2 = TscG7Io2 {};
pub struct TscG7Io2 {}

pub const SPI4_NSS: Spi4Nss = Spi4Nss {};
pub struct Spi4Nss {}

pub const TIM20_CH2: Tim20Ch2 = Tim20Ch2 {};
pub struct Tim20Ch2 {}

pub const FMC_A19: FmcA19 = FmcA19 {};
pub struct FmcA19 {}

pub const TRACED1: Traced1 = Traced1 {};
pub struct Traced1 {}

pub const TSC_G7_IO3: TscG7Io3 = TscG7Io3 {};
pub struct TscG7Io3 {}

pub const TIM20_CH1N: Tim20Ch1n = Tim20Ch1n {};
pub struct Tim20Ch1n {}

pub const FMC_A20: FmcA20 = FmcA20 {};
pub struct FmcA20 {}

pub const TRACED2: Traced2 = Traced2 {};
pub struct Traced2 {}

pub const TSC_G7_IO4: TscG7Io4 = TscG7Io4 {};
pub struct TscG7Io4 {}

pub const SPI4_MISO: Spi4Miso = Spi4Miso {};
pub struct Spi4Miso {}

pub const TIM20_CH2N: Tim20Ch2n = Tim20Ch2n {};
pub struct Tim20Ch2n {}

pub const FMC_A21: FmcA21 = FmcA21 {};
pub struct FmcA21 {}

pub const TRACED3: Traced3 = Traced3 {};
pub struct Traced3 {}

pub const SPI4_MOSI: Spi4Mosi = Spi4Mosi {};
pub struct Spi4Mosi {}

pub const TIM20_CH3N: Tim20Ch3n = Tim20Ch3n {};
pub struct Tim20Ch3n {}

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

pub const FMC_D9: FmcD9 = FmcD9 {};
pub struct FmcD9 {}

pub const FMC_D10: FmcD10 = FmcD10 {};
pub struct FmcD10 {}

pub const FMC_D11: FmcD11 = FmcD11 {};
pub struct FmcD11 {}

pub const FMC_D12: FmcD12 = FmcD12 {};
pub struct FmcD12 {}

pub const TIM20_CH3: Tim20Ch3 = Tim20Ch3 {};
pub struct Tim20Ch3 {}

pub const FMC_A2: FmcA2 = FmcA2 {};
pub struct FmcA2 {}

pub const FMC_A3: FmcA3 = FmcA3 {};
pub struct FmcA3 {}

pub const FMC_A4: FmcA4 = FmcA4 {};
pub struct FmcA4 {}

pub const FMC_A5: FmcA5 = FmcA5 {};
pub struct FmcA5 {}

pub const FMC_NIORD: FmcNiord = FmcNiord {};
pub struct FmcNiord {}

pub const TIM20_BKIN: Tim20Bkin = Tim20Bkin {};
pub struct Tim20Bkin {}

pub const FMC_NREG: FmcNreg = FmcNreg {};
pub struct FmcNreg {}

pub const TIM20_BKIN2: Tim20Bkin2 = Tim20Bkin2 {};
pub struct Tim20Bkin2 {}

pub const FMC_NIOWR: FmcNiowr = FmcNiowr {};
pub struct FmcNiowr {}

pub const FMC_CD: FmcCd = FmcCd {};
pub struct FmcCd {}

pub const FMC_INTR: FmcIntr = FmcIntr {};
pub struct FmcIntr {}

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

pub const FMC_A15: FmcA15 = FmcA15 {};
pub struct FmcA15 {}

pub const FMC_INT2: FmcInt2 = FmcInt2 {};
pub struct FmcInt2 {}

pub const FMC_INT3: FmcInt3 = FmcInt3 {};
pub struct FmcInt3 {}

pub const FMC_NE2: FmcNe2 = FmcNe2 {};
pub struct FmcNe2 {}

pub const FMC_NCE3: FmcNce3 = FmcNce3 {};
pub struct FmcNce3 {}

pub const FMC_NCE4_1: FmcNce41 = FmcNce41 {};
pub struct FmcNce41 {}

pub const FMC_NE3: FmcNe3 = FmcNe3 {};
pub struct FmcNe3 {}

pub const FMC_4_2: Fmc42 = Fmc42 {};
pub struct Fmc42 {}

pub const FMC_NE4: FmcNe4 = FmcNe4 {};
pub struct FmcNe4 {}

pub const FMC_A24: FmcA24 = FmcA24 {};
pub struct FmcA24 {}

pub const FMC_A25: FmcA25 = FmcA25 {};
pub struct FmcA25 {}

pub const FMC_A0: FmcA0 = FmcA0 {};
pub struct FmcA0 {}

pub const FMC_A1: FmcA1 = FmcA1 {};
pub struct FmcA1 {}

