#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let sys = board::init();

    println!("Running Systick");
    let st = ::board::systick::SYSTICK;
    loop {
        if let Some(c) = sys.console() {
            c.write(b"Tick: ");
            c.write_u32(st.counter(), 10);
            c.write(b"\r\n");
        }
        st.delay(1000);
    }
}
