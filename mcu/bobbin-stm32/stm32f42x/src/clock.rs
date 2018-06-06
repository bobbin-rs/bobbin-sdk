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
pub struct Hsi {}
impl Clock for Hsi { fn hz() -> Hz { Hz::from_num(16000000) } }

#[derive(Default)]
pub struct Lsi {}
impl Clock for Lsi { fn hz() -> Hz { Hz::from_num(32000) } }


pub trait ClockProvider : Default {
    type Osc: Clock;
    type Osc32: Clock;
    fn osc(&self) -> Hz { Self::Osc::hz() }
    fn osc32(&self) -> Hz { Self::Osc32::hz() }
    fn hsi(&self) -> Hz { Hz::from_num(16000000) }
    fn hse(&self) -> Hz { self.osc() }
    fn lsi(&self) -> Hz { Hz::from_num(32000) }
    fn lse(&self) -> Hz { self.osc32() }
    fn pllclk(&self) -> Hz { unimplemented!() }
    fn pll48clk(&self) -> Hz { unimplemented!() }
    fn sysclk(&self) -> Hz { unimplemented!() }
    fn i2s(&self) -> Hz { unimplemented!() }
    fn otg_hs_scl(&self) -> Hz { unimplemented!() }
    fn hclk(&self) -> Hz { unimplemented!() }
    fn systick(&self) -> Hz { unimplemented!() }
    fn fclk(&self) -> Hz { unimplemented!() }
    fn pclk1(&self) -> Hz { unimplemented!() }
    fn pclk2(&self) -> Hz { unimplemented!() }
    fn tim_pclk1(&self) -> Hz { unimplemented!() }
    fn tim_pclk2(&self) -> Hz { unimplemented!() }
    fn rtc(&self) -> Hz { unimplemented!() }
    fn sai1(&self) -> Hz { unimplemented!() }
    fn sai2(&self) -> Hz { unimplemented!() }
    fn lcd_tft(&self) -> Hz { unimplemented!() }
    fn eth_mactx(&self) -> Hz { unimplemented!() }
    fn eth_macrx(&self) -> Hz { unimplemented!() }
    fn eth_macrmii(&self) -> Hz { unimplemented!() }
}

impl<CP> ClockFor<::dac::Dac> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::dac::Dac) -> Hz { self.pclk1() }
}

impl<CP> ClockFor<::iwdg::Iwdg> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::iwdg::Iwdg) -> Hz { self.lsi() }
}

impl<CP> ClockFor<::wwdg::Wwdg> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::wwdg::Wwdg) -> Hz { self.pclk1() }
}

impl<CP> ClockFor<::crc::Crc> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::crc::Crc) -> Hz { self.hclk() }
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

impl<CP> ClockFor<::tim_gen::Tim3> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_gen::Tim3) -> Hz { self.tim_pclk1() }
}

impl<CP> ClockFor<::tim_gen::Tim4> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_gen::Tim4) -> Hz { self.tim_pclk1() }
}

impl<CP> ClockFor<::tim_gen::Tim5> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_gen::Tim5) -> Hz { self.tim_pclk1() }
}

impl<CP> ClockFor<::tim_gen::Tim9> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_gen::Tim9) -> Hz { self.tim_pclk2() }
}

impl<CP> ClockFor<::tim_gen::Tim10> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_gen::Tim10) -> Hz { self.tim_pclk2() }
}

impl<CP> ClockFor<::tim_gen::Tim11> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_gen::Tim11) -> Hz { self.tim_pclk2() }
}

impl<CP> ClockFor<::tim_gen::Tim12> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_gen::Tim12) -> Hz { self.tim_pclk1() }
}

impl<CP> ClockFor<::tim_gen::Tim13> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_gen::Tim13) -> Hz { self.tim_pclk1() }
}

impl<CP> ClockFor<::tim_gen::Tim14> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_gen::Tim14) -> Hz { self.tim_pclk1() }
}

impl<CP> ClockFor<::tim_adv::Tim1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_adv::Tim1) -> Hz { self.tim_pclk2() }
}

impl<CP> ClockFor<::tim_adv::Tim8> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_adv::Tim8) -> Hz { self.tim_pclk2() }
}

impl<CP> ClockFor<::adc::Adc1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::adc::Adc1) -> Hz { self.pclk2() }
}

impl<CP> ClockFor<::adc::Adc2> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::adc::Adc2) -> Hz { self.pclk2() }
}

impl<CP> ClockFor<::adc::Adc3> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::adc::Adc3) -> Hz { self.pclk2() }
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

impl<CP> ClockFor<::spi::Spi4> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::spi::Spi4) -> Hz { self.pclk2() }
}

impl<CP> ClockFor<::spi::Spi5> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::spi::Spi5) -> Hz { self.pclk2() }
}

impl<CP> ClockFor<::spi::Spi6> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::spi::Spi6) -> Hz { self.pclk2() }
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

impl<CP> ClockFor<::can::Can1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::can::Can1) -> Hz { self.pclk1() }
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

impl<CP> ClockFor<::gpio::Gpiof> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::gpio::Gpiof) -> Hz { self.hclk() }
}

impl<CP> ClockFor<::gpio::Gpiog> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::gpio::Gpiog) -> Hz { self.hclk() }
}

impl<CP> ClockFor<::gpio::Gpioh> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::gpio::Gpioh) -> Hz { self.hclk() }
}

impl<CP> ClockFor<::gpio::Gpioi> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::gpio::Gpioi) -> Hz { self.hclk() }
}

impl<CP> ClockFor<::gpio::Gpioj> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::gpio::Gpioj) -> Hz { self.hclk() }
}

impl<CP> ClockFor<::gpio::Gpiok> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::gpio::Gpiok) -> Hz { self.hclk() }
}

impl<CP> ClockFor<::usart::Usart1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::usart::Usart1) -> Hz { self.pclk2() }
}

impl<CP> ClockFor<::usart::Usart2> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::usart::Usart2) -> Hz { self.pclk1() }
}

impl<CP> ClockFor<::usart::Usart3> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::usart::Usart3) -> Hz { self.pclk1() }
}

impl<CP> ClockFor<::usart::Uart4> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::usart::Uart4) -> Hz { self.pclk1() }
}

impl<CP> ClockFor<::usart::Uart5> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::usart::Uart5) -> Hz { self.pclk1() }
}

impl<CP> ClockFor<::usart::Usart6> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::usart::Usart6) -> Hz { self.pclk2() }
}

impl<CP> ClockFor<::usart::Uart7> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::usart::Uart7) -> Hz { self.pclk1() }
}

impl<CP> ClockFor<::usart::Uart8> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::usart::Uart8) -> Hz { self.pclk1() }
}

impl<CP> ClockFor<::dma::Dma1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::dma::Dma1) -> Hz { self.hclk() }
}

impl<CP> ClockFor<::dma::Dma2> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::dma::Dma2) -> Hz { self.hclk() }
}

