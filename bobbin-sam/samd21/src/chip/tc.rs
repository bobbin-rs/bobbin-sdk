//! Timer/Counter
pub const TC3: Tc3 = Periph(0x42002c00, Tc3Id {});
pub const TC4: Tc4 = Periph(0x42003000, Tc4Id {});
pub const TC5: Tc5 = Periph(0x42003400, Tc5Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="TC Peripheral"]
pub struct Periph<T>(pub u32, pub T); 

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Tc3Id {}
pub type Tc3 = Periph<Tc3Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Tc4Id {}
pub type Tc4 = Periph<Tc4Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Tc5Id {}
pub type Tc5 = Periph<Tc5Id>;

impl super::sig::Signal<super::sig::Tc3Wo0> for Tc3Ch0 {}
impl super::sig::SignalWo<super::sig::Tc3Wo0> for Tc3Ch0 {}
impl super::sig::Signal<super::sig::Tc3Wo1> for Tc3Ch1 {}
impl super::sig::SignalWo<super::sig::Tc3Wo1> for Tc3Ch1 {}

impl super::sig::Signal<super::sig::Tc4Wo0> for Tc4Ch0 {}
impl super::sig::SignalWo<super::sig::Tc4Wo0> for Tc4Ch0 {}
impl super::sig::Signal<super::sig::Tc4Wo1> for Tc4Ch1 {}
impl super::sig::SignalWo<super::sig::Tc4Wo1> for Tc4Ch1 {}

impl super::sig::Signal<super::sig::Tc5Wo0> for Tc5Ch0 {}
impl super::sig::SignalWo<super::sig::Tc5Wo0> for Tc5Ch0 {}
impl super::sig::Signal<super::sig::Tc5Wo1> for Tc5Ch1 {}
impl super::sig::SignalWo<super::sig::Tc5Wo1> for Tc5Ch1 {}


impl<T> Periph<T> {
#[doc="Get 8-bit Counter Mode Peripheral"]
   #[inline] pub fn count8(&self) -> count8::Count8 {
      count8::Count8(self.0 + 0x0)
   }
#[doc="Get 16-bit Counter Mode Peripheral"]
   #[inline] pub fn count16(&self) -> count16::Count16 {
      count16::Count16(self.0 + 0x0)
   }
#[doc="Get 32-bit Counter Mode Peripheral"]
   #[inline] pub fn count32(&self) -> count32::Count32 {
      count32::Count32(self.0 + 0x0)
   }
}
#[doc="8-bit Counter Mode Cluster"]
pub mod count8 {
   #[derive(Clone, Copy, PartialEq, Eq)]
#[doc="8-bit Counter Mode Peripheral"]
   pub struct Count8(pub u32);
impl Count8 {
#[doc="Get the *const pointer for the CC register."]
  #[inline] pub fn cc_ptr(&self, index: usize) -> *const u8 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x18 + (index)) as *const u8
  }
#[doc="Get the *mut pointer for the CC register."]
  #[inline] pub fn cc_mut(&self, index: usize) -> *mut u8 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x18 + (index)) as *mut u8
  }
#[doc="Read the CC register."]
  #[inline] pub fn cc(&self, index: usize) -> Cc { 
     assert!(index < 2);
     unsafe {
        Cc(::core::ptr::read_volatile(((self.0 as usize) + 0x18 + (index)) as *const u8))
     }
  }
#[doc="Write the CC register."]
  #[inline] pub fn set_cc(&self, index: usize, value: Cc) -> &Self {
     assert!(index < 2);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18 + (index)) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the CC register."]
  #[inline] pub fn with_cc<F: FnOnce(Cc) -> Cc>(&self, index: usize, f: F) -> &Self {
     let tmp = self.cc(index);
     self.set_cc(index, f(tmp))
  }

#[doc="Get the *const pointer for the COUNT register."]
  #[inline] pub fn count_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x10) as *const u8
  }
#[doc="Get the *mut pointer for the COUNT register."]
  #[inline] pub fn count_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x10) as *mut u8
  }
#[doc="Read the COUNT register."]
  #[inline] pub fn count(&self) -> Count { 
     unsafe {
        Count(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u8))
     }
  }
#[doc="Write the COUNT register."]
  #[inline] pub fn set_count(&self, value: Count) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the COUNT register."]
  #[inline] pub fn with_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
     let tmp = self.count();
     self.set_count(f(tmp))
  }

#[doc="Get the *const pointer for the PER register."]
  #[inline] pub fn per_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x14) as *const u8
  }
#[doc="Get the *mut pointer for the PER register."]
  #[inline] pub fn per_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x14) as *mut u8
  }
#[doc="Read the PER register."]
  #[inline] pub fn per(&self) -> Per { 
     unsafe {
        Per(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u8))
     }
  }
#[doc="Write the PER register."]
  #[inline] pub fn set_per(&self, value: Per) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the PER register."]
  #[inline] pub fn with_per<F: FnOnce(Per) -> Per>(&self, f: F) -> &Self {
     let tmp = self.per();
     self.set_per(f(tmp))
  }

#[doc="Get the *const pointer for the CTRLA register."]
  #[inline] pub fn ctrla_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x0) as *const u16
  }
#[doc="Get the *mut pointer for the CTRLA register."]
  #[inline] pub fn ctrla_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x0) as *mut u16
  }
#[doc="Read the CTRLA register."]
  #[inline] pub fn ctrla(&self) -> Ctrla { 
     unsafe {
        Ctrla(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u16))
     }
  }
#[doc="Write the CTRLA register."]
  #[inline] pub fn set_ctrla(&self, value: Ctrla) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the CTRLA register."]
  #[inline] pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
     let tmp = self.ctrla();
     self.set_ctrla(f(tmp))
  }

#[doc="Get the *const pointer for the CTRLBCLR register."]
  #[inline] pub fn ctrlbclr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x4) as *const u8
  }
#[doc="Get the *mut pointer for the CTRLBCLR register."]
  #[inline] pub fn ctrlbclr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x4) as *mut u8
  }
#[doc="Read the CTRLBCLR register."]
  #[inline] pub fn ctrlbclr(&self) -> Ctrlbclr { 
     unsafe {
        Ctrlbclr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u8))
     }
  }
#[doc="Write the CTRLBCLR register."]
  #[inline] pub fn set_ctrlbclr(&self, value: Ctrlbclr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the CTRLBCLR register."]
  #[inline] pub fn with_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Self {
     let tmp = self.ctrlbclr();
     self.set_ctrlbclr(f(tmp))
  }

#[doc="Get the *const pointer for the CTRLBSET register."]
  #[inline] pub fn ctrlbset_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x5) as *const u8
  }
#[doc="Get the *mut pointer for the CTRLBSET register."]
  #[inline] pub fn ctrlbset_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x5) as *mut u8
  }
#[doc="Read the CTRLBSET register."]
  #[inline] pub fn ctrlbset(&self) -> Ctrlbset { 
     unsafe {
        Ctrlbset(::core::ptr::read_volatile(((self.0 as usize) + 0x5) as *const u8))
     }
  }
#[doc="Write the CTRLBSET register."]
  #[inline] pub fn set_ctrlbset(&self, value: Ctrlbset) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x5) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the CTRLBSET register."]
  #[inline] pub fn with_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Self {
     let tmp = self.ctrlbset();
     self.set_ctrlbset(f(tmp))
  }

#[doc="Get the *const pointer for the CTRLC register."]
  #[inline] pub fn ctrlc_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x6) as *const u8
  }
#[doc="Get the *mut pointer for the CTRLC register."]
  #[inline] pub fn ctrlc_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x6) as *mut u8
  }
#[doc="Read the CTRLC register."]
  #[inline] pub fn ctrlc(&self) -> Ctrlc { 
     unsafe {
        Ctrlc(::core::ptr::read_volatile(((self.0 as usize) + 0x6) as *const u8))
     }
  }
#[doc="Write the CTRLC register."]
  #[inline] pub fn set_ctrlc(&self, value: Ctrlc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x6) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the CTRLC register."]
  #[inline] pub fn with_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
     let tmp = self.ctrlc();
     self.set_ctrlc(f(tmp))
  }

#[doc="Get the *const pointer for the DBGCTRL register."]
  #[inline] pub fn dbgctrl_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x8) as *const u8
  }
#[doc="Get the *mut pointer for the DBGCTRL register."]
  #[inline] pub fn dbgctrl_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x8) as *mut u8
  }
#[doc="Read the DBGCTRL register."]
  #[inline] pub fn dbgctrl(&self) -> Dbgctrl { 
     unsafe {
        Dbgctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u8))
     }
  }
#[doc="Write the DBGCTRL register."]
  #[inline] pub fn set_dbgctrl(&self, value: Dbgctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the DBGCTRL register."]
  #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
     let tmp = self.dbgctrl();
     self.set_dbgctrl(f(tmp))
  }

#[doc="Get the *const pointer for the EVCTRL register."]
  #[inline] pub fn evctrl_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0xa) as *const u16
  }
#[doc="Get the *mut pointer for the EVCTRL register."]
  #[inline] pub fn evctrl_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0xa) as *mut u16
  }
#[doc="Read the EVCTRL register."]
  #[inline] pub fn evctrl(&self) -> Evctrl { 
     unsafe {
        Evctrl(::core::ptr::read_volatile(((self.0 as usize) + 0xa) as *const u16))
     }
  }
#[doc="Write the EVCTRL register."]
  #[inline] pub fn set_evctrl(&self, value: Evctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the EVCTRL register."]
  #[inline] pub fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
     let tmp = self.evctrl();
     self.set_evctrl(f(tmp))
  }

#[doc="Get the *const pointer for the INTENCLR register."]
  #[inline] pub fn intenclr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xc) as *const u8
  }
#[doc="Get the *mut pointer for the INTENCLR register."]
  #[inline] pub fn intenclr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xc) as *mut u8
  }
#[doc="Read the INTENCLR register."]
  #[inline] pub fn intenclr(&self) -> Intenclr { 
     unsafe {
        Intenclr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u8))
     }
  }
#[doc="Write the INTENCLR register."]
  #[inline] pub fn set_intenclr(&self, value: Intenclr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTENCLR register."]
  #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
     let tmp = self.intenclr();
     self.set_intenclr(f(tmp))
  }

#[doc="Get the *const pointer for the INTENSET register."]
  #[inline] pub fn intenset_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xd) as *const u8
  }
#[doc="Get the *mut pointer for the INTENSET register."]
  #[inline] pub fn intenset_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xd) as *mut u8
  }
#[doc="Read the INTENSET register."]
  #[inline] pub fn intenset(&self) -> Intenset { 
     unsafe {
        Intenset(::core::ptr::read_volatile(((self.0 as usize) + 0xd) as *const u8))
     }
  }
#[doc="Write the INTENSET register."]
  #[inline] pub fn set_intenset(&self, value: Intenset) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTENSET register."]
  #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
     let tmp = self.intenset();
     self.set_intenset(f(tmp))
  }

#[doc="Get the *const pointer for the INTFLAG register."]
  #[inline] pub fn intflag_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xe) as *const u8
  }
#[doc="Get the *mut pointer for the INTFLAG register."]
  #[inline] pub fn intflag_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xe) as *mut u8
  }
#[doc="Read the INTFLAG register."]
  #[inline] pub fn intflag(&self) -> Intflag { 
     unsafe {
        Intflag(::core::ptr::read_volatile(((self.0 as usize) + 0xe) as *const u8))
     }
  }
#[doc="Write the INTFLAG register."]
  #[inline] pub fn set_intflag(&self, value: Intflag) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xe) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTFLAG register."]
  #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
     let tmp = self.intflag();
     self.set_intflag(f(tmp))
  }

#[doc="Get the *const pointer for the READREQ register."]
  #[inline] pub fn readreq_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x2) as *const u16
  }
#[doc="Get the *mut pointer for the READREQ register."]
  #[inline] pub fn readreq_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x2) as *mut u16
  }
#[doc="Read the READREQ register."]
  #[inline] pub fn readreq(&self) -> Readreq { 
     unsafe {
        Readreq(::core::ptr::read_volatile(((self.0 as usize) + 0x2) as *const u16))
     }
  }
#[doc="Write the READREQ register."]
  #[inline] pub fn set_readreq(&self, value: Readreq) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the READREQ register."]
  #[inline] pub fn with_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
     let tmp = self.readreq();
     self.set_readreq(f(tmp))
  }

#[doc="Get the *const pointer for the STATUS register."]
  #[inline] pub fn status_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xf) as *const u8
  }
#[doc="Get the *mut pointer for the STATUS register."]
  #[inline] pub fn status_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xf) as *mut u8
  }
#[doc="Read the STATUS register."]
  #[inline] pub fn status(&self) -> Status { 
     unsafe {
        Status(::core::ptr::read_volatile(((self.0 as usize) + 0xf) as *const u8))
     }
  }

}

#[doc="COUNT8 Compare/Capture"]
#[derive(PartialEq, Eq)]
pub struct Cc(pub u8);
impl Cc {
#[doc="Compare/Capture Value"]
  #[inline] pub fn cc(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
#[doc="Compare/Capture Value"]
  #[inline] pub fn set_cc(mut self, value: u8) -> Self {
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
#[doc="COUNT8 Counter Value"]
#[derive(PartialEq, Eq)]
pub struct Count(pub u8);
impl Count {
#[doc="Counter Value"]
  #[inline] pub fn count(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
#[doc="Counter Value"]
  #[inline] pub fn set_count(mut self, value: u8) -> Self {
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
#[doc="COUNT8 Period Value"]
#[derive(PartialEq, Eq)]
pub struct Per(pub u8);
impl Per {
#[doc="Period Value"]
  #[inline] pub fn per(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
#[doc="Period Value"]
  #[inline] pub fn set_per(mut self, value: u8) -> Self {
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
#[doc="Control A"]
#[derive(PartialEq, Eq)]
pub struct Ctrla(pub u16);
impl Ctrla {
#[doc="Software Reset"]
  #[inline] pub fn swrst(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
#[doc="Software Reset"]
  #[inline] pub fn set_swrst(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Enable"]
  #[inline] pub fn enable(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
#[doc="Enable"]
  #[inline] pub fn set_enable(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="TC Mode"]
  #[inline] pub fn mode(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x3 // [3:2]
  }
#[doc="TC Mode"]
  #[inline] pub fn set_mode(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Waveform Generation Operation"]
  #[inline] pub fn wavegen(&self) -> u16 {
     ((self.0 as u16) >> 5) & 0x3 // [6:5]
  }
#[doc="Waveform Generation Operation"]
  #[inline] pub fn set_wavegen(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Prescaler"]
  #[inline] pub fn prescaler(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x7 // [10:8]
  }
#[doc="Prescaler"]
  #[inline] pub fn set_prescaler(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Run in Standby"]
  #[inline] pub fn runstdby(&self) -> u16 {
     ((self.0 as u16) >> 11) & 0x1 // [11]
  }
#[doc="Run in Standby"]
  #[inline] pub fn set_runstdby(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Prescaler and Counter Synchronization"]
  #[inline] pub fn prescsync(&self) -> u16 {
     ((self.0 as u16) >> 12) & 0x3 // [13:12]
  }
#[doc="Prescaler and Counter Synchronization"]
  #[inline] pub fn set_prescsync(mut self, value: u16) -> Self {
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
#[doc="Control B Clear"]
#[derive(PartialEq, Eq)]
pub struct Ctrlbclr(pub u8);
impl Ctrlbclr {
#[doc="Counter Direction"]
  #[inline] pub fn dir(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Counter Direction"]
  #[inline] pub fn set_dir(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="One-Shot"]
  #[inline] pub fn oneshot(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
#[doc="One-Shot"]
  #[inline] pub fn set_oneshot(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Command"]
  #[inline] pub fn cmd(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x3 // [7:6]
  }
#[doc="Command"]
  #[inline] pub fn set_cmd(mut self, value: u8) -> Self {
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
#[doc="Control B Set"]
#[derive(PartialEq, Eq)]
pub struct Ctrlbset(pub u8);
impl Ctrlbset {
#[doc="Counter Direction"]
  #[inline] pub fn dir(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Counter Direction"]
  #[inline] pub fn set_dir(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="One-Shot"]
  #[inline] pub fn oneshot(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
#[doc="One-Shot"]
  #[inline] pub fn set_oneshot(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Command"]
  #[inline] pub fn cmd(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x3 // [7:6]
  }
#[doc="Command"]
  #[inline] pub fn set_cmd(mut self, value: u8) -> Self {
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
#[doc="Control C"]
#[derive(PartialEq, Eq)]
pub struct Ctrlc(pub u8);
impl Ctrlc {
#[doc="Output Waveform 0 Invert Enable"]
  #[inline] pub fn inven0(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Output Waveform 0 Invert Enable"]
  #[inline] pub fn set_inven0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Output Waveform 1 Invert Enable"]
  #[inline] pub fn inven1(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Output Waveform 1 Invert Enable"]
  #[inline] pub fn set_inven1(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Capture Channel 0 Enable"]
  #[inline] pub fn cpten0(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
#[doc="Capture Channel 0 Enable"]
  #[inline] pub fn set_cpten0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Capture Channel 1 Enable"]
  #[inline] pub fn cpten1(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
#[doc="Capture Channel 1 Enable"]
  #[inline] pub fn set_cpten1(mut self, value: u8) -> Self {
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
#[doc="Debug Control"]
#[derive(PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
#[doc="Debug Run Mode"]
  #[inline] pub fn dbgrun(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Debug Run Mode"]
  #[inline] pub fn set_dbgrun(mut self, value: u8) -> Self {
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
#[doc="Event Control"]
#[derive(PartialEq, Eq)]
pub struct Evctrl(pub u16);
impl Evctrl {
#[doc="Event Action"]
  #[inline] pub fn evact(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x7 // [2:0]
  }
#[doc="Event Action"]
  #[inline] pub fn set_evact(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="TC Inverted Event Input"]
  #[inline] pub fn tcinv(&self) -> u16 {
     ((self.0 as u16) >> 4) & 0x1 // [4]
  }
#[doc="TC Inverted Event Input"]
  #[inline] pub fn set_tcinv(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="TC Event Input"]
  #[inline] pub fn tcei(&self) -> u16 {
     ((self.0 as u16) >> 5) & 0x1 // [5]
  }
#[doc="TC Event Input"]
  #[inline] pub fn set_tcei(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Overflow/Underflow Event Output Enable"]
  #[inline] pub fn ovfeo(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x1 // [8]
  }
#[doc="Overflow/Underflow Event Output Enable"]
  #[inline] pub fn set_ovfeo(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Match or Capture Channel 0 Event Output Enable"]
  #[inline] pub fn mceo0(&self) -> u16 {
     ((self.0 as u16) >> 12) & 0x1 // [12]
  }
#[doc="Match or Capture Channel 0 Event Output Enable"]
  #[inline] pub fn set_mceo0(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Match or Capture Channel 1 Event Output Enable"]
  #[inline] pub fn mceo1(&self) -> u16 {
     ((self.0 as u16) >> 13) & 0x1 // [13]
  }
#[doc="Match or Capture Channel 1 Event Output Enable"]
  #[inline] pub fn set_mceo1(mut self, value: u16) -> Self {
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
#[doc="Interrupt Enable Clear"]
#[derive(PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
#[doc="Overflow Interrupt Enable"]
  #[inline] pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Overflow Interrupt Enable"]
  #[inline] pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Error Interrupt Enable"]
  #[inline] pub fn err(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Error Interrupt Enable"]
  #[inline] pub fn set_err(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Synchronization Ready Interrupt Enable"]
  #[inline] pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
#[doc="Synchronization Ready Interrupt Enable"]
  #[inline] pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Match or Capture Channel 0 Interrupt Enable"]
  #[inline] pub fn mc0(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
#[doc="Match or Capture Channel 0 Interrupt Enable"]
  #[inline] pub fn set_mc0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Match or Capture Channel 1 Interrupt Enable"]
  #[inline] pub fn mc1(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
#[doc="Match or Capture Channel 1 Interrupt Enable"]
  #[inline] pub fn set_mc1(mut self, value: u8) -> Self {
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
#[doc="Interrupt Enable Set"]
#[derive(PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
#[doc="Overflow Interrupt Enable"]
  #[inline] pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Overflow Interrupt Enable"]
  #[inline] pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Error Interrupt Enable"]
  #[inline] pub fn err(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Error Interrupt Enable"]
  #[inline] pub fn set_err(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Synchronization Ready Interrupt Enable"]
  #[inline] pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
#[doc="Synchronization Ready Interrupt Enable"]
  #[inline] pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Match or Capture Channel 0 Interrupt Enable"]
  #[inline] pub fn mc0(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
#[doc="Match or Capture Channel 0 Interrupt Enable"]
  #[inline] pub fn set_mc0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Match or Capture Channel 1 Interrupt Enable"]
  #[inline] pub fn mc1(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
#[doc="Match or Capture Channel 1 Interrupt Enable"]
  #[inline] pub fn set_mc1(mut self, value: u8) -> Self {
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
#[doc="Interrupt Flag Status and Clear"]
#[derive(PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
#[doc="Overflow"]
  #[inline] pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Overflow"]
  #[inline] pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Error"]
  #[inline] pub fn err(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Error"]
  #[inline] pub fn set_err(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Synchronization Ready"]
  #[inline] pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
#[doc="Synchronization Ready"]
  #[inline] pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Match or Capture Channel 0"]
  #[inline] pub fn mc0(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
#[doc="Match or Capture Channel 0"]
  #[inline] pub fn set_mc0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Match or Capture Channel 1"]
  #[inline] pub fn mc1(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
#[doc="Match or Capture Channel 1"]
  #[inline] pub fn set_mc1(mut self, value: u8) -> Self {
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
#[doc="Read Request"]
#[derive(PartialEq, Eq)]
pub struct Readreq(pub u16);
impl Readreq {
#[doc="Address"]
  #[inline] pub fn addr(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1f // [4:0]
  }
#[doc="Address"]
  #[inline] pub fn set_addr(mut self, value: u16) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Read Continuously"]
  #[inline] pub fn rcont(&self) -> u16 {
     ((self.0 as u16) >> 14) & 0x1 // [14]
  }
#[doc="Read Continuously"]
  #[inline] pub fn set_rcont(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Read Request"]
  #[inline] pub fn rreq(&self) -> u16 {
     ((self.0 as u16) >> 15) & 0x1 // [15]
  }
#[doc="Read Request"]
  #[inline] pub fn set_rreq(mut self, value: u16) -> Self {
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
#[doc="Status"]
#[derive(PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
#[doc="Stop"]
  #[inline] pub fn stop(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
#[doc="Stop"]
  #[inline] pub fn set_stop(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Slave"]
  #[inline] pub fn slave(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
#[doc="Slave"]
  #[inline] pub fn set_slave(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Synchronization Busy"]
  #[inline] pub fn syncbusy(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
#[doc="Synchronization Busy"]
  #[inline] pub fn set_syncbusy(mut self, value: u8) -> Self {
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
#[doc="16-bit Counter Mode Cluster"]
pub mod count16 {
   #[derive(Clone, Copy, PartialEq, Eq)]
#[doc="16-bit Counter Mode Peripheral"]
   pub struct Count16(pub u32);
impl Count16 {
#[doc="Get the *const pointer for the CC register."]
  #[inline] pub fn cc_ptr(&self, index: usize) -> *const u16 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x18 + (index << 1)) as *const u16
  }
#[doc="Get the *mut pointer for the CC register."]
  #[inline] pub fn cc_mut(&self, index: usize) -> *mut u16 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x18 + (index << 1)) as *mut u16
  }
#[doc="Read the CC register."]
  #[inline] pub fn cc(&self, index: usize) -> Cc { 
     assert!(index < 2);
     unsafe {
        Cc(::core::ptr::read_volatile(((self.0 as usize) + 0x18 + (index << 1)) as *const u16))
     }
  }
#[doc="Write the CC register."]
  #[inline] pub fn set_cc(&self, index: usize, value: Cc) -> &Self {
     assert!(index < 2);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18 + (index << 1)) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the CC register."]
  #[inline] pub fn with_cc<F: FnOnce(Cc) -> Cc>(&self, index: usize, f: F) -> &Self {
     let tmp = self.cc(index);
     self.set_cc(index, f(tmp))
  }

#[doc="Get the *const pointer for the COUNT register."]
  #[inline] pub fn count_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x10) as *const u16
  }
#[doc="Get the *mut pointer for the COUNT register."]
  #[inline] pub fn count_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x10) as *mut u16
  }
#[doc="Read the COUNT register."]
  #[inline] pub fn count(&self) -> Count { 
     unsafe {
        Count(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u16))
     }
  }
#[doc="Write the COUNT register."]
  #[inline] pub fn set_count(&self, value: Count) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the COUNT register."]
  #[inline] pub fn with_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
     let tmp = self.count();
     self.set_count(f(tmp))
  }

#[doc="Get the *const pointer for the CTRLA register."]
  #[inline] pub fn ctrla_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x0) as *const u16
  }
#[doc="Get the *mut pointer for the CTRLA register."]
  #[inline] pub fn ctrla_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x0) as *mut u16
  }
#[doc="Read the CTRLA register."]
  #[inline] pub fn ctrla(&self) -> Ctrla { 
     unsafe {
        Ctrla(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u16))
     }
  }
#[doc="Write the CTRLA register."]
  #[inline] pub fn set_ctrla(&self, value: Ctrla) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the CTRLA register."]
  #[inline] pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
     let tmp = self.ctrla();
     self.set_ctrla(f(tmp))
  }

#[doc="Get the *const pointer for the CTRLBCLR register."]
  #[inline] pub fn ctrlbclr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x4) as *const u8
  }
#[doc="Get the *mut pointer for the CTRLBCLR register."]
  #[inline] pub fn ctrlbclr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x4) as *mut u8
  }
#[doc="Read the CTRLBCLR register."]
  #[inline] pub fn ctrlbclr(&self) -> Ctrlbclr { 
     unsafe {
        Ctrlbclr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u8))
     }
  }
#[doc="Write the CTRLBCLR register."]
  #[inline] pub fn set_ctrlbclr(&self, value: Ctrlbclr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the CTRLBCLR register."]
  #[inline] pub fn with_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Self {
     let tmp = self.ctrlbclr();
     self.set_ctrlbclr(f(tmp))
  }

#[doc="Get the *const pointer for the CTRLBSET register."]
  #[inline] pub fn ctrlbset_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x5) as *const u8
  }
#[doc="Get the *mut pointer for the CTRLBSET register."]
  #[inline] pub fn ctrlbset_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x5) as *mut u8
  }
#[doc="Read the CTRLBSET register."]
  #[inline] pub fn ctrlbset(&self) -> Ctrlbset { 
     unsafe {
        Ctrlbset(::core::ptr::read_volatile(((self.0 as usize) + 0x5) as *const u8))
     }
  }
#[doc="Write the CTRLBSET register."]
  #[inline] pub fn set_ctrlbset(&self, value: Ctrlbset) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x5) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the CTRLBSET register."]
  #[inline] pub fn with_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Self {
     let tmp = self.ctrlbset();
     self.set_ctrlbset(f(tmp))
  }

#[doc="Get the *const pointer for the CTRLC register."]
  #[inline] pub fn ctrlc_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x6) as *const u8
  }
#[doc="Get the *mut pointer for the CTRLC register."]
  #[inline] pub fn ctrlc_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x6) as *mut u8
  }
#[doc="Read the CTRLC register."]
  #[inline] pub fn ctrlc(&self) -> Ctrlc { 
     unsafe {
        Ctrlc(::core::ptr::read_volatile(((self.0 as usize) + 0x6) as *const u8))
     }
  }
#[doc="Write the CTRLC register."]
  #[inline] pub fn set_ctrlc(&self, value: Ctrlc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x6) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the CTRLC register."]
  #[inline] pub fn with_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
     let tmp = self.ctrlc();
     self.set_ctrlc(f(tmp))
  }

#[doc="Get the *const pointer for the DBGCTRL register."]
  #[inline] pub fn dbgctrl_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x8) as *const u8
  }
#[doc="Get the *mut pointer for the DBGCTRL register."]
  #[inline] pub fn dbgctrl_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x8) as *mut u8
  }
#[doc="Read the DBGCTRL register."]
  #[inline] pub fn dbgctrl(&self) -> Dbgctrl { 
     unsafe {
        Dbgctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u8))
     }
  }
#[doc="Write the DBGCTRL register."]
  #[inline] pub fn set_dbgctrl(&self, value: Dbgctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the DBGCTRL register."]
  #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
     let tmp = self.dbgctrl();
     self.set_dbgctrl(f(tmp))
  }

#[doc="Get the *const pointer for the EVCTRL register."]
  #[inline] pub fn evctrl_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0xa) as *const u16
  }
#[doc="Get the *mut pointer for the EVCTRL register."]
  #[inline] pub fn evctrl_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0xa) as *mut u16
  }
#[doc="Read the EVCTRL register."]
  #[inline] pub fn evctrl(&self) -> Evctrl { 
     unsafe {
        Evctrl(::core::ptr::read_volatile(((self.0 as usize) + 0xa) as *const u16))
     }
  }
#[doc="Write the EVCTRL register."]
  #[inline] pub fn set_evctrl(&self, value: Evctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the EVCTRL register."]
  #[inline] pub fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
     let tmp = self.evctrl();
     self.set_evctrl(f(tmp))
  }

#[doc="Get the *const pointer for the INTENCLR register."]
  #[inline] pub fn intenclr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xc) as *const u8
  }
#[doc="Get the *mut pointer for the INTENCLR register."]
  #[inline] pub fn intenclr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xc) as *mut u8
  }
#[doc="Read the INTENCLR register."]
  #[inline] pub fn intenclr(&self) -> Intenclr { 
     unsafe {
        Intenclr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u8))
     }
  }
#[doc="Write the INTENCLR register."]
  #[inline] pub fn set_intenclr(&self, value: Intenclr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTENCLR register."]
  #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
     let tmp = self.intenclr();
     self.set_intenclr(f(tmp))
  }

#[doc="Get the *const pointer for the INTENSET register."]
  #[inline] pub fn intenset_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xd) as *const u8
  }
#[doc="Get the *mut pointer for the INTENSET register."]
  #[inline] pub fn intenset_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xd) as *mut u8
  }
#[doc="Read the INTENSET register."]
  #[inline] pub fn intenset(&self) -> Intenset { 
     unsafe {
        Intenset(::core::ptr::read_volatile(((self.0 as usize) + 0xd) as *const u8))
     }
  }
#[doc="Write the INTENSET register."]
  #[inline] pub fn set_intenset(&self, value: Intenset) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTENSET register."]
  #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
     let tmp = self.intenset();
     self.set_intenset(f(tmp))
  }

#[doc="Get the *const pointer for the INTFLAG register."]
  #[inline] pub fn intflag_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xe) as *const u8
  }
#[doc="Get the *mut pointer for the INTFLAG register."]
  #[inline] pub fn intflag_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xe) as *mut u8
  }
#[doc="Read the INTFLAG register."]
  #[inline] pub fn intflag(&self) -> Intflag { 
     unsafe {
        Intflag(::core::ptr::read_volatile(((self.0 as usize) + 0xe) as *const u8))
     }
  }
#[doc="Write the INTFLAG register."]
  #[inline] pub fn set_intflag(&self, value: Intflag) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xe) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTFLAG register."]
  #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
     let tmp = self.intflag();
     self.set_intflag(f(tmp))
  }

#[doc="Get the *const pointer for the READREQ register."]
  #[inline] pub fn readreq_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x2) as *const u16
  }
#[doc="Get the *mut pointer for the READREQ register."]
  #[inline] pub fn readreq_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x2) as *mut u16
  }
#[doc="Read the READREQ register."]
  #[inline] pub fn readreq(&self) -> Readreq { 
     unsafe {
        Readreq(::core::ptr::read_volatile(((self.0 as usize) + 0x2) as *const u16))
     }
  }
#[doc="Write the READREQ register."]
  #[inline] pub fn set_readreq(&self, value: Readreq) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the READREQ register."]
  #[inline] pub fn with_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
     let tmp = self.readreq();
     self.set_readreq(f(tmp))
  }

#[doc="Get the *const pointer for the STATUS register."]
  #[inline] pub fn status_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xf) as *const u8
  }
#[doc="Get the *mut pointer for the STATUS register."]
  #[inline] pub fn status_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xf) as *mut u8
  }
#[doc="Read the STATUS register."]
  #[inline] pub fn status(&self) -> Status { 
     unsafe {
        Status(::core::ptr::read_volatile(((self.0 as usize) + 0xf) as *const u8))
     }
  }

}

#[doc="COUNT16 Compare/Capture"]
#[derive(PartialEq, Eq)]
pub struct Cc(pub u16);
impl Cc {
#[doc="Compare/Capture Value"]
  #[inline] pub fn cc(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
#[doc="Compare/Capture Value"]
  #[inline] pub fn set_cc(mut self, value: u16) -> Self {
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
#[doc="COUNT16 Counter Value"]
#[derive(PartialEq, Eq)]
pub struct Count(pub u16);
impl Count {
#[doc="Count Value"]
  #[inline] pub fn count(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
#[doc="Count Value"]
  #[inline] pub fn set_count(mut self, value: u16) -> Self {
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
#[doc="Control A"]
#[derive(PartialEq, Eq)]
pub struct Ctrla(pub u16);
impl Ctrla {
#[doc="Software Reset"]
  #[inline] pub fn swrst(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
#[doc="Software Reset"]
  #[inline] pub fn set_swrst(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Enable"]
  #[inline] pub fn enable(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
#[doc="Enable"]
  #[inline] pub fn set_enable(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="TC Mode"]
  #[inline] pub fn mode(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x3 // [3:2]
  }
#[doc="TC Mode"]
  #[inline] pub fn set_mode(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Waveform Generation Operation"]
  #[inline] pub fn wavegen(&self) -> u16 {
     ((self.0 as u16) >> 5) & 0x3 // [6:5]
  }
#[doc="Waveform Generation Operation"]
  #[inline] pub fn set_wavegen(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Prescaler"]
  #[inline] pub fn prescaler(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x7 // [10:8]
  }
#[doc="Prescaler"]
  #[inline] pub fn set_prescaler(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Run in Standby"]
  #[inline] pub fn runstdby(&self) -> u16 {
     ((self.0 as u16) >> 11) & 0x1 // [11]
  }
#[doc="Run in Standby"]
  #[inline] pub fn set_runstdby(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Prescaler and Counter Synchronization"]
  #[inline] pub fn prescsync(&self) -> u16 {
     ((self.0 as u16) >> 12) & 0x3 // [13:12]
  }
#[doc="Prescaler and Counter Synchronization"]
  #[inline] pub fn set_prescsync(mut self, value: u16) -> Self {
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
#[doc="Control B Clear"]
#[derive(PartialEq, Eq)]
pub struct Ctrlbclr(pub u8);
impl Ctrlbclr {
#[doc="Counter Direction"]
  #[inline] pub fn dir(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Counter Direction"]
  #[inline] pub fn set_dir(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="One-Shot"]
  #[inline] pub fn oneshot(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
#[doc="One-Shot"]
  #[inline] pub fn set_oneshot(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Command"]
  #[inline] pub fn cmd(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x3 // [7:6]
  }
#[doc="Command"]
  #[inline] pub fn set_cmd(mut self, value: u8) -> Self {
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
#[doc="Control B Set"]
#[derive(PartialEq, Eq)]
pub struct Ctrlbset(pub u8);
impl Ctrlbset {
#[doc="Counter Direction"]
  #[inline] pub fn dir(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Counter Direction"]
  #[inline] pub fn set_dir(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="One-Shot"]
  #[inline] pub fn oneshot(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
#[doc="One-Shot"]
  #[inline] pub fn set_oneshot(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Command"]
  #[inline] pub fn cmd(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x3 // [7:6]
  }
#[doc="Command"]
  #[inline] pub fn set_cmd(mut self, value: u8) -> Self {
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
#[doc="Control C"]
#[derive(PartialEq, Eq)]
pub struct Ctrlc(pub u8);
impl Ctrlc {
#[doc="Output Waveform 0 Invert Enable"]
  #[inline] pub fn inven0(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Output Waveform 0 Invert Enable"]
  #[inline] pub fn set_inven0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Output Waveform 1 Invert Enable"]
  #[inline] pub fn inven1(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Output Waveform 1 Invert Enable"]
  #[inline] pub fn set_inven1(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Capture Channel 0 Enable"]
  #[inline] pub fn cpten0(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
#[doc="Capture Channel 0 Enable"]
  #[inline] pub fn set_cpten0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Capture Channel 1 Enable"]
  #[inline] pub fn cpten1(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
#[doc="Capture Channel 1 Enable"]
  #[inline] pub fn set_cpten1(mut self, value: u8) -> Self {
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
#[doc="Debug Control"]
#[derive(PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
#[doc="Debug Run Mode"]
  #[inline] pub fn dbgrun(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Debug Run Mode"]
  #[inline] pub fn set_dbgrun(mut self, value: u8) -> Self {
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
#[doc="Event Control"]
#[derive(PartialEq, Eq)]
pub struct Evctrl(pub u16);
impl Evctrl {
#[doc="Event Action"]
  #[inline] pub fn evact(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x7 // [2:0]
  }
#[doc="Event Action"]
  #[inline] pub fn set_evact(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="TC Inverted Event Input"]
  #[inline] pub fn tcinv(&self) -> u16 {
     ((self.0 as u16) >> 4) & 0x1 // [4]
  }
#[doc="TC Inverted Event Input"]
  #[inline] pub fn set_tcinv(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="TC Event Input"]
  #[inline] pub fn tcei(&self) -> u16 {
     ((self.0 as u16) >> 5) & 0x1 // [5]
  }
#[doc="TC Event Input"]
  #[inline] pub fn set_tcei(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Overflow/Underflow Event Output Enable"]
  #[inline] pub fn ovfeo(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x1 // [8]
  }
#[doc="Overflow/Underflow Event Output Enable"]
  #[inline] pub fn set_ovfeo(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Match or Capture Channel 0 Event Output Enable"]
  #[inline] pub fn mceo0(&self) -> u16 {
     ((self.0 as u16) >> 12) & 0x1 // [12]
  }
#[doc="Match or Capture Channel 0 Event Output Enable"]
  #[inline] pub fn set_mceo0(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Match or Capture Channel 1 Event Output Enable"]
  #[inline] pub fn mceo1(&self) -> u16 {
     ((self.0 as u16) >> 13) & 0x1 // [13]
  }
#[doc="Match or Capture Channel 1 Event Output Enable"]
  #[inline] pub fn set_mceo1(mut self, value: u16) -> Self {
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
#[doc="Interrupt Enable Clear"]
#[derive(PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
#[doc="Overflow Interrupt Enable"]
  #[inline] pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Overflow Interrupt Enable"]
  #[inline] pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Error Interrupt Enable"]
  #[inline] pub fn err(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Error Interrupt Enable"]
  #[inline] pub fn set_err(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Synchronization Ready Interrupt Enable"]
  #[inline] pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
#[doc="Synchronization Ready Interrupt Enable"]
  #[inline] pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Match or Capture Channel 0 Interrupt Enable"]
  #[inline] pub fn mc0(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
#[doc="Match or Capture Channel 0 Interrupt Enable"]
  #[inline] pub fn set_mc0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Match or Capture Channel 1 Interrupt Enable"]
  #[inline] pub fn mc1(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
#[doc="Match or Capture Channel 1 Interrupt Enable"]
  #[inline] pub fn set_mc1(mut self, value: u8) -> Self {
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
#[doc="Interrupt Enable Set"]
#[derive(PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
#[doc="Overflow Interrupt Enable"]
  #[inline] pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Overflow Interrupt Enable"]
  #[inline] pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Error Interrupt Enable"]
  #[inline] pub fn err(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Error Interrupt Enable"]
  #[inline] pub fn set_err(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Synchronization Ready Interrupt Enable"]
  #[inline] pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
#[doc="Synchronization Ready Interrupt Enable"]
  #[inline] pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Match or Capture Channel 0 Interrupt Enable"]
  #[inline] pub fn mc0(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
#[doc="Match or Capture Channel 0 Interrupt Enable"]
  #[inline] pub fn set_mc0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Match or Capture Channel 1 Interrupt Enable"]
  #[inline] pub fn mc1(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
#[doc="Match or Capture Channel 1 Interrupt Enable"]
  #[inline] pub fn set_mc1(mut self, value: u8) -> Self {
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
#[doc="Interrupt Flag Status and Clear"]
#[derive(PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
#[doc="Overflow"]
  #[inline] pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Overflow"]
  #[inline] pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Error"]
  #[inline] pub fn err(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Error"]
  #[inline] pub fn set_err(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Synchronization Ready"]
  #[inline] pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
#[doc="Synchronization Ready"]
  #[inline] pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Match or Capture Channel 0"]
  #[inline] pub fn mc0(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
#[doc="Match or Capture Channel 0"]
  #[inline] pub fn set_mc0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Match or Capture Channel 1"]
  #[inline] pub fn mc1(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
#[doc="Match or Capture Channel 1"]
  #[inline] pub fn set_mc1(mut self, value: u8) -> Self {
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
#[doc="Read Request"]
#[derive(PartialEq, Eq)]
pub struct Readreq(pub u16);
impl Readreq {
#[doc="Address"]
  #[inline] pub fn addr(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1f // [4:0]
  }
#[doc="Address"]
  #[inline] pub fn set_addr(mut self, value: u16) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Read Continuously"]
  #[inline] pub fn rcont(&self) -> u16 {
     ((self.0 as u16) >> 14) & 0x1 // [14]
  }
#[doc="Read Continuously"]
  #[inline] pub fn set_rcont(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Read Request"]
  #[inline] pub fn rreq(&self) -> u16 {
     ((self.0 as u16) >> 15) & 0x1 // [15]
  }
#[doc="Read Request"]
  #[inline] pub fn set_rreq(mut self, value: u16) -> Self {
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
#[doc="Status"]
#[derive(PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
#[doc="Stop"]
  #[inline] pub fn stop(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
#[doc="Stop"]
  #[inline] pub fn set_stop(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Slave"]
  #[inline] pub fn slave(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
#[doc="Slave"]
  #[inline] pub fn set_slave(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Synchronization Busy"]
  #[inline] pub fn syncbusy(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
#[doc="Synchronization Busy"]
  #[inline] pub fn set_syncbusy(mut self, value: u8) -> Self {
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
#[doc="32-bit Counter Mode Cluster"]
pub mod count32 {
   #[derive(Clone, Copy, PartialEq, Eq)]
#[doc="32-bit Counter Mode Peripheral"]
   pub struct Count32(pub u32);
impl Count32 {
#[doc="Get the *const pointer for the CC register."]
  #[inline] pub fn cc_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x18 + (index << 2)) as *const u32
  }
#[doc="Get the *mut pointer for the CC register."]
  #[inline] pub fn cc_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 2);
     ((self.0 as usize) + 0x18 + (index << 2)) as *mut u32
  }
#[doc="Read the CC register."]
  #[inline] pub fn cc(&self, index: usize) -> Cc { 
     assert!(index < 2);
     unsafe {
        Cc(::core::ptr::read_volatile(((self.0 as usize) + 0x18 + (index << 2)) as *const u32))
     }
  }
#[doc="Write the CC register."]
  #[inline] pub fn set_cc(&self, index: usize, value: Cc) -> &Self {
     assert!(index < 2);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CC register."]
  #[inline] pub fn with_cc<F: FnOnce(Cc) -> Cc>(&self, index: usize, f: F) -> &Self {
     let tmp = self.cc(index);
     self.set_cc(index, f(tmp))
  }

#[doc="Get the *const pointer for the COUNT register."]
  #[inline] pub fn count_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the COUNT register."]
  #[inline] pub fn count_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the COUNT register."]
  #[inline] pub fn count(&self) -> Count { 
     unsafe {
        Count(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the COUNT register."]
  #[inline] pub fn set_count(&self, value: Count) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the COUNT register."]
  #[inline] pub fn with_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
     let tmp = self.count();
     self.set_count(f(tmp))
  }

#[doc="Get the *const pointer for the CTRLA register."]
  #[inline] pub fn ctrla_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x0) as *const u16
  }
#[doc="Get the *mut pointer for the CTRLA register."]
  #[inline] pub fn ctrla_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x0) as *mut u16
  }
#[doc="Read the CTRLA register."]
  #[inline] pub fn ctrla(&self) -> Ctrla { 
     unsafe {
        Ctrla(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u16))
     }
  }
#[doc="Write the CTRLA register."]
  #[inline] pub fn set_ctrla(&self, value: Ctrla) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the CTRLA register."]
  #[inline] pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
     let tmp = self.ctrla();
     self.set_ctrla(f(tmp))
  }

#[doc="Get the *const pointer for the CTRLBCLR register."]
  #[inline] pub fn ctrlbclr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x4) as *const u8
  }
#[doc="Get the *mut pointer for the CTRLBCLR register."]
  #[inline] pub fn ctrlbclr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x4) as *mut u8
  }
#[doc="Read the CTRLBCLR register."]
  #[inline] pub fn ctrlbclr(&self) -> Ctrlbclr { 
     unsafe {
        Ctrlbclr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u8))
     }
  }
#[doc="Write the CTRLBCLR register."]
  #[inline] pub fn set_ctrlbclr(&self, value: Ctrlbclr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the CTRLBCLR register."]
  #[inline] pub fn with_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Self {
     let tmp = self.ctrlbclr();
     self.set_ctrlbclr(f(tmp))
  }

#[doc="Get the *const pointer for the CTRLBSET register."]
  #[inline] pub fn ctrlbset_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x5) as *const u8
  }
#[doc="Get the *mut pointer for the CTRLBSET register."]
  #[inline] pub fn ctrlbset_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x5) as *mut u8
  }
#[doc="Read the CTRLBSET register."]
  #[inline] pub fn ctrlbset(&self) -> Ctrlbset { 
     unsafe {
        Ctrlbset(::core::ptr::read_volatile(((self.0 as usize) + 0x5) as *const u8))
     }
  }
#[doc="Write the CTRLBSET register."]
  #[inline] pub fn set_ctrlbset(&self, value: Ctrlbset) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x5) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the CTRLBSET register."]
  #[inline] pub fn with_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Self {
     let tmp = self.ctrlbset();
     self.set_ctrlbset(f(tmp))
  }

#[doc="Get the *const pointer for the CTRLC register."]
  #[inline] pub fn ctrlc_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x6) as *const u8
  }
#[doc="Get the *mut pointer for the CTRLC register."]
  #[inline] pub fn ctrlc_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x6) as *mut u8
  }
#[doc="Read the CTRLC register."]
  #[inline] pub fn ctrlc(&self) -> Ctrlc { 
     unsafe {
        Ctrlc(::core::ptr::read_volatile(((self.0 as usize) + 0x6) as *const u8))
     }
  }
#[doc="Write the CTRLC register."]
  #[inline] pub fn set_ctrlc(&self, value: Ctrlc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x6) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the CTRLC register."]
  #[inline] pub fn with_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
     let tmp = self.ctrlc();
     self.set_ctrlc(f(tmp))
  }

#[doc="Get the *const pointer for the DBGCTRL register."]
  #[inline] pub fn dbgctrl_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x8) as *const u8
  }
#[doc="Get the *mut pointer for the DBGCTRL register."]
  #[inline] pub fn dbgctrl_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x8) as *mut u8
  }
#[doc="Read the DBGCTRL register."]
  #[inline] pub fn dbgctrl(&self) -> Dbgctrl { 
     unsafe {
        Dbgctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u8))
     }
  }
#[doc="Write the DBGCTRL register."]
  #[inline] pub fn set_dbgctrl(&self, value: Dbgctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the DBGCTRL register."]
  #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
     let tmp = self.dbgctrl();
     self.set_dbgctrl(f(tmp))
  }

#[doc="Get the *const pointer for the EVCTRL register."]
  #[inline] pub fn evctrl_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0xa) as *const u16
  }
#[doc="Get the *mut pointer for the EVCTRL register."]
  #[inline] pub fn evctrl_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0xa) as *mut u16
  }
#[doc="Read the EVCTRL register."]
  #[inline] pub fn evctrl(&self) -> Evctrl { 
     unsafe {
        Evctrl(::core::ptr::read_volatile(((self.0 as usize) + 0xa) as *const u16))
     }
  }
#[doc="Write the EVCTRL register."]
  #[inline] pub fn set_evctrl(&self, value: Evctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the EVCTRL register."]
  #[inline] pub fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
     let tmp = self.evctrl();
     self.set_evctrl(f(tmp))
  }

#[doc="Get the *const pointer for the INTENCLR register."]
  #[inline] pub fn intenclr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xc) as *const u8
  }
#[doc="Get the *mut pointer for the INTENCLR register."]
  #[inline] pub fn intenclr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xc) as *mut u8
  }
#[doc="Read the INTENCLR register."]
  #[inline] pub fn intenclr(&self) -> Intenclr { 
     unsafe {
        Intenclr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u8))
     }
  }
#[doc="Write the INTENCLR register."]
  #[inline] pub fn set_intenclr(&self, value: Intenclr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTENCLR register."]
  #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
     let tmp = self.intenclr();
     self.set_intenclr(f(tmp))
  }

#[doc="Get the *const pointer for the INTENSET register."]
  #[inline] pub fn intenset_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xd) as *const u8
  }
#[doc="Get the *mut pointer for the INTENSET register."]
  #[inline] pub fn intenset_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xd) as *mut u8
  }
#[doc="Read the INTENSET register."]
  #[inline] pub fn intenset(&self) -> Intenset { 
     unsafe {
        Intenset(::core::ptr::read_volatile(((self.0 as usize) + 0xd) as *const u8))
     }
  }
#[doc="Write the INTENSET register."]
  #[inline] pub fn set_intenset(&self, value: Intenset) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTENSET register."]
  #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
     let tmp = self.intenset();
     self.set_intenset(f(tmp))
  }

#[doc="Get the *const pointer for the INTFLAG register."]
  #[inline] pub fn intflag_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xe) as *const u8
  }
#[doc="Get the *mut pointer for the INTFLAG register."]
  #[inline] pub fn intflag_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xe) as *mut u8
  }
#[doc="Read the INTFLAG register."]
  #[inline] pub fn intflag(&self) -> Intflag { 
     unsafe {
        Intflag(::core::ptr::read_volatile(((self.0 as usize) + 0xe) as *const u8))
     }
  }
#[doc="Write the INTFLAG register."]
  #[inline] pub fn set_intflag(&self, value: Intflag) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xe) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTFLAG register."]
  #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
     let tmp = self.intflag();
     self.set_intflag(f(tmp))
  }

#[doc="Get the *const pointer for the READREQ register."]
  #[inline] pub fn readreq_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x2) as *const u16
  }
#[doc="Get the *mut pointer for the READREQ register."]
  #[inline] pub fn readreq_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x2) as *mut u16
  }
#[doc="Read the READREQ register."]
  #[inline] pub fn readreq(&self) -> Readreq { 
     unsafe {
        Readreq(::core::ptr::read_volatile(((self.0 as usize) + 0x2) as *const u16))
     }
  }
#[doc="Write the READREQ register."]
  #[inline] pub fn set_readreq(&self, value: Readreq) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the READREQ register."]
  #[inline] pub fn with_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
     let tmp = self.readreq();
     self.set_readreq(f(tmp))
  }

#[doc="Get the *const pointer for the STATUS register."]
  #[inline] pub fn status_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xf) as *const u8
  }
#[doc="Get the *mut pointer for the STATUS register."]
  #[inline] pub fn status_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xf) as *mut u8
  }
#[doc="Read the STATUS register."]
  #[inline] pub fn status(&self) -> Status { 
     unsafe {
        Status(::core::ptr::read_volatile(((self.0 as usize) + 0xf) as *const u8))
     }
  }

}

#[doc="COUNT32 Compare/Capture"]
#[derive(PartialEq, Eq)]
pub struct Cc(pub u32);
impl Cc {
#[doc="Compare/Capture Value"]
  #[inline] pub fn cc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Compare/Capture Value"]
  #[inline] pub fn set_cc(mut self, value: u32) -> Self {
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
#[doc="COUNT32 Counter Value"]
#[derive(PartialEq, Eq)]
pub struct Count(pub u32);
impl Count {
#[doc="Count Value"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Count Value"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
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
#[doc="Control A"]
#[derive(PartialEq, Eq)]
pub struct Ctrla(pub u16);
impl Ctrla {
#[doc="Software Reset"]
  #[inline] pub fn swrst(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
#[doc="Software Reset"]
  #[inline] pub fn set_swrst(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Enable"]
  #[inline] pub fn enable(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
#[doc="Enable"]
  #[inline] pub fn set_enable(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="TC Mode"]
  #[inline] pub fn mode(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x3 // [3:2]
  }
#[doc="TC Mode"]
  #[inline] pub fn set_mode(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Waveform Generation Operation"]
  #[inline] pub fn wavegen(&self) -> u16 {
     ((self.0 as u16) >> 5) & 0x3 // [6:5]
  }
#[doc="Waveform Generation Operation"]
  #[inline] pub fn set_wavegen(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Prescaler"]
  #[inline] pub fn prescaler(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x7 // [10:8]
  }
#[doc="Prescaler"]
  #[inline] pub fn set_prescaler(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Run in Standby"]
  #[inline] pub fn runstdby(&self) -> u16 {
     ((self.0 as u16) >> 11) & 0x1 // [11]
  }
#[doc="Run in Standby"]
  #[inline] pub fn set_runstdby(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Prescaler and Counter Synchronization"]
  #[inline] pub fn prescsync(&self) -> u16 {
     ((self.0 as u16) >> 12) & 0x3 // [13:12]
  }
#[doc="Prescaler and Counter Synchronization"]
  #[inline] pub fn set_prescsync(mut self, value: u16) -> Self {
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
#[doc="Control B Clear"]
#[derive(PartialEq, Eq)]
pub struct Ctrlbclr(pub u8);
impl Ctrlbclr {
#[doc="Counter Direction"]
  #[inline] pub fn dir(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Counter Direction"]
  #[inline] pub fn set_dir(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="One-Shot"]
  #[inline] pub fn oneshot(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
#[doc="One-Shot"]
  #[inline] pub fn set_oneshot(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Command"]
  #[inline] pub fn cmd(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x3 // [7:6]
  }
#[doc="Command"]
  #[inline] pub fn set_cmd(mut self, value: u8) -> Self {
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
#[doc="Control B Set"]
#[derive(PartialEq, Eq)]
pub struct Ctrlbset(pub u8);
impl Ctrlbset {
#[doc="Counter Direction"]
  #[inline] pub fn dir(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Counter Direction"]
  #[inline] pub fn set_dir(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="One-Shot"]
  #[inline] pub fn oneshot(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
#[doc="One-Shot"]
  #[inline] pub fn set_oneshot(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Command"]
  #[inline] pub fn cmd(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x3 // [7:6]
  }
#[doc="Command"]
  #[inline] pub fn set_cmd(mut self, value: u8) -> Self {
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
#[doc="Control C"]
#[derive(PartialEq, Eq)]
pub struct Ctrlc(pub u8);
impl Ctrlc {
#[doc="Output Waveform 0 Invert Enable"]
  #[inline] pub fn inven0(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Output Waveform 0 Invert Enable"]
  #[inline] pub fn set_inven0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Output Waveform 1 Invert Enable"]
  #[inline] pub fn inven1(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Output Waveform 1 Invert Enable"]
  #[inline] pub fn set_inven1(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Capture Channel 0 Enable"]
  #[inline] pub fn cpten0(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
#[doc="Capture Channel 0 Enable"]
  #[inline] pub fn set_cpten0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Capture Channel 1 Enable"]
  #[inline] pub fn cpten1(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
#[doc="Capture Channel 1 Enable"]
  #[inline] pub fn set_cpten1(mut self, value: u8) -> Self {
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
#[doc="Debug Control"]
#[derive(PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
#[doc="Debug Run Mode"]
  #[inline] pub fn dbgrun(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Debug Run Mode"]
  #[inline] pub fn set_dbgrun(mut self, value: u8) -> Self {
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
#[doc="Event Control"]
#[derive(PartialEq, Eq)]
pub struct Evctrl(pub u16);
impl Evctrl {
#[doc="Event Action"]
  #[inline] pub fn evact(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x7 // [2:0]
  }
#[doc="Event Action"]
  #[inline] pub fn set_evact(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="TC Inverted Event Input"]
  #[inline] pub fn tcinv(&self) -> u16 {
     ((self.0 as u16) >> 4) & 0x1 // [4]
  }
#[doc="TC Inverted Event Input"]
  #[inline] pub fn set_tcinv(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="TC Event Input"]
  #[inline] pub fn tcei(&self) -> u16 {
     ((self.0 as u16) >> 5) & 0x1 // [5]
  }
#[doc="TC Event Input"]
  #[inline] pub fn set_tcei(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Overflow/Underflow Event Output Enable"]
  #[inline] pub fn ovfeo(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x1 // [8]
  }
#[doc="Overflow/Underflow Event Output Enable"]
  #[inline] pub fn set_ovfeo(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Match or Capture Channel 0 Event Output Enable"]
  #[inline] pub fn mceo0(&self) -> u16 {
     ((self.0 as u16) >> 12) & 0x1 // [12]
  }
#[doc="Match or Capture Channel 0 Event Output Enable"]
  #[inline] pub fn set_mceo0(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Match or Capture Channel 1 Event Output Enable"]
  #[inline] pub fn mceo1(&self) -> u16 {
     ((self.0 as u16) >> 13) & 0x1 // [13]
  }
#[doc="Match or Capture Channel 1 Event Output Enable"]
  #[inline] pub fn set_mceo1(mut self, value: u16) -> Self {
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
#[doc="Interrupt Enable Clear"]
#[derive(PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
#[doc="Overflow Interrupt Enable"]
  #[inline] pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Overflow Interrupt Enable"]
  #[inline] pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Error Interrupt Enable"]
  #[inline] pub fn err(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Error Interrupt Enable"]
  #[inline] pub fn set_err(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Synchronization Ready Interrupt Enable"]
  #[inline] pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
#[doc="Synchronization Ready Interrupt Enable"]
  #[inline] pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Match or Capture Channel 0 Interrupt Enable"]
  #[inline] pub fn mc0(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
#[doc="Match or Capture Channel 0 Interrupt Enable"]
  #[inline] pub fn set_mc0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Match or Capture Channel 1 Interrupt Enable"]
  #[inline] pub fn mc1(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
#[doc="Match or Capture Channel 1 Interrupt Enable"]
  #[inline] pub fn set_mc1(mut self, value: u8) -> Self {
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
#[doc="Interrupt Enable Set"]
#[derive(PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
#[doc="Overflow Interrupt Enable"]
  #[inline] pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Overflow Interrupt Enable"]
  #[inline] pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Error Interrupt Enable"]
  #[inline] pub fn err(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Error Interrupt Enable"]
  #[inline] pub fn set_err(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Synchronization Ready Interrupt Enable"]
  #[inline] pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
#[doc="Synchronization Ready Interrupt Enable"]
  #[inline] pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Match or Capture Channel 0 Interrupt Enable"]
  #[inline] pub fn mc0(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
#[doc="Match or Capture Channel 0 Interrupt Enable"]
  #[inline] pub fn set_mc0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Match or Capture Channel 1 Interrupt Enable"]
  #[inline] pub fn mc1(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
#[doc="Match or Capture Channel 1 Interrupt Enable"]
  #[inline] pub fn set_mc1(mut self, value: u8) -> Self {
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
#[doc="Interrupt Flag Status and Clear"]
#[derive(PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
#[doc="Overflow"]
  #[inline] pub fn ovf(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Overflow"]
  #[inline] pub fn set_ovf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Error"]
  #[inline] pub fn err(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Error"]
  #[inline] pub fn set_err(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Synchronization Ready"]
  #[inline] pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
#[doc="Synchronization Ready"]
  #[inline] pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Match or Capture Channel 0"]
  #[inline] pub fn mc0(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
#[doc="Match or Capture Channel 0"]
  #[inline] pub fn set_mc0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Match or Capture Channel 1"]
  #[inline] pub fn mc1(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
#[doc="Match or Capture Channel 1"]
  #[inline] pub fn set_mc1(mut self, value: u8) -> Self {
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
#[doc="Read Request"]
#[derive(PartialEq, Eq)]
pub struct Readreq(pub u16);
impl Readreq {
#[doc="Address"]
  #[inline] pub fn addr(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1f // [4:0]
  }
#[doc="Address"]
  #[inline] pub fn set_addr(mut self, value: u16) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Read Continuously"]
  #[inline] pub fn rcont(&self) -> u16 {
     ((self.0 as u16) >> 14) & 0x1 // [14]
  }
#[doc="Read Continuously"]
  #[inline] pub fn set_rcont(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Read Request"]
  #[inline] pub fn rreq(&self) -> u16 {
     ((self.0 as u16) >> 15) & 0x1 // [15]
  }
#[doc="Read Request"]
  #[inline] pub fn set_rreq(mut self, value: u16) -> Self {
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
#[doc="Status"]
#[derive(PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
#[doc="Stop"]
  #[inline] pub fn stop(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
#[doc="Stop"]
  #[inline] pub fn set_stop(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Slave"]
  #[inline] pub fn slave(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
#[doc="Slave"]
  #[inline] pub fn set_slave(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Synchronization Busy"]
  #[inline] pub fn syncbusy(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
#[doc="Synchronization Busy"]
  #[inline] pub fn set_syncbusy(mut self, value: u8) -> Self {
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
#[derive(Clone, Copy, PartialEq)]
#[doc="TC Channel"]
pub struct Channel<P, T> { pub periph: Periph<T>, pub index: usize, pub id: P }

impl<P,T> Channel<P,T> {
   #[inline] pub fn periph(&self) -> &Periph<T> { &self.periph }
   #[inline] pub fn index(&self) -> usize { self.index }
}

pub const TC3_CH0: Channel<Tc3Ch0Id, Tc3Id> = Channel { periph: TC3, index: 0, id: Tc3Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tc3Ch0Id {}
pub type Tc3Ch0 = Channel<Tc3Ch0Id, Tc3Id>;

pub const TC3_CH1: Channel<Tc3Ch1Id, Tc3Id> = Channel { periph: TC3, index: 1, id: Tc3Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tc3Ch1Id {}
pub type Tc3Ch1 = Channel<Tc3Ch1Id, Tc3Id>;

pub const TC4_CH0: Channel<Tc4Ch0Id, Tc4Id> = Channel { periph: TC4, index: 0, id: Tc4Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tc4Ch0Id {}
pub type Tc4Ch0 = Channel<Tc4Ch0Id, Tc4Id>;

pub const TC4_CH1: Channel<Tc4Ch1Id, Tc4Id> = Channel { periph: TC4, index: 1, id: Tc4Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tc4Ch1Id {}
pub type Tc4Ch1 = Channel<Tc4Ch1Id, Tc4Id>;

pub const TC5_CH0: Channel<Tc5Ch0Id, Tc5Id> = Channel { periph: TC5, index: 0, id: Tc5Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tc5Ch0Id {}
pub type Tc5Ch0 = Channel<Tc5Ch0Id, Tc5Id>;

pub const TC5_CH1: Channel<Tc5Ch1Id, Tc5Id> = Channel { periph: TC5, index: 1, id: Tc5Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tc5Ch1Id {}
pub type Tc5Ch1 = Channel<Tc5Ch1Id, Tc5Id>;

