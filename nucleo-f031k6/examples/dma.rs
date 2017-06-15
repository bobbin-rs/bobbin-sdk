#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate nucleo_f031k6 as board;

use board::hal::dma::*;
#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    println!("DMA Test");

    let src = [0xffu8; 1024];
    let dst = [0u8; 1024];
    
    let ch = DMA1_CH1;
    ch.periph().rcc_set_enabled(true);
    println!("Enabled? {}", ch.periph().rcc_enabled());
    ch    
        .set_pa(&src as *const u8 as u32)
        .set_ma(&dst as *const u8 as u32)
        .set_psize(Size::Bit8)
        .set_pinc(true)
        .set_msize(Size::Bit8)
        .set_minc(true)
        .set_mem2mem(true)
        .set_ndt(1024)
        .clr_teif()
        .clr_tcif();

    println!("Starting DMA Transfer");
    ch.clr_tcif().set_enabled(true);

    while !ch.tcif() {}
    for i in 0..1024 {
        assert_eq!(src[i], dst[i]);
    }
    println!("Transfer Verified");
    println!("DMA Example Done");

    loop {}
}
