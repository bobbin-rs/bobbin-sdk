
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="DMA_F3 Peripheral"]
pub struct Periph<T>(pub u32, pub T); 



impl<T> Periph<T> {
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
        Isr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
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
  #[inline] pub fn set_ifcr(&self, value: Ifcr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CCR register."]
  #[inline] pub fn ccr_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 7);
     ((self.0 as usize) + 0x8 + (index * 20)) as *const u32
  }
#[doc="Get the *mut pointer for the CCR register."]
  #[inline] pub fn ccr_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 7);
     ((self.0 as usize) + 0x8 + (index * 20)) as *mut u32
  }
#[doc="Read the CCR register."]
  #[inline] pub fn ccr(&self, index: usize) -> Ccr { 
     assert!(index < 7);
     unsafe {
        Ccr(::core::ptr::read_volatile(((self.0 as usize) + 0x8 + (index * 20)) as *const u32))
     }
  }
#[doc="Write the CCR register."]
  #[inline] pub fn set_ccr(&self, index: usize, value: Ccr) -> &Self {
     assert!(index < 7);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8 + (index * 20)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CCR register."]
  #[inline] pub fn with_ccr<F: FnOnce(Ccr) -> Ccr>(&self, index: usize, f: F) -> &Self {
     let tmp = self.ccr(index);
     self.set_ccr(index, f(tmp))
  }

#[doc="Get the *const pointer for the CNDTR register."]
  #[inline] pub fn cndtr_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 7);
     ((self.0 as usize) + 0xc + (index * 20)) as *const u32
  }
#[doc="Get the *mut pointer for the CNDTR register."]
  #[inline] pub fn cndtr_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 7);
     ((self.0 as usize) + 0xc + (index * 20)) as *mut u32
  }
#[doc="Read the CNDTR register."]
  #[inline] pub fn cndtr(&self, index: usize) -> Cndtr { 
     assert!(index < 7);
     unsafe {
        Cndtr(::core::ptr::read_volatile(((self.0 as usize) + 0xc + (index * 20)) as *const u32))
     }
  }
#[doc="Write the CNDTR register."]
  #[inline] pub fn set_cndtr(&self, index: usize, value: Cndtr) -> &Self {
     assert!(index < 7);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc + (index * 20)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CNDTR register."]
  #[inline] pub fn with_cndtr<F: FnOnce(Cndtr) -> Cndtr>(&self, index: usize, f: F) -> &Self {
     let tmp = self.cndtr(index);
     self.set_cndtr(index, f(tmp))
  }

#[doc="Get the *const pointer for the CPAR register."]
  #[inline] pub fn cpar_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 7);
     ((self.0 as usize) + 0x10 + (index * 20)) as *const u32
  }
#[doc="Get the *mut pointer for the CPAR register."]
  #[inline] pub fn cpar_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 7);
     ((self.0 as usize) + 0x10 + (index * 20)) as *mut u32
  }
#[doc="Read the CPAR register."]
  #[inline] pub fn cpar(&self, index: usize) -> Cpar { 
     assert!(index < 7);
     unsafe {
        Cpar(::core::ptr::read_volatile(((self.0 as usize) + 0x10 + (index * 20)) as *const u32))
     }
  }
#[doc="Write the CPAR register."]
  #[inline] pub fn set_cpar(&self, index: usize, value: Cpar) -> &Self {
     assert!(index < 7);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10 + (index * 20)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CPAR register."]
  #[inline] pub fn with_cpar<F: FnOnce(Cpar) -> Cpar>(&self, index: usize, f: F) -> &Self {
     let tmp = self.cpar(index);
     self.set_cpar(index, f(tmp))
  }

#[doc="Get the *const pointer for the CMAR register."]
  #[inline] pub fn cmar_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 7);
     ((self.0 as usize) + 0x14 + (index * 20)) as *const u32
  }
#[doc="Get the *mut pointer for the CMAR register."]
  #[inline] pub fn cmar_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 7);
     ((self.0 as usize) + 0x14 + (index * 20)) as *mut u32
  }
#[doc="Read the CMAR register."]
  #[inline] pub fn cmar(&self, index: usize) -> Cmar { 
     assert!(index < 7);
     unsafe {
        Cmar(::core::ptr::read_volatile(((self.0 as usize) + 0x14 + (index * 20)) as *const u32))
     }
  }
#[doc="Write the CMAR register."]
  #[inline] pub fn set_cmar(&self, index: usize, value: Cmar) -> &Self {
     assert!(index < 7);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14 + (index * 20)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CMAR register."]
  #[inline] pub fn with_cmar<F: FnOnce(Cmar) -> Cmar>(&self, index: usize, f: F) -> &Self {
     let tmp = self.cmar(index);
     self.set_cmar(index, f(tmp))
  }

}

#[doc="DMA interrupt status register (DMA_ISR)"]
#[derive(PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
#[doc="Channel n Global interrupt flag"]
  #[inline] pub fn gif(&self, index: usize) -> u32 {
     assert!(index < 7);
     let shift: usize = 0 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
#[doc="Channel n Global interrupt flag"]
  #[inline] pub fn set_gif(mut self, index: usize, value: u32) -> Self {
     assert!(index < 7);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + (index << 2);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Channel n Transfer Complete flag"]
  #[inline] pub fn tcif(&self, index: usize) -> u32 {
     assert!(index < 7);
     let shift: usize = 1 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [1]
  }
#[doc="Channel n Transfer Complete flag"]
  #[inline] pub fn set_tcif(mut self, index: usize, value: u32) -> Self {
     assert!(index < 7);
     assert!((value & !0x1) == 0);
     let shift: usize = 1 + (index << 2);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Channel n Half Transfer Complete flag"]
  #[inline] pub fn htif(&self, index: usize) -> u32 {
     assert!(index < 7);
     let shift: usize = 2 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [2]
  }
#[doc="Channel n Half Transfer Complete flag"]
  #[inline] pub fn set_htif(mut self, index: usize, value: u32) -> Self {
     assert!(index < 7);
     assert!((value & !0x1) == 0);
     let shift: usize = 2 + (index << 2);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Channel n Transfer Error flag"]
  #[inline] pub fn teif(&self, index: usize) -> u32 {
     assert!(index < 7);
     let shift: usize = 3 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [3]
  }
#[doc="Channel n Transfer Error flag"]
  #[inline] pub fn set_teif(mut self, index: usize, value: u32) -> Self {
     assert!(index < 7);
     assert!((value & !0x1) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Ifcr(pub u32);
impl Ifcr {
#[doc="Channel n Global interrupt clear"]
  #[inline] pub fn cgif(&self, index: usize) -> u32 {
     assert!(index < 7);
     let shift: usize = 0 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
#[doc="Channel n Global interrupt clear"]
  #[inline] pub fn set_cgif(mut self, index: usize, value: u32) -> Self {
     assert!(index < 7);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + (index << 2);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Channel n Transfer Complete clear"]
  #[inline] pub fn ctcif(&self, index: usize) -> u32 {
     assert!(index < 7);
     let shift: usize = 1 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [1]
  }
#[doc="Channel n Transfer Complete clear"]
  #[inline] pub fn set_ctcif(mut self, index: usize, value: u32) -> Self {
     assert!(index < 7);
     assert!((value & !0x1) == 0);
     let shift: usize = 1 + (index << 2);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Channel n Half Transfer clear"]
  #[inline] pub fn chtif(&self, index: usize) -> u32 {
     assert!(index < 7);
     let shift: usize = 2 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [2]
  }
#[doc="Channel n Half Transfer clear"]
  #[inline] pub fn set_chtif(mut self, index: usize, value: u32) -> Self {
     assert!(index < 7);
     assert!((value & !0x1) == 0);
     let shift: usize = 2 + (index << 2);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Channel n Transfer Error clear"]
  #[inline] pub fn cteif(&self, index: usize) -> u32 {
     assert!(index < 7);
     let shift: usize = 3 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [3]
  }
#[doc="Channel n Transfer Error clear"]
  #[inline] pub fn set_cteif(mut self, index: usize, value: u32) -> Self {
     assert!(index < 7);
     assert!((value & !0x1) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Ccr(pub u32);
impl Ccr {
#[doc="Channel enable"]
  #[inline] pub fn en(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Channel enable"]
  #[inline] pub fn set_en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Transfer complete interrupt enable"]
  #[inline] pub fn tcie(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Transfer complete interrupt enable"]
  #[inline] pub fn set_tcie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Half Transfer interrupt enable"]
  #[inline] pub fn htie(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Half Transfer interrupt enable"]
  #[inline] pub fn set_htie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Transfer error interrupt enable"]
  #[inline] pub fn teie(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Transfer error interrupt enable"]
  #[inline] pub fn set_teie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Data transfer direction"]
  #[inline] pub fn dir(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Data transfer direction"]
  #[inline] pub fn set_dir(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Circular mode"]
  #[inline] pub fn circ(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Circular mode"]
  #[inline] pub fn set_circ(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Peripheral increment mode"]
  #[inline] pub fn pinc(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Peripheral increment mode"]
  #[inline] pub fn set_pinc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Memory increment mode"]
  #[inline] pub fn minc(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Memory increment mode"]
  #[inline] pub fn set_minc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Peripheral size"]
  #[inline] pub fn psize(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
#[doc="Peripheral size"]
  #[inline] pub fn set_psize(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Memory size"]
  #[inline] pub fn msize(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
#[doc="Memory size"]
  #[inline] pub fn set_msize(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Channel Priority level"]
  #[inline] pub fn pl(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x3 // [13:12]
  }
#[doc="Channel Priority level"]
  #[inline] pub fn set_pl(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Memory to memory mode"]
  #[inline] pub fn mem2mem(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="Memory to memory mode"]
  #[inline] pub fn set_mem2mem(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Cndtr(pub u32);
impl Cndtr {
#[doc="Number of data to transfer"]
  #[inline] pub fn ndt(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Number of data to transfer"]
  #[inline] pub fn set_ndt(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Cpar(pub u32);
impl Cpar {
#[doc="Peripheral address"]
  #[inline] pub fn pa(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Peripheral address"]
  #[inline] pub fn set_pa(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Cmar(pub u32);
impl Cmar {
#[doc="Memory address"]
  #[inline] pub fn ma(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Memory address"]
  #[inline] pub fn set_ma(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
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
#[derive(Clone, Copy, PartialEq)]
#[doc="DMA_F3 Channel"]
pub struct Channel<P, T> { pub periph: Periph<T>, pub index: usize, pub id: P }

impl<P,T> Channel<P,T> {
   #[inline] pub fn periph(&self) -> &Periph<T> { &self.periph }
   #[inline] pub fn index(&self) -> usize { self.index }
}

