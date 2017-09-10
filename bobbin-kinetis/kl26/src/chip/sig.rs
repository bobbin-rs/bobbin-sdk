//! Signals

pub trait Signal<T> {}

pub trait Tpm {}
pub trait SignalTpm<T> {}
pub trait LptmrAlt1 {}
pub trait SignalLptmrAlt1<T> {}
pub trait LptmrAlt2 {}
pub trait SignalLptmrAlt2<T> {}
pub trait SpiSck {}
pub trait SignalSpiSck<T> {}
pub trait SpiMiso {}
pub trait SignalSpiMiso<T> {}
pub trait SpiMosi {}
pub trait SignalSpiMosi<T> {}
pub trait SpiPcs0 {}
pub trait SignalSpiPcs0<T> {}
pub trait SpiSout {}
pub trait SignalSpiSout<T> {}
pub trait SpiSin {}
pub trait SignalSpiSin<T> {}
pub trait SpiPcs1 {}
pub trait SignalSpiPcs1<T> {}
pub trait SpiPcs2 {}
pub trait SignalSpiPcs2<T> {}
pub trait SpiPcs3 {}
pub trait SignalSpiPcs3<T> {}
pub trait I2cScl {}
pub trait SignalI2cScl<T> {}
pub trait I2cSda {}
pub trait SignalI2cSda<T> {}
pub trait Tx {}
pub trait SignalTx<T> {}
pub trait Rx {}
pub trait SignalRx<T> {}
pub trait Adc {}
pub trait SignalAdc<T> {}

pub struct Tpm0Ch0 {}
pub struct Tpm0Ch1 {}
pub struct Tpm0Ch2 {}
pub struct Tpm0Ch3 {}
pub struct Tpm0Ch4 {}
pub struct Tpm0Ch5 {}
pub struct Tpm1Ch0 {}
pub struct Tpm1Ch1 {}
pub struct Tpm1Ch2 {}
pub struct Tpm1Ch3 {}
pub struct Tpm1Ch4 {}
pub struct Tpm1Ch5 {}
pub struct Tpm2Ch0 {}
pub struct Tpm2Ch1 {}
pub struct Tpm2Ch2 {}
pub struct Tpm2Ch3 {}
pub struct Tpm2Ch4 {}
pub struct Tpm2Ch5 {}
pub struct Lptmr0Alt1 {}
pub struct Lptmr0Alt2 {}
pub struct Spi0Sck {}
pub struct Spi0Miso {}
pub struct Spi0Mosi {}
pub struct Spi0Pcs0 {}
pub struct Spi1Sck {}
pub struct Spi1Sout {}
pub struct Spi1Sin {}
pub struct Spi1Pcs0 {}
pub struct Spi1Pcs1 {}
pub struct Spi1Pcs2 {}
pub struct Spi1Pcs3 {}
pub struct I2c0Scl {}
pub struct I2c0Sda {}
pub struct I2c1Scl {}
pub struct I2c1Sda {}
pub struct Uart0Tx {}
pub struct Uart0Rx {}
pub struct Uart1Tx {}
pub struct Uart1Rx {}
pub struct Uart2Tx {}
pub struct Uart2Rx {}
pub struct Adc0Dp0 {}
pub struct Adc0Dm0 {}
pub struct Adc0Dp1 {}
pub struct Adc0Dm1 {}
pub struct Adc0Dp2 {}
pub struct Adc0Dm2 {}
pub struct Adc0Dp3 {}
pub struct Adc0Dm3 {}
pub struct Adc0Se4a {}
pub struct Adc0Se4b {}
pub struct Adc0Se5a {}
pub struct Adc0Se5b {}
pub struct Adc0Se6a {}
pub struct Adc0Se6b {}
pub struct Adc0Se7a {}
pub struct Adc0Se7b {}
pub struct Adc0Se8 {}
pub struct Adc0Se9 {}
pub struct Adc0Se10 {}
pub struct Adc0Se11 {}
pub struct Adc0Se12 {}
pub struct Adc0Se13 {}
pub struct Adc0Se14 {}
pub struct Adc0Se15 {}
pub struct Adc0Se16 {}
pub struct Adc0Se17 {}
pub struct Adc0Se18 {}
pub struct Adc0Se19 {}
pub struct Adc0Se20 {}
pub struct Adc0Se21 {}
pub struct Adc0Se22 {}
pub struct Adc0Se23 {}
pub struct Tsi0Ch1 {}
pub struct Pta0 {}
pub struct SwdClk {}
pub struct Tsi0Ch2 {}
pub struct Pta1 {}
pub struct Tsi0Ch3 {}
pub struct Pta2 {}
pub struct Tsi0Ch4 {}
pub struct Pta3 {}
pub struct SwdDio {}
pub struct Tsi0Ch5 {}
pub struct Pta4 {}
pub struct NmiB {}
pub struct Pta5 {}
pub struct UsbClkin {}
pub struct I2s0TxBclk {}
pub struct Pta12 {}
pub struct I2s0Txd0 {}
pub struct Pta13 {}
pub struct I2s0TxFs {}
pub struct Extal0 {}
pub struct Pta18 {}
pub struct TpmClkin0 {}
pub struct Xtal0 {}
pub struct Pta19 {}
pub struct TpmClkin1 {}
pub struct Pta20 {}
pub struct ResetB {}
pub struct Tsi0Ch0 {}
pub struct Ptb0 {}
pub struct Tsi0Ch6 {}
pub struct Ptb1 {}
pub struct Tsi0Ch7 {}
pub struct Ptb2 {}
pub struct Tsi0Ch8 {}
pub struct Ptb3 {}
pub struct Tsi0Ch9 {}
pub struct Ptb16 {}
pub struct Spi1Mosi {}
pub struct Spi1Miso {}
pub struct Tsi0Ch10 {}
pub struct Ptb17 {}
pub struct Tsi0Ch11 {}
pub struct Ptb18 {}
pub struct Tsi0Ch12 {}
pub struct Ptb19 {}
pub struct Tsi0Ch13 {}
pub struct Ptc0 {}
pub struct ExtrgIn {}
pub struct AudiousbSofOut {}
pub struct Cmp0Out {}
pub struct Tsi0Ch14 {}
pub struct Ptc1 {}
pub struct Tsi0Ch15 {}
pub struct Ptc2 {}
pub struct Ptc3 {}
pub struct Clkout {}
pub struct Ptc4 {}
pub struct I2s0Mclk {}
pub struct Ptc5 {}
pub struct I2s0Rxd0 {}
pub struct Cmp0In0 {}
pub struct Ptc6 {}
pub struct I2s0RxBclk {}
pub struct Cmp0In1 {}
pub struct Ptc7 {}
pub struct I2s0RxFs {}
pub struct Cmp0In2 {}
pub struct Ptc8 {}
pub struct Cmp0In3 {}
pub struct Ptc9 {}
pub struct Ptc10 {}
pub struct Ptc11 {}
pub struct Ptd0 {}
pub struct Ptd1 {}
pub struct Ptd2 {}
pub struct Ptd3 {}
pub struct Ptd4 {}
pub struct Ptd5 {}
pub struct Ptd6 {}
pub struct Ptd7 {}
pub struct Pte0 {}
pub struct RtcClkout {}
pub struct Pte1 {}
pub struct Adc0Se0 {}
pub struct Pte20 {}
pub struct Pte21 {}
pub struct Adc0Se3 {}
pub struct Pte22 {}
pub struct Pte23 {}
pub struct Pte24 {}
pub struct Pte25 {}
pub struct Cmp0In5 {}
pub struct Pte29 {}
pub struct Dac0Out {}
pub struct Cmp0In4 {}
pub struct Pte30 {}
pub struct Pte31 {}
