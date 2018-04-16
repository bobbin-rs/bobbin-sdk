
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="ADC Peripheral"]
pub struct AdcPeriph(pub usize); 

pub struct AdcCh { pub periph: AdcPeriph, pub index: usize }

impl AdcPeriph {
    #[doc="Get the SC1 Register."]
    #[inline] pub fn sc1_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Sc1, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Sc1, 0x0, 0x4)
    }

    #[doc="Get the *mut pointer for the SC1 register."]
    #[inline] pub fn sc1_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Sc1 { 
        self.sc1_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the SC1 register."]
    #[inline] pub fn sc1_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Sc1 { 
        self.sc1_reg().ptr(index.into())
    }

    #[doc="Read the SC1 register."]
    #[inline] pub fn sc1<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Sc1 { 
        self.sc1_reg().read(index.into())
    }

    #[doc="Write the SC1 register."]
    #[inline] pub fn write_sc1<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Sc1) -> &Self {
        self.sc1_reg().write(index.into(), value);
        self
    }

    #[doc="Set the SC1 register."]
    #[inline] pub fn set_sc1<I: Into<::bobbin_bits::R2>, F: FnOnce(Sc1) -> Sc1>(&self, index: I, f: F) -> &Self {
        self.sc1_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the SC1 register."]
    #[inline] pub fn with_sc1<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Sc1) -> Sc1>(&self, index: I, f: F) -> &Self {
        self.sc1_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CFG1 Register."]
    #[inline] pub fn cfg1_reg(&self) -> ::bobbin_mcu::register::Register<Cfg1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cfg1, 0x8)
    }

    #[doc="Get the *mut pointer for the CFG1 register."]
    #[inline] pub fn cfg1_mut(&self) -> *mut Cfg1 { 
        self.cfg1_reg().ptr()
    }

    #[doc="Get the *const pointer for the CFG1 register."]
    #[inline] pub fn cfg1_ptr(&self) -> *const Cfg1 { 
        self.cfg1_reg().ptr()
    }

    #[doc="Read the CFG1 register."]
    #[inline] pub fn cfg1(&self) -> Cfg1 { 
        self.cfg1_reg().read()
    }

    #[doc="Write the CFG1 register."]
    #[inline] pub fn write_cfg1(&self, value: Cfg1) -> &Self { 
        self.cfg1_reg().write(value);
        self
    }

    #[doc="Set the CFG1 register."]
    #[inline] pub fn set_cfg1<F: FnOnce(Cfg1) -> Cfg1>(&self, f: F) -> &Self {
        self.cfg1_reg().set(f);
        self
    }

    #[doc="Modify the CFG1 register."]
    #[inline] pub fn with_cfg1<F: FnOnce(Cfg1) -> Cfg1>(&self, f: F) -> &Self {
        self.cfg1_reg().with(f);
        self
    }

    #[doc="Get the CFG2 Register."]
    #[inline] pub fn cfg2_reg(&self) -> ::bobbin_mcu::register::Register<Cfg2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cfg2, 0xc)
    }

    #[doc="Get the *mut pointer for the CFG2 register."]
    #[inline] pub fn cfg2_mut(&self) -> *mut Cfg2 { 
        self.cfg2_reg().ptr()
    }

    #[doc="Get the *const pointer for the CFG2 register."]
    #[inline] pub fn cfg2_ptr(&self) -> *const Cfg2 { 
        self.cfg2_reg().ptr()
    }

    #[doc="Read the CFG2 register."]
    #[inline] pub fn cfg2(&self) -> Cfg2 { 
        self.cfg2_reg().read()
    }

    #[doc="Write the CFG2 register."]
    #[inline] pub fn write_cfg2(&self, value: Cfg2) -> &Self { 
        self.cfg2_reg().write(value);
        self
    }

    #[doc="Set the CFG2 register."]
    #[inline] pub fn set_cfg2<F: FnOnce(Cfg2) -> Cfg2>(&self, f: F) -> &Self {
        self.cfg2_reg().set(f);
        self
    }

    #[doc="Modify the CFG2 register."]
    #[inline] pub fn with_cfg2<F: FnOnce(Cfg2) -> Cfg2>(&self, f: F) -> &Self {
        self.cfg2_reg().with(f);
        self
    }

    #[doc="Get the R Register."]
    #[inline] pub fn r_reg(&self) -> ::bobbin_mcu::register::RegisterArray<R, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut R, 0x10, 0x4)
    }

    #[doc="Get the *mut pointer for the R register."]
    #[inline] pub fn r_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut R { 
        self.r_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the R register."]
    #[inline] pub fn r_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const R { 
        self.r_reg().ptr(index.into())
    }

    #[doc="Read the R register."]
    #[inline] pub fn r<I: Into<::bobbin_bits::R2>>(&self, index: I) -> R { 
        self.r_reg().read(index.into())
    }

    #[doc="Get the CV Register."]
    #[inline] pub fn cv_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Cv, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Cv, 0x18, 0x4)
    }

    #[doc="Get the *mut pointer for the CV register."]
    #[inline] pub fn cv_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Cv { 
        self.cv_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CV register."]
    #[inline] pub fn cv_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Cv { 
        self.cv_reg().ptr(index.into())
    }

    #[doc="Read the CV register."]
    #[inline] pub fn cv<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Cv { 
        self.cv_reg().read(index.into())
    }

    #[doc="Write the CV register."]
    #[inline] pub fn write_cv<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Cv) -> &Self {
        self.cv_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CV register."]
    #[inline] pub fn set_cv<I: Into<::bobbin_bits::R2>, F: FnOnce(Cv) -> Cv>(&self, index: I, f: F) -> &Self {
        self.cv_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CV register."]
    #[inline] pub fn with_cv<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Cv) -> Cv>(&self, index: I, f: F) -> &Self {
        self.cv_reg().with(index.into(), f);
        self
    }

    #[doc="Get the SC2 Register."]
    #[inline] pub fn sc2_reg(&self) -> ::bobbin_mcu::register::Register<Sc2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sc2, 0x20)
    }

    #[doc="Get the *mut pointer for the SC2 register."]
    #[inline] pub fn sc2_mut(&self) -> *mut Sc2 { 
        self.sc2_reg().ptr()
    }

    #[doc="Get the *const pointer for the SC2 register."]
    #[inline] pub fn sc2_ptr(&self) -> *const Sc2 { 
        self.sc2_reg().ptr()
    }

    #[doc="Read the SC2 register."]
    #[inline] pub fn sc2(&self) -> Sc2 { 
        self.sc2_reg().read()
    }

    #[doc="Write the SC2 register."]
    #[inline] pub fn write_sc2(&self, value: Sc2) -> &Self { 
        self.sc2_reg().write(value);
        self
    }

    #[doc="Set the SC2 register."]
    #[inline] pub fn set_sc2<F: FnOnce(Sc2) -> Sc2>(&self, f: F) -> &Self {
        self.sc2_reg().set(f);
        self
    }

    #[doc="Modify the SC2 register."]
    #[inline] pub fn with_sc2<F: FnOnce(Sc2) -> Sc2>(&self, f: F) -> &Self {
        self.sc2_reg().with(f);
        self
    }

    #[doc="Get the SC3 Register."]
    #[inline] pub fn sc3_reg(&self) -> ::bobbin_mcu::register::Register<Sc3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sc3, 0x24)
    }

    #[doc="Get the *mut pointer for the SC3 register."]
    #[inline] pub fn sc3_mut(&self) -> *mut Sc3 { 
        self.sc3_reg().ptr()
    }

    #[doc="Get the *const pointer for the SC3 register."]
    #[inline] pub fn sc3_ptr(&self) -> *const Sc3 { 
        self.sc3_reg().ptr()
    }

    #[doc="Read the SC3 register."]
    #[inline] pub fn sc3(&self) -> Sc3 { 
        self.sc3_reg().read()
    }

    #[doc="Write the SC3 register."]
    #[inline] pub fn write_sc3(&self, value: Sc3) -> &Self { 
        self.sc3_reg().write(value);
        self
    }

    #[doc="Set the SC3 register."]
    #[inline] pub fn set_sc3<F: FnOnce(Sc3) -> Sc3>(&self, f: F) -> &Self {
        self.sc3_reg().set(f);
        self
    }

    #[doc="Modify the SC3 register."]
    #[inline] pub fn with_sc3<F: FnOnce(Sc3) -> Sc3>(&self, f: F) -> &Self {
        self.sc3_reg().with(f);
        self
    }

    #[doc="Get the OFS Register."]
    #[inline] pub fn ofs_reg(&self) -> ::bobbin_mcu::register::Register<Ofs> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ofs, 0x28)
    }

    #[doc="Get the *mut pointer for the OFS register."]
    #[inline] pub fn ofs_mut(&self) -> *mut Ofs { 
        self.ofs_reg().ptr()
    }

    #[doc="Get the *const pointer for the OFS register."]
    #[inline] pub fn ofs_ptr(&self) -> *const Ofs { 
        self.ofs_reg().ptr()
    }

    #[doc="Read the OFS register."]
    #[inline] pub fn ofs(&self) -> Ofs { 
        self.ofs_reg().read()
    }

    #[doc="Write the OFS register."]
    #[inline] pub fn write_ofs(&self, value: Ofs) -> &Self { 
        self.ofs_reg().write(value);
        self
    }

    #[doc="Set the OFS register."]
    #[inline] pub fn set_ofs<F: FnOnce(Ofs) -> Ofs>(&self, f: F) -> &Self {
        self.ofs_reg().set(f);
        self
    }

    #[doc="Modify the OFS register."]
    #[inline] pub fn with_ofs<F: FnOnce(Ofs) -> Ofs>(&self, f: F) -> &Self {
        self.ofs_reg().with(f);
        self
    }

    #[doc="Get the PG Register."]
    #[inline] pub fn pg_reg(&self) -> ::bobbin_mcu::register::Register<Pg> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pg, 0x2c)
    }

    #[doc="Get the *mut pointer for the PG register."]
    #[inline] pub fn pg_mut(&self) -> *mut Pg { 
        self.pg_reg().ptr()
    }

    #[doc="Get the *const pointer for the PG register."]
    #[inline] pub fn pg_ptr(&self) -> *const Pg { 
        self.pg_reg().ptr()
    }

    #[doc="Read the PG register."]
    #[inline] pub fn pg(&self) -> Pg { 
        self.pg_reg().read()
    }

    #[doc="Write the PG register."]
    #[inline] pub fn write_pg(&self, value: Pg) -> &Self { 
        self.pg_reg().write(value);
        self
    }

    #[doc="Set the PG register."]
    #[inline] pub fn set_pg<F: FnOnce(Pg) -> Pg>(&self, f: F) -> &Self {
        self.pg_reg().set(f);
        self
    }

    #[doc="Modify the PG register."]
    #[inline] pub fn with_pg<F: FnOnce(Pg) -> Pg>(&self, f: F) -> &Self {
        self.pg_reg().with(f);
        self
    }

    #[doc="Get the MG Register."]
    #[inline] pub fn mg_reg(&self) -> ::bobbin_mcu::register::Register<Mg> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mg, 0x30)
    }

    #[doc="Get the *mut pointer for the MG register."]
    #[inline] pub fn mg_mut(&self) -> *mut Mg { 
        self.mg_reg().ptr()
    }

    #[doc="Get the *const pointer for the MG register."]
    #[inline] pub fn mg_ptr(&self) -> *const Mg { 
        self.mg_reg().ptr()
    }

    #[doc="Read the MG register."]
    #[inline] pub fn mg(&self) -> Mg { 
        self.mg_reg().read()
    }

    #[doc="Write the MG register."]
    #[inline] pub fn write_mg(&self, value: Mg) -> &Self { 
        self.mg_reg().write(value);
        self
    }

    #[doc="Set the MG register."]
    #[inline] pub fn set_mg<F: FnOnce(Mg) -> Mg>(&self, f: F) -> &Self {
        self.mg_reg().set(f);
        self
    }

    #[doc="Modify the MG register."]
    #[inline] pub fn with_mg<F: FnOnce(Mg) -> Mg>(&self, f: F) -> &Self {
        self.mg_reg().with(f);
        self
    }

    #[doc="Get the CLPD Register."]
    #[inline] pub fn clpd_reg(&self) -> ::bobbin_mcu::register::Register<Clpd> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Clpd, 0x34)
    }

    #[doc="Get the *mut pointer for the CLPD register."]
    #[inline] pub fn clpd_mut(&self) -> *mut Clpd { 
        self.clpd_reg().ptr()
    }

    #[doc="Get the *const pointer for the CLPD register."]
    #[inline] pub fn clpd_ptr(&self) -> *const Clpd { 
        self.clpd_reg().ptr()
    }

    #[doc="Read the CLPD register."]
    #[inline] pub fn clpd(&self) -> Clpd { 
        self.clpd_reg().read()
    }

    #[doc="Write the CLPD register."]
    #[inline] pub fn write_clpd(&self, value: Clpd) -> &Self { 
        self.clpd_reg().write(value);
        self
    }

    #[doc="Set the CLPD register."]
    #[inline] pub fn set_clpd<F: FnOnce(Clpd) -> Clpd>(&self, f: F) -> &Self {
        self.clpd_reg().set(f);
        self
    }

    #[doc="Modify the CLPD register."]
    #[inline] pub fn with_clpd<F: FnOnce(Clpd) -> Clpd>(&self, f: F) -> &Self {
        self.clpd_reg().with(f);
        self
    }

    #[doc="Get the CLPS Register."]
    #[inline] pub fn clps_reg(&self) -> ::bobbin_mcu::register::Register<Clps> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Clps, 0x38)
    }

    #[doc="Get the *mut pointer for the CLPS register."]
    #[inline] pub fn clps_mut(&self) -> *mut Clps { 
        self.clps_reg().ptr()
    }

    #[doc="Get the *const pointer for the CLPS register."]
    #[inline] pub fn clps_ptr(&self) -> *const Clps { 
        self.clps_reg().ptr()
    }

    #[doc="Read the CLPS register."]
    #[inline] pub fn clps(&self) -> Clps { 
        self.clps_reg().read()
    }

    #[doc="Write the CLPS register."]
    #[inline] pub fn write_clps(&self, value: Clps) -> &Self { 
        self.clps_reg().write(value);
        self
    }

    #[doc="Set the CLPS register."]
    #[inline] pub fn set_clps<F: FnOnce(Clps) -> Clps>(&self, f: F) -> &Self {
        self.clps_reg().set(f);
        self
    }

    #[doc="Modify the CLPS register."]
    #[inline] pub fn with_clps<F: FnOnce(Clps) -> Clps>(&self, f: F) -> &Self {
        self.clps_reg().with(f);
        self
    }

    #[doc="Get the CLP4 Register."]
    #[inline] pub fn clp4_reg(&self) -> ::bobbin_mcu::register::Register<Clp4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Clp4, 0x3c)
    }

    #[doc="Get the *mut pointer for the CLP4 register."]
    #[inline] pub fn clp4_mut(&self) -> *mut Clp4 { 
        self.clp4_reg().ptr()
    }

    #[doc="Get the *const pointer for the CLP4 register."]
    #[inline] pub fn clp4_ptr(&self) -> *const Clp4 { 
        self.clp4_reg().ptr()
    }

    #[doc="Read the CLP4 register."]
    #[inline] pub fn clp4(&self) -> Clp4 { 
        self.clp4_reg().read()
    }

    #[doc="Write the CLP4 register."]
    #[inline] pub fn write_clp4(&self, value: Clp4) -> &Self { 
        self.clp4_reg().write(value);
        self
    }

    #[doc="Set the CLP4 register."]
    #[inline] pub fn set_clp4<F: FnOnce(Clp4) -> Clp4>(&self, f: F) -> &Self {
        self.clp4_reg().set(f);
        self
    }

    #[doc="Modify the CLP4 register."]
    #[inline] pub fn with_clp4<F: FnOnce(Clp4) -> Clp4>(&self, f: F) -> &Self {
        self.clp4_reg().with(f);
        self
    }

    #[doc="Get the CLP3 Register."]
    #[inline] pub fn clp3_reg(&self) -> ::bobbin_mcu::register::Register<Clp3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Clp3, 0x40)
    }

    #[doc="Get the *mut pointer for the CLP3 register."]
    #[inline] pub fn clp3_mut(&self) -> *mut Clp3 { 
        self.clp3_reg().ptr()
    }

    #[doc="Get the *const pointer for the CLP3 register."]
    #[inline] pub fn clp3_ptr(&self) -> *const Clp3 { 
        self.clp3_reg().ptr()
    }

    #[doc="Read the CLP3 register."]
    #[inline] pub fn clp3(&self) -> Clp3 { 
        self.clp3_reg().read()
    }

    #[doc="Write the CLP3 register."]
    #[inline] pub fn write_clp3(&self, value: Clp3) -> &Self { 
        self.clp3_reg().write(value);
        self
    }

    #[doc="Set the CLP3 register."]
    #[inline] pub fn set_clp3<F: FnOnce(Clp3) -> Clp3>(&self, f: F) -> &Self {
        self.clp3_reg().set(f);
        self
    }

    #[doc="Modify the CLP3 register."]
    #[inline] pub fn with_clp3<F: FnOnce(Clp3) -> Clp3>(&self, f: F) -> &Self {
        self.clp3_reg().with(f);
        self
    }

    #[doc="Get the CLP2 Register."]
    #[inline] pub fn clp2_reg(&self) -> ::bobbin_mcu::register::Register<Clp2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Clp2, 0x44)
    }

    #[doc="Get the *mut pointer for the CLP2 register."]
    #[inline] pub fn clp2_mut(&self) -> *mut Clp2 { 
        self.clp2_reg().ptr()
    }

    #[doc="Get the *const pointer for the CLP2 register."]
    #[inline] pub fn clp2_ptr(&self) -> *const Clp2 { 
        self.clp2_reg().ptr()
    }

    #[doc="Read the CLP2 register."]
    #[inline] pub fn clp2(&self) -> Clp2 { 
        self.clp2_reg().read()
    }

    #[doc="Write the CLP2 register."]
    #[inline] pub fn write_clp2(&self, value: Clp2) -> &Self { 
        self.clp2_reg().write(value);
        self
    }

    #[doc="Set the CLP2 register."]
    #[inline] pub fn set_clp2<F: FnOnce(Clp2) -> Clp2>(&self, f: F) -> &Self {
        self.clp2_reg().set(f);
        self
    }

    #[doc="Modify the CLP2 register."]
    #[inline] pub fn with_clp2<F: FnOnce(Clp2) -> Clp2>(&self, f: F) -> &Self {
        self.clp2_reg().with(f);
        self
    }

    #[doc="Get the CLP1 Register."]
    #[inline] pub fn clp1_reg(&self) -> ::bobbin_mcu::register::Register<Clp1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Clp1, 0x48)
    }

    #[doc="Get the *mut pointer for the CLP1 register."]
    #[inline] pub fn clp1_mut(&self) -> *mut Clp1 { 
        self.clp1_reg().ptr()
    }

    #[doc="Get the *const pointer for the CLP1 register."]
    #[inline] pub fn clp1_ptr(&self) -> *const Clp1 { 
        self.clp1_reg().ptr()
    }

    #[doc="Read the CLP1 register."]
    #[inline] pub fn clp1(&self) -> Clp1 { 
        self.clp1_reg().read()
    }

    #[doc="Write the CLP1 register."]
    #[inline] pub fn write_clp1(&self, value: Clp1) -> &Self { 
        self.clp1_reg().write(value);
        self
    }

    #[doc="Set the CLP1 register."]
    #[inline] pub fn set_clp1<F: FnOnce(Clp1) -> Clp1>(&self, f: F) -> &Self {
        self.clp1_reg().set(f);
        self
    }

    #[doc="Modify the CLP1 register."]
    #[inline] pub fn with_clp1<F: FnOnce(Clp1) -> Clp1>(&self, f: F) -> &Self {
        self.clp1_reg().with(f);
        self
    }

    #[doc="Get the CLP0 Register."]
    #[inline] pub fn clp0_reg(&self) -> ::bobbin_mcu::register::Register<Clp0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Clp0, 0x4c)
    }

    #[doc="Get the *mut pointer for the CLP0 register."]
    #[inline] pub fn clp0_mut(&self) -> *mut Clp0 { 
        self.clp0_reg().ptr()
    }

    #[doc="Get the *const pointer for the CLP0 register."]
    #[inline] pub fn clp0_ptr(&self) -> *const Clp0 { 
        self.clp0_reg().ptr()
    }

    #[doc="Read the CLP0 register."]
    #[inline] pub fn clp0(&self) -> Clp0 { 
        self.clp0_reg().read()
    }

    #[doc="Write the CLP0 register."]
    #[inline] pub fn write_clp0(&self, value: Clp0) -> &Self { 
        self.clp0_reg().write(value);
        self
    }

    #[doc="Set the CLP0 register."]
    #[inline] pub fn set_clp0<F: FnOnce(Clp0) -> Clp0>(&self, f: F) -> &Self {
        self.clp0_reg().set(f);
        self
    }

    #[doc="Modify the CLP0 register."]
    #[inline] pub fn with_clp0<F: FnOnce(Clp0) -> Clp0>(&self, f: F) -> &Self {
        self.clp0_reg().with(f);
        self
    }

    #[doc="Get the CLMD Register."]
    #[inline] pub fn clmd_reg(&self) -> ::bobbin_mcu::register::Register<Clmd> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Clmd, 0x54)
    }

    #[doc="Get the *mut pointer for the CLMD register."]
    #[inline] pub fn clmd_mut(&self) -> *mut Clmd { 
        self.clmd_reg().ptr()
    }

    #[doc="Get the *const pointer for the CLMD register."]
    #[inline] pub fn clmd_ptr(&self) -> *const Clmd { 
        self.clmd_reg().ptr()
    }

    #[doc="Read the CLMD register."]
    #[inline] pub fn clmd(&self) -> Clmd { 
        self.clmd_reg().read()
    }

    #[doc="Write the CLMD register."]
    #[inline] pub fn write_clmd(&self, value: Clmd) -> &Self { 
        self.clmd_reg().write(value);
        self
    }

    #[doc="Set the CLMD register."]
    #[inline] pub fn set_clmd<F: FnOnce(Clmd) -> Clmd>(&self, f: F) -> &Self {
        self.clmd_reg().set(f);
        self
    }

    #[doc="Modify the CLMD register."]
    #[inline] pub fn with_clmd<F: FnOnce(Clmd) -> Clmd>(&self, f: F) -> &Self {
        self.clmd_reg().with(f);
        self
    }

    #[doc="Get the CLMS Register."]
    #[inline] pub fn clms_reg(&self) -> ::bobbin_mcu::register::Register<Clms> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Clms, 0x58)
    }

    #[doc="Get the *mut pointer for the CLMS register."]
    #[inline] pub fn clms_mut(&self) -> *mut Clms { 
        self.clms_reg().ptr()
    }

    #[doc="Get the *const pointer for the CLMS register."]
    #[inline] pub fn clms_ptr(&self) -> *const Clms { 
        self.clms_reg().ptr()
    }

    #[doc="Read the CLMS register."]
    #[inline] pub fn clms(&self) -> Clms { 
        self.clms_reg().read()
    }

    #[doc="Write the CLMS register."]
    #[inline] pub fn write_clms(&self, value: Clms) -> &Self { 
        self.clms_reg().write(value);
        self
    }

    #[doc="Set the CLMS register."]
    #[inline] pub fn set_clms<F: FnOnce(Clms) -> Clms>(&self, f: F) -> &Self {
        self.clms_reg().set(f);
        self
    }

    #[doc="Modify the CLMS register."]
    #[inline] pub fn with_clms<F: FnOnce(Clms) -> Clms>(&self, f: F) -> &Self {
        self.clms_reg().with(f);
        self
    }

    #[doc="Get the CLM4 Register."]
    #[inline] pub fn clm4_reg(&self) -> ::bobbin_mcu::register::Register<Clm4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Clm4, 0x5c)
    }

    #[doc="Get the *mut pointer for the CLM4 register."]
    #[inline] pub fn clm4_mut(&self) -> *mut Clm4 { 
        self.clm4_reg().ptr()
    }

    #[doc="Get the *const pointer for the CLM4 register."]
    #[inline] pub fn clm4_ptr(&self) -> *const Clm4 { 
        self.clm4_reg().ptr()
    }

    #[doc="Read the CLM4 register."]
    #[inline] pub fn clm4(&self) -> Clm4 { 
        self.clm4_reg().read()
    }

    #[doc="Write the CLM4 register."]
    #[inline] pub fn write_clm4(&self, value: Clm4) -> &Self { 
        self.clm4_reg().write(value);
        self
    }

    #[doc="Set the CLM4 register."]
    #[inline] pub fn set_clm4<F: FnOnce(Clm4) -> Clm4>(&self, f: F) -> &Self {
        self.clm4_reg().set(f);
        self
    }

    #[doc="Modify the CLM4 register."]
    #[inline] pub fn with_clm4<F: FnOnce(Clm4) -> Clm4>(&self, f: F) -> &Self {
        self.clm4_reg().with(f);
        self
    }

    #[doc="Get the CLM3 Register."]
    #[inline] pub fn clm3_reg(&self) -> ::bobbin_mcu::register::Register<Clm3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Clm3, 0x60)
    }

    #[doc="Get the *mut pointer for the CLM3 register."]
    #[inline] pub fn clm3_mut(&self) -> *mut Clm3 { 
        self.clm3_reg().ptr()
    }

    #[doc="Get the *const pointer for the CLM3 register."]
    #[inline] pub fn clm3_ptr(&self) -> *const Clm3 { 
        self.clm3_reg().ptr()
    }

    #[doc="Read the CLM3 register."]
    #[inline] pub fn clm3(&self) -> Clm3 { 
        self.clm3_reg().read()
    }

    #[doc="Write the CLM3 register."]
    #[inline] pub fn write_clm3(&self, value: Clm3) -> &Self { 
        self.clm3_reg().write(value);
        self
    }

    #[doc="Set the CLM3 register."]
    #[inline] pub fn set_clm3<F: FnOnce(Clm3) -> Clm3>(&self, f: F) -> &Self {
        self.clm3_reg().set(f);
        self
    }

    #[doc="Modify the CLM3 register."]
    #[inline] pub fn with_clm3<F: FnOnce(Clm3) -> Clm3>(&self, f: F) -> &Self {
        self.clm3_reg().with(f);
        self
    }

    #[doc="Get the CLM2 Register."]
    #[inline] pub fn clm2_reg(&self) -> ::bobbin_mcu::register::Register<Clm2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Clm2, 0x64)
    }

    #[doc="Get the *mut pointer for the CLM2 register."]
    #[inline] pub fn clm2_mut(&self) -> *mut Clm2 { 
        self.clm2_reg().ptr()
    }

    #[doc="Get the *const pointer for the CLM2 register."]
    #[inline] pub fn clm2_ptr(&self) -> *const Clm2 { 
        self.clm2_reg().ptr()
    }

    #[doc="Read the CLM2 register."]
    #[inline] pub fn clm2(&self) -> Clm2 { 
        self.clm2_reg().read()
    }

    #[doc="Write the CLM2 register."]
    #[inline] pub fn write_clm2(&self, value: Clm2) -> &Self { 
        self.clm2_reg().write(value);
        self
    }

    #[doc="Set the CLM2 register."]
    #[inline] pub fn set_clm2<F: FnOnce(Clm2) -> Clm2>(&self, f: F) -> &Self {
        self.clm2_reg().set(f);
        self
    }

    #[doc="Modify the CLM2 register."]
    #[inline] pub fn with_clm2<F: FnOnce(Clm2) -> Clm2>(&self, f: F) -> &Self {
        self.clm2_reg().with(f);
        self
    }

    #[doc="Get the CLM1 Register."]
    #[inline] pub fn clm1_reg(&self) -> ::bobbin_mcu::register::Register<Clm1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Clm1, 0x68)
    }

    #[doc="Get the *mut pointer for the CLM1 register."]
    #[inline] pub fn clm1_mut(&self) -> *mut Clm1 { 
        self.clm1_reg().ptr()
    }

    #[doc="Get the *const pointer for the CLM1 register."]
    #[inline] pub fn clm1_ptr(&self) -> *const Clm1 { 
        self.clm1_reg().ptr()
    }

    #[doc="Read the CLM1 register."]
    #[inline] pub fn clm1(&self) -> Clm1 { 
        self.clm1_reg().read()
    }

    #[doc="Write the CLM1 register."]
    #[inline] pub fn write_clm1(&self, value: Clm1) -> &Self { 
        self.clm1_reg().write(value);
        self
    }

    #[doc="Set the CLM1 register."]
    #[inline] pub fn set_clm1<F: FnOnce(Clm1) -> Clm1>(&self, f: F) -> &Self {
        self.clm1_reg().set(f);
        self
    }

    #[doc="Modify the CLM1 register."]
    #[inline] pub fn with_clm1<F: FnOnce(Clm1) -> Clm1>(&self, f: F) -> &Self {
        self.clm1_reg().with(f);
        self
    }

    #[doc="Get the CLM0 Register."]
    #[inline] pub fn clm0_reg(&self) -> ::bobbin_mcu::register::Register<Clm0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Clm0, 0x6c)
    }

    #[doc="Get the *mut pointer for the CLM0 register."]
    #[inline] pub fn clm0_mut(&self) -> *mut Clm0 { 
        self.clm0_reg().ptr()
    }

    #[doc="Get the *const pointer for the CLM0 register."]
    #[inline] pub fn clm0_ptr(&self) -> *const Clm0 { 
        self.clm0_reg().ptr()
    }

    #[doc="Read the CLM0 register."]
    #[inline] pub fn clm0(&self) -> Clm0 { 
        self.clm0_reg().read()
    }

    #[doc="Write the CLM0 register."]
    #[inline] pub fn write_clm0(&self, value: Clm0) -> &Self { 
        self.clm0_reg().write(value);
        self
    }

    #[doc="Set the CLM0 register."]
    #[inline] pub fn set_clm0<F: FnOnce(Clm0) -> Clm0>(&self, f: F) -> &Self {
        self.clm0_reg().set(f);
        self
    }

    #[doc="Modify the CLM0 register."]
    #[inline] pub fn with_clm0<F: FnOnce(Clm0) -> Clm0>(&self, f: F) -> &Self {
        self.clm0_reg().with(f);
        self
    }

}

#[doc="ADC Status and Control Registers 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sc1(pub u32);
impl Sc1 {
    #[doc="Input channel select"]
    #[inline] pub fn adch(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if ADCH != 0"]
    #[inline] pub fn test_adch(&self) -> bool {
        self.adch() != 0
    }

    #[doc="Sets the ADCH field."]
    #[inline] pub fn set_adch<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Differential Mode Enable"]
    #[inline] pub fn diff(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DIFF != 0"]
    #[inline] pub fn test_diff(&self) -> bool {
        self.diff() != 0
    }

    #[doc="Sets the DIFF field."]
    #[inline] pub fn set_diff<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Interrupt Enable"]
    #[inline] pub fn aien(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if AIEN != 0"]
    #[inline] pub fn test_aien(&self) -> bool {
        self.aien() != 0
    }

    #[doc="Sets the AIEN field."]
    #[inline] pub fn set_aien<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Conversion Complete Flag"]
    #[inline] pub fn coco(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if COCO != 0"]
    #[inline] pub fn test_coco(&self) -> bool {
        self.coco() != 0
    }

    #[doc="Sets the COCO field."]
    #[inline] pub fn set_coco<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Sc1 {
    #[inline]
    fn from(other: u32) -> Self {
         Sc1(other)
    }
}

impl ::core::fmt::Display for Sc1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sc1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.adch() != 0 { try!(write!(f, " adch=0x{:x}", self.adch()))}
        if self.diff() != 0 { try!(write!(f, " diff"))}
        if self.aien() != 0 { try!(write!(f, " aien"))}
        if self.coco() != 0 { try!(write!(f, " coco"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Configuration Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfg1(pub u32);
impl Cfg1 {
    #[doc="Input Clock Select"]
    #[inline] pub fn adiclk(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if ADICLK != 0"]
    #[inline] pub fn test_adiclk(&self) -> bool {
        self.adiclk() != 0
    }

    #[doc="Sets the ADICLK field."]
    #[inline] pub fn set_adiclk<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Conversion mode selection"]
    #[inline] pub fn mode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Sample Time Configuration"]
    #[inline] pub fn adlsmp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ADLSMP != 0"]
    #[inline] pub fn test_adlsmp(&self) -> bool {
        self.adlsmp() != 0
    }

    #[doc="Sets the ADLSMP field."]
    #[inline] pub fn set_adlsmp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Clock Divide Select"]
    #[inline] pub fn adiv(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if ADIV != 0"]
    #[inline] pub fn test_adiv(&self) -> bool {
        self.adiv() != 0
    }

    #[doc="Sets the ADIV field."]
    #[inline] pub fn set_adiv<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Low-Power Configuration"]
    #[inline] pub fn adlpc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ADLPC != 0"]
    #[inline] pub fn test_adlpc(&self) -> bool {
        self.adlpc() != 0
    }

    #[doc="Sets the ADLPC field."]
    #[inline] pub fn set_adlpc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Cfg1 {
    #[inline]
    fn from(other: u32) -> Self {
         Cfg1(other)
    }
}

impl ::core::fmt::Display for Cfg1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfg1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.adiclk() != 0 { try!(write!(f, " adiclk=0x{:x}", self.adiclk()))}
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        if self.adlsmp() != 0 { try!(write!(f, " adlsmp"))}
        if self.adiv() != 0 { try!(write!(f, " adiv=0x{:x}", self.adiv()))}
        if self.adlpc() != 0 { try!(write!(f, " adlpc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Configuration Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfg2(pub u32);
impl Cfg2 {
    #[doc="Long Sample Time Select"]
    #[inline] pub fn adlsts(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if ADLSTS != 0"]
    #[inline] pub fn test_adlsts(&self) -> bool {
        self.adlsts() != 0
    }

    #[doc="Sets the ADLSTS field."]
    #[inline] pub fn set_adlsts<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="High-Speed Configuration"]
    #[inline] pub fn adhsc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ADHSC != 0"]
    #[inline] pub fn test_adhsc(&self) -> bool {
        self.adhsc() != 0
    }

    #[doc="Sets the ADHSC field."]
    #[inline] pub fn set_adhsc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Asynchronous Clock Output Enable"]
    #[inline] pub fn adacken(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ADACKEN != 0"]
    #[inline] pub fn test_adacken(&self) -> bool {
        self.adacken() != 0
    }

    #[doc="Sets the ADACKEN field."]
    #[inline] pub fn set_adacken<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="ADC Mux Select"]
    #[inline] pub fn muxsel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if MUXSEL != 0"]
    #[inline] pub fn test_muxsel(&self) -> bool {
        self.muxsel() != 0
    }

    #[doc="Sets the MUXSEL field."]
    #[inline] pub fn set_muxsel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Cfg2 {
    #[inline]
    fn from(other: u32) -> Self {
         Cfg2(other)
    }
}

impl ::core::fmt::Display for Cfg2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfg2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.adlsts() != 0 { try!(write!(f, " adlsts=0x{:x}", self.adlsts()))}
        if self.adhsc() != 0 { try!(write!(f, " adhsc"))}
        if self.adacken() != 0 { try!(write!(f, " adacken"))}
        if self.muxsel() != 0 { try!(write!(f, " muxsel"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Data Result Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R(pub u32);
impl R {
    #[doc="Data result"]
    #[inline] pub fn d(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if D != 0"]
    #[inline] pub fn test_d(&self) -> bool {
        self.d() != 0
    }

    #[doc="Sets the D field."]
    #[inline] pub fn set_d<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data result (16 bit)"]
    #[inline] pub fn d16(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if D16 != 0"]
    #[inline] pub fn test_d16(&self) -> bool {
        self.d16() != 0
    }

    #[doc="Sets the D16 field."]
    #[inline] pub fn set_d16<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data result (12 bit)"]
    #[inline] pub fn d12(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if D12 != 0"]
    #[inline] pub fn test_d12(&self) -> bool {
        self.d12() != 0
    }

    #[doc="Sets the D12 field."]
    #[inline] pub fn set_d12<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data result (10 bit)"]
    #[inline] pub fn d10(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if D10 != 0"]
    #[inline] pub fn test_d10(&self) -> bool {
        self.d10() != 0
    }

    #[doc="Sets the D10 field."]
    #[inline] pub fn set_d10<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data result (8 bit)"]
    #[inline] pub fn d8(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if D8 != 0"]
    #[inline] pub fn test_d8(&self) -> bool {
        self.d8() != 0
    }

    #[doc="Sets the D8 field."]
    #[inline] pub fn set_d8<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R {
    #[inline]
    fn from(other: u32) -> Self {
         R(other)
    }
}

impl ::core::fmt::Display for R {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.d() != 0 { try!(write!(f, " d=0x{:x}", self.d()))}
        if self.d16() != 0 { try!(write!(f, " d16=0x{:x}", self.d16()))}
        if self.d12() != 0 { try!(write!(f, " d12=0x{:x}", self.d12()))}
        if self.d10() != 0 { try!(write!(f, " d10=0x{:x}", self.d10()))}
        if self.d8() != 0 { try!(write!(f, " d8=0x{:x}", self.d8()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Compare Value Registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cv(pub u32);
impl Cv {
    #[doc="Compare Value."]
    #[inline] pub fn cv(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CV != 0"]
    #[inline] pub fn test_cv(&self) -> bool {
        self.cv() != 0
    }

    #[doc="Sets the CV field."]
    #[inline] pub fn set_cv<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cv {
    #[inline]
    fn from(other: u32) -> Self {
         Cv(other)
    }
}

impl ::core::fmt::Display for Cv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cv() != 0 { try!(write!(f, " cv=0x{:x}", self.cv()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status and Control Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sc2(pub u32);
impl Sc2 {
    #[doc="Voltage Reference Selection"]
    #[inline] pub fn refsel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if REFSEL != 0"]
    #[inline] pub fn test_refsel(&self) -> bool {
        self.refsel() != 0
    }

    #[doc="Sets the REFSEL field."]
    #[inline] pub fn set_refsel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DMA Enable"]
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

    #[doc="Compare Function Range Enable"]
    #[inline] pub fn acren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ACREN != 0"]
    #[inline] pub fn test_acren(&self) -> bool {
        self.acren() != 0
    }

    #[doc="Sets the ACREN field."]
    #[inline] pub fn set_acren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Compare Function Greater Than Enable"]
    #[inline] pub fn acfgt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ACFGT != 0"]
    #[inline] pub fn test_acfgt(&self) -> bool {
        self.acfgt() != 0
    }

    #[doc="Sets the ACFGT field."]
    #[inline] pub fn set_acfgt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Compare Function Enable"]
    #[inline] pub fn acfe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACFE != 0"]
    #[inline] pub fn test_acfe(&self) -> bool {
        self.acfe() != 0
    }

    #[doc="Sets the ACFE field."]
    #[inline] pub fn set_acfe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Conversion Trigger Select"]
    #[inline] pub fn adtrg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if ADTRG != 0"]
    #[inline] pub fn test_adtrg(&self) -> bool {
        self.adtrg() != 0
    }

    #[doc="Sets the ADTRG field."]
    #[inline] pub fn set_adtrg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Conversion Active"]
    #[inline] pub fn adact(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ADACT != 0"]
    #[inline] pub fn test_adact(&self) -> bool {
        self.adact() != 0
    }

    #[doc="Sets the ADACT field."]
    #[inline] pub fn set_adact<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Sc2 {
    #[inline]
    fn from(other: u32) -> Self {
         Sc2(other)
    }
}

impl ::core::fmt::Display for Sc2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sc2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.refsel() != 0 { try!(write!(f, " refsel=0x{:x}", self.refsel()))}
        if self.dmaen() != 0 { try!(write!(f, " dmaen"))}
        if self.acren() != 0 { try!(write!(f, " acren"))}
        if self.acfgt() != 0 { try!(write!(f, " acfgt"))}
        if self.acfe() != 0 { try!(write!(f, " acfe"))}
        if self.adtrg() != 0 { try!(write!(f, " adtrg"))}
        if self.adact() != 0 { try!(write!(f, " adact"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status and Control Register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sc3(pub u32);
impl Sc3 {
    #[doc="Hardware Average Select"]
    #[inline] pub fn avgs(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if AVGS != 0"]
    #[inline] pub fn test_avgs(&self) -> bool {
        self.avgs() != 0
    }

    #[doc="Sets the AVGS field."]
    #[inline] pub fn set_avgs<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Hardware Average Enable"]
    #[inline] pub fn avge(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if AVGE != 0"]
    #[inline] pub fn test_avge(&self) -> bool {
        self.avge() != 0
    }

    #[doc="Sets the AVGE field."]
    #[inline] pub fn set_avge<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Continuous Conversion Enable"]
    #[inline] pub fn adco(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ADCO != 0"]
    #[inline] pub fn test_adco(&self) -> bool {
        self.adco() != 0
    }

    #[doc="Sets the ADCO field."]
    #[inline] pub fn set_adco<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Calibration Failed Flag"]
    #[inline] pub fn calf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CALF != 0"]
    #[inline] pub fn test_calf(&self) -> bool {
        self.calf() != 0
    }

    #[doc="Sets the CALF field."]
    #[inline] pub fn set_calf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Calibration"]
    #[inline] pub fn cal(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CAL != 0"]
    #[inline] pub fn test_cal(&self) -> bool {
        self.cal() != 0
    }

    #[doc="Sets the CAL field."]
    #[inline] pub fn set_cal<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Sc3 {
    #[inline]
    fn from(other: u32) -> Self {
         Sc3(other)
    }
}

impl ::core::fmt::Display for Sc3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sc3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.avgs() != 0 { try!(write!(f, " avgs=0x{:x}", self.avgs()))}
        if self.avge() != 0 { try!(write!(f, " avge"))}
        if self.adco() != 0 { try!(write!(f, " adco"))}
        if self.calf() != 0 { try!(write!(f, " calf"))}
        if self.cal() != 0 { try!(write!(f, " cal"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Offset Correction Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ofs(pub u32);
impl Ofs {
    #[doc="Offset Error Correction Value"]
    #[inline] pub fn ofs(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if OFS != 0"]
    #[inline] pub fn test_ofs(&self) -> bool {
        self.ofs() != 0
    }

    #[doc="Sets the OFS field."]
    #[inline] pub fn set_ofs<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ofs {
    #[inline]
    fn from(other: u32) -> Self {
         Ofs(other)
    }
}

impl ::core::fmt::Display for Ofs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ofs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ofs() != 0 { try!(write!(f, " ofs=0x{:x}", self.ofs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Plus-Side Gain Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pg(pub u32);
impl Pg {
    #[doc="Plus-Side Gain"]
    #[inline] pub fn pg(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if PG != 0"]
    #[inline] pub fn test_pg(&self) -> bool {
        self.pg() != 0
    }

    #[doc="Sets the PG field."]
    #[inline] pub fn set_pg<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pg {
    #[inline]
    fn from(other: u32) -> Self {
         Pg(other)
    }
}

impl ::core::fmt::Display for Pg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pg() != 0 { try!(write!(f, " pg=0x{:x}", self.pg()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Minus-Side Gain Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mg(pub u32);
impl Mg {
    #[doc="Minus-Side Gain"]
    #[inline] pub fn mg(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if MG != 0"]
    #[inline] pub fn test_mg(&self) -> bool {
        self.mg() != 0
    }

    #[doc="Sets the MG field."]
    #[inline] pub fn set_mg<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mg {
    #[inline]
    fn from(other: u32) -> Self {
         Mg(other)
    }
}

impl ::core::fmt::Display for Mg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mg() != 0 { try!(write!(f, " mg=0x{:x}", self.mg()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Plus-Side General Calibration Value Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clpd(pub u32);
impl Clpd {
    #[doc="Calibration Value"]
    #[inline] pub fn clpd(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if CLPD != 0"]
    #[inline] pub fn test_clpd(&self) -> bool {
        self.clpd() != 0
    }

    #[doc="Sets the CLPD field."]
    #[inline] pub fn set_clpd<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clpd {
    #[inline]
    fn from(other: u32) -> Self {
         Clpd(other)
    }
}

impl ::core::fmt::Display for Clpd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clpd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clpd() != 0 { try!(write!(f, " clpd=0x{:x}", self.clpd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Plus-Side General Calibration Value Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clps(pub u32);
impl Clps {
    #[doc="Calibration Value"]
    #[inline] pub fn clps(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if CLPS != 0"]
    #[inline] pub fn test_clps(&self) -> bool {
        self.clps() != 0
    }

    #[doc="Sets the CLPS field."]
    #[inline] pub fn set_clps<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clps {
    #[inline]
    fn from(other: u32) -> Self {
         Clps(other)
    }
}

impl ::core::fmt::Display for Clps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clps() != 0 { try!(write!(f, " clps=0x{:x}", self.clps()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Plus-Side General Calibration Value Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clp4(pub u32);
impl Clp4 {
    #[doc="Calibration Value"]
    #[inline] pub fn clp4(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if CLP4 != 0"]
    #[inline] pub fn test_clp4(&self) -> bool {
        self.clp4() != 0
    }

    #[doc="Sets the CLP4 field."]
    #[inline] pub fn set_clp4<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clp4 {
    #[inline]
    fn from(other: u32) -> Self {
         Clp4(other)
    }
}

impl ::core::fmt::Display for Clp4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clp4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clp4() != 0 { try!(write!(f, " clp4=0x{:x}", self.clp4()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Plus-Side General Calibration Value Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clp3(pub u32);
impl Clp3 {
    #[doc="Calibration Value"]
    #[inline] pub fn clp3(&self) -> ::bobbin_bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if CLP3 != 0"]
    #[inline] pub fn test_clp3(&self) -> bool {
        self.clp3() != 0
    }

    #[doc="Sets the CLP3 field."]
    #[inline] pub fn set_clp3<V: Into<::bobbin_bits::U9>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clp3 {
    #[inline]
    fn from(other: u32) -> Self {
         Clp3(other)
    }
}

impl ::core::fmt::Display for Clp3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clp3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clp3() != 0 { try!(write!(f, " clp3=0x{:x}", self.clp3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Plus-Side General Calibration Value Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clp2(pub u32);
impl Clp2 {
    #[doc="Calibration Value"]
    #[inline] pub fn clp2(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CLP2 != 0"]
    #[inline] pub fn test_clp2(&self) -> bool {
        self.clp2() != 0
    }

    #[doc="Sets the CLP2 field."]
    #[inline] pub fn set_clp2<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clp2 {
    #[inline]
    fn from(other: u32) -> Self {
         Clp2(other)
    }
}

impl ::core::fmt::Display for Clp2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clp2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clp2() != 0 { try!(write!(f, " clp2=0x{:x}", self.clp2()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Plus-Side General Calibration Value Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clp1(pub u32);
impl Clp1 {
    #[doc="Calibration Value"]
    #[inline] pub fn clp1(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if CLP1 != 0"]
    #[inline] pub fn test_clp1(&self) -> bool {
        self.clp1() != 0
    }

    #[doc="Sets the CLP1 field."]
    #[inline] pub fn set_clp1<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clp1 {
    #[inline]
    fn from(other: u32) -> Self {
         Clp1(other)
    }
}

impl ::core::fmt::Display for Clp1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clp1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clp1() != 0 { try!(write!(f, " clp1=0x{:x}", self.clp1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Plus-Side General Calibration Value Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clp0(pub u32);
impl Clp0 {
    #[doc="Calibration Value"]
    #[inline] pub fn clp0(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if CLP0 != 0"]
    #[inline] pub fn test_clp0(&self) -> bool {
        self.clp0() != 0
    }

    #[doc="Sets the CLP0 field."]
    #[inline] pub fn set_clp0<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clp0 {
    #[inline]
    fn from(other: u32) -> Self {
         Clp0(other)
    }
}

impl ::core::fmt::Display for Clp0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clp0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clp0() != 0 { try!(write!(f, " clp0=0x{:x}", self.clp0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Minus-Side General Calibration Value Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clmd(pub u32);
impl Clmd {
    #[doc="Calibration Value"]
    #[inline] pub fn clmd(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if CLMD != 0"]
    #[inline] pub fn test_clmd(&self) -> bool {
        self.clmd() != 0
    }

    #[doc="Sets the CLMD field."]
    #[inline] pub fn set_clmd<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clmd {
    #[inline]
    fn from(other: u32) -> Self {
         Clmd(other)
    }
}

impl ::core::fmt::Display for Clmd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clmd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clmd() != 0 { try!(write!(f, " clmd=0x{:x}", self.clmd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Minus-Side General Calibration Value Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clms(pub u32);
impl Clms {
    #[doc="Calibration Value"]
    #[inline] pub fn clms(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if CLMS != 0"]
    #[inline] pub fn test_clms(&self) -> bool {
        self.clms() != 0
    }

    #[doc="Sets the CLMS field."]
    #[inline] pub fn set_clms<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clms {
    #[inline]
    fn from(other: u32) -> Self {
         Clms(other)
    }
}

impl ::core::fmt::Display for Clms {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clms {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clms() != 0 { try!(write!(f, " clms=0x{:x}", self.clms()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Minus-Side General Calibration Value Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clm4(pub u32);
impl Clm4 {
    #[doc="Calibration Value"]
    #[inline] pub fn clm4(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if CLM4 != 0"]
    #[inline] pub fn test_clm4(&self) -> bool {
        self.clm4() != 0
    }

    #[doc="Sets the CLM4 field."]
    #[inline] pub fn set_clm4<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clm4 {
    #[inline]
    fn from(other: u32) -> Self {
         Clm4(other)
    }
}

impl ::core::fmt::Display for Clm4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clm4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clm4() != 0 { try!(write!(f, " clm4=0x{:x}", self.clm4()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Minus-Side General Calibration Value Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clm3(pub u32);
impl Clm3 {
    #[doc="Calibration Value"]
    #[inline] pub fn clm3(&self) -> ::bobbin_bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if CLM3 != 0"]
    #[inline] pub fn test_clm3(&self) -> bool {
        self.clm3() != 0
    }

    #[doc="Sets the CLM3 field."]
    #[inline] pub fn set_clm3<V: Into<::bobbin_bits::U9>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clm3 {
    #[inline]
    fn from(other: u32) -> Self {
         Clm3(other)
    }
}

impl ::core::fmt::Display for Clm3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clm3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clm3() != 0 { try!(write!(f, " clm3=0x{:x}", self.clm3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Minus-Side General Calibration Value Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clm2(pub u32);
impl Clm2 {
    #[doc="Calibration Value"]
    #[inline] pub fn clm2(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CLM2 != 0"]
    #[inline] pub fn test_clm2(&self) -> bool {
        self.clm2() != 0
    }

    #[doc="Sets the CLM2 field."]
    #[inline] pub fn set_clm2<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clm2 {
    #[inline]
    fn from(other: u32) -> Self {
         Clm2(other)
    }
}

impl ::core::fmt::Display for Clm2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clm2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clm2() != 0 { try!(write!(f, " clm2=0x{:x}", self.clm2()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Minus-Side General Calibration Value Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clm1(pub u32);
impl Clm1 {
    #[doc="Calibration Value"]
    #[inline] pub fn clm1(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if CLM1 != 0"]
    #[inline] pub fn test_clm1(&self) -> bool {
        self.clm1() != 0
    }

    #[doc="Sets the CLM1 field."]
    #[inline] pub fn set_clm1<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clm1 {
    #[inline]
    fn from(other: u32) -> Self {
         Clm1(other)
    }
}

impl ::core::fmt::Display for Clm1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clm1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clm1() != 0 { try!(write!(f, " clm1=0x{:x}", self.clm1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Minus-Side General Calibration Value Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clm0(pub u32);
impl Clm0 {
    #[doc="Calibration Value"]
    #[inline] pub fn clm0(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if CLM0 != 0"]
    #[inline] pub fn test_clm0(&self) -> bool {
        self.clm0() != 0
    }

    #[doc="Sets the CLM0 field."]
    #[inline] pub fn set_clm0<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clm0 {
    #[inline]
    fn from(other: u32) -> Self {
         Clm0(other)
    }
}

impl ::core::fmt::Display for Clm0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clm0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clm0() != 0 { try!(write!(f, " clm0=0x{:x}", self.clm0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

