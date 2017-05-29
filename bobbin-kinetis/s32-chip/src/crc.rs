pub const CRC: Crc = Crc(0x40032000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Crc(pub u32);

impl Crc {
  pub unsafe fn data(&self) -> Data { 
     Data(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
  }
  pub unsafe fn set_data(&mut self, value: Data) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
  }
  pub unsafe fn with_data<F: FnOnce(Data) -> Data>(&mut self, f: F) {
     let tmp = self.data();
     self.set_data(f(tmp))
  }

  pub unsafe fn datal(&self) -> Datal { 
     Datal(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u16))
  }
  pub unsafe fn set_datal(&mut self, value: Datal) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u16, value.0);
  }
  pub unsafe fn with_datal<F: FnOnce(Datal) -> Datal>(&mut self, f: F) {
     let tmp = self.datal();
     self.set_datal(f(tmp))
  }

  pub unsafe fn datall(&self) -> Datall { 
     Datall(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u8))
  }
  pub unsafe fn set_datall(&mut self, value: Datall) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u8, value.0);
  }
  pub unsafe fn with_datall<F: FnOnce(Datall) -> Datall>(&mut self, f: F) {
     let tmp = self.datall();
     self.set_datall(f(tmp))
  }

  pub unsafe fn datalu(&self) -> Datalu { 
     Datalu(::core::ptr::read_volatile(((self.0 as usize) + 0x1) as *const u8))
  }
  pub unsafe fn set_datalu(&mut self, value: Datalu) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1) as *mut u8, value.0);
  }
  pub unsafe fn with_datalu<F: FnOnce(Datalu) -> Datalu>(&mut self, f: F) {
     let tmp = self.datalu();
     self.set_datalu(f(tmp))
  }

  pub unsafe fn datah(&self) -> Datah { 
     Datah(::core::ptr::read_volatile(((self.0 as usize) + 0x2) as *const u16))
  }
  pub unsafe fn set_datah(&mut self, value: Datah) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x2) as *mut u16, value.0);
  }
  pub unsafe fn with_datah<F: FnOnce(Datah) -> Datah>(&mut self, f: F) {
     let tmp = self.datah();
     self.set_datah(f(tmp))
  }

  pub unsafe fn datahl(&self) -> Datahl { 
     Datahl(::core::ptr::read_volatile(((self.0 as usize) + 0x2) as *const u8))
  }
  pub unsafe fn set_datahl(&mut self, value: Datahl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x2) as *mut u8, value.0);
  }
  pub unsafe fn with_datahl<F: FnOnce(Datahl) -> Datahl>(&mut self, f: F) {
     let tmp = self.datahl();
     self.set_datahl(f(tmp))
  }

  pub unsafe fn datahu(&self) -> Datahu { 
     Datahu(::core::ptr::read_volatile(((self.0 as usize) + 0x3) as *const u8))
  }
  pub unsafe fn set_datahu(&mut self, value: Datahu) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x3) as *mut u8, value.0);
  }
  pub unsafe fn with_datahu<F: FnOnce(Datahu) -> Datahu>(&mut self, f: F) {
     let tmp = self.datahu();
     self.set_datahu(f(tmp))
  }

  pub unsafe fn gpoly(&self) -> Gpoly { 
     Gpoly(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
  }
  pub unsafe fn set_gpoly(&mut self, value: Gpoly) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
  }
  pub unsafe fn with_gpoly<F: FnOnce(Gpoly) -> Gpoly>(&mut self, f: F) {
     let tmp = self.gpoly();
     self.set_gpoly(f(tmp))
  }

  pub unsafe fn ctrl(&self) -> Ctrl { 
     Ctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
  }
  pub unsafe fn set_ctrl(&mut self, value: Ctrl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
  }
  pub unsafe fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&mut self, f: F) {
     let tmp = self.ctrl();
     self.set_ctrl(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Data(pub u32);

impl Data {
  pub fn ll(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_ll(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn lu(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
  pub fn set_lu(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

  pub fn hl(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  pub fn set_hl(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn hu(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
  pub fn set_hu(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

}

impl ::core::fmt::Display for Data {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Data {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ll() != 0 { try!(write!(f, " ll=0x{:x}", self.ll()))}
      if self.lu() != 0 { try!(write!(f, " lu=0x{:x}", self.lu()))}
      if self.hl() != 0 { try!(write!(f, " hl=0x{:x}", self.hl()))}
      if self.hu() != 0 { try!(write!(f, " hu=0x{:x}", self.hu()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Datal(pub u16);

impl Datal {
  pub fn datal(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
  pub fn set_datal(mut self, value: u16) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Datal {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Datal {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.datal() != 0 { try!(write!(f, " datal=0x{:x}", self.datal()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Datall(pub u8);

impl Datall {
  pub fn datall(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_datall(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Datall {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Datall {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.datall() != 0 { try!(write!(f, " datall=0x{:x}", self.datall()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Datalu(pub u8);

impl Datalu {
  pub fn datalu(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_datalu(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Datalu {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Datalu {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.datalu() != 0 { try!(write!(f, " datalu=0x{:x}", self.datalu()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Datah(pub u16);

impl Datah {
  pub fn datah(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
  pub fn set_datah(mut self, value: u16) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Datah {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Datah {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.datah() != 0 { try!(write!(f, " datah=0x{:x}", self.datah()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Datahl(pub u8);

impl Datahl {
  pub fn datahl(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_datahl(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Datahl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Datahl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.datahl() != 0 { try!(write!(f, " datahl=0x{:x}", self.datahl()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Datahu(pub u8);

impl Datahu {
  pub fn datahu(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_datahu(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Datahu {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Datahu {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.datahu() != 0 { try!(write!(f, " datahu=0x{:x}", self.datahu()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Gpoly(pub u32);

impl Gpoly {
  pub fn low(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_low(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn high(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xffff // [31:16]
  }
  pub fn set_high(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 16);
     self.0 |= value << 16;
     self
  }

}

impl ::core::fmt::Display for Gpoly {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Gpoly {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.low() != 0 { try!(write!(f, " low=0x{:x}", self.low()))}
      if self.high() != 0 { try!(write!(f, " high=0x{:x}", self.high()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ctrl(pub u32);

impl Ctrl {
  pub fn tcrc(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_tcrc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn was(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_was(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn fxor(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_fxor(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  pub fn totr(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x3 // [29:28]
  }
  pub fn set_totr(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn tot(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x3 // [31:30]
  }
  pub fn set_tot(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 30);
     self.0 |= value << 30;
     self
  }

}

impl ::core::fmt::Display for Ctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tcrc() != 0 { try!(write!(f, " tcrc"))}
      if self.was() != 0 { try!(write!(f, " was"))}
      if self.fxor() != 0 { try!(write!(f, " fxor"))}
      if self.totr() != 0 { try!(write!(f, " totr=0x{:x}", self.totr()))}
      if self.tot() != 0 { try!(write!(f, " tot=0x{:x}", self.tot()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

