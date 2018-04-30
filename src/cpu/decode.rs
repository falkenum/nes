
use super::CPU;
#[allow(unused_imports)]
use super::InstrArg::{ Implied, Immediate, Address };

// addressing modes
impl CPU {
    pub fn absolute_x  (&self, val : u16) -> u16 { val.wrapping_add(self.x as u16) }
    pub fn absolute_y  (&self, val : u16) -> u16 { val.wrapping_add(self.y as u16) }
    pub fn zero_page   (&self, val : u8)  -> u16 { val as u16 }
    pub fn zero_page_x (&self, val : u8)  -> u16 { val.wrapping_add(self.x) as u16 }
    pub fn zero_page_y (&self, val : u8)  -> u16 { val.wrapping_add(self.y) as u16 }
    pub fn indirect_x  (&self, val : u8)  -> u16 {
        let a = val.wrapping_add(self.x);
        self.indirect(a as u16)
    }
    pub fn indirect_y  (&self, val : u8)  -> u16 {
        self.indirect(val as u16) + self.y as u16
    }
    pub fn indirect    (&self, val : u16) -> u16 {
        let addr_low = val as u8;
        let addr_high = val & 0xFF00;
        let i = val as usize;
        // We need to add 1 to the lower 8 bits separately in order to
        // accurately simulate how the 6502 handles page boundries -- A page is
        // 0xFF bytes.
        // If mem[0] = 1 and mem[FF] = 2, then JMP ($00FF) should evaluate
        // to JMP $0201
        let j = (addr_low.wrapping_add(1) as u16 + addr_high) as usize;
        self.mem[i] as u16 + ((self.mem[j] as u16) << 8)
    }
}

fn unimplemented(_ : &mut CPU) {
    panic!("called unimplemented instruction");
}

// refer to this page to see what each opcode does
// http://www.6502.org/tutorials/6502opcodes.html#STA

const NUM_OPCODES : usize = 256;
pub const INSTR : [&Fn(&mut CPU); NUM_OPCODES] = [
    // TODO delete leading underscores once finished
    /* 0x00 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x01 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x02 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x03 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x04 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x05 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x06 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x07 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x08 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x09 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x0A */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x0B */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x0C */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x0D */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x0E */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x0F */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x10 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x11 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x12 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x13 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x14 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x15 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x16 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x17 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x18 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x19 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x1A */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x1B */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x1C */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x1D */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x1E */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x1F */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x20 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x21 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x22 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x23 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x24 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x25 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x26 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x27 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x28 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x29 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x2A */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x2B */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x2C */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x2D */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x2E */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x2F */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x30 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x31 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x32 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x33 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x34 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x35 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x36 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x37 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x38 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x39 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x3A */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x3B */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x3C */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x3D */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x3E */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x3F */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x40 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x41 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x42 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x43 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x44 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x45 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x46 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x47 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x48 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x49 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x4A */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x4B */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x4C */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x4D */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x4E */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x4F */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x50 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x51 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x52 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x53 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x54 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x55 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x56 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x57 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x58 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x59 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x5A */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x5B */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x5C */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x5D */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x5E */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x5F */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x60 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x61 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x62 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x63 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x64 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x65 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x66 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x67 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x68 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x69 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x6A */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x6B */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x6C */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x6D */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x6E */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x6F */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x70 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x71 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x72 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x73 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x74 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x75 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x76 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x77 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x78 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x79 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x7A */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x7B */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x7C */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x7D */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x7E */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x7F */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x80 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x81 */ &|_cpu : &mut CPU| {
        let n = _cpu.pc_getb();
        let n = _cpu.indirect_x(n);
        _cpu.sta(Address(n));
    },
    /* 0x82 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x83 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x84 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x85 */ &|_cpu : &mut CPU| {
        let n = _cpu.pc_getb();
        let n = _cpu.zero_page(n);
        _cpu.sta(Address(n));
    },
    /* 0x86 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x87 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x88 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x89 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x8A */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x8B */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x8C */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x8D */ &|_cpu : &mut CPU| {
        let n = _cpu.pc_getdb();
        _cpu.sta(Address(n));
    },
    /* 0x8E */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x8F */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x90 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x91 */ &|_cpu : &mut CPU| {
        let n = _cpu.pc_getb();
        let n = _cpu.indirect_y(n);
        _cpu.sta(Address(n));
    },
    /* 0x92 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x93 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x94 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x95 */ &|_cpu : &mut CPU| {
        let n = _cpu.pc_getb();
        let n = _cpu.zero_page_x(n);
        _cpu.sta(Address(n));
    },
    /* 0x96 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x97 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x98 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x99 */ &|_cpu : &mut CPU| {
        let n = _cpu.pc_getdb();
        let n = _cpu.absolute_y(n);
        _cpu.sta(Address(n));
    },
    /* 0x9A */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x9B */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x9C */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x9D */ &|_cpu : &mut CPU| {
        let n = _cpu.pc_getdb();
        let n = _cpu.absolute_x(n);
        _cpu.sta(Address(n));
    },
    /* 0x9E */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0x9F */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xA0 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xA1 */ &|_cpu : &mut CPU| {
        let n = _cpu.pc_getb();
        let n = _cpu.indirect_x(n);
        _cpu.lda(Address(n));
    },
    /* 0xA2 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xA3 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xA4 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xA5 */ &|_cpu : &mut CPU| {
        let n = _cpu.pc_getb();
        let n = _cpu.zero_page(n);
        _cpu.lda(Address(n));
    },
    /* 0xA6 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xA7 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xA8 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xA9 */ &|_cpu : &mut CPU| {
        let n = _cpu.pc_getb();
        _cpu.lda(Immediate(n));
    },
    /* 0xAA */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xAB */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xAC */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xAD */ &|_cpu : &mut CPU| {
        let n = _cpu.pc_getdb();
        _cpu.lda(Address(n));
    },
    /* 0xAE */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xAF */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xB0 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xB1 */ &|_cpu : &mut CPU| {
        let n = _cpu.pc_getb();
        let n = _cpu.indirect_y(n);
        _cpu.lda(Address(n));
    },
    /* 0xB2 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xB3 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xB4 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xB5 */ &|_cpu : &mut CPU| {
        let n = _cpu.pc_getb();
        let n = _cpu.zero_page_x(n);
        _cpu.lda(Address(n));
    },
    /* 0xB6 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xB7 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xB8 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xB9 */ &|_cpu : &mut CPU| {
        let n = _cpu.pc_getdb();
        let n = _cpu.absolute_y(n);
        _cpu.lda(Address(n));
    },
    /* 0xBA */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xBB */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xBC */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xBD */ &|_cpu : &mut CPU| {
        let n = _cpu.pc_getdb();
        let n = _cpu.absolute_x(n);
        _cpu.lda(Address(n));
    },
    /* 0xBE */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xBF */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xC0 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xC1 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xC2 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xC3 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xC4 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xC5 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xC6 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xC7 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xC8 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xC9 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xCA */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xCB */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xCC */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xCD */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xCE */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xCF */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xD0 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xD1 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xD2 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xD3 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xD4 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xD5 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xD6 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xD7 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xD8 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xD9 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xDA */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xDB */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xDC */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xDD */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xDE */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xDF */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xE0 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xE1 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xE2 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xE3 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xE4 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xE5 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xE6 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xE7 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xE8 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xE9 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xEA */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xEB */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xEC */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xED */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xEE */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xEF */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xF0 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xF1 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xF2 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xF3 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xF4 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xF5 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xF6 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xF7 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xF8 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xF9 */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xFA */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xFB */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xFC */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xFD */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xFE */ &|_cpu : &mut CPU| unimplemented(_cpu),
    /* 0xFF */ &|_cpu : &mut CPU| unimplemented(_cpu),
];
