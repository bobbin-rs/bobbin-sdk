#[allow(unused_imports)] use bobbin_common::bits;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="LPIT Peripheral"]
pub struct Periph<T>(pub u32, pub T); 



impl<T> Periph<T> {
#[doc="Get the *const pointer for the VERID register."]
  #[inline] pub fn verid_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the VERID register."]
  #[inline] pub fn verid_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the VERID register."]
  #[inline] pub fn verid(&self) -> Verid { 
     unsafe {
        Verid(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }

#[doc="Get the *const pointer for the PARAM register."]
  #[inline] pub fn param_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the PARAM register."]
  #[inline] pub fn param_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the PARAM register."]
  #[inline] pub fn param(&self) -> Param { 
     unsafe {
        Param(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }

#[doc="Get the *const pointer for the MCR register."]
  #[inline] pub fn mcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the MCR register."]
  #[inline] pub fn mcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the MCR register."]
  #[inline] pub fn mcr(&self) -> Mcr { 
     unsafe {
        Mcr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the MCR register."]
  #[inline] pub fn set_mcr<F: FnOnce(Mcr) -> Mcr>(&self, f: F) -> &Self {
     let value = f(Mcr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MCR register."]
  #[inline] pub fn with_mcr<F: FnOnce(Mcr) -> Mcr>(&self, f: F) -> &Self {
     let tmp = self.mcr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the MSR register."]
  #[inline] pub fn msr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the MSR register."]
  #[inline] pub fn msr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the MSR register."]
  #[inline] pub fn msr(&self) -> Msr { 
     unsafe {
        Msr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the MSR register."]
  #[inline] pub fn set_msr<F: FnOnce(Msr) -> Msr>(&self, f: F) -> &Self {
     let value = f(Msr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MSR register."]
  #[inline] pub fn with_msr<F: FnOnce(Msr) -> Msr>(&self, f: F) -> &Self {
     let tmp = self.msr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the MIER register."]
  #[inline] pub fn mier_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the MIER register."]
  #[inline] pub fn mier_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the MIER register."]
  #[inline] pub fn mier(&self) -> Mier { 
     unsafe {
        Mier(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the MIER register."]
  #[inline] pub fn set_mier<F: FnOnce(Mier) -> Mier>(&self, f: F) -> &Self {
     let value = f(Mier(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MIER register."]
  #[inline] pub fn with_mier<F: FnOnce(Mier) -> Mier>(&self, f: F) -> &Self {
     let tmp = self.mier();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the SETTEN register."]
  #[inline] pub fn setten_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the SETTEN register."]
  #[inline] pub fn setten_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Read the SETTEN register."]
  #[inline] pub fn setten(&self) -> Setten { 
     unsafe {
        Setten(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
#[doc="Write the SETTEN register."]
  #[inline] pub fn set_setten<F: FnOnce(Setten) -> Setten>(&self, f: F) -> &Self {
     let value = f(Setten(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SETTEN register."]
  #[inline] pub fn with_setten<F: FnOnce(Setten) -> Setten>(&self, f: F) -> &Self {
     let tmp = self.setten();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CLRTEN register."]
  #[inline] pub fn clrten_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
#[doc="Get the *mut pointer for the CLRTEN register."]
  #[inline] pub fn clrten_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
#[doc="Read the CLRTEN register."]
  #[inline] pub fn clrten(&self) -> Clrten { 
     unsafe {
        Clrten(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
#[doc="Write the CLRTEN register."]
  #[inline] pub fn set_clrten<F: FnOnce(Clrten) -> Clrten>(&self, f: F) -> &Self {
     let value = f(Clrten(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CLRTEN register."]
  #[inline] pub fn with_clrten<F: FnOnce(Clrten) -> Clrten>(&self, f: F) -> &Self {
     let tmp = self.clrten();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the TVAL register."]
  #[inline] pub fn tval_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     ((self.0 as usize) + 0x20 + (index << 4)) as *const u32
  }
#[doc="Get the *mut pointer for the TVAL register."]
  #[inline] pub fn tval_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     ((self.0 as usize) + 0x20 + (index << 4)) as *mut u32
  }
#[doc="Read the TVAL register."]
  #[inline] pub fn tval<I: Into<bits::R4>>(&self, index: I) -> Tval { 
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     unsafe {
        Tval(::core::ptr::read_volatile(((self.0 as usize) + 0x20 + (index << 4)) as *const u32))
     }
  }
#[doc="Write the TVAL register."]
  #[inline] pub fn set_tval<I: Into<bits::R4>, F: FnOnce(Tval) -> Tval>(&self, index: I, f: F) -> &Self {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let value = f(Tval(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20 + (index << 4)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TVAL register."]
  #[inline] pub fn with_tval<I: Into<bits::R4> + Copy, F: FnOnce(Tval) -> Tval>(&self, index: I, f: F) -> &Self {
     let tmp = self.tval(index);
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CVAL register."]
  #[inline] pub fn cval_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     ((self.0 as usize) + 0x24 + (index << 4)) as *const u32
  }
#[doc="Get the *mut pointer for the CVAL register."]
  #[inline] pub fn cval_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     ((self.0 as usize) + 0x24 + (index << 4)) as *mut u32
  }
#[doc="Read the CVAL register."]
  #[inline] pub fn cval<I: Into<bits::R4>>(&self, index: I) -> Cval { 
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     unsafe {
        Cval(::core::ptr::read_volatile(((self.0 as usize) + 0x24 + (index << 4)) as *const u32))
     }
  }

#[doc="Get the *const pointer for the TCTRL register."]
  #[inline] pub fn tctrl_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     ((self.0 as usize) + 0x28 + (index << 4)) as *const u32
  }
#[doc="Get the *mut pointer for the TCTRL register."]
  #[inline] pub fn tctrl_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     ((self.0 as usize) + 0x28 + (index << 4)) as *mut u32
  }
#[doc="Read the TCTRL register."]
  #[inline] pub fn tctrl<I: Into<bits::R4>>(&self, index: I) -> Tctrl { 
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     unsafe {
        Tctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x28 + (index << 4)) as *const u32))
     }
  }
#[doc="Write the TCTRL register."]
  #[inline] pub fn set_tctrl<I: Into<bits::R4>, F: FnOnce(Tctrl) -> Tctrl>(&self, index: I, f: F) -> &Self {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let value = f(Tctrl(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x28 + (index << 4)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TCTRL register."]
  #[inline] pub fn with_tctrl<I: Into<bits::R4> + Copy, F: FnOnce(Tctrl) -> Tctrl>(&self, index: I, f: F) -> &Self {
     let tmp = self.tctrl(index);
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
     }
     self
  }

}

#[doc="Version ID Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Verid(pub u32);
impl Verid {
#[doc="Feature Number"]
  #[inline] pub fn feature(&self) -> bits::U16 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
  }
#[doc="Feature Number"]
  #[inline] pub fn set_feature<V: Into<bits::U16>>(mut self, value: V) -> Self {
     let value: bits::U16 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Minor Version Number"]
  #[inline] pub fn minor(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
  }
#[doc="Minor Version Number"]
  #[inline] pub fn set_minor<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Major Version Number"]
  #[inline] pub fn major(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
  }
#[doc="Major Version Number"]
  #[inline] pub fn set_major<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

}
impl ::core::fmt::Display for Verid {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Verid {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.feature() != 0 { try!(write!(f, " feature=0x{:x}", self.feature()))}
      if self.minor() != 0 { try!(write!(f, " minor=0x{:x}", self.minor()))}
      if self.major() != 0 { try!(write!(f, " major=0x{:x}", self.major()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Parameter Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Param(pub u32);
impl Param {
#[doc="Number of Timer Channels"]
  #[inline] pub fn channel(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="Number of Timer Channels"]
  #[inline] pub fn set_channel<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Number of External Trigger Inputs"]
  #[inline] pub fn ext_trig(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
  }
#[doc="Number of External Trigger Inputs"]
  #[inline] pub fn set_ext_trig<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

}
impl ::core::fmt::Display for Param {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Param {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.channel() != 0 { try!(write!(f, " channel=0x{:x}", self.channel()))}
      if self.ext_trig() != 0 { try!(write!(f, " ext_trig=0x{:x}", self.ext_trig()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Module Control Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mcr(pub u32);
impl Mcr {
#[doc="Module Clock Enable"]
  #[inline] pub fn m_cen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Module Clock Enable"]
  #[inline] pub fn set_m_cen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Software Reset Bit"]
  #[inline] pub fn sw_rst(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Software Reset Bit"]
  #[inline] pub fn set_sw_rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="DOZE Mode Enable Bit"]
  #[inline] pub fn doze_en(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="DOZE Mode Enable Bit"]
  #[inline] pub fn set_doze_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Debug Enable Bit"]
  #[inline] pub fn dbg_en(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="Debug Enable Bit"]
  #[inline] pub fn set_dbg_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}
impl ::core::fmt::Display for Mcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Mcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.m_cen() != 0 { try!(write!(f, " m_cen"))}
      if self.sw_rst() != 0 { try!(write!(f, " sw_rst"))}
      if self.doze_en() != 0 { try!(write!(f, " doze_en"))}
      if self.dbg_en() != 0 { try!(write!(f, " dbg_en"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Module Status Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Msr(pub u32);
impl Msr {
#[doc="Channel n Timer Interrupt Flag"]
  #[inline] pub fn tif<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Channel n Timer Interrupt Flag"]
  #[inline] pub fn set_tif<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Msr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Msr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tif(0) != 0 { try!(write!(f, " tif[0]"))}
      if self.tif(1) != 0 { try!(write!(f, " tif[1]"))}
      if self.tif(2) != 0 { try!(write!(f, " tif[2]"))}
      if self.tif(3) != 0 { try!(write!(f, " tif[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Module Interrupt Enable Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mier(pub u32);
impl Mier {
#[doc="Channel n Timer Interrupt Enable"]
  #[inline] pub fn tie<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Channel n Timer Interrupt Enable"]
  #[inline] pub fn set_tie<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Mier {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Mier {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tie(0) != 0 { try!(write!(f, " tie[0]"))}
      if self.tie(1) != 0 { try!(write!(f, " tie[1]"))}
      if self.tie(2) != 0 { try!(write!(f, " tie[2]"))}
      if self.tie(3) != 0 { try!(write!(f, " tie[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Set Timer Enable Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Setten(pub u32);
impl Setten {
#[doc="Set Timer n Enable"]
  #[inline] pub fn set_t_en<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Set Timer n Enable"]
  #[inline] pub fn set_set_t_en<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Setten {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Setten {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.set_t_en(0) != 0 { try!(write!(f, " set_t_en[0]"))}
      if self.set_t_en(1) != 0 { try!(write!(f, " set_t_en[1]"))}
      if self.set_t_en(2) != 0 { try!(write!(f, " set_t_en[2]"))}
      if self.set_t_en(3) != 0 { try!(write!(f, " set_t_en[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Clear Timer Enable Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clrten(pub u32);
impl Clrten {
#[doc="Clear Timer n Enable"]
  #[inline] pub fn clr_t_en<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Clear Timer n Enable"]
  #[inline] pub fn set_clr_t_en<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Clrten {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clrten {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clr_t_en(0) != 0 { try!(write!(f, " clr_t_en[0]"))}
      if self.clr_t_en(1) != 0 { try!(write!(f, " clr_t_en[1]"))}
      if self.clr_t_en(2) != 0 { try!(write!(f, " clr_t_en[2]"))}
      if self.clr_t_en(3) != 0 { try!(write!(f, " clr_t_en[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Timer Value Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tval(pub u32);
impl Tval {
#[doc="Timer Value"]
  #[inline] pub fn tmr_val(&self) -> bits::U32 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
  }
#[doc="Timer Value"]
  #[inline] pub fn set_tmr_val<V: Into<bits::U32>>(mut self, value: V) -> Self {
     let value: bits::U32 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tval {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tval {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Current Timer Value"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cval(pub u32);
impl Cval {
#[doc="Current Timer Value"]
  #[inline] pub fn tmr_cur_val(&self) -> bits::U32 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
  }
#[doc="Current Timer Value"]
  #[inline] pub fn set_tmr_cur_val<V: Into<bits::U32>>(mut self, value: V) -> Self {
     let value: bits::U32 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cval {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cval {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Timer Control Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tctrl(pub u32);
impl Tctrl {
#[doc="Timer Enable"]
  #[inline] pub fn t_en(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Timer Enable"]
  #[inline] pub fn set_t_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Chain Channel"]
  #[inline] pub fn chain(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Chain Channel"]
  #[inline] pub fn set_chain<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Timer Operation Mode"]
  #[inline] pub fn mode(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
  }
#[doc="Timer Operation Mode"]
  #[inline] pub fn set_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Timer Start On Trigger"]
  #[inline] pub fn tsot(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
  }
#[doc="Timer Start On Trigger"]
  #[inline] pub fn set_tsot<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Timer Stop On Interrupt"]
  #[inline] pub fn tsoi(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
  }
#[doc="Timer Stop On Interrupt"]
  #[inline] pub fn set_tsoi<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="Timer Reload On Trigger"]
  #[inline] pub fn trot(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
  }
#[doc="Timer Reload On Trigger"]
  #[inline] pub fn set_trot<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="Trigger Source"]
  #[inline] pub fn trg_src(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
  }
#[doc="Trigger Source"]
  #[inline] pub fn set_trg_src<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="Trigger Select"]
  #[inline] pub fn trg_sel(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
  }
#[doc="Trigger Select"]
  #[inline] pub fn set_trg_sel<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

}
impl ::core::fmt::Display for Tctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.t_en() != 0 { try!(write!(f, " t_en"))}
      if self.chain() != 0 { try!(write!(f, " chain"))}
      if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
      if self.tsot() != 0 { try!(write!(f, " tsot"))}
      if self.tsoi() != 0 { try!(write!(f, " tsoi"))}
      if self.trot() != 0 { try!(write!(f, " trot"))}
      if self.trg_src() != 0 { try!(write!(f, " trg_src"))}
      if self.trg_sel() != 0 { try!(write!(f, " trg_sel=0x{:x}", self.trg_sel()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(Clone, Copy, PartialEq)]
#[doc="LPIT Channel"]
pub struct Channel<P, T> { pub periph: Periph<T>, pub index: usize, pub id: P }

impl<P,T> Channel<P,T> {
   #[inline] pub fn periph(&self) -> &Periph<T> { &self.periph }
   #[inline] pub fn index(&self) -> usize { self.index }
}

