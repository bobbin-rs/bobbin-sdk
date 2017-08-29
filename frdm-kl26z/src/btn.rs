use hal::gpio::*;
use hal::port::*;

pub use common::digital::DigitalInput;

pub const BTN0: Pc3 = PC3;
pub const BTN0_PT: Ptc3 = PTC3;

pub fn init() {
    BTN0_PT.port().sim_enable();
    BTN0_PT.set_mux_gpio();
    BTN0.set_dir_input();
}

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