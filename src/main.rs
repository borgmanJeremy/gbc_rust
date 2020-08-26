use gba::cpu::*;
use gba::memory::*;

fn main() {
    let mem = MemoryMap::new(0xFFFF);
    let _cpu = Cpu::new(&mem);
}
