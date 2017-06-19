#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate arduino_zero as board;

use core::mem;

use board::hal::dmac;
use board::hal::dmac::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    println!("Running DMA Test");    

    let mut src = [0u8; 32];
    let mut dst = [0u8; 32];

    for (i, s) in src.iter_mut().enumerate() {
        *s = i as u8;
    }

    let ch = DMAC_CH0;
    let dma = ch.periph();

    println!("Enabling clocks");
    dma.pm_set_enabled(true);
    
    println!("Resetting DMA");
    dma.set_ctrl(dmac::Ctrl(0).set_swrst(1));

    let mut buf = [0u8; 1024];
    let ptr = unsafe {
        let mut ptr = buf.as_mut_ptr();
        while ptr as u32 % 16 != 0 {        
            ptr = ptr.offset(1);
        }
        ptr      
    };
    println!("ptr: {:08x}", ptr as u32);        
    let desc: &mut Transfer = unsafe { mem::transmute(ptr) };
    let wb: &mut Transfer = unsafe { mem::transmute(ptr.offset(16))};
    println!("desc: {:p}", desc);
    println!("wb: {:p}", wb);
    {
        // SRCADDR / DSTADDR must be the ending values after a transfer
        unsafe {
            desc.set_srcaddr(Srcaddr(src.as_ptr().offset(src.len() as isize) as u32));
            desc.set_dstaddr(Dstaddr(dst.as_mut_ptr().offset(dst.len() as isize) as u32));
        }
        desc.set_btcnt(Btcnt(src.len() as u16));
        desc.with_btctrl(|r| r.set_dstinc(1).set_srcinc(1).set_valid(1));

    }    


    // Set Descriptor Base
    dma.set_baseaddr(Baseaddr(desc as *mut Transfer as u32));
    // Set Writeback Base
    dma.set_wrbaddr(Wrbaddr(wb as *mut Transfer as u32));

    // Set Priority Level 0 Enabled
    dma.with_ctrl(|r| r.set_lvlen(0, 1));

    // Enable DMAC
    dma.with_ctrl(|r| r.set_dmaenable(1));


    println!("SRC:           {:p}", &src);
    println!("DST:           {:p}", &dst);
    println!("DESC:          {:p}", desc);
    println!("  BTCTRL:      {:?}", desc.btctrl());
    println!("  BTCNT:       {:?}", desc.btcnt());
    println!("  SRCADDR:     {:?}", desc.srcaddr());
    println!("  DSTADDR:     {:?}", desc.dstaddr());
    println!("  DESCADDR:    {:?}", desc.descaddr());
    println!("CTRL:          {:?}", dma.ctrl());    
    println!("BASEADDR:      {:?}", dma.baseaddr());
    println!("WRBADDR:       {:?}", dma.wrbaddr());
    println!("BUSYCH:        {:?}", dma.busych());
    println!("PENDCH:        {:?}", dma.pendch());
    println!("ACTIVE:        {:?}", dma.active());


    // Set Channel ID
    dma.set_chid(Chid(0).set_id(ch.index() as u8));
    println!("Resetting channel");
    dma.set_chctrla(Chctrla(0).set_swrst(1));

    dma.set_chid(Chid(0).set_id(ch.index() as u8));
    dma.set_chctrlb(Chctrlb(0).set_trigact(0x3));

    println!("CHID:          {:?}", dma.chid());
    println!("CHCTRLA:       {:?}", dma.chctrla());
    println!("CHCTRLB:       {:?}", dma.chctrlb());
    println!("CHINTFLAG:     {:?}", dma.chintflag());
    println!("CHSTATUS:      {:?}", dma.chstatus());


    println!("Enabling Channel");
    // Set Channel Enabled
    dma.set_chid(Chid(0).set_id(ch.index() as u8));
    dma.set_chctrla(Chctrla(0).set_enable(1));
    println!("CHINTFLAG:     {:?}", dma.chintflag());
    println!("CHSTATUS:      {:?}", dma.chstatus());

    println!("Triggering Channel");
    dma.set_swtrigctrl(Swtrigctrl(0).set_swtrig(ch.index(), 1));
    println!("CHINTFLAG:     {:?}", dma.chintflag());
    println!("CHSTATUS:      {:?}", dma.chstatus());

    loop {
        let f = dma.chintflag();
        if f.terr() != 0 {
            println!("Transfer Error");
            break;
        }
        if f.tcmpl() != 0 {
            println!("Transfer Complete");
            break;
        }
        if f.susp() != 0 {
            println!("Transfer Suspended");
            break;
        }
    }    

    if &src[..] == &dst[..] {
        println!("SRC = DST: OK");
    } else {
        println!("SRC != DST: Failure");
    }

    // for i in 0..32 {
    //     println!("{}: {:02x} {:02x}", i, src[i], dst[i]);
    // }
    println!("DMA Test Done");
    loop {}
}
