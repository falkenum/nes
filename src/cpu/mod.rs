#[cfg(test)]
mod tests;
mod instructions;
mod debugger;

use self::instructions::InstrArg;
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

const IO_FIRST : u16 = 0x4000;
const IO_LAST  : u16 = 0x4017;

const OAMDMA : u16 = 0x4014;
const CONTROLLER_1 : u16 = 0x4016;


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
    stalled_cycles : usize,
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
                self.ppu.borrow_mut().reg_read((addr % PPUREGS_SIZE) as u8),
            OAMDMA => 0,
            CONTROLLER_1 => self.controller.borrow_mut().read_next(),
            IO_FIRST...IO_LAST => 0, //TODO
            _ => panic!("couldn't map addr 0x{:04x} to CPU memory", addr),
        }
    }
    fn storeb(&mut self, addr : u16, val : u8) {
        match addr {
            RAM_FIRST...RAM_LAST => self.ram[(addr % RAM_SIZE) as usize] = val,
            CART_FIRST...CART_LAST => self.cart.borrow_mut().storeb(addr, val),
            PPUREGS_FIRST...PPUREGS_LAST =>
                self.ppu.borrow_mut().reg_write((addr % PPUREGS_SIZE) as u8, val),
            OAMDMA => self.oamdma(val),
            CONTROLLER_1 => self.controller.borrow_mut().set_strobe(val),
            IO_FIRST...IO_LAST => (), //TODO
            _ => panic!("couldn't map addr 0x{:04x} to CPU memory", addr),
        }
    }
}

impl CPUMem {
    fn oamdma(&mut self, page_num : u8) {
        let src_addr = (page_num as u16) << 8;

        for i in 0..256 {
            let val = self.loadb(src_addr + i);
            self.ppu.borrow_mut().oamdma_write(val);
        }

        // 514 cycles for each oamdma
        // TODO odd/even timing?
        // TODO figure out a better solution for stalling oamdma cycles
        self.stalled_cycles = 514;
    }

    fn fetch_stalled_cycles(&mut self) -> usize {
        let ret = self.stalled_cycles;
        self.stalled_cycles = 0;
        ret
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
    interrupt_status : InterruptStatus,
    cycles : usize,
    debug_mode : bool,
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

const STACK_BEGIN : u16 = 0x100;

#[derive(Debug, PartialEq)]
enum InterruptStatus {
    None,
    NMI,
    IRQ,
    Reset,
}

impl CPU {

    pub fn step_for_scanlines(&mut self, num_scanlines : usize) {
        const CYCLES_PER_SCANLINE : usize = 114;
        while self.cycles < CYCLES_PER_SCANLINE*num_scanlines {
            self.cycles += self.step();
        }
        self.cycles -= CYCLES_PER_SCANLINE*num_scanlines;
    }

    // does interrupt if requested
    // executes next instruction, returns cycles passed
    pub fn step(&mut self) -> usize {
        let mut cycles = 0;

        const INTERRUPT_CYCLES : usize = 7;

        match self.interrupt_status {
            InterruptStatus::NMI => {
                self.nmi();
                cycles += INTERRUPT_CYCLES;
            },
            InterruptStatus::Reset => {
                self.reset();
                cycles += INTERRUPT_CYCLES;
            },
            InterruptStatus::IRQ => unimplemented!(),
            InterruptStatus::None => (),
        };

        self.interrupt_status = InterruptStatus::None;

        let decode_result = instructions::decode::fetch_and_decode(self);
        let op = decode_result.op;
        let op_str = decode_result.op_str;

        cycles += decode_result.num_cycles;

        if self.debug_mode {
            println!("at PC 0x{:04X}: {}", self.pc, op_str);
        }

        (op.instr)(self, op.arg);

        // TODO test
        cycles += self.mem.fetch_stalled_cycles();

        cycles
    }

    pub fn get_pc(&self) -> u16 { self.pc }

    fn push(&mut self, val : u8) {
        self.mem.storeb(STACK_BEGIN + self.sp as u16, val);
        self.sp = self.sp.wrapping_sub(1);
    }

    fn pop(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.mem.loadb(STACK_BEGIN + self.sp as u16)
    }

    pub fn send_reset(&mut self) {
        debug_assert_eq!(self.interrupt_status, InterruptStatus::None);
        self.interrupt_status = InterruptStatus::Reset;
    }

    // to be called at startup of the NES
    fn reset(&mut self) {
        let dest_high = self.mem.loadb(0xFFFD);
        let dest_low = self.mem.loadb(0xFFFC);
        self.pc = concat_bytes(dest_high, dest_low);
    }

    pub fn send_nmi(&mut self) {
        debug_assert_eq!(self.interrupt_status, InterruptStatus::None);
        self.interrupt_status = InterruptStatus::NMI;
    }

    fn nmi(&mut self) {

        let (ret_high, ret_low) = split_bytes(self.pc);
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

    fn unimpl(&mut self, _ : InstrArg) {
        panic!("called unimplemented instruction");
    }

    pub fn test() -> CPU {
        let cart = Cartridge::test_ref();
        let ppu  = ComponentRc::new(PPU::new(cart.new_ref()));
        let apu  = ComponentRc::new(APU::new());
        let controller = ComponentRc::new(Controller::new());

        CPU::new(cart, ppu, apu, controller, false)
    }

    pub fn new(cart : ComponentRc<Cartridge>,
               ppu  : ComponentRc<PPU>,
               apu  : ComponentRc<APU>,
               controller : ComponentRc<Controller>, debug_mode : bool) -> CPU {

        CPU {
            a : 0,
            x : 0,
            y : 0,
            sp : 0xFF,
            pc : 0x8000,
            cycles : 0,
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
                stalled_cycles : 0,
            },
            interrupt_status : InterruptStatus::None,
            debug_mode : debug_mode,
        }
    }
}
