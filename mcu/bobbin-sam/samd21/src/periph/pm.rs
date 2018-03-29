#[allow(unused_imports)] use ::bobbin_common::*;

#[doc="Power Manager"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PmPeriph(pub usize);
impl PmPeriph {
    #[doc="Get the *mut pointer for the AHBMASK register."]
    #[inline] pub fn ahbmask_mut(&self) -> *mut Ahbmask { 
        (self.0 + 0x14) as *mut Ahbmask
    }

    #[doc="Get the *const pointer for the AHBMASK register."]
    #[inline] pub fn ahbmask_ptr(&self) -> *const Ahbmask { 
           self.ahbmask_mut()
    }

    #[doc="Read the AHBMASK register."]
    #[inline] pub fn ahbmask(&self) -> Ahbmask { 
        unsafe {
            read_volatile(self.ahbmask_ptr())
        }
    }

    #[doc="Write the AHBMASK register."]
    #[inline] pub fn set_ahbmask<F: FnOnce(Ahbmask) -> Ahbmask>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahbmask_mut(), f(Ahbmask(0)));
        }
        self
    }

    #[doc="Modify the AHBMASK register."]
    #[inline] pub fn with_ahbmask<F: FnOnce(Ahbmask) -> Ahbmask>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahbmask_mut(), f(self.ahbmask()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APBAMASK register."]
    #[inline] pub fn apbamask_mut(&self) -> *mut Apbamask { 
        (self.0 + 0x18) as *mut Apbamask
    }

    #[doc="Get the *const pointer for the APBAMASK register."]
    #[inline] pub fn apbamask_ptr(&self) -> *const Apbamask { 
           self.apbamask_mut()
    }

    #[doc="Read the APBAMASK register."]
    #[inline] pub fn apbamask(&self) -> Apbamask { 
        unsafe {
            read_volatile(self.apbamask_ptr())
        }
    }

    #[doc="Write the APBAMASK register."]
    #[inline] pub fn set_apbamask<F: FnOnce(Apbamask) -> Apbamask>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apbamask_mut(), f(Apbamask(0)));
        }
        self
    }

    #[doc="Modify the APBAMASK register."]
    #[inline] pub fn with_apbamask<F: FnOnce(Apbamask) -> Apbamask>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apbamask_mut(), f(self.apbamask()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APBASEL register."]
    #[inline] pub fn apbasel_mut(&self) -> *mut Apbasel { 
        (self.0 + 0x9) as *mut Apbasel
    }

    #[doc="Get the *const pointer for the APBASEL register."]
    #[inline] pub fn apbasel_ptr(&self) -> *const Apbasel { 
           self.apbasel_mut()
    }

    #[doc="Read the APBASEL register."]
    #[inline] pub fn apbasel(&self) -> Apbasel { 
        unsafe {
            read_volatile(self.apbasel_ptr())
        }
    }

    #[doc="Write the APBASEL register."]
    #[inline] pub fn set_apbasel<F: FnOnce(Apbasel) -> Apbasel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apbasel_mut(), f(Apbasel(0)));
        }
        self
    }

    #[doc="Modify the APBASEL register."]
    #[inline] pub fn with_apbasel<F: FnOnce(Apbasel) -> Apbasel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apbasel_mut(), f(self.apbasel()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APBBMASK register."]
    #[inline] pub fn apbbmask_mut(&self) -> *mut Apbbmask { 
        (self.0 + 0x1c) as *mut Apbbmask
    }

    #[doc="Get the *const pointer for the APBBMASK register."]
    #[inline] pub fn apbbmask_ptr(&self) -> *const Apbbmask { 
           self.apbbmask_mut()
    }

    #[doc="Read the APBBMASK register."]
    #[inline] pub fn apbbmask(&self) -> Apbbmask { 
        unsafe {
            read_volatile(self.apbbmask_ptr())
        }
    }

    #[doc="Write the APBBMASK register."]
    #[inline] pub fn set_apbbmask<F: FnOnce(Apbbmask) -> Apbbmask>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apbbmask_mut(), f(Apbbmask(0)));
        }
        self
    }

    #[doc="Modify the APBBMASK register."]
    #[inline] pub fn with_apbbmask<F: FnOnce(Apbbmask) -> Apbbmask>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apbbmask_mut(), f(self.apbbmask()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APBBSEL register."]
    #[inline] pub fn apbbsel_mut(&self) -> *mut Apbbsel { 
        (self.0 + 0xa) as *mut Apbbsel
    }

    #[doc="Get the *const pointer for the APBBSEL register."]
    #[inline] pub fn apbbsel_ptr(&self) -> *const Apbbsel { 
           self.apbbsel_mut()
    }

    #[doc="Read the APBBSEL register."]
    #[inline] pub fn apbbsel(&self) -> Apbbsel { 
        unsafe {
            read_volatile(self.apbbsel_ptr())
        }
    }

    #[doc="Write the APBBSEL register."]
    #[inline] pub fn set_apbbsel<F: FnOnce(Apbbsel) -> Apbbsel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apbbsel_mut(), f(Apbbsel(0)));
        }
        self
    }

    #[doc="Modify the APBBSEL register."]
    #[inline] pub fn with_apbbsel<F: FnOnce(Apbbsel) -> Apbbsel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apbbsel_mut(), f(self.apbbsel()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APBCMASK register."]
    #[inline] pub fn apbcmask_mut(&self) -> *mut Apbcmask { 
        (self.0 + 0x20) as *mut Apbcmask
    }

    #[doc="Get the *const pointer for the APBCMASK register."]
    #[inline] pub fn apbcmask_ptr(&self) -> *const Apbcmask { 
           self.apbcmask_mut()
    }

    #[doc="Read the APBCMASK register."]
    #[inline] pub fn apbcmask(&self) -> Apbcmask { 
        unsafe {
            read_volatile(self.apbcmask_ptr())
        }
    }

    #[doc="Write the APBCMASK register."]
    #[inline] pub fn set_apbcmask<F: FnOnce(Apbcmask) -> Apbcmask>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apbcmask_mut(), f(Apbcmask(0)));
        }
        self
    }

    #[doc="Modify the APBCMASK register."]
    #[inline] pub fn with_apbcmask<F: FnOnce(Apbcmask) -> Apbcmask>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apbcmask_mut(), f(self.apbcmask()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APBCSEL register."]
    #[inline] pub fn apbcsel_mut(&self) -> *mut Apbcsel { 
        (self.0 + 0xb) as *mut Apbcsel
    }

    #[doc="Get the *const pointer for the APBCSEL register."]
    #[inline] pub fn apbcsel_ptr(&self) -> *const Apbcsel { 
           self.apbcsel_mut()
    }

    #[doc="Read the APBCSEL register."]
    #[inline] pub fn apbcsel(&self) -> Apbcsel { 
        unsafe {
            read_volatile(self.apbcsel_ptr())
        }
    }

    #[doc="Write the APBCSEL register."]
    #[inline] pub fn set_apbcsel<F: FnOnce(Apbcsel) -> Apbcsel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apbcsel_mut(), f(Apbcsel(0)));
        }
        self
    }

    #[doc="Modify the APBCSEL register."]
    #[inline] pub fn with_apbcsel<F: FnOnce(Apbcsel) -> Apbcsel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apbcsel_mut(), f(self.apbcsel()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CPUSEL register."]
    #[inline] pub fn cpusel_mut(&self) -> *mut Cpusel { 
        (self.0 + 0x8) as *mut Cpusel
    }

    #[doc="Get the *const pointer for the CPUSEL register."]
    #[inline] pub fn cpusel_ptr(&self) -> *const Cpusel { 
           self.cpusel_mut()
    }

    #[doc="Read the CPUSEL register."]
    #[inline] pub fn cpusel(&self) -> Cpusel { 
        unsafe {
            read_volatile(self.cpusel_ptr())
        }
    }

    #[doc="Write the CPUSEL register."]
    #[inline] pub fn set_cpusel<F: FnOnce(Cpusel) -> Cpusel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cpusel_mut(), f(Cpusel(0)));
        }
        self
    }

    #[doc="Modify the CPUSEL register."]
    #[inline] pub fn with_cpusel<F: FnOnce(Cpusel) -> Cpusel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cpusel_mut(), f(self.cpusel()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTRL register."]
    #[inline] pub fn ctrl_mut(&self) -> *mut Ctrl { 
        (self.0 + 0x0) as *mut Ctrl
    }

    #[doc="Get the *const pointer for the CTRL register."]
    #[inline] pub fn ctrl_ptr(&self) -> *const Ctrl { 
           self.ctrl_mut()
    }

    #[doc="Read the CTRL register."]
    #[inline] pub fn ctrl(&self) -> Ctrl { 
        unsafe {
            read_volatile(self.ctrl_ptr())
        }
    }

    #[doc="Write the CTRL register."]
    #[inline] pub fn set_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrl_mut(), f(Ctrl(0)));
        }
        self
    }

    #[doc="Modify the CTRL register."]
    #[inline] pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrl_mut(), f(self.ctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTENCLR register."]
    #[inline] pub fn intenclr_mut(&self) -> *mut Intenclr { 
        (self.0 + 0x34) as *mut Intenclr
    }

    #[doc="Get the *const pointer for the INTENCLR register."]
    #[inline] pub fn intenclr_ptr(&self) -> *const Intenclr { 
           self.intenclr_mut()
    }

    #[doc="Read the INTENCLR register."]
    #[inline] pub fn intenclr(&self) -> Intenclr { 
        unsafe {
            read_volatile(self.intenclr_ptr())
        }
    }

    #[doc="Write the INTENCLR register."]
    #[inline] pub fn set_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intenclr_mut(), f(Intenclr(0)));
        }
        self
    }

    #[doc="Modify the INTENCLR register."]
    #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intenclr_mut(), f(self.intenclr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTENSET register."]
    #[inline] pub fn intenset_mut(&self) -> *mut Intenset { 
        (self.0 + 0x35) as *mut Intenset
    }

    #[doc="Get the *const pointer for the INTENSET register."]
    #[inline] pub fn intenset_ptr(&self) -> *const Intenset { 
           self.intenset_mut()
    }

    #[doc="Read the INTENSET register."]
    #[inline] pub fn intenset(&self) -> Intenset { 
        unsafe {
            read_volatile(self.intenset_ptr())
        }
    }

    #[doc="Write the INTENSET register."]
    #[inline] pub fn set_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intenset_mut(), f(Intenset(0)));
        }
        self
    }

    #[doc="Modify the INTENSET register."]
    #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intenset_mut(), f(self.intenset()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTFLAG register."]
    #[inline] pub fn intflag_mut(&self) -> *mut Intflag { 
        (self.0 + 0x36) as *mut Intflag
    }

    #[doc="Get the *const pointer for the INTFLAG register."]
    #[inline] pub fn intflag_ptr(&self) -> *const Intflag { 
           self.intflag_mut()
    }

    #[doc="Read the INTFLAG register."]
    #[inline] pub fn intflag(&self) -> Intflag { 
        unsafe {
            read_volatile(self.intflag_ptr())
        }
    }

    #[doc="Write the INTFLAG register."]
    #[inline] pub fn set_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intflag_mut(), f(Intflag(0)));
        }
        self
    }

    #[doc="Modify the INTFLAG register."]
    #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intflag_mut(), f(self.intflag()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RCAUSE register."]
    #[inline] pub fn rcause_mut(&self) -> *mut Rcause { 
        (self.0 + 0x38) as *mut Rcause
    }

    #[doc="Get the *const pointer for the RCAUSE register."]
    #[inline] pub fn rcause_ptr(&self) -> *const Rcause { 
           self.rcause_mut()
    }

    #[doc="Read the RCAUSE register."]
    #[inline] pub fn rcause(&self) -> Rcause { 
        unsafe {
            read_volatile(self.rcause_ptr())
        }
    }

    #[doc="Get the *mut pointer for the SLEEP register."]
    #[inline] pub fn sleep_mut(&self) -> *mut Sleep { 
        (self.0 + 0x1) as *mut Sleep
    }

    #[doc="Get the *const pointer for the SLEEP register."]
    #[inline] pub fn sleep_ptr(&self) -> *const Sleep { 
           self.sleep_mut()
    }

    #[doc="Read the SLEEP register."]
    #[inline] pub fn sleep(&self) -> Sleep { 
        unsafe {
            read_volatile(self.sleep_ptr())
        }
    }

    #[doc="Write the SLEEP register."]
    #[inline] pub fn set_sleep<F: FnOnce(Sleep) -> Sleep>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sleep_mut(), f(Sleep(0)));
        }
        self
    }

    #[doc="Modify the SLEEP register."]
    #[inline] pub fn with_sleep<F: FnOnce(Sleep) -> Sleep>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sleep_mut(), f(self.sleep()));
        }
        self
    }

}

#[doc="AHB Mask"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahbmask(pub u32);
impl Ahbmask {
    #[doc="HPB0 AHB Clock Enable"]
    #[inline] pub fn hpb0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HPB0 != 0"]
    #[inline] pub fn test_hpb0(&self) -> bool {
        self.hpb0() != 0
    }

    #[doc="Sets the HPB0 field."]
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

    #[doc="Returns true if HPB1 != 0"]
    #[inline] pub fn test_hpb1(&self) -> bool {
        self.hpb1() != 0
    }

    #[doc="Sets the HPB1 field."]
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

    #[doc="Returns true if HPB2 != 0"]
    #[inline] pub fn test_hpb2(&self) -> bool {
        self.hpb2() != 0
    }

    #[doc="Sets the HPB2 field."]
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

    #[doc="Returns true if DSU != 0"]
    #[inline] pub fn test_dsu(&self) -> bool {
        self.dsu() != 0
    }

    #[doc="Sets the DSU field."]
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

    #[doc="Returns true if NVMCTRL != 0"]
    #[inline] pub fn test_nvmctrl(&self) -> bool {
        self.nvmctrl() != 0
    }

    #[doc="Sets the NVMCTRL field."]
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

    #[doc="Returns true if DMAC != 0"]
    #[inline] pub fn test_dmac(&self) -> bool {
        self.dmac() != 0
    }

    #[doc="Sets the DMAC field."]
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

    #[doc="Returns true if USB != 0"]
    #[inline] pub fn test_usb(&self) -> bool {
        self.usb() != 0
    }

    #[doc="Sets the USB field."]
    #[inline] pub fn set_usb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for Ahbmask {
    #[inline]
    fn from(other: u32) -> Self {
         Ahbmask(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apbamask(pub u32);
impl Apbamask {
    #[doc="PAC0 APB Clock Enable"]
    #[inline] pub fn pac0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAC0 != 0"]
    #[inline] pub fn test_pac0(&self) -> bool {
        self.pac0() != 0
    }

    #[doc="Sets the PAC0 field."]
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

    #[doc="Returns true if PM != 0"]
    #[inline] pub fn test_pm(&self) -> bool {
        self.pm() != 0
    }

    #[doc="Sets the PM field."]
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

    #[doc="Returns true if SYSCTRL != 0"]
    #[inline] pub fn test_sysctrl(&self) -> bool {
        self.sysctrl() != 0
    }

    #[doc="Sets the SYSCTRL field."]
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

    #[doc="Returns true if GCLK != 0"]
    #[inline] pub fn test_gclk(&self) -> bool {
        self.gclk() != 0
    }

    #[doc="Sets the GCLK field."]
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

    #[doc="Returns true if WDT != 0"]
    #[inline] pub fn test_wdt(&self) -> bool {
        self.wdt() != 0
    }

    #[doc="Sets the WDT field."]
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

    #[doc="Returns true if RTC != 0"]
    #[inline] pub fn test_rtc(&self) -> bool {
        self.rtc() != 0
    }

    #[doc="Sets the RTC field."]
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

    #[doc="Returns true if EIC != 0"]
    #[inline] pub fn test_eic(&self) -> bool {
        self.eic() != 0
    }

    #[doc="Sets the EIC field."]
    #[inline] pub fn set_eic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for Apbamask {
    #[inline]
    fn from(other: u32) -> Self {
         Apbamask(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apbasel(pub u8);
impl Apbasel {
    #[doc="APBA Prescaler Selection"]
    #[inline] pub fn apbadiv(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if APBADIV != 0"]
    #[inline] pub fn test_apbadiv(&self) -> bool {
        self.apbadiv() != 0
    }

    #[doc="Sets the APBADIV field."]
    #[inline] pub fn set_apbadiv<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Apbasel {
    #[inline]
    fn from(other: u8) -> Self {
         Apbasel(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apbbmask(pub u32);
impl Apbbmask {
    #[doc="PAC1 APB Clock Enable"]
    #[inline] pub fn pac1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAC1 != 0"]
    #[inline] pub fn test_pac1(&self) -> bool {
        self.pac1() != 0
    }

    #[doc="Sets the PAC1 field."]
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

    #[doc="Returns true if DSU != 0"]
    #[inline] pub fn test_dsu(&self) -> bool {
        self.dsu() != 0
    }

    #[doc="Sets the DSU field."]
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

    #[doc="Returns true if NVMCTRL != 0"]
    #[inline] pub fn test_nvmctrl(&self) -> bool {
        self.nvmctrl() != 0
    }

    #[doc="Sets the NVMCTRL field."]
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

    #[doc="Returns true if PORT != 0"]
    #[inline] pub fn test_port(&self) -> bool {
        self.port() != 0
    }

    #[doc="Sets the PORT field."]
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

    #[doc="Returns true if DMAC != 0"]
    #[inline] pub fn test_dmac(&self) -> bool {
        self.dmac() != 0
    }

    #[doc="Sets the DMAC field."]
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

    #[doc="Returns true if USB != 0"]
    #[inline] pub fn test_usb(&self) -> bool {
        self.usb() != 0
    }

    #[doc="Sets the USB field."]
    #[inline] pub fn set_usb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u32> for Apbbmask {
    #[inline]
    fn from(other: u32) -> Self {
         Apbbmask(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apbbsel(pub u8);
impl Apbbsel {
    #[doc="APBB Prescaler Selection"]
    #[inline] pub fn apbbdiv(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if APBBDIV != 0"]
    #[inline] pub fn test_apbbdiv(&self) -> bool {
        self.apbbdiv() != 0
    }

    #[doc="Sets the APBBDIV field."]
    #[inline] pub fn set_apbbdiv<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Apbbsel {
    #[inline]
    fn from(other: u8) -> Self {
         Apbbsel(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apbcmask(pub u32);
impl Apbcmask {
    #[doc="PAC2 APB Clock Enable"]
    #[inline] pub fn pac2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAC2 != 0"]
    #[inline] pub fn test_pac2(&self) -> bool {
        self.pac2() != 0
    }

    #[doc="Sets the PAC2 field."]
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

    #[doc="Returns true if EVSYS != 0"]
    #[inline] pub fn test_evsys(&self) -> bool {
        self.evsys() != 0
    }

    #[doc="Sets the EVSYS field."]
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

    #[doc="Returns true if SERCOM0 != 0"]
    #[inline] pub fn test_sercom0(&self) -> bool {
        self.sercom0() != 0
    }

    #[doc="Sets the SERCOM0 field."]
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

    #[doc="Returns true if SERCOM1 != 0"]
    #[inline] pub fn test_sercom1(&self) -> bool {
        self.sercom1() != 0
    }

    #[doc="Sets the SERCOM1 field."]
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

    #[doc="Returns true if SERCOM2 != 0"]
    #[inline] pub fn test_sercom2(&self) -> bool {
        self.sercom2() != 0
    }

    #[doc="Sets the SERCOM2 field."]
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

    #[doc="Returns true if SERCOM3 != 0"]
    #[inline] pub fn test_sercom3(&self) -> bool {
        self.sercom3() != 0
    }

    #[doc="Sets the SERCOM3 field."]
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

    #[doc="Returns true if SERCOM4 != 0"]
    #[inline] pub fn test_sercom4(&self) -> bool {
        self.sercom4() != 0
    }

    #[doc="Sets the SERCOM4 field."]
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

    #[doc="Returns true if SERCOM5 != 0"]
    #[inline] pub fn test_sercom5(&self) -> bool {
        self.sercom5() != 0
    }

    #[doc="Sets the SERCOM5 field."]
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

    #[doc="Returns true if TCC0 != 0"]
    #[inline] pub fn test_tcc0(&self) -> bool {
        self.tcc0() != 0
    }

    #[doc="Sets the TCC0 field."]
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

    #[doc="Returns true if TCC1 != 0"]
    #[inline] pub fn test_tcc1(&self) -> bool {
        self.tcc1() != 0
    }

    #[doc="Sets the TCC1 field."]
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

    #[doc="Returns true if TCC2 != 0"]
    #[inline] pub fn test_tcc2(&self) -> bool {
        self.tcc2() != 0
    }

    #[doc="Sets the TCC2 field."]
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

    #[doc="Returns true if TC3 != 0"]
    #[inline] pub fn test_tc3(&self) -> bool {
        self.tc3() != 0
    }

    #[doc="Sets the TC3 field."]
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

    #[doc="Returns true if TC4 != 0"]
    #[inline] pub fn test_tc4(&self) -> bool {
        self.tc4() != 0
    }

    #[doc="Sets the TC4 field."]
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

    #[doc="Returns true if TC5 != 0"]
    #[inline] pub fn test_tc5(&self) -> bool {
        self.tc5() != 0
    }

    #[doc="Sets the TC5 field."]
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

    #[doc="Returns true if ADC != 0"]
    #[inline] pub fn test_adc(&self) -> bool {
        self.adc() != 0
    }

    #[doc="Sets the ADC field."]
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

    #[doc="Returns true if AC != 0"]
    #[inline] pub fn test_ac(&self) -> bool {
        self.ac() != 0
    }

    #[doc="Sets the AC field."]
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

    #[doc="Returns true if DAC != 0"]
    #[inline] pub fn test_dac(&self) -> bool {
        self.dac() != 0
    }

    #[doc="Sets the DAC field."]
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

    #[doc="Returns true if I2S != 0"]
    #[inline] pub fn test_i2s(&self) -> bool {
        self.i2s() != 0
    }

    #[doc="Sets the I2S field."]
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

    #[doc="Returns true if ATW != 0"]
    #[inline] pub fn test_atw(&self) -> bool {
        self.atw() != 0
    }

    #[doc="Sets the ATW field."]
    #[inline] pub fn set_atw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

}

impl From<u32> for Apbcmask {
    #[inline]
    fn from(other: u32) -> Self {
         Apbcmask(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apbcsel(pub u8);
impl Apbcsel {
    #[doc="APBC Prescaler Selection"]
    #[inline] pub fn apbcdiv(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if APBCDIV != 0"]
    #[inline] pub fn test_apbcdiv(&self) -> bool {
        self.apbcdiv() != 0
    }

    #[doc="Sets the APBCDIV field."]
    #[inline] pub fn set_apbcdiv<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Apbcsel {
    #[inline]
    fn from(other: u8) -> Self {
         Apbcsel(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cpusel(pub u8);
impl Cpusel {
    #[doc="CPU Prescaler Selection"]
    #[inline] pub fn cpudiv(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if CPUDIV != 0"]
    #[inline] pub fn test_cpudiv(&self) -> bool {
        self.cpudiv() != 0
    }

    #[doc="Sets the CPUDIV field."]
    #[inline] pub fn set_cpudiv<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Cpusel {
    #[inline]
    fn from(other: u8) -> Self {
         Cpusel(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u8);
impl Ctrl {
    #[doc="Clock Failure Detector Enable"]
    #[inline] pub fn cfden(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CFDEN != 0"]
    #[inline] pub fn test_cfden(&self) -> bool {
        self.cfden() != 0
    }

    #[doc="Sets the CFDEN field."]
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

    #[doc="Returns true if BKUPCLK != 0"]
    #[inline] pub fn test_bkupclk(&self) -> bool {
        self.bkupclk() != 0
    }

    #[doc="Sets the BKUPCLK field."]
    #[inline] pub fn set_bkupclk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u8> for Ctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Ctrl(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
    #[doc="Clock Ready Interrupt Enable"]
    #[inline] pub fn ckrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CKRDY != 0"]
    #[inline] pub fn test_ckrdy(&self) -> bool {
        self.ckrdy() != 0
    }

    #[doc="Sets the CKRDY field."]
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

    #[doc="Returns true if CFD != 0"]
    #[inline] pub fn test_cfd(&self) -> bool {
        self.cfd() != 0
    }

    #[doc="Sets the CFD field."]
    #[inline] pub fn set_cfd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Intenclr {
    #[inline]
    fn from(other: u8) -> Self {
         Intenclr(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
    #[doc="Clock Ready Interrupt Enable"]
    #[inline] pub fn ckrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CKRDY != 0"]
    #[inline] pub fn test_ckrdy(&self) -> bool {
        self.ckrdy() != 0
    }

    #[doc="Sets the CKRDY field."]
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

    #[doc="Returns true if CFD != 0"]
    #[inline] pub fn test_cfd(&self) -> bool {
        self.cfd() != 0
    }

    #[doc="Sets the CFD field."]
    #[inline] pub fn set_cfd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Intenset {
    #[inline]
    fn from(other: u8) -> Self {
         Intenset(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
    #[doc="Clock Ready"]
    #[inline] pub fn ckrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CKRDY != 0"]
    #[inline] pub fn test_ckrdy(&self) -> bool {
        self.ckrdy() != 0
    }

    #[doc="Sets the CKRDY field."]
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

    #[doc="Returns true if CFD != 0"]
    #[inline] pub fn test_cfd(&self) -> bool {
        self.cfd() != 0
    }

    #[doc="Sets the CFD field."]
    #[inline] pub fn set_cfd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Intflag {
    #[inline]
    fn from(other: u8) -> Self {
         Intflag(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rcause(pub u8);
impl Rcause {
    #[doc="Power On Reset"]
    #[inline] pub fn por(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if POR != 0"]
    #[inline] pub fn test_por(&self) -> bool {
        self.por() != 0
    }

    #[doc="Sets the POR field."]
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

    #[doc="Returns true if BOD12 != 0"]
    #[inline] pub fn test_bod12(&self) -> bool {
        self.bod12() != 0
    }

    #[doc="Sets the BOD12 field."]
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

    #[doc="Returns true if BOD33 != 0"]
    #[inline] pub fn test_bod33(&self) -> bool {
        self.bod33() != 0
    }

    #[doc="Sets the BOD33 field."]
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

    #[doc="Returns true if EXT != 0"]
    #[inline] pub fn test_ext(&self) -> bool {
        self.ext() != 0
    }

    #[doc="Sets the EXT field."]
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

    #[doc="Returns true if WDT != 0"]
    #[inline] pub fn test_wdt(&self) -> bool {
        self.wdt() != 0
    }

    #[doc="Sets the WDT field."]
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

    #[doc="Returns true if SYST != 0"]
    #[inline] pub fn test_syst(&self) -> bool {
        self.syst() != 0
    }

    #[doc="Sets the SYST field."]
    #[inline] pub fn set_syst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u8> for Rcause {
    #[inline]
    fn from(other: u8) -> Self {
         Rcause(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sleep(pub u8);
impl Sleep {
    #[doc="Idle Mode Configuration"]
    #[inline] pub fn idle(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if IDLE != 0"]
    #[inline] pub fn test_idle(&self) -> bool {
        self.idle() != 0
    }

    #[doc="Sets the IDLE field."]
    #[inline] pub fn set_idle<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Sleep {
    #[inline]
    fn from(other: u8) -> Self {
         Sleep(other)
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

