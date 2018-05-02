//! An example showing how to use bobbin-hal to create and instantiate an interrupt-driven serial
//! driver. This version assumes that the serial device and tx / rx pins have already been enabled 
//! and configured.
//! 
use bobbin_mcu::prelude::*;
use bobbin_hal::prelude::*;
use bobbin_sys::prelude::*;
use bobbin_sys::heap::Error;

pub struct EchoApp<MCU, USART> 
where
    MCU: Mcu,
    USART: 'static + SerialTx<u8> + SerialTxIrq + SerialRx<u8> + SerialRxIrq + Sync,
{
    s: SerialDriver<MCU, USART>,
}

impl<MCU, USART> EchoApp<MCU, USART> 
where
    MCU: Mcu,
    USART: 'static + SerialTx<u8> + SerialTxIrq + SerialRx<u8> + SerialRxIrq + Sync,
{
    pub fn new<S: SystemProvider, U: Into<USART> + Irq<I>, I: IrqType>(sys: &mut System<S>, usart: U, irq_type: I) -> Result<Self, Error> {
        Ok(
            EchoApp {
                s: SerialDriver::new(sys, usart, irq_type)?,
            }
        )
    }
    pub fn run(&mut self) -> ! {
        let s = &mut self.s;
        s.write_all(b"Serial Driver Echo Test\r\n");      
        let mut buf = [0u8; 64];
        loop {
            let n = s.read(&mut buf);
            if n > 0 {
                for b in &buf[..n] {
                    if *b == 13 {
                        s.write_all(b"\r\n");
                    } else {
                        s.write_all(&[*b]);
                    }
                }
            }
            s.sleep();
        }        
    }
}


pub struct Config {
    pub tx_len: usize,
    pub rx_len: usize,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            tx_len: 64,
            rx_len: 64,
        }
    }
}

pub struct SerialDriver<MCU, USART> 
where
    MCU: Mcu,    
    USART: 'static + SerialTx<u8> + SerialTxIrq + SerialRx<u8> + SerialRxIrq + Sync,
{
    guard: Guard<'static, SerialHandler<USART>, MCU>,
    tx_ring: &'static Ring<'static, u8>,
    rx_ring: &'static Ring<'static, u8>,
}

impl<MCU, USART> SerialDriver<MCU, USART> 
where
    MCU: Mcu,
    USART: 'static + SerialTx<u8> + SerialTxIrq + SerialRx<u8> + SerialRxIrq + Sync,
{
    pub fn new<S: SystemProvider<Mcu=MCU>, U: Into<USART> + Irq<I>, I: IrqType>(sys: &mut System<S>, usart: U, irq_type: I) -> Result<Self, Error> 
    {
        Self::new_with_config(sys, usart, irq_type, Config::default())
    }

    pub fn new_with_config<S: SystemProvider<Mcu=MCU>, U: Into<USART> + Irq<I>, I: IrqType>(sys: &mut System<S>, usart: U, _irq_type: I, cfg: Config) -> Result<Self, Error> 
    {
        // let irq_number = USART.irq_number_for(irq_type);
        let irq_number = <U as Irq<I>>::Output::irq_number();

        let tx_buf = sys.heap_mut().try_slice(0u8, cfg.tx_len)?;
        let tx_ring = sys.heap_mut().try_new(Ring::new(tx_buf))?;
        let tx_reader = sys.heap_mut().try_new(tx_ring.reader())?;

        let rx_buf = sys.heap_mut().try_slice(0u8, cfg.rx_len)?;
        let rx_ring = sys.heap_mut().try_new(Ring::new(rx_buf))?;
        let rx_writer = sys.heap_mut().try_new(rx_ring.writer())?;        
        let handler = sys.heap_mut().try_new(SerialHandler::new(usart, tx_reader, rx_writer))?;
        let guard = sys.dispatcher_mut().register_handler(irq_number, handler).unwrap_or_abort("Unable to register IRQ handler");
        Ok(Self { guard, tx_ring, rx_ring })
    }

    #[inline]
    fn sleep(&self) {
        MCU::sleep()
    }    

    pub fn write_all(&mut self, buf: &[u8]) -> usize {                
        let mut n = 0;
        while n < buf.len() {            
            let sent = self.write(&buf[n..]);            
            if sent == 0 {
                self.sleep();
            } else {
                n += sent;
            }
        }
        n
    }

    pub fn write(&mut self, buf: &[u8]) -> usize {
        let len = self.tx_ring.write(buf);
        self.guard.tx_start();        
        len
    }

    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        let len = self.rx_ring.read(buf);
        self.guard.rx_start();
        len
    }    
}

pub struct SerialHandler<P: SerialTx<u8> + SerialTxIrq + SerialRx<u8> + SerialRxIrq + Sync> {
    usart: P,
    reader: &'static Reader<'static, u8>,
    writer: &'static Writer<'static, u8>,
}

impl<P: SerialTx<u8> + SerialTxIrq + SerialRx<u8> + SerialRxIrq + Sync> SerialHandler<P> {
    pub fn new<U: Into<P>>(usart: U, reader: &'static Reader<'static, u8>, writer: &'static Writer<'static, u8>) -> Self {
        let usart = usart.into();
        Self { usart, reader, writer }
    }

    pub fn tx_start(&self) {
        self.usart.enable_tx_irq();
    }

    pub fn tx_stop(&self) {
        self.usart.disable_tx_irq();
    }

    pub fn rx_start(&self) {
        self.usart.enable_rx_irq();
    }

    pub fn rx_stop(&self) {
        self.usart.disable_rx_irq();
    }
}

impl<P: SerialTx<u8> + SerialTxIrq + SerialRx<u8> + SerialRxIrq + Sync> HandleIrq for SerialHandler<P> {
    fn handle_irq(&self, _: u8) {              
        if self.usart.can_tx() {
            if let Some(b) = self.reader.get() {
                self.usart.tx(b);
            } else {
                self.tx_stop();
            }
        }
        if self.usart.can_rx() {
            if self.writer.rem() > 0 {
                self.writer.put(self.usart.rx());
            } else {
                self.rx_stop();
            }
        }        
        // if self.usart.isr().test_ore() {
        //     self.usart.with_icr(|r| r.set_orecf(1));
        // }
    }
}