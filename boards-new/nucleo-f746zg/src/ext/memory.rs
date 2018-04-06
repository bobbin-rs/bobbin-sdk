use ::core::slice;
use ::core::ops::Range;
use ::core::fmt;

pub struct Memory {}

impl Memory {
    fn bss() -> Range<usize> {
        unsafe { &_sbss as *const u32 as usize .. &_ebss as *const u32 as usize }
    }

    fn data() -> Range<usize> {
        unsafe { &_sdata as *const u32 as usize .. &_edata as *const u32 as usize }
    }

    fn stack() -> Range<usize> {
        unsafe { &_sstack as *const u32 as usize .. &_estack as *const u32 as usize }
    }

    fn heap() -> Range<usize> {
        unsafe { &_sheap as *const u32 as usize .. &_eheap as *const u32 as usize }
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        for (label, a, b) in &[
            ("     RAM ", Self::bss().start, Self::stack().end),            
            ("     BSS ", Self::bss().start, Self::bss().end),
            ("    DATA ", Self::data().start, Self::data().end),
            ("    HEAP ", Self::heap().start, Self::heap().end),
            ("   STACK ", Self::stack().start, Self::stack().end),
        ] {        
            writeln!(out, "{}   0x{:08x} - 0x{:08x} ({} bytes)", label, a, b, b - a)?;
        }
        Ok(())
    }
}


extern "C" {
    static mut _stext: u32;

    static mut _sbss: u32;
    static mut _ebss: u32;
    
    static mut _sdata: u32;
    static mut _edata: u32;


    static mut _sstack: u32;
    static mut _estack: u32;
    static mut _stack_size: u32;

    static mut _sheap: u32;
    static mut _eheap: u32;
    static mut _heap_size: u32;
}