//! Common ADC registers
#[allow(unused_imports)] use bobbin_common::*;

periph!(CAdcPeriph, C_ADC, CAdc, 0x40012300);

#[doc="Common ADC registers"]
pub trait CAdcPeriph : Base {
#[doc="Get the *const pointer for the CSR register."]
   #[inline] fn csr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Get the *mut pointer for the CSR register."]
   #[inline] fn csr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Read the CSR register."]
   #[inline] fn csr(&self) -> Csr { 
      unsafe {
         Csr(::core::ptr::read_volatile((self.base() + 0x0) as *const u32))
      }
   }

#[doc="Get the *const pointer for the CCR register."]
   #[inline] fn ccr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Get the *mut pointer for the CCR register."]
   #[inline] fn ccr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Read the CCR register."]
   #[inline] fn ccr(&self) -> Ccr { 
      unsafe {
         Ccr(::core::ptr::read_volatile((self.base() + 0x4) as *const u32))
      }
   }
#[doc="Write the CCR register."]
   #[inline] fn set_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
      let value = f(Ccr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CCR register."]
   #[inline] fn with_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ccr(::core::ptr::read_volatile((self.base() + 0x4) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CDR register."]
   #[inline] fn cdr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Get the *mut pointer for the CDR register."]
   #[inline] fn cdr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Read the CDR register."]
   #[inline] fn cdr(&self) -> Cdr { 
      unsafe {
         Cdr(::core::ptr::read_volatile((self.base() + 0x8) as *const u32))
      }
   }

}

#[doc="ADC Common status register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Csr(pub u32);
impl Csr {
#[doc="Overrun flag of ADC3"]
   #[inline] pub fn ovr3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
   }
#[doc="Overrun flag of ADC3"]
   #[inline] pub fn set_ovr3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 21);
      self.0 |= value << 21;
      self
   }

#[doc="Regular channel Start flag of ADC 3"]
   #[inline] pub fn strt3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
   }
#[doc="Regular channel Start flag of ADC 3"]
   #[inline] pub fn set_strt3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 20);
      self.0 |= value << 20;
      self
   }

#[doc="Injected channel Start flag of ADC 3"]
   #[inline] pub fn jstrt3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
   }
#[doc="Injected channel Start flag of ADC 3"]
   #[inline] pub fn set_jstrt3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 19);
      self.0 |= value << 19;
      self
   }

#[doc="Injected channel end of conversion of ADC 3"]
   #[inline] pub fn jeoc3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
   }
#[doc="Injected channel end of conversion of ADC 3"]
   #[inline] pub fn set_jeoc3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 18);
      self.0 |= value << 18;
      self
   }

#[doc="End of conversion of ADC 3"]
   #[inline] pub fn eoc3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="End of conversion of ADC 3"]
   #[inline] pub fn set_eoc3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

#[doc="Analog watchdog flag of ADC 3"]
   #[inline] pub fn awd3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="Analog watchdog flag of ADC 3"]
   #[inline] pub fn set_awd3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Overrun flag of ADC 2"]
   #[inline] pub fn ovr2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="Overrun flag of ADC 2"]
   #[inline] pub fn set_ovr2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

#[doc="Regular channel Start flag of ADC 2"]
   #[inline] pub fn strt2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="Regular channel Start flag of ADC 2"]
   #[inline] pub fn set_strt2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Injected channel Start flag of ADC 2"]
   #[inline] pub fn jstrt2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="Injected channel Start flag of ADC 2"]
   #[inline] pub fn set_jstrt2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="Injected channel end of conversion of ADC 2"]
   #[inline] pub fn jeoc2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="Injected channel end of conversion of ADC 2"]
   #[inline] pub fn set_jeoc2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="End of conversion of ADC 2"]
   #[inline] pub fn eoc2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="End of conversion of ADC 2"]
   #[inline] pub fn set_eoc2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="Analog watchdog flag of ADC 2"]
   #[inline] pub fn awd2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Analog watchdog flag of ADC 2"]
   #[inline] pub fn set_awd2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Overrun flag of ADC 1"]
   #[inline] pub fn ovr1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Overrun flag of ADC 1"]
   #[inline] pub fn set_ovr1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Regular channel Start flag of ADC 1"]
   #[inline] pub fn strt1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Regular channel Start flag of ADC 1"]
   #[inline] pub fn set_strt1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Injected channel Start flag of ADC 1"]
   #[inline] pub fn jstrt1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Injected channel Start flag of ADC 1"]
   #[inline] pub fn set_jstrt1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Injected channel end of conversion of ADC 1"]
   #[inline] pub fn jeoc1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Injected channel end of conversion of ADC 1"]
   #[inline] pub fn set_jeoc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="End of conversion of ADC 1"]
   #[inline] pub fn eoc1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="End of conversion of ADC 1"]
   #[inline] pub fn set_eoc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Analog watchdog flag of ADC 1"]
   #[inline] pub fn awd1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Analog watchdog flag of ADC 1"]
   #[inline] pub fn set_awd1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
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
      if self.ovr3() != 0 { try!(write!(f, " ovr3"))}
      if self.strt3() != 0 { try!(write!(f, " strt3"))}
      if self.jstrt3() != 0 { try!(write!(f, " jstrt3"))}
      if self.jeoc3() != 0 { try!(write!(f, " jeoc3"))}
      if self.eoc3() != 0 { try!(write!(f, " eoc3"))}
      if self.awd3() != 0 { try!(write!(f, " awd3"))}
      if self.ovr2() != 0 { try!(write!(f, " ovr2"))}
      if self.strt2() != 0 { try!(write!(f, " strt2"))}
      if self.jstrt2() != 0 { try!(write!(f, " jstrt2"))}
      if self.jeoc2() != 0 { try!(write!(f, " jeoc2"))}
      if self.eoc2() != 0 { try!(write!(f, " eoc2"))}
      if self.awd2() != 0 { try!(write!(f, " awd2"))}
      if self.ovr1() != 0 { try!(write!(f, " ovr1"))}
      if self.strt1() != 0 { try!(write!(f, " strt1"))}
      if self.jstrt1() != 0 { try!(write!(f, " jstrt1"))}
      if self.jeoc1() != 0 { try!(write!(f, " jeoc1"))}
      if self.eoc1() != 0 { try!(write!(f, " eoc1"))}
      if self.awd1() != 0 { try!(write!(f, " awd1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC common control register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ccr(pub u32);
impl Ccr {
#[doc="Temperature sensor and VREFINT enable"]
   #[inline] pub fn tsvrefe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
   }
#[doc="Temperature sensor and VREFINT enable"]
   #[inline] pub fn set_tsvrefe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 23);
      self.0 |= value << 23;
      self
   }

#[doc="VBAT enable"]
   #[inline] pub fn vbate(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
   }
#[doc="VBAT enable"]
   #[inline] pub fn set_vbate<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 22);
      self.0 |= value << 22;
      self
   }

#[doc="ADC prescaler"]
   #[inline] pub fn adcpre(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
   }
#[doc="ADC prescaler"]
   #[inline] pub fn set_adcpre<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Direct memory access mode for multi ADC mode"]
   #[inline] pub fn dma(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
   }
#[doc="Direct memory access mode for multi ADC mode"]
   #[inline] pub fn set_dma<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="DMA disable selection for multi-ADC mode"]
   #[inline] pub fn dds(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="DMA disable selection for multi-ADC mode"]
   #[inline] pub fn set_dds<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

#[doc="Delay between 2 sampling phases"]
   #[inline] pub fn delay(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
   }
#[doc="Delay between 2 sampling phases"]
   #[inline] pub fn set_delay<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Multi ADC mode selection"]
   #[inline] pub fn mult(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
   }
#[doc="Multi ADC mode selection"]
   #[inline] pub fn set_mult<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 0);
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
      if self.tsvrefe() != 0 { try!(write!(f, " tsvrefe"))}
      if self.vbate() != 0 { try!(write!(f, " vbate"))}
      if self.adcpre() != 0 { try!(write!(f, " adcpre=0x{:x}", self.adcpre()))}
      if self.dma() != 0 { try!(write!(f, " dma=0x{:x}", self.dma()))}
      if self.dds() != 0 { try!(write!(f, " dds"))}
      if self.delay() != 0 { try!(write!(f, " delay=0x{:x}", self.delay()))}
      if self.mult() != 0 { try!(write!(f, " mult=0x{:x}", self.mult()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC common regular data register for dual and triple modes"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cdr(pub u32);
impl Cdr {
#[doc="2nd data item of a pair of regular conversions"]
   #[inline] pub fn data2(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
   }
#[doc="2nd data item of a pair of regular conversions"]
   #[inline] pub fn set_data2<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 16);
      self.0 |= value << 16;
      self
   }

#[doc="1st data item of a pair of regular conversions"]
   #[inline] pub fn data1(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="1st data item of a pair of regular conversions"]
   #[inline] pub fn set_data1<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
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
      if self.data2() != 0 { try!(write!(f, " data2=0x{:x}", self.data2()))}
      if self.data1() != 0 { try!(write!(f, " data1=0x{:x}", self.data1()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

