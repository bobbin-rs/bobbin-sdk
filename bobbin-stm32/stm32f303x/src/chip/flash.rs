//! Flash
#[allow(unused_imports)] use bobbin_common::*;

periph!(FLASH, Flash, 0x40022000);

#[doc="Flash"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Flash(pub usize);
impl Flash {
    #[doc="Get the *const pointer for the ACR register."]
    #[inline] pub fn acr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x0) as *const u32
    }

    #[doc="Get the *mut pointer for the ACR register."]
    #[inline] pub fn acr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x0) as *mut u32
    }

    #[doc="Read the ACR register."]
    #[inline] pub fn acr(&self) -> Acr { 
        unsafe {
            Acr(read_volatile((self.0 + 0x0) as *const u32))
        }
    }

    #[doc="Write the ACR register."]
    #[inline] pub fn set_acr<F: FnOnce(Acr) -> Acr>(&self, f: F) -> &Self {
        let value = f(Acr(0));
        unsafe {
            write_volatile((self.0 + 0x0) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the ACR register."]
    #[inline] pub fn with_acr<F: FnOnce(Acr) -> Acr>(&self, f: F) -> &Self {
        let tmp = self.acr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x0) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the KEYR register."]
    #[inline] pub fn keyr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x4) as *const u32
    }

    #[doc="Get the *mut pointer for the KEYR register."]
    #[inline] pub fn keyr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x4) as *mut u32
    }

    #[doc="Write the KEYR register."]
    #[inline] pub fn set_keyr<F: FnOnce(Keyr) -> Keyr>(&self, f: F) -> &Self {
        let value = f(Keyr(0));
        unsafe {
            write_volatile((self.0 + 0x4) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the OPTKEYR register."]
    #[inline] pub fn optkeyr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x8) as *const u32
    }

    #[doc="Get the *mut pointer for the OPTKEYR register."]
    #[inline] pub fn optkeyr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x8) as *mut u32
    }

    #[doc="Write the OPTKEYR register."]
    #[inline] pub fn set_optkeyr<F: FnOnce(Optkeyr) -> Optkeyr>(&self, f: F) -> &Self {
        let value = f(Optkeyr(0));
        unsafe {
            write_volatile((self.0 + 0x8) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the SR register."]
    #[inline] pub fn sr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xc) as *const u32
    }

    #[doc="Get the *mut pointer for the SR register."]
    #[inline] pub fn sr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xc) as *mut u32
    }

    #[doc="Read the SR register."]
    #[inline] pub fn sr(&self) -> Sr { 
        unsafe {
            Sr(read_volatile((self.0 + 0xc) as *const u32))
        }
    }

    #[doc="Write the SR register."]
    #[inline] pub fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        let value = f(Sr(0));
        unsafe {
            write_volatile((self.0 + 0xc) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the SR register."]
    #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        let tmp = self.sr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xc) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the CR register."]
    #[inline] pub fn cr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x10) as *const u32
    }

    #[doc="Get the *mut pointer for the CR register."]
    #[inline] pub fn cr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x10) as *mut u32
    }

    #[doc="Read the CR register."]
    #[inline] pub fn cr(&self) -> Cr { 
        unsafe {
            Cr(read_volatile((self.0 + 0x10) as *const u32))
        }
    }

    #[doc="Write the CR register."]
    #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        let value = f(Cr(0));
        unsafe {
            write_volatile((self.0 + 0x10) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the CR register."]
    #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        let tmp = self.cr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x10) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the AR register."]
    #[inline] pub fn ar_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x14) as *const u32
    }

    #[doc="Get the *mut pointer for the AR register."]
    #[inline] pub fn ar_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x14) as *mut u32
    }

    #[doc="Write the AR register."]
    #[inline] pub fn set_ar<F: FnOnce(Ar) -> Ar>(&self, f: F) -> &Self {
        let value = f(Ar(0));
        unsafe {
            write_volatile((self.0 + 0x14) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the OBR register."]
    #[inline] pub fn obr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x1c) as *const u32
    }

    #[doc="Get the *mut pointer for the OBR register."]
    #[inline] pub fn obr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x1c) as *mut u32
    }

    #[doc="Read the OBR register."]
    #[inline] pub fn obr(&self) -> Obr { 
        unsafe {
            Obr(read_volatile((self.0 + 0x1c) as *const u32))
        }
    }

    #[doc="Get the *const pointer for the WRPR register."]
    #[inline] pub fn wrpr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x20) as *const u32
    }

    #[doc="Get the *mut pointer for the WRPR register."]
    #[inline] pub fn wrpr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x20) as *mut u32
    }

    #[doc="Read the WRPR register."]
    #[inline] pub fn wrpr(&self) -> Wrpr { 
        unsafe {
            Wrpr(read_volatile((self.0 + 0x20) as *const u32))
        }
    }

}

#[doc="Flash access control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Acr(pub u32);
impl Acr {
    #[doc="LATENCY"]
    #[inline] pub fn latency(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="LATENCY"]
    #[inline] pub fn test_latency(&self) -> bool {
        self.latency != 0
    }

    #[doc="LATENCY"]
    #[inline] pub fn set_latency<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="PRFTBE"]
    #[inline] pub fn prftbe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="PRFTBE"]
    #[inline] pub fn test_prftbe(&self) -> bool {
        self.prftbe != 0
    }

    #[doc="PRFTBE"]
    #[inline] pub fn set_prftbe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="PRFTBS"]
    #[inline] pub fn prftbs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="PRFTBS"]
    #[inline] pub fn test_prftbs(&self) -> bool {
        self.prftbs != 0
    }

    #[doc="PRFTBS"]
    #[inline] pub fn set_prftbs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
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
    #[doc="Flash Key"]
    #[inline] pub fn fkeyr(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Flash Key"]
    #[inline] pub fn test_fkeyr(&self) -> bool {
        self.fkeyr != 0
    }

    #[doc="Flash Key"]
    #[inline] pub fn set_fkeyr<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
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
    #[inline] pub fn optkeyr(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Option byte key"]
    #[inline] pub fn test_optkeyr(&self) -> bool {
        self.optkeyr != 0
    }

    #[doc="Option byte key"]
    #[inline] pub fn set_optkeyr<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
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

#[doc="Flash status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="End of operation"]
    #[inline] pub fn eop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="End of operation"]
    #[inline] pub fn test_eop(&self) -> bool {
        self.eop != 0
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
    #[inline] pub fn wrprt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Write protection error"]
    #[inline] pub fn test_wrprt(&self) -> bool {
        self.wrprt != 0
    }

    #[doc="Write protection error"]
    #[inline] pub fn set_wrprt<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        self.pgerr != 0
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
        self.bsy != 0
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

impl ::core::fmt::Display for Sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.eop() != 0 { try!(write!(f, " eop"))}
        if self.wrprt() != 0 { try!(write!(f, " wrprt"))}
        if self.pgerr() != 0 { try!(write!(f, " pgerr"))}
        if self.bsy() != 0 { try!(write!(f, " bsy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Force option byte loading"]
    #[inline] pub fn force_optload(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Force option byte loading"]
    #[inline] pub fn test_force_optload(&self) -> bool {
        self.force_optload != 0
    }

    #[doc="Force option byte loading"]
    #[inline] pub fn set_force_optload<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="End of operation interrupt enable"]
    #[inline] pub fn eopie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="End of operation interrupt enable"]
    #[inline] pub fn test_eopie(&self) -> bool {
        self.eopie != 0
    }

    #[doc="End of operation interrupt enable"]
    #[inline] pub fn set_eopie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Error interrupt enable"]
    #[inline] pub fn errie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Error interrupt enable"]
    #[inline] pub fn test_errie(&self) -> bool {
        self.errie != 0
    }

    #[doc="Error interrupt enable"]
    #[inline] pub fn set_errie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Option bytes write enable"]
    #[inline] pub fn optwre(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Option bytes write enable"]
    #[inline] pub fn test_optwre(&self) -> bool {
        self.optwre != 0
    }

    #[doc="Option bytes write enable"]
    #[inline] pub fn set_optwre<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Lock"]
    #[inline] pub fn lock(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Lock"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock != 0
    }

    #[doc="Lock"]
    #[inline] pub fn set_lock<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Start"]
    #[inline] pub fn strt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Start"]
    #[inline] pub fn test_strt(&self) -> bool {
        self.strt != 0
    }

    #[doc="Start"]
    #[inline] pub fn set_strt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Option byte erase"]
    #[inline] pub fn opter(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Option byte erase"]
    #[inline] pub fn test_opter(&self) -> bool {
        self.opter != 0
    }

    #[doc="Option byte erase"]
    #[inline] pub fn set_opter<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Option byte programming"]
    #[inline] pub fn optpg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Option byte programming"]
    #[inline] pub fn test_optpg(&self) -> bool {
        self.optpg != 0
    }

    #[doc="Option byte programming"]
    #[inline] pub fn set_optpg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Mass erase"]
    #[inline] pub fn mer(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Mass erase"]
    #[inline] pub fn test_mer(&self) -> bool {
        self.mer != 0
    }

    #[doc="Mass erase"]
    #[inline] pub fn set_mer<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Page erase"]
    #[inline] pub fn per(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Page erase"]
    #[inline] pub fn test_per(&self) -> bool {
        self.per != 0
    }

    #[doc="Page erase"]
    #[inline] pub fn set_per<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Programming"]
    #[inline] pub fn pg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Programming"]
    #[inline] pub fn test_pg(&self) -> bool {
        self.pg != 0
    }

    #[doc="Programming"]
    #[inline] pub fn set_pg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
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
        if self.force_optload() != 0 { try!(write!(f, " force_optload"))}
        if self.eopie() != 0 { try!(write!(f, " eopie"))}
        if self.errie() != 0 { try!(write!(f, " errie"))}
        if self.optwre() != 0 { try!(write!(f, " optwre"))}
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.strt() != 0 { try!(write!(f, " strt"))}
        if self.opter() != 0 { try!(write!(f, " opter"))}
        if self.optpg() != 0 { try!(write!(f, " optpg"))}
        if self.mer() != 0 { try!(write!(f, " mer"))}
        if self.per() != 0 { try!(write!(f, " per"))}
        if self.pg() != 0 { try!(write!(f, " pg"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ar(pub u32);
impl Ar {
    #[doc="Flash address"]
    #[inline] pub fn far(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Flash address"]
    #[inline] pub fn test_far(&self) -> bool {
        self.far != 0
    }

    #[doc="Flash address"]
    #[inline] pub fn set_far<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
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
        self.opterr != 0
    }

    #[doc="Option byte error"]
    #[inline] pub fn set_opterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Level 1 protection status"]
    #[inline] pub fn level1_prot(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Level 1 protection status"]
    #[inline] pub fn test_level1_prot(&self) -> bool {
        self.level1_prot != 0
    }

    #[doc="Level 1 protection status"]
    #[inline] pub fn set_level1_prot<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Level 2 protection status"]
    #[inline] pub fn level2_prot(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Level 2 protection status"]
    #[inline] pub fn test_level2_prot(&self) -> bool {
        self.level2_prot != 0
    }

    #[doc="Level 2 protection status"]
    #[inline] pub fn set_level2_prot<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="WDG_SW"]
    #[inline] pub fn wdg_sw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="WDG_SW"]
    #[inline] pub fn test_wdg_sw(&self) -> bool {
        self.wdg_sw != 0
    }

    #[doc="WDG_SW"]
    #[inline] pub fn set_wdg_sw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="nRST_STOP"]
    #[inline] pub fn nrst_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="nRST_STOP"]
    #[inline] pub fn test_nrst_stop(&self) -> bool {
        self.nrst_stop != 0
    }

    #[doc="nRST_STOP"]
    #[inline] pub fn set_nrst_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="nRST_STDBY"]
    #[inline] pub fn nrst_stdby(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="nRST_STDBY"]
    #[inline] pub fn test_nrst_stdby(&self) -> bool {
        self.nrst_stdby != 0
    }

    #[doc="nRST_STDBY"]
    #[inline] pub fn set_nrst_stdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="BOOT1"]
    #[inline] pub fn boot1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="BOOT1"]
    #[inline] pub fn test_boot1(&self) -> bool {
        self.boot1 != 0
    }

    #[doc="BOOT1"]
    #[inline] pub fn set_boot1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="VDDA_MONITOR"]
    #[inline] pub fn vdda_monitor(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="VDDA_MONITOR"]
    #[inline] pub fn test_vdda_monitor(&self) -> bool {
        self.vdda_monitor != 0
    }

    #[doc="VDDA_MONITOR"]
    #[inline] pub fn set_vdda_monitor<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="SRAM_PARITY_CHECK"]
    #[inline] pub fn sram_parity_check(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="SRAM_PARITY_CHECK"]
    #[inline] pub fn test_sram_parity_check(&self) -> bool {
        self.sram_parity_check != 0
    }

    #[doc="SRAM_PARITY_CHECK"]
    #[inline] pub fn set_sram_parity_check<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Data0"]
    #[inline] pub fn data0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Data0"]
    #[inline] pub fn test_data0(&self) -> bool {
        self.data0 != 0
    }

    #[doc="Data0"]
    #[inline] pub fn set_data0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Data1"]
    #[inline] pub fn data1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Data1"]
    #[inline] pub fn test_data1(&self) -> bool {
        self.data1 != 0
    }

    #[doc="Data1"]
    #[inline] pub fn set_data1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
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
        if self.level1_prot() != 0 { try!(write!(f, " level1_prot"))}
        if self.level2_prot() != 0 { try!(write!(f, " level2_prot"))}
        if self.wdg_sw() != 0 { try!(write!(f, " wdg_sw"))}
        if self.nrst_stop() != 0 { try!(write!(f, " nrst_stop"))}
        if self.nrst_stdby() != 0 { try!(write!(f, " nrst_stdby"))}
        if self.boot1() != 0 { try!(write!(f, " boot1"))}
        if self.vdda_monitor() != 0 { try!(write!(f, " vdda_monitor"))}
        if self.sram_parity_check() != 0 { try!(write!(f, " sram_parity_check"))}
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
        self.wrp != 0
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


