use hal::port::*;
use hal::gpio::{GpioExt};

pub const BTN0: Ptc12 = PTC12;
pub const BTN1: Ptc13 = PTC13;

pub fn init() {
    BTN0.port.pcc_enable();
    BTN0.set_mux_gpio();
    BTN0.gpio_pin().set_dir_input();

    BTN1.port.pcc_enable();
    BTN1.set_mux_gpio();
    BTN1.gpio_pin().set_dir_input();
}