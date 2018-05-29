#[cfg(test)]
mod tests;
mod instructions;

use cartridge::Cartridge;
use super::{ ComponentRc, PPU, APU, Controller };
use Memory;

const RAM_FIRST     : u16 = 0x0000;
const RAM_LAST      : u16 = 0x1FFF;
const RAM_SIZE      : u16 = 0x0800;
const CART_FIRST    : u16 = 0x4020;
const CART_LAST     : u16 = 0xFFFF;
const PPUREGS_FIRST : u16 = 0x2000;
const PPUREGS_LAST  : u16 = 0x3FFF;
const PPUREGS_SIZE  : u16 = 0x0008;


pub struct CPUMem {
    // 0000 - 07FF : ram
    // 0800 - 1FFF : mirrors of ram
    // 2000 - 2007 : PPU regs
    // 2008 - 3FFF : Mirrors of PPU regs
    // 4000 - 4017 : APU and IO regs
    // 4018 - 401F : test mode stuff
    // 4020 - FFFF : cartridge
    ram : [u8; RAM_SIZE as usize],
    cart : ComponentRc<Cartridge>,
    ppu  : ComponentRc<PPU>,
    apu  : ComponentRc<APU>,
    controller : ComponentRc<Controller>,
}

fn split_bytes(val : u16) -> (u8, u8) {
    ((val >> 8) as u8, val as u8)
}

fn concat_bytes(high : u8, low : u8) -> u16 {
    ((high as u16) << 8) + low as u16
}

impl Memory for CPUMem {
    fn loadb(&self, addr : u16) -> u8 {
        match addr {
            RAM_FIRST...RAM_LAST => self.ram[(addr % RAM_SIZE) as usize],
            CART_FIRST...CART_LAST => self.cart.borrow().loadb(addr),
            PPUREGS_FIRST...PPUREGS_LAST =>
                self.ppu.borrow().reg_read((addr % PPUREGS_SIZE) as u8),
            _ => panic!(("couldn't map addr 0x{:04x} to CPU memory", addr)),
        }
    }
    fn storeb(&mut self, addr : u16, val : u8) {
        match addr {
            RAM_FIRST...RAM_LAST => self.ram[(addr % RAM_SIZE) as usize] = val,
            CART_FIRST...CART_LAST => self.cart.borrow_mut().storeb(addr, val),
            PPUREGS_FIRST...PPUREGS_LAST =>
                self.ppu.borrow_mut().reg_write((addr % PPUREGS_SIZE) as u8, val),
            _ => panic!("couldn't map addr 0x{:04x} to CPU memory", addr),
        }
    }
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
    cycles : usize,
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

// The number of cycles that each machine operation takes. Indexed by opcode number.
//
// This is copied from FCEU.
static CYCLE_TABLE: [usize; 256] = [
    /*0x00*/ 7,6,2,8,3,3,5,5,3,2,2,2,4,4,6,6,
    /*0x10*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
    /*0x20*/ 6,6,2,8,3,3,5,5,4,2,2,2,4,4,6,6,
    /*0x30*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
    /*0x40*/ 6,6,2,8,3,3,5,5,3,2,2,2,3,4,6,6,
    /*0x50*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
    /*0x60*/ 6,6,2,8,3,3,5,5,4,2,2,2,5,4,6,6,
    /*0x70*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
    /*0x80*/ 2,6,2,6,3,3,3,3,2,2,2,2,4,4,4,4,
    /*0x90*/ 2,6,2,6,4,4,4,4,2,5,2,5,5,5,5,5,
    /*0xA0*/ 2,6,2,6,3,3,3,3,2,2,2,2,4,4,4,4,
    /*0xB0*/ 2,5,2,5,4,4,4,4,2,4,2,4,4,4,4,4,
    /*0xC0*/ 2,6,2,8,3,3,5,5,2,2,2,2,4,4,6,6,
    /*0xD0*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
    /*0xE0*/ 2,6,3,8,3,3,5,5,2,2,2,2,4,4,6,6,
    /*0xF0*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
];

const STACK_BEGIN : u16 = 0x100;

impl CPU {
    pub fn step (&mut self) {
        let op = self.pc_getb();

        // println!("executing opcode 0x{:X} at pc 0x{:X}", op, self.pc - 1);

        self.add_cycles(CYCLE_TABLE[op as usize]);
        instructions::decode::INSTR[op as usize](self);
    }

    pub fn get_pc (&self) -> u16 { self.pc }

    fn add_cycles(&mut self, c : usize) { self.cycles += c; }
    fn reset_cycles(&mut self) { self.cycles = 0; }
    fn get_cycles(&self) -> usize { self.cycles }

    fn push(&mut self, val : u8) {
        self.mem.storeb(STACK_BEGIN + self.sp as u16, val);
        self.sp -= 1;
    }

    fn pop(&mut self) -> u8 {
        self.sp += 1;
        self.mem.loadb(STACK_BEGIN + self.sp as u16)
    }

    pub fn nmi(&mut self) {
        // 7-clock interrupt sequence, same timing as BRK
        self.add_cycles(7);

        let (ret_high, ret_low) = split_bytes(self.pc + 1);
        self.push(ret_high);
        self.push(ret_low);

        let dest_high = self.mem.loadb(0xFFFB);
        let dest_low = self.mem.loadb(0xFFFA);
        self.pc = concat_bytes(dest_high, dest_low);

        let status = self.flags.to_byte();
        self.push(status);

        self.flags.i = true;
    }

    fn pc_getdb(&mut self) -> u16  {
        let ret = self.mem.loadb(self.pc) as u16 +
            ((self.mem.loadb(self.pc + 1) as u16) << 8);
        self.pc += 2;
        ret
    }

    fn pc_getb(&mut self) -> u8 {
        let ret = self.mem.loadb(self.pc);
        self.pc += 1;
        ret
    }

    fn set_z(&mut self, result : u8) {
        self.flags.z = result == 0;
    }

    fn set_n(&mut self, result : u8) {
        self.flags.n = result & 0x80 != 0;
    }
    pub fn test() -> CPU {
        let cart = ComponentRc::new(Cartridge::test());
        let ppu  = ComponentRc::new(PPU::new(cart.new_ref()));
        let apu  = ComponentRc::new(APU::new());
        let controller = ComponentRc::new(Controller::new());

        CPU::new(cart, ppu, apu, controller)
    }

    pub fn new(cart : ComponentRc<Cartridge>,
               ppu  : ComponentRc<PPU>,
               apu  : ComponentRc<APU>,
               controller : ComponentRc<Controller> ) -> CPU {
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
                ram : [0; RAM_SIZE as usize],
                cart : cart,
                ppu : ppu,
                apu : apu,
                controller : controller,
            },
            cycles : 0,
        }
    }
}
