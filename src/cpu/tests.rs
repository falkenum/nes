
use super::{ CPU, RAM_LAST, RAM_SIZE };
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
fn mapping() {
    let mut c = CPU::new();

    c.mem[0x0] = 0xA;
    c.mem[0x1] = 0xB;
    c.mem[0xFF] = 3;
    c.mem[0x7FF] = 4;

    assert_eq!(c.mem[0x0800], 0xA);
    assert_eq!(c.mem[0x1000], 0xA);
    assert_eq!(c.mem[0x1800], 0xA);
    assert_eq!(c.mem[0x0FFF], 0x4);
    assert_eq!(c.mem[0x17FF], 0x4);
    assert_eq!(c.mem[0x1FFF], 0x4);

    c.mem[0x1FFF] = 5;
    assert_eq!(c.mem[0x07FF], 0x5);
}
