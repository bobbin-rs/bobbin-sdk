#![no_std]
#![no_main]

#[macro_use]
extern crate evb_s32k144 as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("LPSPI Test");

    let l1 = board::spi::lpspi1();

    let mut cfg = board::hal::lpspi::Config::default();
    cfg.master = true;

    cfg.sckpcs = 4;
    cfg.pcssck = 9;
    cfg.dbt = 8;
    cfg.sckdiv = 8;

    cfg.txwater = 3;

    l1.set_enabled(false);
    l1.configure(&cfg);
    let mut tgt = board::hal::lpspi::Target::default();
    tgt.cpha = true;
    tgt.prescale = 2;
    tgt.pcs = 3;
    tgt.framesz = 15;

    l1.configure_target(&tgt);
    l1.set_enabled(true);
    unsafe {
        let s = l1.lpspi;
        println!("CR:     {:?}", s.cr());
        println!("SR:     {:?}", s.sr());
        println!("CFGR0:  {:?}", s.cfgr0());
        println!("CFGR1:  {:?}", s.cfgr1());
        println!("CCR:    {:?}", s.ccr());
        println!("FCR:    {:?}", s.fcr());
        println!("FSR:    {:?}", s.fsr());
        println!("TCR:    {:?}", s.tcr());
        println!("RSR:    {:?}", s.rsr());
    }
    

    loop {
        println!("Sending...");
        l1.send(0xFD00);
        println!("Receiving...");
        println!("{:08x}", l1.recv());
        board::delay(1000);
    }
}