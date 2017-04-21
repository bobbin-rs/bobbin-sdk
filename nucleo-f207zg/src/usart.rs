use chip::usart::USART3;
use hal::rcc;
use hal::usart;
use hal::gpio;

// Clock @ 120_000_000
// APB1 @ Clock / 4 = 30_000_000

pub fn usart3(rx: gpio::PinUnknown, tx: gpio::PinUnknown) -> usart::UsartDevice {
    rcc::set_usart_enabled(USART3, true);
    //rcc::set_usart_clock(USART3, rcc::UsartClock::Pclk);

    let tx = tx.into_altfn(gpio::AltFn::AF7);
    let rx = rx.into_altfn(gpio::AltFn::AF7);
    let u = usart::device(USART3, tx, rx);
    u.enable(30_000_000 / 115_200);
    u
}

pub unsafe fn usart3_unchecked(rx: gpio::PinUnknown, tx: gpio::PinUnknown) -> usart::UsartDevice {
    let tx = tx.into_altfn_unchecked();
    let rx = rx.into_altfn_unchecked();
    usart::device(USART3, tx, rx) 
}