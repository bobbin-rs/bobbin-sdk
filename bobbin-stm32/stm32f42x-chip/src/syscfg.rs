pub const SYSCFG: Syscfg = Syscfg(0x40013800);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Syscfg(pub u32);
impl Syscfg {
  #[inline]
  pub fn memrm_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
  #[inline]
  pub fn memrm_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
  #[inline]
  pub fn memrm(&self) -> Memrm { 
     unsafe {
       Memrm(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  #[inline]
  pub fn set_memrm(&self, value: Memrm) -> &Syscfg {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_memrm<F: FnOnce(Memrm) -> Memrm>(&self, f: F) -> &Syscfg {
     let tmp = self.memrm();
     self.set_memrm(f(tmp))
  }

  #[inline]
  pub fn pmc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
  #[inline]
  pub fn pmc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
  #[inline]
  pub fn pmc(&self) -> Pmc { 
     unsafe {
       Pmc(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
  #[inline]
  pub fn set_pmc(&self, value: Pmc) -> &Syscfg {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_pmc<F: FnOnce(Pmc) -> Pmc>(&self, f: F) -> &Syscfg {
     let tmp = self.pmc();
     self.set_pmc(f(tmp))
  }

  #[inline]
  pub fn exticr1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
  #[inline]
  pub fn exticr1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
  #[inline]
  pub fn exticr1(&self) -> Exticr1 { 
     unsafe {
       Exticr1(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
  #[inline]
  pub fn set_exticr1(&self, value: Exticr1) -> &Syscfg {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_exticr1<F: FnOnce(Exticr1) -> Exticr1>(&self, f: F) -> &Syscfg {
     let tmp = self.exticr1();
     self.set_exticr1(f(tmp))
  }

  #[inline]
  pub fn exticr2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
  #[inline]
  pub fn exticr2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
  #[inline]
  pub fn exticr2(&self) -> Exticr2 { 
     unsafe {
       Exticr2(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
  #[inline]
  pub fn set_exticr2(&self, value: Exticr2) -> &Syscfg {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_exticr2<F: FnOnce(Exticr2) -> Exticr2>(&self, f: F) -> &Syscfg {
     let tmp = self.exticr2();
     self.set_exticr2(f(tmp))
  }

  #[inline]
  pub fn exticr3_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
  #[inline]
  pub fn exticr3_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
  #[inline]
  pub fn exticr3(&self) -> Exticr3 { 
     unsafe {
       Exticr3(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
  #[inline]
  pub fn set_exticr3(&self, value: Exticr3) -> &Syscfg {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_exticr3<F: FnOnce(Exticr3) -> Exticr3>(&self, f: F) -> &Syscfg {
     let tmp = self.exticr3();
     self.set_exticr3(f(tmp))
  }

  #[inline]
  pub fn exticr4_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
  #[inline]
  pub fn exticr4_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
  #[inline]
  pub fn exticr4(&self) -> Exticr4 { 
     unsafe {
       Exticr4(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
  #[inline]
  pub fn set_exticr4(&self, value: Exticr4) -> &Syscfg {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_exticr4<F: FnOnce(Exticr4) -> Exticr4>(&self, f: F) -> &Syscfg {
     let tmp = self.exticr4();
     self.set_exticr4(f(tmp))
  }

  #[inline]
  pub fn cmpcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
  #[inline]
  pub fn cmpcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
  #[inline]
  pub fn cmpcr(&self) -> Cmpcr { 
     unsafe {
       Cmpcr(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }

}

#[derive(PartialEq, Eq)]
pub struct Memrm(pub u32);
impl Memrm {
  #[inline]
  pub fn mem_mode(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline]
  pub fn set_mem_mode(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Memrm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Memrm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mem_mode() != 0 { try!(write!(f, " mem_mode=0x{:x}", self.mem_mode()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Pmc(pub u32);
impl Pmc {
  #[inline]
  pub fn mii_rmii_sel(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  #[inline]
  pub fn set_mii_rmii_sel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

}
impl ::core::fmt::Display for Pmc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pmc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mii_rmii_sel() != 0 { try!(write!(f, " mii_rmii_sel"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Exticr1(pub u32);
impl Exticr1 {
  #[inline]
  pub fn exti3(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0xf // [15:12]
  }
  #[inline]
  pub fn set_exti3(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn exti2(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  #[inline]
  pub fn set_exti2(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn exti1(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
  #[inline]
  pub fn set_exti1(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn exti0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  #[inline]
  pub fn set_exti0(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Exticr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Exticr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.exti3() != 0 { try!(write!(f, " exti3=0x{:x}", self.exti3()))}
      if self.exti2() != 0 { try!(write!(f, " exti2=0x{:x}", self.exti2()))}
      if self.exti1() != 0 { try!(write!(f, " exti1=0x{:x}", self.exti1()))}
      if self.exti0() != 0 { try!(write!(f, " exti0=0x{:x}", self.exti0()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Exticr2(pub u32);
impl Exticr2 {
  #[inline]
  pub fn exti7(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0xf // [15:12]
  }
  #[inline]
  pub fn set_exti7(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn exti6(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  #[inline]
  pub fn set_exti6(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn exti5(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
  #[inline]
  pub fn set_exti5(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn exti4(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  #[inline]
  pub fn set_exti4(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Exticr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Exticr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.exti7() != 0 { try!(write!(f, " exti7=0x{:x}", self.exti7()))}
      if self.exti6() != 0 { try!(write!(f, " exti6=0x{:x}", self.exti6()))}
      if self.exti5() != 0 { try!(write!(f, " exti5=0x{:x}", self.exti5()))}
      if self.exti4() != 0 { try!(write!(f, " exti4=0x{:x}", self.exti4()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Exticr3(pub u32);
impl Exticr3 {
  #[inline]
  pub fn exti11(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0xf // [15:12]
  }
  #[inline]
  pub fn set_exti11(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn exti10(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  #[inline]
  pub fn set_exti10(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn exti9(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
  #[inline]
  pub fn set_exti9(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn exti8(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  #[inline]
  pub fn set_exti8(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Exticr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Exticr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.exti11() != 0 { try!(write!(f, " exti11=0x{:x}", self.exti11()))}
      if self.exti10() != 0 { try!(write!(f, " exti10=0x{:x}", self.exti10()))}
      if self.exti9() != 0 { try!(write!(f, " exti9=0x{:x}", self.exti9()))}
      if self.exti8() != 0 { try!(write!(f, " exti8=0x{:x}", self.exti8()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Exticr4(pub u32);
impl Exticr4 {
  #[inline]
  pub fn exti15(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0xf // [15:12]
  }
  #[inline]
  pub fn set_exti15(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn exti14(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  #[inline]
  pub fn set_exti14(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn exti13(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
  #[inline]
  pub fn set_exti13(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn exti12(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  #[inline]
  pub fn set_exti12(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Exticr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Exticr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.exti15() != 0 { try!(write!(f, " exti15=0x{:x}", self.exti15()))}
      if self.exti14() != 0 { try!(write!(f, " exti14=0x{:x}", self.exti14()))}
      if self.exti13() != 0 { try!(write!(f, " exti13=0x{:x}", self.exti13()))}
      if self.exti12() != 0 { try!(write!(f, " exti12=0x{:x}", self.exti12()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cmpcr(pub u32);
impl Cmpcr {
  #[inline]
  pub fn ready(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_ready(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn cmp_pd(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_cmp_pd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cmpcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cmpcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ready() != 0 { try!(write!(f, " ready"))}
      if self.cmp_pd() != 0 { try!(write!(f, " cmp_pd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
