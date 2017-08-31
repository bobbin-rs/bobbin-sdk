#[allow(unused_imports)] use bobbin_common::*;


#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="DMA_F3 Peripheral"]
pub struct DmaPeriph(pub usize); 


impl DmaPeriph {
#[doc="Get the *const pointer for the ISR register."]
   #[inline] pub fn isr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x0) as *const u32
   }
#[doc="Get the *mut pointer for the ISR register."]
   #[inline] pub fn isr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x0) as *mut u32
   }
#[doc="Read the ISR register."]
   #[inline] pub fn isr(&self) -> Isr { 
      unsafe {
         Isr(read_volatile((self.0 + 0x0) as *const u32))
      }
   }

#[doc="Get the *const pointer for the IFCR register."]
   #[inline] pub fn ifcr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x4) as *const u32
   }
#[doc="Get the *mut pointer for the IFCR register."]
   #[inline] pub fn ifcr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x4) as *mut u32
   }
#[doc="Write the IFCR register."]
   #[inline] pub fn set_ifcr<F: FnOnce(Ifcr) -> Ifcr>(&self, f: F) -> &Self {
      let value = f(Ifcr(0));
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CCR register."]
   #[inline] pub fn ccr_ptr<I: Into<bits::R7>>(&self, index: I) -> *const u32 { 
      let index: bits::R7 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x8 + (index * 20)) as *const u32
   }
#[doc="Get the *mut pointer for the CCR register."]
   #[inline] pub fn ccr_mut<I: Into<bits::R7>>(&self, index: I) -> *mut u32 { 
      let index: bits::R7 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x8 + (index * 20)) as *mut u32
   }
#[doc="Read the CCR register."]
   #[inline] pub fn ccr<I: Into<bits::R7>>(&self, index: I) -> Ccr { 
      let index: bits::R7 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Ccr(read_volatile((self.0 + 0x8 + (index * 20)) as *const u32))
      }
   }
#[doc="Write the CCR register."]
   #[inline] pub fn set_ccr<I: Into<bits::R7>, F: FnOnce(Ccr) -> Ccr>(&self, index: I, f: F) -> &Self {
      let index: bits::R7 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Ccr(0));
      unsafe {
         write_volatile((self.0 + 0x8 + (index * 20)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CCR register."]
   #[inline] pub fn with_ccr<I: Into<bits::R7> + Copy, F: FnOnce(Ccr) -> Ccr>(&self, index: I, f: F) -> &Self {
      let index: bits::R7 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.ccr(index);
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x8 + (index * 20)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CNDTR register."]
   #[inline] pub fn cndtr_ptr<I: Into<bits::R7>>(&self, index: I) -> *const u32 { 
      let index: bits::R7 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0xc + (index * 20)) as *const u32
   }
#[doc="Get the *mut pointer for the CNDTR register."]
   #[inline] pub fn cndtr_mut<I: Into<bits::R7>>(&self, index: I) -> *mut u32 { 
      let index: bits::R7 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0xc + (index * 20)) as *mut u32
   }
#[doc="Read the CNDTR register."]
   #[inline] pub fn cndtr<I: Into<bits::R7>>(&self, index: I) -> Cndtr { 
      let index: bits::R7 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Cndtr(read_volatile((self.0 + 0xc + (index * 20)) as *const u32))
      }
   }
#[doc="Write the CNDTR register."]
   #[inline] pub fn set_cndtr<I: Into<bits::R7>, F: FnOnce(Cndtr) -> Cndtr>(&self, index: I, f: F) -> &Self {
      let index: bits::R7 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Cndtr(0));
      unsafe {
         write_volatile((self.0 + 0xc + (index * 20)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CNDTR register."]
   #[inline] pub fn with_cndtr<I: Into<bits::R7> + Copy, F: FnOnce(Cndtr) -> Cndtr>(&self, index: I, f: F) -> &Self {
      let index: bits::R7 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.cndtr(index);
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xc + (index * 20)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CPAR register."]
   #[inline] pub fn cpar_ptr<I: Into<bits::R7>>(&self, index: I) -> *const u32 { 
      let index: bits::R7 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x10 + (index * 20)) as *const u32
   }
#[doc="Get the *mut pointer for the CPAR register."]
   #[inline] pub fn cpar_mut<I: Into<bits::R7>>(&self, index: I) -> *mut u32 { 
      let index: bits::R7 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x10 + (index * 20)) as *mut u32
   }
#[doc="Read the CPAR register."]
   #[inline] pub fn cpar<I: Into<bits::R7>>(&self, index: I) -> Cpar { 
      let index: bits::R7 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Cpar(read_volatile((self.0 + 0x10 + (index * 20)) as *const u32))
      }
   }
#[doc="Write the CPAR register."]
   #[inline] pub fn set_cpar<I: Into<bits::R7>, F: FnOnce(Cpar) -> Cpar>(&self, index: I, f: F) -> &Self {
      let index: bits::R7 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Cpar(0));
      unsafe {
         write_volatile((self.0 + 0x10 + (index * 20)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CPAR register."]
   #[inline] pub fn with_cpar<I: Into<bits::R7> + Copy, F: FnOnce(Cpar) -> Cpar>(&self, index: I, f: F) -> &Self {
      let index: bits::R7 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.cpar(index);
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x10 + (index * 20)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CMAR register."]
   #[inline] pub fn cmar_ptr<I: Into<bits::R7>>(&self, index: I) -> *const u32 { 
      let index: bits::R7 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x14 + (index * 20)) as *const u32
   }
#[doc="Get the *mut pointer for the CMAR register."]
   #[inline] pub fn cmar_mut<I: Into<bits::R7>>(&self, index: I) -> *mut u32 { 
      let index: bits::R7 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x14 + (index * 20)) as *mut u32
   }
#[doc="Read the CMAR register."]
   #[inline] pub fn cmar<I: Into<bits::R7>>(&self, index: I) -> Cmar { 
      let index: bits::R7 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Cmar(read_volatile((self.0 + 0x14 + (index * 20)) as *const u32))
      }
   }
#[doc="Write the CMAR register."]
   #[inline] pub fn set_cmar<I: Into<bits::R7>, F: FnOnce(Cmar) -> Cmar>(&self, index: I, f: F) -> &Self {
      let index: bits::R7 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Cmar(0));
      unsafe {
         write_volatile((self.0 + 0x14 + (index * 20)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CMAR register."]
   #[inline] pub fn with_cmar<I: Into<bits::R7> + Copy, F: FnOnce(Cmar) -> Cmar>(&self, index: I, f: F) -> &Self {
      let index: bits::R7 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.cmar(index);
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x14 + (index * 20)) as *mut u32, value.0);
      }
      self
   }

}

#[doc="DMA interrupt status register (DMA_ISR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
#[doc="Channel n Global interrupt flag"]
   #[inline] pub fn gif<I: Into<bits::R7>>(&self, index: I) -> bits::U1 {
      let index: bits::R7 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + (index << 2);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Channel n Global interrupt flag"]
   #[inline] pub fn set_gif<I: Into<bits::R7>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R7 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + (index << 2);
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="Channel n Transfer Complete flag"]
   #[inline] pub fn tcif<I: Into<bits::R7>>(&self, index: I) -> bits::U1 {
      let index: bits::R7 = index.into();
      let index: usize = index.value();
      let shift: usize = 1 + (index << 2);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [1]
   }
#[doc="Channel n Transfer Complete flag"]
   #[inline] pub fn set_tcif<I: Into<bits::R7>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R7 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 1 + (index << 2);
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="Channel n Half Transfer Complete flag"]
   #[inline] pub fn htif<I: Into<bits::R7>>(&self, index: I) -> bits::U1 {
      let index: bits::R7 = index.into();
      let index: usize = index.value();
      let shift: usize = 2 + (index << 2);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [2]
   }
#[doc="Channel n Half Transfer Complete flag"]
   #[inline] pub fn set_htif<I: Into<bits::R7>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R7 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 2 + (index << 2);
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="Channel n Transfer Error flag"]
   #[inline] pub fn teif<I: Into<bits::R7>>(&self, index: I) -> bits::U1 {
      let index: bits::R7 = index.into();
      let index: usize = index.value();
      let shift: usize = 3 + (index << 2);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [3]
   }
#[doc="Channel n Transfer Error flag"]
   #[inline] pub fn set_teif<I: Into<bits::R7>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R7 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 3 + (index << 2);
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Isr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Isr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.gif(0) != 0 { try!(write!(f, " gif[0]"))}
      if self.gif(1) != 0 { try!(write!(f, " gif[1]"))}
      if self.gif(2) != 0 { try!(write!(f, " gif[2]"))}
      if self.gif(3) != 0 { try!(write!(f, " gif[3]"))}
      if self.gif(4) != 0 { try!(write!(f, " gif[4]"))}
      if self.gif(5) != 0 { try!(write!(f, " gif[5]"))}
      if self.gif(6) != 0 { try!(write!(f, " gif[6]"))}
      if self.tcif(0) != 0 { try!(write!(f, " tcif[0]"))}
      if self.tcif(1) != 0 { try!(write!(f, " tcif[1]"))}
      if self.tcif(2) != 0 { try!(write!(f, " tcif[2]"))}
      if self.tcif(3) != 0 { try!(write!(f, " tcif[3]"))}
      if self.tcif(4) != 0 { try!(write!(f, " tcif[4]"))}
      if self.tcif(5) != 0 { try!(write!(f, " tcif[5]"))}
      if self.tcif(6) != 0 { try!(write!(f, " tcif[6]"))}
      if self.htif(0) != 0 { try!(write!(f, " htif[0]"))}
      if self.htif(1) != 0 { try!(write!(f, " htif[1]"))}
      if self.htif(2) != 0 { try!(write!(f, " htif[2]"))}
      if self.htif(3) != 0 { try!(write!(f, " htif[3]"))}
      if self.htif(4) != 0 { try!(write!(f, " htif[4]"))}
      if self.htif(5) != 0 { try!(write!(f, " htif[5]"))}
      if self.htif(6) != 0 { try!(write!(f, " htif[6]"))}
      if self.teif(0) != 0 { try!(write!(f, " teif[0]"))}
      if self.teif(1) != 0 { try!(write!(f, " teif[1]"))}
      if self.teif(2) != 0 { try!(write!(f, " teif[2]"))}
      if self.teif(3) != 0 { try!(write!(f, " teif[3]"))}
      if self.teif(4) != 0 { try!(write!(f, " teif[4]"))}
      if self.teif(5) != 0 { try!(write!(f, " teif[5]"))}
      if self.teif(6) != 0 { try!(write!(f, " teif[6]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA interrupt flag clear register (DMA_IFCR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ifcr(pub u32);
impl Ifcr {
#[doc="Channel n Global interrupt clear"]
   #[inline] pub fn cgif<I: Into<bits::R7>>(&self, index: I) -> bits::U1 {
      let index: bits::R7 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + (index << 2);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Channel n Global interrupt clear"]
   #[inline] pub fn set_cgif<I: Into<bits::R7>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R7 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + (index << 2);
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="Channel n Transfer Complete clear"]
   #[inline] pub fn ctcif<I: Into<bits::R7>>(&self, index: I) -> bits::U1 {
      let index: bits::R7 = index.into();
      let index: usize = index.value();
      let shift: usize = 1 + (index << 2);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [1]
   }
#[doc="Channel n Transfer Complete clear"]
   #[inline] pub fn set_ctcif<I: Into<bits::R7>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R7 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 1 + (index << 2);
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="Channel n Half Transfer clear"]
   #[inline] pub fn chtif<I: Into<bits::R7>>(&self, index: I) -> bits::U1 {
      let index: bits::R7 = index.into();
      let index: usize = index.value();
      let shift: usize = 2 + (index << 2);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [2]
   }
#[doc="Channel n Half Transfer clear"]
   #[inline] pub fn set_chtif<I: Into<bits::R7>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R7 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 2 + (index << 2);
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="Channel n Transfer Error clear"]
   #[inline] pub fn cteif<I: Into<bits::R7>>(&self, index: I) -> bits::U1 {
      let index: bits::R7 = index.into();
      let index: usize = index.value();
      let shift: usize = 3 + (index << 2);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [3]
   }
#[doc="Channel n Transfer Error clear"]
   #[inline] pub fn set_cteif<I: Into<bits::R7>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R7 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 3 + (index << 2);
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Ifcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ifcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cgif(0) != 0 { try!(write!(f, " cgif[0]"))}
      if self.cgif(1) != 0 { try!(write!(f, " cgif[1]"))}
      if self.cgif(2) != 0 { try!(write!(f, " cgif[2]"))}
      if self.cgif(3) != 0 { try!(write!(f, " cgif[3]"))}
      if self.cgif(4) != 0 { try!(write!(f, " cgif[4]"))}
      if self.cgif(5) != 0 { try!(write!(f, " cgif[5]"))}
      if self.cgif(6) != 0 { try!(write!(f, " cgif[6]"))}
      if self.ctcif(0) != 0 { try!(write!(f, " ctcif[0]"))}
      if self.ctcif(1) != 0 { try!(write!(f, " ctcif[1]"))}
      if self.ctcif(2) != 0 { try!(write!(f, " ctcif[2]"))}
      if self.ctcif(3) != 0 { try!(write!(f, " ctcif[3]"))}
      if self.ctcif(4) != 0 { try!(write!(f, " ctcif[4]"))}
      if self.ctcif(5) != 0 { try!(write!(f, " ctcif[5]"))}
      if self.ctcif(6) != 0 { try!(write!(f, " ctcif[6]"))}
      if self.chtif(0) != 0 { try!(write!(f, " chtif[0]"))}
      if self.chtif(1) != 0 { try!(write!(f, " chtif[1]"))}
      if self.chtif(2) != 0 { try!(write!(f, " chtif[2]"))}
      if self.chtif(3) != 0 { try!(write!(f, " chtif[3]"))}
      if self.chtif(4) != 0 { try!(write!(f, " chtif[4]"))}
      if self.chtif(5) != 0 { try!(write!(f, " chtif[5]"))}
      if self.chtif(6) != 0 { try!(write!(f, " chtif[6]"))}
      if self.cteif(0) != 0 { try!(write!(f, " cteif[0]"))}
      if self.cteif(1) != 0 { try!(write!(f, " cteif[1]"))}
      if self.cteif(2) != 0 { try!(write!(f, " cteif[2]"))}
      if self.cteif(3) != 0 { try!(write!(f, " cteif[3]"))}
      if self.cteif(4) != 0 { try!(write!(f, " cteif[4]"))}
      if self.cteif(5) != 0 { try!(write!(f, " cteif[5]"))}
      if self.cteif(6) != 0 { try!(write!(f, " cteif[6]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA channel configuration register (DMA_CCR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccr(pub u32);
impl Ccr {
#[doc="Channel enable"]
   #[inline] pub fn en(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Channel enable"]
   #[inline] pub fn set_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Transfer complete interrupt enable"]
   #[inline] pub fn tcie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Transfer complete interrupt enable"]
   #[inline] pub fn set_tcie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Half Transfer interrupt enable"]
   #[inline] pub fn htie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Half Transfer interrupt enable"]
   #[inline] pub fn set_htie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Transfer error interrupt enable"]
   #[inline] pub fn teie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Transfer error interrupt enable"]
   #[inline] pub fn set_teie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Data transfer direction"]
   #[inline] pub fn dir(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Data transfer direction"]
   #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Circular mode"]
   #[inline] pub fn circ(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Circular mode"]
   #[inline] pub fn set_circ<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Peripheral increment mode"]
   #[inline] pub fn pinc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Peripheral increment mode"]
   #[inline] pub fn set_pinc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Memory increment mode"]
   #[inline] pub fn minc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Memory increment mode"]
   #[inline] pub fn set_minc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="Peripheral size"]
   #[inline] pub fn psize(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
   }
#[doc="Peripheral size"]
   #[inline] pub fn set_psize<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Memory size"]
   #[inline] pub fn msize(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
   }
#[doc="Memory size"]
   #[inline] pub fn set_msize<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="Channel Priority level"]
   #[inline] pub fn pl(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
   }
#[doc="Channel Priority level"]
   #[inline] pub fn set_pl<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Memory to memory mode"]
   #[inline] pub fn mem2mem(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="Memory to memory mode"]
   #[inline] pub fn set_mem2mem<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

}
impl ::core::fmt::Display for Ccr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ccr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.en() != 0 { try!(write!(f, " en"))}
      if self.tcie() != 0 { try!(write!(f, " tcie"))}
      if self.htie() != 0 { try!(write!(f, " htie"))}
      if self.teie() != 0 { try!(write!(f, " teie"))}
      if self.dir() != 0 { try!(write!(f, " dir"))}
      if self.circ() != 0 { try!(write!(f, " circ"))}
      if self.pinc() != 0 { try!(write!(f, " pinc"))}
      if self.minc() != 0 { try!(write!(f, " minc"))}
      if self.psize() != 0 { try!(write!(f, " psize=0x{:x}", self.psize()))}
      if self.msize() != 0 { try!(write!(f, " msize=0x{:x}", self.msize()))}
      if self.pl() != 0 { try!(write!(f, " pl=0x{:x}", self.pl()))}
      if self.mem2mem() != 0 { try!(write!(f, " mem2mem"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA channel n number of data register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cndtr(pub u32);
impl Cndtr {
#[doc="Number of data to transfer"]
   #[inline] pub fn ndt(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Number of data to transfer"]
   #[inline] pub fn set_ndt<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Cndtr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cndtr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ndt() != 0 { try!(write!(f, " ndt=0x{:x}", self.ndt()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA channel n peripheral address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cpar(pub u32);
impl Cpar {
#[doc="Peripheral address"]
   #[inline] pub fn pa(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Peripheral address"]
   #[inline] pub fn set_pa<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Cpar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cpar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA channel n memory address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cmar(pub u32);
impl Cmar {
#[doc="Memory address"]
   #[inline] pub fn ma(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Memory address"]
   #[inline] pub fn set_ma<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Cmar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cmar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
pub struct DmaCh { pub periph: DmaPeriph, pub index: usize }

