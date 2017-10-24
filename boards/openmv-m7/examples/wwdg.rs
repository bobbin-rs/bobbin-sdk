#![no_std]
#![no_main]

#[macro_use]
extern crate openmv_m7 as board;

use board::hal::rcc::RCC;
use board::hal::wwdg::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running WWDG Test");
    println!("RCC_CSR: {:?}", RCC.csr());
    println!("Clearing Reset Flags");
    RCC.with_csr(|r| r.set_rmvf(1));
        
    board::delay(1000);

    // APB1 = 32MHz
    // APB1 / 4096 = 7812
    // APB1 / 4096 / 8 = 976hz

    let h = EwiHandler {};

    WWDG.rcc_set_enabled(true);

    let _h = WWDG.register_wwdg_handler(&h);

    WWDG.irq_wwdg().set_enabled(true);

    WWDG.configure(Config {
        early_wake_interrupt: true,
        time_base: TimeBase::Div8,
        window: T_MAX,
    });
    WWDG.activate(T_MAX);
    let mut i = 0;
    loop {
        board::delay(40);
        if i < 10 {
            println!("{} {:02x}", i, WWDG.cr().t());
            //println!("refresh {} {:?} {:?}", i, WWDG.cr(), WWDG.cfr());
            WWDG.refresh(T_MAX);
            i += 1;
        }        
    }
}

pub struct EwiHandler {}

impl HandleWwdg for EwiHandler {
    fn handle_wwdg(&self) {
        println!("EWI!");
        if WWDG.sr().ewif() != 0 {
            WWDG.set_sr(Sr(0).set_ewif(0));
        }        
    }
}