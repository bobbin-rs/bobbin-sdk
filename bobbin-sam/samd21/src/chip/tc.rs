//! Timer/Counter
#[allow(unused_imports)] use bobbin_common::*;

periph!(TcPeriph, TC3, Tc3, 0x42002c00);
periph!(TcPeriph, TC4, Tc4, 0x42003000);
periph!(TcPeriph, TC5, Tc5, 0x42003400);

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


pub trait TcPeriph : Base {}

pub mod count8 {
   #[allow(unused_imports)] use bobbin_common::*;

pub trait Count8Periph : Base {
#[doc="Get the *const pointer for the CC register."]
   #[inline] fn cc_ptr<I: Into<bits::R2>>(&self, index: I) -> *const u8 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x18 + (index))
   }
#[doc="Get the *mut pointer for the CC register."]
   #[inline] fn cc_mut<I: Into<bits::R2>>(&self, index: I) -> *mut u8 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x18 + (index))
   }
#[doc="Read the CC register."]
   #[inline] fn cc<I: Into<bits::R2>>(&self, index: I) -> Cc { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Cc(::core::ptr::read_volatile((self.base() + 0x18 + (index)) as *const u8))
      }
   }
#[doc="Write the CC register."]
   #[inline] fn set_cc<I: Into<bits::R2>, F: FnOnce(Cc) -> Cc>(&self, index: I, f: F) -> &Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Cc(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x18 + (index)) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the CC register."]
   #[inline] fn with_cc<I: Into<bits::R2> + Copy, F: FnOnce(Cc) -> Cc>(&self, index: I, f: F) -> &Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      let tmp = unsafe {
         Cc(::core::ptr::read_volatile((self.base() + 0x18 + (index)) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x18 + (index)) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the COUNT register."]
   #[inline] fn count_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0x10)
   }
#[doc="Get the *mut pointer for the COUNT register."]
   #[inline] fn count_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0x10)
   }
#[doc="Read the COUNT register."]
   #[inline] fn count(&self) -> Count { 
      unsafe {
         Count(::core::ptr::read_volatile((self.base() + 0x10) as *const u8))
      }
   }
#[doc="Write the COUNT register."]
   #[inline] fn set_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
      let value = f(Count(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x10) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the COUNT register."]
   #[inline] fn with_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Count(::core::ptr::read_volatile((self.base() + 0x10) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x10) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PER register."]
   #[inline] fn per_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0x14)
   }
#[doc="Get the *mut pointer for the PER register."]
   #[inline] fn per_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0x14)
   }
#[doc="Read the PER register."]
   #[inline] fn per(&self) -> Per { 
      unsafe {
         Per(::core::ptr::read_volatile((self.base() + 0x14) as *const u8))
      }
   }
#[doc="Write the PER register."]
   #[inline] fn set_per<F: FnOnce(Per) -> Per>(&self, f: F) -> &Self {
      let value = f(Per(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x14) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the PER register."]
   #[inline] fn with_per<F: FnOnce(Per) -> Per>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Per(::core::ptr::read_volatile((self.base() + 0x14) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x14) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRLA register."]
   #[inline] fn ctrla_ptr(&self) -> *const u16 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Get the *mut pointer for the CTRLA register."]
   #[inline] fn ctrla_mut(&self) -> *mut u16 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Read the CTRLA register."]
   #[inline] fn ctrla(&self) -> Ctrla { 
      unsafe {
         Ctrla(::core::ptr::read_volatile((self.base() + 0x0) as *const u16))
      }
   }
#[doc="Write the CTRLA register."]
   #[inline] fn set_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
      let value = f(Ctrla(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x0) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the CTRLA register."]
   #[inline] fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ctrla(::core::ptr::read_volatile((self.base() + 0x0) as *const u16))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x0) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRLBCLR register."]
   #[inline] fn ctrlbclr_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Get the *mut pointer for the CTRLBCLR register."]
   #[inline] fn ctrlbclr_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Read the CTRLBCLR register."]
   #[inline] fn ctrlbclr(&self) -> Ctrlbclr { 
      unsafe {
         Ctrlbclr(::core::ptr::read_volatile((self.base() + 0x4) as *const u8))
      }
   }
#[doc="Write the CTRLBCLR register."]
   #[inline] fn set_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Self {
      let value = f(Ctrlbclr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the CTRLBCLR register."]
   #[inline] fn with_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ctrlbclr(::core::ptr::read_volatile((self.base() + 0x4) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRLBSET register."]
   #[inline] fn ctrlbset_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0x5)
   }
#[doc="Get the *mut pointer for the CTRLBSET register."]
   #[inline] fn ctrlbset_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0x5)
   }
#[doc="Read the CTRLBSET register."]
   #[inline] fn ctrlbset(&self) -> Ctrlbset { 
      unsafe {
         Ctrlbset(::core::ptr::read_volatile((self.base() + 0x5) as *const u8))
      }
   }
#[doc="Write the CTRLBSET register."]
   #[inline] fn set_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Self {
      let value = f(Ctrlbset(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x5) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the CTRLBSET register."]
   #[inline] fn with_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ctrlbset(::core::ptr::read_volatile((self.base() + 0x5) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x5) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRLC register."]
   #[inline] fn ctrlc_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0x6)
   }
#[doc="Get the *mut pointer for the CTRLC register."]
   #[inline] fn ctrlc_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0x6)
   }
#[doc="Read the CTRLC register."]
   #[inline] fn ctrlc(&self) -> Ctrlc { 
      unsafe {
         Ctrlc(::core::ptr::read_volatile((self.base() + 0x6) as *const u8))
      }
   }
#[doc="Write the CTRLC register."]
   #[inline] fn set_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
      let value = f(Ctrlc(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x6) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the CTRLC register."]
   #[inline] fn with_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ctrlc(::core::ptr::read_volatile((self.base() + 0x6) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x6) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DBGCTRL register."]
   #[inline] fn dbgctrl_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Get the *mut pointer for the DBGCTRL register."]
   #[inline] fn dbgctrl_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Read the DBGCTRL register."]
   #[inline] fn dbgctrl(&self) -> Dbgctrl { 
      unsafe {
         Dbgctrl(::core::ptr::read_volatile((self.base() + 0x8) as *const u8))
      }
   }
#[doc="Write the DBGCTRL register."]
   #[inline] fn set_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
      let value = f(Dbgctrl(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x8) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the DBGCTRL register."]
   #[inline] fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Dbgctrl(::core::ptr::read_volatile((self.base() + 0x8) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x8) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the EVCTRL register."]
   #[inline] fn evctrl_ptr(&self) -> *const u16 { 
       <Self as Base>::addr(&self, 0xa)
   }
#[doc="Get the *mut pointer for the EVCTRL register."]
   #[inline] fn evctrl_mut(&self) -> *mut u16 { 
       <Self as Base>::addr(&self, 0xa)
   }
#[doc="Read the EVCTRL register."]
   #[inline] fn evctrl(&self) -> Evctrl { 
      unsafe {
         Evctrl(::core::ptr::read_volatile((self.base() + 0xa) as *const u16))
      }
   }
#[doc="Write the EVCTRL register."]
   #[inline] fn set_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
      let value = f(Evctrl(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the EVCTRL register."]
   #[inline] fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Evctrl(::core::ptr::read_volatile((self.base() + 0xa) as *const u16))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTENCLR register."]
   #[inline] fn intenclr_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0xc)
   }
#[doc="Get the *mut pointer for the INTENCLR register."]
   #[inline] fn intenclr_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0xc)
   }
#[doc="Read the INTENCLR register."]
   #[inline] fn intenclr(&self) -> Intenclr { 
      unsafe {
         Intenclr(::core::ptr::read_volatile((self.base() + 0xc) as *const u8))
      }
   }
#[doc="Write the INTENCLR register."]
   #[inline] fn set_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let value = f(Intenclr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTENCLR register."]
   #[inline] fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Intenclr(::core::ptr::read_volatile((self.base() + 0xc) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTENSET register."]
   #[inline] fn intenset_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0xd)
   }
#[doc="Get the *mut pointer for the INTENSET register."]
   #[inline] fn intenset_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0xd)
   }
#[doc="Read the INTENSET register."]
   #[inline] fn intenset(&self) -> Intenset { 
      unsafe {
         Intenset(::core::ptr::read_volatile((self.base() + 0xd) as *const u8))
      }
   }
#[doc="Write the INTENSET register."]
   #[inline] fn set_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let value = f(Intenset(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTENSET register."]
   #[inline] fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Intenset(::core::ptr::read_volatile((self.base() + 0xd) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTFLAG register."]
   #[inline] fn intflag_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0xe)
   }
#[doc="Get the *mut pointer for the INTFLAG register."]
   #[inline] fn intflag_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0xe)
   }
#[doc="Read the INTFLAG register."]
   #[inline] fn intflag(&self) -> Intflag { 
      unsafe {
         Intflag(::core::ptr::read_volatile((self.base() + 0xe) as *const u8))
      }
   }
#[doc="Write the INTFLAG register."]
   #[inline] fn set_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let value = f(Intflag(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xe) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTFLAG register."]
   #[inline] fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Intflag(::core::ptr::read_volatile((self.base() + 0xe) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xe) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the READREQ register."]
   #[inline] fn readreq_ptr(&self) -> *const u16 { 
       <Self as Base>::addr(&self, 0x2)
   }
#[doc="Get the *mut pointer for the READREQ register."]
   #[inline] fn readreq_mut(&self) -> *mut u16 { 
       <Self as Base>::addr(&self, 0x2)
   }
#[doc="Read the READREQ register."]
   #[inline] fn readreq(&self) -> Readreq { 
      unsafe {
         Readreq(::core::ptr::read_volatile((self.base() + 0x2) as *const u16))
      }
   }
#[doc="Write the READREQ register."]
   #[inline] fn set_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
      let value = f(Readreq(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x2) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the READREQ register."]
   #[inline] fn with_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Readreq(::core::ptr::read_volatile((self.base() + 0x2) as *const u16))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x2) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the STATUS register."]
   #[inline] fn status_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0xf)
   }
#[doc="Get the *mut pointer for the STATUS register."]
   #[inline] fn status_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0xf)
   }
#[doc="Read the STATUS register."]
   #[inline] fn status(&self) -> Status { 
      unsafe {
         Status(::core::ptr::read_volatile((self.base() + 0xf) as *const u8))
      }
   }

}

#[doc="COUNT8 Compare/Capture"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cc(pub u8);
impl Cc {
#[doc="Compare/Capture Value"]
   #[inline] pub fn cc(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Compare/Capture Value"]
   #[inline] pub fn set_cc<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Count(pub u8);
impl Count {
#[doc="Counter Value"]
   #[inline] pub fn count(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Counter Value"]
   #[inline] pub fn set_count<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Per(pub u8);
impl Per {
#[doc="Period Value"]
   #[inline] pub fn per(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Period Value"]
   #[inline] pub fn set_per<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u16);
impl Ctrla {
#[doc="Software Reset"]
   #[inline] pub fn swrst(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Software Reset"]
   #[inline] pub fn set_swrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Enable"]
   #[inline] pub fn enable(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Enable"]
   #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="TC Mode"]
   #[inline] pub fn mode(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
   }
#[doc="TC Mode"]
   #[inline] pub fn set_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x3 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Waveform Generation Operation"]
   #[inline] pub fn wavegen(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
   }
#[doc="Waveform Generation Operation"]
   #[inline] pub fn set_wavegen<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x3 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Prescaler"]
   #[inline] pub fn prescaler(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
   }
#[doc="Prescaler"]
   #[inline] pub fn set_prescaler<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x7 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Run in Standby"]
   #[inline] pub fn runstdby(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="Run in Standby"]
   #[inline] pub fn set_runstdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="Prescaler and Counter Synchronization"]
   #[inline] pub fn prescsync(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
   }
#[doc="Prescaler and Counter Synchronization"]
   #[inline] pub fn set_prescsync<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u16 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrlbclr(pub u8);
impl Ctrlbclr {
#[doc="Counter Direction"]
   #[inline] pub fn dir(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Counter Direction"]
   #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="One-Shot"]
   #[inline] pub fn oneshot(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="One-Shot"]
   #[inline] pub fn set_oneshot<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Command"]
   #[inline] pub fn cmd(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
   }
#[doc="Command"]
   #[inline] pub fn set_cmd<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u8 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrlbset(pub u8);
impl Ctrlbset {
#[doc="Counter Direction"]
   #[inline] pub fn dir(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Counter Direction"]
   #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="One-Shot"]
   #[inline] pub fn oneshot(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="One-Shot"]
   #[inline] pub fn set_oneshot<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Command"]
   #[inline] pub fn cmd(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
   }
#[doc="Command"]
   #[inline] pub fn set_cmd<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u8 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrlc(pub u8);
impl Ctrlc {
#[doc="Output Waveform 0 Invert Enable"]
   #[inline] pub fn inven0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Output Waveform 0 Invert Enable"]
   #[inline] pub fn set_inven0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Output Waveform 1 Invert Enable"]
   #[inline] pub fn inven1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Output Waveform 1 Invert Enable"]
   #[inline] pub fn set_inven1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Capture Channel 0 Enable"]
   #[inline] pub fn cpten0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Capture Channel 0 Enable"]
   #[inline] pub fn set_cpten0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Capture Channel 1 Enable"]
   #[inline] pub fn cpten1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Capture Channel 1 Enable"]
   #[inline] pub fn set_cpten1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
#[doc="Debug Run Mode"]
   #[inline] pub fn dbgrun(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Debug Run Mode"]
   #[inline] pub fn set_dbgrun<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u16);
impl Evctrl {
#[doc="Event Action"]
   #[inline] pub fn evact(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
   }
#[doc="Event Action"]
   #[inline] pub fn set_evact<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x7 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="TC Inverted Event Input"]
   #[inline] pub fn tcinv(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="TC Inverted Event Input"]
   #[inline] pub fn set_tcinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="TC Event Input"]
   #[inline] pub fn tcei(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="TC Event Input"]
   #[inline] pub fn set_tcei<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Overflow/Underflow Event Output Enable"]
   #[inline] pub fn ovfeo(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Overflow/Underflow Event Output Enable"]
   #[inline] pub fn set_ovfeo<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Match or Capture Channel 0 Event Output Enable"]
   #[inline] pub fn mceo0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="Match or Capture Channel 0 Event Output Enable"]
   #[inline] pub fn set_mceo0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Match or Capture Channel 1 Event Output Enable"]
   #[inline] pub fn mceo1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="Match or Capture Channel 1 Event Output Enable"]
   #[inline] pub fn set_mceo1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Error Interrupt Enable"]
   #[inline] pub fn err(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Error Interrupt Enable"]
   #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Match or Capture Channel 0 Interrupt Enable"]
   #[inline] pub fn mc0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Match or Capture Channel 0 Interrupt Enable"]
   #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Match or Capture Channel 1 Interrupt Enable"]
   #[inline] pub fn mc1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Match or Capture Channel 1 Interrupt Enable"]
   #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Error Interrupt Enable"]
   #[inline] pub fn err(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Error Interrupt Enable"]
   #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Match or Capture Channel 0 Interrupt Enable"]
   #[inline] pub fn mc0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Match or Capture Channel 0 Interrupt Enable"]
   #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Match or Capture Channel 1 Interrupt Enable"]
   #[inline] pub fn mc1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Match or Capture Channel 1 Interrupt Enable"]
   #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
#[doc="Overflow"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Overflow"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Error"]
   #[inline] pub fn err(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Error"]
   #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Synchronization Ready"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Synchronization Ready"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Match or Capture Channel 0"]
   #[inline] pub fn mc0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Match or Capture Channel 0"]
   #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Match or Capture Channel 1"]
   #[inline] pub fn mc1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Match or Capture Channel 1"]
   #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Readreq(pub u16);
impl Readreq {
#[doc="Address"]
   #[inline] pub fn addr(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
   }
#[doc="Address"]
   #[inline] pub fn set_addr<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1f << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Read Continuously"]
   #[inline] pub fn rcont(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="Read Continuously"]
   #[inline] pub fn set_rcont<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="Read Request"]
   #[inline] pub fn rreq(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Read Request"]
   #[inline] pub fn set_rreq<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
#[doc="Stop"]
   #[inline] pub fn stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Stop"]
   #[inline] pub fn set_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Slave"]
   #[inline] pub fn slave(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Slave"]
   #[inline] pub fn set_slave<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Synchronization Busy"]
   #[inline] pub fn syncbusy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Synchronization Busy"]
   #[inline] pub fn set_syncbusy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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

pub mod count16 {
   #[allow(unused_imports)] use bobbin_common::*;

pub trait Count16Periph : Base {
#[doc="Get the *const pointer for the CC register."]
   #[inline] fn cc_ptr<I: Into<bits::R2>>(&self, index: I) -> *const u16 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x18 + (index << 1))
   }
#[doc="Get the *mut pointer for the CC register."]
   #[inline] fn cc_mut<I: Into<bits::R2>>(&self, index: I) -> *mut u16 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x18 + (index << 1))
   }
#[doc="Read the CC register."]
   #[inline] fn cc<I: Into<bits::R2>>(&self, index: I) -> Cc { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Cc(::core::ptr::read_volatile((self.base() + 0x18 + (index << 1)) as *const u16))
      }
   }
#[doc="Write the CC register."]
   #[inline] fn set_cc<I: Into<bits::R2>, F: FnOnce(Cc) -> Cc>(&self, index: I, f: F) -> &Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Cc(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x18 + (index << 1)) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the CC register."]
   #[inline] fn with_cc<I: Into<bits::R2> + Copy, F: FnOnce(Cc) -> Cc>(&self, index: I, f: F) -> &Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      let tmp = unsafe {
         Cc(::core::ptr::read_volatile((self.base() + 0x18 + (index << 1)) as *const u16))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x18 + (index << 1)) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the COUNT register."]
   #[inline] fn count_ptr(&self) -> *const u16 { 
       <Self as Base>::addr(&self, 0x10)
   }
#[doc="Get the *mut pointer for the COUNT register."]
   #[inline] fn count_mut(&self) -> *mut u16 { 
       <Self as Base>::addr(&self, 0x10)
   }
#[doc="Read the COUNT register."]
   #[inline] fn count(&self) -> Count { 
      unsafe {
         Count(::core::ptr::read_volatile((self.base() + 0x10) as *const u16))
      }
   }
#[doc="Write the COUNT register."]
   #[inline] fn set_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
      let value = f(Count(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x10) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the COUNT register."]
   #[inline] fn with_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Count(::core::ptr::read_volatile((self.base() + 0x10) as *const u16))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x10) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRLA register."]
   #[inline] fn ctrla_ptr(&self) -> *const u16 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Get the *mut pointer for the CTRLA register."]
   #[inline] fn ctrla_mut(&self) -> *mut u16 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Read the CTRLA register."]
   #[inline] fn ctrla(&self) -> Ctrla { 
      unsafe {
         Ctrla(::core::ptr::read_volatile((self.base() + 0x0) as *const u16))
      }
   }
#[doc="Write the CTRLA register."]
   #[inline] fn set_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
      let value = f(Ctrla(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x0) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the CTRLA register."]
   #[inline] fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ctrla(::core::ptr::read_volatile((self.base() + 0x0) as *const u16))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x0) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRLBCLR register."]
   #[inline] fn ctrlbclr_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Get the *mut pointer for the CTRLBCLR register."]
   #[inline] fn ctrlbclr_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Read the CTRLBCLR register."]
   #[inline] fn ctrlbclr(&self) -> Ctrlbclr { 
      unsafe {
         Ctrlbclr(::core::ptr::read_volatile((self.base() + 0x4) as *const u8))
      }
   }
#[doc="Write the CTRLBCLR register."]
   #[inline] fn set_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Self {
      let value = f(Ctrlbclr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the CTRLBCLR register."]
   #[inline] fn with_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ctrlbclr(::core::ptr::read_volatile((self.base() + 0x4) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRLBSET register."]
   #[inline] fn ctrlbset_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0x5)
   }
#[doc="Get the *mut pointer for the CTRLBSET register."]
   #[inline] fn ctrlbset_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0x5)
   }
#[doc="Read the CTRLBSET register."]
   #[inline] fn ctrlbset(&self) -> Ctrlbset { 
      unsafe {
         Ctrlbset(::core::ptr::read_volatile((self.base() + 0x5) as *const u8))
      }
   }
#[doc="Write the CTRLBSET register."]
   #[inline] fn set_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Self {
      let value = f(Ctrlbset(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x5) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the CTRLBSET register."]
   #[inline] fn with_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ctrlbset(::core::ptr::read_volatile((self.base() + 0x5) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x5) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRLC register."]
   #[inline] fn ctrlc_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0x6)
   }
#[doc="Get the *mut pointer for the CTRLC register."]
   #[inline] fn ctrlc_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0x6)
   }
#[doc="Read the CTRLC register."]
   #[inline] fn ctrlc(&self) -> Ctrlc { 
      unsafe {
         Ctrlc(::core::ptr::read_volatile((self.base() + 0x6) as *const u8))
      }
   }
#[doc="Write the CTRLC register."]
   #[inline] fn set_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
      let value = f(Ctrlc(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x6) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the CTRLC register."]
   #[inline] fn with_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ctrlc(::core::ptr::read_volatile((self.base() + 0x6) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x6) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DBGCTRL register."]
   #[inline] fn dbgctrl_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Get the *mut pointer for the DBGCTRL register."]
   #[inline] fn dbgctrl_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Read the DBGCTRL register."]
   #[inline] fn dbgctrl(&self) -> Dbgctrl { 
      unsafe {
         Dbgctrl(::core::ptr::read_volatile((self.base() + 0x8) as *const u8))
      }
   }
#[doc="Write the DBGCTRL register."]
   #[inline] fn set_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
      let value = f(Dbgctrl(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x8) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the DBGCTRL register."]
   #[inline] fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Dbgctrl(::core::ptr::read_volatile((self.base() + 0x8) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x8) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the EVCTRL register."]
   #[inline] fn evctrl_ptr(&self) -> *const u16 { 
       <Self as Base>::addr(&self, 0xa)
   }
#[doc="Get the *mut pointer for the EVCTRL register."]
   #[inline] fn evctrl_mut(&self) -> *mut u16 { 
       <Self as Base>::addr(&self, 0xa)
   }
#[doc="Read the EVCTRL register."]
   #[inline] fn evctrl(&self) -> Evctrl { 
      unsafe {
         Evctrl(::core::ptr::read_volatile((self.base() + 0xa) as *const u16))
      }
   }
#[doc="Write the EVCTRL register."]
   #[inline] fn set_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
      let value = f(Evctrl(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the EVCTRL register."]
   #[inline] fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Evctrl(::core::ptr::read_volatile((self.base() + 0xa) as *const u16))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTENCLR register."]
   #[inline] fn intenclr_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0xc)
   }
#[doc="Get the *mut pointer for the INTENCLR register."]
   #[inline] fn intenclr_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0xc)
   }
#[doc="Read the INTENCLR register."]
   #[inline] fn intenclr(&self) -> Intenclr { 
      unsafe {
         Intenclr(::core::ptr::read_volatile((self.base() + 0xc) as *const u8))
      }
   }
#[doc="Write the INTENCLR register."]
   #[inline] fn set_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let value = f(Intenclr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTENCLR register."]
   #[inline] fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Intenclr(::core::ptr::read_volatile((self.base() + 0xc) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTENSET register."]
   #[inline] fn intenset_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0xd)
   }
#[doc="Get the *mut pointer for the INTENSET register."]
   #[inline] fn intenset_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0xd)
   }
#[doc="Read the INTENSET register."]
   #[inline] fn intenset(&self) -> Intenset { 
      unsafe {
         Intenset(::core::ptr::read_volatile((self.base() + 0xd) as *const u8))
      }
   }
#[doc="Write the INTENSET register."]
   #[inline] fn set_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let value = f(Intenset(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTENSET register."]
   #[inline] fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Intenset(::core::ptr::read_volatile((self.base() + 0xd) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTFLAG register."]
   #[inline] fn intflag_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0xe)
   }
#[doc="Get the *mut pointer for the INTFLAG register."]
   #[inline] fn intflag_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0xe)
   }
#[doc="Read the INTFLAG register."]
   #[inline] fn intflag(&self) -> Intflag { 
      unsafe {
         Intflag(::core::ptr::read_volatile((self.base() + 0xe) as *const u8))
      }
   }
#[doc="Write the INTFLAG register."]
   #[inline] fn set_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let value = f(Intflag(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xe) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTFLAG register."]
   #[inline] fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Intflag(::core::ptr::read_volatile((self.base() + 0xe) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xe) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the READREQ register."]
   #[inline] fn readreq_ptr(&self) -> *const u16 { 
       <Self as Base>::addr(&self, 0x2)
   }
#[doc="Get the *mut pointer for the READREQ register."]
   #[inline] fn readreq_mut(&self) -> *mut u16 { 
       <Self as Base>::addr(&self, 0x2)
   }
#[doc="Read the READREQ register."]
   #[inline] fn readreq(&self) -> Readreq { 
      unsafe {
         Readreq(::core::ptr::read_volatile((self.base() + 0x2) as *const u16))
      }
   }
#[doc="Write the READREQ register."]
   #[inline] fn set_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
      let value = f(Readreq(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x2) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the READREQ register."]
   #[inline] fn with_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Readreq(::core::ptr::read_volatile((self.base() + 0x2) as *const u16))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x2) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the STATUS register."]
   #[inline] fn status_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0xf)
   }
#[doc="Get the *mut pointer for the STATUS register."]
   #[inline] fn status_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0xf)
   }
#[doc="Read the STATUS register."]
   #[inline] fn status(&self) -> Status { 
      unsafe {
         Status(::core::ptr::read_volatile((self.base() + 0xf) as *const u8))
      }
   }

}

#[doc="COUNT16 Compare/Capture"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cc(pub u16);
impl Cc {
#[doc="Compare/Capture Value"]
   #[inline] pub fn cc(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Compare/Capture Value"]
   #[inline] pub fn set_cc<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u16 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Count(pub u16);
impl Count {
#[doc="Count Value"]
   #[inline] pub fn count(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Count Value"]
   #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u16 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u16);
impl Ctrla {
#[doc="Software Reset"]
   #[inline] pub fn swrst(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Software Reset"]
   #[inline] pub fn set_swrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Enable"]
   #[inline] pub fn enable(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Enable"]
   #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="TC Mode"]
   #[inline] pub fn mode(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
   }
#[doc="TC Mode"]
   #[inline] pub fn set_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x3 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Waveform Generation Operation"]
   #[inline] pub fn wavegen(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
   }
#[doc="Waveform Generation Operation"]
   #[inline] pub fn set_wavegen<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x3 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Prescaler"]
   #[inline] pub fn prescaler(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
   }
#[doc="Prescaler"]
   #[inline] pub fn set_prescaler<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x7 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Run in Standby"]
   #[inline] pub fn runstdby(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="Run in Standby"]
   #[inline] pub fn set_runstdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="Prescaler and Counter Synchronization"]
   #[inline] pub fn prescsync(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
   }
#[doc="Prescaler and Counter Synchronization"]
   #[inline] pub fn set_prescsync<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u16 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrlbclr(pub u8);
impl Ctrlbclr {
#[doc="Counter Direction"]
   #[inline] pub fn dir(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Counter Direction"]
   #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="One-Shot"]
   #[inline] pub fn oneshot(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="One-Shot"]
   #[inline] pub fn set_oneshot<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Command"]
   #[inline] pub fn cmd(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
   }
#[doc="Command"]
   #[inline] pub fn set_cmd<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u8 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrlbset(pub u8);
impl Ctrlbset {
#[doc="Counter Direction"]
   #[inline] pub fn dir(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Counter Direction"]
   #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="One-Shot"]
   #[inline] pub fn oneshot(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="One-Shot"]
   #[inline] pub fn set_oneshot<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Command"]
   #[inline] pub fn cmd(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
   }
#[doc="Command"]
   #[inline] pub fn set_cmd<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u8 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrlc(pub u8);
impl Ctrlc {
#[doc="Output Waveform 0 Invert Enable"]
   #[inline] pub fn inven0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Output Waveform 0 Invert Enable"]
   #[inline] pub fn set_inven0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Output Waveform 1 Invert Enable"]
   #[inline] pub fn inven1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Output Waveform 1 Invert Enable"]
   #[inline] pub fn set_inven1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Capture Channel 0 Enable"]
   #[inline] pub fn cpten0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Capture Channel 0 Enable"]
   #[inline] pub fn set_cpten0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Capture Channel 1 Enable"]
   #[inline] pub fn cpten1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Capture Channel 1 Enable"]
   #[inline] pub fn set_cpten1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
#[doc="Debug Run Mode"]
   #[inline] pub fn dbgrun(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Debug Run Mode"]
   #[inline] pub fn set_dbgrun<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u16);
impl Evctrl {
#[doc="Event Action"]
   #[inline] pub fn evact(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
   }
#[doc="Event Action"]
   #[inline] pub fn set_evact<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x7 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="TC Inverted Event Input"]
   #[inline] pub fn tcinv(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="TC Inverted Event Input"]
   #[inline] pub fn set_tcinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="TC Event Input"]
   #[inline] pub fn tcei(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="TC Event Input"]
   #[inline] pub fn set_tcei<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Overflow/Underflow Event Output Enable"]
   #[inline] pub fn ovfeo(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Overflow/Underflow Event Output Enable"]
   #[inline] pub fn set_ovfeo<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Match or Capture Channel 0 Event Output Enable"]
   #[inline] pub fn mceo0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="Match or Capture Channel 0 Event Output Enable"]
   #[inline] pub fn set_mceo0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Match or Capture Channel 1 Event Output Enable"]
   #[inline] pub fn mceo1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="Match or Capture Channel 1 Event Output Enable"]
   #[inline] pub fn set_mceo1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Error Interrupt Enable"]
   #[inline] pub fn err(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Error Interrupt Enable"]
   #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Match or Capture Channel 0 Interrupt Enable"]
   #[inline] pub fn mc0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Match or Capture Channel 0 Interrupt Enable"]
   #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Match or Capture Channel 1 Interrupt Enable"]
   #[inline] pub fn mc1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Match or Capture Channel 1 Interrupt Enable"]
   #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Error Interrupt Enable"]
   #[inline] pub fn err(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Error Interrupt Enable"]
   #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Match or Capture Channel 0 Interrupt Enable"]
   #[inline] pub fn mc0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Match or Capture Channel 0 Interrupt Enable"]
   #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Match or Capture Channel 1 Interrupt Enable"]
   #[inline] pub fn mc1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Match or Capture Channel 1 Interrupt Enable"]
   #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
#[doc="Overflow"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Overflow"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Error"]
   #[inline] pub fn err(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Error"]
   #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Synchronization Ready"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Synchronization Ready"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Match or Capture Channel 0"]
   #[inline] pub fn mc0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Match or Capture Channel 0"]
   #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Match or Capture Channel 1"]
   #[inline] pub fn mc1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Match or Capture Channel 1"]
   #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Readreq(pub u16);
impl Readreq {
#[doc="Address"]
   #[inline] pub fn addr(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
   }
#[doc="Address"]
   #[inline] pub fn set_addr<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1f << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Read Continuously"]
   #[inline] pub fn rcont(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="Read Continuously"]
   #[inline] pub fn set_rcont<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="Read Request"]
   #[inline] pub fn rreq(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Read Request"]
   #[inline] pub fn set_rreq<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
#[doc="Stop"]
   #[inline] pub fn stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Stop"]
   #[inline] pub fn set_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Slave"]
   #[inline] pub fn slave(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Slave"]
   #[inline] pub fn set_slave<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Synchronization Busy"]
   #[inline] pub fn syncbusy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Synchronization Busy"]
   #[inline] pub fn set_syncbusy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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

pub mod count32 {
   #[allow(unused_imports)] use bobbin_common::*;

pub trait Count32Periph : Base {
#[doc="Get the *const pointer for the CC register."]
   #[inline] fn cc_ptr<I: Into<bits::R2>>(&self, index: I) -> *const u32 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x18 + (index << 2))
   }
#[doc="Get the *mut pointer for the CC register."]
   #[inline] fn cc_mut<I: Into<bits::R2>>(&self, index: I) -> *mut u32 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x18 + (index << 2))
   }
#[doc="Read the CC register."]
   #[inline] fn cc<I: Into<bits::R2>>(&self, index: I) -> Cc { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Cc(::core::ptr::read_volatile((self.base() + 0x18 + (index << 2)) as *const u32))
      }
   }
#[doc="Write the CC register."]
   #[inline] fn set_cc<I: Into<bits::R2>, F: FnOnce(Cc) -> Cc>(&self, index: I, f: F) -> &Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Cc(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x18 + (index << 2)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CC register."]
   #[inline] fn with_cc<I: Into<bits::R2> + Copy, F: FnOnce(Cc) -> Cc>(&self, index: I, f: F) -> &Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      let tmp = unsafe {
         Cc(::core::ptr::read_volatile((self.base() + 0x18 + (index << 2)) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x18 + (index << 2)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the COUNT register."]
   #[inline] fn count_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x10)
   }
#[doc="Get the *mut pointer for the COUNT register."]
   #[inline] fn count_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x10)
   }
#[doc="Read the COUNT register."]
   #[inline] fn count(&self) -> Count { 
      unsafe {
         Count(::core::ptr::read_volatile((self.base() + 0x10) as *const u32))
      }
   }
#[doc="Write the COUNT register."]
   #[inline] fn set_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
      let value = f(Count(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x10) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the COUNT register."]
   #[inline] fn with_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Count(::core::ptr::read_volatile((self.base() + 0x10) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x10) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRLA register."]
   #[inline] fn ctrla_ptr(&self) -> *const u16 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Get the *mut pointer for the CTRLA register."]
   #[inline] fn ctrla_mut(&self) -> *mut u16 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Read the CTRLA register."]
   #[inline] fn ctrla(&self) -> Ctrla { 
      unsafe {
         Ctrla(::core::ptr::read_volatile((self.base() + 0x0) as *const u16))
      }
   }
#[doc="Write the CTRLA register."]
   #[inline] fn set_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
      let value = f(Ctrla(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x0) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the CTRLA register."]
   #[inline] fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ctrla(::core::ptr::read_volatile((self.base() + 0x0) as *const u16))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x0) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRLBCLR register."]
   #[inline] fn ctrlbclr_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Get the *mut pointer for the CTRLBCLR register."]
   #[inline] fn ctrlbclr_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Read the CTRLBCLR register."]
   #[inline] fn ctrlbclr(&self) -> Ctrlbclr { 
      unsafe {
         Ctrlbclr(::core::ptr::read_volatile((self.base() + 0x4) as *const u8))
      }
   }
#[doc="Write the CTRLBCLR register."]
   #[inline] fn set_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Self {
      let value = f(Ctrlbclr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the CTRLBCLR register."]
   #[inline] fn with_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ctrlbclr(::core::ptr::read_volatile((self.base() + 0x4) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRLBSET register."]
   #[inline] fn ctrlbset_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0x5)
   }
#[doc="Get the *mut pointer for the CTRLBSET register."]
   #[inline] fn ctrlbset_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0x5)
   }
#[doc="Read the CTRLBSET register."]
   #[inline] fn ctrlbset(&self) -> Ctrlbset { 
      unsafe {
         Ctrlbset(::core::ptr::read_volatile((self.base() + 0x5) as *const u8))
      }
   }
#[doc="Write the CTRLBSET register."]
   #[inline] fn set_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Self {
      let value = f(Ctrlbset(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x5) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the CTRLBSET register."]
   #[inline] fn with_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ctrlbset(::core::ptr::read_volatile((self.base() + 0x5) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x5) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRLC register."]
   #[inline] fn ctrlc_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0x6)
   }
#[doc="Get the *mut pointer for the CTRLC register."]
   #[inline] fn ctrlc_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0x6)
   }
#[doc="Read the CTRLC register."]
   #[inline] fn ctrlc(&self) -> Ctrlc { 
      unsafe {
         Ctrlc(::core::ptr::read_volatile((self.base() + 0x6) as *const u8))
      }
   }
#[doc="Write the CTRLC register."]
   #[inline] fn set_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
      let value = f(Ctrlc(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x6) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the CTRLC register."]
   #[inline] fn with_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ctrlc(::core::ptr::read_volatile((self.base() + 0x6) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x6) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DBGCTRL register."]
   #[inline] fn dbgctrl_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Get the *mut pointer for the DBGCTRL register."]
   #[inline] fn dbgctrl_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Read the DBGCTRL register."]
   #[inline] fn dbgctrl(&self) -> Dbgctrl { 
      unsafe {
         Dbgctrl(::core::ptr::read_volatile((self.base() + 0x8) as *const u8))
      }
   }
#[doc="Write the DBGCTRL register."]
   #[inline] fn set_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
      let value = f(Dbgctrl(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x8) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the DBGCTRL register."]
   #[inline] fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Dbgctrl(::core::ptr::read_volatile((self.base() + 0x8) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x8) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the EVCTRL register."]
   #[inline] fn evctrl_ptr(&self) -> *const u16 { 
       <Self as Base>::addr(&self, 0xa)
   }
#[doc="Get the *mut pointer for the EVCTRL register."]
   #[inline] fn evctrl_mut(&self) -> *mut u16 { 
       <Self as Base>::addr(&self, 0xa)
   }
#[doc="Read the EVCTRL register."]
   #[inline] fn evctrl(&self) -> Evctrl { 
      unsafe {
         Evctrl(::core::ptr::read_volatile((self.base() + 0xa) as *const u16))
      }
   }
#[doc="Write the EVCTRL register."]
   #[inline] fn set_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
      let value = f(Evctrl(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the EVCTRL register."]
   #[inline] fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Evctrl(::core::ptr::read_volatile((self.base() + 0xa) as *const u16))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTENCLR register."]
   #[inline] fn intenclr_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0xc)
   }
#[doc="Get the *mut pointer for the INTENCLR register."]
   #[inline] fn intenclr_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0xc)
   }
#[doc="Read the INTENCLR register."]
   #[inline] fn intenclr(&self) -> Intenclr { 
      unsafe {
         Intenclr(::core::ptr::read_volatile((self.base() + 0xc) as *const u8))
      }
   }
#[doc="Write the INTENCLR register."]
   #[inline] fn set_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let value = f(Intenclr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTENCLR register."]
   #[inline] fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Intenclr(::core::ptr::read_volatile((self.base() + 0xc) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTENSET register."]
   #[inline] fn intenset_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0xd)
   }
#[doc="Get the *mut pointer for the INTENSET register."]
   #[inline] fn intenset_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0xd)
   }
#[doc="Read the INTENSET register."]
   #[inline] fn intenset(&self) -> Intenset { 
      unsafe {
         Intenset(::core::ptr::read_volatile((self.base() + 0xd) as *const u8))
      }
   }
#[doc="Write the INTENSET register."]
   #[inline] fn set_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let value = f(Intenset(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTENSET register."]
   #[inline] fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Intenset(::core::ptr::read_volatile((self.base() + 0xd) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTFLAG register."]
   #[inline] fn intflag_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0xe)
   }
#[doc="Get the *mut pointer for the INTFLAG register."]
   #[inline] fn intflag_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0xe)
   }
#[doc="Read the INTFLAG register."]
   #[inline] fn intflag(&self) -> Intflag { 
      unsafe {
         Intflag(::core::ptr::read_volatile((self.base() + 0xe) as *const u8))
      }
   }
#[doc="Write the INTFLAG register."]
   #[inline] fn set_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let value = f(Intflag(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xe) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTFLAG register."]
   #[inline] fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Intflag(::core::ptr::read_volatile((self.base() + 0xe) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xe) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the READREQ register."]
   #[inline] fn readreq_ptr(&self) -> *const u16 { 
       <Self as Base>::addr(&self, 0x2)
   }
#[doc="Get the *mut pointer for the READREQ register."]
   #[inline] fn readreq_mut(&self) -> *mut u16 { 
       <Self as Base>::addr(&self, 0x2)
   }
#[doc="Read the READREQ register."]
   #[inline] fn readreq(&self) -> Readreq { 
      unsafe {
         Readreq(::core::ptr::read_volatile((self.base() + 0x2) as *const u16))
      }
   }
#[doc="Write the READREQ register."]
   #[inline] fn set_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
      let value = f(Readreq(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x2) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the READREQ register."]
   #[inline] fn with_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Readreq(::core::ptr::read_volatile((self.base() + 0x2) as *const u16))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x2) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the STATUS register."]
   #[inline] fn status_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0xf)
   }
#[doc="Get the *mut pointer for the STATUS register."]
   #[inline] fn status_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0xf)
   }
#[doc="Read the STATUS register."]
   #[inline] fn status(&self) -> Status { 
      unsafe {
         Status(::core::ptr::read_volatile((self.base() + 0xf) as *const u8))
      }
   }

}

#[doc="COUNT32 Compare/Capture"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cc(pub u32);
impl Cc {
#[doc="Compare/Capture Value"]
   #[inline] pub fn cc(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Compare/Capture Value"]
   #[inline] pub fn set_cc<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Count(pub u32);
impl Count {
#[doc="Count Value"]
   #[inline] pub fn count(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Count Value"]
   #[inline] pub fn set_count<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u16);
impl Ctrla {
#[doc="Software Reset"]
   #[inline] pub fn swrst(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Software Reset"]
   #[inline] pub fn set_swrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Enable"]
   #[inline] pub fn enable(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Enable"]
   #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="TC Mode"]
   #[inline] pub fn mode(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
   }
#[doc="TC Mode"]
   #[inline] pub fn set_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x3 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Waveform Generation Operation"]
   #[inline] pub fn wavegen(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
   }
#[doc="Waveform Generation Operation"]
   #[inline] pub fn set_wavegen<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x3 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Prescaler"]
   #[inline] pub fn prescaler(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
   }
#[doc="Prescaler"]
   #[inline] pub fn set_prescaler<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x7 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Run in Standby"]
   #[inline] pub fn runstdby(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="Run in Standby"]
   #[inline] pub fn set_runstdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="Prescaler and Counter Synchronization"]
   #[inline] pub fn prescsync(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
   }
#[doc="Prescaler and Counter Synchronization"]
   #[inline] pub fn set_prescsync<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u16 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrlbclr(pub u8);
impl Ctrlbclr {
#[doc="Counter Direction"]
   #[inline] pub fn dir(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Counter Direction"]
   #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="One-Shot"]
   #[inline] pub fn oneshot(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="One-Shot"]
   #[inline] pub fn set_oneshot<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Command"]
   #[inline] pub fn cmd(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
   }
#[doc="Command"]
   #[inline] pub fn set_cmd<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u8 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrlbset(pub u8);
impl Ctrlbset {
#[doc="Counter Direction"]
   #[inline] pub fn dir(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Counter Direction"]
   #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="One-Shot"]
   #[inline] pub fn oneshot(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="One-Shot"]
   #[inline] pub fn set_oneshot<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Command"]
   #[inline] pub fn cmd(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
   }
#[doc="Command"]
   #[inline] pub fn set_cmd<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u8 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrlc(pub u8);
impl Ctrlc {
#[doc="Output Waveform 0 Invert Enable"]
   #[inline] pub fn inven0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Output Waveform 0 Invert Enable"]
   #[inline] pub fn set_inven0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Output Waveform 1 Invert Enable"]
   #[inline] pub fn inven1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Output Waveform 1 Invert Enable"]
   #[inline] pub fn set_inven1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Capture Channel 0 Enable"]
   #[inline] pub fn cpten0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Capture Channel 0 Enable"]
   #[inline] pub fn set_cpten0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Capture Channel 1 Enable"]
   #[inline] pub fn cpten1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Capture Channel 1 Enable"]
   #[inline] pub fn set_cpten1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
#[doc="Debug Run Mode"]
   #[inline] pub fn dbgrun(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Debug Run Mode"]
   #[inline] pub fn set_dbgrun<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u16);
impl Evctrl {
#[doc="Event Action"]
   #[inline] pub fn evact(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
   }
#[doc="Event Action"]
   #[inline] pub fn set_evact<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x7 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="TC Inverted Event Input"]
   #[inline] pub fn tcinv(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="TC Inverted Event Input"]
   #[inline] pub fn set_tcinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="TC Event Input"]
   #[inline] pub fn tcei(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="TC Event Input"]
   #[inline] pub fn set_tcei<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Overflow/Underflow Event Output Enable"]
   #[inline] pub fn ovfeo(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Overflow/Underflow Event Output Enable"]
   #[inline] pub fn set_ovfeo<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Match or Capture Channel 0 Event Output Enable"]
   #[inline] pub fn mceo0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="Match or Capture Channel 0 Event Output Enable"]
   #[inline] pub fn set_mceo0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Match or Capture Channel 1 Event Output Enable"]
   #[inline] pub fn mceo1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="Match or Capture Channel 1 Event Output Enable"]
   #[inline] pub fn set_mceo1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Error Interrupt Enable"]
   #[inline] pub fn err(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Error Interrupt Enable"]
   #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Match or Capture Channel 0 Interrupt Enable"]
   #[inline] pub fn mc0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Match or Capture Channel 0 Interrupt Enable"]
   #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Match or Capture Channel 1 Interrupt Enable"]
   #[inline] pub fn mc1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Match or Capture Channel 1 Interrupt Enable"]
   #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Error Interrupt Enable"]
   #[inline] pub fn err(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Error Interrupt Enable"]
   #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Match or Capture Channel 0 Interrupt Enable"]
   #[inline] pub fn mc0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Match or Capture Channel 0 Interrupt Enable"]
   #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Match or Capture Channel 1 Interrupt Enable"]
   #[inline] pub fn mc1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Match or Capture Channel 1 Interrupt Enable"]
   #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
#[doc="Overflow"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Overflow"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Error"]
   #[inline] pub fn err(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Error"]
   #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Synchronization Ready"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Synchronization Ready"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Match or Capture Channel 0"]
   #[inline] pub fn mc0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Match or Capture Channel 0"]
   #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Match or Capture Channel 1"]
   #[inline] pub fn mc1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Match or Capture Channel 1"]
   #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Readreq(pub u16);
impl Readreq {
#[doc="Address"]
   #[inline] pub fn addr(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
   }
#[doc="Address"]
   #[inline] pub fn set_addr<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1f << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Read Continuously"]
   #[inline] pub fn rcont(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="Read Continuously"]
   #[inline] pub fn set_rcont<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="Read Request"]
   #[inline] pub fn rreq(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Read Request"]
   #[inline] pub fn set_rreq<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
#[doc="Stop"]
   #[inline] pub fn stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Stop"]
   #[inline] pub fn set_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Slave"]
   #[inline] pub fn slave(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Slave"]
   #[inline] pub fn set_slave<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Synchronization Busy"]
   #[inline] pub fn syncbusy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Synchronization Busy"]
   #[inline] pub fn set_syncbusy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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

channel!(TC3_CH0, Tc3Ch0, TC3, Tc3, 0);
channel!(TC3_CH1, Tc3Ch1, TC3, Tc3, 1);
channel!(TC4_CH0, Tc4Ch0, TC4, Tc4, 0);
channel!(TC4_CH1, Tc4Ch1, TC4, Tc4, 1);
channel!(TC5_CH0, Tc5Ch0, TC5, Tc5, 0);
channel!(TC5_CH1, Tc5Ch1, TC5, Tc5, 1);
