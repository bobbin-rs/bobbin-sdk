pub use ::bobbin_mcu::*;
use ::bobbin_common::owned::*;

pub mod sim;
pub mod scg;
pub mod pcc;
pub mod rcm;
pub mod wdog;
pub mod rtc;
pub mod smc;
pub mod crc;
pub mod dmamux;
pub mod edma;
pub mod ftm;
pub mod lpit;
pub mod lptmr;
pub mod flexcan;
pub mod port;
pub mod gpio;
pub mod lpuart;
pub mod lpi2c;
pub mod lpspi;
pub mod adc;
pub mod sig;
pub mod pin;
pub mod irq;

#[derive(Debug, Default)]
pub struct S32k {}

impl Mcu for S32k {
    fn id(&self) -> &'static str { "S32K" }
}

impl S32k {
    pub fn sim(&self) -> Option<Owned<sim::Sim>> { sim::Sim::acquire() }
    pub fn scg(&self) -> Option<Owned<scg::Scg>> { scg::Scg::acquire() }
    pub fn pcc(&self) -> Option<Owned<pcc::Pcc>> { pcc::Pcc::acquire() }
    pub fn rcm(&self) -> Option<Owned<rcm::Rcm>> { rcm::Rcm::acquire() }
    pub fn wdog(&self) -> Option<Owned<wdog::Wdog>> { wdog::Wdog::acquire() }
    pub fn rtc(&self) -> Option<Owned<rtc::Rtc>> { rtc::Rtc::acquire() }
    pub fn smc(&self) -> Option<Owned<smc::Smc>> { smc::Smc::acquire() }
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
    pub fn ftm3(&self) -> Option<Owned<ftm::Ftm3>> { ftm::Ftm3::acquire() }
        pub fn ftm3_ch0(&self) -> Option<Owned<ftm::Ftm3Ch0>> { ftm::Ftm3Ch0::acquire() }
        pub fn ftm3_ch1(&self) -> Option<Owned<ftm::Ftm3Ch1>> { ftm::Ftm3Ch1::acquire() }
        pub fn ftm3_ch2(&self) -> Option<Owned<ftm::Ftm3Ch2>> { ftm::Ftm3Ch2::acquire() }
        pub fn ftm3_ch3(&self) -> Option<Owned<ftm::Ftm3Ch3>> { ftm::Ftm3Ch3::acquire() }
        pub fn ftm3_ch4(&self) -> Option<Owned<ftm::Ftm3Ch4>> { ftm::Ftm3Ch4::acquire() }
        pub fn ftm3_ch5(&self) -> Option<Owned<ftm::Ftm3Ch5>> { ftm::Ftm3Ch5::acquire() }
        pub fn ftm3_ch6(&self) -> Option<Owned<ftm::Ftm3Ch6>> { ftm::Ftm3Ch6::acquire() }
        pub fn ftm3_ch7(&self) -> Option<Owned<ftm::Ftm3Ch7>> { ftm::Ftm3Ch7::acquire() }
    pub fn lpit0(&self) -> Option<Owned<lpit::Lpit0>> { lpit::Lpit0::acquire() }
        pub fn lpit0_ch0(&self) -> Option<Owned<lpit::Lpit0Ch0>> { lpit::Lpit0Ch0::acquire() }
        pub fn lpit0_ch1(&self) -> Option<Owned<lpit::Lpit0Ch1>> { lpit::Lpit0Ch1::acquire() }
        pub fn lpit0_ch2(&self) -> Option<Owned<lpit::Lpit0Ch2>> { lpit::Lpit0Ch2::acquire() }
        pub fn lpit0_ch3(&self) -> Option<Owned<lpit::Lpit0Ch3>> { lpit::Lpit0Ch3::acquire() }
    pub fn lptmr0(&self) -> Option<Owned<lptmr::Lptmr0>> { lptmr::Lptmr0::acquire() }
    pub fn can0(&self) -> Option<Owned<flexcan::Can0>> { flexcan::Can0::acquire() }
    pub fn can1(&self) -> Option<Owned<flexcan::Can1>> { flexcan::Can1::acquire() }
    pub fn can2(&self) -> Option<Owned<flexcan::Can2>> { flexcan::Can2::acquire() }
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
        pub fn ptb14(&self) -> Option<Owned<pin::Ptb14>> { pin::Ptb14::acquire() }
        pub fn ptb15(&self) -> Option<Owned<pin::Ptb15>> { pin::Ptb15::acquire() }
        pub fn ptb16(&self) -> Option<Owned<pin::Ptb16>> { pin::Ptb16::acquire() }
        pub fn ptb17(&self) -> Option<Owned<pin::Ptb17>> { pin::Ptb17::acquire() }
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
        pub fn ptd16(&self) -> Option<Owned<pin::Ptd16>> { pin::Ptd16::acquire() }
        pub fn ptd17(&self) -> Option<Owned<pin::Ptd17>> { pin::Ptd17::acquire() }
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
        pub fn pte13(&self) -> Option<Owned<pin::Pte13>> { pin::Pte13::acquire() }
        pub fn pte14(&self) -> Option<Owned<pin::Pte14>> { pin::Pte14::acquire() }
        pub fn pte15(&self) -> Option<Owned<pin::Pte15>> { pin::Pte15::acquire() }
        pub fn pte16(&self) -> Option<Owned<pin::Pte16>> { pin::Pte16::acquire() }
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
        pub fn pb14(&self) -> Option<Owned<pin::Pb14>> { pin::Pb14::acquire() }
        pub fn pb15(&self) -> Option<Owned<pin::Pb15>> { pin::Pb15::acquire() }
        pub fn pb16(&self) -> Option<Owned<pin::Pb16>> { pin::Pb16::acquire() }
        pub fn pb17(&self) -> Option<Owned<pin::Pb17>> { pin::Pb17::acquire() }
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
        pub fn pd16(&self) -> Option<Owned<pin::Pd16>> { pin::Pd16::acquire() }
        pub fn pd17(&self) -> Option<Owned<pin::Pd17>> { pin::Pd17::acquire() }
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
        pub fn pe13(&self) -> Option<Owned<pin::Pe13>> { pin::Pe13::acquire() }
        pub fn pe14(&self) -> Option<Owned<pin::Pe14>> { pin::Pe14::acquire() }
        pub fn pe15(&self) -> Option<Owned<pin::Pe15>> { pin::Pe15::acquire() }
        pub fn pe16(&self) -> Option<Owned<pin::Pe16>> { pin::Pe16::acquire() }
    pub fn lpuart0(&self) -> Option<Owned<lpuart::Lpuart0>> { lpuart::Lpuart0::acquire() }
    pub fn lpuart1(&self) -> Option<Owned<lpuart::Lpuart1>> { lpuart::Lpuart1::acquire() }
    pub fn lpuart2(&self) -> Option<Owned<lpuart::Lpuart2>> { lpuart::Lpuart2::acquire() }
    pub fn lpi2c0(&self) -> Option<Owned<lpi2c::Lpi2c0>> { lpi2c::Lpi2c0::acquire() }
    pub fn lpspi0(&self) -> Option<Owned<lpspi::Lpspi0>> { lpspi::Lpspi0::acquire() }
    pub fn lpspi1(&self) -> Option<Owned<lpspi::Lpspi1>> { lpspi::Lpspi1::acquire() }
    pub fn lpspi2(&self) -> Option<Owned<lpspi::Lpspi2>> { lpspi::Lpspi2::acquire() }
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
        pub fn adc0_in0(&self) -> Option<Owned<adc::Adc0In0>> { adc::Adc0In0::acquire() }
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
        pub fn adc1_in0(&self) -> Option<Owned<adc::Adc1In0>> { adc::Adc1In0::acquire() }
        pub fn adc1_bandgap(&self) -> Option<Owned<adc::Adc1Bandgap>> { adc::Adc1Bandgap::acquire() }
        pub fn adc1_refsh(&self) -> Option<Owned<adc::Adc1Refsh>> { adc::Adc1Refsh::acquire() }
        pub fn adc1_refsl(&self) -> Option<Owned<adc::Adc1Refsl>> { adc::Adc1Refsl::acquire() }
}

impl Get<sim::Sim> for S32k {
    fn get(&self) -> sim::Sim { sim::SIM }
}

impl GetPeriph<sim::SimPeriph> for S32k {
    fn get_periph(&self) -> sim::SimPeriph { sim::SIM_PERIPH }
}

impl GetPeriphInstance<sim::SimPeriph> for S32k {
    fn get_periph_instance(&self, index: usize) -> Option<sim::SimPeriph> {
        match index { 
            0 => Some(sim::SIM_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<scg::Scg> for S32k {
    fn get(&self) -> scg::Scg { scg::SCG }
}

impl GetPeriph<scg::ScgPeriph> for S32k {
    fn get_periph(&self) -> scg::ScgPeriph { scg::SCG_PERIPH }
}

impl GetPeriphInstance<scg::ScgPeriph> for S32k {
    fn get_periph_instance(&self, index: usize) -> Option<scg::ScgPeriph> {
        match index { 
            0 => Some(scg::SCG_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<pcc::Pcc> for S32k {
    fn get(&self) -> pcc::Pcc { pcc::PCC }
}

impl GetPeriph<pcc::PccPeriph> for S32k {
    fn get_periph(&self) -> pcc::PccPeriph { pcc::PCC_PERIPH }
}

impl GetPeriphInstance<pcc::PccPeriph> for S32k {
    fn get_periph_instance(&self, index: usize) -> Option<pcc::PccPeriph> {
        match index { 
            0 => Some(pcc::PCC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<rcm::Rcm> for S32k {
    fn get(&self) -> rcm::Rcm { rcm::RCM }
}

impl GetPeriph<rcm::RcmPeriph> for S32k {
    fn get_periph(&self) -> rcm::RcmPeriph { rcm::RCM_PERIPH }
}

impl GetPeriphInstance<rcm::RcmPeriph> for S32k {
    fn get_periph_instance(&self, index: usize) -> Option<rcm::RcmPeriph> {
        match index { 
            0 => Some(rcm::RCM_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<wdog::Wdog> for S32k {
    fn get(&self) -> wdog::Wdog { wdog::WDOG }
}

impl GetPeriph<wdog::WdogPeriph> for S32k {
    fn get_periph(&self) -> wdog::WdogPeriph { wdog::WDOG_PERIPH }
}

impl GetPeriphInstance<wdog::WdogPeriph> for S32k {
    fn get_periph_instance(&self, index: usize) -> Option<wdog::WdogPeriph> {
        match index { 
            0 => Some(wdog::WDOG_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<rtc::Rtc> for S32k {
    fn get(&self) -> rtc::Rtc { rtc::RTC }
}

impl GetPeriph<rtc::RtcPeriph> for S32k {
    fn get_periph(&self) -> rtc::RtcPeriph { rtc::RTC_PERIPH }
}

impl GetPeriphInstance<rtc::RtcPeriph> for S32k {
    fn get_periph_instance(&self, index: usize) -> Option<rtc::RtcPeriph> {
        match index { 
            0 => Some(rtc::RTC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<smc::Smc> for S32k {
    fn get(&self) -> smc::Smc { smc::SMC }
}

impl GetPeriph<smc::SmcPeriph> for S32k {
    fn get_periph(&self) -> smc::SmcPeriph { smc::SMC_PERIPH }
}

impl GetPeriphInstance<smc::SmcPeriph> for S32k {
    fn get_periph_instance(&self, index: usize) -> Option<smc::SmcPeriph> {
        match index { 
            0 => Some(smc::SMC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<crc::Crc> for S32k {
    fn get(&self) -> crc::Crc { crc::CRC }
}

impl GetPeriph<crc::CrcPeriph> for S32k {
    fn get_periph(&self) -> crc::CrcPeriph { crc::CRC_PERIPH }
}

impl GetPeriphInstance<crc::CrcPeriph> for S32k {
    fn get_periph_instance(&self, index: usize) -> Option<crc::CrcPeriph> {
        match index {
            0 => Some(crc::CRC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<dmamux::Dmamux> for S32k {
    fn get(&self) -> dmamux::Dmamux { dmamux::DMAMUX }
}

impl GetPeriph<dmamux::DmamuxPeriph> for S32k {
    fn get_periph(&self) -> dmamux::DmamuxPeriph { dmamux::DMAMUX_PERIPH }
}

impl GetPeriphInstance<dmamux::DmamuxPeriph> for S32k {
    fn get_periph_instance(&self, index: usize) -> Option<dmamux::DmamuxPeriph> {
        match index {
            0 => Some(dmamux::DMAMUX_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<edma::Dma> for S32k {
    fn get(&self) -> edma::Dma { edma::DMA }
}

impl GetPeriph<edma::EdmaPeriph> for S32k {
    fn get_periph(&self) -> edma::EdmaPeriph { edma::DMA_PERIPH }
}

impl GetPeriphInstance<edma::EdmaPeriph> for S32k {
    fn get_periph_instance(&self, index: usize) -> Option<edma::EdmaPeriph> {
        match index {
            0 => Some(edma::DMA_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<ftm::Ftm0> for S32k {
    fn get(&self) -> ftm::Ftm0 { ftm::FTM0 }
}

impl Get<ftm::Ftm1> for S32k {
    fn get(&self) -> ftm::Ftm1 { ftm::FTM1 }
}

impl Get<ftm::Ftm2> for S32k {
    fn get(&self) -> ftm::Ftm2 { ftm::FTM2 }
}

impl Get<ftm::Ftm3> for S32k {
    fn get(&self) -> ftm::Ftm3 { ftm::FTM3 }
}

impl GetPeriphInstance<ftm::FtmPeriph> for S32k {
    fn get_periph_instance(&self, index: usize) -> Option<ftm::FtmPeriph> {
        match index {
            0 => Some(ftm::FTM0_PERIPH),
            1 => Some(ftm::FTM1_PERIPH),
            2 => Some(ftm::FTM2_PERIPH),
            3 => Some(ftm::FTM3_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 4 }
}

impl Get<lpit::Lpit0> for S32k {
    fn get(&self) -> lpit::Lpit0 { lpit::LPIT0 }
}

impl GetPeriph<lpit::LpitPeriph> for S32k {
    fn get_periph(&self) -> lpit::LpitPeriph { lpit::LPIT0_PERIPH }
}

impl GetPeriphInstance<lpit::LpitPeriph> for S32k {
    fn get_periph_instance(&self, index: usize) -> Option<lpit::LpitPeriph> {
        match index {
            0 => Some(lpit::LPIT0_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<lptmr::Lptmr0> for S32k {
    fn get(&self) -> lptmr::Lptmr0 { lptmr::LPTMR0 }
}

impl GetPeriph<lptmr::LptmrPeriph> for S32k {
    fn get_periph(&self) -> lptmr::LptmrPeriph { lptmr::LPTMR0_PERIPH }
}

impl GetPeriphInstance<lptmr::LptmrPeriph> for S32k {
    fn get_periph_instance(&self, index: usize) -> Option<lptmr::LptmrPeriph> {
        match index {
            0 => Some(lptmr::LPTMR0_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<flexcan::Can0> for S32k {
    fn get(&self) -> flexcan::Can0 { flexcan::CAN0 }
}

impl Get<flexcan::Can1> for S32k {
    fn get(&self) -> flexcan::Can1 { flexcan::CAN1 }
}

impl Get<flexcan::Can2> for S32k {
    fn get(&self) -> flexcan::Can2 { flexcan::CAN2 }
}

impl GetPeriphInstance<flexcan::FlexcanPeriph> for S32k {
    fn get_periph_instance(&self, index: usize) -> Option<flexcan::FlexcanPeriph> {
        match index {
            0 => Some(flexcan::CAN0_PERIPH),
            1 => Some(flexcan::CAN1_PERIPH),
            2 => Some(flexcan::CAN2_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 3 }
}

impl Get<port::Porta> for S32k {
    fn get(&self) -> port::Porta { port::PORTA }
}

impl Get<port::Portb> for S32k {
    fn get(&self) -> port::Portb { port::PORTB }
}

impl Get<port::Portc> for S32k {
    fn get(&self) -> port::Portc { port::PORTC }
}

impl Get<port::Portd> for S32k {
    fn get(&self) -> port::Portd { port::PORTD }
}

impl Get<port::Porte> for S32k {
    fn get(&self) -> port::Porte { port::PORTE }
}

impl GetPeriphInstance<port::PortPeriph> for S32k {
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

impl Get<gpio::Gpioa> for S32k {
    fn get(&self) -> gpio::Gpioa { gpio::GPIOA }
}

impl Get<gpio::Gpiob> for S32k {
    fn get(&self) -> gpio::Gpiob { gpio::GPIOB }
}

impl Get<gpio::Gpioc> for S32k {
    fn get(&self) -> gpio::Gpioc { gpio::GPIOC }
}

impl Get<gpio::Gpiod> for S32k {
    fn get(&self) -> gpio::Gpiod { gpio::GPIOD }
}

impl Get<gpio::Gpioe> for S32k {
    fn get(&self) -> gpio::Gpioe { gpio::GPIOE }
}

impl GetPeriphInstance<gpio::GpioPeriph> for S32k {
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

impl Get<lpuart::Lpuart0> for S32k {
    fn get(&self) -> lpuart::Lpuart0 { lpuart::LPUART0 }
}

impl Get<lpuart::Lpuart1> for S32k {
    fn get(&self) -> lpuart::Lpuart1 { lpuart::LPUART1 }
}

impl Get<lpuart::Lpuart2> for S32k {
    fn get(&self) -> lpuart::Lpuart2 { lpuart::LPUART2 }
}

impl GetPeriphInstance<lpuart::LpuartPeriph> for S32k {
    fn get_periph_instance(&self, index: usize) -> Option<lpuart::LpuartPeriph> {
        match index {
            0 => Some(lpuart::LPUART0_PERIPH),
            1 => Some(lpuart::LPUART1_PERIPH),
            2 => Some(lpuart::LPUART2_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 3 }
}

impl Get<lpi2c::Lpi2c0> for S32k {
    fn get(&self) -> lpi2c::Lpi2c0 { lpi2c::LPI2C0 }
}

impl GetPeriph<lpi2c::Lpi2cPeriph> for S32k {
    fn get_periph(&self) -> lpi2c::Lpi2cPeriph { lpi2c::LPI2C0_PERIPH }
}

impl GetPeriphInstance<lpi2c::Lpi2cPeriph> for S32k {
    fn get_periph_instance(&self, index: usize) -> Option<lpi2c::Lpi2cPeriph> {
        match index {
            0 => Some(lpi2c::LPI2C0_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<lpspi::Lpspi0> for S32k {
    fn get(&self) -> lpspi::Lpspi0 { lpspi::LPSPI0 }
}

impl Get<lpspi::Lpspi1> for S32k {
    fn get(&self) -> lpspi::Lpspi1 { lpspi::LPSPI1 }
}

impl Get<lpspi::Lpspi2> for S32k {
    fn get(&self) -> lpspi::Lpspi2 { lpspi::LPSPI2 }
}

impl GetPeriphInstance<lpspi::LpspiPeriph> for S32k {
    fn get_periph_instance(&self, index: usize) -> Option<lpspi::LpspiPeriph> {
        match index {
            0 => Some(lpspi::LPSPI0_PERIPH),
            1 => Some(lpspi::LPSPI1_PERIPH),
            2 => Some(lpspi::LPSPI2_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 3 }
}

impl Get<adc::Adc0> for S32k {
    fn get(&self) -> adc::Adc0 { adc::ADC0 }
}

impl Get<adc::Adc1> for S32k {
    fn get(&self) -> adc::Adc1 { adc::ADC1 }
}

impl GetPeriphInstance<adc::AdcPeriph> for S32k {
    fn get_periph_instance(&self, index: usize) -> Option<adc::AdcPeriph> {
        match index {
            0 => Some(adc::ADC0_PERIPH),
            1 => Some(adc::ADC1_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

