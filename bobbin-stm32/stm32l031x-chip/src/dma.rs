pub const DMA1: Dma = Dma(0x40020000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dma(pub u32);
impl Dma {
  #[inline]
  pub fn isr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
  #[inline]
  pub fn isr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
  #[inline]
  pub fn isr(&self) -> Isr { 
     unsafe {
       Isr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }

  #[inline]
  pub fn ifcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
  #[inline]
  pub fn ifcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
  #[inline]
  pub fn set_ifcr(&self, value: Ifcr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }

  #[inline]
  pub fn ccr_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 7);
     ((self.0 as usize) + 0x8 + (index * 20)) as *const u32
  }
  #[inline]
  pub fn ccr_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 7);
     ((self.0 as usize) + 0x8 + (index * 20)) as *mut u32
  }
  #[inline]
  pub fn ccr(&self, index: usize) -> Ccr { 
     assert!(index < 7);
     unsafe {
        Ccr(::core::ptr::read_volatile(((self.0 as usize) + 0x8 + (index * 20)) as *const u32))
     }
  }
  #[inline]
  pub fn set_ccr(&self, index: usize, value: Ccr) -> &Self {
     assert!(index < 7);
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x8 + (index * 20)) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ccr<F: FnOnce(Ccr) -> Ccr>(&self, index: usize, f: F) -> &Self {
     let tmp = self.ccr(index);
     self.set_ccr(index, f(tmp))
  }

  #[inline]
  pub fn cndtr_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 7);
     ((self.0 as usize) + 0xc + (index * 20)) as *const u32
  }
  #[inline]
  pub fn cndtr_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 7);
     ((self.0 as usize) + 0xc + (index * 20)) as *mut u32
  }
  #[inline]
  pub fn cndtr(&self, index: usize) -> Cndtr { 
     assert!(index < 7);
     unsafe {
        Cndtr(::core::ptr::read_volatile(((self.0 as usize) + 0xc + (index * 20)) as *const u32))
     }
  }
  #[inline]
  pub fn set_cndtr(&self, index: usize, value: Cndtr) -> &Self {
     assert!(index < 7);
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc + (index * 20)) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_cndtr<F: FnOnce(Cndtr) -> Cndtr>(&self, index: usize, f: F) -> &Self {
     let tmp = self.cndtr(index);
     self.set_cndtr(index, f(tmp))
  }

  #[inline]
  pub fn cpar_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 7);
     ((self.0 as usize) + 0x10 + (index * 20)) as *const u32
  }
  #[inline]
  pub fn cpar_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 7);
     ((self.0 as usize) + 0x10 + (index * 20)) as *mut u32
  }
  #[inline]
  pub fn cpar(&self, index: usize) -> Cpar { 
     assert!(index < 7);
     unsafe {
        Cpar(::core::ptr::read_volatile(((self.0 as usize) + 0x10 + (index * 20)) as *const u32))
     }
  }
  #[inline]
  pub fn set_cpar(&self, index: usize, value: Cpar) -> &Self {
     assert!(index < 7);
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x10 + (index * 20)) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_cpar<F: FnOnce(Cpar) -> Cpar>(&self, index: usize, f: F) -> &Self {
     let tmp = self.cpar(index);
     self.set_cpar(index, f(tmp))
  }

  #[inline]
  pub fn cmar_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 7);
     ((self.0 as usize) + 0x14 + (index * 20)) as *const u32
  }
  #[inline]
  pub fn cmar_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 7);
     ((self.0 as usize) + 0x14 + (index * 20)) as *mut u32
  }
  #[inline]
  pub fn cmar(&self, index: usize) -> Cmar { 
     assert!(index < 7);
     unsafe {
        Cmar(::core::ptr::read_volatile(((self.0 as usize) + 0x14 + (index * 20)) as *const u32))
     }
  }
  #[inline]
  pub fn set_cmar(&self, index: usize, value: Cmar) -> &Self {
     assert!(index < 7);
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x14 + (index * 20)) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_cmar<F: FnOnce(Cmar) -> Cmar>(&self, index: usize, f: F) -> &Self {
     let tmp = self.cmar(index);
     self.set_cmar(index, f(tmp))
  }

  #[inline]
  pub fn cselr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa8) as *const u32
  }
  #[inline]
  pub fn cselr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa8) as *mut u32
  }
  #[inline]
  pub fn cselr(&self) -> Cselr { 
     unsafe {
       Cselr(::core::ptr::read_volatile(((self.0 as usize) + 0xa8) as *const u32))
     }
  }
  #[inline]
  pub fn set_cselr(&self, value: Cselr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xa8) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_cselr<F: FnOnce(Cselr) -> Cselr>(&self, f: F) -> &Self {
     let tmp = self.cselr();
     self.set_cselr(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
  #[inline]
  pub fn teif(&self, index: usize) -> u32 {
     assert!(index < 7);
     let shift: usize = 3 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [3]
  }
  #[inline]
  pub fn set_teif(mut self, index: usize, value: u32) -> Self {
     assert!(index < 7);
     assert!((value & !0x1) == 0);
     let shift: usize = 3 + (index << 2);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline]
  pub fn htif(&self, index: usize) -> u32 {
     assert!(index < 7);
     let shift: usize = 2 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [2]
  }
  #[inline]
  pub fn set_htif(mut self, index: usize, value: u32) -> Self {
     assert!(index < 7);
     assert!((value & !0x1) == 0);
     let shift: usize = 2 + (index << 2);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline]
  pub fn tcif(&self, index: usize) -> u32 {
     assert!(index < 7);
     let shift: usize = 1 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [1]
  }
  #[inline]
  pub fn set_tcif(mut self, index: usize, value: u32) -> Self {
     assert!(index < 7);
     assert!((value & !0x1) == 0);
     let shift: usize = 1 + (index << 2);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline]
  pub fn gif(&self, index: usize) -> u32 {
     assert!(index < 7);
     let shift: usize = 0 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_gif(mut self, index: usize, value: u32) -> Self {
     assert!(index < 7);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + (index << 2);
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
      if self.teif(0) != 0 { try!(write!(f, " teif[0]"))}
      if self.teif(1) != 0 { try!(write!(f, " teif[1]"))}
      if self.teif(2) != 0 { try!(write!(f, " teif[2]"))}
      if self.teif(3) != 0 { try!(write!(f, " teif[3]"))}
      if self.teif(4) != 0 { try!(write!(f, " teif[4]"))}
      if self.teif(5) != 0 { try!(write!(f, " teif[5]"))}
      if self.teif(6) != 0 { try!(write!(f, " teif[6]"))}
      if self.htif(0) != 0 { try!(write!(f, " htif[0]"))}
      if self.htif(1) != 0 { try!(write!(f, " htif[1]"))}
      if self.htif(2) != 0 { try!(write!(f, " htif[2]"))}
      if self.htif(3) != 0 { try!(write!(f, " htif[3]"))}
      if self.htif(4) != 0 { try!(write!(f, " htif[4]"))}
      if self.htif(5) != 0 { try!(write!(f, " htif[5]"))}
      if self.htif(6) != 0 { try!(write!(f, " htif[6]"))}
      if self.tcif(0) != 0 { try!(write!(f, " tcif[0]"))}
      if self.tcif(1) != 0 { try!(write!(f, " tcif[1]"))}
      if self.tcif(2) != 0 { try!(write!(f, " tcif[2]"))}
      if self.tcif(3) != 0 { try!(write!(f, " tcif[3]"))}
      if self.tcif(4) != 0 { try!(write!(f, " tcif[4]"))}
      if self.tcif(5) != 0 { try!(write!(f, " tcif[5]"))}
      if self.tcif(6) != 0 { try!(write!(f, " tcif[6]"))}
      if self.gif(0) != 0 { try!(write!(f, " gif[0]"))}
      if self.gif(1) != 0 { try!(write!(f, " gif[1]"))}
      if self.gif(2) != 0 { try!(write!(f, " gif[2]"))}
      if self.gif(3) != 0 { try!(write!(f, " gif[3]"))}
      if self.gif(4) != 0 { try!(write!(f, " gif[4]"))}
      if self.gif(5) != 0 { try!(write!(f, " gif[5]"))}
      if self.gif(6) != 0 { try!(write!(f, " gif[6]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ifcr(pub u32);
impl Ifcr {
  #[inline]
  pub fn cteif(&self, index: usize) -> u32 {
     assert!(index < 7);
     let shift: usize = 3 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [3]
  }
  #[inline]
  pub fn set_cteif(mut self, index: usize, value: u32) -> Self {
     assert!(index < 7);
     assert!((value & !0x1) == 0);
     let shift: usize = 3 + (index << 2);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline]
  pub fn chtif(&self, index: usize) -> u32 {
     assert!(index < 7);
     let shift: usize = 2 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [2]
  }
  #[inline]
  pub fn set_chtif(mut self, index: usize, value: u32) -> Self {
     assert!(index < 7);
     assert!((value & !0x1) == 0);
     let shift: usize = 2 + (index << 2);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline]
  pub fn ctcif(&self, index: usize) -> u32 {
     assert!(index < 7);
     let shift: usize = 1 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [1]
  }
  #[inline]
  pub fn set_ctcif(mut self, index: usize, value: u32) -> Self {
     assert!(index < 7);
     assert!((value & !0x1) == 0);
     let shift: usize = 1 + (index << 2);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline]
  pub fn cgif(&self, index: usize) -> u32 {
     assert!(index < 7);
     let shift: usize = 0 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_cgif(mut self, index: usize, value: u32) -> Self {
     assert!(index < 7);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + (index << 2);
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
      if self.cteif(0) != 0 { try!(write!(f, " cteif[0]"))}
      if self.cteif(1) != 0 { try!(write!(f, " cteif[1]"))}
      if self.cteif(2) != 0 { try!(write!(f, " cteif[2]"))}
      if self.cteif(3) != 0 { try!(write!(f, " cteif[3]"))}
      if self.cteif(4) != 0 { try!(write!(f, " cteif[4]"))}
      if self.cteif(5) != 0 { try!(write!(f, " cteif[5]"))}
      if self.cteif(6) != 0 { try!(write!(f, " cteif[6]"))}
      if self.chtif(0) != 0 { try!(write!(f, " chtif[0]"))}
      if self.chtif(1) != 0 { try!(write!(f, " chtif[1]"))}
      if self.chtif(2) != 0 { try!(write!(f, " chtif[2]"))}
      if self.chtif(3) != 0 { try!(write!(f, " chtif[3]"))}
      if self.chtif(4) != 0 { try!(write!(f, " chtif[4]"))}
      if self.chtif(5) != 0 { try!(write!(f, " chtif[5]"))}
      if self.chtif(6) != 0 { try!(write!(f, " chtif[6]"))}
      if self.ctcif(0) != 0 { try!(write!(f, " ctcif[0]"))}
      if self.ctcif(1) != 0 { try!(write!(f, " ctcif[1]"))}
      if self.ctcif(2) != 0 { try!(write!(f, " ctcif[2]"))}
      if self.ctcif(3) != 0 { try!(write!(f, " ctcif[3]"))}
      if self.ctcif(4) != 0 { try!(write!(f, " ctcif[4]"))}
      if self.ctcif(5) != 0 { try!(write!(f, " ctcif[5]"))}
      if self.ctcif(6) != 0 { try!(write!(f, " ctcif[6]"))}
      if self.cgif(0) != 0 { try!(write!(f, " cgif[0]"))}
      if self.cgif(1) != 0 { try!(write!(f, " cgif[1]"))}
      if self.cgif(2) != 0 { try!(write!(f, " cgif[2]"))}
      if self.cgif(3) != 0 { try!(write!(f, " cgif[3]"))}
      if self.cgif(4) != 0 { try!(write!(f, " cgif[4]"))}
      if self.cgif(5) != 0 { try!(write!(f, " cgif[5]"))}
      if self.cgif(6) != 0 { try!(write!(f, " cgif[6]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ccr(pub u32);
impl Ccr {
  #[inline]
  pub fn mem2mem(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline]
  pub fn set_mem2mem(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline]
  pub fn pl(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x3 // [13:12]
  }
  #[inline]
  pub fn set_pl(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn msize(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
  #[inline]
  pub fn set_msize(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn psize(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  #[inline]
  pub fn set_psize(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn minc(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_minc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn pinc(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_pinc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn circ(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_circ(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn dir(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_dir(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn teie(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_teie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn htie(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_htie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn tcie(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_tcie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn en(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
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
      if self.mem2mem() != 0 { try!(write!(f, " mem2mem"))}
      if self.pl() != 0 { try!(write!(f, " pl=0x{:x}", self.pl()))}
      if self.msize() != 0 { try!(write!(f, " msize=0x{:x}", self.msize()))}
      if self.psize() != 0 { try!(write!(f, " psize=0x{:x}", self.psize()))}
      if self.minc() != 0 { try!(write!(f, " minc"))}
      if self.pinc() != 0 { try!(write!(f, " pinc"))}
      if self.circ() != 0 { try!(write!(f, " circ"))}
      if self.dir() != 0 { try!(write!(f, " dir"))}
      if self.teie() != 0 { try!(write!(f, " teie"))}
      if self.htie() != 0 { try!(write!(f, " htie"))}
      if self.tcie() != 0 { try!(write!(f, " tcie"))}
      if self.en() != 0 { try!(write!(f, " en"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cndtr(pub u32);
impl Cndtr {
  #[inline]
  pub fn ndt(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline]
  pub fn set_ndt(mut self, value: u32) -> Self {
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
#[derive(PartialEq, Eq)]
pub struct Cpar(pub u32);
impl Cpar {
  #[inline]
  pub fn pa(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
  pub fn set_pa(mut self, value: u32) -> Self {
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
#[derive(PartialEq, Eq)]
pub struct Cmar(pub u32);
impl Cmar {
  #[inline]
  pub fn ma(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
  pub fn set_ma(mut self, value: u32) -> Self {
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
#[derive(PartialEq, Eq)]
pub struct Cselr(pub u32);
impl Cselr {
  #[inline]
  pub fn cs(&self, index: usize) -> u32 {
     assert!(index < 7);
     let shift: usize = 0 + (index << 2);
     ((self.0 as u32) >> shift) & 0xf // [3:0]
  }
  #[inline]
  pub fn set_cs(mut self, index: usize, value: u32) -> Self {
     assert!(index < 7);
     assert!((value & !0xf) == 0);
     let shift: usize = 0 + (index << 2);
     self.0 &= !(0xf << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Cselr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cselr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cs(0) != 0 { try!(write!(f, " cs[0]=0x{:x}", self.cs(0)))}
      if self.cs(1) != 0 { try!(write!(f, " cs[1]=0x{:x}", self.cs(1)))}
      if self.cs(2) != 0 { try!(write!(f, " cs[2]=0x{:x}", self.cs(2)))}
      if self.cs(3) != 0 { try!(write!(f, " cs[3]=0x{:x}", self.cs(3)))}
      if self.cs(4) != 0 { try!(write!(f, " cs[4]=0x{:x}", self.cs(4)))}
      if self.cs(5) != 0 { try!(write!(f, " cs[5]=0x{:x}", self.cs(5)))}
      if self.cs(6) != 0 { try!(write!(f, " cs[6]=0x{:x}", self.cs(6)))}
      try!(write!(f, "]"));
      Ok(())
   }
}

