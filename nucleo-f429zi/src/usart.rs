use chip::gpio::*;
use chip::usart::{USART3};
use hal::rcc;
use hal::usart;
use hal::gpio;

// PD8 / PD9
// Clock @ 168_000_000
// APB1 @ Clock / 4 = 42_000_000

pub fn init() {
    let tx = PD8;
    let rx = PD9;

    rcc::set_usart_enabled(USART3, true);
    let _ = gpio::pin(tx).into_altfn(tx.af_usart3_tx());
    let _ = gpio::pin(rx).into_altfn(rx.af_usart3_rx());
    let u = usart::device(USART3);
    u.enable(42_000_000 / 115_200);    
}

pub fn usart3() -> usart::UsartDevice {
    usart::device(USART3) 
}


// use chip::usart::{USART3, USART6};
// use hal::rcc;
// use hal::usart;
// use hal::gpio;

// // Clock @ 168_000_000
// // APB1 @ Clock / 4 = 42_000_000

// pub fn usart3(rx: gpio::PinUnknown, tx: gpio::PinUnknown) -> usart::UsartDevice {
//     rcc::set_usart_enabled(USART3, true);
//     //rcc::set_usart_clock(USART3, rcc::UsartClock::Pclk);

//     let tx = tx.into_altfn(gpio::AltFn::AF7);
//     let rx = rx.into_altfn(gpio::AltFn::AF7);
//     let u = usart::device(USART3, tx, rx);
//     u.enable(42_000_000 / 115_200);
//     u
// }

// pub unsafe fn usart3_unchecked(rx: gpio::PinUnknown, tx: gpio::PinUnknown) -> usart::UsartDevice {
//     let tx = tx.into_altfn_unchecked();
//     let rx = rx.into_altfn_unchecked();
//     usart::device(USART3, tx, rx) 
// }

// pub fn usart6(rx: gpio::PinUnknown, tx: gpio::PinUnknown) -> usart::UsartDevice {
//     rcc::set_usart_enabled(USART6, true);

//     let tx = tx.into_altfn(gpio::AltFn::AF8);
//     let rx = rx.into_altfn(gpio::AltFn::AF8);
//     let u = usart::device(USART6, tx, rx);
//     u.enable(84_000_000 / 115_200);
//     u
// }

// pub unsafe fn usart6_unchecked(rx: gpio::PinUnknown, tx: gpio::PinUnknown) -> usart::UsartDevice {
//     let tx = tx.into_altfn_unchecked();
//     let rx = rx.into_altfn_unchecked();
//     usart::device(USART6, tx, rx) 
// }