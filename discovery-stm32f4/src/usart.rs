use chip::gpio::*;
use chip::usart::{USART1};
use hal::rcc;
use hal::usart;
use hal::gpio;

// USART 1 / PB6 / PB7
// Clock @ 168_000_000
// APB2 @ Clock / 2 = 84_000_000

pub fn init() {
    let tx = PB6;
    let rx = PB7;

    rcc::set_usart_enabled(USART1, true);
    let _ = gpio::pin(tx).into_altfn(tx.af_usart1_tx());
    let _ = gpio::pin(rx).into_altfn(rx.af_usart1_rx());
    let u = usart::device(USART1);
    u.enable(84_000_000 / 115_200);    
}

pub fn usart1() -> usart::UsartDevice {
    usart::device(USART1) 
}

// use chip::usart::USART1;
// use hal::rcc;
// use hal::usart;
// use hal::gpio;

// // Clock @ 168_000_000
// // APB2 @ Clock / 2 = 84_000_000

// pub fn usart3(rx: gpio::PinUnknown, tx: gpio::PinUnknown) -> usart::UsartDevice {
//     rcc::set_usart_enabled(USART1, true);
//     //rcc::set_usart_clock(USART3, rcc::UsartClock::Pclk);

//     let tx = tx.into_altfn(gpio::AltFn::AF7);
//     let rx = rx.into_altfn(gpio::AltFn::AF7);
//     let u = usart::device(USART1, tx, rx);
//     u.enable(84_000_000 / 115_200);
//     u
// }

// pub unsafe fn usart3_unchecked(rx: gpio::PinUnknown, tx: gpio::PinUnknown) -> usart::UsartDevice {
//     let tx = tx.into_altfn_unchecked();
//     let rx = rx.into_altfn_unchecked();
//     usart::device(USART1, tx, rx) 
// }