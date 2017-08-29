#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::pit::*;

periph!( PIT, Pit, _PIT, PitPeriph, 0x40037000);



channel!(PIT_CH0, PitCh0, PIT, Pit, _PIT_CH0, PitCh, _PIT, 0);
channel!(PIT_CH1, PitCh1, PIT, Pit, _PIT_CH1, PitCh, _PIT, 1);
channel!(PIT_CH2, PitCh2, PIT, Pit, _PIT_CH2, PitCh, _PIT, 2);
channel!(PIT_CH3, PitCh3, PIT, Pit, _PIT_CH3, PitCh, _PIT, 3);

pub trait IrqPit<T> {
   fn irq_pit(&self) -> T;
}

impl IrqPit<super::irq::IrqPit0> for PitCh0 {
   fn irq_pit(&self) -> super::irq::IrqPit0 { super::irq::IRQ_PIT0 }
}

impl IrqPit<super::irq::IrqPit1> for PitCh1 {
   fn irq_pit(&self) -> super::irq::IrqPit1 { super::irq::IRQ_PIT1 }
}

impl IrqPit<super::irq::IrqPit2> for PitCh2 {
   fn irq_pit(&self) -> super::irq::IrqPit2 { super::irq::IRQ_PIT2 }
}

impl IrqPit<super::irq::IrqPit3> for PitCh3 {
   fn irq_pit(&self) -> super::irq::IrqPit3 { super::irq::IRQ_PIT3 }
}

