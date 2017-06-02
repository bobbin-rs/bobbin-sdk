pub const PORTA: Porta = Porta {};
pub const PORTA_REF: &Porta = &PORTA;
pub const PORTA_IMPL: PortImpl = PortImpl(0x41004400);
pub const PORTA_IMPL_REF: &PortImpl = &PORTA_IMPL;

pub struct Porta {}
impl ::core::ops::Deref for Porta {
   type Target = PortImpl;
   #[inline]
   fn deref(&self) -> &PortImpl { PORTA_IMPL_REF }
}


pub const PORTB: Portb = Portb {};
pub const PORTB_REF: &Portb = &PORTB;
pub const PORTB_IMPL: PortImpl = PortImpl(0x41004480);
pub const PORTB_IMPL_REF: &PortImpl = &PORTB_IMPL;

pub struct Portb {}
impl ::core::ops::Deref for Portb {
   type Target = PortImpl;
   #[inline]
   fn deref(&self) -> &PortImpl { PORTB_IMPL_REF }
}



#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PortImpl(pub u32);
impl PortImpl {
  #[inline]
  pub fn ctrl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
  #[inline]
  pub fn ctrl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
  #[inline]
  pub fn ctrl(&self) -> Ctrl { 
     unsafe {
       Ctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }
  #[inline]
  pub fn set_ctrl(&self, value: Ctrl) -> &PortImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &PortImpl {
     let tmp = self.ctrl();
     self.set_ctrl(f(tmp))
  }

  #[inline]
  pub fn dir_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
  #[inline]
  pub fn dir_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
  #[inline]
  pub fn dir(&self) -> Dir { 
     unsafe {
       Dir(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  #[inline]
  pub fn set_dir(&self, value: Dir) -> &PortImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dir<F: FnOnce(Dir) -> Dir>(&self, f: F) -> &PortImpl {
     let tmp = self.dir();
     self.set_dir(f(tmp))
  }

  #[inline]
  pub fn dirclr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
  #[inline]
  pub fn dirclr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
  #[inline]
  pub fn dirclr(&self) -> Dirclr { 
     unsafe {
       Dirclr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
  #[inline]
  pub fn set_dirclr(&self, value: Dirclr) -> &PortImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dirclr<F: FnOnce(Dirclr) -> Dirclr>(&self, f: F) -> &PortImpl {
     let tmp = self.dirclr();
     self.set_dirclr(f(tmp))
  }

  #[inline]
  pub fn dirset_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
  #[inline]
  pub fn dirset_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
  #[inline]
  pub fn dirset(&self) -> Dirset { 
     unsafe {
       Dirset(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
  #[inline]
  pub fn set_dirset(&self, value: Dirset) -> &PortImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dirset<F: FnOnce(Dirset) -> Dirset>(&self, f: F) -> &PortImpl {
     let tmp = self.dirset();
     self.set_dirset(f(tmp))
  }

  #[inline]
  pub fn dirtgl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
  #[inline]
  pub fn dirtgl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
  #[inline]
  pub fn dirtgl(&self) -> Dirtgl { 
     unsafe {
       Dirtgl(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
  #[inline]
  pub fn set_dirtgl(&self, value: Dirtgl) -> &PortImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dirtgl<F: FnOnce(Dirtgl) -> Dirtgl>(&self, f: F) -> &PortImpl {
     let tmp = self.dirtgl();
     self.set_dirtgl(f(tmp))
  }

  #[inline]
  pub fn in_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
  #[inline]
  pub fn in_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
  #[inline]
  pub fn _in(&self) -> In { 
     unsafe {
       In(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }

  #[inline]
  pub fn out_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
  #[inline]
  pub fn out_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
  #[inline]
  pub fn out(&self) -> Out { 
     unsafe {
       Out(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
  #[inline]
  pub fn set_out(&self, value: Out) -> &PortImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_out<F: FnOnce(Out) -> Out>(&self, f: F) -> &PortImpl {
     let tmp = self.out();
     self.set_out(f(tmp))
  }

  #[inline]
  pub fn outclr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
  #[inline]
  pub fn outclr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
  #[inline]
  pub fn outclr(&self) -> Outclr { 
     unsafe {
       Outclr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
  #[inline]
  pub fn set_outclr(&self, value: Outclr) -> &PortImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_outclr<F: FnOnce(Outclr) -> Outclr>(&self, f: F) -> &PortImpl {
     let tmp = self.outclr();
     self.set_outclr(f(tmp))
  }

  #[inline]
  pub fn outset_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
  #[inline]
  pub fn outset_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
  #[inline]
  pub fn outset(&self) -> Outset { 
     unsafe {
       Outset(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
  #[inline]
  pub fn set_outset(&self, value: Outset) -> &PortImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_outset<F: FnOnce(Outset) -> Outset>(&self, f: F) -> &PortImpl {
     let tmp = self.outset();
     self.set_outset(f(tmp))
  }

  #[inline]
  pub fn outtgl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
  #[inline]
  pub fn outtgl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
  #[inline]
  pub fn outtgl(&self) -> Outtgl { 
     unsafe {
       Outtgl(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
  #[inline]
  pub fn set_outtgl(&self, value: Outtgl) -> &PortImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_outtgl<F: FnOnce(Outtgl) -> Outtgl>(&self, f: F) -> &PortImpl {
     let tmp = self.outtgl();
     self.set_outtgl(f(tmp))
  }

  #[inline]
  pub fn pincfg_ptr(&self, index: usize) -> *const u8 { 
     assert!(index < 32);
     ((self.0 as usize) + 0x40 + (index)) as *const u8
  }
  #[inline]
  pub fn pincfg_mut(&self, index: usize) -> *mut u8 { 
     assert!(index < 32);
     ((self.0 as usize) + 0x40 + (index)) as *mut u8
  }
  #[inline]
  pub fn pincfg(&self, index: usize) -> Pincfg { 
     assert!(index < 32);
     unsafe {
        Pincfg(::core::ptr::read_volatile(((self.0 as usize) + 0x40 + (index)) as *const u8))
     }
  }
  #[inline]
  pub fn set_pincfg(&self, index: usize, value: Pincfg) -> &PortImpl {
     assert!(index < 32);
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x40 + (index)) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_pincfg<F: FnOnce(Pincfg) -> Pincfg>(&self, index: usize, f: F) -> &PortImpl {
     let tmp = self.pincfg(index);
     self.set_pincfg(index, f(tmp))
  }

  #[inline]
  pub fn pmux_ptr(&self, index: usize) -> *const u8 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x30 + (index)) as *const u8
  }
  #[inline]
  pub fn pmux_mut(&self, index: usize) -> *mut u8 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x30 + (index)) as *mut u8
  }
  #[inline]
  pub fn pmux(&self, index: usize) -> Pmux { 
     assert!(index < 16);
     unsafe {
        Pmux(::core::ptr::read_volatile(((self.0 as usize) + 0x30 + (index)) as *const u8))
     }
  }
  #[inline]
  pub fn set_pmux(&self, index: usize, value: Pmux) -> &PortImpl {
     assert!(index < 16);
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x30 + (index)) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_pmux<F: FnOnce(Pmux) -> Pmux>(&self, index: usize, f: F) -> &PortImpl {
     let tmp = self.pmux(index);
     self.set_pmux(index, f(tmp))
  }

  #[inline]
  pub fn wrconfig_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x28) as *const u32
  }
  #[inline]
  pub fn wrconfig_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x28) as *mut u32
  }
  #[inline]
  pub fn set_wrconfig(&self, value: Wrconfig) -> &PortImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
     }
     self
  }

}

#[derive(PartialEq, Eq)]
pub struct Ctrl(pub u32);
impl Ctrl {
  #[inline]
  pub fn sampling(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn dir(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
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
  #[inline]
  pub fn dirclr(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
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
  #[inline]
  pub fn dirset(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
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
  #[inline]
  pub fn dirtgl(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
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
  #[inline]
  pub fn _in(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
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
  #[inline]
  pub fn out(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
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
  #[inline]
  pub fn outclr(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
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
  #[inline]
  pub fn outset(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
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
  #[inline]
  pub fn outtgl(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
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
  #[inline]
  pub fn pmuxen(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_pmuxen(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn inen(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_inen(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn pullen(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_pullen(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn drvstr(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  #[inline]
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
  #[inline]
  pub fn pmux(&self, index: usize) -> u8 {
     assert!(index < 2);
     let shift: usize = 0 + (index << 2);
     ((self.0 as u8) >> shift) & 0xf // [3:0]
  }
  #[inline]
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
  #[inline]
  pub fn pinmask(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline]
  pub fn set_pinmask(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn pmuxen(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
  pub fn set_pmuxen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn inen(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline]
  pub fn set_inen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline]
  pub fn pullen(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline]
  pub fn set_pullen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  #[inline]
  pub fn drvstr(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  #[inline]
  pub fn set_drvstr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  #[inline]
  pub fn pmux(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  #[inline]
  pub fn set_pmux(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

  #[inline]
  pub fn wrpmux(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  #[inline]
  pub fn set_wrpmux(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

  #[inline]
  pub fn wrpincfg(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  #[inline]
  pub fn set_wrpincfg(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  #[inline]
  pub fn hwsel(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  #[inline]
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
pub struct PinImpl {
  pub port: PortImpl,
  pub index: usize,
}

pub trait Pin<T> {
   fn port(&self) -> T;
   fn index(&self) -> usize;
}

pub trait AltFn<T> {
   fn alt_fn(&self) -> usize;
}

pub const PA00: Pa00 = Pa00 {}; 
pub const PA00_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 0 };
pub const PA00_IMPL_REF: &PinImpl = &PA00_IMPL;

impl ::core::ops::Deref for Pa00 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA00_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa00 {}

impl Pin<Porta> for Pa00 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::Extint0> for Pa00 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom1Pad0> for Pa00 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc2> for Pa00 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo0> for Pa00 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PA01: Pa01 = Pa01 {}; 
pub const PA01_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 1 };
pub const PA01_IMPL_REF: &PinImpl = &PA01_IMPL;

impl ::core::ops::Deref for Pa01 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA01_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa01 {}

impl Pin<Porta> for Pa01 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::Extint1> for Pa01 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom1Pad1> for Pa01 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc2> for Pa01 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo1> for Pa01 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PA02: Pa02 = Pa02 {}; 
pub const PA02_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 2 };
pub const PA02_IMPL_REF: &PinImpl = &PA02_IMPL;

impl ::core::ops::Deref for Pa02 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA02_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa02 {}

impl Pin<Porta> for Pa02 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::Extint2> for Pa02 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain0> for Pa02 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y0> for Pa02 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Vout> for Pa02 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PA03: Pa03 = Pa03 {}; 
pub const PA03_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 3 };
pub const PA03_IMPL_REF: &PinImpl = &PA03_IMPL;

impl ::core::ops::Deref for Pa03 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA03_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa03 {}

impl Pin<Porta> for Pa03 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::Extint3> for Pa03 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc> for Pa03 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Vrefadac> for Pa03 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Vrefa> for Pa03 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ain1> for Pa03 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y1> for Pa03 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PA04: Pa04 = Pa04 {}; 
pub const PA04_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 4 };
pub const PA04_IMPL_REF: &PinImpl = &PA04_IMPL;

impl ::core::ops::Deref for Pa04 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA04_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa04 {}

impl Pin<Porta> for Pa04 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::Extint4> for Pa04 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc> for Pa04 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Vrefb> for Pa04 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ain4> for Pa04 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ain0> for Pa04 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y2> for Pa04 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom0Pad0> for Pa04 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc0> for Pa04 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo0> for Pa04 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PA05: Pa05 = Pa05 {}; 
pub const PA05_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 5 };
pub const PA05_IMPL_REF: &PinImpl = &PA05_IMPL;

impl ::core::ops::Deref for Pa05 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA05_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa05 {}

impl Pin<Porta> for Pa05 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 5 }
}

impl AltFn<super::sig::Extint5> for Pa05 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain5> for Pa05 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ain1> for Pa05 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y3> for Pa05 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom0Pad1> for Pa05 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc0> for Pa05 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo1> for Pa05 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PA06: Pa06 = Pa06 {}; 
pub const PA06_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 6 };
pub const PA06_IMPL_REF: &PinImpl = &PA06_IMPL;

impl ::core::ops::Deref for Pa06 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA06_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa06 {}

impl Pin<Porta> for Pa06 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 6 }
}

impl AltFn<super::sig::Extint6> for Pa06 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain6> for Pa06 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ain2> for Pa06 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y4> for Pa06 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom0Pad2> for Pa06 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc1> for Pa06 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo0> for Pa06 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PA07: Pa07 = Pa07 {}; 
pub const PA07_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 7 };
pub const PA07_IMPL_REF: &PinImpl = &PA07_IMPL;

impl ::core::ops::Deref for Pa07 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA07_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa07 {}

impl Pin<Porta> for Pa07 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 7 }
}

impl AltFn<super::sig::Extint7> for Pa07 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain7> for Pa07 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ain3> for Pa07 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y5> for Pa07 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom0Pad3> for Pa07 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc1> for Pa07 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo1> for Pa07 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2s> for Pa07 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Sd0> for Pa07 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PA08: Pa08 = Pa08 {}; 
pub const PA08_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 8 };
pub const PA08_IMPL_REF: &PinImpl = &PA08_IMPL;

impl ::core::ops::Deref for Pa08 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA08_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa08 {}

impl Pin<Porta> for Pa08 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 8 }
}

impl AltFn<super::sig::Nmi> for Pa08 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain16> for Pa08 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::X0> for Pa08 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom0Pad0> for Pa08 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom2Pad0> for Pa08 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc0> for Pa08 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo0> for Pa08 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc1> for Pa08 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Wo2> for Pa08 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s> for Pa08 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Sd1> for Pa08 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PA09: Pa09 = Pa09 {}; 
pub const PA09_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 9 };
pub const PA09_IMPL_REF: &PinImpl = &PA09_IMPL;

impl ::core::ops::Deref for Pa09 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA09_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa09 {}

impl Pin<Porta> for Pa09 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 9 }
}

impl AltFn<super::sig::Extint9> for Pa09 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain17> for Pa09 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::X1> for Pa09 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom0Pad1> for Pa09 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom2Pad1> for Pa09 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc0> for Pa09 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo1> for Pa09 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc1> for Pa09 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Wo3> for Pa09 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s> for Pa09 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Mck0> for Pa09 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PA10: Pa10 = Pa10 {}; 
pub const PA10_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 10 };
pub const PA10_IMPL_REF: &PinImpl = &PA10_IMPL;

impl ::core::ops::Deref for Pa10 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA10_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa10 {}

impl Pin<Porta> for Pa10 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 10 }
}

impl AltFn<super::sig::Extint10> for Pa10 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain18> for Pa10 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::X2> for Pa10 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom0Pad2> for Pa10 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom2Pad2> for Pa10 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc1> for Pa10 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo0> for Pa10 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0> for Pa10 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Wo2> for Pa10 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s> for Pa10 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Sck0> for Pa10 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::GclkIo4> for Pa10 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA11: Pa11 = Pa11 {}; 
pub const PA11_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 11 };
pub const PA11_IMPL_REF: &PinImpl = &PA11_IMPL;

impl ::core::ops::Deref for Pa11 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA11_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa11 {}

impl Pin<Porta> for Pa11 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 11 }
}

impl AltFn<super::sig::Extint11> for Pa11 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain19> for Pa11 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::X3> for Pa11 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom0Pad3> for Pa11 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom2Pad3> for Pa11 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc1> for Pa11 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo1> for Pa11 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0> for Pa11 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Wo3> for Pa11 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s> for Pa11 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Fs0> for Pa11 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::GclkIo5> for Pa11 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA12: Pa12 = Pa12 {}; 
pub const PA12_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 12 };
pub const PA12_IMPL_REF: &PinImpl = &PA12_IMPL;

impl ::core::ops::Deref for Pa12 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA12_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa12 {}

impl Pin<Porta> for Pa12 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 12 }
}

impl AltFn<super::sig::Extint12> for Pa12 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom2Pad0> for Pa12 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom4Pad0> for Pa12 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc2> for Pa12 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo0> for Pa12 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0> for Pa12 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Wo6> for Pa12 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ac> for Pa12 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Cmp0> for Pa12 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA13: Pa13 = Pa13 {}; 
pub const PA13_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 13 };
pub const PA13_IMPL_REF: &PinImpl = &PA13_IMPL;

impl ::core::ops::Deref for Pa13 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA13_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa13 {}

impl Pin<Porta> for Pa13 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 13 }
}

impl AltFn<super::sig::Extint13> for Pa13 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom2Pad1> for Pa13 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom4Pad1> for Pa13 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc2> for Pa13 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo1> for Pa13 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0> for Pa13 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Wo7> for Pa13 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ac> for Pa13 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Cmp1> for Pa13 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA14: Pa14 = Pa14 {}; 
pub const PA14_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 14 };
pub const PA14_IMPL_REF: &PinImpl = &PA14_IMPL;

impl ::core::ops::Deref for Pa14 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA14_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa14 {}

impl Pin<Porta> for Pa14 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 14 }
}

impl AltFn<super::sig::Extint14> for Pa14 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom2Pad2> for Pa14 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom4Pad2> for Pa14 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc3> for Pa14 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo0> for Pa14 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0> for Pa14 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Wo4> for Pa14 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::GclkIo0> for Pa14 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA15: Pa15 = Pa15 {}; 
pub const PA15_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 15 };
pub const PA15_IMPL_REF: &PinImpl = &PA15_IMPL;

impl ::core::ops::Deref for Pa15 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA15_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa15 {}

impl Pin<Porta> for Pa15 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 15 }
}

impl AltFn<super::sig::Extint15> for Pa15 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom2Pad3> for Pa15 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom4Pad3> for Pa15 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc3> for Pa15 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo1> for Pa15 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0> for Pa15 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Wo5> for Pa15 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::GclkIo1> for Pa15 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA16: Pa16 = Pa16 {}; 
pub const PA16_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 16 };
pub const PA16_IMPL_REF: &PinImpl = &PA16_IMPL;

impl ::core::ops::Deref for Pa16 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA16_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa16 {}

impl Pin<Porta> for Pa16 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 16 }
}

impl AltFn<super::sig::Extint0> for Pa16 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::X4> for Pa16 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom1Pad0> for Pa16 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom3Pad0> for Pa16 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc2> for Pa16 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo0> for Pa16 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0> for Pa16 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Wo6> for Pa16 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::GclkIo2> for Pa16 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA17: Pa17 = Pa17 {}; 
pub const PA17_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 17 };
pub const PA17_IMPL_REF: &PinImpl = &PA17_IMPL;

impl ::core::ops::Deref for Pa17 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA17_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa17 {}

impl Pin<Porta> for Pa17 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 17 }
}

impl AltFn<super::sig::Extint1> for Pa17 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::X5> for Pa17 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom1Pad1> for Pa17 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom3Pad1> for Pa17 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc2> for Pa17 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo1> for Pa17 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0> for Pa17 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Wo7> for Pa17 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::GclkIo3> for Pa17 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA18: Pa18 = Pa18 {}; 
pub const PA18_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 18 };
pub const PA18_IMPL_REF: &PinImpl = &PA18_IMPL;

impl ::core::ops::Deref for Pa18 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA18_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa18 {}

impl Pin<Porta> for Pa18 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 18 }
}

impl AltFn<super::sig::Extint2> for Pa18 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::X6> for Pa18 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom1Pad2> for Pa18 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom3Pad2> for Pa18 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc3> for Pa18 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo0> for Pa18 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0> for Pa18 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Wo2> for Pa18 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ac> for Pa18 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Cmp0> for Pa18 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA19: Pa19 = Pa19 {}; 
pub const PA19_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 19 };
pub const PA19_IMPL_REF: &PinImpl = &PA19_IMPL;

impl ::core::ops::Deref for Pa19 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA19_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa19 {}

impl Pin<Porta> for Pa19 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 19 }
}

impl AltFn<super::sig::Extint3> for Pa19 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::X7> for Pa19 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom1Pad3> for Pa19 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom3Pad3> for Pa19 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc3> for Pa19 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo1> for Pa19 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0> for Pa19 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Wo3> for Pa19 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s> for Pa19 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Sd0> for Pa19 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Ac> for Pa19 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Cmp1> for Pa19 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA20: Pa20 = Pa20 {}; 
pub const PA20_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 20 };
pub const PA20_IMPL_REF: &PinImpl = &PA20_IMPL;

impl ::core::ops::Deref for Pa20 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA20_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa20 {}

impl Pin<Porta> for Pa20 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 20 }
}

impl AltFn<super::sig::Extint4> for Pa20 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::X8> for Pa20 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom5Pad2> for Pa20 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom3Pad2> for Pa20 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc7> for Pa20 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo0> for Pa20 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0> for Pa20 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Wo6> for Pa20 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s> for Pa20 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Sck0> for Pa20 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::GclkIo4> for Pa20 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA21: Pa21 = Pa21 {}; 
pub const PA21_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 21 };
pub const PA21_IMPL_REF: &PinImpl = &PA21_IMPL;

impl ::core::ops::Deref for Pa21 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA21_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa21 {}

impl Pin<Porta> for Pa21 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 21 }
}

impl AltFn<super::sig::Extint5> for Pa21 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::X9> for Pa21 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom5Pad3> for Pa21 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom3Pad3> for Pa21 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc7> for Pa21 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo1> for Pa21 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0> for Pa21 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Wo7> for Pa21 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s> for Pa21 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Fs0> for Pa21 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::GclkIo5> for Pa21 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA22: Pa22 = Pa22 {}; 
pub const PA22_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 22 };
pub const PA22_IMPL_REF: &PinImpl = &PA22_IMPL;

impl ::core::ops::Deref for Pa22 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA22_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa22 {}

impl Pin<Porta> for Pa22 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 22 }
}

impl AltFn<super::sig::Extint6> for Pa22 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::X10> for Pa22 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom3Pad0> for Pa22 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom5Pad0> for Pa22 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc4> for Pa22 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo0> for Pa22 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0> for Pa22 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Wo4> for Pa22 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::GclkIo6> for Pa22 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA23: Pa23 = Pa23 {}; 
pub const PA23_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 23 };
pub const PA23_IMPL_REF: &PinImpl = &PA23_IMPL;

impl ::core::ops::Deref for Pa23 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA23_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa23 {}

impl Pin<Porta> for Pa23 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 23 }
}

impl AltFn<super::sig::Extint7> for Pa23 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::X11> for Pa23 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom3Pad1> for Pa23 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom5Pad1> for Pa23 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc4> for Pa23 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo1> for Pa23 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0> for Pa23 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Wo5> for Pa23 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Usb> for Pa23 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Sof1khz> for Pa23 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::GclkIo7> for Pa23 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA24: Pa24 = Pa24 {}; 
pub const PA24_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 24 };
pub const PA24_IMPL_REF: &PinImpl = &PA24_IMPL;

impl ::core::ops::Deref for Pa24 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA24_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa24 {}

impl Pin<Porta> for Pa24 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 24 }
}

impl AltFn<super::sig::Extint12> for Pa24 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom3Pad2> for Pa24 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom5Pad2> for Pa24 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc5> for Pa24 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo0> for Pa24 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc1> for Pa24 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Wo2> for Pa24 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Usb> for Pa24 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Dm> for Pa24 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PA25: Pa25 = Pa25 {}; 
pub const PA25_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 25 };
pub const PA25_IMPL_REF: &PinImpl = &PA25_IMPL;

impl ::core::ops::Deref for Pa25 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA25_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa25 {}

impl Pin<Porta> for Pa25 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 25 }
}

impl AltFn<super::sig::Extint13> for Pa25 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom3Pad3> for Pa25 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom5Pad3> for Pa25 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc5> for Pa25 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo1> for Pa25 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc1> for Pa25 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Wo3> for Pa25 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Usb> for Pa25 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Dp> for Pa25 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PA27: Pa27 = Pa27 {}; 
pub const PA27_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 27 };
pub const PA27_IMPL_REF: &PinImpl = &PA27_IMPL;

impl ::core::ops::Deref for Pa27 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA27_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa27 {}

impl Pin<Porta> for Pa27 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 27 }
}

impl AltFn<super::sig::Extint15> for Pa27 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::GclkIo0> for Pa27 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA28: Pa28 = Pa28 {}; 
pub const PA28_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 28 };
pub const PA28_IMPL_REF: &PinImpl = &PA28_IMPL;

impl ::core::ops::Deref for Pa28 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA28_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa28 {}

impl Pin<Porta> for Pa28 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 28 }
}

impl AltFn<super::sig::Extint8> for Pa28 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::GclkIo0> for Pa28 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA30: Pa30 = Pa30 {}; 
pub const PA30_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 30 };
pub const PA30_IMPL_REF: &PinImpl = &PA30_IMPL;

impl ::core::ops::Deref for Pa30 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA30_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa30 {}

impl Pin<Porta> for Pa30 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 30 }
}

impl AltFn<super::sig::Extint10> for Pa30 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom1Pad2> for Pa30 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc1> for Pa30 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo0> for Pa30 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Swclk> for Pa30 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::GclkIo0> for Pa30 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA31: Pa31 = Pa31 {}; 
pub const PA31_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 31 };
pub const PA31_IMPL_REF: &PinImpl = &PA31_IMPL;

impl ::core::ops::Deref for Pa31 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA31_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa31 {}

impl Pin<Porta> for Pa31 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 31 }
}

impl AltFn<super::sig::Extint11> for Pa31 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom1Pad3> for Pa31 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc1> for Pa31 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo1> for Pa31 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Swdio> for Pa31 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PB00: Pb00 = Pb00 {}; 
pub const PB00_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 0 };
pub const PB00_IMPL_REF: &PinImpl = &PB00_IMPL;

impl ::core::ops::Deref for Pb00 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB00_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb00 {}

impl Pin<Portb> for Pb00 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::Extint0> for Pb00 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain8> for Pb00 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y6> for Pb00 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom5Pad2> for Pb00 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc7> for Pb00 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo0> for Pb00 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PB01: Pb01 = Pb01 {}; 
pub const PB01_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 1 };
pub const PB01_IMPL_REF: &PinImpl = &PB01_IMPL;

impl ::core::ops::Deref for Pb01 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB01_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb01 {}

impl Pin<Portb> for Pb01 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::Extint1> for Pb01 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain9> for Pb01 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y7> for Pb01 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom5Pad3> for Pb01 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc7> for Pb01 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo1> for Pb01 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PB02: Pb02 = Pb02 {}; 
pub const PB02_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 2 };
pub const PB02_IMPL_REF: &PinImpl = &PB02_IMPL;

impl ::core::ops::Deref for Pb02 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB02_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb02 {}

impl Pin<Portb> for Pb02 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::Extint2> for Pb02 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain10> for Pb02 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y8> for Pb02 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom5Pad0> for Pb02 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc6> for Pb02 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo0> for Pb02 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PB03: Pb03 = Pb03 {}; 
pub const PB03_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 3 };
pub const PB03_IMPL_REF: &PinImpl = &PB03_IMPL;

impl ::core::ops::Deref for Pb03 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB03_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb03 {}

impl Pin<Portb> for Pb03 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::Extint3> for Pb03 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain11> for Pb03 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y9> for Pb03 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom5Pad1> for Pb03 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc6> for Pb03 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo1> for Pb03 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PB04: Pb04 = Pb04 {}; 
pub const PB04_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 4 };
pub const PB04_IMPL_REF: &PinImpl = &PB04_IMPL;

impl ::core::ops::Deref for Pb04 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB04_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb04 {}

impl Pin<Portb> for Pb04 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::Extint4> for Pb04 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain12> for Pb04 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y10> for Pb04 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PB05: Pb05 = Pb05 {}; 
pub const PB05_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 5 };
pub const PB05_IMPL_REF: &PinImpl = &PB05_IMPL;

impl ::core::ops::Deref for Pb05 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB05_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb05 {}

impl Pin<Portb> for Pb05 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 5 }
}

impl AltFn<super::sig::Extint5> for Pb05 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain13> for Pb05 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y11> for Pb05 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PB06: Pb06 = Pb06 {}; 
pub const PB06_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 6 };
pub const PB06_IMPL_REF: &PinImpl = &PB06_IMPL;

impl ::core::ops::Deref for Pb06 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB06_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb06 {}

impl Pin<Portb> for Pb06 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 6 }
}

impl AltFn<super::sig::Extint6> for Pb06 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain14> for Pb06 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y12> for Pb06 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PB07: Pb07 = Pb07 {}; 
pub const PB07_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 7 };
pub const PB07_IMPL_REF: &PinImpl = &PB07_IMPL;

impl ::core::ops::Deref for Pb07 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB07_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb07 {}

impl Pin<Portb> for Pb07 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 7 }
}

impl AltFn<super::sig::Extint7> for Pb07 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain15> for Pb07 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y13> for Pb07 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PB08: Pb08 = Pb08 {}; 
pub const PB08_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 8 };
pub const PB08_IMPL_REF: &PinImpl = &PB08_IMPL;

impl ::core::ops::Deref for Pb08 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB08_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb08 {}

impl Pin<Portb> for Pb08 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 8 }
}

impl AltFn<super::sig::Extint8> for Pb08 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain2> for Pb08 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y14> for Pb08 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom4Pad0> for Pb08 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc4> for Pb08 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo0> for Pb08 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PB09: Pb09 = Pb09 {}; 
pub const PB09_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 9 };
pub const PB09_IMPL_REF: &PinImpl = &PB09_IMPL;

impl ::core::ops::Deref for Pb09 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB09_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb09 {}

impl Pin<Portb> for Pb09 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 9 }
}

impl AltFn<super::sig::Extint9> for Pb09 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain3> for Pb09 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y15> for Pb09 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom4Pad1> for Pb09 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc4> for Pb09 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo1> for Pb09 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PB10: Pb10 = Pb10 {}; 
pub const PB10_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 10 };
pub const PB10_IMPL_REF: &PinImpl = &PB10_IMPL;

impl ::core::ops::Deref for Pb10 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB10_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb10 {}

impl Pin<Portb> for Pb10 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 10 }
}

impl AltFn<super::sig::Extint10> for Pb10 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom4Pad2> for Pb10 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc5> for Pb10 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo0> for Pb10 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0> for Pb10 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Wo4> for Pb10 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s> for Pb10 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Mck1> for Pb10 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::GclkIo4> for Pb10 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PB11: Pb11 = Pb11 {}; 
pub const PB11_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 11 };
pub const PB11_IMPL_REF: &PinImpl = &PB11_IMPL;

impl ::core::ops::Deref for Pb11 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB11_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb11 {}

impl Pin<Portb> for Pb11 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 11 }
}

impl AltFn<super::sig::Extint11> for Pb11 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom4Pad3> for Pb11 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc5> for Pb11 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo1> for Pb11 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0> for Pb11 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Wo5> for Pb11 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s> for Pb11 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Sck1> for Pb11 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::GclkIo5> for Pb11 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PB12: Pb12 = Pb12 {}; 
pub const PB12_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 12 };
pub const PB12_IMPL_REF: &PinImpl = &PB12_IMPL;

impl ::core::ops::Deref for Pb12 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB12_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb12 {}

impl Pin<Portb> for Pb12 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 12 }
}

impl AltFn<super::sig::Extint12> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::X12> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom4Pad0> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tc4> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo0> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Wo6> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Fs1> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::GclkIo6> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PB13: Pb13 = Pb13 {}; 
pub const PB13_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 13 };
pub const PB13_IMPL_REF: &PinImpl = &PB13_IMPL;

impl ::core::ops::Deref for Pb13 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB13_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb13 {}

impl Pin<Portb> for Pb13 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 13 }
}

impl AltFn<super::sig::Extint13> for Pb13 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::X13> for Pb13 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom4Pad1> for Pb13 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tc4> for Pb13 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo1> for Pb13 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0> for Pb13 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Wo7> for Pb13 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::GclkIo7> for Pb13 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PB14: Pb14 = Pb14 {}; 
pub const PB14_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 14 };
pub const PB14_IMPL_REF: &PinImpl = &PB14_IMPL;

impl ::core::ops::Deref for Pb14 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB14_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb14 {}

impl Pin<Portb> for Pb14 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 14 }
}

impl AltFn<super::sig::Extint14> for Pb14 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::X14> for Pb14 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom4Pad2> for Pb14 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tc5> for Pb14 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo0> for Pb14 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::GclkIo0> for Pb14 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PB15: Pb15 = Pb15 {}; 
pub const PB15_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 15 };
pub const PB15_IMPL_REF: &PinImpl = &PB15_IMPL;

impl ::core::ops::Deref for Pb15 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB15_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb15 {}

impl Pin<Portb> for Pb15 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 15 }
}

impl AltFn<super::sig::Extint15> for Pb15 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::X15> for Pb15 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom4Pad3> for Pb15 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tc5> for Pb15 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo1> for Pb15 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::GclkIo1> for Pb15 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PB16: Pb16 = Pb16 {}; 
pub const PB16_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 16 };
pub const PB16_IMPL_REF: &PinImpl = &PB16_IMPL;

impl ::core::ops::Deref for Pb16 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB16_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb16 {}

impl Pin<Portb> for Pb16 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 16 }
}

impl AltFn<super::sig::Extint0> for Pb16 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom5Pad0> for Pb16 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tc6> for Pb16 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo0> for Pb16 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0> for Pb16 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Wo4> for Pb16 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s> for Pb16 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Sd1> for Pb16 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::GclkIo2> for Pb16 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PB17: Pb17 = Pb17 {}; 
pub const PB17_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 17 };
pub const PB17_IMPL_REF: &PinImpl = &PB17_IMPL;

impl ::core::ops::Deref for Pb17 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB17_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb17 {}

impl Pin<Portb> for Pb17 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 17 }
}

impl AltFn<super::sig::Extint1> for Pb17 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom5Pad1> for Pb17 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tc6> for Pb17 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo1> for Pb17 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0> for Pb17 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Wo5> for Pb17 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s> for Pb17 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Mck0> for Pb17 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::GclkIo3> for Pb17 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PB22: Pb22 = Pb22 {}; 
pub const PB22_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 22 };
pub const PB22_IMPL_REF: &PinImpl = &PB22_IMPL;

impl ::core::ops::Deref for Pb22 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB22_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb22 {}

impl Pin<Portb> for Pb22 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 22 }
}

impl AltFn<super::sig::Extint6> for Pb22 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom5Pad2> for Pb22 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc7> for Pb22 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo0> for Pb22 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::GclkIo0> for Pb22 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PB23: Pb23 = Pb23 {}; 
pub const PB23_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 23 };
pub const PB23_IMPL_REF: &PinImpl = &PB23_IMPL;

impl ::core::ops::Deref for Pb23 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB23_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb23 {}

impl Pin<Portb> for Pb23 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 23 }
}

impl AltFn<super::sig::Extint7> for Pb23 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom5Pad3> for Pb23 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc7> for Pb23 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo1> for Pb23 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::GclkIo1> for Pb23 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PB30: Pb30 = Pb30 {}; 
pub const PB30_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 30 };
pub const PB30_IMPL_REF: &PinImpl = &PB30_IMPL;

impl ::core::ops::Deref for Pb30 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB30_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb30 {}

impl Pin<Portb> for Pb30 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 30 }
}

impl AltFn<super::sig::Extint14> for Pb30 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom5Pad0> for Pb30 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc0> for Pb30 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo0> for Pb30 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc1> for Pb30 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Wo2> for Pb30 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PB31: Pb31 = Pb31 {}; 
pub const PB31_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 31 };
pub const PB31_IMPL_REF: &PinImpl = &PB31_IMPL;

impl ::core::ops::Deref for Pb31 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB31_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb31 {}

impl Pin<Portb> for Pb31 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 31 }
}

impl AltFn<super::sig::Extint15> for Pb31 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom5Pad1> for Pb31 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc0> for Pb31 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Wo1> for Pb31 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc1> for Pb31 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Wo3> for Pb31 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

