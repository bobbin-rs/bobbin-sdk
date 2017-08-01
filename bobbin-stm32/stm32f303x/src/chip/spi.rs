pub const SPI1: Spi1 = Periph(0x40013000, Spi1Id {});
pub const SPI2: Spi2 = Periph(0x40003800, Spi2Id {});
pub const SPI3: Spi3 = Periph(0x40003c00, Spi3Id {});

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
  #[inline] pub fn dr_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0xc) as *const u16
  }
#[doc="Get the *mut pointer for the DR register."]
  #[inline] pub fn dr_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0xc) as *mut u16
  }
#[doc="Read the DR register."]
  #[inline] pub fn dr(&self) -> Dr { 
     unsafe {
        Dr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u16))
     }
  }
#[doc="Write the DR register."]
  #[inline] pub fn set_dr(&self, value: Dr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the DR register."]
  #[inline] pub fn with_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
     let tmp = self.dr();
     self.set_dr(f(tmp))
  }

#[doc="Get the *const pointer for the DR8 register."]
  #[inline] pub fn dr8_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xc) as *const u8
  }
#[doc="Get the *mut pointer for the DR8 register."]
  #[inline] pub fn dr8_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xc) as *mut u8
  }
#[doc="Read the DR8 register."]
  #[inline] pub fn dr8(&self) -> Dr8 { 
     unsafe {
        Dr8(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u8))
     }
  }
#[doc="Write the DR8 register."]
  #[inline] pub fn set_dr8(&self, value: Dr8) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the DR8 register."]
  #[inline] pub fn with_dr8<F: FnOnce(Dr8) -> Dr8>(&self, f: F) -> &Self {
     let tmp = self.dr8();
     self.set_dr8(f(tmp))
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
  #[inline] pub fn bidimode(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="Bidirectional data mode enable"]
  #[inline] pub fn set_bidimode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Output enable in bidirectional mode"]
  #[inline] pub fn bidioe(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="Output enable in bidirectional mode"]
  #[inline] pub fn set_bidioe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Hardware CRC calculation enable"]
  #[inline] pub fn crcen(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="Hardware CRC calculation enable"]
  #[inline] pub fn set_crcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="CRC transfer next"]
  #[inline] pub fn crcnext(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="CRC transfer next"]
  #[inline] pub fn set_crcnext(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Data frame format"]
  #[inline] pub fn dff(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="Data frame format"]
  #[inline] pub fn set_dff(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Receive only"]
  #[inline] pub fn rxonly(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="Receive only"]
  #[inline] pub fn set_rxonly(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Software slave management"]
  #[inline] pub fn ssm(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Software slave management"]
  #[inline] pub fn set_ssm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Internal slave select"]
  #[inline] pub fn ssi(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Internal slave select"]
  #[inline] pub fn set_ssi(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Frame format"]
  #[inline] pub fn lsbfirst(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Frame format"]
  #[inline] pub fn set_lsbfirst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="SPI enable"]
  #[inline] pub fn spe(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="SPI enable"]
  #[inline] pub fn set_spe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Baud rate control"]
  #[inline] pub fn br(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x7 // [5:3]
  }
#[doc="Baud rate control"]
  #[inline] pub fn set_br(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Master selection"]
  #[inline] pub fn mstr(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Master selection"]
  #[inline] pub fn set_mstr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Clock polarity"]
  #[inline] pub fn cpol(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Clock polarity"]
  #[inline] pub fn set_cpol(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Clock phase"]
  #[inline] pub fn cpha(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Clock phase"]
  #[inline] pub fn set_cpha(mut self, value: u32) -> Self {
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
#[doc="Rx buffer DMA enable"]
  #[inline] pub fn rxdmaen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Rx buffer DMA enable"]
  #[inline] pub fn set_rxdmaen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Tx buffer DMA enable"]
  #[inline] pub fn txdmaen(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Tx buffer DMA enable"]
  #[inline] pub fn set_txdmaen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="SS output enable"]
  #[inline] pub fn ssoe(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="SS output enable"]
  #[inline] pub fn set_ssoe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="NSS pulse management"]
  #[inline] pub fn nssp(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="NSS pulse management"]
  #[inline] pub fn set_nssp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Frame format"]
  #[inline] pub fn frf(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Frame format"]
  #[inline] pub fn set_frf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Error interrupt enable"]
  #[inline] pub fn errie(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Error interrupt enable"]
  #[inline] pub fn set_errie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="RX buffer not empty interrupt enable"]
  #[inline] pub fn rxneie(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="RX buffer not empty interrupt enable"]
  #[inline] pub fn set_rxneie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Tx buffer empty interrupt enable"]
  #[inline] pub fn txeie(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Tx buffer empty interrupt enable"]
  #[inline] pub fn set_txeie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Data size"]
  #[inline] pub fn ds(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
#[doc="Data size"]
  #[inline] pub fn set_ds(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="FIFO reception threshold"]
  #[inline] pub fn frxth(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="FIFO reception threshold"]
  #[inline] pub fn set_frxth(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Last DMA transfer for reception"]
  #[inline] pub fn ldma_rx(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="Last DMA transfer for reception"]
  #[inline] pub fn set_ldma_rx(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Last DMA transfer for transmission"]
  #[inline] pub fn ldma_tx(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="Last DMA transfer for transmission"]
  #[inline] pub fn set_ldma_tx(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
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
      if self.rxdmaen() != 0 { try!(write!(f, " rxdmaen"))}
      if self.txdmaen() != 0 { try!(write!(f, " txdmaen"))}
      if self.ssoe() != 0 { try!(write!(f, " ssoe"))}
      if self.nssp() != 0 { try!(write!(f, " nssp"))}
      if self.frf() != 0 { try!(write!(f, " frf"))}
      if self.errie() != 0 { try!(write!(f, " errie"))}
      if self.rxneie() != 0 { try!(write!(f, " rxneie"))}
      if self.txeie() != 0 { try!(write!(f, " txeie"))}
      if self.ds() != 0 { try!(write!(f, " ds=0x{:x}", self.ds()))}
      if self.frxth() != 0 { try!(write!(f, " frxth"))}
      if self.ldma_rx() != 0 { try!(write!(f, " ldma_rx"))}
      if self.ldma_tx() != 0 { try!(write!(f, " ldma_tx"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="status register"]
#[derive(PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
#[doc="Receive buffer not empty"]
  #[inline] pub fn rxne(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Receive buffer not empty"]
  #[inline] pub fn set_rxne(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Transmit buffer empty"]
  #[inline] pub fn txe(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Transmit buffer empty"]
  #[inline] pub fn set_txe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Channel side"]
  #[inline] pub fn chside(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Channel side"]
  #[inline] pub fn set_chside(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Underrun flag"]
  #[inline] pub fn udr(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Underrun flag"]
  #[inline] pub fn set_udr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="CRC error flag"]
  #[inline] pub fn crcerr(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="CRC error flag"]
  #[inline] pub fn set_crcerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Mode fault"]
  #[inline] pub fn modf(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Mode fault"]
  #[inline] pub fn set_modf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Overrun flag"]
  #[inline] pub fn ovr(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Overrun flag"]
  #[inline] pub fn set_ovr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Busy flag"]
  #[inline] pub fn bsy(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Busy flag"]
  #[inline] pub fn set_bsy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="TI frame format error"]
  #[inline] pub fn tifrfe(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="TI frame format error"]
  #[inline] pub fn set_tifrfe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="FIFO reception level"]
  #[inline] pub fn frlvl(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x3 // [10:9]
  }
#[doc="FIFO reception level"]
  #[inline] pub fn set_frlvl(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="FIFO transmission level"]
  #[inline] pub fn ftlvl(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x3 // [12:11]
  }
#[doc="FIFO transmission level"]
  #[inline] pub fn set_ftlvl(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 11);
     self.0 |= value << 11;
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
      if self.rxne() != 0 { try!(write!(f, " rxne"))}
      if self.txe() != 0 { try!(write!(f, " txe"))}
      if self.chside() != 0 { try!(write!(f, " chside"))}
      if self.udr() != 0 { try!(write!(f, " udr"))}
      if self.crcerr() != 0 { try!(write!(f, " crcerr"))}
      if self.modf() != 0 { try!(write!(f, " modf"))}
      if self.ovr() != 0 { try!(write!(f, " ovr"))}
      if self.bsy() != 0 { try!(write!(f, " bsy"))}
      if self.tifrfe() != 0 { try!(write!(f, " tifrfe"))}
      if self.frlvl() != 0 { try!(write!(f, " frlvl=0x{:x}", self.frlvl()))}
      if self.ftlvl() != 0 { try!(write!(f, " ftlvl=0x{:x}", self.ftlvl()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="data register"]
#[derive(PartialEq, Eq)]
pub struct Dr(pub u16);
impl Dr {
#[doc="Data register"]
  #[inline] pub fn dr(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
#[doc="Data register"]
  #[inline] pub fn set_dr(mut self, value: u16) -> Self {
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
#[doc="data register"]
#[derive(PartialEq, Eq)]
pub struct Dr8(pub u8);
impl Dr8 {
#[doc="Data register"]
  #[inline] pub fn dr8(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
#[doc="Data register"]
  #[inline] pub fn set_dr8(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dr8 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dr8 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dr8() != 0 { try!(write!(f, " dr8=0x{:x}", self.dr8()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="CRC polynomial register"]
#[derive(PartialEq, Eq)]
pub struct Crcpr(pub u32);
impl Crcpr {
#[doc="CRC polynomial register"]
  #[inline] pub fn crcpoly(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="CRC polynomial register"]
  #[inline] pub fn set_crcpoly(mut self, value: u32) -> Self {
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
  #[inline] pub fn rxcrc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Rx CRC register"]
  #[inline] pub fn set_rxcrc(mut self, value: u32) -> Self {
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
  #[inline] pub fn txcrc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Tx CRC register"]
  #[inline] pub fn set_txcrc(mut self, value: u32) -> Self {
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
  #[inline] pub fn i2smod(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="I2S mode selection"]
  #[inline] pub fn set_i2smod(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="I2S Enable"]
  #[inline] pub fn i2se(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="I2S Enable"]
  #[inline] pub fn set_i2se(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="I2S configuration mode"]
  #[inline] pub fn i2scfg(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
#[doc="I2S configuration mode"]
  #[inline] pub fn set_i2scfg(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="PCM frame synchronization"]
  #[inline] pub fn pcmsync(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="PCM frame synchronization"]
  #[inline] pub fn set_pcmsync(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="I2S standard selection"]
  #[inline] pub fn i2sstd(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
#[doc="I2S standard selection"]
  #[inline] pub fn set_i2sstd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Steady state clock polarity"]
  #[inline] pub fn ckpol(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Steady state clock polarity"]
  #[inline] pub fn set_ckpol(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Data length to be transferred"]
  #[inline] pub fn datlen(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x3 // [2:1]
  }
#[doc="Data length to be transferred"]
  #[inline] pub fn set_datlen(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Channel length (number of bits per audio channel)"]
  #[inline] pub fn chlen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Channel length (number of bits per audio channel)"]
  #[inline] pub fn set_chlen(mut self, value: u32) -> Self {
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
  #[inline] pub fn mckoe(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Master clock output enable"]
  #[inline] pub fn set_mckoe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Odd factor for the prescaler"]
  #[inline] pub fn odd(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Odd factor for the prescaler"]
  #[inline] pub fn set_odd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="I2S Linear prescaler"]
  #[inline] pub fn i2sdiv(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
#[doc="I2S Linear prescaler"]
  #[inline] pub fn set_i2sdiv(mut self, value: u32) -> Self {
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
