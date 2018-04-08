#![no_std]
#![no_main]

#[macro_use]
extern crate arduino_zero as board;

static mut DATA: [u8; 1024] = [0u8; 1024];

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    for i in 0..1024 {
        unsafe { DATA[i] = i as u8; }
    }

    println!("Memory Test");
    println!("{:?}", ::board::Memory {});
    let heap = board::Heap {};

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

