pub use stm32_common::chip::dma::*;

pub const DMA1: Dma1 = Periph(0x40026000, Dma1Id {});
pub const DMA2: Dma2 = Periph(0x40026400, Dma2Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dma1Id {}
pub type Dma1 = Periph<Dma1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dma2Id {}
pub type Dma2 = Periph<Dma2Id>;




pub struct Channel<P, T> { pub periph: Periph<T>, pub index: usize, pub id: P }

impl<P,T> Channel<P,T> {
   #[inline] pub fn periph(&self) -> &Periph<T> { &self.periph }
   #[inline] pub fn index(&self) -> usize { self.index }
}

pub const DMA1_STREAM0: Channel<Dma1Stream0Id, Dma1Id> = Channel { periph: DMA1, index: 0, id: Dma1Stream0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Dma1Stream0Id {}
pub type Dma1Stream0 = Channel<Dma1Stream0Id, Dma1Id>;

pub const DMA1_STREAM1: Channel<Dma1Stream1Id, Dma1Id> = Channel { periph: DMA1, index: 1, id: Dma1Stream1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Dma1Stream1Id {}
pub type Dma1Stream1 = Channel<Dma1Stream1Id, Dma1Id>;

pub const DMA1_STREAM2: Channel<Dma1Stream2Id, Dma1Id> = Channel { periph: DMA1, index: 2, id: Dma1Stream2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Dma1Stream2Id {}
pub type Dma1Stream2 = Channel<Dma1Stream2Id, Dma1Id>;

pub const DMA1_STREAM3: Channel<Dma1Stream3Id, Dma1Id> = Channel { periph: DMA1, index: 3, id: Dma1Stream3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Dma1Stream3Id {}
pub type Dma1Stream3 = Channel<Dma1Stream3Id, Dma1Id>;

pub const DMA1_STREAM4: Channel<Dma1Stream4Id, Dma1Id> = Channel { periph: DMA1, index: 4, id: Dma1Stream4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Dma1Stream4Id {}
pub type Dma1Stream4 = Channel<Dma1Stream4Id, Dma1Id>;

pub const DMA1_STREAM5: Channel<Dma1Stream5Id, Dma1Id> = Channel { periph: DMA1, index: 5, id: Dma1Stream5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Dma1Stream5Id {}
pub type Dma1Stream5 = Channel<Dma1Stream5Id, Dma1Id>;

pub const DMA1_STREAM6: Channel<Dma1Stream6Id, Dma1Id> = Channel { periph: DMA1, index: 6, id: Dma1Stream6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Dma1Stream6Id {}
pub type Dma1Stream6 = Channel<Dma1Stream6Id, Dma1Id>;

pub const DMA1_STREAM7: Channel<Dma1Stream7Id, Dma1Id> = Channel { periph: DMA1, index: 7, id: Dma1Stream7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Dma1Stream7Id {}
pub type Dma1Stream7 = Channel<Dma1Stream7Id, Dma1Id>;

pub const DMA2_STREAM0: Channel<Dma2Stream0Id, Dma2Id> = Channel { periph: DMA2, index: 0, id: Dma2Stream0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Dma2Stream0Id {}
pub type Dma2Stream0 = Channel<Dma2Stream0Id, Dma2Id>;

pub const DMA2_STREAM1: Channel<Dma2Stream1Id, Dma2Id> = Channel { periph: DMA2, index: 1, id: Dma2Stream1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Dma2Stream1Id {}
pub type Dma2Stream1 = Channel<Dma2Stream1Id, Dma2Id>;

pub const DMA2_STREAM2: Channel<Dma2Stream2Id, Dma2Id> = Channel { periph: DMA2, index: 2, id: Dma2Stream2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Dma2Stream2Id {}
pub type Dma2Stream2 = Channel<Dma2Stream2Id, Dma2Id>;

pub const DMA2_STREAM3: Channel<Dma2Stream3Id, Dma2Id> = Channel { periph: DMA2, index: 3, id: Dma2Stream3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Dma2Stream3Id {}
pub type Dma2Stream3 = Channel<Dma2Stream3Id, Dma2Id>;

pub const DMA2_STREAM4: Channel<Dma2Stream4Id, Dma2Id> = Channel { periph: DMA2, index: 4, id: Dma2Stream4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Dma2Stream4Id {}
pub type Dma2Stream4 = Channel<Dma2Stream4Id, Dma2Id>;

pub const DMA2_STREAM5: Channel<Dma2Stream5Id, Dma2Id> = Channel { periph: DMA2, index: 5, id: Dma2Stream5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Dma2Stream5Id {}
pub type Dma2Stream5 = Channel<Dma2Stream5Id, Dma2Id>;

pub const DMA2_STREAM6: Channel<Dma2Stream6Id, Dma2Id> = Channel { periph: DMA2, index: 6, id: Dma2Stream6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Dma2Stream6Id {}
pub type Dma2Stream6 = Channel<Dma2Stream6Id, Dma2Id>;

pub const DMA2_STREAM7: Channel<Dma2Stream7Id, Dma2Id> = Channel { periph: DMA2, index: 7, id: Dma2Stream7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Dma2Stream7Id {}
pub type Dma2Stream7 = Channel<Dma2Stream7Id, Dma2Id>;

pub trait IrqDma<T> {
   fn irq_dma(&self) -> super::irq::Irq<T>;
}

pub trait RegisterDmaHandler {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleDma {
   fn handle_dma(&self);
}

impl IrqDma<super::irq::Dma1Stream0Id> for Dma1Stream0 {
   fn irq_dma(&self) -> super::irq::IrqDma1Stream0 { super::irq::IRQ_DMA1_STREAM0 }
}

impl RegisterDmaHandler for Dma1Stream0 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(11, Some(wrapper::<F>));
       super::irq::IrqGuard::new(11)
   }
}

impl IrqDma<super::irq::Dma1Stream1Id> for Dma1Stream1 {
   fn irq_dma(&self) -> super::irq::IrqDma1Stream1 { super::irq::IRQ_DMA1_STREAM1 }
}

impl RegisterDmaHandler for Dma1Stream1 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(12, Some(wrapper::<F>));
       super::irq::IrqGuard::new(12)
   }
}

impl IrqDma<super::irq::Dma1Stream2Id> for Dma1Stream2 {
   fn irq_dma(&self) -> super::irq::IrqDma1Stream2 { super::irq::IRQ_DMA1_STREAM2 }
}

impl RegisterDmaHandler for Dma1Stream2 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(13, Some(wrapper::<F>));
       super::irq::IrqGuard::new(13)
   }
}

impl IrqDma<super::irq::Dma1Stream3Id> for Dma1Stream3 {
   fn irq_dma(&self) -> super::irq::IrqDma1Stream3 { super::irq::IRQ_DMA1_STREAM3 }
}

impl RegisterDmaHandler for Dma1Stream3 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(14, Some(wrapper::<F>));
       super::irq::IrqGuard::new(14)
   }
}

impl IrqDma<super::irq::Dma1Stream4Id> for Dma1Stream4 {
   fn irq_dma(&self) -> super::irq::IrqDma1Stream4 { super::irq::IRQ_DMA1_STREAM4 }
}

impl RegisterDmaHandler for Dma1Stream4 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(15, Some(wrapper::<F>));
       super::irq::IrqGuard::new(15)
   }
}

impl IrqDma<super::irq::Dma1Stream5Id> for Dma1Stream5 {
   fn irq_dma(&self) -> super::irq::IrqDma1Stream5 { super::irq::IRQ_DMA1_STREAM5 }
}

impl RegisterDmaHandler for Dma1Stream5 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(16, Some(wrapper::<F>));
       super::irq::IrqGuard::new(16)
   }
}

impl IrqDma<super::irq::Dma1Stream6Id> for Dma1Stream6 {
   fn irq_dma(&self) -> super::irq::IrqDma1Stream6 { super::irq::IRQ_DMA1_STREAM6 }
}

impl RegisterDmaHandler for Dma1Stream6 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(17, Some(wrapper::<F>));
       super::irq::IrqGuard::new(17)
   }
}

impl IrqDma<super::irq::Dma1Stream7Id> for Dma1Stream7 {
   fn irq_dma(&self) -> super::irq::IrqDma1Stream7 { super::irq::IRQ_DMA1_STREAM7 }
}

impl RegisterDmaHandler for Dma1Stream7 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(47, Some(wrapper::<F>));
       super::irq::IrqGuard::new(47)
   }
}

impl IrqDma<super::irq::Dma2Stream0Id> for Dma2Stream0 {
   fn irq_dma(&self) -> super::irq::IrqDma2Stream0 { super::irq::IRQ_DMA2_STREAM0 }
}

impl RegisterDmaHandler for Dma2Stream0 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(56, Some(wrapper::<F>));
       super::irq::IrqGuard::new(56)
   }
}

impl IrqDma<super::irq::Dma2Stream1Id> for Dma2Stream1 {
   fn irq_dma(&self) -> super::irq::IrqDma2Stream1 { super::irq::IRQ_DMA2_STREAM1 }
}

impl RegisterDmaHandler for Dma2Stream1 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(57, Some(wrapper::<F>));
       super::irq::IrqGuard::new(57)
   }
}

impl IrqDma<super::irq::Dma2Stream2Id> for Dma2Stream2 {
   fn irq_dma(&self) -> super::irq::IrqDma2Stream2 { super::irq::IRQ_DMA2_STREAM2 }
}

impl RegisterDmaHandler for Dma2Stream2 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(58, Some(wrapper::<F>));
       super::irq::IrqGuard::new(58)
   }
}

impl IrqDma<super::irq::Dma2Stream3Id> for Dma2Stream3 {
   fn irq_dma(&self) -> super::irq::IrqDma2Stream3 { super::irq::IRQ_DMA2_STREAM3 }
}

impl RegisterDmaHandler for Dma2Stream3 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(59, Some(wrapper::<F>));
       super::irq::IrqGuard::new(59)
   }
}

impl IrqDma<super::irq::Dma2Stream4Id> for Dma2Stream4 {
   fn irq_dma(&self) -> super::irq::IrqDma2Stream4 { super::irq::IRQ_DMA2_STREAM4 }
}

impl RegisterDmaHandler for Dma2Stream4 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(60, Some(wrapper::<F>));
       super::irq::IrqGuard::new(60)
   }
}

impl IrqDma<super::irq::Dma2Stream5Id> for Dma2Stream5 {
   fn irq_dma(&self) -> super::irq::IrqDma2Stream5 { super::irq::IRQ_DMA2_STREAM5 }
}

impl RegisterDmaHandler for Dma2Stream5 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(68, Some(wrapper::<F>));
       super::irq::IrqGuard::new(68)
   }
}

impl IrqDma<super::irq::Dma2Stream6Id> for Dma2Stream6 {
   fn irq_dma(&self) -> super::irq::IrqDma2Stream6 { super::irq::IRQ_DMA2_STREAM6 }
}

impl RegisterDmaHandler for Dma2Stream6 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(69, Some(wrapper::<F>));
       super::irq::IrqGuard::new(69)
   }
}

impl IrqDma<super::irq::Dma2Stream7Id> for Dma2Stream7 {
   fn irq_dma(&self) -> super::irq::IrqDma2Stream7 { super::irq::IRQ_DMA2_STREAM7 }
}

impl RegisterDmaHandler for Dma2Stream7 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(70, Some(wrapper::<F>));
       super::irq::IrqGuard::new(70)
   }
}

