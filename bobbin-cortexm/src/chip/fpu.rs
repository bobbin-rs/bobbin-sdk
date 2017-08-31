//! FPU
#[allow(unused_imports)] use bobbin_common::*;

periph!(FPU, Fpu, 0xe000e000);

#[doc="FPU"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fpu(pub usize);
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
         Cpacr(read_volatile((self.0 + 0xd88) as *const u32))
      }
   }
#[doc="Write the CPACR register."]
   #[inline] pub fn set_cpacr<F: FnOnce(Cpacr) -> Cpacr>(&self, f: F) -> &Self {
      let value = f(Cpacr(0));
      unsafe {
         write_volatile((self.0 + 0xd88) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CPACR register."]
   #[inline] pub fn with_cpacr<F: FnOnce(Cpacr) -> Cpacr>(&self, f: F) -> &Self {
      let tmp = self.cpacr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xd88) as *mut u32, value.0);
      }
      self
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
         Fpccr(read_volatile((self.0 + 0xf34) as *const u32))
      }
   }
#[doc="Write the FPCCR register."]
   #[inline] pub fn set_fpccr<F: FnOnce(Fpccr) -> Fpccr>(&self, f: F) -> &Self {
      let value = f(Fpccr(0));
      unsafe {
         write_volatile((self.0 + 0xf34) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the FPCCR register."]
   #[inline] pub fn with_fpccr<F: FnOnce(Fpccr) -> Fpccr>(&self, f: F) -> &Self {
      let tmp = self.fpccr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xf34) as *mut u32, value.0);
      }
      self
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
         Fpcar(read_volatile((self.0 + 0xf38) as *const u32))
      }
   }
#[doc="Write the FPCAR register."]
   #[inline] pub fn set_fpcar<F: FnOnce(Fpcar) -> Fpcar>(&self, f: F) -> &Self {
      let value = f(Fpcar(0));
      unsafe {
         write_volatile((self.0 + 0xf38) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the FPCAR register."]
   #[inline] pub fn with_fpcar<F: FnOnce(Fpcar) -> Fpcar>(&self, f: F) -> &Self {
      let tmp = self.fpcar();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xf38) as *mut u32, value.0);
      }
      self
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
         Fpdscr(read_volatile((self.0 + 0xf3c) as *const u32))
      }
   }
#[doc="Write the FPDSCR register."]
   #[inline] pub fn set_fpdscr<F: FnOnce(Fpdscr) -> Fpdscr>(&self, f: F) -> &Self {
      let value = f(Fpdscr(0));
      unsafe {
         write_volatile((self.0 + 0xf3c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the FPDSCR register."]
   #[inline] pub fn with_fpdscr<F: FnOnce(Fpdscr) -> Fpdscr>(&self, f: F) -> &Self {
      let tmp = self.fpdscr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xf3c) as *mut u32, value.0);
      }
      self
   }

}

#[doc="Coprocessor Access Control Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cpacr(pub u32);
impl Cpacr {
#[doc="CP10 Access privileges"]
   #[inline] pub fn cp10(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
   }
#[doc="CP10 Access privileges"]
   #[inline] pub fn set_cp10<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 20);
      self.0 |= value << 20;
      self
   }

#[doc="CP11 Access privileges"]
   #[inline] pub fn cp11(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
   }
#[doc="CP11 Access privileges"]
   #[inline] pub fn set_cp11<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fpccr(pub u32);
impl Fpccr {
   #[inline] pub fn aspen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
   #[inline] pub fn set_aspen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

   #[inline] pub fn lspen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
   #[inline] pub fn set_lspen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

   #[inline] pub fn monrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
   #[inline] pub fn set_monrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

   #[inline] pub fn bfrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
   #[inline] pub fn set_bfrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

   #[inline] pub fn mmrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
   #[inline] pub fn set_mmrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

   #[inline] pub fn hfrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
   #[inline] pub fn set_hfrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

   #[inline] pub fn thread(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
   #[inline] pub fn set_thread<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

   #[inline] pub fn user(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
   #[inline] pub fn set_user<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[inline] pub fn lspact(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
   #[inline] pub fn set_lspact<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fpcar(pub u32);
impl Fpcar {
   #[inline] pub fn address(&self) -> bits::U29 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1fffffff) as u32) } // [31:3]
   }
   #[inline] pub fn set_address<V: Into<bits::U29>>(mut self, value: V) -> Self {
      let value: bits::U29 = value.into();
      let value: u32 = value.into();
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fpdscr(pub u32);
impl Fpdscr {
   #[inline] pub fn ahp(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
   }
   #[inline] pub fn set_ahp<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 26);
      self.0 |= value << 26;
      self
   }

   #[inline] pub fn dn(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
   }
   #[inline] pub fn set_dn<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 25);
      self.0 |= value << 25;
      self
   }

   #[inline] pub fn fz(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
   }
   #[inline] pub fn set_fz<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 24);
      self.0 |= value << 24;
      self
   }

   #[inline] pub fn rmode(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
   }
   #[inline] pub fn set_rmode<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
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

