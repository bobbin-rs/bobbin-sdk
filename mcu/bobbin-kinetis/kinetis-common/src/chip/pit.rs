
#[allow(unused_imports)] use bobbin_common::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="PIT Peripheral"]
pub struct PitPeriph(pub usize); 

pub struct PitCh { pub periph: PitPeriph, pub index: usize }

impl PitPeriph {
    #[doc="Get the *mut pointer for the MCR register."]
    #[inline] pub fn mcr_mut(&self) -> *mut Mcr { 
        (self.0 + 0x0) as *mut Mcr
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

    #[doc="Get the *mut pointer for the LDVAL register."]
    #[inline] pub fn ldval_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Ldval { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x100 + (index << 4)) as *mut Ldval
    }

    #[doc="Get the *const pointer for the LDVAL register."]
    #[inline] pub fn ldval_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Ldval { 
           self.ldval_mut(index)
    }

    #[doc="Read the LDVAL register."]
    #[inline] pub fn ldval<I: Into<bits::R4>>(&self, index: I) -> Ldval { 
        unsafe {
            read_volatile(self.ldval_ptr(index))
        }
    }

    #[doc="Write the LDVAL register."]
    #[inline] pub fn set_ldval<I: Into<bits::R4>, F: FnOnce(Ldval) -> Ldval>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.ldval_mut(index), f(Ldval(0)));
        }
        self
    }

    #[doc="Modify the LDVAL register."]
    #[inline] pub fn with_ldval<I: Into<bits::R4> + Copy, F: FnOnce(Ldval) -> Ldval>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.ldval_mut(index), f(self.ldval(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the CVAL register."]
    #[inline] pub fn cval_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Cval { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x104 + (index << 4)) as *mut Cval
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
        (self.0 + 0x108 + (index << 4)) as *mut Tctrl
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

    #[doc="Get the *mut pointer for the TFLG register."]
    #[inline] pub fn tflg_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Tflg { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x10c + (index << 4)) as *mut Tflg
    }

    #[doc="Get the *const pointer for the TFLG register."]
    #[inline] pub fn tflg_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Tflg { 
           self.tflg_mut(index)
    }

    #[doc="Read the TFLG register."]
    #[inline] pub fn tflg<I: Into<bits::R4>>(&self, index: I) -> Tflg { 
        unsafe {
            read_volatile(self.tflg_ptr(index))
        }
    }

    #[doc="Write the TFLG register."]
    #[inline] pub fn set_tflg<I: Into<bits::R4>, F: FnOnce(Tflg) -> Tflg>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tflg_mut(index), f(Tflg(0)));
        }
        self
    }

    #[doc="Modify the TFLG register."]
    #[inline] pub fn with_tflg<I: Into<bits::R4> + Copy, F: FnOnce(Tflg) -> Tflg>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tflg_mut(index), f(self.tflg(index)));
        }
        self
    }

}

#[doc="PIT Module Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mcr(pub u32);
impl Mcr {
    #[doc="Freeze"]
    #[inline] pub fn frz(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FRZ != 0"]
    #[inline] pub fn test_frz(&self) -> bool {
        self.frz() != 0
    }

    #[doc="Sets the FRZ field."]
    #[inline] pub fn set_frz<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Module Disable - (PIT section)"]
    #[inline] pub fn mdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MDIS != 0"]
    #[inline] pub fn test_mdis(&self) -> bool {
        self.mdis() != 0
    }

    #[doc="Sets the MDIS field."]
    #[inline] pub fn set_mdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
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
        if self.frz() != 0 { try!(write!(f, " frz"))}
        if self.mdis() != 0 { try!(write!(f, " mdis"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Timer Load Value Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ldval(pub u32);
impl Ldval {
    #[doc="Timer Start Value"]
    #[inline] pub fn tsv(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if TSV != 0"]
    #[inline] pub fn test_tsv(&self) -> bool {
        self.tsv() != 0
    }

    #[doc="Sets the TSV field."]
    #[inline] pub fn set_tsv<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ldval {
    #[inline]
    fn from(other: u32) -> Self {
         Ldval(other)
    }
}

impl ::core::fmt::Display for Ldval {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ldval {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Current Timer Value Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cval(pub u32);
impl Cval {
    #[doc="Current Timer Value"]
    #[inline] pub fn tvl(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if TVL != 0"]
    #[inline] pub fn test_tvl(&self) -> bool {
        self.tvl() != 0
    }

    #[doc="Sets the TVL field."]
    #[inline] pub fn set_tvl<V: Into<bits::U32>>(mut self, value: V) -> Self {
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
    #[inline] pub fn ten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TEN != 0"]
    #[inline] pub fn test_ten(&self) -> bool {
        self.ten() != 0
    }

    #[doc="Sets the TEN field."]
    #[inline] pub fn set_ten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Timer Interrupt Enable"]
    #[inline] pub fn tie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TIE != 0"]
    #[inline] pub fn test_tie(&self) -> bool {
        self.tie() != 0
    }

    #[doc="Sets the TIE field."]
    #[inline] pub fn set_tie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Chain Mode"]
    #[inline] pub fn chn(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CHN != 0"]
    #[inline] pub fn test_chn(&self) -> bool {
        self.chn() != 0
    }

    #[doc="Sets the CHN field."]
    #[inline] pub fn set_chn<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
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
        if self.ten() != 0 { try!(write!(f, " ten"))}
        if self.tie() != 0 { try!(write!(f, " tie"))}
        if self.chn() != 0 { try!(write!(f, " chn"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Timer Flag Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tflg(pub u32);
impl Tflg {
    #[doc="Timer Interrupt Flag"]
    #[inline] pub fn tif(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TIF != 0"]
    #[inline] pub fn test_tif(&self) -> bool {
        self.tif() != 0
    }

    #[doc="Sets the TIF field."]
    #[inline] pub fn set_tif<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tflg {
    #[inline]
    fn from(other: u32) -> Self {
         Tflg(other)
    }
}

impl ::core::fmt::Display for Tflg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tflg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tif() != 0 { try!(write!(f, " tif"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

