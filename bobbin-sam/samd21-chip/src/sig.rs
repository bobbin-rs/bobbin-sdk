pub trait Signal<T> {}

pub trait Pad0 {}
pub trait SignalPad0<T> {}
pub trait Pad1 {}
pub trait SignalPad1<T> {}
pub trait Pad2 {}
pub trait SignalPad2<T> {}
pub trait Pad3 {}
pub trait SignalPad3<T> {}

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

pub const TCC2: Tcc2 = Tcc2 {};
pub struct Tcc2 {}

pub const WO_0: Wo0 = Wo0 {};
pub struct Wo0 {}

pub const EXTINT_1: Extint1 = Extint1 {};
pub struct Extint1 {}

pub const WO_1: Wo1 = Wo1 {};
pub struct Wo1 {}

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

pub const TCC0: Tcc0 = Tcc0 {};
pub struct Tcc0 {}

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

pub const TCC1: Tcc1 = Tcc1 {};
pub struct Tcc1 {}

pub const EXTINT_7: Extint7 = Extint7 {};
pub struct Extint7 {}

pub const AIN_7: Ain7 = Ain7 {};
pub struct Ain7 {}

pub const AIN_3: Ain3 = Ain3 {};
pub struct Ain3 {}

pub const Y_5: Y5 = Y5 {};
pub struct Y5 {}

pub const I2S: I2s = I2s {};
pub struct I2s {}

pub const SD_0: Sd0 = Sd0 {};
pub struct Sd0 {}

pub const NMI: Nmi = Nmi {};
pub struct Nmi {}

pub const AIN_16: Ain16 = Ain16 {};
pub struct Ain16 {}

pub const X_0: X0 = X0 {};
pub struct X0 {}

pub const WO_2: Wo2 = Wo2 {};
pub struct Wo2 {}

pub const SD_1: Sd1 = Sd1 {};
pub struct Sd1 {}

pub const EXTINT_9: Extint9 = Extint9 {};
pub struct Extint9 {}

pub const AIN_17: Ain17 = Ain17 {};
pub struct Ain17 {}

pub const X_1: X1 = X1 {};
pub struct X1 {}

pub const WO_3: Wo3 = Wo3 {};
pub struct Wo3 {}

pub const MCK_0: Mck0 = Mck0 {};
pub struct Mck0 {}

pub const EXTINT_10: Extint10 = Extint10 {};
pub struct Extint10 {}

pub const AIN_18: Ain18 = Ain18 {};
pub struct Ain18 {}

pub const X_2: X2 = X2 {};
pub struct X2 {}

pub const SCK_0: Sck0 = Sck0 {};
pub struct Sck0 {}

pub const GCLK_IO_4: GclkIo4 = GclkIo4 {};
pub struct GclkIo4 {}

pub const EXTINT_11: Extint11 = Extint11 {};
pub struct Extint11 {}

pub const AIN_19: Ain19 = Ain19 {};
pub struct Ain19 {}

pub const X_3: X3 = X3 {};
pub struct X3 {}

pub const FS_0: Fs0 = Fs0 {};
pub struct Fs0 {}

pub const GCLK_IO_5: GclkIo5 = GclkIo5 {};
pub struct GclkIo5 {}

pub const EXTINT_12: Extint12 = Extint12 {};
pub struct Extint12 {}

pub const WO_6: Wo6 = Wo6 {};
pub struct Wo6 {}

pub const AC: Ac = Ac {};
pub struct Ac {}

pub const CMP_0: Cmp0 = Cmp0 {};
pub struct Cmp0 {}

pub const EXTINT_13: Extint13 = Extint13 {};
pub struct Extint13 {}

pub const WO_7: Wo7 = Wo7 {};
pub struct Wo7 {}

pub const CMP_1: Cmp1 = Cmp1 {};
pub struct Cmp1 {}

pub const EXTINT_14: Extint14 = Extint14 {};
pub struct Extint14 {}

pub const TC3: Tc3 = Tc3 {};
pub struct Tc3 {}

pub const WO_4: Wo4 = Wo4 {};
pub struct Wo4 {}

pub const GCLK_IO_0: GclkIo0 = GclkIo0 {};
pub struct GclkIo0 {}

pub const EXTINT_15: Extint15 = Extint15 {};
pub struct Extint15 {}

pub const WO_5: Wo5 = Wo5 {};
pub struct Wo5 {}

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

pub const TC7: Tc7 = Tc7 {};
pub struct Tc7 {}

pub const X_9: X9 = X9 {};
pub struct X9 {}

pub const X_10: X10 = X10 {};
pub struct X10 {}

pub const TC4: Tc4 = Tc4 {};
pub struct Tc4 {}

pub const GCLK_IO_6: GclkIo6 = GclkIo6 {};
pub struct GclkIo6 {}

pub const X_11: X11 = X11 {};
pub struct X11 {}

pub const USB: Usb = Usb {};
pub struct Usb {}

pub const SOF1KHZ: Sof1khz = Sof1khz {};
pub struct Sof1khz {}

pub const GCLK_IO_7: GclkIo7 = GclkIo7 {};
pub struct GclkIo7 {}

pub const TC5: Tc5 = Tc5 {};
pub struct Tc5 {}

pub const DM: Dm = Dm {};
pub struct Dm {}

pub const DP: Dp = Dp {};
pub struct Dp {}

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

pub const TC6: Tc6 = Tc6 {};
pub struct Tc6 {}

pub const AIN_11: Ain11 = Ain11 {};
pub struct Ain11 {}

pub const Y_9: Y9 = Y9 {};
pub struct Y9 {}

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

pub const MCK_1: Mck1 = Mck1 {};
pub struct Mck1 {}

pub const SCK_1: Sck1 = Sck1 {};
pub struct Sck1 {}

pub const X_12: X12 = X12 {};
pub struct X12 {}

pub const FS_1: Fs1 = Fs1 {};
pub struct Fs1 {}

pub const X_13: X13 = X13 {};
pub struct X13 {}

pub const X_14: X14 = X14 {};
pub struct X14 {}

pub const X_15: X15 = X15 {};
pub struct X15 {}

