use hal::gpio::*;

pub use common::Pin;
pub use common::hal::digital::DigitalOutput;
pub use hal::gpio::LinkPortPin;

pub const LED0: Pd0 = PD0;
pub const LED1: Pd15 = PD15;
pub const LED2: Pd16 = PD16;

pub fn init() {
    LED0.port_pin().port().pcc_set_enabled(true);
    LED0.port_pin().set_mux_gpio();
    LED0.set_dir_output().set_output(true);
    
    LED1.port_pin().port().pcc_set_enabled(true);
    LED1.port_pin().set_mux_gpio();
    LED1.set_dir_output().set_output(true);

    LED2.port_pin().port().pcc_set_enabled(true);
    LED2.port_pin().set_mux_gpio();
    LED2.set_dir_output().set_output(true);
}