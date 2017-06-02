pub const NVMCTRL: Nvmctrl = Nvmctrl(0x41004000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Nvmctrl(pub u32);
impl Nvmctrl {
  #[inline]
  pub fn addr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
  #[inline]
  pub fn addr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
  #[inline]
  pub fn addr(&self) -> Addr { 
     unsafe {
       Addr(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
  #[inline]
  pub fn set_addr(&self, value: Addr) -> &Nvmctrl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_addr<F: FnOnce(Addr) -> Addr>(&self, f: F) -> &Nvmctrl {
     let tmp = self.addr();
     self.set_addr(f(tmp))
  }

  #[inline]
  pub fn ctrla_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x0) as *const u16
  }
  #[inline]
  pub fn ctrla_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x0) as *mut u16
  }
  #[inline]
  pub fn ctrla(&self) -> Ctrla { 
     unsafe {
       Ctrla(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u16))
     }
  }
  #[inline]
  pub fn set_ctrla(&self, value: Ctrla) -> &Nvmctrl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u16, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Nvmctrl {
     let tmp = self.ctrla();
     self.set_ctrla(f(tmp))
  }

  #[inline]
  pub fn ctrlb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
  #[inline]
  pub fn ctrlb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
  #[inline]
  pub fn ctrlb(&self) -> Ctrlb { 
     unsafe {
       Ctrlb(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
  #[inline]
  pub fn set_ctrlb(&self, value: Ctrlb) -> &Nvmctrl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ctrlb<F: FnOnce(Ctrlb) -> Ctrlb>(&self, f: F) -> &Nvmctrl {
     let tmp = self.ctrlb();
     self.set_ctrlb(f(tmp))
  }

  #[inline]
  pub fn intenclr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xc) as *const u8
  }
  #[inline]
  pub fn intenclr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xc) as *mut u8
  }
  #[inline]
  pub fn intenclr(&self) -> Intenclr { 
     unsafe {
       Intenclr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u8))
     }
  }
  #[inline]
  pub fn set_intenclr(&self, value: Intenclr) -> &Nvmctrl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Nvmctrl {
     let tmp = self.intenclr();
     self.set_intenclr(f(tmp))
  }

  #[inline]
  pub fn intenset_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x10) as *const u8
  }
  #[inline]
  pub fn intenset_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x10) as *mut u8
  }
  #[inline]
  pub fn intenset(&self) -> Intenset { 
     unsafe {
       Intenset(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u8))
     }
  }
  #[inline]
  pub fn set_intenset(&self, value: Intenset) -> &Nvmctrl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Nvmctrl {
     let tmp = self.intenset();
     self.set_intenset(f(tmp))
  }

  #[inline]
  pub fn intflag_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x14) as *const u8
  }
  #[inline]
  pub fn intflag_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x14) as *mut u8
  }
  #[inline]
  pub fn intflag(&self) -> Intflag { 
     unsafe {
       Intflag(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u8))
     }
  }
  #[inline]
  pub fn set_intflag(&self, value: Intflag) -> &Nvmctrl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Nvmctrl {
     let tmp = self.intflag();
     self.set_intflag(f(tmp))
  }

  #[inline]
  pub fn lock_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x20) as *const u16
  }
  #[inline]
  pub fn lock_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x20) as *mut u16
  }
  #[inline]
  pub fn lock(&self) -> Lock { 
     unsafe {
       Lock(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u16))
     }
  }
  #[inline]
  pub fn set_lock(&self, value: Lock) -> &Nvmctrl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u16, value.0);
     }
     self
  }
  #[inline]
  pub fn with_lock<F: FnOnce(Lock) -> Lock>(&self, f: F) -> &Nvmctrl {
     let tmp = self.lock();
     self.set_lock(f(tmp))
  }

  #[inline]
  pub fn param_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
  #[inline]
  pub fn param_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
  #[inline]
  pub fn param(&self) -> Param { 
     unsafe {
       Param(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
  #[inline]
  pub fn set_param(&self, value: Param) -> &Nvmctrl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_param<F: FnOnce(Param) -> Param>(&self, f: F) -> &Nvmctrl {
     let tmp = self.param();
     self.set_param(f(tmp))
  }

  #[inline]
  pub fn status_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x18) as *const u16
  }
  #[inline]
  pub fn status_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x18) as *mut u16
  }
  #[inline]
  pub fn status(&self) -> Status { 
     unsafe {
       Status(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u16))
     }
  }
  #[inline]
  pub fn set_status(&self, value: Status) -> &Nvmctrl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u16, value.0);
     }
     self
  }
  #[inline]
  pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Nvmctrl {
     let tmp = self.status();
     self.set_status(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Addr(pub u32);
impl Addr {
  #[inline]
  pub fn addr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3fffff // [21:0]
  }
  #[inline]
  pub fn set_addr(mut self, value: u32) -> Self {
     assert!((value & !0x3fffff) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Ctrla(pub u16);
impl Ctrla {
  #[inline]
  pub fn cmd(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x7f // [6:0]
  }
  #[inline]
  pub fn set_cmd(mut self, value: u16) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn cmdex(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0xff // [15:8]
  }
  #[inline]
  pub fn set_cmdex(mut self, value: u16) -> Self {
     assert!((value & !0xff) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Ctrlb(pub u32);
impl Ctrlb {
  #[inline]
  pub fn rws(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0xf // [4:1]
  }
  #[inline]
  pub fn set_rws(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn manw(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_manw(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn sleepprm(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  #[inline]
  pub fn set_sleepprm(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn readmode(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3 // [17:16]
  }
  #[inline]
  pub fn set_readmode(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn cachedis(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline]
  pub fn set_cachedis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
  #[inline]
  pub fn ready(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_ready(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn error(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_error(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
  #[inline]
  pub fn ready(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_ready(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn error(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_error(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
  #[inline]
  pub fn ready(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_ready(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn error(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_error(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Lock(pub u16);
impl Lock {
  #[inline]
  pub fn lock(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
  #[inline]
  pub fn set_lock(mut self, value: u16) -> Self {
     assert!((value & !0xffff) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Param(pub u32);
impl Param {
  #[inline]
  pub fn nvmp(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline]
  pub fn set_nvmp(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn psz(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x7 // [18:16]
  }
  #[inline]
  pub fn set_psz(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Status(pub u16);
impl Status {
  #[inline]
  pub fn prm(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_prm(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn load(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_load(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn proge(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_proge(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn locke(&self) -> u16 {
     ((self.0 as u16) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_locke(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn nvme(&self) -> u16 {
     ((self.0 as u16) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_nvme(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn sb(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_sb(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
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
