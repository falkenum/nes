use super::InstrArg::{ Implied, Immediate, Address };
use super::CPU;

// addressing modes
impl CPU {
    fn relative    (&self, val : u8)  -> u16 {
        // sign extend val and add to pc
        ((val as u16) | (0xFF00 * ((val >> 7) as u16))).wrapping_add(self.pc)
    }
    fn absolute_x  (&self, val : u16) -> u16 { val.wrapping_add(self.x as u16) }
    fn absolute_y  (&self, val : u16) -> u16 { val.wrapping_add(self.y as u16) }
    fn zero_page   (&self, val : u8)  -> u16 { val as u16 }
    fn zero_page_x (&self, val : u8)  -> u16 { val.wrapping_add(self.x) as u16 }
    fn zero_page_y (&self, val : u8)  -> u16 { val.wrapping_add(self.y) as u16 }
    fn indirect_x  (&self, val : u8)  -> u16 {
        let a = val.wrapping_add(self.x);
        self.indirect(a as u16)
    }
    fn indirect_y  (&self, val : u8)  -> u16 {
        self.indirect(val as u16).wrapping_add(self.y as u16)
    }
    fn indirect    (&self, val : u16) -> u16 {
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
    ( $obj:ident, implied  ) => {{
        Implied
    }};
    ( $obj:ident, relative  ) => {{
        let n = $obj.pc_getb();
        let n = $obj.relative(n);
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

const NUM_OPCODES : usize = 256;
pub const INSTR : [&'static Fn(&mut CPU); NUM_OPCODES] = [
    /* 0x00 */ instr!(implied, brk),
    /* 0x01 */ instr!(indirect_x, ora),
    /* 0x02 */ unimpl!(),
    /* 0x03 */ unimpl!(),
    /* 0x04 */ instr!(zero_page, nop),
    /* 0x05 */ instr!(zero_page, ora),
    /* 0x06 */ instr!(zero_page, asl),
    /* 0x07 */ unimpl!(),
    /* 0x08 */ instr!(implied, php),
    /* 0x09 */ instr!(immediate, ora),
    /* 0x0A */ instr!(implied, asl),
    /* 0x0B */ unimpl!(),
    /* 0x0C */ instr!(absolute, nop),
    /* 0x0D */ instr!(absolute, ora),
    /* 0x0E */ instr!(absolute, asl),
    /* 0x0F */ unimpl!(),
    /* 0x10 */ instr!(relative, bpl),
    /* 0x11 */ instr!(indirect_y, ora),
    /* 0x12 */ unimpl!(),
    /* 0x13 */ unimpl!(),
    /* 0x14 */ instr!(zero_page_x, nop),
    /* 0x15 */ instr!(zero_page_x, ora),
    /* 0x16 */ instr!(zero_page_x, asl),
    /* 0x17 */ unimpl!(),
    /* 0x18 */ instr!(implied, clc),
    /* 0x19 */ instr!(absolute_y, ora),
    /* 0x1A */ instr!(implied, nop),
    /* 0x1B */ unimpl!(),
    /* 0x1C */ instr!(absolute_x, nop),
    /* 0x1D */ instr!(absolute_x, ora),
    /* 0x1E */ instr!(absolute_x, asl),
    /* 0x1F */ unimpl!(),
    /* 0x20 */ instr!(absolute, jsr),
    /* 0x21 */ instr!(indirect_x, and),
    /* 0x22 */ unimpl!(),
    /* 0x23 */ unimpl!(),
    /* 0x24 */ instr!(zero_page, bit),
    /* 0x25 */ instr!(zero_page, and),
    /* 0x26 */ instr!(zero_page, rol),
    /* 0x27 */ unimpl!(),
    /* 0x28 */ instr!(implied, plp),
    /* 0x29 */ instr!(immediate, and),
    /* 0x2A */ instr!(implied, rol),
    /* 0x2B */ unimpl!(),
    /* 0x2C */ instr!(absolute, bit),
    /* 0x2D */ instr!(absolute, and),
    /* 0x2E */ instr!(absolute, rol),
    /* 0x2F */ unimpl!(),
    /* 0x30 */ instr!(relative, bmi),
    /* 0x31 */ instr!(indirect_y, and),
    /* 0x32 */ unimpl!(),
    /* 0x33 */ unimpl!(),
    /* 0x34 */ instr!(zero_page_x, nop),
    /* 0x35 */ instr!(zero_page_x, and),
    /* 0x36 */ instr!(zero_page_x, rol),
    /* 0x37 */ unimpl!(),
    /* 0x38 */ instr!(implied, sec),
    /* 0x39 */ instr!(absolute_y, and),
    /* 0x3A */ instr!(implied, nop),
    /* 0x3B */ unimpl!(),
    /* 0x3C */ instr!(absolute_x, nop),
    /* 0x3D */ instr!(absolute_x, and),
    /* 0x3E */ instr!(absolute_x, rol),
    /* 0x3F */ unimpl!(),
    /* 0x40 */ instr!(implied, rti),
    /* 0x41 */ instr!(indirect_x, eor),
    /* 0x42 */ unimpl!(),
    /* 0x43 */ unimpl!(),
    /* 0x44 */ instr!(zero_page, nop),
    /* 0x45 */ instr!(zero_page, eor),
    /* 0x46 */ instr!(zero_page, lsr),
    /* 0x47 */ unimpl!(),
    /* 0x48 */ instr!(implied, pha),
    /* 0x49 */ instr!(immediate, eor),
    /* 0x4A */ instr!(implied, lsr),
    /* 0x4B */ unimpl!(),
    /* 0x4C */ instr!(absolute, jmp),
    /* 0x4D */ instr!(absolute, eor),
    /* 0x4E */ instr!(absolute, lsr),
    /* 0x4F */ unimpl!(),
    /* 0x50 */ instr!(relative, bvc),
    /* 0x51 */ instr!(indirect_y, eor),
    /* 0x52 */ unimpl!(),
    /* 0x53 */ unimpl!(),
    /* 0x54 */ instr!(zero_page_x, nop),
    /* 0x55 */ instr!(zero_page_x, eor),
    /* 0x56 */ instr!(zero_page_x, lsr),
    /* 0x57 */ unimpl!(),
    /* 0x58 */ instr!(implied, cli),
    /* 0x59 */ instr!(absolute_y, eor),
    /* 0x5A */ instr!(implied, nop),
    /* 0x5B */ unimpl!(),
    /* 0x5C */ instr!(absolute_x, nop),
    /* 0x5D */ instr!(absolute_x, eor),
    /* 0x5E */ instr!(absolute_x, lsr),
    /* 0x5F */ unimpl!(),
    /* 0x60 */ instr!(implied, rts),
    /* 0x61 */ instr!(indirect_x, adc),
    /* 0x62 */ unimpl!(),
    /* 0x63 */ unimpl!(),
    /* 0x64 */ instr!(zero_page, nop),
    /* 0x65 */ instr!(zero_page, adc),
    /* 0x66 */ instr!(zero_page, ror),
    /* 0x67 */ unimpl!(),
    /* 0x68 */ instr!(implied, pla),
    /* 0x69 */ instr!(immediate, adc),
    /* 0x6A */ instr!(implied, ror),
    /* 0x6B */ unimpl!(),
    /* 0x6C */ instr!(indirect, jmp),
    /* 0x6D */ instr!(absolute, adc),
    /* 0x6E */ instr!(absolute, ror),
    /* 0x6F */ unimpl!(),
    /* 0x70 */ instr!(relative, bvs),
    /* 0x71 */ instr!(zero_page_y, adc),
    /* 0x72 */ unimpl!(),
    /* 0x73 */ unimpl!(),
    /* 0x74 */ instr!(zero_page_x, nop),
    /* 0x75 */ instr!(zero_page_x, adc),
    /* 0x76 */ instr!(zero_page_x, ror),
    /* 0x77 */ unimpl!(),
    /* 0x78 */ instr!(implied, sei),
    /* 0x79 */ instr!(absolute_y, adc),
    /* 0x7A */ instr!(implied, nop),
    /* 0x7B */ unimpl!(),
    /* 0x7C */ instr!(absolute_x, nop),
    /* 0x7D */ instr!(absolute_x, adc),
    /* 0x7E */ instr!(absolute_x, ror),
    /* 0x7F */ unimpl!(),
    /* 0x80 */ instr!(immediate, nop),
    /* 0x81 */ instr!(indirect_x, sta),
    /* 0x82 */ unimpl!(),
    /* 0x83 */ unimpl!(),
    /* 0x84 */ instr!(zero_page, sty),
    /* 0x85 */ instr!(zero_page, sta),
    /* 0x86 */ instr!(zero_page, stx),
    /* 0x87 */ unimpl!(),
    /* 0x88 */ instr!(implied, dey),
    /* 0x89 */ unimpl!(),
    /* 0x8A */ instr!(implied, txa),
    /* 0x8B */ unimpl!(),
    /* 0x8C */ instr!(absolute, sty),
    /* 0x8D */ instr!(absolute, sta),
    /* 0x8E */ instr!(absolute, stx),
    /* 0x8F */ unimpl!(),
    /* 0x90 */ instr!(relative, bcc),
    /* 0x91 */ instr!(indirect_y, sta),
    /* 0x92 */ unimpl!(),
    /* 0x93 */ unimpl!(),
    /* 0x94 */ instr!(zero_page_x, sty),
    /* 0x95 */ instr!(zero_page_x, sta),
    /* 0x96 */ instr!(zero_page_y, stx),
    /* 0x97 */ unimpl!(),
    /* 0x98 */ instr!(implied, tya),
    /* 0x99 */ instr!(absolute_y, sta),
    /* 0x9A */ instr!(implied, txs),
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
    /* 0xA8 */ instr!(implied, tay),
    /* 0xA9 */ instr!(immediate, lda),
    /* 0xAA */ instr!(implied, tax),
    /* 0xAB */ unimpl!(),
    /* 0xAC */ instr!(absolute, ldy),
    /* 0xAD */ instr!(absolute, lda),
    /* 0xAE */ instr!(absolute, ldx),
    /* 0xAF */ unimpl!(),
    /* 0xB0 */ instr!(relative,  bcs),
    /* 0xB1 */ instr!(indirect_y, lda),
    /* 0xB2 */ unimpl!(),
    /* 0xB3 */ unimpl!(),
    /* 0xB4 */ instr!(zero_page_x, ldy),
    /* 0xB5 */ instr!(zero_page_x, lda),
    /* 0xB6 */ instr!(zero_page_y, ldx),
    /* 0xB7 */ unimpl!(),
    /* 0xB8 */ instr!(implied, clv),
    /* 0xB9 */ instr!(absolute_y, lda),
    /* 0xBA */ instr!(implied, tsx),
    /* 0xBB */ unimpl!(),
    /* 0xBC */ instr!(absolute_x, ldy),
    /* 0xBD */ instr!(absolute_x, lda),
    /* 0xBE */ instr!(absolute_y, ldx),
    /* 0xBF */ unimpl!(),
    /* 0xC0 */ instr!(immediate, cpy),
    /* 0xC1 */ instr!(indirect_x, cmp),
    /* 0xC2 */ unimpl!(),
    /* 0xC3 */ unimpl!(),
    /* 0xC4 */ instr!(zero_page, cpy),
    /* 0xC5 */ instr!(zero_page, cmp),
    /* 0xC6 */ instr!(zero_page, dec),
    /* 0xC7 */ unimpl!(),
    /* 0xC8 */ instr!(implied, iny),
    /* 0xC9 */ instr!(immediate, cmp),
    /* 0xCA */ instr!(implied, dex),
    /* 0xCB */ unimpl!(),
    /* 0xCC */ instr!(absolute, cpy),
    /* 0xCD */ instr!(absolute, cmp),
    /* 0xCE */ instr!(absolute, dec),
    /* 0xCF */ unimpl!(),
    /* 0xD0 */ instr!(relative, bne),
    /* 0xD1 */ instr!(indirect_y, cmp),
    /* 0xD2 */ unimpl!(),
    /* 0xD3 */ unimpl!(),
    /* 0xD4 */ instr!(zero_page_x, nop),
    /* 0xD5 */ instr!(zero_page_x, cmp),
    /* 0xD6 */ instr!(zero_page_x, dec),
    /* 0xD7 */ unimpl!(),
    /* 0xD8 */ instr!(implied, cld),
    /* 0xD9 */ instr!(absolute_y, cmp),
    /* 0xDA */ instr!(implied, nop),
    /* 0xDB */ unimpl!(),
    /* 0xDC */ instr!(absolute_x, nop),
    /* 0xDD */ instr!(absolute_x, cmp),
    /* 0xDE */ instr!(absolute_x, dec),
    /* 0xDF */ unimpl!(),
    /* 0xE0 */ instr!(immediate, cpx),
    /* 0xE1 */ instr!(indirect_x, sbc),
    /* 0xE2 */ unimpl!(),
    /* 0xE3 */ unimpl!(),
    /* 0xE4 */ instr!(zero_page, cpx),
    /* 0xE5 */ instr!(zero_page, sbc),
    /* 0xE6 */ instr!(zero_page, inc),
    /* 0xE7 */ unimpl!(),
    /* 0xE8 */ instr!(implied, inx),
    /* 0xE9 */ instr!(immediate, sbc),
    /* 0xEA */ instr!(implied, nop),
    /* 0xEB */ unimpl!(),
    /* 0xEC */ instr!(absolute, cpx),
    /* 0xED */ instr!(absolute, sbc),
    /* 0xEE */ instr!(absolute, inc),
    /* 0xEF */ unimpl!(),
    /* 0xF0 */ instr!(relative, beq),
    /* 0xF1 */ instr!(indirect_y, sbc),
    /* 0xF2 */ unimpl!(),
    /* 0xF3 */ unimpl!(),
    /* 0xF4 */ instr!(zero_page_x, nop),
    /* 0xF5 */ instr!(zero_page_x, sbc),
    /* 0xF6 */ instr!(zero_page_x, inc),
    /* 0xF7 */ unimpl!(),
    /* 0xF8 */ instr!(implied, sed),
    /* 0xF9 */ instr!(absolute_y, sbc),
    /* 0xFA */ instr!(implied, nop),
    /* 0xFB */ unimpl!(),
    /* 0xFC */ instr!(absolute_x, nop),
    /* 0xFD */ instr!(absolute_x, sbc),
    /* 0xFE */ instr!(absolute_x, inc),
    /* 0xFF */ unimpl!(),
];

#[cfg(test)]
mod tests {
    #[test]
    fn addr_modes() {

        let mut c = super::CPU::new();

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

        c.pc = 0x8000;
        assert_eq!(c.relative(0x50), 0x8050);
        assert_eq!(c.relative(0xFF), 0x7FFF);
        c.pc = 0x8080;
        assert_eq!(c.relative(0x80), 0x8000);
    }
}
