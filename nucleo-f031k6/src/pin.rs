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

pindef!(d0, GPIOA, 10);
pindef!(d1, GPIOA, 9);
pindef!(d2, GPIOA, 12);
pindef!(d3, GPIOB, 0);
pindef!(d4, GPIOB, 7);
pindef!(d5, GPIOB, 6);
pindef!(d6, GPIOB, 1);
pindef!(d7, GPIOC, 14);
pindef!(d8, GPIOC, 15);
pindef!(d9, GPIOA, 8);
pindef!(d10, GPIOA, 11);
pindef!(d11, GPIOB, 5);
pindef!(d12, GPIOB, 4);

pindef!(a0, GPIOA, 0);
pindef!(a1, GPIOA, 1);
pindef!(a2, GPIOA, 3);
pindef!(a3, GPIOA, 4);
pindef!(a4, GPIOC, 5);
pindef!(a5, GPIOC, 6);
pindef!(a6, GPIOC, 7);
pindef!(a7, GPIOC, 2);
