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
irq_number!(IRQ_48, Irq48, 48);
irq_number!(IRQ_49, Irq49, 49);
irq_number!(IRQ_50, Irq50, 50);
irq_number!(IRQ_51, Irq51, 51);
irq_number!(IRQ_52, Irq52, 52);
irq_number!(IRQ_53, Irq53, 53);
irq_number!(IRQ_54, Irq54, 54);
irq_number!(IRQ_55, Irq55, 55);
irq_number!(IRQ_56, Irq56, 56);
irq_number!(IRQ_57, Irq57, 57);
irq_number!(IRQ_58, Irq58, 58);
irq_number!(IRQ_59, Irq59, 59);
irq_number!(IRQ_60, Irq60, 60);
irq_number!(IRQ_61, Irq61, 61);
irq_number!(IRQ_62, Irq62, 62);
irq_number!(IRQ_63, Irq63, 63);
irq_number!(IRQ_64, Irq64, 64);
irq_number!(IRQ_65, Irq65, 65);
irq_number!(IRQ_66, Irq66, 66);
irq_number!(IRQ_67, Irq67, 67);
irq_number!(IRQ_68, Irq68, 68);
irq_number!(IRQ_69, Irq69, 69);
irq_number!(IRQ_70, Irq70, 70);
irq_number!(IRQ_71, Irq71, 71);
irq_number!(IRQ_72, Irq72, 72);
irq_number!(IRQ_73, Irq73, 73);
irq_number!(IRQ_74, Irq74, 74);
irq_number!(IRQ_75, Irq75, 75);
irq_number!(IRQ_76, Irq76, 76);
irq_number!(IRQ_77, Irq77, 77);
irq_number!(IRQ_78, Irq78, 78);
irq_number!(IRQ_79, Irq79, 79);
irq_number!(IRQ_80, Irq80, 80);
irq_number!(IRQ_81, Irq81, 81);
irq_number!(IRQ_82, Irq82, 82);
irq_number!(IRQ_83, Irq83, 83);
irq_number!(IRQ_84, Irq84, 84);

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

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_48_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_49_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_50_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_51_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_52_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_53_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_54_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_55_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_56_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_57_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_58_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_59_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_60_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_61_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_62_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_63_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_64_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_65_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_66_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_67_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_68_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_69_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_70_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_71_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_72_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_73_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_74_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_75_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_76_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_77_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_78_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_79_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_80_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_81_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_82_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_83_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_84_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[cfg_attr(target_os="none", link_section=".vector_table.interrupts")]
#[no_mangle]
pub static mut INTERRUPTS: [Option<Handler>; 85] = [
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
];

irq_type!(IRQ_DMA, IrqDma);
irq_type!(IRQ_I2C_EV, IrqI2cEv);
irq_type!(IRQ_I2C_ER, IrqI2cEr);
irq_type!(IRQ_ADC, IrqAdc);
irq_type!(IRQ_TIM, IrqTim);
irq_type!(IRQ_TIM_BRK, IrqTimBrk);
irq_type!(IRQ_TIM_UP, IrqTimUp);
irq_type!(IRQ_TIM_TRG_COM, IrqTimTrgCom);
irq_type!(IRQ_TIM_CC, IrqTimCc);
irq_type!(IRQ_LPTIM, IrqLptim);
irq_type!(IRQ_USART, IrqUsart);
irq_type!(IRQ_LPUART, IrqLpuart);
irq_type!(IRQ_SPI, IrqSpi);
irq_type!(IRQ_EXTI0, IrqExti0);
irq_type!(IRQ_EXTI1, IrqExti1);
irq_type!(IRQ_EXTI2, IrqExti2);
irq_type!(IRQ_EXTI3, IrqExti3);
irq_type!(IRQ_EXTI4, IrqExti4);
irq_type!(IRQ_EXTI9_5, IrqExti95);
irq_type!(IRQ_EXTI15_10, IrqExti1510);
irq_type!(IRQ_CAN_TX, IrqCanTx);
irq_type!(IRQ_CAN_RX0, IrqCanRx0);
irq_type!(IRQ_CAN_RX1, IrqCanRx1);
irq_type!(IRQ_CAN_SCE, IrqCanSce);
irq_type!(IRQ_RTC_TAMP, IrqRtcTamp);
irq_type!(IRQ_RTC_WKUP, IrqRtcWkup);
irq_type!(IRQ_RTC_ALARM, IrqRtcAlarm);
irq_type!(IRQ_DFSDM_FLT0, IrqDfsdmFlt0);
irq_type!(IRQ_DFSDM_FLT1, IrqDfsdmFlt1);

irq!(::dac::Dac1, IrqDac, Irq54);
irq_type!(IRQ_DAC, IrqDac);
irq!(::dma::Dma1Ch1, IrqDma, Irq11);
irq!(::dma::Dma1Ch2, IrqDma, Irq12);
irq!(::dma::Dma1Ch3, IrqDma, Irq13);
irq!(::dma::Dma1Ch4, IrqDma, Irq14);
irq!(::dma::Dma1Ch5, IrqDma, Irq15);
irq!(::dma::Dma1Ch6, IrqDma, Irq16);
irq!(::dma::Dma1Ch7, IrqDma, Irq17);
irq!(::dma::Dma2Ch1, IrqDma, Irq56);
irq!(::dma::Dma2Ch2, IrqDma, Irq57);
irq!(::dma::Dma2Ch3, IrqDma, Irq58);
irq!(::dma::Dma2Ch4, IrqDma, Irq59);
irq!(::dma::Dma2Ch5, IrqDma, Irq60);
irq!(::dma::Dma2Ch6, IrqDma, Irq68);
irq!(::dma::Dma2Ch7, IrqDma, Irq69);
irq!(::lcd::Lcd, IrqLcd, Irq78);
irq_type!(IRQ_LCD, IrqLcd);
irq!(::tsc::Tsc, IrqTsc, Irq77);
irq_type!(IRQ_TSC, IrqTsc);
irq!(::wwdg::Wwdg, IrqWwdg, Irq0);
irq_type!(IRQ_WWDG, IrqWwdg);
irq!(::comp::Comp, IrqComp, Irq64);
irq_type!(IRQ_COMP, IrqComp);
irq!(::i2c::I2c1, IrqI2cEv, Irq31);
irq!(::i2c::I2c1, IrqI2cEr, Irq32);
irq!(::i2c::I2c2, IrqI2cEv, Irq33);
irq!(::i2c::I2c2, IrqI2cEr, Irq34);
irq!(::i2c::I2c3, IrqI2cEv, Irq72);
irq!(::i2c::I2c3, IrqI2cEr, Irq73);
irq!(::i2c::I2c4, IrqI2cEv, Irq83);
irq!(::i2c::I2c4, IrqI2cEr, Irq84);
irq!(::flash::Flash, IrqFlash, Irq4);
irq_type!(IRQ_FLASH, IrqFlash);
irq!(::rng::Rng, IrqRng, Irq80);
irq_type!(IRQ_RNG, IrqRng);
irq!(::aes::Aes, IrqAes, Irq79);
irq_type!(IRQ_AES, IrqAes);
irq!(::adc::Adc, IrqAdc, Irq18);
irq!(::sai::Sai1, IrqSai, Irq74);
irq_type!(IRQ_SAI, IrqSai);
irq!(::tim_gen::Tim2, IrqTim, Irq28);
irq!(::tim_gen::Tim3, IrqTim, Irq29);
irq!(::tim_gen::Tim15, IrqTim, Irq24);
irq!(::tim_gen::Tim16, IrqTim, Irq25);
irq!(::tim_adv::Tim1, IrqTimBrk, Irq24);
irq!(::tim_adv::Tim1, IrqTimUp, Irq25);
irq!(::tim_adv::Tim1, IrqTimTrgCom, Irq26);
irq!(::tim_adv::Tim1, IrqTimCc, Irq27);
irq!(::tim_bas::Tim6, IrqTim, Irq54);
irq!(::tim_bas::Tim7, IrqTim, Irq55);
irq!(::lptim::Lptim1, IrqLptim, Irq65);
irq!(::lptim::Lptim2, IrqLptim, Irq66);
irq!(::usart::Usart1, IrqUsart, Irq37);
irq!(::usart::Usart2, IrqUsart, Irq38);
irq!(::usart::Usart3, IrqUsart, Irq39);
irq!(::usart::Uart4, IrqUsart, Irq52);
irq!(::lpuart::Lpuart1, IrqLpuart, Irq70);
irq!(::spi::Spi1, IrqSpi, Irq35);
irq!(::spi::Spi2, IrqSpi, Irq36);
irq!(::spi::Spi3, IrqSpi, Irq51);
irq!(::sdio::Sdmmc, IrqSdio, Irq49);
irq_type!(IRQ_SDIO, IrqSdio);
irq!(::exti::Exti, IrqExti, Irq1);
irq!(::exti::Exti, IrqExti0, Irq6);
irq!(::exti::Exti, IrqExti1, Irq7);
irq!(::exti::Exti, IrqExti2, Irq8);
irq!(::exti::Exti, IrqExti3, Irq9);
irq!(::exti::Exti, IrqExti4, Irq10);
irq!(::exti::Exti, IrqExti95, Irq23);
irq!(::exti::Exti, IrqExti1510, Irq40);
irq_type!(IRQ_EXTI, IrqExti);
irq!(::vref::Vrefbuf, IrqVref, Irq81);
irq_type!(IRQ_VREF, IrqVref);
irq!(::can::Can1, IrqCanTx, Irq19);
irq!(::can::Can1, IrqCanRx0, Irq20);
irq!(::can::Can1, IrqCanRx1, Irq21);
irq!(::can::Can1, IrqCanSce, Irq22);
irq!(::rtc::Rtc, IrqRtcTamp, Irq2);
irq!(::rtc::Rtc, IrqRtcWkup, Irq3);
irq!(::rtc::Rtc, IrqRtcAlarm, Irq41);
irq!(::swpmi::Swpmi1, IrqSwpmi, Irq76);
irq_type!(IRQ_SWPMI, IrqSwpmi);
irq!(::crs::Crs, IrqCrs, Irq82);
irq_type!(IRQ_CRS, IrqCrs);
irq!(::usb::UsbFs, IrqUsb, Irq67);
irq_type!(IRQ_USB, IrqUsb);
irq!(::dfsdm::Dfsdm, IrqDfsdmFlt0, Irq61);
irq!(::dfsdm::Dfsdm, IrqDfsdmFlt1, Irq62);
irq!(::quadspi::Quadspi, IrqQuadspi, Irq71);
irq_type!(IRQ_QUADSPI, IrqQuadspi);
