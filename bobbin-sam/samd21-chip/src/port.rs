pub const PORTA: Port = Port(0x41004400);
pub const PORTB: Port = Port(0x41004480);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Port(pub u32);

impl Port {
  pub unsafe fn ctrl(&self) -> Ctrl { 
     Ctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
  }
  pub unsafe fn set_ctrl(&mut self, value: Ctrl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
  }
  pub unsafe fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&mut self, f: F) {
     let tmp = self.ctrl();
     self.set_ctrl(f(tmp))
  }

  pub unsafe fn dir(&self) -> Dir { 
     Dir(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
  }
  pub unsafe fn set_dir(&mut self, value: Dir) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
  }
  pub unsafe fn with_dir<F: FnOnce(Dir) -> Dir>(&mut self, f: F) {
     let tmp = self.dir();
     self.set_dir(f(tmp))
  }

  pub unsafe fn dirclr(&self) -> Dirclr { 
     Dirclr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
  }
  pub unsafe fn set_dirclr(&mut self, value: Dirclr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
  }
  pub unsafe fn with_dirclr<F: FnOnce(Dirclr) -> Dirclr>(&mut self, f: F) {
     let tmp = self.dirclr();
     self.set_dirclr(f(tmp))
  }

  pub unsafe fn dirset(&self) -> Dirset { 
     Dirset(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
  }
  pub unsafe fn set_dirset(&mut self, value: Dirset) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
  }
  pub unsafe fn with_dirset<F: FnOnce(Dirset) -> Dirset>(&mut self, f: F) {
     let tmp = self.dirset();
     self.set_dirset(f(tmp))
  }

  pub unsafe fn dirtgl(&self) -> Dirtgl { 
     Dirtgl(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
  }
  pub unsafe fn set_dirtgl(&mut self, value: Dirtgl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
  }
  pub unsafe fn with_dirtgl<F: FnOnce(Dirtgl) -> Dirtgl>(&mut self, f: F) {
     let tmp = self.dirtgl();
     self.set_dirtgl(f(tmp))
  }

  pub unsafe fn _in(&self) -> In { 
     In(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
  }

  pub unsafe fn out(&self) -> Out { 
     Out(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
  }
  pub unsafe fn set_out(&mut self, value: Out) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
  }
  pub unsafe fn with_out<F: FnOnce(Out) -> Out>(&mut self, f: F) {
     let tmp = self.out();
     self.set_out(f(tmp))
  }

  pub unsafe fn outclr(&self) -> Outclr { 
     Outclr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
  }
  pub unsafe fn set_outclr(&mut self, value: Outclr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
  }
  pub unsafe fn with_outclr<F: FnOnce(Outclr) -> Outclr>(&mut self, f: F) {
     let tmp = self.outclr();
     self.set_outclr(f(tmp))
  }

  pub unsafe fn outset(&self) -> Outset { 
     Outset(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
  }
  pub unsafe fn set_outset(&mut self, value: Outset) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
  }
  pub unsafe fn with_outset<F: FnOnce(Outset) -> Outset>(&mut self, f: F) {
     let tmp = self.outset();
     self.set_outset(f(tmp))
  }

  pub unsafe fn outtgl(&self) -> Outtgl { 
     Outtgl(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
  }
  pub unsafe fn set_outtgl(&mut self, value: Outtgl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
  }
  pub unsafe fn with_outtgl<F: FnOnce(Outtgl) -> Outtgl>(&mut self, f: F) {
     let tmp = self.outtgl();
     self.set_outtgl(f(tmp))
  }

  pub unsafe fn pincfg(&self, index: usize) -> Pincfg { 
     assert!(index < 32);
     Pincfg(::core::ptr::read_volatile(((self.0 as usize) + 0x40 + (index)) as *const u8))
  }
  pub unsafe fn set_pincfg(&mut self, index: usize, value: Pincfg) {
     assert!(index < 32);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x40 + (index)) as *mut u8, value.0);
  }
  pub unsafe fn with_pincfg<F: FnOnce(Pincfg) -> Pincfg>(&mut self, index: usize, f: F) {
     let tmp = self.pincfg(index);
     self.set_pincfg(index, f(tmp))
  }

  pub unsafe fn pmux(&self, index: usize) -> Pmux { 
     assert!(index < 16);
     Pmux(::core::ptr::read_volatile(((self.0 as usize) + 0x30 + (index)) as *const u8))
  }
  pub unsafe fn set_pmux(&mut self, index: usize, value: Pmux) {
     assert!(index < 16);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x30 + (index)) as *mut u8, value.0);
  }
  pub unsafe fn with_pmux<F: FnOnce(Pmux) -> Pmux>(&mut self, index: usize, f: F) {
     let tmp = self.pmux(index);
     self.set_pmux(index, f(tmp))
  }

  pub unsafe fn set_wrconfig(&mut self, value: Wrconfig) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
  }

}

#[derive(PartialEq, Eq)]
pub struct Ctrl(pub u32);

impl Ctrl {
  pub fn sampling(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_sampling(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
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
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dir(pub u32);

impl Dir {
  pub fn dir(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_dir(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Dir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dir(0) != 0 { try!(write!(f, " dir[0]"))}
      if self.dir(1) != 0 { try!(write!(f, " dir[1]"))}
      if self.dir(2) != 0 { try!(write!(f, " dir[2]"))}
      if self.dir(3) != 0 { try!(write!(f, " dir[3]"))}
      if self.dir(4) != 0 { try!(write!(f, " dir[4]"))}
      if self.dir(5) != 0 { try!(write!(f, " dir[5]"))}
      if self.dir(6) != 0 { try!(write!(f, " dir[6]"))}
      if self.dir(7) != 0 { try!(write!(f, " dir[7]"))}
      if self.dir(8) != 0 { try!(write!(f, " dir[8]"))}
      if self.dir(9) != 0 { try!(write!(f, " dir[9]"))}
      if self.dir(10) != 0 { try!(write!(f, " dir[10]"))}
      if self.dir(11) != 0 { try!(write!(f, " dir[11]"))}
      if self.dir(12) != 0 { try!(write!(f, " dir[12]"))}
      if self.dir(13) != 0 { try!(write!(f, " dir[13]"))}
      if self.dir(14) != 0 { try!(write!(f, " dir[14]"))}
      if self.dir(15) != 0 { try!(write!(f, " dir[15]"))}
      if self.dir(16) != 0 { try!(write!(f, " dir[16]"))}
      if self.dir(17) != 0 { try!(write!(f, " dir[17]"))}
      if self.dir(18) != 0 { try!(write!(f, " dir[18]"))}
      if self.dir(19) != 0 { try!(write!(f, " dir[19]"))}
      if self.dir(20) != 0 { try!(write!(f, " dir[20]"))}
      if self.dir(21) != 0 { try!(write!(f, " dir[21]"))}
      if self.dir(22) != 0 { try!(write!(f, " dir[22]"))}
      if self.dir(23) != 0 { try!(write!(f, " dir[23]"))}
      if self.dir(24) != 0 { try!(write!(f, " dir[24]"))}
      if self.dir(25) != 0 { try!(write!(f, " dir[25]"))}
      if self.dir(26) != 0 { try!(write!(f, " dir[26]"))}
      if self.dir(27) != 0 { try!(write!(f, " dir[27]"))}
      if self.dir(28) != 0 { try!(write!(f, " dir[28]"))}
      if self.dir(29) != 0 { try!(write!(f, " dir[29]"))}
      if self.dir(30) != 0 { try!(write!(f, " dir[30]"))}
      if self.dir(31) != 0 { try!(write!(f, " dir[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dirclr(pub u32);

impl Dirclr {
  pub fn dirclr(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_dirclr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Dirclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dirclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dirclr(0) != 0 { try!(write!(f, " dirclr[0]"))}
      if self.dirclr(1) != 0 { try!(write!(f, " dirclr[1]"))}
      if self.dirclr(2) != 0 { try!(write!(f, " dirclr[2]"))}
      if self.dirclr(3) != 0 { try!(write!(f, " dirclr[3]"))}
      if self.dirclr(4) != 0 { try!(write!(f, " dirclr[4]"))}
      if self.dirclr(5) != 0 { try!(write!(f, " dirclr[5]"))}
      if self.dirclr(6) != 0 { try!(write!(f, " dirclr[6]"))}
      if self.dirclr(7) != 0 { try!(write!(f, " dirclr[7]"))}
      if self.dirclr(8) != 0 { try!(write!(f, " dirclr[8]"))}
      if self.dirclr(9) != 0 { try!(write!(f, " dirclr[9]"))}
      if self.dirclr(10) != 0 { try!(write!(f, " dirclr[10]"))}
      if self.dirclr(11) != 0 { try!(write!(f, " dirclr[11]"))}
      if self.dirclr(12) != 0 { try!(write!(f, " dirclr[12]"))}
      if self.dirclr(13) != 0 { try!(write!(f, " dirclr[13]"))}
      if self.dirclr(14) != 0 { try!(write!(f, " dirclr[14]"))}
      if self.dirclr(15) != 0 { try!(write!(f, " dirclr[15]"))}
      if self.dirclr(16) != 0 { try!(write!(f, " dirclr[16]"))}
      if self.dirclr(17) != 0 { try!(write!(f, " dirclr[17]"))}
      if self.dirclr(18) != 0 { try!(write!(f, " dirclr[18]"))}
      if self.dirclr(19) != 0 { try!(write!(f, " dirclr[19]"))}
      if self.dirclr(20) != 0 { try!(write!(f, " dirclr[20]"))}
      if self.dirclr(21) != 0 { try!(write!(f, " dirclr[21]"))}
      if self.dirclr(22) != 0 { try!(write!(f, " dirclr[22]"))}
      if self.dirclr(23) != 0 { try!(write!(f, " dirclr[23]"))}
      if self.dirclr(24) != 0 { try!(write!(f, " dirclr[24]"))}
      if self.dirclr(25) != 0 { try!(write!(f, " dirclr[25]"))}
      if self.dirclr(26) != 0 { try!(write!(f, " dirclr[26]"))}
      if self.dirclr(27) != 0 { try!(write!(f, " dirclr[27]"))}
      if self.dirclr(28) != 0 { try!(write!(f, " dirclr[28]"))}
      if self.dirclr(29) != 0 { try!(write!(f, " dirclr[29]"))}
      if self.dirclr(30) != 0 { try!(write!(f, " dirclr[30]"))}
      if self.dirclr(31) != 0 { try!(write!(f, " dirclr[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dirset(pub u32);

impl Dirset {
  pub fn dirset(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_dirset(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Dirset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dirset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dirset(0) != 0 { try!(write!(f, " dirset[0]"))}
      if self.dirset(1) != 0 { try!(write!(f, " dirset[1]"))}
      if self.dirset(2) != 0 { try!(write!(f, " dirset[2]"))}
      if self.dirset(3) != 0 { try!(write!(f, " dirset[3]"))}
      if self.dirset(4) != 0 { try!(write!(f, " dirset[4]"))}
      if self.dirset(5) != 0 { try!(write!(f, " dirset[5]"))}
      if self.dirset(6) != 0 { try!(write!(f, " dirset[6]"))}
      if self.dirset(7) != 0 { try!(write!(f, " dirset[7]"))}
      if self.dirset(8) != 0 { try!(write!(f, " dirset[8]"))}
      if self.dirset(9) != 0 { try!(write!(f, " dirset[9]"))}
      if self.dirset(10) != 0 { try!(write!(f, " dirset[10]"))}
      if self.dirset(11) != 0 { try!(write!(f, " dirset[11]"))}
      if self.dirset(12) != 0 { try!(write!(f, " dirset[12]"))}
      if self.dirset(13) != 0 { try!(write!(f, " dirset[13]"))}
      if self.dirset(14) != 0 { try!(write!(f, " dirset[14]"))}
      if self.dirset(15) != 0 { try!(write!(f, " dirset[15]"))}
      if self.dirset(16) != 0 { try!(write!(f, " dirset[16]"))}
      if self.dirset(17) != 0 { try!(write!(f, " dirset[17]"))}
      if self.dirset(18) != 0 { try!(write!(f, " dirset[18]"))}
      if self.dirset(19) != 0 { try!(write!(f, " dirset[19]"))}
      if self.dirset(20) != 0 { try!(write!(f, " dirset[20]"))}
      if self.dirset(21) != 0 { try!(write!(f, " dirset[21]"))}
      if self.dirset(22) != 0 { try!(write!(f, " dirset[22]"))}
      if self.dirset(23) != 0 { try!(write!(f, " dirset[23]"))}
      if self.dirset(24) != 0 { try!(write!(f, " dirset[24]"))}
      if self.dirset(25) != 0 { try!(write!(f, " dirset[25]"))}
      if self.dirset(26) != 0 { try!(write!(f, " dirset[26]"))}
      if self.dirset(27) != 0 { try!(write!(f, " dirset[27]"))}
      if self.dirset(28) != 0 { try!(write!(f, " dirset[28]"))}
      if self.dirset(29) != 0 { try!(write!(f, " dirset[29]"))}
      if self.dirset(30) != 0 { try!(write!(f, " dirset[30]"))}
      if self.dirset(31) != 0 { try!(write!(f, " dirset[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dirtgl(pub u32);

impl Dirtgl {
  pub fn dirtgl(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_dirtgl(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Dirtgl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dirtgl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dirtgl(0) != 0 { try!(write!(f, " dirtgl[0]"))}
      if self.dirtgl(1) != 0 { try!(write!(f, " dirtgl[1]"))}
      if self.dirtgl(2) != 0 { try!(write!(f, " dirtgl[2]"))}
      if self.dirtgl(3) != 0 { try!(write!(f, " dirtgl[3]"))}
      if self.dirtgl(4) != 0 { try!(write!(f, " dirtgl[4]"))}
      if self.dirtgl(5) != 0 { try!(write!(f, " dirtgl[5]"))}
      if self.dirtgl(6) != 0 { try!(write!(f, " dirtgl[6]"))}
      if self.dirtgl(7) != 0 { try!(write!(f, " dirtgl[7]"))}
      if self.dirtgl(8) != 0 { try!(write!(f, " dirtgl[8]"))}
      if self.dirtgl(9) != 0 { try!(write!(f, " dirtgl[9]"))}
      if self.dirtgl(10) != 0 { try!(write!(f, " dirtgl[10]"))}
      if self.dirtgl(11) != 0 { try!(write!(f, " dirtgl[11]"))}
      if self.dirtgl(12) != 0 { try!(write!(f, " dirtgl[12]"))}
      if self.dirtgl(13) != 0 { try!(write!(f, " dirtgl[13]"))}
      if self.dirtgl(14) != 0 { try!(write!(f, " dirtgl[14]"))}
      if self.dirtgl(15) != 0 { try!(write!(f, " dirtgl[15]"))}
      if self.dirtgl(16) != 0 { try!(write!(f, " dirtgl[16]"))}
      if self.dirtgl(17) != 0 { try!(write!(f, " dirtgl[17]"))}
      if self.dirtgl(18) != 0 { try!(write!(f, " dirtgl[18]"))}
      if self.dirtgl(19) != 0 { try!(write!(f, " dirtgl[19]"))}
      if self.dirtgl(20) != 0 { try!(write!(f, " dirtgl[20]"))}
      if self.dirtgl(21) != 0 { try!(write!(f, " dirtgl[21]"))}
      if self.dirtgl(22) != 0 { try!(write!(f, " dirtgl[22]"))}
      if self.dirtgl(23) != 0 { try!(write!(f, " dirtgl[23]"))}
      if self.dirtgl(24) != 0 { try!(write!(f, " dirtgl[24]"))}
      if self.dirtgl(25) != 0 { try!(write!(f, " dirtgl[25]"))}
      if self.dirtgl(26) != 0 { try!(write!(f, " dirtgl[26]"))}
      if self.dirtgl(27) != 0 { try!(write!(f, " dirtgl[27]"))}
      if self.dirtgl(28) != 0 { try!(write!(f, " dirtgl[28]"))}
      if self.dirtgl(29) != 0 { try!(write!(f, " dirtgl[29]"))}
      if self.dirtgl(30) != 0 { try!(write!(f, " dirtgl[30]"))}
      if self.dirtgl(31) != 0 { try!(write!(f, " dirtgl[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct In(pub u32);

impl In {
  pub fn _in(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_in(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for In {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for In {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self._in(0) != 0 { try!(write!(f, " _in[0]"))}
      if self._in(1) != 0 { try!(write!(f, " _in[1]"))}
      if self._in(2) != 0 { try!(write!(f, " _in[2]"))}
      if self._in(3) != 0 { try!(write!(f, " _in[3]"))}
      if self._in(4) != 0 { try!(write!(f, " _in[4]"))}
      if self._in(5) != 0 { try!(write!(f, " _in[5]"))}
      if self._in(6) != 0 { try!(write!(f, " _in[6]"))}
      if self._in(7) != 0 { try!(write!(f, " _in[7]"))}
      if self._in(8) != 0 { try!(write!(f, " _in[8]"))}
      if self._in(9) != 0 { try!(write!(f, " _in[9]"))}
      if self._in(10) != 0 { try!(write!(f, " _in[10]"))}
      if self._in(11) != 0 { try!(write!(f, " _in[11]"))}
      if self._in(12) != 0 { try!(write!(f, " _in[12]"))}
      if self._in(13) != 0 { try!(write!(f, " _in[13]"))}
      if self._in(14) != 0 { try!(write!(f, " _in[14]"))}
      if self._in(15) != 0 { try!(write!(f, " _in[15]"))}
      if self._in(16) != 0 { try!(write!(f, " _in[16]"))}
      if self._in(17) != 0 { try!(write!(f, " _in[17]"))}
      if self._in(18) != 0 { try!(write!(f, " _in[18]"))}
      if self._in(19) != 0 { try!(write!(f, " _in[19]"))}
      if self._in(20) != 0 { try!(write!(f, " _in[20]"))}
      if self._in(21) != 0 { try!(write!(f, " _in[21]"))}
      if self._in(22) != 0 { try!(write!(f, " _in[22]"))}
      if self._in(23) != 0 { try!(write!(f, " _in[23]"))}
      if self._in(24) != 0 { try!(write!(f, " _in[24]"))}
      if self._in(25) != 0 { try!(write!(f, " _in[25]"))}
      if self._in(26) != 0 { try!(write!(f, " _in[26]"))}
      if self._in(27) != 0 { try!(write!(f, " _in[27]"))}
      if self._in(28) != 0 { try!(write!(f, " _in[28]"))}
      if self._in(29) != 0 { try!(write!(f, " _in[29]"))}
      if self._in(30) != 0 { try!(write!(f, " _in[30]"))}
      if self._in(31) != 0 { try!(write!(f, " _in[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Out(pub u32);

impl Out {
  pub fn out(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_out(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Out {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Out {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.out(0) != 0 { try!(write!(f, " out[0]"))}
      if self.out(1) != 0 { try!(write!(f, " out[1]"))}
      if self.out(2) != 0 { try!(write!(f, " out[2]"))}
      if self.out(3) != 0 { try!(write!(f, " out[3]"))}
      if self.out(4) != 0 { try!(write!(f, " out[4]"))}
      if self.out(5) != 0 { try!(write!(f, " out[5]"))}
      if self.out(6) != 0 { try!(write!(f, " out[6]"))}
      if self.out(7) != 0 { try!(write!(f, " out[7]"))}
      if self.out(8) != 0 { try!(write!(f, " out[8]"))}
      if self.out(9) != 0 { try!(write!(f, " out[9]"))}
      if self.out(10) != 0 { try!(write!(f, " out[10]"))}
      if self.out(11) != 0 { try!(write!(f, " out[11]"))}
      if self.out(12) != 0 { try!(write!(f, " out[12]"))}
      if self.out(13) != 0 { try!(write!(f, " out[13]"))}
      if self.out(14) != 0 { try!(write!(f, " out[14]"))}
      if self.out(15) != 0 { try!(write!(f, " out[15]"))}
      if self.out(16) != 0 { try!(write!(f, " out[16]"))}
      if self.out(17) != 0 { try!(write!(f, " out[17]"))}
      if self.out(18) != 0 { try!(write!(f, " out[18]"))}
      if self.out(19) != 0 { try!(write!(f, " out[19]"))}
      if self.out(20) != 0 { try!(write!(f, " out[20]"))}
      if self.out(21) != 0 { try!(write!(f, " out[21]"))}
      if self.out(22) != 0 { try!(write!(f, " out[22]"))}
      if self.out(23) != 0 { try!(write!(f, " out[23]"))}
      if self.out(24) != 0 { try!(write!(f, " out[24]"))}
      if self.out(25) != 0 { try!(write!(f, " out[25]"))}
      if self.out(26) != 0 { try!(write!(f, " out[26]"))}
      if self.out(27) != 0 { try!(write!(f, " out[27]"))}
      if self.out(28) != 0 { try!(write!(f, " out[28]"))}
      if self.out(29) != 0 { try!(write!(f, " out[29]"))}
      if self.out(30) != 0 { try!(write!(f, " out[30]"))}
      if self.out(31) != 0 { try!(write!(f, " out[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Outclr(pub u32);

impl Outclr {
  pub fn outclr(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_outclr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Outclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Outclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.outclr(0) != 0 { try!(write!(f, " outclr[0]"))}
      if self.outclr(1) != 0 { try!(write!(f, " outclr[1]"))}
      if self.outclr(2) != 0 { try!(write!(f, " outclr[2]"))}
      if self.outclr(3) != 0 { try!(write!(f, " outclr[3]"))}
      if self.outclr(4) != 0 { try!(write!(f, " outclr[4]"))}
      if self.outclr(5) != 0 { try!(write!(f, " outclr[5]"))}
      if self.outclr(6) != 0 { try!(write!(f, " outclr[6]"))}
      if self.outclr(7) != 0 { try!(write!(f, " outclr[7]"))}
      if self.outclr(8) != 0 { try!(write!(f, " outclr[8]"))}
      if self.outclr(9) != 0 { try!(write!(f, " outclr[9]"))}
      if self.outclr(10) != 0 { try!(write!(f, " outclr[10]"))}
      if self.outclr(11) != 0 { try!(write!(f, " outclr[11]"))}
      if self.outclr(12) != 0 { try!(write!(f, " outclr[12]"))}
      if self.outclr(13) != 0 { try!(write!(f, " outclr[13]"))}
      if self.outclr(14) != 0 { try!(write!(f, " outclr[14]"))}
      if self.outclr(15) != 0 { try!(write!(f, " outclr[15]"))}
      if self.outclr(16) != 0 { try!(write!(f, " outclr[16]"))}
      if self.outclr(17) != 0 { try!(write!(f, " outclr[17]"))}
      if self.outclr(18) != 0 { try!(write!(f, " outclr[18]"))}
      if self.outclr(19) != 0 { try!(write!(f, " outclr[19]"))}
      if self.outclr(20) != 0 { try!(write!(f, " outclr[20]"))}
      if self.outclr(21) != 0 { try!(write!(f, " outclr[21]"))}
      if self.outclr(22) != 0 { try!(write!(f, " outclr[22]"))}
      if self.outclr(23) != 0 { try!(write!(f, " outclr[23]"))}
      if self.outclr(24) != 0 { try!(write!(f, " outclr[24]"))}
      if self.outclr(25) != 0 { try!(write!(f, " outclr[25]"))}
      if self.outclr(26) != 0 { try!(write!(f, " outclr[26]"))}
      if self.outclr(27) != 0 { try!(write!(f, " outclr[27]"))}
      if self.outclr(28) != 0 { try!(write!(f, " outclr[28]"))}
      if self.outclr(29) != 0 { try!(write!(f, " outclr[29]"))}
      if self.outclr(30) != 0 { try!(write!(f, " outclr[30]"))}
      if self.outclr(31) != 0 { try!(write!(f, " outclr[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Outset(pub u32);

impl Outset {
  pub fn outset(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_outset(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Outset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Outset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.outset(0) != 0 { try!(write!(f, " outset[0]"))}
      if self.outset(1) != 0 { try!(write!(f, " outset[1]"))}
      if self.outset(2) != 0 { try!(write!(f, " outset[2]"))}
      if self.outset(3) != 0 { try!(write!(f, " outset[3]"))}
      if self.outset(4) != 0 { try!(write!(f, " outset[4]"))}
      if self.outset(5) != 0 { try!(write!(f, " outset[5]"))}
      if self.outset(6) != 0 { try!(write!(f, " outset[6]"))}
      if self.outset(7) != 0 { try!(write!(f, " outset[7]"))}
      if self.outset(8) != 0 { try!(write!(f, " outset[8]"))}
      if self.outset(9) != 0 { try!(write!(f, " outset[9]"))}
      if self.outset(10) != 0 { try!(write!(f, " outset[10]"))}
      if self.outset(11) != 0 { try!(write!(f, " outset[11]"))}
      if self.outset(12) != 0 { try!(write!(f, " outset[12]"))}
      if self.outset(13) != 0 { try!(write!(f, " outset[13]"))}
      if self.outset(14) != 0 { try!(write!(f, " outset[14]"))}
      if self.outset(15) != 0 { try!(write!(f, " outset[15]"))}
      if self.outset(16) != 0 { try!(write!(f, " outset[16]"))}
      if self.outset(17) != 0 { try!(write!(f, " outset[17]"))}
      if self.outset(18) != 0 { try!(write!(f, " outset[18]"))}
      if self.outset(19) != 0 { try!(write!(f, " outset[19]"))}
      if self.outset(20) != 0 { try!(write!(f, " outset[20]"))}
      if self.outset(21) != 0 { try!(write!(f, " outset[21]"))}
      if self.outset(22) != 0 { try!(write!(f, " outset[22]"))}
      if self.outset(23) != 0 { try!(write!(f, " outset[23]"))}
      if self.outset(24) != 0 { try!(write!(f, " outset[24]"))}
      if self.outset(25) != 0 { try!(write!(f, " outset[25]"))}
      if self.outset(26) != 0 { try!(write!(f, " outset[26]"))}
      if self.outset(27) != 0 { try!(write!(f, " outset[27]"))}
      if self.outset(28) != 0 { try!(write!(f, " outset[28]"))}
      if self.outset(29) != 0 { try!(write!(f, " outset[29]"))}
      if self.outset(30) != 0 { try!(write!(f, " outset[30]"))}
      if self.outset(31) != 0 { try!(write!(f, " outset[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Outtgl(pub u32);

impl Outtgl {
  pub fn outtgl(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_outtgl(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Outtgl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Outtgl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.outtgl(0) != 0 { try!(write!(f, " outtgl[0]"))}
      if self.outtgl(1) != 0 { try!(write!(f, " outtgl[1]"))}
      if self.outtgl(2) != 0 { try!(write!(f, " outtgl[2]"))}
      if self.outtgl(3) != 0 { try!(write!(f, " outtgl[3]"))}
      if self.outtgl(4) != 0 { try!(write!(f, " outtgl[4]"))}
      if self.outtgl(5) != 0 { try!(write!(f, " outtgl[5]"))}
      if self.outtgl(6) != 0 { try!(write!(f, " outtgl[6]"))}
      if self.outtgl(7) != 0 { try!(write!(f, " outtgl[7]"))}
      if self.outtgl(8) != 0 { try!(write!(f, " outtgl[8]"))}
      if self.outtgl(9) != 0 { try!(write!(f, " outtgl[9]"))}
      if self.outtgl(10) != 0 { try!(write!(f, " outtgl[10]"))}
      if self.outtgl(11) != 0 { try!(write!(f, " outtgl[11]"))}
      if self.outtgl(12) != 0 { try!(write!(f, " outtgl[12]"))}
      if self.outtgl(13) != 0 { try!(write!(f, " outtgl[13]"))}
      if self.outtgl(14) != 0 { try!(write!(f, " outtgl[14]"))}
      if self.outtgl(15) != 0 { try!(write!(f, " outtgl[15]"))}
      if self.outtgl(16) != 0 { try!(write!(f, " outtgl[16]"))}
      if self.outtgl(17) != 0 { try!(write!(f, " outtgl[17]"))}
      if self.outtgl(18) != 0 { try!(write!(f, " outtgl[18]"))}
      if self.outtgl(19) != 0 { try!(write!(f, " outtgl[19]"))}
      if self.outtgl(20) != 0 { try!(write!(f, " outtgl[20]"))}
      if self.outtgl(21) != 0 { try!(write!(f, " outtgl[21]"))}
      if self.outtgl(22) != 0 { try!(write!(f, " outtgl[22]"))}
      if self.outtgl(23) != 0 { try!(write!(f, " outtgl[23]"))}
      if self.outtgl(24) != 0 { try!(write!(f, " outtgl[24]"))}
      if self.outtgl(25) != 0 { try!(write!(f, " outtgl[25]"))}
      if self.outtgl(26) != 0 { try!(write!(f, " outtgl[26]"))}
      if self.outtgl(27) != 0 { try!(write!(f, " outtgl[27]"))}
      if self.outtgl(28) != 0 { try!(write!(f, " outtgl[28]"))}
      if self.outtgl(29) != 0 { try!(write!(f, " outtgl[29]"))}
      if self.outtgl(30) != 0 { try!(write!(f, " outtgl[30]"))}
      if self.outtgl(31) != 0 { try!(write!(f, " outtgl[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Pincfg(pub u8);

impl Pincfg {
  pub fn pmuxen(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_pmuxen(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn inen(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_inen(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn pullen(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  pub fn set_pullen(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn drvstr(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_drvstr(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

}

impl ::core::fmt::Display for Pincfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Pincfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pmuxen() != 0 { try!(write!(f, " pmuxen"))}
      if self.inen() != 0 { try!(write!(f, " inen"))}
      if self.pullen() != 0 { try!(write!(f, " pullen"))}
      if self.drvstr() != 0 { try!(write!(f, " drvstr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Pmux(pub u8);

impl Pmux {
  pub fn pmux(&self, index: usize) -> u8 {
     assert!(index < 2);
     let shift: usize = 0 + (index << 2);
     ((self.0 as u8) >> shift) & 0xf // [3:0]
  }
  pub fn set_pmux(mut self, index: usize, value: u8) -> Self {
     assert!(index < 2);
     assert!((value & !0xf) == 0);
     let shift: usize = 0 + (index << 2);
     self.0 &= !(0xf << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Pmux {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Pmux {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pmux(0) != 0 { try!(write!(f, " pmux[0]=0x{:x}", self.pmux(0)))}
      if self.pmux(1) != 0 { try!(write!(f, " pmux[1]=0x{:x}", self.pmux(1)))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Wrconfig(pub u32);

impl Wrconfig {
  pub fn pinmask(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_pinmask(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn pmuxen(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_pmuxen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn inen(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_inen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn pullen(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_pullen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn drvstr(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_drvstr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn pmux(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  pub fn set_pmux(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

  pub fn wrpmux(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  pub fn set_wrpmux(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn wrpincfg(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_wrpincfg(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn hwsel(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_hwsel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}

impl ::core::fmt::Display for Wrconfig {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Wrconfig {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pinmask() != 0 { try!(write!(f, " pinmask=0x{:x}", self.pinmask()))}
      if self.pmuxen() != 0 { try!(write!(f, " pmuxen"))}
      if self.inen() != 0 { try!(write!(f, " inen"))}
      if self.pullen() != 0 { try!(write!(f, " pullen"))}
      if self.drvstr() != 0 { try!(write!(f, " drvstr"))}
      if self.pmux() != 0 { try!(write!(f, " pmux=0x{:x}", self.pmux()))}
      if self.wrpmux() != 0 { try!(write!(f, " wrpmux"))}
      if self.wrpincfg() != 0 { try!(write!(f, " wrpincfg"))}
      if self.hwsel() != 0 { try!(write!(f, " hwsel"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub trait Pin {
   fn port(&self) -> Port;
   fn index(&self) -> usize;
}

pub trait AfExtint0 {
   fn af_extint_0(&self) -> usize;
}

pub trait AfSercom1Pad0 {
   fn af_sercom1_pad_0(&self) -> usize;
}

pub trait AfTcc2Wo0 {
   fn af_tcc2_wo_0(&self) -> usize;
}

pub trait AfExtint1 {
   fn af_extint_1(&self) -> usize;
}

pub trait AfSercom1Pad1 {
   fn af_sercom1_pad_1(&self) -> usize;
}

pub trait AfTcc2Wo1 {
   fn af_tcc2_wo_1(&self) -> usize;
}

pub trait AfExtint2 {
   fn af_extint_2(&self) -> usize;
}

pub trait AfAin0 {
   fn af_ain_0(&self) -> usize;
}

pub trait AfY0 {
   fn af_y_0(&self) -> usize;
}

pub trait AfVout {
   fn af_vout(&self) -> usize;
}

pub trait AfExtint3 {
   fn af_extint_3(&self) -> usize;
}

pub trait AfAdcVrefadacVrefa {
   fn af_adc_vrefadac_vrefa(&self) -> usize;
}

pub trait AfAin1 {
   fn af_ain_1(&self) -> usize;
}

pub trait AfY1 {
   fn af_y_1(&self) -> usize;
}

pub trait AfExtint4 {
   fn af_extint_4(&self) -> usize;
}

pub trait AfAdcVrefb {
   fn af_adc_vrefb(&self) -> usize;
}

pub trait AfAin4 {
   fn af_ain_4(&self) -> usize;
}

pub trait AfY2 {
   fn af_y_2(&self) -> usize;
}

pub trait AfSercom0Pad0 {
   fn af_sercom0_pad_0(&self) -> usize;
}

pub trait AfTcc0Wo0 {
   fn af_tcc0_wo_0(&self) -> usize;
}

pub trait AfExtint5 {
   fn af_extint_5(&self) -> usize;
}

pub trait AfAin5 {
   fn af_ain_5(&self) -> usize;
}

pub trait AfY3 {
   fn af_y_3(&self) -> usize;
}

pub trait AfSercom0Pad1 {
   fn af_sercom0_pad_1(&self) -> usize;
}

pub trait AfTcc0Wo1 {
   fn af_tcc0_wo_1(&self) -> usize;
}

pub trait AfExtint6 {
   fn af_extint_6(&self) -> usize;
}

pub trait AfAin6 {
   fn af_ain_6(&self) -> usize;
}

pub trait AfAin2 {
   fn af_ain_2(&self) -> usize;
}

pub trait AfY4 {
   fn af_y_4(&self) -> usize;
}

pub trait AfSercom0Pad2 {
   fn af_sercom0_pad_2(&self) -> usize;
}

pub trait AfTcc1Wo0 {
   fn af_tcc1_wo_0(&self) -> usize;
}

pub trait AfExtint7 {
   fn af_extint_7(&self) -> usize;
}

pub trait AfAin7 {
   fn af_ain_7(&self) -> usize;
}

pub trait AfAin3 {
   fn af_ain_3(&self) -> usize;
}

pub trait AfY5 {
   fn af_y_5(&self) -> usize;
}

pub trait AfSercom0Pad3 {
   fn af_sercom0_pad_3(&self) -> usize;
}

pub trait AfTcc1Wo1 {
   fn af_tcc1_wo_1(&self) -> usize;
}

pub trait AfI2sSd0 {
   fn af_i2s_sd_0(&self) -> usize;
}

pub trait AfNmi {
   fn af_nmi(&self) -> usize;
}

pub trait AfAin16 {
   fn af_ain_16(&self) -> usize;
}

pub trait AfX0 {
   fn af_x_0(&self) -> usize;
}

pub trait AfSercom2Pad0 {
   fn af_sercom2_pad_0(&self) -> usize;
}

pub trait AfTcc1Wo2 {
   fn af_tcc1_wo_2(&self) -> usize;
}

pub trait AfI2sSd1 {
   fn af_i2s_sd_1(&self) -> usize;
}

pub trait AfExtint9 {
   fn af_extint_9(&self) -> usize;
}

pub trait AfAin17 {
   fn af_ain_17(&self) -> usize;
}

pub trait AfX1 {
   fn af_x_1(&self) -> usize;
}

pub trait AfSercom2Pad1 {
   fn af_sercom2_pad_1(&self) -> usize;
}

pub trait AfTcc1Wo3 {
   fn af_tcc1_wo_3(&self) -> usize;
}

pub trait AfI2sMck0 {
   fn af_i2s_mck_0(&self) -> usize;
}

pub trait AfExtint10 {
   fn af_extint_10(&self) -> usize;
}

pub trait AfAin18 {
   fn af_ain_18(&self) -> usize;
}

pub trait AfX2 {
   fn af_x_2(&self) -> usize;
}

pub trait AfSercom2Pad2 {
   fn af_sercom2_pad_2(&self) -> usize;
}

pub trait AfTcc0Wo2 {
   fn af_tcc0_wo_2(&self) -> usize;
}

pub trait AfI2sSck0 {
   fn af_i2s_sck_0(&self) -> usize;
}

pub trait AfGclkIo4 {
   fn af_gclk_io_4(&self) -> usize;
}

pub trait AfExtint11 {
   fn af_extint_11(&self) -> usize;
}

pub trait AfAin19 {
   fn af_ain_19(&self) -> usize;
}

pub trait AfX3 {
   fn af_x_3(&self) -> usize;
}

pub trait AfSercom2Pad3 {
   fn af_sercom2_pad_3(&self) -> usize;
}

pub trait AfTcc0Wo3 {
   fn af_tcc0_wo_3(&self) -> usize;
}

pub trait AfI2sFs0 {
   fn af_i2s_fs_0(&self) -> usize;
}

pub trait AfGclkIo5 {
   fn af_gclk_io_5(&self) -> usize;
}

pub trait AfExtint12 {
   fn af_extint_12(&self) -> usize;
}

pub trait AfSercom4Pad0 {
   fn af_sercom4_pad_0(&self) -> usize;
}

pub trait AfTcc0Wo6 {
   fn af_tcc0_wo_6(&self) -> usize;
}

pub trait AfAcCmp0 {
   fn af_ac_cmp_0(&self) -> usize;
}

pub trait AfExtint13 {
   fn af_extint_13(&self) -> usize;
}

pub trait AfSercom4Pad1 {
   fn af_sercom4_pad_1(&self) -> usize;
}

pub trait AfTcc0Wo7 {
   fn af_tcc0_wo_7(&self) -> usize;
}

pub trait AfAcCmp1 {
   fn af_ac_cmp_1(&self) -> usize;
}

pub trait AfExtint14 {
   fn af_extint_14(&self) -> usize;
}

pub trait AfSercom4Pad2 {
   fn af_sercom4_pad_2(&self) -> usize;
}

pub trait AfTc3Wo0 {
   fn af_tc3_wo_0(&self) -> usize;
}

pub trait AfTcc0Wo4 {
   fn af_tcc0_wo_4(&self) -> usize;
}

pub trait AfGclkIo0 {
   fn af_gclk_io_0(&self) -> usize;
}

pub trait AfExtint15 {
   fn af_extint_15(&self) -> usize;
}

pub trait AfSercom4Pad3 {
   fn af_sercom4_pad_3(&self) -> usize;
}

pub trait AfTc3Wo1 {
   fn af_tc3_wo_1(&self) -> usize;
}

pub trait AfTcc0Wo5 {
   fn af_tcc0_wo_5(&self) -> usize;
}

pub trait AfGclkIo1 {
   fn af_gclk_io_1(&self) -> usize;
}

pub trait AfX4 {
   fn af_x_4(&self) -> usize;
}

pub trait AfSercom3Pad0 {
   fn af_sercom3_pad_0(&self) -> usize;
}

pub trait AfGclkIo2 {
   fn af_gclk_io_2(&self) -> usize;
}

pub trait AfX5 {
   fn af_x_5(&self) -> usize;
}

pub trait AfSercom3Pad1 {
   fn af_sercom3_pad_1(&self) -> usize;
}

pub trait AfGclkIo3 {
   fn af_gclk_io_3(&self) -> usize;
}

pub trait AfX6 {
   fn af_x_6(&self) -> usize;
}

pub trait AfSercom1Pad2 {
   fn af_sercom1_pad_2(&self) -> usize;
}

pub trait AfSercom3Pad2 {
   fn af_sercom3_pad_2(&self) -> usize;
}

pub trait AfX7 {
   fn af_x_7(&self) -> usize;
}

pub trait AfSercom1Pad3 {
   fn af_sercom1_pad_3(&self) -> usize;
}

pub trait AfSercom3Pad3 {
   fn af_sercom3_pad_3(&self) -> usize;
}

pub trait AfX8 {
   fn af_x_8(&self) -> usize;
}

pub trait AfSercom5Pad2 {
   fn af_sercom5_pad_2(&self) -> usize;
}

pub trait AfTc7Wo0 {
   fn af_tc7_wo_0(&self) -> usize;
}

pub trait AfX9 {
   fn af_x_9(&self) -> usize;
}

pub trait AfSercom5Pad3 {
   fn af_sercom5_pad_3(&self) -> usize;
}

pub trait AfTc7Wo1 {
   fn af_tc7_wo_1(&self) -> usize;
}

pub trait AfX10 {
   fn af_x_10(&self) -> usize;
}

pub trait AfSercom5Pad0 {
   fn af_sercom5_pad_0(&self) -> usize;
}

pub trait AfTc4Wo0 {
   fn af_tc4_wo_0(&self) -> usize;
}

pub trait AfGclkIo6 {
   fn af_gclk_io_6(&self) -> usize;
}

pub trait AfX11 {
   fn af_x_11(&self) -> usize;
}

pub trait AfSercom5Pad1 {
   fn af_sercom5_pad_1(&self) -> usize;
}

pub trait AfTc4Wo1 {
   fn af_tc4_wo_1(&self) -> usize;
}

pub trait AfUsbSof1khz {
   fn af_usb_sof1khz(&self) -> usize;
}

pub trait AfGclkIo7 {
   fn af_gclk_io_7(&self) -> usize;
}

pub trait AfTc5Wo0 {
   fn af_tc5_wo_0(&self) -> usize;
}

pub trait AfUsbDm {
   fn af_usb_dm(&self) -> usize;
}

pub trait AfTc5Wo1 {
   fn af_tc5_wo_1(&self) -> usize;
}

pub trait AfUsbDp {
   fn af_usb_dp(&self) -> usize;
}

pub trait AfExtint8 {
   fn af_extint_8(&self) -> usize;
}

pub trait AfSwclk {
   fn af_swclk(&self) -> usize;
}

pub trait AfSwdio {
   fn af_swdio(&self) -> usize;
}

pub trait AfAin8 {
   fn af_ain_8(&self) -> usize;
}

pub trait AfY6 {
   fn af_y_6(&self) -> usize;
}

pub trait AfAin9 {
   fn af_ain_9(&self) -> usize;
}

pub trait AfY7 {
   fn af_y_7(&self) -> usize;
}

pub trait AfAin10 {
   fn af_ain_10(&self) -> usize;
}

pub trait AfY8 {
   fn af_y_8(&self) -> usize;
}

pub trait AfTc6Wo0 {
   fn af_tc6_wo_0(&self) -> usize;
}

pub trait AfAin11 {
   fn af_ain_11(&self) -> usize;
}

pub trait AfY9 {
   fn af_y_9(&self) -> usize;
}

pub trait AfTc6Wo1 {
   fn af_tc6_wo_1(&self) -> usize;
}

pub trait AfAin12 {
   fn af_ain_12(&self) -> usize;
}

pub trait AfY10 {
   fn af_y_10(&self) -> usize;
}

pub trait AfAin13 {
   fn af_ain_13(&self) -> usize;
}

pub trait AfY11 {
   fn af_y_11(&self) -> usize;
}

pub trait AfAin14 {
   fn af_ain_14(&self) -> usize;
}

pub trait AfY12 {
   fn af_y_12(&self) -> usize;
}

pub trait AfAin15 {
   fn af_ain_15(&self) -> usize;
}

pub trait AfY13 {
   fn af_y_13(&self) -> usize;
}

pub trait AfY14 {
   fn af_y_14(&self) -> usize;
}

pub trait AfY15 {
   fn af_y_15(&self) -> usize;
}

pub trait AfI2sMck1 {
   fn af_i2s_mck_1(&self) -> usize;
}

pub trait AfI2sSck1 {
   fn af_i2s_sck_1(&self) -> usize;
}

pub trait AfX12 {
   fn af_x_12(&self) -> usize;
}

pub trait AfI2sFs1 {
   fn af_i2s_fs_1(&self) -> usize;
}

pub trait AfX13 {
   fn af_x_13(&self) -> usize;
}

pub trait AfX14 {
   fn af_x_14(&self) -> usize;
}

pub trait AfX15 {
   fn af_x_15(&self) -> usize;
}

pub const PA00: Pa00 = Pa00 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa00 {}

impl Pin for Pa00 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 0 }
}

impl AfExtint0 for Pa00 {
   fn af_extint_0(&self) -> usize { 0 }
}

impl AfSercom1Pad0 for Pa00 {
   fn af_sercom1_pad_0(&self) -> usize { 2 }
}

impl AfTcc2Wo0 for Pa00 {
   fn af_tcc2_wo_0(&self) -> usize { 3 }
}

pub const PA01: Pa01 = Pa01 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa01 {}

impl Pin for Pa01 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 1 }
}

impl AfExtint1 for Pa01 {
   fn af_extint_1(&self) -> usize { 0 }
}

impl AfSercom1Pad1 for Pa01 {
   fn af_sercom1_pad_1(&self) -> usize { 2 }
}

impl AfTcc2Wo1 for Pa01 {
   fn af_tcc2_wo_1(&self) -> usize { 3 }
}

pub const PA02: Pa02 = Pa02 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa02 {}

impl Pin for Pa02 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 2 }
}

impl AfExtint2 for Pa02 {
   fn af_extint_2(&self) -> usize { 0 }
}

impl AfAin0 for Pa02 {
   fn af_ain_0(&self) -> usize { 1 }
}

impl AfY0 for Pa02 {
   fn af_y_0(&self) -> usize { 1 }
}

impl AfVout for Pa02 {
   fn af_vout(&self) -> usize { 1 }
}

pub const PA03: Pa03 = Pa03 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa03 {}

impl Pin for Pa03 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 3 }
}

impl AfExtint3 for Pa03 {
   fn af_extint_3(&self) -> usize { 0 }
}

impl AfAdcVrefadacVrefa for Pa03 {
   fn af_adc_vrefadac_vrefa(&self) -> usize { 1 }
}

impl AfAin1 for Pa03 {
   fn af_ain_1(&self) -> usize { 1 }
}

impl AfY1 for Pa03 {
   fn af_y_1(&self) -> usize { 1 }
}

pub const PA04: Pa04 = Pa04 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa04 {}

impl Pin for Pa04 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 4 }
}

impl AfExtint4 for Pa04 {
   fn af_extint_4(&self) -> usize { 0 }
}

impl AfAdcVrefb for Pa04 {
   fn af_adc_vrefb(&self) -> usize { 1 }
}

impl AfAin4 for Pa04 {
   fn af_ain_4(&self) -> usize { 1 }
}

impl AfAin0 for Pa04 {
   fn af_ain_0(&self) -> usize { 1 }
}

impl AfY2 for Pa04 {
   fn af_y_2(&self) -> usize { 1 }
}

impl AfSercom0Pad0 for Pa04 {
   fn af_sercom0_pad_0(&self) -> usize { 2 }
}

impl AfTcc0Wo0 for Pa04 {
   fn af_tcc0_wo_0(&self) -> usize { 3 }
}

pub const PA05: Pa05 = Pa05 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa05 {}

impl Pin for Pa05 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 5 }
}

impl AfExtint5 for Pa05 {
   fn af_extint_5(&self) -> usize { 0 }
}

impl AfAin5 for Pa05 {
   fn af_ain_5(&self) -> usize { 1 }
}

impl AfAin1 for Pa05 {
   fn af_ain_1(&self) -> usize { 1 }
}

impl AfY3 for Pa05 {
   fn af_y_3(&self) -> usize { 1 }
}

impl AfSercom0Pad1 for Pa05 {
   fn af_sercom0_pad_1(&self) -> usize { 2 }
}

impl AfTcc0Wo1 for Pa05 {
   fn af_tcc0_wo_1(&self) -> usize { 3 }
}

pub const PA06: Pa06 = Pa06 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa06 {}

impl Pin for Pa06 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 6 }
}

impl AfExtint6 for Pa06 {
   fn af_extint_6(&self) -> usize { 0 }
}

impl AfAin6 for Pa06 {
   fn af_ain_6(&self) -> usize { 1 }
}

impl AfAin2 for Pa06 {
   fn af_ain_2(&self) -> usize { 1 }
}

impl AfY4 for Pa06 {
   fn af_y_4(&self) -> usize { 1 }
}

impl AfSercom0Pad2 for Pa06 {
   fn af_sercom0_pad_2(&self) -> usize { 2 }
}

impl AfTcc1Wo0 for Pa06 {
   fn af_tcc1_wo_0(&self) -> usize { 3 }
}

pub const PA07: Pa07 = Pa07 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa07 {}

impl Pin for Pa07 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 7 }
}

impl AfExtint7 for Pa07 {
   fn af_extint_7(&self) -> usize { 0 }
}

impl AfAin7 for Pa07 {
   fn af_ain_7(&self) -> usize { 1 }
}

impl AfAin3 for Pa07 {
   fn af_ain_3(&self) -> usize { 1 }
}

impl AfY5 for Pa07 {
   fn af_y_5(&self) -> usize { 1 }
}

impl AfSercom0Pad3 for Pa07 {
   fn af_sercom0_pad_3(&self) -> usize { 2 }
}

impl AfTcc1Wo1 for Pa07 {
   fn af_tcc1_wo_1(&self) -> usize { 3 }
}

impl AfI2sSd0 for Pa07 {
   fn af_i2s_sd_0(&self) -> usize { 5 }
}

pub const PA08: Pa08 = Pa08 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa08 {}

impl Pin for Pa08 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 8 }
}

impl AfNmi for Pa08 {
   fn af_nmi(&self) -> usize { 0 }
}

impl AfAin16 for Pa08 {
   fn af_ain_16(&self) -> usize { 1 }
}

impl AfX0 for Pa08 {
   fn af_x_0(&self) -> usize { 1 }
}

impl AfSercom0Pad0 for Pa08 {
   fn af_sercom0_pad_0(&self) -> usize { 1 }
}

impl AfSercom2Pad0 for Pa08 {
   fn af_sercom2_pad_0(&self) -> usize { 2 }
}

impl AfTcc0Wo0 for Pa08 {
   fn af_tcc0_wo_0(&self) -> usize { 3 }
}

impl AfTcc1Wo2 for Pa08 {
   fn af_tcc1_wo_2(&self) -> usize { 4 }
}

impl AfI2sSd1 for Pa08 {
   fn af_i2s_sd_1(&self) -> usize { 5 }
}

pub const PA09: Pa09 = Pa09 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa09 {}

impl Pin for Pa09 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 9 }
}

impl AfExtint9 for Pa09 {
   fn af_extint_9(&self) -> usize { 0 }
}

impl AfAin17 for Pa09 {
   fn af_ain_17(&self) -> usize { 1 }
}

impl AfX1 for Pa09 {
   fn af_x_1(&self) -> usize { 1 }
}

impl AfSercom0Pad1 for Pa09 {
   fn af_sercom0_pad_1(&self) -> usize { 1 }
}

impl AfSercom2Pad1 for Pa09 {
   fn af_sercom2_pad_1(&self) -> usize { 2 }
}

impl AfTcc0Wo1 for Pa09 {
   fn af_tcc0_wo_1(&self) -> usize { 3 }
}

impl AfTcc1Wo3 for Pa09 {
   fn af_tcc1_wo_3(&self) -> usize { 4 }
}

impl AfI2sMck0 for Pa09 {
   fn af_i2s_mck_0(&self) -> usize { 5 }
}

pub const PA10: Pa10 = Pa10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa10 {}

impl Pin for Pa10 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 10 }
}

impl AfExtint10 for Pa10 {
   fn af_extint_10(&self) -> usize { 0 }
}

impl AfAin18 for Pa10 {
   fn af_ain_18(&self) -> usize { 1 }
}

impl AfX2 for Pa10 {
   fn af_x_2(&self) -> usize { 1 }
}

impl AfSercom0Pad2 for Pa10 {
   fn af_sercom0_pad_2(&self) -> usize { 1 }
}

impl AfSercom2Pad2 for Pa10 {
   fn af_sercom2_pad_2(&self) -> usize { 2 }
}

impl AfTcc1Wo0 for Pa10 {
   fn af_tcc1_wo_0(&self) -> usize { 3 }
}

impl AfTcc0Wo2 for Pa10 {
   fn af_tcc0_wo_2(&self) -> usize { 4 }
}

impl AfI2sSck0 for Pa10 {
   fn af_i2s_sck_0(&self) -> usize { 5 }
}

impl AfGclkIo4 for Pa10 {
   fn af_gclk_io_4(&self) -> usize { 6 }
}

pub const PA11: Pa11 = Pa11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa11 {}

impl Pin for Pa11 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 11 }
}

impl AfExtint11 for Pa11 {
   fn af_extint_11(&self) -> usize { 0 }
}

impl AfAin19 for Pa11 {
   fn af_ain_19(&self) -> usize { 1 }
}

impl AfX3 for Pa11 {
   fn af_x_3(&self) -> usize { 1 }
}

impl AfSercom0Pad3 for Pa11 {
   fn af_sercom0_pad_3(&self) -> usize { 1 }
}

impl AfSercom2Pad3 for Pa11 {
   fn af_sercom2_pad_3(&self) -> usize { 2 }
}

impl AfTcc1Wo1 for Pa11 {
   fn af_tcc1_wo_1(&self) -> usize { 3 }
}

impl AfTcc0Wo3 for Pa11 {
   fn af_tcc0_wo_3(&self) -> usize { 4 }
}

impl AfI2sFs0 for Pa11 {
   fn af_i2s_fs_0(&self) -> usize { 5 }
}

impl AfGclkIo5 for Pa11 {
   fn af_gclk_io_5(&self) -> usize { 6 }
}

pub const PA12: Pa12 = Pa12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa12 {}

impl Pin for Pa12 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 12 }
}

impl AfExtint12 for Pa12 {
   fn af_extint_12(&self) -> usize { 0 }
}

impl AfSercom2Pad0 for Pa12 {
   fn af_sercom2_pad_0(&self) -> usize { 1 }
}

impl AfSercom4Pad0 for Pa12 {
   fn af_sercom4_pad_0(&self) -> usize { 2 }
}

impl AfTcc2Wo0 for Pa12 {
   fn af_tcc2_wo_0(&self) -> usize { 3 }
}

impl AfTcc0Wo6 for Pa12 {
   fn af_tcc0_wo_6(&self) -> usize { 4 }
}

impl AfAcCmp0 for Pa12 {
   fn af_ac_cmp_0(&self) -> usize { 6 }
}

pub const PA13: Pa13 = Pa13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa13 {}

impl Pin for Pa13 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 13 }
}

impl AfExtint13 for Pa13 {
   fn af_extint_13(&self) -> usize { 0 }
}

impl AfSercom2Pad1 for Pa13 {
   fn af_sercom2_pad_1(&self) -> usize { 1 }
}

impl AfSercom4Pad1 for Pa13 {
   fn af_sercom4_pad_1(&self) -> usize { 2 }
}

impl AfTcc2Wo1 for Pa13 {
   fn af_tcc2_wo_1(&self) -> usize { 3 }
}

impl AfTcc0Wo7 for Pa13 {
   fn af_tcc0_wo_7(&self) -> usize { 4 }
}

impl AfAcCmp1 for Pa13 {
   fn af_ac_cmp_1(&self) -> usize { 6 }
}

pub const PA14: Pa14 = Pa14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa14 {}

impl Pin for Pa14 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 14 }
}

impl AfExtint14 for Pa14 {
   fn af_extint_14(&self) -> usize { 0 }
}

impl AfSercom2Pad2 for Pa14 {
   fn af_sercom2_pad_2(&self) -> usize { 1 }
}

impl AfSercom4Pad2 for Pa14 {
   fn af_sercom4_pad_2(&self) -> usize { 2 }
}

impl AfTc3Wo0 for Pa14 {
   fn af_tc3_wo_0(&self) -> usize { 3 }
}

impl AfTcc0Wo4 for Pa14 {
   fn af_tcc0_wo_4(&self) -> usize { 4 }
}

impl AfGclkIo0 for Pa14 {
   fn af_gclk_io_0(&self) -> usize { 6 }
}

pub const PA15: Pa15 = Pa15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa15 {}

impl Pin for Pa15 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 15 }
}

impl AfExtint15 for Pa15 {
   fn af_extint_15(&self) -> usize { 0 }
}

impl AfSercom2Pad3 for Pa15 {
   fn af_sercom2_pad_3(&self) -> usize { 1 }
}

impl AfSercom4Pad3 for Pa15 {
   fn af_sercom4_pad_3(&self) -> usize { 2 }
}

impl AfTc3Wo1 for Pa15 {
   fn af_tc3_wo_1(&self) -> usize { 3 }
}

impl AfTcc0Wo5 for Pa15 {
   fn af_tcc0_wo_5(&self) -> usize { 4 }
}

impl AfGclkIo1 for Pa15 {
   fn af_gclk_io_1(&self) -> usize { 6 }
}

pub const PA16: Pa16 = Pa16 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa16 {}

impl Pin for Pa16 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 16 }
}

impl AfExtint0 for Pa16 {
   fn af_extint_0(&self) -> usize { 0 }
}

impl AfX4 for Pa16 {
   fn af_x_4(&self) -> usize { 1 }
}

impl AfSercom1Pad0 for Pa16 {
   fn af_sercom1_pad_0(&self) -> usize { 1 }
}

impl AfSercom3Pad0 for Pa16 {
   fn af_sercom3_pad_0(&self) -> usize { 2 }
}

impl AfTcc2Wo0 for Pa16 {
   fn af_tcc2_wo_0(&self) -> usize { 3 }
}

impl AfTcc0Wo6 for Pa16 {
   fn af_tcc0_wo_6(&self) -> usize { 4 }
}

impl AfGclkIo2 for Pa16 {
   fn af_gclk_io_2(&self) -> usize { 6 }
}

pub const PA17: Pa17 = Pa17 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa17 {}

impl Pin for Pa17 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 17 }
}

impl AfExtint1 for Pa17 {
   fn af_extint_1(&self) -> usize { 0 }
}

impl AfX5 for Pa17 {
   fn af_x_5(&self) -> usize { 1 }
}

impl AfSercom1Pad1 for Pa17 {
   fn af_sercom1_pad_1(&self) -> usize { 1 }
}

impl AfSercom3Pad1 for Pa17 {
   fn af_sercom3_pad_1(&self) -> usize { 2 }
}

impl AfTcc2Wo1 for Pa17 {
   fn af_tcc2_wo_1(&self) -> usize { 3 }
}

impl AfTcc0Wo7 for Pa17 {
   fn af_tcc0_wo_7(&self) -> usize { 4 }
}

impl AfGclkIo3 for Pa17 {
   fn af_gclk_io_3(&self) -> usize { 6 }
}

pub const PA18: Pa18 = Pa18 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa18 {}

impl Pin for Pa18 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 18 }
}

impl AfExtint2 for Pa18 {
   fn af_extint_2(&self) -> usize { 0 }
}

impl AfX6 for Pa18 {
   fn af_x_6(&self) -> usize { 1 }
}

impl AfSercom1Pad2 for Pa18 {
   fn af_sercom1_pad_2(&self) -> usize { 1 }
}

impl AfSercom3Pad2 for Pa18 {
   fn af_sercom3_pad_2(&self) -> usize { 2 }
}

impl AfTc3Wo0 for Pa18 {
   fn af_tc3_wo_0(&self) -> usize { 3 }
}

impl AfTcc0Wo2 for Pa18 {
   fn af_tcc0_wo_2(&self) -> usize { 4 }
}

impl AfAcCmp0 for Pa18 {
   fn af_ac_cmp_0(&self) -> usize { 6 }
}

pub const PA19: Pa19 = Pa19 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa19 {}

impl Pin for Pa19 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 19 }
}

impl AfExtint3 for Pa19 {
   fn af_extint_3(&self) -> usize { 0 }
}

impl AfX7 for Pa19 {
   fn af_x_7(&self) -> usize { 1 }
}

impl AfSercom1Pad3 for Pa19 {
   fn af_sercom1_pad_3(&self) -> usize { 1 }
}

impl AfSercom3Pad3 for Pa19 {
   fn af_sercom3_pad_3(&self) -> usize { 2 }
}

impl AfTc3Wo1 for Pa19 {
   fn af_tc3_wo_1(&self) -> usize { 3 }
}

impl AfTcc0Wo3 for Pa19 {
   fn af_tcc0_wo_3(&self) -> usize { 4 }
}

impl AfI2sSd0 for Pa19 {
   fn af_i2s_sd_0(&self) -> usize { 5 }
}

impl AfAcCmp1 for Pa19 {
   fn af_ac_cmp_1(&self) -> usize { 6 }
}

pub const PA20: Pa20 = Pa20 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa20 {}

impl Pin for Pa20 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 20 }
}

impl AfExtint4 for Pa20 {
   fn af_extint_4(&self) -> usize { 0 }
}

impl AfX8 for Pa20 {
   fn af_x_8(&self) -> usize { 1 }
}

impl AfSercom5Pad2 for Pa20 {
   fn af_sercom5_pad_2(&self) -> usize { 1 }
}

impl AfSercom3Pad2 for Pa20 {
   fn af_sercom3_pad_2(&self) -> usize { 2 }
}

impl AfTc7Wo0 for Pa20 {
   fn af_tc7_wo_0(&self) -> usize { 3 }
}

impl AfTcc0Wo6 for Pa20 {
   fn af_tcc0_wo_6(&self) -> usize { 4 }
}

impl AfI2sSck0 for Pa20 {
   fn af_i2s_sck_0(&self) -> usize { 5 }
}

impl AfGclkIo4 for Pa20 {
   fn af_gclk_io_4(&self) -> usize { 6 }
}

pub const PA21: Pa21 = Pa21 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa21 {}

impl Pin for Pa21 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 21 }
}

impl AfExtint5 for Pa21 {
   fn af_extint_5(&self) -> usize { 0 }
}

impl AfX9 for Pa21 {
   fn af_x_9(&self) -> usize { 1 }
}

impl AfSercom5Pad3 for Pa21 {
   fn af_sercom5_pad_3(&self) -> usize { 1 }
}

impl AfSercom3Pad3 for Pa21 {
   fn af_sercom3_pad_3(&self) -> usize { 2 }
}

impl AfTc7Wo1 for Pa21 {
   fn af_tc7_wo_1(&self) -> usize { 3 }
}

impl AfTcc0Wo7 for Pa21 {
   fn af_tcc0_wo_7(&self) -> usize { 4 }
}

impl AfI2sFs0 for Pa21 {
   fn af_i2s_fs_0(&self) -> usize { 5 }
}

impl AfGclkIo5 for Pa21 {
   fn af_gclk_io_5(&self) -> usize { 6 }
}

pub const PA22: Pa22 = Pa22 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa22 {}

impl Pin for Pa22 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 22 }
}

impl AfExtint6 for Pa22 {
   fn af_extint_6(&self) -> usize { 0 }
}

impl AfX10 for Pa22 {
   fn af_x_10(&self) -> usize { 1 }
}

impl AfSercom3Pad0 for Pa22 {
   fn af_sercom3_pad_0(&self) -> usize { 1 }
}

impl AfSercom5Pad0 for Pa22 {
   fn af_sercom5_pad_0(&self) -> usize { 2 }
}

impl AfTc4Wo0 for Pa22 {
   fn af_tc4_wo_0(&self) -> usize { 3 }
}

impl AfTcc0Wo4 for Pa22 {
   fn af_tcc0_wo_4(&self) -> usize { 4 }
}

impl AfGclkIo6 for Pa22 {
   fn af_gclk_io_6(&self) -> usize { 6 }
}

pub const PA23: Pa23 = Pa23 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa23 {}

impl Pin for Pa23 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 23 }
}

impl AfExtint7 for Pa23 {
   fn af_extint_7(&self) -> usize { 0 }
}

impl AfX11 for Pa23 {
   fn af_x_11(&self) -> usize { 1 }
}

impl AfSercom3Pad1 for Pa23 {
   fn af_sercom3_pad_1(&self) -> usize { 1 }
}

impl AfSercom5Pad1 for Pa23 {
   fn af_sercom5_pad_1(&self) -> usize { 2 }
}

impl AfTc4Wo1 for Pa23 {
   fn af_tc4_wo_1(&self) -> usize { 3 }
}

impl AfTcc0Wo5 for Pa23 {
   fn af_tcc0_wo_5(&self) -> usize { 4 }
}

impl AfUsbSof1khz for Pa23 {
   fn af_usb_sof1khz(&self) -> usize { 5 }
}

impl AfGclkIo7 for Pa23 {
   fn af_gclk_io_7(&self) -> usize { 6 }
}

pub const PA24: Pa24 = Pa24 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa24 {}

impl Pin for Pa24 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 24 }
}

impl AfExtint12 for Pa24 {
   fn af_extint_12(&self) -> usize { 0 }
}

impl AfSercom3Pad2 for Pa24 {
   fn af_sercom3_pad_2(&self) -> usize { 1 }
}

impl AfSercom5Pad2 for Pa24 {
   fn af_sercom5_pad_2(&self) -> usize { 2 }
}

impl AfTc5Wo0 for Pa24 {
   fn af_tc5_wo_0(&self) -> usize { 3 }
}

impl AfTcc1Wo2 for Pa24 {
   fn af_tcc1_wo_2(&self) -> usize { 4 }
}

impl AfUsbDm for Pa24 {
   fn af_usb_dm(&self) -> usize { 5 }
}

pub const PA25: Pa25 = Pa25 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa25 {}

impl Pin for Pa25 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 25 }
}

impl AfExtint13 for Pa25 {
   fn af_extint_13(&self) -> usize { 0 }
}

impl AfSercom3Pad3 for Pa25 {
   fn af_sercom3_pad_3(&self) -> usize { 1 }
}

impl AfSercom5Pad3 for Pa25 {
   fn af_sercom5_pad_3(&self) -> usize { 2 }
}

impl AfTc5Wo1 for Pa25 {
   fn af_tc5_wo_1(&self) -> usize { 3 }
}

impl AfTcc1Wo3 for Pa25 {
   fn af_tcc1_wo_3(&self) -> usize { 4 }
}

impl AfUsbDp for Pa25 {
   fn af_usb_dp(&self) -> usize { 5 }
}

pub const PA27: Pa27 = Pa27 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa27 {}

impl Pin for Pa27 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 27 }
}

impl AfExtint15 for Pa27 {
   fn af_extint_15(&self) -> usize { 0 }
}

impl AfGclkIo0 for Pa27 {
   fn af_gclk_io_0(&self) -> usize { 6 }
}

pub const PA28: Pa28 = Pa28 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa28 {}

impl Pin for Pa28 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 28 }
}

impl AfExtint8 for Pa28 {
   fn af_extint_8(&self) -> usize { 0 }
}

impl AfGclkIo0 for Pa28 {
   fn af_gclk_io_0(&self) -> usize { 6 }
}

pub const PA30: Pa30 = Pa30 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa30 {}

impl Pin for Pa30 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 30 }
}

impl AfExtint10 for Pa30 {
   fn af_extint_10(&self) -> usize { 0 }
}

impl AfSercom1Pad2 for Pa30 {
   fn af_sercom1_pad_2(&self) -> usize { 2 }
}

impl AfTcc1Wo0 for Pa30 {
   fn af_tcc1_wo_0(&self) -> usize { 3 }
}

impl AfSwclk for Pa30 {
   fn af_swclk(&self) -> usize { 5 }
}

impl AfGclkIo0 for Pa30 {
   fn af_gclk_io_0(&self) -> usize { 6 }
}

pub const PA31: Pa31 = Pa31 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa31 {}

impl Pin for Pa31 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 31 }
}

impl AfExtint11 for Pa31 {
   fn af_extint_11(&self) -> usize { 0 }
}

impl AfSercom1Pad3 for Pa31 {
   fn af_sercom1_pad_3(&self) -> usize { 2 }
}

impl AfTcc1Wo1 for Pa31 {
   fn af_tcc1_wo_1(&self) -> usize { 3 }
}

impl AfSwdio for Pa31 {
   fn af_swdio(&self) -> usize { 5 }
}

pub const PB00: Pb00 = Pb00 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb00 {}

impl Pin for Pb00 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 0 }
}

impl AfExtint0 for Pb00 {
   fn af_extint_0(&self) -> usize { 0 }
}

impl AfAin8 for Pb00 {
   fn af_ain_8(&self) -> usize { 1 }
}

impl AfY6 for Pb00 {
   fn af_y_6(&self) -> usize { 1 }
}

impl AfSercom5Pad2 for Pb00 {
   fn af_sercom5_pad_2(&self) -> usize { 2 }
}

impl AfTc7Wo0 for Pb00 {
   fn af_tc7_wo_0(&self) -> usize { 3 }
}

pub const PB01: Pb01 = Pb01 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb01 {}

impl Pin for Pb01 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 1 }
}

impl AfExtint1 for Pb01 {
   fn af_extint_1(&self) -> usize { 0 }
}

impl AfAin9 for Pb01 {
   fn af_ain_9(&self) -> usize { 1 }
}

impl AfY7 for Pb01 {
   fn af_y_7(&self) -> usize { 1 }
}

impl AfSercom5Pad3 for Pb01 {
   fn af_sercom5_pad_3(&self) -> usize { 2 }
}

impl AfTc7Wo1 for Pb01 {
   fn af_tc7_wo_1(&self) -> usize { 3 }
}

pub const PB02: Pb02 = Pb02 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb02 {}

impl Pin for Pb02 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 2 }
}

impl AfExtint2 for Pb02 {
   fn af_extint_2(&self) -> usize { 0 }
}

impl AfAin10 for Pb02 {
   fn af_ain_10(&self) -> usize { 1 }
}

impl AfY8 for Pb02 {
   fn af_y_8(&self) -> usize { 1 }
}

impl AfSercom5Pad0 for Pb02 {
   fn af_sercom5_pad_0(&self) -> usize { 2 }
}

impl AfTc6Wo0 for Pb02 {
   fn af_tc6_wo_0(&self) -> usize { 3 }
}

pub const PB03: Pb03 = Pb03 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb03 {}

impl Pin for Pb03 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 3 }
}

impl AfExtint3 for Pb03 {
   fn af_extint_3(&self) -> usize { 0 }
}

impl AfAin11 for Pb03 {
   fn af_ain_11(&self) -> usize { 1 }
}

impl AfY9 for Pb03 {
   fn af_y_9(&self) -> usize { 1 }
}

impl AfSercom5Pad1 for Pb03 {
   fn af_sercom5_pad_1(&self) -> usize { 2 }
}

impl AfTc6Wo1 for Pb03 {
   fn af_tc6_wo_1(&self) -> usize { 3 }
}

pub const PB04: Pb04 = Pb04 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb04 {}

impl Pin for Pb04 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 4 }
}

impl AfExtint4 for Pb04 {
   fn af_extint_4(&self) -> usize { 0 }
}

impl AfAin12 for Pb04 {
   fn af_ain_12(&self) -> usize { 1 }
}

impl AfY10 for Pb04 {
   fn af_y_10(&self) -> usize { 1 }
}

pub const PB05: Pb05 = Pb05 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb05 {}

impl Pin for Pb05 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 5 }
}

impl AfExtint5 for Pb05 {
   fn af_extint_5(&self) -> usize { 0 }
}

impl AfAin13 for Pb05 {
   fn af_ain_13(&self) -> usize { 1 }
}

impl AfY11 for Pb05 {
   fn af_y_11(&self) -> usize { 1 }
}

pub const PB06: Pb06 = Pb06 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb06 {}

impl Pin for Pb06 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 6 }
}

impl AfExtint6 for Pb06 {
   fn af_extint_6(&self) -> usize { 0 }
}

impl AfAin14 for Pb06 {
   fn af_ain_14(&self) -> usize { 1 }
}

impl AfY12 for Pb06 {
   fn af_y_12(&self) -> usize { 1 }
}

pub const PB07: Pb07 = Pb07 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb07 {}

impl Pin for Pb07 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 7 }
}

impl AfExtint7 for Pb07 {
   fn af_extint_7(&self) -> usize { 0 }
}

impl AfAin15 for Pb07 {
   fn af_ain_15(&self) -> usize { 1 }
}

impl AfY13 for Pb07 {
   fn af_y_13(&self) -> usize { 1 }
}

pub const PB08: Pb08 = Pb08 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb08 {}

impl Pin for Pb08 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 8 }
}

impl AfExtint8 for Pb08 {
   fn af_extint_8(&self) -> usize { 0 }
}

impl AfAin2 for Pb08 {
   fn af_ain_2(&self) -> usize { 1 }
}

impl AfY14 for Pb08 {
   fn af_y_14(&self) -> usize { 1 }
}

impl AfSercom4Pad0 for Pb08 {
   fn af_sercom4_pad_0(&self) -> usize { 2 }
}

impl AfTc4Wo0 for Pb08 {
   fn af_tc4_wo_0(&self) -> usize { 3 }
}

pub const PB09: Pb09 = Pb09 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb09 {}

impl Pin for Pb09 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 9 }
}

impl AfExtint9 for Pb09 {
   fn af_extint_9(&self) -> usize { 0 }
}

impl AfAin3 for Pb09 {
   fn af_ain_3(&self) -> usize { 1 }
}

impl AfY15 for Pb09 {
   fn af_y_15(&self) -> usize { 1 }
}

impl AfSercom4Pad1 for Pb09 {
   fn af_sercom4_pad_1(&self) -> usize { 2 }
}

impl AfTc4Wo1 for Pb09 {
   fn af_tc4_wo_1(&self) -> usize { 3 }
}

pub const PB10: Pb10 = Pb10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb10 {}

impl Pin for Pb10 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 10 }
}

impl AfExtint10 for Pb10 {
   fn af_extint_10(&self) -> usize { 0 }
}

impl AfSercom4Pad2 for Pb10 {
   fn af_sercom4_pad_2(&self) -> usize { 2 }
}

impl AfTc5Wo0 for Pb10 {
   fn af_tc5_wo_0(&self) -> usize { 3 }
}

impl AfTcc0Wo4 for Pb10 {
   fn af_tcc0_wo_4(&self) -> usize { 4 }
}

impl AfI2sMck1 for Pb10 {
   fn af_i2s_mck_1(&self) -> usize { 5 }
}

impl AfGclkIo4 for Pb10 {
   fn af_gclk_io_4(&self) -> usize { 6 }
}

pub const PB11: Pb11 = Pb11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb11 {}

impl Pin for Pb11 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 11 }
}

impl AfExtint11 for Pb11 {
   fn af_extint_11(&self) -> usize { 0 }
}

impl AfSercom4Pad3 for Pb11 {
   fn af_sercom4_pad_3(&self) -> usize { 2 }
}

impl AfTc5Wo1 for Pb11 {
   fn af_tc5_wo_1(&self) -> usize { 3 }
}

impl AfTcc0Wo5 for Pb11 {
   fn af_tcc0_wo_5(&self) -> usize { 4 }
}

impl AfI2sSck1 for Pb11 {
   fn af_i2s_sck_1(&self) -> usize { 5 }
}

impl AfGclkIo5 for Pb11 {
   fn af_gclk_io_5(&self) -> usize { 6 }
}

pub const PB12: Pb12 = Pb12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb12 {}

impl Pin for Pb12 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 12 }
}

impl AfExtint12 for Pb12 {
   fn af_extint_12(&self) -> usize { 0 }
}

impl AfX12 for Pb12 {
   fn af_x_12(&self) -> usize { 1 }
}

impl AfSercom4Pad0 for Pb12 {
   fn af_sercom4_pad_0(&self) -> usize { 1 }
}

impl AfTc4Wo0 for Pb12 {
   fn af_tc4_wo_0(&self) -> usize { 3 }
}

impl AfTcc0Wo6 for Pb12 {
   fn af_tcc0_wo_6(&self) -> usize { 4 }
}

impl AfI2sFs1 for Pb12 {
   fn af_i2s_fs_1(&self) -> usize { 5 }
}

impl AfGclkIo6 for Pb12 {
   fn af_gclk_io_6(&self) -> usize { 6 }
}

pub const PB13: Pb13 = Pb13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb13 {}

impl Pin for Pb13 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 13 }
}

impl AfExtint13 for Pb13 {
   fn af_extint_13(&self) -> usize { 0 }
}

impl AfX13 for Pb13 {
   fn af_x_13(&self) -> usize { 1 }
}

impl AfSercom4Pad1 for Pb13 {
   fn af_sercom4_pad_1(&self) -> usize { 1 }
}

impl AfTc4Wo1 for Pb13 {
   fn af_tc4_wo_1(&self) -> usize { 3 }
}

impl AfTcc0Wo7 for Pb13 {
   fn af_tcc0_wo_7(&self) -> usize { 4 }
}

impl AfGclkIo7 for Pb13 {
   fn af_gclk_io_7(&self) -> usize { 6 }
}

pub const PB14: Pb14 = Pb14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb14 {}

impl Pin for Pb14 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 14 }
}

impl AfExtint14 for Pb14 {
   fn af_extint_14(&self) -> usize { 0 }
}

impl AfX14 for Pb14 {
   fn af_x_14(&self) -> usize { 1 }
}

impl AfSercom4Pad2 for Pb14 {
   fn af_sercom4_pad_2(&self) -> usize { 1 }
}

impl AfTc5Wo0 for Pb14 {
   fn af_tc5_wo_0(&self) -> usize { 3 }
}

impl AfGclkIo0 for Pb14 {
   fn af_gclk_io_0(&self) -> usize { 6 }
}

pub const PB15: Pb15 = Pb15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb15 {}

impl Pin for Pb15 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 15 }
}

impl AfExtint15 for Pb15 {
   fn af_extint_15(&self) -> usize { 0 }
}

impl AfX15 for Pb15 {
   fn af_x_15(&self) -> usize { 1 }
}

impl AfSercom4Pad3 for Pb15 {
   fn af_sercom4_pad_3(&self) -> usize { 1 }
}

impl AfTc5Wo1 for Pb15 {
   fn af_tc5_wo_1(&self) -> usize { 3 }
}

impl AfGclkIo1 for Pb15 {
   fn af_gclk_io_1(&self) -> usize { 6 }
}

pub const PB16: Pb16 = Pb16 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb16 {}

impl Pin for Pb16 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 16 }
}

impl AfExtint0 for Pb16 {
   fn af_extint_0(&self) -> usize { 0 }
}

impl AfSercom5Pad0 for Pb16 {
   fn af_sercom5_pad_0(&self) -> usize { 1 }
}

impl AfTc6Wo0 for Pb16 {
   fn af_tc6_wo_0(&self) -> usize { 3 }
}

impl AfTcc0Wo4 for Pb16 {
   fn af_tcc0_wo_4(&self) -> usize { 4 }
}

impl AfI2sSd1 for Pb16 {
   fn af_i2s_sd_1(&self) -> usize { 5 }
}

impl AfGclkIo2 for Pb16 {
   fn af_gclk_io_2(&self) -> usize { 6 }
}

pub const PB17: Pb17 = Pb17 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb17 {}

impl Pin for Pb17 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 17 }
}

impl AfExtint1 for Pb17 {
   fn af_extint_1(&self) -> usize { 0 }
}

impl AfSercom5Pad1 for Pb17 {
   fn af_sercom5_pad_1(&self) -> usize { 1 }
}

impl AfTc6Wo1 for Pb17 {
   fn af_tc6_wo_1(&self) -> usize { 3 }
}

impl AfTcc0Wo5 for Pb17 {
   fn af_tcc0_wo_5(&self) -> usize { 4 }
}

impl AfI2sMck0 for Pb17 {
   fn af_i2s_mck_0(&self) -> usize { 5 }
}

impl AfGclkIo3 for Pb17 {
   fn af_gclk_io_3(&self) -> usize { 6 }
}

pub const PB22: Pb22 = Pb22 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb22 {}

impl Pin for Pb22 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 22 }
}

impl AfExtint6 for Pb22 {
   fn af_extint_6(&self) -> usize { 0 }
}

impl AfSercom5Pad2 for Pb22 {
   fn af_sercom5_pad_2(&self) -> usize { 2 }
}

impl AfTc7Wo0 for Pb22 {
   fn af_tc7_wo_0(&self) -> usize { 3 }
}

impl AfGclkIo0 for Pb22 {
   fn af_gclk_io_0(&self) -> usize { 6 }
}

pub const PB23: Pb23 = Pb23 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb23 {}

impl Pin for Pb23 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 23 }
}

impl AfExtint7 for Pb23 {
   fn af_extint_7(&self) -> usize { 0 }
}

impl AfSercom5Pad3 for Pb23 {
   fn af_sercom5_pad_3(&self) -> usize { 2 }
}

impl AfTc7Wo1 for Pb23 {
   fn af_tc7_wo_1(&self) -> usize { 3 }
}

impl AfGclkIo1 for Pb23 {
   fn af_gclk_io_1(&self) -> usize { 6 }
}

pub const PB30: Pb30 = Pb30 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb30 {}

impl Pin for Pb30 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 30 }
}

impl AfExtint14 for Pb30 {
   fn af_extint_14(&self) -> usize { 0 }
}

impl AfSercom5Pad0 for Pb30 {
   fn af_sercom5_pad_0(&self) -> usize { 2 }
}

impl AfTcc0Wo0 for Pb30 {
   fn af_tcc0_wo_0(&self) -> usize { 3 }
}

impl AfTcc1Wo2 for Pb30 {
   fn af_tcc1_wo_2(&self) -> usize { 4 }
}

pub const PB31: Pb31 = Pb31 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb31 {}

impl Pin for Pb31 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 31 }
}

impl AfExtint15 for Pb31 {
   fn af_extint_15(&self) -> usize { 0 }
}

impl AfSercom5Pad1 for Pb31 {
   fn af_sercom5_pad_1(&self) -> usize { 2 }
}

impl AfTcc0Wo1 for Pb31 {
   fn af_tcc0_wo_1(&self) -> usize { 3 }
}

impl AfTcc1Wo3 for Pb31 {
   fn af_tcc1_wo_3(&self) -> usize { 4 }
}

