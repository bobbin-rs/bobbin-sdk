
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Periph<T>(pub u32, pub T); 



impl<T> Periph<T> {
  #[inline] pub fn sc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
  #[inline] pub fn sc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
  #[inline] pub fn sc(&self) -> Sc { 
     unsafe {
        Sc(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  #[inline] pub fn set_sc(&self, value: Sc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_sc<F: FnOnce(Sc) -> Sc>(&self, f: F) -> &Self {
     let tmp = self.sc();
     self.set_sc(f(tmp))
  }

  #[inline] pub fn cnt_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
  #[inline] pub fn cnt_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
  #[inline] pub fn cnt(&self) -> Cnt { 
     unsafe {
        Cnt(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
  #[inline] pub fn set_cnt(&self, value: Cnt) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_cnt<F: FnOnce(Cnt) -> Cnt>(&self, f: F) -> &Self {
     let tmp = self.cnt();
     self.set_cnt(f(tmp))
  }

  #[inline] pub fn mod_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
  #[inline] pub fn mod_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
  #[inline] pub fn _mod(&self) -> Mod { 
     unsafe {
        Mod(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
  #[inline] pub fn set_mod(&self, value: Mod) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_mod<F: FnOnce(Mod) -> Mod>(&self, f: F) -> &Self {
     let tmp = self._mod();
     self.set_mod(f(tmp))
  }

  #[inline] pub fn csc_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 8);
     ((self.0 as usize) + 0xc + (index << 3)) as *const u32
  }
  #[inline] pub fn csc_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 8);
     ((self.0 as usize) + 0xc + (index << 3)) as *mut u32
  }
  #[inline] pub fn csc(&self, index: usize) -> Csc { 
     assert!(index < 8);
     unsafe {
        Csc(::core::ptr::read_volatile(((self.0 as usize) + 0xc + (index << 3)) as *const u32))
     }
  }
  #[inline] pub fn set_csc(&self, index: usize, value: Csc) -> &Self {
     assert!(index < 8);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc + (index << 3)) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_csc<F: FnOnce(Csc) -> Csc>(&self, index: usize, f: F) -> &Self {
     let tmp = self.csc(index);
     self.set_csc(index, f(tmp))
  }

  #[inline] pub fn cv_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 8);
     ((self.0 as usize) + 0x10 + (index << 3)) as *const u32
  }
  #[inline] pub fn cv_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 8);
     ((self.0 as usize) + 0x10 + (index << 3)) as *mut u32
  }
  #[inline] pub fn cv(&self, index: usize) -> Cv { 
     assert!(index < 8);
     unsafe {
        Cv(::core::ptr::read_volatile(((self.0 as usize) + 0x10 + (index << 3)) as *const u32))
     }
  }
  #[inline] pub fn set_cv(&self, index: usize, value: Cv) -> &Self {
     assert!(index < 8);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10 + (index << 3)) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_cv<F: FnOnce(Cv) -> Cv>(&self, index: usize, f: F) -> &Self {
     let tmp = self.cv(index);
     self.set_cv(index, f(tmp))
  }

  #[inline] pub fn cntin_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4c) as *const u32
  }
  #[inline] pub fn cntin_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4c) as *mut u32
  }
  #[inline] pub fn cntin(&self) -> Cntin { 
     unsafe {
        Cntin(::core::ptr::read_volatile(((self.0 as usize) + 0x4c) as *const u32))
     }
  }
  #[inline] pub fn set_cntin(&self, value: Cntin) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_cntin<F: FnOnce(Cntin) -> Cntin>(&self, f: F) -> &Self {
     let tmp = self.cntin();
     self.set_cntin(f(tmp))
  }

  #[inline] pub fn status_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x50) as *const u32
  }
  #[inline] pub fn status_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x50) as *mut u32
  }
  #[inline] pub fn status(&self) -> Status { 
     unsafe {
        Status(::core::ptr::read_volatile(((self.0 as usize) + 0x50) as *const u32))
     }
  }
  #[inline] pub fn set_status(&self, value: Status) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x50) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
     let tmp = self.status();
     self.set_status(f(tmp))
  }

  #[inline] pub fn mode_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x54) as *const u32
  }
  #[inline] pub fn mode_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x54) as *mut u32
  }
  #[inline] pub fn mode(&self) -> Mode { 
     unsafe {
        Mode(::core::ptr::read_volatile(((self.0 as usize) + 0x54) as *const u32))
     }
  }
  #[inline] pub fn set_mode(&self, value: Mode) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x54) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_mode<F: FnOnce(Mode) -> Mode>(&self, f: F) -> &Self {
     let tmp = self.mode();
     self.set_mode(f(tmp))
  }

  #[inline] pub fn sync_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x58) as *const u32
  }
  #[inline] pub fn sync_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x58) as *mut u32
  }
  #[inline] pub fn sync(&self) -> Sync { 
     unsafe {
        Sync(::core::ptr::read_volatile(((self.0 as usize) + 0x58) as *const u32))
     }
  }
  #[inline] pub fn set_sync(&self, value: Sync) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x58) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_sync<F: FnOnce(Sync) -> Sync>(&self, f: F) -> &Self {
     let tmp = self.sync();
     self.set_sync(f(tmp))
  }

  #[inline] pub fn outinit_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x5c) as *const u32
  }
  #[inline] pub fn outinit_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x5c) as *mut u32
  }
  #[inline] pub fn outinit(&self) -> Outinit { 
     unsafe {
        Outinit(::core::ptr::read_volatile(((self.0 as usize) + 0x5c) as *const u32))
     }
  }
  #[inline] pub fn set_outinit(&self, value: Outinit) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x5c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_outinit<F: FnOnce(Outinit) -> Outinit>(&self, f: F) -> &Self {
     let tmp = self.outinit();
     self.set_outinit(f(tmp))
  }

  #[inline] pub fn outmask_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x60) as *const u32
  }
  #[inline] pub fn outmask_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x60) as *mut u32
  }
  #[inline] pub fn outmask(&self) -> Outmask { 
     unsafe {
        Outmask(::core::ptr::read_volatile(((self.0 as usize) + 0x60) as *const u32))
     }
  }
  #[inline] pub fn set_outmask(&self, value: Outmask) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x60) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_outmask<F: FnOnce(Outmask) -> Outmask>(&self, f: F) -> &Self {
     let tmp = self.outmask();
     self.set_outmask(f(tmp))
  }

  #[inline] pub fn combine_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x64) as *const u32
  }
  #[inline] pub fn combine_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x64) as *mut u32
  }
  #[inline] pub fn combine(&self) -> Combine { 
     unsafe {
        Combine(::core::ptr::read_volatile(((self.0 as usize) + 0x64) as *const u32))
     }
  }
  #[inline] pub fn set_combine(&self, value: Combine) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x64) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_combine<F: FnOnce(Combine) -> Combine>(&self, f: F) -> &Self {
     let tmp = self.combine();
     self.set_combine(f(tmp))
  }

  #[inline] pub fn deadtime_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x68) as *const u32
  }
  #[inline] pub fn deadtime_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x68) as *mut u32
  }
  #[inline] pub fn deadtime(&self) -> Deadtime { 
     unsafe {
        Deadtime(::core::ptr::read_volatile(((self.0 as usize) + 0x68) as *const u32))
     }
  }
  #[inline] pub fn set_deadtime(&self, value: Deadtime) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x68) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_deadtime<F: FnOnce(Deadtime) -> Deadtime>(&self, f: F) -> &Self {
     let tmp = self.deadtime();
     self.set_deadtime(f(tmp))
  }

  #[inline] pub fn exttrig_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x6c) as *const u32
  }
  #[inline] pub fn exttrig_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x6c) as *mut u32
  }
  #[inline] pub fn exttrig(&self) -> Exttrig { 
     unsafe {
        Exttrig(::core::ptr::read_volatile(((self.0 as usize) + 0x6c) as *const u32))
     }
  }
  #[inline] pub fn set_exttrig(&self, value: Exttrig) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x6c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_exttrig<F: FnOnce(Exttrig) -> Exttrig>(&self, f: F) -> &Self {
     let tmp = self.exttrig();
     self.set_exttrig(f(tmp))
  }

  #[inline] pub fn pol_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x70) as *const u32
  }
  #[inline] pub fn pol_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x70) as *mut u32
  }
  #[inline] pub fn pol(&self) -> Pol { 
     unsafe {
        Pol(::core::ptr::read_volatile(((self.0 as usize) + 0x70) as *const u32))
     }
  }
  #[inline] pub fn set_pol(&self, value: Pol) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x70) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pol<F: FnOnce(Pol) -> Pol>(&self, f: F) -> &Self {
     let tmp = self.pol();
     self.set_pol(f(tmp))
  }

  #[inline] pub fn fms_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x74) as *const u32
  }
  #[inline] pub fn fms_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x74) as *mut u32
  }
  #[inline] pub fn fms(&self) -> Fms { 
     unsafe {
        Fms(::core::ptr::read_volatile(((self.0 as usize) + 0x74) as *const u32))
     }
  }
  #[inline] pub fn set_fms(&self, value: Fms) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x74) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_fms<F: FnOnce(Fms) -> Fms>(&self, f: F) -> &Self {
     let tmp = self.fms();
     self.set_fms(f(tmp))
  }

  #[inline] pub fn filter_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x78) as *const u32
  }
  #[inline] pub fn filter_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x78) as *mut u32
  }
  #[inline] pub fn filter(&self) -> Filter { 
     unsafe {
        Filter(::core::ptr::read_volatile(((self.0 as usize) + 0x78) as *const u32))
     }
  }
  #[inline] pub fn set_filter(&self, value: Filter) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x78) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_filter<F: FnOnce(Filter) -> Filter>(&self, f: F) -> &Self {
     let tmp = self.filter();
     self.set_filter(f(tmp))
  }

  #[inline] pub fn fltctrl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x7c) as *const u32
  }
  #[inline] pub fn fltctrl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x7c) as *mut u32
  }
  #[inline] pub fn fltctrl(&self) -> Fltctrl { 
     unsafe {
        Fltctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x7c) as *const u32))
     }
  }
  #[inline] pub fn set_fltctrl(&self, value: Fltctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x7c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_fltctrl<F: FnOnce(Fltctrl) -> Fltctrl>(&self, f: F) -> &Self {
     let tmp = self.fltctrl();
     self.set_fltctrl(f(tmp))
  }

  #[inline] pub fn qdctrl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x80) as *const u32
  }
  #[inline] pub fn qdctrl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x80) as *mut u32
  }
  #[inline] pub fn qdctrl(&self) -> Qdctrl { 
     unsafe {
        Qdctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x80) as *const u32))
     }
  }
  #[inline] pub fn set_qdctrl(&self, value: Qdctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x80) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_qdctrl<F: FnOnce(Qdctrl) -> Qdctrl>(&self, f: F) -> &Self {
     let tmp = self.qdctrl();
     self.set_qdctrl(f(tmp))
  }

  #[inline] pub fn conf_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x84) as *const u32
  }
  #[inline] pub fn conf_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x84) as *mut u32
  }
  #[inline] pub fn conf(&self) -> Conf { 
     unsafe {
        Conf(::core::ptr::read_volatile(((self.0 as usize) + 0x84) as *const u32))
     }
  }
  #[inline] pub fn set_conf(&self, value: Conf) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x84) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_conf<F: FnOnce(Conf) -> Conf>(&self, f: F) -> &Self {
     let tmp = self.conf();
     self.set_conf(f(tmp))
  }

  #[inline] pub fn fltpol_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x88) as *const u32
  }
  #[inline] pub fn fltpol_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x88) as *mut u32
  }
  #[inline] pub fn fltpol(&self) -> Fltpol { 
     unsafe {
        Fltpol(::core::ptr::read_volatile(((self.0 as usize) + 0x88) as *const u32))
     }
  }
  #[inline] pub fn set_fltpol(&self, value: Fltpol) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x88) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_fltpol<F: FnOnce(Fltpol) -> Fltpol>(&self, f: F) -> &Self {
     let tmp = self.fltpol();
     self.set_fltpol(f(tmp))
  }

  #[inline] pub fn synconf_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8c) as *const u32
  }
  #[inline] pub fn synconf_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8c) as *mut u32
  }
  #[inline] pub fn synconf(&self) -> Synconf { 
     unsafe {
        Synconf(::core::ptr::read_volatile(((self.0 as usize) + 0x8c) as *const u32))
     }
  }
  #[inline] pub fn set_synconf(&self, value: Synconf) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_synconf<F: FnOnce(Synconf) -> Synconf>(&self, f: F) -> &Self {
     let tmp = self.synconf();
     self.set_synconf(f(tmp))
  }

  #[inline] pub fn invctrl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x90) as *const u32
  }
  #[inline] pub fn invctrl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x90) as *mut u32
  }
  #[inline] pub fn invctrl(&self) -> Invctrl { 
     unsafe {
        Invctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x90) as *const u32))
     }
  }
  #[inline] pub fn set_invctrl(&self, value: Invctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x90) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_invctrl<F: FnOnce(Invctrl) -> Invctrl>(&self, f: F) -> &Self {
     let tmp = self.invctrl();
     self.set_invctrl(f(tmp))
  }

  #[inline] pub fn swoctrl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x94) as *const u32
  }
  #[inline] pub fn swoctrl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x94) as *mut u32
  }
  #[inline] pub fn swoctrl(&self) -> Swoctrl { 
     unsafe {
        Swoctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x94) as *const u32))
     }
  }
  #[inline] pub fn set_swoctrl(&self, value: Swoctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x94) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_swoctrl<F: FnOnce(Swoctrl) -> Swoctrl>(&self, f: F) -> &Self {
     let tmp = self.swoctrl();
     self.set_swoctrl(f(tmp))
  }

  #[inline] pub fn pwmload_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x98) as *const u32
  }
  #[inline] pub fn pwmload_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x98) as *mut u32
  }
  #[inline] pub fn pwmload(&self) -> Pwmload { 
     unsafe {
        Pwmload(::core::ptr::read_volatile(((self.0 as usize) + 0x98) as *const u32))
     }
  }
  #[inline] pub fn set_pwmload(&self, value: Pwmload) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x98) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pwmload<F: FnOnce(Pwmload) -> Pwmload>(&self, f: F) -> &Self {
     let tmp = self.pwmload();
     self.set_pwmload(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Sc(pub u32);
impl Sc {
  #[inline] pub fn ps(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
  #[inline] pub fn set_ps(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn clks(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x3 // [4:3]
  }
  #[inline] pub fn set_clks(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn cpwms(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_cpwms(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn toie(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_toie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn tof(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_tof(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Sc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ps() != 0 { try!(write!(f, " ps=0x{:x}", self.ps()))}
      if self.clks() != 0 { try!(write!(f, " clks=0x{:x}", self.clks()))}
      if self.cpwms() != 0 { try!(write!(f, " cpwms"))}
      if self.toie() != 0 { try!(write!(f, " toie"))}
      if self.tof() != 0 { try!(write!(f, " tof"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cnt(pub u32);
impl Cnt {
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cnt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cnt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Mod(pub u32);
impl Mod {
  #[inline] pub fn _mod(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_mod(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Mod {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Mod {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self._mod() != 0 { try!(write!(f, " mod=0x{:x}", self._mod()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Csc(pub u32);
impl Csc {
  #[inline] pub fn dma(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_dma(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn elsa(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_elsa(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn elsb(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_elsb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn msa(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_msa(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn msb(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_msb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn chie(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_chie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn chf(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_chf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Csc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Csc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dma() != 0 { try!(write!(f, " dma"))}
      if self.elsa() != 0 { try!(write!(f, " elsa"))}
      if self.elsb() != 0 { try!(write!(f, " elsb"))}
      if self.msa() != 0 { try!(write!(f, " msa"))}
      if self.msb() != 0 { try!(write!(f, " msb"))}
      if self.chie() != 0 { try!(write!(f, " chie"))}
      if self.chf() != 0 { try!(write!(f, " chf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cv(pub u32);
impl Cv {
  #[inline] pub fn val(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_val(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cv {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cv {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.val() != 0 { try!(write!(f, " val=0x{:x}", self.val()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cntin(pub u32);
impl Cntin {
  #[inline] pub fn init(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_init(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cntin {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cntin {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.init() != 0 { try!(write!(f, " init=0x{:x}", self.init()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Status(pub u32);
impl Status {
  #[inline] pub fn chf(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_chf(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
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
      if self.chf(0) != 0 { try!(write!(f, " chf[0]"))}
      if self.chf(1) != 0 { try!(write!(f, " chf[1]"))}
      if self.chf(2) != 0 { try!(write!(f, " chf[2]"))}
      if self.chf(3) != 0 { try!(write!(f, " chf[3]"))}
      if self.chf(4) != 0 { try!(write!(f, " chf[4]"))}
      if self.chf(5) != 0 { try!(write!(f, " chf[5]"))}
      if self.chf(6) != 0 { try!(write!(f, " chf[6]"))}
      if self.chf(7) != 0 { try!(write!(f, " chf[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Mode(pub u32);
impl Mode {
  #[inline] pub fn ftmen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_ftmen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn init(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_init(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn wpdis(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_wpdis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwmsync(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwmsync(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn captest(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_captest(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn faultm(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x3 // [6:5]
  }
  #[inline] pub fn set_faultm(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn faultie(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_faultie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Mode {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Mode {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ftmen() != 0 { try!(write!(f, " ftmen"))}
      if self.init() != 0 { try!(write!(f, " init"))}
      if self.wpdis() != 0 { try!(write!(f, " wpdis"))}
      if self.pwmsync() != 0 { try!(write!(f, " pwmsync"))}
      if self.captest() != 0 { try!(write!(f, " captest"))}
      if self.faultm() != 0 { try!(write!(f, " faultm=0x{:x}", self.faultm()))}
      if self.faultie() != 0 { try!(write!(f, " faultie"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Sync(pub u32);
impl Sync {
  #[inline] pub fn cntmin(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_cntmin(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn cntmax(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_cntmax(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn reinit(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_reinit(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn synchom(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_synchom(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn trig0(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_trig0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn trig1(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_trig1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn trig2(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_trig2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn swsync(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_swsync(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
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
      if self.cntmin() != 0 { try!(write!(f, " cntmin"))}
      if self.cntmax() != 0 { try!(write!(f, " cntmax"))}
      if self.reinit() != 0 { try!(write!(f, " reinit"))}
      if self.synchom() != 0 { try!(write!(f, " synchom"))}
      if self.trig0() != 0 { try!(write!(f, " trig0"))}
      if self.trig1() != 0 { try!(write!(f, " trig1"))}
      if self.trig2() != 0 { try!(write!(f, " trig2"))}
      if self.swsync() != 0 { try!(write!(f, " swsync"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Outinit(pub u32);
impl Outinit {
  #[inline] pub fn choi(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_choi(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Outinit {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Outinit {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.choi(0) != 0 { try!(write!(f, " choi[0]"))}
      if self.choi(1) != 0 { try!(write!(f, " choi[1]"))}
      if self.choi(2) != 0 { try!(write!(f, " choi[2]"))}
      if self.choi(3) != 0 { try!(write!(f, " choi[3]"))}
      if self.choi(4) != 0 { try!(write!(f, " choi[4]"))}
      if self.choi(5) != 0 { try!(write!(f, " choi[5]"))}
      if self.choi(6) != 0 { try!(write!(f, " choi[6]"))}
      if self.choi(7) != 0 { try!(write!(f, " choi[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Outmask(pub u32);
impl Outmask {
  #[inline] pub fn chom(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_chom(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Outmask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Outmask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.chom(0) != 0 { try!(write!(f, " chom[0]"))}
      if self.chom(1) != 0 { try!(write!(f, " chom[1]"))}
      if self.chom(2) != 0 { try!(write!(f, " chom[2]"))}
      if self.chom(3) != 0 { try!(write!(f, " chom[3]"))}
      if self.chom(4) != 0 { try!(write!(f, " chom[4]"))}
      if self.chom(5) != 0 { try!(write!(f, " chom[5]"))}
      if self.chom(6) != 0 { try!(write!(f, " chom[6]"))}
      if self.chom(7) != 0 { try!(write!(f, " chom[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Combine(pub u32);
impl Combine {
  #[inline] pub fn combine(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 0 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_combine(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn comp(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 1 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [1]
  }
  #[inline] pub fn set_comp(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 1 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn decapen(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 2 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [2]
  }
  #[inline] pub fn set_decapen(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 2 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn decap(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 3 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [3]
  }
  #[inline] pub fn set_decap(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 3 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn dten(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 4 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [4]
  }
  #[inline] pub fn set_dten(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 4 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn syncen(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 5 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [5]
  }
  #[inline] pub fn set_syncen(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 5 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn faulten(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 6 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [6]
  }
  #[inline] pub fn set_faulten(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 6 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Combine {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Combine {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.combine(0) != 0 { try!(write!(f, " combine[0]"))}
      if self.combine(1) != 0 { try!(write!(f, " combine[1]"))}
      if self.combine(2) != 0 { try!(write!(f, " combine[2]"))}
      if self.combine(3) != 0 { try!(write!(f, " combine[3]"))}
      if self.comp(0) != 0 { try!(write!(f, " comp[0]"))}
      if self.comp(1) != 0 { try!(write!(f, " comp[1]"))}
      if self.comp(2) != 0 { try!(write!(f, " comp[2]"))}
      if self.comp(3) != 0 { try!(write!(f, " comp[3]"))}
      if self.decapen(0) != 0 { try!(write!(f, " decapen[0]"))}
      if self.decapen(1) != 0 { try!(write!(f, " decapen[1]"))}
      if self.decapen(2) != 0 { try!(write!(f, " decapen[2]"))}
      if self.decapen(3) != 0 { try!(write!(f, " decapen[3]"))}
      if self.decap(0) != 0 { try!(write!(f, " decap[0]"))}
      if self.decap(1) != 0 { try!(write!(f, " decap[1]"))}
      if self.decap(2) != 0 { try!(write!(f, " decap[2]"))}
      if self.decap(3) != 0 { try!(write!(f, " decap[3]"))}
      if self.dten(0) != 0 { try!(write!(f, " dten[0]"))}
      if self.dten(1) != 0 { try!(write!(f, " dten[1]"))}
      if self.dten(2) != 0 { try!(write!(f, " dten[2]"))}
      if self.dten(3) != 0 { try!(write!(f, " dten[3]"))}
      if self.syncen(0) != 0 { try!(write!(f, " syncen[0]"))}
      if self.syncen(1) != 0 { try!(write!(f, " syncen[1]"))}
      if self.syncen(2) != 0 { try!(write!(f, " syncen[2]"))}
      if self.syncen(3) != 0 { try!(write!(f, " syncen[3]"))}
      if self.faulten(0) != 0 { try!(write!(f, " faulten[0]"))}
      if self.faulten(1) != 0 { try!(write!(f, " faulten[1]"))}
      if self.faulten(2) != 0 { try!(write!(f, " faulten[2]"))}
      if self.faulten(3) != 0 { try!(write!(f, " faulten[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Deadtime(pub u32);
impl Deadtime {
  #[inline] pub fn dtval(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3f // [5:0]
  }
  #[inline] pub fn set_dtval(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn dtps(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
  #[inline] pub fn set_dtps(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

}
impl ::core::fmt::Display for Deadtime {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Deadtime {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dtval() != 0 { try!(write!(f, " dtval=0x{:x}", self.dtval()))}
      if self.dtps() != 0 { try!(write!(f, " dtps=0x{:x}", self.dtps()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Exttrig(pub u32);
impl Exttrig {
  #[inline] pub fn ch2trig(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_ch2trig(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn ch3trig(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_ch3trig(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn ch4trig(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_ch4trig(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn ch5trig(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_ch5trig(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn ch0trig(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_ch0trig(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn ch1trig(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_ch1trig(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn inittrigen(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_inittrigen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn trigf(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_trigf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Exttrig {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Exttrig {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ch2trig() != 0 { try!(write!(f, " ch2trig"))}
      if self.ch3trig() != 0 { try!(write!(f, " ch3trig"))}
      if self.ch4trig() != 0 { try!(write!(f, " ch4trig"))}
      if self.ch5trig() != 0 { try!(write!(f, " ch5trig"))}
      if self.ch0trig() != 0 { try!(write!(f, " ch0trig"))}
      if self.ch1trig() != 0 { try!(write!(f, " ch1trig"))}
      if self.inittrigen() != 0 { try!(write!(f, " inittrigen"))}
      if self.trigf() != 0 { try!(write!(f, " trigf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Pol(pub u32);
impl Pol {
  #[inline] pub fn pol(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_pol(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Pol {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pol {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pol(0) != 0 { try!(write!(f, " pol[0]"))}
      if self.pol(1) != 0 { try!(write!(f, " pol[1]"))}
      if self.pol(2) != 0 { try!(write!(f, " pol[2]"))}
      if self.pol(3) != 0 { try!(write!(f, " pol[3]"))}
      if self.pol(4) != 0 { try!(write!(f, " pol[4]"))}
      if self.pol(5) != 0 { try!(write!(f, " pol[5]"))}
      if self.pol(6) != 0 { try!(write!(f, " pol[6]"))}
      if self.pol(7) != 0 { try!(write!(f, " pol[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Fms(pub u32);
impl Fms {
  #[inline] pub fn faultf0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_faultf0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn faultf1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_faultf1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn faultf2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_faultf2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn faultf3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_faultf3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn faultin(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_faultin(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn wpen(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_wpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn faultf(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_faultf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Fms {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fms {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.faultf0() != 0 { try!(write!(f, " faultf0"))}
      if self.faultf1() != 0 { try!(write!(f, " faultf1"))}
      if self.faultf2() != 0 { try!(write!(f, " faultf2"))}
      if self.faultf3() != 0 { try!(write!(f, " faultf3"))}
      if self.faultin() != 0 { try!(write!(f, " faultin"))}
      if self.wpen() != 0 { try!(write!(f, " wpen"))}
      if self.faultf() != 0 { try!(write!(f, " faultf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Filter(pub u32);
impl Filter {
  #[inline] pub fn chfval(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 0 + (index << 2);
     ((self.0 as u32) >> shift) & 0xf // [3:0]
  }
  #[inline] pub fn set_chfval(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0xf) == 0);
     let shift: usize = 0 + (index << 2);
     self.0 &= !(0xf << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Filter {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Filter {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.chfval(0) != 0 { try!(write!(f, " chfval[0]=0x{:x}", self.chfval(0)))}
      if self.chfval(1) != 0 { try!(write!(f, " chfval[1]=0x{:x}", self.chfval(1)))}
      if self.chfval(2) != 0 { try!(write!(f, " chfval[2]=0x{:x}", self.chfval(2)))}
      if self.chfval(3) != 0 { try!(write!(f, " chfval[3]=0x{:x}", self.chfval(3)))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Fltctrl(pub u32);
impl Fltctrl {
  #[inline] pub fn faulten(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_faulten(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn ffltren(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 4 + index;
     ((self.0 as u32) >> shift) & 0x1 // [4]
  }
  #[inline] pub fn set_ffltren(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 4 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn ffval(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  #[inline] pub fn set_ffval(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

}
impl ::core::fmt::Display for Fltctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fltctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.faulten(0) != 0 { try!(write!(f, " faulten[0]"))}
      if self.faulten(1) != 0 { try!(write!(f, " faulten[1]"))}
      if self.faulten(2) != 0 { try!(write!(f, " faulten[2]"))}
      if self.faulten(3) != 0 { try!(write!(f, " faulten[3]"))}
      if self.ffltren(0) != 0 { try!(write!(f, " ffltren[0]"))}
      if self.ffltren(1) != 0 { try!(write!(f, " ffltren[1]"))}
      if self.ffltren(2) != 0 { try!(write!(f, " ffltren[2]"))}
      if self.ffltren(3) != 0 { try!(write!(f, " ffltren[3]"))}
      if self.ffval() != 0 { try!(write!(f, " ffval=0x{:x}", self.ffval()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Qdctrl(pub u32);
impl Qdctrl {
  #[inline] pub fn quaden(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_quaden(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn tofdir(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_tofdir(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn quadir(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_quadir(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn quadmode(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_quadmode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn phbpol(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_phbpol(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn phapol(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_phapol(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn phbfltren(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_phbfltren(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn phafltren(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_phafltren(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Qdctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Qdctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.quaden() != 0 { try!(write!(f, " quaden"))}
      if self.tofdir() != 0 { try!(write!(f, " tofdir"))}
      if self.quadir() != 0 { try!(write!(f, " quadir"))}
      if self.quadmode() != 0 { try!(write!(f, " quadmode"))}
      if self.phbpol() != 0 { try!(write!(f, " phbpol"))}
      if self.phapol() != 0 { try!(write!(f, " phapol"))}
      if self.phbfltren() != 0 { try!(write!(f, " phbfltren"))}
      if self.phafltren() != 0 { try!(write!(f, " phafltren"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Conf(pub u32);
impl Conf {
  #[inline] pub fn numtof(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1f // [4:0]
  }
  #[inline] pub fn set_numtof(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn bdmmode(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
  #[inline] pub fn set_bdmmode(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn gtbeen(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_gtbeen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn gtbeout(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_gtbeout(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

}
impl ::core::fmt::Display for Conf {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Conf {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.numtof() != 0 { try!(write!(f, " numtof=0x{:x}", self.numtof()))}
      if self.bdmmode() != 0 { try!(write!(f, " bdmmode=0x{:x}", self.bdmmode()))}
      if self.gtbeen() != 0 { try!(write!(f, " gtbeen"))}
      if self.gtbeout() != 0 { try!(write!(f, " gtbeout"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Fltpol(pub u32);
impl Fltpol {
  #[inline] pub fn fltpol(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_fltpol(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Fltpol {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fltpol {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fltpol(0) != 0 { try!(write!(f, " fltpol[0]"))}
      if self.fltpol(1) != 0 { try!(write!(f, " fltpol[1]"))}
      if self.fltpol(2) != 0 { try!(write!(f, " fltpol[2]"))}
      if self.fltpol(3) != 0 { try!(write!(f, " fltpol[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Synconf(pub u32);
impl Synconf {
  #[inline] pub fn hwtrigmode(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_hwtrigmode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn cntinc(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_cntinc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn invc(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_invc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn swoc(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_swoc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn syncmode(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_syncmode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn swrstcnt(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_swrstcnt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn swwrbuf(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_swwrbuf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn swom(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_swom(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn swinvc(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_swinvc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline] pub fn swsoc(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline] pub fn set_swsoc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn hwrstcnt(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline] pub fn set_hwrstcnt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline] pub fn hwwrbuf(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline] pub fn set_hwwrbuf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline] pub fn hwom(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline] pub fn set_hwom(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  #[inline] pub fn hwinvc(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  #[inline] pub fn set_hwinvc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  #[inline] pub fn hwsoc(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  #[inline] pub fn set_hwsoc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

}
impl ::core::fmt::Display for Synconf {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Synconf {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.hwtrigmode() != 0 { try!(write!(f, " hwtrigmode"))}
      if self.cntinc() != 0 { try!(write!(f, " cntinc"))}
      if self.invc() != 0 { try!(write!(f, " invc"))}
      if self.swoc() != 0 { try!(write!(f, " swoc"))}
      if self.syncmode() != 0 { try!(write!(f, " syncmode"))}
      if self.swrstcnt() != 0 { try!(write!(f, " swrstcnt"))}
      if self.swwrbuf() != 0 { try!(write!(f, " swwrbuf"))}
      if self.swom() != 0 { try!(write!(f, " swom"))}
      if self.swinvc() != 0 { try!(write!(f, " swinvc"))}
      if self.swsoc() != 0 { try!(write!(f, " swsoc"))}
      if self.hwrstcnt() != 0 { try!(write!(f, " hwrstcnt"))}
      if self.hwwrbuf() != 0 { try!(write!(f, " hwwrbuf"))}
      if self.hwom() != 0 { try!(write!(f, " hwom"))}
      if self.hwinvc() != 0 { try!(write!(f, " hwinvc"))}
      if self.hwsoc() != 0 { try!(write!(f, " hwsoc"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Invctrl(pub u32);
impl Invctrl {
  #[inline] pub fn inven(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_inven(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Invctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Invctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.inven(0) != 0 { try!(write!(f, " inven[0]"))}
      if self.inven(1) != 0 { try!(write!(f, " inven[1]"))}
      if self.inven(2) != 0 { try!(write!(f, " inven[2]"))}
      if self.inven(3) != 0 { try!(write!(f, " inven[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Swoctrl(pub u32);
impl Swoctrl {
  #[inline] pub fn choc(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_choc(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn chocv(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 8 + index;
     ((self.0 as u32) >> shift) & 0x1 // [8]
  }
  #[inline] pub fn set_chocv(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 8 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Swoctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Swoctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.choc(0) != 0 { try!(write!(f, " choc[0]"))}
      if self.choc(1) != 0 { try!(write!(f, " choc[1]"))}
      if self.choc(2) != 0 { try!(write!(f, " choc[2]"))}
      if self.choc(3) != 0 { try!(write!(f, " choc[3]"))}
      if self.choc(4) != 0 { try!(write!(f, " choc[4]"))}
      if self.choc(5) != 0 { try!(write!(f, " choc[5]"))}
      if self.choc(6) != 0 { try!(write!(f, " choc[6]"))}
      if self.choc(7) != 0 { try!(write!(f, " choc[7]"))}
      if self.chocv(0) != 0 { try!(write!(f, " chocv[0]"))}
      if self.chocv(1) != 0 { try!(write!(f, " chocv[1]"))}
      if self.chocv(2) != 0 { try!(write!(f, " chocv[2]"))}
      if self.chocv(3) != 0 { try!(write!(f, " chocv[3]"))}
      if self.chocv(4) != 0 { try!(write!(f, " chocv[4]"))}
      if self.chocv(5) != 0 { try!(write!(f, " chocv[5]"))}
      if self.chocv(6) != 0 { try!(write!(f, " chocv[6]"))}
      if self.chocv(7) != 0 { try!(write!(f, " chocv[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Pwmload(pub u32);
impl Pwmload {
  #[inline] pub fn chsel(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_chsel(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn ldok(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_ldok(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

}
impl ::core::fmt::Display for Pwmload {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pwmload {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.chsel(0) != 0 { try!(write!(f, " chsel[0]"))}
      if self.chsel(1) != 0 { try!(write!(f, " chsel[1]"))}
      if self.chsel(2) != 0 { try!(write!(f, " chsel[2]"))}
      if self.chsel(3) != 0 { try!(write!(f, " chsel[3]"))}
      if self.chsel(4) != 0 { try!(write!(f, " chsel[4]"))}
      if self.chsel(5) != 0 { try!(write!(f, " chsel[5]"))}
      if self.chsel(6) != 0 { try!(write!(f, " chsel[6]"))}
      if self.chsel(7) != 0 { try!(write!(f, " chsel[7]"))}
      if self.ldok() != 0 { try!(write!(f, " ldok"))}
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

