use chip::gpio::*;
use chip::usart::{USART2};
use hal::rcc;
use hal::usart;
use hal::gpio;

// PA2 / PA3

pub fn init() {
    let tx = PA2;
    let rx = PA3;

    rcc::set_usart_enabled(USART2, true);
    rcc::set_usart_clock(USART2, rcc::UsartClock::Pclk);
    let _ = gpio::pin(tx).into_altfn(tx.af_usart2_tx());
    let _ = gpio::pin(rx).into_altfn(rx.af_usart2_rx());
    let u = usart::device(USART2);
    u.enable(36_000_000 / 115_200);    
}

pub fn usart2() -> usart::UsartDevice {
    usart::device(USART2) 
}

// pub fn usart2(rx: gpio::PinUnknown, tx: gpio::PinUnknown) -> usart::UsartDevice {
//     rcc::set_usart_enabled(USART2, true);
//     rcc::set_usart_clock(USART2, rcc::UsartClock::Pclk);

//     let _tx = tx.into_altfn(7);
//     let _rx = rx.into_altfn(7);
//     let u = usart::device(USART2);
//     u.enable(36_000_000 / 115_200);
//     u
// }

// pub unsafe fn usart2_unchecked(rx: gpio::PinUnknown, tx: gpio::PinUnknown) -> usart::UsartDevice {
//     let tx = tx.into_altfn_unchecked();
//     let rx = rx.into_altfn_unchecked();
//     usart::device(USART2, tx, rx) 
// }