use std::prelude::v1::Vec;
use std::fmt;
use std::mem;
use std::ptr;


pub struct Vm {
    regions: Vec<Region>,
}

impl Vm {
    pub fn new() -> Self {
        Vm {
            regions: Vec::new(),
        }
    }

    pub fn at(&self, addr: usize) -> Option<&Region> {
        self.overlaps(addr, 0)
    }

    pub fn at_mut(&mut self, addr: usize) -> Option<&mut Region> {
        self.overlaps_mut(addr, 0)
    }

    pub fn overlaps(&self, addr: usize, len: usize) -> Option<&Region> {
        for r in self.regions.iter() {
            if r.overlaps(addr, len) {
                return Some(r)
            }
        }
        None
    }

    pub fn overlaps_mut(&mut self, addr: usize, len: usize) -> Option<&mut Region> {
        for r in self.regions.iter_mut() {
            if r.overlaps(addr, len) {
                return Some(r)
            }
        }
        None
    }    

    pub fn add_region(&mut self, addr: usize, len: usize) {
        if let Some(r) = self.overlaps(addr, len) {
            panic!("New region overlaps with existing region {:?}", r);
        }
        self.regions.push(Region::new(addr, len));                
    }

    pub fn read<T>(&self, addr: *const T) -> T {
        let addr = addr as usize;
        let len: usize = mem::size_of::<T>();
        if let Some(r) = self.overlaps(addr, len) {
            let start = addr - r.addr;
            let end = start + len;
            let s = &r.memory[start..end];
            unsafe { ptr::read(s.as_ptr() as *const T) }
        } else {
            panic!("Attempted read out of bounds at 0x{:08x}", addr)
        }
    }

    pub fn write<T>(&mut self, addr: *mut T, value: T) {
        let addr = addr as usize;
        let len: usize = mem::size_of::<T>();
        if let Some(r) = self.overlaps_mut(addr, len) {
            let start = addr - r.addr;
            let end = start + len;
            let s = &mut r.memory[start..end];
            unsafe { ptr::write(s.as_mut_ptr() as *mut T, value) }
        } else {
            panic!("Attempted write out of bounds at 0x{:08x}", addr)
        }
    }
}

pub struct Region {
    addr: usize,
    memory: Vec<u8>,
}

impl Region {
    pub fn new(addr: usize, len: usize) -> Self {
        let mut memory = Vec::with_capacity(len);
        memory.resize(len, 0);
        Region {
            addr: addr,
            memory: memory,
        }
    }

    pub fn len(&self) -> usize {
        self.memory.len()
    }

    pub fn end(&self) -> usize {
        self.addr + self.len() - 1
    }

    pub fn overlaps(&self, addr: usize, len: usize) -> bool {
        addr >= self.addr && (addr + len) <= (self.addr + self.len())
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.memory
    }
    
    pub fn as_slice_mut(&mut self) -> &mut[u8] {
        &mut self.memory
    }
}

impl fmt::Debug for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:08x}-{:08x}]", self.addr, self.end())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlaps() {
        let mut vm = Vm::new();
        vm.add_region(0x0000, 0x1000);
        assert_eq!(vm.overlaps(0x0000, 0x1).is_some(), true);

        assert_eq!(vm.overlaps(0x1000, 0x1).is_some(), false);
        assert_eq!(vm.overlaps(0x1000-1, 0x1).is_some(), true);

        assert_eq!(vm.overlaps(0x1000-1, 0x2).is_some(), false);
        assert_eq!(vm.overlaps(0x1000-2, 0x2).is_some(), true);        

        assert_eq!(vm.overlaps(0x1000-3, 0x4).is_some(), false);
        assert_eq!(vm.overlaps(0x1000-4, 0x4).is_some(), true);        

        vm.add_region(0x2000, 0x1000);
        assert_eq!(vm.overlaps(0x2000, 0x1).is_some(), true);
        assert_eq!(vm.overlaps(0x2000-1, 0x1).is_some(), false);

        assert_eq!(vm.overlaps(0x3000, 0x1).is_some(), false);
        assert_eq!(vm.overlaps(0x3000-1, 0x1).is_some(), true);
    }

    #[test]
    fn test_read_u8() {
        let mut vm = Vm::new();
        vm.add_region(0x1000, 0x1000);

        {
            let r = vm.at_mut(0x1000).unwrap();
            let s = r.as_slice_mut();
            for i in 0..0x1000 {
                s[i] = i as u8;
            }
        }

        for i in 0..0x1000 {
            let v: u8 = vm.read((0x1000 + i) as *const u8);
            assert_eq!(v, i as u8);
        }
    }

    #[test]
    fn test_write_u8() {
        let mut vm = Vm::new();
        vm.add_region(0x1000, 0x1000);

        for i in 0..0x1000 {
            vm.write((0x1000 + i) as *mut u8, i as u8);
        }
        let r = vm.at(0x1000).unwrap();
        let s = r.as_slice();
        for i in 0..0x1000 {
            assert_eq!(s[i], i as u8);
        }
    }
}