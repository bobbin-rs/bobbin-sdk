//! FLASH
#[allow(unused_imports)] use bobbin_common::bits;
pub const FLASH: Flash = Flash(0x40023c00);

#[doc="FLASH"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Flash(pub u32);
impl Flash {
#[doc="Get the *const pointer for the ACR register."]
  #[inline] pub fn acr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the ACR register."]
  #[inline] pub fn acr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the ACR register."]
  #[inline] pub fn acr(&self) -> Acr { 
     unsafe {
        Acr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the ACR register."]
  #[inline] pub fn set_acr(&self, value: Acr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ACR register."]
  #[inline] pub fn with_acr<F: FnOnce(Acr) -> Acr>(&self, f: F) -> &Self {
     let tmp = self.acr();
     self.set_acr(f(tmp))
  }

#[doc="Get the *const pointer for the KEYR register."]
  #[inline] pub fn keyr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the KEYR register."]
  #[inline] pub fn keyr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Write the KEYR register."]
  #[inline] pub fn set_keyr(&self, value: Keyr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the OPTKEYR register."]
  #[inline] pub fn optkeyr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the OPTKEYR register."]
  #[inline] pub fn optkeyr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Write the OPTKEYR register."]
  #[inline] pub fn set_optkeyr(&self, value: Optkeyr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the SR register."]
  #[inline] pub fn sr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the SR register."]
  #[inline] pub fn sr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the SR register."]
  #[inline] pub fn sr(&self) -> Sr { 
     unsafe {
        Sr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the SR register."]
  #[inline] pub fn set_sr(&self, value: Sr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SR register."]
  #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
     let tmp = self.sr();
     self.set_sr(f(tmp))
  }

#[doc="Get the *const pointer for the CR register."]
  #[inline] pub fn cr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the CR register."]
  #[inline] pub fn cr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the CR register."]
  #[inline] pub fn cr(&self) -> Cr { 
     unsafe {
        Cr(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the CR register."]
  #[inline] pub fn set_cr(&self, value: Cr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CR register."]
  #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
     let tmp = self.cr();
     self.set_cr(f(tmp))
  }

#[doc="Get the *const pointer for the OPTCR register."]
  #[inline] pub fn optcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the OPTCR register."]
  #[inline] pub fn optcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Read the OPTCR register."]
  #[inline] pub fn optcr(&self) -> Optcr { 
     unsafe {
        Optcr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
#[doc="Write the OPTCR register."]
  #[inline] pub fn set_optcr(&self, value: Optcr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the OPTCR register."]
  #[inline] pub fn with_optcr<F: FnOnce(Optcr) -> Optcr>(&self, f: F) -> &Self {
     let tmp = self.optcr();
     self.set_optcr(f(tmp))
  }

}

#[doc="Flash access control register"]
#[derive(PartialEq, Eq)]
pub struct Acr(pub u32);
impl Acr {
#[doc="Latency"]
  #[inline] pub fn latency(&self) -> bits::B3 {
     (((self.0 as u32) >> 0) & 0x7).into() // [2:0]
  }
#[doc="Latency"]
  #[inline] pub fn set_latency<V: Into<bits::B3>>(mut self, value: V) -> Self {
     let value: bits::B3 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Prefetch enable"]
  #[inline] pub fn prften(&self) -> bits::B1 {
     (((self.0 as u32) >> 8) & 0x1).into() // [8]
  }
#[doc="Prefetch enable"]
  #[inline] pub fn set_prften<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Instruction cache enable"]
  #[inline] pub fn icen(&self) -> bits::B1 {
     (((self.0 as u32) >> 9) & 0x1).into() // [9]
  }
#[doc="Instruction cache enable"]
  #[inline] pub fn set_icen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Data cache enable"]
  #[inline] pub fn dcen(&self) -> bits::B1 {
     (((self.0 as u32) >> 10) & 0x1).into() // [10]
  }
#[doc="Data cache enable"]
  #[inline] pub fn set_dcen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Instruction cache reset"]
  #[inline] pub fn icrst(&self) -> bits::B1 {
     (((self.0 as u32) >> 11) & 0x1).into() // [11]
  }
#[doc="Instruction cache reset"]
  #[inline] pub fn set_icrst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Data cache reset"]
  #[inline] pub fn dcrst(&self) -> bits::B1 {
     (((self.0 as u32) >> 12) & 0x1).into() // [12]
  }
#[doc="Data cache reset"]
  #[inline] pub fn set_dcrst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

}
impl ::core::fmt::Display for Acr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Acr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.latency() != 0 { try!(write!(f, " latency=0x{:x}", self.latency()))}
      if self.prften() != 0 { try!(write!(f, " prften"))}
      if self.icen() != 0 { try!(write!(f, " icen"))}
      if self.dcen() != 0 { try!(write!(f, " dcen"))}
      if self.icrst() != 0 { try!(write!(f, " icrst"))}
      if self.dcrst() != 0 { try!(write!(f, " dcrst"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Flash key register"]
#[derive(PartialEq, Eq)]
pub struct Keyr(pub u32);
impl Keyr {
#[doc="FPEC key"]
  #[inline] pub fn key(&self) -> bits::B32 {
     (((self.0 as u32) >> 0) & 0xffffffff).into() // [31:0]
  }
#[doc="FPEC key"]
  #[inline] pub fn set_key<V: Into<bits::B32>>(mut self, value: V) -> Self {
     let value: bits::B32 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Keyr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Keyr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Flash option key register"]
#[derive(PartialEq, Eq)]
pub struct Optkeyr(pub u32);
impl Optkeyr {
#[doc="Option byte key"]
  #[inline] pub fn optkey(&self) -> bits::B32 {
     (((self.0 as u32) >> 0) & 0xffffffff).into() // [31:0]
  }
#[doc="Option byte key"]
  #[inline] pub fn set_optkey<V: Into<bits::B32>>(mut self, value: V) -> Self {
     let value: bits::B32 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Optkeyr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Optkeyr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Status register"]
#[derive(PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
#[doc="End of operation"]
  #[inline] pub fn eop(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="End of operation"]
  #[inline] pub fn set_eop<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Operation error"]
  #[inline] pub fn operr(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="Operation error"]
  #[inline] pub fn set_operr<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Write protection error"]
  #[inline] pub fn wrperr(&self) -> bits::B1 {
     (((self.0 as u32) >> 4) & 0x1).into() // [4]
  }
#[doc="Write protection error"]
  #[inline] pub fn set_wrperr<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Programming alignment error"]
  #[inline] pub fn pgaerr(&self) -> bits::B1 {
     (((self.0 as u32) >> 5) & 0x1).into() // [5]
  }
#[doc="Programming alignment error"]
  #[inline] pub fn set_pgaerr<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Programming parallelism error"]
  #[inline] pub fn pgperr(&self) -> bits::B1 {
     (((self.0 as u32) >> 6) & 0x1).into() // [6]
  }
#[doc="Programming parallelism error"]
  #[inline] pub fn set_pgperr<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Programming sequence error"]
  #[inline] pub fn pgserr(&self) -> bits::B1 {
     (((self.0 as u32) >> 7) & 0x1).into() // [7]
  }
#[doc="Programming sequence error"]
  #[inline] pub fn set_pgserr<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Busy"]
  #[inline] pub fn bsy(&self) -> bits::B1 {
     (((self.0 as u32) >> 16) & 0x1).into() // [16]
  }
#[doc="Busy"]
  #[inline] pub fn set_bsy<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Sr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.eop() != 0 { try!(write!(f, " eop"))}
      if self.operr() != 0 { try!(write!(f, " operr"))}
      if self.wrperr() != 0 { try!(write!(f, " wrperr"))}
      if self.pgaerr() != 0 { try!(write!(f, " pgaerr"))}
      if self.pgperr() != 0 { try!(write!(f, " pgperr"))}
      if self.pgserr() != 0 { try!(write!(f, " pgserr"))}
      if self.bsy() != 0 { try!(write!(f, " bsy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control register"]
#[derive(PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
#[doc="Programming"]
  #[inline] pub fn pg(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Programming"]
  #[inline] pub fn set_pg<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Sector Erase"]
  #[inline] pub fn ser(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="Sector Erase"]
  #[inline] pub fn set_ser<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Mass Erase"]
  #[inline] pub fn mer(&self) -> bits::B1 {
     (((self.0 as u32) >> 2) & 0x1).into() // [2]
  }
#[doc="Mass Erase"]
  #[inline] pub fn set_mer<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Sector number"]
  #[inline] pub fn snb(&self) -> bits::B4 {
     (((self.0 as u32) >> 3) & 0xf).into() // [6:3]
  }
#[doc="Sector number"]
  #[inline] pub fn set_snb<V: Into<bits::B4>>(mut self, value: V) -> Self {
     let value: bits::B4 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Program size"]
  #[inline] pub fn psize(&self) -> bits::B2 {
     (((self.0 as u32) >> 8) & 0x3).into() // [9:8]
  }
#[doc="Program size"]
  #[inline] pub fn set_psize<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Start"]
  #[inline] pub fn strt(&self) -> bits::B1 {
     (((self.0 as u32) >> 16) & 0x1).into() // [16]
  }
#[doc="Start"]
  #[inline] pub fn set_strt<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="End of operation interrupt enable"]
  #[inline] pub fn eopie(&self) -> bits::B1 {
     (((self.0 as u32) >> 24) & 0x1).into() // [24]
  }
#[doc="End of operation interrupt enable"]
  #[inline] pub fn set_eopie<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Error interrupt enable"]
  #[inline] pub fn errie(&self) -> bits::B1 {
     (((self.0 as u32) >> 25) & 0x1).into() // [25]
  }
#[doc="Error interrupt enable"]
  #[inline] pub fn set_errie<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="Lock"]
  #[inline] pub fn lock(&self) -> bits::B1 {
     (((self.0 as u32) >> 31) & 0x1).into() // [31]
  }
#[doc="Lock"]
  #[inline] pub fn set_lock<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Cr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pg() != 0 { try!(write!(f, " pg"))}
      if self.ser() != 0 { try!(write!(f, " ser"))}
      if self.mer() != 0 { try!(write!(f, " mer"))}
      if self.snb() != 0 { try!(write!(f, " snb=0x{:x}", self.snb()))}
      if self.psize() != 0 { try!(write!(f, " psize=0x{:x}", self.psize()))}
      if self.strt() != 0 { try!(write!(f, " strt"))}
      if self.eopie() != 0 { try!(write!(f, " eopie"))}
      if self.errie() != 0 { try!(write!(f, " errie"))}
      if self.lock() != 0 { try!(write!(f, " lock"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Flash option control register"]
#[derive(PartialEq, Eq)]
pub struct Optcr(pub u32);
impl Optcr {
#[doc="Option lock"]
  #[inline] pub fn optlock(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Option lock"]
  #[inline] pub fn set_optlock<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Option start"]
  #[inline] pub fn optstrt(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="Option start"]
  #[inline] pub fn set_optstrt<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="BOR reset Level"]
  #[inline] pub fn bor_lev(&self) -> bits::B2 {
     (((self.0 as u32) >> 2) & 0x3).into() // [3:2]
  }
#[doc="BOR reset Level"]
  #[inline] pub fn set_bor_lev<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="WDG_SW User option bytes"]
  #[inline] pub fn wdg_sw(&self) -> bits::B1 {
     (((self.0 as u32) >> 5) & 0x1).into() // [5]
  }
#[doc="WDG_SW User option bytes"]
  #[inline] pub fn set_wdg_sw<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="nRST_STOP User option bytes"]
  #[inline] pub fn nrst_stop(&self) -> bits::B1 {
     (((self.0 as u32) >> 6) & 0x1).into() // [6]
  }
#[doc="nRST_STOP User option bytes"]
  #[inline] pub fn set_nrst_stop<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="nRST_STDBY User option bytes"]
  #[inline] pub fn nrst_stdby(&self) -> bits::B1 {
     (((self.0 as u32) >> 7) & 0x1).into() // [7]
  }
#[doc="nRST_STDBY User option bytes"]
  #[inline] pub fn set_nrst_stdby<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Read protect"]
  #[inline] pub fn rdp(&self) -> bits::B8 {
     (((self.0 as u32) >> 8) & 0xff).into() // [15:8]
  }
#[doc="Read protect"]
  #[inline] pub fn set_rdp<V: Into<bits::B8>>(mut self, value: V) -> Self {
     let value: bits::B8 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Not write protect"]
  #[inline] pub fn nwrp(&self) -> bits::B12 {
     (((self.0 as u32) >> 16) & 0xfff).into() // [27:16]
  }
#[doc="Not write protect"]
  #[inline] pub fn set_nwrp<V: Into<bits::B12>>(mut self, value: V) -> Self {
     let value: bits::B12 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Optcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Optcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.optlock() != 0 { try!(write!(f, " optlock"))}
      if self.optstrt() != 0 { try!(write!(f, " optstrt"))}
      if self.bor_lev() != 0 { try!(write!(f, " bor_lev=0x{:x}", self.bor_lev()))}
      if self.wdg_sw() != 0 { try!(write!(f, " wdg_sw"))}
      if self.nrst_stop() != 0 { try!(write!(f, " nrst_stop"))}
      if self.nrst_stdby() != 0 { try!(write!(f, " nrst_stdby"))}
      if self.rdp() != 0 { try!(write!(f, " rdp=0x{:x}", self.rdp()))}
      if self.nwrp() != 0 { try!(write!(f, " nwrp=0x{:x}", self.nwrp()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

