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
    memory: MemoryMap,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            flag: Flag::new(),
            reg: Reg::new(),
            memory: MemoryMap::new(0xFFFF),
        }
    }

    pub fn step(&mut self) {
        match self.memory.map[self.reg.pc as usize] {
            0x06 => {
                self.reg.b = self.memory.map[self.reg.pc as usize + 1];
                self.reg.pc += 2;
            }
            _ => panic!("{} op code not implemented", self.reg.pc),
        }
    }
}

fn main() {
    let mut cpu = Cpu::new();
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn load_b() {
        let instruction = 0x06;
        let mut cpu = Cpu::new();
        cpu.memory.map[0] = 0x06;
        cpu.memory.map[1] = 0xFE;

        cpu.step();
        assert_eq!(cpu.reg.b , 0xFE);
    }
}
