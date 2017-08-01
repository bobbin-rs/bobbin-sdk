#![no_std]
#![no_main]

#[macro_use]
extern crate ek_tm4c1294xl as board;
use board::clock::CLK;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    
    println!("Running Clock Test");    
    println!("{:?}", CLK);
    println!("");

    loop {
        println!("Changing Clock to 60MHz");
        board::delay(50);
        board::clock::CLK_60MHZ.configure();
        board::delay(50);
        board::console::reinit();
        println!("Clock Changed");

        board::delay(1000);

        println!("Changing Clock to 16MHz");
        board::delay(50);
        board::clock::CLK_16MHZ.configure();
        board::delay(50);
        board::console::reinit();
        println!("Clock Changed");

        board::delay(1000);
        println!("Changing Clock to 120MHz");
        board::delay(50);
        board::clock::CLK_120MHZ.configure();
        board::delay(50);
        board::console::reinit();
        println!("Clock Changed");
        board::delay(1000);
    }
}
