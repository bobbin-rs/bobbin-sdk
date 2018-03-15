#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::rcc::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( RCC, Rcc, RCC_PERIPH, RccPeriph, 0x40021000, 0x00);

impl En for super::tsc::Tsc {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb1enr().tscen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb1enr(|r| r.set_tscen(value)); }
}

impl En for super::crc::Crc {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb1enr().crcen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb1enr(|r| r.set_crcen(value)); }
}

impl En for super::flash::Flash {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb1enr().flashen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb1enr(|r| r.set_flashen(value)); }
}

impl En for super::dma::Dma2 {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb1enr().dma2en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb1enr(|r| r.set_dma2en(value)); }
}

impl En for super::dma::Dma1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb1enr().dma1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb1enr(|r| r.set_dma1en(value)); }
}

impl En for super::rng::Rng {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb2enr().rngen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb2enr(|r| r.set_rngen(value)); }
}

impl En for super::aes::Aes {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb2enr().aesen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb2enr(|r| r.set_aesen(value)); }
}

impl En for super::adc::Adc {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb2enr().adcen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb2enr(|r| r.set_adcen(value)); }
}

impl En for super::gpio::Gpioh {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb2enr().gpiohen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb2enr(|r| r.set_gpiohen(value)); }
}

impl En for super::gpio::Gpioe {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb2enr().gpioeen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb2enr(|r| r.set_gpioeen(value)); }
}

impl En for super::gpio::Gpiod {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb2enr().gpioden() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb2enr(|r| r.set_gpioden(value)); }
}

impl En for super::gpio::Gpioc {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb2enr().gpiocen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb2enr(|r| r.set_gpiocen(value)); }
}

impl En for super::gpio::Gpiob {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb2enr().gpioben() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb2enr(|r| r.set_gpioben(value)); }
}

impl En for super::gpio::Gpioa {
    #[inline] fn en(&self) -> bits::U1 { RCC.ahb2enr().gpioaen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_ahb2enr(|r| r.set_gpioaen(value)); }
}

impl En for super::lptim::Lptim1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr1().lptim1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr1(|r| r.set_lptim1en(value)); }
}

impl En for super::opamp::Opamp {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr1().opampen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr1(|r| r.set_opampen(value)); }
}

impl En for super::dac::Dac1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr1().dac1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr1(|r| r.set_dac1en(value)); }
}

impl En for super::pwr::Pwr {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr1().pwren() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr1(|r| r.set_pwren(value)); }
}

impl En for super::can::Can1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr1().can1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr1(|r| r.set_can1en(value)); }
}

impl En for super::i2c::I2c3 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr1().i2c3en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr1(|r| r.set_i2c3en(value)); }
}

impl En for super::i2c::I2c1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr1().i2c1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr1(|r| r.set_i2c1en(value)); }
}

impl En for super::usart::Usart3 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr1().usart3en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr1(|r| r.set_usart3en(value)); }
}

impl En for super::usart::Usart2 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr1().usart2en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr1(|r| r.set_usart2en(value)); }
}

impl En for super::spi::Spi3 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr1().spi3en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr1(|r| r.set_spi3en(value)); }
}

impl En for super::spi::Spi2 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr1().spi2en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr1(|r| r.set_spi2en(value)); }
}

impl En for super::wwdg::Wwdg {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr1().wwdgen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr1(|r| r.set_wwdgen(value)); }
}

impl En for super::lcd::Lcd {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr1().lcden() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr1(|r| r.set_lcden(value)); }
}

impl En for super::tim_bas::Tim7 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr1().tim7en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr1(|r| r.set_tim7en(value)); }
}

impl En for super::tim_bas::Tim6 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr1().tim6en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr1(|r| r.set_tim6en(value)); }
}

impl En for super::tim_gen::Tim2 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr1().tim2en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr1(|r| r.set_tim2en(value)); }
}

impl En for super::crs::Crs {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr1().crsen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr1(|r| r.set_crsen(value)); }
}

impl En for super::tim_gen::Tim3 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr1().tim3en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr1(|r| r.set_tim3en(value)); }
}

impl En for super::usart::Uart4 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr1().uart4en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr1(|r| r.set_uart4en(value)); }
}

impl En for super::i2c::I2c2 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr1().i2c2en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr1(|r| r.set_i2c2en(value)); }
}

impl En for super::lptim::Lptim2 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr2().lptim2en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr2(|r| r.set_lptim2en(value)); }
}

impl En for super::swpmi::Swpmi1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr2().swpmi1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr2(|r| r.set_swpmi1en(value)); }
}

impl En for super::lpuart::Lpuart1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr2().lpuart1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr2(|r| r.set_lpuart1en(value)); }
}

impl En for super::i2c::I2c4 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb1enr2().i2c4en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb1enr2(|r| r.set_i2c4en(value)); }
}

impl En for super::dfsdm::Dfsdm {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().dfsdmen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_dfsdmen(value)); }
}

impl En for super::sai::Sai1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().sai1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_sai1en(value)); }
}

impl En for super::tim_gen::Tim16 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().tim16en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_tim16en(value)); }
}

impl En for super::tim_gen::Tim15 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().tim15en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_tim15en(value)); }
}

impl En for super::usart::Usart1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().usart1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_usart1en(value)); }
}

impl En for super::spi::Spi1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().spi1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_spi1en(value)); }
}

impl En for super::tim_adv::Tim1 {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().tim1en() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_tim1en(value)); }
}

impl En for super::firewall::Firewall {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().firewallen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_firewallen(value)); }
}

impl En for super::syscfg::Syscfg {
    #[inline] fn en(&self) -> bits::U1 { RCC.apb2enr().syscfgen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_apb2enr(|r| r.set_syscfgen(value)); }
}

impl Sel for super::adc::Adc {
    #[inline] fn sel(&self) -> bits::U2 { RCC.ccipr().adcsel() }
    #[inline] fn set_sel<V: Into<bits::U2>>(&self, value: V) { RCC.with_ccipr(|r| r.set_adcsel(value)); }
}

impl Sel for super::sai::Sai1 {
    #[inline] fn sel(&self) -> bits::U2 { RCC.ccipr().sai1sel() }
    #[inline] fn set_sel<V: Into<bits::U2>>(&self, value: V) { RCC.with_ccipr(|r| r.set_sai1sel(value)); }
}

impl Sel for super::lptim::Lptim2 {
    #[inline] fn sel(&self) -> bits::U2 { RCC.ccipr().lptim2sel() }
    #[inline] fn set_sel<V: Into<bits::U2>>(&self, value: V) { RCC.with_ccipr(|r| r.set_lptim2sel(value)); }
}

impl Sel for super::lptim::Lptim1 {
    #[inline] fn sel(&self) -> bits::U2 { RCC.ccipr().lptim1sel() }
    #[inline] fn set_sel<V: Into<bits::U2>>(&self, value: V) { RCC.with_ccipr(|r| r.set_lptim1sel(value)); }
}

impl Sel for super::i2c::I2c3 {
    #[inline] fn sel(&self) -> bits::U2 { RCC.ccipr().i2c3sel() }
    #[inline] fn set_sel<V: Into<bits::U2>>(&self, value: V) { RCC.with_ccipr(|r| r.set_i2c3sel(value)); }
}

impl Sel for super::i2c::I2c2 {
    #[inline] fn sel(&self) -> bits::U2 { RCC.ccipr().i2c2sel() }
    #[inline] fn set_sel<V: Into<bits::U2>>(&self, value: V) { RCC.with_ccipr(|r| r.set_i2c2sel(value)); }
}

impl Sel for super::i2c::I2c1 {
    #[inline] fn sel(&self) -> bits::U2 { RCC.ccipr().i2c1sel() }
    #[inline] fn set_sel<V: Into<bits::U2>>(&self, value: V) { RCC.with_ccipr(|r| r.set_i2c1sel(value)); }
}

impl Sel for super::lpuart::Lpuart1 {
    #[inline] fn sel(&self) -> bits::U2 { RCC.ccipr().lpuart1sel() }
    #[inline] fn set_sel<V: Into<bits::U2>>(&self, value: V) { RCC.with_ccipr(|r| r.set_lpuart1sel(value)); }
}

impl Sel for super::usart::Uart4 {
    #[inline] fn sel(&self) -> bits::U2 { RCC.ccipr().uart4sel() }
    #[inline] fn set_sel<V: Into<bits::U2>>(&self, value: V) { RCC.with_ccipr(|r| r.set_uart4sel(value)); }
}

impl Sel for super::usart::Usart3 {
    #[inline] fn sel(&self) -> bits::U2 { RCC.ccipr().usart3sel() }
    #[inline] fn set_sel<V: Into<bits::U2>>(&self, value: V) { RCC.with_ccipr(|r| r.set_usart3sel(value)); }
}

impl Sel for super::usart::Usart2 {
    #[inline] fn sel(&self) -> bits::U2 { RCC.ccipr().usart2sel() }
    #[inline] fn set_sel<V: Into<bits::U2>>(&self, value: V) { RCC.with_ccipr(|r| r.set_usart2sel(value)); }
}

impl Sel for super::usart::Usart1 {
    #[inline] fn sel(&self) -> bits::U2 { RCC.ccipr().usart1sel() }
    #[inline] fn set_sel<V: Into<bits::U2>>(&self, value: V) { RCC.with_ccipr(|r| r.set_usart1sel(value)); }
}

impl En for super::rtc::Rtc {
    #[inline] fn en(&self) -> bits::U1 { RCC.bdcr().rtcen() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { RCC.with_bdcr(|r| r.set_rtcen(value)); }
}

impl Sel for super::rtc::Rtc {
    #[inline] fn sel(&self) -> bits::U2 { RCC.bdcr().rtcsel() }
    #[inline] fn set_sel<V: Into<bits::U2>>(&self, value: V) { RCC.with_bdcr(|r| r.set_rtcsel(value)); }
}


