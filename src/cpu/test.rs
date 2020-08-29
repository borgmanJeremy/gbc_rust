use super::*;

#[test]
fn test_clear_zero() {
    let mut flags = Flag::new();

    flags.z = true;

    flags.clear_zero_flag();
    assert_eq!(flags.z, false)
}

#[test]
fn test_set_zero() {
    let mut flags = Flag::new();

    flags.z = false;

    flags.set_zero_flag();
    assert_eq!(flags.z, true)
}

#[test]
fn test_adjust_zero() {
    let mut flags = Flag::new();

    flags.z = false;
    flags.adjust_zero_flag(0);
    assert_eq!(flags.z, true);

    flags.z = true;
    flags.adjust_zero_flag(1);
    assert_eq!(flags.z, false);
}

#[test]
fn test_clear_subtract() {
    let mut flags = Flag::new();

    flags.n = true;

    flags.clear_subtract_flag();
    assert_eq!(flags.n, false)
}

#[test]
fn test_set_subtract() {
    let mut flags = Flag::new();

    flags.n = false;

    flags.set_subtract_flag();
    assert_eq!(flags.n, true)
}

#[test]
fn test_adjust_carry() {
    let mut flags = Flag::new();

    flags.c = false;
    flags.adjust_carry_flag(0xFF + 1);
    assert_eq!(flags.c, true);

    flags.c = true;
    flags.adjust_carry_flag(0xFF);
    assert_eq!(flags.c, false);
}

#[test]
fn test_adjust_half_carry() {
    let mut flags = Flag::new();

    flags.h = false;
    flags.adjust_half_carry_flag(0xF, 0x0F + 1);
    assert_eq!(flags.h, true);

    flags.h = true;
    flags.adjust_half_carry_flag(0xE, 0x0E + 1);
    assert_eq!(flags.h, false);
}

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
fn test_hl_decrement() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.reg.h = 0x00;
    cpu.reg.l = 0x01;
    cpu.reg.decrement_hl();
    assert_eq!(cpu.reg.h, 0x00);
    assert_eq!(cpu.reg.l, 0x00);

    cpu.reg.h = 0x00;
    cpu.reg.l = 0x00;
    cpu.reg.decrement_hl();
    assert_eq!(cpu.reg.h, 0xFF);
    assert_eq!(cpu.reg.l, 0xFF);

    cpu.reg.h = 0x01;
    cpu.reg.l = 0x00;
    cpu.reg.decrement_hl();
    assert_eq!(cpu.reg.h, 0x00);
    assert_eq!(cpu.reg.l, 0xFF);
}

#[test]
fn test_hl_increment() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.reg.h = 0xFF;
    cpu.reg.l = 0xFF;
    cpu.reg.increment_hl();
    assert_eq!(cpu.reg.h, 0x00);
    assert_eq!(cpu.reg.l, 0x00);

    cpu.reg.h = 0x00;
    cpu.reg.l = 0x00;
    cpu.reg.increment_hl();
    assert_eq!(cpu.reg.h, 0x00);
    assert_eq!(cpu.reg.l, 0x01);

    cpu.reg.h = 0x00;
    cpu.reg.l = 0xFF;
    cpu.reg.increment_hl();
    assert_eq!(cpu.reg.h, 0x01);
    assert_eq!(cpu.reg.l, 0x00);
}
#[test]
fn test_bc_dereference() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);
    cpu.reg.b = 0x80;
    cpu.reg.c = 0x12;

    let address = cpu.reg.bc_address();
    assert_eq!(address, 0x8012);
}

#[test]
fn test_de_dereference() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);
    cpu.reg.d = 0x80;
    cpu.reg.e = 0x12;

    let address = cpu.reg.de_address();
    assert_eq!(address, 0x8012);
}

#[test]
fn test_twobyte_dereference() {
    let mem = MemoryMap::new(0xFFFF);
    let cpu = Cpu::new(&mem);
    cpu.memory.write(0, 0x12);
    cpu.memory.write(1, 0x80);

    let address = cpu.two_byte_address(0x00);
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

#[test]
fn load_a_from_bc() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x0A);
    cpu.memory.write(0x0124, 0x55);
    cpu.reg.a = 0x00;
    cpu.reg.b = 0x01;
    cpu.reg.c = 0x24;

    cpu.step();
    assert_eq!(cpu.reg.a, 0x55);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x08)
}

#[test]
fn load_a_from_two_bytes() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0xFA);
    cpu.memory.write(1, 0x24);
    cpu.memory.write(2, 0x01);
    cpu.memory.write(0x0124, 0x55);
    cpu.reg.a = 0x00;

    cpu.step();
    assert_eq!(cpu.reg.a, 0x55);
    assert_eq!(cpu.reg.pc, 0x03);
    assert_eq!(cpu.cycles, 16)
}

#[test]
fn load_a_from_de() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x1A);
    cpu.memory.write(0x0124, 0x55);
    cpu.reg.a = 0x00;
    cpu.reg.d = 0x01;
    cpu.reg.e = 0x24;

    cpu.step();
    assert_eq!(cpu.reg.a, 0x55);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x08)
}

#[test]
fn load_a_from_byte() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x3E);
    cpu.memory.write(1, 0x55);
    cpu.reg.a = 0x00;

    cpu.step();
    assert_eq!(cpu.reg.a, 0x55);
    assert_eq!(cpu.reg.pc, 0x02);
    assert_eq!(cpu.cycles, 0x08)
}

#[test]
fn load_b_from_a() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x47);
    cpu.reg.b = 0x00;
    cpu.reg.a = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.b, 0x01);
    assert_eq!(cpu.reg.a, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_c_from_a() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x4F);
    cpu.reg.c = 0x00;
    cpu.reg.a = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.c, 0x01);
    assert_eq!(cpu.reg.a, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_d_from_a() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x57);
    cpu.reg.d = 0x00;
    cpu.reg.a = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.d, 0x01);
    assert_eq!(cpu.reg.a, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_e_from_a() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x5F);
    cpu.reg.e = 0x00;
    cpu.reg.a = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.e, 0x01);
    assert_eq!(cpu.reg.a, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_h_from_a() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x67);
    cpu.reg.h = 0x00;
    cpu.reg.a = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.h, 0x01);
    assert_eq!(cpu.reg.a, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_l_from_a() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x6F);
    cpu.reg.l = 0x00;
    cpu.reg.a = 0x01;

    cpu.step();
    assert_eq!(cpu.reg.l, 0x01);
    assert_eq!(cpu.reg.a, 0x01);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x04);
}

#[test]
fn load_bc_from_a() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x02);
    cpu.memory.write(0x0124, 0x00);
    cpu.reg.b = 0x01;
    cpu.reg.c = 0x24;

    cpu.reg.a = 0x55;

    cpu.step();
    assert_eq!(cpu.memory.read(0x0124), 0x55);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x08)
}

#[test]
fn load_de_from_a() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x12);
    cpu.memory.write(0x0124, 0x00);
    cpu.reg.d = 0x01;
    cpu.reg.e = 0x24;

    cpu.reg.a = 0x55;

    cpu.step();
    assert_eq!(cpu.memory.read(0x0124), 0x55);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x08)
}

#[test]
fn load_hl_from_a() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x77);
    cpu.memory.write(0x0124, 0x00);
    cpu.reg.h = 0x01;
    cpu.reg.l = 0x24;

    cpu.reg.a = 0x55;

    cpu.step();
    assert_eq!(cpu.memory.read(0x0124), 0x55);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 0x08)
}

#[test]
fn load_two_bytes_from_a() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0xEA);
    cpu.memory.write(1, 0x24);
    cpu.memory.write(2, 0x01);

    cpu.reg.a = 0x55;

    cpu.step();
    assert_eq!(cpu.memory.read(0x0124), 0x55);
    assert_eq!(cpu.reg.pc, 0x03);
    assert_eq!(cpu.cycles, 16)
}

#[test]
fn load_a_from_00ff_plus_c() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0xF2);
    cpu.memory.write(0xFF10, 0x55);

    cpu.reg.c = 0x10;

    cpu.reg.a = 0x00;

    cpu.step();
    assert_eq!(cpu.reg.a, 0x55);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 8)
}

#[test]
fn load_00ff_plus_c_from_a() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0xE2);
    cpu.reg.a = 0x55;
    cpu.reg.c = 0x10;

    cpu.step();
    assert_eq!(cpu.memory.read(0xFF10), 0x55);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 8)
}

#[test]
fn load_a_from_hl_dec() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x3A);
    cpu.memory.write(0x0124, 0x55);
    cpu.reg.a = 0x00;
    cpu.reg.h = 0x01;
    cpu.reg.l = 0x24;

    cpu.step();
    assert_eq!(cpu.reg.a, 0x55);
    assert_eq!(cpu.reg.h, 0x01);
    assert_eq!(cpu.reg.l, 0x23);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 8)
}

#[test]
fn load_hl_from_a_dec() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x32);
    cpu.reg.a = 0x55;
    cpu.reg.h = 0x01;
    cpu.reg.l = 0x24;

    cpu.step();
    assert_eq!(cpu.memory.read(0x0124), 0x55);
    assert_eq!(cpu.reg.h, 0x01);
    assert_eq!(cpu.reg.l, 0x23);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 8)
}

#[test]
fn load_a_from_hl_inc() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x2A);
    cpu.memory.write(0x0124, 0x55);
    cpu.reg.a = 0x00;
    cpu.reg.h = 0x01;
    cpu.reg.l = 0x24;

    cpu.step();
    assert_eq!(cpu.reg.a, 0x55);
    assert_eq!(cpu.reg.h, 0x01);
    assert_eq!(cpu.reg.l, 0x25);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 8)
}

#[test]
fn load_hl_from_a_inc() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x22);
    cpu.reg.a = 0x55;
    cpu.reg.h = 0x01;
    cpu.reg.l = 0x24;

    cpu.step();
    assert_eq!(cpu.memory.read(0x0124), 0x55);
    assert_eq!(cpu.reg.h, 0x01);
    assert_eq!(cpu.reg.l, 0x25);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 8)
}

#[test]
fn load_00ff_plus_n_from_a() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0xE0);
    cpu.memory.write(1, 0x10);
    cpu.reg.a = 0x55;

    cpu.step();
    assert_eq!(cpu.memory.read(0xFF10), 0x55);
    assert_eq!(cpu.reg.pc, 0x02);
    assert_eq!(cpu.cycles, 12)
}

#[test]
fn load_a_from_00ff_plus_n() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0xF0);
    cpu.memory.write(1, 0x10);
    cpu.memory.write(0xFF10, 0x55);

    cpu.reg.a = 0x00;

    cpu.step();
    assert_eq!(cpu.reg.a, 0x55);
    assert_eq!(cpu.reg.pc, 0x02);
    assert_eq!(cpu.cycles, 12)
}

#[test]
fn load_bc_from_nn() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x01);
    cpu.memory.write(1, 0x55);
    cpu.memory.write(2, 0xAA);

    cpu.reg.b = 0x00;
    cpu.reg.c = 0x00;

    cpu.step();
    assert_eq!(cpu.reg.b, 0xAA);
    assert_eq!(cpu.reg.c, 0x55);
    assert_eq!(cpu.reg.pc, 0x03);
    assert_eq!(cpu.cycles, 12)
}

#[test]
fn load_de_from_nn() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x11);
    cpu.memory.write(1, 0x55);
    cpu.memory.write(2, 0xAA);

    cpu.reg.d = 0x00;
    cpu.reg.e = 0x00;

    cpu.step();
    assert_eq!(cpu.reg.d, 0xAA);
    assert_eq!(cpu.reg.e, 0x55);
    assert_eq!(cpu.reg.pc, 0x03);
    assert_eq!(cpu.cycles, 12)
}

#[test]
fn load_hl_from_nn() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x21);
    cpu.memory.write(1, 0x55);
    cpu.memory.write(2, 0xAA);

    cpu.reg.h = 0x00;
    cpu.reg.l = 0x00;

    cpu.step();
    assert_eq!(cpu.reg.h, 0xAA);
    assert_eq!(cpu.reg.l, 0x55);
    assert_eq!(cpu.reg.pc, 0x03);
    assert_eq!(cpu.cycles, 12)
}

#[test]
fn load_sp_from_nn() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0x31);
    cpu.memory.write(1, 0x55);
    cpu.memory.write(2, 0xAA);

    cpu.reg.sp = 0x00;

    cpu.step();
    assert_eq!(cpu.reg.sp, 0xAA55);
    assert_eq!(cpu.reg.pc, 0x03);
    assert_eq!(cpu.cycles, 12)
}

#[test]
fn load_sp_from_hl() {
    let mem = MemoryMap::new(0xFFFF);
    let mut cpu = Cpu::new(&mem);

    cpu.memory.write(0, 0xF9);

    cpu.reg.h = 0x01;
    cpu.reg.l = 0x24;
    cpu.reg.sp = 0x00;

    cpu.step();
    assert_eq!(cpu.reg.sp, 0x0124);
    assert_eq!(cpu.reg.pc, 0x01);
    assert_eq!(cpu.cycles, 8)
}
