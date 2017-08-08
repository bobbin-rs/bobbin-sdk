//! System configuration controller
#[allow(unused_imports)] use bobbin_common::bits;
pub const SYSCFG: Syscfg = Syscfg(0x40010000);

#[doc="System configuration controller"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Syscfg(pub u32);
impl Syscfg {
#[doc="Get the *const pointer for the CFGR1 register."]
  #[inline] pub fn cfgr1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the CFGR1 register."]
  #[inline] pub fn cfgr1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the CFGR1 register."]
  #[inline] pub fn cfgr1(&self) -> Cfgr1 { 
     unsafe {
        Cfgr1(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the CFGR1 register."]
  #[inline] pub fn set_cfgr1<F: FnOnce(Cfgr1) -> Cfgr1>(&self, f: F) -> &Self {
     let value = f(Cfgr1(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CFGR1 register."]
  #[inline] pub fn with_cfgr1<F: FnOnce(Cfgr1) -> Cfgr1>(&self, f: F) -> &Self {
     let tmp = self.cfgr1();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the EXTICR1 register."]
  #[inline] pub fn exticr1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the EXTICR1 register."]
  #[inline] pub fn exticr1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the EXTICR1 register."]
  #[inline] pub fn exticr1(&self) -> Exticr1 { 
     unsafe {
        Exticr1(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the EXTICR1 register."]
  #[inline] pub fn set_exticr1<F: FnOnce(Exticr1) -> Exticr1>(&self, f: F) -> &Self {
     let value = f(Exticr1(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the EXTICR1 register."]
  #[inline] pub fn with_exticr1<F: FnOnce(Exticr1) -> Exticr1>(&self, f: F) -> &Self {
     let tmp = self.exticr1();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the EXTICR2 register."]
  #[inline] pub fn exticr2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the EXTICR2 register."]
  #[inline] pub fn exticr2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the EXTICR2 register."]
  #[inline] pub fn exticr2(&self) -> Exticr2 { 
     unsafe {
        Exticr2(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the EXTICR2 register."]
  #[inline] pub fn set_exticr2<F: FnOnce(Exticr2) -> Exticr2>(&self, f: F) -> &Self {
     let value = f(Exticr2(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the EXTICR2 register."]
  #[inline] pub fn with_exticr2<F: FnOnce(Exticr2) -> Exticr2>(&self, f: F) -> &Self {
     let tmp = self.exticr2();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the EXTICR3 register."]
  #[inline] pub fn exticr3_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the EXTICR3 register."]
  #[inline] pub fn exticr3_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the EXTICR3 register."]
  #[inline] pub fn exticr3(&self) -> Exticr3 { 
     unsafe {
        Exticr3(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the EXTICR3 register."]
  #[inline] pub fn set_exticr3<F: FnOnce(Exticr3) -> Exticr3>(&self, f: F) -> &Self {
     let value = f(Exticr3(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the EXTICR3 register."]
  #[inline] pub fn with_exticr3<F: FnOnce(Exticr3) -> Exticr3>(&self, f: F) -> &Self {
     let tmp = self.exticr3();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the EXTICR4 register."]
  #[inline] pub fn exticr4_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the EXTICR4 register."]
  #[inline] pub fn exticr4_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Read the EXTICR4 register."]
  #[inline] pub fn exticr4(&self) -> Exticr4 { 
     unsafe {
        Exticr4(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
#[doc="Write the EXTICR4 register."]
  #[inline] pub fn set_exticr4<F: FnOnce(Exticr4) -> Exticr4>(&self, f: F) -> &Self {
     let value = f(Exticr4(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the EXTICR4 register."]
  #[inline] pub fn with_exticr4<F: FnOnce(Exticr4) -> Exticr4>(&self, f: F) -> &Self {
     let tmp = self.exticr4();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CFGR2 register."]
  #[inline] pub fn cfgr2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
#[doc="Get the *mut pointer for the CFGR2 register."]
  #[inline] pub fn cfgr2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
#[doc="Read the CFGR2 register."]
  #[inline] pub fn cfgr2(&self) -> Cfgr2 { 
     unsafe {
        Cfgr2(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
#[doc="Write the CFGR2 register."]
  #[inline] pub fn set_cfgr2<F: FnOnce(Cfgr2) -> Cfgr2>(&self, f: F) -> &Self {
     let value = f(Cfgr2(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CFGR2 register."]
  #[inline] pub fn with_cfgr2<F: FnOnce(Cfgr2) -> Cfgr2>(&self, f: F) -> &Self {
     let tmp = self.cfgr2();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the RCR register."]
  #[inline] pub fn rcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the RCR register."]
  #[inline] pub fn rcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the RCR register."]
  #[inline] pub fn rcr(&self) -> Rcr { 
     unsafe {
        Rcr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the RCR register."]
  #[inline] pub fn set_rcr<F: FnOnce(Rcr) -> Rcr>(&self, f: F) -> &Self {
     let value = f(Rcr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the RCR register."]
  #[inline] pub fn with_rcr<F: FnOnce(Rcr) -> Rcr>(&self, f: F) -> &Self {
     let tmp = self.rcr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }

}

#[doc="configuration register 1"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cfgr1(pub u32);
impl Cfgr1 {
#[doc="Memory mapping selection bits"]
  #[inline] pub fn mem_mode(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
  }
#[doc="Memory mapping selection bits"]
  #[inline] pub fn set_mem_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="USB interrupt remap"]
  #[inline] pub fn usb_it_rmp(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="USB interrupt remap"]
  #[inline] pub fn set_usb_it_rmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Timer 1 ITR3 selection"]
  #[inline] pub fn tim1_itr_rmp(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="Timer 1 ITR3 selection"]
  #[inline] pub fn set_tim1_itr_rmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="DAC trigger remap (when TSEL = 001)"]
  #[inline] pub fn dac_trig_rmp(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="DAC trigger remap (when TSEL = 001)"]
  #[inline] pub fn set_dac_trig_rmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="ADC24 DMA remapping bit"]
  #[inline] pub fn adc24_dma_rmp(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
  }
#[doc="ADC24 DMA remapping bit"]
  #[inline] pub fn set_adc24_dma_rmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="TIM16 DMA request remapping bit"]
  #[inline] pub fn tim16_dma_rmp(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
  }
#[doc="TIM16 DMA request remapping bit"]
  #[inline] pub fn set_tim16_dma_rmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="TIM17 DMA request remapping bit"]
  #[inline] pub fn tim17_dma_rmp(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
  }
#[doc="TIM17 DMA request remapping bit"]
  #[inline] pub fn set_tim17_dma_rmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="TIM6 and DAC1 DMA request remapping bit"]
  #[inline] pub fn tim6_dac1_dma_rmp(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
  }
#[doc="TIM6 and DAC1 DMA request remapping bit"]
  #[inline] pub fn set_tim6_dac1_dma_rmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="TIM7 and DAC2 DMA request remapping bit"]
  #[inline] pub fn tim7_dac2_dma_rmp(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
  }
#[doc="TIM7 and DAC2 DMA request remapping bit"]
  #[inline] pub fn set_tim7_dac2_dma_rmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Fast Mode Plus (FM+) driving capability activation bits."]
  #[inline] pub fn i2c_pb6_fm(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
  }
#[doc="Fast Mode Plus (FM+) driving capability activation bits."]
  #[inline] pub fn set_i2c_pb6_fm<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Fast Mode Plus (FM+) driving capability activation bits."]
  #[inline] pub fn i2c_pb7_fm(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
  }
#[doc="Fast Mode Plus (FM+) driving capability activation bits."]
  #[inline] pub fn set_i2c_pb7_fm<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="Fast Mode Plus (FM+) driving capability activation bits."]
  #[inline] pub fn i2c_pb8_fm(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
  }
#[doc="Fast Mode Plus (FM+) driving capability activation bits."]
  #[inline] pub fn set_i2c_pb8_fm<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="Fast Mode Plus (FM+) driving capability activation bits."]
  #[inline] pub fn i2c_pb9_fm(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
  }
#[doc="Fast Mode Plus (FM+) driving capability activation bits."]
  #[inline] pub fn set_i2c_pb9_fm<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="I2C1 Fast Mode Plus"]
  #[inline] pub fn i2c1_fm(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
  }
#[doc="I2C1 Fast Mode Plus"]
  #[inline] pub fn set_i2c1_fm<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="I2C2 Fast Mode Plus"]
  #[inline] pub fn i2c2_fm(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
  }
#[doc="I2C2 Fast Mode Plus"]
  #[inline] pub fn set_i2c2_fm<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="Encoder mode"]
  #[inline] pub fn encoder_mode(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
  }
#[doc="Encoder mode"]
  #[inline] pub fn set_encoder_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Interrupt enable bits from FPU"]
  #[inline] pub fn fpu_it(&self) -> bits::U6 {
     unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x3f) as u8) } // [31:26]
  }
#[doc="Interrupt enable bits from FPU"]
  #[inline] pub fn set_fpu_it<V: Into<bits::U6>>(mut self, value: V) -> Self {
     let value: bits::U6 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3f << 26);
     self.0 |= value << 26;
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
      if self.mem_mode() != 0 { try!(write!(f, " mem_mode=0x{:x}", self.mem_mode()))}
      if self.usb_it_rmp() != 0 { try!(write!(f, " usb_it_rmp"))}
      if self.tim1_itr_rmp() != 0 { try!(write!(f, " tim1_itr_rmp"))}
      if self.dac_trig_rmp() != 0 { try!(write!(f, " dac_trig_rmp"))}
      if self.adc24_dma_rmp() != 0 { try!(write!(f, " adc24_dma_rmp"))}
      if self.tim16_dma_rmp() != 0 { try!(write!(f, " tim16_dma_rmp"))}
      if self.tim17_dma_rmp() != 0 { try!(write!(f, " tim17_dma_rmp"))}
      if self.tim6_dac1_dma_rmp() != 0 { try!(write!(f, " tim6_dac1_dma_rmp"))}
      if self.tim7_dac2_dma_rmp() != 0 { try!(write!(f, " tim7_dac2_dma_rmp"))}
      if self.i2c_pb6_fm() != 0 { try!(write!(f, " i2c_pb6_fm"))}
      if self.i2c_pb7_fm() != 0 { try!(write!(f, " i2c_pb7_fm"))}
      if self.i2c_pb8_fm() != 0 { try!(write!(f, " i2c_pb8_fm"))}
      if self.i2c_pb9_fm() != 0 { try!(write!(f, " i2c_pb9_fm"))}
      if self.i2c1_fm() != 0 { try!(write!(f, " i2c1_fm"))}
      if self.i2c2_fm() != 0 { try!(write!(f, " i2c2_fm"))}
      if self.encoder_mode() != 0 { try!(write!(f, " encoder_mode=0x{:x}", self.encoder_mode()))}
      if self.fpu_it() != 0 { try!(write!(f, " fpu_it=0x{:x}", self.fpu_it()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="external interrupt configuration register 1"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Exticr1(pub u32);
impl Exticr1 {
#[doc="EXTI 3 configuration bits"]
  #[inline] pub fn exti3(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
  }
#[doc="EXTI 3 configuration bits"]
  #[inline] pub fn set_exti3<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

#[doc="EXTI 2 configuration bits"]
  #[inline] pub fn exti2(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
  }
#[doc="EXTI 2 configuration bits"]
  #[inline] pub fn set_exti2<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="EXTI 1 configuration bits"]
  #[inline] pub fn exti1(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
  }
#[doc="EXTI 1 configuration bits"]
  #[inline] pub fn set_exti1<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

#[doc="EXTI 0 configuration bits"]
  #[inline] pub fn exti0(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="EXTI 0 configuration bits"]
  #[inline] pub fn set_exti0<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
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
#[doc="external interrupt configuration register 2"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Exticr2(pub u32);
impl Exticr2 {
#[doc="EXTI 7 configuration bits"]
  #[inline] pub fn exti7(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
  }
#[doc="EXTI 7 configuration bits"]
  #[inline] pub fn set_exti7<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

#[doc="EXTI 6 configuration bits"]
  #[inline] pub fn exti6(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
  }
#[doc="EXTI 6 configuration bits"]
  #[inline] pub fn set_exti6<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="EXTI 5 configuration bits"]
  #[inline] pub fn exti5(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
  }
#[doc="EXTI 5 configuration bits"]
  #[inline] pub fn set_exti5<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

#[doc="EXTI 4 configuration bits"]
  #[inline] pub fn exti4(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="EXTI 4 configuration bits"]
  #[inline] pub fn set_exti4<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
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
#[doc="external interrupt configuration register 3"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Exticr3(pub u32);
impl Exticr3 {
#[doc="EXTI 11 configuration bits"]
  #[inline] pub fn exti11(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
  }
#[doc="EXTI 11 configuration bits"]
  #[inline] pub fn set_exti11<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

#[doc="EXTI 10 configuration bits"]
  #[inline] pub fn exti10(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
  }
#[doc="EXTI 10 configuration bits"]
  #[inline] pub fn set_exti10<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="EXTI 9 configuration bits"]
  #[inline] pub fn exti9(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
  }
#[doc="EXTI 9 configuration bits"]
  #[inline] pub fn set_exti9<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

#[doc="EXTI 8 configuration bits"]
  #[inline] pub fn exti8(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="EXTI 8 configuration bits"]
  #[inline] pub fn set_exti8<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
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
#[doc="external interrupt configuration register 4"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Exticr4(pub u32);
impl Exticr4 {
#[doc="EXTI 15 configuration bits"]
  #[inline] pub fn exti15(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
  }
#[doc="EXTI 15 configuration bits"]
  #[inline] pub fn set_exti15<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

#[doc="EXTI 14 configuration bits"]
  #[inline] pub fn exti14(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
  }
#[doc="EXTI 14 configuration bits"]
  #[inline] pub fn set_exti14<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="EXTI 13 configuration bits"]
  #[inline] pub fn exti13(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
  }
#[doc="EXTI 13 configuration bits"]
  #[inline] pub fn set_exti13<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

#[doc="EXTI 12 configuration bits"]
  #[inline] pub fn exti12(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="EXTI 12 configuration bits"]
  #[inline] pub fn set_exti12<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
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
#[doc="configuration register 2"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cfgr2(pub u32);
impl Cfgr2 {
#[doc="Cortex-M0 LOCKUP bit enable bit"]
  #[inline] pub fn locup_lock(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Cortex-M0 LOCKUP bit enable bit"]
  #[inline] pub fn set_locup_lock<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="SRAM parity lock bit"]
  #[inline] pub fn sram_parity_lock(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="SRAM parity lock bit"]
  #[inline] pub fn set_sram_parity_lock<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="PVD lock enable bit"]
  #[inline] pub fn pvd_lock(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="PVD lock enable bit"]
  #[inline] pub fn set_pvd_lock<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Bypass address bit 29 in parity calculation"]
  #[inline] pub fn byp_add_par(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="Bypass address bit 29 in parity calculation"]
  #[inline] pub fn set_byp_add_par<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="SRAM parity flag"]
  #[inline] pub fn sram_pef(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
  }
#[doc="SRAM parity flag"]
  #[inline] pub fn set_sram_pef<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
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
      if self.locup_lock() != 0 { try!(write!(f, " locup_lock"))}
      if self.sram_parity_lock() != 0 { try!(write!(f, " sram_parity_lock"))}
      if self.pvd_lock() != 0 { try!(write!(f, " pvd_lock"))}
      if self.byp_add_par() != 0 { try!(write!(f, " byp_add_par"))}
      if self.sram_pef() != 0 { try!(write!(f, " sram_pef"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="CCM SRAM protection register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcr(pub u32);
impl Rcr {
#[doc="CCM SRAM page write protection bit"]
  #[inline] pub fn page0_wp(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="CCM SRAM page write protection bit"]
  #[inline] pub fn set_page0_wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="CCM SRAM page write protection bit"]
  #[inline] pub fn page1_wp(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="CCM SRAM page write protection bit"]
  #[inline] pub fn set_page1_wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="CCM SRAM page write protection bit"]
  #[inline] pub fn page2_wp(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="CCM SRAM page write protection bit"]
  #[inline] pub fn set_page2_wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="CCM SRAM page write protection bit"]
  #[inline] pub fn page3_wp(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="CCM SRAM page write protection bit"]
  #[inline] pub fn set_page3_wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="CCM SRAM page write protection bit"]
  #[inline] pub fn page4_wp(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="CCM SRAM page write protection bit"]
  #[inline] pub fn set_page4_wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="CCM SRAM page write protection bit"]
  #[inline] pub fn page5_wp(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="CCM SRAM page write protection bit"]
  #[inline] pub fn set_page5_wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="CCM SRAM page write protection bit"]
  #[inline] pub fn page6_wp(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="CCM SRAM page write protection bit"]
  #[inline] pub fn set_page6_wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="CCM SRAM page write protection bit"]
  #[inline] pub fn page7_wp(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="CCM SRAM page write protection bit"]
  #[inline] pub fn set_page7_wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Rcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.page0_wp() != 0 { try!(write!(f, " page0_wp"))}
      if self.page1_wp() != 0 { try!(write!(f, " page1_wp"))}
      if self.page2_wp() != 0 { try!(write!(f, " page2_wp"))}
      if self.page3_wp() != 0 { try!(write!(f, " page3_wp"))}
      if self.page4_wp() != 0 { try!(write!(f, " page4_wp"))}
      if self.page5_wp() != 0 { try!(write!(f, " page5_wp"))}
      if self.page6_wp() != 0 { try!(write!(f, " page6_wp"))}
      if self.page7_wp() != 0 { try!(write!(f, " page7_wp"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

