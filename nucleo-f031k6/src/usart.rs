use chip::usart::{USART1, USART2};
use hal::rcc;
use hal::usart;
use hal::gpio;

pub fn usart1(rx: gpio::PinUnknown, tx: gpio::PinUnknown) -> usart::UsartDevice {
    rcc::set_usart_enabled(USART1, true);
    let tx = tx.into_altfn(gpio::AltFn::AF4);
    let rx = rx.into_altfn(gpio::AltFn::AF4);
    let u = usart::device(USART1, tx, rx);
    u.enable(32_000_000 / 115_200);    
    u
}

pub unsafe fn usart1_unchecked(rx: gpio::PinUnknown, tx: gpio::PinUnknown) -> usart::UsartDevice {
    let tx = tx.into_altfn_unchecked();
    let rx = rx.into_altfn_unchecked();
    usart::device(USART1, tx, rx) 
}

pub fn usart2(rx: gpio::PinUnknown, tx: gpio::PinUnknown) -> usart::UsartDevice {
    rcc::set_usart_enabled(USART2, true);
    let tx = tx.into_altfn(gpio::AltFn::AF4);
    let rx = rx.into_altfn(gpio::AltFn::AF4);
    let u = usart::device(USART2, tx, rx);
    u.enable(32_000_000 / 115_200);    
    u
}

pub unsafe fn usart2_unchecked(rx: gpio::PinUnknown, tx: gpio::PinUnknown) -> usart::UsartDevice {
    let tx = tx.into_altfn_unchecked();
    let rx = rx.into_altfn_unchecked();
    usart::device(USART2, tx, rx) 
}