//! Extends the chip::scb module.

pub use bobbin_common::Handler;
pub use chip::scb::*;

// VTOR

/// Returns bits [29:7] of the offset of the vector table base.
pub fn tbloff() -> u32 {
    SCB.vtor().tbloff().into()
}

/// Sets bits [29:7] of the offset of the vector table base.
pub fn set_tbloff(value: u32) {
    SCB.set_vtor(|r| r.set_tbloff(value));
}

impl Scb {
    pub fn set_irq_handler(&self, irq_num: usize, f: Option<Handler>) {
        assert!(self.irq_handler(irq_num).is_some() != f.is_some(),"IRQ Handler {} is already set.", irq_num);
        let addr = (self.vtor().0 as usize) + ((irq_num + 16) * 4);
        unsafe {            
            ::core::ptr::write_volatile(addr as *mut Option<Handler>, f)
        }
    }

    pub fn irq_handler(&self, irq_num: usize) -> Option<Handler> {
        let addr = (self.vtor().0 as usize) + ((irq_num + 16) * 4);
        unsafe { 
            ::core::ptr::read_volatile(addr as *const Option<Handler>)
        }
    }
}

// http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0489c/CIHBIGGI.html
// 
// Invalidate the entire data cache
// 
// Software can use the following code example to invalidate the entire data cache, if it has been included in the processor. The operation is carried out by iterating over each line of the cache and using the DCISW register in the Private Peripheral Bus (PPB) memory region to invalidate the line. The number of cache ways and sets is determined by reading the CCSIDR register.
// 
// CCSIDR  EQU 0xE000ED80
// CSSELR  EQU 0xE000ED84
// DCISW   EQU 0xE000EF60
// 
//         MOV r0, #0x0
// 
//         LDR r11, =CSSELR
//         STR r0, [r11]                 ; Select Data Cache size
//         DSB
//                         LDR r11, =CCSIDR
//         LDR r2, [r11]                 ; Cache size identification
//         AND r1, r2, #0x7              ; Number of words in a cache line
//         ADD r7, r1, #0x4
//         MOV r1, #0x3ff
//         ANDS r4, r1, r2, LSR #3
//         MOV r1, #0x7fff        ANDS r2, r1, r2, LSR #13
//         CLZ r6, r4
// 
//         LDR r11, =DCISW
// 
// inv_loop1
//         MOV r1, r4
// 
// inv_loop2
//         LSL r3, r1, r6
//         LSL r8, r2, r7
//         ORRr 3, r3, r8
// 
//         STR r3, [r11]               ; Invalidate D-cache line
// 
//         SUBS r1, r1, #0x1
//         BGE inv_loop2
//         SUBS r2, r2, #0x1
//         BGE inv_loop1        DSB        ISB
// 
// Invalidate instruction cache
// 
// You can use the following code example to invalidate the entire instruction cache, if it has been included in the processor. The operation is carried out by writing to the ICIALLU register in the PPB memory region.
// 
// ICIALLU EQU 0xE000EF50
// 
//         MOV r0, #0x0
//         LDR r11, =ICIALLU
//         STR r0, [r11]
// 
//         DSB
//         ISB
// Enabling data and instruction caches
// You can use the following code example to enable the data and instruction cache after they have been initialized. The operation is carried out by modifying the CCR.IC and CCR.DC fields in the PPB memory region.
// CCR     EQU 0xE000ED14        LDR r11, =CCR        LDR r0, [r11]        ORR r0, r0, #0x1:SHL:16    ; Set CCR.DC field        ORR r0, r0, #0x1:SHL:17    ; Set CCR.IC field        STR r0, [r11]        DSB        ISB

#[inline]
pub unsafe fn invalidate_icache() {
    asm!("
        dsb
        isb
    ");    
    SCB.set_iciallu(|r| r);
    asm!("
        dsb
        isb
    ");
}

#[inline]
pub unsafe fn invalidate_dcache() {
    SCB.set_ccselr(|r| r);
    asm!("dsb");
    let ccsidr = SCB.ccsidr();
    let mut sets = ccsidr.num_sets().value();
    loop {
        let mut ways = ccsidr.associativity().value();
        loop {        
            SCB.set_dcisw(|r| r.set_set(sets).set_way(ways));
            ways -= 1;
            if ways == 0 { break }
        }
        sets -= 1;
        if sets == 0 { break }
    }
}

#[inline]
pub unsafe fn clean_dcache() {
    SCB.set_ccselr(|r| r);
    asm!("dsb");
    let ccsidr = SCB.ccsidr();
    let mut sets = ccsidr.num_sets().value();
    loop {
        let mut ways = ccsidr.associativity().value();
        loop {        
            SCB.set_dccsw(|r| r.set_set(sets).set_way(ways));
            ways -= 1;
            if ways == 0 { break }
        }
        sets -= 1;
        if sets == 0 { break }
    }
}

#[inline]
pub unsafe fn clean_invalidate_dcache() {
    SCB.set_ccselr(|r| r);
    asm!("dsb");
    let ccsidr = SCB.ccsidr();
    let mut sets = ccsidr.num_sets().value();
    loop {
        let mut ways = ccsidr.associativity().value();
        loop {        
            SCB.set_dccisw(|r| r.set_set(sets).set_way(ways));
            ways -= 1;
            if ways == 0 { break }
        }
        sets -= 1;
        if sets == 0 { break }
    }
}

#[inline]
pub unsafe fn enable_icache() {
    asm!("
        dsb
        isb
    ");
    SCB.with_ccr(|r| r.set_ic(1));
    asm!("
        dsb
        isb
    ");    
}

#[inline]
pub unsafe fn disable_icache() {
    asm!("
        dsb
        isb
    ");
    SCB.with_ccr(|r| r.set_ic(0));
    asm!("
        dsb
        isb
    ");    
}


#[inline]
pub unsafe fn enable_dcache() {
    asm!("
        dsb
        isb
    ");   
    SCB.with_ccr(|r| r.set_dc(1));
    asm!("
        dsb
        isb
    ");   
}


#[inline]
pub unsafe fn disable_dcache() {
    asm!("
        dsb
        isb
    ");   
    SCB.with_ccr(|r| r.set_dc(0));
    asm!("
        dsb
        isb
    ");   
}