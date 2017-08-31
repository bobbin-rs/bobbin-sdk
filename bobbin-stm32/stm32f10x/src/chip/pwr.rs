//! Power control
#[allow(unused_imports)] use bobbin_common::*;

periph!(PWR, Pwr, 0x40007000);

#[doc="Power control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pwr(pub usize);
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
         Cr(read_volatile((self.0 + 0x0) as *const u32))
      }
   }
#[doc="Write the CR register."]
   #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
      let value = f(Cr(0));
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CR register."]
   #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
      let tmp = self.cr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
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
         Csr(read_volatile((self.0 + 0x4) as *const u32))
      }
   }
#[doc="Write the CSR register."]
   #[inline] pub fn set_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Self {
      let value = f(Csr(0));
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CSR register."]
   #[inline] pub fn with_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Self {
      let tmp = self.csr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }

}

#[doc="Power control register (PWR_CR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
#[doc="Low Power Deep Sleep"]
   #[inline] pub fn lpds(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Low Power Deep Sleep"]
   #[inline] pub fn set_lpds<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Power Down Deep Sleep"]
   #[inline] pub fn pdds(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Power Down Deep Sleep"]
   #[inline] pub fn set_pdds<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Clear Wake-up Flag"]
   #[inline] pub fn cwuf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Clear Wake-up Flag"]
   #[inline] pub fn set_cwuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Clear STANDBY Flag"]
   #[inline] pub fn csbf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Clear STANDBY Flag"]
   #[inline] pub fn set_csbf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Power Voltage Detector Enable"]
   #[inline] pub fn pvde(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Power Voltage Detector Enable"]
   #[inline] pub fn set_pvde<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="PVD Level Selection"]
   #[inline] pub fn pls(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7) as u8) } // [7:5]
   }
#[doc="PVD Level Selection"]
   #[inline] pub fn set_pls<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Disable Backup Domain write protection"]
   #[inline] pub fn dbp(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Disable Backup Domain write protection"]
   #[inline] pub fn set_dbp<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
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
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Power control register (PWR_CR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr(pub u32);
impl Csr {
#[doc="Wake-Up Flag"]
   #[inline] pub fn wuf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Wake-Up Flag"]
   #[inline] pub fn set_wuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="STANDBY Flag"]
   #[inline] pub fn sbf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="STANDBY Flag"]
   #[inline] pub fn set_sbf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="PVD Output"]
   #[inline] pub fn pvdo(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="PVD Output"]
   #[inline] pub fn set_pvdo<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Enable WKUP pin"]
   #[inline] pub fn ewup(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Enable WKUP pin"]
   #[inline] pub fn set_ewup<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
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
      if self.wuf() != 0 { try!(write!(f, " wuf"))}
      if self.sbf() != 0 { try!(write!(f, " sbf"))}
      if self.pvdo() != 0 { try!(write!(f, " pvdo"))}
      if self.ewup() != 0 { try!(write!(f, " ewup"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

