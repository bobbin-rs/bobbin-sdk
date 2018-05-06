use bobbin_sys::prelude::*;

pub fn run_with_sys<S: SystemProvider>(_sys: System<S>) -> ! {
    println!("IPC Example");
    loop {}
}