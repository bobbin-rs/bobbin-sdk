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
pub struct Hsi {}
impl Clock for Hsi { fn hz() -> Hz { Hz::from_num(8000000) } }

#[derive(Default)]
pub struct Lsi {}
impl Clock for Lsi { fn hz() -> Hz { Hz::from_num(40000) } }


pub trait ClockProvider : Default {
    type Osc: Clock;
    type Osc32: Clock;
    fn osc(&self) -> Hz { Self::Osc::hz() }
    fn osc32(&self) -> Hz { Self::Osc32::hz() }
    fn hsi(&self) -> Hz { Hz::from_num(8000000) }
    fn hse(&self) -> Hz { self.osc() }
    fn lsi(&self) -> Hz { Hz::from_num(40000) }
    fn lse(&self) -> Hz { self.osc32() }
    fn pllclk(&self) -> Hz { unimplemented!() }
    fn sysclk(&self) -> Hz { unimplemented!() }
    fn hclk(&self) -> Hz { unimplemented!() }
    fn systick(&self) -> Hz { unimplemented!() }
    fn fhclk(&self) -> Hz { unimplemented!() }
    fn pclk1(&self) -> Hz { unimplemented!() }
    fn pclk2(&self) -> Hz { unimplemented!() }
    fn tim_pclk1(&self) -> Hz { unimplemented!() }
    fn tim_pclk2(&self) -> Hz { unimplemented!() }
    fn i2c1(&self) -> Hz { unimplemented!() }
    fn i2c2(&self) -> Hz { unimplemented!() }
    fn i2c3(&self) -> Hz { unimplemented!() }
    fn i2s2(&self) -> Hz { unimplemented!() }
    fn i2s3(&self) -> Hz { unimplemented!() }
    fn usbclk(&self) -> Hz { unimplemented!() }
    fn usart1(&self) -> Hz { unimplemented!() }
    fn usart2(&self) -> Hz { unimplemented!() }
    fn usart3(&self) -> Hz { unimplemented!() }
    fn uart4(&self) -> Hz { unimplemented!() }
    fn uart5(&self) -> Hz { unimplemented!() }
    fn tim1(&self) -> Hz { unimplemented!() }
    fn tim2(&self) -> Hz { unimplemented!() }
    fn tim3(&self) -> Hz { unimplemented!() }
    fn tim4(&self) -> Hz { unimplemented!() }
    fn tim8(&self) -> Hz { unimplemented!() }
    fn tim15(&self) -> Hz { unimplemented!() }
    fn tim16(&self) -> Hz { unimplemented!() }
    fn tim17(&self) -> Hz { unimplemented!() }
    fn tim20(&self) -> Hz { unimplemented!() }
    fn rtcclk(&self) -> Hz { unimplemented!() }
    fn adc12(&self) -> Hz { unimplemented!() }
    fn adc34(&self) -> Hz { unimplemented!() }
}

impl<CP> ClockFor<::iwdg::Iwdg> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::iwdg::Iwdg) -> Hz { self.lsi() }
}

impl<CP> ClockFor<::wwdg::Wwdg> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::wwdg::Wwdg) -> Hz { self.pclk1() }
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

impl<CP> ClockFor<::usart::Usart1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::usart::Usart1) -> Hz { self.usart1() }
}

impl<CP> ClockFor<::usart::Usart2> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::usart::Usart2) -> Hz { self.usart2() }
}

impl<CP> ClockFor<::usart::Usart3> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::usart::Usart3) -> Hz { self.usart3() }
}

impl<CP> ClockFor<::usart::Uart4> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::usart::Uart4) -> Hz { self.uart4() }
}

impl<CP> ClockFor<::usart::Uart5> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::usart::Uart5) -> Hz { self.uart5() }
}

impl<CP> ClockFor<::i2c::I2c1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::i2c::I2c1) -> Hz { self.i2c1() }
}

impl<CP> ClockFor<::i2c::I2c2> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::i2c::I2c2) -> Hz { self.i2c2() }
}

impl<CP> ClockFor<::i2c::I2c3> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::i2c::I2c3) -> Hz { self.i2c3() }
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
    fn clock_for(&self, _: ::adc::Adc1) -> Hz { self.adc12() }
}

impl<CP> ClockFor<::adc::Adc2> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::adc::Adc2) -> Hz { self.adc12() }
}

impl<CP> ClockFor<::adc::Adc3> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::adc::Adc3) -> Hz { self.adc34() }
}

impl<CP> ClockFor<::adc::Adc4> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::adc::Adc4) -> Hz { self.adc34() }
}

impl<CP> ClockFor<::dac::Dac> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::dac::Dac) -> Hz { self.pclk1() }
}

impl<CP> ClockFor<::tim_bas::Tim6> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_bas::Tim6) -> Hz { self.tim_pclk1() }
}

impl<CP> ClockFor<::tim_bas::Tim7> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_bas::Tim7) -> Hz { self.tim_pclk1() }
}

impl<CP> ClockFor<::tim_gen::Tim2> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_gen::Tim2) -> Hz { self.tim2() }
}

impl<CP> ClockFor<::tim_gen::Tim3> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_gen::Tim3) -> Hz { self.tim3() }
}

impl<CP> ClockFor<::tim_gen::Tim4> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_gen::Tim4) -> Hz { self.tim4() }
}

impl<CP> ClockFor<::tim_gen::Tim15> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_gen::Tim15) -> Hz { self.tim15() }
}

impl<CP> ClockFor<::tim_gen::Tim16> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_gen::Tim16) -> Hz { self.tim16() }
}

impl<CP> ClockFor<::tim_gen::Tim17> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_gen::Tim17) -> Hz { self.tim17() }
}

impl<CP> ClockFor<::tim_adv::Tim1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_adv::Tim1) -> Hz { self.tim1() }
}

impl<CP> ClockFor<::tim_adv::Tim8> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_adv::Tim8) -> Hz { self.tim8() }
}

impl<CP> ClockFor<::tim_adv::Tim20> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tim_adv::Tim20) -> Hz { self.tim20() }
}

