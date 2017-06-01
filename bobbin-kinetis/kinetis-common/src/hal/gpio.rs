use chip::gpio::*;

pub enum Dir {
    In = 0,
    Out = 1,
}

pub trait GpioExt {    
    fn set_dir(&self, value: Dir) -> &Self;
    fn set_dir_input(&self) -> &Self;
    fn set_dir_output(&self) -> &Self;

    fn output(&self) -> bool;
    fn set_output(&self, value: bool) -> &Self;
    fn toggle_output(&self) -> &Self;
    fn input(&self) -> bool;
}

impl GpioExt for PinImpl {
    fn set_dir(&self, value: Dir) -> &Self {
        self.port.with_pddr(|r| r.set_pdd(self.index, value as u32));
        self
    }
    fn set_dir_input(&self) -> &Self {
        self.set_dir(Dir::In)
    }
    fn set_dir_output(&self) -> &Self {
        self.set_dir(Dir::Out)
    }
    

    fn output(&self) -> bool {
        self.port.pdor().pdo(self.index) != 0
    }

    fn set_output(&self, value: bool) -> &Self {
        if value {
            self.port.set_psor(Psor(0).set_ptso(self.index, 1))
        } else {
            self.port.set_pcor(Pcor(0).set_ptco(self.index, 1))
        };
        self
    }

    fn toggle_output(&self) -> &Self {
        self.port.set_ptor(Ptor(0).set_ptto(self.index, 1));
        self
    }

    fn input(&self) -> bool {
        self.port.pdir().pdi(self.index) != 0
    }
}