#[allow(unused_imports)] use bobbin_common::bits;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="DMAMUX Peripheral"]
pub struct Periph<T>(pub u32, pub T); 



impl<T> Periph<T> {
#[doc="Get the *const pointer for the CHCFG register."]
  #[inline] pub fn chcfg_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u8 { 
     let index: bits::R16 = index.into();
     let index: usize = index.value();
     ((self.0 as usize) + 0x0 + (index)) as *const u8
  }
#[doc="Get the *mut pointer for the CHCFG register."]
  #[inline] pub fn chcfg_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u8 { 
     let index: bits::R16 = index.into();
     let index: usize = index.value();
     ((self.0 as usize) + 0x0 + (index)) as *mut u8
  }
#[doc="Read the CHCFG register."]
  #[inline] pub fn chcfg<I: Into<bits::R16>>(&self, index: I) -> Chcfg { 
     let index: bits::R16 = index.into();
     let index: usize = index.value();
     unsafe {
        Chcfg(::core::ptr::read_volatile(((self.0 as usize) + 0x0 + (index)) as *const u8))
     }
  }
#[doc="Write the CHCFG register."]
  #[inline] pub fn set_chcfg<I: Into<bits::R16>, F: FnOnce(Chcfg) -> Chcfg>(&self, index: I, f: F) -> &Self {
     let index: bits::R16 = index.into();
     let index: usize = index.value();
     let value = f(Chcfg(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0 + (index)) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the CHCFG register."]
  #[inline] pub fn with_chcfg<I: Into<bits::R16> + Copy, F: FnOnce(Chcfg) -> Chcfg>(&self, index: I, f: F) -> &Self {
     let tmp = self.chcfg(index);
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u8, value.0);
     }
     self
  }

}

#[doc="Channel Configuration register"]
#[derive(PartialEq, Eq)]
pub struct Chcfg(pub u8);
impl Chcfg {
#[doc="DMA Channel Source (Slot)"]
  #[inline] pub fn source(&self) -> bits::U6 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
  }
#[doc="DMA Channel Source (Slot)"]
  #[inline] pub fn set_source<V: Into<bits::U6>>(mut self, value: V) -> Self {
     let value: bits::U6 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x3f << 0);
     self.0 |= value << 0;
     self
  }

#[doc="DMA Channel Trigger Enable"]
  #[inline] pub fn trig(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="DMA Channel Trigger Enable"]
  #[inline] pub fn set_trig<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="DMA Channel Enable"]
  #[inline] pub fn enbl(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="DMA Channel Enable"]
  #[inline] pub fn set_enbl<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
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
