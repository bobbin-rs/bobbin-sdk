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
irq_number!(IRQ_86, Irq86, 86);
irq_number!(IRQ_87, Irq87, 87);
irq_number!(IRQ_88, Irq88, 88);
irq_number!(IRQ_89, Irq89, 89);
irq_number!(IRQ_90, Irq90, 90);
irq_number!(IRQ_91, Irq91, 91);
irq_number!(IRQ_92, Irq92, 92);
irq_number!(IRQ_93, Irq93, 93);
irq_number!(IRQ_94, Irq94, 94);
irq_number!(IRQ_95, Irq95, 95);
irq_number!(IRQ_96, Irq96, 96);
irq_number!(IRQ_97, Irq97, 97);
irq_number!(IRQ_98, Irq98, 98);
irq_number!(IRQ_99, Irq99, 99);
irq_number!(IRQ_100, Irq100, 100);
irq_number!(IRQ_101, Irq101, 101);
irq_number!(IRQ_102, Irq102, 102);
irq_number!(IRQ_103, Irq103, 103);
irq_number!(IRQ_104, Irq104, 104);
irq_number!(IRQ_105, Irq105, 105);
irq_number!(IRQ_106, Irq106, 106);
irq_number!(IRQ_107, Irq107, 107);
irq_number!(IRQ_108, Irq108, 108);
irq_number!(IRQ_109, Irq109, 109);
irq_number!(IRQ_110, Irq110, 110);
irq_number!(IRQ_111, Irq111, 111);
irq_number!(IRQ_112, Irq112, 112);
irq_number!(IRQ_113, Irq113, 113);
irq_number!(IRQ_114, Irq114, 114);
irq_number!(IRQ_115, Irq115, 115);
irq_number!(IRQ_116, Irq116, 116);
irq_number!(IRQ_117, Irq117, 117);
irq_number!(IRQ_118, Irq118, 118);
irq_number!(IRQ_119, Irq119, 119);
irq_number!(IRQ_120, Irq120, 120);
irq_number!(IRQ_121, Irq121, 121);
irq_number!(IRQ_122, Irq122, 122);
irq_number!(IRQ_123, Irq123, 123);
irq_number!(IRQ_124, Irq124, 124);
irq_number!(IRQ_125, Irq125, 125);
irq_number!(IRQ_126, Irq126, 126);
irq_number!(IRQ_127, Irq127, 127);
irq_number!(IRQ_128, Irq128, 128);
irq_number!(IRQ_129, Irq129, 129);
irq_number!(IRQ_130, Irq130, 130);
irq_number!(IRQ_131, Irq131, 131);
irq_number!(IRQ_132, Irq132, 132);
irq_number!(IRQ_133, Irq133, 133);
irq_number!(IRQ_134, Irq134, 134);
irq_number!(IRQ_135, Irq135, 135);
irq_number!(IRQ_136, Irq136, 136);
irq_number!(IRQ_137, Irq137, 137);
irq_number!(IRQ_138, Irq138, 138);
irq_number!(IRQ_139, Irq139, 139);
irq_number!(IRQ_140, Irq140, 140);
irq_number!(IRQ_141, Irq141, 141);
irq_number!(IRQ_142, Irq142, 142);
irq_number!(IRQ_143, Irq143, 143);
irq_number!(IRQ_144, Irq144, 144);
irq_number!(IRQ_145, Irq145, 145);
irq_number!(IRQ_146, Irq146, 146);

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

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_86_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_87_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_88_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_89_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_90_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_91_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_92_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_93_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_94_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_95_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_96_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_97_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_98_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_99_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_100_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_101_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_102_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_103_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_104_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_105_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_106_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_107_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_108_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_109_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_110_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_111_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_112_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_113_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_114_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_115_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_116_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_117_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_118_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_119_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_120_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_121_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_122_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_123_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_124_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_125_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_126_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_127_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_128_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_129_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_130_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_131_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_132_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_133_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_134_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_135_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_136_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_137_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_138_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_139_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_140_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_141_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_142_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_143_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_144_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_145_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn IRQ_146_HANDLER() {
    unsafe {
        asm!("b DEFAULT_HANDLER" :::: "volatile");
        ::core::intrinsics::unreachable();
    }
}

#[cfg_attr(target_os="none", link_section=".vector_table.interrupts")]
#[no_mangle]
pub static mut INTERRUPTS: [Option<Handler>; 147] = [
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
    Some(IRQ_91_HANDLER),
    Some(IRQ_92_HANDLER),
    Some(IRQ_93_HANDLER),
    Some(IRQ_94_HANDLER),
    Some(IRQ_95_HANDLER),
    Some(IRQ_96_HANDLER),
    Some(IRQ_97_HANDLER),
    Some(IRQ_98_HANDLER),
    Some(IRQ_99_HANDLER),
    Some(IRQ_100_HANDLER),
    Some(IRQ_101_HANDLER),
    Some(IRQ_102_HANDLER),
    Some(IRQ_103_HANDLER),
    Some(IRQ_104_HANDLER),
    Some(IRQ_105_HANDLER),
    Some(IRQ_106_HANDLER),
    Some(IRQ_107_HANDLER),
    Some(IRQ_108_HANDLER),
    Some(IRQ_109_HANDLER),
    Some(IRQ_110_HANDLER),
    Some(IRQ_111_HANDLER),
    Some(IRQ_112_HANDLER),
    Some(IRQ_113_HANDLER),
    Some(IRQ_114_HANDLER),
    Some(IRQ_115_HANDLER),
    Some(IRQ_116_HANDLER),
    Some(IRQ_117_HANDLER),
    Some(IRQ_118_HANDLER),
    Some(IRQ_119_HANDLER),
    Some(IRQ_120_HANDLER),
    Some(IRQ_121_HANDLER),
    Some(IRQ_122_HANDLER),
    Some(IRQ_123_HANDLER),
    Some(IRQ_124_HANDLER),
    Some(IRQ_125_HANDLER),
    Some(IRQ_126_HANDLER),
    Some(IRQ_127_HANDLER),
    Some(IRQ_128_HANDLER),
    Some(IRQ_129_HANDLER),
    Some(IRQ_130_HANDLER),
    Some(IRQ_131_HANDLER),
    Some(IRQ_132_HANDLER),
    Some(IRQ_133_HANDLER),
    Some(IRQ_134_HANDLER),
    Some(IRQ_135_HANDLER),
    Some(IRQ_136_HANDLER),
    Some(IRQ_137_HANDLER),
    Some(IRQ_138_HANDLER),
    Some(IRQ_139_HANDLER),
    Some(IRQ_140_HANDLER),
    Some(IRQ_141_HANDLER),
    Some(IRQ_142_HANDLER),
    Some(IRQ_143_HANDLER),
    Some(IRQ_144_HANDLER),
    Some(IRQ_145_HANDLER),
    Some(IRQ_146_HANDLER),
];

irq_type!(IRQ_DMA_ERROR, IrqDmaError);
irq_type!(IRQ_DMA, IrqDma);
irq_type!(IRQ_FTM_FAULT, IrqFtmFault);
irq_type!(IRQ_FTM_OVERFLOW, IrqFtmOverflow);
irq_type!(IRQ_FTM, IrqFtm);
irq_type!(IRQ_LPIT, IrqLpit);
irq_type!(IRQ_CAN, IrqCan);
irq_type!(IRQ_CAN_ERROR, IrqCanError);
irq_type!(IRQ_CAN_WAKE_UP, IrqCanWakeUp);
irq_type!(IRQ_CAN_0_15, IrqCan015);
irq_type!(IRQ_CAN_16_31, IrqCan1631);
irq_type!(IRQ_PORT, IrqPort);
irq_type!(IRQ_LPUART_RXTX, IrqLpuartRxtx);
irq_type!(IRQ_LPI2C_MASTER, IrqLpi2cMaster);
irq_type!(IRQ_LPI2C_SLAVE, IrqLpi2cSlave);
irq_type!(IRQ_LPSPI, IrqLpspi);
irq_type!(IRQ_ADC, IrqAdc);
irq_type!(IRQ_RTC, IrqRtc);
irq_type!(IRQ_RTC_SECONDS, IrqRtcSeconds);

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
irq!(::ftm::Ftm0, IrqFtmFault, Irq103);
irq!(::ftm::Ftm0, IrqFtmOverflow, Irq104);
irq!(::ftm::Ftm0Ch0, IrqFtm, Irq99);
irq!(::ftm::Ftm0Ch1, IrqFtm, Irq99);
irq!(::ftm::Ftm0Ch2, IrqFtm, Irq100);
irq!(::ftm::Ftm0Ch3, IrqFtm, Irq100);
irq!(::ftm::Ftm0Ch4, IrqFtm, Irq101);
irq!(::ftm::Ftm0Ch5, IrqFtm, Irq101);
irq!(::ftm::Ftm0Ch6, IrqFtm, Irq102);
irq!(::ftm::Ftm0Ch7, IrqFtm, Irq102);
irq!(::ftm::Ftm1, IrqFtmFault, Irq109);
irq!(::ftm::Ftm1, IrqFtmOverflow, Irq110);
irq!(::ftm::Ftm1Ch0, IrqFtm, Irq105);
irq!(::ftm::Ftm1Ch1, IrqFtm, Irq105);
irq!(::ftm::Ftm1Ch2, IrqFtm, Irq106);
irq!(::ftm::Ftm1Ch3, IrqFtm, Irq106);
irq!(::ftm::Ftm1Ch4, IrqFtm, Irq107);
irq!(::ftm::Ftm1Ch5, IrqFtm, Irq107);
irq!(::ftm::Ftm1Ch6, IrqFtm, Irq108);
irq!(::ftm::Ftm1Ch7, IrqFtm, Irq108);
irq!(::ftm::Ftm2, IrqFtmFault, Irq115);
irq!(::ftm::Ftm2, IrqFtmOverflow, Irq116);
irq!(::ftm::Ftm2Ch0, IrqFtm, Irq111);
irq!(::ftm::Ftm2Ch1, IrqFtm, Irq111);
irq!(::ftm::Ftm2Ch2, IrqFtm, Irq112);
irq!(::ftm::Ftm2Ch3, IrqFtm, Irq112);
irq!(::ftm::Ftm2Ch4, IrqFtm, Irq113);
irq!(::ftm::Ftm2Ch5, IrqFtm, Irq113);
irq!(::ftm::Ftm2Ch6, IrqFtm, Irq114);
irq!(::ftm::Ftm2Ch7, IrqFtm, Irq114);
irq!(::ftm::Ftm3, IrqFtmFault, Irq121);
irq!(::ftm::Ftm3, IrqFtmOverflow, Irq122);
irq!(::ftm::Ftm3Ch0, IrqFtm, Irq117);
irq!(::ftm::Ftm3Ch1, IrqFtm, Irq117);
irq!(::ftm::Ftm3Ch2, IrqFtm, Irq118);
irq!(::ftm::Ftm3Ch3, IrqFtm, Irq118);
irq!(::ftm::Ftm3Ch4, IrqFtm, Irq119);
irq!(::ftm::Ftm3Ch5, IrqFtm, Irq119);
irq!(::ftm::Ftm3Ch6, IrqFtm, Irq120);
irq!(::ftm::Ftm3Ch7, IrqFtm, Irq120);
irq!(::lpit::Lpit0Ch0, IrqLpit, Irq48);
irq!(::lpit::Lpit0Ch1, IrqLpit, Irq49);
irq!(::lpit::Lpit0Ch2, IrqLpit, Irq50);
irq!(::lpit::Lpit0Ch3, IrqLpit, Irq51);
irq!(::lptmr::Lptmr0, IrqLptmr, Irq58);
irq_type!(IRQ_LPTMR, IrqLptmr);
irq!(::flexcan::Can0, IrqCan, Irq78);
irq!(::flexcan::Can0, IrqCanError, Irq79);
irq!(::flexcan::Can0, IrqCanWakeUp, Irq80);
irq!(::flexcan::Can0, IrqCan015, Irq81);
irq!(::flexcan::Can0, IrqCan1631, Irq82);
irq!(::flexcan::Can1, IrqCan, Irq85);
irq!(::flexcan::Can1, IrqCanError, Irq86);
irq!(::flexcan::Can1, IrqCan015, Irq88);
irq!(::flexcan::Can2, IrqCan, Irq92);
irq!(::flexcan::Can2, IrqCanError, Irq93);
irq!(::flexcan::Can2, IrqCan015, Irq95);
irq!(::port::Porta, IrqPort, Irq59);
irq!(::port::Portb, IrqPort, Irq60);
irq!(::port::Portc, IrqPort, Irq61);
irq!(::port::Portd, IrqPort, Irq62);
irq!(::port::Porte, IrqPort, Irq63);
irq_type!(IRQ_PORT, IrqPort);
irq!(::lpuart::Lpuart0, IrqLpuartRxtx, Irq31);
irq!(::lpuart::Lpuart1, IrqLpuartRxtx, Irq33);
irq!(::lpuart::Lpuart2, IrqLpuartRxtx, Irq35);
irq!(::lpi2c::Lpi2c0, IrqLpi2cMaster, Irq24);
irq!(::lpi2c::Lpi2c0, IrqLpi2cSlave, Irq25);
irq!(::lpspi::Lpspi0, IrqLpspi, Irq26);
irq!(::lpspi::Lpspi1, IrqLpspi, Irq27);
irq!(::lpspi::Lpspi2, IrqLpspi, Irq28);
irq!(::adc::Adc0, IrqAdc, Irq39);
irq!(::adc::Adc1, IrqAdc, Irq40);
irq!(::rtc::Rtc, Rtc, Irq46);
irq!(::rtc::Rtc, RtcSeconds, Irq47);
