#[allow(unused_imports)] use bobbin_common::*;


#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="ADC Peripheral"]
pub struct AdcPeriph(pub usize); 


impl AdcPeriph {
    #[doc="Get the *mut pointer for the SC1 register."]
    #[inline] pub fn sc1_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Sc1 { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x0 + (index << 2)) as *mut Sc1
    }

    #[doc="Get the *const pointer for the SC1 register."]
    #[inline] pub fn sc1_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Sc1 { 
           self.sc1_mut(index)
    }

    #[doc="Read the SC1 register."]
    #[inline] pub fn sc1<I: Into<bits::R2>>(&self, index: I) -> Sc1 { 
        unsafe {
            read_volatile(self.sc1_ptr(index))
        }
    }

    #[doc="Write the SC1 register."]
    #[inline] pub fn set_sc1<I: Into<bits::R2>, F: FnOnce(Sc1) -> Sc1>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.sc1_mut(index), f(Sc1(0)));
        }
        self
    }

    #[doc="Modify the SC1 register."]
    #[inline] pub fn with_sc1<I: Into<bits::R2> + Copy, F: FnOnce(Sc1) -> Sc1>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.sc1_mut(index), f(self.sc1(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the CFG1 register."]
    #[inline] pub fn cfg1_mut(&self) -> *mut Cfg1 { 
        (self.0 + 0x8) as *mut Cfg1
    }

    #[doc="Get the *const pointer for the CFG1 register."]
    #[inline] pub fn cfg1_ptr(&self) -> *const Cfg1 { 
           self.cfg1_mut()
    }

    #[doc="Read the CFG1 register."]
    #[inline] pub fn cfg1(&self) -> Cfg1 { 
        unsafe {
            read_volatile(self.cfg1_ptr())
        }
    }

    #[doc="Write the CFG1 register."]
    #[inline] pub fn set_cfg1<F: FnOnce(Cfg1) -> Cfg1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfg1_mut(), f(Cfg1(0)));
        }
        self
    }

    #[doc="Modify the CFG1 register."]
    #[inline] pub fn with_cfg1<F: FnOnce(Cfg1) -> Cfg1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfg1_mut(), f(self.cfg1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CFG2 register."]
    #[inline] pub fn cfg2_mut(&self) -> *mut Cfg2 { 
        (self.0 + 0xc) as *mut Cfg2
    }

    #[doc="Get the *const pointer for the CFG2 register."]
    #[inline] pub fn cfg2_ptr(&self) -> *const Cfg2 { 
           self.cfg2_mut()
    }

    #[doc="Read the CFG2 register."]
    #[inline] pub fn cfg2(&self) -> Cfg2 { 
        unsafe {
            read_volatile(self.cfg2_ptr())
        }
    }

    #[doc="Write the CFG2 register."]
    #[inline] pub fn set_cfg2<F: FnOnce(Cfg2) -> Cfg2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfg2_mut(), f(Cfg2(0)));
        }
        self
    }

    #[doc="Modify the CFG2 register."]
    #[inline] pub fn with_cfg2<F: FnOnce(Cfg2) -> Cfg2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfg2_mut(), f(self.cfg2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the R register."]
    #[inline] pub fn r_mut<I: Into<bits::R2>>(&self, index: I) -> *mut R { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x10 + (index << 2)) as *mut R
    }

    #[doc="Get the *const pointer for the R register."]
    #[inline] pub fn r_ptr<I: Into<bits::R2>>(&self, index: I) -> *const R { 
           self.r_mut(index)
    }

    #[doc="Read the R register."]
    #[inline] pub fn r<I: Into<bits::R2>>(&self, index: I) -> R { 
        unsafe {
            read_volatile(self.r_ptr(index))
        }
    }

    #[doc="Get the *mut pointer for the CV register."]
    #[inline] pub fn cv_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Cv { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x18 + (index << 2)) as *mut Cv
    }

    #[doc="Get the *const pointer for the CV register."]
    #[inline] pub fn cv_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Cv { 
           self.cv_mut(index)
    }

    #[doc="Read the CV register."]
    #[inline] pub fn cv<I: Into<bits::R2>>(&self, index: I) -> Cv { 
        unsafe {
            read_volatile(self.cv_ptr(index))
        }
    }

    #[doc="Write the CV register."]
    #[inline] pub fn set_cv<I: Into<bits::R2>, F: FnOnce(Cv) -> Cv>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.cv_mut(index), f(Cv(0)));
        }
        self
    }

    #[doc="Modify the CV register."]
    #[inline] pub fn with_cv<I: Into<bits::R2> + Copy, F: FnOnce(Cv) -> Cv>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.cv_mut(index), f(self.cv(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the SC2 register."]
    #[inline] pub fn sc2_mut(&self) -> *mut Sc2 { 
        (self.0 + 0x20) as *mut Sc2
    }

    #[doc="Get the *const pointer for the SC2 register."]
    #[inline] pub fn sc2_ptr(&self) -> *const Sc2 { 
           self.sc2_mut()
    }

    #[doc="Read the SC2 register."]
    #[inline] pub fn sc2(&self) -> Sc2 { 
        unsafe {
            read_volatile(self.sc2_ptr())
        }
    }

    #[doc="Write the SC2 register."]
    #[inline] pub fn set_sc2<F: FnOnce(Sc2) -> Sc2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sc2_mut(), f(Sc2(0)));
        }
        self
    }

    #[doc="Modify the SC2 register."]
    #[inline] pub fn with_sc2<F: FnOnce(Sc2) -> Sc2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sc2_mut(), f(self.sc2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SC3 register."]
    #[inline] pub fn sc3_mut(&self) -> *mut Sc3 { 
        (self.0 + 0x24) as *mut Sc3
    }

    #[doc="Get the *const pointer for the SC3 register."]
    #[inline] pub fn sc3_ptr(&self) -> *const Sc3 { 
           self.sc3_mut()
    }

    #[doc="Read the SC3 register."]
    #[inline] pub fn sc3(&self) -> Sc3 { 
        unsafe {
            read_volatile(self.sc3_ptr())
        }
    }

    #[doc="Write the SC3 register."]
    #[inline] pub fn set_sc3<F: FnOnce(Sc3) -> Sc3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sc3_mut(), f(Sc3(0)));
        }
        self
    }

    #[doc="Modify the SC3 register."]
    #[inline] pub fn with_sc3<F: FnOnce(Sc3) -> Sc3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sc3_mut(), f(self.sc3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the OFS register."]
    #[inline] pub fn ofs_mut(&self) -> *mut Ofs { 
        (self.0 + 0x28) as *mut Ofs
    }

    #[doc="Get the *const pointer for the OFS register."]
    #[inline] pub fn ofs_ptr(&self) -> *const Ofs { 
           self.ofs_mut()
    }

    #[doc="Read the OFS register."]
    #[inline] pub fn ofs(&self) -> Ofs { 
        unsafe {
            read_volatile(self.ofs_ptr())
        }
    }

    #[doc="Write the OFS register."]
    #[inline] pub fn set_ofs<F: FnOnce(Ofs) -> Ofs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ofs_mut(), f(Ofs(0)));
        }
        self
    }

    #[doc="Modify the OFS register."]
    #[inline] pub fn with_ofs<F: FnOnce(Ofs) -> Ofs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ofs_mut(), f(self.ofs()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PG register."]
    #[inline] pub fn pg_mut(&self) -> *mut Pg { 
        (self.0 + 0x2c) as *mut Pg
    }

    #[doc="Get the *const pointer for the PG register."]
    #[inline] pub fn pg_ptr(&self) -> *const Pg { 
           self.pg_mut()
    }

    #[doc="Read the PG register."]
    #[inline] pub fn pg(&self) -> Pg { 
        unsafe {
            read_volatile(self.pg_ptr())
        }
    }

    #[doc="Write the PG register."]
    #[inline] pub fn set_pg<F: FnOnce(Pg) -> Pg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pg_mut(), f(Pg(0)));
        }
        self
    }

    #[doc="Modify the PG register."]
    #[inline] pub fn with_pg<F: FnOnce(Pg) -> Pg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pg_mut(), f(self.pg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MG register."]
    #[inline] pub fn mg_mut(&self) -> *mut Mg { 
        (self.0 + 0x30) as *mut Mg
    }

    #[doc="Get the *const pointer for the MG register."]
    #[inline] pub fn mg_ptr(&self) -> *const Mg { 
           self.mg_mut()
    }

    #[doc="Read the MG register."]
    #[inline] pub fn mg(&self) -> Mg { 
        unsafe {
            read_volatile(self.mg_ptr())
        }
    }

    #[doc="Write the MG register."]
    #[inline] pub fn set_mg<F: FnOnce(Mg) -> Mg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mg_mut(), f(Mg(0)));
        }
        self
    }

    #[doc="Modify the MG register."]
    #[inline] pub fn with_mg<F: FnOnce(Mg) -> Mg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mg_mut(), f(self.mg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLPD register."]
    #[inline] pub fn clpd_mut(&self) -> *mut Clpd { 
        (self.0 + 0x34) as *mut Clpd
    }

    #[doc="Get the *const pointer for the CLPD register."]
    #[inline] pub fn clpd_ptr(&self) -> *const Clpd { 
           self.clpd_mut()
    }

    #[doc="Read the CLPD register."]
    #[inline] pub fn clpd(&self) -> Clpd { 
        unsafe {
            read_volatile(self.clpd_ptr())
        }
    }

    #[doc="Write the CLPD register."]
    #[inline] pub fn set_clpd<F: FnOnce(Clpd) -> Clpd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clpd_mut(), f(Clpd(0)));
        }
        self
    }

    #[doc="Modify the CLPD register."]
    #[inline] pub fn with_clpd<F: FnOnce(Clpd) -> Clpd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clpd_mut(), f(self.clpd()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLPS register."]
    #[inline] pub fn clps_mut(&self) -> *mut Clps { 
        (self.0 + 0x38) as *mut Clps
    }

    #[doc="Get the *const pointer for the CLPS register."]
    #[inline] pub fn clps_ptr(&self) -> *const Clps { 
           self.clps_mut()
    }

    #[doc="Read the CLPS register."]
    #[inline] pub fn clps(&self) -> Clps { 
        unsafe {
            read_volatile(self.clps_ptr())
        }
    }

    #[doc="Write the CLPS register."]
    #[inline] pub fn set_clps<F: FnOnce(Clps) -> Clps>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clps_mut(), f(Clps(0)));
        }
        self
    }

    #[doc="Modify the CLPS register."]
    #[inline] pub fn with_clps<F: FnOnce(Clps) -> Clps>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clps_mut(), f(self.clps()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLP4 register."]
    #[inline] pub fn clp4_mut(&self) -> *mut Clp4 { 
        (self.0 + 0x3c) as *mut Clp4
    }

    #[doc="Get the *const pointer for the CLP4 register."]
    #[inline] pub fn clp4_ptr(&self) -> *const Clp4 { 
           self.clp4_mut()
    }

    #[doc="Read the CLP4 register."]
    #[inline] pub fn clp4(&self) -> Clp4 { 
        unsafe {
            read_volatile(self.clp4_ptr())
        }
    }

    #[doc="Write the CLP4 register."]
    #[inline] pub fn set_clp4<F: FnOnce(Clp4) -> Clp4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clp4_mut(), f(Clp4(0)));
        }
        self
    }

    #[doc="Modify the CLP4 register."]
    #[inline] pub fn with_clp4<F: FnOnce(Clp4) -> Clp4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clp4_mut(), f(self.clp4()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLP3 register."]
    #[inline] pub fn clp3_mut(&self) -> *mut Clp3 { 
        (self.0 + 0x40) as *mut Clp3
    }

    #[doc="Get the *const pointer for the CLP3 register."]
    #[inline] pub fn clp3_ptr(&self) -> *const Clp3 { 
           self.clp3_mut()
    }

    #[doc="Read the CLP3 register."]
    #[inline] pub fn clp3(&self) -> Clp3 { 
        unsafe {
            read_volatile(self.clp3_ptr())
        }
    }

    #[doc="Write the CLP3 register."]
    #[inline] pub fn set_clp3<F: FnOnce(Clp3) -> Clp3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clp3_mut(), f(Clp3(0)));
        }
        self
    }

    #[doc="Modify the CLP3 register."]
    #[inline] pub fn with_clp3<F: FnOnce(Clp3) -> Clp3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clp3_mut(), f(self.clp3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLP2 register."]
    #[inline] pub fn clp2_mut(&self) -> *mut Clp2 { 
        (self.0 + 0x44) as *mut Clp2
    }

    #[doc="Get the *const pointer for the CLP2 register."]
    #[inline] pub fn clp2_ptr(&self) -> *const Clp2 { 
           self.clp2_mut()
    }

    #[doc="Read the CLP2 register."]
    #[inline] pub fn clp2(&self) -> Clp2 { 
        unsafe {
            read_volatile(self.clp2_ptr())
        }
    }

    #[doc="Write the CLP2 register."]
    #[inline] pub fn set_clp2<F: FnOnce(Clp2) -> Clp2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clp2_mut(), f(Clp2(0)));
        }
        self
    }

    #[doc="Modify the CLP2 register."]
    #[inline] pub fn with_clp2<F: FnOnce(Clp2) -> Clp2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clp2_mut(), f(self.clp2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLP1 register."]
    #[inline] pub fn clp1_mut(&self) -> *mut Clp1 { 
        (self.0 + 0x48) as *mut Clp1
    }

    #[doc="Get the *const pointer for the CLP1 register."]
    #[inline] pub fn clp1_ptr(&self) -> *const Clp1 { 
           self.clp1_mut()
    }

    #[doc="Read the CLP1 register."]
    #[inline] pub fn clp1(&self) -> Clp1 { 
        unsafe {
            read_volatile(self.clp1_ptr())
        }
    }

    #[doc="Write the CLP1 register."]
    #[inline] pub fn set_clp1<F: FnOnce(Clp1) -> Clp1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clp1_mut(), f(Clp1(0)));
        }
        self
    }

    #[doc="Modify the CLP1 register."]
    #[inline] pub fn with_clp1<F: FnOnce(Clp1) -> Clp1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clp1_mut(), f(self.clp1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLP0 register."]
    #[inline] pub fn clp0_mut(&self) -> *mut Clp0 { 
        (self.0 + 0x4c) as *mut Clp0
    }

    #[doc="Get the *const pointer for the CLP0 register."]
    #[inline] pub fn clp0_ptr(&self) -> *const Clp0 { 
           self.clp0_mut()
    }

    #[doc="Read the CLP0 register."]
    #[inline] pub fn clp0(&self) -> Clp0 { 
        unsafe {
            read_volatile(self.clp0_ptr())
        }
    }

    #[doc="Write the CLP0 register."]
    #[inline] pub fn set_clp0<F: FnOnce(Clp0) -> Clp0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clp0_mut(), f(Clp0(0)));
        }
        self
    }

    #[doc="Modify the CLP0 register."]
    #[inline] pub fn with_clp0<F: FnOnce(Clp0) -> Clp0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clp0_mut(), f(self.clp0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLMD register."]
    #[inline] pub fn clmd_mut(&self) -> *mut Clmd { 
        (self.0 + 0x54) as *mut Clmd
    }

    #[doc="Get the *const pointer for the CLMD register."]
    #[inline] pub fn clmd_ptr(&self) -> *const Clmd { 
           self.clmd_mut()
    }

    #[doc="Read the CLMD register."]
    #[inline] pub fn clmd(&self) -> Clmd { 
        unsafe {
            read_volatile(self.clmd_ptr())
        }
    }

    #[doc="Write the CLMD register."]
    #[inline] pub fn set_clmd<F: FnOnce(Clmd) -> Clmd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clmd_mut(), f(Clmd(0)));
        }
        self
    }

    #[doc="Modify the CLMD register."]
    #[inline] pub fn with_clmd<F: FnOnce(Clmd) -> Clmd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clmd_mut(), f(self.clmd()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLMS register."]
    #[inline] pub fn clms_mut(&self) -> *mut Clms { 
        (self.0 + 0x58) as *mut Clms
    }

    #[doc="Get the *const pointer for the CLMS register."]
    #[inline] pub fn clms_ptr(&self) -> *const Clms { 
           self.clms_mut()
    }

    #[doc="Read the CLMS register."]
    #[inline] pub fn clms(&self) -> Clms { 
        unsafe {
            read_volatile(self.clms_ptr())
        }
    }

    #[doc="Write the CLMS register."]
    #[inline] pub fn set_clms<F: FnOnce(Clms) -> Clms>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clms_mut(), f(Clms(0)));
        }
        self
    }

    #[doc="Modify the CLMS register."]
    #[inline] pub fn with_clms<F: FnOnce(Clms) -> Clms>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clms_mut(), f(self.clms()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLM4 register."]
    #[inline] pub fn clm4_mut(&self) -> *mut Clm4 { 
        (self.0 + 0x5c) as *mut Clm4
    }

    #[doc="Get the *const pointer for the CLM4 register."]
    #[inline] pub fn clm4_ptr(&self) -> *const Clm4 { 
           self.clm4_mut()
    }

    #[doc="Read the CLM4 register."]
    #[inline] pub fn clm4(&self) -> Clm4 { 
        unsafe {
            read_volatile(self.clm4_ptr())
        }
    }

    #[doc="Write the CLM4 register."]
    #[inline] pub fn set_clm4<F: FnOnce(Clm4) -> Clm4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clm4_mut(), f(Clm4(0)));
        }
        self
    }

    #[doc="Modify the CLM4 register."]
    #[inline] pub fn with_clm4<F: FnOnce(Clm4) -> Clm4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clm4_mut(), f(self.clm4()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLM3 register."]
    #[inline] pub fn clm3_mut(&self) -> *mut Clm3 { 
        (self.0 + 0x60) as *mut Clm3
    }

    #[doc="Get the *const pointer for the CLM3 register."]
    #[inline] pub fn clm3_ptr(&self) -> *const Clm3 { 
           self.clm3_mut()
    }

    #[doc="Read the CLM3 register."]
    #[inline] pub fn clm3(&self) -> Clm3 { 
        unsafe {
            read_volatile(self.clm3_ptr())
        }
    }

    #[doc="Write the CLM3 register."]
    #[inline] pub fn set_clm3<F: FnOnce(Clm3) -> Clm3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clm3_mut(), f(Clm3(0)));
        }
        self
    }

    #[doc="Modify the CLM3 register."]
    #[inline] pub fn with_clm3<F: FnOnce(Clm3) -> Clm3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clm3_mut(), f(self.clm3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLM2 register."]
    #[inline] pub fn clm2_mut(&self) -> *mut Clm2 { 
        (self.0 + 0x64) as *mut Clm2
    }

    #[doc="Get the *const pointer for the CLM2 register."]
    #[inline] pub fn clm2_ptr(&self) -> *const Clm2 { 
           self.clm2_mut()
    }

    #[doc="Read the CLM2 register."]
    #[inline] pub fn clm2(&self) -> Clm2 { 
        unsafe {
            read_volatile(self.clm2_ptr())
        }
    }

    #[doc="Write the CLM2 register."]
    #[inline] pub fn set_clm2<F: FnOnce(Clm2) -> Clm2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clm2_mut(), f(Clm2(0)));
        }
        self
    }

    #[doc="Modify the CLM2 register."]
    #[inline] pub fn with_clm2<F: FnOnce(Clm2) -> Clm2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clm2_mut(), f(self.clm2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLM1 register."]
    #[inline] pub fn clm1_mut(&self) -> *mut Clm1 { 
        (self.0 + 0x68) as *mut Clm1
    }

    #[doc="Get the *const pointer for the CLM1 register."]
    #[inline] pub fn clm1_ptr(&self) -> *const Clm1 { 
           self.clm1_mut()
    }

    #[doc="Read the CLM1 register."]
    #[inline] pub fn clm1(&self) -> Clm1 { 
        unsafe {
            read_volatile(self.clm1_ptr())
        }
    }

    #[doc="Write the CLM1 register."]
    #[inline] pub fn set_clm1<F: FnOnce(Clm1) -> Clm1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clm1_mut(), f(Clm1(0)));
        }
        self
    }

    #[doc="Modify the CLM1 register."]
    #[inline] pub fn with_clm1<F: FnOnce(Clm1) -> Clm1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clm1_mut(), f(self.clm1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLM0 register."]
    #[inline] pub fn clm0_mut(&self) -> *mut Clm0 { 
        (self.0 + 0x6c) as *mut Clm0
    }

    #[doc="Get the *const pointer for the CLM0 register."]
    #[inline] pub fn clm0_ptr(&self) -> *const Clm0 { 
           self.clm0_mut()
    }

    #[doc="Read the CLM0 register."]
    #[inline] pub fn clm0(&self) -> Clm0 { 
        unsafe {
            read_volatile(self.clm0_ptr())
        }
    }

    #[doc="Write the CLM0 register."]
    #[inline] pub fn set_clm0<F: FnOnce(Clm0) -> Clm0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clm0_mut(), f(Clm0(0)));
        }
        self
    }

    #[doc="Modify the CLM0 register."]
    #[inline] pub fn with_clm0<F: FnOnce(Clm0) -> Clm0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clm0_mut(), f(self.clm0()));
        }
        self
    }

}

#[doc="ADC Status and Control Registers 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sc1(pub u32);
impl Sc1 {
    #[doc="Input channel select"]
    #[inline] pub fn adch(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if ADCH != 0"]
    #[inline] pub fn test_adch(&self) -> bool {
        self.adch() != 0
    }

    #[doc="Sets the ADCH field."]
    #[inline] pub fn set_adch<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Differential Mode Enable"]
    #[inline] pub fn diff(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DIFF != 0"]
    #[inline] pub fn test_diff(&self) -> bool {
        self.diff() != 0
    }

    #[doc="Sets the DIFF field."]
    #[inline] pub fn set_diff<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Interrupt Enable"]
    #[inline] pub fn aien(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if AIEN != 0"]
    #[inline] pub fn test_aien(&self) -> bool {
        self.aien() != 0
    }

    #[doc="Sets the AIEN field."]
    #[inline] pub fn set_aien<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Conversion Complete Flag"]
    #[inline] pub fn coco(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if COCO != 0"]
    #[inline] pub fn test_coco(&self) -> bool {
        self.coco() != 0
    }

    #[doc="Sets the COCO field."]
    #[inline] pub fn set_coco<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
    #[inline] pub fn adiclk(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if ADICLK != 0"]
    #[inline] pub fn test_adiclk(&self) -> bool {
        self.adiclk() != 0
    }

    #[doc="Sets the ADICLK field."]
    #[inline] pub fn set_adiclk<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Conversion mode selection"]
    #[inline] pub fn mode(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Sample Time Configuration"]
    #[inline] pub fn adlsmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ADLSMP != 0"]
    #[inline] pub fn test_adlsmp(&self) -> bool {
        self.adlsmp() != 0
    }

    #[doc="Sets the ADLSMP field."]
    #[inline] pub fn set_adlsmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Clock Divide Select"]
    #[inline] pub fn adiv(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if ADIV != 0"]
    #[inline] pub fn test_adiv(&self) -> bool {
        self.adiv() != 0
    }

    #[doc="Sets the ADIV field."]
    #[inline] pub fn set_adiv<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Low-Power Configuration"]
    #[inline] pub fn adlpc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ADLPC != 0"]
    #[inline] pub fn test_adlpc(&self) -> bool {
        self.adlpc() != 0
    }

    #[doc="Sets the ADLPC field."]
    #[inline] pub fn set_adlpc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
    #[inline] pub fn adlsts(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if ADLSTS != 0"]
    #[inline] pub fn test_adlsts(&self) -> bool {
        self.adlsts() != 0
    }

    #[doc="Sets the ADLSTS field."]
    #[inline] pub fn set_adlsts<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="High-Speed Configuration"]
    #[inline] pub fn adhsc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ADHSC != 0"]
    #[inline] pub fn test_adhsc(&self) -> bool {
        self.adhsc() != 0
    }

    #[doc="Sets the ADHSC field."]
    #[inline] pub fn set_adhsc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Asynchronous Clock Output Enable"]
    #[inline] pub fn adacken(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ADACKEN != 0"]
    #[inline] pub fn test_adacken(&self) -> bool {
        self.adacken() != 0
    }

    #[doc="Sets the ADACKEN field."]
    #[inline] pub fn set_adacken<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="ADC Mux Select"]
    #[inline] pub fn muxsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if MUXSEL != 0"]
    #[inline] pub fn test_muxsel(&self) -> bool {
        self.muxsel() != 0
    }

    #[doc="Sets the MUXSEL field."]
    #[inline] pub fn set_muxsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
    #[inline] pub fn d(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if D != 0"]
    #[inline] pub fn test_d(&self) -> bool {
        self.d() != 0
    }

    #[doc="Sets the D field."]
    #[inline] pub fn set_d<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data result (16 bit)"]
    #[inline] pub fn d16(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if D16 != 0"]
    #[inline] pub fn test_d16(&self) -> bool {
        self.d16() != 0
    }

    #[doc="Sets the D16 field."]
    #[inline] pub fn set_d16<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data result (12 bit)"]
    #[inline] pub fn d12(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if D12 != 0"]
    #[inline] pub fn test_d12(&self) -> bool {
        self.d12() != 0
    }

    #[doc="Sets the D12 field."]
    #[inline] pub fn set_d12<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data result (10 bit)"]
    #[inline] pub fn d10(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if D10 != 0"]
    #[inline] pub fn test_d10(&self) -> bool {
        self.d10() != 0
    }

    #[doc="Sets the D10 field."]
    #[inline] pub fn set_d10<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data result (8 bit)"]
    #[inline] pub fn d8(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if D8 != 0"]
    #[inline] pub fn test_d8(&self) -> bool {
        self.d8() != 0
    }

    #[doc="Sets the D8 field."]
    #[inline] pub fn set_d8<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
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
    #[inline] pub fn cv(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CV != 0"]
    #[inline] pub fn test_cv(&self) -> bool {
        self.cv() != 0
    }

    #[doc="Sets the CV field."]
    #[inline] pub fn set_cv<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
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
    #[inline] pub fn refsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if REFSEL != 0"]
    #[inline] pub fn test_refsel(&self) -> bool {
        self.refsel() != 0
    }

    #[doc="Sets the REFSEL field."]
    #[inline] pub fn set_refsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DMA Enable"]
    #[inline] pub fn dmaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DMAEN != 0"]
    #[inline] pub fn test_dmaen(&self) -> bool {
        self.dmaen() != 0
    }

    #[doc="Sets the DMAEN field."]
    #[inline] pub fn set_dmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Compare Function Range Enable"]
    #[inline] pub fn acren(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ACREN != 0"]
    #[inline] pub fn test_acren(&self) -> bool {
        self.acren() != 0
    }

    #[doc="Sets the ACREN field."]
    #[inline] pub fn set_acren<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Compare Function Greater Than Enable"]
    #[inline] pub fn acfgt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ACFGT != 0"]
    #[inline] pub fn test_acfgt(&self) -> bool {
        self.acfgt() != 0
    }

    #[doc="Sets the ACFGT field."]
    #[inline] pub fn set_acfgt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Compare Function Enable"]
    #[inline] pub fn acfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACFE != 0"]
    #[inline] pub fn test_acfe(&self) -> bool {
        self.acfe() != 0
    }

    #[doc="Sets the ACFE field."]
    #[inline] pub fn set_acfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Conversion Trigger Select"]
    #[inline] pub fn adtrg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if ADTRG != 0"]
    #[inline] pub fn test_adtrg(&self) -> bool {
        self.adtrg() != 0
    }

    #[doc="Sets the ADTRG field."]
    #[inline] pub fn set_adtrg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Conversion Active"]
    #[inline] pub fn adact(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ADACT != 0"]
    #[inline] pub fn test_adact(&self) -> bool {
        self.adact() != 0
    }

    #[doc="Sets the ADACT field."]
    #[inline] pub fn set_adact<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
    #[inline] pub fn avgs(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if AVGS != 0"]
    #[inline] pub fn test_avgs(&self) -> bool {
        self.avgs() != 0
    }

    #[doc="Sets the AVGS field."]
    #[inline] pub fn set_avgs<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Hardware Average Enable"]
    #[inline] pub fn avge(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if AVGE != 0"]
    #[inline] pub fn test_avge(&self) -> bool {
        self.avge() != 0
    }

    #[doc="Sets the AVGE field."]
    #[inline] pub fn set_avge<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Continuous Conversion Enable"]
    #[inline] pub fn adco(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ADCO != 0"]
    #[inline] pub fn test_adco(&self) -> bool {
        self.adco() != 0
    }

    #[doc="Sets the ADCO field."]
    #[inline] pub fn set_adco<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Calibration Failed Flag"]
    #[inline] pub fn calf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CALF != 0"]
    #[inline] pub fn test_calf(&self) -> bool {
        self.calf() != 0
    }

    #[doc="Sets the CALF field."]
    #[inline] pub fn set_calf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Calibration"]
    #[inline] pub fn cal(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CAL != 0"]
    #[inline] pub fn test_cal(&self) -> bool {
        self.cal() != 0
    }

    #[doc="Sets the CAL field."]
    #[inline] pub fn set_cal<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
    #[inline] pub fn ofs(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if OFS != 0"]
    #[inline] pub fn test_ofs(&self) -> bool {
        self.ofs() != 0
    }

    #[doc="Sets the OFS field."]
    #[inline] pub fn set_ofs<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
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
    #[inline] pub fn pg(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if PG != 0"]
    #[inline] pub fn test_pg(&self) -> bool {
        self.pg() != 0
    }

    #[doc="Sets the PG field."]
    #[inline] pub fn set_pg<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
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
    #[inline] pub fn mg(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if MG != 0"]
    #[inline] pub fn test_mg(&self) -> bool {
        self.mg() != 0
    }

    #[doc="Sets the MG field."]
    #[inline] pub fn set_mg<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
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
    #[inline] pub fn clpd(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if CLPD != 0"]
    #[inline] pub fn test_clpd(&self) -> bool {
        self.clpd() != 0
    }

    #[doc="Sets the CLPD field."]
    #[inline] pub fn set_clpd<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
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
    #[inline] pub fn clps(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if CLPS != 0"]
    #[inline] pub fn test_clps(&self) -> bool {
        self.clps() != 0
    }

    #[doc="Sets the CLPS field."]
    #[inline] pub fn set_clps<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
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
    #[inline] pub fn clp4(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if CLP4 != 0"]
    #[inline] pub fn test_clp4(&self) -> bool {
        self.clp4() != 0
    }

    #[doc="Sets the CLP4 field."]
    #[inline] pub fn set_clp4<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
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
    #[inline] pub fn clp3(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if CLP3 != 0"]
    #[inline] pub fn test_clp3(&self) -> bool {
        self.clp3() != 0
    }

    #[doc="Sets the CLP3 field."]
    #[inline] pub fn set_clp3<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
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
    #[inline] pub fn clp2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CLP2 != 0"]
    #[inline] pub fn test_clp2(&self) -> bool {
        self.clp2() != 0
    }

    #[doc="Sets the CLP2 field."]
    #[inline] pub fn set_clp2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
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
    #[inline] pub fn clp1(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if CLP1 != 0"]
    #[inline] pub fn test_clp1(&self) -> bool {
        self.clp1() != 0
    }

    #[doc="Sets the CLP1 field."]
    #[inline] pub fn set_clp1<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
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
    #[inline] pub fn clp0(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if CLP0 != 0"]
    #[inline] pub fn test_clp0(&self) -> bool {
        self.clp0() != 0
    }

    #[doc="Sets the CLP0 field."]
    #[inline] pub fn set_clp0<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
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
    #[inline] pub fn clmd(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if CLMD != 0"]
    #[inline] pub fn test_clmd(&self) -> bool {
        self.clmd() != 0
    }

    #[doc="Sets the CLMD field."]
    #[inline] pub fn set_clmd<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
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
    #[inline] pub fn clms(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if CLMS != 0"]
    #[inline] pub fn test_clms(&self) -> bool {
        self.clms() != 0
    }

    #[doc="Sets the CLMS field."]
    #[inline] pub fn set_clms<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
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
    #[inline] pub fn clm4(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if CLM4 != 0"]
    #[inline] pub fn test_clm4(&self) -> bool {
        self.clm4() != 0
    }

    #[doc="Sets the CLM4 field."]
    #[inline] pub fn set_clm4<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
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
    #[inline] pub fn clm3(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if CLM3 != 0"]
    #[inline] pub fn test_clm3(&self) -> bool {
        self.clm3() != 0
    }

    #[doc="Sets the CLM3 field."]
    #[inline] pub fn set_clm3<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
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
    #[inline] pub fn clm2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CLM2 != 0"]
    #[inline] pub fn test_clm2(&self) -> bool {
        self.clm2() != 0
    }

    #[doc="Sets the CLM2 field."]
    #[inline] pub fn set_clm2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
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
    #[inline] pub fn clm1(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if CLM1 != 0"]
    #[inline] pub fn test_clm1(&self) -> bool {
        self.clm1() != 0
    }

    #[doc="Sets the CLM1 field."]
    #[inline] pub fn set_clm1<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
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
    #[inline] pub fn clm0(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if CLM0 != 0"]
    #[inline] pub fn test_clm0(&self) -> bool {
        self.clm0() != 0
    }

    #[doc="Sets the CLM0 field."]
    #[inline] pub fn set_clm0<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
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

pub struct AdcCh { pub periph: AdcPeriph, pub index: usize }

