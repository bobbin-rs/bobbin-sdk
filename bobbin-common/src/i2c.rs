pub trait WriteAddr<T> {
    fn write_addr(&self, addr: T, data: &[u8]) -> &Self;
}

pub trait ReadAddr<T> {
    fn read_addr(&self, addr: T, data: &mut [u8]) -> &Self;
}