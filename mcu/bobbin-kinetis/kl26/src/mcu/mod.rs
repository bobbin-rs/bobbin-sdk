pub use ::bobbin_mcu::*;
use ::bobbin_common::owned::*;

pub mod ftfa;
pub mod osc;
pub mod sim;
pub mod mcg;
pub mod rcm;
pub mod dmamux;
pub mod dma;
pub mod tpm;
pub mod pit;
pub mod lptmr;
pub mod spi;
pub mod i2c;
pub mod port;
pub mod gpio;
pub mod uart0;
pub mod uart;
pub mod adc;
pub mod sig;
pub mod pin;
pub mod irq;

#[derive(Debug, Default)]
pub struct Kl26 {}

impl Mcu for Kl26 {
    fn id(&self) -> &'static str { "KL26" }
}

impl Kl26 {
    pub fn ftfa(&self) -> Option<Owned<ftfa::Ftfa>> { ftfa::Ftfa::acquire() }
    pub fn osc0(&self) -> Option<Owned<osc::Osc>> { osc::Osc::acquire() }
    pub fn sim(&self) -> Option<Owned<sim::Sim>> { sim::Sim::acquire() }
    pub fn mcg(&self) -> Option<Owned<mcg::Mcg>> { mcg::Mcg::acquire() }
    pub fn rcm(&self) -> Option<Owned<rcm::Rcm>> { rcm::Rcm::acquire() }
    pub fn dmamux(&self) -> Option<Owned<dmamux::Dmamux>> { dmamux::Dmamux::acquire() }
    pub fn dma(&self) -> Option<Owned<dma::Dma>> { dma::Dma::acquire() }
        pub fn dma0(&self) -> Option<Owned<dma::Dma0>> { dma::Dma0::acquire() }
        pub fn dma1(&self) -> Option<Owned<dma::Dma1>> { dma::Dma1::acquire() }
        pub fn dma2(&self) -> Option<Owned<dma::Dma2>> { dma::Dma2::acquire() }
        pub fn dma3(&self) -> Option<Owned<dma::Dma3>> { dma::Dma3::acquire() }
    pub fn tpm0(&self) -> Option<Owned<tpm::Tpm0>> { tpm::Tpm0::acquire() }
        pub fn tpm0_ch0(&self) -> Option<Owned<tpm::Tpm0Ch0>> { tpm::Tpm0Ch0::acquire() }
        pub fn tpm0_ch1(&self) -> Option<Owned<tpm::Tpm0Ch1>> { tpm::Tpm0Ch1::acquire() }
        pub fn tpm0_ch2(&self) -> Option<Owned<tpm::Tpm0Ch2>> { tpm::Tpm0Ch2::acquire() }
        pub fn tpm0_ch3(&self) -> Option<Owned<tpm::Tpm0Ch3>> { tpm::Tpm0Ch3::acquire() }
        pub fn tpm0_ch4(&self) -> Option<Owned<tpm::Tpm0Ch4>> { tpm::Tpm0Ch4::acquire() }
        pub fn tpm0_ch5(&self) -> Option<Owned<tpm::Tpm0Ch5>> { tpm::Tpm0Ch5::acquire() }
    pub fn tpm1(&self) -> Option<Owned<tpm::Tpm1>> { tpm::Tpm1::acquire() }
        pub fn tpm1_ch0(&self) -> Option<Owned<tpm::Tpm1Ch0>> { tpm::Tpm1Ch0::acquire() }
        pub fn tpm1_ch1(&self) -> Option<Owned<tpm::Tpm1Ch1>> { tpm::Tpm1Ch1::acquire() }
        pub fn tpm1_ch2(&self) -> Option<Owned<tpm::Tpm1Ch2>> { tpm::Tpm1Ch2::acquire() }
        pub fn tpm1_ch3(&self) -> Option<Owned<tpm::Tpm1Ch3>> { tpm::Tpm1Ch3::acquire() }
        pub fn tpm1_ch4(&self) -> Option<Owned<tpm::Tpm1Ch4>> { tpm::Tpm1Ch4::acquire() }
        pub fn tpm1_ch5(&self) -> Option<Owned<tpm::Tpm1Ch5>> { tpm::Tpm1Ch5::acquire() }
    pub fn tpm2(&self) -> Option<Owned<tpm::Tpm2>> { tpm::Tpm2::acquire() }
        pub fn tpm2_ch0(&self) -> Option<Owned<tpm::Tpm2Ch0>> { tpm::Tpm2Ch0::acquire() }
        pub fn tpm2_ch1(&self) -> Option<Owned<tpm::Tpm2Ch1>> { tpm::Tpm2Ch1::acquire() }
        pub fn tpm2_ch2(&self) -> Option<Owned<tpm::Tpm2Ch2>> { tpm::Tpm2Ch2::acquire() }
        pub fn tpm2_ch3(&self) -> Option<Owned<tpm::Tpm2Ch3>> { tpm::Tpm2Ch3::acquire() }
        pub fn tpm2_ch4(&self) -> Option<Owned<tpm::Tpm2Ch4>> { tpm::Tpm2Ch4::acquire() }
        pub fn tpm2_ch5(&self) -> Option<Owned<tpm::Tpm2Ch5>> { tpm::Tpm2Ch5::acquire() }
    pub fn pit(&self) -> Option<Owned<pit::Pit>> { pit::Pit::acquire() }
        pub fn pit_ch0(&self) -> Option<Owned<pit::PitCh0>> { pit::PitCh0::acquire() }
        pub fn pit_ch1(&self) -> Option<Owned<pit::PitCh1>> { pit::PitCh1::acquire() }
        pub fn pit_ch2(&self) -> Option<Owned<pit::PitCh2>> { pit::PitCh2::acquire() }
        pub fn pit_ch3(&self) -> Option<Owned<pit::PitCh3>> { pit::PitCh3::acquire() }
    pub fn lptmr0(&self) -> Option<Owned<lptmr::Lptmr0>> { lptmr::Lptmr0::acquire() }
    pub fn spi0(&self) -> Option<Owned<spi::Spi0>> { spi::Spi0::acquire() }
    pub fn spi1(&self) -> Option<Owned<spi::Spi1>> { spi::Spi1::acquire() }
    pub fn i2c0(&self) -> Option<Owned<i2c::I2c0>> { i2c::I2c0::acquire() }
    pub fn i2c1(&self) -> Option<Owned<i2c::I2c1>> { i2c::I2c1::acquire() }
    pub fn porta(&self) -> Option<Owned<port::Porta>> { port::Porta::acquire() }
        pub fn pta0(&self) -> Option<Owned<pin::Pta0>> { pin::Pta0::acquire() }
        pub fn pta1(&self) -> Option<Owned<pin::Pta1>> { pin::Pta1::acquire() }
        pub fn pta2(&self) -> Option<Owned<pin::Pta2>> { pin::Pta2::acquire() }
        pub fn pta3(&self) -> Option<Owned<pin::Pta3>> { pin::Pta3::acquire() }
        pub fn pta4(&self) -> Option<Owned<pin::Pta4>> { pin::Pta4::acquire() }
        pub fn pta5(&self) -> Option<Owned<pin::Pta5>> { pin::Pta5::acquire() }
        pub fn pta12(&self) -> Option<Owned<pin::Pta12>> { pin::Pta12::acquire() }
        pub fn pta13(&self) -> Option<Owned<pin::Pta13>> { pin::Pta13::acquire() }
        pub fn pta18(&self) -> Option<Owned<pin::Pta18>> { pin::Pta18::acquire() }
        pub fn pta19(&self) -> Option<Owned<pin::Pta19>> { pin::Pta19::acquire() }
        pub fn pta20(&self) -> Option<Owned<pin::Pta20>> { pin::Pta20::acquire() }
    pub fn portb(&self) -> Option<Owned<port::Portb>> { port::Portb::acquire() }
        pub fn ptb0(&self) -> Option<Owned<pin::Ptb0>> { pin::Ptb0::acquire() }
        pub fn ptb1(&self) -> Option<Owned<pin::Ptb1>> { pin::Ptb1::acquire() }
        pub fn ptb2(&self) -> Option<Owned<pin::Ptb2>> { pin::Ptb2::acquire() }
        pub fn ptb3(&self) -> Option<Owned<pin::Ptb3>> { pin::Ptb3::acquire() }
        pub fn ptb16(&self) -> Option<Owned<pin::Ptb16>> { pin::Ptb16::acquire() }
        pub fn ptb17(&self) -> Option<Owned<pin::Ptb17>> { pin::Ptb17::acquire() }
        pub fn ptb18(&self) -> Option<Owned<pin::Ptb18>> { pin::Ptb18::acquire() }
        pub fn ptb19(&self) -> Option<Owned<pin::Ptb19>> { pin::Ptb19::acquire() }
    pub fn portc(&self) -> Option<Owned<port::Portc>> { port::Portc::acquire() }
        pub fn ptc0(&self) -> Option<Owned<pin::Ptc0>> { pin::Ptc0::acquire() }
        pub fn ptc1(&self) -> Option<Owned<pin::Ptc1>> { pin::Ptc1::acquire() }
        pub fn ptc2(&self) -> Option<Owned<pin::Ptc2>> { pin::Ptc2::acquire() }
        pub fn ptc3(&self) -> Option<Owned<pin::Ptc3>> { pin::Ptc3::acquire() }
        pub fn ptc4(&self) -> Option<Owned<pin::Ptc4>> { pin::Ptc4::acquire() }
        pub fn ptc5(&self) -> Option<Owned<pin::Ptc5>> { pin::Ptc5::acquire() }
        pub fn ptc6(&self) -> Option<Owned<pin::Ptc6>> { pin::Ptc6::acquire() }
        pub fn ptc7(&self) -> Option<Owned<pin::Ptc7>> { pin::Ptc7::acquire() }
        pub fn ptc8(&self) -> Option<Owned<pin::Ptc8>> { pin::Ptc8::acquire() }
        pub fn ptc9(&self) -> Option<Owned<pin::Ptc9>> { pin::Ptc9::acquire() }
        pub fn ptc10(&self) -> Option<Owned<pin::Ptc10>> { pin::Ptc10::acquire() }
        pub fn ptc11(&self) -> Option<Owned<pin::Ptc11>> { pin::Ptc11::acquire() }
    pub fn portd(&self) -> Option<Owned<port::Portd>> { port::Portd::acquire() }
        pub fn ptd0(&self) -> Option<Owned<pin::Ptd0>> { pin::Ptd0::acquire() }
        pub fn ptd1(&self) -> Option<Owned<pin::Ptd1>> { pin::Ptd1::acquire() }
        pub fn ptd2(&self) -> Option<Owned<pin::Ptd2>> { pin::Ptd2::acquire() }
        pub fn ptd3(&self) -> Option<Owned<pin::Ptd3>> { pin::Ptd3::acquire() }
        pub fn ptd4(&self) -> Option<Owned<pin::Ptd4>> { pin::Ptd4::acquire() }
        pub fn ptd5(&self) -> Option<Owned<pin::Ptd5>> { pin::Ptd5::acquire() }
        pub fn ptd6(&self) -> Option<Owned<pin::Ptd6>> { pin::Ptd6::acquire() }
        pub fn ptd7(&self) -> Option<Owned<pin::Ptd7>> { pin::Ptd7::acquire() }
    pub fn porte(&self) -> Option<Owned<port::Porte>> { port::Porte::acquire() }
        pub fn pte0(&self) -> Option<Owned<pin::Pte0>> { pin::Pte0::acquire() }
        pub fn pte1(&self) -> Option<Owned<pin::Pte1>> { pin::Pte1::acquire() }
        pub fn pte20(&self) -> Option<Owned<pin::Pte20>> { pin::Pte20::acquire() }
        pub fn pte21(&self) -> Option<Owned<pin::Pte21>> { pin::Pte21::acquire() }
        pub fn pte22(&self) -> Option<Owned<pin::Pte22>> { pin::Pte22::acquire() }
        pub fn pte23(&self) -> Option<Owned<pin::Pte23>> { pin::Pte23::acquire() }
        pub fn pte24(&self) -> Option<Owned<pin::Pte24>> { pin::Pte24::acquire() }
        pub fn pte25(&self) -> Option<Owned<pin::Pte25>> { pin::Pte25::acquire() }
        pub fn pte29(&self) -> Option<Owned<pin::Pte29>> { pin::Pte29::acquire() }
        pub fn pte30(&self) -> Option<Owned<pin::Pte30>> { pin::Pte30::acquire() }
        pub fn pte31(&self) -> Option<Owned<pin::Pte31>> { pin::Pte31::acquire() }
    pub fn gpioa(&self) -> Option<Owned<gpio::Gpioa>> { gpio::Gpioa::acquire() }
        pub fn pa0(&self) -> Option<Owned<pin::Pa0>> { pin::Pa0::acquire() }
        pub fn pa1(&self) -> Option<Owned<pin::Pa1>> { pin::Pa1::acquire() }
        pub fn pa2(&self) -> Option<Owned<pin::Pa2>> { pin::Pa2::acquire() }
        pub fn pa3(&self) -> Option<Owned<pin::Pa3>> { pin::Pa3::acquire() }
        pub fn pa4(&self) -> Option<Owned<pin::Pa4>> { pin::Pa4::acquire() }
        pub fn pa5(&self) -> Option<Owned<pin::Pa5>> { pin::Pa5::acquire() }
        pub fn pa12(&self) -> Option<Owned<pin::Pa12>> { pin::Pa12::acquire() }
        pub fn pa13(&self) -> Option<Owned<pin::Pa13>> { pin::Pa13::acquire() }
        pub fn pa18(&self) -> Option<Owned<pin::Pa18>> { pin::Pa18::acquire() }
        pub fn pa19(&self) -> Option<Owned<pin::Pa19>> { pin::Pa19::acquire() }
        pub fn pa20(&self) -> Option<Owned<pin::Pa20>> { pin::Pa20::acquire() }
    pub fn gpiob(&self) -> Option<Owned<gpio::Gpiob>> { gpio::Gpiob::acquire() }
        pub fn pb0(&self) -> Option<Owned<pin::Pb0>> { pin::Pb0::acquire() }
        pub fn pb1(&self) -> Option<Owned<pin::Pb1>> { pin::Pb1::acquire() }
        pub fn pb2(&self) -> Option<Owned<pin::Pb2>> { pin::Pb2::acquire() }
        pub fn pb3(&self) -> Option<Owned<pin::Pb3>> { pin::Pb3::acquire() }
        pub fn pb16(&self) -> Option<Owned<pin::Pb16>> { pin::Pb16::acquire() }
        pub fn pb17(&self) -> Option<Owned<pin::Pb17>> { pin::Pb17::acquire() }
        pub fn pb18(&self) -> Option<Owned<pin::Pb18>> { pin::Pb18::acquire() }
        pub fn pb19(&self) -> Option<Owned<pin::Pb19>> { pin::Pb19::acquire() }
    pub fn gpioc(&self) -> Option<Owned<gpio::Gpioc>> { gpio::Gpioc::acquire() }
        pub fn pc0(&self) -> Option<Owned<pin::Pc0>> { pin::Pc0::acquire() }
        pub fn pc1(&self) -> Option<Owned<pin::Pc1>> { pin::Pc1::acquire() }
        pub fn pc2(&self) -> Option<Owned<pin::Pc2>> { pin::Pc2::acquire() }
        pub fn pc3(&self) -> Option<Owned<pin::Pc3>> { pin::Pc3::acquire() }
        pub fn pc4(&self) -> Option<Owned<pin::Pc4>> { pin::Pc4::acquire() }
        pub fn pc5(&self) -> Option<Owned<pin::Pc5>> { pin::Pc5::acquire() }
        pub fn pc6(&self) -> Option<Owned<pin::Pc6>> { pin::Pc6::acquire() }
        pub fn pc7(&self) -> Option<Owned<pin::Pc7>> { pin::Pc7::acquire() }
        pub fn pc8(&self) -> Option<Owned<pin::Pc8>> { pin::Pc8::acquire() }
        pub fn pc9(&self) -> Option<Owned<pin::Pc9>> { pin::Pc9::acquire() }
        pub fn pc10(&self) -> Option<Owned<pin::Pc10>> { pin::Pc10::acquire() }
        pub fn pc11(&self) -> Option<Owned<pin::Pc11>> { pin::Pc11::acquire() }
    pub fn gpiod(&self) -> Option<Owned<gpio::Gpiod>> { gpio::Gpiod::acquire() }
        pub fn pd0(&self) -> Option<Owned<pin::Pd0>> { pin::Pd0::acquire() }
        pub fn pd1(&self) -> Option<Owned<pin::Pd1>> { pin::Pd1::acquire() }
        pub fn pd2(&self) -> Option<Owned<pin::Pd2>> { pin::Pd2::acquire() }
        pub fn pd3(&self) -> Option<Owned<pin::Pd3>> { pin::Pd3::acquire() }
        pub fn pd4(&self) -> Option<Owned<pin::Pd4>> { pin::Pd4::acquire() }
        pub fn pd5(&self) -> Option<Owned<pin::Pd5>> { pin::Pd5::acquire() }
        pub fn pd6(&self) -> Option<Owned<pin::Pd6>> { pin::Pd6::acquire() }
        pub fn pd7(&self) -> Option<Owned<pin::Pd7>> { pin::Pd7::acquire() }
    pub fn gpioe(&self) -> Option<Owned<gpio::Gpioe>> { gpio::Gpioe::acquire() }
        pub fn pe0(&self) -> Option<Owned<pin::Pe0>> { pin::Pe0::acquire() }
        pub fn pe1(&self) -> Option<Owned<pin::Pe1>> { pin::Pe1::acquire() }
        pub fn pe20(&self) -> Option<Owned<pin::Pe20>> { pin::Pe20::acquire() }
        pub fn pe21(&self) -> Option<Owned<pin::Pe21>> { pin::Pe21::acquire() }
        pub fn pe22(&self) -> Option<Owned<pin::Pe22>> { pin::Pe22::acquire() }
        pub fn pe23(&self) -> Option<Owned<pin::Pe23>> { pin::Pe23::acquire() }
        pub fn pe24(&self) -> Option<Owned<pin::Pe24>> { pin::Pe24::acquire() }
        pub fn pe25(&self) -> Option<Owned<pin::Pe25>> { pin::Pe25::acquire() }
        pub fn pe29(&self) -> Option<Owned<pin::Pe29>> { pin::Pe29::acquire() }
        pub fn pe30(&self) -> Option<Owned<pin::Pe30>> { pin::Pe30::acquire() }
        pub fn pe31(&self) -> Option<Owned<pin::Pe31>> { pin::Pe31::acquire() }
    pub fn uart0(&self) -> Option<Owned<uart0::Uart0>> { uart0::Uart0::acquire() }
    pub fn uart1(&self) -> Option<Owned<uart::Uart1>> { uart::Uart1::acquire() }
    pub fn uart2(&self) -> Option<Owned<uart::Uart2>> { uart::Uart2::acquire() }
    pub fn adc0(&self) -> Option<Owned<adc::Adc0>> { adc::Adc0::acquire() }
        pub fn adc0_ch0(&self) -> Option<Owned<adc::Adc0Ch0>> { adc::Adc0Ch0::acquire() }
        pub fn adc0_ch1(&self) -> Option<Owned<adc::Adc0Ch1>> { adc::Adc0Ch1::acquire() }
        pub fn adc0_ch2(&self) -> Option<Owned<adc::Adc0Ch2>> { adc::Adc0Ch2::acquire() }
        pub fn adc0_ch3(&self) -> Option<Owned<adc::Adc0Ch3>> { adc::Adc0Ch3::acquire() }
        pub fn adc0_ch4(&self) -> Option<Owned<adc::Adc0Ch4>> { adc::Adc0Ch4::acquire() }
        pub fn adc0_ch5(&self) -> Option<Owned<adc::Adc0Ch5>> { adc::Adc0Ch5::acquire() }
        pub fn adc0_ch6(&self) -> Option<Owned<adc::Adc0Ch6>> { adc::Adc0Ch6::acquire() }
        pub fn adc0_ch7(&self) -> Option<Owned<adc::Adc0Ch7>> { adc::Adc0Ch7::acquire() }
        pub fn adc0_ch8(&self) -> Option<Owned<adc::Adc0Ch8>> { adc::Adc0Ch8::acquire() }
        pub fn adc0_ch9(&self) -> Option<Owned<adc::Adc0Ch9>> { adc::Adc0Ch9::acquire() }
        pub fn adc0_ch10(&self) -> Option<Owned<adc::Adc0Ch10>> { adc::Adc0Ch10::acquire() }
        pub fn adc0_ch11(&self) -> Option<Owned<adc::Adc0Ch11>> { adc::Adc0Ch11::acquire() }
        pub fn adc0_ch12(&self) -> Option<Owned<adc::Adc0Ch12>> { adc::Adc0Ch12::acquire() }
        pub fn adc0_ch13(&self) -> Option<Owned<adc::Adc0Ch13>> { adc::Adc0Ch13::acquire() }
        pub fn adc0_ch14(&self) -> Option<Owned<adc::Adc0Ch14>> { adc::Adc0Ch14::acquire() }
        pub fn adc0_ch15(&self) -> Option<Owned<adc::Adc0Ch15>> { adc::Adc0Ch15::acquire() }
        pub fn adc0_ch16(&self) -> Option<Owned<adc::Adc0Ch16>> { adc::Adc0Ch16::acquire() }
        pub fn adc0_ch17(&self) -> Option<Owned<adc::Adc0Ch17>> { adc::Adc0Ch17::acquire() }
        pub fn adc0_ch18(&self) -> Option<Owned<adc::Adc0Ch18>> { adc::Adc0Ch18::acquire() }
        pub fn adc0_ch19(&self) -> Option<Owned<adc::Adc0Ch19>> { adc::Adc0Ch19::acquire() }
        pub fn adc0_ch20(&self) -> Option<Owned<adc::Adc0Ch20>> { adc::Adc0Ch20::acquire() }
        pub fn adc0_ch21(&self) -> Option<Owned<adc::Adc0Ch21>> { adc::Adc0Ch21::acquire() }
        pub fn adc0_ch22(&self) -> Option<Owned<adc::Adc0Ch22>> { adc::Adc0Ch22::acquire() }
        pub fn adc0_ch23(&self) -> Option<Owned<adc::Adc0Ch23>> { adc::Adc0Ch23::acquire() }
        pub fn adc0_temp(&self) -> Option<Owned<adc::Adc0Temp>> { adc::Adc0Temp::acquire() }
        pub fn adc0_bandgap(&self) -> Option<Owned<adc::Adc0Bandgap>> { adc::Adc0Bandgap::acquire() }
        pub fn adc0_refsh(&self) -> Option<Owned<adc::Adc0Refsh>> { adc::Adc0Refsh::acquire() }
        pub fn adc0_refsl(&self) -> Option<Owned<adc::Adc0Refsl>> { adc::Adc0Refsl::acquire() }
}

impl Get<ftfa::Ftfa> for Kl26 {
    fn get(&self) -> ftfa::Ftfa { ftfa::FTFA }
}

impl GetPeriph<ftfa::FtfaPeriph> for Kl26 {
    fn get_periph(&self) -> ftfa::FtfaPeriph { ftfa::FTFA_PERIPH }
}

impl GetPeriphInstance<ftfa::FtfaPeriph> for Kl26 {
    fn get_periph_instance(&self, index: usize) -> Option<ftfa::FtfaPeriph> {
        match index { 
            0 => Some(ftfa::FTFA_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<osc::Osc> for Kl26 {
    fn get(&self) -> osc::Osc { osc::OSC0 }
}

impl GetPeriph<osc::OscPeriph> for Kl26 {
    fn get_periph(&self) -> osc::OscPeriph { osc::OSC0_PERIPH }
}

impl GetPeriphInstance<osc::OscPeriph> for Kl26 {
    fn get_periph_instance(&self, index: usize) -> Option<osc::OscPeriph> {
        match index { 
            0 => Some(osc::OSC0_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<sim::Sim> for Kl26 {
    fn get(&self) -> sim::Sim { sim::SIM }
}

impl GetPeriph<sim::SimPeriph> for Kl26 {
    fn get_periph(&self) -> sim::SimPeriph { sim::SIM_PERIPH }
}

impl GetPeriphInstance<sim::SimPeriph> for Kl26 {
    fn get_periph_instance(&self, index: usize) -> Option<sim::SimPeriph> {
        match index { 
            0 => Some(sim::SIM_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<mcg::Mcg> for Kl26 {
    fn get(&self) -> mcg::Mcg { mcg::MCG }
}

impl GetPeriph<mcg::McgPeriph> for Kl26 {
    fn get_periph(&self) -> mcg::McgPeriph { mcg::MCG_PERIPH }
}

impl GetPeriphInstance<mcg::McgPeriph> for Kl26 {
    fn get_periph_instance(&self, index: usize) -> Option<mcg::McgPeriph> {
        match index { 
            0 => Some(mcg::MCG_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<rcm::Rcm> for Kl26 {
    fn get(&self) -> rcm::Rcm { rcm::RCM }
}

impl GetPeriph<rcm::RcmPeriph> for Kl26 {
    fn get_periph(&self) -> rcm::RcmPeriph { rcm::RCM_PERIPH }
}

impl GetPeriphInstance<rcm::RcmPeriph> for Kl26 {
    fn get_periph_instance(&self, index: usize) -> Option<rcm::RcmPeriph> {
        match index { 
            0 => Some(rcm::RCM_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<dmamux::Dmamux> for Kl26 {
    fn get(&self) -> dmamux::Dmamux { dmamux::DMAMUX }
}

impl GetPeriph<dmamux::DmamuxPeriph> for Kl26 {
    fn get_periph(&self) -> dmamux::DmamuxPeriph { dmamux::DMAMUX_PERIPH }
}

impl GetPeriphInstance<dmamux::DmamuxPeriph> for Kl26 {
    fn get_periph_instance(&self, index: usize) -> Option<dmamux::DmamuxPeriph> {
        match index {
            0 => Some(dmamux::DMAMUX_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<dma::Dma> for Kl26 {
    fn get(&self) -> dma::Dma { dma::DMA }
}

impl GetPeriph<dma::DmaPeriph> for Kl26 {
    fn get_periph(&self) -> dma::DmaPeriph { dma::DMA_PERIPH }
}

impl GetPeriphInstance<dma::DmaPeriph> for Kl26 {
    fn get_periph_instance(&self, index: usize) -> Option<dma::DmaPeriph> {
        match index {
            0 => Some(dma::DMA_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<tpm::Tpm0> for Kl26 {
    fn get(&self) -> tpm::Tpm0 { tpm::TPM0 }
}

impl Get<tpm::Tpm1> for Kl26 {
    fn get(&self) -> tpm::Tpm1 { tpm::TPM1 }
}

impl Get<tpm::Tpm2> for Kl26 {
    fn get(&self) -> tpm::Tpm2 { tpm::TPM2 }
}

impl GetPeriphInstance<tpm::TpmPeriph> for Kl26 {
    fn get_periph_instance(&self, index: usize) -> Option<tpm::TpmPeriph> {
        match index {
            0 => Some(tpm::TPM0_PERIPH),
            1 => Some(tpm::TPM1_PERIPH),
            2 => Some(tpm::TPM2_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 3 }
}

impl Get<pit::Pit> for Kl26 {
    fn get(&self) -> pit::Pit { pit::PIT }
}

impl GetPeriph<pit::PitPeriph> for Kl26 {
    fn get_periph(&self) -> pit::PitPeriph { pit::PIT_PERIPH }
}

impl GetPeriphInstance<pit::PitPeriph> for Kl26 {
    fn get_periph_instance(&self, index: usize) -> Option<pit::PitPeriph> {
        match index {
            0 => Some(pit::PIT_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<lptmr::Lptmr0> for Kl26 {
    fn get(&self) -> lptmr::Lptmr0 { lptmr::LPTMR0 }
}

impl GetPeriph<lptmr::LptmrPeriph> for Kl26 {
    fn get_periph(&self) -> lptmr::LptmrPeriph { lptmr::LPTMR0_PERIPH }
}

impl GetPeriphInstance<lptmr::LptmrPeriph> for Kl26 {
    fn get_periph_instance(&self, index: usize) -> Option<lptmr::LptmrPeriph> {
        match index {
            0 => Some(lptmr::LPTMR0_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<spi::Spi0> for Kl26 {
    fn get(&self) -> spi::Spi0 { spi::SPI0 }
}

impl Get<spi::Spi1> for Kl26 {
    fn get(&self) -> spi::Spi1 { spi::SPI1 }
}

impl GetPeriphInstance<spi::SpiPeriph> for Kl26 {
    fn get_periph_instance(&self, index: usize) -> Option<spi::SpiPeriph> {
        match index {
            0 => Some(spi::SPI0_PERIPH),
            1 => Some(spi::SPI1_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

impl Get<i2c::I2c0> for Kl26 {
    fn get(&self) -> i2c::I2c0 { i2c::I2C0 }
}

impl Get<i2c::I2c1> for Kl26 {
    fn get(&self) -> i2c::I2c1 { i2c::I2C1 }
}

impl GetPeriphInstance<i2c::I2cPeriph> for Kl26 {
    fn get_periph_instance(&self, index: usize) -> Option<i2c::I2cPeriph> {
        match index {
            0 => Some(i2c::I2C0_PERIPH),
            1 => Some(i2c::I2C1_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

impl Get<port::Porta> for Kl26 {
    fn get(&self) -> port::Porta { port::PORTA }
}

impl Get<port::Portb> for Kl26 {
    fn get(&self) -> port::Portb { port::PORTB }
}

impl Get<port::Portc> for Kl26 {
    fn get(&self) -> port::Portc { port::PORTC }
}

impl Get<port::Portd> for Kl26 {
    fn get(&self) -> port::Portd { port::PORTD }
}

impl Get<port::Porte> for Kl26 {
    fn get(&self) -> port::Porte { port::PORTE }
}

impl GetPeriphInstance<port::PortPeriph> for Kl26 {
    fn get_periph_instance(&self, index: usize) -> Option<port::PortPeriph> {
        match index {
            0 => Some(port::PORTA_PERIPH),
            1 => Some(port::PORTB_PERIPH),
            2 => Some(port::PORTC_PERIPH),
            3 => Some(port::PORTD_PERIPH),
            4 => Some(port::PORTE_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 5 }
}

impl Get<gpio::Gpioa> for Kl26 {
    fn get(&self) -> gpio::Gpioa { gpio::GPIOA }
}

impl Get<gpio::Gpiob> for Kl26 {
    fn get(&self) -> gpio::Gpiob { gpio::GPIOB }
}

impl Get<gpio::Gpioc> for Kl26 {
    fn get(&self) -> gpio::Gpioc { gpio::GPIOC }
}

impl Get<gpio::Gpiod> for Kl26 {
    fn get(&self) -> gpio::Gpiod { gpio::GPIOD }
}

impl Get<gpio::Gpioe> for Kl26 {
    fn get(&self) -> gpio::Gpioe { gpio::GPIOE }
}

impl GetPeriphInstance<gpio::GpioPeriph> for Kl26 {
    fn get_periph_instance(&self, index: usize) -> Option<gpio::GpioPeriph> {
        match index {
            0 => Some(gpio::GPIOA_PERIPH),
            1 => Some(gpio::GPIOB_PERIPH),
            2 => Some(gpio::GPIOC_PERIPH),
            3 => Some(gpio::GPIOD_PERIPH),
            4 => Some(gpio::GPIOE_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 5 }
}

impl Get<uart0::Uart0> for Kl26 {
    fn get(&self) -> uart0::Uart0 { uart0::UART0 }
}

impl GetPeriph<uart0::Uart0Periph> for Kl26 {
    fn get_periph(&self) -> uart0::Uart0Periph { uart0::UART0_PERIPH }
}

impl GetPeriphInstance<uart0::Uart0Periph> for Kl26 {
    fn get_periph_instance(&self, index: usize) -> Option<uart0::Uart0Periph> {
        match index {
            0 => Some(uart0::UART0_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<uart::Uart1> for Kl26 {
    fn get(&self) -> uart::Uart1 { uart::UART1 }
}

impl Get<uart::Uart2> for Kl26 {
    fn get(&self) -> uart::Uart2 { uart::UART2 }
}

impl GetPeriphInstance<uart::UartPeriph> for Kl26 {
    fn get_periph_instance(&self, index: usize) -> Option<uart::UartPeriph> {
        match index {
            0 => Some(uart::UART1_PERIPH),
            1 => Some(uart::UART2_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

impl Get<adc::Adc0> for Kl26 {
    fn get(&self) -> adc::Adc0 { adc::ADC0 }
}

impl GetPeriph<adc::AdcPeriph> for Kl26 {
    fn get_periph(&self) -> adc::AdcPeriph { adc::ADC0_PERIPH }
}

impl GetPeriphInstance<adc::AdcPeriph> for Kl26 {
    fn get_periph_instance(&self, index: usize) -> Option<adc::AdcPeriph> {
        match index {
            0 => Some(adc::ADC0_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

