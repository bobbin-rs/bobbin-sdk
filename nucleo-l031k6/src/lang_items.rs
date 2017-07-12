use core::fmt::Arguments;

#[lang = "panic_fmt"]
unsafe extern "C" fn panic_fmt(_msg: Arguments,
                               _file: &'static str,
                               _line: u32)
                               -> ! {
    println!("[panic]");
    loop {}
}
