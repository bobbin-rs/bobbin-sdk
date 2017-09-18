//! MCU Reset Generator
#[allow(unused_imports)] use bobbin_common::*;

periph!(RSTGEN, Rstgen, 0x40000000);

#[doc="MCU Reset Generator"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rstgen(pub usize);
impl Rstgen {
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

    #[doc="Get the *mut pointer for the SWPOI register."]
    #[inline] pub fn swpoi_mut(&self) -> *mut Swpoi { 
        (self.0 + 0x4) as *mut Swpoi
    }

    #[doc="Get the *const pointer for the SWPOI register."]
    #[inline] pub fn swpoi_ptr(&self) -> *const Swpoi { 
           self.swpoi_mut()
    }

    #[doc="Read the SWPOI register."]
    #[inline] pub fn swpoi(&self) -> Swpoi { 
        unsafe {
            read_volatile(self.swpoi_ptr())
        }
    }

    #[doc="Write the SWPOI register."]
    #[inline] pub fn set_swpoi<F: FnOnce(Swpoi) -> Swpoi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.swpoi_mut(), f(Swpoi(0)));
        }
        self
    }

    #[doc="Modify the SWPOI register."]
    #[inline] pub fn with_swpoi<F: FnOnce(Swpoi) -> Swpoi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.swpoi_mut(), f(self.swpoi()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SWPOR register."]
    #[inline] pub fn swpor_mut(&self) -> *mut Swpor { 
        (self.0 + 0x8) as *mut Swpor
    }

    #[doc="Get the *const pointer for the SWPOR register."]
    #[inline] pub fn swpor_ptr(&self) -> *const Swpor { 
           self.swpor_mut()
    }

    #[doc="Read the SWPOR register."]
    #[inline] pub fn swpor(&self) -> Swpor { 
        unsafe {
            read_volatile(self.swpor_ptr())
        }
    }

    #[doc="Write the SWPOR register."]
    #[inline] pub fn set_swpor<F: FnOnce(Swpor) -> Swpor>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.swpor_mut(), f(Swpor(0)));
        }
        self
    }

    #[doc="Modify the SWPOR register."]
    #[inline] pub fn with_swpor<F: FnOnce(Swpor) -> Swpor>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.swpor_mut(), f(self.swpor()));
        }
        self
    }

    #[doc="Get the *mut pointer for the STAT register."]
    #[inline] pub fn stat_mut(&self) -> *mut Stat { 
        (self.0 + 0xc) as *mut Stat
    }

    #[doc="Get the *const pointer for the STAT register."]
    #[inline] pub fn stat_ptr(&self) -> *const Stat { 
           self.stat_mut()
    }

    #[doc="Read the STAT register."]
    #[inline] pub fn stat(&self) -> Stat { 
        unsafe {
            read_volatile(self.stat_ptr())
        }
    }

    #[doc="Write the STAT register."]
    #[inline] pub fn set_stat<F: FnOnce(Stat) -> Stat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stat_mut(), f(Stat(0)));
        }
        self
    }

    #[doc="Modify the STAT register."]
    #[inline] pub fn with_stat<F: FnOnce(Stat) -> Stat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stat_mut(), f(self.stat()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLRSTAT register."]
    #[inline] pub fn clrstat_mut(&self) -> *mut Clrstat { 
        (self.0 + 0x10) as *mut Clrstat
    }

    #[doc="Get the *const pointer for the CLRSTAT register."]
    #[inline] pub fn clrstat_ptr(&self) -> *const Clrstat { 
           self.clrstat_mut()
    }

    #[doc="Read the CLRSTAT register."]
    #[inline] pub fn clrstat(&self) -> Clrstat { 
        unsafe {
            read_volatile(self.clrstat_ptr())
        }
    }

    #[doc="Write the CLRSTAT register."]
    #[inline] pub fn set_clrstat<F: FnOnce(Clrstat) -> Clrstat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clrstat_mut(), f(Clrstat(0)));
        }
        self
    }

    #[doc="Modify the CLRSTAT register."]
    #[inline] pub fn with_clrstat<F: FnOnce(Clrstat) -> Clrstat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clrstat_mut(), f(self.clrstat()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TPIU_RST register."]
    #[inline] pub fn tpiu_rst_mut(&self) -> *mut TpiuRst { 
        (self.0 + 0x14) as *mut TpiuRst
    }

    #[doc="Get the *const pointer for the TPIU_RST register."]
    #[inline] pub fn tpiu_rst_ptr(&self) -> *const TpiuRst { 
           self.tpiu_rst_mut()
    }

    #[doc="Read the TPIU_RST register."]
    #[inline] pub fn tpiu_rst(&self) -> TpiuRst { 
        unsafe {
            read_volatile(self.tpiu_rst_ptr())
        }
    }

    #[doc="Write the TPIU_RST register."]
    #[inline] pub fn set_tpiu_rst<F: FnOnce(TpiuRst) -> TpiuRst>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tpiu_rst_mut(), f(TpiuRst(0)));
        }
        self
    }

    #[doc="Modify the TPIU_RST register."]
    #[inline] pub fn with_tpiu_rst<F: FnOnce(TpiuRst) -> TpiuRst>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tpiu_rst_mut(), f(self.tpiu_rst()));
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
    #[doc="Watchdog Timer Reset Enable. NOTE: The WDT module must also be configured for WDT reset."]
    #[inline] pub fn wdren(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if WDREN != 0"]
    #[inline] pub fn test_wdren(&self) -> bool {
        self.wdren() != 0
    }

    #[doc="Sets the WDREN field."]
    #[inline] pub fn set_wdren<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Brown out high (2.1v) reset enable."]
    #[inline] pub fn bodhren(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BODHREN != 0"]
    #[inline] pub fn test_bodhren(&self) -> bool {
        self.bodhren() != 0
    }

    #[doc="Sets the BODHREN field."]
    #[inline] pub fn set_bodhren<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.wdren() != 0 { try!(write!(f, " wdren"))}
        if self.bodhren() != 0 { try!(write!(f, " bodhren"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Software POI Reset"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Swpoi(pub u32);
impl Swpoi {
    #[doc="0x1B generates a software POI reset."]
    #[inline] pub fn swpoikey(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if SWPOIKEY != 0"]
    #[inline] pub fn test_swpoikey(&self) -> bool {
        self.swpoikey() != 0
    }

    #[doc="Sets the SWPOIKEY field."]
    #[inline] pub fn set_swpoikey<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Swpoi {
    #[inline]
    fn from(other: u32) -> Self {
         Swpoi(other)
    }
}

impl ::core::fmt::Display for Swpoi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Swpoi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swpoikey() != 0 { try!(write!(f, " swpoikey=0x{:x}", self.swpoikey()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Software POR Reset"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Swpor(pub u32);
impl Swpor {
    #[doc="0xD4 generates a software POR reset."]
    #[inline] pub fn swporkey(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if SWPORKEY != 0"]
    #[inline] pub fn test_swporkey(&self) -> bool {
        self.swporkey() != 0
    }

    #[doc="Sets the SWPORKEY field."]
    #[inline] pub fn set_swporkey<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Swpor {
    #[inline]
    fn from(other: u32) -> Self {
         Swpor(other)
    }
}

impl ::core::fmt::Display for Swpor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Swpor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swporkey() != 0 { try!(write!(f, " swporkey=0x{:x}", self.swporkey()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc="Reset was initiated by a Watchdog Timer Reset."]
    #[inline] pub fn wdrstat(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if WDRSTAT != 0"]
    #[inline] pub fn test_wdrstat(&self) -> bool {
        self.wdrstat() != 0
    }

    #[doc="Sets the WDRSTAT field."]
    #[inline] pub fn set_wdrstat<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Reset was a initiated by Debugger Reset."]
    #[inline] pub fn dbgrstat(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DBGRSTAT != 0"]
    #[inline] pub fn test_dbgrstat(&self) -> bool {
        self.dbgrstat() != 0
    }

    #[doc="Sets the DBGRSTAT field."]
    #[inline] pub fn set_dbgrstat<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Reset was a initiated by Software POI Reset."]
    #[inline] pub fn poirstat(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if POIRSTAT != 0"]
    #[inline] pub fn test_poirstat(&self) -> bool {
        self.poirstat() != 0
    }

    #[doc="Sets the POIRSTAT field."]
    #[inline] pub fn set_poirstat<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Reset was a initiated by SW POR or AIRCR Reset."]
    #[inline] pub fn swrstat(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SWRSTAT != 0"]
    #[inline] pub fn test_swrstat(&self) -> bool {
        self.swrstat() != 0
    }

    #[doc="Sets the SWRSTAT field."]
    #[inline] pub fn set_swrstat<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Reset was initiated by a Brown-Out Reset."]
    #[inline] pub fn borstat(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BORSTAT != 0"]
    #[inline] pub fn test_borstat(&self) -> bool {
        self.borstat() != 0
    }

    #[doc="Sets the BORSTAT field."]
    #[inline] pub fn set_borstat<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Reset was initiated by a Power-On Reset."]
    #[inline] pub fn porstat(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PORSTAT != 0"]
    #[inline] pub fn test_porstat(&self) -> bool {
        self.porstat() != 0
    }

    #[doc="Sets the PORSTAT field."]
    #[inline] pub fn set_porstat<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Reset was initiated by an External Reset."]
    #[inline] pub fn exrstat(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EXRSTAT != 0"]
    #[inline] pub fn test_exrstat(&self) -> bool {
        self.exrstat() != 0
    }

    #[doc="Sets the EXRSTAT field."]
    #[inline] pub fn set_exrstat<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Stat {
    #[inline]
    fn from(other: u32) -> Self {
         Stat(other)
    }
}

impl ::core::fmt::Display for Stat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Stat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wdrstat() != 0 { try!(write!(f, " wdrstat"))}
        if self.dbgrstat() != 0 { try!(write!(f, " dbgrstat"))}
        if self.poirstat() != 0 { try!(write!(f, " poirstat"))}
        if self.swrstat() != 0 { try!(write!(f, " swrstat"))}
        if self.borstat() != 0 { try!(write!(f, " borstat"))}
        if self.porstat() != 0 { try!(write!(f, " porstat"))}
        if self.exrstat() != 0 { try!(write!(f, " exrstat"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clear the status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clrstat(pub u32);
impl Clrstat {
    #[doc="Writing a 1 to this bit clears all bits in the RST_STAT."]
    #[inline] pub fn clrstat(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CLRSTAT != 0"]
    #[inline] pub fn test_clrstat(&self) -> bool {
        self.clrstat() != 0
    }

    #[doc="Sets the CLRSTAT field."]
    #[inline] pub fn set_clrstat<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clrstat {
    #[inline]
    fn from(other: u32) -> Self {
         Clrstat(other)
    }
}

impl ::core::fmt::Display for Clrstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clrstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clrstat() != 0 { try!(write!(f, " clrstat"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TPIU reset"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TpiuRst(pub u32);
impl TpiuRst {
    #[doc="Static reset for the TPIU. Write to \'1\' to assert reset to TPIU. Write to \'0\' to clear the reset."]
    #[inline] pub fn tpiurst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TPIURST != 0"]
    #[inline] pub fn test_tpiurst(&self) -> bool {
        self.tpiurst() != 0
    }

    #[doc="Sets the TPIURST field."]
    #[inline] pub fn set_tpiurst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for TpiuRst {
    #[inline]
    fn from(other: u32) -> Self {
         TpiuRst(other)
    }
}

impl ::core::fmt::Display for TpiuRst {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for TpiuRst {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tpiurst() != 0 { try!(write!(f, " tpiurst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reset Interrupt register: Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc="Enables an interrupt that triggers when VCC is below BODH level."]
    #[inline] pub fn bodh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BODH != 0"]
    #[inline] pub fn test_bodh(&self) -> bool {
        self.bodh() != 0
    }

    #[doc="Sets the BODH field."]
    #[inline] pub fn set_bodh<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.bodh() != 0 { try!(write!(f, " bodh"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reset Interrupt register: Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc="Enables an interrupt that triggers when VCC is below BODH level."]
    #[inline] pub fn bodh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BODH != 0"]
    #[inline] pub fn test_bodh(&self) -> bool {
        self.bodh() != 0
    }

    #[doc="Sets the BODH field."]
    #[inline] pub fn set_bodh<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.bodh() != 0 { try!(write!(f, " bodh"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reset Interrupt register: Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intclr(pub u32);
impl Intclr {
    #[doc="Enables an interrupt that triggers when VCC is below BODH level."]
    #[inline] pub fn bodh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BODH != 0"]
    #[inline] pub fn test_bodh(&self) -> bool {
        self.bodh() != 0
    }

    #[doc="Sets the BODH field."]
    #[inline] pub fn set_bodh<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.bodh() != 0 { try!(write!(f, " bodh"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reset Interrupt register: Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intset(pub u32);
impl Intset {
    #[doc="Enables an interrupt that triggers when VCC is below BODH level."]
    #[inline] pub fn bodh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BODH != 0"]
    #[inline] pub fn test_bodh(&self) -> bool {
        self.bodh() != 0
    }

    #[doc="Sets the BODH field."]
    #[inline] pub fn set_bodh<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.bodh() != 0 { try!(write!(f, " bodh"))}
        try!(write!(f, "]"));
        Ok(())
    }
}


