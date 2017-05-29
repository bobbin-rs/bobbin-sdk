pub use ::chip::exti::*;
use ::chip::irq;
use ::chip::syscfg;
use ::chip::gpio;
use rcc;


pub struct InterruptLine {
    index: usize,
}

pub fn line(index: usize) -> InterruptLine {
    InterruptLine { index: index }
}

impl InterruptLine {
    pub fn irq(&self) -> irq::Irq {
        match self.index {
            0 => irq::IRQ_EXTI0_1,
            1 => irq::IRQ_EXTI0_1,
            2 => irq::IRQ_EXTI2_3,
            3 => irq::IRQ_EXTI2_3,
            4...15 => irq::IRQ_EXTI4_15,
            _ => panic!("Line must be 0..15"),
        }
    }

    pub fn interrupt_mask(&self) -> bool {
        unsafe {
            match self.index {
                0...31 =>  EXTI.imr().mr(self.index) != 0,
                _ => unimplemented!(),
            }
        }
    }

    pub fn set_interrupt_mask(&self, value: bool) {
        let value = if value { 1 } else { 0 };        
        unsafe {
            match self.index {
                0...31 => EXTI.with_imr(|r| r.set_mr(self.index, value)),
                _ => unimplemented!(),
            }
        }
    }

    pub fn event_mask(&self) -> bool {
        unsafe {
            match self.index {
                0...31 =>  EXTI.emr().mr(self.index) != 0,
                _ => unimplemented!(),
            }
        }
    }

    pub fn set_event_mask(&self, value: bool) {
        let value = if value { 1 } else { 0 };        
        unsafe {
            match self.index {
                0...31 => EXTI.with_emr(|r| r.set_mr(self.index, value)),
                _ => unimplemented!(),
            }
        }
    }    

    pub fn rising_trigger(&self) -> bool {
        unsafe {
            match self.index {
                0...31 =>  EXTI.rtsr().tr(self.index) != 0,
                _ => unimplemented!(),
            }
        }
    }

    pub fn set_rising_trigger(&self, value: bool) {
        let value = if value { 1 } else { 0 };        
        unsafe {
            match self.index {
                0...31 => EXTI.with_rtsr(|r| r.set_tr(self.index, value)),
                _ => unimplemented!(),
            }
        }
    } 

    pub fn falling_trigger(&self) -> bool {
        unsafe {
            match self.index {
                0...31 =>  EXTI.ftsr().tr(self.index) != 0,
                _ => unimplemented!(),
            }
        }
    }

    pub fn set_falling_trigger(&self, value: bool) {
        let value = if value { 1 } else { 0 };        
        unsafe {
            match self.index {
                0...31 => EXTI.with_ftsr(|r| r.set_tr(self.index, value)),
                _ => unimplemented!(),
            }
        }
    }             

    pub fn software_interrupt(&self) -> bool {
        unsafe {
            match self.index {
                0...17 =>  EXTI.swier().swier(self.index) != 0,
                18 => panic!("Reserved Value"),
                19...22 =>  EXTI.swier().swier(self.index) != 0,
                _ => unimplemented!(),
            }
        }
    }

    pub fn set_software_interrupt(&self, value: bool) {
        let value = if value { 1 } else { 0 };        
        unsafe {
            match self.index {
                0...17 => EXTI.with_swier(|r| r.set_swier(self.index, value)),
                18 => panic!("Reserved Value"),
                19...22 => EXTI.with_swier(|r| r.set_swier(self.index, value)),
                _ => unimplemented!(),
            }
        }
    }   

    pub fn pending(&self) -> bool {
        unsafe {
            match self.index {
                0...17 =>  EXTI.pr().pr(self.index) != 0,
                18 => panic!("Reserved Value"),
                19...22 =>  EXTI.pr().pr(self.index) != 0,
                _ => unimplemented!(),
            }
        }
    }

    pub fn clr_pending(&self) {
        let value = 1;
        unsafe {
            match self.index {
                0...17 => EXTI.with_pr(|r| r.set_pr(self.index, value)),
                18 => panic!("Reserved Value"),
                19...22 => EXTI.with_pr(|r| r.set_pr(self.index, value)),
                _ => unimplemented!(),
            }
        }
    }

    pub fn external_source(&self) -> gpio::Gpio {
        rcc::set_syscfg_enabled(true);
        let s = syscfg::SYSCFG;
        unsafe {
            i2p(match self.index {
                0 => s.exticr1().exti0(),
                1 => s.exticr1().exti1(),
                2 => s.exticr1().exti2(),
                3 => s.exticr1().exti3(),
                4 => s.exticr2().exti4(),
                5 => s.exticr2().exti5(),
                6 => s.exticr2().exti6(),
                7 => s.exticr2().exti7(),
                8 => s.exticr3().exti8(),
                9 => s.exticr3().exti9(),
                10 => s.exticr3().exti10(),
                11 => s.exticr3().exti11(),
                12 => s.exticr4().exti12(),
                13 => s.exticr4().exti13(),
                14 => s.exticr4().exti14(),
                15 => s.exticr4().exti15(),  
                _ => unimplemented!(),              
            })
        }
    }

    pub fn set_external_source(&self, value: gpio::Gpio) {
        rcc::set_syscfg_enabled(true);
        let mut s = syscfg::SYSCFG;
        let value = p2i(value);
        unsafe {
            match self.index {
                0 => s.with_exticr1(|r| r.set_exti0(value)),
                1 => s.with_exticr1(|r| r.set_exti1(value)),
                2 => s.with_exticr1(|r| r.set_exti2(value)),
                3 => s.with_exticr1(|r| r.set_exti3(value)),
                4 => s.with_exticr2(|r| r.set_exti4(value)),
                5 => s.with_exticr2(|r| r.set_exti5(value)),
                6 => s.with_exticr2(|r| r.set_exti6(value)),
                7 => s.with_exticr2(|r| r.set_exti7(value)),
                8 => s.with_exticr3(|r| r.set_exti8(value)),
                9 => s.with_exticr3(|r| r.set_exti9(value)),
                10 => s.with_exticr3(|r| r.set_exti10(value)),
                11 => s.with_exticr3(|r| r.set_exti11(value)),
                12 => s.with_exticr4(|r| r.set_exti12(value)),
                13 => s.with_exticr4(|r| r.set_exti13(value)),
                14 => s.with_exticr4(|r| r.set_exti14(value)),
                15 => s.with_exticr4(|r| r.set_exti15(value)),
                _ => unimplemented!(),
            }
        }
    }
}

fn i2p(i: u32) -> gpio:: Gpio {
    match i {
        0 => gpio::GPIOA,
        1 => gpio::GPIOB,
        2 => gpio::GPIOC,
        _ => unimplemented!(),
    }
}

fn p2i(p: gpio::Gpio) -> u32 {
    match p {
        gpio::GPIOA => 0,
        gpio::GPIOB => 1,
        gpio::GPIOC => 2,
        _ => unimplemented!(),
    }
}