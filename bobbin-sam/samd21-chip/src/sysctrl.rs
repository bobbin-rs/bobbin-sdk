pub const SYSCTRL: Sysctrl = Sysctrl(0x40000800);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sysctrl(pub u32);

impl Sysctrl {
  pub unsafe fn bod33(&self) -> Bod33 { 
     Bod33(::core::ptr::read_volatile(((self.0 as usize) + 0x34) as *const u32))
  }
  pub unsafe fn set_bod33(&mut self, value: Bod33) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x34) as *mut u32, value.0);
  }
  pub unsafe fn with_bod33<F: FnOnce(Bod33) -> Bod33>(&mut self, f: F) {
     let tmp = self.bod33();
     self.set_bod33(f(tmp))
  }

  pub unsafe fn dfllctrl(&self) -> Dfllctrl { 
     Dfllctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u16))
  }
  pub unsafe fn set_dfllctrl(&mut self, value: Dfllctrl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u16, value.0);
  }
  pub unsafe fn with_dfllctrl<F: FnOnce(Dfllctrl) -> Dfllctrl>(&mut self, f: F) {
     let tmp = self.dfllctrl();
     self.set_dfllctrl(f(tmp))
  }

  pub unsafe fn dfllmul(&self) -> Dfllmul { 
     Dfllmul(::core::ptr::read_volatile(((self.0 as usize) + 0x2c) as *const u32))
  }
  pub unsafe fn set_dfllmul(&mut self, value: Dfllmul) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x2c) as *mut u32, value.0);
  }
  pub unsafe fn with_dfllmul<F: FnOnce(Dfllmul) -> Dfllmul>(&mut self, f: F) {
     let tmp = self.dfllmul();
     self.set_dfllmul(f(tmp))
  }

  pub unsafe fn dfllsync(&self) -> Dfllsync { 
     Dfllsync(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u8))
  }
  pub unsafe fn set_dfllsync(&mut self, value: Dfllsync) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u8, value.0);
  }
  pub unsafe fn with_dfllsync<F: FnOnce(Dfllsync) -> Dfllsync>(&mut self, f: F) {
     let tmp = self.dfllsync();
     self.set_dfllsync(f(tmp))
  }

  pub unsafe fn dfllval(&self) -> Dfllval { 
     Dfllval(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
  }
  pub unsafe fn set_dfllval(&mut self, value: Dfllval) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
  }
  pub unsafe fn with_dfllval<F: FnOnce(Dfllval) -> Dfllval>(&mut self, f: F) {
     let tmp = self.dfllval();
     self.set_dfllval(f(tmp))
  }

  pub unsafe fn dpllctrla(&self) -> Dpllctrla { 
     Dpllctrla(::core::ptr::read_volatile(((self.0 as usize) + 0x44) as *const u8))
  }
  pub unsafe fn set_dpllctrla(&mut self, value: Dpllctrla) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x44) as *mut u8, value.0);
  }
  pub unsafe fn with_dpllctrla<F: FnOnce(Dpllctrla) -> Dpllctrla>(&mut self, f: F) {
     let tmp = self.dpllctrla();
     self.set_dpllctrla(f(tmp))
  }

  pub unsafe fn dpllctrlb(&self) -> Dpllctrlb { 
     Dpllctrlb(::core::ptr::read_volatile(((self.0 as usize) + 0x4c) as *const u32))
  }
  pub unsafe fn set_dpllctrlb(&mut self, value: Dpllctrlb) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4c) as *mut u32, value.0);
  }
  pub unsafe fn with_dpllctrlb<F: FnOnce(Dpllctrlb) -> Dpllctrlb>(&mut self, f: F) {
     let tmp = self.dpllctrlb();
     self.set_dpllctrlb(f(tmp))
  }

  pub unsafe fn dpllratio(&self) -> Dpllratio { 
     Dpllratio(::core::ptr::read_volatile(((self.0 as usize) + 0x48) as *const u32))
  }
  pub unsafe fn set_dpllratio(&mut self, value: Dpllratio) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x48) as *mut u32, value.0);
  }
  pub unsafe fn with_dpllratio<F: FnOnce(Dpllratio) -> Dpllratio>(&mut self, f: F) {
     let tmp = self.dpllratio();
     self.set_dpllratio(f(tmp))
  }

  pub unsafe fn dpllstatus(&self) -> Dpllstatus { 
     Dpllstatus(::core::ptr::read_volatile(((self.0 as usize) + 0x50) as *const u8))
  }

  pub unsafe fn intenclr(&self) -> Intenclr { 
     Intenclr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
  }
  pub unsafe fn set_intenclr(&mut self, value: Intenclr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
  }
  pub unsafe fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&mut self, f: F) {
     let tmp = self.intenclr();
     self.set_intenclr(f(tmp))
  }

  pub unsafe fn intenset(&self) -> Intenset { 
     Intenset(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
  }
  pub unsafe fn set_intenset(&mut self, value: Intenset) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
  }
  pub unsafe fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&mut self, f: F) {
     let tmp = self.intenset();
     self.set_intenset(f(tmp))
  }

  pub unsafe fn intflag(&self) -> Intflag { 
     Intflag(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
  }
  pub unsafe fn set_intflag(&mut self, value: Intflag) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
  }
  pub unsafe fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&mut self, f: F) {
     let tmp = self.intflag();
     self.set_intflag(f(tmp))
  }

  pub unsafe fn osculp32k(&self) -> Osculp32k { 
     Osculp32k(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u8))
  }
  pub unsafe fn set_osculp32k(&mut self, value: Osculp32k) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u8, value.0);
  }
  pub unsafe fn with_osculp32k<F: FnOnce(Osculp32k) -> Osculp32k>(&mut self, f: F) {
     let tmp = self.osculp32k();
     self.set_osculp32k(f(tmp))
  }

  pub unsafe fn osc8m(&self) -> Osc8m { 
     Osc8m(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
  }
  pub unsafe fn set_osc8m(&mut self, value: Osc8m) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
  }
  pub unsafe fn with_osc8m<F: FnOnce(Osc8m) -> Osc8m>(&mut self, f: F) {
     let tmp = self.osc8m();
     self.set_osc8m(f(tmp))
  }

  pub unsafe fn osc32k(&self) -> Osc32k { 
     Osc32k(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
  }
  pub unsafe fn set_osc32k(&mut self, value: Osc32k) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
  }
  pub unsafe fn with_osc32k<F: FnOnce(Osc32k) -> Osc32k>(&mut self, f: F) {
     let tmp = self.osc32k();
     self.set_osc32k(f(tmp))
  }

  pub unsafe fn pclksr(&self) -> Pclksr { 
     Pclksr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
  }

  pub unsafe fn vref(&self) -> Vref { 
     Vref(::core::ptr::read_volatile(((self.0 as usize) + 0x40) as *const u32))
  }
  pub unsafe fn set_vref(&mut self, value: Vref) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u32, value.0);
  }
  pub unsafe fn with_vref<F: FnOnce(Vref) -> Vref>(&mut self, f: F) {
     let tmp = self.vref();
     self.set_vref(f(tmp))
  }

  pub unsafe fn xosc(&self) -> Xosc { 
     Xosc(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u16))
  }
  pub unsafe fn set_xosc(&mut self, value: Xosc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u16, value.0);
  }
  pub unsafe fn with_xosc<F: FnOnce(Xosc) -> Xosc>(&mut self, f: F) {
     let tmp = self.xosc();
     self.set_xosc(f(tmp))
  }

  pub unsafe fn xosc32k(&self) -> Xosc32k { 
     Xosc32k(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u16))
  }
  pub unsafe fn set_xosc32k(&mut self, value: Xosc32k) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u16, value.0);
  }
  pub unsafe fn with_xosc32k<F: FnOnce(Xosc32k) -> Xosc32k>(&mut self, f: F) {
     let tmp = self.xosc32k();
     self.set_xosc32k(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Bod33(pub u32);

impl Bod33 {
  pub fn enable(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn hyst(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_hyst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn action(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x3 // [4:3]
  }
  pub fn set_action(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn runstdby(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_runstdby(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn mode(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_mode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn cen(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_cen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn psel(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0xf // [15:12]
  }
  pub fn set_psel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

  pub fn level(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3f // [21:16]
  }
  pub fn set_level(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 16);
     self.0 |= value << 16;
     self
  }

}

impl ::core::fmt::Display for Bod33 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Bod33 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.hyst() != 0 { try!(write!(f, " hyst"))}
      if self.action() != 0 { try!(write!(f, " action=0x{:x}", self.action()))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      if self.mode() != 0 { try!(write!(f, " mode"))}
      if self.cen() != 0 { try!(write!(f, " cen"))}
      if self.psel() != 0 { try!(write!(f, " psel=0x{:x}", self.psel()))}
      if self.level() != 0 { try!(write!(f, " level=0x{:x}", self.level()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dfllctrl(pub u16);

impl Dfllctrl {
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
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
  pub fn set_mode(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn stable(&self) -> u16 {
     ((self.0 as u16) >> 3) & 0x1 // [3]
  }
  pub fn set_stable(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn llaw(&self) -> u16 {
     ((self.0 as u16) >> 4) & 0x1 // [4]
  }
  pub fn set_llaw(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn usbcrm(&self) -> u16 {
     ((self.0 as u16) >> 5) & 0x1 // [5]
  }
  pub fn set_usbcrm(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn runstdby(&self) -> u16 {
     ((self.0 as u16) >> 6) & 0x1 // [6]
  }
  pub fn set_runstdby(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn ondemand(&self) -> u16 {
     ((self.0 as u16) >> 7) & 0x1 // [7]
  }
  pub fn set_ondemand(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn ccdis(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x1 // [8]
  }
  pub fn set_ccdis(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn qldis(&self) -> u16 {
     ((self.0 as u16) >> 9) & 0x1 // [9]
  }
  pub fn set_qldis(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn bplckc(&self) -> u16 {
     ((self.0 as u16) >> 10) & 0x1 // [10]
  }
  pub fn set_bplckc(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn waitlock(&self) -> u16 {
     ((self.0 as u16) >> 11) & 0x1 // [11]
  }
  pub fn set_waitlock(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

}

impl ::core::fmt::Display for Dfllctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dfllctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.mode() != 0 { try!(write!(f, " mode"))}
      if self.stable() != 0 { try!(write!(f, " stable"))}
      if self.llaw() != 0 { try!(write!(f, " llaw"))}
      if self.usbcrm() != 0 { try!(write!(f, " usbcrm"))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      if self.ondemand() != 0 { try!(write!(f, " ondemand"))}
      if self.ccdis() != 0 { try!(write!(f, " ccdis"))}
      if self.qldis() != 0 { try!(write!(f, " qldis"))}
      if self.bplckc() != 0 { try!(write!(f, " bplckc"))}
      if self.waitlock() != 0 { try!(write!(f, " waitlock"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dfllmul(pub u32);

impl Dfllmul {
  pub fn mul(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_mul(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn fstep(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3ff // [25:16]
  }
  pub fn set_fstep(mut self, value: u32) -> Self {
     assert!((value & !0x3ff) == 0);
     self.0 &= !(0x3ff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn cstep(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x3f // [31:26]
  }
  pub fn set_cstep(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 26);
     self.0 |= value << 26;
     self
  }

}

impl ::core::fmt::Display for Dfllmul {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dfllmul {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mul() != 0 { try!(write!(f, " mul=0x{:x}", self.mul()))}
      if self.fstep() != 0 { try!(write!(f, " fstep=0x{:x}", self.fstep()))}
      if self.cstep() != 0 { try!(write!(f, " cstep=0x{:x}", self.cstep()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dfllsync(pub u8);

impl Dfllsync {
  pub fn readreq(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_readreq(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for Dfllsync {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dfllsync {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.readreq() != 0 { try!(write!(f, " readreq"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dfllval(pub u32);

impl Dfllval {
  pub fn fine(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3ff // [9:0]
  }
  pub fn set_fine(mut self, value: u32) -> Self {
     assert!((value & !0x3ff) == 0);
     self.0 &= !(0x3ff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn coarse(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3f // [15:10]
  }
  pub fn set_coarse(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 10);
     self.0 |= value << 10;
     self
  }

  pub fn diff(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xffff // [31:16]
  }
  pub fn set_diff(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 16);
     self.0 |= value << 16;
     self
  }

}

impl ::core::fmt::Display for Dfllval {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dfllval {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fine() != 0 { try!(write!(f, " fine=0x{:x}", self.fine()))}
      if self.coarse() != 0 { try!(write!(f, " coarse=0x{:x}", self.coarse()))}
      if self.diff() != 0 { try!(write!(f, " diff=0x{:x}", self.diff()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dpllctrla(pub u8);

impl Dpllctrla {
  pub fn enable(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_enable(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn runstdby(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_runstdby(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn ondemand(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_ondemand(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for Dpllctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dpllctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      if self.ondemand() != 0 { try!(write!(f, " ondemand"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dpllctrlb(pub u32);

impl Dpllctrlb {
  pub fn filter(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  pub fn set_filter(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn lpen(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn wuf(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_wuf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn refclk(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  pub fn set_refclk(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn ltime(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x7 // [10:8]
  }
  pub fn set_ltime(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn lbypass(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_lbypass(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn div(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x7ff // [26:16]
  }
  pub fn set_div(mut self, value: u32) -> Self {
     assert!((value & !0x7ff) == 0);
     self.0 &= !(0x7ff << 16);
     self.0 |= value << 16;
     self
  }

}

impl ::core::fmt::Display for Dpllctrlb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dpllctrlb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.filter() != 0 { try!(write!(f, " filter=0x{:x}", self.filter()))}
      if self.lpen() != 0 { try!(write!(f, " lpen"))}
      if self.wuf() != 0 { try!(write!(f, " wuf"))}
      if self.refclk() != 0 { try!(write!(f, " refclk=0x{:x}", self.refclk()))}
      if self.ltime() != 0 { try!(write!(f, " ltime=0x{:x}", self.ltime()))}
      if self.lbypass() != 0 { try!(write!(f, " lbypass"))}
      if self.div() != 0 { try!(write!(f, " div=0x{:x}", self.div()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dpllratio(pub u32);

impl Dpllratio {
  pub fn ldr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  pub fn set_ldr(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn ldrfrac(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  pub fn set_ldrfrac(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

}

impl ::core::fmt::Display for Dpllratio {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dpllratio {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ldr() != 0 { try!(write!(f, " ldr=0x{:x}", self.ldr()))}
      if self.ldrfrac() != 0 { try!(write!(f, " ldrfrac=0x{:x}", self.ldrfrac()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dpllstatus(pub u8);

impl Dpllstatus {
  pub fn lock(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_lock(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn clkrdy(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_clkrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn enable(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  pub fn set_enable(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn div(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  pub fn set_div(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}

impl ::core::fmt::Display for Dpllstatus {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dpllstatus {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lock() != 0 { try!(write!(f, " lock"))}
      if self.clkrdy() != 0 { try!(write!(f, " clkrdy"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.div() != 0 { try!(write!(f, " div"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Intenclr(pub u32);

impl Intenclr {
  pub fn xoscrdy(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_xoscrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn xosc32krdy(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_xosc32krdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn osc32krdy(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_osc32krdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn osc8mrdy(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_osc8mrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn dfllrdy(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_dfllrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn dflloob(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_dflloob(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn dflllckf(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_dflllckf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn dflllckc(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_dflllckc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn dfllrcs(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_dfllrcs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn bod33rdy(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_bod33rdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn bod33det(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_bod33det(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn b33srdy(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_b33srdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn dplllckr(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_dplllckr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn dplllckf(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_dplllckf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn dplllto(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_dplllto(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
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
      if self.xoscrdy() != 0 { try!(write!(f, " xoscrdy"))}
      if self.xosc32krdy() != 0 { try!(write!(f, " xosc32krdy"))}
      if self.osc32krdy() != 0 { try!(write!(f, " osc32krdy"))}
      if self.osc8mrdy() != 0 { try!(write!(f, " osc8mrdy"))}
      if self.dfllrdy() != 0 { try!(write!(f, " dfllrdy"))}
      if self.dflloob() != 0 { try!(write!(f, " dflloob"))}
      if self.dflllckf() != 0 { try!(write!(f, " dflllckf"))}
      if self.dflllckc() != 0 { try!(write!(f, " dflllckc"))}
      if self.dfllrcs() != 0 { try!(write!(f, " dfllrcs"))}
      if self.bod33rdy() != 0 { try!(write!(f, " bod33rdy"))}
      if self.bod33det() != 0 { try!(write!(f, " bod33det"))}
      if self.b33srdy() != 0 { try!(write!(f, " b33srdy"))}
      if self.dplllckr() != 0 { try!(write!(f, " dplllckr"))}
      if self.dplllckf() != 0 { try!(write!(f, " dplllckf"))}
      if self.dplllto() != 0 { try!(write!(f, " dplllto"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Intenset(pub u32);

impl Intenset {
  pub fn xoscrdy(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_xoscrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn xosc32krdy(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_xosc32krdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn osc32krdy(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_osc32krdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn osc8mrdy(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_osc8mrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn dfllrdy(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_dfllrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn dflloob(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_dflloob(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn dflllckf(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_dflllckf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn dflllckc(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_dflllckc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn dfllrcs(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_dfllrcs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn bod33rdy(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_bod33rdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn bod33det(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_bod33det(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn b33srdy(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_b33srdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn dplllckr(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_dplllckr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn dplllckf(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_dplllckf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn dplllto(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_dplllto(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
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
      if self.xoscrdy() != 0 { try!(write!(f, " xoscrdy"))}
      if self.xosc32krdy() != 0 { try!(write!(f, " xosc32krdy"))}
      if self.osc32krdy() != 0 { try!(write!(f, " osc32krdy"))}
      if self.osc8mrdy() != 0 { try!(write!(f, " osc8mrdy"))}
      if self.dfllrdy() != 0 { try!(write!(f, " dfllrdy"))}
      if self.dflloob() != 0 { try!(write!(f, " dflloob"))}
      if self.dflllckf() != 0 { try!(write!(f, " dflllckf"))}
      if self.dflllckc() != 0 { try!(write!(f, " dflllckc"))}
      if self.dfllrcs() != 0 { try!(write!(f, " dfllrcs"))}
      if self.bod33rdy() != 0 { try!(write!(f, " bod33rdy"))}
      if self.bod33det() != 0 { try!(write!(f, " bod33det"))}
      if self.b33srdy() != 0 { try!(write!(f, " b33srdy"))}
      if self.dplllckr() != 0 { try!(write!(f, " dplllckr"))}
      if self.dplllckf() != 0 { try!(write!(f, " dplllckf"))}
      if self.dplllto() != 0 { try!(write!(f, " dplllto"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Intflag(pub u32);

impl Intflag {
  pub fn xoscrdy(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_xoscrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn xosc32krdy(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_xosc32krdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn osc32krdy(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_osc32krdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn osc8mrdy(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_osc8mrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn dfllrdy(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_dfllrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn dflloob(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_dflloob(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn dflllckf(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_dflllckf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn dflllckc(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_dflllckc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn dfllrcs(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_dfllrcs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn bod33rdy(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_bod33rdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn bod33det(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_bod33det(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn b33srdy(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_b33srdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn dplllckr(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_dplllckr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn dplllckf(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_dplllckf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn dplllto(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_dplllto(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
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
      if self.xoscrdy() != 0 { try!(write!(f, " xoscrdy"))}
      if self.xosc32krdy() != 0 { try!(write!(f, " xosc32krdy"))}
      if self.osc32krdy() != 0 { try!(write!(f, " osc32krdy"))}
      if self.osc8mrdy() != 0 { try!(write!(f, " osc8mrdy"))}
      if self.dfllrdy() != 0 { try!(write!(f, " dfllrdy"))}
      if self.dflloob() != 0 { try!(write!(f, " dflloob"))}
      if self.dflllckf() != 0 { try!(write!(f, " dflllckf"))}
      if self.dflllckc() != 0 { try!(write!(f, " dflllckc"))}
      if self.dfllrcs() != 0 { try!(write!(f, " dfllrcs"))}
      if self.bod33rdy() != 0 { try!(write!(f, " bod33rdy"))}
      if self.bod33det() != 0 { try!(write!(f, " bod33det"))}
      if self.b33srdy() != 0 { try!(write!(f, " b33srdy"))}
      if self.dplllckr() != 0 { try!(write!(f, " dplllckr"))}
      if self.dplllckf() != 0 { try!(write!(f, " dplllckf"))}
      if self.dplllto() != 0 { try!(write!(f, " dplllto"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Osculp32k(pub u8);

impl Osculp32k {
  pub fn calib(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1f // [4:0]
  }
  pub fn set_calib(mut self, value: u8) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

  pub fn wrtlock(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_wrtlock(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for Osculp32k {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Osculp32k {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.calib() != 0 { try!(write!(f, " calib=0x{:x}", self.calib()))}
      if self.wrtlock() != 0 { try!(write!(f, " wrtlock"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Osc8m(pub u32);

impl Osc8m {
  pub fn enable(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn runstdby(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_runstdby(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn ondemand(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_ondemand(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn presc(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  pub fn set_presc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn calib(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xfff // [27:16]
  }
  pub fn set_calib(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn frange(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x3 // [31:30]
  }
  pub fn set_frange(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 30);
     self.0 |= value << 30;
     self
  }

}

impl ::core::fmt::Display for Osc8m {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Osc8m {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      if self.ondemand() != 0 { try!(write!(f, " ondemand"))}
      if self.presc() != 0 { try!(write!(f, " presc=0x{:x}", self.presc()))}
      if self.calib() != 0 { try!(write!(f, " calib=0x{:x}", self.calib()))}
      if self.frange() != 0 { try!(write!(f, " frange=0x{:x}", self.frange()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Osc32k(pub u32);

impl Osc32k {
  pub fn enable(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn en32k(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_en32k(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn en1k(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_en1k(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn runstdby(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_runstdby(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn ondemand(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_ondemand(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn startup(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x7 // [10:8]
  }
  pub fn set_startup(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn wrtlock(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_wrtlock(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn calib(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x7f // [22:16]
  }
  pub fn set_calib(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 16);
     self.0 |= value << 16;
     self
  }

}

impl ::core::fmt::Display for Osc32k {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Osc32k {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.en32k() != 0 { try!(write!(f, " en32k"))}
      if self.en1k() != 0 { try!(write!(f, " en1k"))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      if self.ondemand() != 0 { try!(write!(f, " ondemand"))}
      if self.startup() != 0 { try!(write!(f, " startup=0x{:x}", self.startup()))}
      if self.wrtlock() != 0 { try!(write!(f, " wrtlock"))}
      if self.calib() != 0 { try!(write!(f, " calib=0x{:x}", self.calib()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Pclksr(pub u32);

impl Pclksr {
  pub fn xoscrdy(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_xoscrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn xosc32krdy(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_xosc32krdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn osc32krdy(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_osc32krdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn osc8mrdy(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_osc8mrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn dfllrdy(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_dfllrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn dflloob(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_dflloob(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn dflllckf(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_dflllckf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn dflllckc(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_dflllckc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn dfllrcs(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_dfllrcs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn bod33rdy(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_bod33rdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn bod33det(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_bod33det(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn b33srdy(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_b33srdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn dplllckr(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_dplllckr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn dplllckf(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_dplllckf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn dplllto(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_dplllto(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

}

impl ::core::fmt::Display for Pclksr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Pclksr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.xoscrdy() != 0 { try!(write!(f, " xoscrdy"))}
      if self.xosc32krdy() != 0 { try!(write!(f, " xosc32krdy"))}
      if self.osc32krdy() != 0 { try!(write!(f, " osc32krdy"))}
      if self.osc8mrdy() != 0 { try!(write!(f, " osc8mrdy"))}
      if self.dfllrdy() != 0 { try!(write!(f, " dfllrdy"))}
      if self.dflloob() != 0 { try!(write!(f, " dflloob"))}
      if self.dflllckf() != 0 { try!(write!(f, " dflllckf"))}
      if self.dflllckc() != 0 { try!(write!(f, " dflllckc"))}
      if self.dfllrcs() != 0 { try!(write!(f, " dfllrcs"))}
      if self.bod33rdy() != 0 { try!(write!(f, " bod33rdy"))}
      if self.bod33det() != 0 { try!(write!(f, " bod33det"))}
      if self.b33srdy() != 0 { try!(write!(f, " b33srdy"))}
      if self.dplllckr() != 0 { try!(write!(f, " dplllckr"))}
      if self.dplllckf() != 0 { try!(write!(f, " dplllckf"))}
      if self.dplllto() != 0 { try!(write!(f, " dplllto"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Vref(pub u32);

impl Vref {
  pub fn tsen(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_tsen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn bgouten(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_bgouten(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn calib(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x7ff // [26:16]
  }
  pub fn set_calib(mut self, value: u32) -> Self {
     assert!((value & !0x7ff) == 0);
     self.0 &= !(0x7ff << 16);
     self.0 |= value << 16;
     self
  }

}

impl ::core::fmt::Display for Vref {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Vref {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tsen() != 0 { try!(write!(f, " tsen"))}
      if self.bgouten() != 0 { try!(write!(f, " bgouten"))}
      if self.calib() != 0 { try!(write!(f, " calib=0x{:x}", self.calib()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Xosc(pub u16);

impl Xosc {
  pub fn enable(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
  pub fn set_enable(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn xtalen(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
  pub fn set_xtalen(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn runstdby(&self) -> u16 {
     ((self.0 as u16) >> 6) & 0x1 // [6]
  }
  pub fn set_runstdby(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn ondemand(&self) -> u16 {
     ((self.0 as u16) >> 7) & 0x1 // [7]
  }
  pub fn set_ondemand(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn gain(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x7 // [10:8]
  }
  pub fn set_gain(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn ampgc(&self) -> u16 {
     ((self.0 as u16) >> 11) & 0x1 // [11]
  }
  pub fn set_ampgc(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn startup(&self) -> u16 {
     ((self.0 as u16) >> 12) & 0xf // [15:12]
  }
  pub fn set_startup(mut self, value: u16) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

}

impl ::core::fmt::Display for Xosc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Xosc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.xtalen() != 0 { try!(write!(f, " xtalen"))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      if self.ondemand() != 0 { try!(write!(f, " ondemand"))}
      if self.gain() != 0 { try!(write!(f, " gain=0x{:x}", self.gain()))}
      if self.ampgc() != 0 { try!(write!(f, " ampgc"))}
      if self.startup() != 0 { try!(write!(f, " startup=0x{:x}", self.startup()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Xosc32k(pub u16);

impl Xosc32k {
  pub fn enable(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
  pub fn set_enable(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn xtalen(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
  pub fn set_xtalen(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn en32k(&self) -> u16 {
     ((self.0 as u16) >> 3) & 0x1 // [3]
  }
  pub fn set_en32k(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn en1k(&self) -> u16 {
     ((self.0 as u16) >> 4) & 0x1 // [4]
  }
  pub fn set_en1k(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn aampen(&self) -> u16 {
     ((self.0 as u16) >> 5) & 0x1 // [5]
  }
  pub fn set_aampen(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn runstdby(&self) -> u16 {
     ((self.0 as u16) >> 6) & 0x1 // [6]
  }
  pub fn set_runstdby(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn ondemand(&self) -> u16 {
     ((self.0 as u16) >> 7) & 0x1 // [7]
  }
  pub fn set_ondemand(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn startup(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x7 // [10:8]
  }
  pub fn set_startup(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn wrtlock(&self) -> u16 {
     ((self.0 as u16) >> 12) & 0x1 // [12]
  }
  pub fn set_wrtlock(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

}

impl ::core::fmt::Display for Xosc32k {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Xosc32k {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.xtalen() != 0 { try!(write!(f, " xtalen"))}
      if self.en32k() != 0 { try!(write!(f, " en32k"))}
      if self.en1k() != 0 { try!(write!(f, " en1k"))}
      if self.aampen() != 0 { try!(write!(f, " aampen"))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      if self.ondemand() != 0 { try!(write!(f, " ondemand"))}
      if self.startup() != 0 { try!(write!(f, " startup=0x{:x}", self.startup()))}
      if self.wrtlock() != 0 { try!(write!(f, " wrtlock"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

