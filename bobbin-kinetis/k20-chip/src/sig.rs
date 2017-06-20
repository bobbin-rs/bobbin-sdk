pub trait Signal<T> {}

pub trait Ftm {}
pub trait SignalFtm<T> {}
pub trait Tx {}
pub trait SignalTx<T> {}
pub trait Rx {}
pub trait SignalRx<T> {}

pub const FTM0_CH0: Ftm0Ch0 = Ftm0Ch0 {};
pub struct Ftm0Ch0 {}
impl Ftm for Ftm0Ch0 {}

pub const FTM0_CH1: Ftm0Ch1 = Ftm0Ch1 {};
pub struct Ftm0Ch1 {}
impl Ftm for Ftm0Ch1 {}

pub const FTM0_CH2: Ftm0Ch2 = Ftm0Ch2 {};
pub struct Ftm0Ch2 {}
impl Ftm for Ftm0Ch2 {}

pub const FTM0_CH3: Ftm0Ch3 = Ftm0Ch3 {};
pub struct Ftm0Ch3 {}
impl Ftm for Ftm0Ch3 {}

pub const FTM0_CH4: Ftm0Ch4 = Ftm0Ch4 {};
pub struct Ftm0Ch4 {}
impl Ftm for Ftm0Ch4 {}

pub const FTM0_CH5: Ftm0Ch5 = Ftm0Ch5 {};
pub struct Ftm0Ch5 {}
impl Ftm for Ftm0Ch5 {}

pub const FTM0_CH6: Ftm0Ch6 = Ftm0Ch6 {};
pub struct Ftm0Ch6 {}
impl Ftm for Ftm0Ch6 {}

pub const FTM0_CH7: Ftm0Ch7 = Ftm0Ch7 {};
pub struct Ftm0Ch7 {}
impl Ftm for Ftm0Ch7 {}

pub const FTM1_CH0: Ftm1Ch0 = Ftm1Ch0 {};
pub struct Ftm1Ch0 {}
impl Ftm for Ftm1Ch0 {}

pub const FTM1_CH1: Ftm1Ch1 = Ftm1Ch1 {};
pub struct Ftm1Ch1 {}
impl Ftm for Ftm1Ch1 {}

pub const FTM1_CH2: Ftm1Ch2 = Ftm1Ch2 {};
pub struct Ftm1Ch2 {}
impl Ftm for Ftm1Ch2 {}

pub const FTM1_CH3: Ftm1Ch3 = Ftm1Ch3 {};
pub struct Ftm1Ch3 {}
impl Ftm for Ftm1Ch3 {}

pub const FTM1_CH4: Ftm1Ch4 = Ftm1Ch4 {};
pub struct Ftm1Ch4 {}
impl Ftm for Ftm1Ch4 {}

pub const FTM1_CH5: Ftm1Ch5 = Ftm1Ch5 {};
pub struct Ftm1Ch5 {}
impl Ftm for Ftm1Ch5 {}

pub const FTM1_CH6: Ftm1Ch6 = Ftm1Ch6 {};
pub struct Ftm1Ch6 {}
impl Ftm for Ftm1Ch6 {}

pub const FTM1_CH7: Ftm1Ch7 = Ftm1Ch7 {};
pub struct Ftm1Ch7 {}
impl Ftm for Ftm1Ch7 {}

pub const FTM2_CH0: Ftm2Ch0 = Ftm2Ch0 {};
pub struct Ftm2Ch0 {}
impl Ftm for Ftm2Ch0 {}

pub const FTM2_CH1: Ftm2Ch1 = Ftm2Ch1 {};
pub struct Ftm2Ch1 {}
impl Ftm for Ftm2Ch1 {}

pub const FTM2_CH2: Ftm2Ch2 = Ftm2Ch2 {};
pub struct Ftm2Ch2 {}
impl Ftm for Ftm2Ch2 {}

pub const FTM2_CH3: Ftm2Ch3 = Ftm2Ch3 {};
pub struct Ftm2Ch3 {}
impl Ftm for Ftm2Ch3 {}

pub const FTM2_CH4: Ftm2Ch4 = Ftm2Ch4 {};
pub struct Ftm2Ch4 {}
impl Ftm for Ftm2Ch4 {}

pub const FTM2_CH5: Ftm2Ch5 = Ftm2Ch5 {};
pub struct Ftm2Ch5 {}
impl Ftm for Ftm2Ch5 {}

pub const FTM2_CH6: Ftm2Ch6 = Ftm2Ch6 {};
pub struct Ftm2Ch6 {}
impl Ftm for Ftm2Ch6 {}

pub const FTM2_CH7: Ftm2Ch7 = Ftm2Ch7 {};
pub struct Ftm2Ch7 {}
impl Ftm for Ftm2Ch7 {}

pub const UART0_TX: Uart0Tx = Uart0Tx {};
pub struct Uart0Tx {}
impl Tx for Uart0Tx {}

pub const UART0_RX: Uart0Rx = Uart0Rx {};
pub struct Uart0Rx {}
impl Rx for Uart0Rx {}

pub const UART1_TX: Uart1Tx = Uart1Tx {};
pub struct Uart1Tx {}
impl Tx for Uart1Tx {}

pub const UART1_RX: Uart1Rx = Uart1Rx {};
pub struct Uart1Rx {}
impl Rx for Uart1Rx {}

pub const UART2_TX: Uart2Tx = Uart2Tx {};
pub struct Uart2Tx {}
impl Tx for Uart2Tx {}

pub const UART2_RX: Uart2Rx = Uart2Rx {};
pub struct Uart2Rx {}
impl Rx for Uart2Rx {}

pub const UART3_TX: Uart3Tx = Uart3Tx {};
pub struct Uart3Tx {}
impl Tx for Uart3Tx {}

pub const UART3_RX: Uart3Rx = Uart3Rx {};
pub struct Uart3Rx {}
impl Rx for Uart3Rx {}

pub const UART4_TX: Uart4Tx = Uart4Tx {};
pub struct Uart4Tx {}
impl Tx for Uart4Tx {}

pub const UART4_RX: Uart4Rx = Uart4Rx {};
pub struct Uart4Rx {}
impl Rx for Uart4Rx {}

pub const UART5_TX: Uart5Tx = Uart5Tx {};
pub struct Uart5Tx {}
impl Tx for Uart5Tx {}

pub const UART5_RX: Uart5Rx = Uart5Rx {};
pub struct Uart5Rx {}
impl Rx for Uart5Rx {}

pub const PTA0: Pta0 = Pta0 {};
pub struct Pta0 {}

pub const UART0_CTS_B: Uart0CtsB = Uart0CtsB {};
pub struct Uart0CtsB {}

pub const UART0_COL_B: Uart0ColB = Uart0ColB {};
pub struct Uart0ColB {}

pub const JTAG_TCLK: JtagTclk = JtagTclk {};
pub struct JtagTclk {}

pub const SWD_CLK: SwdClk = SwdClk {};
pub struct SwdClk {}

pub const PTA1: Pta1 = Pta1 {};
pub struct Pta1 {}

pub const JTAG_TDI: JtagTdi = JtagTdi {};
pub struct JtagTdi {}

pub const PTA2: Pta2 = Pta2 {};
pub struct Pta2 {}

pub const JTAG_TDO: JtagTdo = JtagTdo {};
pub struct JtagTdo {}

pub const TRACE_SWO: TraceSwo = TraceSwo {};
pub struct TraceSwo {}

pub const PTA3: Pta3 = Pta3 {};
pub struct Pta3 {}

pub const UART0_RTS_B: Uart0RtsB = Uart0RtsB {};
pub struct Uart0RtsB {}

pub const JTAG_TMS: JtagTms = JtagTms {};
pub struct JtagTms {}

pub const SWD_DIO: SwdDio = SwdDio {};
pub struct SwdDio {}

pub const PTA4: Pta4 = Pta4 {};
pub struct Pta4 {}

pub const NMI_B: NmiB = NmiB {};
pub struct NmiB {}

pub const PTA5: Pta5 = Pta5 {};
pub struct Pta5 {}

pub const USB_CLKIN: UsbClkin = UsbClkin {};
pub struct UsbClkin {}

pub const RMII0_RXER: Rmii0Rxer = Rmii0Rxer {};
pub struct Rmii0Rxer {}

pub const MII0_RXER: Mii0Rxer = Mii0Rxer {};
pub struct Mii0Rxer {}

pub const CMP2_OUT: Cmp2Out = Cmp2Out {};
pub struct Cmp2Out {}

pub const I2S0_TX_BCLK: I2s0TxBclk = I2s0TxBclk {};
pub struct I2s0TxBclk {}

pub const JTAG_TRST_B: JtagTrstB = JtagTrstB {};
pub struct JtagTrstB {}

pub const PTA6: Pta6 = Pta6 {};
pub struct Pta6 {}

pub const CLKOUT: Clkout = Clkout {};
pub struct Clkout {}

pub const TRACE_CLKOUT: TraceClkout = TraceClkout {};
pub struct TraceClkout {}

pub const ADC0_SE10: Adc0Se10 = Adc0Se10 {};
pub struct Adc0Se10 {}

pub const PTA7: Pta7 = Pta7 {};
pub struct Pta7 {}

pub const TRACE_D3: TraceD3 = TraceD3 {};
pub struct TraceD3 {}

pub const ADC0_SE11: Adc0Se11 = Adc0Se11 {};
pub struct Adc0Se11 {}

pub const PTA8: Pta8 = Pta8 {};
pub struct Pta8 {}

pub const FTM1_QD_PHA: Ftm1QdPha = Ftm1QdPha {};
pub struct Ftm1QdPha {}

pub const TRACE_D2: TraceD2 = TraceD2 {};
pub struct TraceD2 {}

pub const PTA9: Pta9 = Pta9 {};
pub struct Pta9 {}

pub const MII0_RXD3: Mii0Rxd3 = Mii0Rxd3 {};
pub struct Mii0Rxd3 {}

pub const FTM1_QD_PHB: Ftm1QdPhb = Ftm1QdPhb {};
pub struct Ftm1QdPhb {}

pub const TRACE_D1: TraceD1 = TraceD1 {};
pub struct TraceD1 {}

pub const PTA10: Pta10 = Pta10 {};
pub struct Pta10 {}

pub const MII0_RXD2: Mii0Rxd2 = Mii0Rxd2 {};
pub struct Mii0Rxd2 {}

pub const FTM2_QD_PHA: Ftm2QdPha = Ftm2QdPha {};
pub struct Ftm2QdPha {}

pub const TRACE_D0: TraceD0 = TraceD0 {};
pub struct TraceD0 {}

pub const PTA11: Pta11 = Pta11 {};
pub struct Pta11 {}

pub const MII0_RXCLK: Mii0Rxclk = Mii0Rxclk {};
pub struct Mii0Rxclk {}

pub const I2C2_SDA: I2c2Sda = I2c2Sda {};
pub struct I2c2Sda {}

pub const FTM2_QD_PHB: Ftm2QdPhb = Ftm2QdPhb {};
pub struct Ftm2QdPhb {}

pub const CMP2_IN0: Cmp2In0 = Cmp2In0 {};
pub struct Cmp2In0 {}

pub const PTA12: Pta12 = Pta12 {};
pub struct Pta12 {}

pub const CAN0_TX: Can0Tx = Can0Tx {};
pub struct Can0Tx {}

pub const RMII0_RXD1: Rmii0Rxd1 = Rmii0Rxd1 {};
pub struct Rmii0Rxd1 {}

pub const MII0_RXD1: Mii0Rxd1 = Mii0Rxd1 {};
pub struct Mii0Rxd1 {}

pub const I2C2_SCL: I2c2Scl = I2c2Scl {};
pub struct I2c2Scl {}

pub const I2S0_TXD0: I2s0Txd0 = I2s0Txd0 {};
pub struct I2s0Txd0 {}

pub const CMP2_IN1: Cmp2In1 = Cmp2In1 {};
pub struct Cmp2In1 {}

pub const PTA13: Pta13 = Pta13 {};
pub struct Pta13 {}

pub const CAN0_RX: Can0Rx = Can0Rx {};
pub struct Can0Rx {}

pub const RMII0_RXD0: Rmii0Rxd0 = Rmii0Rxd0 {};
pub struct Rmii0Rxd0 {}

pub const MII0_RXD0: Mii0Rxd0 = Mii0Rxd0 {};
pub struct Mii0Rxd0 {}

pub const I2S0_TX_FS: I2s0TxFs = I2s0TxFs {};
pub struct I2s0TxFs {}

pub const PTA14: Pta14 = Pta14 {};
pub struct Pta14 {}

pub const SPI0_PCS0: Spi0Pcs0 = Spi0Pcs0 {};
pub struct Spi0Pcs0 {}

pub const RMII0_CRS_DV: Rmii0CrsDv = Rmii0CrsDv {};
pub struct Rmii0CrsDv {}

pub const MII0_RXDV: Mii0Rxdv = Mii0Rxdv {};
pub struct Mii0Rxdv {}

pub const I2S0_RX_BCLK: I2s0RxBclk = I2s0RxBclk {};
pub struct I2s0RxBclk {}

pub const I2S0_TXD1: I2s0Txd1 = I2s0Txd1 {};
pub struct I2s0Txd1 {}

pub const PTA15: Pta15 = Pta15 {};
pub struct Pta15 {}

pub const SPI0_SCK: Spi0Sck = Spi0Sck {};
pub struct Spi0Sck {}

pub const RMII0_TXEN: Rmii0Txen = Rmii0Txen {};
pub struct Rmii0Txen {}

pub const MII0_TXEN: Mii0Txen = Mii0Txen {};
pub struct Mii0Txen {}

pub const I2S0_RXD0: I2s0Rxd0 = I2s0Rxd0 {};
pub struct I2s0Rxd0 {}

pub const PTA16: Pta16 = Pta16 {};
pub struct Pta16 {}

pub const SPI0_SOUT: Spi0Sout = Spi0Sout {};
pub struct Spi0Sout {}

pub const RMII0_TXD0: Rmii0Txd0 = Rmii0Txd0 {};
pub struct Rmii0Txd0 {}

pub const MII0_TXD0: Mii0Txd0 = Mii0Txd0 {};
pub struct Mii0Txd0 {}

pub const I2S0_RX_FS: I2s0RxFs = I2s0RxFs {};
pub struct I2s0RxFs {}

pub const I2S0_RXD1: I2s0Rxd1 = I2s0Rxd1 {};
pub struct I2s0Rxd1 {}

pub const ADC1_SE17: Adc1Se17 = Adc1Se17 {};
pub struct Adc1Se17 {}

pub const PTA17: Pta17 = Pta17 {};
pub struct Pta17 {}

pub const SPI0_SIN: Spi0Sin = Spi0Sin {};
pub struct Spi0Sin {}

pub const RMII0_TXD1: Rmii0Txd1 = Rmii0Txd1 {};
pub struct Rmii0Txd1 {}

pub const MII0_TXD1: Mii0Txd1 = Mii0Txd1 {};
pub struct Mii0Txd1 {}

pub const I2S0_MCLK: I2s0Mclk = I2s0Mclk {};
pub struct I2s0Mclk {}

pub const EXTAL0: Extal0 = Extal0 {};
pub struct Extal0 {}

pub const PTA18: Pta18 = Pta18 {};
pub struct Pta18 {}

pub const FTM0_FLT2: Ftm0Flt2 = Ftm0Flt2 {};
pub struct Ftm0Flt2 {}

pub const FTM_CLKIN0: FtmClkin0 = FtmClkin0 {};
pub struct FtmClkin0 {}

pub const XTAL0: Xtal0 = Xtal0 {};
pub struct Xtal0 {}

pub const PTA19: Pta19 = Pta19 {};
pub struct Pta19 {}

pub const FTM1_FLT0: Ftm1Flt0 = Ftm1Flt0 {};
pub struct Ftm1Flt0 {}

pub const FTM_CLKIN1: FtmClkin1 = FtmClkin1 {};
pub struct FtmClkin1 {}

pub const LPTMR0_ALT1: Lptmr0Alt1 = Lptmr0Alt1 {};
pub struct Lptmr0Alt1 {}

pub const PTA24: Pta24 = Pta24 {};
pub struct Pta24 {}

pub const MII0_TXD2: Mii0Txd2 = Mii0Txd2 {};
pub struct Mii0Txd2 {}

pub const FB_A29: FbA29 = FbA29 {};
pub struct FbA29 {}

pub const PTA25: Pta25 = Pta25 {};
pub struct Pta25 {}

pub const MII0_TXCLK: Mii0Txclk = Mii0Txclk {};
pub struct Mii0Txclk {}

pub const FB_A28: FbA28 = FbA28 {};
pub struct FbA28 {}

pub const PTA26: Pta26 = Pta26 {};
pub struct Pta26 {}

pub const MII0_TXD3: Mii0Txd3 = Mii0Txd3 {};
pub struct Mii0Txd3 {}

pub const FB_A27: FbA27 = FbA27 {};
pub struct FbA27 {}

pub const PTA27: Pta27 = Pta27 {};
pub struct Pta27 {}

pub const MII0_CRS: Mii0Crs = Mii0Crs {};
pub struct Mii0Crs {}

pub const FB_A26: FbA26 = FbA26 {};
pub struct FbA26 {}

pub const PTA28: Pta28 = Pta28 {};
pub struct Pta28 {}

pub const MII0_TXER: Mii0Txer = Mii0Txer {};
pub struct Mii0Txer {}

pub const FB_A25: FbA25 = FbA25 {};
pub struct FbA25 {}

pub const PTA29: Pta29 = Pta29 {};
pub struct Pta29 {}

pub const MII0_COL: Mii0Col = Mii0Col {};
pub struct Mii0Col {}

pub const FB_A24: FbA24 = FbA24 {};
pub struct FbA24 {}

pub const ADC0_SE8: Adc0Se8 = Adc0Se8 {};
pub struct Adc0Se8 {}

pub const ADC1_SE8: Adc1Se8 = Adc1Se8 {};
pub struct Adc1Se8 {}

pub const PTB0: Ptb0 = Ptb0 {};
pub struct Ptb0 {}

pub const I2C0_SCL: I2c0Scl = I2c0Scl {};
pub struct I2c0Scl {}

pub const RMII0_MDIO: Rmii0Mdio = Rmii0Mdio {};
pub struct Rmii0Mdio {}

pub const MII0_MDIO: Mii0Mdio = Mii0Mdio {};
pub struct Mii0Mdio {}

pub const ADC0_SE9: Adc0Se9 = Adc0Se9 {};
pub struct Adc0Se9 {}

pub const ADC1_SE9: Adc1Se9 = Adc1Se9 {};
pub struct Adc1Se9 {}

pub const PTB1: Ptb1 = Ptb1 {};
pub struct Ptb1 {}

pub const I2C0_SDA: I2c0Sda = I2c0Sda {};
pub struct I2c0Sda {}

pub const RMII0_MDC: Rmii0Mdc = Rmii0Mdc {};
pub struct Rmii0Mdc {}

pub const MII0_MDC: Mii0Mdc = Mii0Mdc {};
pub struct Mii0Mdc {}

pub const ADC0_SE12: Adc0Se12 = Adc0Se12 {};
pub struct Adc0Se12 {}

pub const PTB2: Ptb2 = Ptb2 {};
pub struct Ptb2 {}

pub const ENET0_1588_TMR0: Enet01588Tmr0 = Enet01588Tmr0 {};
pub struct Enet01588Tmr0 {}

pub const FTM0_FLT3: Ftm0Flt3 = Ftm0Flt3 {};
pub struct Ftm0Flt3 {}

pub const ADC0_SE13: Adc0Se13 = Adc0Se13 {};
pub struct Adc0Se13 {}

pub const PTB3: Ptb3 = Ptb3 {};
pub struct Ptb3 {}

pub const ENET0_1588_TMR1: Enet01588Tmr1 = Enet01588Tmr1 {};
pub struct Enet01588Tmr1 {}

pub const FTM0_FLT0: Ftm0Flt0 = Ftm0Flt0 {};
pub struct Ftm0Flt0 {}

pub const ADC1_SE10: Adc1Se10 = Adc1Se10 {};
pub struct Adc1Se10 {}

pub const PTB4: Ptb4 = Ptb4 {};
pub struct Ptb4 {}

pub const ENET0_1588_TMR2: Enet01588Tmr2 = Enet01588Tmr2 {};
pub struct Enet01588Tmr2 {}

pub const ADC1_SE11: Adc1Se11 = Adc1Se11 {};
pub struct Adc1Se11 {}

pub const PTB5: Ptb5 = Ptb5 {};
pub struct Ptb5 {}

pub const ENET0_1588_TMR3: Enet01588Tmr3 = Enet01588Tmr3 {};
pub struct Enet01588Tmr3 {}

pub const FTM2_FLT0: Ftm2Flt0 = Ftm2Flt0 {};
pub struct Ftm2Flt0 {}

pub const ADC1_SE12: Adc1Se12 = Adc1Se12 {};
pub struct Adc1Se12 {}

pub const PTB6: Ptb6 = Ptb6 {};
pub struct Ptb6 {}

pub const FB_AD23: FbAd23 = FbAd23 {};
pub struct FbAd23 {}

pub const ADC1_SE13: Adc1Se13 = Adc1Se13 {};
pub struct Adc1Se13 {}

pub const PTB7: Ptb7 = Ptb7 {};
pub struct Ptb7 {}

pub const FB_AD22: FbAd22 = FbAd22 {};
pub struct FbAd22 {}

pub const PTB8: Ptb8 = Ptb8 {};
pub struct Ptb8 {}

pub const UART3_RTS_B: Uart3RtsB = Uart3RtsB {};
pub struct Uart3RtsB {}

pub const FB_AD21: FbAd21 = FbAd21 {};
pub struct FbAd21 {}

pub const PTB9: Ptb9 = Ptb9 {};
pub struct Ptb9 {}

pub const SPI1_PCS1: Spi1Pcs1 = Spi1Pcs1 {};
pub struct Spi1Pcs1 {}

pub const UART3_CTS_B: Uart3CtsB = Uart3CtsB {};
pub struct Uart3CtsB {}

pub const FB_AD20: FbAd20 = FbAd20 {};
pub struct FbAd20 {}

pub const ADC1_SE14: Adc1Se14 = Adc1Se14 {};
pub struct Adc1Se14 {}

pub const PTB10: Ptb10 = Ptb10 {};
pub struct Ptb10 {}

pub const SPI1_PCS0: Spi1Pcs0 = Spi1Pcs0 {};
pub struct Spi1Pcs0 {}

pub const FB_AD19: FbAd19 = FbAd19 {};
pub struct FbAd19 {}

pub const FTM0_FLT1: Ftm0Flt1 = Ftm0Flt1 {};
pub struct Ftm0Flt1 {}

pub const ADC1_SE15: Adc1Se15 = Adc1Se15 {};
pub struct Adc1Se15 {}

pub const PTB11: Ptb11 = Ptb11 {};
pub struct Ptb11 {}

pub const SPI1_SCK: Spi1Sck = Spi1Sck {};
pub struct Spi1Sck {}

pub const FB_AD18: FbAd18 = FbAd18 {};
pub struct FbAd18 {}

pub const PTB12: Ptb12 = Ptb12 {};
pub struct Ptb12 {}

pub const PTB13: Ptb13 = Ptb13 {};
pub struct Ptb13 {}

pub const PTB16: Ptb16 = Ptb16 {};
pub struct Ptb16 {}

pub const SPI1_SOUT: Spi1Sout = Spi1Sout {};
pub struct Spi1Sout {}

pub const FB_AD17: FbAd17 = FbAd17 {};
pub struct FbAd17 {}

pub const EWM_IN: EwmIn = EwmIn {};
pub struct EwmIn {}

pub const PTB17: Ptb17 = Ptb17 {};
pub struct Ptb17 {}

pub const SPI1_SIN: Spi1Sin = Spi1Sin {};
pub struct Spi1Sin {}

pub const FB_AD16: FbAd16 = FbAd16 {};
pub struct FbAd16 {}

pub const EWM_OUT_B: EwmOutB = EwmOutB {};
pub struct EwmOutB {}

pub const PTB18: Ptb18 = Ptb18 {};
pub struct Ptb18 {}

pub const FB_AD15: FbAd15 = FbAd15 {};
pub struct FbAd15 {}

pub const PTB19: Ptb19 = Ptb19 {};
pub struct Ptb19 {}

pub const FB_OE_B: FbOeB = FbOeB {};
pub struct FbOeB {}

pub const PTB20: Ptb20 = Ptb20 {};
pub struct Ptb20 {}

pub const SPI2_PCS0: Spi2Pcs0 = Spi2Pcs0 {};
pub struct Spi2Pcs0 {}

pub const FB_AD31: FbAd31 = FbAd31 {};
pub struct FbAd31 {}

pub const CMP0_OUT: Cmp0Out = Cmp0Out {};
pub struct Cmp0Out {}

pub const PTB21: Ptb21 = Ptb21 {};
pub struct Ptb21 {}

pub const SPI2_SCK: Spi2Sck = Spi2Sck {};
pub struct Spi2Sck {}

pub const FB_AD30: FbAd30 = FbAd30 {};
pub struct FbAd30 {}

pub const CMP1_OUT: Cmp1Out = Cmp1Out {};
pub struct Cmp1Out {}

pub const PTB22: Ptb22 = Ptb22 {};
pub struct Ptb22 {}

pub const SPI2_SOUT: Spi2Sout = Spi2Sout {};
pub struct Spi2Sout {}

pub const FB_AD29: FbAd29 = FbAd29 {};
pub struct FbAd29 {}

pub const PTB23: Ptb23 = Ptb23 {};
pub struct Ptb23 {}

pub const SPI2_SIN: Spi2Sin = Spi2Sin {};
pub struct Spi2Sin {}

pub const SPI0_PCS5: Spi0Pcs5 = Spi0Pcs5 {};
pub struct Spi0Pcs5 {}

pub const FB_AD28: FbAd28 = FbAd28 {};
pub struct FbAd28 {}

pub const ADC0_SE14: Adc0Se14 = Adc0Se14 {};
pub struct Adc0Se14 {}

pub const PTC0: Ptc0 = Ptc0 {};
pub struct Ptc0 {}

pub const SPI0_PCS4: Spi0Pcs4 = Spi0Pcs4 {};
pub struct Spi0Pcs4 {}

pub const PDB0_EXTRG: Pdb0Extrg = Pdb0Extrg {};
pub struct Pdb0Extrg {}

pub const USB_SOF_OUT: UsbSofOut = UsbSofOut {};
pub struct UsbSofOut {}

pub const FB_AD14: FbAd14 = FbAd14 {};
pub struct FbAd14 {}

pub const ADC0_SE15: Adc0Se15 = Adc0Se15 {};
pub struct Adc0Se15 {}

pub const PTC1: Ptc1 = Ptc1 {};
pub struct Ptc1 {}

pub const SPI0_PCS3: Spi0Pcs3 = Spi0Pcs3 {};
pub struct Spi0Pcs3 {}

pub const UART1_RTS_B: Uart1RtsB = Uart1RtsB {};
pub struct Uart1RtsB {}

pub const FB_AD13: FbAd13 = FbAd13 {};
pub struct FbAd13 {}

pub const ADC0_SE4B: Adc0Se4b = Adc0Se4b {};
pub struct Adc0Se4b {}

pub const CMP1_IN0: Cmp1In0 = Cmp1In0 {};
pub struct Cmp1In0 {}

pub const PTC2: Ptc2 = Ptc2 {};
pub struct Ptc2 {}

pub const SPI0_PCS2: Spi0Pcs2 = Spi0Pcs2 {};
pub struct Spi0Pcs2 {}

pub const UART1_CTS_B: Uart1CtsB = Uart1CtsB {};
pub struct Uart1CtsB {}

pub const FB_AD12: FbAd12 = FbAd12 {};
pub struct FbAd12 {}

pub const CMP1_IN1: Cmp1In1 = Cmp1In1 {};
pub struct Cmp1In1 {}

pub const PTC3: Ptc3 = Ptc3 {};
pub struct Ptc3 {}

pub const SPI0_PCS1: Spi0Pcs1 = Spi0Pcs1 {};
pub struct Spi0Pcs1 {}

pub const PTC4: Ptc4 = Ptc4 {};
pub struct Ptc4 {}

pub const FB_AD11: FbAd11 = FbAd11 {};
pub struct FbAd11 {}

pub const PTC5: Ptc5 = Ptc5 {};
pub struct Ptc5 {}

pub const LPTMR0_ALT2: Lptmr0Alt2 = Lptmr0Alt2 {};
pub struct Lptmr0Alt2 {}

pub const FB_AD10: FbAd10 = FbAd10 {};
pub struct FbAd10 {}

pub const CMP0_IN0: Cmp0In0 = Cmp0In0 {};
pub struct Cmp0In0 {}

pub const PTC6: Ptc6 = Ptc6 {};
pub struct Ptc6 {}

pub const FB_AD9: FbAd9 = FbAd9 {};
pub struct FbAd9 {}

pub const CMP0_IN1: Cmp0In1 = Cmp0In1 {};
pub struct Cmp0In1 {}

pub const PTC7: Ptc7 = Ptc7 {};
pub struct Ptc7 {}

pub const FB_AD8: FbAd8 = FbAd8 {};
pub struct FbAd8 {}

pub const ADC1_SE4B: Adc1Se4b = Adc1Se4b {};
pub struct Adc1Se4b {}

pub const CMP0_IN2: Cmp0In2 = Cmp0In2 {};
pub struct Cmp0In2 {}

pub const PTC8: Ptc8 = Ptc8 {};
pub struct Ptc8 {}

pub const FTM3_CH4: Ftm3Ch4 = Ftm3Ch4 {};
pub struct Ftm3Ch4 {}

pub const FB_AD7: FbAd7 = FbAd7 {};
pub struct FbAd7 {}

pub const ADC1_SE5B: Adc1Se5b = Adc1Se5b {};
pub struct Adc1Se5b {}

pub const CMP0_IN3: Cmp0In3 = Cmp0In3 {};
pub struct Cmp0In3 {}

pub const PTC9: Ptc9 = Ptc9 {};
pub struct Ptc9 {}

pub const FTM3_CH5: Ftm3Ch5 = Ftm3Ch5 {};
pub struct Ftm3Ch5 {}

pub const FB_AD6: FbAd6 = FbAd6 {};
pub struct FbAd6 {}

pub const ADC1_SE6B: Adc1Se6b = Adc1Se6b {};
pub struct Adc1Se6b {}

pub const PTC10: Ptc10 = Ptc10 {};
pub struct Ptc10 {}

pub const I2C1_SCL: I2c1Scl = I2c1Scl {};
pub struct I2c1Scl {}

pub const FTM3_CH6: Ftm3Ch6 = Ftm3Ch6 {};
pub struct Ftm3Ch6 {}

pub const FB_AD5: FbAd5 = FbAd5 {};
pub struct FbAd5 {}

pub const ADC1_SE7B: Adc1Se7b = Adc1Se7b {};
pub struct Adc1Se7b {}

pub const PTC11: Ptc11 = Ptc11 {};
pub struct Ptc11 {}

pub const I2C1_SDA: I2c1Sda = I2c1Sda {};
pub struct I2c1Sda {}

pub const FTM3_CH7: Ftm3Ch7 = Ftm3Ch7 {};
pub struct Ftm3Ch7 {}

pub const FB_RW_B: FbRwB = FbRwB {};
pub struct FbRwB {}

pub const PTC12: Ptc12 = Ptc12 {};
pub struct Ptc12 {}

pub const UART4_RTS_B: Uart4RtsB = Uart4RtsB {};
pub struct Uart4RtsB {}

pub const FB_AD27: FbAd27 = FbAd27 {};
pub struct FbAd27 {}

pub const FTM3_FLT0: Ftm3Flt0 = Ftm3Flt0 {};
pub struct Ftm3Flt0 {}

pub const PTC13: Ptc13 = Ptc13 {};
pub struct Ptc13 {}

pub const UART4_CTS_B: Uart4CtsB = Uart4CtsB {};
pub struct Uart4CtsB {}

pub const FB_AD26: FbAd26 = FbAd26 {};
pub struct FbAd26 {}

pub const PTC14: Ptc14 = Ptc14 {};
pub struct Ptc14 {}

pub const FB_AD25: FbAd25 = FbAd25 {};
pub struct FbAd25 {}

pub const PTC15: Ptc15 = Ptc15 {};
pub struct Ptc15 {}

pub const FB_AD24: FbAd24 = FbAd24 {};
pub struct FbAd24 {}

pub const PTC16: Ptc16 = Ptc16 {};
pub struct Ptc16 {}

pub const FB_CS5_B: FbCs5B = FbCs5B {};
pub struct FbCs5B {}

pub const FB_TSIZ1: FbTsiz1 = FbTsiz1 {};
pub struct FbTsiz1 {}

pub const FB_BE23_16_BLS15_8_B: FbBe2316Bls158B = FbBe2316Bls158B {};
pub struct FbBe2316Bls158B {}

pub const PTC17: Ptc17 = Ptc17 {};
pub struct Ptc17 {}

pub const FB_CS4_B: FbCs4B = FbCs4B {};
pub struct FbCs4B {}

pub const FB_TSIZ0: FbTsiz0 = FbTsiz0 {};
pub struct FbTsiz0 {}

pub const FB_BE31_24_BLS7_0_B: FbBe3124Bls70B = FbBe3124Bls70B {};
pub struct FbBe3124Bls70B {}

pub const PTC18: Ptc18 = Ptc18 {};
pub struct Ptc18 {}

pub const FB_TBST_B: FbTbstB = FbTbstB {};
pub struct FbTbstB {}

pub const FB_CS2_B: FbCs2B = FbCs2B {};
pub struct FbCs2B {}

pub const FB_BE15_8_BLS23_16_B: FbBe158Bls2316B = FbBe158Bls2316B {};
pub struct FbBe158Bls2316B {}

pub const PTC19: Ptc19 = Ptc19 {};
pub struct Ptc19 {}

pub const FB_CS3_B: FbCs3B = FbCs3B {};
pub struct FbCs3B {}

pub const FB_BE7_0_BLS31_24_B: FbBe70Bls3124B = FbBe70Bls3124B {};
pub struct FbBe70Bls3124B {}

pub const FB_TA_B: FbTaB = FbTaB {};
pub struct FbTaB {}

pub const PTD0: Ptd0 = Ptd0 {};
pub struct Ptd0 {}

pub const UART2_RTS_B: Uart2RtsB = Uart2RtsB {};
pub struct Uart2RtsB {}

pub const FTM3_CH0: Ftm3Ch0 = Ftm3Ch0 {};
pub struct Ftm3Ch0 {}

pub const FB_ALE: FbAle = FbAle {};
pub struct FbAle {}

pub const FB_CS1_B: FbCs1B = FbCs1B {};
pub struct FbCs1B {}

pub const FB_TS_B: FbTsB = FbTsB {};
pub struct FbTsB {}

pub const ADC0_SE5B: Adc0Se5b = Adc0Se5b {};
pub struct Adc0Se5b {}

pub const PTD1: Ptd1 = Ptd1 {};
pub struct Ptd1 {}

pub const UART2_CTS_B: Uart2CtsB = Uart2CtsB {};
pub struct Uart2CtsB {}

pub const FTM3_CH1: Ftm3Ch1 = Ftm3Ch1 {};
pub struct Ftm3Ch1 {}

pub const FB_CS0_B: FbCs0B = FbCs0B {};
pub struct FbCs0B {}

pub const PTD2: Ptd2 = Ptd2 {};
pub struct Ptd2 {}

pub const FTM3_CH2: Ftm3Ch2 = Ftm3Ch2 {};
pub struct Ftm3Ch2 {}

pub const FB_AD4: FbAd4 = FbAd4 {};
pub struct FbAd4 {}

pub const PTD3: Ptd3 = Ptd3 {};
pub struct Ptd3 {}

pub const FTM3_CH3: Ftm3Ch3 = Ftm3Ch3 {};
pub struct Ftm3Ch3 {}

pub const FB_AD3: FbAd3 = FbAd3 {};
pub struct FbAd3 {}

pub const PTD4: Ptd4 = Ptd4 {};
pub struct Ptd4 {}

pub const FB_AD2: FbAd2 = FbAd2 {};
pub struct FbAd2 {}

pub const ADC0_SE6B: Adc0Se6b = Adc0Se6b {};
pub struct Adc0Se6b {}

pub const PTD5: Ptd5 = Ptd5 {};
pub struct Ptd5 {}

pub const FB_AD1: FbAd1 = FbAd1 {};
pub struct FbAd1 {}

pub const ADC0_SE7B: Adc0Se7b = Adc0Se7b {};
pub struct Adc0Se7b {}

pub const PTD6: Ptd6 = Ptd6 {};
pub struct Ptd6 {}

pub const FB_AD0: FbAd0 = FbAd0 {};
pub struct FbAd0 {}

pub const PTD7: Ptd7 = Ptd7 {};
pub struct Ptd7 {}

pub const CMT_IRO: CmtIro = CmtIro {};
pub struct CmtIro {}

pub const PTD8: Ptd8 = Ptd8 {};
pub struct Ptd8 {}

pub const FB_A16: FbA16 = FbA16 {};
pub struct FbA16 {}

pub const PTD9: Ptd9 = Ptd9 {};
pub struct Ptd9 {}

pub const FB_A17: FbA17 = FbA17 {};
pub struct FbA17 {}

pub const PTD10: Ptd10 = Ptd10 {};
pub struct Ptd10 {}

pub const UART5_RTS_B: Uart5RtsB = Uart5RtsB {};
pub struct Uart5RtsB {}

pub const FB_A18: FbA18 = FbA18 {};
pub struct FbA18 {}

pub const PTD11: Ptd11 = Ptd11 {};
pub struct Ptd11 {}

pub const UART5_CTS_B: Uart5CtsB = Uart5CtsB {};
pub struct Uart5CtsB {}

pub const SDHC0_CLKIN: Sdhc0Clkin = Sdhc0Clkin {};
pub struct Sdhc0Clkin {}

pub const FB_A19: FbA19 = FbA19 {};
pub struct FbA19 {}

pub const PTD12: Ptd12 = Ptd12 {};
pub struct Ptd12 {}

pub const SDHC0_D4: Sdhc0D4 = Sdhc0D4 {};
pub struct Sdhc0D4 {}

pub const FB_A20: FbA20 = FbA20 {};
pub struct FbA20 {}

pub const PTD13: Ptd13 = Ptd13 {};
pub struct Ptd13 {}

pub const SDHC0_D5: Sdhc0D5 = Sdhc0D5 {};
pub struct Sdhc0D5 {}

pub const FB_A21: FbA21 = FbA21 {};
pub struct FbA21 {}

pub const PTD14: Ptd14 = Ptd14 {};
pub struct Ptd14 {}

pub const SDHC0_D6: Sdhc0D6 = Sdhc0D6 {};
pub struct Sdhc0D6 {}

pub const FB_A22: FbA22 = FbA22 {};
pub struct FbA22 {}

pub const PTD15: Ptd15 = Ptd15 {};
pub struct Ptd15 {}

pub const SPI2_PCS1: Spi2Pcs1 = Spi2Pcs1 {};
pub struct Spi2Pcs1 {}

pub const SDHC0_D7: Sdhc0D7 = Sdhc0D7 {};
pub struct Sdhc0D7 {}

pub const FB_A23: FbA23 = FbA23 {};
pub struct FbA23 {}

pub const ADC1_SE4A: Adc1Se4a = Adc1Se4a {};
pub struct Adc1Se4a {}

pub const PTE0: Pte0 = Pte0 {};
pub struct Pte0 {}

pub const SDHC0_D1: Sdhc0D1 = Sdhc0D1 {};
pub struct Sdhc0D1 {}

pub const RTC_CLKOUT: RtcClkout = RtcClkout {};
pub struct RtcClkout {}

pub const ADC1_SE5A: Adc1Se5a = Adc1Se5a {};
pub struct Adc1Se5a {}

pub const PTE1: Pte1 = Pte1 {};
pub struct Pte1 {}

pub const SDHC0_D0: Sdhc0D0 = Sdhc0D0 {};
pub struct Sdhc0D0 {}

pub const ADC0_DP2: Adc0Dp2 = Adc0Dp2 {};
pub struct Adc0Dp2 {}

pub const ADC1_SE6A: Adc1Se6a = Adc1Se6a {};
pub struct Adc1Se6a {}

pub const PTE2: Pte2 = Pte2 {};
pub struct Pte2 {}

pub const SDHC0_DCLK: Sdhc0Dclk = Sdhc0Dclk {};
pub struct Sdhc0Dclk {}

pub const ADC0_DM2: Adc0Dm2 = Adc0Dm2 {};
pub struct Adc0Dm2 {}

pub const ADC1_SE7A: Adc1Se7a = Adc1Se7a {};
pub struct Adc1Se7a {}

pub const PTE3: Pte3 = Pte3 {};
pub struct Pte3 {}

pub const SDHC0_CMD: Sdhc0Cmd = Sdhc0Cmd {};
pub struct Sdhc0Cmd {}

pub const PTE4: Pte4 = Pte4 {};
pub struct Pte4 {}

pub const SDHC0_D3: Sdhc0D3 = Sdhc0D3 {};
pub struct Sdhc0D3 {}

pub const PTE5: Pte5 = Pte5 {};
pub struct Pte5 {}

pub const SPI1_PCS2: Spi1Pcs2 = Spi1Pcs2 {};
pub struct Spi1Pcs2 {}

pub const SDHC0_D2: Sdhc0D2 = Sdhc0D2 {};
pub struct Sdhc0D2 {}

pub const PTE6: Pte6 = Pte6 {};
pub struct Pte6 {}

pub const SPI1_PCS3: Spi1Pcs3 = Spi1Pcs3 {};
pub struct Spi1Pcs3 {}

pub const PTE7: Pte7 = Pte7 {};
pub struct Pte7 {}

pub const PTE8: Pte8 = Pte8 {};
pub struct Pte8 {}

pub const PTE9: Pte9 = Pte9 {};
pub struct Pte9 {}

pub const PTE10: Pte10 = Pte10 {};
pub struct Pte10 {}

pub const PTE11: Pte11 = Pte11 {};
pub struct Pte11 {}

pub const PTE12: Pte12 = Pte12 {};
pub struct Pte12 {}

pub const ADC0_SE17: Adc0Se17 = Adc0Se17 {};
pub struct Adc0Se17 {}

pub const PTE24: Pte24 = Pte24 {};
pub struct Pte24 {}

pub const ADC0_SE18: Adc0Se18 = Adc0Se18 {};
pub struct Adc0Se18 {}

pub const PTE25: Pte25 = Pte25 {};
pub struct Pte25 {}

pub const PTE26: Pte26 = Pte26 {};
pub struct Pte26 {}

pub const ENET_1588_CLKIN: Enet1588Clkin = Enet1588Clkin {};
pub struct Enet1588Clkin {}

pub const PTE27: Pte27 = Pte27 {};
pub struct Pte27 {}

pub const PTE28: Pte28 = Pte28 {};
pub struct Pte28 {}

