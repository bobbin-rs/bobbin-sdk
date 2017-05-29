pub const SYSCTL: Sysctl = Sysctl(0x400fe000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sysctl(pub u32);

impl Sysctl {
  pub unsafe fn did0(&self) -> Did0 { 
     Did0(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
  }
  pub unsafe fn set_did0(&mut self, value: Did0) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
  }
  pub unsafe fn with_did0<F: FnOnce(Did0) -> Did0>(&mut self, f: F) {
     let tmp = self.did0();
     self.set_did0(f(tmp))
  }

  pub unsafe fn did1(&self) -> Did1 { 
     Did1(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
  }
  pub unsafe fn set_did1(&mut self, value: Did1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
  }
  pub unsafe fn with_did1<F: FnOnce(Did1) -> Did1>(&mut self, f: F) {
     let tmp = self.did1();
     self.set_did1(f(tmp))
  }

  pub unsafe fn ptboctl(&self) -> Ptboctl { 
     Ptboctl(::core::ptr::read_volatile(((self.0 as usize) + 0x38) as *const u32))
  }
  pub unsafe fn set_ptboctl(&mut self, value: Ptboctl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x38) as *mut u32, value.0);
  }
  pub unsafe fn with_ptboctl<F: FnOnce(Ptboctl) -> Ptboctl>(&mut self, f: F) {
     let tmp = self.ptboctl();
     self.set_ptboctl(f(tmp))
  }

  pub unsafe fn ris(&self) -> Ris { 
     Ris(::core::ptr::read_volatile(((self.0 as usize) + 0x50) as *const u32))
  }
  pub unsafe fn set_ris(&mut self, value: Ris) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x50) as *mut u32, value.0);
  }
  pub unsafe fn with_ris<F: FnOnce(Ris) -> Ris>(&mut self, f: F) {
     let tmp = self.ris();
     self.set_ris(f(tmp))
  }

  pub unsafe fn imc(&self) -> Imc { 
     Imc(::core::ptr::read_volatile(((self.0 as usize) + 0x54) as *const u32))
  }
  pub unsafe fn set_imc(&mut self, value: Imc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x54) as *mut u32, value.0);
  }
  pub unsafe fn with_imc<F: FnOnce(Imc) -> Imc>(&mut self, f: F) {
     let tmp = self.imc();
     self.set_imc(f(tmp))
  }

  pub unsafe fn misc(&self) -> Misc { 
     Misc(::core::ptr::read_volatile(((self.0 as usize) + 0x58) as *const u32))
  }
  pub unsafe fn set_misc(&mut self, value: Misc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x58) as *mut u32, value.0);
  }
  pub unsafe fn with_misc<F: FnOnce(Misc) -> Misc>(&mut self, f: F) {
     let tmp = self.misc();
     self.set_misc(f(tmp))
  }

  pub unsafe fn resc(&self) -> Resc { 
     Resc(::core::ptr::read_volatile(((self.0 as usize) + 0x5c) as *const u32))
  }
  pub unsafe fn set_resc(&mut self, value: Resc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x5c) as *mut u32, value.0);
  }
  pub unsafe fn with_resc<F: FnOnce(Resc) -> Resc>(&mut self, f: F) {
     let tmp = self.resc();
     self.set_resc(f(tmp))
  }

  pub unsafe fn pwrtc(&self) -> Pwrtc { 
     Pwrtc(::core::ptr::read_volatile(((self.0 as usize) + 0x60) as *const u32))
  }
  pub unsafe fn set_pwrtc(&mut self, value: Pwrtc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x60) as *mut u32, value.0);
  }
  pub unsafe fn with_pwrtc<F: FnOnce(Pwrtc) -> Pwrtc>(&mut self, f: F) {
     let tmp = self.pwrtc();
     self.set_pwrtc(f(tmp))
  }

  pub unsafe fn nmic(&self) -> Nmic { 
     Nmic(::core::ptr::read_volatile(((self.0 as usize) + 0x64) as *const u32))
  }
  pub unsafe fn set_nmic(&mut self, value: Nmic) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x64) as *mut u32, value.0);
  }
  pub unsafe fn with_nmic<F: FnOnce(Nmic) -> Nmic>(&mut self, f: F) {
     let tmp = self.nmic();
     self.set_nmic(f(tmp))
  }

  pub unsafe fn moscctl(&self) -> Moscctl { 
     Moscctl(::core::ptr::read_volatile(((self.0 as usize) + 0x7c) as *const u32))
  }
  pub unsafe fn set_moscctl(&mut self, value: Moscctl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x7c) as *mut u32, value.0);
  }
  pub unsafe fn with_moscctl<F: FnOnce(Moscctl) -> Moscctl>(&mut self, f: F) {
     let tmp = self.moscctl();
     self.set_moscctl(f(tmp))
  }

  pub unsafe fn rsclkcfg(&self) -> Rsclkcfg { 
     Rsclkcfg(::core::ptr::read_volatile(((self.0 as usize) + 0xb0) as *const u32))
  }
  pub unsafe fn set_rsclkcfg(&mut self, value: Rsclkcfg) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xb0) as *mut u32, value.0);
  }
  pub unsafe fn with_rsclkcfg<F: FnOnce(Rsclkcfg) -> Rsclkcfg>(&mut self, f: F) {
     let tmp = self.rsclkcfg();
     self.set_rsclkcfg(f(tmp))
  }

  pub unsafe fn memtim0(&self) -> Memtim0 { 
     Memtim0(::core::ptr::read_volatile(((self.0 as usize) + 0xc0) as *const u32))
  }
  pub unsafe fn set_memtim0(&mut self, value: Memtim0) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc0) as *mut u32, value.0);
  }
  pub unsafe fn with_memtim0<F: FnOnce(Memtim0) -> Memtim0>(&mut self, f: F) {
     let tmp = self.memtim0();
     self.set_memtim0(f(tmp))
  }

  pub unsafe fn altclkcfg(&self) -> Altclkcfg { 
     Altclkcfg(::core::ptr::read_volatile(((self.0 as usize) + 0x138) as *const u32))
  }
  pub unsafe fn set_altclkcfg(&mut self, value: Altclkcfg) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x138) as *mut u32, value.0);
  }
  pub unsafe fn with_altclkcfg<F: FnOnce(Altclkcfg) -> Altclkcfg>(&mut self, f: F) {
     let tmp = self.altclkcfg();
     self.set_altclkcfg(f(tmp))
  }

  pub unsafe fn dsclkcfg(&self) -> Dsclkcfg { 
     Dsclkcfg(::core::ptr::read_volatile(((self.0 as usize) + 0x144) as *const u32))
  }
  pub unsafe fn set_dsclkcfg(&mut self, value: Dsclkcfg) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x144) as *mut u32, value.0);
  }
  pub unsafe fn with_dsclkcfg<F: FnOnce(Dsclkcfg) -> Dsclkcfg>(&mut self, f: F) {
     let tmp = self.dsclkcfg();
     self.set_dsclkcfg(f(tmp))
  }

  pub unsafe fn divsclk(&self) -> Divsclk { 
     Divsclk(::core::ptr::read_volatile(((self.0 as usize) + 0x148) as *const u32))
  }
  pub unsafe fn set_divsclk(&mut self, value: Divsclk) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x148) as *mut u32, value.0);
  }
  pub unsafe fn with_divsclk<F: FnOnce(Divsclk) -> Divsclk>(&mut self, f: F) {
     let tmp = self.divsclk();
     self.set_divsclk(f(tmp))
  }

  pub unsafe fn sysprop(&self) -> Sysprop { 
     Sysprop(::core::ptr::read_volatile(((self.0 as usize) + 0x14c) as *const u32))
  }
  pub unsafe fn set_sysprop(&mut self, value: Sysprop) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x14c) as *mut u32, value.0);
  }
  pub unsafe fn with_sysprop<F: FnOnce(Sysprop) -> Sysprop>(&mut self, f: F) {
     let tmp = self.sysprop();
     self.set_sysprop(f(tmp))
  }

  pub unsafe fn piosccal(&self) -> Piosccal { 
     Piosccal(::core::ptr::read_volatile(((self.0 as usize) + 0x150) as *const u32))
  }
  pub unsafe fn set_piosccal(&mut self, value: Piosccal) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x150) as *mut u32, value.0);
  }
  pub unsafe fn with_piosccal<F: FnOnce(Piosccal) -> Piosccal>(&mut self, f: F) {
     let tmp = self.piosccal();
     self.set_piosccal(f(tmp))
  }

  pub unsafe fn pioscstat(&self) -> Pioscstat { 
     Pioscstat(::core::ptr::read_volatile(((self.0 as usize) + 0x154) as *const u32))
  }
  pub unsafe fn set_pioscstat(&mut self, value: Pioscstat) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x154) as *mut u32, value.0);
  }
  pub unsafe fn with_pioscstat<F: FnOnce(Pioscstat) -> Pioscstat>(&mut self, f: F) {
     let tmp = self.pioscstat();
     self.set_pioscstat(f(tmp))
  }

  pub unsafe fn pllfreq0(&self) -> Pllfreq0 { 
     Pllfreq0(::core::ptr::read_volatile(((self.0 as usize) + 0x160) as *const u32))
  }
  pub unsafe fn set_pllfreq0(&mut self, value: Pllfreq0) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x160) as *mut u32, value.0);
  }
  pub unsafe fn with_pllfreq0<F: FnOnce(Pllfreq0) -> Pllfreq0>(&mut self, f: F) {
     let tmp = self.pllfreq0();
     self.set_pllfreq0(f(tmp))
  }

  pub unsafe fn pllfreq1(&self) -> Pllfreq1 { 
     Pllfreq1(::core::ptr::read_volatile(((self.0 as usize) + 0x164) as *const u32))
  }
  pub unsafe fn set_pllfreq1(&mut self, value: Pllfreq1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x164) as *mut u32, value.0);
  }
  pub unsafe fn with_pllfreq1<F: FnOnce(Pllfreq1) -> Pllfreq1>(&mut self, f: F) {
     let tmp = self.pllfreq1();
     self.set_pllfreq1(f(tmp))
  }

  pub unsafe fn pllstat(&self) -> Pllstat { 
     Pllstat(::core::ptr::read_volatile(((self.0 as usize) + 0x168) as *const u32))
  }
  pub unsafe fn set_pllstat(&mut self, value: Pllstat) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x168) as *mut u32, value.0);
  }
  pub unsafe fn with_pllstat<F: FnOnce(Pllstat) -> Pllstat>(&mut self, f: F) {
     let tmp = self.pllstat();
     self.set_pllstat(f(tmp))
  }

  pub unsafe fn slppwrcfg(&self) -> Slppwrcfg { 
     Slppwrcfg(::core::ptr::read_volatile(((self.0 as usize) + 0x188) as *const u32))
  }
  pub unsafe fn set_slppwrcfg(&mut self, value: Slppwrcfg) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x188) as *mut u32, value.0);
  }
  pub unsafe fn with_slppwrcfg<F: FnOnce(Slppwrcfg) -> Slppwrcfg>(&mut self, f: F) {
     let tmp = self.slppwrcfg();
     self.set_slppwrcfg(f(tmp))
  }

  pub unsafe fn dslppwrcfg(&self) -> Dslppwrcfg { 
     Dslppwrcfg(::core::ptr::read_volatile(((self.0 as usize) + 0x18c) as *const u32))
  }
  pub unsafe fn set_dslppwrcfg(&mut self, value: Dslppwrcfg) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x18c) as *mut u32, value.0);
  }
  pub unsafe fn with_dslppwrcfg<F: FnOnce(Dslppwrcfg) -> Dslppwrcfg>(&mut self, f: F) {
     let tmp = self.dslppwrcfg();
     self.set_dslppwrcfg(f(tmp))
  }

  pub unsafe fn nvmstat(&self) -> Nvmstat { 
     Nvmstat(::core::ptr::read_volatile(((self.0 as usize) + 0x1a0) as *const u32))
  }
  pub unsafe fn set_nvmstat(&mut self, value: Nvmstat) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1a0) as *mut u32, value.0);
  }
  pub unsafe fn with_nvmstat<F: FnOnce(Nvmstat) -> Nvmstat>(&mut self, f: F) {
     let tmp = self.nvmstat();
     self.set_nvmstat(f(tmp))
  }

  pub unsafe fn ldospctl(&self) -> Ldospctl { 
     Ldospctl(::core::ptr::read_volatile(((self.0 as usize) + 0x1b4) as *const u32))
  }
  pub unsafe fn set_ldospctl(&mut self, value: Ldospctl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1b4) as *mut u32, value.0);
  }
  pub unsafe fn with_ldospctl<F: FnOnce(Ldospctl) -> Ldospctl>(&mut self, f: F) {
     let tmp = self.ldospctl();
     self.set_ldospctl(f(tmp))
  }

  pub unsafe fn ldodpctl(&self) -> Ldodpctl { 
     Ldodpctl(::core::ptr::read_volatile(((self.0 as usize) + 0x1bc) as *const u32))
  }
  pub unsafe fn set_ldodpctl(&mut self, value: Ldodpctl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1bc) as *mut u32, value.0);
  }
  pub unsafe fn with_ldodpctl<F: FnOnce(Ldodpctl) -> Ldodpctl>(&mut self, f: F) {
     let tmp = self.ldodpctl();
     self.set_ldodpctl(f(tmp))
  }

  pub unsafe fn resbehavctl(&self) -> Resbehavctl { 
     Resbehavctl(::core::ptr::read_volatile(((self.0 as usize) + 0x1d8) as *const u32))
  }
  pub unsafe fn set_resbehavctl(&mut self, value: Resbehavctl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1d8) as *mut u32, value.0);
  }
  pub unsafe fn with_resbehavctl<F: FnOnce(Resbehavctl) -> Resbehavctl>(&mut self, f: F) {
     let tmp = self.resbehavctl();
     self.set_resbehavctl(f(tmp))
  }

  pub unsafe fn hssr(&self) -> Hssr { 
     Hssr(::core::ptr::read_volatile(((self.0 as usize) + 0x1f4) as *const u32))
  }
  pub unsafe fn set_hssr(&mut self, value: Hssr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1f4) as *mut u32, value.0);
  }
  pub unsafe fn with_hssr<F: FnOnce(Hssr) -> Hssr>(&mut self, f: F) {
     let tmp = self.hssr();
     self.set_hssr(f(tmp))
  }

  pub unsafe fn usbpds(&self) -> Usbpds { 
     Usbpds(::core::ptr::read_volatile(((self.0 as usize) + 0x280) as *const u32))
  }
  pub unsafe fn set_usbpds(&mut self, value: Usbpds) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x280) as *mut u32, value.0);
  }
  pub unsafe fn with_usbpds<F: FnOnce(Usbpds) -> Usbpds>(&mut self, f: F) {
     let tmp = self.usbpds();
     self.set_usbpds(f(tmp))
  }

  pub unsafe fn usbmpc(&self) -> Usbmpc { 
     Usbmpc(::core::ptr::read_volatile(((self.0 as usize) + 0x284) as *const u32))
  }
  pub unsafe fn set_usbmpc(&mut self, value: Usbmpc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x284) as *mut u32, value.0);
  }
  pub unsafe fn with_usbmpc<F: FnOnce(Usbmpc) -> Usbmpc>(&mut self, f: F) {
     let tmp = self.usbmpc();
     self.set_usbmpc(f(tmp))
  }

  pub unsafe fn emacpds(&self) -> Emacpds { 
     Emacpds(::core::ptr::read_volatile(((self.0 as usize) + 0x288) as *const u32))
  }
  pub unsafe fn set_emacpds(&mut self, value: Emacpds) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x288) as *mut u32, value.0);
  }
  pub unsafe fn with_emacpds<F: FnOnce(Emacpds) -> Emacpds>(&mut self, f: F) {
     let tmp = self.emacpds();
     self.set_emacpds(f(tmp))
  }

  pub unsafe fn emacmpc(&self) -> Emacmpc { 
     Emacmpc(::core::ptr::read_volatile(((self.0 as usize) + 0x28c) as *const u32))
  }
  pub unsafe fn set_emacmpc(&mut self, value: Emacmpc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x28c) as *mut u32, value.0);
  }
  pub unsafe fn with_emacmpc<F: FnOnce(Emacmpc) -> Emacmpc>(&mut self, f: F) {
     let tmp = self.emacmpc();
     self.set_emacmpc(f(tmp))
  }

  pub unsafe fn ppwd(&self) -> Ppwd { 
     Ppwd(::core::ptr::read_volatile(((self.0 as usize) + 0x300) as *const u32))
  }
  pub unsafe fn set_ppwd(&mut self, value: Ppwd) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x300) as *mut u32, value.0);
  }
  pub unsafe fn with_ppwd<F: FnOnce(Ppwd) -> Ppwd>(&mut self, f: F) {
     let tmp = self.ppwd();
     self.set_ppwd(f(tmp))
  }

  pub unsafe fn pptimer(&self) -> Pptimer { 
     Pptimer(::core::ptr::read_volatile(((self.0 as usize) + 0x304) as *const u32))
  }
  pub unsafe fn set_pptimer(&mut self, value: Pptimer) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x304) as *mut u32, value.0);
  }
  pub unsafe fn with_pptimer<F: FnOnce(Pptimer) -> Pptimer>(&mut self, f: F) {
     let tmp = self.pptimer();
     self.set_pptimer(f(tmp))
  }

  pub unsafe fn ppgpio(&self) -> Ppgpio { 
     Ppgpio(::core::ptr::read_volatile(((self.0 as usize) + 0x308) as *const u32))
  }
  pub unsafe fn set_ppgpio(&mut self, value: Ppgpio) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x308) as *mut u32, value.0);
  }
  pub unsafe fn with_ppgpio<F: FnOnce(Ppgpio) -> Ppgpio>(&mut self, f: F) {
     let tmp = self.ppgpio();
     self.set_ppgpio(f(tmp))
  }

  pub unsafe fn ppdma(&self) -> Ppdma { 
     Ppdma(::core::ptr::read_volatile(((self.0 as usize) + 0x30c) as *const u32))
  }
  pub unsafe fn set_ppdma(&mut self, value: Ppdma) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x30c) as *mut u32, value.0);
  }
  pub unsafe fn with_ppdma<F: FnOnce(Ppdma) -> Ppdma>(&mut self, f: F) {
     let tmp = self.ppdma();
     self.set_ppdma(f(tmp))
  }

  pub unsafe fn ppepi(&self) -> Ppepi { 
     Ppepi(::core::ptr::read_volatile(((self.0 as usize) + 0x310) as *const u32))
  }
  pub unsafe fn set_ppepi(&mut self, value: Ppepi) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x310) as *mut u32, value.0);
  }
  pub unsafe fn with_ppepi<F: FnOnce(Ppepi) -> Ppepi>(&mut self, f: F) {
     let tmp = self.ppepi();
     self.set_ppepi(f(tmp))
  }

  pub unsafe fn pphib(&self) -> Pphib { 
     Pphib(::core::ptr::read_volatile(((self.0 as usize) + 0x314) as *const u32))
  }
  pub unsafe fn set_pphib(&mut self, value: Pphib) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x314) as *mut u32, value.0);
  }
  pub unsafe fn with_pphib<F: FnOnce(Pphib) -> Pphib>(&mut self, f: F) {
     let tmp = self.pphib();
     self.set_pphib(f(tmp))
  }

  pub unsafe fn ppuart(&self) -> Ppuart { 
     Ppuart(::core::ptr::read_volatile(((self.0 as usize) + 0x318) as *const u32))
  }
  pub unsafe fn set_ppuart(&mut self, value: Ppuart) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x318) as *mut u32, value.0);
  }
  pub unsafe fn with_ppuart<F: FnOnce(Ppuart) -> Ppuart>(&mut self, f: F) {
     let tmp = self.ppuart();
     self.set_ppuart(f(tmp))
  }

  pub unsafe fn ppssi(&self) -> Ppssi { 
     Ppssi(::core::ptr::read_volatile(((self.0 as usize) + 0x31c) as *const u32))
  }
  pub unsafe fn set_ppssi(&mut self, value: Ppssi) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x31c) as *mut u32, value.0);
  }
  pub unsafe fn with_ppssi<F: FnOnce(Ppssi) -> Ppssi>(&mut self, f: F) {
     let tmp = self.ppssi();
     self.set_ppssi(f(tmp))
  }

  pub unsafe fn ppi2c(&self) -> Ppi2c { 
     Ppi2c(::core::ptr::read_volatile(((self.0 as usize) + 0x320) as *const u32))
  }
  pub unsafe fn set_ppi2c(&mut self, value: Ppi2c) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x320) as *mut u32, value.0);
  }
  pub unsafe fn with_ppi2c<F: FnOnce(Ppi2c) -> Ppi2c>(&mut self, f: F) {
     let tmp = self.ppi2c();
     self.set_ppi2c(f(tmp))
  }

  pub unsafe fn ppusb(&self) -> Ppusb { 
     Ppusb(::core::ptr::read_volatile(((self.0 as usize) + 0x328) as *const u32))
  }
  pub unsafe fn set_ppusb(&mut self, value: Ppusb) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x328) as *mut u32, value.0);
  }
  pub unsafe fn with_ppusb<F: FnOnce(Ppusb) -> Ppusb>(&mut self, f: F) {
     let tmp = self.ppusb();
     self.set_ppusb(f(tmp))
  }

  pub unsafe fn ppephy(&self) -> Ppephy { 
     Ppephy(::core::ptr::read_volatile(((self.0 as usize) + 0x330) as *const u32))
  }
  pub unsafe fn set_ppephy(&mut self, value: Ppephy) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x330) as *mut u32, value.0);
  }
  pub unsafe fn with_ppephy<F: FnOnce(Ppephy) -> Ppephy>(&mut self, f: F) {
     let tmp = self.ppephy();
     self.set_ppephy(f(tmp))
  }

  pub unsafe fn ppcan(&self) -> Ppcan { 
     Ppcan(::core::ptr::read_volatile(((self.0 as usize) + 0x334) as *const u32))
  }
  pub unsafe fn set_ppcan(&mut self, value: Ppcan) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x334) as *mut u32, value.0);
  }
  pub unsafe fn with_ppcan<F: FnOnce(Ppcan) -> Ppcan>(&mut self, f: F) {
     let tmp = self.ppcan();
     self.set_ppcan(f(tmp))
  }

  pub unsafe fn ppadc(&self) -> Ppadc { 
     Ppadc(::core::ptr::read_volatile(((self.0 as usize) + 0x338) as *const u32))
  }
  pub unsafe fn set_ppadc(&mut self, value: Ppadc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x338) as *mut u32, value.0);
  }
  pub unsafe fn with_ppadc<F: FnOnce(Ppadc) -> Ppadc>(&mut self, f: F) {
     let tmp = self.ppadc();
     self.set_ppadc(f(tmp))
  }

  pub unsafe fn ppacmp(&self) -> Ppacmp { 
     Ppacmp(::core::ptr::read_volatile(((self.0 as usize) + 0x33c) as *const u32))
  }
  pub unsafe fn set_ppacmp(&mut self, value: Ppacmp) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x33c) as *mut u32, value.0);
  }
  pub unsafe fn with_ppacmp<F: FnOnce(Ppacmp) -> Ppacmp>(&mut self, f: F) {
     let tmp = self.ppacmp();
     self.set_ppacmp(f(tmp))
  }

  pub unsafe fn pppwm(&self) -> Pppwm { 
     Pppwm(::core::ptr::read_volatile(((self.0 as usize) + 0x340) as *const u32))
  }
  pub unsafe fn set_pppwm(&mut self, value: Pppwm) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x340) as *mut u32, value.0);
  }
  pub unsafe fn with_pppwm<F: FnOnce(Pppwm) -> Pppwm>(&mut self, f: F) {
     let tmp = self.pppwm();
     self.set_pppwm(f(tmp))
  }

  pub unsafe fn ppqei(&self) -> Ppqei { 
     Ppqei(::core::ptr::read_volatile(((self.0 as usize) + 0x344) as *const u32))
  }
  pub unsafe fn set_ppqei(&mut self, value: Ppqei) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x344) as *mut u32, value.0);
  }
  pub unsafe fn with_ppqei<F: FnOnce(Ppqei) -> Ppqei>(&mut self, f: F) {
     let tmp = self.ppqei();
     self.set_ppqei(f(tmp))
  }

  pub unsafe fn pplpc(&self) -> Pplpc { 
     Pplpc(::core::ptr::read_volatile(((self.0 as usize) + 0x348) as *const u32))
  }
  pub unsafe fn set_pplpc(&mut self, value: Pplpc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x348) as *mut u32, value.0);
  }
  pub unsafe fn with_pplpc<F: FnOnce(Pplpc) -> Pplpc>(&mut self, f: F) {
     let tmp = self.pplpc();
     self.set_pplpc(f(tmp))
  }

  pub unsafe fn pppeci(&self) -> Pppeci { 
     Pppeci(::core::ptr::read_volatile(((self.0 as usize) + 0x350) as *const u32))
  }
  pub unsafe fn set_pppeci(&mut self, value: Pppeci) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x350) as *mut u32, value.0);
  }
  pub unsafe fn with_pppeci<F: FnOnce(Pppeci) -> Pppeci>(&mut self, f: F) {
     let tmp = self.pppeci();
     self.set_pppeci(f(tmp))
  }

  pub unsafe fn ppfan(&self) -> Ppfan { 
     Ppfan(::core::ptr::read_volatile(((self.0 as usize) + 0x354) as *const u32))
  }
  pub unsafe fn set_ppfan(&mut self, value: Ppfan) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x354) as *mut u32, value.0);
  }
  pub unsafe fn with_ppfan<F: FnOnce(Ppfan) -> Ppfan>(&mut self, f: F) {
     let tmp = self.ppfan();
     self.set_ppfan(f(tmp))
  }

  pub unsafe fn ppeeprom(&self) -> Ppeeprom { 
     Ppeeprom(::core::ptr::read_volatile(((self.0 as usize) + 0x358) as *const u32))
  }
  pub unsafe fn set_ppeeprom(&mut self, value: Ppeeprom) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x358) as *mut u32, value.0);
  }
  pub unsafe fn with_ppeeprom<F: FnOnce(Ppeeprom) -> Ppeeprom>(&mut self, f: F) {
     let tmp = self.ppeeprom();
     self.set_ppeeprom(f(tmp))
  }

  pub unsafe fn ppwtimer(&self) -> Ppwtimer { 
     Ppwtimer(::core::ptr::read_volatile(((self.0 as usize) + 0x35c) as *const u32))
  }
  pub unsafe fn set_ppwtimer(&mut self, value: Ppwtimer) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x35c) as *mut u32, value.0);
  }
  pub unsafe fn with_ppwtimer<F: FnOnce(Ppwtimer) -> Ppwtimer>(&mut self, f: F) {
     let tmp = self.ppwtimer();
     self.set_ppwtimer(f(tmp))
  }

  pub unsafe fn pprts(&self) -> Pprts { 
     Pprts(::core::ptr::read_volatile(((self.0 as usize) + 0x370) as *const u32))
  }
  pub unsafe fn set_pprts(&mut self, value: Pprts) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x370) as *mut u32, value.0);
  }
  pub unsafe fn with_pprts<F: FnOnce(Pprts) -> Pprts>(&mut self, f: F) {
     let tmp = self.pprts();
     self.set_pprts(f(tmp))
  }

  pub unsafe fn ppccm(&self) -> Ppccm { 
     Ppccm(::core::ptr::read_volatile(((self.0 as usize) + 0x374) as *const u32))
  }
  pub unsafe fn set_ppccm(&mut self, value: Ppccm) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x374) as *mut u32, value.0);
  }
  pub unsafe fn with_ppccm<F: FnOnce(Ppccm) -> Ppccm>(&mut self, f: F) {
     let tmp = self.ppccm();
     self.set_ppccm(f(tmp))
  }

  pub unsafe fn pplcd(&self) -> Pplcd { 
     Pplcd(::core::ptr::read_volatile(((self.0 as usize) + 0x390) as *const u32))
  }
  pub unsafe fn set_pplcd(&mut self, value: Pplcd) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x390) as *mut u32, value.0);
  }
  pub unsafe fn with_pplcd<F: FnOnce(Pplcd) -> Pplcd>(&mut self, f: F) {
     let tmp = self.pplcd();
     self.set_pplcd(f(tmp))
  }

  pub unsafe fn ppowire(&self) -> Ppowire { 
     Ppowire(::core::ptr::read_volatile(((self.0 as usize) + 0x398) as *const u32))
  }
  pub unsafe fn set_ppowire(&mut self, value: Ppowire) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x398) as *mut u32, value.0);
  }
  pub unsafe fn with_ppowire<F: FnOnce(Ppowire) -> Ppowire>(&mut self, f: F) {
     let tmp = self.ppowire();
     self.set_ppowire(f(tmp))
  }

  pub unsafe fn ppemac(&self) -> Ppemac { 
     Ppemac(::core::ptr::read_volatile(((self.0 as usize) + 0x39c) as *const u32))
  }
  pub unsafe fn set_ppemac(&mut self, value: Ppemac) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x39c) as *mut u32, value.0);
  }
  pub unsafe fn with_ppemac<F: FnOnce(Ppemac) -> Ppemac>(&mut self, f: F) {
     let tmp = self.ppemac();
     self.set_ppemac(f(tmp))
  }

  pub unsafe fn pphim(&self) -> Pphim { 
     Pphim(::core::ptr::read_volatile(((self.0 as usize) + 0x3a4) as *const u32))
  }
  pub unsafe fn set_pphim(&mut self, value: Pphim) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x3a4) as *mut u32, value.0);
  }
  pub unsafe fn with_pphim<F: FnOnce(Pphim) -> Pphim>(&mut self, f: F) {
     let tmp = self.pphim();
     self.set_pphim(f(tmp))
  }

  pub unsafe fn srwd(&self) -> Srwd { 
     Srwd(::core::ptr::read_volatile(((self.0 as usize) + 0x500) as *const u32))
  }
  pub unsafe fn set_srwd(&mut self, value: Srwd) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x500) as *mut u32, value.0);
  }
  pub unsafe fn with_srwd<F: FnOnce(Srwd) -> Srwd>(&mut self, f: F) {
     let tmp = self.srwd();
     self.set_srwd(f(tmp))
  }

  pub unsafe fn srtimer(&self) -> Srtimer { 
     Srtimer(::core::ptr::read_volatile(((self.0 as usize) + 0x504) as *const u32))
  }
  pub unsafe fn set_srtimer(&mut self, value: Srtimer) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x504) as *mut u32, value.0);
  }
  pub unsafe fn with_srtimer<F: FnOnce(Srtimer) -> Srtimer>(&mut self, f: F) {
     let tmp = self.srtimer();
     self.set_srtimer(f(tmp))
  }

  pub unsafe fn srgpio(&self) -> Srgpio { 
     Srgpio(::core::ptr::read_volatile(((self.0 as usize) + 0x508) as *const u32))
  }
  pub unsafe fn set_srgpio(&mut self, value: Srgpio) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x508) as *mut u32, value.0);
  }
  pub unsafe fn with_srgpio<F: FnOnce(Srgpio) -> Srgpio>(&mut self, f: F) {
     let tmp = self.srgpio();
     self.set_srgpio(f(tmp))
  }

  pub unsafe fn srdma(&self) -> Srdma { 
     Srdma(::core::ptr::read_volatile(((self.0 as usize) + 0x50c) as *const u32))
  }
  pub unsafe fn set_srdma(&mut self, value: Srdma) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x50c) as *mut u32, value.0);
  }
  pub unsafe fn with_srdma<F: FnOnce(Srdma) -> Srdma>(&mut self, f: F) {
     let tmp = self.srdma();
     self.set_srdma(f(tmp))
  }

  pub unsafe fn srepi(&self) -> Srepi { 
     Srepi(::core::ptr::read_volatile(((self.0 as usize) + 0x510) as *const u32))
  }
  pub unsafe fn set_srepi(&mut self, value: Srepi) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x510) as *mut u32, value.0);
  }
  pub unsafe fn with_srepi<F: FnOnce(Srepi) -> Srepi>(&mut self, f: F) {
     let tmp = self.srepi();
     self.set_srepi(f(tmp))
  }

  pub unsafe fn srhib(&self) -> Srhib { 
     Srhib(::core::ptr::read_volatile(((self.0 as usize) + 0x514) as *const u32))
  }
  pub unsafe fn set_srhib(&mut self, value: Srhib) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x514) as *mut u32, value.0);
  }
  pub unsafe fn with_srhib<F: FnOnce(Srhib) -> Srhib>(&mut self, f: F) {
     let tmp = self.srhib();
     self.set_srhib(f(tmp))
  }

  pub unsafe fn sruart(&self) -> Sruart { 
     Sruart(::core::ptr::read_volatile(((self.0 as usize) + 0x518) as *const u32))
  }
  pub unsafe fn set_sruart(&mut self, value: Sruart) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x518) as *mut u32, value.0);
  }
  pub unsafe fn with_sruart<F: FnOnce(Sruart) -> Sruart>(&mut self, f: F) {
     let tmp = self.sruart();
     self.set_sruart(f(tmp))
  }

  pub unsafe fn srssi(&self) -> Srssi { 
     Srssi(::core::ptr::read_volatile(((self.0 as usize) + 0x51c) as *const u32))
  }
  pub unsafe fn set_srssi(&mut self, value: Srssi) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x51c) as *mut u32, value.0);
  }
  pub unsafe fn with_srssi<F: FnOnce(Srssi) -> Srssi>(&mut self, f: F) {
     let tmp = self.srssi();
     self.set_srssi(f(tmp))
  }

  pub unsafe fn sri2c(&self) -> Sri2c { 
     Sri2c(::core::ptr::read_volatile(((self.0 as usize) + 0x520) as *const u32))
  }
  pub unsafe fn set_sri2c(&mut self, value: Sri2c) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x520) as *mut u32, value.0);
  }
  pub unsafe fn with_sri2c<F: FnOnce(Sri2c) -> Sri2c>(&mut self, f: F) {
     let tmp = self.sri2c();
     self.set_sri2c(f(tmp))
  }

  pub unsafe fn srusb(&self) -> Srusb { 
     Srusb(::core::ptr::read_volatile(((self.0 as usize) + 0x528) as *const u32))
  }
  pub unsafe fn set_srusb(&mut self, value: Srusb) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x528) as *mut u32, value.0);
  }
  pub unsafe fn with_srusb<F: FnOnce(Srusb) -> Srusb>(&mut self, f: F) {
     let tmp = self.srusb();
     self.set_srusb(f(tmp))
  }

  pub unsafe fn srephy(&self) -> Srephy { 
     Srephy(::core::ptr::read_volatile(((self.0 as usize) + 0x530) as *const u32))
  }
  pub unsafe fn set_srephy(&mut self, value: Srephy) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x530) as *mut u32, value.0);
  }
  pub unsafe fn with_srephy<F: FnOnce(Srephy) -> Srephy>(&mut self, f: F) {
     let tmp = self.srephy();
     self.set_srephy(f(tmp))
  }

  pub unsafe fn srcan(&self) -> Srcan { 
     Srcan(::core::ptr::read_volatile(((self.0 as usize) + 0x534) as *const u32))
  }
  pub unsafe fn set_srcan(&mut self, value: Srcan) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x534) as *mut u32, value.0);
  }
  pub unsafe fn with_srcan<F: FnOnce(Srcan) -> Srcan>(&mut self, f: F) {
     let tmp = self.srcan();
     self.set_srcan(f(tmp))
  }

  pub unsafe fn sradc(&self) -> Sradc { 
     Sradc(::core::ptr::read_volatile(((self.0 as usize) + 0x538) as *const u32))
  }
  pub unsafe fn set_sradc(&mut self, value: Sradc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x538) as *mut u32, value.0);
  }
  pub unsafe fn with_sradc<F: FnOnce(Sradc) -> Sradc>(&mut self, f: F) {
     let tmp = self.sradc();
     self.set_sradc(f(tmp))
  }

  pub unsafe fn sracmp(&self) -> Sracmp { 
     Sracmp(::core::ptr::read_volatile(((self.0 as usize) + 0x53c) as *const u32))
  }
  pub unsafe fn set_sracmp(&mut self, value: Sracmp) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x53c) as *mut u32, value.0);
  }
  pub unsafe fn with_sracmp<F: FnOnce(Sracmp) -> Sracmp>(&mut self, f: F) {
     let tmp = self.sracmp();
     self.set_sracmp(f(tmp))
  }

  pub unsafe fn srpwm(&self) -> Srpwm { 
     Srpwm(::core::ptr::read_volatile(((self.0 as usize) + 0x540) as *const u32))
  }
  pub unsafe fn set_srpwm(&mut self, value: Srpwm) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x540) as *mut u32, value.0);
  }
  pub unsafe fn with_srpwm<F: FnOnce(Srpwm) -> Srpwm>(&mut self, f: F) {
     let tmp = self.srpwm();
     self.set_srpwm(f(tmp))
  }

  pub unsafe fn srqei(&self) -> Srqei { 
     Srqei(::core::ptr::read_volatile(((self.0 as usize) + 0x544) as *const u32))
  }
  pub unsafe fn set_srqei(&mut self, value: Srqei) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x544) as *mut u32, value.0);
  }
  pub unsafe fn with_srqei<F: FnOnce(Srqei) -> Srqei>(&mut self, f: F) {
     let tmp = self.srqei();
     self.set_srqei(f(tmp))
  }

  pub unsafe fn sreeprom(&self) -> Sreeprom { 
     Sreeprom(::core::ptr::read_volatile(((self.0 as usize) + 0x558) as *const u32))
  }
  pub unsafe fn set_sreeprom(&mut self, value: Sreeprom) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x558) as *mut u32, value.0);
  }
  pub unsafe fn with_sreeprom<F: FnOnce(Sreeprom) -> Sreeprom>(&mut self, f: F) {
     let tmp = self.sreeprom();
     self.set_sreeprom(f(tmp))
  }

  pub unsafe fn srccm(&self) -> Srccm { 
     Srccm(::core::ptr::read_volatile(((self.0 as usize) + 0x574) as *const u32))
  }
  pub unsafe fn set_srccm(&mut self, value: Srccm) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x574) as *mut u32, value.0);
  }
  pub unsafe fn with_srccm<F: FnOnce(Srccm) -> Srccm>(&mut self, f: F) {
     let tmp = self.srccm();
     self.set_srccm(f(tmp))
  }

  pub unsafe fn sremac(&self) -> Sremac { 
     Sremac(::core::ptr::read_volatile(((self.0 as usize) + 0x59c) as *const u32))
  }
  pub unsafe fn set_sremac(&mut self, value: Sremac) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x59c) as *mut u32, value.0);
  }
  pub unsafe fn with_sremac<F: FnOnce(Sremac) -> Sremac>(&mut self, f: F) {
     let tmp = self.sremac();
     self.set_sremac(f(tmp))
  }

  pub unsafe fn rcgcwd(&self) -> Rcgcwd { 
     Rcgcwd(::core::ptr::read_volatile(((self.0 as usize) + 0x600) as *const u32))
  }
  pub unsafe fn set_rcgcwd(&mut self, value: Rcgcwd) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x600) as *mut u32, value.0);
  }
  pub unsafe fn with_rcgcwd<F: FnOnce(Rcgcwd) -> Rcgcwd>(&mut self, f: F) {
     let tmp = self.rcgcwd();
     self.set_rcgcwd(f(tmp))
  }

  pub unsafe fn rcgctimer(&self) -> Rcgctimer { 
     Rcgctimer(::core::ptr::read_volatile(((self.0 as usize) + 0x604) as *const u32))
  }
  pub unsafe fn set_rcgctimer(&mut self, value: Rcgctimer) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x604) as *mut u32, value.0);
  }
  pub unsafe fn with_rcgctimer<F: FnOnce(Rcgctimer) -> Rcgctimer>(&mut self, f: F) {
     let tmp = self.rcgctimer();
     self.set_rcgctimer(f(tmp))
  }

  pub unsafe fn rcgcgpio(&self) -> Rcgcgpio { 
     Rcgcgpio(::core::ptr::read_volatile(((self.0 as usize) + 0x608) as *const u32))
  }
  pub unsafe fn set_rcgcgpio(&mut self, value: Rcgcgpio) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x608) as *mut u32, value.0);
  }
  pub unsafe fn with_rcgcgpio<F: FnOnce(Rcgcgpio) -> Rcgcgpio>(&mut self, f: F) {
     let tmp = self.rcgcgpio();
     self.set_rcgcgpio(f(tmp))
  }

  pub unsafe fn rcgcdma(&self) -> Rcgcdma { 
     Rcgcdma(::core::ptr::read_volatile(((self.0 as usize) + 0x60c) as *const u32))
  }
  pub unsafe fn set_rcgcdma(&mut self, value: Rcgcdma) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x60c) as *mut u32, value.0);
  }
  pub unsafe fn with_rcgcdma<F: FnOnce(Rcgcdma) -> Rcgcdma>(&mut self, f: F) {
     let tmp = self.rcgcdma();
     self.set_rcgcdma(f(tmp))
  }

  pub unsafe fn rcgcepi(&self) -> Rcgcepi { 
     Rcgcepi(::core::ptr::read_volatile(((self.0 as usize) + 0x610) as *const u32))
  }
  pub unsafe fn set_rcgcepi(&mut self, value: Rcgcepi) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x610) as *mut u32, value.0);
  }
  pub unsafe fn with_rcgcepi<F: FnOnce(Rcgcepi) -> Rcgcepi>(&mut self, f: F) {
     let tmp = self.rcgcepi();
     self.set_rcgcepi(f(tmp))
  }

  pub unsafe fn rcgchib(&self) -> Rcgchib { 
     Rcgchib(::core::ptr::read_volatile(((self.0 as usize) + 0x614) as *const u32))
  }
  pub unsafe fn set_rcgchib(&mut self, value: Rcgchib) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x614) as *mut u32, value.0);
  }
  pub unsafe fn with_rcgchib<F: FnOnce(Rcgchib) -> Rcgchib>(&mut self, f: F) {
     let tmp = self.rcgchib();
     self.set_rcgchib(f(tmp))
  }

  pub unsafe fn rcgcuart(&self) -> Rcgcuart { 
     Rcgcuart(::core::ptr::read_volatile(((self.0 as usize) + 0x618) as *const u32))
  }
  pub unsafe fn set_rcgcuart(&mut self, value: Rcgcuart) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x618) as *mut u32, value.0);
  }
  pub unsafe fn with_rcgcuart<F: FnOnce(Rcgcuart) -> Rcgcuart>(&mut self, f: F) {
     let tmp = self.rcgcuart();
     self.set_rcgcuart(f(tmp))
  }

  pub unsafe fn rcgcssi(&self) -> Rcgcssi { 
     Rcgcssi(::core::ptr::read_volatile(((self.0 as usize) + 0x61c) as *const u32))
  }
  pub unsafe fn set_rcgcssi(&mut self, value: Rcgcssi) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x61c) as *mut u32, value.0);
  }
  pub unsafe fn with_rcgcssi<F: FnOnce(Rcgcssi) -> Rcgcssi>(&mut self, f: F) {
     let tmp = self.rcgcssi();
     self.set_rcgcssi(f(tmp))
  }

  pub unsafe fn rcgci2c(&self) -> Rcgci2c { 
     Rcgci2c(::core::ptr::read_volatile(((self.0 as usize) + 0x620) as *const u32))
  }
  pub unsafe fn set_rcgci2c(&mut self, value: Rcgci2c) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x620) as *mut u32, value.0);
  }
  pub unsafe fn with_rcgci2c<F: FnOnce(Rcgci2c) -> Rcgci2c>(&mut self, f: F) {
     let tmp = self.rcgci2c();
     self.set_rcgci2c(f(tmp))
  }

  pub unsafe fn rcgcusb(&self) -> Rcgcusb { 
     Rcgcusb(::core::ptr::read_volatile(((self.0 as usize) + 0x628) as *const u32))
  }
  pub unsafe fn set_rcgcusb(&mut self, value: Rcgcusb) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x628) as *mut u32, value.0);
  }
  pub unsafe fn with_rcgcusb<F: FnOnce(Rcgcusb) -> Rcgcusb>(&mut self, f: F) {
     let tmp = self.rcgcusb();
     self.set_rcgcusb(f(tmp))
  }

  pub unsafe fn rcgcephy(&self) -> Rcgcephy { 
     Rcgcephy(::core::ptr::read_volatile(((self.0 as usize) + 0x630) as *const u32))
  }
  pub unsafe fn set_rcgcephy(&mut self, value: Rcgcephy) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x630) as *mut u32, value.0);
  }
  pub unsafe fn with_rcgcephy<F: FnOnce(Rcgcephy) -> Rcgcephy>(&mut self, f: F) {
     let tmp = self.rcgcephy();
     self.set_rcgcephy(f(tmp))
  }

  pub unsafe fn rcgccan(&self) -> Rcgccan { 
     Rcgccan(::core::ptr::read_volatile(((self.0 as usize) + 0x634) as *const u32))
  }
  pub unsafe fn set_rcgccan(&mut self, value: Rcgccan) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x634) as *mut u32, value.0);
  }
  pub unsafe fn with_rcgccan<F: FnOnce(Rcgccan) -> Rcgccan>(&mut self, f: F) {
     let tmp = self.rcgccan();
     self.set_rcgccan(f(tmp))
  }

  pub unsafe fn rcgcadc(&self) -> Rcgcadc { 
     Rcgcadc(::core::ptr::read_volatile(((self.0 as usize) + 0x638) as *const u32))
  }
  pub unsafe fn set_rcgcadc(&mut self, value: Rcgcadc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x638) as *mut u32, value.0);
  }
  pub unsafe fn with_rcgcadc<F: FnOnce(Rcgcadc) -> Rcgcadc>(&mut self, f: F) {
     let tmp = self.rcgcadc();
     self.set_rcgcadc(f(tmp))
  }

  pub unsafe fn rcgcacmp(&self) -> Rcgcacmp { 
     Rcgcacmp(::core::ptr::read_volatile(((self.0 as usize) + 0x63c) as *const u32))
  }
  pub unsafe fn set_rcgcacmp(&mut self, value: Rcgcacmp) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x63c) as *mut u32, value.0);
  }
  pub unsafe fn with_rcgcacmp<F: FnOnce(Rcgcacmp) -> Rcgcacmp>(&mut self, f: F) {
     let tmp = self.rcgcacmp();
     self.set_rcgcacmp(f(tmp))
  }

  pub unsafe fn rcgcpwm(&self) -> Rcgcpwm { 
     Rcgcpwm(::core::ptr::read_volatile(((self.0 as usize) + 0x640) as *const u32))
  }
  pub unsafe fn set_rcgcpwm(&mut self, value: Rcgcpwm) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x640) as *mut u32, value.0);
  }
  pub unsafe fn with_rcgcpwm<F: FnOnce(Rcgcpwm) -> Rcgcpwm>(&mut self, f: F) {
     let tmp = self.rcgcpwm();
     self.set_rcgcpwm(f(tmp))
  }

  pub unsafe fn rcgcqei(&self) -> Rcgcqei { 
     Rcgcqei(::core::ptr::read_volatile(((self.0 as usize) + 0x644) as *const u32))
  }
  pub unsafe fn set_rcgcqei(&mut self, value: Rcgcqei) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x644) as *mut u32, value.0);
  }
  pub unsafe fn with_rcgcqei<F: FnOnce(Rcgcqei) -> Rcgcqei>(&mut self, f: F) {
     let tmp = self.rcgcqei();
     self.set_rcgcqei(f(tmp))
  }

  pub unsafe fn rcgceeprom(&self) -> Rcgceeprom { 
     Rcgceeprom(::core::ptr::read_volatile(((self.0 as usize) + 0x658) as *const u32))
  }
  pub unsafe fn set_rcgceeprom(&mut self, value: Rcgceeprom) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x658) as *mut u32, value.0);
  }
  pub unsafe fn with_rcgceeprom<F: FnOnce(Rcgceeprom) -> Rcgceeprom>(&mut self, f: F) {
     let tmp = self.rcgceeprom();
     self.set_rcgceeprom(f(tmp))
  }

  pub unsafe fn rcgcccm(&self) -> Rcgcccm { 
     Rcgcccm(::core::ptr::read_volatile(((self.0 as usize) + 0x674) as *const u32))
  }
  pub unsafe fn set_rcgcccm(&mut self, value: Rcgcccm) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x674) as *mut u32, value.0);
  }
  pub unsafe fn with_rcgcccm<F: FnOnce(Rcgcccm) -> Rcgcccm>(&mut self, f: F) {
     let tmp = self.rcgcccm();
     self.set_rcgcccm(f(tmp))
  }

  pub unsafe fn rcgcemac(&self) -> Rcgcemac { 
     Rcgcemac(::core::ptr::read_volatile(((self.0 as usize) + 0x69c) as *const u32))
  }
  pub unsafe fn set_rcgcemac(&mut self, value: Rcgcemac) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x69c) as *mut u32, value.0);
  }
  pub unsafe fn with_rcgcemac<F: FnOnce(Rcgcemac) -> Rcgcemac>(&mut self, f: F) {
     let tmp = self.rcgcemac();
     self.set_rcgcemac(f(tmp))
  }

  pub unsafe fn scgcwd(&self) -> Scgcwd { 
     Scgcwd(::core::ptr::read_volatile(((self.0 as usize) + 0x700) as *const u32))
  }
  pub unsafe fn set_scgcwd(&mut self, value: Scgcwd) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x700) as *mut u32, value.0);
  }
  pub unsafe fn with_scgcwd<F: FnOnce(Scgcwd) -> Scgcwd>(&mut self, f: F) {
     let tmp = self.scgcwd();
     self.set_scgcwd(f(tmp))
  }

  pub unsafe fn scgctimer(&self) -> Scgctimer { 
     Scgctimer(::core::ptr::read_volatile(((self.0 as usize) + 0x704) as *const u32))
  }
  pub unsafe fn set_scgctimer(&mut self, value: Scgctimer) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x704) as *mut u32, value.0);
  }
  pub unsafe fn with_scgctimer<F: FnOnce(Scgctimer) -> Scgctimer>(&mut self, f: F) {
     let tmp = self.scgctimer();
     self.set_scgctimer(f(tmp))
  }

  pub unsafe fn scgcgpio(&self) -> Scgcgpio { 
     Scgcgpio(::core::ptr::read_volatile(((self.0 as usize) + 0x708) as *const u32))
  }
  pub unsafe fn set_scgcgpio(&mut self, value: Scgcgpio) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x708) as *mut u32, value.0);
  }
  pub unsafe fn with_scgcgpio<F: FnOnce(Scgcgpio) -> Scgcgpio>(&mut self, f: F) {
     let tmp = self.scgcgpio();
     self.set_scgcgpio(f(tmp))
  }

  pub unsafe fn scgcdma(&self) -> Scgcdma { 
     Scgcdma(::core::ptr::read_volatile(((self.0 as usize) + 0x70c) as *const u32))
  }
  pub unsafe fn set_scgcdma(&mut self, value: Scgcdma) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x70c) as *mut u32, value.0);
  }
  pub unsafe fn with_scgcdma<F: FnOnce(Scgcdma) -> Scgcdma>(&mut self, f: F) {
     let tmp = self.scgcdma();
     self.set_scgcdma(f(tmp))
  }

  pub unsafe fn scgcepi(&self) -> Scgcepi { 
     Scgcepi(::core::ptr::read_volatile(((self.0 as usize) + 0x710) as *const u32))
  }
  pub unsafe fn set_scgcepi(&mut self, value: Scgcepi) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x710) as *mut u32, value.0);
  }
  pub unsafe fn with_scgcepi<F: FnOnce(Scgcepi) -> Scgcepi>(&mut self, f: F) {
     let tmp = self.scgcepi();
     self.set_scgcepi(f(tmp))
  }

  pub unsafe fn scgchib(&self) -> Scgchib { 
     Scgchib(::core::ptr::read_volatile(((self.0 as usize) + 0x714) as *const u32))
  }
  pub unsafe fn set_scgchib(&mut self, value: Scgchib) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x714) as *mut u32, value.0);
  }
  pub unsafe fn with_scgchib<F: FnOnce(Scgchib) -> Scgchib>(&mut self, f: F) {
     let tmp = self.scgchib();
     self.set_scgchib(f(tmp))
  }

  pub unsafe fn scgcuart(&self) -> Scgcuart { 
     Scgcuart(::core::ptr::read_volatile(((self.0 as usize) + 0x718) as *const u32))
  }
  pub unsafe fn set_scgcuart(&mut self, value: Scgcuart) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x718) as *mut u32, value.0);
  }
  pub unsafe fn with_scgcuart<F: FnOnce(Scgcuart) -> Scgcuart>(&mut self, f: F) {
     let tmp = self.scgcuart();
     self.set_scgcuart(f(tmp))
  }

  pub unsafe fn scgcssi(&self) -> Scgcssi { 
     Scgcssi(::core::ptr::read_volatile(((self.0 as usize) + 0x71c) as *const u32))
  }
  pub unsafe fn set_scgcssi(&mut self, value: Scgcssi) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x71c) as *mut u32, value.0);
  }
  pub unsafe fn with_scgcssi<F: FnOnce(Scgcssi) -> Scgcssi>(&mut self, f: F) {
     let tmp = self.scgcssi();
     self.set_scgcssi(f(tmp))
  }

  pub unsafe fn scgci2c(&self) -> Scgci2c { 
     Scgci2c(::core::ptr::read_volatile(((self.0 as usize) + 0x720) as *const u32))
  }
  pub unsafe fn set_scgci2c(&mut self, value: Scgci2c) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x720) as *mut u32, value.0);
  }
  pub unsafe fn with_scgci2c<F: FnOnce(Scgci2c) -> Scgci2c>(&mut self, f: F) {
     let tmp = self.scgci2c();
     self.set_scgci2c(f(tmp))
  }

  pub unsafe fn scgcusb(&self) -> Scgcusb { 
     Scgcusb(::core::ptr::read_volatile(((self.0 as usize) + 0x728) as *const u32))
  }
  pub unsafe fn set_scgcusb(&mut self, value: Scgcusb) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x728) as *mut u32, value.0);
  }
  pub unsafe fn with_scgcusb<F: FnOnce(Scgcusb) -> Scgcusb>(&mut self, f: F) {
     let tmp = self.scgcusb();
     self.set_scgcusb(f(tmp))
  }

  pub unsafe fn scgcephy(&self) -> Scgcephy { 
     Scgcephy(::core::ptr::read_volatile(((self.0 as usize) + 0x730) as *const u32))
  }
  pub unsafe fn set_scgcephy(&mut self, value: Scgcephy) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x730) as *mut u32, value.0);
  }
  pub unsafe fn with_scgcephy<F: FnOnce(Scgcephy) -> Scgcephy>(&mut self, f: F) {
     let tmp = self.scgcephy();
     self.set_scgcephy(f(tmp))
  }

  pub unsafe fn scgccan(&self) -> Scgccan { 
     Scgccan(::core::ptr::read_volatile(((self.0 as usize) + 0x734) as *const u32))
  }
  pub unsafe fn set_scgccan(&mut self, value: Scgccan) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x734) as *mut u32, value.0);
  }
  pub unsafe fn with_scgccan<F: FnOnce(Scgccan) -> Scgccan>(&mut self, f: F) {
     let tmp = self.scgccan();
     self.set_scgccan(f(tmp))
  }

  pub unsafe fn scgcadc(&self) -> Scgcadc { 
     Scgcadc(::core::ptr::read_volatile(((self.0 as usize) + 0x738) as *const u32))
  }
  pub unsafe fn set_scgcadc(&mut self, value: Scgcadc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x738) as *mut u32, value.0);
  }
  pub unsafe fn with_scgcadc<F: FnOnce(Scgcadc) -> Scgcadc>(&mut self, f: F) {
     let tmp = self.scgcadc();
     self.set_scgcadc(f(tmp))
  }

  pub unsafe fn scgcacmp(&self) -> Scgcacmp { 
     Scgcacmp(::core::ptr::read_volatile(((self.0 as usize) + 0x73c) as *const u32))
  }
  pub unsafe fn set_scgcacmp(&mut self, value: Scgcacmp) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x73c) as *mut u32, value.0);
  }
  pub unsafe fn with_scgcacmp<F: FnOnce(Scgcacmp) -> Scgcacmp>(&mut self, f: F) {
     let tmp = self.scgcacmp();
     self.set_scgcacmp(f(tmp))
  }

  pub unsafe fn scgcpwm(&self) -> Scgcpwm { 
     Scgcpwm(::core::ptr::read_volatile(((self.0 as usize) + 0x740) as *const u32))
  }
  pub unsafe fn set_scgcpwm(&mut self, value: Scgcpwm) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x740) as *mut u32, value.0);
  }
  pub unsafe fn with_scgcpwm<F: FnOnce(Scgcpwm) -> Scgcpwm>(&mut self, f: F) {
     let tmp = self.scgcpwm();
     self.set_scgcpwm(f(tmp))
  }

  pub unsafe fn scgcqei(&self) -> Scgcqei { 
     Scgcqei(::core::ptr::read_volatile(((self.0 as usize) + 0x744) as *const u32))
  }
  pub unsafe fn set_scgcqei(&mut self, value: Scgcqei) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x744) as *mut u32, value.0);
  }
  pub unsafe fn with_scgcqei<F: FnOnce(Scgcqei) -> Scgcqei>(&mut self, f: F) {
     let tmp = self.scgcqei();
     self.set_scgcqei(f(tmp))
  }

  pub unsafe fn scgceeprom(&self) -> Scgceeprom { 
     Scgceeprom(::core::ptr::read_volatile(((self.0 as usize) + 0x758) as *const u32))
  }
  pub unsafe fn set_scgceeprom(&mut self, value: Scgceeprom) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x758) as *mut u32, value.0);
  }
  pub unsafe fn with_scgceeprom<F: FnOnce(Scgceeprom) -> Scgceeprom>(&mut self, f: F) {
     let tmp = self.scgceeprom();
     self.set_scgceeprom(f(tmp))
  }

  pub unsafe fn scgcccm(&self) -> Scgcccm { 
     Scgcccm(::core::ptr::read_volatile(((self.0 as usize) + 0x774) as *const u32))
  }
  pub unsafe fn set_scgcccm(&mut self, value: Scgcccm) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x774) as *mut u32, value.0);
  }
  pub unsafe fn with_scgcccm<F: FnOnce(Scgcccm) -> Scgcccm>(&mut self, f: F) {
     let tmp = self.scgcccm();
     self.set_scgcccm(f(tmp))
  }

  pub unsafe fn scgcemac(&self) -> Scgcemac { 
     Scgcemac(::core::ptr::read_volatile(((self.0 as usize) + 0x79c) as *const u32))
  }
  pub unsafe fn set_scgcemac(&mut self, value: Scgcemac) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x79c) as *mut u32, value.0);
  }
  pub unsafe fn with_scgcemac<F: FnOnce(Scgcemac) -> Scgcemac>(&mut self, f: F) {
     let tmp = self.scgcemac();
     self.set_scgcemac(f(tmp))
  }

  pub unsafe fn dcgcwd(&self) -> Dcgcwd { 
     Dcgcwd(::core::ptr::read_volatile(((self.0 as usize) + 0x800) as *const u32))
  }
  pub unsafe fn set_dcgcwd(&mut self, value: Dcgcwd) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x800) as *mut u32, value.0);
  }
  pub unsafe fn with_dcgcwd<F: FnOnce(Dcgcwd) -> Dcgcwd>(&mut self, f: F) {
     let tmp = self.dcgcwd();
     self.set_dcgcwd(f(tmp))
  }

  pub unsafe fn dcgctimer(&self) -> Dcgctimer { 
     Dcgctimer(::core::ptr::read_volatile(((self.0 as usize) + 0x804) as *const u32))
  }
  pub unsafe fn set_dcgctimer(&mut self, value: Dcgctimer) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x804) as *mut u32, value.0);
  }
  pub unsafe fn with_dcgctimer<F: FnOnce(Dcgctimer) -> Dcgctimer>(&mut self, f: F) {
     let tmp = self.dcgctimer();
     self.set_dcgctimer(f(tmp))
  }

  pub unsafe fn dcgcgpio(&self) -> Dcgcgpio { 
     Dcgcgpio(::core::ptr::read_volatile(((self.0 as usize) + 0x808) as *const u32))
  }
  pub unsafe fn set_dcgcgpio(&mut self, value: Dcgcgpio) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x808) as *mut u32, value.0);
  }
  pub unsafe fn with_dcgcgpio<F: FnOnce(Dcgcgpio) -> Dcgcgpio>(&mut self, f: F) {
     let tmp = self.dcgcgpio();
     self.set_dcgcgpio(f(tmp))
  }

  pub unsafe fn dcgcdma(&self) -> Dcgcdma { 
     Dcgcdma(::core::ptr::read_volatile(((self.0 as usize) + 0x80c) as *const u32))
  }
  pub unsafe fn set_dcgcdma(&mut self, value: Dcgcdma) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x80c) as *mut u32, value.0);
  }
  pub unsafe fn with_dcgcdma<F: FnOnce(Dcgcdma) -> Dcgcdma>(&mut self, f: F) {
     let tmp = self.dcgcdma();
     self.set_dcgcdma(f(tmp))
  }

  pub unsafe fn dcgcepi(&self) -> Dcgcepi { 
     Dcgcepi(::core::ptr::read_volatile(((self.0 as usize) + 0x810) as *const u32))
  }
  pub unsafe fn set_dcgcepi(&mut self, value: Dcgcepi) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x810) as *mut u32, value.0);
  }
  pub unsafe fn with_dcgcepi<F: FnOnce(Dcgcepi) -> Dcgcepi>(&mut self, f: F) {
     let tmp = self.dcgcepi();
     self.set_dcgcepi(f(tmp))
  }

  pub unsafe fn dcgchib(&self) -> Dcgchib { 
     Dcgchib(::core::ptr::read_volatile(((self.0 as usize) + 0x814) as *const u32))
  }
  pub unsafe fn set_dcgchib(&mut self, value: Dcgchib) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x814) as *mut u32, value.0);
  }
  pub unsafe fn with_dcgchib<F: FnOnce(Dcgchib) -> Dcgchib>(&mut self, f: F) {
     let tmp = self.dcgchib();
     self.set_dcgchib(f(tmp))
  }

  pub unsafe fn dcgcuart(&self) -> Dcgcuart { 
     Dcgcuart(::core::ptr::read_volatile(((self.0 as usize) + 0x818) as *const u32))
  }
  pub unsafe fn set_dcgcuart(&mut self, value: Dcgcuart) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x818) as *mut u32, value.0);
  }
  pub unsafe fn with_dcgcuart<F: FnOnce(Dcgcuart) -> Dcgcuart>(&mut self, f: F) {
     let tmp = self.dcgcuart();
     self.set_dcgcuart(f(tmp))
  }

  pub unsafe fn dcgcssi(&self) -> Dcgcssi { 
     Dcgcssi(::core::ptr::read_volatile(((self.0 as usize) + 0x81c) as *const u32))
  }
  pub unsafe fn set_dcgcssi(&mut self, value: Dcgcssi) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x81c) as *mut u32, value.0);
  }
  pub unsafe fn with_dcgcssi<F: FnOnce(Dcgcssi) -> Dcgcssi>(&mut self, f: F) {
     let tmp = self.dcgcssi();
     self.set_dcgcssi(f(tmp))
  }

  pub unsafe fn dcgci2c(&self) -> Dcgci2c { 
     Dcgci2c(::core::ptr::read_volatile(((self.0 as usize) + 0x820) as *const u32))
  }
  pub unsafe fn set_dcgci2c(&mut self, value: Dcgci2c) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x820) as *mut u32, value.0);
  }
  pub unsafe fn with_dcgci2c<F: FnOnce(Dcgci2c) -> Dcgci2c>(&mut self, f: F) {
     let tmp = self.dcgci2c();
     self.set_dcgci2c(f(tmp))
  }

  pub unsafe fn dcgcusb(&self) -> Dcgcusb { 
     Dcgcusb(::core::ptr::read_volatile(((self.0 as usize) + 0x828) as *const u32))
  }
  pub unsafe fn set_dcgcusb(&mut self, value: Dcgcusb) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x828) as *mut u32, value.0);
  }
  pub unsafe fn with_dcgcusb<F: FnOnce(Dcgcusb) -> Dcgcusb>(&mut self, f: F) {
     let tmp = self.dcgcusb();
     self.set_dcgcusb(f(tmp))
  }

  pub unsafe fn dcgcephy(&self) -> Dcgcephy { 
     Dcgcephy(::core::ptr::read_volatile(((self.0 as usize) + 0x830) as *const u32))
  }
  pub unsafe fn set_dcgcephy(&mut self, value: Dcgcephy) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x830) as *mut u32, value.0);
  }
  pub unsafe fn with_dcgcephy<F: FnOnce(Dcgcephy) -> Dcgcephy>(&mut self, f: F) {
     let tmp = self.dcgcephy();
     self.set_dcgcephy(f(tmp))
  }

  pub unsafe fn dcgccan(&self) -> Dcgccan { 
     Dcgccan(::core::ptr::read_volatile(((self.0 as usize) + 0x834) as *const u32))
  }
  pub unsafe fn set_dcgccan(&mut self, value: Dcgccan) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x834) as *mut u32, value.0);
  }
  pub unsafe fn with_dcgccan<F: FnOnce(Dcgccan) -> Dcgccan>(&mut self, f: F) {
     let tmp = self.dcgccan();
     self.set_dcgccan(f(tmp))
  }

  pub unsafe fn dcgcadc(&self) -> Dcgcadc { 
     Dcgcadc(::core::ptr::read_volatile(((self.0 as usize) + 0x838) as *const u32))
  }
  pub unsafe fn set_dcgcadc(&mut self, value: Dcgcadc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x838) as *mut u32, value.0);
  }
  pub unsafe fn with_dcgcadc<F: FnOnce(Dcgcadc) -> Dcgcadc>(&mut self, f: F) {
     let tmp = self.dcgcadc();
     self.set_dcgcadc(f(tmp))
  }

  pub unsafe fn dcgcacmp(&self) -> Dcgcacmp { 
     Dcgcacmp(::core::ptr::read_volatile(((self.0 as usize) + 0x83c) as *const u32))
  }
  pub unsafe fn set_dcgcacmp(&mut self, value: Dcgcacmp) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x83c) as *mut u32, value.0);
  }
  pub unsafe fn with_dcgcacmp<F: FnOnce(Dcgcacmp) -> Dcgcacmp>(&mut self, f: F) {
     let tmp = self.dcgcacmp();
     self.set_dcgcacmp(f(tmp))
  }

  pub unsafe fn dcgcpwm(&self) -> Dcgcpwm { 
     Dcgcpwm(::core::ptr::read_volatile(((self.0 as usize) + 0x840) as *const u32))
  }
  pub unsafe fn set_dcgcpwm(&mut self, value: Dcgcpwm) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x840) as *mut u32, value.0);
  }
  pub unsafe fn with_dcgcpwm<F: FnOnce(Dcgcpwm) -> Dcgcpwm>(&mut self, f: F) {
     let tmp = self.dcgcpwm();
     self.set_dcgcpwm(f(tmp))
  }

  pub unsafe fn dcgcqei(&self) -> Dcgcqei { 
     Dcgcqei(::core::ptr::read_volatile(((self.0 as usize) + 0x844) as *const u32))
  }
  pub unsafe fn set_dcgcqei(&mut self, value: Dcgcqei) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x844) as *mut u32, value.0);
  }
  pub unsafe fn with_dcgcqei<F: FnOnce(Dcgcqei) -> Dcgcqei>(&mut self, f: F) {
     let tmp = self.dcgcqei();
     self.set_dcgcqei(f(tmp))
  }

  pub unsafe fn dcgceeprom(&self) -> Dcgceeprom { 
     Dcgceeprom(::core::ptr::read_volatile(((self.0 as usize) + 0x858) as *const u32))
  }
  pub unsafe fn set_dcgceeprom(&mut self, value: Dcgceeprom) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x858) as *mut u32, value.0);
  }
  pub unsafe fn with_dcgceeprom<F: FnOnce(Dcgceeprom) -> Dcgceeprom>(&mut self, f: F) {
     let tmp = self.dcgceeprom();
     self.set_dcgceeprom(f(tmp))
  }

  pub unsafe fn dcgcccm(&self) -> Dcgcccm { 
     Dcgcccm(::core::ptr::read_volatile(((self.0 as usize) + 0x874) as *const u32))
  }
  pub unsafe fn set_dcgcccm(&mut self, value: Dcgcccm) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x874) as *mut u32, value.0);
  }
  pub unsafe fn with_dcgcccm<F: FnOnce(Dcgcccm) -> Dcgcccm>(&mut self, f: F) {
     let tmp = self.dcgcccm();
     self.set_dcgcccm(f(tmp))
  }

  pub unsafe fn dcgcemac(&self) -> Dcgcemac { 
     Dcgcemac(::core::ptr::read_volatile(((self.0 as usize) + 0x89c) as *const u32))
  }
  pub unsafe fn set_dcgcemac(&mut self, value: Dcgcemac) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x89c) as *mut u32, value.0);
  }
  pub unsafe fn with_dcgcemac<F: FnOnce(Dcgcemac) -> Dcgcemac>(&mut self, f: F) {
     let tmp = self.dcgcemac();
     self.set_dcgcemac(f(tmp))
  }

  pub unsafe fn pcwd(&self) -> Pcwd { 
     Pcwd(::core::ptr::read_volatile(((self.0 as usize) + 0x900) as *const u32))
  }
  pub unsafe fn set_pcwd(&mut self, value: Pcwd) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x900) as *mut u32, value.0);
  }
  pub unsafe fn with_pcwd<F: FnOnce(Pcwd) -> Pcwd>(&mut self, f: F) {
     let tmp = self.pcwd();
     self.set_pcwd(f(tmp))
  }

  pub unsafe fn pctimer(&self) -> Pctimer { 
     Pctimer(::core::ptr::read_volatile(((self.0 as usize) + 0x904) as *const u32))
  }
  pub unsafe fn set_pctimer(&mut self, value: Pctimer) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x904) as *mut u32, value.0);
  }
  pub unsafe fn with_pctimer<F: FnOnce(Pctimer) -> Pctimer>(&mut self, f: F) {
     let tmp = self.pctimer();
     self.set_pctimer(f(tmp))
  }

  pub unsafe fn pcgpio(&self) -> Pcgpio { 
     Pcgpio(::core::ptr::read_volatile(((self.0 as usize) + 0x908) as *const u32))
  }
  pub unsafe fn set_pcgpio(&mut self, value: Pcgpio) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x908) as *mut u32, value.0);
  }
  pub unsafe fn with_pcgpio<F: FnOnce(Pcgpio) -> Pcgpio>(&mut self, f: F) {
     let tmp = self.pcgpio();
     self.set_pcgpio(f(tmp))
  }

  pub unsafe fn pcdma(&self) -> Pcdma { 
     Pcdma(::core::ptr::read_volatile(((self.0 as usize) + 0x90c) as *const u32))
  }
  pub unsafe fn set_pcdma(&mut self, value: Pcdma) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x90c) as *mut u32, value.0);
  }
  pub unsafe fn with_pcdma<F: FnOnce(Pcdma) -> Pcdma>(&mut self, f: F) {
     let tmp = self.pcdma();
     self.set_pcdma(f(tmp))
  }

  pub unsafe fn pcepi(&self) -> Pcepi { 
     Pcepi(::core::ptr::read_volatile(((self.0 as usize) + 0x910) as *const u32))
  }
  pub unsafe fn set_pcepi(&mut self, value: Pcepi) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x910) as *mut u32, value.0);
  }
  pub unsafe fn with_pcepi<F: FnOnce(Pcepi) -> Pcepi>(&mut self, f: F) {
     let tmp = self.pcepi();
     self.set_pcepi(f(tmp))
  }

  pub unsafe fn pchib(&self) -> Pchib { 
     Pchib(::core::ptr::read_volatile(((self.0 as usize) + 0x914) as *const u32))
  }
  pub unsafe fn set_pchib(&mut self, value: Pchib) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x914) as *mut u32, value.0);
  }
  pub unsafe fn with_pchib<F: FnOnce(Pchib) -> Pchib>(&mut self, f: F) {
     let tmp = self.pchib();
     self.set_pchib(f(tmp))
  }

  pub unsafe fn pcuart(&self) -> Pcuart { 
     Pcuart(::core::ptr::read_volatile(((self.0 as usize) + 0x918) as *const u32))
  }
  pub unsafe fn set_pcuart(&mut self, value: Pcuart) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x918) as *mut u32, value.0);
  }
  pub unsafe fn with_pcuart<F: FnOnce(Pcuart) -> Pcuart>(&mut self, f: F) {
     let tmp = self.pcuart();
     self.set_pcuart(f(tmp))
  }

  pub unsafe fn pcssi(&self) -> Pcssi { 
     Pcssi(::core::ptr::read_volatile(((self.0 as usize) + 0x91c) as *const u32))
  }
  pub unsafe fn set_pcssi(&mut self, value: Pcssi) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x91c) as *mut u32, value.0);
  }
  pub unsafe fn with_pcssi<F: FnOnce(Pcssi) -> Pcssi>(&mut self, f: F) {
     let tmp = self.pcssi();
     self.set_pcssi(f(tmp))
  }

  pub unsafe fn pci2c(&self) -> Pci2c { 
     Pci2c(::core::ptr::read_volatile(((self.0 as usize) + 0x920) as *const u32))
  }
  pub unsafe fn set_pci2c(&mut self, value: Pci2c) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x920) as *mut u32, value.0);
  }
  pub unsafe fn with_pci2c<F: FnOnce(Pci2c) -> Pci2c>(&mut self, f: F) {
     let tmp = self.pci2c();
     self.set_pci2c(f(tmp))
  }

  pub unsafe fn pcusb(&self) -> Pcusb { 
     Pcusb(::core::ptr::read_volatile(((self.0 as usize) + 0x928) as *const u32))
  }
  pub unsafe fn set_pcusb(&mut self, value: Pcusb) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x928) as *mut u32, value.0);
  }
  pub unsafe fn with_pcusb<F: FnOnce(Pcusb) -> Pcusb>(&mut self, f: F) {
     let tmp = self.pcusb();
     self.set_pcusb(f(tmp))
  }

  pub unsafe fn pcephy(&self) -> Pcephy { 
     Pcephy(::core::ptr::read_volatile(((self.0 as usize) + 0x930) as *const u32))
  }
  pub unsafe fn set_pcephy(&mut self, value: Pcephy) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x930) as *mut u32, value.0);
  }
  pub unsafe fn with_pcephy<F: FnOnce(Pcephy) -> Pcephy>(&mut self, f: F) {
     let tmp = self.pcephy();
     self.set_pcephy(f(tmp))
  }

  pub unsafe fn pccan(&self) -> Pccan { 
     Pccan(::core::ptr::read_volatile(((self.0 as usize) + 0x934) as *const u32))
  }
  pub unsafe fn set_pccan(&mut self, value: Pccan) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x934) as *mut u32, value.0);
  }
  pub unsafe fn with_pccan<F: FnOnce(Pccan) -> Pccan>(&mut self, f: F) {
     let tmp = self.pccan();
     self.set_pccan(f(tmp))
  }

  pub unsafe fn pcadc(&self) -> Pcadc { 
     Pcadc(::core::ptr::read_volatile(((self.0 as usize) + 0x938) as *const u32))
  }
  pub unsafe fn set_pcadc(&mut self, value: Pcadc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x938) as *mut u32, value.0);
  }
  pub unsafe fn with_pcadc<F: FnOnce(Pcadc) -> Pcadc>(&mut self, f: F) {
     let tmp = self.pcadc();
     self.set_pcadc(f(tmp))
  }

  pub unsafe fn pcacmp(&self) -> Pcacmp { 
     Pcacmp(::core::ptr::read_volatile(((self.0 as usize) + 0x93c) as *const u32))
  }
  pub unsafe fn set_pcacmp(&mut self, value: Pcacmp) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x93c) as *mut u32, value.0);
  }
  pub unsafe fn with_pcacmp<F: FnOnce(Pcacmp) -> Pcacmp>(&mut self, f: F) {
     let tmp = self.pcacmp();
     self.set_pcacmp(f(tmp))
  }

  pub unsafe fn pcpwm(&self) -> Pcpwm { 
     Pcpwm(::core::ptr::read_volatile(((self.0 as usize) + 0x940) as *const u32))
  }
  pub unsafe fn set_pcpwm(&mut self, value: Pcpwm) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x940) as *mut u32, value.0);
  }
  pub unsafe fn with_pcpwm<F: FnOnce(Pcpwm) -> Pcpwm>(&mut self, f: F) {
     let tmp = self.pcpwm();
     self.set_pcpwm(f(tmp))
  }

  pub unsafe fn pcqei(&self) -> Pcqei { 
     Pcqei(::core::ptr::read_volatile(((self.0 as usize) + 0x944) as *const u32))
  }
  pub unsafe fn set_pcqei(&mut self, value: Pcqei) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x944) as *mut u32, value.0);
  }
  pub unsafe fn with_pcqei<F: FnOnce(Pcqei) -> Pcqei>(&mut self, f: F) {
     let tmp = self.pcqei();
     self.set_pcqei(f(tmp))
  }

  pub unsafe fn pceeprom(&self) -> Pceeprom { 
     Pceeprom(::core::ptr::read_volatile(((self.0 as usize) + 0x958) as *const u32))
  }
  pub unsafe fn set_pceeprom(&mut self, value: Pceeprom) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x958) as *mut u32, value.0);
  }
  pub unsafe fn with_pceeprom<F: FnOnce(Pceeprom) -> Pceeprom>(&mut self, f: F) {
     let tmp = self.pceeprom();
     self.set_pceeprom(f(tmp))
  }

  pub unsafe fn pcccm(&self) -> Pcccm { 
     Pcccm(::core::ptr::read_volatile(((self.0 as usize) + 0x974) as *const u32))
  }
  pub unsafe fn set_pcccm(&mut self, value: Pcccm) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x974) as *mut u32, value.0);
  }
  pub unsafe fn with_pcccm<F: FnOnce(Pcccm) -> Pcccm>(&mut self, f: F) {
     let tmp = self.pcccm();
     self.set_pcccm(f(tmp))
  }

  pub unsafe fn pcemac(&self) -> Pcemac { 
     Pcemac(::core::ptr::read_volatile(((self.0 as usize) + 0x99c) as *const u32))
  }
  pub unsafe fn set_pcemac(&mut self, value: Pcemac) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x99c) as *mut u32, value.0);
  }
  pub unsafe fn with_pcemac<F: FnOnce(Pcemac) -> Pcemac>(&mut self, f: F) {
     let tmp = self.pcemac();
     self.set_pcemac(f(tmp))
  }

  pub unsafe fn prwd(&self) -> Prwd { 
     Prwd(::core::ptr::read_volatile(((self.0 as usize) + 0xa00) as *const u32))
  }
  pub unsafe fn set_prwd(&mut self, value: Prwd) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa00) as *mut u32, value.0);
  }
  pub unsafe fn with_prwd<F: FnOnce(Prwd) -> Prwd>(&mut self, f: F) {
     let tmp = self.prwd();
     self.set_prwd(f(tmp))
  }

  pub unsafe fn prtimer(&self) -> Prtimer { 
     Prtimer(::core::ptr::read_volatile(((self.0 as usize) + 0xa04) as *const u32))
  }
  pub unsafe fn set_prtimer(&mut self, value: Prtimer) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa04) as *mut u32, value.0);
  }
  pub unsafe fn with_prtimer<F: FnOnce(Prtimer) -> Prtimer>(&mut self, f: F) {
     let tmp = self.prtimer();
     self.set_prtimer(f(tmp))
  }

  pub unsafe fn prgpio(&self) -> Prgpio { 
     Prgpio(::core::ptr::read_volatile(((self.0 as usize) + 0xa08) as *const u32))
  }
  pub unsafe fn set_prgpio(&mut self, value: Prgpio) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa08) as *mut u32, value.0);
  }
  pub unsafe fn with_prgpio<F: FnOnce(Prgpio) -> Prgpio>(&mut self, f: F) {
     let tmp = self.prgpio();
     self.set_prgpio(f(tmp))
  }

  pub unsafe fn prdma(&self) -> Prdma { 
     Prdma(::core::ptr::read_volatile(((self.0 as usize) + 0xa0c) as *const u32))
  }
  pub unsafe fn set_prdma(&mut self, value: Prdma) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa0c) as *mut u32, value.0);
  }
  pub unsafe fn with_prdma<F: FnOnce(Prdma) -> Prdma>(&mut self, f: F) {
     let tmp = self.prdma();
     self.set_prdma(f(tmp))
  }

  pub unsafe fn prepi(&self) -> Prepi { 
     Prepi(::core::ptr::read_volatile(((self.0 as usize) + 0xa10) as *const u32))
  }
  pub unsafe fn set_prepi(&mut self, value: Prepi) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa10) as *mut u32, value.0);
  }
  pub unsafe fn with_prepi<F: FnOnce(Prepi) -> Prepi>(&mut self, f: F) {
     let tmp = self.prepi();
     self.set_prepi(f(tmp))
  }

  pub unsafe fn prhib(&self) -> Prhib { 
     Prhib(::core::ptr::read_volatile(((self.0 as usize) + 0xa14) as *const u32))
  }
  pub unsafe fn set_prhib(&mut self, value: Prhib) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa14) as *mut u32, value.0);
  }
  pub unsafe fn with_prhib<F: FnOnce(Prhib) -> Prhib>(&mut self, f: F) {
     let tmp = self.prhib();
     self.set_prhib(f(tmp))
  }

  pub unsafe fn pruart(&self) -> Pruart { 
     Pruart(::core::ptr::read_volatile(((self.0 as usize) + 0xa18) as *const u32))
  }
  pub unsafe fn set_pruart(&mut self, value: Pruart) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa18) as *mut u32, value.0);
  }
  pub unsafe fn with_pruart<F: FnOnce(Pruart) -> Pruart>(&mut self, f: F) {
     let tmp = self.pruart();
     self.set_pruart(f(tmp))
  }

  pub unsafe fn prssi(&self) -> Prssi { 
     Prssi(::core::ptr::read_volatile(((self.0 as usize) + 0xa1c) as *const u32))
  }
  pub unsafe fn set_prssi(&mut self, value: Prssi) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa1c) as *mut u32, value.0);
  }
  pub unsafe fn with_prssi<F: FnOnce(Prssi) -> Prssi>(&mut self, f: F) {
     let tmp = self.prssi();
     self.set_prssi(f(tmp))
  }

  pub unsafe fn pri2c(&self) -> Pri2c { 
     Pri2c(::core::ptr::read_volatile(((self.0 as usize) + 0xa20) as *const u32))
  }
  pub unsafe fn set_pri2c(&mut self, value: Pri2c) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa20) as *mut u32, value.0);
  }
  pub unsafe fn with_pri2c<F: FnOnce(Pri2c) -> Pri2c>(&mut self, f: F) {
     let tmp = self.pri2c();
     self.set_pri2c(f(tmp))
  }

  pub unsafe fn prusb(&self) -> Prusb { 
     Prusb(::core::ptr::read_volatile(((self.0 as usize) + 0xa28) as *const u32))
  }
  pub unsafe fn set_prusb(&mut self, value: Prusb) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa28) as *mut u32, value.0);
  }
  pub unsafe fn with_prusb<F: FnOnce(Prusb) -> Prusb>(&mut self, f: F) {
     let tmp = self.prusb();
     self.set_prusb(f(tmp))
  }

  pub unsafe fn prephy(&self) -> Prephy { 
     Prephy(::core::ptr::read_volatile(((self.0 as usize) + 0xa30) as *const u32))
  }
  pub unsafe fn set_prephy(&mut self, value: Prephy) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa30) as *mut u32, value.0);
  }
  pub unsafe fn with_prephy<F: FnOnce(Prephy) -> Prephy>(&mut self, f: F) {
     let tmp = self.prephy();
     self.set_prephy(f(tmp))
  }

  pub unsafe fn prcan(&self) -> Prcan { 
     Prcan(::core::ptr::read_volatile(((self.0 as usize) + 0xa34) as *const u32))
  }
  pub unsafe fn set_prcan(&mut self, value: Prcan) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa34) as *mut u32, value.0);
  }
  pub unsafe fn with_prcan<F: FnOnce(Prcan) -> Prcan>(&mut self, f: F) {
     let tmp = self.prcan();
     self.set_prcan(f(tmp))
  }

  pub unsafe fn pradc(&self) -> Pradc { 
     Pradc(::core::ptr::read_volatile(((self.0 as usize) + 0xa38) as *const u32))
  }
  pub unsafe fn set_pradc(&mut self, value: Pradc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa38) as *mut u32, value.0);
  }
  pub unsafe fn with_pradc<F: FnOnce(Pradc) -> Pradc>(&mut self, f: F) {
     let tmp = self.pradc();
     self.set_pradc(f(tmp))
  }

  pub unsafe fn pracmp(&self) -> Pracmp { 
     Pracmp(::core::ptr::read_volatile(((self.0 as usize) + 0xa3c) as *const u32))
  }
  pub unsafe fn set_pracmp(&mut self, value: Pracmp) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa3c) as *mut u32, value.0);
  }
  pub unsafe fn with_pracmp<F: FnOnce(Pracmp) -> Pracmp>(&mut self, f: F) {
     let tmp = self.pracmp();
     self.set_pracmp(f(tmp))
  }

  pub unsafe fn prpwm(&self) -> Prpwm { 
     Prpwm(::core::ptr::read_volatile(((self.0 as usize) + 0xa40) as *const u32))
  }
  pub unsafe fn set_prpwm(&mut self, value: Prpwm) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa40) as *mut u32, value.0);
  }
  pub unsafe fn with_prpwm<F: FnOnce(Prpwm) -> Prpwm>(&mut self, f: F) {
     let tmp = self.prpwm();
     self.set_prpwm(f(tmp))
  }

  pub unsafe fn prqei(&self) -> Prqei { 
     Prqei(::core::ptr::read_volatile(((self.0 as usize) + 0xa44) as *const u32))
  }
  pub unsafe fn set_prqei(&mut self, value: Prqei) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa44) as *mut u32, value.0);
  }
  pub unsafe fn with_prqei<F: FnOnce(Prqei) -> Prqei>(&mut self, f: F) {
     let tmp = self.prqei();
     self.set_prqei(f(tmp))
  }

  pub unsafe fn preeprom(&self) -> Preeprom { 
     Preeprom(::core::ptr::read_volatile(((self.0 as usize) + 0xa58) as *const u32))
  }
  pub unsafe fn set_preeprom(&mut self, value: Preeprom) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa58) as *mut u32, value.0);
  }
  pub unsafe fn with_preeprom<F: FnOnce(Preeprom) -> Preeprom>(&mut self, f: F) {
     let tmp = self.preeprom();
     self.set_preeprom(f(tmp))
  }

  pub unsafe fn prccm(&self) -> Prccm { 
     Prccm(::core::ptr::read_volatile(((self.0 as usize) + 0xa74) as *const u32))
  }
  pub unsafe fn set_prccm(&mut self, value: Prccm) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa74) as *mut u32, value.0);
  }
  pub unsafe fn with_prccm<F: FnOnce(Prccm) -> Prccm>(&mut self, f: F) {
     let tmp = self.prccm();
     self.set_prccm(f(tmp))
  }

  pub unsafe fn premac(&self) -> Premac { 
     Premac(::core::ptr::read_volatile(((self.0 as usize) + 0xa9c) as *const u32))
  }
  pub unsafe fn set_premac(&mut self, value: Premac) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa9c) as *mut u32, value.0);
  }
  pub unsafe fn with_premac<F: FnOnce(Premac) -> Premac>(&mut self, f: F) {
     let tmp = self.premac();
     self.set_premac(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Did0(pub u32);

impl Did0 {
  pub fn min(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_min(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn maj(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
  pub fn set_maj(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

  pub fn class(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  pub fn set_class(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn ver(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x7 // [30:28]
  }
  pub fn set_ver(mut self, value: u32) -> Self {
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
  pub fn qual(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  pub fn set_qual(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn rohs(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_rohs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn pkg(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x3 // [4:3]
  }
  pub fn set_pkg(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn temp(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x7 // [7:5]
  }
  pub fn set_temp(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn pincnt(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x7 // [15:13]
  }
  pub fn set_pincnt(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn prtno(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  pub fn set_prtno(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn fam(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  pub fn set_fam(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

  pub fn ver(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0xf // [31:28]
  }
  pub fn set_ver(mut self, value: u32) -> Self {
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
  pub fn vdd_ubor(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  pub fn set_vdd_ubor(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn vdda_ubor(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  pub fn set_vdda_ubor(mut self, value: u32) -> Self {
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
  pub fn borris(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_borris(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn mofris(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_mofris(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn plllris(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_plllris(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn moscpupris(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_moscpupris(mut self, value: u32) -> Self {
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
  pub fn borim(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_borim(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn mofim(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_mofim(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn plllim(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_plllim(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn moscpupim(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_moscpupim(mut self, value: u32) -> Self {
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
  pub fn bormis(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_bormis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn mofmis(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_mofmis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn plllmis(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_plllmis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn moscpupmis(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_moscpupmis(mut self, value: u32) -> Self {
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
  pub fn ext(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_ext(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn por(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_por(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn bor(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_bor(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn wdt0(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_wdt0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn sw(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_sw(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn wdt1(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_wdt1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn hib(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_hib(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn hssr(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_hssr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn moscfail(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_moscfail(mut self, value: u32) -> Self {
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
  pub fn vdd_ubor(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_vdd_ubor(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn vdda_ubor(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_vdda_ubor(mut self, value: u32) -> Self {
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
  pub fn external(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_external(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn power(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_power(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn wdt0(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_wdt0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn wdt1(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_wdt1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn tamper(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_tamper(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn moscfail(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_moscfail(mut self, value: u32) -> Self {
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
  pub fn cval(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_cval(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn moscim(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_moscim(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn noxtal(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_noxtal(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn pwrdn(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_pwrdn(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn oscrng(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_oscrng(mut self, value: u32) -> Self {
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
  pub fn psysdiv(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3ff // [9:0]
  }
  pub fn set_psysdiv(mut self, value: u32) -> Self {
     assert!((value & !0x3ff) == 0);
     self.0 &= !(0x3ff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn osysdiv(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3ff // [19:10]
  }
  pub fn set_osysdiv(mut self, value: u32) -> Self {
     assert!((value & !0x3ff) == 0);
     self.0 &= !(0x3ff << 10);
     self.0 |= value << 10;
     self
  }

  pub fn oscsrc(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0xf // [23:20]
  }
  pub fn set_oscsrc(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

  pub fn pllsrc(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  pub fn set_pllsrc(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

  pub fn usepll(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  pub fn set_usepll(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn acg(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  pub fn set_acg(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  pub fn newfreq(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_newfreq(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn memtimu(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_memtimu(mut self, value: u32) -> Self {
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
  pub fn fws(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  pub fn set_fws(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  pub fn fbce(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_fbce(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn fbcht(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0xf // [9:6]
  }
  pub fn set_fbcht(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 6);
     self.0 |= value << 6;
     self
  }

  pub fn ews(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  pub fn set_ews(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  pub fn ebce(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_ebce(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn ebcht(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0xf // [25:22]
  }
  pub fn set_ebcht(mut self, value: u32) -> Self {
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
  pub fn altclk(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  pub fn set_altclk(mut self, value: u32) -> Self {
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
  pub fn dssysdiv(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3ff // [9:0]
  }
  pub fn set_dssysdiv(mut self, value: u32) -> Self {
     assert!((value & !0x3ff) == 0);
     self.0 &= !(0x3ff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn dsoscsrc(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0xf // [23:20]
  }
  pub fn set_dsoscsrc(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

  pub fn moscdpd(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_moscdpd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pioscpd(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pioscpd(mut self, value: u32) -> Self {
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
  pub fn div(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_div(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn src(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3 // [17:16]
  }
  pub fn set_src(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn en(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_en(mut self, value: u32) -> Self {
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
  pub fn fpu(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_fpu(mut self, value: u32) -> Self {
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
  pub fn ut(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7f // [6:0]
  }
  pub fn set_ut(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 0);
     self.0 |= value << 0;
     self
  }

  pub fn update(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_update(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn cal(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_cal(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn uten(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_uten(mut self, value: u32) -> Self {
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
  pub fn ct(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7f // [6:0]
  }
  pub fn set_ct(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 0);
     self.0 |= value << 0;
     self
  }

  pub fn cr(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  pub fn set_cr(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn dt(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x7f // [22:16]
  }
  pub fn set_dt(mut self, value: u32) -> Self {
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
  pub fn mint(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3ff // [9:0]
  }
  pub fn set_mint(mut self, value: u32) -> Self {
     assert!((value & !0x3ff) == 0);
     self.0 &= !(0x3ff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn mfrac(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3ff // [19:10]
  }
  pub fn set_mfrac(mut self, value: u32) -> Self {
     assert!((value & !0x3ff) == 0);
     self.0 &= !(0x3ff << 10);
     self.0 |= value << 10;
     self
  }

  pub fn pllpwr(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_pllpwr(mut self, value: u32) -> Self {
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
  pub fn n(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1f // [4:0]
  }
  pub fn set_n(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

  pub fn q(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1f // [12:8]
  }
  pub fn set_q(mut self, value: u32) -> Self {
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
  pub fn lock(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_lock(mut self, value: u32) -> Self {
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
  pub fn srampm(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  pub fn set_srampm(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn flashpm(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  pub fn set_flashpm(mut self, value: u32) -> Self {
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
  pub fn srampm(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  pub fn set_srampm(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn flashpm(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  pub fn set_flashpm(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn tspd(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_tspd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn ldosm(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_ldosm(mut self, value: u32) -> Self {
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
  pub fn fwb(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_fwb(mut self, value: u32) -> Self {
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
  pub fn vldo(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_vldo(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn vadjen(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_vadjen(mut self, value: u32) -> Self {
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
  pub fn vldo(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_vldo(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn vadjen(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_vadjen(mut self, value: u32) -> Self {
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
  pub fn extres(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  pub fn set_extres(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn bor(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
  pub fn set_bor(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn wdog0(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  pub fn set_wdog0(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn wdog1(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
  pub fn set_wdog1(mut self, value: u32) -> Self {
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
  pub fn cdoff(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffff // [23:0]
  }
  pub fn set_cdoff(mut self, value: u32) -> Self {
     assert!((value & !0xffffff) == 0);
     self.0 &= !(0xffffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn key(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
  pub fn set_key(mut self, value: u32) -> Self {
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
  pub fn pwrstat(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  pub fn set_pwrstat(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn memstat(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
  pub fn set_memstat(mut self, value: u32) -> Self {
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
  pub fn pwrctl(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  pub fn set_pwrctl(mut self, value: u32) -> Self {
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
  pub fn pwrstat(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  pub fn set_pwrstat(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn memstat(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
  pub fn set_memstat(mut self, value: u32) -> Self {
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
  pub fn pwrctl(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  pub fn set_pwrctl(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_p1(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_p1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn p2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_p2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn p3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_p3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn p4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_p4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn p5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_p5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn p6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_p6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn p7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_p7(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_p1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn p2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_p2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn p3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_p3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn p4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_p4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn p5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_p5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn p6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_p6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn p7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_p7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn p8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_p8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn p9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_p9(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn p10(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_p10(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn p11(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_p11(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn p12(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_p12(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn p13(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_p13(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn p14(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_p14(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_p1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn p2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_p2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn p3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_p3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn p4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_p4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn p5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_p5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn p6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_p6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn p7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_p7(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_p1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn p2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_p2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn p3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_p3(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_p1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn p2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_p2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn p3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_p3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn p4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_p4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn p5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_p5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn p6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_p6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn p7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_p7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn p8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_p8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn p9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_p9(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_p1(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_p1(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_r1(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn r4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_r4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn r5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_r5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn r6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_r6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn r7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_r7(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn r4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_r4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn r5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_r5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn r6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_r6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn r7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_r7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn r8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_r8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn r9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_r9(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn r10(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_r10(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn r11(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_r11(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn r12(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_r12(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn r13(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_r13(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn r14(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_r14(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn r4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_r4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn r5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_r5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn r6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_r6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn r7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_r7(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_r3(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn r4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_r4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn r5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_r5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn r6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_r6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn r7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_r7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn r8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_r8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn r9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_r9(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_r1(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_r1(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_r1(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn r4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_r4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn r5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_r5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn r6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_r6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn r7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_r7(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn r4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_r4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn r5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_r5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn r6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_r6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn r7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_r7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn r8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_r8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn r9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_r9(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn r10(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_r10(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn r11(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_r11(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn r12(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_r12(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn r13(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_r13(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn r14(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_r14(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn r4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_r4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn r5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_r5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn r6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_r6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn r7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_r7(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_r3(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn r4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_r4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn r5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_r5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn r6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_r6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn r7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_r7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn r8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_r8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn r9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_r9(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_r1(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_r1(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn s1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_s1(mut self, value: u32) -> Self {
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
  pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn s1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_s1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn s2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_s2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn s3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_s3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn s4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_s4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn s5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_s5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn s6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_s6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn s7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_s7(mut self, value: u32) -> Self {
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
  pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn s1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_s1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn s2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_s2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn s3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_s3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn s4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_s4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn s5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_s5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn s6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_s6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn s7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_s7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn s8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_s8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn s9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_s9(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn s10(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_s10(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn s11(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_s11(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn s12(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_s12(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn s13(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_s13(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn s14(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_s14(mut self, value: u32) -> Self {
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
  pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_s0(mut self, value: u32) -> Self {
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
  pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_s0(mut self, value: u32) -> Self {
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
  pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_s0(mut self, value: u32) -> Self {
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
  pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn s1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_s1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn s2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_s2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn s3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_s3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn s4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_s4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn s5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_s5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn s6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_s6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn s7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_s7(mut self, value: u32) -> Self {
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
  pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn s1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_s1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn s2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_s2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn s3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_s3(mut self, value: u32) -> Self {
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
  pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn s1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_s1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn s2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_s2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn s3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_s3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn s4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_s4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn s5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_s5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn s6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_s6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn s7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_s7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn s8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_s8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn s9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_s9(mut self, value: u32) -> Self {
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
  pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_s0(mut self, value: u32) -> Self {
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
  pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_s0(mut self, value: u32) -> Self {
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
  pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn s1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_s1(mut self, value: u32) -> Self {
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
  pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_s0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn s1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_s1(mut self, value: u32) -> Self {
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
  pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_s0(mut self, value: u32) -> Self {
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
  pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_s0(mut self, value: u32) -> Self {
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
  pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_s0(mut self, value: u32) -> Self {
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
  pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_s0(mut self, value: u32) -> Self {
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
  pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_s0(mut self, value: u32) -> Self {
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
  pub fn s0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_s0(mut self, value: u32) -> Self {
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
  pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn d1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_d1(mut self, value: u32) -> Self {
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
  pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn d1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_d1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn d2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_d2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn d3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_d3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn d4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_d4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn d5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_d5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn d6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_d6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn d7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_d7(mut self, value: u32) -> Self {
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
  pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn d1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_d1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn d2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_d2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn d3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_d3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn d4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_d4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn d5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_d5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn d6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_d6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn d7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_d7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn d8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_d8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn d9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_d9(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn d10(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_d10(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn d11(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_d11(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn d12(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_d12(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn d13(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_d13(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn d14(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_d14(mut self, value: u32) -> Self {
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
  pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_d0(mut self, value: u32) -> Self {
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
  pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_d0(mut self, value: u32) -> Self {
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
  pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_d0(mut self, value: u32) -> Self {
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
  pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn d1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_d1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn d2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_d2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn d3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_d3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn d4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_d4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn d5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_d5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn d6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_d6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn d7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_d7(mut self, value: u32) -> Self {
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
  pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn d1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_d1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn d2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_d2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn d3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_d3(mut self, value: u32) -> Self {
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
  pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn d1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_d1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn d2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_d2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn d3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_d3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn d4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_d4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn d5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_d5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn d6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_d6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn d7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_d7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn d8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_d8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn d9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_d9(mut self, value: u32) -> Self {
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
  pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_d0(mut self, value: u32) -> Self {
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
  pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_d0(mut self, value: u32) -> Self {
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
  pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn d1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_d1(mut self, value: u32) -> Self {
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
  pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_d0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn d1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_d1(mut self, value: u32) -> Self {
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
  pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_d0(mut self, value: u32) -> Self {
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
  pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_d0(mut self, value: u32) -> Self {
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
  pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_d0(mut self, value: u32) -> Self {
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
  pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_d0(mut self, value: u32) -> Self {
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
  pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_d0(mut self, value: u32) -> Self {
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
  pub fn d0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_d0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_p1(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_p1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn p2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_p2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn p3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_p3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn p4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_p4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn p5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_p5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn p6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_p6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn p7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_p7(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_p1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn p2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_p2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn p3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_p3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn p4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_p4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn p5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_p5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn p6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_p6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn p7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_p7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn p8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_p8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn p9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_p9(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn p10(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_p10(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn p11(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_p11(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn p12(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_p12(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn p13(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_p13(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn p14(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_p14(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_p1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn p2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_p2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn p3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_p3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn p4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_p4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn p5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_p5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn p6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_p6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn p7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_p7(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_p1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn p2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_p2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn p3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_p3(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_p1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn p2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_p2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn p3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_p3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn p4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_p4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn p5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_p5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn p6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_p6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn p7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_p7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn p8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_p8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn p9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_p9(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_p1(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn p1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_p1(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn p0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_p0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_r1(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn r4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_r4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn r5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_r5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn r6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_r6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn r7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_r7(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn r4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_r4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn r5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_r5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn r6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_r6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn r7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_r7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn r8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_r8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn r9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_r9(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn r10(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_r10(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn r11(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_r11(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn r12(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_r12(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn r13(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_r13(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn r14(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_r14(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn r4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_r4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn r5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_r5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn r6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_r6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn r7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_r7(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_r3(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_r1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn r2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_r2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn r3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_r3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn r4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_r4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn r5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_r5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn r6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_r6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn r7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_r7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn r8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_r8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn r9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_r9(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_r1(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn r1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_r1(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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
  pub fn r0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_r0(mut self, value: u32) -> Self {
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

