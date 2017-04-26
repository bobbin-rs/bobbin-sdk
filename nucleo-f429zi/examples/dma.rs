#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f429zi as board;

use board::chip::dma::DMA2;
use board::hal::dma;
use board::hal::rcc;

// Set up Memory-Memory DMA using DMA2 Stream 0
// Interrupt on completion

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running DMA Example");

    let src = [0xffu8; 1024];
    let dst = [0u8; 1024];

    rcc::set_dma_enabled(DMA2, true);
    let s = dma::stream(DMA2, 0);
    s    
        .set_pa(&src as *const u8 as u32)
        .set_m0a(&dst as *const u8 as u32)
        .set_psize(dma::Size::Bit8)
        .set_pinc(true)
        .set_msize(dma::Size::Bit8)
        .set_minc(true)
        .set_dir(dma::Dir::MtoM)
        .set_ndt(1024)
        .clr_teif()
        .clr_tcif()
        .set_enabled(true);

    println!("Waiting for DMA to complete");
    let mut n = 0;
    loop {
        if n == 100_000 {
            panic!("Timeout")
        }
        if s.teif() {
            panic!("DMA Error")
        }
        if s.tcif() {
            break
        }
        n += 1;
    }
    println!("Transfer Complete");
    for i in 0..1024 {
        assert_eq!(src[i], dst[i]);
    }
    println!("Transfer Verified");
    println!("DMA Example Done");
    loop {}
}
