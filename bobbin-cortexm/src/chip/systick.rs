//! System Timer, SysTick
#[allow(unused_imports)] use bobbin_common::bits;
pub const SYSTICK: Systick = Systick(0xe000e000);

#[doc="System Timer, SysTick"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Systick(pub u32);
impl Systick {
#[doc="Get the *const pointer for the CSR register."]
  #[inline] pub fn csr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the CSR register."]
  #[inline] pub fn csr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the CSR register."]
  #[inline] pub fn csr(&self) -> Csr { 
     unsafe {
        Csr(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the CSR register."]
  #[inline] pub fn set_csr(&self, value: Csr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CSR register."]
  #[inline] pub fn with_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Self {
     let tmp = self.csr();
     self.set_csr(f(tmp))
  }

#[doc="Get the *const pointer for the RVR register."]
  #[inline] pub fn rvr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the RVR register."]
  #[inline] pub fn rvr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Read the RVR register."]
  #[inline] pub fn rvr(&self) -> Rvr { 
     unsafe {
        Rvr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
#[doc="Write the RVR register."]
  #[inline] pub fn set_rvr(&self, value: Rvr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the RVR register."]
  #[inline] pub fn with_rvr<F: FnOnce(Rvr) -> Rvr>(&self, f: F) -> &Self {
     let tmp = self.rvr();
     self.set_rvr(f(tmp))
  }

#[doc="Get the *const pointer for the CVR register."]
  #[inline] pub fn cvr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
#[doc="Get the *mut pointer for the CVR register."]
  #[inline] pub fn cvr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
#[doc="Read the CVR register."]
  #[inline] pub fn cvr(&self) -> Cvr { 
     unsafe {
        Cvr(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
#[doc="Write the CVR register."]
  #[inline] pub fn set_cvr(&self, value: Cvr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CVR register."]
  #[inline] pub fn with_cvr<F: FnOnce(Cvr) -> Cvr>(&self, f: F) -> &Self {
     let tmp = self.cvr();
     self.set_cvr(f(tmp))
  }

#[doc="Get the *const pointer for the CALIB register."]
  #[inline] pub fn calib_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the CALIB register."]
  #[inline] pub fn calib_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Read the CALIB register."]
  #[inline] pub fn calib(&self) -> Calib { 
     unsafe {
        Calib(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
#[doc="Write the CALIB register."]
  #[inline] pub fn set_calib(&self, value: Calib) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CALIB register."]
  #[inline] pub fn with_calib<F: FnOnce(Calib) -> Calib>(&self, f: F) -> &Self {
     let tmp = self.calib();
     self.set_calib(f(tmp))
  }

}

#[doc="Control and Status Register"]
#[derive(PartialEq, Eq)]
pub struct Csr(pub u32);
impl Csr {
#[doc="Returns 1 if timer counted to 0 since last time this was read."]
  #[inline] pub fn countflag(&self) -> bits::B1 {
     (((self.0 as u32) >> 16) & 0x1).into() // [16]
  }
#[doc="Returns 1 if timer counted to 0 since last time this was read."]
  #[inline] pub fn set_countflag<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Indicates the clock source: 0 = external clock, 1 = processor clock."]
  #[inline] pub fn clksource(&self) -> bits::B1 {
     (((self.0 as u32) >> 2) & 0x1).into() // [2]
  }
#[doc="Indicates the clock source: 0 = external clock, 1 = processor clock."]
  #[inline] pub fn set_clksource<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Enables SysTick exception request: 0 = counting down to zero does not assert the SysTick exception request, 1 = counting down to zero asserts the SysTick exception request."]
  #[inline] pub fn tickint(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="Enables SysTick exception request: 0 = counting down to zero does not assert the SysTick exception request, 1 = counting down to zero asserts the SysTick exception request."]
  #[inline] pub fn set_tickint<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Enables the counter: 0 = counter disabled, 1 = counter enabled."]
  #[inline] pub fn enable(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Enables the counter: 0 = counter disabled, 1 = counter enabled."]
  #[inline] pub fn set_enable<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Csr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Csr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.countflag() != 0 { try!(write!(f, " countflag"))}
      if self.clksource() != 0 { try!(write!(f, " clksource"))}
      if self.tickint() != 0 { try!(write!(f, " tickint"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Reload Value Register"]
#[derive(PartialEq, Eq)]
pub struct Rvr(pub u32);
impl Rvr {
#[doc="Value to load into the SYST_CVR register when the counter is enabled and when it reaches 0"]
  #[inline] pub fn reload(&self) -> bits::B24 {
     (((self.0 as u32) >> 0) & 0xffffff).into() // [23:0]
  }
#[doc="Value to load into the SYST_CVR register when the counter is enabled and when it reaches 0"]
  #[inline] pub fn set_reload<V: Into<bits::B24>>(mut self, value: V) -> Self {
     let value: bits::B24 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xffffff) == 0);
     self.0 &= !(0xffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Rvr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rvr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.reload() != 0 { try!(write!(f, " reload=0x{:x}", self.reload()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Current Value Register"]
#[derive(PartialEq, Eq)]
pub struct Cvr(pub u32);
impl Cvr {
#[doc="Reads return the current value of the SysTick counter. A write of any value clears the field to 0, and also clears the SYST_CSR COUNTFLAG bit to 0."]
  #[inline] pub fn current(&self) -> bits::B24 {
     (((self.0 as u32) >> 0) & 0xffffff).into() // [23:0]
  }
#[doc="Reads return the current value of the SysTick counter. A write of any value clears the field to 0, and also clears the SYST_CSR COUNTFLAG bit to 0."]
  #[inline] pub fn set_current<V: Into<bits::B24>>(mut self, value: V) -> Self {
     let value: bits::B24 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xffffff) == 0);
     self.0 &= !(0xffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cvr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cvr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.current() != 0 { try!(write!(f, " current=0x{:x}", self.current()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Calibration Value Register"]
#[derive(PartialEq, Eq)]
pub struct Calib(pub u32);
impl Calib {
#[doc="Indicates whether the device provides a reference clock to the processor"]
  #[inline] pub fn noref(&self) -> bits::B1 {
     (((self.0 as u32) >> 31) & 0x1).into() // [31]
  }
#[doc="Indicates whether the device provides a reference clock to the processor"]
  #[inline] pub fn set_noref<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

#[doc="Indicates whether the TENMS value is exact: 0 = TENMS value is exact, 1 = TENMS value is inexact, or not given."]
  #[inline] pub fn skew(&self) -> bits::B1 {
     (((self.0 as u32) >> 30) & 0x1).into() // [30]
  }
#[doc="Indicates whether the TENMS value is exact: 0 = TENMS value is exact, 1 = TENMS value is inexact, or not given."]
  #[inline] pub fn set_skew<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Reload value for 10ms (100Hz) timing, subject to system clock skew errors."]
  #[inline] pub fn tenms(&self) -> bits::B24 {
     (((self.0 as u32) >> 0) & 0xffffff).into() // [23:0]
  }
#[doc="Reload value for 10ms (100Hz) timing, subject to system clock skew errors."]
  #[inline] pub fn set_tenms<V: Into<bits::B24>>(mut self, value: V) -> Self {
     let value: bits::B24 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xffffff) == 0);
     self.0 &= !(0xffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Calib {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Calib {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.noref() != 0 { try!(write!(f, " noref"))}
      if self.skew() != 0 { try!(write!(f, " skew"))}
      if self.tenms() != 0 { try!(write!(f, " tenms=0x{:x}", self.tenms()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

