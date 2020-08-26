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

#[test]
fn load_c_from_b() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x48);
    cpu.reg.b = 0x01;
    cpu.reg.c = 0x00;
    cpu.step();
    assert_eq!(cpu.reg.b, 0x01);
    assert_eq!(cpu.reg.c, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_c_from_c() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x49);
    cpu.reg.c = 0x01;
    cpu.reg.c = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.c, 0x01);
    assert_eq!(cpu.reg.c, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_c_from_d() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x4A);
    cpu.reg.c = 0x00;
    cpu.reg.d = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.c, 0x01);
    assert_eq!(cpu.reg.d, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}
#[test]
fn load_c_from_e() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x4B);
    cpu.reg.c = 0x00;
    cpu.reg.e = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.c, 0x01);
    assert_eq!(cpu.reg.e, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_c_from_h() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x4C);
    cpu.reg.c = 0x00;
    cpu.reg.h = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.c, 0x01);
    assert_eq!(cpu.reg.h, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_c_from_l() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x4D);
    cpu.reg.c = 0x00;
    cpu.reg.l = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.c, 0x01);
    assert_eq!(cpu.reg.l, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_c_from_hl() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x4E);
    cpu.memory.write(0x0124, 0x55);
    cpu.reg.c = 0x00;
    cpu.reg.h = 0x01;
    cpu.reg.l = 0x24;

    cpu.step();
    assert_eq!(cpu.reg.c, 0x55);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x08);
}

#[test]
fn load_d_from_b() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x50);
    cpu.reg.b = 0x01;
    cpu.reg.d = 0x00;
    cpu.step();
    assert_eq!(cpu.reg.b, 0x01);
    assert_eq!(cpu.reg.d, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_d_from_c() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x51);
    cpu.reg.c = 0x01;
    cpu.reg.d = 0x00;

    cpu.step();
    assert_eq!(cpu.reg.c, 0x01);
    assert_eq!(cpu.reg.d, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_d_from_d() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x52);
    cpu.reg.d = 0x00;
    cpu.reg.d = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.d, 0x01);
    assert_eq!(cpu.reg.d, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}
#[test]
fn load_d_from_e() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x53);
    cpu.reg.d = 0x00;
    cpu.reg.e = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.d, 0x01);
    assert_eq!(cpu.reg.e, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_d_from_h() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x54);
    cpu.reg.d = 0x00;
    cpu.reg.h = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.d, 0x01);
    assert_eq!(cpu.reg.h, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_d_from_l() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x55);
    cpu.reg.d = 0x00;
    cpu.reg.l = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.d, 0x01);
    assert_eq!(cpu.reg.l, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_d_from_hl() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x56);
    cpu.memory.write(0x0124, 0x55);
    cpu.reg.d = 0x00;
    cpu.reg.h = 0x01;
    cpu.reg.l = 0x24;

    cpu.step();
    assert_eq!(cpu.reg.d, 0x55);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x08);
}

#[test]
fn load_e_from_b() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x58);
    cpu.reg.b = 0x01;
    cpu.reg.e = 0x00;
    cpu.step();
    assert_eq!(cpu.reg.b, 0x01);
    assert_eq!(cpu.reg.e, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_e_from_c() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x59);
    cpu.reg.c = 0x01;
    cpu.reg.e = 0x00;

    cpu.step();
    assert_eq!(cpu.reg.c, 0x01);
    assert_eq!(cpu.reg.e, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_e_from_d() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x5A);
    cpu.reg.e = 0x00;
    cpu.reg.d = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.e, 0x01);
    assert_eq!(cpu.reg.d, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}
#[test]
fn load_e_from_e() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x5B);
    cpu.reg.e = 0x00;
    cpu.reg.e = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.e, 0x01);
    assert_eq!(cpu.reg.e, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_e_from_h() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x5C);
    cpu.reg.e = 0x00;
    cpu.reg.h = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.e, 0x01);
    assert_eq!(cpu.reg.h, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_e_from_l() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x5D);
    cpu.reg.e = 0x00;
    cpu.reg.l = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.e, 0x01);
    assert_eq!(cpu.reg.l, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_e_from_hl() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x5E);
    cpu.memory.write(0x0124, 0x55);
    cpu.reg.e = 0x00;
    cpu.reg.h = 0x01;
    cpu.reg.l = 0x24;

    cpu.step();
    assert_eq!(cpu.reg.e, 0x55);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x08);
}

#[test]
fn load_h_from_b() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x60);
    cpu.reg.b = 0x01;
    cpu.reg.h = 0x00;
    cpu.step();
    assert_eq!(cpu.reg.b, 0x01);
    assert_eq!(cpu.reg.h, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_h_from_c() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x61);
    cpu.reg.c = 0x01;
    cpu.reg.h = 0x00;

    cpu.step();
    assert_eq!(cpu.reg.c, 0x01);
    assert_eq!(cpu.reg.h, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_h_from_d() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x62);
    cpu.reg.h = 0x00;
    cpu.reg.d = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.h, 0x01);
    assert_eq!(cpu.reg.d, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}
#[test]
fn load_h_from_e() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x63);
    cpu.reg.h = 0x00;
    cpu.reg.e = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.e, 0x01);
    assert_eq!(cpu.reg.h, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_h_from_h() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x64);
    cpu.reg.h = 0x00;
    cpu.reg.h = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.h, 0x01);
    assert_eq!(cpu.reg.h, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_h_from_l() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x65);
    cpu.reg.h = 0x00;
    cpu.reg.l = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.h, 0x01);
    assert_eq!(cpu.reg.l, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_h_from_hl() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x66);
    cpu.memory.write(0x0124, 0x55);
    cpu.reg.h = 0x00;
    cpu.reg.h = 0x01;
    cpu.reg.l = 0x24;

    cpu.step();
    assert_eq!(cpu.reg.h, 0x55);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x08);
}

#[test]
fn load_l_from_b() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x68);
    cpu.reg.b = 0x01;
    cpu.reg.l = 0x00;
    cpu.step();
    assert_eq!(cpu.reg.b, 0x01);
    assert_eq!(cpu.reg.l, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_l_from_c() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x69);
    cpu.reg.c = 0x01;
    cpu.reg.l = 0x00;

    cpu.step();
    assert_eq!(cpu.reg.c, 0x01);
    assert_eq!(cpu.reg.l, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_l_from_d() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x6A);
    cpu.reg.l = 0x00;
    cpu.reg.d = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.l, 0x01);
    assert_eq!(cpu.reg.d, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}
#[test]
fn load_l_from_e() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x6B);
    cpu.reg.l = 0x00;
    cpu.reg.e = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.e, 0x01);
    assert_eq!(cpu.reg.l, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_l_from_h() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x6C);
    cpu.reg.l = 0x00;
    cpu.reg.h = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.l, 0x01);
    assert_eq!(cpu.reg.h, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_l_from_l() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x6D);
    cpu.reg.l = 0x00;
    cpu.reg.l = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.l, 0x01);
    assert_eq!(cpu.reg.l, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_l_from_hl() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x6E);
    cpu.memory.write(0x0124, 0x55);
    cpu.reg.l = 0x00;
    cpu.reg.h = 0x01;
    cpu.reg.l = 0x24;

    cpu.step();
    assert_eq!(cpu.reg.l, 0x55);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x08);
}

#[test]
fn load_hl_from_b() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x70);
    cpu.reg.b = 0x55;
    cpu.reg.h = 0x01;
    cpu.reg.l = 0x24;

    cpu.step();
    assert_eq!(cpu.memory.read(0x0124), 0x55);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x08);
}

#[test]
fn load_hl_from_c() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x71);
    cpu.reg.c = 0x55;
    cpu.reg.h = 0x01;
    cpu.reg.l = 0x24;

    cpu.step();
    assert_eq!(cpu.memory.read(0x0124), 0x55);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x08);
}

#[test]
fn load_hl_from_d() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x72);
    cpu.reg.d = 0x55;
    cpu.reg.h = 0x01;
    cpu.reg.l = 0x24;

    cpu.step();
    assert_eq!(cpu.memory.read(0x0124), 0x55);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x08);
}

#[test]
fn load_hl_from_e() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x73);
    cpu.reg.e = 0x55;
    cpu.reg.h = 0x01;
    cpu.reg.l = 0x24;

    cpu.step();
    assert_eq!(cpu.memory.read(0x0124), 0x55);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x08);
}

#[test]
fn load_hl_from_h() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x74);
    //cpu.reg.h = 0x55;
    cpu.reg.h = 0x01;
    cpu.reg.l = 0x24;

    cpu.step();
    assert_eq!(cpu.memory.read(0x0124), 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x08);
}

#[test]
fn load_hl_from_l() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x75);
    //cpu.reg.h = 0x55;
    cpu.reg.h = 0x01;
    cpu.reg.l = 0x24;

    cpu.step();
    assert_eq!(cpu.memory.read(0x0124), 0x24);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x08);
}

#[test]
fn load_hl_from_immediate() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x36);
    cpu.memory.write(1, 0x55);
    //cpu.reg.h = 0x55;
    cpu.reg.h = 0x01;
    cpu.reg.l = 0x24;

    cpu.step();
    assert_eq!(cpu.memory.read(0x0124), 0x55);
    assert_eq!(cpu.reg.pc, 0x02);
    assert_eq!(cpu.cycles, 12);
}
