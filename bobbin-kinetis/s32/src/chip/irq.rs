//! Interrupts

#[allow(unused_imports)] use bobbin_common::*;

pub type Handler = extern "C" fn();

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
irq!(IRQ_FTM0_FAULT, IrqFtm0Fault, 103);
irq!(IRQ_FTM0_OVERFLOW, IrqFtm0Overflow, 104);
irq!(IRQ_FTM0_CH0, IrqFtm0Ch0, 99);
irq!(IRQ_FTM0_CH1, IrqFtm0Ch1, 99);
irq!(IRQ_FTM0_CH2, IrqFtm0Ch2, 100);
irq!(IRQ_FTM0_CH3, IrqFtm0Ch3, 100);
irq!(IRQ_FTM0_CH4, IrqFtm0Ch4, 101);
irq!(IRQ_FTM0_CH5, IrqFtm0Ch5, 101);
irq!(IRQ_FTM0_CH6, IrqFtm0Ch6, 102);
irq!(IRQ_FTM0_CH7, IrqFtm0Ch7, 102);
irq!(IRQ_FTM1_FAULT, IrqFtm1Fault, 109);
irq!(IRQ_FTM1_OVERFLOW, IrqFtm1Overflow, 110);
irq!(IRQ_FTM1_CH0, IrqFtm1Ch0, 105);
irq!(IRQ_FTM1_CH1, IrqFtm1Ch1, 105);
irq!(IRQ_FTM1_CH2, IrqFtm1Ch2, 106);
irq!(IRQ_FTM1_CH3, IrqFtm1Ch3, 106);
irq!(IRQ_FTM1_CH4, IrqFtm1Ch4, 107);
irq!(IRQ_FTM1_CH5, IrqFtm1Ch5, 107);
irq!(IRQ_FTM1_CH6, IrqFtm1Ch6, 108);
irq!(IRQ_FTM1_CH7, IrqFtm1Ch7, 108);
irq!(IRQ_FTM2_FAULT, IrqFtm2Fault, 115);
irq!(IRQ_FTM2_OVERFLOW, IrqFtm2Overflow, 116);
irq!(IRQ_FTM2_CH0, IrqFtm2Ch0, 111);
irq!(IRQ_FTM2_CH1, IrqFtm2Ch1, 111);
irq!(IRQ_FTM2_CH2, IrqFtm2Ch2, 112);
irq!(IRQ_FTM2_CH3, IrqFtm2Ch3, 112);
irq!(IRQ_FTM2_CH4, IrqFtm2Ch4, 113);
irq!(IRQ_FTM2_CH5, IrqFtm2Ch5, 113);
irq!(IRQ_FTM2_CH6, IrqFtm2Ch6, 114);
irq!(IRQ_FTM2_CH7, IrqFtm2Ch7, 114);
irq!(IRQ_FTM3_FAULT, IrqFtm3Fault, 121);
irq!(IRQ_FTM3_OVERFLOW, IrqFtm3Overflow, 122);
irq!(IRQ_FTM3_CH0, IrqFtm3Ch0, 117);
irq!(IRQ_FTM3_CH1, IrqFtm3Ch1, 117);
irq!(IRQ_FTM3_CH2, IrqFtm3Ch2, 118);
irq!(IRQ_FTM3_CH3, IrqFtm3Ch3, 118);
irq!(IRQ_FTM3_CH4, IrqFtm3Ch4, 119);
irq!(IRQ_FTM3_CH5, IrqFtm3Ch5, 119);
irq!(IRQ_FTM3_CH6, IrqFtm3Ch6, 120);
irq!(IRQ_FTM3_CH7, IrqFtm3Ch7, 120);
irq!(IRQ_LPIT0_CH0, IrqLpit0Ch0, 48);
irq!(IRQ_LPIT0_CH1, IrqLpit0Ch1, 49);
irq!(IRQ_LPIT0_CH2, IrqLpit0Ch2, 50);
irq!(IRQ_LPIT0_CH3, IrqLpit0Ch3, 51);
irq!(IRQ_LPTMR0, IrqLptmr0, 58);
irq!(IRQ_CAN0_ORED, IrqCan0Ored, 78);
irq!(IRQ_CAN0_ERROR, IrqCan0Error, 79);
irq!(IRQ_CAN0_WAKE_UP, IrqCan0WakeUp, 80);
irq!(IRQ_CAN0_ORED_0_15_MB, IrqCan0Ored015Mb, 81);
irq!(IRQ_CAN0_ORED_16_31_MB, IrqCan0Ored1631Mb, 82);
irq!(IRQ_CAN1_ORED, IrqCan1Ored, 85);
irq!(IRQ_CAN1_ERROR, IrqCan1Error, 86);
irq!(IRQ_CAN1_ORED_0_15_MB, IrqCan1Ored015Mb, 88);
irq!(IRQ_CAN2_ORED, IrqCan2Ored, 92);
irq!(IRQ_CAN2_ERROR, IrqCan2Error, 93);
irq!(IRQ_CAN2_ORED_0_15_MB, IrqCan2Ored015Mb, 95);
irq!(IRQ_PORTA, IrqPorta, 59);
irq!(IRQ_PORTB, IrqPortb, 60);
irq!(IRQ_PORTC, IrqPortc, 61);
irq!(IRQ_PORTD, IrqPortd, 62);
irq!(IRQ_PORTE, IrqPorte, 63);
irq!(IRQ_LPUART0_RXTX, IrqLpuart0Rxtx, 31);
irq!(IRQ_LPUART1_RXTX, IrqLpuart1Rxtx, 33);
irq!(IRQ_LPUART2_RXTX, IrqLpuart2Rxtx, 35);
irq!(IRQ_LPSPI0, IrqLpspi0, 26);
irq!(IRQ_LPSPI1, IrqLpspi1, 27);
irq!(IRQ_LPSPI2, IrqLpspi2, 28);
irq!(IRQ_ADC0, IrqAdc0, 39);
irq!(IRQ_ADC1, IrqAdc1, 40);

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
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 147] = [
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
    None,                          // IRQ 23: No Description
    None,
    None,
    None,                          // IRQ 26: No Description
    None,                          // IRQ 27: No Description
    None,                          // IRQ 28: No Description
    None,
    None,
    None,                          // IRQ 31: No Description
    None,
    None,                          // IRQ 33: No Description
    None,
    None,                          // IRQ 35: No Description
    None,
    None,
    None,
    None,                          // IRQ 39: No Description
    None,                          // IRQ 40: No Description
    None,
    None,
    None,
    None,
    None,
    None,                          // IRQ 46: No Description
    None,                          // IRQ 47: No Description
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,                          // IRQ 57: No Description
    None,                          // IRQ 58: No Description
    None,                          // IRQ 59: No Description
    None,                          // IRQ 60: No Description
    None,                          // IRQ 61: No Description
    None,                          // IRQ 62: No Description
    None,                          // IRQ 63: No Description
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
    None,                          // IRQ 78: No Description
    None,                          // IRQ 79: No Description
    None,                          // IRQ 80: No Description
    None,                          // IRQ 81: No Description
    None,                          // IRQ 82: No Description
    None,
    None,
    None,                          // IRQ 85: No Description
    None,                          // IRQ 86: No Description
    None,
    None,                          // IRQ 88: No Description
    None,
    None,
    None,
    None,                          // IRQ 92: No Description
    None,                          // IRQ 93: No Description
    None,
    None,                          // IRQ 95: No Description
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,                          // IRQ 103: No Description
    None,                          // IRQ 104: No Description
    None,
    None,
    None,
    None,
    None,                          // IRQ 109: No Description
    None,                          // IRQ 110: No Description
    None,
    None,
    None,
    None,
    None,                          // IRQ 115: No Description
    None,                          // IRQ 116: No Description
    None,
    None,
    None,
    None,
    None,                          // IRQ 121: No Description
    None,                          // IRQ 122: No Description
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
];

#[cfg_attr(target_os="none", link_section=".bss.r_interrupts")]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 147] = [None; 147];

