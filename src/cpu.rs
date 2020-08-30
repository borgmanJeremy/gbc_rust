use super::memory::MemoryMap;
use std::num::Wrapping;

enum OperationType {
    B8,
    B16,
}

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
    fn bc_address(&self) -> usize {
        ((self.b as usize) << 8) + self.c as usize
    }

    fn de_address(&self) -> usize {
        ((self.d as usize) << 8) + self.e as usize
    }

    fn hl_address(&self) -> usize {
        ((self.h as usize) << 8) + self.l as usize
    }

    fn decrement_hl(&mut self) {
        let mut long = Wrapping(((self.h as u16) << 8) + self.l as u16);
        long = long - Wrapping(1u16);

        self.l = (long.0 & 0x00FF) as u8;
        self.h = ((long.0 >> 8) & 0x00FF) as u8;
    }

    fn increment_hl(&mut self) {
        let mut long = Wrapping(((self.h as u16) << 8) + self.l as u16);
        long = long + Wrapping(1u16);

        self.l = (long.0 & 0x00FF) as u8;
        self.h = ((long.0 >> 8) & 0x00FF) as u8;
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
            z: false, // zero flag
            n: false, // subtract flag
            h: false, // half carry
            c: false, // cary flag
        }
    }

    fn set_zero_flag(&mut self) {
        self.z = true;
    }
    fn clear_zero_flag(&mut self) {
        self.z = false;
    }

    fn adjust_zero_flag(&mut self, result: u16) {
        if result & 0xFF == 0 {
            self.z = true;
        } else {
            self.z = false;
        }
    }
    fn adjust_carry_flag(&mut self, result: u32, op_type: OperationType) {
        match op_type {
            OperationType::B8 => {
                if result > 0xFF {
                    self.c = true;
                } else {
                    self.c = false;
                }
            }
            OperationType::B16 => {
                if result > 0xFFFF {
                    self.c = true;
                } else {
                    self.c = false;
                }
            }
        }
    }

    fn adjust_half_carry_flag(&mut self, input: u8, result: u32) {
        if input & 0x8 != 0 {
            if result & 0x10 != 0 {
                self.h = true;
            } else {
                self.h = false
            }
        }
        // Can't carry out if the bit wasnt one before
        else {
            self.h = false;
        }
    }

    fn set_subtract_flag(&mut self) {
        self.n = true;
    }
    fn clear_subtract_flag(&mut self) {
        self.n = false;
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

    fn push_to_stack(&mut self, value: u8) {
        self.memory.write(self.reg.sp as usize - 1, value);

        self.reg.sp = self.reg.sp - 1;
    }

    fn two_byte_address(&self, base_address: usize) -> usize {
        ((self.memory.read(base_address + 1) as usize) << 8)
            + self.memory.read(base_address) as usize
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

            //A Loads from dereferenced
            0x0A => {
                let address = self.reg.bc_address();
                let value = self.memory.read(address);
                self.reg.a = value;
                self.reg.pc += 1;
                self.cycles += 8;
            }
            0x1A => {
                let address = self.reg.de_address();
                let value = self.memory.read(address);
                self.reg.a = value;
                self.reg.pc += 1;
                self.cycles += 8;
            }

            0xFA => {
                let address = self.two_byte_address(self.reg.pc as usize + 1);
                let value = self.memory.read(address);
                self.reg.a = value;
                self.reg.pc += 3;
                self.cycles += 16;
            }

            0x3E => {
                self.reg.a = self.memory.read(self.reg.pc as usize + 1);
                self.reg.pc += 2;
                self.cycles += 8;
            }

            0x47 => {
                self.reg.b = self.reg.a;
                self.reg.pc += 1;
                self.cycles += 4;
            }
            0x4F => {
                self.reg.c = self.reg.a;
                self.reg.pc += 1;
                self.cycles += 4;
            }

            0x57 => {
                self.reg.d = self.reg.a;
                self.reg.pc += 1;
                self.cycles += 4;
            }

            0x5F => {
                self.reg.e = self.reg.a;
                self.reg.pc += 1;
                self.cycles += 4;
            }

            0x67 => {
                self.reg.h = self.reg.a;
                self.reg.pc += 1;
                self.cycles += 4;
            }

            0x6F => {
                self.reg.l = self.reg.a;
                self.reg.pc += 1;
                self.cycles += 4;
            }

            0x02 => {
                let address = self.reg.bc_address();
                self.memory.write(address, self.reg.a);
                self.reg.pc += 1;
                self.cycles += 8;
            }
            0x12 => {
                let address = self.reg.de_address();
                self.memory.write(address, self.reg.a);
                self.reg.pc += 1;
                self.cycles += 8;
            }

            0x77 => {
                let address = self.reg.hl_address();
                self.memory.write(address, self.reg.a);
                self.reg.pc += 1;
                self.cycles += 8;
            }

            0xEA => {
                let address = self.two_byte_address(self.reg.pc as usize + 1);
                self.memory.write(address, self.reg.a);
                self.reg.pc += 3;
                self.cycles += 16;
            }

            0xF2 => {
                let value = self.memory.read(0xFF00 + self.reg.c as usize);
                self.reg.a = value;
                self.reg.pc += 1;
                self.cycles += 8;
            }

            0xE2 => {
                self.memory.write(0xFF00 + self.reg.c as usize, self.reg.a);
                self.reg.pc += 1;
                self.cycles += 8;
            }

            0x3A => {
                let address = self.reg.hl_address();
                self.reg.a = self.memory.read(address);
                self.reg.decrement_hl();
                self.reg.pc += 1;
                self.cycles += 8;
            }

            0x32 => {
                let address = self.reg.hl_address();
                self.memory.write(address, self.reg.a);
                self.reg.decrement_hl();
                self.reg.pc += 1;
                self.cycles += 8;
            }

            0x2A => {
                let address = self.reg.hl_address();
                self.reg.a = self.memory.read(address);
                self.reg.increment_hl();
                self.reg.pc += 1;
                self.cycles += 8;
            }

            0x22 => {
                let address = self.reg.hl_address();
                self.memory.write(address, self.reg.a);
                self.reg.increment_hl();
                self.reg.pc += 1;
                self.cycles += 8;
            }

            0xE0 => {
                self.memory.write(
                    0xFF00 + self.memory.read((self.reg.pc + 1) as usize) as usize,
                    self.reg.a,
                );
                self.reg.pc += 2;
                self.cycles += 12;
            }

            0xF0 => {
                let pc_val = self.memory.read((self.reg.pc + 1) as usize);
                let value = self.memory.read(0xFF00 + pc_val as usize);
                self.reg.a = value;
                self.reg.pc += 2;
                self.cycles += 12;
            }

            0x01 => {
                self.reg.b = self.memory.read((self.reg.pc + 2) as usize);
                self.reg.c = self.memory.read((self.reg.pc + 1) as usize);

                self.reg.pc += 3;
                self.cycles += 12;
            }

            0x11 => {
                self.reg.d = self.memory.read((self.reg.pc + 2) as usize);
                self.reg.e = self.memory.read((self.reg.pc + 1) as usize);

                self.reg.pc += 3;
                self.cycles += 12;
            }

            0x21 => {
                self.reg.h = self.memory.read((self.reg.pc + 2) as usize);
                self.reg.l = self.memory.read((self.reg.pc + 1) as usize);

                self.reg.pc += 3;
                self.cycles += 12;
            }

            0x31 => {
                self.reg.sp = ((self.memory.read((self.reg.pc + 2) as usize) as u16) << 8)
                    + self.memory.read((self.reg.pc + 1) as usize) as u16;

                self.reg.pc += 3;
                self.cycles += 12;
            }

            0xF9 => {
                self.reg.sp = self.reg.hl_address() as u16;

                self.reg.pc += 1;
                self.cycles += 8;
            }

            0xF8 => {
                let orig = self.reg.sp;
                let n = self.memory.read((self.reg.pc + 1) as usize) as u16;
                let result = orig as u32 + n as u32;

                self.reg.l = result as u8 & 0xFF;
                self.reg.h = (result >> 8) as u8 & 0xFF;

                self.flag.clear_zero_flag();
                self.flag.clear_subtract_flag();
                self.flag.adjust_half_carry_flag(orig as u8, result);
                self.flag.adjust_carry_flag(result, OperationType::B16);

                self.reg.pc += 2;
                self.cycles += 12;
            }

            0x08 => {
                let address = self.two_byte_address((self.reg.pc + 1) as usize);

                self.memory.write(address, (self.reg.sp & 0xFF) as u8);
                self.memory
                    .write(address + 1, ((self.reg.sp >> 8) & 0xFF) as u8);
                self.reg.pc += 3;
                self.cycles += 20;
            }

            0xF5 => {
                let mut flag_reg: u8 = 0;

                if self.flag.z == true {
                    flag_reg |= 0x01;
                }

                if self.flag.n == true {
                    flag_reg |= 0x02;
                }

                if self.flag.h == true {
                    flag_reg |= 0x04;
                }

                if self.flag.c == true {
                    flag_reg |= 0x09;
                }

                self.push_to_stack(self.reg.a);
                self.push_to_stack(flag_reg);

                self.reg.pc += 1;
                self.cycles += 16;
            }

            0xC5 => {
                self.push_to_stack(self.reg.b);
                self.push_to_stack(self.reg.c);
                self.reg.pc += 1;
                self.cycles += 16;
            }
            0xD5 => {
                self.push_to_stack(self.reg.d);
                self.push_to_stack(self.reg.e);
                self.reg.pc += 1;
                self.cycles += 16;
            }

            0xE5 => {
                self.push_to_stack(self.reg.h);
                self.push_to_stack(self.reg.l);
                self.reg.pc += 1;
                self.cycles += 16;
            }

            _ => panic!("{} op code not implemented", self.reg.pc),
        }
    }
}

#[cfg(test)]
mod test;
