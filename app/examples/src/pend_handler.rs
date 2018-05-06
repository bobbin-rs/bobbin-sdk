use bobbin_sys::prelude::*;
use core::cell::UnsafeCell;

pub fn run_with_sys<S: SystemProvider>(mut sys: System<S>) -> ! {
    let ticker = Ticker::new();
    let pender = Pender::new();

    let _guard_tick = match sys.tick_mut().register(&ticker) {
        Ok(guard) => guard,
        Err(_) => {
            println!("Error registering tick handler.");
            loop {}
        }        
    };

    let _guard_pend = match sys.pend_mut().register(&pender) {
        Ok(guard) => guard,
        Err(_) => {
            println!("Error registering pend handler.");
            loop {}
        }        
    };

    sys.run(|sys| {
        loop {
            sys.console().write(b"Tick...");
            sys.console().write_u32(ticker.counter(), 10);
            sys.console().writeln(b"");
            sys.tick().delay(500);
        }
    })    
}

pub struct Ticker {
    counter: UnsafeCell<u32>,
}

impl Ticker {
    pub fn new() -> Self {
        Self { counter: UnsafeCell::new(0) }        
    }

    pub fn counter(&self) -> u32 {
        unsafe { *self.counter.get() }
    }
}

impl HandleTick for Ticker {
    fn handle_tick(&self, c: u32) {
        if c % 1000 == 0 {
            println!("... tick");
            unsafe { (*self.counter.get()) += 1 }
        }
    }
}

pub struct Pender {
    counter: UnsafeCell<u32>,
}

impl Pender {
    pub fn new() -> Self {
        Self { counter: UnsafeCell::new(0) }
    }
    pub fn counter(&self) -> u32 {
        unsafe { *self.counter.get() }
    }
}

impl HandlePend for Pender {
    fn handle_pend(&self) {
        println!("pend {}", self.counter());
        unsafe { (*self.counter.get()) += 1 }
    }
}