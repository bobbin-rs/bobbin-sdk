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

impl super::sig::Signal<super::sig::T0ccp0> for Timer0a {}
impl super::sig::SignalCcp<super::sig::T0ccp0> for Timer0a {}
impl super::sig::Signal<super::sig::T0ccp1> for Timer0b {}
impl super::sig::SignalCcp<super::sig::T0ccp1> for Timer0b {}

impl super::sig::Signal<super::sig::T1ccp0> for Timer1a {}
impl super::sig::SignalCcp<super::sig::T1ccp0> for Timer1a {}
impl super::sig::Signal<super::sig::T1ccp1> for Timer1b {}
impl super::sig::SignalCcp<super::sig::T1ccp1> for Timer1b {}

impl super::sig::Signal<super::sig::T2ccp0> for Timer2a {}
impl super::sig::SignalCcp<super::sig::T2ccp0> for Timer2a {}
impl super::sig::Signal<super::sig::T2ccp1> for Timer2b {}
impl super::sig::SignalCcp<super::sig::T2ccp1> for Timer2b {}

impl super::sig::Signal<super::sig::T3ccp0> for Timer3a {}
impl super::sig::SignalCcp<super::sig::T3ccp0> for Timer3a {}
impl super::sig::Signal<super::sig::T3ccp1> for Timer3b {}
impl super::sig::SignalCcp<super::sig::T3ccp1> for Timer3b {}

impl super::sig::Signal<super::sig::T4ccp0> for Timer4a {}
impl super::sig::SignalCcp<super::sig::T4ccp0> for Timer4a {}
impl super::sig::Signal<super::sig::T4ccp1> for Timer4b {}
impl super::sig::SignalCcp<super::sig::T4ccp1> for Timer4b {}

impl super::sig::Signal<super::sig::T5ccp0> for Timer5a {}
impl super::sig::SignalCcp<super::sig::T5ccp0> for Timer5a {}
impl super::sig::Signal<super::sig::T5ccp1> for Timer5b {}
impl super::sig::SignalCcp<super::sig::T5ccp1> for Timer5b {}

impl super::sig::Signal<super::sig::T6ccp0> for Timer6a {}
impl super::sig::SignalCcp<super::sig::T6ccp0> for Timer6a {}
impl super::sig::Signal<super::sig::T6ccp1> for Timer6b {}
impl super::sig::SignalCcp<super::sig::T6ccp1> for Timer6b {}

impl super::sig::Signal<super::sig::T7ccp0> for Timer7a {}
impl super::sig::SignalCcp<super::sig::T7ccp0> for Timer7a {}
impl super::sig::Signal<super::sig::T7ccp1> for Timer7b {}
impl super::sig::SignalCcp<super::sig::T7ccp1> for Timer7b {}


impl<T> Periph<T> {
  #[inline] pub fn cfg_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
  #[inline] pub fn cfg_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
  #[inline] pub fn cfg(&self) -> Cfg { 
     unsafe {
        Cfg(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  #[inline] pub fn set_cfg(&self, value: Cfg) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_cfg<F: FnOnce(Cfg) -> Cfg>(&self, f: F) -> &Self {
     let tmp = self.cfg();
     self.set_cfg(f(tmp))
  }

  #[inline] pub fn tmr_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x4 + (index << 2)) as *const u32
  }
  #[inline] pub fn tmr_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x4 + (index << 2)) as *mut u32
  }
  #[inline] pub fn tmr(&self, index: usize) -> Tmr { 
     assert!(index < 2);
     unsafe {
        Tmr(::core::ptr::read_volatile(((self.0 as usize) + 0x4 + (index << 2)) as *const u32))
     }
  }
  #[inline] pub fn set_tmr(&self, index: usize, value: Tmr) -> &Self {
     assert!(index < 2);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_tmr<F: FnOnce(Tmr) -> Tmr>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tmr(index);
     self.set_tmr(index, f(tmp))
  }

  #[inline] pub fn ctl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
  #[inline] pub fn ctl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
  #[inline] pub fn ctl(&self) -> Ctl { 
     unsafe {
        Ctl(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
  #[inline] pub fn set_ctl(&self, value: Ctl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ctl<F: FnOnce(Ctl) -> Ctl>(&self, f: F) -> &Self {
     let tmp = self.ctl();
     self.set_ctl(f(tmp))
  }

  #[inline] pub fn sync_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
  #[inline] pub fn sync_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
  #[inline] pub fn sync(&self) -> Sync { 
     unsafe {
        Sync(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
  #[inline] pub fn set_sync(&self, value: Sync) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_sync<F: FnOnce(Sync) -> Sync>(&self, f: F) -> &Self {
     let tmp = self.sync();
     self.set_sync(f(tmp))
  }

  #[inline] pub fn imr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
  #[inline] pub fn imr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
  #[inline] pub fn imr(&self) -> Imr { 
     unsafe {
        Imr(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
  #[inline] pub fn set_imr(&self, value: Imr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_imr<F: FnOnce(Imr) -> Imr>(&self, f: F) -> &Self {
     let tmp = self.imr();
     self.set_imr(f(tmp))
  }

  #[inline] pub fn ris_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
  #[inline] pub fn ris_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
  #[inline] pub fn ris(&self) -> Ris { 
     unsafe {
        Ris(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
  #[inline] pub fn set_ris(&self, value: Ris) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
     let tmp = self.ris();
     self.set_ris(f(tmp))
  }

  #[inline] pub fn mis_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
  #[inline] pub fn mis_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
  #[inline] pub fn mis(&self) -> Mis { 
     unsafe {
        Mis(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
  #[inline] pub fn set_mis(&self, value: Mis) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_mis<F: FnOnce(Mis) -> Mis>(&self, f: F) -> &Self {
     let tmp = self.mis();
     self.set_mis(f(tmp))
  }

  #[inline] pub fn icr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
  #[inline] pub fn icr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
  #[inline] pub fn set_icr(&self, value: Icr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }

  #[inline] pub fn tilr_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x28 + (index << 2)) as *const u32
  }
  #[inline] pub fn tilr_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x28 + (index << 2)) as *mut u32
  }
  #[inline] pub fn tilr(&self, index: usize) -> Tilr { 
     assert!(index < 2);
     unsafe {
        Tilr(::core::ptr::read_volatile(((self.0 as usize) + 0x28 + (index << 2)) as *const u32))
     }
  }
  #[inline] pub fn set_tilr(&self, index: usize, value: Tilr) -> &Self {
     assert!(index < 2);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x28 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_tilr<F: FnOnce(Tilr) -> Tilr>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tilr(index);
     self.set_tilr(index, f(tmp))
  }

  #[inline] pub fn tmtchr_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x30 + (index << 2)) as *const u32
  }
  #[inline] pub fn tmtchr_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x30 + (index << 2)) as *mut u32
  }
  #[inline] pub fn tmtchr(&self, index: usize) -> Tmtchr { 
     assert!(index < 2);
     unsafe {
        Tmtchr(::core::ptr::read_volatile(((self.0 as usize) + 0x30 + (index << 2)) as *const u32))
     }
  }
  #[inline] pub fn set_tmtchr(&self, index: usize, value: Tmtchr) -> &Self {
     assert!(index < 2);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x30 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_tmtchr<F: FnOnce(Tmtchr) -> Tmtchr>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tmtchr(index);
     self.set_tmtchr(index, f(tmp))
  }

  #[inline] pub fn tpr_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x38 + (index << 2)) as *const u32
  }
  #[inline] pub fn tpr_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x38 + (index << 2)) as *mut u32
  }
  #[inline] pub fn tpr(&self, index: usize) -> Tpr { 
     assert!(index < 2);
     unsafe {
        Tpr(::core::ptr::read_volatile(((self.0 as usize) + 0x38 + (index << 2)) as *const u32))
     }
  }
  #[inline] pub fn set_tpr(&self, index: usize, value: Tpr) -> &Self {
     assert!(index < 2);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x38 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_tpr<F: FnOnce(Tpr) -> Tpr>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tpr(index);
     self.set_tpr(index, f(tmp))
  }

  #[inline] pub fn tpmr_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x40 + (index << 2)) as *const u32
  }
  #[inline] pub fn tpmr_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x40 + (index << 2)) as *mut u32
  }
  #[inline] pub fn tpmr(&self, index: usize) -> Tpmr { 
     assert!(index < 2);
     unsafe {
        Tpmr(::core::ptr::read_volatile(((self.0 as usize) + 0x40 + (index << 2)) as *const u32))
     }
  }
  #[inline] pub fn set_tpmr(&self, index: usize, value: Tpmr) -> &Self {
     assert!(index < 2);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x40 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_tpmr<F: FnOnce(Tpmr) -> Tpmr>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tpmr(index);
     self.set_tpmr(index, f(tmp))
  }

  #[inline] pub fn tr_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x48 + (index << 2)) as *const u32
  }
  #[inline] pub fn tr_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x48 + (index << 2)) as *mut u32
  }
  #[inline] pub fn tr(&self, index: usize) -> Tr { 
     assert!(index < 2);
     unsafe {
        Tr(::core::ptr::read_volatile(((self.0 as usize) + 0x48 + (index << 2)) as *const u32))
     }
  }
  #[inline] pub fn set_tr(&self, index: usize, value: Tr) -> &Self {
     assert!(index < 2);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x48 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_tr<F: FnOnce(Tr) -> Tr>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tr(index);
     self.set_tr(index, f(tmp))
  }

  #[inline] pub fn tv_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x50 + (index << 2)) as *const u32
  }
  #[inline] pub fn tv_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x50 + (index << 2)) as *mut u32
  }
  #[inline] pub fn tv(&self, index: usize) -> Tv { 
     assert!(index < 2);
     unsafe {
        Tv(::core::ptr::read_volatile(((self.0 as usize) + 0x50 + (index << 2)) as *const u32))
     }
  }
  #[inline] pub fn set_tv(&self, index: usize, value: Tv) -> &Self {
     assert!(index < 2);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x50 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_tv<F: FnOnce(Tv) -> Tv>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tv(index);
     self.set_tv(index, f(tmp))
  }

  #[inline] pub fn rtcpd_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x58) as *const u32
  }
  #[inline] pub fn rtcpd_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x58) as *mut u32
  }
  #[inline] pub fn rtcpd(&self) -> Rtcpd { 
     unsafe {
        Rtcpd(::core::ptr::read_volatile(((self.0 as usize) + 0x58) as *const u32))
     }
  }
  #[inline] pub fn set_rtcpd(&self, value: Rtcpd) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x58) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_rtcpd<F: FnOnce(Rtcpd) -> Rtcpd>(&self, f: F) -> &Self {
     let tmp = self.rtcpd();
     self.set_rtcpd(f(tmp))
  }

  #[inline] pub fn tps_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x5c + (index << 2)) as *const u32
  }
  #[inline] pub fn tps_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x5c + (index << 2)) as *mut u32
  }
  #[inline] pub fn tps(&self, index: usize) -> Tps { 
     assert!(index < 2);
     unsafe {
        Tps(::core::ptr::read_volatile(((self.0 as usize) + 0x5c + (index << 2)) as *const u32))
     }
  }
  #[inline] pub fn set_tps(&self, index: usize, value: Tps) -> &Self {
     assert!(index < 2);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x5c + (index << 2)) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_tps<F: FnOnce(Tps) -> Tps>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tps(index);
     self.set_tps(index, f(tmp))
  }

  #[inline] pub fn dmaev_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x6c) as *const u32
  }
  #[inline] pub fn dmaev_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x6c) as *mut u32
  }
  #[inline] pub fn dmaev(&self) -> Dmaev { 
     unsafe {
        Dmaev(::core::ptr::read_volatile(((self.0 as usize) + 0x6c) as *const u32))
     }
  }
  #[inline] pub fn set_dmaev(&self, value: Dmaev) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x6c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dmaev<F: FnOnce(Dmaev) -> Dmaev>(&self, f: F) -> &Self {
     let tmp = self.dmaev();
     self.set_dmaev(f(tmp))
  }

  #[inline] pub fn adcev_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x70) as *const u32
  }
  #[inline] pub fn adcev_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x70) as *mut u32
  }
  #[inline] pub fn adcev(&self) -> Adcev { 
     unsafe {
        Adcev(::core::ptr::read_volatile(((self.0 as usize) + 0x70) as *const u32))
     }
  }
  #[inline] pub fn set_adcev(&self, value: Adcev) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x70) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_adcev<F: FnOnce(Adcev) -> Adcev>(&self, f: F) -> &Self {
     let tmp = self.adcev();
     self.set_adcev(f(tmp))
  }

  #[inline] pub fn pp_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xfc0) as *const u32
  }
  #[inline] pub fn pp_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xfc0) as *mut u32
  }
  #[inline] pub fn pp(&self) -> Pp { 
     unsafe {
        Pp(::core::ptr::read_volatile(((self.0 as usize) + 0xfc0) as *const u32))
     }
  }
  #[inline] pub fn set_pp(&self, value: Pp) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xfc0) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pp<F: FnOnce(Pp) -> Pp>(&self, f: F) -> &Self {
     let tmp = self.pp();
     self.set_pp(f(tmp))
  }

  #[inline] pub fn cc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xfc8) as *const u32
  }
  #[inline] pub fn cc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xfc8) as *mut u32
  }
  #[inline] pub fn cc(&self) -> Cc { 
     unsafe {
        Cc(::core::ptr::read_volatile(((self.0 as usize) + 0xfc8) as *const u32))
     }
  }
  #[inline] pub fn set_cc(&self, value: Cc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xfc8) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_cc<F: FnOnce(Cc) -> Cc>(&self, f: F) -> &Self {
     let tmp = self.cc();
     self.set_cc(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Cfg(pub u32);
impl Cfg {
  #[inline] pub fn cfg(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
  #[inline] pub fn set_cfg(mut self, value: u32) -> Self {
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
pub struct Tmr(pub u32);
impl Tmr {
  #[inline] pub fn tmr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline] pub fn set_tmr(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn tcmr(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_tcmr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn tams(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_tams(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn tcdir(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_tcdir(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn tmie(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_tmie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn twot(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_twot(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn tsnaps(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_tsnaps(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn tild(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_tild(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn tpwmie(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_tpwmie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn tmrsu(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_tmrsu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn tplo(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_tplo(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline] pub fn tcintd(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline] pub fn set_tcintd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn tcact(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x7 // [15:13]
  }
  #[inline] pub fn set_tcact(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 13);
     self.0 |= value << 13;
     self
  }

}
impl ::core::fmt::Display for Tmr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tmr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tmr() != 0 { try!(write!(f, " tmr=0x{:x}", self.tmr()))}
      if self.tcmr() != 0 { try!(write!(f, " tcmr"))}
      if self.tams() != 0 { try!(write!(f, " tams"))}
      if self.tcdir() != 0 { try!(write!(f, " tcdir"))}
      if self.tmie() != 0 { try!(write!(f, " tmie"))}
      if self.twot() != 0 { try!(write!(f, " twot"))}
      if self.tsnaps() != 0 { try!(write!(f, " tsnaps"))}
      if self.tild() != 0 { try!(write!(f, " tild"))}
      if self.tpwmie() != 0 { try!(write!(f, " tpwmie"))}
      if self.tmrsu() != 0 { try!(write!(f, " tmrsu"))}
      if self.tplo() != 0 { try!(write!(f, " tplo"))}
      if self.tcintd() != 0 { try!(write!(f, " tcintd"))}
      if self.tcact() != 0 { try!(write!(f, " tcact=0x{:x}", self.tcact()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ctl(pub u32);
impl Ctl {
  #[inline] pub fn ten(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 0 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_ten(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn tstall(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 1 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [1]
  }
  #[inline] pub fn set_tstall(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 1 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn tevent(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 2 + (index << 3);
     ((self.0 as u32) >> shift) & 0x3 // [3:2]
  }
  #[inline] pub fn set_tevent(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x3) == 0);
     let shift: usize = 2 + (index << 3);
     self.0 &= !(0x3 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn rtcen(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_rtcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn tote(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 5 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [5]
  }
  #[inline] pub fn set_tote(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 5 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn tpwml(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 6 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [6]
  }
  #[inline] pub fn set_tpwml(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 6 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
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
      if self.ten(0) != 0 { try!(write!(f, " ten[0]"))}
      if self.ten(1) != 0 { try!(write!(f, " ten[1]"))}
      if self.tstall(0) != 0 { try!(write!(f, " tstall[0]"))}
      if self.tstall(1) != 0 { try!(write!(f, " tstall[1]"))}
      if self.tevent(0) != 0 { try!(write!(f, " tevent[0]=0x{:x}", self.tevent(0)))}
      if self.tevent(1) != 0 { try!(write!(f, " tevent[1]=0x{:x}", self.tevent(1)))}
      if self.rtcen() != 0 { try!(write!(f, " rtcen"))}
      if self.tote(0) != 0 { try!(write!(f, " tote[0]"))}
      if self.tote(1) != 0 { try!(write!(f, " tote[1]"))}
      if self.tpwml(0) != 0 { try!(write!(f, " tpwml[0]"))}
      if self.tpwml(1) != 0 { try!(write!(f, " tpwml[1]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Sync(pub u32);
impl Sync {
  #[inline] pub fn synct(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + (index << 1);
     ((self.0 as u32) >> shift) & 0x3 // [1:0]
  }
  #[inline] pub fn set_synct(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x3) == 0);
     let shift: usize = 0 + (index << 1);
     self.0 &= !(0x3 << shift);
     self.0 |= value << shift;
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
      if self.synct(0) != 0 { try!(write!(f, " synct[0]=0x{:x}", self.synct(0)))}
      if self.synct(1) != 0 { try!(write!(f, " synct[1]=0x{:x}", self.synct(1)))}
      if self.synct(2) != 0 { try!(write!(f, " synct[2]=0x{:x}", self.synct(2)))}
      if self.synct(3) != 0 { try!(write!(f, " synct[3]=0x{:x}", self.synct(3)))}
      if self.synct(4) != 0 { try!(write!(f, " synct[4]=0x{:x}", self.synct(4)))}
      if self.synct(5) != 0 { try!(write!(f, " synct[5]=0x{:x}", self.synct(5)))}
      if self.synct(6) != 0 { try!(write!(f, " synct[6]=0x{:x}", self.synct(6)))}
      if self.synct(7) != 0 { try!(write!(f, " synct[7]=0x{:x}", self.synct(7)))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Imr(pub u32);
impl Imr {
  #[inline] pub fn ttoim(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 0 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_ttoim(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn cmim(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 1 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [1]
  }
  #[inline] pub fn set_cmim(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 1 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn ceim(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 2 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [2]
  }
  #[inline] pub fn set_ceim(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 2 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn rtcim(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_rtcim(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn tmim(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 4 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [4]
  }
  #[inline] pub fn set_tmim(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 4 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn dmaim(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 5 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [5]
  }
  #[inline] pub fn set_dmaim(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 5 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
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
      if self.ttoim(0) != 0 { try!(write!(f, " ttoim[0]"))}
      if self.ttoim(1) != 0 { try!(write!(f, " ttoim[1]"))}
      if self.cmim(0) != 0 { try!(write!(f, " cmim[0]"))}
      if self.cmim(1) != 0 { try!(write!(f, " cmim[1]"))}
      if self.ceim(0) != 0 { try!(write!(f, " ceim[0]"))}
      if self.ceim(1) != 0 { try!(write!(f, " ceim[1]"))}
      if self.rtcim() != 0 { try!(write!(f, " rtcim"))}
      if self.tmim(0) != 0 { try!(write!(f, " tmim[0]"))}
      if self.tmim(1) != 0 { try!(write!(f, " tmim[1]"))}
      if self.dmaim(0) != 0 { try!(write!(f, " dmaim[0]"))}
      if self.dmaim(1) != 0 { try!(write!(f, " dmaim[1]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ris(pub u32);
impl Ris {
  #[inline] pub fn ttoris(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 0 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_ttoris(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn cmris(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 1 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [1]
  }
  #[inline] pub fn set_cmris(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 1 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn ceris(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 2 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [2]
  }
  #[inline] pub fn set_ceris(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 2 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn rtcris(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_rtcris(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn tmris(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 4 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [4]
  }
  #[inline] pub fn set_tmris(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 4 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn dmaris(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 5 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [5]
  }
  #[inline] pub fn set_dmaris(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 5 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
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
      if self.ttoris(0) != 0 { try!(write!(f, " ttoris[0]"))}
      if self.ttoris(1) != 0 { try!(write!(f, " ttoris[1]"))}
      if self.cmris(0) != 0 { try!(write!(f, " cmris[0]"))}
      if self.cmris(1) != 0 { try!(write!(f, " cmris[1]"))}
      if self.ceris(0) != 0 { try!(write!(f, " ceris[0]"))}
      if self.ceris(1) != 0 { try!(write!(f, " ceris[1]"))}
      if self.rtcris() != 0 { try!(write!(f, " rtcris"))}
      if self.tmris(0) != 0 { try!(write!(f, " tmris[0]"))}
      if self.tmris(1) != 0 { try!(write!(f, " tmris[1]"))}
      if self.dmaris(0) != 0 { try!(write!(f, " dmaris[0]"))}
      if self.dmaris(1) != 0 { try!(write!(f, " dmaris[1]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Mis(pub u32);
impl Mis {
  #[inline] pub fn ttomis(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 0 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_ttomis(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn cmmis(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 1 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [1]
  }
  #[inline] pub fn set_cmmis(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 1 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn cemis(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 2 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [2]
  }
  #[inline] pub fn set_cemis(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 2 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn rtcmis(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_rtcmis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn tmmis(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 4 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [4]
  }
  #[inline] pub fn set_tmmis(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 4 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn dmamis(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 5 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [5]
  }
  #[inline] pub fn set_dmamis(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 5 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
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
      if self.ttomis(0) != 0 { try!(write!(f, " ttomis[0]"))}
      if self.ttomis(1) != 0 { try!(write!(f, " ttomis[1]"))}
      if self.cmmis(0) != 0 { try!(write!(f, " cmmis[0]"))}
      if self.cmmis(1) != 0 { try!(write!(f, " cmmis[1]"))}
      if self.cemis(0) != 0 { try!(write!(f, " cemis[0]"))}
      if self.cemis(1) != 0 { try!(write!(f, " cemis[1]"))}
      if self.rtcmis() != 0 { try!(write!(f, " rtcmis"))}
      if self.tmmis(0) != 0 { try!(write!(f, " tmmis[0]"))}
      if self.tmmis(1) != 0 { try!(write!(f, " tmmis[1]"))}
      if self.dmamis(0) != 0 { try!(write!(f, " dmamis[0]"))}
      if self.dmamis(1) != 0 { try!(write!(f, " dmamis[1]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
  #[inline] pub fn ttocint(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 0 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_ttocint(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn cmcint(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 1 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [1]
  }
  #[inline] pub fn set_cmcint(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 1 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn cecint(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 2 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [2]
  }
  #[inline] pub fn set_cecint(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 2 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn rtccint(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_rtccint(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn tmcint(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 4 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [4]
  }
  #[inline] pub fn set_tmcint(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 4 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn dmaint(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 5 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [5]
  }
  #[inline] pub fn set_dmaint(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 5 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
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
      if self.ttocint(0) != 0 { try!(write!(f, " ttocint[0]"))}
      if self.ttocint(1) != 0 { try!(write!(f, " ttocint[1]"))}
      if self.cmcint(0) != 0 { try!(write!(f, " cmcint[0]"))}
      if self.cmcint(1) != 0 { try!(write!(f, " cmcint[1]"))}
      if self.cecint(0) != 0 { try!(write!(f, " cecint[0]"))}
      if self.cecint(1) != 0 { try!(write!(f, " cecint[1]"))}
      if self.rtccint() != 0 { try!(write!(f, " rtccint"))}
      if self.tmcint(0) != 0 { try!(write!(f, " tmcint[0]"))}
      if self.tmcint(1) != 0 { try!(write!(f, " tmcint[1]"))}
      if self.dmaint(0) != 0 { try!(write!(f, " dmaint[0]"))}
      if self.dmaint(1) != 0 { try!(write!(f, " dmaint[1]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Tilr(pub u32);
impl Tilr {
  #[inline] pub fn tilr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_tilr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tilr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tilr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Tmtchr(pub u32);
impl Tmtchr {
  #[inline] pub fn tmatchr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_tmatchr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tmtchr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tmtchr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Tpr(pub u32);
impl Tpr {
  #[inline] pub fn tpsr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  #[inline] pub fn set_tpsr(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tpsr() != 0 { try!(write!(f, " tpsr=0x{:x}", self.tpsr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Tpmr(pub u32);
impl Tpmr {
  #[inline] pub fn tpsmr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  #[inline] pub fn set_tpsmr(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tpmr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tpmr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tpsmr() != 0 { try!(write!(f, " tpsmr=0x{:x}", self.tpsmr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Tr(pub u32);
impl Tr {
  #[inline] pub fn tr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_tr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Tv(pub u32);
impl Tv {
  #[inline] pub fn tv(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_tv(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tv {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tv {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Rtcpd(pub u32);
impl Rtcpd {
  #[inline] pub fn rtcpd(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_rtcpd(mut self, value: u32) -> Self {
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
pub struct Tps(pub u32);
impl Tps {
  #[inline] pub fn pss(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_pss(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tps {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tps {
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
  #[inline] pub fn ttodmaen(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 0 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_ttodmaen(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn cmdmaen(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 1 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [1]
  }
  #[inline] pub fn set_cmdmaen(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 1 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn cedmaen(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 2 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [2]
  }
  #[inline] pub fn set_cedmaen(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 2 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn rtcdmaen(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_rtcdmaen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn tmdmaen(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 4 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [4]
  }
  #[inline] pub fn set_tmdmaen(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 4 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
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
      if self.ttodmaen(0) != 0 { try!(write!(f, " ttodmaen[0]"))}
      if self.ttodmaen(1) != 0 { try!(write!(f, " ttodmaen[1]"))}
      if self.cmdmaen(0) != 0 { try!(write!(f, " cmdmaen[0]"))}
      if self.cmdmaen(1) != 0 { try!(write!(f, " cmdmaen[1]"))}
      if self.cedmaen(0) != 0 { try!(write!(f, " cedmaen[0]"))}
      if self.cedmaen(1) != 0 { try!(write!(f, " cedmaen[1]"))}
      if self.rtcdmaen() != 0 { try!(write!(f, " rtcdmaen"))}
      if self.tmdmaen(0) != 0 { try!(write!(f, " tmdmaen[0]"))}
      if self.tmdmaen(1) != 0 { try!(write!(f, " tmdmaen[1]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Adcev(pub u32);
impl Adcev {
  #[inline] pub fn ttoadcen(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 0 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_ttoadcen(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn cmadcen(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 1 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [1]
  }
  #[inline] pub fn set_cmadcen(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 1 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn ceadcen(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 2 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [2]
  }
  #[inline] pub fn set_ceadcen(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 2 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn rtcadcen(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_rtcadcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn tmadcen(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 4 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [4]
  }
  #[inline] pub fn set_tmadcen(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 4 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
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
      if self.ttoadcen(0) != 0 { try!(write!(f, " ttoadcen[0]"))}
      if self.ttoadcen(1) != 0 { try!(write!(f, " ttoadcen[1]"))}
      if self.cmadcen(0) != 0 { try!(write!(f, " cmadcen[0]"))}
      if self.cmadcen(1) != 0 { try!(write!(f, " cmadcen[1]"))}
      if self.ceadcen(0) != 0 { try!(write!(f, " ceadcen[0]"))}
      if self.ceadcen(1) != 0 { try!(write!(f, " ceadcen[1]"))}
      if self.rtcadcen() != 0 { try!(write!(f, " rtcadcen"))}
      if self.tmadcen(0) != 0 { try!(write!(f, " tmadcen[0]"))}
      if self.tmadcen(1) != 0 { try!(write!(f, " tmadcen[1]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Pp(pub u32);
impl Pp {
  #[inline] pub fn size(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_size(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn chain(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_chain(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn synccnt(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_synccnt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn altclk(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_altclk(mut self, value: u32) -> Self {
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
  #[inline] pub fn altclk(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_altclk(mut self, value: u32) -> Self {
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
#[derive(Clone, Copy, PartialEq)]
pub struct Channel<P, T> { pub periph: Periph<T>, pub index: usize, pub id: P }

impl<P,T> Channel<P,T> {
   #[inline] pub fn periph(&self) -> &Periph<T> { &self.periph }
   #[inline] pub fn index(&self) -> usize { self.index }
}

pub const TIMER0A: Channel<Timer0aId, Timer0Id> = Channel { periph: TIMER0, index: 0, id: Timer0aId {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Timer0aId {}
pub type Timer0a = Channel<Timer0aId, Timer0Id>;

pub const TIMER0B: Channel<Timer0bId, Timer0Id> = Channel { periph: TIMER0, index: 1, id: Timer0bId {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Timer0bId {}
pub type Timer0b = Channel<Timer0bId, Timer0Id>;

pub const TIMER1A: Channel<Timer1aId, Timer1Id> = Channel { periph: TIMER1, index: 0, id: Timer1aId {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Timer1aId {}
pub type Timer1a = Channel<Timer1aId, Timer1Id>;

pub const TIMER1B: Channel<Timer1bId, Timer1Id> = Channel { periph: TIMER1, index: 1, id: Timer1bId {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Timer1bId {}
pub type Timer1b = Channel<Timer1bId, Timer1Id>;

pub const TIMER2A: Channel<Timer2aId, Timer2Id> = Channel { periph: TIMER2, index: 0, id: Timer2aId {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Timer2aId {}
pub type Timer2a = Channel<Timer2aId, Timer2Id>;

pub const TIMER2B: Channel<Timer2bId, Timer2Id> = Channel { periph: TIMER2, index: 1, id: Timer2bId {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Timer2bId {}
pub type Timer2b = Channel<Timer2bId, Timer2Id>;

pub const TIMER3A: Channel<Timer3aId, Timer3Id> = Channel { periph: TIMER3, index: 0, id: Timer3aId {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Timer3aId {}
pub type Timer3a = Channel<Timer3aId, Timer3Id>;

pub const TIMER3B: Channel<Timer3bId, Timer3Id> = Channel { periph: TIMER3, index: 1, id: Timer3bId {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Timer3bId {}
pub type Timer3b = Channel<Timer3bId, Timer3Id>;

pub const TIMER4A: Channel<Timer4aId, Timer4Id> = Channel { periph: TIMER4, index: 0, id: Timer4aId {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Timer4aId {}
pub type Timer4a = Channel<Timer4aId, Timer4Id>;

pub const TIMER4B: Channel<Timer4bId, Timer4Id> = Channel { periph: TIMER4, index: 1, id: Timer4bId {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Timer4bId {}
pub type Timer4b = Channel<Timer4bId, Timer4Id>;

pub const TIMER5A: Channel<Timer5aId, Timer5Id> = Channel { periph: TIMER5, index: 0, id: Timer5aId {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Timer5aId {}
pub type Timer5a = Channel<Timer5aId, Timer5Id>;

pub const TIMER5B: Channel<Timer5bId, Timer5Id> = Channel { periph: TIMER5, index: 1, id: Timer5bId {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Timer5bId {}
pub type Timer5b = Channel<Timer5bId, Timer5Id>;

pub const TIMER6A: Channel<Timer6aId, Timer6Id> = Channel { periph: TIMER6, index: 0, id: Timer6aId {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Timer6aId {}
pub type Timer6a = Channel<Timer6aId, Timer6Id>;

pub const TIMER6B: Channel<Timer6bId, Timer6Id> = Channel { periph: TIMER6, index: 1, id: Timer6bId {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Timer6bId {}
pub type Timer6b = Channel<Timer6bId, Timer6Id>;

pub const TIMER7A: Channel<Timer7aId, Timer7Id> = Channel { periph: TIMER7, index: 0, id: Timer7aId {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Timer7aId {}
pub type Timer7a = Channel<Timer7aId, Timer7Id>;

pub const TIMER7B: Channel<Timer7bId, Timer7Id> = Channel { periph: TIMER7, index: 1, id: Timer7bId {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Timer7bId {}
pub type Timer7b = Channel<Timer7bId, Timer7Id>;

pub trait IrqTimer<T> {
   fn irq_timer(&self) -> super::irq::Irq<T>;
}

pub trait RegisterTimerHandler {
   fn register_timer_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTimer>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleTimer {
   fn handle_timer(&self);
}

impl IrqTimer<super::irq::Timer0aId> for Timer0a {
   fn irq_timer(&self) -> super::irq::IrqTimer0a { super::irq::IRQ_TIMER0A }
}

impl RegisterTimerHandler for Timer0a {
   fn register_timer_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTimer>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTimer>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_timer() }
       }
       super::irq::set_handler(19, Some(wrapper::<F>));
       super::irq::IrqGuard::new(19)
   }
}

impl IrqTimer<super::irq::Timer0bId> for Timer0b {
   fn irq_timer(&self) -> super::irq::IrqTimer0b { super::irq::IRQ_TIMER0B }
}

impl RegisterTimerHandler for Timer0b {
   fn register_timer_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTimer>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTimer>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_timer() }
       }
       super::irq::set_handler(20, Some(wrapper::<F>));
       super::irq::IrqGuard::new(20)
   }
}

impl IrqTimer<super::irq::Timer1aId> for Timer1a {
   fn irq_timer(&self) -> super::irq::IrqTimer1a { super::irq::IRQ_TIMER1A }
}

impl RegisterTimerHandler for Timer1a {
   fn register_timer_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTimer>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTimer>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_timer() }
       }
       super::irq::set_handler(21, Some(wrapper::<F>));
       super::irq::IrqGuard::new(21)
   }
}

impl IrqTimer<super::irq::Timer1bId> for Timer1b {
   fn irq_timer(&self) -> super::irq::IrqTimer1b { super::irq::IRQ_TIMER1B }
}

impl RegisterTimerHandler for Timer1b {
   fn register_timer_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTimer>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTimer>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_timer() }
       }
       super::irq::set_handler(22, Some(wrapper::<F>));
       super::irq::IrqGuard::new(22)
   }
}

impl IrqTimer<super::irq::Timer2aId> for Timer2a {
   fn irq_timer(&self) -> super::irq::IrqTimer2a { super::irq::IRQ_TIMER2A }
}

impl RegisterTimerHandler for Timer2a {
   fn register_timer_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTimer>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTimer>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_timer() }
       }
       super::irq::set_handler(23, Some(wrapper::<F>));
       super::irq::IrqGuard::new(23)
   }
}

impl IrqTimer<super::irq::Timer2bId> for Timer2b {
   fn irq_timer(&self) -> super::irq::IrqTimer2b { super::irq::IRQ_TIMER2B }
}

impl RegisterTimerHandler for Timer2b {
   fn register_timer_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTimer>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTimer>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_timer() }
       }
       super::irq::set_handler(24, Some(wrapper::<F>));
       super::irq::IrqGuard::new(24)
   }
}

impl IrqTimer<super::irq::Timer3aId> for Timer3a {
   fn irq_timer(&self) -> super::irq::IrqTimer3a { super::irq::IRQ_TIMER3A }
}

impl RegisterTimerHandler for Timer3a {
   fn register_timer_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTimer>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTimer>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_timer() }
       }
       super::irq::set_handler(35, Some(wrapper::<F>));
       super::irq::IrqGuard::new(35)
   }
}

impl IrqTimer<super::irq::Timer3bId> for Timer3b {
   fn irq_timer(&self) -> super::irq::IrqTimer3b { super::irq::IRQ_TIMER3B }
}

impl RegisterTimerHandler for Timer3b {
   fn register_timer_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTimer>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTimer>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_timer() }
       }
       super::irq::set_handler(36, Some(wrapper::<F>));
       super::irq::IrqGuard::new(36)
   }
}

impl IrqTimer<super::irq::Timer4aId> for Timer4a {
   fn irq_timer(&self) -> super::irq::IrqTimer4a { super::irq::IRQ_TIMER4A }
}

impl RegisterTimerHandler for Timer4a {
   fn register_timer_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTimer>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTimer>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_timer() }
       }
       super::irq::set_handler(63, Some(wrapper::<F>));
       super::irq::IrqGuard::new(63)
   }
}

impl IrqTimer<super::irq::Timer4bId> for Timer4b {
   fn irq_timer(&self) -> super::irq::IrqTimer4b { super::irq::IRQ_TIMER4B }
}

impl RegisterTimerHandler for Timer4b {
   fn register_timer_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTimer>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTimer>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_timer() }
       }
       super::irq::set_handler(64, Some(wrapper::<F>));
       super::irq::IrqGuard::new(64)
   }
}

impl IrqTimer<super::irq::Timer5aId> for Timer5a {
   fn irq_timer(&self) -> super::irq::IrqTimer5a { super::irq::IRQ_TIMER5A }
}

impl RegisterTimerHandler for Timer5a {
   fn register_timer_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTimer>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTimer>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_timer() }
       }
       super::irq::set_handler(65, Some(wrapper::<F>));
       super::irq::IrqGuard::new(65)
   }
}

impl IrqTimer<super::irq::Timer5bId> for Timer5b {
   fn irq_timer(&self) -> super::irq::IrqTimer5b { super::irq::IRQ_TIMER5B }
}

impl RegisterTimerHandler for Timer5b {
   fn register_timer_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTimer>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTimer>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_timer() }
       }
       super::irq::set_handler(66, Some(wrapper::<F>));
       super::irq::IrqGuard::new(66)
   }
}

impl IrqTimer<super::irq::Timer6aId> for Timer6a {
   fn irq_timer(&self) -> super::irq::IrqTimer6a { super::irq::IRQ_TIMER6A }
}

impl RegisterTimerHandler for Timer6a {
   fn register_timer_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTimer>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTimer>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_timer() }
       }
       super::irq::set_handler(98, Some(wrapper::<F>));
       super::irq::IrqGuard::new(98)
   }
}

impl IrqTimer<super::irq::Timer6bId> for Timer6b {
   fn irq_timer(&self) -> super::irq::IrqTimer6b { super::irq::IRQ_TIMER6B }
}

impl RegisterTimerHandler for Timer6b {
   fn register_timer_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTimer>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTimer>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_timer() }
       }
       super::irq::set_handler(99, Some(wrapper::<F>));
       super::irq::IrqGuard::new(99)
   }
}

impl IrqTimer<super::irq::Timer7aId> for Timer7a {
   fn irq_timer(&self) -> super::irq::IrqTimer7a { super::irq::IRQ_TIMER7A }
}

impl RegisterTimerHandler for Timer7a {
   fn register_timer_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTimer>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTimer>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_timer() }
       }
       super::irq::set_handler(100, Some(wrapper::<F>));
       super::irq::IrqGuard::new(100)
   }
}

impl IrqTimer<super::irq::Timer7bId> for Timer7b {
   fn irq_timer(&self) -> super::irq::IrqTimer7b { super::irq::IRQ_TIMER7B }
}

impl RegisterTimerHandler for Timer7b {
   fn register_timer_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTimer>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTimer>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_timer() }
       }
       super::irq::set_handler(101, Some(wrapper::<F>));
       super::irq::IrqGuard::new(101)
   }
}

