::bobbin_mcu::periph!( PCC, Pcc, PCC_PERIPH, PccPeriph, PCC_OWNED, PCC_REF_COUNT, 0x43002c00, 0x00, 0x15);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="PCC Peripheral"]
pub struct PccPeriph(pub usize); 

impl PccPeriph {
    #[doc="Get the MR Register."]
    #[inline] pub fn mr_reg(&self) -> ::bobbin_mcu::register::Register<Mr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mr, 0x0)
    }

    #[doc="Get the *mut pointer for the MR register."]
    #[inline] pub fn mr_mut(&self) -> *mut Mr { 
        self.mr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MR register."]
    #[inline] pub fn mr_ptr(&self) -> *const Mr { 
        self.mr_reg().ptr()
    }

    #[doc="Read the MR register."]
    #[inline] pub fn mr(&self) -> Mr { 
        self.mr_reg().read()
    }

    #[doc="Write the MR register."]
    #[inline] pub fn write_mr(&self, value: Mr) -> &Self { 
        self.mr_reg().write(value);
        self
    }

    #[doc="Set the MR register."]
    #[inline] pub fn set_mr<F: FnOnce(Mr) -> Mr>(&self, f: F) -> &Self {
        self.mr_reg().set(f);
        self
    }

    #[doc="Modify the MR register."]
    #[inline] pub fn with_mr<F: FnOnce(Mr) -> Mr>(&self, f: F) -> &Self {
        self.mr_reg().with(f);
        self
    }

    #[doc="Get the IER Register."]
    #[inline] pub fn ier_reg(&self) -> ::bobbin_mcu::register::Register<Ier> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ier, 0x4)
    }

    #[doc="Get the *mut pointer for the IER register."]
    #[inline] pub fn ier_mut(&self) -> *mut Ier { 
        self.ier_reg().ptr()
    }

    #[doc="Get the *const pointer for the IER register."]
    #[inline] pub fn ier_ptr(&self) -> *const Ier { 
        self.ier_reg().ptr()
    }

    #[doc="Write the IER register."]
    #[inline] pub fn write_ier(&self, value: Ier) -> &Self { 
        self.ier_reg().write(value);
        self
    }

    #[doc="Set the IER register."]
    #[inline] pub fn set_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
        self.ier_reg().set(f);
        self
    }

    #[doc="Get the IDR Register."]
    #[inline] pub fn idr_reg(&self) -> ::bobbin_mcu::register::Register<Idr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Idr, 0x8)
    }

    #[doc="Get the *mut pointer for the IDR register."]
    #[inline] pub fn idr_mut(&self) -> *mut Idr { 
        self.idr_reg().ptr()
    }

    #[doc="Get the *const pointer for the IDR register."]
    #[inline] pub fn idr_ptr(&self) -> *const Idr { 
        self.idr_reg().ptr()
    }

    #[doc="Write the IDR register."]
    #[inline] pub fn write_idr(&self, value: Idr) -> &Self { 
        self.idr_reg().write(value);
        self
    }

    #[doc="Set the IDR register."]
    #[inline] pub fn set_idr<F: FnOnce(Idr) -> Idr>(&self, f: F) -> &Self {
        self.idr_reg().set(f);
        self
    }

    #[doc="Get the IMR Register."]
    #[inline] pub fn imr_reg(&self) -> ::bobbin_mcu::register::Register<Imr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Imr, 0xc)
    }

    #[doc="Get the *mut pointer for the IMR register."]
    #[inline] pub fn imr_mut(&self) -> *mut Imr { 
        self.imr_reg().ptr()
    }

    #[doc="Get the *const pointer for the IMR register."]
    #[inline] pub fn imr_ptr(&self) -> *const Imr { 
        self.imr_reg().ptr()
    }

    #[doc="Read the IMR register."]
    #[inline] pub fn imr(&self) -> Imr { 
        self.imr_reg().read()
    }

    #[doc="Get the ISR Register."]
    #[inline] pub fn isr_reg(&self) -> ::bobbin_mcu::register::Register<Isr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Isr, 0x10)
    }

    #[doc="Get the *mut pointer for the ISR register."]
    #[inline] pub fn isr_mut(&self) -> *mut Isr { 
        self.isr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ISR register."]
    #[inline] pub fn isr_ptr(&self) -> *const Isr { 
        self.isr_reg().ptr()
    }

    #[doc="Read the ISR register."]
    #[inline] pub fn isr(&self) -> Isr { 
        self.isr_reg().read()
    }

    #[doc="Get the RHR Register."]
    #[inline] pub fn rhr_reg(&self) -> ::bobbin_mcu::register::Register<Rhr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rhr, 0x14)
    }

    #[doc="Get the *mut pointer for the RHR register."]
    #[inline] pub fn rhr_mut(&self) -> *mut Rhr { 
        self.rhr_reg().ptr()
    }

    #[doc="Get the *const pointer for the RHR register."]
    #[inline] pub fn rhr_ptr(&self) -> *const Rhr { 
        self.rhr_reg().ptr()
    }

    #[doc="Read the RHR register."]
    #[inline] pub fn rhr(&self) -> Rhr { 
        self.rhr_reg().read()
    }

    #[doc="Get the WPMR Register."]
    #[inline] pub fn wpmr_reg(&self) -> ::bobbin_mcu::register::Register<Wpmr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Wpmr, 0xe0)
    }

    #[doc="Get the *mut pointer for the WPMR register."]
    #[inline] pub fn wpmr_mut(&self) -> *mut Wpmr { 
        self.wpmr_reg().ptr()
    }

    #[doc="Get the *const pointer for the WPMR register."]
    #[inline] pub fn wpmr_ptr(&self) -> *const Wpmr { 
        self.wpmr_reg().ptr()
    }

    #[doc="Read the WPMR register."]
    #[inline] pub fn wpmr(&self) -> Wpmr { 
        self.wpmr_reg().read()
    }

    #[doc="Write the WPMR register."]
    #[inline] pub fn write_wpmr(&self, value: Wpmr) -> &Self { 
        self.wpmr_reg().write(value);
        self
    }

    #[doc="Set the WPMR register."]
    #[inline] pub fn set_wpmr<F: FnOnce(Wpmr) -> Wpmr>(&self, f: F) -> &Self {
        self.wpmr_reg().set(f);
        self
    }

    #[doc="Modify the WPMR register."]
    #[inline] pub fn with_wpmr<F: FnOnce(Wpmr) -> Wpmr>(&self, f: F) -> &Self {
        self.wpmr_reg().with(f);
        self
    }

    #[doc="Get the WPSR Register."]
    #[inline] pub fn wpsr_reg(&self) -> ::bobbin_mcu::register::Register<Wpsr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Wpsr, 0xe4)
    }

    #[doc="Get the *mut pointer for the WPSR register."]
    #[inline] pub fn wpsr_mut(&self) -> *mut Wpsr { 
        self.wpsr_reg().ptr()
    }

    #[doc="Get the *const pointer for the WPSR register."]
    #[inline] pub fn wpsr_ptr(&self) -> *const Wpsr { 
        self.wpsr_reg().ptr()
    }

    #[doc="Read the WPSR register."]
    #[inline] pub fn wpsr(&self) -> Wpsr { 
        self.wpsr_reg().read()
    }

}

#[doc="Mode Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mr(pub u32);
impl Mr {
    #[doc="Parallel Capture Enable"]
    #[inline] pub fn pcen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PCEN != 0"]
    #[inline] pub fn test_pcen(&self) -> bool {
        self.pcen() != 0
    }

    #[doc="Sets the PCEN field."]
    #[inline] pub fn set_pcen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data size"]
    #[inline] pub fn dsize(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if DSIZE != 0"]
    #[inline] pub fn test_dsize(&self) -> bool {
        self.dsize() != 0
    }

    #[doc="Sets the DSIZE field."]
    #[inline] pub fn set_dsize<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Scale data"]
    #[inline] pub fn scale(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SCALE != 0"]
    #[inline] pub fn test_scale(&self) -> bool {
        self.scale() != 0
    }

    #[doc="Sets the SCALE field."]
    #[inline] pub fn set_scale<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Always Sampling"]
    #[inline] pub fn alwys(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ALWYS != 0"]
    #[inline] pub fn test_alwys(&self) -> bool {
        self.alwys() != 0
    }

    #[doc="Sets the ALWYS field."]
    #[inline] pub fn set_alwys<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Half Sampling"]
    #[inline] pub fn halfs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if HALFS != 0"]
    #[inline] pub fn test_halfs(&self) -> bool {
        self.halfs() != 0
    }

    #[doc="Sets the HALFS field."]
    #[inline] pub fn set_halfs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="First sample"]
    #[inline] pub fn frsts(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if FRSTS != 0"]
    #[inline] pub fn test_frsts(&self) -> bool {
        self.frsts() != 0
    }

    #[doc="Sets the FRSTS field."]
    #[inline] pub fn set_frsts<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Input Data Size"]
    #[inline] pub fn isize(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Returns true if ISIZE != 0"]
    #[inline] pub fn test_isize(&self) -> bool {
        self.isize() != 0
    }

    #[doc="Sets the ISIZE field."]
    #[inline] pub fn set_isize<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Clear If Disabled"]
    #[inline] pub fn cid(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x3) as u8) } // [31:30]
    }

    #[doc="Returns true if CID != 0"]
    #[inline] pub fn test_cid(&self) -> bool {
        self.cid() != 0
    }

    #[doc="Sets the CID field."]
    #[inline] pub fn set_cid<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 30);
        self.0 |= value << 30;
        self
    }

}

impl From<u32> for Mr {
    #[inline]
    fn from(other: u32) -> Self {
         Mr(other)
    }
}

impl ::core::fmt::Display for Mr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcen() != 0 { try!(write!(f, " pcen"))}
        if self.dsize() != 0 { try!(write!(f, " dsize=0x{:x}", self.dsize()))}
        if self.scale() != 0 { try!(write!(f, " scale"))}
        if self.alwys() != 0 { try!(write!(f, " alwys"))}
        if self.halfs() != 0 { try!(write!(f, " halfs"))}
        if self.frsts() != 0 { try!(write!(f, " frsts"))}
        if self.isize() != 0 { try!(write!(f, " isize=0x{:x}", self.isize()))}
        if self.cid() != 0 { try!(write!(f, " cid=0x{:x}", self.cid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc="Data Ready Interrupt Enable"]
    #[inline] pub fn drdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DRDY != 0"]
    #[inline] pub fn test_drdy(&self) -> bool {
        self.drdy() != 0
    }

    #[doc="Sets the DRDY field."]
    #[inline] pub fn set_drdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Overrun Error Interrupt Enable"]
    #[inline] pub fn ovre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OVRE != 0"]
    #[inline] pub fn test_ovre(&self) -> bool {
        self.ovre() != 0
    }

    #[doc="Sets the OVRE field."]
    #[inline] pub fn set_ovre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Ier {
    #[inline]
    fn from(other: u32) -> Self {
         Ier(other)
    }
}

impl ::core::fmt::Display for Ier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.drdy() != 0 { try!(write!(f, " drdy"))}
        if self.ovre() != 0 { try!(write!(f, " ovre"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Disable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Idr(pub u32);
impl Idr {
    #[doc="Data Ready Interrupt Disable"]
    #[inline] pub fn drdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DRDY != 0"]
    #[inline] pub fn test_drdy(&self) -> bool {
        self.drdy() != 0
    }

    #[doc="Sets the DRDY field."]
    #[inline] pub fn set_drdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Overrun Error Interrupt Disable"]
    #[inline] pub fn ovre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OVRE != 0"]
    #[inline] pub fn test_ovre(&self) -> bool {
        self.ovre() != 0
    }

    #[doc="Sets the OVRE field."]
    #[inline] pub fn set_ovre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Idr {
    #[inline]
    fn from(other: u32) -> Self {
         Idr(other)
    }
}

impl ::core::fmt::Display for Idr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Idr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.drdy() != 0 { try!(write!(f, " drdy"))}
        if self.ovre() != 0 { try!(write!(f, " ovre"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Mask Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Imr(pub u32);
impl Imr {
    #[doc="Data Ready Interrupt Mask"]
    #[inline] pub fn drdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DRDY != 0"]
    #[inline] pub fn test_drdy(&self) -> bool {
        self.drdy() != 0
    }

    #[doc="Sets the DRDY field."]
    #[inline] pub fn set_drdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Overrun Error Interrupt Mask"]
    #[inline] pub fn ovre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OVRE != 0"]
    #[inline] pub fn test_ovre(&self) -> bool {
        self.ovre() != 0
    }

    #[doc="Sets the OVRE field."]
    #[inline] pub fn set_ovre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Imr {
    #[inline]
    fn from(other: u32) -> Self {
         Imr(other)
    }
}

impl ::core::fmt::Display for Imr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Imr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.drdy() != 0 { try!(write!(f, " drdy"))}
        if self.ovre() != 0 { try!(write!(f, " ovre"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc="Data Ready Interrupt Status"]
    #[inline] pub fn drdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DRDY != 0"]
    #[inline] pub fn test_drdy(&self) -> bool {
        self.drdy() != 0
    }

    #[doc="Sets the DRDY field."]
    #[inline] pub fn set_drdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Overrun Error Interrupt Status"]
    #[inline] pub fn ovre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OVRE != 0"]
    #[inline] pub fn test_ovre(&self) -> bool {
        self.ovre() != 0
    }

    #[doc="Sets the OVRE field."]
    #[inline] pub fn set_ovre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Isr {
    #[inline]
    fn from(other: u32) -> Self {
         Isr(other)
    }
}

impl ::core::fmt::Display for Isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.drdy() != 0 { try!(write!(f, " drdy"))}
        if self.ovre() != 0 { try!(write!(f, " ovre"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reception Holding Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rhr(pub u32);
impl Rhr {
    #[doc="Reception Data"]
    #[inline] pub fn rdata(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if RDATA != 0"]
    #[inline] pub fn test_rdata(&self) -> bool {
        self.rdata() != 0
    }

    #[doc="Sets the RDATA field."]
    #[inline] pub fn set_rdata<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rhr {
    #[inline]
    fn from(other: u32) -> Self {
         Rhr(other)
    }
}

impl ::core::fmt::Display for Rhr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rhr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Write Protection Mode Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wpmr(pub u32);
impl Wpmr {
    #[doc="Write Protection Enable"]
    #[inline] pub fn wpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if WPEN != 0"]
    #[inline] pub fn test_wpen(&self) -> bool {
        self.wpen() != 0
    }

    #[doc="Sets the WPEN field."]
    #[inline] pub fn set_wpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Write Protection Key"]
    #[inline] pub fn wpkey(&self) -> ::bobbin_bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if WPKEY != 0"]
    #[inline] pub fn test_wpkey(&self) -> bool {
        self.wpkey() != 0
    }

    #[doc="Sets the WPKEY field."]
    #[inline] pub fn set_wpkey<V: Into<::bobbin_bits::U24>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Wpmr {
    #[inline]
    fn from(other: u32) -> Self {
         Wpmr(other)
    }
}

impl ::core::fmt::Display for Wpmr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wpmr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wpen() != 0 { try!(write!(f, " wpen"))}
        if self.wpkey() != 0 { try!(write!(f, " wpkey=0x{:x}", self.wpkey()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Write Protection Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wpsr(pub u32);
impl Wpsr {
    #[doc="Write Protection Violation Source"]
    #[inline] pub fn wpvs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if WPVS != 0"]
    #[inline] pub fn test_wpvs(&self) -> bool {
        self.wpvs() != 0
    }

    #[doc="Sets the WPVS field."]
    #[inline] pub fn set_wpvs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Write Protection Violation Status"]
    #[inline] pub fn wpvsrc(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffff) as u16) } // [23:8]
    }

    #[doc="Returns true if WPVSRC != 0"]
    #[inline] pub fn test_wpvsrc(&self) -> bool {
        self.wpvsrc() != 0
    }

    #[doc="Sets the WPVSRC field."]
    #[inline] pub fn set_wpvsrc<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Wpsr {
    #[inline]
    fn from(other: u32) -> Self {
         Wpsr(other)
    }
}

impl ::core::fmt::Display for Wpsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wpsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wpvs() != 0 { try!(write!(f, " wpvs"))}
        if self.wpvsrc() != 0 { try!(write!(f, " wpvsrc=0x{:x}", self.wpvsrc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

