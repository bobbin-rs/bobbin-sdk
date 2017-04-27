use r0;
use hal;
use core::ptr;
//use chip::wdog::{self, WDOG};

#[doc(hidden)]
#[export_name = "_default_exception_handler"]
pub unsafe extern "C" fn default_handler_entry_point() -> ! {
    println!("EXCEPTION");
    loop {}
}

#[doc(hidden)]
#[export_name = "_reset"]
#[naked]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        static _ebss: u32;
        static _edata: u32;
        static _evector: u32;
        static _sidata: u32;
        static mut _srvector: u32;
        static _ervector: u32;
        static mut _sbss: u32;
        static mut _sdata: u32;
        static mut _svector: u32;

        // `main`, the entry point of the user program
        // NOTE the right signature of `main` is `fn() -> !`. But the user might
        // get that wrong so let's err on the side of caution and install a
        // safety net. (See below)
        fn main();
    }
    // Unlock WDOG and set configuration and counter
    // Counter must be explicitly set!    
    ptr::write_volatile(0x40052004 as *mut u32, 0xD928C520);
    ptr::write_volatile(0x40052000 as *mut u32, 0x00002900);
    ptr::write_volatile(0x40052008 as *mut u32, 0x00000400);

    r0::zero_bss(&mut _sbss, &_ebss);
    r0::init_data(&mut _sdata, &_edata, &_sidata);

    // Copy ISR vectors to RAM
    r0::init_data(&mut _srvector, &_ervector, &_svector); 
    hal::scb::set_tbloff(0x1FFF8000 >> 7);    

    main();
    loop {}
}