#[cfg(test)]
mod tests;
mod instructions;

const RAM_FIRST : usize = 0x0000;
const RAM_SIZE : usize = 0x0800;
const RAM_LAST : usize = 0x1FFF;
const CART_FIRST : usize = 0x4020;
const CART_LAST : usize = 0xFFFF;

struct Cartridge {
    rom : [u8; 0x8000],
    irq_vec : [u8; 2],
}
impl Cartridge {
    fn new() -> Cartridge {
        Cartridge {
            rom : [0; 0x8000],
            irq_vec: [0; 2],
        }
    }
}
impl Index<usize> for Cartridge {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0xFFFE => &self.irq_vec[0],
            0xFFFF => &self.irq_vec[1],
            _      => &self.rom[index - CART_FIRST],
        }
    }
}

impl IndexMut<usize> for Cartridge {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0xFFFE => &mut self.irq_vec[0],
            0xFFFF => &mut self.irq_vec[1],
            _      => &mut self.rom[index - CART_FIRST],
        }
    }
}

struct CPUMem {
    // 0000 - 07FF : ram
    // 0800 - 1FFF : mirrors of ram
    // 2000 - 2007 : PPU regs
    // 2008 - 3FFF : Mirrors of PPU regs
    // 4000 - 4017 : APU and IO regs
    // 4018 - 401F : test mode stuff
    // 4020 - FFFF : cartridge
    ram : [u8; RAM_SIZE],
    cart : Cartridge,
}


struct CPUFlags {
    n : bool,
    v : bool,
    d : bool,
    i : bool,
    z : bool,
    c : bool,
}
impl CPUFlags {
    fn from_byte(val : u8) -> CPUFlags {
        CPUFlags {
            n : (val & 0x80 != 0),
            v : (val & 0x40 != 0),
            d : (val & 0x08 != 0),
            i : (val & 0x04 != 0),
            z : (val & 0x02 != 0),
            c : (val & 0x01 != 0),
        }
    }
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
// use std::fmt;
// impl fmt::Debug for CPU {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "CPU reg a: {}", self.a)
//     }
// }

use std::ops::{ Index, IndexMut };
impl Index<usize> for CPUMem {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            RAM_FIRST...RAM_LAST => &self.ram[index % RAM_SIZE],
            CART_FIRST...CART_LAST => &self.cart[index],
            // TODO other types of memory
            _ => panic!("couldn't map the index to CPU memory"),
        }
    }
}

impl IndexMut<usize> for CPUMem {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            RAM_FIRST...RAM_LAST => &mut self.ram[index % RAM_SIZE],
            CART_FIRST...CART_LAST => &mut self.cart[index],
            // TODO other types of memory
            _ => panic!("couldn't map the index to CPU memory"),
        }
    }
}


impl CPU {

    pub fn step (&mut self) {
        let op = self.pc_getb();
        self.exec_op(op);
    }

    fn exec_op(&mut self, op : u8) {
        instructions::decode::INSTR[op as usize](self);
    }

    fn pc_getdb(&mut self) -> u16  {
        let ret = self.mem[self.pc as usize] as u16 +
            ((self.mem[(self.pc + 1) as usize] as u16) << 8);
        self.pc += 2;
        ret
    }
    fn pc_getb(&mut self) -> u8 {
        let ret = self.mem[self.pc as usize];
        self.pc += 1;
        ret
    }

    fn set_z(&mut self, result : u8) {
        self.flags.z = result == 0;
    }

    fn set_n(&mut self, result : u8) {
        self.flags.n = result & 0x80 != 0;
    }

    pub fn new() -> CPU {
        CPU {
            a : 0,
            x : 0,
            y : 0,
            sp : 0xFF,
            pc : 0x8000,
            flags : CPUFlags {
                n : false,
                v : false,
                d : false,
                i : false,
                z : false,
                c : false,
            },
            mem : CPUMem {
                ram : [0; RAM_SIZE],
                cart : Cartridge::new(),
            },
        }
    }
}
