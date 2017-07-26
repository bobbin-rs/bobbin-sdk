#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate arduino_zero as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Clock Test");
    let clk = board::clock::CLK;
    println!("XOSC:        {:?}", clk.xosc());
    println!("XOSC32K:     {:?}", clk.xosc32k());
    println!("OSC32K:      {:?}", clk.osc32k());
    println!("OSCULP32K:   {:?}", clk.osculp32k());
    println!("OSC8M:       {:?} EN: {} RDY: {} PRES: {:?}", clk.osc8m(), clk.osc8m_enabled(), clk.osc8m_rdy(), clk.osc8m_pre());
    println!("DFLL:        {:?} EN: {} MUL: {}", clk.dfll(), clk.dfll_enabled(), clk.dfll_mul());
    println!("DPLL:        {:?} EN: {} MUL: {} DIV: {}", clk.dpll(), clk.dpll_enabled(), clk.dpll_mul(), clk.dpll_div());
    loop {}
}
