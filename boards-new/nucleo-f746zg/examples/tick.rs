#![no_std]
#![no_main]

extern crate nucleo_f746zg as board;
extern crate examples;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();    
    let brd = board::board();
    let mut app = examples::tick::Tick::new(brd.console(), brd, 500);
    app.run()
}
