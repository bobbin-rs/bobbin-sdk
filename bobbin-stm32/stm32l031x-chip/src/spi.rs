pub const SPI1: Spi1 = Periph(0x40013000, Spi1Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Periph<T>(pub u32, pub T); 

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Spi1Id {}
pub type Spi1 = Periph<Spi1Id>;

impl super::sig::Signal<super::sig::Spi1Nss> for Spi1 {}
impl super::sig::SignalNss<super::sig::Spi1Nss> for Spi1 {}
impl super::sig::Signal<super::sig::Spi1Miso> for Spi1 {}
impl super::sig::SignalMiso<super::sig::Spi1Miso> for Spi1 {}
impl super::sig::Signal<super::sig::Spi1Mosi> for Spi1 {}
impl super::sig::SignalMosi<super::sig::Spi1Mosi> for Spi1 {}
impl super::sig::Signal<super::sig::Spi1Sck> for Spi1 {}
impl super::sig::SignalSck<super::sig::Spi1Sck> for Spi1 {}


impl<T> Periph<T> {
  #[inline] pub fn cr1_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x0) as *const u16
  }
  #[inline] pub fn cr1_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x0) as *mut u16
  }
  #[inline] pub fn cr1(&self) -> Cr1 { 
     unsafe {
        Cr1(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u16))
     }
  }
  #[inline] pub fn set_cr1(&self, value: Cr1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u16, value.0);
     }
     self
  }
  #[inline] pub fn with_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
     let tmp = self.cr1();
     self.set_cr1(f(tmp))
  }

  #[inline] pub fn cr2_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x4) as *const u16
  }
  #[inline] pub fn cr2_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x4) as *mut u16
  }
  #[inline] pub fn cr2(&self) -> Cr2 { 
     unsafe {
        Cr2(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u16))
     }
  }
  #[inline] pub fn set_cr2(&self, value: Cr2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u16, value.0);
     }
     self
  }
  #[inline] pub fn with_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
     let tmp = self.cr2();
     self.set_cr2(f(tmp))
  }

  #[inline] pub fn sr_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x8) as *const u16
  }
  #[inline] pub fn sr_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x8) as *mut u16
  }
  #[inline] pub fn sr(&self) -> Sr { 
     unsafe {
        Sr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u16))
     }
  }
  #[inline] pub fn set_sr(&self, value: Sr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u16, value.0);
     }
     self
  }
  #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
     let tmp = self.sr();
     self.set_sr(f(tmp))
  }

  #[inline] pub fn dr_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0xc) as *const u16
  }
  #[inline] pub fn dr_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0xc) as *mut u16
  }
  #[inline] pub fn dr(&self) -> Dr { 
     unsafe {
        Dr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u16))
     }
  }
  #[inline] pub fn set_dr(&self, value: Dr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u16, value.0);
     }
     self
  }
  #[inline] pub fn with_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
     let tmp = self.dr();
     self.set_dr(f(tmp))
  }

  #[inline] pub fn crcpr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
  #[inline] pub fn crcpr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
  #[inline] pub fn crcpr(&self) -> Crcpr { 
     unsafe {
        Crcpr(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
  #[inline] pub fn set_crcpr(&self, value: Crcpr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_crcpr<F: FnOnce(Crcpr) -> Crcpr>(&self, f: F) -> &Self {
     let tmp = self.crcpr();
     self.set_crcpr(f(tmp))
  }

  #[inline] pub fn rxcrcr_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x14) as *const u16
  }
  #[inline] pub fn rxcrcr_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x14) as *mut u16
  }
  #[inline] pub fn rxcrcr(&self) -> Rxcrcr { 
     unsafe {
        Rxcrcr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u16))
     }
  }

  #[inline] pub fn txcrcr_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x18) as *const u16
  }
  #[inline] pub fn txcrcr_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x18) as *mut u16
  }
  #[inline] pub fn txcrcr(&self) -> Txcrcr { 
     unsafe {
        Txcrcr(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u16))
     }
  }

  #[inline] pub fn i2scfgr_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x1c) as *const u16
  }
  #[inline] pub fn i2scfgr_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x1c) as *mut u16
  }
  #[inline] pub fn i2scfgr(&self) -> I2scfgr { 
     unsafe {
        I2scfgr(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u16))
     }
  }
  #[inline] pub fn set_i2scfgr(&self, value: I2scfgr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u16, value.0);
     }
     self
  }
  #[inline] pub fn with_i2scfgr<F: FnOnce(I2scfgr) -> I2scfgr>(&self, f: F) -> &Self {
     let tmp = self.i2scfgr();
     self.set_i2scfgr(f(tmp))
  }

  #[inline] pub fn i2spr_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x20) as *const u16
  }
  #[inline] pub fn i2spr_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x20) as *mut u16
  }
  #[inline] pub fn i2spr(&self) -> I2spr { 
     unsafe {
        I2spr(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u16))
     }
  }
  #[inline] pub fn set_i2spr(&self, value: I2spr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u16, value.0);
     }
     self
  }
  #[inline] pub fn with_i2spr<F: FnOnce(I2spr) -> I2spr>(&self, f: F) -> &Self {
     let tmp = self.i2spr();
     self.set_i2spr(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Cr1(pub u16);
impl Cr1 {
  #[inline] pub fn bidimode(&self) -> u16 {
     ((self.0 as u16) >> 15) & 0x1 // [15]
  }
  #[inline] pub fn set_bidimode(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  #[inline] pub fn bidioe(&self) -> u16 {
     ((self.0 as u16) >> 14) & 0x1 // [14]
  }
  #[inline] pub fn set_bidioe(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline] pub fn crcen(&self) -> u16 {
     ((self.0 as u16) >> 13) & 0x1 // [13]
  }
  #[inline] pub fn set_crcen(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline] pub fn crcnext(&self) -> u16 {
     ((self.0 as u16) >> 12) & 0x1 // [12]
  }
  #[inline] pub fn set_crcnext(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn dff(&self) -> u16 {
     ((self.0 as u16) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_dff(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline] pub fn rxonly(&self) -> u16 {
     ((self.0 as u16) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_rxonly(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn ssm(&self) -> u16 {
     ((self.0 as u16) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_ssm(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn ssi(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_ssi(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn lsbfirst(&self) -> u16 {
     ((self.0 as u16) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_lsbfirst(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn spe(&self) -> u16 {
     ((self.0 as u16) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_spe(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn br(&self) -> u16 {
     ((self.0 as u16) >> 3) & 0x7 // [5:3]
  }
  #[inline] pub fn set_br(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn mstr(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_mstr(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn cpol(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_cpol(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn cpha(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_cpha(mut self, value: u16) -> Self {
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
pub struct Cr2(pub u16);
impl Cr2 {
  #[inline] pub fn rxdmaen(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_rxdmaen(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn txdmaen(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_txdmaen(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn ssoe(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_ssoe(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn frf(&self) -> u16 {
     ((self.0 as u16) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_frf(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn errie(&self) -> u16 {
     ((self.0 as u16) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_errie(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn rxneie(&self) -> u16 {
     ((self.0 as u16) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_rxneie(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn txeie(&self) -> u16 {
     ((self.0 as u16) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_txeie(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
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
      if self.frf() != 0 { try!(write!(f, " frf"))}
      if self.errie() != 0 { try!(write!(f, " errie"))}
      if self.rxneie() != 0 { try!(write!(f, " rxneie"))}
      if self.txeie() != 0 { try!(write!(f, " txeie"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Sr(pub u16);
impl Sr {
  #[inline] pub fn rxne(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_rxne(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn txe(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_txe(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn chside(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_chside(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn udr(&self) -> u16 {
     ((self.0 as u16) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_udr(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn crcerr(&self) -> u16 {
     ((self.0 as u16) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_crcerr(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn modf(&self) -> u16 {
     ((self.0 as u16) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_modf(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn ovr(&self) -> u16 {
     ((self.0 as u16) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_ovr(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn bsy(&self) -> u16 {
     ((self.0 as u16) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_bsy(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn tifrfe(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_tifrfe(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
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
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dr(pub u16);
impl Dr {
  #[inline] pub fn dr(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
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
#[derive(PartialEq, Eq)]
pub struct Crcpr(pub u32);
impl Crcpr {
  #[inline] pub fn crcpoly(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
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
#[derive(PartialEq, Eq)]
pub struct Rxcrcr(pub u16);
impl Rxcrcr {
  #[inline] pub fn rxcrc(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_rxcrc(mut self, value: u16) -> Self {
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
pub struct Txcrcr(pub u16);
impl Txcrcr {
  #[inline] pub fn txcrc(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_txcrc(mut self, value: u16) -> Self {
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
pub struct I2scfgr(pub u16);
impl I2scfgr {
  #[inline] pub fn i2smod(&self) -> u16 {
     ((self.0 as u16) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_i2smod(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline] pub fn i2se(&self) -> u16 {
     ((self.0 as u16) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_i2se(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn i2scfg(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x3 // [9:8]
  }
  #[inline] pub fn set_i2scfg(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn pcmsync(&self) -> u16 {
     ((self.0 as u16) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_pcmsync(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn i2sstd(&self) -> u16 {
     ((self.0 as u16) >> 4) & 0x3 // [5:4]
  }
  #[inline] pub fn set_i2sstd(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn ckpol(&self) -> u16 {
     ((self.0 as u16) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_ckpol(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn datlen(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x3 // [2:1]
  }
  #[inline] pub fn set_datlen(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn chlen(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_chlen(mut self, value: u16) -> Self {
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
pub struct I2spr(pub u16);
impl I2spr {
  #[inline] pub fn mckoe(&self) -> u16 {
     ((self.0 as u16) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_mckoe(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn odd(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_odd(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn i2sdiv(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xff // [7:0]
  }
  #[inline] pub fn set_i2sdiv(mut self, value: u16) -> Self {
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
pub trait IrqSpi<T> {
   fn irq_spi(&self) -> super::irq::Irq<T>;
}

pub trait RegisterSpiHandler {
   fn register_spi_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleSpi>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleSpi {
   fn handle_spi(&self);
}

impl IrqSpi<super::irq::Spi1Id> for Spi1 {
   fn irq_spi(&self) -> super::irq::IrqSpi1 { super::irq::IRQ_SPI1 }
}

impl RegisterSpiHandler for Spi1 {
   fn register_spi_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleSpi>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleSpi>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_spi() }
       }
       super::irq::set_handler(25, Some(wrapper::<F>));
       super::irq::IrqGuard::new(25)
   }
}

