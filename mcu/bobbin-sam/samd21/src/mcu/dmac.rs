#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::dmac::*;

// //! Direct Memory Access Controller

periph!( DMAC, Dmac, DMAC_PERIPH, DmacPeriph, 0x41004800, 0x06);

pub struct DmacCh { pub periph: DmacPeriph, pub index: usize }
channel!(DMAC_CH0, DmacCh0, DMAC, Dmac, DMAC_CH0_CH, DmacCh, DMAC_PERIPH, 0);
channel!(DMAC_CH1, DmacCh1, DMAC, Dmac, DMAC_CH1_CH, DmacCh, DMAC_PERIPH, 1);
channel!(DMAC_CH2, DmacCh2, DMAC, Dmac, DMAC_CH2_CH, DmacCh, DMAC_PERIPH, 2);
channel!(DMAC_CH3, DmacCh3, DMAC, Dmac, DMAC_CH3_CH, DmacCh, DMAC_PERIPH, 3);
channel!(DMAC_CH4, DmacCh4, DMAC, Dmac, DMAC_CH4_CH, DmacCh, DMAC_PERIPH, 4);
channel!(DMAC_CH5, DmacCh5, DMAC, Dmac, DMAC_CH5_CH, DmacCh, DMAC_PERIPH, 5);
channel!(DMAC_CH6, DmacCh6, DMAC, Dmac, DMAC_CH6_CH, DmacCh, DMAC_PERIPH, 6);
channel!(DMAC_CH7, DmacCh7, DMAC, Dmac, DMAC_CH7_CH, DmacCh, DMAC_PERIPH, 7);
channel!(DMAC_CH8, DmacCh8, DMAC, Dmac, DMAC_CH8_CH, DmacCh, DMAC_PERIPH, 8);
channel!(DMAC_CH9, DmacCh9, DMAC, Dmac, DMAC_CH9_CH, DmacCh, DMAC_PERIPH, 9);
channel!(DMAC_CH10, DmacCh10, DMAC, Dmac, DMAC_CH10_CH, DmacCh, DMAC_PERIPH, 10);
channel!(DMAC_CH11, DmacCh11, DMAC, Dmac, DMAC_CH11_CH, DmacCh, DMAC_PERIPH, 11);
// Gate { name: None, gate_type: Some("EN"), periph: Some("PM"), register: Some("AHBMASK"), field: Some("DMAC"), description: None }
impl ::bobbin_common::gate::GateEn for Dmac {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::pm::PM.ahbmask().dmac() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::pm::PM.with_ahbmask(|r| r.set_dmac(value));
        self
    }
}

