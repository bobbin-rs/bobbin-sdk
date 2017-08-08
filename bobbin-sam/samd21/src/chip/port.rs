//! I/O Pin Controller
#[allow(unused_imports)] use bobbin_common::bits;
pub const PORTA: Porta = Periph(0x41004400, PortaId {});
pub const PORTB: Portb = Periph(0x41004480, PortbId {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="PORT Peripheral"]
pub struct Periph<T>(pub u32, pub T); 

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct PortaId {}
pub type Porta = Periph<PortaId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct PortbId {}
pub type Portb = Periph<PortbId>;




impl<T> Periph<T> {
#[doc="Get the *const pointer for the CTRL register."]
  #[inline] pub fn ctrl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
#[doc="Get the *mut pointer for the CTRL register."]
  #[inline] pub fn ctrl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
#[doc="Read the CTRL register."]
  #[inline] pub fn ctrl(&self) -> Ctrl { 
     unsafe {
        Ctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }
#[doc="Write the CTRL register."]
  #[inline] pub fn set_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
     let value = f(Ctrl(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CTRL register."]
  #[inline] pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
     let tmp = self.ctrl();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the DIR register."]
  #[inline] pub fn dir_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the DIR register."]
  #[inline] pub fn dir_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the DIR register."]
  #[inline] pub fn dir(&self) -> Dir { 
     unsafe {
        Dir(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the DIR register."]
  #[inline] pub fn set_dir<F: FnOnce(Dir) -> Dir>(&self, f: F) -> &Self {
     let value = f(Dir(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DIR register."]
  #[inline] pub fn with_dir<F: FnOnce(Dir) -> Dir>(&self, f: F) -> &Self {
     let tmp = self.dir();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the DIRCLR register."]
  #[inline] pub fn dirclr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the DIRCLR register."]
  #[inline] pub fn dirclr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the DIRCLR register."]
  #[inline] pub fn dirclr(&self) -> Dirclr { 
     unsafe {
        Dirclr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the DIRCLR register."]
  #[inline] pub fn set_dirclr<F: FnOnce(Dirclr) -> Dirclr>(&self, f: F) -> &Self {
     let value = f(Dirclr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DIRCLR register."]
  #[inline] pub fn with_dirclr<F: FnOnce(Dirclr) -> Dirclr>(&self, f: F) -> &Self {
     let tmp = self.dirclr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the DIRSET register."]
  #[inline] pub fn dirset_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the DIRSET register."]
  #[inline] pub fn dirset_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the DIRSET register."]
  #[inline] pub fn dirset(&self) -> Dirset { 
     unsafe {
        Dirset(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the DIRSET register."]
  #[inline] pub fn set_dirset<F: FnOnce(Dirset) -> Dirset>(&self, f: F) -> &Self {
     let value = f(Dirset(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DIRSET register."]
  #[inline] pub fn with_dirset<F: FnOnce(Dirset) -> Dirset>(&self, f: F) -> &Self {
     let tmp = self.dirset();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the DIRTGL register."]
  #[inline] pub fn dirtgl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the DIRTGL register."]
  #[inline] pub fn dirtgl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the DIRTGL register."]
  #[inline] pub fn dirtgl(&self) -> Dirtgl { 
     unsafe {
        Dirtgl(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the DIRTGL register."]
  #[inline] pub fn set_dirtgl<F: FnOnce(Dirtgl) -> Dirtgl>(&self, f: F) -> &Self {
     let value = f(Dirtgl(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DIRTGL register."]
  #[inline] pub fn with_dirtgl<F: FnOnce(Dirtgl) -> Dirtgl>(&self, f: F) -> &Self {
     let tmp = self.dirtgl();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the IN register."]
  #[inline] pub fn in_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
#[doc="Get the *mut pointer for the IN register."]
  #[inline] pub fn in_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
#[doc="Read the IN register."]
  #[inline] pub fn _in(&self) -> In { 
     unsafe {
        In(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }

#[doc="Get the *const pointer for the OUT register."]
  #[inline] pub fn out_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the OUT register."]
  #[inline] pub fn out_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the OUT register."]
  #[inline] pub fn out(&self) -> Out { 
     unsafe {
        Out(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the OUT register."]
  #[inline] pub fn set_out<F: FnOnce(Out) -> Out>(&self, f: F) -> &Self {
     let value = f(Out(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the OUT register."]
  #[inline] pub fn with_out<F: FnOnce(Out) -> Out>(&self, f: F) -> &Self {
     let tmp = self.out();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the OUTCLR register."]
  #[inline] pub fn outclr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the OUTCLR register."]
  #[inline] pub fn outclr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Read the OUTCLR register."]
  #[inline] pub fn outclr(&self) -> Outclr { 
     unsafe {
        Outclr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
#[doc="Write the OUTCLR register."]
  #[inline] pub fn set_outclr<F: FnOnce(Outclr) -> Outclr>(&self, f: F) -> &Self {
     let value = f(Outclr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the OUTCLR register."]
  #[inline] pub fn with_outclr<F: FnOnce(Outclr) -> Outclr>(&self, f: F) -> &Self {
     let tmp = self.outclr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the OUTSET register."]
  #[inline] pub fn outset_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
#[doc="Get the *mut pointer for the OUTSET register."]
  #[inline] pub fn outset_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
#[doc="Read the OUTSET register."]
  #[inline] pub fn outset(&self) -> Outset { 
     unsafe {
        Outset(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
#[doc="Write the OUTSET register."]
  #[inline] pub fn set_outset<F: FnOnce(Outset) -> Outset>(&self, f: F) -> &Self {
     let value = f(Outset(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the OUTSET register."]
  #[inline] pub fn with_outset<F: FnOnce(Outset) -> Outset>(&self, f: F) -> &Self {
     let tmp = self.outset();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the OUTTGL register."]
  #[inline] pub fn outtgl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the OUTTGL register."]
  #[inline] pub fn outtgl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Read the OUTTGL register."]
  #[inline] pub fn outtgl(&self) -> Outtgl { 
     unsafe {
        Outtgl(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
#[doc="Write the OUTTGL register."]
  #[inline] pub fn set_outtgl<F: FnOnce(Outtgl) -> Outtgl>(&self, f: F) -> &Self {
     let value = f(Outtgl(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the OUTTGL register."]
  #[inline] pub fn with_outtgl<F: FnOnce(Outtgl) -> Outtgl>(&self, f: F) -> &Self {
     let tmp = self.outtgl();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the PINCFG register."]
  #[inline] pub fn pincfg_ptr<I: Into<bits::R32>>(&self, index: I) -> *const u8 { 
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     ((self.0 as usize) + 0x40 + (index)) as *const u8
  }
#[doc="Get the *mut pointer for the PINCFG register."]
  #[inline] pub fn pincfg_mut<I: Into<bits::R32>>(&self, index: I) -> *mut u8 { 
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     ((self.0 as usize) + 0x40 + (index)) as *mut u8
  }
#[doc="Read the PINCFG register."]
  #[inline] pub fn pincfg<I: Into<bits::R32>>(&self, index: I) -> Pincfg { 
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     unsafe {
        Pincfg(::core::ptr::read_volatile(((self.0 as usize) + 0x40 + (index)) as *const u8))
     }
  }
#[doc="Write the PINCFG register."]
  #[inline] pub fn set_pincfg<I: Into<bits::R32>, F: FnOnce(Pincfg) -> Pincfg>(&self, index: I, f: F) -> &Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value = f(Pincfg(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x40 + (index)) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the PINCFG register."]
  #[inline] pub fn with_pincfg<I: Into<bits::R32> + Copy, F: FnOnce(Pincfg) -> Pincfg>(&self, index: I, f: F) -> &Self {
     let tmp = self.pincfg(index);
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u8, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the PMUX register."]
  #[inline] pub fn pmux_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u8 { 
     let index: bits::R16 = index.into();
     let index: usize = index.value();
     ((self.0 as usize) + 0x30 + (index)) as *const u8
  }
#[doc="Get the *mut pointer for the PMUX register."]
  #[inline] pub fn pmux_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u8 { 
     let index: bits::R16 = index.into();
     let index: usize = index.value();
     ((self.0 as usize) + 0x30 + (index)) as *mut u8
  }
#[doc="Read the PMUX register."]
  #[inline] pub fn pmux<I: Into<bits::R16>>(&self, index: I) -> Pmux { 
     let index: bits::R16 = index.into();
     let index: usize = index.value();
     unsafe {
        Pmux(::core::ptr::read_volatile(((self.0 as usize) + 0x30 + (index)) as *const u8))
     }
  }
#[doc="Write the PMUX register."]
  #[inline] pub fn set_pmux<I: Into<bits::R16>, F: FnOnce(Pmux) -> Pmux>(&self, index: I, f: F) -> &Self {
     let index: bits::R16 = index.into();
     let index: usize = index.value();
     let value = f(Pmux(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x30 + (index)) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the PMUX register."]
  #[inline] pub fn with_pmux<I: Into<bits::R16> + Copy, F: FnOnce(Pmux) -> Pmux>(&self, index: I, f: F) -> &Self {
     let tmp = self.pmux(index);
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u8, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the WRCONFIG register."]
  #[inline] pub fn wrconfig_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x28) as *const u32
  }
#[doc="Get the *mut pointer for the WRCONFIG register."]
  #[inline] pub fn wrconfig_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x28) as *mut u32
  }
#[doc="Write the WRCONFIG register."]
  #[inline] pub fn set_wrconfig<F: FnOnce(Wrconfig) -> Wrconfig>(&self, f: F) -> &Self {
     let value = f(Wrconfig(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
     }
     self
  }

}

#[doc="Control"]
#[derive(PartialEq, Eq)]
pub struct Ctrl(pub u32);
impl Ctrl {
#[doc="Input Sampling Mode"]
  #[inline] pub fn sampling(&self) -> bits::U32 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
  }
#[doc="Input Sampling Mode"]
  #[inline] pub fn set_sampling<V: Into<bits::U32>>(mut self, value: V) -> Self {
     let value: bits::U32 = value.into();
     let value: u32 = value.into();
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
#[doc="Data Direction"]
#[derive(PartialEq, Eq)]
pub struct Dir(pub u32);
impl Dir {
#[doc="Port Data Direction"]
  #[inline] pub fn dir<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Port Data Direction"]
  #[inline] pub fn set_dir<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
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
#[doc="Data Direction Clear"]
#[derive(PartialEq, Eq)]
pub struct Dirclr(pub u32);
impl Dirclr {
#[doc="Port Data Direction Clear"]
  #[inline] pub fn dirclr<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Port Data Direction Clear"]
  #[inline] pub fn set_dirclr<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
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
#[doc="Data Direction Set"]
#[derive(PartialEq, Eq)]
pub struct Dirset(pub u32);
impl Dirset {
#[doc="Port Data Direction Set"]
  #[inline] pub fn dirset<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Port Data Direction Set"]
  #[inline] pub fn set_dirset<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
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
#[doc="Data Direction Toggle"]
#[derive(PartialEq, Eq)]
pub struct Dirtgl(pub u32);
impl Dirtgl {
#[doc="Port Data Direction Toggle"]
  #[inline] pub fn dirtgl<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Port Data Direction Toggle"]
  #[inline] pub fn set_dirtgl<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
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
#[doc="Data Input Value"]
#[derive(PartialEq, Eq)]
pub struct In(pub u32);
impl In {
#[doc="Port Data Input Value"]
  #[inline] pub fn _in<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Port Data Input Value"]
  #[inline] pub fn set_in<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
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
#[doc="Data Output Value"]
#[derive(PartialEq, Eq)]
pub struct Out(pub u32);
impl Out {
#[doc="Port Data Output Value"]
  #[inline] pub fn out<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Port Data Output Value"]
  #[inline] pub fn set_out<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
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
#[doc="Data Output Value Clear"]
#[derive(PartialEq, Eq)]
pub struct Outclr(pub u32);
impl Outclr {
#[doc="Port Data Output Value Clear"]
  #[inline] pub fn outclr<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Port Data Output Value Clear"]
  #[inline] pub fn set_outclr<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
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
#[doc="Data Output Value Set"]
#[derive(PartialEq, Eq)]
pub struct Outset(pub u32);
impl Outset {
#[doc="Port Data Output Value Set"]
  #[inline] pub fn outset<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Port Data Output Value Set"]
  #[inline] pub fn set_outset<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
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
#[doc="Data Output Value Toggle"]
#[derive(PartialEq, Eq)]
pub struct Outtgl(pub u32);
impl Outtgl {
#[doc="Port Data Output Value Toggle"]
  #[inline] pub fn outtgl<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Port Data Output Value Toggle"]
  #[inline] pub fn set_outtgl<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
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
#[doc="Pin Configuration n"]
#[derive(PartialEq, Eq)]
pub struct Pincfg(pub u8);
impl Pincfg {
#[doc="Peripheral Multiplexer Enable"]
  #[inline] pub fn pmuxen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Peripheral Multiplexer Enable"]
  #[inline] pub fn set_pmuxen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Input Enable"]
  #[inline] pub fn inen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Input Enable"]
  #[inline] pub fn set_inen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Pull Enable"]
  #[inline] pub fn pullen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="Pull Enable"]
  #[inline] pub fn set_pullen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Output Driver Strength Selection"]
  #[inline] pub fn drvstr(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="Output Driver Strength Selection"]
  #[inline] pub fn set_drvstr<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
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
#[doc="Peripheral Multiplexing n - Group 0"]
#[derive(PartialEq, Eq)]
pub struct Pmux(pub u8);
impl Pmux {
#[doc="Peripheral Multiplexing Even"]
  #[inline] pub fn pmux<I: Into<bits::R2>>(&self, index: I) -> bits::U4 {
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + (index << 2);
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xf) as u8) } // [3:0]
  }
#[doc="Peripheral Multiplexing Even"]
  #[inline] pub fn set_pmux<I: Into<bits::R2>, V: Into<bits::U4>>(mut self, index: I, value: V) -> Self {
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     let value: bits::U4 = value.into();
     let value: u8 = value.into();
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
#[doc="Write Configuration"]
#[derive(PartialEq, Eq)]
pub struct Wrconfig(pub u32);
impl Wrconfig {
#[doc="Pin Mask for Multiple Pin Configuration"]
  #[inline] pub fn pinmask(&self) -> bits::U16 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
  }
#[doc="Pin Mask for Multiple Pin Configuration"]
  #[inline] pub fn set_pinmask<V: Into<bits::U16>>(mut self, value: V) -> Self {
     let value: bits::U16 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Peripheral Multiplexer Enable"]
  #[inline] pub fn pmuxen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
  }
#[doc="Peripheral Multiplexer Enable"]
  #[inline] pub fn set_pmuxen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Input Enable"]
  #[inline] pub fn inen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
  }
#[doc="Input Enable"]
  #[inline] pub fn set_inen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="Pull Enable"]
  #[inline] pub fn pullen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
  }
#[doc="Pull Enable"]
  #[inline] pub fn set_pullen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="Output Driver Strength Selection"]
  #[inline] pub fn drvstr(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
  }
#[doc="Output Driver Strength Selection"]
  #[inline] pub fn set_drvstr<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Peripheral Multiplexing"]
  #[inline] pub fn pmux(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
  }
#[doc="Peripheral Multiplexing"]
  #[inline] pub fn set_pmux<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Write PMUX"]
  #[inline] pub fn wrpmux(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
  }
#[doc="Write PMUX"]
  #[inline] pub fn set_wrpmux<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="Write PINCFG"]
  #[inline] pub fn wrpincfg(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
  }
#[doc="Write PINCFG"]
  #[inline] pub fn set_wrpincfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Half-Word Select"]
  #[inline] pub fn hwsel(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
  }
#[doc="Half-Word Select"]
  #[inline] pub fn set_hwsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
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
#[doc="PORT Pin"]
pub struct Pin<P, T> { pub port: Periph<T>, pub index: usize, pub id: P }

impl<P,T> Pin<P,T> {
   #[inline] pub fn port(&self) -> &Periph<T> { &self.port }
   #[inline] pub fn index(&self) -> usize { self.index }
}
pub trait AltFn<T> {
   fn alt_fn(&self) -> usize;
}

pub const PA00: Pin<Pa00Id, PortaId> = Pin { port: PORTA, index: 0, id: Pa00Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa00Id {}
pub type Pa00 = Pin<Pa00Id, PortaId>;
impl AltFn<super::sig::Extint0> for Pa00Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom1Pad0> for Pa00Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc2Wo0> for Pa00Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PA01: Pin<Pa01Id, PortaId> = Pin { port: PORTA, index: 1, id: Pa01Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa01Id {}
pub type Pa01 = Pin<Pa01Id, PortaId>;
impl AltFn<super::sig::Extint1> for Pa01Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom1Pad1> for Pa01Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc2Wo1> for Pa01Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PA02: Pin<Pa02Id, PortaId> = Pin { port: PORTA, index: 2, id: Pa02Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa02Id {}
pub type Pa02 = Pin<Pa02Id, PortaId>;
impl AltFn<super::sig::Extint2> for Pa02Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain0> for Pa02Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y0> for Pa02Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Vout> for Pa02Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PA03: Pin<Pa03Id, PortaId> = Pin { port: PORTA, index: 3, id: Pa03Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa03Id {}
pub type Pa03 = Pin<Pa03Id, PortaId>;
impl AltFn<super::sig::Extint3> for Pa03Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc> for Pa03Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Vrefadac> for Pa03Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Vrefa> for Pa03Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ain1> for Pa03Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y1> for Pa03Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PA04: Pin<Pa04Id, PortaId> = Pin { port: PORTA, index: 4, id: Pa04Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa04Id {}
pub type Pa04 = Pin<Pa04Id, PortaId>;
impl AltFn<super::sig::Extint4> for Pa04Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc> for Pa04Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Vrefb> for Pa04Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ain4> for Pa04Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ain0> for Pa04Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y2> for Pa04Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom0Pad0> for Pa04Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc0Wo0> for Pa04Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PA05: Pin<Pa05Id, PortaId> = Pin { port: PORTA, index: 5, id: Pa05Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa05Id {}
pub type Pa05 = Pin<Pa05Id, PortaId>;
impl AltFn<super::sig::Extint5> for Pa05Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain5> for Pa05Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ain1> for Pa05Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y3> for Pa05Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom0Pad1> for Pa05Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc0Wo1> for Pa05Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PA06: Pin<Pa06Id, PortaId> = Pin { port: PORTA, index: 6, id: Pa06Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa06Id {}
pub type Pa06 = Pin<Pa06Id, PortaId>;
impl AltFn<super::sig::Extint6> for Pa06Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain6> for Pa06Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ain2> for Pa06Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y4> for Pa06Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom0Pad2> for Pa06Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc1Wo0> for Pa06Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PA07: Pin<Pa07Id, PortaId> = Pin { port: PORTA, index: 7, id: Pa07Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa07Id {}
pub type Pa07 = Pin<Pa07Id, PortaId>;
impl AltFn<super::sig::Extint7> for Pa07Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain7> for Pa07Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ain3> for Pa07Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y5> for Pa07Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom0Pad3> for Pa07Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc1Wo1> for Pa07Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2sSd0> for Pa07Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PA08: Pin<Pa08Id, PortaId> = Pin { port: PORTA, index: 8, id: Pa08Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa08Id {}
pub type Pa08 = Pin<Pa08Id, PortaId>;
impl AltFn<super::sig::Nmi> for Pa08Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain16> for Pa08Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::X0> for Pa08Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom0Pad0> for Pa08Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom2Pad0> for Pa08Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc0Wo0> for Pa08Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc1Wo2> for Pa08Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2sSd1> for Pa08Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PA09: Pin<Pa09Id, PortaId> = Pin { port: PORTA, index: 9, id: Pa09Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa09Id {}
pub type Pa09 = Pin<Pa09Id, PortaId>;
impl AltFn<super::sig::Extint9> for Pa09Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain17> for Pa09Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::X1> for Pa09Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom0Pad1> for Pa09Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom2Pad1> for Pa09Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc0Wo1> for Pa09Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc1Wo3> for Pa09Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2sMck0> for Pa09Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PA10: Pin<Pa10Id, PortaId> = Pin { port: PORTA, index: 10, id: Pa10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa10Id {}
pub type Pa10 = Pin<Pa10Id, PortaId>;
impl AltFn<super::sig::Extint10> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain18> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::X2> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom0Pad2> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom2Pad2> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc1Wo0> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0Wo2> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2sSck0> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::GclkIo4> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA11: Pin<Pa11Id, PortaId> = Pin { port: PORTA, index: 11, id: Pa11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa11Id {}
pub type Pa11 = Pin<Pa11Id, PortaId>;
impl AltFn<super::sig::Extint11> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain19> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::X3> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom0Pad3> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom2Pad3> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc1Wo1> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0Wo3> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2sFs0> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::GclkIo5> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA12: Pin<Pa12Id, PortaId> = Pin { port: PORTA, index: 12, id: Pa12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa12Id {}
pub type Pa12 = Pin<Pa12Id, PortaId>;
impl AltFn<super::sig::Extint12> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom2Pad0> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom4Pad0> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc2Wo0> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0Wo6> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::AcCmp0> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA13: Pin<Pa13Id, PortaId> = Pin { port: PORTA, index: 13, id: Pa13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa13Id {}
pub type Pa13 = Pin<Pa13Id, PortaId>;
impl AltFn<super::sig::Extint13> for Pa13Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom2Pad1> for Pa13Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom4Pad1> for Pa13Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc2Wo1> for Pa13Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0Wo7> for Pa13Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::AcCmp1> for Pa13Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA14: Pin<Pa14Id, PortaId> = Pin { port: PORTA, index: 14, id: Pa14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa14Id {}
pub type Pa14 = Pin<Pa14Id, PortaId>;
impl AltFn<super::sig::Extint14> for Pa14Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom2Pad2> for Pa14Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom4Pad2> for Pa14Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc3Wo0> for Pa14Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0Wo4> for Pa14Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::GclkIo0> for Pa14Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA15: Pin<Pa15Id, PortaId> = Pin { port: PORTA, index: 15, id: Pa15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa15Id {}
pub type Pa15 = Pin<Pa15Id, PortaId>;
impl AltFn<super::sig::Extint15> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom2Pad3> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom4Pad3> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc3Wo1> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0Wo5> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::GclkIo1> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA16: Pin<Pa16Id, PortaId> = Pin { port: PORTA, index: 16, id: Pa16Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa16Id {}
pub type Pa16 = Pin<Pa16Id, PortaId>;
impl AltFn<super::sig::Extint0> for Pa16Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::X4> for Pa16Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom1Pad0> for Pa16Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom3Pad0> for Pa16Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc2Wo0> for Pa16Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0Wo6> for Pa16Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::GclkIo2> for Pa16Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA17: Pin<Pa17Id, PortaId> = Pin { port: PORTA, index: 17, id: Pa17Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa17Id {}
pub type Pa17 = Pin<Pa17Id, PortaId>;
impl AltFn<super::sig::Extint1> for Pa17Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::X5> for Pa17Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom1Pad1> for Pa17Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom3Pad1> for Pa17Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc2Wo1> for Pa17Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0Wo7> for Pa17Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::GclkIo3> for Pa17Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA18: Pin<Pa18Id, PortaId> = Pin { port: PORTA, index: 18, id: Pa18Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa18Id {}
pub type Pa18 = Pin<Pa18Id, PortaId>;
impl AltFn<super::sig::Extint2> for Pa18Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::X6> for Pa18Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom1Pad2> for Pa18Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom3Pad2> for Pa18Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc3Wo0> for Pa18Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0Wo2> for Pa18Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::AcCmp0> for Pa18Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA19: Pin<Pa19Id, PortaId> = Pin { port: PORTA, index: 19, id: Pa19Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa19Id {}
pub type Pa19 = Pin<Pa19Id, PortaId>;
impl AltFn<super::sig::Extint3> for Pa19Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::X7> for Pa19Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom1Pad3> for Pa19Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom3Pad3> for Pa19Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc3Wo1> for Pa19Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0Wo3> for Pa19Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2sSd0> for Pa19Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::AcCmp1> for Pa19Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA20: Pin<Pa20Id, PortaId> = Pin { port: PORTA, index: 20, id: Pa20Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa20Id {}
pub type Pa20 = Pin<Pa20Id, PortaId>;
impl AltFn<super::sig::Extint4> for Pa20Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::X8> for Pa20Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom5Pad2> for Pa20Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom3Pad2> for Pa20Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc7Wo0> for Pa20Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0Wo6> for Pa20Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2sSck0> for Pa20Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::GclkIo4> for Pa20Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA21: Pin<Pa21Id, PortaId> = Pin { port: PORTA, index: 21, id: Pa21Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa21Id {}
pub type Pa21 = Pin<Pa21Id, PortaId>;
impl AltFn<super::sig::Extint5> for Pa21Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::X9> for Pa21Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom5Pad3> for Pa21Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom3Pad3> for Pa21Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc7Wo1> for Pa21Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0Wo7> for Pa21Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2sFs0> for Pa21Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::GclkIo5> for Pa21Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA22: Pin<Pa22Id, PortaId> = Pin { port: PORTA, index: 22, id: Pa22Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa22Id {}
pub type Pa22 = Pin<Pa22Id, PortaId>;
impl AltFn<super::sig::Extint6> for Pa22Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::X10> for Pa22Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom3Pad0> for Pa22Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom5Pad0> for Pa22Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc4Wo0> for Pa22Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0Wo4> for Pa22Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::GclkIo6> for Pa22Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA23: Pin<Pa23Id, PortaId> = Pin { port: PORTA, index: 23, id: Pa23Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa23Id {}
pub type Pa23 = Pin<Pa23Id, PortaId>;
impl AltFn<super::sig::Extint7> for Pa23Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::X11> for Pa23Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom3Pad1> for Pa23Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom5Pad1> for Pa23Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc4Wo1> for Pa23Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0Wo5> for Pa23Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::UsbSof1khz> for Pa23Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::GclkIo7> for Pa23Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA24: Pin<Pa24Id, PortaId> = Pin { port: PORTA, index: 24, id: Pa24Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa24Id {}
pub type Pa24 = Pin<Pa24Id, PortaId>;
impl AltFn<super::sig::Extint12> for Pa24Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom3Pad2> for Pa24Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom5Pad2> for Pa24Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc5Wo0> for Pa24Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc1Wo2> for Pa24Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::UsbDm> for Pa24Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PA25: Pin<Pa25Id, PortaId> = Pin { port: PORTA, index: 25, id: Pa25Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa25Id {}
pub type Pa25 = Pin<Pa25Id, PortaId>;
impl AltFn<super::sig::Extint13> for Pa25Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom3Pad3> for Pa25Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sercom5Pad3> for Pa25Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc5Wo1> for Pa25Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc1Wo3> for Pa25Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::UsbDp> for Pa25Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PA27: Pin<Pa27Id, PortaId> = Pin { port: PORTA, index: 27, id: Pa27Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa27Id {}
pub type Pa27 = Pin<Pa27Id, PortaId>;
impl AltFn<super::sig::Extint15> for Pa27Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::GclkIo0> for Pa27Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA28: Pin<Pa28Id, PortaId> = Pin { port: PORTA, index: 28, id: Pa28Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa28Id {}
pub type Pa28 = Pin<Pa28Id, PortaId>;
impl AltFn<super::sig::Extint8> for Pa28Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::GclkIo0> for Pa28Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA30: Pin<Pa30Id, PortaId> = Pin { port: PORTA, index: 30, id: Pa30Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa30Id {}
pub type Pa30 = Pin<Pa30Id, PortaId>;
impl AltFn<super::sig::Extint10> for Pa30Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom1Pad2> for Pa30Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc1Wo0> for Pa30Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Swclk> for Pa30Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::GclkIo0> for Pa30Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA31: Pin<Pa31Id, PortaId> = Pin { port: PORTA, index: 31, id: Pa31Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa31Id {}
pub type Pa31 = Pin<Pa31Id, PortaId>;
impl AltFn<super::sig::Extint11> for Pa31Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom1Pad3> for Pa31Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc1Wo1> for Pa31Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Swdio> for Pa31Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PB00: Pin<Pb00Id, PortbId> = Pin { port: PORTB, index: 0, id: Pb00Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb00Id {}
pub type Pb00 = Pin<Pb00Id, PortbId>;
impl AltFn<super::sig::Extint0> for Pb00Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain8> for Pb00Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y6> for Pb00Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom5Pad2> for Pb00Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc7Wo0> for Pb00Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PB01: Pin<Pb01Id, PortbId> = Pin { port: PORTB, index: 1, id: Pb01Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb01Id {}
pub type Pb01 = Pin<Pb01Id, PortbId>;
impl AltFn<super::sig::Extint1> for Pb01Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain9> for Pb01Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y7> for Pb01Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom5Pad3> for Pb01Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc7Wo1> for Pb01Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PB02: Pin<Pb02Id, PortbId> = Pin { port: PORTB, index: 2, id: Pb02Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb02Id {}
pub type Pb02 = Pin<Pb02Id, PortbId>;
impl AltFn<super::sig::Extint2> for Pb02Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain10> for Pb02Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y8> for Pb02Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom5Pad0> for Pb02Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc6Wo0> for Pb02Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PB03: Pin<Pb03Id, PortbId> = Pin { port: PORTB, index: 3, id: Pb03Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb03Id {}
pub type Pb03 = Pin<Pb03Id, PortbId>;
impl AltFn<super::sig::Extint3> for Pb03Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain11> for Pb03Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y9> for Pb03Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom5Pad1> for Pb03Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc6Wo1> for Pb03Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PB04: Pin<Pb04Id, PortbId> = Pin { port: PORTB, index: 4, id: Pb04Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb04Id {}
pub type Pb04 = Pin<Pb04Id, PortbId>;
impl AltFn<super::sig::Extint4> for Pb04Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain12> for Pb04Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y10> for Pb04Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PB05: Pin<Pb05Id, PortbId> = Pin { port: PORTB, index: 5, id: Pb05Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb05Id {}
pub type Pb05 = Pin<Pb05Id, PortbId>;
impl AltFn<super::sig::Extint5> for Pb05Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain13> for Pb05Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y11> for Pb05Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PB06: Pin<Pb06Id, PortbId> = Pin { port: PORTB, index: 6, id: Pb06Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb06Id {}
pub type Pb06 = Pin<Pb06Id, PortbId>;
impl AltFn<super::sig::Extint6> for Pb06Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain14> for Pb06Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y12> for Pb06Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PB07: Pin<Pb07Id, PortbId> = Pin { port: PORTB, index: 7, id: Pb07Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb07Id {}
pub type Pb07 = Pin<Pb07Id, PortbId>;
impl AltFn<super::sig::Extint7> for Pb07Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain15> for Pb07Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y13> for Pb07Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PB08: Pin<Pb08Id, PortbId> = Pin { port: PORTB, index: 8, id: Pb08Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb08Id {}
pub type Pb08 = Pin<Pb08Id, PortbId>;
impl AltFn<super::sig::Extint8> for Pb08Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain2> for Pb08Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y14> for Pb08Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom4Pad0> for Pb08Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc4Wo0> for Pb08Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PB09: Pin<Pb09Id, PortbId> = Pin { port: PORTB, index: 9, id: Pb09Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb09Id {}
pub type Pb09 = Pin<Pb09Id, PortbId>;
impl AltFn<super::sig::Extint9> for Pb09Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain3> for Pb09Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Y15> for Pb09Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom4Pad1> for Pb09Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc4Wo1> for Pb09Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PB10: Pin<Pb10Id, PortbId> = Pin { port: PORTB, index: 10, id: Pb10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb10Id {}
pub type Pb10 = Pin<Pb10Id, PortbId>;
impl AltFn<super::sig::Extint10> for Pb10Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom4Pad2> for Pb10Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc5Wo0> for Pb10Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0Wo4> for Pb10Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2sMck1> for Pb10Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::GclkIo4> for Pb10Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PB11: Pin<Pb11Id, PortbId> = Pin { port: PORTB, index: 11, id: Pb11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb11Id {}
pub type Pb11 = Pin<Pb11Id, PortbId>;
impl AltFn<super::sig::Extint11> for Pb11Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom4Pad3> for Pb11Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc5Wo1> for Pb11Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0Wo5> for Pb11Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2sSck1> for Pb11Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::GclkIo5> for Pb11Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PB12: Pin<Pb12Id, PortbId> = Pin { port: PORTB, index: 12, id: Pb12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb12Id {}
pub type Pb12 = Pin<Pb12Id, PortbId>;
impl AltFn<super::sig::Extint12> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::X12> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom4Pad0> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tc4Wo0> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0Wo6> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2sFs1> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::GclkIo6> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PB13: Pin<Pb13Id, PortbId> = Pin { port: PORTB, index: 13, id: Pb13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb13Id {}
pub type Pb13 = Pin<Pb13Id, PortbId>;
impl AltFn<super::sig::Extint13> for Pb13Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::X13> for Pb13Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom4Pad1> for Pb13Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tc4Wo1> for Pb13Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0Wo7> for Pb13Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::GclkIo7> for Pb13Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PB14: Pin<Pb14Id, PortbId> = Pin { port: PORTB, index: 14, id: Pb14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb14Id {}
pub type Pb14 = Pin<Pb14Id, PortbId>;
impl AltFn<super::sig::Extint14> for Pb14Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::X14> for Pb14Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom4Pad2> for Pb14Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tc5Wo0> for Pb14Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::GclkIo0> for Pb14Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PB15: Pin<Pb15Id, PortbId> = Pin { port: PORTB, index: 15, id: Pb15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb15Id {}
pub type Pb15 = Pin<Pb15Id, PortbId>;
impl AltFn<super::sig::Extint15> for Pb15Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::X15> for Pb15Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Sercom4Pad3> for Pb15Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tc5Wo1> for Pb15Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::GclkIo1> for Pb15Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PB16: Pin<Pb16Id, PortbId> = Pin { port: PORTB, index: 16, id: Pb16Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb16Id {}
pub type Pb16 = Pin<Pb16Id, PortbId>;
impl AltFn<super::sig::Extint0> for Pb16Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom5Pad0> for Pb16Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tc6Wo0> for Pb16Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0Wo4> for Pb16Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2sSd1> for Pb16Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::GclkIo2> for Pb16Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PB17: Pin<Pb17Id, PortbId> = Pin { port: PORTB, index: 17, id: Pb17Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb17Id {}
pub type Pb17 = Pin<Pb17Id, PortbId>;
impl AltFn<super::sig::Extint1> for Pb17Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom5Pad1> for Pb17Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tc6Wo1> for Pb17Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc0Wo5> for Pb17Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2sMck0> for Pb17Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::GclkIo3> for Pb17Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PB22: Pin<Pb22Id, PortbId> = Pin { port: PORTB, index: 22, id: Pb22Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb22Id {}
pub type Pb22 = Pin<Pb22Id, PortbId>;
impl AltFn<super::sig::Extint6> for Pb22Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom5Pad2> for Pb22Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc7Wo0> for Pb22Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::GclkIo0> for Pb22Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PB23: Pin<Pb23Id, PortbId> = Pin { port: PORTB, index: 23, id: Pb23Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb23Id {}
pub type Pb23 = Pin<Pb23Id, PortbId>;
impl AltFn<super::sig::Extint7> for Pb23Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom5Pad3> for Pb23Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tc7Wo1> for Pb23Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::GclkIo1> for Pb23Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PB30: Pin<Pb30Id, PortbId> = Pin { port: PORTB, index: 30, id: Pb30Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb30Id {}
pub type Pb30 = Pin<Pb30Id, PortbId>;
impl AltFn<super::sig::Extint14> for Pb30Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom5Pad0> for Pb30Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc0Wo0> for Pb30Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc1Wo2> for Pb30Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PB31: Pin<Pb31Id, PortbId> = Pin { port: PORTB, index: 31, id: Pb31Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb31Id {}
pub type Pb31 = Pin<Pb31Id, PortbId>;
impl AltFn<super::sig::Extint15> for Pb31Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sercom5Pad1> for Pb31Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tcc0Wo1> for Pb31Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tcc1Wo3> for Pb31Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

