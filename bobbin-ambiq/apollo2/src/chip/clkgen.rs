//! Clock Generator
#[allow(unused_imports)] use bobbin_common::*;

periph!(CLKGEN, Clkgen, 0x40004000);

#[doc="Clock Generator"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clkgen(pub usize);
impl Clkgen {
    #[doc="Get the *mut pointer for the CALXT register."]
    #[inline] pub fn calxt_mut(&self) -> *mut Calxt { 
        (self.0 + 0x0) as *mut Calxt
    }

    #[doc="Get the *const pointer for the CALXT register."]
    #[inline] pub fn calxt_ptr(&self) -> *const Calxt { 
           self.calxt_mut()
    }

    #[doc="Read the CALXT register."]
    #[inline] pub fn calxt(&self) -> Calxt { 
        unsafe {
            read_volatile(self.calxt_ptr())
        }
    }

    #[doc="Write the CALXT register."]
    #[inline] pub fn set_calxt<F: FnOnce(Calxt) -> Calxt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.calxt_mut(), f(Calxt(0)));
        }
        self
    }

    #[doc="Modify the CALXT register."]
    #[inline] pub fn with_calxt<F: FnOnce(Calxt) -> Calxt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.calxt_mut(), f(self.calxt()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CALRC register."]
    #[inline] pub fn calrc_mut(&self) -> *mut Calrc { 
        (self.0 + 0x4) as *mut Calrc
    }

    #[doc="Get the *const pointer for the CALRC register."]
    #[inline] pub fn calrc_ptr(&self) -> *const Calrc { 
           self.calrc_mut()
    }

    #[doc="Read the CALRC register."]
    #[inline] pub fn calrc(&self) -> Calrc { 
        unsafe {
            read_volatile(self.calrc_ptr())
        }
    }

    #[doc="Write the CALRC register."]
    #[inline] pub fn set_calrc<F: FnOnce(Calrc) -> Calrc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.calrc_mut(), f(Calrc(0)));
        }
        self
    }

    #[doc="Modify the CALRC register."]
    #[inline] pub fn with_calrc<F: FnOnce(Calrc) -> Calrc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.calrc_mut(), f(self.calrc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ACALCTR register."]
    #[inline] pub fn acalctr_mut(&self) -> *mut Acalctr { 
        (self.0 + 0x8) as *mut Acalctr
    }

    #[doc="Get the *const pointer for the ACALCTR register."]
    #[inline] pub fn acalctr_ptr(&self) -> *const Acalctr { 
           self.acalctr_mut()
    }

    #[doc="Read the ACALCTR register."]
    #[inline] pub fn acalctr(&self) -> Acalctr { 
        unsafe {
            read_volatile(self.acalctr_ptr())
        }
    }

    #[doc="Write the ACALCTR register."]
    #[inline] pub fn set_acalctr<F: FnOnce(Acalctr) -> Acalctr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.acalctr_mut(), f(Acalctr(0)));
        }
        self
    }

    #[doc="Modify the ACALCTR register."]
    #[inline] pub fn with_acalctr<F: FnOnce(Acalctr) -> Acalctr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.acalctr_mut(), f(self.acalctr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the OCTRL register."]
    #[inline] pub fn octrl_mut(&self) -> *mut Octrl { 
        (self.0 + 0xc) as *mut Octrl
    }

    #[doc="Get the *const pointer for the OCTRL register."]
    #[inline] pub fn octrl_ptr(&self) -> *const Octrl { 
           self.octrl_mut()
    }

    #[doc="Read the OCTRL register."]
    #[inline] pub fn octrl(&self) -> Octrl { 
        unsafe {
            read_volatile(self.octrl_ptr())
        }
    }

    #[doc="Write the OCTRL register."]
    #[inline] pub fn set_octrl<F: FnOnce(Octrl) -> Octrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.octrl_mut(), f(Octrl(0)));
        }
        self
    }

    #[doc="Modify the OCTRL register."]
    #[inline] pub fn with_octrl<F: FnOnce(Octrl) -> Octrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.octrl_mut(), f(self.octrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLKOUT register."]
    #[inline] pub fn clkout_mut(&self) -> *mut Clkout { 
        (self.0 + 0x10) as *mut Clkout
    }

    #[doc="Get the *const pointer for the CLKOUT register."]
    #[inline] pub fn clkout_ptr(&self) -> *const Clkout { 
           self.clkout_mut()
    }

    #[doc="Read the CLKOUT register."]
    #[inline] pub fn clkout(&self) -> Clkout { 
        unsafe {
            read_volatile(self.clkout_ptr())
        }
    }

    #[doc="Write the CLKOUT register."]
    #[inline] pub fn set_clkout<F: FnOnce(Clkout) -> Clkout>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clkout_mut(), f(Clkout(0)));
        }
        self
    }

    #[doc="Modify the CLKOUT register."]
    #[inline] pub fn with_clkout<F: FnOnce(Clkout) -> Clkout>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clkout_mut(), f(self.clkout()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLKKEY register."]
    #[inline] pub fn clkkey_mut(&self) -> *mut Clkkey { 
        (self.0 + 0x14) as *mut Clkkey
    }

    #[doc="Get the *const pointer for the CLKKEY register."]
    #[inline] pub fn clkkey_ptr(&self) -> *const Clkkey { 
           self.clkkey_mut()
    }

    #[doc="Read the CLKKEY register."]
    #[inline] pub fn clkkey(&self) -> Clkkey { 
        unsafe {
            read_volatile(self.clkkey_ptr())
        }
    }

    #[doc="Write the CLKKEY register."]
    #[inline] pub fn set_clkkey<F: FnOnce(Clkkey) -> Clkkey>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clkkey_mut(), f(Clkkey(0)));
        }
        self
    }

    #[doc="Modify the CLKKEY register."]
    #[inline] pub fn with_clkkey<F: FnOnce(Clkkey) -> Clkkey>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clkkey_mut(), f(self.clkkey()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CCTRL register."]
    #[inline] pub fn cctrl_mut(&self) -> *mut Cctrl { 
        (self.0 + 0x18) as *mut Cctrl
    }

    #[doc="Get the *const pointer for the CCTRL register."]
    #[inline] pub fn cctrl_ptr(&self) -> *const Cctrl { 
           self.cctrl_mut()
    }

    #[doc="Read the CCTRL register."]
    #[inline] pub fn cctrl(&self) -> Cctrl { 
        unsafe {
            read_volatile(self.cctrl_ptr())
        }
    }

    #[doc="Write the CCTRL register."]
    #[inline] pub fn set_cctrl<F: FnOnce(Cctrl) -> Cctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cctrl_mut(), f(Cctrl(0)));
        }
        self
    }

    #[doc="Modify the CCTRL register."]
    #[inline] pub fn with_cctrl<F: FnOnce(Cctrl) -> Cctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cctrl_mut(), f(self.cctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the STATUS register."]
    #[inline] pub fn status_mut(&self) -> *mut Status { 
        (self.0 + 0x1c) as *mut Status
    }

    #[doc="Get the *const pointer for the STATUS register."]
    #[inline] pub fn status_ptr(&self) -> *const Status { 
           self.status_mut()
    }

    #[doc="Read the STATUS register."]
    #[inline] pub fn status(&self) -> Status { 
        unsafe {
            read_volatile(self.status_ptr())
        }
    }

    #[doc="Write the STATUS register."]
    #[inline] pub fn set_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.status_mut(), f(Status(0)));
        }
        self
    }

    #[doc="Modify the STATUS register."]
    #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.status_mut(), f(self.status()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HFADJ register."]
    #[inline] pub fn hfadj_mut(&self) -> *mut Hfadj { 
        (self.0 + 0x20) as *mut Hfadj
    }

    #[doc="Get the *const pointer for the HFADJ register."]
    #[inline] pub fn hfadj_ptr(&self) -> *const Hfadj { 
           self.hfadj_mut()
    }

    #[doc="Read the HFADJ register."]
    #[inline] pub fn hfadj(&self) -> Hfadj { 
        unsafe {
            read_volatile(self.hfadj_ptr())
        }
    }

    #[doc="Write the HFADJ register."]
    #[inline] pub fn set_hfadj<F: FnOnce(Hfadj) -> Hfadj>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfadj_mut(), f(Hfadj(0)));
        }
        self
    }

    #[doc="Modify the HFADJ register."]
    #[inline] pub fn with_hfadj<F: FnOnce(Hfadj) -> Hfadj>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfadj_mut(), f(self.hfadj()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HFVAL register."]
    #[inline] pub fn hfval_mut(&self) -> *mut Hfval { 
        (self.0 + 0x24) as *mut Hfval
    }

    #[doc="Get the *const pointer for the HFVAL register."]
    #[inline] pub fn hfval_ptr(&self) -> *const Hfval { 
           self.hfval_mut()
    }

    #[doc="Read the HFVAL register."]
    #[inline] pub fn hfval(&self) -> Hfval { 
        unsafe {
            read_volatile(self.hfval_ptr())
        }
    }

    #[doc="Write the HFVAL register."]
    #[inline] pub fn set_hfval<F: FnOnce(Hfval) -> Hfval>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfval_mut(), f(Hfval(0)));
        }
        self
    }

    #[doc="Modify the HFVAL register."]
    #[inline] pub fn with_hfval<F: FnOnce(Hfval) -> Hfval>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfval_mut(), f(self.hfval()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLOCKEN register."]
    #[inline] pub fn clocken_mut(&self) -> *mut Clocken { 
        (self.0 + 0x28) as *mut Clocken
    }

    #[doc="Get the *const pointer for the CLOCKEN register."]
    #[inline] pub fn clocken_ptr(&self) -> *const Clocken { 
           self.clocken_mut()
    }

    #[doc="Read the CLOCKEN register."]
    #[inline] pub fn clocken(&self) -> Clocken { 
        unsafe {
            read_volatile(self.clocken_ptr())
        }
    }

    #[doc="Write the CLOCKEN register."]
    #[inline] pub fn set_clocken<F: FnOnce(Clocken) -> Clocken>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clocken_mut(), f(Clocken(0)));
        }
        self
    }

    #[doc="Modify the CLOCKEN register."]
    #[inline] pub fn with_clocken<F: FnOnce(Clocken) -> Clocken>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clocken_mut(), f(self.clocken()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLOCKEN2 register."]
    #[inline] pub fn clocken2_mut(&self) -> *mut Clocken2 { 
        (self.0 + 0x2c) as *mut Clocken2
    }

    #[doc="Get the *const pointer for the CLOCKEN2 register."]
    #[inline] pub fn clocken2_ptr(&self) -> *const Clocken2 { 
           self.clocken2_mut()
    }

    #[doc="Read the CLOCKEN2 register."]
    #[inline] pub fn clocken2(&self) -> Clocken2 { 
        unsafe {
            read_volatile(self.clocken2_ptr())
        }
    }

    #[doc="Write the CLOCKEN2 register."]
    #[inline] pub fn set_clocken2<F: FnOnce(Clocken2) -> Clocken2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clocken2_mut(), f(Clocken2(0)));
        }
        self
    }

    #[doc="Modify the CLOCKEN2 register."]
    #[inline] pub fn with_clocken2<F: FnOnce(Clocken2) -> Clocken2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clocken2_mut(), f(self.clocken2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLOCKEN3 register."]
    #[inline] pub fn clocken3_mut(&self) -> *mut Clocken3 { 
        (self.0 + 0x30) as *mut Clocken3
    }

    #[doc="Get the *const pointer for the CLOCKEN3 register."]
    #[inline] pub fn clocken3_ptr(&self) -> *const Clocken3 { 
           self.clocken3_mut()
    }

    #[doc="Read the CLOCKEN3 register."]
    #[inline] pub fn clocken3(&self) -> Clocken3 { 
        unsafe {
            read_volatile(self.clocken3_ptr())
        }
    }

    #[doc="Write the CLOCKEN3 register."]
    #[inline] pub fn set_clocken3<F: FnOnce(Clocken3) -> Clocken3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clocken3_mut(), f(Clocken3(0)));
        }
        self
    }

    #[doc="Modify the CLOCKEN3 register."]
    #[inline] pub fn with_clocken3<F: FnOnce(Clocken3) -> Clocken3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clocken3_mut(), f(self.clocken3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the UARTEN register."]
    #[inline] pub fn uarten_mut(&self) -> *mut Uarten { 
        (self.0 + 0x34) as *mut Uarten
    }

    #[doc="Get the *const pointer for the UARTEN register."]
    #[inline] pub fn uarten_ptr(&self) -> *const Uarten { 
           self.uarten_mut()
    }

    #[doc="Read the UARTEN register."]
    #[inline] pub fn uarten(&self) -> Uarten { 
        unsafe {
            read_volatile(self.uarten_ptr())
        }
    }

    #[doc="Write the UARTEN register."]
    #[inline] pub fn set_uarten<F: FnOnce(Uarten) -> Uarten>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.uarten_mut(), f(Uarten(0)));
        }
        self
    }

    #[doc="Modify the UARTEN register."]
    #[inline] pub fn with_uarten<F: FnOnce(Uarten) -> Uarten>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.uarten_mut(), f(self.uarten()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTRLOW register."]
    #[inline] pub fn ctrlow_mut(&self) -> *mut Ctrlow { 
        (self.0 + 0x40) as *mut Ctrlow
    }

    #[doc="Get the *const pointer for the CTRLOW register."]
    #[inline] pub fn ctrlow_ptr(&self) -> *const Ctrlow { 
           self.ctrlow_mut()
    }

    #[doc="Read the CTRLOW register."]
    #[inline] pub fn ctrlow(&self) -> Ctrlow { 
        unsafe {
            read_volatile(self.ctrlow_ptr())
        }
    }

    #[doc="Write the CTRLOW register."]
    #[inline] pub fn set_ctrlow<F: FnOnce(Ctrlow) -> Ctrlow>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlow_mut(), f(Ctrlow(0)));
        }
        self
    }

    #[doc="Modify the CTRLOW register."]
    #[inline] pub fn with_ctrlow<F: FnOnce(Ctrlow) -> Ctrlow>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlow_mut(), f(self.ctrlow()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTRUP register."]
    #[inline] pub fn ctrup_mut(&self) -> *mut Ctrup { 
        (self.0 + 0x44) as *mut Ctrup
    }

    #[doc="Get the *const pointer for the CTRUP register."]
    #[inline] pub fn ctrup_ptr(&self) -> *const Ctrup { 
           self.ctrup_mut()
    }

    #[doc="Read the CTRUP register."]
    #[inline] pub fn ctrup(&self) -> Ctrup { 
        unsafe {
            read_volatile(self.ctrup_ptr())
        }
    }

    #[doc="Write the CTRUP register."]
    #[inline] pub fn set_ctrup<F: FnOnce(Ctrup) -> Ctrup>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrup_mut(), f(Ctrup(0)));
        }
        self
    }

    #[doc="Modify the CTRUP register."]
    #[inline] pub fn with_ctrup<F: FnOnce(Ctrup) -> Ctrup>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrup_mut(), f(self.ctrup()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALMLOW register."]
    #[inline] pub fn almlow_mut(&self) -> *mut Almlow { 
        (self.0 + 0x48) as *mut Almlow
    }

    #[doc="Get the *const pointer for the ALMLOW register."]
    #[inline] pub fn almlow_ptr(&self) -> *const Almlow { 
           self.almlow_mut()
    }

    #[doc="Read the ALMLOW register."]
    #[inline] pub fn almlow(&self) -> Almlow { 
        unsafe {
            read_volatile(self.almlow_ptr())
        }
    }

    #[doc="Write the ALMLOW register."]
    #[inline] pub fn set_almlow<F: FnOnce(Almlow) -> Almlow>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.almlow_mut(), f(Almlow(0)));
        }
        self
    }

    #[doc="Modify the ALMLOW register."]
    #[inline] pub fn with_almlow<F: FnOnce(Almlow) -> Almlow>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.almlow_mut(), f(self.almlow()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALMUP register."]
    #[inline] pub fn almup_mut(&self) -> *mut Almup { 
        (self.0 + 0x4c) as *mut Almup
    }

    #[doc="Get the *const pointer for the ALMUP register."]
    #[inline] pub fn almup_ptr(&self) -> *const Almup { 
           self.almup_mut()
    }

    #[doc="Read the ALMUP register."]
    #[inline] pub fn almup(&self) -> Almup { 
        unsafe {
            read_volatile(self.almup_ptr())
        }
    }

    #[doc="Write the ALMUP register."]
    #[inline] pub fn set_almup<F: FnOnce(Almup) -> Almup>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.almup_mut(), f(Almup(0)));
        }
        self
    }

    #[doc="Modify the ALMUP register."]
    #[inline] pub fn with_almup<F: FnOnce(Almup) -> Almup>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.almup_mut(), f(self.almup()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RTCCTL register."]
    #[inline] pub fn rtcctl_mut(&self) -> *mut Rtcctl { 
        (self.0 + 0x50) as *mut Rtcctl
    }

    #[doc="Get the *const pointer for the RTCCTL register."]
    #[inline] pub fn rtcctl_ptr(&self) -> *const Rtcctl { 
           self.rtcctl_mut()
    }

    #[doc="Read the RTCCTL register."]
    #[inline] pub fn rtcctl(&self) -> Rtcctl { 
        unsafe {
            read_volatile(self.rtcctl_ptr())
        }
    }

    #[doc="Write the RTCCTL register."]
    #[inline] pub fn set_rtcctl<F: FnOnce(Rtcctl) -> Rtcctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rtcctl_mut(), f(Rtcctl(0)));
        }
        self
    }

    #[doc="Modify the RTCCTL register."]
    #[inline] pub fn with_rtcctl<F: FnOnce(Rtcctl) -> Rtcctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rtcctl_mut(), f(self.rtcctl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTEN register."]
    #[inline] pub fn inten_mut(&self) -> *mut Inten { 
        (self.0 + 0x100) as *mut Inten
    }

    #[doc="Get the *const pointer for the INTEN register."]
    #[inline] pub fn inten_ptr(&self) -> *const Inten { 
           self.inten_mut()
    }

    #[doc="Read the INTEN register."]
    #[inline] pub fn inten(&self) -> Inten { 
        unsafe {
            read_volatile(self.inten_ptr())
        }
    }

    #[doc="Write the INTEN register."]
    #[inline] pub fn set_inten<F: FnOnce(Inten) -> Inten>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.inten_mut(), f(Inten(0)));
        }
        self
    }

    #[doc="Modify the INTEN register."]
    #[inline] pub fn with_inten<F: FnOnce(Inten) -> Inten>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.inten_mut(), f(self.inten()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTSTAT register."]
    #[inline] pub fn intstat_mut(&self) -> *mut Intstat { 
        (self.0 + 0x104) as *mut Intstat
    }

    #[doc="Get the *const pointer for the INTSTAT register."]
    #[inline] pub fn intstat_ptr(&self) -> *const Intstat { 
           self.intstat_mut()
    }

    #[doc="Read the INTSTAT register."]
    #[inline] pub fn intstat(&self) -> Intstat { 
        unsafe {
            read_volatile(self.intstat_ptr())
        }
    }

    #[doc="Write the INTSTAT register."]
    #[inline] pub fn set_intstat<F: FnOnce(Intstat) -> Intstat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intstat_mut(), f(Intstat(0)));
        }
        self
    }

    #[doc="Modify the INTSTAT register."]
    #[inline] pub fn with_intstat<F: FnOnce(Intstat) -> Intstat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intstat_mut(), f(self.intstat()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTCLR register."]
    #[inline] pub fn intclr_mut(&self) -> *mut Intclr { 
        (self.0 + 0x108) as *mut Intclr
    }

    #[doc="Get the *const pointer for the INTCLR register."]
    #[inline] pub fn intclr_ptr(&self) -> *const Intclr { 
           self.intclr_mut()
    }

    #[doc="Read the INTCLR register."]
    #[inline] pub fn intclr(&self) -> Intclr { 
        unsafe {
            read_volatile(self.intclr_ptr())
        }
    }

    #[doc="Write the INTCLR register."]
    #[inline] pub fn set_intclr<F: FnOnce(Intclr) -> Intclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intclr_mut(), f(Intclr(0)));
        }
        self
    }

    #[doc="Modify the INTCLR register."]
    #[inline] pub fn with_intclr<F: FnOnce(Intclr) -> Intclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intclr_mut(), f(self.intclr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTSET register."]
    #[inline] pub fn intset_mut(&self) -> *mut Intset { 
        (self.0 + 0x10c) as *mut Intset
    }

    #[doc="Get the *const pointer for the INTSET register."]
    #[inline] pub fn intset_ptr(&self) -> *const Intset { 
           self.intset_mut()
    }

    #[doc="Read the INTSET register."]
    #[inline] pub fn intset(&self) -> Intset { 
        unsafe {
            read_volatile(self.intset_ptr())
        }
    }

    #[doc="Write the INTSET register."]
    #[inline] pub fn set_intset<F: FnOnce(Intset) -> Intset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intset_mut(), f(Intset(0)));
        }
        self
    }

    #[doc="Modify the INTSET register."]
    #[inline] pub fn with_intset<F: FnOnce(Intset) -> Intset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intset_mut(), f(self.intset()));
        }
        self
    }

}

#[doc="XT Oscillator Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Calxt(pub u32);
impl Calxt {
    #[doc="XT Oscillator calibration value"]
    #[inline] pub fn calxt(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if CALXT != 0"]
    #[inline] pub fn test_calxt(&self) -> bool {
        self.calxt() != 0
    }

    #[doc="Sets the CALXT field."]
    #[inline] pub fn set_calxt<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Calxt {
    #[inline]
    fn from(other: u32) -> Self {
         Calxt(other)
    }
}

impl ::core::fmt::Display for Calxt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Calxt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.calxt() != 0 { try!(write!(f, " calxt=0x{:x}", self.calxt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RC Oscillator Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Calrc(pub u32);
impl Calrc {
    #[doc="LFRC Oscillator calibration value"]
    #[inline] pub fn calrc(&self) -> bits::U18 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ffff) as u32) } // [17:0]
    }

    #[doc="Returns true if CALRC != 0"]
    #[inline] pub fn test_calrc(&self) -> bool {
        self.calrc() != 0
    }

    #[doc="Sets the CALRC field."]
    #[inline] pub fn set_calrc<V: Into<bits::U18>>(mut self, value: V) -> Self {
        let value: bits::U18 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Calrc {
    #[inline]
    fn from(other: u32) -> Self {
         Calrc(other)
    }
}

impl ::core::fmt::Display for Calrc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Calrc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.calrc() != 0 { try!(write!(f, " calrc=0x{:x}", self.calrc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Autocalibration Counter"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Acalctr(pub u32);
impl Acalctr {
    #[doc="Autocalibration Counter result."]
    #[inline] pub fn acalctr(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
    }

    #[doc="Returns true if ACALCTR != 0"]
    #[inline] pub fn test_acalctr(&self) -> bool {
        self.acalctr() != 0
    }

    #[doc="Sets the ACALCTR field."]
    #[inline] pub fn set_acalctr<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Acalctr {
    #[inline]
    fn from(other: u32) -> Self {
         Acalctr(other)
    }
}

impl ::core::fmt::Display for Acalctr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Acalctr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.acalctr() != 0 { try!(write!(f, " acalctr=0x{:x}", self.acalctr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Oscillator Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Octrl(pub u32);
impl Octrl {
    #[doc="Autocalibration control"]
    #[inline] pub fn acal(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if ACAL != 0"]
    #[inline] pub fn test_acal(&self) -> bool {
        self.acal() != 0
    }

    #[doc="Sets the ACAL field."]
    #[inline] pub fn set_acal<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Selects the RTC oscillator (1 => LFRC, 0 => XT)"]
    #[inline] pub fn osel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if OSEL != 0"]
    #[inline] pub fn test_osel(&self) -> bool {
        self.osel() != 0
    }

    #[doc="Sets the OSEL field."]
    #[inline] pub fn set_osel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Oscillator switch on failure function"]
    #[inline] pub fn fos(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FOS != 0"]
    #[inline] pub fn test_fos(&self) -> bool {
        self.fos() != 0
    }

    #[doc="Sets the FOS field."]
    #[inline] pub fn set_fos<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Stop the LFRC Oscillator to the RTC"]
    #[inline] pub fn stoprc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if STOPRC != 0"]
    #[inline] pub fn test_stoprc(&self) -> bool {
        self.stoprc() != 0
    }

    #[doc="Sets the STOPRC field."]
    #[inline] pub fn set_stoprc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Stop the XT Oscillator to the RTC"]
    #[inline] pub fn stopxt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if STOPXT != 0"]
    #[inline] pub fn test_stopxt(&self) -> bool {
        self.stopxt() != 0
    }

    #[doc="Sets the STOPXT field."]
    #[inline] pub fn set_stopxt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Octrl {
    #[inline]
    fn from(other: u32) -> Self {
         Octrl(other)
    }
}

impl ::core::fmt::Display for Octrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Octrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.acal() != 0 { try!(write!(f, " acal=0x{:x}", self.acal()))}
        if self.osel() != 0 { try!(write!(f, " osel"))}
        if self.fos() != 0 { try!(write!(f, " fos"))}
        if self.stoprc() != 0 { try!(write!(f, " stoprc"))}
        if self.stopxt() != 0 { try!(write!(f, " stopxt"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CLKOUT Frequency Select"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clkout(pub u32);
impl Clkout {
    #[doc="Enable the CLKOUT signal"]
    #[inline] pub fn cken(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CKEN != 0"]
    #[inline] pub fn test_cken(&self) -> bool {
        self.cken() != 0
    }

    #[doc="Sets the CKEN field."]
    #[inline] pub fn set_cken<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="CLKOUT signal select"]
    #[inline] pub fn cksel(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if CKSEL != 0"]
    #[inline] pub fn test_cksel(&self) -> bool {
        self.cksel() != 0
    }

    #[doc="Sets the CKSEL field."]
    #[inline] pub fn set_cksel<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clkout {
    #[inline]
    fn from(other: u32) -> Self {
         Clkout(other)
    }
}

impl ::core::fmt::Display for Clkout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clkout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cken() != 0 { try!(write!(f, " cken"))}
        if self.cksel() != 0 { try!(write!(f, " cksel=0x{:x}", self.cksel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Key Register for Clock Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clkkey(pub u32);
impl Clkkey {
    #[doc="Key register value."]
    #[inline] pub fn clkkey(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CLKKEY != 0"]
    #[inline] pub fn test_clkkey(&self) -> bool {
        self.clkkey() != 0
    }

    #[doc="Sets the CLKKEY field."]
    #[inline] pub fn set_clkkey<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clkkey {
    #[inline]
    fn from(other: u32) -> Self {
         Clkkey(other)
    }
}

impl ::core::fmt::Display for Clkkey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clkkey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HFRC Clock Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cctrl(pub u32);
impl Cctrl {
    #[doc="Core Clock divisor"]
    #[inline] pub fn coresel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CORESEL != 0"]
    #[inline] pub fn test_coresel(&self) -> bool {
        self.coresel() != 0
    }

    #[doc="Sets the CORESEL field."]
    #[inline] pub fn set_coresel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Cctrl(other)
    }
}

impl ::core::fmt::Display for Cctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.coresel() != 0 { try!(write!(f, " coresel"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock Generator Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u32);
impl Status {
    #[doc="XT Oscillator is enabled but not oscillating"]
    #[inline] pub fn oscf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OSCF != 0"]
    #[inline] pub fn test_oscf(&self) -> bool {
        self.oscf() != 0
    }

    #[doc="Sets the OSCF field."]
    #[inline] pub fn set_oscf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Current RTC oscillator (1 => LFRC, 0 => XT)"]
    #[inline] pub fn omode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OMODE != 0"]
    #[inline] pub fn test_omode(&self) -> bool {
        self.omode() != 0
    }

    #[doc="Sets the OMODE field."]
    #[inline] pub fn set_omode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Status {
    #[inline]
    fn from(other: u32) -> Self {
         Status(other)
    }
}

impl ::core::fmt::Display for Status {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Status {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.oscf() != 0 { try!(write!(f, " oscf"))}
        if self.omode() != 0 { try!(write!(f, " omode"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HFRC Adjustment"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hfadj(pub u32);
impl Hfadj {
    #[doc="Gain control for HFRC adjustment"]
    #[inline] pub fn hfadj_gain(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x7) as u8) } // [23:21]
    }

    #[doc="Returns true if HFADJ_GAIN != 0"]
    #[inline] pub fn test_hfadj_gain(&self) -> bool {
        self.hfadj_gain() != 0
    }

    #[doc="Sets the HFADJ_GAIN field."]
    #[inline] pub fn set_hfadj_gain<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="XT warmup period for HFRC adjustment"]
    #[inline] pub fn hfwarmup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if HFWARMUP != 0"]
    #[inline] pub fn test_hfwarmup(&self) -> bool {
        self.hfwarmup() != 0
    }

    #[doc="Sets the HFWARMUP field."]
    #[inline] pub fn set_hfwarmup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Target HFRC adjustment value."]
    #[inline] pub fn hfxtadj(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xfff) as u16) } // [19:8]
    }

    #[doc="Returns true if HFXTADJ != 0"]
    #[inline] pub fn test_hfxtadj(&self) -> bool {
        self.hfxtadj() != 0
    }

    #[doc="Sets the HFXTADJ field."]
    #[inline] pub fn set_hfxtadj<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Repeat period for HFRC adjustment"]
    #[inline] pub fn hfadjck(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7) as u8) } // [3:1]
    }

    #[doc="Returns true if HFADJCK != 0"]
    #[inline] pub fn test_hfadjck(&self) -> bool {
        self.hfadjck() != 0
    }

    #[doc="Sets the HFADJCK field."]
    #[inline] pub fn set_hfadjck<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="HFRC adjustment control"]
    #[inline] pub fn hfadjen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HFADJEN != 0"]
    #[inline] pub fn test_hfadjen(&self) -> bool {
        self.hfadjen() != 0
    }

    #[doc="Sets the HFADJEN field."]
    #[inline] pub fn set_hfadjen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Hfadj {
    #[inline]
    fn from(other: u32) -> Self {
         Hfadj(other)
    }
}

impl ::core::fmt::Display for Hfadj {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hfadj {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hfadj_gain() != 0 { try!(write!(f, " hfadj_gain=0x{:x}", self.hfadj_gain()))}
        if self.hfwarmup() != 0 { try!(write!(f, " hfwarmup"))}
        if self.hfxtadj() != 0 { try!(write!(f, " hfxtadj=0x{:x}", self.hfxtadj()))}
        if self.hfadjck() != 0 { try!(write!(f, " hfadjck=0x{:x}", self.hfadjck()))}
        if self.hfadjen() != 0 { try!(write!(f, " hfadjen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HFADJ readback"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hfval(pub u32);
impl Hfval {
    #[doc="Current HFTUNE value"]
    #[inline] pub fn hftunerb(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if HFTUNERB != 0"]
    #[inline] pub fn test_hftunerb(&self) -> bool {
        self.hftunerb() != 0
    }

    #[doc="Sets the HFTUNERB field."]
    #[inline] pub fn set_hftunerb<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Hfval {
    #[inline]
    fn from(other: u32) -> Self {
         Hfval(other)
    }
}

impl ::core::fmt::Display for Hfval {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hfval {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hftunerb() != 0 { try!(write!(f, " hftunerb=0x{:x}", self.hftunerb()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock Enable Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clocken(pub u32);
impl Clocken {
    #[doc="Clock enable status"]
    #[inline] pub fn clocken(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CLOCKEN != 0"]
    #[inline] pub fn test_clocken(&self) -> bool {
        self.clocken() != 0
    }

    #[doc="Sets the CLOCKEN field."]
    #[inline] pub fn set_clocken<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clocken {
    #[inline]
    fn from(other: u32) -> Self {
         Clocken(other)
    }
}

impl ::core::fmt::Display for Clocken {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clocken {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock Enable Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clocken2(pub u32);
impl Clocken2 {
    #[doc="Clock enable status 2"]
    #[inline] pub fn clocken2(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CLOCKEN2 != 0"]
    #[inline] pub fn test_clocken2(&self) -> bool {
        self.clocken2() != 0
    }

    #[doc="Sets the CLOCKEN2 field."]
    #[inline] pub fn set_clocken2<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clocken2 {
    #[inline]
    fn from(other: u32) -> Self {
         Clocken2(other)
    }
}

impl ::core::fmt::Display for Clocken2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clocken2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock Enable Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clocken3(pub u32);
impl Clocken3 {
    #[doc="Clock enable status 3"]
    #[inline] pub fn clocken3(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CLOCKEN3 != 0"]
    #[inline] pub fn test_clocken3(&self) -> bool {
        self.clocken3() != 0
    }

    #[doc="Sets the CLOCKEN3 field."]
    #[inline] pub fn set_clocken3<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clocken3 {
    #[inline]
    fn from(other: u32) -> Self {
         Clocken3(other)
    }
}

impl ::core::fmt::Display for Clocken3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clocken3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Uarten(pub u32);
impl Uarten {
    #[doc="UART1 system clock control"]
    #[inline] pub fn uart1en(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if UART1EN != 0"]
    #[inline] pub fn test_uart1en(&self) -> bool {
        self.uart1en() != 0
    }

    #[doc="Sets the UART1EN field."]
    #[inline] pub fn set_uart1en<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="UART0 system clock control"]
    #[inline] pub fn uart0en(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if UART0EN != 0"]
    #[inline] pub fn test_uart0en(&self) -> bool {
        self.uart0en() != 0
    }

    #[doc="Sets the UART0EN field."]
    #[inline] pub fn set_uart0en<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Uarten {
    #[inline]
    fn from(other: u32) -> Self {
         Uarten(other)
    }
}

impl ::core::fmt::Display for Uarten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Uarten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.uart1en() != 0 { try!(write!(f, " uart1en=0x{:x}", self.uart1en()))}
        if self.uart0en() != 0 { try!(write!(f, " uart0en=0x{:x}", self.uart0en()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RTC Counters Lower"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlow(pub u32);
impl Ctrlow {
    #[doc="Hours Counter"]
    #[inline] pub fn ctrhr(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3f) as u8) } // [29:24]
    }

    #[doc="Returns true if CTRHR != 0"]
    #[inline] pub fn test_ctrhr(&self) -> bool {
        self.ctrhr() != 0
    }

    #[doc="Sets the CTRHR field."]
    #[inline] pub fn set_ctrhr<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Minutes Counter"]
    #[inline] pub fn ctrmin(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7f) as u8) } // [22:16]
    }

    #[doc="Returns true if CTRMIN != 0"]
    #[inline] pub fn test_ctrmin(&self) -> bool {
        self.ctrmin() != 0
    }

    #[doc="Sets the CTRMIN field."]
    #[inline] pub fn set_ctrmin<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Seconds Counter"]
    #[inline] pub fn ctrsec(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7f) as u8) } // [14:8]
    }

    #[doc="Returns true if CTRSEC != 0"]
    #[inline] pub fn test_ctrsec(&self) -> bool {
        self.ctrsec() != 0
    }

    #[doc="Sets the CTRSEC field."]
    #[inline] pub fn set_ctrsec<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="100ths of a second Counter"]
    #[inline] pub fn ctr100(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CTR100 != 0"]
    #[inline] pub fn test_ctr100(&self) -> bool {
        self.ctr100() != 0
    }

    #[doc="Sets the CTR100 field."]
    #[inline] pub fn set_ctr100<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ctrlow {
    #[inline]
    fn from(other: u32) -> Self {
         Ctrlow(other)
    }
}

impl ::core::fmt::Display for Ctrlow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrlow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ctrhr() != 0 { try!(write!(f, " ctrhr=0x{:x}", self.ctrhr()))}
        if self.ctrmin() != 0 { try!(write!(f, " ctrmin=0x{:x}", self.ctrmin()))}
        if self.ctrsec() != 0 { try!(write!(f, " ctrsec=0x{:x}", self.ctrsec()))}
        if self.ctr100() != 0 { try!(write!(f, " ctr100=0x{:x}", self.ctr100()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RTC Counters Upper"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrup(pub u32);
impl Ctrup {
    #[doc="Counter read error status"]
    #[inline] pub fn cterr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CTERR != 0"]
    #[inline] pub fn test_cterr(&self) -> bool {
        self.cterr() != 0
    }

    #[doc="Sets the CTERR field."]
    #[inline] pub fn set_cterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Century enable"]
    #[inline] pub fn ceb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if CEB != 0"]
    #[inline] pub fn test_ceb(&self) -> bool {
        self.ceb() != 0
    }

    #[doc="Sets the CEB field."]
    #[inline] pub fn set_ceb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Century"]
    #[inline] pub fn cb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if CB != 0"]
    #[inline] pub fn test_cb(&self) -> bool {
        self.cb() != 0
    }

    #[doc="Sets the CB field."]
    #[inline] pub fn set_cb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Weekdays Counter"]
    #[inline] pub fn ctrwkdy(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if CTRWKDY != 0"]
    #[inline] pub fn test_ctrwkdy(&self) -> bool {
        self.ctrwkdy() != 0
    }

    #[doc="Sets the CTRWKDY field."]
    #[inline] pub fn set_ctrwkdy<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Years Counter"]
    #[inline] pub fn ctryr(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if CTRYR != 0"]
    #[inline] pub fn test_ctryr(&self) -> bool {
        self.ctryr() != 0
    }

    #[doc="Sets the CTRYR field."]
    #[inline] pub fn set_ctryr<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Months Counter"]
    #[inline] pub fn ctrmo(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1f) as u8) } // [12:8]
    }

    #[doc="Returns true if CTRMO != 0"]
    #[inline] pub fn test_ctrmo(&self) -> bool {
        self.ctrmo() != 0
    }

    #[doc="Sets the CTRMO field."]
    #[inline] pub fn set_ctrmo<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Date Counter"]
    #[inline] pub fn ctrdate(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if CTRDATE != 0"]
    #[inline] pub fn test_ctrdate(&self) -> bool {
        self.ctrdate() != 0
    }

    #[doc="Sets the CTRDATE field."]
    #[inline] pub fn set_ctrdate<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ctrup {
    #[inline]
    fn from(other: u32) -> Self {
         Ctrup(other)
    }
}

impl ::core::fmt::Display for Ctrup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cterr() != 0 { try!(write!(f, " cterr"))}
        if self.ceb() != 0 { try!(write!(f, " ceb"))}
        if self.cb() != 0 { try!(write!(f, " cb"))}
        if self.ctrwkdy() != 0 { try!(write!(f, " ctrwkdy=0x{:x}", self.ctrwkdy()))}
        if self.ctryr() != 0 { try!(write!(f, " ctryr=0x{:x}", self.ctryr()))}
        if self.ctrmo() != 0 { try!(write!(f, " ctrmo=0x{:x}", self.ctrmo()))}
        if self.ctrdate() != 0 { try!(write!(f, " ctrdate=0x{:x}", self.ctrdate()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RTC Alarms Lower"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Almlow(pub u32);
impl Almlow {
    #[doc="Hours Alarm"]
    #[inline] pub fn almhr(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3f) as u8) } // [29:24]
    }

    #[doc="Returns true if ALMHR != 0"]
    #[inline] pub fn test_almhr(&self) -> bool {
        self.almhr() != 0
    }

    #[doc="Sets the ALMHR field."]
    #[inline] pub fn set_almhr<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Minutes Alarm"]
    #[inline] pub fn almmin(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7f) as u8) } // [22:16]
    }

    #[doc="Returns true if ALMMIN != 0"]
    #[inline] pub fn test_almmin(&self) -> bool {
        self.almmin() != 0
    }

    #[doc="Sets the ALMMIN field."]
    #[inline] pub fn set_almmin<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Seconds Alarm"]
    #[inline] pub fn almsec(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7f) as u8) } // [14:8]
    }

    #[doc="Returns true if ALMSEC != 0"]
    #[inline] pub fn test_almsec(&self) -> bool {
        self.almsec() != 0
    }

    #[doc="Sets the ALMSEC field."]
    #[inline] pub fn set_almsec<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="100ths of a second Alarm"]
    #[inline] pub fn alm100(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if ALM100 != 0"]
    #[inline] pub fn test_alm100(&self) -> bool {
        self.alm100() != 0
    }

    #[doc="Sets the ALM100 field."]
    #[inline] pub fn set_alm100<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Almlow {
    #[inline]
    fn from(other: u32) -> Self {
         Almlow(other)
    }
}

impl ::core::fmt::Display for Almlow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Almlow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.almhr() != 0 { try!(write!(f, " almhr=0x{:x}", self.almhr()))}
        if self.almmin() != 0 { try!(write!(f, " almmin=0x{:x}", self.almmin()))}
        if self.almsec() != 0 { try!(write!(f, " almsec=0x{:x}", self.almsec()))}
        if self.alm100() != 0 { try!(write!(f, " alm100=0x{:x}", self.alm100()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RTC Alarms Upper"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Almup(pub u32);
impl Almup {
    #[doc="Weekdays Alarm"]
    #[inline] pub fn almwkdy(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Returns true if ALMWKDY != 0"]
    #[inline] pub fn test_almwkdy(&self) -> bool {
        self.almwkdy() != 0
    }

    #[doc="Sets the ALMWKDY field."]
    #[inline] pub fn set_almwkdy<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Months Alarm"]
    #[inline] pub fn almmo(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1f) as u8) } // [12:8]
    }

    #[doc="Returns true if ALMMO != 0"]
    #[inline] pub fn test_almmo(&self) -> bool {
        self.almmo() != 0
    }

    #[doc="Sets the ALMMO field."]
    #[inline] pub fn set_almmo<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Date Alarm"]
    #[inline] pub fn almdate(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if ALMDATE != 0"]
    #[inline] pub fn test_almdate(&self) -> bool {
        self.almdate() != 0
    }

    #[doc="Sets the ALMDATE field."]
    #[inline] pub fn set_almdate<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Almup {
    #[inline]
    fn from(other: u32) -> Self {
         Almup(other)
    }
}

impl ::core::fmt::Display for Almup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Almup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.almwkdy() != 0 { try!(write!(f, " almwkdy=0x{:x}", self.almwkdy()))}
        if self.almmo() != 0 { try!(write!(f, " almmo=0x{:x}", self.almmo()))}
        if self.almdate() != 0 { try!(write!(f, " almdate=0x{:x}", self.almdate()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RTC Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rtcctl(pub u32);
impl Rtcctl {
    #[doc="Hours Counter mode"]
    #[inline] pub fn hr1224(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if HR1224 != 0"]
    #[inline] pub fn test_hr1224(&self) -> bool {
        self.hr1224() != 0
    }

    #[doc="Sets the HR1224 field."]
    #[inline] pub fn set_hr1224<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="RTC input clock control"]
    #[inline] pub fn rstop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RSTOP != 0"]
    #[inline] pub fn test_rstop(&self) -> bool {
        self.rstop() != 0
    }

    #[doc="Sets the RSTOP field."]
    #[inline] pub fn set_rstop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Alarm repeat interval"]
    #[inline] pub fn rpt(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7) as u8) } // [3:1]
    }

    #[doc="Returns true if RPT != 0"]
    #[inline] pub fn test_rpt(&self) -> bool {
        self.rpt() != 0
    }

    #[doc="Sets the RPT field."]
    #[inline] pub fn set_rpt<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Counter write control"]
    #[inline] pub fn wrtc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if WRTC != 0"]
    #[inline] pub fn test_wrtc(&self) -> bool {
        self.wrtc() != 0
    }

    #[doc="Sets the WRTC field."]
    #[inline] pub fn set_wrtc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rtcctl {
    #[inline]
    fn from(other: u32) -> Self {
         Rtcctl(other)
    }
}

impl ::core::fmt::Display for Rtcctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rtcctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hr1224() != 0 { try!(write!(f, " hr1224"))}
        if self.rstop() != 0 { try!(write!(f, " rstop"))}
        if self.rpt() != 0 { try!(write!(f, " rpt=0x{:x}", self.rpt()))}
        if self.wrtc() != 0 { try!(write!(f, " wrtc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CLKGEN Interrupt Register: Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc="RTC Alarm interrupt"]
    #[inline] pub fn alm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ALM != 0"]
    #[inline] pub fn test_alm(&self) -> bool {
        self.alm() != 0
    }

    #[doc="Sets the ALM field."]
    #[inline] pub fn set_alm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="XT Oscillator Fail interrupt"]
    #[inline] pub fn of(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if OF != 0"]
    #[inline] pub fn test_of(&self) -> bool {
        self.of() != 0
    }

    #[doc="Sets the OF field."]
    #[inline] pub fn set_of<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Autocalibration Complete interrupt"]
    #[inline] pub fn acc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ACC != 0"]
    #[inline] pub fn test_acc(&self) -> bool {
        self.acc() != 0
    }

    #[doc="Sets the ACC field."]
    #[inline] pub fn set_acc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Autocalibration Fail interrupt"]
    #[inline] pub fn acf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ACF != 0"]
    #[inline] pub fn test_acf(&self) -> bool {
        self.acf() != 0
    }

    #[doc="Sets the ACF field."]
    #[inline] pub fn set_acf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Inten {
    #[inline]
    fn from(other: u32) -> Self {
         Inten(other)
    }
}

impl ::core::fmt::Display for Inten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Inten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.alm() != 0 { try!(write!(f, " alm"))}
        if self.of() != 0 { try!(write!(f, " of"))}
        if self.acc() != 0 { try!(write!(f, " acc"))}
        if self.acf() != 0 { try!(write!(f, " acf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CLKGEN Interrupt Register: Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc="RTC Alarm interrupt"]
    #[inline] pub fn alm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ALM != 0"]
    #[inline] pub fn test_alm(&self) -> bool {
        self.alm() != 0
    }

    #[doc="Sets the ALM field."]
    #[inline] pub fn set_alm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="XT Oscillator Fail interrupt"]
    #[inline] pub fn of(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if OF != 0"]
    #[inline] pub fn test_of(&self) -> bool {
        self.of() != 0
    }

    #[doc="Sets the OF field."]
    #[inline] pub fn set_of<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Autocalibration Complete interrupt"]
    #[inline] pub fn acc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ACC != 0"]
    #[inline] pub fn test_acc(&self) -> bool {
        self.acc() != 0
    }

    #[doc="Sets the ACC field."]
    #[inline] pub fn set_acc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Autocalibration Fail interrupt"]
    #[inline] pub fn acf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ACF != 0"]
    #[inline] pub fn test_acf(&self) -> bool {
        self.acf() != 0
    }

    #[doc="Sets the ACF field."]
    #[inline] pub fn set_acf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Intstat {
    #[inline]
    fn from(other: u32) -> Self {
         Intstat(other)
    }
}

impl ::core::fmt::Display for Intstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.alm() != 0 { try!(write!(f, " alm"))}
        if self.of() != 0 { try!(write!(f, " of"))}
        if self.acc() != 0 { try!(write!(f, " acc"))}
        if self.acf() != 0 { try!(write!(f, " acf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CLKGEN Interrupt Register: Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intclr(pub u32);
impl Intclr {
    #[doc="RTC Alarm interrupt"]
    #[inline] pub fn alm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ALM != 0"]
    #[inline] pub fn test_alm(&self) -> bool {
        self.alm() != 0
    }

    #[doc="Sets the ALM field."]
    #[inline] pub fn set_alm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="XT Oscillator Fail interrupt"]
    #[inline] pub fn of(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if OF != 0"]
    #[inline] pub fn test_of(&self) -> bool {
        self.of() != 0
    }

    #[doc="Sets the OF field."]
    #[inline] pub fn set_of<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Autocalibration Complete interrupt"]
    #[inline] pub fn acc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ACC != 0"]
    #[inline] pub fn test_acc(&self) -> bool {
        self.acc() != 0
    }

    #[doc="Sets the ACC field."]
    #[inline] pub fn set_acc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Autocalibration Fail interrupt"]
    #[inline] pub fn acf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ACF != 0"]
    #[inline] pub fn test_acf(&self) -> bool {
        self.acf() != 0
    }

    #[doc="Sets the ACF field."]
    #[inline] pub fn set_acf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Intclr {
    #[inline]
    fn from(other: u32) -> Self {
         Intclr(other)
    }
}

impl ::core::fmt::Display for Intclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.alm() != 0 { try!(write!(f, " alm"))}
        if self.of() != 0 { try!(write!(f, " of"))}
        if self.acc() != 0 { try!(write!(f, " acc"))}
        if self.acf() != 0 { try!(write!(f, " acf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CLKGEN Interrupt Register: Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intset(pub u32);
impl Intset {
    #[doc="RTC Alarm interrupt"]
    #[inline] pub fn alm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ALM != 0"]
    #[inline] pub fn test_alm(&self) -> bool {
        self.alm() != 0
    }

    #[doc="Sets the ALM field."]
    #[inline] pub fn set_alm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="XT Oscillator Fail interrupt"]
    #[inline] pub fn of(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if OF != 0"]
    #[inline] pub fn test_of(&self) -> bool {
        self.of() != 0
    }

    #[doc="Sets the OF field."]
    #[inline] pub fn set_of<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Autocalibration Complete interrupt"]
    #[inline] pub fn acc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ACC != 0"]
    #[inline] pub fn test_acc(&self) -> bool {
        self.acc() != 0
    }

    #[doc="Sets the ACC field."]
    #[inline] pub fn set_acc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Autocalibration Fail interrupt"]
    #[inline] pub fn acf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ACF != 0"]
    #[inline] pub fn test_acf(&self) -> bool {
        self.acf() != 0
    }

    #[doc="Sets the ACF field."]
    #[inline] pub fn set_acf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Intset {
    #[inline]
    fn from(other: u32) -> Self {
         Intset(other)
    }
}

impl ::core::fmt::Display for Intset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.alm() != 0 { try!(write!(f, " alm"))}
        if self.of() != 0 { try!(write!(f, " of"))}
        if self.acc() != 0 { try!(write!(f, " acc"))}
        if self.acf() != 0 { try!(write!(f, " acf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}


