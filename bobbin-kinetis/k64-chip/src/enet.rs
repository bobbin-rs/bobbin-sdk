pub const ENET: Enet = Enet(0x400c0000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Enet(pub u32);

impl Enet {
  pub unsafe fn eir(&self) -> Eir { 
     Eir(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
  }
  pub unsafe fn set_eir(&mut self, value: Eir) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
  }
  pub unsafe fn with_eir<F: FnOnce(Eir) -> Eir>(&mut self, f: F) {
     let tmp = self.eir();
     self.set_eir(f(tmp))
  }

  pub unsafe fn eimr(&self) -> Eimr { 
     Eimr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
  }
  pub unsafe fn set_eimr(&mut self, value: Eimr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
  }
  pub unsafe fn with_eimr<F: FnOnce(Eimr) -> Eimr>(&mut self, f: F) {
     let tmp = self.eimr();
     self.set_eimr(f(tmp))
  }

  pub unsafe fn rdar(&self) -> Rdar { 
     Rdar(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
  }
  pub unsafe fn set_rdar(&mut self, value: Rdar) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
  }
  pub unsafe fn with_rdar<F: FnOnce(Rdar) -> Rdar>(&mut self, f: F) {
     let tmp = self.rdar();
     self.set_rdar(f(tmp))
  }

  pub unsafe fn tdar(&self) -> Tdar { 
     Tdar(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
  }
  pub unsafe fn set_tdar(&mut self, value: Tdar) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
  }
  pub unsafe fn with_tdar<F: FnOnce(Tdar) -> Tdar>(&mut self, f: F) {
     let tmp = self.tdar();
     self.set_tdar(f(tmp))
  }

  pub unsafe fn ecr(&self) -> Ecr { 
     Ecr(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
  }
  pub unsafe fn set_ecr(&mut self, value: Ecr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
  }
  pub unsafe fn with_ecr<F: FnOnce(Ecr) -> Ecr>(&mut self, f: F) {
     let tmp = self.ecr();
     self.set_ecr(f(tmp))
  }

  pub unsafe fn mmfr(&self) -> Mmfr { 
     Mmfr(::core::ptr::read_volatile(((self.0 as usize) + 0x40) as *const u32))
  }
  pub unsafe fn set_mmfr(&mut self, value: Mmfr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u32, value.0);
  }
  pub unsafe fn with_mmfr<F: FnOnce(Mmfr) -> Mmfr>(&mut self, f: F) {
     let tmp = self.mmfr();
     self.set_mmfr(f(tmp))
  }

  pub unsafe fn mscr(&self) -> Mscr { 
     Mscr(::core::ptr::read_volatile(((self.0 as usize) + 0x44) as *const u32))
  }
  pub unsafe fn set_mscr(&mut self, value: Mscr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x44) as *mut u32, value.0);
  }
  pub unsafe fn with_mscr<F: FnOnce(Mscr) -> Mscr>(&mut self, f: F) {
     let tmp = self.mscr();
     self.set_mscr(f(tmp))
  }

  pub unsafe fn mibc(&self) -> Mibc { 
     Mibc(::core::ptr::read_volatile(((self.0 as usize) + 0x64) as *const u32))
  }
  pub unsafe fn set_mibc(&mut self, value: Mibc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x64) as *mut u32, value.0);
  }
  pub unsafe fn with_mibc<F: FnOnce(Mibc) -> Mibc>(&mut self, f: F) {
     let tmp = self.mibc();
     self.set_mibc(f(tmp))
  }

  pub unsafe fn rcr(&self) -> Rcr { 
     Rcr(::core::ptr::read_volatile(((self.0 as usize) + 0x84) as *const u32))
  }
  pub unsafe fn set_rcr(&mut self, value: Rcr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x84) as *mut u32, value.0);
  }
  pub unsafe fn with_rcr<F: FnOnce(Rcr) -> Rcr>(&mut self, f: F) {
     let tmp = self.rcr();
     self.set_rcr(f(tmp))
  }

  pub unsafe fn tcr(&self) -> Tcr { 
     Tcr(::core::ptr::read_volatile(((self.0 as usize) + 0xc4) as *const u32))
  }
  pub unsafe fn set_tcr(&mut self, value: Tcr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc4) as *mut u32, value.0);
  }
  pub unsafe fn with_tcr<F: FnOnce(Tcr) -> Tcr>(&mut self, f: F) {
     let tmp = self.tcr();
     self.set_tcr(f(tmp))
  }

  pub unsafe fn palr(&self) -> Palr { 
     Palr(::core::ptr::read_volatile(((self.0 as usize) + 0xe4) as *const u32))
  }
  pub unsafe fn set_palr(&mut self, value: Palr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xe4) as *mut u32, value.0);
  }
  pub unsafe fn with_palr<F: FnOnce(Palr) -> Palr>(&mut self, f: F) {
     let tmp = self.palr();
     self.set_palr(f(tmp))
  }

  pub unsafe fn paur(&self) -> Paur { 
     Paur(::core::ptr::read_volatile(((self.0 as usize) + 0xe8) as *const u32))
  }
  pub unsafe fn set_paur(&mut self, value: Paur) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xe8) as *mut u32, value.0);
  }
  pub unsafe fn with_paur<F: FnOnce(Paur) -> Paur>(&mut self, f: F) {
     let tmp = self.paur();
     self.set_paur(f(tmp))
  }

  pub unsafe fn opd(&self) -> Opd { 
     Opd(::core::ptr::read_volatile(((self.0 as usize) + 0xec) as *const u32))
  }
  pub unsafe fn set_opd(&mut self, value: Opd) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xec) as *mut u32, value.0);
  }
  pub unsafe fn with_opd<F: FnOnce(Opd) -> Opd>(&mut self, f: F) {
     let tmp = self.opd();
     self.set_opd(f(tmp))
  }

  pub unsafe fn iaur(&self) -> Iaur { 
     Iaur(::core::ptr::read_volatile(((self.0 as usize) + 0x118) as *const u32))
  }
  pub unsafe fn set_iaur(&mut self, value: Iaur) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x118) as *mut u32, value.0);
  }
  pub unsafe fn with_iaur<F: FnOnce(Iaur) -> Iaur>(&mut self, f: F) {
     let tmp = self.iaur();
     self.set_iaur(f(tmp))
  }

  pub unsafe fn ialr(&self) -> Ialr { 
     Ialr(::core::ptr::read_volatile(((self.0 as usize) + 0x11c) as *const u32))
  }
  pub unsafe fn set_ialr(&mut self, value: Ialr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x11c) as *mut u32, value.0);
  }
  pub unsafe fn with_ialr<F: FnOnce(Ialr) -> Ialr>(&mut self, f: F) {
     let tmp = self.ialr();
     self.set_ialr(f(tmp))
  }

  pub unsafe fn gaur(&self) -> Gaur { 
     Gaur(::core::ptr::read_volatile(((self.0 as usize) + 0x120) as *const u32))
  }
  pub unsafe fn set_gaur(&mut self, value: Gaur) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x120) as *mut u32, value.0);
  }
  pub unsafe fn with_gaur<F: FnOnce(Gaur) -> Gaur>(&mut self, f: F) {
     let tmp = self.gaur();
     self.set_gaur(f(tmp))
  }

  pub unsafe fn galr(&self) -> Galr { 
     Galr(::core::ptr::read_volatile(((self.0 as usize) + 0x124) as *const u32))
  }
  pub unsafe fn set_galr(&mut self, value: Galr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x124) as *mut u32, value.0);
  }
  pub unsafe fn with_galr<F: FnOnce(Galr) -> Galr>(&mut self, f: F) {
     let tmp = self.galr();
     self.set_galr(f(tmp))
  }

  pub unsafe fn tfwr(&self) -> Tfwr { 
     Tfwr(::core::ptr::read_volatile(((self.0 as usize) + 0x144) as *const u32))
  }
  pub unsafe fn set_tfwr(&mut self, value: Tfwr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x144) as *mut u32, value.0);
  }
  pub unsafe fn with_tfwr<F: FnOnce(Tfwr) -> Tfwr>(&mut self, f: F) {
     let tmp = self.tfwr();
     self.set_tfwr(f(tmp))
  }

  pub unsafe fn rdsr(&self) -> Rdsr { 
     Rdsr(::core::ptr::read_volatile(((self.0 as usize) + 0x180) as *const u32))
  }
  pub unsafe fn set_rdsr(&mut self, value: Rdsr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x180) as *mut u32, value.0);
  }
  pub unsafe fn with_rdsr<F: FnOnce(Rdsr) -> Rdsr>(&mut self, f: F) {
     let tmp = self.rdsr();
     self.set_rdsr(f(tmp))
  }

  pub unsafe fn tdsr(&self) -> Tdsr { 
     Tdsr(::core::ptr::read_volatile(((self.0 as usize) + 0x184) as *const u32))
  }
  pub unsafe fn set_tdsr(&mut self, value: Tdsr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x184) as *mut u32, value.0);
  }
  pub unsafe fn with_tdsr<F: FnOnce(Tdsr) -> Tdsr>(&mut self, f: F) {
     let tmp = self.tdsr();
     self.set_tdsr(f(tmp))
  }

  pub unsafe fn mrbr(&self) -> Mrbr { 
     Mrbr(::core::ptr::read_volatile(((self.0 as usize) + 0x188) as *const u32))
  }
  pub unsafe fn set_mrbr(&mut self, value: Mrbr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x188) as *mut u32, value.0);
  }
  pub unsafe fn with_mrbr<F: FnOnce(Mrbr) -> Mrbr>(&mut self, f: F) {
     let tmp = self.mrbr();
     self.set_mrbr(f(tmp))
  }

  pub unsafe fn rsfl(&self) -> Rsfl { 
     Rsfl(::core::ptr::read_volatile(((self.0 as usize) + 0x190) as *const u32))
  }
  pub unsafe fn set_rsfl(&mut self, value: Rsfl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x190) as *mut u32, value.0);
  }
  pub unsafe fn with_rsfl<F: FnOnce(Rsfl) -> Rsfl>(&mut self, f: F) {
     let tmp = self.rsfl();
     self.set_rsfl(f(tmp))
  }

  pub unsafe fn rsem(&self) -> Rsem { 
     Rsem(::core::ptr::read_volatile(((self.0 as usize) + 0x194) as *const u32))
  }
  pub unsafe fn set_rsem(&mut self, value: Rsem) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x194) as *mut u32, value.0);
  }
  pub unsafe fn with_rsem<F: FnOnce(Rsem) -> Rsem>(&mut self, f: F) {
     let tmp = self.rsem();
     self.set_rsem(f(tmp))
  }

  pub unsafe fn raem(&self) -> Raem { 
     Raem(::core::ptr::read_volatile(((self.0 as usize) + 0x198) as *const u32))
  }
  pub unsafe fn set_raem(&mut self, value: Raem) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x198) as *mut u32, value.0);
  }
  pub unsafe fn with_raem<F: FnOnce(Raem) -> Raem>(&mut self, f: F) {
     let tmp = self.raem();
     self.set_raem(f(tmp))
  }

  pub unsafe fn rafl(&self) -> Rafl { 
     Rafl(::core::ptr::read_volatile(((self.0 as usize) + 0x19c) as *const u32))
  }
  pub unsafe fn set_rafl(&mut self, value: Rafl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x19c) as *mut u32, value.0);
  }
  pub unsafe fn with_rafl<F: FnOnce(Rafl) -> Rafl>(&mut self, f: F) {
     let tmp = self.rafl();
     self.set_rafl(f(tmp))
  }

  pub unsafe fn tsem(&self) -> Tsem { 
     Tsem(::core::ptr::read_volatile(((self.0 as usize) + 0x1a0) as *const u32))
  }
  pub unsafe fn set_tsem(&mut self, value: Tsem) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1a0) as *mut u32, value.0);
  }
  pub unsafe fn with_tsem<F: FnOnce(Tsem) -> Tsem>(&mut self, f: F) {
     let tmp = self.tsem();
     self.set_tsem(f(tmp))
  }

  pub unsafe fn taem(&self) -> Taem { 
     Taem(::core::ptr::read_volatile(((self.0 as usize) + 0x1a4) as *const u32))
  }
  pub unsafe fn set_taem(&mut self, value: Taem) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1a4) as *mut u32, value.0);
  }
  pub unsafe fn with_taem<F: FnOnce(Taem) -> Taem>(&mut self, f: F) {
     let tmp = self.taem();
     self.set_taem(f(tmp))
  }

  pub unsafe fn tafl(&self) -> Tafl { 
     Tafl(::core::ptr::read_volatile(((self.0 as usize) + 0x1a8) as *const u32))
  }
  pub unsafe fn set_tafl(&mut self, value: Tafl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1a8) as *mut u32, value.0);
  }
  pub unsafe fn with_tafl<F: FnOnce(Tafl) -> Tafl>(&mut self, f: F) {
     let tmp = self.tafl();
     self.set_tafl(f(tmp))
  }

  pub unsafe fn tipg(&self) -> Tipg { 
     Tipg(::core::ptr::read_volatile(((self.0 as usize) + 0x1ac) as *const u32))
  }
  pub unsafe fn set_tipg(&mut self, value: Tipg) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1ac) as *mut u32, value.0);
  }
  pub unsafe fn with_tipg<F: FnOnce(Tipg) -> Tipg>(&mut self, f: F) {
     let tmp = self.tipg();
     self.set_tipg(f(tmp))
  }

  pub unsafe fn ftrl(&self) -> Ftrl { 
     Ftrl(::core::ptr::read_volatile(((self.0 as usize) + 0x1b0) as *const u32))
  }
  pub unsafe fn set_ftrl(&mut self, value: Ftrl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1b0) as *mut u32, value.0);
  }
  pub unsafe fn with_ftrl<F: FnOnce(Ftrl) -> Ftrl>(&mut self, f: F) {
     let tmp = self.ftrl();
     self.set_ftrl(f(tmp))
  }

  pub unsafe fn tacc(&self) -> Tacc { 
     Tacc(::core::ptr::read_volatile(((self.0 as usize) + 0x1c0) as *const u32))
  }
  pub unsafe fn set_tacc(&mut self, value: Tacc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1c0) as *mut u32, value.0);
  }
  pub unsafe fn with_tacc<F: FnOnce(Tacc) -> Tacc>(&mut self, f: F) {
     let tmp = self.tacc();
     self.set_tacc(f(tmp))
  }

  pub unsafe fn racc(&self) -> Racc { 
     Racc(::core::ptr::read_volatile(((self.0 as usize) + 0x1c4) as *const u32))
  }
  pub unsafe fn set_racc(&mut self, value: Racc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1c4) as *mut u32, value.0);
  }
  pub unsafe fn with_racc<F: FnOnce(Racc) -> Racc>(&mut self, f: F) {
     let tmp = self.racc();
     self.set_racc(f(tmp))
  }

  pub unsafe fn rmon_t_packets(&self) -> RmonTPackets { 
     RmonTPackets(::core::ptr::read_volatile(((self.0 as usize) + 0x204) as *const u32))
  }

  pub unsafe fn rmon_t_bc_pkt(&self) -> RmonTBcPkt { 
     RmonTBcPkt(::core::ptr::read_volatile(((self.0 as usize) + 0x208) as *const u32))
  }

  pub unsafe fn rmon_t_mc_pkt(&self) -> RmonTMcPkt { 
     RmonTMcPkt(::core::ptr::read_volatile(((self.0 as usize) + 0x20c) as *const u32))
  }

  pub unsafe fn rmon_t_crc_align(&self) -> RmonTCrcAlign { 
     RmonTCrcAlign(::core::ptr::read_volatile(((self.0 as usize) + 0x210) as *const u32))
  }

  pub unsafe fn rmon_t_undersize(&self) -> RmonTUndersize { 
     RmonTUndersize(::core::ptr::read_volatile(((self.0 as usize) + 0x214) as *const u32))
  }

  pub unsafe fn rmon_t_oversize(&self) -> RmonTOversize { 
     RmonTOversize(::core::ptr::read_volatile(((self.0 as usize) + 0x218) as *const u32))
  }

  pub unsafe fn rmon_t_frag(&self) -> RmonTFrag { 
     RmonTFrag(::core::ptr::read_volatile(((self.0 as usize) + 0x21c) as *const u32))
  }

  pub unsafe fn rmon_t_jab(&self) -> RmonTJab { 
     RmonTJab(::core::ptr::read_volatile(((self.0 as usize) + 0x220) as *const u32))
  }

  pub unsafe fn rmon_t_col(&self) -> RmonTCol { 
     RmonTCol(::core::ptr::read_volatile(((self.0 as usize) + 0x224) as *const u32))
  }

  pub unsafe fn rmon_t_p64(&self) -> RmonTP64 { 
     RmonTP64(::core::ptr::read_volatile(((self.0 as usize) + 0x228) as *const u32))
  }

  pub unsafe fn rmon_t_p65to127(&self) -> RmonTP65to127 { 
     RmonTP65to127(::core::ptr::read_volatile(((self.0 as usize) + 0x22c) as *const u32))
  }

  pub unsafe fn rmon_t_p128to255(&self) -> RmonTP128to255 { 
     RmonTP128to255(::core::ptr::read_volatile(((self.0 as usize) + 0x230) as *const u32))
  }

  pub unsafe fn rmon_t_p256to511(&self) -> RmonTP256to511 { 
     RmonTP256to511(::core::ptr::read_volatile(((self.0 as usize) + 0x234) as *const u32))
  }

  pub unsafe fn rmon_t_p512to1023(&self) -> RmonTP512to1023 { 
     RmonTP512to1023(::core::ptr::read_volatile(((self.0 as usize) + 0x238) as *const u32))
  }

  pub unsafe fn rmon_t_p1024to2047(&self) -> RmonTP1024to2047 { 
     RmonTP1024to2047(::core::ptr::read_volatile(((self.0 as usize) + 0x23c) as *const u32))
  }

  pub unsafe fn rmon_t_p_gte2048(&self) -> RmonTPGte2048 { 
     RmonTPGte2048(::core::ptr::read_volatile(((self.0 as usize) + 0x240) as *const u32))
  }

  pub unsafe fn rmon_t_octets(&self) -> RmonTOctets { 
     RmonTOctets(::core::ptr::read_volatile(((self.0 as usize) + 0x244) as *const u32))
  }

  pub unsafe fn ieee_t_frame_ok(&self) -> IeeeTFrameOk { 
     IeeeTFrameOk(::core::ptr::read_volatile(((self.0 as usize) + 0x24c) as *const u32))
  }

  pub unsafe fn ieee_t_1col(&self) -> IeeeT1col { 
     IeeeT1col(::core::ptr::read_volatile(((self.0 as usize) + 0x250) as *const u32))
  }

  pub unsafe fn ieee_t_mcol(&self) -> IeeeTMcol { 
     IeeeTMcol(::core::ptr::read_volatile(((self.0 as usize) + 0x254) as *const u32))
  }

  pub unsafe fn ieee_t_def(&self) -> IeeeTDef { 
     IeeeTDef(::core::ptr::read_volatile(((self.0 as usize) + 0x258) as *const u32))
  }

  pub unsafe fn ieee_t_lcol(&self) -> IeeeTLcol { 
     IeeeTLcol(::core::ptr::read_volatile(((self.0 as usize) + 0x25c) as *const u32))
  }

  pub unsafe fn ieee_t_excol(&self) -> IeeeTExcol { 
     IeeeTExcol(::core::ptr::read_volatile(((self.0 as usize) + 0x260) as *const u32))
  }

  pub unsafe fn ieee_t_macerr(&self) -> IeeeTMacerr { 
     IeeeTMacerr(::core::ptr::read_volatile(((self.0 as usize) + 0x264) as *const u32))
  }

  pub unsafe fn ieee_t_cserr(&self) -> IeeeTCserr { 
     IeeeTCserr(::core::ptr::read_volatile(((self.0 as usize) + 0x268) as *const u32))
  }

  pub unsafe fn ieee_t_fdxfc(&self) -> IeeeTFdxfc { 
     IeeeTFdxfc(::core::ptr::read_volatile(((self.0 as usize) + 0x270) as *const u32))
  }

  pub unsafe fn ieee_t_octets_ok(&self) -> IeeeTOctetsOk { 
     IeeeTOctetsOk(::core::ptr::read_volatile(((self.0 as usize) + 0x274) as *const u32))
  }

  pub unsafe fn rmon_r_packets(&self) -> RmonRPackets { 
     RmonRPackets(::core::ptr::read_volatile(((self.0 as usize) + 0x284) as *const u32))
  }

  pub unsafe fn rmon_r_bc_pkt(&self) -> RmonRBcPkt { 
     RmonRBcPkt(::core::ptr::read_volatile(((self.0 as usize) + 0x288) as *const u32))
  }

  pub unsafe fn rmon_r_mc_pkt(&self) -> RmonRMcPkt { 
     RmonRMcPkt(::core::ptr::read_volatile(((self.0 as usize) + 0x28c) as *const u32))
  }

  pub unsafe fn rmon_r_crc_align(&self) -> RmonRCrcAlign { 
     RmonRCrcAlign(::core::ptr::read_volatile(((self.0 as usize) + 0x290) as *const u32))
  }

  pub unsafe fn rmon_r_undersize(&self) -> RmonRUndersize { 
     RmonRUndersize(::core::ptr::read_volatile(((self.0 as usize) + 0x294) as *const u32))
  }

  pub unsafe fn rmon_r_oversize(&self) -> RmonROversize { 
     RmonROversize(::core::ptr::read_volatile(((self.0 as usize) + 0x298) as *const u32))
  }

  pub unsafe fn rmon_r_frag(&self) -> RmonRFrag { 
     RmonRFrag(::core::ptr::read_volatile(((self.0 as usize) + 0x29c) as *const u32))
  }

  pub unsafe fn rmon_r_jab(&self) -> RmonRJab { 
     RmonRJab(::core::ptr::read_volatile(((self.0 as usize) + 0x2a0) as *const u32))
  }

  pub unsafe fn rmon_r_p64(&self) -> RmonRP64 { 
     RmonRP64(::core::ptr::read_volatile(((self.0 as usize) + 0x2a8) as *const u32))
  }

  pub unsafe fn rmon_r_p65to127(&self) -> RmonRP65to127 { 
     RmonRP65to127(::core::ptr::read_volatile(((self.0 as usize) + 0x2ac) as *const u32))
  }

  pub unsafe fn rmon_r_p128to255(&self) -> RmonRP128to255 { 
     RmonRP128to255(::core::ptr::read_volatile(((self.0 as usize) + 0x2b0) as *const u32))
  }

  pub unsafe fn rmon_r_p256to511(&self) -> RmonRP256to511 { 
     RmonRP256to511(::core::ptr::read_volatile(((self.0 as usize) + 0x2b4) as *const u32))
  }

  pub unsafe fn rmon_r_p512to1023(&self) -> RmonRP512to1023 { 
     RmonRP512to1023(::core::ptr::read_volatile(((self.0 as usize) + 0x2b8) as *const u32))
  }

  pub unsafe fn rmon_r_p1024to2047(&self) -> RmonRP1024to2047 { 
     RmonRP1024to2047(::core::ptr::read_volatile(((self.0 as usize) + 0x2bc) as *const u32))
  }

  pub unsafe fn rmon_r_p_gte2048(&self) -> RmonRPGte2048 { 
     RmonRPGte2048(::core::ptr::read_volatile(((self.0 as usize) + 0x2c0) as *const u32))
  }

  pub unsafe fn rmon_r_octets(&self) -> RmonROctets { 
     RmonROctets(::core::ptr::read_volatile(((self.0 as usize) + 0x2c4) as *const u32))
  }

  pub unsafe fn ieee_r_drop(&self) -> IeeeRDrop { 
     IeeeRDrop(::core::ptr::read_volatile(((self.0 as usize) + 0x2c8) as *const u32))
  }

  pub unsafe fn ieee_r_frame_ok(&self) -> IeeeRFrameOk { 
     IeeeRFrameOk(::core::ptr::read_volatile(((self.0 as usize) + 0x2cc) as *const u32))
  }

  pub unsafe fn ieee_r_crc(&self) -> IeeeRCrc { 
     IeeeRCrc(::core::ptr::read_volatile(((self.0 as usize) + 0x2d0) as *const u32))
  }

  pub unsafe fn ieee_r_align(&self) -> IeeeRAlign { 
     IeeeRAlign(::core::ptr::read_volatile(((self.0 as usize) + 0x2d4) as *const u32))
  }

  pub unsafe fn ieee_r_macerr(&self) -> IeeeRMacerr { 
     IeeeRMacerr(::core::ptr::read_volatile(((self.0 as usize) + 0x2d8) as *const u32))
  }

  pub unsafe fn ieee_r_fdxfc(&self) -> IeeeRFdxfc { 
     IeeeRFdxfc(::core::ptr::read_volatile(((self.0 as usize) + 0x2dc) as *const u32))
  }

  pub unsafe fn ieee_r_octets_ok(&self) -> IeeeROctetsOk { 
     IeeeROctetsOk(::core::ptr::read_volatile(((self.0 as usize) + 0x2e0) as *const u32))
  }

  pub unsafe fn atcr(&self) -> Atcr { 
     Atcr(::core::ptr::read_volatile(((self.0 as usize) + 0x400) as *const u32))
  }
  pub unsafe fn set_atcr(&mut self, value: Atcr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x400) as *mut u32, value.0);
  }
  pub unsafe fn with_atcr<F: FnOnce(Atcr) -> Atcr>(&mut self, f: F) {
     let tmp = self.atcr();
     self.set_atcr(f(tmp))
  }

  pub unsafe fn atvr(&self) -> Atvr { 
     Atvr(::core::ptr::read_volatile(((self.0 as usize) + 0x404) as *const u32))
  }
  pub unsafe fn set_atvr(&mut self, value: Atvr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x404) as *mut u32, value.0);
  }
  pub unsafe fn with_atvr<F: FnOnce(Atvr) -> Atvr>(&mut self, f: F) {
     let tmp = self.atvr();
     self.set_atvr(f(tmp))
  }

  pub unsafe fn atoff(&self) -> Atoff { 
     Atoff(::core::ptr::read_volatile(((self.0 as usize) + 0x408) as *const u32))
  }
  pub unsafe fn set_atoff(&mut self, value: Atoff) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x408) as *mut u32, value.0);
  }
  pub unsafe fn with_atoff<F: FnOnce(Atoff) -> Atoff>(&mut self, f: F) {
     let tmp = self.atoff();
     self.set_atoff(f(tmp))
  }

  pub unsafe fn atper(&self) -> Atper { 
     Atper(::core::ptr::read_volatile(((self.0 as usize) + 0x40c) as *const u32))
  }
  pub unsafe fn set_atper(&mut self, value: Atper) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x40c) as *mut u32, value.0);
  }
  pub unsafe fn with_atper<F: FnOnce(Atper) -> Atper>(&mut self, f: F) {
     let tmp = self.atper();
     self.set_atper(f(tmp))
  }

  pub unsafe fn atcor(&self) -> Atcor { 
     Atcor(::core::ptr::read_volatile(((self.0 as usize) + 0x410) as *const u32))
  }
  pub unsafe fn set_atcor(&mut self, value: Atcor) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x410) as *mut u32, value.0);
  }
  pub unsafe fn with_atcor<F: FnOnce(Atcor) -> Atcor>(&mut self, f: F) {
     let tmp = self.atcor();
     self.set_atcor(f(tmp))
  }

  pub unsafe fn atinc(&self) -> Atinc { 
     Atinc(::core::ptr::read_volatile(((self.0 as usize) + 0x414) as *const u32))
  }
  pub unsafe fn set_atinc(&mut self, value: Atinc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x414) as *mut u32, value.0);
  }
  pub unsafe fn with_atinc<F: FnOnce(Atinc) -> Atinc>(&mut self, f: F) {
     let tmp = self.atinc();
     self.set_atinc(f(tmp))
  }

  pub unsafe fn atstmp(&self) -> Atstmp { 
     Atstmp(::core::ptr::read_volatile(((self.0 as usize) + 0x418) as *const u32))
  }

  pub unsafe fn tgsr(&self) -> Tgsr { 
     Tgsr(::core::ptr::read_volatile(((self.0 as usize) + 0x604) as *const u32))
  }
  pub unsafe fn set_tgsr(&mut self, value: Tgsr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x604) as *mut u32, value.0);
  }
  pub unsafe fn with_tgsr<F: FnOnce(Tgsr) -> Tgsr>(&mut self, f: F) {
     let tmp = self.tgsr();
     self.set_tgsr(f(tmp))
  }

  pub unsafe fn tcsr(&self, index: usize) -> Tcsr { 
     assert!(index < 4);
     Tcsr(::core::ptr::read_volatile(((self.0 as usize) + 0x608 + (index << 3)) as *const u32))
  }
  pub unsafe fn set_tcsr(&mut self, index: usize, value: Tcsr) {
     assert!(index < 4);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x608 + (index << 3)) as *mut u32, value.0);
  }
  pub unsafe fn with_tcsr<F: FnOnce(Tcsr) -> Tcsr>(&mut self, index: usize, f: F) {
     let tmp = self.tcsr(index);
     self.set_tcsr(index, f(tmp))
  }

  pub unsafe fn tccr(&self, index: usize) -> Tccr { 
     assert!(index < 4);
     Tccr(::core::ptr::read_volatile(((self.0 as usize) + 0x60c + (index << 3)) as *const u32))
  }
  pub unsafe fn set_tccr(&mut self, index: usize, value: Tccr) {
     assert!(index < 4);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x60c + (index << 3)) as *mut u32, value.0);
  }
  pub unsafe fn with_tccr<F: FnOnce(Tccr) -> Tccr>(&mut self, index: usize, f: F) {
     let tmp = self.tccr(index);
     self.set_tccr(index, f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Eir(pub u32);

impl Eir {
  pub fn ts_timer(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_ts_timer(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn ts_avail(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_ts_avail(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn wakeup(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_wakeup(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn plr(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_plr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn un(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  pub fn set_un(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  pub fn rl(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_rl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn lc(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_lc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn eberr(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_eberr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn mii(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_mii(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn rxb(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_rxb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn rxf(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_rxf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn txb(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_txb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  pub fn txf(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  pub fn set_txf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  pub fn gra(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  pub fn set_gra(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn babt(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  pub fn set_babt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  pub fn babr(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_babr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

}

impl ::core::fmt::Display for Eir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Eir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ts_timer() != 0 { try!(write!(f, " ts_timer"))}
      if self.ts_avail() != 0 { try!(write!(f, " ts_avail"))}
      if self.wakeup() != 0 { try!(write!(f, " wakeup"))}
      if self.plr() != 0 { try!(write!(f, " plr"))}
      if self.un() != 0 { try!(write!(f, " un"))}
      if self.rl() != 0 { try!(write!(f, " rl"))}
      if self.lc() != 0 { try!(write!(f, " lc"))}
      if self.eberr() != 0 { try!(write!(f, " eberr"))}
      if self.mii() != 0 { try!(write!(f, " mii"))}
      if self.rxb() != 0 { try!(write!(f, " rxb"))}
      if self.rxf() != 0 { try!(write!(f, " rxf"))}
      if self.txb() != 0 { try!(write!(f, " txb"))}
      if self.txf() != 0 { try!(write!(f, " txf"))}
      if self.gra() != 0 { try!(write!(f, " gra"))}
      if self.babt() != 0 { try!(write!(f, " babt"))}
      if self.babr() != 0 { try!(write!(f, " babr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Eimr(pub u32);

impl Eimr {
  pub fn ts_timer(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_ts_timer(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn ts_avail(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_ts_avail(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn wakeup(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_wakeup(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn plr(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_plr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn un(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  pub fn set_un(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  pub fn rl(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_rl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn lc(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_lc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn eberr(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_eberr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn mii(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_mii(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn rxb(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_rxb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn rxf(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_rxf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn txb(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_txb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  pub fn txf(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  pub fn set_txf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  pub fn gra(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  pub fn set_gra(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn babt(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  pub fn set_babt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  pub fn babr(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_babr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

}

impl ::core::fmt::Display for Eimr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Eimr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ts_timer() != 0 { try!(write!(f, " ts_timer"))}
      if self.ts_avail() != 0 { try!(write!(f, " ts_avail"))}
      if self.wakeup() != 0 { try!(write!(f, " wakeup"))}
      if self.plr() != 0 { try!(write!(f, " plr"))}
      if self.un() != 0 { try!(write!(f, " un"))}
      if self.rl() != 0 { try!(write!(f, " rl"))}
      if self.lc() != 0 { try!(write!(f, " lc"))}
      if self.eberr() != 0 { try!(write!(f, " eberr"))}
      if self.mii() != 0 { try!(write!(f, " mii"))}
      if self.rxb() != 0 { try!(write!(f, " rxb"))}
      if self.rxf() != 0 { try!(write!(f, " rxf"))}
      if self.txb() != 0 { try!(write!(f, " txb"))}
      if self.txf() != 0 { try!(write!(f, " txf"))}
      if self.gra() != 0 { try!(write!(f, " gra"))}
      if self.babt() != 0 { try!(write!(f, " babt"))}
      if self.babr() != 0 { try!(write!(f, " babr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rdar(pub u32);

impl Rdar {
  pub fn rdar(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_rdar(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

}

impl ::core::fmt::Display for Rdar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rdar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rdar() != 0 { try!(write!(f, " rdar"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Tdar(pub u32);

impl Tdar {
  pub fn tdar(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_tdar(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

}

impl ::core::fmt::Display for Tdar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Tdar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tdar() != 0 { try!(write!(f, " tdar"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ecr(pub u32);

impl Ecr {
  pub fn _reset(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_reset(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn etheren(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_etheren(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn magicen(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_magicen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn sleep(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_sleep(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn en1588(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_en1588(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn dbgen(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_dbgen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn stopen(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_stopen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn dbswp(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_dbswp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

}

impl ::core::fmt::Display for Ecr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ecr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self._reset() != 0 { try!(write!(f, " _reset"))}
      if self.etheren() != 0 { try!(write!(f, " etheren"))}
      if self.magicen() != 0 { try!(write!(f, " magicen"))}
      if self.sleep() != 0 { try!(write!(f, " sleep"))}
      if self.en1588() != 0 { try!(write!(f, " en1588"))}
      if self.dbgen() != 0 { try!(write!(f, " dbgen"))}
      if self.stopen() != 0 { try!(write!(f, " stopen"))}
      if self.dbswp() != 0 { try!(write!(f, " dbswp"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Mmfr(pub u32);

impl Mmfr {
  pub fn data(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_data(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn ta(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3 // [17:16]
  }
  pub fn set_ta(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn ra(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1f // [22:18]
  }
  pub fn set_ra(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 18);
     self.0 |= value << 18;
     self
  }

  pub fn pa(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1f // [27:23]
  }
  pub fn set_pa(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 23);
     self.0 |= value << 23;
     self
  }

  pub fn op(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x3 // [29:28]
  }
  pub fn set_op(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn st(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x3 // [31:30]
  }
  pub fn set_st(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 30);
     self.0 |= value << 30;
     self
  }

}

impl ::core::fmt::Display for Mmfr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Mmfr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
      if self.ta() != 0 { try!(write!(f, " ta=0x{:x}", self.ta()))}
      if self.ra() != 0 { try!(write!(f, " ra=0x{:x}", self.ra()))}
      if self.pa() != 0 { try!(write!(f, " pa=0x{:x}", self.pa()))}
      if self.op() != 0 { try!(write!(f, " op=0x{:x}", self.op()))}
      if self.st() != 0 { try!(write!(f, " st=0x{:x}", self.st()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Mscr(pub u32);

impl Mscr {
  pub fn mii_speed(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x3f // [6:1]
  }
  pub fn set_mii_speed(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 1);
     self.0 |= value << 1;
     self
  }

  pub fn dis_pre(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_dis_pre(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn holdtime(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x7 // [10:8]
  }
  pub fn set_holdtime(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

}

impl ::core::fmt::Display for Mscr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Mscr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mii_speed() != 0 { try!(write!(f, " mii_speed=0x{:x}", self.mii_speed()))}
      if self.dis_pre() != 0 { try!(write!(f, " dis_pre"))}
      if self.holdtime() != 0 { try!(write!(f, " holdtime=0x{:x}", self.holdtime()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Mibc(pub u32);

impl Mibc {
  pub fn mib_clear(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  pub fn set_mib_clear(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  pub fn mib_idle(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_mib_idle(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn mib_dis(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_mib_dis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}

impl ::core::fmt::Display for Mibc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Mibc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mib_clear() != 0 { try!(write!(f, " mib_clear"))}
      if self.mib_idle() != 0 { try!(write!(f, " mib_idle"))}
      if self.mib_dis() != 0 { try!(write!(f, " mib_dis"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rcr(pub u32);

impl Rcr {
  pub fn _loop(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_loop(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn drt(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_drt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn mii_mode(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_mii_mode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn prom(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_prom(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn bc_rej(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_bc_rej(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn fce(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_fce(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn rmii_mode(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_rmii_mode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn rmii_10t(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_rmii_10t(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn paden(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_paden(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn paufwd(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_paufwd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn crcfwd(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_crcfwd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn cfen(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_cfen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn max_fl(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3fff // [29:16]
  }
  pub fn set_max_fl(mut self, value: u32) -> Self {
     assert!((value & !0x3fff) == 0);
     self.0 &= !(0x3fff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn nlc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_nlc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn grs(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_grs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}

impl ::core::fmt::Display for Rcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self._loop() != 0 { try!(write!(f, " _loop"))}
      if self.drt() != 0 { try!(write!(f, " drt"))}
      if self.mii_mode() != 0 { try!(write!(f, " mii_mode"))}
      if self.prom() != 0 { try!(write!(f, " prom"))}
      if self.bc_rej() != 0 { try!(write!(f, " bc_rej"))}
      if self.fce() != 0 { try!(write!(f, " fce"))}
      if self.rmii_mode() != 0 { try!(write!(f, " rmii_mode"))}
      if self.rmii_10t() != 0 { try!(write!(f, " rmii_10t"))}
      if self.paden() != 0 { try!(write!(f, " paden"))}
      if self.paufwd() != 0 { try!(write!(f, " paufwd"))}
      if self.crcfwd() != 0 { try!(write!(f, " crcfwd"))}
      if self.cfen() != 0 { try!(write!(f, " cfen"))}
      if self.max_fl() != 0 { try!(write!(f, " max_fl=0x{:x}", self.max_fl()))}
      if self.nlc() != 0 { try!(write!(f, " nlc"))}
      if self.grs() != 0 { try!(write!(f, " grs"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Tcr(pub u32);

impl Tcr {
  pub fn gts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_gts(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn fden(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_fden(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn tfc_pause(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_tfc_pause(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn rfc_pause(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_rfc_pause(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn addsel(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x7 // [7:5]
  }
  pub fn set_addsel(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn addins(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_addins(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn crcfwd(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_crcfwd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

}

impl ::core::fmt::Display for Tcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Tcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.gts() != 0 { try!(write!(f, " gts"))}
      if self.fden() != 0 { try!(write!(f, " fden"))}
      if self.tfc_pause() != 0 { try!(write!(f, " tfc_pause"))}
      if self.rfc_pause() != 0 { try!(write!(f, " rfc_pause"))}
      if self.addsel() != 0 { try!(write!(f, " addsel=0x{:x}", self.addsel()))}
      if self.addins() != 0 { try!(write!(f, " addins"))}
      if self.crcfwd() != 0 { try!(write!(f, " crcfwd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Palr(pub u32);

impl Palr {
  pub fn paddr1(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_paddr1(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Palr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Palr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Paur(pub u32);

impl Paur {
  pub fn _type(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_type(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn paddr2(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xffff // [31:16]
  }
  pub fn set_paddr2(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 16);
     self.0 |= value << 16;
     self
  }

}

impl ::core::fmt::Display for Paur {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Paur {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self._type() != 0 { try!(write!(f, " type=0x{:x}", self._type()))}
      if self.paddr2() != 0 { try!(write!(f, " paddr2=0x{:x}", self.paddr2()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Opd(pub u32);

impl Opd {
  pub fn pause_dur(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_pause_dur(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn opcode(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xffff // [31:16]
  }
  pub fn set_opcode(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 16);
     self.0 |= value << 16;
     self
  }

}

impl ::core::fmt::Display for Opd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Opd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pause_dur() != 0 { try!(write!(f, " pause_dur=0x{:x}", self.pause_dur()))}
      if self.opcode() != 0 { try!(write!(f, " opcode=0x{:x}", self.opcode()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Iaur(pub u32);

impl Iaur {
  pub fn iaddr1(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_iaddr1(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Iaur {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Iaur {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ialr(pub u32);

impl Ialr {
  pub fn iaddr2(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_iaddr2(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Ialr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ialr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Gaur(pub u32);

impl Gaur {
  pub fn gaddr1(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_gaddr1(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Gaur {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Gaur {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Galr(pub u32);

impl Galr {
  pub fn gaddr2(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_gaddr2(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Galr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Galr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Tfwr(pub u32);

impl Tfwr {
  pub fn tfwr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3f // [5:0]
  }
  pub fn set_tfwr(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 0);
     self.0 |= value << 0;
     self
  }

  pub fn strfwd(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_strfwd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

}

impl ::core::fmt::Display for Tfwr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Tfwr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tfwr() != 0 { try!(write!(f, " tfwr=0x{:x}", self.tfwr()))}
      if self.strfwd() != 0 { try!(write!(f, " strfwd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rdsr(pub u32);

impl Rdsr {
  pub fn r_des_start(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1fffffff // [31:3]
  }
  pub fn set_r_des_start(mut self, value: u32) -> Self {
     assert!((value & !0x1fffffff) == 0);
     self.0 &= !(0x1fffffff << 3);
     self.0 |= value << 3;
     self
  }

}

impl ::core::fmt::Display for Rdsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rdsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r_des_start() != 0 { try!(write!(f, " r_des_start=0x{:x}", self.r_des_start()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Tdsr(pub u32);

impl Tdsr {
  pub fn x_des_start(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1fffffff // [31:3]
  }
  pub fn set_x_des_start(mut self, value: u32) -> Self {
     assert!((value & !0x1fffffff) == 0);
     self.0 &= !(0x1fffffff << 3);
     self.0 |= value << 3;
     self
  }

}

impl ::core::fmt::Display for Tdsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Tdsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.x_des_start() != 0 { try!(write!(f, " x_des_start=0x{:x}", self.x_des_start()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Mrbr(pub u32);

impl Mrbr {
  pub fn r_buf_size(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3ff // [13:4]
  }
  pub fn set_r_buf_size(mut self, value: u32) -> Self {
     assert!((value & !0x3ff) == 0);
     self.0 &= !(0x3ff << 4);
     self.0 |= value << 4;
     self
  }

}

impl ::core::fmt::Display for Mrbr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Mrbr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r_buf_size() != 0 { try!(write!(f, " r_buf_size=0x{:x}", self.r_buf_size()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rsfl(pub u32);

impl Rsfl {
  pub fn rx_section_full(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_rx_section_full(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Rsfl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rsfl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rx_section_full() != 0 { try!(write!(f, " rx_section_full=0x{:x}", self.rx_section_full()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rsem(pub u32);

impl Rsem {
  pub fn rx_section_empty(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_rx_section_empty(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn stat_section_empty(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1f // [20:16]
  }
  pub fn set_stat_section_empty(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 16);
     self.0 |= value << 16;
     self
  }

}

impl ::core::fmt::Display for Rsem {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rsem {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rx_section_empty() != 0 { try!(write!(f, " rx_section_empty=0x{:x}", self.rx_section_empty()))}
      if self.stat_section_empty() != 0 { try!(write!(f, " stat_section_empty=0x{:x}", self.stat_section_empty()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Raem(pub u32);

impl Raem {
  pub fn rx_almost_empty(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_rx_almost_empty(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Raem {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Raem {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rx_almost_empty() != 0 { try!(write!(f, " rx_almost_empty=0x{:x}", self.rx_almost_empty()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rafl(pub u32);

impl Rafl {
  pub fn rx_almost_full(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_rx_almost_full(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Rafl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rafl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rx_almost_full() != 0 { try!(write!(f, " rx_almost_full=0x{:x}", self.rx_almost_full()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Tsem(pub u32);

impl Tsem {
  pub fn tx_section_empty(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_tx_section_empty(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Tsem {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Tsem {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tx_section_empty() != 0 { try!(write!(f, " tx_section_empty=0x{:x}", self.tx_section_empty()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Taem(pub u32);

impl Taem {
  pub fn tx_almost_empty(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_tx_almost_empty(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Taem {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Taem {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tx_almost_empty() != 0 { try!(write!(f, " tx_almost_empty=0x{:x}", self.tx_almost_empty()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Tafl(pub u32);

impl Tafl {
  pub fn tx_almost_full(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_tx_almost_full(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Tafl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Tafl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tx_almost_full() != 0 { try!(write!(f, " tx_almost_full=0x{:x}", self.tx_almost_full()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Tipg(pub u32);

impl Tipg {
  pub fn ipg(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1f // [4:0]
  }
  pub fn set_ipg(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Tipg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Tipg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ipg() != 0 { try!(write!(f, " ipg=0x{:x}", self.ipg()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ftrl(pub u32);

impl Ftrl {
  pub fn trunc_fl(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3fff // [13:0]
  }
  pub fn set_trunc_fl(mut self, value: u32) -> Self {
     assert!((value & !0x3fff) == 0);
     self.0 &= !(0x3fff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Ftrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ftrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.trunc_fl() != 0 { try!(write!(f, " trunc_fl=0x{:x}", self.trunc_fl()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Tacc(pub u32);

impl Tacc {
  pub fn shift16(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_shift16(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn ipchk(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_ipchk(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn prochk(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_prochk(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

}

impl ::core::fmt::Display for Tacc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Tacc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.shift16() != 0 { try!(write!(f, " shift16"))}
      if self.ipchk() != 0 { try!(write!(f, " ipchk"))}
      if self.prochk() != 0 { try!(write!(f, " prochk"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Racc(pub u32);

impl Racc {
  pub fn padrem(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_padrem(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn ipdis(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_ipdis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn prodis(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_prodis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn linedis(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_linedis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn shift16(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_shift16(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for Racc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Racc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.padrem() != 0 { try!(write!(f, " padrem"))}
      if self.ipdis() != 0 { try!(write!(f, " ipdis"))}
      if self.prodis() != 0 { try!(write!(f, " prodis"))}
      if self.linedis() != 0 { try!(write!(f, " linedis"))}
      if self.shift16() != 0 { try!(write!(f, " shift16"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonTPackets(pub u32);

impl RmonTPackets {
  pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonTPackets {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonTPackets {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonTBcPkt(pub u32);

impl RmonTBcPkt {
  pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonTBcPkt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonTBcPkt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonTMcPkt(pub u32);

impl RmonTMcPkt {
  pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonTMcPkt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonTMcPkt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonTCrcAlign(pub u32);

impl RmonTCrcAlign {
  pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonTCrcAlign {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonTCrcAlign {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonTUndersize(pub u32);

impl RmonTUndersize {
  pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonTUndersize {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonTUndersize {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonTOversize(pub u32);

impl RmonTOversize {
  pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonTOversize {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonTOversize {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonTFrag(pub u32);

impl RmonTFrag {
  pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonTFrag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonTFrag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonTJab(pub u32);

impl RmonTJab {
  pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonTJab {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonTJab {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonTCol(pub u32);

impl RmonTCol {
  pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonTCol {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonTCol {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonTP64(pub u32);

impl RmonTP64 {
  pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonTP64 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonTP64 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonTP65to127(pub u32);

impl RmonTP65to127 {
  pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonTP65to127 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonTP65to127 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonTP128to255(pub u32);

impl RmonTP128to255 {
  pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonTP128to255 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonTP128to255 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonTP256to511(pub u32);

impl RmonTP256to511 {
  pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonTP256to511 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonTP256to511 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonTP512to1023(pub u32);

impl RmonTP512to1023 {
  pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonTP512to1023 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonTP512to1023 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonTP1024to2047(pub u32);

impl RmonTP1024to2047 {
  pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonTP1024to2047 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonTP1024to2047 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonTPGte2048(pub u32);

impl RmonTPGte2048 {
  pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonTPGte2048 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonTPGte2048 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonTOctets(pub u32);

impl RmonTOctets {
  pub fn txocts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_txocts(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonTOctets {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonTOctets {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct IeeeTFrameOk(pub u32);

impl IeeeTFrameOk {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for IeeeTFrameOk {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for IeeeTFrameOk {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct IeeeT1col(pub u32);

impl IeeeT1col {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for IeeeT1col {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for IeeeT1col {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct IeeeTMcol(pub u32);

impl IeeeTMcol {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for IeeeTMcol {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for IeeeTMcol {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct IeeeTDef(pub u32);

impl IeeeTDef {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for IeeeTDef {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for IeeeTDef {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct IeeeTLcol(pub u32);

impl IeeeTLcol {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for IeeeTLcol {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for IeeeTLcol {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct IeeeTExcol(pub u32);

impl IeeeTExcol {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for IeeeTExcol {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for IeeeTExcol {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct IeeeTMacerr(pub u32);

impl IeeeTMacerr {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for IeeeTMacerr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for IeeeTMacerr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct IeeeTCserr(pub u32);

impl IeeeTCserr {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for IeeeTCserr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for IeeeTCserr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct IeeeTFdxfc(pub u32);

impl IeeeTFdxfc {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for IeeeTFdxfc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for IeeeTFdxfc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct IeeeTOctetsOk(pub u32);

impl IeeeTOctetsOk {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for IeeeTOctetsOk {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for IeeeTOctetsOk {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonRPackets(pub u32);

impl RmonRPackets {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonRPackets {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonRPackets {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonRBcPkt(pub u32);

impl RmonRBcPkt {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonRBcPkt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonRBcPkt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonRMcPkt(pub u32);

impl RmonRMcPkt {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonRMcPkt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonRMcPkt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonRCrcAlign(pub u32);

impl RmonRCrcAlign {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonRCrcAlign {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonRCrcAlign {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonRUndersize(pub u32);

impl RmonRUndersize {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonRUndersize {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonRUndersize {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonROversize(pub u32);

impl RmonROversize {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonROversize {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonROversize {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonRFrag(pub u32);

impl RmonRFrag {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonRFrag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonRFrag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonRJab(pub u32);

impl RmonRJab {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonRJab {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonRJab {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonRP64(pub u32);

impl RmonRP64 {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonRP64 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonRP64 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonRP65to127(pub u32);

impl RmonRP65to127 {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonRP65to127 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonRP65to127 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonRP128to255(pub u32);

impl RmonRP128to255 {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonRP128to255 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonRP128to255 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonRP256to511(pub u32);

impl RmonRP256to511 {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonRP256to511 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonRP256to511 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonRP512to1023(pub u32);

impl RmonRP512to1023 {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonRP512to1023 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonRP512to1023 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonRP1024to2047(pub u32);

impl RmonRP1024to2047 {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonRP1024to2047 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonRP1024to2047 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonRPGte2048(pub u32);

impl RmonRPGte2048 {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonRPGte2048 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonRPGte2048 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct RmonROctets(pub u32);

impl RmonROctets {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for RmonROctets {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for RmonROctets {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct IeeeRDrop(pub u32);

impl IeeeRDrop {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for IeeeRDrop {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for IeeeRDrop {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct IeeeRFrameOk(pub u32);

impl IeeeRFrameOk {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for IeeeRFrameOk {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for IeeeRFrameOk {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct IeeeRCrc(pub u32);

impl IeeeRCrc {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for IeeeRCrc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for IeeeRCrc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct IeeeRAlign(pub u32);

impl IeeeRAlign {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for IeeeRAlign {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for IeeeRAlign {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct IeeeRMacerr(pub u32);

impl IeeeRMacerr {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for IeeeRMacerr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for IeeeRMacerr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct IeeeRFdxfc(pub u32);

impl IeeeRFdxfc {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for IeeeRFdxfc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for IeeeRFdxfc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct IeeeROctetsOk(pub u32);

impl IeeeROctetsOk {
  pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for IeeeROctetsOk {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for IeeeROctetsOk {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Atcr(pub u32);

impl Atcr {
  pub fn en(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn offen(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_offen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn offrst(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_offrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn peren(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_peren(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn pinper(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_pinper(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn restart(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_restart(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn capture(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_capture(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn slave(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_slave(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

}

impl ::core::fmt::Display for Atcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Atcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.en() != 0 { try!(write!(f, " en"))}
      if self.offen() != 0 { try!(write!(f, " offen"))}
      if self.offrst() != 0 { try!(write!(f, " offrst"))}
      if self.peren() != 0 { try!(write!(f, " peren"))}
      if self.pinper() != 0 { try!(write!(f, " pinper"))}
      if self.restart() != 0 { try!(write!(f, " restart"))}
      if self.capture() != 0 { try!(write!(f, " capture"))}
      if self.slave() != 0 { try!(write!(f, " slave"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Atvr(pub u32);

impl Atvr {
  pub fn atime(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_atime(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Atvr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Atvr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Atoff(pub u32);

impl Atoff {
  pub fn offset(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_offset(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Atoff {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Atoff {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Atper(pub u32);

impl Atper {
  pub fn period(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_period(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Atper {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Atper {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Atcor(pub u32);

impl Atcor {
  pub fn cor(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7fffffff // [30:0]
  }
  pub fn set_cor(mut self, value: u32) -> Self {
     assert!((value & !0x7fffffff) == 0);
     self.0 &= !(0x7fffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Atcor {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Atcor {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cor() != 0 { try!(write!(f, " cor=0x{:x}", self.cor()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Atinc(pub u32);

impl Atinc {
  pub fn inc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7f // [6:0]
  }
  pub fn set_inc(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 0);
     self.0 |= value << 0;
     self
  }

  pub fn inc_corr(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x7f // [14:8]
  }
  pub fn set_inc_corr(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 8);
     self.0 |= value << 8;
     self
  }

}

impl ::core::fmt::Display for Atinc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Atinc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.inc() != 0 { try!(write!(f, " inc=0x{:x}", self.inc()))}
      if self.inc_corr() != 0 { try!(write!(f, " inc_corr=0x{:x}", self.inc_corr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Atstmp(pub u32);

impl Atstmp {
  pub fn timestamp(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_timestamp(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Atstmp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Atstmp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Tgsr(pub u32);

impl Tgsr {
  pub fn tf0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_tf0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn tf1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_tf1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn tf2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_tf2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn tf3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_tf3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}

impl ::core::fmt::Display for Tgsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Tgsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tf0() != 0 { try!(write!(f, " tf0"))}
      if self.tf1() != 0 { try!(write!(f, " tf1"))}
      if self.tf2() != 0 { try!(write!(f, " tf2"))}
      if self.tf3() != 0 { try!(write!(f, " tf3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Tcsr(pub u32);

impl Tcsr {
  pub fn tdre(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_tdre(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn tmode(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0xf // [5:2]
  }
  pub fn set_tmode(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 2);
     self.0 |= value << 2;
     self
  }

  pub fn tie(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_tie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn tf(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_tf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for Tcsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Tcsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tdre() != 0 { try!(write!(f, " tdre"))}
      if self.tmode() != 0 { try!(write!(f, " tmode=0x{:x}", self.tmode()))}
      if self.tie() != 0 { try!(write!(f, " tie"))}
      if self.tf() != 0 { try!(write!(f, " tf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Tccr(pub u32);

impl Tccr {
  pub fn tcc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_tcc(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Tccr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Tccr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

