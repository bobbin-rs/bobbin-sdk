pub const TCC0: Tcc0 = Periph(0x42002000, Tcc0Id {});
pub const TCC1: Tcc1 = Periph(0x42002400, Tcc1Id {});
pub const TCC2: Tcc2 = Periph(0x42002800, Tcc2Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Periph<T>(pub u32, pub T); 

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tcc0Id {}
pub type Tcc0 = Periph<Tcc0Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tcc1Id {}
pub type Tcc1 = Periph<Tcc1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tcc2Id {}
pub type Tcc2 = Periph<Tcc2Id>;





impl<T> Periph<T> {
  #[inline]
  pub fn cc_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x44 + (index << 2)) as *const u32
  }
  #[inline]
  pub fn cc_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x44 + (index << 2)) as *mut u32
  }
  #[inline]
  pub fn cc(&self, index: usize) -> Cc { 
     assert!(index < 4);
     unsafe {
        Cc(::core::ptr::read_volatile(((self.0 as usize) + 0x44 + (index << 2)) as *const u32))
     }
  }
  #[inline]
  pub fn set_cc(&self, index: usize, value: Cc) -> &Self {
     assert!(index < 4);
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x44 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_cc<F: FnOnce(Cc) -> Cc>(&self, index: usize, f: F) -> &Self {
     let tmp = self.cc(index);
     self.set_cc(index, f(tmp))
  }

  #[inline]
  pub fn ccb_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x70 + (index << 2)) as *const u32
  }
  #[inline]
  pub fn ccb_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x70 + (index << 2)) as *mut u32
  }
  #[inline]
  pub fn ccb(&self, index: usize) -> Ccb { 
     assert!(index < 4);
     unsafe {
        Ccb(::core::ptr::read_volatile(((self.0 as usize) + 0x70 + (index << 2)) as *const u32))
     }
  }
  #[inline]
  pub fn set_ccb(&self, index: usize, value: Ccb) -> &Self {
     assert!(index < 4);
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x70 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ccb<F: FnOnce(Ccb) -> Ccb>(&self, index: usize, f: F) -> &Self {
     let tmp = self.ccb(index);
     self.set_ccb(index, f(tmp))
  }

  #[inline]
  pub fn count_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x34) as *const u32
  }
  #[inline]
  pub fn count_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x34) as *mut u32
  }
  #[inline]
  pub fn count(&self) -> Count { 
     unsafe {
       Count(::core::ptr::read_volatile(((self.0 as usize) + 0x34) as *const u32))
     }
  }
  #[inline]
  pub fn set_count(&self, value: Count) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x34) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
     let tmp = self.count();
     self.set_count(f(tmp))
  }

  #[inline]
  pub fn ctrla_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
  #[inline]
  pub fn ctrla_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
  #[inline]
  pub fn ctrla(&self) -> Ctrla { 
     unsafe {
       Ctrla(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  #[inline]
  pub fn set_ctrla(&self, value: Ctrla) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
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
  pub fn set_ctrlbclr(&self, value: Ctrlbclr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Self {
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
  pub fn set_ctrlbset(&self, value: Ctrlbset) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x5) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Self {
     let tmp = self.ctrlbset();
     self.set_ctrlbset(f(tmp))
  }

  #[inline]
  pub fn dbgctrl_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x1e) as *const u8
  }
  #[inline]
  pub fn dbgctrl_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x1e) as *mut u8
  }
  #[inline]
  pub fn dbgctrl(&self) -> Dbgctrl { 
     unsafe {
       Dbgctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x1e) as *const u8))
     }
  }
  #[inline]
  pub fn set_dbgctrl(&self, value: Dbgctrl) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x1e) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
     let tmp = self.dbgctrl();
     self.set_dbgctrl(f(tmp))
  }

  #[inline]
  pub fn drvctrl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
  #[inline]
  pub fn drvctrl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
  #[inline]
  pub fn drvctrl(&self) -> Drvctrl { 
     unsafe {
       Drvctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
  #[inline]
  pub fn set_drvctrl(&self, value: Drvctrl) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_drvctrl<F: FnOnce(Drvctrl) -> Drvctrl>(&self, f: F) -> &Self {
     let tmp = self.drvctrl();
     self.set_drvctrl(f(tmp))
  }

  #[inline]
  pub fn evctrl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
  #[inline]
  pub fn evctrl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
  #[inline]
  pub fn evctrl(&self) -> Evctrl { 
     unsafe {
       Evctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
  #[inline]
  pub fn set_evctrl(&self, value: Evctrl) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
     let tmp = self.evctrl();
     self.set_evctrl(f(tmp))
  }

  #[inline]
  pub fn fctrla_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
  #[inline]
  pub fn fctrla_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
  #[inline]
  pub fn fctrla(&self) -> Fctrla { 
     unsafe {
       Fctrla(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
  #[inline]
  pub fn set_fctrla(&self, value: Fctrla) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_fctrla<F: FnOnce(Fctrla) -> Fctrla>(&self, f: F) -> &Self {
     let tmp = self.fctrla();
     self.set_fctrla(f(tmp))
  }

  #[inline]
  pub fn fctrlb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
  #[inline]
  pub fn fctrlb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
  #[inline]
  pub fn fctrlb(&self) -> Fctrlb { 
     unsafe {
       Fctrlb(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
  #[inline]
  pub fn set_fctrlb(&self, value: Fctrlb) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_fctrlb<F: FnOnce(Fctrlb) -> Fctrlb>(&self, f: F) -> &Self {
     let tmp = self.fctrlb();
     self.set_fctrlb(f(tmp))
  }

  #[inline]
  pub fn intenclr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
  #[inline]
  pub fn intenclr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
  #[inline]
  pub fn intenclr(&self) -> Intenclr { 
     unsafe {
       Intenclr(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }
  #[inline]
  pub fn set_intenclr(&self, value: Intenclr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
     let tmp = self.intenclr();
     self.set_intenclr(f(tmp))
  }

  #[inline]
  pub fn intenset_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x28) as *const u32
  }
  #[inline]
  pub fn intenset_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x28) as *mut u32
  }
  #[inline]
  pub fn intenset(&self) -> Intenset { 
     unsafe {
       Intenset(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
     }
  }
  #[inline]
  pub fn set_intenset(&self, value: Intenset) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
     let tmp = self.intenset();
     self.set_intenset(f(tmp))
  }

  #[inline]
  pub fn intflag_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2c) as *const u32
  }
  #[inline]
  pub fn intflag_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2c) as *mut u32
  }
  #[inline]
  pub fn intflag(&self) -> Intflag { 
     unsafe {
       Intflag(::core::ptr::read_volatile(((self.0 as usize) + 0x2c) as *const u32))
     }
  }
  #[inline]
  pub fn set_intflag(&self, value: Intflag) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x2c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
     let tmp = self.intflag();
     self.set_intflag(f(tmp))
  }

  #[inline]
  pub fn patt_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x38) as *const u16
  }
  #[inline]
  pub fn patt_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x38) as *mut u16
  }
  #[inline]
  pub fn patt(&self) -> Patt { 
     unsafe {
       Patt(::core::ptr::read_volatile(((self.0 as usize) + 0x38) as *const u16))
     }
  }
  #[inline]
  pub fn set_patt(&self, value: Patt) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x38) as *mut u16, value.0);
     }
     self
  }
  #[inline]
  pub fn with_patt<F: FnOnce(Patt) -> Patt>(&self, f: F) -> &Self {
     let tmp = self.patt();
     self.set_patt(f(tmp))
  }

  #[inline]
  pub fn pattb_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x64) as *const u16
  }
  #[inline]
  pub fn pattb_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x64) as *mut u16
  }
  #[inline]
  pub fn pattb(&self) -> Pattb { 
     unsafe {
       Pattb(::core::ptr::read_volatile(((self.0 as usize) + 0x64) as *const u16))
     }
  }
  #[inline]
  pub fn set_pattb(&self, value: Pattb) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x64) as *mut u16, value.0);
     }
     self
  }
  #[inline]
  pub fn with_pattb<F: FnOnce(Pattb) -> Pattb>(&self, f: F) -> &Self {
     let tmp = self.pattb();
     self.set_pattb(f(tmp))
  }

  #[inline]
  pub fn per_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x40) as *const u32
  }
  #[inline]
  pub fn per_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x40) as *mut u32
  }
  #[inline]
  pub fn per(&self) -> Per { 
     unsafe {
       Per(::core::ptr::read_volatile(((self.0 as usize) + 0x40) as *const u32))
     }
  }
  #[inline]
  pub fn set_per(&self, value: Per) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_per<F: FnOnce(Per) -> Per>(&self, f: F) -> &Self {
     let tmp = self.per();
     self.set_per(f(tmp))
  }

  #[inline]
  pub fn perb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x6c) as *const u32
  }
  #[inline]
  pub fn perb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x6c) as *mut u32
  }
  #[inline]
  pub fn perb(&self) -> Perb { 
     unsafe {
       Perb(::core::ptr::read_volatile(((self.0 as usize) + 0x6c) as *const u32))
     }
  }
  #[inline]
  pub fn set_perb(&self, value: Perb) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x6c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_perb<F: FnOnce(Perb) -> Perb>(&self, f: F) -> &Self {
     let tmp = self.perb();
     self.set_perb(f(tmp))
  }

  #[inline]
  pub fn status_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x30) as *const u32
  }
  #[inline]
  pub fn status_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x30) as *mut u32
  }
  #[inline]
  pub fn status(&self) -> Status { 
     unsafe {
       Status(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u32))
     }
  }
  #[inline]
  pub fn set_status(&self, value: Status) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
     let tmp = self.status();
     self.set_status(f(tmp))
  }

  #[inline]
  pub fn syncbusy_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
  #[inline]
  pub fn syncbusy_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
  #[inline]
  pub fn syncbusy(&self) -> Syncbusy { 
     unsafe {
       Syncbusy(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }

  #[inline]
  pub fn wave_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x3c) as *const u32
  }
  #[inline]
  pub fn wave_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x3c) as *mut u32
  }
  #[inline]
  pub fn wave(&self) -> Wave { 
     unsafe {
       Wave(::core::ptr::read_volatile(((self.0 as usize) + 0x3c) as *const u32))
     }
  }
  #[inline]
  pub fn set_wave(&self, value: Wave) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x3c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_wave<F: FnOnce(Wave) -> Wave>(&self, f: F) -> &Self {
     let tmp = self.wave();
     self.set_wave(f(tmp))
  }

  #[inline]
  pub fn waveb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x68) as *const u32
  }
  #[inline]
  pub fn waveb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x68) as *mut u32
  }
  #[inline]
  pub fn waveb(&self) -> Waveb { 
     unsafe {
       Waveb(::core::ptr::read_volatile(((self.0 as usize) + 0x68) as *const u32))
     }
  }
  #[inline]
  pub fn set_waveb(&self, value: Waveb) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x68) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_waveb<F: FnOnce(Waveb) -> Waveb>(&self, f: F) -> &Self {
     let tmp = self.waveb();
     self.set_waveb(f(tmp))
  }

  #[inline]
  pub fn wexctrl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
  #[inline]
  pub fn wexctrl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
  #[inline]
  pub fn wexctrl(&self) -> Wexctrl { 
     unsafe {
       Wexctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
  #[inline]
  pub fn set_wexctrl(&self, value: Wexctrl) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_wexctrl<F: FnOnce(Wexctrl) -> Wexctrl>(&self, f: F) -> &Self {
     let tmp = self.wexctrl();
     self.set_wexctrl(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Cc(pub u32);
impl Cc {
  #[inline]
  pub fn cc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffff // [23:0]
  }
  #[inline]
  pub fn set_cc(mut self, value: u32) -> Self {
     assert!((value & !0xffffff) == 0);
     self.0 &= !(0xffffff << 0);
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
pub struct Ccb(pub u32);
impl Ccb {
  #[inline]
  pub fn ccb(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffff // [23:0]
  }
  #[inline]
  pub fn set_ccb(mut self, value: u32) -> Self {
     assert!((value & !0xffffff) == 0);
     self.0 &= !(0xffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ccb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ccb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ccb() != 0 { try!(write!(f, " ccb=0x{:x}", self.ccb()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Count(pub u32);
impl Count {
  #[inline]
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffff // [23:0]
  }
  #[inline]
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffffff) == 0);
     self.0 &= !(0xffffff << 0);
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
pub struct Ctrla(pub u32);
impl Ctrla {
  #[inline]
  pub fn swrst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_swrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn enable(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn resolution(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x3 // [6:5]
  }
  #[inline]
  pub fn set_resolution(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn prescaler(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x7 // [10:8]
  }
  #[inline]
  pub fn set_prescaler(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn runstdby(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_runstdby(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn prescsync(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x3 // [13:12]
  }
  #[inline]
  pub fn set_prescsync(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn alock(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline]
  pub fn set_alock(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline]
  pub fn msync(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  #[inline]
  pub fn set_msync(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  #[inline]
  pub fn cpten0(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  #[inline]
  pub fn set_cpten0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  #[inline]
  pub fn cpten1(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  #[inline]
  pub fn set_cpten1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  #[inline]
  pub fn cpten2(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  #[inline]
  pub fn set_cpten2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  #[inline]
  pub fn cpten3(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  #[inline]
  pub fn set_cpten3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
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
      if self.resolution() != 0 { try!(write!(f, " resolution=0x{:x}", self.resolution()))}
      if self.prescaler() != 0 { try!(write!(f, " prescaler=0x{:x}", self.prescaler()))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      if self.prescsync() != 0 { try!(write!(f, " prescsync=0x{:x}", self.prescsync()))}
      if self.alock() != 0 { try!(write!(f, " alock"))}
      if self.msync() != 0 { try!(write!(f, " msync"))}
      if self.cpten0() != 0 { try!(write!(f, " cpten0"))}
      if self.cpten1() != 0 { try!(write!(f, " cpten1"))}
      if self.cpten2() != 0 { try!(write!(f, " cpten2"))}
      if self.cpten3() != 0 { try!(write!(f, " cpten3"))}
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
  pub fn lupd(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_lupd(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
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
  pub fn idxcmd(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x3 // [4:3]
  }
  #[inline]
  pub fn set_idxcmd(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn cmd(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x7 // [7:5]
  }
  #[inline]
  pub fn set_cmd(mut self, value: u8) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 5);
     self.0 |= value << 5;
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
      if self.lupd() != 0 { try!(write!(f, " lupd"))}
      if self.oneshot() != 0 { try!(write!(f, " oneshot"))}
      if self.idxcmd() != 0 { try!(write!(f, " idxcmd=0x{:x}", self.idxcmd()))}
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
  pub fn lupd(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_lupd(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
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
  pub fn idxcmd(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x3 // [4:3]
  }
  #[inline]
  pub fn set_idxcmd(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn cmd(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x7 // [7:5]
  }
  #[inline]
  pub fn set_cmd(mut self, value: u8) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 5);
     self.0 |= value << 5;
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
      if self.lupd() != 0 { try!(write!(f, " lupd"))}
      if self.oneshot() != 0 { try!(write!(f, " oneshot"))}
      if self.idxcmd() != 0 { try!(write!(f, " idxcmd=0x{:x}", self.idxcmd()))}
      if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
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

  #[inline]
  pub fn fddbd(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_fddbd(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
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
      if self.fddbd() != 0 { try!(write!(f, " fddbd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Drvctrl(pub u32);
impl Drvctrl {
  #[inline]
  pub fn nre0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_nre0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn nre1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_nre1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn nre2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_nre2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn nre3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_nre3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn nre4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_nre4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn nre5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_nre5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn nre6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_nre6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn nre7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_nre7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn nrv0(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_nrv0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn nrv1(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_nrv1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn nrv2(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_nrv2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn nrv3(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_nrv3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn nrv4(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline]
  pub fn set_nrv4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn nrv5(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_nrv5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline]
  pub fn nrv6(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline]
  pub fn set_nrv6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline]
  pub fn nrv7(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  #[inline]
  pub fn set_nrv7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  #[inline]
  pub fn inven0(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
  pub fn set_inven0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn inven1(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline]
  pub fn set_inven1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline]
  pub fn inven2(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline]
  pub fn set_inven2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  #[inline]
  pub fn inven3(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  #[inline]
  pub fn set_inven3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  #[inline]
  pub fn inven4(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  #[inline]
  pub fn set_inven4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  #[inline]
  pub fn inven5(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  #[inline]
  pub fn set_inven5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  #[inline]
  pub fn inven6(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  #[inline]
  pub fn set_inven6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  #[inline]
  pub fn inven7(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  #[inline]
  pub fn set_inven7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  #[inline]
  pub fn filterval0(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  #[inline]
  pub fn set_filterval0(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

  #[inline]
  pub fn filterval1(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0xf // [31:28]
  }
  #[inline]
  pub fn set_filterval1(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 28);
     self.0 |= value << 28;
     self
  }

}
impl ::core::fmt::Display for Drvctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Drvctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.nre0() != 0 { try!(write!(f, " nre0"))}
      if self.nre1() != 0 { try!(write!(f, " nre1"))}
      if self.nre2() != 0 { try!(write!(f, " nre2"))}
      if self.nre3() != 0 { try!(write!(f, " nre3"))}
      if self.nre4() != 0 { try!(write!(f, " nre4"))}
      if self.nre5() != 0 { try!(write!(f, " nre5"))}
      if self.nre6() != 0 { try!(write!(f, " nre6"))}
      if self.nre7() != 0 { try!(write!(f, " nre7"))}
      if self.nrv0() != 0 { try!(write!(f, " nrv0"))}
      if self.nrv1() != 0 { try!(write!(f, " nrv1"))}
      if self.nrv2() != 0 { try!(write!(f, " nrv2"))}
      if self.nrv3() != 0 { try!(write!(f, " nrv3"))}
      if self.nrv4() != 0 { try!(write!(f, " nrv4"))}
      if self.nrv5() != 0 { try!(write!(f, " nrv5"))}
      if self.nrv6() != 0 { try!(write!(f, " nrv6"))}
      if self.nrv7() != 0 { try!(write!(f, " nrv7"))}
      if self.inven0() != 0 { try!(write!(f, " inven0"))}
      if self.inven1() != 0 { try!(write!(f, " inven1"))}
      if self.inven2() != 0 { try!(write!(f, " inven2"))}
      if self.inven3() != 0 { try!(write!(f, " inven3"))}
      if self.inven4() != 0 { try!(write!(f, " inven4"))}
      if self.inven5() != 0 { try!(write!(f, " inven5"))}
      if self.inven6() != 0 { try!(write!(f, " inven6"))}
      if self.inven7() != 0 { try!(write!(f, " inven7"))}
      if self.filterval0() != 0 { try!(write!(f, " filterval0=0x{:x}", self.filterval0()))}
      if self.filterval1() != 0 { try!(write!(f, " filterval1=0x{:x}", self.filterval1()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Evctrl(pub u32);
impl Evctrl {
  #[inline]
  pub fn evact0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
  #[inline]
  pub fn set_evact0(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn evact1(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x7 // [5:3]
  }
  #[inline]
  pub fn set_evact1(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn cntsel(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
  #[inline]
  pub fn set_cntsel(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn ovfeo(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_ovfeo(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn trgeo(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_trgeo(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn cnteo(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_cnteo(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn tcinv0(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline]
  pub fn set_tcinv0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn tcinv1(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_tcinv1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline]
  pub fn tcei0(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline]
  pub fn set_tcei0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline]
  pub fn tcei1(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  #[inline]
  pub fn set_tcei1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  #[inline]
  pub fn mcei0(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
  pub fn set_mcei0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn mcei1(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline]
  pub fn set_mcei1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline]
  pub fn mcei2(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline]
  pub fn set_mcei2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  #[inline]
  pub fn mcei3(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  #[inline]
  pub fn set_mcei3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  #[inline]
  pub fn mceo0(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  #[inline]
  pub fn set_mceo0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  #[inline]
  pub fn mceo1(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  #[inline]
  pub fn set_mceo1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  #[inline]
  pub fn mceo2(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  #[inline]
  pub fn set_mceo2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  #[inline]
  pub fn mceo3(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  #[inline]
  pub fn set_mceo3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
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
      if self.evact0() != 0 { try!(write!(f, " evact0=0x{:x}", self.evact0()))}
      if self.evact1() != 0 { try!(write!(f, " evact1=0x{:x}", self.evact1()))}
      if self.cntsel() != 0 { try!(write!(f, " cntsel=0x{:x}", self.cntsel()))}
      if self.ovfeo() != 0 { try!(write!(f, " ovfeo"))}
      if self.trgeo() != 0 { try!(write!(f, " trgeo"))}
      if self.cnteo() != 0 { try!(write!(f, " cnteo"))}
      if self.tcinv0() != 0 { try!(write!(f, " tcinv0"))}
      if self.tcinv1() != 0 { try!(write!(f, " tcinv1"))}
      if self.tcei0() != 0 { try!(write!(f, " tcei0"))}
      if self.tcei1() != 0 { try!(write!(f, " tcei1"))}
      if self.mcei0() != 0 { try!(write!(f, " mcei0"))}
      if self.mcei1() != 0 { try!(write!(f, " mcei1"))}
      if self.mcei2() != 0 { try!(write!(f, " mcei2"))}
      if self.mcei3() != 0 { try!(write!(f, " mcei3"))}
      if self.mceo0() != 0 { try!(write!(f, " mceo0"))}
      if self.mceo1() != 0 { try!(write!(f, " mceo1"))}
      if self.mceo2() != 0 { try!(write!(f, " mceo2"))}
      if self.mceo3() != 0 { try!(write!(f, " mceo3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Fctrla(pub u32);
impl Fctrla {
  #[inline]
  pub fn src(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline]
  pub fn set_src(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn keep(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_keep(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn qual(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_qual(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn blank(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x3 // [6:5]
  }
  #[inline]
  pub fn set_blank(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn restart(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_restart(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn halt(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  #[inline]
  pub fn set_halt(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn chsel(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
  #[inline]
  pub fn set_chsel(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn capture(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x7 // [14:12]
  }
  #[inline]
  pub fn set_capture(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn blankval(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  #[inline]
  pub fn set_blankval(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn filterval(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  #[inline]
  pub fn set_filterval(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

}
impl ::core::fmt::Display for Fctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.src() != 0 { try!(write!(f, " src=0x{:x}", self.src()))}
      if self.keep() != 0 { try!(write!(f, " keep"))}
      if self.qual() != 0 { try!(write!(f, " qual"))}
      if self.blank() != 0 { try!(write!(f, " blank=0x{:x}", self.blank()))}
      if self.restart() != 0 { try!(write!(f, " restart"))}
      if self.halt() != 0 { try!(write!(f, " halt=0x{:x}", self.halt()))}
      if self.chsel() != 0 { try!(write!(f, " chsel=0x{:x}", self.chsel()))}
      if self.capture() != 0 { try!(write!(f, " capture=0x{:x}", self.capture()))}
      if self.blankval() != 0 { try!(write!(f, " blankval=0x{:x}", self.blankval()))}
      if self.filterval() != 0 { try!(write!(f, " filterval=0x{:x}", self.filterval()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Fctrlb(pub u32);
impl Fctrlb {
  #[inline]
  pub fn src(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline]
  pub fn set_src(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn keep(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_keep(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn qual(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_qual(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn blank(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x3 // [6:5]
  }
  #[inline]
  pub fn set_blank(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn restart(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_restart(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn halt(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  #[inline]
  pub fn set_halt(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn chsel(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
  #[inline]
  pub fn set_chsel(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn capture(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x7 // [14:12]
  }
  #[inline]
  pub fn set_capture(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn blankval(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  #[inline]
  pub fn set_blankval(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn filterval(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  #[inline]
  pub fn set_filterval(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

}
impl ::core::fmt::Display for Fctrlb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fctrlb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.src() != 0 { try!(write!(f, " src=0x{:x}", self.src()))}
      if self.keep() != 0 { try!(write!(f, " keep"))}
      if self.qual() != 0 { try!(write!(f, " qual"))}
      if self.blank() != 0 { try!(write!(f, " blank=0x{:x}", self.blank()))}
      if self.restart() != 0 { try!(write!(f, " restart"))}
      if self.halt() != 0 { try!(write!(f, " halt=0x{:x}", self.halt()))}
      if self.chsel() != 0 { try!(write!(f, " chsel=0x{:x}", self.chsel()))}
      if self.capture() != 0 { try!(write!(f, " capture=0x{:x}", self.capture()))}
      if self.blankval() != 0 { try!(write!(f, " blankval=0x{:x}", self.blankval()))}
      if self.filterval() != 0 { try!(write!(f, " filterval=0x{:x}", self.filterval()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Intenclr(pub u32);
impl Intenclr {
  #[inline]
  pub fn ovf(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_ovf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn trg(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_trg(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn cnt(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_cnt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn err(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_err(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn dfs(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_dfs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn faulta(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline]
  pub fn set_faulta(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn faultb(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_faultb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline]
  pub fn fault0(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline]
  pub fn set_fault0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline]
  pub fn fault1(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  #[inline]
  pub fn set_fault1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  #[inline]
  pub fn mc0(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
  pub fn set_mc0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn mc1(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline]
  pub fn set_mc1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline]
  pub fn mc2(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline]
  pub fn set_mc2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  #[inline]
  pub fn mc3(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  #[inline]
  pub fn set_mc3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
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
      if self.trg() != 0 { try!(write!(f, " trg"))}
      if self.cnt() != 0 { try!(write!(f, " cnt"))}
      if self.err() != 0 { try!(write!(f, " err"))}
      if self.dfs() != 0 { try!(write!(f, " dfs"))}
      if self.faulta() != 0 { try!(write!(f, " faulta"))}
      if self.faultb() != 0 { try!(write!(f, " faultb"))}
      if self.fault0() != 0 { try!(write!(f, " fault0"))}
      if self.fault1() != 0 { try!(write!(f, " fault1"))}
      if self.mc0() != 0 { try!(write!(f, " mc0"))}
      if self.mc1() != 0 { try!(write!(f, " mc1"))}
      if self.mc2() != 0 { try!(write!(f, " mc2"))}
      if self.mc3() != 0 { try!(write!(f, " mc3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Intenset(pub u32);
impl Intenset {
  #[inline]
  pub fn ovf(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_ovf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn trg(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_trg(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn cnt(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_cnt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn err(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_err(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn dfs(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_dfs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn faulta(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline]
  pub fn set_faulta(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn faultb(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_faultb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline]
  pub fn fault0(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline]
  pub fn set_fault0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline]
  pub fn fault1(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  #[inline]
  pub fn set_fault1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  #[inline]
  pub fn mc0(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
  pub fn set_mc0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn mc1(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline]
  pub fn set_mc1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline]
  pub fn mc2(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline]
  pub fn set_mc2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  #[inline]
  pub fn mc3(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  #[inline]
  pub fn set_mc3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
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
      if self.trg() != 0 { try!(write!(f, " trg"))}
      if self.cnt() != 0 { try!(write!(f, " cnt"))}
      if self.err() != 0 { try!(write!(f, " err"))}
      if self.dfs() != 0 { try!(write!(f, " dfs"))}
      if self.faulta() != 0 { try!(write!(f, " faulta"))}
      if self.faultb() != 0 { try!(write!(f, " faultb"))}
      if self.fault0() != 0 { try!(write!(f, " fault0"))}
      if self.fault1() != 0 { try!(write!(f, " fault1"))}
      if self.mc0() != 0 { try!(write!(f, " mc0"))}
      if self.mc1() != 0 { try!(write!(f, " mc1"))}
      if self.mc2() != 0 { try!(write!(f, " mc2"))}
      if self.mc3() != 0 { try!(write!(f, " mc3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Intflag(pub u32);
impl Intflag {
  #[inline]
  pub fn ovf(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_ovf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn trg(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_trg(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn cnt(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_cnt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn err(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_err(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn dfs(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_dfs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn faulta(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline]
  pub fn set_faulta(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn faultb(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_faultb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline]
  pub fn fault0(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline]
  pub fn set_fault0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline]
  pub fn fault1(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  #[inline]
  pub fn set_fault1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  #[inline]
  pub fn mc0(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
  pub fn set_mc0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn mc1(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline]
  pub fn set_mc1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline]
  pub fn mc2(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline]
  pub fn set_mc2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  #[inline]
  pub fn mc3(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  #[inline]
  pub fn set_mc3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
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
      if self.trg() != 0 { try!(write!(f, " trg"))}
      if self.cnt() != 0 { try!(write!(f, " cnt"))}
      if self.err() != 0 { try!(write!(f, " err"))}
      if self.dfs() != 0 { try!(write!(f, " dfs"))}
      if self.faulta() != 0 { try!(write!(f, " faulta"))}
      if self.faultb() != 0 { try!(write!(f, " faultb"))}
      if self.fault0() != 0 { try!(write!(f, " fault0"))}
      if self.fault1() != 0 { try!(write!(f, " fault1"))}
      if self.mc0() != 0 { try!(write!(f, " mc0"))}
      if self.mc1() != 0 { try!(write!(f, " mc1"))}
      if self.mc2() != 0 { try!(write!(f, " mc2"))}
      if self.mc3() != 0 { try!(write!(f, " mc3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Patt(pub u16);
impl Patt {
  #[inline]
  pub fn pge0(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_pge0(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn pge1(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_pge1(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn pge2(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_pge2(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn pge3(&self) -> u16 {
     ((self.0 as u16) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_pge3(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn pge4(&self) -> u16 {
     ((self.0 as u16) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_pge4(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn pge5(&self) -> u16 {
     ((self.0 as u16) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_pge5(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn pge6(&self) -> u16 {
     ((self.0 as u16) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_pge6(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn pge7(&self) -> u16 {
     ((self.0 as u16) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_pge7(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn pgv0(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_pgv0(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn pgv1(&self) -> u16 {
     ((self.0 as u16) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_pgv1(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn pgv2(&self) -> u16 {
     ((self.0 as u16) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_pgv2(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn pgv3(&self) -> u16 {
     ((self.0 as u16) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_pgv3(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn pgv4(&self) -> u16 {
     ((self.0 as u16) >> 12) & 0x1 // [12]
  }
  #[inline]
  pub fn set_pgv4(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn pgv5(&self) -> u16 {
     ((self.0 as u16) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_pgv5(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline]
  pub fn pgv6(&self) -> u16 {
     ((self.0 as u16) >> 14) & 0x1 // [14]
  }
  #[inline]
  pub fn set_pgv6(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline]
  pub fn pgv7(&self) -> u16 {
     ((self.0 as u16) >> 15) & 0x1 // [15]
  }
  #[inline]
  pub fn set_pgv7(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

}
impl ::core::fmt::Display for Patt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Patt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pge0() != 0 { try!(write!(f, " pge0"))}
      if self.pge1() != 0 { try!(write!(f, " pge1"))}
      if self.pge2() != 0 { try!(write!(f, " pge2"))}
      if self.pge3() != 0 { try!(write!(f, " pge3"))}
      if self.pge4() != 0 { try!(write!(f, " pge4"))}
      if self.pge5() != 0 { try!(write!(f, " pge5"))}
      if self.pge6() != 0 { try!(write!(f, " pge6"))}
      if self.pge7() != 0 { try!(write!(f, " pge7"))}
      if self.pgv0() != 0 { try!(write!(f, " pgv0"))}
      if self.pgv1() != 0 { try!(write!(f, " pgv1"))}
      if self.pgv2() != 0 { try!(write!(f, " pgv2"))}
      if self.pgv3() != 0 { try!(write!(f, " pgv3"))}
      if self.pgv4() != 0 { try!(write!(f, " pgv4"))}
      if self.pgv5() != 0 { try!(write!(f, " pgv5"))}
      if self.pgv6() != 0 { try!(write!(f, " pgv6"))}
      if self.pgv7() != 0 { try!(write!(f, " pgv7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Pattb(pub u16);
impl Pattb {
  #[inline]
  pub fn pgeb0(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_pgeb0(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn pgeb1(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_pgeb1(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn pgeb2(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_pgeb2(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn pgeb3(&self) -> u16 {
     ((self.0 as u16) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_pgeb3(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn pgeb4(&self) -> u16 {
     ((self.0 as u16) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_pgeb4(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn pgeb5(&self) -> u16 {
     ((self.0 as u16) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_pgeb5(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn pgeb6(&self) -> u16 {
     ((self.0 as u16) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_pgeb6(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn pgeb7(&self) -> u16 {
     ((self.0 as u16) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_pgeb7(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn pgvb0(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_pgvb0(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn pgvb1(&self) -> u16 {
     ((self.0 as u16) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_pgvb1(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn pgvb2(&self) -> u16 {
     ((self.0 as u16) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_pgvb2(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn pgvb3(&self) -> u16 {
     ((self.0 as u16) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_pgvb3(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn pgvb4(&self) -> u16 {
     ((self.0 as u16) >> 12) & 0x1 // [12]
  }
  #[inline]
  pub fn set_pgvb4(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn pgvb5(&self) -> u16 {
     ((self.0 as u16) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_pgvb5(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline]
  pub fn pgvb6(&self) -> u16 {
     ((self.0 as u16) >> 14) & 0x1 // [14]
  }
  #[inline]
  pub fn set_pgvb6(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline]
  pub fn pgvb7(&self) -> u16 {
     ((self.0 as u16) >> 15) & 0x1 // [15]
  }
  #[inline]
  pub fn set_pgvb7(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

}
impl ::core::fmt::Display for Pattb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pattb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pgeb0() != 0 { try!(write!(f, " pgeb0"))}
      if self.pgeb1() != 0 { try!(write!(f, " pgeb1"))}
      if self.pgeb2() != 0 { try!(write!(f, " pgeb2"))}
      if self.pgeb3() != 0 { try!(write!(f, " pgeb3"))}
      if self.pgeb4() != 0 { try!(write!(f, " pgeb4"))}
      if self.pgeb5() != 0 { try!(write!(f, " pgeb5"))}
      if self.pgeb6() != 0 { try!(write!(f, " pgeb6"))}
      if self.pgeb7() != 0 { try!(write!(f, " pgeb7"))}
      if self.pgvb0() != 0 { try!(write!(f, " pgvb0"))}
      if self.pgvb1() != 0 { try!(write!(f, " pgvb1"))}
      if self.pgvb2() != 0 { try!(write!(f, " pgvb2"))}
      if self.pgvb3() != 0 { try!(write!(f, " pgvb3"))}
      if self.pgvb4() != 0 { try!(write!(f, " pgvb4"))}
      if self.pgvb5() != 0 { try!(write!(f, " pgvb5"))}
      if self.pgvb6() != 0 { try!(write!(f, " pgvb6"))}
      if self.pgvb7() != 0 { try!(write!(f, " pgvb7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Per(pub u32);
impl Per {
  #[inline]
  pub fn per(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffff // [23:0]
  }
  #[inline]
  pub fn set_per(mut self, value: u32) -> Self {
     assert!((value & !0xffffff) == 0);
     self.0 &= !(0xffffff << 0);
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
pub struct Perb(pub u32);
impl Perb {
  #[inline]
  pub fn perb(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffff // [23:0]
  }
  #[inline]
  pub fn set_perb(mut self, value: u32) -> Self {
     assert!((value & !0xffffff) == 0);
     self.0 &= !(0xffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Perb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Perb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.perb() != 0 { try!(write!(f, " perb=0x{:x}", self.perb()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Status(pub u32);
impl Status {
  #[inline]
  pub fn stop(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_stop(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn idx(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_idx(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn dfs(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_dfs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn pattbv(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_pattbv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn wavebv(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_wavebv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn perbv(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_perbv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn faultain(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_faultain(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn faultbin(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_faultbin(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn fault0in(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_fault0in(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn fault1in(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_fault1in(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn faulta(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline]
  pub fn set_faulta(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn faultb(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_faultb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline]
  pub fn fault0(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline]
  pub fn set_fault0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline]
  pub fn fault1(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  #[inline]
  pub fn set_fault1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  #[inline]
  pub fn ccbv0(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
  pub fn set_ccbv0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn ccbv1(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline]
  pub fn set_ccbv1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline]
  pub fn ccbv2(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline]
  pub fn set_ccbv2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  #[inline]
  pub fn ccbv3(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  #[inline]
  pub fn set_ccbv3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  #[inline]
  pub fn cmp0(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  #[inline]
  pub fn set_cmp0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  #[inline]
  pub fn cmp1(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  #[inline]
  pub fn set_cmp1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  #[inline]
  pub fn cmp2(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  #[inline]
  pub fn set_cmp2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  #[inline]
  pub fn cmp3(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  #[inline]
  pub fn set_cmp3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
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
      if self.idx() != 0 { try!(write!(f, " idx"))}
      if self.dfs() != 0 { try!(write!(f, " dfs"))}
      if self.pattbv() != 0 { try!(write!(f, " pattbv"))}
      if self.wavebv() != 0 { try!(write!(f, " wavebv"))}
      if self.perbv() != 0 { try!(write!(f, " perbv"))}
      if self.faultain() != 0 { try!(write!(f, " faultain"))}
      if self.faultbin() != 0 { try!(write!(f, " faultbin"))}
      if self.fault0in() != 0 { try!(write!(f, " fault0in"))}
      if self.fault1in() != 0 { try!(write!(f, " fault1in"))}
      if self.faulta() != 0 { try!(write!(f, " faulta"))}
      if self.faultb() != 0 { try!(write!(f, " faultb"))}
      if self.fault0() != 0 { try!(write!(f, " fault0"))}
      if self.fault1() != 0 { try!(write!(f, " fault1"))}
      if self.ccbv0() != 0 { try!(write!(f, " ccbv0"))}
      if self.ccbv1() != 0 { try!(write!(f, " ccbv1"))}
      if self.ccbv2() != 0 { try!(write!(f, " ccbv2"))}
      if self.ccbv3() != 0 { try!(write!(f, " ccbv3"))}
      if self.cmp0() != 0 { try!(write!(f, " cmp0"))}
      if self.cmp1() != 0 { try!(write!(f, " cmp1"))}
      if self.cmp2() != 0 { try!(write!(f, " cmp2"))}
      if self.cmp3() != 0 { try!(write!(f, " cmp3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
  #[inline]
  pub fn swrst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_swrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn enable(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn ctrlb(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_ctrlb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn status(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_status(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn patt(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_patt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn wave(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_wave(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn per(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_per(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn cc0(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_cc0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn cc1(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_cc1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn cc2(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_cc2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn cc3(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_cc3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn pattb(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
  pub fn set_pattb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn waveb(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline]
  pub fn set_waveb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline]
  pub fn perb(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline]
  pub fn set_perb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  #[inline]
  pub fn ccb0(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  #[inline]
  pub fn set_ccb0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  #[inline]
  pub fn ccb1(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  #[inline]
  pub fn set_ccb1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  #[inline]
  pub fn ccb2(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  #[inline]
  pub fn set_ccb2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  #[inline]
  pub fn ccb3(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  #[inline]
  pub fn set_ccb3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

}
impl ::core::fmt::Display for Syncbusy {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Syncbusy {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swrst() != 0 { try!(write!(f, " swrst"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.ctrlb() != 0 { try!(write!(f, " ctrlb"))}
      if self.status() != 0 { try!(write!(f, " status"))}
      if self.count() != 0 { try!(write!(f, " count"))}
      if self.patt() != 0 { try!(write!(f, " patt"))}
      if self.wave() != 0 { try!(write!(f, " wave"))}
      if self.per() != 0 { try!(write!(f, " per"))}
      if self.cc0() != 0 { try!(write!(f, " cc0"))}
      if self.cc1() != 0 { try!(write!(f, " cc1"))}
      if self.cc2() != 0 { try!(write!(f, " cc2"))}
      if self.cc3() != 0 { try!(write!(f, " cc3"))}
      if self.pattb() != 0 { try!(write!(f, " pattb"))}
      if self.waveb() != 0 { try!(write!(f, " waveb"))}
      if self.perb() != 0 { try!(write!(f, " perb"))}
      if self.ccb0() != 0 { try!(write!(f, " ccb0"))}
      if self.ccb1() != 0 { try!(write!(f, " ccb1"))}
      if self.ccb2() != 0 { try!(write!(f, " ccb2"))}
      if self.ccb3() != 0 { try!(write!(f, " ccb3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Wave(pub u32);
impl Wave {
  #[inline]
  pub fn wavegen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
  #[inline]
  pub fn set_wavegen(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn ramp(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  #[inline]
  pub fn set_ramp(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn ciperen(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_ciperen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn ciccen0(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_ciccen0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn ciccen1(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_ciccen1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn ciccen2(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_ciccen2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn ciccen3(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_ciccen3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn pol0(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
  pub fn set_pol0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn pol1(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline]
  pub fn set_pol1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline]
  pub fn pol2(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline]
  pub fn set_pol2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  #[inline]
  pub fn pol3(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  #[inline]
  pub fn set_pol3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  #[inline]
  pub fn swap0(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  #[inline]
  pub fn set_swap0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  #[inline]
  pub fn swap1(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  #[inline]
  pub fn set_swap1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  #[inline]
  pub fn swap2(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  #[inline]
  pub fn set_swap2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  #[inline]
  pub fn swap3(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  #[inline]
  pub fn set_swap3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

}
impl ::core::fmt::Display for Wave {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wave {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wavegen() != 0 { try!(write!(f, " wavegen=0x{:x}", self.wavegen()))}
      if self.ramp() != 0 { try!(write!(f, " ramp=0x{:x}", self.ramp()))}
      if self.ciperen() != 0 { try!(write!(f, " ciperen"))}
      if self.ciccen0() != 0 { try!(write!(f, " ciccen0"))}
      if self.ciccen1() != 0 { try!(write!(f, " ciccen1"))}
      if self.ciccen2() != 0 { try!(write!(f, " ciccen2"))}
      if self.ciccen3() != 0 { try!(write!(f, " ciccen3"))}
      if self.pol0() != 0 { try!(write!(f, " pol0"))}
      if self.pol1() != 0 { try!(write!(f, " pol1"))}
      if self.pol2() != 0 { try!(write!(f, " pol2"))}
      if self.pol3() != 0 { try!(write!(f, " pol3"))}
      if self.swap0() != 0 { try!(write!(f, " swap0"))}
      if self.swap1() != 0 { try!(write!(f, " swap1"))}
      if self.swap2() != 0 { try!(write!(f, " swap2"))}
      if self.swap3() != 0 { try!(write!(f, " swap3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Waveb(pub u32);
impl Waveb {
  #[inline]
  pub fn wavegenb(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
  #[inline]
  pub fn set_wavegenb(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn rampb(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  #[inline]
  pub fn set_rampb(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn ciperenb(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_ciperenb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn ciccenb0(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_ciccenb0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn ciccenb1(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_ciccenb1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn ciccenb2(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_ciccenb2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn ciccenb3(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_ciccenb3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn polb0(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
  pub fn set_polb0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn polb1(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline]
  pub fn set_polb1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline]
  pub fn polb2(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline]
  pub fn set_polb2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  #[inline]
  pub fn polb3(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  #[inline]
  pub fn set_polb3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  #[inline]
  pub fn swapb0(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  #[inline]
  pub fn set_swapb0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  #[inline]
  pub fn swapb1(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  #[inline]
  pub fn set_swapb1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  #[inline]
  pub fn swapb2(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  #[inline]
  pub fn set_swapb2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  #[inline]
  pub fn swapb3(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  #[inline]
  pub fn set_swapb3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

}
impl ::core::fmt::Display for Waveb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Waveb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wavegenb() != 0 { try!(write!(f, " wavegenb=0x{:x}", self.wavegenb()))}
      if self.rampb() != 0 { try!(write!(f, " rampb=0x{:x}", self.rampb()))}
      if self.ciperenb() != 0 { try!(write!(f, " ciperenb"))}
      if self.ciccenb0() != 0 { try!(write!(f, " ciccenb0"))}
      if self.ciccenb1() != 0 { try!(write!(f, " ciccenb1"))}
      if self.ciccenb2() != 0 { try!(write!(f, " ciccenb2"))}
      if self.ciccenb3() != 0 { try!(write!(f, " ciccenb3"))}
      if self.polb0() != 0 { try!(write!(f, " polb0"))}
      if self.polb1() != 0 { try!(write!(f, " polb1"))}
      if self.polb2() != 0 { try!(write!(f, " polb2"))}
      if self.polb3() != 0 { try!(write!(f, " polb3"))}
      if self.swapb0() != 0 { try!(write!(f, " swapb0"))}
      if self.swapb1() != 0 { try!(write!(f, " swapb1"))}
      if self.swapb2() != 0 { try!(write!(f, " swapb2"))}
      if self.swapb3() != 0 { try!(write!(f, " swapb3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Wexctrl(pub u32);
impl Wexctrl {
  #[inline]
  pub fn otmx(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline]
  pub fn set_otmx(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn dtien0(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_dtien0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn dtien1(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_dtien1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn dtien2(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_dtien2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn dtien3(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_dtien3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn dtls(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  #[inline]
  pub fn set_dtls(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn dths(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
  #[inline]
  pub fn set_dths(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

}
impl ::core::fmt::Display for Wexctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wexctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.otmx() != 0 { try!(write!(f, " otmx=0x{:x}", self.otmx()))}
      if self.dtien0() != 0 { try!(write!(f, " dtien0"))}
      if self.dtien1() != 0 { try!(write!(f, " dtien1"))}
      if self.dtien2() != 0 { try!(write!(f, " dtien2"))}
      if self.dtien3() != 0 { try!(write!(f, " dtien3"))}
      if self.dtls() != 0 { try!(write!(f, " dtls=0x{:x}", self.dtls()))}
      if self.dths() != 0 { try!(write!(f, " dths=0x{:x}", self.dths()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
