pub type Handler = unsafe extern "C" fn();

pub const IRQ_GPIOA: Irq = Irq(0);       // IRQ 0: No Description
pub const IRQ_GPIOB: Irq = Irq(1);       // IRQ 1: No Description
pub const IRQ_GPIOC: Irq = Irq(2);       // IRQ 2: No Description
pub const IRQ_GPIOD: Irq = Irq(3);       // IRQ 3: No Description
pub const IRQ_GPIOE: Irq = Irq(4);       // IRQ 4: No Description
pub const IRQ_UART0: Irq = Irq(5);       // IRQ 5: No Description
pub const IRQ_UART1: Irq = Irq(6);       // IRQ 6: No Description
pub const IRQ_SSI0: Irq = Irq(7);        // IRQ 7: No Description
pub const IRQ_I2C0: Irq = Irq(8);        // IRQ 8: No Description
pub const IRQ_TIMER0A: Irq = Irq(19);    // IRQ 19: No Description
pub const IRQ_TIMER0B: Irq = Irq(20);    // IRQ 20: No Description
pub const IRQ_TIMER1A: Irq = Irq(21);    // IRQ 21: No Description
pub const IRQ_TIMER1B: Irq = Irq(22);    // IRQ 22: No Description
pub const IRQ_TIMER2A: Irq = Irq(23);    // IRQ 23: No Description
pub const IRQ_TIMER2B: Irq = Irq(24);    // IRQ 24: No Description
pub const IRQ_SYSCTL: Irq = Irq(28);     // IRQ 28: No Description
pub const IRQ_GPIOF: Irq = Irq(30);      // IRQ 30: No Description
pub const IRQ_GPIOG: Irq = Irq(31);      // IRQ 31: No Description
pub const IRQ_GPIOH: Irq = Irq(32);      // IRQ 32: No Description
pub const IRQ_UART2: Irq = Irq(33);      // IRQ 33: No Description
pub const IRQ_SSI1: Irq = Irq(34);       // IRQ 34: No Description
pub const IRQ_TIMER3A: Irq = Irq(35);    // IRQ 35: No Description
pub const IRQ_TIMER3B: Irq = Irq(36);    // IRQ 36: No Description
pub const IRQ_I2C1: Irq = Irq(37);       // IRQ 37: No Description
pub const IRQ_EMAC0: Irq = Irq(40);      // IRQ 40: No Description
pub const IRQ_GPIOL: Irq = Irq(53);      // IRQ 53: No Description
pub const IRQ_GPIOJ: Irq = Irq(54);      // IRQ 54: No Description
pub const IRQ_GPIOK: Irq = Irq(55);      // IRQ 55: No Description
pub const IRQ_UART3: Irq = Irq(56);      // IRQ 56: No Description
pub const IRQ_UART4: Irq = Irq(57);      // IRQ 57: No Description
pub const IRQ_UART5: Irq = Irq(58);      // IRQ 58: No Description
pub const IRQ_UART6: Irq = Irq(59);      // IRQ 59: No Description
pub const IRQ_UART7: Irq = Irq(60);      // IRQ 60: No Description
pub const IRQ_I2C2: Irq = Irq(61);       // IRQ 61: No Description
pub const IRQ_I2C3: Irq = Irq(62);       // IRQ 62: No Description
pub const IRQ_TIMER4A: Irq = Irq(63);    // IRQ 63: No Description
pub const IRQ_TIMER4B: Irq = Irq(64);    // IRQ 64: No Description
pub const IRQ_TIMER5A: Irq = Irq(65);    // IRQ 65: No Description
pub const IRQ_TIMER5B: Irq = Irq(66);    // IRQ 66: No Description
pub const IRQ_GPIOM: Irq = Irq(72);      // IRQ 72: No Description
pub const IRQ_GPION: Irq = Irq(73);      // IRQ 73: No Description
pub const IRQ_GPIOP0: Irq = Irq(76);     // IRQ 76: No Description
pub const IRQ_GPIOP1: Irq = Irq(77);     // IRQ 77: No Description
pub const IRQ_GPIOP2: Irq = Irq(78);     // IRQ 78: No Description
pub const IRQ_GPIOP3: Irq = Irq(79);     // IRQ 79: No Description
pub const IRQ_GPIOP4: Irq = Irq(80);     // IRQ 80: No Description
pub const IRQ_GPIOP5: Irq = Irq(81);     // IRQ 81: No Description
pub const IRQ_GPIOP6: Irq = Irq(82);     // IRQ 82: No Description
pub const IRQ_GPIOP7: Irq = Irq(83);     // IRQ 83: No Description
pub const IRQ_GPIOQ0: Irq = Irq(84);     // IRQ 84: No Description
pub const IRQ_GPIOQ1: Irq = Irq(85);     // IRQ 85: No Description
pub const IRQ_GPIOQ2: Irq = Irq(86);     // IRQ 86: No Description
pub const IRQ_GPIOQ3: Irq = Irq(87);     // IRQ 87: No Description
pub const IRQ_GPIOQ4: Irq = Irq(88);     // IRQ 88: No Description
pub const IRQ_GPIOQ5: Irq = Irq(89);     // IRQ 89: No Description
pub const IRQ_GPIOQ6: Irq = Irq(90);     // IRQ 90: No Description
pub const IRQ_GPIOQ7: Irq = Irq(91);     // IRQ 91: No Description
pub const IRQ_TIMER6A: Irq = Irq(98);    // IRQ 98: No Description
pub const IRQ_TIMER6B: Irq = Irq(99);    // IRQ 99: No Description
pub const IRQ_TIMER7A: Irq = Irq(100);   // IRQ 100: No Description
pub const IRQ_TIMER7B: Irq = Irq(101);   // IRQ 101: No Description

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
   None,                          // IRQ 19: No Description
   None,                          // IRQ 20: No Description
   None,                          // IRQ 21: No Description
   None,                          // IRQ 22: No Description
   None,                          // IRQ 23: No Description
   None,                          // IRQ 24: No Description
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
   None,                          // IRQ 35: No Description
   None,                          // IRQ 36: No Description
   None,                          // IRQ 37: No Description
   None,
   None,
   None,                          // IRQ 40: No Description
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
   None,                          // IRQ 63: No Description
   None,                          // IRQ 64: No Description
   None,                          // IRQ 65: No Description
   None,                          // IRQ 66: No Description
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
   None,                          // IRQ 98: No Description
   None,                          // IRQ 99: No Description
   None,                          // IRQ 100: No Description
   None,                          // IRQ 101: No Description
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

