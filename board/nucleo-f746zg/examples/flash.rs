#![no_std]
#![no_main]

extern crate nucleo_f746zg as board;
extern crate examples;

pub const FLASH_ADDR: *mut u8 = 0x0801_0000 as *mut u8;
pub const FLASH_LEN: usize = 0x100;

#[no_mangle]
pub extern "C" fn main() -> ! {
    examples::flash::run(board::init(), FLASH_ADDR, FLASH_LEN)
}

// #[no_mangle]
// pub extern "C" fn main() -> ! {
//     let mut brd = board::init();
//     brd.run(|brd| {
//         if let Some(console) = brd.console() {
//             console.writeln(b"Erasing Flash");

//             FLASH.erase_begin();
//             FLASH.erase(FLASH_ADDR).unwrap_or_abort("Error erasing flash");
//             FLASH.erase_end();
//             unsafe { console.dump_ptr(FLASH_ADDR as *const u8, FLASH_LEN); }

//             console.writeln(b"Writing Flash");
//             let mut buf = [0u8; 0x100];
//             for i in 0..buf.len() {
//                 buf[i] = i as u8;
//             }
//             FLASH.write_begin();
//             FLASH.write(FLASH_ADDR as *mut u8, &buf).unwrap_or_abort("Error writing flash");
//             FLASH.write_end();
//             unsafe { console.dump_ptr(FLASH_ADDR as *const u8, FLASH_LEN); }
//             console.writeln(b"Done");    
//         }
//         loop {}
//     })
// }