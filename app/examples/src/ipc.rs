use bobbin_sys::prelude::*;
use bobbin_ipc::flag::*;
use bobbin_ipc::counter::*;
use bobbin_ipc::semaphore::*;
use bobbin_ipc::mailbox::*;
use bobbin_ipc::fifo::*;

pub fn run_with_sys<S: SystemProvider>(mut sys: System<S>) -> ! {
    if true {
        println!("Flag Example");
        // Flag Example
        let flag_bool: &mut bool = sys.heap_mut().new(false);
        let (flag_setter, flag_getter) = flag_pair(flag_bool);

        let flag_task = FlagTask { flag_setter };
        let _flag_guard = match sys.tick_mut().register(&flag_task) {
            Ok(guard) => guard,
            Err(_) => {
                println!("Unable to register flag task");
                loop {}
            }            
        };

        sys.run(|_| {
            println!("Waiting for five ticks");
            for i in 0..5 {
                while flag_getter.is_clr() {}
                println!("Tick {}...", i+1);
                flag_getter.clr();
            }
        });
    }

    if true {
        println!("Counter Example");
        let counter: &mut u32 = sys.heap_mut().new(0);
        let (counter_getter, counter_setter) = counter_pair(counter);

        let counter_task = CounterTask { counter_setter };
        let _counter_guard = match sys.tick_mut().register(&counter_task) {
            Ok(guard) => guard,
            Err(_) => {
                println!("Unable to register counter task");
                loop {}
            }
        };
        sys.run(|sys| {
            for i in 0..5 {
                println!("Counter: {}: {}", i, counter_getter.get());
                sys.tick().delay(500);
            }
        })        
    }

    if true {
        println!("Semaphore Example");
        let (sem_head, sem_tail): (&mut u32, &mut u32) = (sys.heap_mut().new(0u32), sys.heap_mut().new(0u32));
        let (sem_incr, sem_decr) = semaphore_pair(sem_head, sem_tail);
        let sem_task = SemaphoreTask { sem_incr };
        let _sem_guard = match sys.tick_mut().register(&sem_task) {
            Ok(guard) => guard,
            Err(_) => {
                println!("Unable to register semaphore task");
                loop {}
            }
        };
        sys.run(|_| {
            for i in 0..15 {
                while sem_decr.get() == 0 {}
                sem_decr.decr(1);
                println!("Tick... {}", i);
            }
        })
    }

    if true {
        println!("Mailbox Example");
        let mb: &'static mut Mailbox<Message> = sys.heap_mut().new(Mailbox::new(Message { id: 0, value: 0}));
        let (mb_tx, mb_rx) = mailbox_pair(mb);
        let mb_task = MailboxTask { mb_tx, counter: ::core::cell::Cell::new(0) };
        let _mb_guard = match sys.tick_mut().register(&mb_task) {
            Ok(guard) => guard,
            Err(_) => {
                println!("Unable to register mailbox task");
                loop {}
            }
        };

        sys.run(|_| {
            for _ in 0..5 {
                while !mb_rx.can_recv() {}
                if let Some(msg) = mb_rx.borrow() {
                    println!("id: {}, value: {}", msg.id, msg.value);
                    mb_rx.recv();
                }
            }
        })

    }

    if true {
        println!("FIFO Example");
        let mut fifo_head = FifoHeader::new();
        let mut fifo_data = [0u32; 8];
        let (fifo_send, fifo_recv) = fifo_pair(&mut fifo_head, &mut fifo_data);
        let fifo_task = FifoTask { fifo_send };
        let _fifo_guard = match sys.tick_mut().register(&fifo_task) {
            Ok(guard) => guard,
            Err(_) => {
                println!("Unable to register mailbox task");
                loop {}
            }
        };    
        sys.run(|_| {
            for _ in 0..5 {
                while fifo_recv.len() == 0 {}
                if let Some(msg) = fifo_recv.recv() {
                    println!("Recv: {}", msg);
                }
            }
        })            
    }


    println!("[done]");
    loop {}
}

pub struct FlagTask {
    flag_setter: FlagSet<'static>,
}

impl HandleTick for FlagTask {
    fn handle_tick(&self, c: u32) {
        if c % 500 == 0 {
            self.flag_setter.set();
        }
    }
}

pub struct CounterTask {
    counter_setter: CounterSet<'static, u32>,    
}

impl HandleTick for CounterTask {
    fn handle_tick(&self, c: u32) {
        if c % 100 == 0 {
            self.counter_setter.set(self.counter_setter.get().wrapping_add(1));
        }
    }
}

pub struct SemaphoreTask {
    sem_incr: SemaphoreIncr<'static, u32>,
}

impl HandleTick for SemaphoreTask {
    fn handle_tick(&self, c: u32) {
        if c % 500 == 0 {
            self.sem_incr.incr(3);
        }
    }
}

pub struct Message {
    pub id: u32,
    pub value: u32,
}

pub struct MailboxTask {
    mb_tx: MailboxSender<'static, Message>,
    counter: ::core::cell::Cell<u32>,
}

impl HandleTick for MailboxTask {
    fn handle_tick(&self, c: u32) {
        if c % 500 == 0 {
            if let Some(msg) = self.mb_tx.borrow_mut() {
                *msg = Message { id: self.counter.get(), value: c };
                self.mb_tx.send();
            }
            self.counter.set(self.counter.get().wrapping_add(1));
        }        
    }
}

pub struct FifoTask {
    fifo_send: FifoSender<'static, u32>,
}

impl HandleTick for FifoTask {
    fn handle_tick(&self, c: u32) {
        if c % 500 == 0 {
            self.fifo_send.send(c);
        }
    }
}