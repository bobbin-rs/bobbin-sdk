#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate onebit_elle0 as board;

use board::hal::dma::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    println!("DMA Test");

    let src = [0xffu8; 1024];
    let dst = [0u8; 1024];
    
    // NOTE: Only DMA2 supports Memory-to-Memory transfers

    let ch = DMA2_STREAM0;

    let dma_test = DmaTest::new(ch);
    let _g = ch.register_dma_handler(&dma_test);
    ch.irq_dma().set_enabled(true);

    ch.periph().rcc_set_enabled(true);
    ch    
        .set_pa(&src as *const u8 as u32)
        .set_m0a(&dst as *const u8 as u32)
        .set_psize(Size::Bit8)
        .set_pinc(true)
        .set_msize(Size::Bit8)
        .set_minc(true)
        .set_dir(Dir::MtoM)
        .set_ndt(1024)
        .set_tcie(true)
        .clr_teif()
        .clr_tcif();

    println!("Starting DMA Transfer");
    ch.clr_tcif().set_enabled(true);

    //while !ch.tcif() {}
    while !dma_test.done() {}
    for i in 0..1024 {
        assert_eq!(src[i], dst[i]);
    }
    println!("Transfer Verified");
    println!("DMA Example Done");

    loop {}
}

use core::cell::UnsafeCell;

unsafe impl<P, T> Sync for DmaTest<P, T> {}
unsafe impl<P, T> Send for DmaTest<P, T> {}

pub struct DmaTest<P, T> {
    channel: Channel<P, T>,
    done: UnsafeCell<bool>,
}

impl<P, T> DmaTest<P, T> {
    fn new(channel: Channel<P, T>) -> Self {
        DmaTest { channel: channel, done: UnsafeCell::new(false) }
    }

    fn done(&self) -> bool {
        use core::ptr;
        unsafe { ptr::read_volatile(self.done.get()) }
    }
}


impl<P, T> HandleDma for DmaTest<P, T> {
    fn handle_dma(&self) {
        use core::ptr;

        self.channel.clr_tcif();
        println!("** handle_dma **");
        unsafe {
            ptr::write_volatile(self.done.get(), true);
        }
        
    }
}