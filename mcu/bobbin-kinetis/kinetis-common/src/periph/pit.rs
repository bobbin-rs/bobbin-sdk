
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="PIT Peripheral"]
pub struct PitPeriph(pub usize); 

pub struct PitCh { pub periph: PitPeriph, pub index: usize }

impl PitPeriph {
    #[doc="Get the MCR Register."]
    #[inline] pub fn mcr_reg(&self) -> ::bobbin_mcu::register::Register<Mcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mcr, 0x0)
    }

    #[doc="Get the *mut pointer for the MCR register."]
    #[inline] pub fn mcr_mut(&self) -> *mut Mcr { 
        self.mcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MCR register."]
    #[inline] pub fn mcr_ptr(&self) -> *const Mcr { 
        self.mcr_reg().ptr()
    }

    #[doc="Read the MCR register."]
    #[inline] pub fn mcr(&self) -> Mcr { 
        self.mcr_reg().read()
    }

    #[doc="Write the MCR register."]
    #[inline] pub fn write_mcr(&self, value: Mcr) -> &Self { 
        self.mcr_reg().write(value);
        self
    }

    #[doc="Set the MCR register."]
    #[inline] pub fn set_mcr<F: FnOnce(Mcr) -> Mcr>(&self, f: F) -> &Self {
        self.mcr_reg().set(f);
        self
    }

    #[doc="Modify the MCR register."]
    #[inline] pub fn with_mcr<F: FnOnce(Mcr) -> Mcr>(&self, f: F) -> &Self {
        self.mcr_reg().with(f);
        self
    }

    #[doc="Get the LDVAL Register."]
    #[inline] pub fn ldval_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Ldval, ::bobbin_bits::R4> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Ldval, 0x100, 0x10)
    }

    #[doc="Get the *mut pointer for the LDVAL register."]
    #[inline] pub fn ldval_mut<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *mut Ldval { 
        self.ldval_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the LDVAL register."]
    #[inline] pub fn ldval_ptr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *const Ldval { 
        self.ldval_reg().ptr(index.into())
    }

    #[doc="Read the LDVAL register."]
    #[inline] pub fn ldval<I: Into<::bobbin_bits::R4>>(&self, index: I) -> Ldval { 
        self.ldval_reg().read(index.into())
    }

    #[doc="Write the LDVAL register."]
    #[inline] pub fn write_ldval<I: Into<::bobbin_bits::R4>>(&self, index: I, value: Ldval) -> &Self {
        self.ldval_reg().write(index.into(), value);
        self
    }

    #[doc="Set the LDVAL register."]
    #[inline] pub fn set_ldval<I: Into<::bobbin_bits::R4>, F: FnOnce(Ldval) -> Ldval>(&self, index: I, f: F) -> &Self {
        self.ldval_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the LDVAL register."]
    #[inline] pub fn with_ldval<I: Into<::bobbin_bits::R4> + Copy, F: FnOnce(Ldval) -> Ldval>(&self, index: I, f: F) -> &Self {
        self.ldval_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CVAL Register."]
    #[inline] pub fn cval_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Cval, ::bobbin_bits::R4> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Cval, 0x104, 0x10)
    }

    #[doc="Get the *mut pointer for the CVAL register."]
    #[inline] pub fn cval_mut<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *mut Cval { 
        self.cval_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CVAL register."]
    #[inline] pub fn cval_ptr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *const Cval { 
        self.cval_reg().ptr(index.into())
    }

    #[doc="Read the CVAL register."]
    #[inline] pub fn cval<I: Into<::bobbin_bits::R4>>(&self, index: I) -> Cval { 
        self.cval_reg().read(index.into())
    }

    #[doc="Get the TCTRL Register."]
    #[inline] pub fn tctrl_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Tctrl, ::bobbin_bits::R4> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Tctrl, 0x108, 0x10)
    }

    #[doc="Get the *mut pointer for the TCTRL register."]
    #[inline] pub fn tctrl_mut<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *mut Tctrl { 
        self.tctrl_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the TCTRL register."]
    #[inline] pub fn tctrl_ptr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *const Tctrl { 
        self.tctrl_reg().ptr(index.into())
    }

    #[doc="Read the TCTRL register."]
    #[inline] pub fn tctrl<I: Into<::bobbin_bits::R4>>(&self, index: I) -> Tctrl { 
        self.tctrl_reg().read(index.into())
    }

    #[doc="Write the TCTRL register."]
    #[inline] pub fn write_tctrl<I: Into<::bobbin_bits::R4>>(&self, index: I, value: Tctrl) -> &Self {
        self.tctrl_reg().write(index.into(), value);
        self
    }

    #[doc="Set the TCTRL register."]
    #[inline] pub fn set_tctrl<I: Into<::bobbin_bits::R4>, F: FnOnce(Tctrl) -> Tctrl>(&self, index: I, f: F) -> &Self {
        self.tctrl_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the TCTRL register."]
    #[inline] pub fn with_tctrl<I: Into<::bobbin_bits::R4> + Copy, F: FnOnce(Tctrl) -> Tctrl>(&self, index: I, f: F) -> &Self {
        self.tctrl_reg().with(index.into(), f);
        self
    }

    #[doc="Get the TFLG Register."]
    #[inline] pub fn tflg_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Tflg, ::bobbin_bits::R4> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Tflg, 0x10c, 0x10)
    }

    #[doc="Get the *mut pointer for the TFLG register."]
    #[inline] pub fn tflg_mut<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *mut Tflg { 
        self.tflg_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the TFLG register."]
    #[inline] pub fn tflg_ptr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *const Tflg { 
        self.tflg_reg().ptr(index.into())
    }

    #[doc="Read the TFLG register."]
    #[inline] pub fn tflg<I: Into<::bobbin_bits::R4>>(&self, index: I) -> Tflg { 
        self.tflg_reg().read(index.into())
    }

    #[doc="Write the TFLG register."]
    #[inline] pub fn write_tflg<I: Into<::bobbin_bits::R4>>(&self, index: I, value: Tflg) -> &Self {
        self.tflg_reg().write(index.into(), value);
        self
    }

    #[doc="Set the TFLG register."]
    #[inline] pub fn set_tflg<I: Into<::bobbin_bits::R4>, F: FnOnce(Tflg) -> Tflg>(&self, index: I, f: F) -> &Self {
        self.tflg_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the TFLG register."]
    #[inline] pub fn with_tflg<I: Into<::bobbin_bits::R4> + Copy, F: FnOnce(Tflg) -> Tflg>(&self, index: I, f: F) -> &Self {
        self.tflg_reg().with(index.into(), f);
        self
    }

}

#[doc="PIT Module Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mcr(pub u32);
impl Mcr {
    #[doc="Freeze"]
    #[inline] pub fn frz(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FRZ != 0"]
    #[inline] pub fn test_frz(&self) -> bool {
        self.frz() != 0
    }

    #[doc="Sets the FRZ field."]
    #[inline] pub fn set_frz<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Module Disable - (PIT section)"]
    #[inline] pub fn mdis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MDIS != 0"]
    #[inline] pub fn test_mdis(&self) -> bool {
        self.mdis() != 0
    }

    #[doc="Sets the MDIS field."]
    #[inline] pub fn set_mdis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
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
    #[inline] pub fn tsv(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if TSV != 0"]
    #[inline] pub fn test_tsv(&self) -> bool {
        self.tsv() != 0
    }

    #[doc="Sets the TSV field."]
    #[inline] pub fn set_tsv<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
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
    #[inline] pub fn tvl(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if TVL != 0"]
    #[inline] pub fn test_tvl(&self) -> bool {
        self.tvl() != 0
    }

    #[doc="Sets the TVL field."]
    #[inline] pub fn set_tvl<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
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
    #[inline] pub fn ten(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TEN != 0"]
    #[inline] pub fn test_ten(&self) -> bool {
        self.ten() != 0
    }

    #[doc="Sets the TEN field."]
    #[inline] pub fn set_ten<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Timer Interrupt Enable"]
    #[inline] pub fn tie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TIE != 0"]
    #[inline] pub fn test_tie(&self) -> bool {
        self.tie() != 0
    }

    #[doc="Sets the TIE field."]
    #[inline] pub fn set_tie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Chain Mode"]
    #[inline] pub fn chn(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CHN != 0"]
    #[inline] pub fn test_chn(&self) -> bool {
        self.chn() != 0
    }

    #[doc="Sets the CHN field."]
    #[inline] pub fn set_chn<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
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
    #[inline] pub fn tif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TIF != 0"]
    #[inline] pub fn test_tif(&self) -> bool {
        self.tif() != 0
    }

    #[doc="Sets the TIF field."]
    #[inline] pub fn set_tif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
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

