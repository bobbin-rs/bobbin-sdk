
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="ADC_F1 Peripheral"]
pub struct AdcPeriph(pub usize); 

pub struct AdcCh { pub periph: AdcPeriph, pub index: usize }

impl AdcPeriph {
    #[doc="Get the SR Register."]
    #[inline] pub fn sr_reg(&self) -> ::bobbin_mcu::register::Register<Sr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sr, 0x0)
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

    #[doc="Get the CR1 Register."]
    #[inline] pub fn cr1_reg(&self) -> ::bobbin_mcu::register::Register<Cr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr1, 0x4)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr2, 0x8)
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

    #[doc="Get the SMPR1 Register."]
    #[inline] pub fn smpr1_reg(&self) -> ::bobbin_mcu::register::Register<Smpr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Smpr1, 0xc)
    }

    #[doc="Get the *mut pointer for the SMPR1 register."]
    #[inline] pub fn smpr1_mut(&self) -> *mut Smpr1 { 
        self.smpr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the SMPR1 register."]
    #[inline] pub fn smpr1_ptr(&self) -> *const Smpr1 { 
        self.smpr1_reg().ptr()
    }

    #[doc="Read the SMPR1 register."]
    #[inline] pub fn smpr1(&self) -> Smpr1 { 
        self.smpr1_reg().read()
    }

    #[doc="Write the SMPR1 register."]
    #[inline] pub fn write_smpr1(&self, value: Smpr1) -> &Self { 
        self.smpr1_reg().write(value);
        self
    }

    #[doc="Set the SMPR1 register."]
    #[inline] pub fn set_smpr1<F: FnOnce(Smpr1) -> Smpr1>(&self, f: F) -> &Self {
        self.smpr1_reg().set(f);
        self
    }

    #[doc="Modify the SMPR1 register."]
    #[inline] pub fn with_smpr1<F: FnOnce(Smpr1) -> Smpr1>(&self, f: F) -> &Self {
        self.smpr1_reg().with(f);
        self
    }

    #[doc="Get the SMPR2 Register."]
    #[inline] pub fn smpr2_reg(&self) -> ::bobbin_mcu::register::Register<Smpr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Smpr2, 0x10)
    }

    #[doc="Get the *mut pointer for the SMPR2 register."]
    #[inline] pub fn smpr2_mut(&self) -> *mut Smpr2 { 
        self.smpr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the SMPR2 register."]
    #[inline] pub fn smpr2_ptr(&self) -> *const Smpr2 { 
        self.smpr2_reg().ptr()
    }

    #[doc="Read the SMPR2 register."]
    #[inline] pub fn smpr2(&self) -> Smpr2 { 
        self.smpr2_reg().read()
    }

    #[doc="Write the SMPR2 register."]
    #[inline] pub fn write_smpr2(&self, value: Smpr2) -> &Self { 
        self.smpr2_reg().write(value);
        self
    }

    #[doc="Set the SMPR2 register."]
    #[inline] pub fn set_smpr2<F: FnOnce(Smpr2) -> Smpr2>(&self, f: F) -> &Self {
        self.smpr2_reg().set(f);
        self
    }

    #[doc="Modify the SMPR2 register."]
    #[inline] pub fn with_smpr2<F: FnOnce(Smpr2) -> Smpr2>(&self, f: F) -> &Self {
        self.smpr2_reg().with(f);
        self
    }

    #[doc="Get the JOFR Register."]
    #[inline] pub fn jofr_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Jofr, ::bobbin_bits::R4> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Jofr, 0x14, 0x4)
    }

    #[doc="Get the *mut pointer for the JOFR register."]
    #[inline] pub fn jofr_mut<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *mut Jofr { 
        self.jofr_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the JOFR register."]
    #[inline] pub fn jofr_ptr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *const Jofr { 
        self.jofr_reg().ptr(index.into())
    }

    #[doc="Read the JOFR register."]
    #[inline] pub fn jofr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> Jofr { 
        self.jofr_reg().read(index.into())
    }

    #[doc="Write the JOFR register."]
    #[inline] pub fn write_jofr<I: Into<::bobbin_bits::R4>>(&self, index: I, value: Jofr) -> &Self {
        self.jofr_reg().write(index.into(), value);
        self
    }

    #[doc="Set the JOFR register."]
    #[inline] pub fn set_jofr<I: Into<::bobbin_bits::R4>, F: FnOnce(Jofr) -> Jofr>(&self, index: I, f: F) -> &Self {
        self.jofr_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the JOFR register."]
    #[inline] pub fn with_jofr<I: Into<::bobbin_bits::R4> + Copy, F: FnOnce(Jofr) -> Jofr>(&self, index: I, f: F) -> &Self {
        self.jofr_reg().with(index.into(), f);
        self
    }

    #[doc="Get the HTR Register."]
    #[inline] pub fn htr_reg(&self) -> ::bobbin_mcu::register::Register<Htr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Htr, 0x24)
    }

    #[doc="Get the *mut pointer for the HTR register."]
    #[inline] pub fn htr_mut(&self) -> *mut Htr { 
        self.htr_reg().ptr()
    }

    #[doc="Get the *const pointer for the HTR register."]
    #[inline] pub fn htr_ptr(&self) -> *const Htr { 
        self.htr_reg().ptr()
    }

    #[doc="Read the HTR register."]
    #[inline] pub fn htr(&self) -> Htr { 
        self.htr_reg().read()
    }

    #[doc="Write the HTR register."]
    #[inline] pub fn write_htr(&self, value: Htr) -> &Self { 
        self.htr_reg().write(value);
        self
    }

    #[doc="Set the HTR register."]
    #[inline] pub fn set_htr<F: FnOnce(Htr) -> Htr>(&self, f: F) -> &Self {
        self.htr_reg().set(f);
        self
    }

    #[doc="Modify the HTR register."]
    #[inline] pub fn with_htr<F: FnOnce(Htr) -> Htr>(&self, f: F) -> &Self {
        self.htr_reg().with(f);
        self
    }

    #[doc="Get the LTR Register."]
    #[inline] pub fn ltr_reg(&self) -> ::bobbin_mcu::register::Register<Ltr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ltr, 0x28)
    }

    #[doc="Get the *mut pointer for the LTR register."]
    #[inline] pub fn ltr_mut(&self) -> *mut Ltr { 
        self.ltr_reg().ptr()
    }

    #[doc="Get the *const pointer for the LTR register."]
    #[inline] pub fn ltr_ptr(&self) -> *const Ltr { 
        self.ltr_reg().ptr()
    }

    #[doc="Read the LTR register."]
    #[inline] pub fn ltr(&self) -> Ltr { 
        self.ltr_reg().read()
    }

    #[doc="Write the LTR register."]
    #[inline] pub fn write_ltr(&self, value: Ltr) -> &Self { 
        self.ltr_reg().write(value);
        self
    }

    #[doc="Set the LTR register."]
    #[inline] pub fn set_ltr<F: FnOnce(Ltr) -> Ltr>(&self, f: F) -> &Self {
        self.ltr_reg().set(f);
        self
    }

    #[doc="Modify the LTR register."]
    #[inline] pub fn with_ltr<F: FnOnce(Ltr) -> Ltr>(&self, f: F) -> &Self {
        self.ltr_reg().with(f);
        self
    }

    #[doc="Get the SQR1 Register."]
    #[inline] pub fn sqr1_reg(&self) -> ::bobbin_mcu::register::Register<Sqr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sqr1, 0x2c)
    }

    #[doc="Get the *mut pointer for the SQR1 register."]
    #[inline] pub fn sqr1_mut(&self) -> *mut Sqr1 { 
        self.sqr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the SQR1 register."]
    #[inline] pub fn sqr1_ptr(&self) -> *const Sqr1 { 
        self.sqr1_reg().ptr()
    }

    #[doc="Read the SQR1 register."]
    #[inline] pub fn sqr1(&self) -> Sqr1 { 
        self.sqr1_reg().read()
    }

    #[doc="Write the SQR1 register."]
    #[inline] pub fn write_sqr1(&self, value: Sqr1) -> &Self { 
        self.sqr1_reg().write(value);
        self
    }

    #[doc="Set the SQR1 register."]
    #[inline] pub fn set_sqr1<F: FnOnce(Sqr1) -> Sqr1>(&self, f: F) -> &Self {
        self.sqr1_reg().set(f);
        self
    }

    #[doc="Modify the SQR1 register."]
    #[inline] pub fn with_sqr1<F: FnOnce(Sqr1) -> Sqr1>(&self, f: F) -> &Self {
        self.sqr1_reg().with(f);
        self
    }

    #[doc="Get the SQR2 Register."]
    #[inline] pub fn sqr2_reg(&self) -> ::bobbin_mcu::register::Register<Sqr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sqr2, 0x30)
    }

    #[doc="Get the *mut pointer for the SQR2 register."]
    #[inline] pub fn sqr2_mut(&self) -> *mut Sqr2 { 
        self.sqr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the SQR2 register."]
    #[inline] pub fn sqr2_ptr(&self) -> *const Sqr2 { 
        self.sqr2_reg().ptr()
    }

    #[doc="Read the SQR2 register."]
    #[inline] pub fn sqr2(&self) -> Sqr2 { 
        self.sqr2_reg().read()
    }

    #[doc="Write the SQR2 register."]
    #[inline] pub fn write_sqr2(&self, value: Sqr2) -> &Self { 
        self.sqr2_reg().write(value);
        self
    }

    #[doc="Set the SQR2 register."]
    #[inline] pub fn set_sqr2<F: FnOnce(Sqr2) -> Sqr2>(&self, f: F) -> &Self {
        self.sqr2_reg().set(f);
        self
    }

    #[doc="Modify the SQR2 register."]
    #[inline] pub fn with_sqr2<F: FnOnce(Sqr2) -> Sqr2>(&self, f: F) -> &Self {
        self.sqr2_reg().with(f);
        self
    }

    #[doc="Get the SQR3 Register."]
    #[inline] pub fn sqr3_reg(&self) -> ::bobbin_mcu::register::Register<Sqr3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sqr3, 0x34)
    }

    #[doc="Get the *mut pointer for the SQR3 register."]
    #[inline] pub fn sqr3_mut(&self) -> *mut Sqr3 { 
        self.sqr3_reg().ptr()
    }

    #[doc="Get the *const pointer for the SQR3 register."]
    #[inline] pub fn sqr3_ptr(&self) -> *const Sqr3 { 
        self.sqr3_reg().ptr()
    }

    #[doc="Read the SQR3 register."]
    #[inline] pub fn sqr3(&self) -> Sqr3 { 
        self.sqr3_reg().read()
    }

    #[doc="Write the SQR3 register."]
    #[inline] pub fn write_sqr3(&self, value: Sqr3) -> &Self { 
        self.sqr3_reg().write(value);
        self
    }

    #[doc="Set the SQR3 register."]
    #[inline] pub fn set_sqr3<F: FnOnce(Sqr3) -> Sqr3>(&self, f: F) -> &Self {
        self.sqr3_reg().set(f);
        self
    }

    #[doc="Modify the SQR3 register."]
    #[inline] pub fn with_sqr3<F: FnOnce(Sqr3) -> Sqr3>(&self, f: F) -> &Self {
        self.sqr3_reg().with(f);
        self
    }

    #[doc="Get the JSQR Register."]
    #[inline] pub fn jsqr_reg(&self) -> ::bobbin_mcu::register::Register<Jsqr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Jsqr, 0x38)
    }

    #[doc="Get the *mut pointer for the JSQR register."]
    #[inline] pub fn jsqr_mut(&self) -> *mut Jsqr { 
        self.jsqr_reg().ptr()
    }

    #[doc="Get the *const pointer for the JSQR register."]
    #[inline] pub fn jsqr_ptr(&self) -> *const Jsqr { 
        self.jsqr_reg().ptr()
    }

    #[doc="Read the JSQR register."]
    #[inline] pub fn jsqr(&self) -> Jsqr { 
        self.jsqr_reg().read()
    }

    #[doc="Write the JSQR register."]
    #[inline] pub fn write_jsqr(&self, value: Jsqr) -> &Self { 
        self.jsqr_reg().write(value);
        self
    }

    #[doc="Set the JSQR register."]
    #[inline] pub fn set_jsqr<F: FnOnce(Jsqr) -> Jsqr>(&self, f: F) -> &Self {
        self.jsqr_reg().set(f);
        self
    }

    #[doc="Modify the JSQR register."]
    #[inline] pub fn with_jsqr<F: FnOnce(Jsqr) -> Jsqr>(&self, f: F) -> &Self {
        self.jsqr_reg().with(f);
        self
    }

    #[doc="Get the JDR Register."]
    #[inline] pub fn jdr_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Jdr, ::bobbin_bits::R4> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Jdr, 0x3c, 0x4)
    }

    #[doc="Get the *mut pointer for the JDR register."]
    #[inline] pub fn jdr_mut<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *mut Jdr { 
        self.jdr_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the JDR register."]
    #[inline] pub fn jdr_ptr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *const Jdr { 
        self.jdr_reg().ptr(index.into())
    }

    #[doc="Read the JDR register."]
    #[inline] pub fn jdr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> Jdr { 
        self.jdr_reg().read(index.into())
    }

    #[doc="Get the DR Register."]
    #[inline] pub fn dr_reg(&self) -> ::bobbin_mcu::register::Register<Dr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dr, 0x4c)
    }

    #[doc="Get the *mut pointer for the DR register."]
    #[inline] pub fn dr_mut(&self) -> *mut Dr { 
        self.dr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DR register."]
    #[inline] pub fn dr_ptr(&self) -> *const Dr { 
        self.dr_reg().ptr()
    }

    #[doc="Read the DR register."]
    #[inline] pub fn dr(&self) -> Dr { 
        self.dr_reg().read()
    }

}

#[doc="status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="Overrun"]
    #[inline] pub fn ovr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if OVR != 0"]
    #[inline] pub fn test_ovr(&self) -> bool {
        self.ovr() != 0
    }

    #[doc="Sets the OVR field."]
    #[inline] pub fn set_ovr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Regular channel start flag"]
    #[inline] pub fn strt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if STRT != 0"]
    #[inline] pub fn test_strt(&self) -> bool {
        self.strt() != 0
    }

    #[doc="Sets the STRT field."]
    #[inline] pub fn set_strt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Injected channel start flag"]
    #[inline] pub fn jstrt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if JSTRT != 0"]
    #[inline] pub fn test_jstrt(&self) -> bool {
        self.jstrt() != 0
    }

    #[doc="Sets the JSTRT field."]
    #[inline] pub fn set_jstrt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Injected channel end of conversion"]
    #[inline] pub fn jeoc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if JEOC != 0"]
    #[inline] pub fn test_jeoc(&self) -> bool {
        self.jeoc() != 0
    }

    #[doc="Sets the JEOC field."]
    #[inline] pub fn set_jeoc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Regular channel end of conversion"]
    #[inline] pub fn eoc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EOC != 0"]
    #[inline] pub fn test_eoc(&self) -> bool {
        self.eoc() != 0
    }

    #[doc="Sets the EOC field."]
    #[inline] pub fn set_eoc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Analog watchdog flag"]
    #[inline] pub fn awd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if AWD != 0"]
    #[inline] pub fn test_awd(&self) -> bool {
        self.awd() != 0
    }

    #[doc="Sets the AWD field."]
    #[inline] pub fn set_awd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.ovr() != 0 { try!(write!(f, " ovr"))}
        if self.strt() != 0 { try!(write!(f, " strt"))}
        if self.jstrt() != 0 { try!(write!(f, " jstrt"))}
        if self.jeoc() != 0 { try!(write!(f, " jeoc"))}
        if self.eoc() != 0 { try!(write!(f, " eoc"))}
        if self.awd() != 0 { try!(write!(f, " awd"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="control register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc="Analog watchdog enable on regular channels"]
    #[inline] pub fn awden(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if AWDEN != 0"]
    #[inline] pub fn test_awden(&self) -> bool {
        self.awden() != 0
    }

    #[doc="Sets the AWDEN field."]
    #[inline] pub fn set_awden<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Analog watchdog enable on injected channels"]
    #[inline] pub fn jawden(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if JAWDEN != 0"]
    #[inline] pub fn test_jawden(&self) -> bool {
        self.jawden() != 0
    }

    #[doc="Sets the JAWDEN field."]
    #[inline] pub fn set_jawden<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Dual mode selection"]
    #[inline] pub fn dualmod(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if DUALMOD != 0"]
    #[inline] pub fn test_dualmod(&self) -> bool {
        self.dualmod() != 0
    }

    #[doc="Sets the DUALMOD field."]
    #[inline] pub fn set_dualmod<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Discontinuous mode channel count"]
    #[inline] pub fn discnum(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x7) as u8) } // [15:13]
    }

    #[doc="Returns true if DISCNUM != 0"]
    #[inline] pub fn test_discnum(&self) -> bool {
        self.discnum() != 0
    }

    #[doc="Sets the DISCNUM field."]
    #[inline] pub fn set_discnum<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Discontinuous mode on injected channels"]
    #[inline] pub fn jdiscen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if JDISCEN != 0"]
    #[inline] pub fn test_jdiscen(&self) -> bool {
        self.jdiscen() != 0
    }

    #[doc="Sets the JDISCEN field."]
    #[inline] pub fn set_jdiscen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Discontinuous mode on regular channels"]
    #[inline] pub fn discen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DISCEN != 0"]
    #[inline] pub fn test_discen(&self) -> bool {
        self.discen() != 0
    }

    #[doc="Sets the DISCEN field."]
    #[inline] pub fn set_discen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Automatic injected group conversion"]
    #[inline] pub fn jauto(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if JAUTO != 0"]
    #[inline] pub fn test_jauto(&self) -> bool {
        self.jauto() != 0
    }

    #[doc="Sets the JAUTO field."]
    #[inline] pub fn set_jauto<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Enable the watchdog on a single channel in scan mode"]
    #[inline] pub fn awdsgl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if AWDSGL != 0"]
    #[inline] pub fn test_awdsgl(&self) -> bool {
        self.awdsgl() != 0
    }

    #[doc="Sets the AWDSGL field."]
    #[inline] pub fn set_awdsgl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Scan mode"]
    #[inline] pub fn scan(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SCAN != 0"]
    #[inline] pub fn test_scan(&self) -> bool {
        self.scan() != 0
    }

    #[doc="Sets the SCAN field."]
    #[inline] pub fn set_scan<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Interrupt enable for injected channels"]
    #[inline] pub fn jeocie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if JEOCIE != 0"]
    #[inline] pub fn test_jeocie(&self) -> bool {
        self.jeocie() != 0
    }

    #[doc="Sets the JEOCIE field."]
    #[inline] pub fn set_jeocie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Analog watchdog interrupt enable"]
    #[inline] pub fn awdie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if AWDIE != 0"]
    #[inline] pub fn test_awdie(&self) -> bool {
        self.awdie() != 0
    }

    #[doc="Sets the AWDIE field."]
    #[inline] pub fn set_awdie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Interrupt enable for EOC"]
    #[inline] pub fn eocie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if EOCIE != 0"]
    #[inline] pub fn test_eocie(&self) -> bool {
        self.eocie() != 0
    }

    #[doc="Sets the EOCIE field."]
    #[inline] pub fn set_eocie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Analog watchdog channel select bits"]
    #[inline] pub fn awdch(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if AWDCH != 0"]
    #[inline] pub fn test_awdch(&self) -> bool {
        self.awdch() != 0
    }

    #[doc="Sets the AWDCH field."]
    #[inline] pub fn set_awdch<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
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
        if self.awden() != 0 { try!(write!(f, " awden"))}
        if self.jawden() != 0 { try!(write!(f, " jawden"))}
        if self.dualmod() != 0 { try!(write!(f, " dualmod=0x{:x}", self.dualmod()))}
        if self.discnum() != 0 { try!(write!(f, " discnum=0x{:x}", self.discnum()))}
        if self.jdiscen() != 0 { try!(write!(f, " jdiscen"))}
        if self.discen() != 0 { try!(write!(f, " discen"))}
        if self.jauto() != 0 { try!(write!(f, " jauto"))}
        if self.awdsgl() != 0 { try!(write!(f, " awdsgl"))}
        if self.scan() != 0 { try!(write!(f, " scan"))}
        if self.jeocie() != 0 { try!(write!(f, " jeocie"))}
        if self.awdie() != 0 { try!(write!(f, " awdie"))}
        if self.eocie() != 0 { try!(write!(f, " eocie"))}
        if self.awdch() != 0 { try!(write!(f, " awdch=0x{:x}", self.awdch()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="control register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr2(pub u32);
impl Cr2 {
    #[doc="Temperature sensor and Vrefint enable"]
    #[inline] pub fn tsvrefe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if TSVREFE != 0"]
    #[inline] pub fn test_tsvrefe(&self) -> bool {
        self.tsvrefe() != 0
    }

    #[doc="Sets the TSVREFE field."]
    #[inline] pub fn set_tsvrefe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Start conversion of regular channels"]
    #[inline] pub fn swstart(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if SWSTART != 0"]
    #[inline] pub fn test_swstart(&self) -> bool {
        self.swstart() != 0
    }

    #[doc="Sets the SWSTART field."]
    #[inline] pub fn set_swstart<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Start conversion of injected channels"]
    #[inline] pub fn jswstart(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if JSWSTART != 0"]
    #[inline] pub fn test_jswstart(&self) -> bool {
        self.jswstart() != 0
    }

    #[doc="Sets the JSWSTART field."]
    #[inline] pub fn set_jswstart<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="External trigger conversion mode for regular channels"]
    #[inline] pub fn exttrig(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if EXTTRIG != 0"]
    #[inline] pub fn test_exttrig(&self) -> bool {
        self.exttrig() != 0
    }

    #[doc="Sets the EXTTRIG field."]
    #[inline] pub fn set_exttrig<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="External event select for regular group"]
    #[inline] pub fn extsel(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x7) as u8) } // [19:17]
    }

    #[doc="Returns true if EXTSEL != 0"]
    #[inline] pub fn test_extsel(&self) -> bool {
        self.extsel() != 0
    }

    #[doc="Sets the EXTSEL field."]
    #[inline] pub fn set_extsel<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="External trigger conversion mode for injected channels"]
    #[inline] pub fn jexttrig(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if JEXTTRIG != 0"]
    #[inline] pub fn test_jexttrig(&self) -> bool {
        self.jexttrig() != 0
    }

    #[doc="Sets the JEXTTRIG field."]
    #[inline] pub fn set_jexttrig<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="External event select for injected group"]
    #[inline] pub fn jextsel(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if JEXTSEL != 0"]
    #[inline] pub fn test_jextsel(&self) -> bool {
        self.jextsel() != 0
    }

    #[doc="Sets the JEXTSEL field."]
    #[inline] pub fn set_jextsel<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Data alignment"]
    #[inline] pub fn align(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if ALIGN != 0"]
    #[inline] pub fn test_align(&self) -> bool {
        self.align() != 0
    }

    #[doc="Sets the ALIGN field."]
    #[inline] pub fn set_align<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Direct memory access mode (for single ADC mode)"]
    #[inline] pub fn dma(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DMA != 0"]
    #[inline] pub fn test_dma(&self) -> bool {
        self.dma() != 0
    }

    #[doc="Sets the DMA field."]
    #[inline] pub fn set_dma<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Reset Calibration"]
    #[inline] pub fn rstcal(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RSTCAL != 0"]
    #[inline] pub fn test_rstcal(&self) -> bool {
        self.rstcal() != 0
    }

    #[doc="Sets the RSTCAL field."]
    #[inline] pub fn set_rstcal<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="A/D Calibration"]
    #[inline] pub fn cal(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CAL != 0"]
    #[inline] pub fn test_cal(&self) -> bool {
        self.cal() != 0
    }

    #[doc="Sets the CAL field."]
    #[inline] pub fn set_cal<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Continuous conversion"]
    #[inline] pub fn cont(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CONT != 0"]
    #[inline] pub fn test_cont(&self) -> bool {
        self.cont() != 0
    }

    #[doc="Sets the CONT field."]
    #[inline] pub fn set_cont<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="A/D Converter ON / OFF"]
    #[inline] pub fn adon(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ADON != 0"]
    #[inline] pub fn test_adon(&self) -> bool {
        self.adon() != 0
    }

    #[doc="Sets the ADON field."]
    #[inline] pub fn set_adon<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.tsvrefe() != 0 { try!(write!(f, " tsvrefe"))}
        if self.swstart() != 0 { try!(write!(f, " swstart"))}
        if self.jswstart() != 0 { try!(write!(f, " jswstart"))}
        if self.exttrig() != 0 { try!(write!(f, " exttrig"))}
        if self.extsel() != 0 { try!(write!(f, " extsel=0x{:x}", self.extsel()))}
        if self.jexttrig() != 0 { try!(write!(f, " jexttrig"))}
        if self.jextsel() != 0 { try!(write!(f, " jextsel=0x{:x}", self.jextsel()))}
        if self.align() != 0 { try!(write!(f, " align"))}
        if self.dma() != 0 { try!(write!(f, " dma"))}
        if self.rstcal() != 0 { try!(write!(f, " rstcal"))}
        if self.cal() != 0 { try!(write!(f, " cal"))}
        if self.cont() != 0 { try!(write!(f, " cont"))}
        if self.adon() != 0 { try!(write!(f, " adon"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="sample time register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Smpr1(pub u32);
impl Smpr1 {
    #[doc="Sample time bits"]
    #[inline] pub fn smp<I: Into<::bobbin_bits::R8>>(&self, index: I) -> ::bobbin_bits::U3 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index * 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if SMP != 0"]
    #[inline] pub fn test_smp<I: Into<::bobbin_bits::R8>>(&self, index: I) -> bool{
        self.smp(index) != 0
    }

    #[doc="Sets the SMP field."]
    #[inline] pub fn set_smp<I: Into<::bobbin_bits::R8>, V: Into<::bobbin_bits::U3>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index * 3);
        self.0 &= !(0x7 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Smpr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Smpr1(other)
    }
}

impl ::core::fmt::Display for Smpr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Smpr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.smp(0) != 0 { try!(write!(f, " smp[0]=0x{:x}", self.smp(0)))}
        if self.smp(1) != 0 { try!(write!(f, " smp[1]=0x{:x}", self.smp(1)))}
        if self.smp(2) != 0 { try!(write!(f, " smp[2]=0x{:x}", self.smp(2)))}
        if self.smp(3) != 0 { try!(write!(f, " smp[3]=0x{:x}", self.smp(3)))}
        if self.smp(4) != 0 { try!(write!(f, " smp[4]=0x{:x}", self.smp(4)))}
        if self.smp(5) != 0 { try!(write!(f, " smp[5]=0x{:x}", self.smp(5)))}
        if self.smp(6) != 0 { try!(write!(f, " smp[6]=0x{:x}", self.smp(6)))}
        if self.smp(7) != 0 { try!(write!(f, " smp[7]=0x{:x}", self.smp(7)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="sample time register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Smpr2(pub u32);
impl Smpr2 {
    #[doc="Sample time bits"]
    #[inline] pub fn smp<I: Into<::bobbin_bits::R9>>(&self, index: I) -> ::bobbin_bits::U3 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index * 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if SMP != 0"]
    #[inline] pub fn test_smp<I: Into<::bobbin_bits::R9>>(&self, index: I) -> bool{
        self.smp(index) != 0
    }

    #[doc="Sets the SMP field."]
    #[inline] pub fn set_smp<I: Into<::bobbin_bits::R9>, V: Into<::bobbin_bits::U3>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index * 3);
        self.0 &= !(0x7 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Smpr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Smpr2(other)
    }
}

impl ::core::fmt::Display for Smpr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Smpr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.smp(0) != 0 { try!(write!(f, " smp[0]=0x{:x}", self.smp(0)))}
        if self.smp(1) != 0 { try!(write!(f, " smp[1]=0x{:x}", self.smp(1)))}
        if self.smp(2) != 0 { try!(write!(f, " smp[2]=0x{:x}", self.smp(2)))}
        if self.smp(3) != 0 { try!(write!(f, " smp[3]=0x{:x}", self.smp(3)))}
        if self.smp(4) != 0 { try!(write!(f, " smp[4]=0x{:x}", self.smp(4)))}
        if self.smp(5) != 0 { try!(write!(f, " smp[5]=0x{:x}", self.smp(5)))}
        if self.smp(6) != 0 { try!(write!(f, " smp[6]=0x{:x}", self.smp(6)))}
        if self.smp(7) != 0 { try!(write!(f, " smp[7]=0x{:x}", self.smp(7)))}
        if self.smp(8) != 0 { try!(write!(f, " smp[8]=0x{:x}", self.smp(8)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="injected channel data offset register x"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Jofr(pub u32);
impl Jofr {
    #[doc="Data offset for injected channel x"]
    #[inline] pub fn joffset(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if JOFFSET != 0"]
    #[inline] pub fn test_joffset(&self) -> bool {
        self.joffset() != 0
    }

    #[doc="Sets the JOFFSET field."]
    #[inline] pub fn set_joffset<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Jofr {
    #[inline]
    fn from(other: u32) -> Self {
         Jofr(other)
    }
}

impl ::core::fmt::Display for Jofr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Jofr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.joffset() != 0 { try!(write!(f, " joffset=0x{:x}", self.joffset()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="watchdog higher threshold register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Htr(pub u32);
impl Htr {
    #[doc="Analog watchdog higher threshold"]
    #[inline] pub fn ht(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if HT != 0"]
    #[inline] pub fn test_ht(&self) -> bool {
        self.ht() != 0
    }

    #[doc="Sets the HT field."]
    #[inline] pub fn set_ht<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Htr {
    #[inline]
    fn from(other: u32) -> Self {
         Htr(other)
    }
}

impl ::core::fmt::Display for Htr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Htr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ht() != 0 { try!(write!(f, " ht=0x{:x}", self.ht()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="watchdog lower threshold register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ltr(pub u32);
impl Ltr {
    #[doc="Analog watchdog lower threshold"]
    #[inline] pub fn lt(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if LT != 0"]
    #[inline] pub fn test_lt(&self) -> bool {
        self.lt() != 0
    }

    #[doc="Sets the LT field."]
    #[inline] pub fn set_lt<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ltr {
    #[inline]
    fn from(other: u32) -> Self {
         Ltr(other)
    }
}

impl ::core::fmt::Display for Ltr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ltr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lt() != 0 { try!(write!(f, " lt=0x{:x}", self.lt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="regular sequence register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sqr1(pub u32);
impl Sqr1 {
    #[doc="Regular channel sequence length"]
    #[inline] pub fn l(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if L != 0"]
    #[inline] pub fn test_l(&self) -> bool {
        self.l() != 0
    }

    #[doc="Sets the L field."]
    #[inline] pub fn set_l<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="16th conversion in regular sequence"]
    #[inline] pub fn sq16(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1f) as u8) } // [19:15]
    }

    #[doc="Returns true if SQ16 != 0"]
    #[inline] pub fn test_sq16(&self) -> bool {
        self.sq16() != 0
    }

    #[doc="Sets the SQ16 field."]
    #[inline] pub fn set_sq16<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="15th conversion in regular sequence"]
    #[inline] pub fn sq15(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1f) as u8) } // [14:10]
    }

    #[doc="Returns true if SQ15 != 0"]
    #[inline] pub fn test_sq15(&self) -> bool {
        self.sq15() != 0
    }

    #[doc="Sets the SQ15 field."]
    #[inline] pub fn set_sq15<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="14th conversion in regular sequence"]
    #[inline] pub fn sq14(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1f) as u8) } // [9:5]
    }

    #[doc="Returns true if SQ14 != 0"]
    #[inline] pub fn test_sq14(&self) -> bool {
        self.sq14() != 0
    }

    #[doc="Sets the SQ14 field."]
    #[inline] pub fn set_sq14<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="13th conversion in regular sequence"]
    #[inline] pub fn sq13(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if SQ13 != 0"]
    #[inline] pub fn test_sq13(&self) -> bool {
        self.sq13() != 0
    }

    #[doc="Sets the SQ13 field."]
    #[inline] pub fn set_sq13<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sqr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Sqr1(other)
    }
}

impl ::core::fmt::Display for Sqr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sqr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.l() != 0 { try!(write!(f, " l=0x{:x}", self.l()))}
        if self.sq16() != 0 { try!(write!(f, " sq16=0x{:x}", self.sq16()))}
        if self.sq15() != 0 { try!(write!(f, " sq15=0x{:x}", self.sq15()))}
        if self.sq14() != 0 { try!(write!(f, " sq14=0x{:x}", self.sq14()))}
        if self.sq13() != 0 { try!(write!(f, " sq13=0x{:x}", self.sq13()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="regular sequence register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sqr2(pub u32);
impl Sqr2 {
    #[doc="12th conversion in regular sequence"]
    #[inline] pub fn sq12(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1f) as u8) } // [29:25]
    }

    #[doc="Returns true if SQ12 != 0"]
    #[inline] pub fn test_sq12(&self) -> bool {
        self.sq12() != 0
    }

    #[doc="Sets the SQ12 field."]
    #[inline] pub fn set_sq12<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="11th conversion in regular sequence"]
    #[inline] pub fn sq11(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1f) as u8) } // [24:20]
    }

    #[doc="Returns true if SQ11 != 0"]
    #[inline] pub fn test_sq11(&self) -> bool {
        self.sq11() != 0
    }

    #[doc="Sets the SQ11 field."]
    #[inline] pub fn set_sq11<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="10th conversion in regular sequence"]
    #[inline] pub fn sq10(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1f) as u8) } // [19:15]
    }

    #[doc="Returns true if SQ10 != 0"]
    #[inline] pub fn test_sq10(&self) -> bool {
        self.sq10() != 0
    }

    #[doc="Sets the SQ10 field."]
    #[inline] pub fn set_sq10<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="9th conversion in regular sequence"]
    #[inline] pub fn sq9(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1f) as u8) } // [14:10]
    }

    #[doc="Returns true if SQ9 != 0"]
    #[inline] pub fn test_sq9(&self) -> bool {
        self.sq9() != 0
    }

    #[doc="Sets the SQ9 field."]
    #[inline] pub fn set_sq9<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="8th conversion in regular sequence"]
    #[inline] pub fn sq8(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1f) as u8) } // [9:5]
    }

    #[doc="Returns true if SQ8 != 0"]
    #[inline] pub fn test_sq8(&self) -> bool {
        self.sq8() != 0
    }

    #[doc="Sets the SQ8 field."]
    #[inline] pub fn set_sq8<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="7th conversion in regular sequence"]
    #[inline] pub fn sq7(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if SQ7 != 0"]
    #[inline] pub fn test_sq7(&self) -> bool {
        self.sq7() != 0
    }

    #[doc="Sets the SQ7 field."]
    #[inline] pub fn set_sq7<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sqr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Sqr2(other)
    }
}

impl ::core::fmt::Display for Sqr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sqr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sq12() != 0 { try!(write!(f, " sq12=0x{:x}", self.sq12()))}
        if self.sq11() != 0 { try!(write!(f, " sq11=0x{:x}", self.sq11()))}
        if self.sq10() != 0 { try!(write!(f, " sq10=0x{:x}", self.sq10()))}
        if self.sq9() != 0 { try!(write!(f, " sq9=0x{:x}", self.sq9()))}
        if self.sq8() != 0 { try!(write!(f, " sq8=0x{:x}", self.sq8()))}
        if self.sq7() != 0 { try!(write!(f, " sq7=0x{:x}", self.sq7()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="regular sequence register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sqr3(pub u32);
impl Sqr3 {
    #[doc="6th conversion in regular sequence"]
    #[inline] pub fn sq6(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1f) as u8) } // [29:25]
    }

    #[doc="Returns true if SQ6 != 0"]
    #[inline] pub fn test_sq6(&self) -> bool {
        self.sq6() != 0
    }

    #[doc="Sets the SQ6 field."]
    #[inline] pub fn set_sq6<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="5th conversion in regular sequence"]
    #[inline] pub fn sq5(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1f) as u8) } // [24:20]
    }

    #[doc="Returns true if SQ5 != 0"]
    #[inline] pub fn test_sq5(&self) -> bool {
        self.sq5() != 0
    }

    #[doc="Sets the SQ5 field."]
    #[inline] pub fn set_sq5<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="4th conversion in regular sequence"]
    #[inline] pub fn sq4(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1f) as u8) } // [19:15]
    }

    #[doc="Returns true if SQ4 != 0"]
    #[inline] pub fn test_sq4(&self) -> bool {
        self.sq4() != 0
    }

    #[doc="Sets the SQ4 field."]
    #[inline] pub fn set_sq4<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="3rd conversion in regular sequence"]
    #[inline] pub fn sq3(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1f) as u8) } // [14:10]
    }

    #[doc="Returns true if SQ3 != 0"]
    #[inline] pub fn test_sq3(&self) -> bool {
        self.sq3() != 0
    }

    #[doc="Sets the SQ3 field."]
    #[inline] pub fn set_sq3<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="2nd conversion in regular sequence"]
    #[inline] pub fn sq2(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1f) as u8) } // [9:5]
    }

    #[doc="Returns true if SQ2 != 0"]
    #[inline] pub fn test_sq2(&self) -> bool {
        self.sq2() != 0
    }

    #[doc="Sets the SQ2 field."]
    #[inline] pub fn set_sq2<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="1st conversion in regular sequence"]
    #[inline] pub fn sq1(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if SQ1 != 0"]
    #[inline] pub fn test_sq1(&self) -> bool {
        self.sq1() != 0
    }

    #[doc="Sets the SQ1 field."]
    #[inline] pub fn set_sq1<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sqr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Sqr3(other)
    }
}

impl ::core::fmt::Display for Sqr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sqr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sq6() != 0 { try!(write!(f, " sq6=0x{:x}", self.sq6()))}
        if self.sq5() != 0 { try!(write!(f, " sq5=0x{:x}", self.sq5()))}
        if self.sq4() != 0 { try!(write!(f, " sq4=0x{:x}", self.sq4()))}
        if self.sq3() != 0 { try!(write!(f, " sq3=0x{:x}", self.sq3()))}
        if self.sq2() != 0 { try!(write!(f, " sq2=0x{:x}", self.sq2()))}
        if self.sq1() != 0 { try!(write!(f, " sq1=0x{:x}", self.sq1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="injected sequence register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Jsqr(pub u32);
impl Jsqr {
    #[doc="Injected sequence length"]
    #[inline] pub fn jl(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if JL != 0"]
    #[inline] pub fn test_jl(&self) -> bool {
        self.jl() != 0
    }

    #[doc="Sets the JL field."]
    #[inline] pub fn set_jl<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="4th conversion in injected sequence"]
    #[inline] pub fn jsq4(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1f) as u8) } // [19:15]
    }

    #[doc="Returns true if JSQ4 != 0"]
    #[inline] pub fn test_jsq4(&self) -> bool {
        self.jsq4() != 0
    }

    #[doc="Sets the JSQ4 field."]
    #[inline] pub fn set_jsq4<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="3rd conversion in injected sequence"]
    #[inline] pub fn jsq3(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1f) as u8) } // [14:10]
    }

    #[doc="Returns true if JSQ3 != 0"]
    #[inline] pub fn test_jsq3(&self) -> bool {
        self.jsq3() != 0
    }

    #[doc="Sets the JSQ3 field."]
    #[inline] pub fn set_jsq3<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="2nd conversion in injected sequence"]
    #[inline] pub fn jsq2(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1f) as u8) } // [9:5]
    }

    #[doc="Returns true if JSQ2 != 0"]
    #[inline] pub fn test_jsq2(&self) -> bool {
        self.jsq2() != 0
    }

    #[doc="Sets the JSQ2 field."]
    #[inline] pub fn set_jsq2<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="1st conversion in injected sequence"]
    #[inline] pub fn jsq1(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if JSQ1 != 0"]
    #[inline] pub fn test_jsq1(&self) -> bool {
        self.jsq1() != 0
    }

    #[doc="Sets the JSQ1 field."]
    #[inline] pub fn set_jsq1<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Jsqr {
    #[inline]
    fn from(other: u32) -> Self {
         Jsqr(other)
    }
}

impl ::core::fmt::Display for Jsqr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Jsqr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.jl() != 0 { try!(write!(f, " jl=0x{:x}", self.jl()))}
        if self.jsq4() != 0 { try!(write!(f, " jsq4=0x{:x}", self.jsq4()))}
        if self.jsq3() != 0 { try!(write!(f, " jsq3=0x{:x}", self.jsq3()))}
        if self.jsq2() != 0 { try!(write!(f, " jsq2=0x{:x}", self.jsq2()))}
        if self.jsq1() != 0 { try!(write!(f, " jsq1=0x{:x}", self.jsq1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="injected data register x"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Jdr(pub u32);
impl Jdr {
    #[doc="Injected data"]
    #[inline] pub fn jdata(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if JDATA != 0"]
    #[inline] pub fn test_jdata(&self) -> bool {
        self.jdata() != 0
    }

    #[doc="Sets the JDATA field."]
    #[inline] pub fn set_jdata<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Jdr {
    #[inline]
    fn from(other: u32) -> Self {
         Jdr(other)
    }
}

impl ::core::fmt::Display for Jdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Jdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.jdata() != 0 { try!(write!(f, " jdata=0x{:x}", self.jdata()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="regular data register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
    #[doc="Regular data"]
    #[inline] pub fn data(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Regular data (12 bit)"]
    #[inline] pub fn data_12(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if DATA_12 != 0"]
    #[inline] pub fn test_data_12(&self) -> bool {
        self.data_12() != 0
    }

    #[doc="Sets the DATA_12 field."]
    #[inline] pub fn set_data_12<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dr {
    #[inline]
    fn from(other: u32) -> Self {
         Dr(other)
    }
}

impl ::core::fmt::Display for Dr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
        if self.data_12() != 0 { try!(write!(f, " data_12=0x{:x}", self.data_12()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

