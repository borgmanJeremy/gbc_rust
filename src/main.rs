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

#[derive(Debug)]
struct MemoryMap {
    map: Vec<u8>,
}

impl MemoryMap {
    fn new(memory_size: usize) -> MemoryMap {
        let mut map = Vec::new();
        map.resize(memory_size, 0);
        MemoryMap { map }
    }
}

#[derive(Debug)]
pub struct Cpu {
    flag: Flag,
    reg: Reg,
    cycles: u32,
    memory: MemoryMap,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            flag: Flag::new(),
            reg: Reg::new(),
            cycles: 0,
            memory: MemoryMap::new(0xFFFF),
        }
    }

    pub fn step(&mut self) {
        match self.memory.map[self.reg.pc as usize] {
            // 8 bit loads (Immediate)
            0x06 => {
                self.reg.b = self.memory.map[self.reg.pc as usize + 1];
                self.reg.pc += 2;
                self.cycles += 8;
            }
            0x0E => {
                self.reg.c = self.memory.map[self.reg.pc as usize + 1];
                self.reg.pc += 2;
                self.cycles += 8;
            }
            0x16 => {
                self.reg.d = self.memory.map[self.reg.pc as usize + 1];
                self.reg.pc += 2;
                self.cycles += 8;
            }
            0x1E => {
                self.reg.e = self.memory.map[self.reg.pc as usize + 1];
                self.reg.pc += 2;
                self.cycles += 8;
            }
            0x26 => {
                self.reg.h = self.memory.map[self.reg.pc as usize + 1];
                self.reg.pc += 2;
                self.cycles += 8;
            }
            0x2E => {
                self.reg.l = self.memory.map[self.reg.pc as usize + 1];
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
                
                self.reg.a = self.memory.map[address];
                self.reg.pc += 1;
                self.cycles += 8;
            }

            _ => panic!("{} op code not implemented", self.reg.pc),
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hl_dereference() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x80;
        cpu.reg.l = 0x12;

        let address = cpu.reg.hl_address();
        assert_eq!(address, 0x8012);
    }

    #[test]
    fn load_immediate_b() {
        let mut cpu = Cpu::new();
        cpu.memory.map[0] = 0x06;
        cpu.memory.map[1] = 0xFE;

        cpu.step();
        assert_eq!(cpu.reg.b, 0xFE);
        assert_eq!(cpu.reg.pc, 0x02);
        assert_eq!(cpu.cycles, 0x08);
    }

    #[test]
    fn load_immediate_c() {
        let mut cpu = Cpu::new();
        cpu.memory.map[0] = 0x0E;
        cpu.memory.map[1] = 0xFE;

        cpu.step();
        assert_eq!(cpu.reg.c, 0xFE);
        assert_eq!(cpu.reg.pc, 0x02);
        assert_eq!(cpu.cycles, 0x08);
    }

    #[test]
    fn load_immediate_d() {
        let mut cpu = Cpu::new();
        cpu.memory.map[0] = 0x16;
        cpu.memory.map[1] = 0xFE;

        cpu.step();
        assert_eq!(cpu.reg.d, 0xFE);
        assert_eq!(cpu.reg.pc, 0x02);
        assert_eq!(cpu.cycles, 0x08);
    }

    #[test]
    fn load_immediate_e() {
        let mut cpu = Cpu::new();
        cpu.memory.map[0] = 0x1E;
        cpu.memory.map[1] = 0xFE;

        cpu.step();
        assert_eq!(cpu.reg.e, 0xFE);
        assert_eq!(cpu.reg.pc, 0x02);
        assert_eq!(cpu.cycles, 0x08);
    }

    #[test]
    fn load_immediate_h() {
        let mut cpu = Cpu::new();
        cpu.memory.map[0] = 0x26;
        cpu.memory.map[1] = 0xFE;

        cpu.step();
        assert_eq!(cpu.reg.h, 0xFE);
        assert_eq!(cpu.reg.pc, 0x02);
        assert_eq!(cpu.cycles, 0x08);
    }

    #[test]
    fn load_immediate_l() {
        let mut cpu = Cpu::new();
        cpu.memory.map[0] = 0x2E;
        cpu.memory.map[1] = 0xFE;

        cpu.step();
        assert_eq!(cpu.reg.l, 0xFE);
        assert_eq!(cpu.reg.pc, 0x02);
        assert_eq!(cpu.cycles, 0x08);
    }

    #[test]
    fn load_a_from_a() {
        let mut cpu = Cpu::new();
        cpu.memory.map[0] = 0x7F;
        cpu.reg.a = 0x01;

        cpu.step();
        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.pc, 0x01);
        assert_eq!(cpu.cycles, 0x04);
    }

    #[test]
    fn load_a_from_b() {
        let mut cpu = Cpu::new();
        cpu.memory.map[0] = 0x78;
        cpu.reg.a = 0x00;
        cpu.reg.b = 0x01;

        cpu.step();
        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.b, 0x01);
        assert_eq!(cpu.reg.pc, 0x01);
        assert_eq!(cpu.cycles, 0x04);
    }

    #[test]
    fn load_a_from_c() {
        let mut cpu = Cpu::new();
        cpu.memory.map[0] = 0x79;
        cpu.reg.a = 0x00;
        cpu.reg.c = 0x01;

        cpu.step();
        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.c, 0x01);
        assert_eq!(cpu.reg.pc, 0x01);
        assert_eq!(cpu.cycles, 0x04);
    }

    #[test]
    fn load_a_from_d() {
        let mut cpu = Cpu::new();
        cpu.memory.map[0] = 0x7A;
        cpu.reg.a = 0x00;
        cpu.reg.d = 0x01;

        cpu.step();
        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.d, 0x01);
        assert_eq!(cpu.reg.pc, 0x01);
        assert_eq!(cpu.cycles, 0x04);
    }
    #[test]
    fn load_a_from_e() {
        let mut cpu = Cpu::new();
        cpu.memory.map[0] = 0x7B;
        cpu.reg.a = 0x00;
        cpu.reg.e = 0x01;

        cpu.step();
        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.e, 0x01);
        assert_eq!(cpu.reg.pc, 0x01);
        assert_eq!(cpu.cycles, 0x04);
    }

    #[test]
    fn load_a_from_h() {
        let mut cpu = Cpu::new();
        cpu.memory.map[0] = 0x7C;
        cpu.reg.a = 0x00;
        cpu.reg.h = 0x01;

        cpu.step();
        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.h, 0x01);
        assert_eq!(cpu.reg.pc, 0x01);
        assert_eq!(cpu.cycles, 0x04);
    }

    #[test]
    fn load_a_from_l() {
        let mut cpu = Cpu::new();
        cpu.memory.map[0] = 0x7D;
        cpu.reg.a = 0x00;
        cpu.reg.l = 0x01;

        cpu.step();
        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.l, 0x01);
        assert_eq!(cpu.reg.pc, 0x01);
        assert_eq!(cpu.cycles, 0x04);
    }
}
