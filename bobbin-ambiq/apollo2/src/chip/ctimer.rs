//! Counter/Timer
#[allow(unused_imports)] use bobbin_common::*;

periph!(CTIMER, Ctimer, 0x40008000);

#[doc="Counter/Timer"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctimer(pub usize);
impl Ctimer {
    #[doc="Get the *mut pointer for the TMR0 register."]
    #[inline] pub fn tmr0_mut(&self) -> *mut Tmr0 { 
        (self.0 + 0x0) as *mut Tmr0
    }

    #[doc="Get the *const pointer for the TMR0 register."]
    #[inline] pub fn tmr0_ptr(&self) -> *const Tmr0 { 
           self.tmr0_mut()
    }

    #[doc="Read the TMR0 register."]
    #[inline] pub fn tmr0(&self) -> Tmr0 { 
        unsafe {
            read_volatile(self.tmr0_ptr())
        }
    }

    #[doc="Write the TMR0 register."]
    #[inline] pub fn set_tmr0<F: FnOnce(Tmr0) -> Tmr0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tmr0_mut(), f(Tmr0(0)));
        }
        self
    }

    #[doc="Modify the TMR0 register."]
    #[inline] pub fn with_tmr0<F: FnOnce(Tmr0) -> Tmr0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tmr0_mut(), f(self.tmr0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CMPRA0 register."]
    #[inline] pub fn cmpra0_mut(&self) -> *mut Cmpra0 { 
        (self.0 + 0x4) as *mut Cmpra0
    }

    #[doc="Get the *const pointer for the CMPRA0 register."]
    #[inline] pub fn cmpra0_ptr(&self) -> *const Cmpra0 { 
           self.cmpra0_mut()
    }

    #[doc="Read the CMPRA0 register."]
    #[inline] pub fn cmpra0(&self) -> Cmpra0 { 
        unsafe {
            read_volatile(self.cmpra0_ptr())
        }
    }

    #[doc="Write the CMPRA0 register."]
    #[inline] pub fn set_cmpra0<F: FnOnce(Cmpra0) -> Cmpra0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmpra0_mut(), f(Cmpra0(0)));
        }
        self
    }

    #[doc="Modify the CMPRA0 register."]
    #[inline] pub fn with_cmpra0<F: FnOnce(Cmpra0) -> Cmpra0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmpra0_mut(), f(self.cmpra0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CMPRB0 register."]
    #[inline] pub fn cmprb0_mut(&self) -> *mut Cmprb0 { 
        (self.0 + 0x8) as *mut Cmprb0
    }

    #[doc="Get the *const pointer for the CMPRB0 register."]
    #[inline] pub fn cmprb0_ptr(&self) -> *const Cmprb0 { 
           self.cmprb0_mut()
    }

    #[doc="Read the CMPRB0 register."]
    #[inline] pub fn cmprb0(&self) -> Cmprb0 { 
        unsafe {
            read_volatile(self.cmprb0_ptr())
        }
    }

    #[doc="Write the CMPRB0 register."]
    #[inline] pub fn set_cmprb0<F: FnOnce(Cmprb0) -> Cmprb0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmprb0_mut(), f(Cmprb0(0)));
        }
        self
    }

    #[doc="Modify the CMPRB0 register."]
    #[inline] pub fn with_cmprb0<F: FnOnce(Cmprb0) -> Cmprb0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmprb0_mut(), f(self.cmprb0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTRL0 register."]
    #[inline] pub fn ctrl0_mut(&self) -> *mut Ctrl0 { 
        (self.0 + 0xc) as *mut Ctrl0
    }

    #[doc="Get the *const pointer for the CTRL0 register."]
    #[inline] pub fn ctrl0_ptr(&self) -> *const Ctrl0 { 
           self.ctrl0_mut()
    }

    #[doc="Read the CTRL0 register."]
    #[inline] pub fn ctrl0(&self) -> Ctrl0 { 
        unsafe {
            read_volatile(self.ctrl0_ptr())
        }
    }

    #[doc="Write the CTRL0 register."]
    #[inline] pub fn set_ctrl0<F: FnOnce(Ctrl0) -> Ctrl0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrl0_mut(), f(Ctrl0(0)));
        }
        self
    }

    #[doc="Modify the CTRL0 register."]
    #[inline] pub fn with_ctrl0<F: FnOnce(Ctrl0) -> Ctrl0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrl0_mut(), f(self.ctrl0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TMR1 register."]
    #[inline] pub fn tmr1_mut(&self) -> *mut Tmr1 { 
        (self.0 + 0x10) as *mut Tmr1
    }

    #[doc="Get the *const pointer for the TMR1 register."]
    #[inline] pub fn tmr1_ptr(&self) -> *const Tmr1 { 
           self.tmr1_mut()
    }

    #[doc="Read the TMR1 register."]
    #[inline] pub fn tmr1(&self) -> Tmr1 { 
        unsafe {
            read_volatile(self.tmr1_ptr())
        }
    }

    #[doc="Write the TMR1 register."]
    #[inline] pub fn set_tmr1<F: FnOnce(Tmr1) -> Tmr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tmr1_mut(), f(Tmr1(0)));
        }
        self
    }

    #[doc="Modify the TMR1 register."]
    #[inline] pub fn with_tmr1<F: FnOnce(Tmr1) -> Tmr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tmr1_mut(), f(self.tmr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CMPRA1 register."]
    #[inline] pub fn cmpra1_mut(&self) -> *mut Cmpra1 { 
        (self.0 + 0x14) as *mut Cmpra1
    }

    #[doc="Get the *const pointer for the CMPRA1 register."]
    #[inline] pub fn cmpra1_ptr(&self) -> *const Cmpra1 { 
           self.cmpra1_mut()
    }

    #[doc="Read the CMPRA1 register."]
    #[inline] pub fn cmpra1(&self) -> Cmpra1 { 
        unsafe {
            read_volatile(self.cmpra1_ptr())
        }
    }

    #[doc="Write the CMPRA1 register."]
    #[inline] pub fn set_cmpra1<F: FnOnce(Cmpra1) -> Cmpra1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmpra1_mut(), f(Cmpra1(0)));
        }
        self
    }

    #[doc="Modify the CMPRA1 register."]
    #[inline] pub fn with_cmpra1<F: FnOnce(Cmpra1) -> Cmpra1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmpra1_mut(), f(self.cmpra1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CMPRB1 register."]
    #[inline] pub fn cmprb1_mut(&self) -> *mut Cmprb1 { 
        (self.0 + 0x18) as *mut Cmprb1
    }

    #[doc="Get the *const pointer for the CMPRB1 register."]
    #[inline] pub fn cmprb1_ptr(&self) -> *const Cmprb1 { 
           self.cmprb1_mut()
    }

    #[doc="Read the CMPRB1 register."]
    #[inline] pub fn cmprb1(&self) -> Cmprb1 { 
        unsafe {
            read_volatile(self.cmprb1_ptr())
        }
    }

    #[doc="Write the CMPRB1 register."]
    #[inline] pub fn set_cmprb1<F: FnOnce(Cmprb1) -> Cmprb1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmprb1_mut(), f(Cmprb1(0)));
        }
        self
    }

    #[doc="Modify the CMPRB1 register."]
    #[inline] pub fn with_cmprb1<F: FnOnce(Cmprb1) -> Cmprb1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmprb1_mut(), f(self.cmprb1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTRL1 register."]
    #[inline] pub fn ctrl1_mut(&self) -> *mut Ctrl1 { 
        (self.0 + 0x1c) as *mut Ctrl1
    }

    #[doc="Get the *const pointer for the CTRL1 register."]
    #[inline] pub fn ctrl1_ptr(&self) -> *const Ctrl1 { 
           self.ctrl1_mut()
    }

    #[doc="Read the CTRL1 register."]
    #[inline] pub fn ctrl1(&self) -> Ctrl1 { 
        unsafe {
            read_volatile(self.ctrl1_ptr())
        }
    }

    #[doc="Write the CTRL1 register."]
    #[inline] pub fn set_ctrl1<F: FnOnce(Ctrl1) -> Ctrl1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrl1_mut(), f(Ctrl1(0)));
        }
        self
    }

    #[doc="Modify the CTRL1 register."]
    #[inline] pub fn with_ctrl1<F: FnOnce(Ctrl1) -> Ctrl1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrl1_mut(), f(self.ctrl1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TMR2 register."]
    #[inline] pub fn tmr2_mut(&self) -> *mut Tmr2 { 
        (self.0 + 0x20) as *mut Tmr2
    }

    #[doc="Get the *const pointer for the TMR2 register."]
    #[inline] pub fn tmr2_ptr(&self) -> *const Tmr2 { 
           self.tmr2_mut()
    }

    #[doc="Read the TMR2 register."]
    #[inline] pub fn tmr2(&self) -> Tmr2 { 
        unsafe {
            read_volatile(self.tmr2_ptr())
        }
    }

    #[doc="Write the TMR2 register."]
    #[inline] pub fn set_tmr2<F: FnOnce(Tmr2) -> Tmr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tmr2_mut(), f(Tmr2(0)));
        }
        self
    }

    #[doc="Modify the TMR2 register."]
    #[inline] pub fn with_tmr2<F: FnOnce(Tmr2) -> Tmr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tmr2_mut(), f(self.tmr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CMPRA2 register."]
    #[inline] pub fn cmpra2_mut(&self) -> *mut Cmpra2 { 
        (self.0 + 0x24) as *mut Cmpra2
    }

    #[doc="Get the *const pointer for the CMPRA2 register."]
    #[inline] pub fn cmpra2_ptr(&self) -> *const Cmpra2 { 
           self.cmpra2_mut()
    }

    #[doc="Read the CMPRA2 register."]
    #[inline] pub fn cmpra2(&self) -> Cmpra2 { 
        unsafe {
            read_volatile(self.cmpra2_ptr())
        }
    }

    #[doc="Write the CMPRA2 register."]
    #[inline] pub fn set_cmpra2<F: FnOnce(Cmpra2) -> Cmpra2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmpra2_mut(), f(Cmpra2(0)));
        }
        self
    }

    #[doc="Modify the CMPRA2 register."]
    #[inline] pub fn with_cmpra2<F: FnOnce(Cmpra2) -> Cmpra2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmpra2_mut(), f(self.cmpra2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CMPRB2 register."]
    #[inline] pub fn cmprb2_mut(&self) -> *mut Cmprb2 { 
        (self.0 + 0x28) as *mut Cmprb2
    }

    #[doc="Get the *const pointer for the CMPRB2 register."]
    #[inline] pub fn cmprb2_ptr(&self) -> *const Cmprb2 { 
           self.cmprb2_mut()
    }

    #[doc="Read the CMPRB2 register."]
    #[inline] pub fn cmprb2(&self) -> Cmprb2 { 
        unsafe {
            read_volatile(self.cmprb2_ptr())
        }
    }

    #[doc="Write the CMPRB2 register."]
    #[inline] pub fn set_cmprb2<F: FnOnce(Cmprb2) -> Cmprb2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmprb2_mut(), f(Cmprb2(0)));
        }
        self
    }

    #[doc="Modify the CMPRB2 register."]
    #[inline] pub fn with_cmprb2<F: FnOnce(Cmprb2) -> Cmprb2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmprb2_mut(), f(self.cmprb2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTRL2 register."]
    #[inline] pub fn ctrl2_mut(&self) -> *mut Ctrl2 { 
        (self.0 + 0x2c) as *mut Ctrl2
    }

    #[doc="Get the *const pointer for the CTRL2 register."]
    #[inline] pub fn ctrl2_ptr(&self) -> *const Ctrl2 { 
           self.ctrl2_mut()
    }

    #[doc="Read the CTRL2 register."]
    #[inline] pub fn ctrl2(&self) -> Ctrl2 { 
        unsafe {
            read_volatile(self.ctrl2_ptr())
        }
    }

    #[doc="Write the CTRL2 register."]
    #[inline] pub fn set_ctrl2<F: FnOnce(Ctrl2) -> Ctrl2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrl2_mut(), f(Ctrl2(0)));
        }
        self
    }

    #[doc="Modify the CTRL2 register."]
    #[inline] pub fn with_ctrl2<F: FnOnce(Ctrl2) -> Ctrl2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrl2_mut(), f(self.ctrl2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TMR3 register."]
    #[inline] pub fn tmr3_mut(&self) -> *mut Tmr3 { 
        (self.0 + 0x30) as *mut Tmr3
    }

    #[doc="Get the *const pointer for the TMR3 register."]
    #[inline] pub fn tmr3_ptr(&self) -> *const Tmr3 { 
           self.tmr3_mut()
    }

    #[doc="Read the TMR3 register."]
    #[inline] pub fn tmr3(&self) -> Tmr3 { 
        unsafe {
            read_volatile(self.tmr3_ptr())
        }
    }

    #[doc="Write the TMR3 register."]
    #[inline] pub fn set_tmr3<F: FnOnce(Tmr3) -> Tmr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tmr3_mut(), f(Tmr3(0)));
        }
        self
    }

    #[doc="Modify the TMR3 register."]
    #[inline] pub fn with_tmr3<F: FnOnce(Tmr3) -> Tmr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tmr3_mut(), f(self.tmr3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CMPRA3 register."]
    #[inline] pub fn cmpra3_mut(&self) -> *mut Cmpra3 { 
        (self.0 + 0x34) as *mut Cmpra3
    }

    #[doc="Get the *const pointer for the CMPRA3 register."]
    #[inline] pub fn cmpra3_ptr(&self) -> *const Cmpra3 { 
           self.cmpra3_mut()
    }

    #[doc="Read the CMPRA3 register."]
    #[inline] pub fn cmpra3(&self) -> Cmpra3 { 
        unsafe {
            read_volatile(self.cmpra3_ptr())
        }
    }

    #[doc="Write the CMPRA3 register."]
    #[inline] pub fn set_cmpra3<F: FnOnce(Cmpra3) -> Cmpra3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmpra3_mut(), f(Cmpra3(0)));
        }
        self
    }

    #[doc="Modify the CMPRA3 register."]
    #[inline] pub fn with_cmpra3<F: FnOnce(Cmpra3) -> Cmpra3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmpra3_mut(), f(self.cmpra3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CMPRB3 register."]
    #[inline] pub fn cmprb3_mut(&self) -> *mut Cmprb3 { 
        (self.0 + 0x38) as *mut Cmprb3
    }

    #[doc="Get the *const pointer for the CMPRB3 register."]
    #[inline] pub fn cmprb3_ptr(&self) -> *const Cmprb3 { 
           self.cmprb3_mut()
    }

    #[doc="Read the CMPRB3 register."]
    #[inline] pub fn cmprb3(&self) -> Cmprb3 { 
        unsafe {
            read_volatile(self.cmprb3_ptr())
        }
    }

    #[doc="Write the CMPRB3 register."]
    #[inline] pub fn set_cmprb3<F: FnOnce(Cmprb3) -> Cmprb3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmprb3_mut(), f(Cmprb3(0)));
        }
        self
    }

    #[doc="Modify the CMPRB3 register."]
    #[inline] pub fn with_cmprb3<F: FnOnce(Cmprb3) -> Cmprb3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmprb3_mut(), f(self.cmprb3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTRL3 register."]
    #[inline] pub fn ctrl3_mut(&self) -> *mut Ctrl3 { 
        (self.0 + 0x3c) as *mut Ctrl3
    }

    #[doc="Get the *const pointer for the CTRL3 register."]
    #[inline] pub fn ctrl3_ptr(&self) -> *const Ctrl3 { 
           self.ctrl3_mut()
    }

    #[doc="Read the CTRL3 register."]
    #[inline] pub fn ctrl3(&self) -> Ctrl3 { 
        unsafe {
            read_volatile(self.ctrl3_ptr())
        }
    }

    #[doc="Write the CTRL3 register."]
    #[inline] pub fn set_ctrl3<F: FnOnce(Ctrl3) -> Ctrl3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrl3_mut(), f(Ctrl3(0)));
        }
        self
    }

    #[doc="Modify the CTRL3 register."]
    #[inline] pub fn with_ctrl3<F: FnOnce(Ctrl3) -> Ctrl3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrl3_mut(), f(self.ctrl3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the STCFG register."]
    #[inline] pub fn stcfg_mut(&self) -> *mut Stcfg { 
        (self.0 + 0x100) as *mut Stcfg
    }

    #[doc="Get the *const pointer for the STCFG register."]
    #[inline] pub fn stcfg_ptr(&self) -> *const Stcfg { 
           self.stcfg_mut()
    }

    #[doc="Read the STCFG register."]
    #[inline] pub fn stcfg(&self) -> Stcfg { 
        unsafe {
            read_volatile(self.stcfg_ptr())
        }
    }

    #[doc="Write the STCFG register."]
    #[inline] pub fn set_stcfg<F: FnOnce(Stcfg) -> Stcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stcfg_mut(), f(Stcfg(0)));
        }
        self
    }

    #[doc="Modify the STCFG register."]
    #[inline] pub fn with_stcfg<F: FnOnce(Stcfg) -> Stcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stcfg_mut(), f(self.stcfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the STTMR register."]
    #[inline] pub fn sttmr_mut(&self) -> *mut Sttmr { 
        (self.0 + 0x104) as *mut Sttmr
    }

    #[doc="Get the *const pointer for the STTMR register."]
    #[inline] pub fn sttmr_ptr(&self) -> *const Sttmr { 
           self.sttmr_mut()
    }

    #[doc="Read the STTMR register."]
    #[inline] pub fn sttmr(&self) -> Sttmr { 
        unsafe {
            read_volatile(self.sttmr_ptr())
        }
    }

    #[doc="Write the STTMR register."]
    #[inline] pub fn set_sttmr<F: FnOnce(Sttmr) -> Sttmr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sttmr_mut(), f(Sttmr(0)));
        }
        self
    }

    #[doc="Modify the STTMR register."]
    #[inline] pub fn with_sttmr<F: FnOnce(Sttmr) -> Sttmr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sttmr_mut(), f(self.sttmr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CAPTURE_CONTROL register."]
    #[inline] pub fn capture_control_mut(&self) -> *mut CaptureControl { 
        (self.0 + 0x108) as *mut CaptureControl
    }

    #[doc="Get the *const pointer for the CAPTURE_CONTROL register."]
    #[inline] pub fn capture_control_ptr(&self) -> *const CaptureControl { 
           self.capture_control_mut()
    }

    #[doc="Read the CAPTURE_CONTROL register."]
    #[inline] pub fn capture_control(&self) -> CaptureControl { 
        unsafe {
            read_volatile(self.capture_control_ptr())
        }
    }

    #[doc="Write the CAPTURE_CONTROL register."]
    #[inline] pub fn set_capture_control<F: FnOnce(CaptureControl) -> CaptureControl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.capture_control_mut(), f(CaptureControl(0)));
        }
        self
    }

    #[doc="Modify the CAPTURE_CONTROL register."]
    #[inline] pub fn with_capture_control<F: FnOnce(CaptureControl) -> CaptureControl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.capture_control_mut(), f(self.capture_control()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SCMPR0 register."]
    #[inline] pub fn scmpr0_mut(&self) -> *mut Scmpr0 { 
        (self.0 + 0x110) as *mut Scmpr0
    }

    #[doc="Get the *const pointer for the SCMPR0 register."]
    #[inline] pub fn scmpr0_ptr(&self) -> *const Scmpr0 { 
           self.scmpr0_mut()
    }

    #[doc="Read the SCMPR0 register."]
    #[inline] pub fn scmpr0(&self) -> Scmpr0 { 
        unsafe {
            read_volatile(self.scmpr0_ptr())
        }
    }

    #[doc="Write the SCMPR0 register."]
    #[inline] pub fn set_scmpr0<F: FnOnce(Scmpr0) -> Scmpr0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scmpr0_mut(), f(Scmpr0(0)));
        }
        self
    }

    #[doc="Modify the SCMPR0 register."]
    #[inline] pub fn with_scmpr0<F: FnOnce(Scmpr0) -> Scmpr0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scmpr0_mut(), f(self.scmpr0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SCMPR1 register."]
    #[inline] pub fn scmpr1_mut(&self) -> *mut Scmpr1 { 
        (self.0 + 0x114) as *mut Scmpr1
    }

    #[doc="Get the *const pointer for the SCMPR1 register."]
    #[inline] pub fn scmpr1_ptr(&self) -> *const Scmpr1 { 
           self.scmpr1_mut()
    }

    #[doc="Read the SCMPR1 register."]
    #[inline] pub fn scmpr1(&self) -> Scmpr1 { 
        unsafe {
            read_volatile(self.scmpr1_ptr())
        }
    }

    #[doc="Write the SCMPR1 register."]
    #[inline] pub fn set_scmpr1<F: FnOnce(Scmpr1) -> Scmpr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scmpr1_mut(), f(Scmpr1(0)));
        }
        self
    }

    #[doc="Modify the SCMPR1 register."]
    #[inline] pub fn with_scmpr1<F: FnOnce(Scmpr1) -> Scmpr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scmpr1_mut(), f(self.scmpr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SCMPR2 register."]
    #[inline] pub fn scmpr2_mut(&self) -> *mut Scmpr2 { 
        (self.0 + 0x118) as *mut Scmpr2
    }

    #[doc="Get the *const pointer for the SCMPR2 register."]
    #[inline] pub fn scmpr2_ptr(&self) -> *const Scmpr2 { 
           self.scmpr2_mut()
    }

    #[doc="Read the SCMPR2 register."]
    #[inline] pub fn scmpr2(&self) -> Scmpr2 { 
        unsafe {
            read_volatile(self.scmpr2_ptr())
        }
    }

    #[doc="Write the SCMPR2 register."]
    #[inline] pub fn set_scmpr2<F: FnOnce(Scmpr2) -> Scmpr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scmpr2_mut(), f(Scmpr2(0)));
        }
        self
    }

    #[doc="Modify the SCMPR2 register."]
    #[inline] pub fn with_scmpr2<F: FnOnce(Scmpr2) -> Scmpr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scmpr2_mut(), f(self.scmpr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SCMPR3 register."]
    #[inline] pub fn scmpr3_mut(&self) -> *mut Scmpr3 { 
        (self.0 + 0x11c) as *mut Scmpr3
    }

    #[doc="Get the *const pointer for the SCMPR3 register."]
    #[inline] pub fn scmpr3_ptr(&self) -> *const Scmpr3 { 
           self.scmpr3_mut()
    }

    #[doc="Read the SCMPR3 register."]
    #[inline] pub fn scmpr3(&self) -> Scmpr3 { 
        unsafe {
            read_volatile(self.scmpr3_ptr())
        }
    }

    #[doc="Write the SCMPR3 register."]
    #[inline] pub fn set_scmpr3<F: FnOnce(Scmpr3) -> Scmpr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scmpr3_mut(), f(Scmpr3(0)));
        }
        self
    }

    #[doc="Modify the SCMPR3 register."]
    #[inline] pub fn with_scmpr3<F: FnOnce(Scmpr3) -> Scmpr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scmpr3_mut(), f(self.scmpr3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SCMPR4 register."]
    #[inline] pub fn scmpr4_mut(&self) -> *mut Scmpr4 { 
        (self.0 + 0x120) as *mut Scmpr4
    }

    #[doc="Get the *const pointer for the SCMPR4 register."]
    #[inline] pub fn scmpr4_ptr(&self) -> *const Scmpr4 { 
           self.scmpr4_mut()
    }

    #[doc="Read the SCMPR4 register."]
    #[inline] pub fn scmpr4(&self) -> Scmpr4 { 
        unsafe {
            read_volatile(self.scmpr4_ptr())
        }
    }

    #[doc="Write the SCMPR4 register."]
    #[inline] pub fn set_scmpr4<F: FnOnce(Scmpr4) -> Scmpr4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scmpr4_mut(), f(Scmpr4(0)));
        }
        self
    }

    #[doc="Modify the SCMPR4 register."]
    #[inline] pub fn with_scmpr4<F: FnOnce(Scmpr4) -> Scmpr4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scmpr4_mut(), f(self.scmpr4()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SCMPR5 register."]
    #[inline] pub fn scmpr5_mut(&self) -> *mut Scmpr5 { 
        (self.0 + 0x124) as *mut Scmpr5
    }

    #[doc="Get the *const pointer for the SCMPR5 register."]
    #[inline] pub fn scmpr5_ptr(&self) -> *const Scmpr5 { 
           self.scmpr5_mut()
    }

    #[doc="Read the SCMPR5 register."]
    #[inline] pub fn scmpr5(&self) -> Scmpr5 { 
        unsafe {
            read_volatile(self.scmpr5_ptr())
        }
    }

    #[doc="Write the SCMPR5 register."]
    #[inline] pub fn set_scmpr5<F: FnOnce(Scmpr5) -> Scmpr5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scmpr5_mut(), f(Scmpr5(0)));
        }
        self
    }

    #[doc="Modify the SCMPR5 register."]
    #[inline] pub fn with_scmpr5<F: FnOnce(Scmpr5) -> Scmpr5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scmpr5_mut(), f(self.scmpr5()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SCMPR6 register."]
    #[inline] pub fn scmpr6_mut(&self) -> *mut Scmpr6 { 
        (self.0 + 0x128) as *mut Scmpr6
    }

    #[doc="Get the *const pointer for the SCMPR6 register."]
    #[inline] pub fn scmpr6_ptr(&self) -> *const Scmpr6 { 
           self.scmpr6_mut()
    }

    #[doc="Read the SCMPR6 register."]
    #[inline] pub fn scmpr6(&self) -> Scmpr6 { 
        unsafe {
            read_volatile(self.scmpr6_ptr())
        }
    }

    #[doc="Write the SCMPR6 register."]
    #[inline] pub fn set_scmpr6<F: FnOnce(Scmpr6) -> Scmpr6>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scmpr6_mut(), f(Scmpr6(0)));
        }
        self
    }

    #[doc="Modify the SCMPR6 register."]
    #[inline] pub fn with_scmpr6<F: FnOnce(Scmpr6) -> Scmpr6>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scmpr6_mut(), f(self.scmpr6()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SCMPR7 register."]
    #[inline] pub fn scmpr7_mut(&self) -> *mut Scmpr7 { 
        (self.0 + 0x12c) as *mut Scmpr7
    }

    #[doc="Get the *const pointer for the SCMPR7 register."]
    #[inline] pub fn scmpr7_ptr(&self) -> *const Scmpr7 { 
           self.scmpr7_mut()
    }

    #[doc="Read the SCMPR7 register."]
    #[inline] pub fn scmpr7(&self) -> Scmpr7 { 
        unsafe {
            read_volatile(self.scmpr7_ptr())
        }
    }

    #[doc="Write the SCMPR7 register."]
    #[inline] pub fn set_scmpr7<F: FnOnce(Scmpr7) -> Scmpr7>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scmpr7_mut(), f(Scmpr7(0)));
        }
        self
    }

    #[doc="Modify the SCMPR7 register."]
    #[inline] pub fn with_scmpr7<F: FnOnce(Scmpr7) -> Scmpr7>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scmpr7_mut(), f(self.scmpr7()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SCAPT0 register."]
    #[inline] pub fn scapt0_mut(&self) -> *mut Scapt0 { 
        (self.0 + 0x1e0) as *mut Scapt0
    }

    #[doc="Get the *const pointer for the SCAPT0 register."]
    #[inline] pub fn scapt0_ptr(&self) -> *const Scapt0 { 
           self.scapt0_mut()
    }

    #[doc="Read the SCAPT0 register."]
    #[inline] pub fn scapt0(&self) -> Scapt0 { 
        unsafe {
            read_volatile(self.scapt0_ptr())
        }
    }

    #[doc="Write the SCAPT0 register."]
    #[inline] pub fn set_scapt0<F: FnOnce(Scapt0) -> Scapt0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scapt0_mut(), f(Scapt0(0)));
        }
        self
    }

    #[doc="Modify the SCAPT0 register."]
    #[inline] pub fn with_scapt0<F: FnOnce(Scapt0) -> Scapt0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scapt0_mut(), f(self.scapt0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SCAPT1 register."]
    #[inline] pub fn scapt1_mut(&self) -> *mut Scapt1 { 
        (self.0 + 0x1e4) as *mut Scapt1
    }

    #[doc="Get the *const pointer for the SCAPT1 register."]
    #[inline] pub fn scapt1_ptr(&self) -> *const Scapt1 { 
           self.scapt1_mut()
    }

    #[doc="Read the SCAPT1 register."]
    #[inline] pub fn scapt1(&self) -> Scapt1 { 
        unsafe {
            read_volatile(self.scapt1_ptr())
        }
    }

    #[doc="Write the SCAPT1 register."]
    #[inline] pub fn set_scapt1<F: FnOnce(Scapt1) -> Scapt1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scapt1_mut(), f(Scapt1(0)));
        }
        self
    }

    #[doc="Modify the SCAPT1 register."]
    #[inline] pub fn with_scapt1<F: FnOnce(Scapt1) -> Scapt1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scapt1_mut(), f(self.scapt1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SCAPT2 register."]
    #[inline] pub fn scapt2_mut(&self) -> *mut Scapt2 { 
        (self.0 + 0x1e8) as *mut Scapt2
    }

    #[doc="Get the *const pointer for the SCAPT2 register."]
    #[inline] pub fn scapt2_ptr(&self) -> *const Scapt2 { 
           self.scapt2_mut()
    }

    #[doc="Read the SCAPT2 register."]
    #[inline] pub fn scapt2(&self) -> Scapt2 { 
        unsafe {
            read_volatile(self.scapt2_ptr())
        }
    }

    #[doc="Write the SCAPT2 register."]
    #[inline] pub fn set_scapt2<F: FnOnce(Scapt2) -> Scapt2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scapt2_mut(), f(Scapt2(0)));
        }
        self
    }

    #[doc="Modify the SCAPT2 register."]
    #[inline] pub fn with_scapt2<F: FnOnce(Scapt2) -> Scapt2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scapt2_mut(), f(self.scapt2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SCAPT3 register."]
    #[inline] pub fn scapt3_mut(&self) -> *mut Scapt3 { 
        (self.0 + 0x1ec) as *mut Scapt3
    }

    #[doc="Get the *const pointer for the SCAPT3 register."]
    #[inline] pub fn scapt3_ptr(&self) -> *const Scapt3 { 
           self.scapt3_mut()
    }

    #[doc="Read the SCAPT3 register."]
    #[inline] pub fn scapt3(&self) -> Scapt3 { 
        unsafe {
            read_volatile(self.scapt3_ptr())
        }
    }

    #[doc="Write the SCAPT3 register."]
    #[inline] pub fn set_scapt3<F: FnOnce(Scapt3) -> Scapt3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scapt3_mut(), f(Scapt3(0)));
        }
        self
    }

    #[doc="Modify the SCAPT3 register."]
    #[inline] pub fn with_scapt3<F: FnOnce(Scapt3) -> Scapt3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scapt3_mut(), f(self.scapt3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SNVR0 register."]
    #[inline] pub fn snvr0_mut(&self) -> *mut Snvr0 { 
        (self.0 + 0x1f0) as *mut Snvr0
    }

    #[doc="Get the *const pointer for the SNVR0 register."]
    #[inline] pub fn snvr0_ptr(&self) -> *const Snvr0 { 
           self.snvr0_mut()
    }

    #[doc="Read the SNVR0 register."]
    #[inline] pub fn snvr0(&self) -> Snvr0 { 
        unsafe {
            read_volatile(self.snvr0_ptr())
        }
    }

    #[doc="Write the SNVR0 register."]
    #[inline] pub fn set_snvr0<F: FnOnce(Snvr0) -> Snvr0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.snvr0_mut(), f(Snvr0(0)));
        }
        self
    }

    #[doc="Modify the SNVR0 register."]
    #[inline] pub fn with_snvr0<F: FnOnce(Snvr0) -> Snvr0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.snvr0_mut(), f(self.snvr0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SNVR1 register."]
    #[inline] pub fn snvr1_mut(&self) -> *mut Snvr1 { 
        (self.0 + 0x1f4) as *mut Snvr1
    }

    #[doc="Get the *const pointer for the SNVR1 register."]
    #[inline] pub fn snvr1_ptr(&self) -> *const Snvr1 { 
           self.snvr1_mut()
    }

    #[doc="Read the SNVR1 register."]
    #[inline] pub fn snvr1(&self) -> Snvr1 { 
        unsafe {
            read_volatile(self.snvr1_ptr())
        }
    }

    #[doc="Write the SNVR1 register."]
    #[inline] pub fn set_snvr1<F: FnOnce(Snvr1) -> Snvr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.snvr1_mut(), f(Snvr1(0)));
        }
        self
    }

    #[doc="Modify the SNVR1 register."]
    #[inline] pub fn with_snvr1<F: FnOnce(Snvr1) -> Snvr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.snvr1_mut(), f(self.snvr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SNVR2 register."]
    #[inline] pub fn snvr2_mut(&self) -> *mut Snvr2 { 
        (self.0 + 0x1f8) as *mut Snvr2
    }

    #[doc="Get the *const pointer for the SNVR2 register."]
    #[inline] pub fn snvr2_ptr(&self) -> *const Snvr2 { 
           self.snvr2_mut()
    }

    #[doc="Read the SNVR2 register."]
    #[inline] pub fn snvr2(&self) -> Snvr2 { 
        unsafe {
            read_volatile(self.snvr2_ptr())
        }
    }

    #[doc="Write the SNVR2 register."]
    #[inline] pub fn set_snvr2<F: FnOnce(Snvr2) -> Snvr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.snvr2_mut(), f(Snvr2(0)));
        }
        self
    }

    #[doc="Modify the SNVR2 register."]
    #[inline] pub fn with_snvr2<F: FnOnce(Snvr2) -> Snvr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.snvr2_mut(), f(self.snvr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTEN register."]
    #[inline] pub fn inten_mut(&self) -> *mut Inten { 
        (self.0 + 0x200) as *mut Inten
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
        (self.0 + 0x204) as *mut Intstat
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
        (self.0 + 0x208) as *mut Intclr
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
        (self.0 + 0x20c) as *mut Intset
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

    #[doc="Get the *mut pointer for the STMINTEN register."]
    #[inline] pub fn stminten_mut(&self) -> *mut Stminten { 
        (self.0 + 0x300) as *mut Stminten
    }

    #[doc="Get the *const pointer for the STMINTEN register."]
    #[inline] pub fn stminten_ptr(&self) -> *const Stminten { 
           self.stminten_mut()
    }

    #[doc="Read the STMINTEN register."]
    #[inline] pub fn stminten(&self) -> Stminten { 
        unsafe {
            read_volatile(self.stminten_ptr())
        }
    }

    #[doc="Write the STMINTEN register."]
    #[inline] pub fn set_stminten<F: FnOnce(Stminten) -> Stminten>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stminten_mut(), f(Stminten(0)));
        }
        self
    }

    #[doc="Modify the STMINTEN register."]
    #[inline] pub fn with_stminten<F: FnOnce(Stminten) -> Stminten>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stminten_mut(), f(self.stminten()));
        }
        self
    }

    #[doc="Get the *mut pointer for the STMINTSTAT register."]
    #[inline] pub fn stmintstat_mut(&self) -> *mut Stmintstat { 
        (self.0 + 0x304) as *mut Stmintstat
    }

    #[doc="Get the *const pointer for the STMINTSTAT register."]
    #[inline] pub fn stmintstat_ptr(&self) -> *const Stmintstat { 
           self.stmintstat_mut()
    }

    #[doc="Read the STMINTSTAT register."]
    #[inline] pub fn stmintstat(&self) -> Stmintstat { 
        unsafe {
            read_volatile(self.stmintstat_ptr())
        }
    }

    #[doc="Write the STMINTSTAT register."]
    #[inline] pub fn set_stmintstat<F: FnOnce(Stmintstat) -> Stmintstat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stmintstat_mut(), f(Stmintstat(0)));
        }
        self
    }

    #[doc="Modify the STMINTSTAT register."]
    #[inline] pub fn with_stmintstat<F: FnOnce(Stmintstat) -> Stmintstat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stmintstat_mut(), f(self.stmintstat()));
        }
        self
    }

    #[doc="Get the *mut pointer for the STMINTCLR register."]
    #[inline] pub fn stmintclr_mut(&self) -> *mut Stmintclr { 
        (self.0 + 0x308) as *mut Stmintclr
    }

    #[doc="Get the *const pointer for the STMINTCLR register."]
    #[inline] pub fn stmintclr_ptr(&self) -> *const Stmintclr { 
           self.stmintclr_mut()
    }

    #[doc="Read the STMINTCLR register."]
    #[inline] pub fn stmintclr(&self) -> Stmintclr { 
        unsafe {
            read_volatile(self.stmintclr_ptr())
        }
    }

    #[doc="Write the STMINTCLR register."]
    #[inline] pub fn set_stmintclr<F: FnOnce(Stmintclr) -> Stmintclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stmintclr_mut(), f(Stmintclr(0)));
        }
        self
    }

    #[doc="Modify the STMINTCLR register."]
    #[inline] pub fn with_stmintclr<F: FnOnce(Stmintclr) -> Stmintclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stmintclr_mut(), f(self.stmintclr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the STMINTSET register."]
    #[inline] pub fn stmintset_mut(&self) -> *mut Stmintset { 
        (self.0 + 0x30c) as *mut Stmintset
    }

    #[doc="Get the *const pointer for the STMINTSET register."]
    #[inline] pub fn stmintset_ptr(&self) -> *const Stmintset { 
           self.stmintset_mut()
    }

    #[doc="Read the STMINTSET register."]
    #[inline] pub fn stmintset(&self) -> Stmintset { 
        unsafe {
            read_volatile(self.stmintset_ptr())
        }
    }

    #[doc="Write the STMINTSET register."]
    #[inline] pub fn set_stmintset<F: FnOnce(Stmintset) -> Stmintset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stmintset_mut(), f(Stmintset(0)));
        }
        self
    }

    #[doc="Modify the STMINTSET register."]
    #[inline] pub fn with_stmintset<F: FnOnce(Stmintset) -> Stmintset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stmintset_mut(), f(self.stmintset()));
        }
        self
    }

}

#[doc="Counter/Timer Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tmr0(pub u32);
impl Tmr0 {
    #[doc="Counter/Timer B0."]
    #[inline] pub fn cttmrb0(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if CTTMRB0 != 0"]
    #[inline] pub fn test_cttmrb0(&self) -> bool {
        self.cttmrb0() != 0
    }

    #[doc="Sets the CTTMRB0 field."]
    #[inline] pub fn set_cttmrb0<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Counter/Timer A0."]
    #[inline] pub fn cttmra0(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CTTMRA0 != 0"]
    #[inline] pub fn test_cttmra0(&self) -> bool {
        self.cttmra0() != 0
    }

    #[doc="Sets the CTTMRA0 field."]
    #[inline] pub fn set_cttmra0<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tmr0 {
    #[inline]
    fn from(other: u32) -> Self {
         Tmr0(other)
    }
}

impl ::core::fmt::Display for Tmr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tmr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cttmrb0() != 0 { try!(write!(f, " cttmrb0=0x{:x}", self.cttmrb0()))}
        if self.cttmra0() != 0 { try!(write!(f, " cttmra0=0x{:x}", self.cttmra0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter/Timer A0 Compare Registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cmpra0(pub u32);
impl Cmpra0 {
    #[doc="Counter/Timer A0 Compare Register 1. Holds the upper limit for timer half A."]
    #[inline] pub fn cmpr1a0(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if CMPR1A0 != 0"]
    #[inline] pub fn test_cmpr1a0(&self) -> bool {
        self.cmpr1a0() != 0
    }

    #[doc="Sets the CMPR1A0 field."]
    #[inline] pub fn set_cmpr1a0<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Counter/Timer A0 Compare Register 0. Holds the lower limit for timer half A."]
    #[inline] pub fn cmpr0a0(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CMPR0A0 != 0"]
    #[inline] pub fn test_cmpr0a0(&self) -> bool {
        self.cmpr0a0() != 0
    }

    #[doc="Sets the CMPR0A0 field."]
    #[inline] pub fn set_cmpr0a0<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cmpra0 {
    #[inline]
    fn from(other: u32) -> Self {
         Cmpra0(other)
    }
}

impl ::core::fmt::Display for Cmpra0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cmpra0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmpr1a0() != 0 { try!(write!(f, " cmpr1a0=0x{:x}", self.cmpr1a0()))}
        if self.cmpr0a0() != 0 { try!(write!(f, " cmpr0a0=0x{:x}", self.cmpr0a0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter/Timer B0 Compare Registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cmprb0(pub u32);
impl Cmprb0 {
    #[doc="Counter/Timer B0 Compare Register 1. Holds the upper limit for timer half B."]
    #[inline] pub fn cmpr1b0(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if CMPR1B0 != 0"]
    #[inline] pub fn test_cmpr1b0(&self) -> bool {
        self.cmpr1b0() != 0
    }

    #[doc="Sets the CMPR1B0 field."]
    #[inline] pub fn set_cmpr1b0<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Counter/Timer B0 Compare Register 0. Holds the lower limit for timer half B."]
    #[inline] pub fn cmpr0b0(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CMPR0B0 != 0"]
    #[inline] pub fn test_cmpr0b0(&self) -> bool {
        self.cmpr0b0() != 0
    }

    #[doc="Sets the CMPR0B0 field."]
    #[inline] pub fn set_cmpr0b0<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cmprb0 {
    #[inline]
    fn from(other: u32) -> Self {
         Cmprb0(other)
    }
}

impl ::core::fmt::Display for Cmprb0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cmprb0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmpr1b0() != 0 { try!(write!(f, " cmpr1b0=0x{:x}", self.cmpr1b0()))}
        if self.cmpr0b0() != 0 { try!(write!(f, " cmpr0b0=0x{:x}", self.cmpr0b0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter/Timer Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl0(pub u32);
impl Ctrl0 {
    #[doc="Counter/Timer A0/B0 Link bit."]
    #[inline] pub fn ctlink0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CTLINK0 != 0"]
    #[inline] pub fn test_ctlink0(&self) -> bool {
        self.ctlink0() != 0
    }

    #[doc="Sets the CTLINK0 field."]
    #[inline] pub fn set_ctlink0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Counter/Timer B0 Output Enable bit."]
    #[inline] pub fn tmrb0pe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if TMRB0PE != 0"]
    #[inline] pub fn test_tmrb0pe(&self) -> bool {
        self.tmrb0pe() != 0
    }

    #[doc="Sets the TMRB0PE field."]
    #[inline] pub fn set_tmrb0pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Counter/Timer B0 output polarity."]
    #[inline] pub fn tmrb0pol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if TMRB0POL != 0"]
    #[inline] pub fn test_tmrb0pol(&self) -> bool {
        self.tmrb0pol() != 0
    }

    #[doc="Sets the TMRB0POL field."]
    #[inline] pub fn set_tmrb0pol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Counter/Timer B0 Clear bit."]
    #[inline] pub fn tmrb0clr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if TMRB0CLR != 0"]
    #[inline] pub fn test_tmrb0clr(&self) -> bool {
        self.tmrb0clr() != 0
    }

    #[doc="Sets the TMRB0CLR field."]
    #[inline] pub fn set_tmrb0clr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Counter/Timer B0 Interrupt Enable bit for COMPR1."]
    #[inline] pub fn tmrb0ie1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if TMRB0IE1 != 0"]
    #[inline] pub fn test_tmrb0ie1(&self) -> bool {
        self.tmrb0ie1() != 0
    }

    #[doc="Sets the TMRB0IE1 field."]
    #[inline] pub fn set_tmrb0ie1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Counter/Timer B0 Interrupt Enable bit for COMPR0."]
    #[inline] pub fn tmrb0ie0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if TMRB0IE0 != 0"]
    #[inline] pub fn test_tmrb0ie0(&self) -> bool {
        self.tmrb0ie0() != 0
    }

    #[doc="Sets the TMRB0IE0 field."]
    #[inline] pub fn set_tmrb0ie0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Counter/Timer B0 Function Select."]
    #[inline] pub fn tmrb0fn(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x7) as u8) } // [24:22]
    }

    #[doc="Returns true if TMRB0FN != 0"]
    #[inline] pub fn test_tmrb0fn(&self) -> bool {
        self.tmrb0fn() != 0
    }

    #[doc="Sets the TMRB0FN field."]
    #[inline] pub fn set_tmrb0fn<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Counter/Timer B0 Clock Select."]
    #[inline] pub fn tmrb0clk(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1f) as u8) } // [21:17]
    }

    #[doc="Returns true if TMRB0CLK != 0"]
    #[inline] pub fn test_tmrb0clk(&self) -> bool {
        self.tmrb0clk() != 0
    }

    #[doc="Sets the TMRB0CLK field."]
    #[inline] pub fn set_tmrb0clk<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Counter/Timer B0 Enable bit."]
    #[inline] pub fn tmrb0en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TMRB0EN != 0"]
    #[inline] pub fn test_tmrb0en(&self) -> bool {
        self.tmrb0en() != 0
    }

    #[doc="Sets the TMRB0EN field."]
    #[inline] pub fn set_tmrb0en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Counter/Timer A0 Output Enable bit."]
    #[inline] pub fn tmra0pe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TMRA0PE != 0"]
    #[inline] pub fn test_tmra0pe(&self) -> bool {
        self.tmra0pe() != 0
    }

    #[doc="Sets the TMRA0PE field."]
    #[inline] pub fn set_tmra0pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Counter/Timer A0 output polarity."]
    #[inline] pub fn tmra0pol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TMRA0POL != 0"]
    #[inline] pub fn test_tmra0pol(&self) -> bool {
        self.tmra0pol() != 0
    }

    #[doc="Sets the TMRA0POL field."]
    #[inline] pub fn set_tmra0pol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Counter/Timer A0 Clear bit."]
    #[inline] pub fn tmra0clr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TMRA0CLR != 0"]
    #[inline] pub fn test_tmra0clr(&self) -> bool {
        self.tmra0clr() != 0
    }

    #[doc="Sets the TMRA0CLR field."]
    #[inline] pub fn set_tmra0clr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Counter/Timer A0 Interrupt Enable bit based on COMPR1."]
    #[inline] pub fn tmra0ie1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if TMRA0IE1 != 0"]
    #[inline] pub fn test_tmra0ie1(&self) -> bool {
        self.tmra0ie1() != 0
    }

    #[doc="Sets the TMRA0IE1 field."]
    #[inline] pub fn set_tmra0ie1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Counter/Timer A0 Interrupt Enable bit based on COMPR0."]
    #[inline] pub fn tmra0ie0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TMRA0IE0 != 0"]
    #[inline] pub fn test_tmra0ie0(&self) -> bool {
        self.tmra0ie0() != 0
    }

    #[doc="Sets the TMRA0IE0 field."]
    #[inline] pub fn set_tmra0ie0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Counter/Timer A0 Function Select."]
    #[inline] pub fn tmra0fn(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x7) as u8) } // [8:6]
    }

    #[doc="Returns true if TMRA0FN != 0"]
    #[inline] pub fn test_tmra0fn(&self) -> bool {
        self.tmra0fn() != 0
    }

    #[doc="Sets the TMRA0FN field."]
    #[inline] pub fn set_tmra0fn<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Counter/Timer A0 Clock Select."]
    #[inline] pub fn tmra0clk(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1f) as u8) } // [5:1]
    }

    #[doc="Returns true if TMRA0CLK != 0"]
    #[inline] pub fn test_tmra0clk(&self) -> bool {
        self.tmra0clk() != 0
    }

    #[doc="Sets the TMRA0CLK field."]
    #[inline] pub fn set_tmra0clk<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Counter/Timer A0 Enable bit."]
    #[inline] pub fn tmra0en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TMRA0EN != 0"]
    #[inline] pub fn test_tmra0en(&self) -> bool {
        self.tmra0en() != 0
    }

    #[doc="Sets the TMRA0EN field."]
    #[inline] pub fn set_tmra0en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ctrl0 {
    #[inline]
    fn from(other: u32) -> Self {
         Ctrl0(other)
    }
}

impl ::core::fmt::Display for Ctrl0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrl0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ctlink0() != 0 { try!(write!(f, " ctlink0"))}
        if self.tmrb0pe() != 0 { try!(write!(f, " tmrb0pe"))}
        if self.tmrb0pol() != 0 { try!(write!(f, " tmrb0pol"))}
        if self.tmrb0clr() != 0 { try!(write!(f, " tmrb0clr"))}
        if self.tmrb0ie1() != 0 { try!(write!(f, " tmrb0ie1"))}
        if self.tmrb0ie0() != 0 { try!(write!(f, " tmrb0ie0"))}
        if self.tmrb0fn() != 0 { try!(write!(f, " tmrb0fn=0x{:x}", self.tmrb0fn()))}
        if self.tmrb0clk() != 0 { try!(write!(f, " tmrb0clk=0x{:x}", self.tmrb0clk()))}
        if self.tmrb0en() != 0 { try!(write!(f, " tmrb0en"))}
        if self.tmra0pe() != 0 { try!(write!(f, " tmra0pe"))}
        if self.tmra0pol() != 0 { try!(write!(f, " tmra0pol"))}
        if self.tmra0clr() != 0 { try!(write!(f, " tmra0clr"))}
        if self.tmra0ie1() != 0 { try!(write!(f, " tmra0ie1"))}
        if self.tmra0ie0() != 0 { try!(write!(f, " tmra0ie0"))}
        if self.tmra0fn() != 0 { try!(write!(f, " tmra0fn=0x{:x}", self.tmra0fn()))}
        if self.tmra0clk() != 0 { try!(write!(f, " tmra0clk=0x{:x}", self.tmra0clk()))}
        if self.tmra0en() != 0 { try!(write!(f, " tmra0en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter/Timer Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tmr1(pub u32);
impl Tmr1 {
    #[doc="Counter/Timer B1."]
    #[inline] pub fn cttmrb1(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if CTTMRB1 != 0"]
    #[inline] pub fn test_cttmrb1(&self) -> bool {
        self.cttmrb1() != 0
    }

    #[doc="Sets the CTTMRB1 field."]
    #[inline] pub fn set_cttmrb1<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Counter/Timer A1."]
    #[inline] pub fn cttmra1(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CTTMRA1 != 0"]
    #[inline] pub fn test_cttmra1(&self) -> bool {
        self.cttmra1() != 0
    }

    #[doc="Sets the CTTMRA1 field."]
    #[inline] pub fn set_cttmra1<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tmr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Tmr1(other)
    }
}

impl ::core::fmt::Display for Tmr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tmr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cttmrb1() != 0 { try!(write!(f, " cttmrb1=0x{:x}", self.cttmrb1()))}
        if self.cttmra1() != 0 { try!(write!(f, " cttmra1=0x{:x}", self.cttmra1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter/Timer A1 Compare Registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cmpra1(pub u32);
impl Cmpra1 {
    #[doc="Counter/Timer A1 Compare Register 1."]
    #[inline] pub fn cmpr1a1(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if CMPR1A1 != 0"]
    #[inline] pub fn test_cmpr1a1(&self) -> bool {
        self.cmpr1a1() != 0
    }

    #[doc="Sets the CMPR1A1 field."]
    #[inline] pub fn set_cmpr1a1<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Counter/Timer A1 Compare Register 0."]
    #[inline] pub fn cmpr0a1(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CMPR0A1 != 0"]
    #[inline] pub fn test_cmpr0a1(&self) -> bool {
        self.cmpr0a1() != 0
    }

    #[doc="Sets the CMPR0A1 field."]
    #[inline] pub fn set_cmpr0a1<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cmpra1 {
    #[inline]
    fn from(other: u32) -> Self {
         Cmpra1(other)
    }
}

impl ::core::fmt::Display for Cmpra1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cmpra1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmpr1a1() != 0 { try!(write!(f, " cmpr1a1=0x{:x}", self.cmpr1a1()))}
        if self.cmpr0a1() != 0 { try!(write!(f, " cmpr0a1=0x{:x}", self.cmpr0a1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter/Timer B1 Compare Registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cmprb1(pub u32);
impl Cmprb1 {
    #[doc="Counter/Timer B1 Compare Register 1."]
    #[inline] pub fn cmpr1b1(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if CMPR1B1 != 0"]
    #[inline] pub fn test_cmpr1b1(&self) -> bool {
        self.cmpr1b1() != 0
    }

    #[doc="Sets the CMPR1B1 field."]
    #[inline] pub fn set_cmpr1b1<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Counter/Timer B1 Compare Register 0."]
    #[inline] pub fn cmpr0b1(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CMPR0B1 != 0"]
    #[inline] pub fn test_cmpr0b1(&self) -> bool {
        self.cmpr0b1() != 0
    }

    #[doc="Sets the CMPR0B1 field."]
    #[inline] pub fn set_cmpr0b1<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cmprb1 {
    #[inline]
    fn from(other: u32) -> Self {
         Cmprb1(other)
    }
}

impl ::core::fmt::Display for Cmprb1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cmprb1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmpr1b1() != 0 { try!(write!(f, " cmpr1b1=0x{:x}", self.cmpr1b1()))}
        if self.cmpr0b1() != 0 { try!(write!(f, " cmpr0b1=0x{:x}", self.cmpr0b1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter/Timer Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl1(pub u32);
impl Ctrl1 {
    #[doc="Counter/Timer A1/B1 Link bit."]
    #[inline] pub fn ctlink1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CTLINK1 != 0"]
    #[inline] pub fn test_ctlink1(&self) -> bool {
        self.ctlink1() != 0
    }

    #[doc="Sets the CTLINK1 field."]
    #[inline] pub fn set_ctlink1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Counter/Timer B1 Output Enable bit."]
    #[inline] pub fn tmrb1pe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if TMRB1PE != 0"]
    #[inline] pub fn test_tmrb1pe(&self) -> bool {
        self.tmrb1pe() != 0
    }

    #[doc="Sets the TMRB1PE field."]
    #[inline] pub fn set_tmrb1pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Counter/Timer B1 output polarity."]
    #[inline] pub fn tmrb1pol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if TMRB1POL != 0"]
    #[inline] pub fn test_tmrb1pol(&self) -> bool {
        self.tmrb1pol() != 0
    }

    #[doc="Sets the TMRB1POL field."]
    #[inline] pub fn set_tmrb1pol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Counter/Timer B1 Clear bit."]
    #[inline] pub fn tmrb1clr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if TMRB1CLR != 0"]
    #[inline] pub fn test_tmrb1clr(&self) -> bool {
        self.tmrb1clr() != 0
    }

    #[doc="Sets the TMRB1CLR field."]
    #[inline] pub fn set_tmrb1clr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Counter/Timer B1 Interrupt Enable bit for COMPR1."]
    #[inline] pub fn tmrb1ie1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if TMRB1IE1 != 0"]
    #[inline] pub fn test_tmrb1ie1(&self) -> bool {
        self.tmrb1ie1() != 0
    }

    #[doc="Sets the TMRB1IE1 field."]
    #[inline] pub fn set_tmrb1ie1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Counter/Timer B1 Interrupt Enable bit for COMPR0."]
    #[inline] pub fn tmrb1ie0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if TMRB1IE0 != 0"]
    #[inline] pub fn test_tmrb1ie0(&self) -> bool {
        self.tmrb1ie0() != 0
    }

    #[doc="Sets the TMRB1IE0 field."]
    #[inline] pub fn set_tmrb1ie0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Counter/Timer B1 Function Select."]
    #[inline] pub fn tmrb1fn(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x7) as u8) } // [24:22]
    }

    #[doc="Returns true if TMRB1FN != 0"]
    #[inline] pub fn test_tmrb1fn(&self) -> bool {
        self.tmrb1fn() != 0
    }

    #[doc="Sets the TMRB1FN field."]
    #[inline] pub fn set_tmrb1fn<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Counter/Timer B1 Clock Select."]
    #[inline] pub fn tmrb1clk(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1f) as u8) } // [21:17]
    }

    #[doc="Returns true if TMRB1CLK != 0"]
    #[inline] pub fn test_tmrb1clk(&self) -> bool {
        self.tmrb1clk() != 0
    }

    #[doc="Sets the TMRB1CLK field."]
    #[inline] pub fn set_tmrb1clk<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Counter/Timer B1 Enable bit."]
    #[inline] pub fn tmrb1en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TMRB1EN != 0"]
    #[inline] pub fn test_tmrb1en(&self) -> bool {
        self.tmrb1en() != 0
    }

    #[doc="Sets the TMRB1EN field."]
    #[inline] pub fn set_tmrb1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Counter/Timer A1 Output Enable bit."]
    #[inline] pub fn tmra1pe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TMRA1PE != 0"]
    #[inline] pub fn test_tmra1pe(&self) -> bool {
        self.tmra1pe() != 0
    }

    #[doc="Sets the TMRA1PE field."]
    #[inline] pub fn set_tmra1pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Counter/Timer A1 output polarity."]
    #[inline] pub fn tmra1pol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TMRA1POL != 0"]
    #[inline] pub fn test_tmra1pol(&self) -> bool {
        self.tmra1pol() != 0
    }

    #[doc="Sets the TMRA1POL field."]
    #[inline] pub fn set_tmra1pol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Counter/Timer A1 Clear bit."]
    #[inline] pub fn tmra1clr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TMRA1CLR != 0"]
    #[inline] pub fn test_tmra1clr(&self) -> bool {
        self.tmra1clr() != 0
    }

    #[doc="Sets the TMRA1CLR field."]
    #[inline] pub fn set_tmra1clr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Counter/Timer A1 Interrupt Enable bit based on COMPR1."]
    #[inline] pub fn tmra1ie1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if TMRA1IE1 != 0"]
    #[inline] pub fn test_tmra1ie1(&self) -> bool {
        self.tmra1ie1() != 0
    }

    #[doc="Sets the TMRA1IE1 field."]
    #[inline] pub fn set_tmra1ie1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Counter/Timer A1 Interrupt Enable bit based on COMPR0."]
    #[inline] pub fn tmra1ie0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TMRA1IE0 != 0"]
    #[inline] pub fn test_tmra1ie0(&self) -> bool {
        self.tmra1ie0() != 0
    }

    #[doc="Sets the TMRA1IE0 field."]
    #[inline] pub fn set_tmra1ie0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Counter/Timer A1 Function Select."]
    #[inline] pub fn tmra1fn(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x7) as u8) } // [8:6]
    }

    #[doc="Returns true if TMRA1FN != 0"]
    #[inline] pub fn test_tmra1fn(&self) -> bool {
        self.tmra1fn() != 0
    }

    #[doc="Sets the TMRA1FN field."]
    #[inline] pub fn set_tmra1fn<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Counter/Timer A1 Clock Select."]
    #[inline] pub fn tmra1clk(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1f) as u8) } // [5:1]
    }

    #[doc="Returns true if TMRA1CLK != 0"]
    #[inline] pub fn test_tmra1clk(&self) -> bool {
        self.tmra1clk() != 0
    }

    #[doc="Sets the TMRA1CLK field."]
    #[inline] pub fn set_tmra1clk<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Counter/Timer A1 Enable bit."]
    #[inline] pub fn tmra1en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TMRA1EN != 0"]
    #[inline] pub fn test_tmra1en(&self) -> bool {
        self.tmra1en() != 0
    }

    #[doc="Sets the TMRA1EN field."]
    #[inline] pub fn set_tmra1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ctrl1 {
    #[inline]
    fn from(other: u32) -> Self {
         Ctrl1(other)
    }
}

impl ::core::fmt::Display for Ctrl1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrl1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ctlink1() != 0 { try!(write!(f, " ctlink1"))}
        if self.tmrb1pe() != 0 { try!(write!(f, " tmrb1pe"))}
        if self.tmrb1pol() != 0 { try!(write!(f, " tmrb1pol"))}
        if self.tmrb1clr() != 0 { try!(write!(f, " tmrb1clr"))}
        if self.tmrb1ie1() != 0 { try!(write!(f, " tmrb1ie1"))}
        if self.tmrb1ie0() != 0 { try!(write!(f, " tmrb1ie0"))}
        if self.tmrb1fn() != 0 { try!(write!(f, " tmrb1fn=0x{:x}", self.tmrb1fn()))}
        if self.tmrb1clk() != 0 { try!(write!(f, " tmrb1clk=0x{:x}", self.tmrb1clk()))}
        if self.tmrb1en() != 0 { try!(write!(f, " tmrb1en"))}
        if self.tmra1pe() != 0 { try!(write!(f, " tmra1pe"))}
        if self.tmra1pol() != 0 { try!(write!(f, " tmra1pol"))}
        if self.tmra1clr() != 0 { try!(write!(f, " tmra1clr"))}
        if self.tmra1ie1() != 0 { try!(write!(f, " tmra1ie1"))}
        if self.tmra1ie0() != 0 { try!(write!(f, " tmra1ie0"))}
        if self.tmra1fn() != 0 { try!(write!(f, " tmra1fn=0x{:x}", self.tmra1fn()))}
        if self.tmra1clk() != 0 { try!(write!(f, " tmra1clk=0x{:x}", self.tmra1clk()))}
        if self.tmra1en() != 0 { try!(write!(f, " tmra1en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter/Timer Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tmr2(pub u32);
impl Tmr2 {
    #[doc="Counter/Timer B2."]
    #[inline] pub fn cttmrb2(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if CTTMRB2 != 0"]
    #[inline] pub fn test_cttmrb2(&self) -> bool {
        self.cttmrb2() != 0
    }

    #[doc="Sets the CTTMRB2 field."]
    #[inline] pub fn set_cttmrb2<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Counter/Timer A2."]
    #[inline] pub fn cttmra2(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CTTMRA2 != 0"]
    #[inline] pub fn test_cttmra2(&self) -> bool {
        self.cttmra2() != 0
    }

    #[doc="Sets the CTTMRA2 field."]
    #[inline] pub fn set_cttmra2<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tmr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Tmr2(other)
    }
}

impl ::core::fmt::Display for Tmr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tmr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cttmrb2() != 0 { try!(write!(f, " cttmrb2=0x{:x}", self.cttmrb2()))}
        if self.cttmra2() != 0 { try!(write!(f, " cttmra2=0x{:x}", self.cttmra2()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter/Timer A2 Compare Registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cmpra2(pub u32);
impl Cmpra2 {
    #[doc="Counter/Timer A2 Compare Register 1."]
    #[inline] pub fn cmpr1a2(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if CMPR1A2 != 0"]
    #[inline] pub fn test_cmpr1a2(&self) -> bool {
        self.cmpr1a2() != 0
    }

    #[doc="Sets the CMPR1A2 field."]
    #[inline] pub fn set_cmpr1a2<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Counter/Timer A2 Compare Register 0."]
    #[inline] pub fn cmpr0a2(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CMPR0A2 != 0"]
    #[inline] pub fn test_cmpr0a2(&self) -> bool {
        self.cmpr0a2() != 0
    }

    #[doc="Sets the CMPR0A2 field."]
    #[inline] pub fn set_cmpr0a2<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cmpra2 {
    #[inline]
    fn from(other: u32) -> Self {
         Cmpra2(other)
    }
}

impl ::core::fmt::Display for Cmpra2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cmpra2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmpr1a2() != 0 { try!(write!(f, " cmpr1a2=0x{:x}", self.cmpr1a2()))}
        if self.cmpr0a2() != 0 { try!(write!(f, " cmpr0a2=0x{:x}", self.cmpr0a2()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter/Timer B2 Compare Registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cmprb2(pub u32);
impl Cmprb2 {
    #[doc="Counter/Timer B2 Compare Register 1."]
    #[inline] pub fn cmpr1b2(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if CMPR1B2 != 0"]
    #[inline] pub fn test_cmpr1b2(&self) -> bool {
        self.cmpr1b2() != 0
    }

    #[doc="Sets the CMPR1B2 field."]
    #[inline] pub fn set_cmpr1b2<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Counter/Timer B2 Compare Register 0."]
    #[inline] pub fn cmpr0b2(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CMPR0B2 != 0"]
    #[inline] pub fn test_cmpr0b2(&self) -> bool {
        self.cmpr0b2() != 0
    }

    #[doc="Sets the CMPR0B2 field."]
    #[inline] pub fn set_cmpr0b2<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cmprb2 {
    #[inline]
    fn from(other: u32) -> Self {
         Cmprb2(other)
    }
}

impl ::core::fmt::Display for Cmprb2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cmprb2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmpr1b2() != 0 { try!(write!(f, " cmpr1b2=0x{:x}", self.cmpr1b2()))}
        if self.cmpr0b2() != 0 { try!(write!(f, " cmpr0b2=0x{:x}", self.cmpr0b2()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter/Timer Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl2(pub u32);
impl Ctrl2 {
    #[doc="Counter/Timer A2/B2 Link bit."]
    #[inline] pub fn ctlink2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CTLINK2 != 0"]
    #[inline] pub fn test_ctlink2(&self) -> bool {
        self.ctlink2() != 0
    }

    #[doc="Sets the CTLINK2 field."]
    #[inline] pub fn set_ctlink2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Counter/Timer B2 Output Enable bit."]
    #[inline] pub fn tmrb2pe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if TMRB2PE != 0"]
    #[inline] pub fn test_tmrb2pe(&self) -> bool {
        self.tmrb2pe() != 0
    }

    #[doc="Sets the TMRB2PE field."]
    #[inline] pub fn set_tmrb2pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Counter/Timer B2 output polarity."]
    #[inline] pub fn tmrb2pol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if TMRB2POL != 0"]
    #[inline] pub fn test_tmrb2pol(&self) -> bool {
        self.tmrb2pol() != 0
    }

    #[doc="Sets the TMRB2POL field."]
    #[inline] pub fn set_tmrb2pol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Counter/Timer B2 Clear bit."]
    #[inline] pub fn tmrb2clr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if TMRB2CLR != 0"]
    #[inline] pub fn test_tmrb2clr(&self) -> bool {
        self.tmrb2clr() != 0
    }

    #[doc="Sets the TMRB2CLR field."]
    #[inline] pub fn set_tmrb2clr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Counter/Timer B2 Interrupt Enable bit for COMPR1."]
    #[inline] pub fn tmrb2ie1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if TMRB2IE1 != 0"]
    #[inline] pub fn test_tmrb2ie1(&self) -> bool {
        self.tmrb2ie1() != 0
    }

    #[doc="Sets the TMRB2IE1 field."]
    #[inline] pub fn set_tmrb2ie1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Counter/Timer B2 Interrupt Enable bit for COMPR0."]
    #[inline] pub fn tmrb2ie0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if TMRB2IE0 != 0"]
    #[inline] pub fn test_tmrb2ie0(&self) -> bool {
        self.tmrb2ie0() != 0
    }

    #[doc="Sets the TMRB2IE0 field."]
    #[inline] pub fn set_tmrb2ie0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Counter/Timer B2 Function Select."]
    #[inline] pub fn tmrb2fn(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x7) as u8) } // [24:22]
    }

    #[doc="Returns true if TMRB2FN != 0"]
    #[inline] pub fn test_tmrb2fn(&self) -> bool {
        self.tmrb2fn() != 0
    }

    #[doc="Sets the TMRB2FN field."]
    #[inline] pub fn set_tmrb2fn<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Counter/Timer B2 Clock Select."]
    #[inline] pub fn tmrb2clk(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1f) as u8) } // [21:17]
    }

    #[doc="Returns true if TMRB2CLK != 0"]
    #[inline] pub fn test_tmrb2clk(&self) -> bool {
        self.tmrb2clk() != 0
    }

    #[doc="Sets the TMRB2CLK field."]
    #[inline] pub fn set_tmrb2clk<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Counter/Timer B2 Enable bit."]
    #[inline] pub fn tmrb2en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TMRB2EN != 0"]
    #[inline] pub fn test_tmrb2en(&self) -> bool {
        self.tmrb2en() != 0
    }

    #[doc="Sets the TMRB2EN field."]
    #[inline] pub fn set_tmrb2en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Counter/Timer A2 Output Enable bit."]
    #[inline] pub fn tmra2pe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TMRA2PE != 0"]
    #[inline] pub fn test_tmra2pe(&self) -> bool {
        self.tmra2pe() != 0
    }

    #[doc="Sets the TMRA2PE field."]
    #[inline] pub fn set_tmra2pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Counter/Timer A2 output polarity."]
    #[inline] pub fn tmra2pol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TMRA2POL != 0"]
    #[inline] pub fn test_tmra2pol(&self) -> bool {
        self.tmra2pol() != 0
    }

    #[doc="Sets the TMRA2POL field."]
    #[inline] pub fn set_tmra2pol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Counter/Timer A2 Clear bit."]
    #[inline] pub fn tmra2clr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TMRA2CLR != 0"]
    #[inline] pub fn test_tmra2clr(&self) -> bool {
        self.tmra2clr() != 0
    }

    #[doc="Sets the TMRA2CLR field."]
    #[inline] pub fn set_tmra2clr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Counter/Timer A2 Interrupt Enable bit based on COMPR1."]
    #[inline] pub fn tmra2ie1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if TMRA2IE1 != 0"]
    #[inline] pub fn test_tmra2ie1(&self) -> bool {
        self.tmra2ie1() != 0
    }

    #[doc="Sets the TMRA2IE1 field."]
    #[inline] pub fn set_tmra2ie1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Counter/Timer A2 Interrupt Enable bit based on COMPR0."]
    #[inline] pub fn tmra2ie0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TMRA2IE0 != 0"]
    #[inline] pub fn test_tmra2ie0(&self) -> bool {
        self.tmra2ie0() != 0
    }

    #[doc="Sets the TMRA2IE0 field."]
    #[inline] pub fn set_tmra2ie0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Counter/Timer A2 Function Select."]
    #[inline] pub fn tmra2fn(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x7) as u8) } // [8:6]
    }

    #[doc="Returns true if TMRA2FN != 0"]
    #[inline] pub fn test_tmra2fn(&self) -> bool {
        self.tmra2fn() != 0
    }

    #[doc="Sets the TMRA2FN field."]
    #[inline] pub fn set_tmra2fn<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Counter/Timer A2 Clock Select."]
    #[inline] pub fn tmra2clk(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1f) as u8) } // [5:1]
    }

    #[doc="Returns true if TMRA2CLK != 0"]
    #[inline] pub fn test_tmra2clk(&self) -> bool {
        self.tmra2clk() != 0
    }

    #[doc="Sets the TMRA2CLK field."]
    #[inline] pub fn set_tmra2clk<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Counter/Timer A2 Enable bit."]
    #[inline] pub fn tmra2en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TMRA2EN != 0"]
    #[inline] pub fn test_tmra2en(&self) -> bool {
        self.tmra2en() != 0
    }

    #[doc="Sets the TMRA2EN field."]
    #[inline] pub fn set_tmra2en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ctrl2 {
    #[inline]
    fn from(other: u32) -> Self {
         Ctrl2(other)
    }
}

impl ::core::fmt::Display for Ctrl2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrl2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ctlink2() != 0 { try!(write!(f, " ctlink2"))}
        if self.tmrb2pe() != 0 { try!(write!(f, " tmrb2pe"))}
        if self.tmrb2pol() != 0 { try!(write!(f, " tmrb2pol"))}
        if self.tmrb2clr() != 0 { try!(write!(f, " tmrb2clr"))}
        if self.tmrb2ie1() != 0 { try!(write!(f, " tmrb2ie1"))}
        if self.tmrb2ie0() != 0 { try!(write!(f, " tmrb2ie0"))}
        if self.tmrb2fn() != 0 { try!(write!(f, " tmrb2fn=0x{:x}", self.tmrb2fn()))}
        if self.tmrb2clk() != 0 { try!(write!(f, " tmrb2clk=0x{:x}", self.tmrb2clk()))}
        if self.tmrb2en() != 0 { try!(write!(f, " tmrb2en"))}
        if self.tmra2pe() != 0 { try!(write!(f, " tmra2pe"))}
        if self.tmra2pol() != 0 { try!(write!(f, " tmra2pol"))}
        if self.tmra2clr() != 0 { try!(write!(f, " tmra2clr"))}
        if self.tmra2ie1() != 0 { try!(write!(f, " tmra2ie1"))}
        if self.tmra2ie0() != 0 { try!(write!(f, " tmra2ie0"))}
        if self.tmra2fn() != 0 { try!(write!(f, " tmra2fn=0x{:x}", self.tmra2fn()))}
        if self.tmra2clk() != 0 { try!(write!(f, " tmra2clk=0x{:x}", self.tmra2clk()))}
        if self.tmra2en() != 0 { try!(write!(f, " tmra2en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter/Timer Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tmr3(pub u32);
impl Tmr3 {
    #[doc="Counter/Timer B3."]
    #[inline] pub fn cttmrb3(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if CTTMRB3 != 0"]
    #[inline] pub fn test_cttmrb3(&self) -> bool {
        self.cttmrb3() != 0
    }

    #[doc="Sets the CTTMRB3 field."]
    #[inline] pub fn set_cttmrb3<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Counter/Timer A3."]
    #[inline] pub fn cttmra3(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CTTMRA3 != 0"]
    #[inline] pub fn test_cttmra3(&self) -> bool {
        self.cttmra3() != 0
    }

    #[doc="Sets the CTTMRA3 field."]
    #[inline] pub fn set_cttmra3<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tmr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Tmr3(other)
    }
}

impl ::core::fmt::Display for Tmr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tmr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cttmrb3() != 0 { try!(write!(f, " cttmrb3=0x{:x}", self.cttmrb3()))}
        if self.cttmra3() != 0 { try!(write!(f, " cttmra3=0x{:x}", self.cttmra3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter/Timer A3 Compare Registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cmpra3(pub u32);
impl Cmpra3 {
    #[doc="Counter/Timer A3 Compare Register 1."]
    #[inline] pub fn cmpr1a3(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if CMPR1A3 != 0"]
    #[inline] pub fn test_cmpr1a3(&self) -> bool {
        self.cmpr1a3() != 0
    }

    #[doc="Sets the CMPR1A3 field."]
    #[inline] pub fn set_cmpr1a3<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Counter/Timer A3 Compare Register 0."]
    #[inline] pub fn cmpr0a3(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CMPR0A3 != 0"]
    #[inline] pub fn test_cmpr0a3(&self) -> bool {
        self.cmpr0a3() != 0
    }

    #[doc="Sets the CMPR0A3 field."]
    #[inline] pub fn set_cmpr0a3<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cmpra3 {
    #[inline]
    fn from(other: u32) -> Self {
         Cmpra3(other)
    }
}

impl ::core::fmt::Display for Cmpra3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cmpra3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmpr1a3() != 0 { try!(write!(f, " cmpr1a3=0x{:x}", self.cmpr1a3()))}
        if self.cmpr0a3() != 0 { try!(write!(f, " cmpr0a3=0x{:x}", self.cmpr0a3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter/Timer B3 Compare Registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cmprb3(pub u32);
impl Cmprb3 {
    #[doc="Counter/Timer B3 Compare Register 1."]
    #[inline] pub fn cmpr1b3(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if CMPR1B3 != 0"]
    #[inline] pub fn test_cmpr1b3(&self) -> bool {
        self.cmpr1b3() != 0
    }

    #[doc="Sets the CMPR1B3 field."]
    #[inline] pub fn set_cmpr1b3<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Counter/Timer B3 Compare Register 0."]
    #[inline] pub fn cmpr0b3(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CMPR0B3 != 0"]
    #[inline] pub fn test_cmpr0b3(&self) -> bool {
        self.cmpr0b3() != 0
    }

    #[doc="Sets the CMPR0B3 field."]
    #[inline] pub fn set_cmpr0b3<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cmprb3 {
    #[inline]
    fn from(other: u32) -> Self {
         Cmprb3(other)
    }
}

impl ::core::fmt::Display for Cmprb3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cmprb3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmpr1b3() != 0 { try!(write!(f, " cmpr1b3=0x{:x}", self.cmpr1b3()))}
        if self.cmpr0b3() != 0 { try!(write!(f, " cmpr0b3=0x{:x}", self.cmpr0b3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter/Timer Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl3(pub u32);
impl Ctrl3 {
    #[doc="Counter/Timer A3/B3 Link bit."]
    #[inline] pub fn ctlink3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CTLINK3 != 0"]
    #[inline] pub fn test_ctlink3(&self) -> bool {
        self.ctlink3() != 0
    }

    #[doc="Sets the CTLINK3 field."]
    #[inline] pub fn set_ctlink3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Counter/Timer B3 Output Enable bit."]
    #[inline] pub fn tmrb3pe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if TMRB3PE != 0"]
    #[inline] pub fn test_tmrb3pe(&self) -> bool {
        self.tmrb3pe() != 0
    }

    #[doc="Sets the TMRB3PE field."]
    #[inline] pub fn set_tmrb3pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Counter/Timer B3 output polarity."]
    #[inline] pub fn tmrb3pol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if TMRB3POL != 0"]
    #[inline] pub fn test_tmrb3pol(&self) -> bool {
        self.tmrb3pol() != 0
    }

    #[doc="Sets the TMRB3POL field."]
    #[inline] pub fn set_tmrb3pol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Counter/Timer B3 Clear bit."]
    #[inline] pub fn tmrb3clr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if TMRB3CLR != 0"]
    #[inline] pub fn test_tmrb3clr(&self) -> bool {
        self.tmrb3clr() != 0
    }

    #[doc="Sets the TMRB3CLR field."]
    #[inline] pub fn set_tmrb3clr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Counter/Timer B3 Interrupt Enable bit for COMPR1."]
    #[inline] pub fn tmrb3ie1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if TMRB3IE1 != 0"]
    #[inline] pub fn test_tmrb3ie1(&self) -> bool {
        self.tmrb3ie1() != 0
    }

    #[doc="Sets the TMRB3IE1 field."]
    #[inline] pub fn set_tmrb3ie1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Counter/Timer B3 Interrupt Enable bit for COMPR0."]
    #[inline] pub fn tmrb3ie0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if TMRB3IE0 != 0"]
    #[inline] pub fn test_tmrb3ie0(&self) -> bool {
        self.tmrb3ie0() != 0
    }

    #[doc="Sets the TMRB3IE0 field."]
    #[inline] pub fn set_tmrb3ie0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Counter/Timer B3 Function Select."]
    #[inline] pub fn tmrb3fn(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x7) as u8) } // [24:22]
    }

    #[doc="Returns true if TMRB3FN != 0"]
    #[inline] pub fn test_tmrb3fn(&self) -> bool {
        self.tmrb3fn() != 0
    }

    #[doc="Sets the TMRB3FN field."]
    #[inline] pub fn set_tmrb3fn<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Counter/Timer B3 Clock Select."]
    #[inline] pub fn tmrb3clk(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1f) as u8) } // [21:17]
    }

    #[doc="Returns true if TMRB3CLK != 0"]
    #[inline] pub fn test_tmrb3clk(&self) -> bool {
        self.tmrb3clk() != 0
    }

    #[doc="Sets the TMRB3CLK field."]
    #[inline] pub fn set_tmrb3clk<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Counter/Timer B3 Enable bit."]
    #[inline] pub fn tmrb3en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TMRB3EN != 0"]
    #[inline] pub fn test_tmrb3en(&self) -> bool {
        self.tmrb3en() != 0
    }

    #[doc="Sets the TMRB3EN field."]
    #[inline] pub fn set_tmrb3en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Special Timer A3 enable for ADC function."]
    #[inline] pub fn adcen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if ADCEN != 0"]
    #[inline] pub fn test_adcen(&self) -> bool {
        self.adcen() != 0
    }

    #[doc="Sets the ADCEN field."]
    #[inline] pub fn set_adcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Counter/Timer A3 Output Enable bit."]
    #[inline] pub fn tmra3pe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TMRA3PE != 0"]
    #[inline] pub fn test_tmra3pe(&self) -> bool {
        self.tmra3pe() != 0
    }

    #[doc="Sets the TMRA3PE field."]
    #[inline] pub fn set_tmra3pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Counter/Timer A3 output polarity."]
    #[inline] pub fn tmra3pol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TMRA3POL != 0"]
    #[inline] pub fn test_tmra3pol(&self) -> bool {
        self.tmra3pol() != 0
    }

    #[doc="Sets the TMRA3POL field."]
    #[inline] pub fn set_tmra3pol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Counter/Timer A3 Clear bit."]
    #[inline] pub fn tmra3clr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TMRA3CLR != 0"]
    #[inline] pub fn test_tmra3clr(&self) -> bool {
        self.tmra3clr() != 0
    }

    #[doc="Sets the TMRA3CLR field."]
    #[inline] pub fn set_tmra3clr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Counter/Timer A3 Interrupt Enable bit based on COMPR1."]
    #[inline] pub fn tmra3ie1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if TMRA3IE1 != 0"]
    #[inline] pub fn test_tmra3ie1(&self) -> bool {
        self.tmra3ie1() != 0
    }

    #[doc="Sets the TMRA3IE1 field."]
    #[inline] pub fn set_tmra3ie1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Counter/Timer A3 Interrupt Enable bit based on COMPR0."]
    #[inline] pub fn tmra3ie0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TMRA3IE0 != 0"]
    #[inline] pub fn test_tmra3ie0(&self) -> bool {
        self.tmra3ie0() != 0
    }

    #[doc="Sets the TMRA3IE0 field."]
    #[inline] pub fn set_tmra3ie0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Counter/Timer A3 Function Select."]
    #[inline] pub fn tmra3fn(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x7) as u8) } // [8:6]
    }

    #[doc="Returns true if TMRA3FN != 0"]
    #[inline] pub fn test_tmra3fn(&self) -> bool {
        self.tmra3fn() != 0
    }

    #[doc="Sets the TMRA3FN field."]
    #[inline] pub fn set_tmra3fn<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Counter/Timer A3 Clock Select."]
    #[inline] pub fn tmra3clk(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1f) as u8) } // [5:1]
    }

    #[doc="Returns true if TMRA3CLK != 0"]
    #[inline] pub fn test_tmra3clk(&self) -> bool {
        self.tmra3clk() != 0
    }

    #[doc="Sets the TMRA3CLK field."]
    #[inline] pub fn set_tmra3clk<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Counter/Timer A3 Enable bit."]
    #[inline] pub fn tmra3en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TMRA3EN != 0"]
    #[inline] pub fn test_tmra3en(&self) -> bool {
        self.tmra3en() != 0
    }

    #[doc="Sets the TMRA3EN field."]
    #[inline] pub fn set_tmra3en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ctrl3 {
    #[inline]
    fn from(other: u32) -> Self {
         Ctrl3(other)
    }
}

impl ::core::fmt::Display for Ctrl3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrl3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ctlink3() != 0 { try!(write!(f, " ctlink3"))}
        if self.tmrb3pe() != 0 { try!(write!(f, " tmrb3pe"))}
        if self.tmrb3pol() != 0 { try!(write!(f, " tmrb3pol"))}
        if self.tmrb3clr() != 0 { try!(write!(f, " tmrb3clr"))}
        if self.tmrb3ie1() != 0 { try!(write!(f, " tmrb3ie1"))}
        if self.tmrb3ie0() != 0 { try!(write!(f, " tmrb3ie0"))}
        if self.tmrb3fn() != 0 { try!(write!(f, " tmrb3fn=0x{:x}", self.tmrb3fn()))}
        if self.tmrb3clk() != 0 { try!(write!(f, " tmrb3clk=0x{:x}", self.tmrb3clk()))}
        if self.tmrb3en() != 0 { try!(write!(f, " tmrb3en"))}
        if self.adcen() != 0 { try!(write!(f, " adcen"))}
        if self.tmra3pe() != 0 { try!(write!(f, " tmra3pe"))}
        if self.tmra3pol() != 0 { try!(write!(f, " tmra3pol"))}
        if self.tmra3clr() != 0 { try!(write!(f, " tmra3clr"))}
        if self.tmra3ie1() != 0 { try!(write!(f, " tmra3ie1"))}
        if self.tmra3ie0() != 0 { try!(write!(f, " tmra3ie0"))}
        if self.tmra3fn() != 0 { try!(write!(f, " tmra3fn=0x{:x}", self.tmra3fn()))}
        if self.tmra3clk() != 0 { try!(write!(f, " tmra3clk=0x{:x}", self.tmra3clk()))}
        if self.tmra3en() != 0 { try!(write!(f, " tmra3en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stcfg(pub u32);
impl Stcfg {
    #[doc="Set this bit to one to freeze the clock input to the COUNTER register. Once frozen, the value can be safely written from the MCU. Unfreeze to resume."]
    #[inline] pub fn freeze(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if FREEZE != 0"]
    #[inline] pub fn test_freeze(&self) -> bool {
        self.freeze() != 0
    }

    #[doc="Sets the FREEZE field."]
    #[inline] pub fn set_freeze<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Set this bit to one to clear the System Timer register. If this bit is set to \'1\', the system timer register will stay cleared. It needs to be set to \'0\' for the system timer to start running."]
    #[inline] pub fn clear(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CLEAR != 0"]
    #[inline] pub fn test_clear(&self) -> bool {
        self.clear() != 0
    }

    #[doc="Sets the CLEAR field."]
    #[inline] pub fn set_clear<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline] pub fn compare_h_en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if COMPARE_H_EN != 0"]
    #[inline] pub fn test_compare_h_en(&self) -> bool {
        self.compare_h_en() != 0
    }

    #[doc="Sets the COMPARE_H_EN field."]
    #[inline] pub fn set_compare_h_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline] pub fn compare_g_en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if COMPARE_G_EN != 0"]
    #[inline] pub fn test_compare_g_en(&self) -> bool {
        self.compare_g_en() != 0
    }

    #[doc="Sets the COMPARE_G_EN field."]
    #[inline] pub fn set_compare_g_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline] pub fn compare_f_en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if COMPARE_F_EN != 0"]
    #[inline] pub fn test_compare_f_en(&self) -> bool {
        self.compare_f_en() != 0
    }

    #[doc="Sets the COMPARE_F_EN field."]
    #[inline] pub fn set_compare_f_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline] pub fn compare_e_en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if COMPARE_E_EN != 0"]
    #[inline] pub fn test_compare_e_en(&self) -> bool {
        self.compare_e_en() != 0
    }

    #[doc="Sets the COMPARE_E_EN field."]
    #[inline] pub fn set_compare_e_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline] pub fn compare_d_en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if COMPARE_D_EN != 0"]
    #[inline] pub fn test_compare_d_en(&self) -> bool {
        self.compare_d_en() != 0
    }

    #[doc="Sets the COMPARE_D_EN field."]
    #[inline] pub fn set_compare_d_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline] pub fn compare_c_en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if COMPARE_C_EN != 0"]
    #[inline] pub fn test_compare_c_en(&self) -> bool {
        self.compare_c_en() != 0
    }

    #[doc="Sets the COMPARE_C_EN field."]
    #[inline] pub fn set_compare_c_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline] pub fn compare_b_en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if COMPARE_B_EN != 0"]
    #[inline] pub fn test_compare_b_en(&self) -> bool {
        self.compare_b_en() != 0
    }

    #[doc="Sets the COMPARE_B_EN field."]
    #[inline] pub fn set_compare_b_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline] pub fn compare_a_en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if COMPARE_A_EN != 0"]
    #[inline] pub fn test_compare_a_en(&self) -> bool {
        self.compare_a_en() != 0
    }

    #[doc="Sets the COMPARE_A_EN field."]
    #[inline] pub fn set_compare_a_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Selects an appropriate clock source and divider to use for the System Timer clock."]
    #[inline] pub fn clksel(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CLKSEL != 0"]
    #[inline] pub fn test_clksel(&self) -> bool {
        self.clksel() != 0
    }

    #[doc="Sets the CLKSEL field."]
    #[inline] pub fn set_clksel<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Stcfg {
    #[inline]
    fn from(other: u32) -> Self {
         Stcfg(other)
    }
}

impl ::core::fmt::Display for Stcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Stcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.freeze() != 0 { try!(write!(f, " freeze"))}
        if self.clear() != 0 { try!(write!(f, " clear"))}
        if self.compare_h_en() != 0 { try!(write!(f, " compare_h_en"))}
        if self.compare_g_en() != 0 { try!(write!(f, " compare_g_en"))}
        if self.compare_f_en() != 0 { try!(write!(f, " compare_f_en"))}
        if self.compare_e_en() != 0 { try!(write!(f, " compare_e_en"))}
        if self.compare_d_en() != 0 { try!(write!(f, " compare_d_en"))}
        if self.compare_c_en() != 0 { try!(write!(f, " compare_c_en"))}
        if self.compare_b_en() != 0 { try!(write!(f, " compare_b_en"))}
        if self.compare_a_en() != 0 { try!(write!(f, " compare_a_en"))}
        if self.clksel() != 0 { try!(write!(f, " clksel=0x{:x}", self.clksel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Timer Count Register (Real Time Counter)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sttmr(pub u32);
impl Sttmr {
    #[doc="Value of the 32-bit counter as it ticks over."]
    #[inline] pub fn value(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sttmr {
    #[inline]
    fn from(other: u32) -> Self {
         Sttmr(other)
    }
}

impl ::core::fmt::Display for Sttmr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sttmr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Capture Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct CaptureControl(pub u32);
impl CaptureControl {
    #[doc="Selects whether capture is enabled for the specified capture register."]
    #[inline] pub fn capture_d(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CAPTURE_D != 0"]
    #[inline] pub fn test_capture_d(&self) -> bool {
        self.capture_d() != 0
    }

    #[doc="Sets the CAPTURE_D field."]
    #[inline] pub fn set_capture_d<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Selects whether capture is enabled for the specified capture register."]
    #[inline] pub fn capture_c(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CAPTURE_C != 0"]
    #[inline] pub fn test_capture_c(&self) -> bool {
        self.capture_c() != 0
    }

    #[doc="Sets the CAPTURE_C field."]
    #[inline] pub fn set_capture_c<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Selects whether capture is enabled for the specified capture register."]
    #[inline] pub fn capture_b(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CAPTURE_B != 0"]
    #[inline] pub fn test_capture_b(&self) -> bool {
        self.capture_b() != 0
    }

    #[doc="Sets the CAPTURE_B field."]
    #[inline] pub fn set_capture_b<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Selects whether capture is enabled for the specified capture register."]
    #[inline] pub fn capture_a(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CAPTURE_A != 0"]
    #[inline] pub fn test_capture_a(&self) -> bool {
        self.capture_a() != 0
    }

    #[doc="Sets the CAPTURE_A field."]
    #[inline] pub fn set_capture_a<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for CaptureControl {
    #[inline]
    fn from(other: u32) -> Self {
         CaptureControl(other)
    }
}

impl ::core::fmt::Display for CaptureControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for CaptureControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.capture_d() != 0 { try!(write!(f, " capture_d"))}
        if self.capture_c() != 0 { try!(write!(f, " capture_c"))}
        if self.capture_b() != 0 { try!(write!(f, " capture_b"))}
        if self.capture_a() != 0 { try!(write!(f, " capture_a"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Compare Register A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scmpr0(pub u32);
impl Scmpr0 {
    #[doc="Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_A_EN bit in the REG_CTIMER_STCGF register."]
    #[inline] pub fn value(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scmpr0 {
    #[inline]
    fn from(other: u32) -> Self {
         Scmpr0(other)
    }
}

impl ::core::fmt::Display for Scmpr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scmpr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Compare Register B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scmpr1(pub u32);
impl Scmpr1 {
    #[doc="Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_B_EN bit in the REG_CTIMER_STCGF register."]
    #[inline] pub fn value(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scmpr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Scmpr1(other)
    }
}

impl ::core::fmt::Display for Scmpr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scmpr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Compare Register C"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scmpr2(pub u32);
impl Scmpr2 {
    #[doc="Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_C_EN bit in the REG_CTIMER_STCGF register."]
    #[inline] pub fn value(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scmpr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Scmpr2(other)
    }
}

impl ::core::fmt::Display for Scmpr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scmpr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Compare Register D"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scmpr3(pub u32);
impl Scmpr3 {
    #[doc="Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_D_EN bit in the REG_CTIMER_STCGF register."]
    #[inline] pub fn value(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scmpr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Scmpr3(other)
    }
}

impl ::core::fmt::Display for Scmpr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scmpr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Compare Register E"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scmpr4(pub u32);
impl Scmpr4 {
    #[doc="Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_E_EN bit in the REG_CTIMER_STCGF register."]
    #[inline] pub fn value(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scmpr4 {
    #[inline]
    fn from(other: u32) -> Self {
         Scmpr4(other)
    }
}

impl ::core::fmt::Display for Scmpr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scmpr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Compare Register F"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scmpr5(pub u32);
impl Scmpr5 {
    #[doc="Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_F_EN bit in the REG_CTIMER_STCGF register."]
    #[inline] pub fn value(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scmpr5 {
    #[inline]
    fn from(other: u32) -> Self {
         Scmpr5(other)
    }
}

impl ::core::fmt::Display for Scmpr5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scmpr5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Compare Register G"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scmpr6(pub u32);
impl Scmpr6 {
    #[doc="Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_G_EN bit in the REG_CTIMER_STCGF register."]
    #[inline] pub fn value(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scmpr6 {
    #[inline]
    fn from(other: u32) -> Self {
         Scmpr6(other)
    }
}

impl ::core::fmt::Display for Scmpr6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scmpr6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Compare Register H"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scmpr7(pub u32);
impl Scmpr7 {
    #[doc="Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_H_EN bit in the REG_CTIMER_STCGF register."]
    #[inline] pub fn value(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scmpr7 {
    #[inline]
    fn from(other: u32) -> Self {
         Scmpr7(other)
    }
}

impl ::core::fmt::Display for Scmpr7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scmpr7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Capture Register A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scapt0(pub u32);
impl Scapt0 {
    #[doc="Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
    #[inline] pub fn value(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scapt0 {
    #[inline]
    fn from(other: u32) -> Self {
         Scapt0(other)
    }
}

impl ::core::fmt::Display for Scapt0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scapt0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Capture Register B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scapt1(pub u32);
impl Scapt1 {
    #[doc="Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
    #[inline] pub fn value(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scapt1 {
    #[inline]
    fn from(other: u32) -> Self {
         Scapt1(other)
    }
}

impl ::core::fmt::Display for Scapt1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scapt1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Capture Register C"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scapt2(pub u32);
impl Scapt2 {
    #[doc="Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
    #[inline] pub fn value(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scapt2 {
    #[inline]
    fn from(other: u32) -> Self {
         Scapt2(other)
    }
}

impl ::core::fmt::Display for Scapt2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scapt2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Capture Register D"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scapt3(pub u32);
impl Scapt3 {
    #[doc="Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
    #[inline] pub fn value(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scapt3 {
    #[inline]
    fn from(other: u32) -> Self {
         Scapt3(other)
    }
}

impl ::core::fmt::Display for Scapt3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scapt3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Timer NVRAM_A Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Snvr0(pub u32);
impl Snvr0 {
    #[doc="Value of the 32-bit counter as it ticks over."]
    #[inline] pub fn value(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Snvr0 {
    #[inline]
    fn from(other: u32) -> Self {
         Snvr0(other)
    }
}

impl ::core::fmt::Display for Snvr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Snvr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Timer NVRAM_B Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Snvr1(pub u32);
impl Snvr1 {
    #[doc="Value of the 32-bit counter as it ticks over."]
    #[inline] pub fn value(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Snvr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Snvr1(other)
    }
}

impl ::core::fmt::Display for Snvr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Snvr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Timer NVRAM_C Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Snvr2(pub u32);
impl Snvr2 {
    #[doc="Value of the 32-bit counter as it ticks over."]
    #[inline] pub fn value(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Snvr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Snvr2(other)
    }
}

impl ::core::fmt::Display for Snvr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Snvr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter/Timer Interrupts: Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc="Counter/Timer B3 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb3c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTMRB3C1INT != 0"]
    #[inline] pub fn test_ctmrb3c1int(&self) -> bool {
        self.ctmrb3c1int() != 0
    }

    #[doc="Sets the CTMRB3C1INT field."]
    #[inline] pub fn set_ctmrb3c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Counter/Timer A3 interrupt based on COMPR1."]
    #[inline] pub fn ctmra3c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if CTMRA3C1INT != 0"]
    #[inline] pub fn test_ctmra3c1int(&self) -> bool {
        self.ctmra3c1int() != 0
    }

    #[doc="Sets the CTMRA3C1INT field."]
    #[inline] pub fn set_ctmra3c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Counter/Timer B2 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb2c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if CTMRB2C1INT != 0"]
    #[inline] pub fn test_ctmrb2c1int(&self) -> bool {
        self.ctmrb2c1int() != 0
    }

    #[doc="Sets the CTMRB2C1INT field."]
    #[inline] pub fn set_ctmrb2c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Counter/Timer A2 interrupt based on COMPR1."]
    #[inline] pub fn ctmra2c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CTMRA2C1INT != 0"]
    #[inline] pub fn test_ctmra2c1int(&self) -> bool {
        self.ctmra2c1int() != 0
    }

    #[doc="Sets the CTMRA2C1INT field."]
    #[inline] pub fn set_ctmra2c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Counter/Timer B1 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb1c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CTMRB1C1INT != 0"]
    #[inline] pub fn test_ctmrb1c1int(&self) -> bool {
        self.ctmrb1c1int() != 0
    }

    #[doc="Sets the CTMRB1C1INT field."]
    #[inline] pub fn set_ctmrb1c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Counter/Timer A1 interrupt based on COMPR1."]
    #[inline] pub fn ctmra1c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CTMRA1C1INT != 0"]
    #[inline] pub fn test_ctmra1c1int(&self) -> bool {
        self.ctmra1c1int() != 0
    }

    #[doc="Sets the CTMRA1C1INT field."]
    #[inline] pub fn set_ctmra1c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Counter/Timer B0 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb0c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CTMRB0C1INT != 0"]
    #[inline] pub fn test_ctmrb0c1int(&self) -> bool {
        self.ctmrb0c1int() != 0
    }

    #[doc="Sets the CTMRB0C1INT field."]
    #[inline] pub fn set_ctmrb0c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Counter/Timer A0 interrupt based on COMPR1."]
    #[inline] pub fn ctmra0c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CTMRA0C1INT != 0"]
    #[inline] pub fn test_ctmra0c1int(&self) -> bool {
        self.ctmra0c1int() != 0
    }

    #[doc="Sets the CTMRA0C1INT field."]
    #[inline] pub fn set_ctmra0c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Counter/Timer B3 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb3c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CTMRB3C0INT != 0"]
    #[inline] pub fn test_ctmrb3c0int(&self) -> bool {
        self.ctmrb3c0int() != 0
    }

    #[doc="Sets the CTMRB3C0INT field."]
    #[inline] pub fn set_ctmrb3c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Counter/Timer A3 interrupt based on COMPR0."]
    #[inline] pub fn ctmra3c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CTMRA3C0INT != 0"]
    #[inline] pub fn test_ctmra3c0int(&self) -> bool {
        self.ctmra3c0int() != 0
    }

    #[doc="Sets the CTMRA3C0INT field."]
    #[inline] pub fn set_ctmra3c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Counter/Timer B2 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb2c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CTMRB2C0INT != 0"]
    #[inline] pub fn test_ctmrb2c0int(&self) -> bool {
        self.ctmrb2c0int() != 0
    }

    #[doc="Sets the CTMRB2C0INT field."]
    #[inline] pub fn set_ctmrb2c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Counter/Timer A2 interrupt based on COMPR0."]
    #[inline] pub fn ctmra2c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CTMRA2C0INT != 0"]
    #[inline] pub fn test_ctmra2c0int(&self) -> bool {
        self.ctmra2c0int() != 0
    }

    #[doc="Sets the CTMRA2C0INT field."]
    #[inline] pub fn set_ctmra2c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Counter/Timer B1 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb1c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CTMRB1C0INT != 0"]
    #[inline] pub fn test_ctmrb1c0int(&self) -> bool {
        self.ctmrb1c0int() != 0
    }

    #[doc="Sets the CTMRB1C0INT field."]
    #[inline] pub fn set_ctmrb1c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Counter/Timer A1 interrupt based on COMPR0."]
    #[inline] pub fn ctmra1c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CTMRA1C0INT != 0"]
    #[inline] pub fn test_ctmra1c0int(&self) -> bool {
        self.ctmra1c0int() != 0
    }

    #[doc="Sets the CTMRA1C0INT field."]
    #[inline] pub fn set_ctmra1c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Counter/Timer B0 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb0c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CTMRB0C0INT != 0"]
    #[inline] pub fn test_ctmrb0c0int(&self) -> bool {
        self.ctmrb0c0int() != 0
    }

    #[doc="Sets the CTMRB0C0INT field."]
    #[inline] pub fn set_ctmrb0c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Counter/Timer A0 interrupt based on COMPR0."]
    #[inline] pub fn ctmra0c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CTMRA0C0INT != 0"]
    #[inline] pub fn test_ctmra0c0int(&self) -> bool {
        self.ctmra0c0int() != 0
    }

    #[doc="Sets the CTMRA0C0INT field."]
    #[inline] pub fn set_ctmra0c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.ctmrb3c1int() != 0 { try!(write!(f, " ctmrb3c1int"))}
        if self.ctmra3c1int() != 0 { try!(write!(f, " ctmra3c1int"))}
        if self.ctmrb2c1int() != 0 { try!(write!(f, " ctmrb2c1int"))}
        if self.ctmra2c1int() != 0 { try!(write!(f, " ctmra2c1int"))}
        if self.ctmrb1c1int() != 0 { try!(write!(f, " ctmrb1c1int"))}
        if self.ctmra1c1int() != 0 { try!(write!(f, " ctmra1c1int"))}
        if self.ctmrb0c1int() != 0 { try!(write!(f, " ctmrb0c1int"))}
        if self.ctmra0c1int() != 0 { try!(write!(f, " ctmra0c1int"))}
        if self.ctmrb3c0int() != 0 { try!(write!(f, " ctmrb3c0int"))}
        if self.ctmra3c0int() != 0 { try!(write!(f, " ctmra3c0int"))}
        if self.ctmrb2c0int() != 0 { try!(write!(f, " ctmrb2c0int"))}
        if self.ctmra2c0int() != 0 { try!(write!(f, " ctmra2c0int"))}
        if self.ctmrb1c0int() != 0 { try!(write!(f, " ctmrb1c0int"))}
        if self.ctmra1c0int() != 0 { try!(write!(f, " ctmra1c0int"))}
        if self.ctmrb0c0int() != 0 { try!(write!(f, " ctmrb0c0int"))}
        if self.ctmra0c0int() != 0 { try!(write!(f, " ctmra0c0int"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter/Timer Interrupts: Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc="Counter/Timer B3 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb3c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTMRB3C1INT != 0"]
    #[inline] pub fn test_ctmrb3c1int(&self) -> bool {
        self.ctmrb3c1int() != 0
    }

    #[doc="Sets the CTMRB3C1INT field."]
    #[inline] pub fn set_ctmrb3c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Counter/Timer A3 interrupt based on COMPR1."]
    #[inline] pub fn ctmra3c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if CTMRA3C1INT != 0"]
    #[inline] pub fn test_ctmra3c1int(&self) -> bool {
        self.ctmra3c1int() != 0
    }

    #[doc="Sets the CTMRA3C1INT field."]
    #[inline] pub fn set_ctmra3c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Counter/Timer B2 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb2c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if CTMRB2C1INT != 0"]
    #[inline] pub fn test_ctmrb2c1int(&self) -> bool {
        self.ctmrb2c1int() != 0
    }

    #[doc="Sets the CTMRB2C1INT field."]
    #[inline] pub fn set_ctmrb2c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Counter/Timer A2 interrupt based on COMPR1."]
    #[inline] pub fn ctmra2c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CTMRA2C1INT != 0"]
    #[inline] pub fn test_ctmra2c1int(&self) -> bool {
        self.ctmra2c1int() != 0
    }

    #[doc="Sets the CTMRA2C1INT field."]
    #[inline] pub fn set_ctmra2c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Counter/Timer B1 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb1c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CTMRB1C1INT != 0"]
    #[inline] pub fn test_ctmrb1c1int(&self) -> bool {
        self.ctmrb1c1int() != 0
    }

    #[doc="Sets the CTMRB1C1INT field."]
    #[inline] pub fn set_ctmrb1c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Counter/Timer A1 interrupt based on COMPR1."]
    #[inline] pub fn ctmra1c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CTMRA1C1INT != 0"]
    #[inline] pub fn test_ctmra1c1int(&self) -> bool {
        self.ctmra1c1int() != 0
    }

    #[doc="Sets the CTMRA1C1INT field."]
    #[inline] pub fn set_ctmra1c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Counter/Timer B0 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb0c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CTMRB0C1INT != 0"]
    #[inline] pub fn test_ctmrb0c1int(&self) -> bool {
        self.ctmrb0c1int() != 0
    }

    #[doc="Sets the CTMRB0C1INT field."]
    #[inline] pub fn set_ctmrb0c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Counter/Timer A0 interrupt based on COMPR1."]
    #[inline] pub fn ctmra0c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CTMRA0C1INT != 0"]
    #[inline] pub fn test_ctmra0c1int(&self) -> bool {
        self.ctmra0c1int() != 0
    }

    #[doc="Sets the CTMRA0C1INT field."]
    #[inline] pub fn set_ctmra0c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Counter/Timer B3 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb3c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CTMRB3C0INT != 0"]
    #[inline] pub fn test_ctmrb3c0int(&self) -> bool {
        self.ctmrb3c0int() != 0
    }

    #[doc="Sets the CTMRB3C0INT field."]
    #[inline] pub fn set_ctmrb3c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Counter/Timer A3 interrupt based on COMPR0."]
    #[inline] pub fn ctmra3c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CTMRA3C0INT != 0"]
    #[inline] pub fn test_ctmra3c0int(&self) -> bool {
        self.ctmra3c0int() != 0
    }

    #[doc="Sets the CTMRA3C0INT field."]
    #[inline] pub fn set_ctmra3c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Counter/Timer B2 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb2c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CTMRB2C0INT != 0"]
    #[inline] pub fn test_ctmrb2c0int(&self) -> bool {
        self.ctmrb2c0int() != 0
    }

    #[doc="Sets the CTMRB2C0INT field."]
    #[inline] pub fn set_ctmrb2c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Counter/Timer A2 interrupt based on COMPR0."]
    #[inline] pub fn ctmra2c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CTMRA2C0INT != 0"]
    #[inline] pub fn test_ctmra2c0int(&self) -> bool {
        self.ctmra2c0int() != 0
    }

    #[doc="Sets the CTMRA2C0INT field."]
    #[inline] pub fn set_ctmra2c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Counter/Timer B1 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb1c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CTMRB1C0INT != 0"]
    #[inline] pub fn test_ctmrb1c0int(&self) -> bool {
        self.ctmrb1c0int() != 0
    }

    #[doc="Sets the CTMRB1C0INT field."]
    #[inline] pub fn set_ctmrb1c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Counter/Timer A1 interrupt based on COMPR0."]
    #[inline] pub fn ctmra1c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CTMRA1C0INT != 0"]
    #[inline] pub fn test_ctmra1c0int(&self) -> bool {
        self.ctmra1c0int() != 0
    }

    #[doc="Sets the CTMRA1C0INT field."]
    #[inline] pub fn set_ctmra1c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Counter/Timer B0 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb0c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CTMRB0C0INT != 0"]
    #[inline] pub fn test_ctmrb0c0int(&self) -> bool {
        self.ctmrb0c0int() != 0
    }

    #[doc="Sets the CTMRB0C0INT field."]
    #[inline] pub fn set_ctmrb0c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Counter/Timer A0 interrupt based on COMPR0."]
    #[inline] pub fn ctmra0c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CTMRA0C0INT != 0"]
    #[inline] pub fn test_ctmra0c0int(&self) -> bool {
        self.ctmra0c0int() != 0
    }

    #[doc="Sets the CTMRA0C0INT field."]
    #[inline] pub fn set_ctmra0c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.ctmrb3c1int() != 0 { try!(write!(f, " ctmrb3c1int"))}
        if self.ctmra3c1int() != 0 { try!(write!(f, " ctmra3c1int"))}
        if self.ctmrb2c1int() != 0 { try!(write!(f, " ctmrb2c1int"))}
        if self.ctmra2c1int() != 0 { try!(write!(f, " ctmra2c1int"))}
        if self.ctmrb1c1int() != 0 { try!(write!(f, " ctmrb1c1int"))}
        if self.ctmra1c1int() != 0 { try!(write!(f, " ctmra1c1int"))}
        if self.ctmrb0c1int() != 0 { try!(write!(f, " ctmrb0c1int"))}
        if self.ctmra0c1int() != 0 { try!(write!(f, " ctmra0c1int"))}
        if self.ctmrb3c0int() != 0 { try!(write!(f, " ctmrb3c0int"))}
        if self.ctmra3c0int() != 0 { try!(write!(f, " ctmra3c0int"))}
        if self.ctmrb2c0int() != 0 { try!(write!(f, " ctmrb2c0int"))}
        if self.ctmra2c0int() != 0 { try!(write!(f, " ctmra2c0int"))}
        if self.ctmrb1c0int() != 0 { try!(write!(f, " ctmrb1c0int"))}
        if self.ctmra1c0int() != 0 { try!(write!(f, " ctmra1c0int"))}
        if self.ctmrb0c0int() != 0 { try!(write!(f, " ctmrb0c0int"))}
        if self.ctmra0c0int() != 0 { try!(write!(f, " ctmra0c0int"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter/Timer Interrupts: Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intclr(pub u32);
impl Intclr {
    #[doc="Counter/Timer B3 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb3c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTMRB3C1INT != 0"]
    #[inline] pub fn test_ctmrb3c1int(&self) -> bool {
        self.ctmrb3c1int() != 0
    }

    #[doc="Sets the CTMRB3C1INT field."]
    #[inline] pub fn set_ctmrb3c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Counter/Timer A3 interrupt based on COMPR1."]
    #[inline] pub fn ctmra3c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if CTMRA3C1INT != 0"]
    #[inline] pub fn test_ctmra3c1int(&self) -> bool {
        self.ctmra3c1int() != 0
    }

    #[doc="Sets the CTMRA3C1INT field."]
    #[inline] pub fn set_ctmra3c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Counter/Timer B2 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb2c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if CTMRB2C1INT != 0"]
    #[inline] pub fn test_ctmrb2c1int(&self) -> bool {
        self.ctmrb2c1int() != 0
    }

    #[doc="Sets the CTMRB2C1INT field."]
    #[inline] pub fn set_ctmrb2c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Counter/Timer A2 interrupt based on COMPR1."]
    #[inline] pub fn ctmra2c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CTMRA2C1INT != 0"]
    #[inline] pub fn test_ctmra2c1int(&self) -> bool {
        self.ctmra2c1int() != 0
    }

    #[doc="Sets the CTMRA2C1INT field."]
    #[inline] pub fn set_ctmra2c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Counter/Timer B1 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb1c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CTMRB1C1INT != 0"]
    #[inline] pub fn test_ctmrb1c1int(&self) -> bool {
        self.ctmrb1c1int() != 0
    }

    #[doc="Sets the CTMRB1C1INT field."]
    #[inline] pub fn set_ctmrb1c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Counter/Timer A1 interrupt based on COMPR1."]
    #[inline] pub fn ctmra1c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CTMRA1C1INT != 0"]
    #[inline] pub fn test_ctmra1c1int(&self) -> bool {
        self.ctmra1c1int() != 0
    }

    #[doc="Sets the CTMRA1C1INT field."]
    #[inline] pub fn set_ctmra1c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Counter/Timer B0 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb0c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CTMRB0C1INT != 0"]
    #[inline] pub fn test_ctmrb0c1int(&self) -> bool {
        self.ctmrb0c1int() != 0
    }

    #[doc="Sets the CTMRB0C1INT field."]
    #[inline] pub fn set_ctmrb0c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Counter/Timer A0 interrupt based on COMPR1."]
    #[inline] pub fn ctmra0c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CTMRA0C1INT != 0"]
    #[inline] pub fn test_ctmra0c1int(&self) -> bool {
        self.ctmra0c1int() != 0
    }

    #[doc="Sets the CTMRA0C1INT field."]
    #[inline] pub fn set_ctmra0c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Counter/Timer B3 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb3c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CTMRB3C0INT != 0"]
    #[inline] pub fn test_ctmrb3c0int(&self) -> bool {
        self.ctmrb3c0int() != 0
    }

    #[doc="Sets the CTMRB3C0INT field."]
    #[inline] pub fn set_ctmrb3c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Counter/Timer A3 interrupt based on COMPR0."]
    #[inline] pub fn ctmra3c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CTMRA3C0INT != 0"]
    #[inline] pub fn test_ctmra3c0int(&self) -> bool {
        self.ctmra3c0int() != 0
    }

    #[doc="Sets the CTMRA3C0INT field."]
    #[inline] pub fn set_ctmra3c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Counter/Timer B2 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb2c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CTMRB2C0INT != 0"]
    #[inline] pub fn test_ctmrb2c0int(&self) -> bool {
        self.ctmrb2c0int() != 0
    }

    #[doc="Sets the CTMRB2C0INT field."]
    #[inline] pub fn set_ctmrb2c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Counter/Timer A2 interrupt based on COMPR0."]
    #[inline] pub fn ctmra2c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CTMRA2C0INT != 0"]
    #[inline] pub fn test_ctmra2c0int(&self) -> bool {
        self.ctmra2c0int() != 0
    }

    #[doc="Sets the CTMRA2C0INT field."]
    #[inline] pub fn set_ctmra2c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Counter/Timer B1 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb1c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CTMRB1C0INT != 0"]
    #[inline] pub fn test_ctmrb1c0int(&self) -> bool {
        self.ctmrb1c0int() != 0
    }

    #[doc="Sets the CTMRB1C0INT field."]
    #[inline] pub fn set_ctmrb1c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Counter/Timer A1 interrupt based on COMPR0."]
    #[inline] pub fn ctmra1c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CTMRA1C0INT != 0"]
    #[inline] pub fn test_ctmra1c0int(&self) -> bool {
        self.ctmra1c0int() != 0
    }

    #[doc="Sets the CTMRA1C0INT field."]
    #[inline] pub fn set_ctmra1c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Counter/Timer B0 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb0c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CTMRB0C0INT != 0"]
    #[inline] pub fn test_ctmrb0c0int(&self) -> bool {
        self.ctmrb0c0int() != 0
    }

    #[doc="Sets the CTMRB0C0INT field."]
    #[inline] pub fn set_ctmrb0c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Counter/Timer A0 interrupt based on COMPR0."]
    #[inline] pub fn ctmra0c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CTMRA0C0INT != 0"]
    #[inline] pub fn test_ctmra0c0int(&self) -> bool {
        self.ctmra0c0int() != 0
    }

    #[doc="Sets the CTMRA0C0INT field."]
    #[inline] pub fn set_ctmra0c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.ctmrb3c1int() != 0 { try!(write!(f, " ctmrb3c1int"))}
        if self.ctmra3c1int() != 0 { try!(write!(f, " ctmra3c1int"))}
        if self.ctmrb2c1int() != 0 { try!(write!(f, " ctmrb2c1int"))}
        if self.ctmra2c1int() != 0 { try!(write!(f, " ctmra2c1int"))}
        if self.ctmrb1c1int() != 0 { try!(write!(f, " ctmrb1c1int"))}
        if self.ctmra1c1int() != 0 { try!(write!(f, " ctmra1c1int"))}
        if self.ctmrb0c1int() != 0 { try!(write!(f, " ctmrb0c1int"))}
        if self.ctmra0c1int() != 0 { try!(write!(f, " ctmra0c1int"))}
        if self.ctmrb3c0int() != 0 { try!(write!(f, " ctmrb3c0int"))}
        if self.ctmra3c0int() != 0 { try!(write!(f, " ctmra3c0int"))}
        if self.ctmrb2c0int() != 0 { try!(write!(f, " ctmrb2c0int"))}
        if self.ctmra2c0int() != 0 { try!(write!(f, " ctmra2c0int"))}
        if self.ctmrb1c0int() != 0 { try!(write!(f, " ctmrb1c0int"))}
        if self.ctmra1c0int() != 0 { try!(write!(f, " ctmra1c0int"))}
        if self.ctmrb0c0int() != 0 { try!(write!(f, " ctmrb0c0int"))}
        if self.ctmra0c0int() != 0 { try!(write!(f, " ctmra0c0int"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter/Timer Interrupts: Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intset(pub u32);
impl Intset {
    #[doc="Counter/Timer B3 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb3c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTMRB3C1INT != 0"]
    #[inline] pub fn test_ctmrb3c1int(&self) -> bool {
        self.ctmrb3c1int() != 0
    }

    #[doc="Sets the CTMRB3C1INT field."]
    #[inline] pub fn set_ctmrb3c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Counter/Timer A3 interrupt based on COMPR1."]
    #[inline] pub fn ctmra3c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if CTMRA3C1INT != 0"]
    #[inline] pub fn test_ctmra3c1int(&self) -> bool {
        self.ctmra3c1int() != 0
    }

    #[doc="Sets the CTMRA3C1INT field."]
    #[inline] pub fn set_ctmra3c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Counter/Timer B2 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb2c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if CTMRB2C1INT != 0"]
    #[inline] pub fn test_ctmrb2c1int(&self) -> bool {
        self.ctmrb2c1int() != 0
    }

    #[doc="Sets the CTMRB2C1INT field."]
    #[inline] pub fn set_ctmrb2c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Counter/Timer A2 interrupt based on COMPR1."]
    #[inline] pub fn ctmra2c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CTMRA2C1INT != 0"]
    #[inline] pub fn test_ctmra2c1int(&self) -> bool {
        self.ctmra2c1int() != 0
    }

    #[doc="Sets the CTMRA2C1INT field."]
    #[inline] pub fn set_ctmra2c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Counter/Timer B1 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb1c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CTMRB1C1INT != 0"]
    #[inline] pub fn test_ctmrb1c1int(&self) -> bool {
        self.ctmrb1c1int() != 0
    }

    #[doc="Sets the CTMRB1C1INT field."]
    #[inline] pub fn set_ctmrb1c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Counter/Timer A1 interrupt based on COMPR1."]
    #[inline] pub fn ctmra1c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CTMRA1C1INT != 0"]
    #[inline] pub fn test_ctmra1c1int(&self) -> bool {
        self.ctmra1c1int() != 0
    }

    #[doc="Sets the CTMRA1C1INT field."]
    #[inline] pub fn set_ctmra1c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Counter/Timer B0 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb0c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CTMRB0C1INT != 0"]
    #[inline] pub fn test_ctmrb0c1int(&self) -> bool {
        self.ctmrb0c1int() != 0
    }

    #[doc="Sets the CTMRB0C1INT field."]
    #[inline] pub fn set_ctmrb0c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Counter/Timer A0 interrupt based on COMPR1."]
    #[inline] pub fn ctmra0c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CTMRA0C1INT != 0"]
    #[inline] pub fn test_ctmra0c1int(&self) -> bool {
        self.ctmra0c1int() != 0
    }

    #[doc="Sets the CTMRA0C1INT field."]
    #[inline] pub fn set_ctmra0c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Counter/Timer B3 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb3c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CTMRB3C0INT != 0"]
    #[inline] pub fn test_ctmrb3c0int(&self) -> bool {
        self.ctmrb3c0int() != 0
    }

    #[doc="Sets the CTMRB3C0INT field."]
    #[inline] pub fn set_ctmrb3c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Counter/Timer A3 interrupt based on COMPR0."]
    #[inline] pub fn ctmra3c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CTMRA3C0INT != 0"]
    #[inline] pub fn test_ctmra3c0int(&self) -> bool {
        self.ctmra3c0int() != 0
    }

    #[doc="Sets the CTMRA3C0INT field."]
    #[inline] pub fn set_ctmra3c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Counter/Timer B2 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb2c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CTMRB2C0INT != 0"]
    #[inline] pub fn test_ctmrb2c0int(&self) -> bool {
        self.ctmrb2c0int() != 0
    }

    #[doc="Sets the CTMRB2C0INT field."]
    #[inline] pub fn set_ctmrb2c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Counter/Timer A2 interrupt based on COMPR0."]
    #[inline] pub fn ctmra2c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CTMRA2C0INT != 0"]
    #[inline] pub fn test_ctmra2c0int(&self) -> bool {
        self.ctmra2c0int() != 0
    }

    #[doc="Sets the CTMRA2C0INT field."]
    #[inline] pub fn set_ctmra2c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Counter/Timer B1 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb1c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CTMRB1C0INT != 0"]
    #[inline] pub fn test_ctmrb1c0int(&self) -> bool {
        self.ctmrb1c0int() != 0
    }

    #[doc="Sets the CTMRB1C0INT field."]
    #[inline] pub fn set_ctmrb1c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Counter/Timer A1 interrupt based on COMPR0."]
    #[inline] pub fn ctmra1c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CTMRA1C0INT != 0"]
    #[inline] pub fn test_ctmra1c0int(&self) -> bool {
        self.ctmra1c0int() != 0
    }

    #[doc="Sets the CTMRA1C0INT field."]
    #[inline] pub fn set_ctmra1c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Counter/Timer B0 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb0c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CTMRB0C0INT != 0"]
    #[inline] pub fn test_ctmrb0c0int(&self) -> bool {
        self.ctmrb0c0int() != 0
    }

    #[doc="Sets the CTMRB0C0INT field."]
    #[inline] pub fn set_ctmrb0c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Counter/Timer A0 interrupt based on COMPR0."]
    #[inline] pub fn ctmra0c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CTMRA0C0INT != 0"]
    #[inline] pub fn test_ctmra0c0int(&self) -> bool {
        self.ctmra0c0int() != 0
    }

    #[doc="Sets the CTMRA0C0INT field."]
    #[inline] pub fn set_ctmra0c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.ctmrb3c1int() != 0 { try!(write!(f, " ctmrb3c1int"))}
        if self.ctmra3c1int() != 0 { try!(write!(f, " ctmra3c1int"))}
        if self.ctmrb2c1int() != 0 { try!(write!(f, " ctmrb2c1int"))}
        if self.ctmra2c1int() != 0 { try!(write!(f, " ctmra2c1int"))}
        if self.ctmrb1c1int() != 0 { try!(write!(f, " ctmrb1c1int"))}
        if self.ctmra1c1int() != 0 { try!(write!(f, " ctmra1c1int"))}
        if self.ctmrb0c1int() != 0 { try!(write!(f, " ctmrb0c1int"))}
        if self.ctmra0c1int() != 0 { try!(write!(f, " ctmra0c1int"))}
        if self.ctmrb3c0int() != 0 { try!(write!(f, " ctmrb3c0int"))}
        if self.ctmra3c0int() != 0 { try!(write!(f, " ctmra3c0int"))}
        if self.ctmrb2c0int() != 0 { try!(write!(f, " ctmrb2c0int"))}
        if self.ctmra2c0int() != 0 { try!(write!(f, " ctmra2c0int"))}
        if self.ctmrb1c0int() != 0 { try!(write!(f, " ctmrb1c0int"))}
        if self.ctmra1c0int() != 0 { try!(write!(f, " ctmra1c0int"))}
        if self.ctmrb0c0int() != 0 { try!(write!(f, " ctmrb0c0int"))}
        if self.ctmra0c0int() != 0 { try!(write!(f, " ctmra0c0int"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="STIMER Interrupt registers: Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stminten(pub u32);
impl Stminten {
    #[doc="CAPTURE register D has grabbed the value in the counter"]
    #[inline] pub fn captured(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CAPTURED != 0"]
    #[inline] pub fn test_captured(&self) -> bool {
        self.captured() != 0
    }

    #[doc="Sets the CAPTURED field."]
    #[inline] pub fn set_captured<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CAPTURE register C has grabbed the value in the counter"]
    #[inline] pub fn capturec(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CAPTUREC != 0"]
    #[inline] pub fn test_capturec(&self) -> bool {
        self.capturec() != 0
    }

    #[doc="Sets the CAPTUREC field."]
    #[inline] pub fn set_capturec<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="CAPTURE register B has grabbed the value in the counter"]
    #[inline] pub fn captureb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CAPTUREB != 0"]
    #[inline] pub fn test_captureb(&self) -> bool {
        self.captureb() != 0
    }

    #[doc="Sets the CAPTUREB field."]
    #[inline] pub fn set_captureb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="CAPTURE register A has grabbed the value in the counter"]
    #[inline] pub fn capturea(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CAPTUREA != 0"]
    #[inline] pub fn test_capturea(&self) -> bool {
        self.capturea() != 0
    }

    #[doc="Sets the CAPTUREA field."]
    #[inline] pub fn set_capturea<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
    #[inline] pub fn overflow(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if OVERFLOW != 0"]
    #[inline] pub fn test_overflow(&self) -> bool {
        self.overflow() != 0
    }

    #[doc="Sets the OVERFLOW field."]
    #[inline] pub fn set_overflow<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register H."]
    #[inline] pub fn compareh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if COMPAREH != 0"]
    #[inline] pub fn test_compareh(&self) -> bool {
        self.compareh() != 0
    }

    #[doc="Sets the COMPAREH field."]
    #[inline] pub fn set_compareh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register G."]
    #[inline] pub fn compareg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if COMPAREG != 0"]
    #[inline] pub fn test_compareg(&self) -> bool {
        self.compareg() != 0
    }

    #[doc="Sets the COMPAREG field."]
    #[inline] pub fn set_compareg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register F."]
    #[inline] pub fn comparef(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if COMPAREF != 0"]
    #[inline] pub fn test_comparef(&self) -> bool {
        self.comparef() != 0
    }

    #[doc="Sets the COMPAREF field."]
    #[inline] pub fn set_comparef<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register E."]
    #[inline] pub fn comparee(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if COMPAREE != 0"]
    #[inline] pub fn test_comparee(&self) -> bool {
        self.comparee() != 0
    }

    #[doc="Sets the COMPAREE field."]
    #[inline] pub fn set_comparee<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register D."]
    #[inline] pub fn compared(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if COMPARED != 0"]
    #[inline] pub fn test_compared(&self) -> bool {
        self.compared() != 0
    }

    #[doc="Sets the COMPARED field."]
    #[inline] pub fn set_compared<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register C."]
    #[inline] pub fn comparec(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if COMPAREC != 0"]
    #[inline] pub fn test_comparec(&self) -> bool {
        self.comparec() != 0
    }

    #[doc="Sets the COMPAREC field."]
    #[inline] pub fn set_comparec<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register B."]
    #[inline] pub fn compareb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if COMPAREB != 0"]
    #[inline] pub fn test_compareb(&self) -> bool {
        self.compareb() != 0
    }

    #[doc="Sets the COMPAREB field."]
    #[inline] pub fn set_compareb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register A."]
    #[inline] pub fn comparea(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if COMPAREA != 0"]
    #[inline] pub fn test_comparea(&self) -> bool {
        self.comparea() != 0
    }

    #[doc="Sets the COMPAREA field."]
    #[inline] pub fn set_comparea<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Stminten {
    #[inline]
    fn from(other: u32) -> Self {
         Stminten(other)
    }
}

impl ::core::fmt::Display for Stminten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Stminten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.captured() != 0 { try!(write!(f, " captured"))}
        if self.capturec() != 0 { try!(write!(f, " capturec"))}
        if self.captureb() != 0 { try!(write!(f, " captureb"))}
        if self.capturea() != 0 { try!(write!(f, " capturea"))}
        if self.overflow() != 0 { try!(write!(f, " overflow"))}
        if self.compareh() != 0 { try!(write!(f, " compareh"))}
        if self.compareg() != 0 { try!(write!(f, " compareg"))}
        if self.comparef() != 0 { try!(write!(f, " comparef"))}
        if self.comparee() != 0 { try!(write!(f, " comparee"))}
        if self.compared() != 0 { try!(write!(f, " compared"))}
        if self.comparec() != 0 { try!(write!(f, " comparec"))}
        if self.compareb() != 0 { try!(write!(f, " compareb"))}
        if self.comparea() != 0 { try!(write!(f, " comparea"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="STIMER Interrupt registers: Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stmintstat(pub u32);
impl Stmintstat {
    #[doc="CAPTURE register D has grabbed the value in the counter"]
    #[inline] pub fn captured(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CAPTURED != 0"]
    #[inline] pub fn test_captured(&self) -> bool {
        self.captured() != 0
    }

    #[doc="Sets the CAPTURED field."]
    #[inline] pub fn set_captured<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CAPTURE register C has grabbed the value in the counter"]
    #[inline] pub fn capturec(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CAPTUREC != 0"]
    #[inline] pub fn test_capturec(&self) -> bool {
        self.capturec() != 0
    }

    #[doc="Sets the CAPTUREC field."]
    #[inline] pub fn set_capturec<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="CAPTURE register B has grabbed the value in the counter"]
    #[inline] pub fn captureb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CAPTUREB != 0"]
    #[inline] pub fn test_captureb(&self) -> bool {
        self.captureb() != 0
    }

    #[doc="Sets the CAPTUREB field."]
    #[inline] pub fn set_captureb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="CAPTURE register A has grabbed the value in the counter"]
    #[inline] pub fn capturea(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CAPTUREA != 0"]
    #[inline] pub fn test_capturea(&self) -> bool {
        self.capturea() != 0
    }

    #[doc="Sets the CAPTUREA field."]
    #[inline] pub fn set_capturea<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
    #[inline] pub fn overflow(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if OVERFLOW != 0"]
    #[inline] pub fn test_overflow(&self) -> bool {
        self.overflow() != 0
    }

    #[doc="Sets the OVERFLOW field."]
    #[inline] pub fn set_overflow<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register H."]
    #[inline] pub fn compareh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if COMPAREH != 0"]
    #[inline] pub fn test_compareh(&self) -> bool {
        self.compareh() != 0
    }

    #[doc="Sets the COMPAREH field."]
    #[inline] pub fn set_compareh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register G."]
    #[inline] pub fn compareg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if COMPAREG != 0"]
    #[inline] pub fn test_compareg(&self) -> bool {
        self.compareg() != 0
    }

    #[doc="Sets the COMPAREG field."]
    #[inline] pub fn set_compareg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register F."]
    #[inline] pub fn comparef(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if COMPAREF != 0"]
    #[inline] pub fn test_comparef(&self) -> bool {
        self.comparef() != 0
    }

    #[doc="Sets the COMPAREF field."]
    #[inline] pub fn set_comparef<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register E."]
    #[inline] pub fn comparee(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if COMPAREE != 0"]
    #[inline] pub fn test_comparee(&self) -> bool {
        self.comparee() != 0
    }

    #[doc="Sets the COMPAREE field."]
    #[inline] pub fn set_comparee<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register D."]
    #[inline] pub fn compared(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if COMPARED != 0"]
    #[inline] pub fn test_compared(&self) -> bool {
        self.compared() != 0
    }

    #[doc="Sets the COMPARED field."]
    #[inline] pub fn set_compared<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register C."]
    #[inline] pub fn comparec(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if COMPAREC != 0"]
    #[inline] pub fn test_comparec(&self) -> bool {
        self.comparec() != 0
    }

    #[doc="Sets the COMPAREC field."]
    #[inline] pub fn set_comparec<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register B."]
    #[inline] pub fn compareb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if COMPAREB != 0"]
    #[inline] pub fn test_compareb(&self) -> bool {
        self.compareb() != 0
    }

    #[doc="Sets the COMPAREB field."]
    #[inline] pub fn set_compareb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register A."]
    #[inline] pub fn comparea(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if COMPAREA != 0"]
    #[inline] pub fn test_comparea(&self) -> bool {
        self.comparea() != 0
    }

    #[doc="Sets the COMPAREA field."]
    #[inline] pub fn set_comparea<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Stmintstat {
    #[inline]
    fn from(other: u32) -> Self {
         Stmintstat(other)
    }
}

impl ::core::fmt::Display for Stmintstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Stmintstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.captured() != 0 { try!(write!(f, " captured"))}
        if self.capturec() != 0 { try!(write!(f, " capturec"))}
        if self.captureb() != 0 { try!(write!(f, " captureb"))}
        if self.capturea() != 0 { try!(write!(f, " capturea"))}
        if self.overflow() != 0 { try!(write!(f, " overflow"))}
        if self.compareh() != 0 { try!(write!(f, " compareh"))}
        if self.compareg() != 0 { try!(write!(f, " compareg"))}
        if self.comparef() != 0 { try!(write!(f, " comparef"))}
        if self.comparee() != 0 { try!(write!(f, " comparee"))}
        if self.compared() != 0 { try!(write!(f, " compared"))}
        if self.comparec() != 0 { try!(write!(f, " comparec"))}
        if self.compareb() != 0 { try!(write!(f, " compareb"))}
        if self.comparea() != 0 { try!(write!(f, " comparea"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="STIMER Interrupt registers: Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stmintclr(pub u32);
impl Stmintclr {
    #[doc="CAPTURE register D has grabbed the value in the counter"]
    #[inline] pub fn captured(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CAPTURED != 0"]
    #[inline] pub fn test_captured(&self) -> bool {
        self.captured() != 0
    }

    #[doc="Sets the CAPTURED field."]
    #[inline] pub fn set_captured<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CAPTURE register C has grabbed the value in the counter"]
    #[inline] pub fn capturec(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CAPTUREC != 0"]
    #[inline] pub fn test_capturec(&self) -> bool {
        self.capturec() != 0
    }

    #[doc="Sets the CAPTUREC field."]
    #[inline] pub fn set_capturec<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="CAPTURE register B has grabbed the value in the counter"]
    #[inline] pub fn captureb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CAPTUREB != 0"]
    #[inline] pub fn test_captureb(&self) -> bool {
        self.captureb() != 0
    }

    #[doc="Sets the CAPTUREB field."]
    #[inline] pub fn set_captureb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="CAPTURE register A has grabbed the value in the counter"]
    #[inline] pub fn capturea(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CAPTUREA != 0"]
    #[inline] pub fn test_capturea(&self) -> bool {
        self.capturea() != 0
    }

    #[doc="Sets the CAPTUREA field."]
    #[inline] pub fn set_capturea<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
    #[inline] pub fn overflow(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if OVERFLOW != 0"]
    #[inline] pub fn test_overflow(&self) -> bool {
        self.overflow() != 0
    }

    #[doc="Sets the OVERFLOW field."]
    #[inline] pub fn set_overflow<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register H."]
    #[inline] pub fn compareh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if COMPAREH != 0"]
    #[inline] pub fn test_compareh(&self) -> bool {
        self.compareh() != 0
    }

    #[doc="Sets the COMPAREH field."]
    #[inline] pub fn set_compareh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register G."]
    #[inline] pub fn compareg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if COMPAREG != 0"]
    #[inline] pub fn test_compareg(&self) -> bool {
        self.compareg() != 0
    }

    #[doc="Sets the COMPAREG field."]
    #[inline] pub fn set_compareg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register F."]
    #[inline] pub fn comparef(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if COMPAREF != 0"]
    #[inline] pub fn test_comparef(&self) -> bool {
        self.comparef() != 0
    }

    #[doc="Sets the COMPAREF field."]
    #[inline] pub fn set_comparef<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register E."]
    #[inline] pub fn comparee(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if COMPAREE != 0"]
    #[inline] pub fn test_comparee(&self) -> bool {
        self.comparee() != 0
    }

    #[doc="Sets the COMPAREE field."]
    #[inline] pub fn set_comparee<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register D."]
    #[inline] pub fn compared(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if COMPARED != 0"]
    #[inline] pub fn test_compared(&self) -> bool {
        self.compared() != 0
    }

    #[doc="Sets the COMPARED field."]
    #[inline] pub fn set_compared<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register C."]
    #[inline] pub fn comparec(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if COMPAREC != 0"]
    #[inline] pub fn test_comparec(&self) -> bool {
        self.comparec() != 0
    }

    #[doc="Sets the COMPAREC field."]
    #[inline] pub fn set_comparec<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register B."]
    #[inline] pub fn compareb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if COMPAREB != 0"]
    #[inline] pub fn test_compareb(&self) -> bool {
        self.compareb() != 0
    }

    #[doc="Sets the COMPAREB field."]
    #[inline] pub fn set_compareb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register A."]
    #[inline] pub fn comparea(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if COMPAREA != 0"]
    #[inline] pub fn test_comparea(&self) -> bool {
        self.comparea() != 0
    }

    #[doc="Sets the COMPAREA field."]
    #[inline] pub fn set_comparea<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Stmintclr {
    #[inline]
    fn from(other: u32) -> Self {
         Stmintclr(other)
    }
}

impl ::core::fmt::Display for Stmintclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Stmintclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.captured() != 0 { try!(write!(f, " captured"))}
        if self.capturec() != 0 { try!(write!(f, " capturec"))}
        if self.captureb() != 0 { try!(write!(f, " captureb"))}
        if self.capturea() != 0 { try!(write!(f, " capturea"))}
        if self.overflow() != 0 { try!(write!(f, " overflow"))}
        if self.compareh() != 0 { try!(write!(f, " compareh"))}
        if self.compareg() != 0 { try!(write!(f, " compareg"))}
        if self.comparef() != 0 { try!(write!(f, " comparef"))}
        if self.comparee() != 0 { try!(write!(f, " comparee"))}
        if self.compared() != 0 { try!(write!(f, " compared"))}
        if self.comparec() != 0 { try!(write!(f, " comparec"))}
        if self.compareb() != 0 { try!(write!(f, " compareb"))}
        if self.comparea() != 0 { try!(write!(f, " comparea"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="STIMER Interrupt registers: Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stmintset(pub u32);
impl Stmintset {
    #[doc="CAPTURE register D has grabbed the value in the counter"]
    #[inline] pub fn captured(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CAPTURED != 0"]
    #[inline] pub fn test_captured(&self) -> bool {
        self.captured() != 0
    }

    #[doc="Sets the CAPTURED field."]
    #[inline] pub fn set_captured<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CAPTURE register C has grabbed the value in the counter"]
    #[inline] pub fn capturec(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CAPTUREC != 0"]
    #[inline] pub fn test_capturec(&self) -> bool {
        self.capturec() != 0
    }

    #[doc="Sets the CAPTUREC field."]
    #[inline] pub fn set_capturec<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="CAPTURE register B has grabbed the value in the counter"]
    #[inline] pub fn captureb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CAPTUREB != 0"]
    #[inline] pub fn test_captureb(&self) -> bool {
        self.captureb() != 0
    }

    #[doc="Sets the CAPTUREB field."]
    #[inline] pub fn set_captureb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="CAPTURE register A has grabbed the value in the counter"]
    #[inline] pub fn capturea(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CAPTUREA != 0"]
    #[inline] pub fn test_capturea(&self) -> bool {
        self.capturea() != 0
    }

    #[doc="Sets the CAPTUREA field."]
    #[inline] pub fn set_capturea<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
    #[inline] pub fn overflow(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if OVERFLOW != 0"]
    #[inline] pub fn test_overflow(&self) -> bool {
        self.overflow() != 0
    }

    #[doc="Sets the OVERFLOW field."]
    #[inline] pub fn set_overflow<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register H."]
    #[inline] pub fn compareh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if COMPAREH != 0"]
    #[inline] pub fn test_compareh(&self) -> bool {
        self.compareh() != 0
    }

    #[doc="Sets the COMPAREH field."]
    #[inline] pub fn set_compareh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register G."]
    #[inline] pub fn compareg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if COMPAREG != 0"]
    #[inline] pub fn test_compareg(&self) -> bool {
        self.compareg() != 0
    }

    #[doc="Sets the COMPAREG field."]
    #[inline] pub fn set_compareg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register F."]
    #[inline] pub fn comparef(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if COMPAREF != 0"]
    #[inline] pub fn test_comparef(&self) -> bool {
        self.comparef() != 0
    }

    #[doc="Sets the COMPAREF field."]
    #[inline] pub fn set_comparef<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register E."]
    #[inline] pub fn comparee(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if COMPAREE != 0"]
    #[inline] pub fn test_comparee(&self) -> bool {
        self.comparee() != 0
    }

    #[doc="Sets the COMPAREE field."]
    #[inline] pub fn set_comparee<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register D."]
    #[inline] pub fn compared(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if COMPARED != 0"]
    #[inline] pub fn test_compared(&self) -> bool {
        self.compared() != 0
    }

    #[doc="Sets the COMPARED field."]
    #[inline] pub fn set_compared<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register C."]
    #[inline] pub fn comparec(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if COMPAREC != 0"]
    #[inline] pub fn test_comparec(&self) -> bool {
        self.comparec() != 0
    }

    #[doc="Sets the COMPAREC field."]
    #[inline] pub fn set_comparec<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register B."]
    #[inline] pub fn compareb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if COMPAREB != 0"]
    #[inline] pub fn test_compareb(&self) -> bool {
        self.compareb() != 0
    }

    #[doc="Sets the COMPAREB field."]
    #[inline] pub fn set_compareb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register A."]
    #[inline] pub fn comparea(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if COMPAREA != 0"]
    #[inline] pub fn test_comparea(&self) -> bool {
        self.comparea() != 0
    }

    #[doc="Sets the COMPAREA field."]
    #[inline] pub fn set_comparea<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Stmintset {
    #[inline]
    fn from(other: u32) -> Self {
         Stmintset(other)
    }
}

impl ::core::fmt::Display for Stmintset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Stmintset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.captured() != 0 { try!(write!(f, " captured"))}
        if self.capturec() != 0 { try!(write!(f, " capturec"))}
        if self.captureb() != 0 { try!(write!(f, " captureb"))}
        if self.capturea() != 0 { try!(write!(f, " capturea"))}
        if self.overflow() != 0 { try!(write!(f, " overflow"))}
        if self.compareh() != 0 { try!(write!(f, " compareh"))}
        if self.compareg() != 0 { try!(write!(f, " compareg"))}
        if self.comparef() != 0 { try!(write!(f, " comparef"))}
        if self.comparee() != 0 { try!(write!(f, " comparee"))}
        if self.compared() != 0 { try!(write!(f, " compared"))}
        if self.comparec() != 0 { try!(write!(f, " comparec"))}
        if self.compareb() != 0 { try!(write!(f, " compareb"))}
        if self.comparea() != 0 { try!(write!(f, " comparea"))}
        try!(write!(f, "]"));
        Ok(())
    }
}


