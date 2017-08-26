//! Watchdog Timer
#[allow(unused_imports)] use bobbin_common::*;

periph!(WdtPeriph, WDT, Wdt, 0x40001000);

#[doc="Watchdog Timer"]
pub trait WdtPeriph : Base {
#[doc="Get the *const pointer for the CLEAR register."]
   #[inline] fn clear_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Get the *mut pointer for the CLEAR register."]
   #[inline] fn clear_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Write the CLEAR register."]
   #[inline] fn set_clear<F: FnOnce(Clear) -> Clear>(&self, f: F) -> &Self {
      let value = f(Clear(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x8) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CONFIG register."]
   #[inline] fn config_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0x1)
   }
#[doc="Get the *mut pointer for the CONFIG register."]
   #[inline] fn config_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0x1)
   }
#[doc="Read the CONFIG register."]
   #[inline] fn config(&self) -> Config { 
      unsafe {
         Config(::core::ptr::read_volatile((self.base() + 0x1) as *const u8))
      }
   }
#[doc="Write the CONFIG register."]
   #[inline] fn set_config<F: FnOnce(Config) -> Config>(&self, f: F) -> &Self {
      let value = f(Config(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x1) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the CONFIG register."]
   #[inline] fn with_config<F: FnOnce(Config) -> Config>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Config(::core::ptr::read_volatile((self.base() + 0x1) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x1) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRL register."]
   #[inline] fn ctrl_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Get the *mut pointer for the CTRL register."]
   #[inline] fn ctrl_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Read the CTRL register."]
   #[inline] fn ctrl(&self) -> Ctrl { 
      unsafe {
         Ctrl(::core::ptr::read_volatile((self.base() + 0x0) as *const u8))
      }
   }
#[doc="Write the CTRL register."]
   #[inline] fn set_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
      let value = f(Ctrl(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x0) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the CTRL register."]
   #[inline] fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ctrl(::core::ptr::read_volatile((self.base() + 0x0) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x0) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the EWCTRL register."]
   #[inline] fn ewctrl_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0x2)
   }
#[doc="Get the *mut pointer for the EWCTRL register."]
   #[inline] fn ewctrl_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0x2)
   }
#[doc="Read the EWCTRL register."]
   #[inline] fn ewctrl(&self) -> Ewctrl { 
      unsafe {
         Ewctrl(::core::ptr::read_volatile((self.base() + 0x2) as *const u8))
      }
   }
#[doc="Write the EWCTRL register."]
   #[inline] fn set_ewctrl<F: FnOnce(Ewctrl) -> Ewctrl>(&self, f: F) -> &Self {
      let value = f(Ewctrl(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x2) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the EWCTRL register."]
   #[inline] fn with_ewctrl<F: FnOnce(Ewctrl) -> Ewctrl>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ewctrl(::core::ptr::read_volatile((self.base() + 0x2) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x2) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTENCLR register."]
   #[inline] fn intenclr_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Get the *mut pointer for the INTENCLR register."]
   #[inline] fn intenclr_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Read the INTENCLR register."]
   #[inline] fn intenclr(&self) -> Intenclr { 
      unsafe {
         Intenclr(::core::ptr::read_volatile((self.base() + 0x4) as *const u8))
      }
   }
#[doc="Write the INTENCLR register."]
   #[inline] fn set_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let value = f(Intenclr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTENCLR register."]
   #[inline] fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Intenclr(::core::ptr::read_volatile((self.base() + 0x4) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTENSET register."]
   #[inline] fn intenset_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0x5)
   }
#[doc="Get the *mut pointer for the INTENSET register."]
   #[inline] fn intenset_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0x5)
   }
#[doc="Read the INTENSET register."]
   #[inline] fn intenset(&self) -> Intenset { 
      unsafe {
         Intenset(::core::ptr::read_volatile((self.base() + 0x5) as *const u8))
      }
   }
#[doc="Write the INTENSET register."]
   #[inline] fn set_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let value = f(Intenset(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x5) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTENSET register."]
   #[inline] fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Intenset(::core::ptr::read_volatile((self.base() + 0x5) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x5) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTFLAG register."]
   #[inline] fn intflag_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0x6)
   }
#[doc="Get the *mut pointer for the INTFLAG register."]
   #[inline] fn intflag_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0x6)
   }
#[doc="Read the INTFLAG register."]
   #[inline] fn intflag(&self) -> Intflag { 
      unsafe {
         Intflag(::core::ptr::read_volatile((self.base() + 0x6) as *const u8))
      }
   }
#[doc="Write the INTFLAG register."]
   #[inline] fn set_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let value = f(Intflag(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x6) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTFLAG register."]
   #[inline] fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Intflag(::core::ptr::read_volatile((self.base() + 0x6) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x6) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the STATUS register."]
   #[inline] fn status_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0x7)
   }
#[doc="Get the *mut pointer for the STATUS register."]
   #[inline] fn status_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0x7)
   }
#[doc="Read the STATUS register."]
   #[inline] fn status(&self) -> Status { 
      unsafe {
         Status(::core::ptr::read_volatile((self.base() + 0x7) as *const u8))
      }
   }

}

#[doc="Clear"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clear(pub u8);
impl Clear {
#[doc="Watchdog Clear"]
   #[inline] pub fn clear(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Watchdog Clear"]
   #[inline] pub fn set_clear<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Clear {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clear {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clear() != 0 { try!(write!(f, " clear=0x{:x}", self.clear()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Configuration"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Config(pub u8);
impl Config {
#[doc="Time-Out Period"]
   #[inline] pub fn per(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="Time-Out Period"]
   #[inline] pub fn set_per<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Window Mode Time-Out Period"]
   #[inline] pub fn window(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
   }
#[doc="Window Mode Time-Out Period"]
   #[inline] pub fn set_window<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xf << 4);
      self.0 |= value << 4;
      self
   }

}
impl ::core::fmt::Display for Config {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Config {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.per() != 0 { try!(write!(f, " per=0x{:x}", self.per()))}
      if self.window() != 0 { try!(write!(f, " window=0x{:x}", self.window()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u8);
impl Ctrl {
#[doc="Enable"]
   #[inline] pub fn enable(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Enable"]
   #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Watchdog Timer Window Mode Enable"]
   #[inline] pub fn wen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Watchdog Timer Window Mode Enable"]
   #[inline] pub fn set_wen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Always-On"]
   #[inline] pub fn alwayson(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Always-On"]
   #[inline] pub fn set_alwayson<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
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
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.wen() != 0 { try!(write!(f, " wen"))}
      if self.alwayson() != 0 { try!(write!(f, " alwayson"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Early Warning Interrupt Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ewctrl(pub u8);
impl Ewctrl {
#[doc="Early Warning Interrupt Time Offset"]
   #[inline] pub fn ewoffset(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="Early Warning Interrupt Time Offset"]
   #[inline] pub fn set_ewoffset<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Ewctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ewctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ewoffset() != 0 { try!(write!(f, " ewoffset=0x{:x}", self.ewoffset()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Enable Clear"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
#[doc="Early Warning Interrupt Enable"]
   #[inline] pub fn ew(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Early Warning Interrupt Enable"]
   #[inline] pub fn set_ew<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
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
      if self.ew() != 0 { try!(write!(f, " ew"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Enable Set"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
#[doc="Early Warning Interrupt Enable"]
   #[inline] pub fn ew(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Early Warning Interrupt Enable"]
   #[inline] pub fn set_ew<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
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
      if self.ew() != 0 { try!(write!(f, " ew"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Flag Status and Clear"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
#[doc="Early Warning"]
   #[inline] pub fn ew(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Early Warning"]
   #[inline] pub fn set_ew<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
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
      if self.ew() != 0 { try!(write!(f, " ew"))}
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

