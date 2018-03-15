#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::rcc::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( RCC, Rcc, RCC_PERIPH, RccPeriph, 0x40023800, 0x00);

impl En for super::dma::Dma2 {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb1enr().dma2en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb1enr(|r| r.set_dma2en(value)); }
}

impl En for super::dma::Dma1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb1enr().dma1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb1enr(|r| r.set_dma1en(value)); }
}

impl En for super::crc::Crc {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb1enr().crcen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb1enr(|r| r.set_crcen(value)); }
}

impl En for super::gpio::Gpiok {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb1enr().gpioken() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb1enr(|r| r.set_gpioken(value)); }
}

impl En for super::gpio::Gpioj {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb1enr().gpiojen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb1enr(|r| r.set_gpiojen(value)); }
}

impl En for super::gpio::Gpioi {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb1enr().gpioien() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb1enr(|r| r.set_gpioien(value)); }
}

impl En for super::gpio::Gpioh {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb1enr().gpiohen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb1enr(|r| r.set_gpiohen(value)); }
}

impl En for super::gpio::Gpiog {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb1enr().gpiogen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb1enr(|r| r.set_gpiogen(value)); }
}

impl En for super::gpio::Gpiof {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb1enr().gpiofen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb1enr(|r| r.set_gpiofen(value)); }
}

impl En for super::gpio::Gpioe {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb1enr().gpioeen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb1enr(|r| r.set_gpioeen(value)); }
}

impl En for super::gpio::Gpiod {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb1enr().gpioden() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb1enr(|r| r.set_gpioden(value)); }
}

impl En for super::gpio::Gpioc {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb1enr().gpiocen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb1enr(|r| r.set_gpiocen(value)); }
}

impl En for super::gpio::Gpiob {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb1enr().gpioben() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb1enr(|r| r.set_gpioben(value)); }
}

impl En for super::gpio::Gpioa {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb1enr().gpioaen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb1enr(|r| r.set_gpioaen(value)); }
}

impl En for super::usart::Uart8 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().uart8enr() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_uart8enr(value)); }
}

impl En for super::usart::Uart7 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().uart7enr() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_uart7enr(value)); }
}

impl En for super::i2c::I2c3 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().i2c3en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_i2c3en(value)); }
}

impl En for super::i2c::I2c2 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().i2c2en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_i2c2en(value)); }
}

impl En for super::i2c::I2c1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().i2c1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_i2c1en(value)); }
}

impl En for super::usart::Uart5 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().uart5en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_uart5en(value)); }
}

impl En for super::usart::Uart4 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().uart4en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_uart4en(value)); }
}

impl En for super::usart::Usart3 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().usart3en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_usart3en(value)); }
}

impl En for super::usart::Usart2 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().usart2en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_usart2en(value)); }
}

impl En for super::spi::Spi3 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().spi3en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_spi3en(value)); }
}

impl En for super::spi::Spi2 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().spi2en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_spi2en(value)); }
}

impl En for super::wwdg::Wwdg {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().wwdgen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_wwdgen(value)); }
}

impl En for super::tim_gen::Tim14 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().tim14en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_tim14en(value)); }
}

impl En for super::tim_gen::Tim13 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().tim13en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_tim13en(value)); }
}

impl En for super::tim_gen::Tim12 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().tim12en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_tim12en(value)); }
}

impl En for super::tim_bas::Tim7 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().tim7en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_tim7en(value)); }
}

impl En for super::tim_bas::Tim6 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().tim6en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_tim6en(value)); }
}

impl En for super::tim_gen::Tim5 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().tim5en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_tim5en(value)); }
}

impl En for super::tim_gen::Tim4 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().tim4en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_tim4en(value)); }
}

impl En for super::tim_gen::Tim3 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().tim3en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_tim3en(value)); }
}

impl En for super::tim_gen::Tim2 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().tim2en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_tim2en(value)); }
}

impl En for super::spi::Spi6 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().spi6en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_spi6en(value)); }
}

impl En for super::spi::Spi5 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().spi5en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_spi5en(value)); }
}

impl En for super::tim_gen::Tim11 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().tim11en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_tim11en(value)); }
}

impl En for super::tim_gen::Tim10 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().tim10en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_tim10en(value)); }
}

impl En for super::tim_gen::Tim9 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().tim9en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_tim9en(value)); }
}

impl En for super::syscfg::Syscfg {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().syscfgen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_syscfgen(value)); }
}

impl En for super::spi::Spi4 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().spi4en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_spi4en(value)); }
}

impl En for super::spi::Spi1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().spi1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_spi1en(value)); }
}

impl En for super::adc::Adc3 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().adc3en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_adc3en(value)); }
}

impl En for super::adc::Adc2 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().adc2en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_adc2en(value)); }
}

impl En for super::adc::Adc1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().adc1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_adc1en(value)); }
}

impl En for super::usart::Usart6 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().usart6en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_usart6en(value)); }
}

impl En for super::usart::Usart1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().usart1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_usart1en(value)); }
}

impl En for super::tim_adv::Tim8 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().tim8en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_tim8en(value)); }
}

impl En for super::tim_adv::Tim1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().tim1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_tim1en(value)); }
}


