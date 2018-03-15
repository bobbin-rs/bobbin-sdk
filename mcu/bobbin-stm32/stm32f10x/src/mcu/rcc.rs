#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::rcc::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( RCC, Rcc, RCC_PERIPH, RccPeriph, 0x40021000, 0x00);

impl En for super::dma::Dma1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahbenr().dma1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahbenr(|r| r.set_dma1en(value)); }
}

impl En for super::dma::Dma2 {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahbenr().dma2en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahbenr(|r| r.set_dma2en(value)); }
}

impl En for super::crc::Crc {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahbenr().crcen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahbenr(|r| r.set_crcen(value)); }
}

impl En for super::afio::Afio {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().afioen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_afioen(value)); }
}

impl En for super::gpio::Gpioa {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().iopaen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_iopaen(value)); }
}

impl En for super::gpio::Gpiob {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().iopben() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_iopben(value)); }
}

impl En for super::gpio::Gpioc {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().iopcen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_iopcen(value)); }
}

impl En for super::gpio::Gpiod {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().iopden() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_iopden(value)); }
}

impl En for super::gpio::Gpioe {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().iopeen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_iopeen(value)); }
}

impl En for super::gpio::Gpiof {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().iopfen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_iopfen(value)); }
}

impl En for super::gpio::Gpiog {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().iopgen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_iopgen(value)); }
}

impl En for super::adc::Adc1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().adc1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_adc1en(value)); }
}

impl En for super::adc::Adc2 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().adc2en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_adc2en(value)); }
}

impl En for super::tim_adv::Tim1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().tim1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_tim1en(value)); }
}

impl En for super::spi::Spi1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().spi1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_spi1en(value)); }
}

impl En for super::usart::Usart1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().usart1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_usart1en(value)); }
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


