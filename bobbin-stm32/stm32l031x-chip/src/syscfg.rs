pub const SYSCFG: Syscfg = Syscfg(0x40010000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Syscfg(pub u32);
impl Syscfg {
  #[inline] pub fn cfgr1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
  #[inline] pub fn cfgr1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
  #[inline] pub fn cfgr1(&self) -> Cfgr1 { 
     unsafe {
        Cfgr1(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  #[inline] pub fn set_cfgr1(&self, value: Cfgr1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_cfgr1<F: FnOnce(Cfgr1) -> Cfgr1>(&self, f: F) -> &Self {
     let tmp = self.cfgr1();
     self.set_cfgr1(f(tmp))
  }

  #[inline] pub fn cfgr2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
  #[inline] pub fn cfgr2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
  #[inline] pub fn cfgr2(&self) -> Cfgr2 { 
     unsafe {
        Cfgr2(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
  #[inline] pub fn set_cfgr2(&self, value: Cfgr2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_cfgr2<F: FnOnce(Cfgr2) -> Cfgr2>(&self, f: F) -> &Self {
     let tmp = self.cfgr2();
     self.set_cfgr2(f(tmp))
  }

  #[inline] pub fn exticr1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
  #[inline] pub fn exticr1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
  #[inline] pub fn exticr1(&self) -> Exticr1 { 
     unsafe {
        Exticr1(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
  #[inline] pub fn set_exticr1(&self, value: Exticr1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_exticr1<F: FnOnce(Exticr1) -> Exticr1>(&self, f: F) -> &Self {
     let tmp = self.exticr1();
     self.set_exticr1(f(tmp))
  }

  #[inline] pub fn exticr2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
  #[inline] pub fn exticr2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
  #[inline] pub fn exticr2(&self) -> Exticr2 { 
     unsafe {
        Exticr2(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
  #[inline] pub fn set_exticr2(&self, value: Exticr2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_exticr2<F: FnOnce(Exticr2) -> Exticr2>(&self, f: F) -> &Self {
     let tmp = self.exticr2();
     self.set_exticr2(f(tmp))
  }

  #[inline] pub fn exticr3_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
  #[inline] pub fn exticr3_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
  #[inline] pub fn exticr3(&self) -> Exticr3 { 
     unsafe {
        Exticr3(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
  #[inline] pub fn set_exticr3(&self, value: Exticr3) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_exticr3<F: FnOnce(Exticr3) -> Exticr3>(&self, f: F) -> &Self {
     let tmp = self.exticr3();
     self.set_exticr3(f(tmp))
  }

  #[inline] pub fn exticr4_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
  #[inline] pub fn exticr4_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
  #[inline] pub fn exticr4(&self) -> Exticr4 { 
     unsafe {
        Exticr4(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
  #[inline] pub fn set_exticr4(&self, value: Exticr4) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_exticr4<F: FnOnce(Exticr4) -> Exticr4>(&self, f: F) -> &Self {
     let tmp = self.exticr4();
     self.set_exticr4(f(tmp))
  }

  #[inline] pub fn cfgr3_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
  #[inline] pub fn cfgr3_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
  #[inline] pub fn cfgr3(&self) -> Cfgr3 { 
     unsafe {
        Cfgr3(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
  #[inline] pub fn set_cfgr3(&self, value: Cfgr3) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_cfgr3<F: FnOnce(Cfgr3) -> Cfgr3>(&self, f: F) -> &Self {
     let tmp = self.cfgr3();
     self.set_cfgr3(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Cfgr1(pub u32);
impl Cfgr1 {
  #[inline] pub fn boot_mode(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  #[inline] pub fn set_boot_mode(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn mem_mode(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline] pub fn set_mem_mode(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cfgr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cfgr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.boot_mode() != 0 { try!(write!(f, " boot_mode=0x{:x}", self.boot_mode()))}
      if self.mem_mode() != 0 { try!(write!(f, " mem_mode=0x{:x}", self.mem_mode()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cfgr2(pub u32);
impl Cfgr2 {
  #[inline] pub fn i2c2_fmp(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline] pub fn set_i2c2_fmp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline] pub fn i2c1_fmp(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline] pub fn set_i2c1_fmp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn i2c_pb9_fmp(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_i2c_pb9_fmp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline] pub fn i2c_pb8_fmp(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_i2c_pb8_fmp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn i2c_pb7_fmp(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_i2c_pb7_fmp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn i2c_pb6_fmp(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_i2c_pb6_fmp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn capa(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x7 // [3:1]
  }
  #[inline] pub fn set_capa(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn fwdisen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_fwdisen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cfgr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cfgr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.i2c2_fmp() != 0 { try!(write!(f, " i2c2_fmp"))}
      if self.i2c1_fmp() != 0 { try!(write!(f, " i2c1_fmp"))}
      if self.i2c_pb9_fmp() != 0 { try!(write!(f, " i2c_pb9_fmp"))}
      if self.i2c_pb8_fmp() != 0 { try!(write!(f, " i2c_pb8_fmp"))}
      if self.i2c_pb7_fmp() != 0 { try!(write!(f, " i2c_pb7_fmp"))}
      if self.i2c_pb6_fmp() != 0 { try!(write!(f, " i2c_pb6_fmp"))}
      if self.capa() != 0 { try!(write!(f, " capa=0x{:x}", self.capa()))}
      if self.fwdisen() != 0 { try!(write!(f, " fwdisen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Exticr1(pub u32);
impl Exticr1 {
  #[inline] pub fn exti3(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0xf // [15:12]
  }
  #[inline] pub fn set_exti3(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn exti2(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  #[inline] pub fn set_exti2(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn exti1(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
  #[inline] pub fn set_exti1(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn exti0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_exti0(mut self, value: u32) -> Self {
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
  #[inline] pub fn exti7(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0xf // [15:12]
  }
  #[inline] pub fn set_exti7(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn exti6(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  #[inline] pub fn set_exti6(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn exti5(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
  #[inline] pub fn set_exti5(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn exti4(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_exti4(mut self, value: u32) -> Self {
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
  #[inline] pub fn exti11(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0xf // [15:12]
  }
  #[inline] pub fn set_exti11(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn exti10(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  #[inline] pub fn set_exti10(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn exti9(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
  #[inline] pub fn set_exti9(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn exti8(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_exti8(mut self, value: u32) -> Self {
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
  #[inline] pub fn exti15(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0xf // [15:12]
  }
  #[inline] pub fn set_exti15(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn exti14(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  #[inline] pub fn set_exti14(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn exti13(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
  #[inline] pub fn set_exti13(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn exti12(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_exti12(mut self, value: u32) -> Self {
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
pub struct Cfgr3(pub u32);
impl Cfgr3 {
  #[inline] pub fn ref_lock(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  #[inline] pub fn set_ref_lock(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

  #[inline] pub fn vrefint_rdyf(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  #[inline] pub fn set_vrefint_rdyf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  #[inline] pub fn vrefint_comp_rdyf(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  #[inline] pub fn set_vrefint_comp_rdyf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  #[inline] pub fn vrefint_adc_rdyf(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  #[inline] pub fn set_vrefint_adc_rdyf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

  #[inline] pub fn sensor_adc_rdyf(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  #[inline] pub fn set_sensor_adc_rdyf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  #[inline] pub fn ref_rc48mhz_rdyf(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  #[inline] pub fn set_ref_rc48mhz_rdyf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  #[inline] pub fn enref_rc48mhz(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline] pub fn set_enref_rc48mhz(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline] pub fn enbuf_vrefint_comp(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline] pub fn set_enbuf_vrefint_comp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn enbuf_sensor_adc(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_enbuf_sensor_adc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn enbuf_bgap_adc(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_enbuf_bgap_adc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn sel_vref_out(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  #[inline] pub fn set_sel_vref_out(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn en_bgap(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_en_bgap(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cfgr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cfgr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ref_lock() != 0 { try!(write!(f, " ref_lock"))}
      if self.vrefint_rdyf() != 0 { try!(write!(f, " vrefint_rdyf"))}
      if self.vrefint_comp_rdyf() != 0 { try!(write!(f, " vrefint_comp_rdyf"))}
      if self.vrefint_adc_rdyf() != 0 { try!(write!(f, " vrefint_adc_rdyf"))}
      if self.sensor_adc_rdyf() != 0 { try!(write!(f, " sensor_adc_rdyf"))}
      if self.ref_rc48mhz_rdyf() != 0 { try!(write!(f, " ref_rc48mhz_rdyf"))}
      if self.enref_rc48mhz() != 0 { try!(write!(f, " enref_rc48mhz"))}
      if self.enbuf_vrefint_comp() != 0 { try!(write!(f, " enbuf_vrefint_comp"))}
      if self.enbuf_sensor_adc() != 0 { try!(write!(f, " enbuf_sensor_adc"))}
      if self.enbuf_bgap_adc() != 0 { try!(write!(f, " enbuf_bgap_adc"))}
      if self.sel_vref_out() != 0 { try!(write!(f, " sel_vref_out=0x{:x}", self.sel_vref_out()))}
      if self.en_bgap() != 0 { try!(write!(f, " en_bgap"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

