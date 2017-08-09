#![no_std]
#![no_main]

#[macro_use]
extern crate ek_tm4c1294xl as board;

use board::hal::udma::*;
use core::mem;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    
    println!("Running UDMA Test");

    let mut src = [0u8; 32];
    let mut dst = [0u8; 32];

    println!("SRC: {:p}", &src);
    println!("DST: {:p}", &dst);

    for (i, s) in src.iter_mut().enumerate() {
        *s = i as u8;
    }    

    let mut buf = [0u8; 4096];
    let ptr = unsafe {
        let mut ptr = buf.as_mut_ptr();
        while ptr as u32 % 128 != 0 {        
            ptr = ptr.offset(1);
        }
        ptr      
    };
    let desc: &mut Chdesc = unsafe { mem::transmute(ptr) };
    let desc_addr = desc as *const _ as u32;

    let ch = UDMA_CH0;
    let d = ch.periph();

    println!("Configuring DMA");
    d.sysctl_set_enabled(true);
    d.set_cfg(|r| r.set_masten(1));
    d.set_ctlbase(|r| r.set_addr(desc_addr >> 10));

    d.set_prioset(|r| r.set_set(ch.index(), 1));
    d.set_altclr(|r| r.set_clr(ch.index(), 1));
    d.set_useburstclr(|r| r.set_clr(ch.index(), 1));
    d.set_reqmaskclr(|r| r.set_clr(ch.index(), 1));
    unsafe {
        desc.set_srcendp(|_| Srcendp(src.as_ptr().offset(src.len() as isize) as u32));
        desc.set_dstendp(|_| Dstendp(dst.as_mut_ptr().offset(src.len() as isize)as u32));
    }
    desc.set_chctl(|r| r
        .set_dstinc(0x0)
        .set_dstsize(0x0)
        .set_srcinc(0x0)
        .set_srcsize(0x0)
        .set_xfersize(src.len() as u32)
        .set_xfermode(0x2)
    );
    println!("DESC: {:p}", desc);
    println!("   SRCENDP: {:?}", desc.srcendp());
    println!("   DSTENDP: {:?}", desc.dstendp());
    println!("   CHCTL:   {:?}", desc.chctl());

    d.set_enaset(|r| r.set_set(ch.index(), 1));
    d.set_swreq(|r| r.set_swreq(ch.index(), 1));
    println!("Starting DMA Transfer");

    println!("STAT: {:?}", d.stat());
    println!("CTLBASE: {:?}", d.ctlbase());
    println!("DESC: {:p}", desc);
    println!("   SRCENDP: {:?}", desc.srcendp());
    println!("   DSTENDP: {:?}", desc.srcendp());
    println!("   CHCTL:   {:?}", desc.chctl());

    while d.enaset().set(ch.index()) != 0 {}

    if &src[..] == &dst[..] {
        println!("SRC = DST: OK");
    } else {
        println!("SRC != DST: Failure");
    }
    
    // for i in 0..src.len() {
    //     println!("{}: {:02x} {:02x}", i, src[i], dst[i]);
    // }


    println!("DMA Test Complete");
    loop {}    
}
