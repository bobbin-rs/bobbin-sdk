#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::dma_f3::*;

periph!( DMA1, Dma1, _DMA1, DmaPeriph, 0x40020000);
periph!( DMA2, Dma2, _DMA2, DmaPeriph, 0x40020400);




channel!(DMA1_CH1, Dma1Ch1, DMA1, Dma1, _DMA1_CH1, DmaCh, _DMA1, 0);
channel!(DMA1_CH2, Dma1Ch2, DMA1, Dma1, _DMA1_CH2, DmaCh, _DMA1, 1);
channel!(DMA1_CH3, Dma1Ch3, DMA1, Dma1, _DMA1_CH3, DmaCh, _DMA1, 2);
channel!(DMA1_CH4, Dma1Ch4, DMA1, Dma1, _DMA1_CH4, DmaCh, _DMA1, 3);
channel!(DMA1_CH5, Dma1Ch5, DMA1, Dma1, _DMA1_CH5, DmaCh, _DMA1, 4);
channel!(DMA1_CH6, Dma1Ch6, DMA1, Dma1, _DMA1_CH6, DmaCh, _DMA1, 5);
channel!(DMA1_CH7, Dma1Ch7, DMA1, Dma1, _DMA1_CH7, DmaCh, _DMA1, 6);
channel!(DMA2_CH1, Dma2Ch1, DMA2, Dma2, _DMA2_CH1, DmaCh, _DMA2, 0);
channel!(DMA2_CH2, Dma2Ch2, DMA2, Dma2, _DMA2_CH2, DmaCh, _DMA2, 1);
channel!(DMA2_CH3, Dma2Ch3, DMA2, Dma2, _DMA2_CH3, DmaCh, _DMA2, 2);
channel!(DMA2_CH4, Dma2Ch4, DMA2, Dma2, _DMA2_CH4, DmaCh, _DMA2, 3);
channel!(DMA2_CH5, Dma2Ch5, DMA2, Dma2, _DMA2_CH5, DmaCh, _DMA2, 4);

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

impl IrqDma<super::irq::IrqDma1Ch7> for Dma1Ch7 {
    fn irq_dma(&self) -> super::irq::IrqDma1Ch7 { super::irq::IRQ_DMA1_CH7 }
}

impl IrqDma<super::irq::IrqDma2Ch1> for Dma2Ch1 {
    fn irq_dma(&self) -> super::irq::IrqDma2Ch1 { super::irq::IRQ_DMA2_CH1 }
}

impl IrqDma<super::irq::IrqDma2Ch2> for Dma2Ch2 {
    fn irq_dma(&self) -> super::irq::IrqDma2Ch2 { super::irq::IRQ_DMA2_CH2 }
}

impl IrqDma<super::irq::IrqDma2Ch3> for Dma2Ch3 {
    fn irq_dma(&self) -> super::irq::IrqDma2Ch3 { super::irq::IRQ_DMA2_CH3 }
}

impl IrqDma<super::irq::IrqDma2Ch4> for Dma2Ch4 {
    fn irq_dma(&self) -> super::irq::IrqDma2Ch4 { super::irq::IRQ_DMA2_CH4 }
}

impl IrqDma<super::irq::IrqDma2Ch5> for Dma2Ch5 {
    fn irq_dma(&self) -> super::irq::IrqDma2Ch5 { super::irq::IRQ_DMA2_CH5 }
}

