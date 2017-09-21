#![no_std]
#![no_main]
#![feature(generators, generator_trait)]

#[macro_use]
extern crate evb_s32k144 as board;

use core::ops::Generator;

type Task = Generator<Yield=(),Return=()>;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running Generators Test");
    
    let mut i = 0;
    let mut task_1 = move || {
        loop {
            println!("Task 1 Phase 1: {}", i);
            yield;
            println!("Task 1 Phase 2!: {}", i);
            i += 1;
            yield;
        }
    };
    let mut j = 0;
    let mut task_2 = move || {
        loop {
            println!("Task 2: {}", j);
            j += 1;
            yield;
        }
    };

    let mut tasks: [&mut Task; 2] = [&mut task_1, &mut task_2];

    loop {
        for task in tasks.iter_mut() {
            task.resume();
        }
        board::delay(1000);
    }
}
