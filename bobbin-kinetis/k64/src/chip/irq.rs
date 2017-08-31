//! Interrupts

#[allow(unused_imports)] use bobbin_common::*;

pub type Handler = extern "C" fn();

irq!(IRQ_WDOG, IrqWdog, 22);
irq!(IRQ_DMA_ERROR, IrqDmaError, 16);
irq!(IRQ_DMA0, IrqDma0, 0);
irq!(IRQ_DMA1, IrqDma1, 1);
irq!(IRQ_DMA2, IrqDma2, 2);
irq!(IRQ_DMA3, IrqDma3, 3);
irq!(IRQ_DMA4, IrqDma4, 4);
irq!(IRQ_DMA5, IrqDma5, 5);
irq!(IRQ_DMA6, IrqDma6, 6);
irq!(IRQ_DMA7, IrqDma7, 7);
irq!(IRQ_DMA8, IrqDma8, 8);
irq!(IRQ_DMA9, IrqDma9, 9);
irq!(IRQ_DMA10, IrqDma10, 10);
irq!(IRQ_DMA11, IrqDma11, 11);
irq!(IRQ_DMA12, IrqDma12, 12);
irq!(IRQ_DMA13, IrqDma13, 13);
irq!(IRQ_DMA14, IrqDma14, 14);
irq!(IRQ_DMA15, IrqDma15, 15);
irq!(IRQ_FTM0, IrqFtm0, 42);
irq!(IRQ_FTM1, IrqFtm1, 43);
irq!(IRQ_FTM2, IrqFtm2, 44);
irq!(IRQ_PIT0, IrqPit0, 48);
irq!(IRQ_PIT1, IrqPit1, 49);
irq!(IRQ_PIT2, IrqPit2, 50);
irq!(IRQ_PIT3, IrqPit3, 51);
irq!(IRQ_LPTMR0, IrqLptmr0, 58);
irq!(IRQ_SPI0, IrqSpi0, 26);
irq!(IRQ_SPI1, IrqSpi1, 27);
irq!(IRQ_SPI2, IrqSpi2, 65);
irq!(IRQ_I2C0, IrqI2c0, 24);
irq!(IRQ_I2C1, IrqI2c1, 25);
irq!(IRQ_UART0_LON, IrqUart0Lon, 30);
irq!(IRQ_UART0_RX_TX, IrqUart0RxTx, 31);
irq!(IRQ_UART0_ERR, IrqUart0Err, 32);
irq!(IRQ_UART1_RX_TX, IrqUart1RxTx, 33);
irq!(IRQ_UART1_ERR, IrqUart1Err, 34);
irq!(IRQ_UART2_RX_TX, IrqUart2RxTx, 35);
irq!(IRQ_UART2_ERR, IrqUart2Err, 36);
irq!(IRQ_UART3_RX_TX, IrqUart3RxTx, 37);
irq!(IRQ_UART3_ERR, IrqUart3Err, 38);
irq!(IRQ_UART4_RX_TX, IrqUart4RxTx, 66);
irq!(IRQ_UART4_ERR, IrqUart4Err, 67);
irq!(IRQ_UART5_RX_TX, IrqUart5RxTx, 68);
irq!(IRQ_UART5_ERR, IrqUart5Err, 69);
irq!(IRQ_PORTA, IrqPorta, 59);
irq!(IRQ_PORTB, IrqPortb, 60);
irq!(IRQ_PORTC, IrqPortc, 61);
irq!(IRQ_PORTD, IrqPortd, 62);
irq!(IRQ_PORTE, IrqPorte, 63);
irq!(IRQ_ADC0, IrqAdc0, 39);
irq!(IRQ_ADC1, IrqAdc1, 73);

pub fn handler(index: usize) -> Option<Handler> {
   unsafe { 
      R_INTERRUPT_HANDLERS[index]
   } 
}

pub fn set_handler(index: usize, handler: Option<Handler>) {
   unsafe { 
      assert!(R_INTERRUPT_HANDLERS[index].is_some() != handler.is_some());
      R_INTERRUPT_HANDLERS[index] = handler
  };
}

#[cfg_attr(target_os="none", link_section=".vector.interrupts")]
#[no_mangle]
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 86] = [
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 16: No Description
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 22: No Description
   None,
   None,                          // IRQ 24: No Description
   None,                          // IRQ 25: No Description
   None,                          // IRQ 26: No Description
   None,                          // IRQ 27: No Description
   None,
   None,
   None,                          // IRQ 30: No Description
   None,                          // IRQ 31: No Description
   None,                          // IRQ 32: No Description
   None,                          // IRQ 33: No Description
   None,                          // IRQ 34: No Description
   None,                          // IRQ 35: No Description
   None,                          // IRQ 36: No Description
   None,                          // IRQ 37: No Description
   None,                          // IRQ 38: No Description
   None,                          // IRQ 39: No Description
   None,
   None,
   None,                          // IRQ 42: No Description
   None,                          // IRQ 43: No Description
   None,                          // IRQ 44: No Description
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 58: No Description
   None,                          // IRQ 59: No Description
   None,                          // IRQ 60: No Description
   None,                          // IRQ 61: No Description
   None,                          // IRQ 62: No Description
   None,                          // IRQ 63: No Description
   None,
   None,                          // IRQ 65: No Description
   None,                          // IRQ 66: No Description
   None,                          // IRQ 67: No Description
   None,                          // IRQ 68: No Description
   None,                          // IRQ 69: No Description
   None,
   None,
   None,
   None,                          // IRQ 73: No Description
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 82: No Description
   None,                          // IRQ 83: No Description
   None,                          // IRQ 84: No Description
   None,                          // IRQ 85: No Description
];

#[cfg_attr(target_os="none", link_section=".bss.r_interrupts")]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 86] = [None; 86];

