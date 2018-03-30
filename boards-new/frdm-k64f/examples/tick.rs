#![no_std]
#![no_main]

extern crate frdm_k64f as board;
extern crate embedded_hal as hal;
extern crate examples;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();    
    let brd = board::board();
    let del = DelayTimer::new();
    let mut app = examples::tick::Tick::new(brd.console(), del, 500);
    app.run()
}

pub struct DelayTimer;

impl DelayTimer {
    pub fn new() -> Self { Self {} }
}

impl hal::blocking::delay::DelayMs<u16> for DelayTimer {
    fn delay_ms(&mut self, ms: u16) {
        board::delay(ms.into())
    }
}