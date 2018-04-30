
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
            InstrArg::Immediate(imm)  => val = imm,
            InstrArg::Address(addr) => val = self.mem[addr as usize],
            _                       => panic!("illegal instruction"),
        }
        // checking if val is zero or negative
        self.flags.z = val        == 0;
        self.flags.n = val & 0x80 != 0;

        self.a = val;
    }

    fn sta(&mut self, arg : InstrArg) {
        match arg {
            InstrArg::Address(addr) => self.mem[addr as usize] = self.a,
            _                       => panic!("illegal instruction"),
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

    // pub fn load_cartridge(&mut self, newcart : Cartridge) {
    //     self.mem.cart = newcart;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ram() {
        let c = CPU::new();
        let mut mem = c.mem;

        for i in 0..(RAM_LAST + 1) {
            mem[i] = (i % RAM_SIZE) as u8;
        }
        for i in 0..(RAM_LAST + 1) {
            assert_eq!(mem[i], (i % RAM_SIZE) as u8);
        }
    }

    #[test]
    fn sta() {
        let mut c = CPU::new();
        // zero page
        c.a = 3;
        c.mem[0x8000] = 0x85;
        c.mem[0x8001] = 0x10;
        c.step();
        assert_eq!(c.mem[0x0010], 3);

        let mut c = CPU::new();
        // zero page, X
        c.a = 3;
        c.x = 1;
        c.mem[0x8000] = 0x95;
        c.mem[0x8001] = 0x10;
        c.step();
        assert_eq!(c.mem[0x0011], 3);

        let mut c = CPU::new();
        // absolute
        c.a = 3;
        c.mem[0x8000] = 0x8D;
        c.mem[0x8001] = 0xFF;
        c.mem[0x8002] = 0x10;
        c.step();
        assert_eq!(c.mem[0x10FF], 3);

        let mut c = CPU::new();
        // absolute, x
        c.a = 3;
        c.x = 1;
        c.mem[0x8000] = 0x9D;
        c.mem[0x8001] = 0xFE;
        c.mem[0x8002] = 0x10;
        c.step();
        assert_eq!(c.mem[0x10FF], 3);

        let mut c = CPU::new();
        // absolute, y
        c.a = 3;
        c.y = 1;
        c.mem[0x8000] = 0x99;
        c.mem[0x8001] = 0xFE;
        c.mem[0x8002] = 0x10;
        c.step();
        assert_eq!(c.mem[0x10FF], 3);

        let mut c = CPU::new();
        // indirect, x
        c.a = 3;
        c.x = 1;
        c.mem[0x00FF] = 0x10FF;
        c.mem[0x8000] = 0x81;
        c.mem[0x8001] = 0xFE;
        c.step();
        assert_eq!(c.mem[0x10FF], 3);

        let mut c = CPU::new();
        // indirect, y
        c.a = 3;
        c.y = 1;
        c.mem[0x00FF] = 0x10FE;
        c.mem[0x8000] = 0x91;
        c.mem[0x8001] = 0xFF;
        c.step();
        assert_eq!(c.mem[0x10FF], 3);
    }

    #[test]
    fn lda() {
        let mut c = CPU::new();
        c.mem[0x00] = 0x0A;
        c.mem[0x01] = 0x0B;
        c.mem[0x0FF] = 0x0C;
        c.mem[0x1FFF] = 0x0D;

        // lda #$07 (immediate)
        c.mem[0x8000] = 0xA9;
        c.mem[0x8001] = 0x07;
        c.step();
        assert_eq!(c.a, 0x7);

        // lda $01 (zero page)
        c.mem[0x8002] = 0xA5;
        c.mem[0x8003] = 0x01;
        c.step();
        assert_eq!(c.a, 0xB);

        c.x = 0xFE;
        // lda $01,X (zero page, x)
        c.mem[0x8004] = 0xB5;
        c.mem[0x8005] = 0x01;
        c.step();
        assert_eq!(c.a, 0xC);

        // lda $1FFF (absolute)
        c.mem[0x8006] = 0xAD;
        c.mem[0x8007] = 0xFF;
        c.mem[0x8008] = 0x1F;
        c.step();
        assert_eq!(c.a, 0xD);

        c.x = 1;
        // lda $1000,X (absolute x)
        c.mem[0x8009] = 0xBD;
        c.mem[0x800A] = 0x00;
        c.mem[0x800B] = 0x10;
        c.step();
        assert_eq!(c.a, 0xB);

        c.y = 0xFF;
        // lda $1000,Y (absolute y)
        c.mem[0x800C] = 0xB9;
        c.mem[0x800D] = 0x00;
        c.mem[0x800E] = 0x10;
        c.step();
        assert_eq!(c.a, 0xC);

        c.mem[0x00FE] = 0xAA;
        c.mem[0x00FF] = 0xFE;
        c.mem[0x0000] = 0x00;
        c.x = 0xF;

        // lda ($F0,X) (indirect x)
        c.mem[0x800F] = 0xA1;
        c.mem[0x8010] = 0xF0;
        c.step();
        assert_eq!(c.a, 0xAA);

        c.mem[0x00FD] = 0xBB;
        c.mem[0x00F0] = 0x0D;
        c.y = 0xF0;
        // lda ($F0),Y (indirect y)
        c.mem[0x8011] = 0xB1;
        c.mem[0x8012] = 0xF0;
        c.step();
        assert_eq!(c.a, 0xBB);
    }

    #[test]
    fn mapping() {
        let mut c = CPU::new();

        c.lda(InstrArg::Immediate(0x00));
        assert_eq!(c.a, 0x00);
        assert_eq!(c.flags.z, true);
        assert_eq!(c.flags.n, false);
        c.lda(InstrArg::Immediate(0xFF));
        assert_eq!(c.a, 0xFF);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.n, true);

        c.mem[0x0] = 0xA;
        c.mem[0x1] = 0xB;
        c.mem[0xFF] = 3;
        c.mem[0x7FF] = 4;

        c.lda(InstrArg::Address(0x0000));
        assert_eq!(c.a, 0xA);
        assert_eq!(c.mem[0x0800], 0xA);
        assert_eq!(c.mem[0x1000], 0xA);
        assert_eq!(c.mem[0x1800], 0xA);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.n, false);
        c.lda(InstrArg::Address(0x00FF));
        assert_eq!(c.a, 0x3);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.n, false);
        c.lda(InstrArg::Address(0x07FF));
        assert_eq!(c.a, 0x4);
        assert_eq!(c.mem[0x0FFF], 0x4);
        assert_eq!(c.mem[0x17FF], 0x4);
        assert_eq!(c.mem[0x1FFF], 0x4);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.n, false);

        c.mem[0x1FFF] = 5;
        assert_eq!(c.mem[0x07FF], 0x5);
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
