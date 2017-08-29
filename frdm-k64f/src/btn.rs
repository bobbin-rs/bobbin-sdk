use hal::port::*;
use hal::gpio::*;

pub use common::digital::DigitalInput;

pub const BTN0: Pc6 = PC6;
pub const BTN0_PT: Ptc6 = PTC6;

pub const BTN1: Pa4 = PA4;
pub const BTN1_PT: Pta4 = PTA4;

pub fn init() {
    BTN0_PT.port().sim_enable();
    BTN0_PT.set_mux_gpio();
    BTN0.set_dir_input();

    BTN1_PT.port().sim_enable();
    BTN1_PT.set_mux_gpio();
    BTN1.set_dir_input();
}
