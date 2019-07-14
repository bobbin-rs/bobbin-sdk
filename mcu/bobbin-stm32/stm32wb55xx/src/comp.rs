::bobbin_mcu::periph!( COMP, Comp, COMP_PERIPH, CompPeriph, COMP_OWNED, COMP_REF_COUNT, 0x40010200, 0x00, 0x1e);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="COMP Peripheral"]
pub struct CompPeriph(pub usize); 

impl CompPeriph {
    #[doc="Get the COMP1_CSR Register."]
    #[inline] pub fn comp1_csr_reg(&self) -> ::bobbin_mcu::register::Register<Comp1Csr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Comp1Csr, 0x0)
    }

    #[doc="Get the *mut pointer for the COMP1_CSR register."]
    #[inline] pub fn comp1_csr_mut(&self) -> *mut Comp1Csr { 
        self.comp1_csr_reg().ptr()
    }

    #[doc="Get the *const pointer for the COMP1_CSR register."]
    #[inline] pub fn comp1_csr_ptr(&self) -> *const Comp1Csr { 
        self.comp1_csr_reg().ptr()
    }

    #[doc="Read the COMP1_CSR register."]
    #[inline] pub fn comp1_csr(&self) -> Comp1Csr { 
        self.comp1_csr_reg().read()
    }

    #[doc="Write the COMP1_CSR register."]
    #[inline] pub fn write_comp1_csr(&self, value: Comp1Csr) -> &Self { 
        self.comp1_csr_reg().write(value);
        self
    }

    #[doc="Set the COMP1_CSR register."]
    #[inline] pub fn set_comp1_csr<F: FnOnce(Comp1Csr) -> Comp1Csr>(&self, f: F) -> &Self {
        self.comp1_csr_reg().set(f);
        self
    }

    #[doc="Modify the COMP1_CSR register."]
    #[inline] pub fn with_comp1_csr<F: FnOnce(Comp1Csr) -> Comp1Csr>(&self, f: F) -> &Self {
        self.comp1_csr_reg().with(f);
        self
    }

    #[doc="Get the COMP2_CSR Register."]
    #[inline] pub fn comp2_csr_reg(&self) -> ::bobbin_mcu::register::Register<Comp2Csr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Comp2Csr, 0x4)
    }

    #[doc="Get the *mut pointer for the COMP2_CSR register."]
    #[inline] pub fn comp2_csr_mut(&self) -> *mut Comp2Csr { 
        self.comp2_csr_reg().ptr()
    }

    #[doc="Get the *const pointer for the COMP2_CSR register."]
    #[inline] pub fn comp2_csr_ptr(&self) -> *const Comp2Csr { 
        self.comp2_csr_reg().ptr()
    }

    #[doc="Read the COMP2_CSR register."]
    #[inline] pub fn comp2_csr(&self) -> Comp2Csr { 
        self.comp2_csr_reg().read()
    }

    #[doc="Write the COMP2_CSR register."]
    #[inline] pub fn write_comp2_csr(&self, value: Comp2Csr) -> &Self { 
        self.comp2_csr_reg().write(value);
        self
    }

    #[doc="Set the COMP2_CSR register."]
    #[inline] pub fn set_comp2_csr<F: FnOnce(Comp2Csr) -> Comp2Csr>(&self, f: F) -> &Self {
        self.comp2_csr_reg().set(f);
        self
    }

    #[doc="Modify the COMP2_CSR register."]
    #[inline] pub fn with_comp2_csr<F: FnOnce(Comp2Csr) -> Comp2Csr>(&self, f: F) -> &Self {
        self.comp2_csr_reg().with(f);
        self
    }

}

#[doc="Comparator control and status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Comp1Csr(pub u32);
impl Comp1Csr {
    #[doc="Comparator enable"]
    #[inline] pub fn comp1_en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if COMP1_EN != 0"]
    #[inline] pub fn test_comp1_en(&self) -> bool {
        self.comp1_en() != 0
    }

    #[doc="Sets the COMP1_EN field."]
    #[inline] pub fn set_comp1_en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Comparator power mode"]
    #[inline] pub fn comp1_pwrmode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if COMP1_PWRMODE != 0"]
    #[inline] pub fn test_comp1_pwrmode(&self) -> bool {
        self.comp1_pwrmode() != 0
    }

    #[doc="Sets the COMP1_PWRMODE field."]
    #[inline] pub fn set_comp1_pwrmode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Comparator input minus selection"]
    #[inline] pub fn comp1_inmsel(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if COMP1_INMSEL != 0"]
    #[inline] pub fn test_comp1_inmsel(&self) -> bool {
        self.comp1_inmsel() != 0
    }

    #[doc="Sets the COMP1_INMSEL field."]
    #[inline] pub fn set_comp1_inmsel<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Comparator input plus selection"]
    #[inline] pub fn comp1_inpsel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x3) as u8) } // [8:7]
    }

    #[doc="Returns true if COMP1_INPSEL != 0"]
    #[inline] pub fn test_comp1_inpsel(&self) -> bool {
        self.comp1_inpsel() != 0
    }

    #[doc="Sets the COMP1_INPSEL field."]
    #[inline] pub fn set_comp1_inpsel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Comparator output polarity"]
    #[inline] pub fn comp1_polarity(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if COMP1_POLARITY != 0"]
    #[inline] pub fn test_comp1_polarity(&self) -> bool {
        self.comp1_polarity() != 0
    }

    #[doc="Sets the COMP1_POLARITY field."]
    #[inline] pub fn set_comp1_polarity<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Comparator hysteresis"]
    #[inline] pub fn comp1_hyst(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if COMP1_HYST != 0"]
    #[inline] pub fn test_comp1_hyst(&self) -> bool {
        self.comp1_hyst() != 0
    }

    #[doc="Sets the COMP1_HYST field."]
    #[inline] pub fn set_comp1_hyst<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Comparator blanking source"]
    #[inline] pub fn comp1_blanking(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x7) as u8) } // [20:18]
    }

    #[doc="Returns true if COMP1_BLANKING != 0"]
    #[inline] pub fn test_comp1_blanking(&self) -> bool {
        self.comp1_blanking() != 0
    }

    #[doc="Sets the COMP1_BLANKING field."]
    #[inline] pub fn set_comp1_blanking<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Comparator voltage scaler enable"]
    #[inline] pub fn comp1_brgen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if COMP1_BRGEN != 0"]
    #[inline] pub fn test_comp1_brgen(&self) -> bool {
        self.comp1_brgen() != 0
    }

    #[doc="Sets the COMP1_BRGEN field."]
    #[inline] pub fn set_comp1_brgen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Comparator scaler bridge enable"]
    #[inline] pub fn comp1_scalen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if COMP1_SCALEN != 0"]
    #[inline] pub fn test_comp1_scalen(&self) -> bool {
        self.comp1_scalen() != 0
    }

    #[doc="Sets the COMP1_SCALEN field."]
    #[inline] pub fn set_comp1_scalen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Comparator input minus extended selection"]
    #[inline] pub fn comp1_inmesel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x3) as u8) } // [26:25]
    }

    #[doc="Returns true if COMP1_INMESEL != 0"]
    #[inline] pub fn test_comp1_inmesel(&self) -> bool {
        self.comp1_inmesel() != 0
    }

    #[doc="Sets the COMP1_INMESEL field."]
    #[inline] pub fn set_comp1_inmesel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Comparator output level"]
    #[inline] pub fn comp1_value(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if COMP1_VALUE != 0"]
    #[inline] pub fn test_comp1_value(&self) -> bool {
        self.comp1_value() != 0
    }

    #[doc="Sets the COMP1_VALUE field."]
    #[inline] pub fn set_comp1_value<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Comparator lock"]
    #[inline] pub fn comp1_lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if COMP1_LOCK != 0"]
    #[inline] pub fn test_comp1_lock(&self) -> bool {
        self.comp1_lock() != 0
    }

    #[doc="Sets the COMP1_LOCK field."]
    #[inline] pub fn set_comp1_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Comp1Csr {
    #[inline]
    fn from(other: u32) -> Self {
         Comp1Csr(other)
    }
}

impl ::core::fmt::Display for Comp1Csr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Comp1Csr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.comp1_en() != 0 { try!(write!(f, " comp1_en"))}
        if self.comp1_pwrmode() != 0 { try!(write!(f, " comp1_pwrmode=0x{:x}", self.comp1_pwrmode()))}
        if self.comp1_inmsel() != 0 { try!(write!(f, " comp1_inmsel=0x{:x}", self.comp1_inmsel()))}
        if self.comp1_inpsel() != 0 { try!(write!(f, " comp1_inpsel=0x{:x}", self.comp1_inpsel()))}
        if self.comp1_polarity() != 0 { try!(write!(f, " comp1_polarity"))}
        if self.comp1_hyst() != 0 { try!(write!(f, " comp1_hyst=0x{:x}", self.comp1_hyst()))}
        if self.comp1_blanking() != 0 { try!(write!(f, " comp1_blanking=0x{:x}", self.comp1_blanking()))}
        if self.comp1_brgen() != 0 { try!(write!(f, " comp1_brgen"))}
        if self.comp1_scalen() != 0 { try!(write!(f, " comp1_scalen"))}
        if self.comp1_inmesel() != 0 { try!(write!(f, " comp1_inmesel=0x{:x}", self.comp1_inmesel()))}
        if self.comp1_value() != 0 { try!(write!(f, " comp1_value"))}
        if self.comp1_lock() != 0 { try!(write!(f, " comp1_lock"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Comparator 2 control and status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Comp2Csr(pub u32);
impl Comp2Csr {
    #[doc="Comparator 2 enable bit"]
    #[inline] pub fn comp2_en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if COMP2_EN != 0"]
    #[inline] pub fn test_comp2_en(&self) -> bool {
        self.comp2_en() != 0
    }

    #[doc="Sets the COMP2_EN field."]
    #[inline] pub fn set_comp2_en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Power Mode of the comparator 2"]
    #[inline] pub fn comp2_pwrmode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if COMP2_PWRMODE != 0"]
    #[inline] pub fn test_comp2_pwrmode(&self) -> bool {
        self.comp2_pwrmode() != 0
    }

    #[doc="Sets the COMP2_PWRMODE field."]
    #[inline] pub fn set_comp2_pwrmode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Comparator 2 input minus selection bits"]
    #[inline] pub fn comp2_inmsel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if COMP2_INMSEL != 0"]
    #[inline] pub fn test_comp2_inmsel(&self) -> bool {
        self.comp2_inmsel() != 0
    }

    #[doc="Sets the COMP2_INMSEL field."]
    #[inline] pub fn set_comp2_inmsel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Comparator 1 input plus selection bit"]
    #[inline] pub fn comp2_inpsel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x3) as u8) } // [8:7]
    }

    #[doc="Returns true if COMP2_INPSEL != 0"]
    #[inline] pub fn test_comp2_inpsel(&self) -> bool {
        self.comp2_inpsel() != 0
    }

    #[doc="Sets the COMP2_INPSEL field."]
    #[inline] pub fn set_comp2_inpsel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Windows mode selection bit"]
    #[inline] pub fn comp2_winmode(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if COMP2_WINMODE != 0"]
    #[inline] pub fn test_comp2_winmode(&self) -> bool {
        self.comp2_winmode() != 0
    }

    #[doc="Sets the COMP2_WINMODE field."]
    #[inline] pub fn set_comp2_winmode<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Comparator 2 polarity selection bit"]
    #[inline] pub fn comp2_polarity(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if COMP2_POLARITY != 0"]
    #[inline] pub fn test_comp2_polarity(&self) -> bool {
        self.comp2_polarity() != 0
    }

    #[doc="Sets the COMP2_POLARITY field."]
    #[inline] pub fn set_comp2_polarity<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Comparator 2 hysteresis selection bits"]
    #[inline] pub fn comp2_hyst(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if COMP2_HYST != 0"]
    #[inline] pub fn test_comp2_hyst(&self) -> bool {
        self.comp2_hyst() != 0
    }

    #[doc="Sets the COMP2_HYST field."]
    #[inline] pub fn set_comp2_hyst<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Comparator 2 blanking source selection bits"]
    #[inline] pub fn comp2_blanking(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x7) as u8) } // [20:18]
    }

    #[doc="Returns true if COMP2_BLANKING != 0"]
    #[inline] pub fn test_comp2_blanking(&self) -> bool {
        self.comp2_blanking() != 0
    }

    #[doc="Sets the COMP2_BLANKING field."]
    #[inline] pub fn set_comp2_blanking<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Scaler bridge enable"]
    #[inline] pub fn comp2_brgen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if COMP2_BRGEN != 0"]
    #[inline] pub fn test_comp2_brgen(&self) -> bool {
        self.comp2_brgen() != 0
    }

    #[doc="Sets the COMP2_BRGEN field."]
    #[inline] pub fn set_comp2_brgen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Voltage scaler enable bit"]
    #[inline] pub fn comp2_scalen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if COMP2_SCALEN != 0"]
    #[inline] pub fn test_comp2_scalen(&self) -> bool {
        self.comp2_scalen() != 0
    }

    #[doc="Sets the COMP2_SCALEN field."]
    #[inline] pub fn set_comp2_scalen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="comparator 2 input minus extended selection bits."]
    #[inline] pub fn comp2_inmesel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x3) as u8) } // [26:25]
    }

    #[doc="Returns true if COMP2_INMESEL != 0"]
    #[inline] pub fn test_comp2_inmesel(&self) -> bool {
        self.comp2_inmesel() != 0
    }

    #[doc="Sets the COMP2_INMESEL field."]
    #[inline] pub fn set_comp2_inmesel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Comparator 2 output status bit"]
    #[inline] pub fn comp2_value(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if COMP2_VALUE != 0"]
    #[inline] pub fn test_comp2_value(&self) -> bool {
        self.comp2_value() != 0
    }

    #[doc="Sets the COMP2_VALUE field."]
    #[inline] pub fn set_comp2_value<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="CSR register lock bit"]
    #[inline] pub fn comp2_lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if COMP2_LOCK != 0"]
    #[inline] pub fn test_comp2_lock(&self) -> bool {
        self.comp2_lock() != 0
    }

    #[doc="Sets the COMP2_LOCK field."]
    #[inline] pub fn set_comp2_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Comp2Csr {
    #[inline]
    fn from(other: u32) -> Self {
         Comp2Csr(other)
    }
}

impl ::core::fmt::Display for Comp2Csr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Comp2Csr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.comp2_en() != 0 { try!(write!(f, " comp2_en"))}
        if self.comp2_pwrmode() != 0 { try!(write!(f, " comp2_pwrmode=0x{:x}", self.comp2_pwrmode()))}
        if self.comp2_inmsel() != 0 { try!(write!(f, " comp2_inmsel=0x{:x}", self.comp2_inmsel()))}
        if self.comp2_inpsel() != 0 { try!(write!(f, " comp2_inpsel=0x{:x}", self.comp2_inpsel()))}
        if self.comp2_winmode() != 0 { try!(write!(f, " comp2_winmode"))}
        if self.comp2_polarity() != 0 { try!(write!(f, " comp2_polarity"))}
        if self.comp2_hyst() != 0 { try!(write!(f, " comp2_hyst=0x{:x}", self.comp2_hyst()))}
        if self.comp2_blanking() != 0 { try!(write!(f, " comp2_blanking=0x{:x}", self.comp2_blanking()))}
        if self.comp2_brgen() != 0 { try!(write!(f, " comp2_brgen"))}
        if self.comp2_scalen() != 0 { try!(write!(f, " comp2_scalen"))}
        if self.comp2_inmesel() != 0 { try!(write!(f, " comp2_inmesel=0x{:x}", self.comp2_inmesel()))}
        if self.comp2_value() != 0 { try!(write!(f, " comp2_value"))}
        if self.comp2_lock() != 0 { try!(write!(f, " comp2_lock"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

