use chip::sig::*;
use hal::gpio::*;
use hal::usart::*;
use hal::rcc;


// USART2
// TX = PA2[4] 
// RX = PA15[4]
pub fn init() {
    let tx = PA2;
    let rx = PA15;

    rcc::enable(&USART2);
    rcc::enable(&tx.port());
    rcc::enable(&rx.port());
    tx.mode_altfn(AltFn::<Usart2Tx>::alt_fn(&tx));
    rx.mode_altfn(AltFn::<Usart2Rx>::alt_fn(&rx));

    USART2.enable(32_000_000 / 115_200);
}
