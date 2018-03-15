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

#[cfg_attr(target_os="none", link_section=".vector_table.interrupts")]
#[no_mangle]
pub static mut INTERRUPTS: [Option<Handler>; 30] = [
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
];

irq_type!(IRQ_WWDG, IrqWwdg);
irq_type!(IRQ_RTC, IrqRtc);
irq_type!(IRQ_LPTIM, IrqLptim);
irq_type!(IRQ_EXTI, IrqExti);
irq_type!(IRQ_DMA, IrqDma);
irq_type!(IRQ_I2C, IrqI2c);
irq_type!(IRQ_TIM, IrqTim);
irq_type!(IRQ_USART, IrqUsart);
irq_type!(IRQ_LPUART, IrqLpuart);
irq_type!(IRQ_SPI, IrqSpi);

irq!(::wwdg::Wwdg, IrqWwdg, Irq0);
irq!(::rtc::Rtc, IrqRtc, Irq2);
irq!(::lptim::Lptim, IrqLptim, Irq13);
irq!(::exti::ExtiLine0, IrqExti, Irq5);
irq!(::exti::ExtiLine1, IrqExti, Irq5);
irq!(::exti::ExtiLine2, IrqExti, Irq6);
irq!(::exti::ExtiLine3, IrqExti, Irq6);
irq!(::exti::ExtiLine4, IrqExti, Irq7);
irq!(::exti::ExtiLine5, IrqExti, Irq7);
irq!(::exti::ExtiLine6, IrqExti, Irq7);
irq!(::exti::ExtiLine7, IrqExti, Irq7);
irq!(::exti::ExtiLine8, IrqExti, Irq7);
irq!(::exti::ExtiLine9, IrqExti, Irq7);
irq!(::exti::ExtiLine10, IrqExti, Irq7);
irq!(::exti::ExtiLine11, IrqExti, Irq7);
irq!(::exti::ExtiLine12, IrqExti, Irq7);
irq!(::exti::ExtiLine13, IrqExti, Irq7);
irq!(::exti::ExtiLine14, IrqExti, Irq7);
irq!(::exti::ExtiLine15, IrqExti, Irq7);
irq!(::exti::ExtiLine16, IrqExti, Irq1);
irq!(::exti::ExtiLine17, IrqExti, Irq2);
irq!(::exti::ExtiLine19, IrqExti, Irq2);
irq!(::exti::ExtiLine20, IrqExti, Irq2);
irq!(::exti::ExtiLine21, IrqExti, Irq12);
irq!(::exti::ExtiLine22, IrqExti, Irq12);
irq!(::exti::ExtiLine23, IrqExti, Irq23);
irq!(::exti::ExtiLine24, IrqExti, Irq24);
irq!(::exti::ExtiLine25, IrqExti, Irq27);
irq!(::exti::ExtiLine26, IrqExti, Irq28);
irq!(::exti::ExtiLine28, IrqExti, Irq29);
irq!(::exti::ExtiLine29, IrqExti, Irq13);
irq!(::dma::Dma1Ch1, IrqDma, Irq9);
irq!(::dma::Dma1Ch2, IrqDma, Irq10);
irq!(::dma::Dma1Ch3, IrqDma, Irq10);
irq!(::dma::Dma1Ch4, IrqDma, Irq11);
irq!(::dma::Dma1Ch5, IrqDma, Irq11);
irq!(::dma::Dma1Ch6, IrqDma, Irq11);
irq!(::dma::Dma1, IrqDma, Irq11);
irq_type!(IRQ_DMA, IrqDma);
irq!(::i2c::I2c1, IrqI2c, Irq23);
irq!(::tim_gen::Tim2, IrqTim, Irq15);
irq!(::tim_gen::Tim21, IrqTim, Irq20);
irq!(::tim_gen::Tim22, IrqTim, Irq22);
irq!(::usart::Usart2, IrqUsart, Irq28);
irq!(::lpuart::Lpuart1, IrqLpuart, Irq29);
irq!(::spi::Spi1, IrqSpi, Irq25);
irq!(::adc::Adc1, IrqAdc, Irq12);
irq_type!(IRQ_ADC, IrqAdc);
