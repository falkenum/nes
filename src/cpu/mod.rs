
mod instructions;

#[allow(dead_code)] // TODO delete
#[derive(Debug)]
pub struct CPU {
    a : u8,
    x : u8,
    y : u8,
    sp : u8,
    pc : u16,
    flags : u8,
}

pub struct RAM {}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            a : 0,
            x : 0,
            y : 0,
            sp : 0,
            pc : 0,
            flags : 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test0() {
        let mut c = CPU::new();
        c.exec_op(0xA9, 0x0500);
        assert_eq!(c.a, 5);
    }
}
