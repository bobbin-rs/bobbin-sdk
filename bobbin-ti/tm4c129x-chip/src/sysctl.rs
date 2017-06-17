pub const SYSCTL: Sysctl = Sysctl(0x400fe000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sysctl(pub u32);
impl Sysctl {
  #[inline] pub fn did0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
  #[inline] pub fn did0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
  #[inline] pub fn did0(&self) -> Did0 { 
     unsafe {
        Did0(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  #[inline] pub fn set_did0(&self, value: Did0) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_did0<F: FnOnce(Did0) -> Did0>(&self, f: F) -> &Self {
     let tmp = self.did0();
     self.set_did0(f(tmp))
  }

  #[inline] pub fn did1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
  #[inline] pub fn did1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
  #[inline] pub fn did1(&self) -> Did1 { 
     unsafe {
        Did1(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
  #[inline] pub fn set_did1(&self, value: Did1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_did1<F: FnOnce(Did1) -> Did1>(&self, f: F) -> &Self {
     let tmp = self.did1();
     self.set_did1(f(tmp))
  }

  #[inline] pub fn ptboctl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x38) as *const u32
  }
  #[inline] pub fn ptboctl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x38) as *mut u32
  }
  #[inline] pub fn ptboctl(&self) -> Ptboctl { 
     unsafe {
        Ptboctl(::core::ptr::read_volatile(((self.0 as usize) + 0x38) as *const u32))
     }
  }
  #[inline] pub fn set_ptboctl(&self, value: Ptboctl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x38) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ptboctl<F: FnOnce(Ptboctl) -> Ptboctl>(&self, f: F) -> &Self {
     let tmp = self.ptboctl();
     self.set_ptboctl(f(tmp))
  }

  #[inline] pub fn ris_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x50) as *const u32
  }
  #[inline] pub fn ris_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x50) as *mut u32
  }
  #[inline] pub fn ris(&self) -> Ris { 
     unsafe {
        Ris(::core::ptr::read_volatile(((self.0 as usize) + 0x50) as *const u32))
     }
  }
  #[inline] pub fn set_ris(&self, value: Ris) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x50) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
     let tmp = self.ris();
     self.set_ris(f(tmp))
  }

  #[inline] pub fn imc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x54) as *const u32
  }
  #[inline] pub fn imc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x54) as *mut u32
  }
  #[inline] pub fn imc(&self) -> Imc { 
     unsafe {
        Imc(::core::ptr::read_volatile(((self.0 as usize) + 0x54) as *const u32))
     }
  }
  #[inline] pub fn set_imc(&self, value: Imc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x54) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_imc<F: FnOnce(Imc) -> Imc>(&self, f: F) -> &Self {
     let tmp = self.imc();
     self.set_imc(f(tmp))
  }

  #[inline] pub fn misc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x58) as *const u32
  }
  #[inline] pub fn misc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x58) as *mut u32
  }
  #[inline] pub fn misc(&self) -> Misc { 
     unsafe {
        Misc(::core::ptr::read_volatile(((self.0 as usize) + 0x58) as *const u32))
     }
  }
  #[inline] pub fn set_misc(&self, value: Misc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x58) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_misc<F: FnOnce(Misc) -> Misc>(&self, f: F) -> &Self {
     let tmp = self.misc();
     self.set_misc(f(tmp))
  }

  #[inline] pub fn resc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x5c) as *const u32
  }
  #[inline] pub fn resc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x5c) as *mut u32
  }
  #[inline] pub fn resc(&self) -> Resc { 
     unsafe {
        Resc(::core::ptr::read_volatile(((self.0 as usize) + 0x5c) as *const u32))
     }
  }
  #[inline] pub fn set_resc(&self, value: Resc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x5c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_resc<F: FnOnce(Resc) -> Resc>(&self, f: F) -> &Self {
     let tmp = self.resc();
     self.set_resc(f(tmp))
  }

  #[inline] pub fn pwrtc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x60) as *const u32
  }
  #[inline] pub fn pwrtc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x60) as *mut u32
  }
  #[inline] pub fn pwrtc(&self) -> Pwrtc { 
     unsafe {
        Pwrtc(::core::ptr::read_volatile(((self.0 as usize) + 0x60) as *const u32))
     }
  }
  #[inline] pub fn set_pwrtc(&self, value: Pwrtc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x60) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pwrtc<F: FnOnce(Pwrtc) -> Pwrtc>(&self, f: F) -> &Self {
     let tmp = self.pwrtc();
     self.set_pwrtc(f(tmp))
  }

  #[inline] pub fn nmic_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x64) as *const u32
  }
  #[inline] pub fn nmic_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x64) as *mut u32
  }
  #[inline] pub fn nmic(&self) -> Nmic { 
     unsafe {
        Nmic(::core::ptr::read_volatile(((self.0 as usize) + 0x64) as *const u32))
     }
  }
  #[inline] pub fn set_nmic(&self, value: Nmic) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x64) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_nmic<F: FnOnce(Nmic) -> Nmic>(&self, f: F) -> &Self {
     let tmp = self.nmic();
     self.set_nmic(f(tmp))
  }

  #[inline] pub fn moscctl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x7c) as *const u32
  }
  #[inline] pub fn moscctl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x7c) as *mut u32
  }
  #[inline] pub fn moscctl(&self) -> Moscctl { 
     unsafe {
        Moscctl(::core::ptr::read_volatile(((self.0 as usize) + 0x7c) as *const u32))
     }
  }
  #[inline] pub fn set_moscctl(&self, value: Moscctl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x7c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_moscctl<F: FnOnce(Moscctl) -> Moscctl>(&self, f: F) -> &Self {
     let tmp = self.moscctl();
     self.set_moscctl(f(tmp))
  }

  #[inline] pub fn rsclkcfg_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xb0) as *const u32
  }
  #[inline] pub fn rsclkcfg_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xb0) as *mut u32
  }
  #[inline] pub fn rsclkcfg(&self) -> Rsclkcfg { 
     unsafe {
        Rsclkcfg(::core::ptr::read_volatile(((self.0 as usize) + 0xb0) as *const u32))
     }
  }
  #[inline] pub fn set_rsclkcfg(&self, value: Rsclkcfg) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xb0) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_rsclkcfg<F: FnOnce(Rsclkcfg) -> Rsclkcfg>(&self, f: F) -> &Self {
     let tmp = self.rsclkcfg();
     self.set_rsclkcfg(f(tmp))
  }

  #[inline] pub fn memtim0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc0) as *const u32
  }
  #[inline] pub fn memtim0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc0) as *mut u32
  }
  #[inline] pub fn memtim0(&self) -> Memtim0 { 
     unsafe {
        Memtim0(::core::ptr::read_volatile(((self.0 as usize) + 0xc0) as *const u32))
     }
  }
  #[inline] pub fn set_memtim0(&self, value: Memtim0) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc0) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_memtim0<F: FnOnce(Memtim0) -> Memtim0>(&self, f: F) -> &Self {
     let tmp = self.memtim0();
     self.set_memtim0(f(tmp))
  }

  #[inline] pub fn altclkcfg_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x138) as *const u32
  }
  #[inline] pub fn altclkcfg_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x138) as *mut u32
  }
  #[inline] pub fn altclkcfg(&self) -> Altclkcfg { 
     unsafe {
        Altclkcfg(::core::ptr::read_volatile(((self.0 as usize) + 0x138) as *const u32))
     }
  }
  #[inline] pub fn set_altclkcfg(&self, value: Altclkcfg) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x138) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_altclkcfg<F: FnOnce(Altclkcfg) -> Altclkcfg>(&self, f: F) -> &Self {
     let tmp = self.altclkcfg();
     self.set_altclkcfg(f(tmp))
  }

  #[inline] pub fn dsclkcfg_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x144) as *const u32
  }
  #[inline] pub fn dsclkcfg_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x144) as *mut u32
  }
  #[inline] pub fn dsclkcfg(&self) -> Dsclkcfg { 
     unsafe {
        Dsclkcfg(::core::ptr::read_volatile(((self.0 as usize) + 0x144) as *const u32))
     }
  }
  #[inline] pub fn set_dsclkcfg(&self, value: Dsclkcfg) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x144) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dsclkcfg<F: FnOnce(Dsclkcfg) -> Dsclkcfg>(&self, f: F) -> &Self {
     let tmp = self.dsclkcfg();
     self.set_dsclkcfg(f(tmp))
  }

  #[inline] pub fn divsclk_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x148) as *const u32
  }
  #[inline] pub fn divsclk_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x148) as *mut u32
  }
  #[inline] pub fn divsclk(&self) -> Divsclk { 
     unsafe {
        Divsclk(::core::ptr::read_volatile(((self.0 as usize) + 0x148) as *const u32))
     }
  }
  #[inline] pub fn set_divsclk(&self, value: Divsclk) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x148) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_divsclk<F: FnOnce(Divsclk) -> Divsclk>(&self, f: F) -> &Self {
     let tmp = self.divsclk();
     self.set_divsclk(f(tmp))
  }

  #[inline] pub fn sysprop_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14c) as *const u32
  }
  #[inline] pub fn sysprop_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14c) as *mut u32
  }
  #[inline] pub fn sysprop(&self) -> Sysprop { 
     unsafe {
        Sysprop(::core::ptr::read_volatile(((self.0 as usize) + 0x14c) as *const u32))
     }
  }
  #[inline] pub fn set_sysprop(&self, value: Sysprop) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_sysprop<F: FnOnce(Sysprop) -> Sysprop>(&self, f: F) -> &Self {
     let tmp = self.sysprop();
     self.set_sysprop(f(tmp))
  }

  #[inline] pub fn piosccal_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x150) as *const u32
  }
  #[inline] pub fn piosccal_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x150) as *mut u32
  }
  #[inline] pub fn piosccal(&self) -> Piosccal { 
     unsafe {
        Piosccal(::core::ptr::read_volatile(((self.0 as usize) + 0x150) as *const u32))
     }
  }
  #[inline] pub fn set_piosccal(&self, value: Piosccal) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x150) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_piosccal<F: FnOnce(Piosccal) -> Piosccal>(&self, f: F) -> &Self {
     let tmp = self.piosccal();
     self.set_piosccal(f(tmp))
  }

  #[inline] pub fn pioscstat_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x154) as *const u32
  }
  #[inline] pub fn pioscstat_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x154) as *mut u32
  }
  #[inline] pub fn pioscstat(&self) -> Pioscstat { 
     unsafe {
        Pioscstat(::core::ptr::read_volatile(((self.0 as usize) + 0x154) as *const u32))
     }
  }
  #[inline] pub fn set_pioscstat(&self, value: Pioscstat) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x154) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pioscstat<F: FnOnce(Pioscstat) -> Pioscstat>(&self, f: F) -> &Self {
     let tmp = self.pioscstat();
     self.set_pioscstat(f(tmp))
  }

  #[inline] pub fn pllfreq0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x160) as *const u32
  }
  #[inline] pub fn pllfreq0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x160) as *mut u32
  }
  #[inline] pub fn pllfreq0(&self) -> Pllfreq0 { 
     unsafe {
        Pllfreq0(::core::ptr::read_volatile(((self.0 as usize) + 0x160) as *const u32))
     }
  }
  #[inline] pub fn set_pllfreq0(&self, value: Pllfreq0) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x160) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pllfreq0<F: FnOnce(Pllfreq0) -> Pllfreq0>(&self, f: F) -> &Self {
     let tmp = self.pllfreq0();
     self.set_pllfreq0(f(tmp))
  }

  #[inline] pub fn pllfreq1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x164) as *const u32
  }
  #[inline] pub fn pllfreq1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x164) as *mut u32
  }
  #[inline] pub fn pllfreq1(&self) -> Pllfreq1 { 
     unsafe {
        Pllfreq1(::core::ptr::read_volatile(((self.0 as usize) + 0x164) as *const u32))
     }
  }
  #[inline] pub fn set_pllfreq1(&self, value: Pllfreq1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x164) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pllfreq1<F: FnOnce(Pllfreq1) -> Pllfreq1>(&self, f: F) -> &Self {
     let tmp = self.pllfreq1();
     self.set_pllfreq1(f(tmp))
  }

  #[inline] pub fn pllstat_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x168) as *const u32
  }
  #[inline] pub fn pllstat_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x168) as *mut u32
  }
  #[inline] pub fn pllstat(&self) -> Pllstat { 
     unsafe {
        Pllstat(::core::ptr::read_volatile(((self.0 as usize) + 0x168) as *const u32))
     }
  }
  #[inline] pub fn set_pllstat(&self, value: Pllstat) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x168) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pllstat<F: FnOnce(Pllstat) -> Pllstat>(&self, f: F) -> &Self {
     let tmp = self.pllstat();
     self.set_pllstat(f(tmp))
  }

  #[inline] pub fn slppwrcfg_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x188) as *const u32
  }
  #[inline] pub fn slppwrcfg_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x188) as *mut u32
  }
  #[inline] pub fn slppwrcfg(&self) -> Slppwrcfg { 
     unsafe {
        Slppwrcfg(::core::ptr::read_volatile(((self.0 as usize) + 0x188) as *const u32))
     }
  }
  #[inline] pub fn set_slppwrcfg(&self, value: Slppwrcfg) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x188) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_slppwrcfg<F: FnOnce(Slppwrcfg) -> Slppwrcfg>(&self, f: F) -> &Self {
     let tmp = self.slppwrcfg();
     self.set_slppwrcfg(f(tmp))
  }

  #[inline] pub fn dslppwrcfg_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18c) as *const u32
  }
  #[inline] pub fn dslppwrcfg_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18c) as *mut u32
  }
  #[inline] pub fn dslppwrcfg(&self) -> Dslppwrcfg { 
     unsafe {
        Dslppwrcfg(::core::ptr::read_volatile(((self.0 as usize) + 0x18c) as *const u32))
     }
  }
  #[inline] pub fn set_dslppwrcfg(&self, value: Dslppwrcfg) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dslppwrcfg<F: FnOnce(Dslppwrcfg) -> Dslppwrcfg>(&self, f: F) -> &Self {
     let tmp = self.dslppwrcfg();
     self.set_dslppwrcfg(f(tmp))
  }

  #[inline] pub fn nvmstat_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1a0) as *const u32
  }
  #[inline] pub fn nvmstat_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1a0) as *mut u32
  }
  #[inline] pub fn nvmstat(&self) -> Nvmstat { 
     unsafe {
        Nvmstat(::core::ptr::read_volatile(((self.0 as usize) + 0x1a0) as *const u32))
     }
  }
  #[inline] pub fn set_nvmstat(&self, value: Nvmstat) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1a0) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_nvmstat<F: FnOnce(Nvmstat) -> Nvmstat>(&self, f: F) -> &Self {
     let tmp = self.nvmstat();
     self.set_nvmstat(f(tmp))
  }

  #[inline] pub fn ldospctl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1b4) as *const u32
  }
  #[inline] pub fn ldospctl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1b4) as *mut u32
  }
  #[inline] pub fn ldospctl(&self) -> Ldospctl { 
     unsafe {
        Ldospctl(::core::ptr::read_volatile(((self.0 as usize) + 0x1b4) as *const u32))
     }
  }
  #[inline] pub fn set_ldospctl(&self, value: Ldospctl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1b4) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ldospctl<F: FnOnce(Ldospctl) -> Ldospctl>(&self, f: F) -> &Self {
     let tmp = self.ldospctl();
     self.set_ldospctl(f(tmp))
  }

  #[inline] pub fn ldodpctl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1bc) as *const u32
  }
  #[inline] pub fn ldodpctl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1bc) as *mut u32
  }
  #[inline] pub fn ldodpctl(&self) -> Ldodpctl { 
     unsafe {
        Ldodpctl(::core::ptr::read_volatile(((self.0 as usize) + 0x1bc) as *const u32))
     }
  }
  #[inline] pub fn set_ldodpctl(&self, value: Ldodpctl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1bc) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ldodpctl<F: FnOnce(Ldodpctl) -> Ldodpctl>(&self, f: F) -> &Self {
     let tmp = self.ldodpctl();
     self.set_ldodpctl(f(tmp))
  }

  #[inline] pub fn resbehavctl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1d8) as *const u32
  }
  #[inline] pub fn resbehavctl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1d8) as *mut u32
  }
  #[inline] pub fn resbehavctl(&self) -> Resbehavctl { 
     unsafe {
        Resbehavctl(::core::ptr::read_volatile(((self.0 as usize) + 0x1d8) as *const u32))
     }
  }
  #[inline] pub fn set_resbehavctl(&self, value: Resbehavctl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1d8) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_resbehavctl<F: FnOnce(Resbehavctl) -> Resbehavctl>(&self, f: F) -> &Self {
     let tmp = self.resbehavctl();
     self.set_resbehavctl(f(tmp))
  }

  #[inline] pub fn hssr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1f4) as *const u32
  }
  #[inline] pub fn hssr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1f4) as *mut u32
  }
  #[inline] pub fn hssr(&self) -> Hssr { 
     unsafe {
        Hssr(::core::ptr::read_volatile(((self.0 as usize) + 0x1f4) as *const u32))
     }
  }
  #[inline] pub fn set_hssr(&self, value: Hssr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1f4) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_hssr<F: FnOnce(Hssr) -> Hssr>(&self, f: F) -> &Self {
     let tmp = self.hssr();
     self.set_hssr(f(tmp))
  }

  #[inline] pub fn usbpds_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x280) as *const u32
  }
  #[inline] pub fn usbpds_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x280) as *mut u32
  }
  #[inline] pub fn usbpds(&self) -> Usbpds { 
     unsafe {
        Usbpds(::core::ptr::read_volatile(((self.0 as usize) + 0x280) as *const u32))
     }
  }
  #[inline] pub fn set_usbpds(&self, value: Usbpds) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x280) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_usbpds<F: FnOnce(Usbpds) -> Usbpds>(&self, f: F) -> &Self {
     let tmp = self.usbpds();
     self.set_usbpds(f(tmp))
  }

  #[inline] pub fn usbmpc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x284) as *const u32
  }
  #[inline] pub fn usbmpc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x284) as *mut u32
  }
  #[inline] pub fn usbmpc(&self) -> Usbmpc { 
     unsafe {
        Usbmpc(::core::ptr::read_volatile(((self.0 as usize) + 0x284) as *const u32))
     }
  }
  #[inline] pub fn set_usbmpc(&self, value: Usbmpc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x284) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_usbmpc<F: FnOnce(Usbmpc) -> Usbmpc>(&self, f: F) -> &Self {
     let tmp = self.usbmpc();
     self.set_usbmpc(f(tmp))
  }

  #[inline] pub fn emacpds_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x288) as *const u32
  }
  #[inline] pub fn emacpds_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x288) as *mut u32
  }
  #[inline] pub fn emacpds(&self) -> Emacpds { 
     unsafe {
        Emacpds(::core::ptr::read_volatile(((self.0 as usize) + 0x288) as *const u32))
     }
  }
  #[inline] pub fn set_emacpds(&self, value: Emacpds) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x288) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_emacpds<F: FnOnce(Emacpds) -> Emacpds>(&self, f: F) -> &Self {
     let tmp = self.emacpds();
     self.set_emacpds(f(tmp))
  }

  #[inline] pub fn emacmpc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x28c) as *const u32
  }
  #[inline] pub fn emacmpc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x28c) as *mut u32
  }
  #[inline] pub fn emacmpc(&self) -> Emacmpc { 
     unsafe {
        Emacmpc(::core::ptr::read_volatile(((self.0 as usize) + 0x28c) as *const u32))
     }
  }
  #[inline] pub fn set_emacmpc(&self, value: Emacmpc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x28c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_emacmpc<F: FnOnce(Emacmpc) -> Emacmpc>(&self, f: F) -> &Self {
     let tmp = self.emacmpc();
     self.set_emacmpc(f(tmp))
  }

  #[inline] pub fn ppwd_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x300) as *const u32
  }
  #[inline] pub fn ppwd_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x300) as *mut u32
  }
  #[inline] pub fn ppwd(&self) -> Ppwd { 
     unsafe {
        Ppwd(::core::ptr::read_volatile(((self.0 as usize) + 0x300) as *const u32))
     }
  }
  #[inline] pub fn set_ppwd(&self, value: Ppwd) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x300) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ppwd<F: FnOnce(Ppwd) -> Ppwd>(&self, f: F) -> &Self {
     let tmp = self.ppwd();
     self.set_ppwd(f(tmp))
  }

  #[inline] pub fn pptimer_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x304) as *const u32
  }
  #[inline] pub fn pptimer_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x304) as *mut u32
  }
  #[inline] pub fn pptimer(&self) -> Pptimer { 
     unsafe {
        Pptimer(::core::ptr::read_volatile(((self.0 as usize) + 0x304) as *const u32))
     }
  }
  #[inline] pub fn set_pptimer(&self, value: Pptimer) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x304) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pptimer<F: FnOnce(Pptimer) -> Pptimer>(&self, f: F) -> &Self {
     let tmp = self.pptimer();
     self.set_pptimer(f(tmp))
  }

  #[inline] pub fn ppgpio_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x308) as *const u32
  }
  #[inline] pub fn ppgpio_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x308) as *mut u32
  }
  #[inline] pub fn ppgpio(&self) -> Ppgpio { 
     unsafe {
        Ppgpio(::core::ptr::read_volatile(((self.0 as usize) + 0x308) as *const u32))
     }
  }
  #[inline] pub fn set_ppgpio(&self, value: Ppgpio) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x308) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ppgpio<F: FnOnce(Ppgpio) -> Ppgpio>(&self, f: F) -> &Self {
     let tmp = self.ppgpio();
     self.set_ppgpio(f(tmp))
  }

  #[inline] pub fn ppdma_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x30c) as *const u32
  }
  #[inline] pub fn ppdma_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x30c) as *mut u32
  }
  #[inline] pub fn ppdma(&self) -> Ppdma { 
     unsafe {
        Ppdma(::core::ptr::read_volatile(((self.0 as usize) + 0x30c) as *const u32))
     }
  }
  #[inline] pub fn set_ppdma(&self, value: Ppdma) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x30c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ppdma<F: FnOnce(Ppdma) -> Ppdma>(&self, f: F) -> &Self {
     let tmp = self.ppdma();
     self.set_ppdma(f(tmp))
  }

  #[inline] pub fn ppepi_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x310) as *const u32
  }
  #[inline] pub fn ppepi_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x310) as *mut u32
  }
  #[inline] pub fn ppepi(&self) -> Ppepi { 
     unsafe {
        Ppepi(::core::ptr::read_volatile(((self.0 as usize) + 0x310) as *const u32))
     }
  }
  #[inline] pub fn set_ppepi(&self, value: Ppepi) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x310) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ppepi<F: FnOnce(Ppepi) -> Ppepi>(&self, f: F) -> &Self {
     let tmp = self.ppepi();
     self.set_ppepi(f(tmp))
  }

  #[inline] pub fn pphib_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x314) as *const u32
  }
  #[inline] pub fn pphib_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x314) as *mut u32
  }
  #[inline] pub fn pphib(&self) -> Pphib { 
     unsafe {
        Pphib(::core::ptr::read_volatile(((self.0 as usize) + 0x314) as *const u32))
     }
  }
  #[inline] pub fn set_pphib(&self, value: Pphib) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x314) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pphib<F: FnOnce(Pphib) -> Pphib>(&self, f: F) -> &Self {
     let tmp = self.pphib();
     self.set_pphib(f(tmp))
  }

  #[inline] pub fn ppuart_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x318) as *const u32
  }
  #[inline] pub fn ppuart_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x318) as *mut u32
  }
  #[inline] pub fn ppuart(&self) -> Ppuart { 
     unsafe {
        Ppuart(::core::ptr::read_volatile(((self.0 as usize) + 0x318) as *const u32))
     }
  }
  #[inline] pub fn set_ppuart(&self, value: Ppuart) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x318) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ppuart<F: FnOnce(Ppuart) -> Ppuart>(&self, f: F) -> &Self {
     let tmp = self.ppuart();
     self.set_ppuart(f(tmp))
  }

  #[inline] pub fn ppssi_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x31c) as *const u32
  }
  #[inline] pub fn ppssi_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x31c) as *mut u32
  }
  #[inline] pub fn ppssi(&self) -> Ppssi { 
     unsafe {
        Ppssi(::core::ptr::read_volatile(((self.0 as usize) + 0x31c) as *const u32))
     }
  }
  #[inline] pub fn set_ppssi(&self, value: Ppssi) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x31c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ppssi<F: FnOnce(Ppssi) -> Ppssi>(&self, f: F) -> &Self {
     let tmp = self.ppssi();
     self.set_ppssi(f(tmp))
  }

  #[inline] pub fn ppi2c_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x320) as *const u32
  }
  #[inline] pub fn ppi2c_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x320) as *mut u32
  }
  #[inline] pub fn ppi2c(&self) -> Ppi2c { 
     unsafe {
        Ppi2c(::core::ptr::read_volatile(((self.0 as usize) + 0x320) as *const u32))
     }
  }
  #[inline] pub fn set_ppi2c(&self, value: Ppi2c) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x320) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ppi2c<F: FnOnce(Ppi2c) -> Ppi2c>(&self, f: F) -> &Self {
     let tmp = self.ppi2c();
     self.set_ppi2c(f(tmp))
  }

  #[inline] pub fn ppusb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x328) as *const u32
  }
  #[inline] pub fn ppusb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x328) as *mut u32
  }
  #[inline] pub fn ppusb(&self) -> Ppusb { 
     unsafe {
        Ppusb(::core::ptr::read_volatile(((self.0 as usize) + 0x328) as *const u32))
     }
  }
  #[inline] pub fn set_ppusb(&self, value: Ppusb) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x328) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ppusb<F: FnOnce(Ppusb) -> Ppusb>(&self, f: F) -> &Self {
     let tmp = self.ppusb();
     self.set_ppusb(f(tmp))
  }

  #[inline] pub fn ppephy_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x330) as *const u32
  }
  #[inline] pub fn ppephy_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x330) as *mut u32
  }
  #[inline] pub fn ppephy(&self) -> Ppephy { 
     unsafe {
        Ppephy(::core::ptr::read_volatile(((self.0 as usize) + 0x330) as *const u32))
     }
  }
  #[inline] pub fn set_ppephy(&self, value: Ppephy) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x330) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ppephy<F: FnOnce(Ppephy) -> Ppephy>(&self, f: F) -> &Self {
     let tmp = self.ppephy();
     self.set_ppephy(f(tmp))
  }

  #[inline] pub fn ppcan_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x334) as *const u32
  }
  #[inline] pub fn ppcan_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x334) as *mut u32
  }
  #[inline] pub fn ppcan(&self) -> Ppcan { 
     unsafe {
        Ppcan(::core::ptr::read_volatile(((self.0 as usize) + 0x334) as *const u32))
     }
  }
  #[inline] pub fn set_ppcan(&self, value: Ppcan) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x334) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ppcan<F: FnOnce(Ppcan) -> Ppcan>(&self, f: F) -> &Self {
     let tmp = self.ppcan();
     self.set_ppcan(f(tmp))
  }

  #[inline] pub fn ppadc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x338) as *const u32
  }
  #[inline] pub fn ppadc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x338) as *mut u32
  }
  #[inline] pub fn ppadc(&self) -> Ppadc { 
     unsafe {
        Ppadc(::core::ptr::read_volatile(((self.0 as usize) + 0x338) as *const u32))
     }
  }
  #[inline] pub fn set_ppadc(&self, value: Ppadc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x338) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ppadc<F: FnOnce(Ppadc) -> Ppadc>(&self, f: F) -> &Self {
     let tmp = self.ppadc();
     self.set_ppadc(f(tmp))
  }

  #[inline] pub fn ppacmp_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x33c) as *const u32
  }
  #[inline] pub fn ppacmp_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x33c) as *mut u32
  }
  #[inline] pub fn ppacmp(&self) -> Ppacmp { 
     unsafe {
        Ppacmp(::core::ptr::read_volatile(((self.0 as usize) + 0x33c) as *const u32))
     }
  }
  #[inline] pub fn set_ppacmp(&self, value: Ppacmp) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x33c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ppacmp<F: FnOnce(Ppacmp) -> Ppacmp>(&self, f: F) -> &Self {
     let tmp = self.ppacmp();
     self.set_ppacmp(f(tmp))
  }

  #[inline] pub fn pppwm_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x340) as *const u32
  }
  #[inline] pub fn pppwm_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x340) as *mut u32
  }
  #[inline] pub fn pppwm(&self) -> Pppwm { 
     unsafe {
        Pppwm(::core::ptr::read_volatile(((self.0 as usize) + 0x340) as *const u32))
     }
  }
  #[inline] pub fn set_pppwm(&self, value: Pppwm) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x340) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pppwm<F: FnOnce(Pppwm) -> Pppwm>(&self, f: F) -> &Self {
     let tmp = self.pppwm();
     self.set_pppwm(f(tmp))
  }

  #[inline] pub fn ppqei_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x344) as *const u32
  }
  #[inline] pub fn ppqei_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x344) as *mut u32
  }
  #[inline] pub fn ppqei(&self) -> Ppqei { 
     unsafe {
        Ppqei(::core::ptr::read_volatile(((self.0 as usize) + 0x344) as *const u32))
     }
  }
  #[inline] pub fn set_ppqei(&self, value: Ppqei) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x344) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ppqei<F: FnOnce(Ppqei) -> Ppqei>(&self, f: F) -> &Self {
     let tmp = self.ppqei();
     self.set_ppqei(f(tmp))
  }

  #[inline] pub fn pplpc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x348) as *const u32
  }
  #[inline] pub fn pplpc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x348) as *mut u32
  }
  #[inline] pub fn pplpc(&self) -> Pplpc { 
     unsafe {
        Pplpc(::core::ptr::read_volatile(((self.0 as usize) + 0x348) as *const u32))
     }
  }
  #[inline] pub fn set_pplpc(&self, value: Pplpc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x348) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pplpc<F: FnOnce(Pplpc) -> Pplpc>(&self, f: F) -> &Self {
     let tmp = self.pplpc();
     self.set_pplpc(f(tmp))
  }

  #[inline] pub fn pppeci_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x350) as *const u32
  }
  #[inline] pub fn pppeci_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x350) as *mut u32
  }
  #[inline] pub fn pppeci(&self) -> Pppeci { 
     unsafe {
        Pppeci(::core::ptr::read_volatile(((self.0 as usize) + 0x350) as *const u32))
     }
  }
  #[inline] pub fn set_pppeci(&self, value: Pppeci) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x350) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pppeci<F: FnOnce(Pppeci) -> Pppeci>(&self, f: F) -> &Self {
     let tmp = self.pppeci();
     self.set_pppeci(f(tmp))
  }

  #[inline] pub fn ppfan_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x354) as *const u32
  }
  #[inline] pub fn ppfan_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x354) as *mut u32
  }
  #[inline] pub fn ppfan(&self) -> Ppfan { 
     unsafe {
        Ppfan(::core::ptr::read_volatile(((self.0 as usize) + 0x354) as *const u32))
     }
  }
  #[inline] pub fn set_ppfan(&self, value: Ppfan) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x354) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ppfan<F: FnOnce(Ppfan) -> Ppfan>(&self, f: F) -> &Self {
     let tmp = self.ppfan();
     self.set_ppfan(f(tmp))
  }

  #[inline] pub fn ppeeprom_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x358) as *const u32
  }
  #[inline] pub fn ppeeprom_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x358) as *mut u32
  }
  #[inline] pub fn ppeeprom(&self) -> Ppeeprom { 
     unsafe {
        Ppeeprom(::core::ptr::read_volatile(((self.0 as usize) + 0x358) as *const u32))
     }
  }
  #[inline] pub fn set_ppeeprom(&self, value: Ppeeprom) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x358) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ppeeprom<F: FnOnce(Ppeeprom) -> Ppeeprom>(&self, f: F) -> &Self {
     let tmp = self.ppeeprom();
     self.set_ppeeprom(f(tmp))
  }

  #[inline] pub fn ppwtimer_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x35c) as *const u32
  }
  #[inline] pub fn ppwtimer_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x35c) as *mut u32
  }
  #[inline] pub fn ppwtimer(&self) -> Ppwtimer { 
     unsafe {
        Ppwtimer(::core::ptr::read_volatile(((self.0 as usize) + 0x35c) as *const u32))
     }
  }
  #[inline] pub fn set_ppwtimer(&self, value: Ppwtimer) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x35c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ppwtimer<F: FnOnce(Ppwtimer) -> Ppwtimer>(&self, f: F) -> &Self {
     let tmp = self.ppwtimer();
     self.set_ppwtimer(f(tmp))
  }

  #[inline] pub fn pprts_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x370) as *const u32
  }
  #[inline] pub fn pprts_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x370) as *mut u32
  }
  #[inline] pub fn pprts(&self) -> Pprts { 
     unsafe {
        Pprts(::core::ptr::read_volatile(((self.0 as usize) + 0x370) as *const u32))
     }
  }
  #[inline] pub fn set_pprts(&self, value: Pprts) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x370) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pprts<F: FnOnce(Pprts) -> Pprts>(&self, f: F) -> &Self {
     let tmp = self.pprts();
     self.set_pprts(f(tmp))
  }

  #[inline] pub fn ppccm_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x374) as *const u32
  }
  #[inline] pub fn ppccm_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x374) as *mut u32
  }
  #[inline] pub fn ppccm(&self) -> Ppccm { 
     unsafe {
        Ppccm(::core::ptr::read_volatile(((self.0 as usize) + 0x374) as *const u32))
     }
  }
  #[inline] pub fn set_ppccm(&self, value: Ppccm) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x374) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ppccm<F: FnOnce(Ppccm) -> Ppccm>(&self, f: F) -> &Self {
     let tmp = self.ppccm();
     self.set_ppccm(f(tmp))
  }

  #[inline] pub fn pplcd_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x390) as *const u32
  }
  #[inline] pub fn pplcd_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x390) as *mut u32
  }
  #[inline] pub fn pplcd(&self) -> Pplcd { 
     unsafe {
        Pplcd(::core::ptr::read_volatile(((self.0 as usize) + 0x390) as *const u32))
     }
  }
  #[inline] pub fn set_pplcd(&self, value: Pplcd) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x390) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pplcd<F: FnOnce(Pplcd) -> Pplcd>(&self, f: F) -> &Self {
     let tmp = self.pplcd();
     self.set_pplcd(f(tmp))
  }

  #[inline] pub fn ppowire_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x398) as *const u32
  }
  #[inline] pub fn ppowire_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x398) as *mut u32
  }
  #[inline] pub fn ppowire(&self) -> Ppowire { 
     unsafe {
        Ppowire(::core::ptr::read_volatile(((self.0 as usize) + 0x398) as *const u32))
     }
  }
  #[inline] pub fn set_ppowire(&self, value: Ppowire) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x398) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ppowire<F: FnOnce(Ppowire) -> Ppowire>(&self, f: F) -> &Self {
     let tmp = self.ppowire();
     self.set_ppowire(f(tmp))
  }

  #[inline] pub fn ppemac_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x39c) as *const u32
  }
  #[inline] pub fn ppemac_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x39c) as *mut u32
  }
  #[inline] pub fn ppemac(&self) -> Ppemac { 
     unsafe {
        Ppemac(::core::ptr::read_volatile(((self.0 as usize) + 0x39c) as *const u32))
     }
  }
  #[inline] pub fn set_ppemac(&self, value: Ppemac) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x39c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ppemac<F: FnOnce(Ppemac) -> Ppemac>(&self, f: F) -> &Self {
     let tmp = self.ppemac();
     self.set_ppemac(f(tmp))
  }

  #[inline] pub fn pphim_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x3a4) as *const u32
  }
  #[inline] pub fn pphim_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x3a4) as *mut u32
  }
  #[inline] pub fn pphim(&self) -> Pphim { 
     unsafe {
        Pphim(::core::ptr::read_volatile(((self.0 as usize) + 0x3a4) as *const u32))
     }
  }
  #[inline] pub fn set_pphim(&self, value: Pphim) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x3a4) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pphim<F: FnOnce(Pphim) -> Pphim>(&self, f: F) -> &Self {
     let tmp = self.pphim();
     self.set_pphim(f(tmp))
  }

  #[inline] pub fn srwd_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x500) as *const u32
  }
  #[inline] pub fn srwd_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x500) as *mut u32
  }
  #[inline] pub fn srwd(&self) -> Srwd { 
     unsafe {
        Srwd(::core::ptr::read_volatile(((self.0 as usize) + 0x500) as *const u32))
     }
  }
  #[inline] pub fn set_srwd(&self, value: Srwd) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x500) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_srwd<F: FnOnce(Srwd) -> Srwd>(&self, f: F) -> &Self {
     let tmp = self.srwd();
     self.set_srwd(f(tmp))
  }

  #[inline] pub fn srtimer_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x504) as *const u32
  }
  #[inline] pub fn srtimer_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x504) as *mut u32
  }
  #[inline] pub fn srtimer(&self) -> Srtimer { 
     unsafe {
        Srtimer(::core::ptr::read_volatile(((self.0 as usize) + 0x504) as *const u32))
     }
  }
  #[inline] pub fn set_srtimer(&self, value: Srtimer) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x504) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_srtimer<F: FnOnce(Srtimer) -> Srtimer>(&self, f: F) -> &Self {
     let tmp = self.srtimer();
     self.set_srtimer(f(tmp))
  }

  #[inline] pub fn srgpio_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x508) as *const u32
  }
  #[inline] pub fn srgpio_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x508) as *mut u32
  }
  #[inline] pub fn srgpio(&self) -> Srgpio { 
     unsafe {
        Srgpio(::core::ptr::read_volatile(((self.0 as usize) + 0x508) as *const u32))
     }
  }
  #[inline] pub fn set_srgpio(&self, value: Srgpio) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x508) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_srgpio<F: FnOnce(Srgpio) -> Srgpio>(&self, f: F) -> &Self {
     let tmp = self.srgpio();
     self.set_srgpio(f(tmp))
  }

  #[inline] pub fn srdma_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x50c) as *const u32
  }
  #[inline] pub fn srdma_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x50c) as *mut u32
  }
  #[inline] pub fn srdma(&self) -> Srdma { 
     unsafe {
        Srdma(::core::ptr::read_volatile(((self.0 as usize) + 0x50c) as *const u32))
     }
  }
  #[inline] pub fn set_srdma(&self, value: Srdma) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x50c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_srdma<F: FnOnce(Srdma) -> Srdma>(&self, f: F) -> &Self {
     let tmp = self.srdma();
     self.set_srdma(f(tmp))
  }

  #[inline] pub fn srepi_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x510) as *const u32
  }
  #[inline] pub fn srepi_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x510) as *mut u32
  }
  #[inline] pub fn srepi(&self) -> Srepi { 
     unsafe {
        Srepi(::core::ptr::read_volatile(((self.0 as usize) + 0x510) as *const u32))
     }
  }
  #[inline] pub fn set_srepi(&self, value: Srepi) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x510) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_srepi<F: FnOnce(Srepi) -> Srepi>(&self, f: F) -> &Self {
     let tmp = self.srepi();
     self.set_srepi(f(tmp))
  }

  #[inline] pub fn srhib_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x514) as *const u32
  }
  #[inline] pub fn srhib_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x514) as *mut u32
  }
  #[inline] pub fn srhib(&self) -> Srhib { 
     unsafe {
        Srhib(::core::ptr::read_volatile(((self.0 as usize) + 0x514) as *const u32))
     }
  }
  #[inline] pub fn set_srhib(&self, value: Srhib) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x514) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_srhib<F: FnOnce(Srhib) -> Srhib>(&self, f: F) -> &Self {
     let tmp = self.srhib();
     self.set_srhib(f(tmp))
  }

  #[inline] pub fn sruart_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x518) as *const u32
  }
  #[inline] pub fn sruart_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x518) as *mut u32
  }
  #[inline] pub fn sruart(&self) -> Sruart { 
     unsafe {
        Sruart(::core::ptr::read_volatile(((self.0 as usize) + 0x518) as *const u32))
     }
  }
  #[inline] pub fn set_sruart(&self, value: Sruart) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x518) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_sruart<F: FnOnce(Sruart) -> Sruart>(&self, f: F) -> &Self {
     let tmp = self.sruart();
     self.set_sruart(f(tmp))
  }

  #[inline] pub fn srssi_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x51c) as *const u32
  }
  #[inline] pub fn srssi_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x51c) as *mut u32
  }
  #[inline] pub fn srssi(&self) -> Srssi { 
     unsafe {
        Srssi(::core::ptr::read_volatile(((self.0 as usize) + 0x51c) as *const u32))
     }
  }
  #[inline] pub fn set_srssi(&self, value: Srssi) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x51c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_srssi<F: FnOnce(Srssi) -> Srssi>(&self, f: F) -> &Self {
     let tmp = self.srssi();
     self.set_srssi(f(tmp))
  }

  #[inline] pub fn sri2c_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x520) as *const u32
  }
  #[inline] pub fn sri2c_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x520) as *mut u32
  }
  #[inline] pub fn sri2c(&self) -> Sri2c { 
     unsafe {
        Sri2c(::core::ptr::read_volatile(((self.0 as usize) + 0x520) as *const u32))
     }
  }
  #[inline] pub fn set_sri2c(&self, value: Sri2c) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x520) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_sri2c<F: FnOnce(Sri2c) -> Sri2c>(&self, f: F) -> &Self {
     let tmp = self.sri2c();
     self.set_sri2c(f(tmp))
  }

  #[inline] pub fn srusb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x528) as *const u32
  }
  #[inline] pub fn srusb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x528) as *mut u32
  }
  #[inline] pub fn srusb(&self) -> Srusb { 
     unsafe {
        Srusb(::core::ptr::read_volatile(((self.0 as usize) + 0x528) as *const u32))
     }
  }
  #[inline] pub fn set_srusb(&self, value: Srusb) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x528) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_srusb<F: FnOnce(Srusb) -> Srusb>(&self, f: F) -> &Self {
     let tmp = self.srusb();
     self.set_srusb(f(tmp))
  }

  #[inline] pub fn srephy_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x530) as *const u32
  }
  #[inline] pub fn srephy_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x530) as *mut u32
  }
  #[inline] pub fn srephy(&self) -> Srephy { 
     unsafe {
        Srephy(::core::ptr::read_volatile(((self.0 as usize) + 0x530) as *const u32))
     }
  }
  #[inline] pub fn set_srephy(&self, value: Srephy) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x530) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_srephy<F: FnOnce(Srephy) -> Srephy>(&self, f: F) -> &Self {
     let tmp = self.srephy();
     self.set_srephy(f(tmp))
  }

  #[inline] pub fn srcan_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x534) as *const u32
  }
  #[inline] pub fn srcan_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x534) as *mut u32
  }
  #[inline] pub fn srcan(&self) -> Srcan { 
     unsafe {
        Srcan(::core::ptr::read_volatile(((self.0 as usize) + 0x534) as *const u32))
     }
  }
  #[inline] pub fn set_srcan(&self, value: Srcan) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x534) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_srcan<F: FnOnce(Srcan) -> Srcan>(&self, f: F) -> &Self {
     let tmp = self.srcan();
     self.set_srcan(f(tmp))
  }

  #[inline] pub fn sradc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x538) as *const u32
  }
  #[inline] pub fn sradc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x538) as *mut u32
  }
  #[inline] pub fn sradc(&self) -> Sradc { 
     unsafe {
        Sradc(::core::ptr::read_volatile(((self.0 as usize) + 0x538) as *const u32))
     }
  }
  #[inline] pub fn set_sradc(&self, value: Sradc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x538) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_sradc<F: FnOnce(Sradc) -> Sradc>(&self, f: F) -> &Self {
     let tmp = self.sradc();
     self.set_sradc(f(tmp))
  }

  #[inline] pub fn sracmp_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x53c) as *const u32
  }
  #[inline] pub fn sracmp_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x53c) as *mut u32
  }
  #[inline] pub fn sracmp(&self) -> Sracmp { 
     unsafe {
        Sracmp(::core::ptr::read_volatile(((self.0 as usize) + 0x53c) as *const u32))
     }
  }
  #[inline] pub fn set_sracmp(&self, value: Sracmp) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x53c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_sracmp<F: FnOnce(Sracmp) -> Sracmp>(&self, f: F) -> &Self {
     let tmp = self.sracmp();
     self.set_sracmp(f(tmp))
  }

  #[inline] pub fn srpwm_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x540) as *const u32
  }
  #[inline] pub fn srpwm_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x540) as *mut u32
  }
  #[inline] pub fn srpwm(&self) -> Srpwm { 
     unsafe {
        Srpwm(::core::ptr::read_volatile(((self.0 as usize) + 0x540) as *const u32))
     }
  }
  #[inline] pub fn set_srpwm(&self, value: Srpwm) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x540) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_srpwm<F: FnOnce(Srpwm) -> Srpwm>(&self, f: F) -> &Self {
     let tmp = self.srpwm();
     self.set_srpwm(f(tmp))
  }

  #[inline] pub fn srqei_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x544) as *const u32
  }
  #[inline] pub fn srqei_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x544) as *mut u32
  }
  #[inline] pub fn srqei(&self) -> Srqei { 
     unsafe {
        Srqei(::core::ptr::read_volatile(((self.0 as usize) + 0x544) as *const u32))
     }
  }
  #[inline] pub fn set_srqei(&self, value: Srqei) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x544) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_srqei<F: FnOnce(Srqei) -> Srqei>(&self, f: F) -> &Self {
     let tmp = self.srqei();
     self.set_srqei(f(tmp))
  }

  #[inline] pub fn sreeprom_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x558) as *const u32
  }
  #[inline] pub fn sreeprom_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x558) as *mut u32
  }
  #[inline] pub fn sreeprom(&self) -> Sreeprom { 
     unsafe {
        Sreeprom(::core::ptr::read_volatile(((self.0 as usize) + 0x558) as *const u32))
     }
  }
  #[inline] pub fn set_sreeprom(&self, value: Sreeprom) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x558) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_sreeprom<F: FnOnce(Sreeprom) -> Sreeprom>(&self, f: F) -> &Self {
     let tmp = self.sreeprom();
     self.set_sreeprom(f(tmp))
  }

  #[inline] pub fn srccm_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x574) as *const u32
  }
  #[inline] pub fn srccm_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x574) as *mut u32
  }
  #[inline] pub fn srccm(&self) -> Srccm { 
     unsafe {
        Srccm(::core::ptr::read_volatile(((self.0 as usize) + 0x574) as *const u32))
     }
  }
  #[inline] pub fn set_srccm(&self, value: Srccm) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x574) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_srccm<F: FnOnce(Srccm) -> Srccm>(&self, f: F) -> &Self {
     let tmp = self.srccm();
     self.set_srccm(f(tmp))
  }

  #[inline] pub fn sremac_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x59c) as *const u32
  }
  #[inline] pub fn sremac_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x59c) as *mut u32
  }
  #[inline] pub fn sremac(&self) -> Sremac { 
     unsafe {
        Sremac(::core::ptr::read_volatile(((self.0 as usize) + 0x59c) as *const u32))
     }
  }
  #[inline] pub fn set_sremac(&self, value: Sremac) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x59c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_sremac<F: FnOnce(Sremac) -> Sremac>(&self, f: F) -> &Self {
     let tmp = self.sremac();
     self.set_sremac(f(tmp))
  }

  #[inline] pub fn rcgcwd_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x600) as *const u32
  }
  #[inline] pub fn rcgcwd_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x600) as *mut u32
  }
  #[inline] pub fn rcgcwd(&self) -> Rcgcwd { 
     unsafe {
        Rcgcwd(::core::ptr::read_volatile(((self.0 as usize) + 0x600) as *const u32))
     }
  }
  #[inline] pub fn set_rcgcwd(&self, value: Rcgcwd) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x600) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_rcgcwd<F: FnOnce(Rcgcwd) -> Rcgcwd>(&self, f: F) -> &Self {
     let tmp = self.rcgcwd();
     self.set_rcgcwd(f(tmp))
  }

  #[inline] pub fn rcgctimer_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x604) as *const u32
  }
  #[inline] pub fn rcgctimer_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x604) as *mut u32
  }
  #[inline] pub fn rcgctimer(&self) -> Rcgctimer { 
     unsafe {
        Rcgctimer(::core::ptr::read_volatile(((self.0 as usize) + 0x604) as *const u32))
     }
  }
  #[inline] pub fn set_rcgctimer(&self, value: Rcgctimer) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x604) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_rcgctimer<F: FnOnce(Rcgctimer) -> Rcgctimer>(&self, f: F) -> &Self {
     let tmp = self.rcgctimer();
     self.set_rcgctimer(f(tmp))
  }

  #[inline] pub fn rcgcgpio_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x608) as *const u32
  }
  #[inline] pub fn rcgcgpio_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x608) as *mut u32
  }
  #[inline] pub fn rcgcgpio(&self) -> Rcgcgpio { 
     unsafe {
        Rcgcgpio(::core::ptr::read_volatile(((self.0 as usize) + 0x608) as *const u32))
     }
  }
  #[inline] pub fn set_rcgcgpio(&self, value: Rcgcgpio) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x608) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_rcgcgpio<F: FnOnce(Rcgcgpio) -> Rcgcgpio>(&self, f: F) -> &Self {
     let tmp = self.rcgcgpio();
     self.set_rcgcgpio(f(tmp))
  }

  #[inline] pub fn rcgcdma_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x60c) as *const u32
  }
  #[inline] pub fn rcgcdma_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x60c) as *mut u32
  }
  #[inline] pub fn rcgcdma(&self) -> Rcgcdma { 
     unsafe {
        Rcgcdma(::core::ptr::read_volatile(((self.0 as usize) + 0x60c) as *const u32))
     }
  }
  #[inline] pub fn set_rcgcdma(&self, value: Rcgcdma) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x60c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_rcgcdma<F: FnOnce(Rcgcdma) -> Rcgcdma>(&self, f: F) -> &Self {
     let tmp = self.rcgcdma();
     self.set_rcgcdma(f(tmp))
  }

  #[inline] pub fn rcgcepi_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x610) as *const u32
  }
  #[inline] pub fn rcgcepi_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x610) as *mut u32
  }
  #[inline] pub fn rcgcepi(&self) -> Rcgcepi { 
     unsafe {
        Rcgcepi(::core::ptr::read_volatile(((self.0 as usize) + 0x610) as *const u32))
     }
  }
  #[inline] pub fn set_rcgcepi(&self, value: Rcgcepi) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x610) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_rcgcepi<F: FnOnce(Rcgcepi) -> Rcgcepi>(&self, f: F) -> &Self {
     let tmp = self.rcgcepi();
     self.set_rcgcepi(f(tmp))
  }

  #[inline] pub fn rcgchib_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x614) as *const u32
  }
  #[inline] pub fn rcgchib_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x614) as *mut u32
  }
  #[inline] pub fn rcgchib(&self) -> Rcgchib { 
     unsafe {
        Rcgchib(::core::ptr::read_volatile(((self.0 as usize) + 0x614) as *const u32))
     }
  }
  #[inline] pub fn set_rcgchib(&self, value: Rcgchib) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x614) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_rcgchib<F: FnOnce(Rcgchib) -> Rcgchib>(&self, f: F) -> &Self {
     let tmp = self.rcgchib();
     self.set_rcgchib(f(tmp))
  }

  #[inline] pub fn rcgcuart_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x618) as *const u32
  }
  #[inline] pub fn rcgcuart_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x618) as *mut u32
  }
  #[inline] pub fn rcgcuart(&self) -> Rcgcuart { 
     unsafe {
        Rcgcuart(::core::ptr::read_volatile(((self.0 as usize) + 0x618) as *const u32))
     }
  }
  #[inline] pub fn set_rcgcuart(&self, value: Rcgcuart) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x618) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_rcgcuart<F: FnOnce(Rcgcuart) -> Rcgcuart>(&self, f: F) -> &Self {
     let tmp = self.rcgcuart();
     self.set_rcgcuart(f(tmp))
  }

  #[inline] pub fn rcgcssi_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x61c) as *const u32
  }
  #[inline] pub fn rcgcssi_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x61c) as *mut u32
  }
  #[inline] pub fn rcgcssi(&self) -> Rcgcssi { 
     unsafe {
        Rcgcssi(::core::ptr::read_volatile(((self.0 as usize) + 0x61c) as *const u32))
     }
  }
  #[inline] pub fn set_rcgcssi(&self, value: Rcgcssi) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x61c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_rcgcssi<F: FnOnce(Rcgcssi) -> Rcgcssi>(&self, f: F) -> &Self {
     let tmp = self.rcgcssi();
     self.set_rcgcssi(f(tmp))
  }

  #[inline] pub fn rcgci2c_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x620) as *const u32
  }
  #[inline] pub fn rcgci2c_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x620) as *mut u32
  }
  #[inline] pub fn rcgci2c(&self) -> Rcgci2c { 
     unsafe {
        Rcgci2c(::core::ptr::read_volatile(((self.0 as usize) + 0x620) as *const u32))
     }
  }
  #[inline] pub fn set_rcgci2c(&self, value: Rcgci2c) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x620) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_rcgci2c<F: FnOnce(Rcgci2c) -> Rcgci2c>(&self, f: F) -> &Self {
     let tmp = self.rcgci2c();
     self.set_rcgci2c(f(tmp))
  }

  #[inline] pub fn rcgcusb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x628) as *const u32
  }
  #[inline] pub fn rcgcusb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x628) as *mut u32
  }
  #[inline] pub fn rcgcusb(&self) -> Rcgcusb { 
     unsafe {
        Rcgcusb(::core::ptr::read_volatile(((self.0 as usize) + 0x628) as *const u32))
     }
  }
  #[inline] pub fn set_rcgcusb(&self, value: Rcgcusb) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x628) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_rcgcusb<F: FnOnce(Rcgcusb) -> Rcgcusb>(&self, f: F) -> &Self {
     let tmp = self.rcgcusb();
     self.set_rcgcusb(f(tmp))
  }

  #[inline] pub fn rcgcephy_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x630) as *const u32
  }
  #[inline] pub fn rcgcephy_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x630) as *mut u32
  }
  #[inline] pub fn rcgcephy(&self) -> Rcgcephy { 
     unsafe {
        Rcgcephy(::core::ptr::read_volatile(((self.0 as usize) + 0x630) as *const u32))
     }
  }
  #[inline] pub fn set_rcgcephy(&self, value: Rcgcephy) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x630) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_rcgcephy<F: FnOnce(Rcgcephy) -> Rcgcephy>(&self, f: F) -> &Self {
     let tmp = self.rcgcephy();
     self.set_rcgcephy(f(tmp))
  }

  #[inline] pub fn rcgccan_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x634) as *const u32
  }
  #[inline] pub fn rcgccan_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x634) as *mut u32
  }
  #[inline] pub fn rcgccan(&self) -> Rcgccan { 
     unsafe {
        Rcgccan(::core::ptr::read_volatile(((self.0 as usize) + 0x634) as *const u32))
     }
  }
  #[inline] pub fn set_rcgccan(&self, value: Rcgccan) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x634) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_rcgccan<F: FnOnce(Rcgccan) -> Rcgccan>(&self, f: F) -> &Self {
     let tmp = self.rcgccan();
     self.set_rcgccan(f(tmp))
  }

  #[inline] pub fn rcgcadc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x638) as *const u32
  }
  #[inline] pub fn rcgcadc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x638) as *mut u32
  }
  #[inline] pub fn rcgcadc(&self) -> Rcgcadc { 
     unsafe {
        Rcgcadc(::core::ptr::read_volatile(((self.0 as usize) + 0x638) as *const u32))
     }
  }
  #[inline] pub fn set_rcgcadc(&self, value: Rcgcadc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x638) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_rcgcadc<F: FnOnce(Rcgcadc) -> Rcgcadc>(&self, f: F) -> &Self {
     let tmp = self.rcgcadc();
     self.set_rcgcadc(f(tmp))
  }

  #[inline] pub fn rcgcacmp_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x63c) as *const u32
  }
  #[inline] pub fn rcgcacmp_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x63c) as *mut u32
  }
  #[inline] pub fn rcgcacmp(&self) -> Rcgcacmp { 
     unsafe {
        Rcgcacmp(::core::ptr::read_volatile(((self.0 as usize) + 0x63c) as *const u32))
     }
  }
  #[inline] pub fn set_rcgcacmp(&self, value: Rcgcacmp) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x63c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_rcgcacmp<F: FnOnce(Rcgcacmp) -> Rcgcacmp>(&self, f: F) -> &Self {
     let tmp = self.rcgcacmp();
     self.set_rcgcacmp(f(tmp))
  }

  #[inline] pub fn rcgcpwm_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x640) as *const u32
  }
  #[inline] pub fn rcgcpwm_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x640) as *mut u32
  }
  #[inline] pub fn rcgcpwm(&self) -> Rcgcpwm { 
     unsafe {
        Rcgcpwm(::core::ptr::read_volatile(((self.0 as usize) + 0x640) as *const u32))
     }
  }
  #[inline] pub fn set_rcgcpwm(&self, value: Rcgcpwm) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x640) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_rcgcpwm<F: FnOnce(Rcgcpwm) -> Rcgcpwm>(&self, f: F) -> &Self {
     let tmp = self.rcgcpwm();
     self.set_rcgcpwm(f(tmp))
  }

  #[inline] pub fn rcgcqei_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x644) as *const u32
  }
  #[inline] pub fn rcgcqei_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x644) as *mut u32
  }
  #[inline] pub fn rcgcqei(&self) -> Rcgcqei { 
     unsafe {
        Rcgcqei(::core::ptr::read_volatile(((self.0 as usize) + 0x644) as *const u32))
     }
  }
  #[inline] pub fn set_rcgcqei(&self, value: Rcgcqei) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x644) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_rcgcqei<F: FnOnce(Rcgcqei) -> Rcgcqei>(&self, f: F) -> &Self {
     let tmp = self.rcgcqei();
     self.set_rcgcqei(f(tmp))
  }

  #[inline] pub fn rcgceeprom_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x658) as *const u32
  }
  #[inline] pub fn rcgceeprom_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x658) as *mut u32
  }
  #[inline] pub fn rcgceeprom(&self) -> Rcgceeprom { 
     unsafe {
        Rcgceeprom(::core::ptr::read_volatile(((self.0 as usize) + 0x658) as *const u32))
     }
  }
  #[inline] pub fn set_rcgceeprom(&self, value: Rcgceeprom) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x658) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_rcgceeprom<F: FnOnce(Rcgceeprom) -> Rcgceeprom>(&self, f: F) -> &Self {
     let tmp = self.rcgceeprom();
     self.set_rcgceeprom(f(tmp))
  }

  #[inline] pub fn rcgcccm_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x674) as *const u32
  }
  #[inline] pub fn rcgcccm_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x674) as *mut u32
  }
  #[inline] pub fn rcgcccm(&self) -> Rcgcccm { 
     unsafe {
        Rcgcccm(::core::ptr::read_volatile(((self.0 as usize) + 0x674) as *const u32))
     }
  }
  #[inline] pub fn set_rcgcccm(&self, value: Rcgcccm) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x674) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_rcgcccm<F: FnOnce(Rcgcccm) -> Rcgcccm>(&self, f: F) -> &Self {
     let tmp = self.rcgcccm();
     self.set_rcgcccm(f(tmp))
  }

  #[inline] pub fn rcgcemac_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x69c) as *const u32
  }
  #[inline] pub fn rcgcemac_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x69c) as *mut u32
  }
  #[inline] pub fn rcgcemac(&self) -> Rcgcemac { 
     unsafe {
        Rcgcemac(::core::ptr::read_volatile(((self.0 as usize) + 0x69c) as *const u32))
     }
  }
  #[inline] pub fn set_rcgcemac(&self, value: Rcgcemac) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x69c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_rcgcemac<F: FnOnce(Rcgcemac) -> Rcgcemac>(&self, f: F) -> &Self {
     let tmp = self.rcgcemac();
     self.set_rcgcemac(f(tmp))
  }

  #[inline] pub fn scgcwd_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x700) as *const u32
  }
  #[inline] pub fn scgcwd_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x700) as *mut u32
  }
  #[inline] pub fn scgcwd(&self) -> Scgcwd { 
     unsafe {
        Scgcwd(::core::ptr::read_volatile(((self.0 as usize) + 0x700) as *const u32))
     }
  }
  #[inline] pub fn set_scgcwd(&self, value: Scgcwd) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x700) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_scgcwd<F: FnOnce(Scgcwd) -> Scgcwd>(&self, f: F) -> &Self {
     let tmp = self.scgcwd();
     self.set_scgcwd(f(tmp))
  }

  #[inline] pub fn scgctimer_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x704) as *const u32
  }
  #[inline] pub fn scgctimer_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x704) as *mut u32
  }
  #[inline] pub fn scgctimer(&self) -> Scgctimer { 
     unsafe {
        Scgctimer(::core::ptr::read_volatile(((self.0 as usize) + 0x704) as *const u32))
     }
  }
  #[inline] pub fn set_scgctimer(&self, value: Scgctimer) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x704) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_scgctimer<F: FnOnce(Scgctimer) -> Scgctimer>(&self, f: F) -> &Self {
     let tmp = self.scgctimer();
     self.set_scgctimer(f(tmp))
  }

  #[inline] pub fn scgcgpio_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x708) as *const u32
  }
  #[inline] pub fn scgcgpio_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x708) as *mut u32
  }
  #[inline] pub fn scgcgpio(&self) -> Scgcgpio { 
     unsafe {
        Scgcgpio(::core::ptr::read_volatile(((self.0 as usize) + 0x708) as *const u32))
     }
  }
  #[inline] pub fn set_scgcgpio(&self, value: Scgcgpio) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x708) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_scgcgpio<F: FnOnce(Scgcgpio) -> Scgcgpio>(&self, f: F) -> &Self {
     let tmp = self.scgcgpio();
     self.set_scgcgpio(f(tmp))
  }

  #[inline] pub fn scgcdma_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x70c) as *const u32
  }
  #[inline] pub fn scgcdma_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x70c) as *mut u32
  }
  #[inline] pub fn scgcdma(&self) -> Scgcdma { 
     unsafe {
        Scgcdma(::core::ptr::read_volatile(((self.0 as usize) + 0x70c) as *const u32))
     }
  }
  #[inline] pub fn set_scgcdma(&self, value: Scgcdma) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x70c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_scgcdma<F: FnOnce(Scgcdma) -> Scgcdma>(&self, f: F) -> &Self {
     let tmp = self.scgcdma();
     self.set_scgcdma(f(tmp))
  }

  #[inline] pub fn scgcepi_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x710) as *const u32
  }
  #[inline] pub fn scgcepi_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x710) as *mut u32
  }
  #[inline] pub fn scgcepi(&self) -> Scgcepi { 
     unsafe {
        Scgcepi(::core::ptr::read_volatile(((self.0 as usize) + 0x710) as *const u32))
     }
  }
  #[inline] pub fn set_scgcepi(&self, value: Scgcepi) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x710) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_scgcepi<F: FnOnce(Scgcepi) -> Scgcepi>(&self, f: F) -> &Self {
     let tmp = self.scgcepi();
     self.set_scgcepi(f(tmp))
  }

  #[inline] pub fn scgchib_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x714) as *const u32
  }
  #[inline] pub fn scgchib_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x714) as *mut u32
  }
  #[inline] pub fn scgchib(&self) -> Scgchib { 
     unsafe {
        Scgchib(::core::ptr::read_volatile(((self.0 as usize) + 0x714) as *const u32))
     }
  }
  #[inline] pub fn set_scgchib(&self, value: Scgchib) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x714) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_scgchib<F: FnOnce(Scgchib) -> Scgchib>(&self, f: F) -> &Self {
     let tmp = self.scgchib();
     self.set_scgchib(f(tmp))
  }

  #[inline] pub fn scgcuart_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x718) as *const u32
  }
  #[inline] pub fn scgcuart_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x718) as *mut u32
  }
  #[inline] pub fn scgcuart(&self) -> Scgcuart { 
     unsafe {
        Scgcuart(::core::ptr::read_volatile(((self.0 as usize) + 0x718) as *const u32))
     }
  }
  #[inline] pub fn set_scgcuart(&self, value: Scgcuart) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x718) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_scgcuart<F: FnOnce(Scgcuart) -> Scgcuart>(&self, f: F) -> &Self {
     let tmp = self.scgcuart();
     self.set_scgcuart(f(tmp))
  }

  #[inline] pub fn scgcssi_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x71c) as *const u32
  }
  #[inline] pub fn scgcssi_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x71c) as *mut u32
  }
  #[inline] pub fn scgcssi(&self) -> Scgcssi { 
     unsafe {
        Scgcssi(::core::ptr::read_volatile(((self.0 as usize) + 0x71c) as *const u32))
     }
  }
  #[inline] pub fn set_scgcssi(&self, value: Scgcssi) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x71c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_scgcssi<F: FnOnce(Scgcssi) -> Scgcssi>(&self, f: F) -> &Self {
     let tmp = self.scgcssi();
     self.set_scgcssi(f(tmp))
  }

  #[inline] pub fn scgci2c_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x720) as *const u32
  }
  #[inline] pub fn scgci2c_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x720) as *mut u32
  }
  #[inline] pub fn scgci2c(&self) -> Scgci2c { 
     unsafe {
        Scgci2c(::core::ptr::read_volatile(((self.0 as usize) + 0x720) as *const u32))
     }
  }
  #[inline] pub fn set_scgci2c(&self, value: Scgci2c) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x720) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_scgci2c<F: FnOnce(Scgci2c) -> Scgci2c>(&self, f: F) -> &Self {
     let tmp = self.scgci2c();
     self.set_scgci2c(f(tmp))
  }

  #[inline] pub fn scgcusb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x728) as *const u32
  }
  #[inline] pub fn scgcusb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x728) as *mut u32
  }
  #[inline] pub fn scgcusb(&self) -> Scgcusb { 
     unsafe {
        Scgcusb(::core::ptr::read_volatile(((self.0 as usize) + 0x728) as *const u32))
     }
  }
  #[inline] pub fn set_scgcusb(&self, value: Scgcusb) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x728) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_scgcusb<F: FnOnce(Scgcusb) -> Scgcusb>(&self, f: F) -> &Self {
     let tmp = self.scgcusb();
     self.set_scgcusb(f(tmp))
  }

  #[inline] pub fn scgcephy_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x730) as *const u32
  }
  #[inline] pub fn scgcephy_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x730) as *mut u32
  }
  #[inline] pub fn scgcephy(&self) -> Scgcephy { 
     unsafe {
        Scgcephy(::core::ptr::read_volatile(((self.0 as usize) + 0x730) as *const u32))
     }
  }
  #[inline] pub fn set_scgcephy(&self, value: Scgcephy) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x730) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_scgcephy<F: FnOnce(Scgcephy) -> Scgcephy>(&self, f: F) -> &Self {
     let tmp = self.scgcephy();
     self.set_scgcephy(f(tmp))
  }

  #[inline] pub fn scgccan_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x734) as *const u32
  }
  #[inline] pub fn scgccan_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x734) as *mut u32
  }
  #[inline] pub fn scgccan(&self) -> Scgccan { 
     unsafe {
        Scgccan(::core::ptr::read_volatile(((self.0 as usize) + 0x734) as *const u32))
     }
  }
  #[inline] pub fn set_scgccan(&self, value: Scgccan) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x734) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_scgccan<F: FnOnce(Scgccan) -> Scgccan>(&self, f: F) -> &Self {
     let tmp = self.scgccan();
     self.set_scgccan(f(tmp))
  }

  #[inline] pub fn scgcadc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x738) as *const u32
  }
  #[inline] pub fn scgcadc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x738) as *mut u32
  }
  #[inline] pub fn scgcadc(&self) -> Scgcadc { 
     unsafe {
        Scgcadc(::core::ptr::read_volatile(((self.0 as usize) + 0x738) as *const u32))
     }
  }
  #[inline] pub fn set_scgcadc(&self, value: Scgcadc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x738) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_scgcadc<F: FnOnce(Scgcadc) -> Scgcadc>(&self, f: F) -> &Self {
     let tmp = self.scgcadc();
     self.set_scgcadc(f(tmp))
  }

  #[inline] pub fn scgcacmp_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x73c) as *const u32
  }
  #[inline] pub fn scgcacmp_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x73c) as *mut u32
  }
  #[inline] pub fn scgcacmp(&self) -> Scgcacmp { 
     unsafe {
        Scgcacmp(::core::ptr::read_volatile(((self.0 as usize) + 0x73c) as *const u32))
     }
  }
  #[inline] pub fn set_scgcacmp(&self, value: Scgcacmp) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x73c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_scgcacmp<F: FnOnce(Scgcacmp) -> Scgcacmp>(&self, f: F) -> &Self {
     let tmp = self.scgcacmp();
     self.set_scgcacmp(f(tmp))
  }

  #[inline] pub fn scgcpwm_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x740) as *const u32
  }
  #[inline] pub fn scgcpwm_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x740) as *mut u32
  }
  #[inline] pub fn scgcpwm(&self) -> Scgcpwm { 
     unsafe {
        Scgcpwm(::core::ptr::read_volatile(((self.0 as usize) + 0x740) as *const u32))
     }
  }
  #[inline] pub fn set_scgcpwm(&self, value: Scgcpwm) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x740) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_scgcpwm<F: FnOnce(Scgcpwm) -> Scgcpwm>(&self, f: F) -> &Self {
     let tmp = self.scgcpwm();
     self.set_scgcpwm(f(tmp))
  }

  #[inline] pub fn scgcqei_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x744) as *const u32
  }
  #[inline] pub fn scgcqei_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x744) as *mut u32
  }
  #[inline] pub fn scgcqei(&self) -> Scgcqei { 
     unsafe {
        Scgcqei(::core::ptr::read_volatile(((self.0 as usize) + 0x744) as *const u32))
     }
  }
  #[inline] pub fn set_scgcqei(&self, value: Scgcqei) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x744) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_scgcqei<F: FnOnce(Scgcqei) -> Scgcqei>(&self, f: F) -> &Self {
     let tmp = self.scgcqei();
     self.set_scgcqei(f(tmp))
  }

  #[inline] pub fn scgceeprom_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x758) as *const u32
  }
  #[inline] pub fn scgceeprom_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x758) as *mut u32
  }
  #[inline] pub fn scgceeprom(&self) -> Scgceeprom { 
     unsafe {
        Scgceeprom(::core::ptr::read_volatile(((self.0 as usize) + 0x758) as *const u32))
     }
  }
  #[inline] pub fn set_scgceeprom(&self, value: Scgceeprom) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x758) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_scgceeprom<F: FnOnce(Scgceeprom) -> Scgceeprom>(&self, f: F) -> &Self {
     let tmp = self.scgceeprom();
     self.set_scgceeprom(f(tmp))
  }

  #[inline] pub fn scgcccm_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x774) as *const u32
  }
  #[inline] pub fn scgcccm_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x774) as *mut u32
  }
  #[inline] pub fn scgcccm(&self) -> Scgcccm { 
     unsafe {
        Scgcccm(::core::ptr::read_volatile(((self.0 as usize) + 0x774) as *const u32))
     }
  }
  #[inline] pub fn set_scgcccm(&self, value: Scgcccm) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x774) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_scgcccm<F: FnOnce(Scgcccm) -> Scgcccm>(&self, f: F) -> &Self {
     let tmp = self.scgcccm();
     self.set_scgcccm(f(tmp))
  }

  #[inline] pub fn scgcemac_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x79c) as *const u32
  }
  #[inline] pub fn scgcemac_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x79c) as *mut u32
  }
  #[inline] pub fn scgcemac(&self) -> Scgcemac { 
     unsafe {
        Scgcemac(::core::ptr::read_volatile(((self.0 as usize) + 0x79c) as *const u32))
     }
  }
  #[inline] pub fn set_scgcemac(&self, value: Scgcemac) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x79c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_scgcemac<F: FnOnce(Scgcemac) -> Scgcemac>(&self, f: F) -> &Self {
     let tmp = self.scgcemac();
     self.set_scgcemac(f(tmp))
  }

  #[inline] pub fn dcgcwd_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x800) as *const u32
  }
  #[inline] pub fn dcgcwd_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x800) as *mut u32
  }
  #[inline] pub fn dcgcwd(&self) -> Dcgcwd { 
     unsafe {
        Dcgcwd(::core::ptr::read_volatile(((self.0 as usize) + 0x800) as *const u32))
     }
  }
  #[inline] pub fn set_dcgcwd(&self, value: Dcgcwd) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x800) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dcgcwd<F: FnOnce(Dcgcwd) -> Dcgcwd>(&self, f: F) -> &Self {
     let tmp = self.dcgcwd();
     self.set_dcgcwd(f(tmp))
  }

  #[inline] pub fn dcgctimer_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x804) as *const u32
  }
  #[inline] pub fn dcgctimer_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x804) as *mut u32
  }
  #[inline] pub fn dcgctimer(&self) -> Dcgctimer { 
     unsafe {
        Dcgctimer(::core::ptr::read_volatile(((self.0 as usize) + 0x804) as *const u32))
     }
  }
  #[inline] pub fn set_dcgctimer(&self, value: Dcgctimer) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x804) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dcgctimer<F: FnOnce(Dcgctimer) -> Dcgctimer>(&self, f: F) -> &Self {
     let tmp = self.dcgctimer();
     self.set_dcgctimer(f(tmp))
  }

  #[inline] pub fn dcgcgpio_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x808) as *const u32
  }
  #[inline] pub fn dcgcgpio_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x808) as *mut u32
  }
  #[inline] pub fn dcgcgpio(&self) -> Dcgcgpio { 
     unsafe {
        Dcgcgpio(::core::ptr::read_volatile(((self.0 as usize) + 0x808) as *const u32))
     }
  }
  #[inline] pub fn set_dcgcgpio(&self, value: Dcgcgpio) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x808) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dcgcgpio<F: FnOnce(Dcgcgpio) -> Dcgcgpio>(&self, f: F) -> &Self {
     let tmp = self.dcgcgpio();
     self.set_dcgcgpio(f(tmp))
  }

  #[inline] pub fn dcgcdma_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x80c) as *const u32
  }
  #[inline] pub fn dcgcdma_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x80c) as *mut u32
  }
  #[inline] pub fn dcgcdma(&self) -> Dcgcdma { 
     unsafe {
        Dcgcdma(::core::ptr::read_volatile(((self.0 as usize) + 0x80c) as *const u32))
     }
  }
  #[inline] pub fn set_dcgcdma(&self, value: Dcgcdma) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x80c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dcgcdma<F: FnOnce(Dcgcdma) -> Dcgcdma>(&self, f: F) -> &Self {
     let tmp = self.dcgcdma();
     self.set_dcgcdma(f(tmp))
  }

  #[inline] pub fn dcgcepi_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x810) as *const u32
  }
  #[inline] pub fn dcgcepi_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x810) as *mut u32
  }
  #[inline] pub fn dcgcepi(&self) -> Dcgcepi { 
     unsafe {
        Dcgcepi(::core::ptr::read_volatile(((self.0 as usize) + 0x810) as *const u32))
     }
  }
  #[inline] pub fn set_dcgcepi(&self, value: Dcgcepi) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x810) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dcgcepi<F: FnOnce(Dcgcepi) -> Dcgcepi>(&self, f: F) -> &Self {
     let tmp = self.dcgcepi();
     self.set_dcgcepi(f(tmp))
  }

  #[inline] pub fn dcgchib_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x814) as *const u32
  }
  #[inline] pub fn dcgchib_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x814) as *mut u32
  }
  #[inline] pub fn dcgchib(&self) -> Dcgchib { 
     unsafe {
        Dcgchib(::core::ptr::read_volatile(((self.0 as usize) + 0x814) as *const u32))
     }
  }
  #[inline] pub fn set_dcgchib(&self, value: Dcgchib) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x814) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dcgchib<F: FnOnce(Dcgchib) -> Dcgchib>(&self, f: F) -> &Self {
     let tmp = self.dcgchib();
     self.set_dcgchib(f(tmp))
  }

  #[inline] pub fn dcgcuart_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x818) as *const u32
  }
  #[inline] pub fn dcgcuart_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x818) as *mut u32
  }
  #[inline] pub fn dcgcuart(&self) -> Dcgcuart { 
     unsafe {
        Dcgcuart(::core::ptr::read_volatile(((self.0 as usize) + 0x818) as *const u32))
     }
  }
  #[inline] pub fn set_dcgcuart(&self, value: Dcgcuart) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x818) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dcgcuart<F: FnOnce(Dcgcuart) -> Dcgcuart>(&self, f: F) -> &Self {
     let tmp = self.dcgcuart();
     self.set_dcgcuart(f(tmp))
  }

  #[inline] pub fn dcgcssi_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x81c) as *const u32
  }
  #[inline] pub fn dcgcssi_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x81c) as *mut u32
  }
  #[inline] pub fn dcgcssi(&self) -> Dcgcssi { 
     unsafe {
        Dcgcssi(::core::ptr::read_volatile(((self.0 as usize) + 0x81c) as *const u32))
     }
  }
  #[inline] pub fn set_dcgcssi(&self, value: Dcgcssi) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x81c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dcgcssi<F: FnOnce(Dcgcssi) -> Dcgcssi>(&self, f: F) -> &Self {
     let tmp = self.dcgcssi();
     self.set_dcgcssi(f(tmp))
  }

  #[inline] pub fn dcgci2c_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x820) as *const u32
  }
  #[inline] pub fn dcgci2c_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x820) as *mut u32
  }
  #[inline] pub fn dcgci2c(&self) -> Dcgci2c { 
     unsafe {
        Dcgci2c(::core::ptr::read_volatile(((self.0 as usize) + 0x820) as *const u32))
     }
  }
  #[inline] pub fn set_dcgci2c(&self, value: Dcgci2c) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x820) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dcgci2c<F: FnOnce(Dcgci2c) -> Dcgci2c>(&self, f: F) -> &Self {
     let tmp = self.dcgci2c();
     self.set_dcgci2c(f(tmp))
  }

  #[inline] pub fn dcgcusb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x828) as *const u32
  }
  #[inline] pub fn dcgcusb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x828) as *mut u32
  }
  #[inline] pub fn dcgcusb(&self) -> Dcgcusb { 
     unsafe {
        Dcgcusb(::core::ptr::read_volatile(((self.0 as usize) + 0x828) as *const u32))
     }
  }
  #[inline] pub fn set_dcgcusb(&self, value: Dcgcusb) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x828) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dcgcusb<F: FnOnce(Dcgcusb) -> Dcgcusb>(&self, f: F) -> &Self {
     let tmp = self.dcgcusb();
     self.set_dcgcusb(f(tmp))
  }

  #[inline] pub fn dcgcephy_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x830) as *const u32
  }
  #[inline] pub fn dcgcephy_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x830) as *mut u32
  }
  #[inline] pub fn dcgcephy(&self) -> Dcgcephy { 
     unsafe {
        Dcgcephy(::core::ptr::read_volatile(((self.0 as usize) + 0x830) as *const u32))
     }
  }
  #[inline] pub fn set_dcgcephy(&self, value: Dcgcephy) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x830) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dcgcephy<F: FnOnce(Dcgcephy) -> Dcgcephy>(&self, f: F) -> &Self {
     let tmp = self.dcgcephy();
     self.set_dcgcephy(f(tmp))
  }

  #[inline] pub fn dcgccan_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x834) as *const u32
  }
  #[inline] pub fn dcgccan_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x834) as *mut u32
  }
  #[inline] pub fn dcgccan(&self) -> Dcgccan { 
     unsafe {
        Dcgccan(::core::ptr::read_volatile(((self.0 as usize) + 0x834) as *const u32))
     }
  }
  #[inline] pub fn set_dcgccan(&self, value: Dcgccan) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x834) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dcgccan<F: FnOnce(Dcgccan) -> Dcgccan>(&self, f: F) -> &Self {
     let tmp = self.dcgccan();
     self.set_dcgccan(f(tmp))
  }

  #[inline] pub fn dcgcadc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x838) as *const u32
  }
  #[inline] pub fn dcgcadc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x838) as *mut u32
  }
  #[inline] pub fn dcgcadc(&self) -> Dcgcadc { 
     unsafe {
        Dcgcadc(::core::ptr::read_volatile(((self.0 as usize) + 0x838) as *const u32))
     }
  }
  #[inline] pub fn set_dcgcadc(&self, value: Dcgcadc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x838) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dcgcadc<F: FnOnce(Dcgcadc) -> Dcgcadc>(&self, f: F) -> &Self {
     let tmp = self.dcgcadc();
     self.set_dcgcadc(f(tmp))
  }

  #[inline] pub fn dcgcacmp_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x83c) as *const u32
  }
  #[inline] pub fn dcgcacmp_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x83c) as *mut u32
  }
  #[inline] pub fn dcgcacmp(&self) -> Dcgcacmp { 
     unsafe {
        Dcgcacmp(::core::ptr::read_volatile(((self.0 as usize) + 0x83c) as *const u32))
     }
  }
  #[inline] pub fn set_dcgcacmp(&self, value: Dcgcacmp) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x83c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dcgcacmp<F: FnOnce(Dcgcacmp) -> Dcgcacmp>(&self, f: F) -> &Self {
     let tmp = self.dcgcacmp();
     self.set_dcgcacmp(f(tmp))
  }

  #[inline] pub fn dcgcpwm_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x840) as *const u32
  }
  #[inline] pub fn dcgcpwm_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x840) as *mut u32
  }
  #[inline] pub fn dcgcpwm(&self) -> Dcgcpwm { 
     unsafe {
        Dcgcpwm(::core::ptr::read_volatile(((self.0 as usize) + 0x840) as *const u32))
     }
  }
  #[inline] pub fn set_dcgcpwm(&self, value: Dcgcpwm) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x840) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dcgcpwm<F: FnOnce(Dcgcpwm) -> Dcgcpwm>(&self, f: F) -> &Self {
     let tmp = self.dcgcpwm();
     self.set_dcgcpwm(f(tmp))
  }

  #[inline] pub fn dcgcqei_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x844) as *const u32
  }
  #[inline] pub fn dcgcqei_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x844) as *mut u32
  }
  #[inline] pub fn dcgcqei(&self) -> Dcgcqei { 
     unsafe {
        Dcgcqei(::core::ptr::read_volatile(((self.0 as usize) + 0x844) as *const u32))
     }
  }
  #[inline] pub fn set_dcgcqei(&self, value: Dcgcqei) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x844) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dcgcqei<F: FnOnce(Dcgcqei) -> Dcgcqei>(&self, f: F) -> &Self {
     let tmp = self.dcgcqei();
     self.set_dcgcqei(f(tmp))
  }

  #[inline] pub fn dcgceeprom_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x858) as *const u32
  }
  #[inline] pub fn dcgceeprom_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x858) as *mut u32
  }
  #[inline] pub fn dcgceeprom(&self) -> Dcgceeprom { 
     unsafe {
        Dcgceeprom(::core::ptr::read_volatile(((self.0 as usize) + 0x858) as *const u32))
     }
  }
  #[inline] pub fn set_dcgceeprom(&self, value: Dcgceeprom) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x858) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dcgceeprom<F: FnOnce(Dcgceeprom) -> Dcgceeprom>(&self, f: F) -> &Self {
     let tmp = self.dcgceeprom();
     self.set_dcgceeprom(f(tmp))
  }

  #[inline] pub fn dcgcccm_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x874) as *const u32
  }
  #[inline] pub fn dcgcccm_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x874) as *mut u32
  }
  #[inline] pub fn dcgcccm(&self) -> Dcgcccm { 
     unsafe {
        Dcgcccm(::core::ptr::read_volatile(((self.0 as usize) + 0x874) as *const u32))
     }
  }
  #[inline] pub fn set_dcgcccm(&self, value: Dcgcccm) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x874) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dcgcccm<F: FnOnce(Dcgcccm) -> Dcgcccm>(&self, f: F) -> &Self {
     let tmp = self.dcgcccm();
     self.set_dcgcccm(f(tmp))
  }

  #[inline] pub fn dcgcemac_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x89c) as *const u32
  }
  #[inline] pub fn dcgcemac_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x89c) as *mut u32
  }
  #[inline] pub fn dcgcemac(&self) -> Dcgcemac { 
     unsafe {
        Dcgcemac(::core::ptr::read_volatile(((self.0 as usize) + 0x89c) as *const u32))
     }
  }
  #[inline] pub fn set_dcgcemac(&self, value: Dcgcemac) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x89c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dcgcemac<F: FnOnce(Dcgcemac) -> Dcgcemac>(&self, f: F) -> &Self {
     let tmp = self.dcgcemac();
     self.set_dcgcemac(f(tmp))
  }

  #[inline] pub fn pcwd_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x900) as *const u32
  }
  #[inline] pub fn pcwd_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x900) as *mut u32
  }
  #[inline] pub fn pcwd(&self) -> Pcwd { 
     unsafe {
        Pcwd(::core::ptr::read_volatile(((self.0 as usize) + 0x900) as *const u32))
     }
  }
  #[inline] pub fn set_pcwd(&self, value: Pcwd) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x900) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pcwd<F: FnOnce(Pcwd) -> Pcwd>(&self, f: F) -> &Self {
     let tmp = self.pcwd();
     self.set_pcwd(f(tmp))
  }

  #[inline] pub fn pctimer_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x904) as *const u32
  }
  #[inline] pub fn pctimer_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x904) as *mut u32
  }
  #[inline] pub fn pctimer(&self) -> Pctimer { 
     unsafe {
        Pctimer(::core::ptr::read_volatile(((self.0 as usize) + 0x904) as *const u32))
     }
  }
  #[inline] pub fn set_pctimer(&self, value: Pctimer) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x904) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pctimer<F: FnOnce(Pctimer) -> Pctimer>(&self, f: F) -> &Self {
     let tmp = self.pctimer();
     self.set_pctimer(f(tmp))
  }

  #[inline] pub fn pcgpio_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x908) as *const u32
  }
  #[inline] pub fn pcgpio_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x908) as *mut u32
  }
  #[inline] pub fn pcgpio(&self) -> Pcgpio { 
     unsafe {
        Pcgpio(::core::ptr::read_volatile(((self.0 as usize) + 0x908) as *const u32))
     }
  }
  #[inline] pub fn set_pcgpio(&self, value: Pcgpio) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x908) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pcgpio<F: FnOnce(Pcgpio) -> Pcgpio>(&self, f: F) -> &Self {
     let tmp = self.pcgpio();
     self.set_pcgpio(f(tmp))
  }

  #[inline] pub fn pcdma_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x90c) as *const u32
  }
  #[inline] pub fn pcdma_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x90c) as *mut u32
  }
  #[inline] pub fn pcdma(&self) -> Pcdma { 
     unsafe {
        Pcdma(::core::ptr::read_volatile(((self.0 as usize) + 0x90c) as *const u32))
     }
  }
  #[inline] pub fn set_pcdma(&self, value: Pcdma) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x90c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pcdma<F: FnOnce(Pcdma) -> Pcdma>(&self, f: F) -> &Self {
     let tmp = self.pcdma();
     self.set_pcdma(f(tmp))
  }

  #[inline] pub fn pcepi_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x910) as *const u32
  }
  #[inline] pub fn pcepi_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x910) as *mut u32
  }
  #[inline] pub fn pcepi(&self) -> Pcepi { 
     unsafe {
        Pcepi(::core::ptr::read_volatile(((self.0 as usize) + 0x910) as *const u32))
     }
  }
  #[inline] pub fn set_pcepi(&self, value: Pcepi) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x910) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pcepi<F: FnOnce(Pcepi) -> Pcepi>(&self, f: F) -> &Self {
     let tmp = self.pcepi();
     self.set_pcepi(f(tmp))
  }

  #[inline] pub fn pchib_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x914) as *const u32
  }
  #[inline] pub fn pchib_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x914) as *mut u32
  }
  #[inline] pub fn pchib(&self) -> Pchib { 
     unsafe {
        Pchib(::core::ptr::read_volatile(((self.0 as usize) + 0x914) as *const u32))
     }
  }
  #[inline] pub fn set_pchib(&self, value: Pchib) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x914) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pchib<F: FnOnce(Pchib) -> Pchib>(&self, f: F) -> &Self {
     let tmp = self.pchib();
     self.set_pchib(f(tmp))
  }

  #[inline] pub fn pcuart_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x918) as *const u32
  }
  #[inline] pub fn pcuart_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x918) as *mut u32
  }
  #[inline] pub fn pcuart(&self) -> Pcuart { 
     unsafe {
        Pcuart(::core::ptr::read_volatile(((self.0 as usize) + 0x918) as *const u32))
     }
  }
  #[inline] pub fn set_pcuart(&self, value: Pcuart) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x918) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pcuart<F: FnOnce(Pcuart) -> Pcuart>(&self, f: F) -> &Self {
     let tmp = self.pcuart();
     self.set_pcuart(f(tmp))
  }

  #[inline] pub fn pcssi_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x91c) as *const u32
  }
  #[inline] pub fn pcssi_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x91c) as *mut u32
  }
  #[inline] pub fn pcssi(&self) -> Pcssi { 
     unsafe {
        Pcssi(::core::ptr::read_volatile(((self.0 as usize) + 0x91c) as *const u32))
     }
  }
  #[inline] pub fn set_pcssi(&self, value: Pcssi) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x91c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pcssi<F: FnOnce(Pcssi) -> Pcssi>(&self, f: F) -> &Self {
     let tmp = self.pcssi();
     self.set_pcssi(f(tmp))
  }

  #[inline] pub fn pci2c_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x920) as *const u32
  }
  #[inline] pub fn pci2c_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x920) as *mut u32
  }
  #[inline] pub fn pci2c(&self) -> Pci2c { 
     unsafe {
        Pci2c(::core::ptr::read_volatile(((self.0 as usize) + 0x920) as *const u32))
     }
  }
  #[inline] pub fn set_pci2c(&self, value: Pci2c) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x920) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pci2c<F: FnOnce(Pci2c) -> Pci2c>(&self, f: F) -> &Self {
     let tmp = self.pci2c();
     self.set_pci2c(f(tmp))
  }

  #[inline] pub fn pcusb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x928) as *const u32
  }
  #[inline] pub fn pcusb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x928) as *mut u32
  }
  #[inline] pub fn pcusb(&self) -> Pcusb { 
     unsafe {
        Pcusb(::core::ptr::read_volatile(((self.0 as usize) + 0x928) as *const u32))
     }
  }
  #[inline] pub fn set_pcusb(&self, value: Pcusb) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x928) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pcusb<F: FnOnce(Pcusb) -> Pcusb>(&self, f: F) -> &Self {
     let tmp = self.pcusb();
     self.set_pcusb(f(tmp))
  }

  #[inline] pub fn pcephy_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x930) as *const u32
  }
  #[inline] pub fn pcephy_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x930) as *mut u32
  }
  #[inline] pub fn pcephy(&self) -> Pcephy { 
     unsafe {
        Pcephy(::core::ptr::read_volatile(((self.0 as usize) + 0x930) as *const u32))
     }
  }
  #[inline] pub fn set_pcephy(&self, value: Pcephy) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x930) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pcephy<F: FnOnce(Pcephy) -> Pcephy>(&self, f: F) -> &Self {
     let tmp = self.pcephy();
     self.set_pcephy(f(tmp))
  }

  #[inline] pub fn pccan_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x934) as *const u32
  }
  #[inline] pub fn pccan_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x934) as *mut u32
  }
  #[inline] pub fn pccan(&self) -> Pccan { 
     unsafe {
        Pccan(::core::ptr::read_volatile(((self.0 as usize) + 0x934) as *const u32))
     }
  }
  #[inline] pub fn set_pccan(&self, value: Pccan) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x934) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pccan<F: FnOnce(Pccan) -> Pccan>(&self, f: F) -> &Self {
     let tmp = self.pccan();
     self.set_pccan(f(tmp))
  }

  #[inline] pub fn pcadc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x938) as *const u32
  }
  #[inline] pub fn pcadc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x938) as *mut u32
  }
  #[inline] pub fn pcadc(&self) -> Pcadc { 
     unsafe {
        Pcadc(::core::ptr::read_volatile(((self.0 as usize) + 0x938) as *const u32))
     }
  }
  #[inline] pub fn set_pcadc(&self, value: Pcadc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x938) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pcadc<F: FnOnce(Pcadc) -> Pcadc>(&self, f: F) -> &Self {
     let tmp = self.pcadc();
     self.set_pcadc(f(tmp))
  }

  #[inline] pub fn pcacmp_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x93c) as *const u32
  }
  #[inline] pub fn pcacmp_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x93c) as *mut u32
  }
  #[inline] pub fn pcacmp(&self) -> Pcacmp { 
     unsafe {
        Pcacmp(::core::ptr::read_volatile(((self.0 as usize) + 0x93c) as *const u32))
     }
  }
  #[inline] pub fn set_pcacmp(&self, value: Pcacmp) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x93c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pcacmp<F: FnOnce(Pcacmp) -> Pcacmp>(&self, f: F) -> &Self {
     let tmp = self.pcacmp();
     self.set_pcacmp(f(tmp))
  }

  #[inline] pub fn pcpwm_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x940) as *const u32
  }
  #[inline] pub fn pcpwm_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x940) as *mut u32
  }
  #[inline] pub fn pcpwm(&self) -> Pcpwm { 
     unsafe {
        Pcpwm(::core::ptr::read_volatile(((self.0 as usize) + 0x940) as *const u32))
     }
  }
  #[inline] pub fn set_pcpwm(&self, value: Pcpwm) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x940) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pcpwm<F: FnOnce(Pcpwm) -> Pcpwm>(&self, f: F) -> &Self {
     let tmp = self.pcpwm();
     self.set_pcpwm(f(tmp))
  }

  #[inline] pub fn pcqei_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x944) as *const u32
  }
  #[inline] pub fn pcqei_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x944) as *mut u32
  }
  #[inline] pub fn pcqei(&self) -> Pcqei { 
     unsafe {
        Pcqei(::core::ptr::read_volatile(((self.0 as usize) + 0x944) as *const u32))
     }
  }
  #[inline] pub fn set_pcqei(&self, value: Pcqei) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x944) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pcqei<F: FnOnce(Pcqei) -> Pcqei>(&self, f: F) -> &Self {
     let tmp = self.pcqei();
     self.set_pcqei(f(tmp))
  }

  #[inline] pub fn pceeprom_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x958) as *const u32
  }
  #[inline] pub fn pceeprom_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x958) as *mut u32
  }
  #[inline] pub fn pceeprom(&self) -> Pceeprom { 
     unsafe {
        Pceeprom(::core::ptr::read_volatile(((self.0 as usize) + 0x958) as *const u32))
     }
  }
  #[inline] pub fn set_pceeprom(&self, value: Pceeprom) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x958) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pceeprom<F: FnOnce(Pceeprom) -> Pceeprom>(&self, f: F) -> &Self {
     let tmp = self.pceeprom();
     self.set_pceeprom(f(tmp))
  }

  #[inline] pub fn pcccm_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x974) as *const u32
  }
  #[inline] pub fn pcccm_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x974) as *mut u32
  }
  #[inline] pub fn pcccm(&self) -> Pcccm { 
     unsafe {
        Pcccm(::core::ptr::read_volatile(((self.0 as usize) + 0x974) as *const u32))
     }
  }
  #[inline] pub fn set_pcccm(&self, value: Pcccm) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x974) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pcccm<F: FnOnce(Pcccm) -> Pcccm>(&self, f: F) -> &Self {
     let tmp = self.pcccm();
     self.set_pcccm(f(tmp))
  }

  #[inline] pub fn pcemac_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x99c) as *const u32
  }
  #[inline] pub fn pcemac_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x99c) as *mut u32
  }
  #[inline] pub fn pcemac(&self) -> Pcemac { 
     unsafe {
        Pcemac(::core::ptr::read_volatile(((self.0 as usize) + 0x99c) as *const u32))
     }
  }
  #[inline] pub fn set_pcemac(&self, value: Pcemac) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x99c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pcemac<F: FnOnce(Pcemac) -> Pcemac>(&self, f: F) -> &Self {
     let tmp = self.pcemac();
     self.set_pcemac(f(tmp))
  }

  #[inline] pub fn prwd_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa00) as *const u32
  }
  #[inline] pub fn prwd_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa00) as *mut u32
  }
  #[inline] pub fn prwd(&self) -> Prwd { 
     unsafe {
        Prwd(::core::ptr::read_volatile(((self.0 as usize) + 0xa00) as *const u32))
     }
  }
  #[inline] pub fn set_prwd(&self, value: Prwd) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa00) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_prwd<F: FnOnce(Prwd) -> Prwd>(&self, f: F) -> &Self {
     let tmp = self.prwd();
     self.set_prwd(f(tmp))
  }

  #[inline] pub fn prtimer_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa04) as *const u32
  }
  #[inline] pub fn prtimer_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa04) as *mut u32
  }
  #[inline] pub fn prtimer(&self) -> Prtimer { 
     unsafe {
        Prtimer(::core::ptr::read_volatile(((self.0 as usize) + 0xa04) as *const u32))
     }
  }
  #[inline] pub fn set_prtimer(&self, value: Prtimer) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa04) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_prtimer<F: FnOnce(Prtimer) -> Prtimer>(&self, f: F) -> &Self {
     let tmp = self.prtimer();
     self.set_prtimer(f(tmp))
  }

  #[inline] pub fn prgpio_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa08) as *const u32
  }
  #[inline] pub fn prgpio_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa08) as *mut u32
  }
  #[inline] pub fn prgpio(&self) -> Prgpio { 
     unsafe {
        Prgpio(::core::ptr::read_volatile(((self.0 as usize) + 0xa08) as *const u32))
     }
  }
  #[inline] pub fn set_prgpio(&self, value: Prgpio) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa08) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_prgpio<F: FnOnce(Prgpio) -> Prgpio>(&self, f: F) -> &Self {
     let tmp = self.prgpio();
     self.set_prgpio(f(tmp))
  }

  #[inline] pub fn prdma_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa0c) as *const u32
  }
  #[inline] pub fn prdma_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa0c) as *mut u32
  }
  #[inline] pub fn prdma(&self) -> Prdma { 
     unsafe {
        Prdma(::core::ptr::read_volatile(((self.0 as usize) + 0xa0c) as *const u32))
     }
  }
  #[inline] pub fn set_prdma(&self, value: Prdma) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa0c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_prdma<F: FnOnce(Prdma) -> Prdma>(&self, f: F) -> &Self {
     let tmp = self.prdma();
     self.set_prdma(f(tmp))
  }

  #[inline] pub fn prepi_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa10) as *const u32
  }
  #[inline] pub fn prepi_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa10) as *mut u32
  }
  #[inline] pub fn prepi(&self) -> Prepi { 
     unsafe {
        Prepi(::core::ptr::read_volatile(((self.0 as usize) + 0xa10) as *const u32))
     }
  }
  #[inline] pub fn set_prepi(&self, value: Prepi) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa10) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_prepi<F: FnOnce(Prepi) -> Prepi>(&self, f: F) -> &Self {
     let tmp = self.prepi();
     self.set_prepi(f(tmp))
  }

  #[inline] pub fn prhib_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa14) as *const u32
  }
  #[inline] pub fn prhib_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa14) as *mut u32
  }
  #[inline] pub fn prhib(&self) -> Prhib { 
     unsafe {
        Prhib(::core::ptr::read_volatile(((self.0 as usize) + 0xa14) as *const u32))
     }
  }
  #[inline] pub fn set_prhib(&self, value: Prhib) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa14) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_prhib<F: FnOnce(Prhib) -> Prhib>(&self, f: F) -> &Self {
     let tmp = self.prhib();
     self.set_prhib(f(tmp))
  }

  #[inline] pub fn pruart_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa18) as *const u32
  }
  #[inline] pub fn pruart_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa18) as *mut u32
  }
  #[inline] pub fn pruart(&self) -> Pruart { 
     unsafe {
        Pruart(::core::ptr::read_volatile(((self.0 as usize) + 0xa18) as *const u32))
     }
  }
  #[inline] pub fn set_pruart(&self, value: Pruart) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa18) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pruart<F: FnOnce(Pruart) -> Pruart>(&self, f: F) -> &Self {
     let tmp = self.pruart();
     self.set_pruart(f(tmp))
  }

  #[inline] pub fn prssi_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa1c) as *const u32
  }
  #[inline] pub fn prssi_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa1c) as *mut u32
  }
  #[inline] pub fn prssi(&self) -> Prssi { 
     unsafe {
        Prssi(::core::ptr::read_volatile(((self.0 as usize) + 0xa1c) as *const u32))
     }
  }
  #[inline] pub fn set_prssi(&self, value: Prssi) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa1c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_prssi<F: FnOnce(Prssi) -> Prssi>(&self, f: F) -> &Self {
     let tmp = self.prssi();
     self.set_prssi(f(tmp))
  }

  #[inline] pub fn pri2c_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa20) as *const u32
  }
  #[inline] pub fn pri2c_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa20) as *mut u32
  }
  #[inline] pub fn pri2c(&self) -> Pri2c { 
     unsafe {
        Pri2c(::core::ptr::read_volatile(((self.0 as usize) + 0xa20) as *const u32))
     }
  }
  #[inline] pub fn set_pri2c(&self, value: Pri2c) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa20) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pri2c<F: FnOnce(Pri2c) -> Pri2c>(&self, f: F) -> &Self {
     let tmp = self.pri2c();
     self.set_pri2c(f(tmp))
  }

  #[inline] pub fn prusb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa28) as *const u32
  }
  #[inline] pub fn prusb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa28) as *mut u32
  }
  #[inline] pub fn prusb(&self) -> Prusb { 
     unsafe {
        Prusb(::core::ptr::read_volatile(((self.0 as usize) + 0xa28) as *const u32))
     }
  }
  #[inline] pub fn set_prusb(&self, value: Prusb) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa28) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_prusb<F: FnOnce(Prusb) -> Prusb>(&self, f: F) -> &Self {
     let tmp = self.prusb();
     self.set_prusb(f(tmp))
  }

  #[inline] pub fn prephy_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa30) as *const u32
  }
  #[inline] pub fn prephy_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa30) as *mut u32
  }
  #[inline] pub fn prephy(&self) -> Prephy { 
     unsafe {
        Prephy(::core::ptr::read_volatile(((self.0 as usize) + 0xa30) as *const u32))
     }
  }
  #[inline] pub fn set_prephy(&self, value: Prephy) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa30) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_prephy<F: FnOnce(Prephy) -> Prephy>(&self, f: F) -> &Self {
     let tmp = self.prephy();
     self.set_prephy(f(tmp))
  }

  #[inline] pub fn prcan_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa34) as *const u32
  }
  #[inline] pub fn prcan_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa34) as *mut u32
  }
  #[inline] pub fn prcan(&self) -> Prcan { 
     unsafe {
        Prcan(::core::ptr::read_volatile(((self.0 as usize) + 0xa34) as *const u32))
     }
  }
  #[inline] pub fn set_prcan(&self, value: Prcan) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa34) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_prcan<F: FnOnce(Prcan) -> Prcan>(&self, f: F) -> &Self {
     let tmp = self.prcan();
     self.set_prcan(f(tmp))
  }

  #[inline] pub fn pradc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa38) as *const u32
  }
  #[inline] pub fn pradc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa38) as *mut u32
  }
  #[inline] pub fn pradc(&self) -> Pradc { 
     unsafe {
        Pradc(::core::ptr::read_volatile(((self.0 as usize) + 0xa38) as *const u32))
     }
  }
  #[inline] pub fn set_pradc(&self, value: Pradc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa38) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pradc<F: FnOnce(Pradc) -> Pradc>(&self, f: F) -> &Self {
     let tmp = self.pradc();
     self.set_pradc(f(tmp))
  }

  #[inline] pub fn pracmp_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa3c) as *const u32
  }
  #[inline] pub fn pracmp_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa3c) as *mut u32
  }
  #[inline] pub fn pracmp(&self) -> Pracmp { 
     unsafe {
        Pracmp(::core::ptr::read_volatile(((self.0 as usize) + 0xa3c) as *const u32))
     }
  }
  #[inline] pub fn set_pracmp(&self, value: Pracmp) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa3c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pracmp<F: FnOnce(Pracmp) -> Pracmp>(&self, f: F) -> &Self {
     let tmp = self.pracmp();
     self.set_pracmp(f(tmp))
  }

  #[inline] pub fn prpwm_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa40) as *const u32
  }
  #[inline] pub fn prpwm_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa40) as *mut u32
  }
  #[inline] pub fn prpwm(&self) -> Prpwm { 
     unsafe {
        Prpwm(::core::ptr::read_volatile(((self.0 as usize) + 0xa40) as *const u32))
     }
  }
  #[inline] pub fn set_prpwm(&self, value: Prpwm) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa40) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_prpwm<F: FnOnce(Prpwm) -> Prpwm>(&self, f: F) -> &Self {
     let tmp = self.prpwm();
     self.set_prpwm(f(tmp))
  }

  #[inline] pub fn prqei_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa44) as *const u32
  }
  #[inline] pub fn prqei_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa44) as *mut u32
  }
  #[inline] pub fn prqei(&self) -> Prqei { 
     unsafe {
        Prqei(::core::ptr::read_volatile(((self.0 as usize) + 0xa44) as *const u32))
     }
  }
  #[inline] pub fn set_prqei(&self, value: Prqei) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa44) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_prqei<F: FnOnce(Prqei) -> Prqei>(&self, f: F) -> &Self {
     let tmp = self.prqei();
     self.set_prqei(f(tmp))
  }

  #[inline] pub fn preeprom_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa58) as *const u32
  }
  #[inline] pub fn preeprom_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa58) as *mut u32
  }
  #[inline] pub fn preeprom(&self) -> Preeprom { 
     unsafe {
        Preeprom(::core::ptr::read_volatile(((self.0 as usize) + 0xa58) as *const u32))
     }
  }
  #[inline] pub fn set_preeprom(&self, value: Preeprom) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa58) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_preeprom<F: FnOnce(Preeprom) -> Preeprom>(&self, f: F) -> &Self {
     let tmp = self.preeprom();
     self.set_preeprom(f(tmp))
  }

  #[inline] pub fn prccm_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa74) as *const u32
  }
  #[inline] pub fn prccm_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa74) as *mut u32
  }
  #[inline] pub fn prccm(&self) -> Prccm { 
     unsafe {
        Prccm(::core::ptr::read_volatile(((self.0 as usize) + 0xa74) as *const u32))
     }
  }
  #[inline] pub fn set_prccm(&self, value: Prccm) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa74) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_prccm<F: FnOnce(Prccm) -> Prccm>(&self, f: F) -> &Self {
     let tmp = self.prccm();
     self.set_prccm(f(tmp))
  }

  #[inline] pub fn premac_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa9c) as *const u32
  }
  #[inline] pub fn premac_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa9c) as *mut u32
  }
  #[inline] pub fn premac(&self) -> Premac { 
     unsafe {
        Premac(::core::ptr::read_volatile(((self.0 as usize) + 0xa9c) as *const u32))
     }
  }
  #[inline] pub fn set_premac(&self, value: Premac) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa9c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_premac<F: FnOnce(Premac) -> Premac>(&self, f: F) -> &Self {
     let tmp = self.premac();
     self.set_premac(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Did0(pub u32);
impl Did0 {
  #[inline] pub fn min(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  #[inline] pub fn set_min(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn maj(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
  #[inline] pub fn set_maj(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn class(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  #[inline] pub fn set_class(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  #[inline] pub fn ver(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x7 // [30:28]
  }
  #[inline] pub fn set_ver(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 28);
     self.0 |= value << 28;
     self
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
#[derive(PartialEq, Eq)]
pub struct Did1(pub u32);
impl Did1 {
  #[inline] pub fn qual(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline] pub fn set_qual(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn rohs(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_rohs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pkg(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x3 // [4:3]
  }
  #[inline] pub fn set_pkg(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn temp(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x7 // [7:5]
  }
  #[inline] pub fn set_temp(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn pincnt(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x7 // [15:13]
  }
  #[inline] pub fn set_pincnt(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline] pub fn prtno(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  #[inline] pub fn set_prtno(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  #[inline] pub fn fam(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  #[inline] pub fn set_fam(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

  #[inline] pub fn ver(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0xf // [31:28]
  }
  #[inline] pub fn set_ver(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 28);
     self.0 |= value << 28;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ptboctl(pub u32);
impl Ptboctl {
  #[inline] pub fn vdd_ubor(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline] pub fn set_vdd_ubor(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn vdda_ubor(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  #[inline] pub fn set_vdda_ubor(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ris(pub u32);
impl Ris {
  #[inline] pub fn borris(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_borris(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn mofris(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_mofris(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn plllris(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_plllris(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn moscpupris(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_moscpupris(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
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
#[derive(PartialEq, Eq)]
pub struct Imc(pub u32);
impl Imc {
  #[inline] pub fn borim(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_borim(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn mofim(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_mofim(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn plllim(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_plllim(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn moscpupim(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_moscpupim(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
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
#[derive(PartialEq, Eq)]
pub struct Misc(pub u32);
impl Misc {
  #[inline] pub fn bormis(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_bormis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn mofmis(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_mofmis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn plllmis(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_plllmis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn moscpupmis(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_moscpupmis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
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
#[derive(PartialEq, Eq)]
pub struct Resc(pub u32);
impl Resc {
  #[inline] pub fn ext(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_ext(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn por(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_por(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn bor(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_bor(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn wdt0(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_wdt0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn sw(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_sw(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn wdt1(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_wdt1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn hib(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_hib(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn hssr(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline] pub fn set_hssr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn moscfail(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline] pub fn set_moscfail(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pwrtc(pub u32);
impl Pwrtc {
  #[inline] pub fn vdd_ubor(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_vdd_ubor(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn vdda_ubor(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_vdda_ubor(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
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
#[derive(PartialEq, Eq)]
pub struct Nmic(pub u32);
impl Nmic {
  #[inline] pub fn external(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_external(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn power(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_power(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn wdt0(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_wdt0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn wdt1(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_wdt1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn tamper(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_tamper(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn moscfail(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline] pub fn set_moscfail(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
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
#[derive(PartialEq, Eq)]
pub struct Moscctl(pub u32);
impl Moscctl {
  #[inline] pub fn cval(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_cval(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn moscim(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_moscim(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn noxtal(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_noxtal(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pwrdn(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pwrdn(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn oscrng(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_oscrng(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
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
#[derive(PartialEq, Eq)]
pub struct Rsclkcfg(pub u32);
impl Rsclkcfg {
  #[inline] pub fn psysdiv(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3ff // [9:0]
  }
  #[inline] pub fn set_psysdiv(mut self, value: u32) -> Self {
     assert!((value & !0x3ff) == 0);
     self.0 &= !(0x3ff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn osysdiv(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3ff // [19:10]
  }
  #[inline] pub fn set_osysdiv(mut self, value: u32) -> Self {
     assert!((value & !0x3ff) == 0);
     self.0 &= !(0x3ff << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn oscsrc(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0xf // [23:20]
  }
  #[inline] pub fn set_oscsrc(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

  #[inline] pub fn pllsrc(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  #[inline] pub fn set_pllsrc(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

  #[inline] pub fn usepll(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  #[inline] pub fn set_usepll(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

  #[inline] pub fn acg(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  #[inline] pub fn set_acg(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  #[inline] pub fn newfreq(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  #[inline] pub fn set_newfreq(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  #[inline] pub fn memtimu(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  #[inline] pub fn set_memtimu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
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
#[derive(PartialEq, Eq)]
pub struct Memtim0(pub u32);
impl Memtim0 {
  #[inline] pub fn fws(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_fws(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn fbce(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_fbce(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn fbcht(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0xf // [9:6]
  }
  #[inline] pub fn set_fbcht(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn ews(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  #[inline] pub fn set_ews(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  #[inline] pub fn ebce(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  #[inline] pub fn set_ebce(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  #[inline] pub fn ebcht(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0xf // [25:22]
  }
  #[inline] pub fn set_ebcht(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 22);
     self.0 |= value << 22;
     self
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
#[derive(PartialEq, Eq)]
pub struct Altclkcfg(pub u32);
impl Altclkcfg {
  #[inline] pub fn altclk(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_altclk(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Dsclkcfg(pub u32);
impl Dsclkcfg {
  #[inline] pub fn dssysdiv(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3ff // [9:0]
  }
  #[inline] pub fn set_dssysdiv(mut self, value: u32) -> Self {
     assert!((value & !0x3ff) == 0);
     self.0 &= !(0x3ff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn dsoscsrc(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0xf // [23:20]
  }
  #[inline] pub fn set_dsoscsrc(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

  #[inline] pub fn moscdpd(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  #[inline] pub fn set_moscdpd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  #[inline] pub fn pioscpd(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  #[inline] pub fn set_pioscpd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
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
#[derive(PartialEq, Eq)]
pub struct Divsclk(pub u32);
impl Divsclk {
  #[inline] pub fn div(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  #[inline] pub fn set_div(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn src(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3 // [17:16]
  }
  #[inline] pub fn set_src(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline] pub fn en(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  #[inline] pub fn set_en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
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
#[derive(PartialEq, Eq)]
pub struct Sysprop(pub u32);
impl Sysprop {
  #[inline] pub fn fpu(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_fpu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Piosccal(pub u32);
impl Piosccal {
  #[inline] pub fn ut(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7f // [6:0]
  }
  #[inline] pub fn set_ut(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn update(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_update(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn cal(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_cal(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn uten(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  #[inline] pub fn set_uten(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pioscstat(pub u32);
impl Pioscstat {
  #[inline] pub fn ct(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7f // [6:0]
  }
  #[inline] pub fn set_ct(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn cr(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  #[inline] pub fn set_cr(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn dt(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x7f // [22:16]
  }
  #[inline] pub fn set_dt(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 16);
     self.0 |= value << 16;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pllfreq0(pub u32);
impl Pllfreq0 {
  #[inline] pub fn mint(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3ff // [9:0]
  }
  #[inline] pub fn set_mint(mut self, value: u32) -> Self {
     assert!((value & !0x3ff) == 0);
     self.0 &= !(0x3ff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn mfrac(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3ff // [19:10]
  }
  #[inline] pub fn set_mfrac(mut self, value: u32) -> Self {
     assert!((value & !0x3ff) == 0);
     self.0 &= !(0x3ff << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn pllpwr(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  #[inline] pub fn set_pllpwr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pllfreq1(pub u32);
impl Pllfreq1 {
  #[inline] pub fn n(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1f // [4:0]
  }
  #[inline] pub fn set_n(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn q(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1f // [12:8]
  }
  #[inline] pub fn set_q(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 8);
     self.0 |= value << 8;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pllstat(pub u32);
impl Pllstat {
  #[inline] pub fn lock(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_lock(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Slppwrcfg(pub u32);
impl Slppwrcfg {
  #[inline] pub fn srampm(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline] pub fn set_srampm(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn flashpm(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  #[inline] pub fn set_flashpm(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
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
#[derive(PartialEq, Eq)]
pub struct Dslppwrcfg(pub u32);
impl Dslppwrcfg {
  #[inline] pub fn srampm(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline] pub fn set_srampm(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn flashpm(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  #[inline] pub fn set_flashpm(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn tspd(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_tspd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn ldosm(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_ldosm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
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
#[derive(PartialEq, Eq)]
pub struct Nvmstat(pub u32);
impl Nvmstat {
  #[inline] pub fn fwb(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_fwb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ldospctl(pub u32);
impl Ldospctl {
  #[inline] pub fn vldo(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  #[inline] pub fn set_vldo(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn vadjen(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  #[inline] pub fn set_vadjen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ldodpctl(pub u32);
impl Ldodpctl {
  #[inline] pub fn vldo(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  #[inline] pub fn set_vldo(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn vadjen(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  #[inline] pub fn set_vadjen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
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
#[derive(PartialEq, Eq)]
pub struct Resbehavctl(pub u32);
impl Resbehavctl {
  #[inline] pub fn extres(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline] pub fn set_extres(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn bor(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
  #[inline] pub fn set_bor(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn wdog0(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  #[inline] pub fn set_wdog0(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn wdog1(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
  #[inline] pub fn set_wdog1(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
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
#[derive(PartialEq, Eq)]
pub struct Hssr(pub u32);
impl Hssr {
  #[inline] pub fn cdoff(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffff // [23:0]
  }
  #[inline] pub fn set_cdoff(mut self, value: u32) -> Self {
     assert!((value & !0xffffff) == 0);
     self.0 &= !(0xffffff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn key(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
  #[inline] pub fn set_key(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
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
#[derive(PartialEq, Eq)]
pub struct Usbpds(pub u32);
impl Usbpds {
  #[inline] pub fn pwrstat(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline] pub fn set_pwrstat(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn memstat(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
  #[inline] pub fn set_memstat(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
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
#[derive(PartialEq, Eq)]
pub struct Usbmpc(pub u32);
impl Usbmpc {
  #[inline] pub fn pwrctl(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline] pub fn set_pwrctl(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Emacpds(pub u32);
impl Emacpds {
  #[inline] pub fn pwrstat(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline] pub fn set_pwrstat(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn memstat(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
  #[inline] pub fn set_memstat(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
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
#[derive(PartialEq, Eq)]
pub struct Emacmpc(pub u32);
impl Emacmpc {
  #[inline] pub fn pwrctl(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline] pub fn set_pwrctl(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ppwd(pub u32);
impl Ppwd {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_p1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pptimer(pub u32);
impl Pptimer {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_p1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn p2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_p2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn p3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_p3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn p4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_p4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn p5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_p5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn p6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_p6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn p7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_p7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ppgpio(pub u32);
impl Ppgpio {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_p1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn p2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_p2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn p3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_p3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn p4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_p4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn p5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_p5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn p6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_p6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn p7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_p7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn p8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_p8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn p9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_p9(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn p10(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_p10(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn p11(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_p11(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline] pub fn p12(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline] pub fn set_p12(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn p13(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline] pub fn set_p13(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline] pub fn p14(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline] pub fn set_p14(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ppdma(pub u32);
impl Ppdma {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ppepi(pub u32);
impl Ppepi {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pphib(pub u32);
impl Pphib {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ppuart(pub u32);
impl Ppuart {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_p1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn p2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_p2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn p3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_p3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn p4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_p4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn p5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_p5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn p6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_p6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn p7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_p7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ppssi(pub u32);
impl Ppssi {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_p1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn p2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_p2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn p3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_p3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ppi2c(pub u32);
impl Ppi2c {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_p1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn p2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_p2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn p3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_p3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn p4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_p4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn p5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_p5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn p6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_p6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn p7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_p7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn p8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_p8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn p9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_p9(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ppusb(pub u32);
impl Ppusb {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ppephy(pub u32);
impl Ppephy {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ppcan(pub u32);
impl Ppcan {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_p1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ppadc(pub u32);
impl Ppadc {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_p1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ppacmp(pub u32);
impl Ppacmp {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pppwm(pub u32);
impl Pppwm {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ppqei(pub u32);
impl Ppqei {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pplpc(pub u32);
impl Pplpc {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pppeci(pub u32);
impl Pppeci {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ppfan(pub u32);
impl Ppfan {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ppeeprom(pub u32);
impl Ppeeprom {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ppwtimer(pub u32);
impl Ppwtimer {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pprts(pub u32);
impl Pprts {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ppccm(pub u32);
impl Ppccm {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pplcd(pub u32);
impl Pplcd {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ppowire(pub u32);
impl Ppowire {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ppemac(pub u32);
impl Ppemac {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pphim(pub u32);
impl Pphim {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Srwd(pub u32);
impl Srwd {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
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
#[derive(PartialEq, Eq)]
pub struct Srtimer(pub u32);
impl Srtimer {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn r4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_r4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn r5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_r5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn r6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_r6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn r7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_r7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
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
#[derive(PartialEq, Eq)]
pub struct Srgpio(pub u32);
impl Srgpio {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn r4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_r4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn r5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_r5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn r6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_r6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn r7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_r7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn r8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_r8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn r9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_r9(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn r10(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_r10(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn r11(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_r11(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline] pub fn r12(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline] pub fn set_r12(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn r13(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline] pub fn set_r13(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline] pub fn r14(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline] pub fn set_r14(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
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
#[derive(PartialEq, Eq)]
pub struct Srdma(pub u32);
impl Srdma {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Srepi(pub u32);
impl Srepi {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Srhib(pub u32);
impl Srhib {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Sruart(pub u32);
impl Sruart {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn r4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_r4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn r5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_r5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn r6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_r6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn r7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_r7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
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
#[derive(PartialEq, Eq)]
pub struct Srssi(pub u32);
impl Srssi {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
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
#[derive(PartialEq, Eq)]
pub struct Sri2c(pub u32);
impl Sri2c {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn r4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_r4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn r5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_r5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn r6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_r6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn r7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_r7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn r8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_r8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn r9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_r9(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
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
#[derive(PartialEq, Eq)]
pub struct Srusb(pub u32);
impl Srusb {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Srephy(pub u32);
impl Srephy {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Srcan(pub u32);
impl Srcan {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
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
#[derive(PartialEq, Eq)]
pub struct Sradc(pub u32);
impl Sradc {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
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
#[derive(PartialEq, Eq)]
pub struct Sracmp(pub u32);
impl Sracmp {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Srpwm(pub u32);
impl Srpwm {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Srqei(pub u32);
impl Srqei {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Sreeprom(pub u32);
impl Sreeprom {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Srccm(pub u32);
impl Srccm {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Sremac(pub u32);
impl Sremac {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Rcgcwd(pub u32);
impl Rcgcwd {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
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
#[derive(PartialEq, Eq)]
pub struct Rcgctimer(pub u32);
impl Rcgctimer {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn r4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_r4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn r5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_r5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn r6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_r6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn r7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_r7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
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
#[derive(PartialEq, Eq)]
pub struct Rcgcgpio(pub u32);
impl Rcgcgpio {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn r4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_r4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn r5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_r5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn r6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_r6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn r7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_r7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn r8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_r8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn r9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_r9(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn r10(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_r10(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn r11(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_r11(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline] pub fn r12(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline] pub fn set_r12(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn r13(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline] pub fn set_r13(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline] pub fn r14(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline] pub fn set_r14(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
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
#[derive(PartialEq, Eq)]
pub struct Rcgcdma(pub u32);
impl Rcgcdma {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Rcgcepi(pub u32);
impl Rcgcepi {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Rcgchib(pub u32);
impl Rcgchib {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Rcgcuart(pub u32);
impl Rcgcuart {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn r4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_r4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn r5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_r5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn r6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_r6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn r7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_r7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
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
#[derive(PartialEq, Eq)]
pub struct Rcgcssi(pub u32);
impl Rcgcssi {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
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
#[derive(PartialEq, Eq)]
pub struct Rcgci2c(pub u32);
impl Rcgci2c {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn r4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_r4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn r5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_r5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn r6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_r6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn r7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_r7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn r8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_r8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn r9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_r9(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
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
#[derive(PartialEq, Eq)]
pub struct Rcgcusb(pub u32);
impl Rcgcusb {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Rcgcephy(pub u32);
impl Rcgcephy {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Rcgccan(pub u32);
impl Rcgccan {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
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
#[derive(PartialEq, Eq)]
pub struct Rcgcadc(pub u32);
impl Rcgcadc {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
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
#[derive(PartialEq, Eq)]
pub struct Rcgcacmp(pub u32);
impl Rcgcacmp {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Rcgcpwm(pub u32);
impl Rcgcpwm {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Rcgcqei(pub u32);
impl Rcgcqei {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Rcgceeprom(pub u32);
impl Rcgceeprom {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Rcgcccm(pub u32);
impl Rcgcccm {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Rcgcemac(pub u32);
impl Rcgcemac {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Scgcwd(pub u32);
impl Scgcwd {
  #[inline] pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn s1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_s1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
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
#[derive(PartialEq, Eq)]
pub struct Scgctimer(pub u32);
impl Scgctimer {
  #[inline] pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn s1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_s1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn s2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_s2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn s3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_s3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn s4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_s4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn s5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_s5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn s6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_s6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn s7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_s7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
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
#[derive(PartialEq, Eq)]
pub struct Scgcgpio(pub u32);
impl Scgcgpio {
  #[inline] pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn s1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_s1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn s2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_s2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn s3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_s3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn s4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_s4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn s5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_s5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn s6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_s6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn s7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_s7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn s8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_s8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn s9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_s9(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn s10(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_s10(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn s11(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_s11(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline] pub fn s12(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline] pub fn set_s12(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn s13(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline] pub fn set_s13(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline] pub fn s14(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline] pub fn set_s14(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
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
#[derive(PartialEq, Eq)]
pub struct Scgcdma(pub u32);
impl Scgcdma {
  #[inline] pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Scgcepi(pub u32);
impl Scgcepi {
  #[inline] pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Scgchib(pub u32);
impl Scgchib {
  #[inline] pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Scgcuart(pub u32);
impl Scgcuart {
  #[inline] pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn s1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_s1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn s2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_s2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn s3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_s3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn s4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_s4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn s5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_s5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn s6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_s6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn s7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_s7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
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
#[derive(PartialEq, Eq)]
pub struct Scgcssi(pub u32);
impl Scgcssi {
  #[inline] pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn s1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_s1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn s2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_s2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn s3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_s3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
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
#[derive(PartialEq, Eq)]
pub struct Scgci2c(pub u32);
impl Scgci2c {
  #[inline] pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn s1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_s1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn s2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_s2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn s3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_s3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn s4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_s4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn s5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_s5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn s6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_s6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn s7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_s7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn s8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_s8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn s9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_s9(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
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
#[derive(PartialEq, Eq)]
pub struct Scgcusb(pub u32);
impl Scgcusb {
  #[inline] pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Scgcephy(pub u32);
impl Scgcephy {
  #[inline] pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Scgccan(pub u32);
impl Scgccan {
  #[inline] pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn s1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_s1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
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
#[derive(PartialEq, Eq)]
pub struct Scgcadc(pub u32);
impl Scgcadc {
  #[inline] pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn s1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_s1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
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
#[derive(PartialEq, Eq)]
pub struct Scgcacmp(pub u32);
impl Scgcacmp {
  #[inline] pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Scgcpwm(pub u32);
impl Scgcpwm {
  #[inline] pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Scgcqei(pub u32);
impl Scgcqei {
  #[inline] pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Scgceeprom(pub u32);
impl Scgceeprom {
  #[inline] pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Scgcccm(pub u32);
impl Scgcccm {
  #[inline] pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Scgcemac(pub u32);
impl Scgcemac {
  #[inline] pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Dcgcwd(pub u32);
impl Dcgcwd {
  #[inline] pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn d1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_d1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
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
#[derive(PartialEq, Eq)]
pub struct Dcgctimer(pub u32);
impl Dcgctimer {
  #[inline] pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn d1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_d1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn d2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_d2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn d3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_d3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn d4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_d4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn d5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_d5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn d6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_d6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn d7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_d7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
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
#[derive(PartialEq, Eq)]
pub struct Dcgcgpio(pub u32);
impl Dcgcgpio {
  #[inline] pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn d1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_d1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn d2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_d2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn d3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_d3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn d4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_d4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn d5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_d5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn d6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_d6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn d7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_d7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn d8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_d8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn d9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_d9(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn d10(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_d10(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn d11(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_d11(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline] pub fn d12(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline] pub fn set_d12(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn d13(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline] pub fn set_d13(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline] pub fn d14(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline] pub fn set_d14(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
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
#[derive(PartialEq, Eq)]
pub struct Dcgcdma(pub u32);
impl Dcgcdma {
  #[inline] pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Dcgcepi(pub u32);
impl Dcgcepi {
  #[inline] pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Dcgchib(pub u32);
impl Dcgchib {
  #[inline] pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Dcgcuart(pub u32);
impl Dcgcuart {
  #[inline] pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn d1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_d1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn d2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_d2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn d3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_d3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn d4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_d4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn d5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_d5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn d6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_d6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn d7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_d7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
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
#[derive(PartialEq, Eq)]
pub struct Dcgcssi(pub u32);
impl Dcgcssi {
  #[inline] pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn d1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_d1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn d2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_d2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn d3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_d3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
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
#[derive(PartialEq, Eq)]
pub struct Dcgci2c(pub u32);
impl Dcgci2c {
  #[inline] pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn d1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_d1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn d2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_d2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn d3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_d3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn d4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_d4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn d5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_d5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn d6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_d6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn d7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_d7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn d8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_d8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn d9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_d9(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
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
#[derive(PartialEq, Eq)]
pub struct Dcgcusb(pub u32);
impl Dcgcusb {
  #[inline] pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Dcgcephy(pub u32);
impl Dcgcephy {
  #[inline] pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Dcgccan(pub u32);
impl Dcgccan {
  #[inline] pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn d1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_d1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
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
#[derive(PartialEq, Eq)]
pub struct Dcgcadc(pub u32);
impl Dcgcadc {
  #[inline] pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn d1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_d1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
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
#[derive(PartialEq, Eq)]
pub struct Dcgcacmp(pub u32);
impl Dcgcacmp {
  #[inline] pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Dcgcpwm(pub u32);
impl Dcgcpwm {
  #[inline] pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Dcgcqei(pub u32);
impl Dcgcqei {
  #[inline] pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Dcgceeprom(pub u32);
impl Dcgceeprom {
  #[inline] pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Dcgcccm(pub u32);
impl Dcgcccm {
  #[inline] pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Dcgcemac(pub u32);
impl Dcgcemac {
  #[inline] pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pcwd(pub u32);
impl Pcwd {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_p1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pctimer(pub u32);
impl Pctimer {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_p1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn p2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_p2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn p3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_p3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn p4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_p4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn p5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_p5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn p6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_p6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn p7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_p7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pcgpio(pub u32);
impl Pcgpio {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_p1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn p2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_p2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn p3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_p3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn p4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_p4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn p5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_p5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn p6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_p6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn p7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_p7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn p8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_p8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn p9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_p9(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn p10(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_p10(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn p11(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_p11(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline] pub fn p12(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline] pub fn set_p12(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn p13(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline] pub fn set_p13(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline] pub fn p14(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline] pub fn set_p14(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pcdma(pub u32);
impl Pcdma {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pcepi(pub u32);
impl Pcepi {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pchib(pub u32);
impl Pchib {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pcuart(pub u32);
impl Pcuart {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_p1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn p2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_p2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn p3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_p3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn p4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_p4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn p5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_p5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn p6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_p6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn p7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_p7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pcssi(pub u32);
impl Pcssi {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_p1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn p2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_p2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn p3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_p3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pci2c(pub u32);
impl Pci2c {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_p1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn p2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_p2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn p3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_p3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn p4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_p4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn p5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_p5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn p6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_p6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn p7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_p7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn p8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_p8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn p9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_p9(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pcusb(pub u32);
impl Pcusb {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pcephy(pub u32);
impl Pcephy {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pccan(pub u32);
impl Pccan {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_p1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pcadc(pub u32);
impl Pcadc {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_p1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pcacmp(pub u32);
impl Pcacmp {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pcpwm(pub u32);
impl Pcpwm {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pcqei(pub u32);
impl Pcqei {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pceeprom(pub u32);
impl Pceeprom {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pcccm(pub u32);
impl Pcccm {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pcemac(pub u32);
impl Pcemac {
  #[inline] pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Prwd(pub u32);
impl Prwd {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
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
#[derive(PartialEq, Eq)]
pub struct Prtimer(pub u32);
impl Prtimer {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn r4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_r4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn r5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_r5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn r6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_r6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn r7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_r7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
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
#[derive(PartialEq, Eq)]
pub struct Prgpio(pub u32);
impl Prgpio {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn r4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_r4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn r5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_r5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn r6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_r6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn r7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_r7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn r8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_r8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn r9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_r9(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn r10(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_r10(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn r11(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_r11(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline] pub fn r12(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline] pub fn set_r12(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn r13(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline] pub fn set_r13(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline] pub fn r14(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline] pub fn set_r14(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
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
#[derive(PartialEq, Eq)]
pub struct Prdma(pub u32);
impl Prdma {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Prepi(pub u32);
impl Prepi {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Prhib(pub u32);
impl Prhib {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pruart(pub u32);
impl Pruart {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn r4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_r4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn r5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_r5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn r6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_r6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn r7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_r7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
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
#[derive(PartialEq, Eq)]
pub struct Prssi(pub u32);
impl Prssi {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pri2c(pub u32);
impl Pri2c {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn r4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_r4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn r5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_r5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn r6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_r6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn r7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_r7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn r8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_r8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn r9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_r9(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
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
#[derive(PartialEq, Eq)]
pub struct Prusb(pub u32);
impl Prusb {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Prephy(pub u32);
impl Prephy {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Prcan(pub u32);
impl Prcan {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pradc(pub u32);
impl Pradc {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
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
#[derive(PartialEq, Eq)]
pub struct Pracmp(pub u32);
impl Pracmp {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Prpwm(pub u32);
impl Prpwm {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Prqei(pub u32);
impl Prqei {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Preeprom(pub u32);
impl Preeprom {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Prccm(pub u32);
impl Prccm {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Premac(pub u32);
impl Premac {
  #[inline] pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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

impl Rcgc for super::timer::Timer0 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgctimer().r0() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgctimer(|r| r.set_r0(value)); }
}

impl Rcgc for super::timer::Timer1 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgctimer().r1() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgctimer(|r| r.set_r1(value)); }
}

impl Rcgc for super::timer::Timer2 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgctimer().r2() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgctimer(|r| r.set_r2(value)); }
}

impl Rcgc for super::timer::Timer3 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgctimer().r3() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgctimer(|r| r.set_r3(value)); }
}

impl Rcgc for super::timer::Timer4 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgctimer().r4() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgctimer(|r| r.set_r4(value)); }
}

impl Rcgc for super::timer::Timer5 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgctimer().r5() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgctimer(|r| r.set_r5(value)); }
}

impl Rcgc for super::timer::Timer6 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgctimer().r6() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgctimer(|r| r.set_r6(value)); }
}

impl Rcgc for super::timer::Timer7 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgctimer().r7() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgctimer(|r| r.set_r7(value)); }
}

impl Rcgc for super::gpio::Gpioa {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r0() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r0(value)); }
}

impl Rcgc for super::gpio::Gpiob {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r1() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r1(value)); }
}

impl Rcgc for super::gpio::Gpioc {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r2() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r2(value)); }
}

impl Rcgc for super::gpio::Gpiod {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r3() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r3(value)); }
}

impl Rcgc for super::gpio::Gpioe {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r4() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r4(value)); }
}

impl Rcgc for super::gpio::Gpiof {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r5() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r5(value)); }
}

impl Rcgc for super::gpio::Gpiog {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r6() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r6(value)); }
}

impl Rcgc for super::gpio::Gpioh {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r7() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r7(value)); }
}

impl Rcgc for super::gpio::Gpioj {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r8() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r8(value)); }
}

impl Rcgc for super::gpio::Gpiok {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r9() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r9(value)); }
}

impl Rcgc for super::gpio::Gpiol {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r10() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r10(value)); }
}

impl Rcgc for super::gpio::Gpiom {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r11() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r11(value)); }
}

impl Rcgc for super::gpio::Gpion {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r12() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r12(value)); }
}

impl Rcgc for super::gpio::Gpiop {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r13() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r13(value)); }
}

impl Rcgc for super::gpio::Gpioq {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r14() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r14(value)); }
}

impl Rcgc for super::uart::Uart0 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcuart().r0() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcuart(|r| r.set_r0(value)); }
}

impl Rcgc for super::uart::Uart1 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcuart().r1() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcuart(|r| r.set_r1(value)); }
}

impl Rcgc for super::uart::Uart2 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcuart().r2() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcuart(|r| r.set_r2(value)); }
}

impl Rcgc for super::uart::Uart3 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcuart().r3() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcuart(|r| r.set_r3(value)); }
}

impl Rcgc for super::uart::Uart4 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcuart().r4() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcuart(|r| r.set_r4(value)); }
}

impl Rcgc for super::uart::Uart5 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcuart().r5() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcuart(|r| r.set_r5(value)); }
}

impl Rcgc for super::uart::Uart6 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcuart().r6() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcuart(|r| r.set_r6(value)); }
}

impl Rcgc for super::uart::Uart7 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcuart().r7() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcuart(|r| r.set_r7(value)); }
}

impl Rcgc for super::pwm::Pwm0 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcpwm().r0() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcpwm(|r| r.set_r0(value)); }
}


