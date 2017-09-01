#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::dma::*;

periph!( DMA1, Dma1, _DMA1, DmaPeriph, 0x40026000);
periph!( DMA2, Dma2, _DMA2, DmaPeriph, 0x40026400);




channel!(DMA1_STREAM0, Dma1Stream0, DMA1, Dma1, _DMA1_STREAM0, DmaCh, _DMA1, 0);
channel!(DMA1_STREAM1, Dma1Stream1, DMA1, Dma1, _DMA1_STREAM1, DmaCh, _DMA1, 1);
channel!(DMA1_STREAM2, Dma1Stream2, DMA1, Dma1, _DMA1_STREAM2, DmaCh, _DMA1, 2);
channel!(DMA1_STREAM3, Dma1Stream3, DMA1, Dma1, _DMA1_STREAM3, DmaCh, _DMA1, 3);
channel!(DMA1_STREAM4, Dma1Stream4, DMA1, Dma1, _DMA1_STREAM4, DmaCh, _DMA1, 4);
channel!(DMA1_STREAM5, Dma1Stream5, DMA1, Dma1, _DMA1_STREAM5, DmaCh, _DMA1, 5);
channel!(DMA1_STREAM6, Dma1Stream6, DMA1, Dma1, _DMA1_STREAM6, DmaCh, _DMA1, 6);
channel!(DMA1_STREAM7, Dma1Stream7, DMA1, Dma1, _DMA1_STREAM7, DmaCh, _DMA1, 7);
channel!(DMA2_STREAM0, Dma2Stream0, DMA2, Dma2, _DMA2_STREAM0, DmaCh, _DMA2, 0);
channel!(DMA2_STREAM1, Dma2Stream1, DMA2, Dma2, _DMA2_STREAM1, DmaCh, _DMA2, 1);
channel!(DMA2_STREAM2, Dma2Stream2, DMA2, Dma2, _DMA2_STREAM2, DmaCh, _DMA2, 2);
channel!(DMA2_STREAM3, Dma2Stream3, DMA2, Dma2, _DMA2_STREAM3, DmaCh, _DMA2, 3);
channel!(DMA2_STREAM4, Dma2Stream4, DMA2, Dma2, _DMA2_STREAM4, DmaCh, _DMA2, 4);
channel!(DMA2_STREAM5, Dma2Stream5, DMA2, Dma2, _DMA2_STREAM5, DmaCh, _DMA2, 5);
channel!(DMA2_STREAM6, Dma2Stream6, DMA2, Dma2, _DMA2_STREAM6, DmaCh, _DMA2, 6);
channel!(DMA2_STREAM7, Dma2Stream7, DMA2, Dma2, _DMA2_STREAM7, DmaCh, _DMA2, 7);

pub trait IrqDma<T> {
    fn irq_dma(&self) -> T;
}

impl IrqDma<super::irq::IrqDma1Stream0> for Dma1Stream0 {
    fn irq_dma(&self) -> super::irq::IrqDma1Stream0 { super::irq::IRQ_DMA1_STREAM0 }
}

impl IrqDma<super::irq::IrqDma1Stream1> for Dma1Stream1 {
    fn irq_dma(&self) -> super::irq::IrqDma1Stream1 { super::irq::IRQ_DMA1_STREAM1 }
}

impl IrqDma<super::irq::IrqDma1Stream2> for Dma1Stream2 {
    fn irq_dma(&self) -> super::irq::IrqDma1Stream2 { super::irq::IRQ_DMA1_STREAM2 }
}

impl IrqDma<super::irq::IrqDma1Stream3> for Dma1Stream3 {
    fn irq_dma(&self) -> super::irq::IrqDma1Stream3 { super::irq::IRQ_DMA1_STREAM3 }
}

impl IrqDma<super::irq::IrqDma1Stream4> for Dma1Stream4 {
    fn irq_dma(&self) -> super::irq::IrqDma1Stream4 { super::irq::IRQ_DMA1_STREAM4 }
}

impl IrqDma<super::irq::IrqDma1Stream5> for Dma1Stream5 {
    fn irq_dma(&self) -> super::irq::IrqDma1Stream5 { super::irq::IRQ_DMA1_STREAM5 }
}

impl IrqDma<super::irq::IrqDma1Stream6> for Dma1Stream6 {
    fn irq_dma(&self) -> super::irq::IrqDma1Stream6 { super::irq::IRQ_DMA1_STREAM6 }
}

impl IrqDma<super::irq::IrqDma1Stream7> for Dma1Stream7 {
    fn irq_dma(&self) -> super::irq::IrqDma1Stream7 { super::irq::IRQ_DMA1_STREAM7 }
}

impl IrqDma<super::irq::IrqDma2Stream0> for Dma2Stream0 {
    fn irq_dma(&self) -> super::irq::IrqDma2Stream0 { super::irq::IRQ_DMA2_STREAM0 }
}

impl IrqDma<super::irq::IrqDma2Stream1> for Dma2Stream1 {
    fn irq_dma(&self) -> super::irq::IrqDma2Stream1 { super::irq::IRQ_DMA2_STREAM1 }
}

impl IrqDma<super::irq::IrqDma2Stream2> for Dma2Stream2 {
    fn irq_dma(&self) -> super::irq::IrqDma2Stream2 { super::irq::IRQ_DMA2_STREAM2 }
}

impl IrqDma<super::irq::IrqDma2Stream3> for Dma2Stream3 {
    fn irq_dma(&self) -> super::irq::IrqDma2Stream3 { super::irq::IRQ_DMA2_STREAM3 }
}

impl IrqDma<super::irq::IrqDma2Stream4> for Dma2Stream4 {
    fn irq_dma(&self) -> super::irq::IrqDma2Stream4 { super::irq::IRQ_DMA2_STREAM4 }
}

impl IrqDma<super::irq::IrqDma2Stream5> for Dma2Stream5 {
    fn irq_dma(&self) -> super::irq::IrqDma2Stream5 { super::irq::IRQ_DMA2_STREAM5 }
}

impl IrqDma<super::irq::IrqDma2Stream6> for Dma2Stream6 {
    fn irq_dma(&self) -> super::irq::IrqDma2Stream6 { super::irq::IRQ_DMA2_STREAM6 }
}

impl IrqDma<super::irq::IrqDma2Stream7> for Dma2Stream7 {
    fn irq_dma(&self) -> super::irq::IrqDma2Stream7 { super::irq::IRQ_DMA2_STREAM7 }
}

