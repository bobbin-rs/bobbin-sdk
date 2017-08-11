pub use hal::adc::*;
pub use hal::port::*;

pub const A0: Pa02 = PA02;
pub const A1: Pb08 = PB08;
pub const A2: Pb09 = PB09;
pub const A3: Pa04 = PA04;
pub const A4: Pa05 = PA05;
pub const A5: Pb02 = PB02;

pub fn init() {
    init_adc()
}