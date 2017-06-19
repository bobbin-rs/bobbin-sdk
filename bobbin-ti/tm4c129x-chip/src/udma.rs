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
  #[inline] pub fn udma_waitstat_waitreq(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_udma_waitstat_waitreq(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
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
      if self.udma_waitstat_waitreq(0) != 0 { try!(write!(f, " udma_waitstat_waitreq[0]"))}
      if self.udma_waitstat_waitreq(1) != 0 { try!(write!(f, " udma_waitstat_waitreq[1]"))}
      if self.udma_waitstat_waitreq(2) != 0 { try!(write!(f, " udma_waitstat_waitreq[2]"))}
      if self.udma_waitstat_waitreq(3) != 0 { try!(write!(f, " udma_waitstat_waitreq[3]"))}
      if self.udma_waitstat_waitreq(4) != 0 { try!(write!(f, " udma_waitstat_waitreq[4]"))}
      if self.udma_waitstat_waitreq(5) != 0 { try!(write!(f, " udma_waitstat_waitreq[5]"))}
      if self.udma_waitstat_waitreq(6) != 0 { try!(write!(f, " udma_waitstat_waitreq[6]"))}
      if self.udma_waitstat_waitreq(7) != 0 { try!(write!(f, " udma_waitstat_waitreq[7]"))}
      if self.udma_waitstat_waitreq(8) != 0 { try!(write!(f, " udma_waitstat_waitreq[8]"))}
      if self.udma_waitstat_waitreq(9) != 0 { try!(write!(f, " udma_waitstat_waitreq[9]"))}
      if self.udma_waitstat_waitreq(10) != 0 { try!(write!(f, " udma_waitstat_waitreq[10]"))}
      if self.udma_waitstat_waitreq(11) != 0 { try!(write!(f, " udma_waitstat_waitreq[11]"))}
      if self.udma_waitstat_waitreq(12) != 0 { try!(write!(f, " udma_waitstat_waitreq[12]"))}
      if self.udma_waitstat_waitreq(13) != 0 { try!(write!(f, " udma_waitstat_waitreq[13]"))}
      if self.udma_waitstat_waitreq(14) != 0 { try!(write!(f, " udma_waitstat_waitreq[14]"))}
      if self.udma_waitstat_waitreq(15) != 0 { try!(write!(f, " udma_waitstat_waitreq[15]"))}
      if self.udma_waitstat_waitreq(16) != 0 { try!(write!(f, " udma_waitstat_waitreq[16]"))}
      if self.udma_waitstat_waitreq(17) != 0 { try!(write!(f, " udma_waitstat_waitreq[17]"))}
      if self.udma_waitstat_waitreq(18) != 0 { try!(write!(f, " udma_waitstat_waitreq[18]"))}
      if self.udma_waitstat_waitreq(19) != 0 { try!(write!(f, " udma_waitstat_waitreq[19]"))}
      if self.udma_waitstat_waitreq(20) != 0 { try!(write!(f, " udma_waitstat_waitreq[20]"))}
      if self.udma_waitstat_waitreq(21) != 0 { try!(write!(f, " udma_waitstat_waitreq[21]"))}
      if self.udma_waitstat_waitreq(22) != 0 { try!(write!(f, " udma_waitstat_waitreq[22]"))}
      if self.udma_waitstat_waitreq(23) != 0 { try!(write!(f, " udma_waitstat_waitreq[23]"))}
      if self.udma_waitstat_waitreq(24) != 0 { try!(write!(f, " udma_waitstat_waitreq[24]"))}
      if self.udma_waitstat_waitreq(25) != 0 { try!(write!(f, " udma_waitstat_waitreq[25]"))}
      if self.udma_waitstat_waitreq(26) != 0 { try!(write!(f, " udma_waitstat_waitreq[26]"))}
      if self.udma_waitstat_waitreq(27) != 0 { try!(write!(f, " udma_waitstat_waitreq[27]"))}
      if self.udma_waitstat_waitreq(28) != 0 { try!(write!(f, " udma_waitstat_waitreq[28]"))}
      if self.udma_waitstat_waitreq(29) != 0 { try!(write!(f, " udma_waitstat_waitreq[29]"))}
      if self.udma_waitstat_waitreq(30) != 0 { try!(write!(f, " udma_waitstat_waitreq[30]"))}
      if self.udma_waitstat_waitreq(31) != 0 { try!(write!(f, " udma_waitstat_waitreq[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Swreq(pub u32);
impl Swreq {
  #[inline] pub fn udma_swreq(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_udma_swreq(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
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
      if self.udma_swreq(0) != 0 { try!(write!(f, " udma_swreq[0]"))}
      if self.udma_swreq(1) != 0 { try!(write!(f, " udma_swreq[1]"))}
      if self.udma_swreq(2) != 0 { try!(write!(f, " udma_swreq[2]"))}
      if self.udma_swreq(3) != 0 { try!(write!(f, " udma_swreq[3]"))}
      if self.udma_swreq(4) != 0 { try!(write!(f, " udma_swreq[4]"))}
      if self.udma_swreq(5) != 0 { try!(write!(f, " udma_swreq[5]"))}
      if self.udma_swreq(6) != 0 { try!(write!(f, " udma_swreq[6]"))}
      if self.udma_swreq(7) != 0 { try!(write!(f, " udma_swreq[7]"))}
      if self.udma_swreq(8) != 0 { try!(write!(f, " udma_swreq[8]"))}
      if self.udma_swreq(9) != 0 { try!(write!(f, " udma_swreq[9]"))}
      if self.udma_swreq(10) != 0 { try!(write!(f, " udma_swreq[10]"))}
      if self.udma_swreq(11) != 0 { try!(write!(f, " udma_swreq[11]"))}
      if self.udma_swreq(12) != 0 { try!(write!(f, " udma_swreq[12]"))}
      if self.udma_swreq(13) != 0 { try!(write!(f, " udma_swreq[13]"))}
      if self.udma_swreq(14) != 0 { try!(write!(f, " udma_swreq[14]"))}
      if self.udma_swreq(15) != 0 { try!(write!(f, " udma_swreq[15]"))}
      if self.udma_swreq(16) != 0 { try!(write!(f, " udma_swreq[16]"))}
      if self.udma_swreq(17) != 0 { try!(write!(f, " udma_swreq[17]"))}
      if self.udma_swreq(18) != 0 { try!(write!(f, " udma_swreq[18]"))}
      if self.udma_swreq(19) != 0 { try!(write!(f, " udma_swreq[19]"))}
      if self.udma_swreq(20) != 0 { try!(write!(f, " udma_swreq[20]"))}
      if self.udma_swreq(21) != 0 { try!(write!(f, " udma_swreq[21]"))}
      if self.udma_swreq(22) != 0 { try!(write!(f, " udma_swreq[22]"))}
      if self.udma_swreq(23) != 0 { try!(write!(f, " udma_swreq[23]"))}
      if self.udma_swreq(24) != 0 { try!(write!(f, " udma_swreq[24]"))}
      if self.udma_swreq(25) != 0 { try!(write!(f, " udma_swreq[25]"))}
      if self.udma_swreq(26) != 0 { try!(write!(f, " udma_swreq[26]"))}
      if self.udma_swreq(27) != 0 { try!(write!(f, " udma_swreq[27]"))}
      if self.udma_swreq(28) != 0 { try!(write!(f, " udma_swreq[28]"))}
      if self.udma_swreq(29) != 0 { try!(write!(f, " udma_swreq[29]"))}
      if self.udma_swreq(30) != 0 { try!(write!(f, " udma_swreq[30]"))}
      if self.udma_swreq(31) != 0 { try!(write!(f, " udma_swreq[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Useburstset(pub u32);
impl Useburstset {
  #[inline] pub fn udma_useburstset_set(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_udma_useburstset_set(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
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
      if self.udma_useburstset_set(0) != 0 { try!(write!(f, " udma_useburstset_set[0]"))}
      if self.udma_useburstset_set(1) != 0 { try!(write!(f, " udma_useburstset_set[1]"))}
      if self.udma_useburstset_set(2) != 0 { try!(write!(f, " udma_useburstset_set[2]"))}
      if self.udma_useburstset_set(3) != 0 { try!(write!(f, " udma_useburstset_set[3]"))}
      if self.udma_useburstset_set(4) != 0 { try!(write!(f, " udma_useburstset_set[4]"))}
      if self.udma_useburstset_set(5) != 0 { try!(write!(f, " udma_useburstset_set[5]"))}
      if self.udma_useburstset_set(6) != 0 { try!(write!(f, " udma_useburstset_set[6]"))}
      if self.udma_useburstset_set(7) != 0 { try!(write!(f, " udma_useburstset_set[7]"))}
      if self.udma_useburstset_set(8) != 0 { try!(write!(f, " udma_useburstset_set[8]"))}
      if self.udma_useburstset_set(9) != 0 { try!(write!(f, " udma_useburstset_set[9]"))}
      if self.udma_useburstset_set(10) != 0 { try!(write!(f, " udma_useburstset_set[10]"))}
      if self.udma_useburstset_set(11) != 0 { try!(write!(f, " udma_useburstset_set[11]"))}
      if self.udma_useburstset_set(12) != 0 { try!(write!(f, " udma_useburstset_set[12]"))}
      if self.udma_useburstset_set(13) != 0 { try!(write!(f, " udma_useburstset_set[13]"))}
      if self.udma_useburstset_set(14) != 0 { try!(write!(f, " udma_useburstset_set[14]"))}
      if self.udma_useburstset_set(15) != 0 { try!(write!(f, " udma_useburstset_set[15]"))}
      if self.udma_useburstset_set(16) != 0 { try!(write!(f, " udma_useburstset_set[16]"))}
      if self.udma_useburstset_set(17) != 0 { try!(write!(f, " udma_useburstset_set[17]"))}
      if self.udma_useburstset_set(18) != 0 { try!(write!(f, " udma_useburstset_set[18]"))}
      if self.udma_useburstset_set(19) != 0 { try!(write!(f, " udma_useburstset_set[19]"))}
      if self.udma_useburstset_set(20) != 0 { try!(write!(f, " udma_useburstset_set[20]"))}
      if self.udma_useburstset_set(21) != 0 { try!(write!(f, " udma_useburstset_set[21]"))}
      if self.udma_useburstset_set(22) != 0 { try!(write!(f, " udma_useburstset_set[22]"))}
      if self.udma_useburstset_set(23) != 0 { try!(write!(f, " udma_useburstset_set[23]"))}
      if self.udma_useburstset_set(24) != 0 { try!(write!(f, " udma_useburstset_set[24]"))}
      if self.udma_useburstset_set(25) != 0 { try!(write!(f, " udma_useburstset_set[25]"))}
      if self.udma_useburstset_set(26) != 0 { try!(write!(f, " udma_useburstset_set[26]"))}
      if self.udma_useburstset_set(27) != 0 { try!(write!(f, " udma_useburstset_set[27]"))}
      if self.udma_useburstset_set(28) != 0 { try!(write!(f, " udma_useburstset_set[28]"))}
      if self.udma_useburstset_set(29) != 0 { try!(write!(f, " udma_useburstset_set[29]"))}
      if self.udma_useburstset_set(30) != 0 { try!(write!(f, " udma_useburstset_set[30]"))}
      if self.udma_useburstset_set(31) != 0 { try!(write!(f, " udma_useburstset_set[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Useburstclr(pub u32);
impl Useburstclr {
  #[inline] pub fn udma_useburstclr_clr(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_udma_useburstclr_clr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
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
      if self.udma_useburstclr_clr(0) != 0 { try!(write!(f, " udma_useburstclr_clr[0]"))}
      if self.udma_useburstclr_clr(1) != 0 { try!(write!(f, " udma_useburstclr_clr[1]"))}
      if self.udma_useburstclr_clr(2) != 0 { try!(write!(f, " udma_useburstclr_clr[2]"))}
      if self.udma_useburstclr_clr(3) != 0 { try!(write!(f, " udma_useburstclr_clr[3]"))}
      if self.udma_useburstclr_clr(4) != 0 { try!(write!(f, " udma_useburstclr_clr[4]"))}
      if self.udma_useburstclr_clr(5) != 0 { try!(write!(f, " udma_useburstclr_clr[5]"))}
      if self.udma_useburstclr_clr(6) != 0 { try!(write!(f, " udma_useburstclr_clr[6]"))}
      if self.udma_useburstclr_clr(7) != 0 { try!(write!(f, " udma_useburstclr_clr[7]"))}
      if self.udma_useburstclr_clr(8) != 0 { try!(write!(f, " udma_useburstclr_clr[8]"))}
      if self.udma_useburstclr_clr(9) != 0 { try!(write!(f, " udma_useburstclr_clr[9]"))}
      if self.udma_useburstclr_clr(10) != 0 { try!(write!(f, " udma_useburstclr_clr[10]"))}
      if self.udma_useburstclr_clr(11) != 0 { try!(write!(f, " udma_useburstclr_clr[11]"))}
      if self.udma_useburstclr_clr(12) != 0 { try!(write!(f, " udma_useburstclr_clr[12]"))}
      if self.udma_useburstclr_clr(13) != 0 { try!(write!(f, " udma_useburstclr_clr[13]"))}
      if self.udma_useburstclr_clr(14) != 0 { try!(write!(f, " udma_useburstclr_clr[14]"))}
      if self.udma_useburstclr_clr(15) != 0 { try!(write!(f, " udma_useburstclr_clr[15]"))}
      if self.udma_useburstclr_clr(16) != 0 { try!(write!(f, " udma_useburstclr_clr[16]"))}
      if self.udma_useburstclr_clr(17) != 0 { try!(write!(f, " udma_useburstclr_clr[17]"))}
      if self.udma_useburstclr_clr(18) != 0 { try!(write!(f, " udma_useburstclr_clr[18]"))}
      if self.udma_useburstclr_clr(19) != 0 { try!(write!(f, " udma_useburstclr_clr[19]"))}
      if self.udma_useburstclr_clr(20) != 0 { try!(write!(f, " udma_useburstclr_clr[20]"))}
      if self.udma_useburstclr_clr(21) != 0 { try!(write!(f, " udma_useburstclr_clr[21]"))}
      if self.udma_useburstclr_clr(22) != 0 { try!(write!(f, " udma_useburstclr_clr[22]"))}
      if self.udma_useburstclr_clr(23) != 0 { try!(write!(f, " udma_useburstclr_clr[23]"))}
      if self.udma_useburstclr_clr(24) != 0 { try!(write!(f, " udma_useburstclr_clr[24]"))}
      if self.udma_useburstclr_clr(25) != 0 { try!(write!(f, " udma_useburstclr_clr[25]"))}
      if self.udma_useburstclr_clr(26) != 0 { try!(write!(f, " udma_useburstclr_clr[26]"))}
      if self.udma_useburstclr_clr(27) != 0 { try!(write!(f, " udma_useburstclr_clr[27]"))}
      if self.udma_useburstclr_clr(28) != 0 { try!(write!(f, " udma_useburstclr_clr[28]"))}
      if self.udma_useburstclr_clr(29) != 0 { try!(write!(f, " udma_useburstclr_clr[29]"))}
      if self.udma_useburstclr_clr(30) != 0 { try!(write!(f, " udma_useburstclr_clr[30]"))}
      if self.udma_useburstclr_clr(31) != 0 { try!(write!(f, " udma_useburstclr_clr[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Reqmaskset(pub u32);
impl Reqmaskset {
  #[inline] pub fn udma_reqmaskset_set(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_udma_reqmaskset_set(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
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
      if self.udma_reqmaskset_set(0) != 0 { try!(write!(f, " udma_reqmaskset_set[0]"))}
      if self.udma_reqmaskset_set(1) != 0 { try!(write!(f, " udma_reqmaskset_set[1]"))}
      if self.udma_reqmaskset_set(2) != 0 { try!(write!(f, " udma_reqmaskset_set[2]"))}
      if self.udma_reqmaskset_set(3) != 0 { try!(write!(f, " udma_reqmaskset_set[3]"))}
      if self.udma_reqmaskset_set(4) != 0 { try!(write!(f, " udma_reqmaskset_set[4]"))}
      if self.udma_reqmaskset_set(5) != 0 { try!(write!(f, " udma_reqmaskset_set[5]"))}
      if self.udma_reqmaskset_set(6) != 0 { try!(write!(f, " udma_reqmaskset_set[6]"))}
      if self.udma_reqmaskset_set(7) != 0 { try!(write!(f, " udma_reqmaskset_set[7]"))}
      if self.udma_reqmaskset_set(8) != 0 { try!(write!(f, " udma_reqmaskset_set[8]"))}
      if self.udma_reqmaskset_set(9) != 0 { try!(write!(f, " udma_reqmaskset_set[9]"))}
      if self.udma_reqmaskset_set(10) != 0 { try!(write!(f, " udma_reqmaskset_set[10]"))}
      if self.udma_reqmaskset_set(11) != 0 { try!(write!(f, " udma_reqmaskset_set[11]"))}
      if self.udma_reqmaskset_set(12) != 0 { try!(write!(f, " udma_reqmaskset_set[12]"))}
      if self.udma_reqmaskset_set(13) != 0 { try!(write!(f, " udma_reqmaskset_set[13]"))}
      if self.udma_reqmaskset_set(14) != 0 { try!(write!(f, " udma_reqmaskset_set[14]"))}
      if self.udma_reqmaskset_set(15) != 0 { try!(write!(f, " udma_reqmaskset_set[15]"))}
      if self.udma_reqmaskset_set(16) != 0 { try!(write!(f, " udma_reqmaskset_set[16]"))}
      if self.udma_reqmaskset_set(17) != 0 { try!(write!(f, " udma_reqmaskset_set[17]"))}
      if self.udma_reqmaskset_set(18) != 0 { try!(write!(f, " udma_reqmaskset_set[18]"))}
      if self.udma_reqmaskset_set(19) != 0 { try!(write!(f, " udma_reqmaskset_set[19]"))}
      if self.udma_reqmaskset_set(20) != 0 { try!(write!(f, " udma_reqmaskset_set[20]"))}
      if self.udma_reqmaskset_set(21) != 0 { try!(write!(f, " udma_reqmaskset_set[21]"))}
      if self.udma_reqmaskset_set(22) != 0 { try!(write!(f, " udma_reqmaskset_set[22]"))}
      if self.udma_reqmaskset_set(23) != 0 { try!(write!(f, " udma_reqmaskset_set[23]"))}
      if self.udma_reqmaskset_set(24) != 0 { try!(write!(f, " udma_reqmaskset_set[24]"))}
      if self.udma_reqmaskset_set(25) != 0 { try!(write!(f, " udma_reqmaskset_set[25]"))}
      if self.udma_reqmaskset_set(26) != 0 { try!(write!(f, " udma_reqmaskset_set[26]"))}
      if self.udma_reqmaskset_set(27) != 0 { try!(write!(f, " udma_reqmaskset_set[27]"))}
      if self.udma_reqmaskset_set(28) != 0 { try!(write!(f, " udma_reqmaskset_set[28]"))}
      if self.udma_reqmaskset_set(29) != 0 { try!(write!(f, " udma_reqmaskset_set[29]"))}
      if self.udma_reqmaskset_set(30) != 0 { try!(write!(f, " udma_reqmaskset_set[30]"))}
      if self.udma_reqmaskset_set(31) != 0 { try!(write!(f, " udma_reqmaskset_set[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Reqmaskclr(pub u32);
impl Reqmaskclr {
  #[inline] pub fn udma_reqmaskclr_clr(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_udma_reqmaskclr_clr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
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
      if self.udma_reqmaskclr_clr(0) != 0 { try!(write!(f, " udma_reqmaskclr_clr[0]"))}
      if self.udma_reqmaskclr_clr(1) != 0 { try!(write!(f, " udma_reqmaskclr_clr[1]"))}
      if self.udma_reqmaskclr_clr(2) != 0 { try!(write!(f, " udma_reqmaskclr_clr[2]"))}
      if self.udma_reqmaskclr_clr(3) != 0 { try!(write!(f, " udma_reqmaskclr_clr[3]"))}
      if self.udma_reqmaskclr_clr(4) != 0 { try!(write!(f, " udma_reqmaskclr_clr[4]"))}
      if self.udma_reqmaskclr_clr(5) != 0 { try!(write!(f, " udma_reqmaskclr_clr[5]"))}
      if self.udma_reqmaskclr_clr(6) != 0 { try!(write!(f, " udma_reqmaskclr_clr[6]"))}
      if self.udma_reqmaskclr_clr(7) != 0 { try!(write!(f, " udma_reqmaskclr_clr[7]"))}
      if self.udma_reqmaskclr_clr(8) != 0 { try!(write!(f, " udma_reqmaskclr_clr[8]"))}
      if self.udma_reqmaskclr_clr(9) != 0 { try!(write!(f, " udma_reqmaskclr_clr[9]"))}
      if self.udma_reqmaskclr_clr(10) != 0 { try!(write!(f, " udma_reqmaskclr_clr[10]"))}
      if self.udma_reqmaskclr_clr(11) != 0 { try!(write!(f, " udma_reqmaskclr_clr[11]"))}
      if self.udma_reqmaskclr_clr(12) != 0 { try!(write!(f, " udma_reqmaskclr_clr[12]"))}
      if self.udma_reqmaskclr_clr(13) != 0 { try!(write!(f, " udma_reqmaskclr_clr[13]"))}
      if self.udma_reqmaskclr_clr(14) != 0 { try!(write!(f, " udma_reqmaskclr_clr[14]"))}
      if self.udma_reqmaskclr_clr(15) != 0 { try!(write!(f, " udma_reqmaskclr_clr[15]"))}
      if self.udma_reqmaskclr_clr(16) != 0 { try!(write!(f, " udma_reqmaskclr_clr[16]"))}
      if self.udma_reqmaskclr_clr(17) != 0 { try!(write!(f, " udma_reqmaskclr_clr[17]"))}
      if self.udma_reqmaskclr_clr(18) != 0 { try!(write!(f, " udma_reqmaskclr_clr[18]"))}
      if self.udma_reqmaskclr_clr(19) != 0 { try!(write!(f, " udma_reqmaskclr_clr[19]"))}
      if self.udma_reqmaskclr_clr(20) != 0 { try!(write!(f, " udma_reqmaskclr_clr[20]"))}
      if self.udma_reqmaskclr_clr(21) != 0 { try!(write!(f, " udma_reqmaskclr_clr[21]"))}
      if self.udma_reqmaskclr_clr(22) != 0 { try!(write!(f, " udma_reqmaskclr_clr[22]"))}
      if self.udma_reqmaskclr_clr(23) != 0 { try!(write!(f, " udma_reqmaskclr_clr[23]"))}
      if self.udma_reqmaskclr_clr(24) != 0 { try!(write!(f, " udma_reqmaskclr_clr[24]"))}
      if self.udma_reqmaskclr_clr(25) != 0 { try!(write!(f, " udma_reqmaskclr_clr[25]"))}
      if self.udma_reqmaskclr_clr(26) != 0 { try!(write!(f, " udma_reqmaskclr_clr[26]"))}
      if self.udma_reqmaskclr_clr(27) != 0 { try!(write!(f, " udma_reqmaskclr_clr[27]"))}
      if self.udma_reqmaskclr_clr(28) != 0 { try!(write!(f, " udma_reqmaskclr_clr[28]"))}
      if self.udma_reqmaskclr_clr(29) != 0 { try!(write!(f, " udma_reqmaskclr_clr[29]"))}
      if self.udma_reqmaskclr_clr(30) != 0 { try!(write!(f, " udma_reqmaskclr_clr[30]"))}
      if self.udma_reqmaskclr_clr(31) != 0 { try!(write!(f, " udma_reqmaskclr_clr[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Enaset(pub u32);
impl Enaset {
  #[inline] pub fn udma_enaset_set(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_udma_enaset_set(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
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
      if self.udma_enaset_set(0) != 0 { try!(write!(f, " udma_enaset_set[0]"))}
      if self.udma_enaset_set(1) != 0 { try!(write!(f, " udma_enaset_set[1]"))}
      if self.udma_enaset_set(2) != 0 { try!(write!(f, " udma_enaset_set[2]"))}
      if self.udma_enaset_set(3) != 0 { try!(write!(f, " udma_enaset_set[3]"))}
      if self.udma_enaset_set(4) != 0 { try!(write!(f, " udma_enaset_set[4]"))}
      if self.udma_enaset_set(5) != 0 { try!(write!(f, " udma_enaset_set[5]"))}
      if self.udma_enaset_set(6) != 0 { try!(write!(f, " udma_enaset_set[6]"))}
      if self.udma_enaset_set(7) != 0 { try!(write!(f, " udma_enaset_set[7]"))}
      if self.udma_enaset_set(8) != 0 { try!(write!(f, " udma_enaset_set[8]"))}
      if self.udma_enaset_set(9) != 0 { try!(write!(f, " udma_enaset_set[9]"))}
      if self.udma_enaset_set(10) != 0 { try!(write!(f, " udma_enaset_set[10]"))}
      if self.udma_enaset_set(11) != 0 { try!(write!(f, " udma_enaset_set[11]"))}
      if self.udma_enaset_set(12) != 0 { try!(write!(f, " udma_enaset_set[12]"))}
      if self.udma_enaset_set(13) != 0 { try!(write!(f, " udma_enaset_set[13]"))}
      if self.udma_enaset_set(14) != 0 { try!(write!(f, " udma_enaset_set[14]"))}
      if self.udma_enaset_set(15) != 0 { try!(write!(f, " udma_enaset_set[15]"))}
      if self.udma_enaset_set(16) != 0 { try!(write!(f, " udma_enaset_set[16]"))}
      if self.udma_enaset_set(17) != 0 { try!(write!(f, " udma_enaset_set[17]"))}
      if self.udma_enaset_set(18) != 0 { try!(write!(f, " udma_enaset_set[18]"))}
      if self.udma_enaset_set(19) != 0 { try!(write!(f, " udma_enaset_set[19]"))}
      if self.udma_enaset_set(20) != 0 { try!(write!(f, " udma_enaset_set[20]"))}
      if self.udma_enaset_set(21) != 0 { try!(write!(f, " udma_enaset_set[21]"))}
      if self.udma_enaset_set(22) != 0 { try!(write!(f, " udma_enaset_set[22]"))}
      if self.udma_enaset_set(23) != 0 { try!(write!(f, " udma_enaset_set[23]"))}
      if self.udma_enaset_set(24) != 0 { try!(write!(f, " udma_enaset_set[24]"))}
      if self.udma_enaset_set(25) != 0 { try!(write!(f, " udma_enaset_set[25]"))}
      if self.udma_enaset_set(26) != 0 { try!(write!(f, " udma_enaset_set[26]"))}
      if self.udma_enaset_set(27) != 0 { try!(write!(f, " udma_enaset_set[27]"))}
      if self.udma_enaset_set(28) != 0 { try!(write!(f, " udma_enaset_set[28]"))}
      if self.udma_enaset_set(29) != 0 { try!(write!(f, " udma_enaset_set[29]"))}
      if self.udma_enaset_set(30) != 0 { try!(write!(f, " udma_enaset_set[30]"))}
      if self.udma_enaset_set(31) != 0 { try!(write!(f, " udma_enaset_set[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Enaclr(pub u32);
impl Enaclr {
  #[inline] pub fn udma_enaclr_clr(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_udma_enaclr_clr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
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
      if self.udma_enaclr_clr(0) != 0 { try!(write!(f, " udma_enaclr_clr[0]"))}
      if self.udma_enaclr_clr(1) != 0 { try!(write!(f, " udma_enaclr_clr[1]"))}
      if self.udma_enaclr_clr(2) != 0 { try!(write!(f, " udma_enaclr_clr[2]"))}
      if self.udma_enaclr_clr(3) != 0 { try!(write!(f, " udma_enaclr_clr[3]"))}
      if self.udma_enaclr_clr(4) != 0 { try!(write!(f, " udma_enaclr_clr[4]"))}
      if self.udma_enaclr_clr(5) != 0 { try!(write!(f, " udma_enaclr_clr[5]"))}
      if self.udma_enaclr_clr(6) != 0 { try!(write!(f, " udma_enaclr_clr[6]"))}
      if self.udma_enaclr_clr(7) != 0 { try!(write!(f, " udma_enaclr_clr[7]"))}
      if self.udma_enaclr_clr(8) != 0 { try!(write!(f, " udma_enaclr_clr[8]"))}
      if self.udma_enaclr_clr(9) != 0 { try!(write!(f, " udma_enaclr_clr[9]"))}
      if self.udma_enaclr_clr(10) != 0 { try!(write!(f, " udma_enaclr_clr[10]"))}
      if self.udma_enaclr_clr(11) != 0 { try!(write!(f, " udma_enaclr_clr[11]"))}
      if self.udma_enaclr_clr(12) != 0 { try!(write!(f, " udma_enaclr_clr[12]"))}
      if self.udma_enaclr_clr(13) != 0 { try!(write!(f, " udma_enaclr_clr[13]"))}
      if self.udma_enaclr_clr(14) != 0 { try!(write!(f, " udma_enaclr_clr[14]"))}
      if self.udma_enaclr_clr(15) != 0 { try!(write!(f, " udma_enaclr_clr[15]"))}
      if self.udma_enaclr_clr(16) != 0 { try!(write!(f, " udma_enaclr_clr[16]"))}
      if self.udma_enaclr_clr(17) != 0 { try!(write!(f, " udma_enaclr_clr[17]"))}
      if self.udma_enaclr_clr(18) != 0 { try!(write!(f, " udma_enaclr_clr[18]"))}
      if self.udma_enaclr_clr(19) != 0 { try!(write!(f, " udma_enaclr_clr[19]"))}
      if self.udma_enaclr_clr(20) != 0 { try!(write!(f, " udma_enaclr_clr[20]"))}
      if self.udma_enaclr_clr(21) != 0 { try!(write!(f, " udma_enaclr_clr[21]"))}
      if self.udma_enaclr_clr(22) != 0 { try!(write!(f, " udma_enaclr_clr[22]"))}
      if self.udma_enaclr_clr(23) != 0 { try!(write!(f, " udma_enaclr_clr[23]"))}
      if self.udma_enaclr_clr(24) != 0 { try!(write!(f, " udma_enaclr_clr[24]"))}
      if self.udma_enaclr_clr(25) != 0 { try!(write!(f, " udma_enaclr_clr[25]"))}
      if self.udma_enaclr_clr(26) != 0 { try!(write!(f, " udma_enaclr_clr[26]"))}
      if self.udma_enaclr_clr(27) != 0 { try!(write!(f, " udma_enaclr_clr[27]"))}
      if self.udma_enaclr_clr(28) != 0 { try!(write!(f, " udma_enaclr_clr[28]"))}
      if self.udma_enaclr_clr(29) != 0 { try!(write!(f, " udma_enaclr_clr[29]"))}
      if self.udma_enaclr_clr(30) != 0 { try!(write!(f, " udma_enaclr_clr[30]"))}
      if self.udma_enaclr_clr(31) != 0 { try!(write!(f, " udma_enaclr_clr[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Altset(pub u32);
impl Altset {
  #[inline] pub fn udma_altset_set(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_udma_altset_set(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
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
      if self.udma_altset_set(0) != 0 { try!(write!(f, " udma_altset_set[0]"))}
      if self.udma_altset_set(1) != 0 { try!(write!(f, " udma_altset_set[1]"))}
      if self.udma_altset_set(2) != 0 { try!(write!(f, " udma_altset_set[2]"))}
      if self.udma_altset_set(3) != 0 { try!(write!(f, " udma_altset_set[3]"))}
      if self.udma_altset_set(4) != 0 { try!(write!(f, " udma_altset_set[4]"))}
      if self.udma_altset_set(5) != 0 { try!(write!(f, " udma_altset_set[5]"))}
      if self.udma_altset_set(6) != 0 { try!(write!(f, " udma_altset_set[6]"))}
      if self.udma_altset_set(7) != 0 { try!(write!(f, " udma_altset_set[7]"))}
      if self.udma_altset_set(8) != 0 { try!(write!(f, " udma_altset_set[8]"))}
      if self.udma_altset_set(9) != 0 { try!(write!(f, " udma_altset_set[9]"))}
      if self.udma_altset_set(10) != 0 { try!(write!(f, " udma_altset_set[10]"))}
      if self.udma_altset_set(11) != 0 { try!(write!(f, " udma_altset_set[11]"))}
      if self.udma_altset_set(12) != 0 { try!(write!(f, " udma_altset_set[12]"))}
      if self.udma_altset_set(13) != 0 { try!(write!(f, " udma_altset_set[13]"))}
      if self.udma_altset_set(14) != 0 { try!(write!(f, " udma_altset_set[14]"))}
      if self.udma_altset_set(15) != 0 { try!(write!(f, " udma_altset_set[15]"))}
      if self.udma_altset_set(16) != 0 { try!(write!(f, " udma_altset_set[16]"))}
      if self.udma_altset_set(17) != 0 { try!(write!(f, " udma_altset_set[17]"))}
      if self.udma_altset_set(18) != 0 { try!(write!(f, " udma_altset_set[18]"))}
      if self.udma_altset_set(19) != 0 { try!(write!(f, " udma_altset_set[19]"))}
      if self.udma_altset_set(20) != 0 { try!(write!(f, " udma_altset_set[20]"))}
      if self.udma_altset_set(21) != 0 { try!(write!(f, " udma_altset_set[21]"))}
      if self.udma_altset_set(22) != 0 { try!(write!(f, " udma_altset_set[22]"))}
      if self.udma_altset_set(23) != 0 { try!(write!(f, " udma_altset_set[23]"))}
      if self.udma_altset_set(24) != 0 { try!(write!(f, " udma_altset_set[24]"))}
      if self.udma_altset_set(25) != 0 { try!(write!(f, " udma_altset_set[25]"))}
      if self.udma_altset_set(26) != 0 { try!(write!(f, " udma_altset_set[26]"))}
      if self.udma_altset_set(27) != 0 { try!(write!(f, " udma_altset_set[27]"))}
      if self.udma_altset_set(28) != 0 { try!(write!(f, " udma_altset_set[28]"))}
      if self.udma_altset_set(29) != 0 { try!(write!(f, " udma_altset_set[29]"))}
      if self.udma_altset_set(30) != 0 { try!(write!(f, " udma_altset_set[30]"))}
      if self.udma_altset_set(31) != 0 { try!(write!(f, " udma_altset_set[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Altclr(pub u32);
impl Altclr {
  #[inline] pub fn udma_altclr_clr(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_udma_altclr_clr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
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
      if self.udma_altclr_clr(0) != 0 { try!(write!(f, " udma_altclr_clr[0]"))}
      if self.udma_altclr_clr(1) != 0 { try!(write!(f, " udma_altclr_clr[1]"))}
      if self.udma_altclr_clr(2) != 0 { try!(write!(f, " udma_altclr_clr[2]"))}
      if self.udma_altclr_clr(3) != 0 { try!(write!(f, " udma_altclr_clr[3]"))}
      if self.udma_altclr_clr(4) != 0 { try!(write!(f, " udma_altclr_clr[4]"))}
      if self.udma_altclr_clr(5) != 0 { try!(write!(f, " udma_altclr_clr[5]"))}
      if self.udma_altclr_clr(6) != 0 { try!(write!(f, " udma_altclr_clr[6]"))}
      if self.udma_altclr_clr(7) != 0 { try!(write!(f, " udma_altclr_clr[7]"))}
      if self.udma_altclr_clr(8) != 0 { try!(write!(f, " udma_altclr_clr[8]"))}
      if self.udma_altclr_clr(9) != 0 { try!(write!(f, " udma_altclr_clr[9]"))}
      if self.udma_altclr_clr(10) != 0 { try!(write!(f, " udma_altclr_clr[10]"))}
      if self.udma_altclr_clr(11) != 0 { try!(write!(f, " udma_altclr_clr[11]"))}
      if self.udma_altclr_clr(12) != 0 { try!(write!(f, " udma_altclr_clr[12]"))}
      if self.udma_altclr_clr(13) != 0 { try!(write!(f, " udma_altclr_clr[13]"))}
      if self.udma_altclr_clr(14) != 0 { try!(write!(f, " udma_altclr_clr[14]"))}
      if self.udma_altclr_clr(15) != 0 { try!(write!(f, " udma_altclr_clr[15]"))}
      if self.udma_altclr_clr(16) != 0 { try!(write!(f, " udma_altclr_clr[16]"))}
      if self.udma_altclr_clr(17) != 0 { try!(write!(f, " udma_altclr_clr[17]"))}
      if self.udma_altclr_clr(18) != 0 { try!(write!(f, " udma_altclr_clr[18]"))}
      if self.udma_altclr_clr(19) != 0 { try!(write!(f, " udma_altclr_clr[19]"))}
      if self.udma_altclr_clr(20) != 0 { try!(write!(f, " udma_altclr_clr[20]"))}
      if self.udma_altclr_clr(21) != 0 { try!(write!(f, " udma_altclr_clr[21]"))}
      if self.udma_altclr_clr(22) != 0 { try!(write!(f, " udma_altclr_clr[22]"))}
      if self.udma_altclr_clr(23) != 0 { try!(write!(f, " udma_altclr_clr[23]"))}
      if self.udma_altclr_clr(24) != 0 { try!(write!(f, " udma_altclr_clr[24]"))}
      if self.udma_altclr_clr(25) != 0 { try!(write!(f, " udma_altclr_clr[25]"))}
      if self.udma_altclr_clr(26) != 0 { try!(write!(f, " udma_altclr_clr[26]"))}
      if self.udma_altclr_clr(27) != 0 { try!(write!(f, " udma_altclr_clr[27]"))}
      if self.udma_altclr_clr(28) != 0 { try!(write!(f, " udma_altclr_clr[28]"))}
      if self.udma_altclr_clr(29) != 0 { try!(write!(f, " udma_altclr_clr[29]"))}
      if self.udma_altclr_clr(30) != 0 { try!(write!(f, " udma_altclr_clr[30]"))}
      if self.udma_altclr_clr(31) != 0 { try!(write!(f, " udma_altclr_clr[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Prioset(pub u32);
impl Prioset {
  #[inline] pub fn udma_prioset_set(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_udma_prioset_set(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
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
      if self.udma_prioset_set(0) != 0 { try!(write!(f, " udma_prioset_set[0]"))}
      if self.udma_prioset_set(1) != 0 { try!(write!(f, " udma_prioset_set[1]"))}
      if self.udma_prioset_set(2) != 0 { try!(write!(f, " udma_prioset_set[2]"))}
      if self.udma_prioset_set(3) != 0 { try!(write!(f, " udma_prioset_set[3]"))}
      if self.udma_prioset_set(4) != 0 { try!(write!(f, " udma_prioset_set[4]"))}
      if self.udma_prioset_set(5) != 0 { try!(write!(f, " udma_prioset_set[5]"))}
      if self.udma_prioset_set(6) != 0 { try!(write!(f, " udma_prioset_set[6]"))}
      if self.udma_prioset_set(7) != 0 { try!(write!(f, " udma_prioset_set[7]"))}
      if self.udma_prioset_set(8) != 0 { try!(write!(f, " udma_prioset_set[8]"))}
      if self.udma_prioset_set(9) != 0 { try!(write!(f, " udma_prioset_set[9]"))}
      if self.udma_prioset_set(10) != 0 { try!(write!(f, " udma_prioset_set[10]"))}
      if self.udma_prioset_set(11) != 0 { try!(write!(f, " udma_prioset_set[11]"))}
      if self.udma_prioset_set(12) != 0 { try!(write!(f, " udma_prioset_set[12]"))}
      if self.udma_prioset_set(13) != 0 { try!(write!(f, " udma_prioset_set[13]"))}
      if self.udma_prioset_set(14) != 0 { try!(write!(f, " udma_prioset_set[14]"))}
      if self.udma_prioset_set(15) != 0 { try!(write!(f, " udma_prioset_set[15]"))}
      if self.udma_prioset_set(16) != 0 { try!(write!(f, " udma_prioset_set[16]"))}
      if self.udma_prioset_set(17) != 0 { try!(write!(f, " udma_prioset_set[17]"))}
      if self.udma_prioset_set(18) != 0 { try!(write!(f, " udma_prioset_set[18]"))}
      if self.udma_prioset_set(19) != 0 { try!(write!(f, " udma_prioset_set[19]"))}
      if self.udma_prioset_set(20) != 0 { try!(write!(f, " udma_prioset_set[20]"))}
      if self.udma_prioset_set(21) != 0 { try!(write!(f, " udma_prioset_set[21]"))}
      if self.udma_prioset_set(22) != 0 { try!(write!(f, " udma_prioset_set[22]"))}
      if self.udma_prioset_set(23) != 0 { try!(write!(f, " udma_prioset_set[23]"))}
      if self.udma_prioset_set(24) != 0 { try!(write!(f, " udma_prioset_set[24]"))}
      if self.udma_prioset_set(25) != 0 { try!(write!(f, " udma_prioset_set[25]"))}
      if self.udma_prioset_set(26) != 0 { try!(write!(f, " udma_prioset_set[26]"))}
      if self.udma_prioset_set(27) != 0 { try!(write!(f, " udma_prioset_set[27]"))}
      if self.udma_prioset_set(28) != 0 { try!(write!(f, " udma_prioset_set[28]"))}
      if self.udma_prioset_set(29) != 0 { try!(write!(f, " udma_prioset_set[29]"))}
      if self.udma_prioset_set(30) != 0 { try!(write!(f, " udma_prioset_set[30]"))}
      if self.udma_prioset_set(31) != 0 { try!(write!(f, " udma_prioset_set[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Prioclr(pub u32);
impl Prioclr {
  #[inline] pub fn udma_prioclr_clr(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_udma_prioclr_clr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
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
      if self.udma_prioclr_clr(0) != 0 { try!(write!(f, " udma_prioclr_clr[0]"))}
      if self.udma_prioclr_clr(1) != 0 { try!(write!(f, " udma_prioclr_clr[1]"))}
      if self.udma_prioclr_clr(2) != 0 { try!(write!(f, " udma_prioclr_clr[2]"))}
      if self.udma_prioclr_clr(3) != 0 { try!(write!(f, " udma_prioclr_clr[3]"))}
      if self.udma_prioclr_clr(4) != 0 { try!(write!(f, " udma_prioclr_clr[4]"))}
      if self.udma_prioclr_clr(5) != 0 { try!(write!(f, " udma_prioclr_clr[5]"))}
      if self.udma_prioclr_clr(6) != 0 { try!(write!(f, " udma_prioclr_clr[6]"))}
      if self.udma_prioclr_clr(7) != 0 { try!(write!(f, " udma_prioclr_clr[7]"))}
      if self.udma_prioclr_clr(8) != 0 { try!(write!(f, " udma_prioclr_clr[8]"))}
      if self.udma_prioclr_clr(9) != 0 { try!(write!(f, " udma_prioclr_clr[9]"))}
      if self.udma_prioclr_clr(10) != 0 { try!(write!(f, " udma_prioclr_clr[10]"))}
      if self.udma_prioclr_clr(11) != 0 { try!(write!(f, " udma_prioclr_clr[11]"))}
      if self.udma_prioclr_clr(12) != 0 { try!(write!(f, " udma_prioclr_clr[12]"))}
      if self.udma_prioclr_clr(13) != 0 { try!(write!(f, " udma_prioclr_clr[13]"))}
      if self.udma_prioclr_clr(14) != 0 { try!(write!(f, " udma_prioclr_clr[14]"))}
      if self.udma_prioclr_clr(15) != 0 { try!(write!(f, " udma_prioclr_clr[15]"))}
      if self.udma_prioclr_clr(16) != 0 { try!(write!(f, " udma_prioclr_clr[16]"))}
      if self.udma_prioclr_clr(17) != 0 { try!(write!(f, " udma_prioclr_clr[17]"))}
      if self.udma_prioclr_clr(18) != 0 { try!(write!(f, " udma_prioclr_clr[18]"))}
      if self.udma_prioclr_clr(19) != 0 { try!(write!(f, " udma_prioclr_clr[19]"))}
      if self.udma_prioclr_clr(20) != 0 { try!(write!(f, " udma_prioclr_clr[20]"))}
      if self.udma_prioclr_clr(21) != 0 { try!(write!(f, " udma_prioclr_clr[21]"))}
      if self.udma_prioclr_clr(22) != 0 { try!(write!(f, " udma_prioclr_clr[22]"))}
      if self.udma_prioclr_clr(23) != 0 { try!(write!(f, " udma_prioclr_clr[23]"))}
      if self.udma_prioclr_clr(24) != 0 { try!(write!(f, " udma_prioclr_clr[24]"))}
      if self.udma_prioclr_clr(25) != 0 { try!(write!(f, " udma_prioclr_clr[25]"))}
      if self.udma_prioclr_clr(26) != 0 { try!(write!(f, " udma_prioclr_clr[26]"))}
      if self.udma_prioclr_clr(27) != 0 { try!(write!(f, " udma_prioclr_clr[27]"))}
      if self.udma_prioclr_clr(28) != 0 { try!(write!(f, " udma_prioclr_clr[28]"))}
      if self.udma_prioclr_clr(29) != 0 { try!(write!(f, " udma_prioclr_clr[29]"))}
      if self.udma_prioclr_clr(30) != 0 { try!(write!(f, " udma_prioclr_clr[30]"))}
      if self.udma_prioclr_clr(31) != 0 { try!(write!(f, " udma_prioclr_clr[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Errclr(pub u32);
impl Errclr {
  #[inline] pub fn udma_errclr_errclr(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_udma_errclr_errclr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
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
      if self.udma_errclr_errclr(0) != 0 { try!(write!(f, " udma_errclr_errclr[0]"))}
      if self.udma_errclr_errclr(1) != 0 { try!(write!(f, " udma_errclr_errclr[1]"))}
      if self.udma_errclr_errclr(2) != 0 { try!(write!(f, " udma_errclr_errclr[2]"))}
      if self.udma_errclr_errclr(3) != 0 { try!(write!(f, " udma_errclr_errclr[3]"))}
      if self.udma_errclr_errclr(4) != 0 { try!(write!(f, " udma_errclr_errclr[4]"))}
      if self.udma_errclr_errclr(5) != 0 { try!(write!(f, " udma_errclr_errclr[5]"))}
      if self.udma_errclr_errclr(6) != 0 { try!(write!(f, " udma_errclr_errclr[6]"))}
      if self.udma_errclr_errclr(7) != 0 { try!(write!(f, " udma_errclr_errclr[7]"))}
      if self.udma_errclr_errclr(8) != 0 { try!(write!(f, " udma_errclr_errclr[8]"))}
      if self.udma_errclr_errclr(9) != 0 { try!(write!(f, " udma_errclr_errclr[9]"))}
      if self.udma_errclr_errclr(10) != 0 { try!(write!(f, " udma_errclr_errclr[10]"))}
      if self.udma_errclr_errclr(11) != 0 { try!(write!(f, " udma_errclr_errclr[11]"))}
      if self.udma_errclr_errclr(12) != 0 { try!(write!(f, " udma_errclr_errclr[12]"))}
      if self.udma_errclr_errclr(13) != 0 { try!(write!(f, " udma_errclr_errclr[13]"))}
      if self.udma_errclr_errclr(14) != 0 { try!(write!(f, " udma_errclr_errclr[14]"))}
      if self.udma_errclr_errclr(15) != 0 { try!(write!(f, " udma_errclr_errclr[15]"))}
      if self.udma_errclr_errclr(16) != 0 { try!(write!(f, " udma_errclr_errclr[16]"))}
      if self.udma_errclr_errclr(17) != 0 { try!(write!(f, " udma_errclr_errclr[17]"))}
      if self.udma_errclr_errclr(18) != 0 { try!(write!(f, " udma_errclr_errclr[18]"))}
      if self.udma_errclr_errclr(19) != 0 { try!(write!(f, " udma_errclr_errclr[19]"))}
      if self.udma_errclr_errclr(20) != 0 { try!(write!(f, " udma_errclr_errclr[20]"))}
      if self.udma_errclr_errclr(21) != 0 { try!(write!(f, " udma_errclr_errclr[21]"))}
      if self.udma_errclr_errclr(22) != 0 { try!(write!(f, " udma_errclr_errclr[22]"))}
      if self.udma_errclr_errclr(23) != 0 { try!(write!(f, " udma_errclr_errclr[23]"))}
      if self.udma_errclr_errclr(24) != 0 { try!(write!(f, " udma_errclr_errclr[24]"))}
      if self.udma_errclr_errclr(25) != 0 { try!(write!(f, " udma_errclr_errclr[25]"))}
      if self.udma_errclr_errclr(26) != 0 { try!(write!(f, " udma_errclr_errclr[26]"))}
      if self.udma_errclr_errclr(27) != 0 { try!(write!(f, " udma_errclr_errclr[27]"))}
      if self.udma_errclr_errclr(28) != 0 { try!(write!(f, " udma_errclr_errclr[28]"))}
      if self.udma_errclr_errclr(29) != 0 { try!(write!(f, " udma_errclr_errclr[29]"))}
      if self.udma_errclr_errclr(30) != 0 { try!(write!(f, " udma_errclr_errclr[30]"))}
      if self.udma_errclr_errclr(31) != 0 { try!(write!(f, " udma_errclr_errclr[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Chasgn(pub u32);
impl Chasgn {
  #[inline] pub fn udma_chasgn(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_udma_chasgn(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
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
      if self.udma_chasgn(0) != 0 { try!(write!(f, " udma_chasgn[0]"))}
      if self.udma_chasgn(1) != 0 { try!(write!(f, " udma_chasgn[1]"))}
      if self.udma_chasgn(2) != 0 { try!(write!(f, " udma_chasgn[2]"))}
      if self.udma_chasgn(3) != 0 { try!(write!(f, " udma_chasgn[3]"))}
      if self.udma_chasgn(4) != 0 { try!(write!(f, " udma_chasgn[4]"))}
      if self.udma_chasgn(5) != 0 { try!(write!(f, " udma_chasgn[5]"))}
      if self.udma_chasgn(6) != 0 { try!(write!(f, " udma_chasgn[6]"))}
      if self.udma_chasgn(7) != 0 { try!(write!(f, " udma_chasgn[7]"))}
      if self.udma_chasgn(8) != 0 { try!(write!(f, " udma_chasgn[8]"))}
      if self.udma_chasgn(9) != 0 { try!(write!(f, " udma_chasgn[9]"))}
      if self.udma_chasgn(10) != 0 { try!(write!(f, " udma_chasgn[10]"))}
      if self.udma_chasgn(11) != 0 { try!(write!(f, " udma_chasgn[11]"))}
      if self.udma_chasgn(12) != 0 { try!(write!(f, " udma_chasgn[12]"))}
      if self.udma_chasgn(13) != 0 { try!(write!(f, " udma_chasgn[13]"))}
      if self.udma_chasgn(14) != 0 { try!(write!(f, " udma_chasgn[14]"))}
      if self.udma_chasgn(15) != 0 { try!(write!(f, " udma_chasgn[15]"))}
      if self.udma_chasgn(16) != 0 { try!(write!(f, " udma_chasgn[16]"))}
      if self.udma_chasgn(17) != 0 { try!(write!(f, " udma_chasgn[17]"))}
      if self.udma_chasgn(18) != 0 { try!(write!(f, " udma_chasgn[18]"))}
      if self.udma_chasgn(19) != 0 { try!(write!(f, " udma_chasgn[19]"))}
      if self.udma_chasgn(20) != 0 { try!(write!(f, " udma_chasgn[20]"))}
      if self.udma_chasgn(21) != 0 { try!(write!(f, " udma_chasgn[21]"))}
      if self.udma_chasgn(22) != 0 { try!(write!(f, " udma_chasgn[22]"))}
      if self.udma_chasgn(23) != 0 { try!(write!(f, " udma_chasgn[23]"))}
      if self.udma_chasgn(24) != 0 { try!(write!(f, " udma_chasgn[24]"))}
      if self.udma_chasgn(25) != 0 { try!(write!(f, " udma_chasgn[25]"))}
      if self.udma_chasgn(26) != 0 { try!(write!(f, " udma_chasgn[26]"))}
      if self.udma_chasgn(27) != 0 { try!(write!(f, " udma_chasgn[27]"))}
      if self.udma_chasgn(28) != 0 { try!(write!(f, " udma_chasgn[28]"))}
      if self.udma_chasgn(29) != 0 { try!(write!(f, " udma_chasgn[29]"))}
      if self.udma_chasgn(30) != 0 { try!(write!(f, " udma_chasgn[30]"))}
      if self.udma_chasgn(31) != 0 { try!(write!(f, " udma_chasgn[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Chmap0(pub u32);
impl Chmap0 {
  #[inline] pub fn udma_chmap0_chsel(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + (index << 2);
     ((self.0 as u32) >> shift) & 0xf // [3:0]
  }
  #[inline] pub fn set_udma_chmap0_chsel(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0xf) == 0);
     let shift: usize = 0 + (index << 2);
     self.0 &= !(0xf << shift);
     self.0 |= value << shift;
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
      if self.udma_chmap0_chsel(0) != 0 { try!(write!(f, " udma_chmap0_chsel[0]=0x{:x}", self.udma_chmap0_chsel(0)))}
      if self.udma_chmap0_chsel(1) != 0 { try!(write!(f, " udma_chmap0_chsel[1]=0x{:x}", self.udma_chmap0_chsel(1)))}
      if self.udma_chmap0_chsel(2) != 0 { try!(write!(f, " udma_chmap0_chsel[2]=0x{:x}", self.udma_chmap0_chsel(2)))}
      if self.udma_chmap0_chsel(3) != 0 { try!(write!(f, " udma_chmap0_chsel[3]=0x{:x}", self.udma_chmap0_chsel(3)))}
      if self.udma_chmap0_chsel(4) != 0 { try!(write!(f, " udma_chmap0_chsel[4]=0x{:x}", self.udma_chmap0_chsel(4)))}
      if self.udma_chmap0_chsel(5) != 0 { try!(write!(f, " udma_chmap0_chsel[5]=0x{:x}", self.udma_chmap0_chsel(5)))}
      if self.udma_chmap0_chsel(6) != 0 { try!(write!(f, " udma_chmap0_chsel[6]=0x{:x}", self.udma_chmap0_chsel(6)))}
      if self.udma_chmap0_chsel(7) != 0 { try!(write!(f, " udma_chmap0_chsel[7]=0x{:x}", self.udma_chmap0_chsel(7)))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Chmap1(pub u32);
impl Chmap1 {
  #[inline] pub fn udma_chmap1_chsel(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + (index << 2);
     ((self.0 as u32) >> shift) & 0xf // [3:0]
  }
  #[inline] pub fn set_udma_chmap1_chsel(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0xf) == 0);
     let shift: usize = 0 + (index << 2);
     self.0 &= !(0xf << shift);
     self.0 |= value << shift;
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
      if self.udma_chmap1_chsel(0) != 0 { try!(write!(f, " udma_chmap1_chsel[0]=0x{:x}", self.udma_chmap1_chsel(0)))}
      if self.udma_chmap1_chsel(1) != 0 { try!(write!(f, " udma_chmap1_chsel[1]=0x{:x}", self.udma_chmap1_chsel(1)))}
      if self.udma_chmap1_chsel(2) != 0 { try!(write!(f, " udma_chmap1_chsel[2]=0x{:x}", self.udma_chmap1_chsel(2)))}
      if self.udma_chmap1_chsel(3) != 0 { try!(write!(f, " udma_chmap1_chsel[3]=0x{:x}", self.udma_chmap1_chsel(3)))}
      if self.udma_chmap1_chsel(4) != 0 { try!(write!(f, " udma_chmap1_chsel[4]=0x{:x}", self.udma_chmap1_chsel(4)))}
      if self.udma_chmap1_chsel(5) != 0 { try!(write!(f, " udma_chmap1_chsel[5]=0x{:x}", self.udma_chmap1_chsel(5)))}
      if self.udma_chmap1_chsel(6) != 0 { try!(write!(f, " udma_chmap1_chsel[6]=0x{:x}", self.udma_chmap1_chsel(6)))}
      if self.udma_chmap1_chsel(7) != 0 { try!(write!(f, " udma_chmap1_chsel[7]=0x{:x}", self.udma_chmap1_chsel(7)))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Chmap2(pub u32);
impl Chmap2 {
  #[inline] pub fn udma_chmap2_chsel(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + (index << 2);
     ((self.0 as u32) >> shift) & 0xf // [3:0]
  }
  #[inline] pub fn set_udma_chmap2_chsel(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0xf) == 0);
     let shift: usize = 0 + (index << 2);
     self.0 &= !(0xf << shift);
     self.0 |= value << shift;
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
      if self.udma_chmap2_chsel(0) != 0 { try!(write!(f, " udma_chmap2_chsel[0]=0x{:x}", self.udma_chmap2_chsel(0)))}
      if self.udma_chmap2_chsel(1) != 0 { try!(write!(f, " udma_chmap2_chsel[1]=0x{:x}", self.udma_chmap2_chsel(1)))}
      if self.udma_chmap2_chsel(2) != 0 { try!(write!(f, " udma_chmap2_chsel[2]=0x{:x}", self.udma_chmap2_chsel(2)))}
      if self.udma_chmap2_chsel(3) != 0 { try!(write!(f, " udma_chmap2_chsel[3]=0x{:x}", self.udma_chmap2_chsel(3)))}
      if self.udma_chmap2_chsel(4) != 0 { try!(write!(f, " udma_chmap2_chsel[4]=0x{:x}", self.udma_chmap2_chsel(4)))}
      if self.udma_chmap2_chsel(5) != 0 { try!(write!(f, " udma_chmap2_chsel[5]=0x{:x}", self.udma_chmap2_chsel(5)))}
      if self.udma_chmap2_chsel(6) != 0 { try!(write!(f, " udma_chmap2_chsel[6]=0x{:x}", self.udma_chmap2_chsel(6)))}
      if self.udma_chmap2_chsel(7) != 0 { try!(write!(f, " udma_chmap2_chsel[7]=0x{:x}", self.udma_chmap2_chsel(7)))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Chmap3(pub u32);
impl Chmap3 {
  #[inline] pub fn udma_chmap3_chsel(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + (index << 2);
     ((self.0 as u32) >> shift) & 0xf // [3:0]
  }
  #[inline] pub fn set_udma_chmap3_chsel(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0xf) == 0);
     let shift: usize = 0 + (index << 2);
     self.0 &= !(0xf << shift);
     self.0 |= value << shift;
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
      if self.udma_chmap3_chsel(0) != 0 { try!(write!(f, " udma_chmap3_chsel[0]=0x{:x}", self.udma_chmap3_chsel(0)))}
      if self.udma_chmap3_chsel(1) != 0 { try!(write!(f, " udma_chmap3_chsel[1]=0x{:x}", self.udma_chmap3_chsel(1)))}
      if self.udma_chmap3_chsel(2) != 0 { try!(write!(f, " udma_chmap3_chsel[2]=0x{:x}", self.udma_chmap3_chsel(2)))}
      if self.udma_chmap3_chsel(3) != 0 { try!(write!(f, " udma_chmap3_chsel[3]=0x{:x}", self.udma_chmap3_chsel(3)))}
      if self.udma_chmap3_chsel(4) != 0 { try!(write!(f, " udma_chmap3_chsel[4]=0x{:x}", self.udma_chmap3_chsel(4)))}
      if self.udma_chmap3_chsel(5) != 0 { try!(write!(f, " udma_chmap3_chsel[5]=0x{:x}", self.udma_chmap3_chsel(5)))}
      if self.udma_chmap3_chsel(6) != 0 { try!(write!(f, " udma_chmap3_chsel[6]=0x{:x}", self.udma_chmap3_chsel(6)))}
      if self.udma_chmap3_chsel(7) != 0 { try!(write!(f, " udma_chmap3_chsel[7]=0x{:x}", self.udma_chmap3_chsel(7)))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct Ccs(pub [u8; 16]);

impl Ccs {
   #[inline] pub fn srcendp(&self) -> Srcendp { 
      unsafe {
         Srcendp(::core::ptr::read_volatile(self.0.as_ptr().offset(0x0) as *const u32))
      }
   }
   #[inline] pub fn set_srcendp(&mut self, value: Srcendp) -> &mut Self {
      unsafe {
         ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x0) as *mut u32, value.0);
      }
      self
  }
   #[inline] pub fn with_srcendp<F: FnOnce(Srcendp) -> Srcendp>(&mut self, f: F) -> &mut Self {
      let tmp = self.srcendp();
      self.set_srcendp(f(tmp))
   }

   #[inline] pub fn dstendp(&self) -> Dstendp { 
      unsafe {
         Dstendp(::core::ptr::read_volatile(self.0.as_ptr().offset(0x0) as *const u32))
      }
   }
   #[inline] pub fn set_dstendp(&mut self, value: Dstendp) -> &mut Self {
      unsafe {
         ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x0) as *mut u32, value.0);
      }
      self
  }
   #[inline] pub fn with_dstendp<F: FnOnce(Dstendp) -> Dstendp>(&mut self, f: F) -> &mut Self {
      let tmp = self.dstendp();
      self.set_dstendp(f(tmp))
   }

   #[inline] pub fn chctl(&self) -> Chctl { 
      unsafe {
         Chctl(::core::ptr::read_volatile(self.0.as_ptr().offset(0x0) as *const u32))
      }
   }
   #[inline] pub fn set_chctl(&mut self, value: Chctl) -> &mut Self {
      unsafe {
         ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x0) as *mut u32, value.0);
      }
      self
  }
   #[inline] pub fn with_chctl<F: FnOnce(Chctl) -> Chctl>(&mut self, f: F) -> &mut Self {
      let tmp = self.chctl();
      self.set_chctl(f(tmp))
   }

}
#[derive(PartialEq, Eq)]
pub struct Srcendp(pub u32);
impl Srcendp {
  #[inline] pub fn addr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_addr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Srcendp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Srcendp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dstendp(pub u32);
impl Dstendp {
  #[inline] pub fn addr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_addr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dstendp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dstendp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Chctl(pub u32);
impl Chctl {
  #[inline] pub fn dstinc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x3 // [31:30]
  }
  #[inline] pub fn set_dstinc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 30);
     self.0 |= value << 30;
     self
  }

  #[inline] pub fn dstsize(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x3 // [29:28]
  }
  #[inline] pub fn set_dstsize(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 28);
     self.0 |= value << 28;
     self
  }

  #[inline] pub fn srcinc(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x3 // [27:26]
  }
  #[inline] pub fn set_srcinc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 26);
     self.0 |= value << 26;
     self
  }

  #[inline] pub fn srcsize(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x3 // [25:24]
  }
  #[inline] pub fn set_srcsize(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 24);
     self.0 |= value << 24;
     self
  }

  #[inline] pub fn dstproto(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  #[inline] pub fn set_dstproto(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  #[inline] pub fn srcproto(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline] pub fn set_srcproto(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  #[inline] pub fn arbsize(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0xf // [17:14]
  }
  #[inline] pub fn set_arbsize(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 14);
     self.0 |= value << 14;
     self
  }

  #[inline] pub fn xfersize(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3ff // [13:4]
  }
  #[inline] pub fn set_xfersize(mut self, value: u32) -> Self {
     assert!((value & !0x3ff) == 0);
     self.0 &= !(0x3ff << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn nxtuseburst(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_nxtuseburst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn xfermode(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
  #[inline] pub fn set_xfermode(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Chctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dstinc() != 0 { try!(write!(f, " dstinc=0x{:x}", self.dstinc()))}
      if self.dstsize() != 0 { try!(write!(f, " dstsize=0x{:x}", self.dstsize()))}
      if self.srcinc() != 0 { try!(write!(f, " srcinc=0x{:x}", self.srcinc()))}
      if self.srcsize() != 0 { try!(write!(f, " srcsize=0x{:x}", self.srcsize()))}
      if self.dstproto() != 0 { try!(write!(f, " dstproto"))}
      if self.srcproto() != 0 { try!(write!(f, " srcproto"))}
      if self.arbsize() != 0 { try!(write!(f, " arbsize=0x{:x}", self.arbsize()))}
      if self.xfersize() != 0 { try!(write!(f, " xfersize=0x{:x}", self.xfersize()))}
      if self.nxtuseburst() != 0 { try!(write!(f, " nxtuseburst"))}
      if self.xfermode() != 0 { try!(write!(f, " xfermode=0x{:x}", self.xfermode()))}
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

