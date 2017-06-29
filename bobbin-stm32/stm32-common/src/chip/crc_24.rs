
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
  #[inline] pub fn set_dr(&self, value: Dr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DR register."]
  #[inline] pub fn with_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
     let tmp = self.dr();
     self.set_dr(f(tmp))
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
  #[inline] pub fn set_idr(&self, value: Idr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the IDR register."]
  #[inline] pub fn with_idr<F: FnOnce(Idr) -> Idr>(&self, f: F) -> &Self {
     let tmp = self.idr();
     self.set_idr(f(tmp))
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
  #[inline] pub fn set_cr(&self, value: Cr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CR register."]
  #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
     let tmp = self.cr();
     self.set_cr(f(tmp))
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
  #[inline] pub fn set_init(&self, value: Init) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the INIT register."]
  #[inline] pub fn with_init<F: FnOnce(Init) -> Init>(&self, f: F) -> &Self {
     let tmp = self.init();
     self.set_init(f(tmp))
  }

#[doc="Get the *const pointer for the DUMMY register."]
  #[inline] pub fn dummy_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x100 + (index << 2)) as *const u32
  }
#[doc="Get the *mut pointer for the DUMMY register."]
  #[inline] pub fn dummy_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x100 + (index << 2)) as *mut u32
  }
#[doc="Read the DUMMY register."]
  #[inline] pub fn dummy(&self, index: usize) -> Dummy { 
     assert!(index < 4);
     unsafe {
        Dummy(::core::ptr::read_volatile(((self.0 as usize) + 0x100 + (index << 2)) as *const u32))
     }
  }
#[doc="Write the DUMMY register."]
  #[inline] pub fn set_dummy(&self, index: usize, value: Dummy) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x100 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DUMMY register."]
  #[inline] pub fn with_dummy<F: FnOnce(Dummy) -> Dummy>(&self, index: usize, f: F) -> &Self {
     let tmp = self.dummy(index);
     self.set_dummy(index, f(tmp))
  }

}

#[doc="Data register"]
#[derive(PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
#[doc="Data register bits"]
  #[inline] pub fn dr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Data register bits"]
  #[inline] pub fn set_dr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Idr(pub u32);
impl Idr {
#[doc="General-purpose 8-bit data register bits"]
  #[inline] pub fn idr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
#[doc="General-purpose 8-bit data register bits"]
  #[inline] pub fn set_idr(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
#[doc="reset bit"]
  #[inline] pub fn _reset(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="reset bit"]
  #[inline] pub fn set_reset(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Reverse input data"]
  #[inline] pub fn rev_in(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x3 // [6:5]
  }
#[doc="Reverse input data"]
  #[inline] pub fn set_rev_in(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Reverse output data"]
  #[inline] pub fn rev_out(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Reverse output data"]
  #[inline] pub fn set_rev_out(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Init(pub u32);
impl Init {
#[doc="Programmable initial CRC value"]
  #[inline] pub fn init(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Programmable initial CRC value"]
  #[inline] pub fn set_init(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Dummy(pub u32);
impl Dummy {
#[doc="DUMMY DATA field"]
  #[inline] pub fn data(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="DUMMY DATA field"]
  #[inline] pub fn set_data(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
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
