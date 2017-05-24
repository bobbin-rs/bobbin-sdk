use chip::port::*;
use hal::sim;
use hal::port;

macro_rules! pindef {
    ($id:ident, $pin:expr) => {
        pub fn $id() -> port::PinUnknown { 
            sim::set_port_enabled($pin.port(), true);
            port::pin($pin.port(), $pin.index())
        }
    }
}

pindef!(d0, PTA3);
pindef!(d1, PTA2);
pindef!(d2, PTA10);
pindef!(d3, PTB3);
pindef!(d4, PTB5);
pindef!(d5, PTB4);
pindef!(d6, PTB10);
pindef!(d7, PTA8);
pindef!(d8, PTA9);
pindef!(d9, PTC7);
pindef!(d10, PTB6);
pindef!(d11, PTA7);
pindef!(d12, PTA6);
pindef!(d13, PTA5);
pindef!(d14, PTB9);
pindef!(d15, PTB8);

pindef!(a0, PTA0);
pindef!(a1, PTA1);
pindef!(a2, PTA4);
pindef!(a3, PTB0);
pindef!(a4, PTC1);
pindef!(a5, PTC0);

pindef!(pa0, PTA0);
pindef!(pa1, PTA1);
pindef!(pa2, PTA2);
pindef!(pa3, PTA3);
pindef!(pa4, PTA4);
pindef!(pa5, PTA5);
pindef!(pa6, PTA6);
pindef!(pa7, PTA7);
pindef!(pa8, PTA8);
pindef!(pa9, PTA9);
pindef!(pa10, PTA10);
pindef!(pa11, PTA11);
pindef!(pa12, PTA12);
pindef!(pa13, PTA13);
pindef!(pa14, PTA14);
pindef!(pa15, PTA15);
pindef!(pa16, PTA16);
pindef!(pa17, PTA17);
pindef!(pa18, PTA18);
pindef!(pa19, PTA19);
pindef!(pa24, PTA24);
pindef!(pa25, PTA25);
pindef!(pa26, PTA26);
pindef!(pa27, PTA27);
pindef!(pa28, PTA28);
pindef!(pa29, PTA29);


pindef!(pb0, PTB0);
pindef!(pb1, PTB1);
pindef!(pb2, PTB2);
pindef!(pb3, PTB3);
pindef!(pb4, PTB4);
pindef!(pb5, PTB5);
pindef!(pb6, PTB6);
pindef!(pb7, PTB7);
pindef!(pb8, PTB8);
pindef!(pb9, PTB9);
pindef!(pb10, PTB10);
pindef!(pb11, PTB11);
pindef!(pb12, PTB12);
pindef!(pb13, PTB13);
pindef!(pb16, PTB16);
pindef!(pb17, PTB17);
pindef!(pb18, PTB18);
pindef!(pb19, PTB19);
pindef!(pb20, PTB20);
pindef!(pb21, PTB21);
pindef!(pb22, PTB22);
pindef!(pb23, PTB23);

pindef!(pc0, PTC0);
pindef!(pc1, PTC1);
pindef!(pc2, PTC2);
pindef!(pc3, PTC3);
pindef!(pc4, PTC4);
pindef!(pc5, PTC5);
pindef!(pc6, PTC6);
pindef!(pc7, PTC7);
pindef!(pc8, PTC8);
pindef!(pc9, PTC9);
pindef!(pc10, PTC10);
pindef!(pc11, PTC11);
pindef!(pc12, PTC12);
pindef!(pc13, PTC13);
pindef!(pc14, PTC14);
pindef!(pc15, PTC15);
pindef!(pc16, PTC16);
pindef!(pc17, PTC17);
pindef!(pc18, PTC18);
pindef!(pc19, PTC19);

pindef!(pd0, PTD0);
pindef!(pd1, PTD1);
pindef!(pd2, PTD2);
pindef!(pd3, PTD3);
pindef!(pd4, PTD4);
pindef!(pd5, PTD5);
pindef!(pd6, PTD6);
pindef!(pd7, PTD7);
pindef!(pd8, PTD8);
pindef!(pd9, PTD9);
pindef!(pd10, PTD10);
pindef!(pd11, PTD11);
pindef!(pd12, PTD12);
pindef!(pd13, PTD13);
pindef!(pd14, PTD14);
pindef!(pd15, PTD15);


pindef!(pe0, PTE0);
pindef!(pe1, PTE1);
pindef!(pe2, PTE2);
pindef!(pe3, PTE3);
pindef!(pe4, PTE4);
pindef!(pe5, PTE5);
pindef!(pe6, PTE6);
pindef!(pe7, PTE7);
pindef!(pe8, PTE8);
pindef!(pe9, PTE9);
pindef!(pe10, PTE10);
pindef!(pe11, PTE11);
pindef!(pe12, PTE12);
pindef!(pe24, PTE24);
pindef!(pe25, PTE25);
pindef!(pe26, PTE26);
pindef!(pe27, PTE27);
pindef!(pe28, PTE28);
