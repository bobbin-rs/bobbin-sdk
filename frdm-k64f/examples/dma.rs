#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_k64f as board;

use board::hal::edma::*;

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

    println!("Enabling DMA Clock");
    d.sim_set_enabled(true);    


    d.set_tcd_citer_elinkno(ch.index(), |_| TcdCiterElinkno(0x0001));
    d.set_tcd_biter_elinkno(ch.index(), |_| TcdBiterElinkno(0x0001));
    d.set_tcd_nbytes_mlno(ch.index(), |_| TcdNbytesMlno(32));

    d.set_tcd_saddr(ch.index(), |_| TcdSaddr(0).set_saddr(src.as_ptr() as u32));
    d.set_tcd_soff(ch.index(), |_| TcdSoff(0).set_soff(0x1));
    d.with_tcd_attr(ch.index(), |r| r.set_ssize(0));
    d.set_tcd_slast(ch.index(), |_| TcdSlast(0));
    


    d.set_tcd_daddr(ch.index(), |_| TcdDaddr(0).set_daddr(dst.as_mut_ptr() as u32));
    d.set_tcd_doff(ch.index(), |_| TcdDoff(0).set_doff(0x1));
    d.with_tcd_attr(ch.index(), |r| r.set_dsize(0).set_dmod(0).set_smod(0));
    d.set_tcd_dlastsga(ch.index(), |_| TcdDlastsga(0));
    

    d.with_tcd_csr(ch.index(), |r| r.set_intmajor(0).set_inthalf(0).set_majorlinkch(0).set_majorelink(0));

    println!("DMA_CR: {:?}", d.cr());
    println!("DMA_ES: {:?}", d.es());

    println!("TCD_SADDR: {:?}", d.tcd_saddr(ch.index()));
    println!("TCD_SOFF: {:?}", d.tcd_soff(ch.index()));
    println!("TCD_SLAST: {:?}", d.tcd_slast(ch.index()));

    println!("TCD_DADDR: {:?}", d.tcd_daddr(ch.index()));
    println!("TCD_DOFF: {:?}", d.tcd_doff(ch.index()));
    println!("TCD_DLAST: {:?}", d.tcd_dlastsga(ch.index()));

    println!("TCD_ATTR: {:?}", d.tcd_attr(ch.index()));
    println!("TCD_CSR: {:?}", d.tcd_csr(ch.index()));

    println!("Starting DMA Transfer");
    d.with_tcd_csr(ch.index(), |r| r.set_start(1));
    loop {
        let err = d.err();
        if err.err(ch.index()) != 0 {
            println!("DMA Error");
            println!("DMA_ES: {:?}", d.es());

            break;
        }
        let csr = d.tcd_csr(ch.index());
        if csr.done() != 0 {
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
