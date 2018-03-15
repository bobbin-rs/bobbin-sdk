#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::usb_fs_host::*;

periph!( USB_FS_HOST, UsbFsHost, USB_FS_HOST_PERIPH, UsbFsHostPeriph, 0x50000400, 0x15);

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHB2RSTR"), field: Some("OTGFSRST"), description: None }
impl ::bobbin_common::gate::GateRst for UsbFsHost {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.ahb2rstr().otgfsrst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2rstr(|r| r.set_otgfsrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHB2ENR"), field: Some("OTGFSEN"), description: None }
impl ::bobbin_common::gate::GateEn for UsbFsHost {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.ahb2enr().otgfsen() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2enr(|r| r.set_otgfsen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("AHB2LPENR"), field: Some("OTGFSLPEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for UsbFsHost {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.ahb2lpenr().otgfslpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2lpenr(|r| r.set_otgfslpen(value));
        self
    }
}

