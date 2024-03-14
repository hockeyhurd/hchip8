mod hw;

#[allow(unused_imports)]
use hw::cpu::CPU;
use hw::mem::Mem;
use hw::timer::Timer;

fn main()
{
    let capacity: usize = 4096;
    let mut cpu = CPU::new(capacity);

    // 100 Hz.
    let cycle_time_ms = std::time::Duration::from_millis(10);
    let mut timer = Timer::new(cycle_time_ms);

    while !cpu.is_halted()
    {
        cpu.tick();
        timer.sleep_for_remaining();
        timer.reset();
    }

    println!("CPU is halted\nDumping final CPU state:");
    cpu.print_state(false);

    println!("End of emulator");
}

