#[allow(unused_imports)] use bobbin_common::bits;
pub const SPI1: Spi1 = Periph(0x40013000, Spi1Id {});
pub const SPI2: Spi2 = Periph(0x40003800, Spi2Id {});
pub const SPI3: Spi3 = Periph(0x40003c00, Spi3Id {});
pub const I2S2EXT: I2s2ext = Periph(0x40003400, I2s2extId {});
pub const I2S3EXT: I2s3ext = Periph(0x40004000, I2s3extId {});
pub const SPI4: Spi4 = Periph(0x40013400, Spi4Id {});
pub const SPI5: Spi5 = Periph(0x40015000, Spi5Id {});
pub const SPI6: Spi6 = Periph(0x40015400, Spi6Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="SPI Peripheral"]
pub struct Periph<T>(pub u32, pub T); 

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Spi1Id {}
pub type Spi1 = Periph<Spi1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Spi2Id {}
pub type Spi2 = Periph<Spi2Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Spi3Id {}
pub type Spi3 = Periph<Spi3Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct I2s2extId {}
pub type I2s2ext = Periph<I2s2extId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct I2s3extId {}
pub type I2s3ext = Periph<I2s3extId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Spi4Id {}
pub type Spi4 = Periph<Spi4Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Spi5Id {}
pub type Spi5 = Periph<Spi5Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Spi6Id {}
pub type Spi6 = Periph<Spi6Id>;










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

#[doc="Get the *const pointer for the SR register."]
  #[inline] pub fn sr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the SR register."]
  #[inline] pub fn sr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the SR register."]
  #[inline] pub fn sr(&self) -> Sr { 
     unsafe {
        Sr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the SR register."]
  #[inline] pub fn set_sr(&self, value: Sr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SR register."]
  #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
     let tmp = self.sr();
     self.set_sr(f(tmp))
  }

#[doc="Get the *const pointer for the DR register."]
  #[inline] pub fn dr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the DR register."]
  #[inline] pub fn dr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the DR register."]
  #[inline] pub fn dr(&self) -> Dr { 
     unsafe {
        Dr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the DR register."]
  #[inline] pub fn set_dr(&self, value: Dr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DR register."]
  #[inline] pub fn with_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
     let tmp = self.dr();
     self.set_dr(f(tmp))
  }

#[doc="Get the *const pointer for the CRCPR register."]
  #[inline] pub fn crcpr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the CRCPR register."]
  #[inline] pub fn crcpr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the CRCPR register."]
  #[inline] pub fn crcpr(&self) -> Crcpr { 
     unsafe {
        Crcpr(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the CRCPR register."]
  #[inline] pub fn set_crcpr(&self, value: Crcpr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CRCPR register."]
  #[inline] pub fn with_crcpr<F: FnOnce(Crcpr) -> Crcpr>(&self, f: F) -> &Self {
     let tmp = self.crcpr();
     self.set_crcpr(f(tmp))
  }

#[doc="Get the *const pointer for the RXCRCR register."]
  #[inline] pub fn rxcrcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the RXCRCR register."]
  #[inline] pub fn rxcrcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Read the RXCRCR register."]
  #[inline] pub fn rxcrcr(&self) -> Rxcrcr { 
     unsafe {
        Rxcrcr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }

#[doc="Get the *const pointer for the TXCRCR register."]
  #[inline] pub fn txcrcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
#[doc="Get the *mut pointer for the TXCRCR register."]
  #[inline] pub fn txcrcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
#[doc="Read the TXCRCR register."]
  #[inline] pub fn txcrcr(&self) -> Txcrcr { 
     unsafe {
        Txcrcr(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }

#[doc="Get the *const pointer for the I2SCFGR register."]
  #[inline] pub fn i2scfgr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the I2SCFGR register."]
  #[inline] pub fn i2scfgr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Read the I2SCFGR register."]
  #[inline] pub fn i2scfgr(&self) -> I2scfgr { 
     unsafe {
        I2scfgr(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
#[doc="Write the I2SCFGR register."]
  #[inline] pub fn set_i2scfgr(&self, value: I2scfgr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the I2SCFGR register."]
  #[inline] pub fn with_i2scfgr<F: FnOnce(I2scfgr) -> I2scfgr>(&self, f: F) -> &Self {
     let tmp = self.i2scfgr();
     self.set_i2scfgr(f(tmp))
  }

#[doc="Get the *const pointer for the I2SPR register."]
  #[inline] pub fn i2spr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
#[doc="Get the *mut pointer for the I2SPR register."]
  #[inline] pub fn i2spr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
#[doc="Read the I2SPR register."]
  #[inline] pub fn i2spr(&self) -> I2spr { 
     unsafe {
        I2spr(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
#[doc="Write the I2SPR register."]
  #[inline] pub fn set_i2spr(&self, value: I2spr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the I2SPR register."]
  #[inline] pub fn with_i2spr<F: FnOnce(I2spr) -> I2spr>(&self, f: F) -> &Self {
     let tmp = self.i2spr();
     self.set_i2spr(f(tmp))
  }

}

#[doc="control register 1"]
#[derive(PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
#[doc="Bidirectional data mode enable"]
  #[inline] pub fn bidimode(&self) -> bits::B1 {
     (((self.0 as u32) >> 15) & 0x1).into() // [15]
  }
#[doc="Bidirectional data mode enable"]
  #[inline] pub fn set_bidimode<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Output enable in bidirectional mode"]
  #[inline] pub fn bidioe(&self) -> bits::B1 {
     (((self.0 as u32) >> 14) & 0x1).into() // [14]
  }
#[doc="Output enable in bidirectional mode"]
  #[inline] pub fn set_bidioe<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Hardware CRC calculation enable"]
  #[inline] pub fn crcen(&self) -> bits::B1 {
     (((self.0 as u32) >> 13) & 0x1).into() // [13]
  }
#[doc="Hardware CRC calculation enable"]
  #[inline] pub fn set_crcen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="CRC transfer next"]
  #[inline] pub fn crcnext(&self) -> bits::B1 {
     (((self.0 as u32) >> 12) & 0x1).into() // [12]
  }
#[doc="CRC transfer next"]
  #[inline] pub fn set_crcnext<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Data frame format"]
  #[inline] pub fn dff(&self) -> bits::B1 {
     (((self.0 as u32) >> 11) & 0x1).into() // [11]
  }
#[doc="Data frame format"]
  #[inline] pub fn set_dff<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Receive only"]
  #[inline] pub fn rxonly(&self) -> bits::B1 {
     (((self.0 as u32) >> 10) & 0x1).into() // [10]
  }
#[doc="Receive only"]
  #[inline] pub fn set_rxonly<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Software slave management"]
  #[inline] pub fn ssm(&self) -> bits::B1 {
     (((self.0 as u32) >> 9) & 0x1).into() // [9]
  }
#[doc="Software slave management"]
  #[inline] pub fn set_ssm<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Internal slave select"]
  #[inline] pub fn ssi(&self) -> bits::B1 {
     (((self.0 as u32) >> 8) & 0x1).into() // [8]
  }
#[doc="Internal slave select"]
  #[inline] pub fn set_ssi<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Frame format"]
  #[inline] pub fn lsbfirst(&self) -> bits::B1 {
     (((self.0 as u32) >> 7) & 0x1).into() // [7]
  }
#[doc="Frame format"]
  #[inline] pub fn set_lsbfirst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="SPI enable"]
  #[inline] pub fn spe(&self) -> bits::B1 {
     (((self.0 as u32) >> 6) & 0x1).into() // [6]
  }
#[doc="SPI enable"]
  #[inline] pub fn set_spe<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Baud rate control"]
  #[inline] pub fn br(&self) -> bits::B3 {
     (((self.0 as u32) >> 3) & 0x7).into() // [5:3]
  }
#[doc="Baud rate control"]
  #[inline] pub fn set_br<V: Into<bits::B3>>(mut self, value: V) -> Self {
     let value: bits::B3 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Master selection"]
  #[inline] pub fn mstr(&self) -> bits::B1 {
     (((self.0 as u32) >> 2) & 0x1).into() // [2]
  }
#[doc="Master selection"]
  #[inline] pub fn set_mstr<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Clock polarity"]
  #[inline] pub fn cpol(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="Clock polarity"]
  #[inline] pub fn set_cpol<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Clock phase"]
  #[inline] pub fn cpha(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Clock phase"]
  #[inline] pub fn set_cpha<V: Into<bits::B1>>(mut self, value: V) -> Self {
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
      if self.bidimode() != 0 { try!(write!(f, " bidimode"))}
      if self.bidioe() != 0 { try!(write!(f, " bidioe"))}
      if self.crcen() != 0 { try!(write!(f, " crcen"))}
      if self.crcnext() != 0 { try!(write!(f, " crcnext"))}
      if self.dff() != 0 { try!(write!(f, " dff"))}
      if self.rxonly() != 0 { try!(write!(f, " rxonly"))}
      if self.ssm() != 0 { try!(write!(f, " ssm"))}
      if self.ssi() != 0 { try!(write!(f, " ssi"))}
      if self.lsbfirst() != 0 { try!(write!(f, " lsbfirst"))}
      if self.spe() != 0 { try!(write!(f, " spe"))}
      if self.br() != 0 { try!(write!(f, " br=0x{:x}", self.br()))}
      if self.mstr() != 0 { try!(write!(f, " mstr"))}
      if self.cpol() != 0 { try!(write!(f, " cpol"))}
      if self.cpha() != 0 { try!(write!(f, " cpha"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="control register 2"]
#[derive(PartialEq, Eq)]
pub struct Cr2(pub u32);
impl Cr2 {
#[doc="Tx buffer empty interrupt enable"]
  #[inline] pub fn txeie(&self) -> bits::B1 {
     (((self.0 as u32) >> 7) & 0x1).into() // [7]
  }
#[doc="Tx buffer empty interrupt enable"]
  #[inline] pub fn set_txeie<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="RX buffer not empty interrupt enable"]
  #[inline] pub fn rxneie(&self) -> bits::B1 {
     (((self.0 as u32) >> 6) & 0x1).into() // [6]
  }
#[doc="RX buffer not empty interrupt enable"]
  #[inline] pub fn set_rxneie<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Error interrupt enable"]
  #[inline] pub fn errie(&self) -> bits::B1 {
     (((self.0 as u32) >> 5) & 0x1).into() // [5]
  }
#[doc="Error interrupt enable"]
  #[inline] pub fn set_errie<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Frame format"]
  #[inline] pub fn frf(&self) -> bits::B1 {
     (((self.0 as u32) >> 4) & 0x1).into() // [4]
  }
#[doc="Frame format"]
  #[inline] pub fn set_frf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="SS output enable"]
  #[inline] pub fn ssoe(&self) -> bits::B1 {
     (((self.0 as u32) >> 2) & 0x1).into() // [2]
  }
#[doc="SS output enable"]
  #[inline] pub fn set_ssoe<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Tx buffer DMA enable"]
  #[inline] pub fn txdmaen(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="Tx buffer DMA enable"]
  #[inline] pub fn set_txdmaen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Rx buffer DMA enable"]
  #[inline] pub fn rxdmaen(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Rx buffer DMA enable"]
  #[inline] pub fn set_rxdmaen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
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
      if self.txeie() != 0 { try!(write!(f, " txeie"))}
      if self.rxneie() != 0 { try!(write!(f, " rxneie"))}
      if self.errie() != 0 { try!(write!(f, " errie"))}
      if self.frf() != 0 { try!(write!(f, " frf"))}
      if self.ssoe() != 0 { try!(write!(f, " ssoe"))}
      if self.txdmaen() != 0 { try!(write!(f, " txdmaen"))}
      if self.rxdmaen() != 0 { try!(write!(f, " rxdmaen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="status register"]
#[derive(PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
#[doc="TI frame format error"]
  #[inline] pub fn tifrfe(&self) -> bits::B1 {
     (((self.0 as u32) >> 8) & 0x1).into() // [8]
  }
#[doc="TI frame format error"]
  #[inline] pub fn set_tifrfe<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Busy flag"]
  #[inline] pub fn bsy(&self) -> bits::B1 {
     (((self.0 as u32) >> 7) & 0x1).into() // [7]
  }
#[doc="Busy flag"]
  #[inline] pub fn set_bsy<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Overrun flag"]
  #[inline] pub fn ovr(&self) -> bits::B1 {
     (((self.0 as u32) >> 6) & 0x1).into() // [6]
  }
#[doc="Overrun flag"]
  #[inline] pub fn set_ovr<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Mode fault"]
  #[inline] pub fn modf(&self) -> bits::B1 {
     (((self.0 as u32) >> 5) & 0x1).into() // [5]
  }
#[doc="Mode fault"]
  #[inline] pub fn set_modf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="CRC error flag"]
  #[inline] pub fn crcerr(&self) -> bits::B1 {
     (((self.0 as u32) >> 4) & 0x1).into() // [4]
  }
#[doc="CRC error flag"]
  #[inline] pub fn set_crcerr<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Underrun flag"]
  #[inline] pub fn udr(&self) -> bits::B1 {
     (((self.0 as u32) >> 3) & 0x1).into() // [3]
  }
#[doc="Underrun flag"]
  #[inline] pub fn set_udr<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Channel side"]
  #[inline] pub fn chside(&self) -> bits::B1 {
     (((self.0 as u32) >> 2) & 0x1).into() // [2]
  }
#[doc="Channel side"]
  #[inline] pub fn set_chside<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Transmit buffer empty"]
  #[inline] pub fn txe(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="Transmit buffer empty"]
  #[inline] pub fn set_txe<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Receive buffer not empty"]
  #[inline] pub fn rxne(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Receive buffer not empty"]
  #[inline] pub fn set_rxne<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Sr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tifrfe() != 0 { try!(write!(f, " tifrfe"))}
      if self.bsy() != 0 { try!(write!(f, " bsy"))}
      if self.ovr() != 0 { try!(write!(f, " ovr"))}
      if self.modf() != 0 { try!(write!(f, " modf"))}
      if self.crcerr() != 0 { try!(write!(f, " crcerr"))}
      if self.udr() != 0 { try!(write!(f, " udr"))}
      if self.chside() != 0 { try!(write!(f, " chside"))}
      if self.txe() != 0 { try!(write!(f, " txe"))}
      if self.rxne() != 0 { try!(write!(f, " rxne"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="data register"]
#[derive(PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
#[doc="Data register"]
  #[inline] pub fn dr(&self) -> bits::B16 {
     (((self.0 as u32) >> 0) & 0xffff).into() // [15:0]
  }
#[doc="Data register"]
  #[inline] pub fn set_dr<V: Into<bits::B16>>(mut self, value: V) -> Self {
     let value: bits::B16 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
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
#[doc="CRC polynomial register"]
#[derive(PartialEq, Eq)]
pub struct Crcpr(pub u32);
impl Crcpr {
#[doc="CRC polynomial register"]
  #[inline] pub fn crcpoly(&self) -> bits::B16 {
     (((self.0 as u32) >> 0) & 0xffff).into() // [15:0]
  }
#[doc="CRC polynomial register"]
  #[inline] pub fn set_crcpoly<V: Into<bits::B16>>(mut self, value: V) -> Self {
     let value: bits::B16 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Crcpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Crcpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.crcpoly() != 0 { try!(write!(f, " crcpoly=0x{:x}", self.crcpoly()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="RX CRC register"]
#[derive(PartialEq, Eq)]
pub struct Rxcrcr(pub u32);
impl Rxcrcr {
#[doc="Rx CRC register"]
  #[inline] pub fn rxcrc(&self) -> bits::B16 {
     (((self.0 as u32) >> 0) & 0xffff).into() // [15:0]
  }
#[doc="Rx CRC register"]
  #[inline] pub fn set_rxcrc<V: Into<bits::B16>>(mut self, value: V) -> Self {
     let value: bits::B16 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Rxcrcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rxcrcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rxcrc() != 0 { try!(write!(f, " rxcrc=0x{:x}", self.rxcrc()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="TX CRC register"]
#[derive(PartialEq, Eq)]
pub struct Txcrcr(pub u32);
impl Txcrcr {
#[doc="Tx CRC register"]
  #[inline] pub fn txcrc(&self) -> bits::B16 {
     (((self.0 as u32) >> 0) & 0xffff).into() // [15:0]
  }
#[doc="Tx CRC register"]
  #[inline] pub fn set_txcrc<V: Into<bits::B16>>(mut self, value: V) -> Self {
     let value: bits::B16 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Txcrcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Txcrcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txcrc() != 0 { try!(write!(f, " txcrc=0x{:x}", self.txcrc()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="I2S configuration register"]
#[derive(PartialEq, Eq)]
pub struct I2scfgr(pub u32);
impl I2scfgr {
#[doc="I2S mode selection"]
  #[inline] pub fn i2smod(&self) -> bits::B1 {
     (((self.0 as u32) >> 11) & 0x1).into() // [11]
  }
#[doc="I2S mode selection"]
  #[inline] pub fn set_i2smod<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="I2S Enable"]
  #[inline] pub fn i2se(&self) -> bits::B1 {
     (((self.0 as u32) >> 10) & 0x1).into() // [10]
  }
#[doc="I2S Enable"]
  #[inline] pub fn set_i2se<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="I2S configuration mode"]
  #[inline] pub fn i2scfg(&self) -> bits::B2 {
     (((self.0 as u32) >> 8) & 0x3).into() // [9:8]
  }
#[doc="I2S configuration mode"]
  #[inline] pub fn set_i2scfg<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="PCM frame synchronization"]
  #[inline] pub fn pcmsync(&self) -> bits::B1 {
     (((self.0 as u32) >> 7) & 0x1).into() // [7]
  }
#[doc="PCM frame synchronization"]
  #[inline] pub fn set_pcmsync<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="I2S standard selection"]
  #[inline] pub fn i2sstd(&self) -> bits::B2 {
     (((self.0 as u32) >> 4) & 0x3).into() // [5:4]
  }
#[doc="I2S standard selection"]
  #[inline] pub fn set_i2sstd<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Steady state clock polarity"]
  #[inline] pub fn ckpol(&self) -> bits::B1 {
     (((self.0 as u32) >> 3) & 0x1).into() // [3]
  }
#[doc="Steady state clock polarity"]
  #[inline] pub fn set_ckpol<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Data length to be transferred"]
  #[inline] pub fn datlen(&self) -> bits::B2 {
     (((self.0 as u32) >> 1) & 0x3).into() // [2:1]
  }
#[doc="Data length to be transferred"]
  #[inline] pub fn set_datlen<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Channel length (number of bits per audio channel)"]
  #[inline] pub fn chlen(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Channel length (number of bits per audio channel)"]
  #[inline] pub fn set_chlen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for I2scfgr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for I2scfgr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.i2smod() != 0 { try!(write!(f, " i2smod"))}
      if self.i2se() != 0 { try!(write!(f, " i2se"))}
      if self.i2scfg() != 0 { try!(write!(f, " i2scfg=0x{:x}", self.i2scfg()))}
      if self.pcmsync() != 0 { try!(write!(f, " pcmsync"))}
      if self.i2sstd() != 0 { try!(write!(f, " i2sstd=0x{:x}", self.i2sstd()))}
      if self.ckpol() != 0 { try!(write!(f, " ckpol"))}
      if self.datlen() != 0 { try!(write!(f, " datlen=0x{:x}", self.datlen()))}
      if self.chlen() != 0 { try!(write!(f, " chlen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="I2S prescaler register"]
#[derive(PartialEq, Eq)]
pub struct I2spr(pub u32);
impl I2spr {
#[doc="Master clock output enable"]
  #[inline] pub fn mckoe(&self) -> bits::B1 {
     (((self.0 as u32) >> 9) & 0x1).into() // [9]
  }
#[doc="Master clock output enable"]
  #[inline] pub fn set_mckoe<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Odd factor for the prescaler"]
  #[inline] pub fn odd(&self) -> bits::B1 {
     (((self.0 as u32) >> 8) & 0x1).into() // [8]
  }
#[doc="Odd factor for the prescaler"]
  #[inline] pub fn set_odd<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="I2S Linear prescaler"]
  #[inline] pub fn i2sdiv(&self) -> bits::B8 {
     (((self.0 as u32) >> 0) & 0xff).into() // [7:0]
  }
#[doc="I2S Linear prescaler"]
  #[inline] pub fn set_i2sdiv<V: Into<bits::B8>>(mut self, value: V) -> Self {
     let value: bits::B8 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for I2spr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for I2spr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mckoe() != 0 { try!(write!(f, " mckoe"))}
      if self.odd() != 0 { try!(write!(f, " odd"))}
      if self.i2sdiv() != 0 { try!(write!(f, " i2sdiv=0x{:x}", self.i2sdiv()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
