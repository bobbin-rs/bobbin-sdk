use chip::gpio::*;
use chip::usart::{USART3};
use hal::rcc;
use hal::usart;
use hal::gpio;

// PD8 / PD9
// Clock @ 120_000_000
// APB1 @ Clock / 4 = 30_000_000

pub fn init() {
    let tx = PD8;
    let rx = PD9;

    rcc::set_usart_enabled(USART3, true);
    let _ = gpio::pin(tx).into_altfn(tx.af_usart3_tx());
    let _ = gpio::pin(rx).into_altfn(rx.af_usart3_rx());
    let u = usart::device(USART3);
    u.enable(30_000_000 / 115_200);    
}

pub fn usart3() -> usart::UsartDevice {
    usart::device(USART3) 
}