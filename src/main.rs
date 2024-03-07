mod hw;

#[allow(unused_imports)]
use hw::cpu::CPU;
use hw::mem::Mem;

fn main()
{
    let capacity: usize = 4096;
    let cpu = CPU::new(capacity);
}

