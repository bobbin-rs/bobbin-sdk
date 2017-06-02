use hal::port::*;

pub const LED0: Pa17 = PA17;

pub fn init() {
    LED0.set_mode_output();
}

// pub type Led = port::PinOutput;



// // LED @ D13 = PA17
// pub fn led0() -> Led {
//     pin::pa17().into_digital_output()
// }