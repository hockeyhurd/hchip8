mod env;
mod hw;

#[allow(unused_imports)]
use env::config_data::ConfigData;
use hw::cpu::CPU;
use hw::mem::Mem;
use hw::timer::Timer;

fn main()
{
    let args: Vec<String> = std::env::args().collect();
    let mut config_data = ConfigData::new(args);
    let opt_error_message = config_data.parse();

    match opt_error_message
    {
        Some((code, msg)) =>
        {
            const EXIT_CODE: i32 = -1;
            println!("[ERROR]: '{0}' (exit code: {1}).", msg, code);
            std::process::exit(EXIT_CODE);
        },
        None => {},
    }

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

