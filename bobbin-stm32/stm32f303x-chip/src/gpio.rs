pub use stm32_common::chip::gpio::*;

pub const GPIOA: Gpio = Gpio(0x48000000);
pub const GPIOB: Gpio = Gpio(0x48000400);
pub const GPIOC: Gpio = Gpio(0x48000800);
pub const GPIOD: Gpio = Gpio(0x48000c00);
pub const GPIOE: Gpio = Gpio(0x48001000);
pub const GPIOF: Gpio = Gpio(0x48001400);
pub const GPIOG: Gpio = Gpio(0x48001800);
pub const GPIOH: Gpio = Gpio(0x48001c00);

pub trait Pin {
   fn port(&self) -> Gpio;
   fn index(&self) -> usize;
}

pub trait AfTim2Ch1 {
   fn af_tim2_ch1(&self) -> usize;
}

pub trait AfTim2Etr {
   fn af_tim2_etr(&self) -> usize;
}

pub trait AfTscG1Io1 {
   fn af_tsc_g1_io1(&self) -> usize;
}

pub trait AfUsart2Cts {
   fn af_usart2_cts(&self) -> usize;
}

pub trait AfComp1Out {
   fn af_comp1_out(&self) -> usize;
}

pub trait AfTim8Bkin {
   fn af_tim8_bkin(&self) -> usize;
}

pub trait AfTim8Etr {
   fn af_tim8_etr(&self) -> usize;
}

pub trait AfEventOut {
   fn af_event_out(&self) -> usize;
}

pub trait AfRtcRefin {
   fn af_rtc_refin(&self) -> usize;
}

pub trait AfTim2Ch2 {
   fn af_tim2_ch2(&self) -> usize;
}

pub trait AfTscG1Io2 {
   fn af_tsc_g1_io2(&self) -> usize;
}

pub trait AfUsart2Rts {
   fn af_usart2_rts(&self) -> usize;
}

pub trait AfTim15Ch1n {
   fn af_tim15_ch1n(&self) -> usize;
}

pub trait AfTim2Ch3 {
   fn af_tim2_ch3(&self) -> usize;
}

pub trait AfTscG1Io3 {
   fn af_tsc_g1_io3(&self) -> usize;
}

pub trait AfUsart2Tx {
   fn af_usart2_tx(&self) -> usize;
}

pub trait AfComp2Out {
   fn af_comp2_out(&self) -> usize;
}

pub trait AfTim15Ch1 {
   fn af_tim15_ch1(&self) -> usize;
}

pub trait AfTscG1Io4 {
   fn af_tsc_g1_io4(&self) -> usize;
}

pub trait AfUsart2Rx {
   fn af_usart2_rx(&self) -> usize;
}

pub trait AfTim15Ch2 {
   fn af_tim15_ch2(&self) -> usize;
}

pub trait AfTim3Ch2 {
   fn af_tim3_ch2(&self) -> usize;
}

pub trait AfTscG2Io1 {
   fn af_tsc_g2_io1(&self) -> usize;
}

pub trait AfSpi1Nss {
   fn af_spi1_nss(&self) -> usize;
}

pub trait AfSpi3Nss {
   fn af_spi3_nss(&self) -> usize;
}

pub trait AfI2s3Ws {
   fn af_i2s3_ws(&self) -> usize;
}

pub trait AfUsart2Ck {
   fn af_usart2_ck(&self) -> usize;
}

pub trait AfTscG2Io2 {
   fn af_tsc_g2_io2(&self) -> usize;
}

pub trait AfSpi1Sck {
   fn af_spi1_sck(&self) -> usize;
}

pub trait AfTim16Ch1 {
   fn af_tim16_ch1(&self) -> usize;
}

pub trait AfTim3Ch1 {
   fn af_tim3_ch1(&self) -> usize;
}

pub trait AfTscG2Io3 {
   fn af_tsc_g2_io3(&self) -> usize;
}

pub trait AfSpi1Miso {
   fn af_spi1_miso(&self) -> usize;
}

pub trait AfTim1Bkin {
   fn af_tim1_bkin(&self) -> usize;
}

pub trait AfTim17Ch1 {
   fn af_tim17_ch1(&self) -> usize;
}

pub trait AfTscG2Io4 {
   fn af_tsc_g2_io4(&self) -> usize;
}

pub trait AfTim8Ch1n {
   fn af_tim8_ch1n(&self) -> usize;
}

pub trait AfSpi1Mosi {
   fn af_spi1_mosi(&self) -> usize;
}

pub trait AfTim1Ch1n {
   fn af_tim1_ch1n(&self) -> usize;
}

pub trait AfMco {
   fn af_mco(&self) -> usize;
}

pub trait AfI2c3Scl {
   fn af_i2c3_scl(&self) -> usize;
}

pub trait AfI2c2Smbal {
   fn af_i2c2_smbal(&self) -> usize;
}

pub trait AfI2s2Mck {
   fn af_i2s2_mck(&self) -> usize;
}

pub trait AfTim1Ch1 {
   fn af_tim1_ch1(&self) -> usize;
}

pub trait AfUsart1Ck {
   fn af_usart1_ck(&self) -> usize;
}

pub trait AfComp3Out {
   fn af_comp3_out(&self) -> usize;
}

pub trait AfTim4Etr {
   fn af_tim4_etr(&self) -> usize;
}

pub trait AfI2c3Smbal {
   fn af_i2c3_smbal(&self) -> usize;
}

pub trait AfTscG4Io1 {
   fn af_tsc_g4_io1(&self) -> usize;
}

pub trait AfI2c2Scl {
   fn af_i2c2_scl(&self) -> usize;
}

pub trait AfI2s3Mck {
   fn af_i2s3_mck(&self) -> usize;
}

pub trait AfTim1Ch2 {
   fn af_tim1_ch2(&self) -> usize;
}

pub trait AfUsart1Tx {
   fn af_usart1_tx(&self) -> usize;
}

pub trait AfComp5Out {
   fn af_comp5_out(&self) -> usize;
}

pub trait AfTim15Bkin {
   fn af_tim15_bkin(&self) -> usize;
}

pub trait AfTim17Bkin {
   fn af_tim17_bkin(&self) -> usize;
}

pub trait AfTscG4Io2 {
   fn af_tsc_g4_io2(&self) -> usize;
}

pub trait AfI2c2Sda {
   fn af_i2c2_sda(&self) -> usize;
}

pub trait AfSpi2Miso {
   fn af_spi2_miso(&self) -> usize;
}

pub trait AfI2s2extSd {
   fn af_i2s2ext_sd(&self) -> usize;
}

pub trait AfTim1Ch3 {
   fn af_tim1_ch3(&self) -> usize;
}

pub trait AfUsart1Rx {
   fn af_usart1_rx(&self) -> usize;
}

pub trait AfComp6Out {
   fn af_comp6_out(&self) -> usize;
}

pub trait AfTim2Ch4 {
   fn af_tim2_ch4(&self) -> usize;
}

pub trait AfSpi2Mosi {
   fn af_spi2_mosi(&self) -> usize;
}

pub trait AfI2s2Sd {
   fn af_i2s2_sd(&self) -> usize;
}

pub trait AfUsart1Cts {
   fn af_usart1_cts(&self) -> usize;
}

pub trait AfCanRx {
   fn af_can_rx(&self) -> usize;
}

pub trait AfTim4Ch1 {
   fn af_tim4_ch1(&self) -> usize;
}

pub trait AfTim1Ch4 {
   fn af_tim1_ch4(&self) -> usize;
}

pub trait AfTim1Bkin2 {
   fn af_tim1_bkin2(&self) -> usize;
}

pub trait AfI2sckin {
   fn af_i2sckin(&self) -> usize;
}

pub trait AfTim1Ch2n {
   fn af_tim1_ch2n(&self) -> usize;
}

pub trait AfUsart1Rts {
   fn af_usart1_rts(&self) -> usize;
}

pub trait AfCanTx {
   fn af_can_tx(&self) -> usize;
}

pub trait AfTim4Ch2 {
   fn af_tim4_ch2(&self) -> usize;
}

pub trait AfTim1Etr {
   fn af_tim1_etr(&self) -> usize;
}

pub trait AfSwdio {
   fn af_swdio(&self) -> usize;
}

pub trait AfJtms {
   fn af_jtms(&self) -> usize;
}

pub trait AfTim16Ch1n {
   fn af_tim16_ch1n(&self) -> usize;
}

pub trait AfTscG4Io3 {
   fn af_tsc_g4_io3(&self) -> usize;
}

pub trait AfIrOut {
   fn af_ir_out(&self) -> usize;
}

pub trait AfUsart3Cts {
   fn af_usart3_cts(&self) -> usize;
}

pub trait AfTim4Ch3 {
   fn af_tim4_ch3(&self) -> usize;
}

pub trait AfSwclk {
   fn af_swclk(&self) -> usize;
}

pub trait AfJtck {
   fn af_jtck(&self) -> usize;
}

pub trait AfTscG4Io4 {
   fn af_tsc_g4_io4(&self) -> usize;
}

pub trait AfI2c1Sda {
   fn af_i2c1_sda(&self) -> usize;
}

pub trait AfTim8Ch2 {
   fn af_tim8_ch2(&self) -> usize;
}

pub trait AfJtdi {
   fn af_jtdi(&self) -> usize;
}

pub trait AfTim8Ch1 {
   fn af_tim8_ch1(&self) -> usize;
}

pub trait AfTscSync {
   fn af_tsc_sync(&self) -> usize;
}

pub trait AfI2c1Scl {
   fn af_i2c1_scl(&self) -> usize;
}

pub trait AfTim3Ch3 {
   fn af_tim3_ch3(&self) -> usize;
}

pub trait AfTscG3Io2 {
   fn af_tsc_g3_io2(&self) -> usize;
}

pub trait AfTim8Ch2n {
   fn af_tim8_ch2n(&self) -> usize;
}

pub trait AfTim3Ch4 {
   fn af_tim3_ch4(&self) -> usize;
}

pub trait AfTscG3Io3 {
   fn af_tsc_g3_io3(&self) -> usize;
}

pub trait AfTim8Ch3n {
   fn af_tim8_ch3n(&self) -> usize;
}

pub trait AfTim1Ch3n {
   fn af_tim1_ch3n(&self) -> usize;
}

pub trait AfComp4Out {
   fn af_comp4_out(&self) -> usize;
}

pub trait AfTscG3Io4 {
   fn af_tsc_g3_io4(&self) -> usize;
}

pub trait AfJtdo {
   fn af_jtdo(&self) -> usize;
}

pub trait AfTraceswo {
   fn af_traceswo(&self) -> usize;
}

pub trait AfTscG5Io1 {
   fn af_tsc_g5_io1(&self) -> usize;
}

pub trait AfSpi3Sck {
   fn af_spi3_sck(&self) -> usize;
}

pub trait AfI2s3Ck {
   fn af_i2s3_ck(&self) -> usize;
}

pub trait AfTim3Etr {
   fn af_tim3_etr(&self) -> usize;
}

pub trait AfJtrst {
   fn af_jtrst(&self) -> usize;
}

pub trait AfSpi3Miso {
   fn af_spi3_miso(&self) -> usize;
}

pub trait AfI2s3extSd {
   fn af_i2s3ext_sd(&self) -> usize;
}

pub trait AfTim16Bkin {
   fn af_tim16_bkin(&self) -> usize;
}

pub trait AfI2c1Smbal {
   fn af_i2c1_smbal(&self) -> usize;
}

pub trait AfSpi3Mosi {
   fn af_spi3_mosi(&self) -> usize;
}

pub trait AfI2s3Sd {
   fn af_i2s3_sd(&self) -> usize;
}

pub trait AfI2c3Sda {
   fn af_i2c3_sda(&self) -> usize;
}

pub trait AfTscG5Io3 {
   fn af_tsc_g5_io3(&self) -> usize;
}

pub trait AfTim8Bkin2 {
   fn af_tim8_bkin2(&self) -> usize;
}

pub trait AfTim17Ch1n {
   fn af_tim17_ch1n(&self) -> usize;
}

pub trait AfTscG5Io4 {
   fn af_tsc_g5_io4(&self) -> usize;
}

pub trait AfFmcNadv {
   fn af_fmc_nadv(&self) -> usize;
}

pub trait AfUsart3Rx {
   fn af_usart3_rx(&self) -> usize;
}

pub trait AfTim4Ch4 {
   fn af_tim4_ch4(&self) -> usize;
}

pub trait AfUsart3Tx {
   fn af_usart3_tx(&self) -> usize;
}

pub trait AfTim8Ch3 {
   fn af_tim8_ch3(&self) -> usize;
}

pub trait AfTscG6Io1 {
   fn af_tsc_g6_io1(&self) -> usize;
}

pub trait AfTscG6Io2 {
   fn af_tsc_g6_io2(&self) -> usize;
}

pub trait AfSpi2Nss {
   fn af_spi2_nss(&self) -> usize;
}

pub trait AfI2s2Ws {
   fn af_i2s2_ws(&self) -> usize;
}

pub trait AfUsart3Ck {
   fn af_usart3_ck(&self) -> usize;
}

pub trait AfTscG6Io3 {
   fn af_tsc_g6_io3(&self) -> usize;
}

pub trait AfSpi2Sck {
   fn af_spi2_sck(&self) -> usize;
}

pub trait AfI2s2Ck {
   fn af_i2s2_ck(&self) -> usize;
}

pub trait AfTscG6Io4 {
   fn af_tsc_g6_io4(&self) -> usize;
}

pub trait AfUsart3Rts {
   fn af_usart3_rts(&self) -> usize;
}

pub trait AfComp7Out {
   fn af_comp7_out(&self) -> usize;
}

pub trait AfTim1Chetr {
   fn af_tim1_chetr(&self) -> usize;
}

pub trait AfTscG3Io1 {
   fn af_tsc_g3_io1(&self) -> usize;
}

pub trait AfTim8Ch4 {
   fn af_tim8_ch4(&self) -> usize;
}

pub trait AfUart4Tx {
   fn af_uart4_tx(&self) -> usize;
}

pub trait AfUart4Rx {
   fn af_uart4_rx(&self) -> usize;
}

pub trait AfUart5Tx {
   fn af_uart5_tx(&self) -> usize;
}

pub trait AfFmcD2 {
   fn af_fmc_d2(&self) -> usize;
}

pub trait AfFmcD3 {
   fn af_fmc_d3(&self) -> usize;
}

pub trait AfUart5Rx {
   fn af_uart5_rx(&self) -> usize;
}

pub trait AfFmcClk {
   fn af_fmc_clk(&self) -> usize;
}

pub trait AfFmcNoe {
   fn af_fmc_noe(&self) -> usize;
}

pub trait AfFmcNwe {
   fn af_fmc_nwe(&self) -> usize;
}

pub trait AfFmcNwait {
   fn af_fmc_nwait(&self) -> usize;
}

pub trait AfFmcNe1 {
   fn af_fmc_ne1(&self) -> usize;
}

pub trait AfFmcNce2 {
   fn af_fmc_nce2(&self) -> usize;
}

pub trait AfFmcD13 {
   fn af_fmc_d13(&self) -> usize;
}

pub trait AfFmcD14 {
   fn af_fmc_d14(&self) -> usize;
}

pub trait AfFmcD15 {
   fn af_fmc_d15(&self) -> usize;
}

pub trait AfFmcA16 {
   fn af_fmc_a16(&self) -> usize;
}

pub trait AfTscG8Io1 {
   fn af_tsc_g8_io1(&self) -> usize;
}

pub trait AfFmcA17 {
   fn af_fmc_a17(&self) -> usize;
}

pub trait AfTscG8Io2 {
   fn af_tsc_g8_io2(&self) -> usize;
}

pub trait AfFmcA18 {
   fn af_fmc_a18(&self) -> usize;
}

pub trait AfTscG8Io3 {
   fn af_tsc_g8_io3(&self) -> usize;
}

pub trait AfFmcD0 {
   fn af_fmc_d0(&self) -> usize;
}

pub trait AfTscG8Io4 {
   fn af_tsc_g8_io4(&self) -> usize;
}

pub trait AfFmcD1 {
   fn af_fmc_d1(&self) -> usize;
}

pub trait AfTim20Etr {
   fn af_tim20_etr(&self) -> usize;
}

pub trait AfFmcNbl0 {
   fn af_fmc_nbl0(&self) -> usize;
}

pub trait AfTim20Ch4 {
   fn af_tim20_ch4(&self) -> usize;
}

pub trait AfFmcNbl1 {
   fn af_fmc_nbl1(&self) -> usize;
}

pub trait AfTraceck {
   fn af_traceck(&self) -> usize;
}

pub trait AfTscG7Io1 {
   fn af_tsc_g7_io1(&self) -> usize;
}

pub trait AfSpi4Sck {
   fn af_spi4_sck(&self) -> usize;
}

pub trait AfTim20Ch1 {
   fn af_tim20_ch1(&self) -> usize;
}

pub trait AfFmcA23 {
   fn af_fmc_a23(&self) -> usize;
}

pub trait AfTraced0 {
   fn af_traced0(&self) -> usize;
}

pub trait AfTscG7Io2 {
   fn af_tsc_g7_io2(&self) -> usize;
}

pub trait AfSpi4Nss {
   fn af_spi4_nss(&self) -> usize;
}

pub trait AfTim20Ch2 {
   fn af_tim20_ch2(&self) -> usize;
}

pub trait AfFmcA19 {
   fn af_fmc_a19(&self) -> usize;
}

pub trait AfTraced1 {
   fn af_traced1(&self) -> usize;
}

pub trait AfTscG7Io3 {
   fn af_tsc_g7_io3(&self) -> usize;
}

pub trait AfTim20Ch1n {
   fn af_tim20_ch1n(&self) -> usize;
}

pub trait AfFmcA20 {
   fn af_fmc_a20(&self) -> usize;
}

pub trait AfTraced2 {
   fn af_traced2(&self) -> usize;
}

pub trait AfTscG7Io4 {
   fn af_tsc_g7_io4(&self) -> usize;
}

pub trait AfSpi4Miso {
   fn af_spi4_miso(&self) -> usize;
}

pub trait AfTim20Ch2n {
   fn af_tim20_ch2n(&self) -> usize;
}

pub trait AfFmcA21 {
   fn af_fmc_a21(&self) -> usize;
}

pub trait AfTraced3 {
   fn af_traced3(&self) -> usize;
}

pub trait AfSpi4Mosi {
   fn af_spi4_mosi(&self) -> usize;
}

pub trait AfTim20Ch3n {
   fn af_tim20_ch3n(&self) -> usize;
}

pub trait AfFmcA22 {
   fn af_fmc_a22(&self) -> usize;
}

pub trait AfFmcD4 {
   fn af_fmc_d4(&self) -> usize;
}

pub trait AfFmcD5 {
   fn af_fmc_d5(&self) -> usize;
}

pub trait AfFmcD6 {
   fn af_fmc_d6(&self) -> usize;
}

pub trait AfFmcD7 {
   fn af_fmc_d7(&self) -> usize;
}

pub trait AfFmcD8 {
   fn af_fmc_d8(&self) -> usize;
}

pub trait AfFmcD9 {
   fn af_fmc_d9(&self) -> usize;
}

pub trait AfFmcD10 {
   fn af_fmc_d10(&self) -> usize;
}

pub trait AfFmcD11 {
   fn af_fmc_d11(&self) -> usize;
}

pub trait AfFmcD12 {
   fn af_fmc_d12(&self) -> usize;
}

pub trait AfTim20Ch3 {
   fn af_tim20_ch3(&self) -> usize;
}

pub trait AfFmcA2 {
   fn af_fmc_a2(&self) -> usize;
}

pub trait AfFmcA3 {
   fn af_fmc_a3(&self) -> usize;
}

pub trait AfFmcA4 {
   fn af_fmc_a4(&self) -> usize;
}

pub trait AfFmcA5 {
   fn af_fmc_a5(&self) -> usize;
}

pub trait AfFmcNiord {
   fn af_fmc_niord(&self) -> usize;
}

pub trait AfTim20Bkin {
   fn af_tim20_bkin(&self) -> usize;
}

pub trait AfFmcNreg {
   fn af_fmc_nreg(&self) -> usize;
}

pub trait AfTim20Bkin2 {
   fn af_tim20_bkin2(&self) -> usize;
}

pub trait AfFmcNiowr {
   fn af_fmc_niowr(&self) -> usize;
}

pub trait AfFmcCd {
   fn af_fmc_cd(&self) -> usize;
}

pub trait AfFmcIntr {
   fn af_fmc_intr(&self) -> usize;
}

pub trait AfFmcA6 {
   fn af_fmc_a6(&self) -> usize;
}

pub trait AfFmcA7 {
   fn af_fmc_a7(&self) -> usize;
}

pub trait AfFmcA8 {
   fn af_fmc_a8(&self) -> usize;
}

pub trait AfFmcA9 {
   fn af_fmc_a9(&self) -> usize;
}

pub trait AfFmcA10 {
   fn af_fmc_a10(&self) -> usize;
}

pub trait AfFmcA11 {
   fn af_fmc_a11(&self) -> usize;
}

pub trait AfFmcA12 {
   fn af_fmc_a12(&self) -> usize;
}

pub trait AfFmcA13 {
   fn af_fmc_a13(&self) -> usize;
}

pub trait AfFmcA14 {
   fn af_fmc_a14(&self) -> usize;
}

pub trait AfFmcA15 {
   fn af_fmc_a15(&self) -> usize;
}

pub trait AfFmcInt2 {
   fn af_fmc_int2(&self) -> usize;
}

pub trait AfFmcInt3 {
   fn af_fmc_int3(&self) -> usize;
}

pub trait AfFmcNe2 {
   fn af_fmc_ne2(&self) -> usize;
}

pub trait AfFmcNce3 {
   fn af_fmc_nce3(&self) -> usize;
}

pub trait AfFmcNce41 {
   fn af_fmc_nce4_1(&self) -> usize;
}

pub trait AfFmcNe3 {
   fn af_fmc_ne3(&self) -> usize;
}

pub trait AfFmc42 {
   fn af_fmc_4_2(&self) -> usize;
}

pub trait AfFmcNe4 {
   fn af_fmc_ne4(&self) -> usize;
}

pub trait AfFmcA24 {
   fn af_fmc_a24(&self) -> usize;
}

pub trait AfFmcA25 {
   fn af_fmc_a25(&self) -> usize;
}

pub trait AfFmcA0 {
   fn af_fmc_a0(&self) -> usize;
}

pub trait AfFmcA1 {
   fn af_fmc_a1(&self) -> usize;
}

pub const PA0: Pa0 = Pa0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa0 {}

impl Pin for Pa0 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 0 }
}

impl AfTim2Ch1 for Pa0 {
   fn af_tim2_ch1(&self) -> usize { 1 }
}

impl AfTim2Etr for Pa0 {
   fn af_tim2_etr(&self) -> usize { 1 }
}

impl AfTscG1Io1 for Pa0 {
   fn af_tsc_g1_io1(&self) -> usize { 3 }
}

impl AfUsart2Cts for Pa0 {
   fn af_usart2_cts(&self) -> usize { 7 }
}

impl AfComp1Out for Pa0 {
   fn af_comp1_out(&self) -> usize { 8 }
}

impl AfTim8Bkin for Pa0 {
   fn af_tim8_bkin(&self) -> usize { 9 }
}

impl AfTim8Etr for Pa0 {
   fn af_tim8_etr(&self) -> usize { 10 }
}

impl AfEventOut for Pa0 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PA1: Pa1 = Pa1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa1 {}

impl Pin for Pa1 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 1 }
}

impl AfRtcRefin for Pa1 {
   fn af_rtc_refin(&self) -> usize { 0 }
}

impl AfTim2Ch2 for Pa1 {
   fn af_tim2_ch2(&self) -> usize { 1 }
}

impl AfTscG1Io2 for Pa1 {
   fn af_tsc_g1_io2(&self) -> usize { 3 }
}

impl AfUsart2Rts for Pa1 {
   fn af_usart2_rts(&self) -> usize { 7 }
}

impl AfTim15Ch1n for Pa1 {
   fn af_tim15_ch1n(&self) -> usize { 9 }
}

impl AfEventOut for Pa1 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PA2: Pa2 = Pa2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa2 {}

impl Pin for Pa2 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 2 }
}

impl AfTim2Ch3 for Pa2 {
   fn af_tim2_ch3(&self) -> usize { 1 }
}

impl AfTscG1Io3 for Pa2 {
   fn af_tsc_g1_io3(&self) -> usize { 3 }
}

impl AfUsart2Tx for Pa2 {
   fn af_usart2_tx(&self) -> usize { 7 }
}

impl AfComp2Out for Pa2 {
   fn af_comp2_out(&self) -> usize { 8 }
}

impl AfTim15Ch1 for Pa2 {
   fn af_tim15_ch1(&self) -> usize { 9 }
}

impl AfEventOut for Pa2 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PA3: Pa3 = Pa3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa3 {}

impl Pin for Pa3 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 3 }
}

impl AfTim2Ch3 for Pa3 {
   fn af_tim2_ch3(&self) -> usize { 1 }
}

impl AfTscG1Io4 for Pa3 {
   fn af_tsc_g1_io4(&self) -> usize { 3 }
}

impl AfUsart2Rx for Pa3 {
   fn af_usart2_rx(&self) -> usize { 7 }
}

impl AfTim15Ch2 for Pa3 {
   fn af_tim15_ch2(&self) -> usize { 9 }
}

impl AfEventOut for Pa3 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PA4: Pa4 = Pa4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa4 {}

impl Pin for Pa4 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 4 }
}

impl AfTim3Ch2 for Pa4 {
   fn af_tim3_ch2(&self) -> usize { 2 }
}

impl AfTscG2Io1 for Pa4 {
   fn af_tsc_g2_io1(&self) -> usize { 3 }
}

impl AfSpi1Nss for Pa4 {
   fn af_spi1_nss(&self) -> usize { 5 }
}

impl AfSpi3Nss for Pa4 {
   fn af_spi3_nss(&self) -> usize { 6 }
}

impl AfI2s3Ws for Pa4 {
   fn af_i2s3_ws(&self) -> usize { 6 }
}

impl AfUsart2Ck for Pa4 {
   fn af_usart2_ck(&self) -> usize { 7 }
}

impl AfEventOut for Pa4 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PA5: Pa5 = Pa5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa5 {}

impl Pin for Pa5 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 5 }
}

impl AfTim2Ch1 for Pa5 {
   fn af_tim2_ch1(&self) -> usize { 1 }
}

impl AfTim2Etr for Pa5 {
   fn af_tim2_etr(&self) -> usize { 1 }
}

impl AfTscG2Io2 for Pa5 {
   fn af_tsc_g2_io2(&self) -> usize { 3 }
}

impl AfSpi1Sck for Pa5 {
   fn af_spi1_sck(&self) -> usize { 5 }
}

impl AfEventOut for Pa5 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PA6: Pa6 = Pa6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa6 {}

impl Pin for Pa6 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 6 }
}

impl AfTim16Ch1 for Pa6 {
   fn af_tim16_ch1(&self) -> usize { 1 }
}

impl AfTim3Ch1 for Pa6 {
   fn af_tim3_ch1(&self) -> usize { 2 }
}

impl AfTscG2Io3 for Pa6 {
   fn af_tsc_g2_io3(&self) -> usize { 3 }
}

impl AfTim8Bkin for Pa6 {
   fn af_tim8_bkin(&self) -> usize { 4 }
}

impl AfSpi1Miso for Pa6 {
   fn af_spi1_miso(&self) -> usize { 5 }
}

impl AfTim1Bkin for Pa6 {
   fn af_tim1_bkin(&self) -> usize { 6 }
}

impl AfComp1Out for Pa6 {
   fn af_comp1_out(&self) -> usize { 8 }
}

impl AfEventOut for Pa6 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PA7: Pa7 = Pa7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa7 {}

impl Pin for Pa7 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 7 }
}

impl AfTim17Ch1 for Pa7 {
   fn af_tim17_ch1(&self) -> usize { 1 }
}

impl AfTim3Ch2 for Pa7 {
   fn af_tim3_ch2(&self) -> usize { 2 }
}

impl AfTscG2Io4 for Pa7 {
   fn af_tsc_g2_io4(&self) -> usize { 3 }
}

impl AfTim8Ch1n for Pa7 {
   fn af_tim8_ch1n(&self) -> usize { 4 }
}

impl AfSpi1Mosi for Pa7 {
   fn af_spi1_mosi(&self) -> usize { 5 }
}

impl AfTim1Ch1n for Pa7 {
   fn af_tim1_ch1n(&self) -> usize { 6 }
}

impl AfEventOut for Pa7 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PA8: Pa8 = Pa8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa8 {}

impl Pin for Pa8 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 8 }
}

impl AfMco for Pa8 {
   fn af_mco(&self) -> usize { 0 }
}

impl AfI2c3Scl for Pa8 {
   fn af_i2c3_scl(&self) -> usize { 3 }
}

impl AfI2c2Smbal for Pa8 {
   fn af_i2c2_smbal(&self) -> usize { 4 }
}

impl AfI2s2Mck for Pa8 {
   fn af_i2s2_mck(&self) -> usize { 5 }
}

impl AfTim1Ch1 for Pa8 {
   fn af_tim1_ch1(&self) -> usize { 6 }
}

impl AfUsart1Ck for Pa8 {
   fn af_usart1_ck(&self) -> usize { 7 }
}

impl AfComp3Out for Pa8 {
   fn af_comp3_out(&self) -> usize { 8 }
}

impl AfTim4Etr for Pa8 {
   fn af_tim4_etr(&self) -> usize { 10 }
}

impl AfEventOut for Pa8 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PA9: Pa9 = Pa9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa9 {}

impl Pin for Pa9 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 9 }
}

impl AfI2c3Smbal for Pa9 {
   fn af_i2c3_smbal(&self) -> usize { 2 }
}

impl AfTscG4Io1 for Pa9 {
   fn af_tsc_g4_io1(&self) -> usize { 3 }
}

impl AfI2c2Scl for Pa9 {
   fn af_i2c2_scl(&self) -> usize { 4 }
}

impl AfI2s3Mck for Pa9 {
   fn af_i2s3_mck(&self) -> usize { 5 }
}

impl AfTim1Ch2 for Pa9 {
   fn af_tim1_ch2(&self) -> usize { 6 }
}

impl AfUsart1Tx for Pa9 {
   fn af_usart1_tx(&self) -> usize { 7 }
}

impl AfComp5Out for Pa9 {
   fn af_comp5_out(&self) -> usize { 8 }
}

impl AfTim15Bkin for Pa9 {
   fn af_tim15_bkin(&self) -> usize { 9 }
}

impl AfTim2Ch3 for Pa9 {
   fn af_tim2_ch3(&self) -> usize { 10 }
}

impl AfEventOut for Pa9 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PA10: Pa10 = Pa10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa10 {}

impl Pin for Pa10 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 10 }
}

impl AfTim17Bkin for Pa10 {
   fn af_tim17_bkin(&self) -> usize { 1 }
}

impl AfTscG4Io2 for Pa10 {
   fn af_tsc_g4_io2(&self) -> usize { 3 }
}

impl AfI2c2Sda for Pa10 {
   fn af_i2c2_sda(&self) -> usize { 4 }
}

impl AfSpi2Miso for Pa10 {
   fn af_spi2_miso(&self) -> usize { 5 }
}

impl AfI2s2extSd for Pa10 {
   fn af_i2s2ext_sd(&self) -> usize { 5 }
}

impl AfTim1Ch3 for Pa10 {
   fn af_tim1_ch3(&self) -> usize { 6 }
}

impl AfUsart1Rx for Pa10 {
   fn af_usart1_rx(&self) -> usize { 7 }
}

impl AfComp6Out for Pa10 {
   fn af_comp6_out(&self) -> usize { 8 }
}

impl AfTim2Ch4 for Pa10 {
   fn af_tim2_ch4(&self) -> usize { 10 }
}

impl AfTim8Bkin for Pa10 {
   fn af_tim8_bkin(&self) -> usize { 11 }
}

impl AfEventOut for Pa10 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PA11: Pa11 = Pa11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa11 {}

impl Pin for Pa11 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 11 }
}

impl AfSpi2Mosi for Pa11 {
   fn af_spi2_mosi(&self) -> usize { 5 }
}

impl AfI2s2Sd for Pa11 {
   fn af_i2s2_sd(&self) -> usize { 5 }
}

impl AfTim1Ch1n for Pa11 {
   fn af_tim1_ch1n(&self) -> usize { 6 }
}

impl AfUsart1Cts for Pa11 {
   fn af_usart1_cts(&self) -> usize { 7 }
}

impl AfComp1Out for Pa11 {
   fn af_comp1_out(&self) -> usize { 8 }
}

impl AfCanRx for Pa11 {
   fn af_can_rx(&self) -> usize { 9 }
}

impl AfTim4Ch1 for Pa11 {
   fn af_tim4_ch1(&self) -> usize { 10 }
}

impl AfTim1Ch4 for Pa11 {
   fn af_tim1_ch4(&self) -> usize { 11 }
}

impl AfTim1Bkin2 for Pa11 {
   fn af_tim1_bkin2(&self) -> usize { 12 }
}

impl AfEventOut for Pa11 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PA12: Pa12 = Pa12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa12 {}

impl Pin for Pa12 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 12 }
}

impl AfTim16Ch1 for Pa12 {
   fn af_tim16_ch1(&self) -> usize { 1 }
}

impl AfI2sckin for Pa12 {
   fn af_i2sckin(&self) -> usize { 5 }
}

impl AfTim1Ch2n for Pa12 {
   fn af_tim1_ch2n(&self) -> usize { 6 }
}

impl AfUsart1Rts for Pa12 {
   fn af_usart1_rts(&self) -> usize { 7 }
}

impl AfComp2Out for Pa12 {
   fn af_comp2_out(&self) -> usize { 8 }
}

impl AfCanTx for Pa12 {
   fn af_can_tx(&self) -> usize { 9 }
}

impl AfTim4Ch2 for Pa12 {
   fn af_tim4_ch2(&self) -> usize { 10 }
}

impl AfTim1Etr for Pa12 {
   fn af_tim1_etr(&self) -> usize { 11 }
}

impl AfEventOut for Pa12 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PA13: Pa13 = Pa13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa13 {}

impl Pin for Pa13 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 13 }
}

impl AfSwdio for Pa13 {
   fn af_swdio(&self) -> usize { 0 }
}

impl AfJtms for Pa13 {
   fn af_jtms(&self) -> usize { 0 }
}

impl AfTim16Ch1n for Pa13 {
   fn af_tim16_ch1n(&self) -> usize { 1 }
}

impl AfTscG4Io3 for Pa13 {
   fn af_tsc_g4_io3(&self) -> usize { 3 }
}

impl AfIrOut for Pa13 {
   fn af_ir_out(&self) -> usize { 5 }
}

impl AfUsart3Cts for Pa13 {
   fn af_usart3_cts(&self) -> usize { 7 }
}

impl AfTim4Ch3 for Pa13 {
   fn af_tim4_ch3(&self) -> usize { 10 }
}

impl AfEventOut for Pa13 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PA14: Pa14 = Pa14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa14 {}

impl Pin for Pa14 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 14 }
}

impl AfSwclk for Pa14 {
   fn af_swclk(&self) -> usize { 0 }
}

impl AfJtck for Pa14 {
   fn af_jtck(&self) -> usize { 0 }
}

impl AfTscG4Io4 for Pa14 {
   fn af_tsc_g4_io4(&self) -> usize { 3 }
}

impl AfI2c1Sda for Pa14 {
   fn af_i2c1_sda(&self) -> usize { 4 }
}

impl AfTim8Ch2 for Pa14 {
   fn af_tim8_ch2(&self) -> usize { 5 }
}

impl AfTim1Bkin for Pa14 {
   fn af_tim1_bkin(&self) -> usize { 6 }
}

impl AfUsart2Tx for Pa14 {
   fn af_usart2_tx(&self) -> usize { 7 }
}

impl AfEventOut for Pa14 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PA15: Pa15 = Pa15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa15 {}

impl Pin for Pa15 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 15 }
}

impl AfJtdi for Pa15 {
   fn af_jtdi(&self) -> usize { 0 }
}

impl AfTim2Ch1 for Pa15 {
   fn af_tim2_ch1(&self) -> usize { 1 }
}

impl AfTim2Etr for Pa15 {
   fn af_tim2_etr(&self) -> usize { 1 }
}

impl AfTim8Ch1 for Pa15 {
   fn af_tim8_ch1(&self) -> usize { 2 }
}

impl AfTscSync for Pa15 {
   fn af_tsc_sync(&self) -> usize { 3 }
}

impl AfI2c1Scl for Pa15 {
   fn af_i2c1_scl(&self) -> usize { 4 }
}

impl AfSpi1Nss for Pa15 {
   fn af_spi1_nss(&self) -> usize { 5 }
}

impl AfSpi3Nss for Pa15 {
   fn af_spi3_nss(&self) -> usize { 6 }
}

impl AfI2s3Ws for Pa15 {
   fn af_i2s3_ws(&self) -> usize { 6 }
}

impl AfUsart2Rx for Pa15 {
   fn af_usart2_rx(&self) -> usize { 7 }
}

impl AfTim1Bkin for Pa15 {
   fn af_tim1_bkin(&self) -> usize { 9 }
}

impl AfEventOut for Pa15 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PB0: Pb0 = Pb0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb0 {}

impl Pin for Pb0 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 0 }
}

impl AfTim3Ch3 for Pb0 {
   fn af_tim3_ch3(&self) -> usize { 2 }
}

impl AfTscG3Io2 for Pb0 {
   fn af_tsc_g3_io2(&self) -> usize { 3 }
}

impl AfTim8Ch2n for Pb0 {
   fn af_tim8_ch2n(&self) -> usize { 4 }
}

impl AfTim1Ch2n for Pb0 {
   fn af_tim1_ch2n(&self) -> usize { 6 }
}

impl AfEventOut for Pb0 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PB1: Pb1 = Pb1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb1 {}

impl Pin for Pb1 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 1 }
}

impl AfTim3Ch4 for Pb1 {
   fn af_tim3_ch4(&self) -> usize { 2 }
}

impl AfTscG3Io3 for Pb1 {
   fn af_tsc_g3_io3(&self) -> usize { 3 }
}

impl AfTim8Ch3n for Pb1 {
   fn af_tim8_ch3n(&self) -> usize { 4 }
}

impl AfTim1Ch3n for Pb1 {
   fn af_tim1_ch3n(&self) -> usize { 6 }
}

impl AfComp4Out for Pb1 {
   fn af_comp4_out(&self) -> usize { 8 }
}

impl AfEventOut for Pb1 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PB2: Pb2 = Pb2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb2 {}

impl Pin for Pb2 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 2 }
}

impl AfTscG3Io4 for Pb2 {
   fn af_tsc_g3_io4(&self) -> usize { 3 }
}

impl AfEventOut for Pb2 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PB3: Pb3 = Pb3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb3 {}

impl Pin for Pb3 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 3 }
}

impl AfJtdo for Pb3 {
   fn af_jtdo(&self) -> usize { 0 }
}

impl AfTraceswo for Pb3 {
   fn af_traceswo(&self) -> usize { 0 }
}

impl AfTim2Ch2 for Pb3 {
   fn af_tim2_ch2(&self) -> usize { 1 }
}

impl AfTim4Etr for Pb3 {
   fn af_tim4_etr(&self) -> usize { 2 }
}

impl AfTscG5Io1 for Pb3 {
   fn af_tsc_g5_io1(&self) -> usize { 3 }
}

impl AfTim8Ch1n for Pb3 {
   fn af_tim8_ch1n(&self) -> usize { 4 }
}

impl AfSpi1Sck for Pb3 {
   fn af_spi1_sck(&self) -> usize { 5 }
}

impl AfSpi3Sck for Pb3 {
   fn af_spi3_sck(&self) -> usize { 6 }
}

impl AfI2s3Ck for Pb3 {
   fn af_i2s3_ck(&self) -> usize { 6 }
}

impl AfUsart2Tx for Pb3 {
   fn af_usart2_tx(&self) -> usize { 7 }
}

impl AfTim3Etr for Pb3 {
   fn af_tim3_etr(&self) -> usize { 10 }
}

impl AfEventOut for Pb3 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PB4: Pb4 = Pb4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb4 {}

impl Pin for Pb4 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 4 }
}

impl AfJtrst for Pb4 {
   fn af_jtrst(&self) -> usize { 0 }
}

impl AfTim16Ch1 for Pb4 {
   fn af_tim16_ch1(&self) -> usize { 1 }
}

impl AfTim3Ch1 for Pb4 {
   fn af_tim3_ch1(&self) -> usize { 2 }
}

impl AfTscG5Io1 for Pb4 {
   fn af_tsc_g5_io1(&self) -> usize { 3 }
}

impl AfTim8Ch2n for Pb4 {
   fn af_tim8_ch2n(&self) -> usize { 4 }
}

impl AfSpi1Miso for Pb4 {
   fn af_spi1_miso(&self) -> usize { 5 }
}

impl AfSpi3Miso for Pb4 {
   fn af_spi3_miso(&self) -> usize { 6 }
}

impl AfI2s3extSd for Pb4 {
   fn af_i2s3ext_sd(&self) -> usize { 6 }
}

impl AfUsart2Rx for Pb4 {
   fn af_usart2_rx(&self) -> usize { 7 }
}

impl AfTim17Bkin for Pb4 {
   fn af_tim17_bkin(&self) -> usize { 10 }
}

impl AfEventOut for Pb4 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PB5: Pb5 = Pb5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb5 {}

impl Pin for Pb5 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 5 }
}

impl AfTim16Bkin for Pb5 {
   fn af_tim16_bkin(&self) -> usize { 1 }
}

impl AfTim3Ch2 for Pb5 {
   fn af_tim3_ch2(&self) -> usize { 2 }
}

impl AfTim8Ch3n for Pb5 {
   fn af_tim8_ch3n(&self) -> usize { 3 }
}

impl AfI2c1Smbal for Pb5 {
   fn af_i2c1_smbal(&self) -> usize { 4 }
}

impl AfSpi1Mosi for Pb5 {
   fn af_spi1_mosi(&self) -> usize { 5 }
}

impl AfSpi3Mosi for Pb5 {
   fn af_spi3_mosi(&self) -> usize { 6 }
}

impl AfI2s3Sd for Pb5 {
   fn af_i2s3_sd(&self) -> usize { 6 }
}

impl AfUsart2Ck for Pb5 {
   fn af_usart2_ck(&self) -> usize { 7 }
}

impl AfI2c3Sda for Pb5 {
   fn af_i2c3_sda(&self) -> usize { 8 }
}

impl AfTim17Ch1 for Pb5 {
   fn af_tim17_ch1(&self) -> usize { 10 }
}

impl AfEventOut for Pb5 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PB6: Pb6 = Pb6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb6 {}

impl Pin for Pb6 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 6 }
}

impl AfTim16Ch1n for Pb6 {
   fn af_tim16_ch1n(&self) -> usize { 1 }
}

impl AfTim4Ch1 for Pb6 {
   fn af_tim4_ch1(&self) -> usize { 2 }
}

impl AfTscG5Io3 for Pb6 {
   fn af_tsc_g5_io3(&self) -> usize { 3 }
}

impl AfI2c1Scl for Pb6 {
   fn af_i2c1_scl(&self) -> usize { 4 }
}

impl AfTim8Ch1 for Pb6 {
   fn af_tim8_ch1(&self) -> usize { 5 }
}

impl AfTim8Etr for Pb6 {
   fn af_tim8_etr(&self) -> usize { 6 }
}

impl AfUsart1Tx for Pb6 {
   fn af_usart1_tx(&self) -> usize { 7 }
}

impl AfTim8Bkin2 for Pb6 {
   fn af_tim8_bkin2(&self) -> usize { 10 }
}

impl AfEventOut for Pb6 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PB7: Pb7 = Pb7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb7 {}

impl Pin for Pb7 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 7 }
}

impl AfTim17Ch1n for Pb7 {
   fn af_tim17_ch1n(&self) -> usize { 1 }
}

impl AfTim4Ch2 for Pb7 {
   fn af_tim4_ch2(&self) -> usize { 2 }
}

impl AfTscG5Io4 for Pb7 {
   fn af_tsc_g5_io4(&self) -> usize { 3 }
}

impl AfI2c1Sda for Pb7 {
   fn af_i2c1_sda(&self) -> usize { 4 }
}

impl AfTim8Bkin for Pb7 {
   fn af_tim8_bkin(&self) -> usize { 5 }
}

impl AfUsart1Rx for Pb7 {
   fn af_usart1_rx(&self) -> usize { 7 }
}

impl AfTim3Ch4 for Pb7 {
   fn af_tim3_ch4(&self) -> usize { 10 }
}

impl AfFmcNadv for Pb7 {
   fn af_fmc_nadv(&self) -> usize { 12 }
}

impl AfEventOut for Pb7 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PB8: Pb8 = Pb8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb8 {}

impl Pin for Pb8 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 8 }
}

impl AfTim16Ch1 for Pb8 {
   fn af_tim16_ch1(&self) -> usize { 1 }
}

impl AfTim4Ch3 for Pb8 {
   fn af_tim4_ch3(&self) -> usize { 2 }
}

impl AfTscSync for Pb8 {
   fn af_tsc_sync(&self) -> usize { 3 }
}

impl AfI2c1Scl for Pb8 {
   fn af_i2c1_scl(&self) -> usize { 4 }
}

impl AfUsart3Rx for Pb8 {
   fn af_usart3_rx(&self) -> usize { 7 }
}

impl AfComp1Out for Pb8 {
   fn af_comp1_out(&self) -> usize { 8 }
}

impl AfCanRx for Pb8 {
   fn af_can_rx(&self) -> usize { 9 }
}

impl AfTim8Ch2 for Pb8 {
   fn af_tim8_ch2(&self) -> usize { 10 }
}

impl AfTim1Bkin for Pb8 {
   fn af_tim1_bkin(&self) -> usize { 12 }
}

impl AfEventOut for Pb8 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PB9: Pb9 = Pb9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb9 {}

impl Pin for Pb9 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 9 }
}

impl AfTim17Ch1 for Pb9 {
   fn af_tim17_ch1(&self) -> usize { 1 }
}

impl AfTim4Ch4 for Pb9 {
   fn af_tim4_ch4(&self) -> usize { 2 }
}

impl AfI2c1Sda for Pb9 {
   fn af_i2c1_sda(&self) -> usize { 4 }
}

impl AfIrOut for Pb9 {
   fn af_ir_out(&self) -> usize { 6 }
}

impl AfUsart3Tx for Pb9 {
   fn af_usart3_tx(&self) -> usize { 7 }
}

impl AfComp2Out for Pb9 {
   fn af_comp2_out(&self) -> usize { 8 }
}

impl AfCanTx for Pb9 {
   fn af_can_tx(&self) -> usize { 9 }
}

impl AfTim8Ch3 for Pb9 {
   fn af_tim8_ch3(&self) -> usize { 10 }
}

impl AfEventOut for Pb9 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PB10: Pb10 = Pb10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb10 {}

impl Pin for Pb10 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 10 }
}

impl AfTim2Ch3 for Pb10 {
   fn af_tim2_ch3(&self) -> usize { 1 }
}

impl AfTscSync for Pb10 {
   fn af_tsc_sync(&self) -> usize { 3 }
}

impl AfUsart3Tx for Pb10 {
   fn af_usart3_tx(&self) -> usize { 7 }
}

impl AfEventOut for Pb10 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PB11: Pb11 = Pb11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb11 {}

impl Pin for Pb11 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 11 }
}

impl AfTim2Ch4 for Pb11 {
   fn af_tim2_ch4(&self) -> usize { 1 }
}

impl AfTscG6Io1 for Pb11 {
   fn af_tsc_g6_io1(&self) -> usize { 3 }
}

impl AfUsart3Rx for Pb11 {
   fn af_usart3_rx(&self) -> usize { 7 }
}

impl AfEventOut for Pb11 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PB12: Pb12 = Pb12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb12 {}

impl Pin for Pb12 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 12 }
}

impl AfTscG6Io2 for Pb12 {
   fn af_tsc_g6_io2(&self) -> usize { 3 }
}

impl AfI2c1Smbal for Pb12 {
   fn af_i2c1_smbal(&self) -> usize { 4 }
}

impl AfSpi2Nss for Pb12 {
   fn af_spi2_nss(&self) -> usize { 5 }
}

impl AfI2s2Ws for Pb12 {
   fn af_i2s2_ws(&self) -> usize { 5 }
}

impl AfTim1Bkin for Pb12 {
   fn af_tim1_bkin(&self) -> usize { 6 }
}

impl AfUsart3Ck for Pb12 {
   fn af_usart3_ck(&self) -> usize { 7 }
}

impl AfEventOut for Pb12 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PB13: Pb13 = Pb13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb13 {}

impl Pin for Pb13 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 13 }
}

impl AfTscG6Io3 for Pb13 {
   fn af_tsc_g6_io3(&self) -> usize { 3 }
}

impl AfSpi2Sck for Pb13 {
   fn af_spi2_sck(&self) -> usize { 5 }
}

impl AfI2s2Ck for Pb13 {
   fn af_i2s2_ck(&self) -> usize { 5 }
}

impl AfTim1Ch1n for Pb13 {
   fn af_tim1_ch1n(&self) -> usize { 6 }
}

impl AfUsart3Cts for Pb13 {
   fn af_usart3_cts(&self) -> usize { 7 }
}

impl AfEventOut for Pb13 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PB14: Pb14 = Pb14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb14 {}

impl Pin for Pb14 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 14 }
}

impl AfTim15Ch1 for Pb14 {
   fn af_tim15_ch1(&self) -> usize { 1 }
}

impl AfTscG6Io4 for Pb14 {
   fn af_tsc_g6_io4(&self) -> usize { 3 }
}

impl AfSpi2Miso for Pb14 {
   fn af_spi2_miso(&self) -> usize { 5 }
}

impl AfI2s2extSd for Pb14 {
   fn af_i2s2ext_sd(&self) -> usize { 5 }
}

impl AfTim1Ch2n for Pb14 {
   fn af_tim1_ch2n(&self) -> usize { 6 }
}

impl AfUsart3Rts for Pb14 {
   fn af_usart3_rts(&self) -> usize { 7 }
}

impl AfEventOut for Pb14 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PB15: Pb15 = Pb15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb15 {}

impl Pin for Pb15 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 15 }
}

impl AfRtcRefin for Pb15 {
   fn af_rtc_refin(&self) -> usize { 0 }
}

impl AfTim15Ch2 for Pb15 {
   fn af_tim15_ch2(&self) -> usize { 1 }
}

impl AfTim15Ch1n for Pb15 {
   fn af_tim15_ch1n(&self) -> usize { 2 }
}

impl AfTim1Ch3n for Pb15 {
   fn af_tim1_ch3n(&self) -> usize { 3 }
}

impl AfSpi2Mosi for Pb15 {
   fn af_spi2_mosi(&self) -> usize { 5 }
}

impl AfI2s2Sd for Pb15 {
   fn af_i2s2_sd(&self) -> usize { 5 }
}

impl AfEventOut for Pb15 {
   fn af_event_out(&self) -> usize { 15 }
}

pub const PC0: Pc0 = Pc0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc0 {}

impl Pin for Pc0 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 0 }
}

impl AfEventOut for Pc0 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim1Ch1 for Pc0 {
   fn af_tim1_ch1(&self) -> usize { 2 }
}

pub const PC1: Pc1 = Pc1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc1 {}

impl Pin for Pc1 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 1 }
}

impl AfEventOut for Pc1 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim1Ch2 for Pc1 {
   fn af_tim1_ch2(&self) -> usize { 2 }
}

pub const PC2: Pc2 = Pc2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc2 {}

impl Pin for Pc2 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 2 }
}

impl AfEventOut for Pc2 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim1Ch3 for Pc2 {
   fn af_tim1_ch3(&self) -> usize { 2 }
}

impl AfComp7Out for Pc2 {
   fn af_comp7_out(&self) -> usize { 3 }
}

pub const PC3: Pc3 = Pc3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc3 {}

impl Pin for Pc3 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 3 }
}

impl AfEventOut for Pc3 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim1Ch4 for Pc3 {
   fn af_tim1_ch4(&self) -> usize { 2 }
}

impl AfTim1Bkin2 for Pc3 {
   fn af_tim1_bkin2(&self) -> usize { 6 }
}

pub const PC4: Pc4 = Pc4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc4 {}

impl Pin for Pc4 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 4 }
}

impl AfEventOut for Pc4 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim1Chetr for Pc4 {
   fn af_tim1_chetr(&self) -> usize { 2 }
}

impl AfUsart1Tx for Pc4 {
   fn af_usart1_tx(&self) -> usize { 7 }
}

pub const PC5: Pc5 = Pc5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc5 {}

impl Pin for Pc5 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 5 }
}

impl AfEventOut for Pc5 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim15Bkin for Pc5 {
   fn af_tim15_bkin(&self) -> usize { 2 }
}

impl AfTscG3Io1 for Pc5 {
   fn af_tsc_g3_io1(&self) -> usize { 3 }
}

impl AfUsart1Rx for Pc5 {
   fn af_usart1_rx(&self) -> usize { 7 }
}

pub const PC6: Pc6 = Pc6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc6 {}

impl Pin for Pc6 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 6 }
}

impl AfEventOut for Pc6 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim3Ch1 for Pc6 {
   fn af_tim3_ch1(&self) -> usize { 2 }
}

impl AfTim8Ch1 for Pc6 {
   fn af_tim8_ch1(&self) -> usize { 3 }
}

impl AfI2s2Mck for Pc6 {
   fn af_i2s2_mck(&self) -> usize { 6 }
}

impl AfComp6Out for Pc6 {
   fn af_comp6_out(&self) -> usize { 7 }
}

pub const PC7: Pc7 = Pc7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc7 {}

impl Pin for Pc7 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 7 }
}

impl AfEventOut for Pc7 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim3Ch2 for Pc7 {
   fn af_tim3_ch2(&self) -> usize { 2 }
}

impl AfTim8Ch2 for Pc7 {
   fn af_tim8_ch2(&self) -> usize { 3 }
}

impl AfI2s3Mck for Pc7 {
   fn af_i2s3_mck(&self) -> usize { 6 }
}

impl AfComp5Out for Pc7 {
   fn af_comp5_out(&self) -> usize { 7 }
}

pub const PC8: Pc8 = Pc8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc8 {}

impl Pin for Pc8 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 8 }
}

impl AfEventOut for Pc8 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim3Ch3 for Pc8 {
   fn af_tim3_ch3(&self) -> usize { 2 }
}

impl AfTim8Ch3 for Pc8 {
   fn af_tim8_ch3(&self) -> usize { 3 }
}

impl AfComp3Out for Pc8 {
   fn af_comp3_out(&self) -> usize { 7 }
}

pub const PC9: Pc9 = Pc9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc9 {}

impl Pin for Pc9 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 9 }
}

impl AfEventOut for Pc9 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim3Ch4 for Pc9 {
   fn af_tim3_ch4(&self) -> usize { 2 }
}

impl AfI2c3Sda for Pc9 {
   fn af_i2c3_sda(&self) -> usize { 3 }
}

impl AfTim8Ch4 for Pc9 {
   fn af_tim8_ch4(&self) -> usize { 3 }
}

impl AfI2sckin for Pc9 {
   fn af_i2sckin(&self) -> usize { 5 }
}

impl AfTim8Bkin2 for Pc9 {
   fn af_tim8_bkin2(&self) -> usize { 6 }
}

pub const PC10: Pc10 = Pc10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc10 {}

impl Pin for Pc10 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 10 }
}

impl AfEventOut for Pc10 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim8Ch1n for Pc10 {
   fn af_tim8_ch1n(&self) -> usize { 3 }
}

impl AfUart4Tx for Pc10 {
   fn af_uart4_tx(&self) -> usize { 5 }
}

impl AfSpi3Sck for Pc10 {
   fn af_spi3_sck(&self) -> usize { 6 }
}

impl AfI2s3Ck for Pc10 {
   fn af_i2s3_ck(&self) -> usize { 6 }
}

impl AfUsart3Tx for Pc10 {
   fn af_usart3_tx(&self) -> usize { 7 }
}

pub const PC11: Pc11 = Pc11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc11 {}

impl Pin for Pc11 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 11 }
}

impl AfEventOut for Pc11 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim8Ch2n for Pc11 {
   fn af_tim8_ch2n(&self) -> usize { 3 }
}

impl AfUart4Rx for Pc11 {
   fn af_uart4_rx(&self) -> usize { 5 }
}

impl AfSpi3Miso for Pc11 {
   fn af_spi3_miso(&self) -> usize { 6 }
}

impl AfI2s3extSd for Pc11 {
   fn af_i2s3ext_sd(&self) -> usize { 6 }
}

impl AfUsart3Rx for Pc11 {
   fn af_usart3_rx(&self) -> usize { 7 }
}

pub const PC12: Pc12 = Pc12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc12 {}

impl Pin for Pc12 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 12 }
}

impl AfEventOut for Pc12 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim8Ch3n for Pc12 {
   fn af_tim8_ch3n(&self) -> usize { 3 }
}

impl AfUart5Tx for Pc12 {
   fn af_uart5_tx(&self) -> usize { 5 }
}

impl AfSpi3Mosi for Pc12 {
   fn af_spi3_mosi(&self) -> usize { 6 }
}

impl AfI2s3Sd for Pc12 {
   fn af_i2s3_sd(&self) -> usize { 6 }
}

impl AfUsart3Ck for Pc12 {
   fn af_usart3_ck(&self) -> usize { 7 }
}

pub const PC13: Pc13 = Pc13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc13 {}

impl Pin for Pc13 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 13 }
}

impl AfEventOut for Pc13 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim8Ch1n for Pc13 {
   fn af_tim8_ch1n(&self) -> usize { 3 }
}

pub const PC14: Pc14 = Pc14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc14 {}

impl Pin for Pc14 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 14 }
}

impl AfEventOut for Pc14 {
   fn af_event_out(&self) -> usize { 1 }
}

pub const PC15: Pc15 = Pc15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc15 {}

impl Pin for Pc15 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 15 }
}

impl AfEventOut for Pc15 {
   fn af_event_out(&self) -> usize { 1 }
}

pub const PD0: Pd0 = Pd0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd0 {}

impl Pin for Pd0 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 0 }
}

impl AfEventOut for Pd0 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfCanRx for Pd0 {
   fn af_can_rx(&self) -> usize { 7 }
}

impl AfFmcD2 for Pd0 {
   fn af_fmc_d2(&self) -> usize { 12 }
}

pub const PD1: Pd1 = Pd1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd1 {}

impl Pin for Pd1 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 1 }
}

impl AfEventOut for Pd1 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim8Ch4 for Pd1 {
   fn af_tim8_ch4(&self) -> usize { 3 }
}

impl AfTim8Bkin2 for Pd1 {
   fn af_tim8_bkin2(&self) -> usize { 6 }
}

impl AfCanTx for Pd1 {
   fn af_can_tx(&self) -> usize { 7 }
}

impl AfFmcD3 for Pd1 {
   fn af_fmc_d3(&self) -> usize { 12 }
}

pub const PD2: Pd2 = Pd2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd2 {}

impl Pin for Pd2 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 2 }
}

impl AfEventOut for Pd2 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim3Etr for Pd2 {
   fn af_tim3_etr(&self) -> usize { 2 }
}

impl AfTim8Bkin for Pd2 {
   fn af_tim8_bkin(&self) -> usize { 3 }
}

impl AfUart5Rx for Pd2 {
   fn af_uart5_rx(&self) -> usize { 5 }
}

pub const PD3: Pd3 = Pd3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd3 {}

impl Pin for Pd3 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 3 }
}

impl AfEventOut for Pd3 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim2Ch1 for Pd3 {
   fn af_tim2_ch1(&self) -> usize { 2 }
}

impl AfTim2Etr for Pd3 {
   fn af_tim2_etr(&self) -> usize { 2 }
}

impl AfUsart2Cts for Pd3 {
   fn af_usart2_cts(&self) -> usize { 7 }
}

impl AfFmcClk for Pd3 {
   fn af_fmc_clk(&self) -> usize { 12 }
}

pub const PD4: Pd4 = Pd4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd4 {}

impl Pin for Pd4 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 4 }
}

impl AfEventOut for Pd4 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim2Ch2 for Pd4 {
   fn af_tim2_ch2(&self) -> usize { 2 }
}

impl AfUsart2Rts for Pd4 {
   fn af_usart2_rts(&self) -> usize { 7 }
}

impl AfFmcNoe for Pd4 {
   fn af_fmc_noe(&self) -> usize { 12 }
}

pub const PD5: Pd5 = Pd5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd5 {}

impl Pin for Pd5 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 5 }
}

impl AfEventOut for Pd5 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfUsart2Tx for Pd5 {
   fn af_usart2_tx(&self) -> usize { 7 }
}

impl AfFmcNwe for Pd5 {
   fn af_fmc_nwe(&self) -> usize { 12 }
}

pub const PD6: Pd6 = Pd6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd6 {}

impl Pin for Pd6 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 6 }
}

impl AfEventOut for Pd6 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim2Ch4 for Pd6 {
   fn af_tim2_ch4(&self) -> usize { 2 }
}

impl AfUsart2Rx for Pd6 {
   fn af_usart2_rx(&self) -> usize { 7 }
}

impl AfFmcNwait for Pd6 {
   fn af_fmc_nwait(&self) -> usize { 12 }
}

pub const PD7: Pd7 = Pd7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd7 {}

impl Pin for Pd7 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 7 }
}

impl AfEventOut for Pd7 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim2Ch3 for Pd7 {
   fn af_tim2_ch3(&self) -> usize { 2 }
}

impl AfUsart2Ck for Pd7 {
   fn af_usart2_ck(&self) -> usize { 7 }
}

impl AfFmcNe1 for Pd7 {
   fn af_fmc_ne1(&self) -> usize { 12 }
}

impl AfFmcNce2 for Pd7 {
   fn af_fmc_nce2(&self) -> usize { 12 }
}

pub const PD8: Pd8 = Pd8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd8 {}

impl Pin for Pd8 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 8 }
}

impl AfEventOut for Pd8 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfUsart3Tx for Pd8 {
   fn af_usart3_tx(&self) -> usize { 7 }
}

impl AfFmcD13 for Pd8 {
   fn af_fmc_d13(&self) -> usize { 12 }
}

pub const PD9: Pd9 = Pd9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd9 {}

impl Pin for Pd9 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 9 }
}

impl AfEventOut for Pd9 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfUsart3Rx for Pd9 {
   fn af_usart3_rx(&self) -> usize { 7 }
}

impl AfFmcD14 for Pd9 {
   fn af_fmc_d14(&self) -> usize { 12 }
}

pub const PD10: Pd10 = Pd10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd10 {}

impl Pin for Pd10 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 10 }
}

impl AfEventOut for Pd10 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfUsart3Ck for Pd10 {
   fn af_usart3_ck(&self) -> usize { 7 }
}

impl AfFmcD15 for Pd10 {
   fn af_fmc_d15(&self) -> usize { 12 }
}

pub const PD11: Pd11 = Pd11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd11 {}

impl Pin for Pd11 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 11 }
}

impl AfEventOut for Pd11 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfUsart3Cts for Pd11 {
   fn af_usart3_cts(&self) -> usize { 7 }
}

impl AfFmcA16 for Pd11 {
   fn af_fmc_a16(&self) -> usize { 12 }
}

pub const PD12: Pd12 = Pd12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd12 {}

impl Pin for Pd12 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 12 }
}

impl AfEventOut for Pd12 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim4Ch1 for Pd12 {
   fn af_tim4_ch1(&self) -> usize { 2 }
}

impl AfTscG8Io1 for Pd12 {
   fn af_tsc_g8_io1(&self) -> usize { 3 }
}

impl AfUsart3Rts for Pd12 {
   fn af_usart3_rts(&self) -> usize { 7 }
}

impl AfFmcA17 for Pd12 {
   fn af_fmc_a17(&self) -> usize { 12 }
}

pub const PD13: Pd13 = Pd13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd13 {}

impl Pin for Pd13 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 13 }
}

impl AfEventOut for Pd13 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim4Ch2 for Pd13 {
   fn af_tim4_ch2(&self) -> usize { 2 }
}

impl AfTscG8Io2 for Pd13 {
   fn af_tsc_g8_io2(&self) -> usize { 3 }
}

impl AfFmcA18 for Pd13 {
   fn af_fmc_a18(&self) -> usize { 12 }
}

pub const PD14: Pd14 = Pd14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd14 {}

impl Pin for Pd14 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 14 }
}

impl AfEventOut for Pd14 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim4Ch3 for Pd14 {
   fn af_tim4_ch3(&self) -> usize { 2 }
}

impl AfTscG8Io3 for Pd14 {
   fn af_tsc_g8_io3(&self) -> usize { 3 }
}

impl AfFmcD0 for Pd14 {
   fn af_fmc_d0(&self) -> usize { 12 }
}

pub const PD15: Pd15 = Pd15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd15 {}

impl Pin for Pd15 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 15 }
}

impl AfEventOut for Pd15 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim4Ch4 for Pd15 {
   fn af_tim4_ch4(&self) -> usize { 2 }
}

impl AfTscG8Io4 for Pd15 {
   fn af_tsc_g8_io4(&self) -> usize { 3 }
}

impl AfSpi2Nss for Pd15 {
   fn af_spi2_nss(&self) -> usize { 6 }
}

impl AfFmcD1 for Pd15 {
   fn af_fmc_d1(&self) -> usize { 12 }
}

pub const PE0: Pe0 = Pe0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe0 {}

impl Pin for Pe0 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 0 }
}

impl AfEventOut for Pe0 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim4Etr for Pe0 {
   fn af_tim4_etr(&self) -> usize { 2 }
}

impl AfTim16Ch1 for Pe0 {
   fn af_tim16_ch1(&self) -> usize { 4 }
}

impl AfTim20Etr for Pe0 {
   fn af_tim20_etr(&self) -> usize { 6 }
}

impl AfUsart1Tx for Pe0 {
   fn af_usart1_tx(&self) -> usize { 7 }
}

impl AfFmcNbl0 for Pe0 {
   fn af_fmc_nbl0(&self) -> usize { 12 }
}

pub const PE1: Pe1 = Pe1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe1 {}

impl Pin for Pe1 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 1 }
}

impl AfEventOut for Pe1 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim17Ch1 for Pe1 {
   fn af_tim17_ch1(&self) -> usize { 4 }
}

impl AfTim20Ch4 for Pe1 {
   fn af_tim20_ch4(&self) -> usize { 6 }
}

impl AfUsart1Rx for Pe1 {
   fn af_usart1_rx(&self) -> usize { 7 }
}

impl AfFmcNbl1 for Pe1 {
   fn af_fmc_nbl1(&self) -> usize { 12 }
}

pub const PE2: Pe2 = Pe2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe2 {}

impl Pin for Pe2 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 2 }
}

impl AfTraceck for Pe2 {
   fn af_traceck(&self) -> usize { 0 }
}

impl AfEventOut for Pe2 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim3Ch1 for Pe2 {
   fn af_tim3_ch1(&self) -> usize { 2 }
}

impl AfTscG7Io1 for Pe2 {
   fn af_tsc_g7_io1(&self) -> usize { 3 }
}

impl AfSpi4Sck for Pe2 {
   fn af_spi4_sck(&self) -> usize { 5 }
}

impl AfTim20Ch1 for Pe2 {
   fn af_tim20_ch1(&self) -> usize { 6 }
}

impl AfFmcA23 for Pe2 {
   fn af_fmc_a23(&self) -> usize { 12 }
}

pub const PE3: Pe3 = Pe3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe3 {}

impl Pin for Pe3 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 3 }
}

impl AfTraced0 for Pe3 {
   fn af_traced0(&self) -> usize { 0 }
}

impl AfEventOut for Pe3 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim3Ch2 for Pe3 {
   fn af_tim3_ch2(&self) -> usize { 2 }
}

impl AfTscG7Io2 for Pe3 {
   fn af_tsc_g7_io2(&self) -> usize { 3 }
}

impl AfSpi4Nss for Pe3 {
   fn af_spi4_nss(&self) -> usize { 5 }
}

impl AfTim20Ch2 for Pe3 {
   fn af_tim20_ch2(&self) -> usize { 6 }
}

impl AfFmcA19 for Pe3 {
   fn af_fmc_a19(&self) -> usize { 12 }
}

pub const PE4: Pe4 = Pe4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe4 {}

impl Pin for Pe4 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 4 }
}

impl AfTraced1 for Pe4 {
   fn af_traced1(&self) -> usize { 0 }
}

impl AfEventOut for Pe4 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim3Ch3 for Pe4 {
   fn af_tim3_ch3(&self) -> usize { 2 }
}

impl AfTscG7Io3 for Pe4 {
   fn af_tsc_g7_io3(&self) -> usize { 3 }
}

impl AfSpi4Nss for Pe4 {
   fn af_spi4_nss(&self) -> usize { 5 }
}

impl AfTim20Ch1n for Pe4 {
   fn af_tim20_ch1n(&self) -> usize { 6 }
}

impl AfFmcA20 for Pe4 {
   fn af_fmc_a20(&self) -> usize { 12 }
}

pub const PE5: Pe5 = Pe5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe5 {}

impl Pin for Pe5 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 5 }
}

impl AfTraced2 for Pe5 {
   fn af_traced2(&self) -> usize { 0 }
}

impl AfEventOut for Pe5 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim3Ch4 for Pe5 {
   fn af_tim3_ch4(&self) -> usize { 2 }
}

impl AfTscG7Io4 for Pe5 {
   fn af_tsc_g7_io4(&self) -> usize { 3 }
}

impl AfSpi4Miso for Pe5 {
   fn af_spi4_miso(&self) -> usize { 5 }
}

impl AfTim20Ch2n for Pe5 {
   fn af_tim20_ch2n(&self) -> usize { 6 }
}

impl AfFmcA21 for Pe5 {
   fn af_fmc_a21(&self) -> usize { 12 }
}

pub const PE6: Pe6 = Pe6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe6 {}

impl Pin for Pe6 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 6 }
}

impl AfTraced3 for Pe6 {
   fn af_traced3(&self) -> usize { 0 }
}

impl AfEventOut for Pe6 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfSpi4Mosi for Pe6 {
   fn af_spi4_mosi(&self) -> usize { 5 }
}

impl AfTim20Ch3n for Pe6 {
   fn af_tim20_ch3n(&self) -> usize { 6 }
}

impl AfFmcA22 for Pe6 {
   fn af_fmc_a22(&self) -> usize { 12 }
}

pub const PE7: Pe7 = Pe7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe7 {}

impl Pin for Pe7 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 7 }
}

impl AfEventOut for Pe7 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim1Etr for Pe7 {
   fn af_tim1_etr(&self) -> usize { 2 }
}

impl AfFmcD4 for Pe7 {
   fn af_fmc_d4(&self) -> usize { 12 }
}

pub const PE8: Pe8 = Pe8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe8 {}

impl Pin for Pe8 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 8 }
}

impl AfEventOut for Pe8 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim1Ch1n for Pe8 {
   fn af_tim1_ch1n(&self) -> usize { 2 }
}

impl AfFmcD5 for Pe8 {
   fn af_fmc_d5(&self) -> usize { 12 }
}

pub const PE9: Pe9 = Pe9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe9 {}

impl Pin for Pe9 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 9 }
}

impl AfEventOut for Pe9 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim1Ch1 for Pe9 {
   fn af_tim1_ch1(&self) -> usize { 2 }
}

impl AfFmcD6 for Pe9 {
   fn af_fmc_d6(&self) -> usize { 12 }
}

pub const PE10: Pe10 = Pe10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe10 {}

impl Pin for Pe10 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 10 }
}

impl AfEventOut for Pe10 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim1Ch2n for Pe10 {
   fn af_tim1_ch2n(&self) -> usize { 2 }
}

impl AfFmcD7 for Pe10 {
   fn af_fmc_d7(&self) -> usize { 12 }
}

pub const PE11: Pe11 = Pe11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe11 {}

impl Pin for Pe11 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 11 }
}

impl AfEventOut for Pe11 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim1Ch2 for Pe11 {
   fn af_tim1_ch2(&self) -> usize { 2 }
}

impl AfSpi4Nss for Pe11 {
   fn af_spi4_nss(&self) -> usize { 5 }
}

impl AfFmcD8 for Pe11 {
   fn af_fmc_d8(&self) -> usize { 12 }
}

pub const PE12: Pe12 = Pe12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe12 {}

impl Pin for Pe12 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 12 }
}

impl AfEventOut for Pe12 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim1Ch3n for Pe12 {
   fn af_tim1_ch3n(&self) -> usize { 2 }
}

impl AfSpi4Sck for Pe12 {
   fn af_spi4_sck(&self) -> usize { 5 }
}

impl AfFmcD9 for Pe12 {
   fn af_fmc_d9(&self) -> usize { 12 }
}

pub const PE13: Pe13 = Pe13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe13 {}

impl Pin for Pe13 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 13 }
}

impl AfEventOut for Pe13 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim1Ch3 for Pe13 {
   fn af_tim1_ch3(&self) -> usize { 2 }
}

impl AfSpi4Miso for Pe13 {
   fn af_spi4_miso(&self) -> usize { 5 }
}

impl AfFmcD10 for Pe13 {
   fn af_fmc_d10(&self) -> usize { 12 }
}

pub const PE14: Pe14 = Pe14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe14 {}

impl Pin for Pe14 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 14 }
}

impl AfEventOut for Pe14 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim1Ch4 for Pe14 {
   fn af_tim1_ch4(&self) -> usize { 2 }
}

impl AfSpi4Mosi for Pe14 {
   fn af_spi4_mosi(&self) -> usize { 5 }
}

impl AfTim1Bkin2 for Pe14 {
   fn af_tim1_bkin2(&self) -> usize { 6 }
}

impl AfFmcD11 for Pe14 {
   fn af_fmc_d11(&self) -> usize { 12 }
}

pub const PE15: Pe15 = Pe15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe15 {}

impl Pin for Pe15 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 15 }
}

impl AfEventOut for Pe15 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim1Bkin for Pe15 {
   fn af_tim1_bkin(&self) -> usize { 2 }
}

impl AfUsart3Rx for Pe15 {
   fn af_usart3_rx(&self) -> usize { 6 }
}

impl AfFmcD12 for Pe15 {
   fn af_fmc_d12(&self) -> usize { 12 }
}

pub const PF0: Pf0 = Pf0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf0 {}

impl Pin for Pf0 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 0 }
}

impl AfEventOut for Pf0 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfI2c2Sda for Pf0 {
   fn af_i2c2_sda(&self) -> usize { 4 }
}

impl AfSpi2Nss for Pf0 {
   fn af_spi2_nss(&self) -> usize { 5 }
}

impl AfI2s2Ws for Pf0 {
   fn af_i2s2_ws(&self) -> usize { 5 }
}

impl AfTim1Ch3n for Pf0 {
   fn af_tim1_ch3n(&self) -> usize { 6 }
}

pub const PF1: Pf1 = Pf1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf1 {}

impl Pin for Pf1 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 1 }
}

impl AfEventOut for Pf1 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfI2c2Scl for Pf1 {
   fn af_i2c2_scl(&self) -> usize { 4 }
}

impl AfSpi2Sck for Pf1 {
   fn af_spi2_sck(&self) -> usize { 5 }
}

impl AfI2s2Ck for Pf1 {
   fn af_i2s2_ck(&self) -> usize { 5 }
}

pub const PF2: Pf2 = Pf2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf2 {}

impl Pin for Pf2 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 2 }
}

impl AfEventOut for Pf2 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim20Ch3 for Pf2 {
   fn af_tim20_ch3(&self) -> usize { 2 }
}

impl AfFmcA2 for Pf2 {
   fn af_fmc_a2(&self) -> usize { 12 }
}

pub const PF3: Pf3 = Pf3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf3 {}

impl Pin for Pf3 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 3 }
}

impl AfEventOut for Pf3 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim20Ch4 for Pf3 {
   fn af_tim20_ch4(&self) -> usize { 2 }
}

impl AfFmcA3 for Pf3 {
   fn af_fmc_a3(&self) -> usize { 12 }
}

pub const PF4: Pf4 = Pf4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf4 {}

impl Pin for Pf4 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 4 }
}

impl AfEventOut for Pf4 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfComp1Out for Pf4 {
   fn af_comp1_out(&self) -> usize { 2 }
}

impl AfTim20Ch1n for Pf4 {
   fn af_tim20_ch1n(&self) -> usize { 3 }
}

impl AfFmcA4 for Pf4 {
   fn af_fmc_a4(&self) -> usize { 12 }
}

pub const PF5: Pf5 = Pf5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf5 {}

impl Pin for Pf5 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 5 }
}

impl AfEventOut for Pf5 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim20Ch2n for Pf5 {
   fn af_tim20_ch2n(&self) -> usize { 2 }
}

impl AfFmcA5 for Pf5 {
   fn af_fmc_a5(&self) -> usize { 12 }
}

pub const PF6: Pf6 = Pf6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf6 {}

impl Pin for Pf6 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 6 }
}

impl AfEventOut for Pf6 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim4Ch4 for Pf6 {
   fn af_tim4_ch4(&self) -> usize { 2 }
}

impl AfI2c2Scl for Pf6 {
   fn af_i2c2_scl(&self) -> usize { 4 }
}

impl AfUsart3Rts for Pf6 {
   fn af_usart3_rts(&self) -> usize { 6 }
}

impl AfFmcNiord for Pf6 {
   fn af_fmc_niord(&self) -> usize { 12 }
}

pub const PF7: Pf7 = Pf7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf7 {}

impl Pin for Pf7 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 7 }
}

impl AfEventOut for Pf7 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim20Bkin for Pf7 {
   fn af_tim20_bkin(&self) -> usize { 2 }
}

impl AfFmcNreg for Pf7 {
   fn af_fmc_nreg(&self) -> usize { 12 }
}

pub const PF8: Pf8 = Pf8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf8 {}

impl Pin for Pf8 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 8 }
}

impl AfEventOut for Pf8 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim20Bkin2 for Pf8 {
   fn af_tim20_bkin2(&self) -> usize { 2 }
}

impl AfFmcNiowr for Pf8 {
   fn af_fmc_niowr(&self) -> usize { 12 }
}

pub const PF9: Pf9 = Pf9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf9 {}

impl Pin for Pf9 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 9 }
}

impl AfEventOut for Pf9 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim20Bkin for Pf9 {
   fn af_tim20_bkin(&self) -> usize { 2 }
}

impl AfTim15Ch1 for Pf9 {
   fn af_tim15_ch1(&self) -> usize { 3 }
}

impl AfSpi2Sck for Pf9 {
   fn af_spi2_sck(&self) -> usize { 5 }
}

impl AfFmcCd for Pf9 {
   fn af_fmc_cd(&self) -> usize { 12 }
}

pub const PF10: Pf10 = Pf10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf10 {}

impl Pin for Pf10 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 10 }
}

impl AfEventOut for Pf10 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim20Bkin2 for Pf10 {
   fn af_tim20_bkin2(&self) -> usize { 2 }
}

impl AfTim15Ch2 for Pf10 {
   fn af_tim15_ch2(&self) -> usize { 3 }
}

impl AfSpi2Sck for Pf10 {
   fn af_spi2_sck(&self) -> usize { 5 }
}

impl AfFmcIntr for Pf10 {
   fn af_fmc_intr(&self) -> usize { 12 }
}

pub const PF11: Pf11 = Pf11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf11 {}

impl Pin for Pf11 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 11 }
}

impl AfEventOut for Pf11 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim20Etr for Pf11 {
   fn af_tim20_etr(&self) -> usize { 2 }
}

pub const PF12: Pf12 = Pf12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf12 {}

impl Pin for Pf12 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 12 }
}

impl AfEventOut for Pf12 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim20Ch1 for Pf12 {
   fn af_tim20_ch1(&self) -> usize { 2 }
}

impl AfFmcA6 for Pf12 {
   fn af_fmc_a6(&self) -> usize { 12 }
}

pub const PF13: Pf13 = Pf13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf13 {}

impl Pin for Pf13 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 13 }
}

impl AfEventOut for Pf13 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim20Ch2 for Pf13 {
   fn af_tim20_ch2(&self) -> usize { 2 }
}

impl AfFmcA7 for Pf13 {
   fn af_fmc_a7(&self) -> usize { 12 }
}

pub const PF14: Pf14 = Pf14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf14 {}

impl Pin for Pf14 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 14 }
}

impl AfEventOut for Pf14 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim20Ch3 for Pf14 {
   fn af_tim20_ch3(&self) -> usize { 2 }
}

impl AfFmcA8 for Pf14 {
   fn af_fmc_a8(&self) -> usize { 12 }
}

pub const PF15: Pf15 = Pf15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf15 {}

impl Pin for Pf15 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 15 }
}

impl AfEventOut for Pf15 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim20Ch4 for Pf15 {
   fn af_tim20_ch4(&self) -> usize { 2 }
}

impl AfFmcA9 for Pf15 {
   fn af_fmc_a9(&self) -> usize { 12 }
}

pub const PG0: Pg0 = Pg0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg0 {}

impl Pin for Pg0 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 0 }
}

impl AfEventOut for Pg0 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim20Ch1n for Pg0 {
   fn af_tim20_ch1n(&self) -> usize { 2 }
}

impl AfFmcA10 for Pg0 {
   fn af_fmc_a10(&self) -> usize { 12 }
}

pub const PG1: Pg1 = Pg1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg1 {}

impl Pin for Pg1 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 1 }
}

impl AfEventOut for Pg1 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim20Ch2n for Pg1 {
   fn af_tim20_ch2n(&self) -> usize { 2 }
}

impl AfFmcA11 for Pg1 {
   fn af_fmc_a11(&self) -> usize { 12 }
}

pub const PG2: Pg2 = Pg2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg2 {}

impl Pin for Pg2 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 2 }
}

impl AfEventOut for Pg2 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim20Ch3n for Pg2 {
   fn af_tim20_ch3n(&self) -> usize { 2 }
}

impl AfFmcA12 for Pg2 {
   fn af_fmc_a12(&self) -> usize { 12 }
}

pub const PG3: Pg3 = Pg3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg3 {}

impl Pin for Pg3 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 3 }
}

impl AfEventOut for Pg3 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim20Bkin for Pg3 {
   fn af_tim20_bkin(&self) -> usize { 2 }
}

impl AfFmcA13 for Pg3 {
   fn af_fmc_a13(&self) -> usize { 12 }
}

pub const PG4: Pg4 = Pg4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg4 {}

impl Pin for Pg4 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 4 }
}

impl AfEventOut for Pg4 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim20Bkin2 for Pg4 {
   fn af_tim20_bkin2(&self) -> usize { 2 }
}

impl AfFmcA14 for Pg4 {
   fn af_fmc_a14(&self) -> usize { 12 }
}

pub const PG5: Pg5 = Pg5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg5 {}

impl Pin for Pg5 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 5 }
}

impl AfEventOut for Pg5 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim20Etr for Pg5 {
   fn af_tim20_etr(&self) -> usize { 2 }
}

impl AfFmcA15 for Pg5 {
   fn af_fmc_a15(&self) -> usize { 12 }
}

pub const PG6: Pg6 = Pg6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg6 {}

impl Pin for Pg6 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 6 }
}

impl AfEventOut for Pg6 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfFmcInt2 for Pg6 {
   fn af_fmc_int2(&self) -> usize { 12 }
}

pub const PG7: Pg7 = Pg7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg7 {}

impl Pin for Pg7 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 7 }
}

impl AfEventOut for Pg7 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfFmcInt3 for Pg7 {
   fn af_fmc_int3(&self) -> usize { 12 }
}

pub const PG8: Pg8 = Pg8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg8 {}

impl Pin for Pg8 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 8 }
}

impl AfEventOut for Pg8 {
   fn af_event_out(&self) -> usize { 1 }
}

pub const PG9: Pg9 = Pg9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg9 {}

impl Pin for Pg9 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 9 }
}

impl AfEventOut for Pg9 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfFmcNe2 for Pg9 {
   fn af_fmc_ne2(&self) -> usize { 12 }
}

impl AfFmcNce3 for Pg9 {
   fn af_fmc_nce3(&self) -> usize { 12 }
}

pub const PG10: Pg10 = Pg10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg10 {}

impl Pin for Pg10 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 10 }
}

impl AfEventOut for Pg10 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfFmcNce41 for Pg10 {
   fn af_fmc_nce4_1(&self) -> usize { 12 }
}

impl AfFmcNe3 for Pg10 {
   fn af_fmc_ne3(&self) -> usize { 12 }
}

pub const PG11: Pg11 = Pg11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg11 {}

impl Pin for Pg11 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 11 }
}

impl AfEventOut for Pg11 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfFmc42 for Pg11 {
   fn af_fmc_4_2(&self) -> usize { 12 }
}

pub const PG12: Pg12 = Pg12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg12 {}

impl Pin for Pg12 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 12 }
}

impl AfEventOut for Pg12 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfFmcNe4 for Pg12 {
   fn af_fmc_ne4(&self) -> usize { 12 }
}

pub const PG13: Pg13 = Pg13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg13 {}

impl Pin for Pg13 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 13 }
}

impl AfEventOut for Pg13 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfFmcA24 for Pg13 {
   fn af_fmc_a24(&self) -> usize { 12 }
}

pub const PG14: Pg14 = Pg14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg14 {}

impl Pin for Pg14 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 14 }
}

impl AfEventOut for Pg14 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfFmcA25 for Pg14 {
   fn af_fmc_a25(&self) -> usize { 12 }
}

pub const PG15: Pg15 = Pg15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg15 {}

impl Pin for Pg15 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 15 }
}

impl AfEventOut for Pg15 {
   fn af_event_out(&self) -> usize { 1 }
}

pub const PH0: Ph0 = Ph0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ph0 {}

impl Pin for Ph0 {
   fn port(&self) -> Gpio { GPIOH }
   fn index(&self) -> usize { 0 }
}

impl AfEventOut for Ph0 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim20Ch1 for Ph0 {
   fn af_tim20_ch1(&self) -> usize { 2 }
}

impl AfFmcA0 for Ph0 {
   fn af_fmc_a0(&self) -> usize { 12 }
}

pub const PH1: Ph1 = Ph1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ph1 {}

impl Pin for Ph1 {
   fn port(&self) -> Gpio { GPIOH }
   fn index(&self) -> usize { 1 }
}

impl AfEventOut for Ph1 {
   fn af_event_out(&self) -> usize { 1 }
}

impl AfTim20Ch2 for Ph1 {
   fn af_tim20_ch2(&self) -> usize { 2 }
}

impl AfFmcA1 for Ph1 {
   fn af_fmc_a1(&self) -> usize { 12 }
}

pub const PH2: Ph2 = Ph2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ph2 {}

impl Pin for Ph2 {
   fn port(&self) -> Gpio { GPIOH }
   fn index(&self) -> usize { 2 }
}

impl AfEventOut for Ph2 {
   fn af_event_out(&self) -> usize { 1 }
}

