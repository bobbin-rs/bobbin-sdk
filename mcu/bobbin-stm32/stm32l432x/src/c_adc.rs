
::bobbin_mcu::periph!( C_ADC, CAdc, C_ADC_PERIPH, CAdcPeriph, C_ADC_OWNED, C_ADC_REF_COUNT, 0x50040300, 0x00, 0x04);


#[doc="Analog-to-Digital Converter"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CAdcPeriph(pub usize);
impl CAdcPeriph {
    #[doc="Get the CSR Register."]
    #[inline] pub fn csr_reg(&self) -> ::bobbin_mcu::register::Register<Csr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr, 0x0)
    }

    #[doc="Get the *mut pointer for the CSR register."]
    #[inline] pub fn csr_mut(&self) -> *mut Csr { 
        self.csr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR register."]
    #[inline] pub fn csr_ptr(&self) -> *const Csr { 
        self.csr_reg().ptr()
    }

    #[doc="Read the CSR register."]
    #[inline] pub fn csr(&self) -> Csr { 
        self.csr_reg().read()
    }

    #[doc="Get the CCR Register."]
    #[inline] pub fn ccr_reg(&self) -> ::bobbin_mcu::register::Register<Ccr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ccr, 0x8)
    }

    #[doc="Get the *mut pointer for the CCR register."]
    #[inline] pub fn ccr_mut(&self) -> *mut Ccr { 
        self.ccr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CCR register."]
    #[inline] pub fn ccr_ptr(&self) -> *const Ccr { 
        self.ccr_reg().ptr()
    }

    #[doc="Read the CCR register."]
    #[inline] pub fn ccr(&self) -> Ccr { 
        self.ccr_reg().read()
    }

    #[doc="Write the CCR register."]
    #[inline] pub fn write_ccr(&self, value: Ccr) -> &Self { 
        self.ccr_reg().write(value);
        self
    }

    #[doc="Set the CCR register."]
    #[inline] pub fn set_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
        self.ccr_reg().set(f);
        self
    }

    #[doc="Modify the CCR register."]
    #[inline] pub fn with_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
        self.ccr_reg().with(f);
        self
    }

    #[doc="Get the CDR Register."]
    #[inline] pub fn cdr_reg(&self) -> ::bobbin_mcu::register::Register<Cdr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cdr, 0xc)
    }

    #[doc="Get the *mut pointer for the CDR register."]
    #[inline] pub fn cdr_mut(&self) -> *mut Cdr { 
        self.cdr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CDR register."]
    #[inline] pub fn cdr_ptr(&self) -> *const Cdr { 
        self.cdr_reg().ptr()
    }

    #[doc="Read the CDR register."]
    #[inline] pub fn cdr(&self) -> Cdr { 
        self.cdr_reg().read()
    }

}

#[doc="ADC Common status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc="ADDRDY_MST"]
    #[inline] pub fn addrdy_mst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ADDRDY_MST != 0"]
    #[inline] pub fn test_addrdy_mst(&self) -> bool {
        self.addrdy_mst() != 0
    }

    #[doc="Sets the ADDRDY_MST field."]
    #[inline] pub fn set_addrdy_mst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="EOSMP_MST"]
    #[inline] pub fn eosmp_mst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EOSMP_MST != 0"]
    #[inline] pub fn test_eosmp_mst(&self) -> bool {
        self.eosmp_mst() != 0
    }

    #[doc="Sets the EOSMP_MST field."]
    #[inline] pub fn set_eosmp_mst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="EOC_MST"]
    #[inline] pub fn eoc_mst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if EOC_MST != 0"]
    #[inline] pub fn test_eoc_mst(&self) -> bool {
        self.eoc_mst() != 0
    }

    #[doc="Sets the EOC_MST field."]
    #[inline] pub fn set_eoc_mst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="EOS_MST"]
    #[inline] pub fn eos_mst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if EOS_MST != 0"]
    #[inline] pub fn test_eos_mst(&self) -> bool {
        self.eos_mst() != 0
    }

    #[doc="Sets the EOS_MST field."]
    #[inline] pub fn set_eos_mst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="OVR_MST"]
    #[inline] pub fn ovr_mst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if OVR_MST != 0"]
    #[inline] pub fn test_ovr_mst(&self) -> bool {
        self.ovr_mst() != 0
    }

    #[doc="Sets the OVR_MST field."]
    #[inline] pub fn set_ovr_mst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="JEOC_MST"]
    #[inline] pub fn jeoc_mst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if JEOC_MST != 0"]
    #[inline] pub fn test_jeoc_mst(&self) -> bool {
        self.jeoc_mst() != 0
    }

    #[doc="Sets the JEOC_MST field."]
    #[inline] pub fn set_jeoc_mst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="JEOS_MST"]
    #[inline] pub fn jeos_mst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if JEOS_MST != 0"]
    #[inline] pub fn test_jeos_mst(&self) -> bool {
        self.jeos_mst() != 0
    }

    #[doc="Sets the JEOS_MST field."]
    #[inline] pub fn set_jeos_mst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="AWD1_MST"]
    #[inline] pub fn awd1_mst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if AWD1_MST != 0"]
    #[inline] pub fn test_awd1_mst(&self) -> bool {
        self.awd1_mst() != 0
    }

    #[doc="Sets the AWD1_MST field."]
    #[inline] pub fn set_awd1_mst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="AWD2_MST"]
    #[inline] pub fn awd2_mst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if AWD2_MST != 0"]
    #[inline] pub fn test_awd2_mst(&self) -> bool {
        self.awd2_mst() != 0
    }

    #[doc="Sets the AWD2_MST field."]
    #[inline] pub fn set_awd2_mst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="AWD3_MST"]
    #[inline] pub fn awd3_mst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if AWD3_MST != 0"]
    #[inline] pub fn test_awd3_mst(&self) -> bool {
        self.awd3_mst() != 0
    }

    #[doc="Sets the AWD3_MST field."]
    #[inline] pub fn set_awd3_mst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="JQOVF_MST"]
    #[inline] pub fn jqovf_mst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if JQOVF_MST != 0"]
    #[inline] pub fn test_jqovf_mst(&self) -> bool {
        self.jqovf_mst() != 0
    }

    #[doc="Sets the JQOVF_MST field."]
    #[inline] pub fn set_jqovf_mst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="ADRDY_SLV"]
    #[inline] pub fn adrdy_slv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if ADRDY_SLV != 0"]
    #[inline] pub fn test_adrdy_slv(&self) -> bool {
        self.adrdy_slv() != 0
    }

    #[doc="Sets the ADRDY_SLV field."]
    #[inline] pub fn set_adrdy_slv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="EOSMP_SLV"]
    #[inline] pub fn eosmp_slv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if EOSMP_SLV != 0"]
    #[inline] pub fn test_eosmp_slv(&self) -> bool {
        self.eosmp_slv() != 0
    }

    #[doc="Sets the EOSMP_SLV field."]
    #[inline] pub fn set_eosmp_slv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="End of regular conversion of the slave ADC"]
    #[inline] pub fn eoc_slv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if EOC_SLV != 0"]
    #[inline] pub fn test_eoc_slv(&self) -> bool {
        self.eoc_slv() != 0
    }

    #[doc="Sets the EOC_SLV field."]
    #[inline] pub fn set_eoc_slv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="End of regular sequence flag of the slave ADC"]
    #[inline] pub fn eos_slv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if EOS_SLV != 0"]
    #[inline] pub fn test_eos_slv(&self) -> bool {
        self.eos_slv() != 0
    }

    #[doc="Sets the EOS_SLV field."]
    #[inline] pub fn set_eos_slv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Overrun flag of the slave ADC"]
    #[inline] pub fn ovr_slv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if OVR_SLV != 0"]
    #[inline] pub fn test_ovr_slv(&self) -> bool {
        self.ovr_slv() != 0
    }

    #[doc="Sets the OVR_SLV field."]
    #[inline] pub fn set_ovr_slv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="End of injected conversion flag of the slave ADC"]
    #[inline] pub fn jeoc_slv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if JEOC_SLV != 0"]
    #[inline] pub fn test_jeoc_slv(&self) -> bool {
        self.jeoc_slv() != 0
    }

    #[doc="Sets the JEOC_SLV field."]
    #[inline] pub fn set_jeoc_slv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="End of injected sequence flag of the slave ADC"]
    #[inline] pub fn jeos_slv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if JEOS_SLV != 0"]
    #[inline] pub fn test_jeos_slv(&self) -> bool {
        self.jeos_slv() != 0
    }

    #[doc="Sets the JEOS_SLV field."]
    #[inline] pub fn set_jeos_slv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Analog watchdog 1 flag of the slave ADC"]
    #[inline] pub fn awd1_slv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if AWD1_SLV != 0"]
    #[inline] pub fn test_awd1_slv(&self) -> bool {
        self.awd1_slv() != 0
    }

    #[doc="Sets the AWD1_SLV field."]
    #[inline] pub fn set_awd1_slv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Analog watchdog 2 flag of the slave ADC"]
    #[inline] pub fn awd2_slv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if AWD2_SLV != 0"]
    #[inline] pub fn test_awd2_slv(&self) -> bool {
        self.awd2_slv() != 0
    }

    #[doc="Sets the AWD2_SLV field."]
    #[inline] pub fn set_awd2_slv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Analog watchdog 3 flag of the slave ADC"]
    #[inline] pub fn awd3_slv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if AWD3_SLV != 0"]
    #[inline] pub fn test_awd3_slv(&self) -> bool {
        self.awd3_slv() != 0
    }

    #[doc="Sets the AWD3_SLV field."]
    #[inline] pub fn set_awd3_slv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Injected Context Queue Overflow flag of the slave ADC"]
    #[inline] pub fn jqovf_slv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if JQOVF_SLV != 0"]
    #[inline] pub fn test_jqovf_slv(&self) -> bool {
        self.jqovf_slv() != 0
    }

    #[doc="Sets the JQOVF_SLV field."]
    #[inline] pub fn set_jqovf_slv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

}

impl From<u32> for Csr {
    #[inline]
    fn from(other: u32) -> Self {
         Csr(other)
    }
}

impl ::core::fmt::Display for Csr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addrdy_mst() != 0 { try!(write!(f, " addrdy_mst"))}
        if self.eosmp_mst() != 0 { try!(write!(f, " eosmp_mst"))}
        if self.eoc_mst() != 0 { try!(write!(f, " eoc_mst"))}
        if self.eos_mst() != 0 { try!(write!(f, " eos_mst"))}
        if self.ovr_mst() != 0 { try!(write!(f, " ovr_mst"))}
        if self.jeoc_mst() != 0 { try!(write!(f, " jeoc_mst"))}
        if self.jeos_mst() != 0 { try!(write!(f, " jeos_mst"))}
        if self.awd1_mst() != 0 { try!(write!(f, " awd1_mst"))}
        if self.awd2_mst() != 0 { try!(write!(f, " awd2_mst"))}
        if self.awd3_mst() != 0 { try!(write!(f, " awd3_mst"))}
        if self.jqovf_mst() != 0 { try!(write!(f, " jqovf_mst"))}
        if self.adrdy_slv() != 0 { try!(write!(f, " adrdy_slv"))}
        if self.eosmp_slv() != 0 { try!(write!(f, " eosmp_slv"))}
        if self.eoc_slv() != 0 { try!(write!(f, " eoc_slv"))}
        if self.eos_slv() != 0 { try!(write!(f, " eos_slv"))}
        if self.ovr_slv() != 0 { try!(write!(f, " ovr_slv"))}
        if self.jeoc_slv() != 0 { try!(write!(f, " jeoc_slv"))}
        if self.jeos_slv() != 0 { try!(write!(f, " jeos_slv"))}
        if self.awd1_slv() != 0 { try!(write!(f, " awd1_slv"))}
        if self.awd2_slv() != 0 { try!(write!(f, " awd2_slv"))}
        if self.awd3_slv() != 0 { try!(write!(f, " awd3_slv"))}
        if self.jqovf_slv() != 0 { try!(write!(f, " jqovf_slv"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC common control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccr(pub u32);
impl Ccr {
    #[doc="Multi ADC mode selection"]
    #[inline] pub fn mult(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if MULT != 0"]
    #[inline] pub fn test_mult(&self) -> bool {
        self.mult() != 0
    }

    #[doc="Sets the MULT field."]
    #[inline] pub fn set_mult<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Delay between 2 sampling phases"]
    #[inline] pub fn delay(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if DELAY != 0"]
    #[inline] pub fn test_delay(&self) -> bool {
        self.delay() != 0
    }

    #[doc="Sets the DELAY field."]
    #[inline] pub fn set_delay<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA configuration (for multi-ADC mode)"]
    #[inline] pub fn dmacfg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if DMACFG != 0"]
    #[inline] pub fn test_dmacfg(&self) -> bool {
        self.dmacfg() != 0
    }

    #[doc="Sets the DMACFG field."]
    #[inline] pub fn set_dmacfg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Direct memory access mode for multi ADC mode"]
    #[inline] pub fn mdma(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if MDMA != 0"]
    #[inline] pub fn test_mdma(&self) -> bool {
        self.mdma() != 0
    }

    #[doc="Sets the MDMA field."]
    #[inline] pub fn set_mdma<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="ADC clock mode"]
    #[inline] pub fn ckmode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if CKMODE != 0"]
    #[inline] pub fn test_ckmode(&self) -> bool {
        self.ckmode() != 0
    }

    #[doc="Sets the CKMODE field."]
    #[inline] pub fn set_ckmode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="VREFINT enable"]
    #[inline] pub fn vrefen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if VREFEN != 0"]
    #[inline] pub fn test_vrefen(&self) -> bool {
        self.vrefen() != 0
    }

    #[doc="Sets the VREFEN field."]
    #[inline] pub fn set_vrefen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Temperature sensor enable"]
    #[inline] pub fn tsen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if TSEN != 0"]
    #[inline] pub fn test_tsen(&self) -> bool {
        self.tsen() != 0
    }

    #[doc="Sets the TSEN field."]
    #[inline] pub fn set_tsen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="VBAT enable"]
    #[inline] pub fn vbaten(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if VBATEN != 0"]
    #[inline] pub fn test_vbaten(&self) -> bool {
        self.vbaten() != 0
    }

    #[doc="Sets the VBATEN field."]
    #[inline] pub fn set_vbaten<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ccr {
    #[inline]
    fn from(other: u32) -> Self {
         Ccr(other)
    }
}

impl ::core::fmt::Display for Ccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mult() != 0 { try!(write!(f, " mult=0x{:x}", self.mult()))}
        if self.delay() != 0 { try!(write!(f, " delay=0x{:x}", self.delay()))}
        if self.dmacfg() != 0 { try!(write!(f, " dmacfg"))}
        if self.mdma() != 0 { try!(write!(f, " mdma=0x{:x}", self.mdma()))}
        if self.ckmode() != 0 { try!(write!(f, " ckmode=0x{:x}", self.ckmode()))}
        if self.vrefen() != 0 { try!(write!(f, " vrefen"))}
        if self.tsen() != 0 { try!(write!(f, " tsen"))}
        if self.vbaten() != 0 { try!(write!(f, " vbaten"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC common regular data register for dual and triple modes"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cdr(pub u32);
impl Cdr {
    #[doc="Regular data of the slave ADC"]
    #[inline] pub fn rdata_slv(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if RDATA_SLV != 0"]
    #[inline] pub fn test_rdata_slv(&self) -> bool {
        self.rdata_slv() != 0
    }

    #[doc="Sets the RDATA_SLV field."]
    #[inline] pub fn set_rdata_slv<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Regular data of the master ADC"]
    #[inline] pub fn rdata_mst(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if RDATA_MST != 0"]
    #[inline] pub fn test_rdata_mst(&self) -> bool {
        self.rdata_mst() != 0
    }

    #[doc="Sets the RDATA_MST field."]
    #[inline] pub fn set_rdata_mst<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cdr {
    #[inline]
    fn from(other: u32) -> Self {
         Cdr(other)
    }
}

impl ::core::fmt::Display for Cdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rdata_slv() != 0 { try!(write!(f, " rdata_slv=0x{:x}", self.rdata_slv()))}
        if self.rdata_mst() != 0 { try!(write!(f, " rdata_mst=0x{:x}", self.rdata_mst()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

