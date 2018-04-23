
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

/*
Instr trait
*/

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

    // execute instruction
    // fn exec(instr : ) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test0() {
        let c = CPU::new();
        println!("{:#?}", c);
    }
}
