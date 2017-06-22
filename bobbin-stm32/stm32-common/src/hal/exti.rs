pub use chip::exti::*;

pub trait ExtiExt {

}

impl<T> ExtiExt for Periph<T> {

}

pub trait ExtiLineExt {
    fn set_interrupt_mask(&self, value: bool) -> &Self;
    fn set_rising_trigger(&self, value: bool) -> &Self;
    fn set_falling_trigger(&self, value: bool) -> &Self;
    fn trigger(&self) -> &Self;
    fn clr_pending(&self) -> &Self;    
}

impl<P, T> ExtiLineExt for Channel<P, T> {
    fn set_interrupt_mask(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.periph.with_imr(|r| r.set_mr(self.index, value));
        self
    }
    
    fn set_rising_trigger(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.periph.with_rtsr(|r| r.set_tr(self.index, value));
        self
    }    

    fn set_falling_trigger(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.periph.with_ftsr(|r| r.set_tr(self.index, value));
        self
    }

    fn trigger(&self) -> &Self {
        self.periph.set_swier(Swier(0).set_swi(self.index, 1));
        self
    }   

    fn clr_pending(&self) -> &Self {
        self.periph.set_pr(Pr(0).set_pr(self.index, 1));
        self
    }    
}