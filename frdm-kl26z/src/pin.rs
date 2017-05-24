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

pindef!(d0, PTA1);
pindef!(d1, PTA2);
pindef!(d2, PTD4);
pindef!(d3, PTA12);
pindef!(d4, PTA4);
pindef!(d5, PTA5);
pindef!(d6, PTC8);
pindef!(d7, PTC9);
pindef!(d8, PTA13);
pindef!(d9, PTD5);
pindef!(d10, PTD0);
pindef!(d11, PTD2);
pindef!(d12, PTD3);
pindef!(d13, PTD1);

pindef!(a0, PTB0);
pindef!(a1, PTB1);
pindef!(a2, PTB2);
pindef!(a3, PTB3);
pindef!(a4, PTC2);
pindef!(a5, PTC1);

pindef!(pa0, PTA0);
pindef!(pa1, PTA1);
pindef!(pa2, PTA2);
pindef!(pa3, PTA3);
pindef!(pa4, PTA4);
pindef!(pa5, PTA5);
pindef!(pa12, PTA12);
pindef!(pa13, PTA13);
pindef!(pa18, PTA18);
pindef!(pa19, PTA19);
pindef!(pa20, PTA20);

pindef!(pb0, PTB0);
pindef!(pb1, PTB1);
pindef!(pb2, PTB2);
pindef!(pb3, PTB3);
pindef!(pb16, PTB16);
pindef!(pb17, PTB17);
pindef!(pb18, PTB18);
pindef!(pb19, PTB19);

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

pindef!(pd0, PTD0);
pindef!(pd1, PTD1);
pindef!(pd2, PTD2);
pindef!(pd3, PTD3);
pindef!(pd4, PTD4);
pindef!(pd5, PTD5);
pindef!(pd6, PTD6);
pindef!(pd7, PTD7);

pindef!(pe0, PTE0);
pindef!(pe1, PTE1);
pindef!(pe20, PTE20);
pindef!(pe21, PTE21);
pindef!(pe22, PTE22);
pindef!(pe23, PTE23);
pindef!(pe24, PTE24);
pindef!(pe25, PTE25);
pindef!(pe29, PTE29);
pindef!(pe30, PTE30);
pindef!(pe31, PTE31);