
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="DMAMUX Peripheral"]
pub struct Periph<T>(pub u32, pub T); 



impl<T> Periph<T> {
#[doc="Get the *const pointer for the CHCFG register."]
  #[inline] pub fn chcfg_ptr(&self, index: usize) -> *const u8 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x0 + (index)) as *const u8
  }
#[doc="Get the *mut pointer for the CHCFG register."]
  #[inline] pub fn chcfg_mut(&self, index: usize) -> *mut u8 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x0 + (index)) as *mut u8
  }
#[doc="Read the CHCFG register."]
  #[inline] pub fn chcfg(&self, index: usize) -> Chcfg { 
     assert!(index < 16);
     unsafe {
        Chcfg(::core::ptr::read_volatile(((self.0 as usize) + 0x0 + (index)) as *const u8))
     }
  }
#[doc="Write the CHCFG register."]
  #[inline] pub fn set_chcfg(&self, index: usize, value: Chcfg) -> &Self {
     assert!(index < 16);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0 + (index)) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the CHCFG register."]
  #[inline] pub fn with_chcfg<F: FnOnce(Chcfg) -> Chcfg>(&self, index: usize, f: F) -> &Self {
     let tmp = self.chcfg(index);
     self.set_chcfg(index, f(tmp))
  }

}

#[doc="Channel Configuration register"]
#[derive(PartialEq, Eq)]
pub struct Chcfg(pub u8);
impl Chcfg {
#[doc="DMA Channel Source (Slot)"]
  #[inline] pub fn source(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x3f // [5:0]
  }
#[doc="DMA Channel Source (Slot)"]
  #[inline] pub fn set_source(mut self, value: u8) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 0);
     self.0 |= value << 0;
     self
  }

#[doc="DMA Channel Trigger Enable"]
  #[inline] pub fn trig(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
#[doc="DMA Channel Trigger Enable"]
  #[inline] pub fn set_trig(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="DMA Channel Enable"]
  #[inline] pub fn enbl(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
#[doc="DMA Channel Enable"]
  #[inline] pub fn set_enbl(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Chcfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chcfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.source() != 0 { try!(write!(f, " source=0x{:x}", self.source()))}
      if self.trig() != 0 { try!(write!(f, " trig"))}
      if self.enbl() != 0 { try!(write!(f, " enbl"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
