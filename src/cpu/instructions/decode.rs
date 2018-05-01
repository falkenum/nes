
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

// these are a few macros to help with the implementation of of the instruction
// decoding. The opcode is used with a const lookup table of closures that
// execute the instruction. I implemented instruction handling in this way
// in order to have constant-time decoding of instructions.

macro_rules! unimpl {
    () => {  &|_ : &mut CPU| panic!("called unimplemented instruction"); };
}

macro_rules! read_instr_arg {
    ( $obj:ident, immediate   ) => {{ let n = $obj.pc_getb()  ; Immediate(n)}};
    ( $obj:ident, absolute    ) => {{ let n = $obj.pc_getdb() ; Address(n)}};
    ( $obj:ident, absolute_x  ) => {{
        let n = $obj.pc_getdb();
        let n = $obj.absolute_x(n);
        Address(n)
    }};
    ( $obj:ident, absolute_y  ) => {{
        let n = $obj.pc_getdb();
        let n = $obj.absolute_y(n);
        Address(n)
    }};
    ( $obj:ident, zero_page  ) => {{
        let n = $obj.pc_getb();
        let n = $obj.zero_page(n);
        Address(n)
    }};
    ( $obj:ident, zero_page_x  ) => {{
        let n = $obj.pc_getb();
        let n = $obj.zero_page_x(n);
        Address(n)
    }};
    ( $obj:ident, zero_page_y  ) => {{
        let n = $obj.pc_getb();
        let n = $obj.zero_page_y(n);
        Address(n)
    }};
    ( $obj:ident, indirect  ) => {{
        let n = $obj.pc_getdb();
        let n = $obj.indirect(n);
        Address(n)
    }};
    ( $obj:ident, indirect_x  ) => {{
        let n = $obj.pc_getb();
        let n = $obj.indirect_x(n);
        Address(n)
    }};
    ( $obj:ident, indirect_y  ) => {{
        let n = $obj.pc_getb();
        let n = $obj.indirect_y(n);
        Address(n)
    }};
}

macro_rules! instr {
    ( $addr_mode:ident, $instr:ident ) =>
        {
            &|cpu : &mut CPU| {
                let n = read_instr_arg!(cpu, $addr_mode);
                cpu.$instr(n);
            }
        };
}

// refer to this page to see what each opcode does
// http://www.6502.org/tutorials/6502opcodes.html

const NUM_OPCODES : usize = 256;
pub const INSTR : [&Fn(&mut CPU); NUM_OPCODES] = [
    /* 0x00 */ unimpl!(),
    /* 0x01 */ instr!(indirect_x, ora),
    /* 0x02 */ unimpl!(),
    /* 0x03 */ unimpl!(),
    /* 0x04 */ unimpl!(),
    /* 0x05 */ instr!(zero_page, ora),
    /* 0x06 */ unimpl!(),
    /* 0x07 */ unimpl!(),
    /* 0x08 */ unimpl!(),
    /* 0x09 */ instr!(immediate, ora),
    /* 0x0A */ unimpl!(),
    /* 0x0B */ unimpl!(),
    /* 0x0C */ unimpl!(),
    /* 0x0D */ instr!(absolute, ora),
    /* 0x0E */ unimpl!(),
    /* 0x0F */ unimpl!(),
    /* 0x10 */ unimpl!(),
    /* 0x11 */ instr!(indirect_y, ora),
    /* 0x12 */ unimpl!(),
    /* 0x13 */ unimpl!(),
    /* 0x14 */ unimpl!(),
    /* 0x15 */ instr!(zero_page_x, ora),
    /* 0x16 */ unimpl!(),
    /* 0x17 */ unimpl!(),
    /* 0x18 */ unimpl!(),
    /* 0x19 */ instr!(absolute_y, ora),
    /* 0x1A */ unimpl!(),
    /* 0x1B */ unimpl!(),
    /* 0x1C */ unimpl!(),
    /* 0x1D */ instr!(absolute_x, ora),
    /* 0x1E */ unimpl!(),
    /* 0x1F */ unimpl!(),
    /* 0x20 */ unimpl!(),
    /* 0x21 */ instr!(indirect_x, and),
    /* 0x22 */ unimpl!(),
    /* 0x23 */ unimpl!(),
    /* 0x24 */ unimpl!(),
    /* 0x25 */ instr!(zero_page, and),
    /* 0x26 */ unimpl!(),
    /* 0x27 */ unimpl!(),
    /* 0x28 */ unimpl!(),
    /* 0x29 */ instr!(immediate, and),
    /* 0x2A */ unimpl!(),
    /* 0x2B */ unimpl!(),
    /* 0x2C */ unimpl!(),
    /* 0x2D */ instr!(absolute, and),
    /* 0x2E */ unimpl!(),
    /* 0x2F */ unimpl!(),
    /* 0x30 */ unimpl!(),
    /* 0x31 */ instr!(indirect_y, and),
    /* 0x32 */ unimpl!(),
    /* 0x33 */ unimpl!(),
    /* 0x34 */ unimpl!(),
    /* 0x35 */ instr!(zero_page_x, and),
    /* 0x36 */ unimpl!(),
    /* 0x37 */ unimpl!(),
    /* 0x38 */ unimpl!(),
    /* 0x39 */ instr!(absolute_y, and),
    /* 0x3A */ unimpl!(),
    /* 0x3B */ unimpl!(),
    /* 0x3C */ unimpl!(),
    /* 0x3D */ instr!(absolute_x, and),
    /* 0x3E */ unimpl!(),
    /* 0x3F */ unimpl!(),
    /* 0x40 */ unimpl!(),
    /* 0x41 */ unimpl!(),
    /* 0x42 */ unimpl!(),
    /* 0x43 */ unimpl!(),
    /* 0x44 */ unimpl!(),
    /* 0x45 */ unimpl!(),
    /* 0x46 */ unimpl!(),
    /* 0x47 */ unimpl!(),
    /* 0x48 */ unimpl!(),
    /* 0x49 */ unimpl!(),
    /* 0x4A */ unimpl!(),
    /* 0x4B */ unimpl!(),
    /* 0x4C */ unimpl!(),
    /* 0x4D */ unimpl!(),
    /* 0x4E */ unimpl!(),
    /* 0x4F */ unimpl!(),
    /* 0x50 */ unimpl!(),
    /* 0x51 */ unimpl!(),
    /* 0x52 */ unimpl!(),
    /* 0x53 */ unimpl!(),
    /* 0x54 */ unimpl!(),
    /* 0x55 */ unimpl!(),
    /* 0x56 */ unimpl!(),
    /* 0x57 */ unimpl!(),
    /* 0x58 */ unimpl!(),
    /* 0x59 */ unimpl!(),
    /* 0x5A */ unimpl!(),
    /* 0x5B */ unimpl!(),
    /* 0x5C */ unimpl!(),
    /* 0x5D */ unimpl!(),
    /* 0x5E */ unimpl!(),
    /* 0x5F */ unimpl!(),
    /* 0x60 */ unimpl!(),
    /* 0x61 */ instr!(indirect_x, adc),
    /* 0x62 */ unimpl!(),
    /* 0x63 */ unimpl!(),
    /* 0x64 */ unimpl!(),
    /* 0x65 */ instr!(zero_page, adc),
    /* 0x66 */ unimpl!(),
    /* 0x67 */ unimpl!(),
    /* 0x68 */ unimpl!(),
    /* 0x69 */ instr!(immediate, adc),
    /* 0x6A */ unimpl!(),
    /* 0x6B */ unimpl!(),
    /* 0x6C */ unimpl!(),
    /* 0x6D */ instr!(absolute, adc),
    /* 0x6E */ unimpl!(),
    /* 0x6F */ unimpl!(),
    /* 0x70 */ unimpl!(),
    /* 0x71 */ instr!(zero_page_y, adc),
    /* 0x72 */ unimpl!(),
    /* 0x73 */ unimpl!(),
    /* 0x74 */ unimpl!(),
    /* 0x75 */ instr!(zero_page_x, adc),
    /* 0x76 */ unimpl!(),
    /* 0x77 */ unimpl!(),
    /* 0x78 */ unimpl!(),
    /* 0x79 */ instr!(absolute_y, adc),
    /* 0x7A */ unimpl!(),
    /* 0x7B */ unimpl!(),
    /* 0x7C */ unimpl!(),
    /* 0x7D */ instr!(absolute_x, adc),
    /* 0x7E */ unimpl!(),
    /* 0x7F */ unimpl!(),
    /* 0x80 */ unimpl!(),
    /* 0x81 */ instr!(indirect_x, sta),
    /* 0x82 */ unimpl!(),
    /* 0x83 */ unimpl!(),
    /* 0x84 */ instr!(zero_page, sty),
    /* 0x85 */ instr!(zero_page, sta),
    /* 0x86 */ instr!(zero_page, stx),
    /* 0x87 */ unimpl!(),
    /* 0x88 */ unimpl!(),
    /* 0x89 */ unimpl!(),
    /* 0x8A */ unimpl!(),
    /* 0x8B */ unimpl!(),
    /* 0x8C */ instr!(absolute, sty),
    /* 0x8D */ instr!(absolute, sta),
    /* 0x8E */ instr!(absolute, stx),
    /* 0x8F */ unimpl!(),
    /* 0x90 */ unimpl!(),
    /* 0x91 */ instr!(indirect_y, sta),
    /* 0x92 */ unimpl!(),
    /* 0x93 */ unimpl!(),
    /* 0x94 */ instr!(zero_page_x, sty),
    /* 0x95 */ instr!(zero_page_x, sta),
    /* 0x96 */ instr!(zero_page_y, stx),
    /* 0x97 */ unimpl!(),
    /* 0x98 */ unimpl!(),
    /* 0x99 */ instr!(absolute_y, sta),
    /* 0x9A */ unimpl!(),
    /* 0x9B */ unimpl!(),
    /* 0x9C */ unimpl!(),
    /* 0x9D */ instr!(absolute_x, sta),
    /* 0x9E */ unimpl!(),
    /* 0x9F */ unimpl!(),
    /* 0xA0 */ instr!(immediate, ldy),
    /* 0xA1 */ instr!(indirect_x, lda),
    /* 0xA2 */ instr!(immediate, ldx),
    /* 0xA3 */ unimpl!(),
    /* 0xA4 */ instr!(zero_page, ldy),
    /* 0xA5 */ instr!(zero_page, lda),
    /* 0xA6 */ instr!(zero_page, ldx),
    /* 0xA7 */ unimpl!(),
    /* 0xA8 */ unimpl!(),
    /* 0xA9 */ instr!(immediate, lda),
    /* 0xAA */ unimpl!(),
    /* 0xAB */ unimpl!(),
    /* 0xAC */ instr!(absolute, ldy),
    /* 0xAD */ instr!(absolute, lda),
    /* 0xAE */ instr!(absolute, ldx),
    /* 0xAF */ unimpl!(),
    /* 0xB0 */ unimpl!(),
    /* 0xB1 */ instr!(indirect_y, lda),
    /* 0xB2 */ unimpl!(),
    /* 0xB3 */ unimpl!(),
    /* 0xB4 */ instr!(zero_page_x, ldy),
    /* 0xB5 */ instr!(zero_page_x, lda),
    /* 0xB6 */ instr!(zero_page_y, ldx),
    /* 0xB7 */ unimpl!(),
    /* 0xB8 */ unimpl!(),
    /* 0xB9 */ instr!(absolute_y, lda),
    /* 0xBA */ unimpl!(),
    /* 0xBB */ unimpl!(),
    /* 0xBC */ instr!(absolute_x, ldy),
    /* 0xBD */ instr!(absolute_x, lda),
    /* 0xBE */ instr!(absolute_y, ldx),
    /* 0xBF */ unimpl!(),
    /* 0xC0 */ unimpl!(),
    /* 0xC1 */ unimpl!(),
    /* 0xC2 */ unimpl!(),
    /* 0xC3 */ unimpl!(),
    /* 0xC4 */ unimpl!(),
    /* 0xC5 */ unimpl!(),
    /* 0xC6 */ unimpl!(),
    /* 0xC7 */ unimpl!(),
    /* 0xC8 */ unimpl!(),
    /* 0xC9 */ unimpl!(),
    /* 0xCA */ unimpl!(),
    /* 0xCB */ unimpl!(),
    /* 0xCC */ unimpl!(),
    /* 0xCD */ unimpl!(),
    /* 0xCE */ unimpl!(),
    /* 0xCF */ unimpl!(),
    /* 0xD0 */ unimpl!(),
    /* 0xD1 */ unimpl!(),
    /* 0xD2 */ unimpl!(),
    /* 0xD3 */ unimpl!(),
    /* 0xD4 */ unimpl!(),
    /* 0xD5 */ unimpl!(),
    /* 0xD6 */ unimpl!(),
    /* 0xD7 */ unimpl!(),
    /* 0xD8 */ unimpl!(),
    /* 0xD9 */ unimpl!(),
    /* 0xDA */ unimpl!(),
    /* 0xDB */ unimpl!(),
    /* 0xDC */ unimpl!(),
    /* 0xDD */ unimpl!(),
    /* 0xDE */ unimpl!(),
    /* 0xDF */ unimpl!(),
    /* 0xE0 */ unimpl!(),
    /* 0xE1 */ instr!(indirect_x, sbc),
    /* 0xE2 */ unimpl!(),
    /* 0xE3 */ unimpl!(),
    /* 0xE4 */ unimpl!(),
    /* 0xE5 */ instr!(zero_page, sbc),
    /* 0xE6 */ unimpl!(),
    /* 0xE7 */ unimpl!(),
    /* 0xE8 */ unimpl!(),
    /* 0xE9 */ instr!(immediate, sbc),
    /* 0xEA */ unimpl!(),
    /* 0xEB */ unimpl!(),
    /* 0xEC */ unimpl!(),
    /* 0xED */ instr!(absolute, sbc),
    /* 0xEE */ unimpl!(),
    /* 0xEF */ unimpl!(),
    /* 0xF0 */ unimpl!(),
    /* 0xF1 */ instr!(indirect_y, sbc),
    /* 0xF2 */ unimpl!(),
    /* 0xF3 */ unimpl!(),
    /* 0xF4 */ unimpl!(),
    /* 0xF5 */ instr!(zero_page_x, sbc),
    /* 0xF6 */ unimpl!(),
    /* 0xF7 */ unimpl!(),
    /* 0xF8 */ unimpl!(),
    /* 0xF9 */ instr!(absolute_y, sbc),
    /* 0xFA */ unimpl!(),
    /* 0xFB */ unimpl!(),
    /* 0xFC */ unimpl!(),
    /* 0xFD */ instr!(absolute_x, sbc),
    /* 0xFE */ unimpl!(),
    /* 0xFF */ unimpl!(),
];
