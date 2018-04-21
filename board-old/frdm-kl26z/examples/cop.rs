#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_kl26z as board;

use board::chip::rcm::*;
use board::hal::sim::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running COP Test");
    println!("SRS0: {:?} SRS1: {:?}", RCM.srs0(), RCM.srs1());
    board::delay(1000);

    SIM.cop_set_timeout(CopTimeout::Long);
    println!("COPC: {:?}", SIM.copc());
    #[cfg(not(feature="allow-cop"))]
    println!("Please enable feature \"allow-cop\" to run this example.");

    for i in 0..5 {
        println!("{}", i);
        SIM.cop_reset();
        board::delay(500);
    }
    println!("Waiting for watchdog timeout...");

    loop {}
}
