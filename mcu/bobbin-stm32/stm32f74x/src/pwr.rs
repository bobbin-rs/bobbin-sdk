
::bobbin_mcu::periph!( PWR, Pwr, PWR_PERIPH, PwrPeriph, PWR_OWNED, PWR_REF_COUNT, 0x40007000, 0x00, 0x02);


#[doc="Power control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PwrPeriph(pub usize);
impl PwrPeriph {
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

    #[doc="Get the CSR1 Register."]
    #[inline] pub fn csr1_reg(&self) -> ::bobbin_mcu::register::Register<Csr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr1, 0x4)
    }

    #[doc="Get the *mut pointer for the CSR1 register."]
    #[inline] pub fn csr1_mut(&self) -> *mut Csr1 { 
        self.csr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR1 register."]
    #[inline] pub fn csr1_ptr(&self) -> *const Csr1 { 
        self.csr1_reg().ptr()
    }

    #[doc="Read the CSR1 register."]
    #[inline] pub fn csr1(&self) -> Csr1 { 
        self.csr1_reg().read()
    }

    #[doc="Write the CSR1 register."]
    #[inline] pub fn write_csr1(&self, value: Csr1) -> &Self { 
        self.csr1_reg().write(value);
        self
    }

    #[doc="Set the CSR1 register."]
    #[inline] pub fn set_csr1<F: FnOnce(Csr1) -> Csr1>(&self, f: F) -> &Self {
        self.csr1_reg().set(f);
        self
    }

    #[doc="Modify the CSR1 register."]
    #[inline] pub fn with_csr1<F: FnOnce(Csr1) -> Csr1>(&self, f: F) -> &Self {
        self.csr1_reg().with(f);
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

    #[doc="Get the CSR2 Register."]
    #[inline] pub fn csr2_reg(&self) -> ::bobbin_mcu::register::Register<Csr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr2, 0xc)
    }

    #[doc="Get the *mut pointer for the CSR2 register."]
    #[inline] pub fn csr2_mut(&self) -> *mut Csr2 { 
        self.csr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR2 register."]
    #[inline] pub fn csr2_ptr(&self) -> *const Csr2 { 
        self.csr2_reg().ptr()
    }

    #[doc="Read the CSR2 register."]
    #[inline] pub fn csr2(&self) -> Csr2 { 
        self.csr2_reg().read()
    }

    #[doc="Write the CSR2 register."]
    #[inline] pub fn write_csr2(&self, value: Csr2) -> &Self { 
        self.csr2_reg().write(value);
        self
    }

    #[doc="Set the CSR2 register."]
    #[inline] pub fn set_csr2<F: FnOnce(Csr2) -> Csr2>(&self, f: F) -> &Self {
        self.csr2_reg().set(f);
        self
    }

    #[doc="Modify the CSR2 register."]
    #[inline] pub fn with_csr2<F: FnOnce(Csr2) -> Csr2>(&self, f: F) -> &Self {
        self.csr2_reg().with(f);
        self
    }

}

#[doc="power control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc="Low-power deep sleep"]
    #[inline] pub fn lpds(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LPDS != 0"]
    #[inline] pub fn test_lpds(&self) -> bool {
        self.lpds() != 0
    }

    #[doc="Sets the LPDS field."]
    #[inline] pub fn set_lpds<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Power down deepsleep"]
    #[inline] pub fn pdds(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PDDS != 0"]
    #[inline] pub fn test_pdds(&self) -> bool {
        self.pdds() != 0
    }

    #[doc="Sets the PDDS field."]
    #[inline] pub fn set_pdds<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Clear standby flag"]
    #[inline] pub fn csbf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CSBF != 0"]
    #[inline] pub fn test_csbf(&self) -> bool {
        self.csbf() != 0
    }

    #[doc="Sets the CSBF field."]
    #[inline] pub fn set_csbf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Power voltage detector enable"]
    #[inline] pub fn pvde(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PVDE != 0"]
    #[inline] pub fn test_pvde(&self) -> bool {
        self.pvde() != 0
    }

    #[doc="Sets the PVDE field."]
    #[inline] pub fn set_pvde<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="PVD level selection"]
    #[inline] pub fn pls(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7) as u8) } // [7:5]
    }

    #[doc="Returns true if PLS != 0"]
    #[inline] pub fn test_pls(&self) -> bool {
        self.pls() != 0
    }

    #[doc="Sets the PLS field."]
    #[inline] pub fn set_pls<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Disable backup domain write protection"]
    #[inline] pub fn dbp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DBP != 0"]
    #[inline] pub fn test_dbp(&self) -> bool {
        self.dbp() != 0
    }

    #[doc="Sets the DBP field."]
    #[inline] pub fn set_dbp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Flash power down in Stop mode"]
    #[inline] pub fn fpds(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FPDS != 0"]
    #[inline] pub fn test_fpds(&self) -> bool {
        self.fpds() != 0
    }

    #[doc="Sets the FPDS field."]
    #[inline] pub fn set_fpds<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Low-power regulator in deepsleep under-drive mode"]
    #[inline] pub fn lpuds(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if LPUDS != 0"]
    #[inline] pub fn test_lpuds(&self) -> bool {
        self.lpuds() != 0
    }

    #[doc="Sets the LPUDS field."]
    #[inline] pub fn set_lpuds<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Main regulator in deepsleep under-drive mode"]
    #[inline] pub fn mruds(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if MRUDS != 0"]
    #[inline] pub fn test_mruds(&self) -> bool {
        self.mruds() != 0
    }

    #[doc="Sets the MRUDS field."]
    #[inline] pub fn set_mruds<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="ADCDC1"]
    #[inline] pub fn adcdc1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if ADCDC1 != 0"]
    #[inline] pub fn test_adcdc1(&self) -> bool {
        self.adcdc1() != 0
    }

    #[doc="Sets the ADCDC1 field."]
    #[inline] pub fn set_adcdc1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Regulator voltage scaling output selection"]
    #[inline] pub fn vos(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if VOS != 0"]
    #[inline] pub fn test_vos(&self) -> bool {
        self.vos() != 0
    }

    #[doc="Sets the VOS field."]
    #[inline] pub fn set_vos<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Over-drive enable"]
    #[inline] pub fn oden(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if ODEN != 0"]
    #[inline] pub fn test_oden(&self) -> bool {
        self.oden() != 0
    }

    #[doc="Sets the ODEN field."]
    #[inline] pub fn set_oden<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Over-drive switching enabled"]
    #[inline] pub fn odswen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if ODSWEN != 0"]
    #[inline] pub fn test_odswen(&self) -> bool {
        self.odswen() != 0
    }

    #[doc="Sets the ODSWEN field."]
    #[inline] pub fn set_odswen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Under-drive enable in stop mode"]
    #[inline] pub fn uden(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if UDEN != 0"]
    #[inline] pub fn test_uden(&self) -> bool {
        self.uden() != 0
    }

    #[doc="Sets the UDEN field."]
    #[inline] pub fn set_uden<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
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
        if self.lpds() != 0 { try!(write!(f, " lpds"))}
        if self.pdds() != 0 { try!(write!(f, " pdds"))}
        if self.csbf() != 0 { try!(write!(f, " csbf"))}
        if self.pvde() != 0 { try!(write!(f, " pvde"))}
        if self.pls() != 0 { try!(write!(f, " pls=0x{:x}", self.pls()))}
        if self.dbp() != 0 { try!(write!(f, " dbp"))}
        if self.fpds() != 0 { try!(write!(f, " fpds"))}
        if self.lpuds() != 0 { try!(write!(f, " lpuds"))}
        if self.mruds() != 0 { try!(write!(f, " mruds"))}
        if self.adcdc1() != 0 { try!(write!(f, " adcdc1"))}
        if self.vos() != 0 { try!(write!(f, " vos=0x{:x}", self.vos()))}
        if self.oden() != 0 { try!(write!(f, " oden"))}
        if self.odswen() != 0 { try!(write!(f, " odswen"))}
        if self.uden() != 0 { try!(write!(f, " uden=0x{:x}", self.uden()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="power control/status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr1(pub u32);
impl Csr1 {
    #[doc="Wakeup internal flag"]
    #[inline] pub fn wuif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if WUIF != 0"]
    #[inline] pub fn test_wuif(&self) -> bool {
        self.wuif() != 0
    }

    #[doc="Sets the WUIF field."]
    #[inline] pub fn set_wuif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Standby flag"]
    #[inline] pub fn sbf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SBF != 0"]
    #[inline] pub fn test_sbf(&self) -> bool {
        self.sbf() != 0
    }

    #[doc="Sets the SBF field."]
    #[inline] pub fn set_sbf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="PVD output"]
    #[inline] pub fn pvdo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PVDO != 0"]
    #[inline] pub fn test_pvdo(&self) -> bool {
        self.pvdo() != 0
    }

    #[doc="Sets the PVDO field."]
    #[inline] pub fn set_pvdo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Backup regulator ready"]
    #[inline] pub fn brr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if BRR != 0"]
    #[inline] pub fn test_brr(&self) -> bool {
        self.brr() != 0
    }

    #[doc="Sets the BRR field."]
    #[inline] pub fn set_brr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Backup regulator enable"]
    #[inline] pub fn bre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if BRE != 0"]
    #[inline] pub fn test_bre(&self) -> bool {
        self.bre() != 0
    }

    #[doc="Sets the BRE field."]
    #[inline] pub fn set_bre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Regulator voltage scaling output selection ready bit"]
    #[inline] pub fn vosrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if VOSRDY != 0"]
    #[inline] pub fn test_vosrdy(&self) -> bool {
        self.vosrdy() != 0
    }

    #[doc="Sets the VOSRDY field."]
    #[inline] pub fn set_vosrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Over-drive mode ready"]
    #[inline] pub fn odrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if ODRDY != 0"]
    #[inline] pub fn test_odrdy(&self) -> bool {
        self.odrdy() != 0
    }

    #[doc="Sets the ODRDY field."]
    #[inline] pub fn set_odrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Over-drive mode switching ready"]
    #[inline] pub fn odswrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if ODSWRDY != 0"]
    #[inline] pub fn test_odswrdy(&self) -> bool {
        self.odswrdy() != 0
    }

    #[doc="Sets the ODSWRDY field."]
    #[inline] pub fn set_odswrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Under-drive ready flag"]
    #[inline] pub fn udrdy(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if UDRDY != 0"]
    #[inline] pub fn test_udrdy(&self) -> bool {
        self.udrdy() != 0
    }

    #[doc="Sets the UDRDY field."]
    #[inline] pub fn set_udrdy<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

}

impl From<u32> for Csr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr1(other)
    }
}

impl ::core::fmt::Display for Csr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wuif() != 0 { try!(write!(f, " wuif"))}
        if self.sbf() != 0 { try!(write!(f, " sbf"))}
        if self.pvdo() != 0 { try!(write!(f, " pvdo"))}
        if self.brr() != 0 { try!(write!(f, " brr"))}
        if self.bre() != 0 { try!(write!(f, " bre"))}
        if self.vosrdy() != 0 { try!(write!(f, " vosrdy"))}
        if self.odrdy() != 0 { try!(write!(f, " odrdy"))}
        if self.odswrdy() != 0 { try!(write!(f, " odswrdy"))}
        if self.udrdy() != 0 { try!(write!(f, " udrdy=0x{:x}", self.udrdy()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="power control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr2(pub u32);
impl Cr2 {
    #[doc="Clear Wakeup Pin flag for PA0"]
    #[inline] pub fn cwupf1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CWUPF1 != 0"]
    #[inline] pub fn test_cwupf1(&self) -> bool {
        self.cwupf1() != 0
    }

    #[doc="Sets the CWUPF1 field."]
    #[inline] pub fn set_cwupf1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clear Wakeup Pin flag for PA2"]
    #[inline] pub fn cwupf2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CWUPF2 != 0"]
    #[inline] pub fn test_cwupf2(&self) -> bool {
        self.cwupf2() != 0
    }

    #[doc="Sets the CWUPF2 field."]
    #[inline] pub fn set_cwupf2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Clear Wakeup Pin flag for PC1"]
    #[inline] pub fn cwupf3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CWUPF3 != 0"]
    #[inline] pub fn test_cwupf3(&self) -> bool {
        self.cwupf3() != 0
    }

    #[doc="Sets the CWUPF3 field."]
    #[inline] pub fn set_cwupf3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Clear Wakeup Pin flag for PC13"]
    #[inline] pub fn cwupf4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CWUPF4 != 0"]
    #[inline] pub fn test_cwupf4(&self) -> bool {
        self.cwupf4() != 0
    }

    #[doc="Sets the CWUPF4 field."]
    #[inline] pub fn set_cwupf4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Clear Wakeup Pin flag for PI8"]
    #[inline] pub fn cwupf5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CWUPF5 != 0"]
    #[inline] pub fn test_cwupf5(&self) -> bool {
        self.cwupf5() != 0
    }

    #[doc="Sets the CWUPF5 field."]
    #[inline] pub fn set_cwupf5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Clear Wakeup Pin flag for PI11"]
    #[inline] pub fn cwupf6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CWUPF6 != 0"]
    #[inline] pub fn test_cwupf6(&self) -> bool {
        self.cwupf6() != 0
    }

    #[doc="Sets the CWUPF6 field."]
    #[inline] pub fn set_cwupf6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Wakeup pin polarity bit for PA0"]
    #[inline] pub fn wupp1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if WUPP1 != 0"]
    #[inline] pub fn test_wupp1(&self) -> bool {
        self.wupp1() != 0
    }

    #[doc="Sets the WUPP1 field."]
    #[inline] pub fn set_wupp1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Wakeup pin polarity bit for PA2"]
    #[inline] pub fn wupp2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if WUPP2 != 0"]
    #[inline] pub fn test_wupp2(&self) -> bool {
        self.wupp2() != 0
    }

    #[doc="Sets the WUPP2 field."]
    #[inline] pub fn set_wupp2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Wakeup pin polarity bit for PC1"]
    #[inline] pub fn wupp3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if WUPP3 != 0"]
    #[inline] pub fn test_wupp3(&self) -> bool {
        self.wupp3() != 0
    }

    #[doc="Sets the WUPP3 field."]
    #[inline] pub fn set_wupp3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Wakeup pin polarity bit for PC13"]
    #[inline] pub fn wupp4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if WUPP4 != 0"]
    #[inline] pub fn test_wupp4(&self) -> bool {
        self.wupp4() != 0
    }

    #[doc="Sets the WUPP4 field."]
    #[inline] pub fn set_wupp4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Wakeup pin polarity bit for PI8"]
    #[inline] pub fn wupp5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if WUPP5 != 0"]
    #[inline] pub fn test_wupp5(&self) -> bool {
        self.wupp5() != 0
    }

    #[doc="Sets the WUPP5 field."]
    #[inline] pub fn set_wupp5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Wakeup pin polarity bit for PI11"]
    #[inline] pub fn wupp6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if WUPP6 != 0"]
    #[inline] pub fn test_wupp6(&self) -> bool {
        self.wupp6() != 0
    }

    #[doc="Sets the WUPP6 field."]
    #[inline] pub fn set_wupp6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
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
        if self.cwupf1() != 0 { try!(write!(f, " cwupf1"))}
        if self.cwupf2() != 0 { try!(write!(f, " cwupf2"))}
        if self.cwupf3() != 0 { try!(write!(f, " cwupf3"))}
        if self.cwupf4() != 0 { try!(write!(f, " cwupf4"))}
        if self.cwupf5() != 0 { try!(write!(f, " cwupf5"))}
        if self.cwupf6() != 0 { try!(write!(f, " cwupf6"))}
        if self.wupp1() != 0 { try!(write!(f, " wupp1"))}
        if self.wupp2() != 0 { try!(write!(f, " wupp2"))}
        if self.wupp3() != 0 { try!(write!(f, " wupp3"))}
        if self.wupp4() != 0 { try!(write!(f, " wupp4"))}
        if self.wupp5() != 0 { try!(write!(f, " wupp5"))}
        if self.wupp6() != 0 { try!(write!(f, " wupp6"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="power control/status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr2(pub u32);
impl Csr2 {
    #[doc="Wakeup Pin flag for PA0"]
    #[inline] pub fn wupf1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if WUPF1 != 0"]
    #[inline] pub fn test_wupf1(&self) -> bool {
        self.wupf1() != 0
    }

    #[doc="Sets the WUPF1 field."]
    #[inline] pub fn set_wupf1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Wakeup Pin flag for PA2"]
    #[inline] pub fn wupf2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if WUPF2 != 0"]
    #[inline] pub fn test_wupf2(&self) -> bool {
        self.wupf2() != 0
    }

    #[doc="Sets the WUPF2 field."]
    #[inline] pub fn set_wupf2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Wakeup Pin flag for PC1"]
    #[inline] pub fn wupf3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WUPF3 != 0"]
    #[inline] pub fn test_wupf3(&self) -> bool {
        self.wupf3() != 0
    }

    #[doc="Sets the WUPF3 field."]
    #[inline] pub fn set_wupf3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Wakeup Pin flag for PC13"]
    #[inline] pub fn wupf4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if WUPF4 != 0"]
    #[inline] pub fn test_wupf4(&self) -> bool {
        self.wupf4() != 0
    }

    #[doc="Sets the WUPF4 field."]
    #[inline] pub fn set_wupf4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Wakeup Pin flag for PI8"]
    #[inline] pub fn wupf5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if WUPF5 != 0"]
    #[inline] pub fn test_wupf5(&self) -> bool {
        self.wupf5() != 0
    }

    #[doc="Sets the WUPF5 field."]
    #[inline] pub fn set_wupf5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Wakeup Pin flag for PI11"]
    #[inline] pub fn wupf6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if WUPF6 != 0"]
    #[inline] pub fn test_wupf6(&self) -> bool {
        self.wupf6() != 0
    }

    #[doc="Sets the WUPF6 field."]
    #[inline] pub fn set_wupf6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Enable Wakeup pin for PA0"]
    #[inline] pub fn ewup1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if EWUP1 != 0"]
    #[inline] pub fn test_ewup1(&self) -> bool {
        self.ewup1() != 0
    }

    #[doc="Sets the EWUP1 field."]
    #[inline] pub fn set_ewup1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Enable Wakeup pin for PA2"]
    #[inline] pub fn ewup2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if EWUP2 != 0"]
    #[inline] pub fn test_ewup2(&self) -> bool {
        self.ewup2() != 0
    }

    #[doc="Sets the EWUP2 field."]
    #[inline] pub fn set_ewup2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Enable Wakeup pin for PC1"]
    #[inline] pub fn ewup3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if EWUP3 != 0"]
    #[inline] pub fn test_ewup3(&self) -> bool {
        self.ewup3() != 0
    }

    #[doc="Sets the EWUP3 field."]
    #[inline] pub fn set_ewup3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Enable Wakeup pin for PC13"]
    #[inline] pub fn ewup4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if EWUP4 != 0"]
    #[inline] pub fn test_ewup4(&self) -> bool {
        self.ewup4() != 0
    }

    #[doc="Sets the EWUP4 field."]
    #[inline] pub fn set_ewup4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Enable Wakeup pin for PI8"]
    #[inline] pub fn ewup5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if EWUP5 != 0"]
    #[inline] pub fn test_ewup5(&self) -> bool {
        self.ewup5() != 0
    }

    #[doc="Sets the EWUP5 field."]
    #[inline] pub fn set_ewup5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Enable Wakeup pin for PI11"]
    #[inline] pub fn ewup6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if EWUP6 != 0"]
    #[inline] pub fn test_ewup6(&self) -> bool {
        self.ewup6() != 0
    }

    #[doc="Sets the EWUP6 field."]
    #[inline] pub fn set_ewup6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u32> for Csr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr2(other)
    }
}

impl ::core::fmt::Display for Csr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wupf1() != 0 { try!(write!(f, " wupf1"))}
        if self.wupf2() != 0 { try!(write!(f, " wupf2"))}
        if self.wupf3() != 0 { try!(write!(f, " wupf3"))}
        if self.wupf4() != 0 { try!(write!(f, " wupf4"))}
        if self.wupf5() != 0 { try!(write!(f, " wupf5"))}
        if self.wupf6() != 0 { try!(write!(f, " wupf6"))}
        if self.ewup1() != 0 { try!(write!(f, " ewup1"))}
        if self.ewup2() != 0 { try!(write!(f, " ewup2"))}
        if self.ewup3() != 0 { try!(write!(f, " ewup3"))}
        if self.ewup4() != 0 { try!(write!(f, " ewup4"))}
        if self.ewup5() != 0 { try!(write!(f, " ewup5"))}
        if self.ewup6() != 0 { try!(write!(f, " ewup6"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

