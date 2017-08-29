#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::lpit::*;

periph!( LPIT0, Lpit0, _LPIT0, LpitPeriph, 0x40037000);



channel!(LPIT0_CH0, Lpit0Ch0, LPIT0, Lpit0, _LPIT0_CH0, LpitCh, _LPIT0, 0);
channel!(LPIT0_CH1, Lpit0Ch1, LPIT0, Lpit0, _LPIT0_CH1, LpitCh, _LPIT0, 1);
channel!(LPIT0_CH2, Lpit0Ch2, LPIT0, Lpit0, _LPIT0_CH2, LpitCh, _LPIT0, 2);
channel!(LPIT0_CH3, Lpit0Ch3, LPIT0, Lpit0, _LPIT0_CH3, LpitCh, _LPIT0, 3);

pub trait IrqLpit<T> {
   fn irq_lpit(&self) -> T;
}

impl IrqLpit<super::irq::IrqLpit0Ch0> for Lpit0Ch0 {
   fn irq_lpit(&self) -> super::irq::IrqLpit0Ch0 { super::irq::IRQ_LPIT0_CH0 }
}

impl IrqLpit<super::irq::IrqLpit0Ch1> for Lpit0Ch1 {
   fn irq_lpit(&self) -> super::irq::IrqLpit0Ch1 { super::irq::IRQ_LPIT0_CH1 }
}

impl IrqLpit<super::irq::IrqLpit0Ch2> for Lpit0Ch2 {
   fn irq_lpit(&self) -> super::irq::IrqLpit0Ch2 { super::irq::IRQ_LPIT0_CH2 }
}

impl IrqLpit<super::irq::IrqLpit0Ch3> for Lpit0Ch3 {
   fn irq_lpit(&self) -> super::irq::IrqLpit0Ch3 { super::irq::IRQ_LPIT0_CH3 }
}

