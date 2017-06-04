pub const TIMER0: Timer0 = Periph(0x40030000, Timer0Id {});
pub const TIMER1: Timer1 = Periph(0x40031000, Timer1Id {});
pub const TIMER2: Timer2 = Periph(0x40032000, Timer2Id {});
pub const TIMER3: Timer3 = Periph(0x40033000, Timer3Id {});
pub const TIMER4: Timer4 = Periph(0x40034000, Timer4Id {});
pub const TIMER5: Timer5 = Periph(0x40035000, Timer5Id {});
pub const TIMER6: Timer6 = Periph(0x400e0000, Timer6Id {});
pub const TIMER7: Timer7 = Periph(0x400e1000, Timer7Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Periph<T>(pub u32, pub T); 

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Timer0Id {}
pub type Timer0 = Periph<Timer0Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Timer1Id {}
pub type Timer1 = Periph<Timer1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Timer2Id {}
pub type Timer2 = Periph<Timer2Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Timer3Id {}
pub type Timer3 = Periph<Timer3Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Timer4Id {}
pub type Timer4 = Periph<Timer4Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Timer5Id {}
pub type Timer5 = Periph<Timer5Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Timer6Id {}
pub type Timer6 = Periph<Timer6Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Timer7Id {}
pub type Timer7 = Periph<Timer7Id>;










impl<T> Periph<T> {
  #[inline]
  pub fn cfg_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
  #[inline]
  pub fn cfg_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
  #[inline]
  pub fn cfg(&self) -> Cfg { 
     unsafe {
       Cfg(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  #[inline]
  pub fn set_cfg(&self, value: Cfg) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_cfg<F: FnOnce(Cfg) -> Cfg>(&self, f: F) -> &Self {
     let tmp = self.cfg();
     self.set_cfg(f(tmp))
  }

  #[inline]
  pub fn tamr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
  #[inline]
  pub fn tamr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
  #[inline]
  pub fn tamr(&self) -> Tamr { 
     unsafe {
       Tamr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
  #[inline]
  pub fn set_tamr(&self, value: Tamr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_tamr<F: FnOnce(Tamr) -> Tamr>(&self, f: F) -> &Self {
     let tmp = self.tamr();
     self.set_tamr(f(tmp))
  }

  #[inline]
  pub fn tbmr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
  #[inline]
  pub fn tbmr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
  #[inline]
  pub fn tbmr(&self) -> Tbmr { 
     unsafe {
       Tbmr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
  #[inline]
  pub fn set_tbmr(&self, value: Tbmr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_tbmr<F: FnOnce(Tbmr) -> Tbmr>(&self, f: F) -> &Self {
     let tmp = self.tbmr();
     self.set_tbmr(f(tmp))
  }

  #[inline]
  pub fn ctl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
  #[inline]
  pub fn ctl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
  #[inline]
  pub fn ctl(&self) -> Ctl { 
     unsafe {
       Ctl(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
  #[inline]
  pub fn set_ctl(&self, value: Ctl) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ctl<F: FnOnce(Ctl) -> Ctl>(&self, f: F) -> &Self {
     let tmp = self.ctl();
     self.set_ctl(f(tmp))
  }

  #[inline]
  pub fn sync_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
  #[inline]
  pub fn sync_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
  #[inline]
  pub fn sync(&self) -> Sync { 
     unsafe {
       Sync(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
  #[inline]
  pub fn set_sync(&self, value: Sync) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_sync<F: FnOnce(Sync) -> Sync>(&self, f: F) -> &Self {
     let tmp = self.sync();
     self.set_sync(f(tmp))
  }

  #[inline]
  pub fn imr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
  #[inline]
  pub fn imr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
  #[inline]
  pub fn imr(&self) -> Imr { 
     unsafe {
       Imr(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
  #[inline]
  pub fn set_imr(&self, value: Imr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_imr<F: FnOnce(Imr) -> Imr>(&self, f: F) -> &Self {
     let tmp = self.imr();
     self.set_imr(f(tmp))
  }

  #[inline]
  pub fn ris_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
  #[inline]
  pub fn ris_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
  #[inline]
  pub fn ris(&self) -> Ris { 
     unsafe {
       Ris(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
  #[inline]
  pub fn set_ris(&self, value: Ris) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
     let tmp = self.ris();
     self.set_ris(f(tmp))
  }

  #[inline]
  pub fn mis_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
  #[inline]
  pub fn mis_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
  #[inline]
  pub fn mis(&self) -> Mis { 
     unsafe {
       Mis(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
  #[inline]
  pub fn set_mis(&self, value: Mis) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_mis<F: FnOnce(Mis) -> Mis>(&self, f: F) -> &Self {
     let tmp = self.mis();
     self.set_mis(f(tmp))
  }

  #[inline]
  pub fn icr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
  #[inline]
  pub fn icr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
  #[inline]
  pub fn set_icr(&self, value: Icr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }

  #[inline]
  pub fn tailr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x28) as *const u32
  }
  #[inline]
  pub fn tailr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x28) as *mut u32
  }
  #[inline]
  pub fn tailr(&self) -> Tailr { 
     unsafe {
       Tailr(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
     }
  }
  #[inline]
  pub fn set_tailr(&self, value: Tailr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_tailr<F: FnOnce(Tailr) -> Tailr>(&self, f: F) -> &Self {
     let tmp = self.tailr();
     self.set_tailr(f(tmp))
  }

  #[inline]
  pub fn tbilr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2c) as *const u32
  }
  #[inline]
  pub fn tbilr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2c) as *mut u32
  }
  #[inline]
  pub fn tbilr(&self) -> Tbilr { 
     unsafe {
       Tbilr(::core::ptr::read_volatile(((self.0 as usize) + 0x2c) as *const u32))
     }
  }
  #[inline]
  pub fn set_tbilr(&self, value: Tbilr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x2c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_tbilr<F: FnOnce(Tbilr) -> Tbilr>(&self, f: F) -> &Self {
     let tmp = self.tbilr();
     self.set_tbilr(f(tmp))
  }

  #[inline]
  pub fn tamatchr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x30) as *const u32
  }
  #[inline]
  pub fn tamatchr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x30) as *mut u32
  }
  #[inline]
  pub fn tamatchr(&self) -> Tamatchr { 
     unsafe {
       Tamatchr(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u32))
     }
  }
  #[inline]
  pub fn set_tamatchr(&self, value: Tamatchr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_tamatchr<F: FnOnce(Tamatchr) -> Tamatchr>(&self, f: F) -> &Self {
     let tmp = self.tamatchr();
     self.set_tamatchr(f(tmp))
  }

  #[inline]
  pub fn tbmatchr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x34) as *const u32
  }
  #[inline]
  pub fn tbmatchr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x34) as *mut u32
  }
  #[inline]
  pub fn tbmatchr(&self) -> Tbmatchr { 
     unsafe {
       Tbmatchr(::core::ptr::read_volatile(((self.0 as usize) + 0x34) as *const u32))
     }
  }
  #[inline]
  pub fn set_tbmatchr(&self, value: Tbmatchr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x34) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_tbmatchr<F: FnOnce(Tbmatchr) -> Tbmatchr>(&self, f: F) -> &Self {
     let tmp = self.tbmatchr();
     self.set_tbmatchr(f(tmp))
  }

  #[inline]
  pub fn tapr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x38) as *const u32
  }
  #[inline]
  pub fn tapr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x38) as *mut u32
  }
  #[inline]
  pub fn tapr(&self) -> Tapr { 
     unsafe {
       Tapr(::core::ptr::read_volatile(((self.0 as usize) + 0x38) as *const u32))
     }
  }
  #[inline]
  pub fn set_tapr(&self, value: Tapr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x38) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_tapr<F: FnOnce(Tapr) -> Tapr>(&self, f: F) -> &Self {
     let tmp = self.tapr();
     self.set_tapr(f(tmp))
  }

  #[inline]
  pub fn tbpr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x3c) as *const u32
  }
  #[inline]
  pub fn tbpr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x3c) as *mut u32
  }
  #[inline]
  pub fn tbpr(&self) -> Tbpr { 
     unsafe {
       Tbpr(::core::ptr::read_volatile(((self.0 as usize) + 0x3c) as *const u32))
     }
  }
  #[inline]
  pub fn set_tbpr(&self, value: Tbpr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x3c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_tbpr<F: FnOnce(Tbpr) -> Tbpr>(&self, f: F) -> &Self {
     let tmp = self.tbpr();
     self.set_tbpr(f(tmp))
  }

  #[inline]
  pub fn tapmr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x40) as *const u32
  }
  #[inline]
  pub fn tapmr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x40) as *mut u32
  }
  #[inline]
  pub fn tapmr(&self) -> Tapmr { 
     unsafe {
       Tapmr(::core::ptr::read_volatile(((self.0 as usize) + 0x40) as *const u32))
     }
  }
  #[inline]
  pub fn set_tapmr(&self, value: Tapmr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_tapmr<F: FnOnce(Tapmr) -> Tapmr>(&self, f: F) -> &Self {
     let tmp = self.tapmr();
     self.set_tapmr(f(tmp))
  }

  #[inline]
  pub fn tbpmr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x44) as *const u32
  }
  #[inline]
  pub fn tbpmr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x44) as *mut u32
  }
  #[inline]
  pub fn tbpmr(&self) -> Tbpmr { 
     unsafe {
       Tbpmr(::core::ptr::read_volatile(((self.0 as usize) + 0x44) as *const u32))
     }
  }
  #[inline]
  pub fn set_tbpmr(&self, value: Tbpmr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x44) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_tbpmr<F: FnOnce(Tbpmr) -> Tbpmr>(&self, f: F) -> &Self {
     let tmp = self.tbpmr();
     self.set_tbpmr(f(tmp))
  }

  #[inline]
  pub fn tar_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x48) as *const u32
  }
  #[inline]
  pub fn tar_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x48) as *mut u32
  }
  #[inline]
  pub fn tar(&self) -> Tar { 
     unsafe {
       Tar(::core::ptr::read_volatile(((self.0 as usize) + 0x48) as *const u32))
     }
  }
  #[inline]
  pub fn set_tar(&self, value: Tar) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x48) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_tar<F: FnOnce(Tar) -> Tar>(&self, f: F) -> &Self {
     let tmp = self.tar();
     self.set_tar(f(tmp))
  }

  #[inline]
  pub fn tbr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4c) as *const u32
  }
  #[inline]
  pub fn tbr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4c) as *mut u32
  }
  #[inline]
  pub fn tbr(&self) -> Tbr { 
     unsafe {
       Tbr(::core::ptr::read_volatile(((self.0 as usize) + 0x4c) as *const u32))
     }
  }
  #[inline]
  pub fn set_tbr(&self, value: Tbr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x4c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_tbr<F: FnOnce(Tbr) -> Tbr>(&self, f: F) -> &Self {
     let tmp = self.tbr();
     self.set_tbr(f(tmp))
  }

  #[inline]
  pub fn tav_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x50) as *const u32
  }
  #[inline]
  pub fn tav_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x50) as *mut u32
  }
  #[inline]
  pub fn tav(&self) -> Tav { 
     unsafe {
       Tav(::core::ptr::read_volatile(((self.0 as usize) + 0x50) as *const u32))
     }
  }
  #[inline]
  pub fn set_tav(&self, value: Tav) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x50) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_tav<F: FnOnce(Tav) -> Tav>(&self, f: F) -> &Self {
     let tmp = self.tav();
     self.set_tav(f(tmp))
  }

  #[inline]
  pub fn tbv_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x54) as *const u32
  }
  #[inline]
  pub fn tbv_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x54) as *mut u32
  }
  #[inline]
  pub fn tbv(&self) -> Tbv { 
     unsafe {
       Tbv(::core::ptr::read_volatile(((self.0 as usize) + 0x54) as *const u32))
     }
  }
  #[inline]
  pub fn set_tbv(&self, value: Tbv) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x54) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_tbv<F: FnOnce(Tbv) -> Tbv>(&self, f: F) -> &Self {
     let tmp = self.tbv();
     self.set_tbv(f(tmp))
  }

  #[inline]
  pub fn rtcpd_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x58) as *const u32
  }
  #[inline]
  pub fn rtcpd_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x58) as *mut u32
  }
  #[inline]
  pub fn rtcpd(&self) -> Rtcpd { 
     unsafe {
       Rtcpd(::core::ptr::read_volatile(((self.0 as usize) + 0x58) as *const u32))
     }
  }
  #[inline]
  pub fn set_rtcpd(&self, value: Rtcpd) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x58) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_rtcpd<F: FnOnce(Rtcpd) -> Rtcpd>(&self, f: F) -> &Self {
     let tmp = self.rtcpd();
     self.set_rtcpd(f(tmp))
  }

  #[inline]
  pub fn taps_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x5c) as *const u32
  }
  #[inline]
  pub fn taps_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x5c) as *mut u32
  }
  #[inline]
  pub fn taps(&self) -> Taps { 
     unsafe {
       Taps(::core::ptr::read_volatile(((self.0 as usize) + 0x5c) as *const u32))
     }
  }
  #[inline]
  pub fn set_taps(&self, value: Taps) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x5c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_taps<F: FnOnce(Taps) -> Taps>(&self, f: F) -> &Self {
     let tmp = self.taps();
     self.set_taps(f(tmp))
  }

  #[inline]
  pub fn tbps_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x60) as *const u32
  }
  #[inline]
  pub fn tbps_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x60) as *mut u32
  }
  #[inline]
  pub fn tbps(&self) -> Tbps { 
     unsafe {
       Tbps(::core::ptr::read_volatile(((self.0 as usize) + 0x60) as *const u32))
     }
  }
  #[inline]
  pub fn set_tbps(&self, value: Tbps) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x60) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_tbps<F: FnOnce(Tbps) -> Tbps>(&self, f: F) -> &Self {
     let tmp = self.tbps();
     self.set_tbps(f(tmp))
  }

  #[inline]
  pub fn dmaev_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x6c) as *const u32
  }
  #[inline]
  pub fn dmaev_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x6c) as *mut u32
  }
  #[inline]
  pub fn dmaev(&self) -> Dmaev { 
     unsafe {
       Dmaev(::core::ptr::read_volatile(((self.0 as usize) + 0x6c) as *const u32))
     }
  }
  #[inline]
  pub fn set_dmaev(&self, value: Dmaev) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x6c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dmaev<F: FnOnce(Dmaev) -> Dmaev>(&self, f: F) -> &Self {
     let tmp = self.dmaev();
     self.set_dmaev(f(tmp))
  }

  #[inline]
  pub fn adcev_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x70) as *const u32
  }
  #[inline]
  pub fn adcev_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x70) as *mut u32
  }
  #[inline]
  pub fn adcev(&self) -> Adcev { 
     unsafe {
       Adcev(::core::ptr::read_volatile(((self.0 as usize) + 0x70) as *const u32))
     }
  }
  #[inline]
  pub fn set_adcev(&self, value: Adcev) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x70) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_adcev<F: FnOnce(Adcev) -> Adcev>(&self, f: F) -> &Self {
     let tmp = self.adcev();
     self.set_adcev(f(tmp))
  }

  #[inline]
  pub fn pp_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xfc0) as *const u32
  }
  #[inline]
  pub fn pp_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xfc0) as *mut u32
  }
  #[inline]
  pub fn pp(&self) -> Pp { 
     unsafe {
       Pp(::core::ptr::read_volatile(((self.0 as usize) + 0xfc0) as *const u32))
     }
  }
  #[inline]
  pub fn set_pp(&self, value: Pp) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xfc0) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_pp<F: FnOnce(Pp) -> Pp>(&self, f: F) -> &Self {
     let tmp = self.pp();
     self.set_pp(f(tmp))
  }

  #[inline]
  pub fn cc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xfc8) as *const u32
  }
  #[inline]
  pub fn cc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xfc8) as *mut u32
  }
  #[inline]
  pub fn cc(&self) -> Cc { 
     unsafe {
       Cc(::core::ptr::read_volatile(((self.0 as usize) + 0xfc8) as *const u32))
     }
  }
  #[inline]
  pub fn set_cc(&self, value: Cc) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xfc8) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_cc<F: FnOnce(Cc) -> Cc>(&self, f: F) -> &Self {
     let tmp = self.cc();
     self.set_cc(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Cfg(pub u32);
impl Cfg {
  #[inline]
  pub fn cfg(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
  #[inline]
  pub fn set_cfg(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cfg() != 0 { try!(write!(f, " cfg=0x{:x}", self.cfg()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Tamr(pub u32);
impl Tamr {
  #[inline]
  pub fn tamr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline]
  pub fn set_tamr(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn tacmr(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_tacmr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn taams(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_taams(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn tacdir(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_tacdir(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn tamie(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_tamie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn tawot(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_tawot(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn tasnaps(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_tasnaps(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn taild(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_taild(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn tapwmie(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_tapwmie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn tamrsu(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_tamrsu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn taplo(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_taplo(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn tacintd(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline]
  pub fn set_tacintd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn tcact(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x7 // [15:13]
  }
  #[inline]
  pub fn set_tcact(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 13);
     self.0 |= value << 13;
     self
  }

}
impl ::core::fmt::Display for Tamr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tamr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tamr() != 0 { try!(write!(f, " tamr=0x{:x}", self.tamr()))}
      if self.tacmr() != 0 { try!(write!(f, " tacmr"))}
      if self.taams() != 0 { try!(write!(f, " taams"))}
      if self.tacdir() != 0 { try!(write!(f, " tacdir"))}
      if self.tamie() != 0 { try!(write!(f, " tamie"))}
      if self.tawot() != 0 { try!(write!(f, " tawot"))}
      if self.tasnaps() != 0 { try!(write!(f, " tasnaps"))}
      if self.taild() != 0 { try!(write!(f, " taild"))}
      if self.tapwmie() != 0 { try!(write!(f, " tapwmie"))}
      if self.tamrsu() != 0 { try!(write!(f, " tamrsu"))}
      if self.taplo() != 0 { try!(write!(f, " taplo"))}
      if self.tacintd() != 0 { try!(write!(f, " tacintd"))}
      if self.tcact() != 0 { try!(write!(f, " tcact=0x{:x}", self.tcact()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Tbmr(pub u32);
impl Tbmr {
  #[inline]
  pub fn tbmr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline]
  pub fn set_tbmr(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn tbcmr(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_tbcmr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn tbams(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_tbams(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn tbcdir(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_tbcdir(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn tbmie(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_tbmie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn tbwot(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_tbwot(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn tbsnaps(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_tbsnaps(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn tbild(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_tbild(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn tbpwmie(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_tbpwmie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn tbmrsu(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_tbmrsu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn tbplo(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_tbplo(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn tbcintd(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline]
  pub fn set_tbcintd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn tcact(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x7 // [15:13]
  }
  #[inline]
  pub fn set_tcact(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 13);
     self.0 |= value << 13;
     self
  }

}
impl ::core::fmt::Display for Tbmr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tbmr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tbmr() != 0 { try!(write!(f, " tbmr=0x{:x}", self.tbmr()))}
      if self.tbcmr() != 0 { try!(write!(f, " tbcmr"))}
      if self.tbams() != 0 { try!(write!(f, " tbams"))}
      if self.tbcdir() != 0 { try!(write!(f, " tbcdir"))}
      if self.tbmie() != 0 { try!(write!(f, " tbmie"))}
      if self.tbwot() != 0 { try!(write!(f, " tbwot"))}
      if self.tbsnaps() != 0 { try!(write!(f, " tbsnaps"))}
      if self.tbild() != 0 { try!(write!(f, " tbild"))}
      if self.tbpwmie() != 0 { try!(write!(f, " tbpwmie"))}
      if self.tbmrsu() != 0 { try!(write!(f, " tbmrsu"))}
      if self.tbplo() != 0 { try!(write!(f, " tbplo"))}
      if self.tbcintd() != 0 { try!(write!(f, " tbcintd"))}
      if self.tcact() != 0 { try!(write!(f, " tcact=0x{:x}", self.tcact()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ctl(pub u32);
impl Ctl {
  #[inline]
  pub fn taen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_taen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn tastall(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_tastall(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn taevent(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
  #[inline]
  pub fn set_taevent(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn rtcen(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_rtcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn taote(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_taote(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn tapwml(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_tapwml(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn tben(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_tben(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn tbstall(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_tbstall(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn tbevent(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
  #[inline]
  pub fn set_tbevent(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn tbote(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_tbote(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline]
  pub fn tbpwml(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline]
  pub fn set_tbpwml(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
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
      if self.taen() != 0 { try!(write!(f, " taen"))}
      if self.tastall() != 0 { try!(write!(f, " tastall"))}
      if self.taevent() != 0 { try!(write!(f, " taevent=0x{:x}", self.taevent()))}
      if self.rtcen() != 0 { try!(write!(f, " rtcen"))}
      if self.taote() != 0 { try!(write!(f, " taote"))}
      if self.tapwml() != 0 { try!(write!(f, " tapwml"))}
      if self.tben() != 0 { try!(write!(f, " tben"))}
      if self.tbstall() != 0 { try!(write!(f, " tbstall"))}
      if self.tbevent() != 0 { try!(write!(f, " tbevent=0x{:x}", self.tbevent()))}
      if self.tbote() != 0 { try!(write!(f, " tbote"))}
      if self.tbpwml() != 0 { try!(write!(f, " tbpwml"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Sync(pub u32);
impl Sync {
  #[inline]
  pub fn synct0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline]
  pub fn set_synct0(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn synct1(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
  #[inline]
  pub fn set_synct1(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn synct2(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  #[inline]
  pub fn set_synct2(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn synct3(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
  #[inline]
  pub fn set_synct3(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn synct4(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  #[inline]
  pub fn set_synct4(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn synct5(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
  #[inline]
  pub fn set_synct5(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn synct6(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x3 // [13:12]
  }
  #[inline]
  pub fn set_synct6(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn synct7(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x3 // [15:14]
  }
  #[inline]
  pub fn set_synct7(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 14);
     self.0 |= value << 14;
     self
  }

}
impl ::core::fmt::Display for Sync {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sync {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.synct0() != 0 { try!(write!(f, " synct0=0x{:x}", self.synct0()))}
      if self.synct1() != 0 { try!(write!(f, " synct1=0x{:x}", self.synct1()))}
      if self.synct2() != 0 { try!(write!(f, " synct2=0x{:x}", self.synct2()))}
      if self.synct3() != 0 { try!(write!(f, " synct3=0x{:x}", self.synct3()))}
      if self.synct4() != 0 { try!(write!(f, " synct4=0x{:x}", self.synct4()))}
      if self.synct5() != 0 { try!(write!(f, " synct5=0x{:x}", self.synct5()))}
      if self.synct6() != 0 { try!(write!(f, " synct6=0x{:x}", self.synct6()))}
      if self.synct7() != 0 { try!(write!(f, " synct7=0x{:x}", self.synct7()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Imr(pub u32);
impl Imr {
  #[inline]
  pub fn tatoim(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_tatoim(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn camim(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_camim(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn caeim(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_caeim(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn rtcim(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_rtcim(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn tamim(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_tamim(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn dmaaim(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_dmaaim(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn tbtoim(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_tbtoim(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn cbmim(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_cbmim(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn cbeim(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_cbeim(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn tbmim(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_tbmim(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn dmabim(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_dmabim(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

}
impl ::core::fmt::Display for Imr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Imr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tatoim() != 0 { try!(write!(f, " tatoim"))}
      if self.camim() != 0 { try!(write!(f, " camim"))}
      if self.caeim() != 0 { try!(write!(f, " caeim"))}
      if self.rtcim() != 0 { try!(write!(f, " rtcim"))}
      if self.tamim() != 0 { try!(write!(f, " tamim"))}
      if self.dmaaim() != 0 { try!(write!(f, " dmaaim"))}
      if self.tbtoim() != 0 { try!(write!(f, " tbtoim"))}
      if self.cbmim() != 0 { try!(write!(f, " cbmim"))}
      if self.cbeim() != 0 { try!(write!(f, " cbeim"))}
      if self.tbmim() != 0 { try!(write!(f, " tbmim"))}
      if self.dmabim() != 0 { try!(write!(f, " dmabim"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ris(pub u32);
impl Ris {
  #[inline]
  pub fn tatoris(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_tatoris(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn camris(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_camris(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn caeris(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_caeris(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn rtcris(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_rtcris(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn tamris(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_tamris(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn dmaaris(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_dmaaris(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn tbtoris(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_tbtoris(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn cbmris(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_cbmris(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn cberis(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_cberis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn tbmris(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_tbmris(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn dmabris(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_dmabris(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
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
      if self.tatoris() != 0 { try!(write!(f, " tatoris"))}
      if self.camris() != 0 { try!(write!(f, " camris"))}
      if self.caeris() != 0 { try!(write!(f, " caeris"))}
      if self.rtcris() != 0 { try!(write!(f, " rtcris"))}
      if self.tamris() != 0 { try!(write!(f, " tamris"))}
      if self.dmaaris() != 0 { try!(write!(f, " dmaaris"))}
      if self.tbtoris() != 0 { try!(write!(f, " tbtoris"))}
      if self.cbmris() != 0 { try!(write!(f, " cbmris"))}
      if self.cberis() != 0 { try!(write!(f, " cberis"))}
      if self.tbmris() != 0 { try!(write!(f, " tbmris"))}
      if self.dmabris() != 0 { try!(write!(f, " dmabris"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Mis(pub u32);
impl Mis {
  #[inline]
  pub fn tatomis(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_tatomis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn cammis(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_cammis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn caemis(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_caemis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn rtcmis(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_rtcmis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn tammis(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_tammis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn dmaamis(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_dmaamis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn tbtomis(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_tbtomis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn cbmmis(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_cbmmis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn cbemis(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_cbemis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn tbmmis(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_tbmmis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn dmabmis(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_dmabmis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
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
      if self.tatomis() != 0 { try!(write!(f, " tatomis"))}
      if self.cammis() != 0 { try!(write!(f, " cammis"))}
      if self.caemis() != 0 { try!(write!(f, " caemis"))}
      if self.rtcmis() != 0 { try!(write!(f, " rtcmis"))}
      if self.tammis() != 0 { try!(write!(f, " tammis"))}
      if self.dmaamis() != 0 { try!(write!(f, " dmaamis"))}
      if self.tbtomis() != 0 { try!(write!(f, " tbtomis"))}
      if self.cbmmis() != 0 { try!(write!(f, " cbmmis"))}
      if self.cbemis() != 0 { try!(write!(f, " cbemis"))}
      if self.tbmmis() != 0 { try!(write!(f, " tbmmis"))}
      if self.dmabmis() != 0 { try!(write!(f, " dmabmis"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
  #[inline]
  pub fn tatocint(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_tatocint(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn camcint(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_camcint(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn caecint(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_caecint(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn rtccint(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_rtccint(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn tamcint(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_tamcint(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn dmaaint(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_dmaaint(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn tbtocint(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_tbtocint(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn cbmcint(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_cbmcint(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn cbecint(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_cbecint(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn tbmcint(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_tbmcint(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn dmabint(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_dmabint(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
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
      if self.tatocint() != 0 { try!(write!(f, " tatocint"))}
      if self.camcint() != 0 { try!(write!(f, " camcint"))}
      if self.caecint() != 0 { try!(write!(f, " caecint"))}
      if self.rtccint() != 0 { try!(write!(f, " rtccint"))}
      if self.tamcint() != 0 { try!(write!(f, " tamcint"))}
      if self.dmaaint() != 0 { try!(write!(f, " dmaaint"))}
      if self.tbtocint() != 0 { try!(write!(f, " tbtocint"))}
      if self.cbmcint() != 0 { try!(write!(f, " cbmcint"))}
      if self.cbecint() != 0 { try!(write!(f, " cbecint"))}
      if self.tbmcint() != 0 { try!(write!(f, " tbmcint"))}
      if self.dmabint() != 0 { try!(write!(f, " dmabint"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Tailr(pub u32);
impl Tailr {
  #[inline]
  pub fn tailr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
  pub fn set_tailr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tailr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tailr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Tbilr(pub u32);
impl Tbilr {
  #[inline]
  pub fn tbilr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
  pub fn set_tbilr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tbilr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tbilr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Tamatchr(pub u32);
impl Tamatchr {
  #[inline]
  pub fn tamatchr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
  pub fn set_tamatchr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tamatchr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tamatchr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Tbmatchr(pub u32);
impl Tbmatchr {
  #[inline]
  pub fn tbmatchr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
  pub fn set_tbmatchr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tbmatchr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tbmatchr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Tapr(pub u32);
impl Tapr {
  #[inline]
  pub fn tapsr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  #[inline]
  pub fn set_tapsr(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tapr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tapr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tapsr() != 0 { try!(write!(f, " tapsr=0x{:x}", self.tapsr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Tbpr(pub u32);
impl Tbpr {
  #[inline]
  pub fn tbpsr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  #[inline]
  pub fn set_tbpsr(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tbpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tbpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tbpsr() != 0 { try!(write!(f, " tbpsr=0x{:x}", self.tbpsr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Tapmr(pub u32);
impl Tapmr {
  #[inline]
  pub fn tapsmr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  #[inline]
  pub fn set_tapsmr(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tapmr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tapmr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tapsmr() != 0 { try!(write!(f, " tapsmr=0x{:x}", self.tapsmr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Tbpmr(pub u32);
impl Tbpmr {
  #[inline]
  pub fn tbpsmr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  #[inline]
  pub fn set_tbpsmr(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tbpmr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tbpmr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tbpsmr() != 0 { try!(write!(f, " tbpsmr=0x{:x}", self.tbpsmr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Tar(pub u32);
impl Tar {
  #[inline]
  pub fn tar(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
  pub fn set_tar(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Tbr(pub u32);
impl Tbr {
  #[inline]
  pub fn tbr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
  pub fn set_tbr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tbr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tbr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Tav(pub u32);
impl Tav {
  #[inline]
  pub fn tav(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
  pub fn set_tav(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tav {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tav {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Tbv(pub u32);
impl Tbv {
  #[inline]
  pub fn tbv(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
  pub fn set_tbv(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tbv {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tbv {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Rtcpd(pub u32);
impl Rtcpd {
  #[inline]
  pub fn rtcpd(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline]
  pub fn set_rtcpd(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Rtcpd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rtcpd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rtcpd() != 0 { try!(write!(f, " rtcpd=0x{:x}", self.rtcpd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Taps(pub u32);
impl Taps {
  #[inline]
  pub fn pss(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline]
  pub fn set_pss(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Taps {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Taps {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pss() != 0 { try!(write!(f, " pss=0x{:x}", self.pss()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Tbps(pub u32);
impl Tbps {
  #[inline]
  pub fn pss(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline]
  pub fn set_pss(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tbps {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tbps {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pss() != 0 { try!(write!(f, " pss=0x{:x}", self.pss()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dmaev(pub u32);
impl Dmaev {
  #[inline]
  pub fn tatodmaen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_tatodmaen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn camdmaen(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_camdmaen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn caedmaen(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_caedmaen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn rtcdmaen(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_rtcdmaen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn tamdmaen(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_tamdmaen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn tbtodmaen(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_tbtodmaen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn cbmdmaen(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_cbmdmaen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn cbedmaen(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_cbedmaen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn tbmdmaen(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_tbmdmaen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

}
impl ::core::fmt::Display for Dmaev {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dmaev {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tatodmaen() != 0 { try!(write!(f, " tatodmaen"))}
      if self.camdmaen() != 0 { try!(write!(f, " camdmaen"))}
      if self.caedmaen() != 0 { try!(write!(f, " caedmaen"))}
      if self.rtcdmaen() != 0 { try!(write!(f, " rtcdmaen"))}
      if self.tamdmaen() != 0 { try!(write!(f, " tamdmaen"))}
      if self.tbtodmaen() != 0 { try!(write!(f, " tbtodmaen"))}
      if self.cbmdmaen() != 0 { try!(write!(f, " cbmdmaen"))}
      if self.cbedmaen() != 0 { try!(write!(f, " cbedmaen"))}
      if self.tbmdmaen() != 0 { try!(write!(f, " tbmdmaen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Adcev(pub u32);
impl Adcev {
  #[inline]
  pub fn tatoadcen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_tatoadcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn camadcen(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_camadcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn caeadcen(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_caeadcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn rtcadcen(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_rtcadcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn tamadcen(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_tamadcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn tbtoadcen(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_tbtoadcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn cbmadcen(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_cbmadcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn cbeadcen(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_cbeadcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn tbmadcen(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_tbmadcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

}
impl ::core::fmt::Display for Adcev {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Adcev {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tatoadcen() != 0 { try!(write!(f, " tatoadcen"))}
      if self.camadcen() != 0 { try!(write!(f, " camadcen"))}
      if self.caeadcen() != 0 { try!(write!(f, " caeadcen"))}
      if self.rtcadcen() != 0 { try!(write!(f, " rtcadcen"))}
      if self.tamadcen() != 0 { try!(write!(f, " tamadcen"))}
      if self.tbtoadcen() != 0 { try!(write!(f, " tbtoadcen"))}
      if self.cbmadcen() != 0 { try!(write!(f, " cbmadcen"))}
      if self.cbeadcen() != 0 { try!(write!(f, " cbeadcen"))}
      if self.tbmadcen() != 0 { try!(write!(f, " tbmadcen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Pp(pub u32);
impl Pp {
  #[inline]
  pub fn size(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  #[inline]
  pub fn set_size(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn chain(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_chain(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn synccnt(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_synccnt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn altclk(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_altclk(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

}
impl ::core::fmt::Display for Pp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.size() != 0 { try!(write!(f, " size=0x{:x}", self.size()))}
      if self.chain() != 0 { try!(write!(f, " chain"))}
      if self.synccnt() != 0 { try!(write!(f, " synccnt"))}
      if self.altclk() != 0 { try!(write!(f, " altclk"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cc(pub u32);
impl Cc {
  #[inline]
  pub fn altclk(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_altclk(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
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
      if self.altclk() != 0 { try!(write!(f, " altclk"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
