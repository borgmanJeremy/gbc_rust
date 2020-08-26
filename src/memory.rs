use std::cell::RefCell;

#[derive(Debug)]
pub struct MemoryMap {
    pub mem: RefCell<Vec<u8>>,
}

impl MemoryMap {
    pub fn new(memory_size: usize) -> MemoryMap {
        let mut mem = Vec::new();
        mem.resize(memory_size, 0);
        MemoryMap {
            mem: RefCell::new(mem),
        }
    }

    pub fn write(&self, location: usize, value: u8) {
        let mut my_ref = self.mem.borrow_mut();
        my_ref[location] = value;
    }

    pub fn read(&self, location: usize) -> u8 {
        let my_ref = self.mem.borrow();
        return my_ref[location];
    }
}
