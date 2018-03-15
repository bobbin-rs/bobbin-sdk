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
irq_number!(IRQ_85, Irq85, 85);

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

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_85_HANDLER() {
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

irq_type!(IRQ_WDOG, IrqWdog);
irq_type!(IRQ_DMA_ERROR, IrqDmaError);
irq_type!(IRQ_DMA, IrqDma);
irq_type!(IRQ_FTM, IrqFtm);
irq_type!(IRQ_PIT, IrqPit);
irq_type!(IRQ_LPTMR, IrqLptmr);
irq_type!(IRQ_SPI, IrqSpi);
irq_type!(IRQ_I2C, IrqI2c);
irq_type!(IRQ_UART_LON, IrqUartLon);
irq_type!(IRQ_UART, IrqUart);
irq_type!(IRQ_UART_ERR, IrqUartErr);
irq_type!(IRQ_USB, IrqUsb);
irq_type!(IRQ_PORT, IrqPort);
irq_type!(IRQ_ADC, IrqAdc);

irq!(::wdog::Wdog, IrqWdog, Irq22);
irq!(::edma::Dma, IrqDmaError, Irq16);
irq!(::edma::Dma0, IrqDma, Irq0);
irq!(::edma::Dma1, IrqDma, Irq1);
irq!(::edma::Dma2, IrqDma, Irq2);
irq!(::edma::Dma3, IrqDma, Irq3);
irq!(::edma::Dma4, IrqDma, Irq4);
irq!(::edma::Dma5, IrqDma, Irq5);
irq!(::edma::Dma6, IrqDma, Irq6);
irq!(::edma::Dma7, IrqDma, Irq7);
irq!(::edma::Dma8, IrqDma, Irq8);
irq!(::edma::Dma9, IrqDma, Irq9);
irq!(::edma::Dma10, IrqDma, Irq10);
irq!(::edma::Dma11, IrqDma, Irq11);
irq!(::edma::Dma12, IrqDma, Irq12);
irq!(::edma::Dma13, IrqDma, Irq13);
irq!(::edma::Dma14, IrqDma, Irq14);
irq!(::edma::Dma15, IrqDma, Irq15);
irq!(::ftm::Ftm0, IrqFtm, Irq42);
irq!(::ftm::Ftm1, IrqFtm, Irq43);
irq!(::ftm::Ftm2, IrqFtm, Irq44);
irq!(::pit::PitCh0, IrqPit, Irq48);
irq!(::pit::PitCh1, IrqPit, Irq49);
irq!(::pit::PitCh2, IrqPit, Irq50);
irq!(::pit::PitCh3, IrqPit, Irq51);
irq!(::lptmr::Lptmr0, IrqLptmr, Irq58);
irq!(::spi::Spi0, IrqSpi, Irq26);
irq!(::spi::Spi1, IrqSpi, Irq27);
irq!(::spi::Spi2, IrqSpi, Irq65);
irq!(::i2c::I2c0, IrqI2c, Irq24);
irq!(::i2c::I2c1, IrqI2c, Irq25);
irq!(::uart::Uart0, IrqUartLon, Irq30);
irq!(::uart::Uart0, IrqUart, Irq31);
irq!(::uart::Uart0, IrqUartErr, Irq32);
irq!(::uart::Uart1, IrqUart, Irq33);
irq!(::uart::Uart1, IrqUartErr, Irq34);
irq!(::uart::Uart2, IrqUart, Irq35);
irq!(::uart::Uart2, IrqUartErr, Irq36);
irq!(::uart::Uart3, IrqUart, Irq37);
irq!(::uart::Uart3, IrqUartErr, Irq38);
irq!(::uart::Uart4, IrqUart, Irq66);
irq!(::uart::Uart4, IrqUartErr, Irq67);
irq!(::uart::Uart5, IrqUart, Irq68);
irq!(::uart::Uart5, IrqUartErr, Irq69);
irq!(::usb::Usb0, IrqUsb, Irq53);
irq!(::flexcan::Can0, IrqFlexcan, Irq75);
irq!(::flexcan::Can0, IrqFlexcan, Irq76);
irq!(::flexcan::Can0, IrqFlexcan, Irq77);
irq!(::flexcan::Can0, IrqFlexcan, Irq78);
irq!(::flexcan::Can0, IrqFlexcan, Irq79);
irq!(::flexcan::Can0, IrqFlexcan, Irq80);
irq_type!(IRQ_FLEXCAN, IrqFlexcan);
irq!(::port::Porta, IrqPort, Irq59);
irq!(::port::Portb, IrqPort, Irq60);
irq!(::port::Portc, IrqPort, Irq61);
irq!(::port::Portd, IrqPort, Irq62);
irq!(::port::Porte, IrqPort, Irq63);
irq!(::adc::Adc0, IrqAdc, Irq39);
irq!(::adc::Adc1, IrqAdc, Irq73);
