use bobbin_hal::prelude::*;
use bobbin_sys::prelude::*;

pub fn run<S: SystemProvider + GetFlash>(mut sys: System<S>, flash_addr: *mut u8, flash_len: usize) -> ! 
{
    sys.run(|sys| {
        let flash = sys.flash();
        let console = sys.console();
        console.writeln(b"Erasing Flash");

        flash.erase_begin();
        flash.erase(flash_addr).unwrap_or_abort("Error erasing flash");
        flash.erase_end();
        unsafe { console.dump_ptr(flash_addr as *const u8, flash_len); }

        console.writeln(b"Writing Flash");
        let mut buf = [0u8; 0x100];
        for i in 0..buf.len() {
            buf[i] = i as u8;
        }
        flash.write_begin();
        flash.write(flash_addr as *mut u8, &buf).unwrap_or_abort("Error writing flash");
        flash.write_end();
        unsafe { console.dump_ptr(flash_addr as *const u8, flash_len); }
        console.writeln(b"Done");    

        loop {}
    })    
}