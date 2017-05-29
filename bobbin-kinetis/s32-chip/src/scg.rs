pub const SCG: Scg = Scg(0x40064000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scg(pub u32);

impl Scg {
  pub unsafe fn verid(&self) -> Verid { 
     Verid(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
  }

  pub unsafe fn param(&self) -> Param { 
     Param(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
  }

  pub unsafe fn csr(&self) -> Csr { 
     Csr(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
  }

  pub unsafe fn rccr(&self) -> Rccr { 
     Rccr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
  }
  pub unsafe fn set_rccr(&mut self, value: Rccr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
  }
  pub unsafe fn with_rccr<F: FnOnce(Rccr) -> Rccr>(&mut self, f: F) {
     let tmp = self.rccr();
     self.set_rccr(f(tmp))
  }

  pub unsafe fn vccr(&self) -> Vccr { 
     Vccr(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
  }
  pub unsafe fn set_vccr(&mut self, value: Vccr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
  }
  pub unsafe fn with_vccr<F: FnOnce(Vccr) -> Vccr>(&mut self, f: F) {
     let tmp = self.vccr();
     self.set_vccr(f(tmp))
  }

  pub unsafe fn hccr(&self) -> Hccr { 
     Hccr(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
  }
  pub unsafe fn set_hccr(&mut self, value: Hccr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
  }
  pub unsafe fn with_hccr<F: FnOnce(Hccr) -> Hccr>(&mut self, f: F) {
     let tmp = self.hccr();
     self.set_hccr(f(tmp))
  }

  pub unsafe fn clkoutcnfg(&self) -> Clkoutcnfg { 
     Clkoutcnfg(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
  }
  pub unsafe fn set_clkoutcnfg(&mut self, value: Clkoutcnfg) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
  }
  pub unsafe fn with_clkoutcnfg<F: FnOnce(Clkoutcnfg) -> Clkoutcnfg>(&mut self, f: F) {
     let tmp = self.clkoutcnfg();
     self.set_clkoutcnfg(f(tmp))
  }

  pub unsafe fn sosccsr(&self) -> Sosccsr { 
     Sosccsr(::core::ptr::read_volatile(((self.0 as usize) + 0x100) as *const u32))
  }
  pub unsafe fn set_sosccsr(&mut self, value: Sosccsr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x100) as *mut u32, value.0);
  }
  pub unsafe fn with_sosccsr<F: FnOnce(Sosccsr) -> Sosccsr>(&mut self, f: F) {
     let tmp = self.sosccsr();
     self.set_sosccsr(f(tmp))
  }

  pub unsafe fn soscdiv(&self) -> Soscdiv { 
     Soscdiv(::core::ptr::read_volatile(((self.0 as usize) + 0x104) as *const u32))
  }
  pub unsafe fn set_soscdiv(&mut self, value: Soscdiv) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x104) as *mut u32, value.0);
  }
  pub unsafe fn with_soscdiv<F: FnOnce(Soscdiv) -> Soscdiv>(&mut self, f: F) {
     let tmp = self.soscdiv();
     self.set_soscdiv(f(tmp))
  }

  pub unsafe fn sosccfg(&self) -> Sosccfg { 
     Sosccfg(::core::ptr::read_volatile(((self.0 as usize) + 0x108) as *const u32))
  }
  pub unsafe fn set_sosccfg(&mut self, value: Sosccfg) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x108) as *mut u32, value.0);
  }
  pub unsafe fn with_sosccfg<F: FnOnce(Sosccfg) -> Sosccfg>(&mut self, f: F) {
     let tmp = self.sosccfg();
     self.set_sosccfg(f(tmp))
  }

  pub unsafe fn sirccsr(&self) -> Sirccsr { 
     Sirccsr(::core::ptr::read_volatile(((self.0 as usize) + 0x200) as *const u32))
  }
  pub unsafe fn set_sirccsr(&mut self, value: Sirccsr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x200) as *mut u32, value.0);
  }
  pub unsafe fn with_sirccsr<F: FnOnce(Sirccsr) -> Sirccsr>(&mut self, f: F) {
     let tmp = self.sirccsr();
     self.set_sirccsr(f(tmp))
  }

  pub unsafe fn sircdiv(&self) -> Sircdiv { 
     Sircdiv(::core::ptr::read_volatile(((self.0 as usize) + 0x204) as *const u32))
  }
  pub unsafe fn set_sircdiv(&mut self, value: Sircdiv) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x204) as *mut u32, value.0);
  }
  pub unsafe fn with_sircdiv<F: FnOnce(Sircdiv) -> Sircdiv>(&mut self, f: F) {
     let tmp = self.sircdiv();
     self.set_sircdiv(f(tmp))
  }

  pub unsafe fn sirccfg(&self) -> Sirccfg { 
     Sirccfg(::core::ptr::read_volatile(((self.0 as usize) + 0x208) as *const u32))
  }
  pub unsafe fn set_sirccfg(&mut self, value: Sirccfg) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x208) as *mut u32, value.0);
  }
  pub unsafe fn with_sirccfg<F: FnOnce(Sirccfg) -> Sirccfg>(&mut self, f: F) {
     let tmp = self.sirccfg();
     self.set_sirccfg(f(tmp))
  }

  pub unsafe fn firccsr(&self) -> Firccsr { 
     Firccsr(::core::ptr::read_volatile(((self.0 as usize) + 0x300) as *const u32))
  }
  pub unsafe fn set_firccsr(&mut self, value: Firccsr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x300) as *mut u32, value.0);
  }
  pub unsafe fn with_firccsr<F: FnOnce(Firccsr) -> Firccsr>(&mut self, f: F) {
     let tmp = self.firccsr();
     self.set_firccsr(f(tmp))
  }

  pub unsafe fn fircdiv(&self) -> Fircdiv { 
     Fircdiv(::core::ptr::read_volatile(((self.0 as usize) + 0x304) as *const u32))
  }
  pub unsafe fn set_fircdiv(&mut self, value: Fircdiv) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x304) as *mut u32, value.0);
  }
  pub unsafe fn with_fircdiv<F: FnOnce(Fircdiv) -> Fircdiv>(&mut self, f: F) {
     let tmp = self.fircdiv();
     self.set_fircdiv(f(tmp))
  }

  pub unsafe fn firccfg(&self) -> Firccfg { 
     Firccfg(::core::ptr::read_volatile(((self.0 as usize) + 0x308) as *const u32))
  }
  pub unsafe fn set_firccfg(&mut self, value: Firccfg) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x308) as *mut u32, value.0);
  }
  pub unsafe fn with_firccfg<F: FnOnce(Firccfg) -> Firccfg>(&mut self, f: F) {
     let tmp = self.firccfg();
     self.set_firccfg(f(tmp))
  }

  pub unsafe fn spllcsr(&self) -> Spllcsr { 
     Spllcsr(::core::ptr::read_volatile(((self.0 as usize) + 0x600) as *const u32))
  }
  pub unsafe fn set_spllcsr(&mut self, value: Spllcsr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x600) as *mut u32, value.0);
  }
  pub unsafe fn with_spllcsr<F: FnOnce(Spllcsr) -> Spllcsr>(&mut self, f: F) {
     let tmp = self.spllcsr();
     self.set_spllcsr(f(tmp))
  }

  pub unsafe fn splldiv(&self) -> Splldiv { 
     Splldiv(::core::ptr::read_volatile(((self.0 as usize) + 0x604) as *const u32))
  }
  pub unsafe fn set_splldiv(&mut self, value: Splldiv) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x604) as *mut u32, value.0);
  }
  pub unsafe fn with_splldiv<F: FnOnce(Splldiv) -> Splldiv>(&mut self, f: F) {
     let tmp = self.splldiv();
     self.set_splldiv(f(tmp))
  }

  pub unsafe fn spllcfg(&self) -> Spllcfg { 
     Spllcfg(::core::ptr::read_volatile(((self.0 as usize) + 0x608) as *const u32))
  }
  pub unsafe fn set_spllcfg(&mut self, value: Spllcfg) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x608) as *mut u32, value.0);
  }
  pub unsafe fn with_spllcfg<F: FnOnce(Spllcfg) -> Spllcfg>(&mut self, f: F) {
     let tmp = self.spllcfg();
     self.set_spllcfg(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Verid(pub u32);

impl Verid {
  pub fn version(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_version(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Verid {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Verid {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Param(pub u32);

impl Param {
  pub fn clkpres(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_clkpres(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn divpres(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1f // [31:27]
  }
  pub fn set_divpres(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 27);
     self.0 |= value << 27;
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
      if self.clkpres() != 0 { try!(write!(f, " clkpres=0x{:x}", self.clkpres()))}
      if self.divpres() != 0 { try!(write!(f, " divpres=0x{:x}", self.divpres()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Csr(pub u32);

impl Csr {
  pub fn divslow(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  pub fn set_divslow(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  pub fn divbus(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
  pub fn set_divbus(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

  pub fn divcore(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  pub fn set_divcore(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  pub fn scs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  pub fn set_scs(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

}

impl ::core::fmt::Display for Csr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Csr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.divslow() != 0 { try!(write!(f, " divslow=0x{:x}", self.divslow()))}
      if self.divbus() != 0 { try!(write!(f, " divbus=0x{:x}", self.divbus()))}
      if self.divcore() != 0 { try!(write!(f, " divcore=0x{:x}", self.divcore()))}
      if self.scs() != 0 { try!(write!(f, " scs=0x{:x}", self.scs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rccr(pub u32);

impl Rccr {
  pub fn divslow(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  pub fn set_divslow(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  pub fn divbus(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
  pub fn set_divbus(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

  pub fn divcore(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  pub fn set_divcore(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  pub fn scs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  pub fn set_scs(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

}

impl ::core::fmt::Display for Rccr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rccr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.divslow() != 0 { try!(write!(f, " divslow=0x{:x}", self.divslow()))}
      if self.divbus() != 0 { try!(write!(f, " divbus=0x{:x}", self.divbus()))}
      if self.divcore() != 0 { try!(write!(f, " divcore=0x{:x}", self.divcore()))}
      if self.scs() != 0 { try!(write!(f, " scs=0x{:x}", self.scs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Vccr(pub u32);

impl Vccr {
  pub fn divslow(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  pub fn set_divslow(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  pub fn divbus(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
  pub fn set_divbus(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

  pub fn divcore(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  pub fn set_divcore(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  pub fn scs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  pub fn set_scs(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

}

impl ::core::fmt::Display for Vccr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Vccr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.divslow() != 0 { try!(write!(f, " divslow=0x{:x}", self.divslow()))}
      if self.divbus() != 0 { try!(write!(f, " divbus=0x{:x}", self.divbus()))}
      if self.divcore() != 0 { try!(write!(f, " divcore=0x{:x}", self.divcore()))}
      if self.scs() != 0 { try!(write!(f, " scs=0x{:x}", self.scs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Hccr(pub u32);

impl Hccr {
  pub fn divslow(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  pub fn set_divslow(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  pub fn divbus(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
  pub fn set_divbus(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

  pub fn divcore(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  pub fn set_divcore(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  pub fn scs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  pub fn set_scs(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

}

impl ::core::fmt::Display for Hccr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Hccr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.divslow() != 0 { try!(write!(f, " divslow=0x{:x}", self.divslow()))}
      if self.divbus() != 0 { try!(write!(f, " divbus=0x{:x}", self.divbus()))}
      if self.divcore() != 0 { try!(write!(f, " divcore=0x{:x}", self.divcore()))}
      if self.scs() != 0 { try!(write!(f, " scs=0x{:x}", self.scs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Clkoutcnfg(pub u32);

impl Clkoutcnfg {
  pub fn clkoutsel(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  pub fn set_clkoutsel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

}

impl ::core::fmt::Display for Clkoutcnfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Clkoutcnfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clkoutsel() != 0 { try!(write!(f, " clkoutsel=0x{:x}", self.clkoutsel()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Sosccsr(pub u32);

impl Sosccsr {
  pub fn soscen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_soscen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn sosccm(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_sosccm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn sosccmre(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_sosccmre(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn lk(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_lk(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn soscvld(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_soscvld(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn soscsel(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_soscsel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn soscerr(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_soscerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

}

impl ::core::fmt::Display for Sosccsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sosccsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.soscen() != 0 { try!(write!(f, " soscen"))}
      if self.sosccm() != 0 { try!(write!(f, " sosccm"))}
      if self.sosccmre() != 0 { try!(write!(f, " sosccmre"))}
      if self.lk() != 0 { try!(write!(f, " lk"))}
      if self.soscvld() != 0 { try!(write!(f, " soscvld"))}
      if self.soscsel() != 0 { try!(write!(f, " soscsel"))}
      if self.soscerr() != 0 { try!(write!(f, " soscerr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Soscdiv(pub u32);

impl Soscdiv {
  pub fn soscdiv1(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
  pub fn set_soscdiv1(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn soscdiv2(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x7 // [10:8]
  }
  pub fn set_soscdiv2(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

}

impl ::core::fmt::Display for Soscdiv {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Soscdiv {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.soscdiv1() != 0 { try!(write!(f, " soscdiv1=0x{:x}", self.soscdiv1()))}
      if self.soscdiv2() != 0 { try!(write!(f, " soscdiv2=0x{:x}", self.soscdiv2()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Sosccfg(pub u32);

impl Sosccfg {
  pub fn erefs(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_erefs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn hgo(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_hgo(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn range(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  pub fn set_range(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

}

impl ::core::fmt::Display for Sosccfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sosccfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.erefs() != 0 { try!(write!(f, " erefs"))}
      if self.hgo() != 0 { try!(write!(f, " hgo"))}
      if self.range() != 0 { try!(write!(f, " range=0x{:x}", self.range()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Sirccsr(pub u32);

impl Sirccsr {
  pub fn sircen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_sircen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn sircsten(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_sircsten(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn sirclpen(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_sirclpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn lk(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_lk(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn sircvld(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_sircvld(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn sircsel(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_sircsel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

}

impl ::core::fmt::Display for Sirccsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sirccsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sircen() != 0 { try!(write!(f, " sircen"))}
      if self.sircsten() != 0 { try!(write!(f, " sircsten"))}
      if self.sirclpen() != 0 { try!(write!(f, " sirclpen"))}
      if self.lk() != 0 { try!(write!(f, " lk"))}
      if self.sircvld() != 0 { try!(write!(f, " sircvld"))}
      if self.sircsel() != 0 { try!(write!(f, " sircsel"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Sircdiv(pub u32);

impl Sircdiv {
  pub fn sircdiv1(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
  pub fn set_sircdiv1(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn sircdiv2(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x7 // [10:8]
  }
  pub fn set_sircdiv2(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

}

impl ::core::fmt::Display for Sircdiv {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sircdiv {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sircdiv1() != 0 { try!(write!(f, " sircdiv1=0x{:x}", self.sircdiv1()))}
      if self.sircdiv2() != 0 { try!(write!(f, " sircdiv2=0x{:x}", self.sircdiv2()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Sirccfg(pub u32);

impl Sirccfg {
  pub fn range(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_range(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Sirccfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sirccfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.range() != 0 { try!(write!(f, " range"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Firccsr(pub u32);

impl Firccsr {
  pub fn fircen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_fircen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn fircregoff(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_fircregoff(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn lk(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_lk(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn fircvld(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_fircvld(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn fircsel(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_fircsel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn fircerr(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_fircerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

}

impl ::core::fmt::Display for Firccsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Firccsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fircen() != 0 { try!(write!(f, " fircen"))}
      if self.fircregoff() != 0 { try!(write!(f, " fircregoff"))}
      if self.lk() != 0 { try!(write!(f, " lk"))}
      if self.fircvld() != 0 { try!(write!(f, " fircvld"))}
      if self.fircsel() != 0 { try!(write!(f, " fircsel"))}
      if self.fircerr() != 0 { try!(write!(f, " fircerr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Fircdiv(pub u32);

impl Fircdiv {
  pub fn fircdiv1(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
  pub fn set_fircdiv1(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn fircdiv2(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x7 // [10:8]
  }
  pub fn set_fircdiv2(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

}

impl ::core::fmt::Display for Fircdiv {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Fircdiv {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fircdiv1() != 0 { try!(write!(f, " fircdiv1=0x{:x}", self.fircdiv1()))}
      if self.fircdiv2() != 0 { try!(write!(f, " fircdiv2=0x{:x}", self.fircdiv2()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Firccfg(pub u32);

impl Firccfg {
  pub fn range(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  pub fn set_range(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Firccfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Firccfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.range() != 0 { try!(write!(f, " range=0x{:x}", self.range()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Spllcsr(pub u32);

impl Spllcsr {
  pub fn spllen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_spllen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn spllcm(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_spllcm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn spllcmre(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_spllcmre(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn lk(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_lk(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn spllvld(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_spllvld(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn spllsel(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_spllsel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn spllerr(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_spllerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

}

impl ::core::fmt::Display for Spllcsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Spllcsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.spllen() != 0 { try!(write!(f, " spllen"))}
      if self.spllcm() != 0 { try!(write!(f, " spllcm"))}
      if self.spllcmre() != 0 { try!(write!(f, " spllcmre"))}
      if self.lk() != 0 { try!(write!(f, " lk"))}
      if self.spllvld() != 0 { try!(write!(f, " spllvld"))}
      if self.spllsel() != 0 { try!(write!(f, " spllsel"))}
      if self.spllerr() != 0 { try!(write!(f, " spllerr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Splldiv(pub u32);

impl Splldiv {
  pub fn splldiv1(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
  pub fn set_splldiv1(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn splldiv2(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x7 // [10:8]
  }
  pub fn set_splldiv2(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

}

impl ::core::fmt::Display for Splldiv {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Splldiv {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.splldiv1() != 0 { try!(write!(f, " splldiv1=0x{:x}", self.splldiv1()))}
      if self.splldiv2() != 0 { try!(write!(f, " splldiv2=0x{:x}", self.splldiv2()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Spllcfg(pub u32);

impl Spllcfg {
  pub fn prediv(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x7 // [10:8]
  }
  pub fn set_prediv(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn mult(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1f // [20:16]
  }
  pub fn set_mult(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 16);
     self.0 |= value << 16;
     self
  }

}

impl ::core::fmt::Display for Spllcfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Spllcfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.prediv() != 0 { try!(write!(f, " prediv=0x{:x}", self.prediv()))}
      if self.mult() != 0 { try!(write!(f, " mult=0x{:x}", self.mult()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

