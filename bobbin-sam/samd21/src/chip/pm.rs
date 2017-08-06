//! Power Manager
#[allow(unused_imports)] use bobbin_common::bits;
pub const PM: Pm = Pm(0x40000400);

#[doc="Power Manager"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pm(pub u32);
impl Pm {
#[doc="Get the *const pointer for the AHBMASK register."]
  #[inline] pub fn ahbmask_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the AHBMASK register."]
  #[inline] pub fn ahbmask_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Read the AHBMASK register."]
  #[inline] pub fn ahbmask(&self) -> Ahbmask { 
     unsafe {
        Ahbmask(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
#[doc="Write the AHBMASK register."]
  #[inline] pub fn set_ahbmask(&self, value: Ahbmask) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the AHBMASK register."]
  #[inline] pub fn with_ahbmask<F: FnOnce(Ahbmask) -> Ahbmask>(&self, f: F) -> &Self {
     let tmp = self.ahbmask();
     self.set_ahbmask(f(tmp))
  }

#[doc="Get the *const pointer for the APBAMASK register."]
  #[inline] pub fn apbamask_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
#[doc="Get the *mut pointer for the APBAMASK register."]
  #[inline] pub fn apbamask_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
#[doc="Read the APBAMASK register."]
  #[inline] pub fn apbamask(&self) -> Apbamask { 
     unsafe {
        Apbamask(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
#[doc="Write the APBAMASK register."]
  #[inline] pub fn set_apbamask(&self, value: Apbamask) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the APBAMASK register."]
  #[inline] pub fn with_apbamask<F: FnOnce(Apbamask) -> Apbamask>(&self, f: F) -> &Self {
     let tmp = self.apbamask();
     self.set_apbamask(f(tmp))
  }

#[doc="Get the *const pointer for the APBASEL register."]
  #[inline] pub fn apbasel_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x9) as *const u8
  }
#[doc="Get the *mut pointer for the APBASEL register."]
  #[inline] pub fn apbasel_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x9) as *mut u8
  }
#[doc="Read the APBASEL register."]
  #[inline] pub fn apbasel(&self) -> Apbasel { 
     unsafe {
        Apbasel(::core::ptr::read_volatile(((self.0 as usize) + 0x9) as *const u8))
     }
  }
#[doc="Write the APBASEL register."]
  #[inline] pub fn set_apbasel(&self, value: Apbasel) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x9) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the APBASEL register."]
  #[inline] pub fn with_apbasel<F: FnOnce(Apbasel) -> Apbasel>(&self, f: F) -> &Self {
     let tmp = self.apbasel();
     self.set_apbasel(f(tmp))
  }

#[doc="Get the *const pointer for the APBBMASK register."]
  #[inline] pub fn apbbmask_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the APBBMASK register."]
  #[inline] pub fn apbbmask_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Read the APBBMASK register."]
  #[inline] pub fn apbbmask(&self) -> Apbbmask { 
     unsafe {
        Apbbmask(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
#[doc="Write the APBBMASK register."]
  #[inline] pub fn set_apbbmask(&self, value: Apbbmask) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the APBBMASK register."]
  #[inline] pub fn with_apbbmask<F: FnOnce(Apbbmask) -> Apbbmask>(&self, f: F) -> &Self {
     let tmp = self.apbbmask();
     self.set_apbbmask(f(tmp))
  }

#[doc="Get the *const pointer for the APBBSEL register."]
  #[inline] pub fn apbbsel_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xa) as *const u8
  }
#[doc="Get the *mut pointer for the APBBSEL register."]
  #[inline] pub fn apbbsel_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xa) as *mut u8
  }
#[doc="Read the APBBSEL register."]
  #[inline] pub fn apbbsel(&self) -> Apbbsel { 
     unsafe {
        Apbbsel(::core::ptr::read_volatile(((self.0 as usize) + 0xa) as *const u8))
     }
  }
#[doc="Write the APBBSEL register."]
  #[inline] pub fn set_apbbsel(&self, value: Apbbsel) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the APBBSEL register."]
  #[inline] pub fn with_apbbsel<F: FnOnce(Apbbsel) -> Apbbsel>(&self, f: F) -> &Self {
     let tmp = self.apbbsel();
     self.set_apbbsel(f(tmp))
  }

#[doc="Get the *const pointer for the APBCMASK register."]
  #[inline] pub fn apbcmask_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
#[doc="Get the *mut pointer for the APBCMASK register."]
  #[inline] pub fn apbcmask_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
#[doc="Read the APBCMASK register."]
  #[inline] pub fn apbcmask(&self) -> Apbcmask { 
     unsafe {
        Apbcmask(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
#[doc="Write the APBCMASK register."]
  #[inline] pub fn set_apbcmask(&self, value: Apbcmask) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the APBCMASK register."]
  #[inline] pub fn with_apbcmask<F: FnOnce(Apbcmask) -> Apbcmask>(&self, f: F) -> &Self {
     let tmp = self.apbcmask();
     self.set_apbcmask(f(tmp))
  }

#[doc="Get the *const pointer for the APBCSEL register."]
  #[inline] pub fn apbcsel_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xb) as *const u8
  }
#[doc="Get the *mut pointer for the APBCSEL register."]
  #[inline] pub fn apbcsel_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xb) as *mut u8
  }
#[doc="Read the APBCSEL register."]
  #[inline] pub fn apbcsel(&self) -> Apbcsel { 
     unsafe {
        Apbcsel(::core::ptr::read_volatile(((self.0 as usize) + 0xb) as *const u8))
     }
  }
#[doc="Write the APBCSEL register."]
  #[inline] pub fn set_apbcsel(&self, value: Apbcsel) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xb) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the APBCSEL register."]
  #[inline] pub fn with_apbcsel<F: FnOnce(Apbcsel) -> Apbcsel>(&self, f: F) -> &Self {
     let tmp = self.apbcsel();
     self.set_apbcsel(f(tmp))
  }

#[doc="Get the *const pointer for the CPUSEL register."]
  #[inline] pub fn cpusel_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x8) as *const u8
  }
#[doc="Get the *mut pointer for the CPUSEL register."]
  #[inline] pub fn cpusel_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x8) as *mut u8
  }
#[doc="Read the CPUSEL register."]
  #[inline] pub fn cpusel(&self) -> Cpusel { 
     unsafe {
        Cpusel(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u8))
     }
  }
#[doc="Write the CPUSEL register."]
  #[inline] pub fn set_cpusel(&self, value: Cpusel) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the CPUSEL register."]
  #[inline] pub fn with_cpusel<F: FnOnce(Cpusel) -> Cpusel>(&self, f: F) -> &Self {
     let tmp = self.cpusel();
     self.set_cpusel(f(tmp))
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

#[doc="Get the *const pointer for the INTENCLR register."]
  #[inline] pub fn intenclr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x34) as *const u8
  }
#[doc="Get the *mut pointer for the INTENCLR register."]
  #[inline] pub fn intenclr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x34) as *mut u8
  }
#[doc="Read the INTENCLR register."]
  #[inline] pub fn intenclr(&self) -> Intenclr { 
     unsafe {
        Intenclr(::core::ptr::read_volatile(((self.0 as usize) + 0x34) as *const u8))
     }
  }
#[doc="Write the INTENCLR register."]
  #[inline] pub fn set_intenclr(&self, value: Intenclr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x34) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTENCLR register."]
  #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
     let tmp = self.intenclr();
     self.set_intenclr(f(tmp))
  }

#[doc="Get the *const pointer for the INTENSET register."]
  #[inline] pub fn intenset_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x35) as *const u8
  }
#[doc="Get the *mut pointer for the INTENSET register."]
  #[inline] pub fn intenset_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x35) as *mut u8
  }
#[doc="Read the INTENSET register."]
  #[inline] pub fn intenset(&self) -> Intenset { 
     unsafe {
        Intenset(::core::ptr::read_volatile(((self.0 as usize) + 0x35) as *const u8))
     }
  }
#[doc="Write the INTENSET register."]
  #[inline] pub fn set_intenset(&self, value: Intenset) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x35) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTENSET register."]
  #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
     let tmp = self.intenset();
     self.set_intenset(f(tmp))
  }

#[doc="Get the *const pointer for the INTFLAG register."]
  #[inline] pub fn intflag_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x36) as *const u8
  }
#[doc="Get the *mut pointer for the INTFLAG register."]
  #[inline] pub fn intflag_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x36) as *mut u8
  }
#[doc="Read the INTFLAG register."]
  #[inline] pub fn intflag(&self) -> Intflag { 
     unsafe {
        Intflag(::core::ptr::read_volatile(((self.0 as usize) + 0x36) as *const u8))
     }
  }
#[doc="Write the INTFLAG register."]
  #[inline] pub fn set_intflag(&self, value: Intflag) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x36) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTFLAG register."]
  #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
     let tmp = self.intflag();
     self.set_intflag(f(tmp))
  }

#[doc="Get the *const pointer for the RCAUSE register."]
  #[inline] pub fn rcause_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x38) as *const u8
  }
#[doc="Get the *mut pointer for the RCAUSE register."]
  #[inline] pub fn rcause_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x38) as *mut u8
  }
#[doc="Read the RCAUSE register."]
  #[inline] pub fn rcause(&self) -> Rcause { 
     unsafe {
        Rcause(::core::ptr::read_volatile(((self.0 as usize) + 0x38) as *const u8))
     }
  }

#[doc="Get the *const pointer for the SLEEP register."]
  #[inline] pub fn sleep_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x1) as *const u8
  }
#[doc="Get the *mut pointer for the SLEEP register."]
  #[inline] pub fn sleep_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x1) as *mut u8
  }
#[doc="Read the SLEEP register."]
  #[inline] pub fn sleep(&self) -> Sleep { 
     unsafe {
        Sleep(::core::ptr::read_volatile(((self.0 as usize) + 0x1) as *const u8))
     }
  }
#[doc="Write the SLEEP register."]
  #[inline] pub fn set_sleep(&self, value: Sleep) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the SLEEP register."]
  #[inline] pub fn with_sleep<F: FnOnce(Sleep) -> Sleep>(&self, f: F) -> &Self {
     let tmp = self.sleep();
     self.set_sleep(f(tmp))
  }

}

#[doc="AHB Mask"]
#[derive(PartialEq, Eq)]
pub struct Ahbmask(pub u32);
impl Ahbmask {
#[doc="HPB0 AHB Clock Enable"]
  #[inline] pub fn hpb0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="HPB0 AHB Clock Enable"]
  #[inline] pub fn set_hpb0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="HPB1 AHB Clock Enable"]
  #[inline] pub fn hpb1(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="HPB1 AHB Clock Enable"]
  #[inline] pub fn set_hpb1<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="HPB2 AHB Clock Enable"]
  #[inline] pub fn hpb2(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="HPB2 AHB Clock Enable"]
  #[inline] pub fn set_hpb2<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="DSU AHB Clock Enable"]
  #[inline] pub fn dsu(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="DSU AHB Clock Enable"]
  #[inline] pub fn set_dsu<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="NVMCTRL AHB Clock Enable"]
  #[inline] pub fn nvmctrl(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="NVMCTRL AHB Clock Enable"]
  #[inline] pub fn set_nvmctrl<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="DMAC AHB Clock Enable"]
  #[inline] pub fn dmac(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="DMAC AHB Clock Enable"]
  #[inline] pub fn set_dmac<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="USB AHB Clock Enable"]
  #[inline] pub fn usb(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="USB AHB Clock Enable"]
  #[inline] pub fn set_usb<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
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
#[doc="APBA Mask"]
#[derive(PartialEq, Eq)]
pub struct Apbamask(pub u32);
impl Apbamask {
#[doc="PAC0 APB Clock Enable"]
  #[inline] pub fn pac0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="PAC0 APB Clock Enable"]
  #[inline] pub fn set_pac0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="PM APB Clock Enable"]
  #[inline] pub fn pm(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="PM APB Clock Enable"]
  #[inline] pub fn set_pm<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="SYSCTRL APB Clock Enable"]
  #[inline] pub fn sysctrl(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="SYSCTRL APB Clock Enable"]
  #[inline] pub fn set_sysctrl<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="GCLK APB Clock Enable"]
  #[inline] pub fn gclk(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="GCLK APB Clock Enable"]
  #[inline] pub fn set_gclk<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="WDT APB Clock Enable"]
  #[inline] pub fn wdt(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="WDT APB Clock Enable"]
  #[inline] pub fn set_wdt<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="RTC APB Clock Enable"]
  #[inline] pub fn rtc(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="RTC APB Clock Enable"]
  #[inline] pub fn set_rtc<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="EIC APB Clock Enable"]
  #[inline] pub fn eic(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="EIC APB Clock Enable"]
  #[inline] pub fn set_eic<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
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
#[doc="APBA Clock Select"]
#[derive(PartialEq, Eq)]
pub struct Apbasel(pub u8);
impl Apbasel {
#[doc="APBA Prescaler Selection"]
  #[inline] pub fn apbadiv(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
  }
#[doc="APBA Prescaler Selection"]
  #[inline] pub fn set_apbadiv<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u8 = value.into();
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
#[doc="APBB Mask"]
#[derive(PartialEq, Eq)]
pub struct Apbbmask(pub u32);
impl Apbbmask {
#[doc="PAC1 APB Clock Enable"]
  #[inline] pub fn pac1(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="PAC1 APB Clock Enable"]
  #[inline] pub fn set_pac1<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="DSU APB Clock Enable"]
  #[inline] pub fn dsu(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="DSU APB Clock Enable"]
  #[inline] pub fn set_dsu<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="NVMCTRL APB Clock Enable"]
  #[inline] pub fn nvmctrl(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="NVMCTRL APB Clock Enable"]
  #[inline] pub fn set_nvmctrl<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="PORT APB Clock Enable"]
  #[inline] pub fn port(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="PORT APB Clock Enable"]
  #[inline] pub fn set_port<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="DMAC APB Clock Enable"]
  #[inline] pub fn dmac(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="DMAC APB Clock Enable"]
  #[inline] pub fn set_dmac<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="USB APB Clock Enable"]
  #[inline] pub fn usb(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="USB APB Clock Enable"]
  #[inline] pub fn set_usb<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
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
#[doc="APBB Clock Select"]
#[derive(PartialEq, Eq)]
pub struct Apbbsel(pub u8);
impl Apbbsel {
#[doc="APBB Prescaler Selection"]
  #[inline] pub fn apbbdiv(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
  }
#[doc="APBB Prescaler Selection"]
  #[inline] pub fn set_apbbdiv<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u8 = value.into();
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
#[doc="APBC Mask"]
#[derive(PartialEq, Eq)]
pub struct Apbcmask(pub u32);
impl Apbcmask {
#[doc="PAC2 APB Clock Enable"]
  #[inline] pub fn pac2(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="PAC2 APB Clock Enable"]
  #[inline] pub fn set_pac2<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="EVSYS APB Clock Enable"]
  #[inline] pub fn evsys(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="EVSYS APB Clock Enable"]
  #[inline] pub fn set_evsys<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="SERCOM0 APB Clock Enable"]
  #[inline] pub fn sercom0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="SERCOM0 APB Clock Enable"]
  #[inline] pub fn set_sercom0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="SERCOM1 APB Clock Enable"]
  #[inline] pub fn sercom1(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="SERCOM1 APB Clock Enable"]
  #[inline] pub fn set_sercom1<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="SERCOM2 APB Clock Enable"]
  #[inline] pub fn sercom2(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="SERCOM2 APB Clock Enable"]
  #[inline] pub fn set_sercom2<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="SERCOM3 APB Clock Enable"]
  #[inline] pub fn sercom3(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="SERCOM3 APB Clock Enable"]
  #[inline] pub fn set_sercom3<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="SERCOM4 APB Clock Enable"]
  #[inline] pub fn sercom4(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="SERCOM4 APB Clock Enable"]
  #[inline] pub fn set_sercom4<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="SERCOM5 APB Clock Enable"]
  #[inline] pub fn sercom5(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="SERCOM5 APB Clock Enable"]
  #[inline] pub fn set_sercom5<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="TCC0 APB Clock Enable"]
  #[inline] pub fn tcc0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
  }
#[doc="TCC0 APB Clock Enable"]
  #[inline] pub fn set_tcc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="TCC1 APB Clock Enable"]
  #[inline] pub fn tcc1(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
  }
#[doc="TCC1 APB Clock Enable"]
  #[inline] pub fn set_tcc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="TCC2 APB Clock Enable"]
  #[inline] pub fn tcc2(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
  }
#[doc="TCC2 APB Clock Enable"]
  #[inline] pub fn set_tcc2<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="TC3 APB Clock Enable"]
  #[inline] pub fn tc3(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
  }
#[doc="TC3 APB Clock Enable"]
  #[inline] pub fn set_tc3<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="TC4 APB Clock Enable"]
  #[inline] pub fn tc4(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
  }
#[doc="TC4 APB Clock Enable"]
  #[inline] pub fn set_tc4<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="TC5 APB Clock Enable"]
  #[inline] pub fn tc5(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
  }
#[doc="TC5 APB Clock Enable"]
  #[inline] pub fn set_tc5<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="ADC APB Clock Enable"]
  #[inline] pub fn adc(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
  }
#[doc="ADC APB Clock Enable"]
  #[inline] pub fn set_adc<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="AC APB Clock Enable"]
  #[inline] pub fn ac(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
  }
#[doc="AC APB Clock Enable"]
  #[inline] pub fn set_ac<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="DAC APB Clock Enable"]
  #[inline] pub fn dac(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
  }
#[doc="DAC APB Clock Enable"]
  #[inline] pub fn set_dac<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="I2S APB Clock Enable"]
  #[inline] pub fn i2s(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
  }
#[doc="I2S APB Clock Enable"]
  #[inline] pub fn set_i2s<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="ATW APB Clock Enable"]
  #[inline] pub fn atw(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
  }
#[doc="ATW APB Clock Enable"]
  #[inline] pub fn set_atw<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
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
#[doc="APBC Clock Select"]
#[derive(PartialEq, Eq)]
pub struct Apbcsel(pub u8);
impl Apbcsel {
#[doc="APBC Prescaler Selection"]
  #[inline] pub fn apbcdiv(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
  }
#[doc="APBC Prescaler Selection"]
  #[inline] pub fn set_apbcdiv<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u8 = value.into();
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
#[doc="CPU Clock Select"]
#[derive(PartialEq, Eq)]
pub struct Cpusel(pub u8);
impl Cpusel {
#[doc="CPU Prescaler Selection"]
  #[inline] pub fn cpudiv(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
  }
#[doc="CPU Prescaler Selection"]
  #[inline] pub fn set_cpudiv<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u8 = value.into();
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
#[doc="Control"]
#[derive(PartialEq, Eq)]
pub struct Ctrl(pub u8);
impl Ctrl {
#[doc="Clock Failure Detector Enable"]
  #[inline] pub fn cfden(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="Clock Failure Detector Enable"]
  #[inline] pub fn set_cfden<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Backup Clock Select"]
  #[inline] pub fn bkupclk(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="Backup Clock Select"]
  #[inline] pub fn set_bkupclk<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
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
#[doc="Interrupt Enable Clear"]
#[derive(PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
#[doc="Clock Ready Interrupt Enable"]
  #[inline] pub fn ckrdy(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Clock Ready Interrupt Enable"]
  #[inline] pub fn set_ckrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Clock Failure Detector Interrupt Enable"]
  #[inline] pub fn cfd(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Clock Failure Detector Interrupt Enable"]
  #[inline] pub fn set_cfd<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
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
#[doc="Interrupt Enable Set"]
#[derive(PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
#[doc="Clock Ready Interrupt Enable"]
  #[inline] pub fn ckrdy(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Clock Ready Interrupt Enable"]
  #[inline] pub fn set_ckrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Clock Failure Detector Interrupt Enable"]
  #[inline] pub fn cfd(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Clock Failure Detector Interrupt Enable"]
  #[inline] pub fn set_cfd<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
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
#[doc="Interrupt Flag Status and Clear"]
#[derive(PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
#[doc="Clock Ready"]
  #[inline] pub fn ckrdy(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Clock Ready"]
  #[inline] pub fn set_ckrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Clock Failure Detector"]
  #[inline] pub fn cfd(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Clock Failure Detector"]
  #[inline] pub fn set_cfd<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
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
#[doc="Reset Cause"]
#[derive(PartialEq, Eq)]
pub struct Rcause(pub u8);
impl Rcause {
#[doc="Power On Reset"]
  #[inline] pub fn por(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Power On Reset"]
  #[inline] pub fn set_por<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Brown Out 12 Detector Reset"]
  #[inline] pub fn bod12(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Brown Out 12 Detector Reset"]
  #[inline] pub fn set_bod12<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Brown Out 33 Detector Reset"]
  #[inline] pub fn bod33(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="Brown Out 33 Detector Reset"]
  #[inline] pub fn set_bod33<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="External Reset"]
  #[inline] pub fn ext(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="External Reset"]
  #[inline] pub fn set_ext<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Watchdog Reset"]
  #[inline] pub fn wdt(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="Watchdog Reset"]
  #[inline] pub fn set_wdt<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="System Reset Request"]
  #[inline] pub fn syst(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="System Reset Request"]
  #[inline] pub fn set_syst<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
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
#[doc="Sleep Mode"]
#[derive(PartialEq, Eq)]
pub struct Sleep(pub u8);
impl Sleep {
#[doc="Idle Mode Configuration"]
  #[inline] pub fn idle(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
  }
#[doc="Idle Mode Configuration"]
  #[inline] pub fn set_idle<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u8 = value.into();
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
   #[inline] pub fn en<P: En>(&self, p: &P) -> u32 {
      p.en()
   }
   #[inline] pub fn set_en<P: En>(&self, p: &P, value: u32) {
      p.set_en(value)
   }
}

impl En for super::pm::Pm {
   #[inline] fn en(&self) -> u32 { PM.apbamask().pm().into() }
   #[inline] fn set_en(&self, value: u32) { PM.with_apbamask(|r| r.set_pm(value)); }
}

impl En for super::sysctrl::Sysctrl {
   #[inline] fn en(&self) -> u32 { PM.apbamask().sysctrl().into() }
   #[inline] fn set_en(&self, value: u32) { PM.with_apbamask(|r| r.set_sysctrl(value)); }
}

impl En for super::gclk::Gclk {
   #[inline] fn en(&self) -> u32 { PM.apbamask().gclk().into() }
   #[inline] fn set_en(&self, value: u32) { PM.with_apbamask(|r| r.set_gclk(value)); }
}

impl En for super::wdt::Wdt {
   #[inline] fn en(&self) -> u32 { PM.apbamask().wdt().into() }
   #[inline] fn set_en(&self, value: u32) { PM.with_apbamask(|r| r.set_wdt(value)); }
}

impl En for super::rtc::Rtc {
   #[inline] fn en(&self) -> u32 { PM.apbamask().rtc().into() }
   #[inline] fn set_en(&self, value: u32) { PM.with_apbamask(|r| r.set_rtc(value)); }
}

impl En for super::port::Porta {
   #[inline] fn en(&self) -> u32 { PM.apbbmask().port().into() }
   #[inline] fn set_en(&self, value: u32) { PM.with_apbbmask(|r| r.set_port(value)); }
}

impl En for super::port::Portb {
   #[inline] fn en(&self) -> u32 { PM.apbbmask().port().into() }
   #[inline] fn set_en(&self, value: u32) { PM.with_apbbmask(|r| r.set_port(value)); }
}

impl En for super::sercom::Sercom0 {
   #[inline] fn en(&self) -> u32 { PM.apbcmask().sercom0().into() }
   #[inline] fn set_en(&self, value: u32) { PM.with_apbcmask(|r| r.set_sercom0(value)); }
}

impl En for super::sercom::Sercom1 {
   #[inline] fn en(&self) -> u32 { PM.apbcmask().sercom1().into() }
   #[inline] fn set_en(&self, value: u32) { PM.with_apbcmask(|r| r.set_sercom1(value)); }
}

impl En for super::sercom::Sercom2 {
   #[inline] fn en(&self) -> u32 { PM.apbcmask().sercom2().into() }
   #[inline] fn set_en(&self, value: u32) { PM.with_apbcmask(|r| r.set_sercom2(value)); }
}

impl En for super::sercom::Sercom3 {
   #[inline] fn en(&self) -> u32 { PM.apbcmask().sercom3().into() }
   #[inline] fn set_en(&self, value: u32) { PM.with_apbcmask(|r| r.set_sercom3(value)); }
}

impl En for super::sercom::Sercom4 {
   #[inline] fn en(&self) -> u32 { PM.apbcmask().sercom4().into() }
   #[inline] fn set_en(&self, value: u32) { PM.with_apbcmask(|r| r.set_sercom4(value)); }
}

impl En for super::sercom::Sercom5 {
   #[inline] fn en(&self) -> u32 { PM.apbcmask().sercom5().into() }
   #[inline] fn set_en(&self, value: u32) { PM.with_apbcmask(|r| r.set_sercom5(value)); }
}

impl En for super::tcc::Tcc0 {
   #[inline] fn en(&self) -> u32 { PM.apbcmask().tcc0().into() }
   #[inline] fn set_en(&self, value: u32) { PM.with_apbcmask(|r| r.set_tcc0(value)); }
}

impl En for super::tcc::Tcc1 {
   #[inline] fn en(&self) -> u32 { PM.apbcmask().tcc1().into() }
   #[inline] fn set_en(&self, value: u32) { PM.with_apbcmask(|r| r.set_tcc1(value)); }
}

impl En for super::tcc::Tcc2 {
   #[inline] fn en(&self) -> u32 { PM.apbcmask().tcc2().into() }
   #[inline] fn set_en(&self, value: u32) { PM.with_apbcmask(|r| r.set_tcc2(value)); }
}

impl En for super::tc::Tc3 {
   #[inline] fn en(&self) -> u32 { PM.apbcmask().tc3().into() }
   #[inline] fn set_en(&self, value: u32) { PM.with_apbcmask(|r| r.set_tc3(value)); }
}

impl En for super::tc::Tc4 {
   #[inline] fn en(&self) -> u32 { PM.apbcmask().tc4().into() }
   #[inline] fn set_en(&self, value: u32) { PM.with_apbcmask(|r| r.set_tc4(value)); }
}

impl En for super::tc::Tc5 {
   #[inline] fn en(&self) -> u32 { PM.apbcmask().tc5().into() }
   #[inline] fn set_en(&self, value: u32) { PM.with_apbcmask(|r| r.set_tc5(value)); }
}

impl En for super::adc::Adc {
   #[inline] fn en(&self) -> u32 { PM.apbcmask().adc().into() }
   #[inline] fn set_en(&self, value: u32) { PM.with_apbcmask(|r| r.set_adc(value)); }
}


