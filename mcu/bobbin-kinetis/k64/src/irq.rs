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
::bobbin_mcu::irq_number!(IRQ_28, Irq28, 28);
::bobbin_mcu::irq_number!(IRQ_29, Irq29, 29);
::bobbin_mcu::irq_number!(IRQ_30, Irq30, 30);
::bobbin_mcu::irq_number!(IRQ_31, Irq31, 31);
::bobbin_mcu::irq_number!(IRQ_32, Irq32, 32);
::bobbin_mcu::irq_number!(IRQ_33, Irq33, 33);
::bobbin_mcu::irq_number!(IRQ_34, Irq34, 34);
::bobbin_mcu::irq_number!(IRQ_35, Irq35, 35);
::bobbin_mcu::irq_number!(IRQ_36, Irq36, 36);
::bobbin_mcu::irq_number!(IRQ_37, Irq37, 37);
::bobbin_mcu::irq_number!(IRQ_38, Irq38, 38);
::bobbin_mcu::irq_number!(IRQ_39, Irq39, 39);
::bobbin_mcu::irq_number!(IRQ_40, Irq40, 40);
::bobbin_mcu::irq_number!(IRQ_41, Irq41, 41);
::bobbin_mcu::irq_number!(IRQ_42, Irq42, 42);
::bobbin_mcu::irq_number!(IRQ_43, Irq43, 43);
::bobbin_mcu::irq_number!(IRQ_44, Irq44, 44);
::bobbin_mcu::irq_number!(IRQ_45, Irq45, 45);
::bobbin_mcu::irq_number!(IRQ_46, Irq46, 46);
::bobbin_mcu::irq_number!(IRQ_47, Irq47, 47);
::bobbin_mcu::irq_number!(IRQ_48, Irq48, 48);
::bobbin_mcu::irq_number!(IRQ_49, Irq49, 49);
::bobbin_mcu::irq_number!(IRQ_50, Irq50, 50);
::bobbin_mcu::irq_number!(IRQ_51, Irq51, 51);
::bobbin_mcu::irq_number!(IRQ_52, Irq52, 52);
::bobbin_mcu::irq_number!(IRQ_53, Irq53, 53);
::bobbin_mcu::irq_number!(IRQ_54, Irq54, 54);
::bobbin_mcu::irq_number!(IRQ_55, Irq55, 55);
::bobbin_mcu::irq_number!(IRQ_56, Irq56, 56);
::bobbin_mcu::irq_number!(IRQ_57, Irq57, 57);
::bobbin_mcu::irq_number!(IRQ_58, Irq58, 58);
::bobbin_mcu::irq_number!(IRQ_59, Irq59, 59);
::bobbin_mcu::irq_number!(IRQ_60, Irq60, 60);
::bobbin_mcu::irq_number!(IRQ_61, Irq61, 61);
::bobbin_mcu::irq_number!(IRQ_62, Irq62, 62);
::bobbin_mcu::irq_number!(IRQ_63, Irq63, 63);
::bobbin_mcu::irq_number!(IRQ_64, Irq64, 64);
::bobbin_mcu::irq_number!(IRQ_65, Irq65, 65);
::bobbin_mcu::irq_number!(IRQ_66, Irq66, 66);
::bobbin_mcu::irq_number!(IRQ_67, Irq67, 67);
::bobbin_mcu::irq_number!(IRQ_68, Irq68, 68);
::bobbin_mcu::irq_number!(IRQ_69, Irq69, 69);
::bobbin_mcu::irq_number!(IRQ_70, Irq70, 70);
::bobbin_mcu::irq_number!(IRQ_71, Irq71, 71);
::bobbin_mcu::irq_number!(IRQ_72, Irq72, 72);
::bobbin_mcu::irq_number!(IRQ_73, Irq73, 73);
::bobbin_mcu::irq_number!(IRQ_74, Irq74, 74);
::bobbin_mcu::irq_number!(IRQ_75, Irq75, 75);
::bobbin_mcu::irq_number!(IRQ_76, Irq76, 76);
::bobbin_mcu::irq_number!(IRQ_77, Irq77, 77);
::bobbin_mcu::irq_number!(IRQ_78, Irq78, 78);
::bobbin_mcu::irq_number!(IRQ_79, Irq79, 79);
::bobbin_mcu::irq_number!(IRQ_80, Irq80, 80);
::bobbin_mcu::irq_number!(IRQ_81, Irq81, 81);
::bobbin_mcu::irq_number!(IRQ_82, Irq82, 82);
::bobbin_mcu::irq_number!(IRQ_83, Irq83, 83);
::bobbin_mcu::irq_number!(IRQ_84, Irq84, 84);
::bobbin_mcu::irq_number!(IRQ_85, Irq85, 85);

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

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_28_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_29_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_30_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_31_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_32_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_33_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_34_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_35_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_36_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_37_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_38_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_39_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_40_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_41_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_42_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_43_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_44_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_45_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_46_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_47_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_48_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_49_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_50_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_51_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_52_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_53_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_54_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_55_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_56_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_57_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_58_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_59_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_60_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_61_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_62_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_63_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_64_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_65_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_66_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_67_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_68_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_69_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_70_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_71_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_72_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_73_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_74_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_75_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_76_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_77_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_78_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_79_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_80_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_81_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_82_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_83_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_84_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_85_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[cfg_attr(target_os="none", link_section=".vector_table.interrupts")]
#[no_mangle]
pub static mut INTERRUPTS: [Option<Handler>; 86] = [
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
    Some(IRQ_48_HANDLER),
    Some(IRQ_49_HANDLER),
    Some(IRQ_50_HANDLER),
    Some(IRQ_51_HANDLER),
    Some(IRQ_52_HANDLER),
    Some(IRQ_53_HANDLER),
    Some(IRQ_54_HANDLER),
    Some(IRQ_55_HANDLER),
    Some(IRQ_56_HANDLER),
    Some(IRQ_57_HANDLER),
    Some(IRQ_58_HANDLER),
    Some(IRQ_59_HANDLER),
    Some(IRQ_60_HANDLER),
    Some(IRQ_61_HANDLER),
    Some(IRQ_62_HANDLER),
    Some(IRQ_63_HANDLER),
    Some(IRQ_64_HANDLER),
    Some(IRQ_65_HANDLER),
    Some(IRQ_66_HANDLER),
    Some(IRQ_67_HANDLER),
    Some(IRQ_68_HANDLER),
    Some(IRQ_69_HANDLER),
    Some(IRQ_70_HANDLER),
    Some(IRQ_71_HANDLER),
    Some(IRQ_72_HANDLER),
    Some(IRQ_73_HANDLER),
    Some(IRQ_74_HANDLER),
    Some(IRQ_75_HANDLER),
    Some(IRQ_76_HANDLER),
    Some(IRQ_77_HANDLER),
    Some(IRQ_78_HANDLER),
    Some(IRQ_79_HANDLER),
    Some(IRQ_80_HANDLER),
    Some(IRQ_81_HANDLER),
    Some(IRQ_82_HANDLER),
    Some(IRQ_83_HANDLER),
    Some(IRQ_84_HANDLER),
    Some(IRQ_85_HANDLER),
];

::bobbin_mcu::irq_type!(IRQ_WDOG, IrqWdog);
::bobbin_mcu::irq_type!(IRQ_FTFE, IrqFtfe);
::bobbin_mcu::irq_type!(IRQ_READ_COLLISION, IrqReadCollision);
::bobbin_mcu::irq_type!(IRQ_DMA_ERROR, IrqDmaError);
::bobbin_mcu::irq_type!(IRQ_DMA, IrqDma);
::bobbin_mcu::irq_type!(IRQ_FTM, IrqFtm);
::bobbin_mcu::irq_type!(IRQ_PIT, IrqPit);
::bobbin_mcu::irq_type!(IRQ_LPTMR, IrqLptmr);
::bobbin_mcu::irq_type!(IRQ_SPI, IrqSpi);
::bobbin_mcu::irq_type!(IRQ_I2C, IrqI2c);
::bobbin_mcu::irq_type!(IRQ_UART_LON, IrqUartLon);
::bobbin_mcu::irq_type!(IRQ_UART, IrqUart);
::bobbin_mcu::irq_type!(IRQ_UART_ERR, IrqUartErr);
::bobbin_mcu::irq_type!(IRQ_USB, IrqUsb);
::bobbin_mcu::irq_type!(IRQ_CAN_MB, IrqCanMb);
::bobbin_mcu::irq_type!(IRQ_CAN_BUS_OFF, IrqCanBusOff);
::bobbin_mcu::irq_type!(IRQ_CAN_ERR, IrqCanErr);
::bobbin_mcu::irq_type!(IRQ_CAN_TX_WARN, IrqCanTxWarn);
::bobbin_mcu::irq_type!(IRQ_CAN_RX_WARM, IrqCanRxWarm);
::bobbin_mcu::irq_type!(IRQ_CAN_WAKE, IrqCanWake);
::bobbin_mcu::irq_type!(IRQ_PORT, IrqPort);
::bobbin_mcu::irq_type!(IRQ_ADC, IrqAdc);

::bobbin_mcu::irq!(::wdog::Wdog, ::bobbin_mcu::irq::IrqMain, Irq22);
::bobbin_mcu::irq!(::wdog::Wdog, IrqWdog, Irq22);
::bobbin_mcu::irq!(::ftfe::Ftfe, ::bobbin_mcu::irq::IrqMain, Irq18);
::bobbin_mcu::irq!(::ftfe::Ftfe, IrqFtfe, Irq18);
::bobbin_mcu::irq!(::ftfe::Ftfe, IrqReadCollision, Irq19);
::bobbin_mcu::irq!(::edma::Dma, ::bobbin_mcu::irq::IrqMain, Irq16);
::bobbin_mcu::irq!(::edma::Dma, IrqDmaError, Irq16);
::bobbin_mcu::irq!(::edma::Dma0, ::bobbin_mcu::irq::IrqMain, Irq0);
::bobbin_mcu::irq!(::edma::Dma0, IrqDma, Irq0);
::bobbin_mcu::irq!(::edma::Dma1, ::bobbin_mcu::irq::IrqMain, Irq1);
::bobbin_mcu::irq!(::edma::Dma1, IrqDma, Irq1);
::bobbin_mcu::irq!(::edma::Dma2, ::bobbin_mcu::irq::IrqMain, Irq2);
::bobbin_mcu::irq!(::edma::Dma2, IrqDma, Irq2);
::bobbin_mcu::irq!(::edma::Dma3, ::bobbin_mcu::irq::IrqMain, Irq3);
::bobbin_mcu::irq!(::edma::Dma3, IrqDma, Irq3);
::bobbin_mcu::irq!(::edma::Dma4, ::bobbin_mcu::irq::IrqMain, Irq4);
::bobbin_mcu::irq!(::edma::Dma4, IrqDma, Irq4);
::bobbin_mcu::irq!(::edma::Dma5, ::bobbin_mcu::irq::IrqMain, Irq5);
::bobbin_mcu::irq!(::edma::Dma5, IrqDma, Irq5);
::bobbin_mcu::irq!(::edma::Dma6, ::bobbin_mcu::irq::IrqMain, Irq6);
::bobbin_mcu::irq!(::edma::Dma6, IrqDma, Irq6);
::bobbin_mcu::irq!(::edma::Dma7, ::bobbin_mcu::irq::IrqMain, Irq7);
::bobbin_mcu::irq!(::edma::Dma7, IrqDma, Irq7);
::bobbin_mcu::irq!(::edma::Dma8, ::bobbin_mcu::irq::IrqMain, Irq8);
::bobbin_mcu::irq!(::edma::Dma8, IrqDma, Irq8);
::bobbin_mcu::irq!(::edma::Dma9, ::bobbin_mcu::irq::IrqMain, Irq9);
::bobbin_mcu::irq!(::edma::Dma9, IrqDma, Irq9);
::bobbin_mcu::irq!(::edma::Dma10, ::bobbin_mcu::irq::IrqMain, Irq10);
::bobbin_mcu::irq!(::edma::Dma10, IrqDma, Irq10);
::bobbin_mcu::irq!(::edma::Dma11, ::bobbin_mcu::irq::IrqMain, Irq11);
::bobbin_mcu::irq!(::edma::Dma11, IrqDma, Irq11);
::bobbin_mcu::irq!(::edma::Dma12, ::bobbin_mcu::irq::IrqMain, Irq12);
::bobbin_mcu::irq!(::edma::Dma12, IrqDma, Irq12);
::bobbin_mcu::irq!(::edma::Dma13, ::bobbin_mcu::irq::IrqMain, Irq13);
::bobbin_mcu::irq!(::edma::Dma13, IrqDma, Irq13);
::bobbin_mcu::irq!(::edma::Dma14, ::bobbin_mcu::irq::IrqMain, Irq14);
::bobbin_mcu::irq!(::edma::Dma14, IrqDma, Irq14);
::bobbin_mcu::irq!(::edma::Dma15, ::bobbin_mcu::irq::IrqMain, Irq15);
::bobbin_mcu::irq!(::edma::Dma15, IrqDma, Irq15);
::bobbin_mcu::irq!(::ftm::Ftm0, ::bobbin_mcu::irq::IrqMain, Irq42);
::bobbin_mcu::irq!(::ftm::Ftm0, IrqFtm, Irq42);
::bobbin_mcu::irq!(::ftm::Ftm1, ::bobbin_mcu::irq::IrqMain, Irq43);
::bobbin_mcu::irq!(::ftm::Ftm1, IrqFtm, Irq43);
::bobbin_mcu::irq!(::ftm::Ftm2, ::bobbin_mcu::irq::IrqMain, Irq44);
::bobbin_mcu::irq!(::ftm::Ftm2, IrqFtm, Irq44);
::bobbin_mcu::irq!(::pit::PitCh0, ::bobbin_mcu::irq::IrqMain, Irq48);
::bobbin_mcu::irq!(::pit::PitCh0, IrqPit, Irq48);
::bobbin_mcu::irq!(::pit::PitCh1, ::bobbin_mcu::irq::IrqMain, Irq49);
::bobbin_mcu::irq!(::pit::PitCh1, IrqPit, Irq49);
::bobbin_mcu::irq!(::pit::PitCh2, ::bobbin_mcu::irq::IrqMain, Irq50);
::bobbin_mcu::irq!(::pit::PitCh2, IrqPit, Irq50);
::bobbin_mcu::irq!(::pit::PitCh3, ::bobbin_mcu::irq::IrqMain, Irq51);
::bobbin_mcu::irq!(::pit::PitCh3, IrqPit, Irq51);
::bobbin_mcu::irq!(::lptmr::Lptmr0, ::bobbin_mcu::irq::IrqMain, Irq58);
::bobbin_mcu::irq!(::lptmr::Lptmr0, IrqLptmr, Irq58);
::bobbin_mcu::irq!(::spi::Spi0, ::bobbin_mcu::irq::IrqMain, Irq26);
::bobbin_mcu::irq!(::spi::Spi0, IrqSpi, Irq26);
::bobbin_mcu::irq!(::spi::Spi1, ::bobbin_mcu::irq::IrqMain, Irq27);
::bobbin_mcu::irq!(::spi::Spi1, IrqSpi, Irq27);
::bobbin_mcu::irq!(::spi::Spi2, ::bobbin_mcu::irq::IrqMain, Irq65);
::bobbin_mcu::irq!(::spi::Spi2, IrqSpi, Irq65);
::bobbin_mcu::irq!(::i2c::I2c0, ::bobbin_mcu::irq::IrqMain, Irq24);
::bobbin_mcu::irq!(::i2c::I2c0, IrqI2c, Irq24);
::bobbin_mcu::irq!(::i2c::I2c1, ::bobbin_mcu::irq::IrqMain, Irq25);
::bobbin_mcu::irq!(::i2c::I2c1, IrqI2c, Irq25);
::bobbin_mcu::irq!(::uart::Uart0, ::bobbin_mcu::irq::IrqMain, Irq30);
::bobbin_mcu::irq!(::uart::Uart0, IrqUartLon, Irq30);
::bobbin_mcu::irq!(::uart::Uart0, IrqUart, Irq31);
::bobbin_mcu::irq!(::uart::Uart0, IrqUartErr, Irq32);
::bobbin_mcu::irq!(::uart::Uart1, ::bobbin_mcu::irq::IrqMain, Irq33);
::bobbin_mcu::irq!(::uart::Uart1, IrqUart, Irq33);
::bobbin_mcu::irq!(::uart::Uart1, IrqUartErr, Irq34);
::bobbin_mcu::irq!(::uart::Uart2, ::bobbin_mcu::irq::IrqMain, Irq35);
::bobbin_mcu::irq!(::uart::Uart2, IrqUart, Irq35);
::bobbin_mcu::irq!(::uart::Uart2, IrqUartErr, Irq36);
::bobbin_mcu::irq!(::uart::Uart3, ::bobbin_mcu::irq::IrqMain, Irq37);
::bobbin_mcu::irq!(::uart::Uart3, IrqUart, Irq37);
::bobbin_mcu::irq!(::uart::Uart3, IrqUartErr, Irq38);
::bobbin_mcu::irq!(::uart::Uart4, ::bobbin_mcu::irq::IrqMain, Irq66);
::bobbin_mcu::irq!(::uart::Uart4, IrqUart, Irq66);
::bobbin_mcu::irq!(::uart::Uart4, IrqUartErr, Irq67);
::bobbin_mcu::irq!(::uart::Uart5, ::bobbin_mcu::irq::IrqMain, Irq68);
::bobbin_mcu::irq!(::uart::Uart5, IrqUart, Irq68);
::bobbin_mcu::irq!(::uart::Uart5, IrqUartErr, Irq69);
::bobbin_mcu::irq!(::usb::Usb0, ::bobbin_mcu::irq::IrqMain, Irq53);
::bobbin_mcu::irq!(::usb::Usb0, IrqUsb, Irq53);
::bobbin_mcu::irq!(::flexcan::Can0, ::bobbin_mcu::irq::IrqMain, Irq75);
::bobbin_mcu::irq!(::flexcan::Can0, IrqCanMb, Irq75);
::bobbin_mcu::irq!(::flexcan::Can0, IrqCanBusOff, Irq76);
::bobbin_mcu::irq!(::flexcan::Can0, IrqCanErr, Irq77);
::bobbin_mcu::irq!(::flexcan::Can0, IrqCanTxWarn, Irq78);
::bobbin_mcu::irq!(::flexcan::Can0, IrqCanRxWarm, Irq79);
::bobbin_mcu::irq!(::flexcan::Can0, IrqCanWake, Irq80);
::bobbin_mcu::irq!(::dac::Dac0, ::bobbin_mcu::irq::IrqMain, Irq56);
::bobbin_mcu::irq!(::dac::Dac0, IrqDac, Irq56);
::bobbin_mcu::irq!(::dac::Dac1, ::bobbin_mcu::irq::IrqMain, Irq72);
::bobbin_mcu::irq!(::dac::Dac1, IrqDac, Irq72);
::bobbin_mcu::irq_type!(IRQ_DAC, IrqDac);
::bobbin_mcu::irq!(::port::Porta, ::bobbin_mcu::irq::IrqMain, Irq59);
::bobbin_mcu::irq!(::port::Porta, IrqPort, Irq59);
::bobbin_mcu::irq!(::port::Portb, ::bobbin_mcu::irq::IrqMain, Irq60);
::bobbin_mcu::irq!(::port::Portb, IrqPort, Irq60);
::bobbin_mcu::irq!(::port::Portc, ::bobbin_mcu::irq::IrqMain, Irq61);
::bobbin_mcu::irq!(::port::Portc, IrqPort, Irq61);
::bobbin_mcu::irq!(::port::Portd, ::bobbin_mcu::irq::IrqMain, Irq62);
::bobbin_mcu::irq!(::port::Portd, IrqPort, Irq62);
::bobbin_mcu::irq!(::port::Porte, ::bobbin_mcu::irq::IrqMain, Irq63);
::bobbin_mcu::irq!(::port::Porte, IrqPort, Irq63);
::bobbin_mcu::irq!(::adc::Adc0, ::bobbin_mcu::irq::IrqMain, Irq39);
::bobbin_mcu::irq!(::adc::Adc0, IrqAdc, Irq39);
::bobbin_mcu::irq!(::adc::Adc1, ::bobbin_mcu::irq::IrqMain, Irq73);
::bobbin_mcu::irq!(::adc::Adc1, IrqAdc, Irq73);
::bobbin_mcu::irq!(::enet::Enet, ::bobbin_mcu::irq::IrqMain, Irq82);
