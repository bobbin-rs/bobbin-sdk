pub type Handler = unsafe extern "C" fn();

pub const IRQ_PVD_IRQ: Irq = Irq(1);     // IRQ 1: PVD through EXTI line detection interrupt
pub const IRQ_TAMP_STAMP: Irq = Irq(2);  // IRQ 2: Tamper and TimeStamp interrupts through the EXTI line
pub const IRQ_RCC_IRQ: Irq = Irq(5);     // IRQ 5: RCC global interrupt
pub const IRQ_EXTI0: Irq = Irq(6);       // IRQ 6: EXTI Line0 interrupt
pub const IRQ_EXTI1: Irq = Irq(7);       // IRQ 7: EXTI Line1 interrupt
pub const IRQ_EXTI2: Irq = Irq(8);       // IRQ 8: EXTI Line2 interrupt
pub const IRQ_EXTI3: Irq = Irq(9);       // IRQ 9: EXTI Line3 interrupt
pub const IRQ_EXTI4: Irq = Irq(10);      // IRQ 10: EXTI Line4 interrupt
pub const IRQ_DMA1_STREAM0: Irq = Irq(11); // IRQ 11: DMA1 Stream0 global interrupt
pub const IRQ_DMA1_STREAM1: Irq = Irq(12); // IRQ 12: DMA1 Stream1 global interrupt
pub const IRQ_DMA1_STREAM2: Irq = Irq(13); // IRQ 13: DMA1 Stream2 global interrupt
pub const IRQ_DMA1_STREAM3: Irq = Irq(14); // IRQ 14: DMA1 Stream3 global interrupt
pub const IRQ_DMA1_STREAM4: Irq = Irq(15); // IRQ 15: DMA1 Stream4 global interrupt
pub const IRQ_DMA1_STREAM5: Irq = Irq(16); // IRQ 16: DMA1 Stream5 global interrupt
pub const IRQ_DMA1_STREAM6: Irq = Irq(17); // IRQ 17: DMA1 Stream6 global interrupt
pub const IRQ_ADC: Irq = Irq(18);        // IRQ 18: ADC3 global interrupts
pub const IRQ_EXTI9_5: Irq = Irq(23);    // IRQ 23: EXTI Line[9:5] interrupts
pub const IRQ_TIM1_BRK_TIM9: Irq = Irq(24); // IRQ 24: TIM1 Break interrupt and TIM9 global interrupt
pub const IRQ_TIM1_UP_TIM10: Irq = Irq(25); // IRQ 25: TIM1 Update interrupt and TIM10 global interrupt
pub const IRQ_TIM1_TRG_COM_TIM11: Irq = Irq(26); // IRQ 26: TIM1 Trigger and Commutation interrupts and TIM11 global interrupt
pub const IRQ_TIM1_CC: Irq = Irq(27);    // IRQ 27: TIM1 Capture Compare interrupt
pub const IRQ_TIM2: Irq = Irq(28);       // IRQ 28: TIM2 global interrupt
pub const IRQ_TIM3: Irq = Irq(29);       // IRQ 29: TIM3 global interrupt
pub const IRQ_TIM4: Irq = Irq(30);       // IRQ 30: TIM4 global interrupt
pub const IRQ_I2C1_EV: Irq = Irq(31);    // IRQ 31: I2C1 event interrupt
pub const IRQ_I2C1_ER: Irq = Irq(32);    // IRQ 32: I2C1 error interrupt
pub const IRQ_I2C2_EV: Irq = Irq(33);    // IRQ 33: I2C2 event interrupt
pub const IRQ_I2C2_ER: Irq = Irq(34);    // IRQ 34: I2C2 error interrupt
pub const IRQ_SPI1: Irq = Irq(35);       // IRQ 35: SPI1 global interrupt
pub const IRQ_SPI2: Irq = Irq(36);       // IRQ 36: SPI2 global interrupt
pub const IRQ_USART1: Irq = Irq(37);     // IRQ 37: USART1 global interrupt
pub const IRQ_USART2: Irq = Irq(38);     // IRQ 38: USART2 global interrupt
pub const IRQ_USART3: Irq = Irq(39);     // IRQ 39: USART3 global interrupt
pub const IRQ_EXTI15_10: Irq = Irq(40);  // IRQ 40: EXTI Line[15:10] interrupts
pub const IRQ_TIM8_BRK_TIM12: Irq = Irq(43); // IRQ 43: TIM8 Break interrupt and TIM12 global interrupt
pub const IRQ_TIM8_UP_TIM13: Irq = Irq(44); // IRQ 44: TIM8 Update interrupt and TIM13 global interrupt
pub const IRQ_TIM8_TRG_COM_TIM14: Irq = Irq(45); // IRQ 45: TIM8 Trigger and Commutation interrupts and TIM14 global interrupt
pub const IRQ_TIM8_CC: Irq = Irq(46);    // IRQ 46: TIM8 Capture Compare interrupt
pub const IRQ_DMA1_STREAM7: Irq = Irq(47); // IRQ 47: DMA1 Stream7 global interrupt
pub const IRQ_TIM5: Irq = Irq(50);       // IRQ 50: TIM5 global interrupt
pub const IRQ_SPI3: Irq = Irq(51);       // IRQ 51: SPI3 global interrupt
pub const IRQ_UART4_IRQ: Irq = Irq(52);  // IRQ 52: UART4 global interrupt
pub const IRQ_UART5_IRQ: Irq = Irq(53);  // IRQ 53: UART5 global interrupt
pub const IRQ_TIM6_DAC: Irq = Irq(54);   // IRQ 54: TIM6 global interrupt, DAC1 and DAC2 underrun error interrupt
pub const IRQ_TIM7: Irq = Irq(55);       // IRQ 55: TIM7 global interrupt
pub const IRQ_DMA2_STREAM0: Irq = Irq(56); // IRQ 56: DMA2 Stream0 global interrupt
pub const IRQ_DMA2_STREAM1: Irq = Irq(57); // IRQ 57: DMA2 Stream1 global interrupt
pub const IRQ_DMA2_STREAM2: Irq = Irq(58); // IRQ 58: DMA2 Stream2 global interrupt
pub const IRQ_DMA2_STREAM3: Irq = Irq(59); // IRQ 59: DMA2 Stream3 global interrupt
pub const IRQ_DMA2_STREAM4: Irq = Irq(60); // IRQ 60: DMA2 Stream4 global interrupt
pub const IRQ_ETH_IRQ: Irq = Irq(61);    // IRQ 61: Ethernet global interrupt
pub const IRQ_ETH_WKUP_IRQ: Irq = Irq(62); // IRQ 62: Ethernet Wakeup through EXTI line interrupt
pub const IRQ_DMA2_STREAM5: Irq = Irq(68); // IRQ 68: DMA2 Stream5 global interrupt
pub const IRQ_DMA2_STREAM6: Irq = Irq(69); // IRQ 69: DMA2 Stream6 global interrupt
pub const IRQ_DMA2_STREAM7: Irq = Irq(70); // IRQ 70: DMA2 Stream7 global interrupt
pub const IRQ_USART6: Irq = Irq(71);     // IRQ 71: USART6 global interrupt
pub const IRQ_I2C3_EV: Irq = Irq(72);    // IRQ 72: I2C3 event interrupt
pub const IRQ_I2C3_ER: Irq = Irq(73);    // IRQ 73: I2C3 error interrupt
pub const IRQ_SPI4: Irq = Irq(84);       // IRQ 84: SPI 4 global interrupt
pub const IRQ_SPI5: Irq = Irq(85);       // IRQ 85: SPI 5 global interrupt
pub const IRQ_SPI6: Irq = Irq(86);       // IRQ 86: SPI 6 global interrupt

pub fn set_handler(irq: Irq, handler: Option<Handler>) {
  unsafe { R_INTERRUPT_HANDLERS[irq.0] = handler };
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Irq(pub usize);

impl Irq {
   pub fn set_handler(&self, handler: Option<Handler>) {
      unsafe { R_INTERRUPT_HANDLERS[self.0] = handler };
   }
}

#[link_section = ".vector.interrupts"]
#[no_mangle]
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 91] = [
   None,
   None,                          // IRQ 1: PVD through EXTI line detection interrupt
   None,                          // IRQ 2: Tamper and TimeStamp interrupts through the EXTI line
   None,
   None,
   None,                          // IRQ 5: RCC global interrupt
   None,                          // IRQ 6: EXTI Line0 interrupt
   None,                          // IRQ 7: EXTI Line1 interrupt
   None,                          // IRQ 8: EXTI Line2 interrupt
   None,                          // IRQ 9: EXTI Line3 interrupt
   None,                          // IRQ 10: EXTI Line4 interrupt
   None,                          // IRQ 11: DMA1 Stream0 global interrupt
   None,                          // IRQ 12: DMA1 Stream1 global interrupt
   None,                          // IRQ 13: DMA1 Stream2 global interrupt
   None,                          // IRQ 14: DMA1 Stream3 global interrupt
   None,                          // IRQ 15: DMA1 Stream4 global interrupt
   None,                          // IRQ 16: DMA1 Stream5 global interrupt
   None,                          // IRQ 17: DMA1 Stream6 global interrupt
   None,                          // IRQ 18: ADC3 global interrupts
   None,
   None,
   None,
   None,
   None,                          // IRQ 23: EXTI Line[9:5] interrupts
   None,                          // IRQ 24: TIM1 Break interrupt and TIM9 global interrupt
   None,                          // IRQ 25: TIM1 Update interrupt and TIM10 global interrupt
   None,                          // IRQ 26: TIM1 Trigger and Commutation interrupts and TIM11 global interrupt
   None,                          // IRQ 27: TIM1 Capture Compare interrupt
   None,                          // IRQ 28: TIM2 global interrupt
   None,                          // IRQ 29: TIM3 global interrupt
   None,                          // IRQ 30: TIM4 global interrupt
   None,                          // IRQ 31: I2C1 event interrupt
   None,                          // IRQ 32: I2C1 error interrupt
   None,                          // IRQ 33: I2C2 event interrupt
   None,                          // IRQ 34: I2C2 error interrupt
   None,                          // IRQ 35: SPI1 global interrupt
   None,                          // IRQ 36: SPI2 global interrupt
   None,                          // IRQ 37: USART1 global interrupt
   None,                          // IRQ 38: USART2 global interrupt
   None,                          // IRQ 39: USART3 global interrupt
   None,                          // IRQ 40: EXTI Line[15:10] interrupts
   None,
   None,
   None,                          // IRQ 43: TIM8 Break interrupt and TIM12 global interrupt
   None,                          // IRQ 44: TIM8 Update interrupt and TIM13 global interrupt
   None,                          // IRQ 45: TIM8 Trigger and Commutation interrupts and TIM14 global interrupt
   None,                          // IRQ 46: TIM8 Capture Compare interrupt
   None,                          // IRQ 47: DMA1 Stream7 global interrupt
   None,
   None,
   None,                          // IRQ 50: TIM5 global interrupt
   None,                          // IRQ 51: SPI3 global interrupt
   None,                          // IRQ 52: UART4 global interrupt
   None,                          // IRQ 53: UART5 global interrupt
   None,                          // IRQ 54: TIM6 global interrupt, DAC1 and DAC2 underrun error interrupt
   None,                          // IRQ 55: TIM7 global interrupt
   None,                          // IRQ 56: DMA2 Stream0 global interrupt
   None,                          // IRQ 57: DMA2 Stream1 global interrupt
   None,                          // IRQ 58: DMA2 Stream2 global interrupt
   None,                          // IRQ 59: DMA2 Stream3 global interrupt
   None,                          // IRQ 60: DMA2 Stream4 global interrupt
   None,                          // IRQ 61: Ethernet global interrupt
   None,                          // IRQ 62: Ethernet Wakeup through EXTI line interrupt
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 68: DMA2 Stream5 global interrupt
   None,                          // IRQ 69: DMA2 Stream6 global interrupt
   None,                          // IRQ 70: DMA2 Stream7 global interrupt
   None,                          // IRQ 71: USART6 global interrupt
   None,                          // IRQ 72: I2C3 event interrupt
   None,                          // IRQ 73: I2C3 error interrupt
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
   None,                          // IRQ 84: SPI 4 global interrupt
   None,                          // IRQ 85: SPI 5 global interrupt
   None,                          // IRQ 86: SPI 6 global interrupt
   None,
   None,
   None,
   None,
];

#[link_section = ".bss.r_interrupts"]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 91] = [None; 91];

