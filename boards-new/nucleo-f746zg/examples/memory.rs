#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;

static mut DATA: [u8; 1024] = [0u8; 1024];

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    for i in 0..1024 {
        unsafe { DATA[i] = i as u8; }
    }

    println!("Memory Test");
    println!("DATA:       {:p}", unsafe { &DATA });
    unsafe {
        println!("_sbss:      {:p}", &_sbss);
        println!("_ebss:      {:p}", &_ebss);
        println!("");

        println!("_sdata:     {:p}", &_sdata);
        println!("_edata:     {:p}", &_edata);
        println!("");

        println!("_sstack:    {:p}", &_sstack);
        println!("_estack:    {:p}", &_estack);
        println!("");

        println!("_sheap:     {:p}", &_sheap);
        println!("_eheap:     {:p}", &_eheap);
        println!("");

        println!("_heap_size: {:p}", &_heap_size);

        let stack_size = 0x2000;
        let _free_start = &mut _sheap as *mut u32 as *mut u8;
        let _free_end = (&mut _estack as *mut u32 as *mut u8).offset(-stack_size);
        let _free_len = _free_end as u32 - _free_start as u32;
        let heap = ::core::slice::from_raw_parts_mut(_free_start, _free_len as usize);

        println!("heap len: {:08x} ({} bytes)", heap.len(), heap.len());
        for i in 0..heap.len() {
            heap[i] = i as u8;
        }
        println!("Memory Updated");
    }

    loop {}
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