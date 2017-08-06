#[allow(unused_imports)] use bobbin_common::bits;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="I2C Peripheral"]
pub struct Periph<T>(pub u32, pub T); 



impl<T> Periph<T> {
#[doc="Get the *const pointer for the A1 register."]
  #[inline] pub fn a1_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x0) as *const u8
  }
#[doc="Get the *mut pointer for the A1 register."]
  #[inline] pub fn a1_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x0) as *mut u8
  }
#[doc="Read the A1 register."]
  #[inline] pub fn a1(&self) -> A1 { 
     unsafe {
        A1(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u8))
     }
  }
#[doc="Write the A1 register."]
  #[inline] pub fn set_a1(&self, value: A1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the A1 register."]
  #[inline] pub fn with_a1<F: FnOnce(A1) -> A1>(&self, f: F) -> &Self {
     let tmp = self.a1();
     self.set_a1(f(tmp))
  }

#[doc="Get the *const pointer for the F register."]
  #[inline] pub fn f_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x1) as *const u8
  }
#[doc="Get the *mut pointer for the F register."]
  #[inline] pub fn f_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x1) as *mut u8
  }
#[doc="Read the F register."]
  #[inline] pub fn f(&self) -> F { 
     unsafe {
        F(::core::ptr::read_volatile(((self.0 as usize) + 0x1) as *const u8))
     }
  }
#[doc="Write the F register."]
  #[inline] pub fn set_f(&self, value: F) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the F register."]
  #[inline] pub fn with_f<_F: FnOnce(F) -> F>(&self, f: _F) -> &Self {
     let tmp = self.f();
     self.set_f(f(tmp))
  }

#[doc="Get the *const pointer for the C1 register."]
  #[inline] pub fn c1_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x2) as *const u8
  }
#[doc="Get the *mut pointer for the C1 register."]
  #[inline] pub fn c1_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x2) as *mut u8
  }
#[doc="Read the C1 register."]
  #[inline] pub fn c1(&self) -> C1 { 
     unsafe {
        C1(::core::ptr::read_volatile(((self.0 as usize) + 0x2) as *const u8))
     }
  }
#[doc="Write the C1 register."]
  #[inline] pub fn set_c1(&self, value: C1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the C1 register."]
  #[inline] pub fn with_c1<F: FnOnce(C1) -> C1>(&self, f: F) -> &Self {
     let tmp = self.c1();
     self.set_c1(f(tmp))
  }

#[doc="Get the *const pointer for the S register."]
  #[inline] pub fn s_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x3) as *const u8
  }
#[doc="Get the *mut pointer for the S register."]
  #[inline] pub fn s_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x3) as *mut u8
  }
#[doc="Read the S register."]
  #[inline] pub fn s(&self) -> S { 
     unsafe {
        S(::core::ptr::read_volatile(((self.0 as usize) + 0x3) as *const u8))
     }
  }
#[doc="Write the S register."]
  #[inline] pub fn set_s(&self, value: S) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x3) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the S register."]
  #[inline] pub fn with_s<F: FnOnce(S) -> S>(&self, f: F) -> &Self {
     let tmp = self.s();
     self.set_s(f(tmp))
  }

#[doc="Get the *const pointer for the D register."]
  #[inline] pub fn d_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x4) as *const u8
  }
#[doc="Get the *mut pointer for the D register."]
  #[inline] pub fn d_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x4) as *mut u8
  }
#[doc="Read the D register."]
  #[inline] pub fn d(&self) -> D { 
     unsafe {
        D(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u8))
     }
  }
#[doc="Write the D register."]
  #[inline] pub fn set_d(&self, value: D) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the D register."]
  #[inline] pub fn with_d<F: FnOnce(D) -> D>(&self, f: F) -> &Self {
     let tmp = self.d();
     self.set_d(f(tmp))
  }

#[doc="Get the *const pointer for the C2 register."]
  #[inline] pub fn c2_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x5) as *const u8
  }
#[doc="Get the *mut pointer for the C2 register."]
  #[inline] pub fn c2_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x5) as *mut u8
  }
#[doc="Read the C2 register."]
  #[inline] pub fn c2(&self) -> C2 { 
     unsafe {
        C2(::core::ptr::read_volatile(((self.0 as usize) + 0x5) as *const u8))
     }
  }
#[doc="Write the C2 register."]
  #[inline] pub fn set_c2(&self, value: C2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x5) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the C2 register."]
  #[inline] pub fn with_c2<F: FnOnce(C2) -> C2>(&self, f: F) -> &Self {
     let tmp = self.c2();
     self.set_c2(f(tmp))
  }

#[doc="Get the *const pointer for the FLT register."]
  #[inline] pub fn flt_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x6) as *const u8
  }
#[doc="Get the *mut pointer for the FLT register."]
  #[inline] pub fn flt_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x6) as *mut u8
  }
#[doc="Read the FLT register."]
  #[inline] pub fn flt(&self) -> Flt { 
     unsafe {
        Flt(::core::ptr::read_volatile(((self.0 as usize) + 0x6) as *const u8))
     }
  }
#[doc="Write the FLT register."]
  #[inline] pub fn set_flt(&self, value: Flt) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x6) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the FLT register."]
  #[inline] pub fn with_flt<F: FnOnce(Flt) -> Flt>(&self, f: F) -> &Self {
     let tmp = self.flt();
     self.set_flt(f(tmp))
  }

#[doc="Get the *const pointer for the RA register."]
  #[inline] pub fn ra_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x7) as *const u8
  }
#[doc="Get the *mut pointer for the RA register."]
  #[inline] pub fn ra_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x7) as *mut u8
  }
#[doc="Read the RA register."]
  #[inline] pub fn ra(&self) -> Ra { 
     unsafe {
        Ra(::core::ptr::read_volatile(((self.0 as usize) + 0x7) as *const u8))
     }
  }
#[doc="Write the RA register."]
  #[inline] pub fn set_ra(&self, value: Ra) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x7) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the RA register."]
  #[inline] pub fn with_ra<F: FnOnce(Ra) -> Ra>(&self, f: F) -> &Self {
     let tmp = self.ra();
     self.set_ra(f(tmp))
  }

#[doc="Get the *const pointer for the SMB register."]
  #[inline] pub fn smb_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x8) as *const u8
  }
#[doc="Get the *mut pointer for the SMB register."]
  #[inline] pub fn smb_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x8) as *mut u8
  }
#[doc="Read the SMB register."]
  #[inline] pub fn smb(&self) -> Smb { 
     unsafe {
        Smb(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u8))
     }
  }
#[doc="Write the SMB register."]
  #[inline] pub fn set_smb(&self, value: Smb) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the SMB register."]
  #[inline] pub fn with_smb<F: FnOnce(Smb) -> Smb>(&self, f: F) -> &Self {
     let tmp = self.smb();
     self.set_smb(f(tmp))
  }

#[doc="Get the *const pointer for the A2 register."]
  #[inline] pub fn a2_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x9) as *const u8
  }
#[doc="Get the *mut pointer for the A2 register."]
  #[inline] pub fn a2_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x9) as *mut u8
  }
#[doc="Read the A2 register."]
  #[inline] pub fn a2(&self) -> A2 { 
     unsafe {
        A2(::core::ptr::read_volatile(((self.0 as usize) + 0x9) as *const u8))
     }
  }
#[doc="Write the A2 register."]
  #[inline] pub fn set_a2(&self, value: A2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x9) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the A2 register."]
  #[inline] pub fn with_a2<F: FnOnce(A2) -> A2>(&self, f: F) -> &Self {
     let tmp = self.a2();
     self.set_a2(f(tmp))
  }

#[doc="Get the *const pointer for the SLTH register."]
  #[inline] pub fn slth_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xa) as *const u8
  }
#[doc="Get the *mut pointer for the SLTH register."]
  #[inline] pub fn slth_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xa) as *mut u8
  }
#[doc="Read the SLTH register."]
  #[inline] pub fn slth(&self) -> Slth { 
     unsafe {
        Slth(::core::ptr::read_volatile(((self.0 as usize) + 0xa) as *const u8))
     }
  }
#[doc="Write the SLTH register."]
  #[inline] pub fn set_slth(&self, value: Slth) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the SLTH register."]
  #[inline] pub fn with_slth<F: FnOnce(Slth) -> Slth>(&self, f: F) -> &Self {
     let tmp = self.slth();
     self.set_slth(f(tmp))
  }

#[doc="Get the *const pointer for the SLTL register."]
  #[inline] pub fn sltl_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xb) as *const u8
  }
#[doc="Get the *mut pointer for the SLTL register."]
  #[inline] pub fn sltl_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xb) as *mut u8
  }
#[doc="Read the SLTL register."]
  #[inline] pub fn sltl(&self) -> Sltl { 
     unsafe {
        Sltl(::core::ptr::read_volatile(((self.0 as usize) + 0xb) as *const u8))
     }
  }
#[doc="Write the SLTL register."]
  #[inline] pub fn set_sltl(&self, value: Sltl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xb) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the SLTL register."]
  #[inline] pub fn with_sltl<F: FnOnce(Sltl) -> Sltl>(&self, f: F) -> &Self {
     let tmp = self.sltl();
     self.set_sltl(f(tmp))
  }

}

#[doc="I2C Address Register 1"]
#[derive(PartialEq, Eq)]
pub struct A1(pub u8);
impl A1 {
#[doc="Address"]
  #[inline] pub fn ad(&self) -> bits::B7 {
     (((self.0 as u8) >> 1) & 0x7f).into() // [7:1]
  }
#[doc="Address"]
  #[inline] pub fn set_ad<V: Into<bits::B7>>(mut self, value: V) -> Self {
     let value: bits::B7 = value.into();
     let value: u8 = value.into();
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
#[doc="I2C Frequency Divider register"]
#[derive(PartialEq, Eq)]
pub struct F(pub u8);
impl F {
#[doc="ClockRate"]
  #[inline] pub fn icr(&self) -> bits::B6 {
     (((self.0 as u8) >> 0) & 0x3f).into() // [5:0]
  }
#[doc="ClockRate"]
  #[inline] pub fn set_icr<V: Into<bits::B6>>(mut self, value: V) -> Self {
     let value: bits::B6 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Multiplier Factor"]
  #[inline] pub fn mult(&self) -> bits::B2 {
     (((self.0 as u8) >> 6) & 0x3).into() // [7:6]
  }
#[doc="Multiplier Factor"]
  #[inline] pub fn set_mult<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u8 = value.into();
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
#[doc="I2C Control Register 1"]
#[derive(PartialEq, Eq)]
pub struct C1(pub u8);
impl C1 {
#[doc="DMA Enable"]
  #[inline] pub fn dmaen(&self) -> bits::B1 {
     (((self.0 as u8) >> 0) & 0x1).into() // [0]
  }
#[doc="DMA Enable"]
  #[inline] pub fn set_dmaen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Wakeup Enable"]
  #[inline] pub fn wuen(&self) -> bits::B1 {
     (((self.0 as u8) >> 1) & 0x1).into() // [1]
  }
#[doc="Wakeup Enable"]
  #[inline] pub fn set_wuen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Repeat START"]
  #[inline] pub fn rsta(&self) -> bits::B1 {
     (((self.0 as u8) >> 2) & 0x1).into() // [2]
  }
#[doc="Repeat START"]
  #[inline] pub fn set_rsta<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Transmit Acknowledge Enable"]
  #[inline] pub fn txak(&self) -> bits::B1 {
     (((self.0 as u8) >> 3) & 0x1).into() // [3]
  }
#[doc="Transmit Acknowledge Enable"]
  #[inline] pub fn set_txak<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Transmit Mode Select"]
  #[inline] pub fn tx(&self) -> bits::B1 {
     (((self.0 as u8) >> 4) & 0x1).into() // [4]
  }
#[doc="Transmit Mode Select"]
  #[inline] pub fn set_tx<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Master Mode Select"]
  #[inline] pub fn mst(&self) -> bits::B1 {
     (((self.0 as u8) >> 5) & 0x1).into() // [5]
  }
#[doc="Master Mode Select"]
  #[inline] pub fn set_mst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="I2C Interrupt Enable"]
  #[inline] pub fn iicie(&self) -> bits::B1 {
     (((self.0 as u8) >> 6) & 0x1).into() // [6]
  }
#[doc="I2C Interrupt Enable"]
  #[inline] pub fn set_iicie<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="I2C Enable"]
  #[inline] pub fn iicen(&self) -> bits::B1 {
     (((self.0 as u8) >> 7) & 0x1).into() // [7]
  }
#[doc="I2C Enable"]
  #[inline] pub fn set_iicen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
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
#[doc="I2C Status register"]
#[derive(PartialEq, Eq)]
pub struct S(pub u8);
impl S {
#[doc="Receive Acknowledge"]
  #[inline] pub fn rxak(&self) -> bits::B1 {
     (((self.0 as u8) >> 0) & 0x1).into() // [0]
  }
#[doc="Receive Acknowledge"]
  #[inline] pub fn set_rxak<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Interrupt Flag"]
  #[inline] pub fn iicif(&self) -> bits::B1 {
     (((self.0 as u8) >> 1) & 0x1).into() // [1]
  }
#[doc="Interrupt Flag"]
  #[inline] pub fn set_iicif<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Slave Read/Write"]
  #[inline] pub fn srw(&self) -> bits::B1 {
     (((self.0 as u8) >> 2) & 0x1).into() // [2]
  }
#[doc="Slave Read/Write"]
  #[inline] pub fn set_srw<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Range Address Match"]
  #[inline] pub fn ram(&self) -> bits::B1 {
     (((self.0 as u8) >> 3) & 0x1).into() // [3]
  }
#[doc="Range Address Match"]
  #[inline] pub fn set_ram<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Arbitration Lost"]
  #[inline] pub fn arbl(&self) -> bits::B1 {
     (((self.0 as u8) >> 4) & 0x1).into() // [4]
  }
#[doc="Arbitration Lost"]
  #[inline] pub fn set_arbl<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Bus Busy"]
  #[inline] pub fn busy(&self) -> bits::B1 {
     (((self.0 as u8) >> 5) & 0x1).into() // [5]
  }
#[doc="Bus Busy"]
  #[inline] pub fn set_busy<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Addressed As A Slave"]
  #[inline] pub fn iaas(&self) -> bits::B1 {
     (((self.0 as u8) >> 6) & 0x1).into() // [6]
  }
#[doc="Addressed As A Slave"]
  #[inline] pub fn set_iaas<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Transfer Complete Flag"]
  #[inline] pub fn tcf(&self) -> bits::B1 {
     (((self.0 as u8) >> 7) & 0x1).into() // [7]
  }
#[doc="Transfer Complete Flag"]
  #[inline] pub fn set_tcf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
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
#[doc="I2C Data I/O register"]
#[derive(PartialEq, Eq)]
pub struct D(pub u8);
impl D {
#[doc="Data"]
  #[inline] pub fn data(&self) -> bits::B8 {
     (((self.0 as u8) >> 0) & 0xff).into() // [7:0]
  }
#[doc="Data"]
  #[inline] pub fn set_data<V: Into<bits::B8>>(mut self, value: V) -> Self {
     let value: bits::B8 = value.into();
     let value: u8 = value.into();
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
#[doc="I2C Control Register 2"]
#[derive(PartialEq, Eq)]
pub struct C2(pub u8);
impl C2 {
#[doc="Slave Address"]
  #[inline] pub fn ad(&self) -> bits::B3 {
     (((self.0 as u8) >> 0) & 0x7).into() // [2:0]
  }
#[doc="Slave Address"]
  #[inline] pub fn set_ad<V: Into<bits::B3>>(mut self, value: V) -> Self {
     let value: bits::B3 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Range Address Matching Enable"]
  #[inline] pub fn rmen(&self) -> bits::B1 {
     (((self.0 as u8) >> 3) & 0x1).into() // [3]
  }
#[doc="Range Address Matching Enable"]
  #[inline] pub fn set_rmen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Slave Baud Rate Control"]
  #[inline] pub fn sbrc(&self) -> bits::B1 {
     (((self.0 as u8) >> 4) & 0x1).into() // [4]
  }
#[doc="Slave Baud Rate Control"]
  #[inline] pub fn set_sbrc<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="High Drive Select"]
  #[inline] pub fn hdrs(&self) -> bits::B1 {
     (((self.0 as u8) >> 5) & 0x1).into() // [5]
  }
#[doc="High Drive Select"]
  #[inline] pub fn set_hdrs<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Address Extension"]
  #[inline] pub fn adext(&self) -> bits::B1 {
     (((self.0 as u8) >> 6) & 0x1).into() // [6]
  }
#[doc="Address Extension"]
  #[inline] pub fn set_adext<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="General Call Address Enable"]
  #[inline] pub fn gcaen(&self) -> bits::B1 {
     (((self.0 as u8) >> 7) & 0x1).into() // [7]
  }
#[doc="General Call Address Enable"]
  #[inline] pub fn set_gcaen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
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
#[doc="I2C Programmable Input Glitch Filter register"]
#[derive(PartialEq, Eq)]
pub struct Flt(pub u8);
impl Flt {
#[doc="I2C Programmable Filter Factor"]
  #[inline] pub fn flt(&self) -> bits::B4 {
     (((self.0 as u8) >> 0) & 0xf).into() // [3:0]
  }
#[doc="I2C Programmable Filter Factor"]
  #[inline] pub fn set_flt<V: Into<bits::B4>>(mut self, value: V) -> Self {
     let value: bits::B4 = value.into();
     let value: u8 = value.into();
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

#[doc="I2C Bus Start Detect Flag"]
  #[inline] pub fn startf(&self) -> bits::B1 {
     (((self.0 as u8) >> 4) & 0x1).into() // [4]
  }
#[doc="I2C Bus Start Detect Flag"]
  #[inline] pub fn set_startf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="I2C Bus Stop or Start Interrupt Enable"]
  #[inline] pub fn ssie(&self) -> bits::B1 {
     (((self.0 as u8) >> 5) & 0x1).into() // [5]
  }
#[doc="I2C Bus Stop or Start Interrupt Enable"]
  #[inline] pub fn set_ssie<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="I2C Bus Stop Detect Flag"]
  #[inline] pub fn stopf(&self) -> bits::B1 {
     (((self.0 as u8) >> 6) & 0x1).into() // [6]
  }
#[doc="I2C Bus Stop Detect Flag"]
  #[inline] pub fn set_stopf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Stop Hold Enable"]
  #[inline] pub fn shen(&self) -> bits::B1 {
     (((self.0 as u8) >> 7) & 0x1).into() // [7]
  }
#[doc="Stop Hold Enable"]
  #[inline] pub fn set_shen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
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
#[doc="I2C Range Address register"]
#[derive(PartialEq, Eq)]
pub struct Ra(pub u8);
impl Ra {
#[doc="Range Slave Address"]
  #[inline] pub fn rad(&self) -> bits::B7 {
     (((self.0 as u8) >> 1) & 0x7f).into() // [7:1]
  }
#[doc="Range Slave Address"]
  #[inline] pub fn set_rad<V: Into<bits::B7>>(mut self, value: V) -> Self {
     let value: bits::B7 = value.into();
     let value: u8 = value.into();
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
#[doc="I2C SMBus Control and Status register"]
#[derive(PartialEq, Eq)]
pub struct Smb(pub u8);
impl Smb {
#[doc="SHTF2 Interrupt Enable"]
  #[inline] pub fn shtf2ie(&self) -> bits::B1 {
     (((self.0 as u8) >> 0) & 0x1).into() // [0]
  }
#[doc="SHTF2 Interrupt Enable"]
  #[inline] pub fn set_shtf2ie<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="SCL High Timeout Flag 2"]
  #[inline] pub fn shtf2(&self) -> bits::B1 {
     (((self.0 as u8) >> 1) & 0x1).into() // [1]
  }
#[doc="SCL High Timeout Flag 2"]
  #[inline] pub fn set_shtf2<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="SCL High Timeout Flag 1"]
  #[inline] pub fn shtf1(&self) -> bits::B1 {
     (((self.0 as u8) >> 2) & 0x1).into() // [2]
  }
#[doc="SCL High Timeout Flag 1"]
  #[inline] pub fn set_shtf1<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="SCL Low Timeout Flag"]
  #[inline] pub fn sltf(&self) -> bits::B1 {
     (((self.0 as u8) >> 3) & 0x1).into() // [3]
  }
#[doc="SCL Low Timeout Flag"]
  #[inline] pub fn set_sltf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Timeout Counter Clock Select"]
  #[inline] pub fn tcksel(&self) -> bits::B1 {
     (((self.0 as u8) >> 4) & 0x1).into() // [4]
  }
#[doc="Timeout Counter Clock Select"]
  #[inline] pub fn set_tcksel<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Second I2C Address Enable"]
  #[inline] pub fn siicaen(&self) -> bits::B1 {
     (((self.0 as u8) >> 5) & 0x1).into() // [5]
  }
#[doc="Second I2C Address Enable"]
  #[inline] pub fn set_siicaen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="SMBus Alert Response Address Enable"]
  #[inline] pub fn alerten(&self) -> bits::B1 {
     (((self.0 as u8) >> 6) & 0x1).into() // [6]
  }
#[doc="SMBus Alert Response Address Enable"]
  #[inline] pub fn set_alerten<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Fast NACK/ACK Enable"]
  #[inline] pub fn fack(&self) -> bits::B1 {
     (((self.0 as u8) >> 7) & 0x1).into() // [7]
  }
#[doc="Fast NACK/ACK Enable"]
  #[inline] pub fn set_fack<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u8 = value.into();
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
#[doc="I2C Address Register 2"]
#[derive(PartialEq, Eq)]
pub struct A2(pub u8);
impl A2 {
#[doc="SMBus Address"]
  #[inline] pub fn sad(&self) -> bits::B7 {
     (((self.0 as u8) >> 1) & 0x7f).into() // [7:1]
  }
#[doc="SMBus Address"]
  #[inline] pub fn set_sad<V: Into<bits::B7>>(mut self, value: V) -> Self {
     let value: bits::B7 = value.into();
     let value: u8 = value.into();
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
#[doc="I2C SCL Low Timeout Register High"]
#[derive(PartialEq, Eq)]
pub struct Slth(pub u8);
impl Slth {
#[doc="Most significant byte of SCL low timeout value that determines the timeout period of SCL low."]
  #[inline] pub fn sslt(&self) -> bits::B8 {
     (((self.0 as u8) >> 0) & 0xff).into() // [7:0]
  }
#[doc="Most significant byte of SCL low timeout value that determines the timeout period of SCL low."]
  #[inline] pub fn set_sslt<V: Into<bits::B8>>(mut self, value: V) -> Self {
     let value: bits::B8 = value.into();
     let value: u8 = value.into();
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
#[doc="I2C SCL Low Timeout Register Low"]
#[derive(PartialEq, Eq)]
pub struct Sltl(pub u8);
impl Sltl {
#[doc="Least significant byte of SCL low timeout value that determines the timeout period of SCL low."]
  #[inline] pub fn sslt(&self) -> bits::B8 {
     (((self.0 as u8) >> 0) & 0xff).into() // [7:0]
  }
#[doc="Least significant byte of SCL low timeout value that determines the timeout period of SCL low."]
  #[inline] pub fn set_sslt<V: Into<bits::B8>>(mut self, value: V) -> Self {
     let value: bits::B8 = value.into();
     let value: u8 = value.into();
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
