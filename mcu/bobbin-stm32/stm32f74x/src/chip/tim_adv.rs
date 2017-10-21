#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::tim_adv::*;

periph!( TIM1, Tim1, _TIM1, TimAdvPeriph, 0x40010000);
periph!( TIM8, Tim8, _TIM8, TimAdvPeriph, 0x40010400);

impl super::sig::Signal<super::sig::Tim1Ch1> for Tim1Ch1 {}
impl super::sig::SignalTim<super::sig::Tim1Ch1> for Tim1Ch1 {}
impl super::sig::Signal<super::sig::Tim1Ch2> for Tim1Ch2 {}
impl super::sig::SignalTim<super::sig::Tim1Ch2> for Tim1Ch2 {}
impl super::sig::Signal<super::sig::Tim1Ch3> for Tim1Ch3 {}
impl super::sig::SignalTim<super::sig::Tim1Ch3> for Tim1Ch3 {}
impl super::sig::Signal<super::sig::Tim1Ch4> for Tim1Ch4 {}
impl super::sig::SignalTim<super::sig::Tim1Ch4> for Tim1Ch4 {}

impl super::sig::Signal<super::sig::Tim8Ch1> for Tim8Ch1 {}
impl super::sig::SignalTim<super::sig::Tim8Ch1> for Tim8Ch1 {}
impl super::sig::Signal<super::sig::Tim8Ch2> for Tim8Ch2 {}
impl super::sig::SignalTim<super::sig::Tim8Ch2> for Tim8Ch2 {}
impl super::sig::Signal<super::sig::Tim8Ch3> for Tim8Ch3 {}
impl super::sig::SignalTim<super::sig::Tim8Ch3> for Tim8Ch3 {}
impl super::sig::Signal<super::sig::Tim8Ch4> for Tim8Ch4 {}
impl super::sig::SignalTim<super::sig::Tim8Ch4> for Tim8Ch4 {}


channel!(TIM1_CH1, Tim1Ch1, TIM1, Tim1, _TIM1_CH1, TimAdvCh, _TIM1, 0);
channel!(TIM1_CH2, Tim1Ch2, TIM1, Tim1, _TIM1_CH2, TimAdvCh, _TIM1, 1);
channel!(TIM1_CH3, Tim1Ch3, TIM1, Tim1, _TIM1_CH3, TimAdvCh, _TIM1, 2);
channel!(TIM1_CH4, Tim1Ch4, TIM1, Tim1, _TIM1_CH4, TimAdvCh, _TIM1, 3);
channel!(TIM8_CH1, Tim8Ch1, TIM8, Tim8, _TIM8_CH1, TimAdvCh, _TIM8, 0);
channel!(TIM8_CH2, Tim8Ch2, TIM8, Tim8, _TIM8_CH2, TimAdvCh, _TIM8, 1);
channel!(TIM8_CH3, Tim8Ch3, TIM8, Tim8, _TIM8_CH3, TimAdvCh, _TIM8, 2);
channel!(TIM8_CH4, Tim8Ch4, TIM8, Tim8, _TIM8_CH4, TimAdvCh, _TIM8, 3);

pub trait IrqBrk<T> {
    fn irq_brk(&self) -> T;
}

pub trait IrqUp<T> {
    fn irq_up(&self) -> T;
}

pub trait IrqTrgCom<T> {
    fn irq_trg_com(&self) -> T;
}

pub trait IrqCc<T> {
    fn irq_cc(&self) -> T;
}

impl IrqBrk<super::irq::IrqTim1Brk> for Tim1 {
    fn irq_brk(&self) -> super::irq::IrqTim1Brk { super::irq::IRQ_TIM1_BRK }
}

impl IrqUp<super::irq::IrqTim1Up> for Tim1 {
    fn irq_up(&self) -> super::irq::IrqTim1Up { super::irq::IRQ_TIM1_UP }
}

impl IrqTrgCom<super::irq::IrqTim1TrgCom> for Tim1 {
    fn irq_trg_com(&self) -> super::irq::IrqTim1TrgCom { super::irq::IRQ_TIM1_TRG_COM }
}

impl IrqCc<super::irq::IrqTim1Cc> for Tim1 {
    fn irq_cc(&self) -> super::irq::IrqTim1Cc { super::irq::IRQ_TIM1_CC }
}

impl IrqBrk<super::irq::IrqTim8Brk> for Tim8 {
    fn irq_brk(&self) -> super::irq::IrqTim8Brk { super::irq::IRQ_TIM8_BRK }
}

impl IrqUp<super::irq::IrqTim8Up> for Tim8 {
    fn irq_up(&self) -> super::irq::IrqTim8Up { super::irq::IRQ_TIM8_UP }
}

impl IrqTrgCom<super::irq::IrqTim8TrgCom> for Tim8 {
    fn irq_trg_com(&self) -> super::irq::IrqTim8TrgCom { super::irq::IRQ_TIM8_TRG_COM }
}

impl IrqCc<super::irq::IrqTim8Cc> for Tim8 {
    fn irq_cc(&self) -> super::irq::IrqTim8Cc { super::irq::IRQ_TIM8_CC }
}

