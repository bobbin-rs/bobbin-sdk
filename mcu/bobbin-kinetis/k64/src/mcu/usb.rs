#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::usb::*;

periph!( USB0, Usb0, USB0_PERIPH, UsbPeriph, 0x40072000, 0x1a);

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC4"), field: Some("USBOTG"), description: None }
impl ::bobbin_common::gate::GateEn for Usb0 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::sim::SIM.scgc4().usbotg() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc4(|r| r.set_usbotg(value));
        self
    }
}

