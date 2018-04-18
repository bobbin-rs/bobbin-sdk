pub use ::bobbin_mcu::*;
use ::bobbin_common::owned::*;

pub mod sim;
pub mod mcg;
pub mod osc;
pub mod crc;
pub mod dmamux;
pub mod edma;
pub mod ftm;
pub mod pit;
pub mod spi;
pub mod i2c;
pub mod uart;
pub mod gpio;
pub mod port;
pub mod sig;
pub mod pin;
pub mod irq;

#[derive(Debug, Default)]
pub struct K20 {}

impl Mcu for K20 {
    fn id(&self) -> &'static str { "K20" }
}

impl K20 {
    pub fn sim(&self) -> Option<Owned<sim::Sim>> { sim::Sim::acquire() }
    pub fn mcg(&self) -> Option<Owned<mcg::Mcg>> { mcg::Mcg::acquire() }
    pub fn osc(&self) -> Option<Owned<osc::Osc>> { osc::Osc::acquire() }
    pub fn crc(&self) -> Option<Owned<crc::Crc>> { crc::Crc::acquire() }
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
        pub fn pa16(&self) -> Option<Owned<pin::Pa16>> { pin::Pa16::acquire() }
        pub fn pa17(&self) -> Option<Owned<pin::Pa17>> { pin::Pa17::acquire() }
        pub fn pa18(&self) -> Option<Owned<pin::Pa18>> { pin::Pa18::acquire() }
        pub fn pa19(&self) -> Option<Owned<pin::Pa19>> { pin::Pa19::acquire() }
        pub fn pa24(&self) -> Option<Owned<pin::Pa24>> { pin::Pa24::acquire() }
        pub fn pa25(&self) -> Option<Owned<pin::Pa25>> { pin::Pa25::acquire() }
        pub fn pa26(&self) -> Option<Owned<pin::Pa26>> { pin::Pa26::acquire() }
        pub fn pa27(&self) -> Option<Owned<pin::Pa27>> { pin::Pa27::acquire() }
        pub fn pa28(&self) -> Option<Owned<pin::Pa28>> { pin::Pa28::acquire() }
        pub fn pa29(&self) -> Option<Owned<pin::Pa29>> { pin::Pa29::acquire() }
    pub fn gpiob(&self) -> Option<Owned<gpio::Gpiob>> { gpio::Gpiob::acquire() }
        pub fn pb0(&self) -> Option<Owned<pin::Pb0>> { pin::Pb0::acquire() }
        pub fn pb1(&self) -> Option<Owned<pin::Pb1>> { pin::Pb1::acquire() }
        pub fn pb2(&self) -> Option<Owned<pin::Pb2>> { pin::Pb2::acquire() }
        pub fn pb3(&self) -> Option<Owned<pin::Pb3>> { pin::Pb3::acquire() }
        pub fn pb4(&self) -> Option<Owned<pin::Pb4>> { pin::Pb4::acquire() }
        pub fn pb5(&self) -> Option<Owned<pin::Pb5>> { pin::Pb5::acquire() }
        pub fn pb6(&self) -> Option<Owned<pin::Pb6>> { pin::Pb6::acquire() }
        pub fn pb7(&self) -> Option<Owned<pin::Pb7>> { pin::Pb7::acquire() }
        pub fn pb8(&self) -> Option<Owned<pin::Pb8>> { pin::Pb8::acquire() }
        pub fn pb9(&self) -> Option<Owned<pin::Pb9>> { pin::Pb9::acquire() }
        pub fn pb10(&self) -> Option<Owned<pin::Pb10>> { pin::Pb10::acquire() }
        pub fn pb11(&self) -> Option<Owned<pin::Pb11>> { pin::Pb11::acquire() }
        pub fn pb12(&self) -> Option<Owned<pin::Pb12>> { pin::Pb12::acquire() }
        pub fn pb13(&self) -> Option<Owned<pin::Pb13>> { pin::Pb13::acquire() }
        pub fn pb16(&self) -> Option<Owned<pin::Pb16>> { pin::Pb16::acquire() }
        pub fn pb17(&self) -> Option<Owned<pin::Pb17>> { pin::Pb17::acquire() }
        pub fn pb18(&self) -> Option<Owned<pin::Pb18>> { pin::Pb18::acquire() }
        pub fn pb19(&self) -> Option<Owned<pin::Pb19>> { pin::Pb19::acquire() }
        pub fn pb20(&self) -> Option<Owned<pin::Pb20>> { pin::Pb20::acquire() }
        pub fn pb21(&self) -> Option<Owned<pin::Pb21>> { pin::Pb21::acquire() }
        pub fn pb22(&self) -> Option<Owned<pin::Pb22>> { pin::Pb22::acquire() }
        pub fn pb23(&self) -> Option<Owned<pin::Pb23>> { pin::Pb23::acquire() }
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
        pub fn pc12(&self) -> Option<Owned<pin::Pc12>> { pin::Pc12::acquire() }
        pub fn pc13(&self) -> Option<Owned<pin::Pc13>> { pin::Pc13::acquire() }
        pub fn pc14(&self) -> Option<Owned<pin::Pc14>> { pin::Pc14::acquire() }
        pub fn pc15(&self) -> Option<Owned<pin::Pc15>> { pin::Pc15::acquire() }
        pub fn pc16(&self) -> Option<Owned<pin::Pc16>> { pin::Pc16::acquire() }
        pub fn pc17(&self) -> Option<Owned<pin::Pc17>> { pin::Pc17::acquire() }
        pub fn pc18(&self) -> Option<Owned<pin::Pc18>> { pin::Pc18::acquire() }
        pub fn pc19(&self) -> Option<Owned<pin::Pc19>> { pin::Pc19::acquire() }
    pub fn gpiod(&self) -> Option<Owned<gpio::Gpiod>> { gpio::Gpiod::acquire() }
        pub fn pd0(&self) -> Option<Owned<pin::Pd0>> { pin::Pd0::acquire() }
        pub fn pd1(&self) -> Option<Owned<pin::Pd1>> { pin::Pd1::acquire() }
        pub fn pd2(&self) -> Option<Owned<pin::Pd2>> { pin::Pd2::acquire() }
        pub fn pd3(&self) -> Option<Owned<pin::Pd3>> { pin::Pd3::acquire() }
        pub fn pd4(&self) -> Option<Owned<pin::Pd4>> { pin::Pd4::acquire() }
        pub fn pd5(&self) -> Option<Owned<pin::Pd5>> { pin::Pd5::acquire() }
        pub fn pd6(&self) -> Option<Owned<pin::Pd6>> { pin::Pd6::acquire() }
        pub fn pd7(&self) -> Option<Owned<pin::Pd7>> { pin::Pd7::acquire() }
        pub fn pd8(&self) -> Option<Owned<pin::Pd8>> { pin::Pd8::acquire() }
        pub fn pd9(&self) -> Option<Owned<pin::Pd9>> { pin::Pd9::acquire() }
        pub fn pd10(&self) -> Option<Owned<pin::Pd10>> { pin::Pd10::acquire() }
        pub fn pd11(&self) -> Option<Owned<pin::Pd11>> { pin::Pd11::acquire() }
        pub fn pd12(&self) -> Option<Owned<pin::Pd12>> { pin::Pd12::acquire() }
        pub fn pd13(&self) -> Option<Owned<pin::Pd13>> { pin::Pd13::acquire() }
        pub fn pd14(&self) -> Option<Owned<pin::Pd14>> { pin::Pd14::acquire() }
        pub fn pd15(&self) -> Option<Owned<pin::Pd15>> { pin::Pd15::acquire() }
    pub fn gpioe(&self) -> Option<Owned<gpio::Gpioe>> { gpio::Gpioe::acquire() }
        pub fn pe0(&self) -> Option<Owned<pin::Pe0>> { pin::Pe0::acquire() }
        pub fn pe1(&self) -> Option<Owned<pin::Pe1>> { pin::Pe1::acquire() }
        pub fn pe2(&self) -> Option<Owned<pin::Pe2>> { pin::Pe2::acquire() }
        pub fn pe3(&self) -> Option<Owned<pin::Pe3>> { pin::Pe3::acquire() }
        pub fn pe4(&self) -> Option<Owned<pin::Pe4>> { pin::Pe4::acquire() }
        pub fn pe5(&self) -> Option<Owned<pin::Pe5>> { pin::Pe5::acquire() }
        pub fn pe6(&self) -> Option<Owned<pin::Pe6>> { pin::Pe6::acquire() }
        pub fn pe7(&self) -> Option<Owned<pin::Pe7>> { pin::Pe7::acquire() }
        pub fn pe8(&self) -> Option<Owned<pin::Pe8>> { pin::Pe8::acquire() }
        pub fn pe9(&self) -> Option<Owned<pin::Pe9>> { pin::Pe9::acquire() }
        pub fn pe10(&self) -> Option<Owned<pin::Pe10>> { pin::Pe10::acquire() }
        pub fn pe11(&self) -> Option<Owned<pin::Pe11>> { pin::Pe11::acquire() }
        pub fn pe12(&self) -> Option<Owned<pin::Pe12>> { pin::Pe12::acquire() }
        pub fn pe24(&self) -> Option<Owned<pin::Pe24>> { pin::Pe24::acquire() }
        pub fn pe25(&self) -> Option<Owned<pin::Pe25>> { pin::Pe25::acquire() }
        pub fn pe26(&self) -> Option<Owned<pin::Pe26>> { pin::Pe26::acquire() }
        pub fn pe27(&self) -> Option<Owned<pin::Pe27>> { pin::Pe27::acquire() }
        pub fn pe28(&self) -> Option<Owned<pin::Pe28>> { pin::Pe28::acquire() }
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
}

impl Get<sim::Sim> for K20 {
    fn get(&self) -> sim::Sim { sim::SIM }
}

impl GetPeriph<sim::SimPeriph> for K20 {
    fn get_periph(&self) -> sim::SimPeriph { sim::SIM_PERIPH }
}

impl GetPeriphInstance<sim::SimPeriph> for K20 {
    fn get_periph_instance(&self, index: usize) -> Option<sim::SimPeriph> {
        match index { 
            0 => Some(sim::SIM_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<mcg::Mcg> for K20 {
    fn get(&self) -> mcg::Mcg { mcg::MCG }
}

impl GetPeriph<mcg::McgPeriph> for K20 {
    fn get_periph(&self) -> mcg::McgPeriph { mcg::MCG_PERIPH }
}

impl GetPeriphInstance<mcg::McgPeriph> for K20 {
    fn get_periph_instance(&self, index: usize) -> Option<mcg::McgPeriph> {
        match index { 
            0 => Some(mcg::MCG_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<osc::Osc> for K20 {
    fn get(&self) -> osc::Osc { osc::OSC }
}

impl GetPeriph<osc::OscPeriph> for K20 {
    fn get_periph(&self) -> osc::OscPeriph { osc::OSC_PERIPH }
}

impl GetPeriphInstance<osc::OscPeriph> for K20 {
    fn get_periph_instance(&self, index: usize) -> Option<osc::OscPeriph> {
        match index { 
            0 => Some(osc::OSC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<crc::Crc> for K20 {
    fn get(&self) -> crc::Crc { crc::CRC }
}

impl GetPeriph<crc::CrcPeriph> for K20 {
    fn get_periph(&self) -> crc::CrcPeriph { crc::CRC_PERIPH }
}

impl GetPeriphInstance<crc::CrcPeriph> for K20 {
    fn get_periph_instance(&self, index: usize) -> Option<crc::CrcPeriph> {
        match index {
            0 => Some(crc::CRC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<dmamux::Dmamux> for K20 {
    fn get(&self) -> dmamux::Dmamux { dmamux::DMAMUX }
}

impl GetPeriph<dmamux::DmamuxPeriph> for K20 {
    fn get_periph(&self) -> dmamux::DmamuxPeriph { dmamux::DMAMUX_PERIPH }
}

impl GetPeriphInstance<dmamux::DmamuxPeriph> for K20 {
    fn get_periph_instance(&self, index: usize) -> Option<dmamux::DmamuxPeriph> {
        match index {
            0 => Some(dmamux::DMAMUX_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<edma::Dma> for K20 {
    fn get(&self) -> edma::Dma { edma::DMA }
}

impl GetPeriph<edma::EdmaPeriph> for K20 {
    fn get_periph(&self) -> edma::EdmaPeriph { edma::DMA_PERIPH }
}

impl GetPeriphInstance<edma::EdmaPeriph> for K20 {
    fn get_periph_instance(&self, index: usize) -> Option<edma::EdmaPeriph> {
        match index {
            0 => Some(edma::DMA_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<ftm::Ftm0> for K20 {
    fn get(&self) -> ftm::Ftm0 { ftm::FTM0 }
}

impl Get<ftm::Ftm1> for K20 {
    fn get(&self) -> ftm::Ftm1 { ftm::FTM1 }
}

impl Get<ftm::Ftm2> for K20 {
    fn get(&self) -> ftm::Ftm2 { ftm::FTM2 }
}

impl GetPeriphInstance<ftm::FtmPeriph> for K20 {
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

impl Get<pit::Pit> for K20 {
    fn get(&self) -> pit::Pit { pit::PIT }
}

impl GetPeriph<pit::PitPeriph> for K20 {
    fn get_periph(&self) -> pit::PitPeriph { pit::PIT_PERIPH }
}

impl GetPeriphInstance<pit::PitPeriph> for K20 {
    fn get_periph_instance(&self, index: usize) -> Option<pit::PitPeriph> {
        match index {
            0 => Some(pit::PIT_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<spi::Spi0> for K20 {
    fn get(&self) -> spi::Spi0 { spi::SPI0 }
}

impl Get<spi::Spi1> for K20 {
    fn get(&self) -> spi::Spi1 { spi::SPI1 }
}

impl Get<spi::Spi2> for K20 {
    fn get(&self) -> spi::Spi2 { spi::SPI2 }
}

impl GetPeriphInstance<spi::SpiPeriph> for K20 {
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

impl Get<i2c::I2c0> for K20 {
    fn get(&self) -> i2c::I2c0 { i2c::I2C0 }
}

impl Get<i2c::I2c1> for K20 {
    fn get(&self) -> i2c::I2c1 { i2c::I2C1 }
}

impl GetPeriphInstance<i2c::I2cPeriph> for K20 {
    fn get_periph_instance(&self, index: usize) -> Option<i2c::I2cPeriph> {
        match index {
            0 => Some(i2c::I2C0_PERIPH),
            1 => Some(i2c::I2C1_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

impl Get<uart::Uart0> for K20 {
    fn get(&self) -> uart::Uart0 { uart::UART0 }
}

impl Get<uart::Uart1> for K20 {
    fn get(&self) -> uart::Uart1 { uart::UART1 }
}

impl Get<uart::Uart2> for K20 {
    fn get(&self) -> uart::Uart2 { uart::UART2 }
}

impl Get<uart::Uart3> for K20 {
    fn get(&self) -> uart::Uart3 { uart::UART3 }
}

impl Get<uart::Uart4> for K20 {
    fn get(&self) -> uart::Uart4 { uart::UART4 }
}

impl Get<uart::Uart5> for K20 {
    fn get(&self) -> uart::Uart5 { uart::UART5 }
}

impl GetPeriphInstance<uart::UartPeriph> for K20 {
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

impl Get<gpio::Gpioa> for K20 {
    fn get(&self) -> gpio::Gpioa { gpio::GPIOA }
}

impl Get<gpio::Gpiob> for K20 {
    fn get(&self) -> gpio::Gpiob { gpio::GPIOB }
}

impl Get<gpio::Gpioc> for K20 {
    fn get(&self) -> gpio::Gpioc { gpio::GPIOC }
}

impl Get<gpio::Gpiod> for K20 {
    fn get(&self) -> gpio::Gpiod { gpio::GPIOD }
}

impl Get<gpio::Gpioe> for K20 {
    fn get(&self) -> gpio::Gpioe { gpio::GPIOE }
}

impl GetPeriphInstance<gpio::GpioPeriph> for K20 {
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

impl Get<port::Porta> for K20 {
    fn get(&self) -> port::Porta { port::PORTA }
}

impl Get<port::Portb> for K20 {
    fn get(&self) -> port::Portb { port::PORTB }
}

impl Get<port::Portc> for K20 {
    fn get(&self) -> port::Portc { port::PORTC }
}

impl Get<port::Portd> for K20 {
    fn get(&self) -> port::Portd { port::PORTD }
}

impl Get<port::Porte> for K20 {
    fn get(&self) -> port::Porte { port::PORTE }
}

impl GetPeriphInstance<port::PortPeriph> for K20 {
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

