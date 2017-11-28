pub trait ReadWrite {
  fn read(&self, rw: u8) -> u8;
  fn write(&self, rw: u8, val: u8);
}

pub const REG_XG_OFFS_TC_H: u8 = 0x04;
pub const REG_XG_OFFS_TC_L: u8 = 0x05;
pub const REG_YG_OFFS_TC_H: u8 = 0x07;
pub const REG_YG_OFFS_TC_L: u8 = 0x08;
pub const REG_ZG_OFFS_TC_H: u8 = 0x0a;
pub const REG_ZG_OFFS_TC_L: u8 = 0x0b;
pub const REG_SELF_TEST_X_ACCEL: u8 = 0x0d;
pub const REG_SELF_TEST_Y_ACCEL: u8 = 0x0e;
pub const REG_SELF_TEST_Z_ACCEL: u8 = 0x0f;
pub const REG_XG_OFFS_USRH: u8 = 0x13;
pub const REG_XG_OFFS_USRL: u8 = 0x14;
pub const REG_YG_OFFS_USRH: u8 = 0x15;
pub const REG_YG_OFFS_USRL: u8 = 0x16;
pub const REG_ZG_OFFS_USRH: u8 = 0x17;
pub const REG_ZG_OFFS_USRL: u8 = 0x18;
pub const REG_SMPLRT_DIV: u8 = 0x19;
pub const REG_CONFIG: u8 = 0x1a;
pub const REG_GYRO_CONFIG: u8 = 0x1b;
pub const REG_ACCEL_CONFIG: u8 = 0x1c;
pub const REG_ACCEL_CONFIG_2: u8 = 0x1d;
pub const REG_LP_MODE_CFG: u8 = 0x1e;
pub const REG_ACCEL_WOM_X_THR: u8 = 0x20;
pub const REG_ACCEL_WOM_Y_THR: u8 = 0x21;
pub const REG_ACCEL_WOM_Z_THR: u8 = 0x22;
pub const REG_FIFO_EN: u8 = 0x23;
pub const REG_FSYNC_INT: u8 = 0x36;
pub const REG_INT_PIN_CFG: u8 = 0x37;
pub const REG_INT_ENABLE: u8 = 0x38;
pub const REG_FIFO_WM_INT_STATUS: u8 = 0x39;
pub const REG_INT_STATUS: u8 = 0x3a;
pub const REG_ACCEL_XOUT_H: u8 = 0x3b;
pub const REG_ACCEL_XOUT_L: u8 = 0x3c;
pub const REG_ACCEL_YOUT_H: u8 = 0x3d;
pub const REG_ACCEL_YOUT_L: u8 = 0x3e;
pub const REG_ACCEL_ZOUT_H: u8 = 0x3f;
pub const REG_ACCEL_ZOUT_L: u8 = 0x40;
pub const REG_TEMP_OUT_H: u8 = 0x41;
pub const REG_TEMP_OUT_L: u8 = 0x42;
pub const REG_GYRO_XOUT_H: u8 = 0x43;
pub const REG_GYRO_XOUT_L: u8 = 0x44;
pub const REG_GYRO_YOUT_H: u8 = 0x45;
pub const REG_GYRO_YOUT_L: u8 = 0x46;
pub const REG_GYRO_ZOUT_H: u8 = 0x47;
pub const REG_GYRO_ZOUT_L: u8 = 0x48;
pub const REG_SELF_TEST_X_GYRO: u8 = 0x50;
pub const REG_SELF_TEST_Y_GYRO: u8 = 0x51;
pub const REG_SELF_TEST_Z_GYRO: u8 = 0x52;
pub const REG_FIFO_WM_TH1: u8 = 0x60;
pub const REG_FIFO_WM_TH2: u8 = 0x61;
pub const REG_SIGNAL_PATH_RESET: u8 = 0x68;
pub const REG_ACCEL_INTEL_CTRL: u8 = 0x69;
pub const REG_USER_CTRL: u8 = 0x6a;
pub const REG_PWR_MGMT_1: u8 = 0x6b;
pub const REG_PWR_MGMT_2: u8 = 0x6c;
pub const REG_I2C_IF: u8 = 0x70;
pub const REG_FIFO_COUNTH: u8 = 0x72;
pub const REG_FIFO_COUNTL: u8 = 0x73;
pub const REG_FIFO_R_W: u8 = 0x74;
pub const REG_WHO_AM_I: u8 = 0x75;
pub const REG_XA_OFFSET_H: u8 = 0x77;
pub const REG_XA_OFFSET_L: u8 = 0x78;
pub const REG_YA_OFFSET_H: u8 = 0x7a;
pub const REG_YA_OFFSET_L: u8 = 0x7b;
pub const REG_ZA_OFFSET_H: u8 = 0x7d;
pub const REG_ZA_OFFSET_L: u8 = 0x7e;

pub struct Registers<'a, R: 'a + ReadWrite> {
  rw: &'a R,
}

impl<'a, R: 'a + ReadWrite> Registers<'a, R> {
  pub fn new(rw: &'a R) -> Self {
    Registers { rw: rw }
  }

  pub fn xg_offs_tc_h(&self) -> XgOffsTcH {
    XgOffsTcH(self.rw.read(REG_XG_OFFS_TC_H))
  }
  pub fn set_xg_offs_tc_h(&self, value: XgOffsTcH) {
    self.rw.write(REG_XG_OFFS_TC_H, value.0)
  }
  pub fn with_xg_offs_tc_h<F: FnOnce(XgOffsTcH) -> XgOffsTcH>(&self, f: F) {
    let tmp = XgOffsTcH(self.rw.read(REG_XG_OFFS_TC_H));
    self.rw.write(REG_XG_OFFS_TC_H, f(tmp).0)
  }

  pub fn xg_offs_tc_l(&self) -> XgOffsTcL {
    XgOffsTcL(self.rw.read(REG_XG_OFFS_TC_L))
  }
  pub fn set_xg_offs_tc_l(&self, value: XgOffsTcL) {
    self.rw.write(REG_XG_OFFS_TC_L, value.0)
  }
  pub fn with_xg_offs_tc_l<F: FnOnce(XgOffsTcL) -> XgOffsTcL>(&self, f: F) {
    let tmp = XgOffsTcL(self.rw.read(REG_XG_OFFS_TC_L));
    self.rw.write(REG_XG_OFFS_TC_L, f(tmp).0)
  }

  pub fn yg_offs_tc_h(&self) -> YgOffsTcH {
    YgOffsTcH(self.rw.read(REG_YG_OFFS_TC_H))
  }
  pub fn set_yg_offs_tc_h(&self, value: YgOffsTcH) {
    self.rw.write(REG_YG_OFFS_TC_H, value.0)
  }
  pub fn with_yg_offs_tc_h<F: FnOnce(YgOffsTcH) -> YgOffsTcH>(&self, f: F) {
    let tmp = YgOffsTcH(self.rw.read(REG_YG_OFFS_TC_H));
    self.rw.write(REG_YG_OFFS_TC_H, f(tmp).0)
  }

  pub fn yg_offs_tc_l(&self) -> YgOffsTcL {
    YgOffsTcL(self.rw.read(REG_YG_OFFS_TC_L))
  }
  pub fn set_yg_offs_tc_l(&self, value: YgOffsTcL) {
    self.rw.write(REG_YG_OFFS_TC_L, value.0)
  }
  pub fn with_yg_offs_tc_l<F: FnOnce(YgOffsTcL) -> YgOffsTcL>(&self, f: F) {
    let tmp = YgOffsTcL(self.rw.read(REG_YG_OFFS_TC_L));
    self.rw.write(REG_YG_OFFS_TC_L, f(tmp).0)
  }

  pub fn zg_offs_tc_h(&self) -> ZgOffsTcH {
    ZgOffsTcH(self.rw.read(REG_ZG_OFFS_TC_H))
  }
  pub fn set_zg_offs_tc_h(&self, value: ZgOffsTcH) {
    self.rw.write(REG_ZG_OFFS_TC_H, value.0)
  }
  pub fn with_zg_offs_tc_h<F: FnOnce(ZgOffsTcH) -> ZgOffsTcH>(&self, f: F) {
    let tmp = ZgOffsTcH(self.rw.read(REG_ZG_OFFS_TC_H));
    self.rw.write(REG_ZG_OFFS_TC_H, f(tmp).0)
  }

  pub fn zg_offs_tc_l(&self) -> ZgOffsTcL {
    ZgOffsTcL(self.rw.read(REG_ZG_OFFS_TC_L))
  }
  pub fn set_zg_offs_tc_l(&self, value: ZgOffsTcL) {
    self.rw.write(REG_ZG_OFFS_TC_L, value.0)
  }
  pub fn with_zg_offs_tc_l<F: FnOnce(ZgOffsTcL) -> ZgOffsTcL>(&self, f: F) {
    let tmp = ZgOffsTcL(self.rw.read(REG_ZG_OFFS_TC_L));
    self.rw.write(REG_ZG_OFFS_TC_L, f(tmp).0)
  }

  pub fn self_test_x_accel(&self) -> SelfTestXAccel {
    SelfTestXAccel(self.rw.read(REG_SELF_TEST_X_ACCEL))
  }
  pub fn set_self_test_x_accel(&self, value: SelfTestXAccel) {
    self.rw.write(REG_SELF_TEST_X_ACCEL, value.0)
  }
  pub fn with_self_test_x_accel<F: FnOnce(SelfTestXAccel) -> SelfTestXAccel>(&self, f: F) {
    let tmp = SelfTestXAccel(self.rw.read(REG_SELF_TEST_X_ACCEL));
    self.rw.write(REG_SELF_TEST_X_ACCEL, f(tmp).0)
  }

  pub fn self_test_y_accel(&self) -> SelfTestYAccel {
    SelfTestYAccel(self.rw.read(REG_SELF_TEST_Y_ACCEL))
  }
  pub fn set_self_test_y_accel(&self, value: SelfTestYAccel) {
    self.rw.write(REG_SELF_TEST_Y_ACCEL, value.0)
  }
  pub fn with_self_test_y_accel<F: FnOnce(SelfTestYAccel) -> SelfTestYAccel>(&self, f: F) {
    let tmp = SelfTestYAccel(self.rw.read(REG_SELF_TEST_Y_ACCEL));
    self.rw.write(REG_SELF_TEST_Y_ACCEL, f(tmp).0)
  }

  pub fn self_test_z_accel(&self) -> SelfTestZAccel {
    SelfTestZAccel(self.rw.read(REG_SELF_TEST_Z_ACCEL))
  }
  pub fn set_self_test_z_accel(&self, value: SelfTestZAccel) {
    self.rw.write(REG_SELF_TEST_Z_ACCEL, value.0)
  }
  pub fn with_self_test_z_accel<F: FnOnce(SelfTestZAccel) -> SelfTestZAccel>(&self, f: F) {
    let tmp = SelfTestZAccel(self.rw.read(REG_SELF_TEST_Z_ACCEL));
    self.rw.write(REG_SELF_TEST_Z_ACCEL, f(tmp).0)
  }

  pub fn xg_offs_usrh(&self) -> XgOffsUsrh {
    XgOffsUsrh(self.rw.read(REG_XG_OFFS_USRH))
  }
  pub fn set_xg_offs_usrh(&self, value: XgOffsUsrh) {
    self.rw.write(REG_XG_OFFS_USRH, value.0)
  }
  pub fn with_xg_offs_usrh<F: FnOnce(XgOffsUsrh) -> XgOffsUsrh>(&self, f: F) {
    let tmp = XgOffsUsrh(self.rw.read(REG_XG_OFFS_USRH));
    self.rw.write(REG_XG_OFFS_USRH, f(tmp).0)
  }

  pub fn xg_offs_usrl(&self) -> XgOffsUsrl {
    XgOffsUsrl(self.rw.read(REG_XG_OFFS_USRL))
  }
  pub fn set_xg_offs_usrl(&self, value: XgOffsUsrl) {
    self.rw.write(REG_XG_OFFS_USRL, value.0)
  }
  pub fn with_xg_offs_usrl<F: FnOnce(XgOffsUsrl) -> XgOffsUsrl>(&self, f: F) {
    let tmp = XgOffsUsrl(self.rw.read(REG_XG_OFFS_USRL));
    self.rw.write(REG_XG_OFFS_USRL, f(tmp).0)
  }

  pub fn yg_offs_usrh(&self) -> YgOffsUsrh {
    YgOffsUsrh(self.rw.read(REG_YG_OFFS_USRH))
  }
  pub fn set_yg_offs_usrh(&self, value: YgOffsUsrh) {
    self.rw.write(REG_YG_OFFS_USRH, value.0)
  }
  pub fn with_yg_offs_usrh<F: FnOnce(YgOffsUsrh) -> YgOffsUsrh>(&self, f: F) {
    let tmp = YgOffsUsrh(self.rw.read(REG_YG_OFFS_USRH));
    self.rw.write(REG_YG_OFFS_USRH, f(tmp).0)
  }

  pub fn yg_offs_usrl(&self) -> YgOffsUsrl {
    YgOffsUsrl(self.rw.read(REG_YG_OFFS_USRL))
  }
  pub fn set_yg_offs_usrl(&self, value: YgOffsUsrl) {
    self.rw.write(REG_YG_OFFS_USRL, value.0)
  }
  pub fn with_yg_offs_usrl<F: FnOnce(YgOffsUsrl) -> YgOffsUsrl>(&self, f: F) {
    let tmp = YgOffsUsrl(self.rw.read(REG_YG_OFFS_USRL));
    self.rw.write(REG_YG_OFFS_USRL, f(tmp).0)
  }

  pub fn zg_offs_usrh(&self) -> ZgOffsUsrh {
    ZgOffsUsrh(self.rw.read(REG_ZG_OFFS_USRH))
  }
  pub fn set_zg_offs_usrh(&self, value: ZgOffsUsrh) {
    self.rw.write(REG_ZG_OFFS_USRH, value.0)
  }
  pub fn with_zg_offs_usrh<F: FnOnce(ZgOffsUsrh) -> ZgOffsUsrh>(&self, f: F) {
    let tmp = ZgOffsUsrh(self.rw.read(REG_ZG_OFFS_USRH));
    self.rw.write(REG_ZG_OFFS_USRH, f(tmp).0)
  }

  pub fn zg_offs_usrl(&self) -> ZgOffsUsrl {
    ZgOffsUsrl(self.rw.read(REG_ZG_OFFS_USRL))
  }
  pub fn set_zg_offs_usrl(&self, value: ZgOffsUsrl) {
    self.rw.write(REG_ZG_OFFS_USRL, value.0)
  }
  pub fn with_zg_offs_usrl<F: FnOnce(ZgOffsUsrl) -> ZgOffsUsrl>(&self, f: F) {
    let tmp = ZgOffsUsrl(self.rw.read(REG_ZG_OFFS_USRL));
    self.rw.write(REG_ZG_OFFS_USRL, f(tmp).0)
  }

  pub fn smplrt_div(&self) -> SmplrtDiv {
    SmplrtDiv(self.rw.read(REG_SMPLRT_DIV))
  }
  pub fn set_smplrt_div(&self, value: SmplrtDiv) {
    self.rw.write(REG_SMPLRT_DIV, value.0)
  }
  pub fn with_smplrt_div<F: FnOnce(SmplrtDiv) -> SmplrtDiv>(&self, f: F) {
    let tmp = SmplrtDiv(self.rw.read(REG_SMPLRT_DIV));
    self.rw.write(REG_SMPLRT_DIV, f(tmp).0)
  }

  pub fn config(&self) -> Config {
    Config(self.rw.read(REG_CONFIG))
  }
  pub fn set_config(&self, value: Config) {
    self.rw.write(REG_CONFIG, value.0)
  }
  pub fn with_config<F: FnOnce(Config) -> Config>(&self, f: F) {
    let tmp = Config(self.rw.read(REG_CONFIG));
    self.rw.write(REG_CONFIG, f(tmp).0)
  }

  pub fn gyro_config(&self) -> GyroConfig {
    GyroConfig(self.rw.read(REG_GYRO_CONFIG))
  }
  pub fn set_gyro_config(&self, value: GyroConfig) {
    self.rw.write(REG_GYRO_CONFIG, value.0)
  }
  pub fn with_gyro_config<F: FnOnce(GyroConfig) -> GyroConfig>(&self, f: F) {
    let tmp = GyroConfig(self.rw.read(REG_GYRO_CONFIG));
    self.rw.write(REG_GYRO_CONFIG, f(tmp).0)
  }

  pub fn accel_config(&self) -> AccelConfig {
    AccelConfig(self.rw.read(REG_ACCEL_CONFIG))
  }
  pub fn set_accel_config(&self, value: AccelConfig) {
    self.rw.write(REG_ACCEL_CONFIG, value.0)
  }
  pub fn with_accel_config<F: FnOnce(AccelConfig) -> AccelConfig>(&self, f: F) {
    let tmp = AccelConfig(self.rw.read(REG_ACCEL_CONFIG));
    self.rw.write(REG_ACCEL_CONFIG, f(tmp).0)
  }

  pub fn accel_config_2(&self) -> AccelConfig2 {
    AccelConfig2(self.rw.read(REG_ACCEL_CONFIG_2))
  }
  pub fn set_accel_config_2(&self, value: AccelConfig2) {
    self.rw.write(REG_ACCEL_CONFIG_2, value.0)
  }
  pub fn with_accel_config_2<F: FnOnce(AccelConfig2) -> AccelConfig2>(&self, f: F) {
    let tmp = AccelConfig2(self.rw.read(REG_ACCEL_CONFIG_2));
    self.rw.write(REG_ACCEL_CONFIG_2, f(tmp).0)
  }

  pub fn lp_mode_cfg(&self) -> LpModeCfg {
    LpModeCfg(self.rw.read(REG_LP_MODE_CFG))
  }
  pub fn set_lp_mode_cfg(&self, value: LpModeCfg) {
    self.rw.write(REG_LP_MODE_CFG, value.0)
  }
  pub fn with_lp_mode_cfg<F: FnOnce(LpModeCfg) -> LpModeCfg>(&self, f: F) {
    let tmp = LpModeCfg(self.rw.read(REG_LP_MODE_CFG));
    self.rw.write(REG_LP_MODE_CFG, f(tmp).0)
  }

  pub fn accel_wom_x_thr(&self) -> AccelWomXThr {
    AccelWomXThr(self.rw.read(REG_ACCEL_WOM_X_THR))
  }
  pub fn set_accel_wom_x_thr(&self, value: AccelWomXThr) {
    self.rw.write(REG_ACCEL_WOM_X_THR, value.0)
  }
  pub fn with_accel_wom_x_thr<F: FnOnce(AccelWomXThr) -> AccelWomXThr>(&self, f: F) {
    let tmp = AccelWomXThr(self.rw.read(REG_ACCEL_WOM_X_THR));
    self.rw.write(REG_ACCEL_WOM_X_THR, f(tmp).0)
  }

  pub fn accel_wom_y_thr(&self) -> AccelWomYThr {
    AccelWomYThr(self.rw.read(REG_ACCEL_WOM_Y_THR))
  }
  pub fn set_accel_wom_y_thr(&self, value: AccelWomYThr) {
    self.rw.write(REG_ACCEL_WOM_Y_THR, value.0)
  }
  pub fn with_accel_wom_y_thr<F: FnOnce(AccelWomYThr) -> AccelWomYThr>(&self, f: F) {
    let tmp = AccelWomYThr(self.rw.read(REG_ACCEL_WOM_Y_THR));
    self.rw.write(REG_ACCEL_WOM_Y_THR, f(tmp).0)
  }

  pub fn accel_wom_z_thr(&self) -> AccelWomZThr {
    AccelWomZThr(self.rw.read(REG_ACCEL_WOM_Z_THR))
  }
  pub fn set_accel_wom_z_thr(&self, value: AccelWomZThr) {
    self.rw.write(REG_ACCEL_WOM_Z_THR, value.0)
  }
  pub fn with_accel_wom_z_thr<F: FnOnce(AccelWomZThr) -> AccelWomZThr>(&self, f: F) {
    let tmp = AccelWomZThr(self.rw.read(REG_ACCEL_WOM_Z_THR));
    self.rw.write(REG_ACCEL_WOM_Z_THR, f(tmp).0)
  }

  pub fn fifo_en(&self) -> FifoEn {
    FifoEn(self.rw.read(REG_FIFO_EN))
  }
  pub fn set_fifo_en(&self, value: FifoEn) {
    self.rw.write(REG_FIFO_EN, value.0)
  }
  pub fn with_fifo_en<F: FnOnce(FifoEn) -> FifoEn>(&self, f: F) {
    let tmp = FifoEn(self.rw.read(REG_FIFO_EN));
    self.rw.write(REG_FIFO_EN, f(tmp).0)
  }

  pub fn fsync_int(&self) -> FsyncInt {
    FsyncInt(self.rw.read(REG_FSYNC_INT))
  }
  pub fn set_fsync_int(&self, value: FsyncInt) {
    self.rw.write(REG_FSYNC_INT, value.0)
  }
  pub fn with_fsync_int<F: FnOnce(FsyncInt) -> FsyncInt>(&self, f: F) {
    let tmp = FsyncInt(self.rw.read(REG_FSYNC_INT));
    self.rw.write(REG_FSYNC_INT, f(tmp).0)
  }

  pub fn int_pin_cfg(&self) -> IntPinCfg {
    IntPinCfg(self.rw.read(REG_INT_PIN_CFG))
  }
  pub fn set_int_pin_cfg(&self, value: IntPinCfg) {
    self.rw.write(REG_INT_PIN_CFG, value.0)
  }
  pub fn with_int_pin_cfg<F: FnOnce(IntPinCfg) -> IntPinCfg>(&self, f: F) {
    let tmp = IntPinCfg(self.rw.read(REG_INT_PIN_CFG));
    self.rw.write(REG_INT_PIN_CFG, f(tmp).0)
  }

  pub fn int_enable(&self) -> IntEnable {
    IntEnable(self.rw.read(REG_INT_ENABLE))
  }
  pub fn set_int_enable(&self, value: IntEnable) {
    self.rw.write(REG_INT_ENABLE, value.0)
  }
  pub fn with_int_enable<F: FnOnce(IntEnable) -> IntEnable>(&self, f: F) {
    let tmp = IntEnable(self.rw.read(REG_INT_ENABLE));
    self.rw.write(REG_INT_ENABLE, f(tmp).0)
  }

  pub fn fifo_wm_int_status(&self) -> FifoWmIntStatus {
    FifoWmIntStatus(self.rw.read(REG_FIFO_WM_INT_STATUS))
  }
  pub fn set_fifo_wm_int_status(&self, value: FifoWmIntStatus) {
    self.rw.write(REG_FIFO_WM_INT_STATUS, value.0)
  }
  pub fn with_fifo_wm_int_status<F: FnOnce(FifoWmIntStatus) -> FifoWmIntStatus>(&self, f: F) {
    let tmp = FifoWmIntStatus(self.rw.read(REG_FIFO_WM_INT_STATUS));
    self.rw.write(REG_FIFO_WM_INT_STATUS, f(tmp).0)
  }

  pub fn int_status(&self) -> IntStatus {
    IntStatus(self.rw.read(REG_INT_STATUS))
  }
  pub fn set_int_status(&self, value: IntStatus) {
    self.rw.write(REG_INT_STATUS, value.0)
  }
  pub fn with_int_status<F: FnOnce(IntStatus) -> IntStatus>(&self, f: F) {
    let tmp = IntStatus(self.rw.read(REG_INT_STATUS));
    self.rw.write(REG_INT_STATUS, f(tmp).0)
  }

  pub fn accel_xout_h(&self) -> AccelXoutH {
    AccelXoutH(self.rw.read(REG_ACCEL_XOUT_H))
  }
  pub fn set_accel_xout_h(&self, value: AccelXoutH) {
    self.rw.write(REG_ACCEL_XOUT_H, value.0)
  }
  pub fn with_accel_xout_h<F: FnOnce(AccelXoutH) -> AccelXoutH>(&self, f: F) {
    let tmp = AccelXoutH(self.rw.read(REG_ACCEL_XOUT_H));
    self.rw.write(REG_ACCEL_XOUT_H, f(tmp).0)
  }

  pub fn accel_xout_l(&self) -> AccelXoutL {
    AccelXoutL(self.rw.read(REG_ACCEL_XOUT_L))
  }
  pub fn set_accel_xout_l(&self, value: AccelXoutL) {
    self.rw.write(REG_ACCEL_XOUT_L, value.0)
  }
  pub fn with_accel_xout_l<F: FnOnce(AccelXoutL) -> AccelXoutL>(&self, f: F) {
    let tmp = AccelXoutL(self.rw.read(REG_ACCEL_XOUT_L));
    self.rw.write(REG_ACCEL_XOUT_L, f(tmp).0)
  }

  pub fn accel_yout_h(&self) -> AccelYoutH {
    AccelYoutH(self.rw.read(REG_ACCEL_YOUT_H))
  }
  pub fn set_accel_yout_h(&self, value: AccelYoutH) {
    self.rw.write(REG_ACCEL_YOUT_H, value.0)
  }
  pub fn with_accel_yout_h<F: FnOnce(AccelYoutH) -> AccelYoutH>(&self, f: F) {
    let tmp = AccelYoutH(self.rw.read(REG_ACCEL_YOUT_H));
    self.rw.write(REG_ACCEL_YOUT_H, f(tmp).0)
  }

  pub fn accel_yout_l(&self) -> AccelYoutL {
    AccelYoutL(self.rw.read(REG_ACCEL_YOUT_L))
  }
  pub fn set_accel_yout_l(&self, value: AccelYoutL) {
    self.rw.write(REG_ACCEL_YOUT_L, value.0)
  }
  pub fn with_accel_yout_l<F: FnOnce(AccelYoutL) -> AccelYoutL>(&self, f: F) {
    let tmp = AccelYoutL(self.rw.read(REG_ACCEL_YOUT_L));
    self.rw.write(REG_ACCEL_YOUT_L, f(tmp).0)
  }

  pub fn accel_zout_h(&self) -> AccelZoutH {
    AccelZoutH(self.rw.read(REG_ACCEL_ZOUT_H))
  }
  pub fn set_accel_zout_h(&self, value: AccelZoutH) {
    self.rw.write(REG_ACCEL_ZOUT_H, value.0)
  }
  pub fn with_accel_zout_h<F: FnOnce(AccelZoutH) -> AccelZoutH>(&self, f: F) {
    let tmp = AccelZoutH(self.rw.read(REG_ACCEL_ZOUT_H));
    self.rw.write(REG_ACCEL_ZOUT_H, f(tmp).0)
  }

  pub fn accel_zout_l(&self) -> AccelZoutL {
    AccelZoutL(self.rw.read(REG_ACCEL_ZOUT_L))
  }
  pub fn set_accel_zout_l(&self, value: AccelZoutL) {
    self.rw.write(REG_ACCEL_ZOUT_L, value.0)
  }
  pub fn with_accel_zout_l<F: FnOnce(AccelZoutL) -> AccelZoutL>(&self, f: F) {
    let tmp = AccelZoutL(self.rw.read(REG_ACCEL_ZOUT_L));
    self.rw.write(REG_ACCEL_ZOUT_L, f(tmp).0)
  }

  pub fn temp_out_h(&self) -> TempOutH {
    TempOutH(self.rw.read(REG_TEMP_OUT_H))
  }
  pub fn set_temp_out_h(&self, value: TempOutH) {
    self.rw.write(REG_TEMP_OUT_H, value.0)
  }
  pub fn with_temp_out_h<F: FnOnce(TempOutH) -> TempOutH>(&self, f: F) {
    let tmp = TempOutH(self.rw.read(REG_TEMP_OUT_H));
    self.rw.write(REG_TEMP_OUT_H, f(tmp).0)
  }

  pub fn temp_out_l(&self) -> TempOutL {
    TempOutL(self.rw.read(REG_TEMP_OUT_L))
  }
  pub fn set_temp_out_l(&self, value: TempOutL) {
    self.rw.write(REG_TEMP_OUT_L, value.0)
  }
  pub fn with_temp_out_l<F: FnOnce(TempOutL) -> TempOutL>(&self, f: F) {
    let tmp = TempOutL(self.rw.read(REG_TEMP_OUT_L));
    self.rw.write(REG_TEMP_OUT_L, f(tmp).0)
  }

  pub fn gyro_xout_h(&self) -> GyroXoutH {
    GyroXoutH(self.rw.read(REG_GYRO_XOUT_H))
  }
  pub fn set_gyro_xout_h(&self, value: GyroXoutH) {
    self.rw.write(REG_GYRO_XOUT_H, value.0)
  }
  pub fn with_gyro_xout_h<F: FnOnce(GyroXoutH) -> GyroXoutH>(&self, f: F) {
    let tmp = GyroXoutH(self.rw.read(REG_GYRO_XOUT_H));
    self.rw.write(REG_GYRO_XOUT_H, f(tmp).0)
  }

  pub fn gyro_xout_l(&self) -> GyroXoutL {
    GyroXoutL(self.rw.read(REG_GYRO_XOUT_L))
  }
  pub fn set_gyro_xout_l(&self, value: GyroXoutL) {
    self.rw.write(REG_GYRO_XOUT_L, value.0)
  }
  pub fn with_gyro_xout_l<F: FnOnce(GyroXoutL) -> GyroXoutL>(&self, f: F) {
    let tmp = GyroXoutL(self.rw.read(REG_GYRO_XOUT_L));
    self.rw.write(REG_GYRO_XOUT_L, f(tmp).0)
  }

  pub fn gyro_yout_h(&self) -> GyroYoutH {
    GyroYoutH(self.rw.read(REG_GYRO_YOUT_H))
  }
  pub fn set_gyro_yout_h(&self, value: GyroYoutH) {
    self.rw.write(REG_GYRO_YOUT_H, value.0)
  }
  pub fn with_gyro_yout_h<F: FnOnce(GyroYoutH) -> GyroYoutH>(&self, f: F) {
    let tmp = GyroYoutH(self.rw.read(REG_GYRO_YOUT_H));
    self.rw.write(REG_GYRO_YOUT_H, f(tmp).0)
  }

  pub fn gyro_yout_l(&self) -> GyroYoutL {
    GyroYoutL(self.rw.read(REG_GYRO_YOUT_L))
  }
  pub fn set_gyro_yout_l(&self, value: GyroYoutL) {
    self.rw.write(REG_GYRO_YOUT_L, value.0)
  }
  pub fn with_gyro_yout_l<F: FnOnce(GyroYoutL) -> GyroYoutL>(&self, f: F) {
    let tmp = GyroYoutL(self.rw.read(REG_GYRO_YOUT_L));
    self.rw.write(REG_GYRO_YOUT_L, f(tmp).0)
  }

  pub fn gyro_zout_h(&self) -> GyroZoutH {
    GyroZoutH(self.rw.read(REG_GYRO_ZOUT_H))
  }
  pub fn set_gyro_zout_h(&self, value: GyroZoutH) {
    self.rw.write(REG_GYRO_ZOUT_H, value.0)
  }
  pub fn with_gyro_zout_h<F: FnOnce(GyroZoutH) -> GyroZoutH>(&self, f: F) {
    let tmp = GyroZoutH(self.rw.read(REG_GYRO_ZOUT_H));
    self.rw.write(REG_GYRO_ZOUT_H, f(tmp).0)
  }

  pub fn gyro_zout_l(&self) -> GyroZoutL {
    GyroZoutL(self.rw.read(REG_GYRO_ZOUT_L))
  }
  pub fn set_gyro_zout_l(&self, value: GyroZoutL) {
    self.rw.write(REG_GYRO_ZOUT_L, value.0)
  }
  pub fn with_gyro_zout_l<F: FnOnce(GyroZoutL) -> GyroZoutL>(&self, f: F) {
    let tmp = GyroZoutL(self.rw.read(REG_GYRO_ZOUT_L));
    self.rw.write(REG_GYRO_ZOUT_L, f(tmp).0)
  }

  pub fn self_test_x_gyro(&self) -> SelfTestXGyro {
    SelfTestXGyro(self.rw.read(REG_SELF_TEST_X_GYRO))
  }
  pub fn set_self_test_x_gyro(&self, value: SelfTestXGyro) {
    self.rw.write(REG_SELF_TEST_X_GYRO, value.0)
  }
  pub fn with_self_test_x_gyro<F: FnOnce(SelfTestXGyro) -> SelfTestXGyro>(&self, f: F) {
    let tmp = SelfTestXGyro(self.rw.read(REG_SELF_TEST_X_GYRO));
    self.rw.write(REG_SELF_TEST_X_GYRO, f(tmp).0)
  }

  pub fn self_test_y_gyro(&self) -> SelfTestYGyro {
    SelfTestYGyro(self.rw.read(REG_SELF_TEST_Y_GYRO))
  }
  pub fn set_self_test_y_gyro(&self, value: SelfTestYGyro) {
    self.rw.write(REG_SELF_TEST_Y_GYRO, value.0)
  }
  pub fn with_self_test_y_gyro<F: FnOnce(SelfTestYGyro) -> SelfTestYGyro>(&self, f: F) {
    let tmp = SelfTestYGyro(self.rw.read(REG_SELF_TEST_Y_GYRO));
    self.rw.write(REG_SELF_TEST_Y_GYRO, f(tmp).0)
  }

  pub fn self_test_z_gyro(&self) -> SelfTestZGyro {
    SelfTestZGyro(self.rw.read(REG_SELF_TEST_Z_GYRO))
  }
  pub fn set_self_test_z_gyro(&self, value: SelfTestZGyro) {
    self.rw.write(REG_SELF_TEST_Z_GYRO, value.0)
  }
  pub fn with_self_test_z_gyro<F: FnOnce(SelfTestZGyro) -> SelfTestZGyro>(&self, f: F) {
    let tmp = SelfTestZGyro(self.rw.read(REG_SELF_TEST_Z_GYRO));
    self.rw.write(REG_SELF_TEST_Z_GYRO, f(tmp).0)
  }

  pub fn fifo_wm_th1(&self) -> FifoWmTh1 {
    FifoWmTh1(self.rw.read(REG_FIFO_WM_TH1))
  }
  pub fn set_fifo_wm_th1(&self, value: FifoWmTh1) {
    self.rw.write(REG_FIFO_WM_TH1, value.0)
  }
  pub fn with_fifo_wm_th1<F: FnOnce(FifoWmTh1) -> FifoWmTh1>(&self, f: F) {
    let tmp = FifoWmTh1(self.rw.read(REG_FIFO_WM_TH1));
    self.rw.write(REG_FIFO_WM_TH1, f(tmp).0)
  }

  pub fn fifo_wm_th2(&self) -> FifoWmTh2 {
    FifoWmTh2(self.rw.read(REG_FIFO_WM_TH2))
  }
  pub fn set_fifo_wm_th2(&self, value: FifoWmTh2) {
    self.rw.write(REG_FIFO_WM_TH2, value.0)
  }
  pub fn with_fifo_wm_th2<F: FnOnce(FifoWmTh2) -> FifoWmTh2>(&self, f: F) {
    let tmp = FifoWmTh2(self.rw.read(REG_FIFO_WM_TH2));
    self.rw.write(REG_FIFO_WM_TH2, f(tmp).0)
  }

  pub fn signal_path_reset(&self) -> SignalPathReset {
    SignalPathReset(self.rw.read(REG_SIGNAL_PATH_RESET))
  }
  pub fn set_signal_path_reset(&self, value: SignalPathReset) {
    self.rw.write(REG_SIGNAL_PATH_RESET, value.0)
  }
  pub fn with_signal_path_reset<F: FnOnce(SignalPathReset) -> SignalPathReset>(&self, f: F) {
    let tmp = SignalPathReset(self.rw.read(REG_SIGNAL_PATH_RESET));
    self.rw.write(REG_SIGNAL_PATH_RESET, f(tmp).0)
  }

  pub fn accel_intel_ctrl(&self) -> AccelIntelCtrl {
    AccelIntelCtrl(self.rw.read(REG_ACCEL_INTEL_CTRL))
  }
  pub fn set_accel_intel_ctrl(&self, value: AccelIntelCtrl) {
    self.rw.write(REG_ACCEL_INTEL_CTRL, value.0)
  }
  pub fn with_accel_intel_ctrl<F: FnOnce(AccelIntelCtrl) -> AccelIntelCtrl>(&self, f: F) {
    let tmp = AccelIntelCtrl(self.rw.read(REG_ACCEL_INTEL_CTRL));
    self.rw.write(REG_ACCEL_INTEL_CTRL, f(tmp).0)
  }

  pub fn user_ctrl(&self) -> UserCtrl {
    UserCtrl(self.rw.read(REG_USER_CTRL))
  }
  pub fn set_user_ctrl(&self, value: UserCtrl) {
    self.rw.write(REG_USER_CTRL, value.0)
  }
  pub fn with_user_ctrl<F: FnOnce(UserCtrl) -> UserCtrl>(&self, f: F) {
    let tmp = UserCtrl(self.rw.read(REG_USER_CTRL));
    self.rw.write(REG_USER_CTRL, f(tmp).0)
  }

  pub fn pwr_mgmt_1(&self) -> PwrMgmt1 {
    PwrMgmt1(self.rw.read(REG_PWR_MGMT_1))
  }
  pub fn set_pwr_mgmt_1(&self, value: PwrMgmt1) {
    self.rw.write(REG_PWR_MGMT_1, value.0)
  }
  pub fn with_pwr_mgmt_1<F: FnOnce(PwrMgmt1) -> PwrMgmt1>(&self, f: F) {
    let tmp = PwrMgmt1(self.rw.read(REG_PWR_MGMT_1));
    self.rw.write(REG_PWR_MGMT_1, f(tmp).0)
  }

  pub fn pwr_mgmt_2(&self) -> PwrMgmt2 {
    PwrMgmt2(self.rw.read(REG_PWR_MGMT_2))
  }
  pub fn set_pwr_mgmt_2(&self, value: PwrMgmt2) {
    self.rw.write(REG_PWR_MGMT_2, value.0)
  }
  pub fn with_pwr_mgmt_2<F: FnOnce(PwrMgmt2) -> PwrMgmt2>(&self, f: F) {
    let tmp = PwrMgmt2(self.rw.read(REG_PWR_MGMT_2));
    self.rw.write(REG_PWR_MGMT_2, f(tmp).0)
  }

  pub fn i2c_if(&self) -> I2cIf {
    I2cIf(self.rw.read(REG_I2C_IF))
  }
  pub fn set_i2c_if(&self, value: I2cIf) {
    self.rw.write(REG_I2C_IF, value.0)
  }
  pub fn with_i2c_if<F: FnOnce(I2cIf) -> I2cIf>(&self, f: F) {
    let tmp = I2cIf(self.rw.read(REG_I2C_IF));
    self.rw.write(REG_I2C_IF, f(tmp).0)
  }

  pub fn fifo_counth(&self) -> FifoCounth {
    FifoCounth(self.rw.read(REG_FIFO_COUNTH))
  }
  pub fn set_fifo_counth(&self, value: FifoCounth) {
    self.rw.write(REG_FIFO_COUNTH, value.0)
  }
  pub fn with_fifo_counth<F: FnOnce(FifoCounth) -> FifoCounth>(&self, f: F) {
    let tmp = FifoCounth(self.rw.read(REG_FIFO_COUNTH));
    self.rw.write(REG_FIFO_COUNTH, f(tmp).0)
  }

  pub fn fifo_countl(&self) -> FifoCountl {
    FifoCountl(self.rw.read(REG_FIFO_COUNTL))
  }
  pub fn set_fifo_countl(&self, value: FifoCountl) {
    self.rw.write(REG_FIFO_COUNTL, value.0)
  }
  pub fn with_fifo_countl<F: FnOnce(FifoCountl) -> FifoCountl>(&self, f: F) {
    let tmp = FifoCountl(self.rw.read(REG_FIFO_COUNTL));
    self.rw.write(REG_FIFO_COUNTL, f(tmp).0)
  }

  pub fn fifo_r_w(&self) -> FifoRW {
    FifoRW(self.rw.read(REG_FIFO_R_W))
  }
  pub fn set_fifo_r_w(&self, value: FifoRW) {
    self.rw.write(REG_FIFO_R_W, value.0)
  }
  pub fn with_fifo_r_w<F: FnOnce(FifoRW) -> FifoRW>(&self, f: F) {
    let tmp = FifoRW(self.rw.read(REG_FIFO_R_W));
    self.rw.write(REG_FIFO_R_W, f(tmp).0)
  }

  pub fn who_am_i(&self) -> WhoAmI {
    WhoAmI(self.rw.read(REG_WHO_AM_I))
  }
  pub fn set_who_am_i(&self, value: WhoAmI) {
    self.rw.write(REG_WHO_AM_I, value.0)
  }
  pub fn with_who_am_i<F: FnOnce(WhoAmI) -> WhoAmI>(&self, f: F) {
    let tmp = WhoAmI(self.rw.read(REG_WHO_AM_I));
    self.rw.write(REG_WHO_AM_I, f(tmp).0)
  }

  pub fn xa_offset_h(&self) -> XaOffsetH {
    XaOffsetH(self.rw.read(REG_XA_OFFSET_H))
  }
  pub fn set_xa_offset_h(&self, value: XaOffsetH) {
    self.rw.write(REG_XA_OFFSET_H, value.0)
  }
  pub fn with_xa_offset_h<F: FnOnce(XaOffsetH) -> XaOffsetH>(&self, f: F) {
    let tmp = XaOffsetH(self.rw.read(REG_XA_OFFSET_H));
    self.rw.write(REG_XA_OFFSET_H, f(tmp).0)
  }

  pub fn xa_offset_l(&self) -> XaOffsetL {
    XaOffsetL(self.rw.read(REG_XA_OFFSET_L))
  }
  pub fn set_xa_offset_l(&self, value: XaOffsetL) {
    self.rw.write(REG_XA_OFFSET_L, value.0)
  }
  pub fn with_xa_offset_l<F: FnOnce(XaOffsetL) -> XaOffsetL>(&self, f: F) {
    let tmp = XaOffsetL(self.rw.read(REG_XA_OFFSET_L));
    self.rw.write(REG_XA_OFFSET_L, f(tmp).0)
  }

  pub fn ya_offset_h(&self) -> YaOffsetH {
    YaOffsetH(self.rw.read(REG_YA_OFFSET_H))
  }
  pub fn set_ya_offset_h(&self, value: YaOffsetH) {
    self.rw.write(REG_YA_OFFSET_H, value.0)
  }
  pub fn with_ya_offset_h<F: FnOnce(YaOffsetH) -> YaOffsetH>(&self, f: F) {
    let tmp = YaOffsetH(self.rw.read(REG_YA_OFFSET_H));
    self.rw.write(REG_YA_OFFSET_H, f(tmp).0)
  }

  pub fn ya_offset_l(&self) -> YaOffsetL {
    YaOffsetL(self.rw.read(REG_YA_OFFSET_L))
  }
  pub fn set_ya_offset_l(&self, value: YaOffsetL) {
    self.rw.write(REG_YA_OFFSET_L, value.0)
  }
  pub fn with_ya_offset_l<F: FnOnce(YaOffsetL) -> YaOffsetL>(&self, f: F) {
    let tmp = YaOffsetL(self.rw.read(REG_YA_OFFSET_L));
    self.rw.write(REG_YA_OFFSET_L, f(tmp).0)
  }

  pub fn za_offset_h(&self) -> ZaOffsetH {
    ZaOffsetH(self.rw.read(REG_ZA_OFFSET_H))
  }
  pub fn set_za_offset_h(&self, value: ZaOffsetH) {
    self.rw.write(REG_ZA_OFFSET_H, value.0)
  }
  pub fn with_za_offset_h<F: FnOnce(ZaOffsetH) -> ZaOffsetH>(&self, f: F) {
    let tmp = ZaOffsetH(self.rw.read(REG_ZA_OFFSET_H));
    self.rw.write(REG_ZA_OFFSET_H, f(tmp).0)
  }

  pub fn za_offset_l(&self) -> ZaOffsetL {
    ZaOffsetL(self.rw.read(REG_ZA_OFFSET_L))
  }
  pub fn set_za_offset_l(&self, value: ZaOffsetL) {
    self.rw.write(REG_ZA_OFFSET_L, value.0)
  }
  pub fn with_za_offset_l<F: FnOnce(ZaOffsetL) -> ZaOffsetL>(&self, f: F) {
    let tmp = ZaOffsetL(self.rw.read(REG_ZA_OFFSET_L));
    self.rw.write(REG_ZA_OFFSET_L, f(tmp).0)
  }

}

pub struct XgOffsTcH(pub u8);

impl XgOffsTcH {
  pub fn xg_offs_lp(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x3f // [7:2]
  }

  pub fn set_xg_offs_lp(mut self, value: u8) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 2);
     self.0 |= value << 2;
     self
  }

  pub fn xg_offs_tc_h(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x3 // [1:0]
  }

  pub fn set_xg_offs_tc_h(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for XgOffsTcH {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for XgOffsTcH {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.xg_offs_lp() != 0 { try!(write!(f, " xg_offs_lp=0x{:x}", self.xg_offs_lp()))}
      if self.xg_offs_tc_h() != 0 { try!(write!(f, " xg_offs_tc_h=0x{:x}", self.xg_offs_tc_h()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct XgOffsTcL(pub u8);

impl XgOffsTcL {
  pub fn xg_offs_tc_l(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_xg_offs_tc_l(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for XgOffsTcL {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for XgOffsTcL {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.xg_offs_tc_l() != 0 { try!(write!(f, " xg_offs_tc_l=0x{:x}", self.xg_offs_tc_l()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct YgOffsTcH(pub u8);

impl YgOffsTcH {
  pub fn yg_offs_lp(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x3f // [7:2]
  }

  pub fn set_yg_offs_lp(mut self, value: u8) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 2);
     self.0 |= value << 2;
     self
  }

  pub fn yg_offs_tc_h(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x3 // [1:0]
  }

  pub fn set_yg_offs_tc_h(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for YgOffsTcH {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for YgOffsTcH {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.yg_offs_lp() != 0 { try!(write!(f, " yg_offs_lp=0x{:x}", self.yg_offs_lp()))}
      if self.yg_offs_tc_h() != 0 { try!(write!(f, " yg_offs_tc_h=0x{:x}", self.yg_offs_tc_h()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct YgOffsTcL(pub u8);

impl YgOffsTcL {
  pub fn yg_offs_tc_l(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_yg_offs_tc_l(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for YgOffsTcL {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for YgOffsTcL {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.yg_offs_tc_l() != 0 { try!(write!(f, " yg_offs_tc_l=0x{:x}", self.yg_offs_tc_l()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct ZgOffsTcH(pub u8);

impl ZgOffsTcH {
  pub fn zg_offs_lp(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x3f // [7:2]
  }

  pub fn set_zg_offs_lp(mut self, value: u8) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 2);
     self.0 |= value << 2;
     self
  }

  pub fn zg_offs_tc_h(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x3 // [1:0]
  }

  pub fn set_zg_offs_tc_h(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for ZgOffsTcH {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for ZgOffsTcH {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.zg_offs_lp() != 0 { try!(write!(f, " zg_offs_lp=0x{:x}", self.zg_offs_lp()))}
      if self.zg_offs_tc_h() != 0 { try!(write!(f, " zg_offs_tc_h=0x{:x}", self.zg_offs_tc_h()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct ZgOffsTcL(pub u8);

impl ZgOffsTcL {
  pub fn zg_offs_tc_l(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_zg_offs_tc_l(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for ZgOffsTcL {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for ZgOffsTcL {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.zg_offs_tc_l() != 0 { try!(write!(f, " zg_offs_tc_l=0x{:x}", self.zg_offs_tc_l()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct SelfTestXAccel(pub u8);

impl SelfTestXAccel {
  pub fn xa_st_data(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_xa_st_data(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for SelfTestXAccel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for SelfTestXAccel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.xa_st_data() != 0 { try!(write!(f, " xa_st_data=0x{:x}", self.xa_st_data()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct SelfTestYAccel(pub u8);

impl SelfTestYAccel {
  pub fn ya_st_data(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_ya_st_data(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for SelfTestYAccel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for SelfTestYAccel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.ya_st_data() != 0 { try!(write!(f, " ya_st_data=0x{:x}", self.ya_st_data()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct SelfTestZAccel(pub u8);

impl SelfTestZAccel {
  pub fn za_st_data(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_za_st_data(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for SelfTestZAccel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for SelfTestZAccel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.za_st_data() != 0 { try!(write!(f, " za_st_data=0x{:x}", self.za_st_data()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct XgOffsUsrh(pub u8);

impl XgOffsUsrh {
  pub fn x_offs_usr(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_x_offs_usr(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for XgOffsUsrh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for XgOffsUsrh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.x_offs_usr() != 0 { try!(write!(f, " x_offs_usr=0x{:x}", self.x_offs_usr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct XgOffsUsrl(pub u8);

impl XgOffsUsrl {
  pub fn x_offs_usr(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_x_offs_usr(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for XgOffsUsrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for XgOffsUsrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.x_offs_usr() != 0 { try!(write!(f, " x_offs_usr=0x{:x}", self.x_offs_usr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct YgOffsUsrh(pub u8);

impl YgOffsUsrh {
  pub fn y_offs_usr(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_y_offs_usr(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for YgOffsUsrh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for YgOffsUsrh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.y_offs_usr() != 0 { try!(write!(f, " y_offs_usr=0x{:x}", self.y_offs_usr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct YgOffsUsrl(pub u8);

impl YgOffsUsrl {
  pub fn y_offs_usr(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_y_offs_usr(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for YgOffsUsrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for YgOffsUsrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.y_offs_usr() != 0 { try!(write!(f, " y_offs_usr=0x{:x}", self.y_offs_usr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct ZgOffsUsrh(pub u8);

impl ZgOffsUsrh {
  pub fn z_offs_usr(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_z_offs_usr(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for ZgOffsUsrh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for ZgOffsUsrh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.z_offs_usr() != 0 { try!(write!(f, " z_offs_usr=0x{:x}", self.z_offs_usr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct ZgOffsUsrl(pub u8);

impl ZgOffsUsrl {
  pub fn z_offs_usr(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_z_offs_usr(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for ZgOffsUsrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for ZgOffsUsrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.z_offs_usr() != 0 { try!(write!(f, " z_offs_usr=0x{:x}", self.z_offs_usr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct SmplrtDiv(pub u8);

impl SmplrtDiv {
  pub fn smplrt_div(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_smplrt_div(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for SmplrtDiv {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for SmplrtDiv {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.smplrt_div() != 0 { try!(write!(f, " smplrt_div=0x{:x}", self.smplrt_div()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct Config(pub u8);

impl Config {
  pub fn fifo_mode(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }

  pub fn set_fifo_mode(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn ext_sync_set(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x7 // [5:3]
  }

  pub fn set_ext_sync_set(mut self, value: u8) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn dlpf_cfg(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x7 // [2:0]
  }

  pub fn set_dlpf_cfg(mut self, value: u8) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Config {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Config {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.fifo_mode() != 0 { try!(write!(f, " fifo_mode"))}
      if self.ext_sync_set() != 0 { try!(write!(f, " ext_sync_set=0x{:x}", self.ext_sync_set()))}
      if self.dlpf_cfg() != 0 { try!(write!(f, " dlpf_cfg=0x{:x}", self.dlpf_cfg()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct GyroConfig(pub u8);

impl GyroConfig {
  pub fn xg_st(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }

  pub fn set_xg_st(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn yg_st(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }

  pub fn set_yg_st(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn zg_st(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }

  pub fn set_zg_st(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn fs_sel(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x3 // [4:3]
  }

  pub fn set_fs_sel(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn fchoice_b(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x3 // [1:0]
  }

  pub fn set_fchoice_b(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for GyroConfig {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for GyroConfig {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.xg_st() != 0 { try!(write!(f, " xg_st"))}
      if self.yg_st() != 0 { try!(write!(f, " yg_st"))}
      if self.zg_st() != 0 { try!(write!(f, " zg_st"))}
      if self.fs_sel() != 0 { try!(write!(f, " fs_sel=0x{:x}", self.fs_sel()))}
      if self.fchoice_b() != 0 { try!(write!(f, " fchoice_b=0x{:x}", self.fchoice_b()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct AccelConfig(pub u8);

impl AccelConfig {
  pub fn xa_st(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }

  pub fn set_xa_st(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn ya_st(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }

  pub fn set_ya_st(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn za_st(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }

  pub fn set_za_st(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn accel_fs_sel(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x3 // [4:3]
  }

  pub fn set_accel_fs_sel(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 3);
     self.0 |= value << 3;
     self
  }

}

impl ::core::fmt::Display for AccelConfig {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for AccelConfig {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.xa_st() != 0 { try!(write!(f, " xa_st"))}
      if self.ya_st() != 0 { try!(write!(f, " ya_st"))}
      if self.za_st() != 0 { try!(write!(f, " za_st"))}
      if self.accel_fs_sel() != 0 { try!(write!(f, " accel_fs_sel=0x{:x}", self.accel_fs_sel()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct AccelConfig2(pub u8);

impl AccelConfig2 {
  pub fn dec2_cfg(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }

  pub fn set_dec2_cfg(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn accel_fchoice_b(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }

  pub fn set_accel_fchoice_b(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn a_dlpf_cfg(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }

  pub fn set_a_dlpf_cfg(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for AccelConfig2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for AccelConfig2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.dec2_cfg() != 0 { try!(write!(f, " dec2_cfg"))}
      if self.accel_fchoice_b() != 0 { try!(write!(f, " accel_fchoice_b"))}
      if self.a_dlpf_cfg() != 0 { try!(write!(f, " a_dlpf_cfg"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct LpModeCfg(pub u8);

impl LpModeCfg {
  pub fn gyro_cycle(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }

  pub fn set_gyro_cycle(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn g_avgcfg(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x7 // [6:4]
  }

  pub fn set_g_avgcfg(mut self, value: u8) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 4);
     self.0 |= value << 4;
     self
  }

}

impl ::core::fmt::Display for LpModeCfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for LpModeCfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.gyro_cycle() != 0 { try!(write!(f, " gyro_cycle"))}
      if self.g_avgcfg() != 0 { try!(write!(f, " g_avgcfg=0x{:x}", self.g_avgcfg()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct AccelWomXThr(pub u8);

impl AccelWomXThr {
  pub fn wom_x_th(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_wom_x_th(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for AccelWomXThr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for AccelWomXThr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.wom_x_th() != 0 { try!(write!(f, " wom_x_th=0x{:x}", self.wom_x_th()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct AccelWomYThr(pub u8);

impl AccelWomYThr {
  pub fn wom_y_th(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_wom_y_th(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for AccelWomYThr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for AccelWomYThr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.wom_y_th() != 0 { try!(write!(f, " wom_y_th=0x{:x}", self.wom_y_th()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct AccelWomZThr(pub u8);

impl AccelWomZThr {
  pub fn wom_z_th(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_wom_z_th(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for AccelWomZThr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for AccelWomZThr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.wom_z_th() != 0 { try!(write!(f, " wom_z_th=0x{:x}", self.wom_z_th()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct FifoEn(pub u8);

impl FifoEn {
  pub fn gyro_fifo_en(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }

  pub fn set_gyro_fifo_en(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn accel_fifo_en(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }

  pub fn set_accel_fifo_en(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}

impl ::core::fmt::Display for FifoEn {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for FifoEn {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.gyro_fifo_en() != 0 { try!(write!(f, " gyro_fifo_en"))}
      if self.accel_fifo_en() != 0 { try!(write!(f, " accel_fifo_en"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct FsyncInt(pub u8);

impl FsyncInt {
  pub fn fsync_int(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }

  pub fn set_fsync_int(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for FsyncInt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for FsyncInt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.fsync_int() != 0 { try!(write!(f, " fsync_int"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct IntPinCfg(pub u8);

impl IntPinCfg {
  pub fn int_level(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }

  pub fn set_int_level(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn int_open(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }

  pub fn set_int_open(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn latch_int_en(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }

  pub fn set_latch_int_en(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn int_rd_clear(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }

  pub fn set_int_rd_clear(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn fsync_int_level(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }

  pub fn set_fsync_int_level(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn fsync_int_mode_en(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }

  pub fn set_fsync_int_mode_en(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

}

impl ::core::fmt::Display for IntPinCfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for IntPinCfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.int_level() != 0 { try!(write!(f, " int_level"))}
      if self.int_open() != 0 { try!(write!(f, " int_open"))}
      if self.latch_int_en() != 0 { try!(write!(f, " latch_int_en"))}
      if self.int_rd_clear() != 0 { try!(write!(f, " int_rd_clear"))}
      if self.fsync_int_level() != 0 { try!(write!(f, " fsync_int_level"))}
      if self.fsync_int_mode_en() != 0 { try!(write!(f, " fsync_int_mode_en"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct IntEnable(pub u8);

impl IntEnable {
  pub fn wom_x_int_en(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }

  pub fn set_wom_x_int_en(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn wom_y_int_en(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }

  pub fn set_wom_y_int_en(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn wom_z_int_en(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }

  pub fn set_wom_z_int_en(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn fifo_oflow_en(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }

  pub fn set_fifo_oflow_en(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn fsync_int_en(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }

  pub fn set_fsync_int_en(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn gdrive_int_en(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }

  pub fn set_gdrive_int_en(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn data_rdy_int_en(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }

  pub fn set_data_rdy_int_en(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for IntEnable {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for IntEnable {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.wom_x_int_en() != 0 { try!(write!(f, " wom_x_int_en"))}
      if self.wom_y_int_en() != 0 { try!(write!(f, " wom_y_int_en"))}
      if self.wom_z_int_en() != 0 { try!(write!(f, " wom_z_int_en"))}
      if self.fifo_oflow_en() != 0 { try!(write!(f, " fifo_oflow_en"))}
      if self.fsync_int_en() != 0 { try!(write!(f, " fsync_int_en"))}
      if self.gdrive_int_en() != 0 { try!(write!(f, " gdrive_int_en"))}
      if self.data_rdy_int_en() != 0 { try!(write!(f, " data_rdy_int_en"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct FifoWmIntStatus(pub u8);

impl FifoWmIntStatus {
  pub fn fifo_wm_int(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }

  pub fn set_fifo_wm_int(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

}

impl ::core::fmt::Display for FifoWmIntStatus {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for FifoWmIntStatus {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.fifo_wm_int() != 0 { try!(write!(f, " fifo_wm_int"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct IntStatus(pub u8);

impl IntStatus {
  pub fn wom_x_int(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }

  pub fn set_wom_x_int(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn wom_y_int(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }

  pub fn set_wom_y_int(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn wom_z_int(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }

  pub fn set_wom_z_int(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn fifo_oflow_int(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }

  pub fn set_fifo_oflow_int(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn gdrive_int(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }

  pub fn set_gdrive_int(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn data_rdy_int(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }

  pub fn set_data_rdy_int(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for IntStatus {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for IntStatus {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.wom_x_int() != 0 { try!(write!(f, " wom_x_int"))}
      if self.wom_y_int() != 0 { try!(write!(f, " wom_y_int"))}
      if self.wom_z_int() != 0 { try!(write!(f, " wom_z_int"))}
      if self.fifo_oflow_int() != 0 { try!(write!(f, " fifo_oflow_int"))}
      if self.gdrive_int() != 0 { try!(write!(f, " gdrive_int"))}
      if self.data_rdy_int() != 0 { try!(write!(f, " data_rdy_int"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct AccelXoutH(pub u8);

impl AccelXoutH {
  pub fn accel_xout(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_accel_xout(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for AccelXoutH {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for AccelXoutH {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.accel_xout() != 0 { try!(write!(f, " accel_xout=0x{:x}", self.accel_xout()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct AccelXoutL(pub u8);

impl AccelXoutL {
  pub fn accel_xout(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_accel_xout(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for AccelXoutL {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for AccelXoutL {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.accel_xout() != 0 { try!(write!(f, " accel_xout=0x{:x}", self.accel_xout()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct AccelYoutH(pub u8);

impl AccelYoutH {
  pub fn accel_yout(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_accel_yout(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for AccelYoutH {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for AccelYoutH {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.accel_yout() != 0 { try!(write!(f, " accel_yout=0x{:x}", self.accel_yout()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct AccelYoutL(pub u8);

impl AccelYoutL {
  pub fn accel_yout(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_accel_yout(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for AccelYoutL {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for AccelYoutL {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.accel_yout() != 0 { try!(write!(f, " accel_yout=0x{:x}", self.accel_yout()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct AccelZoutH(pub u8);

impl AccelZoutH {
  pub fn accel_zout(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_accel_zout(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for AccelZoutH {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for AccelZoutH {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.accel_zout() != 0 { try!(write!(f, " accel_zout=0x{:x}", self.accel_zout()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct AccelZoutL(pub u8);

impl AccelZoutL {
  pub fn accel_zout(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_accel_zout(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for AccelZoutL {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for AccelZoutL {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.accel_zout() != 0 { try!(write!(f, " accel_zout=0x{:x}", self.accel_zout()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct TempOutH(pub u8);

impl TempOutH {
  pub fn temp_out(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_temp_out(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for TempOutH {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for TempOutH {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.temp_out() != 0 { try!(write!(f, " temp_out=0x{:x}", self.temp_out()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct TempOutL(pub u8);

impl TempOutL {
  pub fn temp_out(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_temp_out(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for TempOutL {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for TempOutL {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.temp_out() != 0 { try!(write!(f, " temp_out=0x{:x}", self.temp_out()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct GyroXoutH(pub u8);

impl GyroXoutH {
  pub fn gyro_xout(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_gyro_xout(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for GyroXoutH {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for GyroXoutH {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.gyro_xout() != 0 { try!(write!(f, " gyro_xout=0x{:x}", self.gyro_xout()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct GyroXoutL(pub u8);

impl GyroXoutL {
  pub fn gyro_xout(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_gyro_xout(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for GyroXoutL {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for GyroXoutL {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.gyro_xout() != 0 { try!(write!(f, " gyro_xout=0x{:x}", self.gyro_xout()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct GyroYoutH(pub u8);

impl GyroYoutH {
  pub fn gyro_yout(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_gyro_yout(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for GyroYoutH {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for GyroYoutH {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.gyro_yout() != 0 { try!(write!(f, " gyro_yout=0x{:x}", self.gyro_yout()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct GyroYoutL(pub u8);

impl GyroYoutL {
  pub fn gyro_yout(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_gyro_yout(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for GyroYoutL {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for GyroYoutL {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.gyro_yout() != 0 { try!(write!(f, " gyro_yout=0x{:x}", self.gyro_yout()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct GyroZoutH(pub u8);

impl GyroZoutH {
  pub fn gyro_zout(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_gyro_zout(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for GyroZoutH {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for GyroZoutH {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.gyro_zout() != 0 { try!(write!(f, " gyro_zout=0x{:x}", self.gyro_zout()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct GyroZoutL(pub u8);

impl GyroZoutL {
  pub fn gyro_zout(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_gyro_zout(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for GyroZoutL {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for GyroZoutL {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.gyro_zout() != 0 { try!(write!(f, " gyro_zout=0x{:x}", self.gyro_zout()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct SelfTestXGyro(pub u8);

impl SelfTestXGyro {
  pub fn xg_st_data(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_xg_st_data(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for SelfTestXGyro {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for SelfTestXGyro {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.xg_st_data() != 0 { try!(write!(f, " xg_st_data=0x{:x}", self.xg_st_data()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct SelfTestYGyro(pub u8);

impl SelfTestYGyro {
  pub fn yg_st_data(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_yg_st_data(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for SelfTestYGyro {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for SelfTestYGyro {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.yg_st_data() != 0 { try!(write!(f, " yg_st_data=0x{:x}", self.yg_st_data()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct SelfTestZGyro(pub u8);

impl SelfTestZGyro {
  pub fn zg_st_data(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_zg_st_data(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for SelfTestZGyro {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for SelfTestZGyro {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.zg_st_data() != 0 { try!(write!(f, " zg_st_data=0x{:x}", self.zg_st_data()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct FifoWmTh1(pub u8);

impl FifoWmTh1 {
  pub fn fifo_wm_th(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x3 // [1:0]
  }

  pub fn set_fifo_wm_th(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for FifoWmTh1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for FifoWmTh1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.fifo_wm_th() != 0 { try!(write!(f, " fifo_wm_th=0x{:x}", self.fifo_wm_th()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct FifoWmTh2(pub u8);

impl FifoWmTh2 {
  pub fn fifo_wm_th(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_fifo_wm_th(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for FifoWmTh2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for FifoWmTh2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.fifo_wm_th() != 0 { try!(write!(f, " fifo_wm_th=0x{:x}", self.fifo_wm_th()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct SignalPathReset(pub u8);

impl SignalPathReset {
  pub fn accel_rst(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }

  pub fn set_accel_rst(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn temp_rst(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }

  pub fn set_temp_rst(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for SignalPathReset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for SignalPathReset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.accel_rst() != 0 { try!(write!(f, " accel_rst"))}
      if self.temp_rst() != 0 { try!(write!(f, " temp_rst"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct AccelIntelCtrl(pub u8);

impl AccelIntelCtrl {
  pub fn accel_intel_en(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }

  pub fn set_accel_intel_en(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn accel_intel_mode(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }

  pub fn set_accel_intel_mode(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn output_limit(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }

  pub fn set_output_limit(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn wom_th_mode(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }

  pub fn set_wom_th_mode(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for AccelIntelCtrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for AccelIntelCtrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.accel_intel_en() != 0 { try!(write!(f, " accel_intel_en"))}
      if self.accel_intel_mode() != 0 { try!(write!(f, " accel_intel_mode"))}
      if self.output_limit() != 0 { try!(write!(f, " output_limit"))}
      if self.wom_th_mode() != 0 { try!(write!(f, " wom_th_mode"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct UserCtrl(pub u8);

impl UserCtrl {
  pub fn fifo_en(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }

  pub fn set_fifo_en(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn fifo_rst(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }

  pub fn set_fifo_rst(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn sig_cond_rst(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }

  pub fn set_sig_cond_rst(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for UserCtrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for UserCtrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.fifo_en() != 0 { try!(write!(f, " fifo_en"))}
      if self.fifo_rst() != 0 { try!(write!(f, " fifo_rst"))}
      if self.sig_cond_rst() != 0 { try!(write!(f, " sig_cond_rst"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct PwrMgmt1(pub u8);

impl PwrMgmt1 {
  pub fn device_offset(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }

  pub fn set_device_offset(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn sleep(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }

  pub fn set_sleep(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn cycle(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }

  pub fn set_cycle(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn gyro_standby(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }

  pub fn set_gyro_standby(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn temp_dis(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }

  pub fn set_temp_dis(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn clksel(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x7 // [2:0]
  }

  pub fn set_clksel(mut self, value: u8) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for PwrMgmt1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for PwrMgmt1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.device_offset() != 0 { try!(write!(f, " device_offset"))}
      if self.sleep() != 0 { try!(write!(f, " sleep"))}
      if self.cycle() != 0 { try!(write!(f, " cycle"))}
      if self.gyro_standby() != 0 { try!(write!(f, " gyro_standby"))}
      if self.temp_dis() != 0 { try!(write!(f, " temp_dis"))}
      if self.clksel() != 0 { try!(write!(f, " clksel=0x{:x}", self.clksel()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct PwrMgmt2(pub u8);

impl PwrMgmt2 {
  pub fn stby_xa(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }

  pub fn set_stby_xa(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn stby_ya(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }

  pub fn set_stby_ya(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn stby_za(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }

  pub fn set_stby_za(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn stby_xg(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }

  pub fn set_stby_xg(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn stby_yg(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }

  pub fn set_stby_yg(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn stby_zg(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }

  pub fn set_stby_zg(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for PwrMgmt2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for PwrMgmt2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.stby_xa() != 0 { try!(write!(f, " stby_xa"))}
      if self.stby_ya() != 0 { try!(write!(f, " stby_ya"))}
      if self.stby_za() != 0 { try!(write!(f, " stby_za"))}
      if self.stby_xg() != 0 { try!(write!(f, " stby_xg"))}
      if self.stby_yg() != 0 { try!(write!(f, " stby_yg"))}
      if self.stby_zg() != 0 { try!(write!(f, " stby_zg"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct I2cIf(pub u8);

impl I2cIf {
  pub fn i2c_if_dis(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }

  pub fn set_i2c_if_dis(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

}

impl ::core::fmt::Display for I2cIf {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for I2cIf {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.i2c_if_dis() != 0 { try!(write!(f, " i2c_if_dis"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct FifoCounth(pub u8);

impl FifoCounth {
  pub fn fifo_count(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_fifo_count(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for FifoCounth {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for FifoCounth {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.fifo_count() != 0 { try!(write!(f, " fifo_count=0x{:x}", self.fifo_count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct FifoCountl(pub u8);

impl FifoCountl {
  pub fn fifo_count(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_fifo_count(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for FifoCountl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for FifoCountl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.fifo_count() != 0 { try!(write!(f, " fifo_count=0x{:x}", self.fifo_count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct FifoRW(pub u8);

impl FifoRW {
  pub fn fifo_data(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_fifo_data(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for FifoRW {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for FifoRW {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.fifo_data() != 0 { try!(write!(f, " fifo_data=0x{:x}", self.fifo_data()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct WhoAmI(pub u8);

impl WhoAmI {
  pub fn whoami(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_whoami(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for WhoAmI {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for WhoAmI {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.whoami() != 0 { try!(write!(f, " whoami=0x{:x}", self.whoami()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct XaOffsetH(pub u8);

impl XaOffsetH {
  pub fn xa_offs(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_xa_offs(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for XaOffsetH {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for XaOffsetH {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.xa_offs() != 0 { try!(write!(f, " xa_offs=0x{:x}", self.xa_offs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct XaOffsetL(pub u8);

impl XaOffsetL {
  pub fn xa_offs(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x7f // [7:1]
  }

  pub fn set_xa_offs(mut self, value: u8) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 1);
     self.0 |= value << 1;
     self
  }

}

impl ::core::fmt::Display for XaOffsetL {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for XaOffsetL {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.xa_offs() != 0 { try!(write!(f, " xa_offs=0x{:x}", self.xa_offs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct YaOffsetH(pub u8);

impl YaOffsetH {
  pub fn ya_offs(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_ya_offs(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for YaOffsetH {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for YaOffsetH {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.ya_offs() != 0 { try!(write!(f, " ya_offs=0x{:x}", self.ya_offs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct YaOffsetL(pub u8);

impl YaOffsetL {
  pub fn ya_offs(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x7f // [7:1]
  }

  pub fn set_ya_offs(mut self, value: u8) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 1);
     self.0 |= value << 1;
     self
  }

}

impl ::core::fmt::Display for YaOffsetL {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for YaOffsetL {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.ya_offs() != 0 { try!(write!(f, " ya_offs=0x{:x}", self.ya_offs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct ZaOffsetH(pub u8);

impl ZaOffsetH {
  pub fn za_offs(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }

  pub fn set_za_offs(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for ZaOffsetH {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for ZaOffsetH {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.za_offs() != 0 { try!(write!(f, " za_offs=0x{:x}", self.za_offs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct ZaOffsetL(pub u8);

impl ZaOffsetL {
  pub fn za_offs(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x7f // [7:1]
  }

  pub fn set_za_offs(mut self, value: u8) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 1);
     self.0 |= value << 1;
     self
  }

}

impl ::core::fmt::Display for ZaOffsetL {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for ZaOffsetL {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:02x}", self.0));
      if self.za_offs() != 0 { try!(write!(f, " za_offs=0x{:x}", self.za_offs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

