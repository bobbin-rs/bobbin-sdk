#![no_std]
#![no_main]

#[macro_use]
extern crate arduino_zero as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("[start] Running tests for arduino-zero");
    test_systick();
    test_dma();

    println!("[done] All tests passed");
    loop {}
}


fn test_systick() {
    use board::hal::systick;

    let reload_value = 1000;

    // println!("# Disable Systick");
    systick::set_enabled(false);
    assert!(!systick::enabled());

    // println!("# Set Reload Value");
    systick::set_reload_value(reload_value);
    assert_eq!(systick::reload_value(), reload_value);

    // println!("# Set Current Value");
    systick::set_current_value(0);
    assert_eq!(systick::current_value(), 0);

    // println!("# Clear Count Flag");
    let _ = systick::count_flag();
    assert!(!systick::count_flag());


    let mut value_min = systick::current_value();

    // println!("# Start Test");
    systick::set_enabled(true);
    assert!(systick::current_value() > 0);

    while !systick::count_flag() {
        let v = systick::current_value();
        if v < value_min {
            value_min = v;
        }
    }
    assert!(value_min < reload_value);
    systick::set_enabled(false);

    println!("[pass] SYSTICK OK");
}

fn test_dma() {
    // use board::hal::dmac;
    use board::hal::dmac::*;
    use core::mem;
    

    let mut src = [0u8; 32];
    let mut dst = [0u8; 32];

    for (i, s) in src.iter_mut().enumerate() {
        *s = i as u8;
    }

    let dma = DMAC;
    let ch = DMAC_CH0;    

    dma.pm_set_enabled(true);
    dma.set_ctrl(|r| r.set_swrst(1));

    let mut buf = [0u8; 1024];
    let ptr = unsafe {
        let mut ptr = buf.as_mut_ptr();
        while ptr as u32 % 16 != 0 {        
            ptr = ptr.offset(1);
        }
        ptr      
    };

    let desc: &mut Transfer = unsafe { mem::transmute(ptr) };
    let wb: &mut Transfer = unsafe { mem::transmute(ptr.offset(16))};
    let desc_addr = desc as *mut Transfer as u32;
    let wb_addr = wb as *mut Transfer as u32;
    
    {
        // SRCADDR / DSTADDR must be the ending values after a transfer
        unsafe {
            desc.set_srcaddr(|_| Srcaddr(src.as_ptr().offset(src.len() as isize) as u32));
            desc.set_dstaddr(|_| Dstaddr(dst.as_mut_ptr().offset(dst.len() as isize) as u32));
        }
        desc.set_btcnt(|_| Btcnt(src.len() as u16));
        desc.with_btctrl(|r| r.set_dstinc(1).set_srcinc(1).set_valid(1));
    }    


    // Set Descriptor Base
    dma.set_baseaddr(|_| Baseaddr(desc_addr));
    // Set Writeback Base
    dma.set_wrbaddr(|_| Wrbaddr(wb_addr));

    // Set Priority Level 0 Enabled
    dma.with_ctrl(|r| r.set_lvlen(0, 1));

    // Enable DMAC
    dma.with_ctrl(|r| r.set_dmaenable(1));

    // Set Channel ID
    dma.set_chid(|r| r.set_id(ch.index() as u8));
    dma.set_chctrla(|r| r.set_swrst(1));
    dma.set_chid(|r| r.set_id(ch.index() as u8));
    dma.set_chctrlb(|r| r.set_trigact(0x3));

    // Set Channel Enabled
    dma.set_chid(|r| r.set_id(ch.index() as u8));
    dma.set_chctrla(|r| r.set_enable(1));

    dma.set_swtrigctrl(|r| r.set_swtrig(ch.index(), 1));

    loop {
        let f = dma.chintflag();
        if f.terr() != 0 {
            println!("[fail] Transfer Error");
            break;
        }
        if f.tcmpl() != 0 {
            break;
        }
        if f.susp() != 0 {
            println!("[fail] Transfer Suspended");
            break;
        }
    }    

    assert_eq!(&src[..], &dst[..]);
    println!("[pass] DMA OK");    
}