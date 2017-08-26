#[allow(unused_imports)] use bobbin_common::*;

periph!(WatchdogPeriph, WATCHDOG0, Watchdog0, 0x40000000);
periph!(WatchdogPeriph, WATCHDOG1, Watchdog1, 0x40001000);




pub trait WatchdogPeriph : Base {
#[doc="Get the *const pointer for the LOAD register."]
   #[inline] fn load_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Get the *mut pointer for the LOAD register."]
   #[inline] fn load_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Read the LOAD register."]
   #[inline] fn load(&self) -> Load { 
      unsafe {
         Load(::core::ptr::read_volatile((self.base() + 0x0) as *const u32))
      }
   }
#[doc="Write the LOAD register."]
   #[inline] fn set_load<F: FnOnce(Load) -> Load>(&self, f: F) -> &Self {
      let value = f(Load(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the LOAD register."]
   #[inline] fn with_load<F: FnOnce(Load) -> Load>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Load(::core::ptr::read_volatile((self.base() + 0x0) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the VALUE register."]
   #[inline] fn value_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Get the *mut pointer for the VALUE register."]
   #[inline] fn value_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Read the VALUE register."]
   #[inline] fn value(&self) -> Value { 
      unsafe {
         Value(::core::ptr::read_volatile((self.base() + 0x4) as *const u32))
      }
   }
#[doc="Write the VALUE register."]
   #[inline] fn set_value<F: FnOnce(Value) -> Value>(&self, f: F) -> &Self {
      let value = f(Value(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the VALUE register."]
   #[inline] fn with_value<F: FnOnce(Value) -> Value>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Value(::core::ptr::read_volatile((self.base() + 0x4) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTL register."]
   #[inline] fn ctl_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Get the *mut pointer for the CTL register."]
   #[inline] fn ctl_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Read the CTL register."]
   #[inline] fn ctl(&self) -> Ctl { 
      unsafe {
         Ctl(::core::ptr::read_volatile((self.base() + 0x8) as *const u32))
      }
   }
#[doc="Write the CTL register."]
   #[inline] fn set_ctl<F: FnOnce(Ctl) -> Ctl>(&self, f: F) -> &Self {
      let value = f(Ctl(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CTL register."]
   #[inline] fn with_ctl<F: FnOnce(Ctl) -> Ctl>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ctl(::core::ptr::read_volatile((self.base() + 0x8) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x8) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the ICR register."]
   #[inline] fn icr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xc)
   }
#[doc="Get the *mut pointer for the ICR register."]
   #[inline] fn icr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xc)
   }
#[doc="Write the ICR register."]
   #[inline] fn set_icr<F: FnOnce(Icr) -> Icr>(&self, f: F) -> &Self {
      let value = f(Icr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RIS register."]
   #[inline] fn ris_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x10)
   }
#[doc="Get the *mut pointer for the RIS register."]
   #[inline] fn ris_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x10)
   }
#[doc="Read the RIS register."]
   #[inline] fn ris(&self) -> Ris { 
      unsafe {
         Ris(::core::ptr::read_volatile((self.base() + 0x10) as *const u32))
      }
   }
#[doc="Write the RIS register."]
   #[inline] fn set_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
      let value = f(Ris(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x10) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RIS register."]
   #[inline] fn with_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ris(::core::ptr::read_volatile((self.base() + 0x10) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x10) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MIS register."]
   #[inline] fn mis_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x14)
   }
#[doc="Get the *mut pointer for the MIS register."]
   #[inline] fn mis_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x14)
   }
#[doc="Read the MIS register."]
   #[inline] fn mis(&self) -> Mis { 
      unsafe {
         Mis(::core::ptr::read_volatile((self.base() + 0x14) as *const u32))
      }
   }
#[doc="Write the MIS register."]
   #[inline] fn set_mis<F: FnOnce(Mis) -> Mis>(&self, f: F) -> &Self {
      let value = f(Mis(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x14) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MIS register."]
   #[inline] fn with_mis<F: FnOnce(Mis) -> Mis>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Mis(::core::ptr::read_volatile((self.base() + 0x14) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x14) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TEST register."]
   #[inline] fn test_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x418)
   }
#[doc="Get the *mut pointer for the TEST register."]
   #[inline] fn test_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x418)
   }
#[doc="Read the TEST register."]
   #[inline] fn test(&self) -> Test { 
      unsafe {
         Test(::core::ptr::read_volatile((self.base() + 0x418) as *const u32))
      }
   }
#[doc="Write the TEST register."]
   #[inline] fn set_test<F: FnOnce(Test) -> Test>(&self, f: F) -> &Self {
      let value = f(Test(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x418) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the TEST register."]
   #[inline] fn with_test<F: FnOnce(Test) -> Test>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Test(::core::ptr::read_volatile((self.base() + 0x418) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x418) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the LOCK register."]
   #[inline] fn lock_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xc00)
   }
#[doc="Get the *mut pointer for the LOCK register."]
   #[inline] fn lock_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xc00)
   }
#[doc="Read the LOCK register."]
   #[inline] fn lock(&self) -> Lock { 
      unsafe {
         Lock(::core::ptr::read_volatile((self.base() + 0xc00) as *const u32))
      }
   }
#[doc="Write the LOCK register."]
   #[inline] fn set_lock<F: FnOnce(Lock) -> Lock>(&self, f: F) -> &Self {
      let value = f(Lock(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc00) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the LOCK register."]
   #[inline] fn with_lock<F: FnOnce(Lock) -> Lock>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Lock(::core::ptr::read_volatile((self.base() + 0xc00) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc00) as *mut u32, value.0);
      }
      self
   }

}

#[doc="Watchdog Load"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Load(pub u32);
impl Load {
#[doc="Watchdog Load Value"]
   #[inline] pub fn load(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Value(pub u32);
impl Value {
#[doc="Watchdog Value"]
   #[inline] pub fn value(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctl(pub u32);
impl Ctl {
#[doc="Watchdog Interrupt Enable"]
   #[inline] pub fn inten(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
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
   #[inline] pub fn set_wrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
#[doc="Watchdog Interrupt Clear"]
   #[inline] pub fn icr(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ris(pub u32);
impl Ris {
#[doc="Watchdog Raw Interrupt Status"]
   #[inline] pub fn wdtris(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mis(pub u32);
impl Mis {
#[doc="Watchdog Masked Interrupt Status"]
   #[inline] pub fn wdtmis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Test(pub u32);
impl Test {
#[doc="Watchdog Stall Enable"]
   #[inline] pub fn stall(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Lock(pub u32);
impl Lock {
#[doc="Watchdog Lock"]
   #[inline] pub fn lock(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
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
