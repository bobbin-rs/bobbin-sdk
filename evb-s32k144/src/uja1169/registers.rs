/// UJA1169

pub trait ReadWrite {
  fn read(&self, rw: u8) -> u8;
  fn write(&self, rw: u8, val: u8);
}

pub const REG_WDC: u8 = 0x00;
pub const REG_MC: u8 = 0x01;
pub const REG_FSC: u8 = 0x02;
pub const REG_MS: u8 = 0x03;
pub const REG_SEE: u8 = 0x04;
pub const REG_WDS: u8 = 0x05;
pub const REG_GPM: u8 = 0x06;
pub const REG_GPM1: u8 = 0x06;
pub const REG_GPM2: u8 = 0x07;
pub const REG_GPM3: u8 = 0x08;
pub const REG_GPM4: u8 = 0x09;
pub const REG_LC: u8 = 0x0a;
pub const REG_RC: u8 = 0x10;
pub const REG_SS: u8 = 0x1b;
pub const REG_SUEE: u8 = 0x1c;
pub const REG_CANC: u8 = 0x20;
pub const REG_TS: u8 = 0x22;
pub const REG_TEE: u8 = 0x23;
pub const REG_DR: u8 = 0x26;
pub const REG_ID: u8 = 0x27;
pub const REG_ID1: u8 = 0x27;
pub const REG_ID2: u8 = 0x28;
pub const REG_ID3: u8 = 0x29;
pub const REG_ID4: u8 = 0x2a;
pub const REG_M: u8 = 0x2a;
pub const REG_M1: u8 = 0x2a;
pub const REG_M2: u8 = 0x2b;
pub const REG_M3: u8 = 0x2c;
pub const REG_M4: u8 = 0x2d;
pub const REG_FC: u8 = 0x2f;
pub const REG_DM: u8 = 0x68;
pub const REG_DM1: u8 = 0x68;
pub const REG_DM2: u8 = 0x69;
pub const REG_DM3: u8 = 0x6a;
pub const REG_DM4: u8 = 0x6b;
pub const REG_DM5: u8 = 0x6c;
pub const REG_DM6: u8 = 0x6d;
pub const REG_DM7: u8 = 0x6e;
pub const REG_DM8: u8 = 0x6f;
pub const REG_WPS: u8 = 0x4b;
pub const REG_WPE: u8 = 0x4c;
pub const REG_GES: u8 = 0x60;
pub const REG_SES: u8 = 0x61;
pub const REG_SUES: u8 = 0x62;
pub const REG_TES: u8 = 0x63;
pub const REG_WPES: u8 = 0x64;
pub const REG_MTPNVS: u8 = 0x70;
pub const REG_SC: u8 = 0x73;
pub const REG_SBCCC: u8 = 0x74;
pub const REG_MTPNVCRC: u8 = 0x75;
pub const REG_IDS: u8 = 0x7e;

pub struct Registers<'a, R: 'a + ReadWrite> {
  rw: &'a R,
}

impl<'a, R: 'a + ReadWrite> Registers<'a, R> {
  pub fn new(rw: &'a R) -> Self {
    Registers { rw: rw }
  }

  pub fn wdc(&self) -> Wdc {
    Wdc(self.rw.read(REG_WDC))
  }
  pub fn set_wdc(&self, value: Wdc) {
    self.rw.write(REG_WDC, value.0)
  }
  pub fn with_wdc<F: FnOnce(Wdc) -> Wdc>(&self, f: F) {
    let tmp = Wdc(self.rw.read(REG_WDC));
    self.rw.write(REG_WDC, f(tmp).0)
  }

  pub fn mc(&self) -> Mc {
    Mc(self.rw.read(REG_MC))
  }
  pub fn set_mc(&self, value: Mc) {
    self.rw.write(REG_MC, value.0)
  }
  pub fn with_mc<F: FnOnce(Mc) -> Mc>(&self, f: F) {
    let tmp = Mc(self.rw.read(REG_MC));
    self.rw.write(REG_MC, f(tmp).0)
  }

  pub fn fsc(&self) -> Fsc {
    Fsc(self.rw.read(REG_FSC))
  }
  pub fn set_fsc(&self, value: Fsc) {
    self.rw.write(REG_FSC, value.0)
  }
  pub fn with_fsc<F: FnOnce(Fsc) -> Fsc>(&self, f: F) {
    let tmp = Fsc(self.rw.read(REG_FSC));
    self.rw.write(REG_FSC, f(tmp).0)
  }

  pub fn ms(&self) -> Ms {
    Ms(self.rw.read(REG_MS))
  }
  pub fn set_ms(&self, value: Ms) {
    self.rw.write(REG_MS, value.0)
  }
  pub fn with_ms<F: FnOnce(Ms) -> Ms>(&self, f: F) {
    let tmp = Ms(self.rw.read(REG_MS));
    self.rw.write(REG_MS, f(tmp).0)
  }

  pub fn see(&self) -> See {
    See(self.rw.read(REG_SEE))
  }
  pub fn set_see(&self, value: See) {
    self.rw.write(REG_SEE, value.0)
  }
  pub fn with_see<F: FnOnce(See) -> See>(&self, f: F) {
    let tmp = See(self.rw.read(REG_SEE));
    self.rw.write(REG_SEE, f(tmp).0)
  }

  pub fn wds(&self) -> Wds {
    Wds(self.rw.read(REG_WDS))
  }
  pub fn set_wds(&self, value: Wds) {
    self.rw.write(REG_WDS, value.0)
  }
  pub fn with_wds<F: FnOnce(Wds) -> Wds>(&self, f: F) {
    let tmp = Wds(self.rw.read(REG_WDS));
    self.rw.write(REG_WDS, f(tmp).0)
  }

  pub fn gpm(&self, index: usize) -> Gpm {
    assert!(index < 4);
    Gpm(self.rw.read(REG_GPM + index as u8))
  }
  pub fn set_gpm(&self, index: usize, value: Gpm) {
    assert!(index < 4);
    self.rw.write(REG_GPM + index as u8, value.0)
  }
  pub fn with_gpm<F: FnOnce(Gpm) -> Gpm>(&self, index: usize, f: F) {
    assert!(index < 4);
    let tmp = Gpm(self.rw.read(REG_GPM + index as u8));
    self.rw.write(REG_GPM + index as u8, f(tmp).0)
  }

  pub fn lc(&self) -> Lc {
    Lc(self.rw.read(REG_LC))
  }
  pub fn set_lc(&self, value: Lc) {
    self.rw.write(REG_LC, value.0)
  }
  pub fn with_lc<F: FnOnce(Lc) -> Lc>(&self, f: F) {
    let tmp = Lc(self.rw.read(REG_LC));
    self.rw.write(REG_LC, f(tmp).0)
  }

  pub fn rc(&self) -> Rc {
    Rc(self.rw.read(REG_RC))
  }
  pub fn set_rc(&self, value: Rc) {
    self.rw.write(REG_RC, value.0)
  }
  pub fn with_rc<F: FnOnce(Rc) -> Rc>(&self, f: F) {
    let tmp = Rc(self.rw.read(REG_RC));
    self.rw.write(REG_RC, f(tmp).0)
  }

  pub fn ss(&self) -> Ss {
    Ss(self.rw.read(REG_SS))
  }
  pub fn set_ss(&self, value: Ss) {
    self.rw.write(REG_SS, value.0)
  }
  pub fn with_ss<F: FnOnce(Ss) -> Ss>(&self, f: F) {
    let tmp = Ss(self.rw.read(REG_SS));
    self.rw.write(REG_SS, f(tmp).0)
  }

  pub fn suee(&self) -> Suee {
    Suee(self.rw.read(REG_SUEE))
  }
  pub fn set_suee(&self, value: Suee) {
    self.rw.write(REG_SUEE, value.0)
  }
  pub fn with_suee<F: FnOnce(Suee) -> Suee>(&self, f: F) {
    let tmp = Suee(self.rw.read(REG_SUEE));
    self.rw.write(REG_SUEE, f(tmp).0)
  }

  pub fn canc(&self) -> Canc {
    Canc(self.rw.read(REG_CANC))
  }
  pub fn set_canc(&self, value: Canc) {
    self.rw.write(REG_CANC, value.0)
  }
  pub fn with_canc<F: FnOnce(Canc) -> Canc>(&self, f: F) {
    let tmp = Canc(self.rw.read(REG_CANC));
    self.rw.write(REG_CANC, f(tmp).0)
  }

  pub fn ts(&self) -> Ts {
    Ts(self.rw.read(REG_TS))
  }
  pub fn set_ts(&self, value: Ts) {
    self.rw.write(REG_TS, value.0)
  }
  pub fn with_ts<F: FnOnce(Ts) -> Ts>(&self, f: F) {
    let tmp = Ts(self.rw.read(REG_TS));
    self.rw.write(REG_TS, f(tmp).0)
  }

  pub fn tee(&self) -> Tee {
    Tee(self.rw.read(REG_TEE))
  }
  pub fn set_tee(&self, value: Tee) {
    self.rw.write(REG_TEE, value.0)
  }
  pub fn with_tee<F: FnOnce(Tee) -> Tee>(&self, f: F) {
    let tmp = Tee(self.rw.read(REG_TEE));
    self.rw.write(REG_TEE, f(tmp).0)
  }

  pub fn dr(&self) -> Dr {
    Dr(self.rw.read(REG_DR))
  }
  pub fn set_dr(&self, value: Dr) {
    self.rw.write(REG_DR, value.0)
  }
  pub fn with_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) {
    let tmp = Dr(self.rw.read(REG_DR));
    self.rw.write(REG_DR, f(tmp).0)
  }

  pub fn id(&self, index: usize) -> Id {
    assert!(index < 4);
    Id(self.rw.read(REG_ID + index as u8))
  }
  pub fn set_id(&self, index: usize, value: Id) {
    assert!(index < 4);
    self.rw.write(REG_ID + index as u8, value.0)
  }
  pub fn with_id<F: FnOnce(Id) -> Id>(&self, index: usize, f: F) {
    assert!(index < 4);
    let tmp = Id(self.rw.read(REG_ID + index as u8));
    self.rw.write(REG_ID + index as u8, f(tmp).0)
  }

  pub fn m(&self, index: usize) -> M {
    assert!(index < 4);
    M(self.rw.read(REG_M + index as u8))
  }
  pub fn set_m(&self, index: usize, value: M) {
    assert!(index < 4);
    self.rw.write(REG_M + index as u8, value.0)
  }
  pub fn with_m<F: FnOnce(M) -> M>(&self, index: usize, f: F) {
    assert!(index < 4);
    let tmp = M(self.rw.read(REG_M + index as u8));
    self.rw.write(REG_M + index as u8, f(tmp).0)
  }

  pub fn fc(&self) -> Fc {
    Fc(self.rw.read(REG_FC))
  }
  pub fn set_fc(&self, value: Fc) {
    self.rw.write(REG_FC, value.0)
  }
  pub fn with_fc<F: FnOnce(Fc) -> Fc>(&self, f: F) {
    let tmp = Fc(self.rw.read(REG_FC));
    self.rw.write(REG_FC, f(tmp).0)
  }

  pub fn dm(&self, index: usize) -> Dm {
    assert!(index < 8);
    Dm(self.rw.read(REG_DM + index as u8))
  }
  pub fn set_dm(&self, index: usize, value: Dm) {
    assert!(index < 8);
    self.rw.write(REG_DM + index as u8, value.0)
  }
  pub fn with_dm<F: FnOnce(Dm) -> Dm>(&self, index: usize, f: F) {
    assert!(index < 8);
    let tmp = Dm(self.rw.read(REG_DM + index as u8));
    self.rw.write(REG_DM + index as u8, f(tmp).0)
  }

  pub fn wps(&self) -> Wps {
    Wps(self.rw.read(REG_WPS))
  }
  pub fn set_wps(&self, value: Wps) {
    self.rw.write(REG_WPS, value.0)
  }
  pub fn with_wps<F: FnOnce(Wps) -> Wps>(&self, f: F) {
    let tmp = Wps(self.rw.read(REG_WPS));
    self.rw.write(REG_WPS, f(tmp).0)
  }

  pub fn wpe(&self) -> Wpe {
    Wpe(self.rw.read(REG_WPE))
  }
  pub fn set_wpe(&self, value: Wpe) {
    self.rw.write(REG_WPE, value.0)
  }
  pub fn with_wpe<F: FnOnce(Wpe) -> Wpe>(&self, f: F) {
    let tmp = Wpe(self.rw.read(REG_WPE));
    self.rw.write(REG_WPE, f(tmp).0)
  }

  pub fn ges(&self) -> Ges {
    Ges(self.rw.read(REG_GES))
  }
  pub fn set_ges(&self, value: Ges) {
    self.rw.write(REG_GES, value.0)
  }
  pub fn with_ges<F: FnOnce(Ges) -> Ges>(&self, f: F) {
    let tmp = Ges(self.rw.read(REG_GES));
    self.rw.write(REG_GES, f(tmp).0)
  }

  pub fn ses(&self) -> Ses {
    Ses(self.rw.read(REG_SES))
  }
  pub fn set_ses(&self, value: Ses) {
    self.rw.write(REG_SES, value.0)
  }
  pub fn with_ses<F: FnOnce(Ses) -> Ses>(&self, f: F) {
    let tmp = Ses(self.rw.read(REG_SES));
    self.rw.write(REG_SES, f(tmp).0)
  }

  pub fn sues(&self) -> Sues {
    Sues(self.rw.read(REG_SUES))
  }
  pub fn set_sues(&self, value: Sues) {
    self.rw.write(REG_SUES, value.0)
  }
  pub fn with_sues<F: FnOnce(Sues) -> Sues>(&self, f: F) {
    let tmp = Sues(self.rw.read(REG_SUES));
    self.rw.write(REG_SUES, f(tmp).0)
  }

  pub fn tes(&self) -> Tes {
    Tes(self.rw.read(REG_TES))
  }
  pub fn set_tes(&self, value: Tes) {
    self.rw.write(REG_TES, value.0)
  }
  pub fn with_tes<F: FnOnce(Tes) -> Tes>(&self, f: F) {
    let tmp = Tes(self.rw.read(REG_TES));
    self.rw.write(REG_TES, f(tmp).0)
  }

  pub fn wpes(&self) -> Wpes {
    Wpes(self.rw.read(REG_WPES))
  }
  pub fn set_wpes(&self, value: Wpes) {
    self.rw.write(REG_WPES, value.0)
  }
  pub fn with_wpes<F: FnOnce(Wpes) -> Wpes>(&self, f: F) {
    let tmp = Wpes(self.rw.read(REG_WPES));
    self.rw.write(REG_WPES, f(tmp).0)
  }

  pub fn mtpnvs(&self) -> Mtpnvs {
    Mtpnvs(self.rw.read(REG_MTPNVS))
  }
  pub fn set_mtpnvs(&self, value: Mtpnvs) {
    self.rw.write(REG_MTPNVS, value.0)
  }
  pub fn with_mtpnvs<F: FnOnce(Mtpnvs) -> Mtpnvs>(&self, f: F) {
    let tmp = Mtpnvs(self.rw.read(REG_MTPNVS));
    self.rw.write(REG_MTPNVS, f(tmp).0)
  }

  pub fn sc(&self) -> Sc {
    Sc(self.rw.read(REG_SC))
  }
  pub fn set_sc(&self, value: Sc) {
    self.rw.write(REG_SC, value.0)
  }
  pub fn with_sc<F: FnOnce(Sc) -> Sc>(&self, f: F) {
    let tmp = Sc(self.rw.read(REG_SC));
    self.rw.write(REG_SC, f(tmp).0)
  }

  pub fn sbccc(&self) -> Sbccc {
    Sbccc(self.rw.read(REG_SBCCC))
  }
  pub fn set_sbccc(&self, value: Sbccc) {
    self.rw.write(REG_SBCCC, value.0)
  }
  pub fn with_sbccc<F: FnOnce(Sbccc) -> Sbccc>(&self, f: F) {
    let tmp = Sbccc(self.rw.read(REG_SBCCC));
    self.rw.write(REG_SBCCC, f(tmp).0)
  }

  pub fn mtpnvcrc(&self) -> Mtpnvcrc {
    Mtpnvcrc(self.rw.read(REG_MTPNVCRC))
  }
  pub fn set_mtpnvcrc(&self, value: Mtpnvcrc) {
    self.rw.write(REG_MTPNVCRC, value.0)
  }
  pub fn with_mtpnvcrc<F: FnOnce(Mtpnvcrc) -> Mtpnvcrc>(&self, f: F) {
    let tmp = Mtpnvcrc(self.rw.read(REG_MTPNVCRC));
    self.rw.write(REG_MTPNVCRC, f(tmp).0)
  }

  pub fn ids(&self) -> Ids {
    Ids(self.rw.read(REG_IDS))
  }
  pub fn set_ids(&self, value: Ids) {
    self.rw.write(REG_IDS, value.0)
  }
  pub fn with_ids<F: FnOnce(Ids) -> Ids>(&self, f: F) {
    let tmp = Ids(self.rw.read(REG_IDS));
    self.rw.write(REG_IDS, f(tmp).0)
  }

}

pub struct Wdc(pub u8);

impl Wdc {
  pub fn wmc(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x7 // [7:5]
  }

  pub fn set_wmc(mut self, value: u8) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn nwp(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xf // [3:0]
  }

  pub fn set_nwp(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct Mc(pub u8);

impl Mc {
  pub fn mc(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x7 // [2:0]
  }

  pub fn set_mc(mut self, value: u8) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct Fsc(pub u8);

impl Fsc {
  pub fn lhc(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }

  pub fn set_lhc(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn rcc(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x3 // [1:0]
  }

  pub fn set_rcc(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct Ms(pub u8);

impl Ms {
  pub fn otws(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }

  pub fn set_otws(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn nms(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }

  pub fn set_nms(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn rss(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1f // [4:0]
  }

  pub fn set_rss(mut self, value: u8) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct See(pub u8);

impl See {
  pub fn otwe(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }

  pub fn set_otwe(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn spife(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }

  pub fn set_spife(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

}

pub struct Wds(pub u8);

impl Wds {
  pub fn fnms(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }

  pub fn set_fnms(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn sdms(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }

  pub fn set_sdms(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn wds(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x3 // [1:0]
  }

  pub fn set_wds(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct Gpm(pub u8);

impl Gpm {
  pub fn gpm(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_gpm(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct Lc(pub u8);

impl Lc {
  pub fn lkc(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }

  pub fn set_lkc(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct Rc(pub u8);

impl Rc {
  pub fn pdc(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }

  pub fn set_pdc(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn v2c_vextc(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x3 // [3:2]
  }

  pub fn set_v2c_vextc(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn v1rtc(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x3 // [1:0]
  }

  pub fn set_v1rtc(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct Ss(pub u8);

impl Ss {
  pub fn v2s_vexts(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x3 // [2:1]
  }

  pub fn set_v2s_vexts(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn v1s(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }

  pub fn set_v1s(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct Suee(pub u8);

impl Suee {
  pub fn v2oe_vextoe(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }

  pub fn set_v2oe_vextoe(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn v2ue_vextue(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }

  pub fn set_v2ue_vextue(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn v1ue(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }

  pub fn set_v1ue(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct Canc(pub u8);

impl Canc {
  pub fn cfdc(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }

  pub fn set_cfdc(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn pncok(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }

  pub fn set_pncok(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn cpnc(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }

  pub fn set_cpnc(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn cmc(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x3 // [1:0]
  }

  pub fn set_cmc(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct Ts(pub u8);

impl Ts {
  pub fn cts(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }

  pub fn set_cts(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn cpnerr(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }

  pub fn set_cpnerr(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn cpns(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }

  pub fn set_cpns(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn coscs(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }

  pub fn set_coscs(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn cbss(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }

  pub fn set_cbss(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn vcs(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }

  pub fn set_vcs(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn cfs(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }

  pub fn set_cfs(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct Tee(pub u8);

impl Tee {
  pub fn cbse(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }

  pub fn set_cbse(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn cfe(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }

  pub fn set_cfe(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn cwe(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }

  pub fn set_cwe(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct Dr(pub u8);

impl Dr {
  pub fn cdr(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x7 // [2:0]
  }

  pub fn set_cdr(mut self, value: u8) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct Id(pub u8);

impl Id {
  pub fn id(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_id(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct M(pub u8);

impl M {
  pub fn m(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_m(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct Fc(pub u8);

impl Fc {
  pub fn ide(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }

  pub fn set_ide(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn pndm(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }

  pub fn set_pndm(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn dlc(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xf // [3:0]
  }

  pub fn set_dlc(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct Dm(pub u8);

impl Dm {
  pub fn dm(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_dm(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct Wps(pub u8);

impl Wps {
  pub fn wpvs(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }

  pub fn set_wpvs(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

}

pub struct Wpe(pub u8);

impl Wpe {
  pub fn wpre(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }

  pub fn set_wpre(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn wpfe(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }

  pub fn set_wpfe(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct Ges(pub u8);

impl Ges {
  pub fn wpe(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }

  pub fn set_wpe(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn trxe(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }

  pub fn set_trxe(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn supe(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }

  pub fn set_supe(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn syse(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }

  pub fn set_syse(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct Ses(pub u8);

impl Ses {
  pub fn po(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }

  pub fn set_po(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn otw(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }

  pub fn set_otw(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn spif(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }

  pub fn set_spif(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn wdf(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }

  pub fn set_wdf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct Sues(pub u8);

impl Sues {
  pub fn v2o_vexto(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }

  pub fn set_v2o_vexto(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn v2u_vextu(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }

  pub fn set_v2u_vextu(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn v1u(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }

  pub fn set_v1u(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct Tes(pub u8);

impl Tes {
  pub fn pnfdeo(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }

  pub fn set_pnfdeo(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn cbs(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }

  pub fn set_cbs(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn cf(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }

  pub fn set_cf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn cw(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }

  pub fn set_cw(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct Wpes(pub u8);

impl Wpes {
  pub fn wpr(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }

  pub fn set_wpr(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn wpf(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }

  pub fn set_wpf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct Mtpnvs(pub u8);

impl Mtpnvs {
  pub fn wrcnts(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x3f // [7:2]
  }

  pub fn set_wrcnts(mut self, value: u8) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 2);
     self.0 |= value << 2;
     self
  }

  pub fn eccs(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }

  pub fn set_eccs(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn nvmps(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }

  pub fn set_nvmps(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct Sc(pub u8);

impl Sc {
  pub fn rlc(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x3 // [5:4]
  }

  pub fn set_rlc(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn v2suc_vextsuc(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }

  pub fn set_v2suc_vextsuc(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}

pub struct Sbccc(pub u8);

impl Sbccc {
  pub fn v1rtsuc(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x3 // [5:4]
  }

  pub fn set_v1rtsuc(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn fnmc(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }

  pub fn set_fnmc(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn sdmc(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }

  pub fn set_sdmc(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn slpc(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }

  pub fn set_slpc(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct Mtpnvcrc(pub u8);

impl Mtpnvcrc {
  pub fn crcc(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_crcc(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

pub struct Ids(pub u8);

impl Ids {
  pub fn ids(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_ids(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

