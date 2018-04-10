pub use ::bobbin_common::*;
pub use ::bobbin_common::tree::*;
pub use ::hz::Hz;

pub struct ClockTree<T>(T);


// Define Global Clocks

pub struct Hsi {}
impl Clock for Hsi { fn hz() -> Hz { Hz::from_num(16000000) } }

pub struct Lsi {}
impl Clock for Lsi { fn hz() -> Hz { Hz::from_num(32000) } }


pub trait Clocks {
    type Osc: Clock;
    type Osc32: Clock;
    fn osc() -> Hz { Self::Osc::hz() }
    fn osc32() -> Hz { Self::Osc32::hz() }
    fn hsi() -> Hz { Hz::from_num(16000000) }
    fn hse() -> Hz { Self::osc() }
    fn lsi() -> Hz { Hz::from_num(32000) }
    fn lse() -> Hz { Self::osc32() }
    fn pllclk() -> Hz { Hz::from_num(0) }
    fn sysclk() -> Hz { Hz::from_num(0) }
    fn hclk() -> Hz { Hz::from_num(0) }
    fn systick() -> Hz { Hz::from_num(0) }
    fn fhclk() -> Hz { Hz::from_num(0) }
    fn pclk1() -> Hz { Hz::from_num(0) }
    fn pclk2() -> Hz { Hz::from_num(0) }
    fn tim_pclk1() -> Hz { Hz::from_num(0) }
    fn tim_pclk2() -> Hz { Hz::from_num(0) }
    fn i2c1() -> Hz { Hz::from_num(0) }
    fn i2c2() -> Hz { Hz::from_num(0) }
    fn i2c3() -> Hz { Hz::from_num(0) }
    fn i2s2() -> Hz { Hz::from_num(0) }
    fn i2s3() -> Hz { Hz::from_num(0) }
    fn usbclk() -> Hz { Hz::from_num(0) }
    fn usart1() -> Hz { Hz::from_num(0) }
    fn usart2() -> Hz { Hz::from_num(0) }
    fn usart3() -> Hz { Hz::from_num(0) }
    fn uart4() -> Hz { Hz::from_num(0) }
    fn uart5() -> Hz { Hz::from_num(0) }
    fn tim1() -> Hz { Hz::from_num(0) }
    fn tim2() -> Hz { Hz::from_num(0) }
    fn tim3() -> Hz { Hz::from_num(0) }
    fn tim4() -> Hz { Hz::from_num(0) }
    fn tim8() -> Hz { Hz::from_num(0) }
    fn tim15() -> Hz { Hz::from_num(0) }
    fn tim16() -> Hz { Hz::from_num(0) }
    fn tim17() -> Hz { Hz::from_num(0) }
    fn tim20() -> Hz { Hz::from_num(0) }
    fn rtcclk() -> Hz { Hz::from_num(0) }
    fn adc12() -> Hz { Hz::from_num(0) }
    fn adc34() -> Hz { Hz::from_num(0) }
}

impl<T> ClockFor<::iwdg::Iwdg> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::iwdg::Iwdg) -> Hz { T::lsi() }
}

impl<T> ClockFor<::wwdg::Wwdg> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::wwdg::Wwdg) -> Hz { T::pclk1() }
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

impl<T> ClockFor<::gpio::Gpiof> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::gpio::Gpiof) -> Hz { T::hclk() }
}

impl<T> ClockFor<::gpio::Gpiog> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::gpio::Gpiog) -> Hz { T::hclk() }
}

impl<T> ClockFor<::gpio::Gpioh> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::gpio::Gpioh) -> Hz { T::hclk() }
}

impl<T> ClockFor<::usart::Usart1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::usart::Usart1) -> Hz { T::usart1() }
}

impl<T> ClockFor<::usart::Usart2> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::usart::Usart2) -> Hz { T::usart2() }
}

impl<T> ClockFor<::usart::Usart3> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::usart::Usart3) -> Hz { T::usart3() }
}

impl<T> ClockFor<::usart::Uart4> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::usart::Uart4) -> Hz { T::uart4() }
}

impl<T> ClockFor<::usart::Uart5> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::usart::Uart5) -> Hz { T::uart5() }
}

impl<T> ClockFor<::i2c::I2c1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::i2c::I2c1) -> Hz { T::i2c1() }
}

impl<T> ClockFor<::i2c::I2c2> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::i2c::I2c2) -> Hz { T::i2c2() }
}

impl<T> ClockFor<::i2c::I2c3> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::i2c::I2c3) -> Hz { T::i2c3() }
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
    fn clock_for(_: ::adc::Adc1) -> Hz { T::adc12() }
}

impl<T> ClockFor<::adc::Adc2> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::adc::Adc2) -> Hz { T::adc12() }
}

impl<T> ClockFor<::adc::Adc3> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::adc::Adc3) -> Hz { T::adc34() }
}

impl<T> ClockFor<::adc::Adc4> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::adc::Adc4) -> Hz { T::adc34() }
}

impl<T> ClockFor<::dac::Dac> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::dac::Dac) -> Hz { T::pclk1() }
}

impl<T> ClockFor<::tim_bas::Tim6> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_bas::Tim6) -> Hz { T::tim_pclk1() }
}

impl<T> ClockFor<::tim_bas::Tim7> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_bas::Tim7) -> Hz { T::tim_pclk1() }
}

impl<T> ClockFor<::tim_gen::Tim2> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_gen::Tim2) -> Hz { T::tim2() }
}

impl<T> ClockFor<::tim_gen::Tim3> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_gen::Tim3) -> Hz { T::tim3() }
}

impl<T> ClockFor<::tim_gen::Tim4> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_gen::Tim4) -> Hz { T::tim4() }
}

impl<T> ClockFor<::tim_gen::Tim15> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_gen::Tim15) -> Hz { T::tim15() }
}

impl<T> ClockFor<::tim_gen::Tim16> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_gen::Tim16) -> Hz { T::tim16() }
}

impl<T> ClockFor<::tim_gen::Tim17> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_gen::Tim17) -> Hz { T::tim17() }
}

impl<T> ClockFor<::tim_adv::Tim1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_adv::Tim1) -> Hz { T::tim1() }
}

impl<T> ClockFor<::tim_adv::Tim8> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_adv::Tim8) -> Hz { T::tim8() }
}

impl<T> ClockFor<::tim_adv::Tim20> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_adv::Tim20) -> Hz { T::tim20() }
}

