#[allow(unused_imports)] use bobbin_common::bits;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="TPM Peripheral"]
pub struct Periph<T>(pub u32, pub T); 



impl<T> Periph<T> {
#[doc="Get the *const pointer for the SC register."]
  #[inline] pub fn sc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the SC register."]
  #[inline] pub fn sc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the SC register."]
  #[inline] pub fn sc(&self) -> Sc { 
     unsafe {
        Sc(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the SC register."]
  #[inline] pub fn set_sc(&self, value: Sc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SC register."]
  #[inline] pub fn with_sc<F: FnOnce(Sc) -> Sc>(&self, f: F) -> &Self {
     let tmp = self.sc();
     self.set_sc(f(tmp))
  }

#[doc="Get the *const pointer for the CNT register."]
  #[inline] pub fn cnt_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the CNT register."]
  #[inline] pub fn cnt_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the CNT register."]
  #[inline] pub fn cnt(&self) -> Cnt { 
     unsafe {
        Cnt(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the CNT register."]
  #[inline] pub fn set_cnt(&self, value: Cnt) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CNT register."]
  #[inline] pub fn with_cnt<F: FnOnce(Cnt) -> Cnt>(&self, f: F) -> &Self {
     let tmp = self.cnt();
     self.set_cnt(f(tmp))
  }

#[doc="Get the *const pointer for the MOD register."]
  #[inline] pub fn mod_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the MOD register."]
  #[inline] pub fn mod_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the MOD register."]
  #[inline] pub fn _mod(&self) -> Mod { 
     unsafe {
        Mod(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the MOD register."]
  #[inline] pub fn set_mod(&self, value: Mod) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MOD register."]
  #[inline] pub fn with_mod<F: FnOnce(Mod) -> Mod>(&self, f: F) -> &Self {
     let tmp = self._mod();
     self.set_mod(f(tmp))
  }

#[doc="Get the *const pointer for the CSC register."]
  #[inline] pub fn csc_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 6);
     ((self.0 as usize) + 0xc + (index << 3)) as *const u32
  }
#[doc="Get the *mut pointer for the CSC register."]
  #[inline] pub fn csc_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 6);
     ((self.0 as usize) + 0xc + (index << 3)) as *mut u32
  }
#[doc="Read the CSC register."]
  #[inline] pub fn csc(&self, index: usize) -> Csc { 
     assert!(index < 6);
     unsafe {
        Csc(::core::ptr::read_volatile(((self.0 as usize) + 0xc + (index << 3)) as *const u32))
     }
  }
#[doc="Write the CSC register."]
  #[inline] pub fn set_csc(&self, index: usize, value: Csc) -> &Self {
     assert!(index < 6);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc + (index << 3)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CSC register."]
  #[inline] pub fn with_csc<F: FnOnce(Csc) -> Csc>(&self, index: usize, f: F) -> &Self {
     let tmp = self.csc(index);
     self.set_csc(index, f(tmp))
  }

#[doc="Get the *const pointer for the CV register."]
  #[inline] pub fn cv_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 6);
     ((self.0 as usize) + 0x10 + (index << 3)) as *const u32
  }
#[doc="Get the *mut pointer for the CV register."]
  #[inline] pub fn cv_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 6);
     ((self.0 as usize) + 0x10 + (index << 3)) as *mut u32
  }
#[doc="Read the CV register."]
  #[inline] pub fn cv(&self, index: usize) -> Cv { 
     assert!(index < 6);
     unsafe {
        Cv(::core::ptr::read_volatile(((self.0 as usize) + 0x10 + (index << 3)) as *const u32))
     }
  }
#[doc="Write the CV register."]
  #[inline] pub fn set_cv(&self, index: usize, value: Cv) -> &Self {
     assert!(index < 6);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10 + (index << 3)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CV register."]
  #[inline] pub fn with_cv<F: FnOnce(Cv) -> Cv>(&self, index: usize, f: F) -> &Self {
     let tmp = self.cv(index);
     self.set_cv(index, f(tmp))
  }

#[doc="Get the *const pointer for the STATUS register."]
  #[inline] pub fn status_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x50) as *const u32
  }
#[doc="Get the *mut pointer for the STATUS register."]
  #[inline] pub fn status_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x50) as *mut u32
  }
#[doc="Read the STATUS register."]
  #[inline] pub fn status(&self) -> Status { 
     unsafe {
        Status(::core::ptr::read_volatile(((self.0 as usize) + 0x50) as *const u32))
     }
  }
#[doc="Write the STATUS register."]
  #[inline] pub fn set_status(&self, value: Status) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x50) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the STATUS register."]
  #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
     let tmp = self.status();
     self.set_status(f(tmp))
  }

#[doc="Get the *const pointer for the CONF register."]
  #[inline] pub fn conf_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x84) as *const u32
  }
#[doc="Get the *mut pointer for the CONF register."]
  #[inline] pub fn conf_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x84) as *mut u32
  }
#[doc="Read the CONF register."]
  #[inline] pub fn conf(&self) -> Conf { 
     unsafe {
        Conf(::core::ptr::read_volatile(((self.0 as usize) + 0x84) as *const u32))
     }
  }
#[doc="Write the CONF register."]
  #[inline] pub fn set_conf(&self, value: Conf) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x84) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CONF register."]
  #[inline] pub fn with_conf<F: FnOnce(Conf) -> Conf>(&self, f: F) -> &Self {
     let tmp = self.conf();
     self.set_conf(f(tmp))
  }

}

#[doc="Status and Control"]
#[derive(PartialEq, Eq)]
pub struct Sc(pub u32);
impl Sc {
#[doc="Prescale Factor Selection"]
  #[inline] pub fn ps(&self) -> bits::B3 {
     (((self.0 as u32) >> 0) & 0x7).into() // [2:0]
  }
#[doc="Prescale Factor Selection"]
  #[inline] pub fn set_ps<V: Into<bits::B3>>(mut self, value: V) -> Self {
     let value: bits::B3 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Clock Mode Selection"]
  #[inline] pub fn cmod(&self) -> bits::B2 {
     (((self.0 as u32) >> 3) & 0x3).into() // [4:3]
  }
#[doc="Clock Mode Selection"]
  #[inline] pub fn set_cmod<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Center-Aligned PWM Select"]
  #[inline] pub fn cpwms(&self) -> bits::B1 {
     (((self.0 as u32) >> 5) & 0x1).into() // [5]
  }
#[doc="Center-Aligned PWM Select"]
  #[inline] pub fn set_cpwms<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Timer Overflow Interrupt Enable"]
  #[inline] pub fn toie(&self) -> bits::B1 {
     (((self.0 as u32) >> 6) & 0x1).into() // [6]
  }
#[doc="Timer Overflow Interrupt Enable"]
  #[inline] pub fn set_toie<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Timer Overflow Flag"]
  #[inline] pub fn tof(&self) -> bits::B1 {
     (((self.0 as u32) >> 7) & 0x1).into() // [7]
  }
#[doc="Timer Overflow Flag"]
  #[inline] pub fn set_tof<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="DMA Enable"]
  #[inline] pub fn dma(&self) -> bits::B1 {
     (((self.0 as u32) >> 8) & 0x1).into() // [8]
  }
#[doc="DMA Enable"]
  #[inline] pub fn set_dma<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

}
impl ::core::fmt::Display for Sc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ps() != 0 { try!(write!(f, " ps=0x{:x}", self.ps()))}
      if self.cmod() != 0 { try!(write!(f, " cmod=0x{:x}", self.cmod()))}
      if self.cpwms() != 0 { try!(write!(f, " cpwms"))}
      if self.toie() != 0 { try!(write!(f, " toie"))}
      if self.tof() != 0 { try!(write!(f, " tof"))}
      if self.dma() != 0 { try!(write!(f, " dma"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Counter"]
#[derive(PartialEq, Eq)]
pub struct Cnt(pub u32);
impl Cnt {
#[doc="Counter value"]
  #[inline] pub fn count(&self) -> bits::B16 {
     (((self.0 as u32) >> 0) & 0xffff).into() // [15:0]
  }
#[doc="Counter value"]
  #[inline] pub fn set_count<V: Into<bits::B16>>(mut self, value: V) -> Self {
     let value: bits::B16 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cnt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cnt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Modulo"]
#[derive(PartialEq, Eq)]
pub struct Mod(pub u32);
impl Mod {
#[doc="Modulo value"]
  #[inline] pub fn _mod(&self) -> bits::B16 {
     (((self.0 as u32) >> 0) & 0xffff).into() // [15:0]
  }
#[doc="Modulo value"]
  #[inline] pub fn set_mod<V: Into<bits::B16>>(mut self, value: V) -> Self {
     let value: bits::B16 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Mod {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Mod {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self._mod() != 0 { try!(write!(f, " mod=0x{:x}", self._mod()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Channel (n) Status and Control"]
#[derive(PartialEq, Eq)]
pub struct Csc(pub u32);
impl Csc {
#[doc="DMA Enable"]
  #[inline] pub fn dma(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="DMA Enable"]
  #[inline] pub fn set_dma<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Edge or Level Select"]
  #[inline] pub fn elsa(&self) -> bits::B1 {
     (((self.0 as u32) >> 2) & 0x1).into() // [2]
  }
#[doc="Edge or Level Select"]
  #[inline] pub fn set_elsa<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Edge or Level Select"]
  #[inline] pub fn elsb(&self) -> bits::B1 {
     (((self.0 as u32) >> 3) & 0x1).into() // [3]
  }
#[doc="Edge or Level Select"]
  #[inline] pub fn set_elsb<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Channel Mode Select"]
  #[inline] pub fn msa(&self) -> bits::B1 {
     (((self.0 as u32) >> 4) & 0x1).into() // [4]
  }
#[doc="Channel Mode Select"]
  #[inline] pub fn set_msa<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Channel Mode Select"]
  #[inline] pub fn msb(&self) -> bits::B1 {
     (((self.0 as u32) >> 5) & 0x1).into() // [5]
  }
#[doc="Channel Mode Select"]
  #[inline] pub fn set_msb<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Channel Interrupt Enable"]
  #[inline] pub fn chie(&self) -> bits::B1 {
     (((self.0 as u32) >> 6) & 0x1).into() // [6]
  }
#[doc="Channel Interrupt Enable"]
  #[inline] pub fn set_chie<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Channel Flag"]
  #[inline] pub fn chf(&self) -> bits::B1 {
     (((self.0 as u32) >> 7) & 0x1).into() // [7]
  }
#[doc="Channel Flag"]
  #[inline] pub fn set_chf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Csc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Csc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dma() != 0 { try!(write!(f, " dma"))}
      if self.elsa() != 0 { try!(write!(f, " elsa"))}
      if self.elsb() != 0 { try!(write!(f, " elsb"))}
      if self.msa() != 0 { try!(write!(f, " msa"))}
      if self.msb() != 0 { try!(write!(f, " msb"))}
      if self.chie() != 0 { try!(write!(f, " chie"))}
      if self.chf() != 0 { try!(write!(f, " chf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Channel (n) Value"]
#[derive(PartialEq, Eq)]
pub struct Cv(pub u32);
impl Cv {
#[doc="Channel Value"]
  #[inline] pub fn val(&self) -> bits::B16 {
     (((self.0 as u32) >> 0) & 0xffff).into() // [15:0]
  }
#[doc="Channel Value"]
  #[inline] pub fn set_val<V: Into<bits::B16>>(mut self, value: V) -> Self {
     let value: bits::B16 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cv {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cv {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.val() != 0 { try!(write!(f, " val=0x{:x}", self.val()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Capture and Compare Status"]
#[derive(PartialEq, Eq)]
pub struct Status(pub u32);
impl Status {
#[doc="Channel n Flag"]
  #[inline] pub fn chf(&self, index: usize) -> bits::B1 {
     assert!(index < 6);
     let shift: usize = 0 + index;
     (((self.0 as u32) >> shift) & 0x1).into() // [0]
  }
#[doc="Channel n Flag"]
  #[inline] pub fn set_chf<V: Into<bits::B1>>(mut self, index: usize, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!(index < 6);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.chf(0) != 0 { try!(write!(f, " chf[0]"))}
      if self.chf(1) != 0 { try!(write!(f, " chf[1]"))}
      if self.chf(2) != 0 { try!(write!(f, " chf[2]"))}
      if self.chf(3) != 0 { try!(write!(f, " chf[3]"))}
      if self.chf(4) != 0 { try!(write!(f, " chf[4]"))}
      if self.chf(5) != 0 { try!(write!(f, " chf[5]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Configuration"]
#[derive(PartialEq, Eq)]
pub struct Conf(pub u32);
impl Conf {
#[doc="Doze Enable"]
  #[inline] pub fn dozeen(&self) -> bits::B1 {
     (((self.0 as u32) >> 5) & 0x1).into() // [5]
  }
#[doc="Doze Enable"]
  #[inline] pub fn set_dozeen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Debug Mode"]
  #[inline] pub fn dbgmode(&self) -> bits::B2 {
     (((self.0 as u32) >> 6) & 0x3).into() // [7:6]
  }
#[doc="Debug Mode"]
  #[inline] pub fn set_dbgmode<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Global time base enable"]
  #[inline] pub fn gtbeen(&self) -> bits::B1 {
     (((self.0 as u32) >> 9) & 0x1).into() // [9]
  }
#[doc="Global time base enable"]
  #[inline] pub fn set_gtbeen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Counter Start on Trigger"]
  #[inline] pub fn csot(&self) -> bits::B1 {
     (((self.0 as u32) >> 16) & 0x1).into() // [16]
  }
#[doc="Counter Start on Trigger"]
  #[inline] pub fn set_csot<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Counter Stop On Overflow"]
  #[inline] pub fn csoo(&self) -> bits::B1 {
     (((self.0 as u32) >> 17) & 0x1).into() // [17]
  }
#[doc="Counter Stop On Overflow"]
  #[inline] pub fn set_csoo<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="Counter Reload On Trigger"]
  #[inline] pub fn crot(&self) -> bits::B1 {
     (((self.0 as u32) >> 18) & 0x1).into() // [18]
  }
#[doc="Counter Reload On Trigger"]
  #[inline] pub fn set_crot<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="Trigger Select"]
  #[inline] pub fn trgsel(&self) -> bits::B4 {
     (((self.0 as u32) >> 24) & 0xf).into() // [27:24]
  }
#[doc="Trigger Select"]
  #[inline] pub fn set_trgsel<V: Into<bits::B4>>(mut self, value: V) -> Self {
     let value: bits::B4 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

}
impl ::core::fmt::Display for Conf {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Conf {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dozeen() != 0 { try!(write!(f, " dozeen"))}
      if self.dbgmode() != 0 { try!(write!(f, " dbgmode=0x{:x}", self.dbgmode()))}
      if self.gtbeen() != 0 { try!(write!(f, " gtbeen"))}
      if self.csot() != 0 { try!(write!(f, " csot"))}
      if self.csoo() != 0 { try!(write!(f, " csoo"))}
      if self.crot() != 0 { try!(write!(f, " crot"))}
      if self.trgsel() != 0 { try!(write!(f, " trgsel=0x{:x}", self.trgsel()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(Clone, Copy, PartialEq)]
#[doc="TPM Channel"]
pub struct Channel<P, T> { pub periph: Periph<T>, pub index: usize, pub id: P }

impl<P,T> Channel<P,T> {
   #[inline] pub fn periph(&self) -> &Periph<T> { &self.periph }
   #[inline] pub fn index(&self) -> usize { self.index }
}

