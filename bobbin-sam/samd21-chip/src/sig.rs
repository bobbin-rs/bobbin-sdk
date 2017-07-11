//! Signals

pub trait Signal<T> {}

pub trait Wo0 {}
pub trait SignalWo0<T> {}
pub trait Wo1 {}
pub trait SignalWo1<T> {}
pub trait Wo2 {}
pub trait SignalWo2<T> {}
pub trait Wo3 {}
pub trait SignalWo3<T> {}
pub trait Wo4 {}
pub trait SignalWo4<T> {}
pub trait Wo5 {}
pub trait SignalWo5<T> {}
pub trait Wo6 {}
pub trait SignalWo6<T> {}
pub trait Wo7 {}
pub trait SignalWo7<T> {}
pub trait Wo {}
pub trait SignalWo<T> {}
pub trait Pad0 {}
pub trait SignalPad0<T> {}
pub trait Pad1 {}
pub trait SignalPad1<T> {}
pub trait Pad2 {}
pub trait SignalPad2<T> {}
pub trait Pad3 {}
pub trait SignalPad3<T> {}

pub const TCC0_WO_0: Tcc0Wo0 = Tcc0Wo0 {};
pub struct Tcc0Wo0 {}
impl Wo0 for Tcc0Wo0 {}

pub const TCC0_WO_1: Tcc0Wo1 = Tcc0Wo1 {};
pub struct Tcc0Wo1 {}
impl Wo1 for Tcc0Wo1 {}

pub const TCC0_WO_2: Tcc0Wo2 = Tcc0Wo2 {};
pub struct Tcc0Wo2 {}
impl Wo2 for Tcc0Wo2 {}

pub const TCC0_WO_3: Tcc0Wo3 = Tcc0Wo3 {};
pub struct Tcc0Wo3 {}
impl Wo3 for Tcc0Wo3 {}

pub const TCC0_WO_4: Tcc0Wo4 = Tcc0Wo4 {};
pub struct Tcc0Wo4 {}
impl Wo4 for Tcc0Wo4 {}

pub const TCC0_WO_5: Tcc0Wo5 = Tcc0Wo5 {};
pub struct Tcc0Wo5 {}
impl Wo5 for Tcc0Wo5 {}

pub const TCC0_WO_6: Tcc0Wo6 = Tcc0Wo6 {};
pub struct Tcc0Wo6 {}
impl Wo6 for Tcc0Wo6 {}

pub const TCC0_WO_7: Tcc0Wo7 = Tcc0Wo7 {};
pub struct Tcc0Wo7 {}
impl Wo7 for Tcc0Wo7 {}

pub const TCC1_WO_0: Tcc1Wo0 = Tcc1Wo0 {};
pub struct Tcc1Wo0 {}
impl Wo0 for Tcc1Wo0 {}

pub const TCC1_WO_1: Tcc1Wo1 = Tcc1Wo1 {};
pub struct Tcc1Wo1 {}
impl Wo1 for Tcc1Wo1 {}

pub const TCC1_WO_2: Tcc1Wo2 = Tcc1Wo2 {};
pub struct Tcc1Wo2 {}
impl Wo2 for Tcc1Wo2 {}

pub const TCC1_WO_3: Tcc1Wo3 = Tcc1Wo3 {};
pub struct Tcc1Wo3 {}
impl Wo3 for Tcc1Wo3 {}

pub const TCC1_WO_4: Tcc1Wo4 = Tcc1Wo4 {};
pub struct Tcc1Wo4 {}
impl Wo4 for Tcc1Wo4 {}

pub const TCC1_WO_5: Tcc1Wo5 = Tcc1Wo5 {};
pub struct Tcc1Wo5 {}
impl Wo5 for Tcc1Wo5 {}

pub const TCC1_WO_6: Tcc1Wo6 = Tcc1Wo6 {};
pub struct Tcc1Wo6 {}
impl Wo6 for Tcc1Wo6 {}

pub const TCC1_WO_7: Tcc1Wo7 = Tcc1Wo7 {};
pub struct Tcc1Wo7 {}
impl Wo7 for Tcc1Wo7 {}

pub const TCC2_WO_0: Tcc2Wo0 = Tcc2Wo0 {};
pub struct Tcc2Wo0 {}
impl Wo0 for Tcc2Wo0 {}

pub const TCC2_WO_1: Tcc2Wo1 = Tcc2Wo1 {};
pub struct Tcc2Wo1 {}
impl Wo1 for Tcc2Wo1 {}

pub const TCC2_WO_2: Tcc2Wo2 = Tcc2Wo2 {};
pub struct Tcc2Wo2 {}
impl Wo2 for Tcc2Wo2 {}

pub const TCC2_WO_3: Tcc2Wo3 = Tcc2Wo3 {};
pub struct Tcc2Wo3 {}
impl Wo3 for Tcc2Wo3 {}

pub const TCC2_WO_4: Tcc2Wo4 = Tcc2Wo4 {};
pub struct Tcc2Wo4 {}
impl Wo4 for Tcc2Wo4 {}

pub const TCC2_WO_5: Tcc2Wo5 = Tcc2Wo5 {};
pub struct Tcc2Wo5 {}
impl Wo5 for Tcc2Wo5 {}

pub const TCC2_WO_6: Tcc2Wo6 = Tcc2Wo6 {};
pub struct Tcc2Wo6 {}
impl Wo6 for Tcc2Wo6 {}

pub const TCC2_WO_7: Tcc2Wo7 = Tcc2Wo7 {};
pub struct Tcc2Wo7 {}
impl Wo7 for Tcc2Wo7 {}

pub const TC3_WO_0: Tc3Wo0 = Tc3Wo0 {};
pub struct Tc3Wo0 {}
impl Wo for Tc3Wo0 {}

pub const TC3_WO_1: Tc3Wo1 = Tc3Wo1 {};
pub struct Tc3Wo1 {}
impl Wo for Tc3Wo1 {}

pub const TC4_WO_0: Tc4Wo0 = Tc4Wo0 {};
pub struct Tc4Wo0 {}
impl Wo for Tc4Wo0 {}

pub const TC4_WO_1: Tc4Wo1 = Tc4Wo1 {};
pub struct Tc4Wo1 {}
impl Wo for Tc4Wo1 {}

pub const TC5_WO_0: Tc5Wo0 = Tc5Wo0 {};
pub struct Tc5Wo0 {}
impl Wo for Tc5Wo0 {}

pub const TC5_WO_1: Tc5Wo1 = Tc5Wo1 {};
pub struct Tc5Wo1 {}
impl Wo for Tc5Wo1 {}

pub const SERCOM0_PAD_0: Sercom0Pad0 = Sercom0Pad0 {};
pub struct Sercom0Pad0 {}
impl Pad0 for Sercom0Pad0 {}

pub const SERCOM0_PAD_1: Sercom0Pad1 = Sercom0Pad1 {};
pub struct Sercom0Pad1 {}
impl Pad1 for Sercom0Pad1 {}

pub const SERCOM0_PAD_2: Sercom0Pad2 = Sercom0Pad2 {};
pub struct Sercom0Pad2 {}
impl Pad2 for Sercom0Pad2 {}

pub const SERCOM0_PAD_3: Sercom0Pad3 = Sercom0Pad3 {};
pub struct Sercom0Pad3 {}
impl Pad3 for Sercom0Pad3 {}

pub const SERCOM1_PAD_0: Sercom1Pad0 = Sercom1Pad0 {};
pub struct Sercom1Pad0 {}
impl Pad0 for Sercom1Pad0 {}

pub const SERCOM1_PAD_1: Sercom1Pad1 = Sercom1Pad1 {};
pub struct Sercom1Pad1 {}
impl Pad1 for Sercom1Pad1 {}

pub const SERCOM1_PAD_2: Sercom1Pad2 = Sercom1Pad2 {};
pub struct Sercom1Pad2 {}
impl Pad2 for Sercom1Pad2 {}

pub const SERCOM1_PAD_3: Sercom1Pad3 = Sercom1Pad3 {};
pub struct Sercom1Pad3 {}
impl Pad3 for Sercom1Pad3 {}

pub const SERCOM2_PAD_0: Sercom2Pad0 = Sercom2Pad0 {};
pub struct Sercom2Pad0 {}
impl Pad0 for Sercom2Pad0 {}

pub const SERCOM2_PAD_1: Sercom2Pad1 = Sercom2Pad1 {};
pub struct Sercom2Pad1 {}
impl Pad1 for Sercom2Pad1 {}

pub const SERCOM2_PAD_2: Sercom2Pad2 = Sercom2Pad2 {};
pub struct Sercom2Pad2 {}
impl Pad2 for Sercom2Pad2 {}

pub const SERCOM2_PAD_3: Sercom2Pad3 = Sercom2Pad3 {};
pub struct Sercom2Pad3 {}
impl Pad3 for Sercom2Pad3 {}

pub const SERCOM3_PAD_0: Sercom3Pad0 = Sercom3Pad0 {};
pub struct Sercom3Pad0 {}
impl Pad0 for Sercom3Pad0 {}

pub const SERCOM3_PAD_1: Sercom3Pad1 = Sercom3Pad1 {};
pub struct Sercom3Pad1 {}
impl Pad1 for Sercom3Pad1 {}

pub const SERCOM3_PAD_2: Sercom3Pad2 = Sercom3Pad2 {};
pub struct Sercom3Pad2 {}
impl Pad2 for Sercom3Pad2 {}

pub const SERCOM3_PAD_3: Sercom3Pad3 = Sercom3Pad3 {};
pub struct Sercom3Pad3 {}
impl Pad3 for Sercom3Pad3 {}

pub const SERCOM4_PAD_0: Sercom4Pad0 = Sercom4Pad0 {};
pub struct Sercom4Pad0 {}
impl Pad0 for Sercom4Pad0 {}

pub const SERCOM4_PAD_1: Sercom4Pad1 = Sercom4Pad1 {};
pub struct Sercom4Pad1 {}
impl Pad1 for Sercom4Pad1 {}

pub const SERCOM4_PAD_2: Sercom4Pad2 = Sercom4Pad2 {};
pub struct Sercom4Pad2 {}
impl Pad2 for Sercom4Pad2 {}

pub const SERCOM4_PAD_3: Sercom4Pad3 = Sercom4Pad3 {};
pub struct Sercom4Pad3 {}
impl Pad3 for Sercom4Pad3 {}

pub const SERCOM5_PAD_0: Sercom5Pad0 = Sercom5Pad0 {};
pub struct Sercom5Pad0 {}
impl Pad0 for Sercom5Pad0 {}

pub const SERCOM5_PAD_1: Sercom5Pad1 = Sercom5Pad1 {};
pub struct Sercom5Pad1 {}
impl Pad1 for Sercom5Pad1 {}

pub const SERCOM5_PAD_2: Sercom5Pad2 = Sercom5Pad2 {};
pub struct Sercom5Pad2 {}
impl Pad2 for Sercom5Pad2 {}

pub const SERCOM5_PAD_3: Sercom5Pad3 = Sercom5Pad3 {};
pub struct Sercom5Pad3 {}
impl Pad3 for Sercom5Pad3 {}

pub const EXTINT_0: Extint0 = Extint0 {};
pub struct Extint0 {}

pub const EXTINT_1: Extint1 = Extint1 {};
pub struct Extint1 {}

pub const EXTINT_2: Extint2 = Extint2 {};
pub struct Extint2 {}

pub const AIN_0: Ain0 = Ain0 {};
pub struct Ain0 {}

pub const Y_0: Y0 = Y0 {};
pub struct Y0 {}

pub const VOUT: Vout = Vout {};
pub struct Vout {}

pub const EXTINT_3: Extint3 = Extint3 {};
pub struct Extint3 {}

pub const ADC: Adc = Adc {};
pub struct Adc {}

pub const VREFADAC: Vrefadac = Vrefadac {};
pub struct Vrefadac {}

pub const VREFA: Vrefa = Vrefa {};
pub struct Vrefa {}

pub const AIN_1: Ain1 = Ain1 {};
pub struct Ain1 {}

pub const Y_1: Y1 = Y1 {};
pub struct Y1 {}

pub const EXTINT_4: Extint4 = Extint4 {};
pub struct Extint4 {}

pub const VREFB: Vrefb = Vrefb {};
pub struct Vrefb {}

pub const AIN_4: Ain4 = Ain4 {};
pub struct Ain4 {}

pub const Y_2: Y2 = Y2 {};
pub struct Y2 {}

pub const EXTINT_5: Extint5 = Extint5 {};
pub struct Extint5 {}

pub const AIN_5: Ain5 = Ain5 {};
pub struct Ain5 {}

pub const Y_3: Y3 = Y3 {};
pub struct Y3 {}

pub const EXTINT_6: Extint6 = Extint6 {};
pub struct Extint6 {}

pub const AIN_6: Ain6 = Ain6 {};
pub struct Ain6 {}

pub const AIN_2: Ain2 = Ain2 {};
pub struct Ain2 {}

pub const Y_4: Y4 = Y4 {};
pub struct Y4 {}

pub const EXTINT_7: Extint7 = Extint7 {};
pub struct Extint7 {}

pub const AIN_7: Ain7 = Ain7 {};
pub struct Ain7 {}

pub const AIN_3: Ain3 = Ain3 {};
pub struct Ain3 {}

pub const Y_5: Y5 = Y5 {};
pub struct Y5 {}

pub const I2S_SD_0: I2sSd0 = I2sSd0 {};
pub struct I2sSd0 {}

pub const NMI: Nmi = Nmi {};
pub struct Nmi {}

pub const AIN_16: Ain16 = Ain16 {};
pub struct Ain16 {}

pub const X_0: X0 = X0 {};
pub struct X0 {}

pub const I2S_SD_1: I2sSd1 = I2sSd1 {};
pub struct I2sSd1 {}

pub const EXTINT_9: Extint9 = Extint9 {};
pub struct Extint9 {}

pub const AIN_17: Ain17 = Ain17 {};
pub struct Ain17 {}

pub const X_1: X1 = X1 {};
pub struct X1 {}

pub const I2S_MCK_0: I2sMck0 = I2sMck0 {};
pub struct I2sMck0 {}

pub const EXTINT_10: Extint10 = Extint10 {};
pub struct Extint10 {}

pub const AIN_18: Ain18 = Ain18 {};
pub struct Ain18 {}

pub const X_2: X2 = X2 {};
pub struct X2 {}

pub const I2S_SCK_0: I2sSck0 = I2sSck0 {};
pub struct I2sSck0 {}

pub const GCLK_IO_4: GclkIo4 = GclkIo4 {};
pub struct GclkIo4 {}

pub const EXTINT_11: Extint11 = Extint11 {};
pub struct Extint11 {}

pub const AIN_19: Ain19 = Ain19 {};
pub struct Ain19 {}

pub const X_3: X3 = X3 {};
pub struct X3 {}

pub const I2S_FS_0: I2sFs0 = I2sFs0 {};
pub struct I2sFs0 {}

pub const GCLK_IO_5: GclkIo5 = GclkIo5 {};
pub struct GclkIo5 {}

pub const EXTINT_12: Extint12 = Extint12 {};
pub struct Extint12 {}

pub const AC_CMP_0: AcCmp0 = AcCmp0 {};
pub struct AcCmp0 {}

pub const EXTINT_13: Extint13 = Extint13 {};
pub struct Extint13 {}

pub const AC_CMP_1: AcCmp1 = AcCmp1 {};
pub struct AcCmp1 {}

pub const EXTINT_14: Extint14 = Extint14 {};
pub struct Extint14 {}

pub const GCLK_IO_0: GclkIo0 = GclkIo0 {};
pub struct GclkIo0 {}

pub const EXTINT_15: Extint15 = Extint15 {};
pub struct Extint15 {}

pub const GCLK_IO_1: GclkIo1 = GclkIo1 {};
pub struct GclkIo1 {}

pub const X_4: X4 = X4 {};
pub struct X4 {}

pub const GCLK_IO_2: GclkIo2 = GclkIo2 {};
pub struct GclkIo2 {}

pub const X_5: X5 = X5 {};
pub struct X5 {}

pub const GCLK_IO_3: GclkIo3 = GclkIo3 {};
pub struct GclkIo3 {}

pub const X_6: X6 = X6 {};
pub struct X6 {}

pub const X_7: X7 = X7 {};
pub struct X7 {}

pub const X_8: X8 = X8 {};
pub struct X8 {}

pub const TC7_WO_0: Tc7Wo0 = Tc7Wo0 {};
pub struct Tc7Wo0 {}

pub const X_9: X9 = X9 {};
pub struct X9 {}

pub const TC7_WO_1: Tc7Wo1 = Tc7Wo1 {};
pub struct Tc7Wo1 {}

pub const X_10: X10 = X10 {};
pub struct X10 {}

pub const GCLK_IO_6: GclkIo6 = GclkIo6 {};
pub struct GclkIo6 {}

pub const X_11: X11 = X11 {};
pub struct X11 {}

pub const USB_SOF1KHZ: UsbSof1khz = UsbSof1khz {};
pub struct UsbSof1khz {}

pub const GCLK_IO_7: GclkIo7 = GclkIo7 {};
pub struct GclkIo7 {}

pub const USB_DM: UsbDm = UsbDm {};
pub struct UsbDm {}

pub const USB_DP: UsbDp = UsbDp {};
pub struct UsbDp {}

pub const EXTINT_8: Extint8 = Extint8 {};
pub struct Extint8 {}

pub const SWCLK: Swclk = Swclk {};
pub struct Swclk {}

pub const SWDIO: Swdio = Swdio {};
pub struct Swdio {}

pub const AIN_8: Ain8 = Ain8 {};
pub struct Ain8 {}

pub const Y_6: Y6 = Y6 {};
pub struct Y6 {}

pub const AIN_9: Ain9 = Ain9 {};
pub struct Ain9 {}

pub const Y_7: Y7 = Y7 {};
pub struct Y7 {}

pub const AIN_10: Ain10 = Ain10 {};
pub struct Ain10 {}

pub const Y_8: Y8 = Y8 {};
pub struct Y8 {}

pub const TC6_WO_0: Tc6Wo0 = Tc6Wo0 {};
pub struct Tc6Wo0 {}

pub const AIN_11: Ain11 = Ain11 {};
pub struct Ain11 {}

pub const Y_9: Y9 = Y9 {};
pub struct Y9 {}

pub const TC6_WO_1: Tc6Wo1 = Tc6Wo1 {};
pub struct Tc6Wo1 {}

pub const AIN_12: Ain12 = Ain12 {};
pub struct Ain12 {}

pub const Y_10: Y10 = Y10 {};
pub struct Y10 {}

pub const AIN_13: Ain13 = Ain13 {};
pub struct Ain13 {}

pub const Y_11: Y11 = Y11 {};
pub struct Y11 {}

pub const AIN_14: Ain14 = Ain14 {};
pub struct Ain14 {}

pub const Y_12: Y12 = Y12 {};
pub struct Y12 {}

pub const AIN_15: Ain15 = Ain15 {};
pub struct Ain15 {}

pub const Y_13: Y13 = Y13 {};
pub struct Y13 {}

pub const Y_14: Y14 = Y14 {};
pub struct Y14 {}

pub const Y_15: Y15 = Y15 {};
pub struct Y15 {}

pub const I2S_MCK_1: I2sMck1 = I2sMck1 {};
pub struct I2sMck1 {}

pub const I2S_SCK_1: I2sSck1 = I2sSck1 {};
pub struct I2sSck1 {}

pub const X_12: X12 = X12 {};
pub struct X12 {}

pub const I2S_FS_1: I2sFs1 = I2sFs1 {};
pub struct I2sFs1 {}

pub const X_13: X13 = X13 {};
pub struct X13 {}

pub const X_14: X14 = X14 {};
pub struct X14 {}

pub const X_15: X15 = X15 {};
pub struct X15 {}

