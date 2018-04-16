::bobbin_mcu::periph!( USB_FS_PWRCLK, UsbFsPwrclk, USB_FS_PWRCLK_PERIPH, UsbFsPwrclkPeriph, USB_FS_PWRCLK_OWNED, USB_FS_PWRCLK_REF_COUNT, 0x50000e00, 0x00, 0x17);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="USB_FS_PWRCLK Peripheral"]
pub struct UsbFsPwrclkPeriph(pub usize); 

impl UsbFsPwrclkPeriph {
    #[doc="Get the PCGCCTL Register."]
    #[inline] pub fn pcgcctl_reg(&self) -> ::bobbin_mcu::register::Register<Pcgcctl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pcgcctl, 0x0)
    }

    #[doc="Get the *mut pointer for the PCGCCTL register."]
    #[inline] pub fn pcgcctl_mut(&self) -> *mut Pcgcctl { 
        self.pcgcctl_reg().ptr()
    }

    #[doc="Get the *const pointer for the PCGCCTL register."]
    #[inline] pub fn pcgcctl_ptr(&self) -> *const Pcgcctl { 
        self.pcgcctl_reg().ptr()
    }

    #[doc="Read the PCGCCTL register."]
    #[inline] pub fn pcgcctl(&self) -> Pcgcctl { 
        self.pcgcctl_reg().read()
    }

    #[doc="Write the PCGCCTL register."]
    #[inline] pub fn write_pcgcctl(&self, value: Pcgcctl) -> &Self { 
        self.pcgcctl_reg().write(value);
        self
    }

    #[doc="Set the PCGCCTL register."]
    #[inline] pub fn set_pcgcctl<F: FnOnce(Pcgcctl) -> Pcgcctl>(&self, f: F) -> &Self {
        self.pcgcctl_reg().set(f);
        self
    }

    #[doc="Modify the PCGCCTL register."]
    #[inline] pub fn with_pcgcctl<F: FnOnce(Pcgcctl) -> Pcgcctl>(&self, f: F) -> &Self {
        self.pcgcctl_reg().with(f);
        self
    }

}

#[doc="OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcgcctl(pub u32);
impl Pcgcctl {
    #[doc="Stop PHY clock"]
    #[inline] pub fn stppclk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if STPPCLK != 0"]
    #[inline] pub fn test_stppclk(&self) -> bool {
        self.stppclk() != 0
    }

    #[doc="Sets the STPPCLK field."]
    #[inline] pub fn set_stppclk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Gate HCLK"]
    #[inline] pub fn gatehclk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GATEHCLK != 0"]
    #[inline] pub fn test_gatehclk(&self) -> bool {
        self.gatehclk() != 0
    }

    #[doc="Sets the GATEHCLK field."]
    #[inline] pub fn set_gatehclk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="PHY Suspended"]
    #[inline] pub fn physusp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PHYSUSP != 0"]
    #[inline] pub fn test_physusp(&self) -> bool {
        self.physusp() != 0
    }

    #[doc="Sets the PHYSUSP field."]
    #[inline] pub fn set_physusp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
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

