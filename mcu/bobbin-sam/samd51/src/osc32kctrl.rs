::bobbin_mcu::periph!( OSC32KCTRL, Osc32kctrl, OSC32KCTRL_PERIPH, Osc32kctrlPeriph, OSC32KCTRL_OWNED, OSC32KCTRL_REF_COUNT, 0x40001400, 0x00, 0x13);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="OSC32KCTRL Peripheral"]
pub struct Osc32kctrlPeriph(pub usize); 

impl Osc32kctrlPeriph {
    #[doc="Get the INTENCLR Register."]
    #[inline] pub fn intenclr_reg(&self) -> ::bobbin_mcu::register::Register<Intenclr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenclr, 0x0)
    }

    #[doc="Get the *mut pointer for the INTENCLR register."]
    #[inline] pub fn intenclr_mut(&self) -> *mut Intenclr { 
        self.intenclr_reg().ptr()
    }

    #[doc="Get the *const pointer for the INTENCLR register."]
    #[inline] pub fn intenclr_ptr(&self) -> *const Intenclr { 
        self.intenclr_reg().ptr()
    }

    #[doc="Read the INTENCLR register."]
    #[inline] pub fn intenclr(&self) -> Intenclr { 
        self.intenclr_reg().read()
    }

    #[doc="Write the INTENCLR register."]
    #[inline] pub fn write_intenclr(&self, value: Intenclr) -> &Self { 
        self.intenclr_reg().write(value);
        self
    }

    #[doc="Set the INTENCLR register."]
    #[inline] pub fn set_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
        self.intenclr_reg().set(f);
        self
    }

    #[doc="Modify the INTENCLR register."]
    #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
        self.intenclr_reg().with(f);
        self
    }

    #[doc="Get the INTENSET Register."]
    #[inline] pub fn intenset_reg(&self) -> ::bobbin_mcu::register::Register<Intenset> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenset, 0x4)
    }

    #[doc="Get the *mut pointer for the INTENSET register."]
    #[inline] pub fn intenset_mut(&self) -> *mut Intenset { 
        self.intenset_reg().ptr()
    }

    #[doc="Get the *const pointer for the INTENSET register."]
    #[inline] pub fn intenset_ptr(&self) -> *const Intenset { 
        self.intenset_reg().ptr()
    }

    #[doc="Read the INTENSET register."]
    #[inline] pub fn intenset(&self) -> Intenset { 
        self.intenset_reg().read()
    }

    #[doc="Write the INTENSET register."]
    #[inline] pub fn write_intenset(&self, value: Intenset) -> &Self { 
        self.intenset_reg().write(value);
        self
    }

    #[doc="Set the INTENSET register."]
    #[inline] pub fn set_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
        self.intenset_reg().set(f);
        self
    }

    #[doc="Modify the INTENSET register."]
    #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
        self.intenset_reg().with(f);
        self
    }

    #[doc="Get the INTFLAG Register."]
    #[inline] pub fn intflag_reg(&self) -> ::bobbin_mcu::register::Register<Intflag> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflag, 0x8)
    }

    #[doc="Get the *mut pointer for the INTFLAG register."]
    #[inline] pub fn intflag_mut(&self) -> *mut Intflag { 
        self.intflag_reg().ptr()
    }

    #[doc="Get the *const pointer for the INTFLAG register."]
    #[inline] pub fn intflag_ptr(&self) -> *const Intflag { 
        self.intflag_reg().ptr()
    }

    #[doc="Read the INTFLAG register."]
    #[inline] pub fn intflag(&self) -> Intflag { 
        self.intflag_reg().read()
    }

    #[doc="Write the INTFLAG register."]
    #[inline] pub fn write_intflag(&self, value: Intflag) -> &Self { 
        self.intflag_reg().write(value);
        self
    }

    #[doc="Set the INTFLAG register."]
    #[inline] pub fn set_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
        self.intflag_reg().set(f);
        self
    }

    #[doc="Modify the INTFLAG register."]
    #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
        self.intflag_reg().with(f);
        self
    }

    #[doc="Get the STATUS Register."]
    #[inline] pub fn status_reg(&self) -> ::bobbin_mcu::register::Register<Status> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Status, 0xc)
    }

    #[doc="Get the *mut pointer for the STATUS register."]
    #[inline] pub fn status_mut(&self) -> *mut Status { 
        self.status_reg().ptr()
    }

    #[doc="Get the *const pointer for the STATUS register."]
    #[inline] pub fn status_ptr(&self) -> *const Status { 
        self.status_reg().ptr()
    }

    #[doc="Read the STATUS register."]
    #[inline] pub fn status(&self) -> Status { 
        self.status_reg().read()
    }

    #[doc="Get the RTCCTRL Register."]
    #[inline] pub fn rtcctrl_reg(&self) -> ::bobbin_mcu::register::Register<Rtcctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rtcctrl, 0x10)
    }

    #[doc="Get the *mut pointer for the RTCCTRL register."]
    #[inline] pub fn rtcctrl_mut(&self) -> *mut Rtcctrl { 
        self.rtcctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the RTCCTRL register."]
    #[inline] pub fn rtcctrl_ptr(&self) -> *const Rtcctrl { 
        self.rtcctrl_reg().ptr()
    }

    #[doc="Read the RTCCTRL register."]
    #[inline] pub fn rtcctrl(&self) -> Rtcctrl { 
        self.rtcctrl_reg().read()
    }

    #[doc="Write the RTCCTRL register."]
    #[inline] pub fn write_rtcctrl(&self, value: Rtcctrl) -> &Self { 
        self.rtcctrl_reg().write(value);
        self
    }

    #[doc="Set the RTCCTRL register."]
    #[inline] pub fn set_rtcctrl<F: FnOnce(Rtcctrl) -> Rtcctrl>(&self, f: F) -> &Self {
        self.rtcctrl_reg().set(f);
        self
    }

    #[doc="Modify the RTCCTRL register."]
    #[inline] pub fn with_rtcctrl<F: FnOnce(Rtcctrl) -> Rtcctrl>(&self, f: F) -> &Self {
        self.rtcctrl_reg().with(f);
        self
    }

    #[doc="Get the XOSC32K Register."]
    #[inline] pub fn xosc32k_reg(&self) -> ::bobbin_mcu::register::Register<Xosc32k> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Xosc32k, 0x14)
    }

    #[doc="Get the *mut pointer for the XOSC32K register."]
    #[inline] pub fn xosc32k_mut(&self) -> *mut Xosc32k { 
        self.xosc32k_reg().ptr()
    }

    #[doc="Get the *const pointer for the XOSC32K register."]
    #[inline] pub fn xosc32k_ptr(&self) -> *const Xosc32k { 
        self.xosc32k_reg().ptr()
    }

    #[doc="Read the XOSC32K register."]
    #[inline] pub fn xosc32k(&self) -> Xosc32k { 
        self.xosc32k_reg().read()
    }

    #[doc="Write the XOSC32K register."]
    #[inline] pub fn write_xosc32k(&self, value: Xosc32k) -> &Self { 
        self.xosc32k_reg().write(value);
        self
    }

    #[doc="Set the XOSC32K register."]
    #[inline] pub fn set_xosc32k<F: FnOnce(Xosc32k) -> Xosc32k>(&self, f: F) -> &Self {
        self.xosc32k_reg().set(f);
        self
    }

    #[doc="Modify the XOSC32K register."]
    #[inline] pub fn with_xosc32k<F: FnOnce(Xosc32k) -> Xosc32k>(&self, f: F) -> &Self {
        self.xosc32k_reg().with(f);
        self
    }

    #[doc="Get the CFDCTRL Register."]
    #[inline] pub fn cfdctrl_reg(&self) -> ::bobbin_mcu::register::Register<Cfdctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cfdctrl, 0x16)
    }

    #[doc="Get the *mut pointer for the CFDCTRL register."]
    #[inline] pub fn cfdctrl_mut(&self) -> *mut Cfdctrl { 
        self.cfdctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the CFDCTRL register."]
    #[inline] pub fn cfdctrl_ptr(&self) -> *const Cfdctrl { 
        self.cfdctrl_reg().ptr()
    }

    #[doc="Read the CFDCTRL register."]
    #[inline] pub fn cfdctrl(&self) -> Cfdctrl { 
        self.cfdctrl_reg().read()
    }

    #[doc="Write the CFDCTRL register."]
    #[inline] pub fn write_cfdctrl(&self, value: Cfdctrl) -> &Self { 
        self.cfdctrl_reg().write(value);
        self
    }

    #[doc="Set the CFDCTRL register."]
    #[inline] pub fn set_cfdctrl<F: FnOnce(Cfdctrl) -> Cfdctrl>(&self, f: F) -> &Self {
        self.cfdctrl_reg().set(f);
        self
    }

    #[doc="Modify the CFDCTRL register."]
    #[inline] pub fn with_cfdctrl<F: FnOnce(Cfdctrl) -> Cfdctrl>(&self, f: F) -> &Self {
        self.cfdctrl_reg().with(f);
        self
    }

    #[doc="Get the EVCTRL Register."]
    #[inline] pub fn evctrl_reg(&self) -> ::bobbin_mcu::register::Register<Evctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Evctrl, 0x17)
    }

    #[doc="Get the *mut pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_mut(&self) -> *mut Evctrl { 
        self.evctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_ptr(&self) -> *const Evctrl { 
        self.evctrl_reg().ptr()
    }

    #[doc="Read the EVCTRL register."]
    #[inline] pub fn evctrl(&self) -> Evctrl { 
        self.evctrl_reg().read()
    }

    #[doc="Write the EVCTRL register."]
    #[inline] pub fn write_evctrl(&self, value: Evctrl) -> &Self { 
        self.evctrl_reg().write(value);
        self
    }

    #[doc="Set the EVCTRL register."]
    #[inline] pub fn set_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
        self.evctrl_reg().set(f);
        self
    }

    #[doc="Modify the EVCTRL register."]
    #[inline] pub fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
        self.evctrl_reg().with(f);
        self
    }

    #[doc="Get the OSCULP32K Register."]
    #[inline] pub fn osculp32k_reg(&self) -> ::bobbin_mcu::register::Register<Osculp32k> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Osculp32k, 0x1c)
    }

    #[doc="Get the *mut pointer for the OSCULP32K register."]
    #[inline] pub fn osculp32k_mut(&self) -> *mut Osculp32k { 
        self.osculp32k_reg().ptr()
    }

    #[doc="Get the *const pointer for the OSCULP32K register."]
    #[inline] pub fn osculp32k_ptr(&self) -> *const Osculp32k { 
        self.osculp32k_reg().ptr()
    }

    #[doc="Read the OSCULP32K register."]
    #[inline] pub fn osculp32k(&self) -> Osculp32k { 
        self.osculp32k_reg().read()
    }

    #[doc="Write the OSCULP32K register."]
    #[inline] pub fn write_osculp32k(&self, value: Osculp32k) -> &Self { 
        self.osculp32k_reg().write(value);
        self
    }

    #[doc="Set the OSCULP32K register."]
    #[inline] pub fn set_osculp32k<F: FnOnce(Osculp32k) -> Osculp32k>(&self, f: F) -> &Self {
        self.osculp32k_reg().set(f);
        self
    }

    #[doc="Modify the OSCULP32K register."]
    #[inline] pub fn with_osculp32k<F: FnOnce(Osculp32k) -> Osculp32k>(&self, f: F) -> &Self {
        self.osculp32k_reg().with(f);
        self
    }

}

#[doc="Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u32);
impl Intenclr {
    #[doc="XOSC32K Ready Interrupt Enable"]
    #[inline] pub fn xosc32krdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XOSC32KRDY != 0"]
    #[inline] pub fn test_xosc32krdy(&self) -> bool {
        self.xosc32krdy() != 0
    }

    #[doc="Sets the XOSC32KRDY field."]
    #[inline] pub fn set_xosc32krdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="XOSC32K Clock Failure Detector Interrupt Enable"]
    #[inline] pub fn xosc32kfail(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if XOSC32KFAIL != 0"]
    #[inline] pub fn test_xosc32kfail(&self) -> bool {
        self.xosc32kfail() != 0
    }

    #[doc="Sets the XOSC32KFAIL field."]
    #[inline] pub fn set_xosc32kfail<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u32> for Intenclr {
    #[inline]
    fn from(other: u32) -> Self {
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
        if self.xosc32krdy() != 0 { try!(write!(f, " xosc32krdy"))}
        if self.xosc32kfail() != 0 { try!(write!(f, " xosc32kfail"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u32);
impl Intenset {
    #[doc="XOSC32K Ready Interrupt Enable"]
    #[inline] pub fn xosc32krdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XOSC32KRDY != 0"]
    #[inline] pub fn test_xosc32krdy(&self) -> bool {
        self.xosc32krdy() != 0
    }

    #[doc="Sets the XOSC32KRDY field."]
    #[inline] pub fn set_xosc32krdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="XOSC32K Clock Failure Detector Interrupt Enable"]
    #[inline] pub fn xosc32kfail(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if XOSC32KFAIL != 0"]
    #[inline] pub fn test_xosc32kfail(&self) -> bool {
        self.xosc32kfail() != 0
    }

    #[doc="Sets the XOSC32KFAIL field."]
    #[inline] pub fn set_xosc32kfail<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u32> for Intenset {
    #[inline]
    fn from(other: u32) -> Self {
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
        if self.xosc32krdy() != 0 { try!(write!(f, " xosc32krdy"))}
        if self.xosc32kfail() != 0 { try!(write!(f, " xosc32kfail"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u32);
impl Intflag {
    #[doc="XOSC32K Ready"]
    #[inline] pub fn xosc32krdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XOSC32KRDY != 0"]
    #[inline] pub fn test_xosc32krdy(&self) -> bool {
        self.xosc32krdy() != 0
    }

    #[doc="Sets the XOSC32KRDY field."]
    #[inline] pub fn set_xosc32krdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="XOSC32K Clock Failure Detector"]
    #[inline] pub fn xosc32kfail(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if XOSC32KFAIL != 0"]
    #[inline] pub fn test_xosc32kfail(&self) -> bool {
        self.xosc32kfail() != 0
    }

    #[doc="Sets the XOSC32KFAIL field."]
    #[inline] pub fn set_xosc32kfail<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u32> for Intflag {
    #[inline]
    fn from(other: u32) -> Self {
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
        if self.xosc32krdy() != 0 { try!(write!(f, " xosc32krdy"))}
        if self.xosc32kfail() != 0 { try!(write!(f, " xosc32kfail"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power and Clocks Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u32);
impl Status {
    #[doc="XOSC32K Ready"]
    #[inline] pub fn xosc32krdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XOSC32KRDY != 0"]
    #[inline] pub fn test_xosc32krdy(&self) -> bool {
        self.xosc32krdy() != 0
    }

    #[doc="Sets the XOSC32KRDY field."]
    #[inline] pub fn set_xosc32krdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="XOSC32K Clock Failure Detector"]
    #[inline] pub fn xosc32kfail(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if XOSC32KFAIL != 0"]
    #[inline] pub fn test_xosc32kfail(&self) -> bool {
        self.xosc32kfail() != 0
    }

    #[doc="Sets the XOSC32KFAIL field."]
    #[inline] pub fn set_xosc32kfail<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="XOSC32K Clock switch"]
    #[inline] pub fn xosc32ksw(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if XOSC32KSW != 0"]
    #[inline] pub fn test_xosc32ksw(&self) -> bool {
        self.xosc32ksw() != 0
    }

    #[doc="Sets the XOSC32KSW field."]
    #[inline] pub fn set_xosc32ksw<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Status {
    #[inline]
    fn from(other: u32) -> Self {
         Status(other)
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
        if self.xosc32krdy() != 0 { try!(write!(f, " xosc32krdy"))}
        if self.xosc32kfail() != 0 { try!(write!(f, " xosc32kfail"))}
        if self.xosc32ksw() != 0 { try!(write!(f, " xosc32ksw"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RTC Clock Selection"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rtcctrl(pub u8);
impl Rtcctrl {
    #[doc="RTC Clock Selection"]
    #[inline] pub fn rtcsel(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if RTCSEL != 0"]
    #[inline] pub fn test_rtcsel(&self) -> bool {
        self.rtcsel() != 0
    }

    #[doc="Sets the RTCSEL field."]
    #[inline] pub fn set_rtcsel<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Rtcctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Rtcctrl(other)
    }
}

impl ::core::fmt::Display for Rtcctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rtcctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rtcsel() != 0 { try!(write!(f, " rtcsel=0x{:x}", self.rtcsel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="32kHz External Crystal Oscillator (XOSC32K) Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Xosc32k(pub u16);
impl Xosc32k {
    #[doc="Oscillator Enable"]
    #[inline] pub fn enable(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Crystal Oscillator Enable"]
    #[inline] pub fn xtalen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if XTALEN != 0"]
    #[inline] pub fn test_xtalen(&self) -> bool {
        self.xtalen() != 0
    }

    #[doc="Sets the XTALEN field."]
    #[inline] pub fn set_xtalen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="32kHz Output Enable"]
    #[inline] pub fn en32k(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if EN32K != 0"]
    #[inline] pub fn test_en32k(&self) -> bool {
        self.en32k() != 0
    }

    #[doc="Sets the EN32K field."]
    #[inline] pub fn set_en32k<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="1kHz Output Enable"]
    #[inline] pub fn en1k(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if EN1K != 0"]
    #[inline] pub fn test_en1k(&self) -> bool {
        self.en1k() != 0
    }

    #[doc="Sets the EN1K field."]
    #[inline] pub fn set_en1k<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Run in Standby"]
    #[inline] pub fn runstdby(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="On Demand Control"]
    #[inline] pub fn ondemand(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ONDEMAND != 0"]
    #[inline] pub fn test_ondemand(&self) -> bool {
        self.ondemand() != 0
    }

    #[doc="Sets the ONDEMAND field."]
    #[inline] pub fn set_ondemand<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Oscillator Start-Up Time"]
    #[inline] pub fn startup(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if STARTUP != 0"]
    #[inline] pub fn test_startup(&self) -> bool {
        self.startup() != 0
    }

    #[doc="Sets the STARTUP field."]
    #[inline] pub fn set_startup<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Write Lock"]
    #[inline] pub fn wrtlock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if WRTLOCK != 0"]
    #[inline] pub fn test_wrtlock(&self) -> bool {
        self.wrtlock() != 0
    }

    #[doc="Sets the WRTLOCK field."]
    #[inline] pub fn set_wrtlock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Control Gain Mode"]
    #[inline] pub fn cgm(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
    }

    #[doc="Returns true if CGM != 0"]
    #[inline] pub fn test_cgm(&self) -> bool {
        self.cgm() != 0
    }

    #[doc="Sets the CGM field."]
    #[inline] pub fn set_cgm<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u16> for Xosc32k {
    #[inline]
    fn from(other: u16) -> Self {
         Xosc32k(other)
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
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.ondemand() != 0 { try!(write!(f, " ondemand"))}
        if self.startup() != 0 { try!(write!(f, " startup=0x{:x}", self.startup()))}
        if self.wrtlock() != 0 { try!(write!(f, " wrtlock"))}
        if self.cgm() != 0 { try!(write!(f, " cgm=0x{:x}", self.cgm()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock Failure Detector Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfdctrl(pub u8);
impl Cfdctrl {
    #[doc="Clock Failure Detector Enable"]
    #[inline] pub fn cfden(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CFDEN != 0"]
    #[inline] pub fn test_cfden(&self) -> bool {
        self.cfden() != 0
    }

    #[doc="Sets the CFDEN field."]
    #[inline] pub fn set_cfden<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clock Switch Back"]
    #[inline] pub fn swback(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SWBACK != 0"]
    #[inline] pub fn test_swback(&self) -> bool {
        self.swback() != 0
    }

    #[doc="Sets the SWBACK field."]
    #[inline] pub fn set_swback<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Clock Failure Detector Prescaler"]
    #[inline] pub fn cfdpresc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CFDPRESC != 0"]
    #[inline] pub fn test_cfdpresc(&self) -> bool {
        self.cfdpresc() != 0
    }

    #[doc="Sets the CFDPRESC field."]
    #[inline] pub fn set_cfdpresc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u8> for Cfdctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Cfdctrl(other)
    }
}

impl ::core::fmt::Display for Cfdctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfdctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cfden() != 0 { try!(write!(f, " cfden"))}
        if self.swback() != 0 { try!(write!(f, " swback"))}
        if self.cfdpresc() != 0 { try!(write!(f, " cfdpresc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Event Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u8);
impl Evctrl {
    #[doc="Clock Failure Detector Event Output Enable"]
    #[inline] pub fn cfdeo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CFDEO != 0"]
    #[inline] pub fn test_cfdeo(&self) -> bool {
        self.cfdeo() != 0
    }

    #[doc="Sets the CFDEO field."]
    #[inline] pub fn set_cfdeo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Evctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Evctrl(other)
    }
}

impl ::core::fmt::Display for Evctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Evctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cfdeo() != 0 { try!(write!(f, " cfdeo"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Osculp32k(pub u32);
impl Osculp32k {
    #[doc="Enable Out 32k"]
    #[inline] pub fn en32k(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EN32K != 0"]
    #[inline] pub fn test_en32k(&self) -> bool {
        self.en32k() != 0
    }

    #[doc="Sets the EN32K field."]
    #[inline] pub fn set_en32k<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Enable Out 1k"]
    #[inline] pub fn en1k(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if EN1K != 0"]
    #[inline] pub fn test_en1k(&self) -> bool {
        self.en1k() != 0
    }

    #[doc="Sets the EN1K field."]
    #[inline] pub fn set_en1k<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Oscillator Calibration"]
    #[inline] pub fn calib(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3f) as u8) } // [13:8]
    }

    #[doc="Returns true if CALIB != 0"]
    #[inline] pub fn test_calib(&self) -> bool {
        self.calib() != 0
    }

    #[doc="Sets the CALIB field."]
    #[inline] pub fn set_calib<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Write Lock"]
    #[inline] pub fn wrtlock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if WRTLOCK != 0"]
    #[inline] pub fn test_wrtlock(&self) -> bool {
        self.wrtlock() != 0
    }

    #[doc="Sets the WRTLOCK field."]
    #[inline] pub fn set_wrtlock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Osculp32k {
    #[inline]
    fn from(other: u32) -> Self {
         Osculp32k(other)
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
        if self.en32k() != 0 { try!(write!(f, " en32k"))}
        if self.en1k() != 0 { try!(write!(f, " en1k"))}
        if self.calib() != 0 { try!(write!(f, " calib=0x{:x}", self.calib()))}
        if self.wrtlock() != 0 { try!(write!(f, " wrtlock"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

