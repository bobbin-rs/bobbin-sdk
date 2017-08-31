//! Interrupts

#[allow(unused_imports)] use bobbin_common::*;

pub type Handler = extern "C" fn();

irq!(IRQ_ADC0SS0, IrqAdc0ss0, 14);
irq!(IRQ_ADC0SS1, IrqAdc0ss1, 15);
irq!(IRQ_ADC0SS2, IrqAdc0ss2, 16);
irq!(IRQ_ADC0SS3, IrqAdc0ss3, 17);
irq!(IRQ_ADC1SS0, IrqAdc1ss0, 46);
irq!(IRQ_ADC1SS1, IrqAdc1ss1, 47);
irq!(IRQ_ADC1SS2, IrqAdc1ss2, 48);
irq!(IRQ_ADC1SS3, IrqAdc1ss3, 49);
irq!(IRQ_WATCHDOG0, IrqWatchdog0, 18);
irq!(IRQ_UDMA, IrqUdma, 44);
irq!(IRQ_UDMAERR, IrqUdmaerr, 45);
irq!(IRQ_PWM0_FAULT, IrqPwm0Fault, 9);
irq!(IRQ_PWM0_CH0, IrqPwm0Ch0, 10);
irq!(IRQ_PWM0_CH1, IrqPwm0Ch1, 11);
irq!(IRQ_PWM0_CH2, IrqPwm0Ch2, 12);
irq!(IRQ_PWM0_CH3, IrqPwm0Ch3, 43);
irq!(IRQ_TIMER0A, IrqTimer0a, 19);
irq!(IRQ_TIMER0B, IrqTimer0b, 20);
irq!(IRQ_TIMER1A, IrqTimer1a, 21);
irq!(IRQ_TIMER1B, IrqTimer1b, 22);
irq!(IRQ_TIMER2A, IrqTimer2a, 23);
irq!(IRQ_TIMER2B, IrqTimer2b, 24);
irq!(IRQ_TIMER3A, IrqTimer3a, 35);
irq!(IRQ_TIMER3B, IrqTimer3b, 36);
irq!(IRQ_TIMER4A, IrqTimer4a, 63);
irq!(IRQ_TIMER4B, IrqTimer4b, 64);
irq!(IRQ_TIMER5A, IrqTimer5a, 65);
irq!(IRQ_TIMER5B, IrqTimer5b, 66);
irq!(IRQ_TIMER6A, IrqTimer6a, 98);
irq!(IRQ_TIMER6B, IrqTimer6b, 99);
irq!(IRQ_TIMER7A, IrqTimer7a, 100);
irq!(IRQ_TIMER7B, IrqTimer7b, 101);
irq!(IRQ_UART0, IrqUart0, 5);
irq!(IRQ_UART1, IrqUart1, 6);
irq!(IRQ_UART2, IrqUart2, 33);
irq!(IRQ_UART3, IrqUart3, 56);
irq!(IRQ_UART4, IrqUart4, 57);
irq!(IRQ_UART5, IrqUart5, 58);
irq!(IRQ_UART6, IrqUart6, 59);
irq!(IRQ_UART7, IrqUart7, 60);
irq!(IRQ_I2C0, IrqI2c0, 8);
irq!(IRQ_I2C1, IrqI2c1, 37);
irq!(IRQ_I2C2, IrqI2c2, 61);
irq!(IRQ_I2C3, IrqI2c3, 62);
irq!(IRQ_SSI0, IrqSsi0, 7);
irq!(IRQ_SSI1, IrqSsi1, 34);
irq!(IRQ_SSI2, IrqSsi2, 54);
irq!(IRQ_SSI3, IrqSsi3, 55);
irq!(IRQ_GPIOA, IrqGpioa, 0);
irq!(IRQ_GPIOB, IrqGpiob, 1);
irq!(IRQ_GPIOC, IrqGpioc, 2);
irq!(IRQ_GPIOD, IrqGpiod, 3);
irq!(IRQ_GPIOE, IrqGpioe, 4);
irq!(IRQ_GPIOF, IrqGpiof, 30);
irq!(IRQ_GPIOG, IrqGpiog, 31);
irq!(IRQ_GPIOH, IrqGpioh, 32);
irq!(IRQ_GPIOJ, IrqGpioj, 54);
irq!(IRQ_GPIOA_AHB, IrqGpioaAhb, 0);
irq!(IRQ_GPIOB_AHB, IrqGpiobAhb, 1);
irq!(IRQ_GPIOC_AHB, IrqGpiocAhb, 2);
irq!(IRQ_GPIOD_AHB, IrqGpiodAhb, 3);
irq!(IRQ_GPIOE_AHB, IrqGpioeAhb, 4);
irq!(IRQ_GPIOF_AHB, IrqGpiofAhb, 30);
irq!(IRQ_GPIOG_AHB, IrqGpiogAhb, 31);
irq!(IRQ_GPIOH_AHB, IrqGpiohAhb, 32);
irq!(IRQ_GPIOJ_AHB, IrqGpiojAhb, 54);
irq!(IRQ_GPIOK, IrqGpiok, 55);
irq!(IRQ_GPIOL, IrqGpiol, 53);
irq!(IRQ_GPIOM, IrqGpiom, 72);
irq!(IRQ_GPION, IrqGpion, 73);
irq!(IRQ_GPIOP0, IrqGpiop0, 76);
irq!(IRQ_GPIOP1, IrqGpiop1, 77);
irq!(IRQ_GPIOP2, IrqGpiop2, 78);
irq!(IRQ_GPIOP3, IrqGpiop3, 79);
irq!(IRQ_GPIOP4, IrqGpiop4, 80);
irq!(IRQ_GPIOP5, IrqGpiop5, 81);
irq!(IRQ_GPIOP6, IrqGpiop6, 82);
irq!(IRQ_GPIOP7, IrqGpiop7, 83);
irq!(IRQ_GPIOQ0, IrqGpioq0, 84);
irq!(IRQ_GPIOQ1, IrqGpioq1, 85);
irq!(IRQ_GPIOQ2, IrqGpioq2, 86);
irq!(IRQ_GPIOQ3, IrqGpioq3, 87);
irq!(IRQ_GPIOQ4, IrqGpioq4, 88);
irq!(IRQ_GPIOQ5, IrqGpioq5, 89);
irq!(IRQ_GPIOQ6, IrqGpioq6, 90);
irq!(IRQ_GPIOQ7, IrqGpioq7, 91);

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

#[link_section = ".vector.interrupts"]
#[no_mangle]
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 114] = [
   None,                          // IRQ 0: No Description
   None,                          // IRQ 1: No Description
   None,                          // IRQ 2: No Description
   None,                          // IRQ 3: No Description
   None,                          // IRQ 4: No Description
   None,                          // IRQ 5: No Description
   None,                          // IRQ 6: No Description
   None,                          // IRQ 7: No Description
   None,                          // IRQ 8: No Description
   None,                          // IRQ 9: No Description
   None,
   None,
   None,
   None,
   None,                          // IRQ 14: No Description
   None,                          // IRQ 15: No Description
   None,                          // IRQ 16: No Description
   None,                          // IRQ 17: No Description
   None,                          // IRQ 18: No Description
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 28: No Description
   None,
   None,                          // IRQ 30: No Description
   None,                          // IRQ 31: No Description
   None,                          // IRQ 32: No Description
   None,                          // IRQ 33: No Description
   None,                          // IRQ 34: No Description
   None,
   None,
   None,                          // IRQ 37: No Description
   None,
   None,
   None,                          // IRQ 40: No Description
   None,
   None,
   None,
   None,                          // IRQ 44: No Description
   None,                          // IRQ 45: No Description
   None,                          // IRQ 46: No Description
   None,                          // IRQ 47: No Description
   None,                          // IRQ 48: No Description
   None,                          // IRQ 49: No Description
   None,
   None,
   None,
   None,                          // IRQ 53: No Description
   None,                          // IRQ 54: No Description
   None,                          // IRQ 55: No Description
   None,                          // IRQ 56: No Description
   None,                          // IRQ 57: No Description
   None,                          // IRQ 58: No Description
   None,                          // IRQ 59: No Description
   None,                          // IRQ 60: No Description
   None,                          // IRQ 61: No Description
   None,                          // IRQ 62: No Description
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 72: No Description
   None,                          // IRQ 73: No Description
   None,
   None,
   None,                          // IRQ 76: No Description
   None,                          // IRQ 77: No Description
   None,                          // IRQ 78: No Description
   None,                          // IRQ 79: No Description
   None,                          // IRQ 80: No Description
   None,                          // IRQ 81: No Description
   None,                          // IRQ 82: No Description
   None,                          // IRQ 83: No Description
   None,                          // IRQ 84: No Description
   None,                          // IRQ 85: No Description
   None,                          // IRQ 86: No Description
   None,                          // IRQ 87: No Description
   None,                          // IRQ 88: No Description
   None,                          // IRQ 89: No Description
   None,                          // IRQ 90: No Description
   None,                          // IRQ 91: No Description
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
   None,
   None,
   None,
   None,
   None,
   None,
];

#[link_section = ".bss.r_interrupts"]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 114] = [None; 114];

