
#[cfg(test)]
mod tests;
mod decode;
// use super::cartridge::Cartridge;

const RAM_FIRST : usize = 0x0000;
const RAM_SIZE : usize = 0x0800;
const RAM_LAST : usize = 0x1FFF;
const CART_FIRST : usize = 0x4020;
const CART_LAST : usize = 0xFFFF;

struct Cartridge {
    rom : [u8; 0x8000]
}
impl Cartridge {
    fn new() -> Cartridge {
        Cartridge {
            rom : [0; 0x8000]
        }
    }
}
impl Index<usize> for Cartridge {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.rom[index]
    }
}

impl IndexMut<usize> for Cartridge {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rom[index]
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
            CART_FIRST...CART_LAST => &self.cart[index - CART_FIRST],
            // TODO other types of memory
            _ => panic!("couldn't map the index to CPU memory"),
        }
    }
}

impl IndexMut<usize> for CPUMem {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            RAM_FIRST...RAM_LAST => &mut self.ram[index % RAM_SIZE],
            CART_FIRST...CART_LAST => &mut self.cart[index - CART_FIRST],
            // TODO other types of memory
            _ => panic!("couldn't map the index to CPU memory"),
        }
    }
}

// describes the possible types of arguments for instructions
enum InstrArg {
    Implied,
    Immediate(u8),
    Address(u16),
}

macro_rules! bad_instr {
    () => { panic!("illegal instruction") };
}

impl CPU {

    pub fn step (&mut self) {
        let op = self.pc_getb();
        decode::INSTR[op as usize](self);
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

    fn adc(&mut self, arg : InstrArg) {
        let val = match arg {
            InstrArg::Immediate(imm) => imm,
            InstrArg::Address(addr)  => self.mem[addr as usize],
            _                        => bad_instr!(),
        };
        self.flags.c = (val as u16) + (self.a as u16) > 0xFF;

        // xnor bit 7 of both nums (check if both nums have the same sign)
        let same_sign = !((val >> 7) ^ (self.a >> 7));
        let result = val.wrapping_add(self.a);
        // xor same_sign with bit 7 of result (check if result has same sign as )
        self.flags.v = val >> 7 & (same_sign ^ (result >> 7)) != 0;
    }

    fn eor(&mut self, arg : InstrArg) {
        let val = self.a ^ match arg {
            InstrArg::Immediate(imm) => imm,
            InstrArg::Address(addr)  => self.mem[addr as usize],
            _                        => bad_instr!(),
        };
        self.set_z(val);
        self.set_n(val);

        self.a = val;
    }


    fn and(&mut self, arg : InstrArg) {
        let val = self.a & match arg {
            InstrArg::Immediate(imm) => imm,
            InstrArg::Address(addr)  => self.mem[addr as usize],
            _                        => bad_instr!(),
        };
        self.set_z(val);
        self.set_n(val);

        self.a = val;
    }

    fn ora(&mut self, arg : InstrArg) {
        let val = self.a | match arg {
            InstrArg::Immediate(imm) => imm,
            InstrArg::Address(addr)  => self.mem[addr as usize],
            _                        => bad_instr!(),
        };
        self.set_z(val);
        self.set_n(val);

        self.a = val;
    }

    fn ldy(&mut self, arg : InstrArg) {
        let val = match arg {
            InstrArg::Immediate(imm) => imm,
            InstrArg::Address(addr)  => self.mem[addr as usize],
            _                        => bad_instr!(),
        };
        self.set_z(val);
        self.set_n(val);

        self.y = val;
    }

    fn ldx(&mut self, arg : InstrArg) {
        let val = match arg {
            InstrArg::Immediate(imm) => imm,
            InstrArg::Address(addr)  => self.mem[addr as usize],
            _                        => bad_instr!(),
        };
        self.set_z(val);
        self.set_n(val);

        self.x = val;
    }

    fn lda(&mut self, arg : InstrArg) {
        let val = match arg {
            InstrArg::Immediate(imm) => imm,
            InstrArg::Address(addr)  => self.mem[addr as usize],
            _                        => bad_instr!(),
        };
        self.set_z(val);
        self.set_n(val);

        self.a = val;
    }

    fn sta(&mut self, arg : InstrArg) {
        match arg {
            InstrArg::Address(addr) => self.mem[addr as usize] = self.a,
            _                       => bad_instr!(),
        }
    }

    fn stx(&mut self, arg : InstrArg) {
        match arg {
            InstrArg::Address(addr) => self.mem[addr as usize] = self.x,
            _                       => bad_instr!(),
        }
    }

    fn sty(&mut self, arg : InstrArg) {
        match arg {
            InstrArg::Address(addr) => self.mem[addr as usize] = self.y,
            _                       => bad_instr!(),
        }
    }

    pub fn new() -> CPU {
        CPU {
            a : 0,
            x : 0,
            y : 0,
            sp : 0,
            pc : 0x8000,
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
                cart : Cartridge::new(),
            },
        }
    }
}
