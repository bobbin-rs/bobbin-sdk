#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::pcc::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( PCC, Pcc, PCC_PERIPH, PccPeriph, 0x40065000, 0x02);

impl Cgc for super::flexcan::Can0 {
    #[inline] fn cgc(&self) -> bits::U1 { PCC.flexcan0().cgc() }
    #[inline] fn set_cgc<V: Into<bits::U1>>(&self, value: V) { PCC.with_flexcan0(|r| r.set_cgc(value)); }
}

impl Cgc for super::flexcan::Can1 {
    #[inline] fn cgc(&self) -> bits::U1 { PCC.flexcan1().cgc() }
    #[inline] fn set_cgc<V: Into<bits::U1>>(&self, value: V) { PCC.with_flexcan1(|r| r.set_cgc(value)); }
}

impl Pcs for super::ftm::Ftm3 {
    #[inline] fn pcs(&self) -> bits::U3 { PCC.ftm3().pcs() }
    #[inline] fn set_pcs<V: Into<bits::U3>>(&self, value: V) { PCC.with_ftm3(|r| r.set_pcs(value)); }
}

impl Pcs for super::adc::Adc1 {
    #[inline] fn pcs(&self) -> bits::U3 { PCC.adc1().pcs() }
    #[inline] fn set_pcs<V: Into<bits::U3>>(&self, value: V) { PCC.with_adc1(|r| r.set_pcs(value)); }
}

impl Cgc for super::adc::Adc1 {
    #[inline] fn cgc(&self) -> bits::U1 { PCC.adc1().cgc() }
    #[inline] fn set_cgc<V: Into<bits::U1>>(&self, value: V) { PCC.with_adc1(|r| r.set_cgc(value)); }
}

impl Cgc for super::flexcan::Can2 {
    #[inline] fn cgc(&self) -> bits::U1 { PCC.flexcan2().cgc() }
    #[inline] fn set_cgc<V: Into<bits::U1>>(&self, value: V) { PCC.with_flexcan2(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpspi::Lpspi0 {
    #[inline] fn pcs(&self) -> bits::U3 { PCC.lpspi0().pcs() }
    #[inline] fn set_pcs<V: Into<bits::U3>>(&self, value: V) { PCC.with_lpspi0(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpspi::Lpspi0 {
    #[inline] fn cgc(&self) -> bits::U1 { PCC.lpspi0().cgc() }
    #[inline] fn set_cgc<V: Into<bits::U1>>(&self, value: V) { PCC.with_lpspi0(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpspi::Lpspi1 {
    #[inline] fn pcs(&self) -> bits::U3 { PCC.lpspi1().pcs() }
    #[inline] fn set_pcs<V: Into<bits::U3>>(&self, value: V) { PCC.with_lpspi1(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpspi::Lpspi1 {
    #[inline] fn cgc(&self) -> bits::U1 { PCC.lpspi1().cgc() }
    #[inline] fn set_cgc<V: Into<bits::U1>>(&self, value: V) { PCC.with_lpspi1(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpspi::Lpspi2 {
    #[inline] fn pcs(&self) -> bits::U3 { PCC.lpspi2().pcs() }
    #[inline] fn set_pcs<V: Into<bits::U3>>(&self, value: V) { PCC.with_lpspi2(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpspi::Lpspi2 {
    #[inline] fn cgc(&self) -> bits::U1 { PCC.lpspi2().cgc() }
    #[inline] fn set_cgc<V: Into<bits::U1>>(&self, value: V) { PCC.with_lpspi2(|r| r.set_cgc(value)); }
}

impl Cgc for super::crc::Crc {
    #[inline] fn cgc(&self) -> bits::U1 { PCC.crc().cgc() }
    #[inline] fn set_cgc<V: Into<bits::U1>>(&self, value: V) { PCC.with_crc(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpit::Lpit0 {
    #[inline] fn pcs(&self) -> bits::U3 { PCC.lpit().pcs() }
    #[inline] fn set_pcs<V: Into<bits::U3>>(&self, value: V) { PCC.with_lpit(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpit::Lpit0 {
    #[inline] fn cgc(&self) -> bits::U1 { PCC.lpit().cgc() }
    #[inline] fn set_cgc<V: Into<bits::U1>>(&self, value: V) { PCC.with_lpit(|r| r.set_cgc(value)); }
}

impl Pcs for super::ftm::Ftm0 {
    #[inline] fn pcs(&self) -> bits::U3 { PCC.ftm0().pcs() }
    #[inline] fn set_pcs<V: Into<bits::U3>>(&self, value: V) { PCC.with_ftm0(|r| r.set_pcs(value)); }
}

impl Cgc for super::ftm::Ftm0 {
    #[inline] fn cgc(&self) -> bits::U1 { PCC.ftm0().cgc() }
    #[inline] fn set_cgc<V: Into<bits::U1>>(&self, value: V) { PCC.with_ftm0(|r| r.set_cgc(value)); }
}

impl Pcs for super::ftm::Ftm1 {
    #[inline] fn pcs(&self) -> bits::U3 { PCC.ftm1().pcs() }
    #[inline] fn set_pcs<V: Into<bits::U3>>(&self, value: V) { PCC.with_ftm1(|r| r.set_pcs(value)); }
}

impl Cgc for super::ftm::Ftm1 {
    #[inline] fn cgc(&self) -> bits::U1 { PCC.ftm1().cgc() }
    #[inline] fn set_cgc<V: Into<bits::U1>>(&self, value: V) { PCC.with_ftm1(|r| r.set_cgc(value)); }
}

impl Pcs for super::ftm::Ftm2 {
    #[inline] fn pcs(&self) -> bits::U3 { PCC.ftm2().pcs() }
    #[inline] fn set_pcs<V: Into<bits::U3>>(&self, value: V) { PCC.with_ftm2(|r| r.set_pcs(value)); }
}

impl Cgc for super::ftm::Ftm2 {
    #[inline] fn cgc(&self) -> bits::U1 { PCC.ftm2().cgc() }
    #[inline] fn set_cgc<V: Into<bits::U1>>(&self, value: V) { PCC.with_ftm2(|r| r.set_cgc(value)); }
}

impl Pcs for super::adc::Adc0 {
    #[inline] fn pcs(&self) -> bits::U3 { PCC.adc0().pcs() }
    #[inline] fn set_pcs<V: Into<bits::U3>>(&self, value: V) { PCC.with_adc0(|r| r.set_pcs(value)); }
}

impl Cgc for super::adc::Adc0 {
    #[inline] fn cgc(&self) -> bits::U1 { PCC.adc0().cgc() }
    #[inline] fn set_cgc<V: Into<bits::U1>>(&self, value: V) { PCC.with_adc0(|r| r.set_cgc(value)); }
}

impl Cgc for super::rtc::Rtc {
    #[inline] fn cgc(&self) -> bits::U1 { PCC.rtc().cgc() }
    #[inline] fn set_cgc<V: Into<bits::U1>>(&self, value: V) { PCC.with_rtc(|r| r.set_cgc(value)); }
}

impl Pcd for super::lptmr::Lptmr0 {
    #[inline] fn pcd(&self) -> bits::U3 { PCC.lptmr0().pcd() }
    #[inline] fn set_pcd<V: Into<bits::U3>>(&self, value: V) { PCC.with_lptmr0(|r| r.set_pcd(value)); }
}

impl Frac for super::lptmr::Lptmr0 {
    #[inline] fn frac(&self) -> bits::U1 { PCC.lptmr0().frac() }
    #[inline] fn set_frac<V: Into<bits::U1>>(&self, value: V) { PCC.with_lptmr0(|r| r.set_frac(value)); }
}

impl Pcs for super::lptmr::Lptmr0 {
    #[inline] fn pcs(&self) -> bits::U3 { PCC.lptmr0().pcs() }
    #[inline] fn set_pcs<V: Into<bits::U3>>(&self, value: V) { PCC.with_lptmr0(|r| r.set_pcs(value)); }
}

impl Cgc for super::lptmr::Lptmr0 {
    #[inline] fn cgc(&self) -> bits::U1 { PCC.lptmr0().cgc() }
    #[inline] fn set_cgc<V: Into<bits::U1>>(&self, value: V) { PCC.with_lptmr0(|r| r.set_cgc(value)); }
}

impl Cgc for super::port::Porta {
    #[inline] fn cgc(&self) -> bits::U1 { PCC.porta().cgc() }
    #[inline] fn set_cgc<V: Into<bits::U1>>(&self, value: V) { PCC.with_porta(|r| r.set_cgc(value)); }
}

impl Cgc for super::port::Portb {
    #[inline] fn cgc(&self) -> bits::U1 { PCC.portb().cgc() }
    #[inline] fn set_cgc<V: Into<bits::U1>>(&self, value: V) { PCC.with_portb(|r| r.set_cgc(value)); }
}

impl Cgc for super::port::Portc {
    #[inline] fn cgc(&self) -> bits::U1 { PCC.portc().cgc() }
    #[inline] fn set_cgc<V: Into<bits::U1>>(&self, value: V) { PCC.with_portc(|r| r.set_cgc(value)); }
}

impl Cgc for super::port::Portd {
    #[inline] fn cgc(&self) -> bits::U1 { PCC.portd().cgc() }
    #[inline] fn set_cgc<V: Into<bits::U1>>(&self, value: V) { PCC.with_portd(|r| r.set_cgc(value)); }
}

impl Cgc for super::port::Porte {
    #[inline] fn cgc(&self) -> bits::U1 { PCC.porte().cgc() }
    #[inline] fn set_cgc<V: Into<bits::U1>>(&self, value: V) { PCC.with_porte(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpi2c::Lpi2c0 {
    #[inline] fn pcs(&self) -> bits::U3 { PCC.lpi2c0().pcs() }
    #[inline] fn set_pcs<V: Into<bits::U3>>(&self, value: V) { PCC.with_lpi2c0(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpi2c::Lpi2c0 {
    #[inline] fn cgc(&self) -> bits::U1 { PCC.lpi2c0().cgc() }
    #[inline] fn set_cgc<V: Into<bits::U1>>(&self, value: V) { PCC.with_lpi2c0(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpuart::Lpuart0 {
    #[inline] fn pcs(&self) -> bits::U3 { PCC.lpuart0().pcs() }
    #[inline] fn set_pcs<V: Into<bits::U3>>(&self, value: V) { PCC.with_lpuart0(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpuart::Lpuart0 {
    #[inline] fn cgc(&self) -> bits::U1 { PCC.lpuart0().cgc() }
    #[inline] fn set_cgc<V: Into<bits::U1>>(&self, value: V) { PCC.with_lpuart0(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpuart::Lpuart1 {
    #[inline] fn pcs(&self) -> bits::U3 { PCC.lpuart1().pcs() }
    #[inline] fn set_pcs<V: Into<bits::U3>>(&self, value: V) { PCC.with_lpuart1(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpuart::Lpuart1 {
    #[inline] fn cgc(&self) -> bits::U1 { PCC.lpuart1().cgc() }
    #[inline] fn set_cgc<V: Into<bits::U1>>(&self, value: V) { PCC.with_lpuart1(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpuart::Lpuart2 {
    #[inline] fn pcs(&self) -> bits::U3 { PCC.lpuart2().pcs() }
    #[inline] fn set_pcs<V: Into<bits::U3>>(&self, value: V) { PCC.with_lpuart2(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpuart::Lpuart2 {
    #[inline] fn cgc(&self) -> bits::U1 { PCC.lpuart2().cgc() }
    #[inline] fn set_cgc<V: Into<bits::U1>>(&self, value: V) { PCC.with_lpuart2(|r| r.set_cgc(value)); }
}


