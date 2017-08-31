#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::edma::*;

periph!( DMA, Dma, _DMA, EdmaPeriph, 0x40008000);



channel!(DMA0, Dma0, DMA, Dma, _DMA0, EdmaCh, _DMA, 0);
channel!(DMA1, Dma1, DMA, Dma, _DMA1, EdmaCh, _DMA, 1);
channel!(DMA2, Dma2, DMA, Dma, _DMA2, EdmaCh, _DMA, 2);
channel!(DMA3, Dma3, DMA, Dma, _DMA3, EdmaCh, _DMA, 3);
channel!(DMA4, Dma4, DMA, Dma, _DMA4, EdmaCh, _DMA, 4);
channel!(DMA5, Dma5, DMA, Dma, _DMA5, EdmaCh, _DMA, 5);
channel!(DMA6, Dma6, DMA, Dma, _DMA6, EdmaCh, _DMA, 6);
channel!(DMA7, Dma7, DMA, Dma, _DMA7, EdmaCh, _DMA, 7);
channel!(DMA8, Dma8, DMA, Dma, _DMA8, EdmaCh, _DMA, 8);
channel!(DMA9, Dma9, DMA, Dma, _DMA9, EdmaCh, _DMA, 9);
channel!(DMA10, Dma10, DMA, Dma, _DMA10, EdmaCh, _DMA, 10);
channel!(DMA11, Dma11, DMA, Dma, _DMA11, EdmaCh, _DMA, 11);
channel!(DMA12, Dma12, DMA, Dma, _DMA12, EdmaCh, _DMA, 12);
channel!(DMA13, Dma13, DMA, Dma, _DMA13, EdmaCh, _DMA, 13);
channel!(DMA14, Dma14, DMA, Dma, _DMA14, EdmaCh, _DMA, 14);
channel!(DMA15, Dma15, DMA, Dma, _DMA15, EdmaCh, _DMA, 15);

pub trait IrqDmaError<T> {
   fn irq_dma_error(&self) -> T;
}

pub trait IrqDma<T> {
   fn irq_dma(&self) -> T;
}

impl IrqDmaError<super::irq::IrqDmaError> for Dma {
   fn irq_dma_error(&self) -> super::irq::IrqDmaError { super::irq::IRQ_DMA_ERROR }
}

impl IrqDma<super::irq::IrqDma0> for Dma0 {
   fn irq_dma(&self) -> super::irq::IrqDma0 { super::irq::IRQ_DMA0 }
}

impl IrqDma<super::irq::IrqDma1> for Dma1 {
   fn irq_dma(&self) -> super::irq::IrqDma1 { super::irq::IRQ_DMA1 }
}

impl IrqDma<super::irq::IrqDma2> for Dma2 {
   fn irq_dma(&self) -> super::irq::IrqDma2 { super::irq::IRQ_DMA2 }
}

impl IrqDma<super::irq::IrqDma3> for Dma3 {
   fn irq_dma(&self) -> super::irq::IrqDma3 { super::irq::IRQ_DMA3 }
}

impl IrqDma<super::irq::IrqDma4> for Dma4 {
   fn irq_dma(&self) -> super::irq::IrqDma4 { super::irq::IRQ_DMA4 }
}

impl IrqDma<super::irq::IrqDma5> for Dma5 {
   fn irq_dma(&self) -> super::irq::IrqDma5 { super::irq::IRQ_DMA5 }
}

impl IrqDma<super::irq::IrqDma6> for Dma6 {
   fn irq_dma(&self) -> super::irq::IrqDma6 { super::irq::IRQ_DMA6 }
}

impl IrqDma<super::irq::IrqDma7> for Dma7 {
   fn irq_dma(&self) -> super::irq::IrqDma7 { super::irq::IRQ_DMA7 }
}

impl IrqDma<super::irq::IrqDma8> for Dma8 {
   fn irq_dma(&self) -> super::irq::IrqDma8 { super::irq::IRQ_DMA8 }
}

impl IrqDma<super::irq::IrqDma9> for Dma9 {
   fn irq_dma(&self) -> super::irq::IrqDma9 { super::irq::IRQ_DMA9 }
}

impl IrqDma<super::irq::IrqDma10> for Dma10 {
   fn irq_dma(&self) -> super::irq::IrqDma10 { super::irq::IRQ_DMA10 }
}

impl IrqDma<super::irq::IrqDma11> for Dma11 {
   fn irq_dma(&self) -> super::irq::IrqDma11 { super::irq::IRQ_DMA11 }
}

impl IrqDma<super::irq::IrqDma12> for Dma12 {
   fn irq_dma(&self) -> super::irq::IrqDma12 { super::irq::IRQ_DMA12 }
}

impl IrqDma<super::irq::IrqDma13> for Dma13 {
   fn irq_dma(&self) -> super::irq::IrqDma13 { super::irq::IRQ_DMA13 }
}

impl IrqDma<super::irq::IrqDma14> for Dma14 {
   fn irq_dma(&self) -> super::irq::IrqDma14 { super::irq::IRQ_DMA14 }
}

impl IrqDma<super::irq::IrqDma15> for Dma15 {
   fn irq_dma(&self) -> super::irq::IrqDma15 { super::irq::IRQ_DMA15 }
}

