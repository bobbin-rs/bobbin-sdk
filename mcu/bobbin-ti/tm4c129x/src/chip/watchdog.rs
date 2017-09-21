#[allow(unused_imports)] use bobbin_common::*;

periph!( WATCHDOG0, Watchdog0, _WATCHDOG0, WatchdogPeriph, 0x40000000);
periph!( WATCHDOG1, Watchdog1, _WATCHDOG1, WatchdogPeriph, 0x40001000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="WATCHDOG Peripheral"]
pub struct WatchdogPeriph(pub usize); 




impl WatchdogPeriph {
    #[doc="Get the *mut pointer for the LOAD register."]
    #[inline] pub fn load_mut(&self) -> *mut Load { 
        (self.0 + 0x0) as *mut Load
    }

    #[doc="Get the *const pointer for the LOAD register."]
    #[inline] pub fn load_ptr(&self) -> *const Load { 
           self.load_mut()
    }

    #[doc="Read the LOAD register."]
    #[inline] pub fn load(&self) -> Load { 
        unsafe {
            read_volatile(self.load_ptr())
        }
    }

    #[doc="Write the LOAD register."]
    #[inline] pub fn set_load<F: FnOnce(Load) -> Load>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.load_mut(), f(Load(0)));
        }
        self
    }

    #[doc="Modify the LOAD register."]
    #[inline] pub fn with_load<F: FnOnce(Load) -> Load>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.load_mut(), f(self.load()));
        }
        self
    }

    #[doc="Get the *mut pointer for the VALUE register."]
    #[inline] pub fn value_mut(&self) -> *mut Value { 
        (self.0 + 0x4) as *mut Value
    }

    #[doc="Get the *const pointer for the VALUE register."]
    #[inline] pub fn value_ptr(&self) -> *const Value { 
           self.value_mut()
    }

    #[doc="Read the VALUE register."]
    #[inline] pub fn value(&self) -> Value { 
        unsafe {
            read_volatile(self.value_ptr())
        }
    }

    #[doc="Write the VALUE register."]
    #[inline] pub fn set_value<F: FnOnce(Value) -> Value>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.value_mut(), f(Value(0)));
        }
        self
    }

    #[doc="Modify the VALUE register."]
    #[inline] pub fn with_value<F: FnOnce(Value) -> Value>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.value_mut(), f(self.value()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTL register."]
    #[inline] pub fn ctl_mut(&self) -> *mut Ctl { 
        (self.0 + 0x8) as *mut Ctl
    }

    #[doc="Get the *const pointer for the CTL register."]
    #[inline] pub fn ctl_ptr(&self) -> *const Ctl { 
           self.ctl_mut()
    }

    #[doc="Read the CTL register."]
    #[inline] pub fn ctl(&self) -> Ctl { 
        unsafe {
            read_volatile(self.ctl_ptr())
        }
    }

    #[doc="Write the CTL register."]
    #[inline] pub fn set_ctl<F: FnOnce(Ctl) -> Ctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctl_mut(), f(Ctl(0)));
        }
        self
    }

    #[doc="Modify the CTL register."]
    #[inline] pub fn with_ctl<F: FnOnce(Ctl) -> Ctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctl_mut(), f(self.ctl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ICR register."]
    #[inline] pub fn icr_mut(&self) -> *mut Icr { 
        (self.0 + 0xc) as *mut Icr
    }

    #[doc="Get the *const pointer for the ICR register."]
    #[inline] pub fn icr_ptr(&self) -> *const Icr { 
           self.icr_mut()
    }

    #[doc="Write the ICR register."]
    #[inline] pub fn set_icr<F: FnOnce(Icr) -> Icr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icr_mut(), f(Icr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the RIS register."]
    #[inline] pub fn ris_mut(&self) -> *mut Ris { 
        (self.0 + 0x10) as *mut Ris
    }

    #[doc="Get the *const pointer for the RIS register."]
    #[inline] pub fn ris_ptr(&self) -> *const Ris { 
           self.ris_mut()
    }

    #[doc="Read the RIS register."]
    #[inline] pub fn ris(&self) -> Ris { 
        unsafe {
            read_volatile(self.ris_ptr())
        }
    }

    #[doc="Write the RIS register."]
    #[inline] pub fn set_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ris_mut(), f(Ris(0)));
        }
        self
    }

    #[doc="Modify the RIS register."]
    #[inline] pub fn with_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ris_mut(), f(self.ris()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MIS register."]
    #[inline] pub fn mis_mut(&self) -> *mut Mis { 
        (self.0 + 0x14) as *mut Mis
    }

    #[doc="Get the *const pointer for the MIS register."]
    #[inline] pub fn mis_ptr(&self) -> *const Mis { 
           self.mis_mut()
    }

    #[doc="Read the MIS register."]
    #[inline] pub fn mis(&self) -> Mis { 
        unsafe {
            read_volatile(self.mis_ptr())
        }
    }

    #[doc="Write the MIS register."]
    #[inline] pub fn set_mis<F: FnOnce(Mis) -> Mis>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mis_mut(), f(Mis(0)));
        }
        self
    }

    #[doc="Modify the MIS register."]
    #[inline] pub fn with_mis<F: FnOnce(Mis) -> Mis>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mis_mut(), f(self.mis()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TEST register."]
    #[inline] pub fn test_mut(&self) -> *mut Test { 
        (self.0 + 0x418) as *mut Test
    }

    #[doc="Get the *const pointer for the TEST register."]
    #[inline] pub fn test_ptr(&self) -> *const Test { 
           self.test_mut()
    }

    #[doc="Read the TEST register."]
    #[inline] pub fn test(&self) -> Test { 
        unsafe {
            read_volatile(self.test_ptr())
        }
    }

    #[doc="Write the TEST register."]
    #[inline] pub fn set_test<F: FnOnce(Test) -> Test>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.test_mut(), f(Test(0)));
        }
        self
    }

    #[doc="Modify the TEST register."]
    #[inline] pub fn with_test<F: FnOnce(Test) -> Test>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.test_mut(), f(self.test()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LOCK register."]
    #[inline] pub fn lock_mut(&self) -> *mut Lock { 
        (self.0 + 0xc00) as *mut Lock
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

}

#[doc="Watchdog Load"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Load(pub u32);
impl Load {
    #[doc="Watchdog Load Value"]
    #[inline] pub fn load(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if LOAD != 0"]
    #[inline] pub fn test_load(&self) -> bool {
        self.load() != 0
    }

    #[doc="Sets the LOAD field."]
    #[inline] pub fn set_load<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Load {
    #[inline]
    fn from(other: u32) -> Self {
         Load(other)
    }
}

impl ::core::fmt::Display for Load {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Load {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Value(pub u32);
impl Value {
    #[doc="Watchdog Value"]
    #[inline] pub fn value(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Value {
    #[inline]
    fn from(other: u32) -> Self {
         Value(other)
    }
}

impl ::core::fmt::Display for Value {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Value {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctl(pub u32);
impl Ctl {
    #[doc="Watchdog Interrupt Enable"]
    #[inline] pub fn inten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if INTEN != 0"]
    #[inline] pub fn test_inten(&self) -> bool {
        self.inten() != 0
    }

    #[doc="Sets the INTEN field."]
    #[inline] pub fn set_inten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Watchdog Reset Enable"]
    #[inline] pub fn resen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RESEN != 0"]
    #[inline] pub fn test_resen(&self) -> bool {
        self.resen() != 0
    }

    #[doc="Sets the RESEN field."]
    #[inline] pub fn set_resen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Watchdog Interrupt Type"]
    #[inline] pub fn inttype(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if INTTYPE != 0"]
    #[inline] pub fn test_inttype(&self) -> bool {
        self.inttype() != 0
    }

    #[doc="Sets the INTTYPE field."]
    #[inline] pub fn set_inttype<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Write Complete"]
    #[inline] pub fn wrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if WRC != 0"]
    #[inline] pub fn test_wrc(&self) -> bool {
        self.wrc() != 0
    }

    #[doc="Sets the WRC field."]
    #[inline] pub fn set_wrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Ctl {
    #[inline]
    fn from(other: u32) -> Self {
         Ctl(other)
    }
}

impl ::core::fmt::Display for Ctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.inten() != 0 { try!(write!(f, " inten"))}
        if self.resen() != 0 { try!(write!(f, " resen"))}
        if self.inttype() != 0 { try!(write!(f, " inttype"))}
        if self.wrc() != 0 { try!(write!(f, " wrc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Interrupt Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc="Watchdog Interrupt Clear"]
    #[inline] pub fn icr(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ICR != 0"]
    #[inline] pub fn test_icr(&self) -> bool {
        self.icr() != 0
    }

    #[doc="Sets the ICR field."]
    #[inline] pub fn set_icr<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Icr {
    #[inline]
    fn from(other: u32) -> Self {
         Icr(other)
    }
}

impl ::core::fmt::Display for Icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Raw Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ris(pub u32);
impl Ris {
    #[doc="Watchdog Raw Interrupt Status"]
    #[inline] pub fn wdtris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if WDTRIS != 0"]
    #[inline] pub fn test_wdtris(&self) -> bool {
        self.wdtris() != 0
    }

    #[doc="Sets the WDTRIS field."]
    #[inline] pub fn set_wdtris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ris {
    #[inline]
    fn from(other: u32) -> Self {
         Ris(other)
    }
}

impl ::core::fmt::Display for Ris {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ris {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wdtris() != 0 { try!(write!(f, " wdtris"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Masked Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mis(pub u32);
impl Mis {
    #[doc="Watchdog Masked Interrupt Status"]
    #[inline] pub fn wdtmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if WDTMIS != 0"]
    #[inline] pub fn test_wdtmis(&self) -> bool {
        self.wdtmis() != 0
    }

    #[doc="Sets the WDTMIS field."]
    #[inline] pub fn set_wdtmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mis {
    #[inline]
    fn from(other: u32) -> Self {
         Mis(other)
    }
}

impl ::core::fmt::Display for Mis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wdtmis() != 0 { try!(write!(f, " wdtmis"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Test"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Test(pub u32);
impl Test {
    #[doc="Watchdog Stall Enable"]
    #[inline] pub fn stall(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if STALL != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the STALL field."]
    #[inline] pub fn set_stall<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Test {
    #[inline]
    fn from(other: u32) -> Self {
         Test(other)
    }
}

impl ::core::fmt::Display for Test {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Test {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.stall() != 0 { try!(write!(f, " stall"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Lock"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lock(pub u32);
impl Lock {
    #[doc="Watchdog Lock"]
    #[inline] pub fn lock(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
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
        try!(write!(f, "]"));
        Ok(())
    }
}

