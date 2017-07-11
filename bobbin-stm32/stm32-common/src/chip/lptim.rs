
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="LPTIM Peripheral"]
pub struct Periph<T>(pub u32, pub T); 



impl<T> Periph<T> {
#[doc="Get the *const pointer for the ISR register."]
  #[inline] pub fn isr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the ISR register."]
  #[inline] pub fn isr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the ISR register."]
  #[inline] pub fn isr(&self) -> Isr { 
     unsafe {
        Isr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }

#[doc="Get the *const pointer for the ICR register."]
  #[inline] pub fn icr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the ICR register."]
  #[inline] pub fn icr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Write the ICR register."]
  #[inline] pub fn set_icr(&self, value: Icr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the IER register."]
  #[inline] pub fn ier_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the IER register."]
  #[inline] pub fn ier_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the IER register."]
  #[inline] pub fn ier(&self) -> Ier { 
     unsafe {
        Ier(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the IER register."]
  #[inline] pub fn set_ier(&self, value: Ier) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the IER register."]
  #[inline] pub fn with_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
     let tmp = self.ier();
     self.set_ier(f(tmp))
  }

#[doc="Get the *const pointer for the CFGR register."]
  #[inline] pub fn cfgr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the CFGR register."]
  #[inline] pub fn cfgr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the CFGR register."]
  #[inline] pub fn cfgr(&self) -> Cfgr { 
     unsafe {
        Cfgr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the CFGR register."]
  #[inline] pub fn set_cfgr(&self, value: Cfgr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CFGR register."]
  #[inline] pub fn with_cfgr<F: FnOnce(Cfgr) -> Cfgr>(&self, f: F) -> &Self {
     let tmp = self.cfgr();
     self.set_cfgr(f(tmp))
  }

#[doc="Get the *const pointer for the CR register."]
  #[inline] pub fn cr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the CR register."]
  #[inline] pub fn cr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the CR register."]
  #[inline] pub fn cr(&self) -> Cr { 
     unsafe {
        Cr(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the CR register."]
  #[inline] pub fn set_cr(&self, value: Cr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CR register."]
  #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
     let tmp = self.cr();
     self.set_cr(f(tmp))
  }

#[doc="Get the *const pointer for the CMP register."]
  #[inline] pub fn cmp_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the CMP register."]
  #[inline] pub fn cmp_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Read the CMP register."]
  #[inline] pub fn cmp(&self) -> Cmp { 
     unsafe {
        Cmp(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
#[doc="Write the CMP register."]
  #[inline] pub fn set_cmp(&self, value: Cmp) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CMP register."]
  #[inline] pub fn with_cmp<F: FnOnce(Cmp) -> Cmp>(&self, f: F) -> &Self {
     let tmp = self.cmp();
     self.set_cmp(f(tmp))
  }

#[doc="Get the *const pointer for the ARR register."]
  #[inline] pub fn arr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
#[doc="Get the *mut pointer for the ARR register."]
  #[inline] pub fn arr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
#[doc="Read the ARR register."]
  #[inline] pub fn arr(&self) -> Arr { 
     unsafe {
        Arr(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
#[doc="Write the ARR register."]
  #[inline] pub fn set_arr(&self, value: Arr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ARR register."]
  #[inline] pub fn with_arr<F: FnOnce(Arr) -> Arr>(&self, f: F) -> &Self {
     let tmp = self.arr();
     self.set_arr(f(tmp))
  }

#[doc="Get the *const pointer for the CNT register."]
  #[inline] pub fn cnt_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the CNT register."]
  #[inline] pub fn cnt_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Read the CNT register."]
  #[inline] pub fn cnt(&self) -> Cnt { 
     unsafe {
        Cnt(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }

}

#[doc="Interrupt and Status Register"]
#[derive(PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
#[doc="Counter direction change up to down"]
  #[inline] pub fn down(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Counter direction change up to down"]
  #[inline] pub fn set_down(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Counter direction change down to up"]
  #[inline] pub fn up(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Counter direction change down to up"]
  #[inline] pub fn set_up(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Autoreload register update OK"]
  #[inline] pub fn arrok(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Autoreload register update OK"]
  #[inline] pub fn set_arrok(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Compare register update OK"]
  #[inline] pub fn cmpok(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Compare register update OK"]
  #[inline] pub fn set_cmpok(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="External trigger edge event"]
  #[inline] pub fn exttrig(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="External trigger edge event"]
  #[inline] pub fn set_exttrig(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Autoreload match"]
  #[inline] pub fn arrm(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Autoreload match"]
  #[inline] pub fn set_arrm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Compare match"]
  #[inline] pub fn cmpm(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Compare match"]
  #[inline] pub fn set_cmpm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Isr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Isr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.down() != 0 { try!(write!(f, " down"))}
      if self.up() != 0 { try!(write!(f, " up"))}
      if self.arrok() != 0 { try!(write!(f, " arrok"))}
      if self.cmpok() != 0 { try!(write!(f, " cmpok"))}
      if self.exttrig() != 0 { try!(write!(f, " exttrig"))}
      if self.arrm() != 0 { try!(write!(f, " arrm"))}
      if self.cmpm() != 0 { try!(write!(f, " cmpm"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Clear Register"]
#[derive(PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
#[doc="Direction change to down Clear Flag"]
  #[inline] pub fn downcf(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Direction change to down Clear Flag"]
  #[inline] pub fn set_downcf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Direction change to UP Clear Flag"]
  #[inline] pub fn upcf(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Direction change to UP Clear Flag"]
  #[inline] pub fn set_upcf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Autoreload register update OK Clear Flag"]
  #[inline] pub fn arrokcf(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Autoreload register update OK Clear Flag"]
  #[inline] pub fn set_arrokcf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Compare register update OK Clear Flag"]
  #[inline] pub fn cmpokcf(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Compare register update OK Clear Flag"]
  #[inline] pub fn set_cmpokcf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="External trigger valid edge Clear Flag"]
  #[inline] pub fn exttrigcf(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="External trigger valid edge Clear Flag"]
  #[inline] pub fn set_exttrigcf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Autoreload match Clear Flag"]
  #[inline] pub fn arrmcf(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Autoreload match Clear Flag"]
  #[inline] pub fn set_arrmcf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="compare match Clear Flag"]
  #[inline] pub fn cmpmcf(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="compare match Clear Flag"]
  #[inline] pub fn set_cmpmcf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
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
      if self.downcf() != 0 { try!(write!(f, " downcf"))}
      if self.upcf() != 0 { try!(write!(f, " upcf"))}
      if self.arrokcf() != 0 { try!(write!(f, " arrokcf"))}
      if self.cmpokcf() != 0 { try!(write!(f, " cmpokcf"))}
      if self.exttrigcf() != 0 { try!(write!(f, " exttrigcf"))}
      if self.arrmcf() != 0 { try!(write!(f, " arrmcf"))}
      if self.cmpmcf() != 0 { try!(write!(f, " cmpmcf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Enable Register"]
#[derive(PartialEq, Eq)]
pub struct Ier(pub u32);
impl Ier {
#[doc="Direction change to down Interrupt Enable"]
  #[inline] pub fn downie(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Direction change to down Interrupt Enable"]
  #[inline] pub fn set_downie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Direction change to UP Interrupt Enable"]
  #[inline] pub fn upie(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Direction change to UP Interrupt Enable"]
  #[inline] pub fn set_upie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Autoreload register update OK Interrupt Enable"]
  #[inline] pub fn arrokie(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Autoreload register update OK Interrupt Enable"]
  #[inline] pub fn set_arrokie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Compare register update OK Interrupt Enable"]
  #[inline] pub fn cmpokie(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Compare register update OK Interrupt Enable"]
  #[inline] pub fn set_cmpokie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="External trigger valid edge Interrupt Enable"]
  #[inline] pub fn exttrigie(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="External trigger valid edge Interrupt Enable"]
  #[inline] pub fn set_exttrigie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Autoreload match Interrupt Enable"]
  #[inline] pub fn arrmie(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Autoreload match Interrupt Enable"]
  #[inline] pub fn set_arrmie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Compare match Interrupt Enable"]
  #[inline] pub fn cmpmie(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Compare match Interrupt Enable"]
  #[inline] pub fn set_cmpmie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ier {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ier {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.downie() != 0 { try!(write!(f, " downie"))}
      if self.upie() != 0 { try!(write!(f, " upie"))}
      if self.arrokie() != 0 { try!(write!(f, " arrokie"))}
      if self.cmpokie() != 0 { try!(write!(f, " cmpokie"))}
      if self.exttrigie() != 0 { try!(write!(f, " exttrigie"))}
      if self.arrmie() != 0 { try!(write!(f, " arrmie"))}
      if self.cmpmie() != 0 { try!(write!(f, " cmpmie"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Configuration Register"]
#[derive(PartialEq, Eq)]
pub struct Cfgr(pub u32);
impl Cfgr {
#[doc="Encoder mode enable"]
  #[inline] pub fn enc(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
#[doc="Encoder mode enable"]
  #[inline] pub fn set_enc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="counter mode enabled"]
  #[inline] pub fn countmode(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
#[doc="counter mode enabled"]
  #[inline] pub fn set_countmode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="Registers update mode"]
  #[inline] pub fn preload(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="Registers update mode"]
  #[inline] pub fn set_preload(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Waveform shape polarity"]
  #[inline] pub fn wavpol(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
#[doc="Waveform shape polarity"]
  #[inline] pub fn set_wavpol(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="Waveform shape"]
  #[inline] pub fn wave(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
#[doc="Waveform shape"]
  #[inline] pub fn set_wave(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Timeout enable"]
  #[inline] pub fn timout(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
#[doc="Timeout enable"]
  #[inline] pub fn set_timout(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="Trigger enable and polarity"]
  #[inline] pub fn trigen(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x3 // [18:17]
  }
#[doc="Trigger enable and polarity"]
  #[inline] pub fn set_trigen(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="Trigger selector"]
  #[inline] pub fn trigsel(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x7 // [15:13]
  }
#[doc="Trigger selector"]
  #[inline] pub fn set_trigsel(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Clock prescaler"]
  #[inline] pub fn presc(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x7 // [11:9]
  }
#[doc="Clock prescaler"]
  #[inline] pub fn set_presc(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Configurable digital filter for trigger"]
  #[inline] pub fn trgflt(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
#[doc="Configurable digital filter for trigger"]
  #[inline] pub fn set_trgflt(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Configurable digital filter for external clock"]
  #[inline] pub fn ckflt(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x3 // [4:3]
  }
#[doc="Configurable digital filter for external clock"]
  #[inline] pub fn set_ckflt(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Clock Polarity"]
  #[inline] pub fn ckpol(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x3 // [2:1]
  }
#[doc="Clock Polarity"]
  #[inline] pub fn set_ckpol(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Clock selector"]
  #[inline] pub fn cksel(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Clock selector"]
  #[inline] pub fn set_cksel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cfgr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cfgr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.enc() != 0 { try!(write!(f, " enc"))}
      if self.countmode() != 0 { try!(write!(f, " countmode"))}
      if self.preload() != 0 { try!(write!(f, " preload"))}
      if self.wavpol() != 0 { try!(write!(f, " wavpol"))}
      if self.wave() != 0 { try!(write!(f, " wave"))}
      if self.timout() != 0 { try!(write!(f, " timout"))}
      if self.trigen() != 0 { try!(write!(f, " trigen=0x{:x}", self.trigen()))}
      if self.trigsel() != 0 { try!(write!(f, " trigsel=0x{:x}", self.trigsel()))}
      if self.presc() != 0 { try!(write!(f, " presc=0x{:x}", self.presc()))}
      if self.trgflt() != 0 { try!(write!(f, " trgflt=0x{:x}", self.trgflt()))}
      if self.ckflt() != 0 { try!(write!(f, " ckflt=0x{:x}", self.ckflt()))}
      if self.ckpol() != 0 { try!(write!(f, " ckpol=0x{:x}", self.ckpol()))}
      if self.cksel() != 0 { try!(write!(f, " cksel"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control Register"]
#[derive(PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
#[doc="Timer start in continuous mode"]
  #[inline] pub fn cntstrt(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Timer start in continuous mode"]
  #[inline] pub fn set_cntstrt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="LPTIM start in single mode"]
  #[inline] pub fn sngstrt(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="LPTIM start in single mode"]
  #[inline] pub fn set_sngstrt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="LPTIM Enable"]
  #[inline] pub fn enable(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="LPTIM Enable"]
  #[inline] pub fn set_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cntstrt() != 0 { try!(write!(f, " cntstrt"))}
      if self.sngstrt() != 0 { try!(write!(f, " sngstrt"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Compare Register"]
#[derive(PartialEq, Eq)]
pub struct Cmp(pub u32);
impl Cmp {
#[doc="Compare value."]
  #[inline] pub fn cmp(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Compare value."]
  #[inline] pub fn set_cmp(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cmp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cmp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cmp() != 0 { try!(write!(f, " cmp=0x{:x}", self.cmp()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Autoreload Register"]
#[derive(PartialEq, Eq)]
pub struct Arr(pub u32);
impl Arr {
#[doc="Auto reload value."]
  #[inline] pub fn arr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Auto reload value."]
  #[inline] pub fn set_arr(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Arr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Arr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.arr() != 0 { try!(write!(f, " arr=0x{:x}", self.arr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Counter Register"]
#[derive(PartialEq, Eq)]
pub struct Cnt(pub u32);
impl Cnt {
#[doc="Counter value."]
  #[inline] pub fn cnt(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Counter value."]
  #[inline] pub fn set_cnt(mut self, value: u32) -> Self {
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
      if self.cnt() != 0 { try!(write!(f, " cnt=0x{:x}", self.cnt()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
