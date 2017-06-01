pub const I2C0: I2c0 = I2c0 {};
pub const I2C0_REF: &I2c0 = &I2C0;
pub const I2C0_IMPL: I2cImpl = I2cImpl(0x40066000);
pub const I2C0_IMPL_REF: &I2cImpl = &I2C0_IMPL;

pub struct I2c0 {}
impl ::core::ops::Deref for I2c0 {
   type Target = I2cImpl;
   #[inline]
   fn deref(&self) -> &I2cImpl { I2C0_IMPL_REF }
}


pub const I2C1: I2c1 = I2c1 {};
pub const I2C1_REF: &I2c1 = &I2C1;
pub const I2C1_IMPL: I2cImpl = I2cImpl(0x40067000);
pub const I2C1_IMPL_REF: &I2cImpl = &I2C1_IMPL;

pub struct I2c1 {}
impl ::core::ops::Deref for I2c1 {
   type Target = I2cImpl;
   #[inline]
   fn deref(&self) -> &I2cImpl { I2C1_IMPL_REF }
}



#[derive(Clone, Copy, PartialEq, Eq)]
pub struct I2cImpl(pub u32);
impl I2cImpl {
  #[inline]
  pub fn a1_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x0) as *const u8
  }
  #[inline]
  pub fn a1_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x0) as *mut u8
  }
  #[inline]
  pub fn a1(&self) -> A1 { 
     unsafe {
       A1(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u8))
     }
  }
  #[inline]
  pub fn set_a1(&self, value: A1) -> &I2cImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_a1<F: FnOnce(A1) -> A1>(&self, f: F) -> &I2cImpl {
     let tmp = self.a1();
     self.set_a1(f(tmp))
  }

  #[inline]
  pub fn f_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x1) as *const u8
  }
  #[inline]
  pub fn f_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x1) as *mut u8
  }
  #[inline]
  pub fn f(&self) -> F { 
     unsafe {
       F(::core::ptr::read_volatile(((self.0 as usize) + 0x1) as *const u8))
     }
  }
  #[inline]
  pub fn set_f(&self, value: F) -> &I2cImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x1) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_f<_F: FnOnce(F) -> F>(&self, f: _F) -> &I2cImpl {
     let tmp = self.f();
     self.set_f(f(tmp))
  }

  #[inline]
  pub fn c1_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x2) as *const u8
  }
  #[inline]
  pub fn c1_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x2) as *mut u8
  }
  #[inline]
  pub fn c1(&self) -> C1 { 
     unsafe {
       C1(::core::ptr::read_volatile(((self.0 as usize) + 0x2) as *const u8))
     }
  }
  #[inline]
  pub fn set_c1(&self, value: C1) -> &I2cImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x2) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_c1<F: FnOnce(C1) -> C1>(&self, f: F) -> &I2cImpl {
     let tmp = self.c1();
     self.set_c1(f(tmp))
  }

  #[inline]
  pub fn s_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x3) as *const u8
  }
  #[inline]
  pub fn s_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x3) as *mut u8
  }
  #[inline]
  pub fn s(&self) -> S { 
     unsafe {
       S(::core::ptr::read_volatile(((self.0 as usize) + 0x3) as *const u8))
     }
  }
  #[inline]
  pub fn set_s(&self, value: S) -> &I2cImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x3) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_s<F: FnOnce(S) -> S>(&self, f: F) -> &I2cImpl {
     let tmp = self.s();
     self.set_s(f(tmp))
  }

  #[inline]
  pub fn d_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x4) as *const u8
  }
  #[inline]
  pub fn d_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x4) as *mut u8
  }
  #[inline]
  pub fn d(&self) -> D { 
     unsafe {
       D(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u8))
     }
  }
  #[inline]
  pub fn set_d(&self, value: D) -> &I2cImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_d<F: FnOnce(D) -> D>(&self, f: F) -> &I2cImpl {
     let tmp = self.d();
     self.set_d(f(tmp))
  }

  #[inline]
  pub fn c2_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x5) as *const u8
  }
  #[inline]
  pub fn c2_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x5) as *mut u8
  }
  #[inline]
  pub fn c2(&self) -> C2 { 
     unsafe {
       C2(::core::ptr::read_volatile(((self.0 as usize) + 0x5) as *const u8))
     }
  }
  #[inline]
  pub fn set_c2(&self, value: C2) -> &I2cImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x5) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_c2<F: FnOnce(C2) -> C2>(&self, f: F) -> &I2cImpl {
     let tmp = self.c2();
     self.set_c2(f(tmp))
  }

  #[inline]
  pub fn flt_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x6) as *const u8
  }
  #[inline]
  pub fn flt_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x6) as *mut u8
  }
  #[inline]
  pub fn flt(&self) -> Flt { 
     unsafe {
       Flt(::core::ptr::read_volatile(((self.0 as usize) + 0x6) as *const u8))
     }
  }
  #[inline]
  pub fn set_flt(&self, value: Flt) -> &I2cImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x6) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_flt<F: FnOnce(Flt) -> Flt>(&self, f: F) -> &I2cImpl {
     let tmp = self.flt();
     self.set_flt(f(tmp))
  }

  #[inline]
  pub fn ra_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x7) as *const u8
  }
  #[inline]
  pub fn ra_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x7) as *mut u8
  }
  #[inline]
  pub fn ra(&self) -> Ra { 
     unsafe {
       Ra(::core::ptr::read_volatile(((self.0 as usize) + 0x7) as *const u8))
     }
  }
  #[inline]
  pub fn set_ra(&self, value: Ra) -> &I2cImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x7) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ra<F: FnOnce(Ra) -> Ra>(&self, f: F) -> &I2cImpl {
     let tmp = self.ra();
     self.set_ra(f(tmp))
  }

  #[inline]
  pub fn smb_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x8) as *const u8
  }
  #[inline]
  pub fn smb_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x8) as *mut u8
  }
  #[inline]
  pub fn smb(&self) -> Smb { 
     unsafe {
       Smb(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u8))
     }
  }
  #[inline]
  pub fn set_smb(&self, value: Smb) -> &I2cImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_smb<F: FnOnce(Smb) -> Smb>(&self, f: F) -> &I2cImpl {
     let tmp = self.smb();
     self.set_smb(f(tmp))
  }

  #[inline]
  pub fn a2_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x9) as *const u8
  }
  #[inline]
  pub fn a2_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x9) as *mut u8
  }
  #[inline]
  pub fn a2(&self) -> A2 { 
     unsafe {
       A2(::core::ptr::read_volatile(((self.0 as usize) + 0x9) as *const u8))
     }
  }
  #[inline]
  pub fn set_a2(&self, value: A2) -> &I2cImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x9) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_a2<F: FnOnce(A2) -> A2>(&self, f: F) -> &I2cImpl {
     let tmp = self.a2();
     self.set_a2(f(tmp))
  }

  #[inline]
  pub fn slth_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xa) as *const u8
  }
  #[inline]
  pub fn slth_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xa) as *mut u8
  }
  #[inline]
  pub fn slth(&self) -> Slth { 
     unsafe {
       Slth(::core::ptr::read_volatile(((self.0 as usize) + 0xa) as *const u8))
     }
  }
  #[inline]
  pub fn set_slth(&self, value: Slth) -> &I2cImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xa) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_slth<F: FnOnce(Slth) -> Slth>(&self, f: F) -> &I2cImpl {
     let tmp = self.slth();
     self.set_slth(f(tmp))
  }

  #[inline]
  pub fn sltl_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xb) as *const u8
  }
  #[inline]
  pub fn sltl_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xb) as *mut u8
  }
  #[inline]
  pub fn sltl(&self) -> Sltl { 
     unsafe {
       Sltl(::core::ptr::read_volatile(((self.0 as usize) + 0xb) as *const u8))
     }
  }
  #[inline]
  pub fn set_sltl(&self, value: Sltl) -> &I2cImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xb) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_sltl<F: FnOnce(Sltl) -> Sltl>(&self, f: F) -> &I2cImpl {
     let tmp = self.sltl();
     self.set_sltl(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct A1(pub u8);
impl A1 {
  #[inline]
  pub fn ad(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x7f // [7:1]
  }
  #[inline]
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
  #[inline]
  pub fn icr(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x3f // [5:0]
  }
  #[inline]
  pub fn set_icr(mut self, value: u8) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn mult(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x3 // [7:6]
  }
  #[inline]
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
  #[inline]
  pub fn dmaen(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_dmaen(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn wuen(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_wuen(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn rsta(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_rsta(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn txak(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_txak(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn tx(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_tx(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn mst(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_mst(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn iicie(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_iicie(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn iicen(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  #[inline]
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
  #[inline]
  pub fn rxak(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_rxak(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn iicif(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_iicif(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn srw(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_srw(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn ram(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_ram(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn arbl(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_arbl(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn busy(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_busy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn iaas(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_iaas(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn tcf(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  #[inline]
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
  #[inline]
  pub fn data(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  #[inline]
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
  #[inline]
  pub fn ad(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x7 // [2:0]
  }
  #[inline]
  pub fn set_ad(mut self, value: u8) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn rmen(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_rmen(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn sbrc(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_sbrc(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn hdrs(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_hdrs(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn adext(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_adext(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn gcaen(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  #[inline]
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
  #[inline]
  pub fn flt(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xf // [3:0]
  }
  #[inline]
  pub fn set_flt(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn startf(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_startf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn ssie(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_ssie(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn stopf(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_stopf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn shen(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  #[inline]
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
  #[inline]
  pub fn rad(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x7f // [7:1]
  }
  #[inline]
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
  #[inline]
  pub fn shtf2ie(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_shtf2ie(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn shtf2(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_shtf2(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn shtf1(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_shtf1(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn sltf(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_sltf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn tcksel(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_tcksel(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn siicaen(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_siicaen(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn alerten(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_alerten(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn fack(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  #[inline]
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
  #[inline]
  pub fn sad(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x7f // [7:1]
  }
  #[inline]
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
  #[inline]
  pub fn sslt(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  #[inline]
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
  #[inline]
  pub fn sslt(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  #[inline]
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
