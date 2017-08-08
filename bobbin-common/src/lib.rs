#![no_std]

pub extern crate bobbin_bits as bits;

pub struct Pin<T, P>(T, P, usize);

impl<T, P> Pin<T, P> {
    pub fn id(&self) -> &T { &self.0 }
    pub fn periph(&self) -> &P { &self.1 }
    pub fn index(&self) -> usize { self.2 }
    
}

pub struct Channel<T, P>(T, P, usize);

impl<T, P> Channel<T, P> {
    pub fn id(&self) -> &T { &self.0 }
    pub fn periph(&self) -> &P { &self.1 }
    pub fn index(&self) -> usize { self.2 }    
}

pub trait AltFn<S> {
    fn alt_fn(&self) -> usize;
}

impl<T, P, S> AltFn<S> for Pin<T, P> where T: AltFn<S> {
    fn alt_fn(&self) -> usize { self.id().alt_fn() }
}

pub enum PinMode {
    Analog,
    Input,
    Output,
    AltFn(usize),
}

pub trait PortConfigMode {
    fn mode(&self, index: usize) -> PinMode;
    fn set_mode(&self, index: usize, value: PinMode) -> &Self;
}

pub trait PinConfigMode {
    fn mode(&self) -> PinMode;
    fn set_mode(&self, value: PinMode) -> &Self;
}

impl<T, P> PinConfigMode for Pin<T, P> where P: PortConfigMode {
    fn mode(&self) -> PinMode {
        self.periph().mode(self.index())
    }
    fn set_mode(&self, value: PinMode) -> &Self {
        self.periph().set_mode(self.index(), value);
        self
    }
}

pub enum PinPull {
    None,
    PullUp,
    PullDown,
}

pub trait PortConfigPull {
    fn pull(&self, index: usize) -> PinPull;
    fn set_pull(&self, index: usize, value: PinPull) -> &Self;
}

pub trait PinConfigPull {
    fn pull(&self) -> PinPull;
    fn set_pull(&self, value: PinPull) -> &Self;
}

impl<T, P> PinConfigPull for Pin<T, P> where P: PortConfigPull {
    fn pull(&self) -> PinPull {
        self.periph().pull(self.index())
    }
    fn set_pull(&self, value: PinPull) -> &Self {
        self.periph().set_pull(self.index(), value);
        self
    }
}

pub trait PortConfigOpenDrain {
    fn open_drain(&self, index: usize) -> bool;
    fn set_open_drain(&self, index: usize, value: bool) -> &Self;
}

pub trait PinConfigOpenDrain {
    fn open_drain(&self) -> bool;
    fn set_open_drain(&self, value: bool) -> &Self;
}

impl<T, P> PinConfigOpenDrain for Pin<T, P> where P: PortConfigOpenDrain {
    fn open_drain(&self) -> bool {
        self.periph().open_drain(self.index())
    }
    fn set_open_drain(&self, value: bool) -> &Self {
        self.periph().set_open_drain(self.index(), value);
        self
    }
}

pub trait PortOutput {
    fn output(&self, index: usize) -> bool;
    fn set_output(&self, index: usize, value: bool) -> &Self;
    fn toggle_output(&self, index: usize) -> &Self { self.set_output(index, !self.output(index)) }
}

pub trait PinOutput {
    fn output(&self) -> bool;
    fn set_output(&self, value: bool) -> &Self;
    fn toggle_output(&self) -> &Self { self.set_output(!self.output()) }
}

impl<T, P> PinOutput for Pin<T, P> where P: PortOutput {
    fn output(&self) -> bool {
        self.periph().output(self.index())
    }
    fn set_output(&self, value: bool) -> &Self {
        self.periph().set_output(self.index(), value);
        self
    }
}

pub trait PinInput {
    fn input(&self) -> bool;
}

pub trait PortInput {
    fn input(&self, index: usize) -> bool;
}


impl<T, P> PinInput for Pin<T, P> where P: PortInput {
    fn input(&self) -> bool {
        self.periph().input(self.index())
    }
}

pub trait Reset {
    fn reset(&self);
}

pub trait ClockEnable {
    fn clock_is_enabled(&self) -> bool;
    fn clock_set_enabled(&self, value: bool) -> &Self;
    fn clock_enable(&self) -> &Self { self.clock_set_enabled(true) }
    fn clock_disable(&self) -> &Self { self.clock_set_enabled(false) }    
}

impl<P, T> ClockEnable for Pin<P, T> where T: ClockEnable {
    fn clock_is_enabled(&self) -> bool {
        self.periph().clock_is_enabled()
    }
    fn clock_set_enabled(&self, value: bool) -> &Self {
        self.periph().clock_set_enabled(value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const P0: Pin0 = Pin0 {};
    pub const P1: Pin1 = Pin1 {};
    pub const P2: Pin2 = Pin2 {};
    pub const P3: Pin3 = Pin3 {};

    pub struct Pin0 {}
    pub struct Pin1 {}
    pub struct Pin2 {}
    pub struct Pin3 {}

    #[test]
    fn test_inputs() {
        impl<'a> PortInput for &'a [bool] {
            fn input(&self, index: usize) -> bool {
                self[index]
            }            
        }

        let inputs = &[true, false, true, true][..];
        assert_eq!(inputs.input(0), true);
        assert_eq!(inputs.input(1), false);
        assert_eq!(inputs.input(2), true);
        assert_eq!(inputs.input(3), true);
        
        let p0 = Pin(P0, inputs, 0);
        let p1 = Pin(P1, inputs, 1);
        let p2 = Pin(P2, inputs, 2);
        let p3 = Pin(P3, inputs, 3);

        assert_eq!(p0.input(), true);
        assert_eq!(p1.input(), false);
        assert_eq!(p2.input(), true);
        assert_eq!(p3.input(), true);
    }

    #[test]
    fn test_outputs() {
        use core::cell::Cell;
        pub struct Port { 
            cells: [Cell<bool>; 4],
        }

        impl<'a> PortOutput for &'a Port {
            fn output(&self, index: usize) -> bool {
                self.cells[index].get()
            }
            fn set_output(&self, index: usize, value: bool) -> &Self {
                self.cells[index].set(value);
                self
            }
        }

        let port = &Port { cells: [Cell::new(true), Cell::new(false), Cell::new(true), Cell::new(true)]};
        assert_eq!(port.output(0), true);
        assert_eq!(port.output(1), false);
        assert_eq!(port.output(2), true);
        assert_eq!(port.output(3), true);
        
        let p0 = Pin(P0, port, 0);
        let p1 = Pin(P1, port, 1);
        let p2 = Pin(P2, port, 2);
        let p3 = Pin(P3, port, 3);

        assert_eq!(p0.output(), true);
        assert_eq!(p1.output(), false);
        assert_eq!(p2.output(), true);
        assert_eq!(p3.output(), true);
    }    
}
