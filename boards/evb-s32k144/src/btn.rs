use hal::port::*;
use hal::gpio::*;

pub use common::digital::DigitalInput;
pub use hal::gpio::LinkPortPin;

pub const BTN0: Pc12 = PC12;
pub const BTN1: Pc13 = PC13;

pub fn init() {
    BTN0.port_pin().port().pcc_enable();    
    BTN0.port_pin().set_mux_gpio();
    BTN0.set_dir_input();

    BTN1.port_pin().port().pcc_enable();    
    BTN1.port_pin().set_mux_gpio();
    BTN1.set_dir_input();
}