#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::usb_fs_device::*;

periph!( USB_FS_DEVICE, UsbFsDevice, USB_FS_DEVICE_PERIPH, UsbFsDevicePeriph, USB_FS_DEVICE_OWNED, USB_FS_DEVICE_REF_COUNT, 0x50000800, 0x00, 0x16);

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHB2RSTR"), field: Some("OTGFSRST"), description: None }
impl ::bobbin_common::gate::GateRst for UsbFsDevice {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.ahb2rstr().otgfsrst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2rstr(|r| r.set_otgfsrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHB2ENR"), field: Some("OTGFSEN"), description: None }
impl ::bobbin_common::gate::GateEn for UsbFsDevice {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.ahb2enr().otgfsen() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2enr(|r| r.set_otgfsen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("AHB2LPENR"), field: Some("OTGFSLPEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for UsbFsDevice {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.ahb2lpenr().otgfslpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2lpenr(|r| r.set_otgfslpen(value));
        self
    }
}

