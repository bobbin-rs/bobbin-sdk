pub use bobbin_common::timer::*;
pub use chip::pit::*;

pub trait PitExt {
    fn enabled(&self) -> bool;
    fn set_enabled(&self, value: bool) -> &Self;
    fn timer_enabled(&self, index: usize) -> bool;
    fn set_timer_enabled(&self, index: usize, value: bool) -> &Self;
    fn load_value(&self, index: usize) -> u32;
    fn set_load_value(&self, index: usize, value: u32) -> &Self;
    fn counter_value(&self, index: usize) -> u32;
    fn interrupt_flag(&self, index: usize) -> bool;
    fn clr_interrupt_flag(&self, index: usize) -> &Self;
}

impl<T> PitExt for Periph<T> {
    fn enabled(&self) -> bool {
        // MDIS = 1 to disable module
        self.mcr().mdis() == 0
    }

    fn set_enabled(&self, value: bool) -> &Self {
        // MDIS = 1 to disable module
        let value = if value { 0 } else { 1 };
        self.with_mcr(|r| r.set_mdis(value))        
    }

    fn timer_enabled(&self, index: usize) -> bool {
        self.tctrl(index).ten() != 0
    }

    fn set_timer_enabled(&self, index: usize, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_tctrl(index, |r| r.set_ten(value))
    }

    fn load_value(&self, index: usize) -> u32 {
        self.ldval(index).tsv()
    }

    fn set_load_value(&self, index: usize, value: u32) -> &Self {
        self.set_ldval(index, Ldval(0).set_tsv(value))
    }    

    fn counter_value(&self, index: usize) -> u32 {
        self.cval(index).tvl()
    }


    fn interrupt_flag(&self, index: usize) -> bool {        
        self.tflg(index).tif() != 0        
    }

    fn clr_interrupt_flag(&self, index: usize) -> &Self {
        self.set_tflg(index, Tflg(0).set_tif(1))
    }    
}

impl<P, T> Delay<u32> for Channel<P, T> {
    fn delay(&self, value: u32) -> &Self {
        self
            .start_down(value)
            .clr_timeout_flag()
            .wait_timeout_flag()
            .stop()
    }
}

impl<P, T> Start<u32> for Channel<P, T> {
    fn start(&self, value: u32) -> &Self {       
        self.start_down(value)
    }
}

impl<P, T> StartDown<u32> for Channel<P, T> {
    fn start_down(&self, value: u32) -> &Self {       
        self.periph()
            .set_enabled(true)
            .set_load_value(self.index(), value - 1)
            .set_timer_enabled(self.index(), true);
        self
    }
}

impl<P, T> Timer<u32> for Channel<P, T> {
    fn stop(&self) -> &Self {
        self.periph().set_timer_enabled(self.index(), false);
        self
    }

    fn running(&self) -> bool {
        self.periph().timer_enabled(self.index())
    }

    fn period(&self) -> u32 {
        self.periph().load_value(self.index()) + 1
    }
    
    fn set_period(&self, value: u32) -> &Self {
        self.periph().set_load_value(self.index(), value - 1);
        self
    }

    fn counter(&self) -> u32 {
        self.periph().counter_value(self.index())
    }

    fn timeout_flag(&self) -> bool {
        self.periph().interrupt_flag(self.index())
    }

    fn clr_timeout_flag(&self) -> &Self {
        self.periph().clr_interrupt_flag(self.index());
        self
    }
}
