//! Non-Volatile Memory Controller
#[allow(unused_imports)] use bobbin_common::bits;
pub const NVMCTRL: Nvmctrl = Nvmctrl(0x41004000);

#[doc="Non-Volatile Memory Controller"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Nvmctrl(pub u32);
impl Nvmctrl {
#[doc="Get the *const pointer for the ADDR register."]
  #[inline] pub fn addr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the ADDR register."]
  #[inline] pub fn addr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Read the ADDR register."]
  #[inline] pub fn addr(&self) -> Addr { 
     unsafe {
        Addr(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
#[doc="Write the ADDR register."]
  #[inline] pub fn set_addr(&self, value: Addr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ADDR register."]
  #[inline] pub fn with_addr<F: FnOnce(Addr) -> Addr>(&self, f: F) -> &Self {
     let tmp = self.addr();
     self.set_addr(f(tmp))
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

#[doc="Get the *const pointer for the CTRLB register."]
  #[inline] pub fn ctrlb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the CTRLB register."]
  #[inline] pub fn ctrlb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the CTRLB register."]
  #[inline] pub fn ctrlb(&self) -> Ctrlb { 
     unsafe {
        Ctrlb(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the CTRLB register."]
  #[inline] pub fn set_ctrlb(&self, value: Ctrlb) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CTRLB register."]
  #[inline] pub fn with_ctrlb<F: FnOnce(Ctrlb) -> Ctrlb>(&self, f: F) -> &Self {
     let tmp = self.ctrlb();
     self.set_ctrlb(f(tmp))
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
     ((self.0 as usize) + 0x10) as *const u8
  }
#[doc="Get the *mut pointer for the INTENSET register."]
  #[inline] pub fn intenset_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x10) as *mut u8
  }
#[doc="Read the INTENSET register."]
  #[inline] pub fn intenset(&self) -> Intenset { 
     unsafe {
        Intenset(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u8))
     }
  }
#[doc="Write the INTENSET register."]
  #[inline] pub fn set_intenset(&self, value: Intenset) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u8, value.0);
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
     ((self.0 as usize) + 0x14) as *const u8
  }
#[doc="Get the *mut pointer for the INTFLAG register."]
  #[inline] pub fn intflag_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x14) as *mut u8
  }
#[doc="Read the INTFLAG register."]
  #[inline] pub fn intflag(&self) -> Intflag { 
     unsafe {
        Intflag(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u8))
     }
  }
#[doc="Write the INTFLAG register."]
  #[inline] pub fn set_intflag(&self, value: Intflag) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTFLAG register."]
  #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
     let tmp = self.intflag();
     self.set_intflag(f(tmp))
  }

#[doc="Get the *const pointer for the LOCK register."]
  #[inline] pub fn lock_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x20) as *const u16
  }
#[doc="Get the *mut pointer for the LOCK register."]
  #[inline] pub fn lock_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x20) as *mut u16
  }
#[doc="Read the LOCK register."]
  #[inline] pub fn lock(&self) -> Lock { 
     unsafe {
        Lock(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u16))
     }
  }
#[doc="Write the LOCK register."]
  #[inline] pub fn set_lock(&self, value: Lock) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the LOCK register."]
  #[inline] pub fn with_lock<F: FnOnce(Lock) -> Lock>(&self, f: F) -> &Self {
     let tmp = self.lock();
     self.set_lock(f(tmp))
  }

#[doc="Get the *const pointer for the PARAM register."]
  #[inline] pub fn param_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the PARAM register."]
  #[inline] pub fn param_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the PARAM register."]
  #[inline] pub fn param(&self) -> Param { 
     unsafe {
        Param(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the PARAM register."]
  #[inline] pub fn set_param(&self, value: Param) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PARAM register."]
  #[inline] pub fn with_param<F: FnOnce(Param) -> Param>(&self, f: F) -> &Self {
     let tmp = self.param();
     self.set_param(f(tmp))
  }

#[doc="Get the *const pointer for the STATUS register."]
  #[inline] pub fn status_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x18) as *const u16
  }
#[doc="Get the *mut pointer for the STATUS register."]
  #[inline] pub fn status_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x18) as *mut u16
  }
#[doc="Read the STATUS register."]
  #[inline] pub fn status(&self) -> Status { 
     unsafe {
        Status(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u16))
     }
  }
#[doc="Write the STATUS register."]
  #[inline] pub fn set_status(&self, value: Status) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the STATUS register."]
  #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
     let tmp = self.status();
     self.set_status(f(tmp))
  }

}

#[doc="Address"]
#[derive(PartialEq, Eq)]
pub struct Addr(pub u32);
impl Addr {
#[doc="NVM Address"]
  #[inline] pub fn addr(&self) -> bits::U22 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3fffff) as u32) } // [21:0]
  }
#[doc="NVM Address"]
  #[inline] pub fn set_addr<V: Into<bits::U22>>(mut self, value: V) -> Self {
     let value: bits::U22 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3fffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Addr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Addr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control A"]
#[derive(PartialEq, Eq)]
pub struct Ctrla(pub u16);
impl Ctrla {
#[doc="Command"]
  #[inline] pub fn cmd(&self) -> bits::U7 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
  }
#[doc="Command"]
  #[inline] pub fn set_cmd<V: Into<bits::U7>>(mut self, value: V) -> Self {
     let value: bits::U7 = value.into();
     let value: u16 = value.into();
     self.0 &= !(0x7f << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Command Execution"]
  #[inline] pub fn cmdex(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
  }
#[doc="Command Execution"]
  #[inline] pub fn set_cmdex<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u16 = value.into();
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
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
      if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
      if self.cmdex() != 0 { try!(write!(f, " cmdex=0x{:x}", self.cmdex()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control B"]
#[derive(PartialEq, Eq)]
pub struct Ctrlb(pub u32);
impl Ctrlb {
#[doc="NVM Read Wait States"]
  #[inline] pub fn rws(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0xf) as u8) } // [4:1]
  }
#[doc="NVM Read Wait States"]
  #[inline] pub fn set_rws<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Manual Write"]
  #[inline] pub fn manw(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="Manual Write"]
  #[inline] pub fn set_manw<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Power Reduction Mode during Sleep"]
  #[inline] pub fn sleepprm(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
  }
#[doc="Power Reduction Mode during Sleep"]
  #[inline] pub fn set_sleepprm<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="NVMCTRL Read Mode"]
  #[inline] pub fn readmode(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
  }
#[doc="NVMCTRL Read Mode"]
  #[inline] pub fn set_readmode<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Cache Disable"]
  #[inline] pub fn cachedis(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
  }
#[doc="Cache Disable"]
  #[inline] pub fn set_cachedis<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

}
impl ::core::fmt::Display for Ctrlb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrlb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rws() != 0 { try!(write!(f, " rws=0x{:x}", self.rws()))}
      if self.manw() != 0 { try!(write!(f, " manw"))}
      if self.sleepprm() != 0 { try!(write!(f, " sleepprm=0x{:x}", self.sleepprm()))}
      if self.readmode() != 0 { try!(write!(f, " readmode=0x{:x}", self.readmode()))}
      if self.cachedis() != 0 { try!(write!(f, " cachedis"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Enable Clear"]
#[derive(PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
#[doc="NVM Ready Interrupt Enable"]
  #[inline] pub fn ready(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="NVM Ready Interrupt Enable"]
  #[inline] pub fn set_ready<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Error Interrupt Enable"]
  #[inline] pub fn error(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Error Interrupt Enable"]
  #[inline] pub fn set_error<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
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
      if self.ready() != 0 { try!(write!(f, " ready"))}
      if self.error() != 0 { try!(write!(f, " error"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Enable Set"]
#[derive(PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
#[doc="NVM Ready Interrupt Enable"]
  #[inline] pub fn ready(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="NVM Ready Interrupt Enable"]
  #[inline] pub fn set_ready<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Error Interrupt Enable"]
  #[inline] pub fn error(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Error Interrupt Enable"]
  #[inline] pub fn set_error<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
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
      if self.ready() != 0 { try!(write!(f, " ready"))}
      if self.error() != 0 { try!(write!(f, " error"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Flag Status and Clear"]
#[derive(PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
#[doc="NVM Ready"]
  #[inline] pub fn ready(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="NVM Ready"]
  #[inline] pub fn set_ready<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Error"]
  #[inline] pub fn error(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Error"]
  #[inline] pub fn set_error<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
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
      if self.ready() != 0 { try!(write!(f, " ready"))}
      if self.error() != 0 { try!(write!(f, " error"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Lock Section"]
#[derive(PartialEq, Eq)]
pub struct Lock(pub u16);
impl Lock {
#[doc="Region Lock Bits"]
  #[inline] pub fn lock(&self) -> bits::U16 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
  }
#[doc="Region Lock Bits"]
  #[inline] pub fn set_lock<V: Into<bits::U16>>(mut self, value: V) -> Self {
     let value: bits::U16 = value.into();
     let value: u16 = value.into();
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Lock {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Lock {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lock() != 0 { try!(write!(f, " lock=0x{:x}", self.lock()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="NVM Parameter"]
#[derive(PartialEq, Eq)]
pub struct Param(pub u32);
impl Param {
#[doc="NVM Pages"]
  #[inline] pub fn nvmp(&self) -> bits::U16 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
  }
#[doc="NVM Pages"]
  #[inline] pub fn set_nvmp<V: Into<bits::U16>>(mut self, value: V) -> Self {
     let value: bits::U16 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Page Size"]
  #[inline] pub fn psz(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
  }
#[doc="Page Size"]
  #[inline] pub fn set_psz<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7 << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Param {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Param {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.nvmp() != 0 { try!(write!(f, " nvmp=0x{:x}", self.nvmp()))}
      if self.psz() != 0 { try!(write!(f, " psz=0x{:x}", self.psz()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Status"]
#[derive(PartialEq, Eq)]
pub struct Status(pub u16);
impl Status {
#[doc="Power Reduction Mode"]
  #[inline] pub fn prm(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Power Reduction Mode"]
  #[inline] pub fn set_prm<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u16 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="NVM Page Buffer Active Loading"]
  #[inline] pub fn load(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="NVM Page Buffer Active Loading"]
  #[inline] pub fn set_load<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u16 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Programming Error Status"]
  #[inline] pub fn proge(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="Programming Error Status"]
  #[inline] pub fn set_proge<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u16 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Lock Error Status"]
  #[inline] pub fn locke(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="Lock Error Status"]
  #[inline] pub fn set_locke<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u16 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="NVM Error"]
  #[inline] pub fn nvme(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="NVM Error"]
  #[inline] pub fn set_nvme<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u16 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Security Bit Status"]
  #[inline] pub fn sb(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
  }
#[doc="Security Bit Status"]
  #[inline] pub fn set_sb<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u16 = value.into();
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
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
      if self.prm() != 0 { try!(write!(f, " prm"))}
      if self.load() != 0 { try!(write!(f, " load"))}
      if self.proge() != 0 { try!(write!(f, " proge"))}
      if self.locke() != 0 { try!(write!(f, " locke"))}
      if self.nvme() != 0 { try!(write!(f, " nvme"))}
      if self.sb() != 0 { try!(write!(f, " sb"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

