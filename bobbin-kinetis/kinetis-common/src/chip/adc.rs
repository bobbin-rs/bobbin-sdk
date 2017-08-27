#[allow(unused_imports)] use bobbin_common::*;


#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="ADC Peripheral"]
pub struct AdcPeriph(pub usize); 


impl AdcPeriph {
#[doc="Get the *const pointer for the SC1 register."]
   #[inline] pub fn sc1_ptr<I: Into<bits::R2>>(&self, index: I) -> *const u32 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x0 + (index << 2)) as *const u32
   }
#[doc="Get the *mut pointer for the SC1 register."]
   #[inline] pub fn sc1_mut<I: Into<bits::R2>>(&self, index: I) -> *mut u32 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x0 + (index << 2)) as *mut u32
   }
#[doc="Read the SC1 register."]
   #[inline] pub fn sc1<I: Into<bits::R2>>(&self, index: I) -> Sc1 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Sc1(::core::ptr::read_volatile((self.0 + 0x0 + (index << 2)) as *const u32))
      }
   }
#[doc="Write the SC1 register."]
   #[inline] pub fn set_sc1<I: Into<bits::R2>, F: FnOnce(Sc1) -> Sc1>(&self, index: I, f: F) -> &Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Sc1(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0 + (index << 2)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SC1 register."]
   #[inline] pub fn with_sc1<I: Into<bits::R2> + Copy, F: FnOnce(Sc1) -> Sc1>(&self, index: I, f: F) -> &Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.sc1(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0 + (index << 2)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CFG1 register."]
   #[inline] pub fn cfg1_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x8) as *const u32
   }
#[doc="Get the *mut pointer for the CFG1 register."]
   #[inline] pub fn cfg1_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x8) as *mut u32
   }
#[doc="Read the CFG1 register."]
   #[inline] pub fn cfg1(&self) -> Cfg1 { 
      unsafe {
         Cfg1(::core::ptr::read_volatile((self.0 + 0x8) as *const u32))
      }
   }
#[doc="Write the CFG1 register."]
   #[inline] pub fn set_cfg1<F: FnOnce(Cfg1) -> Cfg1>(&self, f: F) -> &Self {
      let value = f(Cfg1(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CFG1 register."]
   #[inline] pub fn with_cfg1<F: FnOnce(Cfg1) -> Cfg1>(&self, f: F) -> &Self {
      let tmp = self.cfg1();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x8) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CFG2 register."]
   #[inline] pub fn cfg2_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xc) as *const u32
   }
#[doc="Get the *mut pointer for the CFG2 register."]
   #[inline] pub fn cfg2_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xc) as *mut u32
   }
#[doc="Read the CFG2 register."]
   #[inline] pub fn cfg2(&self) -> Cfg2 { 
      unsafe {
         Cfg2(::core::ptr::read_volatile((self.0 + 0xc) as *const u32))
      }
   }
#[doc="Write the CFG2 register."]
   #[inline] pub fn set_cfg2<F: FnOnce(Cfg2) -> Cfg2>(&self, f: F) -> &Self {
      let value = f(Cfg2(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CFG2 register."]
   #[inline] pub fn with_cfg2<F: FnOnce(Cfg2) -> Cfg2>(&self, f: F) -> &Self {
      let tmp = self.cfg2();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the R register."]
   #[inline] pub fn r_ptr<I: Into<bits::R2>>(&self, index: I) -> *const u32 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x10 + (index << 2)) as *const u32
   }
#[doc="Get the *mut pointer for the R register."]
   #[inline] pub fn r_mut<I: Into<bits::R2>>(&self, index: I) -> *mut u32 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x10 + (index << 2)) as *mut u32
   }
#[doc="Read the R register."]
   #[inline] pub fn r<I: Into<bits::R2>>(&self, index: I) -> R { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         R(::core::ptr::read_volatile((self.0 + 0x10 + (index << 2)) as *const u32))
      }
   }

#[doc="Get the *const pointer for the CV register."]
   #[inline] pub fn cv_ptr<I: Into<bits::R2>>(&self, index: I) -> *const u32 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x18 + (index << 2)) as *const u32
   }
#[doc="Get the *mut pointer for the CV register."]
   #[inline] pub fn cv_mut<I: Into<bits::R2>>(&self, index: I) -> *mut u32 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x18 + (index << 2)) as *mut u32
   }
#[doc="Read the CV register."]
   #[inline] pub fn cv<I: Into<bits::R2>>(&self, index: I) -> Cv { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Cv(::core::ptr::read_volatile((self.0 + 0x18 + (index << 2)) as *const u32))
      }
   }
#[doc="Write the CV register."]
   #[inline] pub fn set_cv<I: Into<bits::R2>, F: FnOnce(Cv) -> Cv>(&self, index: I, f: F) -> &Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Cv(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x18 + (index << 2)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CV register."]
   #[inline] pub fn with_cv<I: Into<bits::R2> + Copy, F: FnOnce(Cv) -> Cv>(&self, index: I, f: F) -> &Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.cv(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x18 + (index << 2)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SC2 register."]
   #[inline] pub fn sc2_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x20) as *const u32
   }
#[doc="Get the *mut pointer for the SC2 register."]
   #[inline] pub fn sc2_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x20) as *mut u32
   }
#[doc="Read the SC2 register."]
   #[inline] pub fn sc2(&self) -> Sc2 { 
      unsafe {
         Sc2(::core::ptr::read_volatile((self.0 + 0x20) as *const u32))
      }
   }
#[doc="Write the SC2 register."]
   #[inline] pub fn set_sc2<F: FnOnce(Sc2) -> Sc2>(&self, f: F) -> &Self {
      let value = f(Sc2(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x20) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SC2 register."]
   #[inline] pub fn with_sc2<F: FnOnce(Sc2) -> Sc2>(&self, f: F) -> &Self {
      let tmp = self.sc2();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x20) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SC3 register."]
   #[inline] pub fn sc3_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x24) as *const u32
   }
#[doc="Get the *mut pointer for the SC3 register."]
   #[inline] pub fn sc3_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x24) as *mut u32
   }
#[doc="Read the SC3 register."]
   #[inline] pub fn sc3(&self) -> Sc3 { 
      unsafe {
         Sc3(::core::ptr::read_volatile((self.0 + 0x24) as *const u32))
      }
   }
#[doc="Write the SC3 register."]
   #[inline] pub fn set_sc3<F: FnOnce(Sc3) -> Sc3>(&self, f: F) -> &Self {
      let value = f(Sc3(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x24) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SC3 register."]
   #[inline] pub fn with_sc3<F: FnOnce(Sc3) -> Sc3>(&self, f: F) -> &Self {
      let tmp = self.sc3();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x24) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the OFS register."]
   #[inline] pub fn ofs_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x28) as *const u32
   }
#[doc="Get the *mut pointer for the OFS register."]
   #[inline] pub fn ofs_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x28) as *mut u32
   }
#[doc="Read the OFS register."]
   #[inline] pub fn ofs(&self) -> Ofs { 
      unsafe {
         Ofs(::core::ptr::read_volatile((self.0 + 0x28) as *const u32))
      }
   }
#[doc="Write the OFS register."]
   #[inline] pub fn set_ofs<F: FnOnce(Ofs) -> Ofs>(&self, f: F) -> &Self {
      let value = f(Ofs(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x28) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the OFS register."]
   #[inline] pub fn with_ofs<F: FnOnce(Ofs) -> Ofs>(&self, f: F) -> &Self {
      let tmp = self.ofs();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x28) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PG register."]
   #[inline] pub fn pg_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x2c) as *const u32
   }
#[doc="Get the *mut pointer for the PG register."]
   #[inline] pub fn pg_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x2c) as *mut u32
   }
#[doc="Read the PG register."]
   #[inline] pub fn pg(&self) -> Pg { 
      unsafe {
         Pg(::core::ptr::read_volatile((self.0 + 0x2c) as *const u32))
      }
   }
#[doc="Write the PG register."]
   #[inline] pub fn set_pg<F: FnOnce(Pg) -> Pg>(&self, f: F) -> &Self {
      let value = f(Pg(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x2c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PG register."]
   #[inline] pub fn with_pg<F: FnOnce(Pg) -> Pg>(&self, f: F) -> &Self {
      let tmp = self.pg();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x2c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MG register."]
   #[inline] pub fn mg_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x30) as *const u32
   }
#[doc="Get the *mut pointer for the MG register."]
   #[inline] pub fn mg_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x30) as *mut u32
   }
#[doc="Read the MG register."]
   #[inline] pub fn mg(&self) -> Mg { 
      unsafe {
         Mg(::core::ptr::read_volatile((self.0 + 0x30) as *const u32))
      }
   }
#[doc="Write the MG register."]
   #[inline] pub fn set_mg<F: FnOnce(Mg) -> Mg>(&self, f: F) -> &Self {
      let value = f(Mg(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x30) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MG register."]
   #[inline] pub fn with_mg<F: FnOnce(Mg) -> Mg>(&self, f: F) -> &Self {
      let tmp = self.mg();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x30) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLPD register."]
   #[inline] pub fn clpd_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x34) as *const u32
   }
#[doc="Get the *mut pointer for the CLPD register."]
   #[inline] pub fn clpd_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x34) as *mut u32
   }
#[doc="Read the CLPD register."]
   #[inline] pub fn clpd(&self) -> Clpd { 
      unsafe {
         Clpd(::core::ptr::read_volatile((self.0 + 0x34) as *const u32))
      }
   }
#[doc="Write the CLPD register."]
   #[inline] pub fn set_clpd<F: FnOnce(Clpd) -> Clpd>(&self, f: F) -> &Self {
      let value = f(Clpd(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x34) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLPD register."]
   #[inline] pub fn with_clpd<F: FnOnce(Clpd) -> Clpd>(&self, f: F) -> &Self {
      let tmp = self.clpd();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x34) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLPS register."]
   #[inline] pub fn clps_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x38) as *const u32
   }
#[doc="Get the *mut pointer for the CLPS register."]
   #[inline] pub fn clps_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x38) as *mut u32
   }
#[doc="Read the CLPS register."]
   #[inline] pub fn clps(&self) -> Clps { 
      unsafe {
         Clps(::core::ptr::read_volatile((self.0 + 0x38) as *const u32))
      }
   }
#[doc="Write the CLPS register."]
   #[inline] pub fn set_clps<F: FnOnce(Clps) -> Clps>(&self, f: F) -> &Self {
      let value = f(Clps(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x38) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLPS register."]
   #[inline] pub fn with_clps<F: FnOnce(Clps) -> Clps>(&self, f: F) -> &Self {
      let tmp = self.clps();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x38) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLP4 register."]
   #[inline] pub fn clp4_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x3c) as *const u32
   }
#[doc="Get the *mut pointer for the CLP4 register."]
   #[inline] pub fn clp4_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x3c) as *mut u32
   }
#[doc="Read the CLP4 register."]
   #[inline] pub fn clp4(&self) -> Clp4 { 
      unsafe {
         Clp4(::core::ptr::read_volatile((self.0 + 0x3c) as *const u32))
      }
   }
#[doc="Write the CLP4 register."]
   #[inline] pub fn set_clp4<F: FnOnce(Clp4) -> Clp4>(&self, f: F) -> &Self {
      let value = f(Clp4(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x3c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLP4 register."]
   #[inline] pub fn with_clp4<F: FnOnce(Clp4) -> Clp4>(&self, f: F) -> &Self {
      let tmp = self.clp4();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x3c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLP3 register."]
   #[inline] pub fn clp3_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x40) as *const u32
   }
#[doc="Get the *mut pointer for the CLP3 register."]
   #[inline] pub fn clp3_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x40) as *mut u32
   }
#[doc="Read the CLP3 register."]
   #[inline] pub fn clp3(&self) -> Clp3 { 
      unsafe {
         Clp3(::core::ptr::read_volatile((self.0 + 0x40) as *const u32))
      }
   }
#[doc="Write the CLP3 register."]
   #[inline] pub fn set_clp3<F: FnOnce(Clp3) -> Clp3>(&self, f: F) -> &Self {
      let value = f(Clp3(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x40) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLP3 register."]
   #[inline] pub fn with_clp3<F: FnOnce(Clp3) -> Clp3>(&self, f: F) -> &Self {
      let tmp = self.clp3();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x40) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLP2 register."]
   #[inline] pub fn clp2_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x44) as *const u32
   }
#[doc="Get the *mut pointer for the CLP2 register."]
   #[inline] pub fn clp2_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x44) as *mut u32
   }
#[doc="Read the CLP2 register."]
   #[inline] pub fn clp2(&self) -> Clp2 { 
      unsafe {
         Clp2(::core::ptr::read_volatile((self.0 + 0x44) as *const u32))
      }
   }
#[doc="Write the CLP2 register."]
   #[inline] pub fn set_clp2<F: FnOnce(Clp2) -> Clp2>(&self, f: F) -> &Self {
      let value = f(Clp2(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x44) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLP2 register."]
   #[inline] pub fn with_clp2<F: FnOnce(Clp2) -> Clp2>(&self, f: F) -> &Self {
      let tmp = self.clp2();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x44) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLP1 register."]
   #[inline] pub fn clp1_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x48) as *const u32
   }
#[doc="Get the *mut pointer for the CLP1 register."]
   #[inline] pub fn clp1_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x48) as *mut u32
   }
#[doc="Read the CLP1 register."]
   #[inline] pub fn clp1(&self) -> Clp1 { 
      unsafe {
         Clp1(::core::ptr::read_volatile((self.0 + 0x48) as *const u32))
      }
   }
#[doc="Write the CLP1 register."]
   #[inline] pub fn set_clp1<F: FnOnce(Clp1) -> Clp1>(&self, f: F) -> &Self {
      let value = f(Clp1(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x48) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLP1 register."]
   #[inline] pub fn with_clp1<F: FnOnce(Clp1) -> Clp1>(&self, f: F) -> &Self {
      let tmp = self.clp1();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x48) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLP0 register."]
   #[inline] pub fn clp0_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x4c) as *const u32
   }
#[doc="Get the *mut pointer for the CLP0 register."]
   #[inline] pub fn clp0_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x4c) as *mut u32
   }
#[doc="Read the CLP0 register."]
   #[inline] pub fn clp0(&self) -> Clp0 { 
      unsafe {
         Clp0(::core::ptr::read_volatile((self.0 + 0x4c) as *const u32))
      }
   }
#[doc="Write the CLP0 register."]
   #[inline] pub fn set_clp0<F: FnOnce(Clp0) -> Clp0>(&self, f: F) -> &Self {
      let value = f(Clp0(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x4c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLP0 register."]
   #[inline] pub fn with_clp0<F: FnOnce(Clp0) -> Clp0>(&self, f: F) -> &Self {
      let tmp = self.clp0();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x4c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLMD register."]
   #[inline] pub fn clmd_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x54) as *const u32
   }
#[doc="Get the *mut pointer for the CLMD register."]
   #[inline] pub fn clmd_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x54) as *mut u32
   }
#[doc="Read the CLMD register."]
   #[inline] pub fn clmd(&self) -> Clmd { 
      unsafe {
         Clmd(::core::ptr::read_volatile((self.0 + 0x54) as *const u32))
      }
   }
#[doc="Write the CLMD register."]
   #[inline] pub fn set_clmd<F: FnOnce(Clmd) -> Clmd>(&self, f: F) -> &Self {
      let value = f(Clmd(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x54) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLMD register."]
   #[inline] pub fn with_clmd<F: FnOnce(Clmd) -> Clmd>(&self, f: F) -> &Self {
      let tmp = self.clmd();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x54) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLMS register."]
   #[inline] pub fn clms_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x58) as *const u32
   }
#[doc="Get the *mut pointer for the CLMS register."]
   #[inline] pub fn clms_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x58) as *mut u32
   }
#[doc="Read the CLMS register."]
   #[inline] pub fn clms(&self) -> Clms { 
      unsafe {
         Clms(::core::ptr::read_volatile((self.0 + 0x58) as *const u32))
      }
   }
#[doc="Write the CLMS register."]
   #[inline] pub fn set_clms<F: FnOnce(Clms) -> Clms>(&self, f: F) -> &Self {
      let value = f(Clms(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x58) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLMS register."]
   #[inline] pub fn with_clms<F: FnOnce(Clms) -> Clms>(&self, f: F) -> &Self {
      let tmp = self.clms();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x58) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLM4 register."]
   #[inline] pub fn clm4_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x5c) as *const u32
   }
#[doc="Get the *mut pointer for the CLM4 register."]
   #[inline] pub fn clm4_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x5c) as *mut u32
   }
#[doc="Read the CLM4 register."]
   #[inline] pub fn clm4(&self) -> Clm4 { 
      unsafe {
         Clm4(::core::ptr::read_volatile((self.0 + 0x5c) as *const u32))
      }
   }
#[doc="Write the CLM4 register."]
   #[inline] pub fn set_clm4<F: FnOnce(Clm4) -> Clm4>(&self, f: F) -> &Self {
      let value = f(Clm4(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x5c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLM4 register."]
   #[inline] pub fn with_clm4<F: FnOnce(Clm4) -> Clm4>(&self, f: F) -> &Self {
      let tmp = self.clm4();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x5c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLM3 register."]
   #[inline] pub fn clm3_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x60) as *const u32
   }
#[doc="Get the *mut pointer for the CLM3 register."]
   #[inline] pub fn clm3_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x60) as *mut u32
   }
#[doc="Read the CLM3 register."]
   #[inline] pub fn clm3(&self) -> Clm3 { 
      unsafe {
         Clm3(::core::ptr::read_volatile((self.0 + 0x60) as *const u32))
      }
   }
#[doc="Write the CLM3 register."]
   #[inline] pub fn set_clm3<F: FnOnce(Clm3) -> Clm3>(&self, f: F) -> &Self {
      let value = f(Clm3(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x60) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLM3 register."]
   #[inline] pub fn with_clm3<F: FnOnce(Clm3) -> Clm3>(&self, f: F) -> &Self {
      let tmp = self.clm3();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x60) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLM2 register."]
   #[inline] pub fn clm2_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x64) as *const u32
   }
#[doc="Get the *mut pointer for the CLM2 register."]
   #[inline] pub fn clm2_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x64) as *mut u32
   }
#[doc="Read the CLM2 register."]
   #[inline] pub fn clm2(&self) -> Clm2 { 
      unsafe {
         Clm2(::core::ptr::read_volatile((self.0 + 0x64) as *const u32))
      }
   }
#[doc="Write the CLM2 register."]
   #[inline] pub fn set_clm2<F: FnOnce(Clm2) -> Clm2>(&self, f: F) -> &Self {
      let value = f(Clm2(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x64) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLM2 register."]
   #[inline] pub fn with_clm2<F: FnOnce(Clm2) -> Clm2>(&self, f: F) -> &Self {
      let tmp = self.clm2();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x64) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLM1 register."]
   #[inline] pub fn clm1_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x68) as *const u32
   }
#[doc="Get the *mut pointer for the CLM1 register."]
   #[inline] pub fn clm1_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x68) as *mut u32
   }
#[doc="Read the CLM1 register."]
   #[inline] pub fn clm1(&self) -> Clm1 { 
      unsafe {
         Clm1(::core::ptr::read_volatile((self.0 + 0x68) as *const u32))
      }
   }
#[doc="Write the CLM1 register."]
   #[inline] pub fn set_clm1<F: FnOnce(Clm1) -> Clm1>(&self, f: F) -> &Self {
      let value = f(Clm1(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x68) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLM1 register."]
   #[inline] pub fn with_clm1<F: FnOnce(Clm1) -> Clm1>(&self, f: F) -> &Self {
      let tmp = self.clm1();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x68) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLM0 register."]
   #[inline] pub fn clm0_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x6c) as *const u32
   }
#[doc="Get the *mut pointer for the CLM0 register."]
   #[inline] pub fn clm0_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x6c) as *mut u32
   }
#[doc="Read the CLM0 register."]
   #[inline] pub fn clm0(&self) -> Clm0 { 
      unsafe {
         Clm0(::core::ptr::read_volatile((self.0 + 0x6c) as *const u32))
      }
   }
#[doc="Write the CLM0 register."]
   #[inline] pub fn set_clm0<F: FnOnce(Clm0) -> Clm0>(&self, f: F) -> &Self {
      let value = f(Clm0(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x6c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLM0 register."]
   #[inline] pub fn with_clm0<F: FnOnce(Clm0) -> Clm0>(&self, f: F) -> &Self {
      let tmp = self.clm0();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x6c) as *mut u32, value.0);
      }
      self
   }

}

#[doc="ADC Status and Control Registers 1"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sc1(pub u32);
impl Sc1 {
#[doc="Input channel select"]
   #[inline] pub fn adch(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
   }
#[doc="Input channel select"]
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
#[doc="Differential Mode Enable"]
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
#[doc="Interrupt Enable"]
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
#[doc="Conversion Complete Flag"]
   #[inline] pub fn set_coco<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cfg1(pub u32);
impl Cfg1 {
#[doc="Input Clock Select"]
   #[inline] pub fn adiclk(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="Input Clock Select"]
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
#[doc="Conversion mode selection"]
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
#[doc="Sample Time Configuration"]
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
#[doc="Clock Divide Select"]
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
#[doc="Low-Power Configuration"]
   #[inline] pub fn set_adlpc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cfg2(pub u32);
impl Cfg2 {
#[doc="Long Sample Time Select"]
   #[inline] pub fn adlsts(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="Long Sample Time Select"]
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
#[doc="High-Speed Configuration"]
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
#[doc="Asynchronous Clock Output Enable"]
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
#[doc="ADC Mux Select"]
   #[inline] pub fn set_muxsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct R(pub u32);
impl R {
#[doc="Data result"]
   #[inline] pub fn d(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Data result"]
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
#[doc="Data result (16 bit)"]
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
#[doc="Data result (12 bit)"]
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
#[doc="Data result (10 bit)"]
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
#[doc="Data result (8 bit)"]
   #[inline] pub fn set_d8<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cv(pub u32);
impl Cv {
#[doc="Compare Value."]
   #[inline] pub fn cv(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Compare Value."]
   #[inline] pub fn set_cv<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sc2(pub u32);
impl Sc2 {
#[doc="Voltage Reference Selection"]
   #[inline] pub fn refsel(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="Voltage Reference Selection"]
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
#[doc="DMA Enable"]
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
#[doc="Compare Function Range Enable"]
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
#[doc="Compare Function Greater Than Enable"]
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
#[doc="Compare Function Enable"]
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
#[doc="Conversion Trigger Select"]
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
#[doc="Conversion Active"]
   #[inline] pub fn set_adact<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sc3(pub u32);
impl Sc3 {
#[doc="Hardware Average Select"]
   #[inline] pub fn avgs(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="Hardware Average Select"]
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
#[doc="Hardware Average Enable"]
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
#[doc="Continuous Conversion Enable"]
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
#[doc="Calibration Failed Flag"]
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
#[doc="Calibration"]
   #[inline] pub fn set_cal<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ofs(pub u32);
impl Ofs {
#[doc="Offset Error Correction Value"]
   #[inline] pub fn ofs(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Offset Error Correction Value"]
   #[inline] pub fn set_ofs<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pg(pub u32);
impl Pg {
#[doc="Plus-Side Gain"]
   #[inline] pub fn pg(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Plus-Side Gain"]
   #[inline] pub fn set_pg<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mg(pub u32);
impl Mg {
#[doc="Minus-Side Gain"]
   #[inline] pub fn mg(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Minus-Side Gain"]
   #[inline] pub fn set_mg<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clpd(pub u32);
impl Clpd {
#[doc="Calibration Value"]
   #[inline] pub fn clpd(&self) -> bits::U6 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
   }
#[doc="Calibration Value"]
   #[inline] pub fn set_clpd<V: Into<bits::U6>>(mut self, value: V) -> Self {
      let value: bits::U6 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3f << 0);
      self.0 |= value << 0;
      self
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clps(pub u32);
impl Clps {
#[doc="Calibration Value"]
   #[inline] pub fn clps(&self) -> bits::U6 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
   }
#[doc="Calibration Value"]
   #[inline] pub fn set_clps<V: Into<bits::U6>>(mut self, value: V) -> Self {
      let value: bits::U6 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3f << 0);
      self.0 |= value << 0;
      self
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clp4(pub u32);
impl Clp4 {
#[doc="Calibration Value"]
   #[inline] pub fn clp4(&self) -> bits::U10 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
   }
#[doc="Calibration Value"]
   #[inline] pub fn set_clp4<V: Into<bits::U10>>(mut self, value: V) -> Self {
      let value: bits::U10 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3ff << 0);
      self.0 |= value << 0;
      self
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clp3(pub u32);
impl Clp3 {
#[doc="Calibration Value"]
   #[inline] pub fn clp3(&self) -> bits::U9 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
   }
#[doc="Calibration Value"]
   #[inline] pub fn set_clp3<V: Into<bits::U9>>(mut self, value: V) -> Self {
      let value: bits::U9 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1ff << 0);
      self.0 |= value << 0;
      self
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clp2(pub u32);
impl Clp2 {
#[doc="Calibration Value"]
   #[inline] pub fn clp2(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Calibration Value"]
   #[inline] pub fn set_clp2<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clp1(pub u32);
impl Clp1 {
#[doc="Calibration Value"]
   #[inline] pub fn clp1(&self) -> bits::U7 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
   }
#[doc="Calibration Value"]
   #[inline] pub fn set_clp1<V: Into<bits::U7>>(mut self, value: V) -> Self {
      let value: bits::U7 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7f << 0);
      self.0 |= value << 0;
      self
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clp0(pub u32);
impl Clp0 {
#[doc="Calibration Value"]
   #[inline] pub fn clp0(&self) -> bits::U6 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
   }
#[doc="Calibration Value"]
   #[inline] pub fn set_clp0<V: Into<bits::U6>>(mut self, value: V) -> Self {
      let value: bits::U6 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3f << 0);
      self.0 |= value << 0;
      self
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clmd(pub u32);
impl Clmd {
#[doc="Calibration Value"]
   #[inline] pub fn clmd(&self) -> bits::U6 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
   }
#[doc="Calibration Value"]
   #[inline] pub fn set_clmd<V: Into<bits::U6>>(mut self, value: V) -> Self {
      let value: bits::U6 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3f << 0);
      self.0 |= value << 0;
      self
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clms(pub u32);
impl Clms {
#[doc="Calibration Value"]
   #[inline] pub fn clms(&self) -> bits::U6 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
   }
#[doc="Calibration Value"]
   #[inline] pub fn set_clms<V: Into<bits::U6>>(mut self, value: V) -> Self {
      let value: bits::U6 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3f << 0);
      self.0 |= value << 0;
      self
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clm4(pub u32);
impl Clm4 {
#[doc="Calibration Value"]
   #[inline] pub fn clm4(&self) -> bits::U10 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
   }
#[doc="Calibration Value"]
   #[inline] pub fn set_clm4<V: Into<bits::U10>>(mut self, value: V) -> Self {
      let value: bits::U10 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3ff << 0);
      self.0 |= value << 0;
      self
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clm3(pub u32);
impl Clm3 {
#[doc="Calibration Value"]
   #[inline] pub fn clm3(&self) -> bits::U9 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
   }
#[doc="Calibration Value"]
   #[inline] pub fn set_clm3<V: Into<bits::U9>>(mut self, value: V) -> Self {
      let value: bits::U9 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1ff << 0);
      self.0 |= value << 0;
      self
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clm2(pub u32);
impl Clm2 {
#[doc="Calibration Value"]
   #[inline] pub fn clm2(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Calibration Value"]
   #[inline] pub fn set_clm2<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clm1(pub u32);
impl Clm1 {
#[doc="Calibration Value"]
   #[inline] pub fn clm1(&self) -> bits::U7 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
   }
#[doc="Calibration Value"]
   #[inline] pub fn set_clm1<V: Into<bits::U7>>(mut self, value: V) -> Self {
      let value: bits::U7 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7f << 0);
      self.0 |= value << 0;
      self
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clm0(pub u32);
impl Clm0 {
#[doc="Calibration Value"]
   #[inline] pub fn clm0(&self) -> bits::U6 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
   }
#[doc="Calibration Value"]
   #[inline] pub fn set_clm0<V: Into<bits::U6>>(mut self, value: V) -> Self {
      let value: bits::U6 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3f << 0);
      self.0 |= value << 0;
      self
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
