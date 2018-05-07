use core::ptr;
use core::cmp;
use core::cell::UnsafeCell;
use core::marker::PhantomData;

pub struct Mailbox<T: Send> {
    flag: bool,
    data: T,
}

impl<T: Send> Mailbox<T> {
    pub fn new(data: T) -> Self {
        Mailbox { flag: false, data}
    }
}


pub struct MailboxSender<'a, T: Send + 'a> {
    ptr: *mut Mailbox<T>,
    _phantom: PhantomData<&'a mut Mailbox<T>>
}

impl<'a, T: Send + 'a> MailboxSender<'a, T> {
    fn mailbox(&self) -> &'a Mailbox<T> {
        unsafe { &*self.ptr }
    }

    fn mailbox_mut(&self) -> &'a mut Mailbox<T> {
        unsafe { &mut *self.ptr }
    }

    pub fn can_send(&self) -> bool {
        unsafe { ptr::read_volatile(&self.mailbox().flag) == false }
    }

    pub fn borrow(&self) -> Option<&T> {
        if self.can_send() {
            Some(&self.mailbox().data)
        } else {
            None
        }
    }
    pub fn borrow_mut(&self) -> Option<&mut T> {
        if self.can_send() {
            Some(&mut self.mailbox_mut().data )
        } else {
            None
        }
    }    
    pub fn send(&self) {
        if self.can_send() {            
            unsafe { ptr::write_volatile(&mut self.mailbox_mut().flag, true) }
        }
    }    
}

pub struct MailboxReceiver<'a, T: Send + 'a> {
    ptr: *mut Mailbox<T>,
    _phantom: PhantomData<&'a mut Mailbox<T>>
}

impl<'a, T: Send + 'a> MailboxReceiver<'a, T> {
    fn mailbox(&self) -> &'a Mailbox<T> {
        unsafe { &*self.ptr }
    }

    fn mailbox_mut(&self) -> &'a mut Mailbox<T> {
        unsafe { &mut *self.ptr }
    }

    pub fn can_recv(&self) -> bool {
        unsafe { ptr::read_volatile(&self.mailbox().flag)  == true }
    }
    pub fn borrow(&self) -> Option<&T> {
        if self.can_recv() {
            Some(&self.mailbox().data)
        } else {
            None
        }
    }
    pub fn borrow_mut(&self) -> Option<&mut T> {
        if self.can_recv() {
            Some(&mut self.mailbox_mut().data)
        } else {
            None
        }
    }      
    pub fn recv(&self) {
        if self.can_recv() {        
            unsafe { ptr::write_volatile(&mut self.mailbox_mut().flag, false) };
        }
    }    
}

pub fn mailbox_pair<'a, T: Send + 'a>(mailbox: &'a mut Mailbox<T>) -> (MailboxSender<'a, T>, MailboxReceiver<'a, T>) {
    {
        (
            MailboxSender { ptr: mailbox, _phantom: PhantomData },
            MailboxReceiver { ptr: mailbox, _phantom: PhantomData },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mailbox() {
        #[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
        struct Config {
            size: u32,
            speed: u32,
        }

        let mut mb = Mailbox::new(Config::default());
        let (mb_send, mb_recv) = mailbox_pair(&mut mb);

        let cfg = Config { size: 1024, speed: 256 };
        assert_eq!(mb_send.can_send(), true);
        assert_eq!(mb_recv.can_recv(), false);        

        if let Some(cfg_send) = mb_send.borrow_mut() {
            *cfg_send = cfg;
            mb_send.send();
        }
        assert_eq!(mb_send.can_send(), false);
        if let Some(cfg_recv) = mb_recv.borrow_mut() {
            assert_eq!(*cfg_recv, cfg);
            cfg_recv.speed = 1024;
            mb_recv.recv();
        }
        assert_eq!(mb_send.can_send(), true);
        if let Some(cfg_send) = mb_send.borrow_mut() {
            assert_eq!(cfg_send.speed, 1024);
        }
    }
    #[test]
    fn test_mailbox_empty() {
        let mut mb = Mailbox::new(());
        let (mb_send, mb_recv) = mailbox_pair(&mut mb);
        assert_eq!(mb_send.can_send(), true);
        assert_eq!(mb_recv.can_recv(), false);
        if mb_send.can_send() {
            mb_send.send();
        }
        assert_eq!(mb_send.can_send(), false);
        assert_eq!(mb_recv.can_recv(), true);
        if let Some(v) = mb_recv.borrow_mut() {
            *v = ();
            mb_recv.recv();
        }
    }
}