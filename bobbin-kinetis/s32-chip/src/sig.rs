pub trait Signal<T> {}

pub trait Ftm {}
pub trait SignalFtm<T> {}
pub trait Tx {}
pub trait SignalTx<T> {}
pub trait Rx {}
pub trait SignalRx<T> {}
pub trait Cts {}
pub trait SignalCts<T> {}
pub trait Rts {}
pub trait SignalRts<T> {}

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

pub const FTM3_CH0: Ftm3Ch0 = Ftm3Ch0 {};
pub struct Ftm3Ch0 {}
impl Ftm for Ftm3Ch0 {}

pub const FTM3_CH1: Ftm3Ch1 = Ftm3Ch1 {};
pub struct Ftm3Ch1 {}
impl Ftm for Ftm3Ch1 {}

pub const FTM3_CH2: Ftm3Ch2 = Ftm3Ch2 {};
pub struct Ftm3Ch2 {}
impl Ftm for Ftm3Ch2 {}

pub const FTM3_CH3: Ftm3Ch3 = Ftm3Ch3 {};
pub struct Ftm3Ch3 {}
impl Ftm for Ftm3Ch3 {}

pub const FTM3_CH4: Ftm3Ch4 = Ftm3Ch4 {};
pub struct Ftm3Ch4 {}
impl Ftm for Ftm3Ch4 {}

pub const FTM3_CH5: Ftm3Ch5 = Ftm3Ch5 {};
pub struct Ftm3Ch5 {}
impl Ftm for Ftm3Ch5 {}

pub const FTM3_CH6: Ftm3Ch6 = Ftm3Ch6 {};
pub struct Ftm3Ch6 {}
impl Ftm for Ftm3Ch6 {}

pub const FTM3_CH7: Ftm3Ch7 = Ftm3Ch7 {};
pub struct Ftm3Ch7 {}
impl Ftm for Ftm3Ch7 {}

pub const LPUART0_TX: Lpuart0Tx = Lpuart0Tx {};
pub struct Lpuart0Tx {}
impl Tx for Lpuart0Tx {}

pub const LPUART0_RX: Lpuart0Rx = Lpuart0Rx {};
pub struct Lpuart0Rx {}
impl Rx for Lpuart0Rx {}

pub const LPUART0_CTS: Lpuart0Cts = Lpuart0Cts {};
pub struct Lpuart0Cts {}
impl Cts for Lpuart0Cts {}

pub const LPUART0_RTS: Lpuart0Rts = Lpuart0Rts {};
pub struct Lpuart0Rts {}
impl Rts for Lpuart0Rts {}

pub const LPUART1_TX: Lpuart1Tx = Lpuart1Tx {};
pub struct Lpuart1Tx {}
impl Tx for Lpuart1Tx {}

pub const LPUART1_RX: Lpuart1Rx = Lpuart1Rx {};
pub struct Lpuart1Rx {}
impl Rx for Lpuart1Rx {}

pub const LPUART1_CTS: Lpuart1Cts = Lpuart1Cts {};
pub struct Lpuart1Cts {}
impl Cts for Lpuart1Cts {}

pub const LPUART1_RTS: Lpuart1Rts = Lpuart1Rts {};
pub struct Lpuart1Rts {}
impl Rts for Lpuart1Rts {}

pub const LPUART2_TX: Lpuart2Tx = Lpuart2Tx {};
pub struct Lpuart2Tx {}
impl Tx for Lpuart2Tx {}

pub const LPUART2_RX: Lpuart2Rx = Lpuart2Rx {};
pub struct Lpuart2Rx {}
impl Rx for Lpuart2Rx {}

pub const LPUART2_CTS: Lpuart2Cts = Lpuart2Cts {};
pub struct Lpuart2Cts {}
impl Cts for Lpuart2Cts {}

pub const LPUART2_RTS: Lpuart2Rts = Lpuart2Rts {};
pub struct Lpuart2Rts {}
impl Rts for Lpuart2Rts {}

pub const ADC0_SE0: Adc0Se0 = Adc0Se0 {};
pub struct Adc0Se0 {}

pub const CMP0_IN0: Cmp0In0 = Cmp0In0 {};
pub struct Cmp0In0 {}

pub const PTA0: Pta0 = Pta0 {};
pub struct Pta0 {}

pub const LPI2C0_SCLS: Lpi2c0Scls = Lpi2c0Scls {};
pub struct Lpi2c0Scls {}

pub const FXIO_D2: FxioD2 = FxioD2 {};
pub struct FxioD2 {}

pub const FTM2_QD_PHA: Ftm2QdPha = Ftm2QdPha {};
pub struct Ftm2QdPha {}

pub const TRGMUX_OUT3: TrgmuxOut3 = TrgmuxOut3 {};
pub struct TrgmuxOut3 {}

pub const ADC0_SE1: Adc0Se1 = Adc0Se1 {};
pub struct Adc0Se1 {}

pub const CMP0_IN1: Cmp0In1 = Cmp0In1 {};
pub struct Cmp0In1 {}

pub const PTA1: Pta1 = Pta1 {};
pub struct Pta1 {}

pub const LPI2C0_SDAS: Lpi2c0Sdas = Lpi2c0Sdas {};
pub struct Lpi2c0Sdas {}

pub const FXIO_D3: FxioD3 = FxioD3 {};
pub struct FxioD3 {}

pub const FTM1_QD_PHA: Ftm1QdPha = Ftm1QdPha {};
pub struct Ftm1QdPha {}

pub const TRGMUX_OUT0: TrgmuxOut0 = TrgmuxOut0 {};
pub struct TrgmuxOut0 {}

pub const ADC1_SE0: Adc1Se0 = Adc1Se0 {};
pub struct Adc1Se0 {}

pub const PTA2: Pta2 = Pta2 {};
pub struct Pta2 {}

pub const LPI2C0_SDA: Lpi2c0Sda = Lpi2c0Sda {};
pub struct Lpi2c0Sda {}

pub const EWM_OUT_B: EwmOutB = EwmOutB {};
pub struct EwmOutB {}

pub const FXIO_D4: FxioD4 = FxioD4 {};
pub struct FxioD4 {}

pub const ADC1_SE1: Adc1Se1 = Adc1Se1 {};
pub struct Adc1Se1 {}

pub const PTA3: Pta3 = Pta3 {};
pub struct Pta3 {}

pub const LPI2C0_SCL: Lpi2c0Scl = Lpi2c0Scl {};
pub struct Lpi2c0Scl {}

pub const EWM_IN: EwmIn = EwmIn {};
pub struct EwmIn {}

pub const FXIO_D5: FxioD5 = FxioD5 {};
pub struct FxioD5 {}

pub const PTA4: Pta4 = Pta4 {};
pub struct Pta4 {}

pub const CMP0_OUT: Cmp0Out = Cmp0Out {};
pub struct Cmp0Out {}

pub const JTAG_TMS: JtagTms = JtagTms {};
pub struct JtagTms {}

pub const SWD_DIO: SwdDio = SwdDio {};
pub struct SwdDio {}

pub const PTA5: Pta5 = Pta5 {};
pub struct Pta5 {}

pub const TCLK1: Tclk1 = Tclk1 {};
pub struct Tclk1 {}

pub const RESET_B: ResetB = ResetB {};
pub struct ResetB {}

pub const ADC0_SE2: Adc0Se2 = Adc0Se2 {};
pub struct Adc0Se2 {}

pub const PTA6: Pta6 = Pta6 {};
pub struct Pta6 {}

pub const FTM0_FLT1: Ftm0Flt1 = Ftm0Flt1 {};
pub struct Ftm0Flt1 {}

pub const LPSPI1_PCS1: Lpspi1Pcs1 = Lpspi1Pcs1 {};
pub struct Lpspi1Pcs1 {}

pub const ADC0_SE3: Adc0Se3 = Adc0Se3 {};
pub struct Adc0Se3 {}

pub const PTA7: Pta7 = Pta7 {};
pub struct Pta7 {}

pub const FTM0_FLT2: Ftm0Flt2 = Ftm0Flt2 {};
pub struct Ftm0Flt2 {}

pub const RTC_CLKIN: RtcClkin = RtcClkin {};
pub struct RtcClkin {}

pub const PTA8: Pta8 = Pta8 {};
pub struct Pta8 {}

pub const LPSPI2_SOUT: Lpspi2Sout = Lpspi2Sout {};
pub struct Lpspi2Sout {}

pub const FXIO_D6: FxioD6 = FxioD6 {};
pub struct FxioD6 {}

pub const FTM3_FLT3: Ftm3Flt3 = Ftm3Flt3 {};
pub struct Ftm3Flt3 {}

pub const PTA9: Pta9 = Pta9 {};
pub struct Pta9 {}

pub const LPSPI2_PCS0: Lpspi2Pcs0 = Lpspi2Pcs0 {};
pub struct Lpspi2Pcs0 {}

pub const FXIO_D7: FxioD7 = FxioD7 {};
pub struct FxioD7 {}

pub const FTM3_FLT2: Ftm3Flt2 = Ftm3Flt2 {};
pub struct Ftm3Flt2 {}

pub const FTM1_FLT3: Ftm1Flt3 = Ftm1Flt3 {};
pub struct Ftm1Flt3 {}

pub const PTA10: Pta10 = Pta10 {};
pub struct Pta10 {}

pub const FXIO_D0: FxioD0 = FxioD0 {};
pub struct FxioD0 {}

pub const JTAG_TDO: JtagTdo = JtagTdo {};
pub struct JtagTdo {}

pub const NOETM_TRACE_SWO: NoetmTraceSwo = NoetmTraceSwo {};
pub struct NoetmTraceSwo {}

pub const PTA11: Pta11 = Pta11 {};
pub struct Pta11 {}

pub const FXIO_D1: FxioD1 = FxioD1 {};
pub struct FxioD1 {}

pub const CMP0_RRT: Cmp0Rrt = Cmp0Rrt {};
pub struct Cmp0Rrt {}

pub const PTA12: Pta12 = Pta12 {};
pub struct Pta12 {}

pub const CAN1_RX: Can1Rx = Can1Rx {};
pub struct Can1Rx {}

pub const FTM2_QD_PHB: Ftm2QdPhb = Ftm2QdPhb {};
pub struct Ftm2QdPhb {}

pub const PTA13: Pta13 = Pta13 {};
pub struct Pta13 {}

pub const CAN1_TX: Can1Tx = Can1Tx {};
pub struct Can1Tx {}

pub const PTA14: Pta14 = Pta14 {};
pub struct Pta14 {}

pub const FTM0_FLT0: Ftm0Flt0 = Ftm0Flt0 {};
pub struct Ftm0Flt0 {}

pub const FTM3_FLT1: Ftm3Flt1 = Ftm3Flt1 {};
pub struct Ftm3Flt1 {}

pub const FTM1_FLT0: Ftm1Flt0 = Ftm1Flt0 {};
pub struct Ftm1Flt0 {}

pub const ADC1_SE12: Adc1Se12 = Adc1Se12 {};
pub struct Adc1Se12 {}

pub const PTA15: Pta15 = Pta15 {};
pub struct Pta15 {}

pub const LPSPI0_PCS3: Lpspi0Pcs3 = Lpspi0Pcs3 {};
pub struct Lpspi0Pcs3 {}

pub const LPSPI2_PCS3: Lpspi2Pcs3 = Lpspi2Pcs3 {};
pub struct Lpspi2Pcs3 {}

pub const ADC1_SE13: Adc1Se13 = Adc1Se13 {};
pub struct Adc1Se13 {}

pub const PTA16: Pta16 = Pta16 {};
pub struct Pta16 {}

pub const LPSPI1_PCS2: Lpspi1Pcs2 = Lpspi1Pcs2 {};
pub struct Lpspi1Pcs2 {}

pub const PTA17: Pta17 = Pta17 {};
pub struct Pta17 {}

pub const FTM3_FLT0: Ftm3Flt0 = Ftm3Flt0 {};
pub struct Ftm3Flt0 {}

pub const ADC0_SE4: Adc0Se4 = Adc0Se4 {};
pub struct Adc0Se4 {}

pub const ADC1_SE14: Adc1Se14 = Adc1Se14 {};
pub struct Adc1Se14 {}

pub const PTB0: Ptb0 = Ptb0 {};
pub struct Ptb0 {}

pub const LPSPI0_PCS0: Lpspi0Pcs0 = Lpspi0Pcs0 {};
pub struct Lpspi0Pcs0 {}

pub const LPTMR0_ALT3: Lptmr0Alt3 = Lptmr0Alt3 {};
pub struct Lptmr0Alt3 {}

pub const CAN0_RX: Can0Rx = Can0Rx {};
pub struct Can0Rx {}

pub const ADC0_SE5: Adc0Se5 = Adc0Se5 {};
pub struct Adc0Se5 {}

pub const ADC1_SE15: Adc1Se15 = Adc1Se15 {};
pub struct Adc1Se15 {}

pub const PTB1: Ptb1 = Ptb1 {};
pub struct Ptb1 {}

pub const LPSPI0_SOUT: Lpspi0Sout = Lpspi0Sout {};
pub struct Lpspi0Sout {}

pub const TCLK0: Tclk0 = Tclk0 {};
pub struct Tclk0 {}

pub const CAN0_TX: Can0Tx = Can0Tx {};
pub struct Can0Tx {}

pub const ADC0_SE6: Adc0Se6 = Adc0Se6 {};
pub struct Adc0Se6 {}

pub const PTB2: Ptb2 = Ptb2 {};
pub struct Ptb2 {}

pub const LPSPI0_SCK: Lpspi0Sck = Lpspi0Sck {};
pub struct Lpspi0Sck {}

pub const FTM1_QD_PHB: Ftm1QdPhb = Ftm1QdPhb {};
pub struct Ftm1QdPhb {}

pub const TRGMUX_IN3: TrgmuxIn3 = TrgmuxIn3 {};
pub struct TrgmuxIn3 {}

pub const ADC0_SE7: Adc0Se7 = Adc0Se7 {};
pub struct Adc0Se7 {}

pub const PTB3: Ptb3 = Ptb3 {};
pub struct Ptb3 {}

pub const LPSPI0_SIN: Lpspi0Sin = Lpspi0Sin {};
pub struct Lpspi0Sin {}

pub const TRGMUX_IN2: TrgmuxIn2 = TrgmuxIn2 {};
pub struct TrgmuxIn2 {}

pub const PTB4: Ptb4 = Ptb4 {};
pub struct Ptb4 {}

pub const TRGMUX_IN1: TrgmuxIn1 = TrgmuxIn1 {};
pub struct TrgmuxIn1 {}

pub const PTB5: Ptb5 = Ptb5 {};
pub struct Ptb5 {}

pub const LPSPI0_PCS1: Lpspi0Pcs1 = Lpspi0Pcs1 {};
pub struct Lpspi0Pcs1 {}

pub const CLKOUT: Clkout = Clkout {};
pub struct Clkout {}

pub const TRGMUX_IN0: TrgmuxIn0 = TrgmuxIn0 {};
pub struct TrgmuxIn0 {}

pub const XTAL: Xtal = Xtal {};
pub struct Xtal {}

pub const PTB6: Ptb6 = Ptb6 {};
pub struct Ptb6 {}

pub const EXTAL: Extal = Extal {};
pub struct Extal {}

pub const PTB7: Ptb7 = Ptb7 {};
pub struct Ptb7 {}

pub const PTB8: Ptb8 = Ptb8 {};
pub struct Ptb8 {}

pub const PTB9: Ptb9 = Ptb9 {};
pub struct Ptb9 {}

pub const PTB10: Ptb10 = Ptb10 {};
pub struct Ptb10 {}

pub const PTB11: Ptb11 = Ptb11 {};
pub struct Ptb11 {}

pub const LPI2C0_HREQ: Lpi2c0Hreq = Lpi2c0Hreq {};
pub struct Lpi2c0Hreq {}

pub const ADC1_SE7: Adc1Se7 = Adc1Se7 {};
pub struct Adc1Se7 {}

pub const PTB12: Ptb12 = Ptb12 {};
pub struct Ptb12 {}

pub const CAN2_RX: Can2Rx = Can2Rx {};
pub struct Can2Rx {}

pub const ADC1_SE8: Adc1Se8 = Adc1Se8 {};
pub struct Adc1Se8 {}

pub const ADC0_SE8: Adc0Se8 = Adc0Se8 {};
pub struct Adc0Se8 {}

pub const PTB13: Ptb13 = Ptb13 {};
pub struct Ptb13 {}

pub const CAN2_TX: Can2Tx = Can2Tx {};
pub struct Can2Tx {}

pub const ADC1_SE9: Adc1Se9 = Adc1Se9 {};
pub struct Adc1Se9 {}

pub const ADC0_SE9: Adc0Se9 = Adc0Se9 {};
pub struct Adc0Se9 {}

pub const PTB14: Ptb14 = Ptb14 {};
pub struct Ptb14 {}

pub const LPSPI1_SCK: Lpspi1Sck = Lpspi1Sck {};
pub struct Lpspi1Sck {}

pub const PTB15: Ptb15 = Ptb15 {};
pub struct Ptb15 {}

pub const LPSPI1_SIN: Lpspi1Sin = Lpspi1Sin {};
pub struct Lpspi1Sin {}

pub const PTB16: Ptb16 = Ptb16 {};
pub struct Ptb16 {}

pub const LPSPI1_SOUT: Lpspi1Sout = Lpspi1Sout {};
pub struct Lpspi1Sout {}

pub const PTB17: Ptb17 = Ptb17 {};
pub struct Ptb17 {}

pub const LPSPI1_PCS3: Lpspi1Pcs3 = Lpspi1Pcs3 {};
pub struct Lpspi1Pcs3 {}

pub const PTC0: Ptc0 = Ptc0 {};
pub struct Ptc0 {}

pub const LPSPI2_SIN: Lpspi2Sin = Lpspi2Sin {};
pub struct Lpspi2Sin {}

pub const PTC1: Ptc1 = Ptc1 {};
pub struct Ptc1 {}

pub const ADC0_SE10: Adc0Se10 = Adc0Se10 {};
pub struct Adc0Se10 {}

pub const CMP0_IN5: Cmp0In5 = Cmp0In5 {};
pub struct Cmp0In5 {}

pub const PTC2: Ptc2 = Ptc2 {};
pub struct Ptc2 {}

pub const ADC0_SE11: Adc0Se11 = Adc0Se11 {};
pub struct Adc0Se11 {}

pub const CMP0_IN4: Cmp0In4 = Cmp0In4 {};
pub struct Cmp0In4 {}

pub const PTC3: Ptc3 = Ptc3 {};
pub struct Ptc3 {}

pub const CMP0_IN2: Cmp0In2 = Cmp0In2 {};
pub struct Cmp0In2 {}

pub const PTC4: Ptc4 = Ptc4 {};
pub struct Ptc4 {}

pub const RTC_CLKOUT: RtcClkout = RtcClkout {};
pub struct RtcClkout {}

pub const JTAG_TCLK: JtagTclk = JtagTclk {};
pub struct JtagTclk {}

pub const SWD_CLK: SwdClk = SwdClk {};
pub struct SwdClk {}

pub const PTC5: Ptc5 = Ptc5 {};
pub struct Ptc5 {}

pub const JTAG_TDI: JtagTdi = JtagTdi {};
pub struct JtagTdi {}

pub const ADC1_SE4: Adc1Se4 = Adc1Se4 {};
pub struct Adc1Se4 {}

pub const PTC6: Ptc6 = Ptc6 {};
pub struct Ptc6 {}

pub const ADC1_SE5: Adc1Se5 = Adc1Se5 {};
pub struct Adc1Se5 {}

pub const PTC7: Ptc7 = Ptc7 {};
pub struct Ptc7 {}

pub const PTC8: Ptc8 = Ptc8 {};
pub struct Ptc8 {}

pub const PTC9: Ptc9 = Ptc9 {};
pub struct Ptc9 {}

pub const FTM1_FLT1: Ftm1Flt1 = Ftm1Flt1 {};
pub struct Ftm1Flt1 {}

pub const PTC10: Ptc10 = Ptc10 {};
pub struct Ptc10 {}

pub const TRGMUX_IN11: TrgmuxIn11 = TrgmuxIn11 {};
pub struct TrgmuxIn11 {}

pub const PTC11: Ptc11 = Ptc11 {};
pub struct Ptc11 {}

pub const TRGMUX_IN10: TrgmuxIn10 = TrgmuxIn10 {};
pub struct TrgmuxIn10 {}

pub const PTC12: Ptc12 = Ptc12 {};
pub struct Ptc12 {}

pub const PTC13: Ptc13 = Ptc13 {};
pub struct Ptc13 {}

pub const ADC0_SE12: Adc0Se12 = Adc0Se12 {};
pub struct Adc0Se12 {}

pub const PTC14: Ptc14 = Ptc14 {};
pub struct Ptc14 {}

pub const TRGMUX_IN9: TrgmuxIn9 = TrgmuxIn9 {};
pub struct TrgmuxIn9 {}

pub const ADC0_SE13: Adc0Se13 = Adc0Se13 {};
pub struct Adc0Se13 {}

pub const PTC15: Ptc15 = Ptc15 {};
pub struct Ptc15 {}

pub const LPSPI2_SCK: Lpspi2Sck = Lpspi2Sck {};
pub struct Lpspi2Sck {}

pub const TRGMUX_IN8: TrgmuxIn8 = TrgmuxIn8 {};
pub struct TrgmuxIn8 {}

pub const ADC0_SE14: Adc0Se14 = Adc0Se14 {};
pub struct Adc0Se14 {}

pub const PTC16: Ptc16 = Ptc16 {};
pub struct Ptc16 {}

pub const FTM1_FLT2: Ftm1Flt2 = Ftm1Flt2 {};
pub struct Ftm1Flt2 {}

pub const ADC0_SE15: Adc0Se15 = Adc0Se15 {};
pub struct Adc0Se15 {}

pub const PTC17: Ptc17 = Ptc17 {};
pub struct Ptc17 {}

pub const PTD0: Ptd0 = Ptd0 {};
pub struct Ptd0 {}

pub const TRGMUX_OUT1: TrgmuxOut1 = TrgmuxOut1 {};
pub struct TrgmuxOut1 {}

pub const PTD1: Ptd1 = Ptd1 {};
pub struct Ptd1 {}

pub const TRGMUX_OUT2: TrgmuxOut2 = TrgmuxOut2 {};
pub struct TrgmuxOut2 {}

pub const ADC1_SE2: Adc1Se2 = Adc1Se2 {};
pub struct Adc1Se2 {}

pub const PTD2: Ptd2 = Ptd2 {};
pub struct Ptd2 {}

pub const TRGMUX_IN5: TrgmuxIn5 = TrgmuxIn5 {};
pub struct TrgmuxIn5 {}

pub const ADC1_SE3: Adc1Se3 = Adc1Se3 {};
pub struct Adc1Se3 {}

pub const PTD3: Ptd3 = Ptd3 {};
pub struct Ptd3 {}

pub const LPSPI1_PCS0: Lpspi1Pcs0 = Lpspi1Pcs0 {};
pub struct Lpspi1Pcs0 {}

pub const TRGMUX_IN4: TrgmuxIn4 = TrgmuxIn4 {};
pub struct TrgmuxIn4 {}

pub const NMI_B: NmiB = NmiB {};
pub struct NmiB {}

pub const ADC1_SE6: Adc1Se6 = Adc1Se6 {};
pub struct Adc1Se6 {}

pub const PTD4: Ptd4 = Ptd4 {};
pub struct Ptd4 {}

pub const FTM0_FLT3: Ftm0Flt3 = Ftm0Flt3 {};
pub struct Ftm0Flt3 {}

pub const PTD5: Ptd5 = Ptd5 {};
pub struct Ptd5 {}

pub const LPTMR0_ALT2: Lptmr0Alt2 = Lptmr0Alt2 {};
pub struct Lptmr0Alt2 {}

pub const FTM2_FLT1: Ftm2Flt1 = Ftm2Flt1 {};
pub struct Ftm2Flt1 {}

pub const TRGMUX_IN7: TrgmuxIn7 = TrgmuxIn7 {};
pub struct TrgmuxIn7 {}

pub const CMP0_IN7: Cmp0In7 = Cmp0In7 {};
pub struct Cmp0In7 {}

pub const PTD6: Ptd6 = Ptd6 {};
pub struct Ptd6 {}

pub const FTM2_FLT2: Ftm2Flt2 = Ftm2Flt2 {};
pub struct Ftm2Flt2 {}

pub const CMP0_IN6: Cmp0In6 = Cmp0In6 {};
pub struct Cmp0In6 {}

pub const PTD7: Ptd7 = Ptd7 {};
pub struct Ptd7 {}

pub const FTM2_FLT3: Ftm2Flt3 = Ftm2Flt3 {};
pub struct Ftm2Flt3 {}

pub const PTD8: Ptd8 = Ptd8 {};
pub struct Ptd8 {}

pub const PTD9: Ptd9 = Ptd9 {};
pub struct Ptd9 {}

pub const PTD10: Ptd10 = Ptd10 {};
pub struct Ptd10 {}

pub const PTD11: Ptd11 = Ptd11 {};
pub struct Ptd11 {}

pub const PTD12: Ptd12 = Ptd12 {};
pub struct Ptd12 {}

pub const PTD13: Ptd13 = Ptd13 {};
pub struct Ptd13 {}

pub const PTD14: Ptd14 = Ptd14 {};
pub struct Ptd14 {}

pub const PTD15: Ptd15 = Ptd15 {};
pub struct Ptd15 {}

pub const PTD16: Ptd16 = Ptd16 {};
pub struct Ptd16 {}

pub const PTD17: Ptd17 = Ptd17 {};
pub struct Ptd17 {}

pub const PTE0: Pte0 = Pte0 {};
pub struct Pte0 {}

pub const PTE1: Pte1 = Pte1 {};
pub struct Pte1 {}

pub const ADC1_SE10: Adc1Se10 = Adc1Se10 {};
pub struct Adc1Se10 {}

pub const PTE2: Pte2 = Pte2 {};
pub struct Pte2 {}

pub const PTE3: Pte3 = Pte3 {};
pub struct Pte3 {}

pub const FTM2_FLT0: Ftm2Flt0 = Ftm2Flt0 {};
pub struct Ftm2Flt0 {}

pub const TRGMUX_IN6: TrgmuxIn6 = TrgmuxIn6 {};
pub struct TrgmuxIn6 {}

pub const PTE4: Pte4 = Pte4 {};
pub struct Pte4 {}

pub const PTE5: Pte5 = Pte5 {};
pub struct Pte5 {}

pub const TCLK2: Tclk2 = Tclk2 {};
pub struct Tclk2 {}

pub const ADC1_SE11: Adc1Se11 = Adc1Se11 {};
pub struct Adc1Se11 {}

pub const PTE6: Pte6 = Pte6 {};
pub struct Pte6 {}

pub const LPSPI0_PCS2: Lpspi0Pcs2 = Lpspi0Pcs2 {};
pub struct Lpspi0Pcs2 {}

pub const PTE7: Pte7 = Pte7 {};
pub struct Pte7 {}

pub const CMP0_IN3: Cmp0In3 = Cmp0In3 {};
pub struct Cmp0In3 {}

pub const PTE8: Pte8 = Pte8 {};
pub struct Pte8 {}

pub const PTE9: Pte9 = Pte9 {};
pub struct Pte9 {}

pub const PTE10: Pte10 = Pte10 {};
pub struct Pte10 {}

pub const LPSPI2_PCS1: Lpspi2Pcs1 = Lpspi2Pcs1 {};
pub struct Lpspi2Pcs1 {}

pub const TRGMUX_OUT4: TrgmuxOut4 = TrgmuxOut4 {};
pub struct TrgmuxOut4 {}

pub const PTE11: Pte11 = Pte11 {};
pub struct Pte11 {}

pub const LPTMR0_ALT1: Lptmr0Alt1 = Lptmr0Alt1 {};
pub struct Lptmr0Alt1 {}

pub const TRGMUX_OUT5: TrgmuxOut5 = TrgmuxOut5 {};
pub struct TrgmuxOut5 {}

pub const PTE12: Pte12 = Pte12 {};
pub struct Pte12 {}

pub const PTE13: Pte13 = Pte13 {};
pub struct Pte13 {}

pub const LPSPI2_PCS2: Lpspi2Pcs2 = Lpspi2Pcs2 {};
pub struct Lpspi2Pcs2 {}

pub const PTE14: Pte14 = Pte14 {};
pub struct Pte14 {}

pub const PTE15: Pte15 = Pte15 {};
pub struct Pte15 {}

pub const TRGMUX_OUT6: TrgmuxOut6 = TrgmuxOut6 {};
pub struct TrgmuxOut6 {}

pub const PTE16: Pte16 = Pte16 {};
pub struct Pte16 {}

pub const TRGMUX_OUT7: TrgmuxOut7 = TrgmuxOut7 {};
pub struct TrgmuxOut7 {}

