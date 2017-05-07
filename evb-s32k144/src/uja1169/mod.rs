use hal::lpspi;

pub mod registers;

pub struct Uja1169 {
    target: lpspi::Target,
}

pub fn device(target: lpspi::Target) -> Uja1169 {
    Uja1169 { target: target }
}

impl Uja1169 {    
    pub fn reg(&self) -> registers::Registers<Self> {
        registers::Registers::new(self)
    }

    pub fn read_register(&self, index: u8) -> u8 {
        self.target.send(((index as u16) << 9) | (1 << 8));
        self.target.recv() as u8
    }

    pub fn write_register(&self, index: u8, value: u8) {
        self.target.send(((index as u16) << 9) | (value as u16))
    }
}

impl registers::ReadWrite for Uja1169 {
    fn read(&self, index: u8) -> u8 {
        self.read_register(index)
    }
    fn write(&self, index: u8, value: u8) {
        self.write_register(index, value)
    }
}