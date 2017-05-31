use chip::sig::*;
use chip::gpio::*;
use chip::usart::{USART2, Usart2};
use hal::rcc::{RCC, RccExt};
use hal::usart::UsartExt;
use hal::gpio::PinExt;


// USART2
// TX = PA2
// RX = PA3
pub fn init() {
    let tx = PA2;
    let rx = PA3;

    RCC.set_enabled(&USART2, true);
    RCC.set_enabled(&tx.port(), true);
    RCC.set_enabled(&rx.port(), true);
    tx.mode_altfn(AltFn::<Usart2Tx>::alt_fn(&tx));
    rx.mode_altfn(AltFn::<Usart2Rx>::alt_fn(&rx));
    USART2.enable(36_000_000 / 115_200);
}

pub fn usart2() -> Usart2 {
    USART2
}