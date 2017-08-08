//! Multipurpose Clock Generator module
#[allow(unused_imports)] use bobbin_common::bits;
pub const MCG: Mcg = Mcg(0x40064000);

#[doc="Multipurpose Clock Generator module"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mcg(pub u32);
impl Mcg {
#[doc="Get the *const pointer for the C1 register."]
  #[inline] pub fn c1_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x0) as *const u8
  }
#[doc="Get the *mut pointer for the C1 register."]
  #[inline] pub fn c1_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x0) as *mut u8
  }
#[doc="Read the C1 register."]
  #[inline] pub fn c1(&self) -> C1 { 
     unsafe {
        C1(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u8))
     }
  }
#[doc="Write the C1 register."]
  #[inline] pub fn set_c1<F: FnOnce(C1) -> C1>(&self, f: F) -> &Self {
     let value = f(C1(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the C1 register."]
  #[inline] pub fn with_c1<F: FnOnce(C1) -> C1>(&self, f: F) -> &Self {
     let tmp = self.c1();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u8, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the C2 register."]
  #[inline] pub fn c2_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x1) as *const u8
  }
#[doc="Get the *mut pointer for the C2 register."]
  #[inline] pub fn c2_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x1) as *mut u8
  }
#[doc="Read the C2 register."]
  #[inline] pub fn c2(&self) -> C2 { 
     unsafe {
        C2(::core::ptr::read_volatile(((self.0 as usize) + 0x1) as *const u8))
     }
  }
#[doc="Write the C2 register."]
  #[inline] pub fn set_c2<F: FnOnce(C2) -> C2>(&self, f: F) -> &Self {
     let value = f(C2(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the C2 register."]
  #[inline] pub fn with_c2<F: FnOnce(C2) -> C2>(&self, f: F) -> &Self {
     let tmp = self.c2();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1) as *mut u8, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the C3 register."]
  #[inline] pub fn c3_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x2) as *const u8
  }
#[doc="Get the *mut pointer for the C3 register."]
  #[inline] pub fn c3_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x2) as *mut u8
  }
#[doc="Read the C3 register."]
  #[inline] pub fn c3(&self) -> C3 { 
     unsafe {
        C3(::core::ptr::read_volatile(((self.0 as usize) + 0x2) as *const u8))
     }
  }
#[doc="Write the C3 register."]
  #[inline] pub fn set_c3<F: FnOnce(C3) -> C3>(&self, f: F) -> &Self {
     let value = f(C3(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the C3 register."]
  #[inline] pub fn with_c3<F: FnOnce(C3) -> C3>(&self, f: F) -> &Self {
     let tmp = self.c3();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2) as *mut u8, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the C4 register."]
  #[inline] pub fn c4_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x3) as *const u8
  }
#[doc="Get the *mut pointer for the C4 register."]
  #[inline] pub fn c4_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x3) as *mut u8
  }
#[doc="Read the C4 register."]
  #[inline] pub fn c4(&self) -> C4 { 
     unsafe {
        C4(::core::ptr::read_volatile(((self.0 as usize) + 0x3) as *const u8))
     }
  }
#[doc="Write the C4 register."]
  #[inline] pub fn set_c4<F: FnOnce(C4) -> C4>(&self, f: F) -> &Self {
     let value = f(C4(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x3) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the C4 register."]
  #[inline] pub fn with_c4<F: FnOnce(C4) -> C4>(&self, f: F) -> &Self {
     let tmp = self.c4();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x3) as *mut u8, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the C5 register."]
  #[inline] pub fn c5_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x4) as *const u8
  }
#[doc="Get the *mut pointer for the C5 register."]
  #[inline] pub fn c5_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x4) as *mut u8
  }
#[doc="Read the C5 register."]
  #[inline] pub fn c5(&self) -> C5 { 
     unsafe {
        C5(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u8))
     }
  }
#[doc="Write the C5 register."]
  #[inline] pub fn set_c5<F: FnOnce(C5) -> C5>(&self, f: F) -> &Self {
     let value = f(C5(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the C5 register."]
  #[inline] pub fn with_c5<F: FnOnce(C5) -> C5>(&self, f: F) -> &Self {
     let tmp = self.c5();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u8, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the C6 register."]
  #[inline] pub fn c6_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x5) as *const u8
  }
#[doc="Get the *mut pointer for the C6 register."]
  #[inline] pub fn c6_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x5) as *mut u8
  }
#[doc="Read the C6 register."]
  #[inline] pub fn c6(&self) -> C6 { 
     unsafe {
        C6(::core::ptr::read_volatile(((self.0 as usize) + 0x5) as *const u8))
     }
  }
#[doc="Write the C6 register."]
  #[inline] pub fn set_c6<F: FnOnce(C6) -> C6>(&self, f: F) -> &Self {
     let value = f(C6(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x5) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the C6 register."]
  #[inline] pub fn with_c6<F: FnOnce(C6) -> C6>(&self, f: F) -> &Self {
     let tmp = self.c6();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x5) as *mut u8, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the S register."]
  #[inline] pub fn s_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x6) as *const u8
  }
#[doc="Get the *mut pointer for the S register."]
  #[inline] pub fn s_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x6) as *mut u8
  }
#[doc="Read the S register."]
  #[inline] pub fn s(&self) -> S { 
     unsafe {
        S(::core::ptr::read_volatile(((self.0 as usize) + 0x6) as *const u8))
     }
  }
#[doc="Write the S register."]
  #[inline] pub fn set_s<F: FnOnce(S) -> S>(&self, f: F) -> &Self {
     let value = f(S(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x6) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the S register."]
  #[inline] pub fn with_s<F: FnOnce(S) -> S>(&self, f: F) -> &Self {
     let tmp = self.s();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x6) as *mut u8, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the SC register."]
  #[inline] pub fn sc_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x8) as *const u8
  }
#[doc="Get the *mut pointer for the SC register."]
  #[inline] pub fn sc_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x8) as *mut u8
  }
#[doc="Read the SC register."]
  #[inline] pub fn sc(&self) -> Sc { 
     unsafe {
        Sc(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u8))
     }
  }
#[doc="Write the SC register."]
  #[inline] pub fn set_sc<F: FnOnce(Sc) -> Sc>(&self, f: F) -> &Self {
     let value = f(Sc(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the SC register."]
  #[inline] pub fn with_sc<F: FnOnce(Sc) -> Sc>(&self, f: F) -> &Self {
     let tmp = self.sc();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u8, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the ATCVH register."]
  #[inline] pub fn atcvh_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xa) as *const u8
  }
#[doc="Get the *mut pointer for the ATCVH register."]
  #[inline] pub fn atcvh_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xa) as *mut u8
  }
#[doc="Read the ATCVH register."]
  #[inline] pub fn atcvh(&self) -> Atcvh { 
     unsafe {
        Atcvh(::core::ptr::read_volatile(((self.0 as usize) + 0xa) as *const u8))
     }
  }
#[doc="Write the ATCVH register."]
  #[inline] pub fn set_atcvh<F: FnOnce(Atcvh) -> Atcvh>(&self, f: F) -> &Self {
     let value = f(Atcvh(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the ATCVH register."]
  #[inline] pub fn with_atcvh<F: FnOnce(Atcvh) -> Atcvh>(&self, f: F) -> &Self {
     let tmp = self.atcvh();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa) as *mut u8, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the ATCVL register."]
  #[inline] pub fn atcvl_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xb) as *const u8
  }
#[doc="Get the *mut pointer for the ATCVL register."]
  #[inline] pub fn atcvl_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xb) as *mut u8
  }
#[doc="Read the ATCVL register."]
  #[inline] pub fn atcvl(&self) -> Atcvl { 
     unsafe {
        Atcvl(::core::ptr::read_volatile(((self.0 as usize) + 0xb) as *const u8))
     }
  }
#[doc="Write the ATCVL register."]
  #[inline] pub fn set_atcvl<F: FnOnce(Atcvl) -> Atcvl>(&self, f: F) -> &Self {
     let value = f(Atcvl(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xb) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the ATCVL register."]
  #[inline] pub fn with_atcvl<F: FnOnce(Atcvl) -> Atcvl>(&self, f: F) -> &Self {
     let tmp = self.atcvl();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xb) as *mut u8, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the C7 register."]
  #[inline] pub fn c7_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xc) as *const u8
  }
#[doc="Get the *mut pointer for the C7 register."]
  #[inline] pub fn c7_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xc) as *mut u8
  }
#[doc="Read the C7 register."]
  #[inline] pub fn c7(&self) -> C7 { 
     unsafe {
        C7(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u8))
     }
  }
#[doc="Write the C7 register."]
  #[inline] pub fn set_c7<F: FnOnce(C7) -> C7>(&self, f: F) -> &Self {
     let value = f(C7(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the C7 register."]
  #[inline] pub fn with_c7<F: FnOnce(C7) -> C7>(&self, f: F) -> &Self {
     let tmp = self.c7();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u8, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the C8 register."]
  #[inline] pub fn c8_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xd) as *const u8
  }
#[doc="Get the *mut pointer for the C8 register."]
  #[inline] pub fn c8_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xd) as *mut u8
  }
#[doc="Read the C8 register."]
  #[inline] pub fn c8(&self) -> C8 { 
     unsafe {
        C8(::core::ptr::read_volatile(((self.0 as usize) + 0xd) as *const u8))
     }
  }
#[doc="Write the C8 register."]
  #[inline] pub fn set_c8<F: FnOnce(C8) -> C8>(&self, f: F) -> &Self {
     let value = f(C8(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the C8 register."]
  #[inline] pub fn with_c8<F: FnOnce(C8) -> C8>(&self, f: F) -> &Self {
     let tmp = self.c8();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd) as *mut u8, value.0);
     }
     self
  }

}

#[doc="MCG Control 1 Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct C1(pub u8);
impl C1 {
#[doc="Internal Reference Stop Enable"]
  #[inline] pub fn irefsten(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Internal Reference Stop Enable"]
  #[inline] pub fn set_irefsten<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Internal Reference Clock Enable"]
  #[inline] pub fn irclken(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Internal Reference Clock Enable"]
  #[inline] pub fn set_irclken<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Internal Reference Select"]
  #[inline] pub fn irefs(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="Internal Reference Select"]
  #[inline] pub fn set_irefs<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="FLL External Reference Divider"]
  #[inline] pub fn frdiv(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
  }
#[doc="FLL External Reference Divider"]
  #[inline] pub fn set_frdiv<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x7 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Clock Source Select"]
  #[inline] pub fn clks(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
  }
#[doc="Clock Source Select"]
  #[inline] pub fn set_clks<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u8 = value.into();
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
#[doc="MCG Control 2 Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct C2(pub u8);
impl C2 {
#[doc="Internal Reference Clock Select"]
  #[inline] pub fn ircs(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Internal Reference Clock Select"]
  #[inline] pub fn set_ircs<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Low Power Select"]
  #[inline] pub fn lp(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Low Power Select"]
  #[inline] pub fn set_lp<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="External Reference Select"]
  #[inline] pub fn erefs(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="External Reference Select"]
  #[inline] pub fn set_erefs<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="High Gain Oscillator Select"]
  #[inline] pub fn hgo(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="High Gain Oscillator Select"]
  #[inline] pub fn set_hgo<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Frequency Range Select"]
  #[inline] pub fn range(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
  }
#[doc="Frequency Range Select"]
  #[inline] pub fn set_range<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Fast Internal Reference Clock Fine Trim"]
  #[inline] pub fn fcftrim(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="Fast Internal Reference Clock Fine Trim"]
  #[inline] pub fn set_fcftrim<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Loss of Clock Reset Enable"]
  #[inline] pub fn locre0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="Loss of Clock Reset Enable"]
  #[inline] pub fn set_locre0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
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
      if self.erefs() != 0 { try!(write!(f, " erefs"))}
      if self.hgo() != 0 { try!(write!(f, " hgo"))}
      if self.range() != 0 { try!(write!(f, " range=0x{:x}", self.range()))}
      if self.fcftrim() != 0 { try!(write!(f, " fcftrim"))}
      if self.locre0() != 0 { try!(write!(f, " locre0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="MCG Control 3 Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct C3(pub u8);
impl C3 {
#[doc="Slow Internal Reference Clock Trim Setting"]
  #[inline] pub fn sctrim(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="Slow Internal Reference Clock Trim Setting"]
  #[inline] pub fn set_sctrim<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u8 = value.into();
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
#[doc="MCG Control 4 Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct C4(pub u8);
impl C4 {
#[doc="Slow Internal Reference Clock Fine Trim"]
  #[inline] pub fn scftrim(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Slow Internal Reference Clock Fine Trim"]
  #[inline] pub fn set_scftrim<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Fast Internal Reference Clock Trim Setting"]
  #[inline] pub fn fctrim(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0xf) as u8) } // [4:1]
  }
#[doc="Fast Internal Reference Clock Trim Setting"]
  #[inline] pub fn set_fctrim<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0xf << 1);
     self.0 |= value << 1;
     self
  }

#[doc="DCO Range Select"]
  #[inline] pub fn drst_drs(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
  }
#[doc="DCO Range Select"]
  #[inline] pub fn set_drst_drs<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="DCO Maximum Frequency with 32.768 kHz Reference"]
  #[inline] pub fn dmx32(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="DCO Maximum Frequency with 32.768 kHz Reference"]
  #[inline] pub fn set_dmx32<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
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
#[doc="MCG Control 5 Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct C5(pub u8);
impl C5 {
#[doc="PLL External Reference Divider"]
  #[inline] pub fn prdiv0(&self) -> bits::U5 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
  }
#[doc="PLL External Reference Divider"]
  #[inline] pub fn set_prdiv0<V: Into<bits::U5>>(mut self, value: V) -> Self {
     let value: bits::U5 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

#[doc="PLL Stop Enable"]
  #[inline] pub fn pllsten0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="PLL Stop Enable"]
  #[inline] pub fn set_pllsten0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="PLL Clock Enable"]
  #[inline] pub fn pllclken0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="PLL Clock Enable"]
  #[inline] pub fn set_pllclken0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
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
#[doc="MCG Control 6 Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct C6(pub u8);
impl C6 {
#[doc="VCO 0 Divider"]
  #[inline] pub fn vdiv0(&self) -> bits::U5 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
  }
#[doc="VCO 0 Divider"]
  #[inline] pub fn set_vdiv0<V: Into<bits::U5>>(mut self, value: V) -> Self {
     let value: bits::U5 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Clock Monitor Enable"]
  #[inline] pub fn cme0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="Clock Monitor Enable"]
  #[inline] pub fn set_cme0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="PLL Select"]
  #[inline] pub fn plls(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="PLL Select"]
  #[inline] pub fn set_plls<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Loss of Lock Interrrupt Enable"]
  #[inline] pub fn lolie0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="Loss of Lock Interrrupt Enable"]
  #[inline] pub fn set_lolie0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
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
#[doc="MCG Status Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct S(pub u8);
impl S {
#[doc="Internal Reference Clock Status"]
  #[inline] pub fn ircst(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Internal Reference Clock Status"]
  #[inline] pub fn set_ircst<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="OSC Initialization"]
  #[inline] pub fn oscinit0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="OSC Initialization"]
  #[inline] pub fn set_oscinit0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Clock Mode Status"]
  #[inline] pub fn clkst(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
  }
#[doc="Clock Mode Status"]
  #[inline] pub fn set_clkst<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Internal Reference Status"]
  #[inline] pub fn irefst(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="Internal Reference Status"]
  #[inline] pub fn set_irefst<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="PLL Select Status"]
  #[inline] pub fn pllst(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="PLL Select Status"]
  #[inline] pub fn set_pllst<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Lock Status"]
  #[inline] pub fn lock0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="Lock Status"]
  #[inline] pub fn set_lock0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Loss of Lock Status"]
  #[inline] pub fn lols0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="Loss of Lock Status"]
  #[inline] pub fn set_lols0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
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
#[doc="MCG Status and Control Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sc(pub u8);
impl Sc {
#[doc="OSC0 Loss of Clock Status"]
  #[inline] pub fn locs0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="OSC0 Loss of Clock Status"]
  #[inline] pub fn set_locs0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Fast Clock Internal Reference Divider"]
  #[inline] pub fn fcrdiv(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7) as u8) } // [3:1]
  }
#[doc="Fast Clock Internal Reference Divider"]
  #[inline] pub fn set_fcrdiv<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x7 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="FLL Filter Preserve Enable"]
  #[inline] pub fn fltprsrv(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="FLL Filter Preserve Enable"]
  #[inline] pub fn set_fltprsrv<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Automatic Trim Machine Fail Flag"]
  #[inline] pub fn atmf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="Automatic Trim Machine Fail Flag"]
  #[inline] pub fn set_atmf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Automatic Trim Machine Select"]
  #[inline] pub fn atms(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="Automatic Trim Machine Select"]
  #[inline] pub fn set_atms<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Automatic Trim Machine Enable"]
  #[inline] pub fn atme(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="Automatic Trim Machine Enable"]
  #[inline] pub fn set_atme<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
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
#[doc="MCG Auto Trim Compare Value High Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Atcvh(pub u8);
impl Atcvh {
#[doc="ATM Compare Value High"]
  #[inline] pub fn atcvh(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="ATM Compare Value High"]
  #[inline] pub fn set_atcvh<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u8 = value.into();
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
#[doc="MCG Auto Trim Compare Value Low Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Atcvl(pub u8);
impl Atcvl {
#[doc="ATM Compare Value Low"]
  #[inline] pub fn atcvl(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="ATM Compare Value Low"]
  #[inline] pub fn set_atcvl<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u8 = value.into();
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
#[doc="MCG Control 7 Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct C7(pub u8);
impl C7 {
#[doc="MCG OSC Clock Select"]
  #[inline] pub fn oscsel(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
  }
#[doc="MCG OSC Clock Select"]
  #[inline] pub fn set_oscsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x3 << 0);
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
      if self.oscsel() != 0 { try!(write!(f, " oscsel=0x{:x}", self.oscsel()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="MCG Control 8 Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct C8(pub u8);
impl C8 {
#[doc="RTC Loss of Clock Status"]
  #[inline] pub fn locs1(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="RTC Loss of Clock Status"]
  #[inline] pub fn set_locs1<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Clock Monitor Enable1"]
  #[inline] pub fn cme1(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="Clock Monitor Enable1"]
  #[inline] pub fn set_cme1<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="PLL Loss of Lock Reset Enable"]
  #[inline] pub fn lolre(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="PLL Loss of Lock Reset Enable"]
  #[inline] pub fn set_lolre<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Loss of Clock Reset Enable"]
  #[inline] pub fn locre1(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="Loss of Clock Reset Enable"]
  #[inline] pub fn set_locre1<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
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
      if self.locs1() != 0 { try!(write!(f, " locs1"))}
      if self.cme1() != 0 { try!(write!(f, " cme1"))}
      if self.lolre() != 0 { try!(write!(f, " lolre"))}
      if self.locre1() != 0 { try!(write!(f, " locre1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

