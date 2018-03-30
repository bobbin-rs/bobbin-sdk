#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::nvmctrl::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( NVMCTRL, Nvmctrl, NVMCTRL_PERIPH, NvmctrlPeriph, 0x41004000, 0x00, 0x01);


// Gate { name: None, gate_type: Some("EN"), periph: Some("PM"), register: Some("AHBMASK"), field: Some("NVMCTRL"), description: None }
impl ::bobbin_common::gate::GateEn for Nvmctrl {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::pm::PM.ahbmask().nvmctrl() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::pm::PM.with_ahbmask(|r| r.set_nvmctrl(value));
        self
    }
}

