pub use ::chip::scb::*;

pub struct CpuId {
    pub implementer: u8,
    pub variant: u8,
    pub partno: u16,
    pub revision: u8,
}

// ACR

pub fn disfold() -> bool {
    unsafe {
        SCB.actlr().disfold() != 0 
    }
}

pub fn set_disfold(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_actlr(|r| r.set_disfold(value))
    }
}

pub fn disdefwbuf() -> bool {
    unsafe {
        SCB.actlr().disdefwbuf() != 0
    }
}

pub fn set_disdefwbuf(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_actlr(|r| r.set_disdefwbuf(value))
    }
}

pub fn dismcycint() -> bool {
    unsafe {
        SCB.actlr().dismcycint() != 0
    }
}

pub fn set_dismcycint(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_actlr(|r| r.set_dismcycint(value))
    }
}

// CPUID

pub fn cpuid() -> CpuId {
    unsafe {
        let c = SCB.cpuid();
        CpuId {
            implementer: c.implementer() as u8,
            variant: c.variant() as u8,
            partno: c.partno() as u16,
            revision: c.revision() as u8,
        }
    }
}

// ICSR

pub fn nmipend() -> bool {
    unsafe {
        SCB.icsr().nmipendset() != 0
    }
}

pub fn pendsv() -> bool {
    unsafe {
        SCB.icsr().pendsvset() != 0
    }    
}

pub fn set_pendsv() {
    unsafe {
        SCB.set_icsr(Icsr(0).set_pendsvset(1))
    }
}

pub fn clear_pendsv() {
    unsafe {
        SCB.set_icsr(Icsr(0).set_pendsvclr(1));
    }
}

pub fn pendst() -> bool {
    unsafe {
        SCB.icsr().pendstset() != 0
    }        
}

pub fn set_pendst() {
    unsafe {
        SCB.set_icsr(Icsr(0).set_pendstset(1))
    }
}

pub fn clear_pendst() {
    unsafe {
        SCB.set_icsr(Icsr(0).set_pendstclr(1));
    }
}

pub fn isr_pending() -> bool {
    unsafe {
        SCB.icsr().isrpending() != 0
    }
}

pub fn vectpending() -> u32 {
    unsafe {
        SCB.icsr().vectpending()
    }
}

pub fn rettobase() -> bool {
    unsafe {
        SCB.icsr().rettobase() != 0
    }
}

pub fn vectactive() -> u32 {
    unsafe {
        SCB.icsr().vectactive()
    }
}

// VTOR

pub fn tbloff() -> u32 {
    unsafe {
        SCB.vtor().tbloff()
    }
}

pub fn set_tbloff(value: u32) {
    unsafe {
        SCB.set_vtor(Vtor(0).set_tbloff(value))
    }
}

// AIRCR

pub fn vectkey() -> u16 {
    unsafe {
        SCB.aircr().vectkey() as u16
    }
}

pub fn set_vectkey(value: u16) {
    unsafe {
        SCB.with_aircr(|r| r.set_vectkey(value as u32))
    }
}

pub fn endianness() -> bool {
    unsafe {
        SCB.aircr().endianness() != 0
    }
}

pub fn prigroup() -> u8 {
    unsafe {
        SCB.aircr().prigroup() as u8
    }
}

pub fn set_prigroup(value: u8) {
    unsafe {
        SCB.with_aircr(|r| r.set_prigroup(value as u32))
    }
}

pub fn set_sysresetreq(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_aircr(|r| r.set_sysresetreq(value))
    }
}

// SCR

pub fn sevonpend() -> bool {
    unsafe {
        SCB.scr().sevonpend() != 0
    }
}

pub fn set_sevonpend(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_scr(|r| r.set_sevonpend(value))
    }
}

pub fn sleepdeep() -> bool {
    unsafe {
        SCB.scr().sleepdeep() != 0
    }
}

pub fn set_sleepdeep(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_scr(|r| r.set_sleepdeep(value))
    }
}

pub fn sleeponexit() -> bool {
    unsafe {
        SCB.scr().sleeponexit() != 0
    }
}

pub fn set_sleeponexit(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_scr(|r| r.set_sleeponexit(value))
    }
}

// CCR

pub fn stkalign() -> bool {
    unsafe {
        SCB.ccr().stkalign() != 0
    }
}

pub fn set_stkalign(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_ccr(|r| r.set_stkalign(value))
    }
}

pub fn bfhfnmign() -> bool {
    unsafe {
        SCB.ccr().bfhfnmign() != 0
    }
}

pub fn set_bfhfnmign(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_ccr(|r| r.set_bfhfnmign(value))
    }
}


pub fn div_0_trp() -> bool {
    unsafe {
        SCB.ccr().div_0_trp() != 0
    }
}

pub fn set_div_0_trp(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_ccr(|r| r.set_div_0_trp(value))
    }
}


pub fn unalign_trp() -> bool {
    unsafe {
        SCB.ccr().unalign_trp() != 0
    }
}

pub fn set_unalign_trp(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_ccr(|r| r.set_unalign_trp(value))
    }
}

pub fn usersetmpend() -> bool {
    unsafe {
        SCB.ccr().usersetmpend() != 0
    }
}

pub fn set_usersetmpend(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_ccr(|r| r.set_usersetmpend(value))
    }
}

pub fn nonbasethrdena() -> bool {
    unsafe {
        SCB.ccr().nonbasethrdena() != 0
    }
}

pub fn set_nonbasethrdena(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_ccr(|r| r.set_nonbasethrdena(value))
    }
}

pub fn pri_4() -> u8 {
    unsafe {
        SCB.shpr1().pri_4() as u8
    }
}

pub fn set_pri_4(value: u8) {
    unsafe {
        SCB.with_shpr1(|r| r.set_pri_4(value as u32))
    }
}

pub fn pri_5() -> u8 {
    unsafe {
        SCB.shpr1().pri_5() as u8
    }
}

pub fn set_pri_5(value: u8) {
    unsafe {
        SCB.with_shpr1(|r| r.set_pri_5(value as u32))
    }
}

pub fn pri_6() -> u8 {
    unsafe {
        SCB.shpr1().pri_6() as u8
    }
}

pub fn set_pri_6(value: u8) {
    unsafe {
        SCB.with_shpr1(|r| r.set_pri_6(value as u32))
    }
}

pub fn pri_11() -> u8 {
    unsafe {
        SCB.shpr2().pri_11() as u8
    }
}

pub fn set_pri_11(value: u8) {
    unsafe {
        SCB.with_shpr2(|r| r.set_pri_11(value as u32))
    }
}

pub fn pri_14() -> u8 {
    unsafe {
        SCB.shpr3().pri_14() as u8
    }
}

pub fn set_pri_14(value: u8) {
    unsafe {
        SCB.with_shpr3(|r| r.set_pri_14(value as u32))
    }
}

pub fn pri_15() -> u8 {
    unsafe {
        SCB.shpr3().pri_15() as u8
    }
}

pub fn set_pri_15(value: u8) {
    unsafe {
        SCB.with_shpr3(|r| r.set_pri_15(value as u32))
    }
}

// Handler Priority Aliases

pub fn pri_mem_manage() -> u8 {
    pri_4()
}

pub fn set_pri_mem_manage(value: u8) {
    set_pri_4(value)
}

pub fn pri_bus_fault() -> u8 {
    pri_5()
}

pub fn set_pri_bus_fault(value: u8) {
    set_pri_5(value)
}

pub fn pri_usage_fault() -> u8 {
    pri_6()
}

pub fn set_pri_usage_fault(value: u8) {
    set_pri_6(value)
}

pub fn pri_svcall() -> u8 {
    pri_11()
}

pub fn set_pri_svcall(value: u8) {
    set_pri_11(value)
}

pub fn pri_pendsv() -> u8 {
    pri_14()
}

pub fn set_pri_pendsv(value: u8) {
    set_pri_14(value)
}

pub fn pri_systick() -> u8 {
    pri_15()
}

pub fn set_pri_systick(value: u8) {
    set_pri_15(value)
}

// SHCSR

pub fn usgfaultena() -> bool {
    unsafe {
        SCB.shcsr().usgfaultena() != 0
    }
}

pub fn set_usgfaultena(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_shcsr(|r| r.set_usgfaultena(value))
    }
}

pub fn busfaultena() -> bool {
    unsafe {
        SCB.shcsr().busfaultena() != 0
    }
}

pub fn set_busfaultena(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_shcsr(|r| r.set_busfaultena(value))
    }
}

pub fn memfaultena() -> bool {
    unsafe {
        SCB.shcsr().memfaultena() != 0
    }
}

pub fn set_memfaultena(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_shcsr(|r| r.set_memfaultena(value))
    }
}

pub fn svcallpended() -> bool {
    unsafe {
        SCB.shcsr().svcallpended() != 0
    }
}

pub fn set_svcallpended(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_shcsr(|r| r.set_svcallpended(value))
    }
}

pub fn busfaultpended() -> bool {
    unsafe {
        SCB.shcsr().busfaultpended() != 0
    }
}

pub fn set_busfaultpended(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_shcsr(|r| r.set_busfaultpended(value))
    }
}

pub fn memfaultpended() -> bool {
    unsafe {
        SCB.shcsr().memfaultpended() != 0
    }
}

pub fn set_memfaultpended(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_shcsr(|r| r.set_memfaultpended(value))
    }
}

pub fn usgfaultpended() -> bool {
    unsafe {
        SCB.shcsr().usgfaultpended() != 0
    }
}

pub fn set_usgfaultpended(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_shcsr(|r| r.set_usgfaultpended(value))
    }
}

pub fn systickact() -> bool {
    unsafe {
        SCB.shcsr().systickact() != 0
    }
}

pub fn set_systickact(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_shcsr(|r| r.set_systickact(value))
    }
}

pub fn pendsvact() -> bool {
    unsafe {
        SCB.shcsr().pendsvact() != 0
    }
}

pub fn set_pendsvact(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_shcsr(|r| r.set_pendsvact(value))
    }
}

pub fn monitoract() -> bool {
    unsafe {
        SCB.shcsr().monitoract() != 0
    }
}

pub fn set_monitoract(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_shcsr(|r| r.set_monitoract(value))
    }
}

pub fn svcallact() -> bool {
    unsafe {
        SCB.shcsr().svcallact() != 0
    }
}

pub fn set_svcallact(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_shcsr(|r| r.set_svcallact(value))
    }
}

pub fn usgfaultact() -> bool {
    unsafe {
        SCB.shcsr().usgfaultact() != 0
    }
}

pub fn set_usgfaultact(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_shcsr(|r| r.set_usgfaultact(value))
    }
}

pub fn busfaultact() -> bool {
    unsafe {
        SCB.shcsr().busfaultact() != 0
    }
}

pub fn set_busfaultact(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_shcsr(|r| r.set_busfaultact(value))
    }
}

pub fn memfaultact() -> bool {
    unsafe {
        SCB.shcsr().memfaultact() != 0
    }
}

pub fn set_memfaultact(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_shcsr(|r| r.set_memfaultact(value))
    }
}

// MMFSR

pub fn mmarvalid() -> bool {
    unsafe {
        SCB.mmfsr().mmarvalid() != 0
    }
}

pub fn set_mmarvalid(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_mmfsr(|r| r.set_mmarvalid(value))
    }
}

pub fn mstkerr() -> bool {
    unsafe {
        SCB.mmfsr().mstkerr() != 0
    }
}

pub fn set_mstkerr(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_mmfsr(|r| r.set_mstkerr(value))
    }
}

pub fn munstkerr() -> bool {
    unsafe {
        SCB.mmfsr().munstkerr() != 0
    }
}

pub fn set_munstkerr(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_mmfsr(|r| r.set_munstkerr(value))
    }
}

pub fn daccviol() -> bool {
    unsafe {
        SCB.mmfsr().daccviol() != 0
    }
}

pub fn set_daccviol(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_mmfsr(|r| r.set_daccviol(value))
    }
}

pub fn iaccviol() -> bool {
    unsafe {
        SCB.mmfsr().iaccviol() != 0
    }
}

pub fn set_iaccviol(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_mmfsr(|r| r.set_iaccviol(value))
    }
}

// BFSR

pub fn bfarvalid() -> bool {
    unsafe {
        SCB.bfsr().bfarvalid() != 0
    }
}

pub fn set_bfarvalid(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_bfsr(|r| r.set_bfarvalid(value))
    }
}

pub fn stkerr() -> bool {
    unsafe {
        SCB.bfsr().stkerr() != 0
    }
}

pub fn set_stkerr(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_bfsr(|r| r.set_stkerr(value))
    }
}
pub fn unstkerr() -> bool {
    unsafe {
        SCB.bfsr().unstkerr() != 0
    }
}

pub fn set_unstkerr(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_bfsr(|r| r.set_unstkerr(value))
    }
}
pub fn impreciserr() -> bool {
    unsafe {
        SCB.bfsr().impreciserr() != 0
    }
}

pub fn set_impreciserr(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_bfsr(|r| r.set_impreciserr(value))
    }
}

pub fn preciserr() -> bool {
    unsafe {
        SCB.bfsr().preciserr() != 0
    }
}

pub fn set_preciserr(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_bfsr(|r| r.set_preciserr(value))
    }
}

pub fn ibuserr() -> bool {
    unsafe {
        SCB.bfsr().ibuserr() != 0
    }
}

pub fn set_ibuserr(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_bfsr(|r| r.set_ibuserr(value))
    }
}

// UFSR

pub fn divbyzero() -> bool {
    unsafe {
        SCB.ufsr().divbyzero() != 0
    }
}

pub fn set_divbyzero(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_ufsr(|r| r.set_divbyzero(value))
    }
}

pub fn unaligned() -> bool {
    unsafe {
        SCB.ufsr().unaligned() != 0
    }
}

pub fn set_unaligned(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_ufsr(|r| r.set_unaligned(value))
    }
}

pub fn nocp() -> bool {
    unsafe {
        SCB.ufsr().nocp() != 0
    }
}

pub fn set_nocp(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_ufsr(|r| r.set_nocp(value))
    }
}


pub fn invpc() -> bool {
    unsafe {
        SCB.ufsr().invpc() != 0
    }
}

pub fn set_invpc(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_ufsr(|r| r.set_invpc(value))
    }
}


pub fn invstate() -> bool {
    unsafe {
        SCB.ufsr().invstate() != 0
    }
}

pub fn set_invstate(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_ufsr(|r| r.set_invstate(value))
    }
}

pub fn undefinstr() -> bool {
    unsafe {
        SCB.ufsr().undefinstr() != 0
    }
}

pub fn set_undefinstr(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_ufsr(|r| r.set_undefinstr(value))
    }
}

// HFSR

pub fn debugevt() -> bool {
    unsafe {
        SCB.hfsr().debugevt() != 0
    }
}

pub fn set_debugevt(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_hfsr(|r| r.set_debugevt(value))
    }
}

pub fn forced() -> bool {
    unsafe {
        SCB.hfsr().forced() != 0
    }
}

pub fn set_forced(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_hfsr(|r| r.set_forced(value))
    }
}

pub fn vecttbl() -> bool {
    unsafe {
        SCB.hfsr().vecttbl() != 0
    }
}

pub fn set_vecttbl(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SCB.with_hfsr(|r| r.set_vecttbl(value))
    }
}

pub fn memmanage_fault_address() -> u32 {
    unsafe {
        SCB.mmfar().address()
    }
}

pub fn busfault_address() -> u32 {
    unsafe {
        SCB.bfar().address()
    }
}

pub fn auxiliary_fault_status() -> u32 {
    unsafe {
        SCB.afsr().impdef()
    }
}