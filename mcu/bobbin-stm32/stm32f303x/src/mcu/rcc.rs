#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::rcc::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( RCC, Rcc, RCC_PERIPH, RccPeriph, 0x40021000, 0x00);

impl En for super::dma::Dma1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahbenr().dmaen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahbenr(|r| r.set_dmaen(value)); }
}

impl En for super::dma::Dma2 {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahbenr().dma2en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahbenr(|r| r.set_dma2en(value)); }
}

impl En for super::crc::Crc {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahbenr().crcen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahbenr(|r| r.set_crcen(value)); }
}

impl En for super::gpio::Gpioh {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahbenr().iophen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahbenr(|r| r.set_iophen(value)); }
}

impl En for super::gpio::Gpioa {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahbenr().iopaen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahbenr(|r| r.set_iopaen(value)); }
}

impl En for super::gpio::Gpiob {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahbenr().iopben() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahbenr(|r| r.set_iopben(value)); }
}

impl En for super::gpio::Gpioc {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahbenr().iopcen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahbenr(|r| r.set_iopcen(value)); }
}

impl En for super::gpio::Gpiod {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahbenr().iopden() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahbenr(|r| r.set_iopden(value)); }
}

impl En for super::gpio::Gpioe {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahbenr().iopeen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahbenr(|r| r.set_iopeen(value)); }
}

impl En for super::gpio::Gpiof {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahbenr().iopfen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahbenr(|r| r.set_iopfen(value)); }
}

impl En for super::gpio::Gpiog {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahbenr().iopgen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahbenr(|r| r.set_iopgen(value)); }
}

impl En for super::adc::Adc1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahbenr().adc12en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahbenr(|r| r.set_adc12en(value)); }
}

impl En for super::adc::Adc2 {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahbenr().adc12en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahbenr(|r| r.set_adc12en(value)); }
}

impl En for super::adc::Adc3 {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahbenr().adc34en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahbenr(|r| r.set_adc34en(value)); }
}

impl En for super::adc::Adc4 {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahbenr().adc34en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahbenr(|r| r.set_adc34en(value)); }
}

impl En for super::syscfg::Syscfg {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().syscfgen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_syscfgen(value)); }
}

impl En for super::tim_adv::Tim1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().tim1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_tim1en(value)); }
}

impl En for super::spi::Spi1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().spi1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_spi1en(value)); }
}

impl En for super::tim_adv::Tim8 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().tim8en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_tim8en(value)); }
}

impl En for super::usart::Usart1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().usart1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_usart1en(value)); }
}

impl En for super::tim_gen::Tim15 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().tim15en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_tim15en(value)); }
}

impl En for super::tim_gen::Tim16 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().tim16en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_tim16en(value)); }
}

impl En for super::tim_gen::Tim17 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().tim17en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_tim17en(value)); }
}

impl En for super::tim_adv::Tim20 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().tim20en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_tim20en(value)); }
}

impl En for super::tim_gen::Tim2 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().tim2en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_tim2en(value)); }
}

impl En for super::tim_gen::Tim3 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().tim3en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_tim3en(value)); }
}

impl En for super::tim_gen::Tim4 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().tim4en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_tim4en(value)); }
}

impl En for super::tim_bas::Tim6 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().tim6en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_tim6en(value)); }
}

impl En for super::tim_bas::Tim7 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().tim7en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_tim7en(value)); }
}

impl En for super::wwdg::Wwdg {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().wwdgen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_wwdgen(value)); }
}

impl En for super::spi::Spi2 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().spi2en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_spi2en(value)); }
}

impl En for super::spi::Spi3 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().spi3en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_spi3en(value)); }
}

impl En for super::usart::Usart2 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().usart2en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_usart2en(value)); }
}

impl En for super::usart::Usart3 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().usart3en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_usart3en(value)); }
}

impl En for super::usart::Uart4 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().uart4en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_uart4en(value)); }
}

impl En for super::usart::Uart5 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().uart5en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_uart5en(value)); }
}

impl En for super::i2c::I2c1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().i2c1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_i2c1en(value)); }
}

impl En for super::i2c::I2c2 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().i2c2en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_i2c2en(value)); }
}

impl En for super::pwr::Pwr {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().pwren() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_pwren(value)); }
}

impl En for super::i2c::I2c3 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().i2c3en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_i2c3en(value)); }
}


