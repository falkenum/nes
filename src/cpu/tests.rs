use super::*;

#[test]
fn ram() {
    let c = CPU::new();
    let mut mem = c.mem;

    for i in 0..(RAM_LAST + 1) {
        mem[i] = (i % RAM_SIZE) as u8;
    }
    for i in 0..(RAM_LAST + 1) {
        assert_eq!(mem[i], (i % RAM_SIZE) as u8);
    }
}

#[test]
fn sbc() {
    let mut c = CPU::new();
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
    let mut c = CPU::new();
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
    let mut c = CPU::new();
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

    let mut c = CPU::new();
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
    let mut c = CPU::new();
    c.a = 0b0101_0011;
    c.eor(InstrArg::Immediate(0b1010_0011));
    assert_eq!(c.a, 0xF0);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.n, true);

    let mut c = CPU::new();
    c.a = 0xFF;
    c.eor(InstrArg::Immediate(0xFF));
    assert_eq!(c.a, 0x0);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.n, false);
}

#[test]
fn and() {
    let mut c = CPU::new();
    // immediate
    c.a = 0b0101_0101;
    c.mem[0x8000] = 0x29;
    c.mem[0x8001] = 0x0F;
    c.step();
    assert_eq!(c.a, 0x05);

    let mut c = CPU::new();
    // zero page
    c.a = 0b0101_0101;
    c.mem[0x10] = 0x0F;
    c.mem[0x8000] = 0x25;
    c.mem[0x8001] = 0x10;
    c.step();
    assert_eq!(c.a, 0x05);

    let mut c = CPU::new();
    // zero page, x
    c.a = 0b0101_0101;
    c.x = 1;
    c.mem[0x10] = 0x0F;
    c.mem[0x8000] = 0x35;
    c.mem[0x8001] = 0x0F;
    c.step();
    assert_eq!(c.a, 0x05);

    let mut c = CPU::new();
    // indirect, x
    c.a = 0b0101_0101;
    c.x = 1;
    c.mem[0x1F] = 0x0F;
    c.mem[0x10] = 0x1F;
    c.mem[0x8000] = 0x21;
    c.mem[0x8001] = 0x0F;
    c.step();
    assert_eq!(c.a, 0x05);

    let mut c = CPU::new();
    // indirect, y
    c.a = 0b0101_0101;
    c.y = 1;
    c.mem[0x1F] = 0x0F;
    c.mem[0x10] = 0x1E;
    c.mem[0x8000] = 0x31;
    c.mem[0x8001] = 0x10;
    c.step();
    assert_eq!(c.a, 0x05);

    let mut c = CPU::new();
    // absolute
    c.a = 0b0101_0101;
    c.mem[0x07FF] = 0x0F;
    c.mem[0x8000] = 0x2D;
    c.mem[0x8001] = 0xFF;
    c.mem[0x8002] = 0x07;
    c.step();
    assert_eq!(c.a, 0x05);

    let mut c = CPU::new();
    // absolute, x
    c.a = 0b0101_0101;
    c.x = 1;
    c.mem[0x07FF] = 0x0F;
    c.mem[0x8000] = 0x3D;
    c.mem[0x8001] = 0xFE;
    c.mem[0x8002] = 0x07;
    c.step();
    assert_eq!(c.a, 0x05);

    let mut c = CPU::new();
    // absolute, y
    c.a = 0b0101_0101;
    c.y = 1;
    c.mem[0x07FF] = 0x0F;
    c.mem[0x8000] = 0x39;
    c.mem[0x8001] = 0xFE;
    c.mem[0x8002] = 0x07;
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
    let mut c = CPU::new();
    // immediate
    c.a = 0b0101_0101;
    c.mem[0x8000] = 0x09;
    c.mem[0x8001] = 0b1010_1010;
    c.step();
    assert_eq!(c.a, 0xFF);

    let mut c = CPU::new();
    // zero page
    c.a = 0b0101_0101;
    c.mem[0x10] = 0b1010_1010;
    c.mem[0x8000] = 0x05;
    c.mem[0x8001] = 0x10;
    c.step();
    assert_eq!(c.a, 0xFF);

    let mut c = CPU::new();
    // zero page, x
    c.a = 0b0101_0101;
    c.x = 1;
    c.mem[0x10] = 0b1010_1010;
    c.mem[0x8000] = 0x15;
    c.mem[0x8001] = 0x0F;
    c.step();
    assert_eq!(c.a, 0xFF);

    let mut c = CPU::new();
    // indirect, x
    c.a = 0b0101_0101;
    c.x = 1;
    c.mem[0x1F] = 0b1010_1010;
    c.mem[0x10] = 0x1F;
    c.mem[0x8000] = 0x01;
    c.mem[0x8001] = 0x0F;
    c.step();
    assert_eq!(c.a, 0xFF);

    let mut c = CPU::new();
    // indirect, y
    c.a = 0b0101_0101;
    c.y = 1;
    c.mem[0x1F] = 0b1010_1010;
    c.mem[0x10] = 0x1E;
    c.mem[0x8000] = 0x11;
    c.mem[0x8001] = 0x10;
    c.step();
    assert_eq!(c.a, 0xFF);

    let mut c = CPU::new();
    // absolute
    c.a = 0b0101_0101;
    c.mem[0x07FF] = 0b1010_1010;
    c.mem[0x8000] = 0x0D;
    c.mem[0x8001] = 0xFF;
    c.mem[0x8002] = 0x07;
    c.step();
    assert_eq!(c.a, 0xFF);

    let mut c = CPU::new();
    // absolute, x
    c.a = 0b0101_0101;
    c.x = 1;
    c.mem[0x07FF] = 0b1010_1010;
    c.mem[0x8000] = 0x1D;
    c.mem[0x8001] = 0xFE;
    c.mem[0x8002] = 0x07;
    c.step();
    assert_eq!(c.a, 0xFF);

    let mut c = CPU::new();
    // absolute, y
    c.a = 0b0101_0101;
    c.y = 1;
    c.mem[0x07FF] = 0b1010_1010;
    c.mem[0x8000] = 0x19;
    c.mem[0x8001] = 0xFE;
    c.mem[0x8002] = 0x07;
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
    let mut c = CPU::new();
    // zero page
    c.y = 1;
    c.mem[0x8000] = 0x84;
    c.mem[0x8001] = 0xFF;
    c.step();
    assert_eq!(c.mem[0x00FF], 1);

    let mut c = CPU::new();
    // zero page, x
    c.y = 1;
    c.x = 2;
    c.mem[0x8000] = 0x94;
    c.mem[0x8001] = 0xFD;
    c.step();
    assert_eq!(c.mem[0x00FF], 1);

    let mut c = CPU::new();
    // absolute
    c.y = 1;
    c.mem[0x8000] = 0x8C;
    c.mem[0x8001] = 0xFF;
    c.mem[0x8002] = 0x07;
    c.step();
    assert_eq!(c.mem[0x07FF], 1);
}

#[test]
fn ldy() {
    let mut c = CPU::new();
    // immediate
    c.mem[0x8000] = 0xA0;
    c.mem[0x8001] = 0x10;
    c.step();
    assert_eq!(c.y, 0x10);

    let mut c = CPU::new();
    // zero page
    c.mem[0x00FF] = 0x10;
    c.mem[0x8000] = 0xA4;
    c.mem[0x8001] = 0xFF;
    c.step();
    assert_eq!(c.y, 0x10);

    let mut c = CPU::new();
    // zero page, x
    c.x = 1;
    c.mem[0x00FF] = 0x10;
    c.mem[0x8000] = 0xB4;
    c.mem[0x8001] = 0xFE;
    c.step();
    assert_eq!(c.y, 0x10);

    let mut c = CPU::new();
    // absolute
    c.mem[0x10FF] = 0x10;
    c.mem[0x8000] = 0xAC;
    c.mem[0x8001] = 0xFF;
    c.mem[0x8002] = 0x10;
    c.step();
    assert_eq!(c.y, 0x10);

    let mut c = CPU::new();
    // absolute, x
    c.x = 1;
    c.mem[0x10FF] = 0x10;
    c.mem[0x8000] = 0xBC;
    c.mem[0x8001] = 0xFE;
    c.mem[0x8002] = 0x10;
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
    let mut c = CPU::new();
    // zero page
    c.x = 1;
    c.mem[0x8000] = 0x86;
    c.mem[0x8001] = 0xFF;
    c.step();
    assert_eq!(c.mem[0x00FF], 1);

    let mut c = CPU::new();
    // zero page, y
    c.x = 1;
    c.y = 2;
    c.mem[0x8000] = 0x96;
    c.mem[0x8001] = 0xFD;
    c.step();
    assert_eq!(c.mem[0x00FF], 1);

    let mut c = CPU::new();
    // absolute
    c.x = 1;
    c.mem[0x8000] = 0x8E;
    c.mem[0x8001] = 0xFF;
    c.mem[0x8002] = 0x07;
    c.step();
    assert_eq!(c.mem[0x07FF], 1);
}

#[test]
fn ldx() {
    let mut c = CPU::new();
    // immediate
    c.mem[0x8000] = 0xA2;
    c.mem[0x8001] = 0x10;
    c.step();
    assert_eq!(c.x, 0x10);

    let mut c = CPU::new();
    // zero page
    c.mem[0x00FF] = 0x10;
    c.mem[0x8000] = 0xA6;
    c.mem[0x8001] = 0xFF;
    c.step();
    assert_eq!(c.x, 0x10);

    let mut c = CPU::new();
    // zero page, y
    c.y = 1;
    c.mem[0x00FF] = 0x10;
    c.mem[0x8000] = 0xB6;
    c.mem[0x8001] = 0xFE;
    c.step();
    assert_eq!(c.x, 0x10);

    let mut c = CPU::new();
    // absolute
    c.mem[0x10FF] = 0x10;
    c.mem[0x8000] = 0xAE;
    c.mem[0x8001] = 0xFF;
    c.mem[0x8002] = 0x10;
    c.step();
    assert_eq!(c.x, 0x10);

    let mut c = CPU::new();
    // absolute, y
    c.y = 1;
    c.mem[0x10FF] = 0x10;
    c.mem[0x8000] = 0xBE;
    c.mem[0x8001] = 0xFE;
    c.mem[0x8002] = 0x10;
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
    let mut c = CPU::new();
    // zero page
    c.a = 3;
    c.mem[0x8000] = 0x85;
    c.mem[0x8001] = 0x10;
    c.step();
    assert_eq!(c.mem[0x0010], 3);

    let mut c = CPU::new();
    // zero page, X
    c.a = 3;
    c.x = 1;
    c.mem[0x8000] = 0x95;
    c.mem[0x8001] = 0x10;
    c.step();
    assert_eq!(c.mem[0x0011], 3);

    let mut c = CPU::new();
    // absolute
    c.a = 3;
    c.mem[0x8000] = 0x8D;
    c.mem[0x8001] = 0xFF;
    c.mem[0x8002] = 0x10;
    c.step();
    assert_eq!(c.mem[0x10FF], 3);

    let mut c = CPU::new();
    // absolute, x
    c.a = 3;
    c.x = 1;
    c.mem[0x8000] = 0x9D;
    c.mem[0x8001] = 0xFE;
    c.mem[0x8002] = 0x10;
    c.step();
    assert_eq!(c.mem[0x10FF], 3);

    let mut c = CPU::new();
    // absolute, y
    c.a = 3;
    c.y = 1;
    c.mem[0x8000] = 0x99;
    c.mem[0x8001] = 0xFE;
    c.mem[0x8002] = 0x10;
    c.step();
    assert_eq!(c.mem[0x10FF], 3);

    let mut c = CPU::new();
    // indirect, x
    c.a = 3;
    c.x = 1;
    c.mem[0x00FF] = 0x10FF;
    c.mem[0x8000] = 0x81;
    c.mem[0x8001] = 0xFE;
    c.step();
    assert_eq!(c.mem[0x10FF], 3);

    let mut c = CPU::new();
    // indirect, y
    c.a = 3;
    c.y = 1;
    c.mem[0x00FF] = 0x10FE;
    c.mem[0x8000] = 0x91;
    c.mem[0x8001] = 0xFF;
    c.step();
    assert_eq!(c.mem[0x10FF], 3);
}

#[test]
fn lda() {
    let mut c = CPU::new();
    c.mem[0x00] = 0x0A;
    c.mem[0x01] = 0x0B;
    c.mem[0x0FF] = 0x0C;
    c.mem[0x1FFF] = 0x0D;

    // lda #$07 (immediate)
    c.mem[0x8000] = 0xA9;
    c.mem[0x8001] = 0x07;
    c.step();
    assert_eq!(c.a, 0x7);

    // lda $01 (zero page)
    c.mem[0x8002] = 0xA5;
    c.mem[0x8003] = 0x01;
    c.step();
    assert_eq!(c.a, 0xB);

    c.x = 0xFE;
    // lda $01,X (zero page, x)
    c.mem[0x8004] = 0xB5;
    c.mem[0x8005] = 0x01;
    c.step();
    assert_eq!(c.a, 0xC);

    // lda $1FFF (absolute)
    c.mem[0x8006] = 0xAD;
    c.mem[0x8007] = 0xFF;
    c.mem[0x8008] = 0x1F;
    c.step();
    assert_eq!(c.a, 0xD);

    c.x = 1;
    // lda $1000,X (absolute x)
    c.mem[0x8009] = 0xBD;
    c.mem[0x800A] = 0x00;
    c.mem[0x800B] = 0x10;
    c.step();
    assert_eq!(c.a, 0xB);

    c.y = 0xFF;
    // lda $1000,Y (absolute y)
    c.mem[0x800C] = 0xB9;
    c.mem[0x800D] = 0x00;
    c.mem[0x800E] = 0x10;
    c.step();
    assert_eq!(c.a, 0xC);

    c.mem[0x00FE] = 0xAA;
    c.mem[0x00FF] = 0xFE;
    c.mem[0x0000] = 0x00;
    c.x = 0xF;

    // lda ($F0,X) (indirect x)
    c.mem[0x800F] = 0xA1;
    c.mem[0x8010] = 0xF0;
    c.step();
    assert_eq!(c.a, 0xAA);

    c.mem[0x00FD] = 0xBB;
    c.mem[0x00F0] = 0x0D;
    c.y = 0xF0;
    // lda ($F0),Y (indirect y)
    c.mem[0x8011] = 0xB1;
    c.mem[0x8012] = 0xF0;
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

#[test]
fn mapping() {
    let mut c = CPU::new();

    c.lda(InstrArg::Immediate(0x00));
    assert_eq!(c.a, 0x00);
    c.lda(InstrArg::Immediate(0xFF));
    assert_eq!(c.a, 0xFF);

    c.mem[0x0] = 0xA;
    c.mem[0x1] = 0xB;
    c.mem[0xFF] = 3;
    c.mem[0x7FF] = 4;

    c.lda(InstrArg::Address(0x0000));
    assert_eq!(c.a, 0xA);
    assert_eq!(c.mem[0x0800], 0xA);
    assert_eq!(c.mem[0x1000], 0xA);
    assert_eq!(c.mem[0x1800], 0xA);
    c.lda(InstrArg::Address(0x07FF));
    assert_eq!(c.a, 0x4);
    assert_eq!(c.mem[0x0FFF], 0x4);
    assert_eq!(c.mem[0x17FF], 0x4);
    assert_eq!(c.mem[0x1FFF], 0x4);

    c.mem[0x1FFF] = 5;
    assert_eq!(c.mem[0x07FF], 0x5);
}

#[test]
fn addr_modes() {

    let mut c = CPU::new();

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
}
