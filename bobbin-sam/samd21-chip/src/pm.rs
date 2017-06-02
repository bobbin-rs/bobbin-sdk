pub const PM: Pm = Pm(0x40000400);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pm(pub u32);
impl Pm {
  #[inline]
  pub fn ahbmask_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
  #[inline]
  pub fn ahbmask_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
  #[inline]
  pub fn ahbmask(&self) -> Ahbmask { 
     unsafe {
       Ahbmask(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
  #[inline]
  pub fn set_ahbmask(&self, value: Ahbmask) -> &Pm {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ahbmask<F: FnOnce(Ahbmask) -> Ahbmask>(&self, f: F) -> &Pm {
     let tmp = self.ahbmask();
     self.set_ahbmask(f(tmp))
  }

  #[inline]
  pub fn apbamask_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
  #[inline]
  pub fn apbamask_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
  #[inline]
  pub fn apbamask(&self) -> Apbamask { 
     unsafe {
       Apbamask(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
  #[inline]
  pub fn set_apbamask(&self, value: Apbamask) -> &Pm {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_apbamask<F: FnOnce(Apbamask) -> Apbamask>(&self, f: F) -> &Pm {
     let tmp = self.apbamask();
     self.set_apbamask(f(tmp))
  }

  #[inline]
  pub fn apbasel_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x9) as *const u8
  }
  #[inline]
  pub fn apbasel_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x9) as *mut u8
  }
  #[inline]
  pub fn apbasel(&self) -> Apbasel { 
     unsafe {
       Apbasel(::core::ptr::read_volatile(((self.0 as usize) + 0x9) as *const u8))
     }
  }
  #[inline]
  pub fn set_apbasel(&self, value: Apbasel) -> &Pm {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x9) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_apbasel<F: FnOnce(Apbasel) -> Apbasel>(&self, f: F) -> &Pm {
     let tmp = self.apbasel();
     self.set_apbasel(f(tmp))
  }

  #[inline]
  pub fn apbbmask_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
  #[inline]
  pub fn apbbmask_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
  #[inline]
  pub fn apbbmask(&self) -> Apbbmask { 
     unsafe {
       Apbbmask(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
  #[inline]
  pub fn set_apbbmask(&self, value: Apbbmask) -> &Pm {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_apbbmask<F: FnOnce(Apbbmask) -> Apbbmask>(&self, f: F) -> &Pm {
     let tmp = self.apbbmask();
     self.set_apbbmask(f(tmp))
  }

  #[inline]
  pub fn apbbsel_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xa) as *const u8
  }
  #[inline]
  pub fn apbbsel_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xa) as *mut u8
  }
  #[inline]
  pub fn apbbsel(&self) -> Apbbsel { 
     unsafe {
       Apbbsel(::core::ptr::read_volatile(((self.0 as usize) + 0xa) as *const u8))
     }
  }
  #[inline]
  pub fn set_apbbsel(&self, value: Apbbsel) -> &Pm {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xa) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_apbbsel<F: FnOnce(Apbbsel) -> Apbbsel>(&self, f: F) -> &Pm {
     let tmp = self.apbbsel();
     self.set_apbbsel(f(tmp))
  }

  #[inline]
  pub fn apbcmask_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
  #[inline]
  pub fn apbcmask_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
  #[inline]
  pub fn apbcmask(&self) -> Apbcmask { 
     unsafe {
       Apbcmask(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
  #[inline]
  pub fn set_apbcmask(&self, value: Apbcmask) -> &Pm {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_apbcmask<F: FnOnce(Apbcmask) -> Apbcmask>(&self, f: F) -> &Pm {
     let tmp = self.apbcmask();
     self.set_apbcmask(f(tmp))
  }

  #[inline]
  pub fn apbcsel_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xb) as *const u8
  }
  #[inline]
  pub fn apbcsel_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xb) as *mut u8
  }
  #[inline]
  pub fn apbcsel(&self) -> Apbcsel { 
     unsafe {
       Apbcsel(::core::ptr::read_volatile(((self.0 as usize) + 0xb) as *const u8))
     }
  }
  #[inline]
  pub fn set_apbcsel(&self, value: Apbcsel) -> &Pm {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xb) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_apbcsel<F: FnOnce(Apbcsel) -> Apbcsel>(&self, f: F) -> &Pm {
     let tmp = self.apbcsel();
     self.set_apbcsel(f(tmp))
  }

  #[inline]
  pub fn cpusel_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x8) as *const u8
  }
  #[inline]
  pub fn cpusel_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x8) as *mut u8
  }
  #[inline]
  pub fn cpusel(&self) -> Cpusel { 
     unsafe {
       Cpusel(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u8))
     }
  }
  #[inline]
  pub fn set_cpusel(&self, value: Cpusel) -> &Pm {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_cpusel<F: FnOnce(Cpusel) -> Cpusel>(&self, f: F) -> &Pm {
     let tmp = self.cpusel();
     self.set_cpusel(f(tmp))
  }

  #[inline]
  pub fn ctrl_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x0) as *const u8
  }
  #[inline]
  pub fn ctrl_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x0) as *mut u8
  }
  #[inline]
  pub fn ctrl(&self) -> Ctrl { 
     unsafe {
       Ctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u8))
     }
  }
  #[inline]
  pub fn set_ctrl(&self, value: Ctrl) -> &Pm {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Pm {
     let tmp = self.ctrl();
     self.set_ctrl(f(tmp))
  }

  #[inline]
  pub fn intenclr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x34) as *const u8
  }
  #[inline]
  pub fn intenclr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x34) as *mut u8
  }
  #[inline]
  pub fn intenclr(&self) -> Intenclr { 
     unsafe {
       Intenclr(::core::ptr::read_volatile(((self.0 as usize) + 0x34) as *const u8))
     }
  }
  #[inline]
  pub fn set_intenclr(&self, value: Intenclr) -> &Pm {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x34) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Pm {
     let tmp = self.intenclr();
     self.set_intenclr(f(tmp))
  }

  #[inline]
  pub fn intenset_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x35) as *const u8
  }
  #[inline]
  pub fn intenset_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x35) as *mut u8
  }
  #[inline]
  pub fn intenset(&self) -> Intenset { 
     unsafe {
       Intenset(::core::ptr::read_volatile(((self.0 as usize) + 0x35) as *const u8))
     }
  }
  #[inline]
  pub fn set_intenset(&self, value: Intenset) -> &Pm {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x35) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Pm {
     let tmp = self.intenset();
     self.set_intenset(f(tmp))
  }

  #[inline]
  pub fn intflag_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x36) as *const u8
  }
  #[inline]
  pub fn intflag_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x36) as *mut u8
  }
  #[inline]
  pub fn intflag(&self) -> Intflag { 
     unsafe {
       Intflag(::core::ptr::read_volatile(((self.0 as usize) + 0x36) as *const u8))
     }
  }
  #[inline]
  pub fn set_intflag(&self, value: Intflag) -> &Pm {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x36) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Pm {
     let tmp = self.intflag();
     self.set_intflag(f(tmp))
  }

  #[inline]
  pub fn rcause_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x38) as *const u8
  }
  #[inline]
  pub fn rcause_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x38) as *mut u8
  }
  #[inline]
  pub fn rcause(&self) -> Rcause { 
     unsafe {
       Rcause(::core::ptr::read_volatile(((self.0 as usize) + 0x38) as *const u8))
     }
  }

  #[inline]
  pub fn sleep_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x1) as *const u8
  }
  #[inline]
  pub fn sleep_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x1) as *mut u8
  }
  #[inline]
  pub fn sleep(&self) -> Sleep { 
     unsafe {
       Sleep(::core::ptr::read_volatile(((self.0 as usize) + 0x1) as *const u8))
     }
  }
  #[inline]
  pub fn set_sleep(&self, value: Sleep) -> &Pm {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x1) as *mut u8, value.0);
     }
     self
  }
  #[inline]
  pub fn with_sleep<F: FnOnce(Sleep) -> Sleep>(&self, f: F) -> &Pm {
     let tmp = self.sleep();
     self.set_sleep(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Ahbmask(pub u32);
impl Ahbmask {
  #[inline]
  pub fn hpb0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_hpb0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn hpb1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_hpb1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn hpb2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_hpb2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn dsu(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_dsu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn nvmctrl(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_nvmctrl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn dmac(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_dmac(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn usb(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_usb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

}
impl ::core::fmt::Display for Ahbmask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ahbmask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.hpb0() != 0 { try!(write!(f, " hpb0"))}
      if self.hpb1() != 0 { try!(write!(f, " hpb1"))}
      if self.hpb2() != 0 { try!(write!(f, " hpb2"))}
      if self.dsu() != 0 { try!(write!(f, " dsu"))}
      if self.nvmctrl() != 0 { try!(write!(f, " nvmctrl"))}
      if self.dmac() != 0 { try!(write!(f, " dmac"))}
      if self.usb() != 0 { try!(write!(f, " usb"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Apbamask(pub u32);
impl Apbamask {
  #[inline]
  pub fn pac0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_pac0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn pm(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_pm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn sysctrl(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_sysctrl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn gclk(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_gclk(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn wdt(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_wdt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn rtc(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_rtc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn eic(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_eic(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

}
impl ::core::fmt::Display for Apbamask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Apbamask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pac0() != 0 { try!(write!(f, " pac0"))}
      if self.pm() != 0 { try!(write!(f, " pm"))}
      if self.sysctrl() != 0 { try!(write!(f, " sysctrl"))}
      if self.gclk() != 0 { try!(write!(f, " gclk"))}
      if self.wdt() != 0 { try!(write!(f, " wdt"))}
      if self.rtc() != 0 { try!(write!(f, " rtc"))}
      if self.eic() != 0 { try!(write!(f, " eic"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Apbasel(pub u8);
impl Apbasel {
  #[inline]
  pub fn apbadiv(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x7 // [2:0]
  }
  #[inline]
  pub fn set_apbadiv(mut self, value: u8) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Apbasel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Apbasel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.apbadiv() != 0 { try!(write!(f, " apbadiv=0x{:x}", self.apbadiv()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Apbbmask(pub u32);
impl Apbbmask {
  #[inline]
  pub fn pac1(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_pac1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn dsu(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_dsu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn nvmctrl(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_nvmctrl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn port(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_port(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn dmac(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_dmac(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn usb(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_usb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for Apbbmask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Apbbmask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pac1() != 0 { try!(write!(f, " pac1"))}
      if self.dsu() != 0 { try!(write!(f, " dsu"))}
      if self.nvmctrl() != 0 { try!(write!(f, " nvmctrl"))}
      if self.port() != 0 { try!(write!(f, " port"))}
      if self.dmac() != 0 { try!(write!(f, " dmac"))}
      if self.usb() != 0 { try!(write!(f, " usb"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Apbbsel(pub u8);
impl Apbbsel {
  #[inline]
  pub fn apbbdiv(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x7 // [2:0]
  }
  #[inline]
  pub fn set_apbbdiv(mut self, value: u8) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Apbbsel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Apbbsel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.apbbdiv() != 0 { try!(write!(f, " apbbdiv=0x{:x}", self.apbbdiv()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Apbcmask(pub u32);
impl Apbcmask {
  #[inline]
  pub fn pac2(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_pac2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn evsys(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_evsys(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn sercom0(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_sercom0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn sercom1(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_sercom1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn sercom2(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_sercom2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn sercom3(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_sercom3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn sercom4(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_sercom4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn sercom5(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_sercom5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn tcc0(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_tcc0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn tcc1(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_tcc1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn tcc2(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_tcc2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn tc3(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_tc3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn tc4(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline]
  pub fn set_tc4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn tc5(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_tc5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline]
  pub fn adc(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
  pub fn set_adc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn ac(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline]
  pub fn set_ac(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline]
  pub fn dac(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline]
  pub fn set_dac(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  #[inline]
  pub fn i2s(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  #[inline]
  pub fn set_i2s(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  #[inline]
  pub fn atw(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  #[inline]
  pub fn set_atw(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

}
impl ::core::fmt::Display for Apbcmask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Apbcmask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pac2() != 0 { try!(write!(f, " pac2"))}
      if self.evsys() != 0 { try!(write!(f, " evsys"))}
      if self.sercom0() != 0 { try!(write!(f, " sercom0"))}
      if self.sercom1() != 0 { try!(write!(f, " sercom1"))}
      if self.sercom2() != 0 { try!(write!(f, " sercom2"))}
      if self.sercom3() != 0 { try!(write!(f, " sercom3"))}
      if self.sercom4() != 0 { try!(write!(f, " sercom4"))}
      if self.sercom5() != 0 { try!(write!(f, " sercom5"))}
      if self.tcc0() != 0 { try!(write!(f, " tcc0"))}
      if self.tcc1() != 0 { try!(write!(f, " tcc1"))}
      if self.tcc2() != 0 { try!(write!(f, " tcc2"))}
      if self.tc3() != 0 { try!(write!(f, " tc3"))}
      if self.tc4() != 0 { try!(write!(f, " tc4"))}
      if self.tc5() != 0 { try!(write!(f, " tc5"))}
      if self.adc() != 0 { try!(write!(f, " adc"))}
      if self.ac() != 0 { try!(write!(f, " ac"))}
      if self.dac() != 0 { try!(write!(f, " dac"))}
      if self.i2s() != 0 { try!(write!(f, " i2s"))}
      if self.atw() != 0 { try!(write!(f, " atw"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Apbcsel(pub u8);
impl Apbcsel {
  #[inline]
  pub fn apbcdiv(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x7 // [2:0]
  }
  #[inline]
  pub fn set_apbcdiv(mut self, value: u8) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Apbcsel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Apbcsel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.apbcdiv() != 0 { try!(write!(f, " apbcdiv=0x{:x}", self.apbcdiv()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cpusel(pub u8);
impl Cpusel {
  #[inline]
  pub fn cpudiv(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x7 // [2:0]
  }
  #[inline]
  pub fn set_cpudiv(mut self, value: u8) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cpusel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cpusel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cpudiv() != 0 { try!(write!(f, " cpudiv=0x{:x}", self.cpudiv()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ctrl(pub u8);
impl Ctrl {
  #[inline]
  pub fn cfden(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_cfden(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn bkupclk(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_bkupclk(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
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
      if self.cfden() != 0 { try!(write!(f, " cfden"))}
      if self.bkupclk() != 0 { try!(write!(f, " bkupclk"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
  #[inline]
  pub fn ckrdy(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_ckrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn cfd(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_cfd(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

}
impl ::core::fmt::Display for Intenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ckrdy() != 0 { try!(write!(f, " ckrdy"))}
      if self.cfd() != 0 { try!(write!(f, " cfd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
  #[inline]
  pub fn ckrdy(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_ckrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn cfd(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_cfd(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

}
impl ::core::fmt::Display for Intenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ckrdy() != 0 { try!(write!(f, " ckrdy"))}
      if self.cfd() != 0 { try!(write!(f, " cfd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
  #[inline]
  pub fn ckrdy(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_ckrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn cfd(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_cfd(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

}
impl ::core::fmt::Display for Intflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ckrdy() != 0 { try!(write!(f, " ckrdy"))}
      if self.cfd() != 0 { try!(write!(f, " cfd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Rcause(pub u8);
impl Rcause {
  #[inline]
  pub fn por(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_por(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn bod12(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_bod12(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn bod33(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_bod33(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn ext(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_ext(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn wdt(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_wdt(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn syst(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_syst(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

}
impl ::core::fmt::Display for Rcause {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rcause {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.por() != 0 { try!(write!(f, " por"))}
      if self.bod12() != 0 { try!(write!(f, " bod12"))}
      if self.bod33() != 0 { try!(write!(f, " bod33"))}
      if self.ext() != 0 { try!(write!(f, " ext"))}
      if self.wdt() != 0 { try!(write!(f, " wdt"))}
      if self.syst() != 0 { try!(write!(f, " syst"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Sleep(pub u8);
impl Sleep {
  #[inline]
  pub fn idle(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x3 // [1:0]
  }
  #[inline]
  pub fn set_idle(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Sleep {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sleep {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.idle() != 0 { try!(write!(f, " idle=0x{:x}", self.idle()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
pub trait En {
   fn en(&self) -> u32;
   fn set_en(&self, value: u32);
}

impl Pm {
   #[inline]
   pub fn en<P: En>(&self, p: &P) -> u32 {
      p.en()
   }
   #[inline]
   pub fn set_en<P: En>(&self, p: &P, value: u32) {
      p.set_en(value)
   }
}

impl En for super::pm::Pm {
   #[inline]
   fn en(&self) -> u32 { PM.apbamask().pm() }
   #[inline]
   fn set_en(&self, value: u32) { PM.with_apbamask(|r| r.set_pm(value)); }
}

impl En for super::sysctrl::Sysctrl {
   #[inline]
   fn en(&self) -> u32 { PM.apbamask().sysctrl() }
   #[inline]
   fn set_en(&self, value: u32) { PM.with_apbamask(|r| r.set_sysctrl(value)); }
}

impl En for super::gclk::Gclk {
   #[inline]
   fn en(&self) -> u32 { PM.apbamask().gclk() }
   #[inline]
   fn set_en(&self, value: u32) { PM.with_apbamask(|r| r.set_gclk(value)); }
}

impl En for super::rtc::Rtc {
   #[inline]
   fn en(&self) -> u32 { PM.apbamask().rtc() }
   #[inline]
   fn set_en(&self, value: u32) { PM.with_apbamask(|r| r.set_rtc(value)); }
}

impl En for super::port::Porta {
   #[inline]
   fn en(&self) -> u32 { PM.apbbmask().port() }
   #[inline]
   fn set_en(&self, value: u32) { PM.with_apbbmask(|r| r.set_port(value)); }
}

impl En for super::port::Portb {
   #[inline]
   fn en(&self) -> u32 { PM.apbbmask().port() }
   #[inline]
   fn set_en(&self, value: u32) { PM.with_apbbmask(|r| r.set_port(value)); }
}

impl En for super::sercom::Sercom0 {
   #[inline]
   fn en(&self) -> u32 { PM.apbcmask().sercom0() }
   #[inline]
   fn set_en(&self, value: u32) { PM.with_apbcmask(|r| r.set_sercom0(value)); }
}

impl En for super::sercom::Sercom1 {
   #[inline]
   fn en(&self) -> u32 { PM.apbcmask().sercom1() }
   #[inline]
   fn set_en(&self, value: u32) { PM.with_apbcmask(|r| r.set_sercom1(value)); }
}

impl En for super::sercom::Sercom2 {
   #[inline]
   fn en(&self) -> u32 { PM.apbcmask().sercom2() }
   #[inline]
   fn set_en(&self, value: u32) { PM.with_apbcmask(|r| r.set_sercom2(value)); }
}

impl En for super::sercom::Sercom3 {
   #[inline]
   fn en(&self) -> u32 { PM.apbcmask().sercom3() }
   #[inline]
   fn set_en(&self, value: u32) { PM.with_apbcmask(|r| r.set_sercom3(value)); }
}

impl En for super::sercom::Sercom4 {
   #[inline]
   fn en(&self) -> u32 { PM.apbcmask().sercom4() }
   #[inline]
   fn set_en(&self, value: u32) { PM.with_apbcmask(|r| r.set_sercom4(value)); }
}

impl En for super::sercom::Sercom5 {
   #[inline]
   fn en(&self) -> u32 { PM.apbcmask().sercom5() }
   #[inline]
   fn set_en(&self, value: u32) { PM.with_apbcmask(|r| r.set_sercom5(value)); }
}

impl En for super::tcc::Tcc0 {
   #[inline]
   fn en(&self) -> u32 { PM.apbcmask().tcc0() }
   #[inline]
   fn set_en(&self, value: u32) { PM.with_apbcmask(|r| r.set_tcc0(value)); }
}

impl En for super::tcc::Tcc1 {
   #[inline]
   fn en(&self) -> u32 { PM.apbcmask().tcc1() }
   #[inline]
   fn set_en(&self, value: u32) { PM.with_apbcmask(|r| r.set_tcc1(value)); }
}

impl En for super::tcc::Tcc2 {
   #[inline]
   fn en(&self) -> u32 { PM.apbcmask().tcc2() }
   #[inline]
   fn set_en(&self, value: u32) { PM.with_apbcmask(|r| r.set_tcc2(value)); }
}

impl En for super::tc::Tc3 {
   #[inline]
   fn en(&self) -> u32 { PM.apbcmask().tc3() }
   #[inline]
   fn set_en(&self, value: u32) { PM.with_apbcmask(|r| r.set_tc3(value)); }
}

impl En for super::tc::Tc4 {
   #[inline]
   fn en(&self) -> u32 { PM.apbcmask().tc4() }
   #[inline]
   fn set_en(&self, value: u32) { PM.with_apbcmask(|r| r.set_tc4(value)); }
}

impl En for super::tc::Tc5 {
   #[inline]
   fn en(&self) -> u32 { PM.apbcmask().tc5() }
   #[inline]
   fn set_en(&self, value: u32) { PM.with_apbcmask(|r| r.set_tc5(value)); }
}

impl En for super::adc::Adc {
   #[inline]
   fn en(&self) -> u32 { PM.apbcmask().adc() }
   #[inline]
   fn set_en(&self, value: u32) { PM.with_apbcmask(|r| r.set_adc(value)); }
}

