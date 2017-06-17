pub const PWM0: Pwm0 = Periph(0x40028000, Pwm0Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Periph<T>(pub u32, pub T); 

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pwm0Id {}
pub type Pwm0 = Periph<Pwm0Id>;

impl super::sig::Signal<super::sig::M0pwm0> for Pwm0Ch0 {}
impl super::sig::SignalPwm<super::sig::M0pwm0> for Pwm0Ch0 {}
impl super::sig::Signal<super::sig::M0pwm1> for Pwm0Ch1 {}
impl super::sig::SignalPwm<super::sig::M0pwm1> for Pwm0Ch1 {}
impl super::sig::Signal<super::sig::M0pwm2> for Pwm0Ch2 {}
impl super::sig::SignalPwm<super::sig::M0pwm2> for Pwm0Ch2 {}
impl super::sig::Signal<super::sig::M0pwm3> for Pwm0Ch3 {}
impl super::sig::SignalPwm<super::sig::M0pwm3> for Pwm0Ch3 {}


impl<T> Periph<T> {
  #[inline] pub fn ctl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
  #[inline] pub fn ctl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
  #[inline] pub fn ctl(&self) -> Ctl { 
     unsafe {
        Ctl(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  #[inline] pub fn set_ctl(&self, value: Ctl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ctl<F: FnOnce(Ctl) -> Ctl>(&self, f: F) -> &Self {
     let tmp = self.ctl();
     self.set_ctl(f(tmp))
  }

  #[inline] pub fn sync_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
  #[inline] pub fn sync_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
  #[inline] pub fn sync(&self) -> Sync { 
     unsafe {
        Sync(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
  #[inline] pub fn set_sync(&self, value: Sync) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_sync<F: FnOnce(Sync) -> Sync>(&self, f: F) -> &Self {
     let tmp = self.sync();
     self.set_sync(f(tmp))
  }

  #[inline] pub fn enable_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
  #[inline] pub fn enable_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
  #[inline] pub fn enable(&self) -> Enable { 
     unsafe {
        Enable(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
  #[inline] pub fn set_enable(&self, value: Enable) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_enable<F: FnOnce(Enable) -> Enable>(&self, f: F) -> &Self {
     let tmp = self.enable();
     self.set_enable(f(tmp))
  }

  #[inline] pub fn invert_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
  #[inline] pub fn invert_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
  #[inline] pub fn invert(&self) -> Invert { 
     unsafe {
        Invert(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
  #[inline] pub fn set_invert(&self, value: Invert) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_invert<F: FnOnce(Invert) -> Invert>(&self, f: F) -> &Self {
     let tmp = self.invert();
     self.set_invert(f(tmp))
  }

  #[inline] pub fn fault_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
  #[inline] pub fn fault_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
  #[inline] pub fn fault(&self) -> Fault { 
     unsafe {
        Fault(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
  #[inline] pub fn set_fault(&self, value: Fault) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_fault<F: FnOnce(Fault) -> Fault>(&self, f: F) -> &Self {
     let tmp = self.fault();
     self.set_fault(f(tmp))
  }

  #[inline] pub fn inten_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
  #[inline] pub fn inten_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
  #[inline] pub fn inten(&self) -> Inten { 
     unsafe {
        Inten(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
  #[inline] pub fn set_inten(&self, value: Inten) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_inten<F: FnOnce(Inten) -> Inten>(&self, f: F) -> &Self {
     let tmp = self.inten();
     self.set_inten(f(tmp))
  }

  #[inline] pub fn ris_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
  #[inline] pub fn ris_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
  #[inline] pub fn ris(&self) -> Ris { 
     unsafe {
        Ris(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
  #[inline] pub fn set_ris(&self, value: Ris) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
     let tmp = self.ris();
     self.set_ris(f(tmp))
  }

  #[inline] pub fn isc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
  #[inline] pub fn isc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
  #[inline] pub fn isc(&self) -> Isc { 
     unsafe {
        Isc(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
  #[inline] pub fn set_isc(&self, value: Isc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_isc<F: FnOnce(Isc) -> Isc>(&self, f: F) -> &Self {
     let tmp = self.isc();
     self.set_isc(f(tmp))
  }

  #[inline] pub fn status_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
  #[inline] pub fn status_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
  #[inline] pub fn status(&self) -> Status { 
     unsafe {
        Status(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
  #[inline] pub fn set_status(&self, value: Status) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
     let tmp = self.status();
     self.set_status(f(tmp))
  }

  #[inline] pub fn faultval_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
  #[inline] pub fn faultval_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
  #[inline] pub fn faultval(&self) -> Faultval { 
     unsafe {
        Faultval(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }
  #[inline] pub fn set_faultval(&self, value: Faultval) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_faultval<F: FnOnce(Faultval) -> Faultval>(&self, f: F) -> &Self {
     let tmp = self.faultval();
     self.set_faultval(f(tmp))
  }

  #[inline] pub fn enupd_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x28) as *const u32
  }
  #[inline] pub fn enupd_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x28) as *mut u32
  }
  #[inline] pub fn enupd(&self) -> Enupd { 
     unsafe {
        Enupd(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
     }
  }
  #[inline] pub fn set_enupd(&self, value: Enupd) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_enupd<F: FnOnce(Enupd) -> Enupd>(&self, f: F) -> &Self {
     let tmp = self.enupd();
     self.set_enupd(f(tmp))
  }

  #[inline] pub fn _0_ctl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x40) as *const u32
  }
  #[inline] pub fn _0_ctl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x40) as *mut u32
  }
  #[inline] pub fn _0_ctl(&self) -> _0Ctl { 
     unsafe {
        _0Ctl(::core::ptr::read_volatile(((self.0 as usize) + 0x40) as *const u32))
     }
  }
  #[inline] pub fn set_0_ctl(&self, value: _0Ctl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_0_ctl<F: FnOnce(_0Ctl) -> _0Ctl>(&self, f: F) -> &Self {
     let tmp = self._0_ctl();
     self.set_0_ctl(f(tmp))
  }

  #[inline] pub fn _0_inten_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x44) as *const u32
  }
  #[inline] pub fn _0_inten_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x44) as *mut u32
  }
  #[inline] pub fn _0_inten(&self) -> _0Inten { 
     unsafe {
        _0Inten(::core::ptr::read_volatile(((self.0 as usize) + 0x44) as *const u32))
     }
  }
  #[inline] pub fn set_0_inten(&self, value: _0Inten) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x44) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_0_inten<F: FnOnce(_0Inten) -> _0Inten>(&self, f: F) -> &Self {
     let tmp = self._0_inten();
     self.set_0_inten(f(tmp))
  }

  #[inline] pub fn _0_ris_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x48) as *const u32
  }
  #[inline] pub fn _0_ris_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x48) as *mut u32
  }
  #[inline] pub fn _0_ris(&self) -> _0Ris { 
     unsafe {
        _0Ris(::core::ptr::read_volatile(((self.0 as usize) + 0x48) as *const u32))
     }
  }
  #[inline] pub fn set_0_ris(&self, value: _0Ris) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x48) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_0_ris<F: FnOnce(_0Ris) -> _0Ris>(&self, f: F) -> &Self {
     let tmp = self._0_ris();
     self.set_0_ris(f(tmp))
  }

  #[inline] pub fn _0_isc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4c) as *const u32
  }
  #[inline] pub fn _0_isc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4c) as *mut u32
  }
  #[inline] pub fn _0_isc(&self) -> _0Isc { 
     unsafe {
        _0Isc(::core::ptr::read_volatile(((self.0 as usize) + 0x4c) as *const u32))
     }
  }
  #[inline] pub fn set_0_isc(&self, value: _0Isc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_0_isc<F: FnOnce(_0Isc) -> _0Isc>(&self, f: F) -> &Self {
     let tmp = self._0_isc();
     self.set_0_isc(f(tmp))
  }

  #[inline] pub fn _0_load_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x50) as *const u32
  }
  #[inline] pub fn _0_load_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x50) as *mut u32
  }
  #[inline] pub fn _0_load(&self) -> _0Load { 
     unsafe {
        _0Load(::core::ptr::read_volatile(((self.0 as usize) + 0x50) as *const u32))
     }
  }
  #[inline] pub fn set_0_load(&self, value: _0Load) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x50) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_0_load<F: FnOnce(_0Load) -> _0Load>(&self, f: F) -> &Self {
     let tmp = self._0_load();
     self.set_0_load(f(tmp))
  }

  #[inline] pub fn _0_count_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x54) as *const u32
  }
  #[inline] pub fn _0_count_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x54) as *mut u32
  }
  #[inline] pub fn _0_count(&self) -> _0Count { 
     unsafe {
        _0Count(::core::ptr::read_volatile(((self.0 as usize) + 0x54) as *const u32))
     }
  }
  #[inline] pub fn set_0_count(&self, value: _0Count) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x54) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_0_count<F: FnOnce(_0Count) -> _0Count>(&self, f: F) -> &Self {
     let tmp = self._0_count();
     self.set_0_count(f(tmp))
  }

  #[inline] pub fn _0_cmpa_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x58) as *const u32
  }
  #[inline] pub fn _0_cmpa_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x58) as *mut u32
  }
  #[inline] pub fn _0_cmpa(&self) -> _0Cmpa { 
     unsafe {
        _0Cmpa(::core::ptr::read_volatile(((self.0 as usize) + 0x58) as *const u32))
     }
  }
  #[inline] pub fn set_0_cmpa(&self, value: _0Cmpa) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x58) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_0_cmpa<F: FnOnce(_0Cmpa) -> _0Cmpa>(&self, f: F) -> &Self {
     let tmp = self._0_cmpa();
     self.set_0_cmpa(f(tmp))
  }

  #[inline] pub fn _0_cmpb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x5c) as *const u32
  }
  #[inline] pub fn _0_cmpb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x5c) as *mut u32
  }
  #[inline] pub fn _0_cmpb(&self) -> _0Cmpb { 
     unsafe {
        _0Cmpb(::core::ptr::read_volatile(((self.0 as usize) + 0x5c) as *const u32))
     }
  }
  #[inline] pub fn set_0_cmpb(&self, value: _0Cmpb) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x5c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_0_cmpb<F: FnOnce(_0Cmpb) -> _0Cmpb>(&self, f: F) -> &Self {
     let tmp = self._0_cmpb();
     self.set_0_cmpb(f(tmp))
  }

  #[inline] pub fn _0_gena_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x60) as *const u32
  }
  #[inline] pub fn _0_gena_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x60) as *mut u32
  }
  #[inline] pub fn _0_gena(&self) -> _0Gena { 
     unsafe {
        _0Gena(::core::ptr::read_volatile(((self.0 as usize) + 0x60) as *const u32))
     }
  }
  #[inline] pub fn set_0_gena(&self, value: _0Gena) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x60) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_0_gena<F: FnOnce(_0Gena) -> _0Gena>(&self, f: F) -> &Self {
     let tmp = self._0_gena();
     self.set_0_gena(f(tmp))
  }

  #[inline] pub fn _0_genb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x64) as *const u32
  }
  #[inline] pub fn _0_genb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x64) as *mut u32
  }
  #[inline] pub fn _0_genb(&self) -> _0Genb { 
     unsafe {
        _0Genb(::core::ptr::read_volatile(((self.0 as usize) + 0x64) as *const u32))
     }
  }
  #[inline] pub fn set_0_genb(&self, value: _0Genb) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x64) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_0_genb<F: FnOnce(_0Genb) -> _0Genb>(&self, f: F) -> &Self {
     let tmp = self._0_genb();
     self.set_0_genb(f(tmp))
  }

  #[inline] pub fn _0_dbctl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x68) as *const u32
  }
  #[inline] pub fn _0_dbctl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x68) as *mut u32
  }
  #[inline] pub fn _0_dbctl(&self) -> _0Dbctl { 
     unsafe {
        _0Dbctl(::core::ptr::read_volatile(((self.0 as usize) + 0x68) as *const u32))
     }
  }
  #[inline] pub fn set_0_dbctl(&self, value: _0Dbctl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x68) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_0_dbctl<F: FnOnce(_0Dbctl) -> _0Dbctl>(&self, f: F) -> &Self {
     let tmp = self._0_dbctl();
     self.set_0_dbctl(f(tmp))
  }

  #[inline] pub fn _0_dbrise_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x6c) as *const u32
  }
  #[inline] pub fn _0_dbrise_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x6c) as *mut u32
  }
  #[inline] pub fn _0_dbrise(&self) -> _0Dbrise { 
     unsafe {
        _0Dbrise(::core::ptr::read_volatile(((self.0 as usize) + 0x6c) as *const u32))
     }
  }
  #[inline] pub fn set_0_dbrise(&self, value: _0Dbrise) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x6c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_0_dbrise<F: FnOnce(_0Dbrise) -> _0Dbrise>(&self, f: F) -> &Self {
     let tmp = self._0_dbrise();
     self.set_0_dbrise(f(tmp))
  }

  #[inline] pub fn _0_dbfall_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x70) as *const u32
  }
  #[inline] pub fn _0_dbfall_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x70) as *mut u32
  }
  #[inline] pub fn _0_dbfall(&self) -> _0Dbfall { 
     unsafe {
        _0Dbfall(::core::ptr::read_volatile(((self.0 as usize) + 0x70) as *const u32))
     }
  }
  #[inline] pub fn set_0_dbfall(&self, value: _0Dbfall) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x70) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_0_dbfall<F: FnOnce(_0Dbfall) -> _0Dbfall>(&self, f: F) -> &Self {
     let tmp = self._0_dbfall();
     self.set_0_dbfall(f(tmp))
  }

  #[inline] pub fn _0_fltsrc0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x74) as *const u32
  }
  #[inline] pub fn _0_fltsrc0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x74) as *mut u32
  }
  #[inline] pub fn _0_fltsrc0(&self) -> _0Fltsrc0 { 
     unsafe {
        _0Fltsrc0(::core::ptr::read_volatile(((self.0 as usize) + 0x74) as *const u32))
     }
  }
  #[inline] pub fn set_0_fltsrc0(&self, value: _0Fltsrc0) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x74) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_0_fltsrc0<F: FnOnce(_0Fltsrc0) -> _0Fltsrc0>(&self, f: F) -> &Self {
     let tmp = self._0_fltsrc0();
     self.set_0_fltsrc0(f(tmp))
  }

  #[inline] pub fn _0_fltsrc1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x78) as *const u32
  }
  #[inline] pub fn _0_fltsrc1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x78) as *mut u32
  }
  #[inline] pub fn _0_fltsrc1(&self) -> _0Fltsrc1 { 
     unsafe {
        _0Fltsrc1(::core::ptr::read_volatile(((self.0 as usize) + 0x78) as *const u32))
     }
  }
  #[inline] pub fn set_0_fltsrc1(&self, value: _0Fltsrc1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x78) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_0_fltsrc1<F: FnOnce(_0Fltsrc1) -> _0Fltsrc1>(&self, f: F) -> &Self {
     let tmp = self._0_fltsrc1();
     self.set_0_fltsrc1(f(tmp))
  }

  #[inline] pub fn _0_minfltper_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x7c) as *const u32
  }
  #[inline] pub fn _0_minfltper_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x7c) as *mut u32
  }
  #[inline] pub fn _0_minfltper(&self) -> _0Minfltper { 
     unsafe {
        _0Minfltper(::core::ptr::read_volatile(((self.0 as usize) + 0x7c) as *const u32))
     }
  }
  #[inline] pub fn set_0_minfltper(&self, value: _0Minfltper) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x7c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_0_minfltper<F: FnOnce(_0Minfltper) -> _0Minfltper>(&self, f: F) -> &Self {
     let tmp = self._0_minfltper();
     self.set_0_minfltper(f(tmp))
  }

  #[inline] pub fn _1_ctl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x80) as *const u32
  }
  #[inline] pub fn _1_ctl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x80) as *mut u32
  }
  #[inline] pub fn _1_ctl(&self) -> _1Ctl { 
     unsafe {
        _1Ctl(::core::ptr::read_volatile(((self.0 as usize) + 0x80) as *const u32))
     }
  }
  #[inline] pub fn set_1_ctl(&self, value: _1Ctl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x80) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_1_ctl<F: FnOnce(_1Ctl) -> _1Ctl>(&self, f: F) -> &Self {
     let tmp = self._1_ctl();
     self.set_1_ctl(f(tmp))
  }

  #[inline] pub fn _1_inten_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x84) as *const u32
  }
  #[inline] pub fn _1_inten_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x84) as *mut u32
  }
  #[inline] pub fn _1_inten(&self) -> _1Inten { 
     unsafe {
        _1Inten(::core::ptr::read_volatile(((self.0 as usize) + 0x84) as *const u32))
     }
  }
  #[inline] pub fn set_1_inten(&self, value: _1Inten) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x84) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_1_inten<F: FnOnce(_1Inten) -> _1Inten>(&self, f: F) -> &Self {
     let tmp = self._1_inten();
     self.set_1_inten(f(tmp))
  }

  #[inline] pub fn _1_ris_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x88) as *const u32
  }
  #[inline] pub fn _1_ris_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x88) as *mut u32
  }
  #[inline] pub fn _1_ris(&self) -> _1Ris { 
     unsafe {
        _1Ris(::core::ptr::read_volatile(((self.0 as usize) + 0x88) as *const u32))
     }
  }
  #[inline] pub fn set_1_ris(&self, value: _1Ris) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x88) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_1_ris<F: FnOnce(_1Ris) -> _1Ris>(&self, f: F) -> &Self {
     let tmp = self._1_ris();
     self.set_1_ris(f(tmp))
  }

  #[inline] pub fn _1_isc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8c) as *const u32
  }
  #[inline] pub fn _1_isc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8c) as *mut u32
  }
  #[inline] pub fn _1_isc(&self) -> _1Isc { 
     unsafe {
        _1Isc(::core::ptr::read_volatile(((self.0 as usize) + 0x8c) as *const u32))
     }
  }
  #[inline] pub fn set_1_isc(&self, value: _1Isc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_1_isc<F: FnOnce(_1Isc) -> _1Isc>(&self, f: F) -> &Self {
     let tmp = self._1_isc();
     self.set_1_isc(f(tmp))
  }

  #[inline] pub fn _1_load_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x90) as *const u32
  }
  #[inline] pub fn _1_load_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x90) as *mut u32
  }
  #[inline] pub fn _1_load(&self) -> _1Load { 
     unsafe {
        _1Load(::core::ptr::read_volatile(((self.0 as usize) + 0x90) as *const u32))
     }
  }
  #[inline] pub fn set_1_load(&self, value: _1Load) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x90) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_1_load<F: FnOnce(_1Load) -> _1Load>(&self, f: F) -> &Self {
     let tmp = self._1_load();
     self.set_1_load(f(tmp))
  }

  #[inline] pub fn _1_count_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x94) as *const u32
  }
  #[inline] pub fn _1_count_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x94) as *mut u32
  }
  #[inline] pub fn _1_count(&self) -> _1Count { 
     unsafe {
        _1Count(::core::ptr::read_volatile(((self.0 as usize) + 0x94) as *const u32))
     }
  }
  #[inline] pub fn set_1_count(&self, value: _1Count) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x94) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_1_count<F: FnOnce(_1Count) -> _1Count>(&self, f: F) -> &Self {
     let tmp = self._1_count();
     self.set_1_count(f(tmp))
  }

  #[inline] pub fn _1_cmpa_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x98) as *const u32
  }
  #[inline] pub fn _1_cmpa_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x98) as *mut u32
  }
  #[inline] pub fn _1_cmpa(&self) -> _1Cmpa { 
     unsafe {
        _1Cmpa(::core::ptr::read_volatile(((self.0 as usize) + 0x98) as *const u32))
     }
  }
  #[inline] pub fn set_1_cmpa(&self, value: _1Cmpa) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x98) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_1_cmpa<F: FnOnce(_1Cmpa) -> _1Cmpa>(&self, f: F) -> &Self {
     let tmp = self._1_cmpa();
     self.set_1_cmpa(f(tmp))
  }

  #[inline] pub fn _1_cmpb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x9c) as *const u32
  }
  #[inline] pub fn _1_cmpb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x9c) as *mut u32
  }
  #[inline] pub fn _1_cmpb(&self) -> _1Cmpb { 
     unsafe {
        _1Cmpb(::core::ptr::read_volatile(((self.0 as usize) + 0x9c) as *const u32))
     }
  }
  #[inline] pub fn set_1_cmpb(&self, value: _1Cmpb) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x9c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_1_cmpb<F: FnOnce(_1Cmpb) -> _1Cmpb>(&self, f: F) -> &Self {
     let tmp = self._1_cmpb();
     self.set_1_cmpb(f(tmp))
  }

  #[inline] pub fn _1_gena_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa0) as *const u32
  }
  #[inline] pub fn _1_gena_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa0) as *mut u32
  }
  #[inline] pub fn _1_gena(&self) -> _1Gena { 
     unsafe {
        _1Gena(::core::ptr::read_volatile(((self.0 as usize) + 0xa0) as *const u32))
     }
  }
  #[inline] pub fn set_1_gena(&self, value: _1Gena) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa0) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_1_gena<F: FnOnce(_1Gena) -> _1Gena>(&self, f: F) -> &Self {
     let tmp = self._1_gena();
     self.set_1_gena(f(tmp))
  }

  #[inline] pub fn _1_genb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa4) as *const u32
  }
  #[inline] pub fn _1_genb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa4) as *mut u32
  }
  #[inline] pub fn _1_genb(&self) -> _1Genb { 
     unsafe {
        _1Genb(::core::ptr::read_volatile(((self.0 as usize) + 0xa4) as *const u32))
     }
  }
  #[inline] pub fn set_1_genb(&self, value: _1Genb) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa4) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_1_genb<F: FnOnce(_1Genb) -> _1Genb>(&self, f: F) -> &Self {
     let tmp = self._1_genb();
     self.set_1_genb(f(tmp))
  }

  #[inline] pub fn _1_dbctl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa8) as *const u32
  }
  #[inline] pub fn _1_dbctl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa8) as *mut u32
  }
  #[inline] pub fn _1_dbctl(&self) -> _1Dbctl { 
     unsafe {
        _1Dbctl(::core::ptr::read_volatile(((self.0 as usize) + 0xa8) as *const u32))
     }
  }
  #[inline] pub fn set_1_dbctl(&self, value: _1Dbctl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa8) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_1_dbctl<F: FnOnce(_1Dbctl) -> _1Dbctl>(&self, f: F) -> &Self {
     let tmp = self._1_dbctl();
     self.set_1_dbctl(f(tmp))
  }

  #[inline] pub fn _1_dbrise_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xac) as *const u32
  }
  #[inline] pub fn _1_dbrise_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xac) as *mut u32
  }
  #[inline] pub fn _1_dbrise(&self) -> _1Dbrise { 
     unsafe {
        _1Dbrise(::core::ptr::read_volatile(((self.0 as usize) + 0xac) as *const u32))
     }
  }
  #[inline] pub fn set_1_dbrise(&self, value: _1Dbrise) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xac) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_1_dbrise<F: FnOnce(_1Dbrise) -> _1Dbrise>(&self, f: F) -> &Self {
     let tmp = self._1_dbrise();
     self.set_1_dbrise(f(tmp))
  }

  #[inline] pub fn _1_dbfall_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xb0) as *const u32
  }
  #[inline] pub fn _1_dbfall_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xb0) as *mut u32
  }
  #[inline] pub fn _1_dbfall(&self) -> _1Dbfall { 
     unsafe {
        _1Dbfall(::core::ptr::read_volatile(((self.0 as usize) + 0xb0) as *const u32))
     }
  }
  #[inline] pub fn set_1_dbfall(&self, value: _1Dbfall) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xb0) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_1_dbfall<F: FnOnce(_1Dbfall) -> _1Dbfall>(&self, f: F) -> &Self {
     let tmp = self._1_dbfall();
     self.set_1_dbfall(f(tmp))
  }

  #[inline] pub fn _1_fltsrc0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xb4) as *const u32
  }
  #[inline] pub fn _1_fltsrc0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xb4) as *mut u32
  }
  #[inline] pub fn _1_fltsrc0(&self) -> _1Fltsrc0 { 
     unsafe {
        _1Fltsrc0(::core::ptr::read_volatile(((self.0 as usize) + 0xb4) as *const u32))
     }
  }
  #[inline] pub fn set_1_fltsrc0(&self, value: _1Fltsrc0) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xb4) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_1_fltsrc0<F: FnOnce(_1Fltsrc0) -> _1Fltsrc0>(&self, f: F) -> &Self {
     let tmp = self._1_fltsrc0();
     self.set_1_fltsrc0(f(tmp))
  }

  #[inline] pub fn _1_fltsrc1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xb8) as *const u32
  }
  #[inline] pub fn _1_fltsrc1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xb8) as *mut u32
  }
  #[inline] pub fn _1_fltsrc1(&self) -> _1Fltsrc1 { 
     unsafe {
        _1Fltsrc1(::core::ptr::read_volatile(((self.0 as usize) + 0xb8) as *const u32))
     }
  }
  #[inline] pub fn set_1_fltsrc1(&self, value: _1Fltsrc1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xb8) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_1_fltsrc1<F: FnOnce(_1Fltsrc1) -> _1Fltsrc1>(&self, f: F) -> &Self {
     let tmp = self._1_fltsrc1();
     self.set_1_fltsrc1(f(tmp))
  }

  #[inline] pub fn _1_minfltper_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xbc) as *const u32
  }
  #[inline] pub fn _1_minfltper_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xbc) as *mut u32
  }
  #[inline] pub fn _1_minfltper(&self) -> _1Minfltper { 
     unsafe {
        _1Minfltper(::core::ptr::read_volatile(((self.0 as usize) + 0xbc) as *const u32))
     }
  }
  #[inline] pub fn set_1_minfltper(&self, value: _1Minfltper) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xbc) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_1_minfltper<F: FnOnce(_1Minfltper) -> _1Minfltper>(&self, f: F) -> &Self {
     let tmp = self._1_minfltper();
     self.set_1_minfltper(f(tmp))
  }

  #[inline] pub fn _2_ctl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc0) as *const u32
  }
  #[inline] pub fn _2_ctl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc0) as *mut u32
  }
  #[inline] pub fn _2_ctl(&self) -> _2Ctl { 
     unsafe {
        _2Ctl(::core::ptr::read_volatile(((self.0 as usize) + 0xc0) as *const u32))
     }
  }
  #[inline] pub fn set_2_ctl(&self, value: _2Ctl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc0) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_2_ctl<F: FnOnce(_2Ctl) -> _2Ctl>(&self, f: F) -> &Self {
     let tmp = self._2_ctl();
     self.set_2_ctl(f(tmp))
  }

  #[inline] pub fn _2_inten_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc4) as *const u32
  }
  #[inline] pub fn _2_inten_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc4) as *mut u32
  }
  #[inline] pub fn _2_inten(&self) -> _2Inten { 
     unsafe {
        _2Inten(::core::ptr::read_volatile(((self.0 as usize) + 0xc4) as *const u32))
     }
  }
  #[inline] pub fn set_2_inten(&self, value: _2Inten) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc4) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_2_inten<F: FnOnce(_2Inten) -> _2Inten>(&self, f: F) -> &Self {
     let tmp = self._2_inten();
     self.set_2_inten(f(tmp))
  }

  #[inline] pub fn _2_ris_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc8) as *const u32
  }
  #[inline] pub fn _2_ris_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc8) as *mut u32
  }
  #[inline] pub fn _2_ris(&self) -> _2Ris { 
     unsafe {
        _2Ris(::core::ptr::read_volatile(((self.0 as usize) + 0xc8) as *const u32))
     }
  }
  #[inline] pub fn set_2_ris(&self, value: _2Ris) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc8) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_2_ris<F: FnOnce(_2Ris) -> _2Ris>(&self, f: F) -> &Self {
     let tmp = self._2_ris();
     self.set_2_ris(f(tmp))
  }

  #[inline] pub fn _2_isc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xcc) as *const u32
  }
  #[inline] pub fn _2_isc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xcc) as *mut u32
  }
  #[inline] pub fn _2_isc(&self) -> _2Isc { 
     unsafe {
        _2Isc(::core::ptr::read_volatile(((self.0 as usize) + 0xcc) as *const u32))
     }
  }
  #[inline] pub fn set_2_isc(&self, value: _2Isc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xcc) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_2_isc<F: FnOnce(_2Isc) -> _2Isc>(&self, f: F) -> &Self {
     let tmp = self._2_isc();
     self.set_2_isc(f(tmp))
  }

  #[inline] pub fn _2_load_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xd0) as *const u32
  }
  #[inline] pub fn _2_load_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xd0) as *mut u32
  }
  #[inline] pub fn _2_load(&self) -> _2Load { 
     unsafe {
        _2Load(::core::ptr::read_volatile(((self.0 as usize) + 0xd0) as *const u32))
     }
  }
  #[inline] pub fn set_2_load(&self, value: _2Load) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd0) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_2_load<F: FnOnce(_2Load) -> _2Load>(&self, f: F) -> &Self {
     let tmp = self._2_load();
     self.set_2_load(f(tmp))
  }

  #[inline] pub fn _2_count_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xd4) as *const u32
  }
  #[inline] pub fn _2_count_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xd4) as *mut u32
  }
  #[inline] pub fn _2_count(&self) -> _2Count { 
     unsafe {
        _2Count(::core::ptr::read_volatile(((self.0 as usize) + 0xd4) as *const u32))
     }
  }
  #[inline] pub fn set_2_count(&self, value: _2Count) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd4) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_2_count<F: FnOnce(_2Count) -> _2Count>(&self, f: F) -> &Self {
     let tmp = self._2_count();
     self.set_2_count(f(tmp))
  }

  #[inline] pub fn _2_cmpa_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xd8) as *const u32
  }
  #[inline] pub fn _2_cmpa_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xd8) as *mut u32
  }
  #[inline] pub fn _2_cmpa(&self) -> _2Cmpa { 
     unsafe {
        _2Cmpa(::core::ptr::read_volatile(((self.0 as usize) + 0xd8) as *const u32))
     }
  }
  #[inline] pub fn set_2_cmpa(&self, value: _2Cmpa) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd8) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_2_cmpa<F: FnOnce(_2Cmpa) -> _2Cmpa>(&self, f: F) -> &Self {
     let tmp = self._2_cmpa();
     self.set_2_cmpa(f(tmp))
  }

  #[inline] pub fn _2_cmpb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xdc) as *const u32
  }
  #[inline] pub fn _2_cmpb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xdc) as *mut u32
  }
  #[inline] pub fn _2_cmpb(&self) -> _2Cmpb { 
     unsafe {
        _2Cmpb(::core::ptr::read_volatile(((self.0 as usize) + 0xdc) as *const u32))
     }
  }
  #[inline] pub fn set_2_cmpb(&self, value: _2Cmpb) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xdc) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_2_cmpb<F: FnOnce(_2Cmpb) -> _2Cmpb>(&self, f: F) -> &Self {
     let tmp = self._2_cmpb();
     self.set_2_cmpb(f(tmp))
  }

  #[inline] pub fn _2_gena_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xe0) as *const u32
  }
  #[inline] pub fn _2_gena_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xe0) as *mut u32
  }
  #[inline] pub fn _2_gena(&self) -> _2Gena { 
     unsafe {
        _2Gena(::core::ptr::read_volatile(((self.0 as usize) + 0xe0) as *const u32))
     }
  }
  #[inline] pub fn set_2_gena(&self, value: _2Gena) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xe0) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_2_gena<F: FnOnce(_2Gena) -> _2Gena>(&self, f: F) -> &Self {
     let tmp = self._2_gena();
     self.set_2_gena(f(tmp))
  }

  #[inline] pub fn _2_genb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xe4) as *const u32
  }
  #[inline] pub fn _2_genb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xe4) as *mut u32
  }
  #[inline] pub fn _2_genb(&self) -> _2Genb { 
     unsafe {
        _2Genb(::core::ptr::read_volatile(((self.0 as usize) + 0xe4) as *const u32))
     }
  }
  #[inline] pub fn set_2_genb(&self, value: _2Genb) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xe4) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_2_genb<F: FnOnce(_2Genb) -> _2Genb>(&self, f: F) -> &Self {
     let tmp = self._2_genb();
     self.set_2_genb(f(tmp))
  }

  #[inline] pub fn _2_dbctl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xe8) as *const u32
  }
  #[inline] pub fn _2_dbctl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xe8) as *mut u32
  }
  #[inline] pub fn _2_dbctl(&self) -> _2Dbctl { 
     unsafe {
        _2Dbctl(::core::ptr::read_volatile(((self.0 as usize) + 0xe8) as *const u32))
     }
  }
  #[inline] pub fn set_2_dbctl(&self, value: _2Dbctl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xe8) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_2_dbctl<F: FnOnce(_2Dbctl) -> _2Dbctl>(&self, f: F) -> &Self {
     let tmp = self._2_dbctl();
     self.set_2_dbctl(f(tmp))
  }

  #[inline] pub fn _2_dbrise_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xec) as *const u32
  }
  #[inline] pub fn _2_dbrise_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xec) as *mut u32
  }
  #[inline] pub fn _2_dbrise(&self) -> _2Dbrise { 
     unsafe {
        _2Dbrise(::core::ptr::read_volatile(((self.0 as usize) + 0xec) as *const u32))
     }
  }
  #[inline] pub fn set_2_dbrise(&self, value: _2Dbrise) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xec) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_2_dbrise<F: FnOnce(_2Dbrise) -> _2Dbrise>(&self, f: F) -> &Self {
     let tmp = self._2_dbrise();
     self.set_2_dbrise(f(tmp))
  }

  #[inline] pub fn _2_dbfall_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xf0) as *const u32
  }
  #[inline] pub fn _2_dbfall_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xf0) as *mut u32
  }
  #[inline] pub fn _2_dbfall(&self) -> _2Dbfall { 
     unsafe {
        _2Dbfall(::core::ptr::read_volatile(((self.0 as usize) + 0xf0) as *const u32))
     }
  }
  #[inline] pub fn set_2_dbfall(&self, value: _2Dbfall) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xf0) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_2_dbfall<F: FnOnce(_2Dbfall) -> _2Dbfall>(&self, f: F) -> &Self {
     let tmp = self._2_dbfall();
     self.set_2_dbfall(f(tmp))
  }

  #[inline] pub fn _2_fltsrc0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xf4) as *const u32
  }
  #[inline] pub fn _2_fltsrc0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xf4) as *mut u32
  }
  #[inline] pub fn _2_fltsrc0(&self) -> _2Fltsrc0 { 
     unsafe {
        _2Fltsrc0(::core::ptr::read_volatile(((self.0 as usize) + 0xf4) as *const u32))
     }
  }
  #[inline] pub fn set_2_fltsrc0(&self, value: _2Fltsrc0) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xf4) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_2_fltsrc0<F: FnOnce(_2Fltsrc0) -> _2Fltsrc0>(&self, f: F) -> &Self {
     let tmp = self._2_fltsrc0();
     self.set_2_fltsrc0(f(tmp))
  }

  #[inline] pub fn _2_fltsrc1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xf8) as *const u32
  }
  #[inline] pub fn _2_fltsrc1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xf8) as *mut u32
  }
  #[inline] pub fn _2_fltsrc1(&self) -> _2Fltsrc1 { 
     unsafe {
        _2Fltsrc1(::core::ptr::read_volatile(((self.0 as usize) + 0xf8) as *const u32))
     }
  }
  #[inline] pub fn set_2_fltsrc1(&self, value: _2Fltsrc1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xf8) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_2_fltsrc1<F: FnOnce(_2Fltsrc1) -> _2Fltsrc1>(&self, f: F) -> &Self {
     let tmp = self._2_fltsrc1();
     self.set_2_fltsrc1(f(tmp))
  }

  #[inline] pub fn _2_minfltper_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xfc) as *const u32
  }
  #[inline] pub fn _2_minfltper_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xfc) as *mut u32
  }
  #[inline] pub fn _2_minfltper(&self) -> _2Minfltper { 
     unsafe {
        _2Minfltper(::core::ptr::read_volatile(((self.0 as usize) + 0xfc) as *const u32))
     }
  }
  #[inline] pub fn set_2_minfltper(&self, value: _2Minfltper) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xfc) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_2_minfltper<F: FnOnce(_2Minfltper) -> _2Minfltper>(&self, f: F) -> &Self {
     let tmp = self._2_minfltper();
     self.set_2_minfltper(f(tmp))
  }

  #[inline] pub fn _3_ctl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x100) as *const u32
  }
  #[inline] pub fn _3_ctl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x100) as *mut u32
  }
  #[inline] pub fn _3_ctl(&self) -> _3Ctl { 
     unsafe {
        _3Ctl(::core::ptr::read_volatile(((self.0 as usize) + 0x100) as *const u32))
     }
  }
  #[inline] pub fn set_3_ctl(&self, value: _3Ctl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x100) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_3_ctl<F: FnOnce(_3Ctl) -> _3Ctl>(&self, f: F) -> &Self {
     let tmp = self._3_ctl();
     self.set_3_ctl(f(tmp))
  }

  #[inline] pub fn _3_inten_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x104) as *const u32
  }
  #[inline] pub fn _3_inten_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x104) as *mut u32
  }
  #[inline] pub fn _3_inten(&self) -> _3Inten { 
     unsafe {
        _3Inten(::core::ptr::read_volatile(((self.0 as usize) + 0x104) as *const u32))
     }
  }
  #[inline] pub fn set_3_inten(&self, value: _3Inten) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x104) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_3_inten<F: FnOnce(_3Inten) -> _3Inten>(&self, f: F) -> &Self {
     let tmp = self._3_inten();
     self.set_3_inten(f(tmp))
  }

  #[inline] pub fn _3_ris_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x108) as *const u32
  }
  #[inline] pub fn _3_ris_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x108) as *mut u32
  }
  #[inline] pub fn _3_ris(&self) -> _3Ris { 
     unsafe {
        _3Ris(::core::ptr::read_volatile(((self.0 as usize) + 0x108) as *const u32))
     }
  }
  #[inline] pub fn set_3_ris(&self, value: _3Ris) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x108) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_3_ris<F: FnOnce(_3Ris) -> _3Ris>(&self, f: F) -> &Self {
     let tmp = self._3_ris();
     self.set_3_ris(f(tmp))
  }

  #[inline] pub fn _3_isc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10c) as *const u32
  }
  #[inline] pub fn _3_isc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10c) as *mut u32
  }
  #[inline] pub fn _3_isc(&self) -> _3Isc { 
     unsafe {
        _3Isc(::core::ptr::read_volatile(((self.0 as usize) + 0x10c) as *const u32))
     }
  }
  #[inline] pub fn set_3_isc(&self, value: _3Isc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_3_isc<F: FnOnce(_3Isc) -> _3Isc>(&self, f: F) -> &Self {
     let tmp = self._3_isc();
     self.set_3_isc(f(tmp))
  }

  #[inline] pub fn _3_load_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x110) as *const u32
  }
  #[inline] pub fn _3_load_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x110) as *mut u32
  }
  #[inline] pub fn _3_load(&self) -> _3Load { 
     unsafe {
        _3Load(::core::ptr::read_volatile(((self.0 as usize) + 0x110) as *const u32))
     }
  }
  #[inline] pub fn set_3_load(&self, value: _3Load) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x110) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_3_load<F: FnOnce(_3Load) -> _3Load>(&self, f: F) -> &Self {
     let tmp = self._3_load();
     self.set_3_load(f(tmp))
  }

  #[inline] pub fn _3_count_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x114) as *const u32
  }
  #[inline] pub fn _3_count_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x114) as *mut u32
  }
  #[inline] pub fn _3_count(&self) -> _3Count { 
     unsafe {
        _3Count(::core::ptr::read_volatile(((self.0 as usize) + 0x114) as *const u32))
     }
  }
  #[inline] pub fn set_3_count(&self, value: _3Count) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x114) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_3_count<F: FnOnce(_3Count) -> _3Count>(&self, f: F) -> &Self {
     let tmp = self._3_count();
     self.set_3_count(f(tmp))
  }

  #[inline] pub fn _3_cmpa_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x118) as *const u32
  }
  #[inline] pub fn _3_cmpa_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x118) as *mut u32
  }
  #[inline] pub fn _3_cmpa(&self) -> _3Cmpa { 
     unsafe {
        _3Cmpa(::core::ptr::read_volatile(((self.0 as usize) + 0x118) as *const u32))
     }
  }
  #[inline] pub fn set_3_cmpa(&self, value: _3Cmpa) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x118) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_3_cmpa<F: FnOnce(_3Cmpa) -> _3Cmpa>(&self, f: F) -> &Self {
     let tmp = self._3_cmpa();
     self.set_3_cmpa(f(tmp))
  }

  #[inline] pub fn _3_cmpb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x11c) as *const u32
  }
  #[inline] pub fn _3_cmpb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x11c) as *mut u32
  }
  #[inline] pub fn _3_cmpb(&self) -> _3Cmpb { 
     unsafe {
        _3Cmpb(::core::ptr::read_volatile(((self.0 as usize) + 0x11c) as *const u32))
     }
  }
  #[inline] pub fn set_3_cmpb(&self, value: _3Cmpb) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x11c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_3_cmpb<F: FnOnce(_3Cmpb) -> _3Cmpb>(&self, f: F) -> &Self {
     let tmp = self._3_cmpb();
     self.set_3_cmpb(f(tmp))
  }

  #[inline] pub fn _3_gena_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x120) as *const u32
  }
  #[inline] pub fn _3_gena_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x120) as *mut u32
  }
  #[inline] pub fn _3_gena(&self) -> _3Gena { 
     unsafe {
        _3Gena(::core::ptr::read_volatile(((self.0 as usize) + 0x120) as *const u32))
     }
  }
  #[inline] pub fn set_3_gena(&self, value: _3Gena) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x120) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_3_gena<F: FnOnce(_3Gena) -> _3Gena>(&self, f: F) -> &Self {
     let tmp = self._3_gena();
     self.set_3_gena(f(tmp))
  }

  #[inline] pub fn _3_genb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x124) as *const u32
  }
  #[inline] pub fn _3_genb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x124) as *mut u32
  }
  #[inline] pub fn _3_genb(&self) -> _3Genb { 
     unsafe {
        _3Genb(::core::ptr::read_volatile(((self.0 as usize) + 0x124) as *const u32))
     }
  }
  #[inline] pub fn set_3_genb(&self, value: _3Genb) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x124) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_3_genb<F: FnOnce(_3Genb) -> _3Genb>(&self, f: F) -> &Self {
     let tmp = self._3_genb();
     self.set_3_genb(f(tmp))
  }

  #[inline] pub fn _3_dbctl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x128) as *const u32
  }
  #[inline] pub fn _3_dbctl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x128) as *mut u32
  }
  #[inline] pub fn _3_dbctl(&self) -> _3Dbctl { 
     unsafe {
        _3Dbctl(::core::ptr::read_volatile(((self.0 as usize) + 0x128) as *const u32))
     }
  }
  #[inline] pub fn set_3_dbctl(&self, value: _3Dbctl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x128) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_3_dbctl<F: FnOnce(_3Dbctl) -> _3Dbctl>(&self, f: F) -> &Self {
     let tmp = self._3_dbctl();
     self.set_3_dbctl(f(tmp))
  }

  #[inline] pub fn _3_dbrise_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x12c) as *const u32
  }
  #[inline] pub fn _3_dbrise_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x12c) as *mut u32
  }
  #[inline] pub fn _3_dbrise(&self) -> _3Dbrise { 
     unsafe {
        _3Dbrise(::core::ptr::read_volatile(((self.0 as usize) + 0x12c) as *const u32))
     }
  }
  #[inline] pub fn set_3_dbrise(&self, value: _3Dbrise) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x12c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_3_dbrise<F: FnOnce(_3Dbrise) -> _3Dbrise>(&self, f: F) -> &Self {
     let tmp = self._3_dbrise();
     self.set_3_dbrise(f(tmp))
  }

  #[inline] pub fn _3_dbfall_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x130) as *const u32
  }
  #[inline] pub fn _3_dbfall_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x130) as *mut u32
  }
  #[inline] pub fn _3_dbfall(&self) -> _3Dbfall { 
     unsafe {
        _3Dbfall(::core::ptr::read_volatile(((self.0 as usize) + 0x130) as *const u32))
     }
  }
  #[inline] pub fn set_3_dbfall(&self, value: _3Dbfall) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x130) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_3_dbfall<F: FnOnce(_3Dbfall) -> _3Dbfall>(&self, f: F) -> &Self {
     let tmp = self._3_dbfall();
     self.set_3_dbfall(f(tmp))
  }

  #[inline] pub fn _3_fltsrc0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x134) as *const u32
  }
  #[inline] pub fn _3_fltsrc0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x134) as *mut u32
  }
  #[inline] pub fn _3_fltsrc0(&self) -> _3Fltsrc0 { 
     unsafe {
        _3Fltsrc0(::core::ptr::read_volatile(((self.0 as usize) + 0x134) as *const u32))
     }
  }
  #[inline] pub fn set_3_fltsrc0(&self, value: _3Fltsrc0) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x134) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_3_fltsrc0<F: FnOnce(_3Fltsrc0) -> _3Fltsrc0>(&self, f: F) -> &Self {
     let tmp = self._3_fltsrc0();
     self.set_3_fltsrc0(f(tmp))
  }

  #[inline] pub fn _3_fltsrc1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x138) as *const u32
  }
  #[inline] pub fn _3_fltsrc1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x138) as *mut u32
  }
  #[inline] pub fn _3_fltsrc1(&self) -> _3Fltsrc1 { 
     unsafe {
        _3Fltsrc1(::core::ptr::read_volatile(((self.0 as usize) + 0x138) as *const u32))
     }
  }
  #[inline] pub fn set_3_fltsrc1(&self, value: _3Fltsrc1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x138) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_3_fltsrc1<F: FnOnce(_3Fltsrc1) -> _3Fltsrc1>(&self, f: F) -> &Self {
     let tmp = self._3_fltsrc1();
     self.set_3_fltsrc1(f(tmp))
  }

  #[inline] pub fn _3_minfltper_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x13c) as *const u32
  }
  #[inline] pub fn _3_minfltper_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x13c) as *mut u32
  }
  #[inline] pub fn _3_minfltper(&self) -> _3Minfltper { 
     unsafe {
        _3Minfltper(::core::ptr::read_volatile(((self.0 as usize) + 0x13c) as *const u32))
     }
  }
  #[inline] pub fn set_3_minfltper(&self, value: _3Minfltper) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x13c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_3_minfltper<F: FnOnce(_3Minfltper) -> _3Minfltper>(&self, f: F) -> &Self {
     let tmp = self._3_minfltper();
     self.set_3_minfltper(f(tmp))
  }

  #[inline] pub fn _0_fltsen_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x800) as *const u32
  }
  #[inline] pub fn _0_fltsen_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x800) as *mut u32
  }
  #[inline] pub fn _0_fltsen(&self) -> _0Fltsen { 
     unsafe {
        _0Fltsen(::core::ptr::read_volatile(((self.0 as usize) + 0x800) as *const u32))
     }
  }
  #[inline] pub fn set_0_fltsen(&self, value: _0Fltsen) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x800) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_0_fltsen<F: FnOnce(_0Fltsen) -> _0Fltsen>(&self, f: F) -> &Self {
     let tmp = self._0_fltsen();
     self.set_0_fltsen(f(tmp))
  }

  #[inline] pub fn _0_fltstat0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x804) as *const u32
  }
  #[inline] pub fn _0_fltstat0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x804) as *mut u32
  }
  #[inline] pub fn _0_fltstat0(&self) -> _0Fltstat0 { 
     unsafe {
        _0Fltstat0(::core::ptr::read_volatile(((self.0 as usize) + 0x804) as *const u32))
     }
  }

  #[inline] pub fn _0_fltstat1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x808) as *const u32
  }
  #[inline] pub fn _0_fltstat1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x808) as *mut u32
  }
  #[inline] pub fn _0_fltstat1(&self) -> _0Fltstat1 { 
     unsafe {
        _0Fltstat1(::core::ptr::read_volatile(((self.0 as usize) + 0x808) as *const u32))
     }
  }

  #[inline] pub fn _1_fltsen_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x880) as *const u32
  }
  #[inline] pub fn _1_fltsen_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x880) as *mut u32
  }
  #[inline] pub fn _1_fltsen(&self) -> _1Fltsen { 
     unsafe {
        _1Fltsen(::core::ptr::read_volatile(((self.0 as usize) + 0x880) as *const u32))
     }
  }
  #[inline] pub fn set_1_fltsen(&self, value: _1Fltsen) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x880) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_1_fltsen<F: FnOnce(_1Fltsen) -> _1Fltsen>(&self, f: F) -> &Self {
     let tmp = self._1_fltsen();
     self.set_1_fltsen(f(tmp))
  }

  #[inline] pub fn _1_fltstat0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x884) as *const u32
  }
  #[inline] pub fn _1_fltstat0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x884) as *mut u32
  }
  #[inline] pub fn _1_fltstat0(&self) -> _1Fltstat0 { 
     unsafe {
        _1Fltstat0(::core::ptr::read_volatile(((self.0 as usize) + 0x884) as *const u32))
     }
  }

  #[inline] pub fn _1_fltstat1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x888) as *const u32
  }
  #[inline] pub fn _1_fltstat1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x888) as *mut u32
  }
  #[inline] pub fn _1_fltstat1(&self) -> _1Fltstat1 { 
     unsafe {
        _1Fltstat1(::core::ptr::read_volatile(((self.0 as usize) + 0x888) as *const u32))
     }
  }

  #[inline] pub fn _2_fltsen_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x900) as *const u32
  }
  #[inline] pub fn _2_fltsen_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x900) as *mut u32
  }
  #[inline] pub fn _2_fltsen(&self) -> _2Fltsen { 
     unsafe {
        _2Fltsen(::core::ptr::read_volatile(((self.0 as usize) + 0x900) as *const u32))
     }
  }
  #[inline] pub fn set_2_fltsen(&self, value: _2Fltsen) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x900) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_2_fltsen<F: FnOnce(_2Fltsen) -> _2Fltsen>(&self, f: F) -> &Self {
     let tmp = self._2_fltsen();
     self.set_2_fltsen(f(tmp))
  }

  #[inline] pub fn _2_fltstat0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x904) as *const u32
  }
  #[inline] pub fn _2_fltstat0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x904) as *mut u32
  }
  #[inline] pub fn _2_fltstat0(&self) -> _2Fltstat0 { 
     unsafe {
        _2Fltstat0(::core::ptr::read_volatile(((self.0 as usize) + 0x904) as *const u32))
     }
  }

  #[inline] pub fn _2_fltstat1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x908) as *const u32
  }
  #[inline] pub fn _2_fltstat1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x908) as *mut u32
  }
  #[inline] pub fn _2_fltstat1(&self) -> _2Fltstat1 { 
     unsafe {
        _2Fltstat1(::core::ptr::read_volatile(((self.0 as usize) + 0x908) as *const u32))
     }
  }

  #[inline] pub fn _3_fltsen_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x980) as *const u32
  }
  #[inline] pub fn _3_fltsen_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x980) as *mut u32
  }
  #[inline] pub fn _3_fltsen(&self) -> _3Fltsen { 
     unsafe {
        _3Fltsen(::core::ptr::read_volatile(((self.0 as usize) + 0x980) as *const u32))
     }
  }
  #[inline] pub fn set_3_fltsen(&self, value: _3Fltsen) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x980) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_3_fltsen<F: FnOnce(_3Fltsen) -> _3Fltsen>(&self, f: F) -> &Self {
     let tmp = self._3_fltsen();
     self.set_3_fltsen(f(tmp))
  }

  #[inline] pub fn _3_fltstat0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x984) as *const u32
  }
  #[inline] pub fn _3_fltstat0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x984) as *mut u32
  }
  #[inline] pub fn _3_fltstat0(&self) -> _3Fltstat0 { 
     unsafe {
        _3Fltstat0(::core::ptr::read_volatile(((self.0 as usize) + 0x984) as *const u32))
     }
  }

  #[inline] pub fn _3_fltstat1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x988) as *const u32
  }
  #[inline] pub fn _3_fltstat1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x988) as *mut u32
  }
  #[inline] pub fn _3_fltstat1(&self) -> _3Fltstat1 { 
     unsafe {
        _3Fltstat1(::core::ptr::read_volatile(((self.0 as usize) + 0x988) as *const u32))
     }
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
pub struct Ctl(pub u32);
impl Ctl {
  #[inline] pub fn pwm_ctl_globalsync0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_ctl_globalsync0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_ctl_globalsync1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_ctl_globalsync1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_ctl_globalsync2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_ctl_globalsync2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_ctl_globalsync3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_ctl_globalsync3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
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
      if self.pwm_ctl_globalsync0() != 0 { try!(write!(f, " pwm_ctl_globalsync0"))}
      if self.pwm_ctl_globalsync1() != 0 { try!(write!(f, " pwm_ctl_globalsync1"))}
      if self.pwm_ctl_globalsync2() != 0 { try!(write!(f, " pwm_ctl_globalsync2"))}
      if self.pwm_ctl_globalsync3() != 0 { try!(write!(f, " pwm_ctl_globalsync3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Sync(pub u32);
impl Sync {
  #[inline] pub fn pwm_sync_sync0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_sync_sync0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_sync_sync1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_sync_sync1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_sync_sync2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_sync_sync2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_sync_sync3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_sync_sync3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
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
      if self.pwm_sync_sync0() != 0 { try!(write!(f, " pwm_sync_sync0"))}
      if self.pwm_sync_sync1() != 0 { try!(write!(f, " pwm_sync_sync1"))}
      if self.pwm_sync_sync2() != 0 { try!(write!(f, " pwm_sync_sync2"))}
      if self.pwm_sync_sync3() != 0 { try!(write!(f, " pwm_sync_sync3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Enable(pub u32);
impl Enable {
  #[inline] pub fn pwm_enable_pwm0en(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_enable_pwm0en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_enable_pwm1en(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_enable_pwm1en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_enable_pwm2en(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_enable_pwm2en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_enable_pwm3en(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_enable_pwm3en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_enable_pwm4en(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_enable_pwm4en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_enable_pwm5en(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_enable_pwm5en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn pwm_enable_pwm6en(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_pwm_enable_pwm6en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn pwm_enable_pwm7en(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_pwm_enable_pwm7en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Enable {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Enable {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_enable_pwm0en() != 0 { try!(write!(f, " pwm_enable_pwm0en"))}
      if self.pwm_enable_pwm1en() != 0 { try!(write!(f, " pwm_enable_pwm1en"))}
      if self.pwm_enable_pwm2en() != 0 { try!(write!(f, " pwm_enable_pwm2en"))}
      if self.pwm_enable_pwm3en() != 0 { try!(write!(f, " pwm_enable_pwm3en"))}
      if self.pwm_enable_pwm4en() != 0 { try!(write!(f, " pwm_enable_pwm4en"))}
      if self.pwm_enable_pwm5en() != 0 { try!(write!(f, " pwm_enable_pwm5en"))}
      if self.pwm_enable_pwm6en() != 0 { try!(write!(f, " pwm_enable_pwm6en"))}
      if self.pwm_enable_pwm7en() != 0 { try!(write!(f, " pwm_enable_pwm7en"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Invert(pub u32);
impl Invert {
  #[inline] pub fn pwm_invert_pwm0inv(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_invert_pwm0inv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_invert_pwm1inv(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_invert_pwm1inv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_invert_pwm2inv(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_invert_pwm2inv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_invert_pwm3inv(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_invert_pwm3inv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_invert_pwm4inv(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_invert_pwm4inv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_invert_pwm5inv(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_invert_pwm5inv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn pwm_invert_pwm6inv(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_pwm_invert_pwm6inv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn pwm_invert_pwm7inv(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_pwm_invert_pwm7inv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Invert {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Invert {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_invert_pwm0inv() != 0 { try!(write!(f, " pwm_invert_pwm0inv"))}
      if self.pwm_invert_pwm1inv() != 0 { try!(write!(f, " pwm_invert_pwm1inv"))}
      if self.pwm_invert_pwm2inv() != 0 { try!(write!(f, " pwm_invert_pwm2inv"))}
      if self.pwm_invert_pwm3inv() != 0 { try!(write!(f, " pwm_invert_pwm3inv"))}
      if self.pwm_invert_pwm4inv() != 0 { try!(write!(f, " pwm_invert_pwm4inv"))}
      if self.pwm_invert_pwm5inv() != 0 { try!(write!(f, " pwm_invert_pwm5inv"))}
      if self.pwm_invert_pwm6inv() != 0 { try!(write!(f, " pwm_invert_pwm6inv"))}
      if self.pwm_invert_pwm7inv() != 0 { try!(write!(f, " pwm_invert_pwm7inv"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Fault(pub u32);
impl Fault {
  #[inline] pub fn pwm_fault_fault0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_fault_fault0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_fault_fault1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_fault_fault1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_fault_fault2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_fault_fault2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_fault_fault3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_fault_fault3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_fault_fault4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_fault_fault4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_fault_fault5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_fault_fault5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn pwm_fault_fault6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_pwm_fault_fault6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn pwm_fault_fault7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_pwm_fault_fault7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Fault {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fault {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_fault_fault0() != 0 { try!(write!(f, " pwm_fault_fault0"))}
      if self.pwm_fault_fault1() != 0 { try!(write!(f, " pwm_fault_fault1"))}
      if self.pwm_fault_fault2() != 0 { try!(write!(f, " pwm_fault_fault2"))}
      if self.pwm_fault_fault3() != 0 { try!(write!(f, " pwm_fault_fault3"))}
      if self.pwm_fault_fault4() != 0 { try!(write!(f, " pwm_fault_fault4"))}
      if self.pwm_fault_fault5() != 0 { try!(write!(f, " pwm_fault_fault5"))}
      if self.pwm_fault_fault6() != 0 { try!(write!(f, " pwm_fault_fault6"))}
      if self.pwm_fault_fault7() != 0 { try!(write!(f, " pwm_fault_fault7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Inten(pub u32);
impl Inten {
  #[inline] pub fn pwm_inten_intpwm0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_inten_intpwm0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_inten_intpwm1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_inten_intpwm1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_inten_intpwm2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_inten_intpwm2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_inten_intpwm3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_inten_intpwm3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_inten_intfault0(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline] pub fn set_pwm_inten_intfault0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline] pub fn pwm_inten_intfault1(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline] pub fn set_pwm_inten_intfault1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline] pub fn pwm_inten_intfault2(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline] pub fn set_pwm_inten_intfault2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  #[inline] pub fn pwm_inten_intfault3(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  #[inline] pub fn set_pwm_inten_intfault3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

}
impl ::core::fmt::Display for Inten {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Inten {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_inten_intpwm0() != 0 { try!(write!(f, " pwm_inten_intpwm0"))}
      if self.pwm_inten_intpwm1() != 0 { try!(write!(f, " pwm_inten_intpwm1"))}
      if self.pwm_inten_intpwm2() != 0 { try!(write!(f, " pwm_inten_intpwm2"))}
      if self.pwm_inten_intpwm3() != 0 { try!(write!(f, " pwm_inten_intpwm3"))}
      if self.pwm_inten_intfault0() != 0 { try!(write!(f, " pwm_inten_intfault0"))}
      if self.pwm_inten_intfault1() != 0 { try!(write!(f, " pwm_inten_intfault1"))}
      if self.pwm_inten_intfault2() != 0 { try!(write!(f, " pwm_inten_intfault2"))}
      if self.pwm_inten_intfault3() != 0 { try!(write!(f, " pwm_inten_intfault3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ris(pub u32);
impl Ris {
  #[inline] pub fn pwm_ris_intpwm0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_ris_intpwm0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_ris_intpwm1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_ris_intpwm1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_ris_intpwm2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_ris_intpwm2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_ris_intpwm3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_ris_intpwm3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_ris_intfault0(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline] pub fn set_pwm_ris_intfault0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline] pub fn pwm_ris_intfault1(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline] pub fn set_pwm_ris_intfault1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline] pub fn pwm_ris_intfault2(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline] pub fn set_pwm_ris_intfault2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  #[inline] pub fn pwm_ris_intfault3(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  #[inline] pub fn set_pwm_ris_intfault3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
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
      if self.pwm_ris_intpwm0() != 0 { try!(write!(f, " pwm_ris_intpwm0"))}
      if self.pwm_ris_intpwm1() != 0 { try!(write!(f, " pwm_ris_intpwm1"))}
      if self.pwm_ris_intpwm2() != 0 { try!(write!(f, " pwm_ris_intpwm2"))}
      if self.pwm_ris_intpwm3() != 0 { try!(write!(f, " pwm_ris_intpwm3"))}
      if self.pwm_ris_intfault0() != 0 { try!(write!(f, " pwm_ris_intfault0"))}
      if self.pwm_ris_intfault1() != 0 { try!(write!(f, " pwm_ris_intfault1"))}
      if self.pwm_ris_intfault2() != 0 { try!(write!(f, " pwm_ris_intfault2"))}
      if self.pwm_ris_intfault3() != 0 { try!(write!(f, " pwm_ris_intfault3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Isc(pub u32);
impl Isc {
  #[inline] pub fn pwm_isc_intpwm0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_isc_intpwm0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_isc_intpwm1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_isc_intpwm1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_isc_intpwm2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_isc_intpwm2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_isc_intpwm3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_isc_intpwm3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_isc_intfault0(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline] pub fn set_pwm_isc_intfault0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline] pub fn pwm_isc_intfault1(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline] pub fn set_pwm_isc_intfault1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline] pub fn pwm_isc_intfault2(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline] pub fn set_pwm_isc_intfault2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  #[inline] pub fn pwm_isc_intfault3(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  #[inline] pub fn set_pwm_isc_intfault3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

}
impl ::core::fmt::Display for Isc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Isc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_isc_intpwm0() != 0 { try!(write!(f, " pwm_isc_intpwm0"))}
      if self.pwm_isc_intpwm1() != 0 { try!(write!(f, " pwm_isc_intpwm1"))}
      if self.pwm_isc_intpwm2() != 0 { try!(write!(f, " pwm_isc_intpwm2"))}
      if self.pwm_isc_intpwm3() != 0 { try!(write!(f, " pwm_isc_intpwm3"))}
      if self.pwm_isc_intfault0() != 0 { try!(write!(f, " pwm_isc_intfault0"))}
      if self.pwm_isc_intfault1() != 0 { try!(write!(f, " pwm_isc_intfault1"))}
      if self.pwm_isc_intfault2() != 0 { try!(write!(f, " pwm_isc_intfault2"))}
      if self.pwm_isc_intfault3() != 0 { try!(write!(f, " pwm_isc_intfault3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Status(pub u32);
impl Status {
  #[inline] pub fn pwm_status_fault0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_status_fault0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_status_fault1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_status_fault1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_status_fault2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_status_fault2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_status_fault3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_status_fault3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
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
      if self.pwm_status_fault0() != 0 { try!(write!(f, " pwm_status_fault0"))}
      if self.pwm_status_fault1() != 0 { try!(write!(f, " pwm_status_fault1"))}
      if self.pwm_status_fault2() != 0 { try!(write!(f, " pwm_status_fault2"))}
      if self.pwm_status_fault3() != 0 { try!(write!(f, " pwm_status_fault3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Faultval(pub u32);
impl Faultval {
  #[inline] pub fn pwm_faultval_pwm0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_faultval_pwm0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_faultval_pwm1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_faultval_pwm1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_faultval_pwm2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_faultval_pwm2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_faultval_pwm3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_faultval_pwm3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_faultval_pwm4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_faultval_pwm4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_faultval_pwm5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_faultval_pwm5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn pwm_faultval_pwm6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_pwm_faultval_pwm6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn pwm_faultval_pwm7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_pwm_faultval_pwm7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Faultval {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Faultval {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_faultval_pwm0() != 0 { try!(write!(f, " pwm_faultval_pwm0"))}
      if self.pwm_faultval_pwm1() != 0 { try!(write!(f, " pwm_faultval_pwm1"))}
      if self.pwm_faultval_pwm2() != 0 { try!(write!(f, " pwm_faultval_pwm2"))}
      if self.pwm_faultval_pwm3() != 0 { try!(write!(f, " pwm_faultval_pwm3"))}
      if self.pwm_faultval_pwm4() != 0 { try!(write!(f, " pwm_faultval_pwm4"))}
      if self.pwm_faultval_pwm5() != 0 { try!(write!(f, " pwm_faultval_pwm5"))}
      if self.pwm_faultval_pwm6() != 0 { try!(write!(f, " pwm_faultval_pwm6"))}
      if self.pwm_faultval_pwm7() != 0 { try!(write!(f, " pwm_faultval_pwm7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Enupd(pub u32);
impl Enupd {
  #[inline] pub fn pwm_enupd_enupd0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline] pub fn set_pwm_enupd_enupd0(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_enupd_enupd1(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
  #[inline] pub fn set_pwm_enupd_enupd1(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_enupd_enupd2(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  #[inline] pub fn set_pwm_enupd_enupd2(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_enupd_enupd3(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
  #[inline] pub fn set_pwm_enupd_enupd3(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn pwm_enupd_enupd4(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  #[inline] pub fn set_pwm_enupd_enupd4(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn pwm_enupd_enupd5(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
  #[inline] pub fn set_pwm_enupd_enupd5(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn pwm_enupd_enupd6(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x3 // [13:12]
  }
  #[inline] pub fn set_pwm_enupd_enupd6(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn pwm_enupd_enupd7(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x3 // [15:14]
  }
  #[inline] pub fn set_pwm_enupd_enupd7(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 14);
     self.0 |= value << 14;
     self
  }

}
impl ::core::fmt::Display for Enupd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Enupd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_enupd_enupd0() != 0 { try!(write!(f, " pwm_enupd_enupd0=0x{:x}", self.pwm_enupd_enupd0()))}
      if self.pwm_enupd_enupd1() != 0 { try!(write!(f, " pwm_enupd_enupd1=0x{:x}", self.pwm_enupd_enupd1()))}
      if self.pwm_enupd_enupd2() != 0 { try!(write!(f, " pwm_enupd_enupd2=0x{:x}", self.pwm_enupd_enupd2()))}
      if self.pwm_enupd_enupd3() != 0 { try!(write!(f, " pwm_enupd_enupd3=0x{:x}", self.pwm_enupd_enupd3()))}
      if self.pwm_enupd_enupd4() != 0 { try!(write!(f, " pwm_enupd_enupd4=0x{:x}", self.pwm_enupd_enupd4()))}
      if self.pwm_enupd_enupd5() != 0 { try!(write!(f, " pwm_enupd_enupd5=0x{:x}", self.pwm_enupd_enupd5()))}
      if self.pwm_enupd_enupd6() != 0 { try!(write!(f, " pwm_enupd_enupd6=0x{:x}", self.pwm_enupd_enupd6()))}
      if self.pwm_enupd_enupd7() != 0 { try!(write!(f, " pwm_enupd_enupd7=0x{:x}", self.pwm_enupd_enupd7()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _0Ctl(pub u32);
impl _0Ctl {
  #[inline] pub fn pwm_0_ctl_enable(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_0_ctl_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_0_ctl_mode(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_0_ctl_mode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_0_ctl_debug(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_0_ctl_debug(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_0_ctl_loadupd(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_0_ctl_loadupd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_0_ctl_cmpaupd(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_0_ctl_cmpaupd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_0_ctl_cmpbupd(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_0_ctl_cmpbupd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn pwm_0_ctl_genaupd(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
  #[inline] pub fn set_pwm_0_ctl_genaupd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn pwm_0_ctl_genbupd(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  #[inline] pub fn set_pwm_0_ctl_genbupd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn pwm_0_ctl_dbctlupd(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
  #[inline] pub fn set_pwm_0_ctl_dbctlupd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn pwm_0_ctl_dbriseupd(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x3 // [13:12]
  }
  #[inline] pub fn set_pwm_0_ctl_dbriseupd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn pwm_0_ctl_dbfallupd(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x3 // [15:14]
  }
  #[inline] pub fn set_pwm_0_ctl_dbfallupd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline] pub fn pwm_0_ctl_fltsrc(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline] pub fn set_pwm_0_ctl_fltsrc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline] pub fn pwm_0_ctl_minfltper(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline] pub fn set_pwm_0_ctl_minfltper(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline] pub fn pwm_0_ctl_latch(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline] pub fn set_pwm_0_ctl_latch(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

}
impl ::core::fmt::Display for _0Ctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _0Ctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_0_ctl_enable() != 0 { try!(write!(f, " pwm_0_ctl_enable"))}
      if self.pwm_0_ctl_mode() != 0 { try!(write!(f, " pwm_0_ctl_mode"))}
      if self.pwm_0_ctl_debug() != 0 { try!(write!(f, " pwm_0_ctl_debug"))}
      if self.pwm_0_ctl_loadupd() != 0 { try!(write!(f, " pwm_0_ctl_loadupd"))}
      if self.pwm_0_ctl_cmpaupd() != 0 { try!(write!(f, " pwm_0_ctl_cmpaupd"))}
      if self.pwm_0_ctl_cmpbupd() != 0 { try!(write!(f, " pwm_0_ctl_cmpbupd"))}
      if self.pwm_0_ctl_genaupd() != 0 { try!(write!(f, " pwm_0_ctl_genaupd=0x{:x}", self.pwm_0_ctl_genaupd()))}
      if self.pwm_0_ctl_genbupd() != 0 { try!(write!(f, " pwm_0_ctl_genbupd=0x{:x}", self.pwm_0_ctl_genbupd()))}
      if self.pwm_0_ctl_dbctlupd() != 0 { try!(write!(f, " pwm_0_ctl_dbctlupd=0x{:x}", self.pwm_0_ctl_dbctlupd()))}
      if self.pwm_0_ctl_dbriseupd() != 0 { try!(write!(f, " pwm_0_ctl_dbriseupd=0x{:x}", self.pwm_0_ctl_dbriseupd()))}
      if self.pwm_0_ctl_dbfallupd() != 0 { try!(write!(f, " pwm_0_ctl_dbfallupd=0x{:x}", self.pwm_0_ctl_dbfallupd()))}
      if self.pwm_0_ctl_fltsrc() != 0 { try!(write!(f, " pwm_0_ctl_fltsrc"))}
      if self.pwm_0_ctl_minfltper() != 0 { try!(write!(f, " pwm_0_ctl_minfltper"))}
      if self.pwm_0_ctl_latch() != 0 { try!(write!(f, " pwm_0_ctl_latch"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _0Inten(pub u32);
impl _0Inten {
  #[inline] pub fn pwm_0_inten_intcntzero(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_0_inten_intcntzero(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_0_inten_intcntload(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_0_inten_intcntload(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_0_inten_intcmpau(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_0_inten_intcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_0_inten_intcmpad(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_0_inten_intcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_0_inten_intcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_0_inten_intcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_0_inten_intcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_0_inten_intcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn pwm_0_inten_trcntzero(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_pwm_0_inten_trcntzero(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn pwm_0_inten_trcntload(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_pwm_0_inten_trcntload(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn pwm_0_inten_trcmpau(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_pwm_0_inten_trcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn pwm_0_inten_trcmpad(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_pwm_0_inten_trcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline] pub fn pwm_0_inten_trcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline] pub fn set_pwm_0_inten_trcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn pwm_0_inten_trcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline] pub fn set_pwm_0_inten_trcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

}
impl ::core::fmt::Display for _0Inten {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _0Inten {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_0_inten_intcntzero() != 0 { try!(write!(f, " pwm_0_inten_intcntzero"))}
      if self.pwm_0_inten_intcntload() != 0 { try!(write!(f, " pwm_0_inten_intcntload"))}
      if self.pwm_0_inten_intcmpau() != 0 { try!(write!(f, " pwm_0_inten_intcmpau"))}
      if self.pwm_0_inten_intcmpad() != 0 { try!(write!(f, " pwm_0_inten_intcmpad"))}
      if self.pwm_0_inten_intcmpbu() != 0 { try!(write!(f, " pwm_0_inten_intcmpbu"))}
      if self.pwm_0_inten_intcmpbd() != 0 { try!(write!(f, " pwm_0_inten_intcmpbd"))}
      if self.pwm_0_inten_trcntzero() != 0 { try!(write!(f, " pwm_0_inten_trcntzero"))}
      if self.pwm_0_inten_trcntload() != 0 { try!(write!(f, " pwm_0_inten_trcntload"))}
      if self.pwm_0_inten_trcmpau() != 0 { try!(write!(f, " pwm_0_inten_trcmpau"))}
      if self.pwm_0_inten_trcmpad() != 0 { try!(write!(f, " pwm_0_inten_trcmpad"))}
      if self.pwm_0_inten_trcmpbu() != 0 { try!(write!(f, " pwm_0_inten_trcmpbu"))}
      if self.pwm_0_inten_trcmpbd() != 0 { try!(write!(f, " pwm_0_inten_trcmpbd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _0Ris(pub u32);
impl _0Ris {
  #[inline] pub fn pwm_0_ris_intcntzero(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_0_ris_intcntzero(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_0_ris_intcntload(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_0_ris_intcntload(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_0_ris_intcmpau(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_0_ris_intcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_0_ris_intcmpad(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_0_ris_intcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_0_ris_intcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_0_ris_intcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_0_ris_intcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_0_ris_intcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for _0Ris {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _0Ris {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_0_ris_intcntzero() != 0 { try!(write!(f, " pwm_0_ris_intcntzero"))}
      if self.pwm_0_ris_intcntload() != 0 { try!(write!(f, " pwm_0_ris_intcntload"))}
      if self.pwm_0_ris_intcmpau() != 0 { try!(write!(f, " pwm_0_ris_intcmpau"))}
      if self.pwm_0_ris_intcmpad() != 0 { try!(write!(f, " pwm_0_ris_intcmpad"))}
      if self.pwm_0_ris_intcmpbu() != 0 { try!(write!(f, " pwm_0_ris_intcmpbu"))}
      if self.pwm_0_ris_intcmpbd() != 0 { try!(write!(f, " pwm_0_ris_intcmpbd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _0Isc(pub u32);
impl _0Isc {
  #[inline] pub fn pwm_0_isc_intcntzero(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_0_isc_intcntzero(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_0_isc_intcntload(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_0_isc_intcntload(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_0_isc_intcmpau(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_0_isc_intcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_0_isc_intcmpad(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_0_isc_intcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_0_isc_intcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_0_isc_intcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_0_isc_intcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_0_isc_intcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for _0Isc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _0Isc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_0_isc_intcntzero() != 0 { try!(write!(f, " pwm_0_isc_intcntzero"))}
      if self.pwm_0_isc_intcntload() != 0 { try!(write!(f, " pwm_0_isc_intcntload"))}
      if self.pwm_0_isc_intcmpau() != 0 { try!(write!(f, " pwm_0_isc_intcmpau"))}
      if self.pwm_0_isc_intcmpad() != 0 { try!(write!(f, " pwm_0_isc_intcmpad"))}
      if self.pwm_0_isc_intcmpbu() != 0 { try!(write!(f, " pwm_0_isc_intcmpbu"))}
      if self.pwm_0_isc_intcmpbd() != 0 { try!(write!(f, " pwm_0_isc_intcmpbd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _0Load(pub u32);
impl _0Load {
  #[inline] pub fn pwm_0_load(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_pwm_0_load(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _0Load {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _0Load {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_0_load() != 0 { try!(write!(f, " pwm_0_load=0x{:x}", self.pwm_0_load()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _0Count(pub u32);
impl _0Count {
  #[inline] pub fn pwm_0_count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_pwm_0_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _0Count {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _0Count {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_0_count() != 0 { try!(write!(f, " pwm_0_count=0x{:x}", self.pwm_0_count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _0Cmpa(pub u32);
impl _0Cmpa {
  #[inline] pub fn pwm_0_cmpa(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_pwm_0_cmpa(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _0Cmpa {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _0Cmpa {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_0_cmpa() != 0 { try!(write!(f, " pwm_0_cmpa=0x{:x}", self.pwm_0_cmpa()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _0Cmpb(pub u32);
impl _0Cmpb {
  #[inline] pub fn pwm_0_cmpb(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_pwm_0_cmpb(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _0Cmpb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _0Cmpb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_0_cmpb() != 0 { try!(write!(f, " pwm_0_cmpb=0x{:x}", self.pwm_0_cmpb()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _0Gena(pub u32);
impl _0Gena {
  #[inline] pub fn pwm_0_gena_actzero(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline] pub fn set_pwm_0_gena_actzero(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_0_gena_actload(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
  #[inline] pub fn set_pwm_0_gena_actload(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_0_gena_actcmpau(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  #[inline] pub fn set_pwm_0_gena_actcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_0_gena_actcmpad(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
  #[inline] pub fn set_pwm_0_gena_actcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn pwm_0_gena_actcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  #[inline] pub fn set_pwm_0_gena_actcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn pwm_0_gena_actcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
  #[inline] pub fn set_pwm_0_gena_actcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

}
impl ::core::fmt::Display for _0Gena {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _0Gena {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_0_gena_actzero() != 0 { try!(write!(f, " pwm_0_gena_actzero=0x{:x}", self.pwm_0_gena_actzero()))}
      if self.pwm_0_gena_actload() != 0 { try!(write!(f, " pwm_0_gena_actload=0x{:x}", self.pwm_0_gena_actload()))}
      if self.pwm_0_gena_actcmpau() != 0 { try!(write!(f, " pwm_0_gena_actcmpau=0x{:x}", self.pwm_0_gena_actcmpau()))}
      if self.pwm_0_gena_actcmpad() != 0 { try!(write!(f, " pwm_0_gena_actcmpad=0x{:x}", self.pwm_0_gena_actcmpad()))}
      if self.pwm_0_gena_actcmpbu() != 0 { try!(write!(f, " pwm_0_gena_actcmpbu=0x{:x}", self.pwm_0_gena_actcmpbu()))}
      if self.pwm_0_gena_actcmpbd() != 0 { try!(write!(f, " pwm_0_gena_actcmpbd=0x{:x}", self.pwm_0_gena_actcmpbd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _0Genb(pub u32);
impl _0Genb {
  #[inline] pub fn pwm_0_genb_actzero(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline] pub fn set_pwm_0_genb_actzero(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_0_genb_actload(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
  #[inline] pub fn set_pwm_0_genb_actload(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_0_genb_actcmpau(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  #[inline] pub fn set_pwm_0_genb_actcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_0_genb_actcmpad(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
  #[inline] pub fn set_pwm_0_genb_actcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn pwm_0_genb_actcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  #[inline] pub fn set_pwm_0_genb_actcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn pwm_0_genb_actcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
  #[inline] pub fn set_pwm_0_genb_actcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

}
impl ::core::fmt::Display for _0Genb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _0Genb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_0_genb_actzero() != 0 { try!(write!(f, " pwm_0_genb_actzero=0x{:x}", self.pwm_0_genb_actzero()))}
      if self.pwm_0_genb_actload() != 0 { try!(write!(f, " pwm_0_genb_actload=0x{:x}", self.pwm_0_genb_actload()))}
      if self.pwm_0_genb_actcmpau() != 0 { try!(write!(f, " pwm_0_genb_actcmpau=0x{:x}", self.pwm_0_genb_actcmpau()))}
      if self.pwm_0_genb_actcmpad() != 0 { try!(write!(f, " pwm_0_genb_actcmpad=0x{:x}", self.pwm_0_genb_actcmpad()))}
      if self.pwm_0_genb_actcmpbu() != 0 { try!(write!(f, " pwm_0_genb_actcmpbu=0x{:x}", self.pwm_0_genb_actcmpbu()))}
      if self.pwm_0_genb_actcmpbd() != 0 { try!(write!(f, " pwm_0_genb_actcmpbd=0x{:x}", self.pwm_0_genb_actcmpbd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _0Dbctl(pub u32);
impl _0Dbctl {
  #[inline] pub fn pwm_0_dbctl_enable(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_0_dbctl_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _0Dbctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _0Dbctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_0_dbctl_enable() != 0 { try!(write!(f, " pwm_0_dbctl_enable"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _0Dbrise(pub u32);
impl _0Dbrise {
  #[inline] pub fn pwm_0_dbrise_delay(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  #[inline] pub fn set_pwm_0_dbrise_delay(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _0Dbrise {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _0Dbrise {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_0_dbrise_delay() != 0 { try!(write!(f, " pwm_0_dbrise_delay=0x{:x}", self.pwm_0_dbrise_delay()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _0Dbfall(pub u32);
impl _0Dbfall {
  #[inline] pub fn pwm_0_dbfall_delay(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  #[inline] pub fn set_pwm_0_dbfall_delay(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _0Dbfall {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _0Dbfall {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_0_dbfall_delay() != 0 { try!(write!(f, " pwm_0_dbfall_delay=0x{:x}", self.pwm_0_dbfall_delay()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _0Fltsrc0(pub u32);
impl _0Fltsrc0 {
  #[inline] pub fn pwm_0_fltsrc0_fault0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_0_fltsrc0_fault0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_0_fltsrc0_fault1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_0_fltsrc0_fault1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_0_fltsrc0_fault2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_0_fltsrc0_fault2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_0_fltsrc0_fault3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_0_fltsrc0_fault3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}
impl ::core::fmt::Display for _0Fltsrc0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _0Fltsrc0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_0_fltsrc0_fault0() != 0 { try!(write!(f, " pwm_0_fltsrc0_fault0"))}
      if self.pwm_0_fltsrc0_fault1() != 0 { try!(write!(f, " pwm_0_fltsrc0_fault1"))}
      if self.pwm_0_fltsrc0_fault2() != 0 { try!(write!(f, " pwm_0_fltsrc0_fault2"))}
      if self.pwm_0_fltsrc0_fault3() != 0 { try!(write!(f, " pwm_0_fltsrc0_fault3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _0Fltsrc1(pub u32);
impl _0Fltsrc1 {
  #[inline] pub fn pwm_0_fltsrc1_dcmp0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_0_fltsrc1_dcmp0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_0_fltsrc1_dcmp1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_0_fltsrc1_dcmp1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_0_fltsrc1_dcmp2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_0_fltsrc1_dcmp2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_0_fltsrc1_dcmp3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_0_fltsrc1_dcmp3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_0_fltsrc1_dcmp4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_0_fltsrc1_dcmp4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_0_fltsrc1_dcmp5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_0_fltsrc1_dcmp5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn pwm_0_fltsrc1_dcmp6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_pwm_0_fltsrc1_dcmp6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn pwm_0_fltsrc1_dcmp7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_pwm_0_fltsrc1_dcmp7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for _0Fltsrc1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _0Fltsrc1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_0_fltsrc1_dcmp0() != 0 { try!(write!(f, " pwm_0_fltsrc1_dcmp0"))}
      if self.pwm_0_fltsrc1_dcmp1() != 0 { try!(write!(f, " pwm_0_fltsrc1_dcmp1"))}
      if self.pwm_0_fltsrc1_dcmp2() != 0 { try!(write!(f, " pwm_0_fltsrc1_dcmp2"))}
      if self.pwm_0_fltsrc1_dcmp3() != 0 { try!(write!(f, " pwm_0_fltsrc1_dcmp3"))}
      if self.pwm_0_fltsrc1_dcmp4() != 0 { try!(write!(f, " pwm_0_fltsrc1_dcmp4"))}
      if self.pwm_0_fltsrc1_dcmp5() != 0 { try!(write!(f, " pwm_0_fltsrc1_dcmp5"))}
      if self.pwm_0_fltsrc1_dcmp6() != 0 { try!(write!(f, " pwm_0_fltsrc1_dcmp6"))}
      if self.pwm_0_fltsrc1_dcmp7() != 0 { try!(write!(f, " pwm_0_fltsrc1_dcmp7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _0Minfltper(pub u32);
impl _0Minfltper {
  #[inline] pub fn pwm_0_minfltper(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_pwm_0_minfltper(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _0Minfltper {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _0Minfltper {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_0_minfltper() != 0 { try!(write!(f, " pwm_0_minfltper=0x{:x}", self.pwm_0_minfltper()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _1Ctl(pub u32);
impl _1Ctl {
  #[inline] pub fn pwm_1_ctl_enable(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_1_ctl_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_1_ctl_mode(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_1_ctl_mode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_1_ctl_debug(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_1_ctl_debug(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_1_ctl_loadupd(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_1_ctl_loadupd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_1_ctl_cmpaupd(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_1_ctl_cmpaupd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_1_ctl_cmpbupd(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_1_ctl_cmpbupd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn pwm_1_ctl_genaupd(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
  #[inline] pub fn set_pwm_1_ctl_genaupd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn pwm_1_ctl_genbupd(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  #[inline] pub fn set_pwm_1_ctl_genbupd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn pwm_1_ctl_dbctlupd(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
  #[inline] pub fn set_pwm_1_ctl_dbctlupd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn pwm_1_ctl_dbriseupd(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x3 // [13:12]
  }
  #[inline] pub fn set_pwm_1_ctl_dbriseupd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn pwm_1_ctl_dbfallupd(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x3 // [15:14]
  }
  #[inline] pub fn set_pwm_1_ctl_dbfallupd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline] pub fn pwm_1_ctl_fltsrc(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline] pub fn set_pwm_1_ctl_fltsrc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline] pub fn pwm_1_ctl_minfltper(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline] pub fn set_pwm_1_ctl_minfltper(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline] pub fn pwm_1_ctl_latch(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline] pub fn set_pwm_1_ctl_latch(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

}
impl ::core::fmt::Display for _1Ctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _1Ctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_1_ctl_enable() != 0 { try!(write!(f, " pwm_1_ctl_enable"))}
      if self.pwm_1_ctl_mode() != 0 { try!(write!(f, " pwm_1_ctl_mode"))}
      if self.pwm_1_ctl_debug() != 0 { try!(write!(f, " pwm_1_ctl_debug"))}
      if self.pwm_1_ctl_loadupd() != 0 { try!(write!(f, " pwm_1_ctl_loadupd"))}
      if self.pwm_1_ctl_cmpaupd() != 0 { try!(write!(f, " pwm_1_ctl_cmpaupd"))}
      if self.pwm_1_ctl_cmpbupd() != 0 { try!(write!(f, " pwm_1_ctl_cmpbupd"))}
      if self.pwm_1_ctl_genaupd() != 0 { try!(write!(f, " pwm_1_ctl_genaupd=0x{:x}", self.pwm_1_ctl_genaupd()))}
      if self.pwm_1_ctl_genbupd() != 0 { try!(write!(f, " pwm_1_ctl_genbupd=0x{:x}", self.pwm_1_ctl_genbupd()))}
      if self.pwm_1_ctl_dbctlupd() != 0 { try!(write!(f, " pwm_1_ctl_dbctlupd=0x{:x}", self.pwm_1_ctl_dbctlupd()))}
      if self.pwm_1_ctl_dbriseupd() != 0 { try!(write!(f, " pwm_1_ctl_dbriseupd=0x{:x}", self.pwm_1_ctl_dbriseupd()))}
      if self.pwm_1_ctl_dbfallupd() != 0 { try!(write!(f, " pwm_1_ctl_dbfallupd=0x{:x}", self.pwm_1_ctl_dbfallupd()))}
      if self.pwm_1_ctl_fltsrc() != 0 { try!(write!(f, " pwm_1_ctl_fltsrc"))}
      if self.pwm_1_ctl_minfltper() != 0 { try!(write!(f, " pwm_1_ctl_minfltper"))}
      if self.pwm_1_ctl_latch() != 0 { try!(write!(f, " pwm_1_ctl_latch"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _1Inten(pub u32);
impl _1Inten {
  #[inline] pub fn pwm_1_inten_intcntzero(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_1_inten_intcntzero(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_1_inten_intcntload(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_1_inten_intcntload(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_1_inten_intcmpau(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_1_inten_intcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_1_inten_intcmpad(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_1_inten_intcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_1_inten_intcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_1_inten_intcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_1_inten_intcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_1_inten_intcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn pwm_1_inten_trcntzero(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_pwm_1_inten_trcntzero(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn pwm_1_inten_trcntload(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_pwm_1_inten_trcntload(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn pwm_1_inten_trcmpau(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_pwm_1_inten_trcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn pwm_1_inten_trcmpad(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_pwm_1_inten_trcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline] pub fn pwm_1_inten_trcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline] pub fn set_pwm_1_inten_trcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn pwm_1_inten_trcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline] pub fn set_pwm_1_inten_trcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

}
impl ::core::fmt::Display for _1Inten {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _1Inten {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_1_inten_intcntzero() != 0 { try!(write!(f, " pwm_1_inten_intcntzero"))}
      if self.pwm_1_inten_intcntload() != 0 { try!(write!(f, " pwm_1_inten_intcntload"))}
      if self.pwm_1_inten_intcmpau() != 0 { try!(write!(f, " pwm_1_inten_intcmpau"))}
      if self.pwm_1_inten_intcmpad() != 0 { try!(write!(f, " pwm_1_inten_intcmpad"))}
      if self.pwm_1_inten_intcmpbu() != 0 { try!(write!(f, " pwm_1_inten_intcmpbu"))}
      if self.pwm_1_inten_intcmpbd() != 0 { try!(write!(f, " pwm_1_inten_intcmpbd"))}
      if self.pwm_1_inten_trcntzero() != 0 { try!(write!(f, " pwm_1_inten_trcntzero"))}
      if self.pwm_1_inten_trcntload() != 0 { try!(write!(f, " pwm_1_inten_trcntload"))}
      if self.pwm_1_inten_trcmpau() != 0 { try!(write!(f, " pwm_1_inten_trcmpau"))}
      if self.pwm_1_inten_trcmpad() != 0 { try!(write!(f, " pwm_1_inten_trcmpad"))}
      if self.pwm_1_inten_trcmpbu() != 0 { try!(write!(f, " pwm_1_inten_trcmpbu"))}
      if self.pwm_1_inten_trcmpbd() != 0 { try!(write!(f, " pwm_1_inten_trcmpbd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _1Ris(pub u32);
impl _1Ris {
  #[inline] pub fn pwm_1_ris_intcntzero(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_1_ris_intcntzero(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_1_ris_intcntload(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_1_ris_intcntload(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_1_ris_intcmpau(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_1_ris_intcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_1_ris_intcmpad(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_1_ris_intcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_1_ris_intcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_1_ris_intcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_1_ris_intcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_1_ris_intcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for _1Ris {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _1Ris {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_1_ris_intcntzero() != 0 { try!(write!(f, " pwm_1_ris_intcntzero"))}
      if self.pwm_1_ris_intcntload() != 0 { try!(write!(f, " pwm_1_ris_intcntload"))}
      if self.pwm_1_ris_intcmpau() != 0 { try!(write!(f, " pwm_1_ris_intcmpau"))}
      if self.pwm_1_ris_intcmpad() != 0 { try!(write!(f, " pwm_1_ris_intcmpad"))}
      if self.pwm_1_ris_intcmpbu() != 0 { try!(write!(f, " pwm_1_ris_intcmpbu"))}
      if self.pwm_1_ris_intcmpbd() != 0 { try!(write!(f, " pwm_1_ris_intcmpbd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _1Isc(pub u32);
impl _1Isc {
  #[inline] pub fn pwm_1_isc_intcntzero(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_1_isc_intcntzero(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_1_isc_intcntload(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_1_isc_intcntload(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_1_isc_intcmpau(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_1_isc_intcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_1_isc_intcmpad(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_1_isc_intcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_1_isc_intcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_1_isc_intcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_1_isc_intcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_1_isc_intcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for _1Isc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _1Isc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_1_isc_intcntzero() != 0 { try!(write!(f, " pwm_1_isc_intcntzero"))}
      if self.pwm_1_isc_intcntload() != 0 { try!(write!(f, " pwm_1_isc_intcntload"))}
      if self.pwm_1_isc_intcmpau() != 0 { try!(write!(f, " pwm_1_isc_intcmpau"))}
      if self.pwm_1_isc_intcmpad() != 0 { try!(write!(f, " pwm_1_isc_intcmpad"))}
      if self.pwm_1_isc_intcmpbu() != 0 { try!(write!(f, " pwm_1_isc_intcmpbu"))}
      if self.pwm_1_isc_intcmpbd() != 0 { try!(write!(f, " pwm_1_isc_intcmpbd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _1Load(pub u32);
impl _1Load {
  #[inline] pub fn pwm_1_load_load(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_pwm_1_load_load(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _1Load {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _1Load {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_1_load_load() != 0 { try!(write!(f, " pwm_1_load_load=0x{:x}", self.pwm_1_load_load()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _1Count(pub u32);
impl _1Count {
  #[inline] pub fn pwm_1_count_count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_pwm_1_count_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _1Count {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _1Count {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_1_count_count() != 0 { try!(write!(f, " pwm_1_count_count=0x{:x}", self.pwm_1_count_count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _1Cmpa(pub u32);
impl _1Cmpa {
  #[inline] pub fn pwm_1_cmpa_compa(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_pwm_1_cmpa_compa(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _1Cmpa {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _1Cmpa {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_1_cmpa_compa() != 0 { try!(write!(f, " pwm_1_cmpa_compa=0x{:x}", self.pwm_1_cmpa_compa()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _1Cmpb(pub u32);
impl _1Cmpb {
  #[inline] pub fn pwm_1_cmpb_compb(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_pwm_1_cmpb_compb(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _1Cmpb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _1Cmpb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_1_cmpb_compb() != 0 { try!(write!(f, " pwm_1_cmpb_compb=0x{:x}", self.pwm_1_cmpb_compb()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _1Gena(pub u32);
impl _1Gena {
  #[inline] pub fn pwm_1_gena_actzero(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline] pub fn set_pwm_1_gena_actzero(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_1_gena_actload(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
  #[inline] pub fn set_pwm_1_gena_actload(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_1_gena_actcmpau(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  #[inline] pub fn set_pwm_1_gena_actcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_1_gena_actcmpad(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
  #[inline] pub fn set_pwm_1_gena_actcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn pwm_1_gena_actcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  #[inline] pub fn set_pwm_1_gena_actcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn pwm_1_gena_actcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
  #[inline] pub fn set_pwm_1_gena_actcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

}
impl ::core::fmt::Display for _1Gena {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _1Gena {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_1_gena_actzero() != 0 { try!(write!(f, " pwm_1_gena_actzero=0x{:x}", self.pwm_1_gena_actzero()))}
      if self.pwm_1_gena_actload() != 0 { try!(write!(f, " pwm_1_gena_actload=0x{:x}", self.pwm_1_gena_actload()))}
      if self.pwm_1_gena_actcmpau() != 0 { try!(write!(f, " pwm_1_gena_actcmpau=0x{:x}", self.pwm_1_gena_actcmpau()))}
      if self.pwm_1_gena_actcmpad() != 0 { try!(write!(f, " pwm_1_gena_actcmpad=0x{:x}", self.pwm_1_gena_actcmpad()))}
      if self.pwm_1_gena_actcmpbu() != 0 { try!(write!(f, " pwm_1_gena_actcmpbu=0x{:x}", self.pwm_1_gena_actcmpbu()))}
      if self.pwm_1_gena_actcmpbd() != 0 { try!(write!(f, " pwm_1_gena_actcmpbd=0x{:x}", self.pwm_1_gena_actcmpbd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _1Genb(pub u32);
impl _1Genb {
  #[inline] pub fn pwm_1_genb_actzero(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline] pub fn set_pwm_1_genb_actzero(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_1_genb_actload(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
  #[inline] pub fn set_pwm_1_genb_actload(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_1_genb_actcmpau(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  #[inline] pub fn set_pwm_1_genb_actcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_1_genb_actcmpad(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
  #[inline] pub fn set_pwm_1_genb_actcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn pwm_1_genb_actcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  #[inline] pub fn set_pwm_1_genb_actcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn pwm_1_genb_actcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
  #[inline] pub fn set_pwm_1_genb_actcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

}
impl ::core::fmt::Display for _1Genb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _1Genb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_1_genb_actzero() != 0 { try!(write!(f, " pwm_1_genb_actzero=0x{:x}", self.pwm_1_genb_actzero()))}
      if self.pwm_1_genb_actload() != 0 { try!(write!(f, " pwm_1_genb_actload=0x{:x}", self.pwm_1_genb_actload()))}
      if self.pwm_1_genb_actcmpau() != 0 { try!(write!(f, " pwm_1_genb_actcmpau=0x{:x}", self.pwm_1_genb_actcmpau()))}
      if self.pwm_1_genb_actcmpad() != 0 { try!(write!(f, " pwm_1_genb_actcmpad=0x{:x}", self.pwm_1_genb_actcmpad()))}
      if self.pwm_1_genb_actcmpbu() != 0 { try!(write!(f, " pwm_1_genb_actcmpbu=0x{:x}", self.pwm_1_genb_actcmpbu()))}
      if self.pwm_1_genb_actcmpbd() != 0 { try!(write!(f, " pwm_1_genb_actcmpbd=0x{:x}", self.pwm_1_genb_actcmpbd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _1Dbctl(pub u32);
impl _1Dbctl {
  #[inline] pub fn pwm_1_dbctl_enable(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_1_dbctl_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _1Dbctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _1Dbctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_1_dbctl_enable() != 0 { try!(write!(f, " pwm_1_dbctl_enable"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _1Dbrise(pub u32);
impl _1Dbrise {
  #[inline] pub fn pwm_1_dbrise_risedelay(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  #[inline] pub fn set_pwm_1_dbrise_risedelay(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _1Dbrise {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _1Dbrise {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_1_dbrise_risedelay() != 0 { try!(write!(f, " pwm_1_dbrise_risedelay=0x{:x}", self.pwm_1_dbrise_risedelay()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _1Dbfall(pub u32);
impl _1Dbfall {
  #[inline] pub fn pwm_1_dbfall_falldelay(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  #[inline] pub fn set_pwm_1_dbfall_falldelay(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _1Dbfall {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _1Dbfall {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_1_dbfall_falldelay() != 0 { try!(write!(f, " pwm_1_dbfall_falldelay=0x{:x}", self.pwm_1_dbfall_falldelay()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _1Fltsrc0(pub u32);
impl _1Fltsrc0 {
  #[inline] pub fn pwm_1_fltsrc0_fault0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_1_fltsrc0_fault0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_1_fltsrc0_fault1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_1_fltsrc0_fault1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_1_fltsrc0_fault2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_1_fltsrc0_fault2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_1_fltsrc0_fault3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_1_fltsrc0_fault3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}
impl ::core::fmt::Display for _1Fltsrc0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _1Fltsrc0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_1_fltsrc0_fault0() != 0 { try!(write!(f, " pwm_1_fltsrc0_fault0"))}
      if self.pwm_1_fltsrc0_fault1() != 0 { try!(write!(f, " pwm_1_fltsrc0_fault1"))}
      if self.pwm_1_fltsrc0_fault2() != 0 { try!(write!(f, " pwm_1_fltsrc0_fault2"))}
      if self.pwm_1_fltsrc0_fault3() != 0 { try!(write!(f, " pwm_1_fltsrc0_fault3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _1Fltsrc1(pub u32);
impl _1Fltsrc1 {
  #[inline] pub fn pwm_1_fltsrc1_dcmp0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_1_fltsrc1_dcmp0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_1_fltsrc1_dcmp1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_1_fltsrc1_dcmp1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_1_fltsrc1_dcmp2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_1_fltsrc1_dcmp2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_1_fltsrc1_dcmp3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_1_fltsrc1_dcmp3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_1_fltsrc1_dcmp4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_1_fltsrc1_dcmp4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_1_fltsrc1_dcmp5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_1_fltsrc1_dcmp5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn pwm_1_fltsrc1_dcmp6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_pwm_1_fltsrc1_dcmp6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn pwm_1_fltsrc1_dcmp7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_pwm_1_fltsrc1_dcmp7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for _1Fltsrc1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _1Fltsrc1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_1_fltsrc1_dcmp0() != 0 { try!(write!(f, " pwm_1_fltsrc1_dcmp0"))}
      if self.pwm_1_fltsrc1_dcmp1() != 0 { try!(write!(f, " pwm_1_fltsrc1_dcmp1"))}
      if self.pwm_1_fltsrc1_dcmp2() != 0 { try!(write!(f, " pwm_1_fltsrc1_dcmp2"))}
      if self.pwm_1_fltsrc1_dcmp3() != 0 { try!(write!(f, " pwm_1_fltsrc1_dcmp3"))}
      if self.pwm_1_fltsrc1_dcmp4() != 0 { try!(write!(f, " pwm_1_fltsrc1_dcmp4"))}
      if self.pwm_1_fltsrc1_dcmp5() != 0 { try!(write!(f, " pwm_1_fltsrc1_dcmp5"))}
      if self.pwm_1_fltsrc1_dcmp6() != 0 { try!(write!(f, " pwm_1_fltsrc1_dcmp6"))}
      if self.pwm_1_fltsrc1_dcmp7() != 0 { try!(write!(f, " pwm_1_fltsrc1_dcmp7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _1Minfltper(pub u32);
impl _1Minfltper {
  #[inline] pub fn pwm_1_minfltper_mfp(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_pwm_1_minfltper_mfp(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _1Minfltper {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _1Minfltper {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_1_minfltper_mfp() != 0 { try!(write!(f, " pwm_1_minfltper_mfp=0x{:x}", self.pwm_1_minfltper_mfp()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _2Ctl(pub u32);
impl _2Ctl {
  #[inline] pub fn pwm_2_ctl_enable(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_2_ctl_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_2_ctl_mode(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_2_ctl_mode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_2_ctl_debug(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_2_ctl_debug(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_2_ctl_loadupd(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_2_ctl_loadupd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_2_ctl_cmpaupd(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_2_ctl_cmpaupd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_2_ctl_cmpbupd(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_2_ctl_cmpbupd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn pwm_2_ctl_genaupd(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
  #[inline] pub fn set_pwm_2_ctl_genaupd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn pwm_2_ctl_genbupd(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  #[inline] pub fn set_pwm_2_ctl_genbupd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn pwm_2_ctl_dbctlupd(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
  #[inline] pub fn set_pwm_2_ctl_dbctlupd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn pwm_2_ctl_dbriseupd(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x3 // [13:12]
  }
  #[inline] pub fn set_pwm_2_ctl_dbriseupd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn pwm_2_ctl_dbfallupd(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x3 // [15:14]
  }
  #[inline] pub fn set_pwm_2_ctl_dbfallupd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline] pub fn pwm_2_ctl_fltsrc(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline] pub fn set_pwm_2_ctl_fltsrc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline] pub fn pwm_2_ctl_minfltper(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline] pub fn set_pwm_2_ctl_minfltper(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline] pub fn pwm_2_ctl_latch(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline] pub fn set_pwm_2_ctl_latch(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

}
impl ::core::fmt::Display for _2Ctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _2Ctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_2_ctl_enable() != 0 { try!(write!(f, " pwm_2_ctl_enable"))}
      if self.pwm_2_ctl_mode() != 0 { try!(write!(f, " pwm_2_ctl_mode"))}
      if self.pwm_2_ctl_debug() != 0 { try!(write!(f, " pwm_2_ctl_debug"))}
      if self.pwm_2_ctl_loadupd() != 0 { try!(write!(f, " pwm_2_ctl_loadupd"))}
      if self.pwm_2_ctl_cmpaupd() != 0 { try!(write!(f, " pwm_2_ctl_cmpaupd"))}
      if self.pwm_2_ctl_cmpbupd() != 0 { try!(write!(f, " pwm_2_ctl_cmpbupd"))}
      if self.pwm_2_ctl_genaupd() != 0 { try!(write!(f, " pwm_2_ctl_genaupd=0x{:x}", self.pwm_2_ctl_genaupd()))}
      if self.pwm_2_ctl_genbupd() != 0 { try!(write!(f, " pwm_2_ctl_genbupd=0x{:x}", self.pwm_2_ctl_genbupd()))}
      if self.pwm_2_ctl_dbctlupd() != 0 { try!(write!(f, " pwm_2_ctl_dbctlupd=0x{:x}", self.pwm_2_ctl_dbctlupd()))}
      if self.pwm_2_ctl_dbriseupd() != 0 { try!(write!(f, " pwm_2_ctl_dbriseupd=0x{:x}", self.pwm_2_ctl_dbriseupd()))}
      if self.pwm_2_ctl_dbfallupd() != 0 { try!(write!(f, " pwm_2_ctl_dbfallupd=0x{:x}", self.pwm_2_ctl_dbfallupd()))}
      if self.pwm_2_ctl_fltsrc() != 0 { try!(write!(f, " pwm_2_ctl_fltsrc"))}
      if self.pwm_2_ctl_minfltper() != 0 { try!(write!(f, " pwm_2_ctl_minfltper"))}
      if self.pwm_2_ctl_latch() != 0 { try!(write!(f, " pwm_2_ctl_latch"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _2Inten(pub u32);
impl _2Inten {
  #[inline] pub fn pwm_2_inten_intcntzero(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_2_inten_intcntzero(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_2_inten_intcntload(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_2_inten_intcntload(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_2_inten_intcmpau(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_2_inten_intcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_2_inten_intcmpad(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_2_inten_intcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_2_inten_intcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_2_inten_intcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_2_inten_intcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_2_inten_intcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn pwm_2_inten_trcntzero(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_pwm_2_inten_trcntzero(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn pwm_2_inten_trcntload(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_pwm_2_inten_trcntload(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn pwm_2_inten_trcmpau(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_pwm_2_inten_trcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn pwm_2_inten_trcmpad(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_pwm_2_inten_trcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline] pub fn pwm_2_inten_trcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline] pub fn set_pwm_2_inten_trcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn pwm_2_inten_trcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline] pub fn set_pwm_2_inten_trcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

}
impl ::core::fmt::Display for _2Inten {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _2Inten {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_2_inten_intcntzero() != 0 { try!(write!(f, " pwm_2_inten_intcntzero"))}
      if self.pwm_2_inten_intcntload() != 0 { try!(write!(f, " pwm_2_inten_intcntload"))}
      if self.pwm_2_inten_intcmpau() != 0 { try!(write!(f, " pwm_2_inten_intcmpau"))}
      if self.pwm_2_inten_intcmpad() != 0 { try!(write!(f, " pwm_2_inten_intcmpad"))}
      if self.pwm_2_inten_intcmpbu() != 0 { try!(write!(f, " pwm_2_inten_intcmpbu"))}
      if self.pwm_2_inten_intcmpbd() != 0 { try!(write!(f, " pwm_2_inten_intcmpbd"))}
      if self.pwm_2_inten_trcntzero() != 0 { try!(write!(f, " pwm_2_inten_trcntzero"))}
      if self.pwm_2_inten_trcntload() != 0 { try!(write!(f, " pwm_2_inten_trcntload"))}
      if self.pwm_2_inten_trcmpau() != 0 { try!(write!(f, " pwm_2_inten_trcmpau"))}
      if self.pwm_2_inten_trcmpad() != 0 { try!(write!(f, " pwm_2_inten_trcmpad"))}
      if self.pwm_2_inten_trcmpbu() != 0 { try!(write!(f, " pwm_2_inten_trcmpbu"))}
      if self.pwm_2_inten_trcmpbd() != 0 { try!(write!(f, " pwm_2_inten_trcmpbd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _2Ris(pub u32);
impl _2Ris {
  #[inline] pub fn pwm_2_ris_intcntzero(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_2_ris_intcntzero(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_2_ris_intcntload(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_2_ris_intcntload(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_2_ris_intcmpau(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_2_ris_intcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_2_ris_intcmpad(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_2_ris_intcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_2_ris_intcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_2_ris_intcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_2_ris_intcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_2_ris_intcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for _2Ris {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _2Ris {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_2_ris_intcntzero() != 0 { try!(write!(f, " pwm_2_ris_intcntzero"))}
      if self.pwm_2_ris_intcntload() != 0 { try!(write!(f, " pwm_2_ris_intcntload"))}
      if self.pwm_2_ris_intcmpau() != 0 { try!(write!(f, " pwm_2_ris_intcmpau"))}
      if self.pwm_2_ris_intcmpad() != 0 { try!(write!(f, " pwm_2_ris_intcmpad"))}
      if self.pwm_2_ris_intcmpbu() != 0 { try!(write!(f, " pwm_2_ris_intcmpbu"))}
      if self.pwm_2_ris_intcmpbd() != 0 { try!(write!(f, " pwm_2_ris_intcmpbd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _2Isc(pub u32);
impl _2Isc {
  #[inline] pub fn pwm_2_isc_intcntzero(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_2_isc_intcntzero(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_2_isc_intcntload(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_2_isc_intcntload(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_2_isc_intcmpau(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_2_isc_intcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_2_isc_intcmpad(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_2_isc_intcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_2_isc_intcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_2_isc_intcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_2_isc_intcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_2_isc_intcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for _2Isc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _2Isc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_2_isc_intcntzero() != 0 { try!(write!(f, " pwm_2_isc_intcntzero"))}
      if self.pwm_2_isc_intcntload() != 0 { try!(write!(f, " pwm_2_isc_intcntload"))}
      if self.pwm_2_isc_intcmpau() != 0 { try!(write!(f, " pwm_2_isc_intcmpau"))}
      if self.pwm_2_isc_intcmpad() != 0 { try!(write!(f, " pwm_2_isc_intcmpad"))}
      if self.pwm_2_isc_intcmpbu() != 0 { try!(write!(f, " pwm_2_isc_intcmpbu"))}
      if self.pwm_2_isc_intcmpbd() != 0 { try!(write!(f, " pwm_2_isc_intcmpbd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _2Load(pub u32);
impl _2Load {
  #[inline] pub fn pwm_2_load_load(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_pwm_2_load_load(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _2Load {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _2Load {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_2_load_load() != 0 { try!(write!(f, " pwm_2_load_load=0x{:x}", self.pwm_2_load_load()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _2Count(pub u32);
impl _2Count {
  #[inline] pub fn pwm_2_count_count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_pwm_2_count_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _2Count {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _2Count {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_2_count_count() != 0 { try!(write!(f, " pwm_2_count_count=0x{:x}", self.pwm_2_count_count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _2Cmpa(pub u32);
impl _2Cmpa {
  #[inline] pub fn pwm_2_cmpa_compa(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_pwm_2_cmpa_compa(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _2Cmpa {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _2Cmpa {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_2_cmpa_compa() != 0 { try!(write!(f, " pwm_2_cmpa_compa=0x{:x}", self.pwm_2_cmpa_compa()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _2Cmpb(pub u32);
impl _2Cmpb {
  #[inline] pub fn pwm_2_cmpb_compb(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_pwm_2_cmpb_compb(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _2Cmpb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _2Cmpb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_2_cmpb_compb() != 0 { try!(write!(f, " pwm_2_cmpb_compb=0x{:x}", self.pwm_2_cmpb_compb()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _2Gena(pub u32);
impl _2Gena {
  #[inline] pub fn pwm_2_gena_actzero(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline] pub fn set_pwm_2_gena_actzero(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_2_gena_actload(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
  #[inline] pub fn set_pwm_2_gena_actload(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_2_gena_actcmpau(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  #[inline] pub fn set_pwm_2_gena_actcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_2_gena_actcmpad(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
  #[inline] pub fn set_pwm_2_gena_actcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn pwm_2_gena_actcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  #[inline] pub fn set_pwm_2_gena_actcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn pwm_2_gena_actcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
  #[inline] pub fn set_pwm_2_gena_actcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

}
impl ::core::fmt::Display for _2Gena {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _2Gena {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_2_gena_actzero() != 0 { try!(write!(f, " pwm_2_gena_actzero=0x{:x}", self.pwm_2_gena_actzero()))}
      if self.pwm_2_gena_actload() != 0 { try!(write!(f, " pwm_2_gena_actload=0x{:x}", self.pwm_2_gena_actload()))}
      if self.pwm_2_gena_actcmpau() != 0 { try!(write!(f, " pwm_2_gena_actcmpau=0x{:x}", self.pwm_2_gena_actcmpau()))}
      if self.pwm_2_gena_actcmpad() != 0 { try!(write!(f, " pwm_2_gena_actcmpad=0x{:x}", self.pwm_2_gena_actcmpad()))}
      if self.pwm_2_gena_actcmpbu() != 0 { try!(write!(f, " pwm_2_gena_actcmpbu=0x{:x}", self.pwm_2_gena_actcmpbu()))}
      if self.pwm_2_gena_actcmpbd() != 0 { try!(write!(f, " pwm_2_gena_actcmpbd=0x{:x}", self.pwm_2_gena_actcmpbd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _2Genb(pub u32);
impl _2Genb {
  #[inline] pub fn pwm_2_genb_actzero(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline] pub fn set_pwm_2_genb_actzero(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_2_genb_actload(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
  #[inline] pub fn set_pwm_2_genb_actload(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_2_genb_actcmpau(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  #[inline] pub fn set_pwm_2_genb_actcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_2_genb_actcmpad(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
  #[inline] pub fn set_pwm_2_genb_actcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn pwm_2_genb_actcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  #[inline] pub fn set_pwm_2_genb_actcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn pwm_2_genb_actcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
  #[inline] pub fn set_pwm_2_genb_actcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

}
impl ::core::fmt::Display for _2Genb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _2Genb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_2_genb_actzero() != 0 { try!(write!(f, " pwm_2_genb_actzero=0x{:x}", self.pwm_2_genb_actzero()))}
      if self.pwm_2_genb_actload() != 0 { try!(write!(f, " pwm_2_genb_actload=0x{:x}", self.pwm_2_genb_actload()))}
      if self.pwm_2_genb_actcmpau() != 0 { try!(write!(f, " pwm_2_genb_actcmpau=0x{:x}", self.pwm_2_genb_actcmpau()))}
      if self.pwm_2_genb_actcmpad() != 0 { try!(write!(f, " pwm_2_genb_actcmpad=0x{:x}", self.pwm_2_genb_actcmpad()))}
      if self.pwm_2_genb_actcmpbu() != 0 { try!(write!(f, " pwm_2_genb_actcmpbu=0x{:x}", self.pwm_2_genb_actcmpbu()))}
      if self.pwm_2_genb_actcmpbd() != 0 { try!(write!(f, " pwm_2_genb_actcmpbd=0x{:x}", self.pwm_2_genb_actcmpbd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _2Dbctl(pub u32);
impl _2Dbctl {
  #[inline] pub fn pwm_2_dbctl_enable(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_2_dbctl_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _2Dbctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _2Dbctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_2_dbctl_enable() != 0 { try!(write!(f, " pwm_2_dbctl_enable"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _2Dbrise(pub u32);
impl _2Dbrise {
  #[inline] pub fn pwm_2_dbrise_risedelay(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  #[inline] pub fn set_pwm_2_dbrise_risedelay(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _2Dbrise {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _2Dbrise {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_2_dbrise_risedelay() != 0 { try!(write!(f, " pwm_2_dbrise_risedelay=0x{:x}", self.pwm_2_dbrise_risedelay()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _2Dbfall(pub u32);
impl _2Dbfall {
  #[inline] pub fn pwm_2_dbfall_falldelay(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  #[inline] pub fn set_pwm_2_dbfall_falldelay(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _2Dbfall {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _2Dbfall {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_2_dbfall_falldelay() != 0 { try!(write!(f, " pwm_2_dbfall_falldelay=0x{:x}", self.pwm_2_dbfall_falldelay()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _2Fltsrc0(pub u32);
impl _2Fltsrc0 {
  #[inline] pub fn pwm_2_fltsrc0_fault0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_2_fltsrc0_fault0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_2_fltsrc0_fault1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_2_fltsrc0_fault1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_2_fltsrc0_fault2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_2_fltsrc0_fault2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_2_fltsrc0_fault3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_2_fltsrc0_fault3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}
impl ::core::fmt::Display for _2Fltsrc0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _2Fltsrc0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_2_fltsrc0_fault0() != 0 { try!(write!(f, " pwm_2_fltsrc0_fault0"))}
      if self.pwm_2_fltsrc0_fault1() != 0 { try!(write!(f, " pwm_2_fltsrc0_fault1"))}
      if self.pwm_2_fltsrc0_fault2() != 0 { try!(write!(f, " pwm_2_fltsrc0_fault2"))}
      if self.pwm_2_fltsrc0_fault3() != 0 { try!(write!(f, " pwm_2_fltsrc0_fault3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _2Fltsrc1(pub u32);
impl _2Fltsrc1 {
  #[inline] pub fn pwm_2_fltsrc1_dcmp0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_2_fltsrc1_dcmp0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_2_fltsrc1_dcmp1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_2_fltsrc1_dcmp1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_2_fltsrc1_dcmp2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_2_fltsrc1_dcmp2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_2_fltsrc1_dcmp3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_2_fltsrc1_dcmp3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_2_fltsrc1_dcmp4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_2_fltsrc1_dcmp4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_2_fltsrc1_dcmp5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_2_fltsrc1_dcmp5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn pwm_2_fltsrc1_dcmp6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_pwm_2_fltsrc1_dcmp6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn pwm_2_fltsrc1_dcmp7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_pwm_2_fltsrc1_dcmp7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for _2Fltsrc1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _2Fltsrc1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_2_fltsrc1_dcmp0() != 0 { try!(write!(f, " pwm_2_fltsrc1_dcmp0"))}
      if self.pwm_2_fltsrc1_dcmp1() != 0 { try!(write!(f, " pwm_2_fltsrc1_dcmp1"))}
      if self.pwm_2_fltsrc1_dcmp2() != 0 { try!(write!(f, " pwm_2_fltsrc1_dcmp2"))}
      if self.pwm_2_fltsrc1_dcmp3() != 0 { try!(write!(f, " pwm_2_fltsrc1_dcmp3"))}
      if self.pwm_2_fltsrc1_dcmp4() != 0 { try!(write!(f, " pwm_2_fltsrc1_dcmp4"))}
      if self.pwm_2_fltsrc1_dcmp5() != 0 { try!(write!(f, " pwm_2_fltsrc1_dcmp5"))}
      if self.pwm_2_fltsrc1_dcmp6() != 0 { try!(write!(f, " pwm_2_fltsrc1_dcmp6"))}
      if self.pwm_2_fltsrc1_dcmp7() != 0 { try!(write!(f, " pwm_2_fltsrc1_dcmp7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _2Minfltper(pub u32);
impl _2Minfltper {
  #[inline] pub fn pwm_2_minfltper_mfp(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_pwm_2_minfltper_mfp(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _2Minfltper {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _2Minfltper {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_2_minfltper_mfp() != 0 { try!(write!(f, " pwm_2_minfltper_mfp=0x{:x}", self.pwm_2_minfltper_mfp()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _3Ctl(pub u32);
impl _3Ctl {
  #[inline] pub fn pwm_3_ctl_enable(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_3_ctl_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_3_ctl_mode(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_3_ctl_mode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_3_ctl_debug(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_3_ctl_debug(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_3_ctl_loadupd(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_3_ctl_loadupd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_3_ctl_cmpaupd(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_3_ctl_cmpaupd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_3_ctl_cmpbupd(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_3_ctl_cmpbupd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn pwm_3_ctl_genaupd(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
  #[inline] pub fn set_pwm_3_ctl_genaupd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn pwm_3_ctl_genbupd(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  #[inline] pub fn set_pwm_3_ctl_genbupd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn pwm_3_ctl_dbctlupd(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
  #[inline] pub fn set_pwm_3_ctl_dbctlupd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn pwm_3_ctl_dbriseupd(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x3 // [13:12]
  }
  #[inline] pub fn set_pwm_3_ctl_dbriseupd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn pwm_3_ctl_dbfallupd(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x3 // [15:14]
  }
  #[inline] pub fn set_pwm_3_ctl_dbfallupd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline] pub fn pwm_3_ctl_fltsrc(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline] pub fn set_pwm_3_ctl_fltsrc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline] pub fn pwm_3_ctl_minfltper(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline] pub fn set_pwm_3_ctl_minfltper(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline] pub fn pwm_3_ctl_latch(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline] pub fn set_pwm_3_ctl_latch(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

}
impl ::core::fmt::Display for _3Ctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _3Ctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_3_ctl_enable() != 0 { try!(write!(f, " pwm_3_ctl_enable"))}
      if self.pwm_3_ctl_mode() != 0 { try!(write!(f, " pwm_3_ctl_mode"))}
      if self.pwm_3_ctl_debug() != 0 { try!(write!(f, " pwm_3_ctl_debug"))}
      if self.pwm_3_ctl_loadupd() != 0 { try!(write!(f, " pwm_3_ctl_loadupd"))}
      if self.pwm_3_ctl_cmpaupd() != 0 { try!(write!(f, " pwm_3_ctl_cmpaupd"))}
      if self.pwm_3_ctl_cmpbupd() != 0 { try!(write!(f, " pwm_3_ctl_cmpbupd"))}
      if self.pwm_3_ctl_genaupd() != 0 { try!(write!(f, " pwm_3_ctl_genaupd=0x{:x}", self.pwm_3_ctl_genaupd()))}
      if self.pwm_3_ctl_genbupd() != 0 { try!(write!(f, " pwm_3_ctl_genbupd=0x{:x}", self.pwm_3_ctl_genbupd()))}
      if self.pwm_3_ctl_dbctlupd() != 0 { try!(write!(f, " pwm_3_ctl_dbctlupd=0x{:x}", self.pwm_3_ctl_dbctlupd()))}
      if self.pwm_3_ctl_dbriseupd() != 0 { try!(write!(f, " pwm_3_ctl_dbriseupd=0x{:x}", self.pwm_3_ctl_dbriseupd()))}
      if self.pwm_3_ctl_dbfallupd() != 0 { try!(write!(f, " pwm_3_ctl_dbfallupd=0x{:x}", self.pwm_3_ctl_dbfallupd()))}
      if self.pwm_3_ctl_fltsrc() != 0 { try!(write!(f, " pwm_3_ctl_fltsrc"))}
      if self.pwm_3_ctl_minfltper() != 0 { try!(write!(f, " pwm_3_ctl_minfltper"))}
      if self.pwm_3_ctl_latch() != 0 { try!(write!(f, " pwm_3_ctl_latch"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _3Inten(pub u32);
impl _3Inten {
  #[inline] pub fn pwm_3_inten_intcntzero(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_3_inten_intcntzero(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_3_inten_intcntload(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_3_inten_intcntload(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_3_inten_intcmpau(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_3_inten_intcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_3_inten_intcmpad(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_3_inten_intcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_3_inten_intcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_3_inten_intcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_3_inten_intcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_3_inten_intcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn pwm_3_inten_trcntzero(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_pwm_3_inten_trcntzero(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn pwm_3_inten_trcntload(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_pwm_3_inten_trcntload(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn pwm_3_inten_trcmpau(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_pwm_3_inten_trcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn pwm_3_inten_trcmpad(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_pwm_3_inten_trcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline] pub fn pwm_3_inten_trcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline] pub fn set_pwm_3_inten_trcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn pwm_3_inten_trcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline] pub fn set_pwm_3_inten_trcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

}
impl ::core::fmt::Display for _3Inten {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _3Inten {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_3_inten_intcntzero() != 0 { try!(write!(f, " pwm_3_inten_intcntzero"))}
      if self.pwm_3_inten_intcntload() != 0 { try!(write!(f, " pwm_3_inten_intcntload"))}
      if self.pwm_3_inten_intcmpau() != 0 { try!(write!(f, " pwm_3_inten_intcmpau"))}
      if self.pwm_3_inten_intcmpad() != 0 { try!(write!(f, " pwm_3_inten_intcmpad"))}
      if self.pwm_3_inten_intcmpbu() != 0 { try!(write!(f, " pwm_3_inten_intcmpbu"))}
      if self.pwm_3_inten_intcmpbd() != 0 { try!(write!(f, " pwm_3_inten_intcmpbd"))}
      if self.pwm_3_inten_trcntzero() != 0 { try!(write!(f, " pwm_3_inten_trcntzero"))}
      if self.pwm_3_inten_trcntload() != 0 { try!(write!(f, " pwm_3_inten_trcntload"))}
      if self.pwm_3_inten_trcmpau() != 0 { try!(write!(f, " pwm_3_inten_trcmpau"))}
      if self.pwm_3_inten_trcmpad() != 0 { try!(write!(f, " pwm_3_inten_trcmpad"))}
      if self.pwm_3_inten_trcmpbu() != 0 { try!(write!(f, " pwm_3_inten_trcmpbu"))}
      if self.pwm_3_inten_trcmpbd() != 0 { try!(write!(f, " pwm_3_inten_trcmpbd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _3Ris(pub u32);
impl _3Ris {
  #[inline] pub fn pwm_3_ris_intcntzero(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_3_ris_intcntzero(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_3_ris_intcntload(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_3_ris_intcntload(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_3_ris_intcmpau(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_3_ris_intcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_3_ris_intcmpad(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_3_ris_intcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_3_ris_intcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_3_ris_intcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_3_ris_intcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_3_ris_intcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for _3Ris {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _3Ris {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_3_ris_intcntzero() != 0 { try!(write!(f, " pwm_3_ris_intcntzero"))}
      if self.pwm_3_ris_intcntload() != 0 { try!(write!(f, " pwm_3_ris_intcntload"))}
      if self.pwm_3_ris_intcmpau() != 0 { try!(write!(f, " pwm_3_ris_intcmpau"))}
      if self.pwm_3_ris_intcmpad() != 0 { try!(write!(f, " pwm_3_ris_intcmpad"))}
      if self.pwm_3_ris_intcmpbu() != 0 { try!(write!(f, " pwm_3_ris_intcmpbu"))}
      if self.pwm_3_ris_intcmpbd() != 0 { try!(write!(f, " pwm_3_ris_intcmpbd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _3Isc(pub u32);
impl _3Isc {
  #[inline] pub fn pwm_3_isc_intcntzero(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_3_isc_intcntzero(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_3_isc_intcntload(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_3_isc_intcntload(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_3_isc_intcmpau(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_3_isc_intcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_3_isc_intcmpad(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_3_isc_intcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_3_isc_intcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_3_isc_intcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_3_isc_intcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_3_isc_intcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for _3Isc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _3Isc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_3_isc_intcntzero() != 0 { try!(write!(f, " pwm_3_isc_intcntzero"))}
      if self.pwm_3_isc_intcntload() != 0 { try!(write!(f, " pwm_3_isc_intcntload"))}
      if self.pwm_3_isc_intcmpau() != 0 { try!(write!(f, " pwm_3_isc_intcmpau"))}
      if self.pwm_3_isc_intcmpad() != 0 { try!(write!(f, " pwm_3_isc_intcmpad"))}
      if self.pwm_3_isc_intcmpbu() != 0 { try!(write!(f, " pwm_3_isc_intcmpbu"))}
      if self.pwm_3_isc_intcmpbd() != 0 { try!(write!(f, " pwm_3_isc_intcmpbd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _3Load(pub u32);
impl _3Load {
  #[inline] pub fn pwm_3_load_load(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_pwm_3_load_load(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _3Load {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _3Load {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_3_load_load() != 0 { try!(write!(f, " pwm_3_load_load=0x{:x}", self.pwm_3_load_load()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _3Count(pub u32);
impl _3Count {
  #[inline] pub fn pwm_3_count_count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_pwm_3_count_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _3Count {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _3Count {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_3_count_count() != 0 { try!(write!(f, " pwm_3_count_count=0x{:x}", self.pwm_3_count_count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _3Cmpa(pub u32);
impl _3Cmpa {
  #[inline] pub fn pwm_3_cmpa_compa(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_pwm_3_cmpa_compa(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _3Cmpa {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _3Cmpa {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_3_cmpa_compa() != 0 { try!(write!(f, " pwm_3_cmpa_compa=0x{:x}", self.pwm_3_cmpa_compa()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _3Cmpb(pub u32);
impl _3Cmpb {
  #[inline] pub fn pwm_3_cmpb_compb(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_pwm_3_cmpb_compb(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _3Cmpb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _3Cmpb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_3_cmpb_compb() != 0 { try!(write!(f, " pwm_3_cmpb_compb=0x{:x}", self.pwm_3_cmpb_compb()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _3Gena(pub u32);
impl _3Gena {
  #[inline] pub fn pwm_3_gena_actzero(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline] pub fn set_pwm_3_gena_actzero(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_3_gena_actload(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
  #[inline] pub fn set_pwm_3_gena_actload(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_3_gena_actcmpau(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  #[inline] pub fn set_pwm_3_gena_actcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_3_gena_actcmpad(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
  #[inline] pub fn set_pwm_3_gena_actcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn pwm_3_gena_actcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  #[inline] pub fn set_pwm_3_gena_actcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn pwm_3_gena_actcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
  #[inline] pub fn set_pwm_3_gena_actcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

}
impl ::core::fmt::Display for _3Gena {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _3Gena {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_3_gena_actzero() != 0 { try!(write!(f, " pwm_3_gena_actzero=0x{:x}", self.pwm_3_gena_actzero()))}
      if self.pwm_3_gena_actload() != 0 { try!(write!(f, " pwm_3_gena_actload=0x{:x}", self.pwm_3_gena_actload()))}
      if self.pwm_3_gena_actcmpau() != 0 { try!(write!(f, " pwm_3_gena_actcmpau=0x{:x}", self.pwm_3_gena_actcmpau()))}
      if self.pwm_3_gena_actcmpad() != 0 { try!(write!(f, " pwm_3_gena_actcmpad=0x{:x}", self.pwm_3_gena_actcmpad()))}
      if self.pwm_3_gena_actcmpbu() != 0 { try!(write!(f, " pwm_3_gena_actcmpbu=0x{:x}", self.pwm_3_gena_actcmpbu()))}
      if self.pwm_3_gena_actcmpbd() != 0 { try!(write!(f, " pwm_3_gena_actcmpbd=0x{:x}", self.pwm_3_gena_actcmpbd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _3Genb(pub u32);
impl _3Genb {
  #[inline] pub fn pwm_3_genb_actzero(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline] pub fn set_pwm_3_genb_actzero(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_3_genb_actload(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
  #[inline] pub fn set_pwm_3_genb_actload(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_3_genb_actcmpau(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  #[inline] pub fn set_pwm_3_genb_actcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_3_genb_actcmpad(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
  #[inline] pub fn set_pwm_3_genb_actcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn pwm_3_genb_actcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  #[inline] pub fn set_pwm_3_genb_actcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn pwm_3_genb_actcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
  #[inline] pub fn set_pwm_3_genb_actcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

}
impl ::core::fmt::Display for _3Genb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _3Genb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_3_genb_actzero() != 0 { try!(write!(f, " pwm_3_genb_actzero=0x{:x}", self.pwm_3_genb_actzero()))}
      if self.pwm_3_genb_actload() != 0 { try!(write!(f, " pwm_3_genb_actload=0x{:x}", self.pwm_3_genb_actload()))}
      if self.pwm_3_genb_actcmpau() != 0 { try!(write!(f, " pwm_3_genb_actcmpau=0x{:x}", self.pwm_3_genb_actcmpau()))}
      if self.pwm_3_genb_actcmpad() != 0 { try!(write!(f, " pwm_3_genb_actcmpad=0x{:x}", self.pwm_3_genb_actcmpad()))}
      if self.pwm_3_genb_actcmpbu() != 0 { try!(write!(f, " pwm_3_genb_actcmpbu=0x{:x}", self.pwm_3_genb_actcmpbu()))}
      if self.pwm_3_genb_actcmpbd() != 0 { try!(write!(f, " pwm_3_genb_actcmpbd=0x{:x}", self.pwm_3_genb_actcmpbd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _3Dbctl(pub u32);
impl _3Dbctl {
  #[inline] pub fn pwm_3_dbctl_enable(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_3_dbctl_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _3Dbctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _3Dbctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_3_dbctl_enable() != 0 { try!(write!(f, " pwm_3_dbctl_enable"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _3Dbrise(pub u32);
impl _3Dbrise {
  #[inline] pub fn pwm_3_dbrise_risedelay(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  #[inline] pub fn set_pwm_3_dbrise_risedelay(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _3Dbrise {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _3Dbrise {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_3_dbrise_risedelay() != 0 { try!(write!(f, " pwm_3_dbrise_risedelay=0x{:x}", self.pwm_3_dbrise_risedelay()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _3Dbfall(pub u32);
impl _3Dbfall {
  #[inline] pub fn pwm_3_dbfall_falldelay(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  #[inline] pub fn set_pwm_3_dbfall_falldelay(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _3Dbfall {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _3Dbfall {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_3_dbfall_falldelay() != 0 { try!(write!(f, " pwm_3_dbfall_falldelay=0x{:x}", self.pwm_3_dbfall_falldelay()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _3Fltsrc0(pub u32);
impl _3Fltsrc0 {
  #[inline] pub fn pwm_3_fltsrc0_fault0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_3_fltsrc0_fault0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_3_fltsrc0_fault1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_3_fltsrc0_fault1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_3_fltsrc0_fault2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_3_fltsrc0_fault2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_3_fltsrc0_fault3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_3_fltsrc0_fault3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}
impl ::core::fmt::Display for _3Fltsrc0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _3Fltsrc0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_3_fltsrc0_fault0() != 0 { try!(write!(f, " pwm_3_fltsrc0_fault0"))}
      if self.pwm_3_fltsrc0_fault1() != 0 { try!(write!(f, " pwm_3_fltsrc0_fault1"))}
      if self.pwm_3_fltsrc0_fault2() != 0 { try!(write!(f, " pwm_3_fltsrc0_fault2"))}
      if self.pwm_3_fltsrc0_fault3() != 0 { try!(write!(f, " pwm_3_fltsrc0_fault3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _3Fltsrc1(pub u32);
impl _3Fltsrc1 {
  #[inline] pub fn pwm_3_fltsrc1_dcmp0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_3_fltsrc1_dcmp0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_3_fltsrc1_dcmp1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_3_fltsrc1_dcmp1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_3_fltsrc1_dcmp2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_3_fltsrc1_dcmp2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_3_fltsrc1_dcmp3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_3_fltsrc1_dcmp3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_3_fltsrc1_dcmp4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_3_fltsrc1_dcmp4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_3_fltsrc1_dcmp5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_3_fltsrc1_dcmp5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn pwm_3_fltsrc1_dcmp6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_pwm_3_fltsrc1_dcmp6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn pwm_3_fltsrc1_dcmp7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_pwm_3_fltsrc1_dcmp7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for _3Fltsrc1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _3Fltsrc1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_3_fltsrc1_dcmp0() != 0 { try!(write!(f, " pwm_3_fltsrc1_dcmp0"))}
      if self.pwm_3_fltsrc1_dcmp1() != 0 { try!(write!(f, " pwm_3_fltsrc1_dcmp1"))}
      if self.pwm_3_fltsrc1_dcmp2() != 0 { try!(write!(f, " pwm_3_fltsrc1_dcmp2"))}
      if self.pwm_3_fltsrc1_dcmp3() != 0 { try!(write!(f, " pwm_3_fltsrc1_dcmp3"))}
      if self.pwm_3_fltsrc1_dcmp4() != 0 { try!(write!(f, " pwm_3_fltsrc1_dcmp4"))}
      if self.pwm_3_fltsrc1_dcmp5() != 0 { try!(write!(f, " pwm_3_fltsrc1_dcmp5"))}
      if self.pwm_3_fltsrc1_dcmp6() != 0 { try!(write!(f, " pwm_3_fltsrc1_dcmp6"))}
      if self.pwm_3_fltsrc1_dcmp7() != 0 { try!(write!(f, " pwm_3_fltsrc1_dcmp7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _3Minfltper(pub u32);
impl _3Minfltper {
  #[inline] pub fn pwm_3_minfltper_mfp(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_pwm_3_minfltper_mfp(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for _3Minfltper {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _3Minfltper {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_3_minfltper_mfp() != 0 { try!(write!(f, " pwm_3_minfltper_mfp=0x{:x}", self.pwm_3_minfltper_mfp()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _0Fltsen(pub u32);
impl _0Fltsen {
  #[inline] pub fn pwm_0_fltsen_fault0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_0_fltsen_fault0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_0_fltsen_fault1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_0_fltsen_fault1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_0_fltsen_fault2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_0_fltsen_fault2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_0_fltsen_fault3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_0_fltsen_fault3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}
impl ::core::fmt::Display for _0Fltsen {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _0Fltsen {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_0_fltsen_fault0() != 0 { try!(write!(f, " pwm_0_fltsen_fault0"))}
      if self.pwm_0_fltsen_fault1() != 0 { try!(write!(f, " pwm_0_fltsen_fault1"))}
      if self.pwm_0_fltsen_fault2() != 0 { try!(write!(f, " pwm_0_fltsen_fault2"))}
      if self.pwm_0_fltsen_fault3() != 0 { try!(write!(f, " pwm_0_fltsen_fault3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _0Fltstat0(pub u32);
impl _0Fltstat0 {
  #[inline] pub fn pwm_0_fltstat0_fault0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_0_fltstat0_fault0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_0_fltstat0_fault1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_0_fltstat0_fault1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_0_fltstat0_fault2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_0_fltstat0_fault2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_0_fltstat0_fault3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_0_fltstat0_fault3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}
impl ::core::fmt::Display for _0Fltstat0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _0Fltstat0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_0_fltstat0_fault0() != 0 { try!(write!(f, " pwm_0_fltstat0_fault0"))}
      if self.pwm_0_fltstat0_fault1() != 0 { try!(write!(f, " pwm_0_fltstat0_fault1"))}
      if self.pwm_0_fltstat0_fault2() != 0 { try!(write!(f, " pwm_0_fltstat0_fault2"))}
      if self.pwm_0_fltstat0_fault3() != 0 { try!(write!(f, " pwm_0_fltstat0_fault3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _0Fltstat1(pub u32);
impl _0Fltstat1 {
  #[inline] pub fn pwm_0_fltstat1_dcmp0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_0_fltstat1_dcmp0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_0_fltstat1_dcmp1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_0_fltstat1_dcmp1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_0_fltstat1_dcmp2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_0_fltstat1_dcmp2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_0_fltstat1_dcmp3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_0_fltstat1_dcmp3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_0_fltstat1_dcmp4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_0_fltstat1_dcmp4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_0_fltstat1_dcmp5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_0_fltstat1_dcmp5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn pwm_0_fltstat1_dcmp6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_pwm_0_fltstat1_dcmp6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn pwm_0_fltstat1_dcmp7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_pwm_0_fltstat1_dcmp7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for _0Fltstat1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _0Fltstat1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_0_fltstat1_dcmp0() != 0 { try!(write!(f, " pwm_0_fltstat1_dcmp0"))}
      if self.pwm_0_fltstat1_dcmp1() != 0 { try!(write!(f, " pwm_0_fltstat1_dcmp1"))}
      if self.pwm_0_fltstat1_dcmp2() != 0 { try!(write!(f, " pwm_0_fltstat1_dcmp2"))}
      if self.pwm_0_fltstat1_dcmp3() != 0 { try!(write!(f, " pwm_0_fltstat1_dcmp3"))}
      if self.pwm_0_fltstat1_dcmp4() != 0 { try!(write!(f, " pwm_0_fltstat1_dcmp4"))}
      if self.pwm_0_fltstat1_dcmp5() != 0 { try!(write!(f, " pwm_0_fltstat1_dcmp5"))}
      if self.pwm_0_fltstat1_dcmp6() != 0 { try!(write!(f, " pwm_0_fltstat1_dcmp6"))}
      if self.pwm_0_fltstat1_dcmp7() != 0 { try!(write!(f, " pwm_0_fltstat1_dcmp7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _1Fltsen(pub u32);
impl _1Fltsen {
  #[inline] pub fn pwm_1_fltsen_fault0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_1_fltsen_fault0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_1_fltsen_fault1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_1_fltsen_fault1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_1_fltsen_fault2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_1_fltsen_fault2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_1_fltsen_fault3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_1_fltsen_fault3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}
impl ::core::fmt::Display for _1Fltsen {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _1Fltsen {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_1_fltsen_fault0() != 0 { try!(write!(f, " pwm_1_fltsen_fault0"))}
      if self.pwm_1_fltsen_fault1() != 0 { try!(write!(f, " pwm_1_fltsen_fault1"))}
      if self.pwm_1_fltsen_fault2() != 0 { try!(write!(f, " pwm_1_fltsen_fault2"))}
      if self.pwm_1_fltsen_fault3() != 0 { try!(write!(f, " pwm_1_fltsen_fault3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _1Fltstat0(pub u32);
impl _1Fltstat0 {
  #[inline] pub fn pwm_1_fltstat0_fault0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_1_fltstat0_fault0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_1_fltstat0_fault1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_1_fltstat0_fault1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_1_fltstat0_fault2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_1_fltstat0_fault2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_1_fltstat0_fault3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_1_fltstat0_fault3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}
impl ::core::fmt::Display for _1Fltstat0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _1Fltstat0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_1_fltstat0_fault0() != 0 { try!(write!(f, " pwm_1_fltstat0_fault0"))}
      if self.pwm_1_fltstat0_fault1() != 0 { try!(write!(f, " pwm_1_fltstat0_fault1"))}
      if self.pwm_1_fltstat0_fault2() != 0 { try!(write!(f, " pwm_1_fltstat0_fault2"))}
      if self.pwm_1_fltstat0_fault3() != 0 { try!(write!(f, " pwm_1_fltstat0_fault3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _1Fltstat1(pub u32);
impl _1Fltstat1 {
  #[inline] pub fn pwm_1_fltstat1_dcmp0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_1_fltstat1_dcmp0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_1_fltstat1_dcmp1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_1_fltstat1_dcmp1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_1_fltstat1_dcmp2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_1_fltstat1_dcmp2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_1_fltstat1_dcmp3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_1_fltstat1_dcmp3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_1_fltstat1_dcmp4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_1_fltstat1_dcmp4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_1_fltstat1_dcmp5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_1_fltstat1_dcmp5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn pwm_1_fltstat1_dcmp6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_pwm_1_fltstat1_dcmp6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn pwm_1_fltstat1_dcmp7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_pwm_1_fltstat1_dcmp7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for _1Fltstat1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _1Fltstat1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_1_fltstat1_dcmp0() != 0 { try!(write!(f, " pwm_1_fltstat1_dcmp0"))}
      if self.pwm_1_fltstat1_dcmp1() != 0 { try!(write!(f, " pwm_1_fltstat1_dcmp1"))}
      if self.pwm_1_fltstat1_dcmp2() != 0 { try!(write!(f, " pwm_1_fltstat1_dcmp2"))}
      if self.pwm_1_fltstat1_dcmp3() != 0 { try!(write!(f, " pwm_1_fltstat1_dcmp3"))}
      if self.pwm_1_fltstat1_dcmp4() != 0 { try!(write!(f, " pwm_1_fltstat1_dcmp4"))}
      if self.pwm_1_fltstat1_dcmp5() != 0 { try!(write!(f, " pwm_1_fltstat1_dcmp5"))}
      if self.pwm_1_fltstat1_dcmp6() != 0 { try!(write!(f, " pwm_1_fltstat1_dcmp6"))}
      if self.pwm_1_fltstat1_dcmp7() != 0 { try!(write!(f, " pwm_1_fltstat1_dcmp7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _2Fltsen(pub u32);
impl _2Fltsen {
  #[inline] pub fn pwm_2_fltsen_fault0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_2_fltsen_fault0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_2_fltsen_fault1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_2_fltsen_fault1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_2_fltsen_fault2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_2_fltsen_fault2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_2_fltsen_fault3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_2_fltsen_fault3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}
impl ::core::fmt::Display for _2Fltsen {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _2Fltsen {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_2_fltsen_fault0() != 0 { try!(write!(f, " pwm_2_fltsen_fault0"))}
      if self.pwm_2_fltsen_fault1() != 0 { try!(write!(f, " pwm_2_fltsen_fault1"))}
      if self.pwm_2_fltsen_fault2() != 0 { try!(write!(f, " pwm_2_fltsen_fault2"))}
      if self.pwm_2_fltsen_fault3() != 0 { try!(write!(f, " pwm_2_fltsen_fault3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _2Fltstat0(pub u32);
impl _2Fltstat0 {
  #[inline] pub fn pwm_2_fltstat0_fault0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_2_fltstat0_fault0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_2_fltstat0_fault1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_2_fltstat0_fault1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_2_fltstat0_fault2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_2_fltstat0_fault2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_2_fltstat0_fault3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_2_fltstat0_fault3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}
impl ::core::fmt::Display for _2Fltstat0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _2Fltstat0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_2_fltstat0_fault0() != 0 { try!(write!(f, " pwm_2_fltstat0_fault0"))}
      if self.pwm_2_fltstat0_fault1() != 0 { try!(write!(f, " pwm_2_fltstat0_fault1"))}
      if self.pwm_2_fltstat0_fault2() != 0 { try!(write!(f, " pwm_2_fltstat0_fault2"))}
      if self.pwm_2_fltstat0_fault3() != 0 { try!(write!(f, " pwm_2_fltstat0_fault3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _2Fltstat1(pub u32);
impl _2Fltstat1 {
  #[inline] pub fn pwm_2_fltstat1_dcmp0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_2_fltstat1_dcmp0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_2_fltstat1_dcmp1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_2_fltstat1_dcmp1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_2_fltstat1_dcmp2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_2_fltstat1_dcmp2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_2_fltstat1_dcmp3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_2_fltstat1_dcmp3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_2_fltstat1_dcmp4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_2_fltstat1_dcmp4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_2_fltstat1_dcmp5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_2_fltstat1_dcmp5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn pwm_2_fltstat1_dcmp6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_pwm_2_fltstat1_dcmp6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn pwm_2_fltstat1_dcmp7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_pwm_2_fltstat1_dcmp7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for _2Fltstat1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _2Fltstat1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_2_fltstat1_dcmp0() != 0 { try!(write!(f, " pwm_2_fltstat1_dcmp0"))}
      if self.pwm_2_fltstat1_dcmp1() != 0 { try!(write!(f, " pwm_2_fltstat1_dcmp1"))}
      if self.pwm_2_fltstat1_dcmp2() != 0 { try!(write!(f, " pwm_2_fltstat1_dcmp2"))}
      if self.pwm_2_fltstat1_dcmp3() != 0 { try!(write!(f, " pwm_2_fltstat1_dcmp3"))}
      if self.pwm_2_fltstat1_dcmp4() != 0 { try!(write!(f, " pwm_2_fltstat1_dcmp4"))}
      if self.pwm_2_fltstat1_dcmp5() != 0 { try!(write!(f, " pwm_2_fltstat1_dcmp5"))}
      if self.pwm_2_fltstat1_dcmp6() != 0 { try!(write!(f, " pwm_2_fltstat1_dcmp6"))}
      if self.pwm_2_fltstat1_dcmp7() != 0 { try!(write!(f, " pwm_2_fltstat1_dcmp7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _3Fltsen(pub u32);
impl _3Fltsen {
  #[inline] pub fn pwm_3_fltsen_fault0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_3_fltsen_fault0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_3_fltsen_fault1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_3_fltsen_fault1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_3_fltsen_fault2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_3_fltsen_fault2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_3_fltsen_fault3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_3_fltsen_fault3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}
impl ::core::fmt::Display for _3Fltsen {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _3Fltsen {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_3_fltsen_fault0() != 0 { try!(write!(f, " pwm_3_fltsen_fault0"))}
      if self.pwm_3_fltsen_fault1() != 0 { try!(write!(f, " pwm_3_fltsen_fault1"))}
      if self.pwm_3_fltsen_fault2() != 0 { try!(write!(f, " pwm_3_fltsen_fault2"))}
      if self.pwm_3_fltsen_fault3() != 0 { try!(write!(f, " pwm_3_fltsen_fault3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _3Fltstat0(pub u32);
impl _3Fltstat0 {
  #[inline] pub fn pwm_3_fltstat0_fault0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_3_fltstat0_fault0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_3_fltstat0_fault1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_3_fltstat0_fault1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_3_fltstat0_fault2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_3_fltstat0_fault2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_3_fltstat0_fault3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_3_fltstat0_fault3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}
impl ::core::fmt::Display for _3Fltstat0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _3Fltstat0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_3_fltstat0_fault0() != 0 { try!(write!(f, " pwm_3_fltstat0_fault0"))}
      if self.pwm_3_fltstat0_fault1() != 0 { try!(write!(f, " pwm_3_fltstat0_fault1"))}
      if self.pwm_3_fltstat0_fault2() != 0 { try!(write!(f, " pwm_3_fltstat0_fault2"))}
      if self.pwm_3_fltstat0_fault3() != 0 { try!(write!(f, " pwm_3_fltstat0_fault3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct _3Fltstat1(pub u32);
impl _3Fltstat1 {
  #[inline] pub fn pwm_3_fltstat1_dcmp0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pwm_3_fltstat1_dcmp0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_3_fltstat1_dcmp1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pwm_3_fltstat1_dcmp1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pwm_3_fltstat1_dcmp2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pwm_3_fltstat1_dcmp2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwm_3_fltstat1_dcmp3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwm_3_fltstat1_dcmp3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pwm_3_fltstat1_dcmp4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pwm_3_fltstat1_dcmp4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_3_fltstat1_dcmp5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pwm_3_fltstat1_dcmp5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn pwm_3_fltstat1_dcmp6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_pwm_3_fltstat1_dcmp6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn pwm_3_fltstat1_dcmp7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_pwm_3_fltstat1_dcmp7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for _3Fltstat1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _3Fltstat1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwm_3_fltstat1_dcmp0() != 0 { try!(write!(f, " pwm_3_fltstat1_dcmp0"))}
      if self.pwm_3_fltstat1_dcmp1() != 0 { try!(write!(f, " pwm_3_fltstat1_dcmp1"))}
      if self.pwm_3_fltstat1_dcmp2() != 0 { try!(write!(f, " pwm_3_fltstat1_dcmp2"))}
      if self.pwm_3_fltstat1_dcmp3() != 0 { try!(write!(f, " pwm_3_fltstat1_dcmp3"))}
      if self.pwm_3_fltstat1_dcmp4() != 0 { try!(write!(f, " pwm_3_fltstat1_dcmp4"))}
      if self.pwm_3_fltstat1_dcmp5() != 0 { try!(write!(f, " pwm_3_fltstat1_dcmp5"))}
      if self.pwm_3_fltstat1_dcmp6() != 0 { try!(write!(f, " pwm_3_fltstat1_dcmp6"))}
      if self.pwm_3_fltstat1_dcmp7() != 0 { try!(write!(f, " pwm_3_fltstat1_dcmp7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Pp(pub u32);
impl Pp {
  #[inline] pub fn pwm_pp_gcnt(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_pwm_pp_gcnt(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_pp_fcnt(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
  #[inline] pub fn set_pwm_pp_fcnt(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pwm_pp_esync(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_pwm_pp_esync(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn pwm_pp_efault(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_pwm_pp_efault(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn pwm_pp_one(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_pwm_pp_one(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
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
      if self.pwm_pp_gcnt() != 0 { try!(write!(f, " pwm_pp_gcnt=0x{:x}", self.pwm_pp_gcnt()))}
      if self.pwm_pp_fcnt() != 0 { try!(write!(f, " pwm_pp_fcnt=0x{:x}", self.pwm_pp_fcnt()))}
      if self.pwm_pp_esync() != 0 { try!(write!(f, " pwm_pp_esync"))}
      if self.pwm_pp_efault() != 0 { try!(write!(f, " pwm_pp_efault"))}
      if self.pwm_pp_one() != 0 { try!(write!(f, " pwm_pp_one"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cc(pub u32);
impl Cc {
  #[inline] pub fn pwm_cc_pwmdiv(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
  #[inline] pub fn set_pwm_cc_pwmdiv(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pwm_cc_usepwm(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_pwm_cc_usepwm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
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
      if self.pwm_cc_pwmdiv() != 0 { try!(write!(f, " pwm_cc_pwmdiv=0x{:x}", self.pwm_cc_pwmdiv()))}
      if self.pwm_cc_usepwm() != 0 { try!(write!(f, " pwm_cc_usepwm"))}
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

pub const PWM0_CH0: Channel<Pwm0Ch0Id, Pwm0Id> = Channel { periph: PWM0, index: 0, id: Pwm0Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pwm0Ch0Id {}
pub type Pwm0Ch0 = Channel<Pwm0Ch0Id, Pwm0Id>;

pub const PWM0_CH1: Channel<Pwm0Ch1Id, Pwm0Id> = Channel { periph: PWM0, index: 1, id: Pwm0Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pwm0Ch1Id {}
pub type Pwm0Ch1 = Channel<Pwm0Ch1Id, Pwm0Id>;

pub const PWM0_CH2: Channel<Pwm0Ch2Id, Pwm0Id> = Channel { periph: PWM0, index: 2, id: Pwm0Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pwm0Ch2Id {}
pub type Pwm0Ch2 = Channel<Pwm0Ch2Id, Pwm0Id>;

pub const PWM0_CH3: Channel<Pwm0Ch3Id, Pwm0Id> = Channel { periph: PWM0, index: 3, id: Pwm0Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pwm0Ch3Id {}
pub type Pwm0Ch3 = Channel<Pwm0Ch3Id, Pwm0Id>;

