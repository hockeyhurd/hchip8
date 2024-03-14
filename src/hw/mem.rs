pub struct Mem
{
    arr: Vec<u8>,
    capacity: usize,
}

impl Mem
{
    pub fn new(capacity: usize) -> Self
    {
        // Self { arr: Vec::<u8>::with_capacity(capacity), capacity: capacity }
        Self { arr: vec![0; capacity], capacity: capacity }
    }

    #[allow(dead_code)]
    pub fn get_capacity(&self) -> usize
    {
        self.capacity
    }

    #[allow(dead_code)]
    pub fn size(&self) -> usize
    {
        self.arr.len()
    }

    #[allow(dead_code)]
    pub fn read_u8(&self, addr: usize) -> Option<u8>
    {
        if addr < self.capacity
        {
            return Some(self.arr[addr]);
        }

        return None;
    }

    #[allow(dead_code)]
    pub fn write_u8(&mut self, addr: usize, value: u8) -> bool
    {
        if addr < self.capacity
        {
            self.arr[addr] = value;
            return true;
        }

        return false;
    }

    #[allow(dead_code)]
    pub fn read_u16(&self, mut addr: usize) -> Option<u16>
    {
        if addr < self.capacity - 1
        {
            let mut result: u16 = self.read_u8(addr).unwrap() as u16;
            addr += 1;
            result <<= 8;
            result |= self.read_u8(addr).unwrap() as u16;

            return Some(result);
        }

        return None;
    }

    #[allow(dead_code)]
    pub fn write_u16(&mut self, mut addr: usize, value: u16) -> bool
    {
        if addr < self.capacity
        {
            self.write_u8(addr, ((value >> 8) & 0xFF) as u8);
            addr += 1;
            self.write_u8(addr, (value & 0xFF) as u8);

            return true;
        }

        return false;
    }

    pub fn print_state(&self, stream: &mut String)
    {
        for addr in 0..self.get_capacity()
        {
            if addr % 8 == 0
            {
                if addr > 0
                {
                    stream.push('\n');
                }

                stream.push('\t');
                stream.push('\t');
            }

            let value = self.read_u8(addr).expect("Expected to unwrap during 'print_stack_block'");
            *stream += &value.to_string();
            stream.push(' ');
        }
    }
}

#[cfg(test)]
mod tests
{
    #[allow(unused_imports)]
    use crate::hw::mem::Mem;

    #[test]
    fn capacity_matches_arr()
    {
        let capacity: usize = 4096;
        let mem = Mem::new(capacity);
        assert_eq!(capacity, mem.get_capacity());
    }

    #[test]
    fn arr_size_is_at_capacity()
    {
        let capacity: usize = 4096;
        let mem = Mem::new(capacity);
        assert_eq!(capacity, mem.size());
    }

    #[test]
    fn arr_is_zero()
    {
        let capacity: usize = 4096;
        let mem = Mem::new(capacity);
        let nonzero_value: u8 = 1;

        for i in 0..mem.get_capacity()
        {
            assert_eq!(mem.read_u8(i).unwrap_or(nonzero_value), 0);
        }
    }

    #[test]
    fn read_u8_out_of_bounds_fails()
    {
        let capacity: usize = 4096;
        let mem = Mem::new(capacity);
        let addr = capacity + 1;
        let nonzero_value: u8 = 1;

        assert_ne!(mem.read_u8(addr).unwrap_or(nonzero_value), 0);
    }

    #[test]
    fn write_u8_in_bounds_passes()
    {
        let capacity: usize = 4096;
        let mut mem = Mem::new(capacity);
        let addr = capacity - 1;
        let value: u8 = 42;
        let nonzero_value: u8 = 1;

        assert_eq!(mem.read_u8(addr).unwrap_or(nonzero_value), 0);
        assert_eq!(mem.write_u8(addr, value), true);
        assert_eq!(mem.read_u8(addr).unwrap_or(nonzero_value), value);
    }

    #[test]
    fn read_u16_out_of_bounds_fails()
    {
        let capacity: usize = 4096;
        let mem = Mem::new(capacity);
        let addr = capacity;
        let nonzero_value: u16 = 1;

        assert_ne!(mem.read_u16(addr).unwrap_or(nonzero_value), 0);
    }

    #[test]
    fn write_u16_in_bounds_passes()
    {
        let capacity: usize = 4096;
        let mut mem = Mem::new(capacity);
        let addr = capacity - 2;
        let value: u16 = 0x4142;
        let nonzero_value: u16 = 1;

        assert_eq!(mem.read_u16(addr).unwrap_or(nonzero_value), 0);
        assert_eq!(mem.write_u16(addr, value), true);
        assert_eq!(mem.read_u16(addr).unwrap_or(nonzero_value), value);
    }
}

