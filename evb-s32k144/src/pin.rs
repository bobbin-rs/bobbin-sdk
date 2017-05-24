use chip::port::*;
use hal::pcc;
use hal::port;

macro_rules! pindef {
    ($id:ident, $pin:expr) => {
        pub fn $id() -> port::PinUnknown { 
            pcc::set_port_enabled($pin.port(), true);
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

pindef!(ptta0, PTA0);
pindef!(ptta1, PTA1);
pindef!(pta2, PTA2);
pindef!(pta3, PTA3);
pindef!(pta4, PTA4);
pindef!(pta5, PTA5);
pindef!(pta6, PTA6);
pindef!(pta7, PTA7);
pindef!(pta8, PTA8);
pindef!(pta9, PTA9);
pindef!(pta10, PTA10);
pindef!(pta11, PTA11);
pindef!(pta12, PTA12);
pindef!(pta13, PTA13);
pindef!(pta14, PTA14);
pindef!(pta15, PTA15);
pindef!(pta16, PTA16);
pindef!(pta17, PTA17);


pindef!(ptb0, PTB0);
pindef!(ptb1, PTB1);
pindef!(ptb2, PTB2);
pindef!(ptb3, PTB3);
pindef!(ptb4, PTB4);
pindef!(ptb5, PTB5);
pindef!(ptb6, PTB6);
pindef!(ptb7, PTB7);
pindef!(ptb8, PTB8);
pindef!(ptb9, PTB9);
pindef!(ptb10, PTB10);
pindef!(ptb11, PTB11);
pindef!(ptb12, PTB12);
pindef!(ptb13, PTB13);
pindef!(ptb14, PTB14);
pindef!(ptb15, PTB15);
pindef!(ptb16, PTB16);
pindef!(ptb17, PTB17);

pindef!(ptc0, PTC0);
pindef!(ptc1, PTC1);
pindef!(ptc2, PTC2);
pindef!(ptc3, PTC3);
pindef!(ptc4, PTC4);
pindef!(ptc5, PTC5);
pindef!(ptc6, PTC6);
pindef!(ptc7, PTC7);
pindef!(ptc8, PTC8);
pindef!(ptc9, PTC9);
pindef!(ptc10, PTC10);
pindef!(ptc11, PTC11);
pindef!(ptc12, PTC12);
pindef!(ptc13, PTC13);
pindef!(ptc14, PTC14);
pindef!(ptc15, PTC15);
pindef!(ptc16, PTC16);
pindef!(ptc17, PTC17);

pindef!(ptd0, PTD0);
pindef!(ptd1, PTD1);
pindef!(ptd2, PTD2);
pindef!(ptd3, PTD3);
pindef!(ptd4, PTD4);
pindef!(ptd5, PTD5);
pindef!(ptd6, PTD6);
pindef!(ptd7, PTD7);
pindef!(ptd8, PTD8);
pindef!(ptd9, PTD9);
pindef!(ptd10, PTD10);
pindef!(ptd11, PTD11);
pindef!(ptd12, PTD12);
pindef!(ptd13, PTD13);
pindef!(ptd14, PTD14);
pindef!(ptd15, PTD15);
pindef!(ptd16, PTD16);
pindef!(ptd17, PTD17);

pindef!(pte0, PTE0);
pindef!(pte1, PTE1);
pindef!(pte2, PTE2);
pindef!(pte3, PTE3);
pindef!(pte4, PTE4);
pindef!(pte5, PTE5);
pindef!(pte6, PTE6);
pindef!(pte7, PTE7);
pindef!(pte8, PTE8);
pindef!(pte9, PTE9);
pindef!(pte10, PTE10);
pindef!(pte11, PTE11);
pindef!(pte12, PTE12);
pindef!(pte13, PTE13);
pindef!(pte14, PTE14);
pindef!(pte15, PTE15);
pindef!(pte16, PTE16);
