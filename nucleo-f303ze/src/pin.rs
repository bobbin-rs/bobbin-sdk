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

// pub fn d0() -> gpio::PinUnknown { gpio::pin(PA3) }
// pub fn d1() -> gpio::PinUnknown { gpio::pin(PA2) }
// pub fn d2() -> gpio::PinUnknown { gpio::pin(PA10) }
// pub fn d3() -> gpio::PinUnknown { gpio::pin(pb3() }
// pub fn d4() -> gpio::PinUnknown { gpio::pin(pb5() }
// pub fn d5() -> gpio::PinUnknown { gpio::pin(pb4() }
// pub fn d6() -> gpio::PinUnknown { gpio::pin(pb10() }
// pub fn d7() -> gpio::PinUnknown { gpio::pin(pa8() }
// pub fn d8() -> gpio::PinUnknown { gpio::pin(pa9() }
// pub fn d9() -> gpio::PinUnknown { gpio::pin(pc7() }
// pub fn d10() -> gpio::PinUnknown { gpio::pin(pb6() }
// pub fn d11() -> gpio::PinUnknown { gpio::pin(pa7() }
// pub fn d12() -> gpio::PinUnknown { gpio::pin(pa6() }
// pub fn d13() -> gpio::PinUnknown { gpio::pin(pa5() }
// pub fn d14() -> gpio::PinUnknown { gpio::pin(pb9() }
// pub fn d15() -> gpio::PinUnknown { gpio::pin(pb8() }

// pub fn a0() -> gpio::PinUnknown { gpio::pin(pa0() }
// pub fn a1() -> gpio::PinUnknown { gpio::pin(pa1() }
// pub fn a2() -> gpio::PinUnknown { gpio::pin(pa4() }
// pub fn a3() -> gpio::PinUnknown { gpio::pin(pb0() }
// pub fn a4() -> gpio::PinUnknown { pc1() }
// pub fn a5() -> gpio::PinUnknown { pc0() }


