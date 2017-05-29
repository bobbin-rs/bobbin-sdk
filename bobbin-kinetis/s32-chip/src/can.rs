pub const CAN0: Can = Can(0x40024000);
pub const CAN1: Can = Can(0x40025000);
pub const CAN2: Can = Can(0x4002b000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Can(pub u32);

impl Can {
  pub unsafe fn mcr(&self) -> Mcr { 
     Mcr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
  }
  pub unsafe fn set_mcr(&mut self, value: Mcr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
  }
  pub unsafe fn with_mcr<F: FnOnce(Mcr) -> Mcr>(&mut self, f: F) {
     let tmp = self.mcr();
     self.set_mcr(f(tmp))
  }

  pub unsafe fn ctrl1(&self) -> Ctrl1 { 
     Ctrl1(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
  }
  pub unsafe fn set_ctrl1(&mut self, value: Ctrl1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
  }
  pub unsafe fn with_ctrl1<F: FnOnce(Ctrl1) -> Ctrl1>(&mut self, f: F) {
     let tmp = self.ctrl1();
     self.set_ctrl1(f(tmp))
  }

  pub unsafe fn timer(&self) -> Timer { 
     Timer(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
  }
  pub unsafe fn set_timer(&mut self, value: Timer) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
  }
  pub unsafe fn with_timer<F: FnOnce(Timer) -> Timer>(&mut self, f: F) {
     let tmp = self.timer();
     self.set_timer(f(tmp))
  }

  pub unsafe fn rxmgmask(&self) -> Rxmgmask { 
     Rxmgmask(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
  }
  pub unsafe fn set_rxmgmask(&mut self, value: Rxmgmask) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
  }
  pub unsafe fn with_rxmgmask<F: FnOnce(Rxmgmask) -> Rxmgmask>(&mut self, f: F) {
     let tmp = self.rxmgmask();
     self.set_rxmgmask(f(tmp))
  }

  pub unsafe fn rx14mask(&self) -> Rx14mask { 
     Rx14mask(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
  }
  pub unsafe fn set_rx14mask(&mut self, value: Rx14mask) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
  }
  pub unsafe fn with_rx14mask<F: FnOnce(Rx14mask) -> Rx14mask>(&mut self, f: F) {
     let tmp = self.rx14mask();
     self.set_rx14mask(f(tmp))
  }

  pub unsafe fn rx15mask(&self) -> Rx15mask { 
     Rx15mask(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
  }
  pub unsafe fn set_rx15mask(&mut self, value: Rx15mask) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
  }
  pub unsafe fn with_rx15mask<F: FnOnce(Rx15mask) -> Rx15mask>(&mut self, f: F) {
     let tmp = self.rx15mask();
     self.set_rx15mask(f(tmp))
  }

  pub unsafe fn ecr(&self) -> Ecr { 
     Ecr(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
  }
  pub unsafe fn set_ecr(&mut self, value: Ecr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
  }
  pub unsafe fn with_ecr<F: FnOnce(Ecr) -> Ecr>(&mut self, f: F) {
     let tmp = self.ecr();
     self.set_ecr(f(tmp))
  }

  pub unsafe fn esr1(&self) -> Esr1 { 
     Esr1(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
  }
  pub unsafe fn set_esr1(&mut self, value: Esr1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
  }
  pub unsafe fn with_esr1<F: FnOnce(Esr1) -> Esr1>(&mut self, f: F) {
     let tmp = self.esr1();
     self.set_esr1(f(tmp))
  }

  pub unsafe fn imask1(&self) -> Imask1 { 
     Imask1(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
  }
  pub unsafe fn set_imask1(&mut self, value: Imask1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
  }
  pub unsafe fn with_imask1<F: FnOnce(Imask1) -> Imask1>(&mut self, f: F) {
     let tmp = self.imask1();
     self.set_imask1(f(tmp))
  }

  pub unsafe fn iflag1(&self) -> Iflag1 { 
     Iflag1(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u32))
  }
  pub unsafe fn set_iflag1(&mut self, value: Iflag1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u32, value.0);
  }
  pub unsafe fn with_iflag1<F: FnOnce(Iflag1) -> Iflag1>(&mut self, f: F) {
     let tmp = self.iflag1();
     self.set_iflag1(f(tmp))
  }

  pub unsafe fn ctrl2(&self) -> Ctrl2 { 
     Ctrl2(::core::ptr::read_volatile(((self.0 as usize) + 0x34) as *const u32))
  }
  pub unsafe fn set_ctrl2(&mut self, value: Ctrl2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x34) as *mut u32, value.0);
  }
  pub unsafe fn with_ctrl2<F: FnOnce(Ctrl2) -> Ctrl2>(&mut self, f: F) {
     let tmp = self.ctrl2();
     self.set_ctrl2(f(tmp))
  }

  pub unsafe fn esr2(&self) -> Esr2 { 
     Esr2(::core::ptr::read_volatile(((self.0 as usize) + 0x38) as *const u32))
  }

  pub unsafe fn crcr(&self) -> Crcr { 
     Crcr(::core::ptr::read_volatile(((self.0 as usize) + 0x44) as *const u32))
  }

  pub unsafe fn rxfgmask(&self) -> Rxfgmask { 
     Rxfgmask(::core::ptr::read_volatile(((self.0 as usize) + 0x48) as *const u32))
  }
  pub unsafe fn set_rxfgmask(&mut self, value: Rxfgmask) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x48) as *mut u32, value.0);
  }
  pub unsafe fn with_rxfgmask<F: FnOnce(Rxfgmask) -> Rxfgmask>(&mut self, f: F) {
     let tmp = self.rxfgmask();
     self.set_rxfgmask(f(tmp))
  }

  pub unsafe fn rxfir(&self) -> Rxfir { 
     Rxfir(::core::ptr::read_volatile(((self.0 as usize) + 0x4c) as *const u32))
  }

  pub unsafe fn cbt(&self) -> Cbt { 
     Cbt(::core::ptr::read_volatile(((self.0 as usize) + 0x50) as *const u32))
  }
  pub unsafe fn set_cbt(&mut self, value: Cbt) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x50) as *mut u32, value.0);
  }
  pub unsafe fn with_cbt<F: FnOnce(Cbt) -> Cbt>(&mut self, f: F) {
     let tmp = self.cbt();
     self.set_cbt(f(tmp))
  }

  pub unsafe fn ram(&self, index: usize) -> Ram { 
     assert!(index < 128);
     Ram(::core::ptr::read_volatile(((self.0 as usize) + 0x80 + (index << 2)) as *const u32))
  }
  pub unsafe fn set_ram(&mut self, index: usize, value: Ram) {
     assert!(index < 128);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x80 + (index << 2)) as *mut u32, value.0);
  }
  pub unsafe fn with_ram<F: FnOnce(Ram) -> Ram>(&mut self, index: usize, f: F) {
     let tmp = self.ram(index);
     self.set_ram(index, f(tmp))
  }

  pub unsafe fn mb8h0(&self, index: usize) -> Mb8h0 { 
     assert!(index < 16);
     Mb8h0(::core::ptr::read_volatile(((self.0 as usize) + 0x80 + (index << 4)) as *const u32))
  }
  pub unsafe fn set_mb8h0(&mut self, index: usize, value: Mb8h0) {
     assert!(index < 16);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x80 + (index << 4)) as *mut u32, value.0);
  }
  pub unsafe fn with_mb8h0<F: FnOnce(Mb8h0) -> Mb8h0>(&mut self, index: usize, f: F) {
     let tmp = self.mb8h0(index);
     self.set_mb8h0(index, f(tmp))
  }

  pub unsafe fn mb8h1(&self, index: usize) -> Mb8h1 { 
     assert!(index < 16);
     Mb8h1(::core::ptr::read_volatile(((self.0 as usize) + 0x84 + (index << 4)) as *const u32))
  }
  pub unsafe fn set_mb8h1(&mut self, index: usize, value: Mb8h1) {
     assert!(index < 16);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x84 + (index << 4)) as *mut u32, value.0);
  }
  pub unsafe fn with_mb8h1<F: FnOnce(Mb8h1) -> Mb8h1>(&mut self, index: usize, f: F) {
     let tmp = self.mb8h1(index);
     self.set_mb8h1(index, f(tmp))
  }

  pub unsafe fn mb8d0(&self, index: usize) -> Mb8d0 { 
     assert!(index < 16);
     Mb8d0(::core::ptr::read_volatile(((self.0 as usize) + 0x88 + (index << 4)) as *const u32))
  }
  pub unsafe fn set_mb8d0(&mut self, index: usize, value: Mb8d0) {
     assert!(index < 16);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x88 + (index << 4)) as *mut u32, value.0);
  }
  pub unsafe fn with_mb8d0<F: FnOnce(Mb8d0) -> Mb8d0>(&mut self, index: usize, f: F) {
     let tmp = self.mb8d0(index);
     self.set_mb8d0(index, f(tmp))
  }

  pub unsafe fn mb8d1(&self, index: usize) -> Mb8d1 { 
     assert!(index < 16);
     Mb8d1(::core::ptr::read_volatile(((self.0 as usize) + 0x8c + (index << 4)) as *const u32))
  }
  pub unsafe fn set_mb8d1(&mut self, index: usize, value: Mb8d1) {
     assert!(index < 16);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8c + (index << 4)) as *mut u32, value.0);
  }
  pub unsafe fn with_mb8d1<F: FnOnce(Mb8d1) -> Mb8d1>(&mut self, index: usize, f: F) {
     let tmp = self.mb8d1(index);
     self.set_mb8d1(index, f(tmp))
  }

  pub unsafe fn rximr(&self, index: usize) -> Rximr { 
     assert!(index < 16);
     Rximr(::core::ptr::read_volatile(((self.0 as usize) + 0x880 + (index << 2)) as *const u32))
  }
  pub unsafe fn set_rximr(&mut self, index: usize, value: Rximr) {
     assert!(index < 16);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x880 + (index << 2)) as *mut u32, value.0);
  }
  pub unsafe fn with_rximr<F: FnOnce(Rximr) -> Rximr>(&mut self, index: usize, f: F) {
     let tmp = self.rximr(index);
     self.set_rximr(index, f(tmp))
  }

  pub unsafe fn ctrl1_pn(&self) -> Ctrl1Pn { 
     Ctrl1Pn(::core::ptr::read_volatile(((self.0 as usize) + 0xb00) as *const u32))
  }
  pub unsafe fn set_ctrl1_pn(&mut self, value: Ctrl1Pn) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xb00) as *mut u32, value.0);
  }
  pub unsafe fn with_ctrl1_pn<F: FnOnce(Ctrl1Pn) -> Ctrl1Pn>(&mut self, f: F) {
     let tmp = self.ctrl1_pn();
     self.set_ctrl1_pn(f(tmp))
  }

  pub unsafe fn ctrl2_pn(&self) -> Ctrl2Pn { 
     Ctrl2Pn(::core::ptr::read_volatile(((self.0 as usize) + 0xb04) as *const u32))
  }
  pub unsafe fn set_ctrl2_pn(&mut self, value: Ctrl2Pn) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xb04) as *mut u32, value.0);
  }
  pub unsafe fn with_ctrl2_pn<F: FnOnce(Ctrl2Pn) -> Ctrl2Pn>(&mut self, f: F) {
     let tmp = self.ctrl2_pn();
     self.set_ctrl2_pn(f(tmp))
  }

  pub unsafe fn wu_mtc(&self) -> WuMtc { 
     WuMtc(::core::ptr::read_volatile(((self.0 as usize) + 0xb08) as *const u32))
  }
  pub unsafe fn set_wu_mtc(&mut self, value: WuMtc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xb08) as *mut u32, value.0);
  }
  pub unsafe fn with_wu_mtc<F: FnOnce(WuMtc) -> WuMtc>(&mut self, f: F) {
     let tmp = self.wu_mtc();
     self.set_wu_mtc(f(tmp))
  }

  pub unsafe fn flt_id1(&self) -> FltId1 { 
     FltId1(::core::ptr::read_volatile(((self.0 as usize) + 0xb0c) as *const u32))
  }
  pub unsafe fn set_flt_id1(&mut self, value: FltId1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xb0c) as *mut u32, value.0);
  }
  pub unsafe fn with_flt_id1<F: FnOnce(FltId1) -> FltId1>(&mut self, f: F) {
     let tmp = self.flt_id1();
     self.set_flt_id1(f(tmp))
  }

  pub unsafe fn flt_dlc(&self) -> FltDlc { 
     FltDlc(::core::ptr::read_volatile(((self.0 as usize) + 0xb10) as *const u32))
  }
  pub unsafe fn set_flt_dlc(&mut self, value: FltDlc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xb10) as *mut u32, value.0);
  }
  pub unsafe fn with_flt_dlc<F: FnOnce(FltDlc) -> FltDlc>(&mut self, f: F) {
     let tmp = self.flt_dlc();
     self.set_flt_dlc(f(tmp))
  }

  pub unsafe fn pl1_lo(&self) -> Pl1Lo { 
     Pl1Lo(::core::ptr::read_volatile(((self.0 as usize) + 0xb14) as *const u32))
  }
  pub unsafe fn set_pl1_lo(&mut self, value: Pl1Lo) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xb14) as *mut u32, value.0);
  }
  pub unsafe fn with_pl1_lo<F: FnOnce(Pl1Lo) -> Pl1Lo>(&mut self, f: F) {
     let tmp = self.pl1_lo();
     self.set_pl1_lo(f(tmp))
  }

  pub unsafe fn pl1_hi(&self) -> Pl1Hi { 
     Pl1Hi(::core::ptr::read_volatile(((self.0 as usize) + 0xb18) as *const u32))
  }
  pub unsafe fn set_pl1_hi(&mut self, value: Pl1Hi) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xb18) as *mut u32, value.0);
  }
  pub unsafe fn with_pl1_hi<F: FnOnce(Pl1Hi) -> Pl1Hi>(&mut self, f: F) {
     let tmp = self.pl1_hi();
     self.set_pl1_hi(f(tmp))
  }

  pub unsafe fn flt_id2_idmask(&self) -> FltId2Idmask { 
     FltId2Idmask(::core::ptr::read_volatile(((self.0 as usize) + 0xb1c) as *const u32))
  }
  pub unsafe fn set_flt_id2_idmask(&mut self, value: FltId2Idmask) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xb1c) as *mut u32, value.0);
  }
  pub unsafe fn with_flt_id2_idmask<F: FnOnce(FltId2Idmask) -> FltId2Idmask>(&mut self, f: F) {
     let tmp = self.flt_id2_idmask();
     self.set_flt_id2_idmask(f(tmp))
  }

  pub unsafe fn pl2_plmask_lo(&self) -> Pl2PlmaskLo { 
     Pl2PlmaskLo(::core::ptr::read_volatile(((self.0 as usize) + 0xb20) as *const u32))
  }
  pub unsafe fn set_pl2_plmask_lo(&mut self, value: Pl2PlmaskLo) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xb20) as *mut u32, value.0);
  }
  pub unsafe fn with_pl2_plmask_lo<F: FnOnce(Pl2PlmaskLo) -> Pl2PlmaskLo>(&mut self, f: F) {
     let tmp = self.pl2_plmask_lo();
     self.set_pl2_plmask_lo(f(tmp))
  }

  pub unsafe fn pl2_plmask_hi(&self) -> Pl2PlmaskHi { 
     Pl2PlmaskHi(::core::ptr::read_volatile(((self.0 as usize) + 0xb24) as *const u32))
  }
  pub unsafe fn set_pl2_plmask_hi(&mut self, value: Pl2PlmaskHi) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xb24) as *mut u32, value.0);
  }
  pub unsafe fn with_pl2_plmask_hi<F: FnOnce(Pl2PlmaskHi) -> Pl2PlmaskHi>(&mut self, f: F) {
     let tmp = self.pl2_plmask_hi();
     self.set_pl2_plmask_hi(f(tmp))
  }

  pub unsafe fn wmb0_cs(&self) -> Wmb0Cs { 
     Wmb0Cs(::core::ptr::read_volatile(((self.0 as usize) + 0xb40) as *const u32))
  }

  pub unsafe fn wmb0_id(&self) -> Wmb0Id { 
     Wmb0Id(::core::ptr::read_volatile(((self.0 as usize) + 0xb44) as *const u32))
  }

  pub unsafe fn wmb0_d03(&self) -> Wmb0D03 { 
     Wmb0D03(::core::ptr::read_volatile(((self.0 as usize) + 0xb48) as *const u32))
  }

  pub unsafe fn wmb0_d47(&self) -> Wmb0D47 { 
     Wmb0D47(::core::ptr::read_volatile(((self.0 as usize) + 0xb4c) as *const u32))
  }

  pub unsafe fn wmb1_cs(&self) -> Wmb1Cs { 
     Wmb1Cs(::core::ptr::read_volatile(((self.0 as usize) + 0xb50) as *const u32))
  }

  pub unsafe fn wmb1_id(&self) -> Wmb1Id { 
     Wmb1Id(::core::ptr::read_volatile(((self.0 as usize) + 0xb54) as *const u32))
  }

  pub unsafe fn wmb1_d03(&self) -> Wmb1D03 { 
     Wmb1D03(::core::ptr::read_volatile(((self.0 as usize) + 0xb58) as *const u32))
  }

  pub unsafe fn wmb1_d47(&self) -> Wmb1D47 { 
     Wmb1D47(::core::ptr::read_volatile(((self.0 as usize) + 0xb5c) as *const u32))
  }

  pub unsafe fn wmb2_cs(&self) -> Wmb2Cs { 
     Wmb2Cs(::core::ptr::read_volatile(((self.0 as usize) + 0xb60) as *const u32))
  }

  pub unsafe fn wmb2_id(&self) -> Wmb2Id { 
     Wmb2Id(::core::ptr::read_volatile(((self.0 as usize) + 0xb64) as *const u32))
  }

  pub unsafe fn wmb2_d03(&self) -> Wmb2D03 { 
     Wmb2D03(::core::ptr::read_volatile(((self.0 as usize) + 0xb68) as *const u32))
  }

  pub unsafe fn wmb2_d47(&self) -> Wmb2D47 { 
     Wmb2D47(::core::ptr::read_volatile(((self.0 as usize) + 0xb6c) as *const u32))
  }

  pub unsafe fn wmb3_cs(&self) -> Wmb3Cs { 
     Wmb3Cs(::core::ptr::read_volatile(((self.0 as usize) + 0xb70) as *const u32))
  }

  pub unsafe fn wmb3_id(&self) -> Wmb3Id { 
     Wmb3Id(::core::ptr::read_volatile(((self.0 as usize) + 0xb74) as *const u32))
  }

  pub unsafe fn wmb3_d03(&self) -> Wmb3D03 { 
     Wmb3D03(::core::ptr::read_volatile(((self.0 as usize) + 0xb78) as *const u32))
  }

  pub unsafe fn wmb3_d47(&self) -> Wmb3D47 { 
     Wmb3D47(::core::ptr::read_volatile(((self.0 as usize) + 0xb7c) as *const u32))
  }

  pub unsafe fn fdctrl(&self) -> Fdctrl { 
     Fdctrl(::core::ptr::read_volatile(((self.0 as usize) + 0xc00) as *const u32))
  }
  pub unsafe fn set_fdctrl(&mut self, value: Fdctrl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc00) as *mut u32, value.0);
  }
  pub unsafe fn with_fdctrl<F: FnOnce(Fdctrl) -> Fdctrl>(&mut self, f: F) {
     let tmp = self.fdctrl();
     self.set_fdctrl(f(tmp))
  }

  pub unsafe fn fdcbt(&self) -> Fdcbt { 
     Fdcbt(::core::ptr::read_volatile(((self.0 as usize) + 0xc04) as *const u32))
  }
  pub unsafe fn set_fdcbt(&mut self, value: Fdcbt) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc04) as *mut u32, value.0);
  }
  pub unsafe fn with_fdcbt<F: FnOnce(Fdcbt) -> Fdcbt>(&mut self, f: F) {
     let tmp = self.fdcbt();
     self.set_fdcbt(f(tmp))
  }

  pub unsafe fn fdcrc(&self) -> Fdcrc { 
     Fdcrc(::core::ptr::read_volatile(((self.0 as usize) + 0xc08) as *const u32))
  }

}

#[derive(PartialEq, Eq)]
pub struct Mcr(pub u32);

impl Mcr {
  pub fn maxmb(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7f // [6:0]
  }
  pub fn set_maxmb(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 0);
     self.0 |= value << 0;
     self
  }

  pub fn idam(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  pub fn set_idam(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn fden(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_fden(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn aen(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_aen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn lprioen(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_lprioen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn pnet_en(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_pnet_en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn dma(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_dma(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn irmq(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_irmq(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn srxdis(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_srxdis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn lpmack(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_lpmack(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn wrnen(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_wrnen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn supv(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_supv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn frzack(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_frzack(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn softrst(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_softrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn notrdy(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  pub fn set_notrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  pub fn halt(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  pub fn set_halt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn rfen(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  pub fn set_rfen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  pub fn frz(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_frz(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn mdis(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_mdis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}

impl ::core::fmt::Display for Mcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Mcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.maxmb() != 0 { try!(write!(f, " maxmb=0x{:x}", self.maxmb()))}
      if self.idam() != 0 { try!(write!(f, " idam=0x{:x}", self.idam()))}
      if self.fden() != 0 { try!(write!(f, " fden"))}
      if self.aen() != 0 { try!(write!(f, " aen"))}
      if self.lprioen() != 0 { try!(write!(f, " lprioen"))}
      if self.pnet_en() != 0 { try!(write!(f, " pnet_en"))}
      if self.dma() != 0 { try!(write!(f, " dma"))}
      if self.irmq() != 0 { try!(write!(f, " irmq"))}
      if self.srxdis() != 0 { try!(write!(f, " srxdis"))}
      if self.lpmack() != 0 { try!(write!(f, " lpmack"))}
      if self.wrnen() != 0 { try!(write!(f, " wrnen"))}
      if self.supv() != 0 { try!(write!(f, " supv"))}
      if self.frzack() != 0 { try!(write!(f, " frzack"))}
      if self.softrst() != 0 { try!(write!(f, " softrst"))}
      if self.notrdy() != 0 { try!(write!(f, " notrdy"))}
      if self.halt() != 0 { try!(write!(f, " halt"))}
      if self.rfen() != 0 { try!(write!(f, " rfen"))}
      if self.frz() != 0 { try!(write!(f, " frz"))}
      if self.mdis() != 0 { try!(write!(f, " mdis"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ctrl1(pub u32);

impl Ctrl1 {
  pub fn propseg(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
  pub fn set_propseg(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn lom(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_lom(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn lbuf(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_lbuf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn tsyn(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_tsyn(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn boffrec(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_boffrec(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn smp(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_smp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn rwrnmsk(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_rwrnmsk(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn twrnmsk(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_twrnmsk(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn lpb(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_lpb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn clksrc(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_clksrc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn errmsk(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_errmsk(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn boffmsk(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_boffmsk(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn pseg2(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x7 // [18:16]
  }
  pub fn set_pseg2(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn pseg1(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x7 // [21:19]
  }
  pub fn set_pseg1(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 19);
     self.0 |= value << 19;
     self
  }

  pub fn rjw(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x3 // [23:22]
  }
  pub fn set_rjw(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn presdiv(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
  pub fn set_presdiv(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
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
      if self.propseg() != 0 { try!(write!(f, " propseg=0x{:x}", self.propseg()))}
      if self.lom() != 0 { try!(write!(f, " lom"))}
      if self.lbuf() != 0 { try!(write!(f, " lbuf"))}
      if self.tsyn() != 0 { try!(write!(f, " tsyn"))}
      if self.boffrec() != 0 { try!(write!(f, " boffrec"))}
      if self.smp() != 0 { try!(write!(f, " smp"))}
      if self.rwrnmsk() != 0 { try!(write!(f, " rwrnmsk"))}
      if self.twrnmsk() != 0 { try!(write!(f, " twrnmsk"))}
      if self.lpb() != 0 { try!(write!(f, " lpb"))}
      if self.clksrc() != 0 { try!(write!(f, " clksrc"))}
      if self.errmsk() != 0 { try!(write!(f, " errmsk"))}
      if self.boffmsk() != 0 { try!(write!(f, " boffmsk"))}
      if self.pseg2() != 0 { try!(write!(f, " pseg2=0x{:x}", self.pseg2()))}
      if self.pseg1() != 0 { try!(write!(f, " pseg1=0x{:x}", self.pseg1()))}
      if self.rjw() != 0 { try!(write!(f, " rjw=0x{:x}", self.rjw()))}
      if self.presdiv() != 0 { try!(write!(f, " presdiv=0x{:x}", self.presdiv()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Timer(pub u32);

impl Timer {
  pub fn timer(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_timer(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Timer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Timer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.timer() != 0 { try!(write!(f, " timer=0x{:x}", self.timer()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rxmgmask(pub u32);

impl Rxmgmask {
  pub fn mg(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_mg(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Rxmgmask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rxmgmask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rx14mask(pub u32);

impl Rx14mask {
  pub fn rx14m(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_rx14m(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Rx14mask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rx14mask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rx15mask(pub u32);

impl Rx15mask {
  pub fn rx15m(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_rx15m(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Rx15mask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rx15mask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ecr(pub u32);

impl Ecr {
  pub fn txerrcnt(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_txerrcnt(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn rxerrcnt(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
  pub fn set_rxerrcnt(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

  pub fn txerrcnt_fast(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  pub fn set_txerrcnt_fast(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn rxerrcnt_fast(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
  pub fn set_rxerrcnt_fast(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
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
      if self.txerrcnt() != 0 { try!(write!(f, " txerrcnt=0x{:x}", self.txerrcnt()))}
      if self.rxerrcnt() != 0 { try!(write!(f, " rxerrcnt=0x{:x}", self.rxerrcnt()))}
      if self.txerrcnt_fast() != 0 { try!(write!(f, " txerrcnt_fast=0x{:x}", self.txerrcnt_fast()))}
      if self.rxerrcnt_fast() != 0 { try!(write!(f, " rxerrcnt_fast=0x{:x}", self.rxerrcnt_fast()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Esr1(pub u32);

impl Esr1 {
  pub fn errint(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_errint(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn boffint(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_boffint(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn rx(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_rx(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn fltconf(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  pub fn set_fltconf(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn tx(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_tx(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn idle(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_idle(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn rxwrn(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_rxwrn(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn txwrn(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_txwrn(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn stferr(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_stferr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn frmerr(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_frmerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn crcerr(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_crcerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn ackerr(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_ackerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn bit0err(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_bit0err(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn bit1err(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_bit1err(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn rwrnint(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_rwrnint(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn twrnint(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_twrnint(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn synch(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_synch(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn boffdoneint(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  pub fn set_boffdoneint(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  pub fn errint_fast(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_errint_fast(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn errovr(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_errovr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn stferr_fast(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_stferr_fast(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  pub fn frmerr_fast(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  pub fn set_frmerr_fast(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  pub fn crcerr_fast(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  pub fn set_crcerr_fast(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn bit0err_fast(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_bit0err_fast(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn bit1err_fast(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_bit1err_fast(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}

impl ::core::fmt::Display for Esr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Esr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.errint() != 0 { try!(write!(f, " errint"))}
      if self.boffint() != 0 { try!(write!(f, " boffint"))}
      if self.rx() != 0 { try!(write!(f, " rx"))}
      if self.fltconf() != 0 { try!(write!(f, " fltconf=0x{:x}", self.fltconf()))}
      if self.tx() != 0 { try!(write!(f, " tx"))}
      if self.idle() != 0 { try!(write!(f, " idle"))}
      if self.rxwrn() != 0 { try!(write!(f, " rxwrn"))}
      if self.txwrn() != 0 { try!(write!(f, " txwrn"))}
      if self.stferr() != 0 { try!(write!(f, " stferr"))}
      if self.frmerr() != 0 { try!(write!(f, " frmerr"))}
      if self.crcerr() != 0 { try!(write!(f, " crcerr"))}
      if self.ackerr() != 0 { try!(write!(f, " ackerr"))}
      if self.bit0err() != 0 { try!(write!(f, " bit0err"))}
      if self.bit1err() != 0 { try!(write!(f, " bit1err"))}
      if self.rwrnint() != 0 { try!(write!(f, " rwrnint"))}
      if self.twrnint() != 0 { try!(write!(f, " twrnint"))}
      if self.synch() != 0 { try!(write!(f, " synch"))}
      if self.boffdoneint() != 0 { try!(write!(f, " boffdoneint"))}
      if self.errint_fast() != 0 { try!(write!(f, " errint_fast"))}
      if self.errovr() != 0 { try!(write!(f, " errovr"))}
      if self.stferr_fast() != 0 { try!(write!(f, " stferr_fast"))}
      if self.frmerr_fast() != 0 { try!(write!(f, " frmerr_fast"))}
      if self.crcerr_fast() != 0 { try!(write!(f, " crcerr_fast"))}
      if self.bit0err_fast() != 0 { try!(write!(f, " bit0err_fast"))}
      if self.bit1err_fast() != 0 { try!(write!(f, " bit1err_fast"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Imask1(pub u32);

impl Imask1 {
  pub fn buf31to0m(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_buf31to0m(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Imask1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Imask1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Iflag1(pub u32);

impl Iflag1 {
  pub fn bufi(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_bufi(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Iflag1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Iflag1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.bufi(0) != 0 { try!(write!(f, " bufi[0]"))}
      if self.bufi(1) != 0 { try!(write!(f, " bufi[1]"))}
      if self.bufi(2) != 0 { try!(write!(f, " bufi[2]"))}
      if self.bufi(3) != 0 { try!(write!(f, " bufi[3]"))}
      if self.bufi(4) != 0 { try!(write!(f, " bufi[4]"))}
      if self.bufi(5) != 0 { try!(write!(f, " bufi[5]"))}
      if self.bufi(6) != 0 { try!(write!(f, " bufi[6]"))}
      if self.bufi(7) != 0 { try!(write!(f, " bufi[7]"))}
      if self.bufi(8) != 0 { try!(write!(f, " bufi[8]"))}
      if self.bufi(9) != 0 { try!(write!(f, " bufi[9]"))}
      if self.bufi(10) != 0 { try!(write!(f, " bufi[10]"))}
      if self.bufi(11) != 0 { try!(write!(f, " bufi[11]"))}
      if self.bufi(12) != 0 { try!(write!(f, " bufi[12]"))}
      if self.bufi(13) != 0 { try!(write!(f, " bufi[13]"))}
      if self.bufi(14) != 0 { try!(write!(f, " bufi[14]"))}
      if self.bufi(15) != 0 { try!(write!(f, " bufi[15]"))}
      if self.bufi(16) != 0 { try!(write!(f, " bufi[16]"))}
      if self.bufi(17) != 0 { try!(write!(f, " bufi[17]"))}
      if self.bufi(18) != 0 { try!(write!(f, " bufi[18]"))}
      if self.bufi(19) != 0 { try!(write!(f, " bufi[19]"))}
      if self.bufi(20) != 0 { try!(write!(f, " bufi[20]"))}
      if self.bufi(21) != 0 { try!(write!(f, " bufi[21]"))}
      if self.bufi(22) != 0 { try!(write!(f, " bufi[22]"))}
      if self.bufi(23) != 0 { try!(write!(f, " bufi[23]"))}
      if self.bufi(24) != 0 { try!(write!(f, " bufi[24]"))}
      if self.bufi(25) != 0 { try!(write!(f, " bufi[25]"))}
      if self.bufi(26) != 0 { try!(write!(f, " bufi[26]"))}
      if self.bufi(27) != 0 { try!(write!(f, " bufi[27]"))}
      if self.bufi(28) != 0 { try!(write!(f, " bufi[28]"))}
      if self.bufi(29) != 0 { try!(write!(f, " bufi[29]"))}
      if self.bufi(30) != 0 { try!(write!(f, " bufi[30]"))}
      if self.bufi(31) != 0 { try!(write!(f, " bufi[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ctrl2(pub u32);

impl Ctrl2 {
  pub fn edfltdis(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_edfltdis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn isocanfden(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_isocanfden(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn prexcen(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_prexcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn timer_src(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_timer_src(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn eacen(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_eacen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn rrs(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_rrs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn mrp(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_mrp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn tasd(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1f // [23:19]
  }
  pub fn set_tasd(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 19);
     self.0 |= value << 19;
     self
  }

  pub fn rffn(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  pub fn set_rffn(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

  pub fn boffdonemsk(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_boffdonemsk(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn errmsk_fast(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_errmsk_fast(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
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
      if self.edfltdis() != 0 { try!(write!(f, " edfltdis"))}
      if self.isocanfden() != 0 { try!(write!(f, " isocanfden"))}
      if self.prexcen() != 0 { try!(write!(f, " prexcen"))}
      if self.timer_src() != 0 { try!(write!(f, " timer_src"))}
      if self.eacen() != 0 { try!(write!(f, " eacen"))}
      if self.rrs() != 0 { try!(write!(f, " rrs"))}
      if self.mrp() != 0 { try!(write!(f, " mrp"))}
      if self.tasd() != 0 { try!(write!(f, " tasd=0x{:x}", self.tasd()))}
      if self.rffn() != 0 { try!(write!(f, " rffn=0x{:x}", self.rffn()))}
      if self.boffdonemsk() != 0 { try!(write!(f, " boffdonemsk"))}
      if self.errmsk_fast() != 0 { try!(write!(f, " errmsk_fast"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Esr2(pub u32);

impl Esr2 {
  pub fn imb(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_imb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn vps(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_vps(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn lptm(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x7f // [22:16]
  }
  pub fn set_lptm(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 16);
     self.0 |= value << 16;
     self
  }

}

impl ::core::fmt::Display for Esr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Esr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.imb() != 0 { try!(write!(f, " imb"))}
      if self.vps() != 0 { try!(write!(f, " vps"))}
      if self.lptm() != 0 { try!(write!(f, " lptm=0x{:x}", self.lptm()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Crcr(pub u32);

impl Crcr {
  pub fn txcrc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7fff // [14:0]
  }
  pub fn set_txcrc(mut self, value: u32) -> Self {
     assert!((value & !0x7fff) == 0);
     self.0 &= !(0x7fff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn mbcrc(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x7f // [22:16]
  }
  pub fn set_mbcrc(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 16);
     self.0 |= value << 16;
     self
  }

}

impl ::core::fmt::Display for Crcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Crcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txcrc() != 0 { try!(write!(f, " txcrc=0x{:x}", self.txcrc()))}
      if self.mbcrc() != 0 { try!(write!(f, " mbcrc=0x{:x}", self.mbcrc()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rxfgmask(pub u32);

impl Rxfgmask {
  pub fn fgm(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_fgm(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Rxfgmask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rxfgmask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rxfir(pub u32);

impl Rxfir {
  pub fn idhit(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1ff // [8:0]
  }
  pub fn set_idhit(mut self, value: u32) -> Self {
     assert!((value & !0x1ff) == 0);
     self.0 &= !(0x1ff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Rxfir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rxfir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.idhit() != 0 { try!(write!(f, " idhit=0x{:x}", self.idhit()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Cbt(pub u32);

impl Cbt {
  pub fn epseg2(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1f // [4:0]
  }
  pub fn set_epseg2(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

  pub fn epseg1(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1f // [9:5]
  }
  pub fn set_epseg1(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 5);
     self.0 |= value << 5;
     self
  }

  pub fn epropseg(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3f // [15:10]
  }
  pub fn set_epropseg(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 10);
     self.0 |= value << 10;
     self
  }

  pub fn erjw(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1f // [20:16]
  }
  pub fn set_erjw(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 16);
     self.0 |= value << 16;
     self
  }

  pub fn epresdiv(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x3ff // [30:21]
  }
  pub fn set_epresdiv(mut self, value: u32) -> Self {
     assert!((value & !0x3ff) == 0);
     self.0 &= !(0x3ff << 21);
     self.0 |= value << 21;
     self
  }

  pub fn btf(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_btf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}

impl ::core::fmt::Display for Cbt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Cbt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.epseg2() != 0 { try!(write!(f, " epseg2=0x{:x}", self.epseg2()))}
      if self.epseg1() != 0 { try!(write!(f, " epseg1=0x{:x}", self.epseg1()))}
      if self.epropseg() != 0 { try!(write!(f, " epropseg=0x{:x}", self.epropseg()))}
      if self.erjw() != 0 { try!(write!(f, " erjw=0x{:x}", self.erjw()))}
      if self.epresdiv() != 0 { try!(write!(f, " epresdiv=0x{:x}", self.epresdiv()))}
      if self.btf() != 0 { try!(write!(f, " btf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ram(pub u32);

impl Ram {
  pub fn byte(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 0 + (index << 3);
     ((self.0 as u32) >> shift) & 0xff // [7:0]
  }
  pub fn set_byte(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0xff) == 0);
     let shift: usize = 0 + (index << 3);
     self.0 &= !(0xff << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Ram {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ram {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.byte(0) != 0 { try!(write!(f, " byte[0]=0x{:x}", self.byte(0)))}
      if self.byte(1) != 0 { try!(write!(f, " byte[1]=0x{:x}", self.byte(1)))}
      if self.byte(2) != 0 { try!(write!(f, " byte[2]=0x{:x}", self.byte(2)))}
      if self.byte(3) != 0 { try!(write!(f, " byte[3]=0x{:x}", self.byte(3)))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Mb8h0(pub u32);

impl Mb8h0 {
  pub fn edl(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_edl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

  pub fn brs(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_brs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn esi(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  pub fn set_esi(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  pub fn code(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  pub fn set_code(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

  pub fn srr(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_srr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn ide(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_ide(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn rtr(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_rtr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn dlc(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  pub fn set_dlc(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  pub fn time_stamp(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_time_stamp(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Mb8h0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Mb8h0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.edl() != 0 { try!(write!(f, " edl"))}
      if self.brs() != 0 { try!(write!(f, " brs"))}
      if self.esi() != 0 { try!(write!(f, " esi"))}
      if self.code() != 0 { try!(write!(f, " code=0x{:x}", self.code()))}
      if self.srr() != 0 { try!(write!(f, " srr"))}
      if self.ide() != 0 { try!(write!(f, " ide"))}
      if self.rtr() != 0 { try!(write!(f, " rtr"))}
      if self.dlc() != 0 { try!(write!(f, " dlc=0x{:x}", self.dlc()))}
      if self.time_stamp() != 0 { try!(write!(f, " time_stamp=0x{:x}", self.time_stamp()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Mb8h1(pub u32);

impl Mb8h1 {
  pub fn prio(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x7 // [31:29]
  }
  pub fn set_prio(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 29);
     self.0 |= value << 29;
     self
  }

  pub fn id_std(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x7ff // [28:18]
  }
  pub fn set_id_std(mut self, value: u32) -> Self {
     assert!((value & !0x7ff) == 0);
     self.0 &= !(0x7ff << 18);
     self.0 |= value << 18;
     self
  }

  pub fn id_ext(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1fffffff // [28:0]
  }
  pub fn set_id_ext(mut self, value: u32) -> Self {
     assert!((value & !0x1fffffff) == 0);
     self.0 &= !(0x1fffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Mb8h1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Mb8h1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.prio() != 0 { try!(write!(f, " prio=0x{:x}", self.prio()))}
      if self.id_std() != 0 { try!(write!(f, " id_std=0x{:x}", self.id_std()))}
      if self.id_ext() != 0 { try!(write!(f, " id_ext=0x{:x}", self.id_ext()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Mb8d0(pub u32);

impl Mb8d0 {
  pub fn byte(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0xff // [7:0]
  }
  pub fn set_byte(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0xff) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0xff << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Mb8d0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Mb8d0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.byte(0) != 0 { try!(write!(f, " byte[0]=0x{:x}", self.byte(0)))}
      if self.byte(1) != 0 { try!(write!(f, " byte[1]=0x{:x}", self.byte(1)))}
      if self.byte(2) != 0 { try!(write!(f, " byte[2]=0x{:x}", self.byte(2)))}
      if self.byte(3) != 0 { try!(write!(f, " byte[3]=0x{:x}", self.byte(3)))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Mb8d1(pub u32);

impl Mb8d1 {
  pub fn byte(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0xff // [7:0]
  }
  pub fn set_byte(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0xff) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0xff << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Mb8d1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Mb8d1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.byte(0) != 0 { try!(write!(f, " byte[0]=0x{:x}", self.byte(0)))}
      if self.byte(1) != 0 { try!(write!(f, " byte[1]=0x{:x}", self.byte(1)))}
      if self.byte(2) != 0 { try!(write!(f, " byte[2]=0x{:x}", self.byte(2)))}
      if self.byte(3) != 0 { try!(write!(f, " byte[3]=0x{:x}", self.byte(3)))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rximr(pub u32);

impl Rximr {
  pub fn mi(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_mi(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Rximr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rximr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ctrl1Pn(pub u32);

impl Ctrl1Pn {
  pub fn fcs(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  pub fn set_fcs(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn idfs(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
  pub fn set_idfs(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn plfs(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  pub fn set_plfs(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn nmatch(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
  pub fn set_nmatch(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

  pub fn wumf_msk(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_wumf_msk(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn wtof_msk(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_wtof_msk(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

}

impl ::core::fmt::Display for Ctrl1Pn {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ctrl1Pn {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fcs() != 0 { try!(write!(f, " fcs=0x{:x}", self.fcs()))}
      if self.idfs() != 0 { try!(write!(f, " idfs=0x{:x}", self.idfs()))}
      if self.plfs() != 0 { try!(write!(f, " plfs=0x{:x}", self.plfs()))}
      if self.nmatch() != 0 { try!(write!(f, " nmatch=0x{:x}", self.nmatch()))}
      if self.wumf_msk() != 0 { try!(write!(f, " wumf_msk"))}
      if self.wtof_msk() != 0 { try!(write!(f, " wtof_msk"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ctrl2Pn(pub u32);

impl Ctrl2Pn {
  pub fn matchto(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_matchto(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Ctrl2Pn {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ctrl2Pn {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.matchto() != 0 { try!(write!(f, " matchto=0x{:x}", self.matchto()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct WuMtc(pub u32);

impl WuMtc {
  pub fn mcounter(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
  pub fn set_mcounter(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

  pub fn wumf(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_wumf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn wtof(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_wtof(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

}

impl ::core::fmt::Display for WuMtc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for WuMtc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mcounter() != 0 { try!(write!(f, " mcounter=0x{:x}", self.mcounter()))}
      if self.wumf() != 0 { try!(write!(f, " wumf"))}
      if self.wtof() != 0 { try!(write!(f, " wtof"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct FltId1(pub u32);

impl FltId1 {
  pub fn flt_id1(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1fffffff // [28:0]
  }
  pub fn set_flt_id1(mut self, value: u32) -> Self {
     assert!((value & !0x1fffffff) == 0);
     self.0 &= !(0x1fffffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn flt_rtr(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  pub fn set_flt_rtr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  pub fn flt_ide(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_flt_ide(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

}

impl ::core::fmt::Display for FltId1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for FltId1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.flt_id1() != 0 { try!(write!(f, " flt_id1=0x{:x}", self.flt_id1()))}
      if self.flt_rtr() != 0 { try!(write!(f, " flt_rtr"))}
      if self.flt_ide() != 0 { try!(write!(f, " flt_ide"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct FltDlc(pub u32);

impl FltDlc {
  pub fn flt_dlc_hi(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  pub fn set_flt_dlc_hi(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  pub fn flt_dlc_lo(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  pub fn set_flt_dlc_lo(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

}

impl ::core::fmt::Display for FltDlc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for FltDlc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.flt_dlc_hi() != 0 { try!(write!(f, " flt_dlc_hi=0x{:x}", self.flt_dlc_hi()))}
      if self.flt_dlc_lo() != 0 { try!(write!(f, " flt_dlc_lo=0x{:x}", self.flt_dlc_lo()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Pl1Lo(pub u32);

impl Pl1Lo {
  pub fn data_byte_3(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_data_byte_3(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn data_byte_2(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
  pub fn set_data_byte_2(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

  pub fn data_byte_1(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  pub fn set_data_byte_1(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn data_byte_0(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
  pub fn set_data_byte_0(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

}

impl ::core::fmt::Display for Pl1Lo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Pl1Lo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data_byte_3() != 0 { try!(write!(f, " data_byte_3=0x{:x}", self.data_byte_3()))}
      if self.data_byte_2() != 0 { try!(write!(f, " data_byte_2=0x{:x}", self.data_byte_2()))}
      if self.data_byte_1() != 0 { try!(write!(f, " data_byte_1=0x{:x}", self.data_byte_1()))}
      if self.data_byte_0() != 0 { try!(write!(f, " data_byte_0=0x{:x}", self.data_byte_0()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Pl1Hi(pub u32);

impl Pl1Hi {
  pub fn data_byte_7(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_data_byte_7(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn data_byte_6(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
  pub fn set_data_byte_6(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

  pub fn data_byte_5(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  pub fn set_data_byte_5(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn data_byte_4(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
  pub fn set_data_byte_4(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

}

impl ::core::fmt::Display for Pl1Hi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Pl1Hi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data_byte_7() != 0 { try!(write!(f, " data_byte_7=0x{:x}", self.data_byte_7()))}
      if self.data_byte_6() != 0 { try!(write!(f, " data_byte_6=0x{:x}", self.data_byte_6()))}
      if self.data_byte_5() != 0 { try!(write!(f, " data_byte_5=0x{:x}", self.data_byte_5()))}
      if self.data_byte_4() != 0 { try!(write!(f, " data_byte_4=0x{:x}", self.data_byte_4()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct FltId2Idmask(pub u32);

impl FltId2Idmask {
  pub fn flt_id2_idmask(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1fffffff // [28:0]
  }
  pub fn set_flt_id2_idmask(mut self, value: u32) -> Self {
     assert!((value & !0x1fffffff) == 0);
     self.0 &= !(0x1fffffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn rtr_msk(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  pub fn set_rtr_msk(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  pub fn ide_msk(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_ide_msk(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

}

impl ::core::fmt::Display for FltId2Idmask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for FltId2Idmask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.flt_id2_idmask() != 0 { try!(write!(f, " flt_id2_idmask=0x{:x}", self.flt_id2_idmask()))}
      if self.rtr_msk() != 0 { try!(write!(f, " rtr_msk"))}
      if self.ide_msk() != 0 { try!(write!(f, " ide_msk"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Pl2PlmaskLo(pub u32);

impl Pl2PlmaskLo {
  pub fn data_byte_3(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_data_byte_3(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn data_byte_2(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
  pub fn set_data_byte_2(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

  pub fn data_byte_1(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  pub fn set_data_byte_1(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn data_byte_0(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
  pub fn set_data_byte_0(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

}

impl ::core::fmt::Display for Pl2PlmaskLo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Pl2PlmaskLo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data_byte_3() != 0 { try!(write!(f, " data_byte_3=0x{:x}", self.data_byte_3()))}
      if self.data_byte_2() != 0 { try!(write!(f, " data_byte_2=0x{:x}", self.data_byte_2()))}
      if self.data_byte_1() != 0 { try!(write!(f, " data_byte_1=0x{:x}", self.data_byte_1()))}
      if self.data_byte_0() != 0 { try!(write!(f, " data_byte_0=0x{:x}", self.data_byte_0()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Pl2PlmaskHi(pub u32);

impl Pl2PlmaskHi {
  pub fn data_byte_7(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_data_byte_7(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn data_byte_6(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
  pub fn set_data_byte_6(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

  pub fn data_byte_5(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  pub fn set_data_byte_5(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn data_byte_4(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
  pub fn set_data_byte_4(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

}

impl ::core::fmt::Display for Pl2PlmaskHi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Pl2PlmaskHi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data_byte_7() != 0 { try!(write!(f, " data_byte_7=0x{:x}", self.data_byte_7()))}
      if self.data_byte_6() != 0 { try!(write!(f, " data_byte_6=0x{:x}", self.data_byte_6()))}
      if self.data_byte_5() != 0 { try!(write!(f, " data_byte_5=0x{:x}", self.data_byte_5()))}
      if self.data_byte_4() != 0 { try!(write!(f, " data_byte_4=0x{:x}", self.data_byte_4()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Wmb0Cs(pub u32);

impl Wmb0Cs {
  pub fn dlc(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  pub fn set_dlc(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  pub fn rtr(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_rtr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn ide(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_ide(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn srr(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_srr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

}

impl ::core::fmt::Display for Wmb0Cs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Wmb0Cs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dlc() != 0 { try!(write!(f, " dlc=0x{:x}", self.dlc()))}
      if self.rtr() != 0 { try!(write!(f, " rtr"))}
      if self.ide() != 0 { try!(write!(f, " ide"))}
      if self.srr() != 0 { try!(write!(f, " srr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Wmb0Id(pub u32);

impl Wmb0Id {
  pub fn id(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1fffffff // [28:0]
  }
  pub fn set_id(mut self, value: u32) -> Self {
     assert!((value & !0x1fffffff) == 0);
     self.0 &= !(0x1fffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Wmb0Id {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Wmb0Id {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Wmb0D03(pub u32);

impl Wmb0D03 {
  pub fn data_byte_3(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_data_byte_3(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn data_byte_2(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
  pub fn set_data_byte_2(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

  pub fn data_byte_1(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  pub fn set_data_byte_1(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn data_byte_0(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
  pub fn set_data_byte_0(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

}

impl ::core::fmt::Display for Wmb0D03 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Wmb0D03 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data_byte_3() != 0 { try!(write!(f, " data_byte_3=0x{:x}", self.data_byte_3()))}
      if self.data_byte_2() != 0 { try!(write!(f, " data_byte_2=0x{:x}", self.data_byte_2()))}
      if self.data_byte_1() != 0 { try!(write!(f, " data_byte_1=0x{:x}", self.data_byte_1()))}
      if self.data_byte_0() != 0 { try!(write!(f, " data_byte_0=0x{:x}", self.data_byte_0()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Wmb0D47(pub u32);

impl Wmb0D47 {
  pub fn data_byte_7(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_data_byte_7(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn data_byte_6(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
  pub fn set_data_byte_6(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

  pub fn data_byte_5(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  pub fn set_data_byte_5(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn data_byte_4(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
  pub fn set_data_byte_4(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

}

impl ::core::fmt::Display for Wmb0D47 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Wmb0D47 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data_byte_7() != 0 { try!(write!(f, " data_byte_7=0x{:x}", self.data_byte_7()))}
      if self.data_byte_6() != 0 { try!(write!(f, " data_byte_6=0x{:x}", self.data_byte_6()))}
      if self.data_byte_5() != 0 { try!(write!(f, " data_byte_5=0x{:x}", self.data_byte_5()))}
      if self.data_byte_4() != 0 { try!(write!(f, " data_byte_4=0x{:x}", self.data_byte_4()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Wmb1Cs(pub u32);

impl Wmb1Cs {
  pub fn dlc(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  pub fn set_dlc(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  pub fn rtr(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_rtr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn ide(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_ide(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn srr(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_srr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

}

impl ::core::fmt::Display for Wmb1Cs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Wmb1Cs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dlc() != 0 { try!(write!(f, " dlc=0x{:x}", self.dlc()))}
      if self.rtr() != 0 { try!(write!(f, " rtr"))}
      if self.ide() != 0 { try!(write!(f, " ide"))}
      if self.srr() != 0 { try!(write!(f, " srr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Wmb1Id(pub u32);

impl Wmb1Id {
  pub fn id(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1fffffff // [28:0]
  }
  pub fn set_id(mut self, value: u32) -> Self {
     assert!((value & !0x1fffffff) == 0);
     self.0 &= !(0x1fffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Wmb1Id {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Wmb1Id {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Wmb1D03(pub u32);

impl Wmb1D03 {
  pub fn data_byte_3(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_data_byte_3(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn data_byte_2(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
  pub fn set_data_byte_2(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

  pub fn data_byte_1(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  pub fn set_data_byte_1(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn data_byte_0(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
  pub fn set_data_byte_0(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

}

impl ::core::fmt::Display for Wmb1D03 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Wmb1D03 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data_byte_3() != 0 { try!(write!(f, " data_byte_3=0x{:x}", self.data_byte_3()))}
      if self.data_byte_2() != 0 { try!(write!(f, " data_byte_2=0x{:x}", self.data_byte_2()))}
      if self.data_byte_1() != 0 { try!(write!(f, " data_byte_1=0x{:x}", self.data_byte_1()))}
      if self.data_byte_0() != 0 { try!(write!(f, " data_byte_0=0x{:x}", self.data_byte_0()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Wmb1D47(pub u32);

impl Wmb1D47 {
  pub fn data_byte_7(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_data_byte_7(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn data_byte_6(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
  pub fn set_data_byte_6(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

  pub fn data_byte_5(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  pub fn set_data_byte_5(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn data_byte_4(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
  pub fn set_data_byte_4(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

}

impl ::core::fmt::Display for Wmb1D47 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Wmb1D47 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data_byte_7() != 0 { try!(write!(f, " data_byte_7=0x{:x}", self.data_byte_7()))}
      if self.data_byte_6() != 0 { try!(write!(f, " data_byte_6=0x{:x}", self.data_byte_6()))}
      if self.data_byte_5() != 0 { try!(write!(f, " data_byte_5=0x{:x}", self.data_byte_5()))}
      if self.data_byte_4() != 0 { try!(write!(f, " data_byte_4=0x{:x}", self.data_byte_4()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Wmb2Cs(pub u32);

impl Wmb2Cs {
  pub fn dlc(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  pub fn set_dlc(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  pub fn rtr(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_rtr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn ide(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_ide(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn srr(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_srr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

}

impl ::core::fmt::Display for Wmb2Cs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Wmb2Cs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dlc() != 0 { try!(write!(f, " dlc=0x{:x}", self.dlc()))}
      if self.rtr() != 0 { try!(write!(f, " rtr"))}
      if self.ide() != 0 { try!(write!(f, " ide"))}
      if self.srr() != 0 { try!(write!(f, " srr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Wmb2Id(pub u32);

impl Wmb2Id {
  pub fn id(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1fffffff // [28:0]
  }
  pub fn set_id(mut self, value: u32) -> Self {
     assert!((value & !0x1fffffff) == 0);
     self.0 &= !(0x1fffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Wmb2Id {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Wmb2Id {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Wmb2D03(pub u32);

impl Wmb2D03 {
  pub fn data_byte_3(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_data_byte_3(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn data_byte_2(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
  pub fn set_data_byte_2(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

  pub fn data_byte_1(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  pub fn set_data_byte_1(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn data_byte_0(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
  pub fn set_data_byte_0(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

}

impl ::core::fmt::Display for Wmb2D03 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Wmb2D03 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data_byte_3() != 0 { try!(write!(f, " data_byte_3=0x{:x}", self.data_byte_3()))}
      if self.data_byte_2() != 0 { try!(write!(f, " data_byte_2=0x{:x}", self.data_byte_2()))}
      if self.data_byte_1() != 0 { try!(write!(f, " data_byte_1=0x{:x}", self.data_byte_1()))}
      if self.data_byte_0() != 0 { try!(write!(f, " data_byte_0=0x{:x}", self.data_byte_0()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Wmb2D47(pub u32);

impl Wmb2D47 {
  pub fn data_byte_7(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_data_byte_7(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn data_byte_6(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
  pub fn set_data_byte_6(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

  pub fn data_byte_5(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  pub fn set_data_byte_5(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn data_byte_4(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
  pub fn set_data_byte_4(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

}

impl ::core::fmt::Display for Wmb2D47 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Wmb2D47 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data_byte_7() != 0 { try!(write!(f, " data_byte_7=0x{:x}", self.data_byte_7()))}
      if self.data_byte_6() != 0 { try!(write!(f, " data_byte_6=0x{:x}", self.data_byte_6()))}
      if self.data_byte_5() != 0 { try!(write!(f, " data_byte_5=0x{:x}", self.data_byte_5()))}
      if self.data_byte_4() != 0 { try!(write!(f, " data_byte_4=0x{:x}", self.data_byte_4()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Wmb3Cs(pub u32);

impl Wmb3Cs {
  pub fn dlc(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  pub fn set_dlc(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  pub fn rtr(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_rtr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn ide(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_ide(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn srr(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_srr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

}

impl ::core::fmt::Display for Wmb3Cs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Wmb3Cs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dlc() != 0 { try!(write!(f, " dlc=0x{:x}", self.dlc()))}
      if self.rtr() != 0 { try!(write!(f, " rtr"))}
      if self.ide() != 0 { try!(write!(f, " ide"))}
      if self.srr() != 0 { try!(write!(f, " srr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Wmb3Id(pub u32);

impl Wmb3Id {
  pub fn id(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1fffffff // [28:0]
  }
  pub fn set_id(mut self, value: u32) -> Self {
     assert!((value & !0x1fffffff) == 0);
     self.0 &= !(0x1fffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Wmb3Id {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Wmb3Id {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Wmb3D03(pub u32);

impl Wmb3D03 {
  pub fn data_byte_3(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_data_byte_3(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn data_byte_2(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
  pub fn set_data_byte_2(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

  pub fn data_byte_1(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  pub fn set_data_byte_1(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn data_byte_0(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
  pub fn set_data_byte_0(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

}

impl ::core::fmt::Display for Wmb3D03 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Wmb3D03 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data_byte_3() != 0 { try!(write!(f, " data_byte_3=0x{:x}", self.data_byte_3()))}
      if self.data_byte_2() != 0 { try!(write!(f, " data_byte_2=0x{:x}", self.data_byte_2()))}
      if self.data_byte_1() != 0 { try!(write!(f, " data_byte_1=0x{:x}", self.data_byte_1()))}
      if self.data_byte_0() != 0 { try!(write!(f, " data_byte_0=0x{:x}", self.data_byte_0()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Wmb3D47(pub u32);

impl Wmb3D47 {
  pub fn data_byte_7(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_data_byte_7(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn data_byte_6(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
  pub fn set_data_byte_6(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

  pub fn data_byte_5(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  pub fn set_data_byte_5(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn data_byte_4(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
  pub fn set_data_byte_4(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

}

impl ::core::fmt::Display for Wmb3D47 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Wmb3D47 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data_byte_7() != 0 { try!(write!(f, " data_byte_7=0x{:x}", self.data_byte_7()))}
      if self.data_byte_6() != 0 { try!(write!(f, " data_byte_6=0x{:x}", self.data_byte_6()))}
      if self.data_byte_5() != 0 { try!(write!(f, " data_byte_5=0x{:x}", self.data_byte_5()))}
      if self.data_byte_4() != 0 { try!(write!(f, " data_byte_4=0x{:x}", self.data_byte_4()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Fdctrl(pub u32);

impl Fdctrl {
  pub fn tdcval(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3f // [5:0]
  }
  pub fn set_tdcval(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 0);
     self.0 |= value << 0;
     self
  }

  pub fn tdcoff(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1f // [12:8]
  }
  pub fn set_tdcoff(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 8);
     self.0 |= value << 8;
     self
  }

  pub fn tdcfail(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_tdcfail(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn tdcen(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_tdcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn mbdsr0(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3 // [17:16]
  }
  pub fn set_mbdsr0(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn fdrate(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_fdrate(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}

impl ::core::fmt::Display for Fdctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Fdctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tdcval() != 0 { try!(write!(f, " tdcval=0x{:x}", self.tdcval()))}
      if self.tdcoff() != 0 { try!(write!(f, " tdcoff=0x{:x}", self.tdcoff()))}
      if self.tdcfail() != 0 { try!(write!(f, " tdcfail"))}
      if self.tdcen() != 0 { try!(write!(f, " tdcen"))}
      if self.mbdsr0() != 0 { try!(write!(f, " mbdsr0=0x{:x}", self.mbdsr0()))}
      if self.fdrate() != 0 { try!(write!(f, " fdrate"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Fdcbt(pub u32);

impl Fdcbt {
  pub fn fpseg2(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
  pub fn set_fpseg2(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn fpseg1(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x7 // [7:5]
  }
  pub fn set_fpseg1(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn fpropseg(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1f // [14:10]
  }
  pub fn set_fpropseg(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 10);
     self.0 |= value << 10;
     self
  }

  pub fn frjw(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x7 // [18:16]
  }
  pub fn set_frjw(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn fpresdiv(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x3ff // [29:20]
  }
  pub fn set_fpresdiv(mut self, value: u32) -> Self {
     assert!((value & !0x3ff) == 0);
     self.0 &= !(0x3ff << 20);
     self.0 |= value << 20;
     self
  }

}

impl ::core::fmt::Display for Fdcbt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Fdcbt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fpseg2() != 0 { try!(write!(f, " fpseg2=0x{:x}", self.fpseg2()))}
      if self.fpseg1() != 0 { try!(write!(f, " fpseg1=0x{:x}", self.fpseg1()))}
      if self.fpropseg() != 0 { try!(write!(f, " fpropseg=0x{:x}", self.fpropseg()))}
      if self.frjw() != 0 { try!(write!(f, " frjw=0x{:x}", self.frjw()))}
      if self.fpresdiv() != 0 { try!(write!(f, " fpresdiv=0x{:x}", self.fpresdiv()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Fdcrc(pub u32);

impl Fdcrc {
  pub fn fd_txcrc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1fffff // [20:0]
  }
  pub fn set_fd_txcrc(mut self, value: u32) -> Self {
     assert!((value & !0x1fffff) == 0);
     self.0 &= !(0x1fffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn fd_mbcrc(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7f // [30:24]
  }
  pub fn set_fd_mbcrc(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 24);
     self.0 |= value << 24;
     self
  }

}

impl ::core::fmt::Display for Fdcrc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Fdcrc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fd_txcrc() != 0 { try!(write!(f, " fd_txcrc=0x{:x}", self.fd_txcrc()))}
      if self.fd_mbcrc() != 0 { try!(write!(f, " fd_mbcrc=0x{:x}", self.fd_mbcrc()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

