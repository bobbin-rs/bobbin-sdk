use hal::port::*;
use hal::gpio::GpioExt;

pub const LED0: Ptb22 = PTB22; // Red
pub const LED1: Ptb21 = PTB21; // Blue
pub const LED2: Pte26 = PTE26; // Green

pub fn init() {
    LED0.port().sim_enable();
    LED0.gpio_pin().set_dir_output().set_output(true);
    LED0.set_mux_gpio();

    LED1.port().sim_enable();
    LED1.gpio_pin().set_dir_output().set_output(true);
    LED1.set_mux_gpio();

    LED2.port().sim_enable();
    LED2.gpio_pin().set_dir_output().set_output(true);
    LED2.set_mux_gpio();
}




// use chip::port::PORTB;
// use hal::sim;
// use hal::port;

// // LED RED = PB22

// pub fn led0() -> port::PinOutput {
//     sim::set_port_enabled(PORTB, true);
//     port::pin(PORTB, 22).into_output()
// }