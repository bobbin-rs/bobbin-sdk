use hal::lpspi;

pub mod registers;

#[derive(Debug)]
pub enum Mode {
    Sleep =   0b000,
    Standby = 0b100,
    Normal =  0b111
}

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
        self.target.send(((index as u16) << 9) | (value as u16));
        self.target.recv();
    }

    pub fn is_forced_normal_mode(&self) -> bool {
        self.reg().wds().fnms() != 0
    }

    pub fn is_software_development_mode(&self) -> bool {
        self.reg().wds().sdms() != 0
    }

    pub fn mode(&self) -> Mode {
        match self.reg().mc().mc() {
            0b000 => Mode::Sleep,
            0b100 => Mode::Standby,
            0b111 => Mode::Normal,
            m @ _ => panic!("Unexpected mode: {:02x}", m),
        }
    }

    pub fn set_mode(&self, mode: Mode) {
        self.reg().with_mc(|r| r.set_mc(mode as u8));
    }

    pub fn nvm_crc(&self, value: [u8; 2]) -> u8 {
        let mut data: u8;
        let mut crc: u8 = 0xff;
        for i in 0..2 {
            data = value[i] ^ crc;
            for _ in 0..8 {
                if data >= 128 {
                    data = data << 1;
                    data = data ^ 0x2f;
                } else {
                    data = data << 1;
                }
            }
            crc = data;
        }
        crc ^ 0xff
    }

    pub fn write_nvm(&self, value: [u8; 2]) {
        self.write_register(0x73, value[0]);
        self.write_register(0x74, value[1]);
        self.write_register(0x75, self.nvm_crc(value));
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