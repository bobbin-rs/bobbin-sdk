#[allow(unused_imports)] use bobbin_common::bits;
pub const C_ADC12: CAdc12 = Periph(0x50000300, CAdc12Id {});
pub const C_ADC34: CAdc34 = Periph(0x50000700, CAdc34Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="C_ADC Peripheral"]
pub struct Periph<T>(pub u32, pub T); 

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct CAdc12Id {}
pub type CAdc12 = Periph<CAdc12Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct CAdc34Id {}
pub type CAdc34 = Periph<CAdc34Id>;




impl<T> Periph<T> {
#[doc="Get the *const pointer for the CSR register."]
  #[inline] pub fn csr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the CSR register."]
  #[inline] pub fn csr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the CSR register."]
  #[inline] pub fn csr(&self) -> Csr { 
     unsafe {
        Csr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }

#[doc="Get the *const pointer for the CCR register."]
  #[inline] pub fn ccr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the CCR register."]
  #[inline] pub fn ccr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the CCR register."]
  #[inline] pub fn ccr(&self) -> Ccr { 
     unsafe {
        Ccr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the CCR register."]
  #[inline] pub fn set_ccr(&self, value: Ccr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CCR register."]
  #[inline] pub fn with_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
     let tmp = self.ccr();
     self.set_ccr(f(tmp))
  }

#[doc="Get the *const pointer for the CDR register."]
  #[inline] pub fn cdr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the CDR register."]
  #[inline] pub fn cdr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the CDR register."]
  #[inline] pub fn cdr(&self) -> Cdr { 
     unsafe {
        Cdr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }

}

#[doc="ADC Common status register"]
#[derive(PartialEq, Eq)]
pub struct Csr(pub u32);
impl Csr {
#[doc="ADDRDY_MST"]
  #[inline] pub fn addrdy_mst(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="ADDRDY_MST"]
  #[inline] pub fn set_addrdy_mst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="EOSMP_MST"]
  #[inline] pub fn eosmp_mst(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="EOSMP_MST"]
  #[inline] pub fn set_eosmp_mst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="EOC_MST"]
  #[inline] pub fn eoc_mst(&self) -> bits::B1 {
     (((self.0 as u32) >> 2) & 0x1).into() // [2]
  }
#[doc="EOC_MST"]
  #[inline] pub fn set_eoc_mst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="EOS_MST"]
  #[inline] pub fn eos_mst(&self) -> bits::B1 {
     (((self.0 as u32) >> 3) & 0x1).into() // [3]
  }
#[doc="EOS_MST"]
  #[inline] pub fn set_eos_mst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="OVR_MST"]
  #[inline] pub fn ovr_mst(&self) -> bits::B1 {
     (((self.0 as u32) >> 4) & 0x1).into() // [4]
  }
#[doc="OVR_MST"]
  #[inline] pub fn set_ovr_mst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="JEOC_MST"]
  #[inline] pub fn jeoc_mst(&self) -> bits::B1 {
     (((self.0 as u32) >> 5) & 0x1).into() // [5]
  }
#[doc="JEOC_MST"]
  #[inline] pub fn set_jeoc_mst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="JEOS_MST"]
  #[inline] pub fn jeos_mst(&self) -> bits::B1 {
     (((self.0 as u32) >> 6) & 0x1).into() // [6]
  }
#[doc="JEOS_MST"]
  #[inline] pub fn set_jeos_mst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="AWD1_MST"]
  #[inline] pub fn awd1_mst(&self) -> bits::B1 {
     (((self.0 as u32) >> 7) & 0x1).into() // [7]
  }
#[doc="AWD1_MST"]
  #[inline] pub fn set_awd1_mst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="AWD2_MST"]
  #[inline] pub fn awd2_mst(&self) -> bits::B1 {
     (((self.0 as u32) >> 8) & 0x1).into() // [8]
  }
#[doc="AWD2_MST"]
  #[inline] pub fn set_awd2_mst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="AWD3_MST"]
  #[inline] pub fn awd3_mst(&self) -> bits::B1 {
     (((self.0 as u32) >> 9) & 0x1).into() // [9]
  }
#[doc="AWD3_MST"]
  #[inline] pub fn set_awd3_mst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="JQOVF_MST"]
  #[inline] pub fn jqovf_mst(&self) -> bits::B1 {
     (((self.0 as u32) >> 10) & 0x1).into() // [10]
  }
#[doc="JQOVF_MST"]
  #[inline] pub fn set_jqovf_mst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="ADRDY_SLV"]
  #[inline] pub fn adrdy_slv(&self) -> bits::B1 {
     (((self.0 as u32) >> 16) & 0x1).into() // [16]
  }
#[doc="ADRDY_SLV"]
  #[inline] pub fn set_adrdy_slv<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="EOSMP_SLV"]
  #[inline] pub fn eosmp_slv(&self) -> bits::B1 {
     (((self.0 as u32) >> 17) & 0x1).into() // [17]
  }
#[doc="EOSMP_SLV"]
  #[inline] pub fn set_eosmp_slv<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="End of regular conversion of the slave ADC"]
  #[inline] pub fn eoc_slv(&self) -> bits::B1 {
     (((self.0 as u32) >> 18) & 0x1).into() // [18]
  }
#[doc="End of regular conversion of the slave ADC"]
  #[inline] pub fn set_eoc_slv<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="End of regular sequence flag of the slave ADC"]
  #[inline] pub fn eos_slv(&self) -> bits::B1 {
     (((self.0 as u32) >> 19) & 0x1).into() // [19]
  }
#[doc="End of regular sequence flag of the slave ADC"]
  #[inline] pub fn set_eos_slv<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="Overrun flag of the slave ADC"]
  #[inline] pub fn ovr_slv(&self) -> bits::B1 {
     (((self.0 as u32) >> 20) & 0x1).into() // [20]
  }
#[doc="Overrun flag of the slave ADC"]
  #[inline] pub fn set_ovr_slv<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="End of injected conversion flag of the slave ADC"]
  #[inline] pub fn jeoc_slv(&self) -> bits::B1 {
     (((self.0 as u32) >> 21) & 0x1).into() // [21]
  }
#[doc="End of injected conversion flag of the slave ADC"]
  #[inline] pub fn set_jeoc_slv<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="End of injected sequence flag of the slave ADC"]
  #[inline] pub fn jeos_slv(&self) -> bits::B1 {
     (((self.0 as u32) >> 22) & 0x1).into() // [22]
  }
#[doc="End of injected sequence flag of the slave ADC"]
  #[inline] pub fn set_jeos_slv<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Analog watchdog 1 flag of the slave ADC"]
  #[inline] pub fn awd1_slv(&self) -> bits::B1 {
     (((self.0 as u32) >> 23) & 0x1).into() // [23]
  }
#[doc="Analog watchdog 1 flag of the slave ADC"]
  #[inline] pub fn set_awd1_slv<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="Analog watchdog 2 flag of the slave ADC"]
  #[inline] pub fn awd2_slv(&self) -> bits::B1 {
     (((self.0 as u32) >> 24) & 0x1).into() // [24]
  }
#[doc="Analog watchdog 2 flag of the slave ADC"]
  #[inline] pub fn set_awd2_slv<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Analog watchdog 3 flag of the slave ADC"]
  #[inline] pub fn awd3_slv(&self) -> bits::B1 {
     (((self.0 as u32) >> 25) & 0x1).into() // [25]
  }
#[doc="Analog watchdog 3 flag of the slave ADC"]
  #[inline] pub fn set_awd3_slv<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="Injected Context Queue Overflow flag of the slave ADC"]
  #[inline] pub fn jqovf_slv(&self) -> bits::B1 {
     (((self.0 as u32) >> 26) & 0x1).into() // [26]
  }
#[doc="Injected Context Queue Overflow flag of the slave ADC"]
  #[inline] pub fn set_jqovf_slv<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

}
impl ::core::fmt::Display for Csr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Csr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.addrdy_mst() != 0 { try!(write!(f, " addrdy_mst"))}
      if self.eosmp_mst() != 0 { try!(write!(f, " eosmp_mst"))}
      if self.eoc_mst() != 0 { try!(write!(f, " eoc_mst"))}
      if self.eos_mst() != 0 { try!(write!(f, " eos_mst"))}
      if self.ovr_mst() != 0 { try!(write!(f, " ovr_mst"))}
      if self.jeoc_mst() != 0 { try!(write!(f, " jeoc_mst"))}
      if self.jeos_mst() != 0 { try!(write!(f, " jeos_mst"))}
      if self.awd1_mst() != 0 { try!(write!(f, " awd1_mst"))}
      if self.awd2_mst() != 0 { try!(write!(f, " awd2_mst"))}
      if self.awd3_mst() != 0 { try!(write!(f, " awd3_mst"))}
      if self.jqovf_mst() != 0 { try!(write!(f, " jqovf_mst"))}
      if self.adrdy_slv() != 0 { try!(write!(f, " adrdy_slv"))}
      if self.eosmp_slv() != 0 { try!(write!(f, " eosmp_slv"))}
      if self.eoc_slv() != 0 { try!(write!(f, " eoc_slv"))}
      if self.eos_slv() != 0 { try!(write!(f, " eos_slv"))}
      if self.ovr_slv() != 0 { try!(write!(f, " ovr_slv"))}
      if self.jeoc_slv() != 0 { try!(write!(f, " jeoc_slv"))}
      if self.jeos_slv() != 0 { try!(write!(f, " jeos_slv"))}
      if self.awd1_slv() != 0 { try!(write!(f, " awd1_slv"))}
      if self.awd2_slv() != 0 { try!(write!(f, " awd2_slv"))}
      if self.awd3_slv() != 0 { try!(write!(f, " awd3_slv"))}
      if self.jqovf_slv() != 0 { try!(write!(f, " jqovf_slv"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC common control register"]
#[derive(PartialEq, Eq)]
pub struct Ccr(pub u32);
impl Ccr {
#[doc="Multi ADC mode selection"]
  #[inline] pub fn mult(&self) -> bits::B5 {
     (((self.0 as u32) >> 0) & 0x1f).into() // [4:0]
  }
#[doc="Multi ADC mode selection"]
  #[inline] pub fn set_mult<V: Into<bits::B5>>(mut self, value: V) -> Self {
     let value: bits::B5 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Delay between 2 sampling phases"]
  #[inline] pub fn delay(&self) -> bits::B4 {
     (((self.0 as u32) >> 8) & 0xf).into() // [11:8]
  }
#[doc="Delay between 2 sampling phases"]
  #[inline] pub fn set_delay<V: Into<bits::B4>>(mut self, value: V) -> Self {
     let value: bits::B4 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="DMA configuration (for multi-ADC mode)"]
  #[inline] pub fn dmacfg(&self) -> bits::B1 {
     (((self.0 as u32) >> 13) & 0x1).into() // [13]
  }
#[doc="DMA configuration (for multi-ADC mode)"]
  #[inline] pub fn set_dmacfg<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Direct memory access mode for multi ADC mode"]
  #[inline] pub fn mdma(&self) -> bits::B2 {
     (((self.0 as u32) >> 14) & 0x3).into() // [15:14]
  }
#[doc="Direct memory access mode for multi ADC mode"]
  #[inline] pub fn set_mdma<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="ADC clock mode"]
  #[inline] pub fn ckmode(&self) -> bits::B2 {
     (((self.0 as u32) >> 16) & 0x3).into() // [17:16]
  }
#[doc="ADC clock mode"]
  #[inline] pub fn set_ckmode<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="VREFINT enable"]
  #[inline] pub fn vrefen(&self) -> bits::B1 {
     (((self.0 as u32) >> 22) & 0x1).into() // [22]
  }
#[doc="VREFINT enable"]
  #[inline] pub fn set_vrefen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Temperature sensor enable"]
  #[inline] pub fn tsen(&self) -> bits::B1 {
     (((self.0 as u32) >> 23) & 0x1).into() // [23]
  }
#[doc="Temperature sensor enable"]
  #[inline] pub fn set_tsen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="VBAT enable"]
  #[inline] pub fn vbaten(&self) -> bits::B1 {
     (((self.0 as u32) >> 24) & 0x1).into() // [24]
  }
#[doc="VBAT enable"]
  #[inline] pub fn set_vbaten<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
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
      if self.mult() != 0 { try!(write!(f, " mult=0x{:x}", self.mult()))}
      if self.delay() != 0 { try!(write!(f, " delay=0x{:x}", self.delay()))}
      if self.dmacfg() != 0 { try!(write!(f, " dmacfg"))}
      if self.mdma() != 0 { try!(write!(f, " mdma=0x{:x}", self.mdma()))}
      if self.ckmode() != 0 { try!(write!(f, " ckmode=0x{:x}", self.ckmode()))}
      if self.vrefen() != 0 { try!(write!(f, " vrefen"))}
      if self.tsen() != 0 { try!(write!(f, " tsen"))}
      if self.vbaten() != 0 { try!(write!(f, " vbaten"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC common regular data register for dual and triple modes"]
#[derive(PartialEq, Eq)]
pub struct Cdr(pub u32);
impl Cdr {
#[doc="Regular data of the slave ADC"]
  #[inline] pub fn rdata_slv(&self) -> bits::B16 {
     (((self.0 as u32) >> 16) & 0xffff).into() // [31:16]
  }
#[doc="Regular data of the slave ADC"]
  #[inline] pub fn set_rdata_slv<V: Into<bits::B16>>(mut self, value: V) -> Self {
     let value: bits::B16 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Regular data of the master ADC"]
  #[inline] pub fn rdata_mst(&self) -> bits::B16 {
     (((self.0 as u32) >> 0) & 0xffff).into() // [15:0]
  }
#[doc="Regular data of the master ADC"]
  #[inline] pub fn set_rdata_mst<V: Into<bits::B16>>(mut self, value: V) -> Self {
     let value: bits::B16 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rdata_slv() != 0 { try!(write!(f, " rdata_slv=0x{:x}", self.rdata_slv()))}
      if self.rdata_mst() != 0 { try!(write!(f, " rdata_mst=0x{:x}", self.rdata_mst()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
