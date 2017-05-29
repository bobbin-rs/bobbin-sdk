pub use stm32_common::chip::gpio::*;

pub const GPIOA: Gpio = Gpio(0x48000000);
pub const GPIOB: Gpio = Gpio(0x48000400);
pub const GPIOC: Gpio = Gpio(0x48000800);
pub const GPIOD: Gpio = Gpio(0x48000c00);
pub const GPIOF: Gpio = Gpio(0x48001400);

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

pub trait AfEventout {
   fn af_eventout(&self) -> usize;
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

pub trait AfUsart2RtsDe {
   fn af_usart2_rts_de(&self) -> usize;
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

pub trait AfTim2Ch4 {
   fn af_tim2_ch4(&self) -> usize;
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

pub trait AfTscG2Io1 {
   fn af_tsc_g2_io1(&self) -> usize;
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

pub trait AfTim16Ch1 {
   fn af_tim16_ch1(&self) -> usize;
}

pub trait AfTscG2Io3 {
   fn af_tsc_g2_io3(&self) -> usize;
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

pub trait AfUsart1RtsDe {
   fn af_usart1_rts_de(&self) -> usize;
}

pub trait AfCanTx {
   fn af_can_tx(&self) -> usize;
}

pub trait AfTim1Etr {
   fn af_tim1_etr(&self) -> usize;
}

pub trait AfSwdat {
   fn af_swdat(&self) -> usize;
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

pub trait AfJtdi {
   fn af_jtdi(&self) -> usize;
}

pub trait AfTscSync {
   fn af_tsc_sync(&self) -> usize;
}

pub trait AfI2c1Scl {
   fn af_i2c1_scl(&self) -> usize;
}

pub trait AfTscG3Io2 {
   fn af_tsc_g3_io2(&self) -> usize;
}

pub trait AfTscG3Io3 {
   fn af_tsc_g3_io3(&self) -> usize;
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

pub trait AfJtrst {
   fn af_jtrst(&self) -> usize;
}

pub trait AfTscG5Io2 {
   fn af_tsc_g5_io2(&self) -> usize;
}

pub trait AfSpi3Miso {
   fn af_spi3_miso(&self) -> usize;
}

pub trait AfI2s3Sd {
   fn af_i2s3_sd(&self) -> usize;
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

pub trait AfI2s3extSd {
   fn af_i2s3ext_sd(&self) -> usize;
}

pub trait AfI2c3Sda {
   fn af_i2c3_sda(&self) -> usize;
}

pub trait AfTscG5Io3 {
   fn af_tsc_g5_io3(&self) -> usize;
}

pub trait AfTim17Ch1n {
   fn af_tim17_ch1n(&self) -> usize;
}

pub trait AfTscG5Io4 {
   fn af_tsc_g5_io4(&self) -> usize;
}

pub trait AfUsart3Rx {
   fn af_usart3_rx(&self) -> usize;
}

pub trait AfUsart3Tx {
   fn af_usart3_tx(&self) -> usize;
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

pub trait AfUsart3RtsDe {
   fn af_usart3_rts_de(&self) -> usize;
}

pub trait AfTscG3Io1 {
   fn af_tsc_g3_io1(&self) -> usize;
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

impl AfEventout for Pa0 {
   fn af_eventout(&self) -> usize { 15 }
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

impl AfUsart2RtsDe for Pa1 {
   fn af_usart2_rts_de(&self) -> usize { 7 }
}

impl AfTim15Ch1n for Pa1 {
   fn af_tim15_ch1n(&self) -> usize { 9 }
}

impl AfEventout for Pa1 {
   fn af_eventout(&self) -> usize { 15 }
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

impl AfEventout for Pa2 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PA3: Pa3 = Pa3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa3 {}

impl Pin for Pa3 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 3 }
}

impl AfTim2Ch4 for Pa3 {
   fn af_tim2_ch4(&self) -> usize { 1 }
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

impl AfEventout for Pa3 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PA4: Pa4 = Pa4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa4 {}

impl Pin for Pa4 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 4 }
}

impl AfTscG2Io1 for Pa4 {
   fn af_tsc_g2_io1(&self) -> usize { 3 }
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

impl AfEventout for Pa4 {
   fn af_eventout(&self) -> usize { 15 }
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

impl AfEventout for Pa5 {
   fn af_eventout(&self) -> usize { 15 }
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

impl AfTscG2Io3 for Pa6 {
   fn af_tsc_g2_io3(&self) -> usize { 3 }
}

impl AfTim1Bkin for Pa6 {
   fn af_tim1_bkin(&self) -> usize { 6 }
}

impl AfEventout for Pa6 {
   fn af_eventout(&self) -> usize { 15 }
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

impl AfTscG2Io4 for Pa7 {
   fn af_tsc_g2_io4(&self) -> usize { 3 }
}

impl AfTim1Ch1n for Pa7 {
   fn af_tim1_ch1n(&self) -> usize { 6 }
}

impl AfEventout for Pa7 {
   fn af_eventout(&self) -> usize { 15 }
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

impl AfEventout for Pa8 {
   fn af_eventout(&self) -> usize { 15 }
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

impl AfTim15Bkin for Pa9 {
   fn af_tim15_bkin(&self) -> usize { 9 }
}

impl AfTim2Ch3 for Pa9 {
   fn af_tim2_ch3(&self) -> usize { 10 }
}

impl AfEventout for Pa9 {
   fn af_eventout(&self) -> usize { 15 }
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

impl AfEventout for Pa10 {
   fn af_eventout(&self) -> usize { 15 }
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

impl AfCanRx for Pa11 {
   fn af_can_rx(&self) -> usize { 9 }
}

impl AfTim1Ch4 for Pa11 {
   fn af_tim1_ch4(&self) -> usize { 11 }
}

impl AfTim1Bkin2 for Pa11 {
   fn af_tim1_bkin2(&self) -> usize { 12 }
}

impl AfEventout for Pa11 {
   fn af_eventout(&self) -> usize { 15 }
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

impl AfUsart1RtsDe for Pa12 {
   fn af_usart1_rts_de(&self) -> usize { 7 }
}

impl AfComp2Out for Pa12 {
   fn af_comp2_out(&self) -> usize { 8 }
}

impl AfCanTx for Pa12 {
   fn af_can_tx(&self) -> usize { 9 }
}

impl AfTim1Etr for Pa12 {
   fn af_tim1_etr(&self) -> usize { 11 }
}

impl AfEventout for Pa12 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PA13: Pa13 = Pa13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa13 {}

impl Pin for Pa13 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 13 }
}

impl AfSwdat for Pa13 {
   fn af_swdat(&self) -> usize { 0 }
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

impl AfEventout for Pa13 {
   fn af_eventout(&self) -> usize { 15 }
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

impl AfTim1Bkin for Pa14 {
   fn af_tim1_bkin(&self) -> usize { 6 }
}

impl AfUsart2Tx for Pa14 {
   fn af_usart2_tx(&self) -> usize { 7 }
}

impl AfEventout for Pa14 {
   fn af_eventout(&self) -> usize { 15 }
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

impl AfTscSync for Pa15 {
   fn af_tsc_sync(&self) -> usize { 3 }
}

impl AfI2c1Scl for Pa15 {
   fn af_i2c1_scl(&self) -> usize { 4 }
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

impl AfEventout for Pa15 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PB0: Pb0 = Pb0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb0 {}

impl Pin for Pb0 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 0 }
}

impl AfTscG3Io2 for Pb0 {
   fn af_tsc_g3_io2(&self) -> usize { 3 }
}

impl AfTim1Ch2n for Pb0 {
   fn af_tim1_ch2n(&self) -> usize { 6 }
}

impl AfEventout for Pb0 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PB1: Pb1 = Pb1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb1 {}

impl Pin for Pb1 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 1 }
}

impl AfTscG3Io3 for Pb1 {
   fn af_tsc_g3_io3(&self) -> usize { 3 }
}

impl AfTim1Ch3n for Pb1 {
   fn af_tim1_ch3n(&self) -> usize { 6 }
}

impl AfComp4Out for Pb1 {
   fn af_comp4_out(&self) -> usize { 8 }
}

impl AfEventout for Pb1 {
   fn af_eventout(&self) -> usize { 15 }
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

impl AfEventout for Pb2 {
   fn af_eventout(&self) -> usize { 15 }
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

impl AfTscG5Io1 for Pb3 {
   fn af_tsc_g5_io1(&self) -> usize { 3 }
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

impl AfEventout for Pb3 {
   fn af_eventout(&self) -> usize { 15 }
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

impl AfTscG5Io2 for Pb4 {
   fn af_tsc_g5_io2(&self) -> usize { 3 }
}

impl AfSpi3Miso for Pb4 {
   fn af_spi3_miso(&self) -> usize { 6 }
}

impl AfI2s3Sd for Pb4 {
   fn af_i2s3_sd(&self) -> usize { 6 }
}

impl AfUsart2Rx for Pb4 {
   fn af_usart2_rx(&self) -> usize { 7 }
}

impl AfTim17Bkin for Pb4 {
   fn af_tim17_bkin(&self) -> usize { 10 }
}

impl AfEventout for Pb4 {
   fn af_eventout(&self) -> usize { 15 }
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

impl AfI2c1Smbal for Pb5 {
   fn af_i2c1_smbal(&self) -> usize { 4 }
}

impl AfSpi3Mosi for Pb5 {
   fn af_spi3_mosi(&self) -> usize { 6 }
}

impl AfI2s3extSd for Pb5 {
   fn af_i2s3ext_sd(&self) -> usize { 6 }
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

impl AfEventout for Pb5 {
   fn af_eventout(&self) -> usize { 15 }
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

impl AfTscG5Io3 for Pb6 {
   fn af_tsc_g5_io3(&self) -> usize { 3 }
}

impl AfI2c1Scl for Pb6 {
   fn af_i2c1_scl(&self) -> usize { 4 }
}

impl AfUsart1Tx for Pb6 {
   fn af_usart1_tx(&self) -> usize { 7 }
}

impl AfEventout for Pb6 {
   fn af_eventout(&self) -> usize { 15 }
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

impl AfTscG5Io4 for Pb7 {
   fn af_tsc_g5_io4(&self) -> usize { 3 }
}

impl AfI2c1Sda for Pb7 {
   fn af_i2c1_sda(&self) -> usize { 4 }
}

impl AfUsart1Rx for Pb7 {
   fn af_usart1_rx(&self) -> usize { 7 }
}

impl AfEventout for Pb7 {
   fn af_eventout(&self) -> usize { 15 }
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

impl AfTscSync for Pb8 {
   fn af_tsc_sync(&self) -> usize { 3 }
}

impl AfI2c1Scl for Pb8 {
   fn af_i2c1_scl(&self) -> usize { 4 }
}

impl AfUsart3Rx for Pb8 {
   fn af_usart3_rx(&self) -> usize { 7 }
}

impl AfCanRx for Pb8 {
   fn af_can_rx(&self) -> usize { 9 }
}

impl AfTim1Bkin for Pb8 {
   fn af_tim1_bkin(&self) -> usize { 12 }
}

impl AfEventout for Pb8 {
   fn af_eventout(&self) -> usize { 15 }
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

impl AfEventout for Pb9 {
   fn af_eventout(&self) -> usize { 15 }
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

impl AfEventout for Pb10 {
   fn af_eventout(&self) -> usize { 15 }
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

impl AfEventout for Pb11 {
   fn af_eventout(&self) -> usize { 15 }
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

impl AfI2c2Smbal for Pb12 {
   fn af_i2c2_smbal(&self) -> usize { 4 }
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

impl AfEventout for Pb12 {
   fn af_eventout(&self) -> usize { 15 }
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

impl AfEventout for Pb13 {
   fn af_eventout(&self) -> usize { 15 }
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

impl AfUsart3RtsDe for Pb14 {
   fn af_usart3_rts_de(&self) -> usize { 7 }
}

impl AfEventout for Pb14 {
   fn af_eventout(&self) -> usize { 15 }
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
   fn af_tim1_ch3n(&self) -> usize { 4 }
}

impl AfSpi2Mosi for Pb15 {
   fn af_spi2_mosi(&self) -> usize { 5 }
}

impl AfI2s2Sd for Pb15 {
   fn af_i2s2_sd(&self) -> usize { 5 }
}

impl AfEventout for Pb15 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PC0: Pc0 = Pc0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc0 {}

impl Pin for Pc0 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 0 }
}

impl AfEventout for Pc0 {
   fn af_eventout(&self) -> usize { 1 }
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

impl AfEventout for Pc1 {
   fn af_eventout(&self) -> usize { 1 }
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

impl AfEventout for Pc2 {
   fn af_eventout(&self) -> usize { 1 }
}

impl AfTim1Ch3 for Pc2 {
   fn af_tim1_ch3(&self) -> usize { 2 }
}

pub const PC3: Pc3 = Pc3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc3 {}

impl Pin for Pc3 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 3 }
}

impl AfEventout for Pc3 {
   fn af_eventout(&self) -> usize { 1 }
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

impl AfEventout for Pc4 {
   fn af_eventout(&self) -> usize { 1 }
}

impl AfTim1Etr for Pc4 {
   fn af_tim1_etr(&self) -> usize { 2 }
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

impl AfEventout for Pc5 {
   fn af_eventout(&self) -> usize { 1 }
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

impl AfEventout for Pc6 {
   fn af_eventout(&self) -> usize { 1 }
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

impl AfEventout for Pc7 {
   fn af_eventout(&self) -> usize { 1 }
}

impl AfI2s3Mck for Pc7 {
   fn af_i2s3_mck(&self) -> usize { 6 }
}

pub const PC8: Pc8 = Pc8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc8 {}

impl Pin for Pc8 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 8 }
}

impl AfEventout for Pc8 {
   fn af_eventout(&self) -> usize { 1 }
}

pub const PC9: Pc9 = Pc9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc9 {}

impl Pin for Pc9 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 9 }
}

impl AfEventout for Pc9 {
   fn af_eventout(&self) -> usize { 1 }
}

impl AfI2c3Sda for Pc9 {
   fn af_i2c3_sda(&self) -> usize { 3 }
}

impl AfI2sckin for Pc9 {
   fn af_i2sckin(&self) -> usize { 5 }
}

pub const PC10: Pc10 = Pc10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc10 {}

impl Pin for Pc10 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 10 }
}

impl AfEventout for Pc10 {
   fn af_eventout(&self) -> usize { 1 }
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

impl AfEventout for Pc11 {
   fn af_eventout(&self) -> usize { 1 }
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

impl AfEventout for Pc12 {
   fn af_eventout(&self) -> usize { 1 }
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

impl AfTim1Ch1n for Pc13 {
   fn af_tim1_ch1n(&self) -> usize { 4 }
}

pub const PC14: Pc14 = Pc14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc14 {}

impl Pin for Pc14 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 14 }
}

pub const PC15: Pc15 = Pc15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc15 {}

impl Pin for Pc15 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 15 }
}

pub const PD2: Pd2 = Pd2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd2 {}

impl Pin for Pd2 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 2 }
}

impl AfEventout for Pd2 {
   fn af_eventout(&self) -> usize { 1 }
}

pub const PF0: Pf0 = Pf0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf0 {}

impl Pin for Pf0 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 0 }
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

impl AfI2c2Scl for Pf1 {
   fn af_i2c2_scl(&self) -> usize { 4 }
}

impl AfSpi2Sck for Pf1 {
   fn af_spi2_sck(&self) -> usize { 5 }
}

impl AfI2s2Ck for Pf1 {
   fn af_i2s2_ck(&self) -> usize { 5 }
}

