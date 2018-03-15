pub use bobbin_common::digital::*;
pub use chip::gpio::*;

pub enum Dir {
    In = 0,
    Out = 1,
}

impl GpioPin {
    pub fn set_dir(&self, value: Dir) -> &Self {
        self.port.with_pddr(|r| r.set_pdd(self.index, value as u32));
        self
    }
    pub fn set_dir_input(&self) -> &Self {
        self.set_dir(Dir::In)
    }
    pub fn set_dir_output(&self) -> &Self {
        self.set_dir(Dir::Out)
    }
}

impl DigitalOutput for GpioPin {
    fn output(&self) -> bool {
        self.port.pdor().pdo(self.index) != 0
    }

    fn set_output(&self, value: bool) -> &Self {
        if value {
            self.port.set_psor(|r| r.set_ptso(self.index, 1))
        } else {
            self.port.set_pcor(|r| r.set_ptco(self.index, 1))
        };
        self
    }

    fn toggle_output(&self) -> &Self {
        self.port.set_ptor(|r| r.set_ptto(self.index, 1));
        self
    }
}
impl DigitalInput for GpioPin {
    fn input(&self) -> bool {
        self.port.pdir().pdi(self.index) != 0
    }
}