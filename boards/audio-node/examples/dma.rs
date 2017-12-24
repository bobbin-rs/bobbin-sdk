#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate audio_node as board;

use board::hal::dma::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    println!("DMA Test");

    let src = [0xffu8; 1024];
    let dst = [0u8; 1024];
    
    let ch = DMA1_CH1;

    // let dma_test = DmaTest::new(ch);
    // let _g = ch.register_dma_handler(&dma_test);
    // ch.irq_dma().set_enabled(true);

    ch.periph().rcc_set_enabled(true);
    
    ch    
        .set_pa(&src as *const u8 as u32)
        .set_ma(&dst as *const u8 as u32)
        .set_psize(Size::Bit8)
        .set_pinc(true)
        .set_msize(Size::Bit8)
        .set_minc(true)
        .set_mem2mem(true)
        .set_ndt(1024)
        .set_tcie(true)
        .clr_teif()
        .clr_tcif();

    println!("Starting DMA Transfer");
    ch.clr_tcif().set_enabled(true);

    while !ch.tcif() {}

    // while !dma_test.done() {}

    for i in 0..1024 {
        assert_eq!(src[i], dst[i]);
    }
    println!("Transfer Verified");
    println!("DMA Example Done");

    loop {}
}

// use core::cell::UnsafeCell;

// unsafe impl<P> Sync for DmaTest<P> {}
// unsafe impl<P> Send for DmaTest<P> {}

// pub struct DmaTest<P: Sized> {
//     channel: Channel<P>,
//     done: UnsafeCell<bool>,
// }

// impl<P: Sized> DmaTest<P> {
//     fn new(channel: Channel<P>) -> Self {
//         DmaTest { channel: channel, done: UnsafeCell::new(false) }
//     }

//     fn done(&self) -> bool {
//         use core::ptr;
//         unsafe { ptr::read_volatile(self.done.get()) }
//     }
// }


// impl<P: Sized> Poll for DmaTest<P> {
//     fn poll(&self) {
//         use core::ptr;

//         self.channel.clr_tcif();
//         println!("** handle_dma **");
//         unsafe {
//             ptr::write_volatile(self.done.get(), true);
//         }
        
//     }
// }