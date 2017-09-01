#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::exti::*;

periph!( EXTI, Exti, _EXTI, ExtiPeriph, 0x40010400);



channel!(EXTI_LINE0, ExtiLine0, EXTI, Exti, _EXTI_LINE0, ExtiCh, _EXTI, 0);
channel!(EXTI_LINE1, ExtiLine1, EXTI, Exti, _EXTI_LINE1, ExtiCh, _EXTI, 1);
channel!(EXTI_LINE2, ExtiLine2, EXTI, Exti, _EXTI_LINE2, ExtiCh, _EXTI, 2);
channel!(EXTI_LINE3, ExtiLine3, EXTI, Exti, _EXTI_LINE3, ExtiCh, _EXTI, 3);
channel!(EXTI_LINE4, ExtiLine4, EXTI, Exti, _EXTI_LINE4, ExtiCh, _EXTI, 4);
channel!(EXTI_LINE5, ExtiLine5, EXTI, Exti, _EXTI_LINE5, ExtiCh, _EXTI, 5);
channel!(EXTI_LINE6, ExtiLine6, EXTI, Exti, _EXTI_LINE6, ExtiCh, _EXTI, 6);
channel!(EXTI_LINE7, ExtiLine7, EXTI, Exti, _EXTI_LINE7, ExtiCh, _EXTI, 7);
channel!(EXTI_LINE8, ExtiLine8, EXTI, Exti, _EXTI_LINE8, ExtiCh, _EXTI, 8);
channel!(EXTI_LINE9, ExtiLine9, EXTI, Exti, _EXTI_LINE9, ExtiCh, _EXTI, 9);
channel!(EXTI_LINE10, ExtiLine10, EXTI, Exti, _EXTI_LINE10, ExtiCh, _EXTI, 10);
channel!(EXTI_LINE11, ExtiLine11, EXTI, Exti, _EXTI_LINE11, ExtiCh, _EXTI, 11);
channel!(EXTI_LINE12, ExtiLine12, EXTI, Exti, _EXTI_LINE12, ExtiCh, _EXTI, 12);
channel!(EXTI_LINE13, ExtiLine13, EXTI, Exti, _EXTI_LINE13, ExtiCh, _EXTI, 13);
channel!(EXTI_LINE14, ExtiLine14, EXTI, Exti, _EXTI_LINE14, ExtiCh, _EXTI, 14);
channel!(EXTI_LINE15, ExtiLine15, EXTI, Exti, _EXTI_LINE15, ExtiCh, _EXTI, 15);
channel!(EXTI_LINE16, ExtiLine16, EXTI, Exti, _EXTI_LINE16, ExtiCh, _EXTI, 16);
channel!(EXTI_LINE17, ExtiLine17, EXTI, Exti, _EXTI_LINE17, ExtiCh, _EXTI, 17);
channel!(EXTI_LINE18, ExtiLine18, EXTI, Exti, _EXTI_LINE18, ExtiCh, _EXTI, 18);
channel!(EXTI_LINE19, ExtiLine19, EXTI, Exti, _EXTI_LINE19, ExtiCh, _EXTI, 19);
channel!(EXTI_LINE20, ExtiLine20, EXTI, Exti, _EXTI_LINE20, ExtiCh, _EXTI, 20);
channel!(EXTI_LINE21, ExtiLine21, EXTI, Exti, _EXTI_LINE21, ExtiCh, _EXTI, 21);
channel!(EXTI_LINE22, ExtiLine22, EXTI, Exti, _EXTI_LINE22, ExtiCh, _EXTI, 22);
channel!(EXTI_LINE23, ExtiLine23, EXTI, Exti, _EXTI_LINE23, ExtiCh, _EXTI, 23);
channel!(EXTI_LINE24, ExtiLine24, EXTI, Exti, _EXTI_LINE24, ExtiCh, _EXTI, 24);
channel!(EXTI_LINE25, ExtiLine25, EXTI, Exti, _EXTI_LINE25, ExtiCh, _EXTI, 25);
channel!(EXTI_LINE26, ExtiLine26, EXTI, Exti, _EXTI_LINE26, ExtiCh, _EXTI, 26);
channel!(EXTI_LINE28, ExtiLine28, EXTI, Exti, _EXTI_LINE28, ExtiCh, _EXTI, 28);
channel!(EXTI_LINE29, ExtiLine29, EXTI, Exti, _EXTI_LINE29, ExtiCh, _EXTI, 29);
channel!(EXTI_LINE30, ExtiLine30, EXTI, Exti, _EXTI_LINE30, ExtiCh, _EXTI, 30);
channel!(EXTI_LINE31, ExtiLine31, EXTI, Exti, _EXTI_LINE31, ExtiCh, _EXTI, 31);
channel!(EXTI_LINE32, ExtiLine32, EXTI, Exti, _EXTI_LINE32, ExtiCh, _EXTI, 32);
channel!(EXTI_LINE33, ExtiLine33, EXTI, Exti, _EXTI_LINE33, ExtiCh, _EXTI, 33);
channel!(EXTI_LINE34, ExtiLine34, EXTI, Exti, _EXTI_LINE34, ExtiCh, _EXTI, 34);
channel!(EXTI_LINE35, ExtiLine35, EXTI, Exti, _EXTI_LINE35, ExtiCh, _EXTI, 35);

pub trait IrqExti<T> {
    fn irq_exti(&self) -> T;
}

impl IrqExti<super::irq::IrqExti0> for ExtiLine0 {
    fn irq_exti(&self) -> super::irq::IrqExti0 { super::irq::IRQ_EXTI0 }
}

impl IrqExti<super::irq::IrqExti1> for ExtiLine1 {
    fn irq_exti(&self) -> super::irq::IrqExti1 { super::irq::IRQ_EXTI1 }
}

impl IrqExti<super::irq::IrqExti2> for ExtiLine2 {
    fn irq_exti(&self) -> super::irq::IrqExti2 { super::irq::IRQ_EXTI2 }
}

impl IrqExti<super::irq::IrqExti3> for ExtiLine3 {
    fn irq_exti(&self) -> super::irq::IrqExti3 { super::irq::IRQ_EXTI3 }
}

impl IrqExti<super::irq::IrqExti4> for ExtiLine4 {
    fn irq_exti(&self) -> super::irq::IrqExti4 { super::irq::IRQ_EXTI4 }
}

impl IrqExti<super::irq::IrqExti5> for ExtiLine5 {
    fn irq_exti(&self) -> super::irq::IrqExti5 { super::irq::IRQ_EXTI5 }
}

impl IrqExti<super::irq::IrqExti6> for ExtiLine6 {
    fn irq_exti(&self) -> super::irq::IrqExti6 { super::irq::IRQ_EXTI6 }
}

impl IrqExti<super::irq::IrqExti7> for ExtiLine7 {
    fn irq_exti(&self) -> super::irq::IrqExti7 { super::irq::IRQ_EXTI7 }
}

impl IrqExti<super::irq::IrqExti8> for ExtiLine8 {
    fn irq_exti(&self) -> super::irq::IrqExti8 { super::irq::IRQ_EXTI8 }
}

impl IrqExti<super::irq::IrqExti9> for ExtiLine9 {
    fn irq_exti(&self) -> super::irq::IrqExti9 { super::irq::IRQ_EXTI9 }
}

impl IrqExti<super::irq::IrqExti10> for ExtiLine10 {
    fn irq_exti(&self) -> super::irq::IrqExti10 { super::irq::IRQ_EXTI10 }
}

impl IrqExti<super::irq::IrqExti11> for ExtiLine11 {
    fn irq_exti(&self) -> super::irq::IrqExti11 { super::irq::IRQ_EXTI11 }
}

impl IrqExti<super::irq::IrqExti12> for ExtiLine12 {
    fn irq_exti(&self) -> super::irq::IrqExti12 { super::irq::IRQ_EXTI12 }
}

impl IrqExti<super::irq::IrqExti13> for ExtiLine13 {
    fn irq_exti(&self) -> super::irq::IrqExti13 { super::irq::IRQ_EXTI13 }
}

impl IrqExti<super::irq::IrqExti14> for ExtiLine14 {
    fn irq_exti(&self) -> super::irq::IrqExti14 { super::irq::IRQ_EXTI14 }
}

impl IrqExti<super::irq::IrqExti15> for ExtiLine15 {
    fn irq_exti(&self) -> super::irq::IrqExti15 { super::irq::IRQ_EXTI15 }
}

impl IrqExti<super::irq::IrqExti16> for ExtiLine16 {
    fn irq_exti(&self) -> super::irq::IrqExti16 { super::irq::IRQ_EXTI16 }
}

impl IrqExti<super::irq::IrqExti17> for ExtiLine17 {
    fn irq_exti(&self) -> super::irq::IrqExti17 { super::irq::IRQ_EXTI17 }
}

impl IrqExti<super::irq::IrqExti18> for ExtiLine18 {
    fn irq_exti(&self) -> super::irq::IrqExti18 { super::irq::IRQ_EXTI18 }
}

impl IrqExti<super::irq::IrqExti19> for ExtiLine19 {
    fn irq_exti(&self) -> super::irq::IrqExti19 { super::irq::IRQ_EXTI19 }
}

impl IrqExti<super::irq::IrqExti20> for ExtiLine20 {
    fn irq_exti(&self) -> super::irq::IrqExti20 { super::irq::IRQ_EXTI20 }
}

impl IrqExti<super::irq::IrqExti21> for ExtiLine21 {
    fn irq_exti(&self) -> super::irq::IrqExti21 { super::irq::IRQ_EXTI21 }
}

impl IrqExti<super::irq::IrqExti22> for ExtiLine22 {
    fn irq_exti(&self) -> super::irq::IrqExti22 { super::irq::IRQ_EXTI22 }
}

impl IrqExti<super::irq::IrqExti23> for ExtiLine23 {
    fn irq_exti(&self) -> super::irq::IrqExti23 { super::irq::IRQ_EXTI23 }
}

impl IrqExti<super::irq::IrqExti24> for ExtiLine24 {
    fn irq_exti(&self) -> super::irq::IrqExti24 { super::irq::IRQ_EXTI24 }
}

impl IrqExti<super::irq::IrqExti25> for ExtiLine25 {
    fn irq_exti(&self) -> super::irq::IrqExti25 { super::irq::IRQ_EXTI25 }
}

impl IrqExti<super::irq::IrqExti26> for ExtiLine26 {
    fn irq_exti(&self) -> super::irq::IrqExti26 { super::irq::IRQ_EXTI26 }
}

impl IrqExti<super::irq::IrqExti28> for ExtiLine28 {
    fn irq_exti(&self) -> super::irq::IrqExti28 { super::irq::IRQ_EXTI28 }
}

impl IrqExti<super::irq::IrqExti29> for ExtiLine29 {
    fn irq_exti(&self) -> super::irq::IrqExti29 { super::irq::IRQ_EXTI29 }
}

impl IrqExti<super::irq::IrqExti30> for ExtiLine30 {
    fn irq_exti(&self) -> super::irq::IrqExti30 { super::irq::IRQ_EXTI30 }
}

impl IrqExti<super::irq::IrqExti31> for ExtiLine31 {
    fn irq_exti(&self) -> super::irq::IrqExti31 { super::irq::IRQ_EXTI31 }
}

impl IrqExti<super::irq::IrqExti32> for ExtiLine32 {
    fn irq_exti(&self) -> super::irq::IrqExti32 { super::irq::IRQ_EXTI32 }
}

impl IrqExti<super::irq::IrqExti33> for ExtiLine33 {
    fn irq_exti(&self) -> super::irq::IrqExti33 { super::irq::IRQ_EXTI33 }
}

impl IrqExti<super::irq::IrqExti34> for ExtiLine34 {
    fn irq_exti(&self) -> super::irq::IrqExti34 { super::irq::IRQ_EXTI34 }
}

impl IrqExti<super::irq::IrqExti35> for ExtiLine35 {
    fn irq_exti(&self) -> super::irq::IrqExti35 { super::irq::IRQ_EXTI35 }
}

