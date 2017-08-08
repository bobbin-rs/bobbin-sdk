#[allow(unused_imports)] use bobbin_common::bits;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="CRC_24 Peripheral"]
pub struct Periph<T>(pub u32, pub T); 



impl<T> Periph<T> {
#[doc="Get the *const pointer for the DR register."]
  #[inline] pub fn dr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the DR register."]
  #[inline] pub fn dr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the DR register."]
  #[inline] pub fn dr(&self) -> Dr { 
     unsafe {
        Dr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the DR register."]
  #[inline] pub fn set_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
     let value = f(Dr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DR register."]
  #[inline] pub fn with_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
     let tmp = self.dr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the IDR register."]
  #[inline] pub fn idr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the IDR register."]
  #[inline] pub fn idr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the IDR register."]
  #[inline] pub fn idr(&self) -> Idr { 
     unsafe {
        Idr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the IDR register."]
  #[inline] pub fn set_idr<F: FnOnce(Idr) -> Idr>(&self, f: F) -> &Self {
     let value = f(Idr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the IDR register."]
  #[inline] pub fn with_idr<F: FnOnce(Idr) -> Idr>(&self, f: F) -> &Self {
     let tmp = self.idr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CR register."]
  #[inline] pub fn cr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the CR register."]
  #[inline] pub fn cr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the CR register."]
  #[inline] pub fn cr(&self) -> Cr { 
     unsafe {
        Cr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the CR register."]
  #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
     let value = f(Cr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CR register."]
  #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
     let tmp = self.cr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the INIT register."]
  #[inline] pub fn init_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the INIT register."]
  #[inline] pub fn init_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the INIT register."]
  #[inline] pub fn init(&self) -> Init { 
     unsafe {
        Init(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the INIT register."]
  #[inline] pub fn set_init<F: FnOnce(Init) -> Init>(&self, f: F) -> &Self {
     let value = f(Init(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the INIT register."]
  #[inline] pub fn with_init<F: FnOnce(Init) -> Init>(&self, f: F) -> &Self {
     let tmp = self.init();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the DUMMY register."]
  #[inline] pub fn dummy_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
     let index: bits::R4 = index.into();
     let index: usize = index.value() as usize;
     ((self.0 as usize) + 0x100 + (index << 2)) as *const u32
  }
#[doc="Get the *mut pointer for the DUMMY register."]
  #[inline] pub fn dummy_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
     let index: bits::R4 = index.into();
     let index: usize = index.value() as usize;
     ((self.0 as usize) + 0x100 + (index << 2)) as *mut u32
  }
#[doc="Read the DUMMY register."]
  #[inline] pub fn dummy<I: Into<bits::R4>>(&self, index: I) -> Dummy { 
     let index: bits::R4 = index.into();
     let index: usize = index.value() as usize;
     unsafe {
        Dummy(::core::ptr::read_volatile(((self.0 as usize) + 0x100 + (index << 2)) as *const u32))
     }
  }
#[doc="Write the DUMMY register."]
  #[inline] pub fn set_dummy<I: Into<bits::R4>, F: FnOnce(Dummy) -> Dummy>(&self, index: I, f: F) -> &Self {
     let index: bits::R4 = index.into();
     let index: usize = index.value() as usize;
     let value = f(Dummy(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x100 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DUMMY register."]
  #[inline] pub fn with_dummy<I: Into<bits::R4> + Copy, F: FnOnce(Dummy) -> Dummy>(&self, index: I, f: F) -> &Self {
     let index: bits::R4 = index.into();
     let index: usize = index.value() as usize;
     let tmp = self.dummy(index);
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x100 + (index << 2)) as *mut u32, value.0);
     }
     self
  }

}

#[doc="Data register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
#[doc="Data register bits"]
  #[inline] pub fn dr(&self) -> bits::U32 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
  }
#[doc="Data register bits"]
  #[inline] pub fn set_dr<V: Into<bits::U32>>(mut self, value: V) -> Self {
     let value: bits::U32 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Independent data register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Idr(pub u32);
impl Idr {
#[doc="General-purpose 8-bit data register bits"]
  #[inline] pub fn idr(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="General-purpose 8-bit data register bits"]
  #[inline] pub fn set_idr<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Idr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Idr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.idr() != 0 { try!(write!(f, " idr=0x{:x}", self.idr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
#[doc="reset bit"]
  #[inline] pub fn _reset(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="reset bit"]
  #[inline] pub fn set_reset<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Reverse input data"]
  #[inline] pub fn rev_in(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
  }
#[doc="Reverse input data"]
  #[inline] pub fn set_rev_in<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Reverse output data"]
  #[inline] pub fn rev_out(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="Reverse output data"]
  #[inline] pub fn set_rev_out<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
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
      if self._reset() != 0 { try!(write!(f, " _reset"))}
      if self.rev_in() != 0 { try!(write!(f, " rev_in=0x{:x}", self.rev_in()))}
      if self.rev_out() != 0 { try!(write!(f, " rev_out"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Initial CRC value"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Init(pub u32);
impl Init {
#[doc="Programmable initial CRC value"]
  #[inline] pub fn init(&self) -> bits::U32 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
  }
#[doc="Programmable initial CRC value"]
  #[inline] pub fn set_init<V: Into<bits::U32>>(mut self, value: V) -> Self {
     let value: bits::U32 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Init {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Init {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DUMMY Array Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dummy(pub u32);
impl Dummy {
#[doc="DUMMY DATA field"]
  #[inline] pub fn data(&self) -> bits::U32 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
  }
#[doc="DUMMY DATA field"]
  #[inline] pub fn set_data<V: Into<bits::U32>>(mut self, value: V) -> Self {
     let value: bits::U32 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dummy {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dummy {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
