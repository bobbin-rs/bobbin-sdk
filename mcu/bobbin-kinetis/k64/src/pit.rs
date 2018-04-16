pub use kinetis_common::pit::*;

::bobbin_mcu::periph!( PIT, Pit, PIT_PERIPH, PitPeriph, PIT_OWNED, PIT_REF_COUNT, 0x40037000, 0x00, 0x0e);

::bobbin_mcu::channel!(PIT_CH0, PitCh0, pit_ch0, PIT, Pit, PIT_CH0_CH, PitCh, PIT_PERIPH, PIT_CH0_OWNED, PIT_CH0_REF_COUNT, 0);
::bobbin_mcu::channel!(PIT_CH1, PitCh1, pit_ch1, PIT, Pit, PIT_CH1_CH, PitCh, PIT_PERIPH, PIT_CH1_OWNED, PIT_CH1_REF_COUNT, 1);
::bobbin_mcu::channel!(PIT_CH2, PitCh2, pit_ch2, PIT, Pit, PIT_CH2_CH, PitCh, PIT_PERIPH, PIT_CH2_OWNED, PIT_CH2_REF_COUNT, 2);
::bobbin_mcu::channel!(PIT_CH3, PitCh3, pit_ch3, PIT, Pit, PIT_CH3_CH, PitCh, PIT_PERIPH, PIT_CH3_OWNED, PIT_CH3_REF_COUNT, 3);
// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC6"), field: Some("PIT"), description: None }
impl ::bobbin_mcu::gate::GateEn for Pit {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::sim::SIM.scgc6().pit() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc6(|r| r.set_pit(value));
        self
    }
}

