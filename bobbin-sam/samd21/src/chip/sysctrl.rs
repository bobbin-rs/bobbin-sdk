//! System Control
pub const SYSCTRL: Sysctrl = Sysctrl(0x40000800);

#[doc="System Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sysctrl(pub u32);
impl Sysctrl {
#[doc="Get the *const pointer for the BOD33 register."]
  #[inline] pub fn bod33_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x34) as *const u32
  }
#[doc="Get the *mut pointer for the BOD33 register."]
  #[inline] pub fn bod33_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x34) as *mut u32
  }
#[doc="Read the BOD33 register."]
  #[inline] pub fn bod33(&self) -> Bod33 { 
     unsafe {
        Bod33(::core::ptr::read_volatile(((self.0 as usize) + 0x34) as *const u32))
     }
  }
#[doc="Write the BOD33 register."]
  #[inline] pub fn set_bod33(&self, value: Bod33) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x34) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the BOD33 register."]
  #[inline] pub fn with_bod33<F: FnOnce(Bod33) -> Bod33>(&self, f: F) -> &Self {
     let tmp = self.bod33();
     self.set_bod33(f(tmp))
  }

#[doc="Get the *const pointer for the DFLLCTRL register."]
  #[inline] pub fn dfllctrl_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x24) as *const u16
  }
#[doc="Get the *mut pointer for the DFLLCTRL register."]
  #[inline] pub fn dfllctrl_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x24) as *mut u16
  }
#[doc="Read the DFLLCTRL register."]
  #[inline] pub fn dfllctrl(&self) -> Dfllctrl { 
     unsafe {
        Dfllctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u16))
     }
  }
#[doc="Write the DFLLCTRL register."]
  #[inline] pub fn set_dfllctrl(&self, value: Dfllctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the DFLLCTRL register."]
  #[inline] pub fn with_dfllctrl<F: FnOnce(Dfllctrl) -> Dfllctrl>(&self, f: F) -> &Self {
     let tmp = self.dfllctrl();
     self.set_dfllctrl(f(tmp))
  }

#[doc="Get the *const pointer for the DFLLMUL register."]
  #[inline] pub fn dfllmul_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2c) as *const u32
  }
#[doc="Get the *mut pointer for the DFLLMUL register."]
  #[inline] pub fn dfllmul_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2c) as *mut u32
  }
#[doc="Read the DFLLMUL register."]
  #[inline] pub fn dfllmul(&self) -> Dfllmul { 
     unsafe {
        Dfllmul(::core::ptr::read_volatile(((self.0 as usize) + 0x2c) as *const u32))
     }
  }
#[doc="Write the DFLLMUL register."]
  #[inline] pub fn set_dfllmul(&self, value: Dfllmul) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DFLLMUL register."]
  #[inline] pub fn with_dfllmul<F: FnOnce(Dfllmul) -> Dfllmul>(&self, f: F) -> &Self {
     let tmp = self.dfllmul();
     self.set_dfllmul(f(tmp))
  }

#[doc="Get the *const pointer for the DFLLSYNC register."]
  #[inline] pub fn dfllsync_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x30) as *const u8
  }
#[doc="Get the *mut pointer for the DFLLSYNC register."]
  #[inline] pub fn dfllsync_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x30) as *mut u8
  }
#[doc="Read the DFLLSYNC register."]
  #[inline] pub fn dfllsync(&self) -> Dfllsync { 
     unsafe {
        Dfllsync(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u8))
     }
  }
#[doc="Write the DFLLSYNC register."]
  #[inline] pub fn set_dfllsync(&self, value: Dfllsync) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the DFLLSYNC register."]
  #[inline] pub fn with_dfllsync<F: FnOnce(Dfllsync) -> Dfllsync>(&self, f: F) -> &Self {
     let tmp = self.dfllsync();
     self.set_dfllsync(f(tmp))
  }

#[doc="Get the *const pointer for the DFLLVAL register."]
  #[inline] pub fn dfllval_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x28) as *const u32
  }
#[doc="Get the *mut pointer for the DFLLVAL register."]
  #[inline] pub fn dfllval_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x28) as *mut u32
  }
#[doc="Read the DFLLVAL register."]
  #[inline] pub fn dfllval(&self) -> Dfllval { 
     unsafe {
        Dfllval(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
     }
  }
#[doc="Write the DFLLVAL register."]
  #[inline] pub fn set_dfllval(&self, value: Dfllval) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DFLLVAL register."]
  #[inline] pub fn with_dfllval<F: FnOnce(Dfllval) -> Dfllval>(&self, f: F) -> &Self {
     let tmp = self.dfllval();
     self.set_dfllval(f(tmp))
  }

#[doc="Get the *const pointer for the DPLLCTRLA register."]
  #[inline] pub fn dpllctrla_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x44) as *const u8
  }
#[doc="Get the *mut pointer for the DPLLCTRLA register."]
  #[inline] pub fn dpllctrla_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x44) as *mut u8
  }
#[doc="Read the DPLLCTRLA register."]
  #[inline] pub fn dpllctrla(&self) -> Dpllctrla { 
     unsafe {
        Dpllctrla(::core::ptr::read_volatile(((self.0 as usize) + 0x44) as *const u8))
     }
  }
#[doc="Write the DPLLCTRLA register."]
  #[inline] pub fn set_dpllctrla(&self, value: Dpllctrla) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x44) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the DPLLCTRLA register."]
  #[inline] pub fn with_dpllctrla<F: FnOnce(Dpllctrla) -> Dpllctrla>(&self, f: F) -> &Self {
     let tmp = self.dpllctrla();
     self.set_dpllctrla(f(tmp))
  }

#[doc="Get the *const pointer for the DPLLCTRLB register."]
  #[inline] pub fn dpllctrlb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4c) as *const u32
  }
#[doc="Get the *mut pointer for the DPLLCTRLB register."]
  #[inline] pub fn dpllctrlb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4c) as *mut u32
  }
#[doc="Read the DPLLCTRLB register."]
  #[inline] pub fn dpllctrlb(&self) -> Dpllctrlb { 
     unsafe {
        Dpllctrlb(::core::ptr::read_volatile(((self.0 as usize) + 0x4c) as *const u32))
     }
  }
#[doc="Write the DPLLCTRLB register."]
  #[inline] pub fn set_dpllctrlb(&self, value: Dpllctrlb) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DPLLCTRLB register."]
  #[inline] pub fn with_dpllctrlb<F: FnOnce(Dpllctrlb) -> Dpllctrlb>(&self, f: F) -> &Self {
     let tmp = self.dpllctrlb();
     self.set_dpllctrlb(f(tmp))
  }

#[doc="Get the *const pointer for the DPLLRATIO register."]
  #[inline] pub fn dpllratio_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x48) as *const u32
  }
#[doc="Get the *mut pointer for the DPLLRATIO register."]
  #[inline] pub fn dpllratio_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x48) as *mut u32
  }
#[doc="Read the DPLLRATIO register."]
  #[inline] pub fn dpllratio(&self) -> Dpllratio { 
     unsafe {
        Dpllratio(::core::ptr::read_volatile(((self.0 as usize) + 0x48) as *const u32))
     }
  }
#[doc="Write the DPLLRATIO register."]
  #[inline] pub fn set_dpllratio(&self, value: Dpllratio) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x48) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DPLLRATIO register."]
  #[inline] pub fn with_dpllratio<F: FnOnce(Dpllratio) -> Dpllratio>(&self, f: F) -> &Self {
     let tmp = self.dpllratio();
     self.set_dpllratio(f(tmp))
  }

#[doc="Get the *const pointer for the DPLLSTATUS register."]
  #[inline] pub fn dpllstatus_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x50) as *const u8
  }
#[doc="Get the *mut pointer for the DPLLSTATUS register."]
  #[inline] pub fn dpllstatus_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x50) as *mut u8
  }
#[doc="Read the DPLLSTATUS register."]
  #[inline] pub fn dpllstatus(&self) -> Dpllstatus { 
     unsafe {
        Dpllstatus(::core::ptr::read_volatile(((self.0 as usize) + 0x50) as *const u8))
     }
  }

#[doc="Get the *const pointer for the INTENCLR register."]
  #[inline] pub fn intenclr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the INTENCLR register."]
  #[inline] pub fn intenclr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the INTENCLR register."]
  #[inline] pub fn intenclr(&self) -> Intenclr { 
     unsafe {
        Intenclr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the INTENCLR register."]
  #[inline] pub fn set_intenclr(&self, value: Intenclr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the INTENCLR register."]
  #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
     let tmp = self.intenclr();
     self.set_intenclr(f(tmp))
  }

#[doc="Get the *const pointer for the INTENSET register."]
  #[inline] pub fn intenset_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the INTENSET register."]
  #[inline] pub fn intenset_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the INTENSET register."]
  #[inline] pub fn intenset(&self) -> Intenset { 
     unsafe {
        Intenset(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the INTENSET register."]
  #[inline] pub fn set_intenset(&self, value: Intenset) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the INTENSET register."]
  #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
     let tmp = self.intenset();
     self.set_intenset(f(tmp))
  }

#[doc="Get the *const pointer for the INTFLAG register."]
  #[inline] pub fn intflag_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the INTFLAG register."]
  #[inline] pub fn intflag_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the INTFLAG register."]
  #[inline] pub fn intflag(&self) -> Intflag { 
     unsafe {
        Intflag(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the INTFLAG register."]
  #[inline] pub fn set_intflag(&self, value: Intflag) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the INTFLAG register."]
  #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
     let tmp = self.intflag();
     self.set_intflag(f(tmp))
  }

#[doc="Get the *const pointer for the OSCULP32K register."]
  #[inline] pub fn osculp32k_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x1c) as *const u8
  }
#[doc="Get the *mut pointer for the OSCULP32K register."]
  #[inline] pub fn osculp32k_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x1c) as *mut u8
  }
#[doc="Read the OSCULP32K register."]
  #[inline] pub fn osculp32k(&self) -> Osculp32k { 
     unsafe {
        Osculp32k(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u8))
     }
  }
#[doc="Write the OSCULP32K register."]
  #[inline] pub fn set_osculp32k(&self, value: Osculp32k) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the OSCULP32K register."]
  #[inline] pub fn with_osculp32k<F: FnOnce(Osculp32k) -> Osculp32k>(&self, f: F) -> &Self {
     let tmp = self.osculp32k();
     self.set_osculp32k(f(tmp))
  }

#[doc="Get the *const pointer for the OSC8M register."]
  #[inline] pub fn osc8m_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
#[doc="Get the *mut pointer for the OSC8M register."]
  #[inline] pub fn osc8m_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
#[doc="Read the OSC8M register."]
  #[inline] pub fn osc8m(&self) -> Osc8m { 
     unsafe {
        Osc8m(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
#[doc="Write the OSC8M register."]
  #[inline] pub fn set_osc8m(&self, value: Osc8m) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the OSC8M register."]
  #[inline] pub fn with_osc8m<F: FnOnce(Osc8m) -> Osc8m>(&self, f: F) -> &Self {
     let tmp = self.osc8m();
     self.set_osc8m(f(tmp))
  }

#[doc="Get the *const pointer for the OSC32K register."]
  #[inline] pub fn osc32k_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
#[doc="Get the *mut pointer for the OSC32K register."]
  #[inline] pub fn osc32k_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
#[doc="Read the OSC32K register."]
  #[inline] pub fn osc32k(&self) -> Osc32k { 
     unsafe {
        Osc32k(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
#[doc="Write the OSC32K register."]
  #[inline] pub fn set_osc32k(&self, value: Osc32k) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the OSC32K register."]
  #[inline] pub fn with_osc32k<F: FnOnce(Osc32k) -> Osc32k>(&self, f: F) -> &Self {
     let tmp = self.osc32k();
     self.set_osc32k(f(tmp))
  }

#[doc="Get the *const pointer for the PCLKSR register."]
  #[inline] pub fn pclksr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the PCLKSR register."]
  #[inline] pub fn pclksr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the PCLKSR register."]
  #[inline] pub fn pclksr(&self) -> Pclksr { 
     unsafe {
        Pclksr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }

#[doc="Get the *const pointer for the VREF register."]
  #[inline] pub fn vref_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x40) as *const u32
  }
#[doc="Get the *mut pointer for the VREF register."]
  #[inline] pub fn vref_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x40) as *mut u32
  }
#[doc="Read the VREF register."]
  #[inline] pub fn vref(&self) -> Vref { 
     unsafe {
        Vref(::core::ptr::read_volatile(((self.0 as usize) + 0x40) as *const u32))
     }
  }
#[doc="Write the VREF register."]
  #[inline] pub fn set_vref(&self, value: Vref) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the VREF register."]
  #[inline] pub fn with_vref<F: FnOnce(Vref) -> Vref>(&self, f: F) -> &Self {
     let tmp = self.vref();
     self.set_vref(f(tmp))
  }

#[doc="Get the *const pointer for the XOSC register."]
  #[inline] pub fn xosc_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x10) as *const u16
  }
#[doc="Get the *mut pointer for the XOSC register."]
  #[inline] pub fn xosc_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x10) as *mut u16
  }
#[doc="Read the XOSC register."]
  #[inline] pub fn xosc(&self) -> Xosc { 
     unsafe {
        Xosc(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u16))
     }
  }
#[doc="Write the XOSC register."]
  #[inline] pub fn set_xosc(&self, value: Xosc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the XOSC register."]
  #[inline] pub fn with_xosc<F: FnOnce(Xosc) -> Xosc>(&self, f: F) -> &Self {
     let tmp = self.xosc();
     self.set_xosc(f(tmp))
  }

#[doc="Get the *const pointer for the XOSC32K register."]
  #[inline] pub fn xosc32k_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x14) as *const u16
  }
#[doc="Get the *mut pointer for the XOSC32K register."]
  #[inline] pub fn xosc32k_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x14) as *mut u16
  }
#[doc="Read the XOSC32K register."]
  #[inline] pub fn xosc32k(&self) -> Xosc32k { 
     unsafe {
        Xosc32k(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u16))
     }
  }
#[doc="Write the XOSC32K register."]
  #[inline] pub fn set_xosc32k(&self, value: Xosc32k) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the XOSC32K register."]
  #[inline] pub fn with_xosc32k<F: FnOnce(Xosc32k) -> Xosc32k>(&self, f: F) -> &Self {
     let tmp = self.xosc32k();
     self.set_xosc32k(f(tmp))
  }

}

#[doc="3.3V Brown-Out Detector (BOD33) Control"]
#[derive(PartialEq, Eq)]
pub struct Bod33(pub u32);
impl Bod33 {
#[doc="Enable"]
  #[inline] pub fn enable(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Enable"]
  #[inline] pub fn set_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Hysteresis"]
  #[inline] pub fn hyst(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Hysteresis"]
  #[inline] pub fn set_hyst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="BOD33 Action"]
  #[inline] pub fn action(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x3 // [4:3]
  }
#[doc="BOD33 Action"]
  #[inline] pub fn set_action(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Run in Standby"]
  #[inline] pub fn runstdby(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Run in Standby"]
  #[inline] pub fn set_runstdby(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Operation Mode"]
  #[inline] pub fn mode(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Operation Mode"]
  #[inline] pub fn set_mode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Clock Enable"]
  #[inline] pub fn cen(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Clock Enable"]
  #[inline] pub fn set_cen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Prescaler Select"]
  #[inline] pub fn psel(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0xf // [15:12]
  }
#[doc="Prescaler Select"]
  #[inline] pub fn set_psel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

#[doc="BOD33 Threshold Level"]
  #[inline] pub fn level(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3f // [21:16]
  }
#[doc="BOD33 Threshold Level"]
  #[inline] pub fn set_level(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Bod33 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Bod33 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.hyst() != 0 { try!(write!(f, " hyst"))}
      if self.action() != 0 { try!(write!(f, " action=0x{:x}", self.action()))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      if self.mode() != 0 { try!(write!(f, " mode"))}
      if self.cen() != 0 { try!(write!(f, " cen"))}
      if self.psel() != 0 { try!(write!(f, " psel=0x{:x}", self.psel()))}
      if self.level() != 0 { try!(write!(f, " level=0x{:x}", self.level()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DFLL48M Control"]
#[derive(PartialEq, Eq)]
pub struct Dfllctrl(pub u16);
impl Dfllctrl {
#[doc="DFLL Enable"]
  #[inline] pub fn enable(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
#[doc="DFLL Enable"]
  #[inline] pub fn set_enable(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Operating Mode Selection"]
  #[inline] pub fn mode(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
#[doc="Operating Mode Selection"]
  #[inline] pub fn set_mode(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Stable DFLL Frequency"]
  #[inline] pub fn stable(&self) -> u16 {
     ((self.0 as u16) >> 3) & 0x1 // [3]
  }
#[doc="Stable DFLL Frequency"]
  #[inline] pub fn set_stable(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Lose Lock After Wake"]
  #[inline] pub fn llaw(&self) -> u16 {
     ((self.0 as u16) >> 4) & 0x1 // [4]
  }
#[doc="Lose Lock After Wake"]
  #[inline] pub fn set_llaw(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="USB Clock Recovery Mode"]
  #[inline] pub fn usbcrm(&self) -> u16 {
     ((self.0 as u16) >> 5) & 0x1 // [5]
  }
#[doc="USB Clock Recovery Mode"]
  #[inline] pub fn set_usbcrm(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Run in Standby"]
  #[inline] pub fn runstdby(&self) -> u16 {
     ((self.0 as u16) >> 6) & 0x1 // [6]
  }
#[doc="Run in Standby"]
  #[inline] pub fn set_runstdby(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="On Demand Control"]
  #[inline] pub fn ondemand(&self) -> u16 {
     ((self.0 as u16) >> 7) & 0x1 // [7]
  }
#[doc="On Demand Control"]
  #[inline] pub fn set_ondemand(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Chill Cycle Disable"]
  #[inline] pub fn ccdis(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x1 // [8]
  }
#[doc="Chill Cycle Disable"]
  #[inline] pub fn set_ccdis(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Quick Lock Disable"]
  #[inline] pub fn qldis(&self) -> u16 {
     ((self.0 as u16) >> 9) & 0x1 // [9]
  }
#[doc="Quick Lock Disable"]
  #[inline] pub fn set_qldis(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Bypass Coarse Lock"]
  #[inline] pub fn bplckc(&self) -> u16 {
     ((self.0 as u16) >> 10) & 0x1 // [10]
  }
#[doc="Bypass Coarse Lock"]
  #[inline] pub fn set_bplckc(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Wait Lock"]
  #[inline] pub fn waitlock(&self) -> u16 {
     ((self.0 as u16) >> 11) & 0x1 // [11]
  }
#[doc="Wait Lock"]
  #[inline] pub fn set_waitlock(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

}
impl ::core::fmt::Display for Dfllctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dfllctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.mode() != 0 { try!(write!(f, " mode"))}
      if self.stable() != 0 { try!(write!(f, " stable"))}
      if self.llaw() != 0 { try!(write!(f, " llaw"))}
      if self.usbcrm() != 0 { try!(write!(f, " usbcrm"))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      if self.ondemand() != 0 { try!(write!(f, " ondemand"))}
      if self.ccdis() != 0 { try!(write!(f, " ccdis"))}
      if self.qldis() != 0 { try!(write!(f, " qldis"))}
      if self.bplckc() != 0 { try!(write!(f, " bplckc"))}
      if self.waitlock() != 0 { try!(write!(f, " waitlock"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DFLL48M Multiplier"]
#[derive(PartialEq, Eq)]
pub struct Dfllmul(pub u32);
impl Dfllmul {
#[doc="DFLL Multiply Factor"]
  #[inline] pub fn mul(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="DFLL Multiply Factor"]
  #[inline] pub fn set_mul(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Fine Maximum Step"]
  #[inline] pub fn fstep(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3ff // [25:16]
  }
#[doc="Fine Maximum Step"]
  #[inline] pub fn set_fstep(mut self, value: u32) -> Self {
     assert!((value & !0x3ff) == 0);
     self.0 &= !(0x3ff << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Coarse Maximum Step"]
  #[inline] pub fn cstep(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x3f // [31:26]
  }
#[doc="Coarse Maximum Step"]
  #[inline] pub fn set_cstep(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 26);
     self.0 |= value << 26;
     self
  }

}
impl ::core::fmt::Display for Dfllmul {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dfllmul {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mul() != 0 { try!(write!(f, " mul=0x{:x}", self.mul()))}
      if self.fstep() != 0 { try!(write!(f, " fstep=0x{:x}", self.fstep()))}
      if self.cstep() != 0 { try!(write!(f, " cstep=0x{:x}", self.cstep()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DFLL48M Synchronization"]
#[derive(PartialEq, Eq)]
pub struct Dfllsync(pub u8);
impl Dfllsync {
#[doc="Read Request"]
  #[inline] pub fn readreq(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
#[doc="Read Request"]
  #[inline] pub fn set_readreq(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Dfllsync {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dfllsync {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.readreq() != 0 { try!(write!(f, " readreq"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DFLL48M Value"]
#[derive(PartialEq, Eq)]
pub struct Dfllval(pub u32);
impl Dfllval {
#[doc="Fine Value"]
  #[inline] pub fn fine(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3ff // [9:0]
  }
#[doc="Fine Value"]
  #[inline] pub fn set_fine(mut self, value: u32) -> Self {
     assert!((value & !0x3ff) == 0);
     self.0 &= !(0x3ff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Coarse Value"]
  #[inline] pub fn coarse(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3f // [15:10]
  }
#[doc="Coarse Value"]
  #[inline] pub fn set_coarse(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Multiplication Ratio Difference"]
  #[inline] pub fn diff(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xffff // [31:16]
  }
#[doc="Multiplication Ratio Difference"]
  #[inline] pub fn set_diff(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Dfllval {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dfllval {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fine() != 0 { try!(write!(f, " fine=0x{:x}", self.fine()))}
      if self.coarse() != 0 { try!(write!(f, " coarse=0x{:x}", self.coarse()))}
      if self.diff() != 0 { try!(write!(f, " diff=0x{:x}", self.diff()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DPLL Control A"]
#[derive(PartialEq, Eq)]
pub struct Dpllctrla(pub u8);
impl Dpllctrla {
#[doc="DPLL Enable"]
  #[inline] pub fn enable(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="DPLL Enable"]
  #[inline] pub fn set_enable(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Run in Standby"]
  #[inline] pub fn runstdby(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
#[doc="Run in Standby"]
  #[inline] pub fn set_runstdby(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="On Demand Clock Activation"]
  #[inline] pub fn ondemand(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
#[doc="On Demand Clock Activation"]
  #[inline] pub fn set_ondemand(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Dpllctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dpllctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      if self.ondemand() != 0 { try!(write!(f, " ondemand"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DPLL Control B"]
#[derive(PartialEq, Eq)]
pub struct Dpllctrlb(pub u32);
impl Dpllctrlb {
#[doc="Proportional Integral Filter Selection"]
  #[inline] pub fn filter(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
#[doc="Proportional Integral Filter Selection"]
  #[inline] pub fn set_filter(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Low-Power Enable"]
  #[inline] pub fn lpen(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Low-Power Enable"]
  #[inline] pub fn set_lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Wake Up Fast"]
  #[inline] pub fn wuf(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Wake Up Fast"]
  #[inline] pub fn set_wuf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Reference Clock Selection"]
  #[inline] pub fn refclk(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
#[doc="Reference Clock Selection"]
  #[inline] pub fn set_refclk(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Lock Time"]
  #[inline] pub fn ltime(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x7 // [10:8]
  }
#[doc="Lock Time"]
  #[inline] pub fn set_ltime(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Lock Bypass"]
  #[inline] pub fn lbypass(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="Lock Bypass"]
  #[inline] pub fn set_lbypass(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Clock Divider"]
  #[inline] pub fn div(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x7ff // [26:16]
  }
#[doc="Clock Divider"]
  #[inline] pub fn set_div(mut self, value: u32) -> Self {
     assert!((value & !0x7ff) == 0);
     self.0 &= !(0x7ff << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Dpllctrlb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dpllctrlb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.filter() != 0 { try!(write!(f, " filter=0x{:x}", self.filter()))}
      if self.lpen() != 0 { try!(write!(f, " lpen"))}
      if self.wuf() != 0 { try!(write!(f, " wuf"))}
      if self.refclk() != 0 { try!(write!(f, " refclk=0x{:x}", self.refclk()))}
      if self.ltime() != 0 { try!(write!(f, " ltime=0x{:x}", self.ltime()))}
      if self.lbypass() != 0 { try!(write!(f, " lbypass"))}
      if self.div() != 0 { try!(write!(f, " div=0x{:x}", self.div()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DPLL Ratio Control"]
#[derive(PartialEq, Eq)]
pub struct Dpllratio(pub u32);
impl Dpllratio {
#[doc="Loop Divider Ratio"]
  #[inline] pub fn ldr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
#[doc="Loop Divider Ratio"]
  #[inline] pub fn set_ldr(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Loop Divider Ratio Fractional Part"]
  #[inline] pub fn ldrfrac(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
#[doc="Loop Divider Ratio Fractional Part"]
  #[inline] pub fn set_ldrfrac(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Dpllratio {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dpllratio {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ldr() != 0 { try!(write!(f, " ldr=0x{:x}", self.ldr()))}
      if self.ldrfrac() != 0 { try!(write!(f, " ldrfrac=0x{:x}", self.ldrfrac()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DPLL Status"]
#[derive(PartialEq, Eq)]
pub struct Dpllstatus(pub u8);
impl Dpllstatus {
#[doc="DPLL Lock Status"]
  #[inline] pub fn lock(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="DPLL Lock Status"]
  #[inline] pub fn set_lock(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Output Clock Ready"]
  #[inline] pub fn clkrdy(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Output Clock Ready"]
  #[inline] pub fn set_clkrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="DPLL Enable"]
  #[inline] pub fn enable(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
#[doc="DPLL Enable"]
  #[inline] pub fn set_enable(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Divider Enable"]
  #[inline] pub fn div(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
#[doc="Divider Enable"]
  #[inline] pub fn set_div(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}
impl ::core::fmt::Display for Dpllstatus {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dpllstatus {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lock() != 0 { try!(write!(f, " lock"))}
      if self.clkrdy() != 0 { try!(write!(f, " clkrdy"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.div() != 0 { try!(write!(f, " div"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Enable Clear"]
#[derive(PartialEq, Eq)]
pub struct Intenclr(pub u32);
impl Intenclr {
#[doc="XOSC Ready Interrupt Enable"]
  #[inline] pub fn xoscrdy(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="XOSC Ready Interrupt Enable"]
  #[inline] pub fn set_xoscrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="XOSC32K Ready Interrupt Enable"]
  #[inline] pub fn xosc32krdy(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="XOSC32K Ready Interrupt Enable"]
  #[inline] pub fn set_xosc32krdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="OSC32K Ready Interrupt Enable"]
  #[inline] pub fn osc32krdy(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="OSC32K Ready Interrupt Enable"]
  #[inline] pub fn set_osc32krdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="OSC8M Ready Interrupt Enable"]
  #[inline] pub fn osc8mrdy(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="OSC8M Ready Interrupt Enable"]
  #[inline] pub fn set_osc8mrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="DFLL Ready Interrupt Enable"]
  #[inline] pub fn dfllrdy(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="DFLL Ready Interrupt Enable"]
  #[inline] pub fn set_dfllrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="DFLL Out Of Bounds Interrupt Enable"]
  #[inline] pub fn dflloob(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="DFLL Out Of Bounds Interrupt Enable"]
  #[inline] pub fn set_dflloob(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="DFLL Lock Fine Interrupt Enable"]
  #[inline] pub fn dflllckf(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="DFLL Lock Fine Interrupt Enable"]
  #[inline] pub fn set_dflllckf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="DFLL Lock Coarse Interrupt Enable"]
  #[inline] pub fn dflllckc(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="DFLL Lock Coarse Interrupt Enable"]
  #[inline] pub fn set_dflllckc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="DFLL Reference Clock Stopped Interrupt Enable"]
  #[inline] pub fn dfllrcs(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="DFLL Reference Clock Stopped Interrupt Enable"]
  #[inline] pub fn set_dfllrcs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="BOD33 Ready Interrupt Enable"]
  #[inline] pub fn bod33rdy(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="BOD33 Ready Interrupt Enable"]
  #[inline] pub fn set_bod33rdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="BOD33 Detection Interrupt Enable"]
  #[inline] pub fn bod33det(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="BOD33 Detection Interrupt Enable"]
  #[inline] pub fn set_bod33det(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="BOD33 Synchronization Ready Interrupt Enable"]
  #[inline] pub fn b33srdy(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="BOD33 Synchronization Ready Interrupt Enable"]
  #[inline] pub fn set_b33srdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="DPLL Lock Rise Interrupt Enable"]
  #[inline] pub fn dplllckr(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="DPLL Lock Rise Interrupt Enable"]
  #[inline] pub fn set_dplllckr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="DPLL Lock Fall Interrupt Enable"]
  #[inline] pub fn dplllckf(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="DPLL Lock Fall Interrupt Enable"]
  #[inline] pub fn set_dplllckf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="DPLL Lock Timeout Interrupt Enable"]
  #[inline] pub fn dplllto(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="DPLL Lock Timeout Interrupt Enable"]
  #[inline] pub fn set_dplllto(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
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
      if self.xoscrdy() != 0 { try!(write!(f, " xoscrdy"))}
      if self.xosc32krdy() != 0 { try!(write!(f, " xosc32krdy"))}
      if self.osc32krdy() != 0 { try!(write!(f, " osc32krdy"))}
      if self.osc8mrdy() != 0 { try!(write!(f, " osc8mrdy"))}
      if self.dfllrdy() != 0 { try!(write!(f, " dfllrdy"))}
      if self.dflloob() != 0 { try!(write!(f, " dflloob"))}
      if self.dflllckf() != 0 { try!(write!(f, " dflllckf"))}
      if self.dflllckc() != 0 { try!(write!(f, " dflllckc"))}
      if self.dfllrcs() != 0 { try!(write!(f, " dfllrcs"))}
      if self.bod33rdy() != 0 { try!(write!(f, " bod33rdy"))}
      if self.bod33det() != 0 { try!(write!(f, " bod33det"))}
      if self.b33srdy() != 0 { try!(write!(f, " b33srdy"))}
      if self.dplllckr() != 0 { try!(write!(f, " dplllckr"))}
      if self.dplllckf() != 0 { try!(write!(f, " dplllckf"))}
      if self.dplllto() != 0 { try!(write!(f, " dplllto"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Enable Set"]
#[derive(PartialEq, Eq)]
pub struct Intenset(pub u32);
impl Intenset {
#[doc="XOSC Ready Interrupt Enable"]
  #[inline] pub fn xoscrdy(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="XOSC Ready Interrupt Enable"]
  #[inline] pub fn set_xoscrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="XOSC32K Ready Interrupt Enable"]
  #[inline] pub fn xosc32krdy(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="XOSC32K Ready Interrupt Enable"]
  #[inline] pub fn set_xosc32krdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="OSC32K Ready Interrupt Enable"]
  #[inline] pub fn osc32krdy(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="OSC32K Ready Interrupt Enable"]
  #[inline] pub fn set_osc32krdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="OSC8M Ready Interrupt Enable"]
  #[inline] pub fn osc8mrdy(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="OSC8M Ready Interrupt Enable"]
  #[inline] pub fn set_osc8mrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="DFLL Ready Interrupt Enable"]
  #[inline] pub fn dfllrdy(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="DFLL Ready Interrupt Enable"]
  #[inline] pub fn set_dfllrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="DFLL Out Of Bounds Interrupt Enable"]
  #[inline] pub fn dflloob(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="DFLL Out Of Bounds Interrupt Enable"]
  #[inline] pub fn set_dflloob(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="DFLL Lock Fine Interrupt Enable"]
  #[inline] pub fn dflllckf(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="DFLL Lock Fine Interrupt Enable"]
  #[inline] pub fn set_dflllckf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="DFLL Lock Coarse Interrupt Enable"]
  #[inline] pub fn dflllckc(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="DFLL Lock Coarse Interrupt Enable"]
  #[inline] pub fn set_dflllckc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="DFLL Reference Clock Stopped Interrupt Enable"]
  #[inline] pub fn dfllrcs(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="DFLL Reference Clock Stopped Interrupt Enable"]
  #[inline] pub fn set_dfllrcs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="BOD33 Ready Interrupt Enable"]
  #[inline] pub fn bod33rdy(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="BOD33 Ready Interrupt Enable"]
  #[inline] pub fn set_bod33rdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="BOD33 Detection Interrupt Enable"]
  #[inline] pub fn bod33det(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="BOD33 Detection Interrupt Enable"]
  #[inline] pub fn set_bod33det(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="BOD33 Synchronization Ready Interrupt Enable"]
  #[inline] pub fn b33srdy(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="BOD33 Synchronization Ready Interrupt Enable"]
  #[inline] pub fn set_b33srdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="DPLL Lock Rise Interrupt Enable"]
  #[inline] pub fn dplllckr(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="DPLL Lock Rise Interrupt Enable"]
  #[inline] pub fn set_dplllckr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="DPLL Lock Fall Interrupt Enable"]
  #[inline] pub fn dplllckf(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="DPLL Lock Fall Interrupt Enable"]
  #[inline] pub fn set_dplllckf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="DPLL Lock Timeout Interrupt Enable"]
  #[inline] pub fn dplllto(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="DPLL Lock Timeout Interrupt Enable"]
  #[inline] pub fn set_dplllto(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
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
      if self.xoscrdy() != 0 { try!(write!(f, " xoscrdy"))}
      if self.xosc32krdy() != 0 { try!(write!(f, " xosc32krdy"))}
      if self.osc32krdy() != 0 { try!(write!(f, " osc32krdy"))}
      if self.osc8mrdy() != 0 { try!(write!(f, " osc8mrdy"))}
      if self.dfllrdy() != 0 { try!(write!(f, " dfllrdy"))}
      if self.dflloob() != 0 { try!(write!(f, " dflloob"))}
      if self.dflllckf() != 0 { try!(write!(f, " dflllckf"))}
      if self.dflllckc() != 0 { try!(write!(f, " dflllckc"))}
      if self.dfllrcs() != 0 { try!(write!(f, " dfllrcs"))}
      if self.bod33rdy() != 0 { try!(write!(f, " bod33rdy"))}
      if self.bod33det() != 0 { try!(write!(f, " bod33det"))}
      if self.b33srdy() != 0 { try!(write!(f, " b33srdy"))}
      if self.dplllckr() != 0 { try!(write!(f, " dplllckr"))}
      if self.dplllckf() != 0 { try!(write!(f, " dplllckf"))}
      if self.dplllto() != 0 { try!(write!(f, " dplllto"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Flag Status and Clear"]
#[derive(PartialEq, Eq)]
pub struct Intflag(pub u32);
impl Intflag {
#[doc="XOSC Ready"]
  #[inline] pub fn xoscrdy(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="XOSC Ready"]
  #[inline] pub fn set_xoscrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="XOSC32K Ready"]
  #[inline] pub fn xosc32krdy(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="XOSC32K Ready"]
  #[inline] pub fn set_xosc32krdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="OSC32K Ready"]
  #[inline] pub fn osc32krdy(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="OSC32K Ready"]
  #[inline] pub fn set_osc32krdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="OSC8M Ready"]
  #[inline] pub fn osc8mrdy(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="OSC8M Ready"]
  #[inline] pub fn set_osc8mrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="DFLL Ready"]
  #[inline] pub fn dfllrdy(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="DFLL Ready"]
  #[inline] pub fn set_dfllrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="DFLL Out Of Bounds"]
  #[inline] pub fn dflloob(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="DFLL Out Of Bounds"]
  #[inline] pub fn set_dflloob(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="DFLL Lock Fine"]
  #[inline] pub fn dflllckf(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="DFLL Lock Fine"]
  #[inline] pub fn set_dflllckf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="DFLL Lock Coarse"]
  #[inline] pub fn dflllckc(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="DFLL Lock Coarse"]
  #[inline] pub fn set_dflllckc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="DFLL Reference Clock Stopped"]
  #[inline] pub fn dfllrcs(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="DFLL Reference Clock Stopped"]
  #[inline] pub fn set_dfllrcs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="BOD33 Ready"]
  #[inline] pub fn bod33rdy(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="BOD33 Ready"]
  #[inline] pub fn set_bod33rdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="BOD33 Detection"]
  #[inline] pub fn bod33det(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="BOD33 Detection"]
  #[inline] pub fn set_bod33det(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="BOD33 Synchronization Ready"]
  #[inline] pub fn b33srdy(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="BOD33 Synchronization Ready"]
  #[inline] pub fn set_b33srdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="DPLL Lock Rise"]
  #[inline] pub fn dplllckr(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="DPLL Lock Rise"]
  #[inline] pub fn set_dplllckr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="DPLL Lock Fall"]
  #[inline] pub fn dplllckf(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="DPLL Lock Fall"]
  #[inline] pub fn set_dplllckf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="DPLL Lock Timeout"]
  #[inline] pub fn dplllto(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="DPLL Lock Timeout"]
  #[inline] pub fn set_dplllto(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
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
      if self.xoscrdy() != 0 { try!(write!(f, " xoscrdy"))}
      if self.xosc32krdy() != 0 { try!(write!(f, " xosc32krdy"))}
      if self.osc32krdy() != 0 { try!(write!(f, " osc32krdy"))}
      if self.osc8mrdy() != 0 { try!(write!(f, " osc8mrdy"))}
      if self.dfllrdy() != 0 { try!(write!(f, " dfllrdy"))}
      if self.dflloob() != 0 { try!(write!(f, " dflloob"))}
      if self.dflllckf() != 0 { try!(write!(f, " dflllckf"))}
      if self.dflllckc() != 0 { try!(write!(f, " dflllckc"))}
      if self.dfllrcs() != 0 { try!(write!(f, " dfllrcs"))}
      if self.bod33rdy() != 0 { try!(write!(f, " bod33rdy"))}
      if self.bod33det() != 0 { try!(write!(f, " bod33det"))}
      if self.b33srdy() != 0 { try!(write!(f, " b33srdy"))}
      if self.dplllckr() != 0 { try!(write!(f, " dplllckr"))}
      if self.dplllckf() != 0 { try!(write!(f, " dplllckf"))}
      if self.dplllto() != 0 { try!(write!(f, " dplllto"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control"]
#[derive(PartialEq, Eq)]
pub struct Osculp32k(pub u8);
impl Osculp32k {
#[doc="Oscillator Calibration"]
  #[inline] pub fn calib(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1f // [4:0]
  }
#[doc="Oscillator Calibration"]
  #[inline] pub fn set_calib(mut self, value: u8) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Write Lock"]
  #[inline] pub fn wrtlock(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
#[doc="Write Lock"]
  #[inline] pub fn set_wrtlock(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Osculp32k {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Osculp32k {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.calib() != 0 { try!(write!(f, " calib=0x{:x}", self.calib()))}
      if self.wrtlock() != 0 { try!(write!(f, " wrtlock"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="8MHz Internal Oscillator (OSC8M) Control"]
#[derive(PartialEq, Eq)]
pub struct Osc8m(pub u32);
impl Osc8m {
#[doc="Oscillator Enable"]
  #[inline] pub fn enable(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Oscillator Enable"]
  #[inline] pub fn set_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Run in Standby"]
  #[inline] pub fn runstdby(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Run in Standby"]
  #[inline] pub fn set_runstdby(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="On Demand Control"]
  #[inline] pub fn ondemand(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="On Demand Control"]
  #[inline] pub fn set_ondemand(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Oscillator Prescaler"]
  #[inline] pub fn presc(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
#[doc="Oscillator Prescaler"]
  #[inline] pub fn set_presc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Oscillator Calibration"]
  #[inline] pub fn calib(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xfff // [27:16]
  }
#[doc="Oscillator Calibration"]
  #[inline] pub fn set_calib(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Oscillator Frequency Range"]
  #[inline] pub fn frange(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x3 // [31:30]
  }
#[doc="Oscillator Frequency Range"]
  #[inline] pub fn set_frange(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 30);
     self.0 |= value << 30;
     self
  }

}
impl ::core::fmt::Display for Osc8m {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Osc8m {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      if self.ondemand() != 0 { try!(write!(f, " ondemand"))}
      if self.presc() != 0 { try!(write!(f, " presc=0x{:x}", self.presc()))}
      if self.calib() != 0 { try!(write!(f, " calib=0x{:x}", self.calib()))}
      if self.frange() != 0 { try!(write!(f, " frange=0x{:x}", self.frange()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="32kHz Internal Oscillator (OSC32K) Control"]
#[derive(PartialEq, Eq)]
pub struct Osc32k(pub u32);
impl Osc32k {
#[doc="Oscillator Enable"]
  #[inline] pub fn enable(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Oscillator Enable"]
  #[inline] pub fn set_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="32kHz Output Enable"]
  #[inline] pub fn en32k(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="32kHz Output Enable"]
  #[inline] pub fn set_en32k(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="1kHz Output Enable"]
  #[inline] pub fn en1k(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="1kHz Output Enable"]
  #[inline] pub fn set_en1k(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Run in Standby"]
  #[inline] pub fn runstdby(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Run in Standby"]
  #[inline] pub fn set_runstdby(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="On Demand Control"]
  #[inline] pub fn ondemand(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="On Demand Control"]
  #[inline] pub fn set_ondemand(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Oscillator Start-Up Time"]
  #[inline] pub fn startup(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x7 // [10:8]
  }
#[doc="Oscillator Start-Up Time"]
  #[inline] pub fn set_startup(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Write Lock"]
  #[inline] pub fn wrtlock(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="Write Lock"]
  #[inline] pub fn set_wrtlock(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Oscillator Calibration"]
  #[inline] pub fn calib(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x7f // [22:16]
  }
#[doc="Oscillator Calibration"]
  #[inline] pub fn set_calib(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Osc32k {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Osc32k {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.en32k() != 0 { try!(write!(f, " en32k"))}
      if self.en1k() != 0 { try!(write!(f, " en1k"))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      if self.ondemand() != 0 { try!(write!(f, " ondemand"))}
      if self.startup() != 0 { try!(write!(f, " startup=0x{:x}", self.startup()))}
      if self.wrtlock() != 0 { try!(write!(f, " wrtlock"))}
      if self.calib() != 0 { try!(write!(f, " calib=0x{:x}", self.calib()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Power and Clocks Status"]
#[derive(PartialEq, Eq)]
pub struct Pclksr(pub u32);
impl Pclksr {
#[doc="XOSC Ready"]
  #[inline] pub fn xoscrdy(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="XOSC Ready"]
  #[inline] pub fn set_xoscrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="XOSC32K Ready"]
  #[inline] pub fn xosc32krdy(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="XOSC32K Ready"]
  #[inline] pub fn set_xosc32krdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="OSC32K Ready"]
  #[inline] pub fn osc32krdy(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="OSC32K Ready"]
  #[inline] pub fn set_osc32krdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="OSC8M Ready"]
  #[inline] pub fn osc8mrdy(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="OSC8M Ready"]
  #[inline] pub fn set_osc8mrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="DFLL Ready"]
  #[inline] pub fn dfllrdy(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="DFLL Ready"]
  #[inline] pub fn set_dfllrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="DFLL Out Of Bounds"]
  #[inline] pub fn dflloob(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="DFLL Out Of Bounds"]
  #[inline] pub fn set_dflloob(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="DFLL Lock Fine"]
  #[inline] pub fn dflllckf(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="DFLL Lock Fine"]
  #[inline] pub fn set_dflllckf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="DFLL Lock Coarse"]
  #[inline] pub fn dflllckc(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="DFLL Lock Coarse"]
  #[inline] pub fn set_dflllckc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="DFLL Reference Clock Stopped"]
  #[inline] pub fn dfllrcs(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="DFLL Reference Clock Stopped"]
  #[inline] pub fn set_dfllrcs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="BOD33 Ready"]
  #[inline] pub fn bod33rdy(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="BOD33 Ready"]
  #[inline] pub fn set_bod33rdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="BOD33 Detection"]
  #[inline] pub fn bod33det(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="BOD33 Detection"]
  #[inline] pub fn set_bod33det(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="BOD33 Synchronization Ready"]
  #[inline] pub fn b33srdy(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="BOD33 Synchronization Ready"]
  #[inline] pub fn set_b33srdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="DPLL Lock Rise"]
  #[inline] pub fn dplllckr(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="DPLL Lock Rise"]
  #[inline] pub fn set_dplllckr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="DPLL Lock Fall"]
  #[inline] pub fn dplllckf(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="DPLL Lock Fall"]
  #[inline] pub fn set_dplllckf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="DPLL Lock Timeout"]
  #[inline] pub fn dplllto(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="DPLL Lock Timeout"]
  #[inline] pub fn set_dplllto(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

}
impl ::core::fmt::Display for Pclksr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pclksr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.xoscrdy() != 0 { try!(write!(f, " xoscrdy"))}
      if self.xosc32krdy() != 0 { try!(write!(f, " xosc32krdy"))}
      if self.osc32krdy() != 0 { try!(write!(f, " osc32krdy"))}
      if self.osc8mrdy() != 0 { try!(write!(f, " osc8mrdy"))}
      if self.dfllrdy() != 0 { try!(write!(f, " dfllrdy"))}
      if self.dflloob() != 0 { try!(write!(f, " dflloob"))}
      if self.dflllckf() != 0 { try!(write!(f, " dflllckf"))}
      if self.dflllckc() != 0 { try!(write!(f, " dflllckc"))}
      if self.dfllrcs() != 0 { try!(write!(f, " dfllrcs"))}
      if self.bod33rdy() != 0 { try!(write!(f, " bod33rdy"))}
      if self.bod33det() != 0 { try!(write!(f, " bod33det"))}
      if self.b33srdy() != 0 { try!(write!(f, " b33srdy"))}
      if self.dplllckr() != 0 { try!(write!(f, " dplllckr"))}
      if self.dplllckf() != 0 { try!(write!(f, " dplllckf"))}
      if self.dplllto() != 0 { try!(write!(f, " dplllto"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Voltage References System (VREF) Control"]
#[derive(PartialEq, Eq)]
pub struct Vref(pub u32);
impl Vref {
#[doc="Temperature Sensor Enable"]
  #[inline] pub fn tsen(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Temperature Sensor Enable"]
  #[inline] pub fn set_tsen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Bandgap Output Enable"]
  #[inline] pub fn bgouten(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Bandgap Output Enable"]
  #[inline] pub fn set_bgouten(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Bandgap Voltage Generator Calibration"]
  #[inline] pub fn calib(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x7ff // [26:16]
  }
#[doc="Bandgap Voltage Generator Calibration"]
  #[inline] pub fn set_calib(mut self, value: u32) -> Self {
     assert!((value & !0x7ff) == 0);
     self.0 &= !(0x7ff << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Vref {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Vref {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tsen() != 0 { try!(write!(f, " tsen"))}
      if self.bgouten() != 0 { try!(write!(f, " bgouten"))}
      if self.calib() != 0 { try!(write!(f, " calib=0x{:x}", self.calib()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="External Multipurpose Crystal Oscillator (XOSC) Control"]
#[derive(PartialEq, Eq)]
pub struct Xosc(pub u16);
impl Xosc {
#[doc="Oscillator Enable"]
  #[inline] pub fn enable(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
#[doc="Oscillator Enable"]
  #[inline] pub fn set_enable(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Crystal Oscillator Enable"]
  #[inline] pub fn xtalen(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
#[doc="Crystal Oscillator Enable"]
  #[inline] pub fn set_xtalen(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Run in Standby"]
  #[inline] pub fn runstdby(&self) -> u16 {
     ((self.0 as u16) >> 6) & 0x1 // [6]
  }
#[doc="Run in Standby"]
  #[inline] pub fn set_runstdby(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="On Demand Control"]
  #[inline] pub fn ondemand(&self) -> u16 {
     ((self.0 as u16) >> 7) & 0x1 // [7]
  }
#[doc="On Demand Control"]
  #[inline] pub fn set_ondemand(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Oscillator Gain"]
  #[inline] pub fn gain(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x7 // [10:8]
  }
#[doc="Oscillator Gain"]
  #[inline] pub fn set_gain(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Automatic Amplitude Gain Control"]
  #[inline] pub fn ampgc(&self) -> u16 {
     ((self.0 as u16) >> 11) & 0x1 // [11]
  }
#[doc="Automatic Amplitude Gain Control"]
  #[inline] pub fn set_ampgc(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Start-Up Time"]
  #[inline] pub fn startup(&self) -> u16 {
     ((self.0 as u16) >> 12) & 0xf // [15:12]
  }
#[doc="Start-Up Time"]
  #[inline] pub fn set_startup(mut self, value: u16) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

}
impl ::core::fmt::Display for Xosc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Xosc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.xtalen() != 0 { try!(write!(f, " xtalen"))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      if self.ondemand() != 0 { try!(write!(f, " ondemand"))}
      if self.gain() != 0 { try!(write!(f, " gain=0x{:x}", self.gain()))}
      if self.ampgc() != 0 { try!(write!(f, " ampgc"))}
      if self.startup() != 0 { try!(write!(f, " startup=0x{:x}", self.startup()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="32kHz External Crystal Oscillator (XOSC32K) Control"]
#[derive(PartialEq, Eq)]
pub struct Xosc32k(pub u16);
impl Xosc32k {
#[doc="Oscillator Enable"]
  #[inline] pub fn enable(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
#[doc="Oscillator Enable"]
  #[inline] pub fn set_enable(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Crystal Oscillator Enable"]
  #[inline] pub fn xtalen(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
#[doc="Crystal Oscillator Enable"]
  #[inline] pub fn set_xtalen(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="32kHz Output Enable"]
  #[inline] pub fn en32k(&self) -> u16 {
     ((self.0 as u16) >> 3) & 0x1 // [3]
  }
#[doc="32kHz Output Enable"]
  #[inline] pub fn set_en32k(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="1kHz Output Enable"]
  #[inline] pub fn en1k(&self) -> u16 {
     ((self.0 as u16) >> 4) & 0x1 // [4]
  }
#[doc="1kHz Output Enable"]
  #[inline] pub fn set_en1k(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Automatic Amplitude Control Enable"]
  #[inline] pub fn aampen(&self) -> u16 {
     ((self.0 as u16) >> 5) & 0x1 // [5]
  }
#[doc="Automatic Amplitude Control Enable"]
  #[inline] pub fn set_aampen(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Run in Standby"]
  #[inline] pub fn runstdby(&self) -> u16 {
     ((self.0 as u16) >> 6) & 0x1 // [6]
  }
#[doc="Run in Standby"]
  #[inline] pub fn set_runstdby(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="On Demand Control"]
  #[inline] pub fn ondemand(&self) -> u16 {
     ((self.0 as u16) >> 7) & 0x1 // [7]
  }
#[doc="On Demand Control"]
  #[inline] pub fn set_ondemand(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Oscillator Start-Up Time"]
  #[inline] pub fn startup(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x7 // [10:8]
  }
#[doc="Oscillator Start-Up Time"]
  #[inline] pub fn set_startup(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Write Lock"]
  #[inline] pub fn wrtlock(&self) -> u16 {
     ((self.0 as u16) >> 12) & 0x1 // [12]
  }
#[doc="Write Lock"]
  #[inline] pub fn set_wrtlock(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

}
impl ::core::fmt::Display for Xosc32k {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Xosc32k {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.xtalen() != 0 { try!(write!(f, " xtalen"))}
      if self.en32k() != 0 { try!(write!(f, " en32k"))}
      if self.en1k() != 0 { try!(write!(f, " en1k"))}
      if self.aampen() != 0 { try!(write!(f, " aampen"))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      if self.ondemand() != 0 { try!(write!(f, " ondemand"))}
      if self.startup() != 0 { try!(write!(f, " startup=0x{:x}", self.startup()))}
      if self.wrtlock() != 0 { try!(write!(f, " wrtlock"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

