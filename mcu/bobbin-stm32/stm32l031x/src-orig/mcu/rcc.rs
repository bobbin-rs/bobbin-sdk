#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::rcc::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( RCC, Rcc, RCC_PERIPH, RccPeriph, 0x40021000, 0x02);

impl Rst for super::gpio::Gpioh {
    #[inline] fn rst(&self) -> bits::U1 { RCC.ioprstr().iophrst() }
    #[inline] fn set_rst<V: Into<bits::U1>>(&self, value: V) { RCC.with_ioprstr(|r| r.set_iophrst(value)); }
}

impl Rst for super::gpio::Gpioc {
    #[inline] fn rst(&self) -> bits::U1 { RCC.ioprstr().iopcrst() }
    #[inline] fn set_rst<V: Into<bits::U1>>(&self, value: V) { RCC.with_ioprstr(|r| r.set_iopcrst(value)); }
}

impl Rst for super::gpio::Gpiob {
    #[inline] fn rst(&self) -> bits::U1 { RCC.ioprstr().iopbrst() }
    #[inline] fn set_rst<V: Into<bits::U1>>(&self, value: V) { RCC.with_ioprstr(|r| r.set_iopbrst(value)); }
}

impl Rst for super::gpio::Gpioa {
    #[inline] fn rst(&self) -> bits::U1 { RCC.ioprstr().ioparst() }
    #[inline] fn set_rst<V: Into<bits::U1>>(&self, value: V) { RCC.with_ioprstr(|r| r.set_ioparst(value)); }
}

impl Rst for super::crc::Crc {
    #[inline] fn rst(&self) -> bits::U1 { RCC.ahbrstr().crcrst() }
    #[inline] fn set_rst<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahbrstr(|r| r.set_crcrst(value)); }
}

impl Rst for super::dma::Dma1 {
    #[inline] fn rst(&self) -> bits::U1 { RCC.ahbrstr().dmarst() }
    #[inline] fn set_rst<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahbrstr(|r| r.set_dmarst(value)); }
}

impl Rst for super::spi::Spi1 {
    #[inline] fn rst(&self) -> bits::U1 { RCC.apb2rstr().spi1rst() }
    #[inline] fn set_rst<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2rstr(|r| r.set_spi1rst(value)); }
}

impl Rst for super::tim_gen::Tim22 {
    #[inline] fn rst(&self) -> bits::U1 { RCC.apb2rstr().tim22rst() }
    #[inline] fn set_rst<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2rstr(|r| r.set_tim22rst(value)); }
}

impl Rst for super::tim_gen::Tim21 {
    #[inline] fn rst(&self) -> bits::U1 { RCC.apb2rstr().tim21rst() }
    #[inline] fn set_rst<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2rstr(|r| r.set_tim21rst(value)); }
}

impl Rst for super::syscfg::Syscfg {
    #[inline] fn rst(&self) -> bits::U1 { RCC.apb2rstr().syscfgrst() }
    #[inline] fn set_rst<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2rstr(|r| r.set_syscfgrst(value)); }
}

impl Rst for super::lptim::Lptim {
    #[inline] fn rst(&self) -> bits::U1 { RCC.apb1rstr().lptim1rst() }
    #[inline] fn set_rst<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1rstr(|r| r.set_lptim1rst(value)); }
}

impl Rst for super::pwr::Pwr {
    #[inline] fn rst(&self) -> bits::U1 { RCC.apb1rstr().pwrrst() }
    #[inline] fn set_rst<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1rstr(|r| r.set_pwrrst(value)); }
}

impl Rst for super::i2c::I2c1 {
    #[inline] fn rst(&self) -> bits::U1 { RCC.apb1rstr().i2c1rst() }
    #[inline] fn set_rst<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1rstr(|r| r.set_i2c1rst(value)); }
}

impl Rst for super::usart::Usart2 {
    #[inline] fn rst(&self) -> bits::U1 { RCC.apb1rstr().usart2rst() }
    #[inline] fn set_rst<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1rstr(|r| r.set_usart2rst(value)); }
}

impl Rst for super::wwdg::Wwdg {
    #[inline] fn rst(&self) -> bits::U1 { RCC.apb1rstr().wwdrst() }
    #[inline] fn set_rst<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1rstr(|r| r.set_wwdrst(value)); }
}

impl Rst for super::tim_gen::Tim2 {
    #[inline] fn rst(&self) -> bits::U1 { RCC.apb1rstr().tim2rst() }
    #[inline] fn set_rst<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1rstr(|r| r.set_tim2rst(value)); }
}

impl En for super::gpio::Gpioa {
    #[inline] fn en(&self) -> bits::U1 { RCC.iopenr().iopaen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_iopenr(|r| r.set_iopaen(value)); }
}

impl En for super::gpio::Gpiob {
    #[inline] fn en(&self) -> bits::U1 { RCC.iopenr().iopben() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_iopenr(|r| r.set_iopben(value)); }
}

impl En for super::gpio::Gpioc {
    #[inline] fn en(&self) -> bits::U1 { RCC.iopenr().iopcen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_iopenr(|r| r.set_iopcen(value)); }
}

impl En for super::gpio::Gpioh {
    #[inline] fn en(&self) -> bits::U1 { RCC.iopenr().iophen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_iopenr(|r| r.set_iophen(value)); }
}

impl En for super::crc::Crc {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahbenr().crcen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahbenr(|r| r.set_crcen(value)); }
}

impl En for super::dma::Dma1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahbenr().dmaen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahbenr(|r| r.set_dmaen(value)); }
}

impl En for super::spi::Spi1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().spi1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_spi1en(value)); }
}

impl En for super::adc::Adc1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().adcen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_adcen(value)); }
}

impl En for super::tim_gen::Tim22 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().tim22en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_tim22en(value)); }
}

impl En for super::tim_gen::Tim21 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().tim21en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_tim21en(value)); }
}

impl En for super::syscfg::Syscfg {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().syscfgen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_syscfgen(value)); }
}

impl En for super::lptim::Lptim {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().lptim1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_lptim1en(value)); }
}

impl En for super::pwr::Pwr {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().pwren() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_pwren(value)); }
}

impl En for super::i2c::I2c1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().i2c1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_i2c1en(value)); }
}

impl En for super::lpuart::Lpuart1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().lpuart1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_lpuart1en(value)); }
}

impl En for super::usart::Usart2 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().usart2en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_usart2en(value)); }
}

impl En for super::wwdg::Wwdg {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().wwdgen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_wwdgen(value)); }
}

impl En for super::tim_gen::Tim2 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr().tim2en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr(|r| r.set_tim2en(value)); }
}

impl Sel for super::lptim::Lptim {
    #[inline] fn sel(&self) -> bits::U2 { RCC.ccipr().lptim1sel() }
    #[inline] fn set_sel<V: Into<bits::U2>>(&self, value: V) { RCC.with_ccipr(|r| r.set_lptim1sel(value)); }
}


