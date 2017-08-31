//! Real-Time Counter
#[allow(unused_imports)] use bobbin_common::*;

periph!(RTC, Rtc, 0x40001400);

#[doc="Real-Time Counter"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rtc(pub usize);
impl Rtc {
#[doc="Get 32-bit Counter with Single 32-bit Compare Peripheral"]
   #[inline] pub fn mode0(&self) -> mode0::Mode0 {
      mode0::Mode0(self.0 + 0x0)
   }
#[doc="Get 16-bit Counter with Two 16-bit Compares Peripheral"]
   #[inline] pub fn mode1(&self) -> mode1::Mode1 {
      mode1::Mode1(self.0 + 0x0)
   }
#[doc="Get Clock/Calendar with Alarm Peripheral"]
   #[inline] pub fn mode2(&self) -> mode2::Mode2 {
      mode2::Mode2(self.0 + 0x0)
   }
}
#[doc="32-bit Counter with Single 32-bit Compare Cluster"]
pub mod mode0 {
#[allow(unused_imports)] use bobbin_common::*;
   #[derive(Clone, Copy, PartialEq, Eq)]
#[doc="32-bit Counter with Single 32-bit Compare Peripheral"]
   pub struct Mode0(pub usize);
impl Mode0 {
#[doc="Get the *const pointer for the DBGCTRL register."]
   #[inline] pub fn dbgctrl_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xb) as *const u8
   }
#[doc="Get the *mut pointer for the DBGCTRL register."]
   #[inline] pub fn dbgctrl_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xb) as *mut u8
   }
#[doc="Read the DBGCTRL register."]
   #[inline] pub fn dbgctrl(&self) -> Dbgctrl { 
      unsafe {
         Dbgctrl(read_volatile((self.0 + 0xb) as *const u8))
      }
   }
#[doc="Write the DBGCTRL register."]
   #[inline] pub fn set_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
      let value = f(Dbgctrl(0));
      unsafe {
         write_volatile((self.0 + 0xb) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the DBGCTRL register."]
   #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
      let tmp = self.dbgctrl();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xb) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the FREQCORR register."]
   #[inline] pub fn freqcorr_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xc) as *const u8
   }
#[doc="Get the *mut pointer for the FREQCORR register."]
   #[inline] pub fn freqcorr_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xc) as *mut u8
   }
#[doc="Read the FREQCORR register."]
   #[inline] pub fn freqcorr(&self) -> Freqcorr { 
      unsafe {
         Freqcorr(read_volatile((self.0 + 0xc) as *const u8))
      }
   }
#[doc="Write the FREQCORR register."]
   #[inline] pub fn set_freqcorr<F: FnOnce(Freqcorr) -> Freqcorr>(&self, f: F) -> &Self {
      let value = f(Freqcorr(0));
      unsafe {
         write_volatile((self.0 + 0xc) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the FREQCORR register."]
   #[inline] pub fn with_freqcorr<F: FnOnce(Freqcorr) -> Freqcorr>(&self, f: F) -> &Self {
      let tmp = self.freqcorr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xc) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the COMP register."]
   #[inline] pub fn comp_ptr<I: Into<bits::R1>>(&self, index: I) -> *const u32 { 
      let index: bits::R1 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x18 + (index << 2)) as *const u32
   }
#[doc="Get the *mut pointer for the COMP register."]
   #[inline] pub fn comp_mut<I: Into<bits::R1>>(&self, index: I) -> *mut u32 { 
      let index: bits::R1 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x18 + (index << 2)) as *mut u32
   }
#[doc="Read the COMP register."]
   #[inline] pub fn comp<I: Into<bits::R1>>(&self, index: I) -> Comp { 
      let index: bits::R1 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Comp(read_volatile((self.0 + 0x18 + (index << 2)) as *const u32))
      }
   }
#[doc="Write the COMP register."]
   #[inline] pub fn set_comp<I: Into<bits::R1>, F: FnOnce(Comp) -> Comp>(&self, index: I, f: F) -> &Self {
      let index: bits::R1 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Comp(0));
      unsafe {
         write_volatile((self.0 + 0x18 + (index << 2)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the COMP register."]
   #[inline] pub fn with_comp<I: Into<bits::R1> + Copy, F: FnOnce(Comp) -> Comp>(&self, index: I, f: F) -> &Self {
      let index: bits::R1 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.comp(index);
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x18 + (index << 2)) as *mut u32, value.0);
      }
      self
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
         Count(read_volatile((self.0 + 0x10) as *const u32))
      }
   }
#[doc="Write the COUNT register."]
   #[inline] pub fn set_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
      let value = f(Count(0));
      unsafe {
         write_volatile((self.0 + 0x10) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the COUNT register."]
   #[inline] pub fn with_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
      let tmp = self.count();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x10) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRL register."]
   #[inline] pub fn ctrl_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0x0) as *const u16
   }
#[doc="Get the *mut pointer for the CTRL register."]
   #[inline] pub fn ctrl_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0x0) as *mut u16
   }
#[doc="Read the CTRL register."]
   #[inline] pub fn ctrl(&self) -> Ctrl { 
      unsafe {
         Ctrl(read_volatile((self.0 + 0x0) as *const u16))
      }
   }
#[doc="Write the CTRL register."]
   #[inline] pub fn set_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
      let value = f(Ctrl(0));
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the CTRL register."]
   #[inline] pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
      let tmp = self.ctrl();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the EVCTRL register."]
   #[inline] pub fn evctrl_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0x4) as *const u16
   }
#[doc="Get the *mut pointer for the EVCTRL register."]
   #[inline] pub fn evctrl_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0x4) as *mut u16
   }
#[doc="Read the EVCTRL register."]
   #[inline] pub fn evctrl(&self) -> Evctrl { 
      unsafe {
         Evctrl(read_volatile((self.0 + 0x4) as *const u16))
      }
   }
#[doc="Write the EVCTRL register."]
   #[inline] pub fn set_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
      let value = f(Evctrl(0));
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the EVCTRL register."]
   #[inline] pub fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
      let tmp = self.evctrl();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTENCLR register."]
   #[inline] pub fn intenclr_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x6) as *const u8
   }
#[doc="Get the *mut pointer for the INTENCLR register."]
   #[inline] pub fn intenclr_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x6) as *mut u8
   }
#[doc="Read the INTENCLR register."]
   #[inline] pub fn intenclr(&self) -> Intenclr { 
      unsafe {
         Intenclr(read_volatile((self.0 + 0x6) as *const u8))
      }
   }
#[doc="Write the INTENCLR register."]
   #[inline] pub fn set_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let value = f(Intenclr(0));
      unsafe {
         write_volatile((self.0 + 0x6) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTENCLR register."]
   #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let tmp = self.intenclr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x6) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTENSET register."]
   #[inline] pub fn intenset_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x7) as *const u8
   }
#[doc="Get the *mut pointer for the INTENSET register."]
   #[inline] pub fn intenset_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x7) as *mut u8
   }
#[doc="Read the INTENSET register."]
   #[inline] pub fn intenset(&self) -> Intenset { 
      unsafe {
         Intenset(read_volatile((self.0 + 0x7) as *const u8))
      }
   }
#[doc="Write the INTENSET register."]
   #[inline] pub fn set_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let value = f(Intenset(0));
      unsafe {
         write_volatile((self.0 + 0x7) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTENSET register."]
   #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let tmp = self.intenset();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x7) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTFLAG register."]
   #[inline] pub fn intflag_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x8) as *const u8
   }
#[doc="Get the *mut pointer for the INTFLAG register."]
   #[inline] pub fn intflag_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x8) as *mut u8
   }
#[doc="Read the INTFLAG register."]
   #[inline] pub fn intflag(&self) -> Intflag { 
      unsafe {
         Intflag(read_volatile((self.0 + 0x8) as *const u8))
      }
   }
#[doc="Write the INTFLAG register."]
   #[inline] pub fn set_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let value = f(Intflag(0));
      unsafe {
         write_volatile((self.0 + 0x8) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTFLAG register."]
   #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let tmp = self.intflag();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x8) as *mut u8, value.0);
      }
      self
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
         Readreq(read_volatile((self.0 + 0x2) as *const u16))
      }
   }
#[doc="Write the READREQ register."]
   #[inline] pub fn set_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
      let value = f(Readreq(0));
      unsafe {
         write_volatile((self.0 + 0x2) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the READREQ register."]
   #[inline] pub fn with_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
      let tmp = self.readreq();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x2) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the STATUS register."]
   #[inline] pub fn status_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xa) as *const u8
   }
#[doc="Get the *mut pointer for the STATUS register."]
   #[inline] pub fn status_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xa) as *mut u8
   }
#[doc="Read the STATUS register."]
   #[inline] pub fn status(&self) -> Status { 
      unsafe {
         Status(read_volatile((self.0 + 0xa) as *const u8))
      }
   }
#[doc="Write the STATUS register."]
   #[inline] pub fn set_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
      let value = f(Status(0));
      unsafe {
         write_volatile((self.0 + 0xa) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the STATUS register."]
   #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
      let tmp = self.status();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xa) as *mut u8, value.0);
      }
      self
   }

}

#[doc="Debug Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
#[doc="Run During Debug"]
   #[inline] pub fn dbgrun(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Run During Debug"]
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
#[doc="Frequency Correction"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Freqcorr(pub u8);
impl Freqcorr {
#[doc="Correction Value"]
   #[inline] pub fn value(&self) -> bits::U7 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
   }
#[doc="Correction Value"]
   #[inline] pub fn set_value<V: Into<bits::U7>>(mut self, value: V) -> Self {
      let value: bits::U7 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x7f << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Correction Sign"]
   #[inline] pub fn sign(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Correction Sign"]
   #[inline] pub fn set_sign<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[doc="MODE0 Compare n Value"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Comp(pub u32);
impl Comp {
#[doc="Compare Value"]
   #[inline] pub fn comp(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Compare Value"]
   #[inline] pub fn set_comp<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
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
#[doc="MODE0 Counter Value"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Count(pub u32);
impl Count {
#[doc="Counter Value"]
   #[inline] pub fn count(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Counter Value"]
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
#[doc="MODE0 Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u16);
impl Ctrl {
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

#[doc="Operating Mode"]
   #[inline] pub fn mode(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
   }
#[doc="Operating Mode"]
   #[inline] pub fn set_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x3 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Clear on Match"]
   #[inline] pub fn matchclr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Clear on Match"]
   #[inline] pub fn set_matchclr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="Prescaler"]
   #[inline] pub fn prescaler(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
   }
#[doc="Prescaler"]
   #[inline] pub fn set_prescaler<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u16 = value.into();
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
#[doc="MODE0 Event Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u16);
impl Evctrl {
#[doc="Periodic Interval 0 Event Output Enable"]
   #[inline] pub fn pereo0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Periodic Interval 0 Event Output Enable"]
   #[inline] pub fn set_pereo0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Periodic Interval 1 Event Output Enable"]
   #[inline] pub fn pereo1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Periodic Interval 1 Event Output Enable"]
   #[inline] pub fn set_pereo1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Periodic Interval 2 Event Output Enable"]
   #[inline] pub fn pereo2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Periodic Interval 2 Event Output Enable"]
   #[inline] pub fn set_pereo2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Periodic Interval 3 Event Output Enable"]
   #[inline] pub fn pereo3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Periodic Interval 3 Event Output Enable"]
   #[inline] pub fn set_pereo3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Periodic Interval 4 Event Output Enable"]
   #[inline] pub fn pereo4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Periodic Interval 4 Event Output Enable"]
   #[inline] pub fn set_pereo4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Periodic Interval 5 Event Output Enable"]
   #[inline] pub fn pereo5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Periodic Interval 5 Event Output Enable"]
   #[inline] pub fn set_pereo5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Periodic Interval 6 Event Output Enable"]
   #[inline] pub fn pereo6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Periodic Interval 6 Event Output Enable"]
   #[inline] pub fn set_pereo6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Periodic Interval 7 Event Output Enable"]
   #[inline] pub fn pereo7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Periodic Interval 7 Event Output Enable"]
   #[inline] pub fn set_pereo7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="Compare 0 Event Output Enable"]
   #[inline] pub fn cmpeo0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Compare 0 Event Output Enable"]
   #[inline] pub fn set_cmpeo0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Overflow Event Output Enable"]
   #[inline] pub fn ovfeo(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Overflow Event Output Enable"]
   #[inline] pub fn set_ovfeo<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
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
#[doc="MODE0 Interrupt Enable Clear"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
#[doc="Compare 0 Interrupt Enable"]
   #[inline] pub fn cmp0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Compare 0 Interrupt Enable"]
   #[inline] pub fn set_cmp0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[doc="MODE0 Interrupt Enable Set"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
#[doc="Compare 0 Interrupt Enable"]
   #[inline] pub fn cmp0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Compare 0 Interrupt Enable"]
   #[inline] pub fn set_cmp0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[doc="MODE0 Interrupt Flag Status and Clear"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
#[doc="Compare 0"]
   #[inline] pub fn cmp0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Compare 0"]
   #[inline] pub fn set_cmp0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Synchronization Ready"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Synchronization Ready"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Overflow"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Overflow"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[doc="Read Request"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Readreq(pub u16);
impl Readreq {
#[doc="Address"]
   #[inline] pub fn addr(&self) -> bits::U6 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
   }
#[doc="Address"]
   #[inline] pub fn set_addr<V: Into<bits::U6>>(mut self, value: V) -> Self {
      let value: bits::U6 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x3f << 0);
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
      if self.syncbusy() != 0 { try!(write!(f, " syncbusy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
}
// End of mode0
#[doc="16-bit Counter with Two 16-bit Compares Cluster"]
pub mod mode1 {
#[allow(unused_imports)] use bobbin_common::*;
   #[derive(Clone, Copy, PartialEq, Eq)]
#[doc="16-bit Counter with Two 16-bit Compares Peripheral"]
   pub struct Mode1(pub usize);
impl Mode1 {
#[doc="Get the *const pointer for the DBGCTRL register."]
   #[inline] pub fn dbgctrl_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xb) as *const u8
   }
#[doc="Get the *mut pointer for the DBGCTRL register."]
   #[inline] pub fn dbgctrl_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xb) as *mut u8
   }
#[doc="Read the DBGCTRL register."]
   #[inline] pub fn dbgctrl(&self) -> Dbgctrl { 
      unsafe {
         Dbgctrl(read_volatile((self.0 + 0xb) as *const u8))
      }
   }
#[doc="Write the DBGCTRL register."]
   #[inline] pub fn set_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
      let value = f(Dbgctrl(0));
      unsafe {
         write_volatile((self.0 + 0xb) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the DBGCTRL register."]
   #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
      let tmp = self.dbgctrl();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xb) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the FREQCORR register."]
   #[inline] pub fn freqcorr_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xc) as *const u8
   }
#[doc="Get the *mut pointer for the FREQCORR register."]
   #[inline] pub fn freqcorr_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xc) as *mut u8
   }
#[doc="Read the FREQCORR register."]
   #[inline] pub fn freqcorr(&self) -> Freqcorr { 
      unsafe {
         Freqcorr(read_volatile((self.0 + 0xc) as *const u8))
      }
   }
#[doc="Write the FREQCORR register."]
   #[inline] pub fn set_freqcorr<F: FnOnce(Freqcorr) -> Freqcorr>(&self, f: F) -> &Self {
      let value = f(Freqcorr(0));
      unsafe {
         write_volatile((self.0 + 0xc) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the FREQCORR register."]
   #[inline] pub fn with_freqcorr<F: FnOnce(Freqcorr) -> Freqcorr>(&self, f: F) -> &Self {
      let tmp = self.freqcorr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xc) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the COMP register."]
   #[inline] pub fn comp_ptr<I: Into<bits::R2>>(&self, index: I) -> *const u16 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x18 + (index << 1)) as *const u16
   }
#[doc="Get the *mut pointer for the COMP register."]
   #[inline] pub fn comp_mut<I: Into<bits::R2>>(&self, index: I) -> *mut u16 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x18 + (index << 1)) as *mut u16
   }
#[doc="Read the COMP register."]
   #[inline] pub fn comp<I: Into<bits::R2>>(&self, index: I) -> Comp { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Comp(read_volatile((self.0 + 0x18 + (index << 1)) as *const u16))
      }
   }
#[doc="Write the COMP register."]
   #[inline] pub fn set_comp<I: Into<bits::R2>, F: FnOnce(Comp) -> Comp>(&self, index: I, f: F) -> &Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Comp(0));
      unsafe {
         write_volatile((self.0 + 0x18 + (index << 1)) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the COMP register."]
   #[inline] pub fn with_comp<I: Into<bits::R2> + Copy, F: FnOnce(Comp) -> Comp>(&self, index: I, f: F) -> &Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.comp(index);
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x18 + (index << 1)) as *mut u16, value.0);
      }
      self
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
         Count(read_volatile((self.0 + 0x10) as *const u16))
      }
   }
#[doc="Write the COUNT register."]
   #[inline] pub fn set_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
      let value = f(Count(0));
      unsafe {
         write_volatile((self.0 + 0x10) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the COUNT register."]
   #[inline] pub fn with_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
      let tmp = self.count();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x10) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRL register."]
   #[inline] pub fn ctrl_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0x0) as *const u16
   }
#[doc="Get the *mut pointer for the CTRL register."]
   #[inline] pub fn ctrl_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0x0) as *mut u16
   }
#[doc="Read the CTRL register."]
   #[inline] pub fn ctrl(&self) -> Ctrl { 
      unsafe {
         Ctrl(read_volatile((self.0 + 0x0) as *const u16))
      }
   }
#[doc="Write the CTRL register."]
   #[inline] pub fn set_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
      let value = f(Ctrl(0));
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the CTRL register."]
   #[inline] pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
      let tmp = self.ctrl();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the EVCTRL register."]
   #[inline] pub fn evctrl_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0x4) as *const u16
   }
#[doc="Get the *mut pointer for the EVCTRL register."]
   #[inline] pub fn evctrl_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0x4) as *mut u16
   }
#[doc="Read the EVCTRL register."]
   #[inline] pub fn evctrl(&self) -> Evctrl { 
      unsafe {
         Evctrl(read_volatile((self.0 + 0x4) as *const u16))
      }
   }
#[doc="Write the EVCTRL register."]
   #[inline] pub fn set_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
      let value = f(Evctrl(0));
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the EVCTRL register."]
   #[inline] pub fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
      let tmp = self.evctrl();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTENCLR register."]
   #[inline] pub fn intenclr_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x6) as *const u8
   }
#[doc="Get the *mut pointer for the INTENCLR register."]
   #[inline] pub fn intenclr_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x6) as *mut u8
   }
#[doc="Read the INTENCLR register."]
   #[inline] pub fn intenclr(&self) -> Intenclr { 
      unsafe {
         Intenclr(read_volatile((self.0 + 0x6) as *const u8))
      }
   }
#[doc="Write the INTENCLR register."]
   #[inline] pub fn set_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let value = f(Intenclr(0));
      unsafe {
         write_volatile((self.0 + 0x6) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTENCLR register."]
   #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let tmp = self.intenclr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x6) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTENSET register."]
   #[inline] pub fn intenset_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x7) as *const u8
   }
#[doc="Get the *mut pointer for the INTENSET register."]
   #[inline] pub fn intenset_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x7) as *mut u8
   }
#[doc="Read the INTENSET register."]
   #[inline] pub fn intenset(&self) -> Intenset { 
      unsafe {
         Intenset(read_volatile((self.0 + 0x7) as *const u8))
      }
   }
#[doc="Write the INTENSET register."]
   #[inline] pub fn set_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let value = f(Intenset(0));
      unsafe {
         write_volatile((self.0 + 0x7) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTENSET register."]
   #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let tmp = self.intenset();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x7) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTFLAG register."]
   #[inline] pub fn intflag_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x8) as *const u8
   }
#[doc="Get the *mut pointer for the INTFLAG register."]
   #[inline] pub fn intflag_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x8) as *mut u8
   }
#[doc="Read the INTFLAG register."]
   #[inline] pub fn intflag(&self) -> Intflag { 
      unsafe {
         Intflag(read_volatile((self.0 + 0x8) as *const u8))
      }
   }
#[doc="Write the INTFLAG register."]
   #[inline] pub fn set_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let value = f(Intflag(0));
      unsafe {
         write_volatile((self.0 + 0x8) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTFLAG register."]
   #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let tmp = self.intflag();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x8) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PER register."]
   #[inline] pub fn per_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0x14) as *const u16
   }
#[doc="Get the *mut pointer for the PER register."]
   #[inline] pub fn per_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0x14) as *mut u16
   }
#[doc="Read the PER register."]
   #[inline] pub fn per(&self) -> Per { 
      unsafe {
         Per(read_volatile((self.0 + 0x14) as *const u16))
      }
   }
#[doc="Write the PER register."]
   #[inline] pub fn set_per<F: FnOnce(Per) -> Per>(&self, f: F) -> &Self {
      let value = f(Per(0));
      unsafe {
         write_volatile((self.0 + 0x14) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the PER register."]
   #[inline] pub fn with_per<F: FnOnce(Per) -> Per>(&self, f: F) -> &Self {
      let tmp = self.per();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x14) as *mut u16, value.0);
      }
      self
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
         Readreq(read_volatile((self.0 + 0x2) as *const u16))
      }
   }
#[doc="Write the READREQ register."]
   #[inline] pub fn set_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
      let value = f(Readreq(0));
      unsafe {
         write_volatile((self.0 + 0x2) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the READREQ register."]
   #[inline] pub fn with_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
      let tmp = self.readreq();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x2) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the STATUS register."]
   #[inline] pub fn status_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xa) as *const u8
   }
#[doc="Get the *mut pointer for the STATUS register."]
   #[inline] pub fn status_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xa) as *mut u8
   }
#[doc="Read the STATUS register."]
   #[inline] pub fn status(&self) -> Status { 
      unsafe {
         Status(read_volatile((self.0 + 0xa) as *const u8))
      }
   }
#[doc="Write the STATUS register."]
   #[inline] pub fn set_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
      let value = f(Status(0));
      unsafe {
         write_volatile((self.0 + 0xa) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the STATUS register."]
   #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
      let tmp = self.status();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xa) as *mut u8, value.0);
      }
      self
   }

}

#[doc="Debug Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
#[doc="Run During Debug"]
   #[inline] pub fn dbgrun(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Run During Debug"]
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
#[doc="Frequency Correction"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Freqcorr(pub u8);
impl Freqcorr {
#[doc="Correction Value"]
   #[inline] pub fn value(&self) -> bits::U7 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
   }
#[doc="Correction Value"]
   #[inline] pub fn set_value<V: Into<bits::U7>>(mut self, value: V) -> Self {
      let value: bits::U7 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x7f << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Correction Sign"]
   #[inline] pub fn sign(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Correction Sign"]
   #[inline] pub fn set_sign<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[doc="MODE1 Compare n Value"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Comp(pub u16);
impl Comp {
#[doc="Compare Value"]
   #[inline] pub fn comp(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Compare Value"]
   #[inline] pub fn set_comp<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u16 = value.into();
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
#[doc="MODE1 Counter Value"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Count(pub u16);
impl Count {
#[doc="Counter Value"]
   #[inline] pub fn count(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Counter Value"]
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
#[doc="MODE1 Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u16);
impl Ctrl {
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

#[doc="Operating Mode"]
   #[inline] pub fn mode(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
   }
#[doc="Operating Mode"]
   #[inline] pub fn set_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x3 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Prescaler"]
   #[inline] pub fn prescaler(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
   }
#[doc="Prescaler"]
   #[inline] pub fn set_prescaler<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u16 = value.into();
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
#[doc="MODE1 Event Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u16);
impl Evctrl {
#[doc="Periodic Interval 0 Event Output Enable"]
   #[inline] pub fn pereo0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Periodic Interval 0 Event Output Enable"]
   #[inline] pub fn set_pereo0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Periodic Interval 1 Event Output Enable"]
   #[inline] pub fn pereo1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Periodic Interval 1 Event Output Enable"]
   #[inline] pub fn set_pereo1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Periodic Interval 2 Event Output Enable"]
   #[inline] pub fn pereo2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Periodic Interval 2 Event Output Enable"]
   #[inline] pub fn set_pereo2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Periodic Interval 3 Event Output Enable"]
   #[inline] pub fn pereo3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Periodic Interval 3 Event Output Enable"]
   #[inline] pub fn set_pereo3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Periodic Interval 4 Event Output Enable"]
   #[inline] pub fn pereo4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Periodic Interval 4 Event Output Enable"]
   #[inline] pub fn set_pereo4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Periodic Interval 5 Event Output Enable"]
   #[inline] pub fn pereo5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Periodic Interval 5 Event Output Enable"]
   #[inline] pub fn set_pereo5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Periodic Interval 6 Event Output Enable"]
   #[inline] pub fn pereo6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Periodic Interval 6 Event Output Enable"]
   #[inline] pub fn set_pereo6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Periodic Interval 7 Event Output Enable"]
   #[inline] pub fn pereo7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Periodic Interval 7 Event Output Enable"]
   #[inline] pub fn set_pereo7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="Compare 0 Event Output Enable"]
   #[inline] pub fn cmpeo0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Compare 0 Event Output Enable"]
   #[inline] pub fn set_cmpeo0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Compare 1 Event Output Enable"]
   #[inline] pub fn cmpeo1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="Compare 1 Event Output Enable"]
   #[inline] pub fn set_cmpeo1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="Overflow Event Output Enable"]
   #[inline] pub fn ovfeo(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Overflow Event Output Enable"]
   #[inline] pub fn set_ovfeo<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
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
#[doc="MODE1 Interrupt Enable Clear"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
#[doc="Compare 0 Interrupt Enable"]
   #[inline] pub fn cmp0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Compare 0 Interrupt Enable"]
   #[inline] pub fn set_cmp0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Compare 1 Interrupt Enable"]
   #[inline] pub fn cmp1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Compare 1 Interrupt Enable"]
   #[inline] pub fn set_cmp1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[doc="MODE1 Interrupt Enable Set"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
#[doc="Compare 0 Interrupt Enable"]
   #[inline] pub fn cmp0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Compare 0 Interrupt Enable"]
   #[inline] pub fn set_cmp0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Compare 1 Interrupt Enable"]
   #[inline] pub fn cmp1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Compare 1 Interrupt Enable"]
   #[inline] pub fn set_cmp1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[doc="MODE1 Interrupt Flag Status and Clear"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
#[doc="Compare 0"]
   #[inline] pub fn cmp0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Compare 0"]
   #[inline] pub fn set_cmp0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Compare 1"]
   #[inline] pub fn cmp1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Compare 1"]
   #[inline] pub fn set_cmp1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Synchronization Ready"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Synchronization Ready"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Overflow"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Overflow"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[doc="MODE1 Counter Period"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Per(pub u16);
impl Per {
#[doc="Counter Period"]
   #[inline] pub fn per(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Counter Period"]
   #[inline] pub fn set_per<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u16 = value.into();
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
#[doc="Read Request"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Readreq(pub u16);
impl Readreq {
#[doc="Address"]
   #[inline] pub fn addr(&self) -> bits::U6 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
   }
#[doc="Address"]
   #[inline] pub fn set_addr<V: Into<bits::U6>>(mut self, value: V) -> Self {
      let value: bits::U6 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x3f << 0);
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
      if self.syncbusy() != 0 { try!(write!(f, " syncbusy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
}
// End of mode1
#[doc="Clock/Calendar with Alarm Cluster"]
pub mod mode2 {
#[allow(unused_imports)] use bobbin_common::*;
   #[derive(Clone, Copy, PartialEq, Eq)]
#[doc="Clock/Calendar with Alarm Peripheral"]
   pub struct Mode2(pub usize);
impl Mode2 {
#[doc="Get the *const pointer for the DBGCTRL register."]
   #[inline] pub fn dbgctrl_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xb) as *const u8
   }
#[doc="Get the *mut pointer for the DBGCTRL register."]
   #[inline] pub fn dbgctrl_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xb) as *mut u8
   }
#[doc="Read the DBGCTRL register."]
   #[inline] pub fn dbgctrl(&self) -> Dbgctrl { 
      unsafe {
         Dbgctrl(read_volatile((self.0 + 0xb) as *const u8))
      }
   }
#[doc="Write the DBGCTRL register."]
   #[inline] pub fn set_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
      let value = f(Dbgctrl(0));
      unsafe {
         write_volatile((self.0 + 0xb) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the DBGCTRL register."]
   #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
      let tmp = self.dbgctrl();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xb) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the FREQCORR register."]
   #[inline] pub fn freqcorr_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xc) as *const u8
   }
#[doc="Get the *mut pointer for the FREQCORR register."]
   #[inline] pub fn freqcorr_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xc) as *mut u8
   }
#[doc="Read the FREQCORR register."]
   #[inline] pub fn freqcorr(&self) -> Freqcorr { 
      unsafe {
         Freqcorr(read_volatile((self.0 + 0xc) as *const u8))
      }
   }
#[doc="Write the FREQCORR register."]
   #[inline] pub fn set_freqcorr<F: FnOnce(Freqcorr) -> Freqcorr>(&self, f: F) -> &Self {
      let value = f(Freqcorr(0));
      unsafe {
         write_volatile((self.0 + 0xc) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the FREQCORR register."]
   #[inline] pub fn with_freqcorr<F: FnOnce(Freqcorr) -> Freqcorr>(&self, f: F) -> &Self {
      let tmp = self.freqcorr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xc) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLOCK register."]
   #[inline] pub fn clock_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x10) as *const u32
   }
#[doc="Get the *mut pointer for the CLOCK register."]
   #[inline] pub fn clock_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x10) as *mut u32
   }
#[doc="Read the CLOCK register."]
   #[inline] pub fn clock(&self) -> Clock { 
      unsafe {
         Clock(read_volatile((self.0 + 0x10) as *const u32))
      }
   }
#[doc="Write the CLOCK register."]
   #[inline] pub fn set_clock<F: FnOnce(Clock) -> Clock>(&self, f: F) -> &Self {
      let value = f(Clock(0));
      unsafe {
         write_volatile((self.0 + 0x10) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLOCK register."]
   #[inline] pub fn with_clock<F: FnOnce(Clock) -> Clock>(&self, f: F) -> &Self {
      let tmp = self.clock();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x10) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRL register."]
   #[inline] pub fn ctrl_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0x0) as *const u16
   }
#[doc="Get the *mut pointer for the CTRL register."]
   #[inline] pub fn ctrl_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0x0) as *mut u16
   }
#[doc="Read the CTRL register."]
   #[inline] pub fn ctrl(&self) -> Ctrl { 
      unsafe {
         Ctrl(read_volatile((self.0 + 0x0) as *const u16))
      }
   }
#[doc="Write the CTRL register."]
   #[inline] pub fn set_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
      let value = f(Ctrl(0));
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the CTRL register."]
   #[inline] pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
      let tmp = self.ctrl();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the EVCTRL register."]
   #[inline] pub fn evctrl_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0x4) as *const u16
   }
#[doc="Get the *mut pointer for the EVCTRL register."]
   #[inline] pub fn evctrl_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0x4) as *mut u16
   }
#[doc="Read the EVCTRL register."]
   #[inline] pub fn evctrl(&self) -> Evctrl { 
      unsafe {
         Evctrl(read_volatile((self.0 + 0x4) as *const u16))
      }
   }
#[doc="Write the EVCTRL register."]
   #[inline] pub fn set_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
      let value = f(Evctrl(0));
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the EVCTRL register."]
   #[inline] pub fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
      let tmp = self.evctrl();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTENCLR register."]
   #[inline] pub fn intenclr_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x6) as *const u8
   }
#[doc="Get the *mut pointer for the INTENCLR register."]
   #[inline] pub fn intenclr_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x6) as *mut u8
   }
#[doc="Read the INTENCLR register."]
   #[inline] pub fn intenclr(&self) -> Intenclr { 
      unsafe {
         Intenclr(read_volatile((self.0 + 0x6) as *const u8))
      }
   }
#[doc="Write the INTENCLR register."]
   #[inline] pub fn set_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let value = f(Intenclr(0));
      unsafe {
         write_volatile((self.0 + 0x6) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTENCLR register."]
   #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let tmp = self.intenclr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x6) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTENSET register."]
   #[inline] pub fn intenset_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x7) as *const u8
   }
#[doc="Get the *mut pointer for the INTENSET register."]
   #[inline] pub fn intenset_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x7) as *mut u8
   }
#[doc="Read the INTENSET register."]
   #[inline] pub fn intenset(&self) -> Intenset { 
      unsafe {
         Intenset(read_volatile((self.0 + 0x7) as *const u8))
      }
   }
#[doc="Write the INTENSET register."]
   #[inline] pub fn set_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let value = f(Intenset(0));
      unsafe {
         write_volatile((self.0 + 0x7) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTENSET register."]
   #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let tmp = self.intenset();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x7) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTFLAG register."]
   #[inline] pub fn intflag_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x8) as *const u8
   }
#[doc="Get the *mut pointer for the INTFLAG register."]
   #[inline] pub fn intflag_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x8) as *mut u8
   }
#[doc="Read the INTFLAG register."]
   #[inline] pub fn intflag(&self) -> Intflag { 
      unsafe {
         Intflag(read_volatile((self.0 + 0x8) as *const u8))
      }
   }
#[doc="Write the INTFLAG register."]
   #[inline] pub fn set_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let value = f(Intflag(0));
      unsafe {
         write_volatile((self.0 + 0x8) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTFLAG register."]
   #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let tmp = self.intflag();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x8) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the ALARM register."]
   #[inline] pub fn alarm_ptr<I: Into<bits::R1>>(&self, index: I) -> *const u32 { 
      let index: bits::R1 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x18 + (index << 3)) as *const u32
   }
#[doc="Get the *mut pointer for the ALARM register."]
   #[inline] pub fn alarm_mut<I: Into<bits::R1>>(&self, index: I) -> *mut u32 { 
      let index: bits::R1 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x18 + (index << 3)) as *mut u32
   }
#[doc="Read the ALARM register."]
   #[inline] pub fn alarm<I: Into<bits::R1>>(&self, index: I) -> Alarm { 
      let index: bits::R1 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Alarm(read_volatile((self.0 + 0x18 + (index << 3)) as *const u32))
      }
   }
#[doc="Write the ALARM register."]
   #[inline] pub fn set_alarm<I: Into<bits::R1>, F: FnOnce(Alarm) -> Alarm>(&self, index: I, f: F) -> &Self {
      let index: bits::R1 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Alarm(0));
      unsafe {
         write_volatile((self.0 + 0x18 + (index << 3)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the ALARM register."]
   #[inline] pub fn with_alarm<I: Into<bits::R1> + Copy, F: FnOnce(Alarm) -> Alarm>(&self, index: I, f: F) -> &Self {
      let index: bits::R1 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.alarm(index);
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x18 + (index << 3)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MASK register."]
   #[inline] pub fn mask_ptr<I: Into<bits::R1>>(&self, index: I) -> *const u8 { 
      let index: bits::R1 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x1c + (index << 3)) as *const u8
   }
#[doc="Get the *mut pointer for the MASK register."]
   #[inline] pub fn mask_mut<I: Into<bits::R1>>(&self, index: I) -> *mut u8 { 
      let index: bits::R1 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x1c + (index << 3)) as *mut u8
   }
#[doc="Read the MASK register."]
   #[inline] pub fn mask<I: Into<bits::R1>>(&self, index: I) -> Mask { 
      let index: bits::R1 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Mask(read_volatile((self.0 + 0x1c + (index << 3)) as *const u8))
      }
   }
#[doc="Write the MASK register."]
   #[inline] pub fn set_mask<I: Into<bits::R1>, F: FnOnce(Mask) -> Mask>(&self, index: I, f: F) -> &Self {
      let index: bits::R1 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Mask(0));
      unsafe {
         write_volatile((self.0 + 0x1c + (index << 3)) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the MASK register."]
   #[inline] pub fn with_mask<I: Into<bits::R1> + Copy, F: FnOnce(Mask) -> Mask>(&self, index: I, f: F) -> &Self {
      let index: bits::R1 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.mask(index);
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x1c + (index << 3)) as *mut u8, value.0);
      }
      self
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
         Readreq(read_volatile((self.0 + 0x2) as *const u16))
      }
   }
#[doc="Write the READREQ register."]
   #[inline] pub fn set_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
      let value = f(Readreq(0));
      unsafe {
         write_volatile((self.0 + 0x2) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the READREQ register."]
   #[inline] pub fn with_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
      let tmp = self.readreq();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x2) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the STATUS register."]
   #[inline] pub fn status_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xa) as *const u8
   }
#[doc="Get the *mut pointer for the STATUS register."]
   #[inline] pub fn status_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xa) as *mut u8
   }
#[doc="Read the STATUS register."]
   #[inline] pub fn status(&self) -> Status { 
      unsafe {
         Status(read_volatile((self.0 + 0xa) as *const u8))
      }
   }
#[doc="Write the STATUS register."]
   #[inline] pub fn set_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
      let value = f(Status(0));
      unsafe {
         write_volatile((self.0 + 0xa) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the STATUS register."]
   #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
      let tmp = self.status();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xa) as *mut u8, value.0);
      }
      self
   }

}

#[doc="Debug Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
#[doc="Run During Debug"]
   #[inline] pub fn dbgrun(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Run During Debug"]
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
#[doc="Frequency Correction"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Freqcorr(pub u8);
impl Freqcorr {
#[doc="Correction Value"]
   #[inline] pub fn value(&self) -> bits::U7 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
   }
#[doc="Correction Value"]
   #[inline] pub fn set_value<V: Into<bits::U7>>(mut self, value: V) -> Self {
      let value: bits::U7 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x7f << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Correction Sign"]
   #[inline] pub fn sign(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Correction Sign"]
   #[inline] pub fn set_sign<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[doc="MODE2 Clock Value"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clock(pub u32);
impl Clock {
#[doc="Second"]
   #[inline] pub fn second(&self) -> bits::U6 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
   }
#[doc="Second"]
   #[inline] pub fn set_second<V: Into<bits::U6>>(mut self, value: V) -> Self {
      let value: bits::U6 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3f << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Minute"]
   #[inline] pub fn minute(&self) -> bits::U6 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3f) as u8) } // [11:6]
   }
#[doc="Minute"]
   #[inline] pub fn set_minute<V: Into<bits::U6>>(mut self, value: V) -> Self {
      let value: bits::U6 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3f << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Hour"]
   #[inline] pub fn hour(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1f) as u8) } // [16:12]
   }
#[doc="Hour"]
   #[inline] pub fn set_hour<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Day"]
   #[inline] pub fn day(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1f) as u8) } // [21:17]
   }
#[doc="Day"]
   #[inline] pub fn set_day<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 17);
      self.0 |= value << 17;
      self
   }

#[doc="Month"]
   #[inline] pub fn month(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0xf) as u8) } // [25:22]
   }
#[doc="Month"]
   #[inline] pub fn set_month<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 22);
      self.0 |= value << 22;
      self
   }

#[doc="Year"]
   #[inline] pub fn year(&self) -> bits::U6 {
      unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x3f) as u8) } // [31:26]
   }
#[doc="Year"]
   #[inline] pub fn set_year<V: Into<bits::U6>>(mut self, value: V) -> Self {
      let value: bits::U6 = value.into();
      let value: u32 = value.into();
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
#[doc="MODE2 Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u16);
impl Ctrl {
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

#[doc="Operating Mode"]
   #[inline] pub fn mode(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
   }
#[doc="Operating Mode"]
   #[inline] pub fn set_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x3 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Clock Representation"]
   #[inline] pub fn clkrep(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Clock Representation"]
   #[inline] pub fn set_clkrep<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Clear on Match"]
   #[inline] pub fn matchclr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Clear on Match"]
   #[inline] pub fn set_matchclr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="Prescaler"]
   #[inline] pub fn prescaler(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
   }
#[doc="Prescaler"]
   #[inline] pub fn set_prescaler<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u16 = value.into();
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
#[doc="MODE2 Event Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u16);
impl Evctrl {
#[doc="Periodic Interval 0 Event Output Enable"]
   #[inline] pub fn pereo0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Periodic Interval 0 Event Output Enable"]
   #[inline] pub fn set_pereo0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Periodic Interval 1 Event Output Enable"]
   #[inline] pub fn pereo1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Periodic Interval 1 Event Output Enable"]
   #[inline] pub fn set_pereo1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Periodic Interval 2 Event Output Enable"]
   #[inline] pub fn pereo2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Periodic Interval 2 Event Output Enable"]
   #[inline] pub fn set_pereo2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Periodic Interval 3 Event Output Enable"]
   #[inline] pub fn pereo3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Periodic Interval 3 Event Output Enable"]
   #[inline] pub fn set_pereo3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Periodic Interval 4 Event Output Enable"]
   #[inline] pub fn pereo4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Periodic Interval 4 Event Output Enable"]
   #[inline] pub fn set_pereo4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Periodic Interval 5 Event Output Enable"]
   #[inline] pub fn pereo5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Periodic Interval 5 Event Output Enable"]
   #[inline] pub fn set_pereo5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Periodic Interval 6 Event Output Enable"]
   #[inline] pub fn pereo6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Periodic Interval 6 Event Output Enable"]
   #[inline] pub fn set_pereo6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Periodic Interval 7 Event Output Enable"]
   #[inline] pub fn pereo7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Periodic Interval 7 Event Output Enable"]
   #[inline] pub fn set_pereo7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="Alarm 0 Event Output Enable"]
   #[inline] pub fn alarmeo0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Alarm 0 Event Output Enable"]
   #[inline] pub fn set_alarmeo0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Overflow Event Output Enable"]
   #[inline] pub fn ovfeo(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Overflow Event Output Enable"]
   #[inline] pub fn set_ovfeo<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
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
#[doc="MODE2 Interrupt Enable Clear"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
#[doc="Alarm 0 Interrupt Enable"]
   #[inline] pub fn alarm0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Alarm 0 Interrupt Enable"]
   #[inline] pub fn set_alarm0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[doc="MODE2 Interrupt Enable Set"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
#[doc="Alarm 0 Interrupt Enable"]
   #[inline] pub fn alarm0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Alarm 0 Interrupt Enable"]
   #[inline] pub fn set_alarm0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[doc="MODE2 Interrupt Flag Status and Clear"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
#[doc="Alarm 0"]
   #[inline] pub fn alarm0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Alarm 0"]
   #[inline] pub fn set_alarm0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Synchronization Ready"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Synchronization Ready"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Overflow"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Overflow"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[doc="MODE2 Alarm n Value"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Alarm(pub u32);
impl Alarm {
#[doc="Second"]
   #[inline] pub fn second(&self) -> bits::U6 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
   }
#[doc="Second"]
   #[inline] pub fn set_second<V: Into<bits::U6>>(mut self, value: V) -> Self {
      let value: bits::U6 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3f << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Minute"]
   #[inline] pub fn minute(&self) -> bits::U6 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3f) as u8) } // [11:6]
   }
#[doc="Minute"]
   #[inline] pub fn set_minute<V: Into<bits::U6>>(mut self, value: V) -> Self {
      let value: bits::U6 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3f << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Hour"]
   #[inline] pub fn hour(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1f) as u8) } // [16:12]
   }
#[doc="Hour"]
   #[inline] pub fn set_hour<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Day"]
   #[inline] pub fn day(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1f) as u8) } // [21:17]
   }
#[doc="Day"]
   #[inline] pub fn set_day<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 17);
      self.0 |= value << 17;
      self
   }

#[doc="Month"]
   #[inline] pub fn month(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0xf) as u8) } // [25:22]
   }
#[doc="Month"]
   #[inline] pub fn set_month<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 22);
      self.0 |= value << 22;
      self
   }

#[doc="Year"]
   #[inline] pub fn year(&self) -> bits::U6 {
      unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x3f) as u8) } // [31:26]
   }
#[doc="Year"]
   #[inline] pub fn set_year<V: Into<bits::U6>>(mut self, value: V) -> Self {
      let value: bits::U6 = value.into();
      let value: u32 = value.into();
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
#[doc="MODE2 Alarm n Mask"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mask(pub u8);
impl Mask {
#[doc="Alarm Mask Selection"]
   #[inline] pub fn sel(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
   }
#[doc="Alarm Mask Selection"]
   #[inline] pub fn set_sel<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u8 = value.into();
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
#[doc="Read Request"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Readreq(pub u16);
impl Readreq {
#[doc="Address"]
   #[inline] pub fn addr(&self) -> bits::U6 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
   }
#[doc="Address"]
   #[inline] pub fn set_addr<V: Into<bits::U6>>(mut self, value: V) -> Self {
      let value: bits::U6 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x3f << 0);
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
      if self.syncbusy() != 0 { try!(write!(f, " syncbusy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
}
// End of mode2

