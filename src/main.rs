use gba::cpu::*;
use gba::memory::*;

fn main() {
    let mem = MemoryMap::new(0xFFFF);
    let cpu = Cpu::new(&mem);
}
