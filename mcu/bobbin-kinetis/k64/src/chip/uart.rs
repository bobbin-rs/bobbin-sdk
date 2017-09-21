#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::uart::*;

periph!( UART0, Uart0, _UART0, UartPeriph, 0x4006a000);
periph!( UART1, Uart1, _UART1, UartPeriph, 0x4006b000);
periph!( UART2, Uart2, _UART2, UartPeriph, 0x4006c000);
periph!( UART3, Uart3, _UART3, UartPeriph, 0x4006d000);
periph!( UART4, Uart4, _UART4, UartPeriph, 0x400ea000);
periph!( UART5, Uart5, _UART5, UartPeriph, 0x400eb000);

impl super::sig::Signal<super::sig::Uart0Tx> for Uart0 {}
impl super::sig::SignalTx<super::sig::Uart0Tx> for Uart0 {}
impl super::sig::Signal<super::sig::Uart0Rx> for Uart0 {}
impl super::sig::SignalRx<super::sig::Uart0Rx> for Uart0 {}

impl super::sig::Signal<super::sig::Uart1Tx> for Uart1 {}
impl super::sig::SignalTx<super::sig::Uart1Tx> for Uart1 {}
impl super::sig::Signal<super::sig::Uart1Rx> for Uart1 {}
impl super::sig::SignalRx<super::sig::Uart1Rx> for Uart1 {}

impl super::sig::Signal<super::sig::Uart2Tx> for Uart2 {}
impl super::sig::SignalTx<super::sig::Uart2Tx> for Uart2 {}
impl super::sig::Signal<super::sig::Uart2Rx> for Uart2 {}
impl super::sig::SignalRx<super::sig::Uart2Rx> for Uart2 {}

impl super::sig::Signal<super::sig::Uart3Tx> for Uart3 {}
impl super::sig::SignalTx<super::sig::Uart3Tx> for Uart3 {}
impl super::sig::Signal<super::sig::Uart3Rx> for Uart3 {}
impl super::sig::SignalRx<super::sig::Uart3Rx> for Uart3 {}

impl super::sig::Signal<super::sig::Uart4Tx> for Uart4 {}
impl super::sig::SignalTx<super::sig::Uart4Tx> for Uart4 {}
impl super::sig::Signal<super::sig::Uart4Rx> for Uart4 {}
impl super::sig::SignalRx<super::sig::Uart4Rx> for Uart4 {}

impl super::sig::Signal<super::sig::Uart5Tx> for Uart5 {}
impl super::sig::SignalTx<super::sig::Uart5Tx> for Uart5 {}
impl super::sig::Signal<super::sig::Uart5Rx> for Uart5 {}
impl super::sig::SignalRx<super::sig::Uart5Rx> for Uart5 {}



pub trait IrqUartLon<T> {
    fn irq_uart_lon(&self) -> T;
}

pub trait IrqUartRxTx<T> {
    fn irq_uart_rx_tx(&self) -> T;
}

pub trait IrqUartErr<T> {
    fn irq_uart_err(&self) -> T;
}

impl IrqUartLon<super::irq::IrqUart0Lon> for Uart0 {
    fn irq_uart_lon(&self) -> super::irq::IrqUart0Lon { super::irq::IRQ_UART0_LON }
}

impl IrqUartRxTx<super::irq::IrqUart0RxTx> for Uart0 {
    fn irq_uart_rx_tx(&self) -> super::irq::IrqUart0RxTx { super::irq::IRQ_UART0_RX_TX }
}

impl IrqUartErr<super::irq::IrqUart0Err> for Uart0 {
    fn irq_uart_err(&self) -> super::irq::IrqUart0Err { super::irq::IRQ_UART0_ERR }
}

impl IrqUartRxTx<super::irq::IrqUart1RxTx> for Uart1 {
    fn irq_uart_rx_tx(&self) -> super::irq::IrqUart1RxTx { super::irq::IRQ_UART1_RX_TX }
}

impl IrqUartErr<super::irq::IrqUart1Err> for Uart1 {
    fn irq_uart_err(&self) -> super::irq::IrqUart1Err { super::irq::IRQ_UART1_ERR }
}

impl IrqUartRxTx<super::irq::IrqUart2RxTx> for Uart2 {
    fn irq_uart_rx_tx(&self) -> super::irq::IrqUart2RxTx { super::irq::IRQ_UART2_RX_TX }
}

impl IrqUartErr<super::irq::IrqUart2Err> for Uart2 {
    fn irq_uart_err(&self) -> super::irq::IrqUart2Err { super::irq::IRQ_UART2_ERR }
}

impl IrqUartRxTx<super::irq::IrqUart3RxTx> for Uart3 {
    fn irq_uart_rx_tx(&self) -> super::irq::IrqUart3RxTx { super::irq::IRQ_UART3_RX_TX }
}

impl IrqUartErr<super::irq::IrqUart3Err> for Uart3 {
    fn irq_uart_err(&self) -> super::irq::IrqUart3Err { super::irq::IRQ_UART3_ERR }
}

impl IrqUartRxTx<super::irq::IrqUart4RxTx> for Uart4 {
    fn irq_uart_rx_tx(&self) -> super::irq::IrqUart4RxTx { super::irq::IRQ_UART4_RX_TX }
}

impl IrqUartErr<super::irq::IrqUart4Err> for Uart4 {
    fn irq_uart_err(&self) -> super::irq::IrqUart4Err { super::irq::IRQ_UART4_ERR }
}

impl IrqUartRxTx<super::irq::IrqUart5RxTx> for Uart5 {
    fn irq_uart_rx_tx(&self) -> super::irq::IrqUart5RxTx { super::irq::IRQ_UART5_RX_TX }
}

impl IrqUartErr<super::irq::IrqUart5Err> for Uart5 {
    fn irq_uart_err(&self) -> super::irq::IrqUart5Err { super::irq::IRQ_UART5_ERR }
}

