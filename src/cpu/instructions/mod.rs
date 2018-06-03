use super::{ CPU, Memory, split_bytes, concat_bytes };
#[cfg(test)]
mod tests;
pub mod decode;

// turns out that I didn't need to implement BCD because the
// NES version of the 6502 (the 2A03) has it disabled, but here it is
fn from_bcd(x : u8) -> u8 { (x & 0x0F) + ((x & 0xF0) >> 4) * 10 }
fn to_bcd(x : u8) -> u8 { ((x / 10) << 4) + (x % 10) }

// describes the possible types of arguments for instructions
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InstrArg {
    Implied,
    Immediate(u8),
    Address(u16),
}

impl CPU {
    fn unwrap_imm_or_memval(&self, arg : InstrArg) -> u8 {
        match arg {
            InstrArg::Immediate(imm) => imm,
            InstrArg::Address(addr)  => self.mem.loadb(addr),
            _                        => panic!("illegal instruction"),
        }
    }

    fn unwrap_addr(&self, arg : InstrArg) -> u16 {
        match arg {
            InstrArg::Address(addr) => addr,
            _                       => panic!("illegal instruction"),
        }
    }

    fn unwrap_implied(&self, arg : InstrArg) {
        match arg {
            InstrArg::Implied => (),
            _                 => panic!("illegal instruction"),
        };
    }

    fn set_compare_flags(&mut self, reg : u8, val : u8) {
        let result = reg.wrapping_sub(val);
        self.set_n(result);
        self.set_z(result);
        self.flags.c = val <= reg;
    }

    fn rti(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);

        let status = self.pop();
        self.flags = super::CPUFlags::from_byte(status);

        let ret_low = self.pop();
        let ret_high = self.pop();
        self.pc = concat_bytes(ret_high, ret_low);
    }

    // unofficial opcodes
    fn lax(&mut self, arg : InstrArg) {
        self.lda(arg);
        self.ldx(arg);
    }
    // end unofficial

    fn brk(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);

        let (ret_high, ret_low) = split_bytes(self.pc + 1);
        self.push(ret_high);
        self.push(ret_low);

        let dest_high = self.mem.loadb(0xFFFF);
        let dest_low = self.mem.loadb(0xFFFE);
        self.pc = concat_bytes(dest_high, dest_low);

        let status = self.flags.to_byte() | (1 << 4); // set b flag
        self.push(status);

        self.flags.i = true;
    }

    fn rts(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);

        let ret_low = self.pop();
        let ret_high = self.pop();
        let ret_addr = concat_bytes(ret_high, ret_low) + 1;

        self.pc = ret_addr;
    }

    fn jsr(&mut self, arg : InstrArg) {
        let dest = match arg {
            InstrArg::Address(addr) => addr,
            _                       => panic!("illegal instruction"),
        };

        // store pointer to addr of jsr + 2 (addr low of jsr argument).
        // subtracting 1 because the pc currently points to byte after this
        // instruction
        let ret_ptr : u16 = self.pc - 1;
        let ret_low : u8 = ret_ptr as u8;
        let ret_high : u8 = (ret_ptr >> 8) as u8;
        self.push(ret_high);
        self.push(ret_low);

        self.pc = dest;
    }

    fn bit(&mut self, arg : InstrArg) {
        let val = match arg {
            InstrArg::Address(addr) => self.mem.loadb(addr),
            _                       => panic!("illegal instruction"),
        };

        self.flags.n = (val & 0x80) != 0;
        self.flags.v = (val & 0x40) != 0;
        self.flags.z = (val & self.a) == 0;
    }

    fn jmp(&mut self, arg : InstrArg) {
        self.pc = match arg {
                InstrArg::Address(addr) => addr,
                _                       => panic!("illegal instruction"),
        };
    }

    fn bpl(&mut self, arg : InstrArg) {
        if !self.flags.n { self.jmp(arg) };
    }

    fn bmi(&mut self, arg : InstrArg) {
        if self.flags.n { self.jmp(arg) };
    }

    fn bvc(&mut self, arg : InstrArg) {
        if !self.flags.v { self.jmp(arg) };
    }

    fn bvs(&mut self, arg : InstrArg) {
        if self.flags.v { self.jmp(arg) };
    }

    fn bcc(&mut self, arg : InstrArg) {
        if !self.flags.c { self.jmp(arg) };
    }

    fn bcs(&mut self, arg : InstrArg) {
        if self.flags.c { self.jmp(arg) };
    }

    fn bne(&mut self, arg : InstrArg) {
        if !self.flags.z { self.jmp(arg) };
    }

    fn beq(&mut self, arg : InstrArg) {
        if self.flags.z { self.jmp(arg) };
    }

    fn nop(&mut self, _arg : InstrArg) {}

    fn clv(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);
        self.flags.v = false;
    }

    fn sei(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);
        self.flags.i = true;
    }

    fn cli(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);
        self.flags.i = false;
    }

    fn sed(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);
        self.flags.d = true;
    }

    fn cld(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);
        self.flags.d = false;
    }

    fn sec(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);
        self.flags.c = true;
    }

    fn clc(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);
        self.flags.c = false;
    }

    fn pla(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);
        self.a = self.pop();

        let a = self.a;
        self.set_n(a);
        self.set_z(a);
    }

    fn pha(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);
        let a = self.a;
        self.push(a);
    }

    fn plp(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);
        let x = self.pop();

        self.flags = super::CPUFlags::from_byte(x);
    }

    fn php(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);
        let result : u8 = {
                let flags = &self.flags;
                ((flags.n as u8) << 7) +
                ((flags.v as u8) << 6) +
                              (1 << 5) +
                ((flags.d as u8) << 3) +
                ((flags.i as u8) << 2) +
                ((flags.z as u8) << 1) +
                (flags.c as u8)
        };

        self.push(result);
    }

    fn txs(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);
        self.sp = self.x;
    }

    fn tsx(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);
        self.x = self.sp;
        let x = self.x;
        self.set_n(x);
        self.set_z(x);
    }

    fn tya(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);
        self.a = self.y;
        let a = self.a;
        self.set_n(a);
        self.set_z(a);
    }

    fn tay(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);
        self.y = self.a;
        let y = self.y;
        self.set_n(y);
        self.set_z(y);
    }

    fn txa(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);
        self.a = self.x;
        let a = self.a;
        self.set_n(a);
        self.set_z(a);
    }

    fn tax(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);
        self.x = self.a;
        let x = self.x;
        self.set_n(x);
        self.set_z(x);
    }

    fn ror(&mut self, arg : InstrArg) {
        let result = {
            let c = self.flags.c as u8;
            match arg {
                InstrArg::Implied        => {
                    self.flags.c = self.a & 0x1 != 0;
                    self.a = (self.a >> 1) + (c << 7);
                    self.a
                },
                InstrArg::Address(addr)  => {
                    let mut target_val = self.mem.loadb(addr);
                    self.flags.c = target_val & 0x1 != 0;
                    target_val = (target_val >> 1) + (c << 7);
                    self.mem.storeb(addr, target_val);
                    target_val
                },

                _                        => panic!("illegal instruction"),
            }
        };
        self.set_n(result);
        self.set_z(result);
    }

    fn rol(&mut self, arg : InstrArg) {
        let result = {
            let c = self.flags.c as u8;
            match arg {
                InstrArg::Implied        => {
                    self.flags.c = self.a & 0x80 != 0;
                    self.a = (self.a << 1) + c;
                    self.a
                },
                InstrArg::Address(addr)  => {
                    let mut target_val = self.mem.loadb(addr);
                    self.flags.c = target_val & 0x80 != 0;
                    target_val = (target_val << 1) + c;
                    self.mem.storeb(addr, target_val);
                    target_val
                },

                _                        => panic!("illegal instruction"),
            }
        };
        self.set_n(result);
        self.set_z(result);
    }

    fn asl(&mut self, arg : InstrArg) {
        let result = {
            match arg {
                InstrArg::Implied        => {
                    self.flags.c = self.a & 0x80 != 0;
                    self.a = self.a << 1;
                    self.a
                },
                InstrArg::Address(addr)  => {
                    let mut target_val = self.mem.loadb(addr);
                    self.flags.c = target_val & 0x80 != 0;
                    target_val = target_val << 1;
                    self.mem.storeb(addr, target_val);
                    target_val
                },

                _                        => panic!("illegal instruction"),
            }
        };
        self.set_n(result);
        self.set_z(result);
    }

    fn lsr(&mut self, arg : InstrArg) {
        let result = {
            match arg {
                InstrArg::Implied        => {
                    self.flags.c = self.a & 0x1 != 0;
                    self.a = self.a >> 1;
                    self.a
                },
                InstrArg::Address(addr)  => {
                    let mut target_val = self.mem.loadb(addr);
                    self.flags.c = target_val & 0x1 != 0;
                    target_val = target_val >> 1;
                    self.mem.storeb(addr, target_val);
                    target_val
                },

                _                        => panic!("illegal instruction"),
            }
        };
        self.flags.n = false;
        self.set_z(result);
    }

    fn iny(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);
        self.y = self.y.wrapping_add(1);
        let y = self.y;

        self.set_n(y);
        self.set_z(y);
    }

    fn inx(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);
        self.x = self.x.wrapping_add(1);
        let x = self.x;

        self.set_n(x);
        self.set_z(x);
    }

    fn inc(&mut self, arg : InstrArg) {
        let addr = self.unwrap_addr(arg);
        let mut mem_val = self.mem.loadb(addr);
        mem_val = mem_val.wrapping_add(1);
        self.mem.storeb(addr, mem_val);

        self.set_n(mem_val);
        self.set_z(mem_val);
    }

    fn dey(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);
        self.y = self.y.wrapping_sub(1);
        let y = self.y;

        self.set_n(y);
        self.set_z(y);
    }

    fn dex(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);
        self.x = self.x.wrapping_sub(1);
        let x = self.x;

        self.set_n(x);
        self.set_z(x);
    }

    fn dec(&mut self, arg : InstrArg) {
        let addr = self.unwrap_addr(arg);
        let mut mem_val = self.mem.loadb(addr);
        mem_val = mem_val.wrapping_sub(1);
        self.mem.storeb(addr, mem_val);

        self.set_n(mem_val);
        self.set_z(mem_val);
    }

    fn cpy(&mut self, arg : InstrArg) {
        let val = self.unwrap_imm_or_memval(arg);

        let y = self.y;
        self.set_compare_flags(y, val);
    }

    fn cpx(&mut self, arg : InstrArg) {
        let val = self.unwrap_imm_or_memval(arg);

        let x = self.x;
        self.set_compare_flags(x, val);
    }

    fn cmp(&mut self, arg : InstrArg) {
        let val = self.unwrap_imm_or_memval(arg);

        let a = self.a;
        self.set_compare_flags(a, val);
    }

    fn sbc(&mut self, arg : InstrArg) {
        let val = self.unwrap_imm_or_memval(arg);

        let result = if self.flags.d {
            let a_bcd = from_bcd(self.a);
            let val_bcd = from_bcd(val);
            let ret = to_bcd((100 + a_bcd - val_bcd - !self.flags.c as u8) % 100);
            self.flags.c = (a_bcd as i16) - (val_bcd as i16)
                                          - !self.flags.c as i16 >= 0;
            ret
        }
        else {
            // A - M - !C
            let ret = self.a.wrapping_sub(val).wrapping_sub(!self.flags.c as u8);
            self.flags.c = (self.a as i16) - (val as i16)
                                           - !self.flags.c as i16 >= 0;
            ret
        };

        // xor bit 7 of both nums (check if nums have different sign)
        let diff_sign = (val >> 7) ^ (self.a >> 7);
        self.flags.v = diff_sign & !((val >> 7) ^ (result >> 7)) != 0;

        self.set_z(result);
        self.set_n(result);

        self.a = result;
    }

    // assuming I don't need to worry about handling invalid BCD the same way
    // that the 6502 does
    fn adc(&mut self, arg : InstrArg) {
        let val = self.unwrap_imm_or_memval(arg);

        // handle the addition differently in bcd mode
        let result = if self.flags.d {
            let a_bcd = from_bcd(self.a);
            let val_bcd = from_bcd(val);
            // valid BCD shouldn't crash/overflow here because 99 + 99 + 1 = 199,
            // less than 255
            let ret = to_bcd((a_bcd + val_bcd + self.flags.c as u8) % 100);
            self.flags.c = (val_bcd as u16) + (a_bcd as u16)
                                            + self.flags.c as u16 > 99;
            ret
        }
        else {
            let ret = val.wrapping_add(self.a).wrapping_add(self.flags.c as u8);
            self.flags.c = (val as u16) + (self.a as u16)
                                        + self.flags.c as u16 > 0xFF;
            ret
        };

        // xnor bit 7 of both nums (check if both nums have the same sign)
        let same_sign = !((val >> 7) ^ (self.a >> 7));
        self.flags.v = same_sign & ((val >> 7) ^ (result >> 7)) != 0;

        self.set_z(result);
        self.set_n(result);

        self.a = result;
    }

    fn eor(&mut self, arg : InstrArg) {
        let val = self.a ^ self.unwrap_imm_or_memval(arg);

        self.set_z(val);
        self.set_n(val);
        self.a = val;
    }


    fn and(&mut self, arg : InstrArg) {
        let val = self.a & self.unwrap_imm_or_memval(arg);

        self.set_z(val);
        self.set_n(val);
        self.a = val;
    }

    fn ora(&mut self, arg : InstrArg) {
        let val = self.a | self.unwrap_imm_or_memval(arg);

        self.set_z(val);
        self.set_n(val);
        self.a = val;
    }

    fn ldy(&mut self, arg : InstrArg) {
        let val = self.unwrap_imm_or_memval(arg);

        self.set_z(val);
        self.set_n(val);
        self.y = val;
    }

    fn ldx(&mut self, arg : InstrArg) {
        let val = self.unwrap_imm_or_memval(arg);

        self.set_z(val);
        self.set_n(val);
        self.x = val;
    }

    fn lda(&mut self, arg : InstrArg) {
        let val = self.unwrap_imm_or_memval(arg);

        self.set_z(val);
        self.set_n(val);
        self.a = val;
    }

    fn sta(&mut self, arg : InstrArg) {
        let addr = self.unwrap_addr(arg);
        self.mem.storeb(addr, self.a);
    }

    fn stx(&mut self, arg : InstrArg) {
        let addr = self.unwrap_addr(arg);
        self.mem.storeb(addr, self.x);
    }

    fn sty(&mut self, arg : InstrArg) {
        let addr = self.unwrap_addr(arg);
        self.mem.storeb(addr, self.y);
    }
}
