use hal::gpio::*;

pub use common::hal::digital::DigitalInput;

pub const BTN0: Pc3 = PC3;

pub fn init() {
    BTN0.port_pin().port().sim_enable();
    BTN0.port_pin().set_mux_gpio();
    BTN0.set_dir_input();
}