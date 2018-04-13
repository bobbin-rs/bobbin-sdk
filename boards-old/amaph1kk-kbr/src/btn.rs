pub use common::digital::DigitalInput;
use hal::gpio::*;

// BTN2 = GP16
// BTN3 = GP18
// BTN4 = GP19

pub const BTN2: Gp16 = GP16;
pub const BTN3: Gp18 = GP18;
pub const BTN4: Gp19 = GP19;

pub fn init() {  
    GPIO.unlock();  
    BTN2
        .set_pad_gpio()
        .set_pad_input_enabled()
        .set_gpio_input_enabled();

    BTN3
        .set_pad_gpio()
        .set_pad_input_enabled()
        .set_gpio_input_enabled();

    BTN4
        .set_pad_gpio()
        .set_pad_input_enabled()
        .set_gpio_input_enabled();

}
