use chip::sig::*;
use chip::gpio::*;
use chip::usart::{USART2, Usart2};
use hal::rcc::{RCC, RccExt};
use hal::usart::UsartExt;
use hal::gpio::PinExt;

// pub fn usart1(rx: gpio::PinUnknown, tx: gpio::PinUnknown) -> usart::UsartDevice {
//     rcc::set_usart_enabled(USART1, true);
//     let tx = tx.into_altfn(4);
//     let rx = rx.into_altfn(4);
//     let u = usart::device(USART1);
//     u.enable(32_000_000 / 115_200);    
//     u
// }

// pub unsafe fn usart1_unchecked(rx: gpio::PinUnknown, tx: gpio::PinUnknown) -> usart::UsartDevice {
//     let tx = tx.into_altfn_unchecked();
//     let rx = rx.into_altfn_unchecked();
//     usart::device(USART1, tx, rx) 
// }


// USART2
// TX = PA2[4] 
// RX = PA15[4]
pub fn init() {
    let tx = PA2;
    let rx = PA15;

    RCC.set_enabled(&USART2, true);
    RCC.set_enabled(&tx.port(), true);
    RCC.set_enabled(&rx.port(), true);
    tx.mode_altfn(AltFn::<Usart2Tx>::alt_fn(&tx));
    rx.mode_altfn(AltFn::<Usart2Rx>::alt_fn(&rx));
    USART2.enable(32_000_000 / 115_200);
}

pub fn usart2() -> Usart2 {
     USART2
}