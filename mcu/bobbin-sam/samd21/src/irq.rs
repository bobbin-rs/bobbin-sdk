//! Interrupts

pub type Handler = unsafe extern "C" fn();

::bobbin_mcu::irq_number!(IRQ_0, Irq0, 0);
::bobbin_mcu::irq_number!(IRQ_1, Irq1, 1);
::bobbin_mcu::irq_number!(IRQ_2, Irq2, 2);
::bobbin_mcu::irq_number!(IRQ_3, Irq3, 3);
::bobbin_mcu::irq_number!(IRQ_4, Irq4, 4);
::bobbin_mcu::irq_number!(IRQ_5, Irq5, 5);
::bobbin_mcu::irq_number!(IRQ_6, Irq6, 6);
::bobbin_mcu::irq_number!(IRQ_7, Irq7, 7);
::bobbin_mcu::irq_number!(IRQ_8, Irq8, 8);
::bobbin_mcu::irq_number!(IRQ_9, Irq9, 9);
::bobbin_mcu::irq_number!(IRQ_10, Irq10, 10);
::bobbin_mcu::irq_number!(IRQ_11, Irq11, 11);
::bobbin_mcu::irq_number!(IRQ_12, Irq12, 12);
::bobbin_mcu::irq_number!(IRQ_13, Irq13, 13);
::bobbin_mcu::irq_number!(IRQ_14, Irq14, 14);
::bobbin_mcu::irq_number!(IRQ_15, Irq15, 15);
::bobbin_mcu::irq_number!(IRQ_16, Irq16, 16);
::bobbin_mcu::irq_number!(IRQ_17, Irq17, 17);
::bobbin_mcu::irq_number!(IRQ_18, Irq18, 18);
::bobbin_mcu::irq_number!(IRQ_19, Irq19, 19);
::bobbin_mcu::irq_number!(IRQ_20, Irq20, 20);
::bobbin_mcu::irq_number!(IRQ_21, Irq21, 21);
::bobbin_mcu::irq_number!(IRQ_22, Irq22, 22);
::bobbin_mcu::irq_number!(IRQ_23, Irq23, 23);
::bobbin_mcu::irq_number!(IRQ_24, Irq24, 24);
::bobbin_mcu::irq_number!(IRQ_25, Irq25, 25);
::bobbin_mcu::irq_number!(IRQ_26, Irq26, 26);
::bobbin_mcu::irq_number!(IRQ_27, Irq27, 27);

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
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_1_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_2_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_3_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_4_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_5_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_6_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_7_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_8_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_9_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_10_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_11_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_12_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_13_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_14_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_15_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_16_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_17_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_18_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_19_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_20_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_21_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_22_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_23_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_24_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_25_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_26_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_27_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[cfg_attr(target_os="none", link_section=".vector_table.interrupts")]
#[no_mangle]
pub static mut INTERRUPTS: [Option<Handler>; 28] = [
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
];

::bobbin_mcu::irq_type!(IRQ_DMA, IrqDma);

::bobbin_mcu::irq!(::dmac::Dmac, ::bobbin_mcu::irq::IrqMain, Irq6);
::bobbin_mcu::irq!(::dmac::Dmac, IrqDma, Irq6);
::bobbin_mcu::irq!(::adc::Adc, ::bobbin_mcu::irq::IrqMain, Irq23);
::bobbin_mcu::irq!(::adc::Adc, IrqAdc, Irq23);
::bobbin_mcu::irq_type!(IRQ_ADC, IrqAdc);
::bobbin_mcu::irq!(::dac::Dac, ::bobbin_mcu::irq::IrqMain, Irq25);
::bobbin_mcu::irq!(::dac::Dac, IrqDac, Irq25);
::bobbin_mcu::irq_type!(IRQ_DAC, IrqDac);
::bobbin_mcu::irq!(::tcc::Tcc0, ::bobbin_mcu::irq::IrqMain, Irq15);
::bobbin_mcu::irq!(::tcc::Tcc0, IrqTcc, Irq15);
::bobbin_mcu::irq!(::tcc::Tcc1, ::bobbin_mcu::irq::IrqMain, Irq16);
::bobbin_mcu::irq!(::tcc::Tcc1, IrqTcc, Irq16);
::bobbin_mcu::irq!(::tcc::Tcc2, ::bobbin_mcu::irq::IrqMain, Irq17);
::bobbin_mcu::irq!(::tcc::Tcc2, IrqTcc, Irq17);
::bobbin_mcu::irq_type!(IRQ_TCC, IrqTcc);
::bobbin_mcu::irq!(::tc::Tc3, ::bobbin_mcu::irq::IrqMain, Irq18);
::bobbin_mcu::irq!(::tc::Tc3, IrqTc, Irq18);
::bobbin_mcu::irq!(::tc::Tc4, ::bobbin_mcu::irq::IrqMain, Irq19);
::bobbin_mcu::irq!(::tc::Tc4, IrqTc, Irq19);
::bobbin_mcu::irq!(::tc::Tc5, ::bobbin_mcu::irq::IrqMain, Irq20);
::bobbin_mcu::irq!(::tc::Tc5, IrqTc, Irq20);
::bobbin_mcu::irq_type!(IRQ_TC, IrqTc);
::bobbin_mcu::irq!(::sercom::Sercom0, ::bobbin_mcu::irq::IrqMain, Irq9);
::bobbin_mcu::irq!(::sercom::Sercom0, IrqSercom, Irq9);
::bobbin_mcu::irq!(::sercom::Sercom1, ::bobbin_mcu::irq::IrqMain, Irq10);
::bobbin_mcu::irq!(::sercom::Sercom1, IrqSercom, Irq10);
::bobbin_mcu::irq!(::sercom::Sercom2, ::bobbin_mcu::irq::IrqMain, Irq11);
::bobbin_mcu::irq!(::sercom::Sercom2, IrqSercom, Irq11);
::bobbin_mcu::irq!(::sercom::Sercom3, ::bobbin_mcu::irq::IrqMain, Irq12);
::bobbin_mcu::irq!(::sercom::Sercom3, IrqSercom, Irq12);
::bobbin_mcu::irq!(::sercom::Sercom4, ::bobbin_mcu::irq::IrqMain, Irq13);
::bobbin_mcu::irq!(::sercom::Sercom4, IrqSercom, Irq13);
::bobbin_mcu::irq!(::sercom::Sercom5, ::bobbin_mcu::irq::IrqMain, Irq14);
::bobbin_mcu::irq!(::sercom::Sercom5, IrqSercom, Irq14);
::bobbin_mcu::irq_type!(IRQ_SERCOM, IrqSercom);
::bobbin_mcu::irq!(::nvmctrl::Nvmctrl, ::bobbin_mcu::irq::IrqMain, Irq5);
::bobbin_mcu::irq!(::pm::Pm, ::bobbin_mcu::irq::IrqMain, Irq0);
::bobbin_mcu::irq!(::sysctrl::Sysctrl, ::bobbin_mcu::irq::IrqMain, Irq1);
::bobbin_mcu::irq!(::wdt::Wdt, ::bobbin_mcu::irq::IrqMain, Irq2);
::bobbin_mcu::irq!(::rtc::Rtc, ::bobbin_mcu::irq::IrqMain, Irq3);
