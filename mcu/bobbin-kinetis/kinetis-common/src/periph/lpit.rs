
#[allow(unused_imports)] use bobbin_common::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="LPIT Peripheral"]
pub struct LpitPeriph(pub usize); 

pub struct LpitCh { pub periph: LpitPeriph, pub index: usize }

impl LpitPeriph {
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

    #[doc="Get the *mut pointer for the MCR register."]
    #[inline] pub fn mcr_mut(&self) -> *mut Mcr { 
        (self.0 + 0x8) as *mut Mcr
    }

    #[doc="Get the *const pointer for the MCR register."]
    #[inline] pub fn mcr_ptr(&self) -> *const Mcr { 
           self.mcr_mut()
    }

    #[doc="Read the MCR register."]
    #[inline] pub fn mcr(&self) -> Mcr { 
        unsafe {
            read_volatile(self.mcr_ptr())
        }
    }

    #[doc="Write the MCR register."]
    #[inline] pub fn set_mcr<F: FnOnce(Mcr) -> Mcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mcr_mut(), f(Mcr(0)));
        }
        self
    }

    #[doc="Modify the MCR register."]
    #[inline] pub fn with_mcr<F: FnOnce(Mcr) -> Mcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mcr_mut(), f(self.mcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MSR register."]
    #[inline] pub fn msr_mut(&self) -> *mut Msr { 
        (self.0 + 0xc) as *mut Msr
    }

    #[doc="Get the *const pointer for the MSR register."]
    #[inline] pub fn msr_ptr(&self) -> *const Msr { 
           self.msr_mut()
    }

    #[doc="Read the MSR register."]
    #[inline] pub fn msr(&self) -> Msr { 
        unsafe {
            read_volatile(self.msr_ptr())
        }
    }

    #[doc="Write the MSR register."]
    #[inline] pub fn set_msr<F: FnOnce(Msr) -> Msr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.msr_mut(), f(Msr(0)));
        }
        self
    }

    #[doc="Modify the MSR register."]
    #[inline] pub fn with_msr<F: FnOnce(Msr) -> Msr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.msr_mut(), f(self.msr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MIER register."]
    #[inline] pub fn mier_mut(&self) -> *mut Mier { 
        (self.0 + 0x10) as *mut Mier
    }

    #[doc="Get the *const pointer for the MIER register."]
    #[inline] pub fn mier_ptr(&self) -> *const Mier { 
           self.mier_mut()
    }

    #[doc="Read the MIER register."]
    #[inline] pub fn mier(&self) -> Mier { 
        unsafe {
            read_volatile(self.mier_ptr())
        }
    }

    #[doc="Write the MIER register."]
    #[inline] pub fn set_mier<F: FnOnce(Mier) -> Mier>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mier_mut(), f(Mier(0)));
        }
        self
    }

    #[doc="Modify the MIER register."]
    #[inline] pub fn with_mier<F: FnOnce(Mier) -> Mier>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mier_mut(), f(self.mier()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SETTEN register."]
    #[inline] pub fn setten_mut(&self) -> *mut Setten { 
        (self.0 + 0x14) as *mut Setten
    }

    #[doc="Get the *const pointer for the SETTEN register."]
    #[inline] pub fn setten_ptr(&self) -> *const Setten { 
           self.setten_mut()
    }

    #[doc="Read the SETTEN register."]
    #[inline] pub fn setten(&self) -> Setten { 
        unsafe {
            read_volatile(self.setten_ptr())
        }
    }

    #[doc="Write the SETTEN register."]
    #[inline] pub fn set_setten<F: FnOnce(Setten) -> Setten>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.setten_mut(), f(Setten(0)));
        }
        self
    }

    #[doc="Modify the SETTEN register."]
    #[inline] pub fn with_setten<F: FnOnce(Setten) -> Setten>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.setten_mut(), f(self.setten()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLRTEN register."]
    #[inline] pub fn clrten_mut(&self) -> *mut Clrten { 
        (self.0 + 0x18) as *mut Clrten
    }

    #[doc="Get the *const pointer for the CLRTEN register."]
    #[inline] pub fn clrten_ptr(&self) -> *const Clrten { 
           self.clrten_mut()
    }

    #[doc="Read the CLRTEN register."]
    #[inline] pub fn clrten(&self) -> Clrten { 
        unsafe {
            read_volatile(self.clrten_ptr())
        }
    }

    #[doc="Write the CLRTEN register."]
    #[inline] pub fn set_clrten<F: FnOnce(Clrten) -> Clrten>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clrten_mut(), f(Clrten(0)));
        }
        self
    }

    #[doc="Modify the CLRTEN register."]
    #[inline] pub fn with_clrten<F: FnOnce(Clrten) -> Clrten>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clrten_mut(), f(self.clrten()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TVAL register."]
    #[inline] pub fn tval_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Tval { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x20 + (index << 4)) as *mut Tval
    }

    #[doc="Get the *const pointer for the TVAL register."]
    #[inline] pub fn tval_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Tval { 
           self.tval_mut(index)
    }

    #[doc="Read the TVAL register."]
    #[inline] pub fn tval<I: Into<bits::R4>>(&self, index: I) -> Tval { 
        unsafe {
            read_volatile(self.tval_ptr(index))
        }
    }

    #[doc="Write the TVAL register."]
    #[inline] pub fn set_tval<I: Into<bits::R4>, F: FnOnce(Tval) -> Tval>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tval_mut(index), f(Tval(0)));
        }
        self
    }

    #[doc="Modify the TVAL register."]
    #[inline] pub fn with_tval<I: Into<bits::R4> + Copy, F: FnOnce(Tval) -> Tval>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tval_mut(index), f(self.tval(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the CVAL register."]
    #[inline] pub fn cval_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Cval { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x24 + (index << 4)) as *mut Cval
    }

    #[doc="Get the *const pointer for the CVAL register."]
    #[inline] pub fn cval_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Cval { 
           self.cval_mut(index)
    }

    #[doc="Read the CVAL register."]
    #[inline] pub fn cval<I: Into<bits::R4>>(&self, index: I) -> Cval { 
        unsafe {
            read_volatile(self.cval_ptr(index))
        }
    }

    #[doc="Get the *mut pointer for the TCTRL register."]
    #[inline] pub fn tctrl_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Tctrl { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x28 + (index << 4)) as *mut Tctrl
    }

    #[doc="Get the *const pointer for the TCTRL register."]
    #[inline] pub fn tctrl_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Tctrl { 
           self.tctrl_mut(index)
    }

    #[doc="Read the TCTRL register."]
    #[inline] pub fn tctrl<I: Into<bits::R4>>(&self, index: I) -> Tctrl { 
        unsafe {
            read_volatile(self.tctrl_ptr(index))
        }
    }

    #[doc="Write the TCTRL register."]
    #[inline] pub fn set_tctrl<I: Into<bits::R4>, F: FnOnce(Tctrl) -> Tctrl>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tctrl_mut(index), f(Tctrl(0)));
        }
        self
    }

    #[doc="Modify the TCTRL register."]
    #[inline] pub fn with_tctrl<I: Into<bits::R4> + Copy, F: FnOnce(Tctrl) -> Tctrl>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tctrl_mut(index), f(self.tctrl(index)));
        }
        self
    }

}

#[doc="Version ID Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc="Feature Number"]
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

#[doc="Parameter Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Param(pub u32);
impl Param {
    #[doc="Number of Timer Channels"]
    #[inline] pub fn channel(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CHANNEL != 0"]
    #[inline] pub fn test_channel(&self) -> bool {
        self.channel() != 0
    }

    #[doc="Sets the CHANNEL field."]
    #[inline] pub fn set_channel<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Number of External Trigger Inputs"]
    #[inline] pub fn ext_trig(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if EXT_TRIG != 0"]
    #[inline] pub fn test_ext_trig(&self) -> bool {
        self.ext_trig() != 0
    }

    #[doc="Sets the EXT_TRIG field."]
    #[inline] pub fn set_ext_trig<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
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
        if self.channel() != 0 { try!(write!(f, " channel=0x{:x}", self.channel()))}
        if self.ext_trig() != 0 { try!(write!(f, " ext_trig=0x{:x}", self.ext_trig()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Module Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mcr(pub u32);
impl Mcr {
    #[doc="Module Clock Enable"]
    #[inline] pub fn m_cen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if M_CEN != 0"]
    #[inline] pub fn test_m_cen(&self) -> bool {
        self.m_cen() != 0
    }

    #[doc="Sets the M_CEN field."]
    #[inline] pub fn set_m_cen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Software Reset Bit"]
    #[inline] pub fn sw_rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SW_RST != 0"]
    #[inline] pub fn test_sw_rst(&self) -> bool {
        self.sw_rst() != 0
    }

    #[doc="Sets the SW_RST field."]
    #[inline] pub fn set_sw_rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DOZE Mode Enable Bit"]
    #[inline] pub fn doze_en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DOZE_EN != 0"]
    #[inline] pub fn test_doze_en(&self) -> bool {
        self.doze_en() != 0
    }

    #[doc="Sets the DOZE_EN field."]
    #[inline] pub fn set_doze_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Debug Enable Bit"]
    #[inline] pub fn dbg_en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DBG_EN != 0"]
    #[inline] pub fn test_dbg_en(&self) -> bool {
        self.dbg_en() != 0
    }

    #[doc="Sets the DBG_EN field."]
    #[inline] pub fn set_dbg_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Mcr {
    #[inline]
    fn from(other: u32) -> Self {
         Mcr(other)
    }
}

impl ::core::fmt::Display for Mcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.m_cen() != 0 { try!(write!(f, " m_cen"))}
        if self.sw_rst() != 0 { try!(write!(f, " sw_rst"))}
        if self.doze_en() != 0 { try!(write!(f, " doze_en"))}
        if self.dbg_en() != 0 { try!(write!(f, " dbg_en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Module Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Msr(pub u32);
impl Msr {
    #[doc="Channel n Timer Interrupt Flag"]
    #[inline] pub fn tif<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TIF != 0"]
    #[inline] pub fn test_tif<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.tif(index) != 0
    }

    #[doc="Sets the TIF field."]
    #[inline] pub fn set_tif<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Msr {
    #[inline]
    fn from(other: u32) -> Self {
         Msr(other)
    }
}

impl ::core::fmt::Display for Msr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Msr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tif(0) != 0 { try!(write!(f, " tif[0]"))}
        if self.tif(1) != 0 { try!(write!(f, " tif[1]"))}
        if self.tif(2) != 0 { try!(write!(f, " tif[2]"))}
        if self.tif(3) != 0 { try!(write!(f, " tif[3]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Module Interrupt Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mier(pub u32);
impl Mier {
    #[doc="Channel n Timer Interrupt Enable"]
    #[inline] pub fn tie<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TIE != 0"]
    #[inline] pub fn test_tie<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.tie(index) != 0
    }

    #[doc="Sets the TIE field."]
    #[inline] pub fn set_tie<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Mier {
    #[inline]
    fn from(other: u32) -> Self {
         Mier(other)
    }
}

impl ::core::fmt::Display for Mier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tie(0) != 0 { try!(write!(f, " tie[0]"))}
        if self.tie(1) != 0 { try!(write!(f, " tie[1]"))}
        if self.tie(2) != 0 { try!(write!(f, " tie[2]"))}
        if self.tie(3) != 0 { try!(write!(f, " tie[3]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Set Timer Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Setten(pub u32);
impl Setten {
    #[doc="Set Timer n Enable"]
    #[inline] pub fn set_t_en<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SET_T_EN != 0"]
    #[inline] pub fn test_set_t_en<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.set_t_en(index) != 0
    }

    #[doc="Sets the SET_T_EN field."]
    #[inline] pub fn set_set_t_en<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Setten {
    #[inline]
    fn from(other: u32) -> Self {
         Setten(other)
    }
}

impl ::core::fmt::Display for Setten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Setten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.set_t_en(0) != 0 { try!(write!(f, " set_t_en[0]"))}
        if self.set_t_en(1) != 0 { try!(write!(f, " set_t_en[1]"))}
        if self.set_t_en(2) != 0 { try!(write!(f, " set_t_en[2]"))}
        if self.set_t_en(3) != 0 { try!(write!(f, " set_t_en[3]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clear Timer Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clrten(pub u32);
impl Clrten {
    #[doc="Clear Timer n Enable"]
    #[inline] pub fn clr_t_en<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CLR_T_EN != 0"]
    #[inline] pub fn test_clr_t_en<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.clr_t_en(index) != 0
    }

    #[doc="Sets the CLR_T_EN field."]
    #[inline] pub fn set_clr_t_en<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Clrten {
    #[inline]
    fn from(other: u32) -> Self {
         Clrten(other)
    }
}

impl ::core::fmt::Display for Clrten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clrten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clr_t_en(0) != 0 { try!(write!(f, " clr_t_en[0]"))}
        if self.clr_t_en(1) != 0 { try!(write!(f, " clr_t_en[1]"))}
        if self.clr_t_en(2) != 0 { try!(write!(f, " clr_t_en[2]"))}
        if self.clr_t_en(3) != 0 { try!(write!(f, " clr_t_en[3]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Timer Value Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tval(pub u32);
impl Tval {
    #[doc="Timer Value"]
    #[inline] pub fn tmr_val(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if TMR_VAL != 0"]
    #[inline] pub fn test_tmr_val(&self) -> bool {
        self.tmr_val() != 0
    }

    #[doc="Sets the TMR_VAL field."]
    #[inline] pub fn set_tmr_val<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tval {
    #[inline]
    fn from(other: u32) -> Self {
         Tval(other)
    }
}

impl ::core::fmt::Display for Tval {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tval {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Current Timer Value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cval(pub u32);
impl Cval {
    #[doc="Current Timer Value"]
    #[inline] pub fn tmr_cur_val(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if TMR_CUR_VAL != 0"]
    #[inline] pub fn test_tmr_cur_val(&self) -> bool {
        self.tmr_cur_val() != 0
    }

    #[doc="Sets the TMR_CUR_VAL field."]
    #[inline] pub fn set_tmr_cur_val<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cval {
    #[inline]
    fn from(other: u32) -> Self {
         Cval(other)
    }
}

impl ::core::fmt::Display for Cval {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cval {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Timer Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tctrl(pub u32);
impl Tctrl {
    #[doc="Timer Enable"]
    #[inline] pub fn t_en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if T_EN != 0"]
    #[inline] pub fn test_t_en(&self) -> bool {
        self.t_en() != 0
    }

    #[doc="Sets the T_EN field."]
    #[inline] pub fn set_t_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Chain Channel"]
    #[inline] pub fn chain(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHAIN != 0"]
    #[inline] pub fn test_chain(&self) -> bool {
        self.chain() != 0
    }

    #[doc="Sets the CHAIN field."]
    #[inline] pub fn set_chain<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Timer Operation Mode"]
    #[inline] pub fn mode(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Timer Start On Trigger"]
    #[inline] pub fn tsot(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TSOT != 0"]
    #[inline] pub fn test_tsot(&self) -> bool {
        self.tsot() != 0
    }

    #[doc="Sets the TSOT field."]
    #[inline] pub fn set_tsot<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Timer Stop On Interrupt"]
    #[inline] pub fn tsoi(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TSOI != 0"]
    #[inline] pub fn test_tsoi(&self) -> bool {
        self.tsoi() != 0
    }

    #[doc="Sets the TSOI field."]
    #[inline] pub fn set_tsoi<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Timer Reload On Trigger"]
    #[inline] pub fn trot(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if TROT != 0"]
    #[inline] pub fn test_trot(&self) -> bool {
        self.trot() != 0
    }

    #[doc="Sets the TROT field."]
    #[inline] pub fn set_trot<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Trigger Source"]
    #[inline] pub fn trg_src(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if TRG_SRC != 0"]
    #[inline] pub fn test_trg_src(&self) -> bool {
        self.trg_src() != 0
    }

    #[doc="Sets the TRG_SRC field."]
    #[inline] pub fn set_trg_src<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Trigger Select"]
    #[inline] pub fn trg_sel(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if TRG_SEL != 0"]
    #[inline] pub fn test_trg_sel(&self) -> bool {
        self.trg_sel() != 0
    }

    #[doc="Sets the TRG_SEL field."]
    #[inline] pub fn set_trg_sel<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Tctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Tctrl(other)
    }
}

impl ::core::fmt::Display for Tctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.t_en() != 0 { try!(write!(f, " t_en"))}
        if self.chain() != 0 { try!(write!(f, " chain"))}
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        if self.tsot() != 0 { try!(write!(f, " tsot"))}
        if self.tsoi() != 0 { try!(write!(f, " tsoi"))}
        if self.trot() != 0 { try!(write!(f, " trot"))}
        if self.trg_src() != 0 { try!(write!(f, " trg_src"))}
        if self.trg_sel() != 0 { try!(write!(f, " trg_sel=0x{:x}", self.trg_sel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

