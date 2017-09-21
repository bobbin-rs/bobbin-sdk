//! Watchdog Timer
#[allow(unused_imports)] use bobbin_common::*;

periph!(WDT, Wdt, 0x40024000);

#[doc="Watchdog Timer"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wdt(pub usize);
impl Wdt {
    #[doc="Get the *mut pointer for the CFG register."]
    #[inline] pub fn cfg_mut(&self) -> *mut Cfg { 
        (self.0 + 0x0) as *mut Cfg
    }

    #[doc="Get the *const pointer for the CFG register."]
    #[inline] pub fn cfg_ptr(&self) -> *const Cfg { 
           self.cfg_mut()
    }

    #[doc="Read the CFG register."]
    #[inline] pub fn cfg(&self) -> Cfg { 
        unsafe {
            read_volatile(self.cfg_ptr())
        }
    }

    #[doc="Write the CFG register."]
    #[inline] pub fn set_cfg<F: FnOnce(Cfg) -> Cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfg_mut(), f(Cfg(0)));
        }
        self
    }

    #[doc="Modify the CFG register."]
    #[inline] pub fn with_cfg<F: FnOnce(Cfg) -> Cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfg_mut(), f(self.cfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RSTRT register."]
    #[inline] pub fn rstrt_mut(&self) -> *mut Rstrt { 
        (self.0 + 0x4) as *mut Rstrt
    }

    #[doc="Get the *const pointer for the RSTRT register."]
    #[inline] pub fn rstrt_ptr(&self) -> *const Rstrt { 
           self.rstrt_mut()
    }

    #[doc="Read the RSTRT register."]
    #[inline] pub fn rstrt(&self) -> Rstrt { 
        unsafe {
            read_volatile(self.rstrt_ptr())
        }
    }

    #[doc="Write the RSTRT register."]
    #[inline] pub fn set_rstrt<F: FnOnce(Rstrt) -> Rstrt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rstrt_mut(), f(Rstrt(0)));
        }
        self
    }

    #[doc="Modify the RSTRT register."]
    #[inline] pub fn with_rstrt<F: FnOnce(Rstrt) -> Rstrt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rstrt_mut(), f(self.rstrt()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LOCK register."]
    #[inline] pub fn lock_mut(&self) -> *mut Lock { 
        (self.0 + 0x8) as *mut Lock
    }

    #[doc="Get the *const pointer for the LOCK register."]
    #[inline] pub fn lock_ptr(&self) -> *const Lock { 
           self.lock_mut()
    }

    #[doc="Read the LOCK register."]
    #[inline] pub fn lock(&self) -> Lock { 
        unsafe {
            read_volatile(self.lock_ptr())
        }
    }

    #[doc="Write the LOCK register."]
    #[inline] pub fn set_lock<F: FnOnce(Lock) -> Lock>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lock_mut(), f(Lock(0)));
        }
        self
    }

    #[doc="Modify the LOCK register."]
    #[inline] pub fn with_lock<F: FnOnce(Lock) -> Lock>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lock_mut(), f(self.lock()));
        }
        self
    }

    #[doc="Get the *mut pointer for the COUNT register."]
    #[inline] pub fn count_mut(&self) -> *mut Count { 
        (self.0 + 0xc) as *mut Count
    }

    #[doc="Get the *const pointer for the COUNT register."]
    #[inline] pub fn count_ptr(&self) -> *const Count { 
           self.count_mut()
    }

    #[doc="Read the COUNT register."]
    #[inline] pub fn count(&self) -> Count { 
        unsafe {
            read_volatile(self.count_ptr())
        }
    }

    #[doc="Write the COUNT register."]
    #[inline] pub fn set_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.count_mut(), f(Count(0)));
        }
        self
    }

    #[doc="Modify the COUNT register."]
    #[inline] pub fn with_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.count_mut(), f(self.count()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTEN register."]
    #[inline] pub fn inten_mut(&self) -> *mut Inten { 
        (self.0 + 0x200) as *mut Inten
    }

    #[doc="Get the *const pointer for the INTEN register."]
    #[inline] pub fn inten_ptr(&self) -> *const Inten { 
           self.inten_mut()
    }

    #[doc="Read the INTEN register."]
    #[inline] pub fn inten(&self) -> Inten { 
        unsafe {
            read_volatile(self.inten_ptr())
        }
    }

    #[doc="Write the INTEN register."]
    #[inline] pub fn set_inten<F: FnOnce(Inten) -> Inten>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.inten_mut(), f(Inten(0)));
        }
        self
    }

    #[doc="Modify the INTEN register."]
    #[inline] pub fn with_inten<F: FnOnce(Inten) -> Inten>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.inten_mut(), f(self.inten()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTSTAT register."]
    #[inline] pub fn intstat_mut(&self) -> *mut Intstat { 
        (self.0 + 0x204) as *mut Intstat
    }

    #[doc="Get the *const pointer for the INTSTAT register."]
    #[inline] pub fn intstat_ptr(&self) -> *const Intstat { 
           self.intstat_mut()
    }

    #[doc="Read the INTSTAT register."]
    #[inline] pub fn intstat(&self) -> Intstat { 
        unsafe {
            read_volatile(self.intstat_ptr())
        }
    }

    #[doc="Write the INTSTAT register."]
    #[inline] pub fn set_intstat<F: FnOnce(Intstat) -> Intstat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intstat_mut(), f(Intstat(0)));
        }
        self
    }

    #[doc="Modify the INTSTAT register."]
    #[inline] pub fn with_intstat<F: FnOnce(Intstat) -> Intstat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intstat_mut(), f(self.intstat()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTCLR register."]
    #[inline] pub fn intclr_mut(&self) -> *mut Intclr { 
        (self.0 + 0x208) as *mut Intclr
    }

    #[doc="Get the *const pointer for the INTCLR register."]
    #[inline] pub fn intclr_ptr(&self) -> *const Intclr { 
           self.intclr_mut()
    }

    #[doc="Read the INTCLR register."]
    #[inline] pub fn intclr(&self) -> Intclr { 
        unsafe {
            read_volatile(self.intclr_ptr())
        }
    }

    #[doc="Write the INTCLR register."]
    #[inline] pub fn set_intclr<F: FnOnce(Intclr) -> Intclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intclr_mut(), f(Intclr(0)));
        }
        self
    }

    #[doc="Modify the INTCLR register."]
    #[inline] pub fn with_intclr<F: FnOnce(Intclr) -> Intclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intclr_mut(), f(self.intclr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTSET register."]
    #[inline] pub fn intset_mut(&self) -> *mut Intset { 
        (self.0 + 0x20c) as *mut Intset
    }

    #[doc="Get the *const pointer for the INTSET register."]
    #[inline] pub fn intset_ptr(&self) -> *const Intset { 
           self.intset_mut()
    }

    #[doc="Read the INTSET register."]
    #[inline] pub fn intset(&self) -> Intset { 
        unsafe {
            read_volatile(self.intset_ptr())
        }
    }

    #[doc="Write the INTSET register."]
    #[inline] pub fn set_intset<F: FnOnce(Intset) -> Intset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intset_mut(), f(Intset(0)));
        }
        self
    }

    #[doc="Modify the INTSET register."]
    #[inline] pub fn with_intset<F: FnOnce(Intset) -> Intset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intset_mut(), f(self.intset()));
        }
        self
    }

}

#[doc="Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc="Select the frequency for the WDT. All values not enumerated below are undefined."]
    #[inline] pub fn clksel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if CLKSEL != 0"]
    #[inline] pub fn test_clksel(&self) -> bool {
        self.clksel() != 0
    }

    #[doc="Sets the CLKSEL field."]
    #[inline] pub fn set_clksel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="This bitfield is the compare value for counter bits 7:0 to generate a watchdog interrupt."]
    #[inline] pub fn intval(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if INTVAL != 0"]
    #[inline] pub fn test_intval(&self) -> bool {
        self.intval() != 0
    }

    #[doc="Sets the INTVAL field."]
    #[inline] pub fn set_intval<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset."]
    #[inline] pub fn resval(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if RESVAL != 0"]
    #[inline] pub fn test_resval(&self) -> bool {
        self.resval() != 0
    }

    #[doc="Sets the RESVAL field."]
    #[inline] pub fn set_resval<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="This bitfield enables the WDT reset."]
    #[inline] pub fn resen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RESEN != 0"]
    #[inline] pub fn test_resen(&self) -> bool {
        self.resen() != 0
    }

    #[doc="Sets the RESEN field."]
    #[inline] pub fn set_resen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This bitfield enables the WDT interrupt. Note : This bit must be set before the interrupt status bit will reflect a watchdog timer expiration. The IER interrupt register must also be enabled for a WDT interrupt to be sent to the NVIC."]
    #[inline] pub fn inten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if INTEN != 0"]
    #[inline] pub fn test_inten(&self) -> bool {
        self.inten() != 0
    }

    #[doc="Sets the INTEN field."]
    #[inline] pub fn set_inten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bitfield enables the WDT."]
    #[inline] pub fn wdten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if WDTEN != 0"]
    #[inline] pub fn test_wdten(&self) -> bool {
        self.wdten() != 0
    }

    #[doc="Sets the WDTEN field."]
    #[inline] pub fn set_wdten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cfg {
    #[inline]
    fn from(other: u32) -> Self {
         Cfg(other)
    }
}

impl ::core::fmt::Display for Cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clksel() != 0 { try!(write!(f, " clksel=0x{:x}", self.clksel()))}
        if self.intval() != 0 { try!(write!(f, " intval=0x{:x}", self.intval()))}
        if self.resval() != 0 { try!(write!(f, " resval=0x{:x}", self.resval()))}
        if self.resen() != 0 { try!(write!(f, " resen"))}
        if self.inten() != 0 { try!(write!(f, " inten"))}
        if self.wdten() != 0 { try!(write!(f, " wdten"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Restart the watchdog timer"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rstrt(pub u32);
impl Rstrt {
    #[doc="Writing 0xB2 to WDTRSTRT restarts the watchdog timer."]
    #[inline] pub fn rstrt(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if RSTRT != 0"]
    #[inline] pub fn test_rstrt(&self) -> bool {
        self.rstrt() != 0
    }

    #[doc="Sets the RSTRT field."]
    #[inline] pub fn set_rstrt<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rstrt {
    #[inline]
    fn from(other: u32) -> Self {
         Rstrt(other)
    }
}

impl ::core::fmt::Display for Rstrt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rstrt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rstrt() != 0 { try!(write!(f, " rstrt=0x{:x}", self.rstrt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Locks the WDT"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lock(pub u32);
impl Lock {
    #[doc="Writing 0x3A locks the watchdog timer. Once locked, the WDTCFG reg cannot be written and WDTEN is set."]
    #[inline] pub fn lock(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Lock {
    #[inline]
    fn from(other: u32) -> Self {
         Lock(other)
    }
}

impl ::core::fmt::Display for Lock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock=0x{:x}", self.lock()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Current Counter Value for WDT"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Count(pub u32);
impl Count {
    #[doc="Read-Only current value of the WDT counter"]
    #[inline] pub fn count(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if COUNT != 0"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Sets the COUNT field."]
    #[inline] pub fn set_count<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Count {
    #[inline]
    fn from(other: u32) -> Self {
         Count(other)
    }
}

impl ::core::fmt::Display for Count {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Count {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="WDT Interrupt register: Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc="Watchdog Timer Interrupt."]
    #[inline] pub fn wdt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if WDT != 0"]
    #[inline] pub fn test_wdt(&self) -> bool {
        self.wdt() != 0
    }

    #[doc="Sets the WDT field."]
    #[inline] pub fn set_wdt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Inten {
    #[inline]
    fn from(other: u32) -> Self {
         Inten(other)
    }
}

impl ::core::fmt::Display for Inten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Inten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wdt() != 0 { try!(write!(f, " wdt"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="WDT Interrupt register: Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc="Watchdog Timer Interrupt."]
    #[inline] pub fn wdt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if WDT != 0"]
    #[inline] pub fn test_wdt(&self) -> bool {
        self.wdt() != 0
    }

    #[doc="Sets the WDT field."]
    #[inline] pub fn set_wdt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Intstat {
    #[inline]
    fn from(other: u32) -> Self {
         Intstat(other)
    }
}

impl ::core::fmt::Display for Intstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wdt() != 0 { try!(write!(f, " wdt"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="WDT Interrupt register: Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intclr(pub u32);
impl Intclr {
    #[doc="Watchdog Timer Interrupt."]
    #[inline] pub fn wdt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if WDT != 0"]
    #[inline] pub fn test_wdt(&self) -> bool {
        self.wdt() != 0
    }

    #[doc="Sets the WDT field."]
    #[inline] pub fn set_wdt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Intclr {
    #[inline]
    fn from(other: u32) -> Self {
         Intclr(other)
    }
}

impl ::core::fmt::Display for Intclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wdt() != 0 { try!(write!(f, " wdt"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="WDT Interrupt register: Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intset(pub u32);
impl Intset {
    #[doc="Watchdog Timer Interrupt."]
    #[inline] pub fn wdt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if WDT != 0"]
    #[inline] pub fn test_wdt(&self) -> bool {
        self.wdt() != 0
    }

    #[doc="Sets the WDT field."]
    #[inline] pub fn set_wdt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Intset {
    #[inline]
    fn from(other: u32) -> Self {
         Intset(other)
    }
}

impl ::core::fmt::Display for Intset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wdt() != 0 { try!(write!(f, " wdt"))}
        try!(write!(f, "]"));
        Ok(())
    }
}


