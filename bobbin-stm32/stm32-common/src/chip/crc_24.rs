
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Periph<T>(pub u32, pub T); 



impl<T> Periph<T> {
  #[inline] pub fn dr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
  #[inline] pub fn dr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
  #[inline] pub fn dr(&self) -> Dr { 
     unsafe {
        Dr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  #[inline] pub fn set_dr(&self, value: Dr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
     let tmp = self.dr();
     self.set_dr(f(tmp))
  }

  #[inline] pub fn idr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
  #[inline] pub fn idr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
  #[inline] pub fn idr(&self) -> Idr { 
     unsafe {
        Idr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
  #[inline] pub fn set_idr(&self, value: Idr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_idr<F: FnOnce(Idr) -> Idr>(&self, f: F) -> &Self {
     let tmp = self.idr();
     self.set_idr(f(tmp))
  }

  #[inline] pub fn cr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
  #[inline] pub fn cr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
  #[inline] pub fn cr(&self) -> Cr { 
     unsafe {
        Cr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
  #[inline] pub fn set_cr(&self, value: Cr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
     let tmp = self.cr();
     self.set_cr(f(tmp))
  }

  #[inline] pub fn init_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
  #[inline] pub fn init_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
  #[inline] pub fn init(&self) -> Init { 
     unsafe {
        Init(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
  #[inline] pub fn set_init(&self, value: Init) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_init<F: FnOnce(Init) -> Init>(&self, f: F) -> &Self {
     let tmp = self.init();
     self.set_init(f(tmp))
  }

  #[inline] pub fn dummy_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x100 + (index << 2)) as *const u32
  }
  #[inline] pub fn dummy_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x100 + (index << 2)) as *mut u32
  }
  #[inline] pub fn dummy(&self, index: usize) -> Dummy { 
     assert!(index < 4);
     unsafe {
        Dummy(::core::ptr::read_volatile(((self.0 as usize) + 0x100 + (index << 2)) as *const u32))
     }
  }
  #[inline] pub fn set_dummy(&self, index: usize, value: Dummy) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x100 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dummy<F: FnOnce(Dummy) -> Dummy>(&self, index: usize, f: F) -> &Self {
     let tmp = self.dummy(index);
     self.set_dummy(index, f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
  #[inline] pub fn dr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
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
#[derive(PartialEq, Eq)]
pub struct Idr(pub u32);
impl Idr {
  #[inline] pub fn idr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
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
#[derive(PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
  #[inline] pub fn _reset(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_reset(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn rev_in(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x3 // [6:5]
  }
  #[inline] pub fn set_rev_in(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn rev_out(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
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
#[derive(PartialEq, Eq)]
pub struct Init(pub u32);
impl Init {
  #[inline] pub fn init(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
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
#[derive(PartialEq, Eq)]
pub struct Dummy(pub u32);
impl Dummy {
  #[inline] pub fn data(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
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
