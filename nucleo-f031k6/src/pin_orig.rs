use chip::gpio::{GPIOA, GPIOB, GPIOC, GPIOH};
use hal::rcc;
use hal::gpio;

macro_rules! pindef {
    ($id:ident, $port:expr, $pin:expr) => {
        pub fn $id() -> gpio::PinUnknown { 
            rcc::set_gpio_enabled($port, true);
            gpio::pin(($port, $pin))
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

pindef!(pa0, GPIOA, 0);
pindef!(pa1, GPIOA, 1);
pindef!(pa2, GPIOA, 2);
pindef!(pa3, GPIOA, 3);
pindef!(pa4, GPIOA, 4);
pindef!(pa5, GPIOA, 5);
pindef!(pa6, GPIOA, 6);
pindef!(pa7, GPIOA, 7);
pindef!(pa8, GPIOA, 8);
pindef!(pa9, GPIOA, 9);
pindef!(pa10, GPIOA, 10);
pindef!(pa11, GPIOA, 11);
pindef!(pa12, GPIOA, 12);
pindef!(pa13, GPIOA, 13);
pindef!(pa14, GPIOA, 14);
pindef!(pa15, GPIOA, 15);

pindef!(pb0, GPIOB, 0);
pindef!(pb1, GPIOB, 1);
pindef!(pb2, GPIOB, 2);
pindef!(pb3, GPIOB, 3);
pindef!(pb4, GPIOB, 4);
pindef!(pb5, GPIOB, 5);
pindef!(pb6, GPIOB, 6);
pindef!(pb7, GPIOB, 7);
pindef!(pb8, GPIOB, 8);
pindef!(pb9, GPIOB, 9);
pindef!(pb10, GPIOB, 10);
pindef!(pb11, GPIOB, 11);
pindef!(pb12, GPIOB, 12);
pindef!(pb13, GPIOB, 13);
pindef!(pb14, GPIOB, 14);
pindef!(pb15, GPIOB, 15);

pindef!(pc0, GPIOC, 0);
pindef!(ph0, GPIOH, 0);
pindef!(ph1, GPIOH, 1);