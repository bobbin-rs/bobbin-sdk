pub use ::bobbin_common::*;
pub use ::bobbin_common::tree::*;
pub use ::hz::Hz;

pub struct ClockTree<T>(T);


// Define Global Clocks

pub struct Hsi16 {}
impl Clock for Hsi16 { fn hz() -> Hz { Hz::from_num(16000000) } }

pub struct Lsi {}
impl Clock for Lsi { fn hz() -> Hz { Hz::from_num(32000) } }


pub trait Clocks {
    type Osc: Clock;
    type Osc32: Clock;
    fn osc() -> Hz { Self::Osc::hz() }
    fn osc32() -> Hz { Self::Osc32::hz() }
    fn hsi16() -> Hz { Hz::from_num(16000000) }
    fn hse() -> Hz { Self::osc() }
    fn lsi() -> Hz { Hz::from_num(32000) }
    fn lse() -> Hz { Self::osc32() }
    fn pllclk() -> Hz { Hz::from_num(0) }
    fn pll48clk() -> Hz { Hz::from_num(0) }
    fn sysclk() -> Hz { Hz::from_num(0) }
    fn hclk() -> Hz { Hz::from_num(0) }
    fn systick() -> Hz { Hz::from_num(0) }
    fn pclk1() -> Hz { Hz::from_num(0) }
    fn pclk2() -> Hz { Hz::from_num(0) }
    fn tim_pclk1() -> Hz { Hz::from_num(0) }
    fn tim_pclk2() -> Hz { Hz::from_num(0) }
    fn usart1() -> Hz { Hz::from_num(0) }
    fn usart2() -> Hz { Hz::from_num(0) }
    fn usart3() -> Hz { Hz::from_num(0) }
    fn lpuart1() -> Hz { Hz::from_num(0) }
    fn i2c1() -> Hz { Hz::from_num(0) }
    fn i2c2() -> Hz { Hz::from_num(0) }
    fn i2c3() -> Hz { Hz::from_num(0) }
    fn lptim1() -> Hz { Hz::from_num(0) }
    fn lptim2() -> Hz { Hz::from_num(0) }
}

impl<T> ClockFor<::flash::Flash> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::flash::Flash) -> Hz { T::hclk() }
}

impl<T> ClockFor<::pwr::Pwr> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::pwr::Pwr) -> Hz { T::pclk1() }
}

impl<T> ClockFor<::syscfg::Syscfg> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::syscfg::Syscfg) -> Hz { T::pclk2() }
}

impl<T> ClockFor<::dac::Dac1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::dac::Dac1) -> Hz { T::pclk1() }
}

impl<T> ClockFor<::rng::Rng> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::rng::Rng) -> Hz { T::hclk() }
}

impl<T> ClockFor<::crc::Crc> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::crc::Crc) -> Hz { T::hclk() }
}

impl<T> ClockFor<::lptim::Lptim1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::lptim::Lptim1) -> Hz { T::lptim1() }
}

impl<T> ClockFor<::lptim::Lptim2> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::lptim::Lptim2) -> Hz { T::lptim2() }
}

impl<T> ClockFor<::dma::Dma1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::dma::Dma1) -> Hz { T::hclk() }
}

impl<T> ClockFor<::dma::Dma2> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::dma::Dma2) -> Hz { T::hclk() }
}

impl<T> ClockFor<::i2c::I2c1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::i2c::I2c1) -> Hz { T::pclk1() }
}

impl<T> ClockFor<::i2c::I2c2> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::i2c::I2c2) -> Hz { T::pclk1() }
}

impl<T> ClockFor<::i2c::I2c3> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::i2c::I2c3) -> Hz { T::pclk1() }
}

impl<T> ClockFor<::tim_adv::Tim1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_adv::Tim1) -> Hz { T::pclk2() }
}

impl<T> ClockFor<::tim_bas::Tim6> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_bas::Tim6) -> Hz { T::tim_pclk1() }
}

impl<T> ClockFor<::tim_bas::Tim7> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_bas::Tim7) -> Hz { T::tim_pclk1() }
}

impl<T> ClockFor<::tim_gen::Tim2> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_gen::Tim2) -> Hz { T::tim_pclk1() }
}

impl<T> ClockFor<::tim_gen::Tim15> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_gen::Tim15) -> Hz { T::pclk2() }
}

impl<T> ClockFor<::tim_gen::Tim16> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_gen::Tim16) -> Hz { T::tim_pclk2() }
}

impl<T> ClockFor<::gpio::Gpioa> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::gpio::Gpioa) -> Hz { T::hclk() }
}

impl<T> ClockFor<::gpio::Gpiob> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::gpio::Gpiob) -> Hz { T::hclk() }
}

impl<T> ClockFor<::gpio::Gpioc> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::gpio::Gpioc) -> Hz { T::hclk() }
}

impl<T> ClockFor<::gpio::Gpiod> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::gpio::Gpiod) -> Hz { T::hclk() }
}

impl<T> ClockFor<::gpio::Gpioe> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::gpio::Gpioe) -> Hz { T::hclk() }
}

impl<T> ClockFor<::gpio::Gpioh> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::gpio::Gpioh) -> Hz { T::hclk() }
}

impl<T> ClockFor<::usart::Usart1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::usart::Usart1) -> Hz { T::pclk2() }
}

impl<T> ClockFor<::usart::Usart2> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::usart::Usart2) -> Hz { T::usart2() }
}

impl<T> ClockFor<::usart::Usart3> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::usart::Usart3) -> Hz { T::pclk1() }
}

impl<T> ClockFor<::lpuart::Lpuart1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::lpuart::Lpuart1) -> Hz { T::lpuart1() }
}

impl<T> ClockFor<::spi::Spi1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::spi::Spi1) -> Hz { T::pclk2() }
}

impl<T> ClockFor<::spi::Spi2> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::spi::Spi2) -> Hz { T::pclk1() }
}

impl<T> ClockFor<::spi::Spi3> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::spi::Spi3) -> Hz { T::pclk1() }
}

impl<T> ClockFor<::adc::Adc1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::adc::Adc1) -> Hz { T::hclk() }
}

impl<T> ClockFor<::adc::Adc2> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::adc::Adc2) -> Hz { T::hclk() }
}

impl<T> ClockFor<::adc::Adc3> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::adc::Adc3) -> Hz { T::hclk() }
}

