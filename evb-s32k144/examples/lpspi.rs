#![no_std]
#![no_main]

#[macro_use]
extern crate evb_s32k144 as board;

use board::hal::lpspi;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("LPSPI Test");

    let l1 = board::spi::lpspi1();
    l1.set_enabled(false);    
    
    l1.configure(lpspi::Config::default()
        .master(true)
        .sckpcs(4)
        .pcssck(9)
        .dbt(8)
        .sckdiv(8)
        .txwater(3)        
    );
    l1.set_enabled(true);
    let t = l1.target()
        .cpha(true)
        .prescale(2)
        .pcs(3)
        .framesz(15);
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

    let u = board::uja1169::device(t);
    for i in 0..128 {
        println!("{:02x}: {:02x}", i, u.read_register(i));
    }
    loop {
        let r = u.reg();
        println!("ids:   {:08x}", r.ids().0);
        println!("ms:    {:08x}", r.ms().0);
        println!("wds:   {:08x}", r.wds().0);
        println!("sc:    {:08x}", r.sc().0);
        println!("sbccc: {:08x}", r.sbccc().0);
        println!("");
        board::delay(1000);
    }
}