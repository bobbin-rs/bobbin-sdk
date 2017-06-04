pub const SYSTICK: Systick = Systick(0xe000e000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Systick(pub u32);
impl Systick {
  #[inline]
  pub fn csr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
  #[inline]
  pub fn csr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
  #[inline]
  pub fn csr(&self) -> Csr { 
     unsafe {
       Csr(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
  #[inline]
  pub fn set_csr(&self, value: Csr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Self {
     let tmp = self.csr();
     self.set_csr(f(tmp))
  }

  #[inline]
  pub fn rvr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
  #[inline]
  pub fn rvr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
  #[inline]
  pub fn rvr(&self) -> Rvr { 
     unsafe {
       Rvr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
  #[inline]
  pub fn set_rvr(&self, value: Rvr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_rvr<F: FnOnce(Rvr) -> Rvr>(&self, f: F) -> &Self {
     let tmp = self.rvr();
     self.set_rvr(f(tmp))
  }

  #[inline]
  pub fn cvr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
  #[inline]
  pub fn cvr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
  #[inline]
  pub fn cvr(&self) -> Cvr { 
     unsafe {
       Cvr(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
  #[inline]
  pub fn set_cvr(&self, value: Cvr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_cvr<F: FnOnce(Cvr) -> Cvr>(&self, f: F) -> &Self {
     let tmp = self.cvr();
     self.set_cvr(f(tmp))
  }

  #[inline]
  pub fn calib_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
  #[inline]
  pub fn calib_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
  #[inline]
  pub fn calib(&self) -> Calib { 
     unsafe {
       Calib(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
  #[inline]
  pub fn set_calib(&self, value: Calib) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_calib<F: FnOnce(Calib) -> Calib>(&self, f: F) -> &Self {
     let tmp = self.calib();
     self.set_calib(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Csr(pub u32);
impl Csr {
  #[inline]
  pub fn countflag(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
  pub fn set_countflag(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn clksource(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_clksource(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn tickint(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_tickint(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn enable(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_enable(mut self, value: u32) -> Self {
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
#[derive(PartialEq, Eq)]
pub struct Rvr(pub u32);
impl Rvr {
  #[inline]
  pub fn reload(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffff // [23:0]
  }
  #[inline]
  pub fn set_reload(mut self, value: u32) -> Self {
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
#[derive(PartialEq, Eq)]
pub struct Cvr(pub u32);
impl Cvr {
  #[inline]
  pub fn current(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffff // [23:0]
  }
  #[inline]
  pub fn set_current(mut self, value: u32) -> Self {
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
#[derive(PartialEq, Eq)]
pub struct Calib(pub u32);
impl Calib {
  #[inline]
  pub fn noref(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  #[inline]
  pub fn set_noref(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

  #[inline]
  pub fn skew(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  #[inline]
  pub fn set_skew(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  #[inline]
  pub fn tenms(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffff // [23:0]
  }
  #[inline]
  pub fn set_tenms(mut self, value: u32) -> Self {
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

