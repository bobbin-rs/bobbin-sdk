pub const NVIC: Nvic = Nvic(0xe000e000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Nvic(pub u32);
impl Nvic {
  pub fn iser(&self, index: usize) -> Iser { 
     assert!(index < 8);
     unsafe {
        Iser(::core::ptr::read_volatile(((self.0 as usize) + 0x100 + (index << 2)) as *const u32))
     }
  }
  pub fn set_iser(&self, index: usize, value: Iser) -> &Nvic {
     assert!(index < 8);
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x100 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
  pub fn with_iser<F: FnOnce(Iser) -> Iser>(&self, index: usize, f: F) -> &Nvic {
     let tmp = self.iser(index);
     self.set_iser(index, f(tmp))
  }

  pub fn icer(&self, index: usize) -> Icer { 
     assert!(index < 8);
     unsafe {
        Icer(::core::ptr::read_volatile(((self.0 as usize) + 0x180 + (index << 2)) as *const u32))
     }
  }
  pub fn set_icer(&self, index: usize, value: Icer) -> &Nvic {
     assert!(index < 8);
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x180 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
  pub fn with_icer<F: FnOnce(Icer) -> Icer>(&self, index: usize, f: F) -> &Nvic {
     let tmp = self.icer(index);
     self.set_icer(index, f(tmp))
  }

  pub fn ispr(&self, index: usize) -> Ispr { 
     assert!(index < 8);
     unsafe {
        Ispr(::core::ptr::read_volatile(((self.0 as usize) + 0x200 + (index << 2)) as *const u32))
     }
  }
  pub fn set_ispr(&self, index: usize, value: Ispr) -> &Nvic {
     assert!(index < 8);
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x200 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
  pub fn with_ispr<F: FnOnce(Ispr) -> Ispr>(&self, index: usize, f: F) -> &Nvic {
     let tmp = self.ispr(index);
     self.set_ispr(index, f(tmp))
  }

  pub fn icpr(&self, index: usize) -> Icpr { 
     assert!(index < 8);
     unsafe {
        Icpr(::core::ptr::read_volatile(((self.0 as usize) + 0x280 + (index << 2)) as *const u32))
     }
  }
  pub fn set_icpr(&self, index: usize, value: Icpr) -> &Nvic {
     assert!(index < 8);
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x280 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
  pub fn with_icpr<F: FnOnce(Icpr) -> Icpr>(&self, index: usize, f: F) -> &Nvic {
     let tmp = self.icpr(index);
     self.set_icpr(index, f(tmp))
  }

  pub fn iabr(&self, index: usize) -> Iabr { 
     assert!(index < 8);
     unsafe {
        Iabr(::core::ptr::read_volatile(((self.0 as usize) + 0x280 + (index << 2)) as *const u32))
     }
  }
  pub fn set_iabr(&self, index: usize, value: Iabr) -> &Nvic {
     assert!(index < 8);
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x280 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
  pub fn with_iabr<F: FnOnce(Iabr) -> Iabr>(&self, index: usize, f: F) -> &Nvic {
     let tmp = self.iabr(index);
     self.set_iabr(index, f(tmp))
  }

  pub fn ipr(&self, index: usize) -> Ipr { 
     assert!(index < 60);
     unsafe {
        Ipr(::core::ptr::read_volatile(((self.0 as usize) + 0x400 + (index << 2)) as *const u32))
     }
  }
  pub fn set_ipr(&self, index: usize, value: Ipr) -> &Nvic {
     assert!(index < 60);
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x400 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
  pub fn with_ipr<F: FnOnce(Ipr) -> Ipr>(&self, index: usize, f: F) -> &Nvic {
     let tmp = self.ipr(index);
     self.set_ipr(index, f(tmp))
  }

  pub fn stir(&self) -> Stir { 
     unsafe {
       Stir(::core::ptr::read_volatile(((self.0 as usize) + 0xf00) as *const u32))
     }
  }
  pub fn set_stir(&self, value: Stir) -> &Nvic {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xf00) as *mut u32, value.0);
     }
     self
  }
  pub fn with_stir<F: FnOnce(Stir) -> Stir>(&self, f: F) -> &Nvic {
     let tmp = self.stir();
     self.set_stir(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Iser(pub u32);
impl Iser {
  pub fn setena(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_setena(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Iser {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Iser {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.setena(0) != 0 { try!(write!(f, " setena[0]"))}
      if self.setena(1) != 0 { try!(write!(f, " setena[1]"))}
      if self.setena(2) != 0 { try!(write!(f, " setena[2]"))}
      if self.setena(3) != 0 { try!(write!(f, " setena[3]"))}
      if self.setena(4) != 0 { try!(write!(f, " setena[4]"))}
      if self.setena(5) != 0 { try!(write!(f, " setena[5]"))}
      if self.setena(6) != 0 { try!(write!(f, " setena[6]"))}
      if self.setena(7) != 0 { try!(write!(f, " setena[7]"))}
      if self.setena(8) != 0 { try!(write!(f, " setena[8]"))}
      if self.setena(9) != 0 { try!(write!(f, " setena[9]"))}
      if self.setena(10) != 0 { try!(write!(f, " setena[10]"))}
      if self.setena(11) != 0 { try!(write!(f, " setena[11]"))}
      if self.setena(12) != 0 { try!(write!(f, " setena[12]"))}
      if self.setena(13) != 0 { try!(write!(f, " setena[13]"))}
      if self.setena(14) != 0 { try!(write!(f, " setena[14]"))}
      if self.setena(15) != 0 { try!(write!(f, " setena[15]"))}
      if self.setena(16) != 0 { try!(write!(f, " setena[16]"))}
      if self.setena(17) != 0 { try!(write!(f, " setena[17]"))}
      if self.setena(18) != 0 { try!(write!(f, " setena[18]"))}
      if self.setena(19) != 0 { try!(write!(f, " setena[19]"))}
      if self.setena(20) != 0 { try!(write!(f, " setena[20]"))}
      if self.setena(21) != 0 { try!(write!(f, " setena[21]"))}
      if self.setena(22) != 0 { try!(write!(f, " setena[22]"))}
      if self.setena(23) != 0 { try!(write!(f, " setena[23]"))}
      if self.setena(24) != 0 { try!(write!(f, " setena[24]"))}
      if self.setena(25) != 0 { try!(write!(f, " setena[25]"))}
      if self.setena(26) != 0 { try!(write!(f, " setena[26]"))}
      if self.setena(27) != 0 { try!(write!(f, " setena[27]"))}
      if self.setena(28) != 0 { try!(write!(f, " setena[28]"))}
      if self.setena(29) != 0 { try!(write!(f, " setena[29]"))}
      if self.setena(30) != 0 { try!(write!(f, " setena[30]"))}
      if self.setena(31) != 0 { try!(write!(f, " setena[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Icer(pub u32);
impl Icer {
  pub fn clrena(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_clrena(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Icer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Icer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clrena(0) != 0 { try!(write!(f, " clrena[0]"))}
      if self.clrena(1) != 0 { try!(write!(f, " clrena[1]"))}
      if self.clrena(2) != 0 { try!(write!(f, " clrena[2]"))}
      if self.clrena(3) != 0 { try!(write!(f, " clrena[3]"))}
      if self.clrena(4) != 0 { try!(write!(f, " clrena[4]"))}
      if self.clrena(5) != 0 { try!(write!(f, " clrena[5]"))}
      if self.clrena(6) != 0 { try!(write!(f, " clrena[6]"))}
      if self.clrena(7) != 0 { try!(write!(f, " clrena[7]"))}
      if self.clrena(8) != 0 { try!(write!(f, " clrena[8]"))}
      if self.clrena(9) != 0 { try!(write!(f, " clrena[9]"))}
      if self.clrena(10) != 0 { try!(write!(f, " clrena[10]"))}
      if self.clrena(11) != 0 { try!(write!(f, " clrena[11]"))}
      if self.clrena(12) != 0 { try!(write!(f, " clrena[12]"))}
      if self.clrena(13) != 0 { try!(write!(f, " clrena[13]"))}
      if self.clrena(14) != 0 { try!(write!(f, " clrena[14]"))}
      if self.clrena(15) != 0 { try!(write!(f, " clrena[15]"))}
      if self.clrena(16) != 0 { try!(write!(f, " clrena[16]"))}
      if self.clrena(17) != 0 { try!(write!(f, " clrena[17]"))}
      if self.clrena(18) != 0 { try!(write!(f, " clrena[18]"))}
      if self.clrena(19) != 0 { try!(write!(f, " clrena[19]"))}
      if self.clrena(20) != 0 { try!(write!(f, " clrena[20]"))}
      if self.clrena(21) != 0 { try!(write!(f, " clrena[21]"))}
      if self.clrena(22) != 0 { try!(write!(f, " clrena[22]"))}
      if self.clrena(23) != 0 { try!(write!(f, " clrena[23]"))}
      if self.clrena(24) != 0 { try!(write!(f, " clrena[24]"))}
      if self.clrena(25) != 0 { try!(write!(f, " clrena[25]"))}
      if self.clrena(26) != 0 { try!(write!(f, " clrena[26]"))}
      if self.clrena(27) != 0 { try!(write!(f, " clrena[27]"))}
      if self.clrena(28) != 0 { try!(write!(f, " clrena[28]"))}
      if self.clrena(29) != 0 { try!(write!(f, " clrena[29]"))}
      if self.clrena(30) != 0 { try!(write!(f, " clrena[30]"))}
      if self.clrena(31) != 0 { try!(write!(f, " clrena[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ispr(pub u32);
impl Ispr {
  pub fn setpend(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_setpend(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Ispr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ispr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.setpend(0) != 0 { try!(write!(f, " setpend[0]"))}
      if self.setpend(1) != 0 { try!(write!(f, " setpend[1]"))}
      if self.setpend(2) != 0 { try!(write!(f, " setpend[2]"))}
      if self.setpend(3) != 0 { try!(write!(f, " setpend[3]"))}
      if self.setpend(4) != 0 { try!(write!(f, " setpend[4]"))}
      if self.setpend(5) != 0 { try!(write!(f, " setpend[5]"))}
      if self.setpend(6) != 0 { try!(write!(f, " setpend[6]"))}
      if self.setpend(7) != 0 { try!(write!(f, " setpend[7]"))}
      if self.setpend(8) != 0 { try!(write!(f, " setpend[8]"))}
      if self.setpend(9) != 0 { try!(write!(f, " setpend[9]"))}
      if self.setpend(10) != 0 { try!(write!(f, " setpend[10]"))}
      if self.setpend(11) != 0 { try!(write!(f, " setpend[11]"))}
      if self.setpend(12) != 0 { try!(write!(f, " setpend[12]"))}
      if self.setpend(13) != 0 { try!(write!(f, " setpend[13]"))}
      if self.setpend(14) != 0 { try!(write!(f, " setpend[14]"))}
      if self.setpend(15) != 0 { try!(write!(f, " setpend[15]"))}
      if self.setpend(16) != 0 { try!(write!(f, " setpend[16]"))}
      if self.setpend(17) != 0 { try!(write!(f, " setpend[17]"))}
      if self.setpend(18) != 0 { try!(write!(f, " setpend[18]"))}
      if self.setpend(19) != 0 { try!(write!(f, " setpend[19]"))}
      if self.setpend(20) != 0 { try!(write!(f, " setpend[20]"))}
      if self.setpend(21) != 0 { try!(write!(f, " setpend[21]"))}
      if self.setpend(22) != 0 { try!(write!(f, " setpend[22]"))}
      if self.setpend(23) != 0 { try!(write!(f, " setpend[23]"))}
      if self.setpend(24) != 0 { try!(write!(f, " setpend[24]"))}
      if self.setpend(25) != 0 { try!(write!(f, " setpend[25]"))}
      if self.setpend(26) != 0 { try!(write!(f, " setpend[26]"))}
      if self.setpend(27) != 0 { try!(write!(f, " setpend[27]"))}
      if self.setpend(28) != 0 { try!(write!(f, " setpend[28]"))}
      if self.setpend(29) != 0 { try!(write!(f, " setpend[29]"))}
      if self.setpend(30) != 0 { try!(write!(f, " setpend[30]"))}
      if self.setpend(31) != 0 { try!(write!(f, " setpend[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Icpr(pub u32);
impl Icpr {
  pub fn clrpend(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_clrpend(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Icpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Icpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clrpend(0) != 0 { try!(write!(f, " clrpend[0]"))}
      if self.clrpend(1) != 0 { try!(write!(f, " clrpend[1]"))}
      if self.clrpend(2) != 0 { try!(write!(f, " clrpend[2]"))}
      if self.clrpend(3) != 0 { try!(write!(f, " clrpend[3]"))}
      if self.clrpend(4) != 0 { try!(write!(f, " clrpend[4]"))}
      if self.clrpend(5) != 0 { try!(write!(f, " clrpend[5]"))}
      if self.clrpend(6) != 0 { try!(write!(f, " clrpend[6]"))}
      if self.clrpend(7) != 0 { try!(write!(f, " clrpend[7]"))}
      if self.clrpend(8) != 0 { try!(write!(f, " clrpend[8]"))}
      if self.clrpend(9) != 0 { try!(write!(f, " clrpend[9]"))}
      if self.clrpend(10) != 0 { try!(write!(f, " clrpend[10]"))}
      if self.clrpend(11) != 0 { try!(write!(f, " clrpend[11]"))}
      if self.clrpend(12) != 0 { try!(write!(f, " clrpend[12]"))}
      if self.clrpend(13) != 0 { try!(write!(f, " clrpend[13]"))}
      if self.clrpend(14) != 0 { try!(write!(f, " clrpend[14]"))}
      if self.clrpend(15) != 0 { try!(write!(f, " clrpend[15]"))}
      if self.clrpend(16) != 0 { try!(write!(f, " clrpend[16]"))}
      if self.clrpend(17) != 0 { try!(write!(f, " clrpend[17]"))}
      if self.clrpend(18) != 0 { try!(write!(f, " clrpend[18]"))}
      if self.clrpend(19) != 0 { try!(write!(f, " clrpend[19]"))}
      if self.clrpend(20) != 0 { try!(write!(f, " clrpend[20]"))}
      if self.clrpend(21) != 0 { try!(write!(f, " clrpend[21]"))}
      if self.clrpend(22) != 0 { try!(write!(f, " clrpend[22]"))}
      if self.clrpend(23) != 0 { try!(write!(f, " clrpend[23]"))}
      if self.clrpend(24) != 0 { try!(write!(f, " clrpend[24]"))}
      if self.clrpend(25) != 0 { try!(write!(f, " clrpend[25]"))}
      if self.clrpend(26) != 0 { try!(write!(f, " clrpend[26]"))}
      if self.clrpend(27) != 0 { try!(write!(f, " clrpend[27]"))}
      if self.clrpend(28) != 0 { try!(write!(f, " clrpend[28]"))}
      if self.clrpend(29) != 0 { try!(write!(f, " clrpend[29]"))}
      if self.clrpend(30) != 0 { try!(write!(f, " clrpend[30]"))}
      if self.clrpend(31) != 0 { try!(write!(f, " clrpend[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Iabr(pub u32);
impl Iabr {
  pub fn active(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_active(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Iabr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Iabr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.active(0) != 0 { try!(write!(f, " active[0]"))}
      if self.active(1) != 0 { try!(write!(f, " active[1]"))}
      if self.active(2) != 0 { try!(write!(f, " active[2]"))}
      if self.active(3) != 0 { try!(write!(f, " active[3]"))}
      if self.active(4) != 0 { try!(write!(f, " active[4]"))}
      if self.active(5) != 0 { try!(write!(f, " active[5]"))}
      if self.active(6) != 0 { try!(write!(f, " active[6]"))}
      if self.active(7) != 0 { try!(write!(f, " active[7]"))}
      if self.active(8) != 0 { try!(write!(f, " active[8]"))}
      if self.active(9) != 0 { try!(write!(f, " active[9]"))}
      if self.active(10) != 0 { try!(write!(f, " active[10]"))}
      if self.active(11) != 0 { try!(write!(f, " active[11]"))}
      if self.active(12) != 0 { try!(write!(f, " active[12]"))}
      if self.active(13) != 0 { try!(write!(f, " active[13]"))}
      if self.active(14) != 0 { try!(write!(f, " active[14]"))}
      if self.active(15) != 0 { try!(write!(f, " active[15]"))}
      if self.active(16) != 0 { try!(write!(f, " active[16]"))}
      if self.active(17) != 0 { try!(write!(f, " active[17]"))}
      if self.active(18) != 0 { try!(write!(f, " active[18]"))}
      if self.active(19) != 0 { try!(write!(f, " active[19]"))}
      if self.active(20) != 0 { try!(write!(f, " active[20]"))}
      if self.active(21) != 0 { try!(write!(f, " active[21]"))}
      if self.active(22) != 0 { try!(write!(f, " active[22]"))}
      if self.active(23) != 0 { try!(write!(f, " active[23]"))}
      if self.active(24) != 0 { try!(write!(f, " active[24]"))}
      if self.active(25) != 0 { try!(write!(f, " active[25]"))}
      if self.active(26) != 0 { try!(write!(f, " active[26]"))}
      if self.active(27) != 0 { try!(write!(f, " active[27]"))}
      if self.active(28) != 0 { try!(write!(f, " active[28]"))}
      if self.active(29) != 0 { try!(write!(f, " active[29]"))}
      if self.active(30) != 0 { try!(write!(f, " active[30]"))}
      if self.active(31) != 0 { try!(write!(f, " active[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ipr(pub u32);
impl Ipr {
  pub fn pri(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 0 + (index << 3);
     ((self.0 as u32) >> shift) & 0xff // [7:0]
  }
  pub fn set_pri(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0xff) == 0);
     let shift: usize = 0 + (index << 3);
     self.0 &= !(0xff << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Ipr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ipr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pri(0) != 0 { try!(write!(f, " pri[0]=0x{:x}", self.pri(0)))}
      if self.pri(1) != 0 { try!(write!(f, " pri[1]=0x{:x}", self.pri(1)))}
      if self.pri(2) != 0 { try!(write!(f, " pri[2]=0x{:x}", self.pri(2)))}
      if self.pri(3) != 0 { try!(write!(f, " pri[3]=0x{:x}", self.pri(3)))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Stir(pub u32);
impl Stir {
  pub fn intid(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_intid(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Stir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Stir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.intid() != 0 { try!(write!(f, " intid=0x{:x}", self.intid()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
