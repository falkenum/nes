use super::{ InstrArg, CPU, Memory };

#[cfg(test)]
mod tests;

#[derive(Copy, Clone)]
pub enum AddrMode {
    Implied,
    Immediate,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Indirect,
    IndirectX,
    IndirectY,
}

pub struct DecodeResult {
    pub num_cycles : usize,
    pub op : Op,
}

pub struct Op {
    pub instr : fn(&mut CPU, InstrArg),
    pub arg : InstrArg,
}

// this isn't a method because I didn't want it to be usable outside of the cpu mod
pub fn fetch_and_decode(cpu : &mut CPU) -> DecodeResult {
    let op = cpu.pc_getb() as usize;

    let (mode, instr) = constants::OPS[op];

    let mut num_cycles = constants::CYCLE_TABLE[op];

    let arg = match mode {
        AddrMode::Implied => InstrArg::Implied,
        AddrMode::Immediate => {
            let b = cpu.pc_getb();
            InstrArg::Immediate(b)
        },
        AddrMode::Relative => {
            let b = cpu.pc_getb();
            // if there is a page crossing, add one cycle
            // TODO test
            // TODO add 1 for taken branch
            // TODO check for negative num
            // if (cpu.pc & 0x00FF) + b as u16 > 0x00FF {
            //     num_cycles += 1;
            // }

            let addr = cpu.relative(b);

            InstrArg::Address(addr)
        },
        AddrMode::Absolute => {
            let addr = cpu.pc_getdb();
            InstrArg::Address(addr)
        },
        AddrMode::AbsoluteX => {
            let addr = cpu.pc_getdb();
            // if there is a page crossing, add one cycle
            // 4 cycles is for specific instructions (TODO elaborate)
            if num_cycles == 4 && ((addr & 0x00FF) + cpu.x as u16) > 0x00FF {
                num_cycles += 1;
            }
            InstrArg::Address(cpu.absolute_x(addr))
        },
        AddrMode::AbsoluteY => {
            let addr = cpu.pc_getdb();
            // if there is a page crossing, add one cycle
            if num_cycles == 4 && ((addr & 0x00FF) + cpu.x as u16) > 0x00FF {
                num_cycles += 1;
            }
            InstrArg::Address(cpu.absolute_y(addr))
        },
        AddrMode::ZeroPage => {
            let b = cpu.pc_getb();
            InstrArg::Address(cpu.zero_page(b))
        },
        AddrMode::ZeroPageX => {
            let b = cpu.pc_getb();
            InstrArg::Address(cpu.zero_page_x(b))
        },
        AddrMode::ZeroPageY => {
            let b = cpu.pc_getb();
            InstrArg::Address(cpu.zero_page_y(b))
        },
        AddrMode::Indirect => {
            let addr = cpu.pc_getdb();
            InstrArg::Address(cpu.indirect(addr))
        },
        AddrMode::IndirectX => {
            let b = cpu.pc_getb();
            InstrArg::Address(cpu.indirect_x(b))
        },
        AddrMode::IndirectY => {
            let b = cpu.pc_getb();
            // if there is a page crossing, add one cycle
            if num_cycles == 5 &&
                ((cpu.indirect(b as u16) & 0x00FF) + cpu.y as u16) > 0x00FF {

                num_cycles += 1;
            }
            let addr = cpu.indirect_y(b);
            InstrArg::Address(addr)
        },
    };

    DecodeResult {
        num_cycles : num_cycles,
        op : Op {
            instr : instr,
            arg : arg,
        },
    }
}

impl CPU {

    fn relative(&self, val : u8) -> u16 {
        // sign extend val and add to pc
        ((val as u16) | (0xFF00 * ((val >> 7) as u16))).wrapping_add(self.pc)
    }

    fn absolute_x  (&self, val : u16) -> u16 {
        val.wrapping_add(self.x as u16)
    }

    fn absolute_y  (&self, val : u16) -> u16 {
        val.wrapping_add(self.y as u16)
    }

    fn zero_page   (&self, val : u8)  -> u16 { val as u16 }
    fn zero_page_x (&self, val : u8)  -> u16 { val.wrapping_add(self.x) as u16 }
    fn zero_page_y (&self, val : u8)  -> u16 { val.wrapping_add(self.y) as u16 }
    fn indirect_x  (&self, val : u8)  -> u16 {
        let a = val.wrapping_add(self.x);
        self.indirect(a as u16)
    }
    fn indirect_y  (&self, val : u8)  -> u16 {
        let val = self.indirect(val as u16);
        self.absolute_y(val)
    }
    fn indirect    (&self, val : u16) -> u16 {
        let addr_low = val as u8;
        let addr_high = val & 0xFF00;
        let i = val;
        // We need to add 1 to the lower 8 bits separately in order to
        // accurately simulate how the 6502 handles page boundries -- A page is
        // 0xFF bytes.
        // If mem[0] = 1 and mem[FF] = 2, then JMP ($00FF) should evaluate
        // to JMP $0201
        let j = addr_low.wrapping_add(1) as u16 + addr_high;
        self.mem.loadb(i) as u16 + ((self.mem.loadb(j) as u16) << 8)
    }
}


mod constants {
    const NUM_OPCODES : usize = 256;
    use super::AddrMode;
    use super::AddrMode::*;
    use super::CPU;
    use super::InstrArg;

    // The number of cycles that each machine operation takes.
    // Indexed by opcode number.
    // This is copied from FCEU.
    pub static CYCLE_TABLE : [usize; 256] = [
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

    pub static OPS : [(AddrMode, fn(&mut CPU, InstrArg)); NUM_OPCODES] = [
        /* 0x00 */ (Implied,   CPU::brk    ),
        /* 0x01 */ (IndirectX, CPU::ora    ),
        /* 0x02 */ (Implied,   CPU::unimpl ),
        /* 0x03 */ (Implied,   CPU::unimpl ),
        /* 0x04 */ (ZeroPage,  CPU::nop    ),
        /* 0x05 */ (ZeroPage,  CPU::ora    ),
        /* 0x06 */ (ZeroPage,  CPU::asl    ),
        /* 0x07 */ (Implied,   CPU::unimpl ),
        /* 0x08 */ (Implied,   CPU::php    ),
        /* 0x09 */ (Immediate, CPU::ora    ),
        /* 0x0A */ (Implied,   CPU::asl    ),
        /* 0x0B */ (Implied,   CPU::unimpl ),
        /* 0x0C */ (Absolute,  CPU::nop    ),
        /* 0x0D */ (Absolute,  CPU::ora    ),
        /* 0x0E */ (Absolute,  CPU::asl    ),
        /* 0x0F */ (Implied,   CPU::unimpl ),
        /* 0x10 */ (Relative,  CPU::bpl    ),
        /* 0x11 */ (IndirectY, CPU::ora    ),
        /* 0x12 */ (Implied,   CPU::unimpl ),
        /* 0x13 */ (Implied,   CPU::unimpl ),
        /* 0x14 */ (ZeroPageX, CPU::nop    ),
        /* 0x15 */ (ZeroPageX, CPU::ora    ),
        /* 0x16 */ (ZeroPageX, CPU::asl    ),
        /* 0x17 */ (Implied,   CPU::unimpl ),
        /* 0x18 */ (Implied,   CPU::clc    ),
        /* 0x19 */ (AbsoluteY, CPU::ora    ),
        /* 0x1A */ (Implied,   CPU::nop    ),
        /* 0x1B */ (Implied,   CPU::unimpl ),
        /* 0x1C */ (AbsoluteX, CPU::nop    ),
        /* 0x1D */ (AbsoluteX, CPU::ora    ),
        /* 0x1E */ (AbsoluteX, CPU::asl    ),
        /* 0x1F */ (Implied,   CPU::unimpl ),
        /* 0x20 */ (Absolute,  CPU::jsr    ),
        /* 0x21 */ (IndirectX, CPU::and    ),
        /* 0x22 */ (Implied,   CPU::unimpl ),
        /* 0x23 */ (Implied,   CPU::unimpl ),
        /* 0x24 */ (ZeroPage,  CPU::bit    ),
        /* 0x25 */ (ZeroPage,  CPU::and    ),
        /* 0x26 */ (ZeroPage,  CPU::rol    ),
        /* 0x27 */ (Implied,   CPU::unimpl ),
        /* 0x28 */ (Implied,   CPU::plp    ),
        /* 0x29 */ (Immediate, CPU::and    ),
        /* 0x2A */ (Implied,   CPU::rol    ),
        /* 0x2B */ (Implied,   CPU::unimpl ),
        /* 0x2C */ (Absolute,  CPU::bit    ),
        /* 0x2D */ (Absolute,  CPU::and    ),
        /* 0x2E */ (Absolute,  CPU::rol    ),
        /* 0x2F */ (Implied,   CPU::unimpl ),
        /* 0x30 */ (Relative,  CPU::bmi    ),
        /* 0x31 */ (IndirectY, CPU::and    ),
        /* 0x32 */ (Implied,   CPU::unimpl ),
        /* 0x33 */ (Implied,   CPU::unimpl ),
        /* 0x34 */ (ZeroPageX, CPU::nop    ),
        /* 0x35 */ (ZeroPageX, CPU::and    ),
        /* 0x36 */ (ZeroPageX, CPU::rol    ),
        /* 0x37 */ (Implied,   CPU::unimpl ),
        /* 0x38 */ (Implied,   CPU::sec    ),
        /* 0x39 */ (AbsoluteY, CPU::and    ),
        /* 0x3A */ (Implied,   CPU::nop    ),
        /* 0x3B */ (Implied,   CPU::unimpl ),
        /* 0x3C */ (AbsoluteX, CPU::nop    ),
        /* 0x3D */ (AbsoluteX, CPU::and    ),
        /* 0x3E */ (AbsoluteX, CPU::rol    ),
        /* 0x3F */ (Implied,   CPU::unimpl ),
        /* 0x40 */ (Implied,   CPU::rti    ),
        /* 0x41 */ (IndirectX, CPU::eor    ),
        /* 0x42 */ (Implied,   CPU::unimpl ),
        /* 0x43 */ (Implied,   CPU::unimpl ),
        /* 0x44 */ (ZeroPage,  CPU::nop    ),
        /* 0x45 */ (ZeroPage,  CPU::eor    ),
        /* 0x46 */ (ZeroPage,  CPU::lsr    ),
        /* 0x47 */ (Implied,   CPU::unimpl ),
        /* 0x48 */ (Implied,   CPU::pha    ),
        /* 0x49 */ (Immediate, CPU::eor    ),
        /* 0x4A */ (Implied,   CPU::lsr    ),
        /* 0x4B */ (Implied,   CPU::unimpl ),
        /* 0x4C */ (Absolute,  CPU::jmp    ),
        /* 0x4D */ (Absolute,  CPU::eor    ),
        /* 0x4E */ (Absolute,  CPU::lsr    ),
        /* 0x4F */ (Implied,   CPU::unimpl ),
        /* 0x50 */ (Relative,  CPU::bvc    ),
        /* 0x51 */ (IndirectY, CPU::eor    ),
        /* 0x52 */ (Implied,   CPU::unimpl ),
        /* 0x53 */ (Implied,   CPU::unimpl ),
        /* 0x54 */ (ZeroPageX, CPU::nop    ),
        /* 0x55 */ (ZeroPageX, CPU::eor    ),
        /* 0x56 */ (ZeroPageX, CPU::lsr    ),
        /* 0x57 */ (Implied,   CPU::unimpl ),
        /* 0x58 */ (Implied,   CPU::cli    ),
        /* 0x59 */ (AbsoluteY, CPU::eor    ),
        /* 0x5A */ (Implied,   CPU::nop    ),
        /* 0x5B */ (Implied,   CPU::unimpl ),
        /* 0x5C */ (AbsoluteX, CPU::nop    ),
        /* 0x5D */ (AbsoluteX, CPU::eor    ),
        /* 0x5E */ (AbsoluteX, CPU::lsr    ),
        /* 0x5F */ (Implied,   CPU::unimpl ),
        /* 0x60 */ (Implied,   CPU::rts    ),
        /* 0x61 */ (IndirectX, CPU::adc    ),
        /* 0x62 */ (Implied,   CPU::unimpl ),
        /* 0x63 */ (Implied,   CPU::unimpl ),
        /* 0x64 */ (ZeroPage,  CPU::nop    ),
        /* 0x65 */ (ZeroPage,  CPU::adc    ),
        /* 0x66 */ (ZeroPage,  CPU::ror    ),
        /* 0x67 */ (Implied,   CPU::unimpl ),
        /* 0x68 */ (Implied,   CPU::pla    ),
        /* 0x69 */ (Immediate, CPU::adc    ),
        /* 0x6A */ (Implied,   CPU::ror    ),
        /* 0x6B */ (Implied,   CPU::unimpl ),
        /* 0x6C */ (Indirect,  CPU::jmp    ),
        /* 0x6D */ (Absolute,  CPU::adc    ),
        /* 0x6E */ (Absolute,  CPU::ror    ),
        /* 0x6F */ (Implied,   CPU::unimpl ),
        /* 0x70 */ (Relative,  CPU::bvs    ),
        /* 0x71 */ (ZeroPageY, CPU::adc    ),
        /* 0x72 */ (Implied,   CPU::unimpl ),
        /* 0x73 */ (Implied,   CPU::unimpl ),
        /* 0x74 */ (ZeroPageX, CPU::nop    ),
        /* 0x75 */ (ZeroPageX, CPU::adc    ),
        /* 0x76 */ (ZeroPageX, CPU::ror    ),
        /* 0x77 */ (Implied,   CPU::unimpl ),
        /* 0x78 */ (Implied,   CPU::sei    ),
        /* 0x79 */ (AbsoluteY, CPU::adc    ),
        /* 0x7A */ (Implied,   CPU::nop    ),
        /* 0x7B */ (Implied,   CPU::unimpl ),
        /* 0x7C */ (AbsoluteX, CPU::nop    ),
        /* 0x7D */ (AbsoluteX, CPU::adc    ),
        /* 0x7E */ (AbsoluteX, CPU::ror    ),
        /* 0x7F */ (Implied,   CPU::unimpl ),
        /* 0x80 */ (Immediate, CPU::nop    ),
        /* 0x81 */ (IndirectX, CPU::sta    ),
        /* 0x82 */ (Implied,   CPU::unimpl ),
        /* 0x83 */ (Implied,   CPU::unimpl ),
        /* 0x84 */ (ZeroPage,  CPU::sty    ),
        /* 0x85 */ (ZeroPage,  CPU::sta    ),
        /* 0x86 */ (ZeroPage,  CPU::stx    ),
        /* 0x87 */ (Implied,   CPU::unimpl ),
        /* 0x88 */ (Implied,   CPU::dey    ),
        /* 0x89 */ (Implied,   CPU::unimpl ),
        /* 0x8A */ (Implied,   CPU::txa    ),
        /* 0x8B */ (Implied,   CPU::unimpl ),
        /* 0x8C */ (Absolute,  CPU::sty    ),
        /* 0x8D */ (Absolute,  CPU::sta    ),
        /* 0x8E */ (Absolute,  CPU::stx    ),
        /* 0x8F */ (Implied,   CPU::unimpl ),
        /* 0x90 */ (Relative,  CPU::bcc    ),
        /* 0x91 */ (IndirectY, CPU::sta    ),
        /* 0x92 */ (Implied,   CPU::unimpl ),
        /* 0x93 */ (Implied,   CPU::unimpl ),
        /* 0x94 */ (ZeroPageX, CPU::sty    ),
        /* 0x95 */ (ZeroPageX, CPU::sta    ),
        /* 0x96 */ (ZeroPageY, CPU::stx    ),
        /* 0x97 */ (Implied,   CPU::unimpl ),
        /* 0x98 */ (Implied,   CPU::tya    ),
        /* 0x99 */ (AbsoluteY, CPU::sta    ),
        /* 0x9A */ (Implied,   CPU::txs    ),
        /* 0x9B */ (Implied,   CPU::unimpl ),
        /* 0x9C */ (Implied,   CPU::unimpl ),
        /* 0x9D */ (AbsoluteX, CPU::sta    ),
        /* 0x9E */ (Implied,   CPU::unimpl ),
        /* 0x9F */ (Implied,   CPU::unimpl ),
        /* 0xA0 */ (Immediate, CPU::ldy    ),
        /* 0xA1 */ (IndirectX, CPU::lda    ),
        /* 0xA2 */ (Immediate, CPU::ldx    ),
        /* 0xA3 */ (IndirectX, CPU::lax    ),
        /* 0xA4 */ (ZeroPage,  CPU::ldy    ),
        /* 0xA5 */ (ZeroPage,  CPU::lda    ),
        /* 0xA6 */ (ZeroPage,  CPU::ldx    ),
        /* 0xA7 */ (ZeroPage,  CPU::lax    ),
        /* 0xA8 */ (Implied,   CPU::tay    ),
        /* 0xA9 */ (Immediate, CPU::lda    ),
        /* 0xAA */ (Implied,   CPU::tax    ),
        /* 0xAB */ (Implied,   CPU::unimpl ),
        /* 0xAC */ (Absolute,  CPU::ldy    ),
        /* 0xAD */ (Absolute,  CPU::lda    ),
        /* 0xAE */ (Absolute,  CPU::ldx    ),
        /* 0xAF */ (Absolute,  CPU::lax    ),
        /* 0xB0 */ (Relative,  CPU::bcs    ),
        /* 0xB1 */ (IndirectY, CPU::lda    ),
        /* 0xB2 */ (Implied,   CPU::unimpl ),
        /* 0xB3 */ (ZeroPageY, CPU::lax    ),
        /* 0xB4 */ (ZeroPageX, CPU::ldy    ),
        /* 0xB5 */ (ZeroPageX, CPU::lda    ),
        /* 0xB6 */ (ZeroPageY, CPU::ldx    ),
        /* 0xB7 */ (ZeroPageY, CPU::lax    ),
        /* 0xB8 */ (Implied,   CPU::clv    ),
        /* 0xB9 */ (AbsoluteY, CPU::lda    ),
        /* 0xBA */ (Implied,   CPU::tsx    ),
        /* 0xBB */ (Implied,   CPU::unimpl ),
        /* 0xBC */ (AbsoluteX, CPU::ldy    ),
        /* 0xBD */ (AbsoluteX, CPU::lda    ),
        /* 0xBE */ (AbsoluteY, CPU::ldx    ),
        /* 0xBF */ (AbsoluteY, CPU::lax    ),
        /* 0xC0 */ (Immediate, CPU::cpy    ),
        /* 0xC1 */ (IndirectX, CPU::cmp    ),
        /* 0xC2 */ (Implied,   CPU::unimpl ),
        /* 0xC3 */ (Implied,   CPU::unimpl ),
        /* 0xC4 */ (ZeroPage,  CPU::cpy    ),
        /* 0xC5 */ (ZeroPage,  CPU::cmp    ),
        /* 0xC6 */ (ZeroPage,  CPU::dec    ),
        /* 0xC7 */ (Implied,   CPU::unimpl ),
        /* 0xC8 */ (Implied,   CPU::iny    ),
        /* 0xC9 */ (Immediate, CPU::cmp    ),
        /* 0xCA */ (Implied,   CPU::dex    ),
        /* 0xCB */ (Implied,   CPU::unimpl ),
        /* 0xCC */ (Absolute,  CPU::cpy    ),
        /* 0xCD */ (Absolute,  CPU::cmp    ),
        /* 0xCE */ (Absolute,  CPU::dec    ),
        /* 0xCF */ (Implied,   CPU::unimpl ),
        /* 0xD0 */ (Relative,  CPU::bne    ),
        /* 0xD1 */ (IndirectY, CPU::cmp    ),
        /* 0xD2 */ (Implied,   CPU::unimpl ),
        /* 0xD3 */ (Implied,   CPU::unimpl ),
        /* 0xD4 */ (ZeroPageX, CPU::nop    ),
        /* 0xD5 */ (ZeroPageX, CPU::cmp    ),
        /* 0xD6 */ (ZeroPageX, CPU::dec    ),
        /* 0xD7 */ (Implied,   CPU::unimpl ),
        /* 0xD8 */ (Implied,   CPU::cld    ),
        /* 0xD9 */ (AbsoluteY, CPU::cmp    ),
        /* 0xDA */ (Implied,   CPU::nop    ),
        /* 0xDB */ (Implied,   CPU::unimpl ),
        /* 0xDC */ (AbsoluteX, CPU::nop    ),
        /* 0xDD */ (AbsoluteX, CPU::cmp    ),
        /* 0xDE */ (AbsoluteX, CPU::dec    ),
        /* 0xDF */ (Implied,   CPU::unimpl ),
        /* 0xE0 */ (Immediate, CPU::cpx    ),
        /* 0xE1 */ (IndirectX, CPU::sbc    ),
        /* 0xE2 */ (Implied,   CPU::unimpl ),
        /* 0xE3 */ (Implied,   CPU::unimpl ),
        /* 0xE4 */ (ZeroPage,  CPU::cpx    ),
        /* 0xE5 */ (ZeroPage,  CPU::sbc    ),
        /* 0xE6 */ (ZeroPage,  CPU::inc    ),
        /* 0xE7 */ (Implied,   CPU::unimpl ),
        /* 0xE8 */ (Implied,   CPU::inx    ),
        /* 0xE9 */ (Immediate, CPU::sbc    ),
        /* 0xEA */ (Implied,   CPU::nop    ),
        /* 0xEB */ (Implied,   CPU::unimpl ),
        /* 0xEC */ (Absolute,  CPU::cpx    ),
        /* 0xED */ (Absolute,  CPU::sbc    ),
        /* 0xEE */ (Absolute,  CPU::inc    ),
        /* 0xEF */ (Implied,   CPU::unimpl ),
        /* 0xF0 */ (Relative,  CPU::beq    ),
        /* 0xF1 */ (IndirectY, CPU::sbc    ),
        /* 0xF2 */ (Implied,   CPU::unimpl ),
        /* 0xF3 */ (Implied,   CPU::unimpl ),
        /* 0xF4 */ (ZeroPageX, CPU::nop    ),
        /* 0xF5 */ (ZeroPageX, CPU::sbc    ),
        /* 0xF6 */ (ZeroPageX, CPU::inc    ),
        /* 0xF7 */ (Implied,   CPU::unimpl ),
        /* 0xF8 */ (Implied,   CPU::sed    ),
        /* 0xF9 */ (AbsoluteY, CPU::sbc    ),
        /* 0xFA */ (Implied,   CPU::nop    ),
        /* 0xFB */ (Implied,   CPU::unimpl ),
        /* 0xFC */ (AbsoluteX, CPU::nop    ),
        /* 0xFD */ (AbsoluteX, CPU::sbc    ),
        /* 0xFE */ (AbsoluteX, CPU::inc    ),
        /* 0xFF */ (Implied,   CPU::unimpl ),
    ];
}
