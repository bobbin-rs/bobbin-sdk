pub use kinetis_common::usb::*;

::bobbin_mcu::periph!( USB0, Usb0, USB0_PERIPH, UsbPeriph, USB0_OWNED, USB0_REF_COUNT, 0x40072000, 0x00, 0x1b);

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC4"), field: Some("USBOTG"), description: None }
impl ::bobbin_mcu::gate::GateEn for Usb0 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::sim::SIM.scgc4().usbotg() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc4(|r| r.set_usbotg(value));
        self
    }
}

