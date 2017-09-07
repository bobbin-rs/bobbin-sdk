#[allow(unused_imports)] use bobbin_common::*;

periph!( ADC0, Adc0, _ADC0, AdcPeriph, 0x4003b000);
periph!( ADC1, Adc1, _ADC1, AdcPeriph, 0x40027000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="ADC Peripheral"]
pub struct AdcPeriph(pub usize); 

impl super::sig::Signal<super::sig::Adc0Se0> for Adc0Ch0 {}
impl super::sig::SignalAdc<super::sig::Adc0Se0> for Adc0Ch0 {}
impl super::sig::Signal<super::sig::Adc0Se1> for Adc0Ch1 {}
impl super::sig::SignalAdc<super::sig::Adc0Se1> for Adc0Ch1 {}
impl super::sig::Signal<super::sig::Adc0Se2> for Adc0Ch2 {}
impl super::sig::SignalAdc<super::sig::Adc0Se2> for Adc0Ch2 {}
impl super::sig::Signal<super::sig::Adc0Se3> for Adc0Ch3 {}
impl super::sig::SignalAdc<super::sig::Adc0Se3> for Adc0Ch3 {}
impl super::sig::Signal<super::sig::Adc0Se4> for Adc0Ch4 {}
impl super::sig::SignalAdc<super::sig::Adc0Se4> for Adc0Ch4 {}
impl super::sig::Signal<super::sig::Adc0Se5> for Adc0Ch5 {}
impl super::sig::SignalAdc<super::sig::Adc0Se5> for Adc0Ch5 {}
impl super::sig::Signal<super::sig::Adc0Se6> for Adc0Ch6 {}
impl super::sig::SignalAdc<super::sig::Adc0Se6> for Adc0Ch6 {}
impl super::sig::Signal<super::sig::Adc0Se7> for Adc0Ch7 {}
impl super::sig::SignalAdc<super::sig::Adc0Se7> for Adc0Ch7 {}
impl super::sig::Signal<super::sig::Adc0Se8> for Adc0Ch8 {}
impl super::sig::SignalAdc<super::sig::Adc0Se8> for Adc0Ch8 {}
impl super::sig::Signal<super::sig::Adc0Se9> for Adc0Ch9 {}
impl super::sig::SignalAdc<super::sig::Adc0Se9> for Adc0Ch9 {}
impl super::sig::Signal<super::sig::Adc0Se10> for Adc0Ch10 {}
impl super::sig::SignalAdc<super::sig::Adc0Se10> for Adc0Ch10 {}
impl super::sig::Signal<super::sig::Adc0Se11> for Adc0Ch11 {}
impl super::sig::SignalAdc<super::sig::Adc0Se11> for Adc0Ch11 {}
impl super::sig::Signal<super::sig::Adc0Se12> for Adc0Ch12 {}
impl super::sig::SignalAdc<super::sig::Adc0Se12> for Adc0Ch12 {}
impl super::sig::Signal<super::sig::Adc0Se13> for Adc0Ch13 {}
impl super::sig::SignalAdc<super::sig::Adc0Se13> for Adc0Ch13 {}
impl super::sig::Signal<super::sig::Adc0Se14> for Adc0Ch14 {}
impl super::sig::SignalAdc<super::sig::Adc0Se14> for Adc0Ch14 {}
impl super::sig::Signal<super::sig::Adc0Se15> for Adc0Ch15 {}
impl super::sig::SignalAdc<super::sig::Adc0Se15> for Adc0Ch15 {}

impl super::sig::Signal<super::sig::Adc1Se0> for Adc1Ch0 {}
impl super::sig::SignalAdc<super::sig::Adc1Se0> for Adc1Ch0 {}
impl super::sig::Signal<super::sig::Adc1Se1> for Adc1Ch1 {}
impl super::sig::SignalAdc<super::sig::Adc1Se1> for Adc1Ch1 {}
impl super::sig::Signal<super::sig::Adc1Se2> for Adc1Ch2 {}
impl super::sig::SignalAdc<super::sig::Adc1Se2> for Adc1Ch2 {}
impl super::sig::Signal<super::sig::Adc1Se3> for Adc1Ch3 {}
impl super::sig::SignalAdc<super::sig::Adc1Se3> for Adc1Ch3 {}
impl super::sig::Signal<super::sig::Adc1Se4> for Adc1Ch4 {}
impl super::sig::SignalAdc<super::sig::Adc1Se4> for Adc1Ch4 {}
impl super::sig::Signal<super::sig::Adc1Se5> for Adc1Ch5 {}
impl super::sig::SignalAdc<super::sig::Adc1Se5> for Adc1Ch5 {}
impl super::sig::Signal<super::sig::Adc1Se6> for Adc1Ch6 {}
impl super::sig::SignalAdc<super::sig::Adc1Se6> for Adc1Ch6 {}
impl super::sig::Signal<super::sig::Adc1Se7> for Adc1Ch7 {}
impl super::sig::SignalAdc<super::sig::Adc1Se7> for Adc1Ch7 {}
impl super::sig::Signal<super::sig::Adc1Se8> for Adc1Ch8 {}
impl super::sig::SignalAdc<super::sig::Adc1Se8> for Adc1Ch8 {}
impl super::sig::Signal<super::sig::Adc1Se9> for Adc1Ch9 {}
impl super::sig::SignalAdc<super::sig::Adc1Se9> for Adc1Ch9 {}
impl super::sig::Signal<super::sig::Adc1Se10> for Adc1Ch10 {}
impl super::sig::SignalAdc<super::sig::Adc1Se10> for Adc1Ch10 {}
impl super::sig::Signal<super::sig::Adc1Se11> for Adc1Ch11 {}
impl super::sig::SignalAdc<super::sig::Adc1Se11> for Adc1Ch11 {}
impl super::sig::Signal<super::sig::Adc1Se12> for Adc1Ch12 {}
impl super::sig::SignalAdc<super::sig::Adc1Se12> for Adc1Ch12 {}
impl super::sig::Signal<super::sig::Adc1Se13> for Adc1Ch13 {}
impl super::sig::SignalAdc<super::sig::Adc1Se13> for Adc1Ch13 {}
impl super::sig::Signal<super::sig::Adc1Se14> for Adc1Ch14 {}
impl super::sig::SignalAdc<super::sig::Adc1Se14> for Adc1Ch14 {}
impl super::sig::Signal<super::sig::Adc1Se15> for Adc1Ch15 {}
impl super::sig::SignalAdc<super::sig::Adc1Se15> for Adc1Ch15 {}


impl AdcPeriph {
    #[doc="Get the *mut pointer for the SC1 register."]
    #[inline] pub fn sc1_mut<I: Into<bits::R16>>(&self, index: I) -> *mut Sc1 { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x0 + (index << 2)) as *mut Sc1
    }

    #[doc="Get the *const pointer for the SC1 register."]
    #[inline] pub fn sc1_ptr<I: Into<bits::R16>>(&self, index: I) -> *const Sc1 { 
           self.sc1_mut(index)
    }

    #[doc="Read the SC1 register."]
    #[inline] pub fn sc1<I: Into<bits::R16>>(&self, index: I) -> Sc1 { 
        unsafe {
            read_volatile(self.sc1_ptr(index))
        }
    }

    #[doc="Write the SC1 register."]
    #[inline] pub fn set_sc1<I: Into<bits::R16>, F: FnOnce(Sc1) -> Sc1>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.sc1_mut(index), f(Sc1(0)));
        }
        self
    }

    #[doc="Modify the SC1 register."]
    #[inline] pub fn with_sc1<I: Into<bits::R16> + Copy, F: FnOnce(Sc1) -> Sc1>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.sc1_mut(index), f(self.sc1(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the CFG1 register."]
    #[inline] pub fn cfg1_mut(&self) -> *mut Cfg1 { 
        (self.0 + 0x40) as *mut Cfg1
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
        (self.0 + 0x44) as *mut Cfg2
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
    #[inline] pub fn r_mut<I: Into<bits::R16>>(&self, index: I) -> *mut R { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x48 + (index << 2)) as *mut R
    }

    #[doc="Get the *const pointer for the R register."]
    #[inline] pub fn r_ptr<I: Into<bits::R16>>(&self, index: I) -> *const R { 
           self.r_mut(index)
    }

    #[doc="Read the R register."]
    #[inline] pub fn r<I: Into<bits::R16>>(&self, index: I) -> R { 
        unsafe {
            read_volatile(self.r_ptr(index))
        }
    }

    #[doc="Get the *mut pointer for the CV register."]
    #[inline] pub fn cv_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Cv { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x88 + (index << 2)) as *mut Cv
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
        (self.0 + 0x90) as *mut Sc2
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
        (self.0 + 0x94) as *mut Sc3
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

    #[doc="Get the *mut pointer for the BASE_OFS register."]
    #[inline] pub fn base_ofs_mut(&self) -> *mut BaseOfs { 
        (self.0 + 0x98) as *mut BaseOfs
    }

    #[doc="Get the *const pointer for the BASE_OFS register."]
    #[inline] pub fn base_ofs_ptr(&self) -> *const BaseOfs { 
           self.base_ofs_mut()
    }

    #[doc="Read the BASE_OFS register."]
    #[inline] pub fn base_ofs(&self) -> BaseOfs { 
        unsafe {
            read_volatile(self.base_ofs_ptr())
        }
    }

    #[doc="Write the BASE_OFS register."]
    #[inline] pub fn set_base_ofs<F: FnOnce(BaseOfs) -> BaseOfs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.base_ofs_mut(), f(BaseOfs(0)));
        }
        self
    }

    #[doc="Modify the BASE_OFS register."]
    #[inline] pub fn with_base_ofs<F: FnOnce(BaseOfs) -> BaseOfs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.base_ofs_mut(), f(self.base_ofs()));
        }
        self
    }

    #[doc="Get the *mut pointer for the OFS register."]
    #[inline] pub fn ofs_mut(&self) -> *mut Ofs { 
        (self.0 + 0x9c) as *mut Ofs
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

    #[doc="Get the *mut pointer for the USR_OFS register."]
    #[inline] pub fn usr_ofs_mut(&self) -> *mut UsrOfs { 
        (self.0 + 0xa0) as *mut UsrOfs
    }

    #[doc="Get the *const pointer for the USR_OFS register."]
    #[inline] pub fn usr_ofs_ptr(&self) -> *const UsrOfs { 
           self.usr_ofs_mut()
    }

    #[doc="Read the USR_OFS register."]
    #[inline] pub fn usr_ofs(&self) -> UsrOfs { 
        unsafe {
            read_volatile(self.usr_ofs_ptr())
        }
    }

    #[doc="Write the USR_OFS register."]
    #[inline] pub fn set_usr_ofs<F: FnOnce(UsrOfs) -> UsrOfs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.usr_ofs_mut(), f(UsrOfs(0)));
        }
        self
    }

    #[doc="Modify the USR_OFS register."]
    #[inline] pub fn with_usr_ofs<F: FnOnce(UsrOfs) -> UsrOfs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.usr_ofs_mut(), f(self.usr_ofs()));
        }
        self
    }

    #[doc="Get the *mut pointer for the XOFS register."]
    #[inline] pub fn xofs_mut(&self) -> *mut Xofs { 
        (self.0 + 0xa4) as *mut Xofs
    }

    #[doc="Get the *const pointer for the XOFS register."]
    #[inline] pub fn xofs_ptr(&self) -> *const Xofs { 
           self.xofs_mut()
    }

    #[doc="Read the XOFS register."]
    #[inline] pub fn xofs(&self) -> Xofs { 
        unsafe {
            read_volatile(self.xofs_ptr())
        }
    }

    #[doc="Write the XOFS register."]
    #[inline] pub fn set_xofs<F: FnOnce(Xofs) -> Xofs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.xofs_mut(), f(Xofs(0)));
        }
        self
    }

    #[doc="Modify the XOFS register."]
    #[inline] pub fn with_xofs<F: FnOnce(Xofs) -> Xofs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.xofs_mut(), f(self.xofs()));
        }
        self
    }

    #[doc="Get the *mut pointer for the YOFS register."]
    #[inline] pub fn yofs_mut(&self) -> *mut Yofs { 
        (self.0 + 0xa8) as *mut Yofs
    }

    #[doc="Get the *const pointer for the YOFS register."]
    #[inline] pub fn yofs_ptr(&self) -> *const Yofs { 
           self.yofs_mut()
    }

    #[doc="Read the YOFS register."]
    #[inline] pub fn yofs(&self) -> Yofs { 
        unsafe {
            read_volatile(self.yofs_ptr())
        }
    }

    #[doc="Write the YOFS register."]
    #[inline] pub fn set_yofs<F: FnOnce(Yofs) -> Yofs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.yofs_mut(), f(Yofs(0)));
        }
        self
    }

    #[doc="Modify the YOFS register."]
    #[inline] pub fn with_yofs<F: FnOnce(Yofs) -> Yofs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.yofs_mut(), f(self.yofs()));
        }
        self
    }

    #[doc="Get the *mut pointer for the G register."]
    #[inline] pub fn g_mut(&self) -> *mut G { 
        (self.0 + 0xac) as *mut G
    }

    #[doc="Get the *const pointer for the G register."]
    #[inline] pub fn g_ptr(&self) -> *const G { 
           self.g_mut()
    }

    #[doc="Read the G register."]
    #[inline] pub fn g(&self) -> G { 
        unsafe {
            read_volatile(self.g_ptr())
        }
    }

    #[doc="Write the G register."]
    #[inline] pub fn set_g<F: FnOnce(G) -> G>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.g_mut(), f(G(0)));
        }
        self
    }

    #[doc="Modify the G register."]
    #[inline] pub fn with_g<F: FnOnce(G) -> G>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.g_mut(), f(self.g()));
        }
        self
    }

    #[doc="Get the *mut pointer for the UG register."]
    #[inline] pub fn ug_mut(&self) -> *mut Ug { 
        (self.0 + 0xb0) as *mut Ug
    }

    #[doc="Get the *const pointer for the UG register."]
    #[inline] pub fn ug_ptr(&self) -> *const Ug { 
           self.ug_mut()
    }

    #[doc="Read the UG register."]
    #[inline] pub fn ug(&self) -> Ug { 
        unsafe {
            read_volatile(self.ug_ptr())
        }
    }

    #[doc="Write the UG register."]
    #[inline] pub fn set_ug<F: FnOnce(Ug) -> Ug>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ug_mut(), f(Ug(0)));
        }
        self
    }

    #[doc="Modify the UG register."]
    #[inline] pub fn with_ug<F: FnOnce(Ug) -> Ug>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ug_mut(), f(self.ug()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLPS register."]
    #[inline] pub fn clps_mut(&self) -> *mut Clps { 
        (self.0 + 0xb4) as *mut Clps
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

    #[doc="Get the *mut pointer for the CLP3 register."]
    #[inline] pub fn clp3_mut(&self) -> *mut Clp3 { 
        (self.0 + 0xb8) as *mut Clp3
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
        (self.0 + 0xbc) as *mut Clp2
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
        (self.0 + 0xc0) as *mut Clp1
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
        (self.0 + 0xc4) as *mut Clp0
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

    #[doc="Get the *mut pointer for the CLPX register."]
    #[inline] pub fn clpx_mut(&self) -> *mut Clpx { 
        (self.0 + 0xc8) as *mut Clpx
    }

    #[doc="Get the *const pointer for the CLPX register."]
    #[inline] pub fn clpx_ptr(&self) -> *const Clpx { 
           self.clpx_mut()
    }

    #[doc="Read the CLPX register."]
    #[inline] pub fn clpx(&self) -> Clpx { 
        unsafe {
            read_volatile(self.clpx_ptr())
        }
    }

    #[doc="Write the CLPX register."]
    #[inline] pub fn set_clpx<F: FnOnce(Clpx) -> Clpx>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clpx_mut(), f(Clpx(0)));
        }
        self
    }

    #[doc="Modify the CLPX register."]
    #[inline] pub fn with_clpx<F: FnOnce(Clpx) -> Clpx>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clpx_mut(), f(self.clpx()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLP9 register."]
    #[inline] pub fn clp9_mut(&self) -> *mut Clp9 { 
        (self.0 + 0xcc) as *mut Clp9
    }

    #[doc="Get the *const pointer for the CLP9 register."]
    #[inline] pub fn clp9_ptr(&self) -> *const Clp9 { 
           self.clp9_mut()
    }

    #[doc="Read the CLP9 register."]
    #[inline] pub fn clp9(&self) -> Clp9 { 
        unsafe {
            read_volatile(self.clp9_ptr())
        }
    }

    #[doc="Write the CLP9 register."]
    #[inline] pub fn set_clp9<F: FnOnce(Clp9) -> Clp9>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clp9_mut(), f(Clp9(0)));
        }
        self
    }

    #[doc="Modify the CLP9 register."]
    #[inline] pub fn with_clp9<F: FnOnce(Clp9) -> Clp9>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clp9_mut(), f(self.clp9()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLPS_OFS register."]
    #[inline] pub fn clps_ofs_mut(&self) -> *mut ClpsOfs { 
        (self.0 + 0xd0) as *mut ClpsOfs
    }

    #[doc="Get the *const pointer for the CLPS_OFS register."]
    #[inline] pub fn clps_ofs_ptr(&self) -> *const ClpsOfs { 
           self.clps_ofs_mut()
    }

    #[doc="Read the CLPS_OFS register."]
    #[inline] pub fn clps_ofs(&self) -> ClpsOfs { 
        unsafe {
            read_volatile(self.clps_ofs_ptr())
        }
    }

    #[doc="Write the CLPS_OFS register."]
    #[inline] pub fn set_clps_ofs<F: FnOnce(ClpsOfs) -> ClpsOfs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clps_ofs_mut(), f(ClpsOfs(0)));
        }
        self
    }

    #[doc="Modify the CLPS_OFS register."]
    #[inline] pub fn with_clps_ofs<F: FnOnce(ClpsOfs) -> ClpsOfs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clps_ofs_mut(), f(self.clps_ofs()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLP3_OFS register."]
    #[inline] pub fn clp3_ofs_mut(&self) -> *mut Clp3Ofs { 
        (self.0 + 0xd4) as *mut Clp3Ofs
    }

    #[doc="Get the *const pointer for the CLP3_OFS register."]
    #[inline] pub fn clp3_ofs_ptr(&self) -> *const Clp3Ofs { 
           self.clp3_ofs_mut()
    }

    #[doc="Read the CLP3_OFS register."]
    #[inline] pub fn clp3_ofs(&self) -> Clp3Ofs { 
        unsafe {
            read_volatile(self.clp3_ofs_ptr())
        }
    }

    #[doc="Write the CLP3_OFS register."]
    #[inline] pub fn set_clp3_ofs<F: FnOnce(Clp3Ofs) -> Clp3Ofs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clp3_ofs_mut(), f(Clp3Ofs(0)));
        }
        self
    }

    #[doc="Modify the CLP3_OFS register."]
    #[inline] pub fn with_clp3_ofs<F: FnOnce(Clp3Ofs) -> Clp3Ofs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clp3_ofs_mut(), f(self.clp3_ofs()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLP2_OFS register."]
    #[inline] pub fn clp2_ofs_mut(&self) -> *mut Clp2Ofs { 
        (self.0 + 0xd8) as *mut Clp2Ofs
    }

    #[doc="Get the *const pointer for the CLP2_OFS register."]
    #[inline] pub fn clp2_ofs_ptr(&self) -> *const Clp2Ofs { 
           self.clp2_ofs_mut()
    }

    #[doc="Read the CLP2_OFS register."]
    #[inline] pub fn clp2_ofs(&self) -> Clp2Ofs { 
        unsafe {
            read_volatile(self.clp2_ofs_ptr())
        }
    }

    #[doc="Write the CLP2_OFS register."]
    #[inline] pub fn set_clp2_ofs<F: FnOnce(Clp2Ofs) -> Clp2Ofs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clp2_ofs_mut(), f(Clp2Ofs(0)));
        }
        self
    }

    #[doc="Modify the CLP2_OFS register."]
    #[inline] pub fn with_clp2_ofs<F: FnOnce(Clp2Ofs) -> Clp2Ofs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clp2_ofs_mut(), f(self.clp2_ofs()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLP1_OFS register."]
    #[inline] pub fn clp1_ofs_mut(&self) -> *mut Clp1Ofs { 
        (self.0 + 0xdc) as *mut Clp1Ofs
    }

    #[doc="Get the *const pointer for the CLP1_OFS register."]
    #[inline] pub fn clp1_ofs_ptr(&self) -> *const Clp1Ofs { 
           self.clp1_ofs_mut()
    }

    #[doc="Read the CLP1_OFS register."]
    #[inline] pub fn clp1_ofs(&self) -> Clp1Ofs { 
        unsafe {
            read_volatile(self.clp1_ofs_ptr())
        }
    }

    #[doc="Write the CLP1_OFS register."]
    #[inline] pub fn set_clp1_ofs<F: FnOnce(Clp1Ofs) -> Clp1Ofs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clp1_ofs_mut(), f(Clp1Ofs(0)));
        }
        self
    }

    #[doc="Modify the CLP1_OFS register."]
    #[inline] pub fn with_clp1_ofs<F: FnOnce(Clp1Ofs) -> Clp1Ofs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clp1_ofs_mut(), f(self.clp1_ofs()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLP0_OFS register."]
    #[inline] pub fn clp0_ofs_mut(&self) -> *mut Clp0Ofs { 
        (self.0 + 0xe0) as *mut Clp0Ofs
    }

    #[doc="Get the *const pointer for the CLP0_OFS register."]
    #[inline] pub fn clp0_ofs_ptr(&self) -> *const Clp0Ofs { 
           self.clp0_ofs_mut()
    }

    #[doc="Read the CLP0_OFS register."]
    #[inline] pub fn clp0_ofs(&self) -> Clp0Ofs { 
        unsafe {
            read_volatile(self.clp0_ofs_ptr())
        }
    }

    #[doc="Write the CLP0_OFS register."]
    #[inline] pub fn set_clp0_ofs<F: FnOnce(Clp0Ofs) -> Clp0Ofs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clp0_ofs_mut(), f(Clp0Ofs(0)));
        }
        self
    }

    #[doc="Modify the CLP0_OFS register."]
    #[inline] pub fn with_clp0_ofs<F: FnOnce(Clp0Ofs) -> Clp0Ofs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clp0_ofs_mut(), f(self.clp0_ofs()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLPX_OFS register."]
    #[inline] pub fn clpx_ofs_mut(&self) -> *mut ClpxOfs { 
        (self.0 + 0xe4) as *mut ClpxOfs
    }

    #[doc="Get the *const pointer for the CLPX_OFS register."]
    #[inline] pub fn clpx_ofs_ptr(&self) -> *const ClpxOfs { 
           self.clpx_ofs_mut()
    }

    #[doc="Read the CLPX_OFS register."]
    #[inline] pub fn clpx_ofs(&self) -> ClpxOfs { 
        unsafe {
            read_volatile(self.clpx_ofs_ptr())
        }
    }

    #[doc="Write the CLPX_OFS register."]
    #[inline] pub fn set_clpx_ofs<F: FnOnce(ClpxOfs) -> ClpxOfs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clpx_ofs_mut(), f(ClpxOfs(0)));
        }
        self
    }

    #[doc="Modify the CLPX_OFS register."]
    #[inline] pub fn with_clpx_ofs<F: FnOnce(ClpxOfs) -> ClpxOfs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clpx_ofs_mut(), f(self.clpx_ofs()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLP9_OFS register."]
    #[inline] pub fn clp9_ofs_mut(&self) -> *mut Clp9Ofs { 
        (self.0 + 0xe8) as *mut Clp9Ofs
    }

    #[doc="Get the *const pointer for the CLP9_OFS register."]
    #[inline] pub fn clp9_ofs_ptr(&self) -> *const Clp9Ofs { 
           self.clp9_ofs_mut()
    }

    #[doc="Read the CLP9_OFS register."]
    #[inline] pub fn clp9_ofs(&self) -> Clp9Ofs { 
        unsafe {
            read_volatile(self.clp9_ofs_ptr())
        }
    }

    #[doc="Write the CLP9_OFS register."]
    #[inline] pub fn set_clp9_ofs<F: FnOnce(Clp9Ofs) -> Clp9Ofs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clp9_ofs_mut(), f(Clp9Ofs(0)));
        }
        self
    }

    #[doc="Modify the CLP9_OFS register."]
    #[inline] pub fn with_clp9_ofs<F: FnOnce(Clp9Ofs) -> Clp9Ofs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clp9_ofs_mut(), f(self.clp9_ofs()));
        }
        self
    }

}

#[doc="ADC Status and Control Register 1"]
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

    #[doc="Clear Latch Trigger in Trigger Handler Block"]
    #[inline] pub fn clrltrg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CLRLTRG != 0"]
    #[inline] pub fn test_clrltrg(&self) -> bool {
        self.clrltrg() != 0
    }

    #[doc="Sets the CLRLTRG field."]
    #[inline] pub fn set_clrltrg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
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
        if self.adiv() != 0 { try!(write!(f, " adiv=0x{:x}", self.adiv()))}
        if self.clrltrg() != 0 { try!(write!(f, " clrltrg"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Configuration Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfg2(pub u32);
impl Cfg2 {
    #[doc="Sample Time Select"]
    #[inline] pub fn smplts(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if SMPLTS != 0"]
    #[inline] pub fn test_smplts(&self) -> bool {
        self.smplts() != 0
    }

    #[doc="Sets the SMPLTS field."]
    #[inline] pub fn set_smplts<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
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
        if self.smplts() != 0 { try!(write!(f, " smplts=0x{:x}", self.smplts()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Data Result Registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R(pub u32);
impl R {
    #[doc="Data result"]
    #[inline] pub fn d(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if D != 0"]
    #[inline] pub fn test_d(&self) -> bool {
        self.d() != 0
    }

    #[doc="Sets the D field."]
    #[inline] pub fn set_d<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
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

    #[doc="Trigger Process Number"]
    #[inline] pub fn trgprnum(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
    }

    #[doc="Returns true if TRGPRNUM != 0"]
    #[inline] pub fn test_trgprnum(&self) -> bool {
        self.trgprnum() != 0
    }

    #[doc="Sets the TRGPRNUM field."]
    #[inline] pub fn set_trgprnum<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Trigger Status"]
    #[inline] pub fn trgstlat(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if TRGSTLAT != 0"]
    #[inline] pub fn test_trgstlat(&self) -> bool {
        self.trgstlat() != 0
    }

    #[doc="Sets the TRGSTLAT field."]
    #[inline] pub fn set_trgstlat<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Error in Multiplexed Trigger Request"]
    #[inline] pub fn trgsterr(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if TRGSTERR != 0"]
    #[inline] pub fn test_trgsterr(&self) -> bool {
        self.trgsterr() != 0
    }

    #[doc="Sets the TRGSTERR field."]
    #[inline] pub fn set_trgsterr<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
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
        if self.trgprnum() != 0 { try!(write!(f, " trgprnum=0x{:x}", self.trgprnum()))}
        if self.trgstlat() != 0 { try!(write!(f, " trgstlat=0x{:x}", self.trgstlat()))}
        if self.trgsterr() != 0 { try!(write!(f, " trgsterr=0x{:x}", self.trgsterr()))}
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
        if self.cal() != 0 { try!(write!(f, " cal"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BASE Offset Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct BaseOfs(pub u32);
impl BaseOfs {
    #[doc="Base Offset Error Correction Value"]
    #[inline] pub fn ba_ofs(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if BA_OFS != 0"]
    #[inline] pub fn test_ba_ofs(&self) -> bool {
        self.ba_ofs() != 0
    }

    #[doc="Sets the BA_OFS field."]
    #[inline] pub fn set_ba_ofs<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for BaseOfs {
    #[inline]
    fn from(other: u32) -> Self {
         BaseOfs(other)
    }
}

impl ::core::fmt::Display for BaseOfs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for BaseOfs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ba_ofs() != 0 { try!(write!(f, " ba_ofs=0x{:x}", self.ba_ofs()))}
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

#[doc="USER Offset Correction Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct UsrOfs(pub u32);
impl UsrOfs {
    #[doc="USER Offset Error Correction Value"]
    #[inline] pub fn usr_ofs(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if USR_OFS != 0"]
    #[inline] pub fn test_usr_ofs(&self) -> bool {
        self.usr_ofs() != 0
    }

    #[doc="Sets the USR_OFS field."]
    #[inline] pub fn set_usr_ofs<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for UsrOfs {
    #[inline]
    fn from(other: u32) -> Self {
         UsrOfs(other)
    }
}

impl ::core::fmt::Display for UsrOfs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for UsrOfs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.usr_ofs() != 0 { try!(write!(f, " usr_ofs=0x{:x}", self.usr_ofs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC X Offset Correction Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Xofs(pub u32);
impl Xofs {
    #[doc="X offset error correction value"]
    #[inline] pub fn xofs(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if XOFS != 0"]
    #[inline] pub fn test_xofs(&self) -> bool {
        self.xofs() != 0
    }

    #[doc="Sets the XOFS field."]
    #[inline] pub fn set_xofs<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Xofs {
    #[inline]
    fn from(other: u32) -> Self {
         Xofs(other)
    }
}

impl ::core::fmt::Display for Xofs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Xofs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xofs() != 0 { try!(write!(f, " xofs=0x{:x}", self.xofs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Y Offset Correction Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Yofs(pub u32);
impl Yofs {
    #[doc="Y offset error correction value"]
    #[inline] pub fn yofs(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if YOFS != 0"]
    #[inline] pub fn test_yofs(&self) -> bool {
        self.yofs() != 0
    }

    #[doc="Sets the YOFS field."]
    #[inline] pub fn set_yofs<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Yofs {
    #[inline]
    fn from(other: u32) -> Self {
         Yofs(other)
    }
}

impl ::core::fmt::Display for Yofs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Yofs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.yofs() != 0 { try!(write!(f, " yofs=0x{:x}", self.yofs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Gain Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct G(pub u32);
impl G {
    #[doc="Gain error adjustment factor for the overall conversion"]
    #[inline] pub fn g(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if G != 0"]
    #[inline] pub fn test_g(&self) -> bool {
        self.g() != 0
    }

    #[doc="Sets the G field."]
    #[inline] pub fn set_g<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for G {
    #[inline]
    fn from(other: u32) -> Self {
         G(other)
    }
}

impl ::core::fmt::Display for G {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for G {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.g() != 0 { try!(write!(f, " g=0x{:x}", self.g()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC User Gain Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ug(pub u32);
impl Ug {
    #[doc="User gain error correction value"]
    #[inline] pub fn ug(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if UG != 0"]
    #[inline] pub fn test_ug(&self) -> bool {
        self.ug() != 0
    }

    #[doc="Sets the UG field."]
    #[inline] pub fn set_ug<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ug {
    #[inline]
    fn from(other: u32) -> Self {
         Ug(other)
    }
}

impl ::core::fmt::Display for Ug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ug() != 0 { try!(write!(f, " ug=0x{:x}", self.ug()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC General Calibration Value Register S"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clps(pub u32);
impl Clps {
    #[doc="Calibration Value"]
    #[inline] pub fn clps(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if CLPS != 0"]
    #[inline] pub fn test_clps(&self) -> bool {
        self.clps() != 0
    }

    #[doc="Sets the CLPS field."]
    #[inline] pub fn set_clps<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
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

#[doc="ADC Plus-Side General Calibration Value Register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clp3(pub u32);
impl Clp3 {
    #[doc="Calibration Value"]
    #[inline] pub fn clp3(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if CLP3 != 0"]
    #[inline] pub fn test_clp3(&self) -> bool {
        self.clp3() != 0
    }

    #[doc="Sets the CLP3 field."]
    #[inline] pub fn set_clp3<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 0);
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

#[doc="ADC Plus-Side General Calibration Value Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clp2(pub u32);
impl Clp2 {
    #[doc="Calibration Value"]
    #[inline] pub fn clp2(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if CLP2 != 0"]
    #[inline] pub fn test_clp2(&self) -> bool {
        self.clp2() != 0
    }

    #[doc="Sets the CLP2 field."]
    #[inline] pub fn set_clp2<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 0);
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

#[doc="ADC Plus-Side General Calibration Value Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clp1(pub u32);
impl Clp1 {
    #[doc="Calibration Value"]
    #[inline] pub fn clp1(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if CLP1 != 0"]
    #[inline] pub fn test_clp1(&self) -> bool {
        self.clp1() != 0
    }

    #[doc="Sets the CLP1 field."]
    #[inline] pub fn set_clp1<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 0);
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

#[doc="ADC Plus-Side General Calibration Value Register 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clp0(pub u32);
impl Clp0 {
    #[doc="Calibration Value"]
    #[inline] pub fn clp0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CLP0 != 0"]
    #[inline] pub fn test_clp0(&self) -> bool {
        self.clp0() != 0
    }

    #[doc="Sets the CLP0 field."]
    #[inline] pub fn set_clp0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
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

#[doc="ADC Plus-Side General Calibration Value Register X"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clpx(pub u32);
impl Clpx {
    #[doc="Calibration Value"]
    #[inline] pub fn clpx(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if CLPX != 0"]
    #[inline] pub fn test_clpx(&self) -> bool {
        self.clpx() != 0
    }

    #[doc="Sets the CLPX field."]
    #[inline] pub fn set_clpx<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clpx {
    #[inline]
    fn from(other: u32) -> Self {
         Clpx(other)
    }
}

impl ::core::fmt::Display for Clpx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clpx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clpx() != 0 { try!(write!(f, " clpx=0x{:x}", self.clpx()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Plus-Side General Calibration Value Register 9"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clp9(pub u32);
impl Clp9 {
    #[doc="Calibration Value"]
    #[inline] pub fn clp9(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if CLP9 != 0"]
    #[inline] pub fn test_clp9(&self) -> bool {
        self.clp9() != 0
    }

    #[doc="Sets the CLP9 field."]
    #[inline] pub fn set_clp9<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clp9 {
    #[inline]
    fn from(other: u32) -> Self {
         Clp9(other)
    }
}

impl ::core::fmt::Display for Clp9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clp9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clp9() != 0 { try!(write!(f, " clp9=0x{:x}", self.clp9()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC General Calibration Offset Value Register S"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct ClpsOfs(pub u32);
impl ClpsOfs {
    #[doc="CLPS Offset"]
    #[inline] pub fn clps_ofs(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CLPS_OFS != 0"]
    #[inline] pub fn test_clps_ofs(&self) -> bool {
        self.clps_ofs() != 0
    }

    #[doc="Sets the CLPS_OFS field."]
    #[inline] pub fn set_clps_ofs<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for ClpsOfs {
    #[inline]
    fn from(other: u32) -> Self {
         ClpsOfs(other)
    }
}

impl ::core::fmt::Display for ClpsOfs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for ClpsOfs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clps_ofs() != 0 { try!(write!(f, " clps_ofs=0x{:x}", self.clps_ofs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Plus-Side General Calibration Offset Value Register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clp3Ofs(pub u32);
impl Clp3Ofs {
    #[doc="CLP3 Offset"]
    #[inline] pub fn clp3_ofs(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CLP3_OFS != 0"]
    #[inline] pub fn test_clp3_ofs(&self) -> bool {
        self.clp3_ofs() != 0
    }

    #[doc="Sets the CLP3_OFS field."]
    #[inline] pub fn set_clp3_ofs<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clp3Ofs {
    #[inline]
    fn from(other: u32) -> Self {
         Clp3Ofs(other)
    }
}

impl ::core::fmt::Display for Clp3Ofs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clp3Ofs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clp3_ofs() != 0 { try!(write!(f, " clp3_ofs=0x{:x}", self.clp3_ofs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Plus-Side General Calibration Offset Value Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clp2Ofs(pub u32);
impl Clp2Ofs {
    #[doc="CLP2 Offset"]
    #[inline] pub fn clp2_ofs(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CLP2_OFS != 0"]
    #[inline] pub fn test_clp2_ofs(&self) -> bool {
        self.clp2_ofs() != 0
    }

    #[doc="Sets the CLP2_OFS field."]
    #[inline] pub fn set_clp2_ofs<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clp2Ofs {
    #[inline]
    fn from(other: u32) -> Self {
         Clp2Ofs(other)
    }
}

impl ::core::fmt::Display for Clp2Ofs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clp2Ofs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clp2_ofs() != 0 { try!(write!(f, " clp2_ofs=0x{:x}", self.clp2_ofs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Plus-Side General Calibration Offset Value Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clp1Ofs(pub u32);
impl Clp1Ofs {
    #[doc="CLP1 Offset"]
    #[inline] pub fn clp1_ofs(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CLP1_OFS != 0"]
    #[inline] pub fn test_clp1_ofs(&self) -> bool {
        self.clp1_ofs() != 0
    }

    #[doc="Sets the CLP1_OFS field."]
    #[inline] pub fn set_clp1_ofs<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clp1Ofs {
    #[inline]
    fn from(other: u32) -> Self {
         Clp1Ofs(other)
    }
}

impl ::core::fmt::Display for Clp1Ofs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clp1Ofs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clp1_ofs() != 0 { try!(write!(f, " clp1_ofs=0x{:x}", self.clp1_ofs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Plus-Side General Calibration Offset Value Register 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clp0Ofs(pub u32);
impl Clp0Ofs {
    #[doc="CLP0 Offset"]
    #[inline] pub fn clp0_ofs(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CLP0_OFS != 0"]
    #[inline] pub fn test_clp0_ofs(&self) -> bool {
        self.clp0_ofs() != 0
    }

    #[doc="Sets the CLP0_OFS field."]
    #[inline] pub fn set_clp0_ofs<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clp0Ofs {
    #[inline]
    fn from(other: u32) -> Self {
         Clp0Ofs(other)
    }
}

impl ::core::fmt::Display for Clp0Ofs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clp0Ofs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clp0_ofs() != 0 { try!(write!(f, " clp0_ofs=0x{:x}", self.clp0_ofs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Plus-Side General Calibration Offset Value Register X"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct ClpxOfs(pub u32);
impl ClpxOfs {
    #[doc="CLPX Offset"]
    #[inline] pub fn clpx_ofs(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if CLPX_OFS != 0"]
    #[inline] pub fn test_clpx_ofs(&self) -> bool {
        self.clpx_ofs() != 0
    }

    #[doc="Sets the CLPX_OFS field."]
    #[inline] pub fn set_clpx_ofs<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for ClpxOfs {
    #[inline]
    fn from(other: u32) -> Self {
         ClpxOfs(other)
    }
}

impl ::core::fmt::Display for ClpxOfs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for ClpxOfs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clpx_ofs() != 0 { try!(write!(f, " clpx_ofs=0x{:x}", self.clpx_ofs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Plus-Side General Calibration Offset Value Register 9"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clp9Ofs(pub u32);
impl Clp9Ofs {
    #[doc="CLP9 Offset"]
    #[inline] pub fn clp9_ofs(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if CLP9_OFS != 0"]
    #[inline] pub fn test_clp9_ofs(&self) -> bool {
        self.clp9_ofs() != 0
    }

    #[doc="Sets the CLP9_OFS field."]
    #[inline] pub fn set_clp9_ofs<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clp9Ofs {
    #[inline]
    fn from(other: u32) -> Self {
         Clp9Ofs(other)
    }
}

impl ::core::fmt::Display for Clp9Ofs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clp9Ofs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clp9_ofs() != 0 { try!(write!(f, " clp9_ofs=0x{:x}", self.clp9_ofs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

pub struct AdcCh { pub periph: AdcPeriph, pub index: usize }
channel!(ADC0_CH0, Adc0Ch0, ADC0, Adc0, _ADC0_CH0, AdcCh, _ADC0, 0);
channel!(ADC0_CH1, Adc0Ch1, ADC0, Adc0, _ADC0_CH1, AdcCh, _ADC0, 1);
channel!(ADC0_CH2, Adc0Ch2, ADC0, Adc0, _ADC0_CH2, AdcCh, _ADC0, 2);
channel!(ADC0_CH3, Adc0Ch3, ADC0, Adc0, _ADC0_CH3, AdcCh, _ADC0, 3);
channel!(ADC0_CH4, Adc0Ch4, ADC0, Adc0, _ADC0_CH4, AdcCh, _ADC0, 4);
channel!(ADC0_CH5, Adc0Ch5, ADC0, Adc0, _ADC0_CH5, AdcCh, _ADC0, 5);
channel!(ADC0_CH6, Adc0Ch6, ADC0, Adc0, _ADC0_CH6, AdcCh, _ADC0, 6);
channel!(ADC0_CH7, Adc0Ch7, ADC0, Adc0, _ADC0_CH7, AdcCh, _ADC0, 7);
channel!(ADC0_CH8, Adc0Ch8, ADC0, Adc0, _ADC0_CH8, AdcCh, _ADC0, 8);
channel!(ADC0_CH9, Adc0Ch9, ADC0, Adc0, _ADC0_CH9, AdcCh, _ADC0, 9);
channel!(ADC0_CH10, Adc0Ch10, ADC0, Adc0, _ADC0_CH10, AdcCh, _ADC0, 10);
channel!(ADC0_CH11, Adc0Ch11, ADC0, Adc0, _ADC0_CH11, AdcCh, _ADC0, 11);
channel!(ADC0_CH12, Adc0Ch12, ADC0, Adc0, _ADC0_CH12, AdcCh, _ADC0, 12);
channel!(ADC0_CH13, Adc0Ch13, ADC0, Adc0, _ADC0_CH13, AdcCh, _ADC0, 13);
channel!(ADC0_CH14, Adc0Ch14, ADC0, Adc0, _ADC0_CH14, AdcCh, _ADC0, 14);
channel!(ADC0_CH15, Adc0Ch15, ADC0, Adc0, _ADC0_CH15, AdcCh, _ADC0, 15);
channel!(ADC0_IN0, Adc0In0, ADC0, Adc0, _ADC0_IN0, AdcCh, _ADC0, 21);
channel!(ADC0_TEMP, Adc0Temp, ADC0, Adc0, _ADC0_TEMP, AdcCh, _ADC0, 26);
channel!(ADC0_BANDGAP, Adc0Bandgap, ADC0, Adc0, _ADC0_BANDGAP, AdcCh, _ADC0, 27);
channel!(ADC0_REFSH, Adc0Refsh, ADC0, Adc0, _ADC0_REFSH, AdcCh, _ADC0, 29);
channel!(ADC0_REFSL, Adc0Refsl, ADC0, Adc0, _ADC0_REFSL, AdcCh, _ADC0, 30);
channel!(ADC1_CH0, Adc1Ch0, ADC1, Adc1, _ADC1_CH0, AdcCh, _ADC1, 0);
channel!(ADC1_CH1, Adc1Ch1, ADC1, Adc1, _ADC1_CH1, AdcCh, _ADC1, 1);
channel!(ADC1_CH2, Adc1Ch2, ADC1, Adc1, _ADC1_CH2, AdcCh, _ADC1, 2);
channel!(ADC1_CH3, Adc1Ch3, ADC1, Adc1, _ADC1_CH3, AdcCh, _ADC1, 3);
channel!(ADC1_CH4, Adc1Ch4, ADC1, Adc1, _ADC1_CH4, AdcCh, _ADC1, 4);
channel!(ADC1_CH5, Adc1Ch5, ADC1, Adc1, _ADC1_CH5, AdcCh, _ADC1, 5);
channel!(ADC1_CH6, Adc1Ch6, ADC1, Adc1, _ADC1_CH6, AdcCh, _ADC1, 6);
channel!(ADC1_CH7, Adc1Ch7, ADC1, Adc1, _ADC1_CH7, AdcCh, _ADC1, 7);
channel!(ADC1_CH8, Adc1Ch8, ADC1, Adc1, _ADC1_CH8, AdcCh, _ADC1, 8);
channel!(ADC1_CH9, Adc1Ch9, ADC1, Adc1, _ADC1_CH9, AdcCh, _ADC1, 9);
channel!(ADC1_CH10, Adc1Ch10, ADC1, Adc1, _ADC1_CH10, AdcCh, _ADC1, 10);
channel!(ADC1_CH11, Adc1Ch11, ADC1, Adc1, _ADC1_CH11, AdcCh, _ADC1, 11);
channel!(ADC1_CH12, Adc1Ch12, ADC1, Adc1, _ADC1_CH12, AdcCh, _ADC1, 12);
channel!(ADC1_CH13, Adc1Ch13, ADC1, Adc1, _ADC1_CH13, AdcCh, _ADC1, 13);
channel!(ADC1_CH14, Adc1Ch14, ADC1, Adc1, _ADC1_CH14, AdcCh, _ADC1, 14);
channel!(ADC1_CH15, Adc1Ch15, ADC1, Adc1, _ADC1_CH15, AdcCh, _ADC1, 15);
channel!(ADC1_IN0, Adc1In0, ADC1, Adc1, _ADC1_IN0, AdcCh, _ADC1, 21);
channel!(ADC1_BANDGAP, Adc1Bandgap, ADC1, Adc1, _ADC1_BANDGAP, AdcCh, _ADC1, 27);
channel!(ADC1_REFSH, Adc1Refsh, ADC1, Adc1, _ADC1_REFSH, AdcCh, _ADC1, 29);
channel!(ADC1_REFSL, Adc1Refsl, ADC1, Adc1, _ADC1_REFSL, AdcCh, _ADC1, 30);

pub trait IrqAdc<T> {
    fn irq_adc(&self) -> T;
}

impl IrqAdc<super::irq::IrqAdc0> for Adc0 {
    fn irq_adc(&self) -> super::irq::IrqAdc0 { super::irq::IRQ_ADC0 }
}

impl IrqAdc<super::irq::IrqAdc1> for Adc1 {
    fn irq_adc(&self) -> super::irq::IrqAdc1 { super::irq::IRQ_ADC1 }
}

