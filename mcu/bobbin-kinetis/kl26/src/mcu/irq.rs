//! Interrupts

#[allow(unused_imports)] use ::bobbin_common::*;

pub type Handler = unsafe extern "C" fn();

irq_number!(IRQ_0, Irq0, 0);
irq_number!(IRQ_1, Irq1, 1);
irq_number!(IRQ_2, Irq2, 2);
irq_number!(IRQ_3, Irq3, 3);
irq_number!(IRQ_4, Irq4, 4);
irq_number!(IRQ_5, Irq5, 5);
irq_number!(IRQ_6, Irq6, 6);
irq_number!(IRQ_7, Irq7, 7);
irq_number!(IRQ_8, Irq8, 8);
irq_number!(IRQ_9, Irq9, 9);
irq_number!(IRQ_10, Irq10, 10);
irq_number!(IRQ_11, Irq11, 11);
irq_number!(IRQ_12, Irq12, 12);
irq_number!(IRQ_13, Irq13, 13);
irq_number!(IRQ_14, Irq14, 14);
irq_number!(IRQ_15, Irq15, 15);
irq_number!(IRQ_16, Irq16, 16);
irq_number!(IRQ_17, Irq17, 17);
irq_number!(IRQ_18, Irq18, 18);
irq_number!(IRQ_19, Irq19, 19);
irq_number!(IRQ_20, Irq20, 20);
irq_number!(IRQ_21, Irq21, 21);
irq_number!(IRQ_22, Irq22, 22);
irq_number!(IRQ_23, Irq23, 23);
irq_number!(IRQ_24, Irq24, 24);
irq_number!(IRQ_25, Irq25, 25);
irq_number!(IRQ_26, Irq26, 26);
irq_number!(IRQ_27, Irq27, 27);
irq_number!(IRQ_28, Irq28, 28);
irq_number!(IRQ_29, Irq29, 29);
irq_number!(IRQ_30, Irq30, 30);
irq_number!(IRQ_31, Irq31, 31);
irq_number!(IRQ_32, Irq32, 32);
irq_number!(IRQ_33, Irq33, 33);
irq_number!(IRQ_34, Irq34, 34);
irq_number!(IRQ_35, Irq35, 35);
irq_number!(IRQ_36, Irq36, 36);
irq_number!(IRQ_37, Irq37, 37);
irq_number!(IRQ_38, Irq38, 38);
irq_number!(IRQ_39, Irq39, 39);
irq_number!(IRQ_40, Irq40, 40);
irq_number!(IRQ_41, Irq41, 41);
irq_number!(IRQ_42, Irq42, 42);
irq_number!(IRQ_43, Irq43, 43);
irq_number!(IRQ_44, Irq44, 44);
irq_number!(IRQ_45, Irq45, 45);
irq_number!(IRQ_46, Irq46, 46);
irq_number!(IRQ_47, Irq47, 47);

extern "C" {
   fn DEFAULT_HANDLER();
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn DH_TRAMPOLINE() {
   DEFAULT_HANDLER();
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_0_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_1_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_2_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_3_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_4_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_5_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_6_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_7_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_8_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_9_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_10_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_11_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_12_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_13_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_14_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_15_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_16_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_17_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_18_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_19_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_20_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_21_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_22_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_23_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_24_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_25_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_26_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_27_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_28_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_29_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_30_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_31_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_32_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_33_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_34_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_35_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_36_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_37_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_38_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_39_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_40_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_41_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_42_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_43_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_44_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_45_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_46_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_47_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[cfg_attr(target_os="none", link_section=".vector_table.interrupts")]
#[no_mangle]
pub static mut INTERRUPTS: [Option<Handler>; 48] = [
    Some(IRQ_0_HANDLER),
    Some(IRQ_1_HANDLER),
    Some(IRQ_2_HANDLER),
    Some(IRQ_3_HANDLER),
    Some(IRQ_4_HANDLER),
    Some(IRQ_5_HANDLER),
    Some(IRQ_6_HANDLER),
    Some(IRQ_7_HANDLER),
    Some(IRQ_8_HANDLER),
    Some(IRQ_9_HANDLER),
    Some(IRQ_10_HANDLER),
    Some(IRQ_11_HANDLER),
    Some(IRQ_12_HANDLER),
    Some(IRQ_13_HANDLER),
    Some(IRQ_14_HANDLER),
    Some(IRQ_15_HANDLER),
    Some(IRQ_16_HANDLER),
    Some(IRQ_17_HANDLER),
    Some(IRQ_18_HANDLER),
    Some(IRQ_19_HANDLER),
    Some(IRQ_20_HANDLER),
    Some(IRQ_21_HANDLER),
    Some(IRQ_22_HANDLER),
    Some(IRQ_23_HANDLER),
    Some(IRQ_24_HANDLER),
    Some(IRQ_25_HANDLER),
    Some(IRQ_26_HANDLER),
    Some(IRQ_27_HANDLER),
    Some(IRQ_28_HANDLER),
    Some(IRQ_29_HANDLER),
    Some(IRQ_30_HANDLER),
    Some(IRQ_31_HANDLER),
    Some(IRQ_32_HANDLER),
    Some(IRQ_33_HANDLER),
    Some(IRQ_34_HANDLER),
    Some(IRQ_35_HANDLER),
    Some(IRQ_36_HANDLER),
    Some(IRQ_37_HANDLER),
    Some(IRQ_38_HANDLER),
    Some(IRQ_39_HANDLER),
    Some(IRQ_40_HANDLER),
    Some(IRQ_41_HANDLER),
    Some(IRQ_42_HANDLER),
    Some(IRQ_43_HANDLER),
    Some(IRQ_44_HANDLER),
    Some(IRQ_45_HANDLER),
    Some(IRQ_46_HANDLER),
    Some(IRQ_47_HANDLER),
];

irq_type!(IRQ_LPTMR, IrqLptmr);
irq_type!(IRQ_SPI, IrqSpi);

irq!(::dma::Dma, IrqDma, Irq0);
irq!(::dma::Dma, IrqDma, Irq1);
irq!(::dma::Dma, IrqDma, Irq2);
irq!(::dma::Dma, IrqDma, Irq3);
irq_type!(IRQ_DMA, IrqDma);
irq!(::tpm::Tpm0, IrqTpm, Irq17);
irq!(::tpm::Tpm1, IrqTpm, Irq18);
irq!(::tpm::Tpm2, IrqTpm, Irq19);
irq_type!(IRQ_TPM, IrqTpm);
irq!(::pit::Pit, IrqPit, Irq22);
irq_type!(IRQ_PIT, IrqPit);
irq!(::lptmr::Lptmr0, IrqLptmr, Irq44);
irq!(::spi::Spi0, IrqSpi, Irq26);
irq!(::spi::Spi1, IrqSpi, Irq27);
irq!(::i2c::I2c0, IrqI2c, Irq24);
irq!(::i2c::I2c1, IrqI2c, Irq25);
irq_type!(IRQ_I2C, IrqI2c);
irq!(::port::Porta, IrqPort, Irq30);
irq!(::port::Portc, IrqPort, Irq31);
irq!(::port::Portd, IrqPort, Irq31);
irq_type!(IRQ_PORT, IrqPort);
irq!(::uart0::Uart0, IrqUart0, Irq12);
irq_type!(IRQ_UART0, IrqUart0);
irq!(::uart::Uart1, IrqUart, Irq13);
irq!(::uart::Uart2, IrqUart, Irq14);
irq_type!(IRQ_UART, IrqUart);
irq!(::adc::Adc0, IrqAdc, Irq39);
irq_type!(IRQ_ADC, IrqAdc);
