pub const MCG: Mcg = Mcg(0x40064000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mcg(pub u32);
impl Mcg {
  #[inline] pub fn c1_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x0) as *const u8
  }
  #[inline] pub fn c1_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x0) as *mut u8
  }
  #[inline] pub fn c1(&self) -> C1 { 
     unsafe {
        C1(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u8))
     }
  }
  #[inline] pub fn set_c1(&self, value: C1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_c1<F: FnOnce(C1) -> C1>(&self, f: F) -> &Self {
     let tmp = self.c1();
     self.set_c1(f(tmp))
  }

  #[inline] pub fn c2_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x1) as *const u8
  }
  #[inline] pub fn c2_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x1) as *mut u8
  }
  #[inline] pub fn c2(&self) -> C2 { 
     unsafe {
        C2(::core::ptr::read_volatile(((self.0 as usize) + 0x1) as *const u8))
     }
  }
  #[inline] pub fn set_c2(&self, value: C2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_c2<F: FnOnce(C2) -> C2>(&self, f: F) -> &Self {
     let tmp = self.c2();
     self.set_c2(f(tmp))
  }

  #[inline] pub fn c3_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x2) as *const u8
  }
  #[inline] pub fn c3_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x2) as *mut u8
  }
  #[inline] pub fn c3(&self) -> C3 { 
     unsafe {
        C3(::core::ptr::read_volatile(((self.0 as usize) + 0x2) as *const u8))
     }
  }
  #[inline] pub fn set_c3(&self, value: C3) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_c3<F: FnOnce(C3) -> C3>(&self, f: F) -> &Self {
     let tmp = self.c3();
     self.set_c3(f(tmp))
  }

  #[inline] pub fn c4_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x3) as *const u8
  }
  #[inline] pub fn c4_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x3) as *mut u8
  }
  #[inline] pub fn c4(&self) -> C4 { 
     unsafe {
        C4(::core::ptr::read_volatile(((self.0 as usize) + 0x3) as *const u8))
     }
  }
  #[inline] pub fn set_c4(&self, value: C4) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x3) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_c4<F: FnOnce(C4) -> C4>(&self, f: F) -> &Self {
     let tmp = self.c4();
     self.set_c4(f(tmp))
  }

  #[inline] pub fn c5_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x4) as *const u8
  }
  #[inline] pub fn c5_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x4) as *mut u8
  }
  #[inline] pub fn c5(&self) -> C5 { 
     unsafe {
        C5(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u8))
     }
  }
  #[inline] pub fn set_c5(&self, value: C5) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_c5<F: FnOnce(C5) -> C5>(&self, f: F) -> &Self {
     let tmp = self.c5();
     self.set_c5(f(tmp))
  }

  #[inline] pub fn c6_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x5) as *const u8
  }
  #[inline] pub fn c6_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x5) as *mut u8
  }
  #[inline] pub fn c6(&self) -> C6 { 
     unsafe {
        C6(::core::ptr::read_volatile(((self.0 as usize) + 0x5) as *const u8))
     }
  }
  #[inline] pub fn set_c6(&self, value: C6) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x5) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_c6<F: FnOnce(C6) -> C6>(&self, f: F) -> &Self {
     let tmp = self.c6();
     self.set_c6(f(tmp))
  }

  #[inline] pub fn s_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x6) as *const u8
  }
  #[inline] pub fn s_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x6) as *mut u8
  }
  #[inline] pub fn s(&self) -> S { 
     unsafe {
        S(::core::ptr::read_volatile(((self.0 as usize) + 0x6) as *const u8))
     }
  }
  #[inline] pub fn set_s(&self, value: S) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x6) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_s<F: FnOnce(S) -> S>(&self, f: F) -> &Self {
     let tmp = self.s();
     self.set_s(f(tmp))
  }

  #[inline] pub fn sc_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x8) as *const u8
  }
  #[inline] pub fn sc_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x8) as *mut u8
  }
  #[inline] pub fn sc(&self) -> Sc { 
     unsafe {
        Sc(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u8))
     }
  }
  #[inline] pub fn set_sc(&self, value: Sc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_sc<F: FnOnce(Sc) -> Sc>(&self, f: F) -> &Self {
     let tmp = self.sc();
     self.set_sc(f(tmp))
  }

  #[inline] pub fn atcvh_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xa) as *const u8
  }
  #[inline] pub fn atcvh_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xa) as *mut u8
  }
  #[inline] pub fn atcvh(&self) -> Atcvh { 
     unsafe {
        Atcvh(::core::ptr::read_volatile(((self.0 as usize) + 0xa) as *const u8))
     }
  }
  #[inline] pub fn set_atcvh(&self, value: Atcvh) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_atcvh<F: FnOnce(Atcvh) -> Atcvh>(&self, f: F) -> &Self {
     let tmp = self.atcvh();
     self.set_atcvh(f(tmp))
  }

  #[inline] pub fn atcvl_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xb) as *const u8
  }
  #[inline] pub fn atcvl_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xb) as *mut u8
  }
  #[inline] pub fn atcvl(&self) -> Atcvl { 
     unsafe {
        Atcvl(::core::ptr::read_volatile(((self.0 as usize) + 0xb) as *const u8))
     }
  }
  #[inline] pub fn set_atcvl(&self, value: Atcvl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xb) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_atcvl<F: FnOnce(Atcvl) -> Atcvl>(&self, f: F) -> &Self {
     let tmp = self.atcvl();
     self.set_atcvl(f(tmp))
  }

  #[inline] pub fn c7_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xc) as *const u8
  }
  #[inline] pub fn c7_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xc) as *mut u8
  }
  #[inline] pub fn c7(&self) -> C7 { 
     unsafe {
        C7(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u8))
     }
  }
  #[inline] pub fn set_c7(&self, value: C7) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_c7<F: FnOnce(C7) -> C7>(&self, f: F) -> &Self {
     let tmp = self.c7();
     self.set_c7(f(tmp))
  }

  #[inline] pub fn c8_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xd) as *const u8
  }
  #[inline] pub fn c8_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xd) as *mut u8
  }
  #[inline] pub fn c8(&self) -> C8 { 
     unsafe {
        C8(::core::ptr::read_volatile(((self.0 as usize) + 0xd) as *const u8))
     }
  }
  #[inline] pub fn set_c8(&self, value: C8) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_c8<F: FnOnce(C8) -> C8>(&self, f: F) -> &Self {
     let tmp = self.c8();
     self.set_c8(f(tmp))
  }

  #[inline] pub fn c9_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xe) as *const u8
  }
  #[inline] pub fn c9_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xe) as *mut u8
  }
  #[inline] pub fn c9(&self) -> C9 { 
     unsafe {
        C9(::core::ptr::read_volatile(((self.0 as usize) + 0xe) as *const u8))
     }
  }

  #[inline] pub fn c10_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xf) as *const u8
  }
  #[inline] pub fn c10_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xf) as *mut u8
  }
  #[inline] pub fn c10(&self) -> C10 { 
     unsafe {
        C10(::core::ptr::read_volatile(((self.0 as usize) + 0xf) as *const u8))
     }
  }

}

#[derive(PartialEq, Eq)]
pub struct C1(pub u8);
impl C1 {
  #[inline] pub fn irefsten(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_irefsten(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn irclken(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_irclken(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn irefs(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_irefs(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn frdiv(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x7 // [5:3]
  }
  #[inline] pub fn set_frdiv(mut self, value: u8) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn clks(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x3 // [7:6]
  }
  #[inline] pub fn set_clks(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
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
      if self.irefsten() != 0 { try!(write!(f, " irefsten"))}
      if self.irclken() != 0 { try!(write!(f, " irclken"))}
      if self.irefs() != 0 { try!(write!(f, " irefs"))}
      if self.frdiv() != 0 { try!(write!(f, " frdiv=0x{:x}", self.frdiv()))}
      if self.clks() != 0 { try!(write!(f, " clks=0x{:x}", self.clks()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct C2(pub u8);
impl C2 {
  #[inline] pub fn ircs(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_ircs(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn lp(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_lp(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn erefs0(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_erefs0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn hgo0(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_hgo0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn range0(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x3 // [5:4]
  }
  #[inline] pub fn set_range0(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn fcftrim(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_fcftrim(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn locre0(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_locre0(mut self, value: u8) -> Self {
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
      if self.ircs() != 0 { try!(write!(f, " ircs"))}
      if self.lp() != 0 { try!(write!(f, " lp"))}
      if self.erefs0() != 0 { try!(write!(f, " erefs0"))}
      if self.hgo0() != 0 { try!(write!(f, " hgo0"))}
      if self.range0() != 0 { try!(write!(f, " range0=0x{:x}", self.range0()))}
      if self.fcftrim() != 0 { try!(write!(f, " fcftrim"))}
      if self.locre0() != 0 { try!(write!(f, " locre0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct C3(pub u8);
impl C3 {
  #[inline] pub fn sctrim(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  #[inline] pub fn set_sctrim(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for C3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for C3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sctrim() != 0 { try!(write!(f, " sctrim=0x{:x}", self.sctrim()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct C4(pub u8);
impl C4 {
  #[inline] pub fn scftrim(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_scftrim(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn fctrim(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0xf // [4:1]
  }
  #[inline] pub fn set_fctrim(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn drst_drs(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x3 // [6:5]
  }
  #[inline] pub fn set_drst_drs(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn dmx32(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_dmx32(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for C4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for C4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.scftrim() != 0 { try!(write!(f, " scftrim"))}
      if self.fctrim() != 0 { try!(write!(f, " fctrim=0x{:x}", self.fctrim()))}
      if self.drst_drs() != 0 { try!(write!(f, " drst_drs=0x{:x}", self.drst_drs()))}
      if self.dmx32() != 0 { try!(write!(f, " dmx32"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct C5(pub u8);
impl C5 {
  #[inline] pub fn prdiv0(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1f // [4:0]
  }
  #[inline] pub fn set_prdiv0(mut self, value: u8) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pllsten0(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pllsten0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn pllclken0(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_pllclken0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

}
impl ::core::fmt::Display for C5 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for C5 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.prdiv0() != 0 { try!(write!(f, " prdiv0=0x{:x}", self.prdiv0()))}
      if self.pllsten0() != 0 { try!(write!(f, " pllsten0"))}
      if self.pllclken0() != 0 { try!(write!(f, " pllclken0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct C6(pub u8);
impl C6 {
  #[inline] pub fn vdiv0(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1f // [4:0]
  }
  #[inline] pub fn set_vdiv0(mut self, value: u8) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn cme0(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_cme0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn plls(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_plls(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn lolie0(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_lolie0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for C6 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for C6 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.vdiv0() != 0 { try!(write!(f, " vdiv0=0x{:x}", self.vdiv0()))}
      if self.cme0() != 0 { try!(write!(f, " cme0"))}
      if self.plls() != 0 { try!(write!(f, " plls"))}
      if self.lolie0() != 0 { try!(write!(f, " lolie0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct S(pub u8);
impl S {
  #[inline] pub fn ircst(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_ircst(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn oscinit0(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_oscinit0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn clkst(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x3 // [3:2]
  }
  #[inline] pub fn set_clkst(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn irefst(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_irefst(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pllst(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pllst(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn lock0(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_lock0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn lols0(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_lols0(mut self, value: u8) -> Self {
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
      if self.ircst() != 0 { try!(write!(f, " ircst"))}
      if self.oscinit0() != 0 { try!(write!(f, " oscinit0"))}
      if self.clkst() != 0 { try!(write!(f, " clkst=0x{:x}", self.clkst()))}
      if self.irefst() != 0 { try!(write!(f, " irefst"))}
      if self.pllst() != 0 { try!(write!(f, " pllst"))}
      if self.lock0() != 0 { try!(write!(f, " lock0"))}
      if self.lols0() != 0 { try!(write!(f, " lols0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Sc(pub u8);
impl Sc {
  #[inline] pub fn locs0(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_locs0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn fcrdiv(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x7 // [3:1]
  }
  #[inline] pub fn set_fcrdiv(mut self, value: u8) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn fltprsrv(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_fltprsrv(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn atmf(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_atmf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn atms(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_atms(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn atme(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_atme(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Sc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.locs0() != 0 { try!(write!(f, " locs0"))}
      if self.fcrdiv() != 0 { try!(write!(f, " fcrdiv=0x{:x}", self.fcrdiv()))}
      if self.fltprsrv() != 0 { try!(write!(f, " fltprsrv"))}
      if self.atmf() != 0 { try!(write!(f, " atmf"))}
      if self.atms() != 0 { try!(write!(f, " atms"))}
      if self.atme() != 0 { try!(write!(f, " atme"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Atcvh(pub u8);
impl Atcvh {
  #[inline] pub fn atcvh(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  #[inline] pub fn set_atcvh(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Atcvh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Atcvh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.atcvh() != 0 { try!(write!(f, " atcvh=0x{:x}", self.atcvh()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Atcvl(pub u8);
impl Atcvl {
  #[inline] pub fn atcvl(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  #[inline] pub fn set_atcvl(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Atcvl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Atcvl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.atcvl() != 0 { try!(write!(f, " atcvl=0x{:x}", self.atcvl()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct C7(pub u8);
impl C7 {
  #[inline] pub fn oscsel(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_oscsel(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for C7 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for C7 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.oscsel() != 0 { try!(write!(f, " oscsel"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct C8(pub u8);
impl C8 {
  #[inline] pub fn lolre(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_lolre(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

}
impl ::core::fmt::Display for C8 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for C8 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lolre() != 0 { try!(write!(f, " lolre"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct C9(pub u8);
impl C9 {
}
impl ::core::fmt::Display for C9 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for C9 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct C10(pub u8);
impl C10 {
}
impl ::core::fmt::Display for C10 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for C10 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

