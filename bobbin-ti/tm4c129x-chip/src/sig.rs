pub trait Signal<T> {}

pub trait Ccp {}
pub trait SignalCcp<T> {}
pub trait Rx {}
pub trait SignalRx<T> {}
pub trait Tx {}
pub trait SignalTx<T> {}

pub const T0CCP0: T0ccp0 = T0ccp0 {};
pub struct T0ccp0 {}
impl Ccp for T0ccp0 {}

pub const T0CCP1: T0ccp1 = T0ccp1 {};
pub struct T0ccp1 {}
impl Ccp for T0ccp1 {}

pub const T1CCP0: T1ccp0 = T1ccp0 {};
pub struct T1ccp0 {}
impl Ccp for T1ccp0 {}

pub const T1CCP1: T1ccp1 = T1ccp1 {};
pub struct T1ccp1 {}
impl Ccp for T1ccp1 {}

pub const T2CCP0: T2ccp0 = T2ccp0 {};
pub struct T2ccp0 {}
impl Ccp for T2ccp0 {}

pub const T2CCP1: T2ccp1 = T2ccp1 {};
pub struct T2ccp1 {}
impl Ccp for T2ccp1 {}

pub const T3CCP0: T3ccp0 = T3ccp0 {};
pub struct T3ccp0 {}
impl Ccp for T3ccp0 {}

pub const T3CCP1: T3ccp1 = T3ccp1 {};
pub struct T3ccp1 {}
impl Ccp for T3ccp1 {}

pub const T4CCP0: T4ccp0 = T4ccp0 {};
pub struct T4ccp0 {}
impl Ccp for T4ccp0 {}

pub const T4CCP1: T4ccp1 = T4ccp1 {};
pub struct T4ccp1 {}
impl Ccp for T4ccp1 {}

pub const T5CCP0: T5ccp0 = T5ccp0 {};
pub struct T5ccp0 {}
impl Ccp for T5ccp0 {}

pub const T5CCP1: T5ccp1 = T5ccp1 {};
pub struct T5ccp1 {}
impl Ccp for T5ccp1 {}

pub const T6CCP0: T6ccp0 = T6ccp0 {};
pub struct T6ccp0 {}
impl Ccp for T6ccp0 {}

pub const T6CCP1: T6ccp1 = T6ccp1 {};
pub struct T6ccp1 {}
impl Ccp for T6ccp1 {}

pub const T7CCP0: T7ccp0 = T7ccp0 {};
pub struct T7ccp0 {}
impl Ccp for T7ccp0 {}

pub const T7CCP1: T7ccp1 = T7ccp1 {};
pub struct T7ccp1 {}
impl Ccp for T7ccp1 {}

pub const U0RX: U0rx = U0rx {};
pub struct U0rx {}
impl Rx for U0rx {}

pub const U0TX: U0tx = U0tx {};
pub struct U0tx {}
impl Tx for U0tx {}

pub const U1RX: U1rx = U1rx {};
pub struct U1rx {}
impl Rx for U1rx {}

pub const U1TX: U1tx = U1tx {};
pub struct U1tx {}
impl Tx for U1tx {}

pub const U2RX: U2rx = U2rx {};
pub struct U2rx {}
impl Rx for U2rx {}

pub const U2TX: U2tx = U2tx {};
pub struct U2tx {}
impl Tx for U2tx {}

pub const U3RX: U3rx = U3rx {};
pub struct U3rx {}
impl Rx for U3rx {}

pub const U3TX: U3tx = U3tx {};
pub struct U3tx {}
impl Tx for U3tx {}

pub const U4RX: U4rx = U4rx {};
pub struct U4rx {}
impl Rx for U4rx {}

pub const U4TX: U4tx = U4tx {};
pub struct U4tx {}
impl Tx for U4tx {}

pub const U5RX: U5rx = U5rx {};
pub struct U5rx {}
impl Rx for U5rx {}

pub const U5TX: U5tx = U5tx {};
pub struct U5tx {}
impl Tx for U5tx {}

pub const U6RX: U6rx = U6rx {};
pub struct U6rx {}
impl Rx for U6rx {}

pub const U6TX: U6tx = U6tx {};
pub struct U6tx {}
impl Tx for U6tx {}

pub const U7RX: U7rx = U7rx {};
pub struct U7rx {}
impl Rx for U7rx {}

pub const U7TX: U7tx = U7tx {};
pub struct U7tx {}
impl Tx for U7tx {}

pub const I2C9SCL: I2c9scl = I2c9scl {};
pub struct I2c9scl {}

pub const CAN0RX: Can0rx = Can0rx {};
pub struct Can0rx {}

pub const I2C9SDA: I2c9sda = I2c9sda {};
pub struct I2c9sda {}

pub const CAN0TX: Can0tx = Can0tx {};
pub struct Can0tx {}

pub const I2C8SCL: I2c8scl = I2c8scl {};
pub struct I2c8scl {}

pub const SSI0CLK: Ssi0clk = Ssi0clk {};
pub struct Ssi0clk {}

pub const I2C8SDA: I2c8sda = I2c8sda {};
pub struct I2c8sda {}

pub const SSI0FSS: Ssi0fss = Ssi0fss {};
pub struct Ssi0fss {}

pub const I2C7SCL: I2c7scl = I2c7scl {};
pub struct I2c7scl {}

pub const SSI0XDAT0: Ssi0xdat0 = Ssi0xdat0 {};
pub struct Ssi0xdat0 {}

pub const I2C7SDA: I2c7sda = I2c7sda {};
pub struct I2c7sda {}

pub const SSI0XDAT1: Ssi0xdat1 = Ssi0xdat1 {};
pub struct Ssi0xdat1 {}

pub const I2C6SCL: I2c6scl = I2c6scl {};
pub struct I2c6scl {}

pub const USB0EPEN: Usb0epen = Usb0epen {};
pub struct Usb0epen {}

pub const SSI0XDAT2: Ssi0xdat2 = Ssi0xdat2 {};
pub struct Ssi0xdat2 {}

pub const EPI0S8: Epi0s8 = Epi0s8 {};
pub struct Epi0s8 {}

pub const I2C6SDA: I2c6sda = I2c6sda {};
pub struct I2c6sda {}

pub const USB0PFLT: Usb0pflt = Usb0pflt {};
pub struct Usb0pflt {}

pub const SSI0XDAT3: Ssi0xdat3 = Ssi0xdat3 {};
pub struct Ssi0xdat3 {}

pub const EPI0S9: Epi0s9 = Epi0s9 {};
pub struct Epi0s9 {}

pub const USB0ID: Usb0id = Usb0id {};
pub struct Usb0id {}

pub const I2C5SCL: I2c5scl = I2c5scl {};
pub struct I2c5scl {}

pub const CAN1RX: Can1rx = Can1rx {};
pub struct Can1rx {}

pub const USB0VBUS: Usb0vbus = Usb0vbus {};
pub struct Usb0vbus {}

pub const I2C5SDA: I2c5sda = I2c5sda {};
pub struct I2c5sda {}

pub const CAN1TX: Can1tx = Can1tx {};
pub struct Can1tx {}

pub const I2C0SCL: I2c0scl = I2c0scl {};
pub struct I2c0scl {}

pub const USB0STP: Usb0stp = Usb0stp {};
pub struct Usb0stp {}

pub const EPI0S27: Epi0s27 = Epi0s27 {};
pub struct Epi0s27 {}

pub const I2C0SDA: I2c0sda = I2c0sda {};
pub struct I2c0sda {}

pub const USB0CLK: Usb0clk = Usb0clk {};
pub struct Usb0clk {}

pub const EPI0S28: Epi0s28 = Epi0s28 {};
pub struct Epi0s28 {}

pub const AIN10: Ain10 = Ain10 {};
pub struct Ain10 {}

pub const U0CTS: U0cts = U0cts {};
pub struct U0cts {}

pub const SSI1FSS: Ssi1fss = Ssi1fss {};
pub struct Ssi1fss {}

pub const AIN11: Ain11 = Ain11 {};
pub struct Ain11 {}

pub const U0RTS: U0rts = U0rts {};
pub struct U0rts {}

pub const SSI1CLK: Ssi1clk = Ssi1clk {};
pub struct Ssi1clk {}

pub const TCK: Tck = Tck {};
pub struct Tck {}

pub const SWCLK: Swclk = Swclk {};
pub struct Swclk {}

pub const TMS: Tms = Tms {};
pub struct Tms {}

pub const SWDIO: Swdio = Swdio {};
pub struct Swdio {}

pub const TDI: Tdi = Tdi {};
pub struct Tdi {}

pub const TDO: Tdo = Tdo {};
pub struct Tdo {}

pub const SWO: Swo = Swo {};
pub struct Swo {}

pub const C1_NEG: C1Neg = C1Neg {};
pub struct C1Neg {}

pub const EPI0S7: Epi0s7 = Epi0s7 {};
pub struct Epi0s7 {}

pub const C1_POS: C1Pos = C1Pos {};
pub struct C1Pos {}

pub const RTCCLK: Rtcclk = Rtcclk {};
pub struct Rtcclk {}

pub const EPI0S6: Epi0s6 = Epi0s6 {};
pub struct Epi0s6 {}

pub const C0_NEG: C0Neg = C0Neg {};
pub struct C0Neg {}

pub const EPI0S5: Epi0s5 = Epi0s5 {};
pub struct Epi0s5 {}

pub const C0_POS: C0Pos = C0Pos {};
pub struct C0Pos {}

pub const EPI0S4: Epi0s4 = Epi0s4 {};
pub struct Epi0s4 {}

pub const AIN15: Ain15 = Ain15 {};
pub struct Ain15 {}

pub const C0O: C0o = C0o {};
pub struct C0o {}

pub const SSI2XDAT1: Ssi2xdat1 = Ssi2xdat1 {};
pub struct Ssi2xdat1 {}

pub const AIN14: Ain14 = Ain14 {};
pub struct Ain14 {}

pub const C1O: C1o = C1o {};
pub struct C1o {}

pub const SSI2XDAT0: Ssi2xdat0 = Ssi2xdat0 {};
pub struct Ssi2xdat0 {}

pub const AIN13: Ain13 = Ain13 {};
pub struct Ain13 {}

pub const C2O: C2o = C2o {};
pub struct C2o {}

pub const SSI2FSS: Ssi2fss = Ssi2fss {};
pub struct Ssi2fss {}

pub const AIN12: Ain12 = Ain12 {};
pub struct Ain12 {}

pub const SSI2CLK: Ssi2clk = Ssi2clk {};
pub struct Ssi2clk {}

pub const AIN7: Ain7 = Ain7 {};
pub struct Ain7 {}

pub const SSI1XDAT2: Ssi1xdat2 = Ssi1xdat2 {};
pub struct Ssi1xdat2 {}

pub const AIN6: Ain6 = Ain6 {};
pub struct Ain6 {}

pub const SSI1XDAT3: Ssi1xdat3 = Ssi1xdat3 {};
pub struct Ssi1xdat3 {}

pub const AIN5: Ain5 = Ain5 {};
pub struct Ain5 {}

pub const U2RTS: U2rts = U2rts {};
pub struct U2rts {}

pub const SSI2XDAT3: Ssi2xdat3 = Ssi2xdat3 {};
pub struct Ssi2xdat3 {}

pub const AIN4: Ain4 = Ain4 {};
pub struct Ain4 {}

pub const U2CTS: U2cts = U2cts {};
pub struct U2cts {}

pub const NMI: Nmi = Nmi {};
pub struct Nmi {}

pub const SSI2XDAT2: Ssi2xdat2 = Ssi2xdat2 {};
pub struct Ssi2xdat2 {}

pub const AIN3: Ain3 = Ain3 {};
pub struct Ain3 {}

pub const U1RTS: U1rts = U1rts {};
pub struct U1rts {}

pub const AIN2: Ain2 = Ain2 {};
pub struct Ain2 {}

pub const U1DSR: U1dsr = U1dsr {};
pub struct U1dsr {}

pub const AIN1: Ain1 = Ain1 {};
pub struct Ain1 {}

pub const U1DCD: U1dcd = U1dcd {};
pub struct U1dcd {}

pub const AIN0: Ain0 = Ain0 {};
pub struct Ain0 {}

pub const U1DTR: U1dtr = U1dtr {};
pub struct U1dtr {}

pub const AIN9: Ain9 = Ain9 {};
pub struct Ain9 {}

pub const U1RI: U1ri = U1ri {};
pub struct U1ri {}

pub const SSI1XDAT0: Ssi1xdat0 = Ssi1xdat0 {};
pub struct Ssi1xdat0 {}

pub const AIN8: Ain8 = Ain8 {};
pub struct Ain8 {}

pub const SSI1XDAT1: Ssi1xdat1 = Ssi1xdat1 {};
pub struct Ssi1xdat1 {}

pub const EN0LED0: En0led0 = En0led0 {};
pub struct En0led0 {}

pub const M0PWM0: M0pwm0 = M0pwm0 {};
pub struct M0pwm0 {}

pub const SSI3XDAT1: Ssi3xdat1 = Ssi3xdat1 {};
pub struct Ssi3xdat1 {}

pub const TRD2: Trd2 = Trd2 {};
pub struct Trd2 {}

pub const EN0LED2: En0led2 = En0led2 {};
pub struct En0led2 {}

pub const M0PWM1: M0pwm1 = M0pwm1 {};
pub struct M0pwm1 {}

pub const SSI3XDAT0: Ssi3xdat0 = Ssi3xdat0 {};
pub struct Ssi3xdat0 {}

pub const TRD1: Trd1 = Trd1 {};
pub struct Trd1 {}

pub const M0PWM2: M0pwm2 = M0pwm2 {};
pub struct M0pwm2 {}

pub const SSI3FSS: Ssi3fss = Ssi3fss {};
pub struct Ssi3fss {}

pub const TRD0: Trd0 = Trd0 {};
pub struct Trd0 {}

pub const M0PWM3: M0pwm3 = M0pwm3 {};
pub struct M0pwm3 {}

pub const SSI3CLK: Ssi3clk = Ssi3clk {};
pub struct Ssi3clk {}

pub const TRCLK: Trclk = Trclk {};
pub struct Trclk {}

pub const EN0LED1: En0led1 = En0led1 {};
pub struct En0led1 {}

pub const M0FAULT0: M0fault0 = M0fault0 {};
pub struct M0fault0 {}

pub const SSI3XDAT2: Ssi3xdat2 = Ssi3xdat2 {};
pub struct Ssi3xdat2 {}

pub const TRD3: Trd3 = Trd3 {};
pub struct Trd3 {}

pub const I2C1SCL: I2c1scl = I2c1scl {};
pub struct I2c1scl {}

pub const EN0PPS: En0pps = En0pps {};
pub struct En0pps {}

pub const M0PWM4: M0pwm4 = M0pwm4 {};
pub struct M0pwm4 {}

pub const EPI0S11: Epi0s11 = Epi0s11 {};
pub struct Epi0s11 {}

pub const I2C1SDA: I2c1sda = I2c1sda {};
pub struct I2c1sda {}

pub const M0PWM5: M0pwm5 = M0pwm5 {};
pub struct M0pwm5 {}

pub const EPI0S10: Epi0s10 = Epi0s10 {};
pub struct Epi0s10 {}

pub const EPI0S0: Epi0s0 = Epi0s0 {};
pub struct Epi0s0 {}

pub const EPI0S1: Epi0s1 = Epi0s1 {};
pub struct Epi0s1 {}

pub const U0DCD: U0dcd = U0dcd {};
pub struct U0dcd {}

pub const EPI0S2: Epi0s2 = Epi0s2 {};
pub struct Epi0s2 {}

pub const U0DSR: U0dsr = U0dsr {};
pub struct U0dsr {}

pub const EPI0S3: Epi0s3 = Epi0s3 {};
pub struct Epi0s3 {}

pub const AIN16: Ain16 = Ain16 {};
pub struct Ain16 {}

pub const AIN17: Ain17 = Ain17 {};
pub struct Ain17 {}

pub const AIN18: Ain18 = Ain18 {};
pub struct Ain18 {}

pub const U4RTS: U4rts = U4rts {};
pub struct U4rts {}

pub const AIN19: Ain19 = Ain19 {};
pub struct Ain19 {}

pub const U4CTS: U4cts = U4cts {};
pub struct U4cts {}

pub const I2C3SCL: I2c3scl = I2c3scl {};
pub struct I2c3scl {}

pub const M0PWM6: M0pwm6 = M0pwm6 {};
pub struct M0pwm6 {}

pub const EPI0S32: Epi0s32 = Epi0s32 {};
pub struct Epi0s32 {}

pub const I2C3SDA: I2c3sda = I2c3sda {};
pub struct I2c3sda {}

pub const M0PWM7: M0pwm7 = M0pwm7 {};
pub struct M0pwm7 {}

pub const EPI0S31: Epi0s31 = Epi0s31 {};
pub struct Epi0s31 {}

pub const I2C4SCL: I2c4scl = I2c4scl {};
pub struct I2c4scl {}

pub const M0FAULT1: M0fault1 = M0fault1 {};
pub struct M0fault1 {}

pub const EPI0S25: Epi0s25 = Epi0s25 {};
pub struct Epi0s25 {}

pub const U0RI: U0ri = U0ri {};
pub struct U0ri {}

pub const I2C4SDA: I2c4sda = I2c4sda {};
pub struct I2c4sda {}

pub const M0FAULT2: M0fault2 = M0fault2 {};
pub struct M0fault2 {}

pub const EPI0S24: Epi0s24 = Epi0s24 {};
pub struct Epi0s24 {}

pub const I2C2SDA: I2c2sda = I2c2sda {};
pub struct I2c2sda {}

pub const M0FAULT3: M0fault3 = M0fault3 {};
pub struct M0fault3 {}

pub const USB0D0: Usb0d0 = Usb0d0 {};
pub struct Usb0d0 {}

pub const EPI0S16: Epi0s16 = Epi0s16 {};
pub struct Epi0s16 {}

pub const I2C2SCL: I2c2scl = I2c2scl {};
pub struct I2c2scl {}

pub const PHA0: Pha0 = Pha0 {};
pub struct Pha0 {}

pub const USB0D1: Usb0d1 = Usb0d1 {};
pub struct Usb0d1 {}

pub const EPI0S17: Epi0s17 = Epi0s17 {};
pub struct Epi0s17 {}

pub const PHB0: Phb0 = Phb0 {};
pub struct Phb0 {}

pub const USB0D2: Usb0d2 = Usb0d2 {};
pub struct Usb0d2 {}

pub const EPI0S18: Epi0s18 = Epi0s18 {};
pub struct Epi0s18 {}

pub const IDX0: Idx0 = Idx0 {};
pub struct Idx0 {}

pub const USB0D3: Usb0d3 = Usb0d3 {};
pub struct Usb0d3 {}

pub const EPI0S19: Epi0s19 = Epi0s19 {};
pub struct Epi0s19 {}

pub const USB0D4: Usb0d4 = Usb0d4 {};
pub struct Usb0d4 {}

pub const EPI0S26: Epi0s26 = Epi0s26 {};
pub struct Epi0s26 {}

pub const USB0D5: Usb0d5 = Usb0d5 {};
pub struct Usb0d5 {}

pub const EPI0S33: Epi0s33 = Epi0s33 {};
pub struct Epi0s33 {}

pub const USB0DP: Usb0dp = Usb0dp {};
pub struct Usb0dp {}

pub const USB0DM: Usb0dm = Usb0dm {};
pub struct Usb0dm {}

pub const EPI0S15: Epi0s15 = Epi0s15 {};
pub struct Epi0s15 {}

pub const EPI0S14: Epi0s14 = Epi0s14 {};
pub struct Epi0s14 {}

pub const EPI0S13: Epi0s13 = Epi0s13 {};
pub struct Epi0s13 {}

pub const EPI0S12: Epi0s12 = Epi0s12 {};
pub struct Epi0s12 {}

pub const TMPR3: Tmpr3 = Tmpr3 {};
pub struct Tmpr3 {}

pub const TMPR2: Tmpr2 = Tmpr2 {};
pub struct Tmpr2 {}

pub const TMPR1: Tmpr1 = Tmpr1 {};
pub struct Tmpr1 {}

pub const TMPR0: Tmpr0 = Tmpr0 {};
pub struct Tmpr0 {}

pub const U1CTS: U1cts = U1cts {};
pub struct U1cts {}

pub const EPI0S29: Epi0s29 = Epi0s29 {};
pub struct Epi0s29 {}

pub const EPI0S30: Epi0s30 = Epi0s30 {};
pub struct Epi0s30 {}

pub const U3RTS: U3rts = U3rts {};
pub struct U3rts {}

pub const EPI0S34: Epi0s34 = Epi0s34 {};
pub struct Epi0s34 {}

pub const U3CTS: U3cts = U3cts {};
pub struct U3cts {}

pub const EPI0S35: Epi0s35 = Epi0s35 {};
pub struct Epi0s35 {}

pub const C2_POS: C2Pos = C2Pos {};
pub struct C2Pos {}

pub const SSI3XDAT: Ssi3xdat = Ssi3xdat {};
pub struct Ssi3xdat {}

pub const C2_NEG: C2Neg = C2Neg {};
pub struct C2Neg {}

pub const U0DTR: U0dtr = U0dtr {};
pub struct U0dtr {}

pub const USB0NXT: Usb0nxt = Usb0nxt {};
pub struct Usb0nxt {}

pub const USB0DIR: Usb0dir = Usb0dir {};
pub struct Usb0dir {}

pub const USB0D7: Usb0d7 = Usb0d7 {};
pub struct Usb0d7 {}

pub const USB0D6: Usb0d6 = Usb0d6 {};
pub struct Usb0d6 {}

pub const EPI0S20: Epi0s20 = Epi0s20 {};
pub struct Epi0s20 {}

pub const EPI0S21: Epi0s21 = Epi0s21 {};
pub struct Epi0s21 {}

pub const EPI0S22: Epi0s22 = Epi0s22 {};
pub struct Epi0s22 {}

pub const EPI0S23: Epi0s23 = Epi0s23 {};
pub struct Epi0s23 {}

pub const DIVSCLK: Divsclk = Divsclk {};
pub struct Divsclk {}

