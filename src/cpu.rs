use super::memory::MemoryMap;
#[derive(Debug)]
struct Reg {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    pc: u16,
    sp: u16,
}

impl Reg {
    fn new() -> Reg {
        Reg {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0,
        }
    }

    fn hl_address(&self) -> usize {
        ((self.h as usize) << 8) + self.l as usize
    }
}

#[derive(Debug)]
struct Flag {
    z: bool,
    n: bool,
    h: bool,
    c: bool,
}

impl Flag {
    fn new() -> Flag {
        Flag {
            z: false,
            n: false,
            h: false,
            c: false,
        }
    }
}

pub struct Cpu<'m> {
    flag: Flag,
    reg: Reg,
    cycles: u32,
    memory: &'m MemoryMap,
}

impl Cpu<'_> {
    pub fn new(memory: &MemoryMap) -> Cpu<'_> {
        Cpu {
            flag: Flag::new(),
            reg: Reg::new(),
            cycles: 0,
            memory: memory,
        }
    }

    pub fn step(&mut self) {
        match self.memory.read(self.reg.pc as usize) {
            // 8 bit loads (Immediate)
            0x06 => {
                self.reg.b = self.memory.read(self.reg.pc as usize + 1);
                self.reg.pc += 2;
                self.cycles += 8;
            }
            0x0E => {
                self.reg.c = self.memory.read(self.reg.pc as usize + 1);
                self.reg.pc += 2;
                self.cycles += 8;
            }
            0x16 => {
                self.reg.d = self.memory.read(self.reg.pc as usize + 1);
                self.reg.pc += 2;
                self.cycles += 8;
            }
            0x1E => {
                self.reg.e = self.memory.read(self.reg.pc as usize + 1);
                self.reg.pc += 2;
                self.cycles += 8;
            }
            0x26 => {
                self.reg.h = self.memory.read(self.reg.pc as usize + 1);
                self.reg.pc += 2;
                self.cycles += 8;
            }
            0x2E => {
                self.reg.l = self.memory.read(self.reg.pc as usize + 1);
                self.reg.pc += 2;
                self.cycles += 8;
            }
            // 8 bit loads to A (Register)
            0x7F => {
                self.reg.a = self.reg.a;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x78 => {
                self.reg.a = self.reg.b;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x79 => {
                self.reg.a = self.reg.c;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x7A => {
                self.reg.a = self.reg.d;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x7B => {
                self.reg.a = self.reg.e;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x7C => {
                self.reg.a = self.reg.h;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x7D => {
                self.reg.a = self.reg.l;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x7E => {
                let address = self.reg.hl_address();
                self.reg.a = self.memory.read(address);
                self.reg.pc += 1;
                self.cycles += 8;
            }
            // 8 bit loads to B (Register)
            0x40 => {
                self.reg.b = self.reg.b;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x41 => {
                self.reg.b = self.reg.c;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x42 => {
                self.reg.b = self.reg.d;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x43 => {
                self.reg.b = self.reg.e;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x44 => {
                self.reg.b = self.reg.h;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x45 => {
                self.reg.b = self.reg.l;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x46 => {
                let address = self.reg.hl_address();
                self.reg.b = self.memory.read(address);
                self.reg.pc += 1;
                self.cycles += 8;
            }

            // 8 bit loads to C (Register)
            0x48 => {
                self.reg.c = self.reg.b;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x49 => {
                self.reg.c = self.reg.c;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x4A => {
                self.reg.c = self.reg.d;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x4B => {
                self.reg.c = self.reg.e;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x4C => {
                self.reg.c = self.reg.h;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x4D => {
                self.reg.c = self.reg.l;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x4E => {
                let address = self.reg.hl_address();
                self.reg.c = self.memory.read(address);
                self.reg.pc += 1;
                self.cycles += 8;
            }

            // 8 bit loads to D (Register)
            0x50 => {
                self.reg.d = self.reg.b;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x51 => {
                self.reg.d = self.reg.c;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x52 => {
                self.reg.d = self.reg.d;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x53 => {
                self.reg.d = self.reg.e;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x54 => {
                self.reg.d = self.reg.h;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x55 => {
                self.reg.d = self.reg.l;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x56 => {
                let address = self.reg.hl_address();
                self.reg.d = self.memory.read(address);
                self.reg.pc += 1;
                self.cycles += 8;
            }

            // 8 bit loads to E (Register)
            0x58 => {
                self.reg.e = self.reg.b;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x59 => {
                self.reg.e = self.reg.c;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x5A => {
                self.reg.e = self.reg.d;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x5B => {
                self.reg.e = self.reg.e;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x5C => {
                self.reg.e = self.reg.h;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x5D => {
                self.reg.e = self.reg.l;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x5E => {
                let address = self.reg.hl_address();
                self.reg.e = self.memory.read(address);
                self.reg.pc += 1;
                self.cycles += 8;
            }

            // 8 bit loads to H (Register)
            0x60 => {
                self.reg.h = self.reg.b;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x61 => {
                self.reg.h = self.reg.c;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x62 => {
                self.reg.h = self.reg.d;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x63 => {
                self.reg.h = self.reg.e;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x64 => {
                self.reg.h = self.reg.h;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x65 => {
                self.reg.h = self.reg.l;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x66 => {
                let address = self.reg.hl_address();
                self.reg.h = self.memory.read(address);
                self.reg.pc += 1;
                self.cycles += 8;
            }

            // 8 bit loads to L (Register)
            0x68 => {
                self.reg.l = self.reg.b;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x69 => {
                self.reg.l = self.reg.c;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x6A => {
                self.reg.l = self.reg.d;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x6B => {
                self.reg.l = self.reg.e;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x6C => {
                self.reg.l = self.reg.h;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x6D => {
                self.reg.l = self.reg.l;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x6E => {
                let address = self.reg.hl_address();
                self.reg.l = self.memory.read(address);
                self.reg.pc += 1;
                self.cycles += 8;
            }

            0x70 => {
                let address = self.reg.hl_address();
                self.memory.write(address, self.reg.b);
                self.reg.pc += 1;
                self.cycles += 8;
            }

            0x71 => {
                let address = self.reg.hl_address();
                self.memory.write(address, self.reg.c);
                self.reg.pc += 1;
                self.cycles += 8;
            }

            0x72 => {
                let address = self.reg.hl_address();
                self.memory.write(address, self.reg.d);
                self.reg.pc += 1;
                self.cycles += 8;
            }

            0x73 => {
                let address = self.reg.hl_address();
                self.memory.write(address, self.reg.e);
                self.reg.pc += 1;
                self.cycles += 8;
            }

            0x74 => {
                let address = self.reg.hl_address();
                self.memory.write(address, self.reg.h);
                self.reg.pc += 1;
                self.cycles += 8;
            }

            0x75 => {
                let address = self.reg.hl_address();
                self.memory.write(address, self.reg.l);
                self.reg.pc += 1;
                self.cycles += 8;
            }

            0x36 => {
                let address = self.reg.hl_address();
                let value = self.memory.read(self.reg.pc as usize + 1);
                self.memory.write(address, value);
                self.reg.pc += 2;
                self.cycles += 12;
            }

            _ => panic!("{} op code not implemented", self.reg.pc),
        }
    }
}

#[cfg(test)]
mod test;
