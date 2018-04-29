
mod decode;
use super::cartridge::Cartridge;

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
use std::fmt;
impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CPU reg a: {}", self.a)
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

// describes the possible types of arguments for instructions
enum InstrArg {
    Empty,
    OneByte(u8),
    TwoByte(u16),
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

    fn lda(&mut self, arg : InstrArg) {
        let val;
        match arg {
            // immediate value
            InstrArg::OneByte(imm)  => val = imm,
            InstrArg::TwoByte(addr) => val = self.mem[addr as usize],
            _                       => panic!("illegal instruction"),
        }
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
                cart : Cartridge::empty(),
            },
        }
    }

    pub fn load_cartridge(&mut self, newcart : Cartridge) {
        self.mem.cart = newcart;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ram() {
        let mut c = CPU::new();
        let mut mem = c.mem;

        for i in 0..(RAM_LAST + 1) {
            mem[i] = (i % RAM_SIZE) as u8;
        }
        for i in 0..(RAM_LAST + 1) {
            assert_eq!(mem[i], (i % RAM_SIZE) as u8);
        }
    }

    #[test]
    fn lda() {
        // let lda_imm = instructions::INSTR[0xA9];
        // lda_imm(&mut c);
        let mut c = CPU::new();
        c.lda(InstrArg::OneByte(0x00));
        assert_eq!(c.a, 0x00);
        assert_eq!(c.flags.z, true);
        assert_eq!(c.flags.n, false);
        c.lda(InstrArg::OneByte(0xFF));
        assert_eq!(c.a, 0xFF);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.n, true);
    }
    #[test]
    fn addr_modes() {

        let mut c = CPU::new();

        c.x = 0x00;
        assert_eq!(c.absolute_x(0x0000_u16), 0x0000_u16);
        assert_eq!(c.absolute_x(0xFFFF_u16), 0xFFFF_u16);
        c.x = 0xFF;
        assert_eq!(c.absolute_x(0x0000_u16), 0x00FF_u16);
        assert_eq!(c.absolute_x(0xFFFF_u16), 0x00FE_u16);

        c.y = 0x00;
        assert_eq!(c.absolute_y(0x0000_u16), 0x0000_u16);
        assert_eq!(c.absolute_y(0xFFFF_u16), 0xFFFF_u16);
        c.y = 0xFF;
        assert_eq!(c.absolute_y(0x0000_u16), 0x00FF_u16);
        assert_eq!(c.absolute_y(0xFFFF_u16), 0x00FE_u16);

        assert_eq!(c.zero_page(0x00_u8), 0x0000_u16);
        assert_eq!(c.zero_page(0xFF_u8), 0x00FF_u16);

        c.x = 0x00;
        assert_eq!(c.zero_page_x(0x00_u8), 0x0000_u16);
        assert_eq!(c.zero_page_x(0xFF_u8), 0x00FF_u16);
        c.x = 0xFF;
        assert_eq!(c.zero_page_x(0x00_u8), 0x00FF_u16);
        // this is an interesting case: zero page addressing with index
        // specifies (according to MOS datasheet) essentially that
        // the index is added to the argument before it is extended to
        // 16 bits, so any carry from that addition is dropped
        assert_eq!(c.zero_page_x(0xFF_u8), 0x00FE_u16);

        c.y = 0x00;
        assert_eq!(c.zero_page_y(0x00_u8), 0x0000_u16);
        assert_eq!(c.zero_page_y(0xFF_u8), 0x00FF_u16);
        c.y = 0xFF;
        assert_eq!(c.zero_page_y(0x00_u8), 0x00FF_u16);
        assert_eq!(c.zero_page_y(0xFF_u8), 0x00FE_u16);

        c.mem[0x0] = 0xA;
        c.mem[0x1] = 0xB;
        c.mem[0x2] = 3;
        c.mem[0xFE] = 1;
        c.mem[0xFF] = 2;
        c.mem[0x100] = 4;
        c.mem[0x1FF] = 5;
        c.mem[0x200] = 6;

        assert_eq!(c.indirect(0x0000_u16), 0x0B0A_u16);
        assert_eq!(c.indirect(0x00FE_u16), 0x0201_u16);

        // wrap around
        assert_eq!(c.indirect(0x00FF_u16), 0x0A02_u16);
        // wrap around
        assert_eq!(c.indirect(0x01FF_u16), 0x0405_u16);

        c.x = 0x00;
        assert_eq!(c.indirect_x(0x00_u8), 0x0B0A_u16);
        assert_eq!(c.indirect_x(0xFE_u8), 0x0201_u16);
        c.x = 0xFE;
        // wrap around
        assert_eq!(c.indirect_x(0x02_u8), 0x0B0A_u16);
        assert_eq!(c.indirect_x(0x00_u8), 0x0201_u16);

        // wrap around
        assert_eq!(c.indirect_x(0x01_u8), 0x0A02_u16);

        c.y = 0x00;
        assert_eq!(c.indirect_y(0x00_u8), 0x0B0A_u16);
        // wrap around
        assert_eq!(c.indirect_y(0xFF_u8), 0x0A02_u16);

        // All kinds of wrap around that I'm still confused about
        c.y = 0x01;
        assert_eq!(c.indirect_y(0xFF_u8), 0x0A03_u16);

        c.y = 0xFE;
        assert_eq!(c.indirect_y(0xFF_u8), 0x0B00_u16);
    }
}
