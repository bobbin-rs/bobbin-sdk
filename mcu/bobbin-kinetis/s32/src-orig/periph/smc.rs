//! System Mode Controller

#[allow(unused_imports)] use ::bobbin_common::*;

#[doc="System Mode Controller"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SmcPeriph(pub usize);
impl SmcPeriph {
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

    #[doc="Get the *mut pointer for the PMPROT register."]
    #[inline] pub fn pmprot_mut(&self) -> *mut Pmprot { 
        (self.0 + 0x8) as *mut Pmprot
    }

    #[doc="Get the *const pointer for the PMPROT register."]
    #[inline] pub fn pmprot_ptr(&self) -> *const Pmprot { 
           self.pmprot_mut()
    }

    #[doc="Read the PMPROT register."]
    #[inline] pub fn pmprot(&self) -> Pmprot { 
        unsafe {
            read_volatile(self.pmprot_ptr())
        }
    }

    #[doc="Write the PMPROT register."]
    #[inline] pub fn set_pmprot<F: FnOnce(Pmprot) -> Pmprot>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pmprot_mut(), f(Pmprot(0)));
        }
        self
    }

    #[doc="Modify the PMPROT register."]
    #[inline] pub fn with_pmprot<F: FnOnce(Pmprot) -> Pmprot>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pmprot_mut(), f(self.pmprot()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PMCTRL register."]
    #[inline] pub fn pmctrl_mut(&self) -> *mut Pmctrl { 
        (self.0 + 0xc) as *mut Pmctrl
    }

    #[doc="Get the *const pointer for the PMCTRL register."]
    #[inline] pub fn pmctrl_ptr(&self) -> *const Pmctrl { 
           self.pmctrl_mut()
    }

    #[doc="Read the PMCTRL register."]
    #[inline] pub fn pmctrl(&self) -> Pmctrl { 
        unsafe {
            read_volatile(self.pmctrl_ptr())
        }
    }

    #[doc="Write the PMCTRL register."]
    #[inline] pub fn set_pmctrl<F: FnOnce(Pmctrl) -> Pmctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pmctrl_mut(), f(Pmctrl(0)));
        }
        self
    }

    #[doc="Modify the PMCTRL register."]
    #[inline] pub fn with_pmctrl<F: FnOnce(Pmctrl) -> Pmctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pmctrl_mut(), f(self.pmctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the STOPCTRL register."]
    #[inline] pub fn stopctrl_mut(&self) -> *mut Stopctrl { 
        (self.0 + 0x10) as *mut Stopctrl
    }

    #[doc="Get the *const pointer for the STOPCTRL register."]
    #[inline] pub fn stopctrl_ptr(&self) -> *const Stopctrl { 
           self.stopctrl_mut()
    }

    #[doc="Read the STOPCTRL register."]
    #[inline] pub fn stopctrl(&self) -> Stopctrl { 
        unsafe {
            read_volatile(self.stopctrl_ptr())
        }
    }

    #[doc="Write the STOPCTRL register."]
    #[inline] pub fn set_stopctrl<F: FnOnce(Stopctrl) -> Stopctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stopctrl_mut(), f(Stopctrl(0)));
        }
        self
    }

    #[doc="Modify the STOPCTRL register."]
    #[inline] pub fn with_stopctrl<F: FnOnce(Stopctrl) -> Stopctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stopctrl_mut(), f(self.stopctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PMSTAT register."]
    #[inline] pub fn pmstat_mut(&self) -> *mut Pmstat { 
        (self.0 + 0x14) as *mut Pmstat
    }

    #[doc="Get the *const pointer for the PMSTAT register."]
    #[inline] pub fn pmstat_ptr(&self) -> *const Pmstat { 
           self.pmstat_mut()
    }

    #[doc="Read the PMSTAT register."]
    #[inline] pub fn pmstat(&self) -> Pmstat { 
        unsafe {
            read_volatile(self.pmstat_ptr())
        }
    }

}

#[doc="SMC Version ID Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc="Feature Specification Number"]
    #[inline] pub fn feature(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if FEATURE != 0"]
    #[inline] pub fn test_feature(&self) -> bool {
        self.feature() != 0
    }

    #[doc="Sets the FEATURE field."]
    #[inline] pub fn set_feature<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Minor Version Number"]
    #[inline] pub fn minor(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if MINOR != 0"]
    #[inline] pub fn test_minor(&self) -> bool {
        self.minor() != 0
    }

    #[doc="Sets the MINOR field."]
    #[inline] pub fn set_minor<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Major Version Number"]
    #[inline] pub fn major(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if MAJOR != 0"]
    #[inline] pub fn test_major(&self) -> bool {
        self.major() != 0
    }

    #[doc="Sets the MAJOR field."]
    #[inline] pub fn set_major<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
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
        if self.feature() != 0 { try!(write!(f, " feature=0x{:x}", self.feature()))}
        if self.minor() != 0 { try!(write!(f, " minor=0x{:x}", self.minor()))}
        if self.major() != 0 { try!(write!(f, " major=0x{:x}", self.major()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SMC Parameter Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Param(pub u32);
impl Param {
    #[doc="Existence of HSRUN feature"]
    #[inline] pub fn ehsrun(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EHSRUN != 0"]
    #[inline] pub fn test_ehsrun(&self) -> bool {
        self.ehsrun() != 0
    }

    #[doc="Sets the EHSRUN field."]
    #[inline] pub fn set_ehsrun<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Existence of LLS feature"]
    #[inline] pub fn ells(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ELLS != 0"]
    #[inline] pub fn test_ells(&self) -> bool {
        self.ells() != 0
    }

    #[doc="Sets the ELLS field."]
    #[inline] pub fn set_ells<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Existence of LLS2 feature"]
    #[inline] pub fn ells2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ELLS2 != 0"]
    #[inline] pub fn test_ells2(&self) -> bool {
        self.ells2() != 0
    }

    #[doc="Sets the ELLS2 field."]
    #[inline] pub fn set_ells2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Existence of VLLS0 feature"]
    #[inline] pub fn evlls0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if EVLLS0 != 0"]
    #[inline] pub fn test_evlls0(&self) -> bool {
        self.evlls0() != 0
    }

    #[doc="Sets the EVLLS0 field."]
    #[inline] pub fn set_evlls0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
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
        if self.ehsrun() != 0 { try!(write!(f, " ehsrun"))}
        if self.ells() != 0 { try!(write!(f, " ells"))}
        if self.ells2() != 0 { try!(write!(f, " ells2"))}
        if self.evlls0() != 0 { try!(write!(f, " evlls0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power Mode Protection register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pmprot(pub u32);
impl Pmprot {
    #[doc="Allow Very-Low-Power Modes"]
    #[inline] pub fn avlp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if AVLP != 0"]
    #[inline] pub fn test_avlp(&self) -> bool {
        self.avlp() != 0
    }

    #[doc="Sets the AVLP field."]
    #[inline] pub fn set_avlp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Allow High Speed Run mode"]
    #[inline] pub fn ahsrun(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if AHSRUN != 0"]
    #[inline] pub fn test_ahsrun(&self) -> bool {
        self.ahsrun() != 0
    }

    #[doc="Sets the AHSRUN field."]
    #[inline] pub fn set_ahsrun<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Pmprot {
    #[inline]
    fn from(other: u32) -> Self {
         Pmprot(other)
    }
}

impl ::core::fmt::Display for Pmprot {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pmprot {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.avlp() != 0 { try!(write!(f, " avlp"))}
        if self.ahsrun() != 0 { try!(write!(f, " ahsrun"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power Mode Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pmctrl(pub u32);
impl Pmctrl {
    #[doc="Stop Mode Control"]
    #[inline] pub fn stopm(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if STOPM != 0"]
    #[inline] pub fn test_stopm(&self) -> bool {
        self.stopm() != 0
    }

    #[doc="Sets the STOPM field."]
    #[inline] pub fn set_stopm<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Very Low Power Stop Aborted"]
    #[inline] pub fn vlpsa(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if VLPSA != 0"]
    #[inline] pub fn test_vlpsa(&self) -> bool {
        self.vlpsa() != 0
    }

    #[doc="Sets the VLPSA field."]
    #[inline] pub fn set_vlpsa<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Run Mode Control"]
    #[inline] pub fn runm(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if RUNM != 0"]
    #[inline] pub fn test_runm(&self) -> bool {
        self.runm() != 0
    }

    #[doc="Sets the RUNM field."]
    #[inline] pub fn set_runm<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u32> for Pmctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Pmctrl(other)
    }
}

impl ::core::fmt::Display for Pmctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pmctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.stopm() != 0 { try!(write!(f, " stopm=0x{:x}", self.stopm()))}
        if self.vlpsa() != 0 { try!(write!(f, " vlpsa"))}
        if self.runm() != 0 { try!(write!(f, " runm=0x{:x}", self.runm()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Stop Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stopctrl(pub u32);
impl Stopctrl {
    #[doc="Stop Option"]
    #[inline] pub fn stopo(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if STOPO != 0"]
    #[inline] pub fn test_stopo(&self) -> bool {
        self.stopo() != 0
    }

    #[doc="Sets the STOPO field."]
    #[inline] pub fn set_stopo<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for Stopctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Stopctrl(other)
    }
}

impl ::core::fmt::Display for Stopctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Stopctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.stopo() != 0 { try!(write!(f, " stopo=0x{:x}", self.stopo()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power Mode Status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pmstat(pub u32);
impl Pmstat {
    #[doc="Power Mode Status"]
    #[inline] pub fn pmstat(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PMSTAT != 0"]
    #[inline] pub fn test_pmstat(&self) -> bool {
        self.pmstat() != 0
    }

    #[doc="Sets the PMSTAT field."]
    #[inline] pub fn set_pmstat<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pmstat {
    #[inline]
    fn from(other: u32) -> Self {
         Pmstat(other)
    }
}

impl ::core::fmt::Display for Pmstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pmstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pmstat() != 0 { try!(write!(f, " pmstat=0x{:x}", self.pmstat()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

