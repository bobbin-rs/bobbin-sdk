#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::dmac::*;

periph!( DMAC, Dmac, DMAC_PERIPH, DmacPeriph, DMAC_OWNED, DMAC_REF_COUNT, 0x41004800, 0x00, 0x06);

channel!(DMAC_CH0, DmacCh0, dmac_ch0, DMAC, Dmac, DMAC_CH0_CH, DmacCh, DMAC_PERIPH, DMAC_CH0_OWNED, DMAC_CH0_REF_COUNT, 0);
channel!(DMAC_CH1, DmacCh1, dmac_ch1, DMAC, Dmac, DMAC_CH1_CH, DmacCh, DMAC_PERIPH, DMAC_CH1_OWNED, DMAC_CH1_REF_COUNT, 1);
channel!(DMAC_CH2, DmacCh2, dmac_ch2, DMAC, Dmac, DMAC_CH2_CH, DmacCh, DMAC_PERIPH, DMAC_CH2_OWNED, DMAC_CH2_REF_COUNT, 2);
channel!(DMAC_CH3, DmacCh3, dmac_ch3, DMAC, Dmac, DMAC_CH3_CH, DmacCh, DMAC_PERIPH, DMAC_CH3_OWNED, DMAC_CH3_REF_COUNT, 3);
channel!(DMAC_CH4, DmacCh4, dmac_ch4, DMAC, Dmac, DMAC_CH4_CH, DmacCh, DMAC_PERIPH, DMAC_CH4_OWNED, DMAC_CH4_REF_COUNT, 4);
channel!(DMAC_CH5, DmacCh5, dmac_ch5, DMAC, Dmac, DMAC_CH5_CH, DmacCh, DMAC_PERIPH, DMAC_CH5_OWNED, DMAC_CH5_REF_COUNT, 5);
channel!(DMAC_CH6, DmacCh6, dmac_ch6, DMAC, Dmac, DMAC_CH6_CH, DmacCh, DMAC_PERIPH, DMAC_CH6_OWNED, DMAC_CH6_REF_COUNT, 6);
channel!(DMAC_CH7, DmacCh7, dmac_ch7, DMAC, Dmac, DMAC_CH7_CH, DmacCh, DMAC_PERIPH, DMAC_CH7_OWNED, DMAC_CH7_REF_COUNT, 7);
channel!(DMAC_CH8, DmacCh8, dmac_ch8, DMAC, Dmac, DMAC_CH8_CH, DmacCh, DMAC_PERIPH, DMAC_CH8_OWNED, DMAC_CH8_REF_COUNT, 8);
channel!(DMAC_CH9, DmacCh9, dmac_ch9, DMAC, Dmac, DMAC_CH9_CH, DmacCh, DMAC_PERIPH, DMAC_CH9_OWNED, DMAC_CH9_REF_COUNT, 9);
channel!(DMAC_CH10, DmacCh10, dmac_ch10, DMAC, Dmac, DMAC_CH10_CH, DmacCh, DMAC_PERIPH, DMAC_CH10_OWNED, DMAC_CH10_REF_COUNT, 10);
channel!(DMAC_CH11, DmacCh11, dmac_ch11, DMAC, Dmac, DMAC_CH11_CH, DmacCh, DMAC_PERIPH, DMAC_CH11_OWNED, DMAC_CH11_REF_COUNT, 11);
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

