#[allow(unused_imports)] use ::bobbin_common::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="FIREWALL Peripheral"]
pub struct FirewallPeriph(pub usize); 

impl FirewallPeriph {
    #[doc="Get the *mut pointer for the CSSA register."]
    #[inline] pub fn cssa_mut(&self) -> *mut Cssa { 
        (self.0 + 0x0) as *mut Cssa
    }

    #[doc="Get the *const pointer for the CSSA register."]
    #[inline] pub fn cssa_ptr(&self) -> *const Cssa { 
           self.cssa_mut()
    }

    #[doc="Read the CSSA register."]
    #[inline] pub fn cssa(&self) -> Cssa { 
        unsafe {
            read_volatile(self.cssa_ptr())
        }
    }

    #[doc="Write the CSSA register."]
    #[inline] pub fn set_cssa<F: FnOnce(Cssa) -> Cssa>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cssa_mut(), f(Cssa(0)));
        }
        self
    }

    #[doc="Modify the CSSA register."]
    #[inline] pub fn with_cssa<F: FnOnce(Cssa) -> Cssa>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cssa_mut(), f(self.cssa()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSL register."]
    #[inline] pub fn csl_mut(&self) -> *mut Csl { 
        (self.0 + 0x4) as *mut Csl
    }

    #[doc="Get the *const pointer for the CSL register."]
    #[inline] pub fn csl_ptr(&self) -> *const Csl { 
           self.csl_mut()
    }

    #[doc="Read the CSL register."]
    #[inline] pub fn csl(&self) -> Csl { 
        unsafe {
            read_volatile(self.csl_ptr())
        }
    }

    #[doc="Write the CSL register."]
    #[inline] pub fn set_csl<F: FnOnce(Csl) -> Csl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csl_mut(), f(Csl(0)));
        }
        self
    }

    #[doc="Modify the CSL register."]
    #[inline] pub fn with_csl<F: FnOnce(Csl) -> Csl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csl_mut(), f(self.csl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the NVDSSA register."]
    #[inline] pub fn nvdssa_mut(&self) -> *mut Nvdssa { 
        (self.0 + 0x8) as *mut Nvdssa
    }

    #[doc="Get the *const pointer for the NVDSSA register."]
    #[inline] pub fn nvdssa_ptr(&self) -> *const Nvdssa { 
           self.nvdssa_mut()
    }

    #[doc="Read the NVDSSA register."]
    #[inline] pub fn nvdssa(&self) -> Nvdssa { 
        unsafe {
            read_volatile(self.nvdssa_ptr())
        }
    }

    #[doc="Write the NVDSSA register."]
    #[inline] pub fn set_nvdssa<F: FnOnce(Nvdssa) -> Nvdssa>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.nvdssa_mut(), f(Nvdssa(0)));
        }
        self
    }

    #[doc="Modify the NVDSSA register."]
    #[inline] pub fn with_nvdssa<F: FnOnce(Nvdssa) -> Nvdssa>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.nvdssa_mut(), f(self.nvdssa()));
        }
        self
    }

    #[doc="Get the *mut pointer for the NVDSL register."]
    #[inline] pub fn nvdsl_mut(&self) -> *mut Nvdsl { 
        (self.0 + 0xc) as *mut Nvdsl
    }

    #[doc="Get the *const pointer for the NVDSL register."]
    #[inline] pub fn nvdsl_ptr(&self) -> *const Nvdsl { 
           self.nvdsl_mut()
    }

    #[doc="Read the NVDSL register."]
    #[inline] pub fn nvdsl(&self) -> Nvdsl { 
        unsafe {
            read_volatile(self.nvdsl_ptr())
        }
    }

    #[doc="Write the NVDSL register."]
    #[inline] pub fn set_nvdsl<F: FnOnce(Nvdsl) -> Nvdsl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.nvdsl_mut(), f(Nvdsl(0)));
        }
        self
    }

    #[doc="Modify the NVDSL register."]
    #[inline] pub fn with_nvdsl<F: FnOnce(Nvdsl) -> Nvdsl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.nvdsl_mut(), f(self.nvdsl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the VDSSA register."]
    #[inline] pub fn vdssa_mut(&self) -> *mut Vdssa { 
        (self.0 + 0x10) as *mut Vdssa
    }

    #[doc="Get the *const pointer for the VDSSA register."]
    #[inline] pub fn vdssa_ptr(&self) -> *const Vdssa { 
           self.vdssa_mut()
    }

    #[doc="Read the VDSSA register."]
    #[inline] pub fn vdssa(&self) -> Vdssa { 
        unsafe {
            read_volatile(self.vdssa_ptr())
        }
    }

    #[doc="Write the VDSSA register."]
    #[inline] pub fn set_vdssa<F: FnOnce(Vdssa) -> Vdssa>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.vdssa_mut(), f(Vdssa(0)));
        }
        self
    }

    #[doc="Modify the VDSSA register."]
    #[inline] pub fn with_vdssa<F: FnOnce(Vdssa) -> Vdssa>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.vdssa_mut(), f(self.vdssa()));
        }
        self
    }

    #[doc="Get the *mut pointer for the VDSL register."]
    #[inline] pub fn vdsl_mut(&self) -> *mut Vdsl { 
        (self.0 + 0x14) as *mut Vdsl
    }

    #[doc="Get the *const pointer for the VDSL register."]
    #[inline] pub fn vdsl_ptr(&self) -> *const Vdsl { 
           self.vdsl_mut()
    }

    #[doc="Read the VDSL register."]
    #[inline] pub fn vdsl(&self) -> Vdsl { 
        unsafe {
            read_volatile(self.vdsl_ptr())
        }
    }

    #[doc="Write the VDSL register."]
    #[inline] pub fn set_vdsl<F: FnOnce(Vdsl) -> Vdsl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.vdsl_mut(), f(Vdsl(0)));
        }
        self
    }

    #[doc="Modify the VDSL register."]
    #[inline] pub fn with_vdsl<F: FnOnce(Vdsl) -> Vdsl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.vdsl_mut(), f(self.vdsl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CR register."]
    #[inline] pub fn cr_mut(&self) -> *mut Cr { 
        (self.0 + 0x20) as *mut Cr
    }

    #[doc="Get the *const pointer for the CR register."]
    #[inline] pub fn cr_ptr(&self) -> *const Cr { 
           self.cr_mut()
    }

    #[doc="Read the CR register."]
    #[inline] pub fn cr(&self) -> Cr { 
        unsafe {
            read_volatile(self.cr_ptr())
        }
    }

    #[doc="Write the CR register."]
    #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr_mut(), f(Cr(0)));
        }
        self
    }

    #[doc="Modify the CR register."]
    #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr_mut(), f(self.cr()));
        }
        self
    }

}

#[doc="Code segment start address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cssa(pub u32);
impl Cssa {
    #[doc="code segment start address"]
    #[inline] pub fn add(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffff) as u16) } // [23:8]
    }

    #[doc="Returns true if ADD != 0"]
    #[inline] pub fn test_add(&self) -> bool {
        self.add() != 0
    }

    #[doc="Sets the ADD field."]
    #[inline] pub fn set_add<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Cssa {
    #[inline]
    fn from(other: u32) -> Self {
         Cssa(other)
    }
}

impl ::core::fmt::Display for Cssa {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cssa {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.add() != 0 { try!(write!(f, " add=0x{:x}", self.add()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Code segment length"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csl(pub u32);
impl Csl {
    #[doc="code segment length"]
    #[inline] pub fn leng(&self) -> bits::U14 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3fff) as u16) } // [21:8]
    }

    #[doc="Returns true if LENG != 0"]
    #[inline] pub fn test_leng(&self) -> bool {
        self.leng() != 0
    }

    #[doc="Sets the LENG field."]
    #[inline] pub fn set_leng<V: Into<bits::U14>>(mut self, value: V) -> Self {
        let value: bits::U14 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fff << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Csl {
    #[inline]
    fn from(other: u32) -> Self {
         Csl(other)
    }
}

impl ::core::fmt::Display for Csl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.leng() != 0 { try!(write!(f, " leng=0x{:x}", self.leng()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Non-volatile data segment start address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Nvdssa(pub u32);
impl Nvdssa {
    #[doc="Non-volatile data segment start address"]
    #[inline] pub fn add(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffff) as u16) } // [23:8]
    }

    #[doc="Returns true if ADD != 0"]
    #[inline] pub fn test_add(&self) -> bool {
        self.add() != 0
    }

    #[doc="Sets the ADD field."]
    #[inline] pub fn set_add<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Nvdssa {
    #[inline]
    fn from(other: u32) -> Self {
         Nvdssa(other)
    }
}

impl ::core::fmt::Display for Nvdssa {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Nvdssa {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.add() != 0 { try!(write!(f, " add=0x{:x}", self.add()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Non-volatile data segment length"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Nvdsl(pub u32);
impl Nvdsl {
    #[doc="Non-volatile data segment length"]
    #[inline] pub fn leng(&self) -> bits::U14 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3fff) as u16) } // [21:8]
    }

    #[doc="Returns true if LENG != 0"]
    #[inline] pub fn test_leng(&self) -> bool {
        self.leng() != 0
    }

    #[doc="Sets the LENG field."]
    #[inline] pub fn set_leng<V: Into<bits::U14>>(mut self, value: V) -> Self {
        let value: bits::U14 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fff << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Nvdsl {
    #[inline]
    fn from(other: u32) -> Self {
         Nvdsl(other)
    }
}

impl ::core::fmt::Display for Nvdsl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Nvdsl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.leng() != 0 { try!(write!(f, " leng=0x{:x}", self.leng()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Volatile data segment start address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Vdssa(pub u32);
impl Vdssa {
    #[doc="Volatile data segment start address"]
    #[inline] pub fn add(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3ff) as u16) } // [15:6]
    }

    #[doc="Returns true if ADD != 0"]
    #[inline] pub fn test_add(&self) -> bool {
        self.add() != 0
    }

    #[doc="Sets the ADD field."]
    #[inline] pub fn set_add<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for Vdssa {
    #[inline]
    fn from(other: u32) -> Self {
         Vdssa(other)
    }
}

impl ::core::fmt::Display for Vdssa {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Vdssa {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.add() != 0 { try!(write!(f, " add=0x{:x}", self.add()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Volatile data segment length"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Vdsl(pub u32);
impl Vdsl {
    #[doc="Non-volatile data segment length"]
    #[inline] pub fn leng(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3ff) as u16) } // [15:6]
    }

    #[doc="Returns true if LENG != 0"]
    #[inline] pub fn test_leng(&self) -> bool {
        self.leng() != 0
    }

    #[doc="Sets the LENG field."]
    #[inline] pub fn set_leng<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for Vdsl {
    #[inline]
    fn from(other: u32) -> Self {
         Vdsl(other)
    }
}

impl ::core::fmt::Display for Vdsl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Vdsl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.leng() != 0 { try!(write!(f, " leng=0x{:x}", self.leng()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Volatile data execution"]
    #[inline] pub fn vde(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if VDE != 0"]
    #[inline] pub fn test_vde(&self) -> bool {
        self.vde() != 0
    }

    #[doc="Sets the VDE field."]
    #[inline] pub fn set_vde<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Volatile data shared"]
    #[inline] pub fn vds(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if VDS != 0"]
    #[inline] pub fn test_vds(&self) -> bool {
        self.vds() != 0
    }

    #[doc="Sets the VDS field."]
    #[inline] pub fn set_vds<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Firewall pre alarm"]
    #[inline] pub fn fpa(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FPA != 0"]
    #[inline] pub fn test_fpa(&self) -> bool {
        self.fpa() != 0
    }

    #[doc="Sets the FPA field."]
    #[inline] pub fn set_fpa<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
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
        if self.vde() != 0 { try!(write!(f, " vde"))}
        if self.vds() != 0 { try!(write!(f, " vds"))}
        if self.fpa() != 0 { try!(write!(f, " fpa"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

