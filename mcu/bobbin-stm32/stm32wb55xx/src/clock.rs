use ::bobbin_mcu::clock::{Clock, ClockFor};
use ::bobbin_mcu::hz::Hz;

#[derive(Default)]
pub struct Clocks<CP: ClockProvider> { provider: CP }

impl<CP: ClockProvider> ::core::ops::Deref for Clocks<CP> {
    type Target = CP;
    fn deref(&self) -> &CP { &self.provider }
}


// Define Global Clocks

#[derive(Default)]
pub struct Hsi16 {}
impl Clock for Hsi16 { fn hz() -> Hz { Hz::from_num(16000000) } }

#[derive(Default)]
pub struct Hsi48 {}
impl Clock for Hsi48 { fn hz() -> Hz { Hz::from_num(48000000) } }

#[derive(Default)]
pub struct Lsi1 {}
impl Clock for Lsi1 { fn hz() -> Hz { Hz::from_num(32000) } }

#[derive(Default)]
pub struct Lsi2 {}
impl Clock for Lsi2 { fn hz() -> Hz { Hz::from_num(32000) } }


pub trait ClockProvider : Default {
    type Osc: Clock;
    type Osc32: Clock;
    fn osc(&self) -> Hz { Self::Osc::hz() }
    fn osc32(&self) -> Hz { Self::Osc32::hz() }
    fn hsi16(&self) -> Hz { Hz::from_num(16000000) }
    fn hsi48(&self) -> Hz { Hz::from_num(48000000) }
    fn hse(&self) -> Hz { self.osc() }
    fn lsi1(&self) -> Hz { Hz::from_num(32000) }
    fn lsi2(&self) -> Hz { Hz::from_num(32000) }
    fn lse(&self) -> Hz { self.osc32() }
    fn lsi(&self) -> Hz { unimplemented!() }
    fn pllclk(&self) -> Hz { unimplemented!() }
    fn pllsai1rclk(&self) -> Hz { unimplemented!() }
    fn pllsai1qclk(&self) -> Hz { unimplemented!() }
    fn pllsai1pclk(&self) -> Hz { unimplemented!() }
    fn sysclk(&self) -> Hz { unimplemented!() }
    fn hclk1(&self) -> Hz { unimplemented!() }
    fn hclk4(&self) -> Hz { unimplemented!() }
    fn hclk2(&self) -> Hz { unimplemented!() }
    fn pclk1(&self) -> Hz { unimplemented!() }
    fn pclk2(&self) -> Hz { unimplemented!() }
    fn rtcclk(&self) -> Hz { unimplemented!() }
    fn systick(&self) -> Hz { unimplemented!() }
    fn tim_pclk1(&self) -> Hz { unimplemented!() }
    fn tim_pclk2(&self) -> Hz { unimplemented!() }
    fn rf(&self) -> Hz { unimplemented!() }
    fn rf_wkup(&self) -> Hz { unimplemented!() }
    fn usart1(&self) -> Hz { unimplemented!() }
    fn lpuart1(&self) -> Hz { unimplemented!() }
    fn i2c1(&self) -> Hz { unimplemented!() }
    fn i2c3(&self) -> Hz { unimplemented!() }
    fn lptim1(&self) -> Hz { unimplemented!() }
    fn lptim2(&self) -> Hz { unimplemented!() }
    fn rng(&self) -> Hz { unimplemented!() }
    fn adc(&self) -> Hz { unimplemented!() }
    fn clk48(&self) -> Hz { unimplemented!() }
    fn sai1(&self) -> Hz { unimplemented!() }
}

impl<CP> ClockFor<::flash::Flash> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::flash::Flash) -> Hz { self.hclk1() }
}

impl<CP> ClockFor<::dma::Dma1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::dma::Dma1) -> Hz { self.hclk1() }
}

impl<CP> ClockFor<::dma::Dma2> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::dma::Dma2) -> Hz { self.hclk1() }
}

impl<CP> ClockFor<::crc::Crc> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::crc::Crc) -> Hz { self.hclk1() }
}

impl<CP> ClockFor<::i2c::I2c1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::i2c::I2c1) -> Hz { self.pclk1() }
}

impl<CP> ClockFor<::i2c::I2c3> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::i2c::I2c3) -> Hz { self.pclk1() }
}

impl<CP> ClockFor<::rng::Rng> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::rng::Rng) -> Hz { self.rng() }
}

impl<CP> ClockFor<::adc::Adc1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::adc::Adc1) -> Hz { self.hclk1() }
}

impl<CP> ClockFor<::gpio::Gpioa> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::gpio::Gpioa) -> Hz { self.hclk1() }
}

impl<CP> ClockFor<::gpio::Gpiob> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::gpio::Gpiob) -> Hz { self.hclk1() }
}

impl<CP> ClockFor<::gpio::Gpioc> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::gpio::Gpioc) -> Hz { self.hclk1() }
}

impl<CP> ClockFor<::gpio::Gpiod> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::gpio::Gpiod) -> Hz { self.hclk1() }
}

impl<CP> ClockFor<::gpio::Gpioe> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::gpio::Gpioe) -> Hz { self.hclk1() }
}

impl<CP> ClockFor<::gpio::Gpioh> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::gpio::Gpioh) -> Hz { self.hclk1() }
}

impl<CP> ClockFor<::lptim::Lptim1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::lptim::Lptim1) -> Hz { self.lptim1() }
}

impl<CP> ClockFor<::lptim::Lptim2> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::lptim::Lptim2) -> Hz { self.lptim2() }
}

impl<CP> ClockFor<::usart::Usart1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::usart::Usart1) -> Hz { self.usart1() }
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

impl<CP> ClockFor<::rtc::Rtc> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::rtc::Rtc) -> Hz { self.pclk1() }
}

