use super::CPU;
#[cfg(test)]
mod tests;
mod decode;



fn from_bcd(x : u8) -> u8 { (x & 0x0F) + ((x & 0xF0) >> 4) * 10 }
fn to_bcd(x : u8) -> u8 { ((x / 10) << 4) + (x % 10) }

// describes the possible types of arguments for instructions
enum InstrArg {
    Implied,
    Immediate(u8),
    Address(u16),
}

impl CPU {
    pub fn exec_op(&mut self, op : u8) {
        decode::INSTR[op as usize](self);
    }

    fn sbc(&mut self, arg : InstrArg) {
        let val = match arg {
            InstrArg::Immediate(imm) => imm,
            InstrArg::Address(addr)  => self.mem[addr as usize],
            _                        => panic!("illegal instruction"),
        };

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
        let val = match arg {
            InstrArg::Immediate(imm) => imm,
            InstrArg::Address(addr)  => self.mem[addr as usize],
            _                        => panic!("illegal instruction"),
        };

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
        let val = self.a ^ match arg {
            InstrArg::Immediate(imm) => imm,
            InstrArg::Address(addr)  => self.mem[addr as usize],
            _                        => panic!("illegal instruction"),
        };
        self.set_z(val);
        self.set_n(val);

        self.a = val;
    }


    fn and(&mut self, arg : InstrArg) {
        let val = self.a & match arg {
            InstrArg::Immediate(imm) => imm,
            InstrArg::Address(addr)  => self.mem[addr as usize],
            _                        => panic!("illegal instruction"),
        };
        self.set_z(val);
        self.set_n(val);

        self.a = val;
    }

    fn ora(&mut self, arg : InstrArg) {
        let val = self.a | match arg {
            InstrArg::Immediate(imm) => imm,
            InstrArg::Address(addr)  => self.mem[addr as usize],
            _                        => panic!("illegal instruction"),
        };
        self.set_z(val);
        self.set_n(val);

        self.a = val;
    }

    fn ldy(&mut self, arg : InstrArg) {
        let val = match arg {
            InstrArg::Immediate(imm) => imm,
            InstrArg::Address(addr)  => self.mem[addr as usize],
            _                        => panic!("illegal instruction"),
        };
        self.set_z(val);
        self.set_n(val);

        self.y = val;
    }

    fn ldx(&mut self, arg : InstrArg) {
        let val = match arg {
            InstrArg::Immediate(imm) => imm,
            InstrArg::Address(addr)  => self.mem[addr as usize],
            _                        => panic!("illegal instruction"),
        };
        self.set_z(val);
        self.set_n(val);

        self.x = val;
    }

    fn lda(&mut self, arg : InstrArg) {
        let val = match arg {
            InstrArg::Immediate(imm) => imm,
            InstrArg::Address(addr)  => self.mem[addr as usize],
            _                        => panic!("illegal instruction"),
        };
        self.set_z(val);
        self.set_n(val);

        self.a = val;
    }

    fn sta(&mut self, arg : InstrArg) {
        match arg {
            InstrArg::Address(addr) => self.mem[addr as usize] = self.a,
            _                       => panic!("illegal instruction"),
        }
    }

    fn stx(&mut self, arg : InstrArg) {
        match arg {
            InstrArg::Address(addr) => self.mem[addr as usize] = self.x,
            _                       => panic!("illegal instruction"),
        }
    }

    fn sty(&mut self, arg : InstrArg) {
        match arg {
            InstrArg::Address(addr) => self.mem[addr as usize] = self.y,
            _                       => panic!("illegal instruction"),
        }
    }
}
