use super::CPU;
#[cfg(test)]
mod tests;
pub mod decode;

fn from_bcd(x : u8) -> u8 { (x & 0x0F) + ((x & 0xF0) >> 4) * 10 }
fn to_bcd(x : u8) -> u8 { ((x / 10) << 4) + (x % 10) }

// describes the possible types of arguments for instructions
enum InstrArg {
    Implied,
    Immediate(u8),
    Address(u16),
}


const STACK_BEGIN : usize = 0x100;

impl CPU {
    fn unwrap_argtype_one(&self, arg : InstrArg) -> u8 {
        match arg {
            InstrArg::Immediate(imm) => imm,
            InstrArg::Address(addr)  => self.mem[addr as usize],
            _                        => panic!("illegal instruction"),
        }
    }

    fn unwrap_addr_ref(&mut self, arg : InstrArg) -> &mut u8 {
        match arg {
            InstrArg::Address(addr) => &mut self.mem[addr as usize],
            _                       => panic!("illegal instruction"),
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

    fn bpl(&mut self, arg : InstrArg) {
        if !self.flags.n { self.pc = self.unwrap_addr(arg) };
    }

    fn bmi(&mut self, arg : InstrArg) {
        if self.flags.n { self.pc = self.unwrap_addr(arg) };
    }

    fn bvc(&mut self, arg : InstrArg) {
        if !self.flags.v { self.pc = self.unwrap_addr(arg) };
    }

    fn bvs(&mut self, arg : InstrArg) {
        if self.flags.v { self.pc = self.unwrap_addr(arg) };
    }

    fn bcc(&mut self, arg : InstrArg) {
        if !self.flags.c { self.pc = self.unwrap_addr(arg) };
    }

    fn bcs(&mut self, arg : InstrArg) {
        if self.flags.c { self.pc = self.unwrap_addr(arg) };
    }

    fn bne(&mut self, arg : InstrArg) {
        if !self.flags.z { self.pc = self.unwrap_addr(arg) };
    }

    fn beq(&mut self, arg : InstrArg) {
        if self.flags.z { self.pc = self.unwrap_addr(arg) };
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
        self.sp += 1;
        self.a = self.mem[STACK_BEGIN + self.sp as usize];

        let a = self.a;
        self.set_n(a);
        self.set_z(a);
    }

    fn pha(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);
        self.mem[STACK_BEGIN + self.sp as usize] = self.a;
        self.sp -= 1;
    }

    fn plp(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);
        self.sp += 1;
        let x = self.mem[STACK_BEGIN + self.sp as usize];

        self.flags = super::CPUFlags {
            n : (x & 0x80 != 0),
            v : (x & 0x40 != 0),
            b : self.flags.b   , // b is the only flag that shouldn't be affected
            d : (x & 0x08 != 0),
            i : (x & 0x04 != 0),
            z : (x & 0x02 != 0),
            c : (x & 0x01 != 0),
        }
    }

    fn php(&mut self, arg : InstrArg) {
        self.unwrap_implied(arg);
        let flags = &self.flags;
        let result : u8 =
            ((flags.n as u8) << 7) +
            ((flags.v as u8) << 6) +
                          (1 << 5) +
            ((flags.b as u8) << 4) +
            ((flags.d as u8) << 3) +
            ((flags.i as u8) << 2) +
            ((flags.z as u8) << 1) +
             (flags.c as u8);
        self.mem[STACK_BEGIN + self.sp as usize] = result;
        self.sp -= 1;
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
            let target_ref = match arg {
                InstrArg::Implied        => &mut self.a,
                InstrArg::Address(addr)  => &mut self.mem[addr as usize],
                _                        => panic!("illegal instruction"),
            };
            let c = self.flags.c as u8;
            self.flags.c = *target_ref & 0x1 != 0;
            *target_ref = (*target_ref >> 1) + (c << 7);
            *target_ref
        };
        self.set_n(result);
        self.set_z(result);
    }

    fn rol(&mut self, arg : InstrArg) {
        let result = {
            let target_ref = match arg {
                InstrArg::Implied        => &mut self.a,
                InstrArg::Address(addr)  => &mut self.mem[addr as usize],
                _                        => panic!("illegal instruction"),
            };
            let c = self.flags.c as u8;
            self.flags.c = *target_ref & 0x80 != 0;
            *target_ref = (*target_ref << 1) + c;
            *target_ref
        };
        self.set_n(result);
        self.set_z(result);
    }

    fn asl(&mut self, arg : InstrArg) {
        let result = {
            let target_ref = match arg {
                InstrArg::Implied        => &mut self.a,
                InstrArg::Address(addr)  => &mut self.mem[addr as usize],
                _                        => panic!("illegal instruction"),
            };
            self.flags.c = *target_ref & 0x80 != 0;
            *target_ref = *target_ref << 1;
            *target_ref
        };
        self.set_n(result);
        self.set_z(result);
    }

    fn lsr(&mut self, arg : InstrArg) {
        let result = {
            let target_ref = match arg {
                InstrArg::Implied        => &mut self.a,
                InstrArg::Address(addr)  => &mut self.mem[addr as usize],
                _                        => panic!("illegal instruction"),
            };
            self.flags.c = *target_ref & 0x1 != 0;
            *target_ref = *target_ref >> 1;
            *target_ref
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
        let mem_val = {
            let mem_ref : &mut u8 = self.unwrap_addr_ref(arg);
            *mem_ref = mem_ref.wrapping_add(1);
            *mem_ref
        };

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
        let mem_val = {
            let mem_ref : &mut u8 = self.unwrap_addr_ref(arg);
            *mem_ref = mem_ref.wrapping_sub(1);
            *mem_ref
        };

        self.set_n(mem_val);
        self.set_z(mem_val);
    }

    fn cpy(&mut self, arg : InstrArg) {
        let val = self.unwrap_argtype_one(arg);

        let y = self.y;
        self.set_compare_flags(y, val);
    }

    fn cpx(&mut self, arg : InstrArg) {
        let val = self.unwrap_argtype_one(arg);

        let x = self.x;
        self.set_compare_flags(x, val);
    }

    fn cmp(&mut self, arg : InstrArg) {
        let val = self.unwrap_argtype_one(arg);

        let a = self.a;
        self.set_compare_flags(a, val);
    }

    fn sbc(&mut self, arg : InstrArg) {
        let val = self.unwrap_argtype_one(arg);

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
        let val = self.unwrap_argtype_one(arg);

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
        let val = self.a ^ self.unwrap_argtype_one(arg);

        self.set_z(val);
        self.set_n(val);
        self.a = val;
    }


    fn and(&mut self, arg : InstrArg) {
        let val = self.a & self.unwrap_argtype_one(arg);

        self.set_z(val);
        self.set_n(val);
        self.a = val;
    }

    fn ora(&mut self, arg : InstrArg) {
        let val = self.a | self.unwrap_argtype_one(arg);

        self.set_z(val);
        self.set_n(val);
        self.a = val;
    }

    fn ldy(&mut self, arg : InstrArg) {
        let val = self.unwrap_argtype_one(arg);

        self.set_z(val);
        self.set_n(val);
        self.y = val;
    }

    fn ldx(&mut self, arg : InstrArg) {
        let val = self.unwrap_argtype_one(arg);

        self.set_z(val);
        self.set_n(val);
        self.x = val;
    }

    fn lda(&mut self, arg : InstrArg) {
        let val = self.unwrap_argtype_one(arg);

        self.set_z(val);
        self.set_n(val);
        self.a = val;
    }

    fn sta(&mut self, arg : InstrArg) {
        *self.unwrap_addr_ref(arg) = self.a;
    }

    fn stx(&mut self, arg : InstrArg) {
        *self.unwrap_addr_ref(arg) = self.x;
    }

    fn sty(&mut self, arg : InstrArg) {
        *self.unwrap_addr_ref(arg) = self.y;
    }
}
