pub use chip::port::*;
use chip::sig::{SignalPad0, SignalPad1, SignalPad2, SignalPad3};
pub use pm::PmEnabled;

use chip::port::Pin;

pub trait PinExt {
    fn set_pull_enabled(&self, value: bool) -> &Self;
    fn set_pmux_enabled(&self, value: bool) -> &Self;
    fn set_dir_output(&self) -> &Self;
    fn set_dir_input(&self) -> &Self;
    fn set_pmux(&self, value: usize) -> &Self;
    fn set_mode_input(&self) -> &Self;
    fn set_mode_output(&self) -> &Self;
    fn set_mode_pmux(&self, value: usize) -> &Self;
    fn output(&self) -> bool;
    fn set_output(&self, bool) -> &Self;
    fn toggle_output(&self) -> &Self;
    fn input(&self) -> bool;
}

impl<P, T> PinExt for Pin<P, T> {
    fn set_pull_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.port.with_pincfg(self.index, |r| r.set_pullen(value));
        self
    }

    fn set_pmux_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.port.with_pincfg(self.index, |r| r.set_pmuxen(value));
        self
    }

    fn set_dir_output(&self) -> &Self {
        self.port.set_dirset(Dirset(0).set_dirset(self.index, 1));
        self
    }

    fn set_dir_input(&self) -> &Self {
        self.port.set_dirclr(Dirclr(0).set_dirclr(self.index, 1));
        self
    }

    fn set_pmux(&self, value: usize) -> &Self {
        let pin = self.index;
        let pin_row = pin >> 1;
        let pin_col = pin & 1;        
        self.port.with_pmux(pin_row, |r| r.set_pmux(pin_col, value as u8));
        self
    }

    fn set_mode_input(&self) -> &Self {
        self.set_dir_input().set_pmux(0)
    }

    fn set_mode_output(&self) -> &Self {
        self.set_dir_output().set_pmux(0)
    }

    fn set_mode_pmux(&self, value: usize) -> &Self {
        self.set_pmux_enabled(true).set_pmux(value)
    }

    fn output(&self) -> bool {
        self.port.out().out(self.index) != 0
    }
    
    fn set_output(&self, value: bool) -> &Self {
        if value {
            self.port.set_outset(Outset(0).set_outset(self.index, 1))
        } else {
            self.port.set_outclr(Outclr(0).set_outclr(self.index, 1))
        };
        self
    }
    fn toggle_output(&self) -> &Self {
        self.set_output(!self.output())
    }

    fn input(&self) -> bool {
        self.port._in()._in(self.index) != 0
    }

}

pub trait ModePad0<T, S> {
    fn mode_pad_0(&self, _: &S) -> &Self;
}

pub trait ModePad1<T, S> {
    fn mode_pad_1(&self, _: &S) -> &Self;
}

pub trait ModePad2<T, S> {
    fn mode_pad_2(&self, _: &S) -> &Self;
}
pub trait ModePad3<T, S> {
    fn mode_pad_3(&self, _: &S) -> &Self;
}

impl<P, O, S, T> ModePad0<T, S> for Pin<P, O> where S: SignalPad0<T>, P: AltFn<T> {
    fn mode_pad_0(&self, _: &S) -> &Self {
        self.set_mode_pmux(self.id.alt_fn());
        self
    }
}

impl<P, O, S, T> ModePad1<T, S> for Pin<P, O> where S: SignalPad1<T>, P: AltFn<T> {
    fn mode_pad_1(&self, _: &S) -> &Self {
        self.set_mode_pmux(self.id.alt_fn());
        self
    }
}

impl<P, O, S, T> ModePad2<T, S> for Pin<P, O> where S: SignalPad2<T>, P: AltFn<T> {
    fn mode_pad_2(&self, _: &S) -> &Self {
        self.set_mode_pmux(self.id.alt_fn());
        self
    }
}

impl<P, O, S, T> ModePad3<T, S> for Pin<P, O> where S: SignalPad3<T>, P: AltFn<T> {
    fn mode_pad_3(&self, _: &S) -> &Self {
        self.set_mode_pmux(self.id.alt_fn());
        self
    }
}
