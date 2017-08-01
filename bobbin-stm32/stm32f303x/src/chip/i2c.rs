pub const I2C1: I2c1 = Periph(0x40005400, I2c1Id {});
pub const I2C2: I2c2 = Periph(0x40005800, I2c2Id {});
pub const I2C3: I2c3 = Periph(0x40007800, I2c3Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="I2C Peripheral"]
pub struct Periph<T>(pub u32, pub T); 

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct I2c1Id {}
pub type I2c1 = Periph<I2c1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct I2c2Id {}
pub type I2c2 = Periph<I2c2Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct I2c3Id {}
pub type I2c3 = Periph<I2c3Id>;





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
  #[inline] pub fn pe(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Peripheral enable"]
  #[inline] pub fn set_pe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="TX Interrupt enable"]
  #[inline] pub fn txie(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="TX Interrupt enable"]
  #[inline] pub fn set_txie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="RX Interrupt enable"]
  #[inline] pub fn rxie(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="RX Interrupt enable"]
  #[inline] pub fn set_rxie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Address match interrupt enable (slave only)"]
  #[inline] pub fn addrie(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Address match interrupt enable (slave only)"]
  #[inline] pub fn set_addrie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Not acknowledge received interrupt enable"]
  #[inline] pub fn nackie(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Not acknowledge received interrupt enable"]
  #[inline] pub fn set_nackie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="STOP detection Interrupt enable"]
  #[inline] pub fn stopie(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="STOP detection Interrupt enable"]
  #[inline] pub fn set_stopie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Transfer Complete interrupt enable"]
  #[inline] pub fn tcie(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Transfer Complete interrupt enable"]
  #[inline] pub fn set_tcie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Error interrupts enable"]
  #[inline] pub fn errie(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Error interrupts enable"]
  #[inline] pub fn set_errie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Digital noise filter"]
  #[inline] pub fn dnf(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
#[doc="Digital noise filter"]
  #[inline] pub fn set_dnf(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Analog noise filter OFF"]
  #[inline] pub fn anfoff(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="Analog noise filter OFF"]
  #[inline] pub fn set_anfoff(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Software reset"]
  #[inline] pub fn swrst(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="Software reset"]
  #[inline] pub fn set_swrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="DMA transmission requests enable"]
  #[inline] pub fn txdmaen(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="DMA transmission requests enable"]
  #[inline] pub fn set_txdmaen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="DMA reception requests enable"]
  #[inline] pub fn rxdmaen(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="DMA reception requests enable"]
  #[inline] pub fn set_rxdmaen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Slave byte control"]
  #[inline] pub fn sbc(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="Slave byte control"]
  #[inline] pub fn set_sbc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Clock stretching disable"]
  #[inline] pub fn nostretch(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="Clock stretching disable"]
  #[inline] pub fn set_nostretch(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="Wakeup from STOP enable"]
  #[inline] pub fn wupen(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="Wakeup from STOP enable"]
  #[inline] pub fn set_wupen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="General call enable"]
  #[inline] pub fn gcen(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
#[doc="General call enable"]
  #[inline] pub fn set_gcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="SMBus Host address enable"]
  #[inline] pub fn smbhen(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
#[doc="SMBus Host address enable"]
  #[inline] pub fn set_smbhen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="SMBus Device Default address enable"]
  #[inline] pub fn smbden(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
#[doc="SMBus Device Default address enable"]
  #[inline] pub fn set_smbden(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="SMBUS alert enable"]
  #[inline] pub fn alerten(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="SMBUS alert enable"]
  #[inline] pub fn set_alerten(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="PEC enable"]
  #[inline] pub fn pecen(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
#[doc="PEC enable"]
  #[inline] pub fn set_pecen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
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
      if self.swrst() != 0 { try!(write!(f, " swrst"))}
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
  #[inline] pub fn pecbyte(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
#[doc="Packet error checking byte"]
  #[inline] pub fn set_pecbyte(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

#[doc="Automatic end mode (master mode)"]
  #[inline] pub fn autoend(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
#[doc="Automatic end mode (master mode)"]
  #[inline] pub fn set_autoend(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="NBYTES reload mode"]
  #[inline] pub fn reload(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
#[doc="NBYTES reload mode"]
  #[inline] pub fn set_reload(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Number of bytes"]
  #[inline] pub fn nbytes(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
#[doc="Number of bytes"]
  #[inline] pub fn set_nbytes(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

#[doc="NACK generation (slave mode)"]
  #[inline] pub fn nack(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="NACK generation (slave mode)"]
  #[inline] pub fn set_nack(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Stop generation (master mode)"]
  #[inline] pub fn stop(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="Stop generation (master mode)"]
  #[inline] pub fn set_stop(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Start generation"]
  #[inline] pub fn start(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="Start generation"]
  #[inline] pub fn set_start(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="10-bit address header only read direction (master receiver mode)"]
  #[inline] pub fn head10r(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="10-bit address header only read direction (master receiver mode)"]
  #[inline] pub fn set_head10r(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="10-bit addressing mode (master mode)"]
  #[inline] pub fn add10(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="10-bit addressing mode (master mode)"]
  #[inline] pub fn set_add10(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Transfer direction (master mode)"]
  #[inline] pub fn rd_wrn(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="Transfer direction (master mode)"]
  #[inline] pub fn set_rd_wrn(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Slave address 9:0 (master mode)"]
  #[inline] pub fn sadd(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3ff // [9:0]
  }
#[doc="Slave address 9:0 (master mode)"]
  #[inline] pub fn set_sadd(mut self, value: u32) -> Self {
     assert!((value & !0x3ff) == 0);
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
  #[inline] pub fn oa1_0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Interface address"]
  #[inline] pub fn set_oa1_0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Interface address"]
  #[inline] pub fn oa1_1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x7f // [7:1]
  }
#[doc="Interface address"]
  #[inline] pub fn set_oa1_1(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Interface address"]
  #[inline] pub fn oa1_8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
#[doc="Interface address"]
  #[inline] pub fn set_oa1_8(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Own Address 1 10-bit mode"]
  #[inline] pub fn oa1mode(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="Own Address 1 10-bit mode"]
  #[inline] pub fn set_oa1mode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Own Address 1 enable"]
  #[inline] pub fn oa1en(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="Own Address 1 enable"]
  #[inline] pub fn set_oa1en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
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
      if self.oa1_0() != 0 { try!(write!(f, " oa1_0"))}
      if self.oa1_1() != 0 { try!(write!(f, " oa1_1=0x{:x}", self.oa1_1()))}
      if self.oa1_8() != 0 { try!(write!(f, " oa1_8=0x{:x}", self.oa1_8()))}
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
  #[inline] pub fn oa2(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x7f // [7:1]
  }
#[doc="Interface address"]
  #[inline] pub fn set_oa2(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Own Address 2 masks"]
  #[inline] pub fn oa2msk(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x7 // [10:8]
  }
#[doc="Own Address 2 masks"]
  #[inline] pub fn set_oa2msk(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Own Address 2 enable"]
  #[inline] pub fn oa2en(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="Own Address 2 enable"]
  #[inline] pub fn set_oa2en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
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
  #[inline] pub fn scll(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
#[doc="SCL low period (master mode)"]
  #[inline] pub fn set_scll(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="SCL high period (master mode)"]
  #[inline] pub fn sclh(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
#[doc="SCL high period (master mode)"]
  #[inline] pub fn set_sclh(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Data hold time"]
  #[inline] pub fn sdadel(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
#[doc="Data hold time"]
  #[inline] pub fn set_sdadel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Data setup time"]
  #[inline] pub fn scldel(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0xf // [23:20]
  }
#[doc="Data setup time"]
  #[inline] pub fn set_scldel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Timing prescaler"]
  #[inline] pub fn presc(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0xf // [31:28]
  }
#[doc="Timing prescaler"]
  #[inline] pub fn set_presc(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
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
  #[inline] pub fn timeouta(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
#[doc="Bus timeout A"]
  #[inline] pub fn set_timeouta(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Idle clock timeout detection"]
  #[inline] pub fn tidle(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="Idle clock timeout detection"]
  #[inline] pub fn set_tidle(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Clock timeout enable"]
  #[inline] pub fn timouten(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="Clock timeout enable"]
  #[inline] pub fn set_timouten(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Bus timeout B"]
  #[inline] pub fn timeoutb(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xfff // [27:16]
  }
#[doc="Bus timeout B"]
  #[inline] pub fn set_timeoutb(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Extended clock timeout enable"]
  #[inline] pub fn texten(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Extended clock timeout enable"]
  #[inline] pub fn set_texten(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
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
  #[inline] pub fn addcode(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x7f // [23:17]
  }
#[doc="Address match code (Slave mode)"]
  #[inline] pub fn set_addcode(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 17);
     self.0 |= value << 17;
     self
  }

#[doc="Transfer direction (Slave mode)"]
  #[inline] pub fn dir(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="Transfer direction (Slave mode)"]
  #[inline] pub fn set_dir(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Bus busy"]
  #[inline] pub fn busy(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="Bus busy"]
  #[inline] pub fn set_busy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="SMBus alert"]
  #[inline] pub fn alert(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="SMBus alert"]
  #[inline] pub fn set_alert(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Timeout or t_low detection flag"]
  #[inline] pub fn timeout(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="Timeout or t_low detection flag"]
  #[inline] pub fn set_timeout(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="PEC Error in reception"]
  #[inline] pub fn pecerr(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="PEC Error in reception"]
  #[inline] pub fn set_pecerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Overrun/Underrun (slave mode)"]
  #[inline] pub fn ovr(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="Overrun/Underrun (slave mode)"]
  #[inline] pub fn set_ovr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Arbitration lost"]
  #[inline] pub fn arlo(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Arbitration lost"]
  #[inline] pub fn set_arlo(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Bus error"]
  #[inline] pub fn berr(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Bus error"]
  #[inline] pub fn set_berr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Transfer Complete Reload"]
  #[inline] pub fn tcr(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Transfer Complete Reload"]
  #[inline] pub fn set_tcr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Transfer Complete (master mode)"]
  #[inline] pub fn tc(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Transfer Complete (master mode)"]
  #[inline] pub fn set_tc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Stop detection flag"]
  #[inline] pub fn stopf(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Stop detection flag"]
  #[inline] pub fn set_stopf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Not acknowledge received flag"]
  #[inline] pub fn nackf(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Not acknowledge received flag"]
  #[inline] pub fn set_nackf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Address matched (slave mode)"]
  #[inline] pub fn addr(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Address matched (slave mode)"]
  #[inline] pub fn set_addr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Receive data register not empty (receivers)"]
  #[inline] pub fn rxne(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Receive data register not empty (receivers)"]
  #[inline] pub fn set_rxne(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Transmit interrupt status (transmitters)"]
  #[inline] pub fn txis(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Transmit interrupt status (transmitters)"]
  #[inline] pub fn set_txis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Transmit data register empty (transmitters)"]
  #[inline] pub fn txe(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Transmit data register empty (transmitters)"]
  #[inline] pub fn set_txe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
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
  #[inline] pub fn alertcf(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="Alert flag clear"]
  #[inline] pub fn set_alertcf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Timeout detection flag clear"]
  #[inline] pub fn timoutcf(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="Timeout detection flag clear"]
  #[inline] pub fn set_timoutcf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="PEC Error flag clear"]
  #[inline] pub fn peccf(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="PEC Error flag clear"]
  #[inline] pub fn set_peccf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Overrun/Underrun flag clear"]
  #[inline] pub fn ovrcf(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="Overrun/Underrun flag clear"]
  #[inline] pub fn set_ovrcf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Arbitration lost flag clear"]
  #[inline] pub fn arlocf(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Arbitration lost flag clear"]
  #[inline] pub fn set_arlocf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Bus error flag clear"]
  #[inline] pub fn berrcf(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Bus error flag clear"]
  #[inline] pub fn set_berrcf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Stop detection flag clear"]
  #[inline] pub fn stopcf(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Stop detection flag clear"]
  #[inline] pub fn set_stopcf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Not Acknowledge flag clear"]
  #[inline] pub fn nackcf(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Not Acknowledge flag clear"]
  #[inline] pub fn set_nackcf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Address Matched flag clear"]
  #[inline] pub fn addrcf(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Address Matched flag clear"]
  #[inline] pub fn set_addrcf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
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
  #[inline] pub fn pec(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
#[doc="Packet error checking register"]
  #[inline] pub fn set_pec(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
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
  #[inline] pub fn rxdata(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
#[doc="8-bit receive data"]
  #[inline] pub fn set_rxdata(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
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
  #[inline] pub fn txdata(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
#[doc="8-bit transmit data"]
  #[inline] pub fn set_txdata(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
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
