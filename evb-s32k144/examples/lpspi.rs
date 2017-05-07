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

    loop {
        // dump_register(&t, 0x7e);
        for i in 0u8..128 {
            dump_register(&t, i);
        }
        // dump_register(&t, 0x0500);
        // dump_register(&t, 0x7e00);
        board::delay(1000);
    }

    fn dump_register(t: &lpspi::Target, value: u8) {
        let v: u16 = ((value as u16) << 9) | (1 << 8);        
        t.send(v);
        let result = t.recv();
        let r = result as u8;
        println!("{:02x}: {:02x}", value, r);
    }
}