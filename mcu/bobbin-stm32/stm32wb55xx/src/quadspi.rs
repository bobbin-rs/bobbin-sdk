::bobbin_mcu::periph!( QUADSPI, Quadspi, QUADSPI_PERIPH, QuadspiPeriph, QUADSPI_OWNED, QUADSPI_REF_COUNT, 0xa0001000, 0x00, 0x1f);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="QUADSPI Peripheral"]
pub struct QuadspiPeriph(pub usize); 

impl QuadspiPeriph {
    #[doc="Get the CR Register."]
    #[inline] pub fn cr_reg(&self) -> ::bobbin_mcu::register::Register<Cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr, 0x0)
    }

    #[doc="Get the *mut pointer for the CR register."]
    #[inline] pub fn cr_mut(&self) -> *mut Cr { 
        self.cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR register."]
    #[inline] pub fn cr_ptr(&self) -> *const Cr { 
        self.cr_reg().ptr()
    }

    #[doc="Read the CR register."]
    #[inline] pub fn cr(&self) -> Cr { 
        self.cr_reg().read()
    }

    #[doc="Write the CR register."]
    #[inline] pub fn write_cr(&self, value: Cr) -> &Self { 
        self.cr_reg().write(value);
        self
    }

    #[doc="Set the CR register."]
    #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        self.cr_reg().set(f);
        self
    }

    #[doc="Modify the CR register."]
    #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        self.cr_reg().with(f);
        self
    }

    #[doc="Get the DCR Register."]
    #[inline] pub fn dcr_reg(&self) -> ::bobbin_mcu::register::Register<Dcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dcr, 0x4)
    }

    #[doc="Get the *mut pointer for the DCR register."]
    #[inline] pub fn dcr_mut(&self) -> *mut Dcr { 
        self.dcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DCR register."]
    #[inline] pub fn dcr_ptr(&self) -> *const Dcr { 
        self.dcr_reg().ptr()
    }

    #[doc="Read the DCR register."]
    #[inline] pub fn dcr(&self) -> Dcr { 
        self.dcr_reg().read()
    }

    #[doc="Write the DCR register."]
    #[inline] pub fn write_dcr(&self, value: Dcr) -> &Self { 
        self.dcr_reg().write(value);
        self
    }

    #[doc="Set the DCR register."]
    #[inline] pub fn set_dcr<F: FnOnce(Dcr) -> Dcr>(&self, f: F) -> &Self {
        self.dcr_reg().set(f);
        self
    }

    #[doc="Modify the DCR register."]
    #[inline] pub fn with_dcr<F: FnOnce(Dcr) -> Dcr>(&self, f: F) -> &Self {
        self.dcr_reg().with(f);
        self
    }

    #[doc="Get the SR Register."]
    #[inline] pub fn sr_reg(&self) -> ::bobbin_mcu::register::Register<Sr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sr, 0x8)
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

    #[doc="Get the FCR Register."]
    #[inline] pub fn fcr_reg(&self) -> ::bobbin_mcu::register::Register<Fcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fcr, 0xc)
    }

    #[doc="Get the *mut pointer for the FCR register."]
    #[inline] pub fn fcr_mut(&self) -> *mut Fcr { 
        self.fcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the FCR register."]
    #[inline] pub fn fcr_ptr(&self) -> *const Fcr { 
        self.fcr_reg().ptr()
    }

    #[doc="Read the FCR register."]
    #[inline] pub fn fcr(&self) -> Fcr { 
        self.fcr_reg().read()
    }

    #[doc="Write the FCR register."]
    #[inline] pub fn write_fcr(&self, value: Fcr) -> &Self { 
        self.fcr_reg().write(value);
        self
    }

    #[doc="Set the FCR register."]
    #[inline] pub fn set_fcr<F: FnOnce(Fcr) -> Fcr>(&self, f: F) -> &Self {
        self.fcr_reg().set(f);
        self
    }

    #[doc="Modify the FCR register."]
    #[inline] pub fn with_fcr<F: FnOnce(Fcr) -> Fcr>(&self, f: F) -> &Self {
        self.fcr_reg().with(f);
        self
    }

    #[doc="Get the DLR Register."]
    #[inline] pub fn dlr_reg(&self) -> ::bobbin_mcu::register::Register<Dlr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dlr, 0x10)
    }

    #[doc="Get the *mut pointer for the DLR register."]
    #[inline] pub fn dlr_mut(&self) -> *mut Dlr { 
        self.dlr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DLR register."]
    #[inline] pub fn dlr_ptr(&self) -> *const Dlr { 
        self.dlr_reg().ptr()
    }

    #[doc="Read the DLR register."]
    #[inline] pub fn dlr(&self) -> Dlr { 
        self.dlr_reg().read()
    }

    #[doc="Write the DLR register."]
    #[inline] pub fn write_dlr(&self, value: Dlr) -> &Self { 
        self.dlr_reg().write(value);
        self
    }

    #[doc="Set the DLR register."]
    #[inline] pub fn set_dlr<F: FnOnce(Dlr) -> Dlr>(&self, f: F) -> &Self {
        self.dlr_reg().set(f);
        self
    }

    #[doc="Modify the DLR register."]
    #[inline] pub fn with_dlr<F: FnOnce(Dlr) -> Dlr>(&self, f: F) -> &Self {
        self.dlr_reg().with(f);
        self
    }

    #[doc="Get the CCR Register."]
    #[inline] pub fn ccr_reg(&self) -> ::bobbin_mcu::register::Register<Ccr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ccr, 0x14)
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

    #[doc="Get the AR Register."]
    #[inline] pub fn ar_reg(&self) -> ::bobbin_mcu::register::Register<Ar> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ar, 0x18)
    }

    #[doc="Get the *mut pointer for the AR register."]
    #[inline] pub fn ar_mut(&self) -> *mut Ar { 
        self.ar_reg().ptr()
    }

    #[doc="Get the *const pointer for the AR register."]
    #[inline] pub fn ar_ptr(&self) -> *const Ar { 
        self.ar_reg().ptr()
    }

    #[doc="Read the AR register."]
    #[inline] pub fn ar(&self) -> Ar { 
        self.ar_reg().read()
    }

    #[doc="Write the AR register."]
    #[inline] pub fn write_ar(&self, value: Ar) -> &Self { 
        self.ar_reg().write(value);
        self
    }

    #[doc="Set the AR register."]
    #[inline] pub fn set_ar<F: FnOnce(Ar) -> Ar>(&self, f: F) -> &Self {
        self.ar_reg().set(f);
        self
    }

    #[doc="Modify the AR register."]
    #[inline] pub fn with_ar<F: FnOnce(Ar) -> Ar>(&self, f: F) -> &Self {
        self.ar_reg().with(f);
        self
    }

    #[doc="Get the ABR Register."]
    #[inline] pub fn abr_reg(&self) -> ::bobbin_mcu::register::Register<Abr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Abr, 0x1c)
    }

    #[doc="Get the *mut pointer for the ABR register."]
    #[inline] pub fn abr_mut(&self) -> *mut Abr { 
        self.abr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ABR register."]
    #[inline] pub fn abr_ptr(&self) -> *const Abr { 
        self.abr_reg().ptr()
    }

    #[doc="Read the ABR register."]
    #[inline] pub fn abr(&self) -> Abr { 
        self.abr_reg().read()
    }

    #[doc="Write the ABR register."]
    #[inline] pub fn write_abr(&self, value: Abr) -> &Self { 
        self.abr_reg().write(value);
        self
    }

    #[doc="Set the ABR register."]
    #[inline] pub fn set_abr<F: FnOnce(Abr) -> Abr>(&self, f: F) -> &Self {
        self.abr_reg().set(f);
        self
    }

    #[doc="Modify the ABR register."]
    #[inline] pub fn with_abr<F: FnOnce(Abr) -> Abr>(&self, f: F) -> &Self {
        self.abr_reg().with(f);
        self
    }

    #[doc="Get the DR Register."]
    #[inline] pub fn dr_reg(&self) -> ::bobbin_mcu::register::Register<Dr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dr, 0x20)
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

    #[doc="Write the DR register."]
    #[inline] pub fn write_dr(&self, value: Dr) -> &Self { 
        self.dr_reg().write(value);
        self
    }

    #[doc="Set the DR register."]
    #[inline] pub fn set_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
        self.dr_reg().set(f);
        self
    }

    #[doc="Modify the DR register."]
    #[inline] pub fn with_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
        self.dr_reg().with(f);
        self
    }

    #[doc="Get the PSMKR Register."]
    #[inline] pub fn psmkr_reg(&self) -> ::bobbin_mcu::register::Register<Psmkr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Psmkr, 0x24)
    }

    #[doc="Get the *mut pointer for the PSMKR register."]
    #[inline] pub fn psmkr_mut(&self) -> *mut Psmkr { 
        self.psmkr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PSMKR register."]
    #[inline] pub fn psmkr_ptr(&self) -> *const Psmkr { 
        self.psmkr_reg().ptr()
    }

    #[doc="Read the PSMKR register."]
    #[inline] pub fn psmkr(&self) -> Psmkr { 
        self.psmkr_reg().read()
    }

    #[doc="Write the PSMKR register."]
    #[inline] pub fn write_psmkr(&self, value: Psmkr) -> &Self { 
        self.psmkr_reg().write(value);
        self
    }

    #[doc="Set the PSMKR register."]
    #[inline] pub fn set_psmkr<F: FnOnce(Psmkr) -> Psmkr>(&self, f: F) -> &Self {
        self.psmkr_reg().set(f);
        self
    }

    #[doc="Modify the PSMKR register."]
    #[inline] pub fn with_psmkr<F: FnOnce(Psmkr) -> Psmkr>(&self, f: F) -> &Self {
        self.psmkr_reg().with(f);
        self
    }

    #[doc="Get the PSMAR Register."]
    #[inline] pub fn psmar_reg(&self) -> ::bobbin_mcu::register::Register<Psmar> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Psmar, 0x28)
    }

    #[doc="Get the *mut pointer for the PSMAR register."]
    #[inline] pub fn psmar_mut(&self) -> *mut Psmar { 
        self.psmar_reg().ptr()
    }

    #[doc="Get the *const pointer for the PSMAR register."]
    #[inline] pub fn psmar_ptr(&self) -> *const Psmar { 
        self.psmar_reg().ptr()
    }

    #[doc="Read the PSMAR register."]
    #[inline] pub fn psmar(&self) -> Psmar { 
        self.psmar_reg().read()
    }

    #[doc="Write the PSMAR register."]
    #[inline] pub fn write_psmar(&self, value: Psmar) -> &Self { 
        self.psmar_reg().write(value);
        self
    }

    #[doc="Set the PSMAR register."]
    #[inline] pub fn set_psmar<F: FnOnce(Psmar) -> Psmar>(&self, f: F) -> &Self {
        self.psmar_reg().set(f);
        self
    }

    #[doc="Modify the PSMAR register."]
    #[inline] pub fn with_psmar<F: FnOnce(Psmar) -> Psmar>(&self, f: F) -> &Self {
        self.psmar_reg().with(f);
        self
    }

    #[doc="Get the PIR Register."]
    #[inline] pub fn pir_reg(&self) -> ::bobbin_mcu::register::Register<Pir> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pir, 0x2c)
    }

    #[doc="Get the *mut pointer for the PIR register."]
    #[inline] pub fn pir_mut(&self) -> *mut Pir { 
        self.pir_reg().ptr()
    }

    #[doc="Get the *const pointer for the PIR register."]
    #[inline] pub fn pir_ptr(&self) -> *const Pir { 
        self.pir_reg().ptr()
    }

    #[doc="Read the PIR register."]
    #[inline] pub fn pir(&self) -> Pir { 
        self.pir_reg().read()
    }

    #[doc="Write the PIR register."]
    #[inline] pub fn write_pir(&self, value: Pir) -> &Self { 
        self.pir_reg().write(value);
        self
    }

    #[doc="Set the PIR register."]
    #[inline] pub fn set_pir<F: FnOnce(Pir) -> Pir>(&self, f: F) -> &Self {
        self.pir_reg().set(f);
        self
    }

    #[doc="Modify the PIR register."]
    #[inline] pub fn with_pir<F: FnOnce(Pir) -> Pir>(&self, f: F) -> &Self {
        self.pir_reg().with(f);
        self
    }

    #[doc="Get the LPTR Register."]
    #[inline] pub fn lptr_reg(&self) -> ::bobbin_mcu::register::Register<Lptr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Lptr, 0x30)
    }

    #[doc="Get the *mut pointer for the LPTR register."]
    #[inline] pub fn lptr_mut(&self) -> *mut Lptr { 
        self.lptr_reg().ptr()
    }

    #[doc="Get the *const pointer for the LPTR register."]
    #[inline] pub fn lptr_ptr(&self) -> *const Lptr { 
        self.lptr_reg().ptr()
    }

    #[doc="Read the LPTR register."]
    #[inline] pub fn lptr(&self) -> Lptr { 
        self.lptr_reg().read()
    }

    #[doc="Write the LPTR register."]
    #[inline] pub fn write_lptr(&self, value: Lptr) -> &Self { 
        self.lptr_reg().write(value);
        self
    }

    #[doc="Set the LPTR register."]
    #[inline] pub fn set_lptr<F: FnOnce(Lptr) -> Lptr>(&self, f: F) -> &Self {
        self.lptr_reg().set(f);
        self
    }

    #[doc="Modify the LPTR register."]
    #[inline] pub fn with_lptr<F: FnOnce(Lptr) -> Lptr>(&self, f: F) -> &Self {
        self.lptr_reg().with(f);
        self
    }

}

#[doc="control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Clock prescaler"]
    #[inline] pub fn prescaler(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if PRESCALER != 0"]
    #[inline] pub fn test_prescaler(&self) -> bool {
        self.prescaler() != 0
    }

    #[doc="Sets the PRESCALER field."]
    #[inline] pub fn set_prescaler<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Polling match mode"]
    #[inline] pub fn pmm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if PMM != 0"]
    #[inline] pub fn test_pmm(&self) -> bool {
        self.pmm() != 0
    }

    #[doc="Sets the PMM field."]
    #[inline] pub fn set_pmm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Automatic poll mode stop"]
    #[inline] pub fn apms(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if APMS != 0"]
    #[inline] pub fn test_apms(&self) -> bool {
        self.apms() != 0
    }

    #[doc="Sets the APMS field."]
    #[inline] pub fn set_apms<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="TimeOut interrupt enable"]
    #[inline] pub fn toie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if TOIE != 0"]
    #[inline] pub fn test_toie(&self) -> bool {
        self.toie() != 0
    }

    #[doc="Sets the TOIE field."]
    #[inline] pub fn set_toie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Status match interrupt enable"]
    #[inline] pub fn smie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if SMIE != 0"]
    #[inline] pub fn test_smie(&self) -> bool {
        self.smie() != 0
    }

    #[doc="Sets the SMIE field."]
    #[inline] pub fn set_smie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="FIFO threshold interrupt enable"]
    #[inline] pub fn ftie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if FTIE != 0"]
    #[inline] pub fn test_ftie(&self) -> bool {
        self.ftie() != 0
    }

    #[doc="Sets the FTIE field."]
    #[inline] pub fn set_ftie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Transfer complete interrupt enable"]
    #[inline] pub fn tcie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TCIE != 0"]
    #[inline] pub fn test_tcie(&self) -> bool {
        self.tcie() != 0
    }

    #[doc="Sets the TCIE field."]
    #[inline] pub fn set_tcie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Transfer error interrupt enable"]
    #[inline] pub fn teie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TEIE != 0"]
    #[inline] pub fn test_teie(&self) -> bool {
        self.teie() != 0
    }

    #[doc="Sets the TEIE field."]
    #[inline] pub fn set_teie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="FIFO threshold level"]
    #[inline] pub fn fthres(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if FTHRES != 0"]
    #[inline] pub fn test_fthres(&self) -> bool {
        self.fthres() != 0
    }

    #[doc="Sets the FTHRES field."]
    #[inline] pub fn set_fthres<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Sample shift"]
    #[inline] pub fn sshift(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if SSHIFT != 0"]
    #[inline] pub fn test_sshift(&self) -> bool {
        self.sshift() != 0
    }

    #[doc="Sets the SSHIFT field."]
    #[inline] pub fn set_sshift<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Timeout counter enable"]
    #[inline] pub fn tcen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TCEN != 0"]
    #[inline] pub fn test_tcen(&self) -> bool {
        self.tcen() != 0
    }

    #[doc="Sets the TCEN field."]
    #[inline] pub fn set_tcen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="DMA enable"]
    #[inline] pub fn dmaen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DMAEN != 0"]
    #[inline] pub fn test_dmaen(&self) -> bool {
        self.dmaen() != 0
    }

    #[doc="Sets the DMAEN field."]
    #[inline] pub fn set_dmaen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Abort request"]
    #[inline] pub fn abort(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ABORT != 0"]
    #[inline] pub fn test_abort(&self) -> bool {
        self.abort() != 0
    }

    #[doc="Sets the ABORT field."]
    #[inline] pub fn set_abort<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Enable"]
    #[inline] pub fn en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EN != 0"]
    #[inline] pub fn test_en(&self) -> bool {
        self.en() != 0
    }

    #[doc="Sets the EN field."]
    #[inline] pub fn set_en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cr {
    #[inline]
    fn from(other: u32) -> Self {
         Cr(other)
    }
}

impl ::core::fmt::Display for Cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.prescaler() != 0 { try!(write!(f, " prescaler=0x{:x}", self.prescaler()))}
        if self.pmm() != 0 { try!(write!(f, " pmm"))}
        if self.apms() != 0 { try!(write!(f, " apms"))}
        if self.toie() != 0 { try!(write!(f, " toie"))}
        if self.smie() != 0 { try!(write!(f, " smie"))}
        if self.ftie() != 0 { try!(write!(f, " ftie"))}
        if self.tcie() != 0 { try!(write!(f, " tcie"))}
        if self.teie() != 0 { try!(write!(f, " teie"))}
        if self.fthres() != 0 { try!(write!(f, " fthres=0x{:x}", self.fthres()))}
        if self.sshift() != 0 { try!(write!(f, " sshift"))}
        if self.tcen() != 0 { try!(write!(f, " tcen"))}
        if self.dmaen() != 0 { try!(write!(f, " dmaen"))}
        if self.abort() != 0 { try!(write!(f, " abort"))}
        if self.en() != 0 { try!(write!(f, " en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcr(pub u32);
impl Dcr {
    #[doc="FLASH memory size"]
    #[inline] pub fn fsize(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1f) as u8) } // [20:16]
    }

    #[doc="Returns true if FSIZE != 0"]
    #[inline] pub fn test_fsize(&self) -> bool {
        self.fsize() != 0
    }

    #[doc="Sets the FSIZE field."]
    #[inline] pub fn set_fsize<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Chip select high time"]
    #[inline] pub fn csht(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if CSHT != 0"]
    #[inline] pub fn test_csht(&self) -> bool {
        self.csht() != 0
    }

    #[doc="Sets the CSHT field."]
    #[inline] pub fn set_csht<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Mode 0 / mode 3"]
    #[inline] pub fn ckmode(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CKMODE != 0"]
    #[inline] pub fn test_ckmode(&self) -> bool {
        self.ckmode() != 0
    }

    #[doc="Sets the CKMODE field."]
    #[inline] pub fn set_ckmode<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dcr {
    #[inline]
    fn from(other: u32) -> Self {
         Dcr(other)
    }
}

impl ::core::fmt::Display for Dcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fsize() != 0 { try!(write!(f, " fsize=0x{:x}", self.fsize()))}
        if self.csht() != 0 { try!(write!(f, " csht=0x{:x}", self.csht()))}
        if self.ckmode() != 0 { try!(write!(f, " ckmode"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="FIFO level"]
    #[inline] pub fn flevel(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1f) as u8) } // [12:8]
    }

    #[doc="Returns true if FLEVEL != 0"]
    #[inline] pub fn test_flevel(&self) -> bool {
        self.flevel() != 0
    }

    #[doc="Sets the FLEVEL field."]
    #[inline] pub fn set_flevel<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Busy"]
    #[inline] pub fn busy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if BUSY != 0"]
    #[inline] pub fn test_busy(&self) -> bool {
        self.busy() != 0
    }

    #[doc="Sets the BUSY field."]
    #[inline] pub fn set_busy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Timeout flag"]
    #[inline] pub fn tof(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TOF != 0"]
    #[inline] pub fn test_tof(&self) -> bool {
        self.tof() != 0
    }

    #[doc="Sets the TOF field."]
    #[inline] pub fn set_tof<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Status match flag"]
    #[inline] pub fn smf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SMF != 0"]
    #[inline] pub fn test_smf(&self) -> bool {
        self.smf() != 0
    }

    #[doc="Sets the SMF field."]
    #[inline] pub fn set_smf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="FIFO threshold flag"]
    #[inline] pub fn ftf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FTF != 0"]
    #[inline] pub fn test_ftf(&self) -> bool {
        self.ftf() != 0
    }

    #[doc="Sets the FTF field."]
    #[inline] pub fn set_ftf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Transfer complete flag"]
    #[inline] pub fn tcf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TCF != 0"]
    #[inline] pub fn test_tcf(&self) -> bool {
        self.tcf() != 0
    }

    #[doc="Sets the TCF field."]
    #[inline] pub fn set_tcf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Transfer error flag"]
    #[inline] pub fn tef(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TEF != 0"]
    #[inline] pub fn test_tef(&self) -> bool {
        self.tef() != 0
    }

    #[doc="Sets the TEF field."]
    #[inline] pub fn set_tef<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.flevel() != 0 { try!(write!(f, " flevel=0x{:x}", self.flevel()))}
        if self.busy() != 0 { try!(write!(f, " busy"))}
        if self.tof() != 0 { try!(write!(f, " tof"))}
        if self.smf() != 0 { try!(write!(f, " smf"))}
        if self.ftf() != 0 { try!(write!(f, " ftf"))}
        if self.tcf() != 0 { try!(write!(f, " tcf"))}
        if self.tef() != 0 { try!(write!(f, " tef"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="flag clear register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fcr(pub u32);
impl Fcr {
    #[doc="Clear timeout flag"]
    #[inline] pub fn ctof(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CTOF != 0"]
    #[inline] pub fn test_ctof(&self) -> bool {
        self.ctof() != 0
    }

    #[doc="Sets the CTOF field."]
    #[inline] pub fn set_ctof<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Clear status match flag"]
    #[inline] pub fn csmf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CSMF != 0"]
    #[inline] pub fn test_csmf(&self) -> bool {
        self.csmf() != 0
    }

    #[doc="Sets the CSMF field."]
    #[inline] pub fn set_csmf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Clear transfer complete flag"]
    #[inline] pub fn ctcf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CTCF != 0"]
    #[inline] pub fn test_ctcf(&self) -> bool {
        self.ctcf() != 0
    }

    #[doc="Sets the CTCF field."]
    #[inline] pub fn set_ctcf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Clear transfer error flag"]
    #[inline] pub fn ctef(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CTEF != 0"]
    #[inline] pub fn test_ctef(&self) -> bool {
        self.ctef() != 0
    }

    #[doc="Sets the CTEF field."]
    #[inline] pub fn set_ctef<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fcr {
    #[inline]
    fn from(other: u32) -> Self {
         Fcr(other)
    }
}

impl ::core::fmt::Display for Fcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ctof() != 0 { try!(write!(f, " ctof"))}
        if self.csmf() != 0 { try!(write!(f, " csmf"))}
        if self.ctcf() != 0 { try!(write!(f, " ctcf"))}
        if self.ctef() != 0 { try!(write!(f, " ctef"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data length register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dlr(pub u32);
impl Dlr {
    #[doc="Data length"]
    #[inline] pub fn dl(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DL != 0"]
    #[inline] pub fn test_dl(&self) -> bool {
        self.dl() != 0
    }

    #[doc="Sets the DL field."]
    #[inline] pub fn set_dl<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dlr {
    #[inline]
    fn from(other: u32) -> Self {
         Dlr(other)
    }
}

impl ::core::fmt::Display for Dlr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dlr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="communication configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccr(pub u32);
impl Ccr {
    #[doc="Double data rate mode"]
    #[inline] pub fn ddrm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if DDRM != 0"]
    #[inline] pub fn test_ddrm(&self) -> bool {
        self.ddrm() != 0
    }

    #[doc="Sets the DDRM field."]
    #[inline] pub fn set_ddrm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Send instruction only once mode"]
    #[inline] pub fn sioo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if SIOO != 0"]
    #[inline] pub fn test_sioo(&self) -> bool {
        self.sioo() != 0
    }

    #[doc="Sets the SIOO field."]
    #[inline] pub fn set_sioo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Functional mode"]
    #[inline] pub fn fmode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x3) as u8) } // [27:26]
    }

    #[doc="Returns true if FMODE != 0"]
    #[inline] pub fn test_fmode(&self) -> bool {
        self.fmode() != 0
    }

    #[doc="Sets the FMODE field."]
    #[inline] pub fn set_fmode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Data mode"]
    #[inline] pub fn dmode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
    }

    #[doc="Returns true if DMODE != 0"]
    #[inline] pub fn test_dmode(&self) -> bool {
        self.dmode() != 0
    }

    #[doc="Sets the DMODE field."]
    #[inline] pub fn set_dmode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Number of dummy cycles"]
    #[inline] pub fn dcyc(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1f) as u8) } // [22:18]
    }

    #[doc="Returns true if DCYC != 0"]
    #[inline] pub fn test_dcyc(&self) -> bool {
        self.dcyc() != 0
    }

    #[doc="Sets the DCYC field."]
    #[inline] pub fn set_dcyc<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Alternate bytes size"]
    #[inline] pub fn absize(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if ABSIZE != 0"]
    #[inline] pub fn test_absize(&self) -> bool {
        self.absize() != 0
    }

    #[doc="Sets the ABSIZE field."]
    #[inline] pub fn set_absize<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Alternate bytes mode"]
    #[inline] pub fn abmode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if ABMODE != 0"]
    #[inline] pub fn test_abmode(&self) -> bool {
        self.abmode() != 0
    }

    #[doc="Sets the ABMODE field."]
    #[inline] pub fn set_abmode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Address size"]
    #[inline] pub fn adsize(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if ADSIZE != 0"]
    #[inline] pub fn test_adsize(&self) -> bool {
        self.adsize() != 0
    }

    #[doc="Sets the ADSIZE field."]
    #[inline] pub fn set_adsize<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Address mode"]
    #[inline] pub fn admode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if ADMODE != 0"]
    #[inline] pub fn test_admode(&self) -> bool {
        self.admode() != 0
    }

    #[doc="Sets the ADMODE field."]
    #[inline] pub fn set_admode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Instruction mode"]
    #[inline] pub fn imode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if IMODE != 0"]
    #[inline] pub fn test_imode(&self) -> bool {
        self.imode() != 0
    }

    #[doc="Sets the IMODE field."]
    #[inline] pub fn set_imode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Instruction"]
    #[inline] pub fn instruction(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if INSTRUCTION != 0"]
    #[inline] pub fn test_instruction(&self) -> bool {
        self.instruction() != 0
    }

    #[doc="Sets the INSTRUCTION field."]
    #[inline] pub fn set_instruction<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
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
        if self.ddrm() != 0 { try!(write!(f, " ddrm"))}
        if self.sioo() != 0 { try!(write!(f, " sioo"))}
        if self.fmode() != 0 { try!(write!(f, " fmode=0x{:x}", self.fmode()))}
        if self.dmode() != 0 { try!(write!(f, " dmode=0x{:x}", self.dmode()))}
        if self.dcyc() != 0 { try!(write!(f, " dcyc=0x{:x}", self.dcyc()))}
        if self.absize() != 0 { try!(write!(f, " absize=0x{:x}", self.absize()))}
        if self.abmode() != 0 { try!(write!(f, " abmode=0x{:x}", self.abmode()))}
        if self.adsize() != 0 { try!(write!(f, " adsize=0x{:x}", self.adsize()))}
        if self.admode() != 0 { try!(write!(f, " admode=0x{:x}", self.admode()))}
        if self.imode() != 0 { try!(write!(f, " imode=0x{:x}", self.imode()))}
        if self.instruction() != 0 { try!(write!(f, " instruction=0x{:x}", self.instruction()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ar(pub u32);
impl Ar {
    #[doc="Address"]
    #[inline] pub fn address(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ADDRESS != 0"]
    #[inline] pub fn test_address(&self) -> bool {
        self.address() != 0
    }

    #[doc="Sets the ADDRESS field."]
    #[inline] pub fn set_address<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ar {
    #[inline]
    fn from(other: u32) -> Self {
         Ar(other)
    }
}

impl ::core::fmt::Display for Ar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ABR"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Abr(pub u32);
impl Abr {
    #[doc="ALTERNATE"]
    #[inline] pub fn alternate(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ALTERNATE != 0"]
    #[inline] pub fn test_alternate(&self) -> bool {
        self.alternate() != 0
    }

    #[doc="Sets the ALTERNATE field."]
    #[inline] pub fn set_alternate<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Abr {
    #[inline]
    fn from(other: u32) -> Self {
         Abr(other)
    }
}

impl ::core::fmt::Display for Abr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Abr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
    #[doc="Data"]
    #[inline] pub fn data(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
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
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="polling status mask register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Psmkr(pub u32);
impl Psmkr {
    #[doc="Status mask"]
    #[inline] pub fn mask(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if MASK != 0"]
    #[inline] pub fn test_mask(&self) -> bool {
        self.mask() != 0
    }

    #[doc="Sets the MASK field."]
    #[inline] pub fn set_mask<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Psmkr {
    #[inline]
    fn from(other: u32) -> Self {
         Psmkr(other)
    }
}

impl ::core::fmt::Display for Psmkr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Psmkr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="polling status match register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Psmar(pub u32);
impl Psmar {
    #[doc="Status match"]
    #[inline] pub fn _match(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if MATCH != 0"]
    #[inline] pub fn test_match(&self) -> bool {
        self._match() != 0
    }

    #[doc="Sets the MATCH field."]
    #[inline] pub fn set_match<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Psmar {
    #[inline]
    fn from(other: u32) -> Self {
         Psmar(other)
    }
}

impl ::core::fmt::Display for Psmar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Psmar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="polling interval register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pir(pub u32);
impl Pir {
    #[doc="Polling interval"]
    #[inline] pub fn interval(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if INTERVAL != 0"]
    #[inline] pub fn test_interval(&self) -> bool {
        self.interval() != 0
    }

    #[doc="Sets the INTERVAL field."]
    #[inline] pub fn set_interval<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pir {
    #[inline]
    fn from(other: u32) -> Self {
         Pir(other)
    }
}

impl ::core::fmt::Display for Pir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.interval() != 0 { try!(write!(f, " interval=0x{:x}", self.interval()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="low-power timeout register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lptr(pub u32);
impl Lptr {
    #[doc="Timeout period"]
    #[inline] pub fn timeout(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if TIMEOUT != 0"]
    #[inline] pub fn test_timeout(&self) -> bool {
        self.timeout() != 0
    }

    #[doc="Sets the TIMEOUT field."]
    #[inline] pub fn set_timeout<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Lptr {
    #[inline]
    fn from(other: u32) -> Self {
         Lptr(other)
    }
}

impl ::core::fmt::Display for Lptr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lptr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.timeout() != 0 { try!(write!(f, " timeout=0x{:x}", self.timeout()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

