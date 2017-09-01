//! Register map for SYSCTL peripheral
#[allow(unused_imports)] use bobbin_common::*;

periph!(SYSCTL, Sysctl, 0x400fe000);

#[doc="Register map for SYSCTL peripheral"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sysctl(pub usize);
impl Sysctl {
    #[doc="Get the *const pointer for the DID0 register."]
    #[inline] pub fn did0_ptr(&self) -> *const Did0 { 
        (self.0 + 0x0) as *const Did0
    }

    #[doc="Get the *mut pointer for the DID0 register."]
    #[inline] pub fn did0_mut(&self) -> *mut Did0 { 
        (self.0 + 0x0) as *mut Did0
    }

    #[doc="Read the DID0 register."]
    #[inline] pub fn did0(&self) -> Did0 { 
        unsafe {
            read_volatile((self.0 + 0x0) as *const Did0)
        }
    }

    #[doc="Write the DID0 register."]
    #[inline] pub fn set_did0<F: FnOnce(Did0) -> Did0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x0) as *mut Did0, f(Did0(0)));
        }
        self
    }

    #[doc="Modify the DID0 register."]
    #[inline] pub fn with_did0<F: FnOnce(Did0) -> Did0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x0) as *mut Did0, f(self.did0()));
        }
        self
    }

    #[doc="Get the *const pointer for the DID1 register."]
    #[inline] pub fn did1_ptr(&self) -> *const Did1 { 
        (self.0 + 0x4) as *const Did1
    }

    #[doc="Get the *mut pointer for the DID1 register."]
    #[inline] pub fn did1_mut(&self) -> *mut Did1 { 
        (self.0 + 0x4) as *mut Did1
    }

    #[doc="Read the DID1 register."]
    #[inline] pub fn did1(&self) -> Did1 { 
        unsafe {
            read_volatile((self.0 + 0x4) as *const Did1)
        }
    }

    #[doc="Write the DID1 register."]
    #[inline] pub fn set_did1<F: FnOnce(Did1) -> Did1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x4) as *mut Did1, f(Did1(0)));
        }
        self
    }

    #[doc="Modify the DID1 register."]
    #[inline] pub fn with_did1<F: FnOnce(Did1) -> Did1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x4) as *mut Did1, f(self.did1()));
        }
        self
    }

    #[doc="Get the *const pointer for the PTBOCTL register."]
    #[inline] pub fn ptboctl_ptr(&self) -> *const Ptboctl { 
        (self.0 + 0x38) as *const Ptboctl
    }

    #[doc="Get the *mut pointer for the PTBOCTL register."]
    #[inline] pub fn ptboctl_mut(&self) -> *mut Ptboctl { 
        (self.0 + 0x38) as *mut Ptboctl
    }

    #[doc="Read the PTBOCTL register."]
    #[inline] pub fn ptboctl(&self) -> Ptboctl { 
        unsafe {
            read_volatile((self.0 + 0x38) as *const Ptboctl)
        }
    }

    #[doc="Write the PTBOCTL register."]
    #[inline] pub fn set_ptboctl<F: FnOnce(Ptboctl) -> Ptboctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x38) as *mut Ptboctl, f(Ptboctl(0)));
        }
        self
    }

    #[doc="Modify the PTBOCTL register."]
    #[inline] pub fn with_ptboctl<F: FnOnce(Ptboctl) -> Ptboctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x38) as *mut Ptboctl, f(self.ptboctl()));
        }
        self
    }

    #[doc="Get the *const pointer for the RIS register."]
    #[inline] pub fn ris_ptr(&self) -> *const Ris { 
        (self.0 + 0x50) as *const Ris
    }

    #[doc="Get the *mut pointer for the RIS register."]
    #[inline] pub fn ris_mut(&self) -> *mut Ris { 
        (self.0 + 0x50) as *mut Ris
    }

    #[doc="Read the RIS register."]
    #[inline] pub fn ris(&self) -> Ris { 
        unsafe {
            read_volatile((self.0 + 0x50) as *const Ris)
        }
    }

    #[doc="Write the RIS register."]
    #[inline] pub fn set_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x50) as *mut Ris, f(Ris(0)));
        }
        self
    }

    #[doc="Modify the RIS register."]
    #[inline] pub fn with_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x50) as *mut Ris, f(self.ris()));
        }
        self
    }

    #[doc="Get the *const pointer for the IMC register."]
    #[inline] pub fn imc_ptr(&self) -> *const Imc { 
        (self.0 + 0x54) as *const Imc
    }

    #[doc="Get the *mut pointer for the IMC register."]
    #[inline] pub fn imc_mut(&self) -> *mut Imc { 
        (self.0 + 0x54) as *mut Imc
    }

    #[doc="Read the IMC register."]
    #[inline] pub fn imc(&self) -> Imc { 
        unsafe {
            read_volatile((self.0 + 0x54) as *const Imc)
        }
    }

    #[doc="Write the IMC register."]
    #[inline] pub fn set_imc<F: FnOnce(Imc) -> Imc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x54) as *mut Imc, f(Imc(0)));
        }
        self
    }

    #[doc="Modify the IMC register."]
    #[inline] pub fn with_imc<F: FnOnce(Imc) -> Imc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x54) as *mut Imc, f(self.imc()));
        }
        self
    }

    #[doc="Get the *const pointer for the MISC register."]
    #[inline] pub fn misc_ptr(&self) -> *const Misc { 
        (self.0 + 0x58) as *const Misc
    }

    #[doc="Get the *mut pointer for the MISC register."]
    #[inline] pub fn misc_mut(&self) -> *mut Misc { 
        (self.0 + 0x58) as *mut Misc
    }

    #[doc="Read the MISC register."]
    #[inline] pub fn misc(&self) -> Misc { 
        unsafe {
            read_volatile((self.0 + 0x58) as *const Misc)
        }
    }

    #[doc="Write the MISC register."]
    #[inline] pub fn set_misc<F: FnOnce(Misc) -> Misc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x58) as *mut Misc, f(Misc(0)));
        }
        self
    }

    #[doc="Modify the MISC register."]
    #[inline] pub fn with_misc<F: FnOnce(Misc) -> Misc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x58) as *mut Misc, f(self.misc()));
        }
        self
    }

    #[doc="Get the *const pointer for the RESC register."]
    #[inline] pub fn resc_ptr(&self) -> *const Resc { 
        (self.0 + 0x5c) as *const Resc
    }

    #[doc="Get the *mut pointer for the RESC register."]
    #[inline] pub fn resc_mut(&self) -> *mut Resc { 
        (self.0 + 0x5c) as *mut Resc
    }

    #[doc="Read the RESC register."]
    #[inline] pub fn resc(&self) -> Resc { 
        unsafe {
            read_volatile((self.0 + 0x5c) as *const Resc)
        }
    }

    #[doc="Write the RESC register."]
    #[inline] pub fn set_resc<F: FnOnce(Resc) -> Resc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x5c) as *mut Resc, f(Resc(0)));
        }
        self
    }

    #[doc="Modify the RESC register."]
    #[inline] pub fn with_resc<F: FnOnce(Resc) -> Resc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x5c) as *mut Resc, f(self.resc()));
        }
        self
    }

    #[doc="Get the *const pointer for the PWRTC register."]
    #[inline] pub fn pwrtc_ptr(&self) -> *const Pwrtc { 
        (self.0 + 0x60) as *const Pwrtc
    }

    #[doc="Get the *mut pointer for the PWRTC register."]
    #[inline] pub fn pwrtc_mut(&self) -> *mut Pwrtc { 
        (self.0 + 0x60) as *mut Pwrtc
    }

    #[doc="Read the PWRTC register."]
    #[inline] pub fn pwrtc(&self) -> Pwrtc { 
        unsafe {
            read_volatile((self.0 + 0x60) as *const Pwrtc)
        }
    }

    #[doc="Write the PWRTC register."]
    #[inline] pub fn set_pwrtc<F: FnOnce(Pwrtc) -> Pwrtc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x60) as *mut Pwrtc, f(Pwrtc(0)));
        }
        self
    }

    #[doc="Modify the PWRTC register."]
    #[inline] pub fn with_pwrtc<F: FnOnce(Pwrtc) -> Pwrtc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x60) as *mut Pwrtc, f(self.pwrtc()));
        }
        self
    }

    #[doc="Get the *const pointer for the NMIC register."]
    #[inline] pub fn nmic_ptr(&self) -> *const Nmic { 
        (self.0 + 0x64) as *const Nmic
    }

    #[doc="Get the *mut pointer for the NMIC register."]
    #[inline] pub fn nmic_mut(&self) -> *mut Nmic { 
        (self.0 + 0x64) as *mut Nmic
    }

    #[doc="Read the NMIC register."]
    #[inline] pub fn nmic(&self) -> Nmic { 
        unsafe {
            read_volatile((self.0 + 0x64) as *const Nmic)
        }
    }

    #[doc="Write the NMIC register."]
    #[inline] pub fn set_nmic<F: FnOnce(Nmic) -> Nmic>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x64) as *mut Nmic, f(Nmic(0)));
        }
        self
    }

    #[doc="Modify the NMIC register."]
    #[inline] pub fn with_nmic<F: FnOnce(Nmic) -> Nmic>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x64) as *mut Nmic, f(self.nmic()));
        }
        self
    }

    #[doc="Get the *const pointer for the MOSCCTL register."]
    #[inline] pub fn moscctl_ptr(&self) -> *const Moscctl { 
        (self.0 + 0x7c) as *const Moscctl
    }

    #[doc="Get the *mut pointer for the MOSCCTL register."]
    #[inline] pub fn moscctl_mut(&self) -> *mut Moscctl { 
        (self.0 + 0x7c) as *mut Moscctl
    }

    #[doc="Read the MOSCCTL register."]
    #[inline] pub fn moscctl(&self) -> Moscctl { 
        unsafe {
            read_volatile((self.0 + 0x7c) as *const Moscctl)
        }
    }

    #[doc="Write the MOSCCTL register."]
    #[inline] pub fn set_moscctl<F: FnOnce(Moscctl) -> Moscctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x7c) as *mut Moscctl, f(Moscctl(0)));
        }
        self
    }

    #[doc="Modify the MOSCCTL register."]
    #[inline] pub fn with_moscctl<F: FnOnce(Moscctl) -> Moscctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x7c) as *mut Moscctl, f(self.moscctl()));
        }
        self
    }

    #[doc="Get the *const pointer for the RSCLKCFG register."]
    #[inline] pub fn rsclkcfg_ptr(&self) -> *const Rsclkcfg { 
        (self.0 + 0xb0) as *const Rsclkcfg
    }

    #[doc="Get the *mut pointer for the RSCLKCFG register."]
    #[inline] pub fn rsclkcfg_mut(&self) -> *mut Rsclkcfg { 
        (self.0 + 0xb0) as *mut Rsclkcfg
    }

    #[doc="Read the RSCLKCFG register."]
    #[inline] pub fn rsclkcfg(&self) -> Rsclkcfg { 
        unsafe {
            read_volatile((self.0 + 0xb0) as *const Rsclkcfg)
        }
    }

    #[doc="Write the RSCLKCFG register."]
    #[inline] pub fn set_rsclkcfg<F: FnOnce(Rsclkcfg) -> Rsclkcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xb0) as *mut Rsclkcfg, f(Rsclkcfg(0)));
        }
        self
    }

    #[doc="Modify the RSCLKCFG register."]
    #[inline] pub fn with_rsclkcfg<F: FnOnce(Rsclkcfg) -> Rsclkcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xb0) as *mut Rsclkcfg, f(self.rsclkcfg()));
        }
        self
    }

    #[doc="Get the *const pointer for the MEMTIM0 register."]
    #[inline] pub fn memtim0_ptr(&self) -> *const Memtim0 { 
        (self.0 + 0xc0) as *const Memtim0
    }

    #[doc="Get the *mut pointer for the MEMTIM0 register."]
    #[inline] pub fn memtim0_mut(&self) -> *mut Memtim0 { 
        (self.0 + 0xc0) as *mut Memtim0
    }

    #[doc="Read the MEMTIM0 register."]
    #[inline] pub fn memtim0(&self) -> Memtim0 { 
        unsafe {
            read_volatile((self.0 + 0xc0) as *const Memtim0)
        }
    }

    #[doc="Write the MEMTIM0 register."]
    #[inline] pub fn set_memtim0<F: FnOnce(Memtim0) -> Memtim0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xc0) as *mut Memtim0, f(Memtim0(0)));
        }
        self
    }

    #[doc="Modify the MEMTIM0 register."]
    #[inline] pub fn with_memtim0<F: FnOnce(Memtim0) -> Memtim0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xc0) as *mut Memtim0, f(self.memtim0()));
        }
        self
    }

    #[doc="Get the *const pointer for the ALTCLKCFG register."]
    #[inline] pub fn altclkcfg_ptr(&self) -> *const Altclkcfg { 
        (self.0 + 0x138) as *const Altclkcfg
    }

    #[doc="Get the *mut pointer for the ALTCLKCFG register."]
    #[inline] pub fn altclkcfg_mut(&self) -> *mut Altclkcfg { 
        (self.0 + 0x138) as *mut Altclkcfg
    }

    #[doc="Read the ALTCLKCFG register."]
    #[inline] pub fn altclkcfg(&self) -> Altclkcfg { 
        unsafe {
            read_volatile((self.0 + 0x138) as *const Altclkcfg)
        }
    }

    #[doc="Write the ALTCLKCFG register."]
    #[inline] pub fn set_altclkcfg<F: FnOnce(Altclkcfg) -> Altclkcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x138) as *mut Altclkcfg, f(Altclkcfg(0)));
        }
        self
    }

    #[doc="Modify the ALTCLKCFG register."]
    #[inline] pub fn with_altclkcfg<F: FnOnce(Altclkcfg) -> Altclkcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x138) as *mut Altclkcfg, f(self.altclkcfg()));
        }
        self
    }

    #[doc="Get the *const pointer for the DSCLKCFG register."]
    #[inline] pub fn dsclkcfg_ptr(&self) -> *const Dsclkcfg { 
        (self.0 + 0x144) as *const Dsclkcfg
    }

    #[doc="Get the *mut pointer for the DSCLKCFG register."]
    #[inline] pub fn dsclkcfg_mut(&self) -> *mut Dsclkcfg { 
        (self.0 + 0x144) as *mut Dsclkcfg
    }

    #[doc="Read the DSCLKCFG register."]
    #[inline] pub fn dsclkcfg(&self) -> Dsclkcfg { 
        unsafe {
            read_volatile((self.0 + 0x144) as *const Dsclkcfg)
        }
    }

    #[doc="Write the DSCLKCFG register."]
    #[inline] pub fn set_dsclkcfg<F: FnOnce(Dsclkcfg) -> Dsclkcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x144) as *mut Dsclkcfg, f(Dsclkcfg(0)));
        }
        self
    }

    #[doc="Modify the DSCLKCFG register."]
    #[inline] pub fn with_dsclkcfg<F: FnOnce(Dsclkcfg) -> Dsclkcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x144) as *mut Dsclkcfg, f(self.dsclkcfg()));
        }
        self
    }

    #[doc="Get the *const pointer for the DIVSCLK register."]
    #[inline] pub fn divsclk_ptr(&self) -> *const Divsclk { 
        (self.0 + 0x148) as *const Divsclk
    }

    #[doc="Get the *mut pointer for the DIVSCLK register."]
    #[inline] pub fn divsclk_mut(&self) -> *mut Divsclk { 
        (self.0 + 0x148) as *mut Divsclk
    }

    #[doc="Read the DIVSCLK register."]
    #[inline] pub fn divsclk(&self) -> Divsclk { 
        unsafe {
            read_volatile((self.0 + 0x148) as *const Divsclk)
        }
    }

    #[doc="Write the DIVSCLK register."]
    #[inline] pub fn set_divsclk<F: FnOnce(Divsclk) -> Divsclk>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x148) as *mut Divsclk, f(Divsclk(0)));
        }
        self
    }

    #[doc="Modify the DIVSCLK register."]
    #[inline] pub fn with_divsclk<F: FnOnce(Divsclk) -> Divsclk>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x148) as *mut Divsclk, f(self.divsclk()));
        }
        self
    }

    #[doc="Get the *const pointer for the SYSPROP register."]
    #[inline] pub fn sysprop_ptr(&self) -> *const Sysprop { 
        (self.0 + 0x14c) as *const Sysprop
    }

    #[doc="Get the *mut pointer for the SYSPROP register."]
    #[inline] pub fn sysprop_mut(&self) -> *mut Sysprop { 
        (self.0 + 0x14c) as *mut Sysprop
    }

    #[doc="Read the SYSPROP register."]
    #[inline] pub fn sysprop(&self) -> Sysprop { 
        unsafe {
            read_volatile((self.0 + 0x14c) as *const Sysprop)
        }
    }

    #[doc="Write the SYSPROP register."]
    #[inline] pub fn set_sysprop<F: FnOnce(Sysprop) -> Sysprop>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x14c) as *mut Sysprop, f(Sysprop(0)));
        }
        self
    }

    #[doc="Modify the SYSPROP register."]
    #[inline] pub fn with_sysprop<F: FnOnce(Sysprop) -> Sysprop>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x14c) as *mut Sysprop, f(self.sysprop()));
        }
        self
    }

    #[doc="Get the *const pointer for the PIOSCCAL register."]
    #[inline] pub fn piosccal_ptr(&self) -> *const Piosccal { 
        (self.0 + 0x150) as *const Piosccal
    }

    #[doc="Get the *mut pointer for the PIOSCCAL register."]
    #[inline] pub fn piosccal_mut(&self) -> *mut Piosccal { 
        (self.0 + 0x150) as *mut Piosccal
    }

    #[doc="Read the PIOSCCAL register."]
    #[inline] pub fn piosccal(&self) -> Piosccal { 
        unsafe {
            read_volatile((self.0 + 0x150) as *const Piosccal)
        }
    }

    #[doc="Write the PIOSCCAL register."]
    #[inline] pub fn set_piosccal<F: FnOnce(Piosccal) -> Piosccal>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x150) as *mut Piosccal, f(Piosccal(0)));
        }
        self
    }

    #[doc="Modify the PIOSCCAL register."]
    #[inline] pub fn with_piosccal<F: FnOnce(Piosccal) -> Piosccal>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x150) as *mut Piosccal, f(self.piosccal()));
        }
        self
    }

    #[doc="Get the *const pointer for the PIOSCSTAT register."]
    #[inline] pub fn pioscstat_ptr(&self) -> *const Pioscstat { 
        (self.0 + 0x154) as *const Pioscstat
    }

    #[doc="Get the *mut pointer for the PIOSCSTAT register."]
    #[inline] pub fn pioscstat_mut(&self) -> *mut Pioscstat { 
        (self.0 + 0x154) as *mut Pioscstat
    }

    #[doc="Read the PIOSCSTAT register."]
    #[inline] pub fn pioscstat(&self) -> Pioscstat { 
        unsafe {
            read_volatile((self.0 + 0x154) as *const Pioscstat)
        }
    }

    #[doc="Write the PIOSCSTAT register."]
    #[inline] pub fn set_pioscstat<F: FnOnce(Pioscstat) -> Pioscstat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x154) as *mut Pioscstat, f(Pioscstat(0)));
        }
        self
    }

    #[doc="Modify the PIOSCSTAT register."]
    #[inline] pub fn with_pioscstat<F: FnOnce(Pioscstat) -> Pioscstat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x154) as *mut Pioscstat, f(self.pioscstat()));
        }
        self
    }

    #[doc="Get the *const pointer for the PLLFREQ0 register."]
    #[inline] pub fn pllfreq0_ptr(&self) -> *const Pllfreq0 { 
        (self.0 + 0x160) as *const Pllfreq0
    }

    #[doc="Get the *mut pointer for the PLLFREQ0 register."]
    #[inline] pub fn pllfreq0_mut(&self) -> *mut Pllfreq0 { 
        (self.0 + 0x160) as *mut Pllfreq0
    }

    #[doc="Read the PLLFREQ0 register."]
    #[inline] pub fn pllfreq0(&self) -> Pllfreq0 { 
        unsafe {
            read_volatile((self.0 + 0x160) as *const Pllfreq0)
        }
    }

    #[doc="Write the PLLFREQ0 register."]
    #[inline] pub fn set_pllfreq0<F: FnOnce(Pllfreq0) -> Pllfreq0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x160) as *mut Pllfreq0, f(Pllfreq0(0)));
        }
        self
    }

    #[doc="Modify the PLLFREQ0 register."]
    #[inline] pub fn with_pllfreq0<F: FnOnce(Pllfreq0) -> Pllfreq0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x160) as *mut Pllfreq0, f(self.pllfreq0()));
        }
        self
    }

    #[doc="Get the *const pointer for the PLLFREQ1 register."]
    #[inline] pub fn pllfreq1_ptr(&self) -> *const Pllfreq1 { 
        (self.0 + 0x164) as *const Pllfreq1
    }

    #[doc="Get the *mut pointer for the PLLFREQ1 register."]
    #[inline] pub fn pllfreq1_mut(&self) -> *mut Pllfreq1 { 
        (self.0 + 0x164) as *mut Pllfreq1
    }

    #[doc="Read the PLLFREQ1 register."]
    #[inline] pub fn pllfreq1(&self) -> Pllfreq1 { 
        unsafe {
            read_volatile((self.0 + 0x164) as *const Pllfreq1)
        }
    }

    #[doc="Write the PLLFREQ1 register."]
    #[inline] pub fn set_pllfreq1<F: FnOnce(Pllfreq1) -> Pllfreq1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x164) as *mut Pllfreq1, f(Pllfreq1(0)));
        }
        self
    }

    #[doc="Modify the PLLFREQ1 register."]
    #[inline] pub fn with_pllfreq1<F: FnOnce(Pllfreq1) -> Pllfreq1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x164) as *mut Pllfreq1, f(self.pllfreq1()));
        }
        self
    }

    #[doc="Get the *const pointer for the PLLSTAT register."]
    #[inline] pub fn pllstat_ptr(&self) -> *const Pllstat { 
        (self.0 + 0x168) as *const Pllstat
    }

    #[doc="Get the *mut pointer for the PLLSTAT register."]
    #[inline] pub fn pllstat_mut(&self) -> *mut Pllstat { 
        (self.0 + 0x168) as *mut Pllstat
    }

    #[doc="Read the PLLSTAT register."]
    #[inline] pub fn pllstat(&self) -> Pllstat { 
        unsafe {
            read_volatile((self.0 + 0x168) as *const Pllstat)
        }
    }

    #[doc="Write the PLLSTAT register."]
    #[inline] pub fn set_pllstat<F: FnOnce(Pllstat) -> Pllstat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x168) as *mut Pllstat, f(Pllstat(0)));
        }
        self
    }

    #[doc="Modify the PLLSTAT register."]
    #[inline] pub fn with_pllstat<F: FnOnce(Pllstat) -> Pllstat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x168) as *mut Pllstat, f(self.pllstat()));
        }
        self
    }

    #[doc="Get the *const pointer for the SLPPWRCFG register."]
    #[inline] pub fn slppwrcfg_ptr(&self) -> *const Slppwrcfg { 
        (self.0 + 0x188) as *const Slppwrcfg
    }

    #[doc="Get the *mut pointer for the SLPPWRCFG register."]
    #[inline] pub fn slppwrcfg_mut(&self) -> *mut Slppwrcfg { 
        (self.0 + 0x188) as *mut Slppwrcfg
    }

    #[doc="Read the SLPPWRCFG register."]
    #[inline] pub fn slppwrcfg(&self) -> Slppwrcfg { 
        unsafe {
            read_volatile((self.0 + 0x188) as *const Slppwrcfg)
        }
    }

    #[doc="Write the SLPPWRCFG register."]
    #[inline] pub fn set_slppwrcfg<F: FnOnce(Slppwrcfg) -> Slppwrcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x188) as *mut Slppwrcfg, f(Slppwrcfg(0)));
        }
        self
    }

    #[doc="Modify the SLPPWRCFG register."]
    #[inline] pub fn with_slppwrcfg<F: FnOnce(Slppwrcfg) -> Slppwrcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x188) as *mut Slppwrcfg, f(self.slppwrcfg()));
        }
        self
    }

    #[doc="Get the *const pointer for the DSLPPWRCFG register."]
    #[inline] pub fn dslppwrcfg_ptr(&self) -> *const Dslppwrcfg { 
        (self.0 + 0x18c) as *const Dslppwrcfg
    }

    #[doc="Get the *mut pointer for the DSLPPWRCFG register."]
    #[inline] pub fn dslppwrcfg_mut(&self) -> *mut Dslppwrcfg { 
        (self.0 + 0x18c) as *mut Dslppwrcfg
    }

    #[doc="Read the DSLPPWRCFG register."]
    #[inline] pub fn dslppwrcfg(&self) -> Dslppwrcfg { 
        unsafe {
            read_volatile((self.0 + 0x18c) as *const Dslppwrcfg)
        }
    }

    #[doc="Write the DSLPPWRCFG register."]
    #[inline] pub fn set_dslppwrcfg<F: FnOnce(Dslppwrcfg) -> Dslppwrcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x18c) as *mut Dslppwrcfg, f(Dslppwrcfg(0)));
        }
        self
    }

    #[doc="Modify the DSLPPWRCFG register."]
    #[inline] pub fn with_dslppwrcfg<F: FnOnce(Dslppwrcfg) -> Dslppwrcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x18c) as *mut Dslppwrcfg, f(self.dslppwrcfg()));
        }
        self
    }

    #[doc="Get the *const pointer for the NVMSTAT register."]
    #[inline] pub fn nvmstat_ptr(&self) -> *const Nvmstat { 
        (self.0 + 0x1a0) as *const Nvmstat
    }

    #[doc="Get the *mut pointer for the NVMSTAT register."]
    #[inline] pub fn nvmstat_mut(&self) -> *mut Nvmstat { 
        (self.0 + 0x1a0) as *mut Nvmstat
    }

    #[doc="Read the NVMSTAT register."]
    #[inline] pub fn nvmstat(&self) -> Nvmstat { 
        unsafe {
            read_volatile((self.0 + 0x1a0) as *const Nvmstat)
        }
    }

    #[doc="Write the NVMSTAT register."]
    #[inline] pub fn set_nvmstat<F: FnOnce(Nvmstat) -> Nvmstat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x1a0) as *mut Nvmstat, f(Nvmstat(0)));
        }
        self
    }

    #[doc="Modify the NVMSTAT register."]
    #[inline] pub fn with_nvmstat<F: FnOnce(Nvmstat) -> Nvmstat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x1a0) as *mut Nvmstat, f(self.nvmstat()));
        }
        self
    }

    #[doc="Get the *const pointer for the LDOSPCTL register."]
    #[inline] pub fn ldospctl_ptr(&self) -> *const Ldospctl { 
        (self.0 + 0x1b4) as *const Ldospctl
    }

    #[doc="Get the *mut pointer for the LDOSPCTL register."]
    #[inline] pub fn ldospctl_mut(&self) -> *mut Ldospctl { 
        (self.0 + 0x1b4) as *mut Ldospctl
    }

    #[doc="Read the LDOSPCTL register."]
    #[inline] pub fn ldospctl(&self) -> Ldospctl { 
        unsafe {
            read_volatile((self.0 + 0x1b4) as *const Ldospctl)
        }
    }

    #[doc="Write the LDOSPCTL register."]
    #[inline] pub fn set_ldospctl<F: FnOnce(Ldospctl) -> Ldospctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x1b4) as *mut Ldospctl, f(Ldospctl(0)));
        }
        self
    }

    #[doc="Modify the LDOSPCTL register."]
    #[inline] pub fn with_ldospctl<F: FnOnce(Ldospctl) -> Ldospctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x1b4) as *mut Ldospctl, f(self.ldospctl()));
        }
        self
    }

    #[doc="Get the *const pointer for the LDODPCTL register."]
    #[inline] pub fn ldodpctl_ptr(&self) -> *const Ldodpctl { 
        (self.0 + 0x1bc) as *const Ldodpctl
    }

    #[doc="Get the *mut pointer for the LDODPCTL register."]
    #[inline] pub fn ldodpctl_mut(&self) -> *mut Ldodpctl { 
        (self.0 + 0x1bc) as *mut Ldodpctl
    }

    #[doc="Read the LDODPCTL register."]
    #[inline] pub fn ldodpctl(&self) -> Ldodpctl { 
        unsafe {
            read_volatile((self.0 + 0x1bc) as *const Ldodpctl)
        }
    }

    #[doc="Write the LDODPCTL register."]
    #[inline] pub fn set_ldodpctl<F: FnOnce(Ldodpctl) -> Ldodpctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x1bc) as *mut Ldodpctl, f(Ldodpctl(0)));
        }
        self
    }

    #[doc="Modify the LDODPCTL register."]
    #[inline] pub fn with_ldodpctl<F: FnOnce(Ldodpctl) -> Ldodpctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x1bc) as *mut Ldodpctl, f(self.ldodpctl()));
        }
        self
    }

    #[doc="Get the *const pointer for the RESBEHAVCTL register."]
    #[inline] pub fn resbehavctl_ptr(&self) -> *const Resbehavctl { 
        (self.0 + 0x1d8) as *const Resbehavctl
    }

    #[doc="Get the *mut pointer for the RESBEHAVCTL register."]
    #[inline] pub fn resbehavctl_mut(&self) -> *mut Resbehavctl { 
        (self.0 + 0x1d8) as *mut Resbehavctl
    }

    #[doc="Read the RESBEHAVCTL register."]
    #[inline] pub fn resbehavctl(&self) -> Resbehavctl { 
        unsafe {
            read_volatile((self.0 + 0x1d8) as *const Resbehavctl)
        }
    }

    #[doc="Write the RESBEHAVCTL register."]
    #[inline] pub fn set_resbehavctl<F: FnOnce(Resbehavctl) -> Resbehavctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x1d8) as *mut Resbehavctl, f(Resbehavctl(0)));
        }
        self
    }

    #[doc="Modify the RESBEHAVCTL register."]
    #[inline] pub fn with_resbehavctl<F: FnOnce(Resbehavctl) -> Resbehavctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x1d8) as *mut Resbehavctl, f(self.resbehavctl()));
        }
        self
    }

    #[doc="Get the *const pointer for the HSSR register."]
    #[inline] pub fn hssr_ptr(&self) -> *const Hssr { 
        (self.0 + 0x1f4) as *const Hssr
    }

    #[doc="Get the *mut pointer for the HSSR register."]
    #[inline] pub fn hssr_mut(&self) -> *mut Hssr { 
        (self.0 + 0x1f4) as *mut Hssr
    }

    #[doc="Read the HSSR register."]
    #[inline] pub fn hssr(&self) -> Hssr { 
        unsafe {
            read_volatile((self.0 + 0x1f4) as *const Hssr)
        }
    }

    #[doc="Write the HSSR register."]
    #[inline] pub fn set_hssr<F: FnOnce(Hssr) -> Hssr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x1f4) as *mut Hssr, f(Hssr(0)));
        }
        self
    }

    #[doc="Modify the HSSR register."]
    #[inline] pub fn with_hssr<F: FnOnce(Hssr) -> Hssr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x1f4) as *mut Hssr, f(self.hssr()));
        }
        self
    }

    #[doc="Get the *const pointer for the USBPDS register."]
    #[inline] pub fn usbpds_ptr(&self) -> *const Usbpds { 
        (self.0 + 0x280) as *const Usbpds
    }

    #[doc="Get the *mut pointer for the USBPDS register."]
    #[inline] pub fn usbpds_mut(&self) -> *mut Usbpds { 
        (self.0 + 0x280) as *mut Usbpds
    }

    #[doc="Read the USBPDS register."]
    #[inline] pub fn usbpds(&self) -> Usbpds { 
        unsafe {
            read_volatile((self.0 + 0x280) as *const Usbpds)
        }
    }

    #[doc="Write the USBPDS register."]
    #[inline] pub fn set_usbpds<F: FnOnce(Usbpds) -> Usbpds>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x280) as *mut Usbpds, f(Usbpds(0)));
        }
        self
    }

    #[doc="Modify the USBPDS register."]
    #[inline] pub fn with_usbpds<F: FnOnce(Usbpds) -> Usbpds>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x280) as *mut Usbpds, f(self.usbpds()));
        }
        self
    }

    #[doc="Get the *const pointer for the USBMPC register."]
    #[inline] pub fn usbmpc_ptr(&self) -> *const Usbmpc { 
        (self.0 + 0x284) as *const Usbmpc
    }

    #[doc="Get the *mut pointer for the USBMPC register."]
    #[inline] pub fn usbmpc_mut(&self) -> *mut Usbmpc { 
        (self.0 + 0x284) as *mut Usbmpc
    }

    #[doc="Read the USBMPC register."]
    #[inline] pub fn usbmpc(&self) -> Usbmpc { 
        unsafe {
            read_volatile((self.0 + 0x284) as *const Usbmpc)
        }
    }

    #[doc="Write the USBMPC register."]
    #[inline] pub fn set_usbmpc<F: FnOnce(Usbmpc) -> Usbmpc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x284) as *mut Usbmpc, f(Usbmpc(0)));
        }
        self
    }

    #[doc="Modify the USBMPC register."]
    #[inline] pub fn with_usbmpc<F: FnOnce(Usbmpc) -> Usbmpc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x284) as *mut Usbmpc, f(self.usbmpc()));
        }
        self
    }

    #[doc="Get the *const pointer for the EMACPDS register."]
    #[inline] pub fn emacpds_ptr(&self) -> *const Emacpds { 
        (self.0 + 0x288) as *const Emacpds
    }

    #[doc="Get the *mut pointer for the EMACPDS register."]
    #[inline] pub fn emacpds_mut(&self) -> *mut Emacpds { 
        (self.0 + 0x288) as *mut Emacpds
    }

    #[doc="Read the EMACPDS register."]
    #[inline] pub fn emacpds(&self) -> Emacpds { 
        unsafe {
            read_volatile((self.0 + 0x288) as *const Emacpds)
        }
    }

    #[doc="Write the EMACPDS register."]
    #[inline] pub fn set_emacpds<F: FnOnce(Emacpds) -> Emacpds>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x288) as *mut Emacpds, f(Emacpds(0)));
        }
        self
    }

    #[doc="Modify the EMACPDS register."]
    #[inline] pub fn with_emacpds<F: FnOnce(Emacpds) -> Emacpds>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x288) as *mut Emacpds, f(self.emacpds()));
        }
        self
    }

    #[doc="Get the *const pointer for the EMACMPC register."]
    #[inline] pub fn emacmpc_ptr(&self) -> *const Emacmpc { 
        (self.0 + 0x28c) as *const Emacmpc
    }

    #[doc="Get the *mut pointer for the EMACMPC register."]
    #[inline] pub fn emacmpc_mut(&self) -> *mut Emacmpc { 
        (self.0 + 0x28c) as *mut Emacmpc
    }

    #[doc="Read the EMACMPC register."]
    #[inline] pub fn emacmpc(&self) -> Emacmpc { 
        unsafe {
            read_volatile((self.0 + 0x28c) as *const Emacmpc)
        }
    }

    #[doc="Write the EMACMPC register."]
    #[inline] pub fn set_emacmpc<F: FnOnce(Emacmpc) -> Emacmpc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x28c) as *mut Emacmpc, f(Emacmpc(0)));
        }
        self
    }

    #[doc="Modify the EMACMPC register."]
    #[inline] pub fn with_emacmpc<F: FnOnce(Emacmpc) -> Emacmpc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x28c) as *mut Emacmpc, f(self.emacmpc()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPWD register."]
    #[inline] pub fn ppwd_ptr(&self) -> *const Ppwd { 
        (self.0 + 0x300) as *const Ppwd
    }

    #[doc="Get the *mut pointer for the PPWD register."]
    #[inline] pub fn ppwd_mut(&self) -> *mut Ppwd { 
        (self.0 + 0x300) as *mut Ppwd
    }

    #[doc="Read the PPWD register."]
    #[inline] pub fn ppwd(&self) -> Ppwd { 
        unsafe {
            read_volatile((self.0 + 0x300) as *const Ppwd)
        }
    }

    #[doc="Write the PPWD register."]
    #[inline] pub fn set_ppwd<F: FnOnce(Ppwd) -> Ppwd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x300) as *mut Ppwd, f(Ppwd(0)));
        }
        self
    }

    #[doc="Modify the PPWD register."]
    #[inline] pub fn with_ppwd<F: FnOnce(Ppwd) -> Ppwd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x300) as *mut Ppwd, f(self.ppwd()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPTIMER register."]
    #[inline] pub fn pptimer_ptr(&self) -> *const Pptimer { 
        (self.0 + 0x304) as *const Pptimer
    }

    #[doc="Get the *mut pointer for the PPTIMER register."]
    #[inline] pub fn pptimer_mut(&self) -> *mut Pptimer { 
        (self.0 + 0x304) as *mut Pptimer
    }

    #[doc="Read the PPTIMER register."]
    #[inline] pub fn pptimer(&self) -> Pptimer { 
        unsafe {
            read_volatile((self.0 + 0x304) as *const Pptimer)
        }
    }

    #[doc="Write the PPTIMER register."]
    #[inline] pub fn set_pptimer<F: FnOnce(Pptimer) -> Pptimer>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x304) as *mut Pptimer, f(Pptimer(0)));
        }
        self
    }

    #[doc="Modify the PPTIMER register."]
    #[inline] pub fn with_pptimer<F: FnOnce(Pptimer) -> Pptimer>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x304) as *mut Pptimer, f(self.pptimer()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPGPIO register."]
    #[inline] pub fn ppgpio_ptr(&self) -> *const Ppgpio { 
        (self.0 + 0x308) as *const Ppgpio
    }

    #[doc="Get the *mut pointer for the PPGPIO register."]
    #[inline] pub fn ppgpio_mut(&self) -> *mut Ppgpio { 
        (self.0 + 0x308) as *mut Ppgpio
    }

    #[doc="Read the PPGPIO register."]
    #[inline] pub fn ppgpio(&self) -> Ppgpio { 
        unsafe {
            read_volatile((self.0 + 0x308) as *const Ppgpio)
        }
    }

    #[doc="Write the PPGPIO register."]
    #[inline] pub fn set_ppgpio<F: FnOnce(Ppgpio) -> Ppgpio>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x308) as *mut Ppgpio, f(Ppgpio(0)));
        }
        self
    }

    #[doc="Modify the PPGPIO register."]
    #[inline] pub fn with_ppgpio<F: FnOnce(Ppgpio) -> Ppgpio>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x308) as *mut Ppgpio, f(self.ppgpio()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPDMA register."]
    #[inline] pub fn ppdma_ptr(&self) -> *const Ppdma { 
        (self.0 + 0x30c) as *const Ppdma
    }

    #[doc="Get the *mut pointer for the PPDMA register."]
    #[inline] pub fn ppdma_mut(&self) -> *mut Ppdma { 
        (self.0 + 0x30c) as *mut Ppdma
    }

    #[doc="Read the PPDMA register."]
    #[inline] pub fn ppdma(&self) -> Ppdma { 
        unsafe {
            read_volatile((self.0 + 0x30c) as *const Ppdma)
        }
    }

    #[doc="Write the PPDMA register."]
    #[inline] pub fn set_ppdma<F: FnOnce(Ppdma) -> Ppdma>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x30c) as *mut Ppdma, f(Ppdma(0)));
        }
        self
    }

    #[doc="Modify the PPDMA register."]
    #[inline] pub fn with_ppdma<F: FnOnce(Ppdma) -> Ppdma>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x30c) as *mut Ppdma, f(self.ppdma()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPEPI register."]
    #[inline] pub fn ppepi_ptr(&self) -> *const Ppepi { 
        (self.0 + 0x310) as *const Ppepi
    }

    #[doc="Get the *mut pointer for the PPEPI register."]
    #[inline] pub fn ppepi_mut(&self) -> *mut Ppepi { 
        (self.0 + 0x310) as *mut Ppepi
    }

    #[doc="Read the PPEPI register."]
    #[inline] pub fn ppepi(&self) -> Ppepi { 
        unsafe {
            read_volatile((self.0 + 0x310) as *const Ppepi)
        }
    }

    #[doc="Write the PPEPI register."]
    #[inline] pub fn set_ppepi<F: FnOnce(Ppepi) -> Ppepi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x310) as *mut Ppepi, f(Ppepi(0)));
        }
        self
    }

    #[doc="Modify the PPEPI register."]
    #[inline] pub fn with_ppepi<F: FnOnce(Ppepi) -> Ppepi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x310) as *mut Ppepi, f(self.ppepi()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPHIB register."]
    #[inline] pub fn pphib_ptr(&self) -> *const Pphib { 
        (self.0 + 0x314) as *const Pphib
    }

    #[doc="Get the *mut pointer for the PPHIB register."]
    #[inline] pub fn pphib_mut(&self) -> *mut Pphib { 
        (self.0 + 0x314) as *mut Pphib
    }

    #[doc="Read the PPHIB register."]
    #[inline] pub fn pphib(&self) -> Pphib { 
        unsafe {
            read_volatile((self.0 + 0x314) as *const Pphib)
        }
    }

    #[doc="Write the PPHIB register."]
    #[inline] pub fn set_pphib<F: FnOnce(Pphib) -> Pphib>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x314) as *mut Pphib, f(Pphib(0)));
        }
        self
    }

    #[doc="Modify the PPHIB register."]
    #[inline] pub fn with_pphib<F: FnOnce(Pphib) -> Pphib>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x314) as *mut Pphib, f(self.pphib()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPUART register."]
    #[inline] pub fn ppuart_ptr(&self) -> *const Ppuart { 
        (self.0 + 0x318) as *const Ppuart
    }

    #[doc="Get the *mut pointer for the PPUART register."]
    #[inline] pub fn ppuart_mut(&self) -> *mut Ppuart { 
        (self.0 + 0x318) as *mut Ppuart
    }

    #[doc="Read the PPUART register."]
    #[inline] pub fn ppuart(&self) -> Ppuart { 
        unsafe {
            read_volatile((self.0 + 0x318) as *const Ppuart)
        }
    }

    #[doc="Write the PPUART register."]
    #[inline] pub fn set_ppuart<F: FnOnce(Ppuart) -> Ppuart>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x318) as *mut Ppuart, f(Ppuart(0)));
        }
        self
    }

    #[doc="Modify the PPUART register."]
    #[inline] pub fn with_ppuart<F: FnOnce(Ppuart) -> Ppuart>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x318) as *mut Ppuart, f(self.ppuart()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPSSI register."]
    #[inline] pub fn ppssi_ptr(&self) -> *const Ppssi { 
        (self.0 + 0x31c) as *const Ppssi
    }

    #[doc="Get the *mut pointer for the PPSSI register."]
    #[inline] pub fn ppssi_mut(&self) -> *mut Ppssi { 
        (self.0 + 0x31c) as *mut Ppssi
    }

    #[doc="Read the PPSSI register."]
    #[inline] pub fn ppssi(&self) -> Ppssi { 
        unsafe {
            read_volatile((self.0 + 0x31c) as *const Ppssi)
        }
    }

    #[doc="Write the PPSSI register."]
    #[inline] pub fn set_ppssi<F: FnOnce(Ppssi) -> Ppssi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x31c) as *mut Ppssi, f(Ppssi(0)));
        }
        self
    }

    #[doc="Modify the PPSSI register."]
    #[inline] pub fn with_ppssi<F: FnOnce(Ppssi) -> Ppssi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x31c) as *mut Ppssi, f(self.ppssi()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPI2C register."]
    #[inline] pub fn ppi2c_ptr(&self) -> *const Ppi2c { 
        (self.0 + 0x320) as *const Ppi2c
    }

    #[doc="Get the *mut pointer for the PPI2C register."]
    #[inline] pub fn ppi2c_mut(&self) -> *mut Ppi2c { 
        (self.0 + 0x320) as *mut Ppi2c
    }

    #[doc="Read the PPI2C register."]
    #[inline] pub fn ppi2c(&self) -> Ppi2c { 
        unsafe {
            read_volatile((self.0 + 0x320) as *const Ppi2c)
        }
    }

    #[doc="Write the PPI2C register."]
    #[inline] pub fn set_ppi2c<F: FnOnce(Ppi2c) -> Ppi2c>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x320) as *mut Ppi2c, f(Ppi2c(0)));
        }
        self
    }

    #[doc="Modify the PPI2C register."]
    #[inline] pub fn with_ppi2c<F: FnOnce(Ppi2c) -> Ppi2c>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x320) as *mut Ppi2c, f(self.ppi2c()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPUSB register."]
    #[inline] pub fn ppusb_ptr(&self) -> *const Ppusb { 
        (self.0 + 0x328) as *const Ppusb
    }

    #[doc="Get the *mut pointer for the PPUSB register."]
    #[inline] pub fn ppusb_mut(&self) -> *mut Ppusb { 
        (self.0 + 0x328) as *mut Ppusb
    }

    #[doc="Read the PPUSB register."]
    #[inline] pub fn ppusb(&self) -> Ppusb { 
        unsafe {
            read_volatile((self.0 + 0x328) as *const Ppusb)
        }
    }

    #[doc="Write the PPUSB register."]
    #[inline] pub fn set_ppusb<F: FnOnce(Ppusb) -> Ppusb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x328) as *mut Ppusb, f(Ppusb(0)));
        }
        self
    }

    #[doc="Modify the PPUSB register."]
    #[inline] pub fn with_ppusb<F: FnOnce(Ppusb) -> Ppusb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x328) as *mut Ppusb, f(self.ppusb()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPEPHY register."]
    #[inline] pub fn ppephy_ptr(&self) -> *const Ppephy { 
        (self.0 + 0x330) as *const Ppephy
    }

    #[doc="Get the *mut pointer for the PPEPHY register."]
    #[inline] pub fn ppephy_mut(&self) -> *mut Ppephy { 
        (self.0 + 0x330) as *mut Ppephy
    }

    #[doc="Read the PPEPHY register."]
    #[inline] pub fn ppephy(&self) -> Ppephy { 
        unsafe {
            read_volatile((self.0 + 0x330) as *const Ppephy)
        }
    }

    #[doc="Write the PPEPHY register."]
    #[inline] pub fn set_ppephy<F: FnOnce(Ppephy) -> Ppephy>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x330) as *mut Ppephy, f(Ppephy(0)));
        }
        self
    }

    #[doc="Modify the PPEPHY register."]
    #[inline] pub fn with_ppephy<F: FnOnce(Ppephy) -> Ppephy>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x330) as *mut Ppephy, f(self.ppephy()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPCAN register."]
    #[inline] pub fn ppcan_ptr(&self) -> *const Ppcan { 
        (self.0 + 0x334) as *const Ppcan
    }

    #[doc="Get the *mut pointer for the PPCAN register."]
    #[inline] pub fn ppcan_mut(&self) -> *mut Ppcan { 
        (self.0 + 0x334) as *mut Ppcan
    }

    #[doc="Read the PPCAN register."]
    #[inline] pub fn ppcan(&self) -> Ppcan { 
        unsafe {
            read_volatile((self.0 + 0x334) as *const Ppcan)
        }
    }

    #[doc="Write the PPCAN register."]
    #[inline] pub fn set_ppcan<F: FnOnce(Ppcan) -> Ppcan>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x334) as *mut Ppcan, f(Ppcan(0)));
        }
        self
    }

    #[doc="Modify the PPCAN register."]
    #[inline] pub fn with_ppcan<F: FnOnce(Ppcan) -> Ppcan>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x334) as *mut Ppcan, f(self.ppcan()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPADC register."]
    #[inline] pub fn ppadc_ptr(&self) -> *const Ppadc { 
        (self.0 + 0x338) as *const Ppadc
    }

    #[doc="Get the *mut pointer for the PPADC register."]
    #[inline] pub fn ppadc_mut(&self) -> *mut Ppadc { 
        (self.0 + 0x338) as *mut Ppadc
    }

    #[doc="Read the PPADC register."]
    #[inline] pub fn ppadc(&self) -> Ppadc { 
        unsafe {
            read_volatile((self.0 + 0x338) as *const Ppadc)
        }
    }

    #[doc="Write the PPADC register."]
    #[inline] pub fn set_ppadc<F: FnOnce(Ppadc) -> Ppadc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x338) as *mut Ppadc, f(Ppadc(0)));
        }
        self
    }

    #[doc="Modify the PPADC register."]
    #[inline] pub fn with_ppadc<F: FnOnce(Ppadc) -> Ppadc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x338) as *mut Ppadc, f(self.ppadc()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPACMP register."]
    #[inline] pub fn ppacmp_ptr(&self) -> *const Ppacmp { 
        (self.0 + 0x33c) as *const Ppacmp
    }

    #[doc="Get the *mut pointer for the PPACMP register."]
    #[inline] pub fn ppacmp_mut(&self) -> *mut Ppacmp { 
        (self.0 + 0x33c) as *mut Ppacmp
    }

    #[doc="Read the PPACMP register."]
    #[inline] pub fn ppacmp(&self) -> Ppacmp { 
        unsafe {
            read_volatile((self.0 + 0x33c) as *const Ppacmp)
        }
    }

    #[doc="Write the PPACMP register."]
    #[inline] pub fn set_ppacmp<F: FnOnce(Ppacmp) -> Ppacmp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x33c) as *mut Ppacmp, f(Ppacmp(0)));
        }
        self
    }

    #[doc="Modify the PPACMP register."]
    #[inline] pub fn with_ppacmp<F: FnOnce(Ppacmp) -> Ppacmp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x33c) as *mut Ppacmp, f(self.ppacmp()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPPWM register."]
    #[inline] pub fn pppwm_ptr(&self) -> *const Pppwm { 
        (self.0 + 0x340) as *const Pppwm
    }

    #[doc="Get the *mut pointer for the PPPWM register."]
    #[inline] pub fn pppwm_mut(&self) -> *mut Pppwm { 
        (self.0 + 0x340) as *mut Pppwm
    }

    #[doc="Read the PPPWM register."]
    #[inline] pub fn pppwm(&self) -> Pppwm { 
        unsafe {
            read_volatile((self.0 + 0x340) as *const Pppwm)
        }
    }

    #[doc="Write the PPPWM register."]
    #[inline] pub fn set_pppwm<F: FnOnce(Pppwm) -> Pppwm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x340) as *mut Pppwm, f(Pppwm(0)));
        }
        self
    }

    #[doc="Modify the PPPWM register."]
    #[inline] pub fn with_pppwm<F: FnOnce(Pppwm) -> Pppwm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x340) as *mut Pppwm, f(self.pppwm()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPQEI register."]
    #[inline] pub fn ppqei_ptr(&self) -> *const Ppqei { 
        (self.0 + 0x344) as *const Ppqei
    }

    #[doc="Get the *mut pointer for the PPQEI register."]
    #[inline] pub fn ppqei_mut(&self) -> *mut Ppqei { 
        (self.0 + 0x344) as *mut Ppqei
    }

    #[doc="Read the PPQEI register."]
    #[inline] pub fn ppqei(&self) -> Ppqei { 
        unsafe {
            read_volatile((self.0 + 0x344) as *const Ppqei)
        }
    }

    #[doc="Write the PPQEI register."]
    #[inline] pub fn set_ppqei<F: FnOnce(Ppqei) -> Ppqei>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x344) as *mut Ppqei, f(Ppqei(0)));
        }
        self
    }

    #[doc="Modify the PPQEI register."]
    #[inline] pub fn with_ppqei<F: FnOnce(Ppqei) -> Ppqei>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x344) as *mut Ppqei, f(self.ppqei()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPLPC register."]
    #[inline] pub fn pplpc_ptr(&self) -> *const Pplpc { 
        (self.0 + 0x348) as *const Pplpc
    }

    #[doc="Get the *mut pointer for the PPLPC register."]
    #[inline] pub fn pplpc_mut(&self) -> *mut Pplpc { 
        (self.0 + 0x348) as *mut Pplpc
    }

    #[doc="Read the PPLPC register."]
    #[inline] pub fn pplpc(&self) -> Pplpc { 
        unsafe {
            read_volatile((self.0 + 0x348) as *const Pplpc)
        }
    }

    #[doc="Write the PPLPC register."]
    #[inline] pub fn set_pplpc<F: FnOnce(Pplpc) -> Pplpc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x348) as *mut Pplpc, f(Pplpc(0)));
        }
        self
    }

    #[doc="Modify the PPLPC register."]
    #[inline] pub fn with_pplpc<F: FnOnce(Pplpc) -> Pplpc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x348) as *mut Pplpc, f(self.pplpc()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPPECI register."]
    #[inline] pub fn pppeci_ptr(&self) -> *const Pppeci { 
        (self.0 + 0x350) as *const Pppeci
    }

    #[doc="Get the *mut pointer for the PPPECI register."]
    #[inline] pub fn pppeci_mut(&self) -> *mut Pppeci { 
        (self.0 + 0x350) as *mut Pppeci
    }

    #[doc="Read the PPPECI register."]
    #[inline] pub fn pppeci(&self) -> Pppeci { 
        unsafe {
            read_volatile((self.0 + 0x350) as *const Pppeci)
        }
    }

    #[doc="Write the PPPECI register."]
    #[inline] pub fn set_pppeci<F: FnOnce(Pppeci) -> Pppeci>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x350) as *mut Pppeci, f(Pppeci(0)));
        }
        self
    }

    #[doc="Modify the PPPECI register."]
    #[inline] pub fn with_pppeci<F: FnOnce(Pppeci) -> Pppeci>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x350) as *mut Pppeci, f(self.pppeci()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPFAN register."]
    #[inline] pub fn ppfan_ptr(&self) -> *const Ppfan { 
        (self.0 + 0x354) as *const Ppfan
    }

    #[doc="Get the *mut pointer for the PPFAN register."]
    #[inline] pub fn ppfan_mut(&self) -> *mut Ppfan { 
        (self.0 + 0x354) as *mut Ppfan
    }

    #[doc="Read the PPFAN register."]
    #[inline] pub fn ppfan(&self) -> Ppfan { 
        unsafe {
            read_volatile((self.0 + 0x354) as *const Ppfan)
        }
    }

    #[doc="Write the PPFAN register."]
    #[inline] pub fn set_ppfan<F: FnOnce(Ppfan) -> Ppfan>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x354) as *mut Ppfan, f(Ppfan(0)));
        }
        self
    }

    #[doc="Modify the PPFAN register."]
    #[inline] pub fn with_ppfan<F: FnOnce(Ppfan) -> Ppfan>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x354) as *mut Ppfan, f(self.ppfan()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPEEPROM register."]
    #[inline] pub fn ppeeprom_ptr(&self) -> *const Ppeeprom { 
        (self.0 + 0x358) as *const Ppeeprom
    }

    #[doc="Get the *mut pointer for the PPEEPROM register."]
    #[inline] pub fn ppeeprom_mut(&self) -> *mut Ppeeprom { 
        (self.0 + 0x358) as *mut Ppeeprom
    }

    #[doc="Read the PPEEPROM register."]
    #[inline] pub fn ppeeprom(&self) -> Ppeeprom { 
        unsafe {
            read_volatile((self.0 + 0x358) as *const Ppeeprom)
        }
    }

    #[doc="Write the PPEEPROM register."]
    #[inline] pub fn set_ppeeprom<F: FnOnce(Ppeeprom) -> Ppeeprom>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x358) as *mut Ppeeprom, f(Ppeeprom(0)));
        }
        self
    }

    #[doc="Modify the PPEEPROM register."]
    #[inline] pub fn with_ppeeprom<F: FnOnce(Ppeeprom) -> Ppeeprom>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x358) as *mut Ppeeprom, f(self.ppeeprom()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPWTIMER register."]
    #[inline] pub fn ppwtimer_ptr(&self) -> *const Ppwtimer { 
        (self.0 + 0x35c) as *const Ppwtimer
    }

    #[doc="Get the *mut pointer for the PPWTIMER register."]
    #[inline] pub fn ppwtimer_mut(&self) -> *mut Ppwtimer { 
        (self.0 + 0x35c) as *mut Ppwtimer
    }

    #[doc="Read the PPWTIMER register."]
    #[inline] pub fn ppwtimer(&self) -> Ppwtimer { 
        unsafe {
            read_volatile((self.0 + 0x35c) as *const Ppwtimer)
        }
    }

    #[doc="Write the PPWTIMER register."]
    #[inline] pub fn set_ppwtimer<F: FnOnce(Ppwtimer) -> Ppwtimer>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x35c) as *mut Ppwtimer, f(Ppwtimer(0)));
        }
        self
    }

    #[doc="Modify the PPWTIMER register."]
    #[inline] pub fn with_ppwtimer<F: FnOnce(Ppwtimer) -> Ppwtimer>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x35c) as *mut Ppwtimer, f(self.ppwtimer()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPRTS register."]
    #[inline] pub fn pprts_ptr(&self) -> *const Pprts { 
        (self.0 + 0x370) as *const Pprts
    }

    #[doc="Get the *mut pointer for the PPRTS register."]
    #[inline] pub fn pprts_mut(&self) -> *mut Pprts { 
        (self.0 + 0x370) as *mut Pprts
    }

    #[doc="Read the PPRTS register."]
    #[inline] pub fn pprts(&self) -> Pprts { 
        unsafe {
            read_volatile((self.0 + 0x370) as *const Pprts)
        }
    }

    #[doc="Write the PPRTS register."]
    #[inline] pub fn set_pprts<F: FnOnce(Pprts) -> Pprts>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x370) as *mut Pprts, f(Pprts(0)));
        }
        self
    }

    #[doc="Modify the PPRTS register."]
    #[inline] pub fn with_pprts<F: FnOnce(Pprts) -> Pprts>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x370) as *mut Pprts, f(self.pprts()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPCCM register."]
    #[inline] pub fn ppccm_ptr(&self) -> *const Ppccm { 
        (self.0 + 0x374) as *const Ppccm
    }

    #[doc="Get the *mut pointer for the PPCCM register."]
    #[inline] pub fn ppccm_mut(&self) -> *mut Ppccm { 
        (self.0 + 0x374) as *mut Ppccm
    }

    #[doc="Read the PPCCM register."]
    #[inline] pub fn ppccm(&self) -> Ppccm { 
        unsafe {
            read_volatile((self.0 + 0x374) as *const Ppccm)
        }
    }

    #[doc="Write the PPCCM register."]
    #[inline] pub fn set_ppccm<F: FnOnce(Ppccm) -> Ppccm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x374) as *mut Ppccm, f(Ppccm(0)));
        }
        self
    }

    #[doc="Modify the PPCCM register."]
    #[inline] pub fn with_ppccm<F: FnOnce(Ppccm) -> Ppccm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x374) as *mut Ppccm, f(self.ppccm()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPLCD register."]
    #[inline] pub fn pplcd_ptr(&self) -> *const Pplcd { 
        (self.0 + 0x390) as *const Pplcd
    }

    #[doc="Get the *mut pointer for the PPLCD register."]
    #[inline] pub fn pplcd_mut(&self) -> *mut Pplcd { 
        (self.0 + 0x390) as *mut Pplcd
    }

    #[doc="Read the PPLCD register."]
    #[inline] pub fn pplcd(&self) -> Pplcd { 
        unsafe {
            read_volatile((self.0 + 0x390) as *const Pplcd)
        }
    }

    #[doc="Write the PPLCD register."]
    #[inline] pub fn set_pplcd<F: FnOnce(Pplcd) -> Pplcd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x390) as *mut Pplcd, f(Pplcd(0)));
        }
        self
    }

    #[doc="Modify the PPLCD register."]
    #[inline] pub fn with_pplcd<F: FnOnce(Pplcd) -> Pplcd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x390) as *mut Pplcd, f(self.pplcd()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPOWIRE register."]
    #[inline] pub fn ppowire_ptr(&self) -> *const Ppowire { 
        (self.0 + 0x398) as *const Ppowire
    }

    #[doc="Get the *mut pointer for the PPOWIRE register."]
    #[inline] pub fn ppowire_mut(&self) -> *mut Ppowire { 
        (self.0 + 0x398) as *mut Ppowire
    }

    #[doc="Read the PPOWIRE register."]
    #[inline] pub fn ppowire(&self) -> Ppowire { 
        unsafe {
            read_volatile((self.0 + 0x398) as *const Ppowire)
        }
    }

    #[doc="Write the PPOWIRE register."]
    #[inline] pub fn set_ppowire<F: FnOnce(Ppowire) -> Ppowire>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x398) as *mut Ppowire, f(Ppowire(0)));
        }
        self
    }

    #[doc="Modify the PPOWIRE register."]
    #[inline] pub fn with_ppowire<F: FnOnce(Ppowire) -> Ppowire>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x398) as *mut Ppowire, f(self.ppowire()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPEMAC register."]
    #[inline] pub fn ppemac_ptr(&self) -> *const Ppemac { 
        (self.0 + 0x39c) as *const Ppemac
    }

    #[doc="Get the *mut pointer for the PPEMAC register."]
    #[inline] pub fn ppemac_mut(&self) -> *mut Ppemac { 
        (self.0 + 0x39c) as *mut Ppemac
    }

    #[doc="Read the PPEMAC register."]
    #[inline] pub fn ppemac(&self) -> Ppemac { 
        unsafe {
            read_volatile((self.0 + 0x39c) as *const Ppemac)
        }
    }

    #[doc="Write the PPEMAC register."]
    #[inline] pub fn set_ppemac<F: FnOnce(Ppemac) -> Ppemac>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x39c) as *mut Ppemac, f(Ppemac(0)));
        }
        self
    }

    #[doc="Modify the PPEMAC register."]
    #[inline] pub fn with_ppemac<F: FnOnce(Ppemac) -> Ppemac>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x39c) as *mut Ppemac, f(self.ppemac()));
        }
        self
    }

    #[doc="Get the *const pointer for the PPHIM register."]
    #[inline] pub fn pphim_ptr(&self) -> *const Pphim { 
        (self.0 + 0x3a4) as *const Pphim
    }

    #[doc="Get the *mut pointer for the PPHIM register."]
    #[inline] pub fn pphim_mut(&self) -> *mut Pphim { 
        (self.0 + 0x3a4) as *mut Pphim
    }

    #[doc="Read the PPHIM register."]
    #[inline] pub fn pphim(&self) -> Pphim { 
        unsafe {
            read_volatile((self.0 + 0x3a4) as *const Pphim)
        }
    }

    #[doc="Write the PPHIM register."]
    #[inline] pub fn set_pphim<F: FnOnce(Pphim) -> Pphim>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x3a4) as *mut Pphim, f(Pphim(0)));
        }
        self
    }

    #[doc="Modify the PPHIM register."]
    #[inline] pub fn with_pphim<F: FnOnce(Pphim) -> Pphim>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x3a4) as *mut Pphim, f(self.pphim()));
        }
        self
    }

    #[doc="Get the *const pointer for the SRWD register."]
    #[inline] pub fn srwd_ptr(&self) -> *const Srwd { 
        (self.0 + 0x500) as *const Srwd
    }

    #[doc="Get the *mut pointer for the SRWD register."]
    #[inline] pub fn srwd_mut(&self) -> *mut Srwd { 
        (self.0 + 0x500) as *mut Srwd
    }

    #[doc="Read the SRWD register."]
    #[inline] pub fn srwd(&self) -> Srwd { 
        unsafe {
            read_volatile((self.0 + 0x500) as *const Srwd)
        }
    }

    #[doc="Write the SRWD register."]
    #[inline] pub fn set_srwd<F: FnOnce(Srwd) -> Srwd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x500) as *mut Srwd, f(Srwd(0)));
        }
        self
    }

    #[doc="Modify the SRWD register."]
    #[inline] pub fn with_srwd<F: FnOnce(Srwd) -> Srwd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x500) as *mut Srwd, f(self.srwd()));
        }
        self
    }

    #[doc="Get the *const pointer for the SRTIMER register."]
    #[inline] pub fn srtimer_ptr(&self) -> *const Srtimer { 
        (self.0 + 0x504) as *const Srtimer
    }

    #[doc="Get the *mut pointer for the SRTIMER register."]
    #[inline] pub fn srtimer_mut(&self) -> *mut Srtimer { 
        (self.0 + 0x504) as *mut Srtimer
    }

    #[doc="Read the SRTIMER register."]
    #[inline] pub fn srtimer(&self) -> Srtimer { 
        unsafe {
            read_volatile((self.0 + 0x504) as *const Srtimer)
        }
    }

    #[doc="Write the SRTIMER register."]
    #[inline] pub fn set_srtimer<F: FnOnce(Srtimer) -> Srtimer>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x504) as *mut Srtimer, f(Srtimer(0)));
        }
        self
    }

    #[doc="Modify the SRTIMER register."]
    #[inline] pub fn with_srtimer<F: FnOnce(Srtimer) -> Srtimer>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x504) as *mut Srtimer, f(self.srtimer()));
        }
        self
    }

    #[doc="Get the *const pointer for the SRGPIO register."]
    #[inline] pub fn srgpio_ptr(&self) -> *const Srgpio { 
        (self.0 + 0x508) as *const Srgpio
    }

    #[doc="Get the *mut pointer for the SRGPIO register."]
    #[inline] pub fn srgpio_mut(&self) -> *mut Srgpio { 
        (self.0 + 0x508) as *mut Srgpio
    }

    #[doc="Read the SRGPIO register."]
    #[inline] pub fn srgpio(&self) -> Srgpio { 
        unsafe {
            read_volatile((self.0 + 0x508) as *const Srgpio)
        }
    }

    #[doc="Write the SRGPIO register."]
    #[inline] pub fn set_srgpio<F: FnOnce(Srgpio) -> Srgpio>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x508) as *mut Srgpio, f(Srgpio(0)));
        }
        self
    }

    #[doc="Modify the SRGPIO register."]
    #[inline] pub fn with_srgpio<F: FnOnce(Srgpio) -> Srgpio>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x508) as *mut Srgpio, f(self.srgpio()));
        }
        self
    }

    #[doc="Get the *const pointer for the SRDMA register."]
    #[inline] pub fn srdma_ptr(&self) -> *const Srdma { 
        (self.0 + 0x50c) as *const Srdma
    }

    #[doc="Get the *mut pointer for the SRDMA register."]
    #[inline] pub fn srdma_mut(&self) -> *mut Srdma { 
        (self.0 + 0x50c) as *mut Srdma
    }

    #[doc="Read the SRDMA register."]
    #[inline] pub fn srdma(&self) -> Srdma { 
        unsafe {
            read_volatile((self.0 + 0x50c) as *const Srdma)
        }
    }

    #[doc="Write the SRDMA register."]
    #[inline] pub fn set_srdma<F: FnOnce(Srdma) -> Srdma>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x50c) as *mut Srdma, f(Srdma(0)));
        }
        self
    }

    #[doc="Modify the SRDMA register."]
    #[inline] pub fn with_srdma<F: FnOnce(Srdma) -> Srdma>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x50c) as *mut Srdma, f(self.srdma()));
        }
        self
    }

    #[doc="Get the *const pointer for the SREPI register."]
    #[inline] pub fn srepi_ptr(&self) -> *const Srepi { 
        (self.0 + 0x510) as *const Srepi
    }

    #[doc="Get the *mut pointer for the SREPI register."]
    #[inline] pub fn srepi_mut(&self) -> *mut Srepi { 
        (self.0 + 0x510) as *mut Srepi
    }

    #[doc="Read the SREPI register."]
    #[inline] pub fn srepi(&self) -> Srepi { 
        unsafe {
            read_volatile((self.0 + 0x510) as *const Srepi)
        }
    }

    #[doc="Write the SREPI register."]
    #[inline] pub fn set_srepi<F: FnOnce(Srepi) -> Srepi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x510) as *mut Srepi, f(Srepi(0)));
        }
        self
    }

    #[doc="Modify the SREPI register."]
    #[inline] pub fn with_srepi<F: FnOnce(Srepi) -> Srepi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x510) as *mut Srepi, f(self.srepi()));
        }
        self
    }

    #[doc="Get the *const pointer for the SRHIB register."]
    #[inline] pub fn srhib_ptr(&self) -> *const Srhib { 
        (self.0 + 0x514) as *const Srhib
    }

    #[doc="Get the *mut pointer for the SRHIB register."]
    #[inline] pub fn srhib_mut(&self) -> *mut Srhib { 
        (self.0 + 0x514) as *mut Srhib
    }

    #[doc="Read the SRHIB register."]
    #[inline] pub fn srhib(&self) -> Srhib { 
        unsafe {
            read_volatile((self.0 + 0x514) as *const Srhib)
        }
    }

    #[doc="Write the SRHIB register."]
    #[inline] pub fn set_srhib<F: FnOnce(Srhib) -> Srhib>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x514) as *mut Srhib, f(Srhib(0)));
        }
        self
    }

    #[doc="Modify the SRHIB register."]
    #[inline] pub fn with_srhib<F: FnOnce(Srhib) -> Srhib>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x514) as *mut Srhib, f(self.srhib()));
        }
        self
    }

    #[doc="Get the *const pointer for the SRUART register."]
    #[inline] pub fn sruart_ptr(&self) -> *const Sruart { 
        (self.0 + 0x518) as *const Sruart
    }

    #[doc="Get the *mut pointer for the SRUART register."]
    #[inline] pub fn sruart_mut(&self) -> *mut Sruart { 
        (self.0 + 0x518) as *mut Sruart
    }

    #[doc="Read the SRUART register."]
    #[inline] pub fn sruart(&self) -> Sruart { 
        unsafe {
            read_volatile((self.0 + 0x518) as *const Sruart)
        }
    }

    #[doc="Write the SRUART register."]
    #[inline] pub fn set_sruart<F: FnOnce(Sruart) -> Sruart>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x518) as *mut Sruart, f(Sruart(0)));
        }
        self
    }

    #[doc="Modify the SRUART register."]
    #[inline] pub fn with_sruart<F: FnOnce(Sruart) -> Sruart>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x518) as *mut Sruart, f(self.sruart()));
        }
        self
    }

    #[doc="Get the *const pointer for the SRSSI register."]
    #[inline] pub fn srssi_ptr(&self) -> *const Srssi { 
        (self.0 + 0x51c) as *const Srssi
    }

    #[doc="Get the *mut pointer for the SRSSI register."]
    #[inline] pub fn srssi_mut(&self) -> *mut Srssi { 
        (self.0 + 0x51c) as *mut Srssi
    }

    #[doc="Read the SRSSI register."]
    #[inline] pub fn srssi(&self) -> Srssi { 
        unsafe {
            read_volatile((self.0 + 0x51c) as *const Srssi)
        }
    }

    #[doc="Write the SRSSI register."]
    #[inline] pub fn set_srssi<F: FnOnce(Srssi) -> Srssi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x51c) as *mut Srssi, f(Srssi(0)));
        }
        self
    }

    #[doc="Modify the SRSSI register."]
    #[inline] pub fn with_srssi<F: FnOnce(Srssi) -> Srssi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x51c) as *mut Srssi, f(self.srssi()));
        }
        self
    }

    #[doc="Get the *const pointer for the SRI2C register."]
    #[inline] pub fn sri2c_ptr(&self) -> *const Sri2c { 
        (self.0 + 0x520) as *const Sri2c
    }

    #[doc="Get the *mut pointer for the SRI2C register."]
    #[inline] pub fn sri2c_mut(&self) -> *mut Sri2c { 
        (self.0 + 0x520) as *mut Sri2c
    }

    #[doc="Read the SRI2C register."]
    #[inline] pub fn sri2c(&self) -> Sri2c { 
        unsafe {
            read_volatile((self.0 + 0x520) as *const Sri2c)
        }
    }

    #[doc="Write the SRI2C register."]
    #[inline] pub fn set_sri2c<F: FnOnce(Sri2c) -> Sri2c>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x520) as *mut Sri2c, f(Sri2c(0)));
        }
        self
    }

    #[doc="Modify the SRI2C register."]
    #[inline] pub fn with_sri2c<F: FnOnce(Sri2c) -> Sri2c>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x520) as *mut Sri2c, f(self.sri2c()));
        }
        self
    }

    #[doc="Get the *const pointer for the SRUSB register."]
    #[inline] pub fn srusb_ptr(&self) -> *const Srusb { 
        (self.0 + 0x528) as *const Srusb
    }

    #[doc="Get the *mut pointer for the SRUSB register."]
    #[inline] pub fn srusb_mut(&self) -> *mut Srusb { 
        (self.0 + 0x528) as *mut Srusb
    }

    #[doc="Read the SRUSB register."]
    #[inline] pub fn srusb(&self) -> Srusb { 
        unsafe {
            read_volatile((self.0 + 0x528) as *const Srusb)
        }
    }

    #[doc="Write the SRUSB register."]
    #[inline] pub fn set_srusb<F: FnOnce(Srusb) -> Srusb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x528) as *mut Srusb, f(Srusb(0)));
        }
        self
    }

    #[doc="Modify the SRUSB register."]
    #[inline] pub fn with_srusb<F: FnOnce(Srusb) -> Srusb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x528) as *mut Srusb, f(self.srusb()));
        }
        self
    }

    #[doc="Get the *const pointer for the SREPHY register."]
    #[inline] pub fn srephy_ptr(&self) -> *const Srephy { 
        (self.0 + 0x530) as *const Srephy
    }

    #[doc="Get the *mut pointer for the SREPHY register."]
    #[inline] pub fn srephy_mut(&self) -> *mut Srephy { 
        (self.0 + 0x530) as *mut Srephy
    }

    #[doc="Read the SREPHY register."]
    #[inline] pub fn srephy(&self) -> Srephy { 
        unsafe {
            read_volatile((self.0 + 0x530) as *const Srephy)
        }
    }

    #[doc="Write the SREPHY register."]
    #[inline] pub fn set_srephy<F: FnOnce(Srephy) -> Srephy>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x530) as *mut Srephy, f(Srephy(0)));
        }
        self
    }

    #[doc="Modify the SREPHY register."]
    #[inline] pub fn with_srephy<F: FnOnce(Srephy) -> Srephy>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x530) as *mut Srephy, f(self.srephy()));
        }
        self
    }

    #[doc="Get the *const pointer for the SRCAN register."]
    #[inline] pub fn srcan_ptr(&self) -> *const Srcan { 
        (self.0 + 0x534) as *const Srcan
    }

    #[doc="Get the *mut pointer for the SRCAN register."]
    #[inline] pub fn srcan_mut(&self) -> *mut Srcan { 
        (self.0 + 0x534) as *mut Srcan
    }

    #[doc="Read the SRCAN register."]
    #[inline] pub fn srcan(&self) -> Srcan { 
        unsafe {
            read_volatile((self.0 + 0x534) as *const Srcan)
        }
    }

    #[doc="Write the SRCAN register."]
    #[inline] pub fn set_srcan<F: FnOnce(Srcan) -> Srcan>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x534) as *mut Srcan, f(Srcan(0)));
        }
        self
    }

    #[doc="Modify the SRCAN register."]
    #[inline] pub fn with_srcan<F: FnOnce(Srcan) -> Srcan>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x534) as *mut Srcan, f(self.srcan()));
        }
        self
    }

    #[doc="Get the *const pointer for the SRADC register."]
    #[inline] pub fn sradc_ptr(&self) -> *const Sradc { 
        (self.0 + 0x538) as *const Sradc
    }

    #[doc="Get the *mut pointer for the SRADC register."]
    #[inline] pub fn sradc_mut(&self) -> *mut Sradc { 
        (self.0 + 0x538) as *mut Sradc
    }

    #[doc="Read the SRADC register."]
    #[inline] pub fn sradc(&self) -> Sradc { 
        unsafe {
            read_volatile((self.0 + 0x538) as *const Sradc)
        }
    }

    #[doc="Write the SRADC register."]
    #[inline] pub fn set_sradc<F: FnOnce(Sradc) -> Sradc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x538) as *mut Sradc, f(Sradc(0)));
        }
        self
    }

    #[doc="Modify the SRADC register."]
    #[inline] pub fn with_sradc<F: FnOnce(Sradc) -> Sradc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x538) as *mut Sradc, f(self.sradc()));
        }
        self
    }

    #[doc="Get the *const pointer for the SRACMP register."]
    #[inline] pub fn sracmp_ptr(&self) -> *const Sracmp { 
        (self.0 + 0x53c) as *const Sracmp
    }

    #[doc="Get the *mut pointer for the SRACMP register."]
    #[inline] pub fn sracmp_mut(&self) -> *mut Sracmp { 
        (self.0 + 0x53c) as *mut Sracmp
    }

    #[doc="Read the SRACMP register."]
    #[inline] pub fn sracmp(&self) -> Sracmp { 
        unsafe {
            read_volatile((self.0 + 0x53c) as *const Sracmp)
        }
    }

    #[doc="Write the SRACMP register."]
    #[inline] pub fn set_sracmp<F: FnOnce(Sracmp) -> Sracmp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x53c) as *mut Sracmp, f(Sracmp(0)));
        }
        self
    }

    #[doc="Modify the SRACMP register."]
    #[inline] pub fn with_sracmp<F: FnOnce(Sracmp) -> Sracmp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x53c) as *mut Sracmp, f(self.sracmp()));
        }
        self
    }

    #[doc="Get the *const pointer for the SRPWM register."]
    #[inline] pub fn srpwm_ptr(&self) -> *const Srpwm { 
        (self.0 + 0x540) as *const Srpwm
    }

    #[doc="Get the *mut pointer for the SRPWM register."]
    #[inline] pub fn srpwm_mut(&self) -> *mut Srpwm { 
        (self.0 + 0x540) as *mut Srpwm
    }

    #[doc="Read the SRPWM register."]
    #[inline] pub fn srpwm(&self) -> Srpwm { 
        unsafe {
            read_volatile((self.0 + 0x540) as *const Srpwm)
        }
    }

    #[doc="Write the SRPWM register."]
    #[inline] pub fn set_srpwm<F: FnOnce(Srpwm) -> Srpwm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x540) as *mut Srpwm, f(Srpwm(0)));
        }
        self
    }

    #[doc="Modify the SRPWM register."]
    #[inline] pub fn with_srpwm<F: FnOnce(Srpwm) -> Srpwm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x540) as *mut Srpwm, f(self.srpwm()));
        }
        self
    }

    #[doc="Get the *const pointer for the SRQEI register."]
    #[inline] pub fn srqei_ptr(&self) -> *const Srqei { 
        (self.0 + 0x544) as *const Srqei
    }

    #[doc="Get the *mut pointer for the SRQEI register."]
    #[inline] pub fn srqei_mut(&self) -> *mut Srqei { 
        (self.0 + 0x544) as *mut Srqei
    }

    #[doc="Read the SRQEI register."]
    #[inline] pub fn srqei(&self) -> Srqei { 
        unsafe {
            read_volatile((self.0 + 0x544) as *const Srqei)
        }
    }

    #[doc="Write the SRQEI register."]
    #[inline] pub fn set_srqei<F: FnOnce(Srqei) -> Srqei>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x544) as *mut Srqei, f(Srqei(0)));
        }
        self
    }

    #[doc="Modify the SRQEI register."]
    #[inline] pub fn with_srqei<F: FnOnce(Srqei) -> Srqei>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x544) as *mut Srqei, f(self.srqei()));
        }
        self
    }

    #[doc="Get the *const pointer for the SREEPROM register."]
    #[inline] pub fn sreeprom_ptr(&self) -> *const Sreeprom { 
        (self.0 + 0x558) as *const Sreeprom
    }

    #[doc="Get the *mut pointer for the SREEPROM register."]
    #[inline] pub fn sreeprom_mut(&self) -> *mut Sreeprom { 
        (self.0 + 0x558) as *mut Sreeprom
    }

    #[doc="Read the SREEPROM register."]
    #[inline] pub fn sreeprom(&self) -> Sreeprom { 
        unsafe {
            read_volatile((self.0 + 0x558) as *const Sreeprom)
        }
    }

    #[doc="Write the SREEPROM register."]
    #[inline] pub fn set_sreeprom<F: FnOnce(Sreeprom) -> Sreeprom>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x558) as *mut Sreeprom, f(Sreeprom(0)));
        }
        self
    }

    #[doc="Modify the SREEPROM register."]
    #[inline] pub fn with_sreeprom<F: FnOnce(Sreeprom) -> Sreeprom>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x558) as *mut Sreeprom, f(self.sreeprom()));
        }
        self
    }

    #[doc="Get the *const pointer for the SRCCM register."]
    #[inline] pub fn srccm_ptr(&self) -> *const Srccm { 
        (self.0 + 0x574) as *const Srccm
    }

    #[doc="Get the *mut pointer for the SRCCM register."]
    #[inline] pub fn srccm_mut(&self) -> *mut Srccm { 
        (self.0 + 0x574) as *mut Srccm
    }

    #[doc="Read the SRCCM register."]
    #[inline] pub fn srccm(&self) -> Srccm { 
        unsafe {
            read_volatile((self.0 + 0x574) as *const Srccm)
        }
    }

    #[doc="Write the SRCCM register."]
    #[inline] pub fn set_srccm<F: FnOnce(Srccm) -> Srccm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x574) as *mut Srccm, f(Srccm(0)));
        }
        self
    }

    #[doc="Modify the SRCCM register."]
    #[inline] pub fn with_srccm<F: FnOnce(Srccm) -> Srccm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x574) as *mut Srccm, f(self.srccm()));
        }
        self
    }

    #[doc="Get the *const pointer for the SREMAC register."]
    #[inline] pub fn sremac_ptr(&self) -> *const Sremac { 
        (self.0 + 0x59c) as *const Sremac
    }

    #[doc="Get the *mut pointer for the SREMAC register."]
    #[inline] pub fn sremac_mut(&self) -> *mut Sremac { 
        (self.0 + 0x59c) as *mut Sremac
    }

    #[doc="Read the SREMAC register."]
    #[inline] pub fn sremac(&self) -> Sremac { 
        unsafe {
            read_volatile((self.0 + 0x59c) as *const Sremac)
        }
    }

    #[doc="Write the SREMAC register."]
    #[inline] pub fn set_sremac<F: FnOnce(Sremac) -> Sremac>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x59c) as *mut Sremac, f(Sremac(0)));
        }
        self
    }

    #[doc="Modify the SREMAC register."]
    #[inline] pub fn with_sremac<F: FnOnce(Sremac) -> Sremac>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x59c) as *mut Sremac, f(self.sremac()));
        }
        self
    }

    #[doc="Get the *const pointer for the RCGCWD register."]
    #[inline] pub fn rcgcwd_ptr(&self) -> *const Rcgcwd { 
        (self.0 + 0x600) as *const Rcgcwd
    }

    #[doc="Get the *mut pointer for the RCGCWD register."]
    #[inline] pub fn rcgcwd_mut(&self) -> *mut Rcgcwd { 
        (self.0 + 0x600) as *mut Rcgcwd
    }

    #[doc="Read the RCGCWD register."]
    #[inline] pub fn rcgcwd(&self) -> Rcgcwd { 
        unsafe {
            read_volatile((self.0 + 0x600) as *const Rcgcwd)
        }
    }

    #[doc="Write the RCGCWD register."]
    #[inline] pub fn set_rcgcwd<F: FnOnce(Rcgcwd) -> Rcgcwd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x600) as *mut Rcgcwd, f(Rcgcwd(0)));
        }
        self
    }

    #[doc="Modify the RCGCWD register."]
    #[inline] pub fn with_rcgcwd<F: FnOnce(Rcgcwd) -> Rcgcwd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x600) as *mut Rcgcwd, f(self.rcgcwd()));
        }
        self
    }

    #[doc="Get the *const pointer for the RCGCTIMER register."]
    #[inline] pub fn rcgctimer_ptr(&self) -> *const Rcgctimer { 
        (self.0 + 0x604) as *const Rcgctimer
    }

    #[doc="Get the *mut pointer for the RCGCTIMER register."]
    #[inline] pub fn rcgctimer_mut(&self) -> *mut Rcgctimer { 
        (self.0 + 0x604) as *mut Rcgctimer
    }

    #[doc="Read the RCGCTIMER register."]
    #[inline] pub fn rcgctimer(&self) -> Rcgctimer { 
        unsafe {
            read_volatile((self.0 + 0x604) as *const Rcgctimer)
        }
    }

    #[doc="Write the RCGCTIMER register."]
    #[inline] pub fn set_rcgctimer<F: FnOnce(Rcgctimer) -> Rcgctimer>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x604) as *mut Rcgctimer, f(Rcgctimer(0)));
        }
        self
    }

    #[doc="Modify the RCGCTIMER register."]
    #[inline] pub fn with_rcgctimer<F: FnOnce(Rcgctimer) -> Rcgctimer>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x604) as *mut Rcgctimer, f(self.rcgctimer()));
        }
        self
    }

    #[doc="Get the *const pointer for the RCGCGPIO register."]
    #[inline] pub fn rcgcgpio_ptr(&self) -> *const Rcgcgpio { 
        (self.0 + 0x608) as *const Rcgcgpio
    }

    #[doc="Get the *mut pointer for the RCGCGPIO register."]
    #[inline] pub fn rcgcgpio_mut(&self) -> *mut Rcgcgpio { 
        (self.0 + 0x608) as *mut Rcgcgpio
    }

    #[doc="Read the RCGCGPIO register."]
    #[inline] pub fn rcgcgpio(&self) -> Rcgcgpio { 
        unsafe {
            read_volatile((self.0 + 0x608) as *const Rcgcgpio)
        }
    }

    #[doc="Write the RCGCGPIO register."]
    #[inline] pub fn set_rcgcgpio<F: FnOnce(Rcgcgpio) -> Rcgcgpio>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x608) as *mut Rcgcgpio, f(Rcgcgpio(0)));
        }
        self
    }

    #[doc="Modify the RCGCGPIO register."]
    #[inline] pub fn with_rcgcgpio<F: FnOnce(Rcgcgpio) -> Rcgcgpio>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x608) as *mut Rcgcgpio, f(self.rcgcgpio()));
        }
        self
    }

    #[doc="Get the *const pointer for the RCGCDMA register."]
    #[inline] pub fn rcgcdma_ptr(&self) -> *const Rcgcdma { 
        (self.0 + 0x60c) as *const Rcgcdma
    }

    #[doc="Get the *mut pointer for the RCGCDMA register."]
    #[inline] pub fn rcgcdma_mut(&self) -> *mut Rcgcdma { 
        (self.0 + 0x60c) as *mut Rcgcdma
    }

    #[doc="Read the RCGCDMA register."]
    #[inline] pub fn rcgcdma(&self) -> Rcgcdma { 
        unsafe {
            read_volatile((self.0 + 0x60c) as *const Rcgcdma)
        }
    }

    #[doc="Write the RCGCDMA register."]
    #[inline] pub fn set_rcgcdma<F: FnOnce(Rcgcdma) -> Rcgcdma>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x60c) as *mut Rcgcdma, f(Rcgcdma(0)));
        }
        self
    }

    #[doc="Modify the RCGCDMA register."]
    #[inline] pub fn with_rcgcdma<F: FnOnce(Rcgcdma) -> Rcgcdma>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x60c) as *mut Rcgcdma, f(self.rcgcdma()));
        }
        self
    }

    #[doc="Get the *const pointer for the RCGCEPI register."]
    #[inline] pub fn rcgcepi_ptr(&self) -> *const Rcgcepi { 
        (self.0 + 0x610) as *const Rcgcepi
    }

    #[doc="Get the *mut pointer for the RCGCEPI register."]
    #[inline] pub fn rcgcepi_mut(&self) -> *mut Rcgcepi { 
        (self.0 + 0x610) as *mut Rcgcepi
    }

    #[doc="Read the RCGCEPI register."]
    #[inline] pub fn rcgcepi(&self) -> Rcgcepi { 
        unsafe {
            read_volatile((self.0 + 0x610) as *const Rcgcepi)
        }
    }

    #[doc="Write the RCGCEPI register."]
    #[inline] pub fn set_rcgcepi<F: FnOnce(Rcgcepi) -> Rcgcepi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x610) as *mut Rcgcepi, f(Rcgcepi(0)));
        }
        self
    }

    #[doc="Modify the RCGCEPI register."]
    #[inline] pub fn with_rcgcepi<F: FnOnce(Rcgcepi) -> Rcgcepi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x610) as *mut Rcgcepi, f(self.rcgcepi()));
        }
        self
    }

    #[doc="Get the *const pointer for the RCGCHIB register."]
    #[inline] pub fn rcgchib_ptr(&self) -> *const Rcgchib { 
        (self.0 + 0x614) as *const Rcgchib
    }

    #[doc="Get the *mut pointer for the RCGCHIB register."]
    #[inline] pub fn rcgchib_mut(&self) -> *mut Rcgchib { 
        (self.0 + 0x614) as *mut Rcgchib
    }

    #[doc="Read the RCGCHIB register."]
    #[inline] pub fn rcgchib(&self) -> Rcgchib { 
        unsafe {
            read_volatile((self.0 + 0x614) as *const Rcgchib)
        }
    }

    #[doc="Write the RCGCHIB register."]
    #[inline] pub fn set_rcgchib<F: FnOnce(Rcgchib) -> Rcgchib>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x614) as *mut Rcgchib, f(Rcgchib(0)));
        }
        self
    }

    #[doc="Modify the RCGCHIB register."]
    #[inline] pub fn with_rcgchib<F: FnOnce(Rcgchib) -> Rcgchib>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x614) as *mut Rcgchib, f(self.rcgchib()));
        }
        self
    }

    #[doc="Get the *const pointer for the RCGCUART register."]
    #[inline] pub fn rcgcuart_ptr(&self) -> *const Rcgcuart { 
        (self.0 + 0x618) as *const Rcgcuart
    }

    #[doc="Get the *mut pointer for the RCGCUART register."]
    #[inline] pub fn rcgcuart_mut(&self) -> *mut Rcgcuart { 
        (self.0 + 0x618) as *mut Rcgcuart
    }

    #[doc="Read the RCGCUART register."]
    #[inline] pub fn rcgcuart(&self) -> Rcgcuart { 
        unsafe {
            read_volatile((self.0 + 0x618) as *const Rcgcuart)
        }
    }

    #[doc="Write the RCGCUART register."]
    #[inline] pub fn set_rcgcuart<F: FnOnce(Rcgcuart) -> Rcgcuart>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x618) as *mut Rcgcuart, f(Rcgcuart(0)));
        }
        self
    }

    #[doc="Modify the RCGCUART register."]
    #[inline] pub fn with_rcgcuart<F: FnOnce(Rcgcuart) -> Rcgcuart>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x618) as *mut Rcgcuart, f(self.rcgcuart()));
        }
        self
    }

    #[doc="Get the *const pointer for the RCGCSSI register."]
    #[inline] pub fn rcgcssi_ptr(&self) -> *const Rcgcssi { 
        (self.0 + 0x61c) as *const Rcgcssi
    }

    #[doc="Get the *mut pointer for the RCGCSSI register."]
    #[inline] pub fn rcgcssi_mut(&self) -> *mut Rcgcssi { 
        (self.0 + 0x61c) as *mut Rcgcssi
    }

    #[doc="Read the RCGCSSI register."]
    #[inline] pub fn rcgcssi(&self) -> Rcgcssi { 
        unsafe {
            read_volatile((self.0 + 0x61c) as *const Rcgcssi)
        }
    }

    #[doc="Write the RCGCSSI register."]
    #[inline] pub fn set_rcgcssi<F: FnOnce(Rcgcssi) -> Rcgcssi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x61c) as *mut Rcgcssi, f(Rcgcssi(0)));
        }
        self
    }

    #[doc="Modify the RCGCSSI register."]
    #[inline] pub fn with_rcgcssi<F: FnOnce(Rcgcssi) -> Rcgcssi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x61c) as *mut Rcgcssi, f(self.rcgcssi()));
        }
        self
    }

    #[doc="Get the *const pointer for the RCGCI2C register."]
    #[inline] pub fn rcgci2c_ptr(&self) -> *const Rcgci2c { 
        (self.0 + 0x620) as *const Rcgci2c
    }

    #[doc="Get the *mut pointer for the RCGCI2C register."]
    #[inline] pub fn rcgci2c_mut(&self) -> *mut Rcgci2c { 
        (self.0 + 0x620) as *mut Rcgci2c
    }

    #[doc="Read the RCGCI2C register."]
    #[inline] pub fn rcgci2c(&self) -> Rcgci2c { 
        unsafe {
            read_volatile((self.0 + 0x620) as *const Rcgci2c)
        }
    }

    #[doc="Write the RCGCI2C register."]
    #[inline] pub fn set_rcgci2c<F: FnOnce(Rcgci2c) -> Rcgci2c>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x620) as *mut Rcgci2c, f(Rcgci2c(0)));
        }
        self
    }

    #[doc="Modify the RCGCI2C register."]
    #[inline] pub fn with_rcgci2c<F: FnOnce(Rcgci2c) -> Rcgci2c>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x620) as *mut Rcgci2c, f(self.rcgci2c()));
        }
        self
    }

    #[doc="Get the *const pointer for the RCGCUSB register."]
    #[inline] pub fn rcgcusb_ptr(&self) -> *const Rcgcusb { 
        (self.0 + 0x628) as *const Rcgcusb
    }

    #[doc="Get the *mut pointer for the RCGCUSB register."]
    #[inline] pub fn rcgcusb_mut(&self) -> *mut Rcgcusb { 
        (self.0 + 0x628) as *mut Rcgcusb
    }

    #[doc="Read the RCGCUSB register."]
    #[inline] pub fn rcgcusb(&self) -> Rcgcusb { 
        unsafe {
            read_volatile((self.0 + 0x628) as *const Rcgcusb)
        }
    }

    #[doc="Write the RCGCUSB register."]
    #[inline] pub fn set_rcgcusb<F: FnOnce(Rcgcusb) -> Rcgcusb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x628) as *mut Rcgcusb, f(Rcgcusb(0)));
        }
        self
    }

    #[doc="Modify the RCGCUSB register."]
    #[inline] pub fn with_rcgcusb<F: FnOnce(Rcgcusb) -> Rcgcusb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x628) as *mut Rcgcusb, f(self.rcgcusb()));
        }
        self
    }

    #[doc="Get the *const pointer for the RCGCEPHY register."]
    #[inline] pub fn rcgcephy_ptr(&self) -> *const Rcgcephy { 
        (self.0 + 0x630) as *const Rcgcephy
    }

    #[doc="Get the *mut pointer for the RCGCEPHY register."]
    #[inline] pub fn rcgcephy_mut(&self) -> *mut Rcgcephy { 
        (self.0 + 0x630) as *mut Rcgcephy
    }

    #[doc="Read the RCGCEPHY register."]
    #[inline] pub fn rcgcephy(&self) -> Rcgcephy { 
        unsafe {
            read_volatile((self.0 + 0x630) as *const Rcgcephy)
        }
    }

    #[doc="Write the RCGCEPHY register."]
    #[inline] pub fn set_rcgcephy<F: FnOnce(Rcgcephy) -> Rcgcephy>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x630) as *mut Rcgcephy, f(Rcgcephy(0)));
        }
        self
    }

    #[doc="Modify the RCGCEPHY register."]
    #[inline] pub fn with_rcgcephy<F: FnOnce(Rcgcephy) -> Rcgcephy>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x630) as *mut Rcgcephy, f(self.rcgcephy()));
        }
        self
    }

    #[doc="Get the *const pointer for the RCGCCAN register."]
    #[inline] pub fn rcgccan_ptr(&self) -> *const Rcgccan { 
        (self.0 + 0x634) as *const Rcgccan
    }

    #[doc="Get the *mut pointer for the RCGCCAN register."]
    #[inline] pub fn rcgccan_mut(&self) -> *mut Rcgccan { 
        (self.0 + 0x634) as *mut Rcgccan
    }

    #[doc="Read the RCGCCAN register."]
    #[inline] pub fn rcgccan(&self) -> Rcgccan { 
        unsafe {
            read_volatile((self.0 + 0x634) as *const Rcgccan)
        }
    }

    #[doc="Write the RCGCCAN register."]
    #[inline] pub fn set_rcgccan<F: FnOnce(Rcgccan) -> Rcgccan>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x634) as *mut Rcgccan, f(Rcgccan(0)));
        }
        self
    }

    #[doc="Modify the RCGCCAN register."]
    #[inline] pub fn with_rcgccan<F: FnOnce(Rcgccan) -> Rcgccan>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x634) as *mut Rcgccan, f(self.rcgccan()));
        }
        self
    }

    #[doc="Get the *const pointer for the RCGCADC register."]
    #[inline] pub fn rcgcadc_ptr(&self) -> *const Rcgcadc { 
        (self.0 + 0x638) as *const Rcgcadc
    }

    #[doc="Get the *mut pointer for the RCGCADC register."]
    #[inline] pub fn rcgcadc_mut(&self) -> *mut Rcgcadc { 
        (self.0 + 0x638) as *mut Rcgcadc
    }

    #[doc="Read the RCGCADC register."]
    #[inline] pub fn rcgcadc(&self) -> Rcgcadc { 
        unsafe {
            read_volatile((self.0 + 0x638) as *const Rcgcadc)
        }
    }

    #[doc="Write the RCGCADC register."]
    #[inline] pub fn set_rcgcadc<F: FnOnce(Rcgcadc) -> Rcgcadc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x638) as *mut Rcgcadc, f(Rcgcadc(0)));
        }
        self
    }

    #[doc="Modify the RCGCADC register."]
    #[inline] pub fn with_rcgcadc<F: FnOnce(Rcgcadc) -> Rcgcadc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x638) as *mut Rcgcadc, f(self.rcgcadc()));
        }
        self
    }

    #[doc="Get the *const pointer for the RCGCACMP register."]
    #[inline] pub fn rcgcacmp_ptr(&self) -> *const Rcgcacmp { 
        (self.0 + 0x63c) as *const Rcgcacmp
    }

    #[doc="Get the *mut pointer for the RCGCACMP register."]
    #[inline] pub fn rcgcacmp_mut(&self) -> *mut Rcgcacmp { 
        (self.0 + 0x63c) as *mut Rcgcacmp
    }

    #[doc="Read the RCGCACMP register."]
    #[inline] pub fn rcgcacmp(&self) -> Rcgcacmp { 
        unsafe {
            read_volatile((self.0 + 0x63c) as *const Rcgcacmp)
        }
    }

    #[doc="Write the RCGCACMP register."]
    #[inline] pub fn set_rcgcacmp<F: FnOnce(Rcgcacmp) -> Rcgcacmp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x63c) as *mut Rcgcacmp, f(Rcgcacmp(0)));
        }
        self
    }

    #[doc="Modify the RCGCACMP register."]
    #[inline] pub fn with_rcgcacmp<F: FnOnce(Rcgcacmp) -> Rcgcacmp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x63c) as *mut Rcgcacmp, f(self.rcgcacmp()));
        }
        self
    }

    #[doc="Get the *const pointer for the RCGCPWM register."]
    #[inline] pub fn rcgcpwm_ptr(&self) -> *const Rcgcpwm { 
        (self.0 + 0x640) as *const Rcgcpwm
    }

    #[doc="Get the *mut pointer for the RCGCPWM register."]
    #[inline] pub fn rcgcpwm_mut(&self) -> *mut Rcgcpwm { 
        (self.0 + 0x640) as *mut Rcgcpwm
    }

    #[doc="Read the RCGCPWM register."]
    #[inline] pub fn rcgcpwm(&self) -> Rcgcpwm { 
        unsafe {
            read_volatile((self.0 + 0x640) as *const Rcgcpwm)
        }
    }

    #[doc="Write the RCGCPWM register."]
    #[inline] pub fn set_rcgcpwm<F: FnOnce(Rcgcpwm) -> Rcgcpwm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x640) as *mut Rcgcpwm, f(Rcgcpwm(0)));
        }
        self
    }

    #[doc="Modify the RCGCPWM register."]
    #[inline] pub fn with_rcgcpwm<F: FnOnce(Rcgcpwm) -> Rcgcpwm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x640) as *mut Rcgcpwm, f(self.rcgcpwm()));
        }
        self
    }

    #[doc="Get the *const pointer for the RCGCQEI register."]
    #[inline] pub fn rcgcqei_ptr(&self) -> *const Rcgcqei { 
        (self.0 + 0x644) as *const Rcgcqei
    }

    #[doc="Get the *mut pointer for the RCGCQEI register."]
    #[inline] pub fn rcgcqei_mut(&self) -> *mut Rcgcqei { 
        (self.0 + 0x644) as *mut Rcgcqei
    }

    #[doc="Read the RCGCQEI register."]
    #[inline] pub fn rcgcqei(&self) -> Rcgcqei { 
        unsafe {
            read_volatile((self.0 + 0x644) as *const Rcgcqei)
        }
    }

    #[doc="Write the RCGCQEI register."]
    #[inline] pub fn set_rcgcqei<F: FnOnce(Rcgcqei) -> Rcgcqei>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x644) as *mut Rcgcqei, f(Rcgcqei(0)));
        }
        self
    }

    #[doc="Modify the RCGCQEI register."]
    #[inline] pub fn with_rcgcqei<F: FnOnce(Rcgcqei) -> Rcgcqei>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x644) as *mut Rcgcqei, f(self.rcgcqei()));
        }
        self
    }

    #[doc="Get the *const pointer for the RCGCEEPROM register."]
    #[inline] pub fn rcgceeprom_ptr(&self) -> *const Rcgceeprom { 
        (self.0 + 0x658) as *const Rcgceeprom
    }

    #[doc="Get the *mut pointer for the RCGCEEPROM register."]
    #[inline] pub fn rcgceeprom_mut(&self) -> *mut Rcgceeprom { 
        (self.0 + 0x658) as *mut Rcgceeprom
    }

    #[doc="Read the RCGCEEPROM register."]
    #[inline] pub fn rcgceeprom(&self) -> Rcgceeprom { 
        unsafe {
            read_volatile((self.0 + 0x658) as *const Rcgceeprom)
        }
    }

    #[doc="Write the RCGCEEPROM register."]
    #[inline] pub fn set_rcgceeprom<F: FnOnce(Rcgceeprom) -> Rcgceeprom>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x658) as *mut Rcgceeprom, f(Rcgceeprom(0)));
        }
        self
    }

    #[doc="Modify the RCGCEEPROM register."]
    #[inline] pub fn with_rcgceeprom<F: FnOnce(Rcgceeprom) -> Rcgceeprom>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x658) as *mut Rcgceeprom, f(self.rcgceeprom()));
        }
        self
    }

    #[doc="Get the *const pointer for the RCGCCCM register."]
    #[inline] pub fn rcgcccm_ptr(&self) -> *const Rcgcccm { 
        (self.0 + 0x674) as *const Rcgcccm
    }

    #[doc="Get the *mut pointer for the RCGCCCM register."]
    #[inline] pub fn rcgcccm_mut(&self) -> *mut Rcgcccm { 
        (self.0 + 0x674) as *mut Rcgcccm
    }

    #[doc="Read the RCGCCCM register."]
    #[inline] pub fn rcgcccm(&self) -> Rcgcccm { 
        unsafe {
            read_volatile((self.0 + 0x674) as *const Rcgcccm)
        }
    }

    #[doc="Write the RCGCCCM register."]
    #[inline] pub fn set_rcgcccm<F: FnOnce(Rcgcccm) -> Rcgcccm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x674) as *mut Rcgcccm, f(Rcgcccm(0)));
        }
        self
    }

    #[doc="Modify the RCGCCCM register."]
    #[inline] pub fn with_rcgcccm<F: FnOnce(Rcgcccm) -> Rcgcccm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x674) as *mut Rcgcccm, f(self.rcgcccm()));
        }
        self
    }

    #[doc="Get the *const pointer for the RCGCEMAC register."]
    #[inline] pub fn rcgcemac_ptr(&self) -> *const Rcgcemac { 
        (self.0 + 0x69c) as *const Rcgcemac
    }

    #[doc="Get the *mut pointer for the RCGCEMAC register."]
    #[inline] pub fn rcgcemac_mut(&self) -> *mut Rcgcemac { 
        (self.0 + 0x69c) as *mut Rcgcemac
    }

    #[doc="Read the RCGCEMAC register."]
    #[inline] pub fn rcgcemac(&self) -> Rcgcemac { 
        unsafe {
            read_volatile((self.0 + 0x69c) as *const Rcgcemac)
        }
    }

    #[doc="Write the RCGCEMAC register."]
    #[inline] pub fn set_rcgcemac<F: FnOnce(Rcgcemac) -> Rcgcemac>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x69c) as *mut Rcgcemac, f(Rcgcemac(0)));
        }
        self
    }

    #[doc="Modify the RCGCEMAC register."]
    #[inline] pub fn with_rcgcemac<F: FnOnce(Rcgcemac) -> Rcgcemac>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x69c) as *mut Rcgcemac, f(self.rcgcemac()));
        }
        self
    }

    #[doc="Get the *const pointer for the SCGCWD register."]
    #[inline] pub fn scgcwd_ptr(&self) -> *const Scgcwd { 
        (self.0 + 0x700) as *const Scgcwd
    }

    #[doc="Get the *mut pointer for the SCGCWD register."]
    #[inline] pub fn scgcwd_mut(&self) -> *mut Scgcwd { 
        (self.0 + 0x700) as *mut Scgcwd
    }

    #[doc="Read the SCGCWD register."]
    #[inline] pub fn scgcwd(&self) -> Scgcwd { 
        unsafe {
            read_volatile((self.0 + 0x700) as *const Scgcwd)
        }
    }

    #[doc="Write the SCGCWD register."]
    #[inline] pub fn set_scgcwd<F: FnOnce(Scgcwd) -> Scgcwd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x700) as *mut Scgcwd, f(Scgcwd(0)));
        }
        self
    }

    #[doc="Modify the SCGCWD register."]
    #[inline] pub fn with_scgcwd<F: FnOnce(Scgcwd) -> Scgcwd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x700) as *mut Scgcwd, f(self.scgcwd()));
        }
        self
    }

    #[doc="Get the *const pointer for the SCGCTIMER register."]
    #[inline] pub fn scgctimer_ptr(&self) -> *const Scgctimer { 
        (self.0 + 0x704) as *const Scgctimer
    }

    #[doc="Get the *mut pointer for the SCGCTIMER register."]
    #[inline] pub fn scgctimer_mut(&self) -> *mut Scgctimer { 
        (self.0 + 0x704) as *mut Scgctimer
    }

    #[doc="Read the SCGCTIMER register."]
    #[inline] pub fn scgctimer(&self) -> Scgctimer { 
        unsafe {
            read_volatile((self.0 + 0x704) as *const Scgctimer)
        }
    }

    #[doc="Write the SCGCTIMER register."]
    #[inline] pub fn set_scgctimer<F: FnOnce(Scgctimer) -> Scgctimer>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x704) as *mut Scgctimer, f(Scgctimer(0)));
        }
        self
    }

    #[doc="Modify the SCGCTIMER register."]
    #[inline] pub fn with_scgctimer<F: FnOnce(Scgctimer) -> Scgctimer>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x704) as *mut Scgctimer, f(self.scgctimer()));
        }
        self
    }

    #[doc="Get the *const pointer for the SCGCGPIO register."]
    #[inline] pub fn scgcgpio_ptr(&self) -> *const Scgcgpio { 
        (self.0 + 0x708) as *const Scgcgpio
    }

    #[doc="Get the *mut pointer for the SCGCGPIO register."]
    #[inline] pub fn scgcgpio_mut(&self) -> *mut Scgcgpio { 
        (self.0 + 0x708) as *mut Scgcgpio
    }

    #[doc="Read the SCGCGPIO register."]
    #[inline] pub fn scgcgpio(&self) -> Scgcgpio { 
        unsafe {
            read_volatile((self.0 + 0x708) as *const Scgcgpio)
        }
    }

    #[doc="Write the SCGCGPIO register."]
    #[inline] pub fn set_scgcgpio<F: FnOnce(Scgcgpio) -> Scgcgpio>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x708) as *mut Scgcgpio, f(Scgcgpio(0)));
        }
        self
    }

    #[doc="Modify the SCGCGPIO register."]
    #[inline] pub fn with_scgcgpio<F: FnOnce(Scgcgpio) -> Scgcgpio>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x708) as *mut Scgcgpio, f(self.scgcgpio()));
        }
        self
    }

    #[doc="Get the *const pointer for the SCGCDMA register."]
    #[inline] pub fn scgcdma_ptr(&self) -> *const Scgcdma { 
        (self.0 + 0x70c) as *const Scgcdma
    }

    #[doc="Get the *mut pointer for the SCGCDMA register."]
    #[inline] pub fn scgcdma_mut(&self) -> *mut Scgcdma { 
        (self.0 + 0x70c) as *mut Scgcdma
    }

    #[doc="Read the SCGCDMA register."]
    #[inline] pub fn scgcdma(&self) -> Scgcdma { 
        unsafe {
            read_volatile((self.0 + 0x70c) as *const Scgcdma)
        }
    }

    #[doc="Write the SCGCDMA register."]
    #[inline] pub fn set_scgcdma<F: FnOnce(Scgcdma) -> Scgcdma>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x70c) as *mut Scgcdma, f(Scgcdma(0)));
        }
        self
    }

    #[doc="Modify the SCGCDMA register."]
    #[inline] pub fn with_scgcdma<F: FnOnce(Scgcdma) -> Scgcdma>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x70c) as *mut Scgcdma, f(self.scgcdma()));
        }
        self
    }

    #[doc="Get the *const pointer for the SCGCEPI register."]
    #[inline] pub fn scgcepi_ptr(&self) -> *const Scgcepi { 
        (self.0 + 0x710) as *const Scgcepi
    }

    #[doc="Get the *mut pointer for the SCGCEPI register."]
    #[inline] pub fn scgcepi_mut(&self) -> *mut Scgcepi { 
        (self.0 + 0x710) as *mut Scgcepi
    }

    #[doc="Read the SCGCEPI register."]
    #[inline] pub fn scgcepi(&self) -> Scgcepi { 
        unsafe {
            read_volatile((self.0 + 0x710) as *const Scgcepi)
        }
    }

    #[doc="Write the SCGCEPI register."]
    #[inline] pub fn set_scgcepi<F: FnOnce(Scgcepi) -> Scgcepi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x710) as *mut Scgcepi, f(Scgcepi(0)));
        }
        self
    }

    #[doc="Modify the SCGCEPI register."]
    #[inline] pub fn with_scgcepi<F: FnOnce(Scgcepi) -> Scgcepi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x710) as *mut Scgcepi, f(self.scgcepi()));
        }
        self
    }

    #[doc="Get the *const pointer for the SCGCHIB register."]
    #[inline] pub fn scgchib_ptr(&self) -> *const Scgchib { 
        (self.0 + 0x714) as *const Scgchib
    }

    #[doc="Get the *mut pointer for the SCGCHIB register."]
    #[inline] pub fn scgchib_mut(&self) -> *mut Scgchib { 
        (self.0 + 0x714) as *mut Scgchib
    }

    #[doc="Read the SCGCHIB register."]
    #[inline] pub fn scgchib(&self) -> Scgchib { 
        unsafe {
            read_volatile((self.0 + 0x714) as *const Scgchib)
        }
    }

    #[doc="Write the SCGCHIB register."]
    #[inline] pub fn set_scgchib<F: FnOnce(Scgchib) -> Scgchib>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x714) as *mut Scgchib, f(Scgchib(0)));
        }
        self
    }

    #[doc="Modify the SCGCHIB register."]
    #[inline] pub fn with_scgchib<F: FnOnce(Scgchib) -> Scgchib>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x714) as *mut Scgchib, f(self.scgchib()));
        }
        self
    }

    #[doc="Get the *const pointer for the SCGCUART register."]
    #[inline] pub fn scgcuart_ptr(&self) -> *const Scgcuart { 
        (self.0 + 0x718) as *const Scgcuart
    }

    #[doc="Get the *mut pointer for the SCGCUART register."]
    #[inline] pub fn scgcuart_mut(&self) -> *mut Scgcuart { 
        (self.0 + 0x718) as *mut Scgcuart
    }

    #[doc="Read the SCGCUART register."]
    #[inline] pub fn scgcuart(&self) -> Scgcuart { 
        unsafe {
            read_volatile((self.0 + 0x718) as *const Scgcuart)
        }
    }

    #[doc="Write the SCGCUART register."]
    #[inline] pub fn set_scgcuart<F: FnOnce(Scgcuart) -> Scgcuart>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x718) as *mut Scgcuart, f(Scgcuart(0)));
        }
        self
    }

    #[doc="Modify the SCGCUART register."]
    #[inline] pub fn with_scgcuart<F: FnOnce(Scgcuart) -> Scgcuart>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x718) as *mut Scgcuart, f(self.scgcuart()));
        }
        self
    }

    #[doc="Get the *const pointer for the SCGCSSI register."]
    #[inline] pub fn scgcssi_ptr(&self) -> *const Scgcssi { 
        (self.0 + 0x71c) as *const Scgcssi
    }

    #[doc="Get the *mut pointer for the SCGCSSI register."]
    #[inline] pub fn scgcssi_mut(&self) -> *mut Scgcssi { 
        (self.0 + 0x71c) as *mut Scgcssi
    }

    #[doc="Read the SCGCSSI register."]
    #[inline] pub fn scgcssi(&self) -> Scgcssi { 
        unsafe {
            read_volatile((self.0 + 0x71c) as *const Scgcssi)
        }
    }

    #[doc="Write the SCGCSSI register."]
    #[inline] pub fn set_scgcssi<F: FnOnce(Scgcssi) -> Scgcssi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x71c) as *mut Scgcssi, f(Scgcssi(0)));
        }
        self
    }

    #[doc="Modify the SCGCSSI register."]
    #[inline] pub fn with_scgcssi<F: FnOnce(Scgcssi) -> Scgcssi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x71c) as *mut Scgcssi, f(self.scgcssi()));
        }
        self
    }

    #[doc="Get the *const pointer for the SCGCI2C register."]
    #[inline] pub fn scgci2c_ptr(&self) -> *const Scgci2c { 
        (self.0 + 0x720) as *const Scgci2c
    }

    #[doc="Get the *mut pointer for the SCGCI2C register."]
    #[inline] pub fn scgci2c_mut(&self) -> *mut Scgci2c { 
        (self.0 + 0x720) as *mut Scgci2c
    }

    #[doc="Read the SCGCI2C register."]
    #[inline] pub fn scgci2c(&self) -> Scgci2c { 
        unsafe {
            read_volatile((self.0 + 0x720) as *const Scgci2c)
        }
    }

    #[doc="Write the SCGCI2C register."]
    #[inline] pub fn set_scgci2c<F: FnOnce(Scgci2c) -> Scgci2c>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x720) as *mut Scgci2c, f(Scgci2c(0)));
        }
        self
    }

    #[doc="Modify the SCGCI2C register."]
    #[inline] pub fn with_scgci2c<F: FnOnce(Scgci2c) -> Scgci2c>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x720) as *mut Scgci2c, f(self.scgci2c()));
        }
        self
    }

    #[doc="Get the *const pointer for the SCGCUSB register."]
    #[inline] pub fn scgcusb_ptr(&self) -> *const Scgcusb { 
        (self.0 + 0x728) as *const Scgcusb
    }

    #[doc="Get the *mut pointer for the SCGCUSB register."]
    #[inline] pub fn scgcusb_mut(&self) -> *mut Scgcusb { 
        (self.0 + 0x728) as *mut Scgcusb
    }

    #[doc="Read the SCGCUSB register."]
    #[inline] pub fn scgcusb(&self) -> Scgcusb { 
        unsafe {
            read_volatile((self.0 + 0x728) as *const Scgcusb)
        }
    }

    #[doc="Write the SCGCUSB register."]
    #[inline] pub fn set_scgcusb<F: FnOnce(Scgcusb) -> Scgcusb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x728) as *mut Scgcusb, f(Scgcusb(0)));
        }
        self
    }

    #[doc="Modify the SCGCUSB register."]
    #[inline] pub fn with_scgcusb<F: FnOnce(Scgcusb) -> Scgcusb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x728) as *mut Scgcusb, f(self.scgcusb()));
        }
        self
    }

    #[doc="Get the *const pointer for the SCGCEPHY register."]
    #[inline] pub fn scgcephy_ptr(&self) -> *const Scgcephy { 
        (self.0 + 0x730) as *const Scgcephy
    }

    #[doc="Get the *mut pointer for the SCGCEPHY register."]
    #[inline] pub fn scgcephy_mut(&self) -> *mut Scgcephy { 
        (self.0 + 0x730) as *mut Scgcephy
    }

    #[doc="Read the SCGCEPHY register."]
    #[inline] pub fn scgcephy(&self) -> Scgcephy { 
        unsafe {
            read_volatile((self.0 + 0x730) as *const Scgcephy)
        }
    }

    #[doc="Write the SCGCEPHY register."]
    #[inline] pub fn set_scgcephy<F: FnOnce(Scgcephy) -> Scgcephy>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x730) as *mut Scgcephy, f(Scgcephy(0)));
        }
        self
    }

    #[doc="Modify the SCGCEPHY register."]
    #[inline] pub fn with_scgcephy<F: FnOnce(Scgcephy) -> Scgcephy>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x730) as *mut Scgcephy, f(self.scgcephy()));
        }
        self
    }

    #[doc="Get the *const pointer for the SCGCCAN register."]
    #[inline] pub fn scgccan_ptr(&self) -> *const Scgccan { 
        (self.0 + 0x734) as *const Scgccan
    }

    #[doc="Get the *mut pointer for the SCGCCAN register."]
    #[inline] pub fn scgccan_mut(&self) -> *mut Scgccan { 
        (self.0 + 0x734) as *mut Scgccan
    }

    #[doc="Read the SCGCCAN register."]
    #[inline] pub fn scgccan(&self) -> Scgccan { 
        unsafe {
            read_volatile((self.0 + 0x734) as *const Scgccan)
        }
    }

    #[doc="Write the SCGCCAN register."]
    #[inline] pub fn set_scgccan<F: FnOnce(Scgccan) -> Scgccan>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x734) as *mut Scgccan, f(Scgccan(0)));
        }
        self
    }

    #[doc="Modify the SCGCCAN register."]
    #[inline] pub fn with_scgccan<F: FnOnce(Scgccan) -> Scgccan>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x734) as *mut Scgccan, f(self.scgccan()));
        }
        self
    }

    #[doc="Get the *const pointer for the SCGCADC register."]
    #[inline] pub fn scgcadc_ptr(&self) -> *const Scgcadc { 
        (self.0 + 0x738) as *const Scgcadc
    }

    #[doc="Get the *mut pointer for the SCGCADC register."]
    #[inline] pub fn scgcadc_mut(&self) -> *mut Scgcadc { 
        (self.0 + 0x738) as *mut Scgcadc
    }

    #[doc="Read the SCGCADC register."]
    #[inline] pub fn scgcadc(&self) -> Scgcadc { 
        unsafe {
            read_volatile((self.0 + 0x738) as *const Scgcadc)
        }
    }

    #[doc="Write the SCGCADC register."]
    #[inline] pub fn set_scgcadc<F: FnOnce(Scgcadc) -> Scgcadc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x738) as *mut Scgcadc, f(Scgcadc(0)));
        }
        self
    }

    #[doc="Modify the SCGCADC register."]
    #[inline] pub fn with_scgcadc<F: FnOnce(Scgcadc) -> Scgcadc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x738) as *mut Scgcadc, f(self.scgcadc()));
        }
        self
    }

    #[doc="Get the *const pointer for the SCGCACMP register."]
    #[inline] pub fn scgcacmp_ptr(&self) -> *const Scgcacmp { 
        (self.0 + 0x73c) as *const Scgcacmp
    }

    #[doc="Get the *mut pointer for the SCGCACMP register."]
    #[inline] pub fn scgcacmp_mut(&self) -> *mut Scgcacmp { 
        (self.0 + 0x73c) as *mut Scgcacmp
    }

    #[doc="Read the SCGCACMP register."]
    #[inline] pub fn scgcacmp(&self) -> Scgcacmp { 
        unsafe {
            read_volatile((self.0 + 0x73c) as *const Scgcacmp)
        }
    }

    #[doc="Write the SCGCACMP register."]
    #[inline] pub fn set_scgcacmp<F: FnOnce(Scgcacmp) -> Scgcacmp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x73c) as *mut Scgcacmp, f(Scgcacmp(0)));
        }
        self
    }

    #[doc="Modify the SCGCACMP register."]
    #[inline] pub fn with_scgcacmp<F: FnOnce(Scgcacmp) -> Scgcacmp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x73c) as *mut Scgcacmp, f(self.scgcacmp()));
        }
        self
    }

    #[doc="Get the *const pointer for the SCGCPWM register."]
    #[inline] pub fn scgcpwm_ptr(&self) -> *const Scgcpwm { 
        (self.0 + 0x740) as *const Scgcpwm
    }

    #[doc="Get the *mut pointer for the SCGCPWM register."]
    #[inline] pub fn scgcpwm_mut(&self) -> *mut Scgcpwm { 
        (self.0 + 0x740) as *mut Scgcpwm
    }

    #[doc="Read the SCGCPWM register."]
    #[inline] pub fn scgcpwm(&self) -> Scgcpwm { 
        unsafe {
            read_volatile((self.0 + 0x740) as *const Scgcpwm)
        }
    }

    #[doc="Write the SCGCPWM register."]
    #[inline] pub fn set_scgcpwm<F: FnOnce(Scgcpwm) -> Scgcpwm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x740) as *mut Scgcpwm, f(Scgcpwm(0)));
        }
        self
    }

    #[doc="Modify the SCGCPWM register."]
    #[inline] pub fn with_scgcpwm<F: FnOnce(Scgcpwm) -> Scgcpwm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x740) as *mut Scgcpwm, f(self.scgcpwm()));
        }
        self
    }

    #[doc="Get the *const pointer for the SCGCQEI register."]
    #[inline] pub fn scgcqei_ptr(&self) -> *const Scgcqei { 
        (self.0 + 0x744) as *const Scgcqei
    }

    #[doc="Get the *mut pointer for the SCGCQEI register."]
    #[inline] pub fn scgcqei_mut(&self) -> *mut Scgcqei { 
        (self.0 + 0x744) as *mut Scgcqei
    }

    #[doc="Read the SCGCQEI register."]
    #[inline] pub fn scgcqei(&self) -> Scgcqei { 
        unsafe {
            read_volatile((self.0 + 0x744) as *const Scgcqei)
        }
    }

    #[doc="Write the SCGCQEI register."]
    #[inline] pub fn set_scgcqei<F: FnOnce(Scgcqei) -> Scgcqei>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x744) as *mut Scgcqei, f(Scgcqei(0)));
        }
        self
    }

    #[doc="Modify the SCGCQEI register."]
    #[inline] pub fn with_scgcqei<F: FnOnce(Scgcqei) -> Scgcqei>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x744) as *mut Scgcqei, f(self.scgcqei()));
        }
        self
    }

    #[doc="Get the *const pointer for the SCGCEEPROM register."]
    #[inline] pub fn scgceeprom_ptr(&self) -> *const Scgceeprom { 
        (self.0 + 0x758) as *const Scgceeprom
    }

    #[doc="Get the *mut pointer for the SCGCEEPROM register."]
    #[inline] pub fn scgceeprom_mut(&self) -> *mut Scgceeprom { 
        (self.0 + 0x758) as *mut Scgceeprom
    }

    #[doc="Read the SCGCEEPROM register."]
    #[inline] pub fn scgceeprom(&self) -> Scgceeprom { 
        unsafe {
            read_volatile((self.0 + 0x758) as *const Scgceeprom)
        }
    }

    #[doc="Write the SCGCEEPROM register."]
    #[inline] pub fn set_scgceeprom<F: FnOnce(Scgceeprom) -> Scgceeprom>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x758) as *mut Scgceeprom, f(Scgceeprom(0)));
        }
        self
    }

    #[doc="Modify the SCGCEEPROM register."]
    #[inline] pub fn with_scgceeprom<F: FnOnce(Scgceeprom) -> Scgceeprom>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x758) as *mut Scgceeprom, f(self.scgceeprom()));
        }
        self
    }

    #[doc="Get the *const pointer for the SCGCCCM register."]
    #[inline] pub fn scgcccm_ptr(&self) -> *const Scgcccm { 
        (self.0 + 0x774) as *const Scgcccm
    }

    #[doc="Get the *mut pointer for the SCGCCCM register."]
    #[inline] pub fn scgcccm_mut(&self) -> *mut Scgcccm { 
        (self.0 + 0x774) as *mut Scgcccm
    }

    #[doc="Read the SCGCCCM register."]
    #[inline] pub fn scgcccm(&self) -> Scgcccm { 
        unsafe {
            read_volatile((self.0 + 0x774) as *const Scgcccm)
        }
    }

    #[doc="Write the SCGCCCM register."]
    #[inline] pub fn set_scgcccm<F: FnOnce(Scgcccm) -> Scgcccm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x774) as *mut Scgcccm, f(Scgcccm(0)));
        }
        self
    }

    #[doc="Modify the SCGCCCM register."]
    #[inline] pub fn with_scgcccm<F: FnOnce(Scgcccm) -> Scgcccm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x774) as *mut Scgcccm, f(self.scgcccm()));
        }
        self
    }

    #[doc="Get the *const pointer for the SCGCEMAC register."]
    #[inline] pub fn scgcemac_ptr(&self) -> *const Scgcemac { 
        (self.0 + 0x79c) as *const Scgcemac
    }

    #[doc="Get the *mut pointer for the SCGCEMAC register."]
    #[inline] pub fn scgcemac_mut(&self) -> *mut Scgcemac { 
        (self.0 + 0x79c) as *mut Scgcemac
    }

    #[doc="Read the SCGCEMAC register."]
    #[inline] pub fn scgcemac(&self) -> Scgcemac { 
        unsafe {
            read_volatile((self.0 + 0x79c) as *const Scgcemac)
        }
    }

    #[doc="Write the SCGCEMAC register."]
    #[inline] pub fn set_scgcemac<F: FnOnce(Scgcemac) -> Scgcemac>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x79c) as *mut Scgcemac, f(Scgcemac(0)));
        }
        self
    }

    #[doc="Modify the SCGCEMAC register."]
    #[inline] pub fn with_scgcemac<F: FnOnce(Scgcemac) -> Scgcemac>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x79c) as *mut Scgcemac, f(self.scgcemac()));
        }
        self
    }

    #[doc="Get the *const pointer for the DCGCWD register."]
    #[inline] pub fn dcgcwd_ptr(&self) -> *const Dcgcwd { 
        (self.0 + 0x800) as *const Dcgcwd
    }

    #[doc="Get the *mut pointer for the DCGCWD register."]
    #[inline] pub fn dcgcwd_mut(&self) -> *mut Dcgcwd { 
        (self.0 + 0x800) as *mut Dcgcwd
    }

    #[doc="Read the DCGCWD register."]
    #[inline] pub fn dcgcwd(&self) -> Dcgcwd { 
        unsafe {
            read_volatile((self.0 + 0x800) as *const Dcgcwd)
        }
    }

    #[doc="Write the DCGCWD register."]
    #[inline] pub fn set_dcgcwd<F: FnOnce(Dcgcwd) -> Dcgcwd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x800) as *mut Dcgcwd, f(Dcgcwd(0)));
        }
        self
    }

    #[doc="Modify the DCGCWD register."]
    #[inline] pub fn with_dcgcwd<F: FnOnce(Dcgcwd) -> Dcgcwd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x800) as *mut Dcgcwd, f(self.dcgcwd()));
        }
        self
    }

    #[doc="Get the *const pointer for the DCGCTIMER register."]
    #[inline] pub fn dcgctimer_ptr(&self) -> *const Dcgctimer { 
        (self.0 + 0x804) as *const Dcgctimer
    }

    #[doc="Get the *mut pointer for the DCGCTIMER register."]
    #[inline] pub fn dcgctimer_mut(&self) -> *mut Dcgctimer { 
        (self.0 + 0x804) as *mut Dcgctimer
    }

    #[doc="Read the DCGCTIMER register."]
    #[inline] pub fn dcgctimer(&self) -> Dcgctimer { 
        unsafe {
            read_volatile((self.0 + 0x804) as *const Dcgctimer)
        }
    }

    #[doc="Write the DCGCTIMER register."]
    #[inline] pub fn set_dcgctimer<F: FnOnce(Dcgctimer) -> Dcgctimer>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x804) as *mut Dcgctimer, f(Dcgctimer(0)));
        }
        self
    }

    #[doc="Modify the DCGCTIMER register."]
    #[inline] pub fn with_dcgctimer<F: FnOnce(Dcgctimer) -> Dcgctimer>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x804) as *mut Dcgctimer, f(self.dcgctimer()));
        }
        self
    }

    #[doc="Get the *const pointer for the DCGCGPIO register."]
    #[inline] pub fn dcgcgpio_ptr(&self) -> *const Dcgcgpio { 
        (self.0 + 0x808) as *const Dcgcgpio
    }

    #[doc="Get the *mut pointer for the DCGCGPIO register."]
    #[inline] pub fn dcgcgpio_mut(&self) -> *mut Dcgcgpio { 
        (self.0 + 0x808) as *mut Dcgcgpio
    }

    #[doc="Read the DCGCGPIO register."]
    #[inline] pub fn dcgcgpio(&self) -> Dcgcgpio { 
        unsafe {
            read_volatile((self.0 + 0x808) as *const Dcgcgpio)
        }
    }

    #[doc="Write the DCGCGPIO register."]
    #[inline] pub fn set_dcgcgpio<F: FnOnce(Dcgcgpio) -> Dcgcgpio>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x808) as *mut Dcgcgpio, f(Dcgcgpio(0)));
        }
        self
    }

    #[doc="Modify the DCGCGPIO register."]
    #[inline] pub fn with_dcgcgpio<F: FnOnce(Dcgcgpio) -> Dcgcgpio>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x808) as *mut Dcgcgpio, f(self.dcgcgpio()));
        }
        self
    }

    #[doc="Get the *const pointer for the DCGCDMA register."]
    #[inline] pub fn dcgcdma_ptr(&self) -> *const Dcgcdma { 
        (self.0 + 0x80c) as *const Dcgcdma
    }

    #[doc="Get the *mut pointer for the DCGCDMA register."]
    #[inline] pub fn dcgcdma_mut(&self) -> *mut Dcgcdma { 
        (self.0 + 0x80c) as *mut Dcgcdma
    }

    #[doc="Read the DCGCDMA register."]
    #[inline] pub fn dcgcdma(&self) -> Dcgcdma { 
        unsafe {
            read_volatile((self.0 + 0x80c) as *const Dcgcdma)
        }
    }

    #[doc="Write the DCGCDMA register."]
    #[inline] pub fn set_dcgcdma<F: FnOnce(Dcgcdma) -> Dcgcdma>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x80c) as *mut Dcgcdma, f(Dcgcdma(0)));
        }
        self
    }

    #[doc="Modify the DCGCDMA register."]
    #[inline] pub fn with_dcgcdma<F: FnOnce(Dcgcdma) -> Dcgcdma>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x80c) as *mut Dcgcdma, f(self.dcgcdma()));
        }
        self
    }

    #[doc="Get the *const pointer for the DCGCEPI register."]
    #[inline] pub fn dcgcepi_ptr(&self) -> *const Dcgcepi { 
        (self.0 + 0x810) as *const Dcgcepi
    }

    #[doc="Get the *mut pointer for the DCGCEPI register."]
    #[inline] pub fn dcgcepi_mut(&self) -> *mut Dcgcepi { 
        (self.0 + 0x810) as *mut Dcgcepi
    }

    #[doc="Read the DCGCEPI register."]
    #[inline] pub fn dcgcepi(&self) -> Dcgcepi { 
        unsafe {
            read_volatile((self.0 + 0x810) as *const Dcgcepi)
        }
    }

    #[doc="Write the DCGCEPI register."]
    #[inline] pub fn set_dcgcepi<F: FnOnce(Dcgcepi) -> Dcgcepi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x810) as *mut Dcgcepi, f(Dcgcepi(0)));
        }
        self
    }

    #[doc="Modify the DCGCEPI register."]
    #[inline] pub fn with_dcgcepi<F: FnOnce(Dcgcepi) -> Dcgcepi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x810) as *mut Dcgcepi, f(self.dcgcepi()));
        }
        self
    }

    #[doc="Get the *const pointer for the DCGCHIB register."]
    #[inline] pub fn dcgchib_ptr(&self) -> *const Dcgchib { 
        (self.0 + 0x814) as *const Dcgchib
    }

    #[doc="Get the *mut pointer for the DCGCHIB register."]
    #[inline] pub fn dcgchib_mut(&self) -> *mut Dcgchib { 
        (self.0 + 0x814) as *mut Dcgchib
    }

    #[doc="Read the DCGCHIB register."]
    #[inline] pub fn dcgchib(&self) -> Dcgchib { 
        unsafe {
            read_volatile((self.0 + 0x814) as *const Dcgchib)
        }
    }

    #[doc="Write the DCGCHIB register."]
    #[inline] pub fn set_dcgchib<F: FnOnce(Dcgchib) -> Dcgchib>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x814) as *mut Dcgchib, f(Dcgchib(0)));
        }
        self
    }

    #[doc="Modify the DCGCHIB register."]
    #[inline] pub fn with_dcgchib<F: FnOnce(Dcgchib) -> Dcgchib>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x814) as *mut Dcgchib, f(self.dcgchib()));
        }
        self
    }

    #[doc="Get the *const pointer for the DCGCUART register."]
    #[inline] pub fn dcgcuart_ptr(&self) -> *const Dcgcuart { 
        (self.0 + 0x818) as *const Dcgcuart
    }

    #[doc="Get the *mut pointer for the DCGCUART register."]
    #[inline] pub fn dcgcuart_mut(&self) -> *mut Dcgcuart { 
        (self.0 + 0x818) as *mut Dcgcuart
    }

    #[doc="Read the DCGCUART register."]
    #[inline] pub fn dcgcuart(&self) -> Dcgcuart { 
        unsafe {
            read_volatile((self.0 + 0x818) as *const Dcgcuart)
        }
    }

    #[doc="Write the DCGCUART register."]
    #[inline] pub fn set_dcgcuart<F: FnOnce(Dcgcuart) -> Dcgcuart>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x818) as *mut Dcgcuart, f(Dcgcuart(0)));
        }
        self
    }

    #[doc="Modify the DCGCUART register."]
    #[inline] pub fn with_dcgcuart<F: FnOnce(Dcgcuart) -> Dcgcuart>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x818) as *mut Dcgcuart, f(self.dcgcuart()));
        }
        self
    }

    #[doc="Get the *const pointer for the DCGCSSI register."]
    #[inline] pub fn dcgcssi_ptr(&self) -> *const Dcgcssi { 
        (self.0 + 0x81c) as *const Dcgcssi
    }

    #[doc="Get the *mut pointer for the DCGCSSI register."]
    #[inline] pub fn dcgcssi_mut(&self) -> *mut Dcgcssi { 
        (self.0 + 0x81c) as *mut Dcgcssi
    }

    #[doc="Read the DCGCSSI register."]
    #[inline] pub fn dcgcssi(&self) -> Dcgcssi { 
        unsafe {
            read_volatile((self.0 + 0x81c) as *const Dcgcssi)
        }
    }

    #[doc="Write the DCGCSSI register."]
    #[inline] pub fn set_dcgcssi<F: FnOnce(Dcgcssi) -> Dcgcssi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x81c) as *mut Dcgcssi, f(Dcgcssi(0)));
        }
        self
    }

    #[doc="Modify the DCGCSSI register."]
    #[inline] pub fn with_dcgcssi<F: FnOnce(Dcgcssi) -> Dcgcssi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x81c) as *mut Dcgcssi, f(self.dcgcssi()));
        }
        self
    }

    #[doc="Get the *const pointer for the DCGCI2C register."]
    #[inline] pub fn dcgci2c_ptr(&self) -> *const Dcgci2c { 
        (self.0 + 0x820) as *const Dcgci2c
    }

    #[doc="Get the *mut pointer for the DCGCI2C register."]
    #[inline] pub fn dcgci2c_mut(&self) -> *mut Dcgci2c { 
        (self.0 + 0x820) as *mut Dcgci2c
    }

    #[doc="Read the DCGCI2C register."]
    #[inline] pub fn dcgci2c(&self) -> Dcgci2c { 
        unsafe {
            read_volatile((self.0 + 0x820) as *const Dcgci2c)
        }
    }

    #[doc="Write the DCGCI2C register."]
    #[inline] pub fn set_dcgci2c<F: FnOnce(Dcgci2c) -> Dcgci2c>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x820) as *mut Dcgci2c, f(Dcgci2c(0)));
        }
        self
    }

    #[doc="Modify the DCGCI2C register."]
    #[inline] pub fn with_dcgci2c<F: FnOnce(Dcgci2c) -> Dcgci2c>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x820) as *mut Dcgci2c, f(self.dcgci2c()));
        }
        self
    }

    #[doc="Get the *const pointer for the DCGCUSB register."]
    #[inline] pub fn dcgcusb_ptr(&self) -> *const Dcgcusb { 
        (self.0 + 0x828) as *const Dcgcusb
    }

    #[doc="Get the *mut pointer for the DCGCUSB register."]
    #[inline] pub fn dcgcusb_mut(&self) -> *mut Dcgcusb { 
        (self.0 + 0x828) as *mut Dcgcusb
    }

    #[doc="Read the DCGCUSB register."]
    #[inline] pub fn dcgcusb(&self) -> Dcgcusb { 
        unsafe {
            read_volatile((self.0 + 0x828) as *const Dcgcusb)
        }
    }

    #[doc="Write the DCGCUSB register."]
    #[inline] pub fn set_dcgcusb<F: FnOnce(Dcgcusb) -> Dcgcusb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x828) as *mut Dcgcusb, f(Dcgcusb(0)));
        }
        self
    }

    #[doc="Modify the DCGCUSB register."]
    #[inline] pub fn with_dcgcusb<F: FnOnce(Dcgcusb) -> Dcgcusb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x828) as *mut Dcgcusb, f(self.dcgcusb()));
        }
        self
    }

    #[doc="Get the *const pointer for the DCGCEPHY register."]
    #[inline] pub fn dcgcephy_ptr(&self) -> *const Dcgcephy { 
        (self.0 + 0x830) as *const Dcgcephy
    }

    #[doc="Get the *mut pointer for the DCGCEPHY register."]
    #[inline] pub fn dcgcephy_mut(&self) -> *mut Dcgcephy { 
        (self.0 + 0x830) as *mut Dcgcephy
    }

    #[doc="Read the DCGCEPHY register."]
    #[inline] pub fn dcgcephy(&self) -> Dcgcephy { 
        unsafe {
            read_volatile((self.0 + 0x830) as *const Dcgcephy)
        }
    }

    #[doc="Write the DCGCEPHY register."]
    #[inline] pub fn set_dcgcephy<F: FnOnce(Dcgcephy) -> Dcgcephy>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x830) as *mut Dcgcephy, f(Dcgcephy(0)));
        }
        self
    }

    #[doc="Modify the DCGCEPHY register."]
    #[inline] pub fn with_dcgcephy<F: FnOnce(Dcgcephy) -> Dcgcephy>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x830) as *mut Dcgcephy, f(self.dcgcephy()));
        }
        self
    }

    #[doc="Get the *const pointer for the DCGCCAN register."]
    #[inline] pub fn dcgccan_ptr(&self) -> *const Dcgccan { 
        (self.0 + 0x834) as *const Dcgccan
    }

    #[doc="Get the *mut pointer for the DCGCCAN register."]
    #[inline] pub fn dcgccan_mut(&self) -> *mut Dcgccan { 
        (self.0 + 0x834) as *mut Dcgccan
    }

    #[doc="Read the DCGCCAN register."]
    #[inline] pub fn dcgccan(&self) -> Dcgccan { 
        unsafe {
            read_volatile((self.0 + 0x834) as *const Dcgccan)
        }
    }

    #[doc="Write the DCGCCAN register."]
    #[inline] pub fn set_dcgccan<F: FnOnce(Dcgccan) -> Dcgccan>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x834) as *mut Dcgccan, f(Dcgccan(0)));
        }
        self
    }

    #[doc="Modify the DCGCCAN register."]
    #[inline] pub fn with_dcgccan<F: FnOnce(Dcgccan) -> Dcgccan>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x834) as *mut Dcgccan, f(self.dcgccan()));
        }
        self
    }

    #[doc="Get the *const pointer for the DCGCADC register."]
    #[inline] pub fn dcgcadc_ptr(&self) -> *const Dcgcadc { 
        (self.0 + 0x838) as *const Dcgcadc
    }

    #[doc="Get the *mut pointer for the DCGCADC register."]
    #[inline] pub fn dcgcadc_mut(&self) -> *mut Dcgcadc { 
        (self.0 + 0x838) as *mut Dcgcadc
    }

    #[doc="Read the DCGCADC register."]
    #[inline] pub fn dcgcadc(&self) -> Dcgcadc { 
        unsafe {
            read_volatile((self.0 + 0x838) as *const Dcgcadc)
        }
    }

    #[doc="Write the DCGCADC register."]
    #[inline] pub fn set_dcgcadc<F: FnOnce(Dcgcadc) -> Dcgcadc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x838) as *mut Dcgcadc, f(Dcgcadc(0)));
        }
        self
    }

    #[doc="Modify the DCGCADC register."]
    #[inline] pub fn with_dcgcadc<F: FnOnce(Dcgcadc) -> Dcgcadc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x838) as *mut Dcgcadc, f(self.dcgcadc()));
        }
        self
    }

    #[doc="Get the *const pointer for the DCGCACMP register."]
    #[inline] pub fn dcgcacmp_ptr(&self) -> *const Dcgcacmp { 
        (self.0 + 0x83c) as *const Dcgcacmp
    }

    #[doc="Get the *mut pointer for the DCGCACMP register."]
    #[inline] pub fn dcgcacmp_mut(&self) -> *mut Dcgcacmp { 
        (self.0 + 0x83c) as *mut Dcgcacmp
    }

    #[doc="Read the DCGCACMP register."]
    #[inline] pub fn dcgcacmp(&self) -> Dcgcacmp { 
        unsafe {
            read_volatile((self.0 + 0x83c) as *const Dcgcacmp)
        }
    }

    #[doc="Write the DCGCACMP register."]
    #[inline] pub fn set_dcgcacmp<F: FnOnce(Dcgcacmp) -> Dcgcacmp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x83c) as *mut Dcgcacmp, f(Dcgcacmp(0)));
        }
        self
    }

    #[doc="Modify the DCGCACMP register."]
    #[inline] pub fn with_dcgcacmp<F: FnOnce(Dcgcacmp) -> Dcgcacmp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x83c) as *mut Dcgcacmp, f(self.dcgcacmp()));
        }
        self
    }

    #[doc="Get the *const pointer for the DCGCPWM register."]
    #[inline] pub fn dcgcpwm_ptr(&self) -> *const Dcgcpwm { 
        (self.0 + 0x840) as *const Dcgcpwm
    }

    #[doc="Get the *mut pointer for the DCGCPWM register."]
    #[inline] pub fn dcgcpwm_mut(&self) -> *mut Dcgcpwm { 
        (self.0 + 0x840) as *mut Dcgcpwm
    }

    #[doc="Read the DCGCPWM register."]
    #[inline] pub fn dcgcpwm(&self) -> Dcgcpwm { 
        unsafe {
            read_volatile((self.0 + 0x840) as *const Dcgcpwm)
        }
    }

    #[doc="Write the DCGCPWM register."]
    #[inline] pub fn set_dcgcpwm<F: FnOnce(Dcgcpwm) -> Dcgcpwm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x840) as *mut Dcgcpwm, f(Dcgcpwm(0)));
        }
        self
    }

    #[doc="Modify the DCGCPWM register."]
    #[inline] pub fn with_dcgcpwm<F: FnOnce(Dcgcpwm) -> Dcgcpwm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x840) as *mut Dcgcpwm, f(self.dcgcpwm()));
        }
        self
    }

    #[doc="Get the *const pointer for the DCGCQEI register."]
    #[inline] pub fn dcgcqei_ptr(&self) -> *const Dcgcqei { 
        (self.0 + 0x844) as *const Dcgcqei
    }

    #[doc="Get the *mut pointer for the DCGCQEI register."]
    #[inline] pub fn dcgcqei_mut(&self) -> *mut Dcgcqei { 
        (self.0 + 0x844) as *mut Dcgcqei
    }

    #[doc="Read the DCGCQEI register."]
    #[inline] pub fn dcgcqei(&self) -> Dcgcqei { 
        unsafe {
            read_volatile((self.0 + 0x844) as *const Dcgcqei)
        }
    }

    #[doc="Write the DCGCQEI register."]
    #[inline] pub fn set_dcgcqei<F: FnOnce(Dcgcqei) -> Dcgcqei>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x844) as *mut Dcgcqei, f(Dcgcqei(0)));
        }
        self
    }

    #[doc="Modify the DCGCQEI register."]
    #[inline] pub fn with_dcgcqei<F: FnOnce(Dcgcqei) -> Dcgcqei>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x844) as *mut Dcgcqei, f(self.dcgcqei()));
        }
        self
    }

    #[doc="Get the *const pointer for the DCGCEEPROM register."]
    #[inline] pub fn dcgceeprom_ptr(&self) -> *const Dcgceeprom { 
        (self.0 + 0x858) as *const Dcgceeprom
    }

    #[doc="Get the *mut pointer for the DCGCEEPROM register."]
    #[inline] pub fn dcgceeprom_mut(&self) -> *mut Dcgceeprom { 
        (self.0 + 0x858) as *mut Dcgceeprom
    }

    #[doc="Read the DCGCEEPROM register."]
    #[inline] pub fn dcgceeprom(&self) -> Dcgceeprom { 
        unsafe {
            read_volatile((self.0 + 0x858) as *const Dcgceeprom)
        }
    }

    #[doc="Write the DCGCEEPROM register."]
    #[inline] pub fn set_dcgceeprom<F: FnOnce(Dcgceeprom) -> Dcgceeprom>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x858) as *mut Dcgceeprom, f(Dcgceeprom(0)));
        }
        self
    }

    #[doc="Modify the DCGCEEPROM register."]
    #[inline] pub fn with_dcgceeprom<F: FnOnce(Dcgceeprom) -> Dcgceeprom>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x858) as *mut Dcgceeprom, f(self.dcgceeprom()));
        }
        self
    }

    #[doc="Get the *const pointer for the DCGCCCM register."]
    #[inline] pub fn dcgcccm_ptr(&self) -> *const Dcgcccm { 
        (self.0 + 0x874) as *const Dcgcccm
    }

    #[doc="Get the *mut pointer for the DCGCCCM register."]
    #[inline] pub fn dcgcccm_mut(&self) -> *mut Dcgcccm { 
        (self.0 + 0x874) as *mut Dcgcccm
    }

    #[doc="Read the DCGCCCM register."]
    #[inline] pub fn dcgcccm(&self) -> Dcgcccm { 
        unsafe {
            read_volatile((self.0 + 0x874) as *const Dcgcccm)
        }
    }

    #[doc="Write the DCGCCCM register."]
    #[inline] pub fn set_dcgcccm<F: FnOnce(Dcgcccm) -> Dcgcccm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x874) as *mut Dcgcccm, f(Dcgcccm(0)));
        }
        self
    }

    #[doc="Modify the DCGCCCM register."]
    #[inline] pub fn with_dcgcccm<F: FnOnce(Dcgcccm) -> Dcgcccm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x874) as *mut Dcgcccm, f(self.dcgcccm()));
        }
        self
    }

    #[doc="Get the *const pointer for the DCGCEMAC register."]
    #[inline] pub fn dcgcemac_ptr(&self) -> *const Dcgcemac { 
        (self.0 + 0x89c) as *const Dcgcemac
    }

    #[doc="Get the *mut pointer for the DCGCEMAC register."]
    #[inline] pub fn dcgcemac_mut(&self) -> *mut Dcgcemac { 
        (self.0 + 0x89c) as *mut Dcgcemac
    }

    #[doc="Read the DCGCEMAC register."]
    #[inline] pub fn dcgcemac(&self) -> Dcgcemac { 
        unsafe {
            read_volatile((self.0 + 0x89c) as *const Dcgcemac)
        }
    }

    #[doc="Write the DCGCEMAC register."]
    #[inline] pub fn set_dcgcemac<F: FnOnce(Dcgcemac) -> Dcgcemac>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x89c) as *mut Dcgcemac, f(Dcgcemac(0)));
        }
        self
    }

    #[doc="Modify the DCGCEMAC register."]
    #[inline] pub fn with_dcgcemac<F: FnOnce(Dcgcemac) -> Dcgcemac>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x89c) as *mut Dcgcemac, f(self.dcgcemac()));
        }
        self
    }

    #[doc="Get the *const pointer for the PCWD register."]
    #[inline] pub fn pcwd_ptr(&self) -> *const Pcwd { 
        (self.0 + 0x900) as *const Pcwd
    }

    #[doc="Get the *mut pointer for the PCWD register."]
    #[inline] pub fn pcwd_mut(&self) -> *mut Pcwd { 
        (self.0 + 0x900) as *mut Pcwd
    }

    #[doc="Read the PCWD register."]
    #[inline] pub fn pcwd(&self) -> Pcwd { 
        unsafe {
            read_volatile((self.0 + 0x900) as *const Pcwd)
        }
    }

    #[doc="Write the PCWD register."]
    #[inline] pub fn set_pcwd<F: FnOnce(Pcwd) -> Pcwd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x900) as *mut Pcwd, f(Pcwd(0)));
        }
        self
    }

    #[doc="Modify the PCWD register."]
    #[inline] pub fn with_pcwd<F: FnOnce(Pcwd) -> Pcwd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x900) as *mut Pcwd, f(self.pcwd()));
        }
        self
    }

    #[doc="Get the *const pointer for the PCTIMER register."]
    #[inline] pub fn pctimer_ptr(&self) -> *const Pctimer { 
        (self.0 + 0x904) as *const Pctimer
    }

    #[doc="Get the *mut pointer for the PCTIMER register."]
    #[inline] pub fn pctimer_mut(&self) -> *mut Pctimer { 
        (self.0 + 0x904) as *mut Pctimer
    }

    #[doc="Read the PCTIMER register."]
    #[inline] pub fn pctimer(&self) -> Pctimer { 
        unsafe {
            read_volatile((self.0 + 0x904) as *const Pctimer)
        }
    }

    #[doc="Write the PCTIMER register."]
    #[inline] pub fn set_pctimer<F: FnOnce(Pctimer) -> Pctimer>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x904) as *mut Pctimer, f(Pctimer(0)));
        }
        self
    }

    #[doc="Modify the PCTIMER register."]
    #[inline] pub fn with_pctimer<F: FnOnce(Pctimer) -> Pctimer>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x904) as *mut Pctimer, f(self.pctimer()));
        }
        self
    }

    #[doc="Get the *const pointer for the PCGPIO register."]
    #[inline] pub fn pcgpio_ptr(&self) -> *const Pcgpio { 
        (self.0 + 0x908) as *const Pcgpio
    }

    #[doc="Get the *mut pointer for the PCGPIO register."]
    #[inline] pub fn pcgpio_mut(&self) -> *mut Pcgpio { 
        (self.0 + 0x908) as *mut Pcgpio
    }

    #[doc="Read the PCGPIO register."]
    #[inline] pub fn pcgpio(&self) -> Pcgpio { 
        unsafe {
            read_volatile((self.0 + 0x908) as *const Pcgpio)
        }
    }

    #[doc="Write the PCGPIO register."]
    #[inline] pub fn set_pcgpio<F: FnOnce(Pcgpio) -> Pcgpio>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x908) as *mut Pcgpio, f(Pcgpio(0)));
        }
        self
    }

    #[doc="Modify the PCGPIO register."]
    #[inline] pub fn with_pcgpio<F: FnOnce(Pcgpio) -> Pcgpio>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x908) as *mut Pcgpio, f(self.pcgpio()));
        }
        self
    }

    #[doc="Get the *const pointer for the PCDMA register."]
    #[inline] pub fn pcdma_ptr(&self) -> *const Pcdma { 
        (self.0 + 0x90c) as *const Pcdma
    }

    #[doc="Get the *mut pointer for the PCDMA register."]
    #[inline] pub fn pcdma_mut(&self) -> *mut Pcdma { 
        (self.0 + 0x90c) as *mut Pcdma
    }

    #[doc="Read the PCDMA register."]
    #[inline] pub fn pcdma(&self) -> Pcdma { 
        unsafe {
            read_volatile((self.0 + 0x90c) as *const Pcdma)
        }
    }

    #[doc="Write the PCDMA register."]
    #[inline] pub fn set_pcdma<F: FnOnce(Pcdma) -> Pcdma>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x90c) as *mut Pcdma, f(Pcdma(0)));
        }
        self
    }

    #[doc="Modify the PCDMA register."]
    #[inline] pub fn with_pcdma<F: FnOnce(Pcdma) -> Pcdma>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x90c) as *mut Pcdma, f(self.pcdma()));
        }
        self
    }

    #[doc="Get the *const pointer for the PCEPI register."]
    #[inline] pub fn pcepi_ptr(&self) -> *const Pcepi { 
        (self.0 + 0x910) as *const Pcepi
    }

    #[doc="Get the *mut pointer for the PCEPI register."]
    #[inline] pub fn pcepi_mut(&self) -> *mut Pcepi { 
        (self.0 + 0x910) as *mut Pcepi
    }

    #[doc="Read the PCEPI register."]
    #[inline] pub fn pcepi(&self) -> Pcepi { 
        unsafe {
            read_volatile((self.0 + 0x910) as *const Pcepi)
        }
    }

    #[doc="Write the PCEPI register."]
    #[inline] pub fn set_pcepi<F: FnOnce(Pcepi) -> Pcepi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x910) as *mut Pcepi, f(Pcepi(0)));
        }
        self
    }

    #[doc="Modify the PCEPI register."]
    #[inline] pub fn with_pcepi<F: FnOnce(Pcepi) -> Pcepi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x910) as *mut Pcepi, f(self.pcepi()));
        }
        self
    }

    #[doc="Get the *const pointer for the PCHIB register."]
    #[inline] pub fn pchib_ptr(&self) -> *const Pchib { 
        (self.0 + 0x914) as *const Pchib
    }

    #[doc="Get the *mut pointer for the PCHIB register."]
    #[inline] pub fn pchib_mut(&self) -> *mut Pchib { 
        (self.0 + 0x914) as *mut Pchib
    }

    #[doc="Read the PCHIB register."]
    #[inline] pub fn pchib(&self) -> Pchib { 
        unsafe {
            read_volatile((self.0 + 0x914) as *const Pchib)
        }
    }

    #[doc="Write the PCHIB register."]
    #[inline] pub fn set_pchib<F: FnOnce(Pchib) -> Pchib>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x914) as *mut Pchib, f(Pchib(0)));
        }
        self
    }

    #[doc="Modify the PCHIB register."]
    #[inline] pub fn with_pchib<F: FnOnce(Pchib) -> Pchib>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x914) as *mut Pchib, f(self.pchib()));
        }
        self
    }

    #[doc="Get the *const pointer for the PCUART register."]
    #[inline] pub fn pcuart_ptr(&self) -> *const Pcuart { 
        (self.0 + 0x918) as *const Pcuart
    }

    #[doc="Get the *mut pointer for the PCUART register."]
    #[inline] pub fn pcuart_mut(&self) -> *mut Pcuart { 
        (self.0 + 0x918) as *mut Pcuart
    }

    #[doc="Read the PCUART register."]
    #[inline] pub fn pcuart(&self) -> Pcuart { 
        unsafe {
            read_volatile((self.0 + 0x918) as *const Pcuart)
        }
    }

    #[doc="Write the PCUART register."]
    #[inline] pub fn set_pcuart<F: FnOnce(Pcuart) -> Pcuart>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x918) as *mut Pcuart, f(Pcuart(0)));
        }
        self
    }

    #[doc="Modify the PCUART register."]
    #[inline] pub fn with_pcuart<F: FnOnce(Pcuart) -> Pcuart>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x918) as *mut Pcuart, f(self.pcuart()));
        }
        self
    }

    #[doc="Get the *const pointer for the PCSSI register."]
    #[inline] pub fn pcssi_ptr(&self) -> *const Pcssi { 
        (self.0 + 0x91c) as *const Pcssi
    }

    #[doc="Get the *mut pointer for the PCSSI register."]
    #[inline] pub fn pcssi_mut(&self) -> *mut Pcssi { 
        (self.0 + 0x91c) as *mut Pcssi
    }

    #[doc="Read the PCSSI register."]
    #[inline] pub fn pcssi(&self) -> Pcssi { 
        unsafe {
            read_volatile((self.0 + 0x91c) as *const Pcssi)
        }
    }

    #[doc="Write the PCSSI register."]
    #[inline] pub fn set_pcssi<F: FnOnce(Pcssi) -> Pcssi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x91c) as *mut Pcssi, f(Pcssi(0)));
        }
        self
    }

    #[doc="Modify the PCSSI register."]
    #[inline] pub fn with_pcssi<F: FnOnce(Pcssi) -> Pcssi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x91c) as *mut Pcssi, f(self.pcssi()));
        }
        self
    }

    #[doc="Get the *const pointer for the PCI2C register."]
    #[inline] pub fn pci2c_ptr(&self) -> *const Pci2c { 
        (self.0 + 0x920) as *const Pci2c
    }

    #[doc="Get the *mut pointer for the PCI2C register."]
    #[inline] pub fn pci2c_mut(&self) -> *mut Pci2c { 
        (self.0 + 0x920) as *mut Pci2c
    }

    #[doc="Read the PCI2C register."]
    #[inline] pub fn pci2c(&self) -> Pci2c { 
        unsafe {
            read_volatile((self.0 + 0x920) as *const Pci2c)
        }
    }

    #[doc="Write the PCI2C register."]
    #[inline] pub fn set_pci2c<F: FnOnce(Pci2c) -> Pci2c>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x920) as *mut Pci2c, f(Pci2c(0)));
        }
        self
    }

    #[doc="Modify the PCI2C register."]
    #[inline] pub fn with_pci2c<F: FnOnce(Pci2c) -> Pci2c>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x920) as *mut Pci2c, f(self.pci2c()));
        }
        self
    }

    #[doc="Get the *const pointer for the PCUSB register."]
    #[inline] pub fn pcusb_ptr(&self) -> *const Pcusb { 
        (self.0 + 0x928) as *const Pcusb
    }

    #[doc="Get the *mut pointer for the PCUSB register."]
    #[inline] pub fn pcusb_mut(&self) -> *mut Pcusb { 
        (self.0 + 0x928) as *mut Pcusb
    }

    #[doc="Read the PCUSB register."]
    #[inline] pub fn pcusb(&self) -> Pcusb { 
        unsafe {
            read_volatile((self.0 + 0x928) as *const Pcusb)
        }
    }

    #[doc="Write the PCUSB register."]
    #[inline] pub fn set_pcusb<F: FnOnce(Pcusb) -> Pcusb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x928) as *mut Pcusb, f(Pcusb(0)));
        }
        self
    }

    #[doc="Modify the PCUSB register."]
    #[inline] pub fn with_pcusb<F: FnOnce(Pcusb) -> Pcusb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x928) as *mut Pcusb, f(self.pcusb()));
        }
        self
    }

    #[doc="Get the *const pointer for the PCEPHY register."]
    #[inline] pub fn pcephy_ptr(&self) -> *const Pcephy { 
        (self.0 + 0x930) as *const Pcephy
    }

    #[doc="Get the *mut pointer for the PCEPHY register."]
    #[inline] pub fn pcephy_mut(&self) -> *mut Pcephy { 
        (self.0 + 0x930) as *mut Pcephy
    }

    #[doc="Read the PCEPHY register."]
    #[inline] pub fn pcephy(&self) -> Pcephy { 
        unsafe {
            read_volatile((self.0 + 0x930) as *const Pcephy)
        }
    }

    #[doc="Write the PCEPHY register."]
    #[inline] pub fn set_pcephy<F: FnOnce(Pcephy) -> Pcephy>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x930) as *mut Pcephy, f(Pcephy(0)));
        }
        self
    }

    #[doc="Modify the PCEPHY register."]
    #[inline] pub fn with_pcephy<F: FnOnce(Pcephy) -> Pcephy>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x930) as *mut Pcephy, f(self.pcephy()));
        }
        self
    }

    #[doc="Get the *const pointer for the PCCAN register."]
    #[inline] pub fn pccan_ptr(&self) -> *const Pccan { 
        (self.0 + 0x934) as *const Pccan
    }

    #[doc="Get the *mut pointer for the PCCAN register."]
    #[inline] pub fn pccan_mut(&self) -> *mut Pccan { 
        (self.0 + 0x934) as *mut Pccan
    }

    #[doc="Read the PCCAN register."]
    #[inline] pub fn pccan(&self) -> Pccan { 
        unsafe {
            read_volatile((self.0 + 0x934) as *const Pccan)
        }
    }

    #[doc="Write the PCCAN register."]
    #[inline] pub fn set_pccan<F: FnOnce(Pccan) -> Pccan>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x934) as *mut Pccan, f(Pccan(0)));
        }
        self
    }

    #[doc="Modify the PCCAN register."]
    #[inline] pub fn with_pccan<F: FnOnce(Pccan) -> Pccan>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x934) as *mut Pccan, f(self.pccan()));
        }
        self
    }

    #[doc="Get the *const pointer for the PCADC register."]
    #[inline] pub fn pcadc_ptr(&self) -> *const Pcadc { 
        (self.0 + 0x938) as *const Pcadc
    }

    #[doc="Get the *mut pointer for the PCADC register."]
    #[inline] pub fn pcadc_mut(&self) -> *mut Pcadc { 
        (self.0 + 0x938) as *mut Pcadc
    }

    #[doc="Read the PCADC register."]
    #[inline] pub fn pcadc(&self) -> Pcadc { 
        unsafe {
            read_volatile((self.0 + 0x938) as *const Pcadc)
        }
    }

    #[doc="Write the PCADC register."]
    #[inline] pub fn set_pcadc<F: FnOnce(Pcadc) -> Pcadc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x938) as *mut Pcadc, f(Pcadc(0)));
        }
        self
    }

    #[doc="Modify the PCADC register."]
    #[inline] pub fn with_pcadc<F: FnOnce(Pcadc) -> Pcadc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x938) as *mut Pcadc, f(self.pcadc()));
        }
        self
    }

    #[doc="Get the *const pointer for the PCACMP register."]
    #[inline] pub fn pcacmp_ptr(&self) -> *const Pcacmp { 
        (self.0 + 0x93c) as *const Pcacmp
    }

    #[doc="Get the *mut pointer for the PCACMP register."]
    #[inline] pub fn pcacmp_mut(&self) -> *mut Pcacmp { 
        (self.0 + 0x93c) as *mut Pcacmp
    }

    #[doc="Read the PCACMP register."]
    #[inline] pub fn pcacmp(&self) -> Pcacmp { 
        unsafe {
            read_volatile((self.0 + 0x93c) as *const Pcacmp)
        }
    }

    #[doc="Write the PCACMP register."]
    #[inline] pub fn set_pcacmp<F: FnOnce(Pcacmp) -> Pcacmp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x93c) as *mut Pcacmp, f(Pcacmp(0)));
        }
        self
    }

    #[doc="Modify the PCACMP register."]
    #[inline] pub fn with_pcacmp<F: FnOnce(Pcacmp) -> Pcacmp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x93c) as *mut Pcacmp, f(self.pcacmp()));
        }
        self
    }

    #[doc="Get the *const pointer for the PCPWM register."]
    #[inline] pub fn pcpwm_ptr(&self) -> *const Pcpwm { 
        (self.0 + 0x940) as *const Pcpwm
    }

    #[doc="Get the *mut pointer for the PCPWM register."]
    #[inline] pub fn pcpwm_mut(&self) -> *mut Pcpwm { 
        (self.0 + 0x940) as *mut Pcpwm
    }

    #[doc="Read the PCPWM register."]
    #[inline] pub fn pcpwm(&self) -> Pcpwm { 
        unsafe {
            read_volatile((self.0 + 0x940) as *const Pcpwm)
        }
    }

    #[doc="Write the PCPWM register."]
    #[inline] pub fn set_pcpwm<F: FnOnce(Pcpwm) -> Pcpwm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x940) as *mut Pcpwm, f(Pcpwm(0)));
        }
        self
    }

    #[doc="Modify the PCPWM register."]
    #[inline] pub fn with_pcpwm<F: FnOnce(Pcpwm) -> Pcpwm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x940) as *mut Pcpwm, f(self.pcpwm()));
        }
        self
    }

    #[doc="Get the *const pointer for the PCQEI register."]
    #[inline] pub fn pcqei_ptr(&self) -> *const Pcqei { 
        (self.0 + 0x944) as *const Pcqei
    }

    #[doc="Get the *mut pointer for the PCQEI register."]
    #[inline] pub fn pcqei_mut(&self) -> *mut Pcqei { 
        (self.0 + 0x944) as *mut Pcqei
    }

    #[doc="Read the PCQEI register."]
    #[inline] pub fn pcqei(&self) -> Pcqei { 
        unsafe {
            read_volatile((self.0 + 0x944) as *const Pcqei)
        }
    }

    #[doc="Write the PCQEI register."]
    #[inline] pub fn set_pcqei<F: FnOnce(Pcqei) -> Pcqei>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x944) as *mut Pcqei, f(Pcqei(0)));
        }
        self
    }

    #[doc="Modify the PCQEI register."]
    #[inline] pub fn with_pcqei<F: FnOnce(Pcqei) -> Pcqei>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x944) as *mut Pcqei, f(self.pcqei()));
        }
        self
    }

    #[doc="Get the *const pointer for the PCEEPROM register."]
    #[inline] pub fn pceeprom_ptr(&self) -> *const Pceeprom { 
        (self.0 + 0x958) as *const Pceeprom
    }

    #[doc="Get the *mut pointer for the PCEEPROM register."]
    #[inline] pub fn pceeprom_mut(&self) -> *mut Pceeprom { 
        (self.0 + 0x958) as *mut Pceeprom
    }

    #[doc="Read the PCEEPROM register."]
    #[inline] pub fn pceeprom(&self) -> Pceeprom { 
        unsafe {
            read_volatile((self.0 + 0x958) as *const Pceeprom)
        }
    }

    #[doc="Write the PCEEPROM register."]
    #[inline] pub fn set_pceeprom<F: FnOnce(Pceeprom) -> Pceeprom>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x958) as *mut Pceeprom, f(Pceeprom(0)));
        }
        self
    }

    #[doc="Modify the PCEEPROM register."]
    #[inline] pub fn with_pceeprom<F: FnOnce(Pceeprom) -> Pceeprom>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x958) as *mut Pceeprom, f(self.pceeprom()));
        }
        self
    }

    #[doc="Get the *const pointer for the PCCCM register."]
    #[inline] pub fn pcccm_ptr(&self) -> *const Pcccm { 
        (self.0 + 0x974) as *const Pcccm
    }

    #[doc="Get the *mut pointer for the PCCCM register."]
    #[inline] pub fn pcccm_mut(&self) -> *mut Pcccm { 
        (self.0 + 0x974) as *mut Pcccm
    }

    #[doc="Read the PCCCM register."]
    #[inline] pub fn pcccm(&self) -> Pcccm { 
        unsafe {
            read_volatile((self.0 + 0x974) as *const Pcccm)
        }
    }

    #[doc="Write the PCCCM register."]
    #[inline] pub fn set_pcccm<F: FnOnce(Pcccm) -> Pcccm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x974) as *mut Pcccm, f(Pcccm(0)));
        }
        self
    }

    #[doc="Modify the PCCCM register."]
    #[inline] pub fn with_pcccm<F: FnOnce(Pcccm) -> Pcccm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x974) as *mut Pcccm, f(self.pcccm()));
        }
        self
    }

    #[doc="Get the *const pointer for the PCEMAC register."]
    #[inline] pub fn pcemac_ptr(&self) -> *const Pcemac { 
        (self.0 + 0x99c) as *const Pcemac
    }

    #[doc="Get the *mut pointer for the PCEMAC register."]
    #[inline] pub fn pcemac_mut(&self) -> *mut Pcemac { 
        (self.0 + 0x99c) as *mut Pcemac
    }

    #[doc="Read the PCEMAC register."]
    #[inline] pub fn pcemac(&self) -> Pcemac { 
        unsafe {
            read_volatile((self.0 + 0x99c) as *const Pcemac)
        }
    }

    #[doc="Write the PCEMAC register."]
    #[inline] pub fn set_pcemac<F: FnOnce(Pcemac) -> Pcemac>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x99c) as *mut Pcemac, f(Pcemac(0)));
        }
        self
    }

    #[doc="Modify the PCEMAC register."]
    #[inline] pub fn with_pcemac<F: FnOnce(Pcemac) -> Pcemac>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x99c) as *mut Pcemac, f(self.pcemac()));
        }
        self
    }

    #[doc="Get the *const pointer for the PRWD register."]
    #[inline] pub fn prwd_ptr(&self) -> *const Prwd { 
        (self.0 + 0xa00) as *const Prwd
    }

    #[doc="Get the *mut pointer for the PRWD register."]
    #[inline] pub fn prwd_mut(&self) -> *mut Prwd { 
        (self.0 + 0xa00) as *mut Prwd
    }

    #[doc="Read the PRWD register."]
    #[inline] pub fn prwd(&self) -> Prwd { 
        unsafe {
            read_volatile((self.0 + 0xa00) as *const Prwd)
        }
    }

    #[doc="Write the PRWD register."]
    #[inline] pub fn set_prwd<F: FnOnce(Prwd) -> Prwd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa00) as *mut Prwd, f(Prwd(0)));
        }
        self
    }

    #[doc="Modify the PRWD register."]
    #[inline] pub fn with_prwd<F: FnOnce(Prwd) -> Prwd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa00) as *mut Prwd, f(self.prwd()));
        }
        self
    }

    #[doc="Get the *const pointer for the PRTIMER register."]
    #[inline] pub fn prtimer_ptr(&self) -> *const Prtimer { 
        (self.0 + 0xa04) as *const Prtimer
    }

    #[doc="Get the *mut pointer for the PRTIMER register."]
    #[inline] pub fn prtimer_mut(&self) -> *mut Prtimer { 
        (self.0 + 0xa04) as *mut Prtimer
    }

    #[doc="Read the PRTIMER register."]
    #[inline] pub fn prtimer(&self) -> Prtimer { 
        unsafe {
            read_volatile((self.0 + 0xa04) as *const Prtimer)
        }
    }

    #[doc="Write the PRTIMER register."]
    #[inline] pub fn set_prtimer<F: FnOnce(Prtimer) -> Prtimer>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa04) as *mut Prtimer, f(Prtimer(0)));
        }
        self
    }

    #[doc="Modify the PRTIMER register."]
    #[inline] pub fn with_prtimer<F: FnOnce(Prtimer) -> Prtimer>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa04) as *mut Prtimer, f(self.prtimer()));
        }
        self
    }

    #[doc="Get the *const pointer for the PRGPIO register."]
    #[inline] pub fn prgpio_ptr(&self) -> *const Prgpio { 
        (self.0 + 0xa08) as *const Prgpio
    }

    #[doc="Get the *mut pointer for the PRGPIO register."]
    #[inline] pub fn prgpio_mut(&self) -> *mut Prgpio { 
        (self.0 + 0xa08) as *mut Prgpio
    }

    #[doc="Read the PRGPIO register."]
    #[inline] pub fn prgpio(&self) -> Prgpio { 
        unsafe {
            read_volatile((self.0 + 0xa08) as *const Prgpio)
        }
    }

    #[doc="Write the PRGPIO register."]
    #[inline] pub fn set_prgpio<F: FnOnce(Prgpio) -> Prgpio>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa08) as *mut Prgpio, f(Prgpio(0)));
        }
        self
    }

    #[doc="Modify the PRGPIO register."]
    #[inline] pub fn with_prgpio<F: FnOnce(Prgpio) -> Prgpio>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa08) as *mut Prgpio, f(self.prgpio()));
        }
        self
    }

    #[doc="Get the *const pointer for the PRDMA register."]
    #[inline] pub fn prdma_ptr(&self) -> *const Prdma { 
        (self.0 + 0xa0c) as *const Prdma
    }

    #[doc="Get the *mut pointer for the PRDMA register."]
    #[inline] pub fn prdma_mut(&self) -> *mut Prdma { 
        (self.0 + 0xa0c) as *mut Prdma
    }

    #[doc="Read the PRDMA register."]
    #[inline] pub fn prdma(&self) -> Prdma { 
        unsafe {
            read_volatile((self.0 + 0xa0c) as *const Prdma)
        }
    }

    #[doc="Write the PRDMA register."]
    #[inline] pub fn set_prdma<F: FnOnce(Prdma) -> Prdma>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa0c) as *mut Prdma, f(Prdma(0)));
        }
        self
    }

    #[doc="Modify the PRDMA register."]
    #[inline] pub fn with_prdma<F: FnOnce(Prdma) -> Prdma>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa0c) as *mut Prdma, f(self.prdma()));
        }
        self
    }

    #[doc="Get the *const pointer for the PREPI register."]
    #[inline] pub fn prepi_ptr(&self) -> *const Prepi { 
        (self.0 + 0xa10) as *const Prepi
    }

    #[doc="Get the *mut pointer for the PREPI register."]
    #[inline] pub fn prepi_mut(&self) -> *mut Prepi { 
        (self.0 + 0xa10) as *mut Prepi
    }

    #[doc="Read the PREPI register."]
    #[inline] pub fn prepi(&self) -> Prepi { 
        unsafe {
            read_volatile((self.0 + 0xa10) as *const Prepi)
        }
    }

    #[doc="Write the PREPI register."]
    #[inline] pub fn set_prepi<F: FnOnce(Prepi) -> Prepi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa10) as *mut Prepi, f(Prepi(0)));
        }
        self
    }

    #[doc="Modify the PREPI register."]
    #[inline] pub fn with_prepi<F: FnOnce(Prepi) -> Prepi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa10) as *mut Prepi, f(self.prepi()));
        }
        self
    }

    #[doc="Get the *const pointer for the PRHIB register."]
    #[inline] pub fn prhib_ptr(&self) -> *const Prhib { 
        (self.0 + 0xa14) as *const Prhib
    }

    #[doc="Get the *mut pointer for the PRHIB register."]
    #[inline] pub fn prhib_mut(&self) -> *mut Prhib { 
        (self.0 + 0xa14) as *mut Prhib
    }

    #[doc="Read the PRHIB register."]
    #[inline] pub fn prhib(&self) -> Prhib { 
        unsafe {
            read_volatile((self.0 + 0xa14) as *const Prhib)
        }
    }

    #[doc="Write the PRHIB register."]
    #[inline] pub fn set_prhib<F: FnOnce(Prhib) -> Prhib>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa14) as *mut Prhib, f(Prhib(0)));
        }
        self
    }

    #[doc="Modify the PRHIB register."]
    #[inline] pub fn with_prhib<F: FnOnce(Prhib) -> Prhib>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa14) as *mut Prhib, f(self.prhib()));
        }
        self
    }

    #[doc="Get the *const pointer for the PRUART register."]
    #[inline] pub fn pruart_ptr(&self) -> *const Pruart { 
        (self.0 + 0xa18) as *const Pruart
    }

    #[doc="Get the *mut pointer for the PRUART register."]
    #[inline] pub fn pruart_mut(&self) -> *mut Pruart { 
        (self.0 + 0xa18) as *mut Pruart
    }

    #[doc="Read the PRUART register."]
    #[inline] pub fn pruart(&self) -> Pruart { 
        unsafe {
            read_volatile((self.0 + 0xa18) as *const Pruart)
        }
    }

    #[doc="Write the PRUART register."]
    #[inline] pub fn set_pruart<F: FnOnce(Pruart) -> Pruart>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa18) as *mut Pruart, f(Pruart(0)));
        }
        self
    }

    #[doc="Modify the PRUART register."]
    #[inline] pub fn with_pruart<F: FnOnce(Pruart) -> Pruart>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa18) as *mut Pruart, f(self.pruart()));
        }
        self
    }

    #[doc="Get the *const pointer for the PRSSI register."]
    #[inline] pub fn prssi_ptr(&self) -> *const Prssi { 
        (self.0 + 0xa1c) as *const Prssi
    }

    #[doc="Get the *mut pointer for the PRSSI register."]
    #[inline] pub fn prssi_mut(&self) -> *mut Prssi { 
        (self.0 + 0xa1c) as *mut Prssi
    }

    #[doc="Read the PRSSI register."]
    #[inline] pub fn prssi(&self) -> Prssi { 
        unsafe {
            read_volatile((self.0 + 0xa1c) as *const Prssi)
        }
    }

    #[doc="Write the PRSSI register."]
    #[inline] pub fn set_prssi<F: FnOnce(Prssi) -> Prssi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa1c) as *mut Prssi, f(Prssi(0)));
        }
        self
    }

    #[doc="Modify the PRSSI register."]
    #[inline] pub fn with_prssi<F: FnOnce(Prssi) -> Prssi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa1c) as *mut Prssi, f(self.prssi()));
        }
        self
    }

    #[doc="Get the *const pointer for the PRI2C register."]
    #[inline] pub fn pri2c_ptr(&self) -> *const Pri2c { 
        (self.0 + 0xa20) as *const Pri2c
    }

    #[doc="Get the *mut pointer for the PRI2C register."]
    #[inline] pub fn pri2c_mut(&self) -> *mut Pri2c { 
        (self.0 + 0xa20) as *mut Pri2c
    }

    #[doc="Read the PRI2C register."]
    #[inline] pub fn pri2c(&self) -> Pri2c { 
        unsafe {
            read_volatile((self.0 + 0xa20) as *const Pri2c)
        }
    }

    #[doc="Write the PRI2C register."]
    #[inline] pub fn set_pri2c<F: FnOnce(Pri2c) -> Pri2c>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa20) as *mut Pri2c, f(Pri2c(0)));
        }
        self
    }

    #[doc="Modify the PRI2C register."]
    #[inline] pub fn with_pri2c<F: FnOnce(Pri2c) -> Pri2c>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa20) as *mut Pri2c, f(self.pri2c()));
        }
        self
    }

    #[doc="Get the *const pointer for the PRUSB register."]
    #[inline] pub fn prusb_ptr(&self) -> *const Prusb { 
        (self.0 + 0xa28) as *const Prusb
    }

    #[doc="Get the *mut pointer for the PRUSB register."]
    #[inline] pub fn prusb_mut(&self) -> *mut Prusb { 
        (self.0 + 0xa28) as *mut Prusb
    }

    #[doc="Read the PRUSB register."]
    #[inline] pub fn prusb(&self) -> Prusb { 
        unsafe {
            read_volatile((self.0 + 0xa28) as *const Prusb)
        }
    }

    #[doc="Write the PRUSB register."]
    #[inline] pub fn set_prusb<F: FnOnce(Prusb) -> Prusb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa28) as *mut Prusb, f(Prusb(0)));
        }
        self
    }

    #[doc="Modify the PRUSB register."]
    #[inline] pub fn with_prusb<F: FnOnce(Prusb) -> Prusb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa28) as *mut Prusb, f(self.prusb()));
        }
        self
    }

    #[doc="Get the *const pointer for the PREPHY register."]
    #[inline] pub fn prephy_ptr(&self) -> *const Prephy { 
        (self.0 + 0xa30) as *const Prephy
    }

    #[doc="Get the *mut pointer for the PREPHY register."]
    #[inline] pub fn prephy_mut(&self) -> *mut Prephy { 
        (self.0 + 0xa30) as *mut Prephy
    }

    #[doc="Read the PREPHY register."]
    #[inline] pub fn prephy(&self) -> Prephy { 
        unsafe {
            read_volatile((self.0 + 0xa30) as *const Prephy)
        }
    }

    #[doc="Write the PREPHY register."]
    #[inline] pub fn set_prephy<F: FnOnce(Prephy) -> Prephy>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa30) as *mut Prephy, f(Prephy(0)));
        }
        self
    }

    #[doc="Modify the PREPHY register."]
    #[inline] pub fn with_prephy<F: FnOnce(Prephy) -> Prephy>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa30) as *mut Prephy, f(self.prephy()));
        }
        self
    }

    #[doc="Get the *const pointer for the PRCAN register."]
    #[inline] pub fn prcan_ptr(&self) -> *const Prcan { 
        (self.0 + 0xa34) as *const Prcan
    }

    #[doc="Get the *mut pointer for the PRCAN register."]
    #[inline] pub fn prcan_mut(&self) -> *mut Prcan { 
        (self.0 + 0xa34) as *mut Prcan
    }

    #[doc="Read the PRCAN register."]
    #[inline] pub fn prcan(&self) -> Prcan { 
        unsafe {
            read_volatile((self.0 + 0xa34) as *const Prcan)
        }
    }

    #[doc="Write the PRCAN register."]
    #[inline] pub fn set_prcan<F: FnOnce(Prcan) -> Prcan>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa34) as *mut Prcan, f(Prcan(0)));
        }
        self
    }

    #[doc="Modify the PRCAN register."]
    #[inline] pub fn with_prcan<F: FnOnce(Prcan) -> Prcan>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa34) as *mut Prcan, f(self.prcan()));
        }
        self
    }

    #[doc="Get the *const pointer for the PRADC register."]
    #[inline] pub fn pradc_ptr(&self) -> *const Pradc { 
        (self.0 + 0xa38) as *const Pradc
    }

    #[doc="Get the *mut pointer for the PRADC register."]
    #[inline] pub fn pradc_mut(&self) -> *mut Pradc { 
        (self.0 + 0xa38) as *mut Pradc
    }

    #[doc="Read the PRADC register."]
    #[inline] pub fn pradc(&self) -> Pradc { 
        unsafe {
            read_volatile((self.0 + 0xa38) as *const Pradc)
        }
    }

    #[doc="Write the PRADC register."]
    #[inline] pub fn set_pradc<F: FnOnce(Pradc) -> Pradc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa38) as *mut Pradc, f(Pradc(0)));
        }
        self
    }

    #[doc="Modify the PRADC register."]
    #[inline] pub fn with_pradc<F: FnOnce(Pradc) -> Pradc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa38) as *mut Pradc, f(self.pradc()));
        }
        self
    }

    #[doc="Get the *const pointer for the PRACMP register."]
    #[inline] pub fn pracmp_ptr(&self) -> *const Pracmp { 
        (self.0 + 0xa3c) as *const Pracmp
    }

    #[doc="Get the *mut pointer for the PRACMP register."]
    #[inline] pub fn pracmp_mut(&self) -> *mut Pracmp { 
        (self.0 + 0xa3c) as *mut Pracmp
    }

    #[doc="Read the PRACMP register."]
    #[inline] pub fn pracmp(&self) -> Pracmp { 
        unsafe {
            read_volatile((self.0 + 0xa3c) as *const Pracmp)
        }
    }

    #[doc="Write the PRACMP register."]
    #[inline] pub fn set_pracmp<F: FnOnce(Pracmp) -> Pracmp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa3c) as *mut Pracmp, f(Pracmp(0)));
        }
        self
    }

    #[doc="Modify the PRACMP register."]
    #[inline] pub fn with_pracmp<F: FnOnce(Pracmp) -> Pracmp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa3c) as *mut Pracmp, f(self.pracmp()));
        }
        self
    }

    #[doc="Get the *const pointer for the PRPWM register."]
    #[inline] pub fn prpwm_ptr(&self) -> *const Prpwm { 
        (self.0 + 0xa40) as *const Prpwm
    }

    #[doc="Get the *mut pointer for the PRPWM register."]
    #[inline] pub fn prpwm_mut(&self) -> *mut Prpwm { 
        (self.0 + 0xa40) as *mut Prpwm
    }

    #[doc="Read the PRPWM register."]
    #[inline] pub fn prpwm(&self) -> Prpwm { 
        unsafe {
            read_volatile((self.0 + 0xa40) as *const Prpwm)
        }
    }

    #[doc="Write the PRPWM register."]
    #[inline] pub fn set_prpwm<F: FnOnce(Prpwm) -> Prpwm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa40) as *mut Prpwm, f(Prpwm(0)));
        }
        self
    }

    #[doc="Modify the PRPWM register."]
    #[inline] pub fn with_prpwm<F: FnOnce(Prpwm) -> Prpwm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa40) as *mut Prpwm, f(self.prpwm()));
        }
        self
    }

    #[doc="Get the *const pointer for the PRQEI register."]
    #[inline] pub fn prqei_ptr(&self) -> *const Prqei { 
        (self.0 + 0xa44) as *const Prqei
    }

    #[doc="Get the *mut pointer for the PRQEI register."]
    #[inline] pub fn prqei_mut(&self) -> *mut Prqei { 
        (self.0 + 0xa44) as *mut Prqei
    }

    #[doc="Read the PRQEI register."]
    #[inline] pub fn prqei(&self) -> Prqei { 
        unsafe {
            read_volatile((self.0 + 0xa44) as *const Prqei)
        }
    }

    #[doc="Write the PRQEI register."]
    #[inline] pub fn set_prqei<F: FnOnce(Prqei) -> Prqei>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa44) as *mut Prqei, f(Prqei(0)));
        }
        self
    }

    #[doc="Modify the PRQEI register."]
    #[inline] pub fn with_prqei<F: FnOnce(Prqei) -> Prqei>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa44) as *mut Prqei, f(self.prqei()));
        }
        self
    }

    #[doc="Get the *const pointer for the PREEPROM register."]
    #[inline] pub fn preeprom_ptr(&self) -> *const Preeprom { 
        (self.0 + 0xa58) as *const Preeprom
    }

    #[doc="Get the *mut pointer for the PREEPROM register."]
    #[inline] pub fn preeprom_mut(&self) -> *mut Preeprom { 
        (self.0 + 0xa58) as *mut Preeprom
    }

    #[doc="Read the PREEPROM register."]
    #[inline] pub fn preeprom(&self) -> Preeprom { 
        unsafe {
            read_volatile((self.0 + 0xa58) as *const Preeprom)
        }
    }

    #[doc="Write the PREEPROM register."]
    #[inline] pub fn set_preeprom<F: FnOnce(Preeprom) -> Preeprom>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa58) as *mut Preeprom, f(Preeprom(0)));
        }
        self
    }

    #[doc="Modify the PREEPROM register."]
    #[inline] pub fn with_preeprom<F: FnOnce(Preeprom) -> Preeprom>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa58) as *mut Preeprom, f(self.preeprom()));
        }
        self
    }

    #[doc="Get the *const pointer for the PRCCM register."]
    #[inline] pub fn prccm_ptr(&self) -> *const Prccm { 
        (self.0 + 0xa74) as *const Prccm
    }

    #[doc="Get the *mut pointer for the PRCCM register."]
    #[inline] pub fn prccm_mut(&self) -> *mut Prccm { 
        (self.0 + 0xa74) as *mut Prccm
    }

    #[doc="Read the PRCCM register."]
    #[inline] pub fn prccm(&self) -> Prccm { 
        unsafe {
            read_volatile((self.0 + 0xa74) as *const Prccm)
        }
    }

    #[doc="Write the PRCCM register."]
    #[inline] pub fn set_prccm<F: FnOnce(Prccm) -> Prccm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa74) as *mut Prccm, f(Prccm(0)));
        }
        self
    }

    #[doc="Modify the PRCCM register."]
    #[inline] pub fn with_prccm<F: FnOnce(Prccm) -> Prccm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa74) as *mut Prccm, f(self.prccm()));
        }
        self
    }

    #[doc="Get the *const pointer for the PREMAC register."]
    #[inline] pub fn premac_ptr(&self) -> *const Premac { 
        (self.0 + 0xa9c) as *const Premac
    }

    #[doc="Get the *mut pointer for the PREMAC register."]
    #[inline] pub fn premac_mut(&self) -> *mut Premac { 
        (self.0 + 0xa9c) as *mut Premac
    }

    #[doc="Read the PREMAC register."]
    #[inline] pub fn premac(&self) -> Premac { 
        unsafe {
            read_volatile((self.0 + 0xa9c) as *const Premac)
        }
    }

    #[doc="Write the PREMAC register."]
    #[inline] pub fn set_premac<F: FnOnce(Premac) -> Premac>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa9c) as *mut Premac, f(Premac(0)));
        }
        self
    }

    #[doc="Modify the PREMAC register."]
    #[inline] pub fn with_premac<F: FnOnce(Premac) -> Premac>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xa9c) as *mut Premac, f(self.premac()));
        }
        self
    }

}

#[doc="Device Identification 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Did0(pub u32);
impl Did0 {
    #[doc="Minor Revision"]
    #[inline] pub fn min(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Minor Revision"]
    #[inline] pub fn test_min(&self) -> bool {
        self.min() != 0
    }

    #[doc="Minor Revision"]
    #[inline] pub fn set_min<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Major Revision"]
    #[inline] pub fn maj(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Major Revision"]
    #[inline] pub fn test_maj(&self) -> bool {
        self.maj() != 0
    }

    #[doc="Major Revision"]
    #[inline] pub fn set_maj<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Device Class"]
    #[inline] pub fn class(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Device Class"]
    #[inline] pub fn test_class(&self) -> bool {
        self.class() != 0
    }

    #[doc="Device Class"]
    #[inline] pub fn set_class<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DID0 Version"]
    #[inline] pub fn ver(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x7) as u8) } // [30:28]
    }

    #[doc="DID0 Version"]
    #[inline] pub fn test_ver(&self) -> bool {
        self.ver() != 0
    }

    #[doc="DID0 Version"]
    #[inline] pub fn set_ver<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Did0 {
    #[inline]
    fn from(other: u32) -> Self {
         Did0(other)
    }
}

impl ::core::fmt::Display for Did0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Did0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.min() != 0 { try!(write!(f, " min=0x{:x}", self.min()))}
        if self.maj() != 0 { try!(write!(f, " maj=0x{:x}", self.maj()))}
        if self.class() != 0 { try!(write!(f, " class=0x{:x}", self.class()))}
        if self.ver() != 0 { try!(write!(f, " ver=0x{:x}", self.ver()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Device Identification 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Did1(pub u32);
impl Did1 {
    #[doc="Qualification Status"]
    #[inline] pub fn qual(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Qualification Status"]
    #[inline] pub fn test_qual(&self) -> bool {
        self.qual() != 0
    }

    #[doc="Qualification Status"]
    #[inline] pub fn set_qual<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="RoHS-Compliance"]
    #[inline] pub fn rohs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="RoHS-Compliance"]
    #[inline] pub fn test_rohs(&self) -> bool {
        self.rohs() != 0
    }

    #[doc="RoHS-Compliance"]
    #[inline] pub fn set_rohs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Package Type"]
    #[inline] pub fn pkg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Package Type"]
    #[inline] pub fn test_pkg(&self) -> bool {
        self.pkg() != 0
    }

    #[doc="Package Type"]
    #[inline] pub fn set_pkg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Temperature Range"]
    #[inline] pub fn temp(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7) as u8) } // [7:5]
    }

    #[doc="Temperature Range"]
    #[inline] pub fn test_temp(&self) -> bool {
        self.temp() != 0
    }

    #[doc="Temperature Range"]
    #[inline] pub fn set_temp<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Package Pin Count"]
    #[inline] pub fn pincnt(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x7) as u8) } // [15:13]
    }

    #[doc="Package Pin Count"]
    #[inline] pub fn test_pincnt(&self) -> bool {
        self.pincnt() != 0
    }

    #[doc="Package Pin Count"]
    #[inline] pub fn set_pincnt<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Part Number"]
    #[inline] pub fn prtno(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Part Number"]
    #[inline] pub fn test_prtno(&self) -> bool {
        self.prtno() != 0
    }

    #[doc="Part Number"]
    #[inline] pub fn set_prtno<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Family"]
    #[inline] pub fn fam(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Family"]
    #[inline] pub fn test_fam(&self) -> bool {
        self.fam() != 0
    }

    #[doc="Family"]
    #[inline] pub fn set_fam<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="DID1 Version"]
    #[inline] pub fn ver(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0xf) as u8) } // [31:28]
    }

    #[doc="DID1 Version"]
    #[inline] pub fn test_ver(&self) -> bool {
        self.ver() != 0
    }

    #[doc="DID1 Version"]
    #[inline] pub fn set_ver<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Did1 {
    #[inline]
    fn from(other: u32) -> Self {
         Did1(other)
    }
}

impl ::core::fmt::Display for Did1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Did1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.qual() != 0 { try!(write!(f, " qual=0x{:x}", self.qual()))}
        if self.rohs() != 0 { try!(write!(f, " rohs"))}
        if self.pkg() != 0 { try!(write!(f, " pkg=0x{:x}", self.pkg()))}
        if self.temp() != 0 { try!(write!(f, " temp=0x{:x}", self.temp()))}
        if self.pincnt() != 0 { try!(write!(f, " pincnt=0x{:x}", self.pincnt()))}
        if self.prtno() != 0 { try!(write!(f, " prtno=0x{:x}", self.prtno()))}
        if self.fam() != 0 { try!(write!(f, " fam=0x{:x}", self.fam()))}
        if self.ver() != 0 { try!(write!(f, " ver=0x{:x}", self.ver()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power-Temp Brown Out Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ptboctl(pub u32);
impl Ptboctl {
    #[doc="VDD (VDDS) under BOR Event Action"]
    #[inline] pub fn vdd_ubor(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="VDD (VDDS) under BOR Event Action"]
    #[inline] pub fn test_vdd_ubor(&self) -> bool {
        self.vdd_ubor() != 0
    }

    #[doc="VDD (VDDS) under BOR Event Action"]
    #[inline] pub fn set_vdd_ubor<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="VDDA under BOR Event Action"]
    #[inline] pub fn vdda_ubor(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="VDDA under BOR Event Action"]
    #[inline] pub fn test_vdda_ubor(&self) -> bool {
        self.vdda_ubor() != 0
    }

    #[doc="VDDA under BOR Event Action"]
    #[inline] pub fn set_vdda_ubor<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Ptboctl {
    #[inline]
    fn from(other: u32) -> Self {
         Ptboctl(other)
    }
}

impl ::core::fmt::Display for Ptboctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ptboctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.vdd_ubor() != 0 { try!(write!(f, " vdd_ubor=0x{:x}", self.vdd_ubor()))}
        if self.vdda_ubor() != 0 { try!(write!(f, " vdda_ubor=0x{:x}", self.vdda_ubor()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Raw Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ris(pub u32);
impl Ris {
    #[doc="Brown-Out Reset Raw Interrupt Status"]
    #[inline] pub fn borris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Brown-Out Reset Raw Interrupt Status"]
    #[inline] pub fn test_borris(&self) -> bool {
        self.borris() != 0
    }

    #[doc="Brown-Out Reset Raw Interrupt Status"]
    #[inline] pub fn set_borris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Main Oscillator Failure Raw Interrupt Status"]
    #[inline] pub fn mofris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Main Oscillator Failure Raw Interrupt Status"]
    #[inline] pub fn test_mofris(&self) -> bool {
        self.mofris() != 0
    }

    #[doc="Main Oscillator Failure Raw Interrupt Status"]
    #[inline] pub fn set_mofris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="PLL Lock Raw Interrupt Status"]
    #[inline] pub fn plllris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="PLL Lock Raw Interrupt Status"]
    #[inline] pub fn test_plllris(&self) -> bool {
        self.plllris() != 0
    }

    #[doc="PLL Lock Raw Interrupt Status"]
    #[inline] pub fn set_plllris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="MOSC Power Up Raw Interrupt Status"]
    #[inline] pub fn moscpupris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="MOSC Power Up Raw Interrupt Status"]
    #[inline] pub fn test_moscpupris(&self) -> bool {
        self.moscpupris() != 0
    }

    #[doc="MOSC Power Up Raw Interrupt Status"]
    #[inline] pub fn set_moscpupris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Ris {
    #[inline]
    fn from(other: u32) -> Self {
         Ris(other)
    }
}

impl ::core::fmt::Display for Ris {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ris {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.borris() != 0 { try!(write!(f, " borris"))}
        if self.mofris() != 0 { try!(write!(f, " mofris"))}
        if self.plllris() != 0 { try!(write!(f, " plllris"))}
        if self.moscpupris() != 0 { try!(write!(f, " moscpupris"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Mask Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Imc(pub u32);
impl Imc {
    #[doc="Brown-Out Reset Interrupt Mask"]
    #[inline] pub fn borim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Brown-Out Reset Interrupt Mask"]
    #[inline] pub fn test_borim(&self) -> bool {
        self.borim() != 0
    }

    #[doc="Brown-Out Reset Interrupt Mask"]
    #[inline] pub fn set_borim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Main Oscillator Failure Interrupt Mask"]
    #[inline] pub fn mofim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Main Oscillator Failure Interrupt Mask"]
    #[inline] pub fn test_mofim(&self) -> bool {
        self.mofim() != 0
    }

    #[doc="Main Oscillator Failure Interrupt Mask"]
    #[inline] pub fn set_mofim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="PLL Lock Interrupt Mask"]
    #[inline] pub fn plllim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="PLL Lock Interrupt Mask"]
    #[inline] pub fn test_plllim(&self) -> bool {
        self.plllim() != 0
    }

    #[doc="PLL Lock Interrupt Mask"]
    #[inline] pub fn set_plllim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="MOSC Power Up Interrupt Mask"]
    #[inline] pub fn moscpupim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="MOSC Power Up Interrupt Mask"]
    #[inline] pub fn test_moscpupim(&self) -> bool {
        self.moscpupim() != 0
    }

    #[doc="MOSC Power Up Interrupt Mask"]
    #[inline] pub fn set_moscpupim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Imc {
    #[inline]
    fn from(other: u32) -> Self {
         Imc(other)
    }
}

impl ::core::fmt::Display for Imc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Imc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.borim() != 0 { try!(write!(f, " borim"))}
        if self.mofim() != 0 { try!(write!(f, " mofim"))}
        if self.plllim() != 0 { try!(write!(f, " plllim"))}
        if self.moscpupim() != 0 { try!(write!(f, " moscpupim"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Masked Interrupt Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Misc(pub u32);
impl Misc {
    #[doc="BOR Masked Interrupt Status"]
    #[inline] pub fn bormis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="BOR Masked Interrupt Status"]
    #[inline] pub fn test_bormis(&self) -> bool {
        self.bormis() != 0
    }

    #[doc="BOR Masked Interrupt Status"]
    #[inline] pub fn set_bormis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Main Oscillator Failure Masked Interrupt Status"]
    #[inline] pub fn mofmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Main Oscillator Failure Masked Interrupt Status"]
    #[inline] pub fn test_mofmis(&self) -> bool {
        self.mofmis() != 0
    }

    #[doc="Main Oscillator Failure Masked Interrupt Status"]
    #[inline] pub fn set_mofmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="PLL Lock Masked Interrupt Status"]
    #[inline] pub fn plllmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="PLL Lock Masked Interrupt Status"]
    #[inline] pub fn test_plllmis(&self) -> bool {
        self.plllmis() != 0
    }

    #[doc="PLL Lock Masked Interrupt Status"]
    #[inline] pub fn set_plllmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="MOSC Power Up Masked Interrupt Status"]
    #[inline] pub fn moscpupmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="MOSC Power Up Masked Interrupt Status"]
    #[inline] pub fn test_moscpupmis(&self) -> bool {
        self.moscpupmis() != 0
    }

    #[doc="MOSC Power Up Masked Interrupt Status"]
    #[inline] pub fn set_moscpupmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Misc {
    #[inline]
    fn from(other: u32) -> Self {
         Misc(other)
    }
}

impl ::core::fmt::Display for Misc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Misc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bormis() != 0 { try!(write!(f, " bormis"))}
        if self.mofmis() != 0 { try!(write!(f, " mofmis"))}
        if self.plllmis() != 0 { try!(write!(f, " plllmis"))}
        if self.moscpupmis() != 0 { try!(write!(f, " moscpupmis"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reset Cause"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Resc(pub u32);
impl Resc {
    #[doc="External Reset"]
    #[inline] pub fn ext(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="External Reset"]
    #[inline] pub fn test_ext(&self) -> bool {
        self.ext() != 0
    }

    #[doc="External Reset"]
    #[inline] pub fn set_ext<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Power-On Reset"]
    #[inline] pub fn por(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Power-On Reset"]
    #[inline] pub fn test_por(&self) -> bool {
        self.por() != 0
    }

    #[doc="Power-On Reset"]
    #[inline] pub fn set_por<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Brown-Out Reset"]
    #[inline] pub fn bor(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Brown-Out Reset"]
    #[inline] pub fn test_bor(&self) -> bool {
        self.bor() != 0
    }

    #[doc="Brown-Out Reset"]
    #[inline] pub fn set_bor<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Watchdog Timer 0 Reset"]
    #[inline] pub fn wdt0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Watchdog Timer 0 Reset"]
    #[inline] pub fn test_wdt0(&self) -> bool {
        self.wdt0() != 0
    }

    #[doc="Watchdog Timer 0 Reset"]
    #[inline] pub fn set_wdt0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Software Reset"]
    #[inline] pub fn sw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Software Reset"]
    #[inline] pub fn test_sw(&self) -> bool {
        self.sw() != 0
    }

    #[doc="Software Reset"]
    #[inline] pub fn set_sw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Watchdog Timer 1 Reset"]
    #[inline] pub fn wdt1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Watchdog Timer 1 Reset"]
    #[inline] pub fn test_wdt1(&self) -> bool {
        self.wdt1() != 0
    }

    #[doc="Watchdog Timer 1 Reset"]
    #[inline] pub fn set_wdt1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="HIB Reset"]
    #[inline] pub fn hib(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="HIB Reset"]
    #[inline] pub fn test_hib(&self) -> bool {
        self.hib() != 0
    }

    #[doc="HIB Reset"]
    #[inline] pub fn set_hib<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="HSSR Reset"]
    #[inline] pub fn hssr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="HSSR Reset"]
    #[inline] pub fn test_hssr(&self) -> bool {
        self.hssr() != 0
    }

    #[doc="HSSR Reset"]
    #[inline] pub fn set_hssr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="MOSC Failure Reset"]
    #[inline] pub fn moscfail(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="MOSC Failure Reset"]
    #[inline] pub fn test_moscfail(&self) -> bool {
        self.moscfail() != 0
    }

    #[doc="MOSC Failure Reset"]
    #[inline] pub fn set_moscfail<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Resc {
    #[inline]
    fn from(other: u32) -> Self {
         Resc(other)
    }
}

impl ::core::fmt::Display for Resc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Resc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ext() != 0 { try!(write!(f, " ext"))}
        if self.por() != 0 { try!(write!(f, " por"))}
        if self.bor() != 0 { try!(write!(f, " bor"))}
        if self.wdt0() != 0 { try!(write!(f, " wdt0"))}
        if self.sw() != 0 { try!(write!(f, " sw"))}
        if self.wdt1() != 0 { try!(write!(f, " wdt1"))}
        if self.hib() != 0 { try!(write!(f, " hib"))}
        if self.hssr() != 0 { try!(write!(f, " hssr"))}
        if self.moscfail() != 0 { try!(write!(f, " moscfail"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power-Temperature Cause"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pwrtc(pub u32);
impl Pwrtc {
    #[doc="VDD Under BOR Status"]
    #[inline] pub fn vdd_ubor(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="VDD Under BOR Status"]
    #[inline] pub fn test_vdd_ubor(&self) -> bool {
        self.vdd_ubor() != 0
    }

    #[doc="VDD Under BOR Status"]
    #[inline] pub fn set_vdd_ubor<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="VDDA Under BOR Status"]
    #[inline] pub fn vdda_ubor(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="VDDA Under BOR Status"]
    #[inline] pub fn test_vdda_ubor(&self) -> bool {
        self.vdda_ubor() != 0
    }

    #[doc="VDDA Under BOR Status"]
    #[inline] pub fn set_vdda_ubor<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Pwrtc {
    #[inline]
    fn from(other: u32) -> Self {
         Pwrtc(other)
    }
}

impl ::core::fmt::Display for Pwrtc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pwrtc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.vdd_ubor() != 0 { try!(write!(f, " vdd_ubor"))}
        if self.vdda_ubor() != 0 { try!(write!(f, " vdda_ubor"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="NMI Cause Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Nmic(pub u32);
impl Nmic {
    #[doc="External Pin NMI"]
    #[inline] pub fn external(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="External Pin NMI"]
    #[inline] pub fn test_external(&self) -> bool {
        self.external() != 0
    }

    #[doc="External Pin NMI"]
    #[inline] pub fn set_external<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Power/Brown Out Event NMI"]
    #[inline] pub fn power(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Power/Brown Out Event NMI"]
    #[inline] pub fn test_power(&self) -> bool {
        self.power() != 0
    }

    #[doc="Power/Brown Out Event NMI"]
    #[inline] pub fn set_power<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Watch Dog Timer (WDT) 0 NMI"]
    #[inline] pub fn wdt0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Watch Dog Timer (WDT) 0 NMI"]
    #[inline] pub fn test_wdt0(&self) -> bool {
        self.wdt0() != 0
    }

    #[doc="Watch Dog Timer (WDT) 0 NMI"]
    #[inline] pub fn set_wdt0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Watch Dog Timer (WDT) 1 NMI"]
    #[inline] pub fn wdt1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Watch Dog Timer (WDT) 1 NMI"]
    #[inline] pub fn test_wdt1(&self) -> bool {
        self.wdt1() != 0
    }

    #[doc="Watch Dog Timer (WDT) 1 NMI"]
    #[inline] pub fn set_wdt1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Tamper Event NMI"]
    #[inline] pub fn tamper(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Tamper Event NMI"]
    #[inline] pub fn test_tamper(&self) -> bool {
        self.tamper() != 0
    }

    #[doc="Tamper Event NMI"]
    #[inline] pub fn set_tamper<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="MOSC Failure NMI"]
    #[inline] pub fn moscfail(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="MOSC Failure NMI"]
    #[inline] pub fn test_moscfail(&self) -> bool {
        self.moscfail() != 0
    }

    #[doc="MOSC Failure NMI"]
    #[inline] pub fn set_moscfail<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Nmic {
    #[inline]
    fn from(other: u32) -> Self {
         Nmic(other)
    }
}

impl ::core::fmt::Display for Nmic {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Nmic {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.external() != 0 { try!(write!(f, " external"))}
        if self.power() != 0 { try!(write!(f, " power"))}
        if self.wdt0() != 0 { try!(write!(f, " wdt0"))}
        if self.wdt1() != 0 { try!(write!(f, " wdt1"))}
        if self.tamper() != 0 { try!(write!(f, " tamper"))}
        if self.moscfail() != 0 { try!(write!(f, " moscfail"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Main Oscillator Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Moscctl(pub u32);
impl Moscctl {
    #[doc="Clock Validation for MOSC"]
    #[inline] pub fn cval(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Clock Validation for MOSC"]
    #[inline] pub fn test_cval(&self) -> bool {
        self.cval() != 0
    }

    #[doc="Clock Validation for MOSC"]
    #[inline] pub fn set_cval<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="MOSC Failure Action"]
    #[inline] pub fn moscim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="MOSC Failure Action"]
    #[inline] pub fn test_moscim(&self) -> bool {
        self.moscim() != 0
    }

    #[doc="MOSC Failure Action"]
    #[inline] pub fn set_moscim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="No Crystal Connected"]
    #[inline] pub fn noxtal(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="No Crystal Connected"]
    #[inline] pub fn test_noxtal(&self) -> bool {
        self.noxtal() != 0
    }

    #[doc="No Crystal Connected"]
    #[inline] pub fn set_noxtal<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Power Down"]
    #[inline] pub fn pwrdn(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Power Down"]
    #[inline] pub fn test_pwrdn(&self) -> bool {
        self.pwrdn() != 0
    }

    #[doc="Power Down"]
    #[inline] pub fn set_pwrdn<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Oscillator Range"]
    #[inline] pub fn oscrng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Oscillator Range"]
    #[inline] pub fn test_oscrng(&self) -> bool {
        self.oscrng() != 0
    }

    #[doc="Oscillator Range"]
    #[inline] pub fn set_oscrng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Moscctl {
    #[inline]
    fn from(other: u32) -> Self {
         Moscctl(other)
    }
}

impl ::core::fmt::Display for Moscctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Moscctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cval() != 0 { try!(write!(f, " cval"))}
        if self.moscim() != 0 { try!(write!(f, " moscim"))}
        if self.noxtal() != 0 { try!(write!(f, " noxtal"))}
        if self.pwrdn() != 0 { try!(write!(f, " pwrdn"))}
        if self.oscrng() != 0 { try!(write!(f, " oscrng"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Run and Sleep Mode Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rsclkcfg(pub u32);
impl Rsclkcfg {
    #[doc="PLL System Clock Divisor"]
    #[inline] pub fn psysdiv(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="PLL System Clock Divisor"]
    #[inline] pub fn test_psysdiv(&self) -> bool {
        self.psysdiv() != 0
    }

    #[doc="PLL System Clock Divisor"]
    #[inline] pub fn set_psysdiv<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Oscillator System Clock Divisor"]
    #[inline] pub fn osysdiv(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3ff) as u16) } // [19:10]
    }

    #[doc="Oscillator System Clock Divisor"]
    #[inline] pub fn test_osysdiv(&self) -> bool {
        self.osysdiv() != 0
    }

    #[doc="Oscillator System Clock Divisor"]
    #[inline] pub fn set_osysdiv<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Oscillator Source"]
    #[inline] pub fn oscsrc(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Oscillator Source"]
    #[inline] pub fn test_oscsrc(&self) -> bool {
        self.oscsrc() != 0
    }

    #[doc="Oscillator Source"]
    #[inline] pub fn set_oscsrc<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="PLL Source"]
    #[inline] pub fn pllsrc(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="PLL Source"]
    #[inline] pub fn test_pllsrc(&self) -> bool {
        self.pllsrc() != 0
    }

    #[doc="PLL Source"]
    #[inline] pub fn set_pllsrc<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Use PLL"]
    #[inline] pub fn usepll(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Use PLL"]
    #[inline] pub fn test_usepll(&self) -> bool {
        self.usepll() != 0
    }

    #[doc="Use PLL"]
    #[inline] pub fn set_usepll<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Auto Clock Gating"]
    #[inline] pub fn acg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Auto Clock Gating"]
    #[inline] pub fn test_acg(&self) -> bool {
        self.acg() != 0
    }

    #[doc="Auto Clock Gating"]
    #[inline] pub fn set_acg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="New PLLFREQ Accept"]
    #[inline] pub fn newfreq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="New PLLFREQ Accept"]
    #[inline] pub fn test_newfreq(&self) -> bool {
        self.newfreq() != 0
    }

    #[doc="New PLLFREQ Accept"]
    #[inline] pub fn set_newfreq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Memory Timing Register Update"]
    #[inline] pub fn memtimu(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Memory Timing Register Update"]
    #[inline] pub fn test_memtimu(&self) -> bool {
        self.memtimu() != 0
    }

    #[doc="Memory Timing Register Update"]
    #[inline] pub fn set_memtimu<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Rsclkcfg {
    #[inline]
    fn from(other: u32) -> Self {
         Rsclkcfg(other)
    }
}

impl ::core::fmt::Display for Rsclkcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rsclkcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.psysdiv() != 0 { try!(write!(f, " psysdiv=0x{:x}", self.psysdiv()))}
        if self.osysdiv() != 0 { try!(write!(f, " osysdiv=0x{:x}", self.osysdiv()))}
        if self.oscsrc() != 0 { try!(write!(f, " oscsrc=0x{:x}", self.oscsrc()))}
        if self.pllsrc() != 0 { try!(write!(f, " pllsrc=0x{:x}", self.pllsrc()))}
        if self.usepll() != 0 { try!(write!(f, " usepll"))}
        if self.acg() != 0 { try!(write!(f, " acg"))}
        if self.newfreq() != 0 { try!(write!(f, " newfreq"))}
        if self.memtimu() != 0 { try!(write!(f, " memtimu"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Memory Timing Parameter Register 0 for Main Flash and EEPROM"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Memtim0(pub u32);
impl Memtim0 {
    #[doc="Flash Wait State"]
    #[inline] pub fn fws(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Flash Wait State"]
    #[inline] pub fn test_fws(&self) -> bool {
        self.fws() != 0
    }

    #[doc="Flash Wait State"]
    #[inline] pub fn set_fws<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Flash Bank Clock Edge"]
    #[inline] pub fn fbce(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Flash Bank Clock Edge"]
    #[inline] pub fn test_fbce(&self) -> bool {
        self.fbce() != 0
    }

    #[doc="Flash Bank Clock Edge"]
    #[inline] pub fn set_fbce<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Flash Bank Clock High Time"]
    #[inline] pub fn fbcht(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0xf) as u8) } // [9:6]
    }

    #[doc="Flash Bank Clock High Time"]
    #[inline] pub fn test_fbcht(&self) -> bool {
        self.fbcht() != 0
    }

    #[doc="Flash Bank Clock High Time"]
    #[inline] pub fn set_fbcht<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="EEPROM Wait States"]
    #[inline] pub fn ews(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="EEPROM Wait States"]
    #[inline] pub fn test_ews(&self) -> bool {
        self.ews() != 0
    }

    #[doc="EEPROM Wait States"]
    #[inline] pub fn set_ews<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="EEPROM Bank Clock Edge"]
    #[inline] pub fn ebce(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="EEPROM Bank Clock Edge"]
    #[inline] pub fn test_ebce(&self) -> bool {
        self.ebce() != 0
    }

    #[doc="EEPROM Bank Clock Edge"]
    #[inline] pub fn set_ebce<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="EEPROM Clock High Time"]
    #[inline] pub fn ebcht(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0xf) as u8) } // [25:22]
    }

    #[doc="EEPROM Clock High Time"]
    #[inline] pub fn test_ebcht(&self) -> bool {
        self.ebcht() != 0
    }

    #[doc="EEPROM Clock High Time"]
    #[inline] pub fn set_ebcht<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 22);
        self.0 |= value << 22;
        self
    }

}

impl From<u32> for Memtim0 {
    #[inline]
    fn from(other: u32) -> Self {
         Memtim0(other)
    }
}

impl ::core::fmt::Display for Memtim0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Memtim0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fws() != 0 { try!(write!(f, " fws=0x{:x}", self.fws()))}
        if self.fbce() != 0 { try!(write!(f, " fbce"))}
        if self.fbcht() != 0 { try!(write!(f, " fbcht=0x{:x}", self.fbcht()))}
        if self.ews() != 0 { try!(write!(f, " ews=0x{:x}", self.ews()))}
        if self.ebce() != 0 { try!(write!(f, " ebce"))}
        if self.ebcht() != 0 { try!(write!(f, " ebcht=0x{:x}", self.ebcht()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Clock Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altclkcfg(pub u32);
impl Altclkcfg {
    #[doc="Alternate Clock Source"]
    #[inline] pub fn altclk(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Alternate Clock Source"]
    #[inline] pub fn test_altclk(&self) -> bool {
        self.altclk() != 0
    }

    #[doc="Alternate Clock Source"]
    #[inline] pub fn set_altclk<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altclkcfg {
    #[inline]
    fn from(other: u32) -> Self {
         Altclkcfg(other)
    }
}

impl ::core::fmt::Display for Altclkcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altclkcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.altclk() != 0 { try!(write!(f, " altclk=0x{:x}", self.altclk()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Deep Sleep Clock Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dsclkcfg(pub u32);
impl Dsclkcfg {
    #[doc="Deep Sleep Clock Divisor"]
    #[inline] pub fn dssysdiv(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Deep Sleep Clock Divisor"]
    #[inline] pub fn test_dssysdiv(&self) -> bool {
        self.dssysdiv() != 0
    }

    #[doc="Deep Sleep Clock Divisor"]
    #[inline] pub fn set_dssysdiv<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Deep Sleep Oscillator Source"]
    #[inline] pub fn dsoscsrc(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Deep Sleep Oscillator Source"]
    #[inline] pub fn test_dsoscsrc(&self) -> bool {
        self.dsoscsrc() != 0
    }

    #[doc="Deep Sleep Oscillator Source"]
    #[inline] pub fn set_dsoscsrc<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="MOSC Disable Power Down"]
    #[inline] pub fn moscdpd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="MOSC Disable Power Down"]
    #[inline] pub fn test_moscdpd(&self) -> bool {
        self.moscdpd() != 0
    }

    #[doc="MOSC Disable Power Down"]
    #[inline] pub fn set_moscdpd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="PIOSC Power Down"]
    #[inline] pub fn pioscpd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="PIOSC Power Down"]
    #[inline] pub fn test_pioscpd(&self) -> bool {
        self.pioscpd() != 0
    }

    #[doc="PIOSC Power Down"]
    #[inline] pub fn set_pioscpd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Dsclkcfg {
    #[inline]
    fn from(other: u32) -> Self {
         Dsclkcfg(other)
    }
}

impl ::core::fmt::Display for Dsclkcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dsclkcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dssysdiv() != 0 { try!(write!(f, " dssysdiv=0x{:x}", self.dssysdiv()))}
        if self.dsoscsrc() != 0 { try!(write!(f, " dsoscsrc=0x{:x}", self.dsoscsrc()))}
        if self.moscdpd() != 0 { try!(write!(f, " moscdpd"))}
        if self.pioscpd() != 0 { try!(write!(f, " pioscpd"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Divisor and Source Clock Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Divsclk(pub u32);
impl Divsclk {
    #[doc="Divisor Value"]
    #[inline] pub fn div(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Divisor Value"]
    #[inline] pub fn test_div(&self) -> bool {
        self.div() != 0
    }

    #[doc="Divisor Value"]
    #[inline] pub fn set_div<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clock Source"]
    #[inline] pub fn src(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Clock Source"]
    #[inline] pub fn test_src(&self) -> bool {
        self.src() != 0
    }

    #[doc="Clock Source"]
    #[inline] pub fn set_src<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DIVSCLK Enable"]
    #[inline] pub fn en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="DIVSCLK Enable"]
    #[inline] pub fn test_en(&self) -> bool {
        self.en() != 0
    }

    #[doc="DIVSCLK Enable"]
    #[inline] pub fn set_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Divsclk {
    #[inline]
    fn from(other: u32) -> Self {
         Divsclk(other)
    }
}

impl ::core::fmt::Display for Divsclk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Divsclk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.div() != 0 { try!(write!(f, " div=0x{:x}", self.div()))}
        if self.src() != 0 { try!(write!(f, " src=0x{:x}", self.src()))}
        if self.en() != 0 { try!(write!(f, " en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Properties"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sysprop(pub u32);
impl Sysprop {
    #[doc="FPU Present"]
    #[inline] pub fn fpu(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="FPU Present"]
    #[inline] pub fn test_fpu(&self) -> bool {
        self.fpu() != 0
    }

    #[doc="FPU Present"]
    #[inline] pub fn set_fpu<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sysprop {
    #[inline]
    fn from(other: u32) -> Self {
         Sysprop(other)
    }
}

impl ::core::fmt::Display for Sysprop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sysprop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fpu() != 0 { try!(write!(f, " fpu"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Precision Internal Oscillator Calibration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Piosccal(pub u32);
impl Piosccal {
    #[doc="User Trim Value"]
    #[inline] pub fn ut(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="User Trim Value"]
    #[inline] pub fn test_ut(&self) -> bool {
        self.ut() != 0
    }

    #[doc="User Trim Value"]
    #[inline] pub fn set_ut<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Update Trim"]
    #[inline] pub fn update(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Update Trim"]
    #[inline] pub fn test_update(&self) -> bool {
        self.update() != 0
    }

    #[doc="Update Trim"]
    #[inline] pub fn set_update<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Start Calibration"]
    #[inline] pub fn cal(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Start Calibration"]
    #[inline] pub fn test_cal(&self) -> bool {
        self.cal() != 0
    }

    #[doc="Start Calibration"]
    #[inline] pub fn set_cal<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Use User Trim Value"]
    #[inline] pub fn uten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Use User Trim Value"]
    #[inline] pub fn test_uten(&self) -> bool {
        self.uten() != 0
    }

    #[doc="Use User Trim Value"]
    #[inline] pub fn set_uten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Piosccal {
    #[inline]
    fn from(other: u32) -> Self {
         Piosccal(other)
    }
}

impl ::core::fmt::Display for Piosccal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Piosccal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ut() != 0 { try!(write!(f, " ut=0x{:x}", self.ut()))}
        if self.update() != 0 { try!(write!(f, " update"))}
        if self.cal() != 0 { try!(write!(f, " cal"))}
        if self.uten() != 0 { try!(write!(f, " uten"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Precision Internal Oscillator Statistics"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pioscstat(pub u32);
impl Pioscstat {
    #[doc="Calibration Trim Value"]
    #[inline] pub fn ct(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Calibration Trim Value"]
    #[inline] pub fn test_ct(&self) -> bool {
        self.ct() != 0
    }

    #[doc="Calibration Trim Value"]
    #[inline] pub fn set_ct<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Calibration Result"]
    #[inline] pub fn cr(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Calibration Result"]
    #[inline] pub fn test_cr(&self) -> bool {
        self.cr() != 0
    }

    #[doc="Calibration Result"]
    #[inline] pub fn set_cr<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Default Trim Value"]
    #[inline] pub fn dt(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7f) as u8) } // [22:16]
    }

    #[doc="Default Trim Value"]
    #[inline] pub fn test_dt(&self) -> bool {
        self.dt() != 0
    }

    #[doc="Default Trim Value"]
    #[inline] pub fn set_dt<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Pioscstat {
    #[inline]
    fn from(other: u32) -> Self {
         Pioscstat(other)
    }
}

impl ::core::fmt::Display for Pioscstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pioscstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ct() != 0 { try!(write!(f, " ct=0x{:x}", self.ct()))}
        if self.cr() != 0 { try!(write!(f, " cr=0x{:x}", self.cr()))}
        if self.dt() != 0 { try!(write!(f, " dt=0x{:x}", self.dt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PLL Frequency 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pllfreq0(pub u32);
impl Pllfreq0 {
    #[doc="PLL M Integer Value"]
    #[inline] pub fn mint(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="PLL M Integer Value"]
    #[inline] pub fn test_mint(&self) -> bool {
        self.mint() != 0
    }

    #[doc="PLL M Integer Value"]
    #[inline] pub fn set_mint<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="PLL M Fractional Value"]
    #[inline] pub fn mfrac(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3ff) as u16) } // [19:10]
    }

    #[doc="PLL M Fractional Value"]
    #[inline] pub fn test_mfrac(&self) -> bool {
        self.mfrac() != 0
    }

    #[doc="PLL M Fractional Value"]
    #[inline] pub fn set_mfrac<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="PLL Power"]
    #[inline] pub fn pllpwr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="PLL Power"]
    #[inline] pub fn test_pllpwr(&self) -> bool {
        self.pllpwr() != 0
    }

    #[doc="PLL Power"]
    #[inline] pub fn set_pllpwr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

}

impl From<u32> for Pllfreq0 {
    #[inline]
    fn from(other: u32) -> Self {
         Pllfreq0(other)
    }
}

impl ::core::fmt::Display for Pllfreq0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pllfreq0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mint() != 0 { try!(write!(f, " mint=0x{:x}", self.mint()))}
        if self.mfrac() != 0 { try!(write!(f, " mfrac=0x{:x}", self.mfrac()))}
        if self.pllpwr() != 0 { try!(write!(f, " pllpwr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PLL Frequency 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pllfreq1(pub u32);
impl Pllfreq1 {
    #[doc="PLL N Value"]
    #[inline] pub fn n(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="PLL N Value"]
    #[inline] pub fn test_n(&self) -> bool {
        self.n() != 0
    }

    #[doc="PLL N Value"]
    #[inline] pub fn set_n<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="PLL Q Value"]
    #[inline] pub fn q(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1f) as u8) } // [12:8]
    }

    #[doc="PLL Q Value"]
    #[inline] pub fn test_q(&self) -> bool {
        self.q() != 0
    }

    #[doc="PLL Q Value"]
    #[inline] pub fn set_q<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Pllfreq1 {
    #[inline]
    fn from(other: u32) -> Self {
         Pllfreq1(other)
    }
}

impl ::core::fmt::Display for Pllfreq1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pllfreq1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.n() != 0 { try!(write!(f, " n=0x{:x}", self.n()))}
        if self.q() != 0 { try!(write!(f, " q=0x{:x}", self.q()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PLL Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pllstat(pub u32);
impl Pllstat {
    #[doc="PLL Lock"]
    #[inline] pub fn lock(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="PLL Lock"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="PLL Lock"]
    #[inline] pub fn set_lock<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pllstat {
    #[inline]
    fn from(other: u32) -> Self {
         Pllstat(other)
    }
}

impl ::core::fmt::Display for Pllstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pllstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Sleep Power Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Slppwrcfg(pub u32);
impl Slppwrcfg {
    #[doc="SRAM Power Modes"]
    #[inline] pub fn srampm(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="SRAM Power Modes"]
    #[inline] pub fn test_srampm(&self) -> bool {
        self.srampm() != 0
    }

    #[doc="SRAM Power Modes"]
    #[inline] pub fn set_srampm<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Flash Power Modes"]
    #[inline] pub fn flashpm(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Flash Power Modes"]
    #[inline] pub fn test_flashpm(&self) -> bool {
        self.flashpm() != 0
    }

    #[doc="Flash Power Modes"]
    #[inline] pub fn set_flashpm<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Slppwrcfg {
    #[inline]
    fn from(other: u32) -> Self {
         Slppwrcfg(other)
    }
}

impl ::core::fmt::Display for Slppwrcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Slppwrcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.srampm() != 0 { try!(write!(f, " srampm=0x{:x}", self.srampm()))}
        if self.flashpm() != 0 { try!(write!(f, " flashpm=0x{:x}", self.flashpm()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Deep-Sleep Power Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dslppwrcfg(pub u32);
impl Dslppwrcfg {
    #[doc="SRAM Power Modes"]
    #[inline] pub fn srampm(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="SRAM Power Modes"]
    #[inline] pub fn test_srampm(&self) -> bool {
        self.srampm() != 0
    }

    #[doc="SRAM Power Modes"]
    #[inline] pub fn set_srampm<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Flash Power Modes"]
    #[inline] pub fn flashpm(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Flash Power Modes"]
    #[inline] pub fn test_flashpm(&self) -> bool {
        self.flashpm() != 0
    }

    #[doc="Flash Power Modes"]
    #[inline] pub fn set_flashpm<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Temperature Sense Power Down"]
    #[inline] pub fn tspd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Temperature Sense Power Down"]
    #[inline] pub fn test_tspd(&self) -> bool {
        self.tspd() != 0
    }

    #[doc="Temperature Sense Power Down"]
    #[inline] pub fn set_tspd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="LDO Sleep Mode"]
    #[inline] pub fn ldosm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="LDO Sleep Mode"]
    #[inline] pub fn test_ldosm(&self) -> bool {
        self.ldosm() != 0
    }

    #[doc="LDO Sleep Mode"]
    #[inline] pub fn set_ldosm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

}

impl From<u32> for Dslppwrcfg {
    #[inline]
    fn from(other: u32) -> Self {
         Dslppwrcfg(other)
    }
}

impl ::core::fmt::Display for Dslppwrcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dslppwrcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.srampm() != 0 { try!(write!(f, " srampm=0x{:x}", self.srampm()))}
        if self.flashpm() != 0 { try!(write!(f, " flashpm=0x{:x}", self.flashpm()))}
        if self.tspd() != 0 { try!(write!(f, " tspd"))}
        if self.ldosm() != 0 { try!(write!(f, " ldosm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Non-Volatile Memory Information"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Nvmstat(pub u32);
impl Nvmstat {
    #[doc="32 Word Flash Write Buffer Available"]
    #[inline] pub fn fwb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="32 Word Flash Write Buffer Available"]
    #[inline] pub fn test_fwb(&self) -> bool {
        self.fwb() != 0
    }

    #[doc="32 Word Flash Write Buffer Available"]
    #[inline] pub fn set_fwb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Nvmstat {
    #[inline]
    fn from(other: u32) -> Self {
         Nvmstat(other)
    }
}

impl ::core::fmt::Display for Nvmstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Nvmstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fwb() != 0 { try!(write!(f, " fwb"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LDO Sleep Power Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ldospctl(pub u32);
impl Ldospctl {
    #[doc="LDO Output Voltage"]
    #[inline] pub fn vldo(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="LDO Output Voltage"]
    #[inline] pub fn test_vldo(&self) -> bool {
        self.vldo() != 0
    }

    #[doc="LDO Output Voltage"]
    #[inline] pub fn set_vldo<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Voltage Adjust Enable"]
    #[inline] pub fn vadjen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Voltage Adjust Enable"]
    #[inline] pub fn test_vadjen(&self) -> bool {
        self.vadjen() != 0
    }

    #[doc="Voltage Adjust Enable"]
    #[inline] pub fn set_vadjen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Ldospctl {
    #[inline]
    fn from(other: u32) -> Self {
         Ldospctl(other)
    }
}

impl ::core::fmt::Display for Ldospctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ldospctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.vldo() != 0 { try!(write!(f, " vldo=0x{:x}", self.vldo()))}
        if self.vadjen() != 0 { try!(write!(f, " vadjen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LDO Deep-Sleep Power Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ldodpctl(pub u32);
impl Ldodpctl {
    #[doc="LDO Output Voltage"]
    #[inline] pub fn vldo(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="LDO Output Voltage"]
    #[inline] pub fn test_vldo(&self) -> bool {
        self.vldo() != 0
    }

    #[doc="LDO Output Voltage"]
    #[inline] pub fn set_vldo<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Voltage Adjust Enable"]
    #[inline] pub fn vadjen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Voltage Adjust Enable"]
    #[inline] pub fn test_vadjen(&self) -> bool {
        self.vadjen() != 0
    }

    #[doc="Voltage Adjust Enable"]
    #[inline] pub fn set_vadjen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Ldodpctl {
    #[inline]
    fn from(other: u32) -> Self {
         Ldodpctl(other)
    }
}

impl ::core::fmt::Display for Ldodpctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ldodpctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.vldo() != 0 { try!(write!(f, " vldo=0x{:x}", self.vldo()))}
        if self.vadjen() != 0 { try!(write!(f, " vadjen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reset Behavior Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Resbehavctl(pub u32);
impl Resbehavctl {
    #[doc="External RST Pin Operation"]
    #[inline] pub fn extres(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="External RST Pin Operation"]
    #[inline] pub fn test_extres(&self) -> bool {
        self.extres() != 0
    }

    #[doc="External RST Pin Operation"]
    #[inline] pub fn set_extres<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="BOR Reset operation"]
    #[inline] pub fn bor(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="BOR Reset operation"]
    #[inline] pub fn test_bor(&self) -> bool {
        self.bor() != 0
    }

    #[doc="BOR Reset operation"]
    #[inline] pub fn set_bor<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Watchdog 0 Reset Operation"]
    #[inline] pub fn wdog0(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Watchdog 0 Reset Operation"]
    #[inline] pub fn test_wdog0(&self) -> bool {
        self.wdog0() != 0
    }

    #[doc="Watchdog 0 Reset Operation"]
    #[inline] pub fn set_wdog0<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Watchdog 1 Reset Operation"]
    #[inline] pub fn wdog1(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Watchdog 1 Reset Operation"]
    #[inline] pub fn test_wdog1(&self) -> bool {
        self.wdog1() != 0
    }

    #[doc="Watchdog 1 Reset Operation"]
    #[inline] pub fn set_wdog1<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for Resbehavctl {
    #[inline]
    fn from(other: u32) -> Self {
         Resbehavctl(other)
    }
}

impl ::core::fmt::Display for Resbehavctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Resbehavctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.extres() != 0 { try!(write!(f, " extres=0x{:x}", self.extres()))}
        if self.bor() != 0 { try!(write!(f, " bor=0x{:x}", self.bor()))}
        if self.wdog0() != 0 { try!(write!(f, " wdog0=0x{:x}", self.wdog0()))}
        if self.wdog1() != 0 { try!(write!(f, " wdog1=0x{:x}", self.wdog1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Hardware System Service Request"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hssr(pub u32);
impl Hssr {
    #[doc="Command Descriptor Pointer"]
    #[inline] pub fn cdoff(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
    }

    #[doc="Command Descriptor Pointer"]
    #[inline] pub fn test_cdoff(&self) -> bool {
        self.cdoff() != 0
    }

    #[doc="Command Descriptor Pointer"]
    #[inline] pub fn set_cdoff<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Write Key"]
    #[inline] pub fn key(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Write Key"]
    #[inline] pub fn test_key(&self) -> bool {
        self.key() != 0
    }

    #[doc="Write Key"]
    #[inline] pub fn set_key<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Hssr {
    #[inline]
    fn from(other: u32) -> Self {
         Hssr(other)
    }
}

impl ::core::fmt::Display for Hssr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hssr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cdoff() != 0 { try!(write!(f, " cdoff=0x{:x}", self.cdoff()))}
        if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USB Power Domain Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Usbpds(pub u32);
impl Usbpds {
    #[doc="Power Domain Status"]
    #[inline] pub fn pwrstat(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Power Domain Status"]
    #[inline] pub fn test_pwrstat(&self) -> bool {
        self.pwrstat() != 0
    }

    #[doc="Power Domain Status"]
    #[inline] pub fn set_pwrstat<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Memory Array Power Status"]
    #[inline] pub fn memstat(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Memory Array Power Status"]
    #[inline] pub fn test_memstat(&self) -> bool {
        self.memstat() != 0
    }

    #[doc="Memory Array Power Status"]
    #[inline] pub fn set_memstat<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u32> for Usbpds {
    #[inline]
    fn from(other: u32) -> Self {
         Usbpds(other)
    }
}

impl ::core::fmt::Display for Usbpds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Usbpds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pwrstat() != 0 { try!(write!(f, " pwrstat=0x{:x}", self.pwrstat()))}
        if self.memstat() != 0 { try!(write!(f, " memstat=0x{:x}", self.memstat()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USB Memory Power Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Usbmpc(pub u32);
impl Usbmpc {
    #[doc="Memory Array Power Control"]
    #[inline] pub fn pwrctl(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Memory Array Power Control"]
    #[inline] pub fn test_pwrctl(&self) -> bool {
        self.pwrctl() != 0
    }

    #[doc="Memory Array Power Control"]
    #[inline] pub fn set_pwrctl<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Usbmpc {
    #[inline]
    fn from(other: u32) -> Self {
         Usbmpc(other)
    }
}

impl ::core::fmt::Display for Usbmpc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Usbmpc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pwrctl() != 0 { try!(write!(f, " pwrctl=0x{:x}", self.pwrctl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Power Domain Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Emacpds(pub u32);
impl Emacpds {
    #[doc="Power Domain Status"]
    #[inline] pub fn pwrstat(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Power Domain Status"]
    #[inline] pub fn test_pwrstat(&self) -> bool {
        self.pwrstat() != 0
    }

    #[doc="Power Domain Status"]
    #[inline] pub fn set_pwrstat<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Memory Array Power Status"]
    #[inline] pub fn memstat(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Memory Array Power Status"]
    #[inline] pub fn test_memstat(&self) -> bool {
        self.memstat() != 0
    }

    #[doc="Memory Array Power Status"]
    #[inline] pub fn set_memstat<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u32> for Emacpds {
    #[inline]
    fn from(other: u32) -> Self {
         Emacpds(other)
    }
}

impl ::core::fmt::Display for Emacpds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Emacpds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pwrstat() != 0 { try!(write!(f, " pwrstat=0x{:x}", self.pwrstat()))}
        if self.memstat() != 0 { try!(write!(f, " memstat=0x{:x}", self.memstat()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Memory Power Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Emacmpc(pub u32);
impl Emacmpc {
    #[doc="Memory Array Power Control"]
    #[inline] pub fn pwrctl(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Memory Array Power Control"]
    #[inline] pub fn test_pwrctl(&self) -> bool {
        self.pwrctl() != 0
    }

    #[doc="Memory Array Power Control"]
    #[inline] pub fn set_pwrctl<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Emacmpc {
    #[inline]
    fn from(other: u32) -> Self {
         Emacmpc(other)
    }
}

impl ::core::fmt::Display for Emacmpc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Emacmpc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pwrctl() != 0 { try!(write!(f, " pwrctl=0x{:x}", self.pwrctl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Timer Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ppwd(pub u32);
impl Ppwd {
    #[doc="Watchdog Timer 0 Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Watchdog Timer 0 Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="Watchdog Timer 0 Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Watchdog Timer 1 Present"]
    #[inline] pub fn p1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Watchdog Timer 1 Present"]
    #[inline] pub fn test_p1(&self) -> bool {
        self.p1() != 0
    }

    #[doc="Watchdog Timer 1 Present"]
    #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Ppwd {
    #[inline]
    fn from(other: u32) -> Self {
         Ppwd(other)
    }
}

impl ::core::fmt::Display for Ppwd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ppwd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        if self.p1() != 0 { try!(write!(f, " p1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="16/32-Bit General-Purpose Timer Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pptimer(pub u32);
impl Pptimer {
    #[doc="16/32-Bit General-Purpose Timer 0 Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="16/32-Bit General-Purpose Timer 0 Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 0 Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 1 Present"]
    #[inline] pub fn p1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="16/32-Bit General-Purpose Timer 1 Present"]
    #[inline] pub fn test_p1(&self) -> bool {
        self.p1() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 1 Present"]
    #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 2 Present"]
    #[inline] pub fn p2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="16/32-Bit General-Purpose Timer 2 Present"]
    #[inline] pub fn test_p2(&self) -> bool {
        self.p2() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 2 Present"]
    #[inline] pub fn set_p2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 3 Present"]
    #[inline] pub fn p3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="16/32-Bit General-Purpose Timer 3 Present"]
    #[inline] pub fn test_p3(&self) -> bool {
        self.p3() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 3 Present"]
    #[inline] pub fn set_p3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 4 Present"]
    #[inline] pub fn p4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="16/32-Bit General-Purpose Timer 4 Present"]
    #[inline] pub fn test_p4(&self) -> bool {
        self.p4() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 4 Present"]
    #[inline] pub fn set_p4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 5 Present"]
    #[inline] pub fn p5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="16/32-Bit General-Purpose Timer 5 Present"]
    #[inline] pub fn test_p5(&self) -> bool {
        self.p5() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 5 Present"]
    #[inline] pub fn set_p5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 6 Present"]
    #[inline] pub fn p6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="16/32-Bit General-Purpose Timer 6 Present"]
    #[inline] pub fn test_p6(&self) -> bool {
        self.p6() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 6 Present"]
    #[inline] pub fn set_p6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 7 Present"]
    #[inline] pub fn p7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="16/32-Bit General-Purpose Timer 7 Present"]
    #[inline] pub fn test_p7(&self) -> bool {
        self.p7() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 7 Present"]
    #[inline] pub fn set_p7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Pptimer {
    #[inline]
    fn from(other: u32) -> Self {
         Pptimer(other)
    }
}

impl ::core::fmt::Display for Pptimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pptimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        if self.p1() != 0 { try!(write!(f, " p1"))}
        if self.p2() != 0 { try!(write!(f, " p2"))}
        if self.p3() != 0 { try!(write!(f, " p3"))}
        if self.p4() != 0 { try!(write!(f, " p4"))}
        if self.p5() != 0 { try!(write!(f, " p5"))}
        if self.p6() != 0 { try!(write!(f, " p6"))}
        if self.p7() != 0 { try!(write!(f, " p7"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="General-Purpose Input/Output Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ppgpio(pub u32);
impl Ppgpio {
    #[doc="GPIO Port A Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="GPIO Port A Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="GPIO Port A Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="GPIO Port B Present"]
    #[inline] pub fn p1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="GPIO Port B Present"]
    #[inline] pub fn test_p1(&self) -> bool {
        self.p1() != 0
    }

    #[doc="GPIO Port B Present"]
    #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="GPIO Port C Present"]
    #[inline] pub fn p2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="GPIO Port C Present"]
    #[inline] pub fn test_p2(&self) -> bool {
        self.p2() != 0
    }

    #[doc="GPIO Port C Present"]
    #[inline] pub fn set_p2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="GPIO Port D Present"]
    #[inline] pub fn p3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="GPIO Port D Present"]
    #[inline] pub fn test_p3(&self) -> bool {
        self.p3() != 0
    }

    #[doc="GPIO Port D Present"]
    #[inline] pub fn set_p3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPIO Port E Present"]
    #[inline] pub fn p4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="GPIO Port E Present"]
    #[inline] pub fn test_p4(&self) -> bool {
        self.p4() != 0
    }

    #[doc="GPIO Port E Present"]
    #[inline] pub fn set_p4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPIO Port F Present"]
    #[inline] pub fn p5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="GPIO Port F Present"]
    #[inline] pub fn test_p5(&self) -> bool {
        self.p5() != 0
    }

    #[doc="GPIO Port F Present"]
    #[inline] pub fn set_p5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="GPIO Port G Present"]
    #[inline] pub fn p6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="GPIO Port G Present"]
    #[inline] pub fn test_p6(&self) -> bool {
        self.p6() != 0
    }

    #[doc="GPIO Port G Present"]
    #[inline] pub fn set_p6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="GPIO Port H Present"]
    #[inline] pub fn p7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="GPIO Port H Present"]
    #[inline] pub fn test_p7(&self) -> bool {
        self.p7() != 0
    }

    #[doc="GPIO Port H Present"]
    #[inline] pub fn set_p7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="GPIO Port J Present"]
    #[inline] pub fn p8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="GPIO Port J Present"]
    #[inline] pub fn test_p8(&self) -> bool {
        self.p8() != 0
    }

    #[doc="GPIO Port J Present"]
    #[inline] pub fn set_p8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="GPIO Port K Present"]
    #[inline] pub fn p9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="GPIO Port K Present"]
    #[inline] pub fn test_p9(&self) -> bool {
        self.p9() != 0
    }

    #[doc="GPIO Port K Present"]
    #[inline] pub fn set_p9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="GPIO Port L Present"]
    #[inline] pub fn p10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="GPIO Port L Present"]
    #[inline] pub fn test_p10(&self) -> bool {
        self.p10() != 0
    }

    #[doc="GPIO Port L Present"]
    #[inline] pub fn set_p10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="GPIO Port M Present"]
    #[inline] pub fn p11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="GPIO Port M Present"]
    #[inline] pub fn test_p11(&self) -> bool {
        self.p11() != 0
    }

    #[doc="GPIO Port M Present"]
    #[inline] pub fn set_p11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="GPIO Port N Present"]
    #[inline] pub fn p12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="GPIO Port N Present"]
    #[inline] pub fn test_p12(&self) -> bool {
        self.p12() != 0
    }

    #[doc="GPIO Port N Present"]
    #[inline] pub fn set_p12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="GPIO Port P Present"]
    #[inline] pub fn p13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="GPIO Port P Present"]
    #[inline] pub fn test_p13(&self) -> bool {
        self.p13() != 0
    }

    #[doc="GPIO Port P Present"]
    #[inline] pub fn set_p13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="GPIO Port Q Present"]
    #[inline] pub fn p14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="GPIO Port Q Present"]
    #[inline] pub fn test_p14(&self) -> bool {
        self.p14() != 0
    }

    #[doc="GPIO Port Q Present"]
    #[inline] pub fn set_p14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

}

impl From<u32> for Ppgpio {
    #[inline]
    fn from(other: u32) -> Self {
         Ppgpio(other)
    }
}

impl ::core::fmt::Display for Ppgpio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ppgpio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        if self.p1() != 0 { try!(write!(f, " p1"))}
        if self.p2() != 0 { try!(write!(f, " p2"))}
        if self.p3() != 0 { try!(write!(f, " p3"))}
        if self.p4() != 0 { try!(write!(f, " p4"))}
        if self.p5() != 0 { try!(write!(f, " p5"))}
        if self.p6() != 0 { try!(write!(f, " p6"))}
        if self.p7() != 0 { try!(write!(f, " p7"))}
        if self.p8() != 0 { try!(write!(f, " p8"))}
        if self.p9() != 0 { try!(write!(f, " p9"))}
        if self.p10() != 0 { try!(write!(f, " p10"))}
        if self.p11() != 0 { try!(write!(f, " p11"))}
        if self.p12() != 0 { try!(write!(f, " p12"))}
        if self.p13() != 0 { try!(write!(f, " p13"))}
        if self.p14() != 0 { try!(write!(f, " p14"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Micro Direct Memory Access Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ppdma(pub u32);
impl Ppdma {
    #[doc="uDMA Module Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="uDMA Module Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="uDMA Module Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ppdma {
    #[inline]
    fn from(other: u32) -> Self {
         Ppdma(other)
    }
}

impl ::core::fmt::Display for Ppdma {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ppdma {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="EPI Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ppepi(pub u32);
impl Ppepi {
    #[doc="EPI Module Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="EPI Module Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="EPI Module Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ppepi {
    #[inline]
    fn from(other: u32) -> Self {
         Ppepi(other)
    }
}

impl ::core::fmt::Display for Ppepi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ppepi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Hibernation Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pphib(pub u32);
impl Pphib {
    #[doc="Hibernation Module Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Hibernation Module Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="Hibernation Module Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pphib {
    #[inline]
    fn from(other: u32) -> Self {
         Pphib(other)
    }
}

impl ::core::fmt::Display for Pphib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pphib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Universal Asynchronous Receiver/Transmitter Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ppuart(pub u32);
impl Ppuart {
    #[doc="UART Module 0 Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="UART Module 0 Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="UART Module 0 Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="UART Module 1 Present"]
    #[inline] pub fn p1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="UART Module 1 Present"]
    #[inline] pub fn test_p1(&self) -> bool {
        self.p1() != 0
    }

    #[doc="UART Module 1 Present"]
    #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="UART Module 2 Present"]
    #[inline] pub fn p2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="UART Module 2 Present"]
    #[inline] pub fn test_p2(&self) -> bool {
        self.p2() != 0
    }

    #[doc="UART Module 2 Present"]
    #[inline] pub fn set_p2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="UART Module 3 Present"]
    #[inline] pub fn p3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="UART Module 3 Present"]
    #[inline] pub fn test_p3(&self) -> bool {
        self.p3() != 0
    }

    #[doc="UART Module 3 Present"]
    #[inline] pub fn set_p3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="UART Module 4 Present"]
    #[inline] pub fn p4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="UART Module 4 Present"]
    #[inline] pub fn test_p4(&self) -> bool {
        self.p4() != 0
    }

    #[doc="UART Module 4 Present"]
    #[inline] pub fn set_p4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="UART Module 5 Present"]
    #[inline] pub fn p5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="UART Module 5 Present"]
    #[inline] pub fn test_p5(&self) -> bool {
        self.p5() != 0
    }

    #[doc="UART Module 5 Present"]
    #[inline] pub fn set_p5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="UART Module 6 Present"]
    #[inline] pub fn p6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="UART Module 6 Present"]
    #[inline] pub fn test_p6(&self) -> bool {
        self.p6() != 0
    }

    #[doc="UART Module 6 Present"]
    #[inline] pub fn set_p6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="UART Module 7 Present"]
    #[inline] pub fn p7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="UART Module 7 Present"]
    #[inline] pub fn test_p7(&self) -> bool {
        self.p7() != 0
    }

    #[doc="UART Module 7 Present"]
    #[inline] pub fn set_p7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Ppuart {
    #[inline]
    fn from(other: u32) -> Self {
         Ppuart(other)
    }
}

impl ::core::fmt::Display for Ppuart {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ppuart {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        if self.p1() != 0 { try!(write!(f, " p1"))}
        if self.p2() != 0 { try!(write!(f, " p2"))}
        if self.p3() != 0 { try!(write!(f, " p3"))}
        if self.p4() != 0 { try!(write!(f, " p4"))}
        if self.p5() != 0 { try!(write!(f, " p5"))}
        if self.p6() != 0 { try!(write!(f, " p6"))}
        if self.p7() != 0 { try!(write!(f, " p7"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Synchronous Serial Interface Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ppssi(pub u32);
impl Ppssi {
    #[doc="SSI Module 0 Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="SSI Module 0 Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="SSI Module 0 Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SSI Module 1 Present"]
    #[inline] pub fn p1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="SSI Module 1 Present"]
    #[inline] pub fn test_p1(&self) -> bool {
        self.p1() != 0
    }

    #[doc="SSI Module 1 Present"]
    #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="SSI Module 2 Present"]
    #[inline] pub fn p2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="SSI Module 2 Present"]
    #[inline] pub fn test_p2(&self) -> bool {
        self.p2() != 0
    }

    #[doc="SSI Module 2 Present"]
    #[inline] pub fn set_p2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SSI Module 3 Present"]
    #[inline] pub fn p3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="SSI Module 3 Present"]
    #[inline] pub fn test_p3(&self) -> bool {
        self.p3() != 0
    }

    #[doc="SSI Module 3 Present"]
    #[inline] pub fn set_p3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Ppssi {
    #[inline]
    fn from(other: u32) -> Self {
         Ppssi(other)
    }
}

impl ::core::fmt::Display for Ppssi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ppssi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        if self.p1() != 0 { try!(write!(f, " p1"))}
        if self.p2() != 0 { try!(write!(f, " p2"))}
        if self.p3() != 0 { try!(write!(f, " p3"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Inter-Integrated Circuit Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ppi2c(pub u32);
impl Ppi2c {
    #[doc="I2C Module 0 Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="I2C Module 0 Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="I2C Module 0 Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="I2C Module 1 Present"]
    #[inline] pub fn p1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="I2C Module 1 Present"]
    #[inline] pub fn test_p1(&self) -> bool {
        self.p1() != 0
    }

    #[doc="I2C Module 1 Present"]
    #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="I2C Module 2 Present"]
    #[inline] pub fn p2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="I2C Module 2 Present"]
    #[inline] pub fn test_p2(&self) -> bool {
        self.p2() != 0
    }

    #[doc="I2C Module 2 Present"]
    #[inline] pub fn set_p2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="I2C Module 3 Present"]
    #[inline] pub fn p3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="I2C Module 3 Present"]
    #[inline] pub fn test_p3(&self) -> bool {
        self.p3() != 0
    }

    #[doc="I2C Module 3 Present"]
    #[inline] pub fn set_p3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="I2C Module 4 Present"]
    #[inline] pub fn p4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="I2C Module 4 Present"]
    #[inline] pub fn test_p4(&self) -> bool {
        self.p4() != 0
    }

    #[doc="I2C Module 4 Present"]
    #[inline] pub fn set_p4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="I2C Module 5 Present"]
    #[inline] pub fn p5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="I2C Module 5 Present"]
    #[inline] pub fn test_p5(&self) -> bool {
        self.p5() != 0
    }

    #[doc="I2C Module 5 Present"]
    #[inline] pub fn set_p5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="I2C Module 6 Present"]
    #[inline] pub fn p6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="I2C Module 6 Present"]
    #[inline] pub fn test_p6(&self) -> bool {
        self.p6() != 0
    }

    #[doc="I2C Module 6 Present"]
    #[inline] pub fn set_p6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="I2C Module 7 Present"]
    #[inline] pub fn p7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="I2C Module 7 Present"]
    #[inline] pub fn test_p7(&self) -> bool {
        self.p7() != 0
    }

    #[doc="I2C Module 7 Present"]
    #[inline] pub fn set_p7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="I2C Module 8 Present"]
    #[inline] pub fn p8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="I2C Module 8 Present"]
    #[inline] pub fn test_p8(&self) -> bool {
        self.p8() != 0
    }

    #[doc="I2C Module 8 Present"]
    #[inline] pub fn set_p8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="I2C Module 9 Present"]
    #[inline] pub fn p9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="I2C Module 9 Present"]
    #[inline] pub fn test_p9(&self) -> bool {
        self.p9() != 0
    }

    #[doc="I2C Module 9 Present"]
    #[inline] pub fn set_p9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

}

impl From<u32> for Ppi2c {
    #[inline]
    fn from(other: u32) -> Self {
         Ppi2c(other)
    }
}

impl ::core::fmt::Display for Ppi2c {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ppi2c {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        if self.p1() != 0 { try!(write!(f, " p1"))}
        if self.p2() != 0 { try!(write!(f, " p2"))}
        if self.p3() != 0 { try!(write!(f, " p3"))}
        if self.p4() != 0 { try!(write!(f, " p4"))}
        if self.p5() != 0 { try!(write!(f, " p5"))}
        if self.p6() != 0 { try!(write!(f, " p6"))}
        if self.p7() != 0 { try!(write!(f, " p7"))}
        if self.p8() != 0 { try!(write!(f, " p8"))}
        if self.p9() != 0 { try!(write!(f, " p9"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Universal Serial Bus Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ppusb(pub u32);
impl Ppusb {
    #[doc="USB Module Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="USB Module Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="USB Module Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ppusb {
    #[inline]
    fn from(other: u32) -> Self {
         Ppusb(other)
    }
}

impl ::core::fmt::Display for Ppusb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ppusb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PHY Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ppephy(pub u32);
impl Ppephy {
    #[doc="Ethernet PHY Module Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Ethernet PHY Module Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="Ethernet PHY Module Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ppephy {
    #[inline]
    fn from(other: u32) -> Self {
         Ppephy(other)
    }
}

impl ::core::fmt::Display for Ppephy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ppephy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Controller Area Network Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ppcan(pub u32);
impl Ppcan {
    #[doc="CAN Module 0 Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="CAN Module 0 Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="CAN Module 0 Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="CAN Module 1 Present"]
    #[inline] pub fn p1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="CAN Module 1 Present"]
    #[inline] pub fn test_p1(&self) -> bool {
        self.p1() != 0
    }

    #[doc="CAN Module 1 Present"]
    #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Ppcan {
    #[inline]
    fn from(other: u32) -> Self {
         Ppcan(other)
    }
}

impl ::core::fmt::Display for Ppcan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ppcan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        if self.p1() != 0 { try!(write!(f, " p1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Analog-to-Digital Converter Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ppadc(pub u32);
impl Ppadc {
    #[doc="ADC Module 0 Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="ADC Module 0 Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="ADC Module 0 Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="ADC Module 1 Present"]
    #[inline] pub fn p1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="ADC Module 1 Present"]
    #[inline] pub fn test_p1(&self) -> bool {
        self.p1() != 0
    }

    #[doc="ADC Module 1 Present"]
    #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Ppadc {
    #[inline]
    fn from(other: u32) -> Self {
         Ppadc(other)
    }
}

impl ::core::fmt::Display for Ppadc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ppadc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        if self.p1() != 0 { try!(write!(f, " p1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Analog Comparator Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ppacmp(pub u32);
impl Ppacmp {
    #[doc="Analog Comparator Module Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Analog Comparator Module Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="Analog Comparator Module Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ppacmp {
    #[inline]
    fn from(other: u32) -> Self {
         Ppacmp(other)
    }
}

impl ::core::fmt::Display for Ppacmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ppacmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pulse Width Modulator Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pppwm(pub u32);
impl Pppwm {
    #[doc="PWM Module 0 Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="PWM Module 0 Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="PWM Module 0 Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pppwm {
    #[inline]
    fn from(other: u32) -> Self {
         Pppwm(other)
    }
}

impl ::core::fmt::Display for Pppwm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pppwm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Quadrature Encoder Interface Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ppqei(pub u32);
impl Ppqei {
    #[doc="QEI Module 0 Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="QEI Module 0 Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="QEI Module 0 Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ppqei {
    #[inline]
    fn from(other: u32) -> Self {
         Ppqei(other)
    }
}

impl ::core::fmt::Display for Ppqei {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ppqei {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Low Pin Count Interface Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pplpc(pub u32);
impl Pplpc {
    #[doc="LPC Module Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="LPC Module Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="LPC Module Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pplpc {
    #[inline]
    fn from(other: u32) -> Self {
         Pplpc(other)
    }
}

impl ::core::fmt::Display for Pplpc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pplpc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Platform Environment Control Interface Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pppeci(pub u32);
impl Pppeci {
    #[doc="PECI Module Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="PECI Module Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="PECI Module Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pppeci {
    #[inline]
    fn from(other: u32) -> Self {
         Pppeci(other)
    }
}

impl ::core::fmt::Display for Pppeci {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pppeci {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Fan Control Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ppfan(pub u32);
impl Ppfan {
    #[doc="FAN Module 0 Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="FAN Module 0 Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="FAN Module 0 Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ppfan {
    #[inline]
    fn from(other: u32) -> Self {
         Ppfan(other)
    }
}

impl ::core::fmt::Display for Ppfan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ppfan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="EEPROM Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ppeeprom(pub u32);
impl Ppeeprom {
    #[doc="EEPROM Module Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="EEPROM Module Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="EEPROM Module Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ppeeprom {
    #[inline]
    fn from(other: u32) -> Self {
         Ppeeprom(other)
    }
}

impl ::core::fmt::Display for Ppeeprom {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ppeeprom {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="32/64-Bit Wide General-Purpose Timer Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ppwtimer(pub u32);
impl Ppwtimer {
    #[doc="32/64-Bit Wide General-Purpose Timer 0 Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="32/64-Bit Wide General-Purpose Timer 0 Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="32/64-Bit Wide General-Purpose Timer 0 Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ppwtimer {
    #[inline]
    fn from(other: u32) -> Self {
         Ppwtimer(other)
    }
}

impl ::core::fmt::Display for Ppwtimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ppwtimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Remote Temperature Sensor Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pprts(pub u32);
impl Pprts {
    #[doc="RTS Module Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="RTS Module Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="RTS Module Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pprts {
    #[inline]
    fn from(other: u32) -> Self {
         Pprts(other)
    }
}

impl ::core::fmt::Display for Pprts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pprts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC and Cryptographic Modules Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ppccm(pub u32);
impl Ppccm {
    #[doc="CRC and Cryptographic Modules Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="CRC and Cryptographic Modules Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="CRC and Cryptographic Modules Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ppccm {
    #[inline]
    fn from(other: u32) -> Self {
         Ppccm(other)
    }
}

impl ::core::fmt::Display for Ppccm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ppccm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LCD Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pplcd(pub u32);
impl Pplcd {
    #[doc="LCD Module Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="LCD Module Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="LCD Module Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pplcd {
    #[inline]
    fn from(other: u32) -> Self {
         Pplcd(other)
    }
}

impl ::core::fmt::Display for Pplcd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pplcd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="1-Wire Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ppowire(pub u32);
impl Ppowire {
    #[doc="1-Wire Module Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="1-Wire Module Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="1-Wire Module Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ppowire {
    #[inline]
    fn from(other: u32) -> Self {
         Ppowire(other)
    }
}

impl ::core::fmt::Display for Ppowire {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ppowire {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ppemac(pub u32);
impl Ppemac {
    #[doc="Ethernet Controller Module Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Ethernet Controller Module Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="Ethernet Controller Module Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ppemac {
    #[inline]
    fn from(other: u32) -> Self {
         Ppemac(other)
    }
}

impl ::core::fmt::Display for Ppemac {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ppemac {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Human Interface Master Peripheral Present"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pphim(pub u32);
impl Pphim {
    #[doc="HIM Module Present"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="HIM Module Present"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="HIM Module Present"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pphim {
    #[inline]
    fn from(other: u32) -> Self {
         Pphim(other)
    }
}

impl ::core::fmt::Display for Pphim {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pphim {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Timer Software Reset"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Srwd(pub u32);
impl Srwd {
    #[doc="Watchdog Timer 0 Software Reset"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Watchdog Timer 0 Software Reset"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="Watchdog Timer 0 Software Reset"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Watchdog Timer 1 Software Reset"]
    #[inline] pub fn r1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Watchdog Timer 1 Software Reset"]
    #[inline] pub fn test_r1(&self) -> bool {
        self.r1() != 0
    }

    #[doc="Watchdog Timer 1 Software Reset"]
    #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Srwd {
    #[inline]
    fn from(other: u32) -> Self {
         Srwd(other)
    }
}

impl ::core::fmt::Display for Srwd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Srwd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        if self.r1() != 0 { try!(write!(f, " r1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="16/32-Bit General-Purpose Timer Software Reset"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Srtimer(pub u32);
impl Srtimer {
    #[doc="16/32-Bit General-Purpose Timer 0 Software Reset"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="16/32-Bit General-Purpose Timer 0 Software Reset"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 0 Software Reset"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 1 Software Reset"]
    #[inline] pub fn r1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="16/32-Bit General-Purpose Timer 1 Software Reset"]
    #[inline] pub fn test_r1(&self) -> bool {
        self.r1() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 1 Software Reset"]
    #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 2 Software Reset"]
    #[inline] pub fn r2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="16/32-Bit General-Purpose Timer 2 Software Reset"]
    #[inline] pub fn test_r2(&self) -> bool {
        self.r2() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 2 Software Reset"]
    #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 3 Software Reset"]
    #[inline] pub fn r3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="16/32-Bit General-Purpose Timer 3 Software Reset"]
    #[inline] pub fn test_r3(&self) -> bool {
        self.r3() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 3 Software Reset"]
    #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 4 Software Reset"]
    #[inline] pub fn r4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="16/32-Bit General-Purpose Timer 4 Software Reset"]
    #[inline] pub fn test_r4(&self) -> bool {
        self.r4() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 4 Software Reset"]
    #[inline] pub fn set_r4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 5 Software Reset"]
    #[inline] pub fn r5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="16/32-Bit General-Purpose Timer 5 Software Reset"]
    #[inline] pub fn test_r5(&self) -> bool {
        self.r5() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 5 Software Reset"]
    #[inline] pub fn set_r5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 6 Software Reset"]
    #[inline] pub fn r6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="16/32-Bit General-Purpose Timer 6 Software Reset"]
    #[inline] pub fn test_r6(&self) -> bool {
        self.r6() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 6 Software Reset"]
    #[inline] pub fn set_r6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 7 Software Reset"]
    #[inline] pub fn r7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="16/32-Bit General-Purpose Timer 7 Software Reset"]
    #[inline] pub fn test_r7(&self) -> bool {
        self.r7() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 7 Software Reset"]
    #[inline] pub fn set_r7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Srtimer {
    #[inline]
    fn from(other: u32) -> Self {
         Srtimer(other)
    }
}

impl ::core::fmt::Display for Srtimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Srtimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        if self.r1() != 0 { try!(write!(f, " r1"))}
        if self.r2() != 0 { try!(write!(f, " r2"))}
        if self.r3() != 0 { try!(write!(f, " r3"))}
        if self.r4() != 0 { try!(write!(f, " r4"))}
        if self.r5() != 0 { try!(write!(f, " r5"))}
        if self.r6() != 0 { try!(write!(f, " r6"))}
        if self.r7() != 0 { try!(write!(f, " r7"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="General-Purpose Input/Output Software Reset"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Srgpio(pub u32);
impl Srgpio {
    #[doc="GPIO Port A Software Reset"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="GPIO Port A Software Reset"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="GPIO Port A Software Reset"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="GPIO Port B Software Reset"]
    #[inline] pub fn r1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="GPIO Port B Software Reset"]
    #[inline] pub fn test_r1(&self) -> bool {
        self.r1() != 0
    }

    #[doc="GPIO Port B Software Reset"]
    #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="GPIO Port C Software Reset"]
    #[inline] pub fn r2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="GPIO Port C Software Reset"]
    #[inline] pub fn test_r2(&self) -> bool {
        self.r2() != 0
    }

    #[doc="GPIO Port C Software Reset"]
    #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="GPIO Port D Software Reset"]
    #[inline] pub fn r3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="GPIO Port D Software Reset"]
    #[inline] pub fn test_r3(&self) -> bool {
        self.r3() != 0
    }

    #[doc="GPIO Port D Software Reset"]
    #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPIO Port E Software Reset"]
    #[inline] pub fn r4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="GPIO Port E Software Reset"]
    #[inline] pub fn test_r4(&self) -> bool {
        self.r4() != 0
    }

    #[doc="GPIO Port E Software Reset"]
    #[inline] pub fn set_r4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPIO Port F Software Reset"]
    #[inline] pub fn r5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="GPIO Port F Software Reset"]
    #[inline] pub fn test_r5(&self) -> bool {
        self.r5() != 0
    }

    #[doc="GPIO Port F Software Reset"]
    #[inline] pub fn set_r5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="GPIO Port G Software Reset"]
    #[inline] pub fn r6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="GPIO Port G Software Reset"]
    #[inline] pub fn test_r6(&self) -> bool {
        self.r6() != 0
    }

    #[doc="GPIO Port G Software Reset"]
    #[inline] pub fn set_r6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="GPIO Port H Software Reset"]
    #[inline] pub fn r7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="GPIO Port H Software Reset"]
    #[inline] pub fn test_r7(&self) -> bool {
        self.r7() != 0
    }

    #[doc="GPIO Port H Software Reset"]
    #[inline] pub fn set_r7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="GPIO Port J Software Reset"]
    #[inline] pub fn r8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="GPIO Port J Software Reset"]
    #[inline] pub fn test_r8(&self) -> bool {
        self.r8() != 0
    }

    #[doc="GPIO Port J Software Reset"]
    #[inline] pub fn set_r8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="GPIO Port K Software Reset"]
    #[inline] pub fn r9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="GPIO Port K Software Reset"]
    #[inline] pub fn test_r9(&self) -> bool {
        self.r9() != 0
    }

    #[doc="GPIO Port K Software Reset"]
    #[inline] pub fn set_r9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="GPIO Port L Software Reset"]
    #[inline] pub fn r10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="GPIO Port L Software Reset"]
    #[inline] pub fn test_r10(&self) -> bool {
        self.r10() != 0
    }

    #[doc="GPIO Port L Software Reset"]
    #[inline] pub fn set_r10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="GPIO Port M Software Reset"]
    #[inline] pub fn r11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="GPIO Port M Software Reset"]
    #[inline] pub fn test_r11(&self) -> bool {
        self.r11() != 0
    }

    #[doc="GPIO Port M Software Reset"]
    #[inline] pub fn set_r11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="GPIO Port N Software Reset"]
    #[inline] pub fn r12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="GPIO Port N Software Reset"]
    #[inline] pub fn test_r12(&self) -> bool {
        self.r12() != 0
    }

    #[doc="GPIO Port N Software Reset"]
    #[inline] pub fn set_r12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="GPIO Port P Software Reset"]
    #[inline] pub fn r13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="GPIO Port P Software Reset"]
    #[inline] pub fn test_r13(&self) -> bool {
        self.r13() != 0
    }

    #[doc="GPIO Port P Software Reset"]
    #[inline] pub fn set_r13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="GPIO Port Q Software Reset"]
    #[inline] pub fn r14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="GPIO Port Q Software Reset"]
    #[inline] pub fn test_r14(&self) -> bool {
        self.r14() != 0
    }

    #[doc="GPIO Port Q Software Reset"]
    #[inline] pub fn set_r14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

}

impl From<u32> for Srgpio {
    #[inline]
    fn from(other: u32) -> Self {
         Srgpio(other)
    }
}

impl ::core::fmt::Display for Srgpio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Srgpio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        if self.r1() != 0 { try!(write!(f, " r1"))}
        if self.r2() != 0 { try!(write!(f, " r2"))}
        if self.r3() != 0 { try!(write!(f, " r3"))}
        if self.r4() != 0 { try!(write!(f, " r4"))}
        if self.r5() != 0 { try!(write!(f, " r5"))}
        if self.r6() != 0 { try!(write!(f, " r6"))}
        if self.r7() != 0 { try!(write!(f, " r7"))}
        if self.r8() != 0 { try!(write!(f, " r8"))}
        if self.r9() != 0 { try!(write!(f, " r9"))}
        if self.r10() != 0 { try!(write!(f, " r10"))}
        if self.r11() != 0 { try!(write!(f, " r11"))}
        if self.r12() != 0 { try!(write!(f, " r12"))}
        if self.r13() != 0 { try!(write!(f, " r13"))}
        if self.r14() != 0 { try!(write!(f, " r14"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Micro Direct Memory Access Software Reset"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Srdma(pub u32);
impl Srdma {
    #[doc="uDMA Module Software Reset"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="uDMA Module Software Reset"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="uDMA Module Software Reset"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Srdma {
    #[inline]
    fn from(other: u32) -> Self {
         Srdma(other)
    }
}

impl ::core::fmt::Display for Srdma {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Srdma {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="EPI Software Reset"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Srepi(pub u32);
impl Srepi {
    #[doc="EPI Module Software Reset"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="EPI Module Software Reset"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="EPI Module Software Reset"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Srepi {
    #[inline]
    fn from(other: u32) -> Self {
         Srepi(other)
    }
}

impl ::core::fmt::Display for Srepi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Srepi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Hibernation Software Reset"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Srhib(pub u32);
impl Srhib {
    #[doc="Hibernation Module Software Reset"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Hibernation Module Software Reset"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="Hibernation Module Software Reset"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Srhib {
    #[inline]
    fn from(other: u32) -> Self {
         Srhib(other)
    }
}

impl ::core::fmt::Display for Srhib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Srhib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Universal Asynchronous Receiver/Transmitter Software Reset"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sruart(pub u32);
impl Sruart {
    #[doc="UART Module 0 Software Reset"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="UART Module 0 Software Reset"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="UART Module 0 Software Reset"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="UART Module 1 Software Reset"]
    #[inline] pub fn r1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="UART Module 1 Software Reset"]
    #[inline] pub fn test_r1(&self) -> bool {
        self.r1() != 0
    }

    #[doc="UART Module 1 Software Reset"]
    #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="UART Module 2 Software Reset"]
    #[inline] pub fn r2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="UART Module 2 Software Reset"]
    #[inline] pub fn test_r2(&self) -> bool {
        self.r2() != 0
    }

    #[doc="UART Module 2 Software Reset"]
    #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="UART Module 3 Software Reset"]
    #[inline] pub fn r3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="UART Module 3 Software Reset"]
    #[inline] pub fn test_r3(&self) -> bool {
        self.r3() != 0
    }

    #[doc="UART Module 3 Software Reset"]
    #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="UART Module 4 Software Reset"]
    #[inline] pub fn r4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="UART Module 4 Software Reset"]
    #[inline] pub fn test_r4(&self) -> bool {
        self.r4() != 0
    }

    #[doc="UART Module 4 Software Reset"]
    #[inline] pub fn set_r4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="UART Module 5 Software Reset"]
    #[inline] pub fn r5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="UART Module 5 Software Reset"]
    #[inline] pub fn test_r5(&self) -> bool {
        self.r5() != 0
    }

    #[doc="UART Module 5 Software Reset"]
    #[inline] pub fn set_r5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="UART Module 6 Software Reset"]
    #[inline] pub fn r6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="UART Module 6 Software Reset"]
    #[inline] pub fn test_r6(&self) -> bool {
        self.r6() != 0
    }

    #[doc="UART Module 6 Software Reset"]
    #[inline] pub fn set_r6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="UART Module 7 Software Reset"]
    #[inline] pub fn r7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="UART Module 7 Software Reset"]
    #[inline] pub fn test_r7(&self) -> bool {
        self.r7() != 0
    }

    #[doc="UART Module 7 Software Reset"]
    #[inline] pub fn set_r7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Sruart {
    #[inline]
    fn from(other: u32) -> Self {
         Sruart(other)
    }
}

impl ::core::fmt::Display for Sruart {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sruart {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        if self.r1() != 0 { try!(write!(f, " r1"))}
        if self.r2() != 0 { try!(write!(f, " r2"))}
        if self.r3() != 0 { try!(write!(f, " r3"))}
        if self.r4() != 0 { try!(write!(f, " r4"))}
        if self.r5() != 0 { try!(write!(f, " r5"))}
        if self.r6() != 0 { try!(write!(f, " r6"))}
        if self.r7() != 0 { try!(write!(f, " r7"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Synchronous Serial Interface Software Reset"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Srssi(pub u32);
impl Srssi {
    #[doc="SSI Module 0 Software Reset"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="SSI Module 0 Software Reset"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="SSI Module 0 Software Reset"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SSI Module 1 Software Reset"]
    #[inline] pub fn r1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="SSI Module 1 Software Reset"]
    #[inline] pub fn test_r1(&self) -> bool {
        self.r1() != 0
    }

    #[doc="SSI Module 1 Software Reset"]
    #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="SSI Module 2 Software Reset"]
    #[inline] pub fn r2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="SSI Module 2 Software Reset"]
    #[inline] pub fn test_r2(&self) -> bool {
        self.r2() != 0
    }

    #[doc="SSI Module 2 Software Reset"]
    #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SSI Module 3 Software Reset"]
    #[inline] pub fn r3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="SSI Module 3 Software Reset"]
    #[inline] pub fn test_r3(&self) -> bool {
        self.r3() != 0
    }

    #[doc="SSI Module 3 Software Reset"]
    #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Srssi {
    #[inline]
    fn from(other: u32) -> Self {
         Srssi(other)
    }
}

impl ::core::fmt::Display for Srssi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Srssi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        if self.r1() != 0 { try!(write!(f, " r1"))}
        if self.r2() != 0 { try!(write!(f, " r2"))}
        if self.r3() != 0 { try!(write!(f, " r3"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Inter-Integrated Circuit Software Reset"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sri2c(pub u32);
impl Sri2c {
    #[doc="I2C Module 0 Software Reset"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="I2C Module 0 Software Reset"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="I2C Module 0 Software Reset"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="I2C Module 1 Software Reset"]
    #[inline] pub fn r1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="I2C Module 1 Software Reset"]
    #[inline] pub fn test_r1(&self) -> bool {
        self.r1() != 0
    }

    #[doc="I2C Module 1 Software Reset"]
    #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="I2C Module 2 Software Reset"]
    #[inline] pub fn r2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="I2C Module 2 Software Reset"]
    #[inline] pub fn test_r2(&self) -> bool {
        self.r2() != 0
    }

    #[doc="I2C Module 2 Software Reset"]
    #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="I2C Module 3 Software Reset"]
    #[inline] pub fn r3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="I2C Module 3 Software Reset"]
    #[inline] pub fn test_r3(&self) -> bool {
        self.r3() != 0
    }

    #[doc="I2C Module 3 Software Reset"]
    #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="I2C Module 4 Software Reset"]
    #[inline] pub fn r4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="I2C Module 4 Software Reset"]
    #[inline] pub fn test_r4(&self) -> bool {
        self.r4() != 0
    }

    #[doc="I2C Module 4 Software Reset"]
    #[inline] pub fn set_r4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="I2C Module 5 Software Reset"]
    #[inline] pub fn r5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="I2C Module 5 Software Reset"]
    #[inline] pub fn test_r5(&self) -> bool {
        self.r5() != 0
    }

    #[doc="I2C Module 5 Software Reset"]
    #[inline] pub fn set_r5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="I2C Module 6 Software Reset"]
    #[inline] pub fn r6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="I2C Module 6 Software Reset"]
    #[inline] pub fn test_r6(&self) -> bool {
        self.r6() != 0
    }

    #[doc="I2C Module 6 Software Reset"]
    #[inline] pub fn set_r6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="I2C Module 7 Software Reset"]
    #[inline] pub fn r7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="I2C Module 7 Software Reset"]
    #[inline] pub fn test_r7(&self) -> bool {
        self.r7() != 0
    }

    #[doc="I2C Module 7 Software Reset"]
    #[inline] pub fn set_r7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="I2C Module 8 Software Reset"]
    #[inline] pub fn r8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="I2C Module 8 Software Reset"]
    #[inline] pub fn test_r8(&self) -> bool {
        self.r8() != 0
    }

    #[doc="I2C Module 8 Software Reset"]
    #[inline] pub fn set_r8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="I2C Module 9 Software Reset"]
    #[inline] pub fn r9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="I2C Module 9 Software Reset"]
    #[inline] pub fn test_r9(&self) -> bool {
        self.r9() != 0
    }

    #[doc="I2C Module 9 Software Reset"]
    #[inline] pub fn set_r9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

}

impl From<u32> for Sri2c {
    #[inline]
    fn from(other: u32) -> Self {
         Sri2c(other)
    }
}

impl ::core::fmt::Display for Sri2c {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sri2c {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        if self.r1() != 0 { try!(write!(f, " r1"))}
        if self.r2() != 0 { try!(write!(f, " r2"))}
        if self.r3() != 0 { try!(write!(f, " r3"))}
        if self.r4() != 0 { try!(write!(f, " r4"))}
        if self.r5() != 0 { try!(write!(f, " r5"))}
        if self.r6() != 0 { try!(write!(f, " r6"))}
        if self.r7() != 0 { try!(write!(f, " r7"))}
        if self.r8() != 0 { try!(write!(f, " r8"))}
        if self.r9() != 0 { try!(write!(f, " r9"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Universal Serial Bus Software Reset"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Srusb(pub u32);
impl Srusb {
    #[doc="USB Module Software Reset"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="USB Module Software Reset"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="USB Module Software Reset"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Srusb {
    #[inline]
    fn from(other: u32) -> Self {
         Srusb(other)
    }
}

impl ::core::fmt::Display for Srusb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Srusb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PHY Software Reset"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Srephy(pub u32);
impl Srephy {
    #[doc="Ethernet PHY Module Software Reset"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Ethernet PHY Module Software Reset"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="Ethernet PHY Module Software Reset"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Srephy {
    #[inline]
    fn from(other: u32) -> Self {
         Srephy(other)
    }
}

impl ::core::fmt::Display for Srephy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Srephy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Controller Area Network Software Reset"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Srcan(pub u32);
impl Srcan {
    #[doc="CAN Module 0 Software Reset"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="CAN Module 0 Software Reset"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="CAN Module 0 Software Reset"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="CAN Module 1 Software Reset"]
    #[inline] pub fn r1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="CAN Module 1 Software Reset"]
    #[inline] pub fn test_r1(&self) -> bool {
        self.r1() != 0
    }

    #[doc="CAN Module 1 Software Reset"]
    #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Srcan {
    #[inline]
    fn from(other: u32) -> Self {
         Srcan(other)
    }
}

impl ::core::fmt::Display for Srcan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Srcan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        if self.r1() != 0 { try!(write!(f, " r1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Analog-to-Digital Converter Software Reset"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sradc(pub u32);
impl Sradc {
    #[doc="ADC Module 0 Software Reset"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="ADC Module 0 Software Reset"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="ADC Module 0 Software Reset"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="ADC Module 1 Software Reset"]
    #[inline] pub fn r1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="ADC Module 1 Software Reset"]
    #[inline] pub fn test_r1(&self) -> bool {
        self.r1() != 0
    }

    #[doc="ADC Module 1 Software Reset"]
    #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Sradc {
    #[inline]
    fn from(other: u32) -> Self {
         Sradc(other)
    }
}

impl ::core::fmt::Display for Sradc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sradc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        if self.r1() != 0 { try!(write!(f, " r1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Analog Comparator Software Reset"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sracmp(pub u32);
impl Sracmp {
    #[doc="Analog Comparator Module 0 Software Reset"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Analog Comparator Module 0 Software Reset"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="Analog Comparator Module 0 Software Reset"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sracmp {
    #[inline]
    fn from(other: u32) -> Self {
         Sracmp(other)
    }
}

impl ::core::fmt::Display for Sracmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sracmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pulse Width Modulator Software Reset"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Srpwm(pub u32);
impl Srpwm {
    #[doc="PWM Module 0 Software Reset"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="PWM Module 0 Software Reset"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="PWM Module 0 Software Reset"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Srpwm {
    #[inline]
    fn from(other: u32) -> Self {
         Srpwm(other)
    }
}

impl ::core::fmt::Display for Srpwm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Srpwm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Quadrature Encoder Interface Software Reset"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Srqei(pub u32);
impl Srqei {
    #[doc="QEI Module 0 Software Reset"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="QEI Module 0 Software Reset"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="QEI Module 0 Software Reset"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Srqei {
    #[inline]
    fn from(other: u32) -> Self {
         Srqei(other)
    }
}

impl ::core::fmt::Display for Srqei {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Srqei {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="EEPROM Software Reset"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sreeprom(pub u32);
impl Sreeprom {
    #[doc="EEPROM Module Software Reset"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="EEPROM Module Software Reset"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="EEPROM Module Software Reset"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sreeprom {
    #[inline]
    fn from(other: u32) -> Self {
         Sreeprom(other)
    }
}

impl ::core::fmt::Display for Sreeprom {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sreeprom {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC and Cryptographic Modules Software Reset"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Srccm(pub u32);
impl Srccm {
    #[doc="CRC and Cryptographic Modules Software Reset"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="CRC and Cryptographic Modules Software Reset"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="CRC and Cryptographic Modules Software Reset"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Srccm {
    #[inline]
    fn from(other: u32) -> Self {
         Srccm(other)
    }
}

impl ::core::fmt::Display for Srccm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Srccm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Software Reset"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sremac(pub u32);
impl Sremac {
    #[doc="Ethernet Controller MAC Module 0 Software Reset"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Ethernet Controller MAC Module 0 Software Reset"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="Ethernet Controller MAC Module 0 Software Reset"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sremac {
    #[inline]
    fn from(other: u32) -> Self {
         Sremac(other)
    }
}

impl ::core::fmt::Display for Sremac {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sremac {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Timer Run Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rcgcwd(pub u32);
impl Rcgcwd {
    #[doc="Watchdog Timer 0 Run Mode Clock Gating Control"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Watchdog Timer 0 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="Watchdog Timer 0 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Watchdog Timer 1 Run Mode Clock Gating Control"]
    #[inline] pub fn r1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Watchdog Timer 1 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r1(&self) -> bool {
        self.r1() != 0
    }

    #[doc="Watchdog Timer 1 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Rcgcwd {
    #[inline]
    fn from(other: u32) -> Self {
         Rcgcwd(other)
    }
}

impl ::core::fmt::Display for Rcgcwd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rcgcwd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        if self.r1() != 0 { try!(write!(f, " r1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="16/32-Bit General-Purpose Timer Run Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rcgctimer(pub u32);
impl Rcgctimer {
    #[doc="16/32-Bit General-Purpose Timer 0 Run Mode Clock Gating Control"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="16/32-Bit General-Purpose Timer 0 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 0 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 1 Run Mode Clock Gating Control"]
    #[inline] pub fn r1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="16/32-Bit General-Purpose Timer 1 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r1(&self) -> bool {
        self.r1() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 1 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 2 Run Mode Clock Gating Control"]
    #[inline] pub fn r2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="16/32-Bit General-Purpose Timer 2 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r2(&self) -> bool {
        self.r2() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 2 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 3 Run Mode Clock Gating Control"]
    #[inline] pub fn r3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="16/32-Bit General-Purpose Timer 3 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r3(&self) -> bool {
        self.r3() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 3 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 4 Run Mode Clock Gating Control"]
    #[inline] pub fn r4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="16/32-Bit General-Purpose Timer 4 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r4(&self) -> bool {
        self.r4() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 4 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 5 Run Mode Clock Gating Control"]
    #[inline] pub fn r5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="16/32-Bit General-Purpose Timer 5 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r5(&self) -> bool {
        self.r5() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 5 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 6 Run Mode Clock Gating Control"]
    #[inline] pub fn r6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="16/32-Bit General-Purpose Timer 6 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r6(&self) -> bool {
        self.r6() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 6 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 7 Run Mode Clock Gating Control"]
    #[inline] pub fn r7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="16/32-Bit General-Purpose Timer 7 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r7(&self) -> bool {
        self.r7() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 7 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Rcgctimer {
    #[inline]
    fn from(other: u32) -> Self {
         Rcgctimer(other)
    }
}

impl ::core::fmt::Display for Rcgctimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rcgctimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        if self.r1() != 0 { try!(write!(f, " r1"))}
        if self.r2() != 0 { try!(write!(f, " r2"))}
        if self.r3() != 0 { try!(write!(f, " r3"))}
        if self.r4() != 0 { try!(write!(f, " r4"))}
        if self.r5() != 0 { try!(write!(f, " r5"))}
        if self.r6() != 0 { try!(write!(f, " r6"))}
        if self.r7() != 0 { try!(write!(f, " r7"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="General-Purpose Input/Output Run Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rcgcgpio(pub u32);
impl Rcgcgpio {
    #[doc="GPIO Port A Run Mode Clock Gating Control"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="GPIO Port A Run Mode Clock Gating Control"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="GPIO Port A Run Mode Clock Gating Control"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="GPIO Port B Run Mode Clock Gating Control"]
    #[inline] pub fn r1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="GPIO Port B Run Mode Clock Gating Control"]
    #[inline] pub fn test_r1(&self) -> bool {
        self.r1() != 0
    }

    #[doc="GPIO Port B Run Mode Clock Gating Control"]
    #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="GPIO Port C Run Mode Clock Gating Control"]
    #[inline] pub fn r2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="GPIO Port C Run Mode Clock Gating Control"]
    #[inline] pub fn test_r2(&self) -> bool {
        self.r2() != 0
    }

    #[doc="GPIO Port C Run Mode Clock Gating Control"]
    #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="GPIO Port D Run Mode Clock Gating Control"]
    #[inline] pub fn r3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="GPIO Port D Run Mode Clock Gating Control"]
    #[inline] pub fn test_r3(&self) -> bool {
        self.r3() != 0
    }

    #[doc="GPIO Port D Run Mode Clock Gating Control"]
    #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPIO Port E Run Mode Clock Gating Control"]
    #[inline] pub fn r4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="GPIO Port E Run Mode Clock Gating Control"]
    #[inline] pub fn test_r4(&self) -> bool {
        self.r4() != 0
    }

    #[doc="GPIO Port E Run Mode Clock Gating Control"]
    #[inline] pub fn set_r4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPIO Port F Run Mode Clock Gating Control"]
    #[inline] pub fn r5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="GPIO Port F Run Mode Clock Gating Control"]
    #[inline] pub fn test_r5(&self) -> bool {
        self.r5() != 0
    }

    #[doc="GPIO Port F Run Mode Clock Gating Control"]
    #[inline] pub fn set_r5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="GPIO Port G Run Mode Clock Gating Control"]
    #[inline] pub fn r6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="GPIO Port G Run Mode Clock Gating Control"]
    #[inline] pub fn test_r6(&self) -> bool {
        self.r6() != 0
    }

    #[doc="GPIO Port G Run Mode Clock Gating Control"]
    #[inline] pub fn set_r6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="GPIO Port H Run Mode Clock Gating Control"]
    #[inline] pub fn r7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="GPIO Port H Run Mode Clock Gating Control"]
    #[inline] pub fn test_r7(&self) -> bool {
        self.r7() != 0
    }

    #[doc="GPIO Port H Run Mode Clock Gating Control"]
    #[inline] pub fn set_r7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="GPIO Port J Run Mode Clock Gating Control"]
    #[inline] pub fn r8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="GPIO Port J Run Mode Clock Gating Control"]
    #[inline] pub fn test_r8(&self) -> bool {
        self.r8() != 0
    }

    #[doc="GPIO Port J Run Mode Clock Gating Control"]
    #[inline] pub fn set_r8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="GPIO Port K Run Mode Clock Gating Control"]
    #[inline] pub fn r9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="GPIO Port K Run Mode Clock Gating Control"]
    #[inline] pub fn test_r9(&self) -> bool {
        self.r9() != 0
    }

    #[doc="GPIO Port K Run Mode Clock Gating Control"]
    #[inline] pub fn set_r9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="GPIO Port L Run Mode Clock Gating Control"]
    #[inline] pub fn r10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="GPIO Port L Run Mode Clock Gating Control"]
    #[inline] pub fn test_r10(&self) -> bool {
        self.r10() != 0
    }

    #[doc="GPIO Port L Run Mode Clock Gating Control"]
    #[inline] pub fn set_r10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="GPIO Port M Run Mode Clock Gating Control"]
    #[inline] pub fn r11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="GPIO Port M Run Mode Clock Gating Control"]
    #[inline] pub fn test_r11(&self) -> bool {
        self.r11() != 0
    }

    #[doc="GPIO Port M Run Mode Clock Gating Control"]
    #[inline] pub fn set_r11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="GPIO Port N Run Mode Clock Gating Control"]
    #[inline] pub fn r12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="GPIO Port N Run Mode Clock Gating Control"]
    #[inline] pub fn test_r12(&self) -> bool {
        self.r12() != 0
    }

    #[doc="GPIO Port N Run Mode Clock Gating Control"]
    #[inline] pub fn set_r12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="GPIO Port P Run Mode Clock Gating Control"]
    #[inline] pub fn r13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="GPIO Port P Run Mode Clock Gating Control"]
    #[inline] pub fn test_r13(&self) -> bool {
        self.r13() != 0
    }

    #[doc="GPIO Port P Run Mode Clock Gating Control"]
    #[inline] pub fn set_r13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="GPIO Port Q Run Mode Clock Gating Control"]
    #[inline] pub fn r14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="GPIO Port Q Run Mode Clock Gating Control"]
    #[inline] pub fn test_r14(&self) -> bool {
        self.r14() != 0
    }

    #[doc="GPIO Port Q Run Mode Clock Gating Control"]
    #[inline] pub fn set_r14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

}

impl From<u32> for Rcgcgpio {
    #[inline]
    fn from(other: u32) -> Self {
         Rcgcgpio(other)
    }
}

impl ::core::fmt::Display for Rcgcgpio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rcgcgpio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        if self.r1() != 0 { try!(write!(f, " r1"))}
        if self.r2() != 0 { try!(write!(f, " r2"))}
        if self.r3() != 0 { try!(write!(f, " r3"))}
        if self.r4() != 0 { try!(write!(f, " r4"))}
        if self.r5() != 0 { try!(write!(f, " r5"))}
        if self.r6() != 0 { try!(write!(f, " r6"))}
        if self.r7() != 0 { try!(write!(f, " r7"))}
        if self.r8() != 0 { try!(write!(f, " r8"))}
        if self.r9() != 0 { try!(write!(f, " r9"))}
        if self.r10() != 0 { try!(write!(f, " r10"))}
        if self.r11() != 0 { try!(write!(f, " r11"))}
        if self.r12() != 0 { try!(write!(f, " r12"))}
        if self.r13() != 0 { try!(write!(f, " r13"))}
        if self.r14() != 0 { try!(write!(f, " r14"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Micro Direct Memory Access Run Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rcgcdma(pub u32);
impl Rcgcdma {
    #[doc="uDMA Module Run Mode Clock Gating Control"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="uDMA Module Run Mode Clock Gating Control"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="uDMA Module Run Mode Clock Gating Control"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rcgcdma {
    #[inline]
    fn from(other: u32) -> Self {
         Rcgcdma(other)
    }
}

impl ::core::fmt::Display for Rcgcdma {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rcgcdma {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="EPI Run Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rcgcepi(pub u32);
impl Rcgcepi {
    #[doc="EPI Module Run Mode Clock Gating Control"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="EPI Module Run Mode Clock Gating Control"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="EPI Module Run Mode Clock Gating Control"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rcgcepi {
    #[inline]
    fn from(other: u32) -> Self {
         Rcgcepi(other)
    }
}

impl ::core::fmt::Display for Rcgcepi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rcgcepi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Hibernation Run Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rcgchib(pub u32);
impl Rcgchib {
    #[doc="Hibernation Module Run Mode Clock Gating Control"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Hibernation Module Run Mode Clock Gating Control"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="Hibernation Module Run Mode Clock Gating Control"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rcgchib {
    #[inline]
    fn from(other: u32) -> Self {
         Rcgchib(other)
    }
}

impl ::core::fmt::Display for Rcgchib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rcgchib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Universal Asynchronous Receiver/Transmitter Run Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rcgcuart(pub u32);
impl Rcgcuart {
    #[doc="UART Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="UART Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="UART Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="UART Module 1 Run Mode Clock Gating Control"]
    #[inline] pub fn r1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="UART Module 1 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r1(&self) -> bool {
        self.r1() != 0
    }

    #[doc="UART Module 1 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="UART Module 2 Run Mode Clock Gating Control"]
    #[inline] pub fn r2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="UART Module 2 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r2(&self) -> bool {
        self.r2() != 0
    }

    #[doc="UART Module 2 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="UART Module 3 Run Mode Clock Gating Control"]
    #[inline] pub fn r3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="UART Module 3 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r3(&self) -> bool {
        self.r3() != 0
    }

    #[doc="UART Module 3 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="UART Module 4 Run Mode Clock Gating Control"]
    #[inline] pub fn r4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="UART Module 4 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r4(&self) -> bool {
        self.r4() != 0
    }

    #[doc="UART Module 4 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="UART Module 5 Run Mode Clock Gating Control"]
    #[inline] pub fn r5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="UART Module 5 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r5(&self) -> bool {
        self.r5() != 0
    }

    #[doc="UART Module 5 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="UART Module 6 Run Mode Clock Gating Control"]
    #[inline] pub fn r6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="UART Module 6 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r6(&self) -> bool {
        self.r6() != 0
    }

    #[doc="UART Module 6 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="UART Module 7 Run Mode Clock Gating Control"]
    #[inline] pub fn r7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="UART Module 7 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r7(&self) -> bool {
        self.r7() != 0
    }

    #[doc="UART Module 7 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Rcgcuart {
    #[inline]
    fn from(other: u32) -> Self {
         Rcgcuart(other)
    }
}

impl ::core::fmt::Display for Rcgcuart {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rcgcuart {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        if self.r1() != 0 { try!(write!(f, " r1"))}
        if self.r2() != 0 { try!(write!(f, " r2"))}
        if self.r3() != 0 { try!(write!(f, " r3"))}
        if self.r4() != 0 { try!(write!(f, " r4"))}
        if self.r5() != 0 { try!(write!(f, " r5"))}
        if self.r6() != 0 { try!(write!(f, " r6"))}
        if self.r7() != 0 { try!(write!(f, " r7"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Synchronous Serial Interface Run Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rcgcssi(pub u32);
impl Rcgcssi {
    #[doc="SSI Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="SSI Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="SSI Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SSI Module 1 Run Mode Clock Gating Control"]
    #[inline] pub fn r1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="SSI Module 1 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r1(&self) -> bool {
        self.r1() != 0
    }

    #[doc="SSI Module 1 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="SSI Module 2 Run Mode Clock Gating Control"]
    #[inline] pub fn r2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="SSI Module 2 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r2(&self) -> bool {
        self.r2() != 0
    }

    #[doc="SSI Module 2 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SSI Module 3 Run Mode Clock Gating Control"]
    #[inline] pub fn r3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="SSI Module 3 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r3(&self) -> bool {
        self.r3() != 0
    }

    #[doc="SSI Module 3 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Rcgcssi {
    #[inline]
    fn from(other: u32) -> Self {
         Rcgcssi(other)
    }
}

impl ::core::fmt::Display for Rcgcssi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rcgcssi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        if self.r1() != 0 { try!(write!(f, " r1"))}
        if self.r2() != 0 { try!(write!(f, " r2"))}
        if self.r3() != 0 { try!(write!(f, " r3"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Inter-Integrated Circuit Run Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rcgci2c(pub u32);
impl Rcgci2c {
    #[doc="I2C Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="I2C Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="I2C Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="I2C Module 1 Run Mode Clock Gating Control"]
    #[inline] pub fn r1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="I2C Module 1 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r1(&self) -> bool {
        self.r1() != 0
    }

    #[doc="I2C Module 1 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="I2C Module 2 Run Mode Clock Gating Control"]
    #[inline] pub fn r2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="I2C Module 2 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r2(&self) -> bool {
        self.r2() != 0
    }

    #[doc="I2C Module 2 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="I2C Module 3 Run Mode Clock Gating Control"]
    #[inline] pub fn r3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="I2C Module 3 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r3(&self) -> bool {
        self.r3() != 0
    }

    #[doc="I2C Module 3 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="I2C Module 4 Run Mode Clock Gating Control"]
    #[inline] pub fn r4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="I2C Module 4 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r4(&self) -> bool {
        self.r4() != 0
    }

    #[doc="I2C Module 4 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="I2C Module 5 Run Mode Clock Gating Control"]
    #[inline] pub fn r5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="I2C Module 5 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r5(&self) -> bool {
        self.r5() != 0
    }

    #[doc="I2C Module 5 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="I2C Module 6 Run Mode Clock Gating Control"]
    #[inline] pub fn r6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="I2C Module 6 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r6(&self) -> bool {
        self.r6() != 0
    }

    #[doc="I2C Module 6 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="I2C Module 7 Run Mode Clock Gating Control"]
    #[inline] pub fn r7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="I2C Module 7 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r7(&self) -> bool {
        self.r7() != 0
    }

    #[doc="I2C Module 7 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="I2C Module 8 Run Mode Clock Gating Control"]
    #[inline] pub fn r8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="I2C Module 8 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r8(&self) -> bool {
        self.r8() != 0
    }

    #[doc="I2C Module 8 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="I2C Module 9 Run Mode Clock Gating Control"]
    #[inline] pub fn r9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="I2C Module 9 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r9(&self) -> bool {
        self.r9() != 0
    }

    #[doc="I2C Module 9 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

}

impl From<u32> for Rcgci2c {
    #[inline]
    fn from(other: u32) -> Self {
         Rcgci2c(other)
    }
}

impl ::core::fmt::Display for Rcgci2c {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rcgci2c {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        if self.r1() != 0 { try!(write!(f, " r1"))}
        if self.r2() != 0 { try!(write!(f, " r2"))}
        if self.r3() != 0 { try!(write!(f, " r3"))}
        if self.r4() != 0 { try!(write!(f, " r4"))}
        if self.r5() != 0 { try!(write!(f, " r5"))}
        if self.r6() != 0 { try!(write!(f, " r6"))}
        if self.r7() != 0 { try!(write!(f, " r7"))}
        if self.r8() != 0 { try!(write!(f, " r8"))}
        if self.r9() != 0 { try!(write!(f, " r9"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Universal Serial Bus Run Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rcgcusb(pub u32);
impl Rcgcusb {
    #[doc="USB Module Run Mode Clock Gating Control"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="USB Module Run Mode Clock Gating Control"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="USB Module Run Mode Clock Gating Control"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rcgcusb {
    #[inline]
    fn from(other: u32) -> Self {
         Rcgcusb(other)
    }
}

impl ::core::fmt::Display for Rcgcusb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rcgcusb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PHY Run Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rcgcephy(pub u32);
impl Rcgcephy {
    #[doc="Ethernet PHY Module Run Mode Clock Gating Control"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Ethernet PHY Module Run Mode Clock Gating Control"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="Ethernet PHY Module Run Mode Clock Gating Control"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rcgcephy {
    #[inline]
    fn from(other: u32) -> Self {
         Rcgcephy(other)
    }
}

impl ::core::fmt::Display for Rcgcephy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rcgcephy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Controller Area Network Run Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rcgccan(pub u32);
impl Rcgccan {
    #[doc="CAN Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="CAN Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="CAN Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="CAN Module 1 Run Mode Clock Gating Control"]
    #[inline] pub fn r1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="CAN Module 1 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r1(&self) -> bool {
        self.r1() != 0
    }

    #[doc="CAN Module 1 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Rcgccan {
    #[inline]
    fn from(other: u32) -> Self {
         Rcgccan(other)
    }
}

impl ::core::fmt::Display for Rcgccan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rcgccan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        if self.r1() != 0 { try!(write!(f, " r1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Analog-to-Digital Converter Run Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rcgcadc(pub u32);
impl Rcgcadc {
    #[doc="ADC Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="ADC Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="ADC Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="ADC Module 1 Run Mode Clock Gating Control"]
    #[inline] pub fn r1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="ADC Module 1 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r1(&self) -> bool {
        self.r1() != 0
    }

    #[doc="ADC Module 1 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Rcgcadc {
    #[inline]
    fn from(other: u32) -> Self {
         Rcgcadc(other)
    }
}

impl ::core::fmt::Display for Rcgcadc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rcgcadc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        if self.r1() != 0 { try!(write!(f, " r1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Analog Comparator Run Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rcgcacmp(pub u32);
impl Rcgcacmp {
    #[doc="Analog Comparator Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Analog Comparator Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="Analog Comparator Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rcgcacmp {
    #[inline]
    fn from(other: u32) -> Self {
         Rcgcacmp(other)
    }
}

impl ::core::fmt::Display for Rcgcacmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rcgcacmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pulse Width Modulator Run Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rcgcpwm(pub u32);
impl Rcgcpwm {
    #[doc="PWM Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="PWM Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="PWM Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rcgcpwm {
    #[inline]
    fn from(other: u32) -> Self {
         Rcgcpwm(other)
    }
}

impl ::core::fmt::Display for Rcgcpwm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rcgcpwm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Quadrature Encoder Interface Run Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rcgcqei(pub u32);
impl Rcgcqei {
    #[doc="QEI Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="QEI Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="QEI Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rcgcqei {
    #[inline]
    fn from(other: u32) -> Self {
         Rcgcqei(other)
    }
}

impl ::core::fmt::Display for Rcgcqei {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rcgcqei {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="EEPROM Run Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rcgceeprom(pub u32);
impl Rcgceeprom {
    #[doc="EEPROM Module Run Mode Clock Gating Control"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="EEPROM Module Run Mode Clock Gating Control"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="EEPROM Module Run Mode Clock Gating Control"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rcgceeprom {
    #[inline]
    fn from(other: u32) -> Self {
         Rcgceeprom(other)
    }
}

impl ::core::fmt::Display for Rcgceeprom {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rcgceeprom {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC and Cryptographic Modules Run Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rcgcccm(pub u32);
impl Rcgcccm {
    #[doc="CRC and Cryptographic Modules Run Mode Clock Gating Control"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="CRC and Cryptographic Modules Run Mode Clock Gating Control"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="CRC and Cryptographic Modules Run Mode Clock Gating Control"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rcgcccm {
    #[inline]
    fn from(other: u32) -> Self {
         Rcgcccm(other)
    }
}

impl ::core::fmt::Display for Rcgcccm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rcgcccm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Run Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rcgcemac(pub u32);
impl Rcgcemac {
    #[doc="Ethernet MAC Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Ethernet MAC Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="Ethernet MAC Module 0 Run Mode Clock Gating Control"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rcgcemac {
    #[inline]
    fn from(other: u32) -> Self {
         Rcgcemac(other)
    }
}

impl ::core::fmt::Display for Rcgcemac {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rcgcemac {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Timer Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgcwd(pub u32);
impl Scgcwd {
    #[doc="Watchdog Timer 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Watchdog Timer 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s0(&self) -> bool {
        self.s0() != 0
    }

    #[doc="Watchdog Timer 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Watchdog Timer 1 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Watchdog Timer 1 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s1(&self) -> bool {
        self.s1() != 0
    }

    #[doc="Watchdog Timer 1 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Scgcwd {
    #[inline]
    fn from(other: u32) -> Self {
         Scgcwd(other)
    }
}

impl ::core::fmt::Display for Scgcwd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgcwd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s0() != 0 { try!(write!(f, " s0"))}
        if self.s1() != 0 { try!(write!(f, " s1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="16/32-Bit General-Purpose Timer Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgctimer(pub u32);
impl Scgctimer {
    #[doc="16/32-Bit General-Purpose Timer 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="16/32-Bit General-Purpose Timer 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s0(&self) -> bool {
        self.s0() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 1 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="16/32-Bit General-Purpose Timer 1 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s1(&self) -> bool {
        self.s1() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 1 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 2 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="16/32-Bit General-Purpose Timer 2 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s2(&self) -> bool {
        self.s2() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 2 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 3 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="16/32-Bit General-Purpose Timer 3 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s3(&self) -> bool {
        self.s3() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 3 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 4 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="16/32-Bit General-Purpose Timer 4 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s4(&self) -> bool {
        self.s4() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 4 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 5 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="16/32-Bit General-Purpose Timer 5 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s5(&self) -> bool {
        self.s5() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 5 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 6 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="16/32-Bit General-Purpose Timer 6 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s6(&self) -> bool {
        self.s6() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 6 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 7 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="16/32-Bit General-Purpose Timer 7 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s7(&self) -> bool {
        self.s7() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 7 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Scgctimer {
    #[inline]
    fn from(other: u32) -> Self {
         Scgctimer(other)
    }
}

impl ::core::fmt::Display for Scgctimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgctimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s0() != 0 { try!(write!(f, " s0"))}
        if self.s1() != 0 { try!(write!(f, " s1"))}
        if self.s2() != 0 { try!(write!(f, " s2"))}
        if self.s3() != 0 { try!(write!(f, " s3"))}
        if self.s4() != 0 { try!(write!(f, " s4"))}
        if self.s5() != 0 { try!(write!(f, " s5"))}
        if self.s6() != 0 { try!(write!(f, " s6"))}
        if self.s7() != 0 { try!(write!(f, " s7"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="General-Purpose Input/Output Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgcgpio(pub u32);
impl Scgcgpio {
    #[doc="GPIO Port A Sleep Mode Clock Gating Control"]
    #[inline] pub fn s0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="GPIO Port A Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s0(&self) -> bool {
        self.s0() != 0
    }

    #[doc="GPIO Port A Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="GPIO Port B Sleep Mode Clock Gating Control"]
    #[inline] pub fn s1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="GPIO Port B Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s1(&self) -> bool {
        self.s1() != 0
    }

    #[doc="GPIO Port B Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="GPIO Port C Sleep Mode Clock Gating Control"]
    #[inline] pub fn s2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="GPIO Port C Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s2(&self) -> bool {
        self.s2() != 0
    }

    #[doc="GPIO Port C Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="GPIO Port D Sleep Mode Clock Gating Control"]
    #[inline] pub fn s3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="GPIO Port D Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s3(&self) -> bool {
        self.s3() != 0
    }

    #[doc="GPIO Port D Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPIO Port E Sleep Mode Clock Gating Control"]
    #[inline] pub fn s4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="GPIO Port E Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s4(&self) -> bool {
        self.s4() != 0
    }

    #[doc="GPIO Port E Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPIO Port F Sleep Mode Clock Gating Control"]
    #[inline] pub fn s5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="GPIO Port F Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s5(&self) -> bool {
        self.s5() != 0
    }

    #[doc="GPIO Port F Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="GPIO Port G Sleep Mode Clock Gating Control"]
    #[inline] pub fn s6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="GPIO Port G Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s6(&self) -> bool {
        self.s6() != 0
    }

    #[doc="GPIO Port G Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="GPIO Port H Sleep Mode Clock Gating Control"]
    #[inline] pub fn s7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="GPIO Port H Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s7(&self) -> bool {
        self.s7() != 0
    }

    #[doc="GPIO Port H Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="GPIO Port J Sleep Mode Clock Gating Control"]
    #[inline] pub fn s8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="GPIO Port J Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s8(&self) -> bool {
        self.s8() != 0
    }

    #[doc="GPIO Port J Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="GPIO Port K Sleep Mode Clock Gating Control"]
    #[inline] pub fn s9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="GPIO Port K Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s9(&self) -> bool {
        self.s9() != 0
    }

    #[doc="GPIO Port K Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="GPIO Port L Sleep Mode Clock Gating Control"]
    #[inline] pub fn s10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="GPIO Port L Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s10(&self) -> bool {
        self.s10() != 0
    }

    #[doc="GPIO Port L Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="GPIO Port M Sleep Mode Clock Gating Control"]
    #[inline] pub fn s11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="GPIO Port M Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s11(&self) -> bool {
        self.s11() != 0
    }

    #[doc="GPIO Port M Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="GPIO Port N Sleep Mode Clock Gating Control"]
    #[inline] pub fn s12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="GPIO Port N Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s12(&self) -> bool {
        self.s12() != 0
    }

    #[doc="GPIO Port N Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="GPIO Port P Sleep Mode Clock Gating Control"]
    #[inline] pub fn s13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="GPIO Port P Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s13(&self) -> bool {
        self.s13() != 0
    }

    #[doc="GPIO Port P Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="GPIO Port Q Sleep Mode Clock Gating Control"]
    #[inline] pub fn s14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="GPIO Port Q Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s14(&self) -> bool {
        self.s14() != 0
    }

    #[doc="GPIO Port Q Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

}

impl From<u32> for Scgcgpio {
    #[inline]
    fn from(other: u32) -> Self {
         Scgcgpio(other)
    }
}

impl ::core::fmt::Display for Scgcgpio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgcgpio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s0() != 0 { try!(write!(f, " s0"))}
        if self.s1() != 0 { try!(write!(f, " s1"))}
        if self.s2() != 0 { try!(write!(f, " s2"))}
        if self.s3() != 0 { try!(write!(f, " s3"))}
        if self.s4() != 0 { try!(write!(f, " s4"))}
        if self.s5() != 0 { try!(write!(f, " s5"))}
        if self.s6() != 0 { try!(write!(f, " s6"))}
        if self.s7() != 0 { try!(write!(f, " s7"))}
        if self.s8() != 0 { try!(write!(f, " s8"))}
        if self.s9() != 0 { try!(write!(f, " s9"))}
        if self.s10() != 0 { try!(write!(f, " s10"))}
        if self.s11() != 0 { try!(write!(f, " s11"))}
        if self.s12() != 0 { try!(write!(f, " s12"))}
        if self.s13() != 0 { try!(write!(f, " s13"))}
        if self.s14() != 0 { try!(write!(f, " s14"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Micro Direct Memory Access Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgcdma(pub u32);
impl Scgcdma {
    #[doc="uDMA Module Sleep Mode Clock Gating Control"]
    #[inline] pub fn s0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="uDMA Module Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s0(&self) -> bool {
        self.s0() != 0
    }

    #[doc="uDMA Module Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scgcdma {
    #[inline]
    fn from(other: u32) -> Self {
         Scgcdma(other)
    }
}

impl ::core::fmt::Display for Scgcdma {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgcdma {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s0() != 0 { try!(write!(f, " s0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="EPI Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgcepi(pub u32);
impl Scgcepi {
    #[doc="EPI Module Sleep Mode Clock Gating Control"]
    #[inline] pub fn s0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="EPI Module Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s0(&self) -> bool {
        self.s0() != 0
    }

    #[doc="EPI Module Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scgcepi {
    #[inline]
    fn from(other: u32) -> Self {
         Scgcepi(other)
    }
}

impl ::core::fmt::Display for Scgcepi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgcepi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s0() != 0 { try!(write!(f, " s0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Hibernation Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgchib(pub u32);
impl Scgchib {
    #[doc="Hibernation Module Sleep Mode Clock Gating Control"]
    #[inline] pub fn s0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Hibernation Module Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s0(&self) -> bool {
        self.s0() != 0
    }

    #[doc="Hibernation Module Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scgchib {
    #[inline]
    fn from(other: u32) -> Self {
         Scgchib(other)
    }
}

impl ::core::fmt::Display for Scgchib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgchib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s0() != 0 { try!(write!(f, " s0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Universal Asynchronous Receiver/Transmitter Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgcuart(pub u32);
impl Scgcuart {
    #[doc="UART Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="UART Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s0(&self) -> bool {
        self.s0() != 0
    }

    #[doc="UART Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="UART Module 1 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="UART Module 1 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s1(&self) -> bool {
        self.s1() != 0
    }

    #[doc="UART Module 1 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="UART Module 2 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="UART Module 2 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s2(&self) -> bool {
        self.s2() != 0
    }

    #[doc="UART Module 2 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="UART Module 3 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="UART Module 3 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s3(&self) -> bool {
        self.s3() != 0
    }

    #[doc="UART Module 3 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="UART Module 4 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="UART Module 4 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s4(&self) -> bool {
        self.s4() != 0
    }

    #[doc="UART Module 4 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="UART Module 5 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="UART Module 5 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s5(&self) -> bool {
        self.s5() != 0
    }

    #[doc="UART Module 5 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="UART Module 6 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="UART Module 6 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s6(&self) -> bool {
        self.s6() != 0
    }

    #[doc="UART Module 6 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="UART Module 7 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="UART Module 7 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s7(&self) -> bool {
        self.s7() != 0
    }

    #[doc="UART Module 7 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Scgcuart {
    #[inline]
    fn from(other: u32) -> Self {
         Scgcuart(other)
    }
}

impl ::core::fmt::Display for Scgcuart {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgcuart {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s0() != 0 { try!(write!(f, " s0"))}
        if self.s1() != 0 { try!(write!(f, " s1"))}
        if self.s2() != 0 { try!(write!(f, " s2"))}
        if self.s3() != 0 { try!(write!(f, " s3"))}
        if self.s4() != 0 { try!(write!(f, " s4"))}
        if self.s5() != 0 { try!(write!(f, " s5"))}
        if self.s6() != 0 { try!(write!(f, " s6"))}
        if self.s7() != 0 { try!(write!(f, " s7"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Synchronous Serial Interface Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgcssi(pub u32);
impl Scgcssi {
    #[doc="SSI Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="SSI Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s0(&self) -> bool {
        self.s0() != 0
    }

    #[doc="SSI Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SSI Module 1 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="SSI Module 1 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s1(&self) -> bool {
        self.s1() != 0
    }

    #[doc="SSI Module 1 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="SSI Module 2 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="SSI Module 2 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s2(&self) -> bool {
        self.s2() != 0
    }

    #[doc="SSI Module 2 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SSI Module 3 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="SSI Module 3 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s3(&self) -> bool {
        self.s3() != 0
    }

    #[doc="SSI Module 3 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Scgcssi {
    #[inline]
    fn from(other: u32) -> Self {
         Scgcssi(other)
    }
}

impl ::core::fmt::Display for Scgcssi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgcssi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s0() != 0 { try!(write!(f, " s0"))}
        if self.s1() != 0 { try!(write!(f, " s1"))}
        if self.s2() != 0 { try!(write!(f, " s2"))}
        if self.s3() != 0 { try!(write!(f, " s3"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Inter-Integrated Circuit Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgci2c(pub u32);
impl Scgci2c {
    #[doc="I2C Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="I2C Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s0(&self) -> bool {
        self.s0() != 0
    }

    #[doc="I2C Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="I2C Module 1 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="I2C Module 1 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s1(&self) -> bool {
        self.s1() != 0
    }

    #[doc="I2C Module 1 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="I2C Module 2 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="I2C Module 2 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s2(&self) -> bool {
        self.s2() != 0
    }

    #[doc="I2C Module 2 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="I2C Module 3 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="I2C Module 3 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s3(&self) -> bool {
        self.s3() != 0
    }

    #[doc="I2C Module 3 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="I2C Module 4 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="I2C Module 4 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s4(&self) -> bool {
        self.s4() != 0
    }

    #[doc="I2C Module 4 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="I2C Module 5 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="I2C Module 5 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s5(&self) -> bool {
        self.s5() != 0
    }

    #[doc="I2C Module 5 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="I2C Module 6 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="I2C Module 6 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s6(&self) -> bool {
        self.s6() != 0
    }

    #[doc="I2C Module 6 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="I2C Module 7 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="I2C Module 7 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s7(&self) -> bool {
        self.s7() != 0
    }

    #[doc="I2C Module 7 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="I2C Module 8 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="I2C Module 8 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s8(&self) -> bool {
        self.s8() != 0
    }

    #[doc="I2C Module 8 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="I2C Module 9 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="I2C Module 9 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s9(&self) -> bool {
        self.s9() != 0
    }

    #[doc="I2C Module 9 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

}

impl From<u32> for Scgci2c {
    #[inline]
    fn from(other: u32) -> Self {
         Scgci2c(other)
    }
}

impl ::core::fmt::Display for Scgci2c {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgci2c {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s0() != 0 { try!(write!(f, " s0"))}
        if self.s1() != 0 { try!(write!(f, " s1"))}
        if self.s2() != 0 { try!(write!(f, " s2"))}
        if self.s3() != 0 { try!(write!(f, " s3"))}
        if self.s4() != 0 { try!(write!(f, " s4"))}
        if self.s5() != 0 { try!(write!(f, " s5"))}
        if self.s6() != 0 { try!(write!(f, " s6"))}
        if self.s7() != 0 { try!(write!(f, " s7"))}
        if self.s8() != 0 { try!(write!(f, " s8"))}
        if self.s9() != 0 { try!(write!(f, " s9"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Universal Serial Bus Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgcusb(pub u32);
impl Scgcusb {
    #[doc="USB Module Sleep Mode Clock Gating Control"]
    #[inline] pub fn s0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="USB Module Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s0(&self) -> bool {
        self.s0() != 0
    }

    #[doc="USB Module Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scgcusb {
    #[inline]
    fn from(other: u32) -> Self {
         Scgcusb(other)
    }
}

impl ::core::fmt::Display for Scgcusb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgcusb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s0() != 0 { try!(write!(f, " s0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PHY Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgcephy(pub u32);
impl Scgcephy {
    #[doc="PHY Module Sleep Mode Clock Gating Control"]
    #[inline] pub fn s0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="PHY Module Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s0(&self) -> bool {
        self.s0() != 0
    }

    #[doc="PHY Module Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scgcephy {
    #[inline]
    fn from(other: u32) -> Self {
         Scgcephy(other)
    }
}

impl ::core::fmt::Display for Scgcephy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgcephy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s0() != 0 { try!(write!(f, " s0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Controller Area Network Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgccan(pub u32);
impl Scgccan {
    #[doc="CAN Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="CAN Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s0(&self) -> bool {
        self.s0() != 0
    }

    #[doc="CAN Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="CAN Module 1 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="CAN Module 1 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s1(&self) -> bool {
        self.s1() != 0
    }

    #[doc="CAN Module 1 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Scgccan {
    #[inline]
    fn from(other: u32) -> Self {
         Scgccan(other)
    }
}

impl ::core::fmt::Display for Scgccan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgccan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s0() != 0 { try!(write!(f, " s0"))}
        if self.s1() != 0 { try!(write!(f, " s1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Analog-to-Digital Converter Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgcadc(pub u32);
impl Scgcadc {
    #[doc="ADC Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="ADC Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s0(&self) -> bool {
        self.s0() != 0
    }

    #[doc="ADC Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="ADC Module 1 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="ADC Module 1 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s1(&self) -> bool {
        self.s1() != 0
    }

    #[doc="ADC Module 1 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Scgcadc {
    #[inline]
    fn from(other: u32) -> Self {
         Scgcadc(other)
    }
}

impl ::core::fmt::Display for Scgcadc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgcadc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s0() != 0 { try!(write!(f, " s0"))}
        if self.s1() != 0 { try!(write!(f, " s1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Analog Comparator Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgcacmp(pub u32);
impl Scgcacmp {
    #[doc="Analog Comparator Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Analog Comparator Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s0(&self) -> bool {
        self.s0() != 0
    }

    #[doc="Analog Comparator Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scgcacmp {
    #[inline]
    fn from(other: u32) -> Self {
         Scgcacmp(other)
    }
}

impl ::core::fmt::Display for Scgcacmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgcacmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s0() != 0 { try!(write!(f, " s0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pulse Width Modulator Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgcpwm(pub u32);
impl Scgcpwm {
    #[doc="PWM Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="PWM Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s0(&self) -> bool {
        self.s0() != 0
    }

    #[doc="PWM Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scgcpwm {
    #[inline]
    fn from(other: u32) -> Self {
         Scgcpwm(other)
    }
}

impl ::core::fmt::Display for Scgcpwm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgcpwm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s0() != 0 { try!(write!(f, " s0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Quadrature Encoder Interface Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgcqei(pub u32);
impl Scgcqei {
    #[doc="QEI Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="QEI Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s0(&self) -> bool {
        self.s0() != 0
    }

    #[doc="QEI Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scgcqei {
    #[inline]
    fn from(other: u32) -> Self {
         Scgcqei(other)
    }
}

impl ::core::fmt::Display for Scgcqei {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgcqei {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s0() != 0 { try!(write!(f, " s0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="EEPROM Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgceeprom(pub u32);
impl Scgceeprom {
    #[doc="EEPROM Module Sleep Mode Clock Gating Control"]
    #[inline] pub fn s0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="EEPROM Module Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s0(&self) -> bool {
        self.s0() != 0
    }

    #[doc="EEPROM Module Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scgceeprom {
    #[inline]
    fn from(other: u32) -> Self {
         Scgceeprom(other)
    }
}

impl ::core::fmt::Display for Scgceeprom {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgceeprom {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s0() != 0 { try!(write!(f, " s0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC and Cryptographic Modules Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgcccm(pub u32);
impl Scgcccm {
    #[doc="CRC and Cryptographic Modules Sleep Mode Clock Gating Control"]
    #[inline] pub fn s0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="CRC and Cryptographic Modules Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s0(&self) -> bool {
        self.s0() != 0
    }

    #[doc="CRC and Cryptographic Modules Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scgcccm {
    #[inline]
    fn from(other: u32) -> Self {
         Scgcccm(other)
    }
}

impl ::core::fmt::Display for Scgcccm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgcccm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s0() != 0 { try!(write!(f, " s0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgcemac(pub u32);
impl Scgcemac {
    #[doc="Ethernet MAC Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn s0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Ethernet MAC Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_s0(&self) -> bool {
        self.s0() != 0
    }

    #[doc="Ethernet MAC Module 0 Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scgcemac {
    #[inline]
    fn from(other: u32) -> Self {
         Scgcemac(other)
    }
}

impl ::core::fmt::Display for Scgcemac {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgcemac {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s0() != 0 { try!(write!(f, " s0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Timer Deep-Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcgcwd(pub u32);
impl Dcgcwd {
    #[doc="Watchdog Timer 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Watchdog Timer 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d0(&self) -> bool {
        self.d0() != 0
    }

    #[doc="Watchdog Timer 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Watchdog Timer 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Watchdog Timer 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d1(&self) -> bool {
        self.d1() != 0
    }

    #[doc="Watchdog Timer 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Dcgcwd {
    #[inline]
    fn from(other: u32) -> Self {
         Dcgcwd(other)
    }
}

impl ::core::fmt::Display for Dcgcwd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcgcwd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.d0() != 0 { try!(write!(f, " d0"))}
        if self.d1() != 0 { try!(write!(f, " d1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="16/32-Bit General-Purpose Timer Deep-Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcgctimer(pub u32);
impl Dcgctimer {
    #[doc="16/32-Bit General-Purpose Timer 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="16/32-Bit General-Purpose Timer 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d0(&self) -> bool {
        self.d0() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="16/32-Bit General-Purpose Timer 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d1(&self) -> bool {
        self.d1() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="16/32-Bit General-Purpose Timer 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d2(&self) -> bool {
        self.d2() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="16/32-Bit General-Purpose Timer 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d3(&self) -> bool {
        self.d3() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 4 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="16/32-Bit General-Purpose Timer 4 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d4(&self) -> bool {
        self.d4() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 4 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 5 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="16/32-Bit General-Purpose Timer 5 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d5(&self) -> bool {
        self.d5() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 5 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 6 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="16/32-Bit General-Purpose Timer 6 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d6(&self) -> bool {
        self.d6() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 6 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 7 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="16/32-Bit General-Purpose Timer 7 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d7(&self) -> bool {
        self.d7() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 7 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Dcgctimer {
    #[inline]
    fn from(other: u32) -> Self {
         Dcgctimer(other)
    }
}

impl ::core::fmt::Display for Dcgctimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcgctimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.d0() != 0 { try!(write!(f, " d0"))}
        if self.d1() != 0 { try!(write!(f, " d1"))}
        if self.d2() != 0 { try!(write!(f, " d2"))}
        if self.d3() != 0 { try!(write!(f, " d3"))}
        if self.d4() != 0 { try!(write!(f, " d4"))}
        if self.d5() != 0 { try!(write!(f, " d5"))}
        if self.d6() != 0 { try!(write!(f, " d6"))}
        if self.d7() != 0 { try!(write!(f, " d7"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="General-Purpose Input/Output Deep-Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcgcgpio(pub u32);
impl Dcgcgpio {
    #[doc="GPIO Port A Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="GPIO Port A Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d0(&self) -> bool {
        self.d0() != 0
    }

    #[doc="GPIO Port A Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="GPIO Port B Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="GPIO Port B Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d1(&self) -> bool {
        self.d1() != 0
    }

    #[doc="GPIO Port B Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="GPIO Port C Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="GPIO Port C Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d2(&self) -> bool {
        self.d2() != 0
    }

    #[doc="GPIO Port C Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="GPIO Port D Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="GPIO Port D Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d3(&self) -> bool {
        self.d3() != 0
    }

    #[doc="GPIO Port D Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPIO Port E Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="GPIO Port E Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d4(&self) -> bool {
        self.d4() != 0
    }

    #[doc="GPIO Port E Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPIO Port F Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="GPIO Port F Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d5(&self) -> bool {
        self.d5() != 0
    }

    #[doc="GPIO Port F Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="GPIO Port G Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="GPIO Port G Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d6(&self) -> bool {
        self.d6() != 0
    }

    #[doc="GPIO Port G Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="GPIO Port H Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="GPIO Port H Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d7(&self) -> bool {
        self.d7() != 0
    }

    #[doc="GPIO Port H Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="GPIO Port J Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="GPIO Port J Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d8(&self) -> bool {
        self.d8() != 0
    }

    #[doc="GPIO Port J Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="GPIO Port K Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="GPIO Port K Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d9(&self) -> bool {
        self.d9() != 0
    }

    #[doc="GPIO Port K Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="GPIO Port L Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="GPIO Port L Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d10(&self) -> bool {
        self.d10() != 0
    }

    #[doc="GPIO Port L Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="GPIO Port M Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="GPIO Port M Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d11(&self) -> bool {
        self.d11() != 0
    }

    #[doc="GPIO Port M Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="GPIO Port N Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="GPIO Port N Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d12(&self) -> bool {
        self.d12() != 0
    }

    #[doc="GPIO Port N Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="GPIO Port P Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="GPIO Port P Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d13(&self) -> bool {
        self.d13() != 0
    }

    #[doc="GPIO Port P Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="GPIO Port Q Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="GPIO Port Q Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d14(&self) -> bool {
        self.d14() != 0
    }

    #[doc="GPIO Port Q Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

}

impl From<u32> for Dcgcgpio {
    #[inline]
    fn from(other: u32) -> Self {
         Dcgcgpio(other)
    }
}

impl ::core::fmt::Display for Dcgcgpio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcgcgpio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.d0() != 0 { try!(write!(f, " d0"))}
        if self.d1() != 0 { try!(write!(f, " d1"))}
        if self.d2() != 0 { try!(write!(f, " d2"))}
        if self.d3() != 0 { try!(write!(f, " d3"))}
        if self.d4() != 0 { try!(write!(f, " d4"))}
        if self.d5() != 0 { try!(write!(f, " d5"))}
        if self.d6() != 0 { try!(write!(f, " d6"))}
        if self.d7() != 0 { try!(write!(f, " d7"))}
        if self.d8() != 0 { try!(write!(f, " d8"))}
        if self.d9() != 0 { try!(write!(f, " d9"))}
        if self.d10() != 0 { try!(write!(f, " d10"))}
        if self.d11() != 0 { try!(write!(f, " d11"))}
        if self.d12() != 0 { try!(write!(f, " d12"))}
        if self.d13() != 0 { try!(write!(f, " d13"))}
        if self.d14() != 0 { try!(write!(f, " d14"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Micro Direct Memory Access Deep-Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcgcdma(pub u32);
impl Dcgcdma {
    #[doc="uDMA Module Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="uDMA Module Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d0(&self) -> bool {
        self.d0() != 0
    }

    #[doc="uDMA Module Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dcgcdma {
    #[inline]
    fn from(other: u32) -> Self {
         Dcgcdma(other)
    }
}

impl ::core::fmt::Display for Dcgcdma {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcgcdma {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.d0() != 0 { try!(write!(f, " d0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="EPI Deep-Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcgcepi(pub u32);
impl Dcgcepi {
    #[doc="EPI Module Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="EPI Module Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d0(&self) -> bool {
        self.d0() != 0
    }

    #[doc="EPI Module Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dcgcepi {
    #[inline]
    fn from(other: u32) -> Self {
         Dcgcepi(other)
    }
}

impl ::core::fmt::Display for Dcgcepi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcgcepi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.d0() != 0 { try!(write!(f, " d0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Hibernation Deep-Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcgchib(pub u32);
impl Dcgchib {
    #[doc="Hibernation Module Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Hibernation Module Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d0(&self) -> bool {
        self.d0() != 0
    }

    #[doc="Hibernation Module Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dcgchib {
    #[inline]
    fn from(other: u32) -> Self {
         Dcgchib(other)
    }
}

impl ::core::fmt::Display for Dcgchib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcgchib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.d0() != 0 { try!(write!(f, " d0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Universal Asynchronous Receiver/Transmitter Deep-Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcgcuart(pub u32);
impl Dcgcuart {
    #[doc="UART Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="UART Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d0(&self) -> bool {
        self.d0() != 0
    }

    #[doc="UART Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="UART Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="UART Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d1(&self) -> bool {
        self.d1() != 0
    }

    #[doc="UART Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="UART Module 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="UART Module 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d2(&self) -> bool {
        self.d2() != 0
    }

    #[doc="UART Module 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="UART Module 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="UART Module 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d3(&self) -> bool {
        self.d3() != 0
    }

    #[doc="UART Module 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="UART Module 4 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="UART Module 4 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d4(&self) -> bool {
        self.d4() != 0
    }

    #[doc="UART Module 4 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="UART Module 5 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="UART Module 5 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d5(&self) -> bool {
        self.d5() != 0
    }

    #[doc="UART Module 5 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="UART Module 6 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="UART Module 6 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d6(&self) -> bool {
        self.d6() != 0
    }

    #[doc="UART Module 6 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="UART Module 7 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="UART Module 7 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d7(&self) -> bool {
        self.d7() != 0
    }

    #[doc="UART Module 7 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Dcgcuart {
    #[inline]
    fn from(other: u32) -> Self {
         Dcgcuart(other)
    }
}

impl ::core::fmt::Display for Dcgcuart {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcgcuart {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.d0() != 0 { try!(write!(f, " d0"))}
        if self.d1() != 0 { try!(write!(f, " d1"))}
        if self.d2() != 0 { try!(write!(f, " d2"))}
        if self.d3() != 0 { try!(write!(f, " d3"))}
        if self.d4() != 0 { try!(write!(f, " d4"))}
        if self.d5() != 0 { try!(write!(f, " d5"))}
        if self.d6() != 0 { try!(write!(f, " d6"))}
        if self.d7() != 0 { try!(write!(f, " d7"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Synchronous Serial Interface Deep-Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcgcssi(pub u32);
impl Dcgcssi {
    #[doc="SSI Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="SSI Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d0(&self) -> bool {
        self.d0() != 0
    }

    #[doc="SSI Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SSI Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="SSI Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d1(&self) -> bool {
        self.d1() != 0
    }

    #[doc="SSI Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="SSI Module 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="SSI Module 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d2(&self) -> bool {
        self.d2() != 0
    }

    #[doc="SSI Module 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SSI Module 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="SSI Module 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d3(&self) -> bool {
        self.d3() != 0
    }

    #[doc="SSI Module 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Dcgcssi {
    #[inline]
    fn from(other: u32) -> Self {
         Dcgcssi(other)
    }
}

impl ::core::fmt::Display for Dcgcssi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcgcssi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.d0() != 0 { try!(write!(f, " d0"))}
        if self.d1() != 0 { try!(write!(f, " d1"))}
        if self.d2() != 0 { try!(write!(f, " d2"))}
        if self.d3() != 0 { try!(write!(f, " d3"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Inter-Integrated Circuit Deep-Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcgci2c(pub u32);
impl Dcgci2c {
    #[doc="I2C Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="I2C Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d0(&self) -> bool {
        self.d0() != 0
    }

    #[doc="I2C Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="I2C Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="I2C Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d1(&self) -> bool {
        self.d1() != 0
    }

    #[doc="I2C Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="I2C Module 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="I2C Module 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d2(&self) -> bool {
        self.d2() != 0
    }

    #[doc="I2C Module 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="I2C Module 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="I2C Module 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d3(&self) -> bool {
        self.d3() != 0
    }

    #[doc="I2C Module 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="I2C Module 4 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="I2C Module 4 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d4(&self) -> bool {
        self.d4() != 0
    }

    #[doc="I2C Module 4 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="I2C Module 5 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="I2C Module 5 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d5(&self) -> bool {
        self.d5() != 0
    }

    #[doc="I2C Module 5 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="I2C Module 6 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="I2C Module 6 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d6(&self) -> bool {
        self.d6() != 0
    }

    #[doc="I2C Module 6 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="I2C Module 7 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="I2C Module 7 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d7(&self) -> bool {
        self.d7() != 0
    }

    #[doc="I2C Module 7 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="I2C Module 8 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="I2C Module 8 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d8(&self) -> bool {
        self.d8() != 0
    }

    #[doc="I2C Module 8 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="I2C Module 9 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="I2C Module 9 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d9(&self) -> bool {
        self.d9() != 0
    }

    #[doc="I2C Module 9 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

}

impl From<u32> for Dcgci2c {
    #[inline]
    fn from(other: u32) -> Self {
         Dcgci2c(other)
    }
}

impl ::core::fmt::Display for Dcgci2c {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcgci2c {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.d0() != 0 { try!(write!(f, " d0"))}
        if self.d1() != 0 { try!(write!(f, " d1"))}
        if self.d2() != 0 { try!(write!(f, " d2"))}
        if self.d3() != 0 { try!(write!(f, " d3"))}
        if self.d4() != 0 { try!(write!(f, " d4"))}
        if self.d5() != 0 { try!(write!(f, " d5"))}
        if self.d6() != 0 { try!(write!(f, " d6"))}
        if self.d7() != 0 { try!(write!(f, " d7"))}
        if self.d8() != 0 { try!(write!(f, " d8"))}
        if self.d9() != 0 { try!(write!(f, " d9"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Universal Serial Bus Deep-Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcgcusb(pub u32);
impl Dcgcusb {
    #[doc="USB Module Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="USB Module Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d0(&self) -> bool {
        self.d0() != 0
    }

    #[doc="USB Module Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dcgcusb {
    #[inline]
    fn from(other: u32) -> Self {
         Dcgcusb(other)
    }
}

impl ::core::fmt::Display for Dcgcusb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcgcusb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.d0() != 0 { try!(write!(f, " d0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PHY Deep-Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcgcephy(pub u32);
impl Dcgcephy {
    #[doc="PHY Module Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="PHY Module Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d0(&self) -> bool {
        self.d0() != 0
    }

    #[doc="PHY Module Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dcgcephy {
    #[inline]
    fn from(other: u32) -> Self {
         Dcgcephy(other)
    }
}

impl ::core::fmt::Display for Dcgcephy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcgcephy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.d0() != 0 { try!(write!(f, " d0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Controller Area Network Deep-Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcgccan(pub u32);
impl Dcgccan {
    #[doc="CAN Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="CAN Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d0(&self) -> bool {
        self.d0() != 0
    }

    #[doc="CAN Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="CAN Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="CAN Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d1(&self) -> bool {
        self.d1() != 0
    }

    #[doc="CAN Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Dcgccan {
    #[inline]
    fn from(other: u32) -> Self {
         Dcgccan(other)
    }
}

impl ::core::fmt::Display for Dcgccan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcgccan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.d0() != 0 { try!(write!(f, " d0"))}
        if self.d1() != 0 { try!(write!(f, " d1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Analog-to-Digital Converter Deep-Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcgcadc(pub u32);
impl Dcgcadc {
    #[doc="ADC Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="ADC Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d0(&self) -> bool {
        self.d0() != 0
    }

    #[doc="ADC Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="ADC Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="ADC Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d1(&self) -> bool {
        self.d1() != 0
    }

    #[doc="ADC Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Dcgcadc {
    #[inline]
    fn from(other: u32) -> Self {
         Dcgcadc(other)
    }
}

impl ::core::fmt::Display for Dcgcadc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcgcadc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.d0() != 0 { try!(write!(f, " d0"))}
        if self.d1() != 0 { try!(write!(f, " d1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Analog Comparator Deep-Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcgcacmp(pub u32);
impl Dcgcacmp {
    #[doc="Analog Comparator Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Analog Comparator Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d0(&self) -> bool {
        self.d0() != 0
    }

    #[doc="Analog Comparator Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dcgcacmp {
    #[inline]
    fn from(other: u32) -> Self {
         Dcgcacmp(other)
    }
}

impl ::core::fmt::Display for Dcgcacmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcgcacmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.d0() != 0 { try!(write!(f, " d0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pulse Width Modulator Deep-Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcgcpwm(pub u32);
impl Dcgcpwm {
    #[doc="PWM Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="PWM Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d0(&self) -> bool {
        self.d0() != 0
    }

    #[doc="PWM Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dcgcpwm {
    #[inline]
    fn from(other: u32) -> Self {
         Dcgcpwm(other)
    }
}

impl ::core::fmt::Display for Dcgcpwm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcgcpwm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.d0() != 0 { try!(write!(f, " d0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Quadrature Encoder Interface Deep-Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcgcqei(pub u32);
impl Dcgcqei {
    #[doc="QEI Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="QEI Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d0(&self) -> bool {
        self.d0() != 0
    }

    #[doc="QEI Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dcgcqei {
    #[inline]
    fn from(other: u32) -> Self {
         Dcgcqei(other)
    }
}

impl ::core::fmt::Display for Dcgcqei {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcgcqei {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.d0() != 0 { try!(write!(f, " d0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="EEPROM Deep-Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcgceeprom(pub u32);
impl Dcgceeprom {
    #[doc="EEPROM Module Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="EEPROM Module Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d0(&self) -> bool {
        self.d0() != 0
    }

    #[doc="EEPROM Module Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dcgceeprom {
    #[inline]
    fn from(other: u32) -> Self {
         Dcgceeprom(other)
    }
}

impl ::core::fmt::Display for Dcgceeprom {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcgceeprom {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.d0() != 0 { try!(write!(f, " d0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC and Cryptographic Modules Deep-Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcgcccm(pub u32);
impl Dcgcccm {
    #[doc="CRC and Cryptographic Modules Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="CRC and Cryptographic Modules Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d0(&self) -> bool {
        self.d0() != 0
    }

    #[doc="CRC and Cryptographic Modules Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dcgcccm {
    #[inline]
    fn from(other: u32) -> Self {
         Dcgcccm(other)
    }
}

impl ::core::fmt::Display for Dcgcccm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcgcccm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.d0() != 0 { try!(write!(f, " d0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Deep-Sleep Mode Clock Gating Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcgcemac(pub u32);
impl Dcgcemac {
    #[doc="Ethernet MAC Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn d0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Ethernet MAC Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn test_d0(&self) -> bool {
        self.d0() != 0
    }

    #[doc="Ethernet MAC Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dcgcemac {
    #[inline]
    fn from(other: u32) -> Self {
         Dcgcemac(other)
    }
}

impl ::core::fmt::Display for Dcgcemac {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcgcemac {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.d0() != 0 { try!(write!(f, " d0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Timer Power Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcwd(pub u32);
impl Pcwd {
    #[doc="Watchdog Timer 0 Power Control"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Watchdog Timer 0 Power Control"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="Watchdog Timer 0 Power Control"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Watchdog Timer 1 Power Control"]
    #[inline] pub fn p1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Watchdog Timer 1 Power Control"]
    #[inline] pub fn test_p1(&self) -> bool {
        self.p1() != 0
    }

    #[doc="Watchdog Timer 1 Power Control"]
    #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Pcwd {
    #[inline]
    fn from(other: u32) -> Self {
         Pcwd(other)
    }
}

impl ::core::fmt::Display for Pcwd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcwd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        if self.p1() != 0 { try!(write!(f, " p1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="16/32-Bit General-Purpose Timer Power Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pctimer(pub u32);
impl Pctimer {
    #[doc="General-Purpose Timer 0 Power Control"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="General-Purpose Timer 0 Power Control"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="General-Purpose Timer 0 Power Control"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="General-Purpose Timer 1 Power Control"]
    #[inline] pub fn p1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="General-Purpose Timer 1 Power Control"]
    #[inline] pub fn test_p1(&self) -> bool {
        self.p1() != 0
    }

    #[doc="General-Purpose Timer 1 Power Control"]
    #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="General-Purpose Timer 2 Power Control"]
    #[inline] pub fn p2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="General-Purpose Timer 2 Power Control"]
    #[inline] pub fn test_p2(&self) -> bool {
        self.p2() != 0
    }

    #[doc="General-Purpose Timer 2 Power Control"]
    #[inline] pub fn set_p2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="General-Purpose Timer 3 Power Control"]
    #[inline] pub fn p3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="General-Purpose Timer 3 Power Control"]
    #[inline] pub fn test_p3(&self) -> bool {
        self.p3() != 0
    }

    #[doc="General-Purpose Timer 3 Power Control"]
    #[inline] pub fn set_p3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="General-Purpose Timer 4 Power Control"]
    #[inline] pub fn p4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="General-Purpose Timer 4 Power Control"]
    #[inline] pub fn test_p4(&self) -> bool {
        self.p4() != 0
    }

    #[doc="General-Purpose Timer 4 Power Control"]
    #[inline] pub fn set_p4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="General-Purpose Timer 5 Power Control"]
    #[inline] pub fn p5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="General-Purpose Timer 5 Power Control"]
    #[inline] pub fn test_p5(&self) -> bool {
        self.p5() != 0
    }

    #[doc="General-Purpose Timer 5 Power Control"]
    #[inline] pub fn set_p5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="General-Purpose Timer 6 Power Control"]
    #[inline] pub fn p6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="General-Purpose Timer 6 Power Control"]
    #[inline] pub fn test_p6(&self) -> bool {
        self.p6() != 0
    }

    #[doc="General-Purpose Timer 6 Power Control"]
    #[inline] pub fn set_p6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="General-Purpose Timer 7 Power Control"]
    #[inline] pub fn p7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="General-Purpose Timer 7 Power Control"]
    #[inline] pub fn test_p7(&self) -> bool {
        self.p7() != 0
    }

    #[doc="General-Purpose Timer 7 Power Control"]
    #[inline] pub fn set_p7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Pctimer {
    #[inline]
    fn from(other: u32) -> Self {
         Pctimer(other)
    }
}

impl ::core::fmt::Display for Pctimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pctimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        if self.p1() != 0 { try!(write!(f, " p1"))}
        if self.p2() != 0 { try!(write!(f, " p2"))}
        if self.p3() != 0 { try!(write!(f, " p3"))}
        if self.p4() != 0 { try!(write!(f, " p4"))}
        if self.p5() != 0 { try!(write!(f, " p5"))}
        if self.p6() != 0 { try!(write!(f, " p6"))}
        if self.p7() != 0 { try!(write!(f, " p7"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="General-Purpose Input/Output Power Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcgpio(pub u32);
impl Pcgpio {
    #[doc="GPIO Port A Power Control"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="GPIO Port A Power Control"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="GPIO Port A Power Control"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="GPIO Port B Power Control"]
    #[inline] pub fn p1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="GPIO Port B Power Control"]
    #[inline] pub fn test_p1(&self) -> bool {
        self.p1() != 0
    }

    #[doc="GPIO Port B Power Control"]
    #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="GPIO Port C Power Control"]
    #[inline] pub fn p2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="GPIO Port C Power Control"]
    #[inline] pub fn test_p2(&self) -> bool {
        self.p2() != 0
    }

    #[doc="GPIO Port C Power Control"]
    #[inline] pub fn set_p2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="GPIO Port D Power Control"]
    #[inline] pub fn p3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="GPIO Port D Power Control"]
    #[inline] pub fn test_p3(&self) -> bool {
        self.p3() != 0
    }

    #[doc="GPIO Port D Power Control"]
    #[inline] pub fn set_p3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPIO Port E Power Control"]
    #[inline] pub fn p4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="GPIO Port E Power Control"]
    #[inline] pub fn test_p4(&self) -> bool {
        self.p4() != 0
    }

    #[doc="GPIO Port E Power Control"]
    #[inline] pub fn set_p4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPIO Port F Power Control"]
    #[inline] pub fn p5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="GPIO Port F Power Control"]
    #[inline] pub fn test_p5(&self) -> bool {
        self.p5() != 0
    }

    #[doc="GPIO Port F Power Control"]
    #[inline] pub fn set_p5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="GPIO Port G Power Control"]
    #[inline] pub fn p6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="GPIO Port G Power Control"]
    #[inline] pub fn test_p6(&self) -> bool {
        self.p6() != 0
    }

    #[doc="GPIO Port G Power Control"]
    #[inline] pub fn set_p6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="GPIO Port H Power Control"]
    #[inline] pub fn p7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="GPIO Port H Power Control"]
    #[inline] pub fn test_p7(&self) -> bool {
        self.p7() != 0
    }

    #[doc="GPIO Port H Power Control"]
    #[inline] pub fn set_p7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="GPIO Port J Power Control"]
    #[inline] pub fn p8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="GPIO Port J Power Control"]
    #[inline] pub fn test_p8(&self) -> bool {
        self.p8() != 0
    }

    #[doc="GPIO Port J Power Control"]
    #[inline] pub fn set_p8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="GPIO Port K Power Control"]
    #[inline] pub fn p9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="GPIO Port K Power Control"]
    #[inline] pub fn test_p9(&self) -> bool {
        self.p9() != 0
    }

    #[doc="GPIO Port K Power Control"]
    #[inline] pub fn set_p9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="GPIO Port L Power Control"]
    #[inline] pub fn p10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="GPIO Port L Power Control"]
    #[inline] pub fn test_p10(&self) -> bool {
        self.p10() != 0
    }

    #[doc="GPIO Port L Power Control"]
    #[inline] pub fn set_p10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="GPIO Port M Power Control"]
    #[inline] pub fn p11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="GPIO Port M Power Control"]
    #[inline] pub fn test_p11(&self) -> bool {
        self.p11() != 0
    }

    #[doc="GPIO Port M Power Control"]
    #[inline] pub fn set_p11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="GPIO Port N Power Control"]
    #[inline] pub fn p12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="GPIO Port N Power Control"]
    #[inline] pub fn test_p12(&self) -> bool {
        self.p12() != 0
    }

    #[doc="GPIO Port N Power Control"]
    #[inline] pub fn set_p12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="GPIO Port P Power Control"]
    #[inline] pub fn p13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="GPIO Port P Power Control"]
    #[inline] pub fn test_p13(&self) -> bool {
        self.p13() != 0
    }

    #[doc="GPIO Port P Power Control"]
    #[inline] pub fn set_p13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="GPIO Port Q Power Control"]
    #[inline] pub fn p14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="GPIO Port Q Power Control"]
    #[inline] pub fn test_p14(&self) -> bool {
        self.p14() != 0
    }

    #[doc="GPIO Port Q Power Control"]
    #[inline] pub fn set_p14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

}

impl From<u32> for Pcgpio {
    #[inline]
    fn from(other: u32) -> Self {
         Pcgpio(other)
    }
}

impl ::core::fmt::Display for Pcgpio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcgpio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        if self.p1() != 0 { try!(write!(f, " p1"))}
        if self.p2() != 0 { try!(write!(f, " p2"))}
        if self.p3() != 0 { try!(write!(f, " p3"))}
        if self.p4() != 0 { try!(write!(f, " p4"))}
        if self.p5() != 0 { try!(write!(f, " p5"))}
        if self.p6() != 0 { try!(write!(f, " p6"))}
        if self.p7() != 0 { try!(write!(f, " p7"))}
        if self.p8() != 0 { try!(write!(f, " p8"))}
        if self.p9() != 0 { try!(write!(f, " p9"))}
        if self.p10() != 0 { try!(write!(f, " p10"))}
        if self.p11() != 0 { try!(write!(f, " p11"))}
        if self.p12() != 0 { try!(write!(f, " p12"))}
        if self.p13() != 0 { try!(write!(f, " p13"))}
        if self.p14() != 0 { try!(write!(f, " p14"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Micro Direct Memory Access Power Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcdma(pub u32);
impl Pcdma {
    #[doc="uDMA Module Power Control"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="uDMA Module Power Control"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="uDMA Module Power Control"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pcdma {
    #[inline]
    fn from(other: u32) -> Self {
         Pcdma(other)
    }
}

impl ::core::fmt::Display for Pcdma {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcdma {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="External Peripheral Interface Power Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcepi(pub u32);
impl Pcepi {
    #[doc="EPI Module Power Control"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="EPI Module Power Control"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="EPI Module Power Control"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pcepi {
    #[inline]
    fn from(other: u32) -> Self {
         Pcepi(other)
    }
}

impl ::core::fmt::Display for Pcepi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcepi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Hibernation Power Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pchib(pub u32);
impl Pchib {
    #[doc="Hibernation Module Power Control"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Hibernation Module Power Control"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="Hibernation Module Power Control"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pchib {
    #[inline]
    fn from(other: u32) -> Self {
         Pchib(other)
    }
}

impl ::core::fmt::Display for Pchib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pchib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Universal Asynchronous Receiver/Transmitter Power Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcuart(pub u32);
impl Pcuart {
    #[doc="UART Module 0 Power Control"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="UART Module 0 Power Control"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="UART Module 0 Power Control"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="UART Module 1 Power Control"]
    #[inline] pub fn p1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="UART Module 1 Power Control"]
    #[inline] pub fn test_p1(&self) -> bool {
        self.p1() != 0
    }

    #[doc="UART Module 1 Power Control"]
    #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="UART Module 2 Power Control"]
    #[inline] pub fn p2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="UART Module 2 Power Control"]
    #[inline] pub fn test_p2(&self) -> bool {
        self.p2() != 0
    }

    #[doc="UART Module 2 Power Control"]
    #[inline] pub fn set_p2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="UART Module 3 Power Control"]
    #[inline] pub fn p3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="UART Module 3 Power Control"]
    #[inline] pub fn test_p3(&self) -> bool {
        self.p3() != 0
    }

    #[doc="UART Module 3 Power Control"]
    #[inline] pub fn set_p3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="UART Module 4 Power Control"]
    #[inline] pub fn p4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="UART Module 4 Power Control"]
    #[inline] pub fn test_p4(&self) -> bool {
        self.p4() != 0
    }

    #[doc="UART Module 4 Power Control"]
    #[inline] pub fn set_p4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="UART Module 5 Power Control"]
    #[inline] pub fn p5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="UART Module 5 Power Control"]
    #[inline] pub fn test_p5(&self) -> bool {
        self.p5() != 0
    }

    #[doc="UART Module 5 Power Control"]
    #[inline] pub fn set_p5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="UART Module 6 Power Control"]
    #[inline] pub fn p6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="UART Module 6 Power Control"]
    #[inline] pub fn test_p6(&self) -> bool {
        self.p6() != 0
    }

    #[doc="UART Module 6 Power Control"]
    #[inline] pub fn set_p6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="UART Module 7 Power Control"]
    #[inline] pub fn p7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="UART Module 7 Power Control"]
    #[inline] pub fn test_p7(&self) -> bool {
        self.p7() != 0
    }

    #[doc="UART Module 7 Power Control"]
    #[inline] pub fn set_p7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Pcuart {
    #[inline]
    fn from(other: u32) -> Self {
         Pcuart(other)
    }
}

impl ::core::fmt::Display for Pcuart {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcuart {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        if self.p1() != 0 { try!(write!(f, " p1"))}
        if self.p2() != 0 { try!(write!(f, " p2"))}
        if self.p3() != 0 { try!(write!(f, " p3"))}
        if self.p4() != 0 { try!(write!(f, " p4"))}
        if self.p5() != 0 { try!(write!(f, " p5"))}
        if self.p6() != 0 { try!(write!(f, " p6"))}
        if self.p7() != 0 { try!(write!(f, " p7"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Synchronous Serial Interface Power Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcssi(pub u32);
impl Pcssi {
    #[doc="SSI Module 0 Power Control"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="SSI Module 0 Power Control"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="SSI Module 0 Power Control"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SSI Module 1 Power Control"]
    #[inline] pub fn p1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="SSI Module 1 Power Control"]
    #[inline] pub fn test_p1(&self) -> bool {
        self.p1() != 0
    }

    #[doc="SSI Module 1 Power Control"]
    #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="SSI Module 2 Power Control"]
    #[inline] pub fn p2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="SSI Module 2 Power Control"]
    #[inline] pub fn test_p2(&self) -> bool {
        self.p2() != 0
    }

    #[doc="SSI Module 2 Power Control"]
    #[inline] pub fn set_p2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SSI Module 3 Power Control"]
    #[inline] pub fn p3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="SSI Module 3 Power Control"]
    #[inline] pub fn test_p3(&self) -> bool {
        self.p3() != 0
    }

    #[doc="SSI Module 3 Power Control"]
    #[inline] pub fn set_p3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Pcssi {
    #[inline]
    fn from(other: u32) -> Self {
         Pcssi(other)
    }
}

impl ::core::fmt::Display for Pcssi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcssi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        if self.p1() != 0 { try!(write!(f, " p1"))}
        if self.p2() != 0 { try!(write!(f, " p2"))}
        if self.p3() != 0 { try!(write!(f, " p3"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Inter-Integrated Circuit Power Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pci2c(pub u32);
impl Pci2c {
    #[doc="I2C Module 0 Power Control"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="I2C Module 0 Power Control"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="I2C Module 0 Power Control"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="I2C Module 1 Power Control"]
    #[inline] pub fn p1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="I2C Module 1 Power Control"]
    #[inline] pub fn test_p1(&self) -> bool {
        self.p1() != 0
    }

    #[doc="I2C Module 1 Power Control"]
    #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="I2C Module 2 Power Control"]
    #[inline] pub fn p2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="I2C Module 2 Power Control"]
    #[inline] pub fn test_p2(&self) -> bool {
        self.p2() != 0
    }

    #[doc="I2C Module 2 Power Control"]
    #[inline] pub fn set_p2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="I2C Module 3 Power Control"]
    #[inline] pub fn p3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="I2C Module 3 Power Control"]
    #[inline] pub fn test_p3(&self) -> bool {
        self.p3() != 0
    }

    #[doc="I2C Module 3 Power Control"]
    #[inline] pub fn set_p3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="I2C Module 4 Power Control"]
    #[inline] pub fn p4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="I2C Module 4 Power Control"]
    #[inline] pub fn test_p4(&self) -> bool {
        self.p4() != 0
    }

    #[doc="I2C Module 4 Power Control"]
    #[inline] pub fn set_p4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="I2C Module 5 Power Control"]
    #[inline] pub fn p5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="I2C Module 5 Power Control"]
    #[inline] pub fn test_p5(&self) -> bool {
        self.p5() != 0
    }

    #[doc="I2C Module 5 Power Control"]
    #[inline] pub fn set_p5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="I2C Module 6 Power Control"]
    #[inline] pub fn p6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="I2C Module 6 Power Control"]
    #[inline] pub fn test_p6(&self) -> bool {
        self.p6() != 0
    }

    #[doc="I2C Module 6 Power Control"]
    #[inline] pub fn set_p6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="I2C Module 7 Power Control"]
    #[inline] pub fn p7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="I2C Module 7 Power Control"]
    #[inline] pub fn test_p7(&self) -> bool {
        self.p7() != 0
    }

    #[doc="I2C Module 7 Power Control"]
    #[inline] pub fn set_p7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="I2C Module 8 Power Control"]
    #[inline] pub fn p8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="I2C Module 8 Power Control"]
    #[inline] pub fn test_p8(&self) -> bool {
        self.p8() != 0
    }

    #[doc="I2C Module 8 Power Control"]
    #[inline] pub fn set_p8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="I2C Module 9 Power Control"]
    #[inline] pub fn p9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="I2C Module 9 Power Control"]
    #[inline] pub fn test_p9(&self) -> bool {
        self.p9() != 0
    }

    #[doc="I2C Module 9 Power Control"]
    #[inline] pub fn set_p9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

}

impl From<u32> for Pci2c {
    #[inline]
    fn from(other: u32) -> Self {
         Pci2c(other)
    }
}

impl ::core::fmt::Display for Pci2c {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pci2c {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        if self.p1() != 0 { try!(write!(f, " p1"))}
        if self.p2() != 0 { try!(write!(f, " p2"))}
        if self.p3() != 0 { try!(write!(f, " p3"))}
        if self.p4() != 0 { try!(write!(f, " p4"))}
        if self.p5() != 0 { try!(write!(f, " p5"))}
        if self.p6() != 0 { try!(write!(f, " p6"))}
        if self.p7() != 0 { try!(write!(f, " p7"))}
        if self.p8() != 0 { try!(write!(f, " p8"))}
        if self.p9() != 0 { try!(write!(f, " p9"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Universal Serial Bus Power Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcusb(pub u32);
impl Pcusb {
    #[doc="USB Module Power Control"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="USB Module Power Control"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="USB Module Power Control"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pcusb {
    #[inline]
    fn from(other: u32) -> Self {
         Pcusb(other)
    }
}

impl ::core::fmt::Display for Pcusb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcusb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PHY Power Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcephy(pub u32);
impl Pcephy {
    #[doc="Ethernet PHY Module Power Control"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Ethernet PHY Module Power Control"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="Ethernet PHY Module Power Control"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pcephy {
    #[inline]
    fn from(other: u32) -> Self {
         Pcephy(other)
    }
}

impl ::core::fmt::Display for Pcephy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcephy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Controller Area Network Power Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccan(pub u32);
impl Pccan {
    #[doc="CAN Module 0 Power Control"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="CAN Module 0 Power Control"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="CAN Module 0 Power Control"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="CAN Module 1 Power Control"]
    #[inline] pub fn p1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="CAN Module 1 Power Control"]
    #[inline] pub fn test_p1(&self) -> bool {
        self.p1() != 0
    }

    #[doc="CAN Module 1 Power Control"]
    #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Pccan {
    #[inline]
    fn from(other: u32) -> Self {
         Pccan(other)
    }
}

impl ::core::fmt::Display for Pccan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        if self.p1() != 0 { try!(write!(f, " p1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Analog-to-Digital Converter Power Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcadc(pub u32);
impl Pcadc {
    #[doc="ADC Module 0 Power Control"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="ADC Module 0 Power Control"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="ADC Module 0 Power Control"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="ADC Module 1 Power Control"]
    #[inline] pub fn p1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="ADC Module 1 Power Control"]
    #[inline] pub fn test_p1(&self) -> bool {
        self.p1() != 0
    }

    #[doc="ADC Module 1 Power Control"]
    #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Pcadc {
    #[inline]
    fn from(other: u32) -> Self {
         Pcadc(other)
    }
}

impl ::core::fmt::Display for Pcadc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcadc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        if self.p1() != 0 { try!(write!(f, " p1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Analog Comparator Power Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcacmp(pub u32);
impl Pcacmp {
    #[doc="Analog Comparator Module 0 Power Control"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Analog Comparator Module 0 Power Control"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="Analog Comparator Module 0 Power Control"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pcacmp {
    #[inline]
    fn from(other: u32) -> Self {
         Pcacmp(other)
    }
}

impl ::core::fmt::Display for Pcacmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcacmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pulse Width Modulator Power Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcpwm(pub u32);
impl Pcpwm {
    #[doc="PWM Module 0 Power Control"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="PWM Module 0 Power Control"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="PWM Module 0 Power Control"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pcpwm {
    #[inline]
    fn from(other: u32) -> Self {
         Pcpwm(other)
    }
}

impl ::core::fmt::Display for Pcpwm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcpwm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Quadrature Encoder Interface Power Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcqei(pub u32);
impl Pcqei {
    #[doc="QEI Module 0 Power Control"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="QEI Module 0 Power Control"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="QEI Module 0 Power Control"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pcqei {
    #[inline]
    fn from(other: u32) -> Self {
         Pcqei(other)
    }
}

impl ::core::fmt::Display for Pcqei {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcqei {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="EEPROM Power Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pceeprom(pub u32);
impl Pceeprom {
    #[doc="EEPROM Module 0 Power Control"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="EEPROM Module 0 Power Control"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="EEPROM Module 0 Power Control"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pceeprom {
    #[inline]
    fn from(other: u32) -> Self {
         Pceeprom(other)
    }
}

impl ::core::fmt::Display for Pceeprom {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pceeprom {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC and Cryptographic Modules Power Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcccm(pub u32);
impl Pcccm {
    #[doc="CRC and Cryptographic Modules Power Control"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="CRC and Cryptographic Modules Power Control"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="CRC and Cryptographic Modules Power Control"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pcccm {
    #[inline]
    fn from(other: u32) -> Self {
         Pcccm(other)
    }
}

impl ::core::fmt::Display for Pcccm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcccm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Power Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcemac(pub u32);
impl Pcemac {
    #[doc="Ethernet MAC Module 0 Power Control"]
    #[inline] pub fn p0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Ethernet MAC Module 0 Power Control"]
    #[inline] pub fn test_p0(&self) -> bool {
        self.p0() != 0
    }

    #[doc="Ethernet MAC Module 0 Power Control"]
    #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pcemac {
    #[inline]
    fn from(other: u32) -> Self {
         Pcemac(other)
    }
}

impl ::core::fmt::Display for Pcemac {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcemac {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p0() != 0 { try!(write!(f, " p0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Timer Peripheral Ready"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Prwd(pub u32);
impl Prwd {
    #[doc="Watchdog Timer 0 Peripheral Ready"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Watchdog Timer 0 Peripheral Ready"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="Watchdog Timer 0 Peripheral Ready"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Watchdog Timer 1 Peripheral Ready"]
    #[inline] pub fn r1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Watchdog Timer 1 Peripheral Ready"]
    #[inline] pub fn test_r1(&self) -> bool {
        self.r1() != 0
    }

    #[doc="Watchdog Timer 1 Peripheral Ready"]
    #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Prwd {
    #[inline]
    fn from(other: u32) -> Self {
         Prwd(other)
    }
}

impl ::core::fmt::Display for Prwd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Prwd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        if self.r1() != 0 { try!(write!(f, " r1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="16/32-Bit General-Purpose Timer Peripheral Ready"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Prtimer(pub u32);
impl Prtimer {
    #[doc="16/32-Bit General-Purpose Timer 0 Peripheral Ready"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="16/32-Bit General-Purpose Timer 0 Peripheral Ready"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 0 Peripheral Ready"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 1 Peripheral Ready"]
    #[inline] pub fn r1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="16/32-Bit General-Purpose Timer 1 Peripheral Ready"]
    #[inline] pub fn test_r1(&self) -> bool {
        self.r1() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 1 Peripheral Ready"]
    #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 2 Peripheral Ready"]
    #[inline] pub fn r2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="16/32-Bit General-Purpose Timer 2 Peripheral Ready"]
    #[inline] pub fn test_r2(&self) -> bool {
        self.r2() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 2 Peripheral Ready"]
    #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 3 Peripheral Ready"]
    #[inline] pub fn r3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="16/32-Bit General-Purpose Timer 3 Peripheral Ready"]
    #[inline] pub fn test_r3(&self) -> bool {
        self.r3() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 3 Peripheral Ready"]
    #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 4 Peripheral Ready"]
    #[inline] pub fn r4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="16/32-Bit General-Purpose Timer 4 Peripheral Ready"]
    #[inline] pub fn test_r4(&self) -> bool {
        self.r4() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 4 Peripheral Ready"]
    #[inline] pub fn set_r4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 5 Peripheral Ready"]
    #[inline] pub fn r5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="16/32-Bit General-Purpose Timer 5 Peripheral Ready"]
    #[inline] pub fn test_r5(&self) -> bool {
        self.r5() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 5 Peripheral Ready"]
    #[inline] pub fn set_r5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 6 Peripheral Ready"]
    #[inline] pub fn r6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="16/32-Bit General-Purpose Timer 6 Peripheral Ready"]
    #[inline] pub fn test_r6(&self) -> bool {
        self.r6() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 6 Peripheral Ready"]
    #[inline] pub fn set_r6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="16/32-Bit General-Purpose Timer 7 Peripheral Ready"]
    #[inline] pub fn r7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="16/32-Bit General-Purpose Timer 7 Peripheral Ready"]
    #[inline] pub fn test_r7(&self) -> bool {
        self.r7() != 0
    }

    #[doc="16/32-Bit General-Purpose Timer 7 Peripheral Ready"]
    #[inline] pub fn set_r7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Prtimer {
    #[inline]
    fn from(other: u32) -> Self {
         Prtimer(other)
    }
}

impl ::core::fmt::Display for Prtimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Prtimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        if self.r1() != 0 { try!(write!(f, " r1"))}
        if self.r2() != 0 { try!(write!(f, " r2"))}
        if self.r3() != 0 { try!(write!(f, " r3"))}
        if self.r4() != 0 { try!(write!(f, " r4"))}
        if self.r5() != 0 { try!(write!(f, " r5"))}
        if self.r6() != 0 { try!(write!(f, " r6"))}
        if self.r7() != 0 { try!(write!(f, " r7"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="General-Purpose Input/Output Peripheral Ready"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Prgpio(pub u32);
impl Prgpio {
    #[doc="GPIO Port A Peripheral Ready"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="GPIO Port A Peripheral Ready"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="GPIO Port A Peripheral Ready"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="GPIO Port B Peripheral Ready"]
    #[inline] pub fn r1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="GPIO Port B Peripheral Ready"]
    #[inline] pub fn test_r1(&self) -> bool {
        self.r1() != 0
    }

    #[doc="GPIO Port B Peripheral Ready"]
    #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="GPIO Port C Peripheral Ready"]
    #[inline] pub fn r2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="GPIO Port C Peripheral Ready"]
    #[inline] pub fn test_r2(&self) -> bool {
        self.r2() != 0
    }

    #[doc="GPIO Port C Peripheral Ready"]
    #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="GPIO Port D Peripheral Ready"]
    #[inline] pub fn r3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="GPIO Port D Peripheral Ready"]
    #[inline] pub fn test_r3(&self) -> bool {
        self.r3() != 0
    }

    #[doc="GPIO Port D Peripheral Ready"]
    #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPIO Port E Peripheral Ready"]
    #[inline] pub fn r4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="GPIO Port E Peripheral Ready"]
    #[inline] pub fn test_r4(&self) -> bool {
        self.r4() != 0
    }

    #[doc="GPIO Port E Peripheral Ready"]
    #[inline] pub fn set_r4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPIO Port F Peripheral Ready"]
    #[inline] pub fn r5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="GPIO Port F Peripheral Ready"]
    #[inline] pub fn test_r5(&self) -> bool {
        self.r5() != 0
    }

    #[doc="GPIO Port F Peripheral Ready"]
    #[inline] pub fn set_r5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="GPIO Port G Peripheral Ready"]
    #[inline] pub fn r6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="GPIO Port G Peripheral Ready"]
    #[inline] pub fn test_r6(&self) -> bool {
        self.r6() != 0
    }

    #[doc="GPIO Port G Peripheral Ready"]
    #[inline] pub fn set_r6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="GPIO Port H Peripheral Ready"]
    #[inline] pub fn r7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="GPIO Port H Peripheral Ready"]
    #[inline] pub fn test_r7(&self) -> bool {
        self.r7() != 0
    }

    #[doc="GPIO Port H Peripheral Ready"]
    #[inline] pub fn set_r7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="GPIO Port J Peripheral Ready"]
    #[inline] pub fn r8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="GPIO Port J Peripheral Ready"]
    #[inline] pub fn test_r8(&self) -> bool {
        self.r8() != 0
    }

    #[doc="GPIO Port J Peripheral Ready"]
    #[inline] pub fn set_r8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="GPIO Port K Peripheral Ready"]
    #[inline] pub fn r9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="GPIO Port K Peripheral Ready"]
    #[inline] pub fn test_r9(&self) -> bool {
        self.r9() != 0
    }

    #[doc="GPIO Port K Peripheral Ready"]
    #[inline] pub fn set_r9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="GPIO Port L Peripheral Ready"]
    #[inline] pub fn r10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="GPIO Port L Peripheral Ready"]
    #[inline] pub fn test_r10(&self) -> bool {
        self.r10() != 0
    }

    #[doc="GPIO Port L Peripheral Ready"]
    #[inline] pub fn set_r10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="GPIO Port M Peripheral Ready"]
    #[inline] pub fn r11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="GPIO Port M Peripheral Ready"]
    #[inline] pub fn test_r11(&self) -> bool {
        self.r11() != 0
    }

    #[doc="GPIO Port M Peripheral Ready"]
    #[inline] pub fn set_r11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="GPIO Port N Peripheral Ready"]
    #[inline] pub fn r12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="GPIO Port N Peripheral Ready"]
    #[inline] pub fn test_r12(&self) -> bool {
        self.r12() != 0
    }

    #[doc="GPIO Port N Peripheral Ready"]
    #[inline] pub fn set_r12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="GPIO Port P Peripheral Ready"]
    #[inline] pub fn r13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="GPIO Port P Peripheral Ready"]
    #[inline] pub fn test_r13(&self) -> bool {
        self.r13() != 0
    }

    #[doc="GPIO Port P Peripheral Ready"]
    #[inline] pub fn set_r13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="GPIO Port Q Peripheral Ready"]
    #[inline] pub fn r14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="GPIO Port Q Peripheral Ready"]
    #[inline] pub fn test_r14(&self) -> bool {
        self.r14() != 0
    }

    #[doc="GPIO Port Q Peripheral Ready"]
    #[inline] pub fn set_r14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

}

impl From<u32> for Prgpio {
    #[inline]
    fn from(other: u32) -> Self {
         Prgpio(other)
    }
}

impl ::core::fmt::Display for Prgpio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Prgpio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        if self.r1() != 0 { try!(write!(f, " r1"))}
        if self.r2() != 0 { try!(write!(f, " r2"))}
        if self.r3() != 0 { try!(write!(f, " r3"))}
        if self.r4() != 0 { try!(write!(f, " r4"))}
        if self.r5() != 0 { try!(write!(f, " r5"))}
        if self.r6() != 0 { try!(write!(f, " r6"))}
        if self.r7() != 0 { try!(write!(f, " r7"))}
        if self.r8() != 0 { try!(write!(f, " r8"))}
        if self.r9() != 0 { try!(write!(f, " r9"))}
        if self.r10() != 0 { try!(write!(f, " r10"))}
        if self.r11() != 0 { try!(write!(f, " r11"))}
        if self.r12() != 0 { try!(write!(f, " r12"))}
        if self.r13() != 0 { try!(write!(f, " r13"))}
        if self.r14() != 0 { try!(write!(f, " r14"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Micro Direct Memory Access Peripheral Ready"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Prdma(pub u32);
impl Prdma {
    #[doc="uDMA Module Peripheral Ready"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="uDMA Module Peripheral Ready"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="uDMA Module Peripheral Ready"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Prdma {
    #[inline]
    fn from(other: u32) -> Self {
         Prdma(other)
    }
}

impl ::core::fmt::Display for Prdma {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Prdma {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="EPI Peripheral Ready"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Prepi(pub u32);
impl Prepi {
    #[doc="EPI Module Peripheral Ready"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="EPI Module Peripheral Ready"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="EPI Module Peripheral Ready"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Prepi {
    #[inline]
    fn from(other: u32) -> Self {
         Prepi(other)
    }
}

impl ::core::fmt::Display for Prepi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Prepi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Hibernation Peripheral Ready"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Prhib(pub u32);
impl Prhib {
    #[doc="Hibernation Module Peripheral Ready"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Hibernation Module Peripheral Ready"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="Hibernation Module Peripheral Ready"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Prhib {
    #[inline]
    fn from(other: u32) -> Self {
         Prhib(other)
    }
}

impl ::core::fmt::Display for Prhib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Prhib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Universal Asynchronous Receiver/Transmitter Peripheral Ready"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pruart(pub u32);
impl Pruart {
    #[doc="UART Module 0 Peripheral Ready"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="UART Module 0 Peripheral Ready"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="UART Module 0 Peripheral Ready"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="UART Module 1 Peripheral Ready"]
    #[inline] pub fn r1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="UART Module 1 Peripheral Ready"]
    #[inline] pub fn test_r1(&self) -> bool {
        self.r1() != 0
    }

    #[doc="UART Module 1 Peripheral Ready"]
    #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="UART Module 2 Peripheral Ready"]
    #[inline] pub fn r2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="UART Module 2 Peripheral Ready"]
    #[inline] pub fn test_r2(&self) -> bool {
        self.r2() != 0
    }

    #[doc="UART Module 2 Peripheral Ready"]
    #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="UART Module 3 Peripheral Ready"]
    #[inline] pub fn r3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="UART Module 3 Peripheral Ready"]
    #[inline] pub fn test_r3(&self) -> bool {
        self.r3() != 0
    }

    #[doc="UART Module 3 Peripheral Ready"]
    #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="UART Module 4 Peripheral Ready"]
    #[inline] pub fn r4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="UART Module 4 Peripheral Ready"]
    #[inline] pub fn test_r4(&self) -> bool {
        self.r4() != 0
    }

    #[doc="UART Module 4 Peripheral Ready"]
    #[inline] pub fn set_r4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="UART Module 5 Peripheral Ready"]
    #[inline] pub fn r5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="UART Module 5 Peripheral Ready"]
    #[inline] pub fn test_r5(&self) -> bool {
        self.r5() != 0
    }

    #[doc="UART Module 5 Peripheral Ready"]
    #[inline] pub fn set_r5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="UART Module 6 Peripheral Ready"]
    #[inline] pub fn r6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="UART Module 6 Peripheral Ready"]
    #[inline] pub fn test_r6(&self) -> bool {
        self.r6() != 0
    }

    #[doc="UART Module 6 Peripheral Ready"]
    #[inline] pub fn set_r6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="UART Module 7 Peripheral Ready"]
    #[inline] pub fn r7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="UART Module 7 Peripheral Ready"]
    #[inline] pub fn test_r7(&self) -> bool {
        self.r7() != 0
    }

    #[doc="UART Module 7 Peripheral Ready"]
    #[inline] pub fn set_r7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Pruart {
    #[inline]
    fn from(other: u32) -> Self {
         Pruart(other)
    }
}

impl ::core::fmt::Display for Pruart {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pruart {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        if self.r1() != 0 { try!(write!(f, " r1"))}
        if self.r2() != 0 { try!(write!(f, " r2"))}
        if self.r3() != 0 { try!(write!(f, " r3"))}
        if self.r4() != 0 { try!(write!(f, " r4"))}
        if self.r5() != 0 { try!(write!(f, " r5"))}
        if self.r6() != 0 { try!(write!(f, " r6"))}
        if self.r7() != 0 { try!(write!(f, " r7"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Synchronous Serial Interface Peripheral Ready"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Prssi(pub u32);
impl Prssi {
    #[doc="SSI Module 0 Peripheral Ready"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="SSI Module 0 Peripheral Ready"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="SSI Module 0 Peripheral Ready"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SSI Module 1 Peripheral Ready"]
    #[inline] pub fn r1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="SSI Module 1 Peripheral Ready"]
    #[inline] pub fn test_r1(&self) -> bool {
        self.r1() != 0
    }

    #[doc="SSI Module 1 Peripheral Ready"]
    #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="SSI Module 2 Peripheral Ready"]
    #[inline] pub fn r2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="SSI Module 2 Peripheral Ready"]
    #[inline] pub fn test_r2(&self) -> bool {
        self.r2() != 0
    }

    #[doc="SSI Module 2 Peripheral Ready"]
    #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SSI Module 3 Peripheral Ready"]
    #[inline] pub fn r3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="SSI Module 3 Peripheral Ready"]
    #[inline] pub fn test_r3(&self) -> bool {
        self.r3() != 0
    }

    #[doc="SSI Module 3 Peripheral Ready"]
    #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Prssi {
    #[inline]
    fn from(other: u32) -> Self {
         Prssi(other)
    }
}

impl ::core::fmt::Display for Prssi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Prssi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        if self.r1() != 0 { try!(write!(f, " r1"))}
        if self.r2() != 0 { try!(write!(f, " r2"))}
        if self.r3() != 0 { try!(write!(f, " r3"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Inter-Integrated Circuit Peripheral Ready"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pri2c(pub u32);
impl Pri2c {
    #[doc="I2C Module 0 Peripheral Ready"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="I2C Module 0 Peripheral Ready"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="I2C Module 0 Peripheral Ready"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="I2C Module 1 Peripheral Ready"]
    #[inline] pub fn r1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="I2C Module 1 Peripheral Ready"]
    #[inline] pub fn test_r1(&self) -> bool {
        self.r1() != 0
    }

    #[doc="I2C Module 1 Peripheral Ready"]
    #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="I2C Module 2 Peripheral Ready"]
    #[inline] pub fn r2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="I2C Module 2 Peripheral Ready"]
    #[inline] pub fn test_r2(&self) -> bool {
        self.r2() != 0
    }

    #[doc="I2C Module 2 Peripheral Ready"]
    #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="I2C Module 3 Peripheral Ready"]
    #[inline] pub fn r3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="I2C Module 3 Peripheral Ready"]
    #[inline] pub fn test_r3(&self) -> bool {
        self.r3() != 0
    }

    #[doc="I2C Module 3 Peripheral Ready"]
    #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="I2C Module 4 Peripheral Ready"]
    #[inline] pub fn r4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="I2C Module 4 Peripheral Ready"]
    #[inline] pub fn test_r4(&self) -> bool {
        self.r4() != 0
    }

    #[doc="I2C Module 4 Peripheral Ready"]
    #[inline] pub fn set_r4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="I2C Module 5 Peripheral Ready"]
    #[inline] pub fn r5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="I2C Module 5 Peripheral Ready"]
    #[inline] pub fn test_r5(&self) -> bool {
        self.r5() != 0
    }

    #[doc="I2C Module 5 Peripheral Ready"]
    #[inline] pub fn set_r5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="I2C Module 6 Peripheral Ready"]
    #[inline] pub fn r6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="I2C Module 6 Peripheral Ready"]
    #[inline] pub fn test_r6(&self) -> bool {
        self.r6() != 0
    }

    #[doc="I2C Module 6 Peripheral Ready"]
    #[inline] pub fn set_r6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="I2C Module 7 Peripheral Ready"]
    #[inline] pub fn r7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="I2C Module 7 Peripheral Ready"]
    #[inline] pub fn test_r7(&self) -> bool {
        self.r7() != 0
    }

    #[doc="I2C Module 7 Peripheral Ready"]
    #[inline] pub fn set_r7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="I2C Module 8 Peripheral Ready"]
    #[inline] pub fn r8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="I2C Module 8 Peripheral Ready"]
    #[inline] pub fn test_r8(&self) -> bool {
        self.r8() != 0
    }

    #[doc="I2C Module 8 Peripheral Ready"]
    #[inline] pub fn set_r8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="I2C Module 9 Peripheral Ready"]
    #[inline] pub fn r9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="I2C Module 9 Peripheral Ready"]
    #[inline] pub fn test_r9(&self) -> bool {
        self.r9() != 0
    }

    #[doc="I2C Module 9 Peripheral Ready"]
    #[inline] pub fn set_r9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

}

impl From<u32> for Pri2c {
    #[inline]
    fn from(other: u32) -> Self {
         Pri2c(other)
    }
}

impl ::core::fmt::Display for Pri2c {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pri2c {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        if self.r1() != 0 { try!(write!(f, " r1"))}
        if self.r2() != 0 { try!(write!(f, " r2"))}
        if self.r3() != 0 { try!(write!(f, " r3"))}
        if self.r4() != 0 { try!(write!(f, " r4"))}
        if self.r5() != 0 { try!(write!(f, " r5"))}
        if self.r6() != 0 { try!(write!(f, " r6"))}
        if self.r7() != 0 { try!(write!(f, " r7"))}
        if self.r8() != 0 { try!(write!(f, " r8"))}
        if self.r9() != 0 { try!(write!(f, " r9"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Universal Serial Bus Peripheral Ready"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Prusb(pub u32);
impl Prusb {
    #[doc="USB Module Peripheral Ready"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="USB Module Peripheral Ready"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="USB Module Peripheral Ready"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Prusb {
    #[inline]
    fn from(other: u32) -> Self {
         Prusb(other)
    }
}

impl ::core::fmt::Display for Prusb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Prusb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PHY Peripheral Ready"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Prephy(pub u32);
impl Prephy {
    #[doc="Ethernet PHY Module Peripheral Ready"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Ethernet PHY Module Peripheral Ready"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="Ethernet PHY Module Peripheral Ready"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Prephy {
    #[inline]
    fn from(other: u32) -> Self {
         Prephy(other)
    }
}

impl ::core::fmt::Display for Prephy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Prephy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Controller Area Network Peripheral Ready"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Prcan(pub u32);
impl Prcan {
    #[doc="CAN Module 0 Peripheral Ready"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="CAN Module 0 Peripheral Ready"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="CAN Module 0 Peripheral Ready"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="CAN Module 1 Peripheral Ready"]
    #[inline] pub fn r1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="CAN Module 1 Peripheral Ready"]
    #[inline] pub fn test_r1(&self) -> bool {
        self.r1() != 0
    }

    #[doc="CAN Module 1 Peripheral Ready"]
    #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Prcan {
    #[inline]
    fn from(other: u32) -> Self {
         Prcan(other)
    }
}

impl ::core::fmt::Display for Prcan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Prcan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        if self.r1() != 0 { try!(write!(f, " r1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Analog-to-Digital Converter Peripheral Ready"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pradc(pub u32);
impl Pradc {
    #[doc="ADC Module 0 Peripheral Ready"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="ADC Module 0 Peripheral Ready"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="ADC Module 0 Peripheral Ready"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="ADC Module 1 Peripheral Ready"]
    #[inline] pub fn r1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="ADC Module 1 Peripheral Ready"]
    #[inline] pub fn test_r1(&self) -> bool {
        self.r1() != 0
    }

    #[doc="ADC Module 1 Peripheral Ready"]
    #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Pradc {
    #[inline]
    fn from(other: u32) -> Self {
         Pradc(other)
    }
}

impl ::core::fmt::Display for Pradc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pradc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        if self.r1() != 0 { try!(write!(f, " r1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Analog Comparator Peripheral Ready"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pracmp(pub u32);
impl Pracmp {
    #[doc="Analog Comparator Module 0 Peripheral Ready"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Analog Comparator Module 0 Peripheral Ready"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="Analog Comparator Module 0 Peripheral Ready"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pracmp {
    #[inline]
    fn from(other: u32) -> Self {
         Pracmp(other)
    }
}

impl ::core::fmt::Display for Pracmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pracmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pulse Width Modulator Peripheral Ready"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Prpwm(pub u32);
impl Prpwm {
    #[doc="PWM Module 0 Peripheral Ready"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="PWM Module 0 Peripheral Ready"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="PWM Module 0 Peripheral Ready"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Prpwm {
    #[inline]
    fn from(other: u32) -> Self {
         Prpwm(other)
    }
}

impl ::core::fmt::Display for Prpwm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Prpwm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Quadrature Encoder Interface Peripheral Ready"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Prqei(pub u32);
impl Prqei {
    #[doc="QEI Module 0 Peripheral Ready"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="QEI Module 0 Peripheral Ready"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="QEI Module 0 Peripheral Ready"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Prqei {
    #[inline]
    fn from(other: u32) -> Self {
         Prqei(other)
    }
}

impl ::core::fmt::Display for Prqei {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Prqei {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="EEPROM Peripheral Ready"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Preeprom(pub u32);
impl Preeprom {
    #[doc="EEPROM Module Peripheral Ready"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="EEPROM Module Peripheral Ready"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="EEPROM Module Peripheral Ready"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Preeprom {
    #[inline]
    fn from(other: u32) -> Self {
         Preeprom(other)
    }
}

impl ::core::fmt::Display for Preeprom {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Preeprom {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC and Cryptographic Modules Peripheral Ready"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Prccm(pub u32);
impl Prccm {
    #[doc="CRC and Cryptographic Modules Peripheral Ready"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="CRC and Cryptographic Modules Peripheral Ready"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="CRC and Cryptographic Modules Peripheral Ready"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Prccm {
    #[inline]
    fn from(other: u32) -> Self {
         Prccm(other)
    }
}

impl ::core::fmt::Display for Prccm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Prccm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Peripheral Ready"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Premac(pub u32);
impl Premac {
    #[doc="Ethernet MAC Module 0 Peripheral Ready"]
    #[inline] pub fn r0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Ethernet MAC Module 0 Peripheral Ready"]
    #[inline] pub fn test_r0(&self) -> bool {
        self.r0() != 0
    }

    #[doc="Ethernet MAC Module 0 Peripheral Ready"]
    #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Premac {
    #[inline]
    fn from(other: u32) -> Self {
         Premac(other)
    }
}

impl ::core::fmt::Display for Premac {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Premac {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0() != 0 { try!(write!(f, " r0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

pub trait Rcgc {
    fn rcgc(&self) -> u32;
    fn set_rcgc(&self, value: u32);
}

impl Sysctl {
    #[inline] pub fn rcgc<P: Rcgc>(&self, p: &P) -> u32 {
        p.rcgc()
    }
    #[inline] pub fn set_rcgc<P: Rcgc>(&self, p: &P, value: u32) {
        p.set_rcgc(value)
    }
}

pub trait Pr {
    fn pr(&self) -> u32;
    fn set_pr(&self, value: u32);
}

impl Sysctl {
    #[inline] pub fn pr<P: Pr>(&self, p: &P) -> u32 {
        p.pr()
    }
    #[inline] pub fn set_pr<P: Pr>(&self, p: &P, value: u32) {
        p.set_pr(value)
    }
}

impl Rcgc for super::watchdog::Watchdog0 {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcwd().r0().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcwd(|r| r.set_r0(value)); }
}

impl Rcgc for super::watchdog::Watchdog1 {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcwd().r1().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcwd(|r| r.set_r1(value)); }
}

impl Rcgc for super::timer::Timer0 {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgctimer().r0().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgctimer(|r| r.set_r0(value)); }
}

impl Rcgc for super::timer::Timer1 {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgctimer().r1().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgctimer(|r| r.set_r1(value)); }
}

impl Rcgc for super::timer::Timer2 {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgctimer().r2().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgctimer(|r| r.set_r2(value)); }
}

impl Rcgc for super::timer::Timer3 {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgctimer().r3().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgctimer(|r| r.set_r3(value)); }
}

impl Rcgc for super::timer::Timer4 {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgctimer().r4().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgctimer(|r| r.set_r4(value)); }
}

impl Rcgc for super::timer::Timer5 {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgctimer().r5().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgctimer(|r| r.set_r5(value)); }
}

impl Rcgc for super::timer::Timer6 {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgctimer().r6().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgctimer(|r| r.set_r6(value)); }
}

impl Rcgc for super::timer::Timer7 {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgctimer().r7().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgctimer(|r| r.set_r7(value)); }
}

impl Rcgc for super::gpio::Gpioa {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r0().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r0(value)); }
}

impl Rcgc for super::gpio::Gpiob {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r1().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r1(value)); }
}

impl Rcgc for super::gpio::Gpioc {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r2().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r2(value)); }
}

impl Rcgc for super::gpio::Gpiod {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r3().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r3(value)); }
}

impl Rcgc for super::gpio::Gpioe {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r4().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r4(value)); }
}

impl Rcgc for super::gpio::Gpiof {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r5().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r5(value)); }
}

impl Rcgc for super::gpio::Gpiog {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r6().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r6(value)); }
}

impl Rcgc for super::gpio::Gpioh {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r7().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r7(value)); }
}

impl Rcgc for super::gpio::Gpioj {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r8().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r8(value)); }
}

impl Rcgc for super::gpio::Gpiok {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r9().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r9(value)); }
}

impl Rcgc for super::gpio::Gpiol {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r10().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r10(value)); }
}

impl Rcgc for super::gpio::Gpiom {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r11().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r11(value)); }
}

impl Rcgc for super::gpio::Gpion {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r12().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r12(value)); }
}

impl Rcgc for super::gpio::Gpiop {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r13().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r13(value)); }
}

impl Rcgc for super::gpio::Gpioq {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r14().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r14(value)); }
}

impl Rcgc for super::udma::Udma {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcdma().r0().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcdma(|r| r.set_r0(value)); }
}

impl Rcgc for super::uart::Uart0 {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcuart().r0().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcuart(|r| r.set_r0(value)); }
}

impl Rcgc for super::uart::Uart1 {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcuart().r1().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcuart(|r| r.set_r1(value)); }
}

impl Rcgc for super::uart::Uart2 {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcuart().r2().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcuart(|r| r.set_r2(value)); }
}

impl Rcgc for super::uart::Uart3 {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcuart().r3().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcuart(|r| r.set_r3(value)); }
}

impl Rcgc for super::uart::Uart4 {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcuart().r4().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcuart(|r| r.set_r4(value)); }
}

impl Rcgc for super::uart::Uart5 {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcuart().r5().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcuart(|r| r.set_r5(value)); }
}

impl Rcgc for super::uart::Uart6 {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcuart().r6().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcuart(|r| r.set_r6(value)); }
}

impl Rcgc for super::uart::Uart7 {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcuart().r7().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcuart(|r| r.set_r7(value)); }
}

impl Rcgc for super::adc::Adc0 {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcadc().r0().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcadc(|r| r.set_r0(value)); }
}

impl Rcgc for super::adc::Adc1 {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcadc().r1().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcadc(|r| r.set_r1(value)); }
}

impl Rcgc for super::pwm::Pwm0 {
    #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcpwm().r0().into() }
    #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcpwm(|r| r.set_r0(value)); }
}

impl Pr for super::watchdog::Watchdog0 {
    #[inline] fn pr(&self) -> u32 { SYSCTL.prwd().r0().into() }
    #[inline] fn set_pr(&self, value: u32) { SYSCTL.with_prwd(|r| r.set_r0(value)); }
}

impl Pr for super::watchdog::Watchdog1 {
    #[inline] fn pr(&self) -> u32 { SYSCTL.prwd().r1().into() }
    #[inline] fn set_pr(&self, value: u32) { SYSCTL.with_prwd(|r| r.set_r1(value)); }
}


