pub use kinetis_common::chip::port::*;

pub const PORTA: Port = Port(0x40049000);
pub const PORTB: Port = Port(0x4004a000);
pub const PORTC: Port = Port(0x4004b000);
pub const PORTD: Port = Port(0x4004c000);
pub const PORTE: Port = Port(0x4004d000);

pub trait Pin {
   fn port(&self) -> Port;
   fn index(&self) -> usize;
}

pub trait AfTsi0Ch1 {
   fn af_tsi0_ch1(&self) -> usize;
}

pub trait AfPta0 {
   fn af_pta0(&self) -> usize;
}

pub trait AfTpm0Ch5 {
   fn af_tpm0_ch5(&self) -> usize;
}

pub trait AfSwdClk {
   fn af_swd_clk(&self) -> usize;
}

pub trait AfTsi0Ch2 {
   fn af_tsi0_ch2(&self) -> usize;
}

pub trait AfPta1 {
   fn af_pta1(&self) -> usize;
}

pub trait AfUart0Rx {
   fn af_uart0_rx(&self) -> usize;
}

pub trait AfTpm2Ch0 {
   fn af_tpm2_ch0(&self) -> usize;
}

pub trait AfTsi0Ch3 {
   fn af_tsi0_ch3(&self) -> usize;
}

pub trait AfPta2 {
   fn af_pta2(&self) -> usize;
}

pub trait AfUart0Tx {
   fn af_uart0_tx(&self) -> usize;
}

pub trait AfTpm2Ch1 {
   fn af_tpm2_ch1(&self) -> usize;
}

pub trait AfTsi0Ch4 {
   fn af_tsi0_ch4(&self) -> usize;
}

pub trait AfPta3 {
   fn af_pta3(&self) -> usize;
}

pub trait AfI2c1Scl {
   fn af_i2c1_scl(&self) -> usize;
}

pub trait AfTpm0Ch0 {
   fn af_tpm0_ch0(&self) -> usize;
}

pub trait AfSwdDio {
   fn af_swd_dio(&self) -> usize;
}

pub trait AfTsi0Ch5 {
   fn af_tsi0_ch5(&self) -> usize;
}

pub trait AfPta4 {
   fn af_pta4(&self) -> usize;
}

pub trait AfI2c1Sda {
   fn af_i2c1_sda(&self) -> usize;
}

pub trait AfTpm0Ch1 {
   fn af_tpm0_ch1(&self) -> usize;
}

pub trait AfNmiB {
   fn af_nmi_b(&self) -> usize;
}

pub trait AfPta5 {
   fn af_pta5(&self) -> usize;
}

pub trait AfUsbClkin {
   fn af_usb_clkin(&self) -> usize;
}

pub trait AfTpm0Ch2 {
   fn af_tpm0_ch2(&self) -> usize;
}

pub trait AfI2s0TxBclk {
   fn af_i2s0_tx_bclk(&self) -> usize;
}

pub trait AfPta12 {
   fn af_pta12(&self) -> usize;
}

pub trait AfTpm1Ch0 {
   fn af_tpm1_ch0(&self) -> usize;
}

pub trait AfI2s0Txd0 {
   fn af_i2s0_txd0(&self) -> usize;
}

pub trait AfPta13 {
   fn af_pta13(&self) -> usize;
}

pub trait AfTpm1Ch1 {
   fn af_tpm1_ch1(&self) -> usize;
}

pub trait AfI2s0TxFs {
   fn af_i2s0_tx_fs(&self) -> usize;
}

pub trait AfExtal0 {
   fn af_extal0(&self) -> usize;
}

pub trait AfPta18 {
   fn af_pta18(&self) -> usize;
}

pub trait AfUart1Rx {
   fn af_uart1_rx(&self) -> usize;
}

pub trait AfTpmClkin0 {
   fn af_tpm_clkin0(&self) -> usize;
}

pub trait AfXtal0 {
   fn af_xtal0(&self) -> usize;
}

pub trait AfPta19 {
   fn af_pta19(&self) -> usize;
}

pub trait AfUart1Tx {
   fn af_uart1_tx(&self) -> usize;
}

pub trait AfTpmClkin1 {
   fn af_tpm_clkin1(&self) -> usize;
}

pub trait AfLptmr0Alt1 {
   fn af_lptmr0_alt1(&self) -> usize;
}

pub trait AfPta20 {
   fn af_pta20(&self) -> usize;
}

pub trait AfResetB {
   fn af_reset_b(&self) -> usize;
}

pub trait AfAdc0Se8 {
   fn af_adc0_se8(&self) -> usize;
}

pub trait AfTsi0Ch0 {
   fn af_tsi0_ch0(&self) -> usize;
}

pub trait AfPtb0 {
   fn af_ptb0(&self) -> usize;
}

pub trait AfI2c0Scl {
   fn af_i2c0_scl(&self) -> usize;
}

pub trait AfAdc0Se9 {
   fn af_adc0_se9(&self) -> usize;
}

pub trait AfTsi0Ch6 {
   fn af_tsi0_ch6(&self) -> usize;
}

pub trait AfPtb1 {
   fn af_ptb1(&self) -> usize;
}

pub trait AfI2c0Sda {
   fn af_i2c0_sda(&self) -> usize;
}

pub trait AfAdc0Se12 {
   fn af_adc0_se12(&self) -> usize;
}

pub trait AfTsi0Ch7 {
   fn af_tsi0_ch7(&self) -> usize;
}

pub trait AfPtb2 {
   fn af_ptb2(&self) -> usize;
}

pub trait AfAdc0Se13 {
   fn af_adc0_se13(&self) -> usize;
}

pub trait AfTsi0Ch8 {
   fn af_tsi0_ch8(&self) -> usize;
}

pub trait AfPtb3 {
   fn af_ptb3(&self) -> usize;
}

pub trait AfTsi0Ch9 {
   fn af_tsi0_ch9(&self) -> usize;
}

pub trait AfPtb16 {
   fn af_ptb16(&self) -> usize;
}

pub trait AfSpi1Mosi {
   fn af_spi1_mosi(&self) -> usize;
}

pub trait AfSpi1Miso {
   fn af_spi1_miso(&self) -> usize;
}

pub trait AfTsi0Ch10 {
   fn af_tsi0_ch10(&self) -> usize;
}

pub trait AfPtb17 {
   fn af_ptb17(&self) -> usize;
}

pub trait AfTsi0Ch11 {
   fn af_tsi0_ch11(&self) -> usize;
}

pub trait AfPtb18 {
   fn af_ptb18(&self) -> usize;
}

pub trait AfTsi0Ch12 {
   fn af_tsi0_ch12(&self) -> usize;
}

pub trait AfPtb19 {
   fn af_ptb19(&self) -> usize;
}

pub trait AfAdc0Se14 {
   fn af_adc0_se14(&self) -> usize;
}

pub trait AfTsi0Ch13 {
   fn af_tsi0_ch13(&self) -> usize;
}

pub trait AfPtc0 {
   fn af_ptc0(&self) -> usize;
}

pub trait AfExtrgIn {
   fn af_extrg_in(&self) -> usize;
}

pub trait AfAudiousbSofOut {
   fn af_audiousb_sof_out(&self) -> usize;
}

pub trait AfCmp0Out {
   fn af_cmp0_out(&self) -> usize;
}

pub trait AfAdc0Se15 {
   fn af_adc0_se15(&self) -> usize;
}

pub trait AfTsi0Ch14 {
   fn af_tsi0_ch14(&self) -> usize;
}

pub trait AfPtc1 {
   fn af_ptc1(&self) -> usize;
}

pub trait AfAdc0Se11 {
   fn af_adc0_se11(&self) -> usize;
}

pub trait AfTsi0Ch15 {
   fn af_tsi0_ch15(&self) -> usize;
}

pub trait AfPtc2 {
   fn af_ptc2(&self) -> usize;
}

pub trait AfPtc3 {
   fn af_ptc3(&self) -> usize;
}

pub trait AfClkout {
   fn af_clkout(&self) -> usize;
}

pub trait AfPtc4 {
   fn af_ptc4(&self) -> usize;
}

pub trait AfSpi0Pcs0 {
   fn af_spi0_pcs0(&self) -> usize;
}

pub trait AfTpm0Ch3 {
   fn af_tpm0_ch3(&self) -> usize;
}

pub trait AfI2s0Mclk {
   fn af_i2s0_mclk(&self) -> usize;
}

pub trait AfPtc5 {
   fn af_ptc5(&self) -> usize;
}

pub trait AfSpi0Sck {
   fn af_spi0_sck(&self) -> usize;
}

pub trait AfLptmr0Alt2 {
   fn af_lptmr0_alt2(&self) -> usize;
}

pub trait AfI2s0Rxd0 {
   fn af_i2s0_rxd0(&self) -> usize;
}

pub trait AfCmp0In0 {
   fn af_cmp0_in0(&self) -> usize;
}

pub trait AfPtc6 {
   fn af_ptc6(&self) -> usize;
}

pub trait AfSpi0Mosi {
   fn af_spi0_mosi(&self) -> usize;
}

pub trait AfI2s0RxBclk {
   fn af_i2s0_rx_bclk(&self) -> usize;
}

pub trait AfSpi0Miso {
   fn af_spi0_miso(&self) -> usize;
}

pub trait AfCmp0In1 {
   fn af_cmp0_in1(&self) -> usize;
}

pub trait AfPtc7 {
   fn af_ptc7(&self) -> usize;
}

pub trait AfI2s0RxFs {
   fn af_i2s0_rx_fs(&self) -> usize;
}

pub trait AfCmp0In2 {
   fn af_cmp0_in2(&self) -> usize;
}

pub trait AfPtc8 {
   fn af_ptc8(&self) -> usize;
}

pub trait AfTpm0Ch4 {
   fn af_tpm0_ch4(&self) -> usize;
}

pub trait AfCmp0In3 {
   fn af_cmp0_in3(&self) -> usize;
}

pub trait AfPtc9 {
   fn af_ptc9(&self) -> usize;
}

pub trait AfPtc10 {
   fn af_ptc10(&self) -> usize;
}

pub trait AfPtc11 {
   fn af_ptc11(&self) -> usize;
}

pub trait AfPtd0 {
   fn af_ptd0(&self) -> usize;
}

pub trait AfAdc0Se5b {
   fn af_adc0_se5b(&self) -> usize;
}

pub trait AfPtd1 {
   fn af_ptd1(&self) -> usize;
}

pub trait AfPtd2 {
   fn af_ptd2(&self) -> usize;
}

pub trait AfUart2Rx {
   fn af_uart2_rx(&self) -> usize;
}

pub trait AfPtd3 {
   fn af_ptd3(&self) -> usize;
}

pub trait AfUart2Tx {
   fn af_uart2_tx(&self) -> usize;
}

pub trait AfPtd4 {
   fn af_ptd4(&self) -> usize;
}

pub trait AfSpi1Pcs0 {
   fn af_spi1_pcs0(&self) -> usize;
}

pub trait AfAdc0Se6b {
   fn af_adc0_se6b(&self) -> usize;
}

pub trait AfPtd5 {
   fn af_ptd5(&self) -> usize;
}

pub trait AfSpi1Sck {
   fn af_spi1_sck(&self) -> usize;
}

pub trait AfAdc0Se7b {
   fn af_adc0_se7b(&self) -> usize;
}

pub trait AfPtd6 {
   fn af_ptd6(&self) -> usize;
}

pub trait AfPtd7 {
   fn af_ptd7(&self) -> usize;
}

pub trait AfPte0 {
   fn af_pte0(&self) -> usize;
}

pub trait AfRtcClkout {
   fn af_rtc_clkout(&self) -> usize;
}

pub trait AfPte1 {
   fn af_pte1(&self) -> usize;
}

pub trait AfAdc0Dp0 {
   fn af_adc0_dp0(&self) -> usize;
}

pub trait AfAdc0Se0 {
   fn af_adc0_se0(&self) -> usize;
}

pub trait AfPte20 {
   fn af_pte20(&self) -> usize;
}

pub trait AfAdc0Dm0 {
   fn af_adc0_dm0(&self) -> usize;
}

pub trait AfAdc0Se4a {
   fn af_adc0_se4a(&self) -> usize;
}

pub trait AfPte21 {
   fn af_pte21(&self) -> usize;
}

pub trait AfAdc0Dp3 {
   fn af_adc0_dp3(&self) -> usize;
}

pub trait AfAdc0Se3 {
   fn af_adc0_se3(&self) -> usize;
}

pub trait AfPte22 {
   fn af_pte22(&self) -> usize;
}

pub trait AfAdc0Dm3 {
   fn af_adc0_dm3(&self) -> usize;
}

pub trait AfAdc0Se7a {
   fn af_adc0_se7a(&self) -> usize;
}

pub trait AfPte23 {
   fn af_pte23(&self) -> usize;
}

pub trait AfPte24 {
   fn af_pte24(&self) -> usize;
}

pub trait AfPte25 {
   fn af_pte25(&self) -> usize;
}

pub trait AfCmp0In5 {
   fn af_cmp0_in5(&self) -> usize;
}

pub trait AfAdc0Se4b {
   fn af_adc0_se4b(&self) -> usize;
}

pub trait AfPte29 {
   fn af_pte29(&self) -> usize;
}

pub trait AfDac0Out {
   fn af_dac0_out(&self) -> usize;
}

pub trait AfAdc0Se23 {
   fn af_adc0_se23(&self) -> usize;
}

pub trait AfCmp0In4 {
   fn af_cmp0_in4(&self) -> usize;
}

pub trait AfPte30 {
   fn af_pte30(&self) -> usize;
}

pub trait AfPte31 {
   fn af_pte31(&self) -> usize;
}

pub const PTA0: Pta0 = Pta0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta0 {}

impl Pin for Pta0 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 0 }
}

impl AfTsi0Ch1 for Pta0 {
   fn af_tsi0_ch1(&self) -> usize { 0 }
}

impl AfPta0 for Pta0 {
   fn af_pta0(&self) -> usize { 1 }
}

impl AfTpm0Ch5 for Pta0 {
   fn af_tpm0_ch5(&self) -> usize { 3 }
}

impl AfSwdClk for Pta0 {
   fn af_swd_clk(&self) -> usize { 7 }
}

pub const PTA1: Pta1 = Pta1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta1 {}

impl Pin for Pta1 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 1 }
}

impl AfTsi0Ch2 for Pta1 {
   fn af_tsi0_ch2(&self) -> usize { 0 }
}

impl AfPta1 for Pta1 {
   fn af_pta1(&self) -> usize { 1 }
}

impl AfUart0Rx for Pta1 {
   fn af_uart0_rx(&self) -> usize { 2 }
}

impl AfTpm2Ch0 for Pta1 {
   fn af_tpm2_ch0(&self) -> usize { 3 }
}

pub const PTA2: Pta2 = Pta2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta2 {}

impl Pin for Pta2 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 2 }
}

impl AfTsi0Ch3 for Pta2 {
   fn af_tsi0_ch3(&self) -> usize { 0 }
}

impl AfPta2 for Pta2 {
   fn af_pta2(&self) -> usize { 1 }
}

impl AfUart0Tx for Pta2 {
   fn af_uart0_tx(&self) -> usize { 2 }
}

impl AfTpm2Ch1 for Pta2 {
   fn af_tpm2_ch1(&self) -> usize { 3 }
}

pub const PTA3: Pta3 = Pta3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta3 {}

impl Pin for Pta3 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 3 }
}

impl AfTsi0Ch4 for Pta3 {
   fn af_tsi0_ch4(&self) -> usize { 0 }
}

impl AfPta3 for Pta3 {
   fn af_pta3(&self) -> usize { 1 }
}

impl AfI2c1Scl for Pta3 {
   fn af_i2c1_scl(&self) -> usize { 2 }
}

impl AfTpm0Ch0 for Pta3 {
   fn af_tpm0_ch0(&self) -> usize { 3 }
}

impl AfSwdDio for Pta3 {
   fn af_swd_dio(&self) -> usize { 7 }
}

pub const PTA4: Pta4 = Pta4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta4 {}

impl Pin for Pta4 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 4 }
}

impl AfTsi0Ch5 for Pta4 {
   fn af_tsi0_ch5(&self) -> usize { 0 }
}

impl AfPta4 for Pta4 {
   fn af_pta4(&self) -> usize { 1 }
}

impl AfI2c1Sda for Pta4 {
   fn af_i2c1_sda(&self) -> usize { 2 }
}

impl AfTpm0Ch1 for Pta4 {
   fn af_tpm0_ch1(&self) -> usize { 3 }
}

impl AfNmiB for Pta4 {
   fn af_nmi_b(&self) -> usize { 7 }
}

pub const PTA5: Pta5 = Pta5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta5 {}

impl Pin for Pta5 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 5 }
}

impl AfPta5 for Pta5 {
   fn af_pta5(&self) -> usize { 1 }
}

impl AfUsbClkin for Pta5 {
   fn af_usb_clkin(&self) -> usize { 2 }
}

impl AfTpm0Ch2 for Pta5 {
   fn af_tpm0_ch2(&self) -> usize { 3 }
}

impl AfI2s0TxBclk for Pta5 {
   fn af_i2s0_tx_bclk(&self) -> usize { 6 }
}

pub const PTA12: Pta12 = Pta12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta12 {}

impl Pin for Pta12 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 12 }
}

impl AfPta12 for Pta12 {
   fn af_pta12(&self) -> usize { 1 }
}

impl AfTpm1Ch0 for Pta12 {
   fn af_tpm1_ch0(&self) -> usize { 3 }
}

impl AfI2s0Txd0 for Pta12 {
   fn af_i2s0_txd0(&self) -> usize { 6 }
}

pub const PTA13: Pta13 = Pta13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta13 {}

impl Pin for Pta13 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 13 }
}

impl AfPta13 for Pta13 {
   fn af_pta13(&self) -> usize { 1 }
}

impl AfTpm1Ch1 for Pta13 {
   fn af_tpm1_ch1(&self) -> usize { 3 }
}

impl AfI2s0TxFs for Pta13 {
   fn af_i2s0_tx_fs(&self) -> usize { 6 }
}

pub const PTA18: Pta18 = Pta18 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta18 {}

impl Pin for Pta18 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 18 }
}

impl AfExtal0 for Pta18 {
   fn af_extal0(&self) -> usize { 0 }
}

impl AfPta18 for Pta18 {
   fn af_pta18(&self) -> usize { 1 }
}

impl AfUart1Rx for Pta18 {
   fn af_uart1_rx(&self) -> usize { 3 }
}

impl AfTpmClkin0 for Pta18 {
   fn af_tpm_clkin0(&self) -> usize { 4 }
}

pub const PTA19: Pta19 = Pta19 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta19 {}

impl Pin for Pta19 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 19 }
}

impl AfXtal0 for Pta19 {
   fn af_xtal0(&self) -> usize { 0 }
}

impl AfPta19 for Pta19 {
   fn af_pta19(&self) -> usize { 1 }
}

impl AfUart1Tx for Pta19 {
   fn af_uart1_tx(&self) -> usize { 3 }
}

impl AfTpmClkin1 for Pta19 {
   fn af_tpm_clkin1(&self) -> usize { 4 }
}

impl AfLptmr0Alt1 for Pta19 {
   fn af_lptmr0_alt1(&self) -> usize { 6 }
}

pub const PTA20: Pta20 = Pta20 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta20 {}

impl Pin for Pta20 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 20 }
}

impl AfPta20 for Pta20 {
   fn af_pta20(&self) -> usize { 1 }
}

impl AfResetB for Pta20 {
   fn af_reset_b(&self) -> usize { 7 }
}

pub const PTB0: Ptb0 = Ptb0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb0 {}

impl Pin for Ptb0 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 0 }
}

impl AfAdc0Se8 for Ptb0 {
   fn af_adc0_se8(&self) -> usize { 0 }
}

impl AfTsi0Ch0 for Ptb0 {
   fn af_tsi0_ch0(&self) -> usize { 0 }
}

impl AfPtb0 for Ptb0 {
   fn af_ptb0(&self) -> usize { 1 }
}

impl AfI2c0Scl for Ptb0 {
   fn af_i2c0_scl(&self) -> usize { 2 }
}

impl AfTpm1Ch0 for Ptb0 {
   fn af_tpm1_ch0(&self) -> usize { 3 }
}

pub const PTB1: Ptb1 = Ptb1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb1 {}

impl Pin for Ptb1 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 1 }
}

impl AfAdc0Se9 for Ptb1 {
   fn af_adc0_se9(&self) -> usize { 0 }
}

impl AfTsi0Ch6 for Ptb1 {
   fn af_tsi0_ch6(&self) -> usize { 0 }
}

impl AfPtb1 for Ptb1 {
   fn af_ptb1(&self) -> usize { 1 }
}

impl AfI2c0Sda for Ptb1 {
   fn af_i2c0_sda(&self) -> usize { 2 }
}

impl AfTpm1Ch1 for Ptb1 {
   fn af_tpm1_ch1(&self) -> usize { 3 }
}

pub const PTB2: Ptb2 = Ptb2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb2 {}

impl Pin for Ptb2 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 2 }
}

impl AfAdc0Se12 for Ptb2 {
   fn af_adc0_se12(&self) -> usize { 0 }
}

impl AfTsi0Ch7 for Ptb2 {
   fn af_tsi0_ch7(&self) -> usize { 0 }
}

impl AfPtb2 for Ptb2 {
   fn af_ptb2(&self) -> usize { 1 }
}

impl AfI2c0Scl for Ptb2 {
   fn af_i2c0_scl(&self) -> usize { 2 }
}

impl AfTpm2Ch0 for Ptb2 {
   fn af_tpm2_ch0(&self) -> usize { 3 }
}

pub const PTB3: Ptb3 = Ptb3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb3 {}

impl Pin for Ptb3 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 3 }
}

impl AfAdc0Se13 for Ptb3 {
   fn af_adc0_se13(&self) -> usize { 0 }
}

impl AfTsi0Ch8 for Ptb3 {
   fn af_tsi0_ch8(&self) -> usize { 0 }
}

impl AfPtb3 for Ptb3 {
   fn af_ptb3(&self) -> usize { 1 }
}

impl AfI2c0Sda for Ptb3 {
   fn af_i2c0_sda(&self) -> usize { 2 }
}

impl AfTpm2Ch1 for Ptb3 {
   fn af_tpm2_ch1(&self) -> usize { 3 }
}

pub const PTB16: Ptb16 = Ptb16 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb16 {}

impl Pin for Ptb16 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 16 }
}

impl AfTsi0Ch9 for Ptb16 {
   fn af_tsi0_ch9(&self) -> usize { 0 }
}

impl AfPtb16 for Ptb16 {
   fn af_ptb16(&self) -> usize { 1 }
}

impl AfSpi1Mosi for Ptb16 {
   fn af_spi1_mosi(&self) -> usize { 2 }
}

impl AfUart0Rx for Ptb16 {
   fn af_uart0_rx(&self) -> usize { 3 }
}

impl AfTpmClkin0 for Ptb16 {
   fn af_tpm_clkin0(&self) -> usize { 4 }
}

impl AfSpi1Miso for Ptb16 {
   fn af_spi1_miso(&self) -> usize { 5 }
}

pub const PTB17: Ptb17 = Ptb17 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb17 {}

impl Pin for Ptb17 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 17 }
}

impl AfTsi0Ch10 for Ptb17 {
   fn af_tsi0_ch10(&self) -> usize { 0 }
}

impl AfPtb17 for Ptb17 {
   fn af_ptb17(&self) -> usize { 1 }
}

impl AfSpi1Miso for Ptb17 {
   fn af_spi1_miso(&self) -> usize { 2 }
}

impl AfUart0Tx for Ptb17 {
   fn af_uart0_tx(&self) -> usize { 3 }
}

impl AfTpmClkin1 for Ptb17 {
   fn af_tpm_clkin1(&self) -> usize { 4 }
}

impl AfSpi1Mosi for Ptb17 {
   fn af_spi1_mosi(&self) -> usize { 5 }
}

pub const PTB18: Ptb18 = Ptb18 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb18 {}

impl Pin for Ptb18 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 18 }
}

impl AfTsi0Ch11 for Ptb18 {
   fn af_tsi0_ch11(&self) -> usize { 0 }
}

impl AfPtb18 for Ptb18 {
   fn af_ptb18(&self) -> usize { 1 }
}

impl AfTpm2Ch0 for Ptb18 {
   fn af_tpm2_ch0(&self) -> usize { 3 }
}

impl AfI2s0TxBclk for Ptb18 {
   fn af_i2s0_tx_bclk(&self) -> usize { 4 }
}

pub const PTB19: Ptb19 = Ptb19 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb19 {}

impl Pin for Ptb19 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 19 }
}

impl AfTsi0Ch12 for Ptb19 {
   fn af_tsi0_ch12(&self) -> usize { 0 }
}

impl AfPtb19 for Ptb19 {
   fn af_ptb19(&self) -> usize { 1 }
}

impl AfTpm2Ch1 for Ptb19 {
   fn af_tpm2_ch1(&self) -> usize { 3 }
}

impl AfI2s0TxFs for Ptb19 {
   fn af_i2s0_tx_fs(&self) -> usize { 4 }
}

pub const PTC0: Ptc0 = Ptc0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc0 {}

impl Pin for Ptc0 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 0 }
}

impl AfAdc0Se14 for Ptc0 {
   fn af_adc0_se14(&self) -> usize { 0 }
}

impl AfTsi0Ch13 for Ptc0 {
   fn af_tsi0_ch13(&self) -> usize { 0 }
}

impl AfPtc0 for Ptc0 {
   fn af_ptc0(&self) -> usize { 1 }
}

impl AfExtrgIn for Ptc0 {
   fn af_extrg_in(&self) -> usize { 3 }
}

impl AfAudiousbSofOut for Ptc0 {
   fn af_audiousb_sof_out(&self) -> usize { 4 }
}

impl AfCmp0Out for Ptc0 {
   fn af_cmp0_out(&self) -> usize { 5 }
}

impl AfI2s0Txd0 for Ptc0 {
   fn af_i2s0_txd0(&self) -> usize { 6 }
}

pub const PTC1: Ptc1 = Ptc1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc1 {}

impl Pin for Ptc1 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 1 }
}

impl AfAdc0Se15 for Ptc1 {
   fn af_adc0_se15(&self) -> usize { 0 }
}

impl AfTsi0Ch14 for Ptc1 {
   fn af_tsi0_ch14(&self) -> usize { 0 }
}

impl AfPtc1 for Ptc1 {
   fn af_ptc1(&self) -> usize { 1 }
}

impl AfI2c1Scl for Ptc1 {
   fn af_i2c1_scl(&self) -> usize { 2 }
}

impl AfTpm0Ch0 for Ptc1 {
   fn af_tpm0_ch0(&self) -> usize { 4 }
}

impl AfI2s0Txd0 for Ptc1 {
   fn af_i2s0_txd0(&self) -> usize { 6 }
}

pub const PTC2: Ptc2 = Ptc2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc2 {}

impl Pin for Ptc2 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 2 }
}

impl AfAdc0Se11 for Ptc2 {
   fn af_adc0_se11(&self) -> usize { 0 }
}

impl AfTsi0Ch15 for Ptc2 {
   fn af_tsi0_ch15(&self) -> usize { 0 }
}

impl AfPtc2 for Ptc2 {
   fn af_ptc2(&self) -> usize { 1 }
}

impl AfI2c1Sda for Ptc2 {
   fn af_i2c1_sda(&self) -> usize { 2 }
}

impl AfTpm0Ch1 for Ptc2 {
   fn af_tpm0_ch1(&self) -> usize { 4 }
}

impl AfI2s0TxFs for Ptc2 {
   fn af_i2s0_tx_fs(&self) -> usize { 6 }
}

pub const PTC3: Ptc3 = Ptc3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc3 {}

impl Pin for Ptc3 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 3 }
}

impl AfPtc3 for Ptc3 {
   fn af_ptc3(&self) -> usize { 1 }
}

impl AfUart1Rx for Ptc3 {
   fn af_uart1_rx(&self) -> usize { 3 }
}

impl AfTpm0Ch2 for Ptc3 {
   fn af_tpm0_ch2(&self) -> usize { 4 }
}

impl AfClkout for Ptc3 {
   fn af_clkout(&self) -> usize { 5 }
}

impl AfI2s0TxBclk for Ptc3 {
   fn af_i2s0_tx_bclk(&self) -> usize { 6 }
}

pub const PTC4: Ptc4 = Ptc4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc4 {}

impl Pin for Ptc4 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 4 }
}

impl AfPtc4 for Ptc4 {
   fn af_ptc4(&self) -> usize { 1 }
}

impl AfSpi0Pcs0 for Ptc4 {
   fn af_spi0_pcs0(&self) -> usize { 2 }
}

impl AfUart1Tx for Ptc4 {
   fn af_uart1_tx(&self) -> usize { 3 }
}

impl AfTpm0Ch3 for Ptc4 {
   fn af_tpm0_ch3(&self) -> usize { 4 }
}

impl AfI2s0Mclk for Ptc4 {
   fn af_i2s0_mclk(&self) -> usize { 5 }
}

pub const PTC5: Ptc5 = Ptc5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc5 {}

impl Pin for Ptc5 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 5 }
}

impl AfPtc5 for Ptc5 {
   fn af_ptc5(&self) -> usize { 1 }
}

impl AfSpi0Sck for Ptc5 {
   fn af_spi0_sck(&self) -> usize { 2 }
}

impl AfLptmr0Alt2 for Ptc5 {
   fn af_lptmr0_alt2(&self) -> usize { 3 }
}

impl AfI2s0Rxd0 for Ptc5 {
   fn af_i2s0_rxd0(&self) -> usize { 4 }
}

impl AfCmp0Out for Ptc5 {
   fn af_cmp0_out(&self) -> usize { 6 }
}

pub const PTC6: Ptc6 = Ptc6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc6 {}

impl Pin for Ptc6 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 6 }
}

impl AfCmp0In0 for Ptc6 {
   fn af_cmp0_in0(&self) -> usize { 0 }
}

impl AfPtc6 for Ptc6 {
   fn af_ptc6(&self) -> usize { 1 }
}

impl AfSpi0Mosi for Ptc6 {
   fn af_spi0_mosi(&self) -> usize { 2 }
}

impl AfExtrgIn for Ptc6 {
   fn af_extrg_in(&self) -> usize { 3 }
}

impl AfI2s0RxBclk for Ptc6 {
   fn af_i2s0_rx_bclk(&self) -> usize { 4 }
}

impl AfSpi0Miso for Ptc6 {
   fn af_spi0_miso(&self) -> usize { 5 }
}

impl AfI2s0Mclk for Ptc6 {
   fn af_i2s0_mclk(&self) -> usize { 6 }
}

pub const PTC7: Ptc7 = Ptc7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc7 {}

impl Pin for Ptc7 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 7 }
}

impl AfCmp0In1 for Ptc7 {
   fn af_cmp0_in1(&self) -> usize { 0 }
}

impl AfPtc7 for Ptc7 {
   fn af_ptc7(&self) -> usize { 1 }
}

impl AfSpi0Miso for Ptc7 {
   fn af_spi0_miso(&self) -> usize { 2 }
}

impl AfAudiousbSofOut for Ptc7 {
   fn af_audiousb_sof_out(&self) -> usize { 3 }
}

impl AfI2s0RxFs for Ptc7 {
   fn af_i2s0_rx_fs(&self) -> usize { 4 }
}

impl AfSpi0Mosi for Ptc7 {
   fn af_spi0_mosi(&self) -> usize { 5 }
}

pub const PTC8: Ptc8 = Ptc8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc8 {}

impl Pin for Ptc8 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 8 }
}

impl AfCmp0In2 for Ptc8 {
   fn af_cmp0_in2(&self) -> usize { 0 }
}

impl AfPtc8 for Ptc8 {
   fn af_ptc8(&self) -> usize { 1 }
}

impl AfI2c0Scl for Ptc8 {
   fn af_i2c0_scl(&self) -> usize { 2 }
}

impl AfTpm0Ch4 for Ptc8 {
   fn af_tpm0_ch4(&self) -> usize { 3 }
}

impl AfI2s0Mclk for Ptc8 {
   fn af_i2s0_mclk(&self) -> usize { 4 }
}

pub const PTC9: Ptc9 = Ptc9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc9 {}

impl Pin for Ptc9 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 9 }
}

impl AfCmp0In3 for Ptc9 {
   fn af_cmp0_in3(&self) -> usize { 0 }
}

impl AfPtc9 for Ptc9 {
   fn af_ptc9(&self) -> usize { 1 }
}

impl AfI2c0Sda for Ptc9 {
   fn af_i2c0_sda(&self) -> usize { 2 }
}

impl AfTpm0Ch5 for Ptc9 {
   fn af_tpm0_ch5(&self) -> usize { 3 }
}

impl AfI2s0RxBclk for Ptc9 {
   fn af_i2s0_rx_bclk(&self) -> usize { 4 }
}

pub const PTC10: Ptc10 = Ptc10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc10 {}

impl Pin for Ptc10 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 10 }
}

impl AfPtc10 for Ptc10 {
   fn af_ptc10(&self) -> usize { 1 }
}

impl AfI2c1Scl for Ptc10 {
   fn af_i2c1_scl(&self) -> usize { 2 }
}

impl AfI2s0RxFs for Ptc10 {
   fn af_i2s0_rx_fs(&self) -> usize { 4 }
}

pub const PTC11: Ptc11 = Ptc11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc11 {}

impl Pin for Ptc11 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 11 }
}

impl AfPtc11 for Ptc11 {
   fn af_ptc11(&self) -> usize { 1 }
}

impl AfI2c1Sda for Ptc11 {
   fn af_i2c1_sda(&self) -> usize { 2 }
}

impl AfI2s0Rxd0 for Ptc11 {
   fn af_i2s0_rxd0(&self) -> usize { 4 }
}

pub const PTD0: Ptd0 = Ptd0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd0 {}

impl Pin for Ptd0 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 0 }
}

impl AfPtd0 for Ptd0 {
   fn af_ptd0(&self) -> usize { 1 }
}

impl AfSpi0Pcs0 for Ptd0 {
   fn af_spi0_pcs0(&self) -> usize { 2 }
}

impl AfTpm0Ch0 for Ptd0 {
   fn af_tpm0_ch0(&self) -> usize { 4 }
}

pub const PTD1: Ptd1 = Ptd1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd1 {}

impl Pin for Ptd1 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 1 }
}

impl AfAdc0Se5b for Ptd1 {
   fn af_adc0_se5b(&self) -> usize { 0 }
}

impl AfPtd1 for Ptd1 {
   fn af_ptd1(&self) -> usize { 1 }
}

impl AfSpi0Sck for Ptd1 {
   fn af_spi0_sck(&self) -> usize { 2 }
}

impl AfTpm0Ch1 for Ptd1 {
   fn af_tpm0_ch1(&self) -> usize { 4 }
}

pub const PTD2: Ptd2 = Ptd2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd2 {}

impl Pin for Ptd2 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 2 }
}

impl AfPtd2 for Ptd2 {
   fn af_ptd2(&self) -> usize { 1 }
}

impl AfSpi0Mosi for Ptd2 {
   fn af_spi0_mosi(&self) -> usize { 2 }
}

impl AfUart2Rx for Ptd2 {
   fn af_uart2_rx(&self) -> usize { 3 }
}

impl AfTpm0Ch2 for Ptd2 {
   fn af_tpm0_ch2(&self) -> usize { 4 }
}

impl AfSpi0Miso for Ptd2 {
   fn af_spi0_miso(&self) -> usize { 5 }
}

pub const PTD3: Ptd3 = Ptd3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd3 {}

impl Pin for Ptd3 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 3 }
}

impl AfPtd3 for Ptd3 {
   fn af_ptd3(&self) -> usize { 1 }
}

impl AfSpi0Miso for Ptd3 {
   fn af_spi0_miso(&self) -> usize { 2 }
}

impl AfUart2Tx for Ptd3 {
   fn af_uart2_tx(&self) -> usize { 3 }
}

impl AfTpm0Ch3 for Ptd3 {
   fn af_tpm0_ch3(&self) -> usize { 4 }
}

impl AfSpi0Mosi for Ptd3 {
   fn af_spi0_mosi(&self) -> usize { 5 }
}

pub const PTD4: Ptd4 = Ptd4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd4 {}

impl Pin for Ptd4 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 4 }
}

impl AfPtd4 for Ptd4 {
   fn af_ptd4(&self) -> usize { 1 }
}

impl AfSpi1Pcs0 for Ptd4 {
   fn af_spi1_pcs0(&self) -> usize { 2 }
}

impl AfUart2Rx for Ptd4 {
   fn af_uart2_rx(&self) -> usize { 3 }
}

impl AfTpm0Ch4 for Ptd4 {
   fn af_tpm0_ch4(&self) -> usize { 4 }
}

pub const PTD5: Ptd5 = Ptd5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd5 {}

impl Pin for Ptd5 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 5 }
}

impl AfAdc0Se6b for Ptd5 {
   fn af_adc0_se6b(&self) -> usize { 0 }
}

impl AfPtd5 for Ptd5 {
   fn af_ptd5(&self) -> usize { 1 }
}

impl AfSpi1Sck for Ptd5 {
   fn af_spi1_sck(&self) -> usize { 2 }
}

impl AfUart2Tx for Ptd5 {
   fn af_uart2_tx(&self) -> usize { 3 }
}

impl AfTpm0Ch5 for Ptd5 {
   fn af_tpm0_ch5(&self) -> usize { 4 }
}

pub const PTD6: Ptd6 = Ptd6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd6 {}

impl Pin for Ptd6 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 6 }
}

impl AfAdc0Se7b for Ptd6 {
   fn af_adc0_se7b(&self) -> usize { 0 }
}

impl AfPtd6 for Ptd6 {
   fn af_ptd6(&self) -> usize { 1 }
}

impl AfSpi1Mosi for Ptd6 {
   fn af_spi1_mosi(&self) -> usize { 2 }
}

impl AfUart0Rx for Ptd6 {
   fn af_uart0_rx(&self) -> usize { 3 }
}

impl AfSpi1Miso for Ptd6 {
   fn af_spi1_miso(&self) -> usize { 5 }
}

pub const PTD7: Ptd7 = Ptd7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd7 {}

impl Pin for Ptd7 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 7 }
}

impl AfPtd7 for Ptd7 {
   fn af_ptd7(&self) -> usize { 1 }
}

impl AfSpi1Miso for Ptd7 {
   fn af_spi1_miso(&self) -> usize { 2 }
}

impl AfUart0Tx for Ptd7 {
   fn af_uart0_tx(&self) -> usize { 3 }
}

impl AfSpi1Mosi for Ptd7 {
   fn af_spi1_mosi(&self) -> usize { 5 }
}

pub const PTE0: Pte0 = Pte0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte0 {}

impl Pin for Pte0 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 0 }
}

impl AfPte0 for Pte0 {
   fn af_pte0(&self) -> usize { 1 }
}

impl AfSpi1Miso for Pte0 {
   fn af_spi1_miso(&self) -> usize { 2 }
}

impl AfUart1Tx for Pte0 {
   fn af_uart1_tx(&self) -> usize { 3 }
}

impl AfRtcClkout for Pte0 {
   fn af_rtc_clkout(&self) -> usize { 4 }
}

impl AfCmp0Out for Pte0 {
   fn af_cmp0_out(&self) -> usize { 5 }
}

impl AfI2c1Sda for Pte0 {
   fn af_i2c1_sda(&self) -> usize { 6 }
}

pub const PTE1: Pte1 = Pte1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte1 {}

impl Pin for Pte1 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 1 }
}

impl AfPte1 for Pte1 {
   fn af_pte1(&self) -> usize { 1 }
}

impl AfSpi1Mosi for Pte1 {
   fn af_spi1_mosi(&self) -> usize { 2 }
}

impl AfUart1Rx for Pte1 {
   fn af_uart1_rx(&self) -> usize { 3 }
}

impl AfSpi1Miso for Pte1 {
   fn af_spi1_miso(&self) -> usize { 5 }
}

impl AfI2c1Scl for Pte1 {
   fn af_i2c1_scl(&self) -> usize { 6 }
}

pub const PTE20: Pte20 = Pte20 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte20 {}

impl Pin for Pte20 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 20 }
}

impl AfAdc0Dp0 for Pte20 {
   fn af_adc0_dp0(&self) -> usize { 0 }
}

impl AfAdc0Se0 for Pte20 {
   fn af_adc0_se0(&self) -> usize { 0 }
}

impl AfPte20 for Pte20 {
   fn af_pte20(&self) -> usize { 1 }
}

impl AfTpm1Ch0 for Pte20 {
   fn af_tpm1_ch0(&self) -> usize { 3 }
}

impl AfUart0Tx for Pte20 {
   fn af_uart0_tx(&self) -> usize { 4 }
}

pub const PTE21: Pte21 = Pte21 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte21 {}

impl Pin for Pte21 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 21 }
}

impl AfAdc0Dm0 for Pte21 {
   fn af_adc0_dm0(&self) -> usize { 0 }
}

impl AfAdc0Se4a for Pte21 {
   fn af_adc0_se4a(&self) -> usize { 0 }
}

impl AfPte21 for Pte21 {
   fn af_pte21(&self) -> usize { 1 }
}

impl AfTpm1Ch1 for Pte21 {
   fn af_tpm1_ch1(&self) -> usize { 3 }
}

impl AfUart0Rx for Pte21 {
   fn af_uart0_rx(&self) -> usize { 4 }
}

pub const PTE22: Pte22 = Pte22 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte22 {}

impl Pin for Pte22 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 22 }
}

impl AfAdc0Dp3 for Pte22 {
   fn af_adc0_dp3(&self) -> usize { 0 }
}

impl AfAdc0Se3 for Pte22 {
   fn af_adc0_se3(&self) -> usize { 0 }
}

impl AfPte22 for Pte22 {
   fn af_pte22(&self) -> usize { 1 }
}

impl AfTpm2Ch0 for Pte22 {
   fn af_tpm2_ch0(&self) -> usize { 3 }
}

impl AfUart2Tx for Pte22 {
   fn af_uart2_tx(&self) -> usize { 4 }
}

pub const PTE23: Pte23 = Pte23 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte23 {}

impl Pin for Pte23 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 23 }
}

impl AfAdc0Dm3 for Pte23 {
   fn af_adc0_dm3(&self) -> usize { 0 }
}

impl AfAdc0Se7a for Pte23 {
   fn af_adc0_se7a(&self) -> usize { 0 }
}

impl AfPte23 for Pte23 {
   fn af_pte23(&self) -> usize { 1 }
}

impl AfTpm2Ch1 for Pte23 {
   fn af_tpm2_ch1(&self) -> usize { 3 }
}

impl AfUart2Rx for Pte23 {
   fn af_uart2_rx(&self) -> usize { 4 }
}

pub const PTE24: Pte24 = Pte24 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte24 {}

impl Pin for Pte24 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 24 }
}

impl AfPte24 for Pte24 {
   fn af_pte24(&self) -> usize { 1 }
}

impl AfTpm0Ch0 for Pte24 {
   fn af_tpm0_ch0(&self) -> usize { 3 }
}

impl AfI2c0Scl for Pte24 {
   fn af_i2c0_scl(&self) -> usize { 5 }
}

pub const PTE25: Pte25 = Pte25 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte25 {}

impl Pin for Pte25 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 25 }
}

impl AfPte25 for Pte25 {
   fn af_pte25(&self) -> usize { 1 }
}

impl AfTpm0Ch1 for Pte25 {
   fn af_tpm0_ch1(&self) -> usize { 3 }
}

impl AfI2c0Sda for Pte25 {
   fn af_i2c0_sda(&self) -> usize { 5 }
}

pub const PTE29: Pte29 = Pte29 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte29 {}

impl Pin for Pte29 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 29 }
}

impl AfCmp0In5 for Pte29 {
   fn af_cmp0_in5(&self) -> usize { 0 }
}

impl AfAdc0Se4b for Pte29 {
   fn af_adc0_se4b(&self) -> usize { 0 }
}

impl AfPte29 for Pte29 {
   fn af_pte29(&self) -> usize { 1 }
}

impl AfTpm0Ch2 for Pte29 {
   fn af_tpm0_ch2(&self) -> usize { 3 }
}

impl AfTpmClkin0 for Pte29 {
   fn af_tpm_clkin0(&self) -> usize { 4 }
}

pub const PTE30: Pte30 = Pte30 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte30 {}

impl Pin for Pte30 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 30 }
}

impl AfDac0Out for Pte30 {
   fn af_dac0_out(&self) -> usize { 0 }
}

impl AfAdc0Se23 for Pte30 {
   fn af_adc0_se23(&self) -> usize { 0 }
}

impl AfCmp0In4 for Pte30 {
   fn af_cmp0_in4(&self) -> usize { 0 }
}

impl AfPte30 for Pte30 {
   fn af_pte30(&self) -> usize { 1 }
}

impl AfTpm0Ch3 for Pte30 {
   fn af_tpm0_ch3(&self) -> usize { 3 }
}

impl AfTpmClkin1 for Pte30 {
   fn af_tpm_clkin1(&self) -> usize { 4 }
}

pub const PTE31: Pte31 = Pte31 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte31 {}

impl Pin for Pte31 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 31 }
}

impl AfPte31 for Pte31 {
   fn af_pte31(&self) -> usize { 1 }
}

impl AfTpm0Ch4 for Pte31 {
   fn af_tpm0_ch4(&self) -> usize { 3 }
}

