//! Signals

pub trait Signal<T> {}

pub trait Tpm {}
pub trait SignalTpm<T> {}
pub trait Tx {}
pub trait SignalTx<T> {}
pub trait Rx {}
pub trait SignalRx<T> {}
pub trait AdcDp0 {}
pub trait SignalAdcDp0<T> {}
pub trait AdcDp1 {}
pub trait SignalAdcDp1<T> {}
pub trait AdcDp2 {}
pub trait SignalAdcDp2<T> {}
pub trait AdcDp3 {}
pub trait SignalAdcDp3<T> {}
pub trait AdcDm0 {}
pub trait SignalAdcDm0<T> {}
pub trait AdcDm1 {}
pub trait SignalAdcDm1<T> {}
pub trait AdcDm2 {}
pub trait SignalAdcDm2<T> {}
pub trait AdcDm3 {}
pub trait SignalAdcDm3<T> {}
pub trait AdcSe4a {}
pub trait SignalAdcSe4a<T> {}
pub trait AdcSe5a {}
pub trait SignalAdcSe5a<T> {}
pub trait AdcSe6a {}
pub trait SignalAdcSe6a<T> {}
pub trait AdcSe7a {}
pub trait SignalAdcSe7a<T> {}
pub trait AdcSe4b {}
pub trait SignalAdcSe4b<T> {}
pub trait AdcSe5b {}
pub trait SignalAdcSe5b<T> {}
pub trait AdcSe6b {}
pub trait SignalAdcSe6b<T> {}
pub trait AdcSe7b {}
pub trait SignalAdcSe7b<T> {}
pub trait AdcSe8 {}
pub trait SignalAdcSe8<T> {}
pub trait AdcSe9 {}
pub trait SignalAdcSe9<T> {}
pub trait AdcSe10 {}
pub trait SignalAdcSe10<T> {}
pub trait AdcSe11 {}
pub trait SignalAdcSe11<T> {}
pub trait AdcSe12 {}
pub trait SignalAdcSe12<T> {}
pub trait AdcSe13 {}
pub trait SignalAdcSe13<T> {}
pub trait AdcSe14 {}
pub trait SignalAdcSe14<T> {}
pub trait AdcSe15 {}
pub trait SignalAdcSe15<T> {}
pub trait AdcSe16 {}
pub trait SignalAdcSe16<T> {}
pub trait AdcSe17 {}
pub trait SignalAdcSe17<T> {}
pub trait AdcSe18 {}
pub trait SignalAdcSe18<T> {}
pub trait AdcSe19 {}
pub trait SignalAdcSe19<T> {}
pub trait AdcSe20 {}
pub trait SignalAdcSe20<T> {}
pub trait AdcSe21 {}
pub trait SignalAdcSe21<T> {}
pub trait AdcSe22 {}
pub trait SignalAdcSe22<T> {}
pub trait AdcSe23 {}
pub trait SignalAdcSe23<T> {}

pub const TPM0_CH0: Tpm0Ch0 = Tpm0Ch0 {};
pub struct Tpm0Ch0 {}
impl Tpm for Tpm0Ch0 {}

pub const TPM0_CH1: Tpm0Ch1 = Tpm0Ch1 {};
pub struct Tpm0Ch1 {}
impl Tpm for Tpm0Ch1 {}

pub const TPM0_CH2: Tpm0Ch2 = Tpm0Ch2 {};
pub struct Tpm0Ch2 {}
impl Tpm for Tpm0Ch2 {}

pub const TPM0_CH3: Tpm0Ch3 = Tpm0Ch3 {};
pub struct Tpm0Ch3 {}
impl Tpm for Tpm0Ch3 {}

pub const TPM0_CH4: Tpm0Ch4 = Tpm0Ch4 {};
pub struct Tpm0Ch4 {}
impl Tpm for Tpm0Ch4 {}

pub const TPM0_CH5: Tpm0Ch5 = Tpm0Ch5 {};
pub struct Tpm0Ch5 {}
impl Tpm for Tpm0Ch5 {}

pub const TPM1_CH0: Tpm1Ch0 = Tpm1Ch0 {};
pub struct Tpm1Ch0 {}
impl Tpm for Tpm1Ch0 {}

pub const TPM1_CH1: Tpm1Ch1 = Tpm1Ch1 {};
pub struct Tpm1Ch1 {}
impl Tpm for Tpm1Ch1 {}

pub const TPM1_CH2: Tpm1Ch2 = Tpm1Ch2 {};
pub struct Tpm1Ch2 {}
impl Tpm for Tpm1Ch2 {}

pub const TPM1_CH3: Tpm1Ch3 = Tpm1Ch3 {};
pub struct Tpm1Ch3 {}
impl Tpm for Tpm1Ch3 {}

pub const TPM1_CH4: Tpm1Ch4 = Tpm1Ch4 {};
pub struct Tpm1Ch4 {}
impl Tpm for Tpm1Ch4 {}

pub const TPM1_CH5: Tpm1Ch5 = Tpm1Ch5 {};
pub struct Tpm1Ch5 {}
impl Tpm for Tpm1Ch5 {}

pub const TPM2_CH0: Tpm2Ch0 = Tpm2Ch0 {};
pub struct Tpm2Ch0 {}
impl Tpm for Tpm2Ch0 {}

pub const TPM2_CH1: Tpm2Ch1 = Tpm2Ch1 {};
pub struct Tpm2Ch1 {}
impl Tpm for Tpm2Ch1 {}

pub const TPM2_CH2: Tpm2Ch2 = Tpm2Ch2 {};
pub struct Tpm2Ch2 {}
impl Tpm for Tpm2Ch2 {}

pub const TPM2_CH3: Tpm2Ch3 = Tpm2Ch3 {};
pub struct Tpm2Ch3 {}
impl Tpm for Tpm2Ch3 {}

pub const TPM2_CH4: Tpm2Ch4 = Tpm2Ch4 {};
pub struct Tpm2Ch4 {}
impl Tpm for Tpm2Ch4 {}

pub const TPM2_CH5: Tpm2Ch5 = Tpm2Ch5 {};
pub struct Tpm2Ch5 {}
impl Tpm for Tpm2Ch5 {}

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

pub const ADC0_DP0: Adc0Dp0 = Adc0Dp0 {};
pub struct Adc0Dp0 {}
impl AdcDp0 for Adc0Dp0 {}

pub const ADC0_DP1: Adc0Dp1 = Adc0Dp1 {};
pub struct Adc0Dp1 {}
impl AdcDp1 for Adc0Dp1 {}

pub const ADC0_DP2: Adc0Dp2 = Adc0Dp2 {};
pub struct Adc0Dp2 {}
impl AdcDp2 for Adc0Dp2 {}

pub const ADC0_DP3: Adc0Dp3 = Adc0Dp3 {};
pub struct Adc0Dp3 {}
impl AdcDp3 for Adc0Dp3 {}

pub const ADC0_DM0: Adc0Dm0 = Adc0Dm0 {};
pub struct Adc0Dm0 {}
impl AdcDm0 for Adc0Dm0 {}

pub const ADC0_DM1: Adc0Dm1 = Adc0Dm1 {};
pub struct Adc0Dm1 {}
impl AdcDm1 for Adc0Dm1 {}

pub const ADC0_DM2: Adc0Dm2 = Adc0Dm2 {};
pub struct Adc0Dm2 {}
impl AdcDm2 for Adc0Dm2 {}

pub const ADC0_DM3: Adc0Dm3 = Adc0Dm3 {};
pub struct Adc0Dm3 {}
impl AdcDm3 for Adc0Dm3 {}

pub const ADC0_SE4A: Adc0Se4a = Adc0Se4a {};
pub struct Adc0Se4a {}
impl AdcSe4a for Adc0Se4a {}

pub const ADC0_SE5A: Adc0Se5a = Adc0Se5a {};
pub struct Adc0Se5a {}
impl AdcSe5a for Adc0Se5a {}

pub const ADC0_SE6A: Adc0Se6a = Adc0Se6a {};
pub struct Adc0Se6a {}
impl AdcSe6a for Adc0Se6a {}

pub const ADC0_SE7A: Adc0Se7a = Adc0Se7a {};
pub struct Adc0Se7a {}
impl AdcSe7a for Adc0Se7a {}

pub const ADC0_SE4B: Adc0Se4b = Adc0Se4b {};
pub struct Adc0Se4b {}
impl AdcSe4b for Adc0Se4b {}

pub const ADC0_SE5B: Adc0Se5b = Adc0Se5b {};
pub struct Adc0Se5b {}
impl AdcSe5b for Adc0Se5b {}

pub const ADC0_SE6B: Adc0Se6b = Adc0Se6b {};
pub struct Adc0Se6b {}
impl AdcSe6b for Adc0Se6b {}

pub const ADC0_SE7B: Adc0Se7b = Adc0Se7b {};
pub struct Adc0Se7b {}
impl AdcSe7b for Adc0Se7b {}

pub const ADC0_SE8: Adc0Se8 = Adc0Se8 {};
pub struct Adc0Se8 {}
impl AdcSe8 for Adc0Se8 {}

pub const ADC0_SE9: Adc0Se9 = Adc0Se9 {};
pub struct Adc0Se9 {}
impl AdcSe9 for Adc0Se9 {}

pub const ADC0_SE10: Adc0Se10 = Adc0Se10 {};
pub struct Adc0Se10 {}
impl AdcSe10 for Adc0Se10 {}

pub const ADC0_SE11: Adc0Se11 = Adc0Se11 {};
pub struct Adc0Se11 {}
impl AdcSe11 for Adc0Se11 {}

pub const ADC0_SE12: Adc0Se12 = Adc0Se12 {};
pub struct Adc0Se12 {}
impl AdcSe12 for Adc0Se12 {}

pub const ADC0_SE13: Adc0Se13 = Adc0Se13 {};
pub struct Adc0Se13 {}
impl AdcSe13 for Adc0Se13 {}

pub const ADC0_SE14: Adc0Se14 = Adc0Se14 {};
pub struct Adc0Se14 {}
impl AdcSe14 for Adc0Se14 {}

pub const ADC0_SE15: Adc0Se15 = Adc0Se15 {};
pub struct Adc0Se15 {}
impl AdcSe15 for Adc0Se15 {}

pub const ADC0_SE16: Adc0Se16 = Adc0Se16 {};
pub struct Adc0Se16 {}
impl AdcSe16 for Adc0Se16 {}

pub const ADC0_SE17: Adc0Se17 = Adc0Se17 {};
pub struct Adc0Se17 {}
impl AdcSe17 for Adc0Se17 {}

pub const ADC0_SE18: Adc0Se18 = Adc0Se18 {};
pub struct Adc0Se18 {}
impl AdcSe18 for Adc0Se18 {}

pub const ADC0_SE19: Adc0Se19 = Adc0Se19 {};
pub struct Adc0Se19 {}
impl AdcSe19 for Adc0Se19 {}

pub const ADC0_SE20: Adc0Se20 = Adc0Se20 {};
pub struct Adc0Se20 {}
impl AdcSe20 for Adc0Se20 {}

pub const ADC0_SE21: Adc0Se21 = Adc0Se21 {};
pub struct Adc0Se21 {}
impl AdcSe21 for Adc0Se21 {}

pub const ADC0_SE22: Adc0Se22 = Adc0Se22 {};
pub struct Adc0Se22 {}
impl AdcSe22 for Adc0Se22 {}

pub const ADC0_SE23: Adc0Se23 = Adc0Se23 {};
pub struct Adc0Se23 {}
impl AdcSe23 for Adc0Se23 {}

pub const TSI0_CH1: Tsi0Ch1 = Tsi0Ch1 {};
pub struct Tsi0Ch1 {}

pub const PTA0: Pta0 = Pta0 {};
pub struct Pta0 {}

pub const SWD_CLK: SwdClk = SwdClk {};
pub struct SwdClk {}

pub const TSI0_CH2: Tsi0Ch2 = Tsi0Ch2 {};
pub struct Tsi0Ch2 {}

pub const PTA1: Pta1 = Pta1 {};
pub struct Pta1 {}

pub const TSI0_CH3: Tsi0Ch3 = Tsi0Ch3 {};
pub struct Tsi0Ch3 {}

pub const PTA2: Pta2 = Pta2 {};
pub struct Pta2 {}

pub const TSI0_CH4: Tsi0Ch4 = Tsi0Ch4 {};
pub struct Tsi0Ch4 {}

pub const PTA3: Pta3 = Pta3 {};
pub struct Pta3 {}

pub const I2C1_SCL: I2c1Scl = I2c1Scl {};
pub struct I2c1Scl {}

pub const SWD_DIO: SwdDio = SwdDio {};
pub struct SwdDio {}

pub const TSI0_CH5: Tsi0Ch5 = Tsi0Ch5 {};
pub struct Tsi0Ch5 {}

pub const PTA4: Pta4 = Pta4 {};
pub struct Pta4 {}

pub const I2C1_SDA: I2c1Sda = I2c1Sda {};
pub struct I2c1Sda {}

pub const NMI_B: NmiB = NmiB {};
pub struct NmiB {}

pub const PTA5: Pta5 = Pta5 {};
pub struct Pta5 {}

pub const USB_CLKIN: UsbClkin = UsbClkin {};
pub struct UsbClkin {}

pub const I2S0_TX_BCLK: I2s0TxBclk = I2s0TxBclk {};
pub struct I2s0TxBclk {}

pub const PTA12: Pta12 = Pta12 {};
pub struct Pta12 {}

pub const I2S0_TXD0: I2s0Txd0 = I2s0Txd0 {};
pub struct I2s0Txd0 {}

pub const PTA13: Pta13 = Pta13 {};
pub struct Pta13 {}

pub const I2S0_TX_FS: I2s0TxFs = I2s0TxFs {};
pub struct I2s0TxFs {}

pub const EXTAL0: Extal0 = Extal0 {};
pub struct Extal0 {}

pub const PTA18: Pta18 = Pta18 {};
pub struct Pta18 {}

pub const TPM_CLKIN0: TpmClkin0 = TpmClkin0 {};
pub struct TpmClkin0 {}

pub const XTAL0: Xtal0 = Xtal0 {};
pub struct Xtal0 {}

pub const PTA19: Pta19 = Pta19 {};
pub struct Pta19 {}

pub const TPM_CLKIN1: TpmClkin1 = TpmClkin1 {};
pub struct TpmClkin1 {}

pub const LPTMR0_ALT1: Lptmr0Alt1 = Lptmr0Alt1 {};
pub struct Lptmr0Alt1 {}

pub const PTA20: Pta20 = Pta20 {};
pub struct Pta20 {}

pub const RESET_B: ResetB = ResetB {};
pub struct ResetB {}

pub const TSI0_CH0: Tsi0Ch0 = Tsi0Ch0 {};
pub struct Tsi0Ch0 {}

pub const PTB0: Ptb0 = Ptb0 {};
pub struct Ptb0 {}

pub const I2C0_SCL: I2c0Scl = I2c0Scl {};
pub struct I2c0Scl {}

pub const TSI0_CH6: Tsi0Ch6 = Tsi0Ch6 {};
pub struct Tsi0Ch6 {}

pub const PTB1: Ptb1 = Ptb1 {};
pub struct Ptb1 {}

pub const I2C0_SDA: I2c0Sda = I2c0Sda {};
pub struct I2c0Sda {}

pub const TSI0_CH7: Tsi0Ch7 = Tsi0Ch7 {};
pub struct Tsi0Ch7 {}

pub const PTB2: Ptb2 = Ptb2 {};
pub struct Ptb2 {}

pub const TSI0_CH8: Tsi0Ch8 = Tsi0Ch8 {};
pub struct Tsi0Ch8 {}

pub const PTB3: Ptb3 = Ptb3 {};
pub struct Ptb3 {}

pub const TSI0_CH9: Tsi0Ch9 = Tsi0Ch9 {};
pub struct Tsi0Ch9 {}

pub const PTB16: Ptb16 = Ptb16 {};
pub struct Ptb16 {}

pub const SPI1_MOSI: Spi1Mosi = Spi1Mosi {};
pub struct Spi1Mosi {}

pub const SPI1_MISO: Spi1Miso = Spi1Miso {};
pub struct Spi1Miso {}

pub const TSI0_CH10: Tsi0Ch10 = Tsi0Ch10 {};
pub struct Tsi0Ch10 {}

pub const PTB17: Ptb17 = Ptb17 {};
pub struct Ptb17 {}

pub const TSI0_CH11: Tsi0Ch11 = Tsi0Ch11 {};
pub struct Tsi0Ch11 {}

pub const PTB18: Ptb18 = Ptb18 {};
pub struct Ptb18 {}

pub const TSI0_CH12: Tsi0Ch12 = Tsi0Ch12 {};
pub struct Tsi0Ch12 {}

pub const PTB19: Ptb19 = Ptb19 {};
pub struct Ptb19 {}

pub const TSI0_CH13: Tsi0Ch13 = Tsi0Ch13 {};
pub struct Tsi0Ch13 {}

pub const PTC0: Ptc0 = Ptc0 {};
pub struct Ptc0 {}

pub const EXTRG_IN: ExtrgIn = ExtrgIn {};
pub struct ExtrgIn {}

pub const AUDIOUSB_SOF_OUT: AudiousbSofOut = AudiousbSofOut {};
pub struct AudiousbSofOut {}

pub const CMP0_OUT: Cmp0Out = Cmp0Out {};
pub struct Cmp0Out {}

pub const TSI0_CH14: Tsi0Ch14 = Tsi0Ch14 {};
pub struct Tsi0Ch14 {}

pub const PTC1: Ptc1 = Ptc1 {};
pub struct Ptc1 {}

pub const TSI0_CH15: Tsi0Ch15 = Tsi0Ch15 {};
pub struct Tsi0Ch15 {}

pub const PTC2: Ptc2 = Ptc2 {};
pub struct Ptc2 {}

pub const PTC3: Ptc3 = Ptc3 {};
pub struct Ptc3 {}

pub const CLKOUT: Clkout = Clkout {};
pub struct Clkout {}

pub const PTC4: Ptc4 = Ptc4 {};
pub struct Ptc4 {}

pub const SPI0_PCS0: Spi0Pcs0 = Spi0Pcs0 {};
pub struct Spi0Pcs0 {}

pub const I2S0_MCLK: I2s0Mclk = I2s0Mclk {};
pub struct I2s0Mclk {}

pub const PTC5: Ptc5 = Ptc5 {};
pub struct Ptc5 {}

pub const SPI0_SCK: Spi0Sck = Spi0Sck {};
pub struct Spi0Sck {}

pub const LPTMR0_ALT2: Lptmr0Alt2 = Lptmr0Alt2 {};
pub struct Lptmr0Alt2 {}

pub const I2S0_RXD0: I2s0Rxd0 = I2s0Rxd0 {};
pub struct I2s0Rxd0 {}

pub const CMP0_IN0: Cmp0In0 = Cmp0In0 {};
pub struct Cmp0In0 {}

pub const PTC6: Ptc6 = Ptc6 {};
pub struct Ptc6 {}

pub const SPI0_MOSI: Spi0Mosi = Spi0Mosi {};
pub struct Spi0Mosi {}

pub const I2S0_RX_BCLK: I2s0RxBclk = I2s0RxBclk {};
pub struct I2s0RxBclk {}

pub const SPI0_MISO: Spi0Miso = Spi0Miso {};
pub struct Spi0Miso {}

pub const CMP0_IN1: Cmp0In1 = Cmp0In1 {};
pub struct Cmp0In1 {}

pub const PTC7: Ptc7 = Ptc7 {};
pub struct Ptc7 {}

pub const I2S0_RX_FS: I2s0RxFs = I2s0RxFs {};
pub struct I2s0RxFs {}

pub const CMP0_IN2: Cmp0In2 = Cmp0In2 {};
pub struct Cmp0In2 {}

pub const PTC8: Ptc8 = Ptc8 {};
pub struct Ptc8 {}

pub const CMP0_IN3: Cmp0In3 = Cmp0In3 {};
pub struct Cmp0In3 {}

pub const PTC9: Ptc9 = Ptc9 {};
pub struct Ptc9 {}

pub const PTC10: Ptc10 = Ptc10 {};
pub struct Ptc10 {}

pub const PTC11: Ptc11 = Ptc11 {};
pub struct Ptc11 {}

pub const PTD0: Ptd0 = Ptd0 {};
pub struct Ptd0 {}

pub const PTD1: Ptd1 = Ptd1 {};
pub struct Ptd1 {}

pub const PTD2: Ptd2 = Ptd2 {};
pub struct Ptd2 {}

pub const PTD3: Ptd3 = Ptd3 {};
pub struct Ptd3 {}

pub const PTD4: Ptd4 = Ptd4 {};
pub struct Ptd4 {}

pub const SPI1_PCS0: Spi1Pcs0 = Spi1Pcs0 {};
pub struct Spi1Pcs0 {}

pub const PTD5: Ptd5 = Ptd5 {};
pub struct Ptd5 {}

pub const SPI1_SCK: Spi1Sck = Spi1Sck {};
pub struct Spi1Sck {}

pub const PTD6: Ptd6 = Ptd6 {};
pub struct Ptd6 {}

pub const PTD7: Ptd7 = Ptd7 {};
pub struct Ptd7 {}

pub const PTE0: Pte0 = Pte0 {};
pub struct Pte0 {}

pub const RTC_CLKOUT: RtcClkout = RtcClkout {};
pub struct RtcClkout {}

pub const PTE1: Pte1 = Pte1 {};
pub struct Pte1 {}

pub const ADC0_SE0: Adc0Se0 = Adc0Se0 {};
pub struct Adc0Se0 {}

pub const PTE20: Pte20 = Pte20 {};
pub struct Pte20 {}

pub const PTE21: Pte21 = Pte21 {};
pub struct Pte21 {}

pub const ADC0_SE3: Adc0Se3 = Adc0Se3 {};
pub struct Adc0Se3 {}

pub const PTE22: Pte22 = Pte22 {};
pub struct Pte22 {}

pub const PTE23: Pte23 = Pte23 {};
pub struct Pte23 {}

pub const PTE24: Pte24 = Pte24 {};
pub struct Pte24 {}

pub const PTE25: Pte25 = Pte25 {};
pub struct Pte25 {}

pub const CMP0_IN5: Cmp0In5 = Cmp0In5 {};
pub struct Cmp0In5 {}

pub const PTE29: Pte29 = Pte29 {};
pub struct Pte29 {}

pub const DAC0_OUT: Dac0Out = Dac0Out {};
pub struct Dac0Out {}

pub const CMP0_IN4: Cmp0In4 = Cmp0In4 {};
pub struct Cmp0In4 {}

pub const PTE30: Pte30 = Pte30 {};
pub struct Pte30 {}

pub const PTE31: Pte31 = Pte31 {};
pub struct Pte31 {}

