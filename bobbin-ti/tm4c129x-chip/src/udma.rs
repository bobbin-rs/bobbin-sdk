pub const UDMA: Udma = Periph(0x400ff000, UdmaId {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Periph<T>(pub u32, pub T); 

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct UdmaId {}
pub type Udma = Periph<UdmaId>;



impl<T> Periph<T> {
  #[inline] pub fn stat_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
  #[inline] pub fn stat_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
  #[inline] pub fn stat(&self) -> Stat { 
     unsafe {
        Stat(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  #[inline] pub fn set_stat(&self, value: Stat) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_stat<F: FnOnce(Stat) -> Stat>(&self, f: F) -> &Self {
     let tmp = self.stat();
     self.set_stat(f(tmp))
  }

  #[inline] pub fn cfg_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
  #[inline] pub fn cfg_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
  #[inline] pub fn set_cfg(&self, value: Cfg) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }

  #[inline] pub fn ctlbase_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
  #[inline] pub fn ctlbase_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
  #[inline] pub fn ctlbase(&self) -> Ctlbase { 
     unsafe {
        Ctlbase(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
  #[inline] pub fn set_ctlbase(&self, value: Ctlbase) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ctlbase<F: FnOnce(Ctlbase) -> Ctlbase>(&self, f: F) -> &Self {
     let tmp = self.ctlbase();
     self.set_ctlbase(f(tmp))
  }

  #[inline] pub fn altbase_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
  #[inline] pub fn altbase_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
  #[inline] pub fn altbase(&self) -> Altbase { 
     unsafe {
        Altbase(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
  #[inline] pub fn set_altbase(&self, value: Altbase) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_altbase<F: FnOnce(Altbase) -> Altbase>(&self, f: F) -> &Self {
     let tmp = self.altbase();
     self.set_altbase(f(tmp))
  }

  #[inline] pub fn waitstat_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
  #[inline] pub fn waitstat_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
  #[inline] pub fn waitstat(&self) -> Waitstat { 
     unsafe {
        Waitstat(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
  #[inline] pub fn set_waitstat(&self, value: Waitstat) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_waitstat<F: FnOnce(Waitstat) -> Waitstat>(&self, f: F) -> &Self {
     let tmp = self.waitstat();
     self.set_waitstat(f(tmp))
  }

  #[inline] pub fn swreq_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
  #[inline] pub fn swreq_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
  #[inline] pub fn set_swreq(&self, value: Swreq) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }

  #[inline] pub fn useburstset_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
  #[inline] pub fn useburstset_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
  #[inline] pub fn useburstset(&self) -> Useburstset { 
     unsafe {
        Useburstset(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
  #[inline] pub fn set_useburstset(&self, value: Useburstset) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_useburstset<F: FnOnce(Useburstset) -> Useburstset>(&self, f: F) -> &Self {
     let tmp = self.useburstset();
     self.set_useburstset(f(tmp))
  }

  #[inline] pub fn useburstclr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
  #[inline] pub fn useburstclr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
  #[inline] pub fn set_useburstclr(&self, value: Useburstclr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }

  #[inline] pub fn reqmaskset_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
  #[inline] pub fn reqmaskset_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
  #[inline] pub fn reqmaskset(&self) -> Reqmaskset { 
     unsafe {
        Reqmaskset(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
  #[inline] pub fn set_reqmaskset(&self, value: Reqmaskset) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_reqmaskset<F: FnOnce(Reqmaskset) -> Reqmaskset>(&self, f: F) -> &Self {
     let tmp = self.reqmaskset();
     self.set_reqmaskset(f(tmp))
  }

  #[inline] pub fn reqmaskclr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
  #[inline] pub fn reqmaskclr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
  #[inline] pub fn set_reqmaskclr(&self, value: Reqmaskclr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }

  #[inline] pub fn enaset_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x28) as *const u32
  }
  #[inline] pub fn enaset_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x28) as *mut u32
  }
  #[inline] pub fn enaset(&self) -> Enaset { 
     unsafe {
        Enaset(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
     }
  }
  #[inline] pub fn set_enaset(&self, value: Enaset) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_enaset<F: FnOnce(Enaset) -> Enaset>(&self, f: F) -> &Self {
     let tmp = self.enaset();
     self.set_enaset(f(tmp))
  }

  #[inline] pub fn enaclr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2c) as *const u32
  }
  #[inline] pub fn enaclr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2c) as *mut u32
  }
  #[inline] pub fn set_enaclr(&self, value: Enaclr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2c) as *mut u32, value.0);
     }
     self
  }

  #[inline] pub fn altset_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x30) as *const u32
  }
  #[inline] pub fn altset_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x30) as *mut u32
  }
  #[inline] pub fn altset(&self) -> Altset { 
     unsafe {
        Altset(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u32))
     }
  }
  #[inline] pub fn set_altset(&self, value: Altset) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_altset<F: FnOnce(Altset) -> Altset>(&self, f: F) -> &Self {
     let tmp = self.altset();
     self.set_altset(f(tmp))
  }

  #[inline] pub fn altclr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x34) as *const u32
  }
  #[inline] pub fn altclr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x34) as *mut u32
  }
  #[inline] pub fn set_altclr(&self, value: Altclr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x34) as *mut u32, value.0);
     }
     self
  }

  #[inline] pub fn prioset_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x38) as *const u32
  }
  #[inline] pub fn prioset_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x38) as *mut u32
  }
  #[inline] pub fn prioset(&self) -> Prioset { 
     unsafe {
        Prioset(::core::ptr::read_volatile(((self.0 as usize) + 0x38) as *const u32))
     }
  }
  #[inline] pub fn set_prioset(&self, value: Prioset) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x38) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_prioset<F: FnOnce(Prioset) -> Prioset>(&self, f: F) -> &Self {
     let tmp = self.prioset();
     self.set_prioset(f(tmp))
  }

  #[inline] pub fn prioclr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x3c) as *const u32
  }
  #[inline] pub fn prioclr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x3c) as *mut u32
  }
  #[inline] pub fn set_prioclr(&self, value: Prioclr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x3c) as *mut u32, value.0);
     }
     self
  }

  #[inline] pub fn errclr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4c) as *const u32
  }
  #[inline] pub fn errclr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4c) as *mut u32
  }
  #[inline] pub fn errclr(&self) -> Errclr { 
     unsafe {
        Errclr(::core::ptr::read_volatile(((self.0 as usize) + 0x4c) as *const u32))
     }
  }
  #[inline] pub fn set_errclr(&self, value: Errclr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_errclr<F: FnOnce(Errclr) -> Errclr>(&self, f: F) -> &Self {
     let tmp = self.errclr();
     self.set_errclr(f(tmp))
  }

  #[inline] pub fn chasgn_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x500) as *const u32
  }
  #[inline] pub fn chasgn_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x500) as *mut u32
  }
  #[inline] pub fn chasgn(&self) -> Chasgn { 
     unsafe {
        Chasgn(::core::ptr::read_volatile(((self.0 as usize) + 0x500) as *const u32))
     }
  }
  #[inline] pub fn set_chasgn(&self, value: Chasgn) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x500) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_chasgn<F: FnOnce(Chasgn) -> Chasgn>(&self, f: F) -> &Self {
     let tmp = self.chasgn();
     self.set_chasgn(f(tmp))
  }

  #[inline] pub fn chmap0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x510) as *const u32
  }
  #[inline] pub fn chmap0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x510) as *mut u32
  }
  #[inline] pub fn chmap0(&self) -> Chmap0 { 
     unsafe {
        Chmap0(::core::ptr::read_volatile(((self.0 as usize) + 0x510) as *const u32))
     }
  }
  #[inline] pub fn set_chmap0(&self, value: Chmap0) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x510) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_chmap0<F: FnOnce(Chmap0) -> Chmap0>(&self, f: F) -> &Self {
     let tmp = self.chmap0();
     self.set_chmap0(f(tmp))
  }

  #[inline] pub fn chmap1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x514) as *const u32
  }
  #[inline] pub fn chmap1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x514) as *mut u32
  }
  #[inline] pub fn chmap1(&self) -> Chmap1 { 
     unsafe {
        Chmap1(::core::ptr::read_volatile(((self.0 as usize) + 0x514) as *const u32))
     }
  }
  #[inline] pub fn set_chmap1(&self, value: Chmap1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x514) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_chmap1<F: FnOnce(Chmap1) -> Chmap1>(&self, f: F) -> &Self {
     let tmp = self.chmap1();
     self.set_chmap1(f(tmp))
  }

  #[inline] pub fn chmap2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x518) as *const u32
  }
  #[inline] pub fn chmap2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x518) as *mut u32
  }
  #[inline] pub fn chmap2(&self) -> Chmap2 { 
     unsafe {
        Chmap2(::core::ptr::read_volatile(((self.0 as usize) + 0x518) as *const u32))
     }
  }
  #[inline] pub fn set_chmap2(&self, value: Chmap2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x518) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_chmap2<F: FnOnce(Chmap2) -> Chmap2>(&self, f: F) -> &Self {
     let tmp = self.chmap2();
     self.set_chmap2(f(tmp))
  }

  #[inline] pub fn chmap3_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x51c) as *const u32
  }
  #[inline] pub fn chmap3_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x51c) as *mut u32
  }
  #[inline] pub fn chmap3(&self) -> Chmap3 { 
     unsafe {
        Chmap3(::core::ptr::read_volatile(((self.0 as usize) + 0x51c) as *const u32))
     }
  }
  #[inline] pub fn set_chmap3(&self, value: Chmap3) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x51c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_chmap3<F: FnOnce(Chmap3) -> Chmap3>(&self, f: F) -> &Self {
     let tmp = self.chmap3();
     self.set_chmap3(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Stat(pub u32);
impl Stat {
  #[inline] pub fn udma_stat_masten(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_udma_stat_masten(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn udma_stat_state(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
  #[inline] pub fn set_udma_stat_state(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn udma_stat_dmachans(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1f // [20:16]
  }
  #[inline] pub fn set_udma_stat_dmachans(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Stat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Stat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.udma_stat_masten() != 0 { try!(write!(f, " udma_stat_masten"))}
      if self.udma_stat_state() != 0 { try!(write!(f, " udma_stat_state=0x{:x}", self.udma_stat_state()))}
      if self.udma_stat_dmachans() != 0 { try!(write!(f, " udma_stat_dmachans=0x{:x}", self.udma_stat_dmachans()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cfg(pub u32);
impl Cfg {
  #[inline] pub fn udma_cfg_masten(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_udma_cfg_masten(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
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
      if self.udma_cfg_masten() != 0 { try!(write!(f, " udma_cfg_masten"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ctlbase(pub u32);
impl Ctlbase {
  #[inline] pub fn udma_ctlbase_addr(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3fffff // [31:10]
  }
  #[inline] pub fn set_udma_ctlbase_addr(mut self, value: u32) -> Self {
     assert!((value & !0x3fffff) == 0);
     self.0 &= !(0x3fffff << 10);
     self.0 |= value << 10;
     self
  }

}
impl ::core::fmt::Display for Ctlbase {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctlbase {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.udma_ctlbase_addr() != 0 { try!(write!(f, " udma_ctlbase_addr=0x{:x}", self.udma_ctlbase_addr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Altbase(pub u32);
impl Altbase {
  #[inline] pub fn udma_altbase_addr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_udma_altbase_addr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Altbase {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Altbase {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Waitstat(pub u32);
impl Waitstat {
  #[inline] pub fn udma_waitstat_waitreq(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_udma_waitstat_waitreq(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Waitstat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Waitstat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Swreq(pub u32);
impl Swreq {
  #[inline] pub fn udma_swreq(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_udma_swreq(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Swreq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Swreq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Useburstset(pub u32);
impl Useburstset {
  #[inline] pub fn udma_useburstset_set(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_udma_useburstset_set(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Useburstset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Useburstset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Useburstclr(pub u32);
impl Useburstclr {
  #[inline] pub fn udma_useburstclr_clr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_udma_useburstclr_clr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Useburstclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Useburstclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Reqmaskset(pub u32);
impl Reqmaskset {
  #[inline] pub fn udma_reqmaskset_set(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_udma_reqmaskset_set(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Reqmaskset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Reqmaskset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Reqmaskclr(pub u32);
impl Reqmaskclr {
  #[inline] pub fn udma_reqmaskclr_clr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_udma_reqmaskclr_clr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Reqmaskclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Reqmaskclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Enaset(pub u32);
impl Enaset {
  #[inline] pub fn udma_enaset_set(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_udma_enaset_set(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Enaset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Enaset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Enaclr(pub u32);
impl Enaclr {
  #[inline] pub fn udma_enaclr_clr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_udma_enaclr_clr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Enaclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Enaclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Altset(pub u32);
impl Altset {
  #[inline] pub fn udma_altset_set(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_udma_altset_set(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Altset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Altset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Altclr(pub u32);
impl Altclr {
  #[inline] pub fn udma_altclr_clr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_udma_altclr_clr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Altclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Altclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Prioset(pub u32);
impl Prioset {
  #[inline] pub fn udma_prioset_set(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_udma_prioset_set(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Prioset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Prioset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Prioclr(pub u32);
impl Prioclr {
  #[inline] pub fn udma_prioclr_clr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_udma_prioclr_clr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Prioclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Prioclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Errclr(pub u32);
impl Errclr {
  #[inline] pub fn udma_errclr_errclr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_udma_errclr_errclr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Errclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Errclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.udma_errclr_errclr() != 0 { try!(write!(f, " udma_errclr_errclr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Chasgn(pub u32);
impl Chasgn {
  #[inline] pub fn udma_chasgn(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_udma_chasgn(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Chasgn {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chasgn {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Chmap0(pub u32);
impl Chmap0 {
  #[inline] pub fn udma_chmap0_ch0sel(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_udma_chmap0_ch0sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn udma_chmap0_ch1sel(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
  #[inline] pub fn set_udma_chmap0_ch1sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn udma_chmap0_ch2sel(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  #[inline] pub fn set_udma_chmap0_ch2sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn udma_chmap0_ch3sel(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0xf // [15:12]
  }
  #[inline] pub fn set_udma_chmap0_ch3sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn udma_chmap0_ch4sel(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  #[inline] pub fn set_udma_chmap0_ch4sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  #[inline] pub fn udma_chmap0_ch5sel(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0xf // [23:20]
  }
  #[inline] pub fn set_udma_chmap0_ch5sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

  #[inline] pub fn udma_chmap0_ch6sel(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  #[inline] pub fn set_udma_chmap0_ch6sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

  #[inline] pub fn udma_chmap0_ch7sel(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0xf // [31:28]
  }
  #[inline] pub fn set_udma_chmap0_ch7sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 28);
     self.0 |= value << 28;
     self
  }

}
impl ::core::fmt::Display for Chmap0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chmap0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.udma_chmap0_ch0sel() != 0 { try!(write!(f, " udma_chmap0_ch0sel=0x{:x}", self.udma_chmap0_ch0sel()))}
      if self.udma_chmap0_ch1sel() != 0 { try!(write!(f, " udma_chmap0_ch1sel=0x{:x}", self.udma_chmap0_ch1sel()))}
      if self.udma_chmap0_ch2sel() != 0 { try!(write!(f, " udma_chmap0_ch2sel=0x{:x}", self.udma_chmap0_ch2sel()))}
      if self.udma_chmap0_ch3sel() != 0 { try!(write!(f, " udma_chmap0_ch3sel=0x{:x}", self.udma_chmap0_ch3sel()))}
      if self.udma_chmap0_ch4sel() != 0 { try!(write!(f, " udma_chmap0_ch4sel=0x{:x}", self.udma_chmap0_ch4sel()))}
      if self.udma_chmap0_ch5sel() != 0 { try!(write!(f, " udma_chmap0_ch5sel=0x{:x}", self.udma_chmap0_ch5sel()))}
      if self.udma_chmap0_ch6sel() != 0 { try!(write!(f, " udma_chmap0_ch6sel=0x{:x}", self.udma_chmap0_ch6sel()))}
      if self.udma_chmap0_ch7sel() != 0 { try!(write!(f, " udma_chmap0_ch7sel=0x{:x}", self.udma_chmap0_ch7sel()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Chmap1(pub u32);
impl Chmap1 {
  #[inline] pub fn udma_chmap1_ch8sel(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_udma_chmap1_ch8sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn udma_chmap1_ch9sel(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
  #[inline] pub fn set_udma_chmap1_ch9sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn udma_chmap1_ch10sel(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  #[inline] pub fn set_udma_chmap1_ch10sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn udma_chmap1_ch11sel(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0xf // [15:12]
  }
  #[inline] pub fn set_udma_chmap1_ch11sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn udma_chmap1_ch12sel(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  #[inline] pub fn set_udma_chmap1_ch12sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  #[inline] pub fn udma_chmap1_ch13sel(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0xf // [23:20]
  }
  #[inline] pub fn set_udma_chmap1_ch13sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

  #[inline] pub fn udma_chmap1_ch14sel(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  #[inline] pub fn set_udma_chmap1_ch14sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

  #[inline] pub fn udma_chmap1_ch15sel(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0xf // [31:28]
  }
  #[inline] pub fn set_udma_chmap1_ch15sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 28);
     self.0 |= value << 28;
     self
  }

}
impl ::core::fmt::Display for Chmap1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chmap1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.udma_chmap1_ch8sel() != 0 { try!(write!(f, " udma_chmap1_ch8sel=0x{:x}", self.udma_chmap1_ch8sel()))}
      if self.udma_chmap1_ch9sel() != 0 { try!(write!(f, " udma_chmap1_ch9sel=0x{:x}", self.udma_chmap1_ch9sel()))}
      if self.udma_chmap1_ch10sel() != 0 { try!(write!(f, " udma_chmap1_ch10sel=0x{:x}", self.udma_chmap1_ch10sel()))}
      if self.udma_chmap1_ch11sel() != 0 { try!(write!(f, " udma_chmap1_ch11sel=0x{:x}", self.udma_chmap1_ch11sel()))}
      if self.udma_chmap1_ch12sel() != 0 { try!(write!(f, " udma_chmap1_ch12sel=0x{:x}", self.udma_chmap1_ch12sel()))}
      if self.udma_chmap1_ch13sel() != 0 { try!(write!(f, " udma_chmap1_ch13sel=0x{:x}", self.udma_chmap1_ch13sel()))}
      if self.udma_chmap1_ch14sel() != 0 { try!(write!(f, " udma_chmap1_ch14sel=0x{:x}", self.udma_chmap1_ch14sel()))}
      if self.udma_chmap1_ch15sel() != 0 { try!(write!(f, " udma_chmap1_ch15sel=0x{:x}", self.udma_chmap1_ch15sel()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Chmap2(pub u32);
impl Chmap2 {
  #[inline] pub fn udma_chmap2_ch16sel(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_udma_chmap2_ch16sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn udma_chmap2_ch17sel(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
  #[inline] pub fn set_udma_chmap2_ch17sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn udma_chmap2_ch18sel(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  #[inline] pub fn set_udma_chmap2_ch18sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn udma_chmap2_ch19sel(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0xf // [15:12]
  }
  #[inline] pub fn set_udma_chmap2_ch19sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn udma_chmap2_ch20sel(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  #[inline] pub fn set_udma_chmap2_ch20sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  #[inline] pub fn udma_chmap2_ch21sel(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0xf // [23:20]
  }
  #[inline] pub fn set_udma_chmap2_ch21sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

  #[inline] pub fn udma_chmap2_ch22sel(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  #[inline] pub fn set_udma_chmap2_ch22sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

  #[inline] pub fn udma_chmap2_ch23sel(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0xf // [31:28]
  }
  #[inline] pub fn set_udma_chmap2_ch23sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 28);
     self.0 |= value << 28;
     self
  }

}
impl ::core::fmt::Display for Chmap2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chmap2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.udma_chmap2_ch16sel() != 0 { try!(write!(f, " udma_chmap2_ch16sel=0x{:x}", self.udma_chmap2_ch16sel()))}
      if self.udma_chmap2_ch17sel() != 0 { try!(write!(f, " udma_chmap2_ch17sel=0x{:x}", self.udma_chmap2_ch17sel()))}
      if self.udma_chmap2_ch18sel() != 0 { try!(write!(f, " udma_chmap2_ch18sel=0x{:x}", self.udma_chmap2_ch18sel()))}
      if self.udma_chmap2_ch19sel() != 0 { try!(write!(f, " udma_chmap2_ch19sel=0x{:x}", self.udma_chmap2_ch19sel()))}
      if self.udma_chmap2_ch20sel() != 0 { try!(write!(f, " udma_chmap2_ch20sel=0x{:x}", self.udma_chmap2_ch20sel()))}
      if self.udma_chmap2_ch21sel() != 0 { try!(write!(f, " udma_chmap2_ch21sel=0x{:x}", self.udma_chmap2_ch21sel()))}
      if self.udma_chmap2_ch22sel() != 0 { try!(write!(f, " udma_chmap2_ch22sel=0x{:x}", self.udma_chmap2_ch22sel()))}
      if self.udma_chmap2_ch23sel() != 0 { try!(write!(f, " udma_chmap2_ch23sel=0x{:x}", self.udma_chmap2_ch23sel()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Chmap3(pub u32);
impl Chmap3 {
  #[inline] pub fn udma_chmap3_ch24sel(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_udma_chmap3_ch24sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn udma_chmap3_ch25sel(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
  #[inline] pub fn set_udma_chmap3_ch25sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn udma_chmap3_ch26sel(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  #[inline] pub fn set_udma_chmap3_ch26sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn udma_chmap3_ch27sel(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0xf // [15:12]
  }
  #[inline] pub fn set_udma_chmap3_ch27sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn udma_chmap3_ch28sel(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  #[inline] pub fn set_udma_chmap3_ch28sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  #[inline] pub fn udma_chmap3_ch29sel(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0xf // [23:20]
  }
  #[inline] pub fn set_udma_chmap3_ch29sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

  #[inline] pub fn udma_chmap3_ch30sel(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  #[inline] pub fn set_udma_chmap3_ch30sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

  #[inline] pub fn udma_chmap3_ch31sel(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0xf // [31:28]
  }
  #[inline] pub fn set_udma_chmap3_ch31sel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 28);
     self.0 |= value << 28;
     self
  }

}
impl ::core::fmt::Display for Chmap3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chmap3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.udma_chmap3_ch24sel() != 0 { try!(write!(f, " udma_chmap3_ch24sel=0x{:x}", self.udma_chmap3_ch24sel()))}
      if self.udma_chmap3_ch25sel() != 0 { try!(write!(f, " udma_chmap3_ch25sel=0x{:x}", self.udma_chmap3_ch25sel()))}
      if self.udma_chmap3_ch26sel() != 0 { try!(write!(f, " udma_chmap3_ch26sel=0x{:x}", self.udma_chmap3_ch26sel()))}
      if self.udma_chmap3_ch27sel() != 0 { try!(write!(f, " udma_chmap3_ch27sel=0x{:x}", self.udma_chmap3_ch27sel()))}
      if self.udma_chmap3_ch28sel() != 0 { try!(write!(f, " udma_chmap3_ch28sel=0x{:x}", self.udma_chmap3_ch28sel()))}
      if self.udma_chmap3_ch29sel() != 0 { try!(write!(f, " udma_chmap3_ch29sel=0x{:x}", self.udma_chmap3_ch29sel()))}
      if self.udma_chmap3_ch30sel() != 0 { try!(write!(f, " udma_chmap3_ch30sel=0x{:x}", self.udma_chmap3_ch30sel()))}
      if self.udma_chmap3_ch31sel() != 0 { try!(write!(f, " udma_chmap3_ch31sel=0x{:x}", self.udma_chmap3_ch31sel()))}
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

pub const UDMA_CH0: Channel<UdmaCh0Id, UdmaId> = Channel { periph: UDMA, index: 0, id: UdmaCh0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh0Id {}
pub type UdmaCh0 = Channel<UdmaCh0Id, UdmaId>;

pub const UDMA_CH1: Channel<UdmaCh1Id, UdmaId> = Channel { periph: UDMA, index: 1, id: UdmaCh1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh1Id {}
pub type UdmaCh1 = Channel<UdmaCh1Id, UdmaId>;

pub const UDMA_CH2: Channel<UdmaCh2Id, UdmaId> = Channel { periph: UDMA, index: 2, id: UdmaCh2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh2Id {}
pub type UdmaCh2 = Channel<UdmaCh2Id, UdmaId>;

pub const UDMA_CH3: Channel<UdmaCh3Id, UdmaId> = Channel { periph: UDMA, index: 3, id: UdmaCh3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh3Id {}
pub type UdmaCh3 = Channel<UdmaCh3Id, UdmaId>;

pub const UDMA_CH4: Channel<UdmaCh4Id, UdmaId> = Channel { periph: UDMA, index: 4, id: UdmaCh4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh4Id {}
pub type UdmaCh4 = Channel<UdmaCh4Id, UdmaId>;

pub const UDMA_CH5: Channel<UdmaCh5Id, UdmaId> = Channel { periph: UDMA, index: 5, id: UdmaCh5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh5Id {}
pub type UdmaCh5 = Channel<UdmaCh5Id, UdmaId>;

pub const UDMA_CH6: Channel<UdmaCh6Id, UdmaId> = Channel { periph: UDMA, index: 6, id: UdmaCh6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh6Id {}
pub type UdmaCh6 = Channel<UdmaCh6Id, UdmaId>;

pub const UDMA_CH7: Channel<UdmaCh7Id, UdmaId> = Channel { periph: UDMA, index: 7, id: UdmaCh7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh7Id {}
pub type UdmaCh7 = Channel<UdmaCh7Id, UdmaId>;

pub const UDMA_CH8: Channel<UdmaCh8Id, UdmaId> = Channel { periph: UDMA, index: 8, id: UdmaCh8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh8Id {}
pub type UdmaCh8 = Channel<UdmaCh8Id, UdmaId>;

pub const UDMA_CH9: Channel<UdmaCh9Id, UdmaId> = Channel { periph: UDMA, index: 9, id: UdmaCh9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh9Id {}
pub type UdmaCh9 = Channel<UdmaCh9Id, UdmaId>;

pub const UDMA_CH10: Channel<UdmaCh10Id, UdmaId> = Channel { periph: UDMA, index: 10, id: UdmaCh10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh10Id {}
pub type UdmaCh10 = Channel<UdmaCh10Id, UdmaId>;

pub const UDMA_CH11: Channel<UdmaCh11Id, UdmaId> = Channel { periph: UDMA, index: 11, id: UdmaCh11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh11Id {}
pub type UdmaCh11 = Channel<UdmaCh11Id, UdmaId>;

pub const UDMA_CH12: Channel<UdmaCh12Id, UdmaId> = Channel { periph: UDMA, index: 12, id: UdmaCh12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh12Id {}
pub type UdmaCh12 = Channel<UdmaCh12Id, UdmaId>;

pub const UDMA_CH13: Channel<UdmaCh13Id, UdmaId> = Channel { periph: UDMA, index: 13, id: UdmaCh13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh13Id {}
pub type UdmaCh13 = Channel<UdmaCh13Id, UdmaId>;

pub const UDMA_CH14: Channel<UdmaCh14Id, UdmaId> = Channel { periph: UDMA, index: 14, id: UdmaCh14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh14Id {}
pub type UdmaCh14 = Channel<UdmaCh14Id, UdmaId>;

pub const UDMA_CH15: Channel<UdmaCh15Id, UdmaId> = Channel { periph: UDMA, index: 15, id: UdmaCh15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh15Id {}
pub type UdmaCh15 = Channel<UdmaCh15Id, UdmaId>;

pub const UDMA_CH16: Channel<UdmaCh16Id, UdmaId> = Channel { periph: UDMA, index: 16, id: UdmaCh16Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh16Id {}
pub type UdmaCh16 = Channel<UdmaCh16Id, UdmaId>;

pub const UDMA_CH17: Channel<UdmaCh17Id, UdmaId> = Channel { periph: UDMA, index: 17, id: UdmaCh17Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh17Id {}
pub type UdmaCh17 = Channel<UdmaCh17Id, UdmaId>;

pub const UDMA_CH18: Channel<UdmaCh18Id, UdmaId> = Channel { periph: UDMA, index: 18, id: UdmaCh18Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh18Id {}
pub type UdmaCh18 = Channel<UdmaCh18Id, UdmaId>;

pub const UDMA_CH19: Channel<UdmaCh19Id, UdmaId> = Channel { periph: UDMA, index: 19, id: UdmaCh19Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh19Id {}
pub type UdmaCh19 = Channel<UdmaCh19Id, UdmaId>;

pub const UDMA_CH20: Channel<UdmaCh20Id, UdmaId> = Channel { periph: UDMA, index: 20, id: UdmaCh20Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh20Id {}
pub type UdmaCh20 = Channel<UdmaCh20Id, UdmaId>;

pub const UDMA_CH21: Channel<UdmaCh21Id, UdmaId> = Channel { periph: UDMA, index: 21, id: UdmaCh21Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh21Id {}
pub type UdmaCh21 = Channel<UdmaCh21Id, UdmaId>;

pub const UDMA_CH22: Channel<UdmaCh22Id, UdmaId> = Channel { periph: UDMA, index: 22, id: UdmaCh22Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh22Id {}
pub type UdmaCh22 = Channel<UdmaCh22Id, UdmaId>;

pub const UDMA_CH23: Channel<UdmaCh23Id, UdmaId> = Channel { periph: UDMA, index: 23, id: UdmaCh23Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh23Id {}
pub type UdmaCh23 = Channel<UdmaCh23Id, UdmaId>;

pub const UDMA_CH24: Channel<UdmaCh24Id, UdmaId> = Channel { periph: UDMA, index: 24, id: UdmaCh24Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh24Id {}
pub type UdmaCh24 = Channel<UdmaCh24Id, UdmaId>;

pub const UDMA_CH25: Channel<UdmaCh25Id, UdmaId> = Channel { periph: UDMA, index: 25, id: UdmaCh25Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh25Id {}
pub type UdmaCh25 = Channel<UdmaCh25Id, UdmaId>;

pub const UDMA_CH26: Channel<UdmaCh26Id, UdmaId> = Channel { periph: UDMA, index: 26, id: UdmaCh26Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh26Id {}
pub type UdmaCh26 = Channel<UdmaCh26Id, UdmaId>;

pub const UDMA_CH27: Channel<UdmaCh27Id, UdmaId> = Channel { periph: UDMA, index: 27, id: UdmaCh27Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh27Id {}
pub type UdmaCh27 = Channel<UdmaCh27Id, UdmaId>;

pub const UDMA_CH28: Channel<UdmaCh28Id, UdmaId> = Channel { periph: UDMA, index: 28, id: UdmaCh28Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh28Id {}
pub type UdmaCh28 = Channel<UdmaCh28Id, UdmaId>;

pub const UDMA_CH29: Channel<UdmaCh29Id, UdmaId> = Channel { periph: UDMA, index: 29, id: UdmaCh29Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh29Id {}
pub type UdmaCh29 = Channel<UdmaCh29Id, UdmaId>;

pub const UDMA_CH30: Channel<UdmaCh30Id, UdmaId> = Channel { periph: UDMA, index: 30, id: UdmaCh30Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh30Id {}
pub type UdmaCh30 = Channel<UdmaCh30Id, UdmaId>;

pub const UDMA_CH31: Channel<UdmaCh31Id, UdmaId> = Channel { periph: UDMA, index: 31, id: UdmaCh31Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct UdmaCh31Id {}
pub type UdmaCh31 = Channel<UdmaCh31Id, UdmaId>;

