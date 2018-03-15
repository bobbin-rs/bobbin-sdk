//! System Clock Generator

#[allow(unused_imports)] use ::bobbin_common::*;

#[doc="System Clock Generator"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ScgPeriph(pub usize);
impl ScgPeriph {
    #[doc="Get the *mut pointer for the VERID register."]
    #[inline] pub fn verid_mut(&self) -> *mut Verid { 
        (self.0 + 0x0) as *mut Verid
    }

    #[doc="Get the *const pointer for the VERID register."]
    #[inline] pub fn verid_ptr(&self) -> *const Verid { 
           self.verid_mut()
    }

    #[doc="Read the VERID register."]
    #[inline] pub fn verid(&self) -> Verid { 
        unsafe {
            read_volatile(self.verid_ptr())
        }
    }

    #[doc="Get the *mut pointer for the PARAM register."]
    #[inline] pub fn param_mut(&self) -> *mut Param { 
        (self.0 + 0x4) as *mut Param
    }

    #[doc="Get the *const pointer for the PARAM register."]
    #[inline] pub fn param_ptr(&self) -> *const Param { 
           self.param_mut()
    }

    #[doc="Read the PARAM register."]
    #[inline] pub fn param(&self) -> Param { 
        unsafe {
            read_volatile(self.param_ptr())
        }
    }

    #[doc="Get the *mut pointer for the CSR register."]
    #[inline] pub fn csr_mut(&self) -> *mut Csr { 
        (self.0 + 0x10) as *mut Csr
    }

    #[doc="Get the *const pointer for the CSR register."]
    #[inline] pub fn csr_ptr(&self) -> *const Csr { 
           self.csr_mut()
    }

    #[doc="Read the CSR register."]
    #[inline] pub fn csr(&self) -> Csr { 
        unsafe {
            read_volatile(self.csr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RCCR register."]
    #[inline] pub fn rccr_mut(&self) -> *mut Rccr { 
        (self.0 + 0x14) as *mut Rccr
    }

    #[doc="Get the *const pointer for the RCCR register."]
    #[inline] pub fn rccr_ptr(&self) -> *const Rccr { 
           self.rccr_mut()
    }

    #[doc="Read the RCCR register."]
    #[inline] pub fn rccr(&self) -> Rccr { 
        unsafe {
            read_volatile(self.rccr_ptr())
        }
    }

    #[doc="Write the RCCR register."]
    #[inline] pub fn set_rccr<F: FnOnce(Rccr) -> Rccr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rccr_mut(), f(Rccr(0)));
        }
        self
    }

    #[doc="Modify the RCCR register."]
    #[inline] pub fn with_rccr<F: FnOnce(Rccr) -> Rccr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rccr_mut(), f(self.rccr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the VCCR register."]
    #[inline] pub fn vccr_mut(&self) -> *mut Vccr { 
        (self.0 + 0x18) as *mut Vccr
    }

    #[doc="Get the *const pointer for the VCCR register."]
    #[inline] pub fn vccr_ptr(&self) -> *const Vccr { 
           self.vccr_mut()
    }

    #[doc="Read the VCCR register."]
    #[inline] pub fn vccr(&self) -> Vccr { 
        unsafe {
            read_volatile(self.vccr_ptr())
        }
    }

    #[doc="Write the VCCR register."]
    #[inline] pub fn set_vccr<F: FnOnce(Vccr) -> Vccr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.vccr_mut(), f(Vccr(0)));
        }
        self
    }

    #[doc="Modify the VCCR register."]
    #[inline] pub fn with_vccr<F: FnOnce(Vccr) -> Vccr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.vccr_mut(), f(self.vccr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCCR register."]
    #[inline] pub fn hccr_mut(&self) -> *mut Hccr { 
        (self.0 + 0x1c) as *mut Hccr
    }

    #[doc="Get the *const pointer for the HCCR register."]
    #[inline] pub fn hccr_ptr(&self) -> *const Hccr { 
           self.hccr_mut()
    }

    #[doc="Read the HCCR register."]
    #[inline] pub fn hccr(&self) -> Hccr { 
        unsafe {
            read_volatile(self.hccr_ptr())
        }
    }

    #[doc="Write the HCCR register."]
    #[inline] pub fn set_hccr<F: FnOnce(Hccr) -> Hccr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hccr_mut(), f(Hccr(0)));
        }
        self
    }

    #[doc="Modify the HCCR register."]
    #[inline] pub fn with_hccr<F: FnOnce(Hccr) -> Hccr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hccr_mut(), f(self.hccr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLKOUTCNFG register."]
    #[inline] pub fn clkoutcnfg_mut(&self) -> *mut Clkoutcnfg { 
        (self.0 + 0x20) as *mut Clkoutcnfg
    }

    #[doc="Get the *const pointer for the CLKOUTCNFG register."]
    #[inline] pub fn clkoutcnfg_ptr(&self) -> *const Clkoutcnfg { 
           self.clkoutcnfg_mut()
    }

    #[doc="Read the CLKOUTCNFG register."]
    #[inline] pub fn clkoutcnfg(&self) -> Clkoutcnfg { 
        unsafe {
            read_volatile(self.clkoutcnfg_ptr())
        }
    }

    #[doc="Write the CLKOUTCNFG register."]
    #[inline] pub fn set_clkoutcnfg<F: FnOnce(Clkoutcnfg) -> Clkoutcnfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clkoutcnfg_mut(), f(Clkoutcnfg(0)));
        }
        self
    }

    #[doc="Modify the CLKOUTCNFG register."]
    #[inline] pub fn with_clkoutcnfg<F: FnOnce(Clkoutcnfg) -> Clkoutcnfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clkoutcnfg_mut(), f(self.clkoutcnfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SOSCCSR register."]
    #[inline] pub fn sosccsr_mut(&self) -> *mut Sosccsr { 
        (self.0 + 0x100) as *mut Sosccsr
    }

    #[doc="Get the *const pointer for the SOSCCSR register."]
    #[inline] pub fn sosccsr_ptr(&self) -> *const Sosccsr { 
           self.sosccsr_mut()
    }

    #[doc="Read the SOSCCSR register."]
    #[inline] pub fn sosccsr(&self) -> Sosccsr { 
        unsafe {
            read_volatile(self.sosccsr_ptr())
        }
    }

    #[doc="Write the SOSCCSR register."]
    #[inline] pub fn set_sosccsr<F: FnOnce(Sosccsr) -> Sosccsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sosccsr_mut(), f(Sosccsr(0)));
        }
        self
    }

    #[doc="Modify the SOSCCSR register."]
    #[inline] pub fn with_sosccsr<F: FnOnce(Sosccsr) -> Sosccsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sosccsr_mut(), f(self.sosccsr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SOSCDIV register."]
    #[inline] pub fn soscdiv_mut(&self) -> *mut Soscdiv { 
        (self.0 + 0x104) as *mut Soscdiv
    }

    #[doc="Get the *const pointer for the SOSCDIV register."]
    #[inline] pub fn soscdiv_ptr(&self) -> *const Soscdiv { 
           self.soscdiv_mut()
    }

    #[doc="Read the SOSCDIV register."]
    #[inline] pub fn soscdiv(&self) -> Soscdiv { 
        unsafe {
            read_volatile(self.soscdiv_ptr())
        }
    }

    #[doc="Write the SOSCDIV register."]
    #[inline] pub fn set_soscdiv<F: FnOnce(Soscdiv) -> Soscdiv>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.soscdiv_mut(), f(Soscdiv(0)));
        }
        self
    }

    #[doc="Modify the SOSCDIV register."]
    #[inline] pub fn with_soscdiv<F: FnOnce(Soscdiv) -> Soscdiv>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.soscdiv_mut(), f(self.soscdiv()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SOSCCFG register."]
    #[inline] pub fn sosccfg_mut(&self) -> *mut Sosccfg { 
        (self.0 + 0x108) as *mut Sosccfg
    }

    #[doc="Get the *const pointer for the SOSCCFG register."]
    #[inline] pub fn sosccfg_ptr(&self) -> *const Sosccfg { 
           self.sosccfg_mut()
    }

    #[doc="Read the SOSCCFG register."]
    #[inline] pub fn sosccfg(&self) -> Sosccfg { 
        unsafe {
            read_volatile(self.sosccfg_ptr())
        }
    }

    #[doc="Write the SOSCCFG register."]
    #[inline] pub fn set_sosccfg<F: FnOnce(Sosccfg) -> Sosccfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sosccfg_mut(), f(Sosccfg(0)));
        }
        self
    }

    #[doc="Modify the SOSCCFG register."]
    #[inline] pub fn with_sosccfg<F: FnOnce(Sosccfg) -> Sosccfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sosccfg_mut(), f(self.sosccfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SIRCCSR register."]
    #[inline] pub fn sirccsr_mut(&self) -> *mut Sirccsr { 
        (self.0 + 0x200) as *mut Sirccsr
    }

    #[doc="Get the *const pointer for the SIRCCSR register."]
    #[inline] pub fn sirccsr_ptr(&self) -> *const Sirccsr { 
           self.sirccsr_mut()
    }

    #[doc="Read the SIRCCSR register."]
    #[inline] pub fn sirccsr(&self) -> Sirccsr { 
        unsafe {
            read_volatile(self.sirccsr_ptr())
        }
    }

    #[doc="Write the SIRCCSR register."]
    #[inline] pub fn set_sirccsr<F: FnOnce(Sirccsr) -> Sirccsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sirccsr_mut(), f(Sirccsr(0)));
        }
        self
    }

    #[doc="Modify the SIRCCSR register."]
    #[inline] pub fn with_sirccsr<F: FnOnce(Sirccsr) -> Sirccsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sirccsr_mut(), f(self.sirccsr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SIRCDIV register."]
    #[inline] pub fn sircdiv_mut(&self) -> *mut Sircdiv { 
        (self.0 + 0x204) as *mut Sircdiv
    }

    #[doc="Get the *const pointer for the SIRCDIV register."]
    #[inline] pub fn sircdiv_ptr(&self) -> *const Sircdiv { 
           self.sircdiv_mut()
    }

    #[doc="Read the SIRCDIV register."]
    #[inline] pub fn sircdiv(&self) -> Sircdiv { 
        unsafe {
            read_volatile(self.sircdiv_ptr())
        }
    }

    #[doc="Write the SIRCDIV register."]
    #[inline] pub fn set_sircdiv<F: FnOnce(Sircdiv) -> Sircdiv>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sircdiv_mut(), f(Sircdiv(0)));
        }
        self
    }

    #[doc="Modify the SIRCDIV register."]
    #[inline] pub fn with_sircdiv<F: FnOnce(Sircdiv) -> Sircdiv>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sircdiv_mut(), f(self.sircdiv()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SIRCCFG register."]
    #[inline] pub fn sirccfg_mut(&self) -> *mut Sirccfg { 
        (self.0 + 0x208) as *mut Sirccfg
    }

    #[doc="Get the *const pointer for the SIRCCFG register."]
    #[inline] pub fn sirccfg_ptr(&self) -> *const Sirccfg { 
           self.sirccfg_mut()
    }

    #[doc="Read the SIRCCFG register."]
    #[inline] pub fn sirccfg(&self) -> Sirccfg { 
        unsafe {
            read_volatile(self.sirccfg_ptr())
        }
    }

    #[doc="Write the SIRCCFG register."]
    #[inline] pub fn set_sirccfg<F: FnOnce(Sirccfg) -> Sirccfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sirccfg_mut(), f(Sirccfg(0)));
        }
        self
    }

    #[doc="Modify the SIRCCFG register."]
    #[inline] pub fn with_sirccfg<F: FnOnce(Sirccfg) -> Sirccfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sirccfg_mut(), f(self.sirccfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FIRCCSR register."]
    #[inline] pub fn firccsr_mut(&self) -> *mut Firccsr { 
        (self.0 + 0x300) as *mut Firccsr
    }

    #[doc="Get the *const pointer for the FIRCCSR register."]
    #[inline] pub fn firccsr_ptr(&self) -> *const Firccsr { 
           self.firccsr_mut()
    }

    #[doc="Read the FIRCCSR register."]
    #[inline] pub fn firccsr(&self) -> Firccsr { 
        unsafe {
            read_volatile(self.firccsr_ptr())
        }
    }

    #[doc="Write the FIRCCSR register."]
    #[inline] pub fn set_firccsr<F: FnOnce(Firccsr) -> Firccsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.firccsr_mut(), f(Firccsr(0)));
        }
        self
    }

    #[doc="Modify the FIRCCSR register."]
    #[inline] pub fn with_firccsr<F: FnOnce(Firccsr) -> Firccsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.firccsr_mut(), f(self.firccsr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FIRCDIV register."]
    #[inline] pub fn fircdiv_mut(&self) -> *mut Fircdiv { 
        (self.0 + 0x304) as *mut Fircdiv
    }

    #[doc="Get the *const pointer for the FIRCDIV register."]
    #[inline] pub fn fircdiv_ptr(&self) -> *const Fircdiv { 
           self.fircdiv_mut()
    }

    #[doc="Read the FIRCDIV register."]
    #[inline] pub fn fircdiv(&self) -> Fircdiv { 
        unsafe {
            read_volatile(self.fircdiv_ptr())
        }
    }

    #[doc="Write the FIRCDIV register."]
    #[inline] pub fn set_fircdiv<F: FnOnce(Fircdiv) -> Fircdiv>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fircdiv_mut(), f(Fircdiv(0)));
        }
        self
    }

    #[doc="Modify the FIRCDIV register."]
    #[inline] pub fn with_fircdiv<F: FnOnce(Fircdiv) -> Fircdiv>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fircdiv_mut(), f(self.fircdiv()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FIRCCFG register."]
    #[inline] pub fn firccfg_mut(&self) -> *mut Firccfg { 
        (self.0 + 0x308) as *mut Firccfg
    }

    #[doc="Get the *const pointer for the FIRCCFG register."]
    #[inline] pub fn firccfg_ptr(&self) -> *const Firccfg { 
           self.firccfg_mut()
    }

    #[doc="Read the FIRCCFG register."]
    #[inline] pub fn firccfg(&self) -> Firccfg { 
        unsafe {
            read_volatile(self.firccfg_ptr())
        }
    }

    #[doc="Write the FIRCCFG register."]
    #[inline] pub fn set_firccfg<F: FnOnce(Firccfg) -> Firccfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.firccfg_mut(), f(Firccfg(0)));
        }
        self
    }

    #[doc="Modify the FIRCCFG register."]
    #[inline] pub fn with_firccfg<F: FnOnce(Firccfg) -> Firccfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.firccfg_mut(), f(self.firccfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SPLLCSR register."]
    #[inline] pub fn spllcsr_mut(&self) -> *mut Spllcsr { 
        (self.0 + 0x600) as *mut Spllcsr
    }

    #[doc="Get the *const pointer for the SPLLCSR register."]
    #[inline] pub fn spllcsr_ptr(&self) -> *const Spllcsr { 
           self.spllcsr_mut()
    }

    #[doc="Read the SPLLCSR register."]
    #[inline] pub fn spllcsr(&self) -> Spllcsr { 
        unsafe {
            read_volatile(self.spllcsr_ptr())
        }
    }

    #[doc="Write the SPLLCSR register."]
    #[inline] pub fn set_spllcsr<F: FnOnce(Spllcsr) -> Spllcsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.spllcsr_mut(), f(Spllcsr(0)));
        }
        self
    }

    #[doc="Modify the SPLLCSR register."]
    #[inline] pub fn with_spllcsr<F: FnOnce(Spllcsr) -> Spllcsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.spllcsr_mut(), f(self.spllcsr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SPLLDIV register."]
    #[inline] pub fn splldiv_mut(&self) -> *mut Splldiv { 
        (self.0 + 0x604) as *mut Splldiv
    }

    #[doc="Get the *const pointer for the SPLLDIV register."]
    #[inline] pub fn splldiv_ptr(&self) -> *const Splldiv { 
           self.splldiv_mut()
    }

    #[doc="Read the SPLLDIV register."]
    #[inline] pub fn splldiv(&self) -> Splldiv { 
        unsafe {
            read_volatile(self.splldiv_ptr())
        }
    }

    #[doc="Write the SPLLDIV register."]
    #[inline] pub fn set_splldiv<F: FnOnce(Splldiv) -> Splldiv>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.splldiv_mut(), f(Splldiv(0)));
        }
        self
    }

    #[doc="Modify the SPLLDIV register."]
    #[inline] pub fn with_splldiv<F: FnOnce(Splldiv) -> Splldiv>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.splldiv_mut(), f(self.splldiv()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SPLLCFG register."]
    #[inline] pub fn spllcfg_mut(&self) -> *mut Spllcfg { 
        (self.0 + 0x608) as *mut Spllcfg
    }

    #[doc="Get the *const pointer for the SPLLCFG register."]
    #[inline] pub fn spllcfg_ptr(&self) -> *const Spllcfg { 
           self.spllcfg_mut()
    }

    #[doc="Read the SPLLCFG register."]
    #[inline] pub fn spllcfg(&self) -> Spllcfg { 
        unsafe {
            read_volatile(self.spllcfg_ptr())
        }
    }

    #[doc="Write the SPLLCFG register."]
    #[inline] pub fn set_spllcfg<F: FnOnce(Spllcfg) -> Spllcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.spllcfg_mut(), f(Spllcfg(0)));
        }
        self
    }

    #[doc="Modify the SPLLCFG register."]
    #[inline] pub fn with_spllcfg<F: FnOnce(Spllcfg) -> Spllcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.spllcfg_mut(), f(self.spllcfg()));
        }
        self
    }

}

#[doc="Version ID Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc="SCG Version Number"]
    #[inline] pub fn version(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VERSION != 0"]
    #[inline] pub fn test_version(&self) -> bool {
        self.version() != 0
    }

    #[doc="Sets the VERSION field."]
    #[inline] pub fn set_version<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Verid {
    #[inline]
    fn from(other: u32) -> Self {
         Verid(other)
    }
}

impl ::core::fmt::Display for Verid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Verid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Parameter Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Param(pub u32);
impl Param {
    #[doc="Clock Present"]
    #[inline] pub fn clkpres(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CLKPRES != 0"]
    #[inline] pub fn test_clkpres(&self) -> bool {
        self.clkpres() != 0
    }

    #[doc="Sets the CLKPRES field."]
    #[inline] pub fn set_clkpres<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Divider Present"]
    #[inline] pub fn divpres(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1f) as u8) } // [31:27]
    }

    #[doc="Returns true if DIVPRES != 0"]
    #[inline] pub fn test_divpres(&self) -> bool {
        self.divpres() != 0
    }

    #[doc="Sets the DIVPRES field."]
    #[inline] pub fn set_divpres<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 27);
        self.0 |= value << 27;
        self
    }

}

impl From<u32> for Param {
    #[inline]
    fn from(other: u32) -> Self {
         Param(other)
    }
}

impl ::core::fmt::Display for Param {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Param {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clkpres() != 0 { try!(write!(f, " clkpres=0x{:x}", self.clkpres()))}
        if self.divpres() != 0 { try!(write!(f, " divpres=0x{:x}", self.divpres()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc="Slow Clock Divide Ratio"]
    #[inline] pub fn divslow(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if DIVSLOW != 0"]
    #[inline] pub fn test_divslow(&self) -> bool {
        self.divslow() != 0
    }

    #[doc="Sets the DIVSLOW field."]
    #[inline] pub fn set_divslow<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Bus Clock Divide Ratio"]
    #[inline] pub fn divbus(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if DIVBUS != 0"]
    #[inline] pub fn test_divbus(&self) -> bool {
        self.divbus() != 0
    }

    #[doc="Sets the DIVBUS field."]
    #[inline] pub fn set_divbus<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Core Clock Divide Ratio"]
    #[inline] pub fn divcore(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if DIVCORE != 0"]
    #[inline] pub fn test_divcore(&self) -> bool {
        self.divcore() != 0
    }

    #[doc="Sets the DIVCORE field."]
    #[inline] pub fn set_divcore<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="System Clock Source"]
    #[inline] pub fn scs(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if SCS != 0"]
    #[inline] pub fn test_scs(&self) -> bool {
        self.scs() != 0
    }

    #[doc="Sets the SCS field."]
    #[inline] pub fn set_scs<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Csr {
    #[inline]
    fn from(other: u32) -> Self {
         Csr(other)
    }
}

impl ::core::fmt::Display for Csr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.divslow() != 0 { try!(write!(f, " divslow=0x{:x}", self.divslow()))}
        if self.divbus() != 0 { try!(write!(f, " divbus=0x{:x}", self.divbus()))}
        if self.divcore() != 0 { try!(write!(f, " divcore=0x{:x}", self.divcore()))}
        if self.scs() != 0 { try!(write!(f, " scs=0x{:x}", self.scs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Run Clock Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rccr(pub u32);
impl Rccr {
    #[doc="Slow Clock Divide Ratio"]
    #[inline] pub fn divslow(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if DIVSLOW != 0"]
    #[inline] pub fn test_divslow(&self) -> bool {
        self.divslow() != 0
    }

    #[doc="Sets the DIVSLOW field."]
    #[inline] pub fn set_divslow<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Bus Clock Divide Ratio"]
    #[inline] pub fn divbus(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if DIVBUS != 0"]
    #[inline] pub fn test_divbus(&self) -> bool {
        self.divbus() != 0
    }

    #[doc="Sets the DIVBUS field."]
    #[inline] pub fn set_divbus<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Core Clock Divide Ratio"]
    #[inline] pub fn divcore(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if DIVCORE != 0"]
    #[inline] pub fn test_divcore(&self) -> bool {
        self.divcore() != 0
    }

    #[doc="Sets the DIVCORE field."]
    #[inline] pub fn set_divcore<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="System Clock Source"]
    #[inline] pub fn scs(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if SCS != 0"]
    #[inline] pub fn test_scs(&self) -> bool {
        self.scs() != 0
    }

    #[doc="Sets the SCS field."]
    #[inline] pub fn set_scs<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Rccr {
    #[inline]
    fn from(other: u32) -> Self {
         Rccr(other)
    }
}

impl ::core::fmt::Display for Rccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.divslow() != 0 { try!(write!(f, " divslow=0x{:x}", self.divslow()))}
        if self.divbus() != 0 { try!(write!(f, " divbus=0x{:x}", self.divbus()))}
        if self.divcore() != 0 { try!(write!(f, " divcore=0x{:x}", self.divcore()))}
        if self.scs() != 0 { try!(write!(f, " scs=0x{:x}", self.scs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="VLPR Clock Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Vccr(pub u32);
impl Vccr {
    #[doc="Slow Clock Divide Ratio"]
    #[inline] pub fn divslow(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if DIVSLOW != 0"]
    #[inline] pub fn test_divslow(&self) -> bool {
        self.divslow() != 0
    }

    #[doc="Sets the DIVSLOW field."]
    #[inline] pub fn set_divslow<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Bus Clock Divide Ratio"]
    #[inline] pub fn divbus(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if DIVBUS != 0"]
    #[inline] pub fn test_divbus(&self) -> bool {
        self.divbus() != 0
    }

    #[doc="Sets the DIVBUS field."]
    #[inline] pub fn set_divbus<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Core Clock Divide Ratio"]
    #[inline] pub fn divcore(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if DIVCORE != 0"]
    #[inline] pub fn test_divcore(&self) -> bool {
        self.divcore() != 0
    }

    #[doc="Sets the DIVCORE field."]
    #[inline] pub fn set_divcore<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="System Clock Source"]
    #[inline] pub fn scs(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if SCS != 0"]
    #[inline] pub fn test_scs(&self) -> bool {
        self.scs() != 0
    }

    #[doc="Sets the SCS field."]
    #[inline] pub fn set_scs<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Vccr {
    #[inline]
    fn from(other: u32) -> Self {
         Vccr(other)
    }
}

impl ::core::fmt::Display for Vccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Vccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.divslow() != 0 { try!(write!(f, " divslow=0x{:x}", self.divslow()))}
        if self.divbus() != 0 { try!(write!(f, " divbus=0x{:x}", self.divbus()))}
        if self.divcore() != 0 { try!(write!(f, " divcore=0x{:x}", self.divcore()))}
        if self.scs() != 0 { try!(write!(f, " scs=0x{:x}", self.scs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HSRUN Clock Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hccr(pub u32);
impl Hccr {
    #[doc="Slow Clock Divide Ratio"]
    #[inline] pub fn divslow(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if DIVSLOW != 0"]
    #[inline] pub fn test_divslow(&self) -> bool {
        self.divslow() != 0
    }

    #[doc="Sets the DIVSLOW field."]
    #[inline] pub fn set_divslow<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Bus Clock Divide Ratio"]
    #[inline] pub fn divbus(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if DIVBUS != 0"]
    #[inline] pub fn test_divbus(&self) -> bool {
        self.divbus() != 0
    }

    #[doc="Sets the DIVBUS field."]
    #[inline] pub fn set_divbus<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Core Clock Divide Ratio"]
    #[inline] pub fn divcore(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if DIVCORE != 0"]
    #[inline] pub fn test_divcore(&self) -> bool {
        self.divcore() != 0
    }

    #[doc="Sets the DIVCORE field."]
    #[inline] pub fn set_divcore<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="System Clock Source"]
    #[inline] pub fn scs(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if SCS != 0"]
    #[inline] pub fn test_scs(&self) -> bool {
        self.scs() != 0
    }

    #[doc="Sets the SCS field."]
    #[inline] pub fn set_scs<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Hccr {
    #[inline]
    fn from(other: u32) -> Self {
         Hccr(other)
    }
}

impl ::core::fmt::Display for Hccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.divslow() != 0 { try!(write!(f, " divslow=0x{:x}", self.divslow()))}
        if self.divbus() != 0 { try!(write!(f, " divbus=0x{:x}", self.divbus()))}
        if self.divcore() != 0 { try!(write!(f, " divcore=0x{:x}", self.divcore()))}
        if self.scs() != 0 { try!(write!(f, " scs=0x{:x}", self.scs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SCG CLKOUT Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clkoutcnfg(pub u32);
impl Clkoutcnfg {
    #[doc="SCG Clkout Select"]
    #[inline] pub fn clkoutsel(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if CLKOUTSEL != 0"]
    #[inline] pub fn test_clkoutsel(&self) -> bool {
        self.clkoutsel() != 0
    }

    #[doc="Sets the CLKOUTSEL field."]
    #[inline] pub fn set_clkoutsel<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Clkoutcnfg {
    #[inline]
    fn from(other: u32) -> Self {
         Clkoutcnfg(other)
    }
}

impl ::core::fmt::Display for Clkoutcnfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clkoutcnfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clkoutsel() != 0 { try!(write!(f, " clkoutsel=0x{:x}", self.clkoutsel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System OSC Control Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sosccsr(pub u32);
impl Sosccsr {
    #[doc="System OSC Enable"]
    #[inline] pub fn soscen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SOSCEN != 0"]
    #[inline] pub fn test_soscen(&self) -> bool {
        self.soscen() != 0
    }

    #[doc="Sets the SOSCEN field."]
    #[inline] pub fn set_soscen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="System OSC Clock Monitor"]
    #[inline] pub fn sosccm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SOSCCM != 0"]
    #[inline] pub fn test_sosccm(&self) -> bool {
        self.sosccm() != 0
    }

    #[doc="Sets the SOSCCM field."]
    #[inline] pub fn set_sosccm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="System OSC Clock Monitor Reset Enable"]
    #[inline] pub fn sosccmre(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if SOSCCMRE != 0"]
    #[inline] pub fn test_sosccmre(&self) -> bool {
        self.sosccmre() != 0
    }

    #[doc="Sets the SOSCCMRE field."]
    #[inline] pub fn set_sosccmre<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Lock Register"]
    #[inline] pub fn lk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if LK != 0"]
    #[inline] pub fn test_lk(&self) -> bool {
        self.lk() != 0
    }

    #[doc="Sets the LK field."]
    #[inline] pub fn set_lk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="System OSC Valid"]
    #[inline] pub fn soscvld(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if SOSCVLD != 0"]
    #[inline] pub fn test_soscvld(&self) -> bool {
        self.soscvld() != 0
    }

    #[doc="Sets the SOSCVLD field."]
    #[inline] pub fn set_soscvld<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="System OSC Selected"]
    #[inline] pub fn soscsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if SOSCSEL != 0"]
    #[inline] pub fn test_soscsel(&self) -> bool {
        self.soscsel() != 0
    }

    #[doc="Sets the SOSCSEL field."]
    #[inline] pub fn set_soscsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="System OSC Clock Error"]
    #[inline] pub fn soscerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if SOSCERR != 0"]
    #[inline] pub fn test_soscerr(&self) -> bool {
        self.soscerr() != 0
    }

    #[doc="Sets the SOSCERR field."]
    #[inline] pub fn set_soscerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

}

impl From<u32> for Sosccsr {
    #[inline]
    fn from(other: u32) -> Self {
         Sosccsr(other)
    }
}

impl ::core::fmt::Display for Sosccsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sosccsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.soscen() != 0 { try!(write!(f, " soscen"))}
        if self.sosccm() != 0 { try!(write!(f, " sosccm"))}
        if self.sosccmre() != 0 { try!(write!(f, " sosccmre"))}
        if self.lk() != 0 { try!(write!(f, " lk"))}
        if self.soscvld() != 0 { try!(write!(f, " soscvld"))}
        if self.soscsel() != 0 { try!(write!(f, " soscsel"))}
        if self.soscerr() != 0 { try!(write!(f, " soscerr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System OSC Divide Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Soscdiv(pub u32);
impl Soscdiv {
    #[doc="System OSC Clock Divide 1"]
    #[inline] pub fn soscdiv1(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if SOSCDIV1 != 0"]
    #[inline] pub fn test_soscdiv1(&self) -> bool {
        self.soscdiv1() != 0
    }

    #[doc="Sets the SOSCDIV1 field."]
    #[inline] pub fn set_soscdiv1<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="System OSC Clock Divide 2"]
    #[inline] pub fn soscdiv2(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if SOSCDIV2 != 0"]
    #[inline] pub fn test_soscdiv2(&self) -> bool {
        self.soscdiv2() != 0
    }

    #[doc="Sets the SOSCDIV2 field."]
    #[inline] pub fn set_soscdiv2<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Soscdiv {
    #[inline]
    fn from(other: u32) -> Self {
         Soscdiv(other)
    }
}

impl ::core::fmt::Display for Soscdiv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Soscdiv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.soscdiv1() != 0 { try!(write!(f, " soscdiv1=0x{:x}", self.soscdiv1()))}
        if self.soscdiv2() != 0 { try!(write!(f, " soscdiv2=0x{:x}", self.soscdiv2()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Oscillator Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sosccfg(pub u32);
impl Sosccfg {
    #[doc="External Reference Select"]
    #[inline] pub fn erefs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if EREFS != 0"]
    #[inline] pub fn test_erefs(&self) -> bool {
        self.erefs() != 0
    }

    #[doc="Sets the EREFS field."]
    #[inline] pub fn set_erefs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="High Gain Oscillator Select"]
    #[inline] pub fn hgo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if HGO != 0"]
    #[inline] pub fn test_hgo(&self) -> bool {
        self.hgo() != 0
    }

    #[doc="Sets the HGO field."]
    #[inline] pub fn set_hgo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="System OSC Range Select"]
    #[inline] pub fn range(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if RANGE != 0"]
    #[inline] pub fn test_range(&self) -> bool {
        self.range() != 0
    }

    #[doc="Sets the RANGE field."]
    #[inline] pub fn set_range<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Sosccfg {
    #[inline]
    fn from(other: u32) -> Self {
         Sosccfg(other)
    }
}

impl ::core::fmt::Display for Sosccfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sosccfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.erefs() != 0 { try!(write!(f, " erefs"))}
        if self.hgo() != 0 { try!(write!(f, " hgo"))}
        if self.range() != 0 { try!(write!(f, " range=0x{:x}", self.range()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Slow IRC Control Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sirccsr(pub u32);
impl Sirccsr {
    #[doc="Slow IRC Enable"]
    #[inline] pub fn sircen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SIRCEN != 0"]
    #[inline] pub fn test_sircen(&self) -> bool {
        self.sircen() != 0
    }

    #[doc="Sets the SIRCEN field."]
    #[inline] pub fn set_sircen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Slow IRC Stop Enable"]
    #[inline] pub fn sircsten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SIRCSTEN != 0"]
    #[inline] pub fn test_sircsten(&self) -> bool {
        self.sircsten() != 0
    }

    #[doc="Sets the SIRCSTEN field."]
    #[inline] pub fn set_sircsten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Slow IRC Low Power Enable"]
    #[inline] pub fn sirclpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SIRCLPEN != 0"]
    #[inline] pub fn test_sirclpen(&self) -> bool {
        self.sirclpen() != 0
    }

    #[doc="Sets the SIRCLPEN field."]
    #[inline] pub fn set_sirclpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Lock Register"]
    #[inline] pub fn lk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if LK != 0"]
    #[inline] pub fn test_lk(&self) -> bool {
        self.lk() != 0
    }

    #[doc="Sets the LK field."]
    #[inline] pub fn set_lk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Slow IRC Valid"]
    #[inline] pub fn sircvld(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if SIRCVLD != 0"]
    #[inline] pub fn test_sircvld(&self) -> bool {
        self.sircvld() != 0
    }

    #[doc="Sets the SIRCVLD field."]
    #[inline] pub fn set_sircvld<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Slow IRC Selected"]
    #[inline] pub fn sircsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if SIRCSEL != 0"]
    #[inline] pub fn test_sircsel(&self) -> bool {
        self.sircsel() != 0
    }

    #[doc="Sets the SIRCSEL field."]
    #[inline] pub fn set_sircsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

}

impl From<u32> for Sirccsr {
    #[inline]
    fn from(other: u32) -> Self {
         Sirccsr(other)
    }
}

impl ::core::fmt::Display for Sirccsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sirccsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sircen() != 0 { try!(write!(f, " sircen"))}
        if self.sircsten() != 0 { try!(write!(f, " sircsten"))}
        if self.sirclpen() != 0 { try!(write!(f, " sirclpen"))}
        if self.lk() != 0 { try!(write!(f, " lk"))}
        if self.sircvld() != 0 { try!(write!(f, " sircvld"))}
        if self.sircsel() != 0 { try!(write!(f, " sircsel"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Slow IRC Divide Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sircdiv(pub u32);
impl Sircdiv {
    #[doc="Slow IRC Clock Divide 1"]
    #[inline] pub fn sircdiv1(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if SIRCDIV1 != 0"]
    #[inline] pub fn test_sircdiv1(&self) -> bool {
        self.sircdiv1() != 0
    }

    #[doc="Sets the SIRCDIV1 field."]
    #[inline] pub fn set_sircdiv1<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Slow IRC Clock Divide 2"]
    #[inline] pub fn sircdiv2(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if SIRCDIV2 != 0"]
    #[inline] pub fn test_sircdiv2(&self) -> bool {
        self.sircdiv2() != 0
    }

    #[doc="Sets the SIRCDIV2 field."]
    #[inline] pub fn set_sircdiv2<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Sircdiv {
    #[inline]
    fn from(other: u32) -> Self {
         Sircdiv(other)
    }
}

impl ::core::fmt::Display for Sircdiv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sircdiv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sircdiv1() != 0 { try!(write!(f, " sircdiv1=0x{:x}", self.sircdiv1()))}
        if self.sircdiv2() != 0 { try!(write!(f, " sircdiv2=0x{:x}", self.sircdiv2()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Slow IRC Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sirccfg(pub u32);
impl Sirccfg {
    #[doc="Frequency Range"]
    #[inline] pub fn range(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RANGE != 0"]
    #[inline] pub fn test_range(&self) -> bool {
        self.range() != 0
    }

    #[doc="Sets the RANGE field."]
    #[inline] pub fn set_range<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sirccfg {
    #[inline]
    fn from(other: u32) -> Self {
         Sirccfg(other)
    }
}

impl ::core::fmt::Display for Sirccfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sirccfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.range() != 0 { try!(write!(f, " range"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Fast IRC Control Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Firccsr(pub u32);
impl Firccsr {
    #[doc="Fast IRC Enable"]
    #[inline] pub fn fircen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FIRCEN != 0"]
    #[inline] pub fn test_fircen(&self) -> bool {
        self.fircen() != 0
    }

    #[doc="Sets the FIRCEN field."]
    #[inline] pub fn set_fircen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Fast IRC Regulator Enable"]
    #[inline] pub fn fircregoff(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FIRCREGOFF != 0"]
    #[inline] pub fn test_fircregoff(&self) -> bool {
        self.fircregoff() != 0
    }

    #[doc="Sets the FIRCREGOFF field."]
    #[inline] pub fn set_fircregoff<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Lock Register"]
    #[inline] pub fn lk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if LK != 0"]
    #[inline] pub fn test_lk(&self) -> bool {
        self.lk() != 0
    }

    #[doc="Sets the LK field."]
    #[inline] pub fn set_lk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Fast IRC Valid status"]
    #[inline] pub fn fircvld(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if FIRCVLD != 0"]
    #[inline] pub fn test_fircvld(&self) -> bool {
        self.fircvld() != 0
    }

    #[doc="Sets the FIRCVLD field."]
    #[inline] pub fn set_fircvld<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Fast IRC Selected status"]
    #[inline] pub fn fircsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if FIRCSEL != 0"]
    #[inline] pub fn test_fircsel(&self) -> bool {
        self.fircsel() != 0
    }

    #[doc="Sets the FIRCSEL field."]
    #[inline] pub fn set_fircsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Fast IRC Clock Error"]
    #[inline] pub fn fircerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if FIRCERR != 0"]
    #[inline] pub fn test_fircerr(&self) -> bool {
        self.fircerr() != 0
    }

    #[doc="Sets the FIRCERR field."]
    #[inline] pub fn set_fircerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

}

impl From<u32> for Firccsr {
    #[inline]
    fn from(other: u32) -> Self {
         Firccsr(other)
    }
}

impl ::core::fmt::Display for Firccsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Firccsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fircen() != 0 { try!(write!(f, " fircen"))}
        if self.fircregoff() != 0 { try!(write!(f, " fircregoff"))}
        if self.lk() != 0 { try!(write!(f, " lk"))}
        if self.fircvld() != 0 { try!(write!(f, " fircvld"))}
        if self.fircsel() != 0 { try!(write!(f, " fircsel"))}
        if self.fircerr() != 0 { try!(write!(f, " fircerr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Fast IRC Divide Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fircdiv(pub u32);
impl Fircdiv {
    #[doc="Fast IRC Clock Divide 1"]
    #[inline] pub fn fircdiv1(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if FIRCDIV1 != 0"]
    #[inline] pub fn test_fircdiv1(&self) -> bool {
        self.fircdiv1() != 0
    }

    #[doc="Sets the FIRCDIV1 field."]
    #[inline] pub fn set_fircdiv1<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Fast IRC Clock Divide 2"]
    #[inline] pub fn fircdiv2(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if FIRCDIV2 != 0"]
    #[inline] pub fn test_fircdiv2(&self) -> bool {
        self.fircdiv2() != 0
    }

    #[doc="Sets the FIRCDIV2 field."]
    #[inline] pub fn set_fircdiv2<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Fircdiv {
    #[inline]
    fn from(other: u32) -> Self {
         Fircdiv(other)
    }
}

impl ::core::fmt::Display for Fircdiv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fircdiv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fircdiv1() != 0 { try!(write!(f, " fircdiv1=0x{:x}", self.fircdiv1()))}
        if self.fircdiv2() != 0 { try!(write!(f, " fircdiv2=0x{:x}", self.fircdiv2()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Fast IRC Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Firccfg(pub u32);
impl Firccfg {
    #[doc="Frequency Range"]
    #[inline] pub fn range(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if RANGE != 0"]
    #[inline] pub fn test_range(&self) -> bool {
        self.range() != 0
    }

    #[doc="Sets the RANGE field."]
    #[inline] pub fn set_range<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Firccfg {
    #[inline]
    fn from(other: u32) -> Self {
         Firccfg(other)
    }
}

impl ::core::fmt::Display for Firccfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Firccfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.range() != 0 { try!(write!(f, " range=0x{:x}", self.range()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System PLL Control Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Spllcsr(pub u32);
impl Spllcsr {
    #[doc="System PLL Enable"]
    #[inline] pub fn spllen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SPLLEN != 0"]
    #[inline] pub fn test_spllen(&self) -> bool {
        self.spllen() != 0
    }

    #[doc="Sets the SPLLEN field."]
    #[inline] pub fn set_spllen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="System PLL Clock Monitor"]
    #[inline] pub fn spllcm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SPLLCM != 0"]
    #[inline] pub fn test_spllcm(&self) -> bool {
        self.spllcm() != 0
    }

    #[doc="Sets the SPLLCM field."]
    #[inline] pub fn set_spllcm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="System PLL Clock Monitor Reset Enable"]
    #[inline] pub fn spllcmre(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if SPLLCMRE != 0"]
    #[inline] pub fn test_spllcmre(&self) -> bool {
        self.spllcmre() != 0
    }

    #[doc="Sets the SPLLCMRE field."]
    #[inline] pub fn set_spllcmre<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Lock Register"]
    #[inline] pub fn lk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if LK != 0"]
    #[inline] pub fn test_lk(&self) -> bool {
        self.lk() != 0
    }

    #[doc="Sets the LK field."]
    #[inline] pub fn set_lk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="System PLL Valid"]
    #[inline] pub fn spllvld(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if SPLLVLD != 0"]
    #[inline] pub fn test_spllvld(&self) -> bool {
        self.spllvld() != 0
    }

    #[doc="Sets the SPLLVLD field."]
    #[inline] pub fn set_spllvld<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="System PLL Selected"]
    #[inline] pub fn spllsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if SPLLSEL != 0"]
    #[inline] pub fn test_spllsel(&self) -> bool {
        self.spllsel() != 0
    }

    #[doc="Sets the SPLLSEL field."]
    #[inline] pub fn set_spllsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="System PLL Clock Error"]
    #[inline] pub fn spllerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if SPLLERR != 0"]
    #[inline] pub fn test_spllerr(&self) -> bool {
        self.spllerr() != 0
    }

    #[doc="Sets the SPLLERR field."]
    #[inline] pub fn set_spllerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

}

impl From<u32> for Spllcsr {
    #[inline]
    fn from(other: u32) -> Self {
         Spllcsr(other)
    }
}

impl ::core::fmt::Display for Spllcsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Spllcsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.spllen() != 0 { try!(write!(f, " spllen"))}
        if self.spllcm() != 0 { try!(write!(f, " spllcm"))}
        if self.spllcmre() != 0 { try!(write!(f, " spllcmre"))}
        if self.lk() != 0 { try!(write!(f, " lk"))}
        if self.spllvld() != 0 { try!(write!(f, " spllvld"))}
        if self.spllsel() != 0 { try!(write!(f, " spllsel"))}
        if self.spllerr() != 0 { try!(write!(f, " spllerr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System PLL Divide Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Splldiv(pub u32);
impl Splldiv {
    #[doc="System PLL Clock Divide 1"]
    #[inline] pub fn splldiv1(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if SPLLDIV1 != 0"]
    #[inline] pub fn test_splldiv1(&self) -> bool {
        self.splldiv1() != 0
    }

    #[doc="Sets the SPLLDIV1 field."]
    #[inline] pub fn set_splldiv1<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="System PLL Clock Divide 2"]
    #[inline] pub fn splldiv2(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if SPLLDIV2 != 0"]
    #[inline] pub fn test_splldiv2(&self) -> bool {
        self.splldiv2() != 0
    }

    #[doc="Sets the SPLLDIV2 field."]
    #[inline] pub fn set_splldiv2<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Splldiv {
    #[inline]
    fn from(other: u32) -> Self {
         Splldiv(other)
    }
}

impl ::core::fmt::Display for Splldiv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Splldiv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.splldiv1() != 0 { try!(write!(f, " splldiv1=0x{:x}", self.splldiv1()))}
        if self.splldiv2() != 0 { try!(write!(f, " splldiv2=0x{:x}", self.splldiv2()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System PLL Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Spllcfg(pub u32);
impl Spllcfg {
    #[doc="PLL Reference Clock Divider"]
    #[inline] pub fn prediv(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if PREDIV != 0"]
    #[inline] pub fn test_prediv(&self) -> bool {
        self.prediv() != 0
    }

    #[doc="Sets the PREDIV field."]
    #[inline] pub fn set_prediv<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="System PLL Multiplier"]
    #[inline] pub fn mult(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1f) as u8) } // [20:16]
    }

    #[doc="Returns true if MULT != 0"]
    #[inline] pub fn test_mult(&self) -> bool {
        self.mult() != 0
    }

    #[doc="Sets the MULT field."]
    #[inline] pub fn set_mult<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Spllcfg {
    #[inline]
    fn from(other: u32) -> Self {
         Spllcfg(other)
    }
}

impl ::core::fmt::Display for Spllcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Spllcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.prediv() != 0 { try!(write!(f, " prediv=0x{:x}", self.prediv()))}
        if self.mult() != 0 { try!(write!(f, " mult=0x{:x}", self.mult()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

