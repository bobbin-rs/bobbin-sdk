use chip::port::PinImpl;

pub enum Pull {
    None,
    PullDown,
    PullUp,
}

pub trait PinExt {
    fn mux(&self) -> usize;
    fn set_mux(&self, value: usize) -> &Self;
    fn set_mux_gpio(&self) -> &Self;
    fn set_pull(&self, value: Pull) -> &Self;
    fn set_pull_down(&self) -> &Self;
    fn set_pull_up(&self) -> &Self;
}

impl PinExt for PinImpl {
    fn mux(&self) -> usize {
        self.port.pcr(self.index).mux() as usize
    }
    fn set_mux(&self, value: usize) -> &Self {
        self.port.with_pcr(self.index, |r| r.set_mux(value as u32));
        self
    }
    fn set_mux_gpio(&self) -> &Self {
        self.set_mux(1)
    }
    fn set_pull(&self, value: Pull) -> &Self {
        self.port.with_pcr(self.index, |r| match value {
            Pull::None => r.set_pe(0).set_ps(0),
            Pull::PullDown => r.set_pe(1).set_ps(0),
            Pull::PullUp => r.set_pe(1).set_ps(1),
        });
        self
    }
    fn set_pull_down(&self) -> &Self {
        self.set_pull(Pull::PullDown)
    }
    fn set_pull_up(&self) -> &Self {
        self.set_pull(Pull::PullUp)
    }
}
