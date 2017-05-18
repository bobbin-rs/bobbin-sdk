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
pub fn usart2_enabled() -> usart::UsartDevice {
    let tx = ::chip::gpio::PA2_USART2_TX;
    let rx = ::chip::gpio::PA15_USART2_RX;

    rcc::set_pinfn_enabled(tx, true);
    rcc::set_pinfn_enabled(rx, true);
    rcc::set_usart_enabled(USART2, true);
    let _tx = gpio::pinfn(tx);
    let _rx = gpio::pinfn(rx);
    let u = usart::device(USART2);
    u.enable(32_000_000 / 115_200);    
    u
}

pub unsafe fn usart2() -> usart::UsartDevice {
    usart::device(USART2) 
}