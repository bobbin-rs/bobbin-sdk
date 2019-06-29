// use bobbin_bits::U4;

use port::*;
// use bobbin_mcu::pin::SetSource;

// pub use bobbin_common::{Pin, AltFn};
// use bobbin_hal::analog::*;
use bobbin_hal::digital::*;

// use chip::sig::{SignalPad0, SignalPad1, SignalPad2, SignalPad3};
// use chip::sig::{SignalWo0, SignalWo1, SignalWo2, SignalWo3, SignalWo4, SignalWo5, SignalWo6, SignalWo7};

// use core::ops::Deref;

impl PortPin {
    // pub fn set_pull_enabled(&self, value: bool) -> &Self {
    //     let value = if value { 1 } else { 0 };
    //     self.port.with_pincfg(self.index, |r| r.set_pullen(value));
    //     self
    // }

    // pub fn set_input_enabled(&self, value: bool) -> &Self {
    //     self.port.with_pincfg(self.index, |r| r.set_inen(value));
    //     self
    // }

    // pub fn set_pmux_enabled(&self, value: bool) -> &Self {
    //     let value = if value { 1 } else { 0 };
    //     self.port.with_pincfg(self.index, |r| r.set_pmuxen(value));
    //     self
    // }

    pub fn set_dir_output(&self) -> &Self {
        self.port.set_dirset(|r| r.set_dirset(self.index, 1));
        self
    }

    pub fn set_dir_input(&self) -> &Self {
        self.port.set_dirclr(|r| r.set_dirclr(self.index, 1));
        self
    }

    // pub fn set_pmux<V: Into<U4>>(&self, value: V) -> &Self {
    //     let pin = self.index;
    //     let pin_row = pin >> 1;
    //     let pin_col = pin & 1;        
    //     self.port.with_pmux(pin_row, |r| r.set_pmux(pin_col, value));
    //     self
    // }

    // pub fn set_mode_input(&self) -> &Self {
    //     self.set_dir_input().set_input_enabled(true).set_pmux_enabled(false).set_pmux(0)
    // }

    // pub fn set_mode_output(&self) -> &Self {
    //     self.set_dir_output().set_input_enabled(false).set_pmux_enabled(false).set_pmux(0)
    // }

    // pub fn set_mode_pmux<V: Into<U4>>(&self, value: V) -> &Self {
    //     self.set_pmux_enabled(true).set_pmux(value)
    // }
}

impl DigitalInput for PortPin {
    fn input(&self) -> bool {
        self.port._in()._in(self.index) != 0
    }
}

impl DigitalOutput for PortPin {
    fn output(&self) -> bool {
        self.port.out().out(self.index) != 0
    }
    
    fn set_output(&self, value: bool) -> &Self {
        if value {
            self.port.set_outset(|r| r.set_outset(self.index, 1))
        } else {
            self.port.set_outclr(|r| r.set_outclr(self.index, 1))
        };
        self
    }
    fn toggle_output(&self) -> &Self {
        self.port.set_outtgl(|r| r.set_outtgl(self.index, 1));
        self
        // self.set_output(!self.output())
    }    
}

// impl SetSource for PortPin {
//     fn set_source<V: Into<U4>>(&self, src: V) {
//         self.set_mode_pmux(src);
//     }
// }


// pub trait ModePad0<SIG, PERIPH> {
//     fn mode_pad_0(&self, _: &PERIPH) -> &Self;
// }

// pub trait ModePad1<SIG, PERIPH> {
//     fn mode_pad_1(&self, _: &PERIPH) -> &Self;
// }

// pub trait ModePad2<SIG, PERIPH> {
//     fn mode_pad_2(&self, _: &PERIPH) -> &Self;
// }

// pub trait ModePad3<SIG, PERIPH> {
//     fn mode_pad_3(&self, _: &PERIPH) -> &Self;
// }

// pub trait ModeWo0<SIG, PERIPH> {
//     fn mode_wo_0(&self, _: &PERIPH) -> &Self;
// }

// pub trait ModeWo1<SIG, PERIPH> {
//     fn mode_wo_1(&self, _: &PERIPH) -> &Self;
// }

// pub trait ModeWo2<SIG, PERIPH> {
//     fn mode_wo_2(&self, _: &PERIPH) -> &Self;
// }

// pub trait ModeWo3<SIG, PERIPH> {
//     fn mode_wo_3(&self, _: &PERIPH) -> &Self;
// }

// pub trait ModeWo4<SIG, PERIPH> {
//     fn mode_wo_4(&self, _: &PERIPH) -> &Self;
// }

// pub trait ModeWo5<SIG, PERIPH> {
//     fn mode_wo_5(&self, _: &PERIPH) -> &Self;
// }

// pub trait ModeWo6<SIG, PERIPH> {
//     fn mode_wo_6(&self, _: &PERIPH) -> &Self;
// }

// pub trait ModeWo7<SIG, PERIPH> {
//     fn mode_wo_7(&self, _: &PERIPH) -> &Self;
// }

// impl<PERIPH, PIN, SIG> ModePad0<SIG, PERIPH> for PIN where PERIPH: SignalPad0<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
//     fn mode_pad_0(&self, _: &PERIPH) -> &Self {
//         self.set_mode_pmux(self.alt_fn());
//         self
//     }
// }

// impl<PERIPH, PIN, SIG> ModePad1<SIG, PERIPH> for PIN where PERIPH: SignalPad1<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
//     fn mode_pad_1(&self, _: &PERIPH) -> &Self {
//         self.set_mode_pmux(self.alt_fn());
//         self
//     }
// }

// impl<PERIPH, PIN, SIG> ModePad2<SIG, PERIPH> for PIN where PERIPH: SignalPad2<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
//     fn mode_pad_2(&self, _: &PERIPH) -> &Self {
//         self.set_mode_pmux(self.alt_fn());
//         self
//     }
// }

// impl<PERIPH, PIN, SIG> ModePad3<SIG, PERIPH> for PIN where PERIPH: SignalPad3<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
//     fn mode_pad_3(&self, _: &PERIPH) -> &Self {
//         self.set_mode_pmux(self.alt_fn());
//         self
//     }
// }

// impl<PERIPH, PIN, SIG> ModeWo0<SIG, PERIPH> for PIN where PERIPH: SignalWo0<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
//     fn mode_wo_0(&self, _: &PERIPH) -> &Self {
//         self.set_mode_pmux(self.alt_fn());
//         self
//     }
// }

// impl<PERIPH, PIN, SIG> ModeWo1<SIG, PERIPH> for PIN where PERIPH: SignalWo1<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
//     fn mode_wo_1(&self, _: &PERIPH) -> &Self {
//         self.set_mode_pmux(self.alt_fn());
//         self
//     }
// }

// impl<PERIPH, PIN, SIG> ModeWo2<SIG, PERIPH> for PIN where PERIPH: SignalWo2<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
//     fn mode_wo_2(&self, _: &PERIPH) -> &Self {
//         self.set_mode_pmux(self.alt_fn());
//         self
//     }
// }

// impl<PERIPH, PIN, SIG> ModeWo3<SIG, PERIPH> for PIN where PERIPH: SignalWo3<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
//     fn mode_wo_3(&self, _: &PERIPH) -> &Self {
//         self.set_mode_pmux(self.alt_fn());
//         self
//     }
// }

// impl<PERIPH, PIN, SIG> ModeWo4<SIG, PERIPH> for PIN where PERIPH: SignalWo4<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
//     fn mode_wo_4(&self, _: &PERIPH) -> &Self {
//         self.set_mode_pmux(self.alt_fn());
//         self
//     }
// }

// impl<PERIPH, PIN, SIG> ModeWo5<SIG, PERIPH> for PIN where PERIPH: SignalWo5<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
//     fn mode_wo_5(&self, _: &PERIPH) -> &Self {
//         self.set_mode_pmux(self.alt_fn());
//         self
//     }
// }

// impl<PERIPH, PIN, SIG> ModeWo6<SIG, PERIPH> for PIN where PERIPH: SignalWo6<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
//     fn mode_wo_6(&self, _: &PERIPH) -> &Self {
//         self.set_mode_pmux(self.alt_fn());
//         self
//     }
// }

// impl<PERIPH, PIN, SIG> ModeWo7<SIG, PERIPH> for PIN where PERIPH: SignalWo7<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
//     fn mode_wo_7(&self, _: &PERIPH) -> &Self {
//         self.set_mode_pmux(self.alt_fn());
//         self
//     }
// }


// pub trait Ain<SIG, PERIPH> {
//     fn mode_ain(&self, _: &PERIPH) -> &Self;
// }

// impl<PERIPH, PIN, SIG> Ain<SIG, PERIPH> for PIN where PERIPH: SignalAin<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
//     fn mode_ain(&self, _: &PERIPH) -> &Self {
//         self.set_mode_pmux(self.alt_fn());
//         self
//     }
// }