#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f429zi as board;

use board::prelude::*;
use board::mcu::tim_gen::*;
use board::mcu::pin::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init().run(|_| {
        println!("Ownership / Refcount Example");
        
        println!("Tim3::owned():     {}", Tim3::owned_mut());
        println!("Tim3::ref_count(): {}", Tim3::ref_count());        
        {

            println!("  -- acquire Tim3");
            let _tim = Tim3::acquire().unwrap();
            {
                println!("  -- acquire Tim3Ch1");
                let _ch1 = Tim3Ch1::acquire().unwrap();
                println!("Tim3::owned():     {}", Tim3::owned_mut());
                println!("Tim3::ref_count(): {}", Tim3::ref_count());

                println!("  -- acquire Tim3Ch2");
                let _ch2 = Tim3Ch2::acquire().unwrap();
                println!("Tim3::owned():     {}", Tim3::owned_mut());
                println!("Tim3::ref_count(): {}", Tim3::ref_count());

                println!("  -- drop Tim3Ch1 and Tim3Ch2");
            }
            println!("Tim3::owned():     {}", Tim3::owned_mut());
            println!("Tim3::ref_count(): {}", Tim3::ref_count());
            println!("  -- drop Tim3");
        }
        println!("Tim3::owned():     {}", Tim3::owned_mut());
        println!("Tim3::ref_count(): {}", Tim3::ref_count());
        loop {}
    })
}
