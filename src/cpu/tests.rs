
use super::{ CPU, RAM_LAST, RAM_SIZE, Memory, Cartridge, RefCell, Rc };
#[test]
fn ram() {
    let c = CPU::new(Rc::new(RefCell::new(Cartridge::test())));
    let mut mem = c.mem;

    for i in 0..(RAM_LAST + 1) {
        mem.storeb(i, (i % RAM_SIZE) as u8);
    }
    for i in 0..(RAM_LAST + 1) {
        assert_eq!(mem.loadb(i), (i % RAM_SIZE) as u8);
    }
}

#[test]
fn mapping() {
    let mut c = CPU::new(Rc::new(RefCell::new(Cartridge::test())));

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
