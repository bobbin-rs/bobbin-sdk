
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpioImpl(pub u32);

impl GpioImpl {
  pub fn moder(&self) -> Moder { 
     unsafe {       Moder(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }  }
  pub fn set_moder(&self, value: Moder) {
     unsafe {       ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }  }
  pub fn with_moder<F: FnOnce(Moder) -> Moder>(&self, f: F) {
     let tmp = self.moder();
     self.set_moder(f(tmp))
  }

  pub fn otyper(&self) -> Otyper { 
     unsafe {       Otyper(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }  }
  pub fn set_otyper(&self, value: Otyper) {
     unsafe {       ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }  }
  pub fn with_otyper<F: FnOnce(Otyper) -> Otyper>(&self, f: F) {
     let tmp = self.otyper();
     self.set_otyper(f(tmp))
  }

  pub fn ospeedr(&self) -> Ospeedr { 
     unsafe {       Ospeedr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }  }
  pub fn set_ospeedr(&self, value: Ospeedr) {
     unsafe {       ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }  }
  pub fn with_ospeedr<F: FnOnce(Ospeedr) -> Ospeedr>(&self, f: F) {
     let tmp = self.ospeedr();
     self.set_ospeedr(f(tmp))
  }

  pub fn pupdr(&self) -> Pupdr { 
     unsafe {       Pupdr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }  }
  pub fn set_pupdr(&self, value: Pupdr) {
     unsafe {       ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }  }
  pub fn with_pupdr<F: FnOnce(Pupdr) -> Pupdr>(&self, f: F) {
     let tmp = self.pupdr();
     self.set_pupdr(f(tmp))
  }

  pub fn idr(&self) -> Idr { 
     unsafe {       Idr(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }  }

  pub fn odr(&self) -> Odr { 
     unsafe {       Odr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }  }
  pub fn set_odr(&self, value: Odr) {
     unsafe {       ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }  }
  pub fn with_odr<F: FnOnce(Odr) -> Odr>(&self, f: F) {
     let tmp = self.odr();
     self.set_odr(f(tmp))
  }

  pub fn set_bsrr(&self, value: Bsrr) {
     unsafe {       ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }  }

  pub fn lckr(&self) -> Lckr { 
     unsafe {       Lckr(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }  }
  pub fn set_lckr(&self, value: Lckr) {
     unsafe {       ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }  }
  pub fn with_lckr<F: FnOnce(Lckr) -> Lckr>(&self, f: F) {
     let tmp = self.lckr();
     self.set_lckr(f(tmp))
  }

  pub fn afrl(&self) -> Afrl { 
     unsafe {       Afrl(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }  }
  pub fn set_afrl(&self, value: Afrl) {
     unsafe {       ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }  }
  pub fn with_afrl<F: FnOnce(Afrl) -> Afrl>(&self, f: F) {
     let tmp = self.afrl();
     self.set_afrl(f(tmp))
  }

  pub fn afrh(&self) -> Afrh { 
     unsafe {       Afrh(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }  }
  pub fn set_afrh(&self, value: Afrh) {
     unsafe {       ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }  }
  pub fn with_afrh<F: FnOnce(Afrh) -> Afrh>(&self, f: F) {
     let tmp = self.afrh();
     self.set_afrh(f(tmp))
  }

  pub fn set_brr(&self, value: Brr) {
     unsafe {       ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
     }  }

}

#[derive(PartialEq, Eq)]
pub struct Moder(pub u32);

impl Moder {
  pub fn moder(&self, index: usize) -> u32 {
     assert!(index < 16);
     let shift: usize = 0 + (index << 1);
     ((self.0 as u32) >> shift) & 0x3 // [1:0]
  }
  pub fn set_moder(mut self, index: usize, value: u32) -> Self {
     assert!(index < 16);
     assert!((value & !0x3) == 0);
     let shift: usize = 0 + (index << 1);
     self.0 &= !(0x3 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Moder {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Moder {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.moder(0) != 0 { try!(write!(f, " moder[0]=0x{:x}", self.moder(0)))}
      if self.moder(1) != 0 { try!(write!(f, " moder[1]=0x{:x}", self.moder(1)))}
      if self.moder(2) != 0 { try!(write!(f, " moder[2]=0x{:x}", self.moder(2)))}
      if self.moder(3) != 0 { try!(write!(f, " moder[3]=0x{:x}", self.moder(3)))}
      if self.moder(4) != 0 { try!(write!(f, " moder[4]=0x{:x}", self.moder(4)))}
      if self.moder(5) != 0 { try!(write!(f, " moder[5]=0x{:x}", self.moder(5)))}
      if self.moder(6) != 0 { try!(write!(f, " moder[6]=0x{:x}", self.moder(6)))}
      if self.moder(7) != 0 { try!(write!(f, " moder[7]=0x{:x}", self.moder(7)))}
      if self.moder(8) != 0 { try!(write!(f, " moder[8]=0x{:x}", self.moder(8)))}
      if self.moder(9) != 0 { try!(write!(f, " moder[9]=0x{:x}", self.moder(9)))}
      if self.moder(10) != 0 { try!(write!(f, " moder[10]=0x{:x}", self.moder(10)))}
      if self.moder(11) != 0 { try!(write!(f, " moder[11]=0x{:x}", self.moder(11)))}
      if self.moder(12) != 0 { try!(write!(f, " moder[12]=0x{:x}", self.moder(12)))}
      if self.moder(13) != 0 { try!(write!(f, " moder[13]=0x{:x}", self.moder(13)))}
      if self.moder(14) != 0 { try!(write!(f, " moder[14]=0x{:x}", self.moder(14)))}
      if self.moder(15) != 0 { try!(write!(f, " moder[15]=0x{:x}", self.moder(15)))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Otyper(pub u32);

impl Otyper {
  pub fn ot(&self, index: usize) -> u32 {
     assert!(index < 16);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_ot(mut self, index: usize, value: u32) -> Self {
     assert!(index < 16);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Otyper {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Otyper {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ot(0) != 0 { try!(write!(f, " ot[0]"))}
      if self.ot(1) != 0 { try!(write!(f, " ot[1]"))}
      if self.ot(2) != 0 { try!(write!(f, " ot[2]"))}
      if self.ot(3) != 0 { try!(write!(f, " ot[3]"))}
      if self.ot(4) != 0 { try!(write!(f, " ot[4]"))}
      if self.ot(5) != 0 { try!(write!(f, " ot[5]"))}
      if self.ot(6) != 0 { try!(write!(f, " ot[6]"))}
      if self.ot(7) != 0 { try!(write!(f, " ot[7]"))}
      if self.ot(8) != 0 { try!(write!(f, " ot[8]"))}
      if self.ot(9) != 0 { try!(write!(f, " ot[9]"))}
      if self.ot(10) != 0 { try!(write!(f, " ot[10]"))}
      if self.ot(11) != 0 { try!(write!(f, " ot[11]"))}
      if self.ot(12) != 0 { try!(write!(f, " ot[12]"))}
      if self.ot(13) != 0 { try!(write!(f, " ot[13]"))}
      if self.ot(14) != 0 { try!(write!(f, " ot[14]"))}
      if self.ot(15) != 0 { try!(write!(f, " ot[15]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ospeedr(pub u32);

impl Ospeedr {
  pub fn ospeedr(&self, index: usize) -> u32 {
     assert!(index < 16);
     let shift: usize = 0 + (index << 1);
     ((self.0 as u32) >> shift) & 0x3 // [1:0]
  }
  pub fn set_ospeedr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 16);
     assert!((value & !0x3) == 0);
     let shift: usize = 0 + (index << 1);
     self.0 &= !(0x3 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Ospeedr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ospeedr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ospeedr(0) != 0 { try!(write!(f, " ospeedr[0]=0x{:x}", self.ospeedr(0)))}
      if self.ospeedr(1) != 0 { try!(write!(f, " ospeedr[1]=0x{:x}", self.ospeedr(1)))}
      if self.ospeedr(2) != 0 { try!(write!(f, " ospeedr[2]=0x{:x}", self.ospeedr(2)))}
      if self.ospeedr(3) != 0 { try!(write!(f, " ospeedr[3]=0x{:x}", self.ospeedr(3)))}
      if self.ospeedr(4) != 0 { try!(write!(f, " ospeedr[4]=0x{:x}", self.ospeedr(4)))}
      if self.ospeedr(5) != 0 { try!(write!(f, " ospeedr[5]=0x{:x}", self.ospeedr(5)))}
      if self.ospeedr(6) != 0 { try!(write!(f, " ospeedr[6]=0x{:x}", self.ospeedr(6)))}
      if self.ospeedr(7) != 0 { try!(write!(f, " ospeedr[7]=0x{:x}", self.ospeedr(7)))}
      if self.ospeedr(8) != 0 { try!(write!(f, " ospeedr[8]=0x{:x}", self.ospeedr(8)))}
      if self.ospeedr(9) != 0 { try!(write!(f, " ospeedr[9]=0x{:x}", self.ospeedr(9)))}
      if self.ospeedr(10) != 0 { try!(write!(f, " ospeedr[10]=0x{:x}", self.ospeedr(10)))}
      if self.ospeedr(11) != 0 { try!(write!(f, " ospeedr[11]=0x{:x}", self.ospeedr(11)))}
      if self.ospeedr(12) != 0 { try!(write!(f, " ospeedr[12]=0x{:x}", self.ospeedr(12)))}
      if self.ospeedr(13) != 0 { try!(write!(f, " ospeedr[13]=0x{:x}", self.ospeedr(13)))}
      if self.ospeedr(14) != 0 { try!(write!(f, " ospeedr[14]=0x{:x}", self.ospeedr(14)))}
      if self.ospeedr(15) != 0 { try!(write!(f, " ospeedr[15]=0x{:x}", self.ospeedr(15)))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Pupdr(pub u32);

impl Pupdr {
  pub fn pupdr(&self, index: usize) -> u32 {
     assert!(index < 16);
     let shift: usize = 0 + (index << 1);
     ((self.0 as u32) >> shift) & 0x3 // [1:0]
  }
  pub fn set_pupdr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 16);
     assert!((value & !0x3) == 0);
     let shift: usize = 0 + (index << 1);
     self.0 &= !(0x3 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Pupdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Pupdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pupdr(0) != 0 { try!(write!(f, " pupdr[0]=0x{:x}", self.pupdr(0)))}
      if self.pupdr(1) != 0 { try!(write!(f, " pupdr[1]=0x{:x}", self.pupdr(1)))}
      if self.pupdr(2) != 0 { try!(write!(f, " pupdr[2]=0x{:x}", self.pupdr(2)))}
      if self.pupdr(3) != 0 { try!(write!(f, " pupdr[3]=0x{:x}", self.pupdr(3)))}
      if self.pupdr(4) != 0 { try!(write!(f, " pupdr[4]=0x{:x}", self.pupdr(4)))}
      if self.pupdr(5) != 0 { try!(write!(f, " pupdr[5]=0x{:x}", self.pupdr(5)))}
      if self.pupdr(6) != 0 { try!(write!(f, " pupdr[6]=0x{:x}", self.pupdr(6)))}
      if self.pupdr(7) != 0 { try!(write!(f, " pupdr[7]=0x{:x}", self.pupdr(7)))}
      if self.pupdr(8) != 0 { try!(write!(f, " pupdr[8]=0x{:x}", self.pupdr(8)))}
      if self.pupdr(9) != 0 { try!(write!(f, " pupdr[9]=0x{:x}", self.pupdr(9)))}
      if self.pupdr(10) != 0 { try!(write!(f, " pupdr[10]=0x{:x}", self.pupdr(10)))}
      if self.pupdr(11) != 0 { try!(write!(f, " pupdr[11]=0x{:x}", self.pupdr(11)))}
      if self.pupdr(12) != 0 { try!(write!(f, " pupdr[12]=0x{:x}", self.pupdr(12)))}
      if self.pupdr(13) != 0 { try!(write!(f, " pupdr[13]=0x{:x}", self.pupdr(13)))}
      if self.pupdr(14) != 0 { try!(write!(f, " pupdr[14]=0x{:x}", self.pupdr(14)))}
      if self.pupdr(15) != 0 { try!(write!(f, " pupdr[15]=0x{:x}", self.pupdr(15)))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Idr(pub u32);

impl Idr {
  pub fn idr(&self, index: usize) -> u32 {
     assert!(index < 16);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_idr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 16);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Idr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Idr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.idr(0) != 0 { try!(write!(f, " idr[0]"))}
      if self.idr(1) != 0 { try!(write!(f, " idr[1]"))}
      if self.idr(2) != 0 { try!(write!(f, " idr[2]"))}
      if self.idr(3) != 0 { try!(write!(f, " idr[3]"))}
      if self.idr(4) != 0 { try!(write!(f, " idr[4]"))}
      if self.idr(5) != 0 { try!(write!(f, " idr[5]"))}
      if self.idr(6) != 0 { try!(write!(f, " idr[6]"))}
      if self.idr(7) != 0 { try!(write!(f, " idr[7]"))}
      if self.idr(8) != 0 { try!(write!(f, " idr[8]"))}
      if self.idr(9) != 0 { try!(write!(f, " idr[9]"))}
      if self.idr(10) != 0 { try!(write!(f, " idr[10]"))}
      if self.idr(11) != 0 { try!(write!(f, " idr[11]"))}
      if self.idr(12) != 0 { try!(write!(f, " idr[12]"))}
      if self.idr(13) != 0 { try!(write!(f, " idr[13]"))}
      if self.idr(14) != 0 { try!(write!(f, " idr[14]"))}
      if self.idr(15) != 0 { try!(write!(f, " idr[15]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Odr(pub u32);

impl Odr {
  pub fn odr(&self, index: usize) -> u32 {
     assert!(index < 16);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_odr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 16);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Odr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Odr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.odr(0) != 0 { try!(write!(f, " odr[0]"))}
      if self.odr(1) != 0 { try!(write!(f, " odr[1]"))}
      if self.odr(2) != 0 { try!(write!(f, " odr[2]"))}
      if self.odr(3) != 0 { try!(write!(f, " odr[3]"))}
      if self.odr(4) != 0 { try!(write!(f, " odr[4]"))}
      if self.odr(5) != 0 { try!(write!(f, " odr[5]"))}
      if self.odr(6) != 0 { try!(write!(f, " odr[6]"))}
      if self.odr(7) != 0 { try!(write!(f, " odr[7]"))}
      if self.odr(8) != 0 { try!(write!(f, " odr[8]"))}
      if self.odr(9) != 0 { try!(write!(f, " odr[9]"))}
      if self.odr(10) != 0 { try!(write!(f, " odr[10]"))}
      if self.odr(11) != 0 { try!(write!(f, " odr[11]"))}
      if self.odr(12) != 0 { try!(write!(f, " odr[12]"))}
      if self.odr(13) != 0 { try!(write!(f, " odr[13]"))}
      if self.odr(14) != 0 { try!(write!(f, " odr[14]"))}
      if self.odr(15) != 0 { try!(write!(f, " odr[15]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Bsrr(pub u32);

impl Bsrr {
  pub fn br(&self, index: usize) -> u32 {
     assert!(index < 16);
     let shift: usize = 16 + index;
     ((self.0 as u32) >> shift) & 0x1 // [16]
  }
  pub fn set_br(mut self, index: usize, value: u32) -> Self {
     assert!(index < 16);
     assert!((value & !0x1) == 0);
     let shift: usize = 16 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  pub fn bs(&self, index: usize) -> u32 {
     assert!(index < 16);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_bs(mut self, index: usize, value: u32) -> Self {
     assert!(index < 16);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Bsrr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Bsrr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.br(0) != 0 { try!(write!(f, " br[0]"))}
      if self.br(1) != 0 { try!(write!(f, " br[1]"))}
      if self.br(2) != 0 { try!(write!(f, " br[2]"))}
      if self.br(3) != 0 { try!(write!(f, " br[3]"))}
      if self.br(4) != 0 { try!(write!(f, " br[4]"))}
      if self.br(5) != 0 { try!(write!(f, " br[5]"))}
      if self.br(6) != 0 { try!(write!(f, " br[6]"))}
      if self.br(7) != 0 { try!(write!(f, " br[7]"))}
      if self.br(8) != 0 { try!(write!(f, " br[8]"))}
      if self.br(9) != 0 { try!(write!(f, " br[9]"))}
      if self.br(10) != 0 { try!(write!(f, " br[10]"))}
      if self.br(11) != 0 { try!(write!(f, " br[11]"))}
      if self.br(12) != 0 { try!(write!(f, " br[12]"))}
      if self.br(13) != 0 { try!(write!(f, " br[13]"))}
      if self.br(14) != 0 { try!(write!(f, " br[14]"))}
      if self.br(15) != 0 { try!(write!(f, " br[15]"))}
      if self.bs(0) != 0 { try!(write!(f, " bs[0]"))}
      if self.bs(1) != 0 { try!(write!(f, " bs[1]"))}
      if self.bs(2) != 0 { try!(write!(f, " bs[2]"))}
      if self.bs(3) != 0 { try!(write!(f, " bs[3]"))}
      if self.bs(4) != 0 { try!(write!(f, " bs[4]"))}
      if self.bs(5) != 0 { try!(write!(f, " bs[5]"))}
      if self.bs(6) != 0 { try!(write!(f, " bs[6]"))}
      if self.bs(7) != 0 { try!(write!(f, " bs[7]"))}
      if self.bs(8) != 0 { try!(write!(f, " bs[8]"))}
      if self.bs(9) != 0 { try!(write!(f, " bs[9]"))}
      if self.bs(10) != 0 { try!(write!(f, " bs[10]"))}
      if self.bs(11) != 0 { try!(write!(f, " bs[11]"))}
      if self.bs(12) != 0 { try!(write!(f, " bs[12]"))}
      if self.bs(13) != 0 { try!(write!(f, " bs[13]"))}
      if self.bs(14) != 0 { try!(write!(f, " bs[14]"))}
      if self.bs(15) != 0 { try!(write!(f, " bs[15]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Lckr(pub u32);

impl Lckr {
  pub fn lckk(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_lckk(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn lck(&self, index: usize) -> u32 {
     assert!(index < 16);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_lck(mut self, index: usize, value: u32) -> Self {
     assert!(index < 16);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Lckr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Lckr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lckk() != 0 { try!(write!(f, " lckk"))}
      if self.lck(0) != 0 { try!(write!(f, " lck[0]"))}
      if self.lck(1) != 0 { try!(write!(f, " lck[1]"))}
      if self.lck(2) != 0 { try!(write!(f, " lck[2]"))}
      if self.lck(3) != 0 { try!(write!(f, " lck[3]"))}
      if self.lck(4) != 0 { try!(write!(f, " lck[4]"))}
      if self.lck(5) != 0 { try!(write!(f, " lck[5]"))}
      if self.lck(6) != 0 { try!(write!(f, " lck[6]"))}
      if self.lck(7) != 0 { try!(write!(f, " lck[7]"))}
      if self.lck(8) != 0 { try!(write!(f, " lck[8]"))}
      if self.lck(9) != 0 { try!(write!(f, " lck[9]"))}
      if self.lck(10) != 0 { try!(write!(f, " lck[10]"))}
      if self.lck(11) != 0 { try!(write!(f, " lck[11]"))}
      if self.lck(12) != 0 { try!(write!(f, " lck[12]"))}
      if self.lck(13) != 0 { try!(write!(f, " lck[13]"))}
      if self.lck(14) != 0 { try!(write!(f, " lck[14]"))}
      if self.lck(15) != 0 { try!(write!(f, " lck[15]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Afrl(pub u32);

impl Afrl {
  pub fn afrl(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + (index << 2);
     ((self.0 as u32) >> shift) & 0xf // [3:0]
  }
  pub fn set_afrl(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0xf) == 0);
     let shift: usize = 0 + (index << 2);
     self.0 &= !(0xf << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Afrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Afrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.afrl(0) != 0 { try!(write!(f, " afrl[0]=0x{:x}", self.afrl(0)))}
      if self.afrl(1) != 0 { try!(write!(f, " afrl[1]=0x{:x}", self.afrl(1)))}
      if self.afrl(2) != 0 { try!(write!(f, " afrl[2]=0x{:x}", self.afrl(2)))}
      if self.afrl(3) != 0 { try!(write!(f, " afrl[3]=0x{:x}", self.afrl(3)))}
      if self.afrl(4) != 0 { try!(write!(f, " afrl[4]=0x{:x}", self.afrl(4)))}
      if self.afrl(5) != 0 { try!(write!(f, " afrl[5]=0x{:x}", self.afrl(5)))}
      if self.afrl(6) != 0 { try!(write!(f, " afrl[6]=0x{:x}", self.afrl(6)))}
      if self.afrl(7) != 0 { try!(write!(f, " afrl[7]=0x{:x}", self.afrl(7)))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Afrh(pub u32);

impl Afrh {
  pub fn afrh(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + (index << 2);
     ((self.0 as u32) >> shift) & 0xf // [3:0]
  }
  pub fn set_afrh(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0xf) == 0);
     let shift: usize = 0 + (index << 2);
     self.0 &= !(0xf << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Afrh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Afrh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.afrh(0) != 0 { try!(write!(f, " afrh[0]=0x{:x}", self.afrh(0)))}
      if self.afrh(1) != 0 { try!(write!(f, " afrh[1]=0x{:x}", self.afrh(1)))}
      if self.afrh(2) != 0 { try!(write!(f, " afrh[2]=0x{:x}", self.afrh(2)))}
      if self.afrh(3) != 0 { try!(write!(f, " afrh[3]=0x{:x}", self.afrh(3)))}
      if self.afrh(4) != 0 { try!(write!(f, " afrh[4]=0x{:x}", self.afrh(4)))}
      if self.afrh(5) != 0 { try!(write!(f, " afrh[5]=0x{:x}", self.afrh(5)))}
      if self.afrh(6) != 0 { try!(write!(f, " afrh[6]=0x{:x}", self.afrh(6)))}
      if self.afrh(7) != 0 { try!(write!(f, " afrh[7]=0x{:x}", self.afrh(7)))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Brr(pub u32);

impl Brr {
  pub fn br(&self, index: usize) -> u32 {
     assert!(index < 16);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_br(mut self, index: usize, value: u32) -> Self {
     assert!(index < 16);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Brr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Brr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.br(0) != 0 { try!(write!(f, " br[0]"))}
      if self.br(1) != 0 { try!(write!(f, " br[1]"))}
      if self.br(2) != 0 { try!(write!(f, " br[2]"))}
      if self.br(3) != 0 { try!(write!(f, " br[3]"))}
      if self.br(4) != 0 { try!(write!(f, " br[4]"))}
      if self.br(5) != 0 { try!(write!(f, " br[5]"))}
      if self.br(6) != 0 { try!(write!(f, " br[6]"))}
      if self.br(7) != 0 { try!(write!(f, " br[7]"))}
      if self.br(8) != 0 { try!(write!(f, " br[8]"))}
      if self.br(9) != 0 { try!(write!(f, " br[9]"))}
      if self.br(10) != 0 { try!(write!(f, " br[10]"))}
      if self.br(11) != 0 { try!(write!(f, " br[11]"))}
      if self.br(12) != 0 { try!(write!(f, " br[12]"))}
      if self.br(13) != 0 { try!(write!(f, " br[13]"))}
      if self.br(14) != 0 { try!(write!(f, " br[14]"))}
      if self.br(15) != 0 { try!(write!(f, " br[15]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

