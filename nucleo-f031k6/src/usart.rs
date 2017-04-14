use chip::usart::USART2;
use hal::rcc;
use driver::usart;
use driver::gpio;

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