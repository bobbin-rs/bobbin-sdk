#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_k64f as board;

use board::chip::mcg::MCG;
// use board::chip::sim::SIM;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Clock Test");
    let clk = board::clock::CLK;

    // println!("MCGFLLCLK: {:?}", clk.mcgffclk());
    // println!("MCGPLLCLK: {:?}", clk.mcgffclk());

    

    println!("LPO: {:?}", clk.lpo());

    println!("OSCCLK:   {:?}", clk.oscclk());
    println!("OSCERCLK:   {:?}", clk.oscerclk());
    println!("OSC32KCLK:   {:?}", clk.osc32kclk());
    println!("IRC48MCLK: {:?}", clk.irc48mclk());
        
    // MCG.with_c1(|r| r.set_irefs(1));

    println!("OSCSELCLK:   {:?}", clk.oscselclk());
    println!("OSCERCLK:  {:?}", clk.oscerclk());
    // println!("ERCLK32K:  {:?}", clk.erclk32k());
    println!("MCGIRCLK:  {:?}", clk.mcgirclk());
    println!("MCGFFCLK:  {:?}", clk.mcgffclk());
    println!("MCGFLLCLK: {:?}", clk.mcgfllclk());
    println!("MCGPLLCLK: {:?}", clk.mcgpllclk());
    println!("MCGOUTCLK: {:?}", clk.mcgoutclk());
    println!("");
    println!("CORE / System: {:?}", clk.core());
    println!("BUS:           {:?}", clk.bus());
    println!("FlexBus:       {:?}", clk.flexbus());
    println!("Flash:         {:?}", clk.flash());
    
    // println!("C1: {:?}", MCG.c1());
    // println!("C2: {:?}", MCG.c2());
    // println!("C3: {:?}", MCG.c3());
    // println!("C4: {:?}", MCG.c4());
    // println!("C5: {:?}", MCG.c5());
    // println!("C6: {:?}", MCG.c6());
    // println!("S: {:?}", MCG.s());
    // println!("SC: {:?}", MCG.sc());
    // println!("C7: {:?}", MCG.c7());
    // println!("C8: {:?}", MCG.c8());

    println!("Change Bus Divider");
    // println!("CLKDIV1: {:?}", SIM.clkdiv1());
    board::delay(100);
    board::console::disable();
    MCG.with_c1(|r| r.set_clks(0b10));
    board::console::enable();
    // println!("CLKDIV1: {:?}", SIM.clkdiv1());
    println!("CORE:           {:?}", clk.core());
    println!("BUS:           {:?}", clk.bus());

    loop {
        board::delay(1000);
        println!("tick");
    }
}
