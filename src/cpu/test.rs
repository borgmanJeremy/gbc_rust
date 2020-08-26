use super::*;

#[test]
fn test_hl_dereference() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);
    cpu.reg.h = 0x80;
    cpu.reg.l = 0x12;

    let address = cpu.reg.hl_address();
    assert_eq!(address, 0x8012);
}

#[test]
fn load_immediate_b() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);
    cpu.memory.write(0, 0x06);
    cpu.memory.write(1, 0xFE);

    cpu.step();
    assert_eq!(cpu.reg.b, 0xFE);
    assert_eq!(cpu.reg.pc, 0x02);
    assert_eq!(cpu.cycles, 0x08);
}

#[test]
fn load_immediate_c() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x0E);
    cpu.memory.write(1, 0xFE);

    cpu.step();
    assert_eq!(cpu.reg.c, 0xFE);
    assert_eq!(cpu.reg.pc, 0x02);
    assert_eq!(cpu.cycles, 0x08);
}

#[test]
fn load_immediate_d() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x16);
    cpu.memory.write(1, 0xFE);

    cpu.step();
    assert_eq!(cpu.reg.d, 0xFE);
    assert_eq!(cpu.reg.pc, 0x02);
    assert_eq!(cpu.cycles, 0x08);
}

#[test]
fn load_immediate_e() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x1E);
    cpu.memory.write(1, 0xFE);

    cpu.step();
    assert_eq!(cpu.reg.e, 0xFE);
    assert_eq!(cpu.reg.pc, 0x02);
    assert_eq!(cpu.cycles, 0x08);
}

#[test]
fn load_immediate_h() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x26);
    cpu.memory.write(1, 0xFE);

    cpu.step();
    assert_eq!(cpu.reg.h, 0xFE);
    assert_eq!(cpu.reg.pc, 0x02);
    assert_eq!(cpu.cycles, 0x08);
}

#[test]
fn load_immediate_l() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x2E);
    cpu.memory.write(1, 0xFE);

    cpu.step();
    assert_eq!(cpu.reg.l, 0xFE);
    assert_eq!(cpu.reg.pc, 0x02);
    assert_eq!(cpu.cycles, 0x08);
}

#[test]
fn load_a_from_a() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x7F);
    cpu.reg.a = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.a, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_a_from_b() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x78);
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
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x79);
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
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x7A);
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
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x7B);
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
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x7C);
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
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x7D);
    cpu.reg.a = 0x00;
    cpu.reg.l = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.a, 0x01);
    assert_eq!(cpu.reg.l, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_a_from_hl() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x7E);
    cpu.memory.write(0x0124, 0x55);
    cpu.reg.a = 0x00;
    cpu.reg.h = 0x01;
    cpu.reg.l = 0x24;

    cpu.step();
    assert_eq!(cpu.reg.a, 0x55);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x08);
}

#[test]
fn load_b_from_b() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x40);
    cpu.reg.b = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.b, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_b_from_c() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x41);
    cpu.reg.b = 0x00;
    cpu.reg.c = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.b, 0x01);
    assert_eq!(cpu.reg.c, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_b_from_d() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x42);
    cpu.reg.b = 0x00;
    cpu.reg.d = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.b, 0x01);
    assert_eq!(cpu.reg.d, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}
#[test]
fn load_b_from_e() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x43);
    cpu.reg.b = 0x00;
    cpu.reg.e = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.b, 0x01);
    assert_eq!(cpu.reg.e, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_b_from_h() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x44);
    cpu.reg.b = 0x00;
    cpu.reg.h = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.b, 0x01);
    assert_eq!(cpu.reg.h, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_b_from_l() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x45);
    cpu.reg.b = 0x00;
    cpu.reg.l = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.b, 0x01);
    assert_eq!(cpu.reg.l, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_b_from_hl() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x46);
    cpu.memory.write(0x0124, 0x55);
    cpu.reg.b = 0x00;
    cpu.reg.h = 0x01;
    cpu.reg.l = 0x24;

    cpu.step();
    assert_eq!(cpu.reg.b, 0x55);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x08);
}
