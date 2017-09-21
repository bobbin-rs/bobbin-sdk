use ::chip::pit::*;

pub struct Timer {
    index: usize,
}

pub fn timer(index: usize) -> Timer {
    Timer {
        index: index
    }
}

pub fn enabled() -> bool {
    // MDIS = 1 to disable module
    unsafe {
        PIT.mcr().mdis() == 0
    }
}

pub fn set_enabled(value: bool) {
    // MDIS = 1 to disable module
    let value = if value { 0 } else { 1 };
    unsafe {
        PIT.with_mcr(|r| r.set_mdis(value))
    }
}

impl Timer {
    pub fn load_value(&self) -> u32 {
        unsafe {
            PIT.ldval(self.index).tsv()
        }
    }

    pub fn set_load_value(&self, value: u32) {
        unsafe {
            PIT.set_ldval(self.index, Ldval(0).set_tsv(value))
        }
    }

    pub fn current_value(&self) -> u32 {
        unsafe {
            PIT.cval(self.index).tvl()
        }
    }    

    pub fn chain(&self) -> bool {
        unsafe {
            PIT.tctrl(self.index).chn() != 0
        }
    }

    pub fn set_chain(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        unsafe {
            PIT.with_tctrl(self.index, |r| r.set_chn(value))
        }
    }

    pub fn interrupt_enabled(&self) -> bool {
        unsafe {
            PIT.tctrl(self.index).tie() != 0
        }
    }

    pub fn set_interrupt_enabled(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        unsafe {
            PIT.with_tctrl(self.index, |r| r.set_tie(value))
        }
    }

    pub fn enabled(&self) -> bool {
        unsafe {
            PIT.tctrl(self.index).ten() != 0
        }
    }

    pub fn set_enabled(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        unsafe {
            PIT.with_tctrl(self.index, |r| r.set_ten(value))
        }
    }

    pub fn interrupt_flag(&self) -> bool {
        unsafe {
            PIT.tflg(self.index).tif() != 0
        }
    }

    pub fn clr_interrupt_flag(&self) {
        unsafe {
            PIT.set_tflg(self.index, Tflg(0).set_tif(1))
        }
    }
}