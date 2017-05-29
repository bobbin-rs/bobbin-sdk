pub const RTC: Rtc = Rtc(0x40001400);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rtc(pub u32);

impl Rtc {
   pub fn mode0(&self) -> mode0::Mode0 {
      mode0::Mode0(self.0 + 0x0)
   }

   pub fn mode1(&self) -> mode1::Mode1 {
      mode1::Mode1(self.0 + 0x0)
   }

   pub fn mode2(&self) -> mode2::Mode2 {
      mode2::Mode2(self.0 + 0x0)
   }

}

pub mod mode0 {
   #[derive(Clone, Copy, PartialEq, Eq)]
   pub struct Mode0(pub u32);

impl Mode0 {
  pub unsafe fn dbgctrl(&self) -> Dbgctrl { 
     Dbgctrl(::core::ptr::read_volatile(((self.0 as usize) + 0xb) as *const u8))
  }
  pub unsafe fn set_dbgctrl(&mut self, value: Dbgctrl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xb) as *mut u8, value.0);
  }
  pub unsafe fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&mut self, f: F) {
     let tmp = self.dbgctrl();
     self.set_dbgctrl(f(tmp))
  }

  pub unsafe fn freqcorr(&self) -> Freqcorr { 
     Freqcorr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u8))
  }
  pub unsafe fn set_freqcorr(&mut self, value: Freqcorr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u8, value.0);
  }
  pub unsafe fn with_freqcorr<F: FnOnce(Freqcorr) -> Freqcorr>(&mut self, f: F) {
     let tmp = self.freqcorr();
     self.set_freqcorr(f(tmp))
  }

  pub unsafe fn comp(&self, index: usize) -> Comp { 
     assert!(index < 1);
     Comp(::core::ptr::read_volatile(((self.0 as usize) + 0x18 + (index << 2)) as *const u32))
  }
  pub unsafe fn set_comp(&mut self, index: usize, value: Comp) {
     assert!(index < 1);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x18 + (index << 2)) as *mut u32, value.0);
  }
  pub unsafe fn with_comp<F: FnOnce(Comp) -> Comp>(&mut self, index: usize, f: F) {
     let tmp = self.comp(index);
     self.set_comp(index, f(tmp))
  }

  pub unsafe fn count(&self) -> Count { 
     Count(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
  }
  pub unsafe fn set_count(&mut self, value: Count) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
  }
  pub unsafe fn with_count<F: FnOnce(Count) -> Count>(&mut self, f: F) {
     let tmp = self.count();
     self.set_count(f(tmp))
  }

  pub unsafe fn ctrl(&self) -> Ctrl { 
     Ctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u16))
  }
  pub unsafe fn set_ctrl(&mut self, value: Ctrl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u16, value.0);
  }
  pub unsafe fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&mut self, f: F) {
     let tmp = self.ctrl();
     self.set_ctrl(f(tmp))
  }

  pub unsafe fn evctrl(&self) -> Evctrl { 
     Evctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u16))
  }
  pub unsafe fn set_evctrl(&mut self, value: Evctrl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u16, value.0);
  }
  pub unsafe fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&mut self, f: F) {
     let tmp = self.evctrl();
     self.set_evctrl(f(tmp))
  }

  pub unsafe fn intenclr(&self) -> Intenclr { 
     Intenclr(::core::ptr::read_volatile(((self.0 as usize) + 0x6) as *const u8))
  }
  pub unsafe fn set_intenclr(&mut self, value: Intenclr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x6) as *mut u8, value.0);
  }
  pub unsafe fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&mut self, f: F) {
     let tmp = self.intenclr();
     self.set_intenclr(f(tmp))
  }

  pub unsafe fn intenset(&self) -> Intenset { 
     Intenset(::core::ptr::read_volatile(((self.0 as usize) + 0x7) as *const u8))
  }
  pub unsafe fn set_intenset(&mut self, value: Intenset) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x7) as *mut u8, value.0);
  }
  pub unsafe fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&mut self, f: F) {
     let tmp = self.intenset();
     self.set_intenset(f(tmp))
  }

  pub unsafe fn intflag(&self) -> Intflag { 
     Intflag(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u8))
  }
  pub unsafe fn set_intflag(&mut self, value: Intflag) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u8, value.0);
  }
  pub unsafe fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&mut self, f: F) {
     let tmp = self.intflag();
     self.set_intflag(f(tmp))
  }

  pub unsafe fn readreq(&self) -> Readreq { 
     Readreq(::core::ptr::read_volatile(((self.0 as usize) + 0x2) as *const u16))
  }
  pub unsafe fn set_readreq(&mut self, value: Readreq) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x2) as *mut u16, value.0);
  }
  pub unsafe fn with_readreq<F: FnOnce(Readreq) -> Readreq>(&mut self, f: F) {
     let tmp = self.readreq();
     self.set_readreq(f(tmp))
  }

  pub unsafe fn status(&self) -> Status { 
     Status(::core::ptr::read_volatile(((self.0 as usize) + 0xa) as *const u8))
  }
  pub unsafe fn set_status(&mut self, value: Status) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa) as *mut u8, value.0);
  }
  pub unsafe fn with_status<F: FnOnce(Status) -> Status>(&mut self, f: F) {
     let tmp = self.status();
     self.set_status(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Dbgctrl(pub u8);

impl Dbgctrl {
  pub fn dbgrun(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
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
pub struct Freqcorr(pub u8);

impl Freqcorr {
  pub fn value(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x7f // [6:0]
  }
  pub fn set_value(mut self, value: u8) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 0);
     self.0 |= value << 0;
     self
  }

  pub fn sign(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_sign(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for Freqcorr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Freqcorr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.value() != 0 { try!(write!(f, " value=0x{:x}", self.value()))}
      if self.sign() != 0 { try!(write!(f, " sign"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Comp(pub u32);

impl Comp {
  pub fn comp(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_comp(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Comp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Comp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Count(pub u32);

impl Count {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
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
pub struct Ctrl(pub u16);

impl Ctrl {
  pub fn swrst(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
  pub fn set_swrst(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn enable(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
  pub fn set_enable(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn mode(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x3 // [3:2]
  }
  pub fn set_mode(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn matchclr(&self) -> u16 {
     ((self.0 as u16) >> 7) & 0x1 // [7]
  }
  pub fn set_matchclr(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn prescaler(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0xf // [11:8]
  }
  pub fn set_prescaler(mut self, value: u16) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

}

impl ::core::fmt::Display for Ctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swrst() != 0 { try!(write!(f, " swrst"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
      if self.matchclr() != 0 { try!(write!(f, " matchclr"))}
      if self.prescaler() != 0 { try!(write!(f, " prescaler=0x{:x}", self.prescaler()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Evctrl(pub u16);

impl Evctrl {
  pub fn pereo0(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
  pub fn set_pereo0(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn pereo1(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
  pub fn set_pereo1(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn pereo2(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
  pub fn set_pereo2(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn pereo3(&self) -> u16 {
     ((self.0 as u16) >> 3) & 0x1 // [3]
  }
  pub fn set_pereo3(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn pereo4(&self) -> u16 {
     ((self.0 as u16) >> 4) & 0x1 // [4]
  }
  pub fn set_pereo4(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn pereo5(&self) -> u16 {
     ((self.0 as u16) >> 5) & 0x1 // [5]
  }
  pub fn set_pereo5(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn pereo6(&self) -> u16 {
     ((self.0 as u16) >> 6) & 0x1 // [6]
  }
  pub fn set_pereo6(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn pereo7(&self) -> u16 {
     ((self.0 as u16) >> 7) & 0x1 // [7]
  }
  pub fn set_pereo7(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn cmpeo0(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x1 // [8]
  }
  pub fn set_cmpeo0(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn ovfeo(&self) -> u16 {
     ((self.0 as u16) >> 15) & 0x1 // [15]
  }
  pub fn set_ovfeo(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
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
      if self.pereo0() != 0 { try!(write!(f, " pereo0"))}
      if self.pereo1() != 0 { try!(write!(f, " pereo1"))}
      if self.pereo2() != 0 { try!(write!(f, " pereo2"))}
      if self.pereo3() != 0 { try!(write!(f, " pereo3"))}
      if self.pereo4() != 0 { try!(write!(f, " pereo4"))}
      if self.pereo5() != 0 { try!(write!(f, " pereo5"))}
      if self.pereo6() != 0 { try!(write!(f, " pereo6"))}
      if self.pereo7() != 0 { try!(write!(f, " pereo7"))}
      if self.cmpeo0() != 0 { try!(write!(f, " cmpeo0"))}
      if self.ovfeo() != 0 { try!(write!(f, " ovfeo"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Intenclr(pub u8);

impl Intenclr {
  pub fn cmp0(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_cmp0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
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
      if self.cmp0() != 0 { try!(write!(f, " cmp0"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Intenset(pub u8);

impl Intenset {
  pub fn cmp0(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_cmp0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
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
      if self.cmp0() != 0 { try!(write!(f, " cmp0"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Intflag(pub u8);

impl Intflag {
  pub fn cmp0(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_cmp0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
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
      if self.cmp0() != 0 { try!(write!(f, " cmp0"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Readreq(pub u16);

impl Readreq {
  pub fn addr(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x3f // [5:0]
  }
  pub fn set_addr(mut self, value: u16) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 0);
     self.0 |= value << 0;
     self
  }

  pub fn rcont(&self) -> u16 {
     ((self.0 as u16) >> 14) & 0x1 // [14]
  }
  pub fn set_rcont(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn rreq(&self) -> u16 {
     ((self.0 as u16) >> 15) & 0x1 // [15]
  }
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
  pub fn syncbusy(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
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
      if self.syncbusy() != 0 { try!(write!(f, " syncbusy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

}
// End of mode0

pub mod mode1 {
   #[derive(Clone, Copy, PartialEq, Eq)]
   pub struct Mode1(pub u32);

impl Mode1 {
  pub unsafe fn dbgctrl(&self) -> Dbgctrl { 
     Dbgctrl(::core::ptr::read_volatile(((self.0 as usize) + 0xb) as *const u8))
  }
  pub unsafe fn set_dbgctrl(&mut self, value: Dbgctrl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xb) as *mut u8, value.0);
  }
  pub unsafe fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&mut self, f: F) {
     let tmp = self.dbgctrl();
     self.set_dbgctrl(f(tmp))
  }

  pub unsafe fn freqcorr(&self) -> Freqcorr { 
     Freqcorr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u8))
  }
  pub unsafe fn set_freqcorr(&mut self, value: Freqcorr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u8, value.0);
  }
  pub unsafe fn with_freqcorr<F: FnOnce(Freqcorr) -> Freqcorr>(&mut self, f: F) {
     let tmp = self.freqcorr();
     self.set_freqcorr(f(tmp))
  }

  pub unsafe fn comp(&self, index: usize) -> Comp { 
     assert!(index < 2);
     Comp(::core::ptr::read_volatile(((self.0 as usize) + 0x18 + (index << 1)) as *const u16))
  }
  pub unsafe fn set_comp(&mut self, index: usize, value: Comp) {
     assert!(index < 2);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x18 + (index << 1)) as *mut u16, value.0);
  }
  pub unsafe fn with_comp<F: FnOnce(Comp) -> Comp>(&mut self, index: usize, f: F) {
     let tmp = self.comp(index);
     self.set_comp(index, f(tmp))
  }

  pub unsafe fn count(&self) -> Count { 
     Count(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u16))
  }
  pub unsafe fn set_count(&mut self, value: Count) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u16, value.0);
  }
  pub unsafe fn with_count<F: FnOnce(Count) -> Count>(&mut self, f: F) {
     let tmp = self.count();
     self.set_count(f(tmp))
  }

  pub unsafe fn ctrl(&self) -> Ctrl { 
     Ctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u16))
  }
  pub unsafe fn set_ctrl(&mut self, value: Ctrl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u16, value.0);
  }
  pub unsafe fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&mut self, f: F) {
     let tmp = self.ctrl();
     self.set_ctrl(f(tmp))
  }

  pub unsafe fn evctrl(&self) -> Evctrl { 
     Evctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u16))
  }
  pub unsafe fn set_evctrl(&mut self, value: Evctrl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u16, value.0);
  }
  pub unsafe fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&mut self, f: F) {
     let tmp = self.evctrl();
     self.set_evctrl(f(tmp))
  }

  pub unsafe fn intenclr(&self) -> Intenclr { 
     Intenclr(::core::ptr::read_volatile(((self.0 as usize) + 0x6) as *const u8))
  }
  pub unsafe fn set_intenclr(&mut self, value: Intenclr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x6) as *mut u8, value.0);
  }
  pub unsafe fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&mut self, f: F) {
     let tmp = self.intenclr();
     self.set_intenclr(f(tmp))
  }

  pub unsafe fn intenset(&self) -> Intenset { 
     Intenset(::core::ptr::read_volatile(((self.0 as usize) + 0x7) as *const u8))
  }
  pub unsafe fn set_intenset(&mut self, value: Intenset) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x7) as *mut u8, value.0);
  }
  pub unsafe fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&mut self, f: F) {
     let tmp = self.intenset();
     self.set_intenset(f(tmp))
  }

  pub unsafe fn intflag(&self) -> Intflag { 
     Intflag(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u8))
  }
  pub unsafe fn set_intflag(&mut self, value: Intflag) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u8, value.0);
  }
  pub unsafe fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&mut self, f: F) {
     let tmp = self.intflag();
     self.set_intflag(f(tmp))
  }

  pub unsafe fn per(&self) -> Per { 
     Per(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u16))
  }
  pub unsafe fn set_per(&mut self, value: Per) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u16, value.0);
  }
  pub unsafe fn with_per<F: FnOnce(Per) -> Per>(&mut self, f: F) {
     let tmp = self.per();
     self.set_per(f(tmp))
  }

  pub unsafe fn readreq(&self) -> Readreq { 
     Readreq(::core::ptr::read_volatile(((self.0 as usize) + 0x2) as *const u16))
  }
  pub unsafe fn set_readreq(&mut self, value: Readreq) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x2) as *mut u16, value.0);
  }
  pub unsafe fn with_readreq<F: FnOnce(Readreq) -> Readreq>(&mut self, f: F) {
     let tmp = self.readreq();
     self.set_readreq(f(tmp))
  }

  pub unsafe fn status(&self) -> Status { 
     Status(::core::ptr::read_volatile(((self.0 as usize) + 0xa) as *const u8))
  }
  pub unsafe fn set_status(&mut self, value: Status) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa) as *mut u8, value.0);
  }
  pub unsafe fn with_status<F: FnOnce(Status) -> Status>(&mut self, f: F) {
     let tmp = self.status();
     self.set_status(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Dbgctrl(pub u8);

impl Dbgctrl {
  pub fn dbgrun(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
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
pub struct Freqcorr(pub u8);

impl Freqcorr {
  pub fn value(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x7f // [6:0]
  }
  pub fn set_value(mut self, value: u8) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 0);
     self.0 |= value << 0;
     self
  }

  pub fn sign(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_sign(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for Freqcorr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Freqcorr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.value() != 0 { try!(write!(f, " value=0x{:x}", self.value()))}
      if self.sign() != 0 { try!(write!(f, " sign"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Comp(pub u16);

impl Comp {
  pub fn comp(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
  pub fn set_comp(mut self, value: u16) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Comp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Comp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.comp() != 0 { try!(write!(f, " comp=0x{:x}", self.comp()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Count(pub u16);

impl Count {
  pub fn count(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
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
pub struct Ctrl(pub u16);

impl Ctrl {
  pub fn swrst(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
  pub fn set_swrst(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn enable(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
  pub fn set_enable(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn mode(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x3 // [3:2]
  }
  pub fn set_mode(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn prescaler(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0xf // [11:8]
  }
  pub fn set_prescaler(mut self, value: u16) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

}

impl ::core::fmt::Display for Ctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swrst() != 0 { try!(write!(f, " swrst"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
      if self.prescaler() != 0 { try!(write!(f, " prescaler=0x{:x}", self.prescaler()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Evctrl(pub u16);

impl Evctrl {
  pub fn pereo0(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
  pub fn set_pereo0(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn pereo1(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
  pub fn set_pereo1(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn pereo2(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
  pub fn set_pereo2(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn pereo3(&self) -> u16 {
     ((self.0 as u16) >> 3) & 0x1 // [3]
  }
  pub fn set_pereo3(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn pereo4(&self) -> u16 {
     ((self.0 as u16) >> 4) & 0x1 // [4]
  }
  pub fn set_pereo4(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn pereo5(&self) -> u16 {
     ((self.0 as u16) >> 5) & 0x1 // [5]
  }
  pub fn set_pereo5(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn pereo6(&self) -> u16 {
     ((self.0 as u16) >> 6) & 0x1 // [6]
  }
  pub fn set_pereo6(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn pereo7(&self) -> u16 {
     ((self.0 as u16) >> 7) & 0x1 // [7]
  }
  pub fn set_pereo7(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn cmpeo0(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x1 // [8]
  }
  pub fn set_cmpeo0(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn cmpeo1(&self) -> u16 {
     ((self.0 as u16) >> 9) & 0x1 // [9]
  }
  pub fn set_cmpeo1(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn ovfeo(&self) -> u16 {
     ((self.0 as u16) >> 15) & 0x1 // [15]
  }
  pub fn set_ovfeo(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
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
      if self.pereo0() != 0 { try!(write!(f, " pereo0"))}
      if self.pereo1() != 0 { try!(write!(f, " pereo1"))}
      if self.pereo2() != 0 { try!(write!(f, " pereo2"))}
      if self.pereo3() != 0 { try!(write!(f, " pereo3"))}
      if self.pereo4() != 0 { try!(write!(f, " pereo4"))}
      if self.pereo5() != 0 { try!(write!(f, " pereo5"))}
      if self.pereo6() != 0 { try!(write!(f, " pereo6"))}
      if self.pereo7() != 0 { try!(write!(f, " pereo7"))}
      if self.cmpeo0() != 0 { try!(write!(f, " cmpeo0"))}
      if self.cmpeo1() != 0 { try!(write!(f, " cmpeo1"))}
      if self.ovfeo() != 0 { try!(write!(f, " ovfeo"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Intenclr(pub u8);

impl Intenclr {
  pub fn cmp0(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_cmp0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn cmp1(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_cmp1(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
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
      if self.cmp0() != 0 { try!(write!(f, " cmp0"))}
      if self.cmp1() != 0 { try!(write!(f, " cmp1"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Intenset(pub u8);

impl Intenset {
  pub fn cmp0(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_cmp0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn cmp1(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_cmp1(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
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
      if self.cmp0() != 0 { try!(write!(f, " cmp0"))}
      if self.cmp1() != 0 { try!(write!(f, " cmp1"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Intflag(pub u8);

impl Intflag {
  pub fn cmp0(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_cmp0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn cmp1(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_cmp1(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
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
      if self.cmp0() != 0 { try!(write!(f, " cmp0"))}
      if self.cmp1() != 0 { try!(write!(f, " cmp1"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Per(pub u16);

impl Per {
  pub fn per(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
  pub fn set_per(mut self, value: u16) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
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
pub struct Readreq(pub u16);

impl Readreq {
  pub fn addr(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x3f // [5:0]
  }
  pub fn set_addr(mut self, value: u16) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 0);
     self.0 |= value << 0;
     self
  }

  pub fn rcont(&self) -> u16 {
     ((self.0 as u16) >> 14) & 0x1 // [14]
  }
  pub fn set_rcont(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn rreq(&self) -> u16 {
     ((self.0 as u16) >> 15) & 0x1 // [15]
  }
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
  pub fn syncbusy(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
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
      if self.syncbusy() != 0 { try!(write!(f, " syncbusy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

}
// End of mode1

pub mod mode2 {
   #[derive(Clone, Copy, PartialEq, Eq)]
   pub struct Mode2(pub u32);

impl Mode2 {
  pub unsafe fn dbgctrl(&self) -> Dbgctrl { 
     Dbgctrl(::core::ptr::read_volatile(((self.0 as usize) + 0xb) as *const u8))
  }
  pub unsafe fn set_dbgctrl(&mut self, value: Dbgctrl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xb) as *mut u8, value.0);
  }
  pub unsafe fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&mut self, f: F) {
     let tmp = self.dbgctrl();
     self.set_dbgctrl(f(tmp))
  }

  pub unsafe fn freqcorr(&self) -> Freqcorr { 
     Freqcorr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u8))
  }
  pub unsafe fn set_freqcorr(&mut self, value: Freqcorr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u8, value.0);
  }
  pub unsafe fn with_freqcorr<F: FnOnce(Freqcorr) -> Freqcorr>(&mut self, f: F) {
     let tmp = self.freqcorr();
     self.set_freqcorr(f(tmp))
  }

  pub unsafe fn clock(&self) -> Clock { 
     Clock(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
  }
  pub unsafe fn set_clock(&mut self, value: Clock) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
  }
  pub unsafe fn with_clock<F: FnOnce(Clock) -> Clock>(&mut self, f: F) {
     let tmp = self.clock();
     self.set_clock(f(tmp))
  }

  pub unsafe fn ctrl(&self) -> Ctrl { 
     Ctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u16))
  }
  pub unsafe fn set_ctrl(&mut self, value: Ctrl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u16, value.0);
  }
  pub unsafe fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&mut self, f: F) {
     let tmp = self.ctrl();
     self.set_ctrl(f(tmp))
  }

  pub unsafe fn evctrl(&self) -> Evctrl { 
     Evctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u16))
  }
  pub unsafe fn set_evctrl(&mut self, value: Evctrl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u16, value.0);
  }
  pub unsafe fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&mut self, f: F) {
     let tmp = self.evctrl();
     self.set_evctrl(f(tmp))
  }

  pub unsafe fn intenclr(&self) -> Intenclr { 
     Intenclr(::core::ptr::read_volatile(((self.0 as usize) + 0x6) as *const u8))
  }
  pub unsafe fn set_intenclr(&mut self, value: Intenclr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x6) as *mut u8, value.0);
  }
  pub unsafe fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&mut self, f: F) {
     let tmp = self.intenclr();
     self.set_intenclr(f(tmp))
  }

  pub unsafe fn intenset(&self) -> Intenset { 
     Intenset(::core::ptr::read_volatile(((self.0 as usize) + 0x7) as *const u8))
  }
  pub unsafe fn set_intenset(&mut self, value: Intenset) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x7) as *mut u8, value.0);
  }
  pub unsafe fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&mut self, f: F) {
     let tmp = self.intenset();
     self.set_intenset(f(tmp))
  }

  pub unsafe fn intflag(&self) -> Intflag { 
     Intflag(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u8))
  }
  pub unsafe fn set_intflag(&mut self, value: Intflag) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u8, value.0);
  }
  pub unsafe fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&mut self, f: F) {
     let tmp = self.intflag();
     self.set_intflag(f(tmp))
  }

  pub unsafe fn alarm(&self, index: usize) -> Alarm { 
     assert!(index < 1);
     Alarm(::core::ptr::read_volatile(((self.0 as usize) + 0x18 + (index << 3)) as *const u32))
  }
  pub unsafe fn set_alarm(&mut self, index: usize, value: Alarm) {
     assert!(index < 1);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x18 + (index << 3)) as *mut u32, value.0);
  }
  pub unsafe fn with_alarm<F: FnOnce(Alarm) -> Alarm>(&mut self, index: usize, f: F) {
     let tmp = self.alarm(index);
     self.set_alarm(index, f(tmp))
  }

  pub unsafe fn mask(&self, index: usize) -> Mask { 
     assert!(index < 1);
     Mask(::core::ptr::read_volatile(((self.0 as usize) + 0x1c + (index << 3)) as *const u8))
  }
  pub unsafe fn set_mask(&mut self, index: usize, value: Mask) {
     assert!(index < 1);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1c + (index << 3)) as *mut u8, value.0);
  }
  pub unsafe fn with_mask<F: FnOnce(Mask) -> Mask>(&mut self, index: usize, f: F) {
     let tmp = self.mask(index);
     self.set_mask(index, f(tmp))
  }

  pub unsafe fn readreq(&self) -> Readreq { 
     Readreq(::core::ptr::read_volatile(((self.0 as usize) + 0x2) as *const u16))
  }
  pub unsafe fn set_readreq(&mut self, value: Readreq) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x2) as *mut u16, value.0);
  }
  pub unsafe fn with_readreq<F: FnOnce(Readreq) -> Readreq>(&mut self, f: F) {
     let tmp = self.readreq();
     self.set_readreq(f(tmp))
  }

  pub unsafe fn status(&self) -> Status { 
     Status(::core::ptr::read_volatile(((self.0 as usize) + 0xa) as *const u8))
  }
  pub unsafe fn set_status(&mut self, value: Status) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa) as *mut u8, value.0);
  }
  pub unsafe fn with_status<F: FnOnce(Status) -> Status>(&mut self, f: F) {
     let tmp = self.status();
     self.set_status(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Dbgctrl(pub u8);

impl Dbgctrl {
  pub fn dbgrun(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
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
pub struct Freqcorr(pub u8);

impl Freqcorr {
  pub fn value(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x7f // [6:0]
  }
  pub fn set_value(mut self, value: u8) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 0);
     self.0 |= value << 0;
     self
  }

  pub fn sign(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_sign(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for Freqcorr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Freqcorr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.value() != 0 { try!(write!(f, " value=0x{:x}", self.value()))}
      if self.sign() != 0 { try!(write!(f, " sign"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Clock(pub u32);

impl Clock {
  pub fn second(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3f // [5:0]
  }
  pub fn set_second(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 0);
     self.0 |= value << 0;
     self
  }

  pub fn minute(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3f // [11:6]
  }
  pub fn set_minute(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 6);
     self.0 |= value << 6;
     self
  }

  pub fn hour(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1f // [16:12]
  }
  pub fn set_hour(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 12);
     self.0 |= value << 12;
     self
  }

  pub fn day(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1f // [21:17]
  }
  pub fn set_day(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 17);
     self.0 |= value << 17;
     self
  }

  pub fn month(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0xf // [25:22]
  }
  pub fn set_month(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 22);
     self.0 |= value << 22;
     self
  }

  pub fn year(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x3f // [31:26]
  }
  pub fn set_year(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 26);
     self.0 |= value << 26;
     self
  }

}

impl ::core::fmt::Display for Clock {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Clock {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.second() != 0 { try!(write!(f, " second=0x{:x}", self.second()))}
      if self.minute() != 0 { try!(write!(f, " minute=0x{:x}", self.minute()))}
      if self.hour() != 0 { try!(write!(f, " hour=0x{:x}", self.hour()))}
      if self.day() != 0 { try!(write!(f, " day=0x{:x}", self.day()))}
      if self.month() != 0 { try!(write!(f, " month=0x{:x}", self.month()))}
      if self.year() != 0 { try!(write!(f, " year=0x{:x}", self.year()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ctrl(pub u16);

impl Ctrl {
  pub fn swrst(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
  pub fn set_swrst(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn enable(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
  pub fn set_enable(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn mode(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x3 // [3:2]
  }
  pub fn set_mode(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn clkrep(&self) -> u16 {
     ((self.0 as u16) >> 6) & 0x1 // [6]
  }
  pub fn set_clkrep(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn matchclr(&self) -> u16 {
     ((self.0 as u16) >> 7) & 0x1 // [7]
  }
  pub fn set_matchclr(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn prescaler(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0xf // [11:8]
  }
  pub fn set_prescaler(mut self, value: u16) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

}

impl ::core::fmt::Display for Ctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swrst() != 0 { try!(write!(f, " swrst"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
      if self.clkrep() != 0 { try!(write!(f, " clkrep"))}
      if self.matchclr() != 0 { try!(write!(f, " matchclr"))}
      if self.prescaler() != 0 { try!(write!(f, " prescaler=0x{:x}", self.prescaler()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Evctrl(pub u16);

impl Evctrl {
  pub fn pereo0(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
  pub fn set_pereo0(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn pereo1(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
  pub fn set_pereo1(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn pereo2(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
  pub fn set_pereo2(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn pereo3(&self) -> u16 {
     ((self.0 as u16) >> 3) & 0x1 // [3]
  }
  pub fn set_pereo3(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn pereo4(&self) -> u16 {
     ((self.0 as u16) >> 4) & 0x1 // [4]
  }
  pub fn set_pereo4(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn pereo5(&self) -> u16 {
     ((self.0 as u16) >> 5) & 0x1 // [5]
  }
  pub fn set_pereo5(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn pereo6(&self) -> u16 {
     ((self.0 as u16) >> 6) & 0x1 // [6]
  }
  pub fn set_pereo6(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn pereo7(&self) -> u16 {
     ((self.0 as u16) >> 7) & 0x1 // [7]
  }
  pub fn set_pereo7(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn alarmeo0(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x1 // [8]
  }
  pub fn set_alarmeo0(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn ovfeo(&self) -> u16 {
     ((self.0 as u16) >> 15) & 0x1 // [15]
  }
  pub fn set_ovfeo(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
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
      if self.pereo0() != 0 { try!(write!(f, " pereo0"))}
      if self.pereo1() != 0 { try!(write!(f, " pereo1"))}
      if self.pereo2() != 0 { try!(write!(f, " pereo2"))}
      if self.pereo3() != 0 { try!(write!(f, " pereo3"))}
      if self.pereo4() != 0 { try!(write!(f, " pereo4"))}
      if self.pereo5() != 0 { try!(write!(f, " pereo5"))}
      if self.pereo6() != 0 { try!(write!(f, " pereo6"))}
      if self.pereo7() != 0 { try!(write!(f, " pereo7"))}
      if self.alarmeo0() != 0 { try!(write!(f, " alarmeo0"))}
      if self.ovfeo() != 0 { try!(write!(f, " ovfeo"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Intenclr(pub u8);

impl Intenclr {
  pub fn alarm0(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_alarm0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
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
      if self.alarm0() != 0 { try!(write!(f, " alarm0"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Intenset(pub u8);

impl Intenset {
  pub fn alarm0(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_alarm0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
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
      if self.alarm0() != 0 { try!(write!(f, " alarm0"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Intflag(pub u8);

impl Intflag {
  pub fn alarm0(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_alarm0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
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
      if self.alarm0() != 0 { try!(write!(f, " alarm0"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Alarm(pub u32);

impl Alarm {
  pub fn second(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3f // [5:0]
  }
  pub fn set_second(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 0);
     self.0 |= value << 0;
     self
  }

  pub fn minute(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3f // [11:6]
  }
  pub fn set_minute(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 6);
     self.0 |= value << 6;
     self
  }

  pub fn hour(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1f // [16:12]
  }
  pub fn set_hour(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 12);
     self.0 |= value << 12;
     self
  }

  pub fn day(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1f // [21:17]
  }
  pub fn set_day(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 17);
     self.0 |= value << 17;
     self
  }

  pub fn month(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0xf // [25:22]
  }
  pub fn set_month(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 22);
     self.0 |= value << 22;
     self
  }

  pub fn year(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x3f // [31:26]
  }
  pub fn set_year(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 26);
     self.0 |= value << 26;
     self
  }

}

impl ::core::fmt::Display for Alarm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Alarm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.second() != 0 { try!(write!(f, " second=0x{:x}", self.second()))}
      if self.minute() != 0 { try!(write!(f, " minute=0x{:x}", self.minute()))}
      if self.hour() != 0 { try!(write!(f, " hour=0x{:x}", self.hour()))}
      if self.day() != 0 { try!(write!(f, " day=0x{:x}", self.day()))}
      if self.month() != 0 { try!(write!(f, " month=0x{:x}", self.month()))}
      if self.year() != 0 { try!(write!(f, " year=0x{:x}", self.year()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Mask(pub u8);

impl Mask {
  pub fn sel(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x7 // [2:0]
  }
  pub fn set_sel(mut self, value: u8) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Mask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Mask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sel() != 0 { try!(write!(f, " sel=0x{:x}", self.sel()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Readreq(pub u16);

impl Readreq {
  pub fn addr(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x3f // [5:0]
  }
  pub fn set_addr(mut self, value: u16) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 0);
     self.0 |= value << 0;
     self
  }

  pub fn rcont(&self) -> u16 {
     ((self.0 as u16) >> 14) & 0x1 // [14]
  }
  pub fn set_rcont(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn rreq(&self) -> u16 {
     ((self.0 as u16) >> 15) & 0x1 // [15]
  }
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
  pub fn syncbusy(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
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
      if self.syncbusy() != 0 { try!(write!(f, " syncbusy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

}
// End of mode2

