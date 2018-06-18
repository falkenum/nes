
use super::{ CPU, RAM_LAST, RAM_SIZE, Memory };

#[test]
fn oamdma() {
    let mut c = CPU::test();

    let oam_page = 0x02;
    let oam_base_addr = (oam_page as u16) << 8;

    // fill page 2 of ram with 0 - 255
    for i in 0..256 {
        c.mem.storeb(oam_base_addr + i, i as u8);
    }
    c.mem.oamdma(oam_page);

    let oam = c.mem.ppu.borrow().oam;

    for i in 0..256 {
        assert_eq!(oam[i], i as u8);
    }

    // test oamaddr not starting at 0
    c.mem.storeb(0x2003, 0x55);

    for i in 0..256 {
        c.mem.storeb(oam_base_addr + i, 0xBC);
    }
    c.mem.oamdma(oam_page);

    let oam = c.mem.ppu.borrow().oam;

    for val in oam.iter() {
        assert_eq!(*val, 0xBC);
    }
}

#[test]
fn ram() {
    let c = CPU::test();
    let mut mem = c.mem;

    for i in 0..(RAM_LAST + 1) {
        mem.storeb(i, (i % RAM_SIZE) as u8);
    }
    for i in 0..(RAM_LAST + 1) {
        assert_eq!(mem.loadb(i), (i % RAM_SIZE) as u8);
    }
}

#[test]
fn interrupt() {
    let mut c = CPU::test();
    c.mem.storeb(0xFFFA, 0xAB);
    c.mem.storeb(0xFFFB, 0xCD);
    c.pc = 0x8001;
    c.nmi();

    assert_eq!(c.mem.loadb(0x01FF), 0x80);
    assert_eq!(c.mem.loadb(0x01FE), 0x01);
    assert_eq!(c.mem.loadb(0x01FD), 0b00100000);
    assert_eq!(c.sp, 0xFC);
    assert_eq!(c.pc, 0xCDAB);
    assert_eq!(c.flags.i, true);

    let mut c = CPU::test();
    c.mem.storeb(0xFFFC, 0xAB);
    c.mem.storeb(0xFFFD, 0xCD);
    c.reset();
    assert_eq!(c.pc, 0xCDAB);
}

#[test]
fn ppu_regs() {
    // let mut c = CPU::test();
    // TODO
}

#[test]
fn mapping() {
    let mut c = CPU::test();

    c.mem.storeb(0x0, 0xA);
    c.mem.storeb(0x1, 0xB);
    c.mem.storeb(0xFF, 3);
    c.mem.storeb(0x7FF, 4);

    assert_eq!(c.mem.loadb(0x0800), 0xA);
    assert_eq!(c.mem.loadb(0x1000), 0xA);
    assert_eq!(c.mem.loadb(0x1800), 0xA);
    assert_eq!(c.mem.loadb(0x0FFF), 0x4);
    assert_eq!(c.mem.loadb(0x17FF), 0x4);
    assert_eq!(c.mem.loadb(0x1FFF), 0x4);

    c.mem.storeb(0x1FFF, 5);
    assert_eq!(c.mem.loadb(0x07FF), 0x5);
}
