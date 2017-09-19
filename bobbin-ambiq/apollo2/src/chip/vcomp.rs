#[allow(unused_imports)] use bobbin_common::*;

periph!( VCOMP, Vcomp, _VCOMP, VcompPeriph, 0x4000c000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="VCOMP Peripheral"]
pub struct VcompPeriph(pub usize); 



impl VcompPeriph {
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

    #[doc="Get the *mut pointer for the STAT register."]
    #[inline] pub fn stat_mut(&self) -> *mut Stat { 
        (self.0 + 0x4) as *mut Stat
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

    #[doc="Get the *mut pointer for the PWDKEY register."]
    #[inline] pub fn pwdkey_mut(&self) -> *mut Pwdkey { 
        (self.0 + 0x8) as *mut Pwdkey
    }

    #[doc="Get the *const pointer for the PWDKEY register."]
    #[inline] pub fn pwdkey_ptr(&self) -> *const Pwdkey { 
           self.pwdkey_mut()
    }

    #[doc="Read the PWDKEY register."]
    #[inline] pub fn pwdkey(&self) -> Pwdkey { 
        unsafe {
            read_volatile(self.pwdkey_ptr())
        }
    }

    #[doc="Write the PWDKEY register."]
    #[inline] pub fn set_pwdkey<F: FnOnce(Pwdkey) -> Pwdkey>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pwdkey_mut(), f(Pwdkey(0)));
        }
        self
    }

    #[doc="Modify the PWDKEY register."]
    #[inline] pub fn with_pwdkey<F: FnOnce(Pwdkey) -> Pwdkey>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pwdkey_mut(), f(self.pwdkey()));
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
    #[doc="When the reference input NSEL is set to NSEL_DAC, this bitfield selects the voltage level for the negative input to the comparator."]
    #[inline] pub fn lvlsel(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if LVLSEL != 0"]
    #[inline] pub fn test_lvlsel(&self) -> bool {
        self.lvlsel() != 0
    }

    #[doc="Sets the LVLSEL field."]
    #[inline] pub fn set_lvlsel<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="This bitfield selects the negative input to the comparator."]
    #[inline] pub fn nsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if NSEL != 0"]
    #[inline] pub fn test_nsel(&self) -> bool {
        self.nsel() != 0
    }

    #[doc="Sets the NSEL field."]
    #[inline] pub fn set_nsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="This bitfield selects the positive input to the comparator."]
    #[inline] pub fn psel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if PSEL != 0"]
    #[inline] pub fn test_psel(&self) -> bool {
        self.psel() != 0
    }

    #[doc="Sets the PSEL field."]
    #[inline] pub fn set_psel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
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
        if self.lvlsel() != 0 { try!(write!(f, " lvlsel=0x{:x}", self.lvlsel()))}
        if self.nsel() != 0 { try!(write!(f, " nsel=0x{:x}", self.nsel()))}
        if self.psel() != 0 { try!(write!(f, " psel=0x{:x}", self.psel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc="This bit indicates the power down state of the voltage comparator."]
    #[inline] pub fn pwdstat(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PWDSTAT != 0"]
    #[inline] pub fn test_pwdstat(&self) -> bool {
        self.pwdstat() != 0
    }

    #[doc="Sets the PWDSTAT field."]
    #[inline] pub fn set_pwdstat<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit is 1 if the positive input of the comparator is greater than the negative input."]
    #[inline] pub fn cmpout(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMPOUT != 0"]
    #[inline] pub fn test_cmpout(&self) -> bool {
        self.cmpout() != 0
    }

    #[doc="Sets the CMPOUT field."]
    #[inline] pub fn set_cmpout<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.pwdstat() != 0 { try!(write!(f, " pwdstat"))}
        if self.cmpout() != 0 { try!(write!(f, " cmpout"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Key Register for Powering Down the Voltage Comparator"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pwdkey(pub u32);
impl Pwdkey {
    #[doc="Key register value."]
    #[inline] pub fn pwdkey(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if PWDKEY != 0"]
    #[inline] pub fn test_pwdkey(&self) -> bool {
        self.pwdkey() != 0
    }

    #[doc="Sets the PWDKEY field."]
    #[inline] pub fn set_pwdkey<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pwdkey {
    #[inline]
    fn from(other: u32) -> Self {
         Pwdkey(other)
    }
}

impl ::core::fmt::Display for Pwdkey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pwdkey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Voltage Comparator Interrupt registers: Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc="This bit is the vcompout high interrupt."]
    #[inline] pub fn outhi(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OUTHI != 0"]
    #[inline] pub fn test_outhi(&self) -> bool {
        self.outhi() != 0
    }

    #[doc="Sets the OUTHI field."]
    #[inline] pub fn set_outhi<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit is the vcompout low interrupt."]
    #[inline] pub fn outlow(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OUTLOW != 0"]
    #[inline] pub fn test_outlow(&self) -> bool {
        self.outlow() != 0
    }

    #[doc="Sets the OUTLOW field."]
    #[inline] pub fn set_outlow<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.outhi() != 0 { try!(write!(f, " outhi"))}
        if self.outlow() != 0 { try!(write!(f, " outlow"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Voltage Comparator Interrupt registers: Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc="This bit is the vcompout high interrupt."]
    #[inline] pub fn outhi(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OUTHI != 0"]
    #[inline] pub fn test_outhi(&self) -> bool {
        self.outhi() != 0
    }

    #[doc="Sets the OUTHI field."]
    #[inline] pub fn set_outhi<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit is the vcompout low interrupt."]
    #[inline] pub fn outlow(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OUTLOW != 0"]
    #[inline] pub fn test_outlow(&self) -> bool {
        self.outlow() != 0
    }

    #[doc="Sets the OUTLOW field."]
    #[inline] pub fn set_outlow<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.outhi() != 0 { try!(write!(f, " outhi"))}
        if self.outlow() != 0 { try!(write!(f, " outlow"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Voltage Comparator Interrupt registers: Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intclr(pub u32);
impl Intclr {
    #[doc="This bit is the vcompout high interrupt."]
    #[inline] pub fn outhi(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OUTHI != 0"]
    #[inline] pub fn test_outhi(&self) -> bool {
        self.outhi() != 0
    }

    #[doc="Sets the OUTHI field."]
    #[inline] pub fn set_outhi<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit is the vcompout low interrupt."]
    #[inline] pub fn outlow(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OUTLOW != 0"]
    #[inline] pub fn test_outlow(&self) -> bool {
        self.outlow() != 0
    }

    #[doc="Sets the OUTLOW field."]
    #[inline] pub fn set_outlow<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.outhi() != 0 { try!(write!(f, " outhi"))}
        if self.outlow() != 0 { try!(write!(f, " outlow"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Voltage Comparator Interrupt registers: Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intset(pub u32);
impl Intset {
    #[doc="This bit is the vcompout high interrupt."]
    #[inline] pub fn outhi(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OUTHI != 0"]
    #[inline] pub fn test_outhi(&self) -> bool {
        self.outhi() != 0
    }

    #[doc="Sets the OUTHI field."]
    #[inline] pub fn set_outhi<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit is the vcompout low interrupt."]
    #[inline] pub fn outlow(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OUTLOW != 0"]
    #[inline] pub fn test_outlow(&self) -> bool {
        self.outlow() != 0
    }

    #[doc="Sets the OUTLOW field."]
    #[inline] pub fn set_outlow<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.outhi() != 0 { try!(write!(f, " outhi"))}
        if self.outlow() != 0 { try!(write!(f, " outlow"))}
        try!(write!(f, "]"));
        Ok(())
    }
}


