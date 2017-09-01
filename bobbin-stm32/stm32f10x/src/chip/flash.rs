//! FLASH
#[allow(unused_imports)] use bobbin_common::*;

periph!(FLASH, Flash, 0x40022000);

#[doc="FLASH"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Flash(pub usize);
impl Flash {
    #[doc="Get the *mut pointer for the ACR register."]
    #[inline] pub fn acr_mut(&self) -> *mut Acr { 
        (self.0 + 0x0) as *mut Acr
    }

    #[doc="Get the *const pointer for the ACR register."]
    #[inline] pub fn acr_ptr(&self) -> *const Acr { 
           self.acr_mut()
    }

    #[doc="Read the ACR register."]
    #[inline] pub fn acr(&self) -> Acr { 
        unsafe {
            read_volatile(self.acr_ptr())
        }
    }

    #[doc="Write the ACR register."]
    #[inline] pub fn set_acr<F: FnOnce(Acr) -> Acr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.acr_mut(), f(Acr(0)));
        }
        self
    }

    #[doc="Modify the ACR register."]
    #[inline] pub fn with_acr<F: FnOnce(Acr) -> Acr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.acr_mut(), f(self.acr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the KEYR register."]
    #[inline] pub fn keyr_mut(&self) -> *mut Keyr { 
        (self.0 + 0x4) as *mut Keyr
    }

    #[doc="Get the *const pointer for the KEYR register."]
    #[inline] pub fn keyr_ptr(&self) -> *const Keyr { 
           self.keyr_mut()
    }

    #[doc="Write the KEYR register."]
    #[inline] pub fn set_keyr<F: FnOnce(Keyr) -> Keyr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.keyr_mut(), f(Keyr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the OPTKEYR register."]
    #[inline] pub fn optkeyr_mut(&self) -> *mut Optkeyr { 
        (self.0 + 0x8) as *mut Optkeyr
    }

    #[doc="Get the *const pointer for the OPTKEYR register."]
    #[inline] pub fn optkeyr_ptr(&self) -> *const Optkeyr { 
           self.optkeyr_mut()
    }

    #[doc="Write the OPTKEYR register."]
    #[inline] pub fn set_optkeyr<F: FnOnce(Optkeyr) -> Optkeyr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.optkeyr_mut(), f(Optkeyr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the SR register."]
    #[inline] pub fn sr_mut(&self) -> *mut Sr { 
        (self.0 + 0xc) as *mut Sr
    }

    #[doc="Get the *const pointer for the SR register."]
    #[inline] pub fn sr_ptr(&self) -> *const Sr { 
           self.sr_mut()
    }

    #[doc="Read the SR register."]
    #[inline] pub fn sr(&self) -> Sr { 
        unsafe {
            read_volatile(self.sr_ptr())
        }
    }

    #[doc="Write the SR register."]
    #[inline] pub fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sr_mut(), f(Sr(0)));
        }
        self
    }

    #[doc="Modify the SR register."]
    #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sr_mut(), f(self.sr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CR register."]
    #[inline] pub fn cr_mut(&self) -> *mut Cr { 
        (self.0 + 0x10) as *mut Cr
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

    #[doc="Get the *mut pointer for the AR register."]
    #[inline] pub fn ar_mut(&self) -> *mut Ar { 
        (self.0 + 0x14) as *mut Ar
    }

    #[doc="Get the *const pointer for the AR register."]
    #[inline] pub fn ar_ptr(&self) -> *const Ar { 
           self.ar_mut()
    }

    #[doc="Write the AR register."]
    #[inline] pub fn set_ar<F: FnOnce(Ar) -> Ar>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ar_mut(), f(Ar(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the OBR register."]
    #[inline] pub fn obr_mut(&self) -> *mut Obr { 
        (self.0 + 0x1c) as *mut Obr
    }

    #[doc="Get the *const pointer for the OBR register."]
    #[inline] pub fn obr_ptr(&self) -> *const Obr { 
           self.obr_mut()
    }

    #[doc="Read the OBR register."]
    #[inline] pub fn obr(&self) -> Obr { 
        unsafe {
            read_volatile(self.obr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the WRPR register."]
    #[inline] pub fn wrpr_mut(&self) -> *mut Wrpr { 
        (self.0 + 0x20) as *mut Wrpr
    }

    #[doc="Get the *const pointer for the WRPR register."]
    #[inline] pub fn wrpr_ptr(&self) -> *const Wrpr { 
           self.wrpr_mut()
    }

    #[doc="Read the WRPR register."]
    #[inline] pub fn wrpr(&self) -> Wrpr { 
        unsafe {
            read_volatile(self.wrpr_ptr())
        }
    }

}

#[doc="Flash access control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Acr(pub u32);
impl Acr {
    #[doc="Latency"]
    #[inline] pub fn latency(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Latency"]
    #[inline] pub fn test_latency(&self) -> bool {
        self.latency() != 0
    }

    #[doc="Latency"]
    #[inline] pub fn set_latency<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Flash half cycle access enable"]
    #[inline] pub fn hlfcya(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Flash half cycle access enable"]
    #[inline] pub fn test_hlfcya(&self) -> bool {
        self.hlfcya() != 0
    }

    #[doc="Flash half cycle access enable"]
    #[inline] pub fn set_hlfcya<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Prefetch buffer enable"]
    #[inline] pub fn prftbe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Prefetch buffer enable"]
    #[inline] pub fn test_prftbe(&self) -> bool {
        self.prftbe() != 0
    }

    #[doc="Prefetch buffer enable"]
    #[inline] pub fn set_prftbe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Prefetch buffer status"]
    #[inline] pub fn prftbs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Prefetch buffer status"]
    #[inline] pub fn test_prftbs(&self) -> bool {
        self.prftbs() != 0
    }

    #[doc="Prefetch buffer status"]
    #[inline] pub fn set_prftbs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u32> for Acr {
    #[inline]
    fn from(other: u32) -> Self {
         Acr(other)
    }
}

impl ::core::fmt::Display for Acr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Acr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.latency() != 0 { try!(write!(f, " latency=0x{:x}", self.latency()))}
        if self.hlfcya() != 0 { try!(write!(f, " hlfcya"))}
        if self.prftbe() != 0 { try!(write!(f, " prftbe"))}
        if self.prftbs() != 0 { try!(write!(f, " prftbs"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash key register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Keyr(pub u32);
impl Keyr {
    #[doc="FPEC key"]
    #[inline] pub fn key(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="FPEC key"]
    #[inline] pub fn test_key(&self) -> bool {
        self.key() != 0
    }

    #[doc="FPEC key"]
    #[inline] pub fn set_key<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Keyr {
    #[inline]
    fn from(other: u32) -> Self {
         Keyr(other)
    }
}

impl ::core::fmt::Display for Keyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Keyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash option key register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Optkeyr(pub u32);
impl Optkeyr {
    #[doc="Option byte key"]
    #[inline] pub fn optkey(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Option byte key"]
    #[inline] pub fn test_optkey(&self) -> bool {
        self.optkey() != 0
    }

    #[doc="Option byte key"]
    #[inline] pub fn set_optkey<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Optkeyr {
    #[inline]
    fn from(other: u32) -> Self {
         Optkeyr(other)
    }
}

impl ::core::fmt::Display for Optkeyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Optkeyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="End of operation"]
    #[inline] pub fn eop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="End of operation"]
    #[inline] pub fn test_eop(&self) -> bool {
        self.eop() != 0
    }

    #[doc="End of operation"]
    #[inline] pub fn set_eop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Write protection error"]
    #[inline] pub fn wrprterr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Write protection error"]
    #[inline] pub fn test_wrprterr(&self) -> bool {
        self.wrprterr() != 0
    }

    #[doc="Write protection error"]
    #[inline] pub fn set_wrprterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Programming error"]
    #[inline] pub fn pgerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Programming error"]
    #[inline] pub fn test_pgerr(&self) -> bool {
        self.pgerr() != 0
    }

    #[doc="Programming error"]
    #[inline] pub fn set_pgerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Busy"]
    #[inline] pub fn bsy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Busy"]
    #[inline] pub fn test_bsy(&self) -> bool {
        self.bsy() != 0
    }

    #[doc="Busy"]
    #[inline] pub fn set_bsy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.eop() != 0 { try!(write!(f, " eop"))}
        if self.wrprterr() != 0 { try!(write!(f, " wrprterr"))}
        if self.pgerr() != 0 { try!(write!(f, " pgerr"))}
        if self.bsy() != 0 { try!(write!(f, " bsy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Programming"]
    #[inline] pub fn pg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Programming"]
    #[inline] pub fn test_pg(&self) -> bool {
        self.pg() != 0
    }

    #[doc="Programming"]
    #[inline] pub fn set_pg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Page Erase"]
    #[inline] pub fn per(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Page Erase"]
    #[inline] pub fn test_per(&self) -> bool {
        self.per() != 0
    }

    #[doc="Page Erase"]
    #[inline] pub fn set_per<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Mass Erase"]
    #[inline] pub fn mer(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Mass Erase"]
    #[inline] pub fn test_mer(&self) -> bool {
        self.mer() != 0
    }

    #[doc="Mass Erase"]
    #[inline] pub fn set_mer<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Option byte programming"]
    #[inline] pub fn optpg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Option byte programming"]
    #[inline] pub fn test_optpg(&self) -> bool {
        self.optpg() != 0
    }

    #[doc="Option byte programming"]
    #[inline] pub fn set_optpg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Option byte erase"]
    #[inline] pub fn opter(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Option byte erase"]
    #[inline] pub fn test_opter(&self) -> bool {
        self.opter() != 0
    }

    #[doc="Option byte erase"]
    #[inline] pub fn set_opter<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Start"]
    #[inline] pub fn strt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Start"]
    #[inline] pub fn test_strt(&self) -> bool {
        self.strt() != 0
    }

    #[doc="Start"]
    #[inline] pub fn set_strt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Lock"]
    #[inline] pub fn lock(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Lock"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Lock"]
    #[inline] pub fn set_lock<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Option bytes write enable"]
    #[inline] pub fn optwre(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Option bytes write enable"]
    #[inline] pub fn test_optwre(&self) -> bool {
        self.optwre() != 0
    }

    #[doc="Option bytes write enable"]
    #[inline] pub fn set_optwre<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Error interrupt enable"]
    #[inline] pub fn errie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Error interrupt enable"]
    #[inline] pub fn test_errie(&self) -> bool {
        self.errie() != 0
    }

    #[doc="Error interrupt enable"]
    #[inline] pub fn set_errie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="End of operation interrupt enable"]
    #[inline] pub fn eopie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="End of operation interrupt enable"]
    #[inline] pub fn test_eopie(&self) -> bool {
        self.eopie() != 0
    }

    #[doc="End of operation interrupt enable"]
    #[inline] pub fn set_eopie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
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
        if self.pg() != 0 { try!(write!(f, " pg"))}
        if self.per() != 0 { try!(write!(f, " per"))}
        if self.mer() != 0 { try!(write!(f, " mer"))}
        if self.optpg() != 0 { try!(write!(f, " optpg"))}
        if self.opter() != 0 { try!(write!(f, " opter"))}
        if self.strt() != 0 { try!(write!(f, " strt"))}
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.optwre() != 0 { try!(write!(f, " optwre"))}
        if self.errie() != 0 { try!(write!(f, " errie"))}
        if self.eopie() != 0 { try!(write!(f, " eopie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ar(pub u32);
impl Ar {
    #[doc="Flash Address"]
    #[inline] pub fn far(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Flash Address"]
    #[inline] pub fn test_far(&self) -> bool {
        self.far() != 0
    }

    #[doc="Flash Address"]
    #[inline] pub fn set_far<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ar {
    #[inline]
    fn from(other: u32) -> Self {
         Ar(other)
    }
}

impl ::core::fmt::Display for Ar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Option byte register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Obr(pub u32);
impl Obr {
    #[doc="Option byte error"]
    #[inline] pub fn opterr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Option byte error"]
    #[inline] pub fn test_opterr(&self) -> bool {
        self.opterr() != 0
    }

    #[doc="Option byte error"]
    #[inline] pub fn set_opterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Read protection"]
    #[inline] pub fn rdprt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Read protection"]
    #[inline] pub fn test_rdprt(&self) -> bool {
        self.rdprt() != 0
    }

    #[doc="Read protection"]
    #[inline] pub fn set_rdprt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="WDG_SW"]
    #[inline] pub fn wdg_sw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="WDG_SW"]
    #[inline] pub fn test_wdg_sw(&self) -> bool {
        self.wdg_sw() != 0
    }

    #[doc="WDG_SW"]
    #[inline] pub fn set_wdg_sw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="nRST_STOP"]
    #[inline] pub fn nrst_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="nRST_STOP"]
    #[inline] pub fn test_nrst_stop(&self) -> bool {
        self.nrst_stop() != 0
    }

    #[doc="nRST_STOP"]
    #[inline] pub fn set_nrst_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="nRST_STDBY"]
    #[inline] pub fn nrst_stdby(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="nRST_STDBY"]
    #[inline] pub fn test_nrst_stdby(&self) -> bool {
        self.nrst_stdby() != 0
    }

    #[doc="nRST_STDBY"]
    #[inline] pub fn set_nrst_stdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data0"]
    #[inline] pub fn data0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0xff) as u8) } // [17:10]
    }

    #[doc="Data0"]
    #[inline] pub fn test_data0(&self) -> bool {
        self.data0() != 0
    }

    #[doc="Data0"]
    #[inline] pub fn set_data0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Data1"]
    #[inline] pub fn data1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0xff) as u8) } // [25:18]
    }

    #[doc="Data1"]
    #[inline] pub fn test_data1(&self) -> bool {
        self.data1() != 0
    }

    #[doc="Data1"]
    #[inline] pub fn set_data1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 18);
        self.0 |= value << 18;
        self
    }

}

impl From<u32> for Obr {
    #[inline]
    fn from(other: u32) -> Self {
         Obr(other)
    }
}

impl ::core::fmt::Display for Obr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Obr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.opterr() != 0 { try!(write!(f, " opterr"))}
        if self.rdprt() != 0 { try!(write!(f, " rdprt"))}
        if self.wdg_sw() != 0 { try!(write!(f, " wdg_sw"))}
        if self.nrst_stop() != 0 { try!(write!(f, " nrst_stop"))}
        if self.nrst_stdby() != 0 { try!(write!(f, " nrst_stdby"))}
        if self.data0() != 0 { try!(write!(f, " data0=0x{:x}", self.data0()))}
        if self.data1() != 0 { try!(write!(f, " data1=0x{:x}", self.data1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Write protection register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wrpr(pub u32);
impl Wrpr {
    #[doc="Write protect"]
    #[inline] pub fn wrp(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Write protect"]
    #[inline] pub fn test_wrp(&self) -> bool {
        self.wrp() != 0
    }

    #[doc="Write protect"]
    #[inline] pub fn set_wrp<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Wrpr {
    #[inline]
    fn from(other: u32) -> Self {
         Wrpr(other)
    }
}

impl ::core::fmt::Display for Wrpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wrpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}


