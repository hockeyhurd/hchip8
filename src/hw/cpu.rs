use crate::Mem;
use crate::hw::opcode::Opcode;

use std::collections::HashMap;

const STARTING_PC: u16 = 0x200;
const STACK_BLOCK_SIZE: u8 = 64;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum EnumRegister
{
    V0, V1, V2, V3, V4, V5, V6, V7,
    V8, V9, VA, VB, VC, VD, VE, VF,
}

impl EnumRegister
{
    const VALUES: [Self; 16] = [
        Self::V0, Self::V1, Self::V2, Self::V3, Self::V4, Self::V5, Self::V6, Self::V7,
        Self::V8, Self::V9, Self::VA, Self::VB, Self::VC, Self::VD, Self::VE, Self::VF,
    ];
}

pub struct CPU
{
    mem: Mem,
    registers: HashMap<EnumRegister, u8>,
    pc: u16,
    sp: u8,
    stack_block: Mem,
    halted: bool,
}

impl CPU
{
    pub fn new(capacity: usize) -> Self
    {
        // According to wikipedia, chip8 reserves the first 512 bytes of main memory.
        // For now, we are assuming the stack pointer will be the first address after that (512).
        let mut result = Self {
            mem: Mem::new(capacity), registers: HashMap::new(),
            pc : STARTING_PC, sp: 0,
            stack_block: Mem::new(STACK_BLOCK_SIZE as usize),
            halted: false,
        };

        result.init();

        result
    }

    fn init(&mut self)
    {
        self.registers.insert(EnumRegister::V0, 0);
        self.registers.insert(EnumRegister::V1, 0);
        self.registers.insert(EnumRegister::V2, 0);
        self.registers.insert(EnumRegister::V3, 0);
        self.registers.insert(EnumRegister::V4, 0);
        self.registers.insert(EnumRegister::V5, 0);
        self.registers.insert(EnumRegister::V6, 0);
        self.registers.insert(EnumRegister::V7, 0);
        self.registers.insert(EnumRegister::V8, 0);
        self.registers.insert(EnumRegister::V9, 0);
        self.registers.insert(EnumRegister::VA, 0);
        self.registers.insert(EnumRegister::VB, 0);
        self.registers.insert(EnumRegister::VC, 0);
        self.registers.insert(EnumRegister::VD, 0);
        self.registers.insert(EnumRegister::VE, 0);
        self.registers.insert(EnumRegister::VF, 0);
    }

    pub fn is_halted(&self) -> bool
    {
        self.halted
    }

    fn set_halted(&mut self)
    {
        self.halted = true;
    }

    fn has_stack_space(&self) -> bool
    {
        (self.sp as usize) < self.stack_block.size()
    }

    #[allow(dead_code)]
    fn pop_stack(&mut self) -> Option<u16>
    {
        if self.sp < 2
        {
            return None;
        }

        self.sp -= 2;
        let result = self.stack_block.read_u16(self.sp as usize);

        return result;
    }

    #[allow(dead_code)]
    fn push_stack(&mut self, ret_address: u16) -> bool
    {
        if self.sp < STACK_BLOCK_SIZE - 1
        {
            self.stack_block.write_u16(self.sp as usize, ret_address);
            self.sp += 2;

            return true;
        }

        return false;
    }

    pub fn print_state(&self, verbose: bool)
    {
        let mut stream = String::with_capacity(0x100);
        stream += "\tMem block size: ";
        stream += &self.mem.get_capacity().to_string();

        if verbose
        {
            stream += "\n\tStack block: {\n";
            self.mem.print_state(&mut stream);
            stream += "\n\t}";
        }

        stream += "\n\tpc: ";
        stream += &self.pc.to_string();
        stream += "\n\tsp: ";
        stream += &self.sp.to_string();
        stream += "\n\tStack block: {\n";
        self.stack_block.print_state(&mut stream);
        stream += "\n\t}";
        stream += "\n\thalted: ";
        stream += &self.halted.to_string();

        println!("{}", stream);
    }

    pub fn tick(&mut self)
    {
        if self.is_halted()
        {
            return;
        }

        // Fetch:
        let pc_ext = self.pc as usize;

        if pc_ext >= self.mem.size()
        {
            self.set_halted();
            return;
        }

        let raw_opcode: u16 = self.mem.read_u16(pc_ext).expect("Ran out of memory (logic error with pc register and main memory capacity).");
        self.pc += 2;

        // Decode:
        let opcode = Opcode::new(raw_opcode);

        // Execute:
        // TODO: Consider moving this to a HashMap??
        match opcode.a
        {
            // Call instruction
            0 => {
                if opcode.b == 0
                {
                    if opcode.c == 0 && opcode.d == 0
                    {
                        // NOTE: if all 0s (NULL) we will use this as a pseudo halt instruction.
                        self.set_halted();
                    }

                    // TODO: 'disp_clear()'
                    else if opcode.c == 0x0E && opcode.d == 0x00
                    {
                        println!("disp_clear() is not yet supported... this instruction will be no-op'd");
                        return;
                    }

                    else if opcode.c == 0x0E && opcode.d == 0x0E
                    {
                        if self.sp < 2
                        {
                            println!("[ERROR]: Return instruction was made but we do not have an address to return to... The system will be halted.");
                            self.set_halted();
                            return;
                        }

                        self.sp -= 2;
                        self.pc = self.stack_block.read_u16(self.sp as usize).expect("Failed to unwrap Option during return instruction.");
                    }

                    else
                    {
                        println!("[1] Could not find instruction for opcode {:?}", opcode);
                    }
                }

                // Call instruction
                else
                {
                    if !self.has_stack_space()
                    {
                        println!("[ERROR]: Call instruction was made but we are out of stack space... The system will be halted.");
                        self.set_halted();
                        return;
                    }

                    self.stack_block.write_u16(self.sp as usize, self.pc);
                    self.sp += 2;
                    self.pc = opcode.raw & 0x0FFF;
                }
            },
            // Jump to 12-bit address.
            1 => {
                let to_addr: u16 = opcode.raw & 0x0FFF;
                self.pc = to_addr;
            },
            // Set register 'b' to value 'c|d'
            6 => {
                let reg = EnumRegister::VALUES[opcode.b as usize];
                let value: u8 = (opcode.raw & 0x00FF) as u8;
                self.write_register(reg, value);
            },
            8 => {
                let regb = EnumRegister::VALUES[opcode.b as usize];
                let regc = EnumRegister::VALUES[opcode.c as usize];
                let value: u8 = self.read_register(regc);

                match opcode.d
                {
                    0 => {
                        self.write_register(regb, value);
                    },
                    1 => {
                        let cur_value = self.read_register(regb);
                        self.write_register(regb, cur_value | value)
                    },
                    2 => {
                        let cur_value = self.read_register(regb);
                        self.write_register(regb, cur_value & value)
                    },
                    3 => {
                        let cur_value = self.read_register(regb);
                        self.write_register(regb, cur_value ^ value)
                    },
                    4 => {
                        let cur_value = self.read_register(regb);
                        self.write_register(regb, cur_value + value)
                    },
                    5 => {
                        let cur_value = self.read_register(regb);
                        self.write_register(regb, cur_value - value)
                    },
                    6 => {
                        let cur_value = self.read_register(regb);
                        self.write_register(EnumRegister::VF, cur_value & 0x01);
                        self.write_register(regb, cur_value >> 1)
                    },
                    7 => {
                        let cur_value = self.read_register(regb);
                        self.write_register(regb, value - cur_value)
                    },
                    0x0E => {
                        let cur_value = self.read_register(regb);
                        self.write_register(EnumRegister::VF, (cur_value & 0x80) >> 7);
                        self.write_register(regb, cur_value << 1);
                    },
                    _ => { println!("[2] Could not find instruction for opcode {:?}", opcode); }
                }
            },
            _ => { println!("[3] Could not find instruction for opcode {:?}", opcode); }
        }
    }

    fn read_register(&self, reg: EnumRegister) -> u8
    {
        let value = self.registers.get(&reg).expect("Failed to unwrap during read_register routine");
        *value
    }

    fn write_register(&mut self, reg: EnumRegister, value: u8)
    {
        let value_at = self.registers.get_mut(&reg).expect("Failed to unwrap during write_register routine");
        *value_at = value;
    }
}

#[cfg(test)]
mod tests
{
    #[allow(unused_imports)]
    use crate::hw::cpu::CPU;
    use crate::hw::cpu::STARTING_PC;
    use crate::hw::cpu::STACK_BLOCK_SIZE;

    use super::EnumRegister;

    #[test]
    fn cpu_and_registers_are_init_correctly()
    {
        let capacity: usize = 4096;
        let cpu = CPU::new(capacity);
        assert_eq!(cpu.mem.size(), capacity);

        let value: u8 = 0;

        for reg in EnumRegister::VALUES
        {
            assert_eq!(cpu.read_register(reg), value);
        }

        assert_eq!(cpu.pc, STARTING_PC);
        assert_eq!(cpu.sp, 0);
        assert_eq!(cpu.stack_block.size(), STACK_BLOCK_SIZE as usize);
        assert!(!cpu.is_halted());
    }

    #[test]
    fn fill_stack_then_empty()
    {
        let capacity: usize = 4096;
        let mut cpu = CPU::new(capacity);
        assert_eq!(cpu.stack_block.size(), STACK_BLOCK_SIZE as usize);

        let half_size = (cpu.stack_block.size() / 2) as u16;

        for i in 0..half_size
        {
            let ret_address = i + 2;
            assert!(cpu.has_stack_space());
            assert!(cpu.push_stack(ret_address));
        }

        // Try to push one more
        assert!(!cpu.has_stack_space());
        assert!(!cpu.push_stack(0xFFFF));

        // Pop until empty
        for i in (0..half_size).rev()
        {
            let expected_ret_address = i + 2;
            let opt_ret_address = cpu.pop_stack();
            assert!(opt_ret_address.is_some());
            assert_eq!(opt_ret_address.unwrap(), expected_ret_address);
        }

        // Try to pop one more
        assert!(cpu.has_stack_space());
        assert!(cpu.pop_stack().is_none());
    }

    #[test]
    fn fill_registers_and_readback()
    {
        let capacity: usize = 4096;
        let mut cpu = CPU::new(capacity);
        let mut i = 0;

        for reg in EnumRegister::VALUES
        {
            let value = i * i;
            cpu.write_register(reg, value);
            assert_eq!(cpu.read_register(reg), value);
            i += 1;
        }
    }

    #[test]
    fn execute_halt_instruction()
    {
        let capacity: usize = 4096;
        let mut cpu = CPU::new(capacity);

        let mem_addr = cpu.pc as usize;

        // Load memory with simple halt instruction, which happens to already be all zeros...
        cpu.mem.write_u16(mem_addr, 0);

        // Try executing our 'fake' program
        assert!(!cpu.is_halted());
        cpu.tick();
        assert!(cpu.is_halted());
    }

    #[test]
    fn execute_set_instruction()
    {
        let capacity: usize = 4096;
        let mut cpu = CPU::new(capacity);

        let mut mem_addr = cpu.pc as usize;

        cpu.mem.write_u16(mem_addr, 0x61AF);
        mem_addr += 2;

        // Then halt
        cpu.mem.write_u16(mem_addr, 0);

        // Try executing our 'fake' program
        assert!(!cpu.is_halted());

        // Make sure register is not our value first
        assert_ne!(cpu.read_register(EnumRegister::V1), 0xAF);

        // This should set V1 = 0xAF
        cpu.tick();

        // Verify V1 register is correct
        assert_eq!(cpu.read_register(EnumRegister::V1), 0xAF);

        // Execute halt instruction
        assert!(!cpu.is_halted());
        cpu.tick();
        assert!(cpu.is_halted());
    }

    #[test]
    fn execute_assign_instruction()
    {
        let capacity: usize = 4096;
        let mut cpu = CPU::new(capacity);
        let mut mem_addr = cpu.pc as usize;

        cpu.mem.write_u16(mem_addr, 0x61A0);
        mem_addr += 2;
        cpu.mem.write_u16(mem_addr, 0x620F);
        mem_addr += 2;
        cpu.mem.write_u16(mem_addr, 0x8120);
        mem_addr += 2;

        // Then halt
        cpu.mem.write_u16(mem_addr, 0);

        // Try executing our 'fake' program
        assert!(!cpu.is_halted());

        // Make sure register is not our value first
        assert_ne!(cpu.read_register(EnumRegister::V1), 0xA0);
        assert_ne!(cpu.read_register(EnumRegister::V2), 0x0F);

        // This should set V1 = 0xA0
        cpu.tick();

        // This should set V2 = 0x0F
        assert!(!cpu.is_halted());
        cpu.tick();

        // This should set V1 = V2
        assert!(!cpu.is_halted());
        cpu.tick();

        // Verify V1 register is correct
        assert_eq!(cpu.read_register(EnumRegister::V1), 0x0F);

        // Execute halt instruction
        assert!(!cpu.is_halted());
        cpu.tick();
        assert!(cpu.is_halted());
    }

    #[test]
    fn execute_or_equal_instruction()
    {
        let capacity: usize = 4096;
        let mut cpu = CPU::new(capacity);
        let mut mem_addr = cpu.pc as usize;

        cpu.mem.write_u16(mem_addr, 0x61A0);
        mem_addr += 2;
        cpu.mem.write_u16(mem_addr, 0x620F);
        mem_addr += 2;
        cpu.mem.write_u16(mem_addr, 0x8121);
        mem_addr += 2;

        // Then halt
        cpu.mem.write_u16(mem_addr, 0);

        // Try executing our 'fake' program
        assert!(!cpu.is_halted());

        // Make sure register is not our value first
        assert_ne!(cpu.read_register(EnumRegister::V1), 0xA0);
        assert_ne!(cpu.read_register(EnumRegister::V2), 0x0F);

        // This should set V1 = 0xA0
        cpu.tick();

        // This should set V2 = 0x0F
        assert!(!cpu.is_halted());
        cpu.tick();

        // This should set V1 = V2
        assert!(!cpu.is_halted());
        cpu.tick();

        // Verify V1 register is correct
        assert_eq!(cpu.read_register(EnumRegister::V1), 0xAF);

        // Execute halt instruction
        assert!(!cpu.is_halted());
        cpu.tick();
        assert!(cpu.is_halted());
    }

    #[test]
    fn execute_and_equal_instruction()
    {
        let capacity: usize = 4096;
        let mut cpu = CPU::new(capacity);
        let mut mem_addr = cpu.pc as usize;

        cpu.mem.write_u16(mem_addr, 0x61A3);
        mem_addr += 2;
        cpu.mem.write_u16(mem_addr, 0x622F);
        mem_addr += 2;
        cpu.mem.write_u16(mem_addr, 0x8122);
        mem_addr += 2;

        // Then halt
        cpu.mem.write_u16(mem_addr, 0);

        // Try executing our 'fake' program
        assert!(!cpu.is_halted());

        // Make sure register is not our value first
        assert_ne!(cpu.read_register(EnumRegister::V1), 0xA3);
        assert_ne!(cpu.read_register(EnumRegister::V2), 0x2F);

        // This should set V1 = 0xA0
        cpu.tick();

        // This should set V2 = 0x0F
        assert!(!cpu.is_halted());
        cpu.tick();

        // This should set V1 = V2
        assert!(!cpu.is_halted());
        cpu.tick();

        // Verify V1 register is correct
        assert_eq!(cpu.read_register(EnumRegister::V1), 0xA3 & 0x2F);

        // Execute halt instruction
        assert!(!cpu.is_halted());
        cpu.tick();
        assert!(cpu.is_halted());
    }

    #[test]
    fn execute_xor_equal_instruction()
    {
        let capacity: usize = 4096;
        let mut cpu = CPU::new(capacity);
        let mut mem_addr = cpu.pc as usize;

        cpu.mem.write_u16(mem_addr, 0x61A3);
        mem_addr += 2;
        cpu.mem.write_u16(mem_addr, 0x622F);
        mem_addr += 2;
        cpu.mem.write_u16(mem_addr, 0x8123);
        mem_addr += 2;

        // Then halt
        cpu.mem.write_u16(mem_addr, 0);

        // Try executing our 'fake' program
        assert!(!cpu.is_halted());

        // Make sure register is not our value first
        assert_ne!(cpu.read_register(EnumRegister::V1), 0xA3);
        assert_ne!(cpu.read_register(EnumRegister::V2), 0x2F);

        // This should set V1 = 0xA0
        cpu.tick();

        // This should set V2 = 0x0F
        assert!(!cpu.is_halted());
        cpu.tick();

        // This should set V1 = V2
        assert!(!cpu.is_halted());
        cpu.tick();

        // Verify V1 register is correct
        assert_eq!(cpu.read_register(EnumRegister::V1), 0xA3 ^ 0x2F);

        // Execute halt instruction
        assert!(!cpu.is_halted());
        cpu.tick();
        assert!(cpu.is_halted());
    }

    #[test]
    fn execute_plus_equal_instruction()
    {
        let capacity: usize = 4096;
        let mut cpu = CPU::new(capacity);
        let mut mem_addr = cpu.pc as usize;

        cpu.mem.write_u16(mem_addr, 0x61A3);
        mem_addr += 2;
        cpu.mem.write_u16(mem_addr, 0x622F);
        mem_addr += 2;
        cpu.mem.write_u16(mem_addr, 0x8125);
        mem_addr += 2;

        // Then halt
        cpu.mem.write_u16(mem_addr, 0);

        // Try executing our 'fake' program
        assert!(!cpu.is_halted());

        // Make sure register is not our value first
        assert_ne!(cpu.read_register(EnumRegister::V1), 0xA3);
        assert_ne!(cpu.read_register(EnumRegister::V2), 0x2F);

        // This should set V1 = 0xA0
        cpu.tick();

        // This should set V2 = 0x0F
        assert!(!cpu.is_halted());
        cpu.tick();

        // This should set V1 = V2
        assert!(!cpu.is_halted());
        cpu.tick();

        // Verify V1 register is correct
        assert_eq!(cpu.read_register(EnumRegister::V1), 0xA3 - 0x2F);

        // Execute halt instruction
        assert!(!cpu.is_halted());
        cpu.tick();
        assert!(cpu.is_halted());
    }

    #[test]
    fn execute_rshift_equal_instruction()
    {
        let capacity: usize = 4096;
        let mut cpu = CPU::new(capacity);
        let mut mem_addr = cpu.pc as usize;

        cpu.mem.write_u16(mem_addr, 0x6103);
        mem_addr += 2;
        cpu.mem.write_u16(mem_addr, 0x8106);
        mem_addr += 2;

        // Then halt
        cpu.mem.write_u16(mem_addr, 0);

        // Try executing our 'fake' program
        assert!(!cpu.is_halted());

        // Make sure register is not our value first
        assert_ne!(cpu.read_register(EnumRegister::V1), 0x03);
        assert_ne!(cpu.read_register(EnumRegister::VF), 0x01);

        // This should set V1 = 0xA0
        cpu.tick();

        // This should set V2 = 0x0F
        assert!(!cpu.is_halted());
        cpu.tick();

        // Verify V1 register is correct
        assert_eq!(cpu.read_register(EnumRegister::V1), 0x01);
        assert_eq!(cpu.read_register(EnumRegister::VF), 0x01);

        // Execute halt instruction
        assert!(!cpu.is_halted());
        cpu.tick();
        assert!(cpu.is_halted());
    }

    #[test]
    fn execute_lshift_equal_instruction()
    {
        let capacity: usize = 4096;
        let mut cpu = CPU::new(capacity);
        let mut mem_addr = cpu.pc as usize;

        cpu.mem.write_u16(mem_addr, 0x6183);
        mem_addr += 2;
        cpu.mem.write_u16(mem_addr, 0x810E);
        mem_addr += 2;

        // Then halt
        cpu.mem.write_u16(mem_addr, 0);

        // Try executing our 'fake' program
        assert!(!cpu.is_halted());

        // Make sure register is not our value first
        assert_ne!(cpu.read_register(EnumRegister::V1), 0x03);
        assert_ne!(cpu.read_register(EnumRegister::VF), 0x01);

        // This should set V1 = 0xA0
        cpu.tick();

        // This should set V1 = V2
        assert!(!cpu.is_halted());
        cpu.tick();

        // Verify V1 register is correct
        assert_eq!(cpu.read_register(EnumRegister::V1), 0x06);
        assert_eq!(cpu.read_register(EnumRegister::VF), 0x01);

        // Execute halt instruction
        assert!(!cpu.is_halted());
        cpu.tick();
        assert!(cpu.is_halted());
    }

    #[test]
    fn execute_inv_minus_equal_instruction()
    {
        let capacity: usize = 4096;
        let mut cpu = CPU::new(capacity);
        let mut mem_addr = cpu.pc as usize;

        cpu.mem.write_u16(mem_addr, 0x6101);
        mem_addr += 2;
        cpu.mem.write_u16(mem_addr, 0x6203);
        mem_addr += 2;
        cpu.mem.write_u16(mem_addr, 0x8127);
        mem_addr += 2;

        // Then halt
        cpu.mem.write_u16(mem_addr, 0);

        // Try executing our 'fake' program
        assert!(!cpu.is_halted());

        // Make sure register is not our value first
        assert_ne!(cpu.read_register(EnumRegister::V1), 0x01);
        assert_ne!(cpu.read_register(EnumRegister::V2), 0x03);

        // This should set V1 = 0xA0
        cpu.tick();

        // This should set V2 = 0x0F
        assert!(!cpu.is_halted());
        cpu.tick();

        // This should set V1 = V2
        assert!(!cpu.is_halted());
        cpu.tick();

        // Verify V1 register is correct
        assert_eq!(cpu.read_register(EnumRegister::V1), 0x02);

        // Execute halt instruction
        assert!(!cpu.is_halted());
        cpu.tick();
        assert!(cpu.is_halted());
    }

    #[test]
    fn execute_jump_instruction()
    {
        let capacity: usize = 4096;
        let mut cpu = CPU::new(capacity);

        let mut mem_addr = cpu.pc as usize;
        let jump_addr = (capacity as u16) - 4;

        cpu.mem.write_u16(mem_addr, 0x1000 | jump_addr);
        mem_addr = jump_addr as usize;

        // Then halt
        cpu.mem.write_u16(mem_addr, 0);

        // Try executing our 'fake' program
        assert!(!cpu.is_halted());
        assert_ne!(cpu.pc, jump_addr);

        // This should jump to 
        cpu.tick();

        // Verify we jumped to the correct address.
        assert_eq!(cpu.pc, jump_addr);

        // Execute halt instruction
        assert!(!cpu.is_halted());
        cpu.tick();
        assert!(cpu.is_halted());
    }

    #[test]
    fn execute_display_clear_instruction_does_no_op()
    {
        let capacity: usize = 4096;
        let mut cpu = CPU::new(capacity);

        let mut mem_addr = cpu.pc as usize;

        // display clear instruction
        cpu.mem.write_u16(mem_addr, 0x00E0);
        mem_addr += 2;

        // Then halt
        cpu.mem.write_u16(mem_addr, 0);

        // Try executing our 'fake' program
        assert!(!cpu.is_halted());

        // This should jump to 
        cpu.tick();

        // Execute halt instruction
        assert!(!cpu.is_halted());
        cpu.tick();
        assert!(cpu.is_halted());
    }

    #[test]
    fn execute_call_set_return_instructions()
    {
        let capacity: usize = 4096;
        let mut cpu = CPU::new(capacity);

        let mut mem_addr = cpu.pc as usize;
        let call_addr = (capacity as u16) - 8;

        // Call instruction
        cpu.mem.write_u16(mem_addr, call_addr);
        let ret_addr = mem_addr + 2;
        mem_addr = call_addr as usize;

        // Then do set
        cpu.mem.write_u16(mem_addr, 0x61AF);
        mem_addr += 2;

        // Then do return
        cpu.mem.write_u16(mem_addr, 0x00EE);
        mem_addr = ret_addr;

        // Then halt
        cpu.mem.write_u16(mem_addr, 0);

        // Try executing our 'fake' program
        assert!(!cpu.is_halted());
        assert_ne!(cpu.pc, call_addr);

        // This should be the call instruction
        cpu.tick();

        // Verify we called/jumped to the correct address.
        assert_eq!(cpu.pc, call_addr);

        // Execute the set instruction
        assert_ne!(cpu.read_register(EnumRegister::V1), 0xAF);
        assert!(!cpu.is_halted());
        cpu.tick();
        assert_eq!(cpu.read_register(EnumRegister::V1), 0xAF);

        // Execute the return instruction
        assert!(!cpu.is_halted());
        assert_ne!(cpu.pc as usize, ret_addr);
        cpu.tick();
        assert_eq!(cpu.pc as usize, ret_addr);

        // Execute halt instruction
        assert!(!cpu.is_halted());
        cpu.tick();
        assert!(cpu.is_halted());
    }
}

