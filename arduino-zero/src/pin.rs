use chip::port::{PORTA, PORTB};
use driver::port;

macro_rules! pindef {
    ($id:ident, $port:expr, $pin:expr) => {
        pub fn $id() -> port::PinUnknown { 
            port::pin($port, $pin)
        }
    }
}
pindef!(rx, PORTA, 11); // RX
pindef!(tx, PORTA, 12); // TX

pindef!(d0, PORTA, 11); // RX
pindef!(d1, PORTA, 12); // TX
pindef!(d2, PORTA, 14);
pindef!(d3, PORTA, 9);
pindef!(d4, PORTA, 8);
pindef!(d5, PORTA, 15);
pindef!(d6, PORTA, 20);
pindef!(d7, PORTA, 21);
pindef!(d8, PORTA, 6);
pindef!(d9, PORTA, 7);
pindef!(d10, PORTA, 18);
pindef!(d11, PORTA, 16);
pindef!(d12, PORTA, 19);
pindef!(d13, PORTA, 17);

pindef!(a0, PORTA, 2);
pindef!(a1, PORTB, 8);
pindef!(a2, PORTB, 9);
pindef!(a3, PORTA, 4);
pindef!(a4, PORTA, 5);
pindef!(a5, PORTB, 2);
pindef!(sda, PORTA, 22);
pindef!(scl, PORTA, 23);

pindef!(miso, PORTA, 12);
pindef!(mosi, PORTB, 10);
pindef!(sck, PORTB, 11);


pindef!(pa0, PORTA, 0);
pindef!(pa1, PORTA, 1);
pindef!(pa2, PORTA, 2);
pindef!(pa3, PORTA, 3);
pindef!(pa4, PORTA, 4);
pindef!(pa5, PORTA, 5);
pindef!(pa6, PORTA, 6);
pindef!(pa7, PORTA, 7);
pindef!(pa8, PORTA, 8);
pindef!(pa9, PORTA, 9);
pindef!(pa10, PORTA, 10);
pindef!(pa11, PORTA, 11);
pindef!(pa12, PORTA, 12);
pindef!(pa13, PORTA, 13);
pindef!(pa14, PORTA, 14);
pindef!(pa15, PORTA, 15);
pindef!(pa16, PORTA, 16);
pindef!(pa17, PORTA, 17);
pindef!(pa18, PORTA, 18);
pindef!(pa19, PORTA, 19);
pindef!(pa20, PORTA, 20);
pindef!(pa21, PORTA, 21);
pindef!(pa22, PORTA, 22);
pindef!(pa23, PORTA, 23);
pindef!(pa24, PORTA, 24);
pindef!(pa25, PORTA, 25);
pindef!(pa26, PORTA, 26);
pindef!(pa27, PORTA, 27);
pindef!(pa28, PORTA, 28);
pindef!(pa29, PORTA, 29);
pindef!(pa30, PORTA, 30);
pindef!(pa31, PORTA, 31);

pindef!(pb0, PORTB, 0);
pindef!(pb1, PORTB, 1);
pindef!(pb2, PORTB, 2);
pindef!(pb3, PORTB, 3);
pindef!(pb4, PORTB, 4);
pindef!(pb5, PORTB, 5);
pindef!(pb6, PORTB, 6);
pindef!(pb7, PORTB, 7);
pindef!(pb8, PORTB, 8);
pindef!(pb9, PORTB, 9);
pindef!(pb10, PORTB, 10);
pindef!(pb11, PORTB, 11);
pindef!(pb12, PORTB, 12);
pindef!(pb13, PORTB, 13);
pindef!(pb14, PORTB, 14);
pindef!(pb15, PORTB, 15);
pindef!(pb16, PORTB, 16);
pindef!(pb17, PORTB, 17);
pindef!(pb18, PORTB, 18);
pindef!(pb19, PORTB, 19);
pindef!(pb20, PORTB, 20);
pindef!(pb21, PORTB, 21);
pindef!(pb22, PORTB, 22);
pindef!(pb23, PORTB, 23);
pindef!(pb24, PORTB, 24);
pindef!(pb25, PORTB, 25);
pindef!(pb26, PORTB, 26);
pindef!(pb27, PORTB, 27);
pindef!(pb28, PORTB, 28);
pindef!(pb29, PORTB, 29);
pindef!(pb30, PORTB, 30);
pindef!(pb31, PORTB, 31);
