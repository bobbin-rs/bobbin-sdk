//! CMU
#[allow(unused_imports)] use bobbin_common::*;

periph!(CMU, Cmu, 0x400e4000);

#[doc="CMU"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cmu(pub usize);
impl Cmu {
    #[doc="Get the *mut pointer for the CTRL register."]
    #[inline] pub fn ctrl_mut(&self) -> *mut Ctrl { 
        (self.0 + 0x0) as *mut Ctrl
    }

    #[doc="Get the *const pointer for the CTRL register."]
    #[inline] pub fn ctrl_ptr(&self) -> *const Ctrl { 
           self.ctrl_mut()
    }

    #[doc="Read the CTRL register."]
    #[inline] pub fn ctrl(&self) -> Ctrl { 
        unsafe {
            read_volatile(self.ctrl_ptr())
        }
    }

    #[doc="Write the CTRL register."]
    #[inline] pub fn set_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrl_mut(), f(Ctrl(0)));
        }
        self
    }

    #[doc="Modify the CTRL register."]
    #[inline] pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrl_mut(), f(self.ctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HFRCOCTRL register."]
    #[inline] pub fn hfrcoctrl_mut(&self) -> *mut Hfrcoctrl { 
        (self.0 + 0x10) as *mut Hfrcoctrl
    }

    #[doc="Get the *const pointer for the HFRCOCTRL register."]
    #[inline] pub fn hfrcoctrl_ptr(&self) -> *const Hfrcoctrl { 
           self.hfrcoctrl_mut()
    }

    #[doc="Read the HFRCOCTRL register."]
    #[inline] pub fn hfrcoctrl(&self) -> Hfrcoctrl { 
        unsafe {
            read_volatile(self.hfrcoctrl_ptr())
        }
    }

    #[doc="Write the HFRCOCTRL register."]
    #[inline] pub fn set_hfrcoctrl<F: FnOnce(Hfrcoctrl) -> Hfrcoctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfrcoctrl_mut(), f(Hfrcoctrl(0)));
        }
        self
    }

    #[doc="Modify the HFRCOCTRL register."]
    #[inline] pub fn with_hfrcoctrl<F: FnOnce(Hfrcoctrl) -> Hfrcoctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfrcoctrl_mut(), f(self.hfrcoctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AUXHFRCOCTRL register."]
    #[inline] pub fn auxhfrcoctrl_mut(&self) -> *mut Auxhfrcoctrl { 
        (self.0 + 0x18) as *mut Auxhfrcoctrl
    }

    #[doc="Get the *const pointer for the AUXHFRCOCTRL register."]
    #[inline] pub fn auxhfrcoctrl_ptr(&self) -> *const Auxhfrcoctrl { 
           self.auxhfrcoctrl_mut()
    }

    #[doc="Read the AUXHFRCOCTRL register."]
    #[inline] pub fn auxhfrcoctrl(&self) -> Auxhfrcoctrl { 
        unsafe {
            read_volatile(self.auxhfrcoctrl_ptr())
        }
    }

    #[doc="Write the AUXHFRCOCTRL register."]
    #[inline] pub fn set_auxhfrcoctrl<F: FnOnce(Auxhfrcoctrl) -> Auxhfrcoctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.auxhfrcoctrl_mut(), f(Auxhfrcoctrl(0)));
        }
        self
    }

    #[doc="Modify the AUXHFRCOCTRL register."]
    #[inline] pub fn with_auxhfrcoctrl<F: FnOnce(Auxhfrcoctrl) -> Auxhfrcoctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.auxhfrcoctrl_mut(), f(self.auxhfrcoctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LFRCOCTRL register."]
    #[inline] pub fn lfrcoctrl_mut(&self) -> *mut Lfrcoctrl { 
        (self.0 + 0x20) as *mut Lfrcoctrl
    }

    #[doc="Get the *const pointer for the LFRCOCTRL register."]
    #[inline] pub fn lfrcoctrl_ptr(&self) -> *const Lfrcoctrl { 
           self.lfrcoctrl_mut()
    }

    #[doc="Read the LFRCOCTRL register."]
    #[inline] pub fn lfrcoctrl(&self) -> Lfrcoctrl { 
        unsafe {
            read_volatile(self.lfrcoctrl_ptr())
        }
    }

    #[doc="Write the LFRCOCTRL register."]
    #[inline] pub fn set_lfrcoctrl<F: FnOnce(Lfrcoctrl) -> Lfrcoctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lfrcoctrl_mut(), f(Lfrcoctrl(0)));
        }
        self
    }

    #[doc="Modify the LFRCOCTRL register."]
    #[inline] pub fn with_lfrcoctrl<F: FnOnce(Lfrcoctrl) -> Lfrcoctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lfrcoctrl_mut(), f(self.lfrcoctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HFXOCTRL register."]
    #[inline] pub fn hfxoctrl_mut(&self) -> *mut Hfxoctrl { 
        (self.0 + 0x24) as *mut Hfxoctrl
    }

    #[doc="Get the *const pointer for the HFXOCTRL register."]
    #[inline] pub fn hfxoctrl_ptr(&self) -> *const Hfxoctrl { 
           self.hfxoctrl_mut()
    }

    #[doc="Read the HFXOCTRL register."]
    #[inline] pub fn hfxoctrl(&self) -> Hfxoctrl { 
        unsafe {
            read_volatile(self.hfxoctrl_ptr())
        }
    }

    #[doc="Write the HFXOCTRL register."]
    #[inline] pub fn set_hfxoctrl<F: FnOnce(Hfxoctrl) -> Hfxoctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfxoctrl_mut(), f(Hfxoctrl(0)));
        }
        self
    }

    #[doc="Modify the HFXOCTRL register."]
    #[inline] pub fn with_hfxoctrl<F: FnOnce(Hfxoctrl) -> Hfxoctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfxoctrl_mut(), f(self.hfxoctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HFXOSTARTUPCTRL register."]
    #[inline] pub fn hfxostartupctrl_mut(&self) -> *mut Hfxostartupctrl { 
        (self.0 + 0x2c) as *mut Hfxostartupctrl
    }

    #[doc="Get the *const pointer for the HFXOSTARTUPCTRL register."]
    #[inline] pub fn hfxostartupctrl_ptr(&self) -> *const Hfxostartupctrl { 
           self.hfxostartupctrl_mut()
    }

    #[doc="Read the HFXOSTARTUPCTRL register."]
    #[inline] pub fn hfxostartupctrl(&self) -> Hfxostartupctrl { 
        unsafe {
            read_volatile(self.hfxostartupctrl_ptr())
        }
    }

    #[doc="Write the HFXOSTARTUPCTRL register."]
    #[inline] pub fn set_hfxostartupctrl<F: FnOnce(Hfxostartupctrl) -> Hfxostartupctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfxostartupctrl_mut(), f(Hfxostartupctrl(0)));
        }
        self
    }

    #[doc="Modify the HFXOSTARTUPCTRL register."]
    #[inline] pub fn with_hfxostartupctrl<F: FnOnce(Hfxostartupctrl) -> Hfxostartupctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfxostartupctrl_mut(), f(self.hfxostartupctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HFXOSTEADYSTATECTRL register."]
    #[inline] pub fn hfxosteadystatectrl_mut(&self) -> *mut Hfxosteadystatectrl { 
        (self.0 + 0x30) as *mut Hfxosteadystatectrl
    }

    #[doc="Get the *const pointer for the HFXOSTEADYSTATECTRL register."]
    #[inline] pub fn hfxosteadystatectrl_ptr(&self) -> *const Hfxosteadystatectrl { 
           self.hfxosteadystatectrl_mut()
    }

    #[doc="Read the HFXOSTEADYSTATECTRL register."]
    #[inline] pub fn hfxosteadystatectrl(&self) -> Hfxosteadystatectrl { 
        unsafe {
            read_volatile(self.hfxosteadystatectrl_ptr())
        }
    }

    #[doc="Write the HFXOSTEADYSTATECTRL register."]
    #[inline] pub fn set_hfxosteadystatectrl<F: FnOnce(Hfxosteadystatectrl) -> Hfxosteadystatectrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfxosteadystatectrl_mut(), f(Hfxosteadystatectrl(0)));
        }
        self
    }

    #[doc="Modify the HFXOSTEADYSTATECTRL register."]
    #[inline] pub fn with_hfxosteadystatectrl<F: FnOnce(Hfxosteadystatectrl) -> Hfxosteadystatectrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfxosteadystatectrl_mut(), f(self.hfxosteadystatectrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HFXOTIMEOUTCTRL register."]
    #[inline] pub fn hfxotimeoutctrl_mut(&self) -> *mut Hfxotimeoutctrl { 
        (self.0 + 0x34) as *mut Hfxotimeoutctrl
    }

    #[doc="Get the *const pointer for the HFXOTIMEOUTCTRL register."]
    #[inline] pub fn hfxotimeoutctrl_ptr(&self) -> *const Hfxotimeoutctrl { 
           self.hfxotimeoutctrl_mut()
    }

    #[doc="Read the HFXOTIMEOUTCTRL register."]
    #[inline] pub fn hfxotimeoutctrl(&self) -> Hfxotimeoutctrl { 
        unsafe {
            read_volatile(self.hfxotimeoutctrl_ptr())
        }
    }

    #[doc="Write the HFXOTIMEOUTCTRL register."]
    #[inline] pub fn set_hfxotimeoutctrl<F: FnOnce(Hfxotimeoutctrl) -> Hfxotimeoutctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfxotimeoutctrl_mut(), f(Hfxotimeoutctrl(0)));
        }
        self
    }

    #[doc="Modify the HFXOTIMEOUTCTRL register."]
    #[inline] pub fn with_hfxotimeoutctrl<F: FnOnce(Hfxotimeoutctrl) -> Hfxotimeoutctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfxotimeoutctrl_mut(), f(self.hfxotimeoutctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LFXOCTRL register."]
    #[inline] pub fn lfxoctrl_mut(&self) -> *mut Lfxoctrl { 
        (self.0 + 0x38) as *mut Lfxoctrl
    }

    #[doc="Get the *const pointer for the LFXOCTRL register."]
    #[inline] pub fn lfxoctrl_ptr(&self) -> *const Lfxoctrl { 
           self.lfxoctrl_mut()
    }

    #[doc="Read the LFXOCTRL register."]
    #[inline] pub fn lfxoctrl(&self) -> Lfxoctrl { 
        unsafe {
            read_volatile(self.lfxoctrl_ptr())
        }
    }

    #[doc="Write the LFXOCTRL register."]
    #[inline] pub fn set_lfxoctrl<F: FnOnce(Lfxoctrl) -> Lfxoctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lfxoctrl_mut(), f(Lfxoctrl(0)));
        }
        self
    }

    #[doc="Modify the LFXOCTRL register."]
    #[inline] pub fn with_lfxoctrl<F: FnOnce(Lfxoctrl) -> Lfxoctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lfxoctrl_mut(), f(self.lfxoctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DPLLCTRL register."]
    #[inline] pub fn dpllctrl_mut(&self) -> *mut Dpllctrl { 
        (self.0 + 0x40) as *mut Dpllctrl
    }

    #[doc="Get the *const pointer for the DPLLCTRL register."]
    #[inline] pub fn dpllctrl_ptr(&self) -> *const Dpllctrl { 
           self.dpllctrl_mut()
    }

    #[doc="Read the DPLLCTRL register."]
    #[inline] pub fn dpllctrl(&self) -> Dpllctrl { 
        unsafe {
            read_volatile(self.dpllctrl_ptr())
        }
    }

    #[doc="Write the DPLLCTRL register."]
    #[inline] pub fn set_dpllctrl<F: FnOnce(Dpllctrl) -> Dpllctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dpllctrl_mut(), f(Dpllctrl(0)));
        }
        self
    }

    #[doc="Modify the DPLLCTRL register."]
    #[inline] pub fn with_dpllctrl<F: FnOnce(Dpllctrl) -> Dpllctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dpllctrl_mut(), f(self.dpllctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DPLLCTRL1 register."]
    #[inline] pub fn dpllctrl1_mut(&self) -> *mut Dpllctrl1 { 
        (self.0 + 0x44) as *mut Dpllctrl1
    }

    #[doc="Get the *const pointer for the DPLLCTRL1 register."]
    #[inline] pub fn dpllctrl1_ptr(&self) -> *const Dpllctrl1 { 
           self.dpllctrl1_mut()
    }

    #[doc="Read the DPLLCTRL1 register."]
    #[inline] pub fn dpllctrl1(&self) -> Dpllctrl1 { 
        unsafe {
            read_volatile(self.dpllctrl1_ptr())
        }
    }

    #[doc="Write the DPLLCTRL1 register."]
    #[inline] pub fn set_dpllctrl1<F: FnOnce(Dpllctrl1) -> Dpllctrl1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dpllctrl1_mut(), f(Dpllctrl1(0)));
        }
        self
    }

    #[doc="Modify the DPLLCTRL1 register."]
    #[inline] pub fn with_dpllctrl1<F: FnOnce(Dpllctrl1) -> Dpllctrl1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dpllctrl1_mut(), f(self.dpllctrl1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CALCTRL register."]
    #[inline] pub fn calctrl_mut(&self) -> *mut Calctrl { 
        (self.0 + 0x50) as *mut Calctrl
    }

    #[doc="Get the *const pointer for the CALCTRL register."]
    #[inline] pub fn calctrl_ptr(&self) -> *const Calctrl { 
           self.calctrl_mut()
    }

    #[doc="Read the CALCTRL register."]
    #[inline] pub fn calctrl(&self) -> Calctrl { 
        unsafe {
            read_volatile(self.calctrl_ptr())
        }
    }

    #[doc="Write the CALCTRL register."]
    #[inline] pub fn set_calctrl<F: FnOnce(Calctrl) -> Calctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.calctrl_mut(), f(Calctrl(0)));
        }
        self
    }

    #[doc="Modify the CALCTRL register."]
    #[inline] pub fn with_calctrl<F: FnOnce(Calctrl) -> Calctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.calctrl_mut(), f(self.calctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CALCNT register."]
    #[inline] pub fn calcnt_mut(&self) -> *mut Calcnt { 
        (self.0 + 0x54) as *mut Calcnt
    }

    #[doc="Get the *const pointer for the CALCNT register."]
    #[inline] pub fn calcnt_ptr(&self) -> *const Calcnt { 
           self.calcnt_mut()
    }

    #[doc="Read the CALCNT register."]
    #[inline] pub fn calcnt(&self) -> Calcnt { 
        unsafe {
            read_volatile(self.calcnt_ptr())
        }
    }

    #[doc="Write the CALCNT register."]
    #[inline] pub fn set_calcnt<F: FnOnce(Calcnt) -> Calcnt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.calcnt_mut(), f(Calcnt(0)));
        }
        self
    }

    #[doc="Modify the CALCNT register."]
    #[inline] pub fn with_calcnt<F: FnOnce(Calcnt) -> Calcnt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.calcnt_mut(), f(self.calcnt()));
        }
        self
    }

    #[doc="Get the *mut pointer for the OSCENCMD register."]
    #[inline] pub fn oscencmd_mut(&self) -> *mut Oscencmd { 
        (self.0 + 0x60) as *mut Oscencmd
    }

    #[doc="Get the *const pointer for the OSCENCMD register."]
    #[inline] pub fn oscencmd_ptr(&self) -> *const Oscencmd { 
           self.oscencmd_mut()
    }

    #[doc="Write the OSCENCMD register."]
    #[inline] pub fn set_oscencmd<F: FnOnce(Oscencmd) -> Oscencmd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.oscencmd_mut(), f(Oscencmd(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the CMD register."]
    #[inline] pub fn cmd_mut(&self) -> *mut Cmd { 
        (self.0 + 0x64) as *mut Cmd
    }

    #[doc="Get the *const pointer for the CMD register."]
    #[inline] pub fn cmd_ptr(&self) -> *const Cmd { 
           self.cmd_mut()
    }

    #[doc="Write the CMD register."]
    #[inline] pub fn set_cmd<F: FnOnce(Cmd) -> Cmd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmd_mut(), f(Cmd(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the DBGCLKSEL register."]
    #[inline] pub fn dbgclksel_mut(&self) -> *mut Dbgclksel { 
        (self.0 + 0x70) as *mut Dbgclksel
    }

    #[doc="Get the *const pointer for the DBGCLKSEL register."]
    #[inline] pub fn dbgclksel_ptr(&self) -> *const Dbgclksel { 
           self.dbgclksel_mut()
    }

    #[doc="Read the DBGCLKSEL register."]
    #[inline] pub fn dbgclksel(&self) -> Dbgclksel { 
        unsafe {
            read_volatile(self.dbgclksel_ptr())
        }
    }

    #[doc="Write the DBGCLKSEL register."]
    #[inline] pub fn set_dbgclksel<F: FnOnce(Dbgclksel) -> Dbgclksel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dbgclksel_mut(), f(Dbgclksel(0)));
        }
        self
    }

    #[doc="Modify the DBGCLKSEL register."]
    #[inline] pub fn with_dbgclksel<F: FnOnce(Dbgclksel) -> Dbgclksel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dbgclksel_mut(), f(self.dbgclksel()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HFCLKSEL register."]
    #[inline] pub fn hfclksel_mut(&self) -> *mut Hfclksel { 
        (self.0 + 0x74) as *mut Hfclksel
    }

    #[doc="Get the *const pointer for the HFCLKSEL register."]
    #[inline] pub fn hfclksel_ptr(&self) -> *const Hfclksel { 
           self.hfclksel_mut()
    }

    #[doc="Write the HFCLKSEL register."]
    #[inline] pub fn set_hfclksel<F: FnOnce(Hfclksel) -> Hfclksel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfclksel_mut(), f(Hfclksel(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the LFACLKSEL register."]
    #[inline] pub fn lfaclksel_mut(&self) -> *mut Lfaclksel { 
        (self.0 + 0x80) as *mut Lfaclksel
    }

    #[doc="Get the *const pointer for the LFACLKSEL register."]
    #[inline] pub fn lfaclksel_ptr(&self) -> *const Lfaclksel { 
           self.lfaclksel_mut()
    }

    #[doc="Read the LFACLKSEL register."]
    #[inline] pub fn lfaclksel(&self) -> Lfaclksel { 
        unsafe {
            read_volatile(self.lfaclksel_ptr())
        }
    }

    #[doc="Write the LFACLKSEL register."]
    #[inline] pub fn set_lfaclksel<F: FnOnce(Lfaclksel) -> Lfaclksel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lfaclksel_mut(), f(Lfaclksel(0)));
        }
        self
    }

    #[doc="Modify the LFACLKSEL register."]
    #[inline] pub fn with_lfaclksel<F: FnOnce(Lfaclksel) -> Lfaclksel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lfaclksel_mut(), f(self.lfaclksel()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LFBCLKSEL register."]
    #[inline] pub fn lfbclksel_mut(&self) -> *mut Lfbclksel { 
        (self.0 + 0x84) as *mut Lfbclksel
    }

    #[doc="Get the *const pointer for the LFBCLKSEL register."]
    #[inline] pub fn lfbclksel_ptr(&self) -> *const Lfbclksel { 
           self.lfbclksel_mut()
    }

    #[doc="Read the LFBCLKSEL register."]
    #[inline] pub fn lfbclksel(&self) -> Lfbclksel { 
        unsafe {
            read_volatile(self.lfbclksel_ptr())
        }
    }

    #[doc="Write the LFBCLKSEL register."]
    #[inline] pub fn set_lfbclksel<F: FnOnce(Lfbclksel) -> Lfbclksel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lfbclksel_mut(), f(Lfbclksel(0)));
        }
        self
    }

    #[doc="Modify the LFBCLKSEL register."]
    #[inline] pub fn with_lfbclksel<F: FnOnce(Lfbclksel) -> Lfbclksel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lfbclksel_mut(), f(self.lfbclksel()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LFECLKSEL register."]
    #[inline] pub fn lfeclksel_mut(&self) -> *mut Lfeclksel { 
        (self.0 + 0x88) as *mut Lfeclksel
    }

    #[doc="Get the *const pointer for the LFECLKSEL register."]
    #[inline] pub fn lfeclksel_ptr(&self) -> *const Lfeclksel { 
           self.lfeclksel_mut()
    }

    #[doc="Read the LFECLKSEL register."]
    #[inline] pub fn lfeclksel(&self) -> Lfeclksel { 
        unsafe {
            read_volatile(self.lfeclksel_ptr())
        }
    }

    #[doc="Write the LFECLKSEL register."]
    #[inline] pub fn set_lfeclksel<F: FnOnce(Lfeclksel) -> Lfeclksel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lfeclksel_mut(), f(Lfeclksel(0)));
        }
        self
    }

    #[doc="Modify the LFECLKSEL register."]
    #[inline] pub fn with_lfeclksel<F: FnOnce(Lfeclksel) -> Lfeclksel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lfeclksel_mut(), f(self.lfeclksel()));
        }
        self
    }

    #[doc="Get the *mut pointer for the STATUS register."]
    #[inline] pub fn status_mut(&self) -> *mut Status { 
        (self.0 + 0x90) as *mut Status
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

    #[doc="Get the *mut pointer for the HFCLKSTATUS register."]
    #[inline] pub fn hfclkstatus_mut(&self) -> *mut Hfclkstatus { 
        (self.0 + 0x94) as *mut Hfclkstatus
    }

    #[doc="Get the *const pointer for the HFCLKSTATUS register."]
    #[inline] pub fn hfclkstatus_ptr(&self) -> *const Hfclkstatus { 
           self.hfclkstatus_mut()
    }

    #[doc="Read the HFCLKSTATUS register."]
    #[inline] pub fn hfclkstatus(&self) -> Hfclkstatus { 
        unsafe {
            read_volatile(self.hfclkstatus_ptr())
        }
    }

    #[doc="Get the *mut pointer for the HFXOTRIMSTATUS register."]
    #[inline] pub fn hfxotrimstatus_mut(&self) -> *mut Hfxotrimstatus { 
        (self.0 + 0x9c) as *mut Hfxotrimstatus
    }

    #[doc="Get the *const pointer for the HFXOTRIMSTATUS register."]
    #[inline] pub fn hfxotrimstatus_ptr(&self) -> *const Hfxotrimstatus { 
           self.hfxotrimstatus_mut()
    }

    #[doc="Read the HFXOTRIMSTATUS register."]
    #[inline] pub fn hfxotrimstatus(&self) -> Hfxotrimstatus { 
        unsafe {
            read_volatile(self.hfxotrimstatus_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IF register."]
    #[inline] pub fn if_mut(&self) -> *mut If { 
        (self.0 + 0xa0) as *mut If
    }

    #[doc="Get the *const pointer for the IF register."]
    #[inline] pub fn if_ptr(&self) -> *const If { 
           self.if_mut()
    }

    #[doc="Read the IF register."]
    #[inline] pub fn _if(&self) -> If { 
        unsafe {
            read_volatile(self.if_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IFS register."]
    #[inline] pub fn ifs_mut(&self) -> *mut Ifs { 
        (self.0 + 0xa4) as *mut Ifs
    }

    #[doc="Get the *const pointer for the IFS register."]
    #[inline] pub fn ifs_ptr(&self) -> *const Ifs { 
           self.ifs_mut()
    }

    #[doc="Write the IFS register."]
    #[inline] pub fn set_ifs<F: FnOnce(Ifs) -> Ifs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ifs_mut(), f(Ifs(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the IFC register."]
    #[inline] pub fn ifc_mut(&self) -> *mut Ifc { 
        (self.0 + 0xa8) as *mut Ifc
    }

    #[doc="Get the *const pointer for the IFC register."]
    #[inline] pub fn ifc_ptr(&self) -> *const Ifc { 
           self.ifc_mut()
    }

    #[doc="Write the IFC register."]
    #[inline] pub fn set_ifc<F: FnOnce(Ifc) -> Ifc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ifc_mut(), f(Ifc(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the IEN register."]
    #[inline] pub fn ien_mut(&self) -> *mut Ien { 
        (self.0 + 0xac) as *mut Ien
    }

    #[doc="Get the *const pointer for the IEN register."]
    #[inline] pub fn ien_ptr(&self) -> *const Ien { 
           self.ien_mut()
    }

    #[doc="Read the IEN register."]
    #[inline] pub fn ien(&self) -> Ien { 
        unsafe {
            read_volatile(self.ien_ptr())
        }
    }

    #[doc="Write the IEN register."]
    #[inline] pub fn set_ien<F: FnOnce(Ien) -> Ien>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ien_mut(), f(Ien(0)));
        }
        self
    }

    #[doc="Modify the IEN register."]
    #[inline] pub fn with_ien<F: FnOnce(Ien) -> Ien>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ien_mut(), f(self.ien()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HFBUSCLKEN0 register."]
    #[inline] pub fn hfbusclken0_mut(&self) -> *mut Hfbusclken0 { 
        (self.0 + 0xb0) as *mut Hfbusclken0
    }

    #[doc="Get the *const pointer for the HFBUSCLKEN0 register."]
    #[inline] pub fn hfbusclken0_ptr(&self) -> *const Hfbusclken0 { 
           self.hfbusclken0_mut()
    }

    #[doc="Read the HFBUSCLKEN0 register."]
    #[inline] pub fn hfbusclken0(&self) -> Hfbusclken0 { 
        unsafe {
            read_volatile(self.hfbusclken0_ptr())
        }
    }

    #[doc="Write the HFBUSCLKEN0 register."]
    #[inline] pub fn set_hfbusclken0<F: FnOnce(Hfbusclken0) -> Hfbusclken0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfbusclken0_mut(), f(Hfbusclken0(0)));
        }
        self
    }

    #[doc="Modify the HFBUSCLKEN0 register."]
    #[inline] pub fn with_hfbusclken0<F: FnOnce(Hfbusclken0) -> Hfbusclken0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfbusclken0_mut(), f(self.hfbusclken0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HFPERCLKEN0 register."]
    #[inline] pub fn hfperclken0_mut(&self) -> *mut Hfperclken0 { 
        (self.0 + 0xc0) as *mut Hfperclken0
    }

    #[doc="Get the *const pointer for the HFPERCLKEN0 register."]
    #[inline] pub fn hfperclken0_ptr(&self) -> *const Hfperclken0 { 
           self.hfperclken0_mut()
    }

    #[doc="Read the HFPERCLKEN0 register."]
    #[inline] pub fn hfperclken0(&self) -> Hfperclken0 { 
        unsafe {
            read_volatile(self.hfperclken0_ptr())
        }
    }

    #[doc="Write the HFPERCLKEN0 register."]
    #[inline] pub fn set_hfperclken0<F: FnOnce(Hfperclken0) -> Hfperclken0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfperclken0_mut(), f(Hfperclken0(0)));
        }
        self
    }

    #[doc="Modify the HFPERCLKEN0 register."]
    #[inline] pub fn with_hfperclken0<F: FnOnce(Hfperclken0) -> Hfperclken0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfperclken0_mut(), f(self.hfperclken0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LFACLKEN0 register."]
    #[inline] pub fn lfaclken0_mut(&self) -> *mut Lfaclken0 { 
        (self.0 + 0xe0) as *mut Lfaclken0
    }

    #[doc="Get the *const pointer for the LFACLKEN0 register."]
    #[inline] pub fn lfaclken0_ptr(&self) -> *const Lfaclken0 { 
           self.lfaclken0_mut()
    }

    #[doc="Read the LFACLKEN0 register."]
    #[inline] pub fn lfaclken0(&self) -> Lfaclken0 { 
        unsafe {
            read_volatile(self.lfaclken0_ptr())
        }
    }

    #[doc="Write the LFACLKEN0 register."]
    #[inline] pub fn set_lfaclken0<F: FnOnce(Lfaclken0) -> Lfaclken0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lfaclken0_mut(), f(Lfaclken0(0)));
        }
        self
    }

    #[doc="Modify the LFACLKEN0 register."]
    #[inline] pub fn with_lfaclken0<F: FnOnce(Lfaclken0) -> Lfaclken0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lfaclken0_mut(), f(self.lfaclken0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LFBCLKEN0 register."]
    #[inline] pub fn lfbclken0_mut(&self) -> *mut Lfbclken0 { 
        (self.0 + 0xe8) as *mut Lfbclken0
    }

    #[doc="Get the *const pointer for the LFBCLKEN0 register."]
    #[inline] pub fn lfbclken0_ptr(&self) -> *const Lfbclken0 { 
           self.lfbclken0_mut()
    }

    #[doc="Read the LFBCLKEN0 register."]
    #[inline] pub fn lfbclken0(&self) -> Lfbclken0 { 
        unsafe {
            read_volatile(self.lfbclken0_ptr())
        }
    }

    #[doc="Write the LFBCLKEN0 register."]
    #[inline] pub fn set_lfbclken0<F: FnOnce(Lfbclken0) -> Lfbclken0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lfbclken0_mut(), f(Lfbclken0(0)));
        }
        self
    }

    #[doc="Modify the LFBCLKEN0 register."]
    #[inline] pub fn with_lfbclken0<F: FnOnce(Lfbclken0) -> Lfbclken0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lfbclken0_mut(), f(self.lfbclken0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LFECLKEN0 register."]
    #[inline] pub fn lfeclken0_mut(&self) -> *mut Lfeclken0 { 
        (self.0 + 0xf0) as *mut Lfeclken0
    }

    #[doc="Get the *const pointer for the LFECLKEN0 register."]
    #[inline] pub fn lfeclken0_ptr(&self) -> *const Lfeclken0 { 
           self.lfeclken0_mut()
    }

    #[doc="Read the LFECLKEN0 register."]
    #[inline] pub fn lfeclken0(&self) -> Lfeclken0 { 
        unsafe {
            read_volatile(self.lfeclken0_ptr())
        }
    }

    #[doc="Write the LFECLKEN0 register."]
    #[inline] pub fn set_lfeclken0<F: FnOnce(Lfeclken0) -> Lfeclken0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lfeclken0_mut(), f(Lfeclken0(0)));
        }
        self
    }

    #[doc="Modify the LFECLKEN0 register."]
    #[inline] pub fn with_lfeclken0<F: FnOnce(Lfeclken0) -> Lfeclken0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lfeclken0_mut(), f(self.lfeclken0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HFPRESC register."]
    #[inline] pub fn hfpresc_mut(&self) -> *mut Hfpresc { 
        (self.0 + 0x100) as *mut Hfpresc
    }

    #[doc="Get the *const pointer for the HFPRESC register."]
    #[inline] pub fn hfpresc_ptr(&self) -> *const Hfpresc { 
           self.hfpresc_mut()
    }

    #[doc="Read the HFPRESC register."]
    #[inline] pub fn hfpresc(&self) -> Hfpresc { 
        unsafe {
            read_volatile(self.hfpresc_ptr())
        }
    }

    #[doc="Write the HFPRESC register."]
    #[inline] pub fn set_hfpresc<F: FnOnce(Hfpresc) -> Hfpresc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfpresc_mut(), f(Hfpresc(0)));
        }
        self
    }

    #[doc="Modify the HFPRESC register."]
    #[inline] pub fn with_hfpresc<F: FnOnce(Hfpresc) -> Hfpresc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfpresc_mut(), f(self.hfpresc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HFCOREPRESC register."]
    #[inline] pub fn hfcorepresc_mut(&self) -> *mut Hfcorepresc { 
        (self.0 + 0x108) as *mut Hfcorepresc
    }

    #[doc="Get the *const pointer for the HFCOREPRESC register."]
    #[inline] pub fn hfcorepresc_ptr(&self) -> *const Hfcorepresc { 
           self.hfcorepresc_mut()
    }

    #[doc="Read the HFCOREPRESC register."]
    #[inline] pub fn hfcorepresc(&self) -> Hfcorepresc { 
        unsafe {
            read_volatile(self.hfcorepresc_ptr())
        }
    }

    #[doc="Write the HFCOREPRESC register."]
    #[inline] pub fn set_hfcorepresc<F: FnOnce(Hfcorepresc) -> Hfcorepresc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfcorepresc_mut(), f(Hfcorepresc(0)));
        }
        self
    }

    #[doc="Modify the HFCOREPRESC register."]
    #[inline] pub fn with_hfcorepresc<F: FnOnce(Hfcorepresc) -> Hfcorepresc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfcorepresc_mut(), f(self.hfcorepresc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HFPERPRESC register."]
    #[inline] pub fn hfperpresc_mut(&self) -> *mut Hfperpresc { 
        (self.0 + 0x10c) as *mut Hfperpresc
    }

    #[doc="Get the *const pointer for the HFPERPRESC register."]
    #[inline] pub fn hfperpresc_ptr(&self) -> *const Hfperpresc { 
           self.hfperpresc_mut()
    }

    #[doc="Read the HFPERPRESC register."]
    #[inline] pub fn hfperpresc(&self) -> Hfperpresc { 
        unsafe {
            read_volatile(self.hfperpresc_ptr())
        }
    }

    #[doc="Write the HFPERPRESC register."]
    #[inline] pub fn set_hfperpresc<F: FnOnce(Hfperpresc) -> Hfperpresc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfperpresc_mut(), f(Hfperpresc(0)));
        }
        self
    }

    #[doc="Modify the HFPERPRESC register."]
    #[inline] pub fn with_hfperpresc<F: FnOnce(Hfperpresc) -> Hfperpresc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfperpresc_mut(), f(self.hfperpresc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HFEXPPRESC register."]
    #[inline] pub fn hfexppresc_mut(&self) -> *mut Hfexppresc { 
        (self.0 + 0x114) as *mut Hfexppresc
    }

    #[doc="Get the *const pointer for the HFEXPPRESC register."]
    #[inline] pub fn hfexppresc_ptr(&self) -> *const Hfexppresc { 
           self.hfexppresc_mut()
    }

    #[doc="Read the HFEXPPRESC register."]
    #[inline] pub fn hfexppresc(&self) -> Hfexppresc { 
        unsafe {
            read_volatile(self.hfexppresc_ptr())
        }
    }

    #[doc="Write the HFEXPPRESC register."]
    #[inline] pub fn set_hfexppresc<F: FnOnce(Hfexppresc) -> Hfexppresc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfexppresc_mut(), f(Hfexppresc(0)));
        }
        self
    }

    #[doc="Modify the HFEXPPRESC register."]
    #[inline] pub fn with_hfexppresc<F: FnOnce(Hfexppresc) -> Hfexppresc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfexppresc_mut(), f(self.hfexppresc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LFAPRESC0 register."]
    #[inline] pub fn lfapresc0_mut(&self) -> *mut Lfapresc0 { 
        (self.0 + 0x120) as *mut Lfapresc0
    }

    #[doc="Get the *const pointer for the LFAPRESC0 register."]
    #[inline] pub fn lfapresc0_ptr(&self) -> *const Lfapresc0 { 
           self.lfapresc0_mut()
    }

    #[doc="Read the LFAPRESC0 register."]
    #[inline] pub fn lfapresc0(&self) -> Lfapresc0 { 
        unsafe {
            read_volatile(self.lfapresc0_ptr())
        }
    }

    #[doc="Write the LFAPRESC0 register."]
    #[inline] pub fn set_lfapresc0<F: FnOnce(Lfapresc0) -> Lfapresc0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lfapresc0_mut(), f(Lfapresc0(0)));
        }
        self
    }

    #[doc="Modify the LFAPRESC0 register."]
    #[inline] pub fn with_lfapresc0<F: FnOnce(Lfapresc0) -> Lfapresc0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lfapresc0_mut(), f(self.lfapresc0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LFBPRESC0 register."]
    #[inline] pub fn lfbpresc0_mut(&self) -> *mut Lfbpresc0 { 
        (self.0 + 0x128) as *mut Lfbpresc0
    }

    #[doc="Get the *const pointer for the LFBPRESC0 register."]
    #[inline] pub fn lfbpresc0_ptr(&self) -> *const Lfbpresc0 { 
           self.lfbpresc0_mut()
    }

    #[doc="Read the LFBPRESC0 register."]
    #[inline] pub fn lfbpresc0(&self) -> Lfbpresc0 { 
        unsafe {
            read_volatile(self.lfbpresc0_ptr())
        }
    }

    #[doc="Write the LFBPRESC0 register."]
    #[inline] pub fn set_lfbpresc0<F: FnOnce(Lfbpresc0) -> Lfbpresc0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lfbpresc0_mut(), f(Lfbpresc0(0)));
        }
        self
    }

    #[doc="Modify the LFBPRESC0 register."]
    #[inline] pub fn with_lfbpresc0<F: FnOnce(Lfbpresc0) -> Lfbpresc0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lfbpresc0_mut(), f(self.lfbpresc0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LFEPRESC0 register."]
    #[inline] pub fn lfepresc0_mut(&self) -> *mut Lfepresc0 { 
        (self.0 + 0x130) as *mut Lfepresc0
    }

    #[doc="Get the *const pointer for the LFEPRESC0 register."]
    #[inline] pub fn lfepresc0_ptr(&self) -> *const Lfepresc0 { 
           self.lfepresc0_mut()
    }

    #[doc="Read the LFEPRESC0 register."]
    #[inline] pub fn lfepresc0(&self) -> Lfepresc0 { 
        unsafe {
            read_volatile(self.lfepresc0_ptr())
        }
    }

    #[doc="Write the LFEPRESC0 register."]
    #[inline] pub fn set_lfepresc0<F: FnOnce(Lfepresc0) -> Lfepresc0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lfepresc0_mut(), f(Lfepresc0(0)));
        }
        self
    }

    #[doc="Modify the LFEPRESC0 register."]
    #[inline] pub fn with_lfepresc0<F: FnOnce(Lfepresc0) -> Lfepresc0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lfepresc0_mut(), f(self.lfepresc0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SYNCBUSY register."]
    #[inline] pub fn syncbusy_mut(&self) -> *mut Syncbusy { 
        (self.0 + 0x140) as *mut Syncbusy
    }

    #[doc="Get the *const pointer for the SYNCBUSY register."]
    #[inline] pub fn syncbusy_ptr(&self) -> *const Syncbusy { 
           self.syncbusy_mut()
    }

    #[doc="Read the SYNCBUSY register."]
    #[inline] pub fn syncbusy(&self) -> Syncbusy { 
        unsafe {
            read_volatile(self.syncbusy_ptr())
        }
    }

    #[doc="Get the *mut pointer for the FREEZE register."]
    #[inline] pub fn freeze_mut(&self) -> *mut Freeze { 
        (self.0 + 0x144) as *mut Freeze
    }

    #[doc="Get the *const pointer for the FREEZE register."]
    #[inline] pub fn freeze_ptr(&self) -> *const Freeze { 
           self.freeze_mut()
    }

    #[doc="Read the FREEZE register."]
    #[inline] pub fn freeze(&self) -> Freeze { 
        unsafe {
            read_volatile(self.freeze_ptr())
        }
    }

    #[doc="Write the FREEZE register."]
    #[inline] pub fn set_freeze<F: FnOnce(Freeze) -> Freeze>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.freeze_mut(), f(Freeze(0)));
        }
        self
    }

    #[doc="Modify the FREEZE register."]
    #[inline] pub fn with_freeze<F: FnOnce(Freeze) -> Freeze>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.freeze_mut(), f(self.freeze()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCNTCTRL register."]
    #[inline] pub fn pcntctrl_mut(&self) -> *mut Pcntctrl { 
        (self.0 + 0x150) as *mut Pcntctrl
    }

    #[doc="Get the *const pointer for the PCNTCTRL register."]
    #[inline] pub fn pcntctrl_ptr(&self) -> *const Pcntctrl { 
           self.pcntctrl_mut()
    }

    #[doc="Read the PCNTCTRL register."]
    #[inline] pub fn pcntctrl(&self) -> Pcntctrl { 
        unsafe {
            read_volatile(self.pcntctrl_ptr())
        }
    }

    #[doc="Write the PCNTCTRL register."]
    #[inline] pub fn set_pcntctrl<F: FnOnce(Pcntctrl) -> Pcntctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pcntctrl_mut(), f(Pcntctrl(0)));
        }
        self
    }

    #[doc="Modify the PCNTCTRL register."]
    #[inline] pub fn with_pcntctrl<F: FnOnce(Pcntctrl) -> Pcntctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pcntctrl_mut(), f(self.pcntctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ADCCTRL register."]
    #[inline] pub fn adcctrl_mut(&self) -> *mut Adcctrl { 
        (self.0 + 0x15c) as *mut Adcctrl
    }

    #[doc="Get the *const pointer for the ADCCTRL register."]
    #[inline] pub fn adcctrl_ptr(&self) -> *const Adcctrl { 
           self.adcctrl_mut()
    }

    #[doc="Read the ADCCTRL register."]
    #[inline] pub fn adcctrl(&self) -> Adcctrl { 
        unsafe {
            read_volatile(self.adcctrl_ptr())
        }
    }

    #[doc="Write the ADCCTRL register."]
    #[inline] pub fn set_adcctrl<F: FnOnce(Adcctrl) -> Adcctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.adcctrl_mut(), f(Adcctrl(0)));
        }
        self
    }

    #[doc="Modify the ADCCTRL register."]
    #[inline] pub fn with_adcctrl<F: FnOnce(Adcctrl) -> Adcctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.adcctrl_mut(), f(self.adcctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ROUTEPEN register."]
    #[inline] pub fn routepen_mut(&self) -> *mut Routepen { 
        (self.0 + 0x170) as *mut Routepen
    }

    #[doc="Get the *const pointer for the ROUTEPEN register."]
    #[inline] pub fn routepen_ptr(&self) -> *const Routepen { 
           self.routepen_mut()
    }

    #[doc="Read the ROUTEPEN register."]
    #[inline] pub fn routepen(&self) -> Routepen { 
        unsafe {
            read_volatile(self.routepen_ptr())
        }
    }

    #[doc="Write the ROUTEPEN register."]
    #[inline] pub fn set_routepen<F: FnOnce(Routepen) -> Routepen>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.routepen_mut(), f(Routepen(0)));
        }
        self
    }

    #[doc="Modify the ROUTEPEN register."]
    #[inline] pub fn with_routepen<F: FnOnce(Routepen) -> Routepen>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.routepen_mut(), f(self.routepen()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ROUTELOC0 register."]
    #[inline] pub fn routeloc0_mut(&self) -> *mut Routeloc0 { 
        (self.0 + 0x174) as *mut Routeloc0
    }

    #[doc="Get the *const pointer for the ROUTELOC0 register."]
    #[inline] pub fn routeloc0_ptr(&self) -> *const Routeloc0 { 
           self.routeloc0_mut()
    }

    #[doc="Read the ROUTELOC0 register."]
    #[inline] pub fn routeloc0(&self) -> Routeloc0 { 
        unsafe {
            read_volatile(self.routeloc0_ptr())
        }
    }

    #[doc="Write the ROUTELOC0 register."]
    #[inline] pub fn set_routeloc0<F: FnOnce(Routeloc0) -> Routeloc0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.routeloc0_mut(), f(Routeloc0(0)));
        }
        self
    }

    #[doc="Modify the ROUTELOC0 register."]
    #[inline] pub fn with_routeloc0<F: FnOnce(Routeloc0) -> Routeloc0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.routeloc0_mut(), f(self.routeloc0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ROUTELOC1 register."]
    #[inline] pub fn routeloc1_mut(&self) -> *mut Routeloc1 { 
        (self.0 + 0x178) as *mut Routeloc1
    }

    #[doc="Get the *const pointer for the ROUTELOC1 register."]
    #[inline] pub fn routeloc1_ptr(&self) -> *const Routeloc1 { 
           self.routeloc1_mut()
    }

    #[doc="Read the ROUTELOC1 register."]
    #[inline] pub fn routeloc1(&self) -> Routeloc1 { 
        unsafe {
            read_volatile(self.routeloc1_ptr())
        }
    }

    #[doc="Write the ROUTELOC1 register."]
    #[inline] pub fn set_routeloc1<F: FnOnce(Routeloc1) -> Routeloc1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.routeloc1_mut(), f(Routeloc1(0)));
        }
        self
    }

    #[doc="Modify the ROUTELOC1 register."]
    #[inline] pub fn with_routeloc1<F: FnOnce(Routeloc1) -> Routeloc1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.routeloc1_mut(), f(self.routeloc1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LOCK register."]
    #[inline] pub fn lock_mut(&self) -> *mut Lock { 
        (self.0 + 0x180) as *mut Lock
    }

    #[doc="Get the *const pointer for the LOCK register."]
    #[inline] pub fn lock_ptr(&self) -> *const Lock { 
           self.lock_mut()
    }

    #[doc="Read the LOCK register."]
    #[inline] pub fn lock(&self) -> Lock { 
        unsafe {
            read_volatile(self.lock_ptr())
        }
    }

    #[doc="Write the LOCK register."]
    #[inline] pub fn set_lock<F: FnOnce(Lock) -> Lock>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lock_mut(), f(Lock(0)));
        }
        self
    }

    #[doc="Modify the LOCK register."]
    #[inline] pub fn with_lock<F: FnOnce(Lock) -> Lock>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lock_mut(), f(self.lock()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HFRCOSS register."]
    #[inline] pub fn hfrcoss_mut(&self) -> *mut Hfrcoss { 
        (self.0 + 0x184) as *mut Hfrcoss
    }

    #[doc="Get the *const pointer for the HFRCOSS register."]
    #[inline] pub fn hfrcoss_ptr(&self) -> *const Hfrcoss { 
           self.hfrcoss_mut()
    }

    #[doc="Read the HFRCOSS register."]
    #[inline] pub fn hfrcoss(&self) -> Hfrcoss { 
        unsafe {
            read_volatile(self.hfrcoss_ptr())
        }
    }

    #[doc="Write the HFRCOSS register."]
    #[inline] pub fn set_hfrcoss<F: FnOnce(Hfrcoss) -> Hfrcoss>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfrcoss_mut(), f(Hfrcoss(0)));
        }
        self
    }

    #[doc="Modify the HFRCOSS register."]
    #[inline] pub fn with_hfrcoss<F: FnOnce(Hfrcoss) -> Hfrcoss>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfrcoss_mut(), f(self.hfrcoss()));
        }
        self
    }

}

#[doc="CMU Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc="Clock Output Select 0"]
    #[inline] pub fn clkoutsel0(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CLKOUTSEL0 != 0"]
    #[inline] pub fn test_clkoutsel0(&self) -> bool {
        self.clkoutsel0() != 0
    }

    #[doc="Sets the CLKOUTSEL0 field."]
    #[inline] pub fn set_clkoutsel0<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clock Output Select 1"]
    #[inline] pub fn clkoutsel1(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0xf) as u8) } // [8:5]
    }

    #[doc="Returns true if CLKOUTSEL1 != 0"]
    #[inline] pub fn test_clkoutsel1(&self) -> bool {
        self.clkoutsel1() != 0
    }

    #[doc="Sets the CLKOUTSEL1 field."]
    #[inline] pub fn set_clkoutsel1<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Wait State for High-Frequency LE Interface"]
    #[inline] pub fn wshfle(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if WSHFLE != 0"]
    #[inline] pub fn test_wshfle(&self) -> bool {
        self.wshfle() != 0
    }

    #[doc="Sets the WSHFLE field."]
    #[inline] pub fn set_wshfle<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="HFPERCLK Enable"]
    #[inline] pub fn hfperclken(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if HFPERCLKEN != 0"]
    #[inline] pub fn test_hfperclken(&self) -> bool {
        self.hfperclken() != 0
    }

    #[doc="Sets the HFPERCLKEN field."]
    #[inline] pub fn set_hfperclken<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

}

impl From<u32> for Ctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Ctrl(other)
    }
}

impl ::core::fmt::Display for Ctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clkoutsel0() != 0 { try!(write!(f, " clkoutsel0=0x{:x}", self.clkoutsel0()))}
        if self.clkoutsel1() != 0 { try!(write!(f, " clkoutsel1=0x{:x}", self.clkoutsel1()))}
        if self.wshfle() != 0 { try!(write!(f, " wshfle"))}
        if self.hfperclken() != 0 { try!(write!(f, " hfperclken"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HFRCO Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hfrcoctrl(pub u32);
impl Hfrcoctrl {
    #[doc="HFRCO Tuning Value"]
    #[inline] pub fn tuning(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if TUNING != 0"]
    #[inline] pub fn test_tuning(&self) -> bool {
        self.tuning() != 0
    }

    #[doc="Sets the TUNING field."]
    #[inline] pub fn set_tuning<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="HFRCO Fine Tuning Value"]
    #[inline] pub fn finetuning(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3f) as u8) } // [13:8]
    }

    #[doc="Returns true if FINETUNING != 0"]
    #[inline] pub fn test_finetuning(&self) -> bool {
        self.finetuning() != 0
    }

    #[doc="Sets the FINETUNING field."]
    #[inline] pub fn set_finetuning<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="HFRCO Frequency Range"]
    #[inline] pub fn freqrange(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1f) as u8) } // [20:16]
    }

    #[doc="Returns true if FREQRANGE != 0"]
    #[inline] pub fn test_freqrange(&self) -> bool {
        self.freqrange() != 0
    }

    #[doc="Sets the FREQRANGE field."]
    #[inline] pub fn set_freqrange<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="HFRCO Comparator Bias Current"]
    #[inline] pub fn cmpbias(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x7) as u8) } // [23:21]
    }

    #[doc="Returns true if CMPBIAS != 0"]
    #[inline] pub fn test_cmpbias(&self) -> bool {
        self.cmpbias() != 0
    }

    #[doc="Sets the CMPBIAS field."]
    #[inline] pub fn set_cmpbias<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="HFRCO LDO High Power Mode"]
    #[inline] pub fn ldohp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if LDOHP != 0"]
    #[inline] pub fn test_ldohp(&self) -> bool {
        self.ldohp() != 0
    }

    #[doc="Sets the LDOHP field."]
    #[inline] pub fn set_ldohp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Locally divide HFRCO Clock Output"]
    #[inline] pub fn clkdiv(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x3) as u8) } // [26:25]
    }

    #[doc="Returns true if CLKDIV != 0"]
    #[inline] pub fn test_clkdiv(&self) -> bool {
        self.clkdiv() != 0
    }

    #[doc="Sets the CLKDIV field."]
    #[inline] pub fn set_clkdiv<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Enable reference for fine tuning"]
    #[inline] pub fn finetuningen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if FINETUNINGEN != 0"]
    #[inline] pub fn test_finetuningen(&self) -> bool {
        self.finetuningen() != 0
    }

    #[doc="Sets the FINETUNINGEN field."]
    #[inline] pub fn set_finetuningen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="HFRCO Temperature Coefficient Trim on Comparator Reference"]
    #[inline] pub fn vreftc(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0xf) as u8) } // [31:28]
    }

    #[doc="Returns true if VREFTC != 0"]
    #[inline] pub fn test_vreftc(&self) -> bool {
        self.vreftc() != 0
    }

    #[doc="Sets the VREFTC field."]
    #[inline] pub fn set_vreftc<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Hfrcoctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Hfrcoctrl(other)
    }
}

impl ::core::fmt::Display for Hfrcoctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hfrcoctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tuning() != 0 { try!(write!(f, " tuning=0x{:x}", self.tuning()))}
        if self.finetuning() != 0 { try!(write!(f, " finetuning=0x{:x}", self.finetuning()))}
        if self.freqrange() != 0 { try!(write!(f, " freqrange=0x{:x}", self.freqrange()))}
        if self.cmpbias() != 0 { try!(write!(f, " cmpbias=0x{:x}", self.cmpbias()))}
        if self.ldohp() != 0 { try!(write!(f, " ldohp"))}
        if self.clkdiv() != 0 { try!(write!(f, " clkdiv=0x{:x}", self.clkdiv()))}
        if self.finetuningen() != 0 { try!(write!(f, " finetuningen"))}
        if self.vreftc() != 0 { try!(write!(f, " vreftc=0x{:x}", self.vreftc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AUXHFRCO Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Auxhfrcoctrl(pub u32);
impl Auxhfrcoctrl {
    #[doc="AUXHFRCO Tuning Value"]
    #[inline] pub fn tuning(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if TUNING != 0"]
    #[inline] pub fn test_tuning(&self) -> bool {
        self.tuning() != 0
    }

    #[doc="Sets the TUNING field."]
    #[inline] pub fn set_tuning<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="AUXHFRCO Fine Tuning Value"]
    #[inline] pub fn finetuning(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3f) as u8) } // [13:8]
    }

    #[doc="Returns true if FINETUNING != 0"]
    #[inline] pub fn test_finetuning(&self) -> bool {
        self.finetuning() != 0
    }

    #[doc="Sets the FINETUNING field."]
    #[inline] pub fn set_finetuning<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="AUXHFRCO Frequency Range"]
    #[inline] pub fn freqrange(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1f) as u8) } // [20:16]
    }

    #[doc="Returns true if FREQRANGE != 0"]
    #[inline] pub fn test_freqrange(&self) -> bool {
        self.freqrange() != 0
    }

    #[doc="Sets the FREQRANGE field."]
    #[inline] pub fn set_freqrange<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="AUXHFRCO Comparator Bias Current"]
    #[inline] pub fn cmpbias(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x7) as u8) } // [23:21]
    }

    #[doc="Returns true if CMPBIAS != 0"]
    #[inline] pub fn test_cmpbias(&self) -> bool {
        self.cmpbias() != 0
    }

    #[doc="Sets the CMPBIAS field."]
    #[inline] pub fn set_cmpbias<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="AUXHFRCO LDO High Power Mode"]
    #[inline] pub fn ldohp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if LDOHP != 0"]
    #[inline] pub fn test_ldohp(&self) -> bool {
        self.ldohp() != 0
    }

    #[doc="Sets the LDOHP field."]
    #[inline] pub fn set_ldohp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Locally divide AUXHFRCO Clock Output"]
    #[inline] pub fn clkdiv(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x3) as u8) } // [26:25]
    }

    #[doc="Returns true if CLKDIV != 0"]
    #[inline] pub fn test_clkdiv(&self) -> bool {
        self.clkdiv() != 0
    }

    #[doc="Sets the CLKDIV field."]
    #[inline] pub fn set_clkdiv<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Enable reference for fine tuning"]
    #[inline] pub fn finetuningen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if FINETUNINGEN != 0"]
    #[inline] pub fn test_finetuningen(&self) -> bool {
        self.finetuningen() != 0
    }

    #[doc="Sets the FINETUNINGEN field."]
    #[inline] pub fn set_finetuningen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="AUXHFRCO Temperature Coefficient Trim on Comparator Reference"]
    #[inline] pub fn vreftc(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0xf) as u8) } // [31:28]
    }

    #[doc="Returns true if VREFTC != 0"]
    #[inline] pub fn test_vreftc(&self) -> bool {
        self.vreftc() != 0
    }

    #[doc="Sets the VREFTC field."]
    #[inline] pub fn set_vreftc<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Auxhfrcoctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Auxhfrcoctrl(other)
    }
}

impl ::core::fmt::Display for Auxhfrcoctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Auxhfrcoctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tuning() != 0 { try!(write!(f, " tuning=0x{:x}", self.tuning()))}
        if self.finetuning() != 0 { try!(write!(f, " finetuning=0x{:x}", self.finetuning()))}
        if self.freqrange() != 0 { try!(write!(f, " freqrange=0x{:x}", self.freqrange()))}
        if self.cmpbias() != 0 { try!(write!(f, " cmpbias=0x{:x}", self.cmpbias()))}
        if self.ldohp() != 0 { try!(write!(f, " ldohp"))}
        if self.clkdiv() != 0 { try!(write!(f, " clkdiv=0x{:x}", self.clkdiv()))}
        if self.finetuningen() != 0 { try!(write!(f, " finetuningen"))}
        if self.vreftc() != 0 { try!(write!(f, " vreftc=0x{:x}", self.vreftc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LFRCO Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lfrcoctrl(pub u32);
impl Lfrcoctrl {
    #[doc="LFRCO Tuning Value"]
    #[inline] pub fn tuning(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if TUNING != 0"]
    #[inline] pub fn test_tuning(&self) -> bool {
        self.tuning() != 0
    }

    #[doc="Sets the TUNING field."]
    #[inline] pub fn set_tuning<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Enable duty cycling of vref"]
    #[inline] pub fn envref(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if ENVREF != 0"]
    #[inline] pub fn test_envref(&self) -> bool {
        self.envref() != 0
    }

    #[doc="Sets the ENVREF field."]
    #[inline] pub fn set_envref<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Enable comparator chopping"]
    #[inline] pub fn enchop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if ENCHOP != 0"]
    #[inline] pub fn test_enchop(&self) -> bool {
        self.enchop() != 0
    }

    #[doc="Sets the ENCHOP field."]
    #[inline] pub fn set_enchop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Enable dynamic element matching"]
    #[inline] pub fn endem(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if ENDEM != 0"]
    #[inline] pub fn test_endem(&self) -> bool {
        self.endem() != 0
    }

    #[doc="Sets the ENDEM field."]
    #[inline] pub fn set_endem<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Control Vref update rate"]
    #[inline] pub fn vrefupdate(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if VREFUPDATE != 0"]
    #[inline] pub fn test_vrefupdate(&self) -> bool {
        self.vrefupdate() != 0
    }

    #[doc="Sets the VREFUPDATE field."]
    #[inline] pub fn set_vrefupdate<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="LFRCO Timeout"]
    #[inline] pub fn timeout(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
    }

    #[doc="Returns true if TIMEOUT != 0"]
    #[inline] pub fn test_timeout(&self) -> bool {
        self.timeout() != 0
    }

    #[doc="Sets the TIMEOUT field."]
    #[inline] pub fn set_timeout<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Tuning of gmc current"]
    #[inline] pub fn gmccurtune(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0xf) as u8) } // [31:28]
    }

    #[doc="Returns true if GMCCURTUNE != 0"]
    #[inline] pub fn test_gmccurtune(&self) -> bool {
        self.gmccurtune() != 0
    }

    #[doc="Sets the GMCCURTUNE field."]
    #[inline] pub fn set_gmccurtune<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Lfrcoctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Lfrcoctrl(other)
    }
}

impl ::core::fmt::Display for Lfrcoctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lfrcoctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tuning() != 0 { try!(write!(f, " tuning=0x{:x}", self.tuning()))}
        if self.envref() != 0 { try!(write!(f, " envref"))}
        if self.enchop() != 0 { try!(write!(f, " enchop"))}
        if self.endem() != 0 { try!(write!(f, " endem"))}
        if self.vrefupdate() != 0 { try!(write!(f, " vrefupdate=0x{:x}", self.vrefupdate()))}
        if self.timeout() != 0 { try!(write!(f, " timeout=0x{:x}", self.timeout()))}
        if self.gmccurtune() != 0 { try!(write!(f, " gmccurtune=0x{:x}", self.gmccurtune()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HFXO Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hfxoctrl(pub u32);
impl Hfxoctrl {
    #[doc="HFXO Mode"]
    #[inline] pub fn mode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="HFXO Automatic Peak Detection and shunt current optimization mode"]
    #[inline] pub fn peakdetshuntoptmode(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if PEAKDETSHUNTOPTMODE != 0"]
    #[inline] pub fn test_peakdetshuntoptmode(&self) -> bool {
        self.peakdetshuntoptmode() != 0
    }

    #[doc="Sets the PEAKDETSHUNTOPTMODE field."]
    #[inline] pub fn set_peakdetshuntoptmode<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Low power mode control. PSR performance is reduced to enable low current consumption."]
    #[inline] pub fn lowpower(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LOWPOWER != 0"]
    #[inline] pub fn test_lowpower(&self) -> bool {
        self.lowpower() != 0
    }

    #[doc="Sets the LOWPOWER field."]
    #[inline] pub fn set_lowpower<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Clamp HFXTAL_N pin to ground when HFXO oscillator is off."]
    #[inline] pub fn xti2gnd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if XTI2GND != 0"]
    #[inline] pub fn test_xti2gnd(&self) -> bool {
        self.xti2gnd() != 0
    }

    #[doc="Sets the XTI2GND field."]
    #[inline] pub fn set_xti2gnd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Clamp HFXTAL_P pin to ground when HFXO oscillator is off."]
    #[inline] pub fn xto2gnd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if XTO2GND != 0"]
    #[inline] pub fn test_xto2gnd(&self) -> bool {
        self.xto2gnd() != 0
    }

    #[doc="Sets the XTO2GND field."]
    #[inline] pub fn set_xto2gnd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="HFXO Low Frequency Timeout"]
    #[inline] pub fn lftimeout(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if LFTIMEOUT != 0"]
    #[inline] pub fn test_lftimeout(&self) -> bool {
        self.lftimeout() != 0
    }

    #[doc="Sets the LFTIMEOUT field."]
    #[inline] pub fn set_lftimeout<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Automatically start of HFXO upon EM0/EM1 entry from EM2/EM3"]
    #[inline] pub fn autostartem0em1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if AUTOSTARTEM0EM1 != 0"]
    #[inline] pub fn test_autostartem0em1(&self) -> bool {
        self.autostartem0em1() != 0
    }

    #[doc="Sets the AUTOSTARTEM0EM1 field."]
    #[inline] pub fn set_autostartem0em1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Automatically start and select of HFXO upon EM0/EM1 entry from EM2/EM3"]
    #[inline] pub fn autostartselem0em1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if AUTOSTARTSELEM0EM1 != 0"]
    #[inline] pub fn test_autostartselem0em1(&self) -> bool {
        self.autostartselem0em1() != 0
    }

    #[doc="Sets the AUTOSTARTSELEM0EM1 field."]
    #[inline] pub fn set_autostartselem0em1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

}

impl From<u32> for Hfxoctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Hfxoctrl(other)
    }
}

impl ::core::fmt::Display for Hfxoctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hfxoctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mode() != 0 { try!(write!(f, " mode"))}
        if self.peakdetshuntoptmode() != 0 { try!(write!(f, " peakdetshuntoptmode=0x{:x}", self.peakdetshuntoptmode()))}
        if self.lowpower() != 0 { try!(write!(f, " lowpower"))}
        if self.xti2gnd() != 0 { try!(write!(f, " xti2gnd"))}
        if self.xto2gnd() != 0 { try!(write!(f, " xto2gnd"))}
        if self.lftimeout() != 0 { try!(write!(f, " lftimeout=0x{:x}", self.lftimeout()))}
        if self.autostartem0em1() != 0 { try!(write!(f, " autostartem0em1"))}
        if self.autostartselem0em1() != 0 { try!(write!(f, " autostartselem0em1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HFXO Startup Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hfxostartupctrl(pub u32);
impl Hfxostartupctrl {
    #[doc="Sets the startup oscillator core bias current. Current (uA) = IBTRIMXOCORE x 40uA. Bits 6 and 5 may only be high in the crystal oscillator startup phase"]
    #[inline] pub fn ibtrimxocore(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if IBTRIMXOCORE != 0"]
    #[inline] pub fn test_ibtrimxocore(&self) -> bool {
        self.ibtrimxocore() != 0
    }

    #[doc="Sets the IBTRIMXOCORE field."]
    #[inline] pub fn set_ibtrimxocore<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Sets oscillator tuning capacitance. Capacitance on HFXTAL_N and HFXTAL_P (pF) = Ctune = Cpar + CTUNE<8:0> x 40fF. Max Ctune 25pF (CLmax ~12.5pF). CL(DNLmax)=50fF ~ 0.6ppm (12.5ppm/pF)"]
    #[inline] pub fn ctune(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1ff) as u16) } // [19:11]
    }

    #[doc="Returns true if CTUNE != 0"]
    #[inline] pub fn test_ctune(&self) -> bool {
        self.ctune() != 0
    }

    #[doc="Sets the CTUNE field."]
    #[inline] pub fn set_ctune<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for Hfxostartupctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Hfxostartupctrl(other)
    }
}

impl ::core::fmt::Display for Hfxostartupctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hfxostartupctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ibtrimxocore() != 0 { try!(write!(f, " ibtrimxocore=0x{:x}", self.ibtrimxocore()))}
        if self.ctune() != 0 { try!(write!(f, " ctune=0x{:x}", self.ctune()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HFXO Steady State control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hfxosteadystatectrl(pub u32);
impl Hfxosteadystatectrl {
    #[doc="Sets the steady state oscillator core bias current. Current (uA) = IBTRIMXOCORE x 40uA. Bits 6 and 5 may only be high in the crystal oscillator startup phase."]
    #[inline] pub fn ibtrimxocore(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if IBTRIMXOCORE != 0"]
    #[inline] pub fn test_ibtrimxocore(&self) -> bool {
        self.ibtrimxocore() != 0
    }

    #[doc="Sets the IBTRIMXOCORE field."]
    #[inline] pub fn set_ibtrimxocore<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Sets the steady state regulator output current level (shunt regulator). Ish = 120uA + REGISH x 120uA"]
    #[inline] pub fn regish(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0xf) as u8) } // [10:7]
    }

    #[doc="Returns true if REGISH != 0"]
    #[inline] pub fn test_regish(&self) -> bool {
        self.regish() != 0
    }

    #[doc="Sets the REGISH field."]
    #[inline] pub fn set_regish<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Sets oscillator tuning capacitance. Capacitance on HFXTAL_N and HFXTAL_P (pF) = Ctune = Cpar + CTUNE<8:0> x 40fF. Max Ctune 25pF (CLmax ~12.5pF). CL(DNLmax)=50fF ~ 0.6ppm (12.5ppm/pF)"]
    #[inline] pub fn ctune(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1ff) as u16) } // [19:11]
    }

    #[doc="Returns true if CTUNE != 0"]
    #[inline] pub fn test_ctune(&self) -> bool {
        self.ctune() != 0
    }

    #[doc="Sets the CTUNE field."]
    #[inline] pub fn set_ctune<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Controls regulator minimum shunt current detection relative to nominal"]
    #[inline] pub fn regselilow(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
    }

    #[doc="Returns true if REGSELILOW != 0"]
    #[inline] pub fn test_regselilow(&self) -> bool {
        self.regselilow() != 0
    }

    #[doc="Sets the REGSELILOW field."]
    #[inline] pub fn set_regselilow<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Enables oscillator peak detectors"]
    #[inline] pub fn peakdeten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PEAKDETEN != 0"]
    #[inline] pub fn test_peakdeten(&self) -> bool {
        self.peakdeten() != 0
    }

    #[doc="Sets the PEAKDETEN field."]
    #[inline] pub fn set_peakdeten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Set regulator output current level (shunt regulator). Ish = 120uA + REGISHUPPER x 120uA"]
    #[inline] pub fn regishupper(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0xf) as u8) } // [31:28]
    }

    #[doc="Returns true if REGISHUPPER != 0"]
    #[inline] pub fn test_regishupper(&self) -> bool {
        self.regishupper() != 0
    }

    #[doc="Sets the REGISHUPPER field."]
    #[inline] pub fn set_regishupper<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Hfxosteadystatectrl {
    #[inline]
    fn from(other: u32) -> Self {
         Hfxosteadystatectrl(other)
    }
}

impl ::core::fmt::Display for Hfxosteadystatectrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hfxosteadystatectrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ibtrimxocore() != 0 { try!(write!(f, " ibtrimxocore=0x{:x}", self.ibtrimxocore()))}
        if self.regish() != 0 { try!(write!(f, " regish=0x{:x}", self.regish()))}
        if self.ctune() != 0 { try!(write!(f, " ctune=0x{:x}", self.ctune()))}
        if self.regselilow() != 0 { try!(write!(f, " regselilow=0x{:x}", self.regselilow()))}
        if self.peakdeten() != 0 { try!(write!(f, " peakdeten"))}
        if self.regishupper() != 0 { try!(write!(f, " regishupper=0x{:x}", self.regishupper()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HFXO Timeout Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hfxotimeoutctrl(pub u32);
impl Hfxotimeoutctrl {
    #[doc="Wait duration in HFXO startup enable wait state"]
    #[inline] pub fn startuptimeout(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if STARTUPTIMEOUT != 0"]
    #[inline] pub fn test_startuptimeout(&self) -> bool {
        self.startuptimeout() != 0
    }

    #[doc="Sets the STARTUPTIMEOUT field."]
    #[inline] pub fn set_startuptimeout<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Wait duration in HFXO startup steady wait state"]
    #[inline] pub fn steadytimeout(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if STEADYTIMEOUT != 0"]
    #[inline] pub fn test_steadytimeout(&self) -> bool {
        self.steadytimeout() != 0
    }

    #[doc="Sets the STEADYTIMEOUT field."]
    #[inline] pub fn set_steadytimeout<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Wait duration in HFXO peak detection wait state"]
    #[inline] pub fn peakdettimeout(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if PEAKDETTIMEOUT != 0"]
    #[inline] pub fn test_peakdettimeout(&self) -> bool {
        self.peakdettimeout() != 0
    }

    #[doc="Sets the PEAKDETTIMEOUT field."]
    #[inline] pub fn set_peakdettimeout<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Wait duration in HFXO shunt current optimization wait state"]
    #[inline] pub fn shuntopttimeout(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if SHUNTOPTTIMEOUT != 0"]
    #[inline] pub fn test_shuntopttimeout(&self) -> bool {
        self.shuntopttimeout() != 0
    }

    #[doc="Sets the SHUNTOPTTIMEOUT field."]
    #[inline] pub fn set_shuntopttimeout<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Hfxotimeoutctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Hfxotimeoutctrl(other)
    }
}

impl ::core::fmt::Display for Hfxotimeoutctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hfxotimeoutctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.startuptimeout() != 0 { try!(write!(f, " startuptimeout=0x{:x}", self.startuptimeout()))}
        if self.steadytimeout() != 0 { try!(write!(f, " steadytimeout=0x{:x}", self.steadytimeout()))}
        if self.peakdettimeout() != 0 { try!(write!(f, " peakdettimeout=0x{:x}", self.peakdettimeout()))}
        if self.shuntopttimeout() != 0 { try!(write!(f, " shuntopttimeout=0x{:x}", self.shuntopttimeout()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LFXO Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lfxoctrl(pub u32);
impl Lfxoctrl {
    #[doc="LFXO Internal Capacitor Array Tuning Value"]
    #[inline] pub fn tuning(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if TUNING != 0"]
    #[inline] pub fn test_tuning(&self) -> bool {
        self.tuning() != 0
    }

    #[doc="Sets the TUNING field."]
    #[inline] pub fn set_tuning<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="LFXO Mode"]
    #[inline] pub fn mode(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="LFXO Startup Gain"]
    #[inline] pub fn gain(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x3) as u8) } // [12:11]
    }

    #[doc="Returns true if GAIN != 0"]
    #[inline] pub fn test_gain(&self) -> bool {
        self.gain() != 0
    }

    #[doc="Sets the GAIN field."]
    #[inline] pub fn set_gain<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="LFXO High XTAL Oscillation Amplitude Enable"]
    #[inline] pub fn highampl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if HIGHAMPL != 0"]
    #[inline] pub fn test_highampl(&self) -> bool {
        self.highampl() != 0
    }

    #[doc="Sets the HIGHAMPL field."]
    #[inline] pub fn set_highampl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="LFXO AGC Enable"]
    #[inline] pub fn agc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if AGC != 0"]
    #[inline] pub fn test_agc(&self) -> bool {
        self.agc() != 0
    }

    #[doc="Sets the AGC field."]
    #[inline] pub fn set_agc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="LFXO Current Trim"]
    #[inline] pub fn cur(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if CUR != 0"]
    #[inline] pub fn test_cur(&self) -> bool {
        self.cur() != 0
    }

    #[doc="Sets the CUR field."]
    #[inline] pub fn set_cur<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="LFXO Buffer Bias Current"]
    #[inline] pub fn bufcur(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if BUFCUR != 0"]
    #[inline] pub fn test_bufcur(&self) -> bool {
        self.bufcur() != 0
    }

    #[doc="Sets the BUFCUR field."]
    #[inline] pub fn set_bufcur<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="LFXO Timeout"]
    #[inline] pub fn timeout(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if TIMEOUT != 0"]
    #[inline] pub fn test_timeout(&self) -> bool {
        self.timeout() != 0
    }

    #[doc="Sets the TIMEOUT field."]
    #[inline] pub fn set_timeout<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Lfxoctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Lfxoctrl(other)
    }
}

impl ::core::fmt::Display for Lfxoctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lfxoctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tuning() != 0 { try!(write!(f, " tuning=0x{:x}", self.tuning()))}
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        if self.gain() != 0 { try!(write!(f, " gain=0x{:x}", self.gain()))}
        if self.highampl() != 0 { try!(write!(f, " highampl"))}
        if self.agc() != 0 { try!(write!(f, " agc"))}
        if self.cur() != 0 { try!(write!(f, " cur=0x{:x}", self.cur()))}
        if self.bufcur() != 0 { try!(write!(f, " bufcur"))}
        if self.timeout() != 0 { try!(write!(f, " timeout=0x{:x}", self.timeout()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DPLL Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dpllctrl(pub u32);
impl Dpllctrl {
    #[doc="Operating Mode Control"]
    #[inline] pub fn mode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Reference Edge Select"]
    #[inline] pub fn edgesel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EDGESEL != 0"]
    #[inline] pub fn test_edgesel(&self) -> bool {
        self.edgesel() != 0
    }

    #[doc="Sets the EDGESEL field."]
    #[inline] pub fn set_edgesel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="automatic recovery ctrl"]
    #[inline] pub fn autorecover(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if AUTORECOVER != 0"]
    #[inline] pub fn test_autorecover(&self) -> bool {
        self.autorecover() != 0
    }

    #[doc="Sets the AUTORECOVER field."]
    #[inline] pub fn set_autorecover<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Reference Clock Selection Control"]
    #[inline] pub fn refsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if REFSEL != 0"]
    #[inline] pub fn test_refsel(&self) -> bool {
        self.refsel() != 0
    }

    #[doc="Sets the REFSEL field."]
    #[inline] pub fn set_refsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Dpllctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Dpllctrl(other)
    }
}

impl ::core::fmt::Display for Dpllctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dpllctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mode() != 0 { try!(write!(f, " mode"))}
        if self.edgesel() != 0 { try!(write!(f, " edgesel"))}
        if self.autorecover() != 0 { try!(write!(f, " autorecover"))}
        if self.refsel() != 0 { try!(write!(f, " refsel=0x{:x}", self.refsel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DPLL Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dpllctrl1(pub u32);
impl Dpllctrl1 {
    #[doc="Factor M"]
    #[inline] pub fn m(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if M != 0"]
    #[inline] pub fn test_m(&self) -> bool {
        self.m() != 0
    }

    #[doc="Sets the M field."]
    #[inline] pub fn set_m<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Factor N"]
    #[inline] pub fn n(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xfff) as u16) } // [27:16]
    }

    #[doc="Returns true if N != 0"]
    #[inline] pub fn test_n(&self) -> bool {
        self.n() != 0
    }

    #[doc="Sets the N field."]
    #[inline] pub fn set_n<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Dpllctrl1 {
    #[inline]
    fn from(other: u32) -> Self {
         Dpllctrl1(other)
    }
}

impl ::core::fmt::Display for Dpllctrl1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dpllctrl1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.m() != 0 { try!(write!(f, " m=0x{:x}", self.m()))}
        if self.n() != 0 { try!(write!(f, " n=0x{:x}", self.n()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Calibration Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Calctrl(pub u32);
impl Calctrl {
    #[doc="Calibration Up-counter Select"]
    #[inline] pub fn upsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if UPSEL != 0"]
    #[inline] pub fn test_upsel(&self) -> bool {
        self.upsel() != 0
    }

    #[doc="Sets the UPSEL field."]
    #[inline] pub fn set_upsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Calibration Down-counter Select"]
    #[inline] pub fn downsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if DOWNSEL != 0"]
    #[inline] pub fn test_downsel(&self) -> bool {
        self.downsel() != 0
    }

    #[doc="Sets the DOWNSEL field."]
    #[inline] pub fn set_downsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Continuous Calibration"]
    #[inline] pub fn cont(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CONT != 0"]
    #[inline] pub fn test_cont(&self) -> bool {
        self.cont() != 0
    }

    #[doc="Sets the CONT field."]
    #[inline] pub fn set_cont<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="PRS Select for PRS Input when selected in UPSEL"]
    #[inline] pub fn prsupsel(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if PRSUPSEL != 0"]
    #[inline] pub fn test_prsupsel(&self) -> bool {
        self.prsupsel() != 0
    }

    #[doc="Sets the PRSUPSEL field."]
    #[inline] pub fn set_prsupsel<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="PRS Select for PRS Input when selected in DOWNSEL"]
    #[inline] pub fn prsdownsel(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if PRSDOWNSEL != 0"]
    #[inline] pub fn test_prsdownsel(&self) -> bool {
        self.prsdownsel() != 0
    }

    #[doc="Sets the PRSDOWNSEL field."]
    #[inline] pub fn set_prsdownsel<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Calctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Calctrl(other)
    }
}

impl ::core::fmt::Display for Calctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Calctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.upsel() != 0 { try!(write!(f, " upsel=0x{:x}", self.upsel()))}
        if self.downsel() != 0 { try!(write!(f, " downsel=0x{:x}", self.downsel()))}
        if self.cont() != 0 { try!(write!(f, " cont"))}
        if self.prsupsel() != 0 { try!(write!(f, " prsupsel=0x{:x}", self.prsupsel()))}
        if self.prsdownsel() != 0 { try!(write!(f, " prsdownsel=0x{:x}", self.prsdownsel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Calibration Counter Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Calcnt(pub u32);
impl Calcnt {
    #[doc="Calibration Counter"]
    #[inline] pub fn calcnt(&self) -> bits::U20 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfffff) as u32) } // [19:0]
    }

    #[doc="Returns true if CALCNT != 0"]
    #[inline] pub fn test_calcnt(&self) -> bool {
        self.calcnt() != 0
    }

    #[doc="Sets the CALCNT field."]
    #[inline] pub fn set_calcnt<V: Into<bits::U20>>(mut self, value: V) -> Self {
        let value: bits::U20 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Calcnt {
    #[inline]
    fn from(other: u32) -> Self {
         Calcnt(other)
    }
}

impl ::core::fmt::Display for Calcnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Calcnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.calcnt() != 0 { try!(write!(f, " calcnt=0x{:x}", self.calcnt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Oscillator Enable/Disable Command Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Oscencmd(pub u32);
impl Oscencmd {
    #[doc="HFRCO Enable"]
    #[inline] pub fn hfrcoen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HFRCOEN != 0"]
    #[inline] pub fn test_hfrcoen(&self) -> bool {
        self.hfrcoen() != 0
    }

    #[doc="Sets the HFRCOEN field."]
    #[inline] pub fn set_hfrcoen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="HFRCO Disable"]
    #[inline] pub fn hfrcodis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if HFRCODIS != 0"]
    #[inline] pub fn test_hfrcodis(&self) -> bool {
        self.hfrcodis() != 0
    }

    #[doc="Sets the HFRCODIS field."]
    #[inline] pub fn set_hfrcodis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="HFXO Enable"]
    #[inline] pub fn hfxoen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if HFXOEN != 0"]
    #[inline] pub fn test_hfxoen(&self) -> bool {
        self.hfxoen() != 0
    }

    #[doc="Sets the HFXOEN field."]
    #[inline] pub fn set_hfxoen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="HFXO Disable"]
    #[inline] pub fn hfxodis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if HFXODIS != 0"]
    #[inline] pub fn test_hfxodis(&self) -> bool {
        self.hfxodis() != 0
    }

    #[doc="Sets the HFXODIS field."]
    #[inline] pub fn set_hfxodis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="AUXHFRCO Enable"]
    #[inline] pub fn auxhfrcoen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if AUXHFRCOEN != 0"]
    #[inline] pub fn test_auxhfrcoen(&self) -> bool {
        self.auxhfrcoen() != 0
    }

    #[doc="Sets the AUXHFRCOEN field."]
    #[inline] pub fn set_auxhfrcoen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="AUXHFRCO Disable"]
    #[inline] pub fn auxhfrcodis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if AUXHFRCODIS != 0"]
    #[inline] pub fn test_auxhfrcodis(&self) -> bool {
        self.auxhfrcodis() != 0
    }

    #[doc="Sets the AUXHFRCODIS field."]
    #[inline] pub fn set_auxhfrcodis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="LFRCO Enable"]
    #[inline] pub fn lfrcoen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LFRCOEN != 0"]
    #[inline] pub fn test_lfrcoen(&self) -> bool {
        self.lfrcoen() != 0
    }

    #[doc="Sets the LFRCOEN field."]
    #[inline] pub fn set_lfrcoen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="LFRCO Disable"]
    #[inline] pub fn lfrcodis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if LFRCODIS != 0"]
    #[inline] pub fn test_lfrcodis(&self) -> bool {
        self.lfrcodis() != 0
    }

    #[doc="Sets the LFRCODIS field."]
    #[inline] pub fn set_lfrcodis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="LFXO Enable"]
    #[inline] pub fn lfxoen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LFXOEN != 0"]
    #[inline] pub fn test_lfxoen(&self) -> bool {
        self.lfxoen() != 0
    }

    #[doc="Sets the LFXOEN field."]
    #[inline] pub fn set_lfxoen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="LFXO Disable"]
    #[inline] pub fn lfxodis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LFXODIS != 0"]
    #[inline] pub fn test_lfxodis(&self) -> bool {
        self.lfxodis() != 0
    }

    #[doc="Sets the LFXODIS field."]
    #[inline] pub fn set_lfxodis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="DPLL Enable"]
    #[inline] pub fn dpllen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DPLLEN != 0"]
    #[inline] pub fn test_dpllen(&self) -> bool {
        self.dpllen() != 0
    }

    #[doc="Sets the DPLLEN field."]
    #[inline] pub fn set_dpllen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="DPLL Disable"]
    #[inline] pub fn dplldis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if DPLLDIS != 0"]
    #[inline] pub fn test_dplldis(&self) -> bool {
        self.dplldis() != 0
    }

    #[doc="Sets the DPLLDIS field."]
    #[inline] pub fn set_dplldis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u32> for Oscencmd {
    #[inline]
    fn from(other: u32) -> Self {
         Oscencmd(other)
    }
}

impl ::core::fmt::Display for Oscencmd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Oscencmd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hfrcoen() != 0 { try!(write!(f, " hfrcoen"))}
        if self.hfrcodis() != 0 { try!(write!(f, " hfrcodis"))}
        if self.hfxoen() != 0 { try!(write!(f, " hfxoen"))}
        if self.hfxodis() != 0 { try!(write!(f, " hfxodis"))}
        if self.auxhfrcoen() != 0 { try!(write!(f, " auxhfrcoen"))}
        if self.auxhfrcodis() != 0 { try!(write!(f, " auxhfrcodis"))}
        if self.lfrcoen() != 0 { try!(write!(f, " lfrcoen"))}
        if self.lfrcodis() != 0 { try!(write!(f, " lfrcodis"))}
        if self.lfxoen() != 0 { try!(write!(f, " lfxoen"))}
        if self.lfxodis() != 0 { try!(write!(f, " lfxodis"))}
        if self.dpllen() != 0 { try!(write!(f, " dpllen"))}
        if self.dplldis() != 0 { try!(write!(f, " dplldis"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Command Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc="Calibration Start"]
    #[inline] pub fn calstart(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CALSTART != 0"]
    #[inline] pub fn test_calstart(&self) -> bool {
        self.calstart() != 0
    }

    #[doc="Sets the CALSTART field."]
    #[inline] pub fn set_calstart<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Calibration Stop"]
    #[inline] pub fn calstop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CALSTOP != 0"]
    #[inline] pub fn test_calstop(&self) -> bool {
        self.calstop() != 0
    }

    #[doc="Sets the CALSTOP field."]
    #[inline] pub fn set_calstop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="HFXO Peak Detection Start"]
    #[inline] pub fn hfxopeakdetstart(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if HFXOPEAKDETSTART != 0"]
    #[inline] pub fn test_hfxopeakdetstart(&self) -> bool {
        self.hfxopeakdetstart() != 0
    }

    #[doc="Sets the HFXOPEAKDETSTART field."]
    #[inline] pub fn set_hfxopeakdetstart<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="HFXO Shunt Current Optimization Start"]
    #[inline] pub fn hfxoshuntoptstart(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if HFXOSHUNTOPTSTART != 0"]
    #[inline] pub fn test_hfxoshuntoptstart(&self) -> bool {
        self.hfxoshuntoptstart() != 0
    }

    #[doc="Sets the HFXOSHUNTOPTSTART field."]
    #[inline] pub fn set_hfxoshuntoptstart<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u32> for Cmd {
    #[inline]
    fn from(other: u32) -> Self {
         Cmd(other)
    }
}

impl ::core::fmt::Display for Cmd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cmd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.calstart() != 0 { try!(write!(f, " calstart"))}
        if self.calstop() != 0 { try!(write!(f, " calstop"))}
        if self.hfxopeakdetstart() != 0 { try!(write!(f, " hfxopeakdetstart"))}
        if self.hfxoshuntoptstart() != 0 { try!(write!(f, " hfxoshuntoptstart"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Debug Trace Clock Select"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dbgclksel(pub u32);
impl Dbgclksel {
    #[doc="Debug Trace Clock"]
    #[inline] pub fn dbg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DBG != 0"]
    #[inline] pub fn test_dbg(&self) -> bool {
        self.dbg() != 0
    }

    #[doc="Sets the DBG field."]
    #[inline] pub fn set_dbg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dbgclksel {
    #[inline]
    fn from(other: u32) -> Self {
         Dbgclksel(other)
    }
}

impl ::core::fmt::Display for Dbgclksel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dbgclksel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dbg() != 0 { try!(write!(f, " dbg"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="High Frequency Clock Select Command Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hfclksel(pub u32);
impl Hfclksel {
    #[doc="HFCLK Select"]
    #[inline] pub fn hf(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if HF != 0"]
    #[inline] pub fn test_hf(&self) -> bool {
        self.hf() != 0
    }

    #[doc="Sets the HF field."]
    #[inline] pub fn set_hf<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Hfclksel {
    #[inline]
    fn from(other: u32) -> Self {
         Hfclksel(other)
    }
}

impl ::core::fmt::Display for Hfclksel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hfclksel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hf() != 0 { try!(write!(f, " hf=0x{:x}", self.hf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Low Frequency A Clock Select Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lfaclksel(pub u32);
impl Lfaclksel {
    #[doc="Clock Select for LFA"]
    #[inline] pub fn lfa(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if LFA != 0"]
    #[inline] pub fn test_lfa(&self) -> bool {
        self.lfa() != 0
    }

    #[doc="Sets the LFA field."]
    #[inline] pub fn set_lfa<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Lfaclksel {
    #[inline]
    fn from(other: u32) -> Self {
         Lfaclksel(other)
    }
}

impl ::core::fmt::Display for Lfaclksel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lfaclksel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lfa() != 0 { try!(write!(f, " lfa=0x{:x}", self.lfa()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Low Frequency B Clock Select Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lfbclksel(pub u32);
impl Lfbclksel {
    #[doc="Clock Select for LFB"]
    #[inline] pub fn lfb(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if LFB != 0"]
    #[inline] pub fn test_lfb(&self) -> bool {
        self.lfb() != 0
    }

    #[doc="Sets the LFB field."]
    #[inline] pub fn set_lfb<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Lfbclksel {
    #[inline]
    fn from(other: u32) -> Self {
         Lfbclksel(other)
    }
}

impl ::core::fmt::Display for Lfbclksel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lfbclksel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lfb() != 0 { try!(write!(f, " lfb=0x{:x}", self.lfb()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Low Frequency E Clock Select Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lfeclksel(pub u32);
impl Lfeclksel {
    #[doc="Clock Select for LFE"]
    #[inline] pub fn lfe(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if LFE != 0"]
    #[inline] pub fn test_lfe(&self) -> bool {
        self.lfe() != 0
    }

    #[doc="Sets the LFE field."]
    #[inline] pub fn set_lfe<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Lfeclksel {
    #[inline]
    fn from(other: u32) -> Self {
         Lfeclksel(other)
    }
}

impl ::core::fmt::Display for Lfeclksel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lfeclksel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lfe() != 0 { try!(write!(f, " lfe=0x{:x}", self.lfe()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u32);
impl Status {
    #[doc="HFRCO Enable Status"]
    #[inline] pub fn hfrcoens(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HFRCOENS != 0"]
    #[inline] pub fn test_hfrcoens(&self) -> bool {
        self.hfrcoens() != 0
    }

    #[doc="Sets the HFRCOENS field."]
    #[inline] pub fn set_hfrcoens<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="HFRCO Ready"]
    #[inline] pub fn hfrcordy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if HFRCORDY != 0"]
    #[inline] pub fn test_hfrcordy(&self) -> bool {
        self.hfrcordy() != 0
    }

    #[doc="Sets the HFRCORDY field."]
    #[inline] pub fn set_hfrcordy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="HFXO Enable Status"]
    #[inline] pub fn hfxoens(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if HFXOENS != 0"]
    #[inline] pub fn test_hfxoens(&self) -> bool {
        self.hfxoens() != 0
    }

    #[doc="Sets the HFXOENS field."]
    #[inline] pub fn set_hfxoens<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="HFXO Ready"]
    #[inline] pub fn hfxordy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if HFXORDY != 0"]
    #[inline] pub fn test_hfxordy(&self) -> bool {
        self.hfxordy() != 0
    }

    #[doc="Sets the HFXORDY field."]
    #[inline] pub fn set_hfxordy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="AUXHFRCO Enable Status"]
    #[inline] pub fn auxhfrcoens(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if AUXHFRCOENS != 0"]
    #[inline] pub fn test_auxhfrcoens(&self) -> bool {
        self.auxhfrcoens() != 0
    }

    #[doc="Sets the AUXHFRCOENS field."]
    #[inline] pub fn set_auxhfrcoens<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="AUXHFRCO Ready"]
    #[inline] pub fn auxhfrcordy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if AUXHFRCORDY != 0"]
    #[inline] pub fn test_auxhfrcordy(&self) -> bool {
        self.auxhfrcordy() != 0
    }

    #[doc="Sets the AUXHFRCORDY field."]
    #[inline] pub fn set_auxhfrcordy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="LFRCO Enable Status"]
    #[inline] pub fn lfrcoens(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LFRCOENS != 0"]
    #[inline] pub fn test_lfrcoens(&self) -> bool {
        self.lfrcoens() != 0
    }

    #[doc="Sets the LFRCOENS field."]
    #[inline] pub fn set_lfrcoens<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="LFRCO Ready"]
    #[inline] pub fn lfrcordy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if LFRCORDY != 0"]
    #[inline] pub fn test_lfrcordy(&self) -> bool {
        self.lfrcordy() != 0
    }

    #[doc="Sets the LFRCORDY field."]
    #[inline] pub fn set_lfrcordy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="LFXO Enable Status"]
    #[inline] pub fn lfxoens(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LFXOENS != 0"]
    #[inline] pub fn test_lfxoens(&self) -> bool {
        self.lfxoens() != 0
    }

    #[doc="Sets the LFXOENS field."]
    #[inline] pub fn set_lfxoens<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="LFXO Ready"]
    #[inline] pub fn lfxordy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LFXORDY != 0"]
    #[inline] pub fn test_lfxordy(&self) -> bool {
        self.lfxordy() != 0
    }

    #[doc="Sets the LFXORDY field."]
    #[inline] pub fn set_lfxordy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="DPLL Enable Status"]
    #[inline] pub fn dpllens(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DPLLENS != 0"]
    #[inline] pub fn test_dpllens(&self) -> bool {
        self.dpllens() != 0
    }

    #[doc="Sets the DPLLENS field."]
    #[inline] pub fn set_dpllens<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="DPLL Ready"]
    #[inline] pub fn dpllrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if DPLLRDY != 0"]
    #[inline] pub fn test_dpllrdy(&self) -> bool {
        self.dpllrdy() != 0
    }

    #[doc="Sets the DPLLRDY field."]
    #[inline] pub fn set_dpllrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Calibration Ready"]
    #[inline] pub fn calrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if CALRDY != 0"]
    #[inline] pub fn test_calrdy(&self) -> bool {
        self.calrdy() != 0
    }

    #[doc="Sets the CALRDY field."]
    #[inline] pub fn set_calrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="HFXO is Required by Hardware (e.g. RAC)"]
    #[inline] pub fn hfxoreq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if HFXOREQ != 0"]
    #[inline] pub fn test_hfxoreq(&self) -> bool {
        self.hfxoreq() != 0
    }

    #[doc="Sets the HFXOREQ field."]
    #[inline] pub fn set_hfxoreq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="HFXO Peak Detection Ready"]
    #[inline] pub fn hfxopeakdetrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if HFXOPEAKDETRDY != 0"]
    #[inline] pub fn test_hfxopeakdetrdy(&self) -> bool {
        self.hfxopeakdetrdy() != 0
    }

    #[doc="Sets the HFXOPEAKDETRDY field."]
    #[inline] pub fn set_hfxopeakdetrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="HFXO Shunt Current Optimization ready"]
    #[inline] pub fn hfxoshuntoptrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if HFXOSHUNTOPTRDY != 0"]
    #[inline] pub fn test_hfxoshuntoptrdy(&self) -> bool {
        self.hfxoshuntoptrdy() != 0
    }

    #[doc="Sets the HFXOSHUNTOPTRDY field."]
    #[inline] pub fn set_hfxoshuntoptrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="HFXO oscillation amplitude is too high"]
    #[inline] pub fn hfxoamphigh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if HFXOAMPHIGH != 0"]
    #[inline] pub fn test_hfxoamphigh(&self) -> bool {
        self.hfxoamphigh() != 0
    }

    #[doc="Sets the HFXOAMPHIGH field."]
    #[inline] pub fn set_hfxoamphigh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="HFXO amplitude tuning value too low"]
    #[inline] pub fn hfxoamplow(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if HFXOAMPLOW != 0"]
    #[inline] pub fn test_hfxoamplow(&self) -> bool {
        self.hfxoamplow() != 0
    }

    #[doc="Sets the HFXOAMPLOW field."]
    #[inline] pub fn set_hfxoamplow<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="HFXO regulator shunt current too low"]
    #[inline] pub fn hfxoregilow(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if HFXOREGILOW != 0"]
    #[inline] pub fn test_hfxoregilow(&self) -> bool {
        self.hfxoregilow() != 0
    }

    #[doc="Sets the HFXOREGILOW field."]
    #[inline] pub fn set_hfxoregilow<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
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
        if self.hfrcoens() != 0 { try!(write!(f, " hfrcoens"))}
        if self.hfrcordy() != 0 { try!(write!(f, " hfrcordy"))}
        if self.hfxoens() != 0 { try!(write!(f, " hfxoens"))}
        if self.hfxordy() != 0 { try!(write!(f, " hfxordy"))}
        if self.auxhfrcoens() != 0 { try!(write!(f, " auxhfrcoens"))}
        if self.auxhfrcordy() != 0 { try!(write!(f, " auxhfrcordy"))}
        if self.lfrcoens() != 0 { try!(write!(f, " lfrcoens"))}
        if self.lfrcordy() != 0 { try!(write!(f, " lfrcordy"))}
        if self.lfxoens() != 0 { try!(write!(f, " lfxoens"))}
        if self.lfxordy() != 0 { try!(write!(f, " lfxordy"))}
        if self.dpllens() != 0 { try!(write!(f, " dpllens"))}
        if self.dpllrdy() != 0 { try!(write!(f, " dpllrdy"))}
        if self.calrdy() != 0 { try!(write!(f, " calrdy"))}
        if self.hfxoreq() != 0 { try!(write!(f, " hfxoreq"))}
        if self.hfxopeakdetrdy() != 0 { try!(write!(f, " hfxopeakdetrdy"))}
        if self.hfxoshuntoptrdy() != 0 { try!(write!(f, " hfxoshuntoptrdy"))}
        if self.hfxoamphigh() != 0 { try!(write!(f, " hfxoamphigh"))}
        if self.hfxoamplow() != 0 { try!(write!(f, " hfxoamplow"))}
        if self.hfxoregilow() != 0 { try!(write!(f, " hfxoregilow"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HFCLK Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hfclkstatus(pub u32);
impl Hfclkstatus {
    #[doc="HFCLK Selected"]
    #[inline] pub fn selected(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if SELECTED != 0"]
    #[inline] pub fn test_selected(&self) -> bool {
        self.selected() != 0
    }

    #[doc="Sets the SELECTED field."]
    #[inline] pub fn set_selected<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Hfclkstatus {
    #[inline]
    fn from(other: u32) -> Self {
         Hfclkstatus(other)
    }
}

impl ::core::fmt::Display for Hfclkstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hfclkstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.selected() != 0 { try!(write!(f, " selected=0x{:x}", self.selected()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HFXO Trim Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hfxotrimstatus(pub u32);
impl Hfxotrimstatus {
    #[doc="Value of IBTRIMXOCORE found by automatic HFXO peak detection algorithm. Can be used as initial value for IBTRIMXOCORE in the CMU_HFXOSTEADYSTATECTRL register if HFXO is to be started again."]
    #[inline] pub fn ibtrimxocore(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if IBTRIMXOCORE != 0"]
    #[inline] pub fn test_ibtrimxocore(&self) -> bool {
        self.ibtrimxocore() != 0
    }

    #[doc="Sets the IBTRIMXOCORE field."]
    #[inline] pub fn set_ibtrimxocore<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Value of REGISH found by automatic HFXO shunt current optimization algorithm. Can be used as initial value for REGISH value in the CMU_HFXOSTEADYSTATECTRL register if HFXO is to be started again."]
    #[inline] pub fn regish(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0xf) as u8) } // [10:7]
    }

    #[doc="Returns true if REGISH != 0"]
    #[inline] pub fn test_regish(&self) -> bool {
        self.regish() != 0
    }

    #[doc="Sets the REGISH field."]
    #[inline] pub fn set_regish<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Hfxotrimstatus {
    #[inline]
    fn from(other: u32) -> Self {
         Hfxotrimstatus(other)
    }
}

impl ::core::fmt::Display for Hfxotrimstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hfxotrimstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ibtrimxocore() != 0 { try!(write!(f, " ibtrimxocore=0x{:x}", self.ibtrimxocore()))}
        if self.regish() != 0 { try!(write!(f, " regish=0x{:x}", self.regish()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct If(pub u32);
impl If {
    #[doc="HFRCO Ready Interrupt Flag"]
    #[inline] pub fn hfrcordy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HFRCORDY != 0"]
    #[inline] pub fn test_hfrcordy(&self) -> bool {
        self.hfrcordy() != 0
    }

    #[doc="Sets the HFRCORDY field."]
    #[inline] pub fn set_hfrcordy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="HFXO Ready Interrupt Flag"]
    #[inline] pub fn hfxordy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if HFXORDY != 0"]
    #[inline] pub fn test_hfxordy(&self) -> bool {
        self.hfxordy() != 0
    }

    #[doc="Sets the HFXORDY field."]
    #[inline] pub fn set_hfxordy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="LFRCO Ready Interrupt Flag"]
    #[inline] pub fn lfrcordy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if LFRCORDY != 0"]
    #[inline] pub fn test_lfrcordy(&self) -> bool {
        self.lfrcordy() != 0
    }

    #[doc="Sets the LFRCORDY field."]
    #[inline] pub fn set_lfrcordy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="LFXO Ready Interrupt Flag"]
    #[inline] pub fn lfxordy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if LFXORDY != 0"]
    #[inline] pub fn test_lfxordy(&self) -> bool {
        self.lfxordy() != 0
    }

    #[doc="Sets the LFXORDY field."]
    #[inline] pub fn set_lfxordy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="AUXHFRCO Ready Interrupt Flag"]
    #[inline] pub fn auxhfrcordy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if AUXHFRCORDY != 0"]
    #[inline] pub fn test_auxhfrcordy(&self) -> bool {
        self.auxhfrcordy() != 0
    }

    #[doc="Sets the AUXHFRCORDY field."]
    #[inline] pub fn set_auxhfrcordy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Calibration Ready Interrupt Flag"]
    #[inline] pub fn calrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CALRDY != 0"]
    #[inline] pub fn test_calrdy(&self) -> bool {
        self.calrdy() != 0
    }

    #[doc="Sets the CALRDY field."]
    #[inline] pub fn set_calrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Calibration Overflow Interrupt Flag"]
    #[inline] pub fn calof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CALOF != 0"]
    #[inline] pub fn test_calof(&self) -> bool {
        self.calof() != 0
    }

    #[doc="Sets the CALOF field."]
    #[inline] pub fn set_calof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="HFXO Disable Error Interrupt Flag"]
    #[inline] pub fn hfxodiserr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if HFXODISERR != 0"]
    #[inline] pub fn test_hfxodiserr(&self) -> bool {
        self.hfxodiserr() != 0
    }

    #[doc="Sets the HFXODISERR field."]
    #[inline] pub fn set_hfxodiserr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="HFXO Automatic Switch Interrupt Flag"]
    #[inline] pub fn hfxoautosw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if HFXOAUTOSW != 0"]
    #[inline] pub fn test_hfxoautosw(&self) -> bool {
        self.hfxoautosw() != 0
    }

    #[doc="Sets the HFXOAUTOSW field."]
    #[inline] pub fn set_hfxoautosw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="HFXO Automatic Peak Detection Error Interrupt Flag"]
    #[inline] pub fn hfxopeakdeterr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if HFXOPEAKDETERR != 0"]
    #[inline] pub fn test_hfxopeakdeterr(&self) -> bool {
        self.hfxopeakdeterr() != 0
    }

    #[doc="Sets the HFXOPEAKDETERR field."]
    #[inline] pub fn set_hfxopeakdeterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="HFXO Automatic Peak Detection Ready Interrupt Flag"]
    #[inline] pub fn hfxopeakdetrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if HFXOPEAKDETRDY != 0"]
    #[inline] pub fn test_hfxopeakdetrdy(&self) -> bool {
        self.hfxopeakdetrdy() != 0
    }

    #[doc="Sets the HFXOPEAKDETRDY field."]
    #[inline] pub fn set_hfxopeakdetrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="HFXO Automatic Shunt Current Optimization Ready Interrupt Flag"]
    #[inline] pub fn hfxoshuntoptrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if HFXOSHUNTOPTRDY != 0"]
    #[inline] pub fn test_hfxoshuntoptrdy(&self) -> bool {
        self.hfxoshuntoptrdy() != 0
    }

    #[doc="Sets the HFXOSHUNTOPTRDY field."]
    #[inline] pub fn set_hfxoshuntoptrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="HFRCO Disable Interrupt Flag"]
    #[inline] pub fn hfrcodis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if HFRCODIS != 0"]
    #[inline] pub fn test_hfrcodis(&self) -> bool {
        self.hfrcodis() != 0
    }

    #[doc="Sets the HFRCODIS field."]
    #[inline] pub fn set_hfrcodis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Low Frequency Timeout Error Interrupt Flag"]
    #[inline] pub fn lftimeouterr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if LFTIMEOUTERR != 0"]
    #[inline] pub fn test_lftimeouterr(&self) -> bool {
        self.lftimeouterr() != 0
    }

    #[doc="Sets the LFTIMEOUTERR field."]
    #[inline] pub fn set_lftimeouterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="DPLL Lock Interrupt Flag"]
    #[inline] pub fn dpllrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if DPLLRDY != 0"]
    #[inline] pub fn test_dpllrdy(&self) -> bool {
        self.dpllrdy() != 0
    }

    #[doc="Sets the DPLLRDY field."]
    #[inline] pub fn set_dpllrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="DPLL Lock Failure Low Interrupt Flag"]
    #[inline] pub fn dplllockfaillow(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DPLLLOCKFAILLOW != 0"]
    #[inline] pub fn test_dplllockfaillow(&self) -> bool {
        self.dplllockfaillow() != 0
    }

    #[doc="Sets the DPLLLOCKFAILLOW field."]
    #[inline] pub fn set_dplllockfaillow<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DPLL Lock Failure Low Interrupt Flag"]
    #[inline] pub fn dplllockfailhigh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DPLLLOCKFAILHIGH != 0"]
    #[inline] pub fn test_dplllockfailhigh(&self) -> bool {
        self.dplllockfailhigh() != 0
    }

    #[doc="Sets the DPLLLOCKFAILHIGH field."]
    #[inline] pub fn set_dplllockfailhigh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="CMU Error Interrupt Flag"]
    #[inline] pub fn cmuerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CMUERR != 0"]
    #[inline] pub fn test_cmuerr(&self) -> bool {
        self.cmuerr() != 0
    }

    #[doc="Sets the CMUERR field."]
    #[inline] pub fn set_cmuerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for If {
    #[inline]
    fn from(other: u32) -> Self {
         If(other)
    }
}

impl ::core::fmt::Display for If {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for If {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hfrcordy() != 0 { try!(write!(f, " hfrcordy"))}
        if self.hfxordy() != 0 { try!(write!(f, " hfxordy"))}
        if self.lfrcordy() != 0 { try!(write!(f, " lfrcordy"))}
        if self.lfxordy() != 0 { try!(write!(f, " lfxordy"))}
        if self.auxhfrcordy() != 0 { try!(write!(f, " auxhfrcordy"))}
        if self.calrdy() != 0 { try!(write!(f, " calrdy"))}
        if self.calof() != 0 { try!(write!(f, " calof"))}
        if self.hfxodiserr() != 0 { try!(write!(f, " hfxodiserr"))}
        if self.hfxoautosw() != 0 { try!(write!(f, " hfxoautosw"))}
        if self.hfxopeakdeterr() != 0 { try!(write!(f, " hfxopeakdeterr"))}
        if self.hfxopeakdetrdy() != 0 { try!(write!(f, " hfxopeakdetrdy"))}
        if self.hfxoshuntoptrdy() != 0 { try!(write!(f, " hfxoshuntoptrdy"))}
        if self.hfrcodis() != 0 { try!(write!(f, " hfrcodis"))}
        if self.lftimeouterr() != 0 { try!(write!(f, " lftimeouterr"))}
        if self.dpllrdy() != 0 { try!(write!(f, " dpllrdy"))}
        if self.dplllockfaillow() != 0 { try!(write!(f, " dplllockfaillow"))}
        if self.dplllockfailhigh() != 0 { try!(write!(f, " dplllockfailhigh"))}
        if self.cmuerr() != 0 { try!(write!(f, " cmuerr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Set Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc="Set HFRCORDY Interrupt Flag"]
    #[inline] pub fn hfrcordy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HFRCORDY != 0"]
    #[inline] pub fn test_hfrcordy(&self) -> bool {
        self.hfrcordy() != 0
    }

    #[doc="Sets the HFRCORDY field."]
    #[inline] pub fn set_hfrcordy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Set HFXORDY Interrupt Flag"]
    #[inline] pub fn hfxordy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if HFXORDY != 0"]
    #[inline] pub fn test_hfxordy(&self) -> bool {
        self.hfxordy() != 0
    }

    #[doc="Sets the HFXORDY field."]
    #[inline] pub fn set_hfxordy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Set LFRCORDY Interrupt Flag"]
    #[inline] pub fn lfrcordy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if LFRCORDY != 0"]
    #[inline] pub fn test_lfrcordy(&self) -> bool {
        self.lfrcordy() != 0
    }

    #[doc="Sets the LFRCORDY field."]
    #[inline] pub fn set_lfrcordy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Set LFXORDY Interrupt Flag"]
    #[inline] pub fn lfxordy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if LFXORDY != 0"]
    #[inline] pub fn test_lfxordy(&self) -> bool {
        self.lfxordy() != 0
    }

    #[doc="Sets the LFXORDY field."]
    #[inline] pub fn set_lfxordy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Set AUXHFRCORDY Interrupt Flag"]
    #[inline] pub fn auxhfrcordy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if AUXHFRCORDY != 0"]
    #[inline] pub fn test_auxhfrcordy(&self) -> bool {
        self.auxhfrcordy() != 0
    }

    #[doc="Sets the AUXHFRCORDY field."]
    #[inline] pub fn set_auxhfrcordy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Set CALRDY Interrupt Flag"]
    #[inline] pub fn calrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CALRDY != 0"]
    #[inline] pub fn test_calrdy(&self) -> bool {
        self.calrdy() != 0
    }

    #[doc="Sets the CALRDY field."]
    #[inline] pub fn set_calrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Set CALOF Interrupt Flag"]
    #[inline] pub fn calof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CALOF != 0"]
    #[inline] pub fn test_calof(&self) -> bool {
        self.calof() != 0
    }

    #[doc="Sets the CALOF field."]
    #[inline] pub fn set_calof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Set HFXODISERR Interrupt Flag"]
    #[inline] pub fn hfxodiserr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if HFXODISERR != 0"]
    #[inline] pub fn test_hfxodiserr(&self) -> bool {
        self.hfxodiserr() != 0
    }

    #[doc="Sets the HFXODISERR field."]
    #[inline] pub fn set_hfxodiserr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Set HFXOAUTOSW Interrupt Flag"]
    #[inline] pub fn hfxoautosw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if HFXOAUTOSW != 0"]
    #[inline] pub fn test_hfxoautosw(&self) -> bool {
        self.hfxoautosw() != 0
    }

    #[doc="Sets the HFXOAUTOSW field."]
    #[inline] pub fn set_hfxoautosw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Set HFXOPEAKDETERR Interrupt Flag"]
    #[inline] pub fn hfxopeakdeterr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if HFXOPEAKDETERR != 0"]
    #[inline] pub fn test_hfxopeakdeterr(&self) -> bool {
        self.hfxopeakdeterr() != 0
    }

    #[doc="Sets the HFXOPEAKDETERR field."]
    #[inline] pub fn set_hfxopeakdeterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Set HFXOPEAKDETRDY Interrupt Flag"]
    #[inline] pub fn hfxopeakdetrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if HFXOPEAKDETRDY != 0"]
    #[inline] pub fn test_hfxopeakdetrdy(&self) -> bool {
        self.hfxopeakdetrdy() != 0
    }

    #[doc="Sets the HFXOPEAKDETRDY field."]
    #[inline] pub fn set_hfxopeakdetrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Set HFXOSHUNTOPTRDY Interrupt Flag"]
    #[inline] pub fn hfxoshuntoptrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if HFXOSHUNTOPTRDY != 0"]
    #[inline] pub fn test_hfxoshuntoptrdy(&self) -> bool {
        self.hfxoshuntoptrdy() != 0
    }

    #[doc="Sets the HFXOSHUNTOPTRDY field."]
    #[inline] pub fn set_hfxoshuntoptrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Set HFRCODIS Interrupt Flag"]
    #[inline] pub fn hfrcodis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if HFRCODIS != 0"]
    #[inline] pub fn test_hfrcodis(&self) -> bool {
        self.hfrcodis() != 0
    }

    #[doc="Sets the HFRCODIS field."]
    #[inline] pub fn set_hfrcodis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Set LFTIMEOUTERR Interrupt Flag"]
    #[inline] pub fn lftimeouterr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if LFTIMEOUTERR != 0"]
    #[inline] pub fn test_lftimeouterr(&self) -> bool {
        self.lftimeouterr() != 0
    }

    #[doc="Sets the LFTIMEOUTERR field."]
    #[inline] pub fn set_lftimeouterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Set DPLLRDY Interrupt Flag"]
    #[inline] pub fn dpllrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if DPLLRDY != 0"]
    #[inline] pub fn test_dpllrdy(&self) -> bool {
        self.dpllrdy() != 0
    }

    #[doc="Sets the DPLLRDY field."]
    #[inline] pub fn set_dpllrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Set DPLLLOCKFAILLOW Interrupt Flag"]
    #[inline] pub fn dplllockfaillow(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DPLLLOCKFAILLOW != 0"]
    #[inline] pub fn test_dplllockfaillow(&self) -> bool {
        self.dplllockfaillow() != 0
    }

    #[doc="Sets the DPLLLOCKFAILLOW field."]
    #[inline] pub fn set_dplllockfaillow<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Set DPLLLOCKFAILHIGH Interrupt Flag"]
    #[inline] pub fn dplllockfailhigh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DPLLLOCKFAILHIGH != 0"]
    #[inline] pub fn test_dplllockfailhigh(&self) -> bool {
        self.dplllockfailhigh() != 0
    }

    #[doc="Sets the DPLLLOCKFAILHIGH field."]
    #[inline] pub fn set_dplllockfailhigh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Set CMUERR Interrupt Flag"]
    #[inline] pub fn cmuerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CMUERR != 0"]
    #[inline] pub fn test_cmuerr(&self) -> bool {
        self.cmuerr() != 0
    }

    #[doc="Sets the CMUERR field."]
    #[inline] pub fn set_cmuerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Ifs {
    #[inline]
    fn from(other: u32) -> Self {
         Ifs(other)
    }
}

impl ::core::fmt::Display for Ifs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ifs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hfrcordy() != 0 { try!(write!(f, " hfrcordy"))}
        if self.hfxordy() != 0 { try!(write!(f, " hfxordy"))}
        if self.lfrcordy() != 0 { try!(write!(f, " lfrcordy"))}
        if self.lfxordy() != 0 { try!(write!(f, " lfxordy"))}
        if self.auxhfrcordy() != 0 { try!(write!(f, " auxhfrcordy"))}
        if self.calrdy() != 0 { try!(write!(f, " calrdy"))}
        if self.calof() != 0 { try!(write!(f, " calof"))}
        if self.hfxodiserr() != 0 { try!(write!(f, " hfxodiserr"))}
        if self.hfxoautosw() != 0 { try!(write!(f, " hfxoautosw"))}
        if self.hfxopeakdeterr() != 0 { try!(write!(f, " hfxopeakdeterr"))}
        if self.hfxopeakdetrdy() != 0 { try!(write!(f, " hfxopeakdetrdy"))}
        if self.hfxoshuntoptrdy() != 0 { try!(write!(f, " hfxoshuntoptrdy"))}
        if self.hfrcodis() != 0 { try!(write!(f, " hfrcodis"))}
        if self.lftimeouterr() != 0 { try!(write!(f, " lftimeouterr"))}
        if self.dpllrdy() != 0 { try!(write!(f, " dpllrdy"))}
        if self.dplllockfaillow() != 0 { try!(write!(f, " dplllockfaillow"))}
        if self.dplllockfailhigh() != 0 { try!(write!(f, " dplllockfailhigh"))}
        if self.cmuerr() != 0 { try!(write!(f, " cmuerr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Clear Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ifc(pub u32);
impl Ifc {
    #[doc="Clear HFRCORDY Interrupt Flag"]
    #[inline] pub fn hfrcordy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HFRCORDY != 0"]
    #[inline] pub fn test_hfrcordy(&self) -> bool {
        self.hfrcordy() != 0
    }

    #[doc="Sets the HFRCORDY field."]
    #[inline] pub fn set_hfrcordy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clear HFXORDY Interrupt Flag"]
    #[inline] pub fn hfxordy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if HFXORDY != 0"]
    #[inline] pub fn test_hfxordy(&self) -> bool {
        self.hfxordy() != 0
    }

    #[doc="Sets the HFXORDY field."]
    #[inline] pub fn set_hfxordy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Clear LFRCORDY Interrupt Flag"]
    #[inline] pub fn lfrcordy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if LFRCORDY != 0"]
    #[inline] pub fn test_lfrcordy(&self) -> bool {
        self.lfrcordy() != 0
    }

    #[doc="Sets the LFRCORDY field."]
    #[inline] pub fn set_lfrcordy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Clear LFXORDY Interrupt Flag"]
    #[inline] pub fn lfxordy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if LFXORDY != 0"]
    #[inline] pub fn test_lfxordy(&self) -> bool {
        self.lfxordy() != 0
    }

    #[doc="Sets the LFXORDY field."]
    #[inline] pub fn set_lfxordy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Clear AUXHFRCORDY Interrupt Flag"]
    #[inline] pub fn auxhfrcordy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if AUXHFRCORDY != 0"]
    #[inline] pub fn test_auxhfrcordy(&self) -> bool {
        self.auxhfrcordy() != 0
    }

    #[doc="Sets the AUXHFRCORDY field."]
    #[inline] pub fn set_auxhfrcordy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Clear CALRDY Interrupt Flag"]
    #[inline] pub fn calrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CALRDY != 0"]
    #[inline] pub fn test_calrdy(&self) -> bool {
        self.calrdy() != 0
    }

    #[doc="Sets the CALRDY field."]
    #[inline] pub fn set_calrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Clear CALOF Interrupt Flag"]
    #[inline] pub fn calof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CALOF != 0"]
    #[inline] pub fn test_calof(&self) -> bool {
        self.calof() != 0
    }

    #[doc="Sets the CALOF field."]
    #[inline] pub fn set_calof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Clear HFXODISERR Interrupt Flag"]
    #[inline] pub fn hfxodiserr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if HFXODISERR != 0"]
    #[inline] pub fn test_hfxodiserr(&self) -> bool {
        self.hfxodiserr() != 0
    }

    #[doc="Sets the HFXODISERR field."]
    #[inline] pub fn set_hfxodiserr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Clear HFXOAUTOSW Interrupt Flag"]
    #[inline] pub fn hfxoautosw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if HFXOAUTOSW != 0"]
    #[inline] pub fn test_hfxoautosw(&self) -> bool {
        self.hfxoautosw() != 0
    }

    #[doc="Sets the HFXOAUTOSW field."]
    #[inline] pub fn set_hfxoautosw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Clear HFXOPEAKDETERR Interrupt Flag"]
    #[inline] pub fn hfxopeakdeterr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if HFXOPEAKDETERR != 0"]
    #[inline] pub fn test_hfxopeakdeterr(&self) -> bool {
        self.hfxopeakdeterr() != 0
    }

    #[doc="Sets the HFXOPEAKDETERR field."]
    #[inline] pub fn set_hfxopeakdeterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Clear HFXOPEAKDETRDY Interrupt Flag"]
    #[inline] pub fn hfxopeakdetrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if HFXOPEAKDETRDY != 0"]
    #[inline] pub fn test_hfxopeakdetrdy(&self) -> bool {
        self.hfxopeakdetrdy() != 0
    }

    #[doc="Sets the HFXOPEAKDETRDY field."]
    #[inline] pub fn set_hfxopeakdetrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Clear HFXOSHUNTOPTRDY Interrupt Flag"]
    #[inline] pub fn hfxoshuntoptrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if HFXOSHUNTOPTRDY != 0"]
    #[inline] pub fn test_hfxoshuntoptrdy(&self) -> bool {
        self.hfxoshuntoptrdy() != 0
    }

    #[doc="Sets the HFXOSHUNTOPTRDY field."]
    #[inline] pub fn set_hfxoshuntoptrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Clear HFRCODIS Interrupt Flag"]
    #[inline] pub fn hfrcodis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if HFRCODIS != 0"]
    #[inline] pub fn test_hfrcodis(&self) -> bool {
        self.hfrcodis() != 0
    }

    #[doc="Sets the HFRCODIS field."]
    #[inline] pub fn set_hfrcodis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Clear LFTIMEOUTERR Interrupt Flag"]
    #[inline] pub fn lftimeouterr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if LFTIMEOUTERR != 0"]
    #[inline] pub fn test_lftimeouterr(&self) -> bool {
        self.lftimeouterr() != 0
    }

    #[doc="Sets the LFTIMEOUTERR field."]
    #[inline] pub fn set_lftimeouterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Clear DPLLRDY Interrupt Flag"]
    #[inline] pub fn dpllrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if DPLLRDY != 0"]
    #[inline] pub fn test_dpllrdy(&self) -> bool {
        self.dpllrdy() != 0
    }

    #[doc="Sets the DPLLRDY field."]
    #[inline] pub fn set_dpllrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Clear DPLLLOCKFAILLOW Interrupt Flag"]
    #[inline] pub fn dplllockfaillow(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DPLLLOCKFAILLOW != 0"]
    #[inline] pub fn test_dplllockfaillow(&self) -> bool {
        self.dplllockfaillow() != 0
    }

    #[doc="Sets the DPLLLOCKFAILLOW field."]
    #[inline] pub fn set_dplllockfaillow<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Clear DPLLLOCKFAILHIGH Interrupt Flag"]
    #[inline] pub fn dplllockfailhigh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DPLLLOCKFAILHIGH != 0"]
    #[inline] pub fn test_dplllockfailhigh(&self) -> bool {
        self.dplllockfailhigh() != 0
    }

    #[doc="Sets the DPLLLOCKFAILHIGH field."]
    #[inline] pub fn set_dplllockfailhigh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Clear CMUERR Interrupt Flag"]
    #[inline] pub fn cmuerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CMUERR != 0"]
    #[inline] pub fn test_cmuerr(&self) -> bool {
        self.cmuerr() != 0
    }

    #[doc="Sets the CMUERR field."]
    #[inline] pub fn set_cmuerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Ifc {
    #[inline]
    fn from(other: u32) -> Self {
         Ifc(other)
    }
}

impl ::core::fmt::Display for Ifc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ifc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hfrcordy() != 0 { try!(write!(f, " hfrcordy"))}
        if self.hfxordy() != 0 { try!(write!(f, " hfxordy"))}
        if self.lfrcordy() != 0 { try!(write!(f, " lfrcordy"))}
        if self.lfxordy() != 0 { try!(write!(f, " lfxordy"))}
        if self.auxhfrcordy() != 0 { try!(write!(f, " auxhfrcordy"))}
        if self.calrdy() != 0 { try!(write!(f, " calrdy"))}
        if self.calof() != 0 { try!(write!(f, " calof"))}
        if self.hfxodiserr() != 0 { try!(write!(f, " hfxodiserr"))}
        if self.hfxoautosw() != 0 { try!(write!(f, " hfxoautosw"))}
        if self.hfxopeakdeterr() != 0 { try!(write!(f, " hfxopeakdeterr"))}
        if self.hfxopeakdetrdy() != 0 { try!(write!(f, " hfxopeakdetrdy"))}
        if self.hfxoshuntoptrdy() != 0 { try!(write!(f, " hfxoshuntoptrdy"))}
        if self.hfrcodis() != 0 { try!(write!(f, " hfrcodis"))}
        if self.lftimeouterr() != 0 { try!(write!(f, " lftimeouterr"))}
        if self.dpllrdy() != 0 { try!(write!(f, " dpllrdy"))}
        if self.dplllockfaillow() != 0 { try!(write!(f, " dplllockfaillow"))}
        if self.dplllockfailhigh() != 0 { try!(write!(f, " dplllockfailhigh"))}
        if self.cmuerr() != 0 { try!(write!(f, " cmuerr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc="HFRCORDY Interrupt Enable"]
    #[inline] pub fn hfrcordy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HFRCORDY != 0"]
    #[inline] pub fn test_hfrcordy(&self) -> bool {
        self.hfrcordy() != 0
    }

    #[doc="Sets the HFRCORDY field."]
    #[inline] pub fn set_hfrcordy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="HFXORDY Interrupt Enable"]
    #[inline] pub fn hfxordy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if HFXORDY != 0"]
    #[inline] pub fn test_hfxordy(&self) -> bool {
        self.hfxordy() != 0
    }

    #[doc="Sets the HFXORDY field."]
    #[inline] pub fn set_hfxordy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="LFRCORDY Interrupt Enable"]
    #[inline] pub fn lfrcordy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if LFRCORDY != 0"]
    #[inline] pub fn test_lfrcordy(&self) -> bool {
        self.lfrcordy() != 0
    }

    #[doc="Sets the LFRCORDY field."]
    #[inline] pub fn set_lfrcordy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="LFXORDY Interrupt Enable"]
    #[inline] pub fn lfxordy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if LFXORDY != 0"]
    #[inline] pub fn test_lfxordy(&self) -> bool {
        self.lfxordy() != 0
    }

    #[doc="Sets the LFXORDY field."]
    #[inline] pub fn set_lfxordy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="AUXHFRCORDY Interrupt Enable"]
    #[inline] pub fn auxhfrcordy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if AUXHFRCORDY != 0"]
    #[inline] pub fn test_auxhfrcordy(&self) -> bool {
        self.auxhfrcordy() != 0
    }

    #[doc="Sets the AUXHFRCORDY field."]
    #[inline] pub fn set_auxhfrcordy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="CALRDY Interrupt Enable"]
    #[inline] pub fn calrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CALRDY != 0"]
    #[inline] pub fn test_calrdy(&self) -> bool {
        self.calrdy() != 0
    }

    #[doc="Sets the CALRDY field."]
    #[inline] pub fn set_calrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="CALOF Interrupt Enable"]
    #[inline] pub fn calof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CALOF != 0"]
    #[inline] pub fn test_calof(&self) -> bool {
        self.calof() != 0
    }

    #[doc="Sets the CALOF field."]
    #[inline] pub fn set_calof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="HFXODISERR Interrupt Enable"]
    #[inline] pub fn hfxodiserr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if HFXODISERR != 0"]
    #[inline] pub fn test_hfxodiserr(&self) -> bool {
        self.hfxodiserr() != 0
    }

    #[doc="Sets the HFXODISERR field."]
    #[inline] pub fn set_hfxodiserr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="HFXOAUTOSW Interrupt Enable"]
    #[inline] pub fn hfxoautosw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if HFXOAUTOSW != 0"]
    #[inline] pub fn test_hfxoautosw(&self) -> bool {
        self.hfxoautosw() != 0
    }

    #[doc="Sets the HFXOAUTOSW field."]
    #[inline] pub fn set_hfxoautosw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="HFXOPEAKDETERR Interrupt Enable"]
    #[inline] pub fn hfxopeakdeterr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if HFXOPEAKDETERR != 0"]
    #[inline] pub fn test_hfxopeakdeterr(&self) -> bool {
        self.hfxopeakdeterr() != 0
    }

    #[doc="Sets the HFXOPEAKDETERR field."]
    #[inline] pub fn set_hfxopeakdeterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="HFXOPEAKDETRDY Interrupt Enable"]
    #[inline] pub fn hfxopeakdetrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if HFXOPEAKDETRDY != 0"]
    #[inline] pub fn test_hfxopeakdetrdy(&self) -> bool {
        self.hfxopeakdetrdy() != 0
    }

    #[doc="Sets the HFXOPEAKDETRDY field."]
    #[inline] pub fn set_hfxopeakdetrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="HFXOSHUNTOPTRDY Interrupt Enable"]
    #[inline] pub fn hfxoshuntoptrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if HFXOSHUNTOPTRDY != 0"]
    #[inline] pub fn test_hfxoshuntoptrdy(&self) -> bool {
        self.hfxoshuntoptrdy() != 0
    }

    #[doc="Sets the HFXOSHUNTOPTRDY field."]
    #[inline] pub fn set_hfxoshuntoptrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="HFRCODIS Interrupt Enable"]
    #[inline] pub fn hfrcodis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if HFRCODIS != 0"]
    #[inline] pub fn test_hfrcodis(&self) -> bool {
        self.hfrcodis() != 0
    }

    #[doc="Sets the HFRCODIS field."]
    #[inline] pub fn set_hfrcodis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="LFTIMEOUTERR Interrupt Enable"]
    #[inline] pub fn lftimeouterr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if LFTIMEOUTERR != 0"]
    #[inline] pub fn test_lftimeouterr(&self) -> bool {
        self.lftimeouterr() != 0
    }

    #[doc="Sets the LFTIMEOUTERR field."]
    #[inline] pub fn set_lftimeouterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="DPLLRDY Interrupt Enable"]
    #[inline] pub fn dpllrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if DPLLRDY != 0"]
    #[inline] pub fn test_dpllrdy(&self) -> bool {
        self.dpllrdy() != 0
    }

    #[doc="Sets the DPLLRDY field."]
    #[inline] pub fn set_dpllrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="DPLLLOCKFAILLOW Interrupt Enable"]
    #[inline] pub fn dplllockfaillow(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DPLLLOCKFAILLOW != 0"]
    #[inline] pub fn test_dplllockfaillow(&self) -> bool {
        self.dplllockfaillow() != 0
    }

    #[doc="Sets the DPLLLOCKFAILLOW field."]
    #[inline] pub fn set_dplllockfaillow<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DPLLLOCKFAILHIGH Interrupt Enable"]
    #[inline] pub fn dplllockfailhigh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DPLLLOCKFAILHIGH != 0"]
    #[inline] pub fn test_dplllockfailhigh(&self) -> bool {
        self.dplllockfailhigh() != 0
    }

    #[doc="Sets the DPLLLOCKFAILHIGH field."]
    #[inline] pub fn set_dplllockfailhigh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="CMUERR Interrupt Enable"]
    #[inline] pub fn cmuerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CMUERR != 0"]
    #[inline] pub fn test_cmuerr(&self) -> bool {
        self.cmuerr() != 0
    }

    #[doc="Sets the CMUERR field."]
    #[inline] pub fn set_cmuerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Ien {
    #[inline]
    fn from(other: u32) -> Self {
         Ien(other)
    }
}

impl ::core::fmt::Display for Ien {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ien {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hfrcordy() != 0 { try!(write!(f, " hfrcordy"))}
        if self.hfxordy() != 0 { try!(write!(f, " hfxordy"))}
        if self.lfrcordy() != 0 { try!(write!(f, " lfrcordy"))}
        if self.lfxordy() != 0 { try!(write!(f, " lfxordy"))}
        if self.auxhfrcordy() != 0 { try!(write!(f, " auxhfrcordy"))}
        if self.calrdy() != 0 { try!(write!(f, " calrdy"))}
        if self.calof() != 0 { try!(write!(f, " calof"))}
        if self.hfxodiserr() != 0 { try!(write!(f, " hfxodiserr"))}
        if self.hfxoautosw() != 0 { try!(write!(f, " hfxoautosw"))}
        if self.hfxopeakdeterr() != 0 { try!(write!(f, " hfxopeakdeterr"))}
        if self.hfxopeakdetrdy() != 0 { try!(write!(f, " hfxopeakdetrdy"))}
        if self.hfxoshuntoptrdy() != 0 { try!(write!(f, " hfxoshuntoptrdy"))}
        if self.hfrcodis() != 0 { try!(write!(f, " hfrcodis"))}
        if self.lftimeouterr() != 0 { try!(write!(f, " lftimeouterr"))}
        if self.dpllrdy() != 0 { try!(write!(f, " dpllrdy"))}
        if self.dplllockfaillow() != 0 { try!(write!(f, " dplllockfaillow"))}
        if self.dplllockfailhigh() != 0 { try!(write!(f, " dplllockfailhigh"))}
        if self.cmuerr() != 0 { try!(write!(f, " cmuerr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="High Frequency Bus Clock Enable Register 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hfbusclken0(pub u32);
impl Hfbusclken0 {
    #[doc="Advanced Encryption Standard Accelerator 0 Clock Enable"]
    #[inline] pub fn crypto0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CRYPTO0 != 0"]
    #[inline] pub fn test_crypto0(&self) -> bool {
        self.crypto0() != 0
    }

    #[doc="Sets the CRYPTO0 field."]
    #[inline] pub fn set_crypto0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Advanced Encryption Standard Accelerator 1 Clock Enable"]
    #[inline] pub fn crypto1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CRYPTO1 != 0"]
    #[inline] pub fn test_crypto1(&self) -> bool {
        self.crypto1() != 0
    }

    #[doc="Sets the CRYPTO1 field."]
    #[inline] pub fn set_crypto1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Low Energy Peripheral Interface Clock Enable"]
    #[inline] pub fn le(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if LE != 0"]
    #[inline] pub fn test_le(&self) -> bool {
        self.le() != 0
    }

    #[doc="Sets the LE field."]
    #[inline] pub fn set_le<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="General purpose Input/Output Clock Enable"]
    #[inline] pub fn gpio(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIO != 0"]
    #[inline] pub fn test_gpio(&self) -> bool {
        self.gpio() != 0
    }

    #[doc="Sets the GPIO field."]
    #[inline] pub fn set_gpio<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Peripheral Reflex System Clock Enable"]
    #[inline] pub fn prs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PRS != 0"]
    #[inline] pub fn test_prs(&self) -> bool {
        self.prs() != 0
    }

    #[doc="Sets the PRS field."]
    #[inline] pub fn set_prs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Linked Direct Memory Access Controller Clock Enable"]
    #[inline] pub fn ldma(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if LDMA != 0"]
    #[inline] pub fn test_ldma(&self) -> bool {
        self.ldma() != 0
    }

    #[doc="Sets the LDMA field."]
    #[inline] pub fn set_ldma<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="General Purpose CRC Clock Enable"]
    #[inline] pub fn gpcrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if GPCRC != 0"]
    #[inline] pub fn test_gpcrc(&self) -> bool {
        self.gpcrc() != 0
    }

    #[doc="Sets the GPCRC field."]
    #[inline] pub fn set_gpcrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for Hfbusclken0 {
    #[inline]
    fn from(other: u32) -> Self {
         Hfbusclken0(other)
    }
}

impl ::core::fmt::Display for Hfbusclken0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hfbusclken0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.crypto0() != 0 { try!(write!(f, " crypto0"))}
        if self.crypto1() != 0 { try!(write!(f, " crypto1"))}
        if self.le() != 0 { try!(write!(f, " le"))}
        if self.gpio() != 0 { try!(write!(f, " gpio"))}
        if self.prs() != 0 { try!(write!(f, " prs"))}
        if self.ldma() != 0 { try!(write!(f, " ldma"))}
        if self.gpcrc() != 0 { try!(write!(f, " gpcrc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="High Frequency Peripheral Clock Enable Register 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hfperclken0(pub u32);
impl Hfperclken0 {
    #[doc="Timer 0 Clock Enable"]
    #[inline] pub fn timer0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TIMER0 != 0"]
    #[inline] pub fn test_timer0(&self) -> bool {
        self.timer0() != 0
    }

    #[doc="Sets the TIMER0 field."]
    #[inline] pub fn set_timer0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Timer 1 Clock Enable"]
    #[inline] pub fn timer1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TIMER1 != 0"]
    #[inline] pub fn test_timer1(&self) -> bool {
        self.timer1() != 0
    }

    #[doc="Sets the TIMER1 field."]
    #[inline] pub fn set_timer1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Wide Timer 0 Clock Enable"]
    #[inline] pub fn wtimer0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WTIMER0 != 0"]
    #[inline] pub fn test_wtimer0(&self) -> bool {
        self.wtimer0() != 0
    }

    #[doc="Sets the WTIMER0 field."]
    #[inline] pub fn set_wtimer0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Wide Timer 1 Clock Enable"]
    #[inline] pub fn wtimer1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if WTIMER1 != 0"]
    #[inline] pub fn test_wtimer1(&self) -> bool {
        self.wtimer1() != 0
    }

    #[doc="Sets the WTIMER1 field."]
    #[inline] pub fn set_wtimer1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
    #[inline] pub fn usart0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if USART0 != 0"]
    #[inline] pub fn test_usart0(&self) -> bool {
        self.usart0() != 0
    }

    #[doc="Sets the USART0 field."]
    #[inline] pub fn set_usart0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
    #[inline] pub fn usart1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if USART1 != 0"]
    #[inline] pub fn test_usart1(&self) -> bool {
        self.usart1() != 0
    }

    #[doc="Sets the USART1 field."]
    #[inline] pub fn set_usart1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Universal Synchronous/Asynchronous Receiver/Transmitter 2 Clock Enable"]
    #[inline] pub fn usart2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if USART2 != 0"]
    #[inline] pub fn test_usart2(&self) -> bool {
        self.usart2() != 0
    }

    #[doc="Sets the USART2 field."]
    #[inline] pub fn set_usart2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Universal Synchronous/Asynchronous Receiver/Transmitter 3 Clock Enable"]
    #[inline] pub fn usart3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if USART3 != 0"]
    #[inline] pub fn test_usart3(&self) -> bool {
        self.usart3() != 0
    }

    #[doc="Sets the USART3 field."]
    #[inline] pub fn set_usart3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="I2C 0 Clock Enable"]
    #[inline] pub fn i2c0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if I2C0 != 0"]
    #[inline] pub fn test_i2c0(&self) -> bool {
        self.i2c0() != 0
    }

    #[doc="Sets the I2C0 field."]
    #[inline] pub fn set_i2c0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="I2C 1 Clock Enable"]
    #[inline] pub fn i2c1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if I2C1 != 0"]
    #[inline] pub fn test_i2c1(&self) -> bool {
        self.i2c1() != 0
    }

    #[doc="Sets the I2C1 field."]
    #[inline] pub fn set_i2c1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Analog Comparator 0 Clock Enable"]
    #[inline] pub fn acmp0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if ACMP0 != 0"]
    #[inline] pub fn test_acmp0(&self) -> bool {
        self.acmp0() != 0
    }

    #[doc="Sets the ACMP0 field."]
    #[inline] pub fn set_acmp0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Analog Comparator 1 Clock Enable"]
    #[inline] pub fn acmp1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if ACMP1 != 0"]
    #[inline] pub fn test_acmp1(&self) -> bool {
        self.acmp1() != 0
    }

    #[doc="Sets the ACMP1 field."]
    #[inline] pub fn set_acmp1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="CryoTimer Clock Enable"]
    #[inline] pub fn cryotimer(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CRYOTIMER != 0"]
    #[inline] pub fn test_cryotimer(&self) -> bool {
        self.cryotimer() != 0
    }

    #[doc="Sets the CRYOTIMER field."]
    #[inline] pub fn set_cryotimer<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Analog to Digital Converter 0 Clock Enable"]
    #[inline] pub fn adc0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if ADC0 != 0"]
    #[inline] pub fn test_adc0(&self) -> bool {
        self.adc0() != 0
    }

    #[doc="Sets the ADC0 field."]
    #[inline] pub fn set_adc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Current Digital to Analog Converter 0 Clock Enable"]
    #[inline] pub fn idac0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if IDAC0 != 0"]
    #[inline] pub fn test_idac0(&self) -> bool {
        self.idac0() != 0
    }

    #[doc="Sets the IDAC0 field."]
    #[inline] pub fn set_idac0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Digital to Analog Converter 0 Clock Enable"]
    #[inline] pub fn vdac0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if VDAC0 != 0"]
    #[inline] pub fn test_vdac0(&self) -> bool {
        self.vdac0() != 0
    }

    #[doc="Sets the VDAC0 field."]
    #[inline] pub fn set_vdac0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Capacitive touch sense module Clock Enable"]
    #[inline] pub fn csen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if CSEN != 0"]
    #[inline] pub fn test_csen(&self) -> bool {
        self.csen() != 0
    }

    #[doc="Sets the CSEN field."]
    #[inline] pub fn set_csen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="True Random Number Generator 0 Clock Enable"]
    #[inline] pub fn trng0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TRNG0 != 0"]
    #[inline] pub fn test_trng0(&self) -> bool {
        self.trng0() != 0
    }

    #[doc="Sets the TRNG0 field."]
    #[inline] pub fn set_trng0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

}

impl From<u32> for Hfperclken0 {
    #[inline]
    fn from(other: u32) -> Self {
         Hfperclken0(other)
    }
}

impl ::core::fmt::Display for Hfperclken0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hfperclken0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.timer0() != 0 { try!(write!(f, " timer0"))}
        if self.timer1() != 0 { try!(write!(f, " timer1"))}
        if self.wtimer0() != 0 { try!(write!(f, " wtimer0"))}
        if self.wtimer1() != 0 { try!(write!(f, " wtimer1"))}
        if self.usart0() != 0 { try!(write!(f, " usart0"))}
        if self.usart1() != 0 { try!(write!(f, " usart1"))}
        if self.usart2() != 0 { try!(write!(f, " usart2"))}
        if self.usart3() != 0 { try!(write!(f, " usart3"))}
        if self.i2c0() != 0 { try!(write!(f, " i2c0"))}
        if self.i2c1() != 0 { try!(write!(f, " i2c1"))}
        if self.acmp0() != 0 { try!(write!(f, " acmp0"))}
        if self.acmp1() != 0 { try!(write!(f, " acmp1"))}
        if self.cryotimer() != 0 { try!(write!(f, " cryotimer"))}
        if self.adc0() != 0 { try!(write!(f, " adc0"))}
        if self.idac0() != 0 { try!(write!(f, " idac0"))}
        if self.vdac0() != 0 { try!(write!(f, " vdac0"))}
        if self.csen() != 0 { try!(write!(f, " csen"))}
        if self.trng0() != 0 { try!(write!(f, " trng0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Low Frequency A Clock Enable Register 0 (Async Reg)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lfaclken0(pub u32);
impl Lfaclken0 {
    #[doc="Low Energy Timer 0 Clock Enable"]
    #[inline] pub fn letimer0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LETIMER0 != 0"]
    #[inline] pub fn test_letimer0(&self) -> bool {
        self.letimer0() != 0
    }

    #[doc="Sets the LETIMER0 field."]
    #[inline] pub fn set_letimer0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Low Energy Sensor Interface Clock Enable"]
    #[inline] pub fn lesense(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LESENSE != 0"]
    #[inline] pub fn test_lesense(&self) -> bool {
        self.lesense() != 0
    }

    #[doc="Sets the LESENSE field."]
    #[inline] pub fn set_lesense<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Lfaclken0 {
    #[inline]
    fn from(other: u32) -> Self {
         Lfaclken0(other)
    }
}

impl ::core::fmt::Display for Lfaclken0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lfaclken0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.letimer0() != 0 { try!(write!(f, " letimer0"))}
        if self.lesense() != 0 { try!(write!(f, " lesense"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Low Frequency B Clock Enable Register 0 (Async Reg)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lfbclken0(pub u32);
impl Lfbclken0 {
    #[doc="Clock Enable"]
    #[inline] pub fn systick(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SYSTICK != 0"]
    #[inline] pub fn test_systick(&self) -> bool {
        self.systick() != 0
    }

    #[doc="Sets the SYSTICK field."]
    #[inline] pub fn set_systick<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Low Energy UART 0 Clock Enable"]
    #[inline] pub fn leuart0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LEUART0 != 0"]
    #[inline] pub fn test_leuart0(&self) -> bool {
        self.leuart0() != 0
    }

    #[doc="Sets the LEUART0 field."]
    #[inline] pub fn set_leuart0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Capacitive touch sense module Clock Enable"]
    #[inline] pub fn csen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CSEN != 0"]
    #[inline] pub fn test_csen(&self) -> bool {
        self.csen() != 0
    }

    #[doc="Sets the CSEN field."]
    #[inline] pub fn set_csen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u32> for Lfbclken0 {
    #[inline]
    fn from(other: u32) -> Self {
         Lfbclken0(other)
    }
}

impl ::core::fmt::Display for Lfbclken0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lfbclken0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.systick() != 0 { try!(write!(f, " systick"))}
        if self.leuart0() != 0 { try!(write!(f, " leuart0"))}
        if self.csen() != 0 { try!(write!(f, " csen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Low Frequency E Clock Enable Register 0 (Async Reg)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lfeclken0(pub u32);
impl Lfeclken0 {
    #[doc="Real-Time Counter and Calendar Clock Enable"]
    #[inline] pub fn rtcc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RTCC != 0"]
    #[inline] pub fn test_rtcc(&self) -> bool {
        self.rtcc() != 0
    }

    #[doc="Sets the RTCC field."]
    #[inline] pub fn set_rtcc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Lfeclken0 {
    #[inline]
    fn from(other: u32) -> Self {
         Lfeclken0(other)
    }
}

impl ::core::fmt::Display for Lfeclken0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lfeclken0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rtcc() != 0 { try!(write!(f, " rtcc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="High Frequency Clock Prescaler Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hfpresc(pub u32);
impl Hfpresc {
    #[doc="HFCLK Prescaler"]
    #[inline] pub fn presc(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1f) as u8) } // [12:8]
    }

    #[doc="Returns true if PRESC != 0"]
    #[inline] pub fn test_presc(&self) -> bool {
        self.presc() != 0
    }

    #[doc="Sets the PRESC field."]
    #[inline] pub fn set_presc<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="HFCLKLE prescaler"]
    #[inline] pub fn hfclklepresc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if HFCLKLEPRESC != 0"]
    #[inline] pub fn test_hfclklepresc(&self) -> bool {
        self.hfclklepresc() != 0
    }

    #[doc="Sets the HFCLKLEPRESC field."]
    #[inline] pub fn set_hfclklepresc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Hfpresc {
    #[inline]
    fn from(other: u32) -> Self {
         Hfpresc(other)
    }
}

impl ::core::fmt::Display for Hfpresc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hfpresc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.presc() != 0 { try!(write!(f, " presc=0x{:x}", self.presc()))}
        if self.hfclklepresc() != 0 { try!(write!(f, " hfclklepresc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="High Frequency Core Clock Prescaler Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hfcorepresc(pub u32);
impl Hfcorepresc {
    #[doc="HFCORECLK Prescaler"]
    #[inline] pub fn presc(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1ff) as u16) } // [16:8]
    }

    #[doc="Returns true if PRESC != 0"]
    #[inline] pub fn test_presc(&self) -> bool {
        self.presc() != 0
    }

    #[doc="Sets the PRESC field."]
    #[inline] pub fn set_presc<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Hfcorepresc {
    #[inline]
    fn from(other: u32) -> Self {
         Hfcorepresc(other)
    }
}

impl ::core::fmt::Display for Hfcorepresc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hfcorepresc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.presc() != 0 { try!(write!(f, " presc=0x{:x}", self.presc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="High Frequency Peripheral Clock Prescaler Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hfperpresc(pub u32);
impl Hfperpresc {
    #[doc="HFPERCLK Prescaler"]
    #[inline] pub fn presc(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1ff) as u16) } // [16:8]
    }

    #[doc="Returns true if PRESC != 0"]
    #[inline] pub fn test_presc(&self) -> bool {
        self.presc() != 0
    }

    #[doc="Sets the PRESC field."]
    #[inline] pub fn set_presc<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Hfperpresc {
    #[inline]
    fn from(other: u32) -> Self {
         Hfperpresc(other)
    }
}

impl ::core::fmt::Display for Hfperpresc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hfperpresc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.presc() != 0 { try!(write!(f, " presc=0x{:x}", self.presc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="High Frequency Export Clock Prescaler Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hfexppresc(pub u32);
impl Hfexppresc {
    #[doc="HFEXPCLK Prescaler"]
    #[inline] pub fn presc(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1f) as u8) } // [12:8]
    }

    #[doc="Returns true if PRESC != 0"]
    #[inline] pub fn test_presc(&self) -> bool {
        self.presc() != 0
    }

    #[doc="Sets the PRESC field."]
    #[inline] pub fn set_presc<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Hfexppresc {
    #[inline]
    fn from(other: u32) -> Self {
         Hfexppresc(other)
    }
}

impl ::core::fmt::Display for Hfexppresc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hfexppresc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.presc() != 0 { try!(write!(f, " presc=0x{:x}", self.presc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Low Frequency A Prescaler Register 0 (Async Reg)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lfapresc0(pub u32);
impl Lfapresc0 {
    #[doc="Low Energy Timer 0 Prescaler"]
    #[inline] pub fn letimer0(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if LETIMER0 != 0"]
    #[inline] pub fn test_letimer0(&self) -> bool {
        self.letimer0() != 0
    }

    #[doc="Sets the LETIMER0 field."]
    #[inline] pub fn set_letimer0<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Low Energy Sensor Interface Prescaler"]
    #[inline] pub fn lesense(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if LESENSE != 0"]
    #[inline] pub fn test_lesense(&self) -> bool {
        self.lesense() != 0
    }

    #[doc="Sets the LESENSE field."]
    #[inline] pub fn set_lesense<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Lfapresc0 {
    #[inline]
    fn from(other: u32) -> Self {
         Lfapresc0(other)
    }
}

impl ::core::fmt::Display for Lfapresc0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lfapresc0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.letimer0() != 0 { try!(write!(f, " letimer0=0x{:x}", self.letimer0()))}
        if self.lesense() != 0 { try!(write!(f, " lesense=0x{:x}", self.lesense()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Low Frequency B Prescaler Register 0 (Async Reg)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lfbpresc0(pub u32);
impl Lfbpresc0 {
    #[doc="Prescaler"]
    #[inline] pub fn systick(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if SYSTICK != 0"]
    #[inline] pub fn test_systick(&self) -> bool {
        self.systick() != 0
    }

    #[doc="Sets the SYSTICK field."]
    #[inline] pub fn set_systick<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Low Energy UART 0 Prescaler"]
    #[inline] pub fn leuart0(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if LEUART0 != 0"]
    #[inline] pub fn test_leuart0(&self) -> bool {
        self.leuart0() != 0
    }

    #[doc="Sets the LEUART0 field."]
    #[inline] pub fn set_leuart0<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Capacitive touch sense module Prescaler"]
    #[inline] pub fn csen(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if CSEN != 0"]
    #[inline] pub fn test_csen(&self) -> bool {
        self.csen() != 0
    }

    #[doc="Sets the CSEN field."]
    #[inline] pub fn set_csen<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Lfbpresc0 {
    #[inline]
    fn from(other: u32) -> Self {
         Lfbpresc0(other)
    }
}

impl ::core::fmt::Display for Lfbpresc0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lfbpresc0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.systick() != 0 { try!(write!(f, " systick=0x{:x}", self.systick()))}
        if self.leuart0() != 0 { try!(write!(f, " leuart0=0x{:x}", self.leuart0()))}
        if self.csen() != 0 { try!(write!(f, " csen=0x{:x}", self.csen()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Low Frequency E Prescaler Register 0 (Async Reg). When waking up from EM4 make sure EM4UNLATCH in EMU_CMD is set for this to take effect"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lfepresc0(pub u32);
impl Lfepresc0 {
    #[doc="Real-Time Counter and Calendar Prescaler"]
    #[inline] pub fn rtcc(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if RTCC != 0"]
    #[inline] pub fn test_rtcc(&self) -> bool {
        self.rtcc() != 0
    }

    #[doc="Sets the RTCC field."]
    #[inline] pub fn set_rtcc<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Lfepresc0 {
    #[inline]
    fn from(other: u32) -> Self {
         Lfepresc0(other)
    }
}

impl ::core::fmt::Display for Lfepresc0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lfepresc0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rtcc() != 0 { try!(write!(f, " rtcc=0x{:x}", self.rtcc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Synchronization Busy Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
    #[doc="Low Frequency A Clock Enable 0 Busy"]
    #[inline] pub fn lfaclken0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LFACLKEN0 != 0"]
    #[inline] pub fn test_lfaclken0(&self) -> bool {
        self.lfaclken0() != 0
    }

    #[doc="Sets the LFACLKEN0 field."]
    #[inline] pub fn set_lfaclken0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Low Frequency A Prescaler 0 Busy"]
    #[inline] pub fn lfapresc0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if LFAPRESC0 != 0"]
    #[inline] pub fn test_lfapresc0(&self) -> bool {
        self.lfapresc0() != 0
    }

    #[doc="Sets the LFAPRESC0 field."]
    #[inline] pub fn set_lfapresc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Low Frequency B Clock Enable 0 Busy"]
    #[inline] pub fn lfbclken0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if LFBCLKEN0 != 0"]
    #[inline] pub fn test_lfbclken0(&self) -> bool {
        self.lfbclken0() != 0
    }

    #[doc="Sets the LFBCLKEN0 field."]
    #[inline] pub fn set_lfbclken0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Low Frequency B Prescaler 0 Busy"]
    #[inline] pub fn lfbpresc0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LFBPRESC0 != 0"]
    #[inline] pub fn test_lfbpresc0(&self) -> bool {
        self.lfbpresc0() != 0
    }

    #[doc="Sets the LFBPRESC0 field."]
    #[inline] pub fn set_lfbpresc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Low Frequency E Clock Enable 0 Busy"]
    #[inline] pub fn lfeclken0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if LFECLKEN0 != 0"]
    #[inline] pub fn test_lfeclken0(&self) -> bool {
        self.lfeclken0() != 0
    }

    #[doc="Sets the LFECLKEN0 field."]
    #[inline] pub fn set_lfeclken0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Low Frequency E Prescaler 0 Busy"]
    #[inline] pub fn lfepresc0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if LFEPRESC0 != 0"]
    #[inline] pub fn test_lfepresc0(&self) -> bool {
        self.lfepresc0() != 0
    }

    #[doc="Sets the LFEPRESC0 field."]
    #[inline] pub fn set_lfepresc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="HFRCO Busy"]
    #[inline] pub fn hfrcobsy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if HFRCOBSY != 0"]
    #[inline] pub fn test_hfrcobsy(&self) -> bool {
        self.hfrcobsy() != 0
    }

    #[doc="Sets the HFRCOBSY field."]
    #[inline] pub fn set_hfrcobsy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="AUXHFRCO Busy"]
    #[inline] pub fn auxhfrcobsy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if AUXHFRCOBSY != 0"]
    #[inline] pub fn test_auxhfrcobsy(&self) -> bool {
        self.auxhfrcobsy() != 0
    }

    #[doc="Sets the AUXHFRCOBSY field."]
    #[inline] pub fn set_auxhfrcobsy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="LFRCO Busy"]
    #[inline] pub fn lfrcobsy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if LFRCOBSY != 0"]
    #[inline] pub fn test_lfrcobsy(&self) -> bool {
        self.lfrcobsy() != 0
    }

    #[doc="Sets the LFRCOBSY field."]
    #[inline] pub fn set_lfrcobsy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="LFRCO VREF Busy"]
    #[inline] pub fn lfrcovrefbsy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if LFRCOVREFBSY != 0"]
    #[inline] pub fn test_lfrcovrefbsy(&self) -> bool {
        self.lfrcovrefbsy() != 0
    }

    #[doc="Sets the LFRCOVREFBSY field."]
    #[inline] pub fn set_lfrcovrefbsy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="HFXO Busy"]
    #[inline] pub fn hfxobsy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if HFXOBSY != 0"]
    #[inline] pub fn test_hfxobsy(&self) -> bool {
        self.hfxobsy() != 0
    }

    #[doc="Sets the HFXOBSY field."]
    #[inline] pub fn set_hfxobsy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="LFXO Busy"]
    #[inline] pub fn lfxobsy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if LFXOBSY != 0"]
    #[inline] pub fn test_lfxobsy(&self) -> bool {
        self.lfxobsy() != 0
    }

    #[doc="Sets the LFXOBSY field."]
    #[inline] pub fn set_lfxobsy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

}

impl From<u32> for Syncbusy {
    #[inline]
    fn from(other: u32) -> Self {
         Syncbusy(other)
    }
}

impl ::core::fmt::Display for Syncbusy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Syncbusy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lfaclken0() != 0 { try!(write!(f, " lfaclken0"))}
        if self.lfapresc0() != 0 { try!(write!(f, " lfapresc0"))}
        if self.lfbclken0() != 0 { try!(write!(f, " lfbclken0"))}
        if self.lfbpresc0() != 0 { try!(write!(f, " lfbpresc0"))}
        if self.lfeclken0() != 0 { try!(write!(f, " lfeclken0"))}
        if self.lfepresc0() != 0 { try!(write!(f, " lfepresc0"))}
        if self.hfrcobsy() != 0 { try!(write!(f, " hfrcobsy"))}
        if self.auxhfrcobsy() != 0 { try!(write!(f, " auxhfrcobsy"))}
        if self.lfrcobsy() != 0 { try!(write!(f, " lfrcobsy"))}
        if self.lfrcovrefbsy() != 0 { try!(write!(f, " lfrcovrefbsy"))}
        if self.hfxobsy() != 0 { try!(write!(f, " hfxobsy"))}
        if self.lfxobsy() != 0 { try!(write!(f, " lfxobsy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Freeze Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Freeze(pub u32);
impl Freeze {
    #[doc="Register Update Freeze"]
    #[inline] pub fn regfreeze(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if REGFREEZE != 0"]
    #[inline] pub fn test_regfreeze(&self) -> bool {
        self.regfreeze() != 0
    }

    #[doc="Sets the REGFREEZE field."]
    #[inline] pub fn set_regfreeze<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Freeze {
    #[inline]
    fn from(other: u32) -> Self {
         Freeze(other)
    }
}

impl ::core::fmt::Display for Freeze {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Freeze {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.regfreeze() != 0 { try!(write!(f, " regfreeze"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCNT Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcntctrl(pub u32);
impl Pcntctrl {
    #[doc="PCNT0 Clock Enable"]
    #[inline] pub fn pcnt0clken(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PCNT0CLKEN != 0"]
    #[inline] pub fn test_pcnt0clken(&self) -> bool {
        self.pcnt0clken() != 0
    }

    #[doc="Sets the PCNT0CLKEN field."]
    #[inline] pub fn set_pcnt0clken<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="PCNT0 Clock Select"]
    #[inline] pub fn pcnt0clksel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PCNT0CLKSEL != 0"]
    #[inline] pub fn test_pcnt0clksel(&self) -> bool {
        self.pcnt0clksel() != 0
    }

    #[doc="Sets the PCNT0CLKSEL field."]
    #[inline] pub fn set_pcnt0clksel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="PCNT1 Clock Enable"]
    #[inline] pub fn pcnt1clken(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PCNT1CLKEN != 0"]
    #[inline] pub fn test_pcnt1clken(&self) -> bool {
        self.pcnt1clken() != 0
    }

    #[doc="Sets the PCNT1CLKEN field."]
    #[inline] pub fn set_pcnt1clken<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="PCNT1 Clock Select"]
    #[inline] pub fn pcnt1clksel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PCNT1CLKSEL != 0"]
    #[inline] pub fn test_pcnt1clksel(&self) -> bool {
        self.pcnt1clksel() != 0
    }

    #[doc="Sets the PCNT1CLKSEL field."]
    #[inline] pub fn set_pcnt1clksel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="PCNT2 Clock Enable"]
    #[inline] pub fn pcnt2clken(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PCNT2CLKEN != 0"]
    #[inline] pub fn test_pcnt2clken(&self) -> bool {
        self.pcnt2clken() != 0
    }

    #[doc="Sets the PCNT2CLKEN field."]
    #[inline] pub fn set_pcnt2clken<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="PCNT2 Clock Select"]
    #[inline] pub fn pcnt2clksel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PCNT2CLKSEL != 0"]
    #[inline] pub fn test_pcnt2clksel(&self) -> bool {
        self.pcnt2clksel() != 0
    }

    #[doc="Sets the PCNT2CLKSEL field."]
    #[inline] pub fn set_pcnt2clksel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u32> for Pcntctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Pcntctrl(other)
    }
}

impl ::core::fmt::Display for Pcntctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcntctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcnt0clken() != 0 { try!(write!(f, " pcnt0clken"))}
        if self.pcnt0clksel() != 0 { try!(write!(f, " pcnt0clksel"))}
        if self.pcnt1clken() != 0 { try!(write!(f, " pcnt1clken"))}
        if self.pcnt1clksel() != 0 { try!(write!(f, " pcnt1clksel"))}
        if self.pcnt2clken() != 0 { try!(write!(f, " pcnt2clken"))}
        if self.pcnt2clksel() != 0 { try!(write!(f, " pcnt2clksel"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Adcctrl(pub u32);
impl Adcctrl {
    #[doc="ADC0 Clock Select"]
    #[inline] pub fn adc0clksel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if ADC0CLKSEL != 0"]
    #[inline] pub fn test_adc0clksel(&self) -> bool {
        self.adc0clksel() != 0
    }

    #[doc="Sets the ADC0CLKSEL field."]
    #[inline] pub fn set_adc0clksel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Invert clock selected by ADC0CLKSEL"]
    #[inline] pub fn adc0clkinv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ADC0CLKINV != 0"]
    #[inline] pub fn test_adc0clkinv(&self) -> bool {
        self.adc0clkinv() != 0
    }

    #[doc="Sets the ADC0CLKINV field."]
    #[inline] pub fn set_adc0clkinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Adcctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Adcctrl(other)
    }
}

impl ::core::fmt::Display for Adcctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Adcctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.adc0clksel() != 0 { try!(write!(f, " adc0clksel=0x{:x}", self.adc0clksel()))}
        if self.adc0clkinv() != 0 { try!(write!(f, " adc0clkinv"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O Routing Pin Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Routepen(pub u32);
impl Routepen {
    #[doc="CLKOUT0 Pin Enable"]
    #[inline] pub fn clkout0pen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CLKOUT0PEN != 0"]
    #[inline] pub fn test_clkout0pen(&self) -> bool {
        self.clkout0pen() != 0
    }

    #[doc="Sets the CLKOUT0PEN field."]
    #[inline] pub fn set_clkout0pen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="CLKOUT1 Pin Enable"]
    #[inline] pub fn clkout1pen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CLKOUT1PEN != 0"]
    #[inline] pub fn test_clkout1pen(&self) -> bool {
        self.clkout1pen() != 0
    }

    #[doc="Sets the CLKOUT1PEN field."]
    #[inline] pub fn set_clkout1pen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="CLKIN0 Pin Enable"]
    #[inline] pub fn clkin0pen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if CLKIN0PEN != 0"]
    #[inline] pub fn test_clkin0pen(&self) -> bool {
        self.clkin0pen() != 0
    }

    #[doc="Sets the CLKIN0PEN field."]
    #[inline] pub fn set_clkin0pen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Routepen {
    #[inline]
    fn from(other: u32) -> Self {
         Routepen(other)
    }
}

impl ::core::fmt::Display for Routepen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Routepen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clkout0pen() != 0 { try!(write!(f, " clkout0pen"))}
        if self.clkout1pen() != 0 { try!(write!(f, " clkout1pen"))}
        if self.clkin0pen() != 0 { try!(write!(f, " clkin0pen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O Routing Location Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Routeloc0(pub u32);
impl Routeloc0 {
    #[doc="I/O Location"]
    #[inline] pub fn clkout0loc(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if CLKOUT0LOC != 0"]
    #[inline] pub fn test_clkout0loc(&self) -> bool {
        self.clkout0loc() != 0
    }

    #[doc="Sets the CLKOUT0LOC field."]
    #[inline] pub fn set_clkout0loc<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="I/O Location"]
    #[inline] pub fn clkout1loc(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3f) as u8) } // [13:8]
    }

    #[doc="Returns true if CLKOUT1LOC != 0"]
    #[inline] pub fn test_clkout1loc(&self) -> bool {
        self.clkout1loc() != 0
    }

    #[doc="Sets the CLKOUT1LOC field."]
    #[inline] pub fn set_clkout1loc<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Routeloc0 {
    #[inline]
    fn from(other: u32) -> Self {
         Routeloc0(other)
    }
}

impl ::core::fmt::Display for Routeloc0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Routeloc0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clkout0loc() != 0 { try!(write!(f, " clkout0loc=0x{:x}", self.clkout0loc()))}
        if self.clkout1loc() != 0 { try!(write!(f, " clkout1loc=0x{:x}", self.clkout1loc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O Routing Location Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Routeloc1(pub u32);
impl Routeloc1 {
    #[doc="I/O Location"]
    #[inline] pub fn clkin0loc(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if CLKIN0LOC != 0"]
    #[inline] pub fn test_clkin0loc(&self) -> bool {
        self.clkin0loc() != 0
    }

    #[doc="Sets the CLKIN0LOC field."]
    #[inline] pub fn set_clkin0loc<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Routeloc1 {
    #[inline]
    fn from(other: u32) -> Self {
         Routeloc1(other)
    }
}

impl ::core::fmt::Display for Routeloc1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Routeloc1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clkin0loc() != 0 { try!(write!(f, " clkin0loc=0x{:x}", self.clkin0loc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Configuration Lock Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lock(pub u32);
impl Lock {
    #[doc="Configuration Lock Key"]
    #[inline] pub fn lockkey(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if LOCKKEY != 0"]
    #[inline] pub fn test_lockkey(&self) -> bool {
        self.lockkey() != 0
    }

    #[doc="Sets the LOCKKEY field."]
    #[inline] pub fn set_lockkey<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Lock {
    #[inline]
    fn from(other: u32) -> Self {
         Lock(other)
    }
}

impl ::core::fmt::Display for Lock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lockkey() != 0 { try!(write!(f, " lockkey=0x{:x}", self.lockkey()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HFRCO Spread Spectrum Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hfrcoss(pub u32);
impl Hfrcoss {
    #[doc="Spread Spectrum Amplitude"]
    #[inline] pub fn ssamp(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if SSAMP != 0"]
    #[inline] pub fn test_ssamp(&self) -> bool {
        self.ssamp() != 0
    }

    #[doc="Sets the SSAMP field."]
    #[inline] pub fn set_ssamp<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Spread Spectrum Update Interval"]
    #[inline] pub fn ssinv(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1f) as u8) } // [12:8]
    }

    #[doc="Returns true if SSINV != 0"]
    #[inline] pub fn test_ssinv(&self) -> bool {
        self.ssinv() != 0
    }

    #[doc="Sets the SSINV field."]
    #[inline] pub fn set_ssinv<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Hfrcoss {
    #[inline]
    fn from(other: u32) -> Self {
         Hfrcoss(other)
    }
}

impl ::core::fmt::Display for Hfrcoss {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hfrcoss {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ssamp() != 0 { try!(write!(f, " ssamp=0x{:x}", self.ssamp()))}
        if self.ssinv() != 0 { try!(write!(f, " ssinv=0x{:x}", self.ssinv()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

pub trait Clken {
    fn clken(&self) -> u32;
    fn set_clken(&self, value: u32);
}

impl Cmu {
    #[inline] pub fn clken<P: Clken>(&self, p: &P) -> u32 {
        p.clken()
    }
    #[inline] pub fn set_clken<P: Clken>(&self, p: &P, value: u32) {
        p.set_clken(value)
    }
}

impl Clken for super::gpio_common::GpioCommon {
    #[inline] fn clken(&self) -> u32 { CMU.hfbusclken0().gpio().into() }
    #[inline] fn set_clken(&self, value: u32) { CMU.with_hfbusclken0(|r| r.set_gpio(value)); }
}


