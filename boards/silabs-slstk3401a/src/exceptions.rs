#[doc(hidden)]
#[export_name = "_default_exception_handler"]
pub unsafe extern "C" fn default_handler_entry_point() -> ! {
    loop {}
}

#[doc(hidden)]
#[export_name = "_reset"]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        fn main();
    }    
    main();
    loop {}
}

pub type Handler = unsafe extern "C" fn();

#[link_section = ".vector.exceptions"]
#[no_mangle]
pub static EXCEPTION_HANDLERS: [Option<Handler>; 15] = [
   Some(_reset),                  // Reset Handler
   Some(_nmi),                    // Non-maskable interrupt.
   Some(_hard_fault),             // All class of fault.
   Some(_memmanage_fault),        // Memory Management
   Some(_bus_fault),              // Pre-fetch fault, memory access fault.
   Some(_usage_fault),            // Undefined instruction or illegal state.
   None,
   None,
   None,
   None,
   Some(_svcall),                 // System service call via SWI instruction
   Some(_debug),                  // Debug
   None,
   Some(_pendsv),                 // Pendable request for system service
   Some(_systick),                // System tick timer
];

extern "C" {
   pub fn _reset();               // Reset Handler
   pub fn _nmi();                 // Non-maskable interrupt.
   pub fn _hard_fault();          // All class of fault.
   pub fn _memmanage_fault();     // Memory Management
   pub fn _bus_fault();           // Pre-fetch fault, memory access fault.
   pub fn _usage_fault();         // Undefined instruction or illegal state.
   pub fn _svcall();              // System service call via SWI instruction
   pub fn _debug();               // Debug
   pub fn _pendsv();              // Pendable request for system service
   pub fn _systick();             // System tick timer
}
