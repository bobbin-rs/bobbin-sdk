//! Alternate function I/O
#[allow(unused_imports)] use bobbin_common::bits;
pub const AFIO: Afio = Afio(0x40010000);

#[doc="Alternate function I/O"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Afio(pub u32);
impl Afio {
#[doc="Get the *const pointer for the EVCR register."]
  #[inline] pub fn evcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the EVCR register."]
  #[inline] pub fn evcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the EVCR register."]
  #[inline] pub fn evcr(&self) -> Evcr { 
     unsafe {
        Evcr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the EVCR register."]
  #[inline] pub fn set_evcr(&self, value: Evcr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the EVCR register."]
  #[inline] pub fn with_evcr<F: FnOnce(Evcr) -> Evcr>(&self, f: F) -> &Self {
     let tmp = self.evcr();
     self.set_evcr(f(tmp))
  }

#[doc="Get the *const pointer for the MAPR register."]
  #[inline] pub fn mapr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the MAPR register."]
  #[inline] pub fn mapr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the MAPR register."]
  #[inline] pub fn mapr(&self) -> Mapr { 
     unsafe {
        Mapr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the MAPR register."]
  #[inline] pub fn set_mapr(&self, value: Mapr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MAPR register."]
  #[inline] pub fn with_mapr<F: FnOnce(Mapr) -> Mapr>(&self, f: F) -> &Self {
     let tmp = self.mapr();
     self.set_mapr(f(tmp))
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
  #[inline] pub fn set_exticr1(&self, value: Exticr1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the EXTICR1 register."]
  #[inline] pub fn with_exticr1<F: FnOnce(Exticr1) -> Exticr1>(&self, f: F) -> &Self {
     let tmp = self.exticr1();
     self.set_exticr1(f(tmp))
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
  #[inline] pub fn set_exticr2(&self, value: Exticr2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the EXTICR2 register."]
  #[inline] pub fn with_exticr2<F: FnOnce(Exticr2) -> Exticr2>(&self, f: F) -> &Self {
     let tmp = self.exticr2();
     self.set_exticr2(f(tmp))
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
  #[inline] pub fn set_exticr3(&self, value: Exticr3) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the EXTICR3 register."]
  #[inline] pub fn with_exticr3<F: FnOnce(Exticr3) -> Exticr3>(&self, f: F) -> &Self {
     let tmp = self.exticr3();
     self.set_exticr3(f(tmp))
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
  #[inline] pub fn set_exticr4(&self, value: Exticr4) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the EXTICR4 register."]
  #[inline] pub fn with_exticr4<F: FnOnce(Exticr4) -> Exticr4>(&self, f: F) -> &Self {
     let tmp = self.exticr4();
     self.set_exticr4(f(tmp))
  }

#[doc="Get the *const pointer for the MAPR2 register."]
  #[inline] pub fn mapr2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the MAPR2 register."]
  #[inline] pub fn mapr2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Read the MAPR2 register."]
  #[inline] pub fn mapr2(&self) -> Mapr2 { 
     unsafe {
        Mapr2(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
#[doc="Write the MAPR2 register."]
  #[inline] pub fn set_mapr2(&self, value: Mapr2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MAPR2 register."]
  #[inline] pub fn with_mapr2<F: FnOnce(Mapr2) -> Mapr2>(&self, f: F) -> &Self {
     let tmp = self.mapr2();
     self.set_mapr2(f(tmp))
  }

}

#[doc="Event Control Register (AFIO_EVCR)"]
#[derive(PartialEq, Eq)]
pub struct Evcr(pub u32);
impl Evcr {
#[doc="Pin selection"]
  #[inline] pub fn pin(&self) -> bits::B4 {
     (((self.0 as u32) >> 0) & 0xf).into() // [3:0]
  }
#[doc="Pin selection"]
  #[inline] pub fn set_pin<V: Into<bits::B4>>(mut self, value: V) -> Self {
     let value: bits::B4 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Port selection"]
  #[inline] pub fn port(&self) -> bits::B3 {
     (((self.0 as u32) >> 4) & 0x7).into() // [6:4]
  }
#[doc="Port selection"]
  #[inline] pub fn set_port<V: Into<bits::B3>>(mut self, value: V) -> Self {
     let value: bits::B3 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Event Output Enable"]
  #[inline] pub fn evoe(&self) -> bits::B1 {
     (((self.0 as u32) >> 7) & 0x1).into() // [7]
  }
#[doc="Event Output Enable"]
  #[inline] pub fn set_evoe<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Evcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Evcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pin() != 0 { try!(write!(f, " pin=0x{:x}", self.pin()))}
      if self.port() != 0 { try!(write!(f, " port=0x{:x}", self.port()))}
      if self.evoe() != 0 { try!(write!(f, " evoe"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="AF remap and debug I/O configuration register (AFIO_MAPR)"]
#[derive(PartialEq, Eq)]
pub struct Mapr(pub u32);
impl Mapr {
#[doc="SPI1 remapping"]
  #[inline] pub fn spi1_remap(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="SPI1 remapping"]
  #[inline] pub fn set_spi1_remap<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="I2C1 remapping"]
  #[inline] pub fn i2c1_remap(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="I2C1 remapping"]
  #[inline] pub fn set_i2c1_remap<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="USART1 remapping"]
  #[inline] pub fn usart1_remap(&self) -> bits::B1 {
     (((self.0 as u32) >> 2) & 0x1).into() // [2]
  }
#[doc="USART1 remapping"]
  #[inline] pub fn set_usart1_remap<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="USART2 remapping"]
  #[inline] pub fn usart2_remap(&self) -> bits::B1 {
     (((self.0 as u32) >> 3) & 0x1).into() // [3]
  }
#[doc="USART2 remapping"]
  #[inline] pub fn set_usart2_remap<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="USART3 remapping"]
  #[inline] pub fn usart3_remap(&self) -> bits::B2 {
     (((self.0 as u32) >> 4) & 0x3).into() // [5:4]
  }
#[doc="USART3 remapping"]
  #[inline] pub fn set_usart3_remap<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="TIM1 remapping"]
  #[inline] pub fn tim1_remap(&self) -> bits::B2 {
     (((self.0 as u32) >> 6) & 0x3).into() // [7:6]
  }
#[doc="TIM1 remapping"]
  #[inline] pub fn set_tim1_remap<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="TIM2 remapping"]
  #[inline] pub fn tim2_remap(&self) -> bits::B2 {
     (((self.0 as u32) >> 8) & 0x3).into() // [9:8]
  }
#[doc="TIM2 remapping"]
  #[inline] pub fn set_tim2_remap<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="TIM3 remapping"]
  #[inline] pub fn tim3_remap(&self) -> bits::B2 {
     (((self.0 as u32) >> 10) & 0x3).into() // [11:10]
  }
#[doc="TIM3 remapping"]
  #[inline] pub fn set_tim3_remap<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="TIM4 remapping"]
  #[inline] pub fn tim4_remap(&self) -> bits::B1 {
     (((self.0 as u32) >> 12) & 0x1).into() // [12]
  }
#[doc="TIM4 remapping"]
  #[inline] pub fn set_tim4_remap<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="CAN1 remapping"]
  #[inline] pub fn can_remap(&self) -> bits::B2 {
     (((self.0 as u32) >> 13) & 0x3).into() // [14:13]
  }
#[doc="CAN1 remapping"]
  #[inline] pub fn set_can_remap<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Port D0/Port D1 mapping on OSCIN/OSCOUT"]
  #[inline] pub fn pd01_remap(&self) -> bits::B1 {
     (((self.0 as u32) >> 15) & 0x1).into() // [15]
  }
#[doc="Port D0/Port D1 mapping on OSCIN/OSCOUT"]
  #[inline] pub fn set_pd01_remap<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Set and cleared by software"]
  #[inline] pub fn tim5ch4_iremap(&self) -> bits::B1 {
     (((self.0 as u32) >> 16) & 0x1).into() // [16]
  }
#[doc="Set and cleared by software"]
  #[inline] pub fn set_tim5ch4_iremap<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="ADC 1 External trigger injected conversion remapping"]
  #[inline] pub fn adc1_etrginj_remap(&self) -> bits::B1 {
     (((self.0 as u32) >> 17) & 0x1).into() // [17]
  }
#[doc="ADC 1 External trigger injected conversion remapping"]
  #[inline] pub fn set_adc1_etrginj_remap<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="ADC 1 external trigger regular conversion remapping"]
  #[inline] pub fn adc1_etrgreg_remap(&self) -> bits::B1 {
     (((self.0 as u32) >> 18) & 0x1).into() // [18]
  }
#[doc="ADC 1 external trigger regular conversion remapping"]
  #[inline] pub fn set_adc1_etrgreg_remap<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="ADC 2 external trigger injected conversion remapping"]
  #[inline] pub fn adc2_etrginj_remap(&self) -> bits::B1 {
     (((self.0 as u32) >> 19) & 0x1).into() // [19]
  }
#[doc="ADC 2 external trigger injected conversion remapping"]
  #[inline] pub fn set_adc2_etrginj_remap<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="ADC 2 external trigger regular conversion remapping"]
  #[inline] pub fn adc2_etrgreg_remap(&self) -> bits::B1 {
     (((self.0 as u32) >> 20) & 0x1).into() // [20]
  }
#[doc="ADC 2 external trigger regular conversion remapping"]
  #[inline] pub fn set_adc2_etrgreg_remap<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Serial wire JTAG configuration"]
  #[inline] pub fn swj_cfg(&self) -> bits::B3 {
     (((self.0 as u32) >> 24) & 0x7).into() // [26:24]
  }
#[doc="Serial wire JTAG configuration"]
  #[inline] pub fn set_swj_cfg<V: Into<bits::B3>>(mut self, value: V) -> Self {
     let value: bits::B3 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

}
impl ::core::fmt::Display for Mapr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Mapr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.spi1_remap() != 0 { try!(write!(f, " spi1_remap"))}
      if self.i2c1_remap() != 0 { try!(write!(f, " i2c1_remap"))}
      if self.usart1_remap() != 0 { try!(write!(f, " usart1_remap"))}
      if self.usart2_remap() != 0 { try!(write!(f, " usart2_remap"))}
      if self.usart3_remap() != 0 { try!(write!(f, " usart3_remap=0x{:x}", self.usart3_remap()))}
      if self.tim1_remap() != 0 { try!(write!(f, " tim1_remap=0x{:x}", self.tim1_remap()))}
      if self.tim2_remap() != 0 { try!(write!(f, " tim2_remap=0x{:x}", self.tim2_remap()))}
      if self.tim3_remap() != 0 { try!(write!(f, " tim3_remap=0x{:x}", self.tim3_remap()))}
      if self.tim4_remap() != 0 { try!(write!(f, " tim4_remap"))}
      if self.can_remap() != 0 { try!(write!(f, " can_remap=0x{:x}", self.can_remap()))}
      if self.pd01_remap() != 0 { try!(write!(f, " pd01_remap"))}
      if self.tim5ch4_iremap() != 0 { try!(write!(f, " tim5ch4_iremap"))}
      if self.adc1_etrginj_remap() != 0 { try!(write!(f, " adc1_etrginj_remap"))}
      if self.adc1_etrgreg_remap() != 0 { try!(write!(f, " adc1_etrgreg_remap"))}
      if self.adc2_etrginj_remap() != 0 { try!(write!(f, " adc2_etrginj_remap"))}
      if self.adc2_etrgreg_remap() != 0 { try!(write!(f, " adc2_etrgreg_remap"))}
      if self.swj_cfg() != 0 { try!(write!(f, " swj_cfg=0x{:x}", self.swj_cfg()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="External interrupt configuration register 1 (AFIO_EXTICR1)"]
#[derive(PartialEq, Eq)]
pub struct Exticr1(pub u32);
impl Exticr1 {
#[doc="EXTI0 configuration"]
  #[inline] pub fn exti0(&self) -> bits::B4 {
     (((self.0 as u32) >> 0) & 0xf).into() // [3:0]
  }
#[doc="EXTI0 configuration"]
  #[inline] pub fn set_exti0<V: Into<bits::B4>>(mut self, value: V) -> Self {
     let value: bits::B4 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

#[doc="EXTI1 configuration"]
  #[inline] pub fn exti1(&self) -> bits::B4 {
     (((self.0 as u32) >> 4) & 0xf).into() // [7:4]
  }
#[doc="EXTI1 configuration"]
  #[inline] pub fn set_exti1<V: Into<bits::B4>>(mut self, value: V) -> Self {
     let value: bits::B4 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

#[doc="EXTI2 configuration"]
  #[inline] pub fn exti2(&self) -> bits::B4 {
     (((self.0 as u32) >> 8) & 0xf).into() // [11:8]
  }
#[doc="EXTI2 configuration"]
  #[inline] pub fn set_exti2<V: Into<bits::B4>>(mut self, value: V) -> Self {
     let value: bits::B4 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="EXTI3 configuration"]
  #[inline] pub fn exti3(&self) -> bits::B4 {
     (((self.0 as u32) >> 12) & 0xf).into() // [15:12]
  }
#[doc="EXTI3 configuration"]
  #[inline] pub fn set_exti3<V: Into<bits::B4>>(mut self, value: V) -> Self {
     let value: bits::B4 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
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
      if self.exti0() != 0 { try!(write!(f, " exti0=0x{:x}", self.exti0()))}
      if self.exti1() != 0 { try!(write!(f, " exti1=0x{:x}", self.exti1()))}
      if self.exti2() != 0 { try!(write!(f, " exti2=0x{:x}", self.exti2()))}
      if self.exti3() != 0 { try!(write!(f, " exti3=0x{:x}", self.exti3()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="External interrupt configuration register 2 (AFIO_EXTICR2)"]
#[derive(PartialEq, Eq)]
pub struct Exticr2(pub u32);
impl Exticr2 {
#[doc="EXTI4 configuration"]
  #[inline] pub fn exti4(&self) -> bits::B4 {
     (((self.0 as u32) >> 0) & 0xf).into() // [3:0]
  }
#[doc="EXTI4 configuration"]
  #[inline] pub fn set_exti4<V: Into<bits::B4>>(mut self, value: V) -> Self {
     let value: bits::B4 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

#[doc="EXTI5 configuration"]
  #[inline] pub fn exti5(&self) -> bits::B4 {
     (((self.0 as u32) >> 4) & 0xf).into() // [7:4]
  }
#[doc="EXTI5 configuration"]
  #[inline] pub fn set_exti5<V: Into<bits::B4>>(mut self, value: V) -> Self {
     let value: bits::B4 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

#[doc="EXTI6 configuration"]
  #[inline] pub fn exti6(&self) -> bits::B4 {
     (((self.0 as u32) >> 8) & 0xf).into() // [11:8]
  }
#[doc="EXTI6 configuration"]
  #[inline] pub fn set_exti6<V: Into<bits::B4>>(mut self, value: V) -> Self {
     let value: bits::B4 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="EXTI7 configuration"]
  #[inline] pub fn exti7(&self) -> bits::B4 {
     (((self.0 as u32) >> 12) & 0xf).into() // [15:12]
  }
#[doc="EXTI7 configuration"]
  #[inline] pub fn set_exti7<V: Into<bits::B4>>(mut self, value: V) -> Self {
     let value: bits::B4 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
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
      if self.exti4() != 0 { try!(write!(f, " exti4=0x{:x}", self.exti4()))}
      if self.exti5() != 0 { try!(write!(f, " exti5=0x{:x}", self.exti5()))}
      if self.exti6() != 0 { try!(write!(f, " exti6=0x{:x}", self.exti6()))}
      if self.exti7() != 0 { try!(write!(f, " exti7=0x{:x}", self.exti7()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="External interrupt configuration register 3 (AFIO_EXTICR3)"]
#[derive(PartialEq, Eq)]
pub struct Exticr3(pub u32);
impl Exticr3 {
#[doc="EXTI8 configuration"]
  #[inline] pub fn exti8(&self) -> bits::B4 {
     (((self.0 as u32) >> 0) & 0xf).into() // [3:0]
  }
#[doc="EXTI8 configuration"]
  #[inline] pub fn set_exti8<V: Into<bits::B4>>(mut self, value: V) -> Self {
     let value: bits::B4 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

#[doc="EXTI9 configuration"]
  #[inline] pub fn exti9(&self) -> bits::B4 {
     (((self.0 as u32) >> 4) & 0xf).into() // [7:4]
  }
#[doc="EXTI9 configuration"]
  #[inline] pub fn set_exti9<V: Into<bits::B4>>(mut self, value: V) -> Self {
     let value: bits::B4 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

#[doc="EXTI10 configuration"]
  #[inline] pub fn exti10(&self) -> bits::B4 {
     (((self.0 as u32) >> 8) & 0xf).into() // [11:8]
  }
#[doc="EXTI10 configuration"]
  #[inline] pub fn set_exti10<V: Into<bits::B4>>(mut self, value: V) -> Self {
     let value: bits::B4 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="EXTI11 configuration"]
  #[inline] pub fn exti11(&self) -> bits::B4 {
     (((self.0 as u32) >> 12) & 0xf).into() // [15:12]
  }
#[doc="EXTI11 configuration"]
  #[inline] pub fn set_exti11<V: Into<bits::B4>>(mut self, value: V) -> Self {
     let value: bits::B4 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
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
      if self.exti8() != 0 { try!(write!(f, " exti8=0x{:x}", self.exti8()))}
      if self.exti9() != 0 { try!(write!(f, " exti9=0x{:x}", self.exti9()))}
      if self.exti10() != 0 { try!(write!(f, " exti10=0x{:x}", self.exti10()))}
      if self.exti11() != 0 { try!(write!(f, " exti11=0x{:x}", self.exti11()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="External interrupt configuration register 4 (AFIO_EXTICR4)"]
#[derive(PartialEq, Eq)]
pub struct Exticr4(pub u32);
impl Exticr4 {
#[doc="EXTI12 configuration"]
  #[inline] pub fn exti12(&self) -> bits::B4 {
     (((self.0 as u32) >> 0) & 0xf).into() // [3:0]
  }
#[doc="EXTI12 configuration"]
  #[inline] pub fn set_exti12<V: Into<bits::B4>>(mut self, value: V) -> Self {
     let value: bits::B4 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

#[doc="EXTI13 configuration"]
  #[inline] pub fn exti13(&self) -> bits::B4 {
     (((self.0 as u32) >> 4) & 0xf).into() // [7:4]
  }
#[doc="EXTI13 configuration"]
  #[inline] pub fn set_exti13<V: Into<bits::B4>>(mut self, value: V) -> Self {
     let value: bits::B4 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

#[doc="EXTI14 configuration"]
  #[inline] pub fn exti14(&self) -> bits::B4 {
     (((self.0 as u32) >> 8) & 0xf).into() // [11:8]
  }
#[doc="EXTI14 configuration"]
  #[inline] pub fn set_exti14<V: Into<bits::B4>>(mut self, value: V) -> Self {
     let value: bits::B4 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="EXTI15 configuration"]
  #[inline] pub fn exti15(&self) -> bits::B4 {
     (((self.0 as u32) >> 12) & 0xf).into() // [15:12]
  }
#[doc="EXTI15 configuration"]
  #[inline] pub fn set_exti15<V: Into<bits::B4>>(mut self, value: V) -> Self {
     let value: bits::B4 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
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
      if self.exti12() != 0 { try!(write!(f, " exti12=0x{:x}", self.exti12()))}
      if self.exti13() != 0 { try!(write!(f, " exti13=0x{:x}", self.exti13()))}
      if self.exti14() != 0 { try!(write!(f, " exti14=0x{:x}", self.exti14()))}
      if self.exti15() != 0 { try!(write!(f, " exti15=0x{:x}", self.exti15()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="AF remap and debug I/O configuration register"]
#[derive(PartialEq, Eq)]
pub struct Mapr2(pub u32);
impl Mapr2 {
#[doc="TIM9 remapping"]
  #[inline] pub fn tim9_remap(&self) -> bits::B1 {
     (((self.0 as u32) >> 5) & 0x1).into() // [5]
  }
#[doc="TIM9 remapping"]
  #[inline] pub fn set_tim9_remap<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="TIM10 remapping"]
  #[inline] pub fn tim10_remap(&self) -> bits::B1 {
     (((self.0 as u32) >> 6) & 0x1).into() // [6]
  }
#[doc="TIM10 remapping"]
  #[inline] pub fn set_tim10_remap<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="TIM11 remapping"]
  #[inline] pub fn tim11_remap(&self) -> bits::B1 {
     (((self.0 as u32) >> 7) & 0x1).into() // [7]
  }
#[doc="TIM11 remapping"]
  #[inline] pub fn set_tim11_remap<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="TIM13 remapping"]
  #[inline] pub fn tim13_remap(&self) -> bits::B1 {
     (((self.0 as u32) >> 8) & 0x1).into() // [8]
  }
#[doc="TIM13 remapping"]
  #[inline] pub fn set_tim13_remap<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="TIM14 remapping"]
  #[inline] pub fn tim14_remap(&self) -> bits::B1 {
     (((self.0 as u32) >> 9) & 0x1).into() // [9]
  }
#[doc="TIM14 remapping"]
  #[inline] pub fn set_tim14_remap<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="NADV connect/disconnect"]
  #[inline] pub fn fsmc_nadv(&self) -> bits::B1 {
     (((self.0 as u32) >> 10) & 0x1).into() // [10]
  }
#[doc="NADV connect/disconnect"]
  #[inline] pub fn set_fsmc_nadv<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

}
impl ::core::fmt::Display for Mapr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Mapr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tim9_remap() != 0 { try!(write!(f, " tim9_remap"))}
      if self.tim10_remap() != 0 { try!(write!(f, " tim10_remap"))}
      if self.tim11_remap() != 0 { try!(write!(f, " tim11_remap"))}
      if self.tim13_remap() != 0 { try!(write!(f, " tim13_remap"))}
      if self.tim14_remap() != 0 { try!(write!(f, " tim14_remap"))}
      if self.fsmc_nadv() != 0 { try!(write!(f, " fsmc_nadv"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

