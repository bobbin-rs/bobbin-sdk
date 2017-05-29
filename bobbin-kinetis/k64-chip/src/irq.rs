pub type Handler = unsafe extern "C" fn();

pub const IRQ_I2C0: Irq = Irq(24);       // IRQ 24: No Description
pub const IRQ_I2C1: Irq = Irq(25);       // IRQ 25: No Description
pub const IRQ_SPI0: Irq = Irq(26);       // IRQ 26: No Description
pub const IRQ_SPI1: Irq = Irq(27);       // IRQ 27: No Description
pub const IRQ_UART0_LON: Irq = Irq(30);  // IRQ 30: No Description
pub const IRQ_UART0_RX_TX: Irq = Irq(31); // IRQ 31: No Description
pub const IRQ_UART0_ERR: Irq = Irq(32);  // IRQ 32: No Description
pub const IRQ_UART1_RX_TX: Irq = Irq(33); // IRQ 33: No Description
pub const IRQ_UART1_ERR: Irq = Irq(34);  // IRQ 34: No Description
pub const IRQ_UART2_RX_TX: Irq = Irq(35); // IRQ 35: No Description
pub const IRQ_UART2_ERR: Irq = Irq(36);  // IRQ 36: No Description
pub const IRQ_UART3_RX_TX: Irq = Irq(37); // IRQ 37: No Description
pub const IRQ_UART3_ERR: Irq = Irq(38);  // IRQ 38: No Description
pub const IRQ_PIT0: Irq = Irq(48);       // IRQ 48: No Description
pub const IRQ_PIT1: Irq = Irq(49);       // IRQ 49: No Description
pub const IRQ_PIT2: Irq = Irq(50);       // IRQ 50: No Description
pub const IRQ_PIT3: Irq = Irq(51);       // IRQ 51: No Description
pub const IRQ_PORTA: Irq = Irq(59);      // IRQ 59: No Description
pub const IRQ_PORTB: Irq = Irq(60);      // IRQ 60: No Description
pub const IRQ_PORTC: Irq = Irq(61);      // IRQ 61: No Description
pub const IRQ_PORTD: Irq = Irq(62);      // IRQ 62: No Description
pub const IRQ_PORTE: Irq = Irq(63);      // IRQ 63: No Description
pub const IRQ_SPI2: Irq = Irq(65);       // IRQ 65: No Description
pub const IRQ_UART4_RX_TX: Irq = Irq(66); // IRQ 66: No Description
pub const IRQ_UART4_ERR: Irq = Irq(67);  // IRQ 67: No Description
pub const IRQ_UART5_RX_TX: Irq = Irq(68); // IRQ 68: No Description
pub const IRQ_UART5_ERR: Irq = Irq(69);  // IRQ 69: No Description
pub const IRQ_ENET_1588_TIMER: Irq = Irq(82); // IRQ 82: No Description
pub const IRQ_ENET_TRANSMIT: Irq = Irq(83); // IRQ 83: No Description
pub const IRQ_ENET_RECEIVE: Irq = Irq(84); // IRQ 84: No Description
pub const IRQ_ENET_ERROR: Irq = Irq(85); // IRQ 85: No Description

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
   None,
   None,
   None,
   None,
   None,
   None,
   None,
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
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 48: No Description
   None,                          // IRQ 49: No Description
   None,                          // IRQ 50: No Description
   None,                          // IRQ 51: No Description
   None,
   None,
   None,
   None,
   None,
   None,
   None,
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
   None,
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

#[link_section = ".bss.r_interrupts"]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 86] = [None; 86];

