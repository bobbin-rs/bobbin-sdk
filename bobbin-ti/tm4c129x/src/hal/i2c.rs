pub use bobbin_common::i2c::*;
pub use super::sysctl::SysctlEnabled;
pub use ::chip::i2c::*;

const BUSY_WAIT: usize = 50;

impl I2cPeriph {
    pub fn init_master(&self) -> &Self {
        self.master().set_mcr(|_| master::Mcr(0).set_mfe(1));
        // Set the desired SCL clock speed of 100 Kbps by writing the I2CMTPR register with the correct value.
        // The value written to the I2CMTPR register represents the number of system clock periods in one SCL 
        // clock period. The TPR value is determined by the following equation:
        // TPR = (System Clock/(2*(SCL_LP + SCL_HP)*SCL_CLK))-1;
        // TPR = (20MHz/(2*(6+4)*100000))-1;
        // TPR = 9
        // Write the I2CMTPR register with the value of 0x0000.0009.

        // Set to 400Khz for now
        // let clk = 400_000;
        // let tpr = (clock::sysclk_hz() / (2 * (6 + 4) * clk)) - 1;
        // TPR = 15
        let tpr = 15;
        self.master().set_mtpr(|_| master::Mtpr(0).set_tpr(tpr));
        self
    }
}

impl I2cTransfer<u8> for I2cPeriph {
    fn write(&self, addr: u8, data: &[u8]) -> &Self {
        self.master().set_msa(|_| master::Msa(0).set_sa(addr));    
        while self.master().mcs_read().busbsy() != 0 {}
        let len = data.len();
        for i in 0..data.len() {
            let start = if i == 0 { 1 } else { 0 };
            let stop = if i == len - 1 { 1 } else { 0 };
            self.master().set_mdr(|_| master::Mdr(0).set_data(data[i]));
            self.master().set_mcs_write(|_| master::McsWrite(0)
                .set_qcmd(0)
                .set_start(start)
                .set_stop(stop)
                .set_run(1)
            );
            // NOTE - BUSY will not change immediately after write, must wait.
            //while self.master().mcs_read().busy() == 0 {}
            unsafe { for _ in 0..BUSY_WAIT { asm!("nop")} }
            while self.master().mcs_read().busy() != 0 {}
        }
        self
    }

    fn read(&self, addr: u8, data: &mut [u8]) -> &Self {
        self.master().set_msa(|_| master::Msa(0).set_sa(addr).set_rs(1));
        let len = data.len();
        for i in 0..data.len() {
            let start = if i == 0 { 1 } else { 0 };
            let stop = if i == len - 1 { 1 } else { 0 };    
            let ack =  if i == len - 1 { 0 } else { 1 };
            //trace!("{}: start: {} stop: {} ack: {}", i, start, stop, ack);
            self.master().set_mcs_write(|_| master::McsWrite(0)
                .set_start(start)
                .set_stop(stop)
                .set_ack(ack)
                .set_run(1)
            );                
            // NOTE - BUSY will not change immediately after write, must wait.
            //while self.master().mcs_read().busy() == 0 {}
            unsafe { for _ in 0..BUSY_WAIT { asm!("nop")} }
            while self.master().mcs_read().busy() != 0 {}
            if self.master().mcs_read().error() != 0 {
                // trace!("Error: {:?}", self.master().mcs_read());
                self.master().set_mcs_write(|_| master::McsWrite(0)
                    .set_stop(1)
                    .set_run(0)
                );
                panic!("Error: {:?}", self.master().mcs_read());
            }
            
            data[i] = self.master().mdr().data().value()
        }
        self                    
    }
    fn transfer(&self, addr: u8, data_out: &[u8], data_in: &mut [u8]) -> &Self {
        while self.master().mcs_read().busbsy() != 0 {}
        self.master().set_msa(|_| master::Msa(0).set_sa(addr));

        let len = data_out.len();
        for i in 0..data_out.len() {
            let start = if i == 0 || i == len - 1 { 1 } else { 0 };
            let stop = 0;
            self.master().set_mdr(|_| master::Mdr(0).set_data(data_out[i]));
            self.master().set_mcs_write(|_| master::McsWrite(0)
                .set_start(start)
                .set_stop(stop)
                .set_run(1)
            );
            // NOTE - BUSY will not change immediately after write, must wait.
            //while self.master().mcs_read().busy() == 0 {}
            unsafe { for _ in 0..BUSY_WAIT { asm!("nop") } }
            while self.master().mcs_read().busy() != 0 {}
        }
        self.master().set_msa(|_| master::Msa(0).set_sa(addr).set_rs(1));
        let len = data_in.len();
        for i in 0..data_in.len() {
            let start = if i == 0 { 1 } else { 0 };
            let stop = if i == len - 1 { 1 } else { 0 };    
            let ack =  if i == len - 1 { 0 } else { 1 };
            self.master().set_mcs_write(|_| master::McsWrite(0)
                .set_start(start)
                .set_stop(stop)
                .set_ack(ack)
                .set_run(1)
            );
            // NOTE - BUSY will not change immediately after write, must wait.
            //while self.master().mcs_read().busy() == 0 {}
            unsafe { for _ in 0..BUSY_WAIT { asm!("nop") } }
            while self.master().mcs_read().busy() != 0 {}
            data_in[i] = self.master().mdr().data().value();
        }
        self            
    }    
}