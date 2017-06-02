pub const TC3: Tc3 = Tc3 {};
pub const TC3_REF: &Tc3 = &TC3;
pub const TC3_IMPL: TcImpl = TcImpl(0x42002c00);
pub const TC3_IMPL_REF: &TcImpl = &TC3_IMPL;

pub struct Tc3 {}
impl ::core::ops::Deref for Tc3 {
   type Target = TcImpl;
   #[inline]
   fn deref(&self) -> &TcImpl { TC3_IMPL_REF }
}


pub const TC4: Tc4 = Tc4 {};
pub const TC4_REF: &Tc4 = &TC4;
pub const TC4_IMPL: TcImpl = TcImpl(0x42003000);
pub const TC4_IMPL_REF: &TcImpl = &TC4_IMPL;

pub struct Tc4 {}
impl ::core::ops::Deref for Tc4 {
   type Target = TcImpl;
   #[inline]
   fn deref(&self) -> &TcImpl { TC4_IMPL_REF }
}


pub const TC5: Tc5 = Tc5 {};
pub const TC5_REF: &Tc5 = &TC5;
pub const TC5_IMPL: TcImpl = TcImpl(0x42003400);
pub const TC5_IMPL_REF: &TcImpl = &TC5_IMPL;

pub struct Tc5 {}
impl ::core::ops::Deref for Tc5 {
   type Target = TcImpl;
   #[inline]
   fn deref(&self) -> &TcImpl { TC5_IMPL_REF }
}



#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TcImpl(pub u32);
impl TcImpl {
   #[inline]
   pub fn count8(&self) -> count8::Count8 {
      count8::Count8(self.0 + 0x0)
   }
   #[inline]
   pub fn count16(&self) -> count16::Count16 {
      count16::Count16(self.0 + 0x0)
   }
   #[inline]
   pub fn count32(&self) -> count32::Count32 {
      count32::Count32(self.0 + 0x0)
   }
}
pub mod count8 {
   #[derive(Clone, Copy, PartialEq, Eq)]
   pub struct Count8(pub u32);
impl Count8 {
  #[inline]
  pub fn cc_ptr(&self, index: usize) -> *const u8 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x18 + (index)) as *const u8
  }
  #[inline]
  pub fn cc_mut(&self, index: usize) -> *mut u8 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x18 + (index)) as *mut u8
  }
  #[inline]
  pub fn cc(&self, index: usize) -> Cc { 
     assert!(index < 2);
     unsafe {
        Cc(::core::ptr::read_volatile(((self.0 as usize) + 0x18 + (index)) as *const u8))
     }
  }
  #[inline]
  pub fn set_cc(&self, index: usize, value: Cc) -> &Count8 {
     assert!(index < 2);
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x18 + (index)) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_cc<F: FnOnce(Cc) -> Cc>(&self, index: usize, f: F) -> &Count8 {
     let tmp = self.cc(index);
     self.set_cc(index, f(tmp))
  }

  #[inline]
  pub fn count_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x10) as *const u8
  }
  #[inline]
  pub fn count_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x10) as *mut u8
  }
  #[inline]
  pub fn count(&self) -> Count { 
     unsafe {
       Count(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u8))
     }
  }
  #[inline]
  pub fn set_count(&self, value: Count) -> &Count8 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Count8 {
     let tmp = self.count();
     self.set_count(f(tmp))
  }

  #[inline]
  pub fn per_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x14) as *const u8
  }
  #[inline]
  pub fn per_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x14) as *mut u8
  }
  #[inline]
  pub fn per(&self) -> Per { 
     unsafe {
       Per(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u8))
     }
  }
  #[inline]
  pub fn set_per(&self, value: Per) -> &Count8 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_per<F: FnOnce(Per) -> Per>(&self, f: F) -> &Count8 {
     let tmp = self.per();
     self.set_per(f(tmp))
  }

  #[inline]
  pub fn ctrla_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x0) as *const u16
  }
  #[inline]
  pub fn ctrla_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x0) as *mut u16
  }
  #[inline]
  pub fn ctrla(&self) -> Ctrla { 
     unsafe {
       Ctrla(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u16))
     }
  }
  #[inline]
  pub fn set_ctrla(&self, value: Ctrla) -> &Count8 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u16, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Count8 {
     let tmp = self.ctrla();
     self.set_ctrla(f(tmp))
  }

  #[inline]
  pub fn ctrlbclr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x4) as *const u8
  }
  #[inline]
  pub fn ctrlbclr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x4) as *mut u8
  }
  #[inline]
  pub fn ctrlbclr(&self) -> Ctrlbclr { 
     unsafe {
       Ctrlbclr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u8))
     }
  }
  #[inline]
  pub fn set_ctrlbclr(&self, value: Ctrlbclr) -> &Count8 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Count8 {
     let tmp = self.ctrlbclr();
     self.set_ctrlbclr(f(tmp))
  }

  #[inline]
  pub fn ctrlbset_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x5) as *const u8
  }
  #[inline]
  pub fn ctrlbset_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x5) as *mut u8
  }
  #[inline]
  pub fn ctrlbset(&self) -> Ctrlbset { 
     unsafe {
       Ctrlbset(::core::ptr::read_volatile(((self.0 as usize) + 0x5) as *const u8))
     }
  }
  #[inline]
  pub fn set_ctrlbset(&self, value: Ctrlbset) -> &Count8 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x5) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Count8 {
     let tmp = self.ctrlbset();
     self.set_ctrlbset(f(tmp))
  }

  #[inline]
  pub fn ctrlc_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x6) as *const u8
  }
  #[inline]
  pub fn ctrlc_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x6) as *mut u8
  }
  #[inline]
  pub fn ctrlc(&self) -> Ctrlc { 
     unsafe {
       Ctrlc(::core::ptr::read_volatile(((self.0 as usize) + 0x6) as *const u8))
     }
  }
  #[inline]
  pub fn set_ctrlc(&self, value: Ctrlc) -> &Count8 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x6) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Count8 {
     let tmp = self.ctrlc();
     self.set_ctrlc(f(tmp))
  }

  #[inline]
  pub fn dbgctrl_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x8) as *const u8
  }
  #[inline]
  pub fn dbgctrl_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x8) as *mut u8
  }
  #[inline]
  pub fn dbgctrl(&self) -> Dbgctrl { 
     unsafe {
       Dbgctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u8))
     }
  }
  #[inline]
  pub fn set_dbgctrl(&self, value: Dbgctrl) -> &Count8 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Count8 {
     let tmp = self.dbgctrl();
     self.set_dbgctrl(f(tmp))
  }

  #[inline]
  pub fn evctrl_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0xa) as *const u16
  }
  #[inline]
  pub fn evctrl_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0xa) as *mut u16
  }
  #[inline]
  pub fn evctrl(&self) -> Evctrl { 
     unsafe {
       Evctrl(::core::ptr::read_volatile(((self.0 as usize) + 0xa) as *const u16))
     }
  }
  #[inline]
  pub fn set_evctrl(&self, value: Evctrl) -> &Count8 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xa) as *mut u16, value.0);
     }
     self
  }
  #[inline]
  pub fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Count8 {
     let tmp = self.evctrl();
     self.set_evctrl(f(tmp))
  }

  #[inline]
  pub fn intenclr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xc) as *const u8
  }
  #[inline]
  pub fn intenclr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xc) as *mut u8
  }
  #[inline]
  pub fn intenclr(&self) -> Intenclr { 
     unsafe {
       Intenclr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u8))
     }
  }
  #[inline]
  pub fn set_intenclr(&self, value: Intenclr) -> &Count8 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Count8 {
     let tmp = self.intenclr();
     self.set_intenclr(f(tmp))
  }

  #[inline]
  pub fn intenset_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xd) as *const u8
  }
  #[inline]
  pub fn intenset_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xd) as *mut u8
  }
  #[inline]
  pub fn intenset(&self) -> Intenset { 
     unsafe {
       Intenset(::core::ptr::read_volatile(((self.0 as usize) + 0xd) as *const u8))
     }
  }
  #[inline]
  pub fn set_intenset(&self, value: Intenset) -> &Count8 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xd) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Count8 {
     let tmp = self.intenset();
     self.set_intenset(f(tmp))
  }

  #[inline]
  pub fn intflag_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xe) as *const u8
  }
  #[inline]
  pub fn intflag_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xe) as *mut u8
  }
  #[inline]
  pub fn intflag(&self) -> Intflag { 
     unsafe {
       Intflag(::core::ptr::read_volatile(((self.0 as usize) + 0xe) as *const u8))
     }
  }
  #[inline]
  pub fn set_intflag(&self, value: Intflag) -> &Count8 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xe) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Count8 {
     let tmp = self.intflag();
     self.set_intflag(f(tmp))
  }

  #[inline]
  pub fn readreq_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x2) as *const u16
  }
  #[inline]
  pub fn readreq_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x2) as *mut u16
  }
  #[inline]
  pub fn readreq(&self) -> Readreq { 
     unsafe {
       Readreq(::core::ptr::read_volatile(((self.0 as usize) + 0x2) as *const u16))
     }
  }
  #[inline]
  pub fn set_readreq(&self, value: Readreq) -> &Count8 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x2) as *mut u16, value.0);
     }
     self
  }
  #[inline]
  pub fn with_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Count8 {
     let tmp = self.readreq();
     self.set_readreq(f(tmp))
  }

  #[inline]
  pub fn status_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xf) as *const u8
  }
  #[inline]
  pub fn status_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xf) as *mut u8
  }
  #[inline]
  pub fn status(&self) -> Status { 
     unsafe {
       Status(::core::ptr::read_volatile(((self.0 as usize) + 0xf) as *const u8))
     }
  }

}

#[derive(PartialEq, Eq)]
pub struct Cc(pub u8);
impl Cc {
  #[inline]
  pub fn cc(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  #[inline]
  pub fn set_cc(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cc() != 0 { try!(write!(f, " cc=0x{:x}", self.cc()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Count(pub u8);
impl Count {
  #[inline]
  pub fn count(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  #[inline]
  pub fn set_count(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Per(pub u8);
impl Per {
  #[inline]
  pub fn per(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  #[inline]
  pub fn set_per(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Per {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Per {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.per() != 0 { try!(write!(f, " per=0x{:x}", self.per()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ctrla(pub u16);
impl Ctrla {
  #[inline]
  pub fn swrst(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_swrst(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn enable(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_enable(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn mode(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x3 // [3:2]
  }
  #[inline]
  pub fn set_mode(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn wavegen(&self) -> u16 {
     ((self.0 as u16) >> 5) & 0x3 // [6:5]
  }
  #[inline]
  pub fn set_wavegen(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn prescaler(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x7 // [10:8]
  }
  #[inline]
  pub fn set_prescaler(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn runstdby(&self) -> u16 {
     ((self.0 as u16) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_runstdby(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn prescsync(&self) -> u16 {
     ((self.0 as u16) >> 12) & 0x3 // [13:12]
  }
  #[inline]
  pub fn set_prescsync(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 12);
     self.0 |= value << 12;
     self
  }

}
impl ::core::fmt::Display for Ctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swrst() != 0 { try!(write!(f, " swrst"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
      if self.wavegen() != 0 { try!(write!(f, " wavegen=0x{:x}", self.wavegen()))}
      if self.prescaler() != 0 { try!(write!(f, " prescaler=0x{:x}", self.prescaler()))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      if self.prescsync() != 0 { try!(write!(f, " prescsync=0x{:x}", self.prescsync()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ctrlbclr(pub u8);
impl Ctrlbclr {
  #[inline]
  pub fn dir(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_dir(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn oneshot(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_oneshot(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn cmd(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x3 // [7:6]
  }
  #[inline]
  pub fn set_cmd(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

}
impl ::core::fmt::Display for Ctrlbclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrlbclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dir() != 0 { try!(write!(f, " dir"))}
      if self.oneshot() != 0 { try!(write!(f, " oneshot"))}
      if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ctrlbset(pub u8);
impl Ctrlbset {
  #[inline]
  pub fn dir(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_dir(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn oneshot(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_oneshot(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn cmd(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x3 // [7:6]
  }
  #[inline]
  pub fn set_cmd(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

}
impl ::core::fmt::Display for Ctrlbset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrlbset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dir() != 0 { try!(write!(f, " dir"))}
      if self.oneshot() != 0 { try!(write!(f, " oneshot"))}
      if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ctrlc(pub u8);
impl Ctrlc {
  #[inline]
  pub fn inven0(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_inven0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn inven1(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_inven1(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn cpten0(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_cpten0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn cpten1(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_cpten1(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for Ctrlc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrlc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.inven0() != 0 { try!(write!(f, " inven0"))}
      if self.inven1() != 0 { try!(write!(f, " inven1"))}
      if self.cpten0() != 0 { try!(write!(f, " cpten0"))}
      if self.cpten1() != 0 { try!(write!(f, " cpten1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
  #[inline]
  pub fn dbgrun(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_dbgrun(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dbgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dbgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dbgrun() != 0 { try!(write!(f, " dbgrun"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Evctrl(pub u16);
impl Evctrl {
  #[inline]
  pub fn evact(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x7 // [2:0]
  }
  #[inline]
  pub fn set_evact(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn tcinv(&self) -> u16 {
     ((self.0 as u16) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_tcinv(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn tcei(&self) -> u16 {
     ((self.0 as u16) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_tcei(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn ovfeo(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_ovfeo(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn mceo0(&self) -> u16 {
     ((self.0 as u16) >> 12) & 0x1 // [12]
  }
  #[inline]
  pub fn set_mceo0(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn mceo1(&self) -> u16 {
     ((self.0 as u16) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_mceo1(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

}
impl ::core::fmt::Display for Evctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Evctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.evact() != 0 { try!(write!(f, " evact=0x{:x}", self.evact()))}
      if self.tcinv() != 0 { try!(write!(f, " tcinv"))}
      if self.tcei() != 0 { try!(write!(f, " tcei"))}
      if self.ovfeo() != 0 { try!(write!(f, " ovfeo"))}
      if self.mceo0() != 0 { try!(write!(f, " mceo0"))}
      if self.mceo1() != 0 { try!(write!(f, " mceo1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
  #[inline]
  pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn err(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_err(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn mc0(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_mc0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn mc1(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_mc1(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for Intenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      if self.err() != 0 { try!(write!(f, " err"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.mc0() != 0 { try!(write!(f, " mc0"))}
      if self.mc1() != 0 { try!(write!(f, " mc1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
  #[inline]
  pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn err(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_err(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn mc0(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_mc0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn mc1(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_mc1(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for Intenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      if self.err() != 0 { try!(write!(f, " err"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.mc0() != 0 { try!(write!(f, " mc0"))}
      if self.mc1() != 0 { try!(write!(f, " mc1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
  #[inline]
  pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn err(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_err(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn mc0(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_mc0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn mc1(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_mc1(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for Intflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      if self.err() != 0 { try!(write!(f, " err"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.mc0() != 0 { try!(write!(f, " mc0"))}
      if self.mc1() != 0 { try!(write!(f, " mc1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Readreq(pub u16);
impl Readreq {
  #[inline]
  pub fn addr(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1f // [4:0]
  }
  #[inline]
  pub fn set_addr(mut self, value: u16) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn rcont(&self) -> u16 {
     ((self.0 as u16) >> 14) & 0x1 // [14]
  }
  #[inline]
  pub fn set_rcont(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline]
  pub fn rreq(&self) -> u16 {
     ((self.0 as u16) >> 15) & 0x1 // [15]
  }
  #[inline]
  pub fn set_rreq(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

}
impl ::core::fmt::Display for Readreq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Readreq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
      if self.rcont() != 0 { try!(write!(f, " rcont"))}
      if self.rreq() != 0 { try!(write!(f, " rreq"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
  #[inline]
  pub fn stop(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_stop(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn slave(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_slave(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn syncbusy(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_syncbusy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.stop() != 0 { try!(write!(f, " stop"))}
      if self.slave() != 0 { try!(write!(f, " slave"))}
      if self.syncbusy() != 0 { try!(write!(f, " syncbusy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
}
// End of count8
pub mod count16 {
   #[derive(Clone, Copy, PartialEq, Eq)]
   pub struct Count16(pub u32);
impl Count16 {
  #[inline]
  pub fn cc_ptr(&self, index: usize) -> *const u16 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x18 + (index << 1)) as *const u16
  }
  #[inline]
  pub fn cc_mut(&self, index: usize) -> *mut u16 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x18 + (index << 1)) as *mut u16
  }
  #[inline]
  pub fn cc(&self, index: usize) -> Cc { 
     assert!(index < 2);
     unsafe {
        Cc(::core::ptr::read_volatile(((self.0 as usize) + 0x18 + (index << 1)) as *const u16))
     }
  }
  #[inline]
  pub fn set_cc(&self, index: usize, value: Cc) -> &Count16 {
     assert!(index < 2);
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x18 + (index << 1)) as *mut u16, value.0);
     }
     self
  }
  #[inline]
  pub fn with_cc<F: FnOnce(Cc) -> Cc>(&self, index: usize, f: F) -> &Count16 {
     let tmp = self.cc(index);
     self.set_cc(index, f(tmp))
  }

  #[inline]
  pub fn count_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x10) as *const u16
  }
  #[inline]
  pub fn count_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x10) as *mut u16
  }
  #[inline]
  pub fn count(&self) -> Count { 
     unsafe {
       Count(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u16))
     }
  }
  #[inline]
  pub fn set_count(&self, value: Count) -> &Count16 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u16, value.0);
     }
     self
  }
  #[inline]
  pub fn with_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Count16 {
     let tmp = self.count();
     self.set_count(f(tmp))
  }

  #[inline]
  pub fn ctrla_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x0) as *const u16
  }
  #[inline]
  pub fn ctrla_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x0) as *mut u16
  }
  #[inline]
  pub fn ctrla(&self) -> Ctrla { 
     unsafe {
       Ctrla(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u16))
     }
  }
  #[inline]
  pub fn set_ctrla(&self, value: Ctrla) -> &Count16 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u16, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Count16 {
     let tmp = self.ctrla();
     self.set_ctrla(f(tmp))
  }

  #[inline]
  pub fn ctrlbclr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x4) as *const u8
  }
  #[inline]
  pub fn ctrlbclr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x4) as *mut u8
  }
  #[inline]
  pub fn ctrlbclr(&self) -> Ctrlbclr { 
     unsafe {
       Ctrlbclr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u8))
     }
  }
  #[inline]
  pub fn set_ctrlbclr(&self, value: Ctrlbclr) -> &Count16 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Count16 {
     let tmp = self.ctrlbclr();
     self.set_ctrlbclr(f(tmp))
  }

  #[inline]
  pub fn ctrlbset_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x5) as *const u8
  }
  #[inline]
  pub fn ctrlbset_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x5) as *mut u8
  }
  #[inline]
  pub fn ctrlbset(&self) -> Ctrlbset { 
     unsafe {
       Ctrlbset(::core::ptr::read_volatile(((self.0 as usize) + 0x5) as *const u8))
     }
  }
  #[inline]
  pub fn set_ctrlbset(&self, value: Ctrlbset) -> &Count16 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x5) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Count16 {
     let tmp = self.ctrlbset();
     self.set_ctrlbset(f(tmp))
  }

  #[inline]
  pub fn ctrlc_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x6) as *const u8
  }
  #[inline]
  pub fn ctrlc_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x6) as *mut u8
  }
  #[inline]
  pub fn ctrlc(&self) -> Ctrlc { 
     unsafe {
       Ctrlc(::core::ptr::read_volatile(((self.0 as usize) + 0x6) as *const u8))
     }
  }
  #[inline]
  pub fn set_ctrlc(&self, value: Ctrlc) -> &Count16 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x6) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Count16 {
     let tmp = self.ctrlc();
     self.set_ctrlc(f(tmp))
  }

  #[inline]
  pub fn dbgctrl_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x8) as *const u8
  }
  #[inline]
  pub fn dbgctrl_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x8) as *mut u8
  }
  #[inline]
  pub fn dbgctrl(&self) -> Dbgctrl { 
     unsafe {
       Dbgctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u8))
     }
  }
  #[inline]
  pub fn set_dbgctrl(&self, value: Dbgctrl) -> &Count16 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Count16 {
     let tmp = self.dbgctrl();
     self.set_dbgctrl(f(tmp))
  }

  #[inline]
  pub fn evctrl_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0xa) as *const u16
  }
  #[inline]
  pub fn evctrl_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0xa) as *mut u16
  }
  #[inline]
  pub fn evctrl(&self) -> Evctrl { 
     unsafe {
       Evctrl(::core::ptr::read_volatile(((self.0 as usize) + 0xa) as *const u16))
     }
  }
  #[inline]
  pub fn set_evctrl(&self, value: Evctrl) -> &Count16 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xa) as *mut u16, value.0);
     }
     self
  }
  #[inline]
  pub fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Count16 {
     let tmp = self.evctrl();
     self.set_evctrl(f(tmp))
  }

  #[inline]
  pub fn intenclr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xc) as *const u8
  }
  #[inline]
  pub fn intenclr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xc) as *mut u8
  }
  #[inline]
  pub fn intenclr(&self) -> Intenclr { 
     unsafe {
       Intenclr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u8))
     }
  }
  #[inline]
  pub fn set_intenclr(&self, value: Intenclr) -> &Count16 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Count16 {
     let tmp = self.intenclr();
     self.set_intenclr(f(tmp))
  }

  #[inline]
  pub fn intenset_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xd) as *const u8
  }
  #[inline]
  pub fn intenset_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xd) as *mut u8
  }
  #[inline]
  pub fn intenset(&self) -> Intenset { 
     unsafe {
       Intenset(::core::ptr::read_volatile(((self.0 as usize) + 0xd) as *const u8))
     }
  }
  #[inline]
  pub fn set_intenset(&self, value: Intenset) -> &Count16 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xd) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Count16 {
     let tmp = self.intenset();
     self.set_intenset(f(tmp))
  }

  #[inline]
  pub fn intflag_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xe) as *const u8
  }
  #[inline]
  pub fn intflag_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xe) as *mut u8
  }
  #[inline]
  pub fn intflag(&self) -> Intflag { 
     unsafe {
       Intflag(::core::ptr::read_volatile(((self.0 as usize) + 0xe) as *const u8))
     }
  }
  #[inline]
  pub fn set_intflag(&self, value: Intflag) -> &Count16 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xe) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Count16 {
     let tmp = self.intflag();
     self.set_intflag(f(tmp))
  }

  #[inline]
  pub fn readreq_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x2) as *const u16
  }
  #[inline]
  pub fn readreq_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x2) as *mut u16
  }
  #[inline]
  pub fn readreq(&self) -> Readreq { 
     unsafe {
       Readreq(::core::ptr::read_volatile(((self.0 as usize) + 0x2) as *const u16))
     }
  }
  #[inline]
  pub fn set_readreq(&self, value: Readreq) -> &Count16 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x2) as *mut u16, value.0);
     }
     self
  }
  #[inline]
  pub fn with_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Count16 {
     let tmp = self.readreq();
     self.set_readreq(f(tmp))
  }

  #[inline]
  pub fn status_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xf) as *const u8
  }
  #[inline]
  pub fn status_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xf) as *mut u8
  }
  #[inline]
  pub fn status(&self) -> Status { 
     unsafe {
       Status(::core::ptr::read_volatile(((self.0 as usize) + 0xf) as *const u8))
     }
  }

}

#[derive(PartialEq, Eq)]
pub struct Cc(pub u16);
impl Cc {
  #[inline]
  pub fn cc(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
  #[inline]
  pub fn set_cc(mut self, value: u16) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cc() != 0 { try!(write!(f, " cc=0x{:x}", self.cc()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Count(pub u16);
impl Count {
  #[inline]
  pub fn count(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
  #[inline]
  pub fn set_count(mut self, value: u16) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ctrla(pub u16);
impl Ctrla {
  #[inline]
  pub fn swrst(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_swrst(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn enable(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_enable(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn mode(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x3 // [3:2]
  }
  #[inline]
  pub fn set_mode(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn wavegen(&self) -> u16 {
     ((self.0 as u16) >> 5) & 0x3 // [6:5]
  }
  #[inline]
  pub fn set_wavegen(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn prescaler(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x7 // [10:8]
  }
  #[inline]
  pub fn set_prescaler(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn runstdby(&self) -> u16 {
     ((self.0 as u16) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_runstdby(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn prescsync(&self) -> u16 {
     ((self.0 as u16) >> 12) & 0x3 // [13:12]
  }
  #[inline]
  pub fn set_prescsync(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 12);
     self.0 |= value << 12;
     self
  }

}
impl ::core::fmt::Display for Ctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swrst() != 0 { try!(write!(f, " swrst"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
      if self.wavegen() != 0 { try!(write!(f, " wavegen=0x{:x}", self.wavegen()))}
      if self.prescaler() != 0 { try!(write!(f, " prescaler=0x{:x}", self.prescaler()))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      if self.prescsync() != 0 { try!(write!(f, " prescsync=0x{:x}", self.prescsync()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ctrlbclr(pub u8);
impl Ctrlbclr {
  #[inline]
  pub fn dir(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_dir(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn oneshot(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_oneshot(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn cmd(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x3 // [7:6]
  }
  #[inline]
  pub fn set_cmd(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

}
impl ::core::fmt::Display for Ctrlbclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrlbclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dir() != 0 { try!(write!(f, " dir"))}
      if self.oneshot() != 0 { try!(write!(f, " oneshot"))}
      if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ctrlbset(pub u8);
impl Ctrlbset {
  #[inline]
  pub fn dir(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_dir(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn oneshot(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_oneshot(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn cmd(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x3 // [7:6]
  }
  #[inline]
  pub fn set_cmd(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

}
impl ::core::fmt::Display for Ctrlbset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrlbset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dir() != 0 { try!(write!(f, " dir"))}
      if self.oneshot() != 0 { try!(write!(f, " oneshot"))}
      if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ctrlc(pub u8);
impl Ctrlc {
  #[inline]
  pub fn inven0(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_inven0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn inven1(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_inven1(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn cpten0(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_cpten0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn cpten1(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_cpten1(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for Ctrlc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrlc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.inven0() != 0 { try!(write!(f, " inven0"))}
      if self.inven1() != 0 { try!(write!(f, " inven1"))}
      if self.cpten0() != 0 { try!(write!(f, " cpten0"))}
      if self.cpten1() != 0 { try!(write!(f, " cpten1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
  #[inline]
  pub fn dbgrun(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_dbgrun(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dbgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dbgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dbgrun() != 0 { try!(write!(f, " dbgrun"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Evctrl(pub u16);
impl Evctrl {
  #[inline]
  pub fn evact(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x7 // [2:0]
  }
  #[inline]
  pub fn set_evact(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn tcinv(&self) -> u16 {
     ((self.0 as u16) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_tcinv(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn tcei(&self) -> u16 {
     ((self.0 as u16) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_tcei(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn ovfeo(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_ovfeo(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn mceo0(&self) -> u16 {
     ((self.0 as u16) >> 12) & 0x1 // [12]
  }
  #[inline]
  pub fn set_mceo0(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn mceo1(&self) -> u16 {
     ((self.0 as u16) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_mceo1(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

}
impl ::core::fmt::Display for Evctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Evctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.evact() != 0 { try!(write!(f, " evact=0x{:x}", self.evact()))}
      if self.tcinv() != 0 { try!(write!(f, " tcinv"))}
      if self.tcei() != 0 { try!(write!(f, " tcei"))}
      if self.ovfeo() != 0 { try!(write!(f, " ovfeo"))}
      if self.mceo0() != 0 { try!(write!(f, " mceo0"))}
      if self.mceo1() != 0 { try!(write!(f, " mceo1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
  #[inline]
  pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn err(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_err(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn mc0(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_mc0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn mc1(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_mc1(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for Intenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      if self.err() != 0 { try!(write!(f, " err"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.mc0() != 0 { try!(write!(f, " mc0"))}
      if self.mc1() != 0 { try!(write!(f, " mc1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
  #[inline]
  pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn err(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_err(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn mc0(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_mc0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn mc1(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_mc1(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for Intenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      if self.err() != 0 { try!(write!(f, " err"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.mc0() != 0 { try!(write!(f, " mc0"))}
      if self.mc1() != 0 { try!(write!(f, " mc1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
  #[inline]
  pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn err(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_err(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn mc0(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_mc0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn mc1(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_mc1(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for Intflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      if self.err() != 0 { try!(write!(f, " err"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.mc0() != 0 { try!(write!(f, " mc0"))}
      if self.mc1() != 0 { try!(write!(f, " mc1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Readreq(pub u16);
impl Readreq {
  #[inline]
  pub fn addr(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1f // [4:0]
  }
  #[inline]
  pub fn set_addr(mut self, value: u16) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn rcont(&self) -> u16 {
     ((self.0 as u16) >> 14) & 0x1 // [14]
  }
  #[inline]
  pub fn set_rcont(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline]
  pub fn rreq(&self) -> u16 {
     ((self.0 as u16) >> 15) & 0x1 // [15]
  }
  #[inline]
  pub fn set_rreq(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

}
impl ::core::fmt::Display for Readreq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Readreq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
      if self.rcont() != 0 { try!(write!(f, " rcont"))}
      if self.rreq() != 0 { try!(write!(f, " rreq"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
  #[inline]
  pub fn stop(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_stop(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn slave(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_slave(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn syncbusy(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_syncbusy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.stop() != 0 { try!(write!(f, " stop"))}
      if self.slave() != 0 { try!(write!(f, " slave"))}
      if self.syncbusy() != 0 { try!(write!(f, " syncbusy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
}
// End of count16
pub mod count32 {
   #[derive(Clone, Copy, PartialEq, Eq)]
   pub struct Count32(pub u32);
impl Count32 {
  #[inline]
  pub fn cc_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x18 + (index << 2)) as *const u32
  }
  #[inline]
  pub fn cc_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x18 + (index << 2)) as *mut u32
  }
  #[inline]
  pub fn cc(&self, index: usize) -> Cc { 
     assert!(index < 2);
     unsafe {
        Cc(::core::ptr::read_volatile(((self.0 as usize) + 0x18 + (index << 2)) as *const u32))
     }
  }
  #[inline]
  pub fn set_cc(&self, index: usize, value: Cc) -> &Count32 {
     assert!(index < 2);
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x18 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_cc<F: FnOnce(Cc) -> Cc>(&self, index: usize, f: F) -> &Count32 {
     let tmp = self.cc(index);
     self.set_cc(index, f(tmp))
  }

  #[inline]
  pub fn count_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
  #[inline]
  pub fn count_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
  #[inline]
  pub fn count(&self) -> Count { 
     unsafe {
       Count(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
  #[inline]
  pub fn set_count(&self, value: Count) -> &Count32 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Count32 {
     let tmp = self.count();
     self.set_count(f(tmp))
  }

  #[inline]
  pub fn ctrla_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x0) as *const u16
  }
  #[inline]
  pub fn ctrla_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x0) as *mut u16
  }
  #[inline]
  pub fn ctrla(&self) -> Ctrla { 
     unsafe {
       Ctrla(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u16))
     }
  }
  #[inline]
  pub fn set_ctrla(&self, value: Ctrla) -> &Count32 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u16, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Count32 {
     let tmp = self.ctrla();
     self.set_ctrla(f(tmp))
  }

  #[inline]
  pub fn ctrlbclr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x4) as *const u8
  }
  #[inline]
  pub fn ctrlbclr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x4) as *mut u8
  }
  #[inline]
  pub fn ctrlbclr(&self) -> Ctrlbclr { 
     unsafe {
       Ctrlbclr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u8))
     }
  }
  #[inline]
  pub fn set_ctrlbclr(&self, value: Ctrlbclr) -> &Count32 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Count32 {
     let tmp = self.ctrlbclr();
     self.set_ctrlbclr(f(tmp))
  }

  #[inline]
  pub fn ctrlbset_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x5) as *const u8
  }
  #[inline]
  pub fn ctrlbset_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x5) as *mut u8
  }
  #[inline]
  pub fn ctrlbset(&self) -> Ctrlbset { 
     unsafe {
       Ctrlbset(::core::ptr::read_volatile(((self.0 as usize) + 0x5) as *const u8))
     }
  }
  #[inline]
  pub fn set_ctrlbset(&self, value: Ctrlbset) -> &Count32 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x5) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Count32 {
     let tmp = self.ctrlbset();
     self.set_ctrlbset(f(tmp))
  }

  #[inline]
  pub fn ctrlc_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x6) as *const u8
  }
  #[inline]
  pub fn ctrlc_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x6) as *mut u8
  }
  #[inline]
  pub fn ctrlc(&self) -> Ctrlc { 
     unsafe {
       Ctrlc(::core::ptr::read_volatile(((self.0 as usize) + 0x6) as *const u8))
     }
  }
  #[inline]
  pub fn set_ctrlc(&self, value: Ctrlc) -> &Count32 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x6) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Count32 {
     let tmp = self.ctrlc();
     self.set_ctrlc(f(tmp))
  }

  #[inline]
  pub fn dbgctrl_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x8) as *const u8
  }
  #[inline]
  pub fn dbgctrl_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x8) as *mut u8
  }
  #[inline]
  pub fn dbgctrl(&self) -> Dbgctrl { 
     unsafe {
       Dbgctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u8))
     }
  }
  #[inline]
  pub fn set_dbgctrl(&self, value: Dbgctrl) -> &Count32 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Count32 {
     let tmp = self.dbgctrl();
     self.set_dbgctrl(f(tmp))
  }

  #[inline]
  pub fn evctrl_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0xa) as *const u16
  }
  #[inline]
  pub fn evctrl_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0xa) as *mut u16
  }
  #[inline]
  pub fn evctrl(&self) -> Evctrl { 
     unsafe {
       Evctrl(::core::ptr::read_volatile(((self.0 as usize) + 0xa) as *const u16))
     }
  }
  #[inline]
  pub fn set_evctrl(&self, value: Evctrl) -> &Count32 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xa) as *mut u16, value.0);
     }
     self
  }
  #[inline]
  pub fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Count32 {
     let tmp = self.evctrl();
     self.set_evctrl(f(tmp))
  }

  #[inline]
  pub fn intenclr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xc) as *const u8
  }
  #[inline]
  pub fn intenclr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xc) as *mut u8
  }
  #[inline]
  pub fn intenclr(&self) -> Intenclr { 
     unsafe {
       Intenclr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u8))
     }
  }
  #[inline]
  pub fn set_intenclr(&self, value: Intenclr) -> &Count32 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Count32 {
     let tmp = self.intenclr();
     self.set_intenclr(f(tmp))
  }

  #[inline]
  pub fn intenset_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xd) as *const u8
  }
  #[inline]
  pub fn intenset_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xd) as *mut u8
  }
  #[inline]
  pub fn intenset(&self) -> Intenset { 
     unsafe {
       Intenset(::core::ptr::read_volatile(((self.0 as usize) + 0xd) as *const u8))
     }
  }
  #[inline]
  pub fn set_intenset(&self, value: Intenset) -> &Count32 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xd) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Count32 {
     let tmp = self.intenset();
     self.set_intenset(f(tmp))
  }

  #[inline]
  pub fn intflag_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xe) as *const u8
  }
  #[inline]
  pub fn intflag_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xe) as *mut u8
  }
  #[inline]
  pub fn intflag(&self) -> Intflag { 
     unsafe {
       Intflag(::core::ptr::read_volatile(((self.0 as usize) + 0xe) as *const u8))
     }
  }
  #[inline]
  pub fn set_intflag(&self, value: Intflag) -> &Count32 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xe) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Count32 {
     let tmp = self.intflag();
     self.set_intflag(f(tmp))
  }

  #[inline]
  pub fn readreq_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x2) as *const u16
  }
  #[inline]
  pub fn readreq_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x2) as *mut u16
  }
  #[inline]
  pub fn readreq(&self) -> Readreq { 
     unsafe {
       Readreq(::core::ptr::read_volatile(((self.0 as usize) + 0x2) as *const u16))
     }
  }
  #[inline]
  pub fn set_readreq(&self, value: Readreq) -> &Count32 {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x2) as *mut u16, value.0);
     }
     self
  }
  #[inline]
  pub fn with_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Count32 {
     let tmp = self.readreq();
     self.set_readreq(f(tmp))
  }

  #[inline]
  pub fn status_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xf) as *const u8
  }
  #[inline]
  pub fn status_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xf) as *mut u8
  }
  #[inline]
  pub fn status(&self) -> Status { 
     unsafe {
       Status(::core::ptr::read_volatile(((self.0 as usize) + 0xf) as *const u8))
     }
  }

}

#[derive(PartialEq, Eq)]
pub struct Cc(pub u32);
impl Cc {
  #[inline]
  pub fn cc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
  pub fn set_cc(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Count(pub u32);
impl Count {
  #[inline]
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
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
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ctrla(pub u16);
impl Ctrla {
  #[inline]
  pub fn swrst(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_swrst(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn enable(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_enable(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn mode(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x3 // [3:2]
  }
  #[inline]
  pub fn set_mode(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn wavegen(&self) -> u16 {
     ((self.0 as u16) >> 5) & 0x3 // [6:5]
  }
  #[inline]
  pub fn set_wavegen(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn prescaler(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x7 // [10:8]
  }
  #[inline]
  pub fn set_prescaler(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn runstdby(&self) -> u16 {
     ((self.0 as u16) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_runstdby(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn prescsync(&self) -> u16 {
     ((self.0 as u16) >> 12) & 0x3 // [13:12]
  }
  #[inline]
  pub fn set_prescsync(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 12);
     self.0 |= value << 12;
     self
  }

}
impl ::core::fmt::Display for Ctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swrst() != 0 { try!(write!(f, " swrst"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
      if self.wavegen() != 0 { try!(write!(f, " wavegen=0x{:x}", self.wavegen()))}
      if self.prescaler() != 0 { try!(write!(f, " prescaler=0x{:x}", self.prescaler()))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      if self.prescsync() != 0 { try!(write!(f, " prescsync=0x{:x}", self.prescsync()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ctrlbclr(pub u8);
impl Ctrlbclr {
  #[inline]
  pub fn dir(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_dir(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn oneshot(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_oneshot(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn cmd(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x3 // [7:6]
  }
  #[inline]
  pub fn set_cmd(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

}
impl ::core::fmt::Display for Ctrlbclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrlbclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dir() != 0 { try!(write!(f, " dir"))}
      if self.oneshot() != 0 { try!(write!(f, " oneshot"))}
      if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ctrlbset(pub u8);
impl Ctrlbset {
  #[inline]
  pub fn dir(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_dir(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn oneshot(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_oneshot(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn cmd(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x3 // [7:6]
  }
  #[inline]
  pub fn set_cmd(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

}
impl ::core::fmt::Display for Ctrlbset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrlbset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dir() != 0 { try!(write!(f, " dir"))}
      if self.oneshot() != 0 { try!(write!(f, " oneshot"))}
      if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ctrlc(pub u8);
impl Ctrlc {
  #[inline]
  pub fn inven0(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_inven0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn inven1(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_inven1(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn cpten0(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_cpten0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn cpten1(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_cpten1(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for Ctrlc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrlc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.inven0() != 0 { try!(write!(f, " inven0"))}
      if self.inven1() != 0 { try!(write!(f, " inven1"))}
      if self.cpten0() != 0 { try!(write!(f, " cpten0"))}
      if self.cpten1() != 0 { try!(write!(f, " cpten1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
  #[inline]
  pub fn dbgrun(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_dbgrun(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dbgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dbgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dbgrun() != 0 { try!(write!(f, " dbgrun"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Evctrl(pub u16);
impl Evctrl {
  #[inline]
  pub fn evact(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x7 // [2:0]
  }
  #[inline]
  pub fn set_evact(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn tcinv(&self) -> u16 {
     ((self.0 as u16) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_tcinv(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn tcei(&self) -> u16 {
     ((self.0 as u16) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_tcei(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn ovfeo(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_ovfeo(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn mceo0(&self) -> u16 {
     ((self.0 as u16) >> 12) & 0x1 // [12]
  }
  #[inline]
  pub fn set_mceo0(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn mceo1(&self) -> u16 {
     ((self.0 as u16) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_mceo1(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

}
impl ::core::fmt::Display for Evctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Evctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.evact() != 0 { try!(write!(f, " evact=0x{:x}", self.evact()))}
      if self.tcinv() != 0 { try!(write!(f, " tcinv"))}
      if self.tcei() != 0 { try!(write!(f, " tcei"))}
      if self.ovfeo() != 0 { try!(write!(f, " ovfeo"))}
      if self.mceo0() != 0 { try!(write!(f, " mceo0"))}
      if self.mceo1() != 0 { try!(write!(f, " mceo1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
  #[inline]
  pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn err(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_err(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn mc0(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_mc0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn mc1(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_mc1(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for Intenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      if self.err() != 0 { try!(write!(f, " err"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.mc0() != 0 { try!(write!(f, " mc0"))}
      if self.mc1() != 0 { try!(write!(f, " mc1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
  #[inline]
  pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn err(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_err(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn mc0(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_mc0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn mc1(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_mc1(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for Intenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      if self.err() != 0 { try!(write!(f, " err"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.mc0() != 0 { try!(write!(f, " mc0"))}
      if self.mc1() != 0 { try!(write!(f, " mc1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
  #[inline]
  pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn err(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_err(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn mc0(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_mc0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn mc1(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_mc1(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for Intflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      if self.err() != 0 { try!(write!(f, " err"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.mc0() != 0 { try!(write!(f, " mc0"))}
      if self.mc1() != 0 { try!(write!(f, " mc1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Readreq(pub u16);
impl Readreq {
  #[inline]
  pub fn addr(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1f // [4:0]
  }
  #[inline]
  pub fn set_addr(mut self, value: u16) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn rcont(&self) -> u16 {
     ((self.0 as u16) >> 14) & 0x1 // [14]
  }
  #[inline]
  pub fn set_rcont(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline]
  pub fn rreq(&self) -> u16 {
     ((self.0 as u16) >> 15) & 0x1 // [15]
  }
  #[inline]
  pub fn set_rreq(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

}
impl ::core::fmt::Display for Readreq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Readreq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
      if self.rcont() != 0 { try!(write!(f, " rcont"))}
      if self.rreq() != 0 { try!(write!(f, " rreq"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
  #[inline]
  pub fn stop(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_stop(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn slave(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_slave(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn syncbusy(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_syncbusy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.stop() != 0 { try!(write!(f, " stop"))}
      if self.slave() != 0 { try!(write!(f, " slave"))}
      if self.syncbusy() != 0 { try!(write!(f, " syncbusy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
}
// End of count32
