#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_kl26z as board;

use board::hal::dma::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("DMA Test");

    let mut src = [0u8; 32];
    let mut dst = [0u8; 32];

    println!("SRC: {:p}", &src);
    println!("DST: {:p}", &dst);

    for (i, s) in src.iter_mut().enumerate() {
        *s = i as u8;
    }

    let ch = DMA0;
    let d = ch.periph();

    d.sim_set_enabled(true);

    d.set_sar(ch.index(), |r| r.set_sar(src.as_ptr() as u32));    
    d.set_dar(ch.index(), |r| r.set_dar(dst.as_mut_ptr() as u32));
    d.set_dsr_bcr(ch.index(), |r| r.set_bcr(src.len() as u32));
    println!("SAR: {:?}", d.sar(ch.index()));
    println!("DAR: {:?}", d.dar(ch.index()));
    println!("BSR_BCR: {:?}", d.dsr_bcr(ch.index()));    
    d.set_dcr(ch.index(), |r| r
        .set_sinc(1)
        .set_ssize(0b01)
        .set_smod(0b0000)
        .set_dinc(1)
        .set_dsize(0b01)
        .set_dmod(0b0000)
        .set_start(1)
    );


    println!("DCR: {:?}", d.dcr(ch.index()));

    loop {
        let dsr = d.dsr_bcr(ch.index());
        if dsr.ce() != 0 {
            println!("Configuration Error");
            break;
        }
        if dsr.bes() != 0 {
            println!("Bus Error on Source");
            break;
        }
        if dsr.bed() != 0 {
            println!("Bus Error on Destination");
            break;
        }
        if dsr.done() != 0 {
            println!("DMA Transfer Done");
            break;
        }
    }

    

    if &src[..] == &dst[..] {
        println!("SRC = DST: OK");
    } else {
        println!("SRC != DST: Failure");
    }
    
    // for i in 0..src.len() {
    //     println!("{}: {:02x} {:02x}", i, src[i], dst[i]);
    // }


    println!("DMA Test Complete");
    loop {
    }
}
