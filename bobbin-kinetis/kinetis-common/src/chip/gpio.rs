#[allow(unused_imports)] use bobbin_common::bits;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="GPIO Peripheral"]
pub struct Periph<T>(pub u32, pub T); 



impl<T> Periph<T> {
#[doc="Get the *const pointer for the PDOR register."]
  #[inline] pub fn pdor_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the PDOR register."]
  #[inline] pub fn pdor_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the PDOR register."]
  #[inline] pub fn pdor(&self) -> Pdor { 
     unsafe {
        Pdor(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the PDOR register."]
  #[inline] pub fn set_pdor<F: FnOnce(Pdor) -> Pdor>(&self, f: F) -> &Self {
     let value = f(Pdor(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PDOR register."]
  #[inline] pub fn with_pdor<F: FnOnce(Pdor) -> Pdor>(&self, f: F) -> &Self {
     let tmp = self.pdor();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the PSOR register."]
  #[inline] pub fn psor_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the PSOR register."]
  #[inline] pub fn psor_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Write the PSOR register."]
  #[inline] pub fn set_psor<F: FnOnce(Psor) -> Psor>(&self, f: F) -> &Self {
     let value = f(Psor(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the PCOR register."]
  #[inline] pub fn pcor_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the PCOR register."]
  #[inline] pub fn pcor_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Write the PCOR register."]
  #[inline] pub fn set_pcor<F: FnOnce(Pcor) -> Pcor>(&self, f: F) -> &Self {
     let value = f(Pcor(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the PTOR register."]
  #[inline] pub fn ptor_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the PTOR register."]
  #[inline] pub fn ptor_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Write the PTOR register."]
  #[inline] pub fn set_ptor<F: FnOnce(Ptor) -> Ptor>(&self, f: F) -> &Self {
     let value = f(Ptor(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the PDIR register."]
  #[inline] pub fn pdir_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the PDIR register."]
  #[inline] pub fn pdir_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the PDIR register."]
  #[inline] pub fn pdir(&self) -> Pdir { 
     unsafe {
        Pdir(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }

#[doc="Get the *const pointer for the PDDR register."]
  #[inline] pub fn pddr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the PDDR register."]
  #[inline] pub fn pddr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Read the PDDR register."]
  #[inline] pub fn pddr(&self) -> Pddr { 
     unsafe {
        Pddr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
#[doc="Write the PDDR register."]
  #[inline] pub fn set_pddr<F: FnOnce(Pddr) -> Pddr>(&self, f: F) -> &Self {
     let value = f(Pddr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PDDR register."]
  #[inline] pub fn with_pddr<F: FnOnce(Pddr) -> Pddr>(&self, f: F) -> &Self {
     let tmp = self.pddr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }

}

#[doc="Port Data Output Register"]
#[derive(PartialEq, Eq)]
pub struct Pdor(pub u32);
impl Pdor {
#[doc="Port Data Output"]
  #[inline] pub fn pdo<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Port Data Output"]
  #[inline] pub fn set_pdo<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     assert!(index < 32);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Pdor {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pdor {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pdo(0) != 0 { try!(write!(f, " pdo[0]"))}
      if self.pdo(1) != 0 { try!(write!(f, " pdo[1]"))}
      if self.pdo(2) != 0 { try!(write!(f, " pdo[2]"))}
      if self.pdo(3) != 0 { try!(write!(f, " pdo[3]"))}
      if self.pdo(4) != 0 { try!(write!(f, " pdo[4]"))}
      if self.pdo(5) != 0 { try!(write!(f, " pdo[5]"))}
      if self.pdo(6) != 0 { try!(write!(f, " pdo[6]"))}
      if self.pdo(7) != 0 { try!(write!(f, " pdo[7]"))}
      if self.pdo(8) != 0 { try!(write!(f, " pdo[8]"))}
      if self.pdo(9) != 0 { try!(write!(f, " pdo[9]"))}
      if self.pdo(10) != 0 { try!(write!(f, " pdo[10]"))}
      if self.pdo(11) != 0 { try!(write!(f, " pdo[11]"))}
      if self.pdo(12) != 0 { try!(write!(f, " pdo[12]"))}
      if self.pdo(13) != 0 { try!(write!(f, " pdo[13]"))}
      if self.pdo(14) != 0 { try!(write!(f, " pdo[14]"))}
      if self.pdo(15) != 0 { try!(write!(f, " pdo[15]"))}
      if self.pdo(16) != 0 { try!(write!(f, " pdo[16]"))}
      if self.pdo(17) != 0 { try!(write!(f, " pdo[17]"))}
      if self.pdo(18) != 0 { try!(write!(f, " pdo[18]"))}
      if self.pdo(19) != 0 { try!(write!(f, " pdo[19]"))}
      if self.pdo(20) != 0 { try!(write!(f, " pdo[20]"))}
      if self.pdo(21) != 0 { try!(write!(f, " pdo[21]"))}
      if self.pdo(22) != 0 { try!(write!(f, " pdo[22]"))}
      if self.pdo(23) != 0 { try!(write!(f, " pdo[23]"))}
      if self.pdo(24) != 0 { try!(write!(f, " pdo[24]"))}
      if self.pdo(25) != 0 { try!(write!(f, " pdo[25]"))}
      if self.pdo(26) != 0 { try!(write!(f, " pdo[26]"))}
      if self.pdo(27) != 0 { try!(write!(f, " pdo[27]"))}
      if self.pdo(28) != 0 { try!(write!(f, " pdo[28]"))}
      if self.pdo(29) != 0 { try!(write!(f, " pdo[29]"))}
      if self.pdo(30) != 0 { try!(write!(f, " pdo[30]"))}
      if self.pdo(31) != 0 { try!(write!(f, " pdo[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Port Set Output Register"]
#[derive(PartialEq, Eq)]
pub struct Psor(pub u32);
impl Psor {
#[doc="Port Set Output"]
  #[inline] pub fn ptso<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Port Set Output"]
  #[inline] pub fn set_ptso<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     assert!(index < 32);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Psor {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Psor {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ptso(0) != 0 { try!(write!(f, " ptso[0]"))}
      if self.ptso(1) != 0 { try!(write!(f, " ptso[1]"))}
      if self.ptso(2) != 0 { try!(write!(f, " ptso[2]"))}
      if self.ptso(3) != 0 { try!(write!(f, " ptso[3]"))}
      if self.ptso(4) != 0 { try!(write!(f, " ptso[4]"))}
      if self.ptso(5) != 0 { try!(write!(f, " ptso[5]"))}
      if self.ptso(6) != 0 { try!(write!(f, " ptso[6]"))}
      if self.ptso(7) != 0 { try!(write!(f, " ptso[7]"))}
      if self.ptso(8) != 0 { try!(write!(f, " ptso[8]"))}
      if self.ptso(9) != 0 { try!(write!(f, " ptso[9]"))}
      if self.ptso(10) != 0 { try!(write!(f, " ptso[10]"))}
      if self.ptso(11) != 0 { try!(write!(f, " ptso[11]"))}
      if self.ptso(12) != 0 { try!(write!(f, " ptso[12]"))}
      if self.ptso(13) != 0 { try!(write!(f, " ptso[13]"))}
      if self.ptso(14) != 0 { try!(write!(f, " ptso[14]"))}
      if self.ptso(15) != 0 { try!(write!(f, " ptso[15]"))}
      if self.ptso(16) != 0 { try!(write!(f, " ptso[16]"))}
      if self.ptso(17) != 0 { try!(write!(f, " ptso[17]"))}
      if self.ptso(18) != 0 { try!(write!(f, " ptso[18]"))}
      if self.ptso(19) != 0 { try!(write!(f, " ptso[19]"))}
      if self.ptso(20) != 0 { try!(write!(f, " ptso[20]"))}
      if self.ptso(21) != 0 { try!(write!(f, " ptso[21]"))}
      if self.ptso(22) != 0 { try!(write!(f, " ptso[22]"))}
      if self.ptso(23) != 0 { try!(write!(f, " ptso[23]"))}
      if self.ptso(24) != 0 { try!(write!(f, " ptso[24]"))}
      if self.ptso(25) != 0 { try!(write!(f, " ptso[25]"))}
      if self.ptso(26) != 0 { try!(write!(f, " ptso[26]"))}
      if self.ptso(27) != 0 { try!(write!(f, " ptso[27]"))}
      if self.ptso(28) != 0 { try!(write!(f, " ptso[28]"))}
      if self.ptso(29) != 0 { try!(write!(f, " ptso[29]"))}
      if self.ptso(30) != 0 { try!(write!(f, " ptso[30]"))}
      if self.ptso(31) != 0 { try!(write!(f, " ptso[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Port Clear Output Register"]
#[derive(PartialEq, Eq)]
pub struct Pcor(pub u32);
impl Pcor {
#[doc="Port Clear Output"]
  #[inline] pub fn ptco<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Port Clear Output"]
  #[inline] pub fn set_ptco<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     assert!(index < 32);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Pcor {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pcor {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ptco(0) != 0 { try!(write!(f, " ptco[0]"))}
      if self.ptco(1) != 0 { try!(write!(f, " ptco[1]"))}
      if self.ptco(2) != 0 { try!(write!(f, " ptco[2]"))}
      if self.ptco(3) != 0 { try!(write!(f, " ptco[3]"))}
      if self.ptco(4) != 0 { try!(write!(f, " ptco[4]"))}
      if self.ptco(5) != 0 { try!(write!(f, " ptco[5]"))}
      if self.ptco(6) != 0 { try!(write!(f, " ptco[6]"))}
      if self.ptco(7) != 0 { try!(write!(f, " ptco[7]"))}
      if self.ptco(8) != 0 { try!(write!(f, " ptco[8]"))}
      if self.ptco(9) != 0 { try!(write!(f, " ptco[9]"))}
      if self.ptco(10) != 0 { try!(write!(f, " ptco[10]"))}
      if self.ptco(11) != 0 { try!(write!(f, " ptco[11]"))}
      if self.ptco(12) != 0 { try!(write!(f, " ptco[12]"))}
      if self.ptco(13) != 0 { try!(write!(f, " ptco[13]"))}
      if self.ptco(14) != 0 { try!(write!(f, " ptco[14]"))}
      if self.ptco(15) != 0 { try!(write!(f, " ptco[15]"))}
      if self.ptco(16) != 0 { try!(write!(f, " ptco[16]"))}
      if self.ptco(17) != 0 { try!(write!(f, " ptco[17]"))}
      if self.ptco(18) != 0 { try!(write!(f, " ptco[18]"))}
      if self.ptco(19) != 0 { try!(write!(f, " ptco[19]"))}
      if self.ptco(20) != 0 { try!(write!(f, " ptco[20]"))}
      if self.ptco(21) != 0 { try!(write!(f, " ptco[21]"))}
      if self.ptco(22) != 0 { try!(write!(f, " ptco[22]"))}
      if self.ptco(23) != 0 { try!(write!(f, " ptco[23]"))}
      if self.ptco(24) != 0 { try!(write!(f, " ptco[24]"))}
      if self.ptco(25) != 0 { try!(write!(f, " ptco[25]"))}
      if self.ptco(26) != 0 { try!(write!(f, " ptco[26]"))}
      if self.ptco(27) != 0 { try!(write!(f, " ptco[27]"))}
      if self.ptco(28) != 0 { try!(write!(f, " ptco[28]"))}
      if self.ptco(29) != 0 { try!(write!(f, " ptco[29]"))}
      if self.ptco(30) != 0 { try!(write!(f, " ptco[30]"))}
      if self.ptco(31) != 0 { try!(write!(f, " ptco[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Port Toggle Output Register"]
#[derive(PartialEq, Eq)]
pub struct Ptor(pub u32);
impl Ptor {
#[doc="Port Toggle Output"]
  #[inline] pub fn ptto<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Port Toggle Output"]
  #[inline] pub fn set_ptto<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     assert!(index < 32);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Ptor {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ptor {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ptto(0) != 0 { try!(write!(f, " ptto[0]"))}
      if self.ptto(1) != 0 { try!(write!(f, " ptto[1]"))}
      if self.ptto(2) != 0 { try!(write!(f, " ptto[2]"))}
      if self.ptto(3) != 0 { try!(write!(f, " ptto[3]"))}
      if self.ptto(4) != 0 { try!(write!(f, " ptto[4]"))}
      if self.ptto(5) != 0 { try!(write!(f, " ptto[5]"))}
      if self.ptto(6) != 0 { try!(write!(f, " ptto[6]"))}
      if self.ptto(7) != 0 { try!(write!(f, " ptto[7]"))}
      if self.ptto(8) != 0 { try!(write!(f, " ptto[8]"))}
      if self.ptto(9) != 0 { try!(write!(f, " ptto[9]"))}
      if self.ptto(10) != 0 { try!(write!(f, " ptto[10]"))}
      if self.ptto(11) != 0 { try!(write!(f, " ptto[11]"))}
      if self.ptto(12) != 0 { try!(write!(f, " ptto[12]"))}
      if self.ptto(13) != 0 { try!(write!(f, " ptto[13]"))}
      if self.ptto(14) != 0 { try!(write!(f, " ptto[14]"))}
      if self.ptto(15) != 0 { try!(write!(f, " ptto[15]"))}
      if self.ptto(16) != 0 { try!(write!(f, " ptto[16]"))}
      if self.ptto(17) != 0 { try!(write!(f, " ptto[17]"))}
      if self.ptto(18) != 0 { try!(write!(f, " ptto[18]"))}
      if self.ptto(19) != 0 { try!(write!(f, " ptto[19]"))}
      if self.ptto(20) != 0 { try!(write!(f, " ptto[20]"))}
      if self.ptto(21) != 0 { try!(write!(f, " ptto[21]"))}
      if self.ptto(22) != 0 { try!(write!(f, " ptto[22]"))}
      if self.ptto(23) != 0 { try!(write!(f, " ptto[23]"))}
      if self.ptto(24) != 0 { try!(write!(f, " ptto[24]"))}
      if self.ptto(25) != 0 { try!(write!(f, " ptto[25]"))}
      if self.ptto(26) != 0 { try!(write!(f, " ptto[26]"))}
      if self.ptto(27) != 0 { try!(write!(f, " ptto[27]"))}
      if self.ptto(28) != 0 { try!(write!(f, " ptto[28]"))}
      if self.ptto(29) != 0 { try!(write!(f, " ptto[29]"))}
      if self.ptto(30) != 0 { try!(write!(f, " ptto[30]"))}
      if self.ptto(31) != 0 { try!(write!(f, " ptto[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Port Data Input Register"]
#[derive(PartialEq, Eq)]
pub struct Pdir(pub u32);
impl Pdir {
#[doc="Port Data Input"]
  #[inline] pub fn pdi<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Port Data Input"]
  #[inline] pub fn set_pdi<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     assert!(index < 32);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Pdir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pdir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pdi(0) != 0 { try!(write!(f, " pdi[0]"))}
      if self.pdi(1) != 0 { try!(write!(f, " pdi[1]"))}
      if self.pdi(2) != 0 { try!(write!(f, " pdi[2]"))}
      if self.pdi(3) != 0 { try!(write!(f, " pdi[3]"))}
      if self.pdi(4) != 0 { try!(write!(f, " pdi[4]"))}
      if self.pdi(5) != 0 { try!(write!(f, " pdi[5]"))}
      if self.pdi(6) != 0 { try!(write!(f, " pdi[6]"))}
      if self.pdi(7) != 0 { try!(write!(f, " pdi[7]"))}
      if self.pdi(8) != 0 { try!(write!(f, " pdi[8]"))}
      if self.pdi(9) != 0 { try!(write!(f, " pdi[9]"))}
      if self.pdi(10) != 0 { try!(write!(f, " pdi[10]"))}
      if self.pdi(11) != 0 { try!(write!(f, " pdi[11]"))}
      if self.pdi(12) != 0 { try!(write!(f, " pdi[12]"))}
      if self.pdi(13) != 0 { try!(write!(f, " pdi[13]"))}
      if self.pdi(14) != 0 { try!(write!(f, " pdi[14]"))}
      if self.pdi(15) != 0 { try!(write!(f, " pdi[15]"))}
      if self.pdi(16) != 0 { try!(write!(f, " pdi[16]"))}
      if self.pdi(17) != 0 { try!(write!(f, " pdi[17]"))}
      if self.pdi(18) != 0 { try!(write!(f, " pdi[18]"))}
      if self.pdi(19) != 0 { try!(write!(f, " pdi[19]"))}
      if self.pdi(20) != 0 { try!(write!(f, " pdi[20]"))}
      if self.pdi(21) != 0 { try!(write!(f, " pdi[21]"))}
      if self.pdi(22) != 0 { try!(write!(f, " pdi[22]"))}
      if self.pdi(23) != 0 { try!(write!(f, " pdi[23]"))}
      if self.pdi(24) != 0 { try!(write!(f, " pdi[24]"))}
      if self.pdi(25) != 0 { try!(write!(f, " pdi[25]"))}
      if self.pdi(26) != 0 { try!(write!(f, " pdi[26]"))}
      if self.pdi(27) != 0 { try!(write!(f, " pdi[27]"))}
      if self.pdi(28) != 0 { try!(write!(f, " pdi[28]"))}
      if self.pdi(29) != 0 { try!(write!(f, " pdi[29]"))}
      if self.pdi(30) != 0 { try!(write!(f, " pdi[30]"))}
      if self.pdi(31) != 0 { try!(write!(f, " pdi[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Port Data Direction Register"]
#[derive(PartialEq, Eq)]
pub struct Pddr(pub u32);
impl Pddr {
#[doc="Port Data Direction"]
  #[inline] pub fn pdd<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Port Data Direction"]
  #[inline] pub fn set_pdd<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     assert!(index < 32);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Pddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pdd(0) != 0 { try!(write!(f, " pdd[0]"))}
      if self.pdd(1) != 0 { try!(write!(f, " pdd[1]"))}
      if self.pdd(2) != 0 { try!(write!(f, " pdd[2]"))}
      if self.pdd(3) != 0 { try!(write!(f, " pdd[3]"))}
      if self.pdd(4) != 0 { try!(write!(f, " pdd[4]"))}
      if self.pdd(5) != 0 { try!(write!(f, " pdd[5]"))}
      if self.pdd(6) != 0 { try!(write!(f, " pdd[6]"))}
      if self.pdd(7) != 0 { try!(write!(f, " pdd[7]"))}
      if self.pdd(8) != 0 { try!(write!(f, " pdd[8]"))}
      if self.pdd(9) != 0 { try!(write!(f, " pdd[9]"))}
      if self.pdd(10) != 0 { try!(write!(f, " pdd[10]"))}
      if self.pdd(11) != 0 { try!(write!(f, " pdd[11]"))}
      if self.pdd(12) != 0 { try!(write!(f, " pdd[12]"))}
      if self.pdd(13) != 0 { try!(write!(f, " pdd[13]"))}
      if self.pdd(14) != 0 { try!(write!(f, " pdd[14]"))}
      if self.pdd(15) != 0 { try!(write!(f, " pdd[15]"))}
      if self.pdd(16) != 0 { try!(write!(f, " pdd[16]"))}
      if self.pdd(17) != 0 { try!(write!(f, " pdd[17]"))}
      if self.pdd(18) != 0 { try!(write!(f, " pdd[18]"))}
      if self.pdd(19) != 0 { try!(write!(f, " pdd[19]"))}
      if self.pdd(20) != 0 { try!(write!(f, " pdd[20]"))}
      if self.pdd(21) != 0 { try!(write!(f, " pdd[21]"))}
      if self.pdd(22) != 0 { try!(write!(f, " pdd[22]"))}
      if self.pdd(23) != 0 { try!(write!(f, " pdd[23]"))}
      if self.pdd(24) != 0 { try!(write!(f, " pdd[24]"))}
      if self.pdd(25) != 0 { try!(write!(f, " pdd[25]"))}
      if self.pdd(26) != 0 { try!(write!(f, " pdd[26]"))}
      if self.pdd(27) != 0 { try!(write!(f, " pdd[27]"))}
      if self.pdd(28) != 0 { try!(write!(f, " pdd[28]"))}
      if self.pdd(29) != 0 { try!(write!(f, " pdd[29]"))}
      if self.pdd(30) != 0 { try!(write!(f, " pdd[30]"))}
      if self.pdd(31) != 0 { try!(write!(f, " pdd[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO Pin"]
pub struct Pin<P, T> { pub port: Periph<T>, pub index: usize, pub id: P }

impl<P,T> Pin<P,T> {
   #[inline] pub fn port(&self) -> &Periph<T> { &self.port }
   #[inline] pub fn index(&self) -> usize { self.index }
}
pub trait AltFn<T> {
   fn alt_fn(&self) -> usize;
}

