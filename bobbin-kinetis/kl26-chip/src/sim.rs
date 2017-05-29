pub const SIM: Sim = Sim(0x40047000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sim(pub u32);

impl Sim {
  pub unsafe fn sopt1(&self) -> Sopt1 { 
     Sopt1(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
  }
  pub unsafe fn set_sopt1(&mut self, value: Sopt1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
  }
  pub unsafe fn with_sopt1<F: FnOnce(Sopt1) -> Sopt1>(&mut self, f: F) {
     let tmp = self.sopt1();
     self.set_sopt1(f(tmp))
  }

  pub unsafe fn sopt1cfg(&self) -> Sopt1cfg { 
     Sopt1cfg(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
  }
  pub unsafe fn set_sopt1cfg(&mut self, value: Sopt1cfg) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
  }
  pub unsafe fn with_sopt1cfg<F: FnOnce(Sopt1cfg) -> Sopt1cfg>(&mut self, f: F) {
     let tmp = self.sopt1cfg();
     self.set_sopt1cfg(f(tmp))
  }

  pub unsafe fn sopt2(&self) -> Sopt2 { 
     Sopt2(::core::ptr::read_volatile(((self.0 as usize) + 0x1004) as *const u32))
  }
  pub unsafe fn set_sopt2(&mut self, value: Sopt2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1004) as *mut u32, value.0);
  }
  pub unsafe fn with_sopt2<F: FnOnce(Sopt2) -> Sopt2>(&mut self, f: F) {
     let tmp = self.sopt2();
     self.set_sopt2(f(tmp))
  }

  pub unsafe fn sopt4(&self) -> Sopt4 { 
     Sopt4(::core::ptr::read_volatile(((self.0 as usize) + 0x100c) as *const u32))
  }
  pub unsafe fn set_sopt4(&mut self, value: Sopt4) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x100c) as *mut u32, value.0);
  }
  pub unsafe fn with_sopt4<F: FnOnce(Sopt4) -> Sopt4>(&mut self, f: F) {
     let tmp = self.sopt4();
     self.set_sopt4(f(tmp))
  }

  pub unsafe fn sopt5(&self) -> Sopt5 { 
     Sopt5(::core::ptr::read_volatile(((self.0 as usize) + 0x1010) as *const u32))
  }
  pub unsafe fn set_sopt5(&mut self, value: Sopt5) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1010) as *mut u32, value.0);
  }
  pub unsafe fn with_sopt5<F: FnOnce(Sopt5) -> Sopt5>(&mut self, f: F) {
     let tmp = self.sopt5();
     self.set_sopt5(f(tmp))
  }

  pub unsafe fn sopt7(&self) -> Sopt7 { 
     Sopt7(::core::ptr::read_volatile(((self.0 as usize) + 0x1018) as *const u32))
  }
  pub unsafe fn set_sopt7(&mut self, value: Sopt7) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1018) as *mut u32, value.0);
  }
  pub unsafe fn with_sopt7<F: FnOnce(Sopt7) -> Sopt7>(&mut self, f: F) {
     let tmp = self.sopt7();
     self.set_sopt7(f(tmp))
  }

  pub unsafe fn sdid(&self) -> Sdid { 
     Sdid(::core::ptr::read_volatile(((self.0 as usize) + 0x1024) as *const u32))
  }

  pub unsafe fn scgc4(&self) -> Scgc4 { 
     Scgc4(::core::ptr::read_volatile(((self.0 as usize) + 0x1034) as *const u32))
  }
  pub unsafe fn set_scgc4(&mut self, value: Scgc4) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1034) as *mut u32, value.0);
  }
  pub unsafe fn with_scgc4<F: FnOnce(Scgc4) -> Scgc4>(&mut self, f: F) {
     let tmp = self.scgc4();
     self.set_scgc4(f(tmp))
  }

  pub unsafe fn scgc5(&self) -> Scgc5 { 
     Scgc5(::core::ptr::read_volatile(((self.0 as usize) + 0x1038) as *const u32))
  }
  pub unsafe fn set_scgc5(&mut self, value: Scgc5) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1038) as *mut u32, value.0);
  }
  pub unsafe fn with_scgc5<F: FnOnce(Scgc5) -> Scgc5>(&mut self, f: F) {
     let tmp = self.scgc5();
     self.set_scgc5(f(tmp))
  }

  pub unsafe fn scgc6(&self) -> Scgc6 { 
     Scgc6(::core::ptr::read_volatile(((self.0 as usize) + 0x103c) as *const u32))
  }
  pub unsafe fn set_scgc6(&mut self, value: Scgc6) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x103c) as *mut u32, value.0);
  }
  pub unsafe fn with_scgc6<F: FnOnce(Scgc6) -> Scgc6>(&mut self, f: F) {
     let tmp = self.scgc6();
     self.set_scgc6(f(tmp))
  }

  pub unsafe fn scgc7(&self) -> Scgc7 { 
     Scgc7(::core::ptr::read_volatile(((self.0 as usize) + 0x1040) as *const u32))
  }
  pub unsafe fn set_scgc7(&mut self, value: Scgc7) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1040) as *mut u32, value.0);
  }
  pub unsafe fn with_scgc7<F: FnOnce(Scgc7) -> Scgc7>(&mut self, f: F) {
     let tmp = self.scgc7();
     self.set_scgc7(f(tmp))
  }

  pub unsafe fn clkdiv1(&self) -> Clkdiv1 { 
     Clkdiv1(::core::ptr::read_volatile(((self.0 as usize) + 0x1044) as *const u32))
  }
  pub unsafe fn set_clkdiv1(&mut self, value: Clkdiv1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1044) as *mut u32, value.0);
  }
  pub unsafe fn with_clkdiv1<F: FnOnce(Clkdiv1) -> Clkdiv1>(&mut self, f: F) {
     let tmp = self.clkdiv1();
     self.set_clkdiv1(f(tmp))
  }

  pub unsafe fn fcfg1(&self) -> Fcfg1 { 
     Fcfg1(::core::ptr::read_volatile(((self.0 as usize) + 0x104c) as *const u32))
  }
  pub unsafe fn set_fcfg1(&mut self, value: Fcfg1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x104c) as *mut u32, value.0);
  }
  pub unsafe fn with_fcfg1<F: FnOnce(Fcfg1) -> Fcfg1>(&mut self, f: F) {
     let tmp = self.fcfg1();
     self.set_fcfg1(f(tmp))
  }

  pub unsafe fn fcfg2(&self) -> Fcfg2 { 
     Fcfg2(::core::ptr::read_volatile(((self.0 as usize) + 0x1050) as *const u32))
  }

  pub unsafe fn uidmh(&self) -> Uidmh { 
     Uidmh(::core::ptr::read_volatile(((self.0 as usize) + 0x1058) as *const u32))
  }

  pub unsafe fn uidml(&self) -> Uidml { 
     Uidml(::core::ptr::read_volatile(((self.0 as usize) + 0x105c) as *const u32))
  }

  pub unsafe fn uidl(&self) -> Uidl { 
     Uidl(::core::ptr::read_volatile(((self.0 as usize) + 0x1060) as *const u32))
  }

  pub unsafe fn copc(&self) -> Copc { 
     Copc(::core::ptr::read_volatile(((self.0 as usize) + 0x1100) as *const u32))
  }
  pub unsafe fn set_copc(&mut self, value: Copc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1100) as *mut u32, value.0);
  }
  pub unsafe fn with_copc<F: FnOnce(Copc) -> Copc>(&mut self, f: F) {
     let tmp = self.copc();
     self.set_copc(f(tmp))
  }

  pub unsafe fn set_srvcop(&mut self, value: Srvcop) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1104) as *mut u32, value.0);
  }

}

#[derive(PartialEq, Eq)]
pub struct Sopt1(pub u32);

impl Sopt1 {
  pub fn osc32ksel(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x3 // [19:18]
  }
  pub fn set_osc32ksel(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn usbvstby(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  pub fn set_usbvstby(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  pub fn usbsstby(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_usbsstby(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn usbregen(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_usbregen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}

impl ::core::fmt::Display for Sopt1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sopt1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.osc32ksel() != 0 { try!(write!(f, " osc32ksel=0x{:x}", self.osc32ksel()))}
      if self.usbvstby() != 0 { try!(write!(f, " usbvstby"))}
      if self.usbsstby() != 0 { try!(write!(f, " usbsstby"))}
      if self.usbregen() != 0 { try!(write!(f, " usbregen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Sopt1cfg(pub u32);

impl Sopt1cfg {
  pub fn urwe(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_urwe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn uvswe(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_uvswe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn usswe(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_usswe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

}

impl ::core::fmt::Display for Sopt1cfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sopt1cfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.urwe() != 0 { try!(write!(f, " urwe"))}
      if self.uvswe() != 0 { try!(write!(f, " uvswe"))}
      if self.usswe() != 0 { try!(write!(f, " usswe"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Sopt2(pub u32);

impl Sopt2 {
  pub fn rtcclkoutsel(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_rtcclkoutsel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn clkoutsel(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x7 // [7:5]
  }
  pub fn set_clkoutsel(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn pllfllsel(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_pllfllsel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn usbsrc(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_usbsrc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn tpmsrc(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x3 // [25:24]
  }
  pub fn set_tpmsrc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn uart0src(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x3 // [27:26]
  }
  pub fn set_uart0src(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 26);
     self.0 |= value << 26;
     self
  }

}

impl ::core::fmt::Display for Sopt2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sopt2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rtcclkoutsel() != 0 { try!(write!(f, " rtcclkoutsel"))}
      if self.clkoutsel() != 0 { try!(write!(f, " clkoutsel=0x{:x}", self.clkoutsel()))}
      if self.pllfllsel() != 0 { try!(write!(f, " pllfllsel"))}
      if self.usbsrc() != 0 { try!(write!(f, " usbsrc"))}
      if self.tpmsrc() != 0 { try!(write!(f, " tpmsrc=0x{:x}", self.tpmsrc()))}
      if self.uart0src() != 0 { try!(write!(f, " uart0src=0x{:x}", self.uart0src()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Sopt4(pub u32);

impl Sopt4 {
  pub fn tpm1ch0src(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x3 // [19:18]
  }
  pub fn set_tpm1ch0src(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn tpm2ch0src(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_tpm2ch0src(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn tpm0clksel(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_tpm0clksel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn tpm1clksel(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_tpm1clksel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn tpm2clksel(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_tpm2clksel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

}

impl ::core::fmt::Display for Sopt4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sopt4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tpm1ch0src() != 0 { try!(write!(f, " tpm1ch0src=0x{:x}", self.tpm1ch0src()))}
      if self.tpm2ch0src() != 0 { try!(write!(f, " tpm2ch0src"))}
      if self.tpm0clksel() != 0 { try!(write!(f, " tpm0clksel"))}
      if self.tpm1clksel() != 0 { try!(write!(f, " tpm1clksel"))}
      if self.tpm2clksel() != 0 { try!(write!(f, " tpm2clksel"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Sopt5(pub u32);

impl Sopt5 {
  pub fn uart0txsrc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  pub fn set_uart0txsrc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn uart0rxsrc(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_uart0rxsrc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn uart1txsrc(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  pub fn set_uart1txsrc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn uart1rxsrc(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_uart1rxsrc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn uart0ode(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_uart0ode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn uart1ode(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_uart1ode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn uart2ode(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_uart2ode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

}

impl ::core::fmt::Display for Sopt5 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sopt5 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.uart0txsrc() != 0 { try!(write!(f, " uart0txsrc=0x{:x}", self.uart0txsrc()))}
      if self.uart0rxsrc() != 0 { try!(write!(f, " uart0rxsrc"))}
      if self.uart1txsrc() != 0 { try!(write!(f, " uart1txsrc=0x{:x}", self.uart1txsrc()))}
      if self.uart1rxsrc() != 0 { try!(write!(f, " uart1rxsrc"))}
      if self.uart0ode() != 0 { try!(write!(f, " uart0ode"))}
      if self.uart1ode() != 0 { try!(write!(f, " uart1ode"))}
      if self.uart2ode() != 0 { try!(write!(f, " uart2ode"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Sopt7(pub u32);

impl Sopt7 {
  pub fn adc0trgsel(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  pub fn set_adc0trgsel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  pub fn adc0pretrgsel(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_adc0pretrgsel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn adc0alttrgen(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_adc0alttrgen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for Sopt7 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sopt7 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.adc0trgsel() != 0 { try!(write!(f, " adc0trgsel=0x{:x}", self.adc0trgsel()))}
      if self.adc0pretrgsel() != 0 { try!(write!(f, " adc0pretrgsel"))}
      if self.adc0alttrgen() != 0 { try!(write!(f, " adc0alttrgen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Sdid(pub u32);

impl Sdid {
  pub fn pinid(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  pub fn set_pinid(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  pub fn dieid(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1f // [11:7]
  }
  pub fn set_dieid(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 7);
     self.0 |= value << 7;
     self
  }

  pub fn revid(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0xf // [15:12]
  }
  pub fn set_revid(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

  pub fn sramsize(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  pub fn set_sramsize(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  pub fn seriesid(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0xf // [23:20]
  }
  pub fn set_seriesid(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

  pub fn subfamid(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  pub fn set_subfamid(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

  pub fn famid(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0xf // [31:28]
  }
  pub fn set_famid(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 28);
     self.0 |= value << 28;
     self
  }

}

impl ::core::fmt::Display for Sdid {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sdid {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pinid() != 0 { try!(write!(f, " pinid=0x{:x}", self.pinid()))}
      if self.dieid() != 0 { try!(write!(f, " dieid=0x{:x}", self.dieid()))}
      if self.revid() != 0 { try!(write!(f, " revid=0x{:x}", self.revid()))}
      if self.sramsize() != 0 { try!(write!(f, " sramsize=0x{:x}", self.sramsize()))}
      if self.seriesid() != 0 { try!(write!(f, " seriesid=0x{:x}", self.seriesid()))}
      if self.subfamid() != 0 { try!(write!(f, " subfamid=0x{:x}", self.subfamid()))}
      if self.famid() != 0 { try!(write!(f, " famid=0x{:x}", self.famid()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Scgc4(pub u32);

impl Scgc4 {
  pub fn i2c0(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_i2c0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn i2c1(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_i2c1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn uart0(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_uart0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn uart1(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_uart1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn uart2(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_uart2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn usbotg(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_usbotg(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn cmp(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  pub fn set_cmp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  pub fn spi0(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_spi0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn spi1(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_spi1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

}

impl ::core::fmt::Display for Scgc4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Scgc4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.i2c0() != 0 { try!(write!(f, " i2c0"))}
      if self.i2c1() != 0 { try!(write!(f, " i2c1"))}
      if self.uart0() != 0 { try!(write!(f, " uart0"))}
      if self.uart1() != 0 { try!(write!(f, " uart1"))}
      if self.uart2() != 0 { try!(write!(f, " uart2"))}
      if self.usbotg() != 0 { try!(write!(f, " usbotg"))}
      if self.cmp() != 0 { try!(write!(f, " cmp"))}
      if self.spi0() != 0 { try!(write!(f, " spi0"))}
      if self.spi1() != 0 { try!(write!(f, " spi1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Scgc5(pub u32);

impl Scgc5 {
  pub fn lptmr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_lptmr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn tsi(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_tsi(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn porta(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_porta(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn portb(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_portb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn portc(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_portc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn portd(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_portd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn porte(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_porte(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

}

impl ::core::fmt::Display for Scgc5 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Scgc5 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lptmr() != 0 { try!(write!(f, " lptmr"))}
      if self.tsi() != 0 { try!(write!(f, " tsi"))}
      if self.porta() != 0 { try!(write!(f, " porta"))}
      if self.portb() != 0 { try!(write!(f, " portb"))}
      if self.portc() != 0 { try!(write!(f, " portc"))}
      if self.portd() != 0 { try!(write!(f, " portd"))}
      if self.porte() != 0 { try!(write!(f, " porte"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Scgc6(pub u32);

impl Scgc6 {
  pub fn ftf(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_ftf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn dmamux(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_dmamux(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn i2s(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_i2s(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn pit(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_pit(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn tpm0(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_tpm0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn tpm1(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_tpm1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn tpm2(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_tpm2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  pub fn adc0(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  pub fn set_adc0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  pub fn rtc(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  pub fn set_rtc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  pub fn dac0(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_dac0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}

impl ::core::fmt::Display for Scgc6 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Scgc6 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ftf() != 0 { try!(write!(f, " ftf"))}
      if self.dmamux() != 0 { try!(write!(f, " dmamux"))}
      if self.i2s() != 0 { try!(write!(f, " i2s"))}
      if self.pit() != 0 { try!(write!(f, " pit"))}
      if self.tpm0() != 0 { try!(write!(f, " tpm0"))}
      if self.tpm1() != 0 { try!(write!(f, " tpm1"))}
      if self.tpm2() != 0 { try!(write!(f, " tpm2"))}
      if self.adc0() != 0 { try!(write!(f, " adc0"))}
      if self.rtc() != 0 { try!(write!(f, " rtc"))}
      if self.dac0() != 0 { try!(write!(f, " dac0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Scgc7(pub u32);

impl Scgc7 {
  pub fn dma(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_dma(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

}

impl ::core::fmt::Display for Scgc7 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Scgc7 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dma() != 0 { try!(write!(f, " dma"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Clkdiv1(pub u32);

impl Clkdiv1 {
  pub fn outdiv4(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x7 // [18:16]
  }
  pub fn set_outdiv4(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn outdiv1(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0xf // [31:28]
  }
  pub fn set_outdiv1(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 28);
     self.0 |= value << 28;
     self
  }

}

impl ::core::fmt::Display for Clkdiv1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Clkdiv1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.outdiv4() != 0 { try!(write!(f, " outdiv4=0x{:x}", self.outdiv4()))}
      if self.outdiv1() != 0 { try!(write!(f, " outdiv1=0x{:x}", self.outdiv1()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Fcfg1(pub u32);

impl Fcfg1 {
  pub fn flashdis(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_flashdis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn flashdoze(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_flashdoze(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn pfsize(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  pub fn set_pfsize(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

}

impl ::core::fmt::Display for Fcfg1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Fcfg1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.flashdis() != 0 { try!(write!(f, " flashdis"))}
      if self.flashdoze() != 0 { try!(write!(f, " flashdoze"))}
      if self.pfsize() != 0 { try!(write!(f, " pfsize=0x{:x}", self.pfsize()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Fcfg2(pub u32);

impl Fcfg2 {
  pub fn maxaddr1(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x7f // [22:16]
  }
  pub fn set_maxaddr1(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 16);
     self.0 |= value << 16;
     self
  }

  pub fn maxaddr0(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7f // [30:24]
  }
  pub fn set_maxaddr0(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 24);
     self.0 |= value << 24;
     self
  }

}

impl ::core::fmt::Display for Fcfg2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Fcfg2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.maxaddr1() != 0 { try!(write!(f, " maxaddr1=0x{:x}", self.maxaddr1()))}
      if self.maxaddr0() != 0 { try!(write!(f, " maxaddr0=0x{:x}", self.maxaddr0()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Uidmh(pub u32);

impl Uidmh {
  pub fn uid(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_uid(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Uidmh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Uidmh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.uid() != 0 { try!(write!(f, " uid=0x{:x}", self.uid()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Uidml(pub u32);

impl Uidml {
  pub fn uid(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_uid(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Uidml {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Uidml {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Uidl(pub u32);

impl Uidl {
  pub fn uid(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_uid(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Uidl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Uidl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Copc(pub u32);

impl Copc {
  pub fn copw(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_copw(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn copclks(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_copclks(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn copt(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
  pub fn set_copt(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

}

impl ::core::fmt::Display for Copc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Copc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.copw() != 0 { try!(write!(f, " copw"))}
      if self.copclks() != 0 { try!(write!(f, " copclks"))}
      if self.copt() != 0 { try!(write!(f, " copt=0x{:x}", self.copt()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Srvcop(pub u32);

impl Srvcop {
  pub fn srvcop(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_srvcop(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Srvcop {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Srvcop {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.srvcop() != 0 { try!(write!(f, " srvcop=0x{:x}", self.srvcop()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

