use chip::gpio::*;
use chip::usart::{USART2};
use hal::rcc;
use hal::usart;
use hal::gpio;

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

    rcc::set_usart_enabled(&USART2, true);    
    gpio::pin(tx).into_altfn(tx.af_usart2_tx());
    gpio::pin(rx).into_altfn(rx.af_usart2_rx());
    let u = usart::device(&USART2);
    u.enable(32_000_000 / 115_200);    
}

pub fn usart2() -> usart::UsartDevice {
    usart::device(&USART2) 
}