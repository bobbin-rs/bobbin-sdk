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
::bobbin_mcu::irq_number!(IRQ_86, Irq86, 86);
::bobbin_mcu::irq_number!(IRQ_87, Irq87, 87);
::bobbin_mcu::irq_number!(IRQ_88, Irq88, 88);
::bobbin_mcu::irq_number!(IRQ_89, Irq89, 89);
::bobbin_mcu::irq_number!(IRQ_90, Irq90, 90);

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

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_86_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_87_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_88_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_89_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_90_HANDLER() {
    #[cfg(target_os="none")]
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[cfg_attr(target_os="none", link_section=".vector_table.interrupts")]
#[no_mangle]
pub static mut INTERRUPTS: [Option<Handler>; 91] = [
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
    Some(IRQ_86_HANDLER),
    Some(IRQ_87_HANDLER),
    Some(IRQ_88_HANDLER),
    Some(IRQ_89_HANDLER),
    Some(IRQ_90_HANDLER),
];

::bobbin_mcu::irq_type!(IRQ_WWDG, IrqWwdg);
::bobbin_mcu::irq_type!(IRQ_EXTI, IrqExti);
::bobbin_mcu::irq_type!(IRQ_TIM, IrqTim);
::bobbin_mcu::irq_type!(IRQ_BRK, IrqBrk);
::bobbin_mcu::irq_type!(IRQ_UP, IrqUp);
::bobbin_mcu::irq_type!(IRQ_TRG_COM, IrqTrgCom);
::bobbin_mcu::irq_type!(IRQ_CC, IrqCc);
::bobbin_mcu::irq_type!(IRQ_ADC, IrqAdc);
::bobbin_mcu::irq_type!(IRQ_I2C_EV, IrqI2cEv);
::bobbin_mcu::irq_type!(IRQ_I2C_ER, IrqI2cEr);
::bobbin_mcu::irq_type!(IRQ_CAN_TX, IrqCanTx);
::bobbin_mcu::irq_type!(IRQ_CAN_RX0, IrqCanRx0);
::bobbin_mcu::irq_type!(IRQ_CAN_RX1, IrqCanRx1);
::bobbin_mcu::irq_type!(IRQ_CAN_SCE, IrqCanSce);
::bobbin_mcu::irq_type!(IRQ_USART, IrqUsart);
::bobbin_mcu::irq_type!(IRQ_DMA, IrqDma);

::bobbin_mcu::irq!(::wwdg::Wwdg, ::bobbin_mcu::irq::IrqMain, Irq0);
::bobbin_mcu::irq!(::wwdg::Wwdg, IrqWwdg, Irq0);
::bobbin_mcu::irq!(::exti::ExtiLine0, ::bobbin_mcu::irq::IrqMain, Irq6);
::bobbin_mcu::irq!(::exti::ExtiLine0, IrqExti, Irq6);
::bobbin_mcu::irq!(::exti::ExtiLine1, ::bobbin_mcu::irq::IrqMain, Irq7);
::bobbin_mcu::irq!(::exti::ExtiLine1, IrqExti, Irq7);
::bobbin_mcu::irq!(::exti::ExtiLine2, ::bobbin_mcu::irq::IrqMain, Irq8);
::bobbin_mcu::irq!(::exti::ExtiLine2, IrqExti, Irq8);
::bobbin_mcu::irq!(::exti::ExtiLine3, ::bobbin_mcu::irq::IrqMain, Irq9);
::bobbin_mcu::irq!(::exti::ExtiLine3, IrqExti, Irq9);
::bobbin_mcu::irq!(::exti::ExtiLine4, ::bobbin_mcu::irq::IrqMain, Irq10);
::bobbin_mcu::irq!(::exti::ExtiLine4, IrqExti, Irq10);
::bobbin_mcu::irq!(::exti::ExtiLine5, ::bobbin_mcu::irq::IrqMain, Irq23);
::bobbin_mcu::irq!(::exti::ExtiLine5, IrqExti, Irq23);
::bobbin_mcu::irq!(::exti::ExtiLine6, ::bobbin_mcu::irq::IrqMain, Irq23);
::bobbin_mcu::irq!(::exti::ExtiLine6, IrqExti, Irq23);
::bobbin_mcu::irq!(::exti::ExtiLine7, ::bobbin_mcu::irq::IrqMain, Irq23);
::bobbin_mcu::irq!(::exti::ExtiLine7, IrqExti, Irq23);
::bobbin_mcu::irq!(::exti::ExtiLine8, ::bobbin_mcu::irq::IrqMain, Irq23);
::bobbin_mcu::irq!(::exti::ExtiLine8, IrqExti, Irq23);
::bobbin_mcu::irq!(::exti::ExtiLine9, ::bobbin_mcu::irq::IrqMain, Irq23);
::bobbin_mcu::irq!(::exti::ExtiLine9, IrqExti, Irq23);
::bobbin_mcu::irq!(::exti::ExtiLine10, ::bobbin_mcu::irq::IrqMain, Irq40);
::bobbin_mcu::irq!(::exti::ExtiLine10, IrqExti, Irq40);
::bobbin_mcu::irq!(::exti::ExtiLine11, ::bobbin_mcu::irq::IrqMain, Irq40);
::bobbin_mcu::irq!(::exti::ExtiLine11, IrqExti, Irq40);
::bobbin_mcu::irq!(::exti::ExtiLine12, ::bobbin_mcu::irq::IrqMain, Irq40);
::bobbin_mcu::irq!(::exti::ExtiLine12, IrqExti, Irq40);
::bobbin_mcu::irq!(::exti::ExtiLine13, ::bobbin_mcu::irq::IrqMain, Irq40);
::bobbin_mcu::irq!(::exti::ExtiLine13, IrqExti, Irq40);
::bobbin_mcu::irq!(::exti::ExtiLine14, ::bobbin_mcu::irq::IrqMain, Irq40);
::bobbin_mcu::irq!(::exti::ExtiLine14, IrqExti, Irq40);
::bobbin_mcu::irq!(::exti::ExtiLine15, ::bobbin_mcu::irq::IrqMain, Irq40);
::bobbin_mcu::irq!(::exti::ExtiLine15, IrqExti, Irq40);
::bobbin_mcu::irq!(::exti::ExtiLine16, ::bobbin_mcu::irq::IrqMain, Irq1);
::bobbin_mcu::irq!(::exti::ExtiLine16, IrqExti, Irq1);
::bobbin_mcu::irq!(::exti::ExtiLine17, ::bobbin_mcu::irq::IrqMain, Irq41);
::bobbin_mcu::irq!(::exti::ExtiLine17, IrqExti, Irq41);
::bobbin_mcu::irq!(::exti::ExtiLine18, ::bobbin_mcu::irq::IrqMain, Irq42);
::bobbin_mcu::irq!(::exti::ExtiLine18, IrqExti, Irq42);
::bobbin_mcu::irq!(::exti::ExtiLine19, ::bobbin_mcu::irq::IrqMain, Irq62);
::bobbin_mcu::irq!(::exti::ExtiLine19, IrqExti, Irq62);
::bobbin_mcu::irq!(::exti::ExtiLine20, ::bobbin_mcu::irq::IrqMain, Irq76);
::bobbin_mcu::irq!(::exti::ExtiLine20, IrqExti, Irq76);
::bobbin_mcu::irq!(::exti::ExtiLine21, ::bobbin_mcu::irq::IrqMain, Irq2);
::bobbin_mcu::irq!(::exti::ExtiLine21, IrqExti, Irq2);
::bobbin_mcu::irq!(::exti::ExtiLine22, ::bobbin_mcu::irq::IrqMain, Irq3);
::bobbin_mcu::irq!(::exti::ExtiLine22, IrqExti, Irq3);
::bobbin_mcu::irq!(::tim_bas::Tim6, ::bobbin_mcu::irq::IrqMain, Irq54);
::bobbin_mcu::irq!(::tim_bas::Tim6, IrqTim, Irq54);
::bobbin_mcu::irq!(::tim_bas::Tim7, ::bobbin_mcu::irq::IrqMain, Irq55);
::bobbin_mcu::irq!(::tim_bas::Tim7, IrqTim, Irq55);
::bobbin_mcu::irq!(::tim_gen::Tim2, ::bobbin_mcu::irq::IrqMain, Irq28);
::bobbin_mcu::irq!(::tim_gen::Tim2, IrqTim, Irq28);
::bobbin_mcu::irq!(::tim_gen::Tim3, ::bobbin_mcu::irq::IrqMain, Irq29);
::bobbin_mcu::irq!(::tim_gen::Tim3, IrqTim, Irq29);
::bobbin_mcu::irq!(::tim_gen::Tim4, ::bobbin_mcu::irq::IrqMain, Irq30);
::bobbin_mcu::irq!(::tim_gen::Tim4, IrqTim, Irq30);
::bobbin_mcu::irq!(::tim_gen::Tim5, ::bobbin_mcu::irq::IrqMain, Irq50);
::bobbin_mcu::irq!(::tim_gen::Tim5, IrqTim, Irq50);
::bobbin_mcu::irq!(::tim_gen::Tim9, ::bobbin_mcu::irq::IrqMain, Irq24);
::bobbin_mcu::irq!(::tim_gen::Tim9, IrqTim, Irq24);
::bobbin_mcu::irq!(::tim_gen::Tim10, ::bobbin_mcu::irq::IrqMain, Irq25);
::bobbin_mcu::irq!(::tim_gen::Tim10, IrqTim, Irq25);
::bobbin_mcu::irq!(::tim_gen::Tim11, ::bobbin_mcu::irq::IrqMain, Irq26);
::bobbin_mcu::irq!(::tim_gen::Tim11, IrqTim, Irq26);
::bobbin_mcu::irq!(::tim_gen::Tim12, ::bobbin_mcu::irq::IrqMain, Irq43);
::bobbin_mcu::irq!(::tim_gen::Tim12, IrqTim, Irq43);
::bobbin_mcu::irq!(::tim_gen::Tim13, ::bobbin_mcu::irq::IrqMain, Irq44);
::bobbin_mcu::irq!(::tim_gen::Tim13, IrqTim, Irq44);
::bobbin_mcu::irq!(::tim_gen::Tim14, ::bobbin_mcu::irq::IrqMain, Irq45);
::bobbin_mcu::irq!(::tim_gen::Tim14, IrqTim, Irq45);
::bobbin_mcu::irq!(::tim_adv::Tim1, ::bobbin_mcu::irq::IrqMain, Irq24);
::bobbin_mcu::irq!(::tim_adv::Tim1, IrqBrk, Irq24);
::bobbin_mcu::irq!(::tim_adv::Tim1, IrqUp, Irq25);
::bobbin_mcu::irq!(::tim_adv::Tim1, IrqTrgCom, Irq26);
::bobbin_mcu::irq!(::tim_adv::Tim1, IrqCc, Irq27);
::bobbin_mcu::irq!(::tim_adv::Tim8, ::bobbin_mcu::irq::IrqMain, Irq43);
::bobbin_mcu::irq!(::tim_adv::Tim8, IrqBrk, Irq43);
::bobbin_mcu::irq!(::tim_adv::Tim8, IrqUp, Irq44);
::bobbin_mcu::irq!(::tim_adv::Tim8, IrqTrgCom, Irq45);
::bobbin_mcu::irq!(::tim_adv::Tim8, IrqCc, Irq46);
::bobbin_mcu::irq!(::adc::Adc1, ::bobbin_mcu::irq::IrqMain, Irq18);
::bobbin_mcu::irq!(::adc::Adc1, IrqAdc, Irq18);
::bobbin_mcu::irq!(::adc::Adc2, ::bobbin_mcu::irq::IrqMain, Irq18);
::bobbin_mcu::irq!(::adc::Adc2, IrqAdc, Irq18);
::bobbin_mcu::irq!(::adc::Adc3, ::bobbin_mcu::irq::IrqMain, Irq18);
::bobbin_mcu::irq!(::adc::Adc3, IrqAdc, Irq18);
::bobbin_mcu::irq!(::spi::Spi1, ::bobbin_mcu::irq::IrqMain, Irq35);
::bobbin_mcu::irq!(::spi::Spi1, IrqSpi, Irq35);
::bobbin_mcu::irq!(::spi::Spi2, ::bobbin_mcu::irq::IrqMain, Irq36);
::bobbin_mcu::irq!(::spi::Spi2, IrqSpi, Irq36);
::bobbin_mcu::irq!(::spi::Spi3, ::bobbin_mcu::irq::IrqMain, Irq51);
::bobbin_mcu::irq!(::spi::Spi3, IrqSpi, Irq51);
::bobbin_mcu::irq!(::spi::Spi4, ::bobbin_mcu::irq::IrqMain, Irq84);
::bobbin_mcu::irq!(::spi::Spi4, IrqSpi, Irq84);
::bobbin_mcu::irq!(::spi::Spi5, ::bobbin_mcu::irq::IrqMain, Irq85);
::bobbin_mcu::irq!(::spi::Spi5, IrqSpi, Irq85);
::bobbin_mcu::irq!(::spi::Spi6, ::bobbin_mcu::irq::IrqMain, Irq86);
::bobbin_mcu::irq!(::spi::Spi6, IrqSpi, Irq86);
::bobbin_mcu::irq_type!(IRQ_SPI, IrqSpi);
::bobbin_mcu::irq!(::i2c::I2c1, ::bobbin_mcu::irq::IrqMain, Irq31);
::bobbin_mcu::irq!(::i2c::I2c1, IrqI2cEv, Irq31);
::bobbin_mcu::irq!(::i2c::I2c1, IrqI2cEr, Irq32);
::bobbin_mcu::irq!(::i2c::I2c2, ::bobbin_mcu::irq::IrqMain, Irq33);
::bobbin_mcu::irq!(::i2c::I2c2, IrqI2cEv, Irq33);
::bobbin_mcu::irq!(::i2c::I2c2, IrqI2cEr, Irq34);
::bobbin_mcu::irq!(::i2c::I2c3, ::bobbin_mcu::irq::IrqMain, Irq72);
::bobbin_mcu::irq!(::i2c::I2c3, IrqI2cEv, Irq72);
::bobbin_mcu::irq!(::i2c::I2c3, IrqI2cEr, Irq73);
::bobbin_mcu::irq!(::can::Can1, ::bobbin_mcu::irq::IrqMain, Irq19);
::bobbin_mcu::irq!(::can::Can1, IrqCanTx, Irq19);
::bobbin_mcu::irq!(::can::Can1, IrqCanRx0, Irq20);
::bobbin_mcu::irq!(::can::Can1, IrqCanRx1, Irq21);
::bobbin_mcu::irq!(::can::Can1, IrqCanSce, Irq22);
::bobbin_mcu::irq!(::can::Can2, ::bobbin_mcu::irq::IrqMain, Irq63);
::bobbin_mcu::irq!(::can::Can2, IrqCanTx, Irq63);
::bobbin_mcu::irq!(::can::Can2, IrqCanRx0, Irq64);
::bobbin_mcu::irq!(::can::Can2, IrqCanRx1, Irq65);
::bobbin_mcu::irq!(::can::Can2, IrqCanSce, Irq66);
::bobbin_mcu::irq!(::usart::Usart1, ::bobbin_mcu::irq::IrqMain, Irq37);
::bobbin_mcu::irq!(::usart::Usart1, IrqUsart, Irq37);
::bobbin_mcu::irq!(::usart::Usart2, ::bobbin_mcu::irq::IrqMain, Irq38);
::bobbin_mcu::irq!(::usart::Usart2, IrqUsart, Irq38);
::bobbin_mcu::irq!(::usart::Usart3, ::bobbin_mcu::irq::IrqMain, Irq39);
::bobbin_mcu::irq!(::usart::Usart3, IrqUsart, Irq39);
::bobbin_mcu::irq!(::usart::Uart4, ::bobbin_mcu::irq::IrqMain, Irq52);
::bobbin_mcu::irq!(::usart::Uart4, IrqUsart, Irq52);
::bobbin_mcu::irq!(::usart::Uart5, ::bobbin_mcu::irq::IrqMain, Irq53);
::bobbin_mcu::irq!(::usart::Uart5, IrqUsart, Irq53);
::bobbin_mcu::irq!(::usart::Usart6, ::bobbin_mcu::irq::IrqMain, Irq71);
::bobbin_mcu::irq!(::usart::Usart6, IrqUsart, Irq71);
::bobbin_mcu::irq!(::usart::Uart7, ::bobbin_mcu::irq::IrqMain, Irq82);
::bobbin_mcu::irq!(::usart::Uart7, IrqUsart, Irq82);
::bobbin_mcu::irq!(::usart::Uart8, ::bobbin_mcu::irq::IrqMain, Irq83);
::bobbin_mcu::irq!(::usart::Uart8, IrqUsart, Irq83);
::bobbin_mcu::irq!(::dma::Dma1Stream0, ::bobbin_mcu::irq::IrqMain, Irq11);
::bobbin_mcu::irq!(::dma::Dma1Stream0, IrqDma, Irq11);
::bobbin_mcu::irq!(::dma::Dma1Stream1, ::bobbin_mcu::irq::IrqMain, Irq12);
::bobbin_mcu::irq!(::dma::Dma1Stream1, IrqDma, Irq12);
::bobbin_mcu::irq!(::dma::Dma1Stream2, ::bobbin_mcu::irq::IrqMain, Irq13);
::bobbin_mcu::irq!(::dma::Dma1Stream2, IrqDma, Irq13);
::bobbin_mcu::irq!(::dma::Dma1Stream3, ::bobbin_mcu::irq::IrqMain, Irq14);
::bobbin_mcu::irq!(::dma::Dma1Stream3, IrqDma, Irq14);
::bobbin_mcu::irq!(::dma::Dma1Stream4, ::bobbin_mcu::irq::IrqMain, Irq15);
::bobbin_mcu::irq!(::dma::Dma1Stream4, IrqDma, Irq15);
::bobbin_mcu::irq!(::dma::Dma1Stream5, ::bobbin_mcu::irq::IrqMain, Irq16);
::bobbin_mcu::irq!(::dma::Dma1Stream5, IrqDma, Irq16);
::bobbin_mcu::irq!(::dma::Dma1Stream6, ::bobbin_mcu::irq::IrqMain, Irq17);
::bobbin_mcu::irq!(::dma::Dma1Stream6, IrqDma, Irq17);
::bobbin_mcu::irq!(::dma::Dma1Stream7, ::bobbin_mcu::irq::IrqMain, Irq47);
::bobbin_mcu::irq!(::dma::Dma1Stream7, IrqDma, Irq47);
::bobbin_mcu::irq!(::dma::Dma2Stream0, ::bobbin_mcu::irq::IrqMain, Irq56);
::bobbin_mcu::irq!(::dma::Dma2Stream0, IrqDma, Irq56);
::bobbin_mcu::irq!(::dma::Dma2Stream1, ::bobbin_mcu::irq::IrqMain, Irq57);
::bobbin_mcu::irq!(::dma::Dma2Stream1, IrqDma, Irq57);
::bobbin_mcu::irq!(::dma::Dma2Stream2, ::bobbin_mcu::irq::IrqMain, Irq58);
::bobbin_mcu::irq!(::dma::Dma2Stream2, IrqDma, Irq58);
::bobbin_mcu::irq!(::dma::Dma2Stream3, ::bobbin_mcu::irq::IrqMain, Irq59);
::bobbin_mcu::irq!(::dma::Dma2Stream3, IrqDma, Irq59);
::bobbin_mcu::irq!(::dma::Dma2Stream4, ::bobbin_mcu::irq::IrqMain, Irq60);
::bobbin_mcu::irq!(::dma::Dma2Stream4, IrqDma, Irq60);
::bobbin_mcu::irq!(::dma::Dma2Stream5, ::bobbin_mcu::irq::IrqMain, Irq68);
::bobbin_mcu::irq!(::dma::Dma2Stream5, IrqDma, Irq68);
::bobbin_mcu::irq!(::dma::Dma2Stream6, ::bobbin_mcu::irq::IrqMain, Irq69);
::bobbin_mcu::irq!(::dma::Dma2Stream6, IrqDma, Irq69);
::bobbin_mcu::irq!(::dma::Dma2Stream7, ::bobbin_mcu::irq::IrqMain, Irq70);
::bobbin_mcu::irq!(::dma::Dma2Stream7, IrqDma, Irq70);
::bobbin_mcu::irq!(::rcc::Rcc, ::bobbin_mcu::irq::IrqMain, Irq5);
::bobbin_mcu::irq!(::pwr::Pwr, ::bobbin_mcu::irq::IrqMain, Irq1);
::bobbin_mcu::irq!(::ethernet_mac::EthernetMac, ::bobbin_mcu::irq::IrqMain, Irq61);
