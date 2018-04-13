pub use ::bobbin_common::*;
pub use ::bobbin_common::tree::*;
pub use ::hz::Hz;

#[derive(Default)]
pub struct Clocks<CP: ClockProvider> { provider: CP }

impl<CP: ClockProvider> Deref for Clocks<CP> {
    type Target = CP;
    fn deref(&self) -> &CP { &self.provider }
}


// Define Global Clocks

#[derive(Default)]
pub struct Hsi16 {}
impl Clock for Hsi16 { fn hz() -> Hz { Hz::from_num(16000000) } }

#[derive(Default)]
pub struct Lsi {}
impl Clock for Lsi { fn hz() -> Hz { Hz::from_num(32000) } }


pub trait ClockProvider : Default {
    type Osc: Clock;
    type Osc32: Clock;
    fn osc(&self) -> Hz { Self::Osc::hz() }
    fn osc32(&self) -> Hz { Self::Osc32::hz() }
    fn hsi16(&self) -> Hz { Hz::from_num(16000000) }
    fn hse(&self) -> Hz { self.osc() }
    fn lsi(&self) -> Hz { Hz::from_num(32000) }
    fn lse(&self) -> Hz { self.osc32() }
    fn pllclk(&self) -> Hz { unimplemented!() }
    fn pll48clk(&self) -> Hz { unimplemented!() }
    fn sysclk(&self) -> Hz { unimplemented!() }
    fn hclk(&self) -> Hz { unimplemented!() }
    fn systick(&self) -> Hz { unimplemented!() }
    fn pclk1(&self) -> Hz { unimplemented!() }
    fn pclk2(&self) -> Hz { unimplemented!() }
    fn tim_pclk1(&self) -> Hz { unimplemented!() }
    fn tim_pclk2(&self) -> Hz { unimplemented!() }
    fn usart1(&self) -> Hz { unimplemented!() }
    fn usart2(&self) -> Hz { unimplemented!() }
    fn usart3(&self) -> Hz { unimplemented!() }
    fn lpuart1(&self) -> Hz { unimplemented!() }
    fn i2c1(&self) -> Hz { unimplemented!() }
    fn i2c2(&self) -> Hz { unimplemented!() }
    fn i2c3(&self) -> Hz { unimplemented!() }
    fn lptim1(&self) -> Hz { unimplemented!() }
    fn lptim2(&self) -> Hz { unimplemented!() }
}

impl<CP> ClockFor<::flash::Flash> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::flash::Flash) -> Hz { self.hclk() }
}

impl<CP> ClockFor<::pwr::Pwr> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::pwr::Pwr) -> Hz { self.pclk1() }
}

impl<CP> ClockFor<::syscfg::Syscfg> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::syscfg::Syscfg) -> Hz { self.pclk2() }
}

impl<CP> ClockFor<::dac::Dac1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::dac::Dac1) -> Hz { self.pclk1() }
}

impl<CP> ClockFor<::rng::Rng> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::rng::Rng) -> Hz { self.hclk() }
}

impl<CP> ClockFor<::crc::Crc> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::crc::Crc) -> Hz { self.hclk() }
}

impl<CP> ClockFor<::lptim::Lptim1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::lptim::Lptim1) -> Hz { self.lptim1() }
}

impl<CP> ClockFor<::lptim::Lptim2> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::lptim::Lptim2) -> Hz { self.lptim2() }
}

impl<CP> ClockFor<::dma::Dma1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::dma::Dma1) -> Hz { self.hclk() }
}

impl<CP> ClockFor<::dma::Dma2> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::dma::Dma2) -> Hz { self.hclk() }
}

impl<CP> ClockFor<::i2c::I2c1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::i2c::I2c1) -> Hz { self.pclk1() }
}

impl<CP> ClockFor<::i2c::I2c2> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::i2c::I2c2) -> Hz { self.pclk1() }
}

impl<CP> ClockFor<::i2c::I2c3> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::i2c::I2c3) -> Hz { self.pclk1() }
}

impl<CP> ClockFor<::tim_adv::Tim1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_adv::Tim1) -> Hz { self.pclk2() }
}

impl<CP> ClockFor<::tim_bas::Tim6> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_bas::Tim6) -> Hz { self.tim_pclk1() }
}

impl<CP> ClockFor<::tim_bas::Tim7> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_bas::Tim7) -> Hz { self.tim_pclk1() }
}

impl<CP> ClockFor<::tim_gen::Tim2> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_gen::Tim2) -> Hz { self.tim_pclk1() }
}

impl<CP> ClockFor<::tim_gen::Tim15> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_gen::Tim15) -> Hz { self.pclk2() }
}

impl<CP> ClockFor<::tim_gen::Tim16> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_gen::Tim16) -> Hz { self.tim_pclk2() }
}

impl<CP> ClockFor<::gpio::Gpioa> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::gpio::Gpioa) -> Hz { self.hclk() }
}

impl<CP> ClockFor<::gpio::Gpiob> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::gpio::Gpiob) -> Hz { self.hclk() }
}

impl<CP> ClockFor<::gpio::Gpioc> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::gpio::Gpioc) -> Hz { self.hclk() }
}

impl<CP> ClockFor<::gpio::Gpiod> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::gpio::Gpiod) -> Hz { self.hclk() }
}

impl<CP> ClockFor<::gpio::Gpioe> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::gpio::Gpioe) -> Hz { self.hclk() }
}

impl<CP> ClockFor<::gpio::Gpioh> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::gpio::Gpioh) -> Hz { self.hclk() }
}

impl<CP> ClockFor<::usart::Usart1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::usart::Usart1) -> Hz { self.pclk2() }
}

impl<CP> ClockFor<::usart::Usart2> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::usart::Usart2) -> Hz { self.usart2() }
}

impl<CP> ClockFor<::usart::Usart3> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::usart::Usart3) -> Hz { self.pclk1() }
}

impl<CP> ClockFor<::lpuart::Lpuart1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::lpuart::Lpuart1) -> Hz { self.lpuart1() }
}

impl<CP> ClockFor<::spi::Spi1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::spi::Spi1) -> Hz { self.pclk2() }
}

impl<CP> ClockFor<::spi::Spi2> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::spi::Spi2) -> Hz { self.pclk1() }
}

impl<CP> ClockFor<::spi::Spi3> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::spi::Spi3) -> Hz { self.pclk1() }
}

impl<CP> ClockFor<::adc::Adc1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::adc::Adc1) -> Hz { self.hclk() }
}

impl<CP> ClockFor<::adc::Adc2> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::adc::Adc2) -> Hz { self.hclk() }
}

impl<CP> ClockFor<::adc::Adc3> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::adc::Adc3) -> Hz { self.hclk() }
}

