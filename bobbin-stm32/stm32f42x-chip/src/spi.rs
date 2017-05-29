pub const SPI1: Spi = Spi(0x40013000);
pub const SPI2: Spi = Spi(0x40003800);
pub const SPI3: Spi = Spi(0x40003c00);
pub const I2S2EXT: Spi = Spi(0x40003400);
pub const I2S3EXT: Spi = Spi(0x40004000);
pub const SPI4: Spi = Spi(0x40013400);
pub const SPI5: Spi = Spi(0x40015000);
pub const SPI6: Spi = Spi(0x40015400);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Spi(pub u32);

impl Spi {
  pub unsafe fn cr1(&self) -> Cr1 { 
     Cr1(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
  }
  pub unsafe fn set_cr1(&mut self, value: Cr1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
  }
  pub unsafe fn with_cr1<F: FnOnce(Cr1) -> Cr1>(&mut self, f: F) {
     let tmp = self.cr1();
     self.set_cr1(f(tmp))
  }

  pub unsafe fn cr2(&self) -> Cr2 { 
     Cr2(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
  }
  pub unsafe fn set_cr2(&mut self, value: Cr2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
  }
  pub unsafe fn with_cr2<F: FnOnce(Cr2) -> Cr2>(&mut self, f: F) {
     let tmp = self.cr2();
     self.set_cr2(f(tmp))
  }

  pub unsafe fn sr(&self) -> Sr { 
     Sr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
  }
  pub unsafe fn set_sr(&mut self, value: Sr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
  }
  pub unsafe fn with_sr<F: FnOnce(Sr) -> Sr>(&mut self, f: F) {
     let tmp = self.sr();
     self.set_sr(f(tmp))
  }

  pub unsafe fn dr(&self) -> Dr { 
     Dr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
  }
  pub unsafe fn set_dr(&mut self, value: Dr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
  }
  pub unsafe fn with_dr<F: FnOnce(Dr) -> Dr>(&mut self, f: F) {
     let tmp = self.dr();
     self.set_dr(f(tmp))
  }

  pub unsafe fn crcpr(&self) -> Crcpr { 
     Crcpr(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
  }
  pub unsafe fn set_crcpr(&mut self, value: Crcpr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
  }
  pub unsafe fn with_crcpr<F: FnOnce(Crcpr) -> Crcpr>(&mut self, f: F) {
     let tmp = self.crcpr();
     self.set_crcpr(f(tmp))
  }

  pub unsafe fn rxcrcr(&self) -> Rxcrcr { 
     Rxcrcr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
  }

  pub unsafe fn txcrcr(&self) -> Txcrcr { 
     Txcrcr(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
  }

  pub unsafe fn i2scfgr(&self) -> I2scfgr { 
     I2scfgr(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
  }
  pub unsafe fn set_i2scfgr(&mut self, value: I2scfgr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
  }
  pub unsafe fn with_i2scfgr<F: FnOnce(I2scfgr) -> I2scfgr>(&mut self, f: F) {
     let tmp = self.i2scfgr();
     self.set_i2scfgr(f(tmp))
  }

  pub unsafe fn i2spr(&self) -> I2spr { 
     I2spr(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
  }
  pub unsafe fn set_i2spr(&mut self, value: I2spr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
  }
  pub unsafe fn with_i2spr<F: FnOnce(I2spr) -> I2spr>(&mut self, f: F) {
     let tmp = self.i2spr();
     self.set_i2spr(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Cr1(pub u32);

impl Cr1 {
  pub fn bidimode(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_bidimode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn bidioe(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_bidioe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn crcen(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_crcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn crcnext(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_crcnext(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn dff(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_dff(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn rxonly(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_rxonly(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn ssm(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_ssm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn ssi(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_ssi(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn lsbfirst(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_lsbfirst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn spe(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_spe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn br(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x7 // [5:3]
  }
  pub fn set_br(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn mstr(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_mstr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn cpol(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_cpol(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn cpha(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_cpha(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Cr2(pub u32);

impl Cr2 {
  pub fn txeie(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_txeie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn rxneie(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_rxneie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn errie(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_errie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn frf(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_frf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn ssoe(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_ssoe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn txdmaen(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_txdmaen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn rxdmaen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_rxdmaen(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Sr(pub u32);

impl Sr {
  pub fn tifrfe(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_tifrfe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn bsy(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_bsy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn ovr(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_ovr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn modf(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_modf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn crcerr(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_crcerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn udr(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_udr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn chside(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_chside(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn txe(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_txe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn rxne(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_rxne(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Dr(pub u32);

impl Dr {
  pub fn dr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_dr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Crcpr(pub u32);

impl Crcpr {
  pub fn crcpoly(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_crcpoly(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Rxcrcr(pub u32);

impl Rxcrcr {
  pub fn rxcrc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_rxcrc(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Txcrcr(pub u32);

impl Txcrcr {
  pub fn txcrc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_txcrc(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct I2scfgr(pub u32);

impl I2scfgr {
  pub fn i2smod(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_i2smod(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn i2se(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_i2se(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn i2scfg(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  pub fn set_i2scfg(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn pcmsync(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_pcmsync(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn i2sstd(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  pub fn set_i2sstd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn ckpol(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_ckpol(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn datlen(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x3 // [2:1]
  }
  pub fn set_datlen(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn chlen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_chlen(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct I2spr(pub u32);

impl I2spr {
  pub fn mckoe(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_mckoe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn odd(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_odd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn i2sdiv(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_i2sdiv(mut self, value: u32) -> Self {
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

