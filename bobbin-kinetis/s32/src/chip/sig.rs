//! Signals

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
pub trait I2cHreq {}
pub trait SignalI2cHreq<T> {}
pub trait I2cScl {}
pub trait SignalI2cScl<T> {}
pub trait I2cSda {}
pub trait SignalI2cSda<T> {}
pub trait I2cScls {}
pub trait SignalI2cScls<T> {}
pub trait I2cSdas {}
pub trait SignalI2cSdas<T> {}
pub trait SpiSck {}
pub trait SignalSpiSck<T> {}
pub trait SpiSout {}
pub trait SignalSpiSout<T> {}
pub trait SpiSin {}
pub trait SignalSpiSin<T> {}
pub trait SpiPcs0 {}
pub trait SignalSpiPcs0<T> {}
pub trait SpiPcs1 {}
pub trait SignalSpiPcs1<T> {}
pub trait SpiPcs2 {}
pub trait SignalSpiPcs2<T> {}
pub trait SpiPcs3 {}
pub trait SignalSpiPcs3<T> {}
pub trait Adc {}
pub trait SignalAdc<T> {}

pub struct Ftm0Ch0 {}
pub struct Ftm0Ch1 {}
pub struct Ftm0Ch2 {}
pub struct Ftm0Ch3 {}
pub struct Ftm0Ch4 {}
pub struct Ftm0Ch5 {}
pub struct Ftm0Ch6 {}
pub struct Ftm0Ch7 {}
pub struct Ftm1Ch0 {}
pub struct Ftm1Ch1 {}
pub struct Ftm1Ch2 {}
pub struct Ftm1Ch3 {}
pub struct Ftm1Ch4 {}
pub struct Ftm1Ch5 {}
pub struct Ftm1Ch6 {}
pub struct Ftm1Ch7 {}
pub struct Ftm2Ch0 {}
pub struct Ftm2Ch1 {}
pub struct Ftm2Ch2 {}
pub struct Ftm2Ch3 {}
pub struct Ftm2Ch4 {}
pub struct Ftm2Ch5 {}
pub struct Ftm2Ch6 {}
pub struct Ftm2Ch7 {}
pub struct Ftm3Ch0 {}
pub struct Ftm3Ch1 {}
pub struct Ftm3Ch2 {}
pub struct Ftm3Ch3 {}
pub struct Ftm3Ch4 {}
pub struct Ftm3Ch5 {}
pub struct Ftm3Ch6 {}
pub struct Ftm3Ch7 {}
pub struct Lpuart0Tx {}
pub struct Lpuart0Rx {}
pub struct Lpuart0Cts {}
pub struct Lpuart0Rts {}
pub struct Lpuart1Tx {}
pub struct Lpuart1Rx {}
pub struct Lpuart1Cts {}
pub struct Lpuart1Rts {}
pub struct Lpuart2Tx {}
pub struct Lpuart2Rx {}
pub struct Lpuart2Cts {}
pub struct Lpuart2Rts {}
pub struct Lpi2c0Hreq {}
pub struct Lpi2c0Scl {}
pub struct Lpi2c0Sda {}
pub struct Lpi2c0Scls {}
pub struct Lpi2c0Sdas {}
pub struct Lpspi0Sck {}
pub struct Lpspi0Sout {}
pub struct Lpspi0Sin {}
pub struct Lpspi0Pcs0 {}
pub struct Lpspi0Pcs1 {}
pub struct Lpspi0Pcs2 {}
pub struct Lpspi0Pcs3 {}
pub struct Lpspi1Sck {}
pub struct Lpspi1Sout {}
pub struct Lpspi1Sin {}
pub struct Lpspi1Pcs0 {}
pub struct Lpspi1Pcs1 {}
pub struct Lpspi1Pcs2 {}
pub struct Lpspi1Pcs3 {}
pub struct Lpspi2Sck {}
pub struct Lpspi2Sout {}
pub struct Lpspi2Sin {}
pub struct Lpspi2Pcs0 {}
pub struct Lpspi2Pcs1 {}
pub struct Lpspi2Pcs2 {}
pub struct Lpspi2Pcs3 {}
pub struct Adc0Se0 {}
pub struct Adc0Se1 {}
pub struct Adc0Se2 {}
pub struct Adc0Se3 {}
pub struct Adc0Se4 {}
pub struct Adc0Se5 {}
pub struct Adc0Se6 {}
pub struct Adc0Se7 {}
pub struct Adc0Se8 {}
pub struct Adc0Se9 {}
pub struct Adc0Se10 {}
pub struct Adc0Se11 {}
pub struct Adc0Se12 {}
pub struct Adc0Se13 {}
pub struct Adc0Se14 {}
pub struct Adc0Se15 {}
pub struct Adc1Se0 {}
pub struct Adc1Se1 {}
pub struct Adc1Se2 {}
pub struct Adc1Se3 {}
pub struct Adc1Se4 {}
pub struct Adc1Se5 {}
pub struct Adc1Se6 {}
pub struct Adc1Se7 {}
pub struct Adc1Se8 {}
pub struct Adc1Se9 {}
pub struct Adc1Se10 {}
pub struct Adc1Se11 {}
pub struct Adc1Se12 {}
pub struct Adc1Se13 {}
pub struct Adc1Se14 {}
pub struct Adc1Se15 {}
pub struct Cmp0In0 {}
pub struct Pta0 {}
pub struct FxioD2 {}
pub struct Ftm2QdPha {}
pub struct TrgmuxOut3 {}
pub struct Cmp0In1 {}
pub struct Pta1 {}
pub struct FxioD3 {}
pub struct Ftm1QdPha {}
pub struct TrgmuxOut0 {}
pub struct Pta2 {}
pub struct EwmOutB {}
pub struct FxioD4 {}
pub struct Pta3 {}
pub struct EwmIn {}
pub struct FxioD5 {}
pub struct Pta4 {}
pub struct Cmp0Out {}
pub struct JtagTms {}
pub struct SwdDio {}
pub struct Pta5 {}
pub struct Tclk1 {}
pub struct ResetB {}
pub struct Pta6 {}
pub struct Ftm0Flt1 {}
pub struct Pta7 {}
pub struct Ftm0Flt2 {}
pub struct RtcClkin {}
pub struct Pta8 {}
pub struct FxioD6 {}
pub struct Ftm3Flt3 {}
pub struct Pta9 {}
pub struct FxioD7 {}
pub struct Ftm3Flt2 {}
pub struct Ftm1Flt3 {}
pub struct Pta10 {}
pub struct FxioD0 {}
pub struct JtagTdo {}
pub struct NoetmTraceSwo {}
pub struct Pta11 {}
pub struct FxioD1 {}
pub struct Cmp0Rrt {}
pub struct Pta12 {}
pub struct Can1Rx {}
pub struct Ftm2QdPhb {}
pub struct Pta13 {}
pub struct Can1Tx {}
pub struct Pta14 {}
pub struct Ftm0Flt0 {}
pub struct Ftm3Flt1 {}
pub struct Ftm1Flt0 {}
pub struct Pta15 {}
pub struct Pta16 {}
pub struct Pta17 {}
pub struct Ftm3Flt0 {}
pub struct Ptb0 {}
pub struct Lptmr0Alt3 {}
pub struct Can0Rx {}
pub struct Ptb1 {}
pub struct Tclk0 {}
pub struct Can0Tx {}
pub struct Ptb2 {}
pub struct Ftm1QdPhb {}
pub struct TrgmuxIn3 {}
pub struct Ptb3 {}
pub struct TrgmuxIn2 {}
pub struct Ptb4 {}
pub struct TrgmuxIn1 {}
pub struct Ptb5 {}
pub struct Clkout {}
pub struct TrgmuxIn0 {}
pub struct Xtal {}
pub struct Ptb6 {}
pub struct Extal {}
pub struct Ptb7 {}
pub struct Ptb8 {}
pub struct Ptb9 {}
pub struct Ptb10 {}
pub struct Ptb11 {}
pub struct Ptb12 {}
pub struct Can2Rx {}
pub struct Ptb13 {}
pub struct Can2Tx {}
pub struct Ptb14 {}
pub struct Ptb15 {}
pub struct Ptb16 {}
pub struct Ptb17 {}
pub struct Ptc0 {}
pub struct Ptc1 {}
pub struct Cmp0In5 {}
pub struct Ptc2 {}
pub struct Cmp0In4 {}
pub struct Ptc3 {}
pub struct Cmp0In2 {}
pub struct Ptc4 {}
pub struct RtcClkout {}
pub struct JtagTclk {}
pub struct SwdClk {}
pub struct Ptc5 {}
pub struct JtagTdi {}
pub struct Ptc6 {}
pub struct Ptc7 {}
pub struct Ptc8 {}
pub struct Ptc9 {}
pub struct Ftm1Flt1 {}
pub struct Ptc10 {}
pub struct TrgmuxIn11 {}
pub struct Ptc11 {}
pub struct TrgmuxIn10 {}
pub struct Ptc12 {}
pub struct Ptc13 {}
pub struct Ptc14 {}
pub struct TrgmuxIn9 {}
pub struct Ptc15 {}
pub struct TrgmuxIn8 {}
pub struct Ptc16 {}
pub struct Ftm1Flt2 {}
pub struct Ptc17 {}
pub struct Ptd0 {}
pub struct TrgmuxOut1 {}
pub struct Ptd1 {}
pub struct TrgmuxOut2 {}
pub struct Ptd2 {}
pub struct TrgmuxIn5 {}
pub struct Ptd3 {}
pub struct TrgmuxIn4 {}
pub struct NmiB {}
pub struct Ptd4 {}
pub struct Ftm0Flt3 {}
pub struct Ptd5 {}
pub struct Lptmr0Alt2 {}
pub struct Ftm2Flt1 {}
pub struct TrgmuxIn7 {}
pub struct Cmp0In7 {}
pub struct Ptd6 {}
pub struct Ftm2Flt2 {}
pub struct Cmp0In6 {}
pub struct Ptd7 {}
pub struct Ftm2Flt3 {}
pub struct Ptd8 {}
pub struct Ptd9 {}
pub struct Ptd10 {}
pub struct Ptd11 {}
pub struct Ptd12 {}
pub struct Ptd13 {}
pub struct Ptd14 {}
pub struct Ptd15 {}
pub struct Ptd16 {}
pub struct Ptd17 {}
pub struct Pte0 {}
pub struct Pte1 {}
pub struct Pte2 {}
pub struct Pte3 {}
pub struct Ftm2Flt0 {}
pub struct TrgmuxIn6 {}
pub struct Pte4 {}
pub struct Pte5 {}
pub struct Tclk2 {}
pub struct Pte6 {}
pub struct Pte7 {}
pub struct Cmp0In3 {}
pub struct Pte8 {}
pub struct Pte9 {}
pub struct Pte10 {}
pub struct TrgmuxOut4 {}
pub struct Pte11 {}
pub struct Lptmr0Alt1 {}
pub struct TrgmuxOut5 {}
pub struct Pte12 {}
pub struct Pte13 {}
pub struct Pte14 {}
pub struct Pte15 {}
pub struct TrgmuxOut6 {}
pub struct Pte16 {}
pub struct TrgmuxOut7 {}
