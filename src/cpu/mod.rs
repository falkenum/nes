
#![allow(dead_code)]

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

struct Flags {
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
    flags : Flags,
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
    pub fn step (&mut self) {
    }
    pub fn new() -> CPU {
        CPU {
            a : 0,
            x : 0,
            y : 0,
            sp : 0,
            pc : 0,
            flags : Flags {
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
    fn test_cpu_mem() {
        let mut mem = CPUMem { ram : [0; RAM_SIZE] };

        mem.ram[8 as usize] = 8;
        // for i in (&mut mem.ram).iter() {
        // }

        assert_eq!(mem[8], 8);
        // assert_eq!(mem[0x7FF], 1);
        // assert_eq!(mem[0x7FF], 1);
    }
}
