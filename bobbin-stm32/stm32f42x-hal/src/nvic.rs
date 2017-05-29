use cortex_m::peripheral;

pub fn current_exception() -> u8 {
    peripheral::scb().icsr.read() as u8
}

pub fn enabled(irq: u8) -> bool {
    let nvic = peripheral::nvic();        
    (nvic.iser[(irq / 32) as usize].read() & (1 << (irq % 32))) != 0    
}

pub fn set_enabled(irq: u8, value: bool) {
    unsafe {
        let nvic = peripheral::nvic_mut(); 
        if value {
            nvic.iser[(irq / 32) as usize].write(1 << (irq % 32))
        } else {
            nvic.icer[(irq / 32) as usize].write(1 << (irq % 32))
        }    
        
    }
}

pub fn enable(irq: u8) {
    set_enabled(irq, true)
}

pub fn disable(irq: u8) {
    set_enabled(irq, false)
}

pub fn pendsv() -> bool {
    (peripheral::scb().icsr.read() & 1 << 28) != 0 
}

pub fn set_pendsv() {
    unsafe {
        // PENDSVSET
        peripheral::scb_mut().icsr.write(peripheral::scb().icsr.read() | 1 << 28);
    }
}

pub fn clear_pendsv() {
    unsafe {
        // PENDSVCLR
        peripheral::scb_mut().icsr.write(peripheral::scb().icsr.read() | 1 << 27);
    }
}


pub fn vect_active() -> u32 {
    peripheral::scb().icsr.read() & 0b11111
}

pub fn vect_pending() -> u32 {
    (peripheral::scb().icsr.read() >> 12) & 0b11111
}
