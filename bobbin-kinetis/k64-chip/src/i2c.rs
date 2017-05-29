pub const I2C0: I2c = I2c(0x40066000);
pub const I2C1: I2c = I2c(0x40067000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct I2c(pub u32);

impl I2c {
  pub unsafe fn a1(&self) -> A1 { 
     A1(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u8))
  }
  pub unsafe fn set_a1(&mut self, value: A1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u8, value.0);
  }
  pub unsafe fn with_a1<F: FnOnce(A1) -> A1>(&mut self, f: F) {
     let tmp = self.a1();
     self.set_a1(f(tmp))
  }

  pub unsafe fn f(&self) -> F { 
     F(::core::ptr::read_volatile(((self.0 as usize) + 0x1) as *const u8))
  }
  pub unsafe fn set_f(&mut self, value: F) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1) as *mut u8, value.0);
  }
  pub unsafe fn with_f<_F: FnOnce(F) -> F>(&mut self, f: _F) {
     let tmp = self.f();
     self.set_f(f(tmp))
  }

  pub unsafe fn c1(&self) -> C1 { 
     C1(::core::ptr::read_volatile(((self.0 as usize) + 0x2) as *const u8))
  }
  pub unsafe fn set_c1(&mut self, value: C1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x2) as *mut u8, value.0);
  }
  pub unsafe fn with_c1<F: FnOnce(C1) -> C1>(&mut self, f: F) {
     let tmp = self.c1();
     self.set_c1(f(tmp))
  }

  pub unsafe fn s(&self) -> S { 
     S(::core::ptr::read_volatile(((self.0 as usize) + 0x3) as *const u8))
  }
  pub unsafe fn set_s(&mut self, value: S) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x3) as *mut u8, value.0);
  }
  pub unsafe fn with_s<F: FnOnce(S) -> S>(&mut self, f: F) {
     let tmp = self.s();
     self.set_s(f(tmp))
  }

  pub unsafe fn d(&self) -> D { 
     D(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u8))
  }
  pub unsafe fn set_d(&mut self, value: D) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u8, value.0);
  }
  pub unsafe fn with_d<F: FnOnce(D) -> D>(&mut self, f: F) {
     let tmp = self.d();
     self.set_d(f(tmp))
  }

  pub unsafe fn c2(&self) -> C2 { 
     C2(::core::ptr::read_volatile(((self.0 as usize) + 0x5) as *const u8))
  }
  pub unsafe fn set_c2(&mut self, value: C2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x5) as *mut u8, value.0);
  }
  pub unsafe fn with_c2<F: FnOnce(C2) -> C2>(&mut self, f: F) {
     let tmp = self.c2();
     self.set_c2(f(tmp))
  }

  pub unsafe fn flt(&self) -> Flt { 
     Flt(::core::ptr::read_volatile(((self.0 as usize) + 0x6) as *const u8))
  }
  pub unsafe fn set_flt(&mut self, value: Flt) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x6) as *mut u8, value.0);
  }
  pub unsafe fn with_flt<F: FnOnce(Flt) -> Flt>(&mut self, f: F) {
     let tmp = self.flt();
     self.set_flt(f(tmp))
  }

  pub unsafe fn ra(&self) -> Ra { 
     Ra(::core::ptr::read_volatile(((self.0 as usize) + 0x7) as *const u8))
  }
  pub unsafe fn set_ra(&mut self, value: Ra) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x7) as *mut u8, value.0);
  }
  pub unsafe fn with_ra<F: FnOnce(Ra) -> Ra>(&mut self, f: F) {
     let tmp = self.ra();
     self.set_ra(f(tmp))
  }

  pub unsafe fn smb(&self) -> Smb { 
     Smb(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u8))
  }
  pub unsafe fn set_smb(&mut self, value: Smb) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u8, value.0);
  }
  pub unsafe fn with_smb<F: FnOnce(Smb) -> Smb>(&mut self, f: F) {
     let tmp = self.smb();
     self.set_smb(f(tmp))
  }

  pub unsafe fn a2(&self) -> A2 { 
     A2(::core::ptr::read_volatile(((self.0 as usize) + 0x9) as *const u8))
  }
  pub unsafe fn set_a2(&mut self, value: A2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x9) as *mut u8, value.0);
  }
  pub unsafe fn with_a2<F: FnOnce(A2) -> A2>(&mut self, f: F) {
     let tmp = self.a2();
     self.set_a2(f(tmp))
  }

  pub unsafe fn slth(&self) -> Slth { 
     Slth(::core::ptr::read_volatile(((self.0 as usize) + 0xa) as *const u8))
  }
  pub unsafe fn set_slth(&mut self, value: Slth) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa) as *mut u8, value.0);
  }
  pub unsafe fn with_slth<F: FnOnce(Slth) -> Slth>(&mut self, f: F) {
     let tmp = self.slth();
     self.set_slth(f(tmp))
  }

  pub unsafe fn sltl(&self) -> Sltl { 
     Sltl(::core::ptr::read_volatile(((self.0 as usize) + 0xb) as *const u8))
  }
  pub unsafe fn set_sltl(&mut self, value: Sltl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xb) as *mut u8, value.0);
  }
  pub unsafe fn with_sltl<F: FnOnce(Sltl) -> Sltl>(&mut self, f: F) {
     let tmp = self.sltl();
     self.set_sltl(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct A1(pub u8);

impl A1 {
  pub fn ad(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x7f // [7:1]
  }
  pub fn set_ad(mut self, value: u8) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 1);
     self.0 |= value << 1;
     self
  }

}

impl ::core::fmt::Display for A1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for A1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ad() != 0 { try!(write!(f, " ad=0x{:x}", self.ad()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct F(pub u8);

impl F {
  pub fn icr(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x3f // [5:0]
  }
  pub fn set_icr(mut self, value: u8) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 0);
     self.0 |= value << 0;
     self
  }

  pub fn mult(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x3 // [7:6]
  }
  pub fn set_mult(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

}

impl ::core::fmt::Display for F {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for F {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.icr() != 0 { try!(write!(f, " icr=0x{:x}", self.icr()))}
      if self.mult() != 0 { try!(write!(f, " mult=0x{:x}", self.mult()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct C1(pub u8);

impl C1 {
  pub fn dmaen(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_dmaen(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn wuen(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_wuen(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn rsta(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  pub fn set_rsta(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn txak(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  pub fn set_txak(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn tx(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  pub fn set_tx(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn mst(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  pub fn set_mst(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn iicie(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_iicie(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn iicen(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_iicen(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for C1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for C1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dmaen() != 0 { try!(write!(f, " dmaen"))}
      if self.wuen() != 0 { try!(write!(f, " wuen"))}
      if self.rsta() != 0 { try!(write!(f, " rsta"))}
      if self.txak() != 0 { try!(write!(f, " txak"))}
      if self.tx() != 0 { try!(write!(f, " tx"))}
      if self.mst() != 0 { try!(write!(f, " mst"))}
      if self.iicie() != 0 { try!(write!(f, " iicie"))}
      if self.iicen() != 0 { try!(write!(f, " iicen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct S(pub u8);

impl S {
  pub fn rxak(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_rxak(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn iicif(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_iicif(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn srw(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  pub fn set_srw(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn ram(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  pub fn set_ram(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn arbl(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  pub fn set_arbl(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn busy(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  pub fn set_busy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn iaas(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_iaas(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn tcf(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_tcf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for S {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for S {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rxak() != 0 { try!(write!(f, " rxak"))}
      if self.iicif() != 0 { try!(write!(f, " iicif"))}
      if self.srw() != 0 { try!(write!(f, " srw"))}
      if self.ram() != 0 { try!(write!(f, " ram"))}
      if self.arbl() != 0 { try!(write!(f, " arbl"))}
      if self.busy() != 0 { try!(write!(f, " busy"))}
      if self.iaas() != 0 { try!(write!(f, " iaas"))}
      if self.tcf() != 0 { try!(write!(f, " tcf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct D(pub u8);

impl D {
  pub fn data(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_data(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for D {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for D {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct C2(pub u8);

impl C2 {
  pub fn ad(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x7 // [2:0]
  }
  pub fn set_ad(mut self, value: u8) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn rmen(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  pub fn set_rmen(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn sbrc(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  pub fn set_sbrc(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn hdrs(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  pub fn set_hdrs(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn adext(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_adext(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn gcaen(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_gcaen(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for C2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for C2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ad() != 0 { try!(write!(f, " ad=0x{:x}", self.ad()))}
      if self.rmen() != 0 { try!(write!(f, " rmen"))}
      if self.sbrc() != 0 { try!(write!(f, " sbrc"))}
      if self.hdrs() != 0 { try!(write!(f, " hdrs"))}
      if self.adext() != 0 { try!(write!(f, " adext"))}
      if self.gcaen() != 0 { try!(write!(f, " gcaen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Flt(pub u8);

impl Flt {
  pub fn flt(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xf // [3:0]
  }
  pub fn set_flt(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  pub fn startf(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  pub fn set_startf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn ssie(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  pub fn set_ssie(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn stopf(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_stopf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn shen(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_shen(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for Flt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Flt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.flt() != 0 { try!(write!(f, " flt=0x{:x}", self.flt()))}
      if self.startf() != 0 { try!(write!(f, " startf"))}
      if self.ssie() != 0 { try!(write!(f, " ssie"))}
      if self.stopf() != 0 { try!(write!(f, " stopf"))}
      if self.shen() != 0 { try!(write!(f, " shen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ra(pub u8);

impl Ra {
  pub fn rad(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x7f // [7:1]
  }
  pub fn set_rad(mut self, value: u8) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 1);
     self.0 |= value << 1;
     self
  }

}

impl ::core::fmt::Display for Ra {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ra {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rad() != 0 { try!(write!(f, " rad=0x{:x}", self.rad()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Smb(pub u8);

impl Smb {
  pub fn shtf2ie(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_shtf2ie(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn shtf2(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_shtf2(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn shtf1(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  pub fn set_shtf1(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn sltf(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  pub fn set_sltf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn tcksel(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  pub fn set_tcksel(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn siicaen(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  pub fn set_siicaen(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn alerten(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_alerten(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn fack(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_fack(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for Smb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Smb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.shtf2ie() != 0 { try!(write!(f, " shtf2ie"))}
      if self.shtf2() != 0 { try!(write!(f, " shtf2"))}
      if self.shtf1() != 0 { try!(write!(f, " shtf1"))}
      if self.sltf() != 0 { try!(write!(f, " sltf"))}
      if self.tcksel() != 0 { try!(write!(f, " tcksel"))}
      if self.siicaen() != 0 { try!(write!(f, " siicaen"))}
      if self.alerten() != 0 { try!(write!(f, " alerten"))}
      if self.fack() != 0 { try!(write!(f, " fack"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct A2(pub u8);

impl A2 {
  pub fn sad(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x7f // [7:1]
  }
  pub fn set_sad(mut self, value: u8) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 1);
     self.0 |= value << 1;
     self
  }

}

impl ::core::fmt::Display for A2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for A2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sad() != 0 { try!(write!(f, " sad=0x{:x}", self.sad()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Slth(pub u8);

impl Slth {
  pub fn sslt(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_sslt(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Slth {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Slth {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sslt() != 0 { try!(write!(f, " sslt=0x{:x}", self.sslt()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Sltl(pub u8);

impl Sltl {
  pub fn sslt(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_sslt(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Sltl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sltl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sslt() != 0 { try!(write!(f, " sslt=0x{:x}", self.sslt()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

