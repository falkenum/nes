
#![allow(dead_code)]
mod instructions;

const RAM_FIRST : usize = 0x0000;
const RAM_SIZE : usize = 0x0800;
const RAM_LAST : usize = 0x1FFF;

struct CPUMem {
    // 0000 - 07FF : ram
    // 0800 - 1FFF : mirrors of ram
    // 2000 - 2007 : PPU regs
    // 2008 - 3FFF : Mirrors of PPU regs
    // 4000 - 4017 : APU and IO regs
    // 4018 - 401F : test mode stuff
    // 4020 - FFFF : cartridge
    ram : [u8; RAM_SIZE],
}

struct CPUFlags {
    n : bool,
    v : bool,
    b : bool,
    d : bool,
    i : bool,
    z : bool,
    c : bool,
}

pub struct CPU {
    a : u8,
    x : u8,
    y : u8,
    sp : u8,
    pc : u16,
    flags : CPUFlags,
    mem : CPUMem,
}

use std::ops::{ Index, IndexMut };
impl Index<usize> for CPUMem {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            RAM_FIRST...RAM_LAST => &self.ram[index % RAM_SIZE],
            // TODO other types of memory
            _ => panic!("couldn't map the index to CPU memory"),
        }
    }
}

impl IndexMut<usize> for CPUMem {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            RAM_FIRST...RAM_LAST => &mut self.ram[index % RAM_SIZE],
            // TODO other types of memory
            _ => panic!("couldn't map the index to CPU memory"),
        }
    }
}

impl CPU {
    fn immediate   (&self, val : u8)  -> u8  { val }
    fn absolute    (&self, val : u16) -> u16 { val }
    fn absolute_x  (&self, val : u16) -> u16 { val + self.x as u16 }
    fn absolute_y  (&self, val : u16) -> u16 { val + self.y as u16 }
    fn zero_page   (&self, val : u8)  -> u16 { val as u16 }
    fn zero_page_x (&self, val : u8)  -> u16 { val as u16 + self.x as u16 }
    fn zero_page_y (&self, val : u8)  -> u16 { val as u16 + self.y as u16 }
    fn indirect_x  (&self, val : u16) -> u16 { self.indirect(val + self.x as u16) }
    fn indirect_y  (&self, val : u16) -> u16 { self.indirect(val) + self.y as u16 }
    fn indirect    (&self, val : u16) -> u16 {
        let i = val as usize;
        self.mem[i] as u16 + (self.mem[i + 1] as u16) << 8
    }

    pub fn step (&mut self) {
    }

    fn lda(&mut self, val : u8) {
        // checking if val is zero or negative
        self.flags.z = val        == 0;
        self.flags.n = val & 0x80 != 0;

        self.a = val;
    }

    pub fn new() -> CPU {
        CPU {
            a : 0,
            x : 0,
            y : 0,
            sp : 0,
            pc : 0,
            flags : CPUFlags {
                n : false,
                v : false,
                b : false,
                d : false,
                i : false,
                z : false,
                c : false,
            },
            mem : CPUMem {
                ram : [0; RAM_SIZE],
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mem() {
        let mut mem = CPUMem { ram : [0; RAM_SIZE] };

        for i in 0..(RAM_LAST + 1) {
            mem[i] = (i % RAM_SIZE) as u8;
        }
        for i in 0..(RAM_LAST + 1) {
            assert_eq!(mem[i], (i % RAM_SIZE) as u8);
        }
    }

    #[test]
    fn lda() {
        let mut c = CPU::new();
        c.lda(0);
        assert_eq!(c.a, 0);
        assert_eq!(c.flags.z, true);
        assert_eq!(c.flags.n, false);
        c.lda(8);
        assert_eq!(c.a, 8);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.n, false);
        c.lda(255);
        assert_eq!(c.a, 255);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.n, true);
    }
    #[test]
    fn addr_modes() {
        let mut c = CPU::new();
        assert_eq!(c.immediate(0u8), 0u8);
        assert_eq!(c.immediate(255u8), 255u8);
        assert_eq!(c.absolute(0u16), 0u16);
        assert_eq!(c.absolute(65535u16), 65535u16);

        // fn absolute_x  (&self, val : u16) -> u16
        c.x = 0;
        assert_eq!(c.absolute_x(0u16), 0u16);
        assert_eq!(c.absolute_x(65535u16), 65535u16);


        // fn absolute_y  (&self, val : u16) -> u16
        // fn zero_page   (&self, val : u8)  -> u16
        // fn zero_page_x (&self, val : u8)  -> u16
        // fn zero_page_y (&self, val : u8)  -> u16
        // fn indirect_x  (&self, val : u16) -> u16
        // fn indirect_y  (&self, val : u16) -> u16
        // fn indirect    (&self, val : u16) -> u16
    }
}
