use chip::gpio::{GPIOA, GPIOB, GPIOC, GPIOD, GPIOE, GPIOF, GPIOG};
use hal::rcc;
use hal::gpio;

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
pindef!(pc1, GPIOC, 1);
pindef!(pc2, GPIOC, 2);
pindef!(pc3, GPIOC, 3);
pindef!(pc4, GPIOC, 4);
pindef!(pc5, GPIOC, 5);
pindef!(pc6, GPIOC, 6);
pindef!(pc7, GPIOC, 7);
pindef!(pc8, GPIOC, 8);
pindef!(pc9, GPIOC, 9);
pindef!(pc10, GPIOC, 10);
pindef!(pc11, GPIOC, 11);
pindef!(pc12, GPIOC, 12);
pindef!(pc13, GPIOC, 13);
pindef!(pc14, GPIOC, 14);
pindef!(pc15, GPIOC, 15);

pindef!(pd0, GPIOD, 0);
pindef!(pd1, GPIOD, 1);
pindef!(pd2, GPIOD, 2);
pindef!(pd3, GPIOD, 3);
pindef!(pd4, GPIOD, 4);
pindef!(pd5, GPIOD, 5);
pindef!(pd6, GPIOD, 6);
pindef!(pd7, GPIOD, 7);
pindef!(pd8, GPIOD, 8);
pindef!(pd9, GPIOD, 9);
pindef!(pd10, GPIOD, 10);
pindef!(pd11, GPIOD, 11);
pindef!(pd12, GPIOD, 12);
pindef!(pd13, GPIOD, 13);
pindef!(pd14, GPIOD, 14);
pindef!(pd15, GPIOD, 15);

pindef!(pe0, GPIOE, 0);
pindef!(pe1, GPIOE, 1);
pindef!(pe2, GPIOE, 2);
pindef!(pe3, GPIOE, 3);
pindef!(pe4, GPIOE, 4);
pindef!(pe5, GPIOE, 5);
pindef!(pe6, GPIOE, 6);
pindef!(pe7, GPIOE, 7);
pindef!(pe8, GPIOE, 8);
pindef!(pe9, GPIOE, 9);
pindef!(pe10, GPIOE, 10);
pindef!(pe11, GPIOE, 11);
pindef!(pe12, GPIOE, 12);
pindef!(pe13, GPIOE, 13);
pindef!(pe14, GPIOE, 14);
pindef!(pe15, GPIOE, 15);

pindef!(pf0, GPIOF, 0);
pindef!(pf1, GPIOF, 1);
pindef!(pf2, GPIOF, 2);
pindef!(pf3, GPIOF, 3);
pindef!(pf4, GPIOF, 4);
pindef!(pf5, GPIOF, 5);
pindef!(pf6, GPIOF, 6);
pindef!(pf7, GPIOF, 7);
pindef!(pf8, GPIOF, 8);
pindef!(pf9, GPIOF, 9);
pindef!(pf10, GPIOF, 10);
pindef!(pf11, GPIOF, 11);
pindef!(pf12, GPIOF, 12);
pindef!(pf13, GPIOF, 13);
pindef!(pf14, GPIOF, 14);
pindef!(pf15, GPIOF, 15);

pindef!(pg0, GPIOG, 0);
pindef!(pg1, GPIOG, 1);
pindef!(pg2, GPIOG, 2);
pindef!(pg3, GPIOG, 3);
pindef!(pg4, GPIOG, 4);
pindef!(pg5, GPIOG, 5);
pindef!(pg6, GPIOG, 6);
pindef!(pg7, GPIOG, 7);
pindef!(pg8, GPIOG, 8);
pindef!(pg9, GPIOG, 9);
pindef!(pg10, GPIOG, 10);
pindef!(pg11, GPIOG, 11);
pindef!(pg12, GPIOG, 12);
pindef!(pg13, GPIOG, 13);
pindef!(pg14, GPIOG, 14);
pindef!(pg15, GPIOG, 15);