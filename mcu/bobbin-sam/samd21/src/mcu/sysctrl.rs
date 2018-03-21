#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::sysctrl::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( SYSCTRL, Sysctrl, SYSCTRL_PERIPH, SysctrlPeriph, 0x40000800, 0x00, 0x03);


// Gate { name: None, gate_type: Some("EN"), periph: Some("PM"), register: Some("APBAMASK"), field: Some("SYSCTRL"), description: None }
impl ::bobbin_common::gate::GateEn for Sysctrl {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::pm::PM.apbamask().sysctrl() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::pm::PM.with_apbamask(|r| r.set_sysctrl(value));
        self
    }
}

