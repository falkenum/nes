#[cfg(test)]
mod tests;
mod instructions;

use cartridge::Cartridge;
use std::cell::RefCell;
use std::rc::Rc;

const RAM_FIRST : usize = 0x0000;
const RAM_SIZE : usize = 0x0800;
const RAM_LAST : usize = 0x1FFF;
const CART_FIRST : usize = 0x4020;
const CART_LAST : usize = 0xFFFF;


struct CPUMem {
    // 0000 - 07FF : ram
    // 0800 - 1FFF : mirrors of ram
    // 2000 - 2007 : PPU regs
    // 2008 - 3FFF : Mirrors of PPU regs
    // 4000 - 4017 : APU and IO regs
    // 4018 - 401F : test mode stuff
    // 4020 - FFFF : cartridge
    ram : [u8; RAM_SIZE],
    cart : Rc<Cartridge>,
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
    fn to_byte(&self) -> u8 {
        ((self.n as u8) << 7) +
        ((self.v as u8) << 6) +
                     (1 << 5) + // unused
                     (0 << 4) + // b flag
        ((self.d as u8) << 3) +
        ((self.i as u8) << 2) +
        ((self.z as u8) << 1) +
         (self.c as u8)
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

use std::fmt;
impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "reg a: 0x{:02X}\
                 \nreg x: 0x{:02X}\
                 \nreg y: 0x{:02X}\
                 \n   sp: 0x{:02X}\
                 \n   pc: 0x{:04X}\
                 \nflags: nv_bdizc\
                 \n------ {:08b}",
               self.a, self.x, self.y, self.sp, self.pc, self.flags.to_byte())
    }
}

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
        println!("executing opcode 0x{:X} at pc 0x{:X}", op, self.pc - 1);
        self.exec_op(op);
    }
    pub fn get_pc (&self) -> u16 { self.pc }

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

    pub fn new(cart : Rc<Cartridge>) -> CPU {
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
                cart : cart,
            },
        }
    }
}
