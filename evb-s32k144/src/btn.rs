use hal::port::*;
use hal::gpio::*;

pub use common::digital::DigitalInput;

pub const BTN0: Pc12 = PC12;
pub const BTN1: Pc13 = PC13;

pub fn init() {
    PORTC.pcc_enable();
    PTC12.set_mux_gpio();
    PTC13.set_mux_gpio();

    BTN0.set_dir_input();
    BTN1.set_dir_input();
}