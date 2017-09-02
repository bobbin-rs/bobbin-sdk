#![no_std]
#![no_main]
#![feature(generators, generator_trait)]

#[macro_use]
extern crate evb_s32k144 as board;

use core::ops::{Generator, GeneratorState};

struct Task<'a> {
    gen: &'a mut Generator<Yield=(), Return=()>,
    active: bool,
}

impl<'a> Task<'a> {
    fn new(gen: &'a mut Generator<Yield=(), Return=()>) -> Self {
        Task {
            gen: gen,
            active: true,
        }
    }

    fn resume(&mut self) {
        if self.active {
            match self.gen.resume() {
                GeneratorState::Complete(_) => {
                    self.active = false;
                    println!("Task Complete");
                },
                _ => {},
            }
        }
    }
}

pub struct IdleTask {
    count: usize,
}

impl IdleTask {
    fn new() -> Self {
        IdleTask { count: 0 }
    }
}

impl Generator for IdleTask {
    type Yield = ();
    type Return = ();
    fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return> {
        println!("Idle: {}", self.count);
        self.count += 1;
        GeneratorState::Yielded(())
    }
}

use core::cell::RefCell;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running Generators Test");    

    let mut idle = IdleTask::new();

    let total: RefCell<usize> = RefCell::new(0);

    let mut i = 0;
    let t = &total;
    let mut g1 = move || {
        loop {
            if i == 5 {
                return;
            }
            println!("Task 1: {}", i);
            *t.borrow_mut() += 1;
            i += 1;
            yield;
        }
    };

    let mut j = 0;
    let t = &total;
    let mut g2 = move || {
        loop {
            println!("Task 2: {}", j);
            *t.borrow_mut() += 1;            
            j += 1;
            yield;
        }
    };

    let mut task_1 = Task::new(&mut g1);
    let mut task_2 = Task::new(&mut g2);
    let mut tasks: [&mut Task] = [
        &mut task_1, 
        &mut task_2,
        &mut idle,
    ];

    loop {
        for task in tasks.iter_mut() {
            task.resume();
        }
        println!("total: {}", &total.borrow());
        // *total.borrow_mut() += 1;
        board::delay(1000);
    }
}
