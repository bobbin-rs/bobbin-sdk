use chip::gpio::{GPIOA, GPIOB, GPIOC};
use hal::rcc;
use driver::gpio;

macro_rules! pindef {
    ($id:ident, $port:expr, $pin:expr) => {
        pub fn $id() -> gpio::PinUnknown { 
            rcc::set_gpio_enabled($port, true);
            gpio::pin($port, $pin)
        }
    }
}

pindef!(d0, GPIOA, 3);
pindef!(d1, GPIOA, 2);
pindef!(d2, GPIOA, 10);
pindef!(d3, GPIOB, 3);
pindef!(d4, GPIOB, 5);
pindef!(d5, GPIOB, 4);
pindef!(d6, GPIOB, 10);
pindef!(d7, GPIOA, 8);
pindef!(d8, GPIOA, 9);
pindef!(d9, GPIOC, 7);
pindef!(d10, GPIOB, 6);
pindef!(d11, GPIOA, 7);
pindef!(d12, GPIOA, 6);
pindef!(d13, GPIOA, 5);
pindef!(d14, GPIOB, 9);
pindef!(d15, GPIOB, 8);

pindef!(a0, GPIOA, 0);
pindef!(a1, GPIOA, 1);
pindef!(a2, GPIOA, 4);
pindef!(a3, GPIOB, 0);
pindef!(a4, GPIOC, 1);
pindef!(a5, GPIOC, 0);
