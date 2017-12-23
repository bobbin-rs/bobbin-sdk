#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::dma_f3::*;

periph!( DMA1, Dma1, _DMA1, DmaPeriph, 0x40020000);
periph!( DMA2, Dma2, _DMA2, DmaPeriph, 0x40020400);





pub trait IrqDma<T> {
    fn irq_dma(&self) -> T;
}

impl IrqDma<super::irq::IrqDma1Ch1> for Dma1 {
    fn irq_dma(&self) -> super::irq::IrqDma1Ch1 { super::irq::IRQ_DMA1_CH1 }
}

impl IrqDma<super::irq::IrqDma1Ch2> for Dma1 {
    fn irq_dma(&self) -> super::irq::IrqDma1Ch2 { super::irq::IRQ_DMA1_CH2 }
}

impl IrqDma<super::irq::IrqDma1Ch3> for Dma1 {
    fn irq_dma(&self) -> super::irq::IrqDma1Ch3 { super::irq::IRQ_DMA1_CH3 }
}

impl IrqDma<super::irq::IrqDma1Ch4> for Dma1 {
    fn irq_dma(&self) -> super::irq::IrqDma1Ch4 { super::irq::IRQ_DMA1_CH4 }
}

impl IrqDma<super::irq::IrqDma1Ch5> for Dma1 {
    fn irq_dma(&self) -> super::irq::IrqDma1Ch5 { super::irq::IRQ_DMA1_CH5 }
}

impl IrqDma<super::irq::IrqDma1Ch6> for Dma1 {
    fn irq_dma(&self) -> super::irq::IrqDma1Ch6 { super::irq::IRQ_DMA1_CH6 }
}

impl IrqDma<super::irq::IrqDma1Ch7> for Dma1 {
    fn irq_dma(&self) -> super::irq::IrqDma1Ch7 { super::irq::IRQ_DMA1_CH7 }
}

impl IrqDma<super::irq::IrqDma2Ch1> for Dma2 {
    fn irq_dma(&self) -> super::irq::IrqDma2Ch1 { super::irq::IRQ_DMA2_CH1 }
}

impl IrqDma<super::irq::IrqDma2Ch2> for Dma2 {
    fn irq_dma(&self) -> super::irq::IrqDma2Ch2 { super::irq::IRQ_DMA2_CH2 }
}

impl IrqDma<super::irq::IrqDma2Ch3> for Dma2 {
    fn irq_dma(&self) -> super::irq::IrqDma2Ch3 { super::irq::IRQ_DMA2_CH3 }
}

impl IrqDma<super::irq::IrqDma2Ch4> for Dma2 {
    fn irq_dma(&self) -> super::irq::IrqDma2Ch4 { super::irq::IRQ_DMA2_CH4 }
}

impl IrqDma<super::irq::IrqDma2Ch5> for Dma2 {
    fn irq_dma(&self) -> super::irq::IrqDma2Ch5 { super::irq::IRQ_DMA2_CH5 }
}

impl IrqDma<super::irq::IrqDma2Ch6> for Dma2 {
    fn irq_dma(&self) -> super::irq::IrqDma2Ch6 { super::irq::IRQ_DMA2_CH6 }
}

impl IrqDma<super::irq::IrqDma2Ch7> for Dma2 {
    fn irq_dma(&self) -> super::irq::IrqDma2Ch7 { super::irq::IRQ_DMA2_CH7 }
}

