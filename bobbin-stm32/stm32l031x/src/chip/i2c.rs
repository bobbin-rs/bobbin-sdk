#[allow(unused_imports)] use bobbin_common::bits;
pub const I2C1: I2c1 = Periph(0x40005400, I2c1Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="I2C Peripheral"]
pub struct Periph<T>(pub u32, pub T); 

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct I2c1Id {}
pub type I2c1 = Periph<I2c1Id>;

impl super::sig::Signal<super::sig::I2c1Smba> for I2c1 {}
impl super::sig::SignalSmba<super::sig::I2c1Smba> for I2c1 {}
impl super::sig::Signal<super::sig::I2c1Scl> for I2c1 {}
impl super::sig::SignalScl<super::sig::I2c1Scl> for I2c1 {}
impl super::sig::Signal<super::sig::I2c1Sda> for I2c1 {}
impl super::sig::SignalSda<super::sig::I2c1Sda> for I2c1 {}


impl<T> Periph<T> {
#[doc="Get the *const pointer for the CR1 register."]
  #[inline] pub fn cr1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the CR1 register."]
  #[inline] pub fn cr1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the CR1 register."]
  #[inline] pub fn cr1(&self) -> Cr1 { 
     unsafe {
        Cr1(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the CR1 register."]
  #[inline] pub fn set_cr1(&self, value: Cr1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CR1 register."]
  #[inline] pub fn with_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
     let tmp = self.cr1();
     self.set_cr1(f(tmp))
  }

#[doc="Get the *const pointer for the CR2 register."]
  #[inline] pub fn cr2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the CR2 register."]
  #[inline] pub fn cr2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the CR2 register."]
  #[inline] pub fn cr2(&self) -> Cr2 { 
     unsafe {
        Cr2(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the CR2 register."]
  #[inline] pub fn set_cr2(&self, value: Cr2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CR2 register."]
  #[inline] pub fn with_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
     let tmp = self.cr2();
     self.set_cr2(f(tmp))
  }

#[doc="Get the *const pointer for the OAR1 register."]
  #[inline] pub fn oar1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the OAR1 register."]
  #[inline] pub fn oar1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the OAR1 register."]
  #[inline] pub fn oar1(&self) -> Oar1 { 
     unsafe {
        Oar1(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the OAR1 register."]
  #[inline] pub fn set_oar1(&self, value: Oar1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the OAR1 register."]
  #[inline] pub fn with_oar1<F: FnOnce(Oar1) -> Oar1>(&self, f: F) -> &Self {
     let tmp = self.oar1();
     self.set_oar1(f(tmp))
  }

#[doc="Get the *const pointer for the OAR2 register."]
  #[inline] pub fn oar2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the OAR2 register."]
  #[inline] pub fn oar2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the OAR2 register."]
  #[inline] pub fn oar2(&self) -> Oar2 { 
     unsafe {
        Oar2(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the OAR2 register."]
  #[inline] pub fn set_oar2(&self, value: Oar2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the OAR2 register."]
  #[inline] pub fn with_oar2<F: FnOnce(Oar2) -> Oar2>(&self, f: F) -> &Self {
     let tmp = self.oar2();
     self.set_oar2(f(tmp))
  }

#[doc="Get the *const pointer for the TIMINGR register."]
  #[inline] pub fn timingr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the TIMINGR register."]
  #[inline] pub fn timingr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the TIMINGR register."]
  #[inline] pub fn timingr(&self) -> Timingr { 
     unsafe {
        Timingr(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the TIMINGR register."]
  #[inline] pub fn set_timingr(&self, value: Timingr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TIMINGR register."]
  #[inline] pub fn with_timingr<F: FnOnce(Timingr) -> Timingr>(&self, f: F) -> &Self {
     let tmp = self.timingr();
     self.set_timingr(f(tmp))
  }

#[doc="Get the *const pointer for the TIMEOUTR register."]
  #[inline] pub fn timeoutr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the TIMEOUTR register."]
  #[inline] pub fn timeoutr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Read the TIMEOUTR register."]
  #[inline] pub fn timeoutr(&self) -> Timeoutr { 
     unsafe {
        Timeoutr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
#[doc="Write the TIMEOUTR register."]
  #[inline] pub fn set_timeoutr(&self, value: Timeoutr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TIMEOUTR register."]
  #[inline] pub fn with_timeoutr<F: FnOnce(Timeoutr) -> Timeoutr>(&self, f: F) -> &Self {
     let tmp = self.timeoutr();
     self.set_timeoutr(f(tmp))
  }

#[doc="Get the *const pointer for the ISR register."]
  #[inline] pub fn isr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
#[doc="Get the *mut pointer for the ISR register."]
  #[inline] pub fn isr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
#[doc="Read the ISR register."]
  #[inline] pub fn isr(&self) -> Isr { 
     unsafe {
        Isr(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
#[doc="Write the ISR register."]
  #[inline] pub fn set_isr(&self, value: Isr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ISR register."]
  #[inline] pub fn with_isr<F: FnOnce(Isr) -> Isr>(&self, f: F) -> &Self {
     let tmp = self.isr();
     self.set_isr(f(tmp))
  }

#[doc="Get the *const pointer for the ICR register."]
  #[inline] pub fn icr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the ICR register."]
  #[inline] pub fn icr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Write the ICR register."]
  #[inline] pub fn set_icr(&self, value: Icr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the PECR register."]
  #[inline] pub fn pecr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
#[doc="Get the *mut pointer for the PECR register."]
  #[inline] pub fn pecr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
#[doc="Read the PECR register."]
  #[inline] pub fn pecr(&self) -> Pecr { 
     unsafe {
        Pecr(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RXDR register."]
  #[inline] pub fn rxdr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
#[doc="Get the *mut pointer for the RXDR register."]
  #[inline] pub fn rxdr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
#[doc="Read the RXDR register."]
  #[inline] pub fn rxdr(&self) -> Rxdr { 
     unsafe {
        Rxdr(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }

#[doc="Get the *const pointer for the TXDR register."]
  #[inline] pub fn txdr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x28) as *const u32
  }
#[doc="Get the *mut pointer for the TXDR register."]
  #[inline] pub fn txdr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x28) as *mut u32
  }
#[doc="Read the TXDR register."]
  #[inline] pub fn txdr(&self) -> Txdr { 
     unsafe {
        Txdr(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
     }
  }
#[doc="Write the TXDR register."]
  #[inline] pub fn set_txdr(&self, value: Txdr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TXDR register."]
  #[inline] pub fn with_txdr<F: FnOnce(Txdr) -> Txdr>(&self, f: F) -> &Self {
     let tmp = self.txdr();
     self.set_txdr(f(tmp))
  }

}

#[doc="Control register 1"]
#[derive(PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
#[doc="Peripheral enable"]
  #[inline] pub fn pe(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Peripheral enable"]
  #[inline] pub fn set_pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="TX Interrupt enable"]
  #[inline] pub fn txie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="TX Interrupt enable"]
  #[inline] pub fn set_txie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="RX Interrupt enable"]
  #[inline] pub fn rxie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="RX Interrupt enable"]
  #[inline] pub fn set_rxie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Address match interrupt enable (slave only)"]
  #[inline] pub fn addrie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="Address match interrupt enable (slave only)"]
  #[inline] pub fn set_addrie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Not acknowledge received interrupt enable"]
  #[inline] pub fn nackie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="Not acknowledge received interrupt enable"]
  #[inline] pub fn set_nackie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="STOP detection Interrupt enable"]
  #[inline] pub fn stopie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="STOP detection Interrupt enable"]
  #[inline] pub fn set_stopie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Transfer Complete interrupt enable"]
  #[inline] pub fn tcie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="Transfer Complete interrupt enable"]
  #[inline] pub fn set_tcie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Error interrupts enable"]
  #[inline] pub fn errie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="Error interrupts enable"]
  #[inline] pub fn set_errie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Digital noise filter"]
  #[inline] pub fn dnf(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
  }
#[doc="Digital noise filter"]
  #[inline] pub fn set_dnf<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Analog noise filter OFF"]
  #[inline] pub fn anfoff(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
  }
#[doc="Analog noise filter OFF"]
  #[inline] pub fn set_anfoff<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="DMA transmission requests enable"]
  #[inline] pub fn txdmaen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
  }
#[doc="DMA transmission requests enable"]
  #[inline] pub fn set_txdmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="DMA reception requests enable"]
  #[inline] pub fn rxdmaen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
  }
#[doc="DMA reception requests enable"]
  #[inline] pub fn set_rxdmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Slave byte control"]
  #[inline] pub fn sbc(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
  }
#[doc="Slave byte control"]
  #[inline] pub fn set_sbc<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Clock stretching disable"]
  #[inline] pub fn nostretch(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
  }
#[doc="Clock stretching disable"]
  #[inline] pub fn set_nostretch<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="Wakeup from STOP enable"]
  #[inline] pub fn wupen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
  }
#[doc="Wakeup from STOP enable"]
  #[inline] pub fn set_wupen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="General call enable"]
  #[inline] pub fn gcen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
  }
#[doc="General call enable"]
  #[inline] pub fn set_gcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="SMBus Host address enable"]
  #[inline] pub fn smbhen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
  }
#[doc="SMBus Host address enable"]
  #[inline] pub fn set_smbhen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="SMBus Device Default address enable"]
  #[inline] pub fn smbden(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
  }
#[doc="SMBus Device Default address enable"]
  #[inline] pub fn set_smbden<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="SMBUS alert enable"]
  #[inline] pub fn alerten(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
  }
#[doc="SMBUS alert enable"]
  #[inline] pub fn set_alerten<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="PEC enable"]
  #[inline] pub fn pecen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
  }
#[doc="PEC enable"]
  #[inline] pub fn set_pecen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

}
impl ::core::fmt::Display for Cr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pe() != 0 { try!(write!(f, " pe"))}
      if self.txie() != 0 { try!(write!(f, " txie"))}
      if self.rxie() != 0 { try!(write!(f, " rxie"))}
      if self.addrie() != 0 { try!(write!(f, " addrie"))}
      if self.nackie() != 0 { try!(write!(f, " nackie"))}
      if self.stopie() != 0 { try!(write!(f, " stopie"))}
      if self.tcie() != 0 { try!(write!(f, " tcie"))}
      if self.errie() != 0 { try!(write!(f, " errie"))}
      if self.dnf() != 0 { try!(write!(f, " dnf=0x{:x}", self.dnf()))}
      if self.anfoff() != 0 { try!(write!(f, " anfoff"))}
      if self.txdmaen() != 0 { try!(write!(f, " txdmaen"))}
      if self.rxdmaen() != 0 { try!(write!(f, " rxdmaen"))}
      if self.sbc() != 0 { try!(write!(f, " sbc"))}
      if self.nostretch() != 0 { try!(write!(f, " nostretch"))}
      if self.wupen() != 0 { try!(write!(f, " wupen"))}
      if self.gcen() != 0 { try!(write!(f, " gcen"))}
      if self.smbhen() != 0 { try!(write!(f, " smbhen"))}
      if self.smbden() != 0 { try!(write!(f, " smbden"))}
      if self.alerten() != 0 { try!(write!(f, " alerten"))}
      if self.pecen() != 0 { try!(write!(f, " pecen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control register 2"]
#[derive(PartialEq, Eq)]
pub struct Cr2(pub u32);
impl Cr2 {
#[doc="Packet error checking byte"]
  #[inline] pub fn pecbyte(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
  }
#[doc="Packet error checking byte"]
  #[inline] pub fn set_pecbyte<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

#[doc="Automatic end mode (master mode)"]
  #[inline] pub fn autoend(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
  }
#[doc="Automatic end mode (master mode)"]
  #[inline] pub fn set_autoend<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="NBYTES reload mode"]
  #[inline] pub fn reload(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
  }
#[doc="NBYTES reload mode"]
  #[inline] pub fn set_reload<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Number of bytes"]
  #[inline] pub fn nbytes(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
  }
#[doc="Number of bytes"]
  #[inline] pub fn set_nbytes<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

#[doc="NACK generation (slave mode)"]
  #[inline] pub fn nack(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
  }
#[doc="NACK generation (slave mode)"]
  #[inline] pub fn set_nack<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Stop generation (master mode)"]
  #[inline] pub fn stop(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
  }
#[doc="Stop generation (master mode)"]
  #[inline] pub fn set_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Start generation"]
  #[inline] pub fn start(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
  }
#[doc="Start generation"]
  #[inline] pub fn set_start<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="10-bit address header only read direction (master receiver mode)"]
  #[inline] pub fn head10r(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
  }
#[doc="10-bit address header only read direction (master receiver mode)"]
  #[inline] pub fn set_head10r<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="10-bit addressing mode (master mode)"]
  #[inline] pub fn add10(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
  }
#[doc="10-bit addressing mode (master mode)"]
  #[inline] pub fn set_add10<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Transfer direction (master mode)"]
  #[inline] pub fn rd_wrn(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
  }
#[doc="Transfer direction (master mode)"]
  #[inline] pub fn set_rd_wrn<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Slave address bit (master mode)"]
  #[inline] pub fn sadd(&self) -> bits::U10 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
  }
#[doc="Slave address bit (master mode)"]
  #[inline] pub fn set_sadd<V: Into<bits::U10>>(mut self, value: V) -> Self {
     let value: bits::U10 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3ff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pecbyte() != 0 { try!(write!(f, " pecbyte"))}
      if self.autoend() != 0 { try!(write!(f, " autoend"))}
      if self.reload() != 0 { try!(write!(f, " reload"))}
      if self.nbytes() != 0 { try!(write!(f, " nbytes=0x{:x}", self.nbytes()))}
      if self.nack() != 0 { try!(write!(f, " nack"))}
      if self.stop() != 0 { try!(write!(f, " stop"))}
      if self.start() != 0 { try!(write!(f, " start"))}
      if self.head10r() != 0 { try!(write!(f, " head10r"))}
      if self.add10() != 0 { try!(write!(f, " add10"))}
      if self.rd_wrn() != 0 { try!(write!(f, " rd_wrn"))}
      if self.sadd() != 0 { try!(write!(f, " sadd=0x{:x}", self.sadd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Own address register 1"]
#[derive(PartialEq, Eq)]
pub struct Oar1(pub u32);
impl Oar1 {
#[doc="Interface address"]
  #[inline] pub fn oa1(&self) -> bits::U10 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
  }
#[doc="Interface address"]
  #[inline] pub fn set_oa1<V: Into<bits::U10>>(mut self, value: V) -> Self {
     let value: bits::U10 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3ff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Own Address 1 10-bit mode"]
  #[inline] pub fn oa1mode(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
  }
#[doc="Own Address 1 10-bit mode"]
  #[inline] pub fn set_oa1mode<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Own Address 1 enable"]
  #[inline] pub fn oa1en(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
  }
#[doc="Own Address 1 enable"]
  #[inline] pub fn set_oa1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

}
impl ::core::fmt::Display for Oar1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Oar1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.oa1() != 0 { try!(write!(f, " oa1=0x{:x}", self.oa1()))}
      if self.oa1mode() != 0 { try!(write!(f, " oa1mode"))}
      if self.oa1en() != 0 { try!(write!(f, " oa1en"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Own address register 2"]
#[derive(PartialEq, Eq)]
pub struct Oar2(pub u32);
impl Oar2 {
#[doc="Interface address"]
  #[inline] pub fn oa2(&self) -> bits::U7 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7f) as u8) } // [7:1]
  }
#[doc="Interface address"]
  #[inline] pub fn set_oa2<V: Into<bits::U7>>(mut self, value: V) -> Self {
     let value: bits::U7 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7f << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Own Address 2 masks"]
  #[inline] pub fn oa2msk(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
  }
#[doc="Own Address 2 masks"]
  #[inline] pub fn set_oa2msk<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Own Address 2 enable"]
  #[inline] pub fn oa2en(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
  }
#[doc="Own Address 2 enable"]
  #[inline] pub fn set_oa2en<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

}
impl ::core::fmt::Display for Oar2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Oar2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.oa2() != 0 { try!(write!(f, " oa2=0x{:x}", self.oa2()))}
      if self.oa2msk() != 0 { try!(write!(f, " oa2msk=0x{:x}", self.oa2msk()))}
      if self.oa2en() != 0 { try!(write!(f, " oa2en"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Timing register"]
#[derive(PartialEq, Eq)]
pub struct Timingr(pub u32);
impl Timingr {
#[doc="SCL low period (master mode)"]
  #[inline] pub fn scll(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="SCL low period (master mode)"]
  #[inline] pub fn set_scll<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="SCL high period (master mode)"]
  #[inline] pub fn sclh(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
  }
#[doc="SCL high period (master mode)"]
  #[inline] pub fn set_sclh<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Data hold time"]
  #[inline] pub fn sdadel(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
  }
#[doc="Data hold time"]
  #[inline] pub fn set_sdadel<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Data setup time"]
  #[inline] pub fn scldel(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
  }
#[doc="Data setup time"]
  #[inline] pub fn set_scldel<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Timing prescaler"]
  #[inline] pub fn presc(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 28) & 0xf) as u8) } // [31:28]
  }
#[doc="Timing prescaler"]
  #[inline] pub fn set_presc<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 28);
     self.0 |= value << 28;
     self
  }

}
impl ::core::fmt::Display for Timingr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Timingr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.scll() != 0 { try!(write!(f, " scll=0x{:x}", self.scll()))}
      if self.sclh() != 0 { try!(write!(f, " sclh=0x{:x}", self.sclh()))}
      if self.sdadel() != 0 { try!(write!(f, " sdadel=0x{:x}", self.sdadel()))}
      if self.scldel() != 0 { try!(write!(f, " scldel=0x{:x}", self.scldel()))}
      if self.presc() != 0 { try!(write!(f, " presc=0x{:x}", self.presc()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Status register 1"]
#[derive(PartialEq, Eq)]
pub struct Timeoutr(pub u32);
impl Timeoutr {
#[doc="Bus timeout A"]
  #[inline] pub fn timeouta(&self) -> bits::U12 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
  }
#[doc="Bus timeout A"]
  #[inline] pub fn set_timeouta<V: Into<bits::U12>>(mut self, value: V) -> Self {
     let value: bits::U12 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Idle clock timeout detection"]
  #[inline] pub fn tidle(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
  }
#[doc="Idle clock timeout detection"]
  #[inline] pub fn set_tidle<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Clock timeout enable"]
  #[inline] pub fn timouten(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
  }
#[doc="Clock timeout enable"]
  #[inline] pub fn set_timouten<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Bus timeout B"]
  #[inline] pub fn timeoutb(&self) -> bits::U12 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xfff) as u16) } // [27:16]
  }
#[doc="Bus timeout B"]
  #[inline] pub fn set_timeoutb<V: Into<bits::U12>>(mut self, value: V) -> Self {
     let value: bits::U12 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xfff << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Extended clock timeout enable"]
  #[inline] pub fn texten(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
  }
#[doc="Extended clock timeout enable"]
  #[inline] pub fn set_texten<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Timeoutr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Timeoutr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.timeouta() != 0 { try!(write!(f, " timeouta=0x{:x}", self.timeouta()))}
      if self.tidle() != 0 { try!(write!(f, " tidle"))}
      if self.timouten() != 0 { try!(write!(f, " timouten"))}
      if self.timeoutb() != 0 { try!(write!(f, " timeoutb=0x{:x}", self.timeoutb()))}
      if self.texten() != 0 { try!(write!(f, " texten"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt and Status register"]
#[derive(PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
#[doc="Address match code (Slave mode)"]
  #[inline] pub fn addcode(&self) -> bits::U7 {
     unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x7f) as u8) } // [23:17]
  }
#[doc="Address match code (Slave mode)"]
  #[inline] pub fn set_addcode<V: Into<bits::U7>>(mut self, value: V) -> Self {
     let value: bits::U7 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7f << 17);
     self.0 |= value << 17;
     self
  }

#[doc="Transfer direction (Slave mode)"]
  #[inline] pub fn dir(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
  }
#[doc="Transfer direction (Slave mode)"]
  #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Bus busy"]
  #[inline] pub fn busy(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
  }
#[doc="Bus busy"]
  #[inline] pub fn set_busy<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="SMBus alert"]
  #[inline] pub fn alert(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
  }
#[doc="SMBus alert"]
  #[inline] pub fn set_alert<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Timeout or t_low detection flag"]
  #[inline] pub fn timeout(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
  }
#[doc="Timeout or t_low detection flag"]
  #[inline] pub fn set_timeout<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="PEC Error in reception"]
  #[inline] pub fn pecerr(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
  }
#[doc="PEC Error in reception"]
  #[inline] pub fn set_pecerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Overrun/Underrun (slave mode)"]
  #[inline] pub fn ovr(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
  }
#[doc="Overrun/Underrun (slave mode)"]
  #[inline] pub fn set_ovr<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Arbitration lost"]
  #[inline] pub fn arlo(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
  }
#[doc="Arbitration lost"]
  #[inline] pub fn set_arlo<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Bus error"]
  #[inline] pub fn berr(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
  }
#[doc="Bus error"]
  #[inline] pub fn set_berr<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Transfer Complete Reload"]
  #[inline] pub fn tcr(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="Transfer Complete Reload"]
  #[inline] pub fn set_tcr<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Transfer Complete (master mode)"]
  #[inline] pub fn tc(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="Transfer Complete (master mode)"]
  #[inline] pub fn set_tc<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Stop detection flag"]
  #[inline] pub fn stopf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="Stop detection flag"]
  #[inline] pub fn set_stopf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Not acknowledge received flag"]
  #[inline] pub fn nackf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="Not acknowledge received flag"]
  #[inline] pub fn set_nackf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Address matched (slave mode)"]
  #[inline] pub fn addr(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="Address matched (slave mode)"]
  #[inline] pub fn set_addr<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Receive data register not empty (receivers)"]
  #[inline] pub fn rxne(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="Receive data register not empty (receivers)"]
  #[inline] pub fn set_rxne<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Transmit interrupt status (transmitters)"]
  #[inline] pub fn txis(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Transmit interrupt status (transmitters)"]
  #[inline] pub fn set_txis<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Transmit data register empty (transmitters)"]
  #[inline] pub fn txe(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Transmit data register empty (transmitters)"]
  #[inline] pub fn set_txe<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Isr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Isr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.addcode() != 0 { try!(write!(f, " addcode=0x{:x}", self.addcode()))}
      if self.dir() != 0 { try!(write!(f, " dir"))}
      if self.busy() != 0 { try!(write!(f, " busy"))}
      if self.alert() != 0 { try!(write!(f, " alert"))}
      if self.timeout() != 0 { try!(write!(f, " timeout"))}
      if self.pecerr() != 0 { try!(write!(f, " pecerr"))}
      if self.ovr() != 0 { try!(write!(f, " ovr"))}
      if self.arlo() != 0 { try!(write!(f, " arlo"))}
      if self.berr() != 0 { try!(write!(f, " berr"))}
      if self.tcr() != 0 { try!(write!(f, " tcr"))}
      if self.tc() != 0 { try!(write!(f, " tc"))}
      if self.stopf() != 0 { try!(write!(f, " stopf"))}
      if self.nackf() != 0 { try!(write!(f, " nackf"))}
      if self.addr() != 0 { try!(write!(f, " addr"))}
      if self.rxne() != 0 { try!(write!(f, " rxne"))}
      if self.txis() != 0 { try!(write!(f, " txis"))}
      if self.txe() != 0 { try!(write!(f, " txe"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt clear register"]
#[derive(PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
#[doc="Alert flag clear"]
  #[inline] pub fn alertcf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
  }
#[doc="Alert flag clear"]
  #[inline] pub fn set_alertcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Timeout detection flag clear"]
  #[inline] pub fn timoutcf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
  }
#[doc="Timeout detection flag clear"]
  #[inline] pub fn set_timoutcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="PEC Error flag clear"]
  #[inline] pub fn peccf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
  }
#[doc="PEC Error flag clear"]
  #[inline] pub fn set_peccf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Overrun/Underrun flag clear"]
  #[inline] pub fn ovrcf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
  }
#[doc="Overrun/Underrun flag clear"]
  #[inline] pub fn set_ovrcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Arbitration lost flag clear"]
  #[inline] pub fn arlocf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
  }
#[doc="Arbitration lost flag clear"]
  #[inline] pub fn set_arlocf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Bus error flag clear"]
  #[inline] pub fn berrcf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
  }
#[doc="Bus error flag clear"]
  #[inline] pub fn set_berrcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Stop detection flag clear"]
  #[inline] pub fn stopcf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="Stop detection flag clear"]
  #[inline] pub fn set_stopcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Not Acknowledge flag clear"]
  #[inline] pub fn nackcf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="Not Acknowledge flag clear"]
  #[inline] pub fn set_nackcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Address Matched flag clear"]
  #[inline] pub fn addrcf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="Address Matched flag clear"]
  #[inline] pub fn set_addrcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}
impl ::core::fmt::Display for Icr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Icr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.alertcf() != 0 { try!(write!(f, " alertcf"))}
      if self.timoutcf() != 0 { try!(write!(f, " timoutcf"))}
      if self.peccf() != 0 { try!(write!(f, " peccf"))}
      if self.ovrcf() != 0 { try!(write!(f, " ovrcf"))}
      if self.arlocf() != 0 { try!(write!(f, " arlocf"))}
      if self.berrcf() != 0 { try!(write!(f, " berrcf"))}
      if self.stopcf() != 0 { try!(write!(f, " stopcf"))}
      if self.nackcf() != 0 { try!(write!(f, " nackcf"))}
      if self.addrcf() != 0 { try!(write!(f, " addrcf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PEC register"]
#[derive(PartialEq, Eq)]
pub struct Pecr(pub u32);
impl Pecr {
#[doc="Packet error checking register"]
  #[inline] pub fn pec(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="Packet error checking register"]
  #[inline] pub fn set_pec<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Pecr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pecr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pec() != 0 { try!(write!(f, " pec=0x{:x}", self.pec()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Receive data register"]
#[derive(PartialEq, Eq)]
pub struct Rxdr(pub u32);
impl Rxdr {
#[doc="8-bit receive data"]
  #[inline] pub fn rxdata(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="8-bit receive data"]
  #[inline] pub fn set_rxdata<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Rxdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rxdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rxdata() != 0 { try!(write!(f, " rxdata=0x{:x}", self.rxdata()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Transmit data register"]
#[derive(PartialEq, Eq)]
pub struct Txdr(pub u32);
impl Txdr {
#[doc="8-bit transmit data"]
  #[inline] pub fn txdata(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="8-bit transmit data"]
  #[inline] pub fn set_txdata<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Txdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Txdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txdata() != 0 { try!(write!(f, " txdata=0x{:x}", self.txdata()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
pub trait IrqI2c<T> {
   fn irq_i2c(&self) -> super::irq::Irq<T>;
}

pub trait RegisterI2cHandler {
   fn register_i2c_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleI2c>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleI2c {
   fn handle_i2c(&self);
}

impl IrqI2c<super::irq::I2c1Id> for I2c1 {
   fn irq_i2c(&self) -> super::irq::IrqI2c1 { super::irq::IRQ_I2C1 }
}

impl RegisterI2cHandler for I2c1 {
   fn register_i2c_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleI2c>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleI2c>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_i2c() }
       }
       super::irq::set_handler(23, Some(wrapper::<F>));
       super::irq::IrqGuard::new(23)
   }
}

