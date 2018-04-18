pub use bobbin_hal::digital::*;
pub use gpio::*;

pub enum Dir {
    In = 0,
    Out = 1,
}

impl GpioCh {
    pub fn set_dir(&self, value: Dir) -> &Self {
        self.periph.with_pddr(|r| r.set_pdd(self.index, value as u32));
        self
    }
    pub fn set_dir_input(&self) -> &Self {
        self.set_dir(Dir::In)
    }
    pub fn set_dir_output(&self) -> &Self {
        self.set_dir(Dir::Out)
    }
}

impl DigitalOutput for GpioCh {
    fn output(&self) -> bool {
        self.periph.pdor().pdo(self.index) != 0
    }

    fn set_output(&self, value: bool) -> &Self {
        if value {
            self.periph.set_psor(|r| r.set_ptso(self.index, 1))
        } else {
            self.periph.set_pcor(|r| r.set_ptco(self.index, 1))
        };
        self
    }

    fn toggle_output(&self) -> &Self {
        self.periph.set_ptor(|r| r.set_ptto(self.index, 1));
        self
    }
}
impl DigitalInput for GpioCh {
    fn input(&self) -> bool {
        self.periph.pdir().pdi(self.index) != 0
    }
}