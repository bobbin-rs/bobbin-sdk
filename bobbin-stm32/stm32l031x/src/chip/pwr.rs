//! Power control
#[allow(unused_imports)] use bobbin_common::bits;
pub const PWR: Pwr = Pwr(0x40007000);

#[doc="Power control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pwr(pub u32);
impl Pwr {
#[doc="Get the *const pointer for the CR register."]
  #[inline] pub fn cr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the CR register."]
  #[inline] pub fn cr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the CR register."]
  #[inline] pub fn cr(&self) -> Cr { 
     unsafe {
        Cr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the CR register."]
  #[inline] pub fn set_cr(&self, value: Cr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CR register."]
  #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
     let tmp = self.cr();
     self.set_cr(f(tmp))
  }

#[doc="Get the *const pointer for the CSR register."]
  #[inline] pub fn csr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the CSR register."]
  #[inline] pub fn csr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the CSR register."]
  #[inline] pub fn csr(&self) -> Csr { 
     unsafe {
        Csr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the CSR register."]
  #[inline] pub fn set_csr(&self, value: Csr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CSR register."]
  #[inline] pub fn with_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Self {
     let tmp = self.csr();
     self.set_csr(f(tmp))
  }

}

#[doc="power control register"]
#[derive(PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
#[doc="Low-power deep sleep"]
  #[inline] pub fn lpds(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Low-power deep sleep"]
  #[inline] pub fn set_lpds<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Power down deepsleep"]
  #[inline] pub fn pdds(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="Power down deepsleep"]
  #[inline] pub fn set_pdds<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Clear wakeup flag"]
  #[inline] pub fn cwuf(&self) -> bits::B1 {
     (((self.0 as u32) >> 2) & 0x1).into() // [2]
  }
#[doc="Clear wakeup flag"]
  #[inline] pub fn set_cwuf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Clear standby flag"]
  #[inline] pub fn csbf(&self) -> bits::B1 {
     (((self.0 as u32) >> 3) & 0x1).into() // [3]
  }
#[doc="Clear standby flag"]
  #[inline] pub fn set_csbf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Power voltage detector enable"]
  #[inline] pub fn pvde(&self) -> bits::B1 {
     (((self.0 as u32) >> 4) & 0x1).into() // [4]
  }
#[doc="Power voltage detector enable"]
  #[inline] pub fn set_pvde<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="PVD level selection"]
  #[inline] pub fn pls(&self) -> bits::B3 {
     (((self.0 as u32) >> 5) & 0x7).into() // [7:5]
  }
#[doc="PVD level selection"]
  #[inline] pub fn set_pls<V: Into<bits::B3>>(mut self, value: V) -> Self {
     let value: bits::B3 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Disable backup domain write protection"]
  #[inline] pub fn dbp(&self) -> bits::B1 {
     (((self.0 as u32) >> 8) & 0x1).into() // [8]
  }
#[doc="Disable backup domain write protection"]
  #[inline] pub fn set_dbp<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Flash power down in Stop mode"]
  #[inline] pub fn fpds(&self) -> bits::B1 {
     (((self.0 as u32) >> 9) & 0x1).into() // [9]
  }
#[doc="Flash power down in Stop mode"]
  #[inline] pub fn set_fpds<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Fast wakeup"]
  #[inline] pub fn fwu(&self) -> bits::B1 {
     (((self.0 as u32) >> 10) & 0x1).into() // [10]
  }
#[doc="Fast wakeup"]
  #[inline] pub fn set_fwu<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Voltage scaling range selection"]
  #[inline] pub fn vos(&self) -> bits::B2 {
     (((self.0 as u32) >> 11) & 0x3).into() // [12:11]
  }
#[doc="Voltage scaling range selection"]
  #[inline] pub fn set_vos<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Deep sleep mode with Flash memory kept off"]
  #[inline] pub fn ds_ee_koff(&self) -> bits::B1 {
     (((self.0 as u32) >> 13) & 0x1).into() // [13]
  }
#[doc="Deep sleep mode with Flash memory kept off"]
  #[inline] pub fn set_ds_ee_koff<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Low power run mode"]
  #[inline] pub fn lprun(&self) -> bits::B1 {
     (((self.0 as u32) >> 14) & 0x1).into() // [14]
  }
#[doc="Low power run mode"]
  #[inline] pub fn set_lprun<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
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
      if self.lpds() != 0 { try!(write!(f, " lpds"))}
      if self.pdds() != 0 { try!(write!(f, " pdds"))}
      if self.cwuf() != 0 { try!(write!(f, " cwuf"))}
      if self.csbf() != 0 { try!(write!(f, " csbf"))}
      if self.pvde() != 0 { try!(write!(f, " pvde"))}
      if self.pls() != 0 { try!(write!(f, " pls=0x{:x}", self.pls()))}
      if self.dbp() != 0 { try!(write!(f, " dbp"))}
      if self.fpds() != 0 { try!(write!(f, " fpds"))}
      if self.fwu() != 0 { try!(write!(f, " fwu"))}
      if self.vos() != 0 { try!(write!(f, " vos=0x{:x}", self.vos()))}
      if self.ds_ee_koff() != 0 { try!(write!(f, " ds_ee_koff"))}
      if self.lprun() != 0 { try!(write!(f, " lprun"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="power control/status register"]
#[derive(PartialEq, Eq)]
pub struct Csr(pub u32);
impl Csr {
#[doc="Backup regulator enable"]
  #[inline] pub fn bre(&self) -> bits::B1 {
     (((self.0 as u32) >> 9) & 0x1).into() // [9]
  }
#[doc="Backup regulator enable"]
  #[inline] pub fn set_bre<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Enable WKUP pin"]
  #[inline] pub fn ewup(&self) -> bits::B1 {
     (((self.0 as u32) >> 8) & 0x1).into() // [8]
  }
#[doc="Enable WKUP pin"]
  #[inline] pub fn set_ewup<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Backup regulator ready"]
  #[inline] pub fn brr(&self) -> bits::B1 {
     (((self.0 as u32) >> 3) & 0x1).into() // [3]
  }
#[doc="Backup regulator ready"]
  #[inline] pub fn set_brr<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="PVD output"]
  #[inline] pub fn pvdo(&self) -> bits::B1 {
     (((self.0 as u32) >> 2) & 0x1).into() // [2]
  }
#[doc="PVD output"]
  #[inline] pub fn set_pvdo<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Standby flag"]
  #[inline] pub fn sbf(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="Standby flag"]
  #[inline] pub fn set_sbf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Wakeup flag"]
  #[inline] pub fn wuf(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Wakeup flag"]
  #[inline] pub fn set_wuf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Voltage Scaling select flag"]
  #[inline] pub fn vosf(&self) -> bits::B1 {
     (((self.0 as u32) >> 4) & 0x1).into() // [4]
  }
#[doc="Voltage Scaling select flag"]
  #[inline] pub fn set_vosf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Regulator LP flag"]
  #[inline] pub fn reglpf(&self) -> bits::B1 {
     (((self.0 as u32) >> 5) & 0x1).into() // [5]
  }
#[doc="Regulator LP flag"]
  #[inline] pub fn set_reglpf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
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
      if self.bre() != 0 { try!(write!(f, " bre"))}
      if self.ewup() != 0 { try!(write!(f, " ewup"))}
      if self.brr() != 0 { try!(write!(f, " brr"))}
      if self.pvdo() != 0 { try!(write!(f, " pvdo"))}
      if self.sbf() != 0 { try!(write!(f, " sbf"))}
      if self.wuf() != 0 { try!(write!(f, " wuf"))}
      if self.vosf() != 0 { try!(write!(f, " vosf"))}
      if self.reglpf() != 0 { try!(write!(f, " reglpf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

