#[allow(unused_imports)] use bobbin_common::*;

periph!( WATCHDOG0, Watchdog0, _WATCHDOG0, WatchdogPeriph, 0x40000000);
periph!( WATCHDOG1, Watchdog1, _WATCHDOG1, WatchdogPeriph, 0x40001000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="WATCHDOG Peripheral"]
pub struct WatchdogPeriph(pub usize); 




impl WatchdogPeriph {
    #[doc="Get the *const pointer for the LOAD register."]
    #[inline] pub fn load_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x0) as *const u32
    }

    #[doc="Get the *mut pointer for the LOAD register."]
    #[inline] pub fn load_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x0) as *mut u32
    }

    #[doc="Read the LOAD register."]
    #[inline] pub fn load(&self) -> Load { 
        unsafe {
            Load(read_volatile((self.0 + 0x0) as *const u32))
        }
    }

    #[doc="Write the LOAD register."]
    #[inline] pub fn set_load<F: FnOnce(Load) -> Load>(&self, f: F) -> &Self {
        let value = f(Load(0));
        unsafe {
            write_volatile((self.0 + 0x0) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the LOAD register."]
    #[inline] pub fn with_load<F: FnOnce(Load) -> Load>(&self, f: F) -> &Self {
        let tmp = self.load();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x0) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the VALUE register."]
    #[inline] pub fn value_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x4) as *const u32
    }

    #[doc="Get the *mut pointer for the VALUE register."]
    #[inline] pub fn value_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x4) as *mut u32
    }

    #[doc="Read the VALUE register."]
    #[inline] pub fn value(&self) -> Value { 
        unsafe {
            Value(read_volatile((self.0 + 0x4) as *const u32))
        }
    }

    #[doc="Write the VALUE register."]
    #[inline] pub fn set_value<F: FnOnce(Value) -> Value>(&self, f: F) -> &Self {
        let value = f(Value(0));
        unsafe {
            write_volatile((self.0 + 0x4) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the VALUE register."]
    #[inline] pub fn with_value<F: FnOnce(Value) -> Value>(&self, f: F) -> &Self {
        let tmp = self.value();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x4) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the CTL register."]
    #[inline] pub fn ctl_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x8) as *const u32
    }

    #[doc="Get the *mut pointer for the CTL register."]
    #[inline] pub fn ctl_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x8) as *mut u32
    }

    #[doc="Read the CTL register."]
    #[inline] pub fn ctl(&self) -> Ctl { 
        unsafe {
            Ctl(read_volatile((self.0 + 0x8) as *const u32))
        }
    }

    #[doc="Write the CTL register."]
    #[inline] pub fn set_ctl<F: FnOnce(Ctl) -> Ctl>(&self, f: F) -> &Self {
        let value = f(Ctl(0));
        unsafe {
            write_volatile((self.0 + 0x8) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the CTL register."]
    #[inline] pub fn with_ctl<F: FnOnce(Ctl) -> Ctl>(&self, f: F) -> &Self {
        let tmp = self.ctl();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x8) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the ICR register."]
    #[inline] pub fn icr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xc) as *const u32
    }

    #[doc="Get the *mut pointer for the ICR register."]
    #[inline] pub fn icr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xc) as *mut u32
    }

    #[doc="Write the ICR register."]
    #[inline] pub fn set_icr<F: FnOnce(Icr) -> Icr>(&self, f: F) -> &Self {
        let value = f(Icr(0));
        unsafe {
            write_volatile((self.0 + 0xc) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the RIS register."]
    #[inline] pub fn ris_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x10) as *const u32
    }

    #[doc="Get the *mut pointer for the RIS register."]
    #[inline] pub fn ris_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x10) as *mut u32
    }

    #[doc="Read the RIS register."]
    #[inline] pub fn ris(&self) -> Ris { 
        unsafe {
            Ris(read_volatile((self.0 + 0x10) as *const u32))
        }
    }

    #[doc="Write the RIS register."]
    #[inline] pub fn set_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
        let value = f(Ris(0));
        unsafe {
            write_volatile((self.0 + 0x10) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the RIS register."]
    #[inline] pub fn with_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
        let tmp = self.ris();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x10) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the MIS register."]
    #[inline] pub fn mis_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x14) as *const u32
    }

    #[doc="Get the *mut pointer for the MIS register."]
    #[inline] pub fn mis_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x14) as *mut u32
    }

    #[doc="Read the MIS register."]
    #[inline] pub fn mis(&self) -> Mis { 
        unsafe {
            Mis(read_volatile((self.0 + 0x14) as *const u32))
        }
    }

    #[doc="Write the MIS register."]
    #[inline] pub fn set_mis<F: FnOnce(Mis) -> Mis>(&self, f: F) -> &Self {
        let value = f(Mis(0));
        unsafe {
            write_volatile((self.0 + 0x14) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the MIS register."]
    #[inline] pub fn with_mis<F: FnOnce(Mis) -> Mis>(&self, f: F) -> &Self {
        let tmp = self.mis();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x14) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the TEST register."]
    #[inline] pub fn test_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x418) as *const u32
    }

    #[doc="Get the *mut pointer for the TEST register."]
    #[inline] pub fn test_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x418) as *mut u32
    }

    #[doc="Read the TEST register."]
    #[inline] pub fn test(&self) -> Test { 
        unsafe {
            Test(read_volatile((self.0 + 0x418) as *const u32))
        }
    }

    #[doc="Write the TEST register."]
    #[inline] pub fn set_test<F: FnOnce(Test) -> Test>(&self, f: F) -> &Self {
        let value = f(Test(0));
        unsafe {
            write_volatile((self.0 + 0x418) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the TEST register."]
    #[inline] pub fn with_test<F: FnOnce(Test) -> Test>(&self, f: F) -> &Self {
        let tmp = self.test();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x418) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the LOCK register."]
    #[inline] pub fn lock_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xc00) as *const u32
    }

    #[doc="Get the *mut pointer for the LOCK register."]
    #[inline] pub fn lock_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xc00) as *mut u32
    }

    #[doc="Read the LOCK register."]
    #[inline] pub fn lock(&self) -> Lock { 
        unsafe {
            Lock(read_volatile((self.0 + 0xc00) as *const u32))
        }
    }

    #[doc="Write the LOCK register."]
    #[inline] pub fn set_lock<F: FnOnce(Lock) -> Lock>(&self, f: F) -> &Self {
        let value = f(Lock(0));
        unsafe {
            write_volatile((self.0 + 0xc00) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the LOCK register."]
    #[inline] pub fn with_lock<F: FnOnce(Lock) -> Lock>(&self, f: F) -> &Self {
        let tmp = self.lock();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xc00) as *mut u32, value.0);
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

    #[doc="Watchdog Load Value"]
    #[inline] pub fn test_load(&self) -> bool {
        self.load() != 0
    }

    #[doc="Watchdog Load Value"]
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

    #[doc="Watchdog Value"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Watchdog Value"]
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

    #[doc="Watchdog Interrupt Enable"]
    #[inline] pub fn test_inten(&self) -> bool {
        self.inten() != 0
    }

    #[doc="Watchdog Interrupt Enable"]
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

    #[doc="Watchdog Reset Enable"]
    #[inline] pub fn test_resen(&self) -> bool {
        self.resen() != 0
    }

    #[doc="Watchdog Reset Enable"]
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

    #[doc="Watchdog Interrupt Type"]
    #[inline] pub fn test_inttype(&self) -> bool {
        self.inttype() != 0
    }

    #[doc="Watchdog Interrupt Type"]
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

    #[doc="Write Complete"]
    #[inline] pub fn test_wrc(&self) -> bool {
        self.wrc() != 0
    }

    #[doc="Write Complete"]
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

    #[doc="Watchdog Interrupt Clear"]
    #[inline] pub fn test_icr(&self) -> bool {
        self.icr() != 0
    }

    #[doc="Watchdog Interrupt Clear"]
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

    #[doc="Watchdog Raw Interrupt Status"]
    #[inline] pub fn test_wdtris(&self) -> bool {
        self.wdtris() != 0
    }

    #[doc="Watchdog Raw Interrupt Status"]
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

    #[doc="Watchdog Masked Interrupt Status"]
    #[inline] pub fn test_wdtmis(&self) -> bool {
        self.wdtmis() != 0
    }

    #[doc="Watchdog Masked Interrupt Status"]
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

    #[doc="Watchdog Stall Enable"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Watchdog Stall Enable"]
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

    #[doc="Watchdog Lock"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Watchdog Lock"]
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


