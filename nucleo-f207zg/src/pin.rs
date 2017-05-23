use hal::gpio;
use chip::gpio::*;

macro_rules! pindef {
    ($id:ident, $pin:expr) => {
        pub fn $id() -> gpio::PinUnknown { 
            gpio::pin($pin)
        }
    }
}

pindef!(d0, PA3);
pindef!(d1, PA2);
pindef!(d2, PA10);
pindef!(d3, PB3);
pindef!(d4, PB5);
pindef!(d5, PB4);
pindef!(d6, PB10);
pindef!(d7, PA8);
pindef!(d8, PA9);
pindef!(d9, PC7);
pindef!(d10, PB6);
pindef!(d11, PA7);
pindef!(d12, PA6);
pindef!(d13, PA5);
pindef!(d14, PB9);
pindef!(d15, PB8);

pindef!(a0, PA0);
pindef!(a1, PA1);
pindef!(a2, PA4);
pindef!(a3, PB0);
pindef!(a4, PC1);
pindef!(a5, PC0);