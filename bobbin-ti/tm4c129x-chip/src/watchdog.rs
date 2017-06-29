pub const WATCHDOG0: Watchdog0 = Periph(0x40000000, Watchdog0Id {});
pub const WATCHDOG1: Watchdog1 = Periph(0x40001000, Watchdog1Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="WATCHDOG Peripheral"]
pub struct Periph<T>(pub u32, pub T); 

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Watchdog0Id {}
pub type Watchdog0 = Periph<Watchdog0Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Watchdog1Id {}
pub type Watchdog1 = Periph<Watchdog1Id>;




impl<T> Periph<T> {
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
        Load(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the LOAD register."]
  #[inline] pub fn set_load(&self, value: Load) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the LOAD register."]
  #[inline] pub fn with_load<F: FnOnce(Load) -> Load>(&self, f: F) -> &Self {
     let tmp = self.load();
     self.set_load(f(tmp))
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
        Value(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the VALUE register."]
  #[inline] pub fn set_value(&self, value: Value) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the VALUE register."]
  #[inline] pub fn with_value<F: FnOnce(Value) -> Value>(&self, f: F) -> &Self {
     let tmp = self.value();
     self.set_value(f(tmp))
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
        Ctl(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the CTL register."]
  #[inline] pub fn set_ctl(&self, value: Ctl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CTL register."]
  #[inline] pub fn with_ctl<F: FnOnce(Ctl) -> Ctl>(&self, f: F) -> &Self {
     let tmp = self.ctl();
     self.set_ctl(f(tmp))
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
  #[inline] pub fn set_icr(&self, value: Icr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
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
        Ris(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the RIS register."]
  #[inline] pub fn set_ris(&self, value: Ris) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the RIS register."]
  #[inline] pub fn with_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
     let tmp = self.ris();
     self.set_ris(f(tmp))
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
        Mis(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
#[doc="Write the MIS register."]
  #[inline] pub fn set_mis(&self, value: Mis) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MIS register."]
  #[inline] pub fn with_mis<F: FnOnce(Mis) -> Mis>(&self, f: F) -> &Self {
     let tmp = self.mis();
     self.set_mis(f(tmp))
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
        Test(::core::ptr::read_volatile(((self.0 as usize) + 0x418) as *const u32))
     }
  }
#[doc="Write the TEST register."]
  #[inline] pub fn set_test(&self, value: Test) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x418) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TEST register."]
  #[inline] pub fn with_test<F: FnOnce(Test) -> Test>(&self, f: F) -> &Self {
     let tmp = self.test();
     self.set_test(f(tmp))
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
        Lock(::core::ptr::read_volatile(((self.0 as usize) + 0xc00) as *const u32))
     }
  }
#[doc="Write the LOCK register."]
  #[inline] pub fn set_lock(&self, value: Lock) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc00) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the LOCK register."]
  #[inline] pub fn with_lock<F: FnOnce(Lock) -> Lock>(&self, f: F) -> &Self {
     let tmp = self.lock();
     self.set_lock(f(tmp))
  }

}

#[doc="Watchdog Load"]
#[derive(PartialEq, Eq)]
pub struct Load(pub u32);
impl Load {
#[doc="Watchdog Load Value"]
  #[inline] pub fn load(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Watchdog Load Value"]
  #[inline] pub fn set_load(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Value(pub u32);
impl Value {
#[doc="Watchdog Value"]
  #[inline] pub fn value(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Watchdog Value"]
  #[inline] pub fn set_value(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ctl(pub u32);
impl Ctl {
#[doc="Watchdog Interrupt Enable"]
  #[inline] pub fn inten(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Watchdog Interrupt Enable"]
  #[inline] pub fn set_inten(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Watchdog Reset Enable"]
  #[inline] pub fn resen(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Watchdog Reset Enable"]
  #[inline] pub fn set_resen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Watchdog Interrupt Type"]
  #[inline] pub fn inttype(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Watchdog Interrupt Type"]
  #[inline] pub fn set_inttype(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Write Complete"]
  #[inline] pub fn wrc(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Write Complete"]
  #[inline] pub fn set_wrc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
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
#[derive(PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
#[doc="Watchdog Interrupt Clear"]
  #[inline] pub fn icr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Watchdog Interrupt Clear"]
  #[inline] pub fn set_icr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ris(pub u32);
impl Ris {
#[doc="Watchdog Raw Interrupt Status"]
  #[inline] pub fn wdtris(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Watchdog Raw Interrupt Status"]
  #[inline] pub fn set_wdtris(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Mis(pub u32);
impl Mis {
#[doc="Watchdog Masked Interrupt Status"]
  #[inline] pub fn wdtmis(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Watchdog Masked Interrupt Status"]
  #[inline] pub fn set_wdtmis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Test(pub u32);
impl Test {
#[doc="Watchdog Stall Enable"]
  #[inline] pub fn stall(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Watchdog Stall Enable"]
  #[inline] pub fn set_stall(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
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
#[derive(PartialEq, Eq)]
pub struct Lock(pub u32);
impl Lock {
#[doc="Watchdog Lock"]
  #[inline] pub fn lock(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Watchdog Lock"]
  #[inline] pub fn set_lock(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
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
