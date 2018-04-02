pub use ::bobbin_common::mcu::*;
use ::bobbin_common::owned::*;

pub mod sim;
pub mod mcg;
pub mod mpu;
pub mod osc;
pub mod rcm;
pub mod enet;
pub mod crc;
pub mod wdog;
pub mod dmamux;
pub mod edma;
pub mod ftm;
pub mod pit;
pub mod lptmr;
pub mod spi;
pub mod i2c;
pub mod uart;
pub mod usb;
pub mod flexcan;
pub mod dac;
pub mod gpio;
pub mod port;
pub mod adc;
pub mod sig;
pub mod pin;
pub mod irq;

#[derive(Debug, Default)]
pub struct K64 {}

impl Mcu for K64 {
    fn id(&self) -> &'static str { "K64" }
}

impl K64 {
    pub fn sim(&self) -> Option<Owned<sim::Sim>> { sim::Sim::acquire() }
    pub fn mcg(&self) -> Option<Owned<mcg::Mcg>> { mcg::Mcg::acquire() }
    pub fn mpu(&self) -> Option<Owned<mpu::Mpu>> { mpu::Mpu::acquire() }
    pub fn osc(&self) -> Option<Owned<osc::Osc>> { osc::Osc::acquire() }
    pub fn rcm(&self) -> Option<Owned<rcm::Rcm>> { rcm::Rcm::acquire() }
    pub fn enet(&self) -> Option<Owned<enet::Enet>> { enet::Enet::acquire() }
    pub fn crc(&self) -> Option<Owned<crc::Crc>> { crc::Crc::acquire() }
    pub fn wdog(&self) -> Option<Owned<wdog::Wdog>> { wdog::Wdog::acquire() }
    pub fn dmamux(&self) -> Option<Owned<dmamux::Dmamux>> { dmamux::Dmamux::acquire() }
    pub fn dma(&self) -> Option<Owned<edma::Dma>> { edma::Dma::acquire() }
        pub fn dma0(&self) -> Option<Owned<edma::Dma0>> { edma::Dma0::acquire() }
        pub fn dma1(&self) -> Option<Owned<edma::Dma1>> { edma::Dma1::acquire() }
        pub fn dma2(&self) -> Option<Owned<edma::Dma2>> { edma::Dma2::acquire() }
        pub fn dma3(&self) -> Option<Owned<edma::Dma3>> { edma::Dma3::acquire() }
        pub fn dma4(&self) -> Option<Owned<edma::Dma4>> { edma::Dma4::acquire() }
        pub fn dma5(&self) -> Option<Owned<edma::Dma5>> { edma::Dma5::acquire() }
        pub fn dma6(&self) -> Option<Owned<edma::Dma6>> { edma::Dma6::acquire() }
        pub fn dma7(&self) -> Option<Owned<edma::Dma7>> { edma::Dma7::acquire() }
        pub fn dma8(&self) -> Option<Owned<edma::Dma8>> { edma::Dma8::acquire() }
        pub fn dma9(&self) -> Option<Owned<edma::Dma9>> { edma::Dma9::acquire() }
        pub fn dma10(&self) -> Option<Owned<edma::Dma10>> { edma::Dma10::acquire() }
        pub fn dma11(&self) -> Option<Owned<edma::Dma11>> { edma::Dma11::acquire() }
        pub fn dma12(&self) -> Option<Owned<edma::Dma12>> { edma::Dma12::acquire() }
        pub fn dma13(&self) -> Option<Owned<edma::Dma13>> { edma::Dma13::acquire() }
        pub fn dma14(&self) -> Option<Owned<edma::Dma14>> { edma::Dma14::acquire() }
        pub fn dma15(&self) -> Option<Owned<edma::Dma15>> { edma::Dma15::acquire() }
    pub fn ftm0(&self) -> Option<Owned<ftm::Ftm0>> { ftm::Ftm0::acquire() }
        pub fn ftm0_ch0(&self) -> Option<Owned<ftm::Ftm0Ch0>> { ftm::Ftm0Ch0::acquire() }
        pub fn ftm0_ch1(&self) -> Option<Owned<ftm::Ftm0Ch1>> { ftm::Ftm0Ch1::acquire() }
        pub fn ftm0_ch2(&self) -> Option<Owned<ftm::Ftm0Ch2>> { ftm::Ftm0Ch2::acquire() }
        pub fn ftm0_ch3(&self) -> Option<Owned<ftm::Ftm0Ch3>> { ftm::Ftm0Ch3::acquire() }
        pub fn ftm0_ch4(&self) -> Option<Owned<ftm::Ftm0Ch4>> { ftm::Ftm0Ch4::acquire() }
        pub fn ftm0_ch5(&self) -> Option<Owned<ftm::Ftm0Ch5>> { ftm::Ftm0Ch5::acquire() }
        pub fn ftm0_ch6(&self) -> Option<Owned<ftm::Ftm0Ch6>> { ftm::Ftm0Ch6::acquire() }
        pub fn ftm0_ch7(&self) -> Option<Owned<ftm::Ftm0Ch7>> { ftm::Ftm0Ch7::acquire() }
    pub fn ftm1(&self) -> Option<Owned<ftm::Ftm1>> { ftm::Ftm1::acquire() }
        pub fn ftm1_ch0(&self) -> Option<Owned<ftm::Ftm1Ch0>> { ftm::Ftm1Ch0::acquire() }
        pub fn ftm1_ch1(&self) -> Option<Owned<ftm::Ftm1Ch1>> { ftm::Ftm1Ch1::acquire() }
        pub fn ftm1_ch2(&self) -> Option<Owned<ftm::Ftm1Ch2>> { ftm::Ftm1Ch2::acquire() }
        pub fn ftm1_ch3(&self) -> Option<Owned<ftm::Ftm1Ch3>> { ftm::Ftm1Ch3::acquire() }
        pub fn ftm1_ch4(&self) -> Option<Owned<ftm::Ftm1Ch4>> { ftm::Ftm1Ch4::acquire() }
        pub fn ftm1_ch5(&self) -> Option<Owned<ftm::Ftm1Ch5>> { ftm::Ftm1Ch5::acquire() }
        pub fn ftm1_ch6(&self) -> Option<Owned<ftm::Ftm1Ch6>> { ftm::Ftm1Ch6::acquire() }
        pub fn ftm1_ch7(&self) -> Option<Owned<ftm::Ftm1Ch7>> { ftm::Ftm1Ch7::acquire() }
    pub fn ftm2(&self) -> Option<Owned<ftm::Ftm2>> { ftm::Ftm2::acquire() }
        pub fn ftm2_ch0(&self) -> Option<Owned<ftm::Ftm2Ch0>> { ftm::Ftm2Ch0::acquire() }
        pub fn ftm2_ch1(&self) -> Option<Owned<ftm::Ftm2Ch1>> { ftm::Ftm2Ch1::acquire() }
        pub fn ftm2_ch2(&self) -> Option<Owned<ftm::Ftm2Ch2>> { ftm::Ftm2Ch2::acquire() }
        pub fn ftm2_ch3(&self) -> Option<Owned<ftm::Ftm2Ch3>> { ftm::Ftm2Ch3::acquire() }
        pub fn ftm2_ch4(&self) -> Option<Owned<ftm::Ftm2Ch4>> { ftm::Ftm2Ch4::acquire() }
        pub fn ftm2_ch5(&self) -> Option<Owned<ftm::Ftm2Ch5>> { ftm::Ftm2Ch5::acquire() }
        pub fn ftm2_ch6(&self) -> Option<Owned<ftm::Ftm2Ch6>> { ftm::Ftm2Ch6::acquire() }
        pub fn ftm2_ch7(&self) -> Option<Owned<ftm::Ftm2Ch7>> { ftm::Ftm2Ch7::acquire() }
    pub fn pit(&self) -> Option<Owned<pit::Pit>> { pit::Pit::acquire() }
        pub fn pit_ch0(&self) -> Option<Owned<pit::PitCh0>> { pit::PitCh0::acquire() }
        pub fn pit_ch1(&self) -> Option<Owned<pit::PitCh1>> { pit::PitCh1::acquire() }
        pub fn pit_ch2(&self) -> Option<Owned<pit::PitCh2>> { pit::PitCh2::acquire() }
        pub fn pit_ch3(&self) -> Option<Owned<pit::PitCh3>> { pit::PitCh3::acquire() }
    pub fn lptmr0(&self) -> Option<Owned<lptmr::Lptmr0>> { lptmr::Lptmr0::acquire() }
    pub fn spi0(&self) -> Option<Owned<spi::Spi0>> { spi::Spi0::acquire() }
    pub fn spi1(&self) -> Option<Owned<spi::Spi1>> { spi::Spi1::acquire() }
    pub fn spi2(&self) -> Option<Owned<spi::Spi2>> { spi::Spi2::acquire() }
    pub fn i2c0(&self) -> Option<Owned<i2c::I2c0>> { i2c::I2c0::acquire() }
    pub fn i2c1(&self) -> Option<Owned<i2c::I2c1>> { i2c::I2c1::acquire() }
    pub fn uart0(&self) -> Option<Owned<uart::Uart0>> { uart::Uart0::acquire() }
    pub fn uart1(&self) -> Option<Owned<uart::Uart1>> { uart::Uart1::acquire() }
    pub fn uart2(&self) -> Option<Owned<uart::Uart2>> { uart::Uart2::acquire() }
    pub fn uart3(&self) -> Option<Owned<uart::Uart3>> { uart::Uart3::acquire() }
    pub fn uart4(&self) -> Option<Owned<uart::Uart4>> { uart::Uart4::acquire() }
    pub fn uart5(&self) -> Option<Owned<uart::Uart5>> { uart::Uart5::acquire() }
    pub fn usb0(&self) -> Option<Owned<usb::Usb0>> { usb::Usb0::acquire() }
    pub fn can0(&self) -> Option<Owned<flexcan::Can0>> { flexcan::Can0::acquire() }
    pub fn dac0(&self) -> Option<Owned<dac::Dac0>> { dac::Dac0::acquire() }
    pub fn dac1(&self) -> Option<Owned<dac::Dac1>> { dac::Dac1::acquire() }
    pub fn gpioa(&self) -> Option<Owned<gpio::Gpioa>> { gpio::Gpioa::acquire() }
        pub fn pa0(&self) -> Option<Owned<gpio::Pa0>> { gpio::Pa0::acquire() }
        pub fn pa1(&self) -> Option<Owned<gpio::Pa1>> { gpio::Pa1::acquire() }
        pub fn pa2(&self) -> Option<Owned<gpio::Pa2>> { gpio::Pa2::acquire() }
        pub fn pa3(&self) -> Option<Owned<gpio::Pa3>> { gpio::Pa3::acquire() }
        pub fn pa4(&self) -> Option<Owned<gpio::Pa4>> { gpio::Pa4::acquire() }
        pub fn pa5(&self) -> Option<Owned<gpio::Pa5>> { gpio::Pa5::acquire() }
        pub fn pa6(&self) -> Option<Owned<gpio::Pa6>> { gpio::Pa6::acquire() }
        pub fn pa7(&self) -> Option<Owned<gpio::Pa7>> { gpio::Pa7::acquire() }
        pub fn pa8(&self) -> Option<Owned<gpio::Pa8>> { gpio::Pa8::acquire() }
        pub fn pa9(&self) -> Option<Owned<gpio::Pa9>> { gpio::Pa9::acquire() }
        pub fn pa10(&self) -> Option<Owned<gpio::Pa10>> { gpio::Pa10::acquire() }
        pub fn pa11(&self) -> Option<Owned<gpio::Pa11>> { gpio::Pa11::acquire() }
        pub fn pa12(&self) -> Option<Owned<gpio::Pa12>> { gpio::Pa12::acquire() }
        pub fn pa13(&self) -> Option<Owned<gpio::Pa13>> { gpio::Pa13::acquire() }
        pub fn pa14(&self) -> Option<Owned<gpio::Pa14>> { gpio::Pa14::acquire() }
        pub fn pa15(&self) -> Option<Owned<gpio::Pa15>> { gpio::Pa15::acquire() }
        pub fn pa16(&self) -> Option<Owned<gpio::Pa16>> { gpio::Pa16::acquire() }
        pub fn pa17(&self) -> Option<Owned<gpio::Pa17>> { gpio::Pa17::acquire() }
        pub fn pa18(&self) -> Option<Owned<gpio::Pa18>> { gpio::Pa18::acquire() }
        pub fn pa19(&self) -> Option<Owned<gpio::Pa19>> { gpio::Pa19::acquire() }
        pub fn pa24(&self) -> Option<Owned<gpio::Pa24>> { gpio::Pa24::acquire() }
        pub fn pa25(&self) -> Option<Owned<gpio::Pa25>> { gpio::Pa25::acquire() }
        pub fn pa26(&self) -> Option<Owned<gpio::Pa26>> { gpio::Pa26::acquire() }
        pub fn pa27(&self) -> Option<Owned<gpio::Pa27>> { gpio::Pa27::acquire() }
        pub fn pa28(&self) -> Option<Owned<gpio::Pa28>> { gpio::Pa28::acquire() }
        pub fn pa29(&self) -> Option<Owned<gpio::Pa29>> { gpio::Pa29::acquire() }
    pub fn gpiob(&self) -> Option<Owned<gpio::Gpiob>> { gpio::Gpiob::acquire() }
        pub fn pb0(&self) -> Option<Owned<gpio::Pb0>> { gpio::Pb0::acquire() }
        pub fn pb1(&self) -> Option<Owned<gpio::Pb1>> { gpio::Pb1::acquire() }
        pub fn pb2(&self) -> Option<Owned<gpio::Pb2>> { gpio::Pb2::acquire() }
        pub fn pb3(&self) -> Option<Owned<gpio::Pb3>> { gpio::Pb3::acquire() }
        pub fn pb4(&self) -> Option<Owned<gpio::Pb4>> { gpio::Pb4::acquire() }
        pub fn pb5(&self) -> Option<Owned<gpio::Pb5>> { gpio::Pb5::acquire() }
        pub fn pb6(&self) -> Option<Owned<gpio::Pb6>> { gpio::Pb6::acquire() }
        pub fn pb7(&self) -> Option<Owned<gpio::Pb7>> { gpio::Pb7::acquire() }
        pub fn pb8(&self) -> Option<Owned<gpio::Pb8>> { gpio::Pb8::acquire() }
        pub fn pb9(&self) -> Option<Owned<gpio::Pb9>> { gpio::Pb9::acquire() }
        pub fn pb10(&self) -> Option<Owned<gpio::Pb10>> { gpio::Pb10::acquire() }
        pub fn pb11(&self) -> Option<Owned<gpio::Pb11>> { gpio::Pb11::acquire() }
        pub fn pb12(&self) -> Option<Owned<gpio::Pb12>> { gpio::Pb12::acquire() }
        pub fn pb13(&self) -> Option<Owned<gpio::Pb13>> { gpio::Pb13::acquire() }
        pub fn pb16(&self) -> Option<Owned<gpio::Pb16>> { gpio::Pb16::acquire() }
        pub fn pb17(&self) -> Option<Owned<gpio::Pb17>> { gpio::Pb17::acquire() }
        pub fn pb18(&self) -> Option<Owned<gpio::Pb18>> { gpio::Pb18::acquire() }
        pub fn pb19(&self) -> Option<Owned<gpio::Pb19>> { gpio::Pb19::acquire() }
        pub fn pb20(&self) -> Option<Owned<gpio::Pb20>> { gpio::Pb20::acquire() }
        pub fn pb21(&self) -> Option<Owned<gpio::Pb21>> { gpio::Pb21::acquire() }
        pub fn pb22(&self) -> Option<Owned<gpio::Pb22>> { gpio::Pb22::acquire() }
        pub fn pb23(&self) -> Option<Owned<gpio::Pb23>> { gpio::Pb23::acquire() }
    pub fn gpioc(&self) -> Option<Owned<gpio::Gpioc>> { gpio::Gpioc::acquire() }
        pub fn pc0(&self) -> Option<Owned<gpio::Pc0>> { gpio::Pc0::acquire() }
        pub fn pc1(&self) -> Option<Owned<gpio::Pc1>> { gpio::Pc1::acquire() }
        pub fn pc2(&self) -> Option<Owned<gpio::Pc2>> { gpio::Pc2::acquire() }
        pub fn pc3(&self) -> Option<Owned<gpio::Pc3>> { gpio::Pc3::acquire() }
        pub fn pc4(&self) -> Option<Owned<gpio::Pc4>> { gpio::Pc4::acquire() }
        pub fn pc5(&self) -> Option<Owned<gpio::Pc5>> { gpio::Pc5::acquire() }
        pub fn pc6(&self) -> Option<Owned<gpio::Pc6>> { gpio::Pc6::acquire() }
        pub fn pc7(&self) -> Option<Owned<gpio::Pc7>> { gpio::Pc7::acquire() }
        pub fn pc8(&self) -> Option<Owned<gpio::Pc8>> { gpio::Pc8::acquire() }
        pub fn pc9(&self) -> Option<Owned<gpio::Pc9>> { gpio::Pc9::acquire() }
        pub fn pc10(&self) -> Option<Owned<gpio::Pc10>> { gpio::Pc10::acquire() }
        pub fn pc11(&self) -> Option<Owned<gpio::Pc11>> { gpio::Pc11::acquire() }
        pub fn pc12(&self) -> Option<Owned<gpio::Pc12>> { gpio::Pc12::acquire() }
        pub fn pc13(&self) -> Option<Owned<gpio::Pc13>> { gpio::Pc13::acquire() }
        pub fn pc14(&self) -> Option<Owned<gpio::Pc14>> { gpio::Pc14::acquire() }
        pub fn pc15(&self) -> Option<Owned<gpio::Pc15>> { gpio::Pc15::acquire() }
        pub fn pc16(&self) -> Option<Owned<gpio::Pc16>> { gpio::Pc16::acquire() }
        pub fn pc17(&self) -> Option<Owned<gpio::Pc17>> { gpio::Pc17::acquire() }
        pub fn pc18(&self) -> Option<Owned<gpio::Pc18>> { gpio::Pc18::acquire() }
        pub fn pc19(&self) -> Option<Owned<gpio::Pc19>> { gpio::Pc19::acquire() }
    pub fn gpiod(&self) -> Option<Owned<gpio::Gpiod>> { gpio::Gpiod::acquire() }
        pub fn pd0(&self) -> Option<Owned<gpio::Pd0>> { gpio::Pd0::acquire() }
        pub fn pd1(&self) -> Option<Owned<gpio::Pd1>> { gpio::Pd1::acquire() }
        pub fn pd2(&self) -> Option<Owned<gpio::Pd2>> { gpio::Pd2::acquire() }
        pub fn pd3(&self) -> Option<Owned<gpio::Pd3>> { gpio::Pd3::acquire() }
        pub fn pd4(&self) -> Option<Owned<gpio::Pd4>> { gpio::Pd4::acquire() }
        pub fn pd5(&self) -> Option<Owned<gpio::Pd5>> { gpio::Pd5::acquire() }
        pub fn pd6(&self) -> Option<Owned<gpio::Pd6>> { gpio::Pd6::acquire() }
        pub fn pd7(&self) -> Option<Owned<gpio::Pd7>> { gpio::Pd7::acquire() }
        pub fn pd8(&self) -> Option<Owned<gpio::Pd8>> { gpio::Pd8::acquire() }
        pub fn pd9(&self) -> Option<Owned<gpio::Pd9>> { gpio::Pd9::acquire() }
        pub fn pd10(&self) -> Option<Owned<gpio::Pd10>> { gpio::Pd10::acquire() }
        pub fn pd11(&self) -> Option<Owned<gpio::Pd11>> { gpio::Pd11::acquire() }
        pub fn pd12(&self) -> Option<Owned<gpio::Pd12>> { gpio::Pd12::acquire() }
        pub fn pd13(&self) -> Option<Owned<gpio::Pd13>> { gpio::Pd13::acquire() }
        pub fn pd14(&self) -> Option<Owned<gpio::Pd14>> { gpio::Pd14::acquire() }
        pub fn pd15(&self) -> Option<Owned<gpio::Pd15>> { gpio::Pd15::acquire() }
    pub fn gpioe(&self) -> Option<Owned<gpio::Gpioe>> { gpio::Gpioe::acquire() }
        pub fn pe0(&self) -> Option<Owned<gpio::Pe0>> { gpio::Pe0::acquire() }
        pub fn pe1(&self) -> Option<Owned<gpio::Pe1>> { gpio::Pe1::acquire() }
        pub fn pe2(&self) -> Option<Owned<gpio::Pe2>> { gpio::Pe2::acquire() }
        pub fn pe3(&self) -> Option<Owned<gpio::Pe3>> { gpio::Pe3::acquire() }
        pub fn pe4(&self) -> Option<Owned<gpio::Pe4>> { gpio::Pe4::acquire() }
        pub fn pe5(&self) -> Option<Owned<gpio::Pe5>> { gpio::Pe5::acquire() }
        pub fn pe6(&self) -> Option<Owned<gpio::Pe6>> { gpio::Pe6::acquire() }
        pub fn pe7(&self) -> Option<Owned<gpio::Pe7>> { gpio::Pe7::acquire() }
        pub fn pe8(&self) -> Option<Owned<gpio::Pe8>> { gpio::Pe8::acquire() }
        pub fn pe9(&self) -> Option<Owned<gpio::Pe9>> { gpio::Pe9::acquire() }
        pub fn pe10(&self) -> Option<Owned<gpio::Pe10>> { gpio::Pe10::acquire() }
        pub fn pe11(&self) -> Option<Owned<gpio::Pe11>> { gpio::Pe11::acquire() }
        pub fn pe12(&self) -> Option<Owned<gpio::Pe12>> { gpio::Pe12::acquire() }
        pub fn pe24(&self) -> Option<Owned<gpio::Pe24>> { gpio::Pe24::acquire() }
        pub fn pe25(&self) -> Option<Owned<gpio::Pe25>> { gpio::Pe25::acquire() }
        pub fn pe26(&self) -> Option<Owned<gpio::Pe26>> { gpio::Pe26::acquire() }
        pub fn pe27(&self) -> Option<Owned<gpio::Pe27>> { gpio::Pe27::acquire() }
        pub fn pe28(&self) -> Option<Owned<gpio::Pe28>> { gpio::Pe28::acquire() }
    pub fn porta(&self) -> Option<Owned<port::Porta>> { port::Porta::acquire() }
        pub fn pta0(&self) -> Option<Owned<pin::Pta0>> { pin::Pta0::acquire() }
        pub fn pta1(&self) -> Option<Owned<pin::Pta1>> { pin::Pta1::acquire() }
        pub fn pta2(&self) -> Option<Owned<pin::Pta2>> { pin::Pta2::acquire() }
        pub fn pta3(&self) -> Option<Owned<pin::Pta3>> { pin::Pta3::acquire() }
        pub fn pta4(&self) -> Option<Owned<pin::Pta4>> { pin::Pta4::acquire() }
        pub fn pta5(&self) -> Option<Owned<pin::Pta5>> { pin::Pta5::acquire() }
        pub fn pta6(&self) -> Option<Owned<pin::Pta6>> { pin::Pta6::acquire() }
        pub fn pta7(&self) -> Option<Owned<pin::Pta7>> { pin::Pta7::acquire() }
        pub fn pta8(&self) -> Option<Owned<pin::Pta8>> { pin::Pta8::acquire() }
        pub fn pta9(&self) -> Option<Owned<pin::Pta9>> { pin::Pta9::acquire() }
        pub fn pta10(&self) -> Option<Owned<pin::Pta10>> { pin::Pta10::acquire() }
        pub fn pta11(&self) -> Option<Owned<pin::Pta11>> { pin::Pta11::acquire() }
        pub fn pta12(&self) -> Option<Owned<pin::Pta12>> { pin::Pta12::acquire() }
        pub fn pta13(&self) -> Option<Owned<pin::Pta13>> { pin::Pta13::acquire() }
        pub fn pta14(&self) -> Option<Owned<pin::Pta14>> { pin::Pta14::acquire() }
        pub fn pta15(&self) -> Option<Owned<pin::Pta15>> { pin::Pta15::acquire() }
        pub fn pta16(&self) -> Option<Owned<pin::Pta16>> { pin::Pta16::acquire() }
        pub fn pta17(&self) -> Option<Owned<pin::Pta17>> { pin::Pta17::acquire() }
        pub fn pta18(&self) -> Option<Owned<pin::Pta18>> { pin::Pta18::acquire() }
        pub fn pta19(&self) -> Option<Owned<pin::Pta19>> { pin::Pta19::acquire() }
        pub fn pta24(&self) -> Option<Owned<pin::Pta24>> { pin::Pta24::acquire() }
        pub fn pta25(&self) -> Option<Owned<pin::Pta25>> { pin::Pta25::acquire() }
        pub fn pta26(&self) -> Option<Owned<pin::Pta26>> { pin::Pta26::acquire() }
        pub fn pta27(&self) -> Option<Owned<pin::Pta27>> { pin::Pta27::acquire() }
        pub fn pta28(&self) -> Option<Owned<pin::Pta28>> { pin::Pta28::acquire() }
        pub fn pta29(&self) -> Option<Owned<pin::Pta29>> { pin::Pta29::acquire() }
    pub fn portb(&self) -> Option<Owned<port::Portb>> { port::Portb::acquire() }
        pub fn ptb0(&self) -> Option<Owned<pin::Ptb0>> { pin::Ptb0::acquire() }
        pub fn ptb1(&self) -> Option<Owned<pin::Ptb1>> { pin::Ptb1::acquire() }
        pub fn ptb2(&self) -> Option<Owned<pin::Ptb2>> { pin::Ptb2::acquire() }
        pub fn ptb3(&self) -> Option<Owned<pin::Ptb3>> { pin::Ptb3::acquire() }
        pub fn ptb4(&self) -> Option<Owned<pin::Ptb4>> { pin::Ptb4::acquire() }
        pub fn ptb5(&self) -> Option<Owned<pin::Ptb5>> { pin::Ptb5::acquire() }
        pub fn ptb6(&self) -> Option<Owned<pin::Ptb6>> { pin::Ptb6::acquire() }
        pub fn ptb7(&self) -> Option<Owned<pin::Ptb7>> { pin::Ptb7::acquire() }
        pub fn ptb8(&self) -> Option<Owned<pin::Ptb8>> { pin::Ptb8::acquire() }
        pub fn ptb9(&self) -> Option<Owned<pin::Ptb9>> { pin::Ptb9::acquire() }
        pub fn ptb10(&self) -> Option<Owned<pin::Ptb10>> { pin::Ptb10::acquire() }
        pub fn ptb11(&self) -> Option<Owned<pin::Ptb11>> { pin::Ptb11::acquire() }
        pub fn ptb12(&self) -> Option<Owned<pin::Ptb12>> { pin::Ptb12::acquire() }
        pub fn ptb13(&self) -> Option<Owned<pin::Ptb13>> { pin::Ptb13::acquire() }
        pub fn ptb16(&self) -> Option<Owned<pin::Ptb16>> { pin::Ptb16::acquire() }
        pub fn ptb17(&self) -> Option<Owned<pin::Ptb17>> { pin::Ptb17::acquire() }
        pub fn ptb18(&self) -> Option<Owned<pin::Ptb18>> { pin::Ptb18::acquire() }
        pub fn ptb19(&self) -> Option<Owned<pin::Ptb19>> { pin::Ptb19::acquire() }
        pub fn ptb20(&self) -> Option<Owned<pin::Ptb20>> { pin::Ptb20::acquire() }
        pub fn ptb21(&self) -> Option<Owned<pin::Ptb21>> { pin::Ptb21::acquire() }
        pub fn ptb22(&self) -> Option<Owned<pin::Ptb22>> { pin::Ptb22::acquire() }
        pub fn ptb23(&self) -> Option<Owned<pin::Ptb23>> { pin::Ptb23::acquire() }
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
        pub fn ptc12(&self) -> Option<Owned<pin::Ptc12>> { pin::Ptc12::acquire() }
        pub fn ptc13(&self) -> Option<Owned<pin::Ptc13>> { pin::Ptc13::acquire() }
        pub fn ptc14(&self) -> Option<Owned<pin::Ptc14>> { pin::Ptc14::acquire() }
        pub fn ptc15(&self) -> Option<Owned<pin::Ptc15>> { pin::Ptc15::acquire() }
        pub fn ptc16(&self) -> Option<Owned<pin::Ptc16>> { pin::Ptc16::acquire() }
        pub fn ptc17(&self) -> Option<Owned<pin::Ptc17>> { pin::Ptc17::acquire() }
        pub fn ptc18(&self) -> Option<Owned<pin::Ptc18>> { pin::Ptc18::acquire() }
        pub fn ptc19(&self) -> Option<Owned<pin::Ptc19>> { pin::Ptc19::acquire() }
    pub fn portd(&self) -> Option<Owned<port::Portd>> { port::Portd::acquire() }
        pub fn ptd0(&self) -> Option<Owned<pin::Ptd0>> { pin::Ptd0::acquire() }
        pub fn ptd1(&self) -> Option<Owned<pin::Ptd1>> { pin::Ptd1::acquire() }
        pub fn ptd2(&self) -> Option<Owned<pin::Ptd2>> { pin::Ptd2::acquire() }
        pub fn ptd3(&self) -> Option<Owned<pin::Ptd3>> { pin::Ptd3::acquire() }
        pub fn ptd4(&self) -> Option<Owned<pin::Ptd4>> { pin::Ptd4::acquire() }
        pub fn ptd5(&self) -> Option<Owned<pin::Ptd5>> { pin::Ptd5::acquire() }
        pub fn ptd6(&self) -> Option<Owned<pin::Ptd6>> { pin::Ptd6::acquire() }
        pub fn ptd7(&self) -> Option<Owned<pin::Ptd7>> { pin::Ptd7::acquire() }
        pub fn ptd8(&self) -> Option<Owned<pin::Ptd8>> { pin::Ptd8::acquire() }
        pub fn ptd9(&self) -> Option<Owned<pin::Ptd9>> { pin::Ptd9::acquire() }
        pub fn ptd10(&self) -> Option<Owned<pin::Ptd10>> { pin::Ptd10::acquire() }
        pub fn ptd11(&self) -> Option<Owned<pin::Ptd11>> { pin::Ptd11::acquire() }
        pub fn ptd12(&self) -> Option<Owned<pin::Ptd12>> { pin::Ptd12::acquire() }
        pub fn ptd13(&self) -> Option<Owned<pin::Ptd13>> { pin::Ptd13::acquire() }
        pub fn ptd14(&self) -> Option<Owned<pin::Ptd14>> { pin::Ptd14::acquire() }
        pub fn ptd15(&self) -> Option<Owned<pin::Ptd15>> { pin::Ptd15::acquire() }
    pub fn porte(&self) -> Option<Owned<port::Porte>> { port::Porte::acquire() }
        pub fn pte0(&self) -> Option<Owned<pin::Pte0>> { pin::Pte0::acquire() }
        pub fn pte1(&self) -> Option<Owned<pin::Pte1>> { pin::Pte1::acquire() }
        pub fn pte2(&self) -> Option<Owned<pin::Pte2>> { pin::Pte2::acquire() }
        pub fn pte3(&self) -> Option<Owned<pin::Pte3>> { pin::Pte3::acquire() }
        pub fn pte4(&self) -> Option<Owned<pin::Pte4>> { pin::Pte4::acquire() }
        pub fn pte5(&self) -> Option<Owned<pin::Pte5>> { pin::Pte5::acquire() }
        pub fn pte6(&self) -> Option<Owned<pin::Pte6>> { pin::Pte6::acquire() }
        pub fn pte7(&self) -> Option<Owned<pin::Pte7>> { pin::Pte7::acquire() }
        pub fn pte8(&self) -> Option<Owned<pin::Pte8>> { pin::Pte8::acquire() }
        pub fn pte9(&self) -> Option<Owned<pin::Pte9>> { pin::Pte9::acquire() }
        pub fn pte10(&self) -> Option<Owned<pin::Pte10>> { pin::Pte10::acquire() }
        pub fn pte11(&self) -> Option<Owned<pin::Pte11>> { pin::Pte11::acquire() }
        pub fn pte12(&self) -> Option<Owned<pin::Pte12>> { pin::Pte12::acquire() }
        pub fn pte24(&self) -> Option<Owned<pin::Pte24>> { pin::Pte24::acquire() }
        pub fn pte25(&self) -> Option<Owned<pin::Pte25>> { pin::Pte25::acquire() }
        pub fn pte26(&self) -> Option<Owned<pin::Pte26>> { pin::Pte26::acquire() }
        pub fn pte27(&self) -> Option<Owned<pin::Pte27>> { pin::Pte27::acquire() }
        pub fn pte28(&self) -> Option<Owned<pin::Pte28>> { pin::Pte28::acquire() }
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
    pub fn adc1(&self) -> Option<Owned<adc::Adc1>> { adc::Adc1::acquire() }
        pub fn adc1_ch0(&self) -> Option<Owned<adc::Adc1Ch0>> { adc::Adc1Ch0::acquire() }
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
        pub fn adc1_ch16(&self) -> Option<Owned<adc::Adc1Ch16>> { adc::Adc1Ch16::acquire() }
        pub fn adc1_ch17(&self) -> Option<Owned<adc::Adc1Ch17>> { adc::Adc1Ch17::acquire() }
        pub fn adc1_ch18(&self) -> Option<Owned<adc::Adc1Ch18>> { adc::Adc1Ch18::acquire() }
        pub fn adc1_ch19(&self) -> Option<Owned<adc::Adc1Ch19>> { adc::Adc1Ch19::acquire() }
        pub fn adc1_ch20(&self) -> Option<Owned<adc::Adc1Ch20>> { adc::Adc1Ch20::acquire() }
        pub fn adc1_ch21(&self) -> Option<Owned<adc::Adc1Ch21>> { adc::Adc1Ch21::acquire() }
        pub fn adc1_ch22(&self) -> Option<Owned<adc::Adc1Ch22>> { adc::Adc1Ch22::acquire() }
        pub fn adc1_ch23(&self) -> Option<Owned<adc::Adc1Ch23>> { adc::Adc1Ch23::acquire() }
        pub fn adc1_temp(&self) -> Option<Owned<adc::Adc1Temp>> { adc::Adc1Temp::acquire() }
        pub fn adc1_bandgap(&self) -> Option<Owned<adc::Adc1Bandgap>> { adc::Adc1Bandgap::acquire() }
        pub fn adc1_refsh(&self) -> Option<Owned<adc::Adc1Refsh>> { adc::Adc1Refsh::acquire() }
        pub fn adc1_refsl(&self) -> Option<Owned<adc::Adc1Refsl>> { adc::Adc1Refsl::acquire() }
}

impl Get<sim::Sim> for K64 {
    fn get(&self) -> sim::Sim { sim::SIM }
}

impl GetPeriph<sim::SimPeriph> for K64 {
    fn get_periph(&self) -> sim::SimPeriph { sim::SIM_PERIPH }
}

impl GetPeriphInstance<sim::SimPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<sim::SimPeriph> {
        match index { 
            0 => Some(sim::SIM_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<mcg::Mcg> for K64 {
    fn get(&self) -> mcg::Mcg { mcg::MCG }
}

impl GetPeriph<mcg::McgPeriph> for K64 {
    fn get_periph(&self) -> mcg::McgPeriph { mcg::MCG_PERIPH }
}

impl GetPeriphInstance<mcg::McgPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<mcg::McgPeriph> {
        match index { 
            0 => Some(mcg::MCG_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<mpu::Mpu> for K64 {
    fn get(&self) -> mpu::Mpu { mpu::MPU }
}

impl GetPeriph<mpu::MpuPeriph> for K64 {
    fn get_periph(&self) -> mpu::MpuPeriph { mpu::MPU_PERIPH }
}

impl GetPeriphInstance<mpu::MpuPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<mpu::MpuPeriph> {
        match index { 
            0 => Some(mpu::MPU_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<osc::Osc> for K64 {
    fn get(&self) -> osc::Osc { osc::OSC }
}

impl GetPeriph<osc::OscPeriph> for K64 {
    fn get_periph(&self) -> osc::OscPeriph { osc::OSC_PERIPH }
}

impl GetPeriphInstance<osc::OscPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<osc::OscPeriph> {
        match index { 
            0 => Some(osc::OSC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<rcm::Rcm> for K64 {
    fn get(&self) -> rcm::Rcm { rcm::RCM }
}

impl GetPeriph<rcm::RcmPeriph> for K64 {
    fn get_periph(&self) -> rcm::RcmPeriph { rcm::RCM_PERIPH }
}

impl GetPeriphInstance<rcm::RcmPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<rcm::RcmPeriph> {
        match index { 
            0 => Some(rcm::RCM_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<enet::Enet> for K64 {
    fn get(&self) -> enet::Enet { enet::ENET }
}

impl GetPeriph<enet::EnetPeriph> for K64 {
    fn get_periph(&self) -> enet::EnetPeriph { enet::ENET_PERIPH }
}

impl GetPeriphInstance<enet::EnetPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<enet::EnetPeriph> {
        match index { 
            0 => Some(enet::ENET_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<crc::Crc> for K64 {
    fn get(&self) -> crc::Crc { crc::CRC }
}

impl GetPeriph<crc::CrcPeriph> for K64 {
    fn get_periph(&self) -> crc::CrcPeriph { crc::CRC_PERIPH }
}

impl GetPeriphInstance<crc::CrcPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<crc::CrcPeriph> {
        match index {
            0 => Some(crc::CRC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<wdog::Wdog> for K64 {
    fn get(&self) -> wdog::Wdog { wdog::WDOG }
}

impl GetPeriph<wdog::WdogPeriph> for K64 {
    fn get_periph(&self) -> wdog::WdogPeriph { wdog::WDOG_PERIPH }
}

impl GetPeriphInstance<wdog::WdogPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<wdog::WdogPeriph> {
        match index {
            0 => Some(wdog::WDOG_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<dmamux::Dmamux> for K64 {
    fn get(&self) -> dmamux::Dmamux { dmamux::DMAMUX }
}

impl GetPeriph<dmamux::DmamuxPeriph> for K64 {
    fn get_periph(&self) -> dmamux::DmamuxPeriph { dmamux::DMAMUX_PERIPH }
}

impl GetPeriphInstance<dmamux::DmamuxPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<dmamux::DmamuxPeriph> {
        match index {
            0 => Some(dmamux::DMAMUX_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<edma::Dma> for K64 {
    fn get(&self) -> edma::Dma { edma::DMA }
}

impl GetPeriph<edma::EdmaPeriph> for K64 {
    fn get_periph(&self) -> edma::EdmaPeriph { edma::DMA_PERIPH }
}

impl GetPeriphInstance<edma::EdmaPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<edma::EdmaPeriph> {
        match index {
            0 => Some(edma::DMA_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<ftm::Ftm0> for K64 {
    fn get(&self) -> ftm::Ftm0 { ftm::FTM0 }
}

impl Get<ftm::Ftm1> for K64 {
    fn get(&self) -> ftm::Ftm1 { ftm::FTM1 }
}

impl Get<ftm::Ftm2> for K64 {
    fn get(&self) -> ftm::Ftm2 { ftm::FTM2 }
}

impl GetPeriphInstance<ftm::FtmPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<ftm::FtmPeriph> {
        match index {
            0 => Some(ftm::FTM0_PERIPH),
            1 => Some(ftm::FTM1_PERIPH),
            2 => Some(ftm::FTM2_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 3 }
}

impl Get<pit::Pit> for K64 {
    fn get(&self) -> pit::Pit { pit::PIT }
}

impl GetPeriph<pit::PitPeriph> for K64 {
    fn get_periph(&self) -> pit::PitPeriph { pit::PIT_PERIPH }
}

impl GetPeriphInstance<pit::PitPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<pit::PitPeriph> {
        match index {
            0 => Some(pit::PIT_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<lptmr::Lptmr0> for K64 {
    fn get(&self) -> lptmr::Lptmr0 { lptmr::LPTMR0 }
}

impl GetPeriph<lptmr::LptmrPeriph> for K64 {
    fn get_periph(&self) -> lptmr::LptmrPeriph { lptmr::LPTMR0_PERIPH }
}

impl GetPeriphInstance<lptmr::LptmrPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<lptmr::LptmrPeriph> {
        match index {
            0 => Some(lptmr::LPTMR0_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<spi::Spi0> for K64 {
    fn get(&self) -> spi::Spi0 { spi::SPI0 }
}

impl Get<spi::Spi1> for K64 {
    fn get(&self) -> spi::Spi1 { spi::SPI1 }
}

impl Get<spi::Spi2> for K64 {
    fn get(&self) -> spi::Spi2 { spi::SPI2 }
}

impl GetPeriphInstance<spi::SpiPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<spi::SpiPeriph> {
        match index {
            0 => Some(spi::SPI0_PERIPH),
            1 => Some(spi::SPI1_PERIPH),
            2 => Some(spi::SPI2_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 3 }
}

impl Get<i2c::I2c0> for K64 {
    fn get(&self) -> i2c::I2c0 { i2c::I2C0 }
}

impl Get<i2c::I2c1> for K64 {
    fn get(&self) -> i2c::I2c1 { i2c::I2C1 }
}

impl GetPeriphInstance<i2c::I2cPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<i2c::I2cPeriph> {
        match index {
            0 => Some(i2c::I2C0_PERIPH),
            1 => Some(i2c::I2C1_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

impl Get<uart::Uart0> for K64 {
    fn get(&self) -> uart::Uart0 { uart::UART0 }
}

impl Get<uart::Uart1> for K64 {
    fn get(&self) -> uart::Uart1 { uart::UART1 }
}

impl Get<uart::Uart2> for K64 {
    fn get(&self) -> uart::Uart2 { uart::UART2 }
}

impl Get<uart::Uart3> for K64 {
    fn get(&self) -> uart::Uart3 { uart::UART3 }
}

impl Get<uart::Uart4> for K64 {
    fn get(&self) -> uart::Uart4 { uart::UART4 }
}

impl Get<uart::Uart5> for K64 {
    fn get(&self) -> uart::Uart5 { uart::UART5 }
}

impl GetPeriphInstance<uart::UartPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<uart::UartPeriph> {
        match index {
            0 => Some(uart::UART0_PERIPH),
            1 => Some(uart::UART1_PERIPH),
            2 => Some(uart::UART2_PERIPH),
            3 => Some(uart::UART3_PERIPH),
            4 => Some(uart::UART4_PERIPH),
            5 => Some(uart::UART5_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 6 }
}

impl Get<usb::Usb0> for K64 {
    fn get(&self) -> usb::Usb0 { usb::USB0 }
}

impl GetPeriph<usb::UsbPeriph> for K64 {
    fn get_periph(&self) -> usb::UsbPeriph { usb::USB0_PERIPH }
}

impl GetPeriphInstance<usb::UsbPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<usb::UsbPeriph> {
        match index {
            0 => Some(usb::USB0_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<flexcan::Can0> for K64 {
    fn get(&self) -> flexcan::Can0 { flexcan::CAN0 }
}

impl GetPeriph<flexcan::FlexcanPeriph> for K64 {
    fn get_periph(&self) -> flexcan::FlexcanPeriph { flexcan::CAN0_PERIPH }
}

impl GetPeriphInstance<flexcan::FlexcanPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<flexcan::FlexcanPeriph> {
        match index {
            0 => Some(flexcan::CAN0_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<dac::Dac0> for K64 {
    fn get(&self) -> dac::Dac0 { dac::DAC0 }
}

impl Get<dac::Dac1> for K64 {
    fn get(&self) -> dac::Dac1 { dac::DAC1 }
}

impl GetPeriphInstance<dac::DacPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<dac::DacPeriph> {
        match index {
            0 => Some(dac::DAC0_PERIPH),
            1 => Some(dac::DAC1_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

impl Get<gpio::Gpioa> for K64 {
    fn get(&self) -> gpio::Gpioa { gpio::GPIOA }
}

impl Get<gpio::Gpiob> for K64 {
    fn get(&self) -> gpio::Gpiob { gpio::GPIOB }
}

impl Get<gpio::Gpioc> for K64 {
    fn get(&self) -> gpio::Gpioc { gpio::GPIOC }
}

impl Get<gpio::Gpiod> for K64 {
    fn get(&self) -> gpio::Gpiod { gpio::GPIOD }
}

impl Get<gpio::Gpioe> for K64 {
    fn get(&self) -> gpio::Gpioe { gpio::GPIOE }
}

impl GetPeriphInstance<gpio::GpioPeriph> for K64 {
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

impl Get<port::Porta> for K64 {
    fn get(&self) -> port::Porta { port::PORTA }
}

impl Get<port::Portb> for K64 {
    fn get(&self) -> port::Portb { port::PORTB }
}

impl Get<port::Portc> for K64 {
    fn get(&self) -> port::Portc { port::PORTC }
}

impl Get<port::Portd> for K64 {
    fn get(&self) -> port::Portd { port::PORTD }
}

impl Get<port::Porte> for K64 {
    fn get(&self) -> port::Porte { port::PORTE }
}

impl GetPeriphInstance<port::PortPeriph> for K64 {
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

impl Get<adc::Adc0> for K64 {
    fn get(&self) -> adc::Adc0 { adc::ADC0 }
}

impl Get<adc::Adc1> for K64 {
    fn get(&self) -> adc::Adc1 { adc::ADC1 }
}

impl GetPeriphInstance<adc::AdcPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<adc::AdcPeriph> {
        match index {
            0 => Some(adc::ADC0_PERIPH),
            1 => Some(adc::ADC1_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

