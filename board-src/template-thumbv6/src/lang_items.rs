use core::fmt::Arguments;
use ::bobbin_console::{write_str, write_u32};

#[lang = "panic_fmt"]
unsafe extern "C" fn panic_fmt(_msg: Arguments,
                               file: &'static str,
                               line: u32)
                               -> ! {
    write_str("[panic] ");
    write_str(file);
    write_str(" at ");
    write_u32(line, 10);
    write_str("\n");
    ::core::intrinsics::abort()
}

