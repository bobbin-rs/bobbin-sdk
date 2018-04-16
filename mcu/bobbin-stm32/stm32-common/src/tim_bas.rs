
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="TIM_BAS Peripheral"]
pub struct TimBasPeriph(pub usize); 

pub struct TimBasCh { pub periph: TimBasPeriph, pub index: usize }

impl TimBasPeriph {
    #[doc="Get the CR1 Register."]
    #[inline] pub fn cr1_reg(&self) -> ::bobbin_mcu::register::Register<Cr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr1, 0x0)
    }

    #[doc="Get the *mut pointer for the CR1 register."]
    #[inline] pub fn cr1_mut(&self) -> *mut Cr1 { 
        self.cr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR1 register."]
    #[inline] pub fn cr1_ptr(&self) -> *const Cr1 { 
        self.cr1_reg().ptr()
    }

    #[doc="Read the CR1 register."]
    #[inline] pub fn cr1(&self) -> Cr1 { 
        self.cr1_reg().read()
    }

    #[doc="Write the CR1 register."]
    #[inline] pub fn write_cr1(&self, value: Cr1) -> &Self { 
        self.cr1_reg().write(value);
        self
    }

    #[doc="Set the CR1 register."]
    #[inline] pub fn set_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
        self.cr1_reg().set(f);
        self
    }

    #[doc="Modify the CR1 register."]
    #[inline] pub fn with_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
        self.cr1_reg().with(f);
        self
    }

    #[doc="Get the CR2 Register."]
    #[inline] pub fn cr2_reg(&self) -> ::bobbin_mcu::register::Register<Cr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr2, 0x4)
    }

    #[doc="Get the *mut pointer for the CR2 register."]
    #[inline] pub fn cr2_mut(&self) -> *mut Cr2 { 
        self.cr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR2 register."]
    #[inline] pub fn cr2_ptr(&self) -> *const Cr2 { 
        self.cr2_reg().ptr()
    }

    #[doc="Read the CR2 register."]
    #[inline] pub fn cr2(&self) -> Cr2 { 
        self.cr2_reg().read()
    }

    #[doc="Write the CR2 register."]
    #[inline] pub fn write_cr2(&self, value: Cr2) -> &Self { 
        self.cr2_reg().write(value);
        self
    }

    #[doc="Set the CR2 register."]
    #[inline] pub fn set_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
        self.cr2_reg().set(f);
        self
    }

    #[doc="Modify the CR2 register."]
    #[inline] pub fn with_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
        self.cr2_reg().with(f);
        self
    }

    #[doc="Get the DIER Register."]
    #[inline] pub fn dier_reg(&self) -> ::bobbin_mcu::register::Register<Dier> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dier, 0xc)
    }

    #[doc="Get the *mut pointer for the DIER register."]
    #[inline] pub fn dier_mut(&self) -> *mut Dier { 
        self.dier_reg().ptr()
    }

    #[doc="Get the *const pointer for the DIER register."]
    #[inline] pub fn dier_ptr(&self) -> *const Dier { 
        self.dier_reg().ptr()
    }

    #[doc="Read the DIER register."]
    #[inline] pub fn dier(&self) -> Dier { 
        self.dier_reg().read()
    }

    #[doc="Write the DIER register."]
    #[inline] pub fn write_dier(&self, value: Dier) -> &Self { 
        self.dier_reg().write(value);
        self
    }

    #[doc="Set the DIER register."]
    #[inline] pub fn set_dier<F: FnOnce(Dier) -> Dier>(&self, f: F) -> &Self {
        self.dier_reg().set(f);
        self
    }

    #[doc="Modify the DIER register."]
    #[inline] pub fn with_dier<F: FnOnce(Dier) -> Dier>(&self, f: F) -> &Self {
        self.dier_reg().with(f);
        self
    }

    #[doc="Get the SR Register."]
    #[inline] pub fn sr_reg(&self) -> ::bobbin_mcu::register::Register<Sr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sr, 0x10)
    }

    #[doc="Get the *mut pointer for the SR register."]
    #[inline] pub fn sr_mut(&self) -> *mut Sr { 
        self.sr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SR register."]
    #[inline] pub fn sr_ptr(&self) -> *const Sr { 
        self.sr_reg().ptr()
    }

    #[doc="Read the SR register."]
    #[inline] pub fn sr(&self) -> Sr { 
        self.sr_reg().read()
    }

    #[doc="Write the SR register."]
    #[inline] pub fn write_sr(&self, value: Sr) -> &Self { 
        self.sr_reg().write(value);
        self
    }

    #[doc="Set the SR register."]
    #[inline] pub fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        self.sr_reg().set(f);
        self
    }

    #[doc="Modify the SR register."]
    #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        self.sr_reg().with(f);
        self
    }

    #[doc="Get the EGR Register."]
    #[inline] pub fn egr_reg(&self) -> ::bobbin_mcu::register::Register<Egr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Egr, 0x14)
    }

    #[doc="Get the *mut pointer for the EGR register."]
    #[inline] pub fn egr_mut(&self) -> *mut Egr { 
        self.egr_reg().ptr()
    }

    #[doc="Get the *const pointer for the EGR register."]
    #[inline] pub fn egr_ptr(&self) -> *const Egr { 
        self.egr_reg().ptr()
    }

    #[doc="Write the EGR register."]
    #[inline] pub fn write_egr(&self, value: Egr) -> &Self { 
        self.egr_reg().write(value);
        self
    }

    #[doc="Set the EGR register."]
    #[inline] pub fn set_egr<F: FnOnce(Egr) -> Egr>(&self, f: F) -> &Self {
        self.egr_reg().set(f);
        self
    }

    #[doc="Get the CNT Register."]
    #[inline] pub fn cnt_reg(&self) -> ::bobbin_mcu::register::Register<Cnt> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cnt, 0x24)
    }

    #[doc="Get the *mut pointer for the CNT register."]
    #[inline] pub fn cnt_mut(&self) -> *mut Cnt { 
        self.cnt_reg().ptr()
    }

    #[doc="Get the *const pointer for the CNT register."]
    #[inline] pub fn cnt_ptr(&self) -> *const Cnt { 
        self.cnt_reg().ptr()
    }

    #[doc="Read the CNT register."]
    #[inline] pub fn cnt(&self) -> Cnt { 
        self.cnt_reg().read()
    }

    #[doc="Write the CNT register."]
    #[inline] pub fn write_cnt(&self, value: Cnt) -> &Self { 
        self.cnt_reg().write(value);
        self
    }

    #[doc="Set the CNT register."]
    #[inline] pub fn set_cnt<F: FnOnce(Cnt) -> Cnt>(&self, f: F) -> &Self {
        self.cnt_reg().set(f);
        self
    }

    #[doc="Modify the CNT register."]
    #[inline] pub fn with_cnt<F: FnOnce(Cnt) -> Cnt>(&self, f: F) -> &Self {
        self.cnt_reg().with(f);
        self
    }

    #[doc="Get the PSC Register."]
    #[inline] pub fn psc_reg(&self) -> ::bobbin_mcu::register::Register<Psc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Psc, 0x28)
    }

    #[doc="Get the *mut pointer for the PSC register."]
    #[inline] pub fn psc_mut(&self) -> *mut Psc { 
        self.psc_reg().ptr()
    }

    #[doc="Get the *const pointer for the PSC register."]
    #[inline] pub fn psc_ptr(&self) -> *const Psc { 
        self.psc_reg().ptr()
    }

    #[doc="Read the PSC register."]
    #[inline] pub fn psc(&self) -> Psc { 
        self.psc_reg().read()
    }

    #[doc="Write the PSC register."]
    #[inline] pub fn write_psc(&self, value: Psc) -> &Self { 
        self.psc_reg().write(value);
        self
    }

    #[doc="Set the PSC register."]
    #[inline] pub fn set_psc<F: FnOnce(Psc) -> Psc>(&self, f: F) -> &Self {
        self.psc_reg().set(f);
        self
    }

    #[doc="Modify the PSC register."]
    #[inline] pub fn with_psc<F: FnOnce(Psc) -> Psc>(&self, f: F) -> &Self {
        self.psc_reg().with(f);
        self
    }

    #[doc="Get the ARR Register."]
    #[inline] pub fn arr_reg(&self) -> ::bobbin_mcu::register::Register<Arr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Arr, 0x2c)
    }

    #[doc="Get the *mut pointer for the ARR register."]
    #[inline] pub fn arr_mut(&self) -> *mut Arr { 
        self.arr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ARR register."]
    #[inline] pub fn arr_ptr(&self) -> *const Arr { 
        self.arr_reg().ptr()
    }

    #[doc="Read the ARR register."]
    #[inline] pub fn arr(&self) -> Arr { 
        self.arr_reg().read()
    }

    #[doc="Write the ARR register."]
    #[inline] pub fn write_arr(&self, value: Arr) -> &Self { 
        self.arr_reg().write(value);
        self
    }

    #[doc="Set the ARR register."]
    #[inline] pub fn set_arr<F: FnOnce(Arr) -> Arr>(&self, f: F) -> &Self {
        self.arr_reg().set(f);
        self
    }

    #[doc="Modify the ARR register."]
    #[inline] pub fn with_arr<F: FnOnce(Arr) -> Arr>(&self, f: F) -> &Self {
        self.arr_reg().with(f);
        self
    }

}

#[doc="control register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc="Counter enable"]
    #[inline] pub fn cen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CEN != 0"]
    #[inline] pub fn test_cen(&self) -> bool {
        self.cen() != 0
    }

    #[doc="Sets the CEN field."]
    #[inline] pub fn set_cen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Update disable"]
    #[inline] pub fn udis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if UDIS != 0"]
    #[inline] pub fn test_udis(&self) -> bool {
        self.udis() != 0
    }

    #[doc="Sets the UDIS field."]
    #[inline] pub fn set_udis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Update request source"]
    #[inline] pub fn urs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if URS != 0"]
    #[inline] pub fn test_urs(&self) -> bool {
        self.urs() != 0
    }

    #[doc="Sets the URS field."]
    #[inline] pub fn set_urs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="One-pulse mode"]
    #[inline] pub fn opm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if OPM != 0"]
    #[inline] pub fn test_opm(&self) -> bool {
        self.opm() != 0
    }

    #[doc="Sets the OPM field."]
    #[inline] pub fn set_opm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Auto-reload preload enable"]
    #[inline] pub fn arpe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ARPE != 0"]
    #[inline] pub fn test_arpe(&self) -> bool {
        self.arpe() != 0
    }

    #[doc="Sets the ARPE field."]
    #[inline] pub fn set_arpe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="UIF status bit remapping"]
    #[inline] pub fn uifremap(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if UIFREMAP != 0"]
    #[inline] pub fn test_uifremap(&self) -> bool {
        self.uifremap() != 0
    }

    #[doc="Sets the UIFREMAP field."]
    #[inline] pub fn set_uifremap<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for Cr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Cr1(other)
    }
}

impl ::core::fmt::Display for Cr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cen() != 0 { try!(write!(f, " cen"))}
        if self.udis() != 0 { try!(write!(f, " udis"))}
        if self.urs() != 0 { try!(write!(f, " urs"))}
        if self.opm() != 0 { try!(write!(f, " opm"))}
        if self.arpe() != 0 { try!(write!(f, " arpe"))}
        if self.uifremap() != 0 { try!(write!(f, " uifremap"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="control register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr2(pub u32);
impl Cr2 {
    #[doc="Master mode selection"]
    #[inline] pub fn mms(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if MMS != 0"]
    #[inline] pub fn test_mms(&self) -> bool {
        self.mms() != 0
    }

    #[doc="Sets the MMS field."]
    #[inline] pub fn set_mms<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Cr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Cr2(other)
    }
}

impl ::core::fmt::Display for Cr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mms() != 0 { try!(write!(f, " mms=0x{:x}", self.mms()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA/Interrupt enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dier(pub u32);
impl Dier {
    #[doc="Update DMA request enable"]
    #[inline] pub fn ude(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if UDE != 0"]
    #[inline] pub fn test_ude(&self) -> bool {
        self.ude() != 0
    }

    #[doc="Sets the UDE field."]
    #[inline] pub fn set_ude<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Update interrupt enable"]
    #[inline] pub fn uie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if UIE != 0"]
    #[inline] pub fn test_uie(&self) -> bool {
        self.uie() != 0
    }

    #[doc="Sets the UIE field."]
    #[inline] pub fn set_uie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dier {
    #[inline]
    fn from(other: u32) -> Self {
         Dier(other)
    }
}

impl ::core::fmt::Display for Dier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ude() != 0 { try!(write!(f, " ude"))}
        if self.uie() != 0 { try!(write!(f, " uie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="Update interrupt flag"]
    #[inline] pub fn uif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if UIF != 0"]
    #[inline] pub fn test_uif(&self) -> bool {
        self.uif() != 0
    }

    #[doc="Sets the UIF field."]
    #[inline] pub fn set_uif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sr {
    #[inline]
    fn from(other: u32) -> Self {
         Sr(other)
    }
}

impl ::core::fmt::Display for Sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.uif() != 0 { try!(write!(f, " uif"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="event generation register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Egr(pub u32);
impl Egr {
    #[doc="Update generation"]
    #[inline] pub fn ug(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if UG != 0"]
    #[inline] pub fn test_ug(&self) -> bool {
        self.ug() != 0
    }

    #[doc="Sets the UG field."]
    #[inline] pub fn set_ug<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Egr {
    #[inline]
    fn from(other: u32) -> Self {
         Egr(other)
    }
}

impl ::core::fmt::Display for Egr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Egr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ug() != 0 { try!(write!(f, " ug"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="counter"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc="Low counter value"]
    #[inline] pub fn cnt(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CNT != 0"]
    #[inline] pub fn test_cnt(&self) -> bool {
        self.cnt() != 0
    }

    #[doc="Sets the CNT field."]
    #[inline] pub fn set_cnt<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="UIF Copy"]
    #[inline] pub fn uifcpy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if UIFCPY != 0"]
    #[inline] pub fn test_uifcpy(&self) -> bool {
        self.uifcpy() != 0
    }

    #[doc="Sets the UIFCPY field."]
    #[inline] pub fn set_uifcpy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Cnt {
    #[inline]
    fn from(other: u32) -> Self {
         Cnt(other)
    }
}

impl ::core::fmt::Display for Cnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cnt() != 0 { try!(write!(f, " cnt=0x{:x}", self.cnt()))}
        if self.uifcpy() != 0 { try!(write!(f, " uifcpy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="prescaler"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Psc(pub u32);
impl Psc {
    #[doc="Prescaler value"]
    #[inline] pub fn psc(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if PSC != 0"]
    #[inline] pub fn test_psc(&self) -> bool {
        self.psc() != 0
    }

    #[doc="Sets the PSC field."]
    #[inline] pub fn set_psc<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Psc {
    #[inline]
    fn from(other: u32) -> Self {
         Psc(other)
    }
}

impl ::core::fmt::Display for Psc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Psc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.psc() != 0 { try!(write!(f, " psc=0x{:x}", self.psc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="auto-reload register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Arr(pub u32);
impl Arr {
    #[doc="Low Auto-reload value"]
    #[inline] pub fn arr(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if ARR != 0"]
    #[inline] pub fn test_arr(&self) -> bool {
        self.arr() != 0
    }

    #[doc="Sets the ARR field."]
    #[inline] pub fn set_arr<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Arr {
    #[inline]
    fn from(other: u32) -> Self {
         Arr(other)
    }
}

impl ::core::fmt::Display for Arr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Arr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.arr() != 0 { try!(write!(f, " arr=0x{:x}", self.arr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

