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
struct Cpu {
    flag: Flag,
    reg: Reg,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            flag: Flag::new(),
            reg: Reg::new(),
        }
    }
}

fn main() {
    let instruction = 0x06;
    let mut cpu = Cpu::new();
    match instruction {
        0x06 => {
            cpu.reg.b = 2;
        }
        _ => panic!("{} op code not implemented", instruction),
    }
}
