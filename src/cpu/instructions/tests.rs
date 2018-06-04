use super::{ CPU, InstrArg, Memory };
use ::cpu::CPUFlags;

#[test]
fn brk_rti() {
    let mut c = CPU::test();
    c.mem.storeb(0xFFFE, 0xAB);
    c.mem.storeb(0xFFFF, 0xCD);
    c.pc = 0x8001;
    c.brk(InstrArg::Implied);
    assert_eq!(c.mem.loadb(0x01FF), 0x80);
    assert_eq!(c.mem.loadb(0x01FE), 0x02);
    assert_eq!(c.mem.loadb(0x01FD), 0b00110000);
    assert_eq!(c.sp, 0xFC);
    assert_eq!(c.pc, 0xCDAB);
    assert_eq!(c.flags.i, true);

    c.rti(InstrArg::Implied);
    assert_eq!(c.sp, 0xFF);
    assert_eq!(c.pc, 0x8002);
    assert_eq!(c.flags.i, false);
}

#[test]
fn rts() {
    let mut c = CPU::test();
    c.sp = 0xFB;
    c.mem.storeb(0x01FF, 0x07);
    c.mem.storeb(0x01FE, 0xFE);
    c.mem.storeb(0x01FD, 0x90);
    c.mem.storeb(0x01FC, 0x02);
    c.rts(InstrArg::Implied);
    assert_eq!(c.sp, 0xFD);
    assert_eq!(c.pc, 0x9003);
    c.rts(InstrArg::Implied);
    assert_eq!(c.sp, 0xFF);
    assert_eq!(c.pc, 0x07FF);
}

#[test]
fn jsr() {
    let mut c = CPU::test();
    c.pc = 0x8003;
    c.jsr(InstrArg::Address(0x7F03));
    assert_eq!(c.sp, 0xFD);
    assert_eq!(c.mem.loadb(0x01FF), 0x80);
    assert_eq!(c.mem.loadb(0x01FE), 0x02);
    assert_eq!(c.pc, 0x7F03);

    c.jsr(InstrArg::Address(0xABCD));
    assert_eq!(c.sp, 0xFB);
    assert_eq!(c.mem.loadb(0x01FF), 0x80);
    assert_eq!(c.mem.loadb(0x01FE), 0x02);
    assert_eq!(c.mem.loadb(0x01FD), 0x7F);
    assert_eq!(c.mem.loadb(0x01FC), 0x02);
    assert_eq!(c.pc, 0xABCD);
}

#[test]
fn bit() {
    let mut c = CPU::test();
    c.a = 0x00;
    c.mem.storeb(0x00FF, 0xFF);
    c.bit(InstrArg::Address(0x00FF));
    assert_eq!(c.flags.n, true);
    assert_eq!(c.flags.v, true);
    assert_eq!(c.flags.z, true);

    c.mem.storeb(0x00FF, 0x8F);
    c.bit(InstrArg::Address(0x00FF));
    assert_eq!(c.flags.n, true);
    assert_eq!(c.flags.v, false);
    assert_eq!(c.flags.z, true);

    c.mem.storeb(0x00FF, 0x4F);
    c.bit(InstrArg::Address(0x00FF));
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.v, true);
    assert_eq!(c.flags.z, true);

    c.mem.storeb(0x00FF, 0x0F);
    c.bit(InstrArg::Address(0x00FF));
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.v, false);
    assert_eq!(c.flags.z, true);

    c.a = 1;
    c.mem.storeb(0x00FF, 0xFF);
    c.bit(InstrArg::Address(0x00FF));
    assert_eq!(c.flags.n, true);
    assert_eq!(c.flags.v, true);
    assert_eq!(c.flags.z, false);

    c.mem.storeb(0x00FF, 0x8F);
    c.bit(InstrArg::Address(0x00FF));
    assert_eq!(c.flags.n, true);
    assert_eq!(c.flags.v, false);
    assert_eq!(c.flags.z, false);

    c.mem.storeb(0x00FF, 0x4F);
    c.bit(InstrArg::Address(0x00FF));
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.v, true);
    assert_eq!(c.flags.z, false);

    c.mem.storeb(0x00FF, 0x0F);
    c.bit(InstrArg::Address(0x00FF));
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.v, false);
    assert_eq!(c.flags.z, false);
}

#[test]
fn branches() {
    let mut c = CPU::test();

    c.pc = 0x8000;
    c.flags.z = false;
    c.beq(InstrArg::Address(0x80FF));
    assert_eq!(c.pc, 0x8000);

    c.pc = 0x8000;
    c.flags.z = true;
    c.beq(InstrArg::Address(0x80FF));
    assert_eq!(c.pc, 0x80FF);

    c.pc = 0x8000;
    c.flags.z = true;
    c.bne(InstrArg::Address(0x80FF));
    assert_eq!(c.pc, 0x8000);

    c.pc = 0x8000;
    c.flags.z = false;
    c.bne(InstrArg::Address(0x80FF));
    assert_eq!(c.pc, 0x80FF);

    c.pc = 0x8000;
    c.flags.c = false;
    c.bcs(InstrArg::Address(0x80FF));
    assert_eq!(c.pc, 0x8000);

    c.pc = 0x8000;
    c.flags.c = true;
    c.bcs(InstrArg::Address(0x80FF));
    assert_eq!(c.pc, 0x80FF);

    c.pc = 0x8000;
    c.flags.c = true;
    c.bcc(InstrArg::Address(0x80FF));
    assert_eq!(c.pc, 0x8000);

    c.pc = 0x8000;
    c.flags.c = false;
    c.bcc(InstrArg::Address(0x80FF));
    assert_eq!(c.pc, 0x80FF);

    c.pc = 0x8000;
    c.flags.v = false;
    c.bvs(InstrArg::Address(0x80FF));
    assert_eq!(c.pc, 0x8000);

    c.pc = 0x8000;
    c.flags.v = true;
    c.bvs(InstrArg::Address(0x80FF));
    assert_eq!(c.pc, 0x80FF);

    c.pc = 0x8000;
    c.flags.v = true;
    c.bvc(InstrArg::Address(0x80FF));
    assert_eq!(c.pc, 0x8000);

    c.pc = 0x8000;
    c.flags.v = false;
    c.bvc(InstrArg::Address(0x80FF));
    assert_eq!(c.pc, 0x80FF);

    c.pc = 0x8000;
    c.flags.n = false;
    c.bmi(InstrArg::Address(0x80FF));
    assert_eq!(c.pc, 0x8000);

    c.pc = 0x8000;
    c.flags.n = true;
    c.bmi(InstrArg::Address(0x80FF));
    assert_eq!(c.pc, 0x80FF);

    c.pc = 0x8000;
    c.flags.n = true;
    c.bpl(InstrArg::Address(0x80FF));
    assert_eq!(c.pc, 0x8000);

    c.pc = 0x8000;
    c.flags.n = false;
    c.bpl(InstrArg::Address(0x80FF));
    assert_eq!(c.pc, 0x80FF);

    c.pc = 0x8000;
    c.jmp(InstrArg::Address(0x80FF));
    assert_eq!(c.pc, 0x80FF);
    c.jmp(InstrArg::Address(0x0));
    assert_eq!(c.pc, 0x0);
}

#[test]
fn flags() {
    let mut c = CPU::test();
    c.sec(InstrArg::Implied);
    assert_eq!(c.flags.c, true);
    c.clc(InstrArg::Implied);
    assert_eq!(c.flags.c, false);

    c.sed(InstrArg::Implied);
    assert_eq!(c.flags.d, true);
    c.cld(InstrArg::Implied);
    assert_eq!(c.flags.d, false);

    c.sei(InstrArg::Implied);
    assert_eq!(c.flags.i, true);
    c.cli(InstrArg::Implied);
    assert_eq!(c.flags.i, false);

    c.flags.v = true;
    c.clv(InstrArg::Implied);
    assert_eq!(c.flags.v, false);
}

#[test]
fn pla() {
    let mut c = CPU::test();
    c.mem.storeb(0x1FF, 0xFF);
    c.sp -= 1;

    c.pla(InstrArg::Implied);
    assert_eq!(c.sp, 0xFF);
    assert_eq!(c.a, 0xFF);
    assert_eq!(c.flags.n, true);
    assert_eq!(c.flags.z, false);
}

#[test]
fn pha() {
    let mut c = CPU::test();
    c.a = 0xFF;

    c.pha(InstrArg::Implied);
    assert_eq!(c.sp, 0xFE);
    assert_eq!(c.mem.loadb(0x1FF), 0xFF);
}

#[test]
fn plp() {
    let mut c = CPU::test();
    c.mem.storeb(0x1FF, 0xFF);
    c.sp -= 1;

    c.plp(InstrArg::Implied);
    assert_eq!(c.sp, 0xFF);
    assert_eq!(c.flags.n, true);
    assert_eq!(c.flags.v, true);
    assert_eq!(c.flags.d, true);
    assert_eq!(c.flags.i, true);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.c, true);

    c.mem.storeb(0x1FF, 0b10110101);
    c.sp -= 1;

    c.plp(InstrArg::Implied);
    assert_eq!(c.sp, 0xFF);
    assert_eq!(c.flags.n, true);
    assert_eq!(c.flags.v, false);
    assert_eq!(c.flags.d, false);
    assert_eq!(c.flags.i, true);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.c, true);
}

#[test]
fn php() {
    let mut c = CPU::test();
    c.flags = CPUFlags {
        n : true,
        v : true,
        d : true,
        i : true,
        z : true,
        c : true,
    };
    c.php(InstrArg::Implied);
    assert_eq!(c.sp, 0xFE);
    assert_eq!(c.mem.loadb(0x1FF), 0b11101111);

    c.flags = CPUFlags {
        n : false,
        v : true,
        d : true,
        i : false,
        z : true,
        c : false,
    };
    c.php(InstrArg::Implied);
    assert_eq!(c.sp, 0xFD);
    assert_eq!(c.mem.loadb(0x1FE), 0b01101010);
}

#[test]
fn txs() {
    // this shouldn't change flags
    let mut c = CPU::test();
    c.x = 0;
    c.flags.z = false;
    c.flags.n = true;
    c.txs(InstrArg::Implied);
    assert_eq!(c.sp, 0);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);
}

#[test]
fn tsx() {
    let mut c = CPU::test();
    c.sp = 5;
    c.tsx(InstrArg::Implied);
    assert_eq!(c.x, 5);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, false);
    c.sp = 0;
    c.tsx(InstrArg::Implied);
    assert_eq!(c.x, 0);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);
    c.sp = 0xFF;
    c.tsx(InstrArg::Implied);
    assert_eq!(c.x, 0xFF);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);
}

#[test]
fn tya() {
    let mut c = CPU::test();
    c.y = 5;
    c.tya(InstrArg::Implied);
    assert_eq!(c.a, 5);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, false);
    c.y = 0;
    c.tya(InstrArg::Implied);
    assert_eq!(c.a, 0);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);
    c.y = 0xFF;
    c.tya(InstrArg::Implied);
    assert_eq!(c.a, 0xFF);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);
}

#[test]
fn tay() {
    let mut c = CPU::test();
    c.a = 5;
    c.tay(InstrArg::Implied);
    assert_eq!(c.y, 5);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, false);
    c.a = 0;
    c.tay(InstrArg::Implied);
    assert_eq!(c.y, 0);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);
    c.a = 0xFF;
    c.tay(InstrArg::Implied);
    assert_eq!(c.y, 0xFF);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);
}

#[test]
fn txa() {
    let mut c = CPU::test();
    c.x = 5;
    c.txa(InstrArg::Implied);
    assert_eq!(c.a, 5);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, false);
    c.x = 0;
    c.txa(InstrArg::Implied);
    assert_eq!(c.a, 0);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);
    c.x = 0xFF;
    c.txa(InstrArg::Implied);
    assert_eq!(c.a, 0xFF);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);
}

#[test]
fn tax() {
    let mut c = CPU::test();
    c.a = 5;
    c.tax(InstrArg::Implied);
    assert_eq!(c.x, 5);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, false);
    c.a = 0;
    c.tax(InstrArg::Implied);
    assert_eq!(c.x, 0);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);
    c.a = 0xFF;
    c.tax(InstrArg::Implied);
    assert_eq!(c.x, 0xFF);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);
}

#[test]
fn rol() {
    let mut c = CPU::test();
    c.a = 0x80;
    c.rol(InstrArg::Implied);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.c, true);
    c.rol(InstrArg::Implied);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.c, false);

    c.mem.storeb(0x00FF, 0x80);
    c.rol(InstrArg::Address(0xFF));
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.c, true);
    c.rol(InstrArg::Address(0xFF));
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.c, false);
}

#[test]
fn ror() {
    let mut c = CPU::test();
    c.a = 0x01;
    c.ror(InstrArg::Implied);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.c, true);
    c.ror(InstrArg::Implied);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);
    assert_eq!(c.flags.c, false);
    c.ror(InstrArg::Implied);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.c, false);

    c.mem.storeb(0x00FF, 0x01);
    c.ror(InstrArg::Address(0xFF));
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.c, true);
    c.ror(InstrArg::Address(0xFF));
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);
    assert_eq!(c.flags.c, false);
    c.ror(InstrArg::Address(0xFF));
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.c, false);
}

#[test]
fn asl() {
    let mut c = CPU::test();
    c.a = 0x01;
    c.asl(InstrArg::Implied);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.c, false);
    c.a = 0x80;
    c.asl(InstrArg::Implied);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.c, true);
    c.asl(InstrArg::Implied);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.c, false);
}

#[test]
fn lsr() {
    let mut c = CPU::test();
    c.a = 0x01;
    c.lsr(InstrArg::Implied);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.c, true);
    c.lsr(InstrArg::Implied);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.c, false);
    c.a = 0x80;
    c.lsr(InstrArg::Implied);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.c, false);
}

#[test]
fn iny() {
    let mut c = CPU::test();
    c.y = 0xFE;
    c.iny(InstrArg::Implied);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);

    c.iny(InstrArg::Implied);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);
}

#[test]
fn inx() {
    let mut c = CPU::test();
    c.x = 0xFE;
    c.inx(InstrArg::Implied);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);

    c.inx(InstrArg::Implied);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);
}

#[test]
fn inc() {
    let mut c = CPU::test();
    c.mem.storeb(0x00FF, 0xFE);
    c.inc(InstrArg::Address(0xFF));
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);

    c.inc(InstrArg::Address(0xFF));
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);
}

#[test]
fn dey() {
    let mut c = CPU::test();
    c.y = 0x1;
    c.dey(InstrArg::Implied);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);

    c.dey(InstrArg::Implied);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);
}

#[test]
fn dex() {
    let mut c = CPU::test();
    c.x = 0x1;
    c.dex(InstrArg::Implied);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);

    c.dex(InstrArg::Implied);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);
}

#[test]
fn dec() {
    let mut c = CPU::test();
    c.mem.storeb(0x00FF, 0x1);
    c.dec(InstrArg::Address(0xFF));
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);

    c.dec(InstrArg::Address(0xFF));
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);
}

#[test]
fn cpx() {
    let mut c = CPU::test();
    c.x = 5;
    // self.a == arg
    c.cpx(InstrArg::Immediate(5));
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.c, true);

    // self.a > arg
    c.cpx(InstrArg::Immediate(4));
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.c, true);

    // self.a < arg
    c.cpx(InstrArg::Immediate(6));
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn cpy() {
    let mut c = CPU::test();
    c.y = 5;
    // self.a == arg
    c.cpy(InstrArg::Immediate(5));
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.c, true);

    // self.a > arg
    c.cpy(InstrArg::Immediate(4));
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.c, true);

    // self.a < arg
    c.cpy(InstrArg::Immediate(6));
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn cmp() {
    let mut c = CPU::test();
    c.a = 5;
    // self.a == arg
    c.cmp(InstrArg::Immediate(5));
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.c, true);

    // self.a > arg
    c.cmp(InstrArg::Immediate(4));
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.c, true);

    // self.a < arg
    c.cmp(InstrArg::Immediate(6));
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn sbc() {
    let mut c = CPU::test();
    c.a = 5;
    c.flags.c = true;
    // pos - pos = pos
    c.sbc(InstrArg::Immediate(3));
    assert_eq!(c.a, 2);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.v, false);
    assert_eq!(c.flags.c, true);
    c.a = 5;
    // pos - pos = neg
    c.sbc(InstrArg::Immediate(7));
    assert_eq!(c.a, 0xFE);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);
    assert_eq!(c.flags.v, false);
    assert_eq!(c.flags.c, false);
    c.sbc(InstrArg::Immediate(0xFE));
    assert_eq!(c.a, 0xFF);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);
    assert_eq!(c.flags.v, false);
    assert_eq!(c.flags.c, false);
    c.a = 0xFF;
    c.flags.c = true;
    // neg - neg = pos
    c.sbc(InstrArg::Immediate(0xFF));
    assert_eq!(c.a, 0x0);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.v, false);
    assert_eq!(c.flags.c, true);
    c.a = 0x7F;
    // pos - neg = neg;
    c.sbc(InstrArg::Immediate(0x80));
    assert_eq!(c.a, 0xFF);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);
    assert_eq!(c.flags.v, true);
    assert_eq!(c.flags.c, false);
    c.a = 0x80;
    c.flags.c = true;
    // neg - pos = pos;
    c.sbc(InstrArg::Immediate(0x01));
    assert_eq!(c.a, 0x7F);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.v, true);
    assert_eq!(c.flags.c, true);

    // BCD
    let mut c = CPU::test();
    c.a = 0x15;
    c.flags.c = true;
    c.flags.d = true;
    // pos - pos = pos
    c.sbc(InstrArg::Immediate(0x06));
    assert_eq!(c.a, 0x09);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.v, false);
    assert_eq!(c.flags.c, true);
    // pos - pos = neg
    c.a = 0x05;
    c.sbc(InstrArg::Immediate(0x06));
    assert_eq!(c.a, 0x99);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);
    assert_eq!(c.flags.v, false);
    assert_eq!(c.flags.c, false);
    c.sbc(InstrArg::Immediate(0x99));
    assert_eq!(c.a, 0x99);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);
    assert_eq!(c.flags.v, false);
    assert_eq!(c.flags.c, false);
    // neg - neg = pos
    c.a = 0x80;
    c.flags.c = true;
    c.sbc(InstrArg::Immediate(0x80));
    assert_eq!(c.a, 0x00);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.v, false);
    assert_eq!(c.flags.c, true);
    // pos - neg = neg;
    c.a = 0x70;
    c.sbc(InstrArg::Immediate(0x80));
    assert_eq!(c.a, 0x90);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);
    assert_eq!(c.flags.v, true);
    assert_eq!(c.flags.c, false);
    // neg - pos = pos;
    c.a = 0x80;
    c.flags.c = true;
    c.sbc(InstrArg::Immediate(0x75));
    assert_eq!(c.a, 0x05);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.v, true);
    assert_eq!(c.flags.c, true);
}

#[test]
fn adc() {
    let mut c = CPU::test();
    c.a = 1;
    // pos + pos = pos
    c.adc(InstrArg::Immediate(1));
    assert_eq!(c.a, 2);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.v, false);
    assert_eq!(c.flags.c, false);
    // pos + pos = neg
    c.adc(InstrArg::Immediate(0x7E));
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);
    assert_eq!(c.flags.v, true);
    assert_eq!(c.flags.c, false);
    // neg + pos = neg
    c.adc(InstrArg::Immediate(0x7F));
    assert_eq!(c.a, 0xFF);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);
    assert_eq!(c.flags.v, false);
    assert_eq!(c.flags.c, false);
    // neg + pos = pos
    c.adc(InstrArg::Immediate(0x01));
    assert_eq!(c.a, 0x00);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.v, false);
    assert_eq!(c.flags.c, true);
    c.adc(InstrArg::Immediate(0x01));
    assert_eq!(c.a, 0x02);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.v, false);
    assert_eq!(c.flags.c, false);
    c.a = 0xFF;
    c.adc(InstrArg::Immediate(0xFF));
    assert_eq!(c.a, 0xFE);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);
    assert_eq!(c.flags.v, false);
    assert_eq!(c.flags.c, true);
    c.a = 0x80;
    c.adc(InstrArg::Immediate(0x80));
    assert_eq!(c.a, 0x01);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.v, true);
    assert_eq!(c.flags.c, true);

    c.a = 0xfe;
    c.flags.c = true;
    c.adc(InstrArg::Immediate(0x01));
    assert_eq!(c.a, 0x00);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.v, false);
    assert_eq!(c.flags.c, true);

    let mut c = CPU::test();
    c.a = 1;
    c.flags.d = true;
    c.adc(InstrArg::Immediate(0x99));
    assert_eq!(c.a, 0x00);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);
    assert_eq!(c.flags.v, false);
    assert_eq!(c.flags.c, true);
    c.a = 0x79;
    c.adc(InstrArg::Immediate(0x1));
    assert_eq!(c.a, 0x81);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);
    assert_eq!(c.flags.v, true);
    assert_eq!(c.flags.c, false);
    c.adc(InstrArg::Immediate(0x99));
    assert_eq!(c.a, 0x80);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);
    assert_eq!(c.flags.v, false);
    assert_eq!(c.flags.c, true);
    c.adc(InstrArg::Immediate(0x99));
    assert_eq!(c.a, 0x80);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);
    assert_eq!(c.flags.v, false);
    assert_eq!(c.flags.c, true);
}

#[test]
fn eor() {
    let mut c = CPU::test();
    c.a = 0b0101_0011;
    c.eor(InstrArg::Immediate(0b1010_0011));
    assert_eq!(c.a, 0xF0);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);

    let mut c = CPU::test();
    c.a = 0xFF;
    c.eor(InstrArg::Immediate(0xFF));
    assert_eq!(c.a, 0x0);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);
}

#[test]
fn and() {
    let mut c = CPU::test();
    // immediate
    c.a = 0b0101_0101;
    c.mem.storeb(0x8000, 0x29);
    c.mem.storeb(0x8001, 0x0F);
    c.step();
    assert_eq!(c.a, 0x05);

    let mut c = CPU::test();
    // zero page
    c.a = 0b0101_0101;
    c.mem.storeb(0x10, 0x0F);
    c.mem.storeb(0x8000, 0x25);
    c.mem.storeb(0x8001, 0x10);
    c.step();
    assert_eq!(c.a, 0x05);

    let mut c = CPU::test();
    // zero page, x
    c.a = 0b0101_0101;
    c.x = 1;
    c.mem.storeb(0x10, 0x0F);
    c.mem.storeb(0x8000, 0x35);
    c.mem.storeb(0x8001, 0x0F);
    c.step();
    assert_eq!(c.a, 0x05);

    let mut c = CPU::test();
    // indirect, x
    c.a = 0b0101_0101;
    c.x = 1;
    c.mem.storeb(0x1F, 0x0F);
    c.mem.storeb(0x10, 0x1F);
    c.mem.storeb(0x8000, 0x21);
    c.mem.storeb(0x8001, 0x0F);
    c.step();
    assert_eq!(c.a, 0x05);

    let mut c = CPU::test();
    // indirect, y
    c.a = 0b0101_0101;
    c.y = 1;
    c.mem.storeb(0x1F, 0x0F);
    c.mem.storeb(0x10, 0x1E);
    c.mem.storeb(0x8000, 0x31);
    c.mem.storeb(0x8001, 0x10);
    c.step();
    assert_eq!(c.a, 0x05);

    let mut c = CPU::test();
    // absolute
    c.a = 0b0101_0101;
    c.mem.storeb(0x07FF, 0x0F);
    c.mem.storeb(0x8000, 0x2D);
    c.mem.storeb(0x8001, 0xFF);
    c.mem.storeb(0x8002, 0x07);
    c.step();
    assert_eq!(c.a, 0x05);

    let mut c = CPU::test();
    // absolute, x
    c.a = 0b0101_0101;
    c.x = 1;
    c.mem.storeb(0x07FF, 0x0F);
    c.mem.storeb(0x8000, 0x3D);
    c.mem.storeb(0x8001, 0xFE);
    c.mem.storeb(0x8002, 0x07);
    c.step();
    assert_eq!(c.a, 0x05);

    let mut c = CPU::test();
    // absolute, y
    c.a = 0b0101_0101;
    c.y = 1;
    c.mem.storeb(0x07FF, 0x0F);
    c.mem.storeb(0x8000, 0x39);
    c.mem.storeb(0x8001, 0xFE);
    c.mem.storeb(0x8002, 0x07);
    c.step();
    assert_eq!(c.a, 0x05);

    c.a = 0b0101_0101;
    c.and(InstrArg::Immediate(0b1010_1010));
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);

    c.a = 0b1010_1010;
    c.and(InstrArg::Immediate(0x80));
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);

    c.a = 0x7F;
    c.ora(InstrArg::Immediate(0x7F));
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, false);
}

#[test]
fn ora() {
    let mut c = CPU::test();
    // immediate
    c.a = 0b0101_0101;
    c.mem.storeb(0x8000, 0x09);
    c.mem.storeb(0x8001, 0b1010_1010);
    c.step();
    assert_eq!(c.a, 0xFF);

    let mut c = CPU::test();
    // zero page
    c.a = 0b0101_0101;
    c.mem.storeb(0x10, 0b1010_1010);
    c.mem.storeb(0x8000, 0x05);
    c.mem.storeb(0x8001, 0x10);
    c.step();
    assert_eq!(c.a, 0xFF);

    let mut c = CPU::test();
    // zero page, x
    c.a = 0b0101_0101;
    c.x = 1;
    c.mem.storeb(0x10, 0b1010_1010);
    c.mem.storeb(0x8000, 0x15);
    c.mem.storeb(0x8001, 0x0F);
    c.step();
    assert_eq!(c.a, 0xFF);

    let mut c = CPU::test();
    // indirect, x
    c.a = 0b0101_0101;
    c.x = 1;
    c.mem.storeb(0x1F, 0b1010_1010);
    c.mem.storeb(0x10, 0x1F);
    c.mem.storeb(0x8000, 0x01);
    c.mem.storeb(0x8001, 0x0F);
    c.step();
    assert_eq!(c.a, 0xFF);

    let mut c = CPU::test();
    // indirect, y
    c.a = 0b0101_0101;
    c.y = 1;
    c.mem.storeb(0x1F, 0b1010_1010);
    c.mem.storeb(0x10, 0x1E);
    c.mem.storeb(0x8000, 0x11);
    c.mem.storeb(0x8001, 0x10);
    c.step();
    assert_eq!(c.a, 0xFF);

    let mut c = CPU::test();
    // absolute
    c.a = 0b0101_0101;
    c.mem.storeb(0x07FF, 0b1010_1010);
    c.mem.storeb(0x8000, 0x0D);
    c.mem.storeb(0x8001, 0xFF);
    c.mem.storeb(0x8002, 0x07);
    c.step();
    assert_eq!(c.a, 0xFF);

    let mut c = CPU::test();
    // absolute, x
    c.a = 0b0101_0101;
    c.x = 1;
    c.mem.storeb(0x07FF, 0b1010_1010);
    c.mem.storeb(0x8000, 0x1D);
    c.mem.storeb(0x8001, 0xFE);
    c.mem.storeb(0x8002, 0x07);
    c.step();
    assert_eq!(c.a, 0xFF);

    let mut c = CPU::test();
    // absolute, y
    c.a = 0b0101_0101;
    c.y = 1;
    c.mem.storeb(0x07FF, 0b1010_1010);
    c.mem.storeb(0x8000, 0x19);
    c.mem.storeb(0x8001, 0xFE);
    c.mem.storeb(0x8002, 0x07);
    c.step();
    assert_eq!(c.a, 0xFF);

    c.a = 0;
    c.ora(InstrArg::Immediate(0x00));
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);

    c.a = 0;
    c.ora(InstrArg::Immediate(0x80));
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);

    c.a = 0x80;
    c.ora(InstrArg::Immediate(0x0));
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);

    c.a = 0x0;
    c.ora(InstrArg::Immediate(0x7F));
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, false);
}

#[test]
fn sty() {
    let mut c = CPU::test();
    // zero page
    c.y = 1;
    c.mem.storeb(0x8000, 0x84);
    c.mem.storeb(0x8001, 0xFF);
    c.step();
    assert_eq!(c.mem.loadb(0x00FF), 1);

    let mut c = CPU::test();
    // zero page, x
    c.y = 1;
    c.x = 2;
    c.mem.storeb(0x8000, 0x94);
    c.mem.storeb(0x8001, 0xFD);
    c.step();
    assert_eq!(c.mem.loadb(0x00FF), 1);

    let mut c = CPU::test();
    // absolute
    c.y = 1;
    c.mem.storeb(0x8000, 0x8C);
    c.mem.storeb(0x8001, 0xFF);
    c.mem.storeb(0x8002, 0x07);
    c.step();
    assert_eq!(c.mem.loadb(0x07FF), 1);
}

#[test]
fn ldy() {
    let mut c = CPU::test();
    // immediate
    c.mem.storeb(0x8000, 0xA0);
    c.mem.storeb(0x8001, 0x10);
    c.step();
    assert_eq!(c.y, 0x10);

    let mut c = CPU::test();
    // zero page
    c.mem.storeb(0x00FF, 0x10);
    c.mem.storeb(0x8000, 0xA4);
    c.mem.storeb(0x8001, 0xFF);
    c.step();
    assert_eq!(c.y, 0x10);

    let mut c = CPU::test();
    // zero page, x
    c.x = 1;
    c.mem.storeb(0x00FF, 0x10);
    c.mem.storeb(0x8000, 0xB4);
    c.mem.storeb(0x8001, 0xFE);
    c.step();
    assert_eq!(c.y, 0x10);

    let mut c = CPU::test();
    // absolute
    c.mem.storeb(0x10FF, 0x10);
    c.mem.storeb(0x8000, 0xAC);
    c.mem.storeb(0x8001, 0xFF);
    c.mem.storeb(0x8002, 0x10);
    c.step();
    assert_eq!(c.y, 0x10);

    let mut c = CPU::test();
    // absolute, x
    c.x = 1;
    c.mem.storeb(0x10FF, 0x10);
    c.mem.storeb(0x8000, 0xBC);
    c.mem.storeb(0x8001, 0xFE);
    c.mem.storeb(0x8002, 0x10);
    c.step();
    assert_eq!(c.y, 0x10);

    c.ldy(InstrArg::Immediate(0x00));
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);

    c.ldy(InstrArg::Immediate(0x80));
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);

    c.ldy(InstrArg::Immediate(0x7F));
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, false);
}

#[test]
fn stx() {
    let mut c = CPU::test();
    // zero page
    c.x = 1;
    c.mem.storeb(0x8000, 0x86);
    c.mem.storeb(0x8001, 0xFF);
    c.step();
    assert_eq!(c.mem.loadb(0x00FF), 1);

    let mut c = CPU::test();
    // zero page, y
    c.x = 1;
    c.y = 2;
    c.mem.storeb(0x8000, 0x96);
    c.mem.storeb(0x8001, 0xFD);
    c.step();
    assert_eq!(c.mem.loadb(0x00FF), 1);

    let mut c = CPU::test();
    // absolute
    c.x = 1;
    c.mem.storeb(0x8000, 0x8E);
    c.mem.storeb(0x8001, 0xFF);
    c.mem.storeb(0x8002, 0x07);
    c.step();
    assert_eq!(c.mem.loadb(0x07FF), 1);
}

#[test]
fn ldx() {
    let mut c = CPU::test();
    // immediate
    c.mem.storeb(0x8000, 0xA2);
    c.mem.storeb(0x8001, 0x10);
    c.step();
    assert_eq!(c.x, 0x10);

    let mut c = CPU::test();
    // zero page
    c.mem.storeb(0x00FF, 0x10);
    c.mem.storeb(0x8000, 0xA6);
    c.mem.storeb(0x8001, 0xFF);
    c.step();
    assert_eq!(c.x, 0x10);

    let mut c = CPU::test();
    // zero page, y
    c.y = 1;
    c.mem.storeb(0x00FF, 0x10);
    c.mem.storeb(0x8000, 0xB6);
    c.mem.storeb(0x8001, 0xFE);
    c.step();
    assert_eq!(c.x, 0x10);

    let mut c = CPU::test();
    // absolute
    c.mem.storeb(0x10FF, 0x10);
    c.mem.storeb(0x8000, 0xAE);
    c.mem.storeb(0x8001, 0xFF);
    c.mem.storeb(0x8002, 0x10);
    c.step();
    assert_eq!(c.x, 0x10);

    let mut c = CPU::test();
    // absolute, y
    c.y = 1;
    c.mem.storeb(0x10FF, 0x10);
    c.mem.storeb(0x8000, 0xBE);
    c.mem.storeb(0x8001, 0xFE);
    c.mem.storeb(0x8002, 0x10);
    c.step();
    assert_eq!(c.x, 0x10);

    c.ldx(InstrArg::Immediate(0x00));
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);

    c.ldx(InstrArg::Immediate(0x80));
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);

    c.ldx(InstrArg::Immediate(0x7F));
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, false);
}

#[test]
fn sta() {
    let mut c = CPU::test();
    // zero page
    c.a = 3;
    c.mem.storeb(0x8000, 0x85);
    c.mem.storeb(0x8001, 0x10);
    c.step();
    assert_eq!(c.mem.loadb(0x0010), 3);

    let mut c = CPU::test();
    // zero page, X
    c.a = 3;
    c.x = 1;
    c.mem.storeb(0x8000, 0x95);
    c.mem.storeb(0x8001, 0x10);
    c.step();
    assert_eq!(c.mem.loadb(0x0011), 3);

    let mut c = CPU::test();
    // absolute
    c.a = 3;
    c.mem.storeb(0x8000, 0x8D);
    c.mem.storeb(0x8001, 0xFF);
    c.mem.storeb(0x8002, 0x10);
    c.step();
    assert_eq!(c.mem.loadb(0x10FF), 3);

    let mut c = CPU::test();
    // absolute, x
    c.a = 3;
    c.x = 1;
    c.mem.storeb(0x8000, 0x9D);
    c.mem.storeb(0x8001, 0xFE);
    c.mem.storeb(0x8002, 0x10);
    c.step();
    assert_eq!(c.mem.loadb(0x10FF), 3);

    let mut c = CPU::test();
    // absolute, y
    c.a = 3;
    c.y = 1;
    c.mem.storeb(0x8000, 0x99);
    c.mem.storeb(0x8001, 0xFE);
    c.mem.storeb(0x8002, 0x10);
    c.step();
    assert_eq!(c.mem.loadb(0x10FF), 3);

    let mut c = CPU::test();
    // indirect, x
    c.a = 3;
    c.x = 1;
    c.mem.storeb(0x00FF, 0xFF);
    c.mem.storeb(0x0100, 0x10);
    c.mem.storeb(0x8000, 0x81);
    c.mem.storeb(0x8001, 0xFE);
    c.step();
    assert_eq!(c.mem.loadb(0x10FF), 3);

    let mut c = CPU::test();
    // indirect, y
    c.a = 3;
    c.y = 1;
    c.mem.storeb(0x00FF, 0xFE);
    c.mem.storeb(0x0100, 0x10);
    c.mem.storeb(0x8000, 0x91);
    c.mem.storeb(0x8001, 0xFF);
    c.step();
    assert_eq!(c.mem.loadb(0x10FF), 3);
}

#[test]
fn lda() {
    let mut c = CPU::test();
    c.mem.storeb(0x00, 0x0A);
    c.mem.storeb(0x01, 0x0B);
    c.mem.storeb(0x0FF, 0x0C);
    c.mem.storeb(0x1FFF, 0x0D);

    // lda #$07 (immediate)
    c.mem.storeb(0x8000, 0xA9);
    c.mem.storeb(0x8001, 0x07);
    c.step();
    assert_eq!(c.a, 0x7);

    // lda $01 (zero page)
    c.mem.storeb(0x8002, 0xA5);
    c.mem.storeb(0x8003, 0x01);
    c.step();
    assert_eq!(c.a, 0xB);

    c.x = 0xFE;
    // lda $01,X (zero page, x)
    c.mem.storeb(0x8004, 0xB5);
    c.mem.storeb(0x8005, 0x01);
    c.step();
    assert_eq!(c.a, 0xC);

    // lda $1FFF (absolute)
    c.mem.storeb(0x8006, 0xAD);
    c.mem.storeb(0x8007, 0xFF);
    c.mem.storeb(0x8008, 0x1F);
    c.step();
    assert_eq!(c.a, 0xD);

    c.x = 1;
    // lda $1000,X (absolute x)
    c.mem.storeb(0x8009, 0xBD);
    c.mem.storeb(0x800A, 0x00);
    c.mem.storeb(0x800B, 0x10);
    c.step();
    assert_eq!(c.a, 0xB);

    c.y = 0xFF;
    // lda $1000,Y (absolute y)
    c.mem.storeb(0x800C, 0xB9);
    c.mem.storeb(0x800D, 0x00);
    c.mem.storeb(0x800E, 0x10);
    c.step();
    assert_eq!(c.a, 0xC);

    c.mem.storeb(0x00FE, 0xAA);
    c.mem.storeb(0x00FF, 0xFE);
    c.mem.storeb(0x0000, 0x00);
    c.x = 0xF;

    // lda ($F0,X) (indirect x)
    c.mem.storeb(0x800F, 0xA1);
    c.mem.storeb(0x8010, 0xF0);
    c.step();
    assert_eq!(c.a, 0xAA);

    c.mem.storeb(0x00FD, 0xBB);
    c.mem.storeb(0x00F0, 0x0D);
    c.y = 0xF0;
    // lda ($F0),Y (indirect y)
    c.mem.storeb(0x8011, 0xB1);
    c.mem.storeb(0x8012, 0xF0);
    c.step();
    assert_eq!(c.a, 0xBB);

    c.lda(InstrArg::Immediate(0x00));
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);

    c.lda(InstrArg::Immediate(0x80));
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);

    c.lda(InstrArg::Immediate(0x7F));
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, false);
}
