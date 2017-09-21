pub use chip::exti::*;

impl ExtiCh {
    pub fn set_interrupt_mask(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.periph.with_imr(|r| r.set_mr(self.index, value));
        self
    }
    
    pub fn set_rising_trigger(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.periph.with_rtsr(|r| r.set_tr(self.index, value));
        self
    }    

    pub fn set_falling_trigger(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.periph.with_ftsr(|r| r.set_tr(self.index, value));
        self
    }

    pub fn trigger(&self) -> &Self {
        self.periph.set_swier(|r| r.set_swi(self.index, 1));
        self
    }   

    pub fn pending(&self) -> bool {
        self.periph.pr().test_pr(self.index)
    }

    pub fn clr_pending(&self) -> &Self {
        self.periph.set_pr(|r| r.set_pr(self.index, 1));
        self
    }    
}