#[allow(unused_imports)] use bobbin_common::bits;
pub const I2C3: I2c3 = Periph(0x40005c00, I2c3Id {});
pub const I2C2: I2c2 = Periph(0x40005800, I2c2Id {});
pub const I2C1: I2c1 = Periph(0x40005400, I2c1Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="I2C Peripheral"]
pub struct Periph<T>(pub u32, pub T); 

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct I2c3Id {}
pub type I2c3 = Periph<I2c3Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct I2c2Id {}
pub type I2c2 = Periph<I2c2Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct I2c1Id {}
pub type I2c1 = Periph<I2c1Id>;





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

#[doc="Get the *const pointer for the DR register."]
  #[inline] pub fn dr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the DR register."]
  #[inline] pub fn dr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the DR register."]
  #[inline] pub fn dr(&self) -> Dr { 
     unsafe {
        Dr(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the DR register."]
  #[inline] pub fn set_dr(&self, value: Dr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DR register."]
  #[inline] pub fn with_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
     let tmp = self.dr();
     self.set_dr(f(tmp))
  }

#[doc="Get the *const pointer for the SR1 register."]
  #[inline] pub fn sr1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the SR1 register."]
  #[inline] pub fn sr1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Read the SR1 register."]
  #[inline] pub fn sr1(&self) -> Sr1 { 
     unsafe {
        Sr1(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
#[doc="Write the SR1 register."]
  #[inline] pub fn set_sr1(&self, value: Sr1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SR1 register."]
  #[inline] pub fn with_sr1<F: FnOnce(Sr1) -> Sr1>(&self, f: F) -> &Self {
     let tmp = self.sr1();
     self.set_sr1(f(tmp))
  }

#[doc="Get the *const pointer for the SR2 register."]
  #[inline] pub fn sr2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
#[doc="Get the *mut pointer for the SR2 register."]
  #[inline] pub fn sr2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
#[doc="Read the SR2 register."]
  #[inline] pub fn sr2(&self) -> Sr2 { 
     unsafe {
        Sr2(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }

#[doc="Get the *const pointer for the CCR register."]
  #[inline] pub fn ccr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the CCR register."]
  #[inline] pub fn ccr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Read the CCR register."]
  #[inline] pub fn ccr(&self) -> Ccr { 
     unsafe {
        Ccr(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
#[doc="Write the CCR register."]
  #[inline] pub fn set_ccr(&self, value: Ccr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CCR register."]
  #[inline] pub fn with_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
     let tmp = self.ccr();
     self.set_ccr(f(tmp))
  }

#[doc="Get the *const pointer for the TRISE register."]
  #[inline] pub fn trise_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
#[doc="Get the *mut pointer for the TRISE register."]
  #[inline] pub fn trise_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
#[doc="Read the TRISE register."]
  #[inline] pub fn trise(&self) -> Trise { 
     unsafe {
        Trise(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
#[doc="Write the TRISE register."]
  #[inline] pub fn set_trise(&self, value: Trise) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TRISE register."]
  #[inline] pub fn with_trise<F: FnOnce(Trise) -> Trise>(&self, f: F) -> &Self {
     let tmp = self.trise();
     self.set_trise(f(tmp))
  }

#[doc="Get the *const pointer for the FLTR register."]
  #[inline] pub fn fltr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
#[doc="Get the *mut pointer for the FLTR register."]
  #[inline] pub fn fltr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
#[doc="Read the FLTR register."]
  #[inline] pub fn fltr(&self) -> Fltr { 
     unsafe {
        Fltr(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }
#[doc="Write the FLTR register."]
  #[inline] pub fn set_fltr(&self, value: Fltr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the FLTR register."]
  #[inline] pub fn with_fltr<F: FnOnce(Fltr) -> Fltr>(&self, f: F) -> &Self {
     let tmp = self.fltr();
     self.set_fltr(f(tmp))
  }

}

#[doc="Control register 1"]
#[derive(PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
#[doc="Software reset"]
  #[inline] pub fn swrst(&self) -> bits::B1 {
     (((self.0 as u32) >> 15) & 0x1).into() // [15]
  }
#[doc="Software reset"]
  #[inline] pub fn set_swrst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="SMBus alert"]
  #[inline] pub fn alert(&self) -> bits::B1 {
     (((self.0 as u32) >> 13) & 0x1).into() // [13]
  }
#[doc="SMBus alert"]
  #[inline] pub fn set_alert<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Packet error checking"]
  #[inline] pub fn pec(&self) -> bits::B1 {
     (((self.0 as u32) >> 12) & 0x1).into() // [12]
  }
#[doc="Packet error checking"]
  #[inline] pub fn set_pec<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Acknowledge/PEC Position (for data reception)"]
  #[inline] pub fn pos(&self) -> bits::B1 {
     (((self.0 as u32) >> 11) & 0x1).into() // [11]
  }
#[doc="Acknowledge/PEC Position (for data reception)"]
  #[inline] pub fn set_pos<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Acknowledge enable"]
  #[inline] pub fn ack(&self) -> bits::B1 {
     (((self.0 as u32) >> 10) & 0x1).into() // [10]
  }
#[doc="Acknowledge enable"]
  #[inline] pub fn set_ack<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Stop generation"]
  #[inline] pub fn stop(&self) -> bits::B1 {
     (((self.0 as u32) >> 9) & 0x1).into() // [9]
  }
#[doc="Stop generation"]
  #[inline] pub fn set_stop<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Start generation"]
  #[inline] pub fn start(&self) -> bits::B1 {
     (((self.0 as u32) >> 8) & 0x1).into() // [8]
  }
#[doc="Start generation"]
  #[inline] pub fn set_start<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Clock stretching disable (Slave mode)"]
  #[inline] pub fn nostretch(&self) -> bits::B1 {
     (((self.0 as u32) >> 7) & 0x1).into() // [7]
  }
#[doc="Clock stretching disable (Slave mode)"]
  #[inline] pub fn set_nostretch<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="General call enable"]
  #[inline] pub fn engc(&self) -> bits::B1 {
     (((self.0 as u32) >> 6) & 0x1).into() // [6]
  }
#[doc="General call enable"]
  #[inline] pub fn set_engc<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="PEC enable"]
  #[inline] pub fn enpec(&self) -> bits::B1 {
     (((self.0 as u32) >> 5) & 0x1).into() // [5]
  }
#[doc="PEC enable"]
  #[inline] pub fn set_enpec<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="ARP enable"]
  #[inline] pub fn enarp(&self) -> bits::B1 {
     (((self.0 as u32) >> 4) & 0x1).into() // [4]
  }
#[doc="ARP enable"]
  #[inline] pub fn set_enarp<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="SMBus type"]
  #[inline] pub fn smbtype(&self) -> bits::B1 {
     (((self.0 as u32) >> 3) & 0x1).into() // [3]
  }
#[doc="SMBus type"]
  #[inline] pub fn set_smbtype<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="SMBus mode"]
  #[inline] pub fn smbus(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="SMBus mode"]
  #[inline] pub fn set_smbus<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Peripheral enable"]
  #[inline] pub fn pe(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Peripheral enable"]
  #[inline] pub fn set_pe<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
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
      if self.swrst() != 0 { try!(write!(f, " swrst"))}
      if self.alert() != 0 { try!(write!(f, " alert"))}
      if self.pec() != 0 { try!(write!(f, " pec"))}
      if self.pos() != 0 { try!(write!(f, " pos"))}
      if self.ack() != 0 { try!(write!(f, " ack"))}
      if self.stop() != 0 { try!(write!(f, " stop"))}
      if self.start() != 0 { try!(write!(f, " start"))}
      if self.nostretch() != 0 { try!(write!(f, " nostretch"))}
      if self.engc() != 0 { try!(write!(f, " engc"))}
      if self.enpec() != 0 { try!(write!(f, " enpec"))}
      if self.enarp() != 0 { try!(write!(f, " enarp"))}
      if self.smbtype() != 0 { try!(write!(f, " smbtype"))}
      if self.smbus() != 0 { try!(write!(f, " smbus"))}
      if self.pe() != 0 { try!(write!(f, " pe"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control register 2"]
#[derive(PartialEq, Eq)]
pub struct Cr2(pub u32);
impl Cr2 {
#[doc="DMA last transfer"]
  #[inline] pub fn last(&self) -> bits::B1 {
     (((self.0 as u32) >> 12) & 0x1).into() // [12]
  }
#[doc="DMA last transfer"]
  #[inline] pub fn set_last<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="DMA requests enable"]
  #[inline] pub fn dmaen(&self) -> bits::B1 {
     (((self.0 as u32) >> 11) & 0x1).into() // [11]
  }
#[doc="DMA requests enable"]
  #[inline] pub fn set_dmaen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Buffer interrupt enable"]
  #[inline] pub fn itbufen(&self) -> bits::B1 {
     (((self.0 as u32) >> 10) & 0x1).into() // [10]
  }
#[doc="Buffer interrupt enable"]
  #[inline] pub fn set_itbufen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Event interrupt enable"]
  #[inline] pub fn itevten(&self) -> bits::B1 {
     (((self.0 as u32) >> 9) & 0x1).into() // [9]
  }
#[doc="Event interrupt enable"]
  #[inline] pub fn set_itevten<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Error interrupt enable"]
  #[inline] pub fn iterren(&self) -> bits::B1 {
     (((self.0 as u32) >> 8) & 0x1).into() // [8]
  }
#[doc="Error interrupt enable"]
  #[inline] pub fn set_iterren<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Peripheral clock frequency"]
  #[inline] pub fn freq(&self) -> bits::B6 {
     (((self.0 as u32) >> 0) & 0x3f).into() // [5:0]
  }
#[doc="Peripheral clock frequency"]
  #[inline] pub fn set_freq<V: Into<bits::B6>>(mut self, value: V) -> Self {
     let value: bits::B6 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 0);
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
      if self.last() != 0 { try!(write!(f, " last"))}
      if self.dmaen() != 0 { try!(write!(f, " dmaen"))}
      if self.itbufen() != 0 { try!(write!(f, " itbufen"))}
      if self.itevten() != 0 { try!(write!(f, " itevten"))}
      if self.iterren() != 0 { try!(write!(f, " iterren"))}
      if self.freq() != 0 { try!(write!(f, " freq=0x{:x}", self.freq()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Own address register 1"]
#[derive(PartialEq, Eq)]
pub struct Oar1(pub u32);
impl Oar1 {
#[doc="Addressing mode (slave mode)"]
  #[inline] pub fn addmode(&self) -> bits::B1 {
     (((self.0 as u32) >> 15) & 0x1).into() // [15]
  }
#[doc="Addressing mode (slave mode)"]
  #[inline] pub fn set_addmode<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Interface address"]
  #[inline] pub fn add10(&self) -> bits::B2 {
     (((self.0 as u32) >> 8) & 0x3).into() // [9:8]
  }
#[doc="Interface address"]
  #[inline] pub fn set_add10<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Interface address"]
  #[inline] pub fn add7(&self) -> bits::B7 {
     (((self.0 as u32) >> 1) & 0x7f).into() // [7:1]
  }
#[doc="Interface address"]
  #[inline] pub fn set_add7<V: Into<bits::B7>>(mut self, value: V) -> Self {
     let value: bits::B7 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Interface address"]
  #[inline] pub fn add0(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Interface address"]
  #[inline] pub fn set_add0<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
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
      if self.addmode() != 0 { try!(write!(f, " addmode"))}
      if self.add10() != 0 { try!(write!(f, " add10=0x{:x}", self.add10()))}
      if self.add7() != 0 { try!(write!(f, " add7=0x{:x}", self.add7()))}
      if self.add0() != 0 { try!(write!(f, " add0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Own address register 2"]
#[derive(PartialEq, Eq)]
pub struct Oar2(pub u32);
impl Oar2 {
#[doc="Interface address"]
  #[inline] pub fn add2(&self) -> bits::B7 {
     (((self.0 as u32) >> 1) & 0x7f).into() // [7:1]
  }
#[doc="Interface address"]
  #[inline] pub fn set_add2<V: Into<bits::B7>>(mut self, value: V) -> Self {
     let value: bits::B7 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Dual addressing mode enable"]
  #[inline] pub fn endual(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Dual addressing mode enable"]
  #[inline] pub fn set_endual<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
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
      if self.add2() != 0 { try!(write!(f, " add2=0x{:x}", self.add2()))}
      if self.endual() != 0 { try!(write!(f, " endual"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Data register"]
#[derive(PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
#[doc="8-bit data register"]
  #[inline] pub fn dr(&self) -> bits::B8 {
     (((self.0 as u32) >> 0) & 0xff).into() // [7:0]
  }
#[doc="8-bit data register"]
  #[inline] pub fn set_dr<V: Into<bits::B8>>(mut self, value: V) -> Self {
     let value: bits::B8 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dr() != 0 { try!(write!(f, " dr=0x{:x}", self.dr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Status register 1"]
#[derive(PartialEq, Eq)]
pub struct Sr1(pub u32);
impl Sr1 {
#[doc="SMBus alert"]
  #[inline] pub fn smbalert(&self) -> bits::B1 {
     (((self.0 as u32) >> 15) & 0x1).into() // [15]
  }
#[doc="SMBus alert"]
  #[inline] pub fn set_smbalert<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Timeout or Tlow error"]
  #[inline] pub fn timeout(&self) -> bits::B1 {
     (((self.0 as u32) >> 14) & 0x1).into() // [14]
  }
#[doc="Timeout or Tlow error"]
  #[inline] pub fn set_timeout<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="PEC Error in reception"]
  #[inline] pub fn pecerr(&self) -> bits::B1 {
     (((self.0 as u32) >> 12) & 0x1).into() // [12]
  }
#[doc="PEC Error in reception"]
  #[inline] pub fn set_pecerr<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Overrun/Underrun"]
  #[inline] pub fn ovr(&self) -> bits::B1 {
     (((self.0 as u32) >> 11) & 0x1).into() // [11]
  }
#[doc="Overrun/Underrun"]
  #[inline] pub fn set_ovr<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Acknowledge failure"]
  #[inline] pub fn af(&self) -> bits::B1 {
     (((self.0 as u32) >> 10) & 0x1).into() // [10]
  }
#[doc="Acknowledge failure"]
  #[inline] pub fn set_af<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Arbitration lost (master mode)"]
  #[inline] pub fn arlo(&self) -> bits::B1 {
     (((self.0 as u32) >> 9) & 0x1).into() // [9]
  }
#[doc="Arbitration lost (master mode)"]
  #[inline] pub fn set_arlo<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Bus error"]
  #[inline] pub fn berr(&self) -> bits::B1 {
     (((self.0 as u32) >> 8) & 0x1).into() // [8]
  }
#[doc="Bus error"]
  #[inline] pub fn set_berr<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Data register empty (transmitters)"]
  #[inline] pub fn txe(&self) -> bits::B1 {
     (((self.0 as u32) >> 7) & 0x1).into() // [7]
  }
#[doc="Data register empty (transmitters)"]
  #[inline] pub fn set_txe<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Data register not empty (receivers)"]
  #[inline] pub fn rxne(&self) -> bits::B1 {
     (((self.0 as u32) >> 6) & 0x1).into() // [6]
  }
#[doc="Data register not empty (receivers)"]
  #[inline] pub fn set_rxne<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Stop detection (slave mode)"]
  #[inline] pub fn stopf(&self) -> bits::B1 {
     (((self.0 as u32) >> 4) & 0x1).into() // [4]
  }
#[doc="Stop detection (slave mode)"]
  #[inline] pub fn set_stopf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="10-bit header sent (Master mode)"]
  #[inline] pub fn add10(&self) -> bits::B1 {
     (((self.0 as u32) >> 3) & 0x1).into() // [3]
  }
#[doc="10-bit header sent (Master mode)"]
  #[inline] pub fn set_add10<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Byte transfer finished"]
  #[inline] pub fn btf(&self) -> bits::B1 {
     (((self.0 as u32) >> 2) & 0x1).into() // [2]
  }
#[doc="Byte transfer finished"]
  #[inline] pub fn set_btf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Address sent (master mode)/matched (slave mode)"]
  #[inline] pub fn addr(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="Address sent (master mode)/matched (slave mode)"]
  #[inline] pub fn set_addr<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Start bit (Master mode)"]
  #[inline] pub fn sb(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Start bit (Master mode)"]
  #[inline] pub fn set_sb<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Sr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.smbalert() != 0 { try!(write!(f, " smbalert"))}
      if self.timeout() != 0 { try!(write!(f, " timeout"))}
      if self.pecerr() != 0 { try!(write!(f, " pecerr"))}
      if self.ovr() != 0 { try!(write!(f, " ovr"))}
      if self.af() != 0 { try!(write!(f, " af"))}
      if self.arlo() != 0 { try!(write!(f, " arlo"))}
      if self.berr() != 0 { try!(write!(f, " berr"))}
      if self.txe() != 0 { try!(write!(f, " txe"))}
      if self.rxne() != 0 { try!(write!(f, " rxne"))}
      if self.stopf() != 0 { try!(write!(f, " stopf"))}
      if self.add10() != 0 { try!(write!(f, " add10"))}
      if self.btf() != 0 { try!(write!(f, " btf"))}
      if self.addr() != 0 { try!(write!(f, " addr"))}
      if self.sb() != 0 { try!(write!(f, " sb"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Status register 2"]
#[derive(PartialEq, Eq)]
pub struct Sr2(pub u32);
impl Sr2 {
#[doc="acket error checking register"]
  #[inline] pub fn pec(&self) -> bits::B8 {
     (((self.0 as u32) >> 8) & 0xff).into() // [15:8]
  }
#[doc="acket error checking register"]
  #[inline] pub fn set_pec<V: Into<bits::B8>>(mut self, value: V) -> Self {
     let value: bits::B8 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Dual flag (Slave mode)"]
  #[inline] pub fn dualf(&self) -> bits::B1 {
     (((self.0 as u32) >> 7) & 0x1).into() // [7]
  }
#[doc="Dual flag (Slave mode)"]
  #[inline] pub fn set_dualf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="SMBus host header (Slave mode)"]
  #[inline] pub fn smbhost(&self) -> bits::B1 {
     (((self.0 as u32) >> 6) & 0x1).into() // [6]
  }
#[doc="SMBus host header (Slave mode)"]
  #[inline] pub fn set_smbhost<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="SMBus device default address (Slave mode)"]
  #[inline] pub fn smbdefault(&self) -> bits::B1 {
     (((self.0 as u32) >> 5) & 0x1).into() // [5]
  }
#[doc="SMBus device default address (Slave mode)"]
  #[inline] pub fn set_smbdefault<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="General call address (Slave mode)"]
  #[inline] pub fn gencall(&self) -> bits::B1 {
     (((self.0 as u32) >> 4) & 0x1).into() // [4]
  }
#[doc="General call address (Slave mode)"]
  #[inline] pub fn set_gencall<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Transmitter/receiver"]
  #[inline] pub fn tra(&self) -> bits::B1 {
     (((self.0 as u32) >> 2) & 0x1).into() // [2]
  }
#[doc="Transmitter/receiver"]
  #[inline] pub fn set_tra<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Bus busy"]
  #[inline] pub fn busy(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="Bus busy"]
  #[inline] pub fn set_busy<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Master/slave"]
  #[inline] pub fn msl(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Master/slave"]
  #[inline] pub fn set_msl<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Sr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pec() != 0 { try!(write!(f, " pec=0x{:x}", self.pec()))}
      if self.dualf() != 0 { try!(write!(f, " dualf"))}
      if self.smbhost() != 0 { try!(write!(f, " smbhost"))}
      if self.smbdefault() != 0 { try!(write!(f, " smbdefault"))}
      if self.gencall() != 0 { try!(write!(f, " gencall"))}
      if self.tra() != 0 { try!(write!(f, " tra"))}
      if self.busy() != 0 { try!(write!(f, " busy"))}
      if self.msl() != 0 { try!(write!(f, " msl"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Clock control register"]
#[derive(PartialEq, Eq)]
pub struct Ccr(pub u32);
impl Ccr {
#[doc="I2C master mode selection"]
  #[inline] pub fn f_s(&self) -> bits::B1 {
     (((self.0 as u32) >> 15) & 0x1).into() // [15]
  }
#[doc="I2C master mode selection"]
  #[inline] pub fn set_f_s<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Fast mode duty cycle"]
  #[inline] pub fn duty(&self) -> bits::B1 {
     (((self.0 as u32) >> 14) & 0x1).into() // [14]
  }
#[doc="Fast mode duty cycle"]
  #[inline] pub fn set_duty<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Clock control register in Fast/Standard mode (Master mode)"]
  #[inline] pub fn ccr(&self) -> bits::B12 {
     (((self.0 as u32) >> 0) & 0xfff).into() // [11:0]
  }
#[doc="Clock control register in Fast/Standard mode (Master mode)"]
  #[inline] pub fn set_ccr<V: Into<bits::B12>>(mut self, value: V) -> Self {
     let value: bits::B12 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
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
      if self.f_s() != 0 { try!(write!(f, " f_s"))}
      if self.duty() != 0 { try!(write!(f, " duty"))}
      if self.ccr() != 0 { try!(write!(f, " ccr=0x{:x}", self.ccr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="TRISE register"]
#[derive(PartialEq, Eq)]
pub struct Trise(pub u32);
impl Trise {
#[doc="Maximum rise time in Fast/Standard mode (Master mode)"]
  #[inline] pub fn trise(&self) -> bits::B6 {
     (((self.0 as u32) >> 0) & 0x3f).into() // [5:0]
  }
#[doc="Maximum rise time in Fast/Standard mode (Master mode)"]
  #[inline] pub fn set_trise<V: Into<bits::B6>>(mut self, value: V) -> Self {
     let value: bits::B6 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Trise {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Trise {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.trise() != 0 { try!(write!(f, " trise=0x{:x}", self.trise()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="I2C FLTR register"]
#[derive(PartialEq, Eq)]
pub struct Fltr(pub u32);
impl Fltr {
#[doc="Digital noise filter"]
  #[inline] pub fn dnf(&self) -> bits::B4 {
     (((self.0 as u32) >> 0) & 0xf).into() // [3:0]
  }
#[doc="Digital noise filter"]
  #[inline] pub fn set_dnf<V: Into<bits::B4>>(mut self, value: V) -> Self {
     let value: bits::B4 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Analog noise filter OFF"]
  #[inline] pub fn anoff(&self) -> bits::B1 {
     (((self.0 as u32) >> 4) & 0x1).into() // [4]
  }
#[doc="Analog noise filter OFF"]
  #[inline] pub fn set_anoff<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

}
impl ::core::fmt::Display for Fltr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fltr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dnf() != 0 { try!(write!(f, " dnf=0x{:x}", self.dnf()))}
      if self.anoff() != 0 { try!(write!(f, " anoff"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
