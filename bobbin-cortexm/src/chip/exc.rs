//! Exceptions

pub type Handler = unsafe extern "C" fn();

pub const EXC_RESET: Exception = Exception(0); // Reset Handler
pub const EXC_NMI: Exception = Exception(1); // Non-maskable interrupt.
pub const EXC_HARD_FAULT: Exception = Exception(2); // All class of fault.
pub const EXC_MEMMANAGE_FAULT: Exception = Exception(3); // Memory Management
pub const EXC_BUS_FAULT: Exception = Exception(4); // Pre-fetch fault, memory access fault.
pub const EXC_USAGE_FAULT: Exception = Exception(5); // Undefined instruction or illegal state.
pub const EXC_SVCALL: Exception = Exception(10); // System service call via SWI instruction
pub const EXC_DEBUG: Exception = Exception(11); // Debug
pub const EXC_PENDSV: Exception = Exception(13); // Pendable request for system service
pub const EXC_SYSTICK: Exception = Exception(14); // System tick timer

pub fn set_handler(exc: Exception, handler: Option<Handler>) {
  unsafe { R_EXCEPTION_HANDLERS[exc.0] = handler };
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Exception(pub usize);

impl Exception {
   pub fn set_handler(&self, handler: Option<Handler>) {
      unsafe { R_EXCEPTION_HANDLERS[self.0] = handler };
   }
}

#[cfg_attr(target_os="none", link_section=".vector.exceptions")]
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

#[cfg_attr(target_os="none", link_section=".bss.r_exceptions")]
#[no_mangle]
pub static mut R_EXCEPTION_HANDLERS: [Option<Handler>; 15] = [None; 15];

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
