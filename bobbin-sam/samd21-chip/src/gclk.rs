pub const GCLK: Gclk = Gclk(0x40000c00);

#[doc="Generic Clock Generator"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Gclk(pub u32);
impl Gclk {
#[doc="Get the *const pointer for the CLKCTRL register."]
  #[inline] pub fn clkctrl_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x2) as *const u16
  }
#[doc="Get the *mut pointer for the CLKCTRL register."]
  #[inline] pub fn clkctrl_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x2) as *mut u16
  }
#[doc="Read the CLKCTRL register."]
  #[inline] pub fn clkctrl(&self) -> Clkctrl { 
     unsafe {
        Clkctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x2) as *const u16))
     }
  }
#[doc="Write the CLKCTRL register."]
  #[inline] pub fn set_clkctrl(&self, value: Clkctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the CLKCTRL register."]
  #[inline] pub fn with_clkctrl<F: FnOnce(Clkctrl) -> Clkctrl>(&self, f: F) -> &Self {
     let tmp = self.clkctrl();
     self.set_clkctrl(f(tmp))
  }

#[doc="Get the *const pointer for the CTRL register."]
  #[inline] pub fn ctrl_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x0) as *const u8
  }
#[doc="Get the *mut pointer for the CTRL register."]
  #[inline] pub fn ctrl_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x0) as *mut u8
  }
#[doc="Read the CTRL register."]
  #[inline] pub fn ctrl(&self) -> Ctrl { 
     unsafe {
        Ctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u8))
     }
  }
#[doc="Write the CTRL register."]
  #[inline] pub fn set_ctrl(&self, value: Ctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the CTRL register."]
  #[inline] pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
     let tmp = self.ctrl();
     self.set_ctrl(f(tmp))
  }

#[doc="Get the *const pointer for the GENCTRL register."]
  #[inline] pub fn genctrl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the GENCTRL register."]
  #[inline] pub fn genctrl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the GENCTRL register."]
  #[inline] pub fn genctrl(&self) -> Genctrl { 
     unsafe {
        Genctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the GENCTRL register."]
  #[inline] pub fn set_genctrl(&self, value: Genctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the GENCTRL register."]
  #[inline] pub fn with_genctrl<F: FnOnce(Genctrl) -> Genctrl>(&self, f: F) -> &Self {
     let tmp = self.genctrl();
     self.set_genctrl(f(tmp))
  }

#[doc="Get the *const pointer for the GENDIV register."]
  #[inline] pub fn gendiv_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the GENDIV register."]
  #[inline] pub fn gendiv_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the GENDIV register."]
  #[inline] pub fn gendiv(&self) -> Gendiv { 
     unsafe {
        Gendiv(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the GENDIV register."]
  #[inline] pub fn set_gendiv(&self, value: Gendiv) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the GENDIV register."]
  #[inline] pub fn with_gendiv<F: FnOnce(Gendiv) -> Gendiv>(&self, f: F) -> &Self {
     let tmp = self.gendiv();
     self.set_gendiv(f(tmp))
  }

#[doc="Get the *const pointer for the STATUS register."]
  #[inline] pub fn status_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x1) as *const u8
  }
#[doc="Get the *mut pointer for the STATUS register."]
  #[inline] pub fn status_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x1) as *mut u8
  }
#[doc="Read the STATUS register."]
  #[inline] pub fn status(&self) -> Status { 
     unsafe {
        Status(::core::ptr::read_volatile(((self.0 as usize) + 0x1) as *const u8))
     }
  }

}

#[doc="Generic Clock Control"]
#[derive(PartialEq, Eq)]
pub struct Clkctrl(pub u16);
impl Clkctrl {
#[doc="Generic Clock Selection ID"]
  #[inline] pub fn id(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x3f // [5:0]
  }
#[doc="Generic Clock Selection ID"]
  #[inline] pub fn set_id(mut self, value: u16) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Generic Clock Generator"]
  #[inline] pub fn gen(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0xf // [11:8]
  }
#[doc="Generic Clock Generator"]
  #[inline] pub fn set_gen(mut self, value: u16) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Clock Enable"]
  #[inline] pub fn clken(&self) -> u16 {
     ((self.0 as u16) >> 14) & 0x1 // [14]
  }
#[doc="Clock Enable"]
  #[inline] pub fn set_clken(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Write Lock"]
  #[inline] pub fn wrtlock(&self) -> u16 {
     ((self.0 as u16) >> 15) & 0x1 // [15]
  }
#[doc="Write Lock"]
  #[inline] pub fn set_wrtlock(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

}
impl ::core::fmt::Display for Clkctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clkctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
      if self.gen() != 0 { try!(write!(f, " gen=0x{:x}", self.gen()))}
      if self.clken() != 0 { try!(write!(f, " clken"))}
      if self.wrtlock() != 0 { try!(write!(f, " wrtlock"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control"]
#[derive(PartialEq, Eq)]
pub struct Ctrl(pub u8);
impl Ctrl {
#[doc="Software Reset"]
  #[inline] pub fn swrst(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Software Reset"]
  #[inline] pub fn set_swrst(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
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
      if self.swrst() != 0 { try!(write!(f, " swrst"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Generic Clock Generator Control"]
#[derive(PartialEq, Eq)]
pub struct Genctrl(pub u32);
impl Genctrl {
#[doc="Generic Clock Generator Selection"]
  #[inline] pub fn id(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
#[doc="Generic Clock Generator Selection"]
  #[inline] pub fn set_id(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Source Select"]
  #[inline] pub fn src(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1f // [12:8]
  }
#[doc="Source Select"]
  #[inline] pub fn set_src(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Generic Clock Generator Enable"]
  #[inline] pub fn genen(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="Generic Clock Generator Enable"]
  #[inline] pub fn set_genen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Improve Duty Cycle"]
  #[inline] pub fn idc(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="Improve Duty Cycle"]
  #[inline] pub fn set_idc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="Output Off Value"]
  #[inline] pub fn oov(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="Output Off Value"]
  #[inline] pub fn set_oov(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="Output Enable"]
  #[inline] pub fn oe(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
#[doc="Output Enable"]
  #[inline] pub fn set_oe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="Divide Selection"]
  #[inline] pub fn divsel(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
#[doc="Divide Selection"]
  #[inline] pub fn set_divsel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Run in Standby"]
  #[inline] pub fn runstdby(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
#[doc="Run in Standby"]
  #[inline] pub fn set_runstdby(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

}
impl ::core::fmt::Display for Genctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Genctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
      if self.src() != 0 { try!(write!(f, " src=0x{:x}", self.src()))}
      if self.genen() != 0 { try!(write!(f, " genen"))}
      if self.idc() != 0 { try!(write!(f, " idc"))}
      if self.oov() != 0 { try!(write!(f, " oov"))}
      if self.oe() != 0 { try!(write!(f, " oe"))}
      if self.divsel() != 0 { try!(write!(f, " divsel"))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Generic Clock Generator Division"]
#[derive(PartialEq, Eq)]
pub struct Gendiv(pub u32);
impl Gendiv {
#[doc="Generic Clock Generator Selection"]
  #[inline] pub fn id(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
#[doc="Generic Clock Generator Selection"]
  #[inline] pub fn set_id(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Division Factor"]
  #[inline] pub fn div(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xffff // [23:8]
  }
#[doc="Division Factor"]
  #[inline] pub fn set_div(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 8);
     self.0 |= value << 8;
     self
  }

}
impl ::core::fmt::Display for Gendiv {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Gendiv {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
      if self.div() != 0 { try!(write!(f, " div=0x{:x}", self.div()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Status"]
#[derive(PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
#[doc="Synchronization Busy Status"]
  #[inline] pub fn syncbusy(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
#[doc="Synchronization Busy Status"]
  #[inline] pub fn set_syncbusy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.syncbusy() != 0 { try!(write!(f, " syncbusy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

