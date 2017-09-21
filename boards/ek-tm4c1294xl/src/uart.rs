use chip::uart::UART0;
use hal::sysctl;
use hal::uart;
use pin;

// PA0 and PA1
pub fn init() -> uart::UartDevice {
    sysctl::set_uart_enabled(UART0, true);

    pin::pa0().into_alt_fn(1);
    pin::pa1().into_alt_fn(1);

    let u = uart::device(UART0);
    u.enable(115_200);
    u
}

pub fn uart0() -> uart::UartDevice {
    uart::device(UART0) 
}