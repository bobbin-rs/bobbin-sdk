use hal::port::*;
use hal::gpio::GpioExt;


pub const BTN0: Ptc6 = PTC6;
pub const BTN1: Pta4 = PTA4;

pub fn init() {
    BTN0.port().sim_enable();
    BTN0.set_mux_gpio();
    BTN0.gpio_pin().set_dir_input();

    BTN1.port().sim_enable();
    BTN0.set_mux_gpio();
    BTN1.gpio_pin().set_dir_input();

}

// use chip::port::{PORTA, PORTC};
// use hal::{sim, port};

// // SW2 = PTC6
// // SW3 = PTA4

// pub fn sw2() -> port::PinInput { 
//     sim::set_port_enabled(PORTC, true);
//     port::pin(PORTC, 6).into_input().with_pull(port::Pull::PullUp)
// }

// pub fn sw3() -> port::PinInput { 
//     sim::set_port_enabled(PORTA, true);
//     port::pin(PORTA, 4).into_input().with_pull(port::Pull::PullUp)
// }