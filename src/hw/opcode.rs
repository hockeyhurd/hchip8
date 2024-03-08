#[derive(Debug)]
pub struct Opcode
{
    pub raw: u16,
    pub a: u16,
    pub b: u16,
    pub c: u16,
    pub d: u16,
}

impl Opcode
{
    pub fn new(raw_opcode: u16) -> Self
    {
        Self
        {
            raw: raw_opcode,
            a: (raw_opcode >> 12) & 0x000F,
            b: (raw_opcode >> 8) & 0x000F,
            c: (raw_opcode >> 4) & 0x000F,
            d: raw_opcode & 0x000F,
        }
    }
}

