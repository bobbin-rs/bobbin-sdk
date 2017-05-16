use chip::usart::{USART2};
use chip::pins;
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
    rcc::set_gpio_enabled(pins::PA2_USART2_TX.0, true);
    rcc::set_gpio_enabled(pins::PA15_USART2_RX.0, true);
    rcc::set_usart_enabled(USART2, true);
    let _tx = gpio::pin_af(pins::PA2_USART2_TX);
    let _rx = gpio::pin_af(pins::PA15_USART2_RX);
    let u = usart::device(USART2);
    u.enable(32_000_000 / 115_200);    
    u
}

pub unsafe fn usart2() -> usart::UsartDevice {
    usart::device(USART2) 
}