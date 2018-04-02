pub use ::bobbin_common::mcu::*;
use ::bobbin_common::owned::*;

pub mod flash;
pub mod pwr;
pub mod rcc;
pub mod syscfg;
pub mod c_adc;
pub mod dac;
pub mod rng;
pub mod iwdg;
pub mod wwdg;
pub mod crc;
pub mod rtc;
pub mod lptim;
pub mod exti;
pub mod dma;
pub mod i2c;
pub mod tim_adv;
pub mod tim_bas;
pub mod tim_gen;
pub mod gpio;
pub mod usart;
pub mod lpuart;
pub mod spi;
pub mod adc;
pub mod sig;
pub mod pin;
pub mod irq;

#[derive(Debug, Default)]
pub struct Stm32l432x {}

impl Mcu for Stm32l432x {
    fn id(&self) -> &'static str { "STM32L432x" }
}

impl Stm32l432x {
    pub fn flash(&self) -> Option<Owned<flash::Flash>> { flash::Flash::acquire() }
    pub fn pwr(&self) -> Option<Owned<pwr::Pwr>> { pwr::Pwr::acquire() }
    pub fn rcc(&self) -> Option<Owned<rcc::Rcc>> { rcc::Rcc::acquire() }
    pub fn syscfg(&self) -> Option<Owned<syscfg::Syscfg>> { syscfg::Syscfg::acquire() }
    pub fn c_adc(&self) -> Option<Owned<c_adc::CAdc>> { c_adc::CAdc::acquire() }
    pub fn dac1(&self) -> Option<Owned<dac::Dac1>> { dac::Dac1::acquire() }
        pub fn dac1_ch1(&self) -> Option<Owned<dac::Dac1Ch1>> { dac::Dac1Ch1::acquire() }
        pub fn dac1_ch2(&self) -> Option<Owned<dac::Dac1Ch2>> { dac::Dac1Ch2::acquire() }
    pub fn rng(&self) -> Option<Owned<rng::Rng>> { rng::Rng::acquire() }
    pub fn iwdg(&self) -> Option<Owned<iwdg::Iwdg>> { iwdg::Iwdg::acquire() }
    pub fn wwdg(&self) -> Option<Owned<wwdg::Wwdg>> { wwdg::Wwdg::acquire() }
    pub fn crc(&self) -> Option<Owned<crc::Crc>> { crc::Crc::acquire() }
    pub fn rtc(&self) -> Option<Owned<rtc::Rtc>> { rtc::Rtc::acquire() }
    pub fn lptim1(&self) -> Option<Owned<lptim::Lptim1>> { lptim::Lptim1::acquire() }
    pub fn lptim2(&self) -> Option<Owned<lptim::Lptim2>> { lptim::Lptim2::acquire() }
    pub fn exti(&self) -> Option<Owned<exti::Exti>> { exti::Exti::acquire() }
        pub fn exti_line0(&self) -> Option<Owned<exti::ExtiLine0>> { exti::ExtiLine0::acquire() }
        pub fn exti_line1(&self) -> Option<Owned<exti::ExtiLine1>> { exti::ExtiLine1::acquire() }
        pub fn exti_line2(&self) -> Option<Owned<exti::ExtiLine2>> { exti::ExtiLine2::acquire() }
        pub fn exti_line3(&self) -> Option<Owned<exti::ExtiLine3>> { exti::ExtiLine3::acquire() }
        pub fn exti_line4(&self) -> Option<Owned<exti::ExtiLine4>> { exti::ExtiLine4::acquire() }
        pub fn exti_line5(&self) -> Option<Owned<exti::ExtiLine5>> { exti::ExtiLine5::acquire() }
        pub fn exti_line6(&self) -> Option<Owned<exti::ExtiLine6>> { exti::ExtiLine6::acquire() }
        pub fn exti_line7(&self) -> Option<Owned<exti::ExtiLine7>> { exti::ExtiLine7::acquire() }
        pub fn exti_line8(&self) -> Option<Owned<exti::ExtiLine8>> { exti::ExtiLine8::acquire() }
        pub fn exti_line9(&self) -> Option<Owned<exti::ExtiLine9>> { exti::ExtiLine9::acquire() }
        pub fn exti_line10(&self) -> Option<Owned<exti::ExtiLine10>> { exti::ExtiLine10::acquire() }
        pub fn exti_line11(&self) -> Option<Owned<exti::ExtiLine11>> { exti::ExtiLine11::acquire() }
        pub fn exti_line12(&self) -> Option<Owned<exti::ExtiLine12>> { exti::ExtiLine12::acquire() }
        pub fn exti_line13(&self) -> Option<Owned<exti::ExtiLine13>> { exti::ExtiLine13::acquire() }
        pub fn exti_line14(&self) -> Option<Owned<exti::ExtiLine14>> { exti::ExtiLine14::acquire() }
        pub fn exti_line15(&self) -> Option<Owned<exti::ExtiLine15>> { exti::ExtiLine15::acquire() }
        pub fn exti_line16(&self) -> Option<Owned<exti::ExtiLine16>> { exti::ExtiLine16::acquire() }
        pub fn exti_line17(&self) -> Option<Owned<exti::ExtiLine17>> { exti::ExtiLine17::acquire() }
        pub fn exti_line18(&self) -> Option<Owned<exti::ExtiLine18>> { exti::ExtiLine18::acquire() }
        pub fn exti_line19(&self) -> Option<Owned<exti::ExtiLine19>> { exti::ExtiLine19::acquire() }
        pub fn exti_line20(&self) -> Option<Owned<exti::ExtiLine20>> { exti::ExtiLine20::acquire() }
        pub fn exti_line21(&self) -> Option<Owned<exti::ExtiLine21>> { exti::ExtiLine21::acquire() }
        pub fn exti_line22(&self) -> Option<Owned<exti::ExtiLine22>> { exti::ExtiLine22::acquire() }
        pub fn exti_line23(&self) -> Option<Owned<exti::ExtiLine23>> { exti::ExtiLine23::acquire() }
        pub fn exti_line24(&self) -> Option<Owned<exti::ExtiLine24>> { exti::ExtiLine24::acquire() }
        pub fn exti_line25(&self) -> Option<Owned<exti::ExtiLine25>> { exti::ExtiLine25::acquire() }
        pub fn exti_line26(&self) -> Option<Owned<exti::ExtiLine26>> { exti::ExtiLine26::acquire() }
        pub fn exti_line28(&self) -> Option<Owned<exti::ExtiLine28>> { exti::ExtiLine28::acquire() }
        pub fn exti_line29(&self) -> Option<Owned<exti::ExtiLine29>> { exti::ExtiLine29::acquire() }
    pub fn dma1(&self) -> Option<Owned<dma::Dma1>> { dma::Dma1::acquire() }
        pub fn dma1_ch1(&self) -> Option<Owned<dma::Dma1Ch1>> { dma::Dma1Ch1::acquire() }
        pub fn dma1_ch2(&self) -> Option<Owned<dma::Dma1Ch2>> { dma::Dma1Ch2::acquire() }
        pub fn dma1_ch3(&self) -> Option<Owned<dma::Dma1Ch3>> { dma::Dma1Ch3::acquire() }
        pub fn dma1_ch4(&self) -> Option<Owned<dma::Dma1Ch4>> { dma::Dma1Ch4::acquire() }
        pub fn dma1_ch5(&self) -> Option<Owned<dma::Dma1Ch5>> { dma::Dma1Ch5::acquire() }
        pub fn dma1_ch6(&self) -> Option<Owned<dma::Dma1Ch6>> { dma::Dma1Ch6::acquire() }
        pub fn dma1_ch7(&self) -> Option<Owned<dma::Dma1Ch7>> { dma::Dma1Ch7::acquire() }
    pub fn dma2(&self) -> Option<Owned<dma::Dma2>> { dma::Dma2::acquire() }
        pub fn dma2_ch1(&self) -> Option<Owned<dma::Dma2Ch1>> { dma::Dma2Ch1::acquire() }
        pub fn dma2_ch2(&self) -> Option<Owned<dma::Dma2Ch2>> { dma::Dma2Ch2::acquire() }
        pub fn dma2_ch3(&self) -> Option<Owned<dma::Dma2Ch3>> { dma::Dma2Ch3::acquire() }
        pub fn dma2_ch4(&self) -> Option<Owned<dma::Dma2Ch4>> { dma::Dma2Ch4::acquire() }
        pub fn dma2_ch5(&self) -> Option<Owned<dma::Dma2Ch5>> { dma::Dma2Ch5::acquire() }
        pub fn dma2_ch6(&self) -> Option<Owned<dma::Dma2Ch6>> { dma::Dma2Ch6::acquire() }
        pub fn dma2_ch7(&self) -> Option<Owned<dma::Dma2Ch7>> { dma::Dma2Ch7::acquire() }
    pub fn i2c1(&self) -> Option<Owned<i2c::I2c1>> { i2c::I2c1::acquire() }
    pub fn i2c2(&self) -> Option<Owned<i2c::I2c2>> { i2c::I2c2::acquire() }
    pub fn i2c3(&self) -> Option<Owned<i2c::I2c3>> { i2c::I2c3::acquire() }
    pub fn tim1(&self) -> Option<Owned<tim_adv::Tim1>> { tim_adv::Tim1::acquire() }
    pub fn tim8(&self) -> Option<Owned<tim_adv::Tim8>> { tim_adv::Tim8::acquire() }
    pub fn tim6(&self) -> Option<Owned<tim_bas::Tim6>> { tim_bas::Tim6::acquire() }
    pub fn tim7(&self) -> Option<Owned<tim_bas::Tim7>> { tim_bas::Tim7::acquire() }
    pub fn tim2(&self) -> Option<Owned<tim_gen::Tim2>> { tim_gen::Tim2::acquire() }
        pub fn tim2_ch1(&self) -> Option<Owned<tim_gen::Tim2Ch1>> { tim_gen::Tim2Ch1::acquire() }
        pub fn tim2_ch2(&self) -> Option<Owned<tim_gen::Tim2Ch2>> { tim_gen::Tim2Ch2::acquire() }
        pub fn tim2_ch3(&self) -> Option<Owned<tim_gen::Tim2Ch3>> { tim_gen::Tim2Ch3::acquire() }
        pub fn tim2_ch4(&self) -> Option<Owned<tim_gen::Tim2Ch4>> { tim_gen::Tim2Ch4::acquire() }
    pub fn tim3(&self) -> Option<Owned<tim_gen::Tim3>> { tim_gen::Tim3::acquire() }
        pub fn tim3_ch1(&self) -> Option<Owned<tim_gen::Tim3Ch1>> { tim_gen::Tim3Ch1::acquire() }
        pub fn tim3_ch2(&self) -> Option<Owned<tim_gen::Tim3Ch2>> { tim_gen::Tim3Ch2::acquire() }
        pub fn tim3_ch3(&self) -> Option<Owned<tim_gen::Tim3Ch3>> { tim_gen::Tim3Ch3::acquire() }
        pub fn tim3_ch4(&self) -> Option<Owned<tim_gen::Tim3Ch4>> { tim_gen::Tim3Ch4::acquire() }
    pub fn tim4(&self) -> Option<Owned<tim_gen::Tim4>> { tim_gen::Tim4::acquire() }
        pub fn tim4_ch1(&self) -> Option<Owned<tim_gen::Tim4Ch1>> { tim_gen::Tim4Ch1::acquire() }
        pub fn tim4_ch2(&self) -> Option<Owned<tim_gen::Tim4Ch2>> { tim_gen::Tim4Ch2::acquire() }
        pub fn tim4_ch3(&self) -> Option<Owned<tim_gen::Tim4Ch3>> { tim_gen::Tim4Ch3::acquire() }
        pub fn tim4_ch4(&self) -> Option<Owned<tim_gen::Tim4Ch4>> { tim_gen::Tim4Ch4::acquire() }
    pub fn tim5(&self) -> Option<Owned<tim_gen::Tim5>> { tim_gen::Tim5::acquire() }
        pub fn tim5_ch1(&self) -> Option<Owned<tim_gen::Tim5Ch1>> { tim_gen::Tim5Ch1::acquire() }
        pub fn tim5_ch2(&self) -> Option<Owned<tim_gen::Tim5Ch2>> { tim_gen::Tim5Ch2::acquire() }
        pub fn tim5_ch3(&self) -> Option<Owned<tim_gen::Tim5Ch3>> { tim_gen::Tim5Ch3::acquire() }
        pub fn tim5_ch4(&self) -> Option<Owned<tim_gen::Tim5Ch4>> { tim_gen::Tim5Ch4::acquire() }
    pub fn tim15(&self) -> Option<Owned<tim_gen::Tim15>> { tim_gen::Tim15::acquire() }
        pub fn tim15_ch1(&self) -> Option<Owned<tim_gen::Tim15Ch1>> { tim_gen::Tim15Ch1::acquire() }
        pub fn tim15_ch2(&self) -> Option<Owned<tim_gen::Tim15Ch2>> { tim_gen::Tim15Ch2::acquire() }
    pub fn tim16(&self) -> Option<Owned<tim_gen::Tim16>> { tim_gen::Tim16::acquire() }
        pub fn tim16_ch1(&self) -> Option<Owned<tim_gen::Tim16Ch1>> { tim_gen::Tim16Ch1::acquire() }
        pub fn tim16_ch2(&self) -> Option<Owned<tim_gen::Tim16Ch2>> { tim_gen::Tim16Ch2::acquire() }
    pub fn gpioa(&self) -> Option<Owned<gpio::Gpioa>> { gpio::Gpioa::acquire() }
        pub fn pa0(&self) -> Option<Owned<pin::Pa0>> { pin::Pa0::acquire() }
        pub fn pa1(&self) -> Option<Owned<pin::Pa1>> { pin::Pa1::acquire() }
        pub fn pa2(&self) -> Option<Owned<pin::Pa2>> { pin::Pa2::acquire() }
        pub fn pa3(&self) -> Option<Owned<pin::Pa3>> { pin::Pa3::acquire() }
        pub fn pa4(&self) -> Option<Owned<pin::Pa4>> { pin::Pa4::acquire() }
        pub fn pa5(&self) -> Option<Owned<pin::Pa5>> { pin::Pa5::acquire() }
        pub fn pa6(&self) -> Option<Owned<pin::Pa6>> { pin::Pa6::acquire() }
        pub fn pa7(&self) -> Option<Owned<pin::Pa7>> { pin::Pa7::acquire() }
        pub fn pa8(&self) -> Option<Owned<pin::Pa8>> { pin::Pa8::acquire() }
        pub fn pa9(&self) -> Option<Owned<pin::Pa9>> { pin::Pa9::acquire() }
        pub fn pa10(&self) -> Option<Owned<pin::Pa10>> { pin::Pa10::acquire() }
        pub fn pa11(&self) -> Option<Owned<pin::Pa11>> { pin::Pa11::acquire() }
        pub fn pa12(&self) -> Option<Owned<pin::Pa12>> { pin::Pa12::acquire() }
        pub fn pa13(&self) -> Option<Owned<pin::Pa13>> { pin::Pa13::acquire() }
        pub fn pa14(&self) -> Option<Owned<pin::Pa14>> { pin::Pa14::acquire() }
        pub fn pa15(&self) -> Option<Owned<pin::Pa15>> { pin::Pa15::acquire() }
    pub fn gpiob(&self) -> Option<Owned<gpio::Gpiob>> { gpio::Gpiob::acquire() }
        pub fn pb0(&self) -> Option<Owned<pin::Pb0>> { pin::Pb0::acquire() }
        pub fn pb1(&self) -> Option<Owned<pin::Pb1>> { pin::Pb1::acquire() }
        pub fn pb3(&self) -> Option<Owned<pin::Pb3>> { pin::Pb3::acquire() }
        pub fn pb4(&self) -> Option<Owned<pin::Pb4>> { pin::Pb4::acquire() }
        pub fn pb5(&self) -> Option<Owned<pin::Pb5>> { pin::Pb5::acquire() }
        pub fn pb6(&self) -> Option<Owned<pin::Pb6>> { pin::Pb6::acquire() }
        pub fn pb7(&self) -> Option<Owned<pin::Pb7>> { pin::Pb7::acquire() }
    pub fn gpioc(&self) -> Option<Owned<gpio::Gpioc>> { gpio::Gpioc::acquire() }
        pub fn pc0(&self) -> Option<Owned<pin::Pc0>> { pin::Pc0::acquire() }
        pub fn pc14(&self) -> Option<Owned<pin::Pc14>> { pin::Pc14::acquire() }
        pub fn pc15(&self) -> Option<Owned<pin::Pc15>> { pin::Pc15::acquire() }
    pub fn gpiod(&self) -> Option<Owned<gpio::Gpiod>> { gpio::Gpiod::acquire() }
    pub fn gpioe(&self) -> Option<Owned<gpio::Gpioe>> { gpio::Gpioe::acquire() }
    pub fn gpiof(&self) -> Option<Owned<gpio::Gpiof>> { gpio::Gpiof::acquire() }
    pub fn gpiog(&self) -> Option<Owned<gpio::Gpiog>> { gpio::Gpiog::acquire() }
    pub fn gpioh(&self) -> Option<Owned<gpio::Gpioh>> { gpio::Gpioh::acquire() }
        pub fn ph3(&self) -> Option<Owned<pin::Ph3>> { pin::Ph3::acquire() }
    pub fn usart1(&self) -> Option<Owned<usart::Usart1>> { usart::Usart1::acquire() }
    pub fn usart2(&self) -> Option<Owned<usart::Usart2>> { usart::Usart2::acquire() }
    pub fn usart3(&self) -> Option<Owned<usart::Usart3>> { usart::Usart3::acquire() }
    pub fn uart4(&self) -> Option<Owned<usart::Uart4>> { usart::Uart4::acquire() }
    pub fn uart5(&self) -> Option<Owned<usart::Uart5>> { usart::Uart5::acquire() }
    pub fn lpuart1(&self) -> Option<Owned<lpuart::Lpuart1>> { lpuart::Lpuart1::acquire() }
    pub fn spi1(&self) -> Option<Owned<spi::Spi1>> { spi::Spi1::acquire() }
    pub fn spi2(&self) -> Option<Owned<spi::Spi2>> { spi::Spi2::acquire() }
    pub fn spi3(&self) -> Option<Owned<spi::Spi3>> { spi::Spi3::acquire() }
    pub fn adc1(&self) -> Option<Owned<adc::Adc1>> { adc::Adc1::acquire() }
        pub fn adc1_ch1(&self) -> Option<Owned<adc::Adc1Ch1>> { adc::Adc1Ch1::acquire() }
        pub fn adc1_ch2(&self) -> Option<Owned<adc::Adc1Ch2>> { adc::Adc1Ch2::acquire() }
        pub fn adc1_ch3(&self) -> Option<Owned<adc::Adc1Ch3>> { adc::Adc1Ch3::acquire() }
        pub fn adc1_ch4(&self) -> Option<Owned<adc::Adc1Ch4>> { adc::Adc1Ch4::acquire() }
        pub fn adc1_ch5(&self) -> Option<Owned<adc::Adc1Ch5>> { adc::Adc1Ch5::acquire() }
        pub fn adc1_ch6(&self) -> Option<Owned<adc::Adc1Ch6>> { adc::Adc1Ch6::acquire() }
        pub fn adc1_ch7(&self) -> Option<Owned<adc::Adc1Ch7>> { adc::Adc1Ch7::acquire() }
        pub fn adc1_ch8(&self) -> Option<Owned<adc::Adc1Ch8>> { adc::Adc1Ch8::acquire() }
        pub fn adc1_ch9(&self) -> Option<Owned<adc::Adc1Ch9>> { adc::Adc1Ch9::acquire() }
        pub fn adc1_ch10(&self) -> Option<Owned<adc::Adc1Ch10>> { adc::Adc1Ch10::acquire() }
        pub fn adc1_ch11(&self) -> Option<Owned<adc::Adc1Ch11>> { adc::Adc1Ch11::acquire() }
        pub fn adc1_ch12(&self) -> Option<Owned<adc::Adc1Ch12>> { adc::Adc1Ch12::acquire() }
        pub fn adc1_ch13(&self) -> Option<Owned<adc::Adc1Ch13>> { adc::Adc1Ch13::acquire() }
        pub fn adc1_ch14(&self) -> Option<Owned<adc::Adc1Ch14>> { adc::Adc1Ch14::acquire() }
        pub fn adc1_ch15(&self) -> Option<Owned<adc::Adc1Ch15>> { adc::Adc1Ch15::acquire() }
        pub fn adc1_temp(&self) -> Option<Owned<adc::Adc1Temp>> { adc::Adc1Temp::acquire() }
        pub fn adc1_refint(&self) -> Option<Owned<adc::Adc1Refint>> { adc::Adc1Refint::acquire() }
        pub fn adc1_bat(&self) -> Option<Owned<adc::Adc1Bat>> { adc::Adc1Bat::acquire() }
    pub fn adc2(&self) -> Option<Owned<adc::Adc2>> { adc::Adc2::acquire() }
        pub fn adc2_ch1(&self) -> Option<Owned<adc::Adc2Ch1>> { adc::Adc2Ch1::acquire() }
        pub fn adc2_ch2(&self) -> Option<Owned<adc::Adc2Ch2>> { adc::Adc2Ch2::acquire() }
        pub fn adc2_ch3(&self) -> Option<Owned<adc::Adc2Ch3>> { adc::Adc2Ch3::acquire() }
        pub fn adc2_ch4(&self) -> Option<Owned<adc::Adc2Ch4>> { adc::Adc2Ch4::acquire() }
        pub fn adc2_ch5(&self) -> Option<Owned<adc::Adc2Ch5>> { adc::Adc2Ch5::acquire() }
        pub fn adc2_ch6(&self) -> Option<Owned<adc::Adc2Ch6>> { adc::Adc2Ch6::acquire() }
        pub fn adc2_ch7(&self) -> Option<Owned<adc::Adc2Ch7>> { adc::Adc2Ch7::acquire() }
        pub fn adc2_ch8(&self) -> Option<Owned<adc::Adc2Ch8>> { adc::Adc2Ch8::acquire() }
        pub fn adc2_ch9(&self) -> Option<Owned<adc::Adc2Ch9>> { adc::Adc2Ch9::acquire() }
        pub fn adc2_ch10(&self) -> Option<Owned<adc::Adc2Ch10>> { adc::Adc2Ch10::acquire() }
        pub fn adc2_ch11(&self) -> Option<Owned<adc::Adc2Ch11>> { adc::Adc2Ch11::acquire() }
        pub fn adc2_ch12(&self) -> Option<Owned<adc::Adc2Ch12>> { adc::Adc2Ch12::acquire() }
        pub fn adc2_ch13(&self) -> Option<Owned<adc::Adc2Ch13>> { adc::Adc2Ch13::acquire() }
        pub fn adc2_ch14(&self) -> Option<Owned<adc::Adc2Ch14>> { adc::Adc2Ch14::acquire() }
        pub fn adc2_ch15(&self) -> Option<Owned<adc::Adc2Ch15>> { adc::Adc2Ch15::acquire() }
    pub fn adc3(&self) -> Option<Owned<adc::Adc3>> { adc::Adc3::acquire() }
        pub fn adc3_ch1(&self) -> Option<Owned<adc::Adc3Ch1>> { adc::Adc3Ch1::acquire() }
        pub fn adc3_ch2(&self) -> Option<Owned<adc::Adc3Ch2>> { adc::Adc3Ch2::acquire() }
        pub fn adc3_ch3(&self) -> Option<Owned<adc::Adc3Ch3>> { adc::Adc3Ch3::acquire() }
        pub fn adc3_ch4(&self) -> Option<Owned<adc::Adc3Ch4>> { adc::Adc3Ch4::acquire() }
        pub fn adc3_ch5(&self) -> Option<Owned<adc::Adc3Ch5>> { adc::Adc3Ch5::acquire() }
        pub fn adc3_ch6(&self) -> Option<Owned<adc::Adc3Ch6>> { adc::Adc3Ch6::acquire() }
        pub fn adc3_ch7(&self) -> Option<Owned<adc::Adc3Ch7>> { adc::Adc3Ch7::acquire() }
        pub fn adc3_ch8(&self) -> Option<Owned<adc::Adc3Ch8>> { adc::Adc3Ch8::acquire() }
        pub fn adc3_ch9(&self) -> Option<Owned<adc::Adc3Ch9>> { adc::Adc3Ch9::acquire() }
        pub fn adc3_ch10(&self) -> Option<Owned<adc::Adc3Ch10>> { adc::Adc3Ch10::acquire() }
        pub fn adc3_ch11(&self) -> Option<Owned<adc::Adc3Ch11>> { adc::Adc3Ch11::acquire() }
        pub fn adc3_ch12(&self) -> Option<Owned<adc::Adc3Ch12>> { adc::Adc3Ch12::acquire() }
        pub fn adc3_ch13(&self) -> Option<Owned<adc::Adc3Ch13>> { adc::Adc3Ch13::acquire() }
        pub fn adc3_ch14(&self) -> Option<Owned<adc::Adc3Ch14>> { adc::Adc3Ch14::acquire() }
        pub fn adc3_ch15(&self) -> Option<Owned<adc::Adc3Ch15>> { adc::Adc3Ch15::acquire() }
}

impl Get<flash::Flash> for Stm32l432x {
    fn get(&self) -> flash::Flash { flash::FLASH }
}

impl GetPeriph<flash::FlashPeriph> for Stm32l432x {
    fn get_periph(&self) -> flash::FlashPeriph { flash::FLASH_PERIPH }
}

impl GetPeriphInstance<flash::FlashPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<flash::FlashPeriph> {
        match index { 
            0 => Some(flash::FLASH_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<pwr::Pwr> for Stm32l432x {
    fn get(&self) -> pwr::Pwr { pwr::PWR }
}

impl GetPeriph<pwr::PwrPeriph> for Stm32l432x {
    fn get_periph(&self) -> pwr::PwrPeriph { pwr::PWR_PERIPH }
}

impl GetPeriphInstance<pwr::PwrPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<pwr::PwrPeriph> {
        match index { 
            0 => Some(pwr::PWR_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<rcc::Rcc> for Stm32l432x {
    fn get(&self) -> rcc::Rcc { rcc::RCC }
}

impl GetPeriph<rcc::RccPeriph> for Stm32l432x {
    fn get_periph(&self) -> rcc::RccPeriph { rcc::RCC_PERIPH }
}

impl GetPeriphInstance<rcc::RccPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<rcc::RccPeriph> {
        match index { 
            0 => Some(rcc::RCC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<syscfg::Syscfg> for Stm32l432x {
    fn get(&self) -> syscfg::Syscfg { syscfg::SYSCFG }
}

impl GetPeriph<syscfg::SyscfgPeriph> for Stm32l432x {
    fn get_periph(&self) -> syscfg::SyscfgPeriph { syscfg::SYSCFG_PERIPH }
}

impl GetPeriphInstance<syscfg::SyscfgPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<syscfg::SyscfgPeriph> {
        match index { 
            0 => Some(syscfg::SYSCFG_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<c_adc::CAdc> for Stm32l432x {
    fn get(&self) -> c_adc::CAdc { c_adc::C_ADC }
}

impl GetPeriph<c_adc::CAdcPeriph> for Stm32l432x {
    fn get_periph(&self) -> c_adc::CAdcPeriph { c_adc::C_ADC_PERIPH }
}

impl GetPeriphInstance<c_adc::CAdcPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<c_adc::CAdcPeriph> {
        match index { 
            0 => Some(c_adc::C_ADC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<dac::Dac1> for Stm32l432x {
    fn get(&self) -> dac::Dac1 { dac::DAC1 }
}

impl GetPeriph<dac::DacPeriph> for Stm32l432x {
    fn get_periph(&self) -> dac::DacPeriph { dac::DAC1_PERIPH }
}

impl GetPeriphInstance<dac::DacPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<dac::DacPeriph> {
        match index {
            0 => Some(dac::DAC1_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<rng::Rng> for Stm32l432x {
    fn get(&self) -> rng::Rng { rng::RNG }
}

impl GetPeriph<rng::RngPeriph> for Stm32l432x {
    fn get_periph(&self) -> rng::RngPeriph { rng::RNG_PERIPH }
}

impl GetPeriphInstance<rng::RngPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<rng::RngPeriph> {
        match index {
            0 => Some(rng::RNG_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<iwdg::Iwdg> for Stm32l432x {
    fn get(&self) -> iwdg::Iwdg { iwdg::IWDG }
}

impl GetPeriph<iwdg::IwdgPeriph> for Stm32l432x {
    fn get_periph(&self) -> iwdg::IwdgPeriph { iwdg::IWDG_PERIPH }
}

impl GetPeriphInstance<iwdg::IwdgPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<iwdg::IwdgPeriph> {
        match index {
            0 => Some(iwdg::IWDG_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<wwdg::Wwdg> for Stm32l432x {
    fn get(&self) -> wwdg::Wwdg { wwdg::WWDG }
}

impl GetPeriph<wwdg::WwdgPeriph> for Stm32l432x {
    fn get_periph(&self) -> wwdg::WwdgPeriph { wwdg::WWDG_PERIPH }
}

impl GetPeriphInstance<wwdg::WwdgPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<wwdg::WwdgPeriph> {
        match index {
            0 => Some(wwdg::WWDG_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<crc::Crc> for Stm32l432x {
    fn get(&self) -> crc::Crc { crc::CRC }
}

impl GetPeriph<crc::CrcPeriph> for Stm32l432x {
    fn get_periph(&self) -> crc::CrcPeriph { crc::CRC_PERIPH }
}

impl GetPeriphInstance<crc::CrcPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<crc::CrcPeriph> {
        match index {
            0 => Some(crc::CRC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<rtc::Rtc> for Stm32l432x {
    fn get(&self) -> rtc::Rtc { rtc::RTC }
}

impl GetPeriph<rtc::RtcPeriph> for Stm32l432x {
    fn get_periph(&self) -> rtc::RtcPeriph { rtc::RTC_PERIPH }
}

impl GetPeriphInstance<rtc::RtcPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<rtc::RtcPeriph> {
        match index {
            0 => Some(rtc::RTC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<lptim::Lptim1> for Stm32l432x {
    fn get(&self) -> lptim::Lptim1 { lptim::LPTIM1 }
}

impl Get<lptim::Lptim2> for Stm32l432x {
    fn get(&self) -> lptim::Lptim2 { lptim::LPTIM2 }
}

impl GetPeriphInstance<lptim::LptimPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<lptim::LptimPeriph> {
        match index {
            0 => Some(lptim::LPTIM1_PERIPH),
            1 => Some(lptim::LPTIM2_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

impl Get<exti::Exti> for Stm32l432x {
    fn get(&self) -> exti::Exti { exti::EXTI }
}

impl GetPeriph<exti::ExtiPeriph> for Stm32l432x {
    fn get_periph(&self) -> exti::ExtiPeriph { exti::EXTI_PERIPH }
}

impl GetPeriphInstance<exti::ExtiPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<exti::ExtiPeriph> {
        match index {
            0 => Some(exti::EXTI_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<dma::Dma1> for Stm32l432x {
    fn get(&self) -> dma::Dma1 { dma::DMA1 }
}

impl Get<dma::Dma2> for Stm32l432x {
    fn get(&self) -> dma::Dma2 { dma::DMA2 }
}

impl GetPeriphInstance<dma::DmaPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<dma::DmaPeriph> {
        match index {
            0 => Some(dma::DMA1_PERIPH),
            1 => Some(dma::DMA2_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

impl Get<i2c::I2c1> for Stm32l432x {
    fn get(&self) -> i2c::I2c1 { i2c::I2C1 }
}

impl Get<i2c::I2c2> for Stm32l432x {
    fn get(&self) -> i2c::I2c2 { i2c::I2C2 }
}

impl Get<i2c::I2c3> for Stm32l432x {
    fn get(&self) -> i2c::I2c3 { i2c::I2C3 }
}

impl GetPeriphInstance<i2c::I2cPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<i2c::I2cPeriph> {
        match index {
            0 => Some(i2c::I2C1_PERIPH),
            1 => Some(i2c::I2C2_PERIPH),
            2 => Some(i2c::I2C3_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 3 }
}

impl Get<tim_adv::Tim1> for Stm32l432x {
    fn get(&self) -> tim_adv::Tim1 { tim_adv::TIM1 }
}

impl Get<tim_adv::Tim8> for Stm32l432x {
    fn get(&self) -> tim_adv::Tim8 { tim_adv::TIM8 }
}

impl GetPeriphInstance<tim_adv::TimAdvPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<tim_adv::TimAdvPeriph> {
        match index {
            0 => Some(tim_adv::TIM1_PERIPH),
            1 => Some(tim_adv::TIM8_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

impl Get<tim_bas::Tim6> for Stm32l432x {
    fn get(&self) -> tim_bas::Tim6 { tim_bas::TIM6 }
}

impl Get<tim_bas::Tim7> for Stm32l432x {
    fn get(&self) -> tim_bas::Tim7 { tim_bas::TIM7 }
}

impl GetPeriphInstance<tim_bas::TimBasPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<tim_bas::TimBasPeriph> {
        match index {
            0 => Some(tim_bas::TIM6_PERIPH),
            1 => Some(tim_bas::TIM7_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

impl Get<tim_gen::Tim2> for Stm32l432x {
    fn get(&self) -> tim_gen::Tim2 { tim_gen::TIM2 }
}

impl Get<tim_gen::Tim3> for Stm32l432x {
    fn get(&self) -> tim_gen::Tim3 { tim_gen::TIM3 }
}

impl Get<tim_gen::Tim4> for Stm32l432x {
    fn get(&self) -> tim_gen::Tim4 { tim_gen::TIM4 }
}

impl Get<tim_gen::Tim5> for Stm32l432x {
    fn get(&self) -> tim_gen::Tim5 { tim_gen::TIM5 }
}

impl Get<tim_gen::Tim15> for Stm32l432x {
    fn get(&self) -> tim_gen::Tim15 { tim_gen::TIM15 }
}

impl Get<tim_gen::Tim16> for Stm32l432x {
    fn get(&self) -> tim_gen::Tim16 { tim_gen::TIM16 }
}

impl GetPeriphInstance<tim_gen::TimGenPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<tim_gen::TimGenPeriph> {
        match index {
            0 => Some(tim_gen::TIM2_PERIPH),
            1 => Some(tim_gen::TIM3_PERIPH),
            2 => Some(tim_gen::TIM4_PERIPH),
            3 => Some(tim_gen::TIM5_PERIPH),
            4 => Some(tim_gen::TIM15_PERIPH),
            5 => Some(tim_gen::TIM16_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 6 }
}

impl Get<gpio::Gpioa> for Stm32l432x {
    fn get(&self) -> gpio::Gpioa { gpio::GPIOA }
}

impl Get<gpio::Gpiob> for Stm32l432x {
    fn get(&self) -> gpio::Gpiob { gpio::GPIOB }
}

impl Get<gpio::Gpioc> for Stm32l432x {
    fn get(&self) -> gpio::Gpioc { gpio::GPIOC }
}

impl Get<gpio::Gpiod> for Stm32l432x {
    fn get(&self) -> gpio::Gpiod { gpio::GPIOD }
}

impl Get<gpio::Gpioe> for Stm32l432x {
    fn get(&self) -> gpio::Gpioe { gpio::GPIOE }
}

impl Get<gpio::Gpiof> for Stm32l432x {
    fn get(&self) -> gpio::Gpiof { gpio::GPIOF }
}

impl Get<gpio::Gpiog> for Stm32l432x {
    fn get(&self) -> gpio::Gpiog { gpio::GPIOG }
}

impl Get<gpio::Gpioh> for Stm32l432x {
    fn get(&self) -> gpio::Gpioh { gpio::GPIOH }
}

impl GetPeriphInstance<gpio::GpioPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<gpio::GpioPeriph> {
        match index {
            0 => Some(gpio::GPIOA_PERIPH),
            1 => Some(gpio::GPIOB_PERIPH),
            2 => Some(gpio::GPIOC_PERIPH),
            3 => Some(gpio::GPIOD_PERIPH),
            4 => Some(gpio::GPIOE_PERIPH),
            5 => Some(gpio::GPIOF_PERIPH),
            6 => Some(gpio::GPIOG_PERIPH),
            7 => Some(gpio::GPIOH_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 8 }
}

impl Get<usart::Usart1> for Stm32l432x {
    fn get(&self) -> usart::Usart1 { usart::USART1 }
}

impl Get<usart::Usart2> for Stm32l432x {
    fn get(&self) -> usart::Usart2 { usart::USART2 }
}

impl Get<usart::Usart3> for Stm32l432x {
    fn get(&self) -> usart::Usart3 { usart::USART3 }
}

impl Get<usart::Uart4> for Stm32l432x {
    fn get(&self) -> usart::Uart4 { usart::UART4 }
}

impl Get<usart::Uart5> for Stm32l432x {
    fn get(&self) -> usart::Uart5 { usart::UART5 }
}

impl GetPeriphInstance<usart::UsartPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<usart::UsartPeriph> {
        match index {
            0 => Some(usart::USART1_PERIPH),
            1 => Some(usart::USART2_PERIPH),
            2 => Some(usart::USART3_PERIPH),
            3 => Some(usart::UART4_PERIPH),
            4 => Some(usart::UART5_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 5 }
}

impl Get<lpuart::Lpuart1> for Stm32l432x {
    fn get(&self) -> lpuart::Lpuart1 { lpuart::LPUART1 }
}

impl GetPeriph<lpuart::LpuartPeriph> for Stm32l432x {
    fn get_periph(&self) -> lpuart::LpuartPeriph { lpuart::LPUART1_PERIPH }
}

impl GetPeriphInstance<lpuart::LpuartPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<lpuart::LpuartPeriph> {
        match index {
            0 => Some(lpuart::LPUART1_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<spi::Spi1> for Stm32l432x {
    fn get(&self) -> spi::Spi1 { spi::SPI1 }
}

impl Get<spi::Spi2> for Stm32l432x {
    fn get(&self) -> spi::Spi2 { spi::SPI2 }
}

impl Get<spi::Spi3> for Stm32l432x {
    fn get(&self) -> spi::Spi3 { spi::SPI3 }
}

impl GetPeriphInstance<spi::SpiPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<spi::SpiPeriph> {
        match index {
            0 => Some(spi::SPI1_PERIPH),
            1 => Some(spi::SPI2_PERIPH),
            2 => Some(spi::SPI3_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 3 }
}

impl Get<adc::Adc1> for Stm32l432x {
    fn get(&self) -> adc::Adc1 { adc::ADC1 }
}

impl Get<adc::Adc2> for Stm32l432x {
    fn get(&self) -> adc::Adc2 { adc::ADC2 }
}

impl Get<adc::Adc3> for Stm32l432x {
    fn get(&self) -> adc::Adc3 { adc::ADC3 }
}

impl GetPeriphInstance<adc::AdcPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<adc::AdcPeriph> {
        match index {
            0 => Some(adc::ADC1_PERIPH),
            1 => Some(adc::ADC2_PERIPH),
            2 => Some(adc::ADC3_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 3 }
}

