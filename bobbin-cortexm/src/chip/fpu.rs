pub const FPU: Fpu = Fpu(0xe000e000);

#[doc="FPU"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fpu(pub u32);
impl Fpu {
#[doc="Get the *const pointer for the CPACR register."]
  #[inline] pub fn cpacr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xd88) as *const u32
  }
#[doc="Get the *mut pointer for the CPACR register."]
  #[inline] pub fn cpacr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xd88) as *mut u32
  }
#[doc="Read the CPACR register."]
  #[inline] pub fn cpacr(&self) -> Cpacr { 
     unsafe {
        Cpacr(::core::ptr::read_volatile(((self.0 as usize) + 0xd88) as *const u32))
     }
  }
#[doc="Write the CPACR register."]
  #[inline] pub fn set_cpacr(&self, value: Cpacr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd88) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CPACR register."]
  #[inline] pub fn with_cpacr<F: FnOnce(Cpacr) -> Cpacr>(&self, f: F) -> &Self {
     let tmp = self.cpacr();
     self.set_cpacr(f(tmp))
  }

#[doc="Get the *const pointer for the FPCCR register."]
  #[inline] pub fn fpccr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xf34) as *const u32
  }
#[doc="Get the *mut pointer for the FPCCR register."]
  #[inline] pub fn fpccr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xf34) as *mut u32
  }
#[doc="Read the FPCCR register."]
  #[inline] pub fn fpccr(&self) -> Fpccr { 
     unsafe {
        Fpccr(::core::ptr::read_volatile(((self.0 as usize) + 0xf34) as *const u32))
     }
  }
#[doc="Write the FPCCR register."]
  #[inline] pub fn set_fpccr(&self, value: Fpccr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xf34) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the FPCCR register."]
  #[inline] pub fn with_fpccr<F: FnOnce(Fpccr) -> Fpccr>(&self, f: F) -> &Self {
     let tmp = self.fpccr();
     self.set_fpccr(f(tmp))
  }

#[doc="Get the *const pointer for the FPCAR register."]
  #[inline] pub fn fpcar_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xf38) as *const u32
  }
#[doc="Get the *mut pointer for the FPCAR register."]
  #[inline] pub fn fpcar_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xf38) as *mut u32
  }
#[doc="Read the FPCAR register."]
  #[inline] pub fn fpcar(&self) -> Fpcar { 
     unsafe {
        Fpcar(::core::ptr::read_volatile(((self.0 as usize) + 0xf38) as *const u32))
     }
  }
#[doc="Write the FPCAR register."]
  #[inline] pub fn set_fpcar(&self, value: Fpcar) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xf38) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the FPCAR register."]
  #[inline] pub fn with_fpcar<F: FnOnce(Fpcar) -> Fpcar>(&self, f: F) -> &Self {
     let tmp = self.fpcar();
     self.set_fpcar(f(tmp))
  }

#[doc="Get the *const pointer for the FPDSCR register."]
  #[inline] pub fn fpdscr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xf3c) as *const u32
  }
#[doc="Get the *mut pointer for the FPDSCR register."]
  #[inline] pub fn fpdscr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xf3c) as *mut u32
  }
#[doc="Read the FPDSCR register."]
  #[inline] pub fn fpdscr(&self) -> Fpdscr { 
     unsafe {
        Fpdscr(::core::ptr::read_volatile(((self.0 as usize) + 0xf3c) as *const u32))
     }
  }
#[doc="Write the FPDSCR register."]
  #[inline] pub fn set_fpdscr(&self, value: Fpdscr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xf3c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the FPDSCR register."]
  #[inline] pub fn with_fpdscr<F: FnOnce(Fpdscr) -> Fpdscr>(&self, f: F) -> &Self {
     let tmp = self.fpdscr();
     self.set_fpdscr(f(tmp))
  }

}

#[doc="Coprocessor Access Control Register"]
#[derive(PartialEq, Eq)]
pub struct Cpacr(pub u32);
impl Cpacr {
#[doc="CP10 Access privileges"]
  #[inline] pub fn cp10(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x3 // [21:20]
  }
#[doc="CP10 Access privileges"]
  #[inline] pub fn set_cp10(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="CP11 Access privileges"]
  #[inline] pub fn cp11(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x3 // [23:22]
  }
#[doc="CP11 Access privileges"]
  #[inline] pub fn set_cp11(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 22);
     self.0 |= value << 22;
     self
  }

}
impl ::core::fmt::Display for Cpacr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cpacr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cp10() != 0 { try!(write!(f, " cp10=0x{:x}", self.cp10()))}
      if self.cp11() != 0 { try!(write!(f, " cp11=0x{:x}", self.cp11()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Floating-point Context Control Register"]
#[derive(PartialEq, Eq)]
pub struct Fpccr(pub u32);
impl Fpccr {
  #[inline] pub fn aspen(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  #[inline] pub fn set_aspen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

  #[inline] pub fn lspen(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  #[inline] pub fn set_lspen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  #[inline] pub fn monrdy(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_monrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn bfrdy(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_bfrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn mmrdy(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_mmrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn hfrdy(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_hfrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn thread(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_thread(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn user(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_user(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn lspact(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_lspact(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Fpccr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fpccr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.aspen() != 0 { try!(write!(f, " aspen"))}
      if self.lspen() != 0 { try!(write!(f, " lspen"))}
      if self.monrdy() != 0 { try!(write!(f, " monrdy"))}
      if self.bfrdy() != 0 { try!(write!(f, " bfrdy"))}
      if self.mmrdy() != 0 { try!(write!(f, " mmrdy"))}
      if self.hfrdy() != 0 { try!(write!(f, " hfrdy"))}
      if self.thread() != 0 { try!(write!(f, " thread"))}
      if self.user() != 0 { try!(write!(f, " user"))}
      if self.lspact() != 0 { try!(write!(f, " lspact"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Floating-point Context Address Register"]
#[derive(PartialEq, Eq)]
pub struct Fpcar(pub u32);
impl Fpcar {
  #[inline] pub fn address(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1fffffff // [31:3]
  }
  #[inline] pub fn set_address(mut self, value: u32) -> Self {
     assert!((value & !0x1fffffff) == 0);
     self.0 &= !(0x1fffffff << 3);
     self.0 |= value << 3;
     self
  }

}
impl ::core::fmt::Display for Fpcar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fpcar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.address() != 0 { try!(write!(f, " address=0x{:x}", self.address()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Floating-point Default Status Control Register"]
#[derive(PartialEq, Eq)]
pub struct Fpdscr(pub u32);
impl Fpdscr {
  #[inline] pub fn ahp(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  #[inline] pub fn set_ahp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  #[inline] pub fn dn(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  #[inline] pub fn set_dn(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  #[inline] pub fn fz(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  #[inline] pub fn set_fz(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  #[inline] pub fn rmode(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x3 // [23:22]
  }
  #[inline] pub fn set_rmode(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 22);
     self.0 |= value << 22;
     self
  }

}
impl ::core::fmt::Display for Fpdscr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fpdscr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ahp() != 0 { try!(write!(f, " ahp"))}
      if self.dn() != 0 { try!(write!(f, " dn"))}
      if self.fz() != 0 { try!(write!(f, " fz"))}
      if self.rmode() != 0 { try!(write!(f, " rmode=0x{:x}", self.rmode()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

