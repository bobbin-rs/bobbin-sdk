pub use chip::usb::*;
pub use core::ops::Deref;

use bobbin_common::ring::Ring;
use bobbin_common::{Irq, Poll};
use bobbin_cortexm::hal::nvic;
use bobbin_cortexm::hal::scb::SCB;

use core::slice;
use core::cell::Cell;
use core::marker::PhantomData;

pub const ENDPOINT_BUF_SIZE: usize = 64;

#[repr(align(512))]
pub struct BufferDescriptorTable(pub [BufferDesc; 8]);

impl BufferDescriptorTable {
    pub fn index(&self, index: usize, tx: bool, odd: bool) -> usize {
        let v = (index * 4) + if tx { 2 } else { 0 } + if odd { 1 } else { 0 };
        v
    }

    pub fn bd(&self, index: usize, tx: bool, odd: bool) -> &BufferDesc {
        &self.0[self.index(index, tx, odd)]
    }

    pub fn bd_mut(&mut self, index: usize, tx: bool, odd: bool) -> &mut BufferDesc {
        &mut self.0[self.index(index, tx, odd)]
    }    
}

pub struct UsbDriver<'a> {
    usb: UsbPeriph,    
    bdt: *mut BufferDescriptorTable,
    bufs: *mut [[u8; ENDPOINT_BUF_SIZE]],
    rx: Ring<'a, u8>,
    tx: Ring<'a, u8>,
    endpoints: [Cell<EndpointState>; 2],
    _phantom: PhantomData<&'a mut BufferDescriptorTable>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EndpointState(bool, bool);

unsafe impl<'a> Sync for UsbDriver<'a> {}
unsafe impl<'a> Send for UsbDriver<'a> {}

impl<'a> UsbDriver<'a> {
    pub fn new<U: Into<UsbPeriph>>(
        usb: U, 
        bdt: &mut BufferDescriptorTable, 
        bufs: &mut [[u8; ENDPOINT_BUF_SIZE]],
        rx: &mut [u8],
        tx: &mut [u8],
    ) -> Self {
        UsbDriver {
            usb: usb.into(),
            bdt: bdt,
            bufs: bufs,
            rx: Ring::new(rx),
            tx: Ring::new(tx),
            endpoints: [
                Cell::new(EndpointState(false, false)), 
                Cell::new(EndpointState(false, false)),
            ],
            _phantom: PhantomData,
        }
    }

    pub fn addr(&self) -> u8 {
        self.usb.addr().addr().value()
    }

    pub fn set_addr(&self, value: u8) {
        self.usb.set_addr(|r| r.set_addr(value));
    }

    pub fn endpoint_state(&self, ep: usize) -> EndpointState {
        self.endpoints[ep].get()
    }

    pub fn set_endpoint_state(&self, ep: usize, value: EndpointState) {
        self.endpoints[ep].set(value)
    }

    pub fn reset_endpoint_state(&self, ep: usize) {
        self.set_endpoint_state(ep, EndpointState(false, false));
    }

    pub fn toggle_endpoint_state(&self, ep: usize) {
        let mut e = self.endpoint_state(ep);
        e.0 = !e.0;
        e.1 = !e.1;
        self.set_endpoint_state(ep, e);
    }

    pub fn odd(&self, ep: usize) -> bool {
        self.endpoint_state(ep).0
    }

    pub fn set_odd(&self, ep: usize, value: bool) {
        let mut e = self.endpoint_state(ep);
        e.0 = value;
        self.set_endpoint_state(ep, e);
    }

    pub fn toggle_odd(&self, ep: usize) {
        let mut e = self.endpoint_state(ep);
        e.0 = !e.0;
        self.set_endpoint_state(ep, e);
    }

    pub fn data1(&self, ep: usize) -> bool {
        self.endpoint_state(ep).1
    }

    pub fn set_data1(&self, ep: usize, value: bool) {
        let mut e = self.endpoint_state(ep);
        e.1 = value;
        self.set_endpoint_state(ep, e);
    }

    pub fn toggle_data1(&self, ep: usize) {
        let mut e = self.endpoint_state(ep);
        e.1 = !e.1;
        self.set_endpoint_state(ep, e);
    }

    pub fn enable_irq<I: Irq>(&self, irq: &I) {        
        nvic::set_enabled(irq.irq_num() as usize, false);
        SCB.set_irq_handler(irq.irq_num() as usize, None);
        SCB.set_irq_handler(irq.irq_num() as usize, Some(irq.wrap(self)));
        nvic::set_enabled(irq.irq_num() as usize, true);
    }

    pub fn reset(&self) {
        self.usb.with_usbtrc0(|r| r.set_usbreset(1));
        while self.usb.usbtrc0().test_usbreset() {}
    }

    pub fn set_bdt(&self, bdt: u32) {
        self.usb.set_bdtpage1(|_| Bdtpage1((bdt >> 8) as u8));
        self.usb.set_bdtpage2(|_| Bdtpage2((bdt >> 16) as u8));
        self.usb.set_bdtpage3(|_| Bdtpage3((bdt >> 24) as u8));
    }

    pub fn bdt_base(&self) -> u32 {
        (self.usb.bdtpage1().0 as u32) << 8 |
        (self.usb.bdtpage2().0 as u32) << 16 |
        (self.usb.bdtpage3().0 as u32) << 24
    }

    pub fn enable(&self) {
        // USB Enable
        self.usb.with_ctl(|r| r.set_usbensofen(1));
        // Leave Suspend, Disable Weak Pulldowns
        self.usb.with_usbctrl(|r| r.set_susp(0).set_pde(0));
        // Enable USB Reset Interrupt
        self.usb.with_inten(|r| r.set_usbrsten(1));
        // Enable Resistor Pullup for USB Full Speed
        self.usb.with_control(|r| r.set_dppullupnonotg(1));
    }

    fn ep_buf(&self, index: usize) -> &[u8; ENDPOINT_BUF_SIZE] {
        unsafe { &(*self.bufs)[index] }
    }

    pub fn buffer(&self, ep: usize, tx: bool, odd: bool) -> &[u8] {
        unsafe { 
            slice::from_raw_parts(
                self.bd(ep, tx, odd).baddr().addr().value() as *const u8, 
                self.bd(ep, tx, odd).bdesc().bc().value() as usize
            )
        }
    }

    pub fn buffer_mut(&self, ep: usize, tx: bool, odd: bool) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(
                self.bd(ep, tx, odd).baddr().addr().value() as *mut u8, 
                self.bd(ep, tx, odd).bdesc().bc().value() as usize
            )
        }
    }

    pub fn bd(&self, ep: usize, tx: bool, odd: bool) -> &BufferDesc {
        unsafe {
            (*self.bdt).bd(ep, tx, odd)
        }
    }

    pub fn bd_mut(&self, ep: usize, tx: bool, odd: bool) -> &mut BufferDesc {
        unsafe {
            (*self.bdt).bd_mut(ep, tx, odd)
        }
    }

    pub fn tx(&self, ep: usize, data: &[u8]) {
        self.tx.enqueue(ep as u8);
        self.tx.enqueue(data.len() as u8);
        for d in data.iter() {
            self.tx.enqueue(*d);
        }
    }

    pub fn check_tx(&self) {
        // Ensure that at least ep + len are present
        if self.tx.len() < 2 { return }

        // Peek at the endpoint number and length
        let ep = self.tx[0] as usize;
        let len = self.tx[1] as usize;
        let odd = self.odd(ep);
        let data1 = self.data1(ep);

        // Return if a full packet isn't in the queue
        if self.tx.len() < 2 + len { return }

        // Return if the current BD is owned
        let tx_bd = self.bd_mut(ep, true, odd);
        if tx_bd.bdesc().test_own() { return }

        let ep = self.tx.dequeue().expect("TX Read EP") as usize;
        let len = self.tx.dequeue().expect("TX Read Len") as usize;
        
        // Set the length so that we can write to the buffer
        tx_bd.set_bdesc(|r| r.set_bc(len).set_data01(data1));

        // Write from the queue to the buffer
        if len > 0 {
            let tx_buf = self.buffer_mut(ep, true, odd);
            for i in 0..len {
                tx_buf[i] = self.tx.dequeue().expect("TX Read Data");
            }
        }        
        // println!("{:p}: {:?} {:?}", tx_bd, tx_bd.bdesc(), tx_bd.baddr());

        // Give ownership of BD
        tx_bd.with_bdesc(|r| r.set_own(1));

        // Update the endpoint's Even / Odd and Data0 / Data1 state
        self.toggle_endpoint_state(ep);

        // Restart the peripheral if suspended
        if self.usb.ctl().test_txsuspendtokenbusy() {
            self.usb.with_ctl(|r| r.set_txsuspendtokenbusy(0));
        }
    }

    // Reads the next packed from the receive queue, if available
    pub fn read_rx(&self, buf: &mut [u8]) -> Option<(u8, u8, u8)> {
        if self.rx.len() == 0 { return None }
        let ep = self.rx.dequeue().unwrap();
        let pid = self.rx.dequeue().unwrap();
        let len = self.rx.dequeue().unwrap();
        for i in 0..len {
            buf[i as usize] = self.rx.dequeue().unwrap();
        }
        Some((ep, pid, len))
    }

    pub fn next(&self) {
        self.check_tx();
    }
}

impl<'a> Poll for UsbDriver<'a> {
    fn poll(&self) {
        let istat = self.usb.istat();

        if istat.test_error() {
            self.usb.set_errstat(|_| Errstat(0xff));
            self.usb.set_istat(|r| r.set_error(1));
            panic!("USB ERROR: {:?}", self.usb.errstat());
        }

        if istat.test_usbrst() {
            self.usb.set_ctl(|r| r);
            self.set_addr(0);
            self.set_bdt(self.bdt as u32);

            for ep in 0..2 {
                let b_0 = self.ep_buf((ep * 4) + 0) as *const u8 as u32;
                let b_1 = self.ep_buf((ep * 4) + 1) as *const u8 as u32;
                let b_2 = self.ep_buf((ep * 4) + 2) as *const u8 as u32;
                let b_3 = self.ep_buf((ep * 4) + 3) as *const u8 as u32;

                self.bd_mut(ep, false, false)
                    .set_bdesc(|r| r.set_own(1).set_bc(ENDPOINT_BUF_SIZE).set_data01(0).set_dts(1))
                    .set_baddr(|r| r.set_addr(b_0));

                self.bd_mut(ep, false, true)
                    .set_bdesc(|r| r.set_own(1).set_bc(ENDPOINT_BUF_SIZE).set_data01(1).set_dts(1))
                    .set_baddr(|r| r.set_addr(b_1));  

                self.bd_mut(ep, true, false)
                    .set_bdesc(|r| r)
                    .set_baddr(|r| r.set_addr(b_2));

                self.bd_mut(ep, true, true)
                    .set_bdesc(|r| r)
                    .set_baddr(|r| r.set_addr(b_3));                        

                self.usb.with_endpt(ep, |r| r.set_eprxen(1).set_eptxen(1).set_ephshk(1));
                self.reset_endpoint_state(ep);
            }
            
            // Clear All Interrupts
            self.usb.set_errstat(|_| Errstat(0xff));
            self.usb.set_istat(|_| Istat(0xff));

            // Set address to zero
            self.usb.set_addr(|r| r.set_addr(0));

            // Enable interrupts
            self.usb.set_erren(|_| Erren(0xff));
            self.usb.with_inten(|r| r
                .set_usbrsten(1)
                .set_erroren(1)
                .set_softoken(1)
                .set_tokdneen(1)
                .set_sleepen(1)
                .set_stallen(1)
            );
            self.usb.set_ctl(|r| r.set_usbensofen(1));
        }

        if istat.test_softok() {
            self.usb.set_istat(|r| r.set_softok(1));
        }

        if istat.test_tokdne() {
            let stat = self.usb.stat();
            let ep = stat.endp().value();
            let tx = stat.test_tx();
            let odd = stat.test_odd();

            let bd = self.bd_mut(ep as usize, tx, odd); 
            let len = bd.bdesc().bc().value();
            
            // println!("BD: tx={} odd={} {:?}", tx, odd, bd.bdesc());
            
            // Read packet and add to RX queue
            self.rx.enqueue(ep as u8);
            self.rx.enqueue(bd.bdesc().tok_pid() as u8);
            self.rx.enqueue(len as u8);
            for b in self.buffer(ep as usize, tx, odd) {
                self.rx.enqueue(*b);
            }

            if !stat.test_tx() {
                // Reset RX Buffer Descriptor
                let data01 = bd.bdesc().data01();
                bd.set_bdesc(|r| r.set_own(1).set_bc(ENDPOINT_BUF_SIZE).set_data01(data01).set_dts(1));
            }

            self.usb.set_istat(|r| r.set_tokdne(1));
        }

        if istat.test_sleep() {
            self.usb.set_istat(|r| r.set_sleep(1));
        }

        if istat.test_stall() {
            self.usb.set_istat(|r| r.set_stall(1));
        }

        self.next();
    }
}