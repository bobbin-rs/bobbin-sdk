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


        println!("{:?}", ::board::ext::memory::Memory {});
    }

    let heap = board::ext::Heap {};

    println!("{:?}", heap);

    unsafe { heap.extend(4096) }

    println!("{:?}", heap);

    #[derive(Debug)]
    pub struct Abc { 
        a: u32,
        b: u32,
        c: u32,
    };

    let v = heap.new(Abc { a: 10, b: 20, c: 30 });
    println!("v @ {:p}: {:?}", v, v);
    println!("{:?}", heap);

    let data = heap.slice(0u16, 1024);
    println!("data @ {:p}", data);
    println!("{:?}", heap);

    heap.align(512);
    println!("{:?}", heap);    
    heap.freeze();
    println!("{:?}", heap);    
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