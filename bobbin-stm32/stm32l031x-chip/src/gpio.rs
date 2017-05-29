pub use stm32_common::chip::gpio::*;

pub const GPIOA: Gpio = Gpio(0x50000000);
pub const GPIOB: Gpio = Gpio(0x50000400);
pub const GPIOC: Gpio = Gpio(0x50000800);
pub const GPIOH: Gpio = Gpio(0x50001c00);

pub trait Pin {
   fn port(&self) -> Gpio;
   fn index(&self) -> usize;
}

pub trait AfLptim1In1 {
   fn af_lptim1_in1(&self) -> usize;
}

pub trait AfTim2Ch1 {
   fn af_tim2_ch1(&self) -> usize;
}

pub trait AfUsart2Cts {
   fn af_usart2_cts(&self) -> usize;
}

pub trait AfTim2Etr {
   fn af_tim2_etr(&self) -> usize;
}

pub trait AfComp1Out {
   fn af_comp1_out(&self) -> usize;
}

pub trait AfEventout {
   fn af_eventout(&self) -> usize;
}

pub trait AfLptim1In2 {
   fn af_lptim1_in2(&self) -> usize;
}

pub trait AfTim2Ch2 {
   fn af_tim2_ch2(&self) -> usize;
}

pub trait AfI2c1Smba {
   fn af_i2c1_smba(&self) -> usize;
}

pub trait AfUsart2Rts {
   fn af_usart2_rts(&self) -> usize;
}

pub trait AfTim21Etr {
   fn af_tim21_etr(&self) -> usize;
}

pub trait AfTim21Ch1 {
   fn af_tim21_ch1(&self) -> usize;
}

pub trait AfTim2Ch3 {
   fn af_tim2_ch3(&self) -> usize;
}

pub trait AfUsart2Tx {
   fn af_usart2_tx(&self) -> usize;
}

pub trait AfLpuart1Tx {
   fn af_lpuart1_tx(&self) -> usize;
}

pub trait AfComp2Out {
   fn af_comp2_out(&self) -> usize;
}

pub trait AfTim21Ch2 {
   fn af_tim21_ch2(&self) -> usize;
}

pub trait AfTim2Ch4 {
   fn af_tim2_ch4(&self) -> usize;
}

pub trait AfUsart2Rx {
   fn af_usart2_rx(&self) -> usize;
}

pub trait AfLpuart1Rx {
   fn af_lpuart1_rx(&self) -> usize;
}

pub trait AfSpi1Nss {
   fn af_spi1_nss(&self) -> usize;
}

pub trait AfUart2Ck {
   fn af_uart2_ck(&self) -> usize;
}

pub trait AfTim22Etr {
   fn af_tim22_etr(&self) -> usize;
}

pub trait AfSpi1Sck {
   fn af_spi1_sck(&self) -> usize;
}

pub trait AfSpiMiso {
   fn af_spi_miso(&self) -> usize;
}

pub trait AfLptim1Etr {
   fn af_lptim1_etr(&self) -> usize;
}

pub trait AfLpuart1Cts {
   fn af_lpuart1_cts(&self) -> usize;
}

pub trait AfTim22Ch1 {
   fn af_tim22_ch1(&self) -> usize;
}

pub trait AfSpi1Mosi {
   fn af_spi1_mosi(&self) -> usize;
}

pub trait AfLptim1Out {
   fn af_lptim1_out(&self) -> usize;
}

pub trait AfTim22Ch2 {
   fn af_tim22_ch2(&self) -> usize;
}

pub trait AfMco {
   fn af_mco(&self) -> usize;
}

pub trait AfUsart2Ck {
   fn af_usart2_ck(&self) -> usize;
}

pub trait AfI2c1Scl {
   fn af_i2c1_scl(&self) -> usize;
}

pub trait AfI2c1Sda {
   fn af_i2c1_sda(&self) -> usize;
}

pub trait AfSpi1Mio {
   fn af_spi1_mio(&self) -> usize;
}

pub trait AfSwdio {
   fn af_swdio(&self) -> usize;
}

pub trait AfSwclk {
   fn af_swclk(&self) -> usize;
}

pub trait AfSpi1Miso {
   fn af_spi1_miso(&self) -> usize;
}

pub trait AfLpuart1Rts {
   fn af_lpuart1_rts(&self) -> usize;
}

pub trait AfSpiSck {
   fn af_spi_sck(&self) -> usize;
}

pub trait AfRtcOut {
   fn af_rtc_out(&self) -> usize;
}

pub trait AfRtcRefin {
   fn af_rtc_refin(&self) -> usize;
}

pub const PA0: Pa0 = Pa0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa0 {}

impl Pin for Pa0 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 0 }
}

impl AfLptim1In1 for Pa0 {
   fn af_lptim1_in1(&self) -> usize { 1 }
}

impl AfTim2Ch1 for Pa0 {
   fn af_tim2_ch1(&self) -> usize { 2 }
}

impl AfUsart2Cts for Pa0 {
   fn af_usart2_cts(&self) -> usize { 4 }
}

impl AfTim2Etr for Pa0 {
   fn af_tim2_etr(&self) -> usize { 5 }
}

impl AfComp1Out for Pa0 {
   fn af_comp1_out(&self) -> usize { 7 }
}

pub const PA1: Pa1 = Pa1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa1 {}

impl Pin for Pa1 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 1 }
}

impl AfEventout for Pa1 {
   fn af_eventout(&self) -> usize { 0 }
}

impl AfLptim1In2 for Pa1 {
   fn af_lptim1_in2(&self) -> usize { 1 }
}

impl AfTim2Ch2 for Pa1 {
   fn af_tim2_ch2(&self) -> usize { 2 }
}

impl AfI2c1Smba for Pa1 {
   fn af_i2c1_smba(&self) -> usize { 3 }
}

impl AfUsart2Rts for Pa1 {
   fn af_usart2_rts(&self) -> usize { 4 }
}

impl AfTim21Etr for Pa1 {
   fn af_tim21_etr(&self) -> usize { 5 }
}

pub const PA2: Pa2 = Pa2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa2 {}

impl Pin for Pa2 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 2 }
}

impl AfTim21Ch1 for Pa2 {
   fn af_tim21_ch1(&self) -> usize { 0 }
}

impl AfTim2Ch3 for Pa2 {
   fn af_tim2_ch3(&self) -> usize { 2 }
}

impl AfUsart2Tx for Pa2 {
   fn af_usart2_tx(&self) -> usize { 4 }
}

impl AfLpuart1Tx for Pa2 {
   fn af_lpuart1_tx(&self) -> usize { 6 }
}

impl AfComp2Out for Pa2 {
   fn af_comp2_out(&self) -> usize { 7 }
}

pub const PA3: Pa3 = Pa3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa3 {}

impl Pin for Pa3 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 3 }
}

impl AfTim21Ch2 for Pa3 {
   fn af_tim21_ch2(&self) -> usize { 0 }
}

impl AfTim2Ch4 for Pa3 {
   fn af_tim2_ch4(&self) -> usize { 2 }
}

impl AfUsart2Rx for Pa3 {
   fn af_usart2_rx(&self) -> usize { 4 }
}

impl AfLpuart1Rx for Pa3 {
   fn af_lpuart1_rx(&self) -> usize { 6 }
}

pub const PA4: Pa4 = Pa4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa4 {}

impl Pin for Pa4 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 4 }
}

impl AfSpi1Nss for Pa4 {
   fn af_spi1_nss(&self) -> usize { 0 }
}

impl AfLptim1In1 for Pa4 {
   fn af_lptim1_in1(&self) -> usize { 1 }
}

impl AfUart2Ck for Pa4 {
   fn af_uart2_ck(&self) -> usize { 4 }
}

impl AfTim22Etr for Pa4 {
   fn af_tim22_etr(&self) -> usize { 5 }
}

pub const PA5: Pa5 = Pa5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa5 {}

impl Pin for Pa5 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 5 }
}

impl AfSpi1Sck for Pa5 {
   fn af_spi1_sck(&self) -> usize { 0 }
}

impl AfLptim1In2 for Pa5 {
   fn af_lptim1_in2(&self) -> usize { 1 }
}

impl AfTim2Etr for Pa5 {
   fn af_tim2_etr(&self) -> usize { 3 }
}

impl AfTim2Ch1 for Pa5 {
   fn af_tim2_ch1(&self) -> usize { 5 }
}

pub const PA6: Pa6 = Pa6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa6 {}

impl Pin for Pa6 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 6 }
}

impl AfSpiMiso for Pa6 {
   fn af_spi_miso(&self) -> usize { 0 }
}

impl AfLptim1Etr for Pa6 {
   fn af_lptim1_etr(&self) -> usize { 1 }
}

impl AfLpuart1Cts for Pa6 {
   fn af_lpuart1_cts(&self) -> usize { 4 }
}

impl AfTim22Ch1 for Pa6 {
   fn af_tim22_ch1(&self) -> usize { 5 }
}

impl AfEventout for Pa6 {
   fn af_eventout(&self) -> usize { 6 }
}

impl AfComp1Out for Pa6 {
   fn af_comp1_out(&self) -> usize { 7 }
}

pub const PA7: Pa7 = Pa7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa7 {}

impl Pin for Pa7 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 7 }
}

impl AfSpi1Mosi for Pa7 {
   fn af_spi1_mosi(&self) -> usize { 0 }
}

impl AfLptim1Out for Pa7 {
   fn af_lptim1_out(&self) -> usize { 1 }
}

impl AfUsart2Cts for Pa7 {
   fn af_usart2_cts(&self) -> usize { 4 }
}

impl AfTim22Ch2 for Pa7 {
   fn af_tim22_ch2(&self) -> usize { 5 }
}

impl AfEventout for Pa7 {
   fn af_eventout(&self) -> usize { 6 }
}

impl AfComp2Out for Pa7 {
   fn af_comp2_out(&self) -> usize { 7 }
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

impl AfLptim1In1 for Pa8 {
   fn af_lptim1_in1(&self) -> usize { 2 }
}

impl AfEventout for Pa8 {
   fn af_eventout(&self) -> usize { 3 }
}

impl AfUsart2Ck for Pa8 {
   fn af_usart2_ck(&self) -> usize { 4 }
}

impl AfTim2Ch1 for Pa8 {
   fn af_tim2_ch1(&self) -> usize { 5 }
}

pub const PA9: Pa9 = Pa9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa9 {}

impl Pin for Pa9 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 9 }
}

impl AfMco for Pa9 {
   fn af_mco(&self) -> usize { 0 }
}

impl AfI2c1Scl for Pa9 {
   fn af_i2c1_scl(&self) -> usize { 1 }
}

impl AfUsart2Tx for Pa9 {
   fn af_usart2_tx(&self) -> usize { 4 }
}

impl AfTim22Ch1 for Pa9 {
   fn af_tim22_ch1(&self) -> usize { 5 }
}

pub const PA10: Pa10 = Pa10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa10 {}

impl Pin for Pa10 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 10 }
}

impl AfI2c1Sda for Pa10 {
   fn af_i2c1_sda(&self) -> usize { 1 }
}

impl AfUsart2Rx for Pa10 {
   fn af_usart2_rx(&self) -> usize { 4 }
}

impl AfTim22Ch2 for Pa10 {
   fn af_tim22_ch2(&self) -> usize { 5 }
}

pub const PA11: Pa11 = Pa11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa11 {}

impl Pin for Pa11 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 11 }
}

impl AfSpi1Mio for Pa11 {
   fn af_spi1_mio(&self) -> usize { 0 }
}

impl AfEventout for Pa11 {
   fn af_eventout(&self) -> usize { 2 }
}

impl AfUsart2Cts for Pa11 {
   fn af_usart2_cts(&self) -> usize { 4 }
}

impl AfTim21Ch2 for Pa11 {
   fn af_tim21_ch2(&self) -> usize { 5 }
}

impl AfComp1Out for Pa11 {
   fn af_comp1_out(&self) -> usize { 7 }
}

pub const PA12: Pa12 = Pa12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa12 {}

impl Pin for Pa12 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 12 }
}

impl AfSpi1Mosi for Pa12 {
   fn af_spi1_mosi(&self) -> usize { 0 }
}

impl AfEventout for Pa12 {
   fn af_eventout(&self) -> usize { 2 }
}

impl AfUsart2Rts for Pa12 {
   fn af_usart2_rts(&self) -> usize { 4 }
}

impl AfComp2Out for Pa12 {
   fn af_comp2_out(&self) -> usize { 7 }
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

impl AfLptim1Etr for Pa13 {
   fn af_lptim1_etr(&self) -> usize { 1 }
}

impl AfLpuart1Rx for Pa13 {
   fn af_lpuart1_rx(&self) -> usize { 6 }
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

impl AfLptim1Out for Pa14 {
   fn af_lptim1_out(&self) -> usize { 1 }
}

impl AfI2c1Smba for Pa14 {
   fn af_i2c1_smba(&self) -> usize { 3 }
}

impl AfUsart2Tx for Pa14 {
   fn af_usart2_tx(&self) -> usize { 4 }
}

impl AfLpuart1Tx for Pa14 {
   fn af_lpuart1_tx(&self) -> usize { 6 }
}

pub const PA15: Pa15 = Pa15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa15 {}

impl Pin for Pa15 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 15 }
}

impl AfSpi1Nss for Pa15 {
   fn af_spi1_nss(&self) -> usize { 0 }
}

impl AfTim2Etr for Pa15 {
   fn af_tim2_etr(&self) -> usize { 2 }
}

impl AfEventout for Pa15 {
   fn af_eventout(&self) -> usize { 3 }
}

impl AfUsart2Rx for Pa15 {
   fn af_usart2_rx(&self) -> usize { 4 }
}

impl AfTim2Ch1 for Pa15 {
   fn af_tim2_ch1(&self) -> usize { 5 }
}

pub const PB0: Pb0 = Pb0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb0 {}

impl Pin for Pb0 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 0 }
}

impl AfEventout for Pb0 {
   fn af_eventout(&self) -> usize { 0 }
}

impl AfSpi1Miso for Pb0 {
   fn af_spi1_miso(&self) -> usize { 1 }
}

impl AfUsart2Rts for Pb0 {
   fn af_usart2_rts(&self) -> usize { 4 }
}

impl AfTim2Ch3 for Pb0 {
   fn af_tim2_ch3(&self) -> usize { 5 }
}

pub const PB1: Pb1 = Pb1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb1 {}

impl Pin for Pb1 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 1 }
}

impl AfUsart2Ck for Pb1 {
   fn af_usart2_ck(&self) -> usize { 0 }
}

impl AfSpi1Mosi for Pb1 {
   fn af_spi1_mosi(&self) -> usize { 1 }
}

impl AfLpuart1Rts for Pb1 {
   fn af_lpuart1_rts(&self) -> usize { 4 }
}

impl AfTim2Ch4 for Pb1 {
   fn af_tim2_ch4(&self) -> usize { 5 }
}

pub const PB2: Pb2 = Pb2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb2 {}

impl Pin for Pb2 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 2 }
}

impl AfLptim1Out for Pb2 {
   fn af_lptim1_out(&self) -> usize { 2 }
}

pub const PB3: Pb3 = Pb3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb3 {}

impl Pin for Pb3 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 3 }
}

impl AfSpi1Sck for Pb3 {
   fn af_spi1_sck(&self) -> usize { 0 }
}

impl AfTim2Ch2 for Pb3 {
   fn af_tim2_ch2(&self) -> usize { 2 }
}

impl AfEventout for Pb3 {
   fn af_eventout(&self) -> usize { 4 }
}

pub const PB4: Pb4 = Pb4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb4 {}

impl Pin for Pb4 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 4 }
}

impl AfSpi1Miso for Pb4 {
   fn af_spi1_miso(&self) -> usize { 0 }
}

impl AfEventout for Pb4 {
   fn af_eventout(&self) -> usize { 2 }
}

impl AfTim22Ch1 for Pb4 {
   fn af_tim22_ch1(&self) -> usize { 4 }
}

pub const PB5: Pb5 = Pb5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb5 {}

impl Pin for Pb5 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 5 }
}

impl AfSpi1Mosi for Pb5 {
   fn af_spi1_mosi(&self) -> usize { 0 }
}

impl AfLptim1In1 for Pb5 {
   fn af_lptim1_in1(&self) -> usize { 2 }
}

impl AfI2c1Smba for Pb5 {
   fn af_i2c1_smba(&self) -> usize { 3 }
}

impl AfTim22Ch2 for Pb5 {
   fn af_tim22_ch2(&self) -> usize { 4 }
}

pub const PB6: Pb6 = Pb6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb6 {}

impl Pin for Pb6 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 6 }
}

impl AfUsart2Tx for Pb6 {
   fn af_usart2_tx(&self) -> usize { 0 }
}

impl AfI2c1Scl for Pb6 {
   fn af_i2c1_scl(&self) -> usize { 1 }
}

impl AfLptim1Etr for Pb6 {
   fn af_lptim1_etr(&self) -> usize { 2 }
}

impl AfTim21Ch1 for Pb6 {
   fn af_tim21_ch1(&self) -> usize { 5 }
}

pub const PB7: Pb7 = Pb7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb7 {}

impl Pin for Pb7 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 7 }
}

impl AfUsart2Rx for Pb7 {
   fn af_usart2_rx(&self) -> usize { 0 }
}

impl AfI2c1Sda for Pb7 {
   fn af_i2c1_sda(&self) -> usize { 1 }
}

impl AfLptim1In2 for Pb7 {
   fn af_lptim1_in2(&self) -> usize { 2 }
}

pub const PB8: Pb8 = Pb8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb8 {}

impl Pin for Pb8 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 8 }
}

impl AfI2c1Scl for Pb8 {
   fn af_i2c1_scl(&self) -> usize { 4 }
}

pub const PB9: Pb9 = Pb9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb9 {}

impl Pin for Pb9 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 9 }
}

impl AfEventout for Pb9 {
   fn af_eventout(&self) -> usize { 2 }
}

impl AfI2c1Sda for Pb9 {
   fn af_i2c1_sda(&self) -> usize { 4 }
}

pub const PB10: Pb10 = Pb10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb10 {}

impl Pin for Pb10 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 10 }
}

impl AfTim2Ch3 for Pb10 {
   fn af_tim2_ch3(&self) -> usize { 2 }
}

impl AfLpuart1Tx for Pb10 {
   fn af_lpuart1_tx(&self) -> usize { 6 }
}

pub const PB11: Pb11 = Pb11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb11 {}

impl Pin for Pb11 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 11 }
}

impl AfEventout for Pb11 {
   fn af_eventout(&self) -> usize { 0 }
}

impl AfTim2Ch4 for Pb11 {
   fn af_tim2_ch4(&self) -> usize { 2 }
}

impl AfLpuart1Rx for Pb11 {
   fn af_lpuart1_rx(&self) -> usize { 6 }
}

pub const PB12: Pb12 = Pb12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb12 {}

impl Pin for Pb12 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 12 }
}

impl AfSpi1Nss for Pb12 {
   fn af_spi1_nss(&self) -> usize { 0 }
}

impl AfEventout for Pb12 {
   fn af_eventout(&self) -> usize { 6 }
}

pub const PB13: Pb13 = Pb13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb13 {}

impl Pin for Pb13 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 13 }
}

impl AfSpiSck for Pb13 {
   fn af_spi_sck(&self) -> usize { 0 }
}

impl AfMco for Pb13 {
   fn af_mco(&self) -> usize { 1 }
}

impl AfTim21Ch1 for Pb13 {
   fn af_tim21_ch1(&self) -> usize { 5 }
}

impl AfLpuart1Cts for Pb13 {
   fn af_lpuart1_cts(&self) -> usize { 6 }
}

pub const PB14: Pb14 = Pb14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb14 {}

impl Pin for Pb14 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 14 }
}

impl AfSpi1Miso for Pb14 {
   fn af_spi1_miso(&self) -> usize { 0 }
}

impl AfRtcOut for Pb14 {
   fn af_rtc_out(&self) -> usize { 2 }
}

impl AfTim21Ch2 for Pb14 {
   fn af_tim21_ch2(&self) -> usize { 5 }
}

impl AfLpuart1Rts for Pb14 {
   fn af_lpuart1_rts(&self) -> usize { 6 }
}

pub const PB15: Pb15 = Pb15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb15 {}

impl Pin for Pb15 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 15 }
}

impl AfSpi1Mosi for Pb15 {
   fn af_spi1_mosi(&self) -> usize { 0 }
}

impl AfRtcRefin for Pb15 {
   fn af_rtc_refin(&self) -> usize { 2 }
}

pub const PC0: Pc0 = Pc0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc0 {}

impl Pin for Pc0 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 0 }
}

impl AfLptim1In1 for Pc0 {
   fn af_lptim1_in1(&self) -> usize { 0 }
}

impl AfEventout for Pc0 {
   fn af_eventout(&self) -> usize { 2 }
}

impl AfLpuart1Rx for Pc0 {
   fn af_lpuart1_rx(&self) -> usize { 6 }
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

pub const PH0: Ph0 = Ph0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ph0 {}

impl Pin for Ph0 {
   fn port(&self) -> Gpio { GPIOH }
   fn index(&self) -> usize { 0 }
}

pub const PH1: Ph1 = Ph1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ph1 {}

impl Pin for Ph1 {
   fn port(&self) -> Gpio { GPIOH }
   fn index(&self) -> usize { 1 }
}

