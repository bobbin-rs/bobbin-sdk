#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::exti::*;

periph!( EXTI, Exti, _EXTI, ExtiPeriph, 0x40013c00);



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

