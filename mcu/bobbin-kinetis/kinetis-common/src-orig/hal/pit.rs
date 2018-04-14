pub use bobbin_common::hal::timer::*;
pub use chip::pit::*;
pub use core::ops::Deref;

impl PitPeriph {
    pub fn enabled(&self) -> bool {
        // MDIS = 1 to disable module
        self.mcr().mdis() == 0
    }

    pub fn set_enabled(&self, value: bool) -> &Self {
        // MDIS = 1 to disable module
        self.with_mcr(|r| r.set_mdis(!value))        
    }

    pub fn timer_enabled(&self, index: usize) -> bool {
        self.tctrl(index).ten() != 0
    }

    pub fn set_timer_enabled(&self, index: usize, value: bool) -> &Self {
        self.with_tctrl(index, |r| r.set_ten(value))
    }

    pub fn load_value(&self, index: usize) -> u32 {
        self.ldval(index).tsv().value()
    }

    pub fn set_load_value(&self, index: usize, value: u32) -> &Self {
        self.set_ldval(index, |r| r.set_tsv(value))
    }    

    pub fn counter_value(&self, index: usize) -> u32 {
        self.cval(index).tvl().value()
    }


    pub fn interrupt_flag(&self, index: usize) -> bool {        
        self.tflg(index).tif() != 0        
    }

    pub fn clr_interrupt_flag(&self, index: usize) -> &Self {
        self.set_tflg(index, |r| r.set_tif(1))
    }    
}

impl Delay<u32> for PitCh {
    fn delay(&self, value: u32) -> &Self {
        self
            .start_down(value)
            .clr_timeout()
            .wait_timeout()
            .stop()
    }
}

impl Start<u32> for PitCh {
    fn start(&self, value: u32) -> &Self {       
        self.start_down(value)
    }
}

impl StartDown<u32> for PitCh {
    fn start_down(&self, value: u32) -> &Self {       
        self.periph
            .set_enabled(true)
            .set_load_value(self.index, value - 1)
            .clr_interrupt_flag(self.index)
            .set_timer_enabled(self.index, true);
        self
    }
}

impl Stop for PitCh {
    fn stop(&self) -> &Self {
        self.periph.set_timer_enabled(self.index, false);
        self
    }
}

impl Running for PitCh {
    fn running(&self) -> bool {
        self.periph.timer_enabled(self.index)
    }
}

impl Period<u32> for PitCh {
    fn period(&self) -> u32 {
        self.periph.load_value(self.index) + 1
    }
}

impl SetPeriod<u32> for PitCh {
    fn set_period(&self, value: u32) -> &Self {
        self.periph.set_load_value(self.index, value - 1);
        self
    }
}

impl Counter<u32> for PitCh {
    fn counter(&self) -> u32 {
        self.periph.counter_value(self.index)
    }
}

impl Timeout for PitCh {
    fn test_timeout(&self) -> bool {
        self.periph.interrupt_flag(self.index)
    }

    fn clr_timeout(&self) -> &Self {
        self.periph.clr_interrupt_flag(self.index);
        self
    }
}

pub fn test_pit(tim_ch: &PitCh) {
    test_timer(tim_ch, 1024);
    test_timer_down(tim_ch, 1024);
}