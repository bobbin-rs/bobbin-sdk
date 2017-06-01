use hal::port::*;
use hal::gpio::GpioExt;

pub const LED0: Ptb22 = PTB22;

pub fn init() {
    LED0.port().sim_enable();
    LED0.gpio_pin().set_dir_output();
    LED0.set_mux_gpio();
}




// use chip::port::PORTB;
// use hal::sim;
// use hal::port;

// // LED RED = PB22

// pub fn led0() -> port::PinOutput {
//     sim::set_port_enabled(PORTB, true);
//     port::pin(PORTB, 22).into_output()
// }