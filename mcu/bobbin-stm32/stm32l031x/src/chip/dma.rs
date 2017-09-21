#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::dma_f3::*;

periph!( DMA1, Dma1, _DMA1, DmaPeriph, 0x40020000);



channel!(DMA1_CH1, Dma1Ch1, DMA1, Dma1, _DMA1_CH1, DmaCh, _DMA1, 0);
channel!(DMA1_CH2, Dma1Ch2, DMA1, Dma1, _DMA1_CH2, DmaCh, _DMA1, 1);
channel!(DMA1_CH3, Dma1Ch3, DMA1, Dma1, _DMA1_CH3, DmaCh, _DMA1, 2);
channel!(DMA1_CH4, Dma1Ch4, DMA1, Dma1, _DMA1_CH4, DmaCh, _DMA1, 3);
channel!(DMA1_CH5, Dma1Ch5, DMA1, Dma1, _DMA1_CH5, DmaCh, _DMA1, 4);
channel!(DMA1_CH6, Dma1Ch6, DMA1, Dma1, _DMA1_CH6, DmaCh, _DMA1, 5);
channel!(DMA1_CH7, Dma1Ch7, DMA1, Dma1, _DMA1_CH7, DmaCh, _DMA1, 6);

pub trait IrqDma<T> {
    fn irq_dma(&self) -> T;
}

impl IrqDma<super::irq::IrqDma1Ch1> for Dma1Ch1 {
    fn irq_dma(&self) -> super::irq::IrqDma1Ch1 { super::irq::IRQ_DMA1_CH1 }
}

impl IrqDma<super::irq::IrqDma1Ch2> for Dma1Ch2 {
    fn irq_dma(&self) -> super::irq::IrqDma1Ch2 { super::irq::IRQ_DMA1_CH2 }
}

impl IrqDma<super::irq::IrqDma1Ch3> for Dma1Ch3 {
    fn irq_dma(&self) -> super::irq::IrqDma1Ch3 { super::irq::IRQ_DMA1_CH3 }
}

impl IrqDma<super::irq::IrqDma1Ch4> for Dma1Ch4 {
    fn irq_dma(&self) -> super::irq::IrqDma1Ch4 { super::irq::IRQ_DMA1_CH4 }
}

impl IrqDma<super::irq::IrqDma1Ch5> for Dma1Ch5 {
    fn irq_dma(&self) -> super::irq::IrqDma1Ch5 { super::irq::IRQ_DMA1_CH5 }
}

impl IrqDma<super::irq::IrqDma1Ch6> for Dma1Ch6 {
    fn irq_dma(&self) -> super::irq::IrqDma1Ch6 { super::irq::IRQ_DMA1_CH6 }
}

