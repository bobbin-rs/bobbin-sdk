
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Periph<T>(pub u32, pub T); 



impl<T> Periph<T> {
  #[inline]
  pub fn pcr_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 32);
     ((self.0 as usize) + 0x0 + (index << 2)) as *const u32
  }
  #[inline]
  pub fn pcr_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 32);
     ((self.0 as usize) + 0x0 + (index << 2)) as *mut u32
  }
  #[inline]
  pub fn pcr(&self, index: usize) -> Pcr { 
     assert!(index < 32);
     unsafe {
        Pcr(::core::ptr::read_volatile(((self.0 as usize) + 0x0 + (index << 2)) as *const u32))
     }
  }
  #[inline]
  pub fn set_pcr(&self, index: usize, value: Pcr) -> &Self {
     assert!(index < 32);
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x0 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_pcr<F: FnOnce(Pcr) -> Pcr>(&self, index: usize, f: F) -> &Self {
     let tmp = self.pcr(index);
     self.set_pcr(index, f(tmp))
  }

  #[inline]
  pub fn gpclr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x80) as *const u32
  }
  #[inline]
  pub fn gpclr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x80) as *mut u32
  }
  #[inline]
  pub fn set_gpclr(&self, value: Gpclr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x80) as *mut u32, value.0);
     }
     self
  }

  #[inline]
  pub fn gpchr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x84) as *const u32
  }
  #[inline]
  pub fn gpchr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x84) as *mut u32
  }
  #[inline]
  pub fn set_gpchr(&self, value: Gpchr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x84) as *mut u32, value.0);
     }
     self
  }

  #[inline]
  pub fn isfr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa0) as *const u32
  }
  #[inline]
  pub fn isfr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa0) as *mut u32
  }
  #[inline]
  pub fn isfr(&self) -> Isfr { 
     unsafe {
       Isfr(::core::ptr::read_volatile(((self.0 as usize) + 0xa0) as *const u32))
     }
  }
  #[inline]
  pub fn set_isfr(&self, value: Isfr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xa0) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_isfr<F: FnOnce(Isfr) -> Isfr>(&self, f: F) -> &Self {
     let tmp = self.isfr();
     self.set_isfr(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Pcr(pub u32);
impl Pcr {
  #[inline]
  pub fn ps(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_ps(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn pe(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_pe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn sre(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_sre(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn pfe(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_pfe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn ode(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_ode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn dse(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_dse(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn mux(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x7 // [10:8]
  }
  #[inline]
  pub fn set_mux(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn lk(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  #[inline]
  pub fn set_lk(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  #[inline]
  pub fn irqc(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  #[inline]
  pub fn set_irqc(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn isf(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  #[inline]
  pub fn set_isf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

}
impl ::core::fmt::Display for Pcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ps() != 0 { try!(write!(f, " ps"))}
      if self.pe() != 0 { try!(write!(f, " pe"))}
      if self.sre() != 0 { try!(write!(f, " sre"))}
      if self.pfe() != 0 { try!(write!(f, " pfe"))}
      if self.ode() != 0 { try!(write!(f, " ode"))}
      if self.dse() != 0 { try!(write!(f, " dse"))}
      if self.mux() != 0 { try!(write!(f, " mux=0x{:x}", self.mux()))}
      if self.lk() != 0 { try!(write!(f, " lk"))}
      if self.irqc() != 0 { try!(write!(f, " irqc=0x{:x}", self.irqc()))}
      if self.isf() != 0 { try!(write!(f, " isf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Gpclr(pub u32);
impl Gpclr {
  #[inline]
  pub fn gpwd(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline]
  pub fn set_gpwd(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn gpwe(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xffff // [31:16]
  }
  #[inline]
  pub fn set_gpwe(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Gpclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Gpclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.gpwd() != 0 { try!(write!(f, " gpwd=0x{:x}", self.gpwd()))}
      if self.gpwe() != 0 { try!(write!(f, " gpwe=0x{:x}", self.gpwe()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Gpchr(pub u32);
impl Gpchr {
  #[inline]
  pub fn gpwd(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline]
  pub fn set_gpwd(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn gpwe(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xffff // [31:16]
  }
  #[inline]
  pub fn set_gpwe(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Gpchr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Gpchr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.gpwd() != 0 { try!(write!(f, " gpwd=0x{:x}", self.gpwd()))}
      if self.gpwe() != 0 { try!(write!(f, " gpwe=0x{:x}", self.gpwe()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Isfr(pub u32);
impl Isfr {
  #[inline]
  pub fn isf(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_isf(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Isfr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Isfr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.isf(0) != 0 { try!(write!(f, " isf[0]"))}
      if self.isf(1) != 0 { try!(write!(f, " isf[1]"))}
      if self.isf(2) != 0 { try!(write!(f, " isf[2]"))}
      if self.isf(3) != 0 { try!(write!(f, " isf[3]"))}
      if self.isf(4) != 0 { try!(write!(f, " isf[4]"))}
      if self.isf(5) != 0 { try!(write!(f, " isf[5]"))}
      if self.isf(6) != 0 { try!(write!(f, " isf[6]"))}
      if self.isf(7) != 0 { try!(write!(f, " isf[7]"))}
      if self.isf(8) != 0 { try!(write!(f, " isf[8]"))}
      if self.isf(9) != 0 { try!(write!(f, " isf[9]"))}
      if self.isf(10) != 0 { try!(write!(f, " isf[10]"))}
      if self.isf(11) != 0 { try!(write!(f, " isf[11]"))}
      if self.isf(12) != 0 { try!(write!(f, " isf[12]"))}
      if self.isf(13) != 0 { try!(write!(f, " isf[13]"))}
      if self.isf(14) != 0 { try!(write!(f, " isf[14]"))}
      if self.isf(15) != 0 { try!(write!(f, " isf[15]"))}
      if self.isf(16) != 0 { try!(write!(f, " isf[16]"))}
      if self.isf(17) != 0 { try!(write!(f, " isf[17]"))}
      if self.isf(18) != 0 { try!(write!(f, " isf[18]"))}
      if self.isf(19) != 0 { try!(write!(f, " isf[19]"))}
      if self.isf(20) != 0 { try!(write!(f, " isf[20]"))}
      if self.isf(21) != 0 { try!(write!(f, " isf[21]"))}
      if self.isf(22) != 0 { try!(write!(f, " isf[22]"))}
      if self.isf(23) != 0 { try!(write!(f, " isf[23]"))}
      if self.isf(24) != 0 { try!(write!(f, " isf[24]"))}
      if self.isf(25) != 0 { try!(write!(f, " isf[25]"))}
      if self.isf(26) != 0 { try!(write!(f, " isf[26]"))}
      if self.isf(27) != 0 { try!(write!(f, " isf[27]"))}
      if self.isf(28) != 0 { try!(write!(f, " isf[28]"))}
      if self.isf(29) != 0 { try!(write!(f, " isf[29]"))}
      if self.isf(30) != 0 { try!(write!(f, " isf[30]"))}
      if self.isf(31) != 0 { try!(write!(f, " isf[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
pub struct Pin<P, T> { pub port: Periph<T>, pub index: usize, pub id: P }

impl<P,T> Pin<P,T> {
   #[inline] pub fn port(&self) -> &Periph<T> { &self.port }
   #[inline] pub fn index(&self) -> usize { self.index }
}
pub trait AltFn<T> {
   #[inline] fn alt_fn(&self) -> usize;
}

