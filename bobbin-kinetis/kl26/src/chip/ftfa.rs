//! Flash configuration field
#[allow(unused_imports)] use bobbin_common::*;

periph!(FTFA, Ftfa, 0x00000400);

#[doc="Flash configuration field"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ftfa(pub usize);
impl Ftfa {
#[doc="Get the *const pointer for the BACKKEY3 register."]
   #[inline] pub fn backkey3_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x0) as *const u8
   }
#[doc="Get the *mut pointer for the BACKKEY3 register."]
   #[inline] pub fn backkey3_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x0) as *mut u8
   }
#[doc="Read the BACKKEY3 register."]
   #[inline] pub fn backkey3(&self) -> Backkey3 { 
      unsafe {
         Backkey3(::core::ptr::read_volatile((self.0 + 0x0) as *const u8))
      }
   }

#[doc="Get the *const pointer for the BACKKEY2 register."]
   #[inline] pub fn backkey2_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x1) as *const u8
   }
#[doc="Get the *mut pointer for the BACKKEY2 register."]
   #[inline] pub fn backkey2_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x1) as *mut u8
   }
#[doc="Read the BACKKEY2 register."]
   #[inline] pub fn backkey2(&self) -> Backkey2 { 
      unsafe {
         Backkey2(::core::ptr::read_volatile((self.0 + 0x1) as *const u8))
      }
   }

#[doc="Get the *const pointer for the BACKKEY1 register."]
   #[inline] pub fn backkey1_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x2) as *const u8
   }
#[doc="Get the *mut pointer for the BACKKEY1 register."]
   #[inline] pub fn backkey1_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x2) as *mut u8
   }
#[doc="Read the BACKKEY1 register."]
   #[inline] pub fn backkey1(&self) -> Backkey1 { 
      unsafe {
         Backkey1(::core::ptr::read_volatile((self.0 + 0x2) as *const u8))
      }
   }

#[doc="Get the *const pointer for the BACKKEY0 register."]
   #[inline] pub fn backkey0_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x3) as *const u8
   }
#[doc="Get the *mut pointer for the BACKKEY0 register."]
   #[inline] pub fn backkey0_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x3) as *mut u8
   }
#[doc="Read the BACKKEY0 register."]
   #[inline] pub fn backkey0(&self) -> Backkey0 { 
      unsafe {
         Backkey0(::core::ptr::read_volatile((self.0 + 0x3) as *const u8))
      }
   }

#[doc="Get the *const pointer for the BACKKEY7 register."]
   #[inline] pub fn backkey7_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x4) as *const u8
   }
#[doc="Get the *mut pointer for the BACKKEY7 register."]
   #[inline] pub fn backkey7_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x4) as *mut u8
   }
#[doc="Read the BACKKEY7 register."]
   #[inline] pub fn backkey7(&self) -> Backkey7 { 
      unsafe {
         Backkey7(::core::ptr::read_volatile((self.0 + 0x4) as *const u8))
      }
   }

#[doc="Get the *const pointer for the BACKKEY6 register."]
   #[inline] pub fn backkey6_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x5) as *const u8
   }
#[doc="Get the *mut pointer for the BACKKEY6 register."]
   #[inline] pub fn backkey6_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x5) as *mut u8
   }
#[doc="Read the BACKKEY6 register."]
   #[inline] pub fn backkey6(&self) -> Backkey6 { 
      unsafe {
         Backkey6(::core::ptr::read_volatile((self.0 + 0x5) as *const u8))
      }
   }

#[doc="Get the *const pointer for the BACKKEY5 register."]
   #[inline] pub fn backkey5_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x6) as *const u8
   }
#[doc="Get the *mut pointer for the BACKKEY5 register."]
   #[inline] pub fn backkey5_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x6) as *mut u8
   }
#[doc="Read the BACKKEY5 register."]
   #[inline] pub fn backkey5(&self) -> Backkey5 { 
      unsafe {
         Backkey5(::core::ptr::read_volatile((self.0 + 0x6) as *const u8))
      }
   }

#[doc="Get the *const pointer for the BACKKEY4 register."]
   #[inline] pub fn backkey4_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x7) as *const u8
   }
#[doc="Get the *mut pointer for the BACKKEY4 register."]
   #[inline] pub fn backkey4_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x7) as *mut u8
   }
#[doc="Read the BACKKEY4 register."]
   #[inline] pub fn backkey4(&self) -> Backkey4 { 
      unsafe {
         Backkey4(::core::ptr::read_volatile((self.0 + 0x7) as *const u8))
      }
   }

#[doc="Get the *const pointer for the FPROT3 register."]
   #[inline] pub fn fprot3_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x8) as *const u8
   }
#[doc="Get the *mut pointer for the FPROT3 register."]
   #[inline] pub fn fprot3_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x8) as *mut u8
   }
#[doc="Read the FPROT3 register."]
   #[inline] pub fn fprot3(&self) -> Fprot3 { 
      unsafe {
         Fprot3(::core::ptr::read_volatile((self.0 + 0x8) as *const u8))
      }
   }

#[doc="Get the *const pointer for the FPROT2 register."]
   #[inline] pub fn fprot2_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x9) as *const u8
   }
#[doc="Get the *mut pointer for the FPROT2 register."]
   #[inline] pub fn fprot2_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x9) as *mut u8
   }
#[doc="Read the FPROT2 register."]
   #[inline] pub fn fprot2(&self) -> Fprot2 { 
      unsafe {
         Fprot2(::core::ptr::read_volatile((self.0 + 0x9) as *const u8))
      }
   }

#[doc="Get the *const pointer for the FPROT1 register."]
   #[inline] pub fn fprot1_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xa) as *const u8
   }
#[doc="Get the *mut pointer for the FPROT1 register."]
   #[inline] pub fn fprot1_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xa) as *mut u8
   }
#[doc="Read the FPROT1 register."]
   #[inline] pub fn fprot1(&self) -> Fprot1 { 
      unsafe {
         Fprot1(::core::ptr::read_volatile((self.0 + 0xa) as *const u8))
      }
   }

#[doc="Get the *const pointer for the FPROT0 register."]
   #[inline] pub fn fprot0_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xb) as *const u8
   }
#[doc="Get the *mut pointer for the FPROT0 register."]
   #[inline] pub fn fprot0_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xb) as *mut u8
   }
#[doc="Read the FPROT0 register."]
   #[inline] pub fn fprot0(&self) -> Fprot0 { 
      unsafe {
         Fprot0(::core::ptr::read_volatile((self.0 + 0xb) as *const u8))
      }
   }

#[doc="Get the *const pointer for the FSEC register."]
   #[inline] pub fn fsec_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xc) as *const u8
   }
#[doc="Get the *mut pointer for the FSEC register."]
   #[inline] pub fn fsec_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xc) as *mut u8
   }
#[doc="Read the FSEC register."]
   #[inline] pub fn fsec(&self) -> Fsec { 
      unsafe {
         Fsec(::core::ptr::read_volatile((self.0 + 0xc) as *const u8))
      }
   }

#[doc="Get the *const pointer for the FOPT register."]
   #[inline] pub fn fopt_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xd) as *const u8
   }
#[doc="Get the *mut pointer for the FOPT register."]
   #[inline] pub fn fopt_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xd) as *mut u8
   }
#[doc="Read the FOPT register."]
   #[inline] pub fn fopt(&self) -> Fopt { 
      unsafe {
         Fopt(::core::ptr::read_volatile((self.0 + 0xd) as *const u8))
      }
   }

}

#[doc="Backdoor Comparison Key 3."]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Backkey3(pub u8);
impl Backkey3 {
#[doc="Backdoor Comparison Key."]
   #[inline] pub fn key(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Backdoor Comparison Key."]
   #[inline] pub fn set_key<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Backkey3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Backkey3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Backdoor Comparison Key 2."]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Backkey2(pub u8);
impl Backkey2 {
#[doc="Backdoor Comparison Key."]
   #[inline] pub fn key(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Backdoor Comparison Key."]
   #[inline] pub fn set_key<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Backkey2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Backkey2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Backdoor Comparison Key 1."]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Backkey1(pub u8);
impl Backkey1 {
#[doc="Backdoor Comparison Key."]
   #[inline] pub fn key(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Backdoor Comparison Key."]
   #[inline] pub fn set_key<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Backkey1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Backkey1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Backdoor Comparison Key 0."]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Backkey0(pub u8);
impl Backkey0 {
#[doc="Backdoor Comparison Key."]
   #[inline] pub fn key(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Backdoor Comparison Key."]
   #[inline] pub fn set_key<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Backkey0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Backkey0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Backdoor Comparison Key 7."]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Backkey7(pub u8);
impl Backkey7 {
#[doc="Backdoor Comparison Key."]
   #[inline] pub fn key(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Backdoor Comparison Key."]
   #[inline] pub fn set_key<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Backkey7 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Backkey7 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Backdoor Comparison Key 6."]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Backkey6(pub u8);
impl Backkey6 {
#[doc="Backdoor Comparison Key."]
   #[inline] pub fn key(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Backdoor Comparison Key."]
   #[inline] pub fn set_key<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Backkey6 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Backkey6 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Backdoor Comparison Key 5."]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Backkey5(pub u8);
impl Backkey5 {
#[doc="Backdoor Comparison Key."]
   #[inline] pub fn key(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Backdoor Comparison Key."]
   #[inline] pub fn set_key<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Backkey5 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Backkey5 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Backdoor Comparison Key 4."]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Backkey4(pub u8);
impl Backkey4 {
#[doc="Backdoor Comparison Key."]
   #[inline] pub fn key(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Backdoor Comparison Key."]
   #[inline] pub fn set_key<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Backkey4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Backkey4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Non-volatile P-Flash Protection 1 - Low Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fprot3(pub u8);
impl Fprot3 {
#[doc="P-Flash Region Protect"]
   #[inline] pub fn prot(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="P-Flash Region Protect"]
   #[inline] pub fn set_prot<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Fprot3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fprot3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.prot() != 0 { try!(write!(f, " prot=0x{:x}", self.prot()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Non-volatile P-Flash Protection 1 - High Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fprot2(pub u8);
impl Fprot2 {
#[doc="P-Flash Region Protect"]
   #[inline] pub fn prot(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="P-Flash Region Protect"]
   #[inline] pub fn set_prot<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Fprot2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fprot2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.prot() != 0 { try!(write!(f, " prot=0x{:x}", self.prot()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Non-volatile P-Flash Protection 0 - Low Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fprot1(pub u8);
impl Fprot1 {
#[doc="P-Flash Region Protect"]
   #[inline] pub fn prot(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="P-Flash Region Protect"]
   #[inline] pub fn set_prot<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Fprot1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fprot1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.prot() != 0 { try!(write!(f, " prot=0x{:x}", self.prot()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Non-volatile P-Flash Protection 0 - High Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fprot0(pub u8);
impl Fprot0 {
#[doc="P-Flash Region Protect"]
   #[inline] pub fn prot(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="P-Flash Region Protect"]
   #[inline] pub fn set_prot<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Fprot0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fprot0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.prot() != 0 { try!(write!(f, " prot=0x{:x}", self.prot()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Non-volatile Flash Security Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fsec(pub u8);
impl Fsec {
#[doc="Flash Security"]
   #[inline] pub fn sec(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="Flash Security"]
   #[inline] pub fn set_sec<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x3 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Freescale Failure Analysis Access Code"]
   #[inline] pub fn fslacc(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
   }
#[doc="Freescale Failure Analysis Access Code"]
   #[inline] pub fn set_fslacc<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x3 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="no description available"]
   #[inline] pub fn meen(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
   }
#[doc="no description available"]
   #[inline] pub fn set_meen<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x3 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Backdoor Key Security Enable"]
   #[inline] pub fn keyen(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
   }
#[doc="Backdoor Key Security Enable"]
   #[inline] pub fn set_keyen<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x3 << 6);
      self.0 |= value << 6;
      self
   }

}
impl ::core::fmt::Display for Fsec {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fsec {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sec() != 0 { try!(write!(f, " sec=0x{:x}", self.sec()))}
      if self.fslacc() != 0 { try!(write!(f, " fslacc=0x{:x}", self.fslacc()))}
      if self.meen() != 0 { try!(write!(f, " meen=0x{:x}", self.meen()))}
      if self.keyen() != 0 { try!(write!(f, " keyen=0x{:x}", self.keyen()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Non-volatile Flash Option Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fopt(pub u8);
impl Fopt {
#[doc="no description available"]
   #[inline] pub fn lpboot0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="no description available"]
   #[inline] pub fn set_lpboot0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="no description available"]
   #[inline] pub fn nmi_dis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="no description available"]
   #[inline] pub fn set_nmi_dis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="no description available"]
   #[inline] pub fn reset_pin_cfg(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="no description available"]
   #[inline] pub fn set_reset_pin_cfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="no description available"]
   #[inline] pub fn lpboot1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="no description available"]
   #[inline] pub fn set_lpboot1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="no description available"]
   #[inline] pub fn fast_init(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="no description available"]
   #[inline] pub fn set_fast_init<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

}
impl ::core::fmt::Display for Fopt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fopt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lpboot0() != 0 { try!(write!(f, " lpboot0"))}
      if self.nmi_dis() != 0 { try!(write!(f, " nmi_dis"))}
      if self.reset_pin_cfg() != 0 { try!(write!(f, " reset_pin_cfg"))}
      if self.lpboot1() != 0 { try!(write!(f, " lpboot1"))}
      if self.fast_init() != 0 { try!(write!(f, " fast_init"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

