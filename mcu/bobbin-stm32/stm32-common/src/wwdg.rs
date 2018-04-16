
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="WWDG Peripheral"]
pub struct WwdgPeriph(pub usize); 

impl WwdgPeriph {
    #[doc="Get the CR Register."]
    #[inline] pub fn cr_reg(&self) -> ::bobbin_mcu::register::Register<Cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr, 0x0)
    }

    #[doc="Get the *mut pointer for the CR register."]
    #[inline] pub fn cr_mut(&self) -> *mut Cr { 
        self.cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR register."]
    #[inline] pub fn cr_ptr(&self) -> *const Cr { 
        self.cr_reg().ptr()
    }

    #[doc="Read the CR register."]
    #[inline] pub fn cr(&self) -> Cr { 
        self.cr_reg().read()
    }

    #[doc="Write the CR register."]
    #[inline] pub fn write_cr(&self, value: Cr) -> &Self { 
        self.cr_reg().write(value);
        self
    }

    #[doc="Set the CR register."]
    #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        self.cr_reg().set(f);
        self
    }

    #[doc="Modify the CR register."]
    #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        self.cr_reg().with(f);
        self
    }

    #[doc="Get the CFR Register."]
    #[inline] pub fn cfr_reg(&self) -> ::bobbin_mcu::register::Register<Cfr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cfr, 0x4)
    }

    #[doc="Get the *mut pointer for the CFR register."]
    #[inline] pub fn cfr_mut(&self) -> *mut Cfr { 
        self.cfr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CFR register."]
    #[inline] pub fn cfr_ptr(&self) -> *const Cfr { 
        self.cfr_reg().ptr()
    }

    #[doc="Read the CFR register."]
    #[inline] pub fn cfr(&self) -> Cfr { 
        self.cfr_reg().read()
    }

    #[doc="Write the CFR register."]
    #[inline] pub fn write_cfr(&self, value: Cfr) -> &Self { 
        self.cfr_reg().write(value);
        self
    }

    #[doc="Set the CFR register."]
    #[inline] pub fn set_cfr<F: FnOnce(Cfr) -> Cfr>(&self, f: F) -> &Self {
        self.cfr_reg().set(f);
        self
    }

    #[doc="Modify the CFR register."]
    #[inline] pub fn with_cfr<F: FnOnce(Cfr) -> Cfr>(&self, f: F) -> &Self {
        self.cfr_reg().with(f);
        self
    }

    #[doc="Get the SR Register."]
    #[inline] pub fn sr_reg(&self) -> ::bobbin_mcu::register::Register<Sr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sr, 0x8)
    }

    #[doc="Get the *mut pointer for the SR register."]
    #[inline] pub fn sr_mut(&self) -> *mut Sr { 
        self.sr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SR register."]
    #[inline] pub fn sr_ptr(&self) -> *const Sr { 
        self.sr_reg().ptr()
    }

    #[doc="Read the SR register."]
    #[inline] pub fn sr(&self) -> Sr { 
        self.sr_reg().read()
    }

    #[doc="Write the SR register."]
    #[inline] pub fn write_sr(&self, value: Sr) -> &Self { 
        self.sr_reg().write(value);
        self
    }

    #[doc="Set the SR register."]
    #[inline] pub fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        self.sr_reg().set(f);
        self
    }

    #[doc="Modify the SR register."]
    #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        self.sr_reg().with(f);
        self
    }

}

#[doc="Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Activation bit"]
    #[inline] pub fn wdga(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if WDGA != 0"]
    #[inline] pub fn test_wdga(&self) -> bool {
        self.wdga() != 0
    }

    #[doc="Sets the WDGA field."]
    #[inline] pub fn set_wdga<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="7-bit counter (MSB to LSB)"]
    #[inline] pub fn t(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if T != 0"]
    #[inline] pub fn test_t(&self) -> bool {
        self.t() != 0
    }

    #[doc="Sets the T field."]
    #[inline] pub fn set_t<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cr {
    #[inline]
    fn from(other: u32) -> Self {
         Cr(other)
    }
}

impl ::core::fmt::Display for Cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wdga() != 0 { try!(write!(f, " wdga"))}
        if self.t() != 0 { try!(write!(f, " t=0x{:x}", self.t()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfr(pub u32);
impl Cfr {
    #[doc="Early wakeup interrupt"]
    #[inline] pub fn ewi(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if EWI != 0"]
    #[inline] pub fn test_ewi(&self) -> bool {
        self.ewi() != 0
    }

    #[doc="Sets the EWI field."]
    #[inline] pub fn set_ewi<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Timer base"]
    #[inline] pub fn wdgtb(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x3) as u8) } // [8:7]
    }

    #[doc="Returns true if WDGTB != 0"]
    #[inline] pub fn test_wdgtb(&self) -> bool {
        self.wdgtb() != 0
    }

    #[doc="Sets the WDGTB field."]
    #[inline] pub fn set_wdgtb<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="7-bit window value"]
    #[inline] pub fn w(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if W != 0"]
    #[inline] pub fn test_w(&self) -> bool {
        self.w() != 0
    }

    #[doc="Sets the W field."]
    #[inline] pub fn set_w<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cfr {
    #[inline]
    fn from(other: u32) -> Self {
         Cfr(other)
    }
}

impl ::core::fmt::Display for Cfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ewi() != 0 { try!(write!(f, " ewi"))}
        if self.wdgtb() != 0 { try!(write!(f, " wdgtb=0x{:x}", self.wdgtb()))}
        if self.w() != 0 { try!(write!(f, " w=0x{:x}", self.w()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="Early wakeup interrupt flag"]
    #[inline] pub fn ewif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EWIF != 0"]
    #[inline] pub fn test_ewif(&self) -> bool {
        self.ewif() != 0
    }

    #[doc="Sets the EWIF field."]
    #[inline] pub fn set_ewif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sr {
    #[inline]
    fn from(other: u32) -> Self {
         Sr(other)
    }
}

impl ::core::fmt::Display for Sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ewif() != 0 { try!(write!(f, " ewif"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

