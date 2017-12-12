#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;

use board::{Box, Vec, VecDeque};
use core::cell::RefCell;

static mut HEAP: [u8; 4096] = [0u8; 4096];

pub struct Counter {
    count: RefCell<usize>,
}

impl Counter {
    pub fn new(count: usize) -> Self {
        Counter { count: RefCell::new(count) }
    }
    pub fn count(&self) -> usize {
        *self.count.borrow()
    }
    pub fn incr(&self) {
        *self.count.borrow_mut() += 1
    }
}


// pub struct CounterHandle<'a>(RefCell<&'a Counter>);

// impl<'a> CounterHandle<'a> {
//     pub fn count(&self) -> usize {
//         self.0.borrow().count()
//     }
//     pub fn incr(&self) -> usize {
//         self.0.borrow_mut().incr()
//     }
// }


#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe { board::init_allocator(&mut HEAP) };
    board::init();
    println!("Allocator Test");
    let n = Box::new(0u32);
    println!("n: {}", n);

    let mut v: Vec<u32> = Vec::new();
    for i in 0..10 {
        v.push(i as u32);
    }

    println!("VEC: {:?}", v);

    let mut ring: VecDeque<u8> = VecDeque::new();
    for i in 0..16 {
        ring.push_back(i as u8);
    }
    println!("ring: {:?}", ring);

    for i in 0..16 {
        assert_eq!(ring.pop_front(), Some(i));
    }

    println!("ring: {:?}", ring);

    let c = Counter::new(0);
    for _ in 0..16 {
        c.incr();
    }
    println!("C: {}", c.count());

    let bc = Box::new(Counter::new(0));
    for _ in 0..16 {
        bc.incr();
    }
    println!("BC: {}", bc.count());

    loop {}
}
