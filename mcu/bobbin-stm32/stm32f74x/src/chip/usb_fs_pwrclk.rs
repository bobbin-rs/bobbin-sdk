#[allow(unused_imports)] use bobbin_common::*;

periph!( USB_FS_PWRCLK, UsbFsPwrclk, _USB_FS_PWRCLK, UsbFsPwrclkPeriph, 0x50000e00);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="USB_FS_PWRCLK Peripheral"]
pub struct UsbFsPwrclkPeriph(pub usize); 



impl UsbFsPwrclkPeriph {
    #[doc="Get the *mut pointer for the PCGCCTL register."]
    #[inline] pub fn pcgcctl_mut(&self) -> *mut Pcgcctl { 
        (self.0 + 0x0) as *mut Pcgcctl
    }

    #[doc="Get the *const pointer for the PCGCCTL register."]
    #[inline] pub fn pcgcctl_ptr(&self) -> *const Pcgcctl { 
           self.pcgcctl_mut()
    }

    #[doc="Read the PCGCCTL register."]
    #[inline] pub fn pcgcctl(&self) -> Pcgcctl { 
        unsafe {
            read_volatile(self.pcgcctl_ptr())
        }
    }

    #[doc="Write the PCGCCTL register."]
    #[inline] pub fn set_pcgcctl<F: FnOnce(Pcgcctl) -> Pcgcctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pcgcctl_mut(), f(Pcgcctl(0)));
        }
        self
    }

    #[doc="Modify the PCGCCTL register."]
    #[inline] pub fn with_pcgcctl<F: FnOnce(Pcgcctl) -> Pcgcctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pcgcctl_mut(), f(self.pcgcctl()));
        }
        self
    }

}

#[doc="OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcgcctl(pub u32);
impl Pcgcctl {
    #[doc="Stop PHY clock"]
    #[inline] pub fn stppclk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if STPPCLK != 0"]
    #[inline] pub fn test_stppclk(&self) -> bool {
        self.stppclk() != 0
    }

    #[doc="Sets the STPPCLK field."]
    #[inline] pub fn set_stppclk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Gate HCLK"]
    #[inline] pub fn gatehclk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GATEHCLK != 0"]
    #[inline] pub fn test_gatehclk(&self) -> bool {
        self.gatehclk() != 0
    }

    #[doc="Sets the GATEHCLK field."]
    #[inline] pub fn set_gatehclk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="PHY Suspended"]
    #[inline] pub fn physusp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PHYSUSP != 0"]
    #[inline] pub fn test_physusp(&self) -> bool {
        self.physusp() != 0
    }

    #[doc="Sets the PHYSUSP field."]
    #[inline] pub fn set_physusp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Pcgcctl {
    #[inline]
    fn from(other: u32) -> Self {
         Pcgcctl(other)
    }
}

impl ::core::fmt::Display for Pcgcctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcgcctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.stppclk() != 0 { try!(write!(f, " stppclk"))}
        if self.gatehclk() != 0 { try!(write!(f, " gatehclk"))}
        if self.physusp() != 0 { try!(write!(f, " physusp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}


